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
    d = datetime.date.fromisoformat(dt[:10])
    return d.isoformat()


def weekday_name(dt):
    d = datetime.date.fromisoformat(dt[:10])
    return WEEKDAYS_TH[d.weekday()]


def weekday_name_en(dt):
    d = datetime.date.fromisoformat(dt[:10])
    return WEEKDAYS_EN[d.weekday()]


def render(template, name, value):
    return template.replace("{{name}}", str(name)).replace("{{value}}", str(value))


def median(xs):
    return statistics.median(xs)


def percentile(xs, p):
    p = int(p)
    if not 1 <= p <= 99:
        raise ValueError(f"percentile p must be between 1 and 99, got {p}")
    if len(xs) < 2:
        raise ValueError("percentile needs at least 2 data points")
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
    print(json.dumps(FUNCS[fn](*args)))


if __name__ == "__main__":
    main()
