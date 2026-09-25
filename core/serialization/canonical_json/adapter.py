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


# Explicit scientific successor. Historical helpers above deliberately retain
# their acceptance and executable; there is no retry between these profiles.
BINARY64_PROFILE = "openpipestress_jcs_binary64_v1"
ENV_BINARY64_EXECUTABLE = "OPENPIPESTRESS_BINARY64_JSON_BIN"
_KNOWN_BINARY64_EXECUTABLE = _KNOWN_EXECUTABLE.with_name("openpipestress_jcs_binary64")
_BINARY64_MAX_BYTES = 8 * 1024 * 1024
_BINARY64_MAX_DEPTH = 128
_BINARY64_MAX_NODES = 262_144
_BINARY64_MAX_ITEMS = 4096
_BINARY64_MAX_REQUEST_BYTES = 64 * 1024 * 1024
_SAFE_INTEGER = 9_007_199_254_740_991


def binary64_executable() -> Path:
    configured = os.environ.get(ENV_BINARY64_EXECUTABLE)
    path = Path(configured).expanduser().resolve() if configured else _KNOWN_BINARY64_EXECUTABLE
    if not path.is_file():
        raise RuntimeError(f"BINARY64-JSON-AUTHORITY-MISSING: {path}; build explicitly with tools/serialization/build_checked_json.py --profile {BINARY64_PROFILE}")
    return path


def _binary64_string(value: str) -> str:
    if type(value) is not str:
        raise ValueError("BINARY64-JSON-STRING-REQUIRED")
    for character in value:
        point = ord(character)
        if 0xD800 <= point <= 0xDFFF:
            raise ValueError("BINARY64-JSON-LONE-SURROGATE")
        if 0xFDD0 <= point <= 0xFDEF or point & 0xFFFF in (0xFFFE, 0xFFFF):
            raise ValueError("BINARY64-JSON-NONCHARACTER")
    return value


def exact_integer(value: Any, minimum: int = -_SAFE_INTEGER, maximum: int = _SAFE_INTEGER) -> int:
    """Schema-owned programmatic counter/index admission; bool is not an int.

    This cannot authenticate discarded decimal spelling. Typed text adapters
    must instead use Rust's original-token exact_integer_at before projection.
    """
    if type(minimum) is not int or type(maximum) is not int or minimum < -_SAFE_INTEGER or maximum > _SAFE_INTEGER or minimum > maximum:
        raise ValueError("BINARY64-JSON-EXACT-INTEGER-BOUNDS")
    if type(value) is not int or not minimum <= value <= maximum:
        raise ValueError("BINARY64-JSON-EXACT-INTEGER-REQUIRED")
    return value


def scientific_real(value: int | float) -> float:
    """Explicitly declare a host integer/float as a scientific binary64 real.

    Integer-to-float conversion can discard digits. This is not exact decimal
    storage; use strings for arbitrary-precision operands or identifiers.
    """
    if type(value) not in (int, float):
        raise ValueError("BINARY64-JSON-SCIENTIFIC-REAL-REQUIRED")
    try:
        result = float(value)
    except OverflowError as exc:
        raise ValueError("BINARY64-JSON-OVERFLOW") from exc
    if not math.isfinite(result):
        raise ValueError("BINARY64-JSON-NON-FINITE")
    if result == 0.0 and math.copysign(1.0, result) < 0:
        raise ValueError("BINARY64-JSON-NEGATIVE-ZERO")
    return result


def _bounded_string_size(value: str, maximum: int, *, escaped: bool) -> int:
    # Count bytes without allocating an encoded/escaped copy. Aliased large
    # strings must pay their full expansion each time they occur in a snapshot.
    size = 2 if escaped else 0
    if len(value) + size > maximum:
        raise ValueError("BINARY64-JSON-BYTE-LIMIT")
    for character in value:
        point = ord(character)
        if 0xD800 <= point <= 0xDFFF:
            raise ValueError("BINARY64-JSON-UTF8")
        if escaped and character in ('"', "\\", "\b", "\t", "\n", "\f", "\r"):
            size += 2
        elif escaped and point < 0x20:
            size += 6
        else:
            size += 1 if point < 0x80 else 2 if point < 0x800 else 3 if point < 0x10000 else 4
        if size > maximum:
            raise ValueError("BINARY64-JSON-BYTE-LIMIT")
    return size


def _snapshot_bytes(state: list[int], amount: int) -> None:
    state[1] += amount
    if state[1] > _BINARY64_MAX_BYTES:
        raise ValueError("BINARY64-JSON-BYTE-LIMIT")


