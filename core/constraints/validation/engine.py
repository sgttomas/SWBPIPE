"""Constraint validation for available OpenPipeStress design knowledge.

This module emits decision-support diagnostics only. It checks supplied
constraint records and design-knowledge references for explicit evidence,
provenance, assumptions, unit metadata, and represented conflict classes. It
does not compute final technical approval or infer missing owner/code criteria.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable, Mapping


SUPPORTED_SOURCE_TYPES = {"user", "project", "import", "agent", "source_derived"}
KNOWN_KINDS = {
    "connectivity",
    "clearance",
    "no_go_volume",
    "support_zone",
    "route_conflict",
    "slope",
    "drain",
    "vent",
    "access",
    "equipment_interface",
    "missing_required_data",
}
CONFLICT_CLASS_BY_KIND = {
    "connectivity": "CONNECTIVITY_CONFLICT",
    "clearance": "CLEARANCE_CONFLICT",
    "no_go_volume": "ROUTE_CONFLICT",
    "route_conflict": "ROUTE_CONFLICT",
    "support_zone": "SUPPORT_ZONE_CONFLICT",
    "slope": "SLOPE_DRAIN_VENT_CONFLICT",
    "drain": "SLOPE_DRAIN_VENT_CONFLICT",
    "vent": "SLOPE_DRAIN_VENT_CONFLICT",
}
MIN_TARGETS_BY_KIND = {
    "connectivity": 2,
    "clearance": 1,
    "no_go_volume": 1,
    "support_zone": 1,
    "route_conflict": 1,
    "slope": 1,
    "drain": 1,
    "vent": 1,
    "access": 1,
    "equipment_interface": 1,
    "missing_required_data": 1,
}
KINDS_REQUIRING_DESIGN_KNOWLEDGE = {
    "clearance",
    "no_go_volume",
    "support_zone",
    "route_conflict",
    "slope",
    "drain",
    "vent",
    "access",
    "equipment_interface",
}
KINDS_REQUIRING_PARAMETERS = {"clearance", "slope", "drain", "vent", "access"}
CANONICAL_DIMENSIONS = {
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
}
PROVENANCE_FIELDS = (
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
    "privacy_classification",
)
PROFESSIONAL_BOUNDARY_FIELDS = (
    "human_review_required",
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_makes_authentication_claim",
)


@dataclass(frozen=True)
class Diagnostic:
    """Stable, provenance-aware validation diagnostic."""

    code: str
    severity: str
    diagnostic_class: str
    affected_references: tuple[Mapping[str, str], ...]
    source_references: tuple[Mapping[str, str], ...]
    message: str
    remediation: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "code": self.code,
            "severity": self.severity,
            "class": self.diagnostic_class,
            "affected_references": [dict(ref) for ref in self.affected_references],
            "source_references": [dict(ref) for ref in self.source_references],
            "message": self.message,
            "remediation": self.remediation,
        }


@dataclass(frozen=True)
class ValidationResult:
    """Validation output for a constraint envelope."""

    diagnostics: tuple[Diagnostic, ...]

    @property
    def has_blocking_findings(self) -> bool:
        return any(item.severity == "blocking" for item in self.diagnostics)

    def to_dict(self) -> dict[str, Any]:
        return {
            "diagnostics": [item.to_dict() for item in self.diagnostics],
            "has_blocking_findings": self.has_blocking_findings,
        }


def validate_constraint_envelope(
    constraint_envelope: Mapping[str, Any] | None,
    design_knowledge_envelope: Mapping[str, Any] | None = None,
) -> ValidationResult:
    """Validate supplied constraints against available design knowledge.

    The function accepts raw mapping data so missing fields can become explicit
    findings instead of exceptions or hidden defaults.
    """

    diagnostics: list[Diagnostic] = []
    knowledge_records = _design_knowledge_records(design_knowledge_envelope)

    if not isinstance(constraint_envelope, Mapping):
        diagnostics.append(
            _diagnostic(
                "CV-MISSING-CONSTRAINT-ENVELOPE",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (),
                (),
                "Constraint validation requires a supplied constraint envelope.",
                "Provide the constraint envelope from user, project, import, agent, or source-derived data.",
            )
        )
        return ValidationResult(tuple(diagnostics))

    constraint_set = constraint_envelope.get("constraint_set")
    diagnostics.extend(
        _check_required_fields(
            "CV-SET-MISSING-FIELD",
            constraint_envelope,
            (
                "schema_version",
                "deliverable_id",
                "package_id",
                "scope_items",
                "objectives",
                "data_boundary",
                "constraint_set",
            ),
            _ref("ConstraintSetEnvelope", constraint_envelope.get("schema_version", "unknown")),
            (),
        )
    )
    diagnostics.extend(
        _check_boundary(
            constraint_envelope.get("data_boundary"),
            _ref("ConstraintSetEnvelope", constraint_envelope.get("schema_version", "unknown")),
        )
    )

    if not isinstance(constraint_set, Mapping):
        diagnostics.append(
            _diagnostic(
                "CV-SET-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (_ref("ConstraintSet", "missing"),),
                (),
                "Constraint envelope does not contain a usable constraint_set object.",
                "Supply constraint_set with constraints, references, provenance, and professional boundary metadata.",
            )
        )
        return ValidationResult(_stable(diagnostics))

    set_ref = _ref("ConstraintSet", str(constraint_set.get("constraint_set_id", "unknown")))
    diagnostics.extend(
        _check_required_fields(
            "CV-SET-MISSING-FIELD",
            constraint_set,
            (
                "constraint_set_id",
                "project_ref",
                "model_ref",
                "design_knowledge_refs",
                "constraints",
                "diagnostics",
                "provenance",
                "professional_boundary",
            ),
            set_ref,
            _source_refs_from_provenance(constraint_set.get("provenance")),
        )
    )
    diagnostics.extend(_check_provenance("CV-SET-PROVENANCE", constraint_set.get("provenance"), set_ref))
    diagnostics.extend(_check_professional_boundary(constraint_set.get("professional_boundary"), set_ref))
    diagnostics.extend(_check_design_refs(constraint_set.get("design_knowledge_refs"), knowledge_records, set_ref))

    constraints = constraint_set.get("constraints")
    if not isinstance(constraints, list) or not constraints:
        diagnostics.append(
            _diagnostic(
                "CV-CONSTRAINTS-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (set_ref,),
                _source_refs_from_provenance(constraint_set.get("provenance")),
                "Constraint set contains no available constraint records to validate.",
                "Provide at least one constraint record or record that required constraint data is unavailable.",
            )
        )
        return ValidationResult(_stable(diagnostics))

    for index, constraint in enumerate(constraints):
        if isinstance(constraint, Mapping):
            diagnostics.extend(_validate_constraint_record(constraint, index, knowledge_records))
        else:
            diagnostics.append(
                _diagnostic(
                    "CV-CONSTRAINT-INVALID-RECORD",
                    "blocking",
                    "SCHEMA_VALIDATION",
                    (_ref("Constraint", f"index:{index}"),),
                    (),
                    "Constraint entry is not an object and cannot be validated.",
                    "Supply each constraint as a structured record matching the constraint schema.",
                )
            )

    return ValidationResult(_stable(diagnostics))


def diagnostic_dicts(result: ValidationResult | Iterable[Diagnostic]) -> list[dict[str, Any]]:
    if isinstance(result, ValidationResult):
        diagnostics = result.diagnostics
    else:
        diagnostics = tuple(result)
    return [item.to_dict() for item in diagnostics]


def _validate_constraint_record(
    constraint: Mapping[str, Any],
    index: int,
    knowledge_records: Mapping[str, Mapping[str, Any]],
) -> list[Diagnostic]:
    diagnostics: list[Diagnostic] = []
    constraint_id = str(constraint.get("constraint_id", f"index:{index}"))
    constraint_ref = _ref("Constraint", constraint_id)
    source_refs = _source_refs_from_provenance(constraint.get("provenance"))
    kind = constraint.get("constraint_kind")

    diagnostics.extend(
        _check_required_fields(
            "CV-CONSTRAINT-MISSING-FIELD",
            constraint,
            (
                "constraint_id",
                "constraint_kind",
                "name",
                "state",
                "source_type",
                "target_refs",
                "design_knowledge_refs",
                "parameters",
                "diagnostics",
                "assumptions",
                "validation_status",
                "provenance",
                "professional_boundary",
            ),
            constraint_ref,
            source_refs,
        )
    )
    diagnostics.extend(_check_provenance("CV-CONSTRAINT-PROVENANCE", constraint.get("provenance"), constraint_ref))
    diagnostics.extend(_check_professional_boundary(constraint.get("professional_boundary"), constraint_ref))

    source_type = constraint.get("source_type")
    if source_type not in SUPPORTED_SOURCE_TYPES:
        diagnostics.append(
            _diagnostic(
                "CV-CONSTRAINT-SOURCE-UNSUPPORTED",
                "warning",
                "PROVENANCE_WARNING",
                (constraint_ref,),
                source_refs,
                f"Constraint source_type {source_type!r} is not a supported supplied-data source.",
                "Use user, project, import, agent, or source_derived, or quarantine the record pending source review.",
            )
        )

    if kind not in KNOWN_KINDS:
        diagnostics.append(
            _diagnostic(
                "CV-CONSTRAINT-KIND-UNSUPPORTED",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (constraint_ref,),
                source_refs,
                f"Constraint kind {kind!r} is not represented by the upstream constraint schema.",
                "Map the record to a supported constraint kind or keep it outside automated validation.",
            )
        )
    elif kind == "TBD":
        diagnostics.append(
            _diagnostic(
                "CV-CONSTRAINT-KIND-TBD",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (constraint_ref,),
                source_refs,
                "Constraint kind is TBD, so the validation class is unresolved.",
                "Classify the constraint as connectivity, route, clearance, support-zone, slope/drain/vent, access, equipment-interface, or missing-required-data.",
            )
        )

    target_refs = constraint.get("target_refs")
    min_targets = MIN_TARGETS_BY_KIND.get(str(kind), 1)
    if not isinstance(target_refs, list) or len(target_refs) < min_targets:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-TARGETS-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (constraint_ref,),
                source_refs,
                f"Constraint kind {kind!r} requires at least {min_targets} target reference(s) from supplied data.",
                "Provide explicit target_refs or add a missing-required-data constraint that records the unavailable target data.",
            )
        )

    design_refs = constraint.get("design_knowledge_refs")
    if kind in KINDS_REQUIRING_DESIGN_KNOWLEDGE and not design_refs:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-DESIGN-KNOWLEDGE-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (constraint_ref,),
                source_refs,
                f"Constraint kind {kind!r} requires explicit design-knowledge references for validation context.",
                "Attach design_knowledge_refs to available endpoint, zone, corridor, requirement, or equipment-interface records.",
            )
        )
    diagnostics.extend(_check_design_refs(design_refs, knowledge_records, constraint_ref))

    parameters = constraint.get("parameters")
    if kind in KINDS_REQUIRING_PARAMETERS and not parameters:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-PARAMETERS-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (constraint_ref,),
                source_refs,
                f"Constraint kind {kind!r} requires explicit parameter evidence; no engineering threshold is inferred.",
                "Provide parameters with provenance and unit metadata where quantities are used, or record the missing parameter explicitly.",
            )
        )
    diagnostics.extend(_check_parameters(parameters, constraint_ref))

    diagnostics.extend(_check_assumptions(constraint.get("assumptions"), constraint_ref))

    validation_status = constraint.get("validation_status")
    if validation_status in {"conflict_detected", "missing_data", "blocked_by_missing_data"}:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-{str(validation_status).upper().replace('_', '-')}",
                "blocking" if validation_status != "conflict_detected" else "warning",
                CONFLICT_CLASS_BY_KIND.get(str(kind), "CONSTRAINT_MISSING_DATA"),
                _tuple_refs(target_refs, fallback=constraint_ref),
                source_refs,
                f"Constraint record reports validation_status {validation_status!r}; the validator preserves this as a diagnostic.",
                "Resolve the upstream constraint finding with supplied evidence and external human review where required.",
            )
        )
    elif kind in CONFLICT_CLASS_BY_KIND:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-AVAILABLE",
                "info",
                CONFLICT_CLASS_BY_KIND[str(kind)],
                _tuple_refs(target_refs, fallback=constraint_ref),
                source_refs,
                f"Constraint record for {kind!r} is available for decision-support review.",
                "Use this diagnostic as decision-support traceability only; final technical decisions remain outside this module.",
            )
        )
    elif kind in {"access", "equipment_interface", "missing_required_data"}:
        diagnostics.append(
            _diagnostic(
                f"CV-{_kind_code(kind)}-AVAILABLE",
                "info",
                "CONSTRAINT_MISSING_DATA" if kind == "missing_required_data" else "SCHEMA_VALIDATION",
                _tuple_refs(target_refs, fallback=constraint_ref),
                source_refs,
                f"Constraint record for {kind!r} is available and remains bounded to supplied evidence.",
                "Review the referenced design knowledge and resolve missing data without inferring owner or code criteria.",
            )
        )

    return diagnostics


def _check_required_fields(
    code: str,
    record: Mapping[str, Any],
    required_fields: Iterable[str],
    affected_ref: Mapping[str, str],
    source_refs: tuple[Mapping[str, str], ...],
) -> list[Diagnostic]:
    diagnostics = []
    for field in required_fields:
        if field not in record:
            diagnostics.append(
                _diagnostic(
                    code,
                    "blocking",
                    "CONSTRAINT_MISSING_DATA",
                    (affected_ref,),
                    source_refs,
                    f"Required field {field!r} is missing from supplied constraint validation data.",
                    "Supply the field explicitly or record why the data is unavailable; no default will be inferred.",
                )
            )
    return diagnostics


def _check_boundary(boundary: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    if not isinstance(boundary, Mapping):
        return [
            _diagnostic(
                "CV-DATA-BOUNDARY-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (affected_ref,),
                (),
                "Data-boundary metadata is missing from the constraint envelope.",
                "Supply explicit data-boundary policies before relying on validation diagnostics.",
            )
        ]
    diagnostics = []
    expected = {
        "public_examples_policy": "invented_or_cleared_data_only",
        "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
        "private_data_policy": "user_controlled_private_paths",
        "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
        "engineering_authority": "human_review_required_outside_software",
    }
    for field, value in expected.items():
        if boundary.get(field) != value:
            diagnostics.append(
                _diagnostic(
                    "CV-DATA-BOUNDARY-MISMATCH",
                    "blocking" if field in {"protected_source_policy", "engineering_authority"} else "warning",
                    "IP_BOUNDARY_WARNING",
                    (affected_ref,),
                    (),
                    f"Data-boundary field {field!r} is not the expected public contract value.",
                    "Use explicit project data-boundary metadata and quarantine records that may cross protected or professional-authority boundaries.",
                )
            )
    return diagnostics


def _check_provenance(prefix: str, provenance: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    if not isinstance(provenance, Mapping):
        return [
            _diagnostic(
                f"{prefix}-MISSING",
                "blocking",
                "PROVENANCE_WARNING",
                (affected_ref,),
                (),
                "Required provenance metadata is missing.",
                "Provide source name, location, license, contributor, redistribution, review, and privacy metadata.",
            )
        ]
    diagnostics = []
    for field in PROVENANCE_FIELDS:
        value = provenance.get(field)
        if value in (None, "", "TBD"):
            diagnostics.append(
                _diagnostic(
                    f"{prefix}-FIELD-MISSING",
                    "warning",
                    "PROVENANCE_WARNING",
                    (affected_ref,),
                    _source_refs_from_provenance(provenance),
                    f"Provenance field {field!r} is missing or TBD.",
                    "Complete provenance before using this record beyond local decision support.",
                )
            )
    if provenance.get("redistribution_status") in {"protected_suspected", "unknown"}:
        diagnostics.append(
            _diagnostic(
                f"{prefix}-REDISTRIBUTION-REVIEW",
                "blocking" if provenance.get("redistribution_status") == "protected_suspected" else "warning",
                "IP_BOUNDARY_WARNING",
                (affected_ref,),
                _source_refs_from_provenance(provenance),
                "Provenance redistribution status requires review before propagation.",
                "Keep protected-suspected or unknown-source data quarantined according to project data-boundary policy.",
            )
        )
    if provenance.get("privacy_classification") in {"private_project_data", "owner_project_metadata_private", "protected_suspected", "redacted"}:
        diagnostics.append(
            _diagnostic(
                f"{prefix}-PRIVACY-REVIEW",
                "warning",
                "IP_BOUNDARY_WARNING",
                (affected_ref,),
                _source_refs_from_provenance(provenance),
                "Provenance privacy classification limits redistribution of this validation evidence.",
                "Keep diagnostics local to the authorized project context unless the data is cleared or redacted.",
            )
        )
    return diagnostics


def _check_professional_boundary(boundary: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    if not isinstance(boundary, Mapping):
        return [
            _diagnostic(
                "CV-PROFESSIONAL-BOUNDARY-MISSING",
                "blocking",
                "CONSTRAINT_MISSING_DATA",
                (affected_ref,),
                (),
                "Professional-boundary metadata is missing.",
                "Supply explicit professional-boundary flags showing human review is required and software makes no authority claims.",
            )
        ]
    diagnostics = []
    for field in PROFESSIONAL_BOUNDARY_FIELDS:
        if field not in boundary:
            diagnostics.append(
                _diagnostic(
                    "CV-PROFESSIONAL-BOUNDARY-FIELD-MISSING",
                    "blocking",
                    "CONSTRAINT_MISSING_DATA",
                    (affected_ref,),
                    (),
                    f"Professional-boundary field {field!r} is missing.",
                    "Provide all professional-boundary flags explicitly; do not infer professional authority.",
                )
            )
    if boundary.get("human_review_required") is not True:
        diagnostics.append(
            _diagnostic(
                "CV-PROFESSIONAL-HUMAN-REVIEW",
                "blocking",
                "IP_BOUNDARY_WARNING",
                (affected_ref,),
                (),
                "Professional boundary does not explicitly require human review.",
                "Set human_review_required to true before using validation output.",
            )
        )
    for field in PROFESSIONAL_BOUNDARY_FIELDS[1:]:
        if boundary.get(field) is not False:
            diagnostics.append(
                _diagnostic(
                    "CV-PROFESSIONAL-AUTHORITY-CLAIM",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    (affected_ref,),
                    (),
                    f"Professional boundary field {field!r} must be false for this validator.",
                    "Remove software authority claims and route final decisions to external human review.",
                )
            )
    return diagnostics


def _check_design_refs(
    refs: Any,
    knowledge_records: Mapping[str, Mapping[str, Any]],
    affected_ref: Mapping[str, str],
) -> list[Diagnostic]:
    diagnostics = []
    if refs is None:
        return diagnostics
    if not isinstance(refs, list):
        return [
            _diagnostic(
                "CV-DESIGN-REFS-INVALID",
                "blocking",
                "SCHEMA_VALIDATION",
                (affected_ref,),
                (),
                "design_knowledge_refs must be a list when supplied.",
                "Provide explicit design-knowledge references as a list of reference objects.",
            )
        ]
    if not knowledge_records:
        return diagnostics
    for ref in refs:
        ref_id = ref.get("ref") if isinstance(ref, Mapping) else None
        if ref_id and ref_id not in knowledge_records:
            diagnostics.append(
                _diagnostic(
                    "CV-DESIGN-REF-UNRESOLVED",
                    "blocking",
                    "CONSTRAINT_MISSING_DATA",
                    (affected_ref, _ref("DesignKnowledgeRecord", str(ref_id))),
                    (),
                    "Constraint references design knowledge that is not present in the supplied design-knowledge envelope.",
                    "Provide the referenced design-knowledge record or mark the reference as missing required data.",
                )
            )
    return diagnostics


def _check_parameters(parameters: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    diagnostics = []
    if parameters is None:
        return diagnostics
    if not isinstance(parameters, list):
        return [
            _diagnostic(
                "CV-PARAMETERS-INVALID",
                "blocking",
                "SCHEMA_VALIDATION",
                (affected_ref,),
                (),
                "parameters must be a list when supplied.",
                "Provide parameters as schema-compatible objects with provenance.",
            )
        ]
    for parameter in parameters:
        if not isinstance(parameter, Mapping):
            diagnostics.append(
                _diagnostic(
                    "CV-PARAMETER-INVALID",
                    "blocking",
                    "SCHEMA_VALIDATION",
                    (affected_ref,),
                    (),
                    "A parameter entry is not an object.",
                    "Replace the parameter entry with a structured parameter object.",
                )
            )
            continue
        parameter_ref = _ref("Parameter", str(parameter.get("parameter_id", "unknown")))
        diagnostics.extend(_check_provenance("CV-PARAMETER-PROVENANCE", parameter.get("provenance"), parameter_ref))
        value_kind = parameter.get("value_kind")
        value = parameter.get("value")
        if value_kind == "quantity":
            diagnostics.extend(_check_quantity(value, parameter_ref))
        elif isinstance(value, Mapping) and {"value", "unit", "dimension"} & set(value.keys()):
            diagnostics.append(
                _diagnostic(
                    "CV-PARAMETER-QUANTITY-KIND-MISMATCH",
                    "blocking",
                    "UNIT_WARNING",
                    (affected_ref, parameter_ref),
                    _source_refs_from_provenance(parameter.get("provenance")),
                    "Parameter value has quantity-shaped metadata but value_kind is not quantity.",
                    "Mark the parameter as value_kind quantity or replace the value with the declared value kind.",
                )
            )
            diagnostics.extend(_check_quantity(value, parameter_ref))
    return diagnostics


def _check_quantity(quantity: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    if not isinstance(quantity, Mapping):
        return [
            _diagnostic(
                "CV-UNIT-QUANTITY-MISSING",
                "blocking",
                "UNIT_WARNING",
                (affected_ref,),
                (),
                "Quantity parameter does not provide a structured quantity value.",
                "Provide value, unit, dimension, and provenance explicitly; no conversion or tolerance will be inferred.",
            )
        ]
    diagnostics = []
    for field in ("value", "unit", "dimension", "provenance"):
        if quantity.get(field) in (None, "", "TBD"):
            diagnostics.append(
                _diagnostic(
                    "CV-UNIT-METADATA-MISSING",
                    "blocking",
                    "UNIT_WARNING",
                    (affected_ref,),
                    _source_refs_from_provenance(quantity.get("provenance")),
                    f"Quantity field {field!r} is missing or TBD.",
                    "Supply explicit unit-bearing metadata; this validator does not convert units or invent tolerances.",
                )
            )
    dimension = quantity.get("dimension")
    if dimension not in (None, "", "TBD") and dimension not in CANONICAL_DIMENSIONS:
        diagnostics.append(
            _diagnostic(
                "CV-UNIT-DIMENSION-UNKNOWN",
                "blocking",
                "UNIT_WARNING",
                (affected_ref,),
                _source_refs_from_provenance(quantity.get("provenance")),
                f"Quantity dimension {dimension!r} is not in the accepted PKG-02 dimension vocabulary.",
                "Use an accepted dimension identifier such as slope, linear_stiffness, rotational_stiffness, or second_moment_area; keep unresolved values as TBD.",
            )
        )
    diagnostics.extend(_check_provenance("CV-QUANTITY-PROVENANCE", quantity.get("provenance"), affected_ref))
    return diagnostics


def _check_assumptions(assumptions: Any, affected_ref: Mapping[str, str]) -> list[Diagnostic]:
    diagnostics = []
    if not isinstance(assumptions, list):
        return diagnostics
    for assumption in assumptions:
        if not isinstance(assumption, Mapping):
            continue
        status = assumption.get("status")
        assumption_ref = _ref("Assumption", str(assumption.get("assumption_id", "unknown")))
        diagnostics.extend(_check_provenance("CV-ASSUMPTION-PROVENANCE", assumption.get("provenance"), assumption_ref))
        if status in {"unresolved", "TBD", None}:
            diagnostics.append(
                _diagnostic(
                    "CV-ASSUMPTION-UNRESOLVED",
                    "warning",
                    "CONSTRAINT_MISSING_DATA",
                    (affected_ref, assumption_ref),
                    _source_refs_from_provenance(assumption.get("provenance")),
                    "Constraint has an unresolved or TBD assumption.",
                    "Resolve the assumption with supplied evidence or preserve it as a validation limitation.",
                )
            )
    return diagnostics


def _design_knowledge_records(envelope: Mapping[str, Any] | None) -> dict[str, Mapping[str, Any]]:
    if not isinstance(envelope, Mapping):
        return {}
    design_knowledge = envelope.get("design_knowledge")
    if not isinstance(design_knowledge, Mapping):
        return {}
    records = design_knowledge.get("records")
    if not isinstance(records, list):
        return {}
    return {
        str(record["id"]): record
        for record in records
        if isinstance(record, Mapping) and "id" in record
    }


def _source_refs_from_provenance(provenance: Any) -> tuple[Mapping[str, str], ...]:
    if not isinstance(provenance, Mapping):
        return ()
    location = provenance.get("source_location")
    name = provenance.get("source_name")
    if not location and not name:
        return ()
    return (_ref("ExternalReference", str(location or name), label=str(name or "")),)


def _tuple_refs(refs: Any, fallback: Mapping[str, str]) -> tuple[Mapping[str, str], ...]:
    if not isinstance(refs, list):
        return (fallback,)
    parsed = []
    for ref in refs:
        if isinstance(ref, Mapping) and ref.get("ref"):
            parsed.append(_ref(str(ref.get("object_type", "TBD")), str(ref["ref"]), label=ref.get("label")))
    return tuple(parsed) or (fallback,)


def _ref(object_type: str, ref: str, label: str | None = None) -> Mapping[str, str]:
    item = {"object_type": object_type, "ref": ref}
    if label:
        item["label"] = label
    return item


def _kind_code(kind: Any) -> str:
    return str(kind or "UNKNOWN").upper().replace("_", "-")


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    affected_references: tuple[Mapping[str, str], ...],
    source_references: tuple[Mapping[str, str], ...],
    message: str,
    remediation: str,
) -> Diagnostic:
    return Diagnostic(
        code=code,
        severity=severity,
        diagnostic_class=diagnostic_class,
        affected_references=affected_references,
        source_references=source_references,
        message=message,
        remediation=remediation,
    )


def _stable(diagnostics: Iterable[Diagnostic]) -> tuple[Diagnostic, ...]:
    return tuple(
        sorted(
            diagnostics,
            key=lambda item: (
                item.code,
                item.severity,
                item.diagnostic_class,
                tuple((ref.get("object_type", ""), ref.get("ref", "")) for ref in item.affected_references),
                item.message,
            ),
        )
    )
