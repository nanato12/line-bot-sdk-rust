.PHONY: publish
publish:
	cargo publish --manifest-path core/lib/Cargo.toml

.PHONY: generate
generate:
	python3 generate-code.py

.PHONY: lint
lint:
	ruff check .
	ruff format --check .
	mypy .

.PHONY: fmt
fmt:
	ruff format .
	ruff check --fix .
