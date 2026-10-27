"""Three-way comparison record for the CAEPIPE correlation pilot (DEL-17-05 Design §6, PRD §22.5).

`build_comparison_record` pairs each quantity a case names with an analytic
hand-calculation value, the SWBPIPE value (headless runner result rows) and the
CAEPIPE value (parsed Print > To File CSV bound back through the MBF writer's
id maps), compares them in SI base units (m, rad, N, N*m) and classifies every
difference. A quantity covered by a declared behaviour difference with an
independent alternative analytic value (for example a Timoshenko value) also
carries a fourth check, `alternative_analytic_vs_caepipe`, at the case's print
resolution tolerance. Cause hypotheses are labelled ASSUMPTION; a
classification is never silently defaulted. `evaluate_hypothesis_series`
evaluates a declared hypothesis across a slenderness series of cases.

The record built from CAEPIPE output is private (Design §8). It is regression
and handoff evidence only, not validation, compatibility proof, code compliance
or professional acceptance; agreement establishes nothing about acceptance.
"""

from __future__ import annotations

import hashlib
import json
import math
from typing import Any, Mapping


COMPARISON_RECORD_KIND = "openpipestress.caepipe_correlation.comparison_record"
COMPARISON_RECORD_VERSION = "2"
EVIDENCE_CLASSIFICATION = ("private regression and handoff evidence; not validation, compatibility proof, code "
                           "compliance or professional acceptance")
TARGET_TOOL = "CAEPIPE"

MATCHED = "matched_within_tolerance"
EXCEEDS = "delta_exceeds_tolerance"
INTERNAL_ONLY = "row_unmatched_internal_only"
EXTERNAL_ONLY = "row_unmatched_external_only"
TOLERANCE_TBD = "tolerance_tbd"
NOT_APPLICABLE = "not_applicable"
OUTCOMES = (MATCHED, EXCEEDS, INTERNAL_ONLY, EXTERNAL_ONLY, TOLERANCE_TBD, NOT_APPLICABLE)

CAUSES = (
    "solver_defect_candidate", "export_mapping_issue", "parser_mapping_issue", "unit_or_axis_mismatch",
    "support_semantics_mismatch", "load_case_mismatch", "missing_model_parameter",
    "documented_behavior_difference", "caepipe_profile_unknown",
)
DISPOSITIONS = ("no_action", "assumption_recorded", "bug_report", "dossier_question", "proposed_code_fix",
                "requires_professional_review")

# check name -> (left source, right source, case tolerance key); left is the internal side of the pair.
CHECKS = {
    "analytic_vs_swbpipe": ("analytic", "swbpipe", "analytic_vs_swbpipe"),
    "swbpipe_vs_caepipe": ("swbpipe", "caepipe", "external"),
    "analytic_vs_caepipe": ("analytic", "caepipe", "external"),
}
# The fourth check, present only on a quantity a declared difference with an alternative analytic covers.
ALTERNATIVE_CHECK = "alternative_analytic_vs_caepipe"
ALTERNATIVE_CHECK_SPEC = ("alternative", "caepipe", "caepipe_print_resolution")
ALL_CHECKS = {**CHECKS, ALTERNATIVE_CHECK: ALTERNATIVE_CHECK_SPEC}

SERIES_RECORD_KIND = "openpipestress.caepipe_correlation.series_record"
SERIES_CONSISTENT = "consistent"
SERIES_INCONSISTENT = "inconsistent"
SERIES_NOT_EVALUATED = "not_evaluated"
SERIES_STATUSES = (SERIES_CONSISTENT, SERIES_INCONSISTENT, SERIES_NOT_EVALUATED)

# Conditions (c)-(e) of the documented-behaviour-difference rule (DEL-17-05 Design §6).
CONDITION_ALTERNATIVE = "alternative_analytic_matched"
CONDITION_SOURCE = "source_basis"
CONDITION_SERIES = "hypothesis_series_consistent"

