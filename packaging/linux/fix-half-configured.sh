#!/usr/bin/env bash
set -euo pipefail

echo "=== Cleaning stuck Fidel / apt / ibus install processes ==="
# Kill only Fidel-related hung install leftovers
pkill -f '/var/lib/dpkg/info/fidel-amharic.postinst' 2>/dev/null || true
# Kill root ibus restart left from broken 0.1.1 postinst
pkill -f '^ibus restart$' 2>/dev/null || true
# Kill stopped/zombie apt sessions that were installing fidel
pkill -f 'apt install .*fidel-amharic' 2>/dev/null || true

# Ensure postinst can never hang again
cat > /var/lib/dpkg/info/fidel-amharic.postinst <<'POST'
#!/bin/sh
set -e
exit 0
POST
chmod 755 /var/lib/dpkg/info/fidel-amharic.postinst

dpkg --configure -a || true

echo ""
echo "=== Package status ==="
dpkg -l fidel-amharic

echo ""
echo "=== Installed files ==="
ls -la /usr/libexec/ibus-engine-fidel-amharic \
      /usr/share/ibus/component/fidel-amharic.xml \
      /usr/share/fidel-amharic/profiles/power_geez_phonetic_unicode.toml

echo ""
echo "=== Done ==="
echo "As your normal user (NOT sudo), run:"
echo "  ibus write-cache || true"
echo "  ibus restart || true"
echo "Then: Settings → Keyboard → Input Sources → + → Amharic (Fidel Amharic)"
