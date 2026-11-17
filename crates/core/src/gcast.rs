//! Google Cast (Chromecast) sender: finds devices with mDNS and plays a stream on the Default Media
//! Receiver over the Cast v2 protocol (TLS on port 8009, length-prefixed protobuf messages).
//! No UI and no OS calls, so it is shared by every build.

use serde_json::{json, Value};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_rustls::rustls::{self, client::danger::*, pki_types::*, DigitallySignedStruct, SignatureScheme};

const DEFAULT_RECEIVER: &str = "CC1AD845";
const NS_CONN: &str = "urn:x-cast:com.google.cast.tp.connection";
const NS_BEAT: &str = "urn:x-cast:com.google.cast.tp.heartbeat";
const NS_RECV: &str = "urn:x-cast:com.google.cast.receiver";
const NS_MEDIA: &str = "urn:x-cast:com.google.cast.media";

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
}

// ---------------------------------------------------------------- discovery (mDNS)

/// Looks for Cast devices (Chromecasts, TVs with Cast built in) for `wait`. A proper mDNS client: it joins the
/// multicast group on every network adapter, which is how Chromecasts answer (a lone question sent from a plain
/// socket is often never answered, and was not on Android).
pub async fn discover(wait: Duration) -> Vec<Device> {
    tokio::task::spawn_blocking(move || browse(wait)).await.unwrap_or_default()
}

fn browse(wait: Duration) -> Vec<Device> {
    use mdns_sd::{ServiceDaemon, ServiceEvent};
    let mdns = match ServiceDaemon::new() {
        Ok(m) => m,
        Err(e) => {
            crate::log(&format!("cast: mDNS could not start: {e}"));
            return Vec::new();
        }
    };
    let rx = match mdns.browse("_googlecast._tcp.local.") {
        Ok(r) => r,
        Err(e) => {
            crate::log(&format!("cast: mDNS browse failed: {e}"));
            let _ = mdns.shutdown();
            return Vec::new();
        }
    };
    let mut devices: Vec<Device> = Vec::new();
    let end = std::time::Instant::now() + wait;
    while let Some(left) = end.checked_duration_since(std::time::Instant::now()) {
        match rx.recv_timeout(left) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                let addrs: Vec<IpAddr> = info.get_addresses().iter().map(|a| a.to_ip_addr()).collect();
                let Some(ip) = addrs.iter().find(|a| a.is_ipv4()).or(addrs.first()).copied() else { continue };
                let name = info
                    .get_property_val_str("fn")
                    .map(str::to_string)
                    .unwrap_or_else(|| info.get_fullname().split("._googlecast").next().unwrap_or("Chromecast").to_string());
                let port = info.get_port();
                if !devices.iter().any(|d| d.ip == ip && d.port == port) {
                    devices.push(Device { name, ip, port });
                }
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }
    let _ = mdns.shutdown();
    crate::log(&format!("cast: found {} Cast device(s): {:?}", devices.len(), devices.iter().map(|d| d.name.as_str()).collect::<Vec<_>>()));
    devices
}

// ---------------------------------------------------------------- protocol

fn varint(buf: &mut Vec<u8>, mut v: usize) {
    while v >= 0x80 {
        buf.push((v as u8 & 0x7F) | 0x80);
        v >>= 7;
    }
    buf.push(v as u8);
}

fn field(buf: &mut Vec<u8>, num: u8, s: &str) {
    buf.push(num << 3 | 2);
    varint(buf, s.len());
    buf.extend_from_slice(s.as_bytes());
}

/// One CastMessage, framed with its 4-byte length.
fn frame(src: &str, dst: &str, ns: &str, payload: &Value) -> Vec<u8> {
    let mut m = vec![0x08, 0x00]; // protocol_version = CASTV2_1_0
    field(&mut m, 2, src);
    field(&mut m, 3, dst);
    field(&mut m, 4, ns);
    m.extend_from_slice(&[0x28, 0x00]); // payload_type = STRING
    field(&mut m, 6, &payload.to_string());
    let mut out = (m.len() as u32).to_be_bytes().to_vec();
    out.extend(m);
    out
}

