#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo " Fidel Amharic — Linux IBus Installer"
echo "=============================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"

echo "Building release binary..."
cargo build --release -p linux-ibus --manifest-path "${ROOT_DIR}/Cargo.toml"

BIN_SRC="${ROOT_DIR}/target/release/linux-ibus"
XML_SRC="${ROOT_DIR}/platform/linux-ibus/data/fidel-amharic.xml"
PROFILE_SRC="${ROOT_DIR}/profiles/power_geez_phonetic_unicode.toml"

INSTALL_BIN="/usr/libexec/ibus-engine-fidel-amharic"
INSTALL_XML="/usr/share/ibus/component/fidel-amharic.xml"
INSTALL_PROFILE_DIR="/usr/share/fidel-amharic/profiles"

echo "Installing Fidel Amharic engine..."
sudo install -Dm755 "${BIN_SRC}" "${INSTALL_BIN}"
sudo sed "s|/home/abdu/fidel/target/release/linux-ibus|${INSTALL_BIN}|g" "${XML_SRC}" | sudo tee "${INSTALL_XML}" > /dev/null
sudo install -Dm644 "${PROFILE_SRC}" "${INSTALL_PROFILE_DIR}/power_geez_phonetic_unicode.toml"

echo "Restarting IBus daemon..."
ibus restart || true

echo ""
echo "Installation complete!"
echo "To enable Fidel Amharic in GNOME:"
echo "  1. Open Settings -> Keyboard -> Input Sources -> Add ('+')"
echo "  2. Search for 'Amharic (Fidel Amharic)' and add it."
echo "  3. Switch to it using Super+Space or the system top bar menu."
