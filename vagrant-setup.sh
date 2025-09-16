#!/bin/bash

# Blog Apps Vagrant Setup Script
# This script helps you get started with the Vagrant environment

set -e

echo "🚀 Blog Apps Vagrant Setup"
echo "=========================="

# Check if Vagrant is installed
if ! command -v vagrant &> /dev/null; then
    echo "❌ Vagrant is not installed. Please install Vagrant first:"
    echo "   https://www.vagrantup.com/downloads"
    exit 1
fi

# Check if VirtualBox is installed
if ! command -v VBoxManage &> /dev/null; then
    echo "❌ VirtualBox is not installed. Please install VirtualBox first:"
    echo "   https://www.virtualbox.org/wiki/Downloads"
    exit 1
fi

echo "✅ Prerequisites check passed"

# Start the Vagrant environment
echo "🔧 Starting Vagrant environment..."
vagrant up

echo ""
echo "🎉 Setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. SSH into the VM: vagrant ssh"
echo "   2. Navigate to project: cd blog-apps"
echo "   3. Copy .env file: cp env.example .env"
echo "   4. Edit .env with your configuration"
echo "   5. Start services: docker-compose up -d"
echo ""
echo "🌐 Service URLs (from host machine using VM IP):"
echo "   - Nginx: http://192.168.56.10:80"
echo "   - Keycloak: http://192.168.56.10:8080"
echo "   - PostgreSQL: 192.168.56.10:5432"
echo ""
echo "🔧 Useful commands:"
echo "   - vagrant ssh       # SSH into the VM"
echo "   - vagrant halt      # Stop the VM"
echo "   - vagrant destroy   # Delete the VM"
echo "   - vagrant reload    # Restart the VM"
