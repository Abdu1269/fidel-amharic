#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — macOS InputMethodKit Build Script
# Builds libfidel_protocol.a for macOS (universal or host architecture)
# and packages FidelAmharic.app

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
BUILD_DIR="${REPO_ROOT}/target/macos"
APP_NAME="FidelAmharic"
APP_BUNDLE="${BUILD_DIR}/${APP_NAME}.app"

echo "=== Building Fidel Amharic macOS Input Method ==="
mkdir -p "${BUILD_DIR}"

# Determine architecture
ARCH="$(uname -m)"
if [ "${ARCH}" = "arm64" ]; then
    PRIMARY_TARGET="aarch64-apple-darwin"
    SWIFT_TARGET="arm64-apple-macos11.0"
else
    PRIMARY_TARGET="x86_64-apple-darwin"
    SWIFT_TARGET="x86_64-apple-macos11.0"
fi

echo "Building Rust engine static library for ${PRIMARY_TARGET}..."
cd "${REPO_ROOT}"
cargo build --release --package fidel-protocol --target "${PRIMARY_TARGET}"

RUST_LIB="${REPO_ROOT}/target/${PRIMARY_TARGET}/release/libfidel_protocol.a"
if [ ! -f "${RUST_LIB}" ]; then
    echo "Error: ${RUST_LIB} not found!"
    exit 1
fi

echo "Creating App Bundle structure at ${APP_BUNDLE}..."
rm -rf "${APP_BUNDLE}"
mkdir -p "${APP_BUNDLE}/Contents/MacOS"
mkdir -p "${APP_BUNDLE}/Contents/Resources/en.lproj"
mkdir -p "${APP_BUNDLE}/Contents/Resources/am.lproj"

# Copy Info.plist and resources
cp "${SCRIPT_DIR}/FidelAmharic/Info.plist" "${APP_BUNDLE}/Contents/Info.plist"
cp "${SCRIPT_DIR}/FidelAmharic/Resources/icon.icns" "${APP_BUNDLE}/Contents/Resources/icon.icns"
cp "${SCRIPT_DIR}/FidelAmharic/Resources/en.lproj/InfoPlist.strings" "${APP_BUNDLE}/Contents/Resources/en.lproj/InfoPlist.strings"
cp "${SCRIPT_DIR}/FidelAmharic/Resources/am.lproj/InfoPlist.strings" "${APP_BUNDLE}/Contents/Resources/am.lproj/InfoPlist.strings"

# Check if swiftc is available
if command -v swiftc >/dev/null 2>&1; then
    echo "Compiling Swift InputMethodKit adapter..."
    swiftc \
        -O \
        -target "${SWIFT_TARGET}" \
        -import-objc-header "${SCRIPT_DIR}/FidelAmharic/FidelAmharic-Bridging-Header.h" \
        -framework Cocoa \
        -framework InputMethodKit \
        "${RUST_LIB}" \
        -o "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}" \
        "${SCRIPT_DIR}/FidelAmharic/main.swift" \
        "${SCRIPT_DIR}/FidelAmharic/FidelServerDelegate.swift" \
        "${SCRIPT_DIR}/FidelAmharic/FidelInputController.swift"

    chmod +x "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"
    echo "=== Build Complete: ${APP_BUNDLE} ==="
else
    echo "Notice: swiftc is not available in the current environment (running on Linux/non-macOS host)."
    echo "Swift source files and bundle resources prepared in ${SCRIPT_DIR}/FidelAmharic."
    echo "When building on macOS, run: ./build.sh"
fi
