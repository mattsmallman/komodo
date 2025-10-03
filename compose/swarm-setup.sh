#!/bin/bash
# Komodo Docker Swarm Quick Setup Script
# This script helps initialize a Docker Swarm cluster and deploy Komodo

set -e

echo "🦎 Komodo Docker Swarm Quick Setup 🦎"
echo "======================================"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if running in swarm mode
if ! docker info | grep -q "Swarm: active"; then
    echo "📋 Docker Swarm is not initialized."
    read -p "Do you want to initialize Docker Swarm? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🔧 Initializing Docker Swarm..."
        docker swarm init
        echo "✅ Docker Swarm initialized successfully!"
    else
        echo "❌ Docker Swarm is required. Exiting."
        exit 1
    fi
else
    echo "✅ Docker Swarm is already active"
fi

echo ""
echo "📋 Creating Docker Secrets..."

# Check and create secrets
create_secret() {
    local secret_name=$1
    local prompt=$2
    local default_value=$3
    
    if docker secret inspect "$secret_name" &> /dev/null; then
        echo "  ℹ️  Secret '$secret_name' already exists, skipping..."
    else
        read -sp "$prompt [$default_value]: " secret_value
        echo
        if [ -z "$secret_value" ]; then
            secret_value=$default_value
        fi
        echo "$secret_value" | docker secret create "$secret_name" -
        echo "  ✅ Created secret: $secret_name"
    fi
}

create_secret "komodo_db_password" "Enter database password" "admin"
create_secret "komodo_jwt_secret" "Enter JWT secret" "$(openssl rand -base64 32)"
create_secret "komodo_webhook_secret" "Enter webhook secret" "$(openssl rand -base64 32)"
create_secret "komodo_passkey" "Enter Komodo passkey" "a_random_passkey"

echo ""
echo "📋 Creating Docker Configs..."

# Create core config if it doesn't exist
if docker config inspect core_config &> /dev/null; then
    echo "  ℹ️  Config 'core_config' already exists, skipping..."
else
    echo "" | docker config create core_config -
    echo "  ✅ Created config: core_config (empty, using defaults)"
fi

echo ""
echo "📋 Select deployment backend:"
echo "  1) MongoDB"
echo "  2) FerretDB (Postgres-backed)"
read -p "Enter your choice (1 or 2): " backend_choice

case $backend_choice in
    1)
        COMPOSE_FILE="compose/mongo.swarm.compose.yaml"
        echo "  Selected: MongoDB"
        ;;
    2)
        COMPOSE_FILE="compose/ferretdb.swarm.compose.yaml"
        echo "  Selected: FerretDB"
        ;;
    *)
        echo "❌ Invalid choice. Exiting."
        exit 1
        ;;
esac

echo ""
echo "🚀 Deploying Komodo stack..."

# Check if compose.env exists
if [ ! -f "compose/compose.env" ]; then
    echo "❌ compose/compose.env not found. Please create it first."
    exit 1
fi

# Deploy the stack
docker stack deploy -c "$COMPOSE_FILE" --compose-file compose/compose.env komodo

echo ""
echo "✅ Komodo stack deployment initiated!"
echo ""
echo "📊 Check deployment status:"
echo "  docker stack services komodo"
echo "  docker service ls"
echo "  docker service logs -f komodo_core"
echo ""
echo "🌐 Access Komodo at: http://localhost:9120"
echo "   (Wait a few moments for services to start)"
echo ""
echo "📚 Documentation: https://komo.do"
echo "💬 Support: https://discord.gg/DRqE8Fvg5c"
