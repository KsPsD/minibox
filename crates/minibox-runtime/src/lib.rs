//! MiniBox Runtime — OCI-compatible container runtime
//!
//! Handles the low-level container operations:
//! - Linux namespace creation (PID, Mount, UTS, Network)
//! - cgroup v2 resource limits
//! - Filesystem isolation (pivot_root, OverlayFS)
//! - OCI runtime spec parsing

pub mod container;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
