<p align="center">
    <img src="https://github.com/kestrelchat/kestrelchat/blob/prod/assets/README/banner.png?raw=true" alt="Kestrel Banner">
</p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.94%2B-6e6ade?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/github/license/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/stars/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/forks/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/last-commit/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/commit-activity/m/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/contributors/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/issues/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/issues-pr/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/languages/code-size/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade" />
  <img src="https://www.aschey.tech/tokei/github.com/kestrelchat/kestrelchat?style=for-the-badge&color=6e6ade&language=Rust,Dockerfile,Python" />
  <a href="https://discord.gg/T8rAX8DmNS">
    <img src="https://img.shields.io/discord/1453177758233661706?style=for-the-badge&logo=discord&logoColor=white&color=6e6ade" />
  </a>
</p>

# About

Kestrel is a modern instant-messaging service written in Rust.

This repository contains only the backend implementation of the system.

It exists because most chat platforms don’t care, are closed, or simply feel clunky. <!--Some of them also just… suck. Looking at you, Valour.-->

# Contributing
## Prerequisites

- Rust 1.94+
- Any IDE with Rust support (I recommend Zed, with AI turned off, please.)
- Docker with Docker Compose (recommended)

## Configuration
Kestrel uses a shared TOML configuration file.

To get started, copy the example file `kestrel.example.toml` to `kestrel.toml`

Everything should be preconfigured for hosting on a single machine, through docker.

# Structure

## Services
- **gateway** - WebSocket handling
- **api** - REST API backend

## Library
- **config** - Shared configuration library for Kestrel

## Running

### With Docker

> Recommended for most users.

To run all services:
```bash
docker compose up --build
```

To run specific services:
```bash
docker compose up --build api
```

### Without Docker

Set your config path:
```bash
export KESTREL_CONFIG=path/to/kestrel.toml
```

Then run the services manually with:
```bash
cargo run -p <service>
```

Any external services (databases, cdns, etc.) must be configured in ``kestrel.toml``.

# External Libraries

Kestrel is built on top of the following open-source Rust libraries:

- [Serde](https://github.com/serde-rs/serde)
- [Serde JSON](https://github.com/serde-rs/json)
- [TOML](https://github.com/toml-rs/toml)
- [Rocket](https://github.com/rwf2/Rocket) 
- [rocket_ws](https://github.com/SergioBenitez/rocket_ws)
- [rocket_okapi](https://github.com/GREsau/rocket_okapi)
- [Chrono](https://github.com/chronotope/chrono)
