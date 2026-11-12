"""Independent verifier of the headless runner's run evidence (DEL-10-05 REQ-017, AC-013).

A result the runner emits for a verb or a hosted command that takes a request carries
`run_evidence` beside the local-private control's members. This verifier recomputes it from
the kept bytes alone: the request bytes the runner read and the emitted result.

- `request.raw_sha256`: SHA-256 of the bytes as read.
- `request.canonical_sha256`: SHA-256 of the request's `openpipestress_jcs_ijson_v1` text.
- `model`: for `solve` the document at `/solve/preview_model/model`, for `size-hangers` the one
  at `/request/model`, as `{pointer, canonical_sha256}`; `{absent: <reason>}` when the command
  carries no model document or this request has none.
- `result.canonical_sha256`: SHA-256 of the emitted output less `run_evidence`, in the profile.
- `binding_sha256`: SHA-256 of the profile text of `{"domain": "openpipestress_runner_evidence_v1",
  "payload": <the evidence less binding_sha256>}`.

A value the profile refuses is stated `{"unavailable": <code>}`; the verifier works out the code the profile
gives (its closed list, its parse depth and its number rule) and requires the same code. An unavailable digest
binds nothing for what it would cover and is reported, never verified; an unavailable request digest leaves
the raw-bytes digest, which still binds the bytes as read. The
invocation and producer cannot be recomputed from bytes; a change to either fails the binding.

Where the local-private control would block the request itself (or cannot judge all of it), the
runner states `request` and `model` each as `{"withheld": <code>}` and keeps the result digest.
A withheld member cannot be compared: `verify` then reports it as `"request: withheld"` and
`"model: withheld"`, never as verified. This verifier does not judge privacy (it imports no
control); it takes a withheld statement only when both members carry the same known code, and
the binding covers the statement.

The evidence is not a signature: whoever can rewrite a result can rewrite its evidence, and
whoever removes it leaves a result that reads as one made before run evidence existed.

Test-only and independent: it imports only the standard library and `canonical_json`.
"""
from __future__ import annotations

from hashlib import sha256
import json
import math
from typing import Any

from . import canonical_json

