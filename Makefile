.PHONY: generate
generate:
	cargo run -p generator

.PHONY: run-sample
run-sample:
	cargo run -p sample
