## Setup local server
```shell
docker-compose up
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
docker-compose run --rm cli aws ssm start-session --target <INSTANCE_ID> --document-name benchmark-server-althea-deployer
```

- Execute on instance
```shell
sudo su - root
cd /root/althea/doc/reference/benchmark-server-althea/app
docker build -q .
docker run --rm -d -p 80:80 <IMAGE_HASH>
```

- Execute on client
```shell
curl <PUBLIC_IP>
```

## Apply local changes to production server
- Execute on client
```shell
git push -u origin <TARGET_BRANCH>
docker-compose run --rm cli aws ssm start-session --target <INSTANCE_ID> --document-name benchmark-server-althea-deployer
```

- Execute on instance
```shell
sudo su - root
cd /root/althea/doc/reference/benchmark-server-althea/app
git pull
git chechout -b <TARGET_BRANCH> origin/<TARGET_BRANCH>

docker ps
docker stop <CONTAINER_ID>

docker build -q .
docker run --rm -d -p 80:80 <IMAGE_ID>
```
