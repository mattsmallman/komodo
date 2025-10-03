use command::run_komodo_command;
use komodo_client::entities::update::Log;
use periphery_client::api::swarm::*;
use resolver_api::Resolve;
use tracing::instrument;

// ============
// SWARM INIT
// ============

impl Resolve<super::Args> for InitSwarm {
  #[instrument(name = "InitSwarm")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let InitSwarm {
      advertise_addr,
      listen_addr,
      data_path_addr,
      default_addr_pool,
      default_addr_pool_mask_length,
    } = self;

    let mut command = String::from("docker swarm init");

    if !advertise_addr.is_empty() {
      command.push_str(&format!(" --advertise-addr {}", advertise_addr));
    }

    if !listen_addr.is_empty() {
      command.push_str(&format!(" --listen-addr {}", listen_addr));
    }

    if !data_path_addr.is_empty() {
      command.push_str(&format!(" --data-path-addr {}", data_path_addr));
    }

    for pool in &default_addr_pool {
      command.push_str(&format!(" --default-addr-pool {}", pool));
    }

    if default_addr_pool_mask_length > 0 {
      command.push_str(&format!(
        " --default-addr-pool-mask-length {}",
        default_addr_pool_mask_length
      ));
    }

    Ok(run_komodo_command("Docker Swarm Init", None, command).await)
  }
}

// ============
// SWARM LEAVE
// ============

impl Resolve<super::Args> for LeaveSwarm {
  #[instrument(name = "LeaveSwarm")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let LeaveSwarm { force } = self;

    let mut command = String::from("docker swarm leave");

    if force {
      command.push_str(" --force");
    }

    Ok(run_komodo_command("Docker Swarm Leave", None, command).await)
  }
}

// ==============
// INSPECT SWARM
// ==============

impl Resolve<super::Args> for InspectSwarm {
  #[instrument(name = "InspectSwarm")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let command = String::from("docker swarm inspect");
    Ok(run_komodo_command("Docker Swarm Inspect", None, command).await)
  }
}

// ===========
// LIST NODES
// ===========

impl Resolve<super::Args> for ListSwarmNodes {
  #[instrument(name = "ListSwarmNodes")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let command = String::from("docker node ls --format '{{json .}}'");
    Ok(run_komodo_command("List Swarm Nodes", None, command).await)
  }
}

// ================
// INSPECT NODE
// ================

impl Resolve<super::Args> for InspectSwarmNode {
  #[instrument(name = "InspectSwarmNode")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let InspectSwarmNode { node_id } = self;
    let command = format!("docker node inspect {}", node_id);
    Ok(run_komodo_command("Inspect Swarm Node", None, command).await)
  }
}

// =================
// CREATE SERVICE
// =================

impl Resolve<super::Args> for CreateSwarmService {
  #[instrument(name = "CreateSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let CreateSwarmService {
      name,
      image,
      replicas,
      env,
      mounts,
      networks,
      ports,
      update_parallelism,
      update_delay,
      update_failure_action,
      update_order,
    } = self;

    let mut command = format!("docker service create --name {} {}", name, image);

    if replicas > 0 {
      command.push_str(&format!(" --replicas {}", replicas));
    }

    for e in &env {
      command.push_str(&format!(" --env {}", e));
    }

    for m in &mounts {
      command.push_str(&format!(" --mount {}", m));
    }

    for n in &networks {
      command.push_str(&format!(" --network {}", n));
    }

    for p in &ports {
      command.push_str(&format!(" --publish {}", p));
    }

    if update_parallelism > 0 {
      command.push_str(&format!(
        " --update-parallelism {}",
        update_parallelism
      ));
    }

    if !update_delay.is_empty() {
      command.push_str(&format!(" --update-delay {}", update_delay));
    }

    if !update_failure_action.is_empty() {
      command.push_str(&format!(
        " --update-failure-action {}",
        update_failure_action
      ));
    }

    if !update_order.is_empty() {
      command.push_str(&format!(" --update-order {}", update_order));
    }

    Ok(run_komodo_command("Create Swarm Service", None, command).await)
  }
}

// =================
// UPDATE SERVICE
// =================

