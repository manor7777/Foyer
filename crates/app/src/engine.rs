//! First-run download of libmpv (the LGPL x86_64 build from the zhongfly/mpv-winbuild releases),
//! so the app can be shipped as a single exe. Skipped when a DLL is already next to the app.

use std::path::PathBuf;

const RELEASES: &str = "https://api.github.com/repos/zhongfly/mpv-winbuild/releases/latest";

#[derive(serde::Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Downloads and installs libmpv-2.dll into the app's data folder. `progress` gets a short status.
pub async fn install(progress: impl Fn(String)) -> Result<PathBuf, String> {
    let http = reqwest::Client::builder().user_agent("Foyer").build().map_err(|e| e.to_string())?;
    progress("Looking up the latest video engine…".into());
    let rel: Release = http.get(RELEASES).send().await.and_then(|r| r.error_for_status()).map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;
    let asset = rel
        .assets
        .iter()
        .find(|a| a.name.starts_with("mpv-dev-lgpl-x86_64-") && !a.name.contains("v3") && a.name.ends_with(".7z"))
        .ok_or("No suitable libmpv build found in the latest release.")?;

    let dir = player::engine_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let archive = dir.join("download.7z");
    let mut resp = http.get(&asset.browser_download_url).send().await.and_then(|r| r.error_for_status()).map_err(|e| e.to_string())?;
    let total = resp.content_length().unwrap_or(0);
    let mut got = 0u64;
    let mut data = Vec::with_capacity(total as usize);
    let mut last_pct = 101;
    while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
        got += chunk.len() as u64;
        data.extend_from_slice(&chunk);
        if total > 0 {
            let pct = (got * 100 / total) as i32;
            if pct != last_pct {
                last_pct = pct;
                progress(format!("Downloading the video engine… {pct}%"));
            }
        }
    }
    std::fs::write(&archive, &data).map_err(|e| e.to_string())?;

    progress("Installing the video engine…".into());
    let out = dir.join("unpack");
    let _ = std::fs::remove_dir_all(&out);
    let (a2, o2) = (archive.clone(), out.clone());
    tokio::task::spawn_blocking(move || sevenz_rust2::decompress_file(&a2, &o2).map_err(|e| e.to_string())).await.map_err(|e| e.to_string())??;
    let found = find_dll(&out).ok_or("The download didn't contain libmpv-2.dll.")?;
    let dest = dir.join("libmpv-2.dll");
    std::fs::copy(&found, &dest).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&out);
    let _ = std::fs::remove_file(&archive);
    Ok(dest)
}

fn find_dll(dir: &std::path::Path) -> Option<PathBuf> {
    for e in std::fs::read_dir(dir).ok()?.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(f) = find_dll(&p) {
                return Some(f);
            }
        } else if matches!(p.file_name().and_then(|n| n.to_str()), Some("libmpv-2.dll" | "mpv-2.dll")) {
            return Some(p);
        }
    }
    None
}
