#!/usr/bin/env bash
# Regenerate the Python, TypeScript, and Rust bindings from schema/.
# CI runs this and fails if the committed bindings differ from its output.
#
# Generator versions are pinned (datamodel-code-generator in
# bindings/python/requirements-dev.txt, json-schema-to-typescript in
# bindings/typescript/package-lock.json, cargo-typify below, rustfmt via
# bindings/rust/rust-toolchain.toml) so output is reproducible.
set -euo pipefail
cd "$(dirname "$0")/.."

TYPIFY_VERSION="0.8.0"
if [[ "$(cargo typify --version 2>/dev/null)" != "cargo-typify $TYPIFY_VERSION" ]]; then
  echo "need cargo-typify $TYPIFY_VERSION: cargo install cargo-typify --version $TYPIFY_VERSION --locked" >&2
  exit 1
fi

BUNDLE=build/betula.bundle.schema.json
python3 scripts/bundle_schema.py "$BUNDLE"

datamodel-codegen \
  --input "$BUNDLE" --input-file-type jsonschema \
  --output bindings/python/src/betula/models.py \
  --output-model-type pydantic_v2.BaseModel --target-python-version 3.11 \
  --use-title-as-name --use-schema-description --field-constraints \
  --use-standard-collections --use-union-operator --enum-field-as-literal all \
  --formatters builtin --disable-timestamp

(cd bindings/typescript && node scripts/generate.mjs "../../$BUNDLE")

cp "$BUNDLE" bindings/rust/schema/betula.bundle.schema.json
python3 bindings/rust/scripts/prepare_typify_input.py "$BUNDLE" build/betula.typify.schema.json bindings/rust/src/kinds.rs
CRATE_VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' bindings/rust/Cargo.toml | head -1)"
(cd bindings/rust && cargo typify --no-builder --crate "betula@$CRATE_VERSION" -d PartialEq \
  ../../build/betula.typify.schema.json -o src/types.rs)

echo "Bindings regenerated."
