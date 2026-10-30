"""The Rust product reader's verdict for Python product code.

Rust is the only product reader of result statements (b7-plan R1, R2). Python
product code that meets one takes the Rust reader's verdict through
`openpipestress-runner read <reader>` and never re-derives it: JSON in on stdin;
out on stdout, the reader's own outcome (the shape the desktop's wasm engine
returns too) inside the envelope `{"artifact", "schema_version", "reader",
"outcome"}`, with exit 0 when `outcome.ok` is true and 1 when it is false.

- `retained-precision` and `retained-precision-transport` (R1):
  `retained_precision::outcome_json`, `{"ok": true, "validation": {...}}` or
  `{"ok": false, "error": {gate, code, detail}}`.
- The base readers (R2, `reader_verdict`): `dispatch` (route, table binding,
  header refusal and standing), each route's reader and `<reader>-transport`,
  `row-semantics`; and `semantic_contract`'s `standing`,
  `classification-summary` and `rule-binding`. A refusal is
  `{"ok": false, "error": {code, detail}}`. B7b: the current routes are
  pressure-source-1, preview-physics-2, pressure-2 and the two successors; a
  stored v2 result is refused by the header (`SOURCE_EXACT_PRESSURE_V2_RETIRED`).

The runner is an explicitly built executable named by OPENPIPESTRESS_RUNNER_BIN,
an absolute path. There is no runtime build, PATH search, shell or fallback.
Any response outside these shapes fails closed (RuntimeError).
Development and test setup: `tools/runner/build_runner.py`.
"""
from __future__ import annotations

from collections.abc import Mapping
import json
import os
from pathlib import Path
import subprocess
from typing import Any

ENV_EXECUTABLE = "OPENPIPESTRESS_RUNNER_BIN"
ARTIFACT = "openpipestress.result_reader_verdict"
SCHEMA_VERSION = "1.0.0"
RETAINED_PRECISION = "retained-precision"
RETAINED_PRECISION_TRANSPORT = "retained-precision-transport"
DISPATCH = "dispatch"
STANDING = "standing"
CLASSIFICATION_SUMMARY = "classification-summary"
RULE_BINDING = "rule-binding"
ROW_SEMANTICS = "row-semantics"
# The desktop's route names (`reader_verdict::ROUTES`); a refused header reads unsupported. B7b (A4/A5): the
# retired v2 routes (physics, physics_source, load_reference, load_reference_source) are gone; a stored v2 result
# is refused at the header (SOURCE_EXACT_PRESSURE_V2_RETIRED).
ROUTES = frozenset({"pressure_source", "preview_physics", "pressure", "retained_preview_physics", "retained_physics"})
UNSUPPORTED = "unsupported"
# Each base route's raw reader (`reader_verdict::READERS`); its transport checks are `<reader>-transport`, which
# pressure-2 does not have (`transport("pressure")` is refused by name, SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED).
ROUTE_READERS = {"preview_physics": "preview-physics", "pressure_source": "pressure-source", "pressure": "pressure"}
STANDING_TOKENS = frozenset({"numerically_eligible", "needs_recompute", "unsupported"})
# The runner's diagnostic, exit 2, for input its JSON parser refuses.
INPUT_NOT_JSON = "openpipestress-runner: input is not JSON: "


class ReaderRefusal(ValueError):
    """The Rust reader refused the statement: its gate (retained reader only), bare code and optional detail."""

    def __init__(self, gate: str | None, code: str, detail: str | None):
        self.gate, self.code, self.detail = gate, code, detail
        super().__init__(detail or code)


def runner_executable() -> Path:
    configured = os.environ.get(ENV_EXECUTABLE)
    if not configured or not Path(configured).is_absolute():
        raise RuntimeError(f"RUST-READER-MISSING: set {ENV_EXECUTABLE} to an explicitly built absolute openpipestress-runner path")
    path = Path(configured)
    if not path.is_file():
        raise RuntimeError(f"RUST-READER-MISSING: {path}")
    return path


def _invalid(reader: str) -> RuntimeError:
    return RuntimeError(f"RUST-READER-RESPONSE-INVALID: {reader}")


