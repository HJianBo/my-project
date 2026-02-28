#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${1:-8000}"
WEB_DIR="${ROOT_DIR}/web"
BUILD_SCRIPT="${ROOT_DIR}/scripts/build_web.sh"

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required to serve web assets" >&2
  exit 1
fi

if [[ ! -f "${WEB_DIR}/index.html" ]]; then
  echo "Missing ${WEB_DIR}/index.html" >&2
  exit 1
fi

if [[ ! -f "${WEB_DIR}/mq_js_bundle.js" || ! -f "${WEB_DIR}/simple_tetris.wasm" ]]; then
  if [[ ! -x "${BUILD_SCRIPT}" ]]; then
    echo "Missing web assets and build script is not executable: ${BUILD_SCRIPT}" >&2
    exit 1
  fi

  echo "Web assets missing. Building release web bundle first..."
  "${BUILD_SCRIPT}" release
fi

echo "Serving ${WEB_DIR} at http://127.0.0.1:${PORT}"
python3 -m http.server "${PORT}" --directory "${WEB_DIR}"