/// (namespace, source, payload) of a received message.
fn decode(mut b: &[u8]) -> Option<(String, String, Value)> {
    let (mut ns, mut src, mut body) = (String::new(), String::new(), String::new());
    while !b.is_empty() {
        let key = b[0];
        b = &b[1..];
        match key & 7 {
            0 => {
                while let Some((&c, rest)) = b.split_first() {
                    b = rest;
                    if c & 0x80 == 0 {
                        break;
                    }
                }
            }
            2 => {
                let (mut len, mut shift) = (0usize, 0);
                while let Some((&c, rest)) = b.split_first() {
                    b = rest;
                    len |= ((c & 0x7F) as usize) << shift;
                    shift += 7;
                    if c & 0x80 == 0 {
                        break;
                    }
                }
                let s = String::from_utf8_lossy(b.get(..len)?).into_owned();
                b = &b[len..];
                match key >> 3 {
                    2 => src = s,
                    4 => ns = s,
                    6 => body = s,
                    _ => {}
                }
            }
            _ => return None,
        }
    }
    Some((ns, src, serde_json::from_str(&body).unwrap_or(Value::Null)))
}

#[derive(Debug)]
struct NoVerify;

impl ServerCertVerifier for NoVerify {
    // Cast devices present self-signed certificates, so there is nothing to verify against.
    fn verify_server_cert(&self, _: &CertificateDer<'_>, _: &[CertificateDer<'_>], _: &ServerName<'_>, _: &[u8], _: UnixTime) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(&self, _: &[u8], _: &CertificateDer<'_>, _: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn verify_tls13_signature(&self, _: &[u8], _: &CertificateDer<'_>, _: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider().signature_verification_algorithms.supported_schemes()
    }
}

// ---------------------------------------------------------------- a cast in progress

#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    Play,
    Pause,
    Seek(f64),
    Stop,
}

/// A running cast. Dropping it ends the connection (the device stops playing).
pub struct Cast {
    tx: UnboundedSender<Cmd>,
    status: Arc<std::sync::Mutex<Status>>,
}

/// What the device last said it was doing.
#[derive(Debug, Clone, Copy, Default)]
pub struct Status {
    /// Seconds into the media.
    pub pos: f64,
    pub paused: bool,
    /// The device finished, was stopped, or the connection ended.
    pub ended: bool,
}

impl Cast {
    pub fn send(&self, c: Cmd) {
        let _ = self.tx.send(c);
    }
    pub fn status(&self) -> Status {
        *self.status.lock().unwrap()
    }
    /// A handle that keeps reporting the status after the `Cast` itself has moved elsewhere.
    pub fn status_handle(&self) -> Arc<std::sync::Mutex<Status>> {
        self.status.clone()
    }
}

