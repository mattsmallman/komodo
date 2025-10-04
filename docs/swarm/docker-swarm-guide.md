# Docker Swarm Support in Komodo (Stack Deployment Mode)

Komodo now includes comprehensive support for deploying stacks to Docker Swarm with a focus on zero-downtime deployments. Docker Swarm is integrated as a deployment mode within the existing Stack resource, allowing you to manage your compose files through version control and deploy them to either standalone Docker or Docker Swarm.

## Features

- **Integrated with Stack Resource**: Use your existing compose files and deploy to Swarm mode
- **Version-Controlled Compose Files**: Full support for Git repos, just like regular stacks
- **Zero-Downtime Deployments**: Rolling updates with configurable parameters
- **Automatic Swarm Initialization**: Optionally auto-initialize swarm on deployment
- **Frontend and CLI Support**: Manage through UI or CLI

## Prerequisites

- Docker Engine 1.12 or later with Swarm mode support
- A server managed by Komodo
- Komodo v1.19 or later

## Quick Start

### 1. Create or Update a Stack for Swarm Mode

You can configure an existing stack or create a new one to use Swarm deployment mode.

**Via the Komodo UI:**
1. Navigate to your Stack
2. Set **Deploy Mode** to `swarm`
3. Configure **Swarm Settings**:
   - Update Parallelism: Number of tasks to update simultaneously (default: 1)
   - Update Delay: Time between updates (e.g., "10s", "1m")
   - Update Failure Action: pause, continue, or rollback
   - Update Order: stop-first or start-first
   - Auto Initialize Swarm: Enable to auto-init swarm if not active
4. Save and deploy

**Via compose file (already supported):**
Your existing compose files work with Swarm! Just add deploy configurations:

```yaml
version: '3.8'
services:
  web:
    image: nginx:latest
    deploy:
      replicas: 3
      update_config:
        parallelism: 2
        delay: 10s
        failure_action: rollback
        order: start-first
    ports:
      - "80:80"
```

### 2. Deploy the Stack

Once configured for swarm mode, deploy normally:

```bash
# Deploy stack (will use docker stack deploy instead of docker compose up)
komodo execute stack my-stack

# Or deploy if changed
komodo execute deploy-stack-if-changed my-stack
```

The Stack will automatically:
1. Check if Swarm is initialized (auto-init if configured)
2. Use `docker stack deploy` instead of `docker compose up`
3. Apply your configured update parameters for zero-downtime rolling updates

## Zero-Downtime Deployment

When you deploy a stack in Swarm mode, Docker orchestrates a rolling update:

### Configuration Options

- **Update Parallelism**: Controls how many tasks to update simultaneously (default: 1)
- **Update Delay**: Time to wait between batch updates (e.g., "10s", "30s", "1m")
- **Update Order**: 
  - `stop-first`: Stop old task before starting new (default, lower resource usage)
  - `start-first`: Start new task before stopping old (zero downtime, requires extra resources)
- **Failure Action**:
  - `pause`: Stop the update and wait for manual intervention
  - `continue`: Continue updating despite failures
  - `rollback`: Automatically rollback to previous version
- **Max Failure Ratio**: Tolerable failure rate (0.0 to 1.0)
- **Monitor Period**: Time to monitor after each update

### Example Stack Configuration

```toml
[[stack]]
name = "my-production-app"
server_id = "my-server"
deploy_mode = "swarm"

# Compose file from Git repo
repo = "owner/repo"
branch = "main"
file_paths = ["docker-compose.yml"]

# Swarm-specific settings
[stack.swarm_config]
update_parallelism = 2
update_delay = "30s"
update_failure_action = "rollback"
update_order = "start-first"
update_max_failure_ratio = 0.1
update_monitor = "5s"
auto_init_swarm = true
```

## Compose File Best Practices

When deploying to Swarm, you can use Docker Compose's deploy configuration:

```yaml
version: '3.8'

services:
  api:
    image: myapp:latest
    deploy:
      replicas: 5
      update_config:
        parallelism: 2
        delay: 30s
        failure_action: rollback
        order: start-first
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 1G
    ports:
      - "8080:8080"
    networks:
      - app-network

networks:
  app-network:
    driver: overlay
```

## Stack Management

All existing stack operations work seamlessly with Swarm mode:

