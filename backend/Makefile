## *********************
## * Makefile commands *
## *********************
##


SHELL := /bin/bash
PYTHON_INTERPRETER := $(shell ls ./venv_pycharm/Scripts/python.exe ./venv/bin/python 2> /dev/null | head -n1 | awk '{print $1;}')


.DEFAULT_GOAL := help


.PHONY: deps
deps:         ## install all dependencies from requirements.txt
	$(PYTHON_INTERPRETER) -m pip install -r requirements.txt


.PHONY: help
help:         ## show this help
	@sed -ne '/@sed/!s/##\s\?//p' $(MAKEFILE_LIST)


.PHONY: init
init:         ## create virtual environment
	virtualenv venv -p python3
	$(MAKE) deps


.PHONY: lint
lint:         ## run linter with less strict checks
lint: DISABLE=invalid-name,unused-argument
lint: pylint


.PHONY: lint.all
lint.all:     ## run linter with all usable checks
lint.all: pylint


.PHONY: prod
prod:         ## run production server
prod:
	PYTHONUNBUFFERED=TRUE ./venv/bin/gunicorn \
		--log-level INFO \
		--access-logfile=- \
		--disable-redirect-access-to-syslog \
		-k flask_sockets.worker \
		-c python:slasyz_ru.config \
		"slasyz_ru.server.main:create_app()"


.PHONY: dev
dev:          ## run development server
dev:
	PYTHONUNBUFFERED=TRUE ./venv/bin/gunicorn \
		--log-level DEBUG \
		--access-logfile=- \
		--disable-redirect-access-to-syslog \
		--reload \
		-k flask_sockets.worker \
		-c python:slasyz_ru.config \
		"slasyz_ru.server.main:create_app()"


.PHONY: pylint
pylint:       ## run pylint (with disabled checks specified in $DISABLE variable)
	cd ..; ./slasyz_ru/$(PYTHON_INTERPRETER) -m pylint ./slasyz_ru/slasyz_ru --rcfile="./slasyz_ru/pylintrc" --disable="$(DISABLE)"


.PHONY: run.flask
run.flask:    ## run using flask
	PYTHONUNBUFFERED=TRUE ./venv/bin/python -m slasyz_ru.server.main


.PHONY: tmux.restart
tmux.restart: ## restart tmux session
tmux.restart:
	$(MAKE) tmux.stop
	sleep 2
	$(MAKE) tmux.start


.PHONY: tmux.start
tmux.start:   ## start tmux session
	tmux new-session -s ToxicTgBot -d "make run.gunicorn"


.PHONY: tmux.stop
tmux.stop:    ## stop tmux session
	( \
		tmux send-keys -t ToxicTgBot C-c; \
		tmux send-keys -t ToxicTgBot C-c \
	) || true
