/*
 * Kestrel - a lightweight real-time messaging service
 * Copyright (C) 2026 Kestrel Chat
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use rocket::futures::StreamExt;
use serde_json::Value;
use ws::WebSocket;

use crate::{opcode::OpCode, protocol::Packet};

pub fn gateway(ws: WebSocket) -> ws::Channel<'static> {
    ws.channel(|mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let msg = match message {
                    Ok(m) => m,
                    Err(_) => break,
                };

                if let ws::Message::Text(text) = msg {
                    if let Ok(packet) = serde_json::from_str::<Packet>(&text) {
                        handle_packet(packet).await;
                    }
                }
            }

            Ok(())
        })
    })
}

async fn handle_packet(packet: Packet) {
    match packet.op {
        OpCode::Dispatch => {
            if let Some(t) = packet.t {
                handle_dispatch(t, packet.d).await;
            }
        }

        OpCode::Heartbeat => {
            handle_heartbeat().await;
        }

        OpCode::Identify => {
            handle_identify(packet.d).await;
        }
    }
}

async fn handle_heartbeat() {
    println!("heartbeat");
}

async fn handle_identify(payload: Value) {
    println!("identify: {:?}", payload);
}

enum DispatchEvent {
    Test,
    Unknown(String),
}

impl DispatchEvent {
    fn from_str(s: &str) -> Self {
        match s {
            "dispatch.test" => DispatchEvent::Test,
            other => DispatchEvent::Unknown(other.to_string()),
        }
    }
}

async fn handle_dispatch(t: String, d: Value) {
    match DispatchEvent::from_str(&t) {
        DispatchEvent::Test => {
            println!("test event: {:?}", d);
        }

        DispatchEvent::Unknown(name) => {
            println!("unknown event: {}", name);
        }
    }
}
