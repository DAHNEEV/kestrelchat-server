<p align="center">
    <img src="https://github.com/kestrelchat/backend/blob/prod/assets/README/banner.png?raw=true" alt="Kestrel Banner">
</p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.94%2B-6e6ade?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/github/license/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/stars/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/forks/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/last-commit/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/commit-activity/m/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/contributors/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/issues/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/issues-pr/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://img.shields.io/github/languages/code-size/kestrelchat/backend?style=for-the-badge&color=6e6ade" />
  <img src="https://www.aschey.tech/tokei/github.com/kestrelchat/backend?style=for-the-badge&color=6e6ade&branch=prod&language=Rust,Dockerfile,Python" />
  <a href="https://discord.gg/T8rAX8DmNS">
    <img src="https://img.shields.io/discord/1453177758233661706?style=for-the-badge&logo=discord&logoColor=white&color=6e6ade" />
  </a>
</p>

# Getting Started
<!--If anyone's good at writing Docs and sees this, reach out to me in Kestrel's community please - Stribes-->
## Prerequisites

Before Running Kestrel, ensure you have:
- Rust (latest stable recommended)
- Docker + Docker Compose (recommended)


## Configuration
Kestrel uses a shared TOML configuration file.

Create your local config:
```bash
cp kestrel.example.toml kestrel.toml
```

Then configure it for your environment.

## Running with Docker 
Kestrel is designed to run as multiple isolated services.

Start the full stack:
```bash
docker compose up --build
```

## Running Manually

### 1. Set config path
```bash
export KESTREL_CONFIG=path/to/kestrel.toml
```

### 2. Run services
Gateway:
```bash
cargo run -p gateway
```
API:
```bash
cargo run -p api
```
