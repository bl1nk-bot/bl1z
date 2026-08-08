#!/usr/bin/env python3
"""obsidian_like plugin — run by bl1z, logic lives here.

Reads JSON args from stdin, prints a JSON value to stdout.
Local dates via the system timezone, real stats (median/percentile), and
template rendering — all things bl1z's expression language lacks natively.
"""
import datetime
import json
import statistics
import sys

WEEKDAYS_TH = ["จันทร์", "อังคาร", "พุธ", "พฤหัสบดี", "ศุกร์", "เสาร์", "อาทิตย์"]
WEEKDAYS_EN = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]


def iso_date(dt):
    # dt is a string like "2026-08-04" or an ISO timestamp
    import datetime as _dt
    if len(dt) > 10 and ("T" in dt or "Z" in dt):
        ts = _dt.datetime.fromisoformat(dt.replace("Z", "+00:00"))
        return ts.astimezone().date().isoformat()
    d = _dt.date.fromisoformat(dt[:10])
    return d.isoformat()


def weekday_name(dt):
    import datetime as _dt
    if len(dt) > 10 and ("T" in dt or "Z" in dt):
        ts = _dt.datetime.fromisoformat(dt.replace("Z", "+00:00"))
        return WEEKDAYS_TH[ts.astimezone().weekday()]
    d = _dt.date.fromisoformat(dt[:10])
    return WEEKDAYS_TH[d.weekday()]


def weekday_name_en(dt):
    import datetime as _dt
    if len(dt) > 10 and ("T" in dt or "Z" in dt):
        ts = _dt.datetime.fromisoformat(dt.replace("Z", "+00:00"))
        return WEEKDAYS_EN[ts.astimezone().weekday()]
    d = _dt.date.fromisoformat(dt[:10])
    return WEEKDAYS_EN[d.weekday()]


def render(template, name, value):
    import re as _re
    return _re.sub(r"\{\{(\w+)\}\}", lambda m: str(name) if m.group(1) == "name" else str(value), template)


def median(xs):
    return statistics.median(xs)


def percentile(xs, p):
    p = int(p)
    if not 0 <= p <= 100:
        raise ValueError(f"percentile p must be between 0 and 100, got {p}")
    if len(xs) < 1:
        raise ValueError("percentile needs at least 1 data point")
    if p == 0:
        return min(xs)
    if p == 100:
        return max(xs)
    if len(xs) < 2:
        raise ValueError("percentile needs at least 2 data points for p=1..99")
    return statistics.quantiles(xs, n=100, method="inclusive")[p - 1]


def completion_rate(tasks):
    if not tasks:
        return 0.0
    return sum(1 for t in tasks if t == "done") / len(tasks)


def today():
    return datetime.date.today().isoformat()


FUNCS = {
    "iso_date": iso_date,
    "weekday_name": weekday_name,
    "weekday_name_en": weekday_name_en,
    "render": render,
    "median": median,
    "percentile": percentile,
    "completion_rate": completion_rate,
    "today": today,
}


def main():
    fn = sys.argv[1]
    args = json.load(sys.stdin)
    if fn not in FUNCS:
        print(json.dumps({"error": f"unknown function: {fn}"}), file=sys.stderr)
        sys.exit(1)
    try:
        print(json.dumps(FUNCS[fn](*args)))
    except (ValueError, TypeError) as e:
        print(json.dumps({"error": f"{fn}: {e}"}), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
