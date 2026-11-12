//! Self-update from this project's GitHub releases.
//!
//! A release carries three files: `Foyer.exe`, `Foyer.exe.sha256` (the exe's SHA-256 plus a
//! `version=` line) and `Foyer.exe.sha256.sig`, an ECDSA P-256 signature of that checksum file made
//! in CI with a key only the repository's secrets hold. The download is installed only if the
//! signature verifies against the public key built in below and the exe matches the signed hash,
//! so a tampered release (or a hijacked download) can't pass.

use crate::{applog, Ctx};
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

const LATEST: &str = "https://api.github.com/repos/manor7777/Foyer/releases/latest";
const ASSET: &str = "Foyer.exe";

/// Public half of the release signing key (uncompressed P-256 point). The private half is the
/// repository secret `UPDATE_SIGNING_KEY`.
const SIGNING_KEY: [u8; 65] = [
    0x04, 0xac, 0x52, 0x3e, 0xfa, 0x96, 0xc9, 0xc1, 0xcf, 0xc7, 0xd4, 0x8a, 0x5c, 0x45, 0xd4, 0x4a, 0x7e, 0x45, 0xc7, 0x2a, 0x45, 0x0d, 0xc3, 0x3b, 0x1d, 0x4a, 0xc7, 0x56, 0x9b, 0xe1, 0x7f, 0x71,
    0x7f, 0xdd, 0xe1, 0x7e, 0xb2, 0x9e, 0xbe, 0x73, 0xd2, 0xd1, 0xcc, 0x51, 0x9f, 0xd5, 0x77, 0xc3, 0x5b, 0x2d, 0x81, 0xb0, 0x18, 0xef, 0xa2, 0xe3, 0x2c, 0x92, 0x36, 0x71, 0xbe, 0x36, 0x98, 0x7a,
    0x04,
];

#[derive(Clone, Debug)]
pub struct Release {
    pub version: String,
    exe_url: String,
    exe_size: u64,
    sha_url: String,
    sig_url: String,
}

pub enum Checked {
    Latest,
    Newer(Release),
}

#[derive(Deserialize)]
struct Api {
    tag_name: String,
    #[serde(default)]
    assets: Vec<ApiAsset>,
}

#[derive(Deserialize)]
struct ApiAsset {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    size: u64,
}

fn http() -> Result<reqwest::Client, String> {
    reqwest::Client::builder().user_agent(concat!("Foyer/", env!("CARGO_PKG_VERSION"))).timeout(Duration::from_secs(60)).build().map_err(|e| e.to_string())
}

pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// `1.2.3` / `v1.2.3` as numbers; anything unparseable counts as 0.
fn version_parts(v: &str) -> [u64; 3] {
    let mut out = [0u64; 3];
    for (i, part) in v.trim_start_matches('v').split('.').take(3).enumerate() {
        out[i] = part.split(|c: char| !c.is_ascii_digit()).next().and_then(|p| p.parse().ok()).unwrap_or(0);
    }
    out
}

fn is_newer(candidate: &str, running: &str) -> bool {
    version_parts(candidate) > version_parts(running)
}

pub async fn check() -> Result<Checked, String> {
    let http = http()?;
    let resp = http.get(LATEST).header("Accept", "application/vnd.github+json").send().await.map_err(|e| e.without_url().to_string())?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(Checked::Latest); // no release published yet
    }
    let api: Api = resp.error_for_status().map_err(|e| e.without_url().to_string())?.json().await.map_err(|e| e.without_url().to_string())?;
    if !is_newer(&api.tag_name, current_version()) {
        return Ok(Checked::Latest);
    }
    let find = |suffix: &str| api.assets.iter().find(|a| a.name == format!("{ASSET}{suffix}"));
    let (Some(exe), Some(sha), Some(sig)) = (find(""), find(".sha256"), find(".sha256.sig")) else {
        // Still being uploaded, or an incomplete release: nothing to offer yet.
        return Ok(Checked::Latest);
    };
    Ok(Checked::Newer(Release {
        version: api.tag_name.trim_start_matches('v').to_string(),
        exe_url: exe.browser_download_url.clone(),
        exe_size: exe.size,
        sha_url: sha.browser_download_url.clone(),
        sig_url: sig.browser_download_url.clone(),
    }))
}

fn bad_signature() -> String {
    "The release's signature is not valid. Nothing was installed.".to_string()
}

/// `signed` must carry a valid ECDSA P-256 (SHA-256) signature, DER encoded as `openssl dgst -sign` writes it.
fn check_signature(key: &[u8], signed: &[u8], sig: &[u8]) -> Result<(), String> {
    let key = VerifyingKey::from_sec1_bytes(key).map_err(|_| bad_signature())?;
    let sig = Signature::from_der(sig).map_err(|_| bad_signature())?;
    key.verify(signed, &sig).map_err(|_| bad_signature())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// (hash, version) from the signed checksum file.
fn parse_checksum(text: &str) -> Result<(String, String), String> {
    let hash = text.split_whitespace().next().unwrap_or("").to_ascii_lowercase();
    if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("The published checksum file is not a SHA-256 hash.".into());
    }
    let version = text.lines().find_map(|l| l.trim().strip_prefix("version=")).unwrap_or("").to_string();
    Ok((hash, version))
}

