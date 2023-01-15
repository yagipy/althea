.PHONY: build
build:
	docker build -t althea .

.PHONY: attach
attach:
	docker run --rm -it -v $$PWD:/althea althea bash

.PHONY: imagepush
imagepush:
	docker build -t yagipy/althea:bullseye -f tool/Dockerfile .
	docker push yagipy/althea:bullseye
