"""Explicit subprocess adapter to the checked Rust JSON authority.

The runtime never builds the executable, searches PATH, invokes a shell, or
falls back to a Python serializer. Development setup is owned by
``tools/serialization/build_checked_json.py``.
"""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from hashlib import sha256
import json
import math
import os
from pathlib import Path
import subprocess
from typing import Any

PROFILE = "openpipestress_jcs_ijson_v1"
PROTOCOL_VERSION = "1.0.0"
ENV_EXECUTABLE = "OPENPIPESTRESS_CHECKED_JSON_BIN"
_KNOWN_EXECUTABLE = Path(__file__).resolve().parent / "target" / "checked-json" / "release" / "openpipestress_jcs_ijson"


def checked_executable() -> Path:
    configured = os.environ.get(ENV_EXECUTABLE)
    path = Path(configured).expanduser().resolve() if configured else _KNOWN_EXECUTABLE
    if not path.is_file():
        raise RuntimeError(f"CHECKED-JSON-AUTHORITY-MISSING: {path}; build explicitly with tools/serialization/build_checked_json.py")
    return path


def _freeze_python(value: Any, active: set[int]) -> Any:
    if value is None or isinstance(value, bool):
        return value
    if isinstance(value, str):
        if any(0xD800 <= ord(character) <= 0xDFFF for character in value):
            raise ValueError("CHECKED-JSON-LONE-SURROGATE")
        return value
    if type(value) is int:
        if abs(value) > 9_007_199_254_740_991:
            raise ValueError("CHECKED-JSON-UNSAFE-INTEGER")
        return value
    if type(value) is float:
        if not math.isfinite(value) or (value.is_integer() and abs(value) > 9_007_199_254_740_991):
            raise ValueError("CHECKED-JSON-NUMBER-OUTSIDE-PROFILE")
        return value
    if type(value) not in (dict, list):
        raise ValueError(f"CHECKED-JSON-UNSUPPORTED-TYPE: {type(value).__name__}")
    identity = id(value)
    if identity in active:
        raise ValueError("CHECKED-JSON-CYCLIC-VALUE")
    active.add(identity)
    try:
        if type(value) is dict:
            if any(not isinstance(key, str) for key in value):
                raise ValueError("CHECKED-JSON-NON-STRING-KEY")
            return {_freeze_python(key, active): _freeze_python(item, active) for key, item in list(value.items())}
        return [_freeze_python(item, active) for item in list(value)]
    finally:
        active.remove(identity)


def canonicalize_batch(items: Sequence[tuple[str, Any]]) -> dict[str, str]:
    request_items = []
    seen: set[str] = set()
    for item_id, value in items:
        if (
            type(item_id) is not str
            or not item_id
            or any(0xD800 <= ord(character) <= 0xDFFF for character in item_id)
            or item_id in seen
        ):
            raise ValueError("CHECKED-JSON-ITEM-ID-INVALID")
        seen.add(item_id)
        frozen = _freeze_python(value, set())
        request_items.append({"id": item_id, "json_text": json.dumps(frozen, ensure_ascii=False, separators=(",", ":"), allow_nan=False)})
    request = {"protocol_version": PROTOCOL_VERSION, "profile": PROFILE, "items": request_items}
    completed = subprocess.run(
        [str(checked_executable())], input=json.dumps(request, ensure_ascii=False, separators=(",", ":")),
        text=True, encoding="utf-8", capture_output=True, check=False, shell=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"CHECKED-JSON-AUTHORITY-REJECTED: {completed.stderr.strip()}")
    response = json.loads(completed.stdout)
    if response.get("protocol_version") != PROTOCOL_VERSION or response.get("profile") != PROFILE:
        raise RuntimeError("CHECKED-JSON-AUTHORITY-PROTOCOL-MISMATCH")
    returned = response.get("items")
    if not isinstance(returned, list) or [row.get("id") for row in returned] != [row["id"] for row in request_items]:
        raise RuntimeError("CHECKED-JSON-AUTHORITY-RESPONSE-INVALID")
    return {row["id"]: row["canonical_json"] for row in returned}


def canonical_json_checked_v1(value: Any) -> str:
    return canonicalize_batch([("value", value)])["value"]


def canonical_sha256_checked_v1(value: Any) -> str:
    return sha256(canonical_json_checked_v1(value).encode("utf-8")).hexdigest()
