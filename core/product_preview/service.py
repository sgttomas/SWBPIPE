"""Schema-adjacent service functions for the invented desktop preview.

This module deliberately stays local and deterministic. It loads invented
fixtures, validates the product-preview shape used by the desktop slice, and
returns status envelopes without running external solvers or applying hidden
model mutations.
"""

from __future__ import annotations

from copy import deepcopy
from hashlib import sha256
import json
import math
from pathlib import Path
from typing import Any, Mapping

from core.analysis_runs import build_preview_analysis_run_envelope


ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / "fixtures" / "product_preview"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
}

PROHIBITED_TEXT = (
    "code compliant",
    "certified",
    "sealed",
    "professional approval",
    "approved for construction",
    "release ready",
    "production ready",
)


def load_preview_model(path: Path | None = None) -> dict[str, Any]:
    """Load the invented preview model fixture."""

    return _read_json(path or FIXTURE_DIR / "invented_preview_model.json")


def load_design_knowledge(path: Path | None = None) -> dict[str, Any]:
    """Load invented design-knowledge records for the preview model."""

    return _read_json(path or FIXTURE_DIR / "invented_design_knowledge.json")


def run_preview_mechanics(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Return a deterministic mechanics/status envelope for the preview.

    The current technical preview consumes an invented result fixture. It does
    not perform external solver/prover execution or rule-pack checking.
    """

    model_record = deepcopy(dict(model or load_preview_model()))
    result = _read_json(FIXTURE_DIR / "invented_mechanics_result.json")
    diagnostics = list(result.get("diagnostics", []))
    diagnostics.extend(validate_preview_model(model_record)["diagnostics"])
    result["diagnostics"] = _stable(diagnostics)
    result["professional_boundary"] = deepcopy(PROFESSIONAL_BOUNDARY)
    result["accepted_model_state_mutated"] = False
    return result


def build_analysis_run_preview(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Build an immutable analysis-run record from the preview mechanics result."""

    model_record = deepcopy(dict(model or load_preview_model()))
    mechanics_result = run_preview_mechanics(model_record)
    return build_preview_analysis_run_envelope(
        mechanics_result,
        model_state_ref={
            "object_type": "ModelState",
            "ref": f"state:{mechanics_result['model_ref']}:preview",
        },
        settings_ref={
            "object_type": "SolverSettings",
            "ref": "settings:preview-linear-static",
        },
        unit_system_ref={
            "object_type": "UnitSystem",
            "ref": "SI-preview-units",
        },
        load_basis_refs=_load_basis_refs(model_record),
    )


def build_report_packet_preview(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Materialize read-only report-packet context for the preview workflow.

    This is a structured handoff of the same computed context shown in the
    desktop report packet. It is not a rendered calculation report, export
    adapter payload, compliance result, or professional acceptance record.
    """

    model_record = deepcopy(dict(model or load_preview_model()))
    mechanics_result = run_preview_mechanics(model_record)
    analysis_run = build_analysis_run_preview(model_record)
    proposal_preview = build_agent_proposal_preview()
    selected_result_refs = _selected_result_refs(mechanics_result)
    result_hashes = [
        hash_ref(
            {"object_type": "Result", "ref": item["id"]},
            "result_value",
            item,
        )
        for item in sorted(
            mechanics_result.get("results", []),
            key=lambda result: result.get("id", ""),
        )
    ]
    packet = {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.report_packet",
        "packet_id": f"report-packet:{mechanics_result['run_id']}",
        "model_ref": mechanics_result["model_ref"],
        "source_run_ref": {
            "object_type": "AnalysisRun",
            "ref": analysis_run["analysis_run"]["run_id"],
        },
        "source_result_envelope_ref": {
            "object_type": "ResultEnvelope",
            "ref": f"result-envelope:{mechanics_result['run_id']}",
        },
        "status": deepcopy(mechanics_result["status"]),
        "selected_result_refs": selected_result_refs,
        "diagnostic_refs": [
            {
                "object_type": "Diagnostic",
                "ref": item.get("id", f"diagnostic:{item.get('code', 'unknown')}"),
            }
            for item in mechanics_result.get("diagnostics", [])
        ],
        "analysis_run_context": {
            "deliverable_id": analysis_run["deliverable_id"],
            "run_id": analysis_run["analysis_run"]["run_id"],
            "immutability_policy": deepcopy(analysis_run["analysis_run"]["immutability_policy"]),
            "result_value_hash_count": len(result_hashes),
            "result_envelope_hash_refs": [
                deepcopy(item)
                for item in analysis_run["analysis_run"]["hashes"]
                if item.get("payload_scope") == "result_envelope"
            ],
            "reproducibility_notes": deepcopy(
                analysis_run["analysis_run"]["reproducibility"]["determinism_notes"]
            ),
        },
        "proposal_context": {
            "proposal_ref": {
                "object_type": "AgentProposal",
                "ref": proposal_preview["proposal"]["proposal_id"],
            },
            "application_status": proposal_preview["application_status"],
            "accepted_model_state_mutated": proposal_preview["accepted_model_state_mutated"],
        },
        "hash_refs": [
            hash_ref(
                {"object_type": "ReportPacket", "ref": f"report-packet:{mechanics_result['run_id']}"},
                "report_packet_context",
                {
                    "model_ref": mechanics_result["model_ref"],
                    "run_id": mechanics_result["run_id"],
                    "selected_result_refs": selected_result_refs,
                    "diagnostic_ids": [
                        item.get("id", f"diagnostic:{item.get('code', 'unknown')}")
                        for item in mechanics_result.get("diagnostics", [])
                    ],
                    "result_hash_count": len(result_hashes),
                },
            )
        ],
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "privacy_boundary": {
            "classification": "invented_public_example",
            "private_payload_embedded": False,
            "protected_payload_embedded": False,
            "telemetry_allowed": False,
        },
        "report_packet_status": {
            "materialization": "read_only_context_packet",
            "rendered_calculation_report": False,
            "result_export_payload": False,
            "external_handoff_payload": False,
            "professional_acceptance_record": False,
        },
        "diagnostics": [],
    }
    packet["hash_refs"].extend(result_hashes)
    return packet


def build_agent_proposal_preview() -> dict[str, Any]:
    """Return a deterministic, non-mutating proposal preview."""

    proposal = _read_json(FIXTURE_DIR / "invented_agent_proposal.json")
    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.agent_proposal_preview",
        "proposal": proposal,
        "validation": deepcopy(proposal["validation"]),
        "diff_preview": deepcopy(proposal["operation"]["changes"]),
        "application_status": "not_applied",
        "accepted_model_state_mutated": False,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "diagnostics": [],
    }


def build_model_tree(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Build simple tree/inspector input records for the desktop UI."""

    record = deepcopy(dict(model or load_preview_model()))
    entities = []
    for collection_name, ref_type in (
        ("nodes", "node"),
        ("pipe_segments", "pipe"),
        ("supports", "support"),
        ("components", "component"),
    ):
        for item in record.get(collection_name, []):
            entities.append(
                {
                    "id": item["id"],
                    "type": ref_type,
                    "label": item.get("label", item["id"]),
                    "properties": _properties(item),
                }
            )
    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.model_tree",
        "entities": sorted(entities, key=lambda item: (item["type"], item["id"])),
    }


def validate_preview_model(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Validate the bounded product-preview shape and guardrail text."""

    record = deepcopy(dict(model or load_preview_model()))
    diagnostics: list[dict[str, str]] = []

    if record.get("document_kind") != "openpipestress.product_preview.model":
        diagnostics.append(_diagnostic("PREVIEW_DOCUMENT_KIND_INVALID", "blocking", "model"))
    if not record.get("schema_version"):
        diagnostics.append(_diagnostic("PREVIEW_SCHEMA_VERSION_MISSING", "blocking", "model"))
    if not record.get("nodes") or not record.get("pipe_segments"):
        diagnostics.append(_diagnostic("PREVIEW_GEOMETRY_INCOMPLETE", "blocking", "model"))

    for collection_name in ("nodes", "pipe_segments", "supports", "materials", "load_cases", "combinations"):
        for item in record.get(collection_name, []):
            _validate_identity_and_provenance(item, collection_name, diagnostics)
            if collection_name == "load_cases":
                for load in item.get("primitive_loads", []):
                    _validate_identity_and_provenance(load, "primitive_loads", diagnostics)
            if collection_name == "combinations":
                if item.get("basis") != "mechanics":
                    diagnostics.append(_diagnostic("PREVIEW_COMBINATION_BASIS_UNSUPPORTED", "blocking", item.get("id", "combination:TBD")))
                if not item.get("terms"):
                    diagnostics.append(_diagnostic("PREVIEW_COMBINATION_TERMS_EMPTY", "blocking", item.get("id", "combination:TBD")))
                load_case_ids = {case.get("id") for case in record.get("load_cases", [])}
                for term in item.get("terms", []):
                    if term.get("load_case") not in load_case_ids:
                        diagnostics.append(_diagnostic("PREVIEW_COMBINATION_LOAD_CASE_UNKNOWN", "blocking", item.get("id", "combination:TBD")))
                    factor = term.get("factor")
                    if not isinstance(factor, (int, float)) or not math.isfinite(factor):
                        diagnostics.append(_diagnostic("PREVIEW_COMBINATION_FACTOR_INVALID", "blocking", item.get("id", "combination:TBD")))

    node_ids = {node.get("id") for node in record.get("nodes", [])}
    for segment in record.get("pipe_segments", []):
        if segment.get("from") not in node_ids or segment.get("to") not in node_ids:
            diagnostics.append(_diagnostic("PREVIEW_PIPE_ENDPOINT_UNKNOWN", "blocking", segment.get("id", "pipe:TBD")))
        section = segment.get("section", {})
        for field in ("outside_diameter", "wall_thickness"):
            quantity = section.get(field)
            if not isinstance(quantity, Mapping) or "value" not in quantity or "unit" not in quantity:
                diagnostics.append(_diagnostic("PREVIEW_UNIT_METADATA_MISSING", "blocking", segment.get("id", "pipe:TBD")))

    for support in record.get("supports", []):
        if support.get("node") not in node_ids:
            diagnostics.append(_diagnostic("PREVIEW_SUPPORT_NODE_UNKNOWN", "blocking", support.get("id", "support:TBD")))

    serialized = canonical_json(record).lower()
    for phrase in PROHIBITED_TEXT:
        if phrase in serialized:
            diagnostics.append(_diagnostic("PREVIEW_PROHIBITED_CLAIM_TEXT", "blocking", phrase))

    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.validation",
        "status": "blocked" if any(item["severity"] == "blocking" for item in diagnostics) else "passed",
        "diagnostics": _stable(diagnostics),
        "status_boundaries": deepcopy(record.get("analysis_status", {})),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def hash_ref(payload_ref: Mapping[str, str], payload_scope: str, payload: Any) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS",
        "payload_ref": dict(payload_ref),
        "payload_scope": payload_scope,
        "value": sha256(canonical_json(payload).encode("utf-8")).hexdigest(),
    }


def _selected_result_refs(mechanics_result: Mapping[str, Any]) -> list[str]:
    results = mechanics_result.get("results", [])
    result_ids = [str(item.get("id", "")) for item in results if isinstance(item, Mapping)]
    selected = [
        mechanics_result.get("summary", {}).get("max_displacement", {}).get("result_ref"),
        mechanics_result.get("summary", {}).get("max_open_formula_stress", {}).get("result_ref"),
        "result:force:pipe-P-120:axial" if "result:force:pipe-P-120:axial" in result_ids else None,
        "result:force:pipe-P-120:axial:end-j"
        if "result:force:pipe-P-120:axial:end-j" in result_ids
        else None,
        "result:force:pipe-P-120:midspan:axial"
        if "result:force:pipe-P-120:midspan:axial" in result_ids
        else None,
        "result:force:pipe-P-120:quarter-1:shear-y"
        if "result:force:pipe-P-120:quarter-1:shear-y" in result_ids
        else None,
        "result:force:pipe-P-120:shear-y"
        if "result:force:pipe-P-120:shear-y" in result_ids
        else None,
        "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
        if "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in result_ids
        else None,
        "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
        if "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y" in result_ids
        else None,
        "result:stress:pipe-P-120:end-j:torsional-shear"
        if "result:stress:pipe-P-120:end-j:torsional-shear" in result_ids
        else None,
        "result:stress:pipe-P-120:quarter-1:torsional-shear"
        if "result:stress:pipe-P-120:quarter-1:torsional-shear" in result_ids
        else None,
    ]
    return [item for item in selected if item]


def _load_basis_refs(model: Mapping[str, Any]) -> list[dict[str, str]]:
    refs = [
        {"object_type": "LoadCase", "ref": str(item["id"])}
        for item in model.get("load_cases", [])
        if isinstance(item, Mapping) and item.get("id")
    ]
    refs.extend(
        {"object_type": "Combination", "ref": str(item["id"])}
        for item in model.get("combinations", [])
        if isinstance(item, Mapping) and item.get("id")
    )
    return refs or [{"object_type": "LoadCase", "ref": "loadcase:preview"}]


def _read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def _properties(item: Mapping[str, Any]) -> list[dict[str, Any]]:
    result = []
    for key, value in item.items():
        if key == "position" and isinstance(value, Mapping):
            result.extend(
                {"field": f"position.{axis}", "value": value.get(axis), "unit": "m"}
                for axis in ("x", "y", "z")
            )
        elif key not in {"section"}:
            result.append({"field": key, "value": value})
    if isinstance(item.get("section"), Mapping):
        for key, quantity in item["section"].items():
            result.append({"field": f"section.{key}", "value": quantity.get("value"), "unit": quantity.get("unit")})
    return result


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": str(target_ref)}


def _validate_identity_and_provenance(
    item: Mapping[str, Any], collection_name: str, diagnostics: list[dict[str, str]]
) -> None:
    item_id = str(item.get("id", ""))
    if not item_id.strip():
        diagnostics.append(_diagnostic("PREVIEW_ID_MISSING", "blocking", collection_name))
    provenance = str(item.get("provenance", "")).lower()
    if "invented" not in provenance and "cleared" not in provenance:
        diagnostics.append(_diagnostic("PREVIEW_PROVENANCE_MISSING", "blocking", item_id or collection_name))


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return sorted((dict(item) for item in items), key=canonical_json)
