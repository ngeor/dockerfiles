# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
  # The most common configuration options are documented and commented below.
  # For a complete reference, please see the online documentation at
  # https://docs.vagrantup.com.

  # Every Vagrant development environment requires a box. You can search for
  # boxes at https://vagrantcloud.com/search.
  config.vm.box = "hashicorp/bionic64"

  # Disable automatic box update checking. If you disable this, then
  # boxes will only be checked for updates when the user runs
  # `vagrant box outdated`. This is not recommended.
  # config.vm.box_check_update = false

  # Create a forwarded port mapping which allows access to a specific port
  # within the machine from a port on the host machine. In the example below,
  # accessing "localhost:8080" will access port 80 on the guest machine.
  # NOTE: This will enable public access to the opened port
  # config.vm.network "forwarded_port", guest: 80, host: 8080

  # Create a forwarded port mapping which allows access to a specific port
  # within the machine from a port on the host machine and only allow access
  # via 127.0.0.1 to disable public access
  config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

  # Create a private network, which allows host-only access to the machine
  # using a specific IP.
  # config.vm.network "private_network", ip: "192.168.33.10"

  # Create a public network, which generally matched to bridged network.
  # Bridged networks make the machine appear as another physical device on
  # your network.
  # config.vm.network "public_network"

  # Share an additional folder to the guest VM. The first argument is
  # the path on the host to the actual folder. The second argument is
  # the path on the guest to mount the folder. And the optional third
  # argument is a set of non-required options.
  # config.vm.synced_folder "../data", "/vagrant_data"

  # Provider-specific configuration so you can fine-tune various
  # backing providers for Vagrant. These expose provider-specific options.
  # Example for VirtualBox:
  #
  # config.vm.provider "virtualbox" do |vb|
  #   # Display the VirtualBox GUI when booting the machine
  #   vb.gui = true
  #
  #   # Customize the amount of memory on the VM:
  #   vb.memory = "1024"
  # end
  #
  # View the documentation for the provider you are using for more
  # information on available options.

  # Enable provisioning with a shell script. Additional provisioners such as
  # Ansible, Chef, Docker, Puppet and Salt are also available. Please see the
  # documentation for more information about their specific syntax and use.
  # config.vm.provision "shell", inline: <<-SHELL
  #   apt-get update
  #   apt-get install -y apache2
  # SHELL

  # Bazel
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y curl gnupg
    curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > /etc/apt/trusted.gpg.d/bazel.gpg
    echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list
    apt-get update
    apt-get install -y bazel-4.2.1
    ln -f -s /usr/bin/bazel-4.2.1 /usr/bin/bazel
  SHELL

  # Docker
  config.vm.provision "shell", inline: <<-SHELL
    apt-get install -y docker-compose
    addgroup vagrant docker
  SHELL

  # Rust
  config.vm.provision "shell", inline: <<-SHELL
    apt-get install -y cargo cargo-doc rustfmt rust-clippy
  SHELL

  # dosbox
  config.vm.provision "shell", inline: <<-SHELL
    apt-get install -y dosbox
  SHELL

  # apache
  config.vm.provision "file", source: "./basic/httpd/extra.conf", destination: "~/extra.conf"
  config.vm.provision "shell", inline: <<-SHELL
    apt-get install -y apache2
    # active cgi module https://httpd.apache.org/docs/current/mod/mod_cgi.html
    a2enmod cgid
    # activate rewrite module https://httpd.apache.org/docs/current/mod/mod_rewrite.html
    a2enmod rewrite
    # activate actions module https://httpd.apache.org/docs/current/mod/mod_actions.html
    a2enmod actions
    # copy extra.conf
    mv /home/vagrant/extra.conf /etc/apache2/conf-available/
    chown root:root /etc/apache2/conf-available/extra.conf
    chmod 644 /etc/apache2/conf-available/extra.conf
    a2enconf extra
    # fix permissions of data folders
    chown -R root:www-data /var/www/html
    chmod 775 /var/www/html
    chown -R root:www-data /usr/lib/cgi-bin
    chmod 775 /usr/lib/cgi-bin/
    # add user to www-data group
    addgroup vagrant www-data
    # restart Apache
    systemctl restart apache2
  SHELL

  # Copy GWBASIC/QBASIC into PATH
  config.vm.provision "file", source: "~/DOSBox/PROGS/GWBASIC/GWBASIC.EXE", destination: "~/GWBASIC.EXE"
  config.vm.provision "file", source: "~/DOSBox/PROGS/QBASIC/QBASIC.EXE", destination: "~/QBASIC.EXE"
  config.vm.provision "shell", inline: <<-SHELL
    mv /home/vagrant/*.EXE /usr/local/bin/
    chown root:root /usr/local/bin/*.EXE
    chmod 444 /usr/local/bin/*.EXE
  SHELL

  # Warm up bazel
  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    cd /vagrant && bazel build //...
    # TODO ./basic/httpd/httpd.sh  does not work due to permission issue copying files
  SHELL
end
