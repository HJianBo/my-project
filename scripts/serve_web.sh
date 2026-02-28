#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${1:-8000}"

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required to serve web assets" >&2
  exit 1
fi

echo "Serving ${ROOT_DIR}/web at http://127.0.0.1:${PORT}"
python3 -m http.server "${PORT}" --directory "${ROOT_DIR}/web"
