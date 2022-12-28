## Setup

```shell
docker-compose up
docker-compose exec infra terraform apply
docker-compose run --rm cli aws ssm start-session --target <INSTANCE_ID> --document-name benchmark-server-deployer
sudo su - root
```