def _run(reader: str, payload: Mapping[str, Any]) -> tuple[bool, dict[str, Any]]:
    """Run one reader over `payload`: whether it passed, and its outcome object.

    A payload with a non-finite number raises ValueError (`READER_INPUT_NOT_JSON`)
    and one with a non-JSON type TypeError, before the reader runs; one the runner's
    JSON parser refuses (an integer beyond binary64 range) raises ValueError
    (`READER_INPUT_NOT_JSON`)."""
    try:
        text = json.dumps(payload, ensure_ascii=False, allow_nan=False, separators=(",", ":"))
    except ValueError as error:
        # A non-finite number has no JSON text: no reader can read the statement, so it is refused.
        raise ValueError(f"READER_INPUT_NOT_JSON: {error}") from error
    completed = subprocess.run([str(runner_executable()), "read", reader], input=text.encode("utf-8"),
                               capture_output=True, check=False, shell=False)
    stderr = completed.stderr.decode("utf-8", "replace").strip()
    if completed.returncode == 2 and stderr.startswith(INPUT_NOT_JSON):
        # The reader's own parser refused the statement's text (for example an integer
        # beyond binary64 range, which Python writes and JSON does not bound): the reader
        # cannot read it, so it is refused, as a non-finite number is above.
        raise ValueError(f"READER_INPUT_NOT_JSON: {stderr[len(INPUT_NOT_JSON):]}")
    if completed.returncode not in (0, 1):
        raise RuntimeError(f"RUST-READER-FAILED: {reader} exited {completed.returncode}: {stderr}")
    try:
        verdict = json.loads(completed.stdout)
    except ValueError as error:
        raise _invalid(reader) from error
    passed = completed.returncode == 0
    outcome = verdict.get("outcome") if isinstance(verdict, dict) else None
    if (not isinstance(verdict, dict) or set(verdict) != {"artifact", "schema_version", "reader", "outcome"}
            or verdict["artifact"] != ARTIFACT or verdict["schema_version"] != SCHEMA_VERSION or verdict["reader"] != reader
            or not isinstance(outcome, dict) or outcome.get("ok") is not passed):
        raise _invalid(reader)
    return passed, outcome


def _refusal(reader: str, error: Any, *, gated: bool) -> ReaderRefusal:
    keys = {"gate", "code", "detail"} if gated else {"code", "detail"}
    if (not isinstance(error, dict) or set(error) != keys or not isinstance(error["code"], str) or not error["code"]
            or (gated and not isinstance(error["gate"], str)) or not (error["detail"] is None or isinstance(error["detail"], str))):
        raise _invalid(reader)
    return ReaderRefusal(error["gate"] if gated else None, error["code"], error["detail"])


def _base(reader: str, payload: Mapping[str, Any], members: set[str]) -> dict[str, Any]:
    """A base reader's passing outcome, whose members besides `ok` are `members`; a refusal raises."""
    passed, outcome = _run(reader, payload)
    if not passed:
        if set(outcome) != {"ok", "error"}:
            raise _invalid(reader)
        raise _refusal(reader, outcome["error"], gated=False)
    if set(outcome) != {"ok", *members}:
        raise _invalid(reader)
    return outcome


def read(reader: str, payload: Mapping[str, Any]) -> dict[str, Any]:
    """Run one retained reader over `payload`; return its validation or raise its refusal."""
    passed, outcome = _run(reader, payload)
    member = "validation" if passed else "error"
    if set(outcome) != {"ok", member} or not isinstance(outcome[member], dict):
        raise _invalid(reader)
    if not passed:
        raise _refusal(reader, outcome["error"], gated=True)
    return outcome["validation"]


