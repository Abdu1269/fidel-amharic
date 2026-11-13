# Operating System Installation & Activation Guide

This guide covers installing and activating Fidel Amharic as a system-wide input method across supported operating systems.

Release installers are **unsigned**. Windows SmartScreen and macOS Gatekeeper may show warnings; use the bypass steps below.

Download builds from the [GitHub Releases](https://github.com/fidel-amharic/fidel/releases) page.

---

## Linux (GNOME / IBus)

### Prerequisites
- Operating system: Ubuntu 22.04 / 24.04 LTS, Debian, or any IBus-enabled desktop.
- `ibus` and `ibus-daemon` installed.

### Click install (.deb)
```bash
sudo dpkg -i fidel-amharic_*_amd64.deb
```

### Portable tarball (no root)
```bash
tar xzf fidel-amharic-*-linux-x86_64.tar.gz
cd fidel-amharic-*-linux-x86_64
./install.sh --user
```

Use `./install.sh --system` for a system-wide install (requires sudo).

### From source
```bash
./packaging/linux/install.sh --user
# or build a local .deb:
# VERSION=0.1.0 ./packaging/linux/build-deb.sh
```

Manual install (after `cargo build --release -p linux-ibus`):
```bash
sudo install -Dm755 target/release/linux-ibus /usr/libexec/ibus-engine-fidel-amharic
sed 's|@FIDEL_ENGINE_PATH@|/usr/libexec/ibus-engine-fidel-amharic|g' \
  platform/linux-ibus/data/fidel-amharic.xml | \
  sudo tee /usr/share/ibus/component/fidel-amharic.xml > /dev/null
sudo install -Dm644 profiles/power_geez_phonetic_unicode.toml \
  /usr/share/fidel-amharic/profiles/power_geez_phonetic_unicode.toml
ibus restart
```

### Activation
1. Open **GNOME Settings** (`gnome-control-center`).
2. Navigate to **Keyboard** -> **Input Sources**.
3. Click the **+** (Add) button.
4. Search for **Amharic**, and select **Fidel Amharic (Power Geez)**.
5. Use `Super + Space` to switch between your default layout and Fidel Amharic.

---

## Windows (Text Services Framework — TSF)

### Architecture
On Windows, Fidel Amharic runs as an in-process COM Text Service (`cdylib` DLL) using Windows Text Services Framework (TSF). It links directly to the shared `fidel-engine`.

### Click install (.msi)
1. Double-click `fidel-amharic-*-windows-x86_64.msi`.
2. Allow elevation when prompted.
3. If **SmartScreen** appears: click **More info** → **Run anyway**.

### Zip fallback
1. Extract the release zip.
2. Right-click `install.ps1` → **Run with PowerShell as Administrator**.
3. The script registers `fidel_amharic.dll` via `regsvr32`.

### Manual registration
```cmd
regsvr32.exe fidel_amharic.dll
```

### Activation
1. Open **Windows Settings** -> **Time & Language** -> **Language & Region**.
2. If Amharic is not present, click **Add a language** -> choose **Amharic**.
3. Under Amharic options, ensure **Fidel Amharic** is selected as the keyboard.
4. Press `Win + Space` to toggle into Fidel Amharic.

### Uninstall
- Use **Apps & features** for the MSI, or run `uninstall.ps1` as Administrator from the zip.

---

## macOS (InputMethodKit)

### Architecture
On macOS, Fidel Amharic runs as an Input Method bundle (`FidelAmharic.app`) located in `~/Library/Input Methods/`. It hosts an `IMKServer` and `IMKInputController` subclass written in Swift, linking to the shared Rust static library `libfidel_protocol.a` via C ABI.

### Click install (.dmg)
1. Open `fidel-amharic-*-macos-universal.dmg`.
2. Double-click **Install Fidel Amharic.command**.
3. If **Gatekeeper** blocks the app:
   - **System Settings → Privacy & Security → Allow anyway**, or
   - Right-click `FidelAmharic.app` → **Open**, or
   - `xattr -cr "~/Library/Input Methods/FidelAmharic.app"`

### Tarball fallback
```bash
tar xzf fidel-amharic-*-macos-universal.tar.gz
cd fidel-amharic-*-macos-universal
./install.sh
```

### From source
```bash
./packaging/macos/install.sh
# optional DMG:
# VERSION=0.1.0 ./packaging/macos/package.sh
```

### Activation
1. Open **System Settings** -> **Keyboard** -> **Text Input** -> **Edit...**.
2. Click **+**, search for **Amharic**, and add **Fidel Amharic**.
3. Switch to Fidel Amharic using `Ctrl + Space` or the input menu in the top menu bar.
