#!/usr/bin/env bash
# bump-versions.sh - Update version from phase number
# Usage: ./scripts/bump-versions.sh <phase|major.minor.patch>
# Example: ./scripts/bump-versions.sh 16  OR  ./scripts/bump-versions.sh 0.2.16
#
# หลักการ: ไม่มี hardcode — phase->version มาจาก .bump-version.json
# (phase_to_version) และทุกไฟล์ที่ต้องเปลี่ยนมี marker/ตำแหน่งของตัวเอง:
#   - Cargo.toml          : version = "..."
#   - README + .th.md     : badge version-X.Y.Z (sync_docs.py)
#   - CHANGELOG + .th.md  : marker `## [Unreleased]` -> เปลี่ยนเป็นเวอร์ชันใหม่
#   - .bump-version.json  : current/next อัปเดตเอง

set -e

INPUT="${1:-}"
if [ -z "$INPUT" ]; then
    echo "Usage: $0 <phase_number|semver>"
    echo "Example: $0 16    (Phase 16 -> 0.2.16)"
    echo "Example: $0 0.2.20 (explicit semver)"
    exit 1
fi

CONFIG_FILE=".bump-version.json"

# Determine version from input — phase mapping อ่านจาก config ไม่ hardcode
if [[ "$INPUT" =~ ^[0-9]+$ ]]; then
    PHASE=$INPUT
    VERSION=$(python3 -c "
import json, sys
cfg = json.load(open('$CONFIG_FILE'))
phase = int(sys.argv[1])
for span, template in cfg['phase_to_version'].items():
    lo, hi = map(int, span.replace('Phase ', '').split('-'))
    if lo <= phase <= hi:
        print(template.format(phase=phase))
        break
else:
    sys.exit(1)
" "$PHASE") || { echo "Phase $PHASE ไม่อยู่ในช่วง phase_to_version ใน $CONFIG_FILE"; exit 1; }
else
    # Input is semver
    if [[ ! "$INPUT" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Version must be a semver like 0.2.16"
        exit 1
    fi
    VERSION="$INPUT"
fi

# Update Cargo.toml
if [ -f "Cargo.toml" ]; then
    sed -i "s/^version = \"[^\"]*\"$/version = \"${VERSION}\"/" Cargo.toml
    echo "Updated Cargo.toml to ${VERSION}"
fi

# Sync doc version refs (README + docs/th/README.th.md) — กลไก, ไม่ต้องมือแก้
if command -v python3 >/dev/null 2>&1; then
    python3 tools/sync_docs.py version "$VERSION"
    # Changelog: marker `## [Unreleased]` -> `## [VERSION] - date` (EN + TH)
    python3 tools/sync_docs.py changelog "$VERSION"
else
    echo "warning: python3 not found — skipping doc sync and changelog"
fi

# Update .bump-version.json state (current/next)
if [ -f "$CONFIG_FILE" ]; then
    python3 - "$VERSION" <<'PY'
import json, sys
cfg = json.load(open(".bump-version.json"))
major, minor, patch = map(int, sys.argv[1].split("."))
cfg["current_version"] = sys.argv[1]
cfg["next_version"] = f"{major}.{minor}.{patch + 1}"
json.dump(cfg, open(".bump-version.json", "w"), indent=2, ensure_ascii=False)
print(f"Updated .bump-version.json: current={sys.argv[1]}, next={cfg['next_version']}")
PY
fi

echo "Version bump complete: ${VERSION}"
echo "Next: เติม changelog entries ที่ [Unreleased] (ถ้ายังว่าง), cargo check (sync Cargo.lock), git tag v${VERSION}"
