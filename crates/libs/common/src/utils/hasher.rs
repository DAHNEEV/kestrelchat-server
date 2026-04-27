// Kestrel - a modern instant-messaging service written in Rust
// Copyright (C) 2026 Kestrel Chat
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub async fn hash(input: &[u8]) -> Result<String, ()> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(input, &salt).map_err(|_| ())?;

    Ok(hash.to_string())
}

pub async fn verify(input: &[u8], digest: &str) -> Result<(), ()> {
    let parsed = PasswordHash::new(digest).map_err(|_| ())?;
    let argon2 = Argon2::default();

    argon2.verify_password(input, &parsed).map_err(|_| ())
}
