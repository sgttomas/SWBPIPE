"""Provider-neutral handoff export workflow for DEL-15-03.

The workflow assembles already-schema-shaped handoff and target-mapping
records into a deterministic export envelope. It does not parse commercial
tool formats, invoke external solvers/provers, ingest downstream results, or
make professional reliance claims.
"""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)
from core.security.redaction import ControlledExport, control_route_export


EXPORT_WORKFLOW_VERSION = "0.1.0"

SUPPORTED_TARGET_SYSTEM_KINDS = {
    "local_fea",
    "external_prover",
    "commercial_tool_reference",
    "generic_downstream_modeling",
    "TBD",
}

UNIT_VALUE_KINDS = {"quantity", "unit", "dimensioned_quantity"}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-15-03 handoff export workflow",
    "source_location": "core/handoff/exporter/workflow.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_attestation": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_classification": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_downstream_modeling": True,
    "supports_downstream_review": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "software_creates_professional_reliance_record": False,
}


def build_handoff_export_workflow(
    *,
    export_workflow_id: str,
    handoff_package: Mapping[str, Any],
    target_mapping_contract: Mapping[str, Any] | None = None,
    target_fixture: Mapping[str, Any] | None = None,
    adapter_contract: Mapping[str, Any] | None = None,
    local_fea_contract: Mapping[str, Any] | None = None,
    redaction_contract: Mapping[str, Any] | None = None,
    transform_contract: Mapping[str, Any] | None = None,
    comparison_contract: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> ControlledExport:
    """Build and execute the controlled provider-neutral export workflow."""

    manifest = _manifest(handoff_package)
    mapping_contract = (
        target_mapping_contract
        if isinstance(target_mapping_contract, Mapping)
        else {}
    )
    fixture = target_fixture if isinstance(target_fixture, Mapping) else {}
    mapping_records = _list(mapping_contract.get("mapping_records"))
    unsupported_records = _unsupported_records(manifest, mapping_contract, fixture)
    payload = {
        "schema_version": EXPORT_WORKFLOW_VERSION,
        "deliverable_id": "DEL-15-03",
        "package_id": "PKG-15",
        "scope_item": "SOW-074",
        "objectives": ["OBJ-017"],
        "export_workflow_id": str(export_workflow_id),
        "workflow_status": "provider_neutral_export_package",
        "target_fixture": (
            _target_fixture(target_fixture)
            if isinstance(target_fixture, Mapping)
            else deepcopy(target_fixture)
        ),
        "source_handoff_package": deepcopy(dict(handoff_package)),
        "target_mapping_contract": (
            deepcopy(dict(target_mapping_contract))
            if isinstance(target_mapping_contract, Mapping)
            else deepcopy(target_mapping_contract)
        ),
        "preserved_context": _preserved_context(manifest, mapping_contract),
        "export_payload": {
            "package_identity": deepcopy(manifest.get("package_identity")),
            "model_hash": deepcopy(manifest.get("model_hash")),
            "units_manifest": deepcopy(manifest.get("units_manifest")),
            "entity_ids": deepcopy(manifest.get("entity_ids")),
            "library_refs": deepcopy(_list(manifest.get("library_refs"))),
            "rule_pack_refs": deepcopy(_list(manifest.get("rule_pack_refs"))),
            "target_mapping_metadata": deepcopy(manifest.get("target_mapping_metadata")),
            "mapping_records": deepcopy(sorted(mapping_records, key=_mapping_id)),
            "unsupported_target_records": unsupported_records,
            "unresolved_assumptions": deepcopy(_list(manifest.get("unresolved_assumptions"))),
            "warnings": deepcopy(_list(manifest.get("warnings"))),
            "diagnostics": [],
            "provenance_references": _provenance_references(manifest, mapping_contract),
            "data_contracts": _data_contracts(
                adapter_contract=adapter_contract,
                local_fea_contract=local_fea_contract,
                redaction_contract=redaction_contract,
                transform_contract=transform_contract,
                comparison_contract=comparison_contract,
            ),
        },
        "privacy": deepcopy(manifest.get("privacy")),
        "provenance": deepcopy(dict(provenance or ENGINE_PROVENANCE)),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    payload["diagnostics"] = diagnostics_for_export_workflow(payload)
    payload["export_payload"]["diagnostics"] = deepcopy(payload["diagnostics"])
    return control_route_export(
        _sort_workflow(payload),
        route_id="REXC-CORE-001",
        export_context="downstream_tool",
        source_findings=payload["diagnostics"],
    )


def diagnostics_for_export_workflow(workflow: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for an export workflow envelope."""

    diagnostics: list[dict[str, Any]] = []
    handoff_package = workflow.get("source_handoff_package")
    manifest = _manifest(handoff_package if isinstance(handoff_package, Mapping) else {})
    target_mapping = workflow.get("target_mapping_contract")
    target_mapping_is_mapping = isinstance(target_mapping, Mapping)
    if not target_mapping_is_mapping:
        diagnostics.append(
            _diagnostic(
                "EXP-TARGET-MAPPING-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target mapping contract is missing.",
                "Provide a DEL-15-02 target mapping contract before export.",
                [_affected("TargetMapping", "missing")],
            )
        )
        target_mapping = {}

    diagnostics.extend(_handoff_diagnostics(workflow, manifest))
    if target_mapping_is_mapping:
        diagnostics.extend(_mapping_diagnostics(manifest, target_mapping))
    diagnostics.extend(_target_fixture_diagnostics(workflow.get("target_fixture"), target_mapping))
    diagnostics.extend(_privacy_diagnostics(workflow.get("privacy")))

    for record in _list(target_mapping.get("mapping_records")):
        diagnostics.extend(_mapping_record_diagnostics(record))

    if not _list(workflow.get("export_payload", {}).get("unsupported_target_records")):
        diagnostics.append(
            _diagnostic(
                "EXP-UNSUPPORTED-RECORDS-MISSING",
                "warning",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Export workflow has no explicit unsupported-target records.",
                "Carry forward handoff, mapping, target capability, or not-invoked records.",
                [_affected("HandoffPackage", _handoff_id(manifest))],
            )
        )

    return sorted(diagnostics, key=canonical_json)


def canonical_json(value: Any) -> str:
    """Serialize an export workflow with stable key ordering."""
    if isinstance(value, ControlledExport):
        value = value.as_dict()
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _manifest(handoff_package: Mapping[str, Any]) -> Mapping[str, Any]:
    manifest = handoff_package.get("handoff_package_manifest")
    return manifest if isinstance(manifest, Mapping) else {}


def _target_fixture(target_fixture: Mapping[str, Any]) -> dict[str, Any]:
    fixture = deepcopy(dict(target_fixture))
    fixture.setdefault("target_system_kind", "TBD")
    fixture.setdefault("target_ref", _ref("ExternalReference", "target:TBD"))
    fixture.setdefault("capabilities", [])
    fixture.setdefault("unsupported_capabilities", [])
    fixture.setdefault("provenance", deepcopy(ENGINE_PROVENANCE))
    return fixture


def _preserved_context(
    manifest: Mapping[str, Any], target_mapping_contract: Mapping[str, Any]
) -> dict[str, Any]:
    return {
        "model_hash": deepcopy(manifest.get("model_hash")),
        "units_manifest": deepcopy(manifest.get("units_manifest")),
        "entity_ids": deepcopy(manifest.get("entity_ids")),
        "library_refs": deepcopy(_list(manifest.get("library_refs"))),
        "rule_pack_refs": deepcopy(_list(manifest.get("rule_pack_refs"))),
        "unresolved_assumptions": deepcopy(_list(manifest.get("unresolved_assumptions"))),
        "warnings": deepcopy(_list(manifest.get("warnings"))),
        "target_mapping_metadata": deepcopy(manifest.get("target_mapping_metadata")),
        "mapping_source_context": deepcopy(
            target_mapping_contract.get("source_context")
            if isinstance(target_mapping_contract, Mapping)
            else None
        ),
    }


def _data_contracts(
    *,
    adapter_contract: Mapping[str, Any] | None,
    local_fea_contract: Mapping[str, Any] | None,
    redaction_contract: Mapping[str, Any] | None,
    transform_contract: Mapping[str, Any] | None,
    comparison_contract: Mapping[str, Any] | None,
) -> dict[str, Any]:
    return {
        "adapter_framework": _contract_ref(
            "DEL-10-02",
            "schema_first_format_neutral_adapter_framework",
            "schemas/adapter_framework.schema.yaml",
            adapter_contract,
        ),
        "local_fea_handoff": _contract_ref(
            "DEL-10-03",
            "schema_first_local_fea_handoff_contract",
            "schemas/local_fea_handoff.schema.yaml",
            local_fea_contract,
        ),
        "redaction_export_controls": _contract_ref(
            "DEL-12-02",
            "explicit_metadata_redaction_controls",
            "schemas/redaction_export_controls.schema.yaml",
            redaction_contract,
        ),
        "physical_to_analytical_transform": _contract_ref(
            "DEL-13-04",
            "physical_to_analytical_reference_contract",
            "core/model_transform/physical_to_analytical/contract.py",
            transform_contract,
        ),
        "comparison_mapping_export": _contract_ref(
            "DEL-14-05",
            "json_csv_review_contract_only",
            "schemas/comparison_mapping.schema.json",
            comparison_contract,
        ),
    }


def _contract_ref(
    deliverable_id: str,
    contract_kind: str,
    schema_ref: str,
    payload: Mapping[str, Any] | None,
) -> dict[str, Any]:
    return {
        "deliverable_id": deliverable_id,
        "contract_kind": contract_kind,
        "schema_ref": schema_ref,
        "payload_ref": deepcopy(payload.get("payload_ref")) if isinstance(payload, Mapping) else None,
        "status": "referenced_contract_not_executed",
    }


def _unsupported_records(
    manifest: Mapping[str, Any],
    target_mapping_contract: Mapping[str, Any],
    target_fixture: Mapping[str, Any],
) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for flag in _list(manifest.get("unsupported_behavior_flags")):
        records.append(_unsupported_record("handoff_manifest", flag))
    for flag in _list(target_mapping_contract.get("unsupported_behavior_flags")):
        records.append(_unsupported_record("target_mapping_contract", flag))
    for flag in _list(target_mapping_contract.get("approximate_behavior_flags")):
        records.append(_unsupported_record("target_mapping_contract", flag))
    for capability in _list(target_fixture.get("unsupported_capabilities")):
        records.append(
            {
                "record_id": str(capability.get("capability_id", "target:unsupported")),
                "source": "target_fixture",
                "behavior_label": str(capability.get("behavior_label", "unsupported_target_behavior")),
                "status": str(capability.get("status", "unsupported")),
                "target_ref": deepcopy(capability.get("target_ref", target_fixture.get("target_ref"))),
                "affected_refs": deepcopy(_list(capability.get("affected_refs"))),
                "assumption_refs": deepcopy(_list(capability.get("assumption_refs"))),
                "warning_refs": deepcopy(_list(capability.get("warning_refs"))),
                "human_review_required": True,
                "provenance": deepcopy(capability.get("provenance", target_fixture.get("provenance", ENGINE_PROVENANCE))),
            }
        )
    records.sort(key=canonical_json)
    return records


def _unsupported_record(source: str, flag: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "record_id": str(flag.get("flag_id", "unsupported:unknown")),
        "source": source,
        "behavior_label": str(flag.get("behavior_label", "unsupported_target_behavior")),
        "status": str(flag.get("status", "unsupported")),
        "target_ref": deepcopy(flag.get("target_ref")),
        "affected_refs": deepcopy(_list(flag.get("affected_refs"))),
        "assumption_refs": deepcopy(_list(flag.get("assumption_refs"))),
        "warning_refs": deepcopy(_list(flag.get("warning_refs"))),
        "human_review_required": True,
        "provenance": deepcopy(flag.get("provenance", ENGINE_PROVENANCE)),
    }


def _provenance_references(
    manifest: Mapping[str, Any], target_mapping_contract: Mapping[str, Any]
) -> list[dict[str, Any]]:
    refs = [
        {"record": "handoff_package_manifest", "provenance": deepcopy(manifest.get("provenance"))},
        {
            "record": "target_mapping_contract",
            "provenance": deepcopy(target_mapping_contract.get("provenance")),
        },
    ]
    for index, item in enumerate(_list(manifest.get("library_refs"))):
        refs.append({"record": f"library_refs[{index}]", "provenance": deepcopy(item.get("provenance"))})
    for index, item in enumerate(_list(manifest.get("rule_pack_refs"))):
        refs.append({"record": f"rule_pack_refs[{index}]", "provenance": deepcopy(item.get("provenance"))})
    return sorted(refs, key=canonical_json)


def _handoff_diagnostics(workflow: Mapping[str, Any], manifest: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    handoff_package = workflow.get("source_handoff_package")
    if not isinstance(handoff_package, Mapping) or handoff_package.get("deliverable_id") != "DEL-15-01":
        diagnostics.append(
            _diagnostic(
                "EXP-HANDOFF-PACKAGE-INVALID",
                "blocking",
                "EXPORT_BLOCKING",
                "Export workflow requires a canonical DEL-15-01 handoff package.",
                "Provide the schema-compliant handoff package before export.",
                [_affected("HandoffPackage", "source_handoff_package")],
            )
        )
    for field in (
        "package_identity",
        "model_hash",
        "units_manifest",
        "entity_ids",
        "target_mapping_metadata",
        "privacy",
        "provenance",
        "professional_boundary",
    ):
        field_value = manifest.get(field)
        if (
            not isinstance(field_value, Mapping) or not field_value
            if field == "units_manifest"
            else not field_value
        ):
            diagnostics.append(
                _diagnostic(
                    "EXP-HANDOFF-MANIFEST-FIELD-MISSING",
                    "blocking",
                    "EXPORT_BLOCKING",
                    f"Handoff manifest is missing {field}.",
                    "Preserve required manifest fields instead of exporting a placeholder.",
                    [_affected("HandoffPackage", field)],
                )
            )
    return diagnostics


def _mapping_diagnostics(
    manifest: Mapping[str, Any], target_mapping_contract: Mapping[str, Any]
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if target_mapping_contract.get("deliverable_id") != "DEL-15-02":
        diagnostics.append(
            _diagnostic(
                "EXP-TARGET-MAPPING-INVALID",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Export workflow requires a DEL-15-02 target mapping contract.",
                "Provide target mapping metadata before export.",
                [_affected("TargetMapping", "target_mapping_contract")],
            )
        )
    source_context = target_mapping_contract.get("source_context")
    if not isinstance(source_context, Mapping):
        diagnostics.append(
            _diagnostic(
                "EXP-MAPPING-CONTEXT-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target mapping source context is missing.",
                "Carry handoff model hash, units, entity IDs, assumptions, warnings, and privacy context.",
                [_affected("TargetMapping", "source_context")],
            )
        )
        return diagnostics

    model_hash = manifest.get("model_hash")
    if source_context.get("model_hash") != model_hash:
        diagnostics.append(
            _diagnostic(
                "EXP-MODEL-HASH-MISMATCH",
                "blocking",
                "HASH_WARNING",
                "Target mapping source context does not match the handoff model hash.",
                "Rebuild target mapping from the same handoff package.",
                [_affected("TargetMapping", "source_context.model_hash")],
            )
        )
    units_manifest = manifest.get("units_manifest")
    if isinstance(units_manifest, Mapping) and (
        source_context.get("units_manifest_ref")
        != units_manifest.get("unit_system_ref")
    ):
        diagnostics.append(
            _diagnostic(
                "EXP-UNITS-MANIFEST-MISMATCH",
                "blocking",
                "UNIT_WARNING",
                "Target mapping source context does not match the handoff units manifest reference.",
                "Rebuild target mapping from the same units manifest.",
                [_affected("TargetMapping", "source_context.units_manifest_ref")],
            )
        )
    return diagnostics


def _target_fixture_diagnostics(
    target_fixture: Any, target_mapping_contract: Mapping[str, Any]
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if not isinstance(target_fixture, Mapping):
        return [
            _diagnostic(
                "EXP-TARGET-FIXTURE-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target fixture is missing.",
                "Provide an invented provider-neutral target fixture.",
                [_affected("ExternalReference", "target_fixture")],
            )
        ]
    target_kind = target_fixture.get("target_system_kind")
    if target_kind not in SUPPORTED_TARGET_SYSTEM_KINDS:
        diagnostics.append(
            _diagnostic(
                "EXP-TARGET-KIND-UNSUPPORTED",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Target fixture kind is outside the provider-neutral taxonomy.",
                "Use a supported generic target kind or record target selection as TBD.",
                [_affected("ExternalReference", str(target_kind))],
            )
        )
    mapping_target_kind = target_mapping_contract.get("target_system_kind")
    if mapping_target_kind is not None and target_kind != mapping_target_kind:
        diagnostics.append(
            _diagnostic(
                "EXP-TARGET-KIND-MISMATCH",
                "warning",
                "TARGET_MAPPING_WARNING",
                "Target fixture kind differs from target mapping contract kind.",
                "Align target metadata or emit explicit unsupported records.",
                [_affected("TargetMapping", "target_system_kind")],
            )
        )
    for field in ("notes", "free_metadata", "metadata", "tags"):
        if field in target_fixture and contains_prohibited_authority_term(target_fixture.get(field)):
            diagnostics.append(
                _diagnostic(
                    "EXP-PROHIBITED-AUTHORITY-TERM",
                    "blocking",
                    "AUTHORITY_BOUNDARY",
                    "Target fixture metadata contains prohibited authority or lifecycle wording.",
                    "Use neutral target metadata and keep any human decision outside the software-generated export envelope.",
                    [_affected("TargetFixtureField", field)],
                )
            )
    for capability in _list(target_fixture.get("unsupported_capabilities")):
        capability_id = str(capability.get("capability_id", "target:unsupported"))
        if contains_prohibited_authority_term(capability.get("behavior_label", "")):
            diagnostics.append(
                _diagnostic(
                    "EXP-PROHIBITED-AUTHORITY-TERM",
                    "blocking",
                    "AUTHORITY_BOUNDARY",
                    "Target fixture unsupported capability label contains prohibited authority or lifecycle wording.",
                    "Use neutral unsupported-target wording and keep any human decision outside the software-generated export envelope.",
                    [_affected("TargetFixtureCapability", capability_id)],
                )
            )
    return diagnostics


def _privacy_diagnostics(privacy: Any) -> list[dict[str, Any]]:
    if not isinstance(privacy, Mapping):
        return [
            _diagnostic(
                "EXP-PRIVACY-MISSING",
                "blocking",
                "PRIVACY_WARNING",
                "Export workflow privacy context is missing.",
                "Carry forward the handoff privacy context.",
                [_affected("HandoffPackage", "privacy")],
            )
        ]
    diagnostics: list[dict[str, Any]] = []
    for key in (
        "private_payload_embedded",
        "protected_payload_embedded",
        "commercial_tool_payload_embedded",
        "telemetry_allowed",
    ):
        if privacy.get(key):
            diagnostics.append(
                _diagnostic(
                    "EXP-PRIVACY-EXPORT-BLOCKED",
                    "blocking",
                    "PRIVACY_WARNING",
                    f"Export privacy context has {key} enabled.",
                    "Redact or remove private, protected, commercial, or telemetry payloads before export.",
                    [_affected("HandoffPackage", f"privacy.{key}")],
                )
            )
    return diagnostics


def _mapping_record_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    if record.get("value_kind") in UNIT_VALUE_KINDS and not isinstance(
        record.get("unit_metadata"), Mapping
    ):
        return [
            _diagnostic(
                "EXP-UNIT-METADATA-MISSING",
                "blocking",
                "UNIT_WARNING",
                "Unit-bearing export mapping is missing unit metadata.",
                "Add explicit unit and dimension metadata or record unsupported behavior.",
                [_affected("TargetMapping", str(record.get("mapping_id", "mapping:unknown")))],
            )
        ]
    return []


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
        "source": _ref("ExternalReference", "core/handoff/exporter/workflow.py"),
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


def _mapping_id(record: Mapping[str, Any]) -> str:
    return str(record.get("mapping_id", ""))


def _handoff_id(manifest: Mapping[str, Any]) -> str:
    identity = manifest.get("package_identity")
    if isinstance(identity, Mapping):
        return str(identity.get("handoff_package_id", "handoff:unknown"))
    return "handoff:unknown"


def _sort_workflow(workflow: dict[str, Any]) -> dict[str, Any]:
    workflow["diagnostics"].sort(key=canonical_json)
    payload = workflow["export_payload"]
    payload["mapping_records"].sort(key=_mapping_id)
    payload["unsupported_target_records"].sort(key=canonical_json)
    payload["provenance_references"].sort(key=canonical_json)
    payload["diagnostics"].sort(key=canonical_json)
    return workflow