### Deploying
```bash
# Regular deploy (uses swarm if deploy_mode is swarm)
komodo execute stack my-stack

# Deploy if changed (checks for updates first)
komodo execute deploy-stack-if-changed my-stack
```

### Viewing Logs
```bash
# View service logs (works in both compose and swarm modes)
komodo execute get-compose-log --project my-stack --tail 100
```

### Managing Services
```bash
# Start/stop/restart work the same
komodo execute start-stack my-stack
komodo execute restart-stack my-stack
komodo execute stop-stack my-stack
```

## Transitioning from Compose to Swarm

To transition an existing Stack from compose mode to swarm mode:

1. **Update the Stack Configuration**:
   - Set `deploy_mode: swarm`
   - Configure `swarm_config` with your desired update parameters
   - Enable `auto_init_swarm` if desired

2. **Update Your Compose File** (optional but recommended):
   - Add `deploy` sections to your services
   - Specify replica counts, update configs, resource limits
   - Use `overlay` networks for multi-host networking

3. **Deploy**:
   - The next deployment will use Docker Swarm
   - Services will be created with rolling update capabilities
   - Your application will benefit from zero-downtime updates

## Advanced Features

### Automatic Swarm Initialization

Set `auto_init_swarm: true` in your swarm config to automatically initialize Docker Swarm if it's not already active. This is useful for single-node setups where you want Swarm's rolling update features without manual initialization.

### Mixed Deployments

You can have some stacks using compose mode and others using swarm mode on the same server. Each stack operates independently based on its `deploy_mode` setting.

### Version Control Integration

All existing Git integration features work with Swarm mode:
- Linked repos
- Webhook triggers
- Auto-deploy on changes
- Compose file version control

## Troubleshooting

### Swarm Not Initialized

If you see "Docker Swarm is not initialized" and `auto_init_swarm` is false:

```bash
# Manually initialize swarm on your server
ssh your-server
docker swarm init
```

Or enable `auto_init_swarm: true` in your stack's swarm configuration.

### Services Not Updating

Check your update configuration:
- Ensure `update_parallelism` is > 0
- Verify `update_failure_action` is set appropriately
- Check service logs for errors

### Rollback Not Working

Rollbacks require a previous version. The first deployment cannot be rolled back. Subsequent deployments can rollback to the previous image/configuration.

## Differences Between Compose and Swarm Modes

| Feature | Compose Mode | Swarm Mode |
|---------|-------------|------------|
| Command | `docker compose up` | `docker stack deploy` |
| Zero-Downtime | No | Yes (rolling updates) |
| Scaling | Manual | Automatic orchestration |
| Health Checks | Basic | Advanced with retries |
| Multi-Node | No | Yes |
| Rollback | Manual | Automatic option |

## API Reference

All existing Stack operations work in Swarm mode:

- `DeployStack` - Deploy stack (uses swarm if configured)
- `DeployStackIfChanged` - Deploy if changes detected
- `StartStack` - Start services
- `RestartStack` - Restart services
- `StopStack` - Stop services
- `DestroyStack` - Remove stack/services

## Examples

### Simple Web Service

```yaml
version: '3.8'
services:
  web:
    image: nginx:alpine
    deploy:
      replicas: 3
      update_config:
        parallelism: 1
        delay: 10s
    ports:
      - "80:80"
```

Stack config:
```toml
[[stack]]
name = "web-service"
deploy_mode = "swarm"
file_contents = "..." # Or use repo
[stack.swarm_config]
auto_init_swarm = true
```

### Production API with Database

```yaml
version: '3.8'
services:
  api:
    image: myapi:latest
    deploy:
      replicas: 5
      update_config:
        parallelism: 2
        delay: 30s
        failure_action: rollback
        order: start-first
    environment:
      - DATABASE_URL=postgres://db:5432/myapp
    
  db:
    image: postgres:15
    deploy:
      replicas: 1
      placement:
        constraints:
          - node.role == manager
    volumes:
      - db-data:/var/lib/postgresql/data

volumes:
  db-data:

networks:
  default:
    driver: overlay
```

## Future Enhancements

- Multi-node swarm support
- Service constraints and placement preferences
- Advanced health check configuration
- Swarm secrets management
- Stack templates for common patterns

## Support

For issues or questions:
- GitHub Issues: https://github.com/mattsmallman/komodo/issues
- Discord: https://discord.gg/DRqE8Fvg5c
- Documentation: https://komo.do
