## Setup local server
```shell
cd vagrant
vagrant up
curl localhost
```

## Setup production server
- Set infra/credential/docker.admin.env
```env
AWS_ACCESS_KEY_ID=<YOUR_AWS_ACCESS_KEY_ID>
AWS_SECRET_ACCESS_KEY=<YOUR_AWS_SECRET_ACCESS_KEY>
AWS_DEFAULT_REGION=<YOUR_AWS_DEFAULT_REGION>
```

- Execute on client
```shell
docker-compose up -d
docker-compose exec infra terraform apply
docker-compose run --rm cli aws ssm start-session --target <INSTANCE_ID> --document-name benchmark-server-deployer
```

- Execute on instance
```shell
sudo su - root
cd /root/althea/doc/reference/benchmark-server/app
go build -o /usr/local/bin/benchmark-server /root/althea/doc/reference/benchmark-server/app/main.go
sudo systemctl start benchmark-server
```

- Execute on client
```shell
curl <PUBLIC_IP>
```

## Apply local changes to production server
- Execute on client
```shell
git push -u origin <TARGET_BRANCH>
docker-compose run --rm cli aws ssm start-session --target <INSTANCE_ID> --document-name benchmark-server-deployer
```

- Execute on instance
```shell
sudo su - root
cd /root/althea/doc/reference/benchmark-server/app
git pull
git chechout <TARGET_BRANCH>
go build -o /usr/local/bin/benchmark-server /root/althea/doc/reference/benchmark-server/app/main.go
sudo systemctl start benchmark-server
```