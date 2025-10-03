# Docker Swarm Deployment

Komodo can be deployed to Docker Swarm for enhanced clustering, service discovery, and built-in orchestration capabilities. This guide will walk you through initializing a Docker Swarm cluster, deploying the Komodo stack, and managing services.

## Overview

Docker Swarm provides:
- **Native clustering** - Manage multiple Docker hosts as a single cluster
- **Service discovery** - Built-in DNS-based service discovery
- **Load balancing** - Automatic load balancing across service replicas
- **Rolling updates** - Zero-downtime deployments with automatic rollback
- **Secrets management** - Secure handling of sensitive data
- **Overlay networking** - Multi-host networking for service communication

## Prerequisites

- Docker Engine 20.10.0 or later
- Multiple nodes (optional, but recommended for production)
- Manager nodes should have stable IP addresses or hostnames

## Initialize Docker Swarm

### Single Node Setup (Development/Testing)

Initialize Swarm on your Docker host:

```bash
docker swarm init
```

This will output a join token that can be used to add worker nodes later.

### Multi-Node Setup (Production)

On the manager node:

```bash
# Initialize swarm with advertise address (use your manager IP)
docker swarm init --advertise-addr <MANAGER-IP>
```

On each worker node:

```bash
# Use the join token from the init command output
docker swarm join --token <WORKER-TOKEN> <MANAGER-IP>:2377
```

To add additional manager nodes (for high availability):

```bash
# Get the manager join token from an existing manager
docker swarm join-token manager

# On the new manager node
docker swarm join --token <MANAGER-TOKEN> <MANAGER-IP>:2377
```

### Verify Swarm Status

```bash
# List all nodes in the swarm
docker node ls

# Inspect swarm
docker info | grep Swarm
```

## Create Docker Secrets

Docker Swarm secrets provide secure storage for sensitive data. Create the required secrets before deploying:

```bash
# Create database password secret
echo "your_secure_db_password" | docker secret create komodo_db_password -

# Create JWT secret for token signing
echo "your_secure_jwt_secret" | docker secret create komodo_jwt_secret -

# Create webhook secret
echo "your_secure_webhook_secret" | docker secret create komodo_webhook_secret -

# Create Komodo passkey for Core-Periphery authentication
echo "your_secure_passkey" | docker secret create komodo_passkey -
```

### List and Inspect Secrets

```bash
# List all secrets
docker secret ls

# Inspect a secret (content is not shown)
docker secret inspect komodo_db_password
```

## Create Docker Configs

Configs allow you to store non-sensitive configuration files:

```bash
# Create a core config file (optional, uses defaults if not provided)
docker config create core_config /path/to/core.config.toml
```

If you don't have a custom config, you can create an empty one:

```bash
echo "" | docker config create core_config -
```

## Configure Environment Variables

Before deploying, configure the environment variables in `compose/compose.env`:

```bash
cd /path/to/komodo
cp compose/compose.env compose/compose.env.swarm
nano compose/compose.env.swarm
```

Key variables to configure:
- `COMPOSE_KOMODO_IMAGE_TAG` - Version tag (e.g., `latest` or `1.19.5`)
- `KOMODO_DB_USERNAME` - Database username
- `KOMODO_DB_PASSWORD` - Database password
- `KOMODO_HOST` - Public URL for the Komodo instance
- `KOMODO_PASSKEY` - Passkey for Core-Periphery authentication
- `PERIPHERY_ROOT_DIRECTORY` - Periphery working directory

## Deploy the Stack

### MongoDB Backend

Deploy Komodo with MongoDB:

```bash
docker stack deploy -c compose/mongo.swarm.compose.yaml --compose-file compose/compose.env.swarm komodo
```

### FerretDB Backend

Deploy Komodo with FerretDB (Postgres-backed MongoDB alternative):

```bash
docker stack deploy -c compose/ferretdb.swarm.compose.yaml --compose-file compose/compose.env.swarm komodo
```

### Standalone Periphery

Deploy only Periphery on worker nodes:

```bash
docker stack deploy -c compose/periphery.swarm.compose.yaml komodo-periphery
```

## Manage Services

### List Services

```bash
# List all services in the stack
docker stack services komodo

# Detailed service information
docker service ls
```

### Inspect Service

```bash
# Get detailed service information
docker service inspect komodo_core

# View service logs
docker service logs komodo_core

# Follow logs in real-time
docker service logs -f komodo_core
```

### Scale Services

Scale services up or down:

```bash
# Scale core to 2 replicas
docker service scale komodo_core=2

# Scale multiple services
docker service scale komodo_core=2 komodo_periphery=3
```

Note: For stateful services like databases, scaling may require additional configuration.

### Update Services

Update service configuration or image:

```bash
# Update service image
docker service update --image ghcr.io/moghtech/komodo-core:1.19.5 komodo_core

# Update environment variable
docker service update --env-add KOMODO_MONITORING_INTERVAL=5-sec komodo_core

# Update resource limits
docker service update --limit-cpu 2 --limit-memory 2G komodo_core
```

### Rolling Updates

Services are configured with rolling update strategies:

```bash
# Trigger an update (Swarm will perform rolling update automatically)
docker service update --force komodo_core
```

The update configuration in the compose files ensures:
- Updates happen one replica at a time (`parallelism: 1`)
- 10-second delay between updates
- Automatic rollback on failure
- Start new containers before stopping old ones (for core service)

### Rollback Service

If an update fails or causes issues:

```bash
# Rollback to previous version
docker service rollback komodo_core
```

## View Service Status

