//! Container lifecycle management

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub id: String,
    pub image: String,
    pub command: Vec<String>,
    pub memory_limit: Option<u64>,  // bytes
    pub cpu_limit: Option<f64>,     // cores
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerState {
    Creating,
    Created,
    Running,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub config: ContainerConfig,
    pub state: ContainerState,
    pub pid: Option<u32>,
}

impl Container {
    pub fn new(config: ContainerConfig) -> Self {
        Self {
            config,
            state: ContainerState::Creating,
            pid: None,
        }
    }

    /// Create a new container (set up namespaces, cgroups, rootfs)
    pub fn create(&mut self) -> Result<()> {
        tracing::info!(id = %self.config.id, "Creating container");
        // TODO: Phase 1 — namespace + cgroup + rootfs setup
        self.state = ContainerState::Created;
        Ok(())
    }

    /// Start the container process
    pub fn start(&mut self) -> Result<()> {
        tracing::info!(id = %self.config.id, "Starting container");
        // TODO: Phase 1 — fork + exec inside namespaces
        self.state = ContainerState::Running;
        Ok(())
    }

    /// Stop the container
    pub fn stop(&mut self) -> Result<()> {
        tracing::info!(id = %self.config.id, "Stopping container");
        // TODO: send SIGTERM → SIGKILL
        self.state = ContainerState::Stopped;
        self.pid = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_lifecycle() {
        let config = ContainerConfig {
            id: "test-001".to_string(),
            image: "alpine:latest".to_string(),
            command: vec!["/bin/sh".to_string()],
            memory_limit: Some(64 * 1024 * 1024), // 64MB
            cpu_limit: Some(1.0),
        };

        let mut container = Container::new(config);
        assert_eq!(container.state, ContainerState::Creating);

        container.create().unwrap();
        assert_eq!(container.state, ContainerState::Created);

        container.start().unwrap();
        assert_eq!(container.state, ContainerState::Running);

        container.stop().unwrap();
        assert_eq!(container.state, ContainerState::Stopped);
    }
}
