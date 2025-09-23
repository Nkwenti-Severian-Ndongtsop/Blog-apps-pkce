#!/bin/bash

# Custom Keycloak Theme Setup Script
# This script helps you set up and test the custom login theme

set -e

echo "🎨 Setting up Custom Keycloak Theme..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "docker-compose.yml" ]; then
    echo -e "${RED}❌ Error: docker-compose.yml not found. Please run this script from the Blog-apps directory.${NC}"
    exit 1
fi

# Check if custom theme exists
if [ ! -d "keycloak/themes/custom-login" ]; then
    echo -e "${RED}❌ Error: Custom theme directory not found at keycloak/themes/custom-login${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Custom theme directory found${NC}"

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo -e "${YELLOW}⚠️  Warning: .env file not found. Creating from env.example...${NC}"
    if [ -f "env.example" ]; then
        cp env.example .env
        echo -e "${GREEN}✅ Created .env file from env.example${NC}"
    else
        echo -e "${RED}❌ Error: env.example not found. Please create a .env file manually.${NC}"
        exit 1
    fi
fi

# Load environment variables
source .env

echo -e "${BLUE}📋 Current Configuration:${NC}"
echo -e "  Keycloak Port: ${KEYCLOAK_PORT:-8080}"
echo -e "  Keycloak Admin: ${KEYCLOAK_ADMIN:-admin}"
echo -e "  Database: ${POSTGRES_DB:-keycloak}"

# Stop any running containers
echo -e "${YELLOW}🛑 Stopping existing containers...${NC}"
docker-compose down

# Pull latest images
echo -e "${BLUE}📥 Pulling Docker images...${NC}"
docker-compose pull

# Start the services
echo -e "${BLUE}🚀 Starting services...${NC}"
docker-compose up -d

# Wait for Keycloak to be ready
echo -e "${YELLOW}⏳ Waiting for Keycloak to start...${NC}"
KEYCLOAK_URL="http://localhost:${KEYCLOAK_PORT:-8080}"

# Function to check if Keycloak is ready
check_keycloak() {
    curl -s -o /dev/null -w "%{http_code}" "$KEYCLOAK_URL/realms/master" 2>/dev/null
}

# Wait up to 2 minutes for Keycloak to start
TIMEOUT=120
ELAPSED=0
while [ $ELAPSED -lt $TIMEOUT ]; do
    if [ "$(check_keycloak)" = "200" ]; then
        echo -e "${GREEN}✅ Keycloak is ready!${NC}"
        break
    fi
    echo -e "${YELLOW}⏳ Still waiting... (${ELAPSED}s/${TIMEOUT}s)${NC}"
    sleep 5
    ELAPSED=$((ELAPSED + 5))
done

if [ $ELAPSED -ge $TIMEOUT ]; then
    echo -e "${RED}❌ Timeout waiting for Keycloak to start${NC}"
    echo -e "${YELLOW}💡 Check logs with: docker-compose logs keycloak${NC}"
    exit 1
fi

echo -e "${GREEN}🎉 Setup Complete!${NC}"
echo ""
echo -e "${BLUE}📝 Next Steps:${NC}"
echo -e "1. Open Keycloak Admin Console: ${KEYCLOAK_URL}/admin"
echo -e "2. Login with: ${KEYCLOAK_ADMIN:-admin} / ${KEYCLOAK_ADMIN_PASSWORD:-admin}"
echo -e "3. Go to your realm → Realm Settings → Themes"
echo -e "4. Set Login Theme to: 'Custom Login Theme'"
echo -e "5. Click Save"
echo ""
echo -e "${BLUE}🔗 Useful URLs:${NC}"
echo -e "  Admin Console: ${KEYCLOAK_URL}/admin"
echo -e "  Realm Login: ${KEYCLOAK_URL}/realms/master/account"
echo ""
echo -e "${BLUE}🛠️  Useful Commands:${NC}"
echo -e "  View logs: ${YELLOW}docker-compose logs -f keycloak${NC}"
echo -e "  Restart: ${YELLOW}docker-compose restart keycloak${NC}"
echo -e "  Stop all: ${YELLOW}docker-compose down${NC}"
echo ""
echo -e "${GREEN}✨ Your custom theme is now available in Keycloak!${NC}"
