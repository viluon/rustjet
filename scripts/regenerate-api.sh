#!/usr/bin/env bash
set -euo pipefail

SPEC_FILE="${1:-regiojet-1.2.0.yaml}"
OUTPUT_DIR="."

echo "Regenerating API client from $SPEC_FILE..."

openapi-generator-cli generate \
  -i "$SPEC_FILE" \
  -g rust \
  -o "$OUTPUT_DIR" \
  --package-name rustjet \
  --skip-validate-spec

cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings
cargo fmt
sed -i '/^\/\/\/$/d' $OUTPUT_DIR/src/models/*.rs