_SI_UNIT = {"displacement": "m", "rotation": "rad", "reaction_force": "N", "reaction_moment": "N*m"}
_TO_SI = {"m": 1.0, "mm": 1.0e-3, "rad": 1.0, "N": 1.0, "kN": 1.0e3, "N*m": 1.0, "kN*m": 1.0e3}
_SWBPIPE_KIND = {
    "displacement": "global_nodal_displacement_{}",
    "rotation": "global_nodal_rotation_{}",
}
_SWBPIPE_REACTION_KIND = "support_reaction_component_v2"
_CAEPIPE_KEY = {
    "displacement": "u{}", "rotation": "r{}", "reaction_force": "f{}", "reaction_moment": "m{}",
}


def canonical_model_hash(model: Any) -> str:
    """sha256 of the model's canonical JSON: sorted keys, compact separators, UTF-8."""
    text = json.dumps(model, sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False)
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def build_comparison_record(
    case: Mapping[str, Any],
    *,
    swbpipe_model: Mapping[str, Any],
    swbpipe_result_payload: Mapping[str, Any],
    mbf_result: Any,
    mbf_text: str,
    parsed_caepipe: Mapping[str, Any] | None,
    analytic: Mapping[str, Mapping[str, Any]] | None,
    alternative_analytic: Mapping[str, Mapping[str, Mapping[str, Any]]] | None = None,
    hypothesis_series: Mapping[str, Mapping[str, Any]] | None = None,
) -> dict[str, Any]:
    """Build the PRD §22.5 comparison record of one case.

    `alternative_analytic` maps a hypothesis id to {quantity id: {"value", "unit", "formula", "basis", ...}}
    (independent alternative analytic values); `hypothesis_series` maps a hypothesis id to its series evaluation
    (at least {"status"}); a hypothesis without one counts as `not_evaluated`.
    """

    mbf = mbf_result.to_dict() if hasattr(mbf_result, "to_dict") else dict(mbf_result)
    envelope = swbpipe_result_payload.get("mechanics_envelope", swbpipe_result_payload)
    results = envelope.get("results") or []
    analytic = analytic or {}
    alternative_analytic = alternative_analytic or {}
    series = {key: dict(value) for key, value in (hypothesis_series or {}).items() if isinstance(value, Mapping)}
    tolerances = case.get("tolerances") or {}
    fault = case.get("fault_injection")
    header = (parsed_caepipe or {}).get("header") or {}
    labels_compared: set[str] = set()

    rows = []
    for spec in case.get("quantities") or []:
        si_unit = _SI_UNIT.get(spec.get("quantity"))
        if si_unit is None:
            raise ValueError(f"quantity {spec.get('id')!r}: unknown quantity kind {spec.get('quantity')!r}")
        swb, swb_source = _swbpipe_value(spec, results)
        cae, cae_source = _caepipe_value(spec, mbf, parsed_caepipe)
        if cae_source.get("caepipe_load_case"):
            labels_compared.add(cae_source["caepipe_load_case"])
        reference = analytic.get(spec["id"])
        ana = _si(reference.get("value"), reference.get("unit")) if isinstance(reference, Mapping) else None
        values = {"analytic": ana, "swbpipe": swb, "caepipe": cae}
        checks = {name: _check(values[left], values[right], tolerances.get(key), si_unit, analytic_side=left == "analytic")
                  for name, (left, right, key) in CHECKS.items()}
        declaration = _covering_declaration(spec["id"], case)
        alternative_source: dict[str, Any] | None = None
        if declaration is not None and isinstance(declaration.get("alternative_analytic"), Mapping):
            alt_reference = (alternative_analytic.get(declaration.get("hypothesis")) or {}).get(spec["id"])
            values["alternative"] = (_si(alt_reference.get("value"), alt_reference.get("unit"))
                                     if isinstance(alt_reference, Mapping) else None)
            alternative_source = ({key: alt_reference.get(key) for key in ("formula", "basis", "unit", "assumption")}
                                  if isinstance(alt_reference, Mapping) else {"available": False})
            alternative_source["declared_difference"] = declaration.get("id")
            left, right, key = ALTERNATIVE_CHECK_SPEC
            checks[ALTERNATIVE_CHECK] = _check(values[left], values[right], tolerances.get(key), si_unit,
                                               analytic_side=True)
        row = {
            "id": spec["id"],
            "quantity": spec["quantity"],
            "component": spec.get("component"),
            "load_case": spec.get("load_case"),
            **({"node": spec["node"]} if "node" in spec else {}),
            **({"support": spec["support"]} if "support" in spec else {}),
            "unit": si_unit,
            "values": values,
            "value_sources": {
                "analytic": ({key: reference.get(key) for key in ("formula", "basis", "unit")}
                             if isinstance(reference, Mapping) else {"available": False}),
                "swbpipe": swb_source,
                "caepipe": cae_source,
                **({"alternative": alternative_source} if alternative_source is not None else {}),
            },
            "checks": checks,
        }
        row.update(_classify(row, declaration, fault, series))
        rows.append(row)

    summary = _summary(rows)
    return {
        "document_kind": COMPARISON_RECORD_KIND,
        "record_version": COMPARISON_RECORD_VERSION,
        "case_id": case.get("case_id"),
        "internal_model_hash": canonical_model_hash(swbpipe_model),
        "exported_model_hash": hashlib.sha256(mbf_text.encode("utf-8")).hexdigest(),
        "target_tool": TARGET_TOOL,
        "target_version": header.get("version"),
        "product_label": header.get("product_label"),
        "import_export_assumptions": {
            "writer_profile": mbf.get("profile_id"),
            "assumptions": list(mbf.get("assumptions") or []),
            "loss_report": list(mbf.get("loss_report") or []),
        },
        "manual_changes": ({"fault_injection": dict(fault), "description": fault.get("description")}
                           if isinstance(fault, Mapping) else "none"),
        "load_case_mapping": dict(mbf.get("load_case_map") or {}),
        "support_mapping": dict(mbf.get("support_map") or {}),
        "comparison_basis": {
            "units": "SI base units: m, rad, N, N*m",
            "sign_convention": ("SWBPIPE global axes and right-hand rotations; reactions are the support's action on "
                                "the pipe. The external prover's anchor and restraint loads (the pipe's load on the support) "
                                "are sign-flipped; its rotations are converted from degrees to radians and lengths "
                                "from mm to m."),
            "caepipe_load_cases_compared": sorted(labels_compared),
            "parser_version": (parsed_caepipe or {}).get("parser_version"),
            "pairing_basis": ("exact id pairing: SWBPIPE node and support ids bound to CAEPIPE node numbers through "
                              "the MBF writer's node_map and support_map; element rows are not compared"),
            "unmatched_orientation": ("in each check the first-named source is the internal side: "
                                      "row_unmatched_internal_only means only that source has a value"),
        },
        "tolerance_basis": {
            name: ({**dict(tolerances[key]), "basis": "ASSUMPTION"} if isinstance(tolerances.get(key), Mapping)
                   else {"status": TOLERANCE_TBD, "basis": "ASSUMPTION"})
            for name, (_, _, key) in ALL_CHECKS.items()
        } | {"pass_rule": "|a - b| <= abs + rel * max(|a|, |b|)",
             "governed_status": tolerances.get("governed_status", "TBD"),
             "note": tolerances.get("basis_note")},
        "declared_differences": list(case.get("declared_differences") or []),
        "hypothesis_series": {
            hypothesis: series_summary(series.get(hypothesis))
            for hypothesis in sorted({str(item.get("hypothesis")) for item in case.get("declared_differences") or []
                                      if isinstance(item, Mapping) and item.get("hypothesis")})
        },
        "quantities": rows,
        "summary": summary,
        "discrepancy_disposition": {
            "counts": summary["by_disposition"],
            "items": [{"quantity": row["id"], "disposition": row["disposition"],
                       "cause": row["cause"]["label"] if row["cause"] else None}
                      for row in rows if row["disposition"] != "no_action"],
        },
        "evidence_classification": EVIDENCE_CLASSIFICATION,
    }


