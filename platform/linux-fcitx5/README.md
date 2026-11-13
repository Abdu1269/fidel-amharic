# Fidel Amharic — Fcitx5 Integration

Fidel Amharic integrates with Linux Fcitx5 in two complementary ways:

## 1. Native IBus Integration via `fcitx5-module-ibus` (Recommended)

Fcitx5 provides full native support for IBus components through its `fcitx5-module-ibus` sub-system. 
When the Fidel Amharic IBus engine descriptor is installed (via `packaging/linux/install.sh`), Fcitx5 automatically discovers `ibus-engine-fidel-amharic` and lists **Amharic (Fidel Amharic)** as an available input method under the Fcitx5 configuration panel.

### Requirements:
- `fcitx5`
- `fcitx5-module-ibus` (included in Ubuntu `fcitx5-frontend-ibus` or Fedora `fcitx5-ibus`)

### Activation:
1. Install via `.deb` (`sudo dpkg -i fidel-amharic_*.deb`) or run `./packaging/linux/install.sh --user` / `--system`.
2. Run `fcitx5-remote -r` to reload Fcitx5.
3. In Fcitx5 Configuration Tool, search for **Amharic (Fidel Amharic)** and add it to your active input methods.

## 2. Direct Fcitx5 Addon Configuration

For environments using standalone Fcitx5 configurations without the IBus bridge:
- Input Method Config: `data/fidel-amharic.conf` placed in `~/.local/share/fcitx5/inputmethod/`
- Addon Config: `data/fidel.conf` placed in `~/.local/share/fcitx5/addon/`
