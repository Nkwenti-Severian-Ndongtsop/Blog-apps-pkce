# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  # Use Ubuntu 22.04 LTS as the base box
  config.vm.box = "ubuntu/jammy64"
  config.vm.box_version = "20240821.0.0"

  # Configure VM settings
  config.vm.provider "virtualbox" do |vb|
    vb.name = "blog-apps-docker"
    vb.memory = "2048"
    vb.cpus = 2
    # Enable nested virtualization for Docker
    vb.customize ["modifyvm", :id, "--nested-hw-virt", "on"]
  end

  # Network configuration
  config.vm.network "private_network", ip: "192.168.56.10"

  # Sync the project folder
  config.vm.synced_folder ".", "/vagrant", type: "virtualbox"

  # Provision with Ansible
  config.vm.provision "ansible_local" do |ansible|
    ansible.playbook = "ansible/docker-setup.yml"
    ansible.install_mode = "default"
    ansible.compatibility_mode = "2.0"
    ansible.become = true
  end
end
