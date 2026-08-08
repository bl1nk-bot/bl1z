#!/usr/bin/env python3
"""Doc sync — 1 หัวข้อ = 1 ไฟล์ต้นฉบับ, ฉบับแปล (docs/th/*.th.md) เกิดจากต้นฉบับ.

หลักการ:
  - เนื้อหา (prose) อยู่ที่ต้นฉบับที่เดียว; ไฟล์แปลแปลจากต้นฉบับ ไม่มีมือแก้ขนาน
  - ค่าเชิงกล (สถานะ checkbox, version tokens) คัดจากต้นฉบับโดยสคริปต์
  - drift ใดๆ (หัวข้อ/checkbox/เวอร์ชันไม่ตรง) ตรวจเจอแบบ fail-fast

Usage:
  python3 tools/sync_docs.py check      # ตรวจทุกคู่ (src ↔ .th.md) + invariant
  python3 tools/sync_docs.py sync       # คัดค่ากลไกจากต้นฉบับไปฉบับแปล
  python3 tools/sync_docs.py version    # อัปเดตเวอร์ชันใน README + ฉบับแปล (อ่านจาก Cargo.toml)
  python3 tools/sync_docs.py version X.Y.Z
  python3 tools/sync_docs.py self-test
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TH = ROOT / "docs" / "th"

VERSION_RE = re.compile(r"\d+\.\d+\.\d+")
CHECKBOX_RE = re.compile(r"^\s*-\s*\[( |x)\]", re.M)
HEADING_RE = re.compile(r"^#{1,6} ", re.M)

# เอกสารที่ไม่มีฉบับแปล (เป็นเอกสารภายใน ไม่ต้อง mirror)
NO_MIRROR = {"PLAN.md"}


def current_version() -> str:
    m = re.search(r'^version = "([^"]+)"', (ROOT / "Cargo.toml").read_text(), re.M)
    if not m:
        sys.exit("Cargo.toml: ไม่พบ version")
    return m.group(1)


def source_md_files():
    files = [p for p in sorted(ROOT.glob("*.md")) if p.name not in NO_MIRROR]
    files += sorted((ROOT / "docs").rglob("*.md"))
    return [p for p in files if "th" not in p.parts or p.parts[p.parts.index("docs") + 1] != "th"]


def mirror_for(src: Path) -> Path:
    """README.md -> docs/th/README.th.md; docs/X/Y.md -> docs/th/X/Y.th.md"""
    rel = src.relative_to(ROOT).parts
    stem = list(rel[1:]) if rel[0] == "docs" else list(rel)
    return TH.joinpath(*stem[:-1], stem[-1][:-3] + ".th.md")


def analyze(text: str):
    return {
        "headings": len(HEADING_RE.findall(text)),
        "checkboxes": CHECKBOX_RE.findall(text),
        "versions": VERSION_RE.findall(text),
    }


def pairs():
    out = []
    for src in source_md_files():
        m = mirror_for(src)
        if m.exists():
            out.append((src, m))
    return out


def invariants():
    """1 เอกสาร = 1 หน้าที่: ตรวจว่าไม่มีการแตกเอกสาร/ย้ายหน้าที่แล้วขัดกัน."""
    problems = []
    if (ROOT / "TODO.md").exists():
        problems.append("TODO.md ยังอยู่ — ถูก merge เข้า PLAN.md แล้ว ต้องลบ (ดู PLAN.md)")
    spec = (ROOT / "SPEC.md").read_text()
    for kw in ("Cranelift", "Wasmtime", "tower-lsp"):
        if kw in spec:
            problems.append(f"SPEC.md ยังมี roadmap item ({kw}) — roadmap ต้องอยู่ใน PLAN.md เท่านั้น")
    return problems


def sync():
    changed = []
    for src, m in pairs():
        a = analyze(src.read_text())
        b = analyze(m.read_text())
        if a["headings"] != b["headings"] or a["checkboxes"] != b["checkboxes"] or a["versions"] != b["versions"]:
            sys.exit(f"{m}: drift กับต้นฉบับ — รัน `check` ก่อน (checkbox {a['checkboxes']} vs {b['checkboxes']}, versions {a['versions']} vs {b['versions']})")
        # สถานะ checkbox: คัดจากต้นฉบับตามลำดับ
        states = iter(a["checkboxes"])
        new = [re.sub(r"\[[ x]\]", f"[{next(states)}]", ln, count=1) if CHECKBOX_RE.match(ln) else ln
               for ln in m.read_text().splitlines(keepends=True)]
        out = "".join(new)
        # version tokens: แทนที่ตามลำดับตำแหน่ง (โครงสร้าง mirror ต้องตรงกัน)
        a_tok, b_tok = VERSION_RE.findall(src.read_text()), VERSION_RE.findall(m.read_text())
        if len(a_tok) == len(b_tok) and a_tok != b_tok:
            it = iter(a_tok)
            out = re.sub(VERSION_RE, lambda _: next(it), out)
        if out != m.read_text():
            m.write_text(out)
            changed.append(str(m.relative_to(ROOT)))
    return changed


def sync_version(v: str | None = None):
    v = v or current_version()
    changed = []
    for f in (ROOT / "README.md", TH / "README.th.md"):
        t = f.read_text()
        n = re.sub(r"version-\d+\.\d+\.\d+", f"version-{v}", t)
        n = re.sub(r'bl1z = "\d+\.\d+\.\d+"', f'bl1z = "{v}"', n)
        if n != t:
            f.write_text(n)
            changed.append(str(f.relative_to(ROOT)))
    return v, changed


def changelog_entry(v: str):
    """Rename marker `## [Unreleased]` -> `## [V] - date` in EN+TH changelog,
    then insert a fresh `## [Unreleased]` above it. Marker must exist exactly once."""
    from datetime import date

    today = date.today().isoformat()
    header = f"## [{v}] - {today}"
    changed = []
    for f in (ROOT / "CHANGELOG.md", TH / "CHANGELOG.th.md"):
        t = f.read_text()
        if f"## [{v}]" in t:
            sys.exit(f"{f}: entry `## [{v}]` มีอยู่แล้ว — เลือกเวอร์ชันอื่น")
        if t.count("## [Unreleased]") != 1:
            sys.exit(f"{f}: ต้องมี marker `## [Unreleased]` 1 จุด (เป็นตำแหน่งแทรก) — เพิ่มก่อนรัน")
        n = t.replace("## [Unreleased]", header, 1)
        n = n.replace(header, "## [Unreleased]\n\n" + header, 1)
        if n != t:
            f.write_text(n)
            changed.append(str(f.relative_to(ROOT)))
    return v, today, changed


def self_test():
    nl = chr(10)
    sample = nl.join(["- [x] a", "- [ ] b", "# H", "v0.2.16"]) + nl
    a = analyze(sample)
    assert a == {"headings": 1, "checkboxes": ["x", " "], "versions": ["0.2.16"]}, a
    assert mirror_for(ROOT / "README.md") == TH / "README.th.md"
    assert mirror_for(ROOT / "docs" / "codedocs" / "architecture.md") == TH / "codedocs" / "architecture.th.md"
    assert mirror_for(ROOT / "docs" / "BENCHMARKS.md") == TH / "BENCHMARKS.th.md"
    print("self-test ok")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    cmd = sys.argv[1]
    if cmd == "self-test":
        self_test()
        return
    if cmd == "check":
        ok = True
        for p in invariants():
            print("INVARIANT:", p)
            ok = False
        for src, m in pairs():
            a, b = analyze(src.read_text()), analyze(m.read_text())
            if a == b:
                print(f"ok: {m.relative_to(ROOT)}")
                continue
            ok = False
            print(f"MISMATCH: {m.relative_to(ROOT)}")
            if a["headings"] != b["headings"]:
                print(f"  headings: src={a['headings']} vs th={b['headings']}")
            if a["checkboxes"] != b["checkboxes"]:
                print(f"  checkboxes: src={a['checkboxes']} vs th={b['checkboxes']}")
            if a["versions"] != b["versions"]:
                print(f"  versions: src={a['versions']} vs th={b['versions']}")
        sys.exit(0 if ok else 1)
    if cmd == "sync":
        changed = sync()
        print("synced:" if changed else "nothing to sync")
        for c in changed:
            print(" ", c)
        return
    if cmd == "version":
        v, changed = sync_version(sys.argv[2] if len(sys.argv) > 2 else None)
        print(f"version {v}:")
        for c in changed or ["(no doc refs to change)"]:
            print(" ", c)
        return
    if cmd == "changelog":
        if len(sys.argv) < 3:
            sys.exit("usage: sync_docs.py changelog <X.Y.Z>")
        v, today, changed = changelog_entry(sys.argv[2])
        print(f"changelog {v} ({today}):")
        for c in changed or ["(nothing to change)"]:
            print(" ", c)
        return
    sys.exit(f"unknown command `{cmd}` — ดู usage ด้านบน")


if __name__ == "__main__":
    main()
