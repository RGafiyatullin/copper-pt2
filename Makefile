
CARGO = cargo
DOCKER = docker
DOCKER_COMPOSE = docker compose

COMPOSE_D = deploy.d/docker-compose.d
COMPOSE_PROJECT_NAME = copper-io-pt2

.PHONY: default
default:


.PHONY: clippy
clippy:
	$(CARGO) clippy

.PHONY: fmt
fmt:
	$(CARGO) +nightly fmt

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: build-release
build-release:
	$(CARGO) build --release

.PHONY: build-debug
build-debug:
	$(CARGO) build

.PHONY: compose-up
compose-up:
	$(DOCKER_COMPOSE) \
		-p $(COMPOSE_PROJECT_NAME) \
		-f $(COMPOSE_D)/docker-compose.yml \
		-f $(COMPOSE_D)/docker-compose.override.yml \
		up -d 

.PHONY: compose-dn
compose-dn:
	$(DOCKER_COMPOSE) \
		-p $(COMPOSE_PROJECT_NAME) \
		-f $(COMPOSE_D)/docker-compose.yml \
		-f $(COMPOSE_D)/docker-compose.override.yml \
		down

.PHONY: compose-build
compose-build:
	$(DOCKER_COMPOSE) \
		-p $(COMPOSE_PROJECT_NAME) \
		-f $(COMPOSE_D)/docker-compose.yml \
		-f $(COMPOSE_D)/docker-compose.override.yml \
		build

.PHONY: compose-db-wipe
compose-db-wipe:
	$(DOCKER) volume rm $(COMPOSE_PROJECT_NAME)_pg-data

.PHONY: compose-db-connect
compose-db-connect:
	$(DOCKER_COMPOSE) \
		-p $(COMPOSE_PROJECT_NAME) \
		-f $(COMPOSE_D)/docker-compose.yml \
		-f $(COMPOSE_D)/docker-compose.override.yml \
		exec pg psql -Udev dev