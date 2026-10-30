"""Independent verifier D1–D5 and complete canonical K4LED.

This module imports neither the kernel's vector generator nor producer arithmetic.
Fractions represent exact binary64 inputs and exact sums. All rounding decisions
use integer quotient/remainder and midpoint comparisons; no floating square root
or decimal approximation is used.
"""
from fractions import Fraction
from math import isqrt
import hashlib
import struct

COMPONENTS = ("UX", "UY", "UZ", "RX", "RY", "RZ")


def _word(word):
    if not isinstance(word, str) or len(word) != 16:
        raise ValueError("binary64 word")
    n = int(word, 16)
    exponent = (n >> 52) & 2047
    if exponent == 2047:
        raise ValueError("nonfinite binary64")
    mantissa = (n & ((1 << 52) - 1)) | ((1 << 52) if exponent else 0)
    exponent = exponent - 1075 if exponent else -1074
    value = Fraction(mantissa << exponent) if exponent >= 0 else Fraction(mantissa, 1 << -exponent)
    return -value if n >> 63 else value


def _pow2(exponent):
    return Fraction(1 << exponent) if exponent >= 0 else Fraction(1, 1 << -exponent)


def _floor_log2(value):
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    return exponent - (value < _pow2(exponent))


def _rn(value):
    if not value:
        return Fraction(0)
    negative = value < 0
    value = abs(value)
    exponent = _floor_log2(value) - 1023
    scaled = value / _pow2(exponent)
    q, r = divmod(scaled.numerator, scaled.denominator)
    twice = 2 * r
    if twice > scaled.denominator or (twice == scaled.denominator and q & 1):
        q += 1
    result = q * _pow2(exponent)
    return -result if negative else result


def _sqrt_rn(value):
    if value <= 0:
        raise ValueError("zero/nonpositive chord")
    exponent = _floor_log2(value) // 2 - 1023
    scaled = value / _pow2(2 * exponent)
    q = isqrt(scaled.numerator // scaled.denominator)
    comparison = 4 * scaled.numerator - (2*q + 1)**2 * scaled.denominator
    if comparison > 0 or (comparison == 0 and q & 1):
        q += 1
    return q * _pow2(exponent)


def declared_entries(xi, xj, intensity):
    """Return six exact declared entries, given three binary64 word arrays."""
    if any(len(x) != 3 for x in (xi, xj, intensity)):
        raise ValueError("coordinate/intensity arity")
    x, y, w = [tuple(map(_word, a)) for a in (xi, xj, intensity)]
    squared = _rn(sum((y[a]*y[a] - 2*x[a]*y[a] + x[a]*x[a] for a in range(3)), Fraction()))
    length = _sqrt_rn(squared)
    g = _rn(length / 12)
    force = [_rn(length * v) / 2 if v else Fraction() for v in w]
    moments = []
    for a in range(3):
        b, c = (a+1) % 3, (a+2) % 3
        # The four products share no rounded chord/cross-product operand.
        moments.append(_rn(g*y[b]*w[c] - g*x[b]*w[c] - g*y[c]*w[b] + g*x[c]*w[b]))
    return force + moments


def _net_parts(value):
    if not value:
        return False, 0, ()
    negative = value < 0
    value = abs(value)
    numerator, denominator = value.numerator, value.denominator
    if denominator & (denominator - 1):
        raise ValueError("non-dyadic ledger")
    low = (numerator & -numerator).bit_length() - 1
    numerator >>= low
    exponent = low - (denominator.bit_length() - 1)
    if numerator.bit_length() > 8192:
        raise ValueError("ledger span")
    words = tuple((numerator >> i) & ((1 << 64) - 1) for i in range(0, numerator.bit_length(), 64))
    return negative, exponent, words


def recompute(source):
    """Return (complete K4LED SHA-256, individual-nonzero global DOF set)."""
    nodes, members = source["id_maps"]["nodes"], source["id_maps"]["members"]
    loads, nodal = source["member_load_terms"], source["nodal_terms"]
    if not (0 < len(loads) <= 128 and len(nodal) <= 128 and len(nodes) <= 32 and len(members) <= 32):
        raise ValueError("member reader domain")
    nets, nonzero = {}, set()
    def add(dof, value):
        nets[dof] = nets.get(dof, Fraction()) + value
        if value:
            nonzero.add(dof)
    for term in nodal:
        n = int(term["dof"]["node"])
        if not 0 <= n < len(nodes):
            raise ValueError("nodal node")
        add(6*n + COMPONENTS.index(term["dof"]["component"]), _word(term["value"]))
    for term in loads:
        mi = int(term["member"])
        if not 0 <= mi < len(members):
            raise ValueError("member index")
        member = members[mi]
        ni, nj = int(member["node_i"]), int(member["node_j"])
        if not (0 <= ni < len(nodes) and 0 <= nj < len(nodes)):
            raise ValueError("member nodes")
        entries = declared_entries(nodes[ni]["coordinates"], nodes[nj]["coordinates"], term["intensity"])
        for end, n in enumerate((ni, nj)):
            for component, v in enumerate(entries):
                add(6*n + component, -v if end and component >= 3 else v)
    digest = hashlib.sha256(b"K4LED\x01" + struct.pack("<I", len(nets)))
    for dof, value in sorted(nets.items()):
        negative, exponent, words = _net_parts(value)
        digest.update(struct.pack("<IBBqI", dof // 6, dof % 6, negative, exponent, len(words)))
        for word in words:
            digest.update(struct.pack("<Q", word))
    return digest.hexdigest(), nonzero
