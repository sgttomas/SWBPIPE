"""Report-section assembly for DEL-08-06.

The assembler consumes already-formed model-state, analysis-run, comparison,
handoff, export-workflow, and external-prover metadata records. It does not
read project files, execute solvers, render reports, invoke external tools, or
create professional/code-compliance determinations.
"""

from __future__ import annotations

from copy import deepcopy
import json
import re
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

AUTHORITY_FLAG_NAMES = {
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_makes_authentication_claim",
    "software_creates_professional_reliance_record",
    "software_creates_external_validation_record",
}

PERMITTED_SOFTWARE_ANALYSIS_STATUSES = {
    "MODEL_INCOMPLETE",
    "MECHANICS_SOLVED",
    "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED",
    "USER_RULE_FAILED",
    "HUMAN_REVIEW_REQUIRED",
}

# DEL-08-06 AC-005: one named vocabulary per status-bearing member the software
# writes. A value outside its member's vocabulary produces a blocking diagnostic and is
# not carried into the set; the value itself is carried nowhere. No member is matched
# by substring: identifiers, field paths, stored values, the user's or a document
# author's text and restated solver text are carried as what they are.
STATUS_VOCABULARIES: dict[str, frozenset[str]] = {
    # The software's analysis statuses (DEL-05-04).
    "software_analysis_statuses": frozenset(PERMITTED_SOFTWARE_ANALYSIS_STATUSES),
    # schemas/model.schema.yaml Diagnostic.severity, which the product's producers write.
    "diagnostic_severities": frozenset({"info", "warning", "error", "blocking"}),
    # core/comparison/model_state/engine.py summary classes; stateComparison.ts ROW_CLASSIFICATIONS.
    "state_row_classifications": frozenset(
        {"added", "removed", "changed", "unchanged", "mapped_changed", "mapped_unchanged", "unresolved"}
    ),
    # Both comparison engines and the desktop records (comparisonRecords.ts).
    "match_bases": frozenset({"stable_id", "explicit_mapping", "manual_mapping", "unmatched", "stable_result_identity"}),
    # stateComparison.ts FieldChangeKind.
    "field_change_kinds": frozenset({"added", "removed", "changed", "unit_changed"}),
    # runComparison.ts NOT_COMPARED_REASONS.
    "not_compared_reasons": frozenset(
        {
            "semantic_identity_differs",
            "left_run_needs_recompute",
            "right_run_needs_recompute",
            "family_not_compared",
            "units_differ",
            "value_not_numeric",
            "dimension_missing",
        }
    ),
    # runComparison.ts UNMATCHED_REASONS.
    "unmatched_reasons": frozenset({"no_row_with_this_result_id", "identity_differs", "duplicate_result_id", "result_id_missing"}),
    "run_sides": frozenset({"left", "right"}),
    # comparisonRecords.ts runSideRef.
    "run_side_kinds": frozenset({"recorded_run", "current_session_run"}),
    # resultReader.ts ReaderStanding.status.
    "standing_statuses": frozenset({"integrity_checked", "needs_recompute"}),
    # core/comparison/analysis_run/engine.py _classify_delta, and the desktop's record.
    "delta_classifications": frozenset(
        {"not_classified", "within_tolerance_profile", "exceeds_tolerance_profile", "not_tolerance_checked"}
    ),
    # The analysis_run engine's bases, the desktop's record, and the desktop's within-run
    # comparison (previewService.ts buildPreviewComparison).
    "delta_classification_bases": frozenset(
        {
            "governed_tolerance_profile_TBD_no_default_engineering_threshold",
            "no_tolerance_profile_supplied",
            "no_matching_tolerance_rule",
            "tolerance_rule_unit_not_normalized_unit",
            "caller_supplied_relative_absolute_tolerance_rule",
            "tolerance_value_not_numeric",
            "caller_supplied_tolerance_rule",
            "no_tolerance_profile",
        }
    ),
    # schemas/comparison_mapping.schema.json MappingStatus.
    "mapping_statuses": frozenset(
        {"automatic_match", "manual_match", "unresolved_mapping", "unmatched_left", "unmatched_right", "ignored", "TBD"}
    ),
    "settings_delta_kinds": frozenset({"added", "removed", "changed"}),
    # runComparison.ts DiagnosticChange.change.
    "diagnostic_changes": frozenset({"added", "removed"}),
    # core/handoff/target_mapping/contract.py SUPPORTED_BEHAVIOR_STATUSES (the schema's enum).
    "supported_behavior_statuses": frozenset({"unsupported", "approximate", "requires_human_review", "not_implemented", "TBD"}),
}