async fn fetch(http: &reqwest::Client, url: &str) -> Result<Vec<u8>, String> {
    let r = http.get(url).send().await.and_then(|r| r.error_for_status()).map_err(|e| e.without_url().to_string())?;
    r.bytes().await.map(|b| b.to_vec()).map_err(|e| e.without_url().to_string())
}

/// Downloads, verifies and swaps in the new exe. The running exe is renamed to `.old` (Windows
/// allows that, not deleting), so the next start picks up the new one.
pub async fn install(rel: &Release, progress: impl Fn(u32)) -> Result<(), String> {
    let http = http()?;
    let sha = fetch(&http, &rel.sha_url).await?;
    let sig = fetch(&http, &rel.sig_url).await?;
    check_signature(&SIGNING_KEY, &sha, &sig)?;
    let (wanted, signed_version) = parse_checksum(&String::from_utf8_lossy(&sha))?;
    if signed_version != rel.version {
        return Err("The release's signed version does not match its name. Nothing was installed.".into());
    }

    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let new = exe.with_extension("exe.new");
    let old = exe.with_extension("exe.old");
    let _ = std::fs::remove_file(&new);

    let mut resp = http.get(&rel.exe_url).timeout(Duration::from_secs(900)).send().await.and_then(|r| r.error_for_status()).map_err(|e| e.without_url().to_string())?;
    let total = resp.content_length().unwrap_or(rel.exe_size);
    let mut data = Vec::with_capacity(total as usize);
    let mut last = 101;
    while let Some(chunk) = resp.chunk().await.map_err(|e| e.without_url().to_string())? {
        data.extend_from_slice(&chunk);
        if total > 0 {
            let pct = (data.len() as u64 * 100 / total).min(100) as u32;
            if pct != last {
                last = pct;
                progress(pct);
            }
        }
    }
    if !data.starts_with(b"MZ") {
        return Err("The download is not a Windows program.".into());
    }
    if hex(&Sha256::digest(&data)) != wanted {
        return Err("The download doesn't match its signed checksum. Nothing was installed.".into());
    }
    std::fs::write(&new, &data).map_err(|e| format!("Couldn't save the update ({e})."))?;

    if old.exists() && std::fs::remove_file(&old).is_err() {
        let aside = exe.with_extension(format!("exe.old{}", chrono::Utc::now().timestamp()));
        std::fs::rename(&old, &aside).map_err(|e| format!("Couldn't move the previous update aside ({e})."))?;
    }
    std::fs::rename(&exe, &old).map_err(|e| format!("Couldn't replace the running program ({e}). If Foyer is in a protected folder, move it somewhere you can write to."))?;
    if let Err(e) = std::fs::rename(&new, &exe) {
        let _ = std::fs::rename(&old, &exe);
        let _ = std::fs::remove_file(&new);
        return Err(format!("Couldn't put the new version in place ({e})."));
    }
    Ok(())
}

