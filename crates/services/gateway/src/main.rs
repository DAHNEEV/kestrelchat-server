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

use rocket::Config;
use ws::WebSocket;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: "127.0.0.1".parse().unwrap(),
        port: 5180, // I think for Kestrel by default we will allocate ports 5180-5189 to us. - Stribes
        ..Config::default()
    };

    rocket::custom(config).mount("/", routes![echo])
}

#[get("/echo")]
fn echo(ws: WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(|mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let msg = message?;

                println!("Received: {:?}", msg);

                stream.send(msg).await?;
            }

            Ok(())
        })
    })
}
