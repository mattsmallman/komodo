use database::mungos::mongodb::Collection;
use indexmap::IndexSet;
use komodo_client::entities::{
  ResourceTarget, ResourceTargetVariant,
  permission::SpecificPermission,
  resource::Resource,
  swarm::{
    PartialSwarmConfig, Swarm, SwarmConfig, SwarmConfigDiff,
    SwarmListItem, SwarmListItemInfo, SwarmQuerySpecifics, SwarmState,
  },
};

use crate::state::db_client;

impl super::KomodoResource for Swarm {
  type Config = SwarmConfig;
  type PartialConfig = PartialSwarmConfig;
  type ConfigDiff = SwarmConfigDiff;
  type Info = komodo_client::entities::swarm::SwarmInfo;
  type ListItem = SwarmListItem;
  type QuerySpecifics = SwarmQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Swarm
  }

  fn resource_target(id: impl Into<String>) -> ResourceTarget {
    ResourceTarget::Swarm(id.into())
  }

  fn creator_specific_permissions() -> IndexSet<SpecificPermission> {
    IndexSet::new()
  }

  fn coll() -> &'static Collection<Resource<Self::Config, Self::Info>>
  {
    &db_client().swarms
  }

  async fn to_list_item(
    swarm: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let node_count = swarm.info.nodes.len();
    let manager_count = swarm
      .info
      .nodes
      .iter()
      .filter(|n| {
        matches!(
          n.role,
          komodo_client::entities::swarm::SwarmNodeRole::Manager
        )
      })
      .count();
    let worker_count = node_count - manager_count;

    SwarmListItem {
      name: swarm.name,
      id: swarm.id,
      template: swarm.template,
      tags: swarm.tags,
      resource_type: ResourceTargetVariant::Swarm,
      info: SwarmListItemInfo {
        server_id: swarm.config.server_id,
        initialized: swarm.info.initialized,
        swarm_id: swarm.info.swarm_id,
        node_count,
        manager_count,
        worker_count,
        state: if swarm.info.initialized {
          SwarmState::Active
        } else {
          SwarmState::NotInitialized
        },
      },
    }
  }
}
