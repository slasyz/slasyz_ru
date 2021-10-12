##*********************
##* Makefile commands *
##*********************
##

export SHELL := /bin/bash

.DEFAULT_GOAL := help


.PHONY: help
help:           ## show this help
	@sed -nE '/@sed/!s/##\s?//p' Makefile


.PHONY: lint
lint:           ## run linter
	cargo clippy


.PHONY: run
run:            ## start the Rust app in foreground
	cargo run


.PHONY: server
server:         ## run server
	caddy run -config caddy/Caddyfile.local
