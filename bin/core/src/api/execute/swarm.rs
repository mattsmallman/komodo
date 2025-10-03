use anyhow::Context;
use formatting::format_serror;
use komodo_client::{
  api::execute::*,
  entities::{
    permission::PermissionLevel,
    swarm::Swarm,
    update::{Log, Update},
    user::User,
  },
};
use periphery_client::api::swarm::*;
use resolver_api::Resolve;
use tracing::instrument;

use crate::{
  helpers::{
    periphery_client,
    update::update_update,
  },
  monitor::update_cache_for_server,
  permission::get_check_permissions,
  resource,
};

use super::ExecuteArgs;

async fn get_swarm_and_server(
  swarm: &str,
  user: &User,
  permission_level: PermissionLevel,
) -> anyhow::Result<(Swarm, String)> {
  let swarm = resource::get_check_permissions::<Swarm>(
    swarm,
    user,
    permission_level,
  )
  .await?;

  Ok((swarm.clone(), swarm.config.server_id.clone()))
}

impl Resolve<ExecuteArgs> for InitSwarm {
  #[instrument(name = "InitSwarm", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (swarm, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::InitSwarm {
        advertise_addr: swarm.config.advertise_addr.clone(),
        listen_addr: swarm.config.listen_addr.clone(),
        data_path_addr: swarm.config.data_path_addr.clone(),
        default_addr_pool: swarm.config.default_addr_pool.clone(),
        default_addr_pool_mask_length: swarm
          .config
          .default_addr_pool_mask_length,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "init swarm",
        format_serror(&e.context("failed to init swarm").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for LeaveSwarm {
  #[instrument(name = "LeaveSwarm", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::LeaveSwarm {
        force: self.force,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "leave swarm",
        format_serror(&e.context("failed to leave swarm").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for DeploySwarmService {
  #[instrument(name = "DeploySwarmService", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::CreateSwarmService {
        name: self.service_name.clone(),
        image: self.image.clone(),
        replicas: self.replicas.unwrap_or(1),
        env: self.env.clone(),
        mounts: self.mounts.clone(),
        networks: self.networks.clone(),
        ports: self.ports.clone(),
        update_parallelism: self.update_parallelism.unwrap_or(1),
        update_delay: self.update_delay.clone(),
        update_failure_action: self.update_failure_action.clone(),
        update_order: self.update_order.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "deploy swarm service",
        format_serror(
          &e.context("failed to deploy swarm service").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for UpdateSwarmService {
  #[instrument(name = "UpdateSwarmService", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::UpdateSwarmService {
        name: self.service_name.clone(),
        image: self.image.clone(),
        replicas: self.replicas,
        env_add: self.env_add.clone(),
        env_rm: self.env_rm.clone(),
        mount_add: Vec::new(),
        mount_rm: Vec::new(),
        network_add: Vec::new(),
        network_rm: Vec::new(),
        publish_add: Vec::new(),
        publish_rm: Vec::new(),
        update_parallelism: self.update_parallelism,
        update_delay: self.update_delay.clone(),
        update_failure_action: self.update_failure_action.clone(),
        update_order: self.update_order.clone(),
        force: self.force,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "update swarm service",
        format_serror(
          &e.context("failed to update swarm service").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for RemoveSwarmService {
  #[instrument(name = "RemoveSwarmService", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::RemoveSwarmService {
        name: self.service_name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "remove swarm service",
        format_serror(
          &e.context("failed to remove swarm service").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for ScaleSwarmService {
  #[instrument(name = "ScaleSwarmService", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::ScaleSwarmService {
        name: self.service_name.clone(),
        replicas: self.replicas,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "scale swarm service",
        format_serror(
          &e.context("failed to scale swarm service").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for GetSwarmServiceLogs {
  #[instrument(name = "GetSwarmServiceLogs", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::GetSwarmServiceLogs {
        name: self.service_name.clone(),
        tail: self.tail.clone(),
        follow: false,
        timestamps: false,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "get swarm service logs",
        format_serror(
          &e.context("failed to get swarm service logs").into(),
        ),
      ),
    };

    update.logs.push(log);

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<ExecuteArgs> for RollbackSwarmService {
  #[instrument(name = "RollbackSwarmService", skip(user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    self,
    ExecuteArgs { user, update }: &ExecuteArgs,
  ) -> serror::Result<Update> {
    let (_, server_id) = get_swarm_and_server(
      &self.swarm,
      user,
      PermissionLevel::Execute,
    )
    .await?;

    let periphery = periphery_client(&server_id)?;

    let log = match periphery
      .request(periphery_client::api::swarm::RollbackSwarmService {
        name: self.service_name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "rollback swarm service",
        format_serror(
          &e.context("failed to rollback swarm service").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server_id, true).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}