# The status-bearing members of each record kind the software writes, by path:
# `a.b` is a member, `a[]` each item of a list. Members a person or a document author
# writes (labels, notes, assumptions, provenance declarations, an external prover's
# flags) are not on these lists: they are carried as the author's text.
STATUS_MEMBERS: dict[str, tuple[tuple[str, str], ...]] = {
    # The native store's model-state record: its status and warnings are the validator's.
    "model_state": (
        ("analysis_status[]", "software_analysis_statuses"),
        ("warnings[].severity", "diagnostic_severities"),
    ),
    "analysis_run": (
        ("analysis_status[]", "software_analysis_statuses"),
        ("diagnostics[].severity", "diagnostic_severities"),
        ("warnings[].severity", "diagnostic_severities"),
    ),
    "model_state_comparison": (
        ("entities[].classification", "state_row_classifications"),
        ("entities[].match_basis", "match_bases"),
        ("entities[].changes[].change_kind", "field_change_kinds"),
        ("diagnostics[].severity", "diagnostic_severities"),
    ),
    "analysis_run_comparison": (
        ("run_context.left.side_kind", "run_side_kinds"),
        ("run_context.right.side_kind", "run_side_kinds"),
        ("run_context.left.standing.status", "standing_statuses"),
        ("run_context.right.standing.status", "standing_statuses"),
        ("run_context.run_gate[]", "not_compared_reasons"),
        ("run_context.not_compared[].reasons[]", "not_compared_reasons"),
        ("run_context.unmatched[].reason", "unmatched_reasons"),
        ("run_context.unmatched[].side", "run_sides"),
        ("result_deltas[].classification", "delta_classifications"),
        ("result_deltas[].classification_basis", "delta_classification_bases"),
        ("result_deltas[].match_basis", "match_bases"),
        ("result_deltas[].mapping_status", "mapping_statuses"),
        ("settings_deltas[].delta_kind", "settings_delta_kinds"),
        ("diagnostics[].severity", "diagnostic_severities"),
        ("diagnostics[].change", "diagnostic_changes"),
    ),
    "handoff_package_manifest": (
        ("unsupported_behavior_flags[].status", "supported_behavior_statuses"),
        ("diagnostics[].severity", "diagnostic_severities"),
        ("warnings[].severity", "diagnostic_severities"),
    ),
    "export_workflow": (
        ("export_payload.unsupported_target_records[].status", "supported_behavior_statuses"),
        ("export_payload.warnings[].severity", "diagnostic_severities"),
        ("diagnostics[].severity", "diagnostic_severities"),
    ),
    "external_prover_metadata": (
        ("diagnostics[].severity", "diagnostic_severities"),
    ),
}

# DEL-08-06 AC-003 by structure: external-prover metadata has no status member in its
# contract (DEL-15-04), so the presence of any of these, whatever its value, is a finding.
EXTERNAL_PROVER_STATUS_MEMBERS = ("status", "execution_status", "validation_status", "verification_status", "acceptance_status")

# Whose text each text-bearing member of a section is (DEL-08-06 AC-005).
TEXT_ORIGINS: dict[str, dict[str, str]] = {
    "model_state_record": {
        "assumptions": "user", "diagnostics": "software", "warnings": "software", "limitations": "source_record",
        "source_provenance": "source_declaration", "privacy_classification": "source_declaration", "review_state": "source_declaration",
    },
    "current_model_state": {
        "authored_analysis_status": "document_author", "diagnostics": "document_author", "warnings": "document_author",
        "source_provenance": "software", "privacy_classification": "software", "review_state": "software",
    },
    "analysis_run_record": {
        "assumptions": "user", "diagnostics": "solver", "warnings": "solver", "limitations": "source_record",
        "solver_context": "solver", "source_provenance": "source_declaration", "privacy_classification": "source_declaration",
        "review_state": "source_declaration",
    },
    "model_state_comparison": {
        "changed_entities": "model_values", "diagnostics": "comparison", "settings": "comparison",
        "source_provenance": "source_declaration", "privacy_classification": "source_declaration", "review_state": "source_declaration",
    },
    "analysis_run_comparison": {
        "largest_differences": "solver_results", "settings_deltas": "solver_settings", "diagnostics": "solver",
        "run_context": "comparison", "pairs_not_compared_listed": "comparison", "rows_unmatched_listed": "comparison",
        "source_provenance": "source_declaration", "privacy_classification": "source_declaration", "review_state": "source_declaration",
    },
    "handoff_package_manifest": {
        "unsupported_target_flags": "software", "unresolved_assumptions": "user", "warnings": "software", "diagnostics": "software",
        "target_mapping_metadata": "software", "source_provenance": "source_declaration", "privacy_classification": "source_declaration",
        "review_state": "source_declaration",
    },
    "export_workflow_record": {
        "unsupported_target_records": "software", "unresolved_assumptions": "user", "warnings": "software", "diagnostics": "software",
        "target_mapping_metadata": "software", "source_provenance": "source_declaration", "privacy_classification": "source_declaration",
        "review_state": "source_declaration",
    },
    "external_prover_metadata": {
        "unsupported_target_flags": "user", "unresolved_assumptions": "user", "warnings": "user", "diagnostics": "software",
        "source_provenance": "source_declaration", "privacy_classification": "source_declaration", "review_state": "source_declaration",
    },
}

PRIVATE_PAYLOAD_KEYS = {
    "payload",
    "content",
    "formula",
    "formulas",
    "private_payload",
    "protected_payload",
    "private_rule_payload",
    "private_library_payload",
    "secret",
    "secrets",
}

