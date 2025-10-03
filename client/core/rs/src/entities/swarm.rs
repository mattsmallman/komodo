use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::Display;
use typeshare::typeshare;

use crate::deserializers::string_list_deserializer;

use super::{
  resource::{Resource, ResourceListItem, ResourceQuery},
};

#[typeshare]
pub type Swarm = Resource<SwarmConfig, SwarmInfo>;

#[typeshare]
pub type SwarmListItem = ResourceListItem<SwarmListItemInfo>;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmListItemInfo {
  /// The server that the swarm manager is deployed on.
  pub server_id: String,
  /// Whether the swarm is initialized
  pub initialized: bool,
  /// Swarm ID
  pub swarm_id: Option<String>,
  /// Number of nodes in the swarm
  pub node_count: usize,
  /// Number of manager nodes
  pub manager_count: usize,
  /// Number of worker nodes
  pub worker_count: usize,
  /// Swarm state
  pub state: SwarmState,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SwarmState {
  /// Swarm is active and operational
  Active,
  /// Swarm is not initialized
  NotInitialized,
  /// Server not reachable for status
  #[default]
  Unknown,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmInfo {
  /// The swarm ID once initialized
  pub swarm_id: Option<String>,
  /// Node ID of the manager
  pub node_id: Option<String>,
  /// Whether the swarm is initialized
  pub initialized: bool,
  /// Information about nodes in the swarm
  #[serde(default)]
  pub nodes: Vec<SwarmNode>,
}

#[typeshare(serialized_as = "Partial<SwarmConfig>")]
pub type _PartialSwarmConfig = PartialSwarmConfig;

/// Docker Swarm configuration
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct SwarmConfig {
  /// The server to use as swarm manager.
  #[serde(default, alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  #[builder(default)]
  pub server_id: String,

  /// Configure quick links that are displayed in the resource header
  #[serde(default, deserialize_with = "string_list_deserializer")]
  #[builder(default)]
  pub links: Vec<String>,

  /// Whether to automatically initialize the swarm if not initialized
  #[serde(default)]
  #[builder(default)]
  pub auto_init: bool,

  /// Advertise address for the swarm manager (optional)
  #[serde(default)]
  #[builder(default)]
  pub advertise_addr: String,

  /// Listen address for the swarm manager (optional)
  #[serde(default)]
  #[builder(default)]
  pub listen_addr: String,

  /// Data path address for the swarm (optional)
  #[serde(default)]
  #[builder(default)]
  pub data_path_addr: String,

  /// Default availability for workers (active, pause, drain)
  #[serde(default = "default_availability")]
  #[builder(default = "default_availability()")]
  #[partial_default(default_availability())]
  pub default_addr_pool: Vec<String>,

  /// Default address pools for the swarm
  #[serde(default)]
  #[builder(default)]
  pub default_addr_pool_mask_length: u32,

  /// Whether to send alerts for swarm state changes
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_alerts: bool,
}

impl SwarmConfig {
  pub fn builder() -> SwarmConfigBuilder {
    SwarmConfigBuilder::default()
  }
}

fn default_availability() -> Vec<String> {
  Vec::new()
}

fn default_send_alerts() -> bool {
  true
}

impl Default for SwarmConfig {
  fn default() -> Self {
    Self {
      server_id: Default::default(),
      links: Default::default(),
      auto_init: Default::default(),
      advertise_addr: Default::default(),
      listen_addr: Default::default(),
      data_path_addr: Default::default(),
      default_addr_pool: default_availability(),
      default_addr_pool_mask_length: Default::default(),
      send_alerts: default_send_alerts(),
    }
  }
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmNode {
  /// Node ID
  pub id: String,
  /// Node hostname
  pub hostname: String,
  /// Node role (manager or worker)
  pub role: SwarmNodeRole,
  /// Node availability (active, pause, drain)
  pub availability: SwarmNodeAvailability,
  /// Node state (ready, down)
  pub state: SwarmNodeState,
  /// Node address
  pub addr: String,
  /// Whether this is the leader node
  pub leader: bool,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmNodeRole {
  #[default]
  Worker,
  Manager,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmNodeAvailability {
  #[default]
  Active,
  Pause,
  Drain,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmNodeState {
  #[default]
  Ready,
  Down,
  Unknown,
  Disconnected,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SwarmActionState {
  pub initializing: bool,
  pub leaving: bool,
  pub updating: bool,
}

#[typeshare]
pub type SwarmQuery = ResourceQuery<SwarmQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct SwarmQuerySpecifics {
  /// Query only for Swarms on these Servers.
  /// If empty, does not filter by Server.
  /// Only accepts Server id (not name).
  #[serde(default)]
  pub server_ids: Vec<String>,
}

impl super::resource::AddFilters for SwarmQuerySpecifics {
  fn add_filters(&self, filters: &mut bson::Document) {
    if !self.server_ids.is_empty() {
      filters.insert(
        "config.server_id",
        bson::doc! { "$in": &self.server_ids },
      );
    }
  }
}

/// Configuration for deploying a service to a swarm
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
pub struct SwarmServiceConfig {
  /// Service name
  pub name: String,
  /// Image to use for the service
  pub image: String,
  /// Number of replicas (for replicated services)
  #[serde(default = "default_replicas")]
  #[builder(default = "default_replicas()")]
  pub replicas: u64,
  /// Environment variables for the service
  #[serde(default)]
  #[builder(default)]
  pub env: Vec<String>,
  /// Mounts for the service
  #[serde(default)]
  #[builder(default)]
  pub mounts: Vec<SwarmServiceMount>,
  /// Networks for the service
  #[serde(default)]
  #[builder(default)]
  pub networks: Vec<String>,
  /// Port mappings
  #[serde(default)]
  #[builder(default)]
  pub ports: Vec<SwarmServicePort>,
  /// Update configuration
  #[serde(default)]
  #[builder(default)]
  pub update_config: SwarmServiceUpdateConfig,
  /// Rollback configuration
  #[serde(default)]
  #[builder(default)]
  pub rollback_config: SwarmServiceRollbackConfig,
  /// Restart policy
  #[serde(default)]
  #[builder(default)]
  pub restart_policy: SwarmServiceRestartPolicy,
  /// Resource limits
  #[serde(default)]
  #[builder(default)]
  pub resources: SwarmServiceResources,
}

fn default_replicas() -> u64 {
  1
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmServiceMount {
  pub source: String,
  pub target: String,
  #[serde(default)]
  pub mount_type: SwarmMountType,
  #[serde(default)]
  pub read_only: bool,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmMountType {
  #[default]
  Bind,
  Volume,
  Tmpfs,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmServicePort {
  #[serde(default)]
  pub protocol: SwarmPortProtocol,
  pub target_port: u32,
  pub published_port: u32,
  #[serde(default)]
  pub publish_mode: SwarmPortPublishMode,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmPortProtocol {
  #[default]
  Tcp,
  Udp,
  Sctp,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmPortPublishMode {
  #[default]
  Ingress,
  Host,
}

/// Update configuration for zero-downtime deployments
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmServiceUpdateConfig {
  /// Number of tasks to update at once (default: 1)
  #[serde(default = "default_parallelism")]
  pub parallelism: u64,
  /// Delay between updates (in nanoseconds, default: 0)
  #[serde(default)]
  pub delay: u64,
  /// Action on update failure (pause, continue, rollback)
  #[serde(default)]
  pub failure_action: SwarmUpdateFailureAction,
  /// Monitor period after each update (in nanoseconds)
  #[serde(default)]
  pub monitor: u64,
  /// Failure tolerance during update (default: 0)
  #[serde(default)]
  pub max_failure_ratio: f32,
  /// Order of operations (stop-first or start-first)
  #[serde(default)]
  pub order: SwarmUpdateOrder,
}

impl Default for SwarmServiceUpdateConfig {
  fn default() -> Self {
    Self {
      parallelism: default_parallelism(),
      delay: 0,
      failure_action: Default::default(),
      monitor: 0,
      max_failure_ratio: 0.0,
      order: Default::default(),
    }
  }
}

fn default_parallelism() -> u64 {
  1
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmUpdateFailureAction {
  #[default]
  Pause,
  Continue,
  Rollback,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum SwarmUpdateOrder {
  #[default]
  StopFirst,
  StartFirst,
}

/// Rollback configuration
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmServiceRollbackConfig {
  /// Number of tasks to rollback at once (default: 1)
  #[serde(default = "default_parallelism")]
  pub parallelism: u64,
  /// Delay between rollbacks (in nanoseconds, default: 0)
  #[serde(default)]
  pub delay: u64,
  /// Action on rollback failure (pause, continue)
  #[serde(default)]
  pub failure_action: SwarmRollbackFailureAction,
  /// Monitor period after each rollback (in nanoseconds)
  #[serde(default)]
  pub monitor: u64,
  /// Failure tolerance during rollback (default: 0)
  #[serde(default)]
  pub max_failure_ratio: f32,
  /// Order of operations (stop-first or start-first)
  #[serde(default)]
  pub order: SwarmUpdateOrder,
}

impl Default for SwarmServiceRollbackConfig {
  fn default() -> Self {
    Self {
      parallelism: default_parallelism(),
      delay: 0,
      failure_action: Default::default(),
      monitor: 0,
      max_failure_ratio: 0.0,
      order: Default::default(),
    }
  }
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmRollbackFailureAction {
  #[default]
  Pause,
  Continue,
}

/// Restart policy for services
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmServiceRestartPolicy {
  /// Condition for restart (none, on-failure, any)
  #[serde(default)]
  pub condition: SwarmRestartCondition,
  /// Delay between restart attempts (in nanoseconds)
  #[serde(default)]
  pub delay: u64,
  /// Maximum attempts during the window
  #[serde(default)]
  pub max_attempts: u64,
  /// Window for restart attempts (in nanoseconds)
  #[serde(default)]
  pub window: u64,
}

impl Default for SwarmServiceRestartPolicy {
  fn default() -> Self {
    Self {
      condition: Default::default(),
      delay: 0,
      max_attempts: 0,
      window: 0,
    }
  }
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum SwarmRestartCondition {
  None,
  OnFailure,
  #[default]
  Any,
}

/// Resource limits and reservations
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmServiceResources {
  #[serde(default)]
  pub limits: SwarmResourceSpec,
  #[serde(default)]
  pub reservations: SwarmResourceSpec,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmResourceSpec {
  /// CPU limit/reservation (in nano CPUs, e.g., 1000000000 = 1 CPU)
  #[serde(default)]
  pub nano_cpus: i64,
  /// Memory limit/reservation (in bytes)
  #[serde(default)]
  pub memory_bytes: i64,
}

/// Information about a swarm service
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmService {
  /// Service ID
  pub id: String,
  /// Service name
  pub name: String,
  /// Service image
  pub image: String,
  /// Service mode (replicated or global)
  pub mode: SwarmServiceMode,
  /// Number of replicas (for replicated services)
  pub replicas: Option<u64>,
  /// Current running replicas
  pub running: Option<u64>,
  /// Ports published by the service
  pub ports: Vec<SwarmServicePort>,
  /// When the service was created
  pub created_at: String,
  /// When the service was last updated
  pub updated_at: String,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SwarmServiceMode {
  #[default]
  Replicated,
  Global,
}