def _si(value: Any, unit: Any) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        return None
    if unit == "deg":
        return math.radians(value)
    factor = _TO_SI.get(unit)
    return None if factor is None else value * factor


def _swbpipe_value(spec: Mapping[str, Any], results: list[Any]) -> tuple[float | None, dict[str, Any]]:
    quantity, component, load_case = spec["quantity"], spec.get("component"), spec.get("load_case")
    if quantity in _SWBPIPE_KIND:
        kind, entity, wanted = _SWBPIPE_KIND[quantity].format(component), spec.get("node"), None
    else:
        kind, entity = _SWBPIPE_REACTION_KIND, spec.get("support")
        wanted = ("F" if quantity == "reaction_force" else "M") + str(component)
    matches = [
        row for row in results
        if isinstance(row, Mapping) and row.get("kind") == kind and row.get("entity_ref") == entity
        and (row.get("basis_ref") or {}).get("ref_id") == load_case
        and (wanted is None or (row.get("metadata") or {}).get("component") == wanted)
    ]
    source = {"kind": kind, "entity_ref": entity, "load_case": load_case, **({"component": wanted} if wanted else {})}
    if len(matches) != 1:
        source["reason"] = f"{len(matches)} matching SWBPIPE result rows (exactly one needed)"
        return None, source
    value = _si(matches[0].get("value"), matches[0].get("unit"))
    source.update(result_id=matches[0].get("id"), unit=matches[0].get("unit"))
    if value is None:
        source["reason"] = f"unit {matches[0].get('unit')!r} is not converted"
    return value, source


