#!/bin/bash
# Build and package TALA Diagram App for static web server deployment
# Usage: ./build-release.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

RELEASE_DIR="release"

echo "=== TALA Diagram App - Release Build ==="
echo ""

# Step 1: Build WASM
echo "[1/3] Building WASM module..."
wasm-pack build --release --out-dir pkg
echo "WASM build complete."
echo ""

# Step 2: Clean and create release directory
echo "[2/3] Packaging release..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR/pkg"

# Copy index.html
cp index.html "$RELEASE_DIR/"

# Copy only the files needed at runtime (not TypeScript defs or README)
cp pkg/tala_diagram_app.js "$RELEASE_DIR/pkg/"
cp pkg/tala_diagram_app_bg.js "$RELEASE_DIR/pkg/"
cp pkg/tala_diagram_app_bg.wasm "$RELEASE_DIR/pkg/"

echo "Release packaged in $RELEASE_DIR/"
echo ""

# Step 3: Show summary
echo "[3/3] Release summary:"
echo ""
echo "Files included:"
find "$RELEASE_DIR" -type f | while read -r f; do
    size=$(stat -f%z "$f" 2>/dev/null || stat -c%s "$f" 2>/dev/null)
    echo "  $f ($size bytes)"
done
echo ""

TOTAL_SIZE=$(du -sh "$RELEASE_DIR" | cut -f1)
echo "Total size: $TOTAL_SIZE"
echo ""

# Step 4: Create zip archive
echo "Creating zip archive..."
ZIP_NAME="tala-diagram-app.zip"
cd "$RELEASE_DIR"
zip -r "../$ZIP_NAME" . > /dev/null
cd "$SCRIPT_DIR"
ZIP_SIZE=$(stat -f%z "$ZIP_NAME" 2>/dev/null || stat -c%s "$ZIP_NAME" 2>/dev/null)
echo "Zip archive: $ZIP_NAME ($ZIP_SIZE bytes)"
echo ""

echo "=== Done! ==="
echo ""
echo "To deploy, extract the zip into your web directory:"
echo "  unzip tala-diagram-app.zip -d /path/to/web/root/"
echo ""
echo "Or copy the $RELEASE_DIR/ folder to any static web server:"
echo "  - nginx: point root to $RELEASE_DIR/"
echo "  - Apache: point DocumentRoot to $RELEASE_DIR/"
echo "  - GitHub Pages: push $RELEASE_DIR/ contents to gh-pages branch"
echo "  - Python: cd $RELEASE_DIR && python3 -m http.server 8080"
