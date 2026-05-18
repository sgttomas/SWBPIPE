"""Report-section assembly for DEL-08-06.

The assembler consumes already-formed model-state, analysis-run, comparison,
handoff, export-workflow, and external-prover metadata records. It does not
read project files, execute solvers, render reports, invoke external tools, or
create professional/code-compliance determinations.
"""

from __future__ import annotations

from copy import deepcopy
import json
from math import isfinite
from typing import Any, Mapping


REPORT_SECTION_VERSION = "0.1.0"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "software_creates_professional_reliance_record": False,
    "software_creates_external_validation_record": False,
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-08-06 report-section assembler",
    "source_location": "core/reporting/state_comparison_handoff_sections/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_attestation": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_classification": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROHIBITED_AUTHORITY_TERMS = {
    "approval",
    "approved",
    "certification",
    "cert" + "ified",
    "code_compliance",
    "code-compliance",
    "code " + "compliant",
    "compliant",
    "seal",
    "se" + "aled",
    "sealing",
    "authentication",
    "authentic" + "ated",
    "endorsement",
    "endorsed",
    "external_validation",
    "external " + "validation",
    "validated",
    "professional_acceptance",
    "professional " + "acceptance",
    "professional_reliance",
    "professional " + "reliance",
    "engineering_acceptance",
    "engineering " + "acceptance",
}

AUTHORITY_FLAG_NAMES = {
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_makes_authentication_claim",
    "software_creates_professional_reliance_record",
    "software_creates_external_validation_record",
}

REQUIRED_MODEL_STATE_FIELDS = (
    "state_id",
    "analysis_status",
    "hashes",
    "warnings",
    "unresolved_assumptions",
    "professional_boundary",
    "provenance",
)

REQUIRED_ANALYSIS_RUN_FIELDS = (
    "run_id",
    "model_state_ref",
    "solver_version",
    "settings_ref",
    "unit_system_ref",
    "diagnostics",
    "result_refs",
    "rule_pack_refs",
    "library_refs",
    "hashes",
    "analysis_status",
    "reproducibility",
    "professional_boundary",
    "provenance",
)


