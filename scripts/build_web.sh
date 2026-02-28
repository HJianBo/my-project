#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="wasm32-unknown-unknown"
CRATE_NAME="simple_tetris"
BUILD_MODE="${1:-debug}"

if [[ "${BUILD_MODE}" != "debug" && "${BUILD_MODE}" != "release" ]]; then
  echo "Usage: $0 [debug|release]" >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx "${TARGET}"; then
  rustup target add "${TARGET}"
fi

if [[ "${BUILD_MODE}" == "release" ]]; then
  cargo build --target "${TARGET}" --release
  WASM_SRC="${ROOT_DIR}/target/${TARGET}/release/${CRATE_NAME}.wasm"
else
  cargo build --target "${TARGET}"
  WASM_SRC="${ROOT_DIR}/target/${TARGET}/debug/${CRATE_NAME}.wasm"
fi

BUNDLE_SRC="$({ find "${CARGO_HOME:-$HOME/.cargo}/registry/src" -path "*/macroquad-*/js/mq_js_bundle.js" 2>/dev/null || true; } | sort -V | tail -n1)"
if [[ -z "${BUNDLE_SRC}" ]]; then
  echo "Could not find mq_js_bundle.js from macroquad in Cargo registry" >&2
  exit 1
fi

mkdir -p "${ROOT_DIR}/web"
cp "${WASM_SRC}" "${ROOT_DIR}/web/${CRATE_NAME}.wasm"
cp "${BUNDLE_SRC}" "${ROOT_DIR}/web/mq_js_bundle.js"

echo "Built web bundle:"
echo "- ${ROOT_DIR}/web/${CRATE_NAME}.wasm"
echo "- ${ROOT_DIR}/web/mq_js_bundle.js"
