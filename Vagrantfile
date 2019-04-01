Vagrant.configure("2") do |config|
	config.vm.box = "bento/debian-9.6"
	config.vm.provision :shell, path: "postgres/bootstrap.sh"
	config.vm.network "forwarded_port", guest: 5432, host: 5432
	config.ssh.forward_x11 = true

	config.vm.provider "virtualbox" do |v|
		v.name = "postgres"
		v.gui = false
		v.cpus = 2
		v.memory = 2048
	end
end
