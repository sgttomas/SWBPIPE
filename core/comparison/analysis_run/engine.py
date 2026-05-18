"""Analysis-run comparison engine for diagnostic OpenPipeStress reviews.

The engine compares already-available analysis-run and result-envelope records.
It does not ingest external solver files, define numerical acceptance defaults,
or make professional/code-compliance determinations.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isfinite
from typing import Any, Iterable, Mapping


COMPARABLE_MAPPING_STATUSES = {"automatic_match", "manual_match"}
SUPPORTED_RESULT_FAMILIES = {
    "displacement",
    "rotation",
    "force",
    "moment",
    "reaction",
    "stress",
    "ratio",
}
RUN_CONTEXT_FIELDS = (
    "run_id",
    "run_name",
    "run_kind",
    "created_at",
    "model_state_ref",
    "solver_version",
    "settings_ref",
    "unit_system_ref",
    "load_basis_refs",
    "diagnostics",
    "result_refs",
    "rule_pack_refs",
    "library_refs",
    "hashes",
    "analysis_status",
    "reproducibility",
)


@dataclass(frozen=True)
class ComparisonDiagnostic:
    code: str
    severity: str
    diagnostic_class: str
    affected_refs: tuple[Mapping[str, str], ...]
    message: str
    remediation: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "code": self.code,
            "severity": self.severity,
            "class": self.diagnostic_class,
            "affected_refs": [dict(ref) for ref in self.affected_refs],
            "message": self.message,
            "remediation": self.remediation,
        }


@dataclass(frozen=True)
class ResultDelta:
    mapping_id: str
    left_result_id: str
    right_result_id: str
    result_family: str
    object_refs: dict[str, Any]
    dimension: str
    normalized_unit: str
    left_magnitude: float
    right_magnitude: float
    left_normalized_magnitude: float
    right_normalized_magnitude: float
    raw_delta: float
    normalized_delta: float
    absolute_normalized_delta: float
    tolerance_profile_ref: str | None
    tolerance_rule_id: str | None
    classification: str
    classification_basis: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "mapping_id": self.mapping_id,
            "left_result_id": self.left_result_id,
            "right_result_id": self.right_result_id,
            "result_family": self.result_family,
            "object_refs": dict(self.object_refs),
            "dimension": self.dimension,
            "normalized_unit": self.normalized_unit,
            "left_magnitude": self.left_magnitude,
            "right_magnitude": self.right_magnitude,
            "left_normalized_magnitude": self.left_normalized_magnitude,
            "right_normalized_magnitude": self.right_normalized_magnitude,
            "raw_delta": self.raw_delta,
            "normalized_delta": self.normalized_delta,
            "absolute_normalized_delta": self.absolute_normalized_delta,
            "tolerance_profile_ref": self.tolerance_profile_ref,
            "tolerance_rule_id": self.tolerance_rule_id,
            "classification": self.classification,
            "classification_basis": self.classification_basis,
        }


@dataclass(frozen=True)
class SettingsDelta:
    setting_key: str
    left_value: Any
    right_value: Any
    delta_kind: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "setting_key": self.setting_key,
            "left_value": self.left_value,
            "right_value": self.right_value,
            "delta_kind": self.delta_kind,
        }


@dataclass(frozen=True)
class AnalysisRunComparison:
    comparison_id: str
    run_context: Mapping[str, Any]
    result_deltas: tuple[ResultDelta, ...]
    settings_deltas: tuple[SettingsDelta, ...]
    diagnostics: tuple[ComparisonDiagnostic, ...]
    professional_boundary: Mapping[str, bool]

    @property
    def has_blocking_findings(self) -> bool:
        return any(item.severity == "blocking" for item in self.diagnostics)

    def to_dict(self) -> dict[str, Any]:
        return {
            "comparison_id": self.comparison_id,
            "run_context": _freeze_to_plain(self.run_context),
            "result_deltas": [item.to_dict() for item in self.result_deltas],
            "settings_deltas": [item.to_dict() for item in self.settings_deltas],
            "diagnostics": [item.to_dict() for item in self.diagnostics],
            "has_blocking_findings": self.has_blocking_findings,
            "professional_boundary": dict(self.professional_boundary),
        }


def compare_analysis_runs(
    *,
    left_run: Mapping[str, Any],
    right_run: Mapping[str, Any],
    left_results: Mapping[str, Any],
    right_results: Mapping[str, Any],
    mappings: Iterable[Mapping[str, Any]],
    tolerance_profile: Mapping[str, Any] | None = None,
    unit_conversions: Mapping[tuple[str, str, str], float] | None = None,
    left_settings: Mapping[str, Any] | None = None,
    right_settings: Mapping[str, Any] | None = None,
    comparison_id: str = "analysis-run-comparison",
) -> AnalysisRunComparison:
    """Compare two analysis runs using explicit mappings and unit metadata."""

    diagnostics: list[ComparisonDiagnostic] = []
    conversions = dict(unit_conversions or {})
    left_run_record = _analysis_run_record(left_run)
    right_run_record = _analysis_run_record(right_run)
    left_index = _quantity_index(left_results)
    right_index = _quantity_index(right_results)
    tolerance_rules = _tolerance_rules(tolerance_profile)

    result_deltas: list[ResultDelta] = []
    for mapping in sorted(tuple(mappings), key=_mapping_sort_key):
        result_delta = _compare_mapping(
            mapping,
            left_index,
            right_index,
            tolerance_profile,
            tolerance_rules,
            conversions,
            diagnostics,
        )
        if result_delta is not None:
            result_deltas.append(result_delta)

    diagnostics.extend(_diagnostics_from_run("left", left_run_record))
    diagnostics.extend(_diagnostics_from_run("right", right_run_record))

    return AnalysisRunComparison(
        comparison_id=comparison_id,
        run_context={
            "left": _run_context(left_run_record),
            "right": _run_context(right_run_record),
        },
        result_deltas=tuple(sorted(result_deltas, key=lambda item: item.mapping_id)),
        settings_deltas=_settings_deltas(left_settings or {}, right_settings or {}),
        diagnostics=_stable_diagnostics(diagnostics),
        professional_boundary={
            "human_review_required": True,
            "software_makes_compliance_claim": False,
            "software_makes_certification_claim": False,
            "software_makes_sealing_claim": False,
            "software_makes_approval_claim": False,
            "software_makes_authentication_claim": False,
        },
    )


def comparison_dict(result: AnalysisRunComparison) -> dict[str, Any]:
    return result.to_dict()


def _compare_mapping(
    mapping: Mapping[str, Any],
    left_index: Mapping[str, Mapping[str, Any]],
    right_index: Mapping[str, Mapping[str, Any]],
    tolerance_profile: Mapping[str, Any] | None,
    tolerance_rules: Mapping[tuple[str, str], Mapping[str, Any]],
    conversions: Mapping[tuple[str, str, str], float],
    diagnostics: list[ComparisonDiagnostic],
) -> ResultDelta | None:
    mapping_id = str(mapping.get("mapping_id", "unknown"))
    status = str(mapping.get("mapping_status", "unresolved_mapping"))
    if status not in COMPARABLE_MAPPING_STATUSES:
        diagnostics.append(
            _diagnostic(
                "ARC-MAPPING-NOT-COMPARABLE",
                "warning",
                "MISSING_MAPPING",
                (_ref("MappingRecord", mapping_id),),
                "Result comparison requires an automatic or reviewed manual mapping.",
                "Supply an explicit comparable mapping record before calculating a result delta.",
            )
        )
        return None

    left_result_id = _mapping_ref_value(mapping.get("left_ref"))
    right_result_id = _mapping_ref_value(mapping.get("right_ref"))
    left_result = left_index.get(left_result_id)
    right_result = right_index.get(right_result_id)
    if left_result is None or right_result is None:
        diagnostics.append(
            _diagnostic(
                "ARC-RESULT-DATA-MISSING",
                "blocking",
                "MISSING_RESULT_DATA",
                (
                    _ref("MappingRecord", mapping_id),
                    _ref("Result", left_result_id or "missing-left-ref"),
                    _ref("Result", right_result_id or "missing-right-ref"),
                ),
                "A mapped result reference is not present in the supplied result envelopes.",
                "Provide both mapped result records or mark the mapping unresolved.",
            )
        )
        return None

    family = str(left_result.get("family", ""))
    right_family = str(right_result.get("family", ""))
    if family != right_family or family not in SUPPORTED_RESULT_FAMILIES:
        diagnostics.append(
            _diagnostic(
                "ARC-RESULT-FAMILY-UNSUPPORTED",
                "warning",
                "UNSUPPORTED_CATEGORY",
                (_ref("MappingRecord", mapping_id),),
                "Mapped result families are unsupported or do not match.",
                "Compare only matching supported result families or keep this category as diagnostic evidence.",
            )
        )
        return None

    left_dimension = str(left_result.get("dimension", ""))
    right_dimension = str(right_result.get("dimension", ""))
    if not left_dimension or not right_dimension or left_dimension != right_dimension:
        diagnostics.append(
            _diagnostic(
                "ARC-DIMENSION-INCOMPATIBLE",
                "blocking",
                "INCOMPATIBLE_UNITS",
                (_ref("MappingRecord", mapping_id),),
                "Mapped results do not carry compatible dimension metadata.",
                "Provide matching explicit dimensions before calculating a normalized delta.",
            )
        )
        return None

    left_unit = str(left_result.get("unit", ""))
    right_unit = str(right_result.get("unit", ""))
    if not left_unit or not right_unit:
        diagnostics.append(
            _diagnostic(
                "ARC-UNIT-MISSING",
                "blocking",
                "INCOMPATIBLE_UNITS",
                (_ref("MappingRecord", mapping_id),),
                "Mapped results require explicit unit metadata.",
                "Supply units for both mapped results before calculating a normalized delta.",
            )
        )
        return None

    left_magnitude = _numeric(left_result.get("magnitude"))
    right_magnitude = _numeric(right_result.get("magnitude"))
    if left_magnitude is None or right_magnitude is None:
        diagnostics.append(
            _diagnostic(
                "ARC-MAGNITUDE-MISSING",
                "blocking",
                "MISSING_RESULT_DATA",
                (_ref("MappingRecord", mapping_id),),
                "Mapped results require finite numeric magnitudes.",
                "Provide numeric magnitudes for both mapped results before calculating a delta.",
            )
        )
        return None

    normalized_unit = _normalization_unit(mapping, left_unit)
    left_factor = _conversion_factor(left_unit, normalized_unit, left_dimension, conversions)
    right_factor = _conversion_factor(right_unit, normalized_unit, right_dimension, conversions)
    if left_factor is None or right_factor is None:
        diagnostics.append(
            _diagnostic(
                "ARC-UNIT-CONVERSION-UNSUPPORTED",
                "blocking",
                "INCOMPATIBLE_UNITS",
                (_ref("MappingRecord", mapping_id),),
                "No explicit conversion path was supplied for the mapped result units.",
                "Provide caller-reviewed conversion factors or map results that already share units.",
            )
        )
        return None

    left_normalized = left_magnitude * left_factor
    right_normalized = right_magnitude * right_factor
    tolerance_rule = tolerance_rules.get((family, left_dimension))
    classification, basis = _classify_delta(
        abs(right_normalized - left_normalized),
        tolerance_profile,
        tolerance_rule,
        normalized_unit,
    )

    return ResultDelta(
        mapping_id=mapping_id,
        left_result_id=left_result_id,
        right_result_id=right_result_id,
        result_family=family,
        object_refs={
            "left": _freeze_to_plain(left_result.get("object_ref")),
            "right": _freeze_to_plain(right_result.get("object_ref")),
        },
        dimension=left_dimension,
        normalized_unit=normalized_unit,
        left_magnitude=left_magnitude,
        right_magnitude=right_magnitude,
        left_normalized_magnitude=left_normalized,
        right_normalized_magnitude=right_normalized,
        raw_delta=right_magnitude - left_magnitude,
        normalized_delta=right_normalized - left_normalized,
        absolute_normalized_delta=abs(right_normalized - left_normalized),
        tolerance_profile_ref=_profile_ref(tolerance_profile),
        tolerance_rule_id=str(tolerance_rule.get("rule_id")) if tolerance_rule else None,
        classification=classification,
        classification_basis=basis,
    )


def _analysis_run_record(run: Mapping[str, Any]) -> Mapping[str, Any]:
    nested = run.get("analysis_run")
    if isinstance(nested, Mapping):
        return nested
    return run


def _run_context(run: Mapping[str, Any]) -> dict[str, Any]:
    return {field: _freeze_to_plain(run.get(field)) for field in RUN_CONTEXT_FIELDS if field in run}


def _quantity_index(result_envelope: Mapping[str, Any]) -> dict[str, Mapping[str, Any]]:
    envelope = result_envelope.get("result_envelope")
    root = envelope if isinstance(envelope, Mapping) else result_envelope
    result_sets = root.get("result_sets", [])
    index: dict[str, Mapping[str, Any]] = {}
    if not isinstance(result_sets, list):
        return index
    for result_set in result_sets:
        if not isinstance(result_set, Mapping):
            continue
        quantities = result_set.get("quantity_results") or result_set.get("results") or []
        if not isinstance(quantities, list):
            continue
        for quantity in quantities:
            if isinstance(quantity, Mapping) and quantity.get("result_id"):
                index[str(quantity["result_id"])] = quantity
    return index


def _tolerance_rules(
    tolerance_profile: Mapping[str, Any] | None,
) -> dict[tuple[str, str], Mapping[str, Any]]:
    if tolerance_profile is None:
        return {}
    profile = tolerance_profile.get("tolerance_profile")
    root = profile if isinstance(profile, Mapping) else tolerance_profile
    rules = root.get("rules", [])
    output: dict[tuple[str, str], Mapping[str, Any]] = {}
    if not isinstance(rules, list):
        return output
    for rule in rules:
        if isinstance(rule, Mapping):
            family = str(rule.get("result_family", ""))
            dimension = str(rule.get("dimension_id", ""))
            if family and dimension:
                output[(family, dimension)] = rule
    return output


def _classify_delta(
    absolute_delta: float,
    tolerance_profile: Mapping[str, Any] | None,
    tolerance_rule: Mapping[str, Any] | None,
    normalized_unit: str,
) -> tuple[str, str]:
    if tolerance_profile is None:
        return "not_classified", "no_tolerance_profile_supplied"
    if tolerance_rule is None:
        return "not_classified", "no_matching_tolerance_rule"
    tolerance_value = _numeric(tolerance_rule.get("tolerance_value"))
    if tolerance_value is None:
        return "not_classified", "tolerance_value_not_numeric"
    rule_unit = _unit_ref_value(tolerance_rule.get("unit_ref"))
    if rule_unit and rule_unit != normalized_unit:
        return "not_classified", "tolerance_rule_unit_not_normalized_unit"
    if absolute_delta <= tolerance_value:
        return "within_tolerance_profile", "caller_supplied_tolerance_rule"
    return "exceeds_tolerance_profile", "caller_supplied_tolerance_rule"


def _settings_deltas(
    left_settings: Mapping[str, Any], right_settings: Mapping[str, Any]
) -> tuple[SettingsDelta, ...]:
    deltas: list[SettingsDelta] = []
    for key in sorted(set(left_settings) | set(right_settings)):
        in_left = key in left_settings
        in_right = key in right_settings
        if not in_left:
            deltas.append(SettingsDelta(key, None, _freeze_to_plain(right_settings[key]), "added"))
        elif not in_right:
            deltas.append(SettingsDelta(key, _freeze_to_plain(left_settings[key]), None, "removed"))
        elif left_settings[key] != right_settings[key]:
            deltas.append(
                SettingsDelta(
                    key,
                    _freeze_to_plain(left_settings[key]),
                    _freeze_to_plain(right_settings[key]),
                    "changed",
                )
            )
    return tuple(deltas)


def _diagnostics_from_run(side: str, run: Mapping[str, Any]) -> list[ComparisonDiagnostic]:
    diagnostics: list[ComparisonDiagnostic] = []
    for item in run.get("diagnostics", []) if isinstance(run.get("diagnostics"), list) else []:
        if not isinstance(item, Mapping):
            continue
        diagnostics.append(
            _diagnostic(
                f"ARC-RUN-DIAGNOSTIC-{side.upper()}",
                str(item.get("severity", "warning")),
                "RUN_DIAGNOSTIC",
                (_ref("AnalysisRun", str(run.get("run_id", side))),),
                str(item.get("message", "Analysis run carried a diagnostic.")),
                str(item.get("remediation", "Review run diagnostics as audit evidence.")),
            )
        )
    return diagnostics


def _mapping_ref_value(value: Any) -> str:
    if isinstance(value, Mapping):
        for key in ("ref", "record_id", "result_id", "id"):
            if value.get(key):
                return str(value[key])
    return str(value or "")


def _unit_ref_value(value: Any) -> str:
    if isinstance(value, Mapping):
        for key in ("ref", "unit", "unit_id", "id"):
            if value.get(key):
                return str(value[key])
    return str(value or "")


def _normalization_unit(mapping: Mapping[str, Any], fallback: str) -> str:
    for key in ("normalized_unit", "normalization_unit", "target_unit"):
        if mapping.get(key):
            return str(mapping[key])
    return fallback


def _conversion_factor(
    source_unit: str,
    target_unit: str,
    dimension: str,
    conversions: Mapping[tuple[str, str, str], float],
) -> float | None:
    if source_unit == target_unit:
        return 1.0
    factor = conversions.get((source_unit, target_unit, dimension))
    if factor is None:
        return None
    numeric = _numeric(factor)
    return numeric if numeric is not None else None


def _numeric(value: Any) -> float | None:
    if isinstance(value, bool):
        return None
    try:
        numeric = float(value)
    except (TypeError, ValueError):
        return None
    if not isfinite(numeric):
        return None
    return numeric


def _profile_ref(tolerance_profile: Mapping[str, Any] | None) -> str | None:
    if tolerance_profile is None:
        return None
    profile = tolerance_profile.get("tolerance_profile")
    root = profile if isinstance(profile, Mapping) else tolerance_profile
    value = root.get("profile_id")
    return str(value) if value else None


def _mapping_sort_key(mapping: Mapping[str, Any]) -> tuple[str, str, str]:
    return (
        str(mapping.get("mapping_id", "")),
        _mapping_ref_value(mapping.get("left_ref")),
        _mapping_ref_value(mapping.get("right_ref")),
    )


def _stable_diagnostics(
    diagnostics: Iterable[ComparisonDiagnostic],
) -> tuple[ComparisonDiagnostic, ...]:
    return tuple(
        sorted(
            diagnostics,
            key=lambda item: (
                item.code,
                item.severity,
                item.diagnostic_class,
                tuple((ref.get("object_type", ""), ref.get("ref", "")) for ref in item.affected_refs),
                item.message,
            ),
        )
    )


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    affected_refs: tuple[Mapping[str, str], ...],
    message: str,
    remediation: str,
) -> ComparisonDiagnostic:
    return ComparisonDiagnostic(
        code=code,
        severity=severity,
        diagnostic_class=diagnostic_class,
        affected_refs=affected_refs,
        message=message,
        remediation=remediation,
    )


def _ref(object_type: str, value: str) -> Mapping[str, str]:
    return {"object_type": object_type, "ref": value}


def _freeze_to_plain(value: Any) -> Any:
    if isinstance(value, Mapping):
        return {str(key): _freeze_to_plain(value[key]) for key in sorted(value)}
    if isinstance(value, tuple):
        return [_freeze_to_plain(item) for item in value]
    if isinstance(value, list):
        return [_freeze_to_plain(item) for item in value]
    return value
