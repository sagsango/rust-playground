SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project
	cargo clean

debug: ## build project with debug info
	cargo build

release: ## build project with -O3 optimizations
	cargo build --release

format: ## Format the project
	cargo fmt

doc: ## Generate the documentation
	cargo doc --no-deps --open
