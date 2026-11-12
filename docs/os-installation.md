# Operating System Installation & Activation Guide

This guide covers installing and activating Fidel Amharic as a system-wide input method across supported operating systems.

---

## Linux (GNOME / IBus)

### Prerequisites
- Operating system: Ubuntu 22.04 / 24.04 LTS, Fedora Workstation 38+, or any IBus-enabled desktop.
- `ibus` and `ibus-daemon` installed.

### Installation
Run the automated installation script from the project root:
```bash
./packaging/linux/install.sh
```

Or install manually:
```bash
# 1. Build release binaries
cargo build --release -p linux-ibus

# 2. Install engine binary to system libexec
sudo install -Dm755 target/release/linux-ibus /usr/libexec/ibus-engine-fidel-amharic

# 3. Install IBus component descriptor
sudo install -Dm644 platform/linux-ibus/data/fidel-amharic.xml /usr/share/ibus/component/fidel-amharic.xml

# 4. Restart IBus daemon to pick up the new engine
ibus restart
```

### Activation
1. Open **GNOME Settings** (`gnome-control-center`).
2. Navigate to **Keyboard** -> **Input Sources**.
3. Click the **+** (Add) button.
4. Click the three vertical dots (More), search for **Amharic**, and select **Fidel Amharic (Power Geez)**.
5. Use `Super + Space` to switch between your default layout and Fidel Amharic.

---

## Windows (Text Services Framework — TSF)

### Architecture
On Windows, Fidel Amharic runs as an in-process COM Text Service (`cdylib` DLL) using Windows Text Services Framework (TSF). It links directly to the shared `fidel-engine`.

### Registration
```cmd
regsvr32.exe fidel_amharic.dll
```
Upon registration, the DLL registers its COM CLSID under `HKCU\Software\Classes\CLSID` and calls `ITfInputProcessorProfiles::Register` with Amharic language ID `0x005E`.

### Activation
1. Open **Windows Settings** -> **Time & Language** -> **Language & Region**.
2. If Amharic is not present, click **Add a language** -> choose **Amharic**.
3. Under Amharic options, ensure **Fidel Amharic (TSF)** is selected as the keyboard.
4. Press `Win + Space` to toggle into Fidel Amharic.

---

## macOS (InputMethodKit)

### Architecture
On macOS, Fidel Amharic runs as an Input Method bundle (`FidelAmharic.app`) located in `~/Library/Input Methods/`. It hosts an `IMKServer` and `IMKInputController` subclass written in Swift, linking to the shared Rust static library `libfidel_protocol.a` via C ABI.

### Installation & Activation
1. Copy `FidelAmharic.app` into `~/Library/Input Methods/`:
   ```bash
   cp -R build/FidelAmharic.app ~/Library/Input\ Methods/
   ```
2. Log out and log back in (or restart the input method server).
3. Open **System Settings** -> **Keyboard** -> **Text Input** -> **Edit...**.
4. Click **+**, search for **Amharic**, and add **Fidel Amharic**.
5. Switch to Fidel Amharic using `Ctrl + Space` or the input menu in the top menu bar.
