# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/focal64"
  config.vm.network "forwarded_port", guest: 80, host: 80
  config.vm.synced_folder "../althea", "/root/althea"

  config.vm.provision "shell", inline: <<-SHELL
    sudo su - root
    apt update
    apt install build-essential libfontconfig1-dev -y

    snap install rustup --classic
    rustup install stable
    rustup default stable
    wget https://apt.llvm.org/llvm.sh
    chmod +x llvm.sh
    ./llvm.sh 14
  SHELL
end
