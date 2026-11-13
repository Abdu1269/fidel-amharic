#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — macOS Distribution Packaging Script
# Creates an unsigned (or optionally signed) DMG for click install.
# Callers: .github/workflows/release.yml
# Related: packaging/macos/install.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
APP_BUNDLE="${APP_BUNDLE:-${REPO_ROOT}/target/macos/FidelAmharic.app}"
OUT_DIR="${OUT_DIR:-${REPO_ROOT}/dist}"
VERSION="${VERSION:-0.1.0}"
VERSION="${VERSION#v}"
DMG_NAME="fidel-amharic-${VERSION}-macos-universal.dmg"

echo "=== Packaging Fidel Amharic for macOS (${VERSION}) ==="

if [ ! -d "${APP_BUNDLE}" ]; then
    echo "FidelAmharic.app not found. Building..."
    "${REPO_ROOT}/platform/macos-imk/build.sh"
fi

if [ ! -d "${APP_BUNDLE}" ]; then
    echo "Error: FidelAmharic.app missing at ${APP_BUNDLE}"
    exit 1
fi

mkdir -p "${OUT_DIR}"

SIGNING_IDENTITY="${SIGNING_IDENTITY:-}"

if [ -n "${SIGNING_IDENTITY}" ]; then
    echo "Signing FidelAmharic.app with identity: ${SIGNING_IDENTITY}..."
    codesign --deep --force --verify --verbose --options runtime --sign "${SIGNING_IDENTITY}" "${APP_BUNDLE}"
else
    echo "Notice: SIGNING_IDENTITY not set. Using ad-hoc signature (unsigned release)."
    codesign --force --deep --sign - "${APP_BUNDLE}" || true
fi

if command -v hdiutil >/dev/null 2>&1; then
    DMG_TMP="${OUT_DIR}/tmp_fidel_dmg"
    rm -rf "${DMG_TMP}" "${OUT_DIR}/${DMG_NAME}"
    mkdir -p "${DMG_TMP}"
    cp -R "${APP_BUNDLE}" "${DMG_TMP}/"
    cp "${SCRIPT_DIR}/install.sh" "${DMG_TMP}/Install Fidel Amharic.command"
    chmod +x "${DMG_TMP}/Install Fidel Amharic.command"

    cat > "${DMG_TMP}/README.txt" <<EOF
Fidel Amharic ${VERSION} (macOS)

This build is unsigned. macOS Gatekeeper may block it.

Install:
  1. Double-click "Install Fidel Amharic.command"
     OR copy FidelAmharic.app into ~/Library/Input Methods/
  2. If blocked: System Settings -> Privacy & Security -> Allow anyway
     or right-click the app -> Open
  3. System Settings -> Keyboard -> Text Input -> Edit -> + -> Amharic -> Fidel Amharic

To clear quarantine from Terminal:
  xattr -cr "~/Library/Input Methods/FidelAmharic.app"
EOF

    hdiutil create -volname "Fidel Amharic ${VERSION}" -srcfolder "${DMG_TMP}" -ov -format UDZO "${OUT_DIR}/${DMG_NAME}"
    rm -rf "${DMG_TMP}"
    echo "=== DMG Package Created: ${OUT_DIR}/${DMG_NAME} ==="
else
    echo "Notice: hdiutil not available. Packaging as tar.gz..."
    tar -czf "${OUT_DIR}/fidel-amharic-${VERSION}-macos-universal.tar.gz" -C "$(dirname "${APP_BUNDLE}")" "$(basename "${APP_BUNDLE}")"
    echo "=== Tarball Created: ${OUT_DIR}/fidel-amharic-${VERSION}-macos-universal.tar.gz ==="
fi
