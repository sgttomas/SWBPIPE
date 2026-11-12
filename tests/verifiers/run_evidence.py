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

A value the profile refuses is stated `{"unavailable": <code>}`; the verifier requires the
profile to refuse it here too and the code to be a `CHECKED-JSON-` refusal (this
implementation names refusals in its own words, so the code itself is not compared). The
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


def _unique_members(pairs: list[tuple[str, Any]]) -> dict:
    keys = [key for key, _ in pairs]
    if len(set(keys)) != len(keys):
        raise ValueError("CHECKED-JSON-INVALID: duplicate object member")
    return dict(pairs)


def _refuse_constant(name: str) -> Any:
    raise ValueError(f"CHECKED-JSON-INVALID: {name} is not JSON")


def checked_parse(raw: bytes) -> Any:
    """The request as the profile reads it: UTF-8 JSON with unique members; numbers are checked
    when the value is written (`canonical_json`)."""
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        raise ValueError("CHECKED-JSON-INVALID: not UTF-8") from None
    try:
        return json.loads(text, object_pairs_hook=_unique_members, parse_constant=_refuse_constant)
    except json.JSONDecodeError as error:
        raise ValueError(f"CHECKED-JSON-INVALID: {error.msg}") from None


def _digest(value: Any) -> Any:
    try:
        return sha256(canonical_json.canonical_json(value).encode("utf-8")).hexdigest()
    except ValueError as error:
        return {"unavailable": str(error).split(":")[0]}


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
        if type(request_digest) is dict:
            refusal = request_digest
    except ValueError as error:
        parsed, refusal = None, {"unavailable": str(error).split(":")[0]}
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


def _same_digest(want: Any, have: Any) -> bool:
    """Equal digests, or both a stated refusal of the profile (the codes are each implementation's own)."""
    if type(want) is dict and set(want) == {"unavailable"}:
        return (type(have) is dict and set(have) == {"unavailable"} and type(have["unavailable"]) is str
                and have["unavailable"].startswith("CHECKED-JSON-"))
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
    """Every evidence member whose recomputed value differs, by dotted name; `[]` only when every
    digest was recomputed and the result and its evidence belong to the request bytes `raw`.
    `["run_evidence"]` when the member is missing or not an object. When the request and model
    were withheld and everything else agrees: `["model: withheld", "request: withheld"]`."""
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
    if not differing and withheld:
        return list(WITHHELD_REPORT)
    return differing


BOUND = "bound"
NOT_BOUND_WITHHELD = "not bound: withheld"
NOT_BOUND_NO_RUN_EVIDENCE = "not bound: no run evidence"


def bound(raw: bytes, emitted: Any) -> str:
    """Whether `emitted` is bound to the request bytes `raw`, for a check that requires the binding. Only
    `"bound"` supplies one; withheld evidence and a result without run evidence fail closed."""
    if type(emitted) is not dict or MEMBER not in emitted:
        return NOT_BOUND_NO_RUN_EVIDENCE
    report = verify(raw, emitted)
    if not report:
        return BOUND
    if report == WITHHELD_REPORT:
        return NOT_BOUND_WITHHELD
    return "not bound: mismatch: " + ", ".join(report)
