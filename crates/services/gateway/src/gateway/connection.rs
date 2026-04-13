use rocket::futures::StreamExt;
use ws::WebSocket;

use crate::{gateway::router, protocol::packet::Packet};

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
                        router::route(packet).await;
                    }
                }
            }

            Ok(())
        })
    })
}
