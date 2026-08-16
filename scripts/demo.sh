#!/usr/bin/env bash
# scripts/demo.sh - ตัวอย่างการใช้งาน bl1z จริง (ใช้บันทึกเป็น GIF)
#
# บันทึกเป็นเทอร์มินัล recording:
#     asciinema rec demo.cast --command "bash scripts/demo.sh"
# แปลง .cast เป็น .gif:
#     python3 scripts/cast2gif.py demo.cast demo.gif
set -e
cd "$(dirname "$0")/.."
export COLUMNS="${COLUMNS:-80}"

STORE="$(mktemp -d "${TMPDIR:-/tmp}/bl1z-demo-store.XXXXXX")"
DEV_DIR="$(mktemp -d "${TMPDIR:-/tmp}/bl1z-demo-dev.XXXXXX")"
export BL1Z_PLUGINS_DIR="$STORE"
trap 'rm -rf "$STORE" "$DEV_DIR"' EXIT
mkdir -p "$DEV_DIR/dev_pack"
cat > "$DEV_DIR/dev_pack/plugin.json" <<'EOF'
{"id":"dev_pack","name":"Dev Pack","version":"0.3.0","author":"somchai_dev","runner":"python3","script":"dev.py","functions":[{"name":"sha","params":["s"]}]}
EOF

sleep 0.8
echo '$ bl1z plugins link examples/plugins/math_extra.json'
bl1z plugins link examples/plugins/math_extra.json
sleep 0.9
echo '$ bl1z plugins link ~/.bl1z-demo-dev/dev_pack'
bl1z plugins link "$DEV_DIR/dev_pack"
sleep 0.9
echo '$ bl1z plugins list'
bl1z plugins list
sleep 1.6
echo '$ bl1z eval "mod(17, 5)"'
bl1z eval "mod(17, 5)"
sleep 1.2
echo '$ bl1z eval "gcd(48, 36)"'
bl1z eval "gcd(48, 36)"
sleep 1.2
echo '$ bl1z plugins disable dev_pack'
bl1z plugins disable dev_pack
sleep 0.6
echo '$ bl1z plugins list'
bl1z plugins list
sleep 1.6
echo '$ bl1z plugins list --markdown'
bl1z plugins list --markdown
sleep 1.6
echo ''
