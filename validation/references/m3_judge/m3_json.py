"""Strict JSON reading for the M3 judge: the only module that parses JSON input.

A duplicate object key (the last member would otherwise win silently), NaN, Infinity or -Infinity is refused
(refutation round 4). The comparison (m3_compare.py) never imports this module; it reads only the canonical
view (m3_types.py) that the validator (m3_view.py) builds from what this module returns.
"""

from __future__ import annotations

import json
import os


class StrictJSONError(ValueError):
    pass


def _no_duplicates(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise StrictJSONError(f"duplicate key {k!r}")
        out[k] = v
    return out


def _no_constant(name):
    raise StrictJSONError(f"non-JSON constant {name}")


def strict_loads(text):
    """json.loads refusing duplicate object keys and NaN/Infinity."""
    return json.loads(text, object_pairs_hook=_no_duplicates, parse_constant=_no_constant)


def read_jsonl(path):
    """(values, parse problems) of a JSON-lines file; a blank line, a non-UTF-8 line or a missing file is a problem."""
    values, problems = [], []
    if not os.path.isfile(path):
        return values, [f"{path}: missing"]
    with open(path, "rb") as f:
        data = f.read()
    parts = data.split(b"\n")
    for i, line in enumerate(parts):
        if not line and i == len(parts) - 1:
            continue
        try:
            values.append(strict_loads(line.decode("utf-8")))
        except (ValueError, UnicodeDecodeError) as e:
            problems.append(f"{path}:{i + 1}: not a strict JSON value ({e})")
    return values, problems


def read_json(path):
    """(value, problems) of a whole-file JSON document."""
    if not os.path.isfile(path):
        return None, [f"{path}: missing"]
    try:
        with open(path, "rb") as f:
            return strict_loads(f.read().decode("utf-8")), []
    except (ValueError, UnicodeDecodeError) as e:
        return None, [f"{path}: not a strict JSON value ({e})"]
