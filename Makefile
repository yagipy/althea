.PHONY: build
build: Dockerfile
	docker build -t althea .

.PHONY: attach
attach:
	docker run --rm -it -v $$PWD:/althea althea bash
