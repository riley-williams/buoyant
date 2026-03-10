#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo run -p buoyant-harness --example book_images -- --no-overlay "$@"
