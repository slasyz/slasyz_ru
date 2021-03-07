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
	PYTHONUNBUFFERED=TRUE ./venv/bin/uvicorn \
		--log-level info \
		-k uvicorn.workers.UvicornWorker \
		--host 0.0.0.0 \
		--port 8000 \
		--factory \
		slasyz_ru.main:create_app


.PHONY: dev
dev:          ## run development server
dev:
	PYTHONUNBUFFERED=TRUE ./venv/bin/uvicorn \
		--log-level debug \
		--reload \
		--host 0.0.0.0 \
		--port 8000 \
		--factory \
		slasyz_ru.main:create_app


.PHONY: pylint
pylint:       ## run pylint (with disabled checks specified in $DISABLE variable)
	cd ..; ./slasyz_ru/$(PYTHON_INTERPRETER) -m pylint ./slasyz_ru/slasyz_ru --rcfile="./slasyz_ru/pylintrc" --disable="$(DISABLE)"


.PHONY: run
run:          ## run directly
	PYTHONUNBUFFERED=TRUE ./venv/bin/python -m slasyz_ru.main


.PHONY: tmux.restart
tmux.restart: ## restart tmux session
tmux.restart:
	$(MAKE) tmux.stop
	sleep 2
	$(MAKE) tmux.start


.PHONY: tmux.start
tmux.start:   ## start tmux session
	tmux new-session -s ToxicTgBot -d "make run.dev"


.PHONY: tmux.stop
tmux.stop:    ## stop tmux session
	( \
		tmux send-keys -t ToxicTgBot C-c; \
		tmux send-keys -t ToxicTgBot C-c \
	) || true
