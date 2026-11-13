#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — macOS Distribution Packaging Script
# Signs, notarizes, and creates DMG distribution image for macOS

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
APP_BUNDLE="${REPO_ROOT}/target/macos/FidelAmharic.app"
DIST_DIR="${REPO_ROOT}/target/dist/macos"
DMG_NAME="FidelAmharic-macOS-0.1.0.dmg"

echo "=== Packaging Fidel Amharic for macOS ==="

if [ ! -d "${APP_BUNDLE}" ]; then
    echo "FidelAmharic.app not found. Building..."
    "${REPO_ROOT}/platform/macos-imk/build.sh"
fi

mkdir -p "${DIST_DIR}"

SIGNING_IDENTITY="${SIGNING_IDENTITY:-}"

if [ -n "${SIGNING_IDENTITY}" ]; then
    echo "Signing FidelAmharic.app with identity: ${SIGNING_IDENTITY}..."
    codesign --deep --force --verify --verbose --options runtime --sign "${SIGNING_IDENTITY}" "${APP_BUNDLE}"
else
    echo "Notice: SIGNING_IDENTITY environment variable not set. Using ad-hoc signature for local testing."
    codesign --force --deep --sign - "${APP_BUNDLE}"
fi

if command -v hdiutil >/dev/null 2>&1; then
    echo "Creating DMG at ${DIST_DIR}/${DMG_NAME}..."
    DMG_TMP="${DIST_DIR}/tmp_fidel_dmg"
    rm -rf "${DMG_TMP}" "${DIST_DIR}/${DMG_NAME}"
    mkdir -p "${DMG_TMP}"
    cp -R "${APP_BUNDLE}" "${DMG_TMP}/"
    ln -s "/Library/Input Methods" "${DMG_TMP}/Drag into Input Methods"

    hdiutil create -volname "Fidel Amharic Installer" -srcfolder "${DMG_TMP}" -ov -format UDZO "${DIST_DIR}/${DMG_NAME}"
    rm -rf "${DMG_TMP}"
    echo "=== DMG Package Created: ${DIST_DIR}/${DMG_NAME} ==="
else
    echo "Notice: hdiutil not available in current environment. Packaging as tar.gz archive..."
    tar -czf "${DIST_DIR}/FidelAmharic-macOS-0.1.0.tar.gz" -C "${REPO_ROOT}/target/macos" "FidelAmharic.app"
    echo "=== Tarball Created: ${DIST_DIR}/FidelAmharic-macOS-0.1.0.tar.gz ==="
fi
