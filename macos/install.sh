#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Building Fuso..."
swift build -c release

echo "Creating app bundle..."
mkdir -p Fuso.app/Contents/MacOS
mkdir -p Fuso.app/Contents/Resources
cp .build/release/Fuso Fuso.app/Contents/MacOS/Fuso
cp Sources/Info.plist Fuso.app/Contents/Info.plist

echo "Installing to /Applications..."
cp -R Fuso.app /Applications/Fuso.app

echo "Done. Launch Fuso from /Applications or Spotlight."
