# Docker Swarm Architecture Examples

This document provides examples of common Docker Swarm commands and their expected outputs when working with the Komodo stack.

## Stack Deployment

```bash
$ docker stack deploy -c compose/mongo.swarm.compose.yaml komodo
Creating network komodo_komodo_network
Creating service komodo_mongo
Creating service komodo_core
Creating service komodo_periphery
```

## Viewing Services

```bash
$ docker stack services komodo
ID             NAME               MODE         REPLICAS   IMAGE                                         PORTS
abc123def456   komodo_core        replicated   1/1        ghcr.io/moghtech/komodo-core:latest           *:9120->9120/tcp
def456ghi789   komodo_mongo       replicated   1/1        mongo:latest                                  
ghi789jkl012   komodo_periphery   global       3/3        ghcr.io/moghtech/komodo-periphery:latest      *:8120->8120/tcp
```

The output shows:
- `MODE`: `replicated` (fixed number) or `global` (one per node)
- `REPLICAS`: Current/Desired replica count
- `PORTS`: Published ports accessible externally

## Service Details

```bash
$ docker service ps komodo_core
ID             NAME              IMAGE                                    NODE       DESIRED STATE   CURRENT STATE
xyz123abc456   komodo_core.1     ghcr.io/moghtech/komodo-core:latest     manager1   Running         Running 5 minutes ago
```

## Network Configuration

```bash
$ docker network ls
NETWORK ID     NAME                    DRIVER    SCOPE
abc123def456   komodo_komodo_network   overlay   swarm

$ docker network inspect komodo_komodo_network --format '{{json .IPAM.Config}}'
[{"Subnet":"10.0.9.0/24","Gateway":"10.0.9.1"}]
```

## Secrets Management

```bash
$ docker secret ls
ID                          NAME                    DRIVER    CREATED          UPDATED
abc123def456abc123def456    komodo_db_password                2 minutes ago    2 minutes ago
def456ghi789def456ghi789    komodo_jwt_secret                 2 minutes ago    2 minutes ago
ghi789jkl012ghi789jkl012    komodo_webhook_secret             2 minutes ago    2 minutes ago
jkl012mno345jkl012mno345    komodo_passkey                    2 minutes ago    2 minutes ago
```

## Scaling Services

```bash
$ docker service scale komodo_core=3
komodo_core scaled to 3
overall progress: 3 out of 3 tasks 
1/3: running   [==================================================>] 
2/3: running   [==================================================>] 
3/3: running   [==================================================>] 
verify: Service converged
```

## Service Logs

```bash
$ docker service logs komodo_core --tail 20
komodo_core.1.xyz123abc456@manager1    | 2025-10-03T22:30:00Z  INFO core: Komodo Core version: v1.19.5
komodo_core.1.xyz123abc456@manager1    | 2025-10-03T22:30:01Z  INFO core: Server started on port 9120
komodo_core.2.def456ghi789@worker1     | 2025-10-03T22:30:05Z  INFO core: Komodo Core version: v1.19.5
komodo_core.2.def456ghi789@worker1     | 2025-10-03T22:30:06Z  INFO core: Server started on port 9120
```

## Rolling Updates

```bash
$ docker service update --image ghcr.io/moghtech/komodo-core:v1.19.6 komodo_core
komodo_core
overall progress: 3 out of 3 tasks 
1/3: running   [==================================================>] 
2/3: running   [==================================================>] 
3/3: running   [==================================================>] 
verify: Service converged
```

The update will:
1. Update one replica at a time (parallelism: 1)
2. Wait 10 seconds between updates
3. Automatically rollback on failure

## Node Management

```bash
$ docker node ls
ID                            HOSTNAME    STATUS    AVAILABILITY   MANAGER STATUS   ENGINE VERSION
abc123def456xyz123def456 *    manager1    Ready     Active         Leader           24.0.0
def456ghi789abc123def456      worker1     Ready     Active                          24.0.0
ghi789jkl012def456ghi789      worker2     Ready     Active                          24.0.0
```

## Stack Visualization

```
┌─────────────────────────────────────────────────────────┐
│                    Docker Swarm Cluster                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Manager    │  │   Worker 1   │  │   Worker 2   │  │
│  │              │  │              │  │              │  │
│  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────┐ │  │
│  │ │  Core    │ │  │ │Periphery │ │  │ │Periphery │ │  │
│  │ │ Replica  │ │  │ │  (Global) │ │  │ │  (Global) │ │  │
│  │ └──────────┘ │  │ └──────────┘ │  │ └──────────┘ │  │
│  │              │  │              │  │              │  │
│  │ ┌──────────┐ │  │              │  │              │  │
│  │ │ MongoDB  │ │  │              │  │              │  │
│  │ └──────────┘ │  │              │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│         │                  │                  │         │
│         └──────────────────┼──────────────────┘         │
│                            │                            │
│                 ┌──────────▼──────────┐                 │
│                 │  Overlay Network    │                 │
│                 │   10.0.9.0/24       │                 │
│                 └─────────────────────┘                 │
│                                                          │
└─────────────────────────────────────────────────────────┘

Secrets (Encrypted):
  • komodo_db_password
  • komodo_jwt_secret
  • komodo_webhook_secret
  • komodo_passkey

External Access:
  • Core: http://<any-node-ip>:9120
  • Periphery: https://<any-node-ip>:8120
```

## Health Check

```bash
$ docker stack ps komodo --filter "desired-state=running"
ID             NAME                                         IMAGE                                    NODE       DESIRED STATE   CURRENT STATE
abc123def456   komodo_core.1                                ghcr.io/moghtech/komodo-core:latest     manager1   Running         Running 10 minutes ago
def456ghi789   komodo_mongo.1                               mongo:latest                            manager1   Running         Running 10 minutes ago
ghi789jkl012   komodo_periphery.abc123def456xyz123def456    ghcr.io/moghtech/komodo-periphery:la... manager1   Running         Running 10 minutes ago
jkl012mno345   komodo_periphery.def456ghi789abc123def456    ghcr.io/moghtech/komodo-periphery:la... worker1    Running         Running 10 minutes ago
mno345pqr678   komodo_periphery.ghi789jkl012def456ghi789    ghcr.io/moghtech/komodo-periphery:la... worker2    Running         Running 10 minutes ago
```

All services showing "Running" indicates a healthy deployment.

## Resource Usage

```bash
$ docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"
CONTAINER                                       CPU %     MEM USAGE / LIMIT
komodo_core.1.xyz123abc456                      2.5%      512MiB / 1GiB
komodo_mongo.1.def456ghi789                     1.2%      256MiB / 512MiB
komodo_periphery.abc123def456.ghi789jkl012      0.5%      128MiB / 512MiB
```

## Cleanup

```bash
$ docker stack rm komodo
Removing service komodo_core
Removing service komodo_mongo
Removing service komodo_periphery
Removing network komodo_komodo_network

$ docker volume prune
WARNING! This will remove all local volumes not used by at least one container.
Are you sure you want to continue? [y/N] y
Deleted Volumes:
komodo_mongo-data
komodo_mongo-config
Total reclaimed space: 2.5GB
```
