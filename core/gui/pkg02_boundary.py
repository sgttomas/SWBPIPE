"""Shared PKG-02 boundary helpers for GUI contract records."""

from __future__ import annotations

from copy import deepcopy
from math import isfinite
from typing import Any, Mapping


CANONICAL_DIMENSIONS = (
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "force_per_length",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "TBD",
)

CANONICAL_ANALYSIS_STATUSES = (
    "MODEL_INCOMPLETE",
    "MECHANICS_SOLVED",
    "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED",
    "USER_RULE_FAILED",
    "HUMAN_REVIEW_REQUIRED",
    "HUMAN_APPROVED_FOR_PROJECT",
)

AUTOMATIC_ANALYSIS_STATUSES = tuple(
    status for status in CANONICAL_ANALYSIS_STATUSES if status != "HUMAN_APPROVED_FOR_PROJECT"
)

HUMAN_ACCEPTANCE_STATUS = "HUMAN_APPROVED_FOR_PROJECT"


def unit_contract() -> dict[str, Any]:
    """Return the GUI-facing PKG-02 unit contract summary."""

    return {
        "contract_ref": "DEL-02-02",
        "schema_ref": "schemas/units.schema.yaml#/$defs/DimensionId",
        "canonical_dimensions": list(CANONICAL_DIMENSIONS),
        "retired_dimension_values_forbidden": True,
        "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
        "missing_unit_behavior": "diagnostic_blocking",
    }


def analysis_boundary_contract() -> dict[str, Any]:
    """Return the GUI-facing PKG-02 analysis-boundary summary."""

    return {
        "contract_ref": "DEL-02-03",
        "schema_ref": "schemas/analysis_boundary.schema.yaml",
        "status_schema_ref": "schemas/analysis_status.schema.yaml",
        "automatic_status_scope": list(AUTOMATIC_ANALYSIS_STATUSES),
        "human_acceptance_status": HUMAN_ACCEPTANCE_STATUS,
        "human_acceptance_authority": "external_hash_bound_human_record_only",
        "software_emits_human_approval_status": False,
        "bound_hashes_required_for_human_acceptance": True,
        "forbidden_software_claims": [
            "code_compliance",
            "certification",
            "sealing",
            "professional_acceptance",
            "authentication",
        ],
    }


def persistence_hash_contract(*, gui_hash_scope: str) -> dict[str, Any]:
    """Return the GUI-facing PKG-02 persistence/hash summary."""

    return {
        "contract_ref": "DEL-02-05",
        "schema_ref": "schemas/project_persistence.schema.yaml",
        "canonicalization": "JCS",
        "payload_hashes_required_for_persisted_or_result_available_records": True,
        "human_acceptance_invalidates_on_hash_change": True,
        "gui_hash_scope": gui_hash_scope,
    }


def pkg02_diagnostic(
    code: str,
    severity: str,
    target_ref: Any,
    *,
    diagnostic_class: str = "TBD",
    source: str = "TBD",
    message: str | None = None,
    remediation: str | None = None,
    contract_ref: str | None = None,
) -> dict[str, Any]:
    """Build a deterministic diagnostic with PKG-02 envelope-adjacent fields."""

    target = _target_text(target_ref)
    return {
        "code": code,
        "diagnostic_id": f"PKG07:{code}:{_slug(target)}",
        "diagnostic_class": diagnostic_class,
        "class": diagnostic_class,
        "severity": severity,
        "source": source,
        "target_ref": target,
        "affected_ref": {"object_type": "GUIContractObject", "ref": target},
        "message": message or code,
        "remediation": remediation
        or "Retain the diagnostic until the referenced PKG-02 contract evidence is supplied.",
        "contract_ref": contract_ref or "PKG-02",
        "provenance": {
            "source_name": "OpenPipeStress PKG-07 GUI contract",
            "source_location": "core/gui/pkg02_boundary.py",
            "privacy_classification": "public_metadata",
        },
    }


def normalize_analysis_statuses(
    value: Any,
    diagnostics: list[dict[str, Any]],
    target_ref: Any,
    *,
    required: bool = False,
) -> list[str]:
    """Validate and normalize software-emitted analysis statuses."""

    raw_statuses = _list(value)
    if isinstance(value, str):
        raw_statuses = [value]
    if not raw_statuses:
        if required:
            diagnostics.append(
                pkg02_diagnostic(
                    "ANALYSIS_STATUS_MISSING",
                    "warning",
                    target_ref,
                    diagnostic_class="TBD",
                    source="report_boundary",
                    message="A result-bearing GUI record lacks canonical analysis status metadata.",
                    remediation="Attach DEL-02-03 software status values before result review or handoff.",
                    contract_ref="DEL-02-03",
                )
            )
        return []

    normalized: list[str] = []
    for status in raw_statuses:
        text = _text(status)
        if text not in CANONICAL_ANALYSIS_STATUSES:
            diagnostics.append(
                pkg02_diagnostic(
                    "ANALYSIS_STATUS_NON_CANONICAL",
                    "blocking",
                    target_ref,
                    diagnostic_class="TBD",
                    source="report_boundary",
                    message=f"Analysis status {text!r} is not in the DEL-02-03 vocabulary.",
                    remediation="Use the canonical PKG-02 analysis status vocabulary.",
                    contract_ref="DEL-02-03",
                )
            )
            continue
        if text == HUMAN_ACCEPTANCE_STATUS:
            diagnostics.append(
                pkg02_diagnostic(
                    "ANALYSIS_STATUS_HUMAN_APPROVAL_NOT_SOFTWARE_EMITTED",
                    "blocking",
                    target_ref,
                    diagnostic_class="TBD",
                    source="human_record_reference",
                    message="Human project acceptance is external and hash-bound; GUI software must not emit it as automatic status.",
                    remediation="Move human acceptance to an external record with bound reviewed hashes.",
                    contract_ref="DEL-02-03",
                )
            )
            continue
        if text not in normalized:
            normalized.append(text)

    order = {status: index for index, status in enumerate(CANONICAL_ANALYSIS_STATUSES)}
    return sorted(normalized, key=order.get)


