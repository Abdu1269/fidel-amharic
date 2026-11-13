#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — Linux IBus/Fcitx5 Installer
# Supports both --user (non-root) and --system (with sudo) installations.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"

INSTALL_MODE="user"
if [ "${1:-}" = "--system" ]; then
    INSTALL_MODE="system"
elif [ "${1:-}" = "--user" ]; then
    INSTALL_MODE="user"
fi

echo "=============================================="
echo " Fidel Amharic — Linux Installer (${INSTALL_MODE})"
echo "=============================================="

echo "Building release binary..."
cargo build --release -p linux-ibus --manifest-path "${ROOT_DIR}/Cargo.toml"

BIN_SRC="${ROOT_DIR}/target/release/linux-ibus"
XML_SRC="${ROOT_DIR}/platform/linux-ibus/data/fidel-amharic.xml"
PROFILE_SRC="${ROOT_DIR}/profiles/power_geez_phonetic_unicode.toml"

if [ "${INSTALL_MODE}" = "system" ]; then
    INSTALL_BIN="/usr/libexec/ibus-engine-fidel-amharic"
    INSTALL_XML="/usr/share/ibus/component/fidel-amharic.xml"
    INSTALL_PROFILE_DIR="/usr/share/fidel-amharic/profiles"

    echo "Installing system-wide into ${INSTALL_BIN}..."
    sudo install -Dm755 "${BIN_SRC}" "${INSTALL_BIN}"
    sudo mkdir -p "$(dirname "${INSTALL_XML}")"
    sed "s|/home/abdu/fidel/target/release/linux-ibus|${INSTALL_BIN}|g" "${XML_SRC}" | sudo tee "${INSTALL_XML}" > /dev/null
    sudo install -Dm644 "${PROFILE_SRC}" "${INSTALL_PROFILE_DIR}/power_geez_phonetic_unicode.toml"
else
    INSTALL_BIN="${HOME}/.local/bin/ibus-engine-fidel-amharic"
    INSTALL_XML="${HOME}/.local/share/ibus/component/fidel-amharic.xml"
    INSTALL_PROFILE_DIR="${HOME}/.local/share/fidel-amharic/profiles"

    echo "Installing for current user into ${INSTALL_BIN}..."
    install -Dm755 "${BIN_SRC}" "${INSTALL_BIN}"
    mkdir -p "$(dirname "${INSTALL_XML}")"
    sed "s|/home/abdu/fidel/target/release/linux-ibus|${INSTALL_BIN}|g" "${XML_SRC}" > "${INSTALL_XML}"
    install -Dm644 "${PROFILE_SRC}" "${INSTALL_PROFILE_DIR}/power_geez_phonetic_unicode.toml"
fi

echo "Updating IBus cache and restarting daemon..."
if command -v ibus >/dev/null 2>&1; then
    timeout 3s ibus write-cache 2>/dev/null || true
    timeout 3s ibus restart 2>/dev/null || true
fi

if command -v fcitx5-remote >/dev/null 2>&1; then
    echo "Reloading Fcitx5..."
    fcitx5-remote -r 2>/dev/null || true
fi

echo ""
echo "=== Installation complete! ==="
echo "To enable Fidel Amharic:"
echo "  1. Open Settings -> Keyboard -> Input Sources -> Add ('+')"
echo "  2. Search for 'Amharic (Fidel Amharic)' and add it."
echo "  3. Switch to it using Super+Space or the system top bar menu."
