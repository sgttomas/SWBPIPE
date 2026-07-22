"""Route-owned adapters for controlled export sinks.

This module deliberately leaves :mod:`controls` unchanged.  Route wrappers
remove payload-carried intent, project raw values into explicit metadata, run
the existing contract, and materialize only the controlled copy.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any, Mapping, Sequence
import copy

from .controls import RedactionDecision, RedactionFinding, redact_export_payload


INTENT_KEYS = {
    "local_private_intent",
    "explicit_local_private_intent",
    "user_intent",
}
PRIVATE_PATH_TOKENS = {
    "component",
    "content",
    "coordinate",
    "design_basis",
    "displacement",
    "force",
    "free_metadata",
    "geometry",
    "diameter",
    "material",
    "moment",
    "owner",
    "path",
    "pipe_segment",
    "project_name",
    "project_ref",
    "project_id",
    "result_value",
    "rotation",
    "rule_detail",
    "stress",
    "text",
    "thickness",
    "value",
}
ROUTE_KEY_PREFIX = "__route_key__"


@dataclass(frozen=True)
class ControlledExport:
    """Observable, fail-closed result returned by every final sink."""

    payload: Any | None
    decisions: tuple[RedactionDecision, ...]
    findings: tuple[RedactionFinding, ...]
    blocked: bool
    summary: dict[str, Any]

    def as_dict(self) -> dict[str, Any]:
        return {
            "payload": copy.deepcopy(self.payload),
            "decisions": [asdict(decision) for decision in self.decisions],
            "findings": [finding.as_schema_dict() for finding in self.findings],
            "blocked": self.blocked,
            "summary": copy.deepcopy(self.summary),
        }

    def __getitem__(self, key: str) -> Any:
        """Compatibility read-through while callers migrate to ``payload``."""

        if key in {"payload", "decisions", "findings", "blocked", "summary"}:
            return self.as_dict()[key]
        if isinstance(self.payload, Mapping):
            return self.payload[key]
        raise KeyError(key)


def control_route_export(
    payload: Any,
    *,
    route_id: str,
    export_context: str,
    explicit_local_private_intent: bool = False,
    require_lossless_materialization: bool = False,
    source_findings: Sequence[Mapping[str, Any]] = (),
) -> ControlledExport:
    """Return a controlled copy for one fixed-context route.

    ``route_id`` is recorded in projected field identifiers and is not a
    caller-selectable policy input.  Final wrappers hard-code it together
    with ``export_context``.
    """

    source = copy.deepcopy(payload)
    source_findings_copy = tuple(copy.deepcopy(dict(item)) for item in source_findings)
    source_blocking_count = sum(
        1
        for item in source_findings_copy
        if str(item.get("severity", "")).casefold() == "blocking"
    )
    projected = _project(
        _strip_source_intent(source),
        route_id=route_id,
        path="$",
    )
    result = redact_export_payload(
        projected,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
    )
    destructive = any(
        decision.action in {"redact_value", "redact_field", "omit_field", "block_export"}
        for decision in result.decisions
    )
    blocked = (
        result.blocked
        or source_blocking_count > 0
        or (require_lossless_materialization and destructive)
    )
    payload_out = None if blocked else _materialize(result.payload)
    summary = result.summary()
    summary["route_id"] = route_id
    summary["source_finding_count"] = len(source_findings_copy)
    summary["source_blocking_count"] = source_blocking_count
    summary["redaction_blocking_count"] = summary["blocking_count"]
    lossless_blocking_count = int(
        require_lossless_materialization and destructive
    )
    summary["lossless_blocking_count"] = lossless_blocking_count
    summary["exposure_blocking_count"] = (
        source_blocking_count
        + summary["redaction_blocking_count"]
        + lossless_blocking_count
    )
    summary["materialization_withheld"] = bool(lossless_blocking_count)
    return ControlledExport(
        payload=payload_out,
        decisions=result.decisions,
        findings=result.findings,
        blocked=blocked,
        summary=summary,
    )


def _strip_source_intent(value: Any) -> Any:
    if isinstance(value, Mapping):
        output: dict[str, Any] = {}
        for key, item in value.items():
            if str(key) in INTENT_KEYS:
                continue
            if key == "export_policy" and isinstance(item, Mapping):
                output[str(key)] = {
                    str(policy_key): _strip_source_intent(policy_value)
                    for policy_key, policy_value in item.items()
                    if str(policy_key) not in INTENT_KEYS
                }
            else:
                output[str(key)] = _strip_source_intent(item)
        return output
    if isinstance(value, list):
        return [_strip_source_intent(item) for item in value]
    if isinstance(value, tuple):
        return [_strip_source_intent(item) for item in value]
    return copy.deepcopy(value)


def _project(
    value: Any, *, route_id: str, path: str, public_basis: bool = False
) -> Any:
    if isinstance(value, Mapping):
        # Existing metadata-bearing items remain contract-native leaves.  They
        # have already had all payload-carried intent removed.
        if "value" in value and _has_contract_metadata(value):
            return copy.deepcopy(dict(value))
        # A public basis is record-local: it can classify scalar leaves owned
        # directly by this record, but it cannot flow through another record
        # or collection.  A nested record must declare its own complete basis.
        bounded_public_basis = _declares_direct_public_basis(value)
        return {
            f"{ROUTE_KEY_PREFIX}{key}": _project(
                item,
                route_id=route_id,
                path=f"{path}.{key}",
                public_basis=bounded_public_basis,
            )
            for key, item in value.items()
        }
    if isinstance(value, (list, tuple)):
        return [
            _project(
                item,
                route_id=route_id,
                path=f"{path}[{index}]",
                public_basis=False,
            )
            for index, item in enumerate(value)
        ]

    privacy, redistribution, review = _leaf_classification(
        path, value, public_basis=public_basis
    )
    return {
        "field_id": f"{route_id}:{path}",
        "field_class": _field_class(path),
        "privacy_classification": privacy,
        "redistribution_status": redistribution,
        "review_status": review,
        "value": copy.deepcopy(value),
        "_route_projected_leaf": True,
    }


def _materialize(value: Any) -> Any:
    if isinstance(value, Mapping):
        if value.get("_route_projected_leaf") is True:
            return copy.deepcopy(value.get("value"))
        return {
            (str(key)[len(ROUTE_KEY_PREFIX):] if str(key).startswith(ROUTE_KEY_PREFIX) else str(key)): _materialize(item)
            for key, item in value.items()
            if key != "_route_projected_leaf"
        }
    if isinstance(value, list):
        return [_materialize(item) for item in value]
    return copy.deepcopy(value)


def _declares_direct_public_basis(value: Mapping[str, Any]) -> bool:
    privacy = value.get("privacy_classification", value.get("classification"))
    redistribution = value.get("redistribution_status")
    return privacy in {"public", "public_metadata", "invented_public_example"} and redistribution in {
        "public_permissive",
        "invented_non_engineering_example",
    }


def _leaf_classification(
    path: str, value: Any, *, public_basis: bool
) -> tuple[str, str, str]:
    lowered = path.lower().replace("-", "_")
    if value is None or value == "TBD" or "tbd" in lowered:
        return "unknown", "unknown", "pending"
    segments = lowered.replace("[", ".").replace("]", "").split(".")
    leaf_segment = segments[-1] if segments else "unknown"
    checksum_value = leaf_segment == "value" and (
        "checksum" in lowered or "hash" in lowered
    )
    unit_map_leaf = ".model_units." in lowered or ".target_export_units." in lowered
    private_value_leaf = not (checksum_value or unit_map_leaf) and any(
        leaf_segment == token or leaf_segment.endswith(f"_{token}")
        for token in PRIVATE_PATH_TOKENS
    )
    if any(token in lowered for token in PRIVATE_PATH_TOKENS):
        return "private_project_data", "private_only", "accepted"
    if public_basis:
        return "public_metadata", "public_permissive", "accepted"
    return "unknown", "unknown", "pending"


def _field_class(path: str) -> str:
    leaf = path.rsplit(".", 1)[-1]
    return leaf.split("[", 1)[0] or "unknown"


def _has_contract_metadata(value: Mapping[str, Any]) -> bool:
    if isinstance(value.get("value"), (Mapping, list, tuple)):
        return False
    if any(
        key in value and not isinstance(value.get(key), (Mapping, list, tuple))
        for key in (
            "privacy_classification",
            "redistribution_status",
            "review_status",
        )
    ):
        return True
    provenance = value.get("provenance")
    return isinstance(provenance, Mapping) and any(
        key in provenance and not isinstance(provenance.get(key), (Mapping, list, tuple))
        for key in ("privacy_classification", "redistribution_status", "review_status")
    )
