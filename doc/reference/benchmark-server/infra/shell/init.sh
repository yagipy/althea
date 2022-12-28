#!/bin/sh

cd /root
snap install go --classic

git clone https://github.com/yagipy/althea.git
sudo cp /root/althea/doc/reference/benchmark-server/systemd/* /etc/systemd/system
go build /althea/doc/reference/benchmark-server/app/main.go -o /usr/local/bin/benchmark-server
sudo systemctl start benchmark-server
