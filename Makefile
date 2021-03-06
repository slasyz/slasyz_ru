## *********************
## * Makefile commands *
## *********************
##


SHELL := /bin/bash
PYTHON_INTERPRETER := $(shell ls ./venv_pycharm/Scripts/python.exe ./venv/bin/python 2> /dev/null | head -n1 | awk '{print $1;}')


.DEFAULT_GOAL := help


.PHONY: init
init:         ## install all dependencies
	make -C backend init
	cd frontend && yarn install


.PHONY: help
help:         ## show this help
	@sed -ne '/@sed/!s/##\s\?//p' $(MAKEFILE_LIST)


.PHONY: dev   ## run development server
dev:
	docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build --remove-orphans


.PHONY: prod  ## run production server
prod:
	docker-compose -f docker-compose.yml -f docker-compose.prod.yml up --build --remove-orphans
