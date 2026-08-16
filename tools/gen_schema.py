#!/usr/bin/env python3
"""Generate JSON Schemas from proto/bl1z_plugin.proto.

The proto is the single source of truth for the plugin interfaces; this
script derives one JSON Schema per root message (proto3 JSON mapping:
camelCase field names) so schemas are never hand-maintained and can't
drift from the proto.

Usage: python3 tools/gen_schema.py
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PROTO = ROOT / "proto" / "bl1z_plugin.proto"

# Root message -> (output file, schema title, audience).
# audience: "plugin-dev" = ไฟล์ที่คนสร้างปลั๊กอินต้องดู
#           "engine"     = bl1z ภายใน (plugin dev ไม่ต้องแตะ)
# Any new root must be added here or generation fails loudly.
OUTPUTS = {
    "Plugin": ("plugin-manifest.schema.json", "bl1z plugin manifest", "plugin-dev"),
    "PluginStoreState": ("schema-store.schema.json", "bl1z plugin store state", "engine"),
    "ScriptCall": ("plugin-protocol.schema.json", "bl1z plugin script protocol", "plugin-dev"),
}

# proto scalar types -> JSON Schema types.
SCALARS = {
    "string": "string",
    "bool": "boolean",
    "int32": "integer",
    "int64": "string",
    "uint32": "integer",
    "uint64": "string",
    "sint32": "integer",
    "sint64": "string",
    "fixed32": "integer",
    "fixed64": "string",
    "sfixed32": "integer",
    "sfixed64": "string",
    "double": "number",
    "float": "number",
}
# Well-known protobuf types mapped to plain JSON.
WKT_ANY = {"google.protobuf.Value"}


def camel(name: str) -> str:
    """snake_case -> camelCase (proto3 JSON mapping)."""
    parts = name.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def parse_messages(text: str):
    """Return {msg_name: [field, ...]} where each field carries
    optional/repeated/map/type/name."""
    messages = {}
    current = None
    for line in text.splitlines():
        line = line.strip()
        m = re.match(r"message (\w+) \{", line)
        if m:
            current = m.group(1)
            messages[current] = []
            continue
        if line == "}":
            current = None
            continue
        if not (current and line and not line.startswith("//")):
            continue
        mm = re.match(r"map<string,\s*(\w+)>\s*(\w+) = \d+;", line)
        if mm:
            messages[current].append(
                {"map": True, "type": mm.group(1), "name": mm.group(2)}
            )
            continue
        fm = re.match(
            r"(optional\s+)?(repeated\s+)?([\w.]+) (\w+) = \d+;", line
        )
        if fm:
            messages[current].append(
                {
                    "optional": bool(fm.group(1)),
                    "repeated": bool(fm.group(2)),
                    "type": fm.group(3),
                    "name": fm.group(4),
                }
            )
            continue
        sys.exit(f"cannot parse proto line in message {current}: `{line}`")
    return messages


def referenced_from(messages, name, seen=None):
    """Transitively collect message names reachable from `name`."""
    seen = seen if seen is not None else set()
    for f in messages[name]:
        if f["type"] in messages and f["type"] not in seen:
            seen.add(f["type"])
            referenced_from(messages, f["type"], seen)
    return seen


def base_schema(field, defs):
    """JSON Schema for one field's bare value (not repeated/map-wrapped)."""
    t = field["type"]
    if t in defs:
        return {"$ref": f"#/$defs/{t}"}
    if t in SCALARS:
        return {"type": SCALARS[t]}
    if t in WKT_ANY:
        return {"description": "any JSON value (null, number, string, bool, array, object)"}
    sys.exit(f"unhandled proto type `{t}` in field `{field['name']}`")


def field_schema(field, defs):
    base = base_schema(field, defs)
    if field.get("map"):
        return {"type": "object", "additionalProperties": base}
    if field["repeated"]:
        return {"type": "array", "items": base}
    return base


def message_schema(messages, name, is_root=False):
    """Object schema for one message (no $defs — those live at top level)."""
    fields = messages[name]
    defs = referenced_from(messages, name)
    schema = {
        "type": "object",
        "required": [
            camel(f["name"])
            for f in fields
            if not f.get("optional") and not f.get("repeated") and not f.get("map")
        ],
        "properties": {
            camel(f["name"]): field_schema(f, defs) for f in fields
        },
    }
    # Bare-map document: ONLY at root level, a root message whose only field
    # is a map renders as the map itself (e.g. state.json is { "<id>": {...} }).
    if is_root and len(fields) == 1 and fields[0].get("map"):
        del schema["required"]
        del schema["properties"]
        # The document IS the map: no map wrapper at the root.
        schema["additionalProperties"] = base_schema(fields[0], defs)
    return schema


def main():
    messages = parse_messages(PROTO.read_text())
    referenced_anywhere = set()
    for msg_name, fields in messages.items():
        for f in fields:
            if f["type"] in messages and f["type"] != msg_name:
                referenced_anywhere.add(f["type"])
    roots = [n for n in messages if n not in referenced_anywhere]
    unknown = [r for r in roots if r not in OUTPUTS]
    if unknown:
        sys.exit(f"root message(s) {unknown} missing from OUTPUTS in tools/gen_schema.py")

    for name in roots:
        filename, title, audience = OUTPUTS[name]
        out = ROOT / filename
        defs = referenced_from(messages, name)
        schema: dict = {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": f"https://raw.githubusercontent.com/bl1nk-bot/bl1z/main/{filename}",
            "$comment": (
                "owner: plugin-dev — ไฟล์ที่คนสร้างปลั๊กอินต้องดู"
                if audience == "plugin-dev"
                else "owner: engine — bl1z ภายใน, plugin dev ไม่ต้องแตะ"
            ),
            "title": title,
            "description": (
                f"GENERATED from {PROTO.relative_to(ROOT).as_posix()} by "
                "tools/gen_schema.py — do not edit by hand."
            ),
            "type": "object",
        }
        if defs:
            schema["$defs"] = {
                n: message_schema(messages, n) for n in sorted(defs)
            }
        body = message_schema(messages, name, is_root=True)
        if "additionalProperties" in body:
            schema["additionalProperties"] = body.pop("additionalProperties")
        schema.update(body)  # required, then properties

        out.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
        print(f"wrote {out.relative_to(ROOT)} from {PROTO.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