def _caepipe_value(spec: Mapping[str, Any], mbf: Mapping[str, Any],
                   parsed: Mapping[str, Any] | None) -> tuple[float | None, dict[str, Any]]:
    quantity, component = spec["quantity"], spec.get("component")
    label = (mbf.get("load_case_map") or {}).get(spec.get("load_case"))
    source: dict[str, Any] = {"caepipe_load_case": label}
    if label is None:
        source["reason"] = "the SWBPIPE load case is not in the writer's load_case_map"
        return None, source
    if parsed is None:
        source["reason"] = "no parsed CAEPIPE results"
        return None, source
    tables = (parsed.get("load_cases") or {}).get(label)
    if not isinstance(tables, Mapping):
        source["reason"] = f"the parsed CAEPIPE results have no load case {label!r}"
        return None, source
    units = parsed.get("units") or {}
    key = _CAEPIPE_KEY[quantity].format(component)
    if quantity in ("displacement", "rotation"):
        node = (mbf.get("node_map") or {}).get(spec.get("node"))
        section, unit, sign = "displacements", units.get("length" if quantity == "displacement" else "rotation"), 1.0
        source.update(section=section, node=node)
    else:
        support = (mbf.get("support_map") or {}).get(spec.get("support"))
        if not isinstance(support, Mapping):
            source["reason"] = "the support is not in the writer's support_map"
            return None, source
        node = support.get("node")
        sharing = [sid for sid, other in (mbf.get("support_map") or {}).items()
                   if isinstance(other, Mapping) and other.get("node") == node]
        if len(sharing) > 1:
            source.update(node=node, reason="several SWBPIPE supports share this CAEPIPE node; its total is not split")
            return None, source
        section = "anchor_loads_global" if support.get("caepipe_kind") == "anchor" else "restraint_loads_global"
        unit, sign = units.get("force" if quantity == "reaction_force" else "moment"), -1.0
        source.update(section=section, node=node, sign_flip=True)
    source.update(key=key, unit=unit)
    if node is None:
        source["reason"] = "the node is not in the writer's node_map"
        return None, source
    entry = (tables.get(section) or {}).get(str(node))
    if not isinstance(entry, Mapping) or key not in entry:
        source["reason"] = f"no {section} entry {key!r} for CAEPIPE node {node}"
        return None, source
    value = _si(entry[key], unit)
    if value is None:
        source["reason"] = f"CAEPIPE unit {unit!r} is not converted"
        return None, source
    return sign * value, source