def retained_precision(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """`retained_precision::validate`: G0-G8 on a successor, bound to `invocation` when one is given."""
    return read(RETAINED_PRECISION, {"source": source, "invocation": invocation})


def retained_precision_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """`retained_precision::validate_transport_metadata`: a transported successor's checks; never eligible."""
    return read(RETAINED_PRECISION_TRANSPORT, {"source": source})


def _optional(payload: dict[str, Any], **members: Any) -> dict[str, Any]:
    payload.update({name: value for name, value in members.items() if value is not None})
    return payload


def dispatch(source: Any, model: Any = None, invocation: Any = None) -> dict[str, Any]:
    """`reader_verdict::dispatch`: `{ok, route, binding, error, standing}`. A header the
    dispatch refuses is returned too (`ok` false, route unsupported, `error` {code, detail}),
    since its standing is still the reader's; it does not raise."""
    passed, outcome = _run(DISPATCH, _optional({"source": source}, model=model, invocation=invocation))
    binding, error, standing = outcome.get("binding"), outcome.get("error"), outcome.get("standing")
    if (set(outcome) != {"ok", "route", "binding", "error", "standing"}
            or outcome["route"] not in (ROUTES if passed else {UNSUPPORTED})
            or not (binding is None or (isinstance(binding, dict) and set(binding) == {"id", "sha256"}
                                        and all(isinstance(value, str) for value in binding.values())))
            or (binding is None) != (outcome["route"] in {"legacy", UNSUPPORTED})
            or not isinstance(standing, dict) or set(standing) != {"route", "token", "status", "eligible", "findings"}
            or standing["token"] not in STANDING_TOKENS or not isinstance(standing["eligible"], bool)
            or not isinstance(standing["findings"], list) or not all(isinstance(item, str) for item in standing["findings"])):
        raise _invalid(DISPATCH)
    if passed:
        if error is not None:
            raise _invalid(DISPATCH)
    else:
        _refusal(DISPATCH, error, gated=False)
    return outcome


def validate(reader: str, source: Any, *, model: Any = None, invocation: Any = None) -> dict[str, Any]:
    """`reader_verdict::validate`: one base reader's raw validation; `{ok, eligible?, findings?}` or its refusal raised."""
    if reader not in ROUTE_READERS.values():
        raise ValueError(f"RUST-READER-UNKNOWN: {reader}")
    members = {"preview-physics": set(), "pressure": set(), "pressure-source": {"eligible", "findings"}}[reader]
    outcome = _base(reader, _optional({"source": source}, model=model, invocation=invocation), members)
    if "eligible" in members and not isinstance(outcome["eligible"], bool):
        raise _invalid(reader)
    if "findings" in members and (not isinstance(outcome["findings"], list) or not all(isinstance(item, str) for item in outcome["findings"])):
        raise _invalid(reader)
    return outcome


def transport(reader: str, source: Any) -> None:
    """`<reader>-transport`: a transported statement's metadata checks; never eligible. A refusal raises."""
    if reader not in ROUTE_READERS.values():
        raise ValueError(f"RUST-READER-UNKNOWN: {reader}")
    _base(f"{reader}-transport", {"source": source}, set())


def standing(source: Any, requested_basis_refs: Any, invocation: Any = None) -> dict[str, Any]:
    """`semantic_contract::numerical_use_standing_with_context` and `standing_reason`: `{token, reason}`."""
    outcome = _base(STANDING, _optional({"source": source, "requested_basis_refs": requested_basis_refs}, invocation=invocation),
                    {"token", "reason"})
    if outcome["token"] not in STANDING_TOKENS or not (outcome["reason"] is None or isinstance(outcome["reason"], str)):
        raise _invalid(STANDING)
    return {"token": outcome["token"], "reason": outcome["reason"]}


def classification_summary(source: Any, invocation: Any = None, requested_basis_refs: Any = None) -> list[dict[str, Any]]:
    """`semantic_contract::classification_summary`: a successor's per-case class counts; empty otherwise."""
    outcome = _base(CLASSIFICATION_SUMMARY, _optional({"source": source}, invocation=invocation, requested_basis_refs=requested_basis_refs),
                    {"summary"})
    if not isinstance(outcome["summary"], list) or not all(isinstance(item, dict) for item in outcome["summary"]):
        raise _invalid(CLASSIFICATION_SUMMARY)
    return outcome["summary"]


def rule_binding(envelope: Any, row: Any) -> str | None:
    """`semantic_contract::rule_binding_refusal`: the refusal code, or None when the row may bind."""
    refusal = _base(RULE_BINDING, {"envelope": envelope, "row": row}, {"refusal"})["refusal"]
    if not (refusal is None or isinstance(refusal, str)):
        raise _invalid(RULE_BINDING)
    return refusal


def row_semantics(row: Any, source: Any = None) -> dict[str, Any]:
    """`reader_verdict::row_semantics`: `{signature, complete_metadata, canonical_metadata}`; a refusal raises."""
    outcome = _base(ROW_SEMANTICS, _optional({"row": row}, source=source), {"signature", "complete_metadata", "canonical_metadata"})
    return {key: outcome[key] for key in ("signature", "complete_metadata", "canonical_metadata")}
