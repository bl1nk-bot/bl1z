#!/usr/bin/env python3
"""string_utils plugin — run by bl1z, logic lives here.

Reads JSON args from stdin, prints a JSON value to stdout.
Regex + unicode handling that the engine's expression language can't do.
"""
import json
import re
import sys


def slugify(s):
    s = s.lower().strip()
    s = re.sub(r"[^a-z0-9]+", "-", s)
    return s.strip("-")


def camel_case(s):
    parts = re.split(r"[^a-z0-9]+", s.lower())
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def count_words(s):
    return len(re.findall(r"\b\w+\b", s))


def starts_with_vowel(s):
    return s[:1].lower() in "aeiou"


FUNCS = {
    "slugify": slugify,
    "camel_case": camel_case,
    "count_words": count_words,
    "starts_with_vowel": starts_with_vowel,
}


def main():
    fn = sys.argv[1]
    args = json.load(sys.stdin)
    if fn not in FUNCS:
        print(json.dumps({"error": f"unknown function: {fn}"}), file=sys.stderr)
        sys.exit(1)
    print(json.dumps(FUNCS[fn](*args)))


if __name__ == "__main__":
    main()
