#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — macOS Installation Script
# Installs FidelAmharic.app into ~/Library/Input Methods/
# Works from a release archive/DMG layout or from a source checkout.
#
# Callers: docs/os-installation.md, packaging/macos/package.sh,
#          .github/workflows/release.yml (copied into release archives/DMG).
# User instruction: "Implement the plan as specified" (IME click-install: 1B unsigned, 2A IME only).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DEST_DIR="${HOME}/Library/Input Methods"

echo "=== Installing Fidel Amharic on macOS ==="

resolve_app() {
    if [ -d "${SCRIPT_DIR}/FidelAmharic.app" ]; then
        APP_SOURCE="${SCRIPT_DIR}/FidelAmharic.app"
        return 0
    fi
    if [ -d "${REPO_ROOT}/target/macos/FidelAmharic.app" ]; then
        APP_SOURCE="${REPO_ROOT}/target/macos/FidelAmharic.app"
        return 0
    fi
    if [ -f "${REPO_ROOT}/platform/macos-imk/build.sh" ]; then
        echo "FidelAmharic.app not found. Building from source..."
        "${REPO_ROOT}/platform/macos-imk/build.sh"
        APP_SOURCE="${REPO_ROOT}/target/macos/FidelAmharic.app"
        return 0
    fi
    return 1
}

if ! resolve_app || [ ! -d "${APP_SOURCE}" ]; then
    echo "Error: FidelAmharic.app was not found next to this script and could not be built."
    exit 1
fi

# Clear quarantine so Gatekeeper does not block an unsigned release build.
if command -v xattr >/dev/null 2>&1; then
    xattr -cr "${APP_SOURCE}" 2>/dev/null || true
fi

mkdir -p "${DEST_DIR}"

echo "Killing any existing Fidel Amharic process..."
killall FidelAmharic 2>/dev/null || true

echo "Copying FidelAmharic.app to ${DEST_DIR}/..."
rm -rf "${DEST_DIR}/FidelAmharic.app"
cp -R "${APP_SOURCE}" "${DEST_DIR}/"
xattr -cr "${DEST_DIR}/FidelAmharic.app" 2>/dev/null || true

echo "Refreshing macOS Input Source cache..."
killall -HUP TextInputMenuAgent 2>/dev/null || true
killall -HUP TextInputSwitcher 2>/dev/null || true

echo ""
echo "=== Installation Successful! ==="
echo "This build is unsigned. If macOS blocks it:"
echo "  System Settings -> Privacy & Security -> Allow anyway"
echo "  or: right-click FidelAmharic.app -> Open"
echo ""
echo "To activate Fidel Amharic on macOS:"
echo "1. Open System Settings -> Keyboard -> Text Input -> Edit..."
echo "2. Click '+' at the bottom left."
echo "3. Select 'Amharic' from the language list."
echo "4. Choose 'Fidel Amharic (Power Geez)' and click 'Add'."
echo "5. Switch to Amharic using the Input Menu in your top menu bar or with Ctrl+Space."
echo ""