MEMBER = "run_evidence"
EVIDENCE_VERSION = "1.0.0"
BINDING_DOMAIN = "openpipestress_runner_evidence_v1"
MODEL_POINTERS = {"solve": "/solve/preview_model/model", "size-hangers": "/request/model"}
ABSENT_NOT_CARRIED = "COMMAND_CARRIES_NO_MODEL_DOCUMENT"
ABSENT_NOT_PRESENT = "MODEL_DOCUMENT_NOT_IN_REQUEST"
CONTROL_MEMBERS = ("payload", "decisions", "findings", "blocked", "summary")
WITHHELD_CODES = ("REQUEST_BLOCKED_BY_LOCAL_PRIVATE_CONTROL", "REQUEST_NOT_JUDGED_BY_LOCAL_PRIVATE_CONTROL")
WITHHELD_REPORT = ["model: withheld", "request: withheld"]
# The checked profile's closed list of refusal codes (`core/serialization/canonical_json/src/lib.rs`,
# `canonical_json_checked_v1_text`: `checked_parse` lines 89-90, `validate_checked_value` lines 99-107).
PROFILE_REFUSAL_CODES = ("CHECKED-JSON-INVALID", "CHECKED-JSON-UNSAFE-INTEGER", "CHECKED-JSON-NUMBER-INVALID",
                         "CHECKED-JSON-NON-FINITE", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT")
# serde_json's parser depth, which the profile's parse inherits: a container nested at this level (the
# outermost counted 1) is refused as CHECKED-JSON-INVALID (`lib.rs` lines 217-219; the boundary is pinned by
# `core/serialization/canonical_json/tests/checked_domain.rs`, `recursion_limit_at_minus_one_at_and_past_the_boundary`).
PARSE_DEPTH_LIMIT = 128
MAX_SAFE_INTEGER = canonical_json.MAX_SAFE_INTEGER
I64_MIN, U64_MAX = -(2**63), 2**64 - 1
DIGESTED = ("request", "model", "result")


class Refused(ValueError):
    """The profile refuses the value; `code` is one of PROFILE_REFUSAL_CODES."""

    def __init__(self, code: str):
        super().__init__(code)
        self.code = code


def _unique_members(pairs: list[tuple[str, Any]]) -> dict:
    keys = [key for key, _ in pairs]
    if len(set(keys)) != len(keys):
        raise Refused("CHECKED-JSON-INVALID")
    return dict(pairs)


def _refuse_constant(name: str) -> Any:
    raise Refused("CHECKED-JSON-INVALID")


def _parse_refusal(value: Any) -> str | None:
    """What the profile's parse refuses in a value read from JSON text, before any number is validated: a
    container at the parse depth, a lone surrogate (in a string or a key), a number token that is not a
    finite binary64 (serde_json reads an integer outside i64/u64 as binary64)."""
    stack = [(value, 1)]
    while stack:
        item, level = stack.pop()
        if type(item) in (dict, list):
            if level >= PARSE_DEPTH_LIMIT:
                return "CHECKED-JSON-INVALID"
            children = list(item.values()) if type(item) is dict else item
            if type(item) is dict and any(0xD800 <= ord(c) <= 0xDFFF for key in item for c in key):
                return "CHECKED-JSON-INVALID"
            stack.extend((child, level + 1) for child in children)
        elif type(item) is str:
            if any(0xD800 <= ord(c) <= 0xDFFF for c in item):
                return "CHECKED-JSON-INVALID"
        elif type(item) is float:
            if not math.isfinite(item):
                return "CHECKED-JSON-INVALID"
        elif type(item) is int and not (I64_MIN <= item <= U64_MAX):
            try:
                if not math.isfinite(float(item)):
                    return "CHECKED-JSON-INVALID"
            except OverflowError:
                return "CHECKED-JSON-INVALID"
    return None


def _number_refusal(value: Any) -> str | None:
    """The profile's number validation, in its order (object members by key, arrays in order), first
    refusal wins: an integer inside i64/u64 beyond 2^53 - 1 is CHECKED-JSON-UNSAFE-INTEGER; any other
    number is a binary64, refused as CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT when integral with magnitude
    beyond 2^53 - 1 (an integer outside i64/u64 is read as one)."""
    if type(value) is dict:
        for key in sorted(value):
            refusal = _number_refusal(value[key])
            if refusal:
                return refusal
    elif type(value) is list:
        for item in value:
            refusal = _number_refusal(item)
            if refusal:
                return refusal
    elif type(value) is int:
        if I64_MIN <= value <= U64_MAX:
            return "CHECKED-JSON-UNSAFE-INTEGER" if abs(value) > MAX_SAFE_INTEGER else None
        return "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"
    elif type(value) is float:
        if value.is_integer() and abs(value) > MAX_SAFE_INTEGER:
            return "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"
    return None


def profile_refusal(value: Any) -> str | None:
    """The code the profile refuses a value read from JSON text with, or None when it carries it."""
    return _parse_refusal(value) or _number_refusal(value)


def checked_parse(raw: bytes) -> Any:
    """The request as the profile reads it; raises Refused with the profile's code where it refuses it."""
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        raise Refused("CHECKED-JSON-INVALID") from None
    try:
        value = json.loads(text, object_pairs_hook=_unique_members, parse_constant=_refuse_constant)
    except (json.JSONDecodeError, RecursionError, ValueError) as error:
        if isinstance(error, Refused):
            raise
        raise Refused("CHECKED-JSON-INVALID") from None
    refusal = profile_refusal(value)
    if refusal:
        raise Refused(refusal)
    return value


def _digest(value: Any) -> Any:
    refusal = profile_refusal(value)
    if refusal:
        return {"unavailable": refusal}
    return sha256(canonical_json.canonical_json(value).encode("utf-8")).hexdigest()


def _pointer(value: Any, pointer: str) -> tuple[bool, Any]:
    for token in pointer.split("/")[1:]:
        token = token.replace("~1", "/").replace("~0", "~")
        if type(value) is dict and token in value:
            value = value[token]
        elif type(value) is list and token.isdigit() and int(token) < len(value):
            value = value[int(token)]
        else:
            return False, None
    return True, value


def _request_and_model(raw: bytes, command: Any) -> tuple[dict, dict]:
    try:
        parsed, refusal = checked_parse(raw), None
        request_digest = _digest(parsed)
    except Refused as error:
        parsed, refusal = None, {"unavailable": error.code}
        request_digest = refusal
    request = {"raw_sha256": sha256(raw).hexdigest(), "canonical_sha256": request_digest}
    pointer = MODEL_POINTERS.get(command) if type(command) is str else None
    if pointer is None:
        model = {"absent": ABSENT_NOT_CARRIED}
    elif refusal is not None:
        model = {"pointer": pointer, "canonical_sha256": refusal}
    else:
        found, value = _pointer(parsed, pointer)
        model = {"pointer": pointer, "canonical_sha256": _digest(value)} if found else {"absent": ABSENT_NOT_PRESENT}
    return request, model


def binding(evidence: dict) -> Any:
    payload = {key: value for key, value in evidence.items() if key != "binding_sha256"}
    return _digest({"domain": BINDING_DOMAIN, "payload": payload})


def _is_unavailable(digest: Any) -> bool:
    return type(digest) is dict and set(digest) == {"unavailable"} and digest["unavailable"] in PROFILE_REFUSAL_CODES


def _same_digest(want: Any, have: Any) -> bool:
    """Equal digests, or the same refusal of the profile, by a code from its closed list."""
    if type(want) is dict:
        return _is_unavailable(want) and want == have
    return type(want) is str and want == have


def _same(want: Any, have: Any) -> bool:
    """Equal JSON values, where a member named `canonical_sha256` is compared as a digest."""
    if type(want) is dict and type(have) is dict:
        return set(want) == set(have) and all(
            _same_digest(want[key], have[key]) if key == "canonical_sha256" else _same(want[key], have[key]) for key in want)
    if type(want) is list and type(have) is list:
        return len(want) == len(have) and all(_same(a, b) for a, b in zip(want, have))
    return want == have and type(want) is type(have)


def _withheld_code(member: Any) -> str | None:
    if type(member) is dict and set(member) == {"withheld"} and member["withheld"] in WITHHELD_CODES:
        return member["withheld"]
    return None


def form(raw: bytes, result: dict, invocation: dict, producer: Any = None, withheld: str | None = None) -> dict:
    """The evidence `result` (without `run_evidence`) carries for the request bytes `raw`, as the runner
    forms it; for synthetic runners in tests. `producer` defaults to a synthetic identity; `withheld`
    states the request and model withheld under that code."""
    if withheld is None:
        request, model = _request_and_model(raw, invocation.get("command"))
    else:
        request, model = {"withheld": withheld}, {"withheld": withheld}
    evidence = {
        "evidence_version": EVIDENCE_VERSION,
        "canonical_profile": canonical_json.PROFILE,
        "invocation": invocation,
        "producer": producer if producer is not None else {"synthetic": "test-only runner"},
        "request": request,
        "model": model,
        "result": {"canonical_sha256": _digest(result)},
    }
    evidence["binding_sha256"] = binding(evidence)
    return evidence


def verify(raw: bytes, emitted: Any) -> list[str]:
    """`[]` only when every digest was recomputed from `raw` and the result and is equal. Otherwise, either
    every evidence member whose recomputed value differs, by dotted name (`["run_evidence"]` when the member
    is missing or not an object), or, when everything compared agrees, what was not compared:
    `"model: withheld"`, `"request: withheld"` and `"<member>.canonical_sha256: unavailable"` (request, model,
    result, in that order). Neither withheld nor unavailable is ever reported as verified."""
    if type(emitted) is not dict or type(emitted.get(MEMBER)) is not dict:
        return [MEMBER]
    stated = emitted[MEMBER]
    invocation = stated.get("invocation")
    command = invocation.get("command") if type(invocation) is dict else None
    code = _withheld_code(stated.get("request"))
    withheld = code is not None and _withheld_code(stated.get("model")) == code
    request, model = ({"withheld": code}, {"withheld": code}) if withheld else _request_and_model(raw, command)
    expected = {
        "evidence_version": EVIDENCE_VERSION,
        "canonical_profile": canonical_json.PROFILE,
        "invocation": invocation,
        "producer": stated.get("producer"),
        "request": request,
        "model": model,
        "result": {"canonical_sha256": _digest({key: value for key, value in emitted.items() if key != MEMBER})},
        "binding_sha256": binding(stated),
    }
    differing = []
    for key in sorted(set(expected) | set(stated)):
        want, have = expected.get(key), stated.get(key)
        if key in ("request", "result") and type(want) is dict and type(have) is dict and not withheld:
            differing += [f"{key}.{name}" for name in sorted(set(want) | set(have))
                          if not (_same_digest if name == "canonical_sha256" else _same)(want.get(name), have.get(name))]
        elif key == "binding_sha256":
            # The binding is always a digest: evidence the profile refuses binds nothing.
            if type(want) is not str or want != have:
                differing.append(key)
        elif not _same(want, have) or key not in stated:
            differing.append({"model": "model.canonical_sha256", "result": "result.canonical_sha256"}.get(key, key))
    if differing:
        return differing
    report = list(WITHHELD_REPORT) if withheld else []
    report += [f"{member}.canonical_sha256: unavailable" for member in DIGESTED
               if type(stated.get(member)) is dict and _is_unavailable(stated[member].get("canonical_sha256"))]
    return report


NOT_ONE_DOCUMENT = "result: not one JSON document"


def verify_bytes(raw: bytes, output: bytes) -> list[str]:
    """`verify` over a result as kept bytes (an output file or captured standard output): bytes that are not one
    whole JSON document, such as a partial file left by a run killed while it wrote, fail by NOT_ONE_DOCUMENT."""
    try:
        emitted = json.loads(output.decode("utf-8"), object_pairs_hook=_unique_members, parse_constant=_refuse_constant)
    except (UnicodeDecodeError, ValueError, RecursionError):
        return [NOT_ONE_DOCUMENT]
    return verify(raw, emitted)


BOUND = "bound"
NOT_BOUND_WITHHELD = "not bound: withheld"
NOT_BOUND_UNAVAILABLE = "not bound: unavailable"
NOT_BOUND_NO_RUN_EVIDENCE = "not bound: no run evidence"
RAW_NOTE = " (request.raw_sha256 binds the request bytes as read)"


def bound(raw: bytes, emitted: Any) -> str:
    """Whether `emitted` is bound to the request bytes `raw`, for a check that requires the binding, in the
    same words as the runner crate's `Binding::answer`. Only `"bound"` supplies one; withheld evidence, an
    unavailable digest and a result without run evidence fail closed, each by its own answer."""
    if type(emitted) is not dict or MEMBER not in emitted:
        return NOT_BOUND_NO_RUN_EVIDENCE
    report = verify(raw, emitted)
    if not report:
        return BOUND
    if not all(entry.endswith((": withheld", ": unavailable")) for entry in report):
        return "not bound: mismatch: " + ", ".join(report)
    unavailable = [entry[: -len(": unavailable")] for entry in report if entry.endswith(": unavailable")]
    if any(entry.endswith(": withheld") for entry in report):
        return NOT_BOUND_WITHHELD + ("; unavailable: " + ", ".join(unavailable) if unavailable else "")
    return (NOT_BOUND_UNAVAILABLE + ": " + ", ".join(unavailable)
            + (RAW_NOTE if "request.canonical_sha256" in unavailable else ""))