def _check(left: float | None, right: float | None, tolerance: Any, unit: str, *, analytic_side: bool) -> dict[str, Any]:
    if analytic_side and left is None:
        return {"outcome": NOT_APPLICABLE, "reason": "no analytic value for this quantity"}
    if left is None and right is None:
        return {"outcome": NOT_APPLICABLE, "reason": "neither value is available"}
    if right is None:
        return {"outcome": INTERNAL_ONLY}
    if left is None:
        return {"outcome": EXTERNAL_ONLY}
    rel = tolerance.get("rel") if isinstance(tolerance, Mapping) else None
    absolute = (tolerance.get("abs") or {}).get(unit) if isinstance(tolerance, Mapping) and isinstance(
        tolerance.get("abs"), Mapping) else None
    difference = right - left
    if not all(isinstance(item, (int, float)) and not isinstance(item, bool) and item >= 0 for item in (rel, absolute)):
        return {"outcome": TOLERANCE_TBD, "difference": difference}
    allowed = absolute + rel * max(abs(left), abs(right))
    return {
        "outcome": MATCHED if abs(difference) <= allowed else EXCEEDS,
        "difference": difference,
        "allowed": allowed,
        "tolerance": {"rel": rel, "abs": absolute, "unit": unit, "basis": "ASSUMPTION"},
    }


def _verdict(check: Mapping[str, Any]) -> str:
    return {MATCHED: "agree", EXCEEDS: "disagree"}.get(check["outcome"], "unknown")


def _cause(label: str, evidence: str, rule: str, alternatives: list[str] | None = None, **extra: Any) -> dict[str, Any]:
    return {"label": label, "basis": "ASSUMPTION", "evidence": evidence, "rule": rule,
            "alternatives": alternatives or [], **extra}


def _covering_declaration(quantity_id: str, case: Mapping[str, Any]) -> Mapping[str, Any] | None:
    """The first declared difference of the case whose `applies_to` names the quantity."""
    for declared in case.get("declared_differences") or []:
        if isinstance(declared, Mapping) and quantity_id in (declared.get("applies_to") or []):
            return declared
    return None


def series_summary(series: Mapping[str, Any] | None) -> dict[str, Any]:
    """The part of a series evaluation a comparison record carries; `not_evaluated` when there is none."""
    if not isinstance(series, Mapping) or series.get("status") not in SERIES_STATUSES:
        return {"status": SERIES_NOT_EVALUATED, "reasons": ["no series evaluation is available"]}
    return {key: series[key] for key in ("status", "fitted_slope", "expected_exponent", "slope_tolerance",
                                         "reasons", "record") if key in series}


def _missing_conditions(row: Mapping[str, Any], declaration: Mapping[str, Any],
                        series: Mapping[str, Mapping[str, Any]]) -> list[dict[str, str]]:
    """Which of conditions (c)-(e) failed or is unevaluated; empty when all hold."""
    missing = []
    check = row["checks"].get(ALTERNATIVE_CHECK)
    if check is None:
        missing.append({"condition": CONDITION_ALTERNATIVE, "state": "unevaluated",
                        "detail": "the declaration gives no alternative analytic for this quantity"})
    elif check["outcome"] == EXCEEDS:
        missing.append({"condition": CONDITION_ALTERNATIVE, "state": "failed",
                        "detail": "the alternative analytic value differs from the external prover's beyond the print-resolution "
                                  "tolerance"})
    elif check["outcome"] != MATCHED:
        missing.append({"condition": CONDITION_ALTERNATIVE, "state": "unevaluated",
                        "detail": f"{ALTERNATIVE_CHECK} outcome {check['outcome']}"
                                  + (f" ({check['reason']})" if check.get("reason") else "")})
    sources = declaration.get("source_basis")
    if not isinstance(sources, list) or not sources:
        missing.append({"condition": CONDITION_SOURCE, "state": "failed",
                        "detail": "the declaration names no public source basis"})
    hypothesis = declaration.get("hypothesis")
    status = series_summary(series.get(hypothesis) if hypothesis else None)["status"]
    if not hypothesis:
        missing.append({"condition": CONDITION_SERIES, "state": "unevaluated",
                        "detail": "the declaration names no hypothesis series"})
    elif status == SERIES_INCONSISTENT:
        missing.append({"condition": CONDITION_SERIES, "state": "failed",
                        "detail": f"hypothesis series {hypothesis!r} is inconsistent"})
    elif status != SERIES_CONSISTENT:
        missing.append({"condition": CONDITION_SERIES, "state": "unevaluated",
                        "detail": f"hypothesis series {hypothesis!r} is not evaluated"})
    return missing


