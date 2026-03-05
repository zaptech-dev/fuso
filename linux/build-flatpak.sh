#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# Generate cargo sources for offline build
if ! command -v flatpak-cargo-generator &> /dev/null; then
    echo "Installing flatpak-cargo-generator..."
    pip install flatpak-cargo-generator 2>/dev/null || pip install --user flatpak-cargo-generator
fi

echo "Generating cargo sources..."
flatpak-cargo-generator linux/Cargo.lock -o linux/cargo-sources.json

echo "Building Flatpak..."
flatpak-builder --force-clean --user --install build-dir linux/dev.zaptech.fuso.yml

echo "Done. Run with: flatpak run dev.zaptech.fuso"
