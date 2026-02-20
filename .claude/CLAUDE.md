# LINE Messaging API SDK for Rust

## Rules

- This is an OSS project.
  All code, comments, commit messages,
  and documentation MUST be in English.
- License: Apache-2.0
- Main branch: `develop`

## Reference SDKs

This SDK MUST follow the same architecture
and patterns as the official LINE Bot SDKs:

- [Java](https://github.com/line/line-bot-sdk-java)
- [Python](https://github.com/line/line-bot-sdk-python)
- [Go](https://github.com/line/line-bot-sdk-go)
- [Node.js](https://github.com/line/line-bot-sdk-nodejs)
- [PHP](https://github.com/line/line-bot-sdk-php)
- [Ruby](https://github.com/line/line-bot-sdk-ruby)

When in doubt about architecture, code generation
approach, or project structure, refer to these.

## Key Patterns (shared across all official SDKs)

1. **OpenAPI-driven code generation** —
   API clients and models are auto-generated from
   `line-openapi/` submodule YAML specs
   using a custom OpenAPI Generator plugin
   (Java/Maven).
2. **`generate-code.py` orchestration** —
   A Python script drives the generation pipeline.
3. **Custom templates (Pebble)** —
   Java, Go, Node.js, Ruby use Pebble templates
   for codegen customization.
4. **Per-service module isolation** —
   Each LINE API surface is an independent
   module/crate.
5. **Webhook = models only** —
   `webhook.yml` generates only model types,
   no API client.
6. **Signature validation is hand-written** —
   HMAC-SHA256 verification of `x-line-signature`.
7. **Generated code is committed** —
   Regenerated only when OpenAPI specs change.
8. **Renovate bot** keeps `line-openapi` submodule
   up to date automatically.

## line-openapi Submodule

- Uses a fork (`nanato12/line-openapi`), NOT the official
  `line/line-openapi`.
- Reason: `info.version` in each YAML spec controls the
  generated crate version in `Cargo.toml`. The fork allows
  bumping versions independently for crates.io publishing.
- Upstream: <https://github.com/line/line-openapi>
- Fork: <https://github.com/nanato12/line-openapi>
- Sync fork before generation:
  `gh repo sync nanato12/line-openapi --source line/line-openapi --branch main`
- Submodule URL MUST use SSH (`git@github.com:`), not HTTPS.

## Code Generation

- Uses `openapi-generator-cli` v7.20.0 (Java) with `--library hyper` (hyper 1.x).
- `progenitor` (Oxide Computer) was evaluated but rejected:
  `messaging-api.yml` and `manage-audience.yml` fail
  due to unsupported `*/*` and `multipart/form-data`
  content types. Also requires nightly rustfmt.
- Generated code uses hyper 1.x, `Arc` (not `Rc`), and
  `Configuration::with_client()` pattern.
- `core/lib/` must be updated BEFORE running `make generate`
  when changing the generator version — `cargo fix` inside
  the generator depends on `core/lib` compiling successfully.

## Code Rules

- `core/line_*/` crates (except `core/lib/`)
  are AUTO-GENERATED. Do NOT manually edit them.
- `core/lib/`, `tools/sources/`, `generator/`,
  `examples/` are hand-written.
- `tools/sources/` contains hand-crafted model
  overrides for polymorphic types — these overwrite
  generated files after generation.
- Ensure dependency versions are consistent between
  `core/lib/Cargo.toml` and generated sub-crate
  `Cargo.toml` files.
- Run `cargo fmt` and `cargo clippy`
  before committing.

## Build Commands

```bash
cargo build              # Build workspace
cargo test --tests       # Run tests
cargo fmt                # Format
cargo clippy             # Lint
make generate            # Regenerate (requires Java)
```
