# Docker Swarm Support in Komodo

Komodo now includes comprehensive support for managing Docker Swarm clusters with a focus on zero-downtime deployments. This guide covers setup, basic usage, and zero-downtime deployment strategies.

## Features

- **Single-Node Swarm Management**: Initialize and manage single-server Docker Swarm setups
- **Zero-Downtime Deployments**: Rolling updates with configurable parameters
- **Service Management**: Create, update, scale, and remove services
- **Logs and Monitoring**: View service logs and monitor deployment status
- **Rollback Support**: Quickly rollback failed deployments

## Prerequisites

- Docker Engine 1.12 or later
- A server managed by Komodo
- Komodo v1.19 or later

## Quick Start

### 1. Create a Swarm Resource

Using the Komodo CLI:

```bash
# This will be available once write operations are implemented
komodo create swarm my-swarm --server my-server
```

Or via the Komodo UI (when implemented):
1. Navigate to Resources > Swarms
2. Click "New Swarm"
3. Configure swarm settings
4. Save

### 2. Initialize the Swarm

```bash
komodo execute swarm-init my-swarm
```

This initializes Docker Swarm on the target server. The swarm will be configured with:
- Single manager node (the target server)
- Default network configuration
- Ready to accept service deployments

### 3. Deploy a Service

```bash
komodo execute swarm-deploy my-swarm \
  --service-name web-app \
  --image nginx:latest \
  --replicas 3 \
  --ports 80:80 \
  --update-parallelism 1 \
  --update-delay 10s \
  --update-failure-action rollback
```

### 4. Update a Service (Zero-Downtime)

```bash
komodo execute swarm-update my-swarm \
  --service-name web-app \
  --image nginx:alpine \
  --update-parallelism 2 \
  --update-delay 10s \
  --update-order start-first
```

## Zero-Downtime Deployment

Komodo's Swarm support is designed specifically for zero-downtime deployments. Here's how it works:

### Rolling Update Strategy

When you update a service, Docker Swarm updates containers in batches:

1. **Parallelism**: Controls how many tasks to update simultaneously
2. **Delay**: Time to wait between batch updates
3. **Update Order**: 
   - `stop-first`: Stop old task before starting new (default)
   - `start-first`: Start new task before stopping old (requires extra resources)

### Failure Handling

Configure how the system responds to failures during deployment:

- **pause**: Stop the update and wait for manual intervention
- **continue**: Continue updating despite failures
- **rollback**: Automatically rollback to previous version

### Example: Production Deployment

```bash
komodo execute swarm-update my-swarm \
  --service-name api-service \
  --image myapp:v2.0 \
  --update-parallelism 2 \
  --update-delay 30s \
  --update-failure-action rollback \
  --update-order start-first
```

This configuration:
- Updates 2 containers at a time
- Waits 30 seconds between batches
- Starts new containers before stopping old ones
- Automatically rolls back if failures are detected

## Service Management

### Scaling Services

```bash
# Scale up
komodo execute swarm-scale my-swarm --service-name web-app --replicas 5

# Scale down
komodo execute swarm-scale my-swarm --service-name web-app --replicas 2
```

### Viewing Logs

```bash
# Get last 100 lines
komodo execute swarm-logs my-swarm --service-name web-app --tail 100

# Get all logs
komodo execute swarm-logs my-swarm --service-name web-app
```

### Rollback

If a deployment goes wrong, rollback to the previous version:

```bash
komodo execute swarm-rollback my-swarm --service-name web-app
```

### Remove Service

```bash
komodo execute swarm-remove my-swarm --service-name web-app
```

## Advanced Configuration

### Custom Network Configuration

When creating a swarm, you can specify custom network settings:

```bash
# With custom address pools (when write API is implemented)
komodo create swarm my-swarm \
  --server my-server \
  --advertise-addr 192.168.1.100 \
  --listen-addr 0.0.0.0:2377 \
  --default-addr-pool 10.20.0.0/16 \
  --default-addr-pool 10.21.0.0/16
```

### Resource Limits

Services can have resource limits and reservations:

```yaml
# Via configuration (when UI is implemented)
resources:
  limits:
    cpus: "2"
    memory: 2G
  reservations:
    cpus: "1"
    memory: 1G
```

### Health Checks

Configure health checks for your services:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost/health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s
```

## Best Practices

1. **Start Small**: Begin with `parallelism: 1` and increase as you gain confidence
2. **Monitor First Updates**: Watch the first deployment carefully
3. **Use Start-First Order**: For critical services, use `start-first` to avoid downtime
4. **Set Appropriate Delays**: Allow time for health checks between updates
5. **Enable Rollback**: Always set `update-failure-action: rollback` for production
6. **Resource Reservations**: Set resource reservations to ensure service quality
7. **Health Checks**: Always configure health checks for automatic failure detection

## Troubleshooting

### Swarm Won't Initialize

Check that:
- Docker is running on the target server
- Port 2377 (swarm management) is available
- The server has network connectivity

### Service Update Fails

Common causes:
- Image not found or not accessible
- Insufficient resources on the node
- Port conflicts
- Health check failures

Check service logs:
```bash
komodo execute swarm-logs my-swarm --service-name my-service --tail 200
```

### Rollback Not Working

Ensure the service has a previous version to rollback to. The first deployment cannot be rolled back.

## API Reference

### Swarm Operations

- `InitSwarm` - Initialize Docker Swarm
- `LeaveSwarm` - Leave Docker Swarm
- `DeploySwarmService` - Deploy new service
- `UpdateSwarmService` - Update existing service
- `RemoveSwarmService` - Remove service
- `ScaleSwarmService` - Scale service replicas
- `GetSwarmServiceLogs` - View service logs
- `RollbackSwarmService` - Rollback service

### CLI Aliases

- `swarm-init` → `InitSwarm`
- `swarm-leave` → `LeaveSwarm`
- `swarm-deploy` → `DeploySwarmService`
- `swarm-update` → `UpdateSwarmService`
- `swarm-remove` → `RemoveSwarmService`
- `swarm-scale` → `ScaleSwarmService`
- `swarm-logs` → `GetSwarmServiceLogs`
- `swarm-rollback` → `RollbackSwarmService`

## Examples

### Deploy a Web Application

```bash
# Deploy with 3 replicas and load balancing
komodo execute swarm-deploy my-swarm \
  --service-name webapp \
  --image myapp:latest \
  --replicas 3 \
  --ports 80:8080 \
  --env APP_ENV=production \
  --env DATABASE_URL=postgres://... \
  --update-parallelism 1 \
  --update-delay 10s \
  --update-failure-action rollback
```

### Zero-Downtime Update

```bash
# Update to new version with zero downtime
komodo execute swarm-update my-swarm \
  --service-name webapp \
  --image myapp:v2.0 \
  --update-parallelism 2 \
  --update-delay 30s \
  --update-order start-first \
  --update-failure-action rollback
```

### Scale for Traffic

```bash
# Handle increased traffic
komodo execute swarm-scale my-swarm \
  --service-name webapp \
  --replicas 10
```

## Future Enhancements

- Multi-node swarm support
- Swarm service templates
- Automatic health check configuration
- Integration with CI/CD pipelines
- Swarm network management
- Volume management for stateful services
- Service constraints and placement preferences

## Support

For issues or questions:
- GitHub Issues: https://github.com/mattsmallman/komodo/issues
- Discord: https://discord.gg/DRqE8Fvg5c
- Documentation: https://komo.do

## Contributing

Contributions are welcome! See the implementation summary for areas that need work.
