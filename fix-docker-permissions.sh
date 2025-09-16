#!/bin/bash

# Fix Docker Permissions Script
# This script helps resolve Docker permission issues in the Vagrant VM

echo "🔧 Docker Permissions Fix Script"
echo "================================"

# Check if we're running inside the VM
if [ ! -f /vagrant/Vagrantfile ]; then
    echo "❌ This script should be run inside the Vagrant VM"
    echo "   Run: vagrant ssh"
    echo "   Then: cd /vagrant && ./fix-docker-permissions.sh"
    exit 1
fi

echo "📋 Checking current Docker setup..."

# Check if Docker is running
if ! systemctl is-active --quiet docker; then
    echo "🔄 Starting Docker service..."
    sudo systemctl start docker
    sudo systemctl enable docker
fi

# Check if user is in docker group
if groups $USER | grep -q docker; then
    echo "✅ User $USER is already in the docker group"
else
    echo "➕ Adding user $USER to docker group..."
    sudo usermod -aG docker $USER
fi

# Test Docker with sudo first
echo "🧪 Testing Docker with sudo..."
if sudo docker run --rm hello-world > /dev/null 2>&1; then
    echo "✅ Docker works with sudo"
else
    echo "❌ Docker not working properly. Check installation."
    exit 1
fi

# Test Docker without sudo
echo "🧪 Testing Docker without sudo..."
if docker run --rm hello-world > /dev/null 2>&1; then
    echo "✅ Docker works without sudo - permissions are correct!"
else
    echo "⚠️  Docker permissions not yet active for current session"
    echo ""
    echo "📋 To fix this, you have two options:"
    echo ""
    echo "   Option 1 (Recommended): Restart your SSH session"
    echo "   - Exit the VM: exit"
    echo "   - SSH back in: vagrant ssh"
    echo ""
    echo "   Option 2: Use newgrp (temporary fix for current session)"
    echo "   - Run: newgrp docker"
    echo "   - Then test: docker run --rm hello-world"
    echo ""
    echo "🔄 Applying temporary fix with newgrp..."
    
    # Try to apply temporary fix
    if newgrp docker <<< 'docker run --rm hello-world' > /dev/null 2>&1; then
        echo "✅ Temporary fix applied successfully!"
        echo "   Docker should now work in this session"
    else
        echo "⚠️  Please restart your SSH session for full effect"
    fi
fi

echo ""
echo "🎉 Docker permissions check complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Navigate to project: cd blog-apps"
echo "   2. Copy environment file: cp env.example .env"
echo "   3. Edit .env with your configuration"
echo "   4. Start services: docker-compose up -d"
