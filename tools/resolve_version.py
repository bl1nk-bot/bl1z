#!/usr/bin/env python3
"""Shared phase→version resolver for release/bump scripts.

Usage: python3 tools/resolve_version.py <phase_number>
Reads .bump-version.json and prints the resolved version string (X.Y.Z).
Exit 1 on parse errors or unmatched phase.
"""
import json
import sys
from pathlib import Path

CONFIG = Path(__file__).resolve().parent.parent / ".bump-version.json"

def main():
    if len(sys.argv) != 2:
        print("usage: resolve_version.py <phase>", file=sys.stderr)
        sys.exit(1)
    try:
        cfg = json.loads(CONFIG.read_text())
    except (json.JSONDecodeError, OSError) as e:
        print(f"error: cannot parse {CONFIG}: {e}", file=sys.stderr)
        sys.exit(1)
    if "phase_to_version" not in cfg or not isinstance(cfg["phase_to_version"], dict):
        print(f"error: {CONFIG} missing or invalid phase_to_version", file=sys.stderr)
        sys.exit(1)
    try:
        phase = int(sys.argv[1])
    except ValueError:
        print(f"error: phase must be an integer, got '{sys.argv[1]}'", file=sys.stderr)
        sys.exit(1)
    for span, template in cfg["phase_to_version"].items():
        lo, hi = map(int, span.replace("Phase ", "").split("-"))
        if lo <= phase <= hi:
            print(template.format(phase=phase))
            return
    print(f"error: Phase {phase} not found in phase_to_version", file=sys.stderr)
    sys.exit(1)

if __name__ == "__main__":
    main()
