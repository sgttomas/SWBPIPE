"""Provider-neutral target mapping contract for DEL-15-02.

The contract records mapping metadata and unsupported or approximate behavior
without selecting a commercial target, invoking an external solver, or making
professional acceptance claims.
"""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)


TARGET_MAPPING_CONTRACT_VERSION = "0.1.0"

SUPPORTED_TARGET_SYSTEM_KINDS = {
    "local_fea",
    "external_prover",
    "commercial_tool_reference",
    "generic_downstream_modeling",
    "TBD",
}
SUPPORTED_MAPPING_KINDS = {
    "entity",
    "field",
    "unit",
    "result",
    "load_case",
    "boundary_condition",
    "metadata",
}
SUPPORTED_BEHAVIOR_STATUSES = {
    "unsupported",
    "approximate",
    "requires_human_review",
    "not_implemented",
    "TBD",
}
UNIT_VALUE_KINDS = {"quantity", "unit", "dimensioned_quantity"}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-15-02 target mapping contract",
    "source_location": "core/handoff/target_mapping/contract.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}


def build_target_mapping_contract(
    *,
    mapping_contract_id: str,
    target_system_kind: str,
    target_ref: Mapping[str, Any],
    source_context: Mapping[str, Any],
    mapping_records: list[Mapping[str, Any]],
    unsupported_behaviors: list[Mapping[str, Any]] | None = None,
    approximate_behaviors: list[Mapping[str, Any]] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build a deterministic DEL-15-02 target mapping contract envelope."""

    unsupported_behaviors = unsupported_behaviors or []
    approximate_behaviors = approximate_behaviors or []
    contract = {
        "schema_version": TARGET_MAPPING_CONTRACT_VERSION,
        "deliverable_id": "DEL-15-02",
        "package_id": "PKG-15",
        "scope_item": "SOW-074",
        "objectives": ["OBJ-017"],
        "mapping_contract_id": mapping_contract_id,
        "mapping_schema_status": "provider_neutral_contract",
        "target_system_kind": target_system_kind,
        "target_ref": deepcopy(dict(target_ref)),
        "source_context": _source_context(source_context),
        "mapping_records": [_mapping_record(item, index) for index, item in enumerate(mapping_records)],
        "unsupported_behavior_flags": [
            _behavior_flag(item, index, default_status="unsupported")
            for index, item in enumerate(unsupported_behaviors)
        ],
        "approximate_behavior_flags": [
            _behavior_flag(item, index, default_status="approximate")
            for index, item in enumerate(approximate_behaviors)
        ],
        "diagnostics": [],
        "provenance": deepcopy(dict(provenance or ENGINE_PROVENANCE)),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    contract["diagnostics"] = diagnostics_for_target_mapping_contract(contract)
    return _sort_contract(contract)


def diagnostics_for_target_mapping_contract(contract: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for missing mapping context."""

    diagnostics: list[dict[str, Any]] = []
    target_kind = contract.get("target_system_kind")
    if target_kind not in SUPPORTED_TARGET_SYSTEM_KINDS:
        diagnostics.append(
            _diagnostic(
                "TM-TARGET-KIND-UNSUPPORTED",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target system kind is not part of the provider-neutral taxonomy.",
                "Use the supported generic target taxonomy or record target selection as TBD.",
                [_affected("target_system_kind", str(target_kind))],
            )
        )

    context = contract.get("source_context")
    if not isinstance(context, Mapping):
        diagnostics.append(
            _diagnostic(
                "TM-SOURCE-CONTEXT-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target mapping requires model hash, units manifest, entity IDs, assumptions, warnings, and privacy context.",
                "Provide a source_context object derived from the handoff package manifest.",
                [_affected("source_context", "missing")],
            )
        )
    else:
        for field in (
            "model_hash",
            "units_manifest_ref",
            "entity_id_refs",
            "library_refs",
            "rule_pack_refs",
            "unresolved_assumption_refs",
            "warning_refs",
            "privacy_context",
        ):
            if not context.get(field):
                diagnostics.append(
                    _diagnostic(
                        "TM-SOURCE-CONTEXT-FIELD-MISSING",
                        "blocking",
                        "TARGET_MAPPING_WARNING",
                        f"Source context is missing {field}.",
                        "Carry forward the handoff package context instead of emitting silent defaults.",
                        [_affected("source_context", field)],
                    )
                )

    for record in _list(contract.get("mapping_records")):
        diagnostics.extend(_mapping_diagnostics(record))

    behavior_flags = _list(contract.get("unsupported_behavior_flags")) + _list(
        contract.get("approximate_behavior_flags")
    )
    for flag in behavior_flags:
        diagnostics.extend(_behavior_diagnostics(flag))

    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize a mapping contract with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _source_context(source_context: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "model_hash": deepcopy(source_context.get("model_hash")),
        "units_manifest_ref": deepcopy(source_context.get("units_manifest_ref")),
        "entity_id_refs": deepcopy(list(source_context.get("entity_id_refs", []))),
        "library_refs": deepcopy(list(source_context.get("library_refs", []))),
        "rule_pack_refs": deepcopy(list(source_context.get("rule_pack_refs", []))),
        "unresolved_assumption_refs": deepcopy(
            list(source_context.get("unresolved_assumption_refs", []))
        ),
        "warning_refs": deepcopy(list(source_context.get("warning_refs", []))),
        "privacy_context": deepcopy(source_context.get("privacy_context")),
    }


def _mapping_record(record: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "mapping_id": str(record.get("mapping_id", f"mapping:{index:04d}")),
        "mapping_kind": str(record.get("mapping_kind", "metadata")),
        "source_ref": deepcopy(dict(record.get("source_ref", _ref("TBD", "missing")))),
        "target_ref": deepcopy(dict(record.get("target_ref", _ref("TBD", "missing")))),
        "value_kind": str(record.get("value_kind", "metadata")),
        "unit_metadata": deepcopy(record.get("unit_metadata")),
        "mapping_status": str(record.get("mapping_status", "mapped")),
        "assumption_refs": deepcopy(list(record.get("assumption_refs", []))),
        "warning_refs": deepcopy(list(record.get("warning_refs", []))),
        "provenance": deepcopy(dict(record.get("provenance", ENGINE_PROVENANCE))),
    }


def _behavior_flag(flag: Mapping[str, Any], index: int, *, default_status: str) -> dict[str, Any]:
    return {
        "flag_id": str(flag.get("flag_id", f"behavior:{index:04d}")),
        "behavior_label": str(flag.get("behavior_label", "unsupported_target_behavior")),
        "status": str(flag.get("status", default_status)),
        "target_ref": deepcopy(dict(flag.get("target_ref", _ref("TBD", "missing")))),
        "affected_refs": deepcopy(list(flag.get("affected_refs", []))),
        "assumption_refs": deepcopy(list(flag.get("assumption_refs", []))),
        "warning_refs": deepcopy(list(flag.get("warning_refs", []))),
        "human_review_required": True,
        "provenance": deepcopy(dict(flag.get("provenance", ENGINE_PROVENANCE))),
    }


def _mapping_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    mapping_id = str(record.get("mapping_id", "unknown"))
    if record.get("mapping_kind") not in SUPPORTED_MAPPING_KINDS:
        diagnostics.append(
            _diagnostic(
                "TM-MAPPING-KIND-UNSUPPORTED",
                "warning",
                "TARGET_MAPPING_WARNING",
                "Mapping kind is outside the provider-neutral taxonomy.",
                "Record the behavior as unsupported or route it to a later target-specific adapter.",
                [_affected("mapping_record", mapping_id)],
            )
        )
    if record.get("value_kind") in UNIT_VALUE_KINDS and not isinstance(
        record.get("unit_metadata"), Mapping
    ):
        diagnostics.append(
            _diagnostic(
                "TM-UNIT-METADATA-MISSING",
                "blocking",
                "UNIT_WARNING",
                "Unit-bearing mapped values require explicit unit and dimension metadata.",
                "Add unit_metadata or emit an unsupported behavior flag instead of a mapped value.",
                [_affected("mapping_record", mapping_id)],
            )
        )
    if record.get("mapping_status") in {"unsupported", "approximate"} and not (
        record.get("assumption_refs") or record.get("warning_refs")
    ):
        diagnostics.append(
            _diagnostic(
                "TM-MAPPING-DISCLOSURE-MISSING",
                "warning",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Unsupported or approximate mappings require traceable assumptions or warnings.",
                "Attach assumption_refs, warning_refs, or an unsupported behavior flag.",
                [_affected("mapping_record", mapping_id)],
            )
        )
    return diagnostics


def _behavior_diagnostics(flag: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    flag_id = str(flag.get("flag_id", "unknown"))
    if contains_prohibited_authority_term(flag.get("behavior_label", "")):
        diagnostics.append(
            _diagnostic(
                "TM-PROHIBITED-AUTHORITY-TERM",
                "blocking",
                "AUTHORITY_BOUNDARY",
                "Behavior label contains prohibited authority or lifecycle wording.",
                "Use neutral unsupported-target wording and keep any human decision outside the software-generated mapping contract.",
                [_affected("unsupported_behavior", flag_id)],
            )
        )
    if flag.get("status") not in SUPPORTED_BEHAVIOR_STATUSES:
        diagnostics.append(
            _diagnostic(
                "TM-BEHAVIOR-STATUS-UNSUPPORTED",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Unsupported behavior status is not part of the contract taxonomy.",
                "Use unsupported, approximate, requires_human_review, not_implemented, or TBD.",
                [_affected("unsupported_behavior", flag_id)],
            )
        )
    if not flag.get("human_review_required"):
        diagnostics.append(
            _diagnostic(
                "TM-HUMAN-REVIEW-REQUIRED",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Unsupported or approximate target behavior requires human review.",
                "Set human_review_required to true.",
                [_affected("unsupported_behavior", flag_id)],
            )
        )
    if not flag.get("affected_refs"):
        diagnostics.append(
            _diagnostic(
                "TM-AFFECTED-REFS-MISSING",
                "warning",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Unsupported behavior should identify affected source or target records.",
                "Provide affected_refs so the disclosure is traceable.",
                [_affected("unsupported_behavior", flag_id)],
            )
        )
    return diagnostics


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[dict[str, str]],
) -> dict[str, Any]:
    return {
        "code": code,
        "severity": severity,
        "class": diagnostic_class,
        "affected_references": affected_refs,
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _affected(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _ref(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _stable(diagnostics: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return sorted(diagnostics, key=canonical_json)


def _sort_contract(contract: dict[str, Any]) -> dict[str, Any]:
    contract["mapping_records"].sort(key=lambda item: item["mapping_id"])
    contract["unsupported_behavior_flags"].sort(key=lambda item: item["flag_id"])
    contract["approximate_behavior_flags"].sort(key=lambda item: item["flag_id"])
    contract["diagnostics"].sort(key=canonical_json)
    return contract
