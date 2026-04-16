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

pub mod routes;

use rocket::Config;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let rocket_config = Config {
        address: "0.0.0.0".parse().expect("valid bind address"),
        port: 5181,
        ..Config::default()
    };

    let swagger =
        rocket_okapi::swagger_ui::make_swagger_ui(&rocket_okapi::swagger_ui::SwaggerUIConfig {
            url: "/openapi.json".to_owned(),
            ..Default::default()
        });

    let rocket = rocket::custom(rocket_config).mount("/swagger", swagger);

    routes::mount(rocket)
}
