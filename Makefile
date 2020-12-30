.DEFAULT_GOAL := default
SHELL := /bin/bash


.PHONY: default
default: | help


.PHONY: help
help:  ## Show all of tasks
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


.PHONY: build
build:  ## Builds Rust code and Python modules
	poetry run maturin build

