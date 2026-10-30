#!/usr/bin/env python3
"""Write the canonical-decimal corpus for the temperature identity's Rust test.

Each line is a binary64 bit pattern (hex) and Python's ``repr`` of it, which is
the canonical decimal of DEL-02-01 Design ``model-document-0.9.0.md`` section
1.5.1 (closest shortest round-trip decimal, exact ties to even). Rows: every
in-domain power of two (the B = 2^52 lower end), tie binaries (16- or 17-digit
shortest decimals whose exact value has one more digit, a final 5), and random
values, all inside the domain |v| <= 1e6 with a canonical exponent >= -24.
The seed is fixed, so ``--check`` reports whether the committed file is current.
Read by ``core/product_physics/src/case_state/temperature/canonical_decimal.rs``.
"""
import random, struct, sys
from decimal import Decimal
from pathlib import Path

OUT = Path(__file__).resolve().parents[2] / "core/product_physics/tests/fixtures/temperature_identity/canonical_decimal_repr_corpus.txt"
TIES, RANDOM = 3000, 1500


def exponent(x):
    """e of the canonical decimal m*10^e, m not divisible by 10."""
    sign, digits, exp = Decimal(repr(x)).as_tuple()
    digits = list(digits)
    while len(digits) > 1 and digits[-1] == 0:
        digits.pop()
        exp += 1
    return exp


def in_domain(x):
    return x != 0 and abs(x) <= 1e6 and exponent(x) >= -24


def is_tie(x):
    shortest = Decimal(repr(x)).normalize().as_tuple().digits
    exact = Decimal(x).normalize().as_tuple().digits
    return len(shortest) >= 16 and len(exact) == len(shortest) + 1 and exact[-1] == 5


def bits(x):
    return struct.pack(">d", x).hex()


def rows():
    rng = random.Random(20261028)
    out = {}
    for k in range(-80, 21):
        x = 2.0 ** k
        if in_domain(x):
            out[bits(x)] = repr(x)
    ties = 0
    while ties < TIES:
        # Few significant bits keep the exact decimal short enough to tie.
        width = rng.randint(1, 30)
        mantissa = (1 << 52) | (rng.getrandbits(width) << (52 - width))
        x = rng.choice((1, -1)) * mantissa * 2.0 ** rng.randint(-85, -32)
        if in_domain(x) and is_tie(x) and bits(x) not in out:
            out[bits(x)] = repr(x)
            ties += 1
    count = 0
    while count < RANDOM:
        x = rng.choice((1, -1)) * 10 ** rng.uniform(-10, 6)
        if in_domain(x) and bits(x) not in out:
            out[bits(x)] = repr(x)
            count += 1
    return out


def text():
    header = "# bits repr; written by tools/fixtures/temperature_canonical_decimal_corpus.py\n"
    return header + "".join(f"{b} {r}\n" for b, r in rows().items())


if __name__ == "__main__":
    body = text()
    if "--check" in sys.argv:
        current = OUT.read_text() if OUT.exists() else ""
        sys.exit(0 if current == body else "corpus is not current")
    OUT.write_text(body)
