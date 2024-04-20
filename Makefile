.PHONY: publish
publish:
	cargo publish --manifest-path core/lib/Cargo.toml

.PHONY: generate
generate:
	cargo run generator/src/main.rs
