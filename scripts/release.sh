#!/bin/bash
# scripts/release.sh - Version and Changelog updater
# Usage: ./scripts/release.sh <phase_number> "<summary>"
# Uses .bump-version.json phase_to_version mapping (same as bump-versions.sh)

set -e

PHASE=$1
SUMMARY=$2

if [[ -z "$PHASE" ]] || [[ -z "$SUMMARY" ]]; then
    echo "Usage: ./scripts/release.sh <phase_number> \"<summary>\""
    echo "Example: ./scripts/release.sh 16 \"Implement plugin ecosystem\""
    exit 1
fi

CONFIG_FILE=".bump-version.json"
if [[ -f "$CONFIG_FILE" ]]; then
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
" "$PHASE") || { echo "Phase $PHASE not found in phase_to_version in $CONFIG_FILE"; exit 1; }
else
    VERSION="0.2.$PHASE"
fi

# Security: validate VERSION contains only safe characters (digits and dots)
# to prevent shell metacharacter injection via template output
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: invalid version format '$VERSION' (expected X.Y.Z)"
    exit 1
fi

DATE=$(date +%Y-%m-%d)

echo "📦 Preparing Release version $VERSION (Phase $PHASE)..."

# 1. Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
echo "✅ Updated Cargo.toml to $VERSION"

# 2. Update CHANGELOG.md
# Insert new entry after ## [Unreleased] or at the top
CHANGELOG_ENTRY="## [$VERSION] - $DATE\n\n### Added\n- (Phase $PHASE): $SUMMARY\n"

if grep -q "## \[Unreleased\]" CHANGELOG.md; then
    sed -i "/## \[Unreleased\]/a \\\n$CHANGELOG_ENTRY" CHANGELOG.md
else
    # If no Unreleased header, insert after the main header
    sed -i "/# Changelog/a \\\n$CHANGELOG_ENTRY" CHANGELOG.md
fi
echo "✅ Updated CHANGELOG.md"

# 3. Commit Report
echo -e "\n📝 Recent commits for Phase $PHASE:"
git log --oneline --grep="Phase $PHASE" -n 10

echo -e "\n🚀 Done! Run 'cargo check' to verify the environment."
