#!/bin/bash

# prepare deps
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get upgrade -y
apt-get install -y xauth

# deps
apt-get install -y postgresql postgresql-client pgadmin3

# set up db
sudo -u postgres psql -c "CREATE USER space WITH PASSWORD 'space';"
sudo -u postgres psql -c "CREATE DATABASE space_db WITH OWNER space;"

# copy configs
cp /vagrant/postgres/*conf /etc/postgresql/9.6/main/
cp /vagrant/postgres/.pg* /home/vagrant/

# give ownership
chown -R vagrant:vagrant /home/vagrant

# systemd
systemctl restart postgresql