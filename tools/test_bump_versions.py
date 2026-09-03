#!/usr/bin/env python3
"""Regression check for release-version template preflight."""
import json
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

with tempfile.TemporaryDirectory() as temp:
    workdir = Path(temp)
    (workdir / "tools").mkdir()
    (workdir / "tools" / "sync_docs.py").touch()
    (workdir / "tools" / "resolve_version.py").touch()
    (workdir / ".bump-version.json").write_text(
        json.dumps({"phase_to_version": {"Phase 1-10": "0.1.{phase:x}"}})
    )
    result = subprocess.run(
        ["bash", str(ROOT / "scripts" / "bump-versions.sh"), "1"],
        cwd=workdir,
        text=True,
        capture_output=True,
    )

assert result.returncode != 0
assert "must produce X.Y.Z" in result.stderr
print("bump-versions preflight: rejects phase-dependent invalid templates")
