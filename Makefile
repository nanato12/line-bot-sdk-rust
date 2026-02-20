.PHONY: publish
publish:
	cargo publish --manifest-path core/line_channel_access_token/Cargo.toml || true
	cargo publish --manifest-path core/line_insight/Cargo.toml || true
	cargo publish --manifest-path core/line_liff/Cargo.toml || true
	cargo publish --manifest-path core/line_manage_audience/Cargo.toml || true
	cargo publish --manifest-path core/line_messaging_api/Cargo.toml || true
	cargo publish --manifest-path core/line_module/Cargo.toml || true
	cargo publish --manifest-path core/line_module_attach/Cargo.toml || true
	cargo publish --manifest-path core/line_shop/Cargo.toml || true
	cargo publish --manifest-path core/line_webhook/Cargo.toml || true
	sleep 30
	cargo publish --manifest-path core/lib/Cargo.toml

.PHONY: generate
generate:
	python3 generate-code.py