def build_state_comparison_handoff_report_sections(
    *,
    section_set_id: str,
    model_states: list[Mapping[str, Any]] | None = None,
    analysis_runs: list[Mapping[str, Any]] | None = None,
    model_state_comparisons: list[Mapping[str, Any]] | None = None,
    analysis_run_comparisons: list[Mapping[str, Any]] | None = None,
    handoff_packages: list[Mapping[str, Any]] | None = None,
    export_workflows: list[Mapping[str, Any]] | None = None,
    external_prover_metadata: list[Mapping[str, Any]] | None = None,
    source_notes: list[str] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build deterministic report-section records from supplied envelopes."""

    diagnostics: list[dict[str, Any]] = []
    state_run_sections = _state_run_sections(
        model_states or [], analysis_runs or [], diagnostics
    )
    comparison_sections = _comparison_sections(
        model_state_comparisons or [], analysis_run_comparisons or [], diagnostics
    )
    handoff_sections = _handoff_sections(
        handoff_packages or [],
        export_workflows or [],
        external_prover_metadata or [],
        diagnostics,
    )

    record = {
        "schema_version": REPORT_SECTION_VERSION,
        "deliverable_id": "DEL-08-06",
        "package_id": "PKG-08",
        "scope_item": "SOW-024",
        "objectives": ["OBJ-007", "OBJ-016", "OBJ-017", "OBJ-018"],
        "section_set_id": str(section_set_id),
        "section_contract_status": "backend_report_section_records_only",
        "source_notes": sorted(str(item) for item in _list(source_notes)),
        "sections": {
            "state_run_sections": state_run_sections,
            "comparison_sections": comparison_sections,
            "handoff_sections": handoff_sections,
        },
        "sow_024_coverage": _sow_024_coverage(
            state_run_sections, comparison_sections, handoff_sections
        ),
        "limitations": _limitations(),
        "unresolved_tbds": _unresolved_tbds(
            state_run_sections, comparison_sections, handoff_sections
        ),
        "diagnostics": [],
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(dict(provenance or ENGINE_PROVENANCE)),
    }
    record["diagnostics"] = diagnostics_for_report_sections(record, diagnostics)
    return _sort_record(record)


def diagnostics_for_report_sections(
    record: Mapping[str, Any], existing: list[dict[str, Any]] | None = None
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for a report-section set."""

    diagnostics = list(existing or [])
    if record.get("deliverable_id") != "DEL-08-06":
        diagnostics.append(
            _diagnostic(
                "SCH-DELIVERABLE-MISMATCH",
                "blocking",
                "REPORT_SECTION_BOUNDARY",
                "Report-section set must use the DEL-08-06 envelope.",
                "Rebuild the section set with the DEL-08-06 assembler.",
                [_affected("ReportSectionSet", str(record.get("section_set_id", "unknown")))],
            )
        )
    diagnostics.extend(_professional_boundary_diagnostics(record.get("professional_boundary")))
    diagnostics.extend(_authority_term_diagnostics(record))
    diagnostics.extend(_numeric_unit_diagnostics(record))
    return sorted(_dedupe_diagnostics(diagnostics), key=canonical_json)


def canonical_json(value: Any) -> str:
    """Serialize report sections with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _state_run_sections(
    model_states: list[Mapping[str, Any]],
    analysis_runs: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    states = [_state_record(item) for item in model_states]
    runs = [_run_record(item) for item in analysis_runs]
    sections: list[dict[str, Any]] = []

    for state in states:
        state_ref = _ref("ModelState", _str(state.get("state_id"), "state:TBD"))
        _missing_required(state, REQUIRED_MODEL_STATE_FIELDS, state_ref, diagnostics)
        _boundary_flags_from_source(state.get("professional_boundary"), state_ref, diagnostics)
        _source_authority_claims(state, state_ref, diagnostics)
        sections.append(
            {
                "section_id": f"state-run:{state_ref['ref']}",
                "section_kind": "model_state_record",
                "state_ref": state_ref,
                "run_ref": None,
                "hash_refs": deepcopy(_list(state.get("hashes"))),
                "diagnostics": _safe_public_context(_list(state.get("warnings"))),
                "warnings": _safe_public_context(_list(state.get("warnings"))),
                "assumptions": _safe_public_context(_list(state.get("unresolved_assumptions"))),
                "analysis_status": sorted(str(item) for item in _list(state.get("analysis_status"))),
                "unit_context": deepcopy(state.get("unit_system_ref") or state.get("units_manifest")),
                "solver_context": None,
                "settings_ref": None,
                "source_provenance": deepcopy(state.get("provenance")),
                "privacy_classification": _privacy_classification(state),
                "review_state": _review_state(state),
                "limitations": _source_limitations(state, state_ref),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )

    for run in runs:
        run_ref = _ref("AnalysisRun", _str(run.get("run_id"), "run:TBD"))
        _missing_required(run, REQUIRED_ANALYSIS_RUN_FIELDS, run_ref, diagnostics)
        _boundary_flags_from_source(run.get("professional_boundary"), run_ref, diagnostics)
        _source_authority_claims(run, run_ref, diagnostics)
        sections.append(
            {
                "section_id": f"state-run:{run_ref['ref']}",
                "section_kind": "analysis_run_record",
                "state_ref": deepcopy(run.get("model_state_ref")),
                "run_ref": run_ref,
                "hash_refs": deepcopy(_list(run.get("hashes"))),
                "diagnostics": _safe_public_context(_list(run.get("diagnostics"))),
                "warnings": _safe_public_context(_list(run.get("warnings"))),
                "assumptions": _safe_public_context(_list(run.get("unresolved_assumptions"))),
                "analysis_status": sorted(str(item) for item in _list(run.get("analysis_status"))),
                "unit_context": deepcopy(run.get("unit_system_ref")),
                "solver_context": deepcopy(run.get("solver_version")),
                "settings_ref": deepcopy(run.get("settings_ref")),
                "load_basis_refs": deepcopy(_list(run.get("load_basis_refs"))),
                "result_refs": deepcopy(_list(run.get("result_refs"))),
                "rule_pack_refs": deepcopy(_safe_refs(run.get("rule_pack_refs"))),
                "library_refs": deepcopy(_safe_refs(run.get("library_refs"))),
                "source_provenance": deepcopy(run.get("provenance")),
                "privacy_classification": _privacy_classification(run),
                "review_state": _review_state(run),
                "reproducibility": deepcopy(run.get("reproducibility")),
                "limitations": _source_limitations(run, run_ref),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )

    return sorted(sections, key=canonical_json)


def _comparison_sections(
    model_state_comparisons: list[Mapping[str, Any]],
    analysis_run_comparisons: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    sections: list[dict[str, Any]] = []
    for item in model_state_comparisons:
        comparison_ref = _ref("ModelStateComparison", _comparison_id(item, "model-state-comparison:TBD"))
        _boundary_flags_from_source(item.get("professional_boundary"), comparison_ref, diagnostics)
        _source_authority_claims(item, comparison_ref, diagnostics)
        sections.append(
            {
                "section_id": f"comparison:{comparison_ref['ref']}",
                "section_kind": "model_state_comparison",
                "comparison_ref": comparison_ref,
                "left_ref": deepcopy(item.get("left_state_ref")),
                "right_ref": deepcopy(item.get("right_state_ref")),
                "summary": deepcopy(item.get("summary")),
                "manual_mappings": _manual_mappings(_list(item.get("entities"))),
                "unmatched_classifications": _unmatched_classifications(_list(item.get("entities"))),
                "tolerance_profile_refs": [],
                "unit_normalized_deltas": [],
                "settings": deepcopy(item.get("settings")),
                "diagnostics": _safe_public_context(_list(item.get("diagnostics"))),
                "source_provenance": deepcopy(item.get("provenance")),
                "privacy_classification": _privacy_classification(item),
                "review_state": _review_state(item),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )

    for item in analysis_run_comparisons:
        comparison_ref = _ref("AnalysisRunComparison", _comparison_id(item, "analysis-run-comparison:TBD"))
        _boundary_flags_from_source(item.get("professional_boundary"), comparison_ref, diagnostics)
        _source_authority_claims(item, comparison_ref, diagnostics)
        deltas = deepcopy(_list(item.get("result_deltas")))
        for delta in deltas:
            _check_delta_units(delta, comparison_ref, diagnostics)
        sections.append(
            {
                "section_id": f"comparison:{comparison_ref['ref']}",
                "section_kind": "analysis_run_comparison",
                "comparison_ref": comparison_ref,
                "run_context": deepcopy(item.get("run_context")),
                "manual_mappings": _manual_mappings(deltas),
                "unmatched_classifications": _unmatched_classifications(_list(item.get("diagnostics"))),
                "tolerance_profile_refs": sorted(
                    {
                        str(delta.get("tolerance_profile_ref"))
                        for delta in deltas
                        if delta.get("tolerance_profile_ref")
                    }
                ),
                "unit_normalized_deltas": deltas,
                "settings_deltas": deepcopy(_list(item.get("settings_deltas"))),
                "diagnostics": _safe_public_context(_list(item.get("diagnostics"))),
                "source_provenance": deepcopy(item.get("provenance")),
                "privacy_classification": _privacy_classification(item),
                "review_state": _review_state(item),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )
    return sorted(sections, key=canonical_json)


def _handoff_sections(
    handoff_packages: list[Mapping[str, Any]],
    export_workflows: list[Mapping[str, Any]],
    external_prover_metadata: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    sections: list[dict[str, Any]] = []
    for package in handoff_packages:
        manifest = package.get("handoff_package_manifest", {})
        package_id = _handoff_id(manifest)
        package_ref = _ref("HandoffPackage", package_id)
        _boundary_flags_from_source(manifest.get("professional_boundary"), package_ref, diagnostics)
        _source_authority_claims(manifest, package_ref, diagnostics)
        sections.append(
            {
                "section_id": f"handoff:{package_id}",
                "section_kind": "handoff_package_manifest",
                "handoff_package_ref": package_ref,
                "model_hash": deepcopy(manifest.get("model_hash")),
                "units_manifest": deepcopy(manifest.get("units_manifest")),
                "entity_ids": deepcopy(manifest.get("entity_ids")),
                "library_refs": deepcopy(_safe_refs(manifest.get("library_refs"))),
                "rule_pack_refs": deepcopy(_safe_refs(manifest.get("rule_pack_refs"))),
                "target_mapping_metadata": deepcopy(manifest.get("target_mapping_metadata")),
                "unsupported_target_flags": _safe_public_context(_list(manifest.get("unsupported_behavior_flags"))),
                "unresolved_assumptions": _safe_public_context(_list(manifest.get("unresolved_assumptions"))),
                "warnings": _safe_public_context(_list(manifest.get("warnings"))),
                "diagnostics": _safe_public_context(_list(manifest.get("diagnostics"))),
                "privacy": deepcopy(manifest.get("privacy")),
                "source_provenance": deepcopy(manifest.get("provenance")),
                "privacy_classification": _privacy_classification(manifest),
                "review_state": _review_state(manifest),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )

    for workflow in export_workflows:
        workflow_ref = _ref("ExportWorkflow", _str(workflow.get("export_workflow_id"), "export:TBD"))
        _boundary_flags_from_source(workflow.get("professional_boundary"), workflow_ref, diagnostics)
        _source_authority_claims(workflow, workflow_ref, diagnostics)
        export_payload = workflow.get("export_payload", {})
        sections.append(
            {
                "section_id": f"handoff:{workflow_ref['ref']}",
                "section_kind": "export_workflow_record",
                "export_workflow_ref": workflow_ref,
                "handoff_package_ref": _workflow_package_ref(workflow),
                "model_hash": deepcopy(export_payload.get("model_hash")),
                "units_manifest": deepcopy(export_payload.get("units_manifest")),
                "entity_ids": deepcopy(export_payload.get("entity_ids")),
                "target_mapping_metadata": deepcopy(export_payload.get("target_mapping_metadata")),
                "unsupported_target_records": _safe_public_context(_list(export_payload.get("unsupported_target_records"))),
                "unresolved_assumptions": _safe_public_context(_list(export_payload.get("unresolved_assumptions"))),
                "warnings": _safe_public_context(_list(export_payload.get("warnings"))),
                "diagnostics": _safe_public_context(_list(workflow.get("diagnostics"))),
                "privacy": deepcopy(workflow.get("privacy")),
                "source_provenance": deepcopy(workflow.get("provenance")),
                "privacy_classification": _privacy_classification(workflow),
                "review_state": _review_state(workflow),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )

    for metadata in external_prover_metadata:
        metadata_ref = _ref("ExternalProverMetadata", _str(metadata.get("metadata_record_id"), "external-prover:TBD"))
        _boundary_flags_from_source(metadata.get("professional_boundary"), metadata_ref, diagnostics)
        _source_authority_claims(metadata, metadata_ref, diagnostics)
        sections.append(
            {
                "section_id": f"handoff:{metadata_ref['ref']}",
                "section_kind": "external_prover_metadata",
                "external_prover_metadata_ref": metadata_ref,
                "handoff_package_refs": deepcopy(_list(metadata.get("handoff_package_refs"))),
                "target_mapping_refs": deepcopy(_list(metadata.get("target_mapping_refs"))),
                "export_workflow_refs": deepcopy(_list(metadata.get("export_workflow_refs"))),
                "immutable_model_state_refs": deepcopy(_list(metadata.get("immutable_model_state_refs"))),
                "unsupported_target_flags": _safe_public_context(_list(metadata.get("unsupported_target_flags"))),
                "unresolved_assumptions": _safe_public_context(_list(metadata.get("assumptions"))),
                "warnings": _safe_public_context(_list(metadata.get("warnings"))),
                "diagnostics": _safe_public_context(_list(metadata.get("diagnostics"))),
                "privacy_classification": _privacy_classification(metadata),
                "review_state": _review_state(metadata),
                "source_provenance": deepcopy(metadata.get("provenance")),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            }
        )
    return sorted(sections, key=canonical_json)


def _sow_024_coverage(state_sections: list[dict[str, Any]], comparison_sections: list[dict[str, Any]], handoff_sections: list[dict[str, Any]]) -> dict[str, str]:
    all_sections = state_sections + comparison_sections + handoff_sections
    return {
        "inputs": "represented_by_state_run_and_handoff_references" if state_sections or handoff_sections else "TBD",
        "sources": "represented_by_source_provenance_and_source_notes" if all_sections else "TBD",
        "warnings": "represented_by_warning_and_diagnostic_lists" if any(item.get("warnings") or item.get("diagnostics") for item in all_sections) else "TBD",
        "assumptions": "represented_by_assumption_lists" if any(item.get("assumptions") or item.get("unresolved_assumptions") for item in all_sections) else "TBD",
        "results": "represented_by_result_refs_and_unit_normalized_deltas" if state_sections or comparison_sections else "TBD",
        "rule_pack_checksums": "represented_by_rule_pack_refs_without_private_payloads" if any(item.get("rule_pack_refs") for item in all_sections) else "TBD",
        "limitations": "represented_by_limitations_and_unresolved_tbds",
    }


def _limitations() -> list[dict[str, Any]]:
    return [
        {
            "limitation_id": "DEL-08-06-LIMIT-BACKEND-SECTIONS-ONLY",
            "statement": "Report sections are backend records for human review; final layout, transport, solver execution, and external-tool execution are out of scope.",
            "human_review_required": True,
        }
    ]


def _unresolved_tbds(*groups: list[dict[str, Any]]) -> list[dict[str, Any]]:
    tbds = [
        {
            "tbd_id": "DEL-08-06-TBD-FINAL-LAYOUT",
            "description": "Final report styling, layout, transport, and release thresholds remain outside this deliverable.",
            "review_needed": True,
        }
    ]
    for section in [item for group in groups for item in group]:
        for key, value in section.items():
            if value is None:
                tbds.append(
                    {
                        "tbd_id": f"DEL-08-06-TBD-{section['section_id']}-{key}",
                        "description": f"Source value for {key} is unavailable in section {section['section_id']}.",
                        "review_needed": True,
                    }
                )
    return sorted(tbds, key=canonical_json)


def _missing_required(
    record: Mapping[str, Any],
    fields: tuple[str, ...],
    affected_ref: Mapping[str, str],
    diagnostics: list[dict[str, Any]],
) -> None:
    for field in fields:
        if _is_missing(record.get(field)):
            diagnostics.append(
                _diagnostic(
                    "SCH-SOURCE-VALUE-MISSING",
                    "warning",
                    "MISSING_SOURCE_VALUE",
                    f"Source field is missing for report section assembly: {field}.",
                    "Carry the field as an explicit report finding or unresolved TBD until source evidence is available.",
                    [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("Field", field)],
                )
            )


def _boundary_flags_from_source(
    boundary: Any, affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> None:
    if not isinstance(boundary, Mapping):
        diagnostics.append(
            _diagnostic(
                "SCH-PROFESSIONAL-BOUNDARY-MISSING",
                "warning",
                "PROFESSIONAL_BOUNDARY",
                "Source record lacks explicit professional-boundary metadata.",
                "Provide source boundary metadata or keep human-review limitation visible.",
                [_affected(affected_ref["object_type"], affected_ref["ref"])],
            )
        )
        return
    for flag in AUTHORITY_FLAG_NAMES:
        if boundary.get(flag) is True:
            diagnostics.append(
                _diagnostic(
                    "SCH-SOFTWARE-AUTHORITY-FLAG-BLOCKED",
                    "blocking",
                    "PROFESSIONAL_BOUNDARY",
                    "Source boundary metadata attempted to set a software authority flag.",
                    "Keep the source as a diagnostic finding and require external human review.",
                    [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("BoundaryFlag", flag)],
                )
            )


def _professional_boundary_diagnostics(boundary: Any) -> list[dict[str, Any]]:
    if not isinstance(boundary, Mapping):
        return [
            _diagnostic(
                "SCH-PROFESSIONAL-BOUNDARY-MISSING",
                "blocking",
                "PROFESSIONAL_BOUNDARY",
                "Section set lacks explicit professional-boundary metadata.",
                "Use the project default non-authoritative boundary.",
                [_affected("ReportSectionSet", "professional_boundary")],
            )
        ]
    diagnostics: list[dict[str, Any]] = []
    if boundary.get("human_review_required") is not True:
        diagnostics.append(
            _diagnostic(
                "SCH-HUMAN-REVIEW-NOT-REQUIRED-BLOCKED",
                "blocking",
                "PROFESSIONAL_BOUNDARY",
                "Section set must require human review.",
                "Set human_review_required to true.",
                [_affected("ReportSectionSet", "professional_boundary")],
            )
        )
    for flag in AUTHORITY_FLAG_NAMES:
        if boundary.get(flag) is True:
            diagnostics.append(
                _diagnostic(
                    "SCH-SOFTWARE-AUTHORITY-FLAG-BLOCKED",
                    "blocking",
                    "PROFESSIONAL_BOUNDARY",
                    "Section set attempted to set a software authority flag.",
                    "Keep all software authority flags false.",
                    [_affected("BoundaryFlag", flag)],
                )
            )
    return diagnostics


def _authority_term_diagnostics(record: Any) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for path, value in _walk_strings(record):
        lower = value.lower()
        if path.endswith("message") or "claim" in path or "status" in path or "boundary" in path:
            if any(term in lower for term in PROHIBITED_AUTHORITY_TERMS):
                diagnostics.append(
                    _diagnostic(
                        "SCH-AUTHORITY-CLAIM-REJECTED",
                        "blocking",
                        "PROFESSIONAL_BOUNDARY",
                        "Report-section source text contains a prohibited authority or reliance claim.",
                        "Replace the source claim with non-authoritative review context.",
                        [_affected("FieldPath", path)],
                    )
                )
    return diagnostics


def _source_authority_claims(
    source: Any, affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> None:
    for path, value in _walk_strings(source):
        lower = value.lower()
        if any(term in lower for term in PROHIBITED_AUTHORITY_TERMS):
            diagnostics.append(
                _diagnostic(
                    "SCH-AUTHORITY-CLAIM-REJECTED",
                    "blocking",
                    "PROFESSIONAL_BOUNDARY",
                    "Report-section source text contains a prohibited authority or reliance claim.",
                    "Replace the source claim with non-authoritative review context.",
                    [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("FieldPath", path)],
                )
            )


def _safe_public_context(value: Any) -> Any:
    if isinstance(value, str):
        lower = value.lower()
        if any(term in lower for term in PROHIBITED_AUTHORITY_TERMS):
            return "[omitted_prohibited_authority_or_reliance_claim]"
        return value
    if isinstance(value, Mapping):
        return {str(key): _safe_public_context(item) for key, item in value.items()}
    if isinstance(value, list):
        return [_safe_public_context(item) for item in value]
    return deepcopy(value)


def _numeric_unit_diagnostics(record: Any) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for path, container in _walk_dicts(record):
        if "magnitude" in container:
            if not isfinite(container.get("magnitude")) if isinstance(container.get("magnitude"), float) else not isinstance(container.get("magnitude"), (int, float)):
                diagnostics.append(_unit_diagnostic(path, "Numeric magnitude is not finite."))
            if _is_missing(container.get("unit")) or _is_missing(container.get("dimension")):
                diagnostics.append(_unit_diagnostic(path, "Numeric value lacks unit or dimension metadata."))
    return diagnostics


def _unit_diagnostic(path: str, message: str) -> dict[str, Any]:
    return _diagnostic(
        "SCH-NUMERIC-UNIT-METADATA-MISSING",
        "blocking",
        "UNIT_METADATA",
        message,
        "Provide unit and dimension metadata or carry the value only as a source diagnostic.",
        [_affected("FieldPath", path)],
    )


def _check_delta_units(
    delta: Mapping[str, Any], affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> None:
    if _is_missing(delta.get("normalized_unit")) or _is_missing(delta.get("dimension")):
        diagnostics.append(
            _diagnostic(
                "SCH-COMPARISON-DELTA-UNIT-MISSING",
                "blocking",
                "UNIT_METADATA",
                "Comparison delta lacks normalized unit or dimension metadata.",
                "Provide normalized unit and dimension before reporting numeric deltas.",
                [_affected(affected_ref["object_type"], affected_ref["ref"])],
            )
        )


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
        "message": message,
        "remediation": remediation,
        "affected_refs": affected_refs,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _affected(object_type: str, ref_value: str) -> dict[str, str]:
    return {"object_type": str(object_type), "ref": str(ref_value)}


def _ref(object_type: str, ref_value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref_value}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _state_record(item: Mapping[str, Any]) -> Mapping[str, Any]:
    record = item.get("model_state")
    return record if isinstance(record, Mapping) else item


def _run_record(item: Mapping[str, Any]) -> Mapping[str, Any]:
    record = item.get("analysis_run")
    return record if isinstance(record, Mapping) else item


def _comparison_id(item: Mapping[str, Any], fallback: str) -> str:
    for key in ("comparison_id", "comparison_ref", "comparison_kind"):
        if item.get(key):
            value = item[key]
            if isinstance(value, Mapping):
                return _str(value.get("ref") or value.get("ref_id"), fallback)
            return str(value)
    return fallback


def _handoff_id(manifest: Any) -> str:
    if isinstance(manifest, Mapping):
        identity = manifest.get("package_identity")
        if isinstance(identity, Mapping):
            return _str(identity.get("handoff_package_id"), "handoff:TBD")
    return "handoff:TBD"


def _workflow_package_ref(workflow: Mapping[str, Any]) -> dict[str, str]:
    package = workflow.get("source_handoff_package")
    if isinstance(package, Mapping):
        return _ref("HandoffPackage", _handoff_id(package.get("handoff_package_manifest", {})))
    return _ref("HandoffPackage", "handoff:TBD")


def _safe_refs(value: Any) -> list[Any]:
    refs = []
    for item in _list(value):
        if isinstance(item, Mapping):
            redacted = deepcopy(dict(item))
            for private_key in ("payload", "content", "private_payload", "protected_payload"):
                redacted.pop(private_key, None)
            refs.append(redacted)
        else:
            refs.append(deepcopy(item))
    return sorted(refs, key=canonical_json)


def _manual_mappings(items: list[Any]) -> list[Any]:
    return sorted(
        [
            deepcopy(item)
            for item in items
            if isinstance(item, Mapping)
            and str(item.get("match_basis") or item.get("mapping_status") or "").startswith("manual")
        ],
        key=canonical_json,
    )


def _unmatched_classifications(items: list[Any]) -> list[Any]:
    unmatched = []
    for item in items:
        if isinstance(item, Mapping):
            text = canonical_json(item).lower()
            if "unmatched" in text or "missing" in text or "unresolved" in text:
                unmatched.append(deepcopy(item))
    return sorted(unmatched, key=canonical_json)


def _privacy_classification(record: Mapping[str, Any]) -> str:
    for key in ("privacy_classification", "review_classification"):
        if record.get(key):
            return str(record[key])
    privacy = record.get("privacy")
    if isinstance(privacy, Mapping) and privacy.get("classification"):
        return str(privacy["classification"])
    provenance = record.get("provenance")
    if isinstance(provenance, Mapping) and provenance.get("privacy_classification"):
        return str(provenance["privacy_classification"])
    return "TBD"


def _review_state(record: Mapping[str, Any]) -> str:
    for key in ("review_status", "review_classification"):
        if record.get(key):
            return str(record[key])
    provenance = record.get("provenance")
    if isinstance(provenance, Mapping):
        return str(provenance.get("review_status") or provenance.get("review_classification") or "TBD")
    return "TBD"


def _source_limitations(record: Mapping[str, Any], affected_ref: Mapping[str, str]) -> list[dict[str, Any]]:
    limitations = []
    if _privacy_classification(record) == "TBD":
        limitations.append(
            {
                "limitation_id": f"privacy:{affected_ref['ref']}",
                "statement": "Source privacy classification is unavailable.",
                "affected_ref": deepcopy(dict(affected_ref)),
                "human_review_required": True,
            }
        )
    if _review_state(record) == "TBD":
        limitations.append(
            {
                "limitation_id": f"review:{affected_ref['ref']}",
                "statement": "Source review state is unavailable.",
                "affected_ref": deepcopy(dict(affected_ref)),
                "human_review_required": True,
            }
        )
    return sorted(limitations, key=canonical_json)


def _is_missing(value: Any) -> bool:
    return value is None or value == "" or value == [] or value == {} or value == "TBD"


def _str(value: Any, fallback: str) -> str:
    return fallback if _is_missing(value) else str(value)


def _walk_strings(value: Any, path: str = "$"):
    if isinstance(value, str):
        yield path, value
    elif isinstance(value, Mapping):
        for key, item in value.items():
            yield from _walk_strings(item, f"{path}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from _walk_strings(item, f"{path}[{index}]")


def _walk_dicts(value: Any, path: str = "$"):
    if isinstance(value, Mapping):
        yield path, value
        for key, item in value.items():
            yield from _walk_dicts(item, f"{path}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from _walk_dicts(item, f"{path}[{index}]")


def _dedupe_diagnostics(diagnostics: list[dict[str, Any]]) -> list[dict[str, Any]]:
    seen = set()
    unique = []
    for item in diagnostics:
        key = canonical_json(item)
        if key not in seen:
            seen.add(key)
            unique.append(item)
    return unique


def _sort_record(record: dict[str, Any]) -> dict[str, Any]:
    return json.loads(canonical_json(record))
