#!/usr/bin/env bash
set -euo pipefail

# Fidel Amharic — build a Debian .deb for the IBus engine (system install).
# Callers: .github/workflows/release.yml
# Sibling install.sh installs from tarball/source; this produces dpkg .deb.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PLACEHOLDER='@FIDEL_ENGINE_PATH@'

VERSION="${VERSION:-0.1.0}"
VERSION="${VERSION#v}"
ARCH="${ARCH:-amd64}"
PKG_NAME="fidel-amharic"
PKG_DIR="${ROOT_DIR}/target/deb/${PKG_NAME}_${VERSION}_${ARCH}"
OUT_DIR="${OUT_DIR:-${ROOT_DIR}/dist}"

BIN_SRC="${BIN_SRC:-${ROOT_DIR}/target/release/linux-ibus}"
XML_SRC="${ROOT_DIR}/platform/linux-ibus/data/fidel-amharic.xml"
PROFILE_SRC="${ROOT_DIR}/profiles/power_geez_phonetic_unicode.toml"

INSTALL_BIN="/usr/libexec/ibus-engine-fidel-amharic"
INSTALL_XML="/usr/share/ibus/component/fidel-amharic.xml"
INSTALL_PROFILE="/usr/share/fidel-amharic/profiles/power_geez_phonetic_unicode.toml"

if [ ! -x "${BIN_SRC}" ]; then
    echo "Building linux-ibus release binary..."
    cargo build --release -p linux-ibus --manifest-path "${ROOT_DIR}/Cargo.toml"
    BIN_SRC="${ROOT_DIR}/target/release/linux-ibus"
fi

if [ ! -f "${BIN_SRC}" ]; then
    echo "Error: missing binary ${BIN_SRC}"
    exit 1
fi

rm -rf "${PKG_DIR}"
mkdir -p \
    "${PKG_DIR}/DEBIAN" \
    "${PKG_DIR}/usr/libexec" \
    "${PKG_DIR}/usr/share/ibus/component" \
    "${PKG_DIR}/usr/share/fidel-amharic/profiles" \
    "${PKG_DIR}/usr/share/doc/${PKG_NAME}"

install -Dm755 "${BIN_SRC}" "${PKG_DIR}${INSTALL_BIN}"
sed "s|${PLACEHOLDER}|${INSTALL_BIN}|g" "${XML_SRC}" > "${PKG_DIR}${INSTALL_XML}"
install -Dm644 "${PROFILE_SRC}" "${PKG_DIR}${INSTALL_PROFILE}"
install -Dm644 "${ROOT_DIR}/LICENSE" "${PKG_DIR}/usr/share/doc/${PKG_NAME}/copyright"
install -Dm644 "${ROOT_DIR}/README.md" "${PKG_DIR}/usr/share/doc/${PKG_NAME}/README.md"

SIZE_KB="$(du -sk "${PKG_DIR}" | cut -f1)"

cat > "${PKG_DIR}/DEBIAN/control" <<EOF
Package: ${PKG_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Fidel Amharic Contributors <noreply@fidel-amharic.local>
Depends: ibus
Installed-Size: ${SIZE_KB}
Homepage: https://github.com/fidel-amharic/fidel
Description: Native Unicode Amharic IME (IBus)
 Fidel Amharic is a system-wide Ethiopic/Amharic input method using IBus
 and a Power Geez-compatible phonetic profile. Fully offline.
EOF

cat > "${PKG_DIR}/DEBIAN/postinst" <<'EOF'
#!/bin/sh
set -e
# Note: Never run 'ibus restart' in postinst as root; ibus-daemon is a per-user
# session service and running it as root hangs waiting for a session D-Bus.
# Update system cache non-blockingly:
if command -v ibus >/dev/null 2>&1; then
    ibus write-cache --system 2>/dev/null || true
fi
exit 0
EOF
chmod 755 "${PKG_DIR}/DEBIAN/postinst"

cat > "${PKG_DIR}/DEBIAN/prerm" <<'EOF'
#!/bin/sh
set -e
exit 0
EOF
chmod 755 "${PKG_DIR}/DEBIAN/prerm"

mkdir -p "${OUT_DIR}"
DEB_PATH="${OUT_DIR}/${PKG_NAME}_${VERSION}_${ARCH}.deb"
dpkg-deb --build --root-owner-group "${PKG_DIR}" "${DEB_PATH}"
echo "Built ${DEB_PATH}"
