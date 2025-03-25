SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project
	cargo clean

build: ## Build the project
	cargo build

format: ## Format the project
	cargo fmt

doc: ## Generate the documentation
	cargo doc --no-deps --open
