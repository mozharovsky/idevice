#!/bin/bash
# Build @xcodekit/idevice → pkg/idevice/
set -e

if [ ! -f "index.js" ] || [ ! -f "index.d.ts" ]; then
  echo "Error: index.js/index.d.ts not found. Run: npx napi build --platform --release"
  exit 1
fi

OUT="pkg/idevice"
rm -rf "$OUT"
mkdir -p "$OUT"

cp npm/idevice/package.json "$OUT/package.json"
cp README.md "$OUT/README.md"
cp index.js "$OUT/index.js"
cp index.d.ts "$OUT/index.d.ts"

echo "Built $OUT/"
