# list recipes
default:
    just --list

# run all checks (fmt, clippy, test)
check: build-web
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo test

# format code
fmt:
    cargo fmt

# run clippy
clippy:
    cargo clippy -- -D warnings

# run tests
test:
    cargo test

# build release
build: build-web
    cargo build --release

# run the bot with args
run *args:
    cargo run --bin bot -- {{args}}

# run the bot
run-bot *args:
    cargo run --bin rustjet-cli -- {{args}}

# build web frontend
build-web:
    cd crates/rustjet-web && npm run build

# watch web frontend for changes
watch-web:
    cd crates/rustjet-web && npm run watch

# run the web server
run-web:
    cargo run --bin rustjet-web

# watch and rebuild on file changes
watch:
    cargo watch -x check

# clean build artifacts
clean:
    cargo clean

# regenerate API client from OpenAPI spec
regen:
    nix develop .#openapi --command ./scripts/regenerate-api.sh

# check flake
flake-check:
    nix flake check

# build with Nix
nix-build:
    nix build .#default
