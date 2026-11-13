#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — macOS Local Installation Script
# Installs FidelAmharic.app into ~/Library/Input Methods/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
APP_SOURCE="${REPO_ROOT}/target/macos/FidelAmharic.app"
DEST_DIR="${HOME}/Library/Input Methods"

echo "=== Installing Fidel Amharic on macOS ==="

if [ ! -d "${APP_SOURCE}" ]; then
    echo "FidelAmharic.app not found at ${APP_SOURCE}."
    echo "Attempting to build first..."
    "${REPO_ROOT}/platform/macos-imk/build.sh"
fi

if [ ! -d "${APP_SOURCE}" ]; then
    echo "Error: FidelAmharic.app build was not found or failed to build."
    exit 1
fi

mkdir -p "${DEST_DIR}"

echo "Killing any existing Fidel Amharic process..."
killall FidelAmharic 2>/dev/null || true

echo "Copying FidelAmharic.app to ${DEST_DIR}/..."
rm -rf "${DEST_DIR}/FidelAmharic.app"
cp -R "${APP_SOURCE}" "${DEST_DIR}/"

echo "Refreshing macOS Input Source cache..."
killall -HUP TextInputMenuAgent 2>/dev/null || true
killall -HUP TextInputSwitcher 2>/dev/null || true

echo ""
echo "=== Installation Successful! ==="
echo "To activate Fidel Amharic on macOS:"
echo "1. Open System Settings -> Keyboard -> Text Input -> Edit..."
echo "2. Click '+' at the bottom left."
echo "3. Select 'Amharic' from the language list."
echo "4. Choose 'Fidel Amharic (Power Geez)' and click 'Add'."
echo "5. Switch to Amharic using the Input Menu in your top menu bar or with Ctrl+Space."
echo ""
