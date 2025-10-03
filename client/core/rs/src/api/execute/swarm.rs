use crate::entities::update::Update;
use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::KomodoExecuteRequest;

/// Initialize Docker Swarm on the target server. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct InitSwarm {
  /// Id or name of the Swarm resource
  pub swarm: String,
}

/// Leave Docker Swarm on the target server. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct LeaveSwarm {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Force leave even if node is a manager
  #[serde(default)]
  pub force: bool,
}

/// Deploy a service to the swarm. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct DeploySwarmService {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
  /// Image to use for the service
  pub image: String,
  /// Number of replicas
  #[serde(default)]
  pub replicas: Option<u64>,
  /// Environment variables for the service
  #[serde(default)]
  pub env: Vec<String>,
  /// Mounts for the service
  #[serde(default)]
  pub mounts: Vec<String>,
  /// Networks for the service
  #[serde(default)]
  pub networks: Vec<String>,
  /// Port mappings
  #[serde(default)]
  pub ports: Vec<String>,
  /// Update parallelism (number of tasks to update at once)
  #[serde(default)]
  pub update_parallelism: Option<u64>,
  /// Delay between updates (e.g., "10s")
  #[serde(default)]
  pub update_delay: String,
  /// Action on update failure (pause, continue, rollback)
  #[serde(default)]
  pub update_failure_action: String,
  /// Order of operations (stop-first or start-first)
  #[serde(default)]
  pub update_order: String,
}

/// Update an existing swarm service (for zero-downtime deployments). Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct UpdateSwarmService {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
  /// New image to use for the service (optional)
  #[serde(default)]
  pub image: String,
  /// New replica count (optional)
  #[serde(default)]
  pub replicas: Option<u64>,
  /// Environment variables to add
  #[serde(default)]
  pub env_add: Vec<String>,
  /// Environment variables to remove
  #[serde(default)]
  pub env_rm: Vec<String>,
  /// Update parallelism (number of tasks to update at once)
  #[serde(default)]
  pub update_parallelism: Option<u64>,
  /// Delay between updates (e.g., "10s")
  #[serde(default)]
  pub update_delay: String,
  /// Action on update failure (pause, continue, rollback)
  #[serde(default)]
  pub update_failure_action: String,
  /// Order of operations (stop-first or start-first)
  #[serde(default)]
  pub update_order: String,
  /// Force update even if no changes detected
  #[serde(default)]
  pub force: bool,
}

/// Remove a service from the swarm. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct RemoveSwarmService {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
}

/// Scale a swarm service. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct ScaleSwarmService {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
  /// Number of replicas
  pub replicas: u64,
}

/// Get logs for a swarm service. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct GetSwarmServiceLogs {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
  /// Number of lines to tail (default: all)
  #[serde(default)]
  pub tail: String,
}

/// Rollback a swarm service to its previous version. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Resolve,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
#[error(serror::Error)]
pub struct RollbackSwarmService {
  /// Id or name of the Swarm resource
  pub swarm: String,
  /// Service name
  pub service_name: String,
}
