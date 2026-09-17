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

    def preflight(mapping):
        (workdir / ".bump-version.json").write_text(json.dumps({"phase_to_version": mapping}))
        return subprocess.run(
            ["bash", str(ROOT / "scripts" / "bump-versions.sh"), "1"],
            cwd=workdir,
            text=True,
            capture_output=True,
        )

    result = preflight({"Phase 1-10": "0.1.{phase:x}"})
    assert result.returncode != 0
    assert "must produce X.Y.Z" in result.stderr

    result = preflight({"Phase 1-10": "0.1.{phase}", "Phase 10-20": "0.2.{phase}"})
    assert result.returncode != 0
    assert "overlapping phase range" in result.stderr

print("bump-versions preflight: rejects invalid and overlapping templates")