def _classify(row: Mapping[str, Any], declaration: Mapping[str, Any] | None, fault: Any,
              series: Mapping[str, Mapping[str, Any]]) -> dict[str, Any]:
    checks, values = row["checks"], row["values"]
    outcomes = {name: checks[name]["outcome"] for name in CHECKS}
    if all(outcome in (MATCHED, NOT_APPLICABLE) for outcome in outcomes.values()) and \
            outcomes["swbpipe_vs_caepipe"] == MATCHED:
        return {"status": "matched", "cause": None, "disposition": "no_action"}

    if values["swbpipe"] is None:
        cause = _cause("missing_model_parameter", "value_missing",
                       "no SWBPIPE value was found for the case's quantity reference",
                       ["solver_defect_candidate"])
        return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}
    if values["caepipe"] is None:
        cause = _cause("parser_mapping_issue", "value_missing",
                       "no CAEPIPE value was found through the writer's id maps and the parsed CSV",
                       ["export_mapping_issue", "load_case_mismatch", "support_semantics_mismatch"])
        return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}

    a_s, s_c, a_c = (_verdict(checks[name]) for name in CHECKS)
    if "disagree" not in (a_s, s_c, a_c):
        return {"status": "no_verdict", "cause": None, "disposition": "requires_professional_review",
                "note": "a tolerance is TBD; no comparison verdict and no cause hypothesis"}

    # A declared difference can explain a SWBPIPE-CAEPIPE difference only when the analytic witness has confirmed
    # SWBPIPE; otherwise the triangle rule names the cause and the declaration is listed as an alternative.
    declared = declaration if s_c == "disagree" else None
    also = {"declared_difference_covers": declared.get("id")} if declared else {}

    if a_s == "disagree":
        if s_c == "agree" and a_c == "disagree":
            cause = _cause("missing_model_parameter", "triangle_rule",
                           "SWBPIPE and CAEPIPE agree and the analytic value alone differs: the case inputs or the "
                           "hand-calculation formula are suspected", ["unit_or_axis_mismatch"])
        else:
            cause = _cause("solver_defect_candidate", "triangle_rule",
                           "SWBPIPE differs from the analytic witness"
                           + (" and from CAEPIPE, which agrees with the witness" if a_c == "agree" else ""),
                           ["missing_model_parameter"]
                           + (["documented_behavior_difference"] if declared else []), **also)
        return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}

    if a_s == "agree":
        applies = isinstance(fault, Mapping) and row["id"] in (fault.get("applies_to") or [row["id"]])
        if applies:
            cause = _cause("export_mapping_issue", "deliberate_fault_injection",
                           "analytic and SWBPIPE agree and CAEPIPE differs; the case deliberately altered the "
                           "exported MBF", ["documented_behavior_difference"] if declared else None,
                           fault_description=fault.get("description"), **also)
            return {"status": "not_matched", "cause": cause, "disposition": "bug_report"}
        if declared:
            missing = _missing_conditions(row, declared, series)
            hypothesis = declared.get("hypothesis")
            if not missing:
                cause = _cause("documented_behavior_difference", "alternative_analytic_and_hypothesis_series",
                               "analytic and SWBPIPE agree, SWBPIPE and CAEPIPE differ, the declaration's "
                               "alternative analytic matches the external prover at print resolution, the declaration cites a "
                               "public source basis and its hypothesis series is consistent",
                               evidence_status="established", declared_difference=declared.get("id"),
                               hypothesis=hypothesis, missing=[])
                return {"status": "not_matched", "cause": cause, "disposition": "assumption_recorded"}
            cause = _cause("documented_behavior_difference", "declared_hypothesis",
                           "analytic and SWBPIPE agree and SWBPIPE and CAEPIPE differ on a quantity a declaration "
                           "covers, but the evidence conditions listed in `missing` do not all hold",
                           ["export_mapping_issue", "caepipe_profile_unknown", "support_semantics_mismatch",
                            "load_case_mismatch", "unit_or_axis_mismatch"],
                           evidence_status="hypothesis", declared_difference=declared.get("id"),
                           hypothesis=hypothesis, missing=missing)
            return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}
        cause = _cause("export_mapping_issue", "triangle_rule",
                       "analytic and SWBPIPE agree and CAEPIPE differs: the CAEPIPE side (export mapping, CAEPIPE "
                       "profile or an undeclared behaviour difference) is suspected",
                       ["caepipe_profile_unknown", "documented_behavior_difference", "support_semantics_mismatch",
                        "load_case_mismatch", "unit_or_axis_mismatch"])
        return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}

    cause = _cause("caepipe_profile_unknown", "no_analytic_witness",
                   "SWBPIPE and CAEPIPE differ and no analytic verdict identifies the side",
                   ["solver_defect_candidate", "export_mapping_issue"]
                   + (["documented_behavior_difference"] if declared else []), **also)
    return {"status": "not_matched", "cause": cause, "disposition": "requires_professional_review"}


