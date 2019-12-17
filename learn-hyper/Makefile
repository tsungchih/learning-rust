DOCKER:=$(shell which docker)

ifeq "$(DOCKER)" ""
	echo "Cannot find command -- docker."
	exit 1
endif

PWD:=$(shell pwd)
DOCKERFILE:=docker/Dockerfile
DOCKER_TAG:=learn-hyper
DOCKER_VER:=latest

.PHONY: build
.PHONY: logs

reload: stop run

build:
	@$(DOCKER) build -f $(DOCKERFILE) -t $(DOCKER_TAG):$(DOCKER_VER) $(PWD)

run:
	@$(DOCKER) run --name $(DOCKER_TAG) -d -p 8088:8088 $(DOCKER_TAG):$(DOCKER_VER)

login:
	@$(DOCKER) attach $(DOCKER_TAG)

logs:
	@$(DOCKER) logs $(DOCKER_TAG)

stop:
	@$(DOCKER) stop $(DOCKER_TAG) && \
	$(DOCKER) rm -v $(DOCKER_TAG)

clean:
	@$(DOCKER) rmi $(DOCKER_TAG):$(DOCKER_VER)

