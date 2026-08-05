#!/usr/bin/env python3
"""math_extra plugin — run by bl1z, logic lives here.

Reads JSON args from stdin, prints a JSON value to stdout.
bl1z only spawns this script; the math below is beyond the engine's
expression language (modulo, gcd, primes).
"""
import json
import sys
import math


def mod(a, b):
    return a % b


def gcd(a, b):
    return math.gcd(int(a), int(b))


def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True


def primes_up_to(n):
    return [i for i in range(2, int(n) + 1) if is_prime(i)]


FUNCS = {
    "mod": mod,
    "gcd": gcd,
    "is_prime": is_prime,
    "primes_up_to": primes_up_to,
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