def evaluate_hypothesis_series(definition: Mapping[str, Any], member_records: Mapping[str, Mapping[str, Any] | None],
                               *, current_mbf_sha256: Mapping[str, str] | None = None) -> dict[str, Any]:
    """Evaluate a slenderness series (Design §7) from its members' comparison records.

    For each member the relative gap (C - S) / S of the definition's quantity is taken from its comparison
    record. Every member must have a record (built from its current MBF when `current_mbf_sha256` names one),
    SWBPIPE confirmed by the analytic witness, both values and a gap larger than the print-resolution absolute
    tolerance; otherwise the status is `not_evaluated` with the reasons. The fitted slope is the log-log slope of
    |relative gap| against span between the shortest and the longest member; the status is `consistent` when every
    relative gap has the expected sign and the slope lies within the tolerance of the expected exponent, otherwise
    `inconsistent`. The result is derived from CAEPIPE output and is private (Design §8).
    """

    quantity = definition.get("quantity")
    expected = float(definition["expected_exponent"])
    tolerance = float((definition.get("slope_tolerance") or {})["abs"])
    sign = definition.get("expected_relative_gap_sign")
    current = current_mbf_sha256 or {}
    members, reasons = [], []
    for member in definition.get("members") or []:
        case_id, span = member["case_id"], float(member["span"])
        entry: dict[str, Any] = {"case_id": case_id, "span": span}
        members.append(entry)
        record = member_records.get(case_id)
        if not isinstance(record, Mapping):
            reasons.append(f"{case_id}: no comparison record")
            continue
        entry["exported_model_hash"] = record.get("exported_model_hash")
        if current.get(case_id) and record.get("exported_model_hash") != current[case_id]:
            reasons.append(f"{case_id}: the comparison record was built from a different MBF")
            continue
        row = next((item for item in record.get("quantities") or [] if item.get("id") == quantity), None)
        if row is None:
            reasons.append(f"{case_id}: the comparison record has no quantity {quantity!r}")
            continue
        swb, cae = (row.get("values") or {}).get("swbpipe"), (row.get("values") or {}).get("caepipe")
        entry.update(unit=row.get("unit"), swbpipe=swb, caepipe=cae)
        if ((row.get("checks") or {}).get("analytic_vs_swbpipe") or {}).get("outcome") != MATCHED:
            reasons.append(f"{case_id}: analytic_vs_swbpipe did not match (SWBPIPE not confirmed by the witness)")
            continue
        if not isinstance(swb, (int, float)) or not isinstance(cae, (int, float)) or swb == 0:
            reasons.append(f"{case_id}: SWBPIPE or CAEPIPE value missing (or SWBPIPE zero)")
            continue
        resolution = ((((record.get("tolerance_basis") or {}).get(ALTERNATIVE_CHECK) or {}).get("abs") or {})
                      .get(row.get("unit")))
        gap = cae - swb
        entry.update(gap=gap, relative_gap=gap / swb, print_resolution_abs=resolution)
        if not isinstance(resolution, (int, float)):
            reasons.append(f"{case_id}: no caepipe_print_resolution tolerance for unit {row.get('unit')!r}")
        elif abs(gap) <= resolution:
            reasons.append(f"{case_id}: the gap is not resolved at print resolution")

    result: dict[str, Any] = {
        "document_kind": SERIES_RECORD_KIND,
        "hypothesis_id": definition.get("hypothesis_id"),
        "quantity": quantity,
        "members": members,
        "expected_exponent": expected,
        "slope_tolerance": {"abs": tolerance, "basis": "ASSUMPTION"},
        "fitted_slope": None,
        "least_squares_slope": None,
        "status": SERIES_NOT_EVALUATED,
        "reasons": reasons,
        "evidence_classification": EVIDENCE_CLASSIFICATION,
    }
    spans = sorted({entry["span"] for entry in members})
    if not reasons and len(spans) < 2:
        reasons.append("fewer than two distinct spans")
    if reasons:
        return result
    short = min(members, key=lambda entry: entry["span"])
    long_ = max(members, key=lambda entry: entry["span"])
    slope = math.log(abs(long_["relative_gap"]) / abs(short["relative_gap"])) / math.log(long_["span"] / short["span"])
    xs = [math.log(entry["span"]) for entry in members]
    ys = [math.log(abs(entry["relative_gap"])) for entry in members]
    mean_x, mean_y = sum(xs) / len(xs), sum(ys) / len(ys)
    lsq = (sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys))
           / sum((x - mean_x) ** 2 for x in xs))
    wrong_sign = [entry["case_id"] for entry in members
                  if sign in (1, -1) and math.copysign(1.0, entry["relative_gap"]) != sign]
    if wrong_sign:
        reasons.append(f"relative gap of the unexpected sign: {', '.join(wrong_sign)}")
    if abs(slope - expected) > tolerance:
        reasons.append("the fitted slope lies outside the tolerance of the expected exponent")
    result.update(fitted_slope=slope, least_squares_slope=lsq,
                  status=SERIES_INCONSISTENT if reasons else SERIES_CONSISTENT)
    return result


