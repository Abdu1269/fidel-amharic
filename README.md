# Fidel Amharic (ፊደል አማርኛ)

> Native, system-wide Unicode Ethiopic input method for Windows, macOS, and Linux.

Fidel Amharic is an open-source native input method editor (IME) designed to allow typing Ethiopic / Ge'ez script (`am`) across all third-party applications, without using simulated keystrokes, clipboard paste automation, or global hooks.

---

## Architecture Overview

```text
       Target Applications (Word, Browser, Terminal, TextEdit)
                                 |
                     Native OS Input Framework
            (Windows TSF | macOS InputMethodKit | Linux IBus)
                                 |
                    Shared Rust Engine (fidel-engine)
                     - Deterministic Trie composer
                     - Declarative TOML profiles (520+ rules)
                     - Pure Unicode Ethiopic output
                                 |
                 Tauri 2 Companion Settings App
                   (Settings, Visualizer, Diagnostics)
```

## Workspace Crates

| Crate | Purpose |
|---|---|
| [`fidel-core`](crates/fidel-core) | Profile schemas, validators, serialization, and profile registry |
| [`fidel-engine`](crates/fidel-engine) | Zero-GUI deterministic Trie composition state machine |
| [`fidel-protocol`](crates/fidel-protocol) | C ABI (`extern "C"`) and C headers for native OS adapters |
| [`fidel-diagnostics`](crates/fidel-diagnostics) | Privacy-safe diagnostic reports (no keystroke logging) |
| [`linux-ibus`](platform/linux-ibus) | Native Linux IBus engine daemon using pure Rust D-Bus (`zbus`) |

## Download (click install)

Prebuilt installers are published on [GitHub Releases](https://github.com/fidel-amharic/fidel/releases):

| Platform | Artifact | Install |
|---|---|---|
| Linux | `.deb` | `sudo dpkg -i fidel-amharic_*_amd64.deb` |
| Windows | `.msi` | Double-click (Admin). If SmartScreen warns: More info → Run anyway |
| macOS | `.dmg` | Open DMG → run **Install Fidel Amharic**. If Gatekeeper blocks: Allow anyway |

Builds are currently **unsigned**. Full steps: [OS Installation Guide](docs/os-installation.md).

## Quick Start (Linux from source)

### Build and Test
```bash
# Run all workspace tests (25 test suites)
cargo test --workspace

# Run linter
cargo clippy --workspace -- -D warnings

# Build release binaries
cargo build --release --workspace
```

### Install IBus Engine
```bash
./packaging/linux/install.sh
```

Then add **Fidel Amharic (Power Geez)** in your desktop's Keyboard / Input Sources settings.

## Documentation

- [System Architecture](docs/architecture.md)
- [Keyboard Layouts & Mappings](docs/keyboard-layouts.md)
- [OS Installation Guide](docs/os-installation.md)
- [Privacy & Security Policy](docs/privacy-security.md)
- [Third-Party Notices](THIRD_PARTY_NOTICES.md)

## License

This project is licensed under the [MIT License](LICENSE). Third-party keyboard mappings and attributions are documented in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
