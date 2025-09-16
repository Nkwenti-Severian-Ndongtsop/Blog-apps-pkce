# Vagrant Setup for Blog Apps

This directory contains a Vagrant configuration to create a virtual machine with Docker and Docker Compose pre-installed for running the Blog Apps project.

## Prerequisites

Before you begin, ensure you have the following installed on your host machine:

- [Vagrant](https://www.vagrantup.com/downloads) (version 2.0+)
- [VirtualBox](https://www.virtualbox.org/wiki/Downloads) (as the VM provider)

## Quick Start

1. **Run the setup script:**
   ```bash
   ./vagrant-setup.sh
   ```

2. **Or manually start the VM:**
   ```bash
   vagrant up
   ```

## VM Configuration

- **OS:** Ubuntu 22.04 LTS (Jammy)
- **Memory:** 2GB RAM
- **CPUs:** 2 cores
- **IP Address:** 192.168.56.10
- **Synced Folder:** Current directory → `/vagrant` in VM

## Network Access

The VM is configured with a static IP address on a private network. Access services directly using the VM's IP:

| Service    | VM Port | VM IP Address | URL                        |
|------------|---------|---------------|----------------------------|
| Nginx      | 80      | 192.168.56.10 | http://192.168.56.10       |
| Keycloak   | 8080    | 192.168.56.10 | http://192.168.56.10:8080  |
| PostgreSQL | 5432    | 192.168.56.10 | 192.168.56.10:5432        |

## What Gets Installed

The Ansible playbook (`ansible/docker-setup.yml`) automatically installs:

- Docker CE (latest stable)
- Docker Compose (v2.21.0)
- Required dependencies and tools
- Adds the `vagrant` user to the `docker` group

## Usage

### Starting the VM
```bash
vagrant up
```

### SSH into the VM
```bash
vagrant ssh
```

### Running the Blog Apps
Once inside the VM:
```bash
cd blog-apps
cp env.example .env
# Edit .env file with your configuration
docker-compose up -d
```

### Managing the VM
```bash
vagrant halt      # Stop the VM
vagrant reload     # Restart the VM
vagrant destroy    # Delete the VM completely
vagrant status     # Check VM status
```

## Project Structure

```
.
├── Vagrantfile              # Vagrant configuration
├── ansible/
│   └── docker-setup.yml     # Ansible playbook for Docker installation
├── vagrant-setup.sh         # Quick setup script
└── VAGRANT_README.md        # This file
```

## Troubleshooting

### VM Won't Start
- Ensure VirtualBox is installed and running
- Check if virtualization is enabled in your BIOS
- Try: `vagrant reload --provision`

### Docker Permission Issues
The Ansible playbook adds the `vagrant` user to the `docker` group. If you encounter permission issues:
```bash
# Inside the VM
sudo usermod -aG docker $USER
# Then logout and login again
```

### Network Issues
If you can't access services on the VM IP:
1. Ensure the VM is running: `vagrant status`
2. Check VM IP: `vagrant ssh -c "ip addr show"`
3. Verify services are running: `vagrant ssh -c "docker ps"`

## Customization

### Changing VM Resources
Edit the `Vagrantfile` to modify:
- Memory: Change `vb.memory = "2048"`
- CPUs: Change `vb.cpus = 2`

### Adding More Software
Edit `ansible/docker-setup.yml` to install additional packages or configure services.

## Clean Up

To completely remove the VM and free up disk space:
```bash
vagrant destroy
vagrant box remove ubuntu/jammy64  # Optional: removes the base box
```
