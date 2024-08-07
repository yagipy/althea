.PHONY: build
build:
	docker build -t althea .

.PHONY: attach
attach:
	docker run -it -v $$PWD:/althea althea bash

.PHONY: imagepush
imagepush:
	docker build -t yagipy/althea:bullseye -f tool/bullseye.dockerfile .
	docker push yagipy/althea:bullseye

.PHONY: clean
clean:
	docker rmi -f althea
