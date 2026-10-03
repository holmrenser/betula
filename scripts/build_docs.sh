#!/usr/bin/env bash
# Build the docs site into docs/_build/html with mystmd.
#
# mystmd 1.11's --strict only fails on errors, not warnings, so a broken
# internal link would still produce a "successful" build. Fail on any
# warning here instead. Set BASE_URL (e.g. /betula) when deploying under a
# subpath.
set -euo pipefail
cd "$(dirname "$0")/.."

python3 scripts/generate_schema_pages.py

log="$(mktemp)"
(cd docs && npx --no-install myst build --html --ci 2>&1) | tee "$log"
if grep -qE '⚠️|⛔' "$log"; then
  echo "docs build reported warnings or errors (see above)" >&2
  exit 1
fi

# Serve the raw schemas and fixtures at stable URLs alongside the pages.
cp -r schema examples docs/_build/html/
