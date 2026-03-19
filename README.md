# 🦀 MiniBox — Lightweight Container Runtime

A lightweight container runtime built with Rust + Tauri.
Learning project inspired by OrbStack / Docker Desktop.

## Goal

Build a minimal but functional container management tool:
- **Rust** for the container runtime (namespace, cgroup, networking)
- **Tauri** for the desktop GUI (lightweight alternative to Electron)

## Architecture

```
┌─────────────────────────────────┐
│  MiniBox Desktop (Tauri)        │  ← GUI: container list, logs, stats
│  Frontend: React/TypeScript     │
└──────────────┬──────────────────┘
               │ Tauri IPC
┌──────────────▼──────────────────┐
│  MiniBox Daemon (Rust)          │  ← Container lifecycle, image mgmt
│  REST API + Unix Socket         │
└──────────────┬──────────────────┘
               │ syscalls
┌──────────────▼──────────────────┐
│  minibox-runtime (Rust)         │  ← OCI runtime (like runc/youki)
│  namespace, cgroup, pivot_root  │
└─────────────────────────────────┘
```

## Tech Stack

| Layer | Tech |
|-------|------|
| GUI | Tauri v2 + React + TypeScript |
| Daemon | Rust (tokio, axum) |
| Runtime | Rust (nix, libc) |
| Container Spec | OCI Runtime Spec |

## Task List

### Phase 0: Setup & Foundation (Week 1)
- [x] Create project structure
- [x] GitHub repo
- [ ] Set up Rust workspace (daemon + runtime + GUI)
- [ ] Install Tauri v2 + React frontend
- [ ] Basic CI (cargo test + clippy)

### Phase 1: Mini Container Runtime (Week 2-5)
> Goal: `minibox run alpine /bin/sh` works

- [ ] **P1-1**: Process isolation with Linux namespaces
  - [ ] PID namespace (unshare)
  - [ ] Mount namespace
  - [ ] UTS namespace (hostname)
  - [ ] Network namespace
- [ ] **P1-2**: Filesystem isolation
  - [ ] pivot_root / chroot
  - [ ] Mount proc, sys, dev
  - [ ] OverlayFS for layers
- [ ] **P1-3**: Resource limits with cgroup v2
  - [ ] Memory limit
  - [ ] CPU limit
  - [ ] PID limit
- [ ] **P1-4**: OCI config.json parsing
  - [ ] Basic OCI runtime spec support
  - [ ] Container create / start / delete lifecycle

### Phase 2: Image Management (Week 6-8)
> Goal: Pull images from Docker Hub

- [ ] **P2-1**: OCI Image spec
  - [ ] Image manifest parsing
  - [ ] Layer download (gzip tar)
  - [ ] Layer unpacking to rootfs
- [ ] **P2-2**: Registry client
  - [ ] Docker Hub v2 API authentication
  - [ ] Image pull with progress
  - [ ] Local image storage

### Phase 3: Daemon & API (Week 9-11)
> Goal: REST API for container management

- [ ] **P3-1**: MiniBox daemon
  - [ ] Unix socket server (axum)
  - [ ] Container CRUD API
  - [ ] Container logs (stdout/stderr capture)
  - [ ] Container stats (memory, CPU from cgroup)
- [ ] **P3-2**: CLI client
  - [ ] `minibox run <image> <cmd>`
  - [ ] `minibox ps` / `minibox logs` / `minibox stop`
  - [ ] `minibox images` / `minibox pull`

### Phase 4: Networking (Week 12-14)
> Goal: Containers can access the internet

- [ ] **P4-1**: Bridge networking
  - [ ] veth pair creation
  - [ ] Bridge device setup
  - [ ] NAT with iptables
- [ ] **P4-2**: Port forwarding
  - [ ] Host → Container port mapping
  - [ ] DNS resolution inside container

### Phase 5: Tauri Desktop App (Week 15-18)
> Goal: GUI for managing containers

- [ ] **P5-1**: Tauri + React setup
  - [ ] IPC bridge (Tauri commands → Rust daemon)
  - [ ] Dark/light theme
- [ ] **P5-2**: Container dashboard
  - [ ] Container list (running/stopped)
  - [ ] Start/stop/delete actions
  - [ ] Real-time logs viewer
  - [ ] Resource usage graphs (memory, CPU)
- [ ] **P5-3**: Image management
  - [ ] Image list
  - [ ] Pull with progress bar
  - [ ] Image delete
- [ ] **P5-4**: Settings
  - [ ] Resource limits (global)
  - [ ] Network config
  - [ ] Storage path config

### Phase 6: Polish & Advanced (Week 19+)
- [ ] Docker Compose (basic) support
- [ ] Volume mounts
- [ ] Multi-arch support (aarch64/x86_64)
- [ ] Auto-update
- [ ] Menu bar app (macOS)

## Development

```bash
# Build runtime
cargo build -p minibox-runtime

# Build daemon
cargo build -p minibox-daemon

# Run GUI (dev mode)
cd gui && npm run tauri dev

# Run tests
cargo test --workspace
```

## References

- [youki](https://github.com/containers/youki) — Rust OCI runtime
- [Tauri](https://tauri.app/) — Rust desktop framework
- [OCI Runtime Spec](https://github.com/opencontainers/runtime-spec)
- [OCI Image Spec](https://github.com/opencontainers/image-spec)
- [Containers from Scratch (Liz Rice)](https://www.youtube.com/watch?v=8fi7uSYlOdc)

## License

MIT
