#!/bin/bash

# prepare deps
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get upgrade -y
apt-get install -y xauth

# deps
apt-get install -y postgresql postgresql-client pgadmin3

# set up db
USERNAME=$(grep POSTGRES_USERNAME /vagrant/src/constants.rs | cut -d '"' -f2)
PASSWORD=$(grep POSTGRES_PASSWORD /vagrant/src/constants.rs | cut -d '"' -f2)
DB_NAME=$(grep POSTGRES_DB_NAME /vagrant/src/constants.rs | cut -d '"' -f2)

sudo -u postgres psql -c "CREATE USER $USERNAME WITH PASSWORD '$PASSWORD';"
sudo -u postgres psql -c "CREATE DATABASE $DB_NAME WITH OWNER $USERNAME;"

# copy configs
cp /vagrant/postgres/*conf /etc/postgresql/9.6/main/
cp /vagrant/postgres/.pg* /home/vagrant/

# give ownership
chown -R vagrant:vagrant /home/vagrant

# systemd
systemctl restart postgresql