NUMERIC_VALUE_KEYS = {
    "magnitude",
    "left_magnitude",
    "right_magnitude",
    "left_normalized_magnitude",
    "right_normalized_magnitude",
    "normalized_delta",
    "absolute_normalized_delta",
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

REQUIRED_HANDOFF_MANIFEST_FIELDS = (
    "package_identity",
    "model_hash",
    "units_manifest",
    "entity_ids",
    "target_mapping_metadata",
    "unsupported_behavior_flags",
    "privacy",
    "professional_boundary",
    "provenance",
)

REQUIRED_EXPORT_WORKFLOW_FIELDS = (
    "export_workflow_id",
    "export_payload",
    "diagnostics",
    "privacy",
    "professional_boundary",
    "provenance",
)

REQUIRED_EXTERNAL_PROVER_METADATA_FIELDS = (
    "metadata_record_id",
    "handoff_package_refs",
    "target_mapping_refs",
    "unsupported_target_flags",
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


SOFTWARE_STANDING_NOT_DERIVED = (
    "The software does not derive a standing for the current model state when the report "
    "package is assembled; the analysis status labels beside it are the document author's."
)

DEC018_UNIT_CONTEXT = {"ref_type": "unit_system", "ref_id": "unit-system:dec-018-si-dual-display"}


def build_current_model_state_section(record: Mapping[str, Any]) -> dict[str, Any]:
    """The section of the model state a desktop report package is built for: the model
    as it stands, named by the model reference and its canonical hash.

    `record` holds `model_ref`, `model_hash`, `authored_analysis_status` (the document's
    labels), `document_diagnostics` and `provenance`. The document's labels and
    diagnostics are its author's: carried as they are, unfiltered and unjudged. The
    software's own statuses are stated beside them; no standing of the state is derived
    at assembly, and the section says so (`software_standing`).
    """

    state_ref = f"state:{record.get('model_ref')}:preview"
    reference = _ref("ModelState", state_ref)
    diagnostics = deepcopy(_list(record.get("document_diagnostics")))
    return _sort_record(
        {
            "section_id": f"state-run:{state_ref}",
            "section_kind": "model_state_record",
            "state_ref": reference,
            "run_ref": None,
            "hash_refs": [
                {
                    "algorithm": "sha256",
                    "canonicalization": "rfc8785_jcs",
                    "payload_ref": deepcopy(reference),
                    "payload_scope": "model_state",
                    "value": deepcopy(record.get("model_hash")),
                }
            ],
            "diagnostics": diagnostics,
            "warnings": deepcopy(diagnostics),
            "assumptions": [],
            "analysis_status": ["HUMAN_REVIEW_REQUIRED"],
            "authored_analysis_status": deepcopy(record.get("authored_analysis_status")),
            "software_standing": {"derived": False, "statement": SOFTWARE_STANDING_NOT_DERIVED},
            "unit_context": deepcopy(DEC018_UNIT_CONTEXT),
            "solver_context": None,
            "settings_ref": None,
            "source_provenance": deepcopy(record.get("provenance")),
            "privacy_classification": "private_project_data",
            "review_state": "pending",
            "limitations": [],
            "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
            "text_origin": deepcopy(TEXT_ORIGINS["current_model_state"]),
        }
    )


def build_persisted_project_report_sections(
    *,
    section_set_id: str,
    persistence_envelope: Mapping[str, Any],
    source_notes: list[str] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build report sections from canonical project run-history records.

    This bounded application-service handoff reads embedded model-state and
    analysis-run records only. It performs no persistence or solver mutation,
    infers no human approval or code compliance, and leaves external/non-JSON
    payload partitioning under the source persistence contract.
    """

    project = persistence_envelope.get("project", {})
    if not isinstance(project, Mapping):
        project = {}
    run_history = project.get("run_history", {})
    if not isinstance(run_history, Mapping):
        run_history = {}
    notes = list(source_notes or [])
    notes.append(
        "Bounded application-service handoff from canonical project.run_history; external payload partitioning unchanged."
    )
    record = build_state_comparison_handoff_report_sections(
        section_set_id=section_set_id,
        model_states=_mapping_list(run_history.get("model_state_records")),
        analysis_runs=_mapping_list(run_history.get("analysis_run_records")),
        source_notes=notes,
        provenance=provenance,
    )
    record["application_service_binding"] = {
        "source": "project.run_history",
        "workflow_grain": "persisted_run_history_to_report_sections",
        "canonical_records_only": True,
        "project_mutated": False,
        "solver_executed": False,
        "human_approval_inferred": False,
        "code_compliance_inferred": False,
        "external_payload_partitioning": "unchanged_source_contract",
    }
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
        authored_status = state.get("analysis_status") if isinstance(state.get("analysis_status"), Mapping) else None
        state = _checked_record(state, "model_state", state_ref, diagnostics)
        section = {
                "section_id": f"state-run:{state_ref['ref']}",
                "section_kind": "model_state_record",
                "state_ref": state_ref,
                "run_ref": None,
                "hash_refs": deepcopy(_list(state.get("hashes"))),
                "diagnostics": _safe_public_context(_list(state.get("warnings"))),
                "warnings": _safe_public_context(_list(state.get("warnings"))),
                "assumptions": _safe_public_context(_list(state.get("unresolved_assumptions"))),
                "analysis_status": _safe_analysis_statuses(state.get("analysis_status")),
                "unit_context": deepcopy(state.get("unit_system_ref") or state.get("units_manifest")),
                "solver_context": None,
                "settings_ref": None,
                "source_provenance": deepcopy(state.get("provenance")),
                "privacy_classification": _privacy_classification(state),
                "review_state": _review_state(state),
                "limitations": _source_limitations(state, state_ref),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
                "text_origin": deepcopy(TEXT_ORIGINS["model_state_record"]),
            }
        # A document's authored analysis-status labels, where a record carries them, are
        # the author's: carried as such and never read as the software's statuses.
        if authored_status is not None:
            section["authored_analysis_status"] = deepcopy(dict(authored_status))
        sections.append(section)

    for run in runs:
        run_ref = _ref("AnalysisRun", _str(run.get("run_id"), "run:TBD"))
        _missing_required(run, REQUIRED_ANALYSIS_RUN_FIELDS, run_ref, diagnostics)
        _boundary_flags_from_source(run.get("professional_boundary"), run_ref, diagnostics)
        run = _checked_record(run, "analysis_run", run_ref, diagnostics)
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
                "analysis_status": _safe_analysis_statuses(run.get("analysis_status")),
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
                "text_origin": deepcopy(TEXT_ORIGINS["analysis_run_record"]),
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
        item = _checked_record(item, "model_state_comparison", comparison_ref, diagnostics)
        sections.append(
            {
                "section_id": f"comparison:{comparison_ref['ref']}",
                "section_kind": "model_state_comparison",
                "comparison_ref": comparison_ref,
                "left_ref": deepcopy(item.get("left_state_ref")),
                "right_ref": deepcopy(item.get("right_state_ref")),
                "summary": deepcopy(item.get("summary")),
                "changed_entities": _changed_entities(_list(item.get("entities"))),
                "hash_refs": _hash_refs(item.get("hashes")),
                "manual_mappings": _manual_mappings(_list(item.get("entities"))),
                "unmatched_classifications": _unmatched_classifications(_list(item.get("entities"))),
                "tolerance_profile_refs": [],
                "unit_normalized_deltas": [],
                "settings": deepcopy(item.get("settings")),
                "csv": _csv_statement(item.get("csv")),
                "diagnostics": _safe_public_context(_list(item.get("diagnostics"))),
                "source_provenance": deepcopy(item.get("provenance")),
                "privacy_classification": _privacy_classification(item),
                "review_state": _review_state(item),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
                "text_origin": deepcopy(TEXT_ORIGINS["model_state_comparison"]),
            }
        )

    for item in analysis_run_comparisons:
        comparison_ref = _ref("AnalysisRunComparison", _comparison_id(item, "analysis-run-comparison:TBD"))
        _boundary_flags_from_source(item.get("professional_boundary"), comparison_ref, diagnostics)
        item = _checked_record(item, "analysis_run_comparison", comparison_ref, diagnostics)
        deltas = deepcopy(_list(item.get("result_deltas")))
        for delta in deltas:
            _check_delta_units(delta, comparison_ref, diagnostics)
        sections.append(
            {
                "section_id": f"comparison:{comparison_ref['ref']}",
                "section_kind": "analysis_run_comparison",
                "comparison_ref": comparison_ref,
                "run_context": _bounded_run_context(item.get("run_context")),
                "hash_refs": _hash_refs(item.get("hashes")),
                "result_difference_counts": _result_difference_counts(deltas, item.get("run_context")),
                "largest_differences": _largest_differences(deltas),
                "pairs_not_compared_listed": _listed_rows(item.get("run_context"), "not_compared"),
                "rows_unmatched_listed": _listed_rows(item.get("run_context"), "unmatched"),
                "summary_coverage": _summary_coverage(deltas, item.get("run_context")),
                "csv": _csv_statement(item.get("csv")),
                "manual_mappings": _manual_mappings(deltas),
                "unmatched_classifications": _unmatched_classifications(_list(item.get("diagnostics"))),
                "tolerance_profile_refs": sorted(
                    {
                        str(delta.get("tolerance_profile_ref"))
                        for delta in deltas
                        if delta.get("tolerance_profile_ref")
                    }
                ),
                "settings_deltas": deepcopy(_list(item.get("settings_deltas"))),
                "diagnostics": _safe_public_context(_list(item.get("diagnostics"))),
                "source_provenance": deepcopy(item.get("provenance")),
                "privacy_classification": _privacy_classification(item),
                "review_state": _review_state(item),
                "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
                "text_origin": deepcopy(TEXT_ORIGINS["analysis_run_comparison"]),
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
        if isinstance(manifest, Mapping):
            _missing_required(manifest, REQUIRED_HANDOFF_MANIFEST_FIELDS, package_ref, diagnostics)
        _boundary_flags_from_source(manifest.get("professional_boundary"), package_ref, diagnostics)
        manifest = _checked_record(manifest, "handoff_package_manifest", package_ref, diagnostics)
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
                "text_origin": deepcopy(TEXT_ORIGINS["handoff_package_manifest"]),
            }
        )

    for workflow in export_workflows:
        workflow_ref = _ref("ExportWorkflow", _str(workflow.get("export_workflow_id"), "export:TBD"))
        _missing_required(workflow, REQUIRED_EXPORT_WORKFLOW_FIELDS, workflow_ref, diagnostics)
        _boundary_flags_from_source(workflow.get("professional_boundary"), workflow_ref, diagnostics)
        workflow = _checked_record(workflow, "export_workflow", workflow_ref, diagnostics)
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
                "text_origin": deepcopy(TEXT_ORIGINS["export_workflow_record"]),
            }
        )

    for metadata in external_prover_metadata:
        metadata_ref = _ref("ExternalProverMetadata", _str(metadata.get("metadata_record_id"), "external-prover:TBD"))
        _missing_required(metadata, REQUIRED_EXTERNAL_PROVER_METADATA_FIELDS, metadata_ref, diagnostics)
        _boundary_flags_from_source(metadata.get("professional_boundary"), metadata_ref, diagnostics)
        _external_prover_status_findings(metadata, metadata_ref, diagnostics)
        _external_prover_execution_findings(metadata, metadata_ref, diagnostics)
        metadata = _checked_record(metadata, "external_prover_metadata", metadata_ref, diagnostics)
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
                "text_origin": deepcopy(TEXT_ORIGINS["external_prover_metadata"]),
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


def _vocabulary_diagnostic(affected_ref: Mapping[str, str], path: str, vocabulary: str) -> dict[str, Any]:
    """A value outside its member's vocabulary: named by record, member and vocabulary.
    The value itself is not echoed anywhere."""

    if vocabulary == "software_analysis_statuses":
        return _diagnostic(
            "SCH-ANALYSIS-STATUS-BLOCKED",
            "blocking",
            "PROFESSIONAL_BOUNDARY",
            "Source analysis status is outside the software's analysis-status vocabulary; it is not carried.",
            "State only the software's analysis statuses in the source record.",
            [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("FieldPath", path), _affected("Vocabulary", vocabulary)],
        )
    return _diagnostic(
        "SCH-STATUS-OUTSIDE-VOCABULARY",
        "blocking",
        "STATUS_VOCABULARY",
        "Source record member holds a value outside the software's vocabulary for that member; the value is not carried.",
        "Correct the source record so the member holds a value of its vocabulary, or omit the member.",
        [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("FieldPath", path), _affected("Vocabulary", vocabulary)],
    )


def _in_vocabulary(value: Any, vocabulary: str) -> bool:
    return isinstance(value, str) and value in STATUS_VOCABULARIES[vocabulary]


def _check_member(
    container: Any,
    segments: list[str],
    path: str,
    vocabulary: str,
    affected_ref: Mapping[str, str],
    diagnostics: list[dict[str, Any]],
) -> None:
    if not isinstance(container, dict) or not segments:
        return
    segment, rest = segments[0], segments[1:]
    many = segment.endswith("[]")
    key = segment[:-2] if many else segment
    if key not in container:
        return
    value = container[key]
    where = f"{path}.{key}"
    if many:
        if not isinstance(value, list):
            return
        if rest:
            for index, item in enumerate(value):
                _check_member(item, rest, f"{where}[{index}]", vocabulary, affected_ref, diagnostics)
            return
        kept = []
        for index, item in enumerate(value):
            if _in_vocabulary(item, vocabulary):
                kept.append(item)
            else:
                diagnostics.append(_vocabulary_diagnostic(affected_ref, f"{where}[{index}]", vocabulary))
        container[key] = kept
        return
    if rest:
        _check_member(value, rest, where, vocabulary, affected_ref, diagnostics)
        return
    if value is None:
        return
    if not _in_vocabulary(value, vocabulary):
        diagnostics.append(_vocabulary_diagnostic(affected_ref, where, vocabulary))
        del container[key]


def _checked_record(
    record: Any, kind: str, affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> dict[str, Any]:
    """A copy of `record` with each status-bearing member of its kind checked against its
    vocabulary (DEL-08-06 AC-005): a value outside it is reported and not carried."""

    checked = deepcopy(dict(record)) if isinstance(record, Mapping) else {}
    checked = json.loads(json.dumps(checked))
    for path, vocabulary in STATUS_MEMBERS[kind]:
        _check_member(checked, path.split("."), "$", vocabulary, affected_ref, diagnostics)
    return checked


def _safe_analysis_statuses(statuses: Any) -> list[str]:
    return sorted(
        str(status)
        for status in _list(statuses)
        if str(status) in PERMITTED_SOFTWARE_ANALYSIS_STATUSES
    )


def _external_prover_status_findings(
    metadata: Mapping[str, Any], affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> None:
    """AC-003 by structure: the metadata's contract has no status member, so the presence
    of any, whatever its value, is a finding."""

    for member in EXTERNAL_PROVER_STATUS_MEMBERS:
        if member in metadata:
            diagnostics.append(
                _diagnostic(
                    "SCH-EXTERNAL-PROVER-STATUS-NOT-ADMITTED",
                    "blocking",
                    "REPORT_SECTION_BOUNDARY",
                    "External-prover metadata carries a status member, which its contract does not have.",
                    "Keep external-prover metadata to names, tags, notes and references; record any decision outside the software.",
                    [_affected(affected_ref["object_type"], affected_ref["ref"]), _affected("FieldPath", f"$.{member}")],
                )
            )


def _external_prover_execution_findings(
    metadata: Mapping[str, Any], affected_ref: Mapping[str, str], diagnostics: list[dict[str, Any]]
) -> None:
    if metadata.get("external_prover_executed") is True or metadata.get("execution_performed") is True:
        diagnostics.append(
            _diagnostic(
                "SCH-EXTERNAL-PROVER-EXECUTION-OUT-OF-SCOPE",
                "blocking",
                "REPORT_SECTION_BOUNDARY",
                "External-prover metadata attempted to record in-scope external tool execution.",
                "Keep DEL-08-06 records metadata-only and route external prover execution to its owning workflow.",
                [_affected(affected_ref["object_type"], affected_ref["ref"])],
            )
        )


def _safe_public_context(value: Any) -> Any:
    """The value with private payload members removed (AC-006). Text is carried as it is:
    no text is matched against terms or replaced (AC-005, REQ-005)."""

    if isinstance(value, Mapping):
        return {
            str(key): _safe_public_context(item)
            for key, item in value.items()
            if str(key) not in PRIVATE_PAYLOAD_KEYS
        }
    if isinstance(value, list):
        return [_safe_public_context(item) for item in value]
    return deepcopy(value)


def _numeric_unit_diagnostics(record: Any) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for path, container in _walk_dicts(record):
        numeric_keys = sorted(key for key in NUMERIC_VALUE_KEYS if key in container)
        if numeric_keys:
            for key in numeric_keys:
                if not _finite_number(container.get(key)):
                    diagnostics.append(_unit_diagnostic(f"{path}.{key}", "Numeric value is not finite."))
            has_unit = not _is_missing(container.get("unit")) or not _is_missing(container.get("normalized_unit"))
            if not has_unit or _is_missing(container.get("dimension")):
                diagnostics.append(_unit_diagnostic(path, "Numeric value lacks unit or dimension metadata."))
    return diagnostics


def _finite_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and isfinite(value)


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
            for private_key in PRIVATE_PAYLOAD_KEYS:
                redacted.pop(private_key, None)
            refs.append(redacted)
        else:
            refs.append(deepcopy(item))
    return sorted(refs, key=canonical_json)


UNCHANGED_CLASSIFICATIONS = {"unchanged", "mapped_unchanged"}

CHANGE_VALUE_KEYS = {"left", "right", "left_value", "right_value", "value"}

# REQ-009, AC-006: a member that holds a library record's own content (its
# provenance record) is never stated by value; the record states it by path and
# the SHA-256 of each side's canonical value, and the section keeps only those.
RECORD_CONTENT_MEMBER = re.compile(r"^provenance_record(?:[.\[]|$)")

ORIGIN_KEYS = ("left_origin_ref", "right_origin_ref")

# REQ-011: the bound on restated result differences.
LARGEST_DIFFERENCES_PER_FAMILY_AND_UNIT = 10
RESULT_FAMILY_NONE = "none"
RUN_CONTEXT_ROW_LISTS = {"not_compared", "unmatched"}

CSV_STATEMENT = (
    "The comparison table's CSV holds every row of the comparison; an export with a filter "
    "holds fewer. The CSV's preamble states the hash of this comparison record."
)


def _change_path(change: Mapping[str, Any]) -> str:
    return str(change.get("path") or change.get("field") or "")


def _restated_change(change: Mapping[str, Any]) -> dict[str, Any]:
    if RECORD_CONTENT_MEMBER.match(_change_path(change)):
        return {str(key): deepcopy(value) for key, value in change.items() if str(key) not in CHANGE_VALUE_KEYS}
    return {str(key): deepcopy(value) for key, value in change.items()}


def _changed_entities(items: list[Any]) -> list[dict[str, Any]]:
    """Every entity row that is not unchanged, by identity, with every changed field.

    REQ-011: the section states the comparison as computed by stable identifier:
    each changed field's path, both stored values and their units, and beside them
    the entity's origin reference where the record states one. A member holding a
    library record's own content keeps only its path and hashes (REQ-009).
    """

    changed = []
    for item in items:
        if not isinstance(item, Mapping):
            continue
        if str(item.get("classification")) in UNCHANGED_CLASSIFICATIONS:
            continue
        entry = {
            "classification": deepcopy(item.get("classification")),
            "left_ref": deepcopy(item.get("left_ref")),
            "right_ref": deepcopy(item.get("right_ref")),
            "changes": [
                _restated_change(change)
                for change in _list(item.get("changes"))
                if isinstance(change, Mapping)
            ],
        }
        for key in ORIGIN_KEYS:
            if key in item:
                entry[key] = deepcopy(item[key])
        changed.append(entry)
    return sorted(changed, key=canonical_json)


def _csv_statement(value: Any) -> Any:
    """The comparison table's CSV: its kind and the row count of the unfiltered export."""

    if not isinstance(value, Mapping):
        return None
    return {
        "table": deepcopy(value.get("table")),
        "row_count": deepcopy(value.get("row_count")),
        "statement": CSV_STATEMENT,
    }


def _bounded_run_context(value: Any) -> Any:
    """The run context without its row lists, which the counts state instead."""

    if not isinstance(value, Mapping):
        return deepcopy(value)
    return {str(key): deepcopy(item) for key, item in value.items() if str(key) not in RUN_CONTEXT_ROW_LISTS}


def _mapping(value: Any) -> Mapping[str, Any]:
    return value if isinstance(value, Mapping) else {}


def _delta_identity(delta: Mapping[str, Any]) -> Mapping[str, Any]:
    return _mapping(delta.get("identity"))


def _delta_family(delta: Mapping[str, Any]) -> str:
    value = delta.get("result_family") or _delta_identity(delta).get("family")
    return str(value) if value else RESULT_FAMILY_NONE


def _delta_result_id(delta: Mapping[str, Any]) -> str:
    return str(delta.get("left_result_id") or delta.get("result_id") or "")


def _delta_object(delta: Mapping[str, Any]) -> Any:
    identity = _delta_identity(delta)
    if "object_ref" in identity:
        return identity.get("object_ref")
    return _mapping(delta.get("object_refs")).get("left")


def _row_family(row: Mapping[str, Any]) -> str:
    value = _delta_identity(row).get("family")
    return str(value) if value else RESULT_FAMILY_NONE


def _empty_counts() -> dict[str, Any]:
    return {"pairs_compared": 0, "pairs_differing": 0, "pairs_not_compared": {}, "rows_unmatched": {"left": {}, "right": {}}}


def _add(counter: dict[str, int], key: str) -> None:
    counter[key] = counter.get(key, 0) + 1


def _result_difference_counts(deltas: list[Any], run_context: Any) -> dict[str, Any]:
    """REQ-011: pairs compared, pairs that differ, pairs not compared by reason and
    rows unmatched by side and reason, for the comparison and for each result family."""

    context = _mapping(run_context)
    total = _empty_counts()
    families: dict[str, dict[str, Any]] = {}

    def buckets(family: str) -> tuple[dict[str, Any], dict[str, Any]]:
        return total, families.setdefault(family, _empty_counts())

    for delta in deltas:
        if not isinstance(delta, Mapping):
            continue
        value = delta.get("normalized_delta")
        for bucket in buckets(_delta_family(delta)):
            bucket["pairs_compared"] += 1
            if _finite_number(value) and value != 0:
                bucket["pairs_differing"] += 1
    for pair in _list(context.get("not_compared")):
        if not isinstance(pair, Mapping):
            continue
        for bucket in buckets(_row_family(pair)):
            for reason in _list(pair.get("reasons")):
                _add(bucket["pairs_not_compared"], str(reason))
    for row in _list(context.get("unmatched")):
        if not isinstance(row, Mapping):
            continue
        for bucket in buckets(_row_family(row)):
            side = bucket["rows_unmatched"].setdefault(str(row.get("side")), {})
            _add(side, str(row.get("reason")))
    return {
        "total": total,
        "by_family": [{"result_family": family, **families[family]} for family in sorted(families)],
    }


ROWS_LISTED_PER_FAMILY = 10

SUMMARY_WHERE = (
    "Every pair and row is in the complete comparison record this section names by its hash "
    "and in the comparison table's CSV."
)


def _row_entry(row: Mapping[str, Any], kind: str) -> dict[str, Any]:
    identity = _delta_identity(row)
    entry = {
        "result_id": str(row.get("result_id") or ""),
        "object_ref": deepcopy(identity.get("object_ref")),
        "basis_ref": deepcopy(identity.get("basis_ref")),
        "component": deepcopy(identity.get("component")),
        "location": deepcopy(identity.get("location")),
    }
    if kind == "not_compared":
        entry.update(
            left_unit=deepcopy(row.get("left_unit")),
            right_unit=deepcopy(row.get("right_unit")),
            reasons=deepcopy(_list(row.get("reasons"))),
        )
    else:
        entry.update(
            side=deepcopy(row.get("side")),
            unit=deepcopy(row.get("unit")),
            reason=deepcopy(row.get("reason")),
            differing_fields=deepcopy(_list(row.get("differing_fields"))),
        )
    return entry


def _row_order(row: Mapping[str, Any]) -> tuple[str, str]:
    return (str(row.get("result_id") or ""), str(row.get("side") or ""))


def _rows_by_family(run_context: Any, kind: str) -> dict[str, list[Mapping[str, Any]]]:
    families: dict[str, list[Mapping[str, Any]]] = {}
    for row in _list(_mapping(run_context).get(kind)):
        if isinstance(row, Mapping):
            families.setdefault(_row_family(row), []).append(row)
    return families


def _listed_rows(run_context: Any, kind: str) -> list[dict[str, Any]]:
    """REQ-011: for each result family, the first ten pairs not compared (or rows
    unmatched) in result identifier order, each with its reasons."""

    families = _rows_by_family(run_context, kind)
    return [
        {
            "result_family": family,
            "rows": [_row_entry(row, kind) for row in sorted(families[family], key=_row_order)[:ROWS_LISTED_PER_FAMILY]],
        }
        for family in sorted(families)
    ]


def _summary_coverage(deltas: list[Any], run_context: Any) -> dict[str, Any]:
    """What the summary covers and what it omits: the families summarised, the rows
    not listed, and where they are."""

    largest = _largest_differences(deltas)
    differing = sum(
        1
        for delta in deltas
        if isinstance(delta, Mapping) and _finite_number(delta.get("normalized_delta")) and delta.get("normalized_delta") != 0
    )
    listed = sum(len(group["differences"]) for group in largest)
    not_listed = {}
    for kind in ("not_compared", "unmatched"):
        families = _rows_by_family(run_context, kind)
        not_listed[kind] = sum(max(0, len(rows) - ROWS_LISTED_PER_FAMILY) for rows in families.values())
    families = sorted(
        {_delta_family(delta) for delta in deltas if isinstance(delta, Mapping)}
        | set(_rows_by_family(run_context, "not_compared"))
        | set(_rows_by_family(run_context, "unmatched"))
    )
    return {
        "result_families": families,
        "differences_not_listed": differing - listed,
        "pairs_not_compared_not_listed": not_listed["not_compared"],
        "rows_unmatched_not_listed": not_listed["unmatched"],
        "where": SUMMARY_WHERE,
    }


def _difference_rank(delta: Mapping[str, Any]) -> tuple[float, str]:
    return (-abs(delta["normalized_delta"]), _delta_result_id(delta))


def _difference_entry(delta: Mapping[str, Any]) -> dict[str, Any]:
    identity = _delta_identity(delta)
    return {
        "result_id": _delta_result_id(delta),
        "object_ref": deepcopy(_delta_object(delta)),
        "basis_ref": deepcopy(identity.get("basis_ref")),
        "component": deepcopy(identity.get("component")),
        "location": deepcopy(identity.get("location")),
        "left_magnitude": deepcopy(delta.get("left_magnitude")),
        "right_magnitude": deepcopy(delta.get("right_magnitude")),
        "normalized_delta": deepcopy(delta.get("normalized_delta")),
        "normalized_unit": deepcopy(delta.get("normalized_unit")),
        "dimension": deepcopy(delta.get("dimension")),
    }


def _largest_differences(deltas: list[Any]) -> list[dict[str, Any]]:
    """REQ-011: for each result family and unit, the ten largest differences by
    absolute value, at most one for any one object (its largest, then its lowest
    result identifier), ordered by absolute difference, largest first, then by
    result identifier. A zero difference is never listed."""

    groups: dict[tuple[str, str], dict[str, Mapping[str, Any]]] = {}
    for delta in deltas:
        if not isinstance(delta, Mapping):
            continue
        value = delta.get("normalized_delta")
        if not _finite_number(value) or value == 0:
            continue
        unit = delta.get("normalized_unit")
        key = (_delta_family(delta), str(unit) if unit else "")
        by_object = groups.setdefault(key, {})
        object_key = canonical_json(_delta_object(delta))
        current = by_object.get(object_key)
        if current is None or _difference_rank(delta) < _difference_rank(current):
            by_object[object_key] = delta
    return [
        {
            "result_family": family,
            "normalized_unit": unit,
            "differences": [
                _difference_entry(delta)
                for delta in sorted(groups[(family, unit)].values(), key=_difference_rank)[:LARGEST_DIFFERENCES_PER_FAMILY_AND_UNIT]
            ],
        }
        for family, unit in sorted(groups)
    ]


def _hash_refs(value: Any) -> list[Any]:
    """The comparison record's own hash references, as the record states them."""

    return sorted((deepcopy(item) for item in _list(value)), key=canonical_json)


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


# An entity row is unmatched by its classification or match basis; a diagnostic by its
# class. Matched by token, never by text.
UNMATCHED_ROW_CLASSIFICATIONS = frozenset({"added", "removed", "unresolved"})
UNMATCHED_DIAGNOSTIC_CLASSES = frozenset({"UNMATCHED_ENTITY", "MAPPING_UNRESOLVED", "MISSING_RESULT_DATA"})


def _unmatched_classifications(items: list[Any]) -> list[Any]:
    """Classification tokens and references only, never message bodies (the messages
    stay in the section's diagnostics)."""

    unmatched = []
    for item in items:
        if not isinstance(item, Mapping):
            continue
        if "classification" in item:
            if str(item.get("classification")) in UNMATCHED_ROW_CLASSIFICATIONS or item.get("match_basis") == "unmatched":
                unmatched.append(
                    {
                        "classification": deepcopy(item.get("classification")),
                        "match_basis": deepcopy(item.get("match_basis")),
                        "left_ref": deepcopy(item.get("left_ref")),
                        "right_ref": deepcopy(item.get("right_ref")),
                    }
                )
        elif "code" in item and str(item.get("class")) in UNMATCHED_DIAGNOSTIC_CLASSES:
            unmatched.append(
                {
                    "code": deepcopy(item.get("code")),
                    "class": deepcopy(item.get("class")),
                    "affected_refs": deepcopy(_list(item.get("affected_refs"))),
                }
            )
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
    for index, item in enumerate(_list(record.get("limitations"))):
        if isinstance(item, str) and item:
            limitations.append(
                {
                    "limitation_id": f"source:{affected_ref['ref']}:{index}",
                    "statement": _safe_public_context(item),
                    "affected_ref": deepcopy(dict(affected_ref)),
                    "human_review_required": True,
                }
            )
        elif isinstance(item, Mapping) and item.get("statement"):
            limitation = deepcopy(_safe_public_context(dict(item)))
            limitation.setdefault(
                "limitation_id", f"source:{affected_ref['ref']}:{index}"
            )
            limitation["affected_ref"] = deepcopy(dict(affected_ref))
            limitation["human_review_required"] = True
            limitations.append(limitation)
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


def _mapping_list(value: Any) -> list[Mapping[str, Any]]:
    return [item for item in _list(value) if isinstance(item, Mapping)]


def _is_missing(value: Any) -> bool:
    return value is None or value == "" or value == [] or value == {} or value == "TBD"


def _str(value: Any, fallback: str) -> str:
    return fallback if _is_missing(value) else str(value)


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