### List Tasks (Container instances)

```bash
# List all tasks for a service
docker service ps komodo_core

# Include stopped/failed tasks
docker service ps --no-trunc komodo_core

# Filter by desired state
docker service ps --filter "desired-state=running" komodo_core
```

### Node Information

```bash
# List nodes
docker node ls

# Inspect node
docker node inspect <NODE-ID>

# View tasks running on a node
docker node ps <NODE-ID>
```

## Remove/Update Stack

### Update Stack

To update the stack configuration:

```bash
# Edit the compose file, then redeploy
docker stack deploy -c compose/mongo.swarm.compose.yaml --compose-file compose/compose.env.swarm komodo
```

Swarm will detect changes and update only the affected services.

### Remove Stack

```bash
# Remove entire stack
docker stack rm komodo

# Verify removal
docker stack ls
```

Note: Volumes are not automatically removed. Remove them manually if needed:

```bash
# List volumes
docker volume ls

# Remove volumes
docker volume rm komodo_mongo-data komodo_mongo-config
```

## Networking

The Swarm compose files use overlay networks for multi-host communication:

```bash
# List networks
docker network ls

# Inspect the Komodo network
docker network inspect komodo_komodo_network

# Services can communicate using service names as DNS
# Example: mongo:27017, core:9120, periphery:8120
```

## Access Services

### Internal Access

Services within the swarm can access each other using service names:
- MongoDB: `mongo:27017`
- FerretDB: `ferretdb:27017`
- Komodo Core: `core:9120`
- Periphery: `periphery:8120`

### External Access

To expose services externally:

1. **Port Publishing**: Services are published using `mode: host` for direct node access
   - Core: `http://<NODE-IP>:9120`
   - Periphery: `https://<NODE-IP>:8120`

2. **Load Balancer**: Use an external load balancer (e.g., Traefik, Nginx) to distribute traffic across nodes

3. **Ingress Network**: Swarm's built-in ingress network routes traffic to available service replicas

## Monitoring and Health

### Check Service Health

```bash
# View service status
docker service ls

# Check service logs for errors
docker service logs --tail 100 komodo_core

# View resource usage
docker stats $(docker ps -q)
```

### Stack Events

```bash
# View stack events
docker stack ps komodo

# Filter for failed tasks
docker stack ps --filter "desired-state=failed" komodo
```

## Backup and Restore

### Backup Volumes

```bash
# Stop the stack
docker stack rm komodo

# Backup volumes
docker run --rm -v komodo_mongo-data:/data -v $(pwd):/backup alpine \
  tar czf /backup/mongo-data-backup.tar.gz -C /data .

# Restart the stack
docker stack deploy -c compose/mongo.swarm.compose.yaml --compose-file compose/compose.env.swarm komodo
```

### Restore Volumes

```bash
# Stop the stack
docker stack rm komodo

# Restore volumes
docker run --rm -v komodo_mongo-data:/data -v $(pwd):/backup alpine \
  tar xzf /backup/mongo-data-backup.tar.gz -C /data

# Restart the stack
docker stack deploy -c compose/mongo.swarm.compose.yaml --compose-file compose/compose.env.swarm komodo
```

## Troubleshooting

### Service Won't Start

```bash
# Check service status
docker service ps komodo_core

# View error logs
docker service logs komodo_core --tail 50

# Inspect service configuration
docker service inspect komodo_core
```

### Network Issues

```bash
# Test connectivity between services
docker exec $(docker ps -q -f name=komodo_core) ping mongo

# Inspect network
docker network inspect komodo_komodo_network
```

### Secret Access Issues

```bash
# Verify secrets exist
docker secret ls

# Check service has access to secrets
docker service inspect komodo_core --format '{{ .Spec.TaskTemplate.ContainerSpec.Secrets }}'
```

### Node Issues

```bash
# Check node status
docker node ls

# Drain node for maintenance
docker node update --availability drain <NODE-ID>

# Return node to active
docker node update --availability active <NODE-ID>
```

## Production Best Practices

1. **High Availability**: Deploy at least 3 manager nodes for quorum
2. **Resource Limits**: Set appropriate CPU and memory limits for all services
3. **Secrets Rotation**: Regularly rotate secrets and use external secret management
4. **Monitoring**: Implement monitoring with Prometheus/Grafana
5. **Logging**: Use centralized logging (e.g., ELK stack, Loki)
6. **Backups**: Automate regular backups of volumes and configurations
7. **Updates**: Test updates in staging before production deployment
8. **Security**: Use TLS for Swarm communication and keep Docker up to date

## Differences from Docker Compose

Key differences when using Docker Swarm:

1. **Deploy Section**: Swarm uses `deploy` instead of `restart` policies
2. **Secrets**: Must be created separately and referenced as external
3. **Configs**: Similar to secrets but for non-sensitive data
4. **Networks**: Overlay networks enable multi-host communication
5. **Scaling**: Built-in service replication and load balancing
6. **Updates**: Rolling updates with automatic rollback support
7. **Placement**: Control which nodes run which services

## Additional Resources

- [Docker Swarm Documentation](https://docs.docker.com/engine/swarm/)
- [Docker Stack Deploy Reference](https://docs.docker.com/engine/reference/commandline/stack_deploy/)
- [Docker Service Management](https://docs.docker.com/engine/swarm/services/)
- [Komodo Documentation](https://komo.do)

## Support

For issues or questions:
- [GitHub Issues](https://github.com/moghtech/komodo/issues)
- [Discord Community](https://discord.gg/DRqE8Fvg5c)