def _freeze_binary64(value: Any, active: set[int], depth: int, state: list[int]) -> Any:
    if depth > _BINARY64_MAX_DEPTH:
        raise ValueError("BINARY64-JSON-DEPTH-LIMIT")
    state[0] += 1
    if state[0] > _BINARY64_MAX_NODES:
        raise ValueError("BINARY64-JSON-NODE-LIMIT")
    if value is None or type(value) is bool:
        _snapshot_bytes(state, 4 if value is None or value else 5)
        return value
    if type(value) is str:
        _snapshot_bytes(state, _bounded_string_size(value, _BINARY64_MAX_BYTES - state[1], escaped=True))
        return _binary64_string(value)
    if type(value) is int:
        integer = exact_integer(value)
        _snapshot_bytes(state, len(str(integer)))
        return integer
    if type(value) is float:
        real = scientific_real(value)
        _snapshot_bytes(state, len(json.dumps(real, allow_nan=False)))
        return real
    if type(value) not in (dict, list):
        raise ValueError(f"BINARY64-JSON-UNSUPPORTED-TYPE: {type(value).__name__}")
    identity = id(value)
    if identity in active:
        raise ValueError("BINARY64-JSON-CYCLIC-VALUE")
    # Punctuation is known before constructing a frozen container.
    _snapshot_bytes(state, 2 + max(0, len(value) - 1))
    active.add(identity)
    try:
        if type(value) is dict:
            frozen = {}
            for key, item in value.items():
                _binary64_string(key)
                _snapshot_bytes(state, _bounded_string_size(key, _BINARY64_MAX_BYTES - state[1], escaped=True) + 1)
                frozen[key] = _freeze_binary64(item, active, depth + 1, state)
            return frozen
        return [_freeze_binary64(item, active, depth + 1, state) for item in value]
    finally:
        active.remove(identity)


def _response_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate response key")
        result[key] = value
    return result


def canonicalize_binary64_text_batch(items: Sequence[tuple[str, str]]) -> dict[str, str]:
    """Canonicalize original text through Rust without Python parsing it first.

    Every generic numeric token declares a binary64 real. This authenticates
    semantic canonical bytes, not original spelling or a typed record schema.
    """
    request_items = []
    seen: set[str] = set()
    request_header = {"protocol_version": PROTOCOL_VERSION, "profile": BINARY64_PROFILE, "items": []}
    request_size = len(json.dumps(request_header, separators=(",", ":")))
    if request_size > _BINARY64_MAX_REQUEST_BYTES:
        raise ValueError("BINARY64-JSON-REQUEST-BYTE-LIMIT")
    for item_id, json_text in items:
        _binary64_string(item_id)
        if not item_id or item_id in seen:
            raise ValueError("BINARY64-JSON-ITEM-ID-INVALID")
        if type(json_text) is not str:
            raise ValueError("BINARY64-JSON-TEXT-REQUIRED")
        # Per-document UTF-8 and aggregate escaped-envelope accounting happen
        # before materialization, including aliases and batches of many items.
        _bounded_string_size(json_text, _BINARY64_MAX_BYTES, escaped=False)
        request_size += len('{"id":,"json_text":}') + (1 if request_items else 0)
        request_size += _bounded_string_size(item_id, _BINARY64_MAX_REQUEST_BYTES - request_size, escaped=True)
        request_size += _bounded_string_size(json_text, _BINARY64_MAX_REQUEST_BYTES - request_size, escaped=True)
        seen.add(item_id)
        request_items.append({"id": item_id, "json_text": json_text})
        if len(request_items) > _BINARY64_MAX_ITEMS:
            raise ValueError("BINARY64-JSON-BATCH-LIMIT")
    request_header["items"] = request_items
    request = json.dumps(request_header, ensure_ascii=False, separators=(",", ":"), allow_nan=False)
    completed = subprocess.run([str(binary64_executable())], input=request, text=True, encoding="utf-8", capture_output=True, check=False, shell=False)
    if completed.returncode != 0:
        raise RuntimeError(f"BINARY64-JSON-AUTHORITY-REJECTED: {completed.stderr.strip()}")
    try:
        response = json.loads(completed.stdout, object_pairs_hook=_response_pairs)
        if type(response) is not dict or set(response) != {"protocol_version", "profile", "items"}:
            raise ValueError("response shape")
        if response["protocol_version"] != PROTOCOL_VERSION or response["profile"] != BINARY64_PROFILE:
            raise ValueError("profile or protocol mismatch")
        returned = response["items"]
        if type(returned) is not list or len(returned) != len(request_items):
            raise ValueError("response items")
        for row, submitted in zip(returned, request_items):
            if type(row) is not dict or set(row) != {"id", "canonical_json"} or row["id"] != submitted["id"] or type(row["canonical_json"]) is not str:
                raise ValueError("response row")
            _binary64_string(row["canonical_json"])
    except (ValueError, TypeError, KeyError) as exc:
        raise RuntimeError("BINARY64-JSON-AUTHORITY-RESPONSE-INVALID") from exc
    return {row["id"]: row["canonical_json"] for row in returned}


def canonicalize_binary64_batch(items: Sequence[tuple[str, Any]]) -> dict[str, str]:
    """Freeze actual plain Python snapshots before transport; no raw custody."""
    def texts():
        for item_id, value in items:
            frozen = _freeze_binary64(value, set(), 0, [0, 0])
            yield item_id, json.dumps(frozen, ensure_ascii=False, separators=(",", ":"), allow_nan=False)
    # Stream each bounded snapshot into aggregate admission instead of retaining
    # all serialized documents before checking the request's total expansion.
    return canonicalize_binary64_text_batch(texts())


def canonical_json_binary64_v1_text(json_text: str) -> str:
    return canonicalize_binary64_text_batch([("value", json_text)])["value"]


def canonical_json_binary64_v1(value: Any) -> str:
    return canonicalize_binary64_batch([("value", value)])["value"]


def canonical_sha256_binary64_v1(value: Any) -> str:
    return sha256(canonical_json_binary64_v1(value).encode("utf-8")).hexdigest()


def canonical_sha256_binary64_v1_text(json_text: str) -> str:
    return sha256(canonical_json_binary64_v1_text(json_text).encode("utf-8")).hexdigest()
