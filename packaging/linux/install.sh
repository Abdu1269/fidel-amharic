#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — Linux IBus/Fcitx5 Installer
# Works from a release tarball (prebuilt binary) or from a source checkout.
# Supports --user (non-root) and --system (sudo) installations.
#
# Callers: README.md, docs/os-installation.md, packaging/linux/build-deb.sh,
#          .github/workflows/release.yml (copied into release archives).
# User instruction: "Implement the plan as specified" (IME click-install: 1B unsigned, 2A IME only).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PLACEHOLDER='@FIDEL_ENGINE_PATH@'

INSTALL_MODE="user"
if [ "${1:-}" = "--system" ]; then
    INSTALL_MODE="system"
elif [ "${1:-}" = "--user" ]; then
    INSTALL_MODE="user"
elif [ -n "${1:-}" ]; then
    echo "Usage: $0 [--user|--system]"
    exit 1
fi

echo "=============================================="
echo " Fidel Amharic — Linux Installer (${INSTALL_MODE})"
echo "=============================================="

resolve_assets() {
    # Release layout: binary + xml + profile next to this script
    if [ -x "${SCRIPT_DIR}/ibus-engine-fidel-amharic" ] \
        && [ -f "${SCRIPT_DIR}/fidel-amharic.xml" ] \
        && [ -f "${SCRIPT_DIR}/power_geez_phonetic_unicode.toml" ]; then
        BIN_SRC="${SCRIPT_DIR}/ibus-engine-fidel-amharic"
        XML_SRC="${SCRIPT_DIR}/fidel-amharic.xml"
        PROFILE_SRC="${SCRIPT_DIR}/power_geez_phonetic_unicode.toml"
        return 0
    fi

    # Source-tree layout: build if needed
    if [ -f "${ROOT_DIR}/Cargo.toml" ]; then
        echo "Building release binary from source..."
        cargo build --release -p linux-ibus --manifest-path "${ROOT_DIR}/Cargo.toml"
        BIN_SRC="${ROOT_DIR}/target/release/linux-ibus"
        XML_SRC="${ROOT_DIR}/platform/linux-ibus/data/fidel-amharic.xml"
        PROFILE_SRC="${ROOT_DIR}/profiles/power_geez_phonetic_unicode.toml"
        return 0
    fi

    echo "Error: could not find prebuilt engine or source tree."
    echo "Download a release package or run from a Fidel checkout."
    exit 1
}

resolve_assets

if [ ! -x "${BIN_SRC}" ]; then
    echo "Error: engine binary not executable: ${BIN_SRC}"
    exit 1
fi
if [ ! -f "${XML_SRC}" ]; then
    echo "Error: missing component XML: ${XML_SRC}"
    exit 1
fi
if [ ! -f "${PROFILE_SRC}" ]; then
    echo "Error: missing profile: ${PROFILE_SRC}"
    exit 1
fi

write_xml() {
    local dest="$1"
    local engine_path="$2"
    sed "s|${PLACEHOLDER}|${engine_path}|g" "${XML_SRC}" > "${dest}"
}

if [ "${INSTALL_MODE}" = "system" ]; then
    INSTALL_BIN="/usr/libexec/ibus-engine-fidel-amharic"
    INSTALL_XML="/usr/share/ibus/component/fidel-amharic.xml"
    INSTALL_PROFILE_DIR="/usr/share/fidel-amharic/profiles"

    echo "Installing system-wide into ${INSTALL_BIN}..."
    sudo install -Dm755 "${BIN_SRC}" "${INSTALL_BIN}"
    sudo mkdir -p "$(dirname "${INSTALL_XML}")"
    write_xml /tmp/fidel-amharic.xml "${INSTALL_BIN}"
    sudo install -Dm644 /tmp/fidel-amharic.xml "${INSTALL_XML}"
    rm -f /tmp/fidel-amharic.xml
    sudo install -Dm644 "${PROFILE_SRC}" "${INSTALL_PROFILE_DIR}/power_geez_phonetic_unicode.toml"
else
    INSTALL_BIN="${HOME}/.local/bin/ibus-engine-fidel-amharic"
    INSTALL_XML="${HOME}/.local/share/ibus/component/fidel-amharic.xml"
    INSTALL_PROFILE_DIR="${HOME}/.local/share/fidel-amharic/profiles"

    echo "Installing for current user into ${INSTALL_BIN}..."
    install -Dm755 "${BIN_SRC}" "${INSTALL_BIN}"
    mkdir -p "$(dirname "${INSTALL_XML}")"
    write_xml "${INSTALL_XML}" "${INSTALL_BIN}"
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
