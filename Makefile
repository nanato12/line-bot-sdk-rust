.PHONY: env
env:
	test -f .env || cp .env.template .env

.PHONY: generate
generate:
	cargo run -p generator

.PHONY: run-actix-sample
run-actix-sample:
	cargo run --example actix-sample --features="actix_support"

.PHONY: run-rocket-sample
run-rocket-sample:
	cargo run --example rocket-sample --features="rocket_support"