/// Starts `url` on the device. `content_type` is e.g. "application/x-mpegURL" for an HLS stream.
pub async fn start(dev: &Device, url: String, content_type: &str, title: &str, subtitle: &str, image: Option<String>, start_secs: f64) -> Result<Cast, String> {
    let tcp = tokio::time::timeout(Duration::from_secs(5), TcpStream::connect((dev.ip, dev.port))).await.map_err(|_| "The device didn't answer")?.map_err(|e| e.to_string())?;
    let cfg = rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .map_err(|e| e.to_string())?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerify))
        .with_no_client_auth();
    let tls = tokio_rustls::TlsConnector::from(Arc::new(cfg)).connect(ServerName::IpAddress(dev.ip.into()), tcp).await.map_err(|e| e.to_string())?;
    let (mut rd, mut wr) = tokio::io::split(tls);

    wr.write_all(&frame("sender-0", "receiver-0", NS_CONN, &json!({"type": "CONNECT"}))).await.map_err(|e| e.to_string())?;
    wr.write_all(&frame("sender-0", "receiver-0", NS_RECV, &json!({"type": "LAUNCH", "appId": DEFAULT_RECEIVER, "requestId": 1}))).await.map_err(|e| e.to_string())?;

    // Messages arrive from a reader task so the writer can also answer commands and send heartbeats.
    let (mtx, mut mrx) = unbounded_channel::<(String, String, Value)>();
    tokio::spawn(async move {
        loop {
            let mut len = [0u8; 4];
            if rd.read_exact(&mut len).await.is_err() {
                break;
            }
            let n = u32::from_be_bytes(len) as usize;
            if n > 1 << 20 {
                break;
            }
            let mut body = vec![0u8; n];
            if rd.read_exact(&mut body).await.is_err() {
                break;
            }
            if let Some(m) = decode(&body) {
                if mtx.send(m).is_err() {
                    break;
                }
            }
        }
    });

    // Wait for the receiver app to come up, then talk to it directly.
    let (transport, session) = tokio::time::timeout(Duration::from_secs(15), async {
        while let Some((_, _, v)) = mrx.recv().await {
            if v["type"] == "RECEIVER_STATUS" {
                if let Some(app) = v["status"]["applications"].as_array().and_then(|a| a.iter().find(|a| a["appId"] == DEFAULT_RECEIVER)) {
                    if let (Some(t), Some(s)) = (app["transportId"].as_str(), app["sessionId"].as_str()) {
                        return Some((t.to_string(), s.to_string()));
                    }
                }
            }
        }
        None
    })
    .await
    .map_err(|_| "The device didn't start its player")?
    .ok_or("The device closed the connection")?;

    wr.write_all(&frame("sender-0", &transport, NS_CONN, &json!({"type": "CONNECT"}))).await.map_err(|e| e.to_string())?;
    let load = json!({
        "type": "LOAD", "requestId": 2, "sessionId": session, "autoplay": true, "currentTime": start_secs,
        "media": { "contentId": url, "streamType": "BUFFERED", "contentType": content_type, "metadata": { "metadataType": 0, "title": title, "subtitle": subtitle, "images": image.map(|u| json!([{"url": u}])).unwrap_or_else(|| json!([])) } }
    });
    wr.write_all(&frame("sender-0", &transport, NS_MEDIA, &load)).await.map_err(|e| e.to_string())?;

    let (tx, mut rx) = unbounded_channel::<Cmd>();
    let status = Arc::new(std::sync::Mutex::new(Status { pos: start_secs, ..Default::default() }));
    let st = status.clone();
    tokio::spawn(async move {
        let mut media_session: Option<i64> = None;
        let mut beat = tokio::time::interval(Duration::from_secs(5));
        let mut req = 10;
        loop {
            let out = tokio::select! {
                _ = beat.tick() => {
                    // Ask how far along it is (it only volunteers this when something changes).
                    req += 1;
                    let ask = frame("sender-0", &transport, NS_MEDIA, &json!({"type": "GET_STATUS", "requestId": req}));
                    if wr.write_all(&ask).await.is_err() {
                        break;
                    }
                    Some(frame("sender-0", "receiver-0", NS_BEAT, &json!({"type": "PING"})))
                }
                m = mrx.recv() => match m {
                    None => break,
                    Some((ns, _, v)) => {
                        if ns == NS_BEAT && v["type"] == "PING" {
                            Some(frame("sender-0", "receiver-0", NS_BEAT, &json!({"type": "PONG"})))
                        } else {
                            if v["type"] == "MEDIA_STATUS" {
                                if let Some(id) = v["status"][0]["mediaSessionId"].as_i64() {
                                    media_session = Some(id);
                                }
                                let m = &v["status"][0];
                                let mut g = st.lock().unwrap();
                                if let Some(t) = m["currentTime"].as_f64() {
                                    g.pos = t;
                                }
                                match m["playerState"].as_str() {
                                    Some("PAUSED") => g.paused = true,
                                    Some("PLAYING") | Some("BUFFERING") => g.paused = false,
                                    Some("IDLE") if m["idleReason"].is_string() => g.ended = true,
                                    _ => {}
                                }
                            }
                            // The person ended it on the device, or it was taken over.
                            if v["type"] == "RECEIVER_STATUS" && v["status"]["applications"].as_array().is_some_and(|a| a.is_empty()) {
                                break;
                            }
                            None
                        }
                    }
                },
                c = rx.recv() => {
                    let Some(c) = c else { break };
                    req += 1;
                    let media = |t: &str, extra: Value| {
                        let mut p = json!({"type": t, "requestId": req, "mediaSessionId": media_session.unwrap_or(1)});
                        if let (Some(o), Some(e)) = (p.as_object_mut(), extra.as_object()) {
                            o.extend(e.clone());
                        }
                        frame("sender-0", &transport, NS_MEDIA, &p)
                    };
                    match c {
                        Cmd::Play => Some(media("PLAY", json!({}))),
                        Cmd::Pause => Some(media("PAUSE", json!({}))),
                        Cmd::Seek(s) => Some(media("SEEK", json!({"currentTime": s}))),
                        Cmd::Stop => {
                            let _ = wr.write_all(&frame("sender-0", "receiver-0", NS_RECV, &json!({"type": "STOP", "requestId": req, "sessionId": session}))).await;
                            break;
                        }
                    }
                }
            };
            if let Some(o) = out {
                if wr.write_all(&o).await.is_err() {
                    break;
                }
            }
        }
        st.lock().unwrap().ended = true;
    });
    Ok(Cast { tx, status })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_round_trip() {
        let f = frame("sender-0", "receiver-0", NS_CONN, &json!({"type": "CONNECT"}));
        let (ns, src, v) = decode(&f[4..]).unwrap();
        assert_eq!((ns.as_str(), src.as_str(), v["type"].as_str()), (NS_CONN, "sender-0", Some("CONNECT")));
        assert_eq!(u32::from_be_bytes(f[..4].try_into().unwrap()) as usize, f.len() - 4);
    }
}
