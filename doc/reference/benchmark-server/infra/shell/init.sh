#!/bin/sh

cd /root
snap install go --classic

git clone https://github.com/yagipy/althea.git
sudo ln -sf /root/althea/doc/reference/benchmark-server/systemd/* /etc/systemd/system
#cd /root/althea/doc/reference/benchmark-server/app
#go build -o /usr/local/bin/benchmark-server /root/althea/doc/reference/benchmark-server/app/main.go
#sudo systemctl start benchmark-server