impl Resolve<super::Args> for UpdateSwarmService {
  #[instrument(name = "UpdateSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let UpdateSwarmService {
      name,
      image,
      replicas,
      env_add,
      env_rm,
      mount_add,
      mount_rm,
      network_add,
      network_rm,
      publish_add,
      publish_rm,
      update_parallelism,
      update_delay,
      update_failure_action,
      update_order,
      force,
    } = self;

    let mut command = format!("docker service update");

    if !image.is_empty() {
      command.push_str(&format!(" --image {}", image));
    }

    if let Some(r) = replicas {
      command.push_str(&format!(" --replicas {}", r));
    }

    for e in &env_add {
      command.push_str(&format!(" --env-add {}", e));
    }

    for e in &env_rm {
      command.push_str(&format!(" --env-rm {}", e));
    }

    for m in &mount_add {
      command.push_str(&format!(" --mount-add {}", m));
    }

    for m in &mount_rm {
      command.push_str(&format!(" --mount-rm {}", m));
    }

    for n in &network_add {
      command.push_str(&format!(" --network-add {}", n));
    }

    for n in &network_rm {
      command.push_str(&format!(" --network-rm {}", n));
    }

    for p in &publish_add {
      command.push_str(&format!(" --publish-add {}", p));
    }

    for p in &publish_rm {
      command.push_str(&format!(" --publish-rm {}", p));
    }

    if let Some(parallelism) = update_parallelism {
      command.push_str(&format!(" --update-parallelism {}", parallelism));
    }

    if !update_delay.is_empty() {
      command.push_str(&format!(" --update-delay {}", update_delay));
    }

    if !update_failure_action.is_empty() {
      command.push_str(&format!(
        " --update-failure-action {}",
        update_failure_action
      ));
    }

    if !update_order.is_empty() {
      command.push_str(&format!(" --update-order {}", update_order));
    }

    if force {
      command.push_str(" --force");
    }

    command.push_str(&format!(" {}", name));

    Ok(run_komodo_command("Update Swarm Service", None, command).await)
  }
}

// ================
// REMOVE SERVICE
// ================

impl Resolve<super::Args> for RemoveSwarmService {
  #[instrument(name = "RemoveSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let RemoveSwarmService { name } = self;
    let command = format!("docker service rm {}", name);
    Ok(run_komodo_command("Remove Swarm Service", None, command).await)
  }
}

// ===============
// SCALE SERVICE
// ===============

impl Resolve<super::Args> for ScaleSwarmService {
  #[instrument(name = "ScaleSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let ScaleSwarmService { name, replicas } = self;
    let command = format!("docker service scale {}={}", name, replicas);
    Ok(run_komodo_command("Scale Swarm Service", None, command).await)
  }
}

// ================
// LIST SERVICES
// ================

impl Resolve<super::Args> for ListSwarmServices {
  #[instrument(name = "ListSwarmServices")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let command = String::from("docker service ls --format '{{json .}}'");
    Ok(run_komodo_command("List Swarm Services", None, command).await)
  }
}

// ==================
// INSPECT SERVICE
// ==================

impl Resolve<super::Args> for InspectSwarmService {
  #[instrument(name = "InspectSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let InspectSwarmService { name } = self;
    let command = format!("docker service inspect {}", name);
    Ok(
      run_komodo_command("Inspect Swarm Service", None, command).await,
    )
  }
}

// ================
// SERVICE LOGS
// ================

impl Resolve<super::Args> for GetSwarmServiceLogs {
  #[instrument(name = "GetSwarmServiceLogs")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let GetSwarmServiceLogs {
      name,
      tail,
      follow,
      timestamps,
    } = self;

    let mut command = format!("docker service logs");

    if !tail.is_empty() {
      command.push_str(&format!(" --tail {}", tail));
    }

    if follow {
      command.push_str(" --follow");
    }

    if timestamps {
      command.push_str(" --timestamps");
    }

    command.push_str(&format!(" {}", name));

    Ok(
      run_komodo_command("Get Swarm Service Logs", None, command).await,
    )
  }
}

// ======================
// ROLLBACK SERVICE
// ======================

impl Resolve<super::Args> for RollbackSwarmService {
  #[instrument(name = "RollbackSwarmService")]
  async fn resolve(self, _: &super::Args) -> serror::Result<Log> {
    let RollbackSwarmService { name } = self;
    let command = format!("docker service rollback {}", name);
    Ok(
      run_komodo_command("Rollback Swarm Service", None, command).await,
    )
  }
}