def quantity_metadata(
    *,
    value: Any,
    unit: Any,
    dimension: Any,
    diagnostics: list[dict[str, Any]],
    target_ref: Any,
    code_prefix: str,
) -> dict[str, Any]:
    """Validate explicit unit and dimension metadata for GUI-visible quantities."""

    numeric = _is_number(value)
    unit_text = _text(unit)
    dimension_text = _text(dimension)
    unit_present = bool(unit_text) and unit_text != "TBD"
    dimension_present = bool(dimension_text) and dimension_text != "TBD"
    unit_bearing = numeric or unit_present or dimension_present

    if unit_bearing and not unit_present:
        diagnostics.append(
            pkg02_diagnostic(
                f"{code_prefix}_UNIT_METADATA_MISSING",
                "blocking",
                target_ref,
                diagnostic_class="TBD",
                source="model_validation",
                message="A unit-bearing GUI value is missing explicit unit metadata.",
                remediation="Provide a unit identifier or keep the value blocked as unresolved.",
                contract_ref="DEL-02-02",
            )
        )
    if unit_bearing and not dimension_present:
        diagnostics.append(
            pkg02_diagnostic(
                f"{code_prefix}_DIMENSION_METADATA_MISSING",
                "blocking",
                target_ref,
                diagnostic_class="TBD",
                source="model_validation",
                message="A unit-bearing GUI value is missing explicit dimension metadata.",
                remediation="Provide a canonical DEL-02-02 dimension identifier.",
                contract_ref="DEL-02-02",
            )
        )
    if dimension_text and dimension_text not in CANONICAL_DIMENSIONS:
        diagnostics.append(
            pkg02_diagnostic(
                f"{code_prefix}_DIMENSION_NON_CANONICAL",
                "blocking",
                target_ref,
                diagnostic_class="TBD",
                source="model_validation",
                message=f"Dimension {dimension_text!r} is not in the accepted DEL-02-02 vocabulary.",
                remediation="Use a canonical dimension identifier or retain the value as TBD.",
                contract_ref="DEL-02-02",
            )
        )

    return {
        "contract_ref": "DEL-02-02",
        "schema_ref": "schemas/units.schema.yaml#/$defs/DimensionId",
        "requires_explicit_unit_metadata": unit_bearing,
        "unit_metadata_present": unit_present,
        "dimension_metadata_present": dimension_present,
        "dimension_is_canonical": not dimension_text or dimension_text in CANONICAL_DIMENSIONS,
        "dimension": dimension_text or "TBD",
    }


def hash_boundary_status(hashes: Any) -> dict[str, Any]:
    """Summarize DEL-02-05 hash evidence without interpreting payload contents."""

    records = [deepcopy(item) for item in _list(hashes) if isinstance(item, Mapping)]
    return {
        "contract_ref": "DEL-02-05",
        "hashes_present": bool(records),
        "hash_count": len(records),
        "canonicalization_values": sorted(
            {
                str(item.get("canonicalization"))
                for item in records
                if item.get("canonicalization") not in (None, "")
            }
        ),
        "algorithm_values": sorted(
            {str(item.get("algorithm")) for item in records if item.get("algorithm") not in (None, "")}
        ),
    }


def source_of_truth_boundary(*, surface: str, mutation_route: str) -> dict[str, Any]:
    """Describe how a GUI surface avoids becoming the canonical data authority."""

    return {
        "surface": surface,
        "gui_is_source_of_truth": False,
        "canonical_source_of_truth_ref": "PKG-02 physical_source_of_truth model and governed persistence records",
        "mutation_route": mutation_route,
    }


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any) -> str:
    text = str(value).strip() if value is not None else ""
    return "" if text == "None" else text


def _is_number(value: Any) -> bool:
    return not isinstance(value, bool) and isinstance(value, (int, float)) and isfinite(float(value))


def _target_text(target_ref: Any) -> str:
    if isinstance(target_ref, str):
        return target_ref
    if isinstance(target_ref, Mapping):
        ref = target_ref.get("ref") or target_ref.get("ref_id") or target_ref.get("id")
        object_type = target_ref.get("object_type") or target_ref.get("ref_type") or "Object"
        return f"{object_type}:{ref}" if ref else str(dict(target_ref))
    return str(target_ref)


def _slug(value: str) -> str:
    return "".join(char if char.isalnum() else "-" for char in value)[:96] or "target"
