//! Google Cast (Chromecast) sender: finds devices with mDNS and plays a stream on the Default Media
//! Receiver over the Cast v2 protocol (TLS on port 8009, length-prefixed protobuf messages).
//! No UI and no OS calls, so it is shared by every build.

use serde_json::{json, Value};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
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

fn read_name(buf: &[u8], mut pos: usize) -> Option<(String, usize)> {
    let mut out = String::new();
    let mut end = None;
    for _ in 0..32 {
        let len = *buf.get(pos)? as usize;
        if len == 0 {
            return Some((out, end.unwrap_or(pos + 1)));
        }
        if len & 0xC0 == 0xC0 {
            let off = ((len & 0x3F) << 8) | *buf.get(pos + 1)? as usize;
            end.get_or_insert(pos + 2);
            pos = off;
            continue;
        }
        out.push_str(std::str::from_utf8(buf.get(pos + 1..pos + 1 + len)?).ok()?);
        out.push('.');
        pos += 1 + len;
    }
    None
}

/// Devices named in one mDNS answer: (instance name, friendly name, port).
fn parse_answers(buf: &[u8]) -> Vec<(String, String, u16)> {
    let mut found = Vec::new();
    let u16at = |p: usize| buf.get(p..p + 2).map(|b| u16::from_be_bytes([b[0], b[1]]) as usize);
    let (Some(qd), Some(an), Some(ns), Some(ar)) = (u16at(4), u16at(6), u16at(8), u16at(10)) else { return found };
    let mut pos = 12;
    for _ in 0..qd {
        let Some((_, p)) = read_name(buf, pos) else { return found };
        pos = p + 4;
    }
    let mut names: Vec<(String, String)> = Vec::new(); // instance -> friendly name
    let mut ports: Vec<(String, u16)> = Vec::new();
    for _ in 0..an + ns + ar {
        let Some((name, p)) = read_name(buf, pos) else { break };
        let (Some(ty), Some(len)) = (u16at(p), u16at(p + 8)) else { break };
        let data = p + 10;
        if data + len > buf.len() {
            break;
        }
        match ty {
            33 if len >= 6 => ports.push((name, u16at(data + 4).unwrap_or(8009) as u16)),
            16 => {
                let mut i = data;
                while i < data + len {
                    let l = buf[i] as usize;
                    if let Some(kv) = buf.get(i + 1..i + 1 + l).and_then(|b| std::str::from_utf8(b).ok()) {
                        if let Some(f) = kv.strip_prefix("fn=") {
                            names.push((name.clone(), f.to_string()));
                        }
                    }
                    i += 1 + l;
                }
            }
            _ => {}
        }
        pos = data + len;
    }
    for (inst, port) in ports {
        let fname = names.iter().find(|(n, _)| *n == inst).map(|(_, f)| f.clone()).unwrap_or_else(|| inst.split("._googlecast").next().unwrap_or("Chromecast").to_string());
        found.push((inst, fname, port));
    }
    found
}

/// Asks the local network for Cast devices and collects answers for `wait`.
pub async fn discover(wait: Duration) -> Vec<Device> {
    let mut devices: Vec<Device> = Vec::new();
    let Ok(sock) = UdpSocket::bind("0.0.0.0:0").await else { return devices };
    let mut q: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
    for part in ["_googlecast", "_tcp", "local"] {
        q.push(part.len() as u8);
        q.extend_from_slice(part.as_bytes());
    }
    q.extend_from_slice(&[0, 0, 12, 0, 1]); // PTR, class IN
    let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)), 5353);
    let _ = sock.send_to(&q, dest).await;
    let mut buf = [0u8; 4096];
    let started = tokio::time::Instant::now();
    let mut resent = false;
    while let Some(left) = wait.checked_sub(started.elapsed()) {
        if !resent && started.elapsed() > wait / 2 {
            resent = true;
            let _ = sock.send_to(&q, dest).await;
        }
        let Ok(Ok((n, from))) = tokio::time::timeout(left.min(Duration::from_millis(300)), sock.recv_from(&mut buf)).await else { continue };
        for (_, name, port) in parse_answers(&buf[..n]) {
            if !devices.iter().any(|d| d.ip == from.ip() && d.port == port) {
                devices.push(Device { name, ip: from.ip(), port });
            }
        }
    }
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
}

impl Cast {
    pub fn send(&self, c: Cmd) {
        let _ = self.tx.send(c);
    }
}

/// Starts `url` on the device. `content_type` is e.g. "application/x-mpegURL" for an HLS stream.
pub async fn start(dev: &Device, url: String, content_type: &str, title: &str, start_secs: f64) -> Result<Cast, String> {
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
        "media": { "contentId": url, "streamType": "BUFFERED", "contentType": content_type, "metadata": { "metadataType": 0, "title": title } }
    });
    wr.write_all(&frame("sender-0", &transport, NS_MEDIA, &load)).await.map_err(|e| e.to_string())?;

    let (tx, mut rx) = unbounded_channel::<Cmd>();
    tokio::spawn(async move {
        let mut media_session: Option<i64> = None;
        let mut beat = tokio::time::interval(Duration::from_secs(5));
        let mut req = 10;
        loop {
            let out = tokio::select! {
                _ = beat.tick() => Some(frame("sender-0", "receiver-0", NS_BEAT, &json!({"type": "PING"}))),
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
    });
    Ok(Cast { tx })
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
