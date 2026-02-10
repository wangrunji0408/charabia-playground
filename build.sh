#!/bin/bash
set -e

echo "==> Building WASM with wasm-pack..."
wasm-pack build --target web --release --out-dir web/pkg

# Remove unnecessary files from pkg
rm -f web/pkg/.gitignore web/pkg/package.json web/pkg/README.md

echo "==> Build complete! Open web/index.html to preview."
echo "    (use a local server, e.g.: python3 -m http.server -d web 8080)"
