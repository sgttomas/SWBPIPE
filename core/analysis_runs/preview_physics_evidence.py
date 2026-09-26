"""Closed preview-physics-1 reader checks (T0R S1 §9); never producer authentication.

The checks bind the published rows, diagnostics, headlines and the closed
``contract_evidence`` statement to one another. They do not recompute the
solve, the enclosures or the support laws; numerical standing still needs the
caller's authentic source, model, input and build binding.
"""
from __future__ import annotations

from collections.abc import Mapping
import json
import math
from pathlib import Path
import sys
from typing import Any

CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
PROFILE = "product_preview_mechanics_v1"
ROOT = Path(__file__).resolve().parents[2]
TABLE_PATH = ROOT / "fixtures/results/semantic_contract_v0_3_preview_physics_1.json"
TABLE = json.loads(TABLE_PATH.read_text(encoding="utf-8"))
LIMITATIONS = list(TABLE["supported_profile_limitations"])
RETIRED_KINDS = frozenset(TABLE["retired_source_kinds"])
RETIRED_CODES = frozenset({"COMPONENT_STRESS_MULTIPLIER_APPLIED", "COMBINATION_STRESS_SUMMARY_SKIPPED"})

MAXIMUM_KIND = "pipe_elastic_normal_stress_maximum_v2"
INTENSIFIED_KIND = "component_equal_factor_intensified_bending_stress_v1"
SUPPORT_COMPONENT_KIND = "support_reaction_component_v2"
SUPPORT_FORCE_KIND = "support_reaction_force_magnitude_v2"
SUPPORT_MOMENT_KIND = "support_reaction_moment_magnitude_v2"
SUPPORT_KINDS = frozenset({SUPPORT_COMPONENT_KIND, SUPPORT_FORCE_KIND, SUPPORT_MOMENT_KIND})
SUPPORT_COMPONENTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
SUPPORT_ROW_KINDS = {**{c: (SUPPORT_COMPONENT_KIND, "N") for c in SUPPORT_COMPONENTS[:3]},
                     **{c: (SUPPORT_COMPONENT_KIND, "N*m") for c in SUPPORT_COMPONENTS[3:]},
                     "force_magnitude": (SUPPORT_FORCE_KIND, "N"), "moment_magnitude": (SUPPORT_MOMENT_KIND, "N*m")}
