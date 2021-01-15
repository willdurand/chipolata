default: help

WASM_PACK_OPTS ?=

bootstrap: ## install the tools required to build and run this project
	cargo install cargo-watch
.PHONY: bootstrap

cli-dev: ## build and run the CLI in debug mode
	cargo run --features=cli "$(rom)"
.PHONY: cli-dev

release-cli:
	cargo build --features=cli
.PHONY: release-cli

release-web: ## build the web app in release mode
release-web: WASM_PACK_OPTS = --release
release-web: setup-web build-wasm-bindings
	rm -rf docs/*.wasm docs/*.js
	cd web && NODE_ENV=production npm run build
.PHONY: release-web

dev: ## run the dev commands to work on the web app
	$(MAKE) -j2 web-dev wasm-dev
.PHONY: dev

setup-web: build-wasm-bindings ## setup the web app
	cd web && npm install --include=dev
.PHONY: setup-web

web-dev: ## run the web app in dev mode locally
web-dev: setup-web
	cd web && NODE_ENV=development npm start
.PHONY: web-dev

wasm-dev: ## watch rust code changes and rebuild the WASM lib
	cargo watch -i "pkg/*" -s "make WASM_PACK_OPTS='--dev' build-wasm-bindings"
.PHONY: wasm-dev

build-wasm-bindings: ## build the WASM bindings
	wasm-pack build --no-typescript $(WASM_PACK_OPTS)
.PHONY: build-wasm-bindings

clean:
	cargo clean
	rm -rf pkg/ web/node_modules/
.PHONY: clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help
