#!/usr/bin/env bash
set -euo pipefail

SPEC_FILE="${1:-RegiojetNL3-regio-jet_api-1.1.0-resolved.yaml}"
OUTPUT_DIR="."

echo "Regenerating API client from $SPEC_FILE..."

openapi-generator-cli generate \
  -i "$SPEC_FILE" \
  -g rust \
  -o "$OUTPUT_DIR" \
  --package-name rustjet \
  --skip-validate-spec

echo "API client regenerated"
echo "Protected files (see .openapi-generator-ignore):"
cat .openapi-generator-ignore