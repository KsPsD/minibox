<p align="center">
  <h1 align="center">🦀 MiniBox</h1>
  <p align="center">A lightweight, fast container runtime built with Rust</p>
</p>

<p align="center">
  <a href="https://github.com/KsPsD/minibox/actions"><img src="https://img.shields.io/github/actions/workflow/status/KsPsD/minibox/ci.yml?branch=main&style=flat-square" alt="CI"></a>
  <a href="https://github.com/KsPsD/minibox/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
  <a href="https://github.com/KsPsD/minibox/releases"><img src="https://img.shields.io/github/v/release/KsPsD/minibox?style=flat-square&color=orange" alt="Release"></a>
</p>

---

MiniBox is a minimal container runtime that aims to be fast, lightweight, and easy to understand. Built from scratch in Rust with a Tauri-based desktop GUI.

> ⚠️ **Early Development** — Not ready for production use.

## Features

- 🚀 **Fast startup** — No GC, target cold start <100ms
- 💾 **Low memory** — Rust-native, dynamic VM memory with balloon
- 🖥️ **Desktop GUI** — Tauri-based app (lightweight alternative to Electron)
- 📦 **OCI compatible** — Works with standard container images from Docker Hub
- 🔒 **Secure by default** — Memory-safe runtime with namespace/cgroup isolation

## Architecture

```
┌──────────────────────────────┐
│  MiniBox Desktop (Tauri)     │  GUI: dashboard, logs, stats
└──────────────┬───────────────┘
               │ IPC
┌──────────────▼───────────────┐
│  minibox-daemon              │  Container lifecycle, image mgmt
│  REST API (Unix Socket)      │
└──────────────┬───────────────┘
               │ syscalls
┌──────────────▼───────────────┐
│  minibox-runtime             │  OCI runtime (namespace, cgroup,
│                              │  overlayfs, pivot_root)
└──────────────────────────────┘
```

## Quick Start

```bash
# Install from source
cargo install --path crates/minibox-cli

# Run a container
minibox run ubuntu:latest /bin/bash

# List running containers
minibox ps

# Pull an image
minibox pull alpine:latest
```

## Build from Source

```bash
git clone https://github.com/KsPsD/minibox.git
cd minibox

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run the daemon
cargo run -p minibox-daemon

# Run the GUI (requires Node.js)
cd gui && npm install && npm run tauri dev
```

## Project Structure

```
minibox/
├── crates/
│   ├── minibox-runtime/   # OCI container runtime
│   ├── minibox-daemon/    # Container management daemon
│   └── minibox-cli/       # Command-line interface
└── gui/                   # Tauri + React desktop app
```

## Roadmap

| Phase | Description | Goal |
|-------|------------|------|
| **1** | Linux Runtime + OCI Spec | `minibox run ubuntu /bin/bash` works |
| **2** | Image Management | Pull from Docker Hub, layer management |
| **3** | macOS Integration | Virtualization.framework + lightweight kernel |
| **4** | Performance | Memory balloon, VirtioFS, cold start <100ms |
| **5** | Desktop App | Tauri GUI, docker-compose compat |

See [GitHub Issues](https://github.com/KsPsD/minibox/issues) for detailed tasks.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [youki](https://github.com/containers/youki) — Rust OCI runtime
- [Tauri](https://tauri.app/) — Lightweight desktop framework
- [OCI Runtime Spec](https://github.com/opencontainers/runtime-spec)
- [OCI Image Spec](https://github.com/opencontainers/image-spec)
- [vhost-device](https://github.com/rust-vmm/vhost-device) — Rust VirtioFS

## License

[MIT](LICENSE)
