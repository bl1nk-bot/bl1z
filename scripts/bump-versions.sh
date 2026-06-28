#!/usr/bin/env bash
# bump-versions.sh - Update version from phase number
# Usage: ./scripts/bump-versions.sh <phase|major.minor.patch>
# Example: ./scripts/bump-versions.sh 16  OR  ./scripts/bump-versions.sh 0.2.16

set -e

INPUT="${1:-}"
if [ -z "$INPUT" ]; then
    echo "Usage: $0 <phase_number|semver>"
    echo "Example: $0 16    (Phase 16 -> 0.2.16)"
    echo "Example: $0 0.2.20 (explicit semver)"
    exit 1
fi

CONFIG_FILE=".bump-version.json"

# Determine version from input
if [[ "$INPUT" =~ ^[0-9]+$ ]]; then
    # Input is phase number
    PHASE=$INPUT
    if [ "$PHASE" -ge 1 ] && [ "$PHASE" -le 11 ]; then
        VERSION="0.1.${PHASE}"
    elif [ "$PHASE" -ge 12 ] && [ "$PHASE" -le 20 ]; then
        VERSION="0.2.${PHASE}"
    else
        echo "Phase must be 1-20"
        exit 1
    fi
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
    echo "Updated version to ${VERSION}"
fi

echo "Version bump complete: ${VERSION}"
echo "Next: git tag v${VERSION}"