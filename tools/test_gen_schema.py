#!/usr/bin/env python3
"""Self-check for tools/gen_schema.py.

Validates the tricky mappings (bare-map root, WKT any, message refs) against
real artifacts. Run: python3 tools/test_gen_schema.py
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import gen_schema as g  # noqa: E402
try:
    import jsonschema  # noqa: E402
except ImportError:
    print("warning: jsonschema not installed — run: pip install jsonschema")
    sys.exit(1)

msgs = g.parse_messages(g.PROTO.read_text())

# 1. Manifest: example plugin JSONs must validate.
manifest = g.message_schema(msgs, "Plugin")
manifest["$defs"] = {n: g.message_schema(msgs, n) for n in g.referenced_from(msgs, "Plugin")}
for p in (g.ROOT / "examples" / "plugins").glob("*.json"):
    jsonschema.validate(g.json.loads(p.read_text()), manifest)
    print(f"  manifest ok: {p.name}")

# 2. Store state: a bare map of PluginEntry (real state.json shape).
store = g.message_schema(msgs, "PluginStoreState")
store["$defs"] = {n: g.message_schema(msgs, n) for n in g.referenced_from(msgs, "PluginStoreState")}
jsonschema.validate(
    {"math_extra": {"enabled": True, "path": "/home/u/.bl1z/plugins/math_extra/plugin.json"}},
    store,
)
try:
    jsonschema.validate({"a": {"enabled": "yes", "path": 1}}, store)
    raise SystemExit("store schema accepted an invalid PluginEntry")
except jsonschema.ValidationError:
    print("  store ok: bare map validates, bad entries rejected")

# 3. Script protocol: WKT fields are "any", not dropped.
proto = g.message_schema(msgs, "ScriptCall")
assert set(proto["properties"]) == {"function", "args", "result"}, proto["properties"]
assert proto["properties"]["args"]["items"].get("description")
print("  protocol ok: function/args/result present, WKT any preserved")

print("gen_schema self-check: ok")
