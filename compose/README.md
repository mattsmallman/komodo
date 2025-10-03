# Komodo Docker Compose Files

This directory contains Docker Compose files for deploying Komodo with different database backends and deployment methods.

## Standard Docker Compose Files

Use these files for deploying with `docker compose`:

- **`mongo.compose.yaml`** - Deploy with MongoDB backend
- **`ferretdb.compose.yaml`** - Deploy with FerretDB (Postgres-backed) backend  
- **`periphery.compose.yaml`** - Deploy standalone Periphery agent
- **`compose.env`** - Environment variables configuration

### Deploy with Docker Compose

```bash
# MongoDB backend
docker compose -p komodo -f compose/mongo.compose.yaml --env-file compose/compose.env up -d

# FerretDB backend
docker compose -p komodo -f compose/ferretdb.compose.yaml --env-file compose/compose.env up -d

# Standalone Periphery
docker compose -p komodo-periphery -f compose/periphery.compose.yaml up -d
```

## Docker Swarm Files

Use these files for deploying to Docker Swarm:

- **`mongo.swarm.compose.yaml`** - Swarm deployment with MongoDB backend
- **`ferretdb.swarm.compose.yaml`** - Swarm deployment with FerretDB backend
- **`periphery.swarm.compose.yaml`** - Swarm deployment for standalone Periphery

### Deploy with Docker Swarm

See the [Docker Swarm documentation](../docsite/docs/setup/docker-swarm.md) for complete setup instructions.

Quick start:

```bash
# Initialize Swarm (if not already done)
docker swarm init

# Create required secrets
echo "your_db_password" | docker secret create komodo_db_password -
echo "your_jwt_secret" | docker secret create komodo_jwt_secret -
echo "your_webhook_secret" | docker secret create komodo_webhook_secret -
echo "your_passkey" | docker secret create komodo_passkey -

# Create empty config (or provide your own)
echo "" | docker config create core_config -

# Deploy MongoDB stack
docker stack deploy -c compose/mongo.swarm.compose.yaml --compose-file compose/compose.env komodo

# Or deploy FerretDB stack
docker stack deploy -c compose/ferretdb.swarm.compose.yaml --compose-file compose/compose.env komodo
```

## Key Features

### Docker Compose
- Simple single-host deployment
- Quick development setup
- Standard restart policies

### Docker Swarm
- Multi-host clustering
- Built-in service discovery
- Rolling updates with rollback
- Secrets management
- Overlay networking
- Service replication and scaling
- Resource constraints
- Placement strategies

## Environment Variables

Configure your deployment by editing `compose.env`:

```bash
# Image version
COMPOSE_KOMODO_IMAGE_TAG=latest

# Database credentials
KOMODO_DB_USERNAME=admin
KOMODO_DB_PASSWORD=admin

# Authentication
KOMODO_PASSKEY=a_random_passkey

# Core settings
KOMODO_HOST=https://demo.komo.do
KOMODO_FIRST_SERVER=https://periphery:8120

# Periphery settings
PERIPHERY_ROOT_DIRECTORY=/etc/komodo
```

See `compose.env` for the complete list of available configuration options.

## Choosing a Deployment Method

### Use Docker Compose when:
- Running on a single host
- Development and testing
- Simple deployments
- Don't need high availability

### Use Docker Swarm when:
- Running on multiple hosts
- Need clustering and orchestration
- Require high availability
- Want rolling updates
- Need secrets management
- Require service scaling

## Documentation

- [Standard Setup Docs](https://komo.do/docs/setup)
- [Docker Swarm Setup](../docsite/docs/setup/docker-swarm.md)
- [Docker Compose Resource](../docsite/docs/resources/docker-compose.md)

## Support

- [GitHub Repository](https://github.com/moghtech/komodo)
- [Discord Community](https://discord.gg/DRqE8Fvg5c)
- [Documentation](https://komo.do)