SUPPORT_SIGN = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate"
MAXIMUM_METADATA = {
    "component": "maximum_absolute_normal_stress", "coordinate_system": "pipe_section",
    "location": "governing_station", "basis": "recovered_from_open_mechanics_stress_components",
    "sign_convention": "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim",
}
INTENSIFIED_SIGN_PREFIX = "nonnegative i*hypot(My,Mz)/Z at the member end; i="
EXTREMA_KEYS = frozenset({"pipe_id", "result_id", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions", "approximation", "coefficient_basis", "enclosure_scope"})
EXTREMA_CONSTANTS = {
    "approximation": "piecewise_quadratic_straight_section_statics",
    "coefficient_basis": "j_side_section_equilibrium_binary64",
    "enclosure_scope": "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate",
}
CASE_KEYS = frozenset({"load_case_id", "pipe_stress_extrema", "stress_maximum_coverage", "support_attribution", "intensified_measures"})
INTENSIFIED_KEYS = frozenset({"result_id", "component_id", "pipe_id", "location", "factor_role", "sif", "sif_source_reference", "section_modulus_m3", "bending_moment_y_n_m", "bending_moment_z_n_m"})
WITHHELD_REASONS = frozenset({"SUPPORT_ACTION_ATTRIBUTION_WITHHELD", "CONSTANT_EFFORT_NOT_CONSUMED"})
WITHHELD_DIAGNOSTIC_PREFIX = {
    "SUPPORT_ACTION_ATTRIBUTION_WITHHELD": "diagnostic:preview-physics:attribution:",
    "CONSTANT_EFFORT_NOT_CONSUMED": "diagnostic:preview-physics:constant-effort-not-consumed:",
}
GATE_REASONS = ("NONLINEAR_COMBINATION_REQUIRES_SOLVE", "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", "COMBINATION_MODULUS_BASIS_MIXED")
RANGE_BASIS = "explicit_user_range_envelope"
PER_SUPPORT_NONLINEAR_KINDS = ("nonlinear_support_final_reaction", "nonlinear_support_final_displacement", "nonlinear_support_active_set_state_code")
# Representation guard only (S1 §9.7); not an engineering tolerance.
GUARD = 64.0 * sys.float_info.epsilon
TINY = sys.float_info.min


def _require(condition: Any, detail: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: {detail}")


def _text(value: Any) -> bool:
    return isinstance(value, str) and bool(value)


def _number(value: Any) -> bool:
    return type(value) in (int, float) and math.isfinite(value)


def _integer(value: Any) -> bool:
    """A4 N7/N10: an integer by numeric value (4 or 4.0); never a boolean or 4.5."""
    return _number(value) and float(value).is_integer()


def _strings(value: Any) -> bool:
    return isinstance(value, list) and all(_text(item) for item in value) and len(set(value)) == len(value)


def _shape(value: Any, keys: frozenset[str] | set[str], detail: str) -> None:
    _require(isinstance(value, Mapping) and set(value) == set(keys), detail)


def _finite_tree(value: Any) -> None:
    if type(value) in (int, float):
        _require(math.isfinite(value), "nonfinite evidence")
    elif isinstance(value, Mapping):
        for item in value.values():
            _finite_tree(item)
    elif isinstance(value, list):
        for item in value:
            _finite_tree(item)


def _length_prefixed(*parts: str) -> str:
    """The producer's exact_source_identity: ``{UTF-8 byte length}:{part}`` per part."""
    return "".join(f"{len(part.encode('utf-8'))}:{part}" for part in parts)


def _row_segments(*parts: str) -> str:
    """Row-id segments (S1 §3): ``{L(part)}:{part}`` joined by ``:``, unlike ID()."""
    return ":".join(f"{len(part.encode('utf-8'))}:{part}" for part in parts)


def _consistent_norm(magnitude: float, components: list[float]) -> bool:
    x, y, z = components
    expected = math.hypot(math.hypot(x, y), z)
    return magnitude >= 0 and abs(magnitude - expected) <= GUARD * max(abs(magnitude), TINY)


def signature(row: Mapping[str, Any]) -> Mapping[str, Any] | None:
    """First table variant, in table order, whose ``source_basis`` is absent or equal.

    Same rule as the other readers (S1 §1); uniqueness is not required for
    this table because arc variants precede their generic variants.
    """
    kind, unit = row.get("kind"), row.get("unit")
    _require(kind not in RETIRED_KINDS, f"retired kind {kind}")
    candidates = [entry for entry in TABLE["rows"] if entry["kind"] == kind]
    _require(candidates, f"kind without signature {kind}")
    units = [entry for entry in candidates if entry["unit"] == unit]
    _require(units, f"unit contradiction {kind}/{unit}")
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    component = metadata.get("component")
    for pool in ([entry for entry in units if entry.get("component") == component],
                 [entry for entry in units if entry.get("component") is None]):
        if pool:
            exact = [entry for entry in pool if "source_basis" not in entry or entry["source_basis"] == metadata.get("basis")]
            _require(exact, f"basis contradiction {kind}")
            return exact[0]
    _require(component is None, f"component contradiction {kind}/{component}")
    return None


def _header(source: Mapping[str, Any]) -> None:
    _require(not any(key in source for key in ("source_block_recovery", "carrier_evidence")), "unsupported source namespace")
    _require(source.get("schema_version") == "0.2.0", "schema version")
    _require(source.get("producer") == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": CONTRACT_ID}, "producer")
    formulation = source.get("formulation_basis")
    _shape(formulation, {"profile_id", "limitations"}, "formulation basis")
    _require(formulation["profile_id"] == PROFILE and formulation["limitations"] == LIMITATIONS, "formulation profile or limitations")
    evidence = source.get("contract_evidence")
    _shape(evidence, {"preview_cases", "combination_gates"}, "contract evidence namespace")
    _require(isinstance(evidence["preview_cases"], list) and isinstance(evidence["combination_gates"], list), "contract evidence lists")
    _finite_tree(evidence)


def _cases(evidence: Mapping[str, Any]) -> dict[str, Mapping[str, Any]]:
    cases: dict[str, Mapping[str, Any]] = {}
    for case in evidence["preview_cases"]:
        _shape(case, CASE_KEYS, "preview case shape")
        cid = case["load_case_id"]
        _require(_text(cid) and cid not in cases, "preview case identity")
        cases[cid] = case
        _require(isinstance(case["pipe_stress_extrema"], list) and isinstance(case["intensified_measures"], list), "preview case lists")
        coverage = case["stress_maximum_coverage"]
        _shape(coverage, {"complete", "unavailable_pipe_ids", "outside_domain_pipe_ids"}, "maximum coverage shape")
        _require(type(coverage["complete"]) is bool and _strings(coverage["unavailable_pipe_ids"]) and _strings(coverage["outside_domain_pipe_ids"]), "maximum coverage values")
        _require(not set(coverage["unavailable_pipe_ids"]) & set(coverage["outside_domain_pipe_ids"]), "maximum coverage overlap")
        _require(coverage["complete"] == (not coverage["unavailable_pipe_ids"] and not coverage["outside_domain_pipe_ids"]), "maximum coverage completeness")
        attribution = case["support_attribution"]
        _shape(attribution, {"attributed_support_ids", "withheld"}, "support attribution shape")
        _require(_strings(attribution["attributed_support_ids"]) and isinstance(attribution["withheld"], list), "support attribution values")
        for record in attribution["withheld"]:
            _shape(record, {"support_id", "reason"}, "withheld support shape")
            _require(_text(record["support_id"]) and record["reason"] in WITHHELD_REASONS, "withheld support values")
        # A3 2: a support is attributed or withheld, never both.
        _require(not set(attribution["attributed_support_ids"]) & {record["support_id"] for record in attribution["withheld"]}, "support both attributed and withheld")
        for extremum in case["pipe_stress_extrema"]:
            _shape(extremum, EXTREMA_KEYS, "extrema shape")
            _require(_text(extremum["pipe_id"]) and _text(extremum["result_id"]) and all(extremum[key] == value for key, value in EXTREMA_CONSTANTS.items()), "extrema identity or basis")
            _require(all(_number(extremum[key]) for key in ("station_fraction", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa")), "extrema numbers")
            _require(_integer(extremum["span_index"]) and extremum["span_index"] >= 0 and _integer(extremum["subdivisions"]) and 0 <= extremum["subdivisions"] <= 131072, "extrema integers")
            _require(0 <= extremum["station_fraction"] <= 1 and 0 <= extremum["local_fraction"] <= 1, "extrema fractions")
            _require(0 <= extremum["value_lower_pa"] <= extremum["value_upper_pa"], "extrema bounds")
        pipes = [extremum["pipe_id"] for extremum in case["pipe_stress_extrema"]]
        _require(len(set(pipes)) == len(pipes) and not set(pipes) & (set(coverage["unavailable_pipe_ids"]) | set(coverage["outside_domain_pipe_ids"])), "extrema member partition")
        for measure in case["intensified_measures"]:
            _shape(measure, INTENSIFIED_KEYS, "intensified measure shape")
            _require(all(_text(measure[key]) for key in ("result_id", "component_id", "pipe_id")) and measure["location"] in {"end_i", "end_j"} and measure["factor_role"] in {"bend", "branch_header", "branch_branch"} and _text(measure["sif_source_reference"]), "intensified measure identity")
            _require(all(_number(measure[key]) for key in ("sif", "section_modulus_m3", "bending_moment_y_n_m", "bending_moment_z_n_m")) and measure["sif"] > 0 and measure["section_modulus_m3"] > 0, "intensified measure inputs")
    # A2 3: dispositions are structural, so every case carries the same sets.
    dispositions = {(frozenset(case["support_attribution"]["attributed_support_ids"]), frozenset((r["support_id"], r["reason"]) for r in case["support_attribution"]["withheld"])) for case in cases.values()}
    _require(len(dispositions) <= 1, "support attribution differs between cases")
    extrema_ids = [extremum["result_id"] for case in cases.values() for extremum in case["pipe_stress_extrema"]]
    measure_ids = [measure["result_id"] for case in cases.values() for measure in case["intensified_measures"]]
    _require(len(set(extrema_ids)) == len(extrema_ids) and len(set(measure_ids)) == len(measure_ids), "duplicate evidence result binding")
    return cases


def _gates(evidence: Mapping[str, Any]) -> dict[str, Mapping[str, Any]]:
    gates: dict[str, Mapping[str, Any]] = {}
    for gate in evidence["combination_gates"]:
        _shape(gate, {"combination_id", "withheld", "reason"}, "combination gate shape")
        _require(_text(gate["combination_id"]) and gate["combination_id"] not in gates, "combination gate identity")
        _require(type(gate["withheld"]) is bool and ((gate["reason"] in GATE_REASONS) if gate["withheld"] else gate["reason"] is None), "combination gate reason")
        gates[gate["combination_id"]] = gate
    return gates


def validate_preview_physics_evidence(source: Mapping[str, Any]) -> None:
    try:
        _validate(source)
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise ValueError("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error


def _validate(source: Mapping[str, Any]) -> None:
    # 1. Header, closed evidence key sets, finite numbers.
    _header(source)
    evidence = source["contract_evidence"]
    cases = _cases(evidence)
    gates = _gates(evidence)
    results, diagnostics = source.get("results"), source.get("diagnostics")
    _require(isinstance(results, list) and isinstance(diagnostics, list), "result collections")
    _require(all(isinstance(row, Mapping) and _text(row.get("id")) for row in results) and all(isinstance(item, Mapping) and _text(item.get("id")) for item in diagnostics), "evidence identities")
    rows = {row["id"]: row for row in results}
    _require(len(rows) == len(results), "duplicate result ID")

    # 2. Every row kind has a signature; no retired kind or diagnostic code.
    for row in results:
        _require(_number(row.get("value")) and _text(row.get("kind")) and _text(row.get("unit")) and _text(row.get("entity_ref")), "source row fields")
        _require(row.get("metadata") is None or isinstance(row.get("metadata"), Mapping), "source row metadata")
        signature(row)
        if row["kind"] in SUPPORT_KINDS:
            # A4 N6: every support-action row is global, whatever its basis_ref.
            _require((row.get("metadata") or {}).get("coordinate_system") == "global", "support action frame")
        basis = row.get("basis_ref")
        if basis is not None:
            _shape(basis, {"ref_type", "ref_id"}, "row basis reference")
    for item in diagnostics:
        _require(item.get("code") not in RETIRED_CODES, f"retired diagnostic code {item.get('code')}")

    # 3. Completeness (F-1): result-namespace references only.
    for item in diagnostics:
        # A1 a / A2 5: the key may be absent; when present it is an array of non-empty strings.
        refs = item.get("affected_refs", [])
        _require(isinstance(refs, list) and all(_text(ref) for ref in refs), "diagnostic reference list")
        for ref in refs:
            _require(not ref.startswith("result:") or ref in rows, f"dangling result reference {ref}")
    summary = source.get("summary")
    _require(isinstance(summary, Mapping), "summary")
    for field, headline in summary.items():
        if isinstance(headline, Mapping) and headline.get("result_ref") is not None:
            _require(headline["result_ref"] in rows, f"dangling summary result reference {field}")

    # 4. Case scope.
    quality = source.get("numerical_quality", {}).get("cases", [])
    quality_ids = [case.get("basis_ref", {}).get("ref_id") for case in quality]
    if source.get("status", {}).get("mechanics") == "MECHANICS_SOLVED":
        _require(list(cases) == quality_ids, "preview case coverage")
    else:
        # A1 e: a blocked envelope has empty evidence, no rows and null headlines.
        _require(not evidence["preview_cases"] and not evidence["combination_gates"], "blocked envelope carries preview evidence")
        _require(not results, "blocked envelope carries result rows")
        _require(summary.get("max_displacement") is None and summary.get("max_open_formula_stress") is None, "blocked envelope carries a headline")
        _require("component_stress_modifier_count" not in summary or (_integer(summary["component_stress_modifier_count"]) and summary["component_stress_modifier_count"] == 0), "blocked envelope counts intensified rows")
    for row in results:
        basis = row.get("basis_ref")
        if basis is None:
            continue
        if basis["ref_type"] == "combination":
            gate = gates.get(basis["ref_id"])
            # 9. Combination rows only for admitted combinations.
            _require(gate is not None and gate["withheld"] is False, "combination row without an admitting gate")
            _require(row["kind"] not in {MAXIMUM_KIND, INTENSIFIED_KIND}, "maximum or intensified row for a combination")
            refs = row.get("source_result_refs", [])
            _require(isinstance(refs, list) and all(isinstance(ref, str) and ref in rows for ref in refs), "combination source reference")
            if row["kind"] in SUPPORT_KINDS:
                # A2 4: only basis and sign_convention follow the combination.
                metadata = row.get("metadata") or {}
                component = metadata.get("component")
                _require(component in SUPPORT_ROW_KINDS and SUPPORT_ROW_KINDS[component] == (row["kind"], row["unit"]) and metadata.get("coordinate_system") == "global" and metadata.get("location") == "node", "combination support row semantics")

    def case_rows(cid: str, kind: str | None = None) -> list[Mapping[str, Any]]:
        return [row for row in results if row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} and (kind is None or row["kind"] == kind)]

    # 5. Maxima bind to their enclosure; coverage partitions the case members.
    maximum_rows = [row for row in results if row["kind"] == MAXIMUM_KIND]
    bound: set[str] = set()
    for row in maximum_rows:
        _require(row["unit"] == "Pa" and row.get("metadata") == MAXIMUM_METADATA and (row.get("basis_ref") or {}).get("ref_type") == "load_case", "maximum row semantics")
        cid = row["basis_ref"]["ref_id"]
        _require(row["id"] == f"result:elastic-maximum:{_row_segments(cid, row['entity_ref'])}", "maximum row identity")
    for cid, case in cases.items():
        coverage = case["stress_maximum_coverage"]
        members = {row["entity_ref"] for row in case_rows(cid, "element_local_axial_force")}
        covered = [extremum["pipe_id"] for extremum in case["pipe_stress_extrema"]]
        _require(set(covered) | set(coverage["unavailable_pipe_ids"]) | set(coverage["outside_domain_pipe_ids"]) == members, "maximum coverage does not partition the case members")
        for extremum in case["pipe_stress_extrema"]:
            row = rows.get(extremum["result_id"])
            _require(row is not None and row["kind"] == MAXIMUM_KIND and row["entity_ref"] == extremum["pipe_id"] and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}, "extrema result binding")
            lower, upper, value = extremum["value_lower_pa"], extremum["value_upper_pa"], row["value"]
            _require(lower <= value <= upper and value == lower + 0.5 * (upper - lower), "maximum outside or off its enclosure")
            bound.add(row["id"])
    _require(bound == {row["id"] for row in maximum_rows}, "maximum row without exactly one enclosure")

    # 6. Headlines over all load cases, with identity ties.
    def governing(candidates: list[Mapping[str, Any]]) -> Mapping[str, Any]:
        return min(candidates, key=lambda row: (-row["value"], row["basis_ref"]["ref_id"], row["entity_ref"]))

    complete = bool(cases) and all(case["stress_maximum_coverage"]["complete"] for case in cases.values())
    stress = summary.get("max_open_formula_stress")
    if complete and maximum_rows:
        selected = governing(maximum_rows)
        _require(stress == {"value": selected["value"], "unit": "Pa", "location_ref": selected["entity_ref"], "result_ref": selected["id"]}, "stress headline does not govern")
    else:
        _require(stress is None, "stress headline while coverage is incomplete or no maximum exists")
    displacement_rows = [row for row in results if row["kind"] == "displacement_magnitude" and (row.get("basis_ref") or {}).get("ref_type") == "load_case"]
    displacement = summary.get("max_displacement")
    if displacement_rows:
        selected = governing(displacement_rows)
        _require(displacement == {"value": selected["value"], "unit": "mm", "location_ref": selected["entity_ref"], "result_ref": selected["id"]}, "displacement headline does not govern")
    else:
        _require(displacement is None, "displacement headline without displacement rows")
    intensified_rows = [row for row in results if row["kind"] == INTENSIFIED_KIND]
    if "component_stress_modifier_count" in summary:
        _require(_integer(summary["component_stress_modifier_count"]) and summary["component_stress_modifier_count"] == len(intensified_rows), "intensified row count")

    # 7. Support actions: attributed => exactly eight rows; withheld => none.
    withheld_diagnostics: set[tuple[str, str]] = set()
    for item in diagnostics:
        if item.get("code") in WITHHELD_REASONS:
            refs = item.get("affected_refs")
            _require(isinstance(refs, list) and refs and _text(refs[0]) and item["id"] == f"{WITHHELD_DIAGNOSTIC_PREFIX[item['code']]}{_length_prefixed(refs[0])}", "withholding diagnostic identity")
            _require((refs[0], item["code"]) not in withheld_diagnostics, "duplicate withholding diagnostic")
            withheld_diagnostics.add((refs[0], item["code"]))
    for cid, case in cases.items():
        attribution = case["support_attribution"]
        grouped: dict[str, dict[str, Mapping[str, Any]]] = {}
        for row in case_rows(cid):
            if row["kind"] not in SUPPORT_KINDS:
                continue
            component = (row.get("metadata") or {}).get("component")
            slots = grouped.setdefault(row["entity_ref"], {})
            _require(component not in slots, "duplicate support action component")
            slots[component] = row
        _require(set(grouped) == set(attribution["attributed_support_ids"]), "support action rows do not match attribution")
        for support, slots in grouped.items():
            _require(set(slots) == set(SUPPORT_ROW_KINDS), "support action component coverage")
            for component, row in slots.items():
                kind, unit = SUPPORT_ROW_KINDS[component]
                _require(row["kind"] == kind and row["unit"] == unit and row["id"] == f"result:support-action:{_row_segments(cid, support)}:{component}", "support action identity")
                _require(row.get("metadata") == {"component": component, "coordinate_system": "global", "location": "node", "basis": "recovered_from_assembled_support_law", "sign_convention": SUPPORT_SIGN}, "support action semantics")
            _require(_consistent_norm(slots["force_magnitude"]["value"], [slots[c]["value"] for c in SUPPORT_COMPONENTS[:3]]), "support force magnitude inconsistent with components")
            _require(_consistent_norm(slots["moment_magnitude"]["value"], [slots[c]["value"] for c in SUPPORT_COMPONENTS[3:]]), "support moment magnitude inconsistent with components")
        # A1 b: in every case the withheld records equal the withholding
        # diagnostics (support = affected_refs[0], reason = code, §8 ids).
        records = {(record["support_id"], record["reason"]) for record in attribution["withheld"]}
        _require(len(records) == len(attribution["withheld"]) and records == withheld_diagnostics, "withheld support record and diagnostic disagree")
        listed = set(attribution["attributed_support_ids"]) | {support for support, _ in records}
        _require(all(row["entity_ref"] in listed for kind in PER_SUPPORT_NONLINEAR_KINDS for row in case_rows(cid, kind)), "per-support nonlinear row names an unlisted support")
        ambiguous = {support for support, reason in records if reason == "SUPPORT_ACTION_ATTRIBUTION_WITHHELD"}
        _require(not any(row["entity_ref"] in ambiguous for row in case_rows(cid, "nonlinear_support_final_reaction")), "withheld support still publishes a final reaction")

    # 8. Intensified rows <=> intensified_measures.
    measured: set[str] = set()
    for cid, case in cases.items():
        for measure in case["intensified_measures"]:
            row = rows.get(measure["result_id"])
            _require(row is not None and row["kind"] == INTENSIFIED_KIND and row["unit"] == "Pa" and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} and row["entity_ref"] == measure["component_id"], "intensified result binding")
            metadata = row.get("metadata") or {}
            _require(metadata.get("component") == "equal_factor_intensified_bending_stress" and metadata.get("coordinate_system") == "pipe_section" and metadata.get("location") == measure["location"] and metadata.get("basis") == "user_sif_times_member_section_bending_stress_v1" and isinstance(metadata.get("sign_convention"), str) and metadata["sign_convention"].startswith(INTENSIFIED_SIGN_PREFIX), "intensified row semantics")
            expected = measure["sif"] * (math.hypot(measure["bending_moment_y_n_m"], measure["bending_moment_z_n_m"]) / measure["section_modulus_m3"])
            # A3 1: i*hypot(My,Mz)/Z with i > 0 is never negative.
            _require(row["value"] >= 0, "negative intensified value")
            _require(abs(row["value"] - expected) <= GUARD * max(abs(row["value"]), TINY), "intensified value inconsistent with its inputs")
            refs = row.get("source_result_refs")
            _require(isinstance(refs, list) and all(isinstance(ref, str) and ref in rows and rows[ref].get("basis_ref") == row["basis_ref"] for ref in refs), "intensified source reference does not resolve in the same case")
            measured.add(row["id"])
    _require(measured == {row["id"] for row in intensified_rows}, "intensified row without exactly one measure")

    # 9. Admitted mechanics/subtraction magnitudes follow their components.
    combination_rows: dict[str, list[Mapping[str, Any]]] = {}
    for row in results:
        basis = row.get("basis_ref")
        if basis is not None and basis["ref_type"] == "combination":
            combination_rows.setdefault(basis["ref_id"], []).append(row)
    for combination, members in combination_rows.items():
        bases = {(row.get("metadata") or {}).get("basis") for row in members} - {None}
        # A1 c: range envelopes are exempt; mechanics and subtraction are checked.
        if RANGE_BASIS in bases:
            continue
        nodal = {(row["entity_ref"], row["kind"]): row for row in members if row["kind"].startswith("global_nodal_displacement_")}
        supports: dict[tuple[str, str], Mapping[str, Any]] = {}
        for row in members:
            if row["kind"] in SUPPORT_KINDS:
                key = (row["entity_ref"], (row.get("metadata") or {}).get("component"))
                _require(key not in supports, "duplicate combined support component")
                supports[key] = row
        for row in members:
            if row["kind"] == "displacement_magnitude":
                parts = [nodal.get((row["entity_ref"], f"global_nodal_displacement_{axis}")) for axis in "xyz"]
                _require(all(parts) and _consistent_norm(row["value"], [part["value"] for part in parts]), "combined displacement magnitude inconsistent with components")
            elif row["kind"] in {SUPPORT_FORCE_KIND, SUPPORT_MOMENT_KIND}:
                names = SUPPORT_COMPONENTS[:3] if row["kind"] == SUPPORT_FORCE_KIND else SUPPORT_COMPONENTS[3:]
                parts = [supports.get((row["entity_ref"], name)) for name in names]
                _require(all(parts) and _consistent_norm(row["value"], [part["value"] for part in parts]), "combined support magnitude inconsistent with components")


def validate_transport_metadata(source: Mapping[str, Any]) -> None:
    """Closed statement shape and internal consistency only; creates no row evidence.

    A retained package carries the statement without the raw rows, so the row
    joins of ``validate_preview_physics_evidence`` are checked only when the
    supplied raw envelope is validated separately.
    """
    from .source_blocks import _shape as schema_shape
    try:
        _require(not any(key in source for key in ("source_block_recovery", "carrier_evidence")), "unsupported source namespace")
        formulation = source.get("formulation_basis")
        _shape(formulation, {"profile_id", "limitations"}, "formulation basis")
        _require(formulation["profile_id"] == PROFILE and formulation["limitations"] == LIMITATIONS, "formulation profile or limitations")
        schema = json.loads((ROOT / "schemas/results.v0.3.schema.yaml").read_text(encoding="utf-8"))
        evidence = source.get("contract_evidence")
        _require(schema_shape(evidence, schema["$defs"]["PreviewPhysicsContractEvidence"], schema), "transport evidence shape")
        _finite_tree(evidence)
        _cases(evidence)
        _gates(evidence)
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise ValueError("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error
