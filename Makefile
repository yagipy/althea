.PHONY: build
build: Vagrantfile
	vagrant up

.PHONY: attach
attach:
	vagrant ssh

.PHONY: stop
stop:
	vagrant halt

.PHONY: clean
clean:
	vagrant destroy

#.PHONY: build
#build: Dockerfile
#	docker build -t althea .
#
#.PHONY: attach
#attach:
#	docker run --rm -it -v $$PWD:/althea althea bash