def _summary(rows: list[Mapping[str, Any]]) -> dict[str, Any]:
    by_outcome = {name: {outcome: 0 for outcome in OUTCOMES} for name in ALL_CHECKS}
    by_cause: dict[str, int] = {}
    by_evidence_status: dict[str, int] = {}
    by_disposition: dict[str, int] = {}
    by_status: dict[str, int] = {}
    for row in rows:
        for name, check in row["checks"].items():
            by_outcome[name][check["outcome"]] += 1
        label = row["cause"]["label"] if row["cause"] else "none"
        by_cause[label] = by_cause.get(label, 0) + 1
        if row["cause"] and row["cause"].get("evidence_status"):
            status = row["cause"]["evidence_status"]
            by_evidence_status[status] = by_evidence_status.get(status, 0) + 1
        by_disposition[row["disposition"]] = by_disposition.get(row["disposition"], 0) + 1
        by_status[row["status"]] = by_status.get(row["status"], 0) + 1
    return {
        "quantities": len(rows),
        "by_outcome": {name: {key: value for key, value in counts.items() if value} for name, counts in by_outcome.items()},
        "by_cause": dict(sorted(by_cause.items())),
        "by_evidence_status": dict(sorted(by_evidence_status.items())),
        "by_disposition": dict(sorted(by_disposition.items())),
        "by_status": dict(sorted(by_status.items())),
    }
