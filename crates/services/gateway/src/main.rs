/*
 * Kestrel - a modern instant-messaging service written in Rust
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

mod gateway;
mod handlers;
mod protocol;

use config::Config as AppConfig;
use rocket::Config as RocketConfig;

use crate::gateway::gateway_route;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let config = AppConfig::load().expect("Failed to load config");

    let rocket_config = RocketConfig {
        address: config.network.host.parse().expect("valid bind address"),
        port: config.network.ports.gateway,
        ..RocketConfig::default()
    };

    rocket::custom(rocket_config).mount("/", routes![gateway_route])
}
