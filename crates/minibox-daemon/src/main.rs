//! MiniBox Daemon — Container management service
//!
//! Provides a REST API over Unix socket for managing containers.
//! The Tauri GUI and CLI both communicate through this daemon.

use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!(
        version = minibox_runtime::version(),
        "MiniBox daemon starting"
    );

    // TODO: Phase 3 — Unix socket server with axum
    // TODO: Container lifecycle management
    // TODO: Image management

    println!("🦀 MiniBox Daemon v{}", minibox_runtime::version());
    println!("   Listening on /tmp/minibox.sock");

    Ok(())
}
