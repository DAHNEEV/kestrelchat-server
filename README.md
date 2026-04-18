<h1>
  Kestrel Backend
  
  ![Rust](https://img.shields.io/badge/rust-1.94%2B-orange?style=flat-square&logo=rust)
  [![License](https://img.shields.io/github/license/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/blob/main/LICENSE)
  [![Contributors](https://img.shields.io/github/contributors/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/graphs/contributors)
  [![Stars](https://img.shields.io/github/stars/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/stargazers)
  [![Forks](https://img.shields.io/github/forks/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/network/members)
  [![Pull Requests](https://img.shields.io/github/issues-pr/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/pulls)
  [![Issues](https://img.shields.io/github/issues/kestrelchat/backend?style=flat-square&logoColor=white)](https://github.com/kestrelchat/backend/issues)
</h1>
Monorepo for Kestrel's backend services<br/>
</div>

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
