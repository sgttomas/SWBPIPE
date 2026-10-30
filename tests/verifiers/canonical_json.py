"""The checked canonical JSON profile the retained verifier hashes with, in Python.

`openpipestress_jcs_ijson_v1`: RFC 8785 (JCS) text with the I-JSON admission of
`core/serialization/canonical_json` (`canonical_json_checked_v1_text`) and of its
Python bridge's value checks. Object keys sort by UTF-16 code units; strings
escape as `JSON.stringify`; numbers render as ECMAScript `Number::toString`
(shortest round-trip digits, `-0` as `0`); integers and integral numbers beyond
2^53 - 1, non-finite numbers, lone surrogates, non-string keys, cycles and
non-JSON types are refused (ValueError).

Test-only and independent: it calls no crate the producer links.
`tests/test_verifier_independence.py` checks it against that crate's committed
vectors (`fixtures/canonical_hash/cases.json`,
`fixtures/canonical_hash/binary64_transport_vectors.json`).
"""
from __future__ import annotations

from hashlib import sha256
import json
import math
from typing import Any

PROFILE = "openpipestress_jcs_ijson_v1"
MAX_SAFE_INTEGER = 9_007_199_254_740_991


def _string(text: str) -> str:
    if any(0xD800 <= ord(character) <= 0xDFFF for character in text):
        raise ValueError("CHECKED-JSON-LONE-SURROGATE")
    # Python's escapes are JSON.stringify's for well-formed text: \" \\ \b \f \n \r \t,
    # other controls as lowercase \u00xx, everything else literal.
    return json.dumps(text, ensure_ascii=False)


def ecma_number(value: float) -> str:
    """ECMAScript Number::toString (radix 10) of a finite double."""
    if not math.isfinite(value):
        raise ValueError("CHECKED-JSON-NON-FINITE")
    if value == 0:
        return "0"
    # repr gives the shortest round-trip digits (closest candidate, ties to even).
    mantissa, _, exponent = repr(abs(value)).partition("e")
    whole, _, fraction = mantissa.partition(".")
    raw = (whole + fraction).lstrip("0")
    digits = raw.rstrip("0")
    k = len(digits)
    n = k + (int(exponent) if exponent else 0) - len(fraction) + (len(raw) - k)
    sign = "-" if value < 0 else ""
    if k <= n <= 21:
        return sign + digits + "0" * (n - k)
    if 0 < n <= 21:
        return sign + digits[:n] + "." + digits[n:]
    if -6 < n <= 0:
        return sign + "0." + "0" * -n + digits
    return sign + digits[0] + ("." + digits[1:] if k > 1 else "") + "e" + ("+" if n - 1 >= 0 else "-") + str(abs(n - 1))


def _write(value: Any, out: list[str], active: set[int]) -> None:
    if value is None:
        out.append("null")
    elif value is True:
        out.append("true")
    elif value is False:
        out.append("false")
    elif isinstance(value, str):
        out.append(_string(value))
    elif type(value) is int:
        if abs(value) > MAX_SAFE_INTEGER:
            raise ValueError("CHECKED-JSON-UNSAFE-INTEGER")
        out.append(str(value))
    elif type(value) is float:
        if not math.isfinite(value) or (value.is_integer() and abs(value) > MAX_SAFE_INTEGER):
            raise ValueError("CHECKED-JSON-NUMBER-OUTSIDE-PROFILE")
        out.append(ecma_number(value))
    elif type(value) in (dict, list):
        if id(value) in active:
            raise ValueError("CHECKED-JSON-CYCLIC-VALUE")
        active.add(id(value))
        try:
            if type(value) is list:
                out.append("[")
                for index, item in enumerate(value):
                    if index:
                        out.append(",")
                    _write(item, out, active)
                out.append("]")
            else:
                if any(not isinstance(key, str) for key in value):
                    raise ValueError("CHECKED-JSON-NON-STRING-KEY")
                out.append("{")
                for index, key in enumerate(sorted(value, key=lambda key: key.encode("utf-16-be", "surrogatepass"))):
                    if index:
                        out.append(",")
                    out.append(_string(key))
                    out.append(":")
                    _write(value[key], out, active)
                out.append("}")
        finally:
            active.discard(id(value))
    else:
        raise ValueError(f"CHECKED-JSON-UNSUPPORTED-TYPE: {type(value).__name__}")


def canonical_json(value: Any) -> str:
    out: list[str] = []
    _write(value, out, set())
    return "".join(out)


def canonical_sha256(value: Any) -> str:
    return sha256(canonical_json(value).encode("utf-8")).hexdigest()


# --- The unchecked text (W-6a's serializer) ----------------------------------
#
# The desktop's model hash and the handoff package members are the *unchecked*
# RFC 8785 text of `core/serialization/canonical_json` (`canonical_json`, through
# `open_pipe_stress_operation_applier`): a document as serde_json reads it. An
# integer literal inside i64/u64 keeps its exact decimal text; any other number
# (a fraction, an exponent, an integer outside i64/u64) is a binary64 rendered as
# ECMAScript `Number::toString`. Nothing inside the JSON grammar is refused
# except what serde_json refuses (non-finite results, lone surrogates).
# `fixtures/handoff/canonical_text_vectors.json` pins it across the languages.

I64_MIN = -(2**63)
U64_MAX = 2**64 - 1


def _unchecked_number(value: int | float) -> str:
    if type(value) is int:
        if I64_MIN <= value <= U64_MAX:
            return str(value)
        value = float(value)  # serde_json reads it as the nearest binary64
    if not math.isfinite(value):
        raise ValueError("CANONICAL-JSON-NON-FINITE")
    return ecma_number(value)


def _write_unchecked(value: Any, out: list[str]) -> None:
    if value is None:
        out.append("null")
    elif value is True:
        out.append("true")
    elif value is False:
        out.append("false")
    elif isinstance(value, str):
        out.append(_string(value))
    elif type(value) in (int, float):
        out.append(_unchecked_number(value))
    elif type(value) is list:
        out.append("[")
        for index, item in enumerate(value):
            if index:
                out.append(",")
            _write_unchecked(item, out)
        out.append("]")
    elif type(value) is dict:
        out.append("{")
        for index, key in enumerate(sorted(value, key=lambda key: key.encode("utf-16-be", "surrogatepass"))):
            if index:
                out.append(",")
            out.append(_string(key))
            out.append(":")
            _write_unchecked(value[key], out)
        out.append("}")
    else:
        raise ValueError(f"CANONICAL-JSON-UNSUPPORTED-TYPE: {type(value).__name__}")


def _refuse_constant(name: str) -> Any:
    raise ValueError(f"CANONICAL-JSON-NOT-JSON: {name}")


def parse_json(text: str) -> Any:
    """JSON text as serde_json reads it: NaN and Infinity are not JSON."""
    return json.loads(text, parse_constant=_refuse_constant)


def canonical_json_unchecked(value: Any) -> str:
    """The unchecked RFC 8785 text of a parsed JSON value (see above)."""
    out: list[str] = []
    _write_unchecked(value, out)
    return "".join(out)
