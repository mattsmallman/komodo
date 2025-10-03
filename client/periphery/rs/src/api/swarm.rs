use komodo_client::entities::update::Log;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

// Re-export types from komodo_client
pub use komodo_client::entities::swarm::*;

// ============
// SWARM INIT
// ============

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct InitSwarm {
  /// Optional advertise address
  #[serde(default)]
  pub advertise_addr: String,
  /// Optional listen address
  #[serde(default)]
  pub listen_addr: String,
  /// Optional data path address
  #[serde(default)]
  pub data_path_addr: String,
  /// Default address pools
  #[serde(default)]
  pub default_addr_pool: Vec<String>,
  /// Default address pool mask length
  #[serde(default)]
  pub default_addr_pool_mask_length: u32,
}

// ============
// SWARM LEAVE
// ============

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct LeaveSwarm {
  /// Force leave even if node is a manager
  #[serde(default)]
  pub force: bool,
}

// ==============
// INSPECT SWARM
// ==============

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct InspectSwarm {}

// ===========
// LIST NODES
// ===========

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct ListSwarmNodes {}

// ================
// INSPECT NODE
// ================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct InspectSwarmNode {
  pub node_id: String,
}

// =================
// CREATE SERVICE
// =================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct CreateSwarmService {
  pub name: String,
  pub image: String,
  #[serde(default)]
  pub replicas: u64,
  #[serde(default)]
  pub env: Vec<String>,
  #[serde(default)]
  pub mounts: Vec<String>,
  #[serde(default)]
  pub networks: Vec<String>,
  #[serde(default)]
  pub ports: Vec<String>,
  #[serde(default)]
  pub update_parallelism: u64,
  #[serde(default)]
  pub update_delay: String,
  #[serde(default)]
  pub update_failure_action: String,
  #[serde(default)]
  pub update_order: String,
}

// =================
// UPDATE SERVICE
// =================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct UpdateSwarmService {
  pub name: String,
  #[serde(default)]
  pub image: String,
  #[serde(default)]
  pub replicas: Option<u64>,
  #[serde(default)]
  pub env_add: Vec<String>,
  #[serde(default)]
  pub env_rm: Vec<String>,
  #[serde(default)]
  pub mount_add: Vec<String>,
  #[serde(default)]
  pub mount_rm: Vec<String>,
  #[serde(default)]
  pub network_add: Vec<String>,
  #[serde(default)]
  pub network_rm: Vec<String>,
  #[serde(default)]
  pub publish_add: Vec<String>,
  #[serde(default)]
  pub publish_rm: Vec<String>,
  #[serde(default)]
  pub update_parallelism: Option<u64>,
  #[serde(default)]
  pub update_delay: String,
  #[serde(default)]
  pub update_failure_action: String,
  #[serde(default)]
  pub update_order: String,
  #[serde(default)]
  pub force: bool,
}

// ================
// REMOVE SERVICE
// ================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct RemoveSwarmService {
  pub name: String,
}

// ===============
// SCALE SERVICE
// ===============

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct ScaleSwarmService {
  pub name: String,
  pub replicas: u64,
}

// ================
// LIST SERVICES
// ================

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct ListSwarmServices {}

// ==================
// INSPECT SERVICE
// ==================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct InspectSwarmService {
  pub name: String,
}

// ================
// SERVICE LOGS
// ================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct GetSwarmServiceLogs {
  pub name: String,
  /// Number of lines to tail (default: all)
  #[serde(default)]
  pub tail: String,
  /// Follow log output
  #[serde(default)]
  pub follow: bool,
  /// Show timestamps
  #[serde(default)]
  pub timestamps: bool,
}

// ======================
// ROLLBACK SERVICE
// ======================

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct RollbackSwarmService {
  pub name: String,
}