/// Removes files an earlier update left behind.
pub fn cleanup() {
    let Ok(exe) = std::env::current_exe() else { return };
    let _ = std::fs::remove_file(exe.with_extension("exe.new"));
    let (Some(dir), Some(name)) = (exe.parent(), exe.file_stem().and_then(|s| s.to_str())) else { return };
    let prefix = format!("{name}.exe.old");
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        if e.file_name().to_string_lossy().starts_with(&prefix) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

fn exe_path() -> Option<PathBuf> {
    std::env::current_exe().ok()
}

/// What the last check found, for the Install button.
static FOUND: Mutex<Option<Release>> = Mutex::new(None);

impl Ctx {
    fn update_ui(&self, f: impl FnOnce(&crate::AppWindow) + Send + 'static) {
        let weak = self.ui.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                f(&ui);
            }
        });
    }

    /// Looks for a newer release a few seconds after start (when switched on), and installs it
    /// right away when automatic installation is on and nothing is playing.
    pub fn update_startup(&self) {
        cleanup();
        let (check, install) = {
            let s = self.st.lock().unwrap();
            (s.cfg.update_check, s.cfg.update_install)
        };
        if !check {
            return;
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(6)).await;
            ctx.update_check_inner(false, install).await;
        });
    }

    pub fn update_check(&self) {
        let ctx = self.clone();
        self.rt.spawn(async move { ctx.update_check_inner(true, false).await });
    }

    async fn update_check_inner(&self, manual: bool, auto_install: bool) {
        self.update_ui(|ui| {
            ui.set_update_checking(true);
            ui.set_update_status("Checking for updates…".into());
        });
        match check().await {
            Ok(Checked::Latest) => {
                *FOUND.lock().unwrap() = None;
                self.update_ui(|ui| {
                    ui.set_update_checking(false);
                    ui.set_update_available(false);
                    ui.set_update_status(format!("You're up to date (version {}).", current_version()).into());
                });
            }
            Ok(Checked::Newer(rel)) => {
                applog(&format!("update available: {}", rel.version));
                let v = rel.version.clone();
                *FOUND.lock().unwrap() = Some(rel);
                let playing = self.st.lock().unwrap().playing;
                let v2 = v.clone();
                self.update_ui(move |ui| {
                    ui.set_update_checking(false);
                    ui.set_update_available(true);
                    ui.set_update_status(format!("Version {v2} is available.").into());
                });
                if auto_install && !playing {
                    self.update_install_inner().await;
                } else if !manual {
                    self.notify_user("Update available".into(), format!("Foyer {v} is ready to install (Settings → About)."), None);
                }
            }
            Err(e) => {
                applog(&format!("update check failed: {e}"));
                self.update_ui(move |ui| {
                    ui.set_update_checking(false);
                    ui.set_update_status(format!("Couldn't check for updates: {e}").into());
                });
            }
        }
    }

    pub fn update_install(&self) {
        let ctx = self.clone();
        self.rt.spawn(async move { ctx.update_install_inner().await });
    }

    async fn update_install_inner(&self) {
        let Some(rel) = FOUND.lock().unwrap().clone() else { return };
        self.update_ui(|ui| {
            ui.set_update_installing(true);
            ui.set_update_status("Downloading the update…".into());
        });
        let ctx = self.clone();
        let result = install(&rel, |pct| {
            ctx.update_ui(move |ui| ui.set_update_status(format!("Downloading the update… {pct}%").into()));
        })
        .await;
        match result {
            Ok(()) => {
                applog(&format!("update installed: {}", rel.version));
                *FOUND.lock().unwrap() = None;
                let v = rel.version.clone();
                self.update_ui(move |ui| {
                    ui.set_update_installing(false);
                    ui.set_update_available(false);
                    ui.set_update_ready(true);
                    ui.set_update_status(format!("Version {v} is installed. Restart Foyer to finish.").into());
                });
                self.notify_user("Foyer updated".into(), format!("Version {} is installed. Restart Foyer to use it.", rel.version), None);
            }
            Err(e) => {
                applog(&format!("update failed: {e}"));
                self.update_ui(move |ui| {
                    ui.set_update_installing(false);
                    ui.set_update_status(format!("Update failed: {e}").into());
                });
            }
        }
    }

    /// Starts the new exe and closes this one. Not while something is playing.
    pub fn update_restart(&self) {
        if self.st.lock().unwrap().playing {
            self.update_ui(|ui| ui.set_update_status("Stop playback first, then restart.".into()));
            return;
        }
        if let Some(exe) = exe_path() {
            if std::process::Command::new(exe).spawn().is_ok() {
                let _ = slint::quit_event_loop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::{signature::Signer, SigningKey};

    #[test]
    fn compares_versions() {
        assert!(is_newer("v0.13.0", "0.12.0"));
        assert!(is_newer("1.0.0", "0.99.9"));
        assert!(!is_newer("0.12.0", "0.12.0"));
        assert!(!is_newer("0.11.9", "0.12.0"));
        assert!(is_newer("0.12.10", "0.12.9"));
    }

    #[test]
    fn parses_checksum_file() {
        let h = "a".repeat(64);
        let (hash, ver) = parse_checksum(&format!("{h}\nversion=0.13.0\n")).unwrap();
        assert_eq!((hash.as_str(), ver.as_str()), (h.as_str(), "0.13.0"));
        assert!(parse_checksum("nothex\n").is_err());
    }

    #[test]
    fn accepts_only_a_valid_signature() {
        let sk = SigningKey::from_slice(&[7u8; 32]).unwrap();
        let pk = sk.verifying_key().to_encoded_point(false);
        let sig: Signature = sk.sign(b"hash\nversion=1.0.0\n");
        let der = sig.to_der();
        assert!(check_signature(pk.as_bytes(), b"hash\nversion=1.0.0\n", der.as_bytes()).is_ok());
        assert!(check_signature(pk.as_bytes(), b"hash\nversion=9.9.9\n", der.as_bytes()).is_err());
        assert!(check_signature(&SIGNING_KEY, b"hash\nversion=1.0.0\n", der.as_bytes()).is_err());
    }

    #[test]
    fn embedded_key_is_a_valid_point() {
        assert!(VerifyingKey::from_sec1_bytes(&SIGNING_KEY).is_ok());
    }
}
