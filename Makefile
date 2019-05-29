.PHONY: deps
deps:
	cargo install cargo-web

.PHONY: build
build:
	cargo web build

.PHONY: server
start:
	cargo web start
