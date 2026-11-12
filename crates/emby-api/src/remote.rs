//! Remote control: advertises this client as controllable (so the server dashboard shows
//! Pause / Stop / Send message) and listens on Emby's websocket for those commands.

use crate::Client;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone)]
pub enum RemoteEvent {
    Pause,
    Unpause,
    PlayPause,
    Stop,
    SeekTicks(i64),
    Rewind,
    FastForward,
    Next,
    Previous,
    SetVolume(i32),
    Mute,
    Unmute,
    ToggleMute,
    Message { header: String, text: String },
    Play { item_id: String, start_ticks: i64 },
}

/// Runs until the task is aborted; reconnects on failure.
pub async fn run(client: Client, on_event: Arc<dyn Fn(RemoteEvent) + Send + Sync>) {
    loop {
        let _ = client.post_capabilities().await;
        crate::log("remote control: connecting");
        let r = session(&client, &on_event).await;
        crate::log(&format!("remote control: disconnected ({})", r.map(|_| "closed".to_string()).unwrap_or_else(|e| e.to_string())));
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn session(client: &Client, on_event: &Arc<dyn Fn(RemoteEvent) + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
    let url = client.websocket_url().ok_or("not signed in")?;
    let (ws, _) = tokio_tungstenite::connect_async(url).await?;
    let (mut tx, mut rx) = ws.split();
    let mut keepalive = tokio::time::interval(Duration::from_secs(30));
    loop {
        tokio::select! {
            _ = keepalive.tick() => {
                tx.send(Message::Text(r#"{"MessageType":"KeepAlive"}"#.into())).await?;
            }
            msg = rx.next() => {
                let Some(msg) = msg else { return Ok(()) };
                match msg? {
                    Message::Text(t) => {
                        if let Ok(v) = serde_json::from_str::<Value>(&t) {
                            if v["MessageType"] == "ForceKeepAlive" {
                                tx.send(Message::Text(r#"{"MessageType":"KeepAlive"}"#.into())).await?;
                            } else if let Some(ev) = parse(&v) {
                                crate::log(&format!("remote command: {ev:?}"));
                                on_event(ev);
                            }
                        }
                    }
                    Message::Close(_) => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn parse(v: &Value) -> Option<RemoteEvent> {
    let data = &v["Data"];
    match v["MessageType"].as_str()? {
        "Playstate" => Some(match data["Command"].as_str()? {
            "Pause" => RemoteEvent::Pause,
            "Unpause" => RemoteEvent::Unpause,
            "PlayPause" => RemoteEvent::PlayPause,
            "Stop" => RemoteEvent::Stop,
            "Seek" => RemoteEvent::SeekTicks(data["SeekPositionTicks"].as_i64()?),
            "Rewind" => RemoteEvent::Rewind,
            "FastForward" => RemoteEvent::FastForward,
            "NextTrack" => RemoteEvent::Next,
            "PreviousTrack" => RemoteEvent::Previous,
            _ => return None,
        }),
        "GeneralCommand" => {
            let args = &data["Arguments"];
            Some(match data["Name"].as_str()? {
                "DisplayMessage" => RemoteEvent::Message {
                    header: args["Header"].as_str().unwrap_or("").to_string(),
                    text: args["Text"].as_str().unwrap_or("").to_string(),
                },
                "SetVolume" => RemoteEvent::SetVolume(args["Volume"].as_str()?.parse().ok()?),
                "Mute" => RemoteEvent::Mute,
                "Unmute" => RemoteEvent::Unmute,
                "ToggleMute" => RemoteEvent::ToggleMute,
                _ => return None,
            })
        }
        "Play" => Some(RemoteEvent::Play {
            item_id: match &data["ItemIds"][0] {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => return None,
            },
            start_ticks: data["StartPositionTicks"].as_i64().unwrap_or(0),
        }),
        _ => None,
    }
}
