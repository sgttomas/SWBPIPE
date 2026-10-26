"""The combination pass of pressure-2 (W-16 slice b): an independent test-only verifier.

Mirror of the Rust reader ``core/reporting/result_export/src/exact_combination_evidence.rs``
(DEL-05-02 Design ``exact-combinations.md`` section 6), the only product reader. The direct-case
checks run on a view without the combination rows and the ``exact_combinations`` record
(``direct_view``); this pass then re-derives every combination of the received document from its
record's complete expression, the operand rows and the case evidence, and compares complete
coverage: the published combination rows in order (every field and value bit), the operand
sources and the ``pressure_free_operand_recovery`` witnesses, the gate, the operands' standing,
the missing sources and withheld rows, and the diagnostics that state them. ``model_checks``
binds each record to the captured model (G2).

Independence: standard-library imports only, and its sibling ``section_basis`` (bit equality).
Its algebra is its own and exact: a linear combination is the exact rational sum of the
represented factors times the operand values (``fractions.Fraction``), rounded once to nearest
by integer division (an exact zero is +0.0); a subtraction is one binary64 difference; a range
envelope selects by mode, ties to the smallest operand id; a recomputed magnitude is the
correctly rounded square root of the exact sum of squares; gate 3's unit factor-sum rule is
decided on the exact rational sum. Internal source consistency only: never producer origin,
solver accuracy or numerical eligibility (combination rows never enter it).
"""
from __future__ import annotations

from collections.abc import Mapping
from copy import deepcopy
from fractions import Fraction
import math
from typing import Any

from .section_basis import bit_equal

STANDING = "derived_from_case_results"
RECOVERY_METHOD = "pressure_free_operand_recovery"
SOURCE_RECOVERY_EXCLUSION = ("excluded: a document with a load combination attempts no retained-source recovery for any of its "
                             "cases, primitive cases included; every case publishes ordinary structural recovery")
MODULUS_BASIS_MIXED = "COMBINATION_MODULUS_BASIS_MIXED"
RESOLVED_STIFFNESS_MIXED = "COMBINATION_RESOLVED_STIFFNESS_MIXED"
INSTALLED_REFERENCE = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"
DERIVED = "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS"
SOURCE_RECOVERY_EXCLUDED = "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED"
SOURCE_MISSING = "LOAD_COMBINATION_SOURCE_RESULT_MISSING"
ROW_WITHHELD = "LOAD_COMBINATION_ROW_WITHHELD"
MAXIMUM_UNAVAILABLE = "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE"
INTENSIFIED_UNAVAILABLE = "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE"
PREFIX = "diagnostic:exact-combination:"
EXCLUDED_ID = "diagnostic:exact-combination:source-recovery-excluded"
RECORD_KEYS = {"combination_id", "basis", "expression", "operands", "gate", "rows", "recoveries", "missing_sources",
               "withheld", "standing", "source_recovery"}
OPERAND_KEYS = {"load_case_id", "modulus_basis_ref", "modulus_basis_temperature", "numerical_quality"}
SEPARATOR = "; section_basis="
BASES = {"mechanics": "explicit_user_linear_combination", "result_state_subtraction": "explicit_user_result_state_subtraction",
         "range_envelope": "explicit_user_range_envelope"}
LINEAR_SIGN = "positive value follows explicit user linear combination of matching source result sign conventions"
SUBTRACTION_SIGN = ("positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source "
                    "result sign conventions; a signed difference of solved states, not an equilibrium state")
MODULUS_SIGN = "presence record; each range operand's solve basis is recorded explicitly"
MODES = ("min", "max", "min_abs", "max_abs")
TIE_ROD = "connector_tie_rod_force_v1"
SUPPORT_COMPONENT = "support_reaction_component_v2"
SUPPORT_FORCE = "support_reaction_force_magnitude_v2"
SUPPORT_MOMENT = "support_reaction_moment_magnitude_v2"
SUPPORT_COMPONENTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
DISPLACEMENT = (("global_nodal_displacement_x", "nodal_displacement_x"), ("global_nodal_displacement_y", "nodal_displacement_y"),
                ("global_nodal_displacement_z", "nodal_displacement_z"))
ZERO_AT_NO_PRESSURE = {"pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2", "pipe_toroidal_membrane_hoop_stress_v1",
                       "pipe_toroidal_radial_surface_stress_v1"}
WALL, EFFECTIVE, END_ACTION, MEMBRANE = ("pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2",
                                         "pipe_axial_membrane_stress_v2")
AXIAL_FORCE = "element_local_axial_force"
TIE_ROD_ROW, MISSING_KEY_FIELDS, DIMENSIONLESS = "tie-rod row", "row missing key fields", "dimensionless row"
ALGEBRA_REFUSED, SUPPORT_SOURCE_MISSING = "load-case algebra refused", "source support row missing in an operand case"
MAXIMUM_KINDS = {"pipe_elastic_normal_stress_maximum_v2", "element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"}
INTENSIFIED_KINDS = {"curved_bend_directional_intensified_bending_stress_v1", "curved_bend_directional_intensified_bending_maximum_v1"}
UNITS = {"mm", "rad", "N", "N*m", "MPa", "Pa", "m"}
BOUND = Fraction(1, 2 ** 46)


def _fail(code: str) -> ValueError:
    return ValueError(f"SOURCE_PHYSICS_COMBINATION_{code}")


def _require(ok: bool, code: str) -> None:
    if not ok:
        raise _fail(code)


def _keys(value: Any, keys: set[str]) -> bool:
    return isinstance(value, Mapping) and set(value) == keys


def _text(value: Any, code: str) -> str:
    _require(isinstance(value, str) and bool(value), code)
    return value


def _items(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def identity(parts: list[str]) -> str:
    """Each part length-prefixed by its UTF-8 byte length (Rust ``str::len``)."""
    return "".join(f"{len(part.encode('utf-8'))}:{part}" for part in parts)


def _stable_suffix(value: str) -> str:
    return value.replace(":", "-")


def _is_combination(row: Any) -> bool:
    return isinstance(row, Mapping) and isinstance(row.get("basis_ref"), Mapping) and row["basis_ref"].get("ref_type") == "combination"


def _case_of(row: Any) -> str | None:
    basis = row.get("basis_ref") if isinstance(row, Mapping) else None
    if isinstance(basis, Mapping) and basis.get("ref_type") == "load_case" and isinstance(basis.get("ref_id"), str):
        return basis["ref_id"]
    return None


def present(source: Mapping[str, Any]) -> bool:
    evidence = source.get("contract_evidence")
    return (isinstance(evidence, Mapping) and "exact_combinations" in evidence) or any(_is_combination(r) for r in _items(source.get("results")))


def direct_view(source: Mapping[str, Any]) -> dict[str, Any]:
    evidence = source.get("contract_evidence")
    _require(isinstance(evidence, Mapping) and "exact_combinations" in evidence, "EVIDENCE_MISSING")
    records = evidence["exact_combinations"]
    _require(isinstance(records, list) and bool(records), "EVIDENCE_SHAPE")
    view = deepcopy(dict(source))
    view["contract_evidence"] = {k: v for k, v in view["contract_evidence"].items() if k != "exact_combinations"}
    view["results"] = [r for r in _items(view.get("results")) if not _is_combination(r)]
    return view


# ----------------------------------------------------------------------------- exact arithmetic

def _round(value: Fraction) -> float | None:
    """The rational rounded once to the nearest double (ties to even); +0.0 for zero; None beyond range."""
    if value == 0:
        return 0.0
    try:
        out = value.numerator / value.denominator  # CPython: correctly rounded integer division
    except OverflowError:
        return None
    return out + 0.0 if out == 0 else out


def _sqrt_rounded(q: Fraction) -> float:
    """The correctly rounded square root of a nonnegative rational."""
    if q == 0:
        return 0.0
    d = math.sqrt(float(q))
    for _ in range(4):
        up, down = math.nextafter(d, math.inf), math.nextafter(d, 0.0)
        mid_up = (Fraction(d) + Fraction(up)) / 2
        mid_down = (Fraction(d) + Fraction(down)) / 2
        if mid_up * mid_up < q or (mid_up * mid_up == q and (struct_bits(up) & 1) == 0):
            d = up
        elif mid_down * mid_down > q or (mid_down * mid_down == q and (struct_bits(down) & 1) == 0):
            d = down
        else:
            return d
    return d


def struct_bits(x: float) -> int:
    import struct
    return struct.unpack(">Q", struct.pack(">d", x))[0]


def norm3(x: float, y: float, z: float) -> float:
    return _sqrt_rounded(Fraction(x) ** 2 + Fraction(y) ** 2 + Fraction(z) ** 2)


def unit_factor_sum(factors: list[float]) -> bool:
    if not all(isinstance(f, (int, float)) and math.isfinite(f) for f in factors):
        return False
    return abs(sum((Fraction(f) for f in factors), Fraction(0)) - 1) <= BOUND


class Expression:
    def __init__(self, basis: str, value: Any):
        self.basis = basis
        if basis == "mechanics":
            _require(_keys(value, {"terms"}) and isinstance(value["terms"], list) and value["terms"], "EXPRESSION_SHAPE")
            self.terms = []
            for term in value["terms"]:
                _require(_keys(term, {"load_case_id", "factor"}), "EXPRESSION_SHAPE")
                factor = term["factor"]
                if not (type(factor) in (int, float) and math.isfinite(factor)):
                    raise _fail("EXPRESSION_FACTOR")
                case = _text(term["load_case_id"], "EXPRESSION_SHAPE")
                _require(all(c != case for c, _ in self.terms), "EXPRESSION_DUPLICATE")
                self.terms.append((case, float(factor)))
            self.operands = [c for c, _ in self.terms]
        elif basis == "result_state_subtraction":
            _require(_keys(value, {"minuend_id", "subtrahend_id"}), "EXPRESSION_SHAPE")
            m, s = _text(value["minuend_id"], "EXPRESSION_SHAPE"), _text(value["subtrahend_id"], "EXPRESSION_SHAPE")
            _require(m != s, "EXPRESSION_DUPLICATE")
            self.operands = [m, s]
            self.authored = [m, s]
        elif basis == "range_envelope":
            _require(_keys(value, {"operand_ids", "mode"}) and isinstance(value["operand_ids"], list), "EXPRESSION_SHAPE")
            authored: list[str] = []
            for item in value["operand_ids"]:
                item = _text(item, "EXPRESSION_SHAPE")
                _require(item not in authored, "EXPRESSION_DUPLICATE")
                authored.append(item)
            _require(bool(authored), "EXPRESSION_SHAPE")
            _require(value["mode"] in MODES, "EXPRESSION_MODE")
            self.mode = value["mode"]
            self.authored = authored
            self.operands = sorted(authored)
        else:
            raise _fail("BASIS")

    @property
    def range(self) -> bool:
        return self.basis == "range_envelope"

    def sign(self) -> str:
        if self.basis == "mechanics":
            return LINEAR_SIGN
        if self.basis == "result_state_subtraction":
            return SUBTRACTION_SIGN
        return (f"value is the {self.mode} mode-selected source result across explicit user range-envelope operands and keeps that "
                "source result sign convention")

    def evaluate(self, values: list[float]) -> float | None:
        if self.basis == "mechanics":
            return _round(sum((Fraction(f) * Fraction(v) for (_, f), v in zip(self.terms, values)), Fraction(0)))
        if self.basis == "result_state_subtraction":
            return values[0] - values[1]
        best = values[0]
        for value in values[1:]:
            better = {"min": value < best, "max": value > best, "min_abs": abs(value) < abs(best),
                      "max_abs": abs(value) > abs(best)}[self.mode]
            if better:
                best = value
        return best


# ----------------------------------------------------------------------------- keys and facts

def key_of(row: Mapping[str, Any]) -> tuple | None:
    metadata = row.get("metadata")
    if isinstance(metadata, Mapping):
        basis = metadata.get("basis")
        fields = [metadata.get("location"), metadata.get("component"), metadata.get("coordinate_system")]
        if not isinstance(basis, str) or not all(isinstance(f, str) for f in fields):
            return None
        section = basis.split(SEPARATOR, 1)[1] if SEPARATOR in basis else ""
        location, component, frame = fields
    elif row.get("kind") == "displacement_magnitude" and "metadata" not in row:
        location = component = frame = section = ""
    else:
        return None
    if not all(isinstance(row.get(k), str) for k in ("kind", "unit", "entity_ref")):
        return None
    return (row["kind"], row["unit"], row["entity_ref"], location, component, frame, section)


def combined_id(combination: str, key: tuple) -> str:
    return "result:combination:" + identity([combination, *key])


def _case_rows(results: list[Any], case: str) -> tuple[dict, list]:
    by_key: dict = {}
    order = [r for r in results if _case_of(r) == case]
    for row in order:
        key = key_of(row)
        if key is not None:
            by_key.setdefault(key, []).append(row)
    return by_key, order


def _case_facts(evidence: Mapping[str, Any], case: str) -> dict[str, Any]:
    exact = next((c for c in _items(evidence.get("exact_cases")) if isinstance(c, Mapping) and c.get("load_case_id") == case), None)

    def indexed(key):
        return [(item["pipe_id"], item) for item in _items(exact.get(key) if exact else None) if isinstance(item, Mapping) and isinstance(item.get("pipe_id"), str)]
    materials, sections = indexed("pipe_materials"), indexed("pipe_sections")
    members: list[str] = []
    for pipe, _ in materials + sections:
        if pipe not in members:
            members.append(pipe)
    region_members = {m for r in _items(evidence.get("pressure")) if isinstance(r, Mapping) and r.get("load_case_id") == case
                      for m in _items(r.get("member_pipe_ids")) if isinstance(m, str)}
    record = next((r for r in _items(evidence.get("load_reference_states")) if isinstance(r, Mapping) and r.get("load_case_id") == case), None)
    states = {m["pipe_id"]: m for m in _items(record.get("members") if record else None) if isinstance(m, Mapping) and isinstance(m.get("pipe_id"), str)}
    return {"sections": dict(sections), "materials": dict(materials), "material_basis": exact.get("material_basis") if exact else None,
            "regions": region_members, "states": states, "members": members}


def _value_eq(a: Any, b: Any) -> bool:
    """Structural equality with numbers compared as binary64 values (the producer's gate equality)."""
    numbers = (int, float)
    if type(a) in numbers and type(b) in numbers and not isinstance(a, bool) and not isinstance(b, bool):
        return float(a) == float(b)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(_value_eq(x, y) for x, y in zip(a, b))
    if isinstance(a, Mapping) and isinstance(b, Mapping):
        return a.keys() == b.keys() and all(_value_eq(a[k], b[k]) for k in a)
    return type(a) is type(b) and a == b


def _stiffness(facts: Mapping[str, Any], pipe: str) -> dict[str, Any]:
    material = facts["materials"].get(pipe) or {}
    state = facts["states"].get(pipe)
    return {"E": material.get("E_pa"), "nu": material.get("nu"), "G": material.get("G_pa"), "material": material.get("material_id"),
            "constitutive": material.get("constitutive_basis"), "selection": material.get("material_selection_kind"),
            "case_basis": facts["material_basis"],
            "state": None if state is None else [state.get(k) for k in ("material_selection_kind", "material_selection_temperature_k",
                                                                         "interpolation_fraction", "selected_E_pa", "selected_nu")],
            "section": facts["sections"].get(pipe)}


def _installed_reference(connector: Any) -> bool:
    for record in _items(connector):
        if not isinstance(record, Mapping):
            continue
        if any(type(v) in (int, float) and v != 0 for v in _items(record.get("q_ref"))):
            return True
        if any(isinstance(rod, Mapping) and type(rod.get("reference_extension_m")) in (int, float) and rod["reference_extension_m"] != 0
               for rod in _items(record.get("tie_rods"))):
            return True
    return False


def _modulus_key(operand: Mapping[str, Any]) -> str | None:
    reference, temperature = operand["modulus_basis_ref"], operand["modulus_basis_temperature"]
    if reference is None and temperature is None:
        return None
    if temperature is None:
        return "exact:" + _text(reference, "OPERAND_SHAPE")
    _require(reference is None and _keys(temperature, {"value", "unit"}), "OPERAND_SHAPE")
    _text(temperature["unit"], "OPERAND_SHAPE")
    value = temperature["value"]
    _require(type(value) in (int, float) and math.isfinite(value), "OPERAND_SHAPE")
    return f"temperature_kelvin_bits:{struct_bits(float(value)):016x}"


def _gate(expression: Expression, operands: list[Mapping[str, Any]], facts: list[dict], connector: Any) -> str | None:
    if expression.basis != "mechanics":
        return None
    if len({_modulus_key(o) for o in operands}) > 1:
        return MODULUS_BASIS_MIXED
    members: list[str] = []
    for f in facts:
        for pipe in f["members"]:
            if pipe not in members:
                members.append(pipe)
    for pipe in members:
        first = _stiffness(facts[0], pipe)
        if any(not _value_eq(_stiffness(f, pipe), first) for f in facts[1:]):
            return RESOLVED_STIFFNESS_MIXED
    if _installed_reference(connector) and not unit_factor_sum([f for _, f in expression.terms]):
        return INSTALLED_REFERENCE
    return None


# ----------------------------------------------------------------------------- derivation

def _recover(key: tuple, case: str, by_key: dict, facts: Mapping[str, Any]) -> tuple[float, dict] | None:
    kind, _, pipe, location, component, _, section = key
    if pipe not in facts["sections"] or pipe in facts["regions"]:
        return None

    def witness(source, sign, area, value):
        return {"load_case_id": case, "pipe_id": pipe, "kind": kind, "component": component, "location": location,
                "section_basis": section or None, "method": RECOVERY_METHOD,
                "source_result_id": source["id"] if source is not None else None, "sign": sign, "area_m2": area,
                "rounding": "one_binary64_division" if area is not None else "none", "value": value}
    if kind in ZERO_AT_NO_PRESSURE:
        return 0.0, witness(None, "none", None, 0.0)
    found = by_key.get((AXIAL_FORCE, "N", pipe, location, "axial_force", "element_local", ""))
    if kind not in (END_ACTION, WALL, EFFECTIVE, MEMBRANE) or not found or len(found) != 1:
        return None
    source = found[0]
    value = float(source["value"])
    if kind == END_ACTION:
        return value, witness(source, "unchanged", None, value)
    # 0 - N, as the producer negates: a negated zero is +0.0.
    n, sign = (0.0 - value, "negated") if location == "end_i" else (value, "unchanged")
    if kind == MEMBRANE:
        leaf = facts["sections"][pipe].get(section) if isinstance(facts["sections"][pipe], Mapping) else None
        area = leaf.get("As_m2") if isinstance(leaf, Mapping) else None
        if type(area) not in (int, float):
            return None
        area = float(area)
        value = n / area
        return value, witness(source, sign, area, value)
    return n, witness(source, sign, None, n)


def _combined_row(combination: str, expression: Expression, template: Mapping[str, Any], key: tuple, value: float, refs: list[str]) -> dict:
    row = deepcopy(dict(template))
    row["id"] = combined_id(combination, key)
    row["value"] = value
    row["basis_ref"] = {"ref_type": "combination", "ref_id": combination}
    if refs:
        row["source_result_refs"] = list(refs)
    else:
        row.pop("source_result_refs", None)
    if isinstance(row.get("metadata"), Mapping):
        row["metadata"] = dict(row["metadata"])
        if not key[6]:
            row["metadata"]["basis"] = BASES[expression.basis]
        row["metadata"]["sign_convention"] = expression.sign()
    return row


def _derive(combination: str, expression: Expression, results: list[Any], evidence: Mapping[str, Any],
            linear: list[str], recomputed: list[str]) -> dict[str, list]:
    operands = expression.operands
    cases = [_case_rows(results, c) for c in operands]
    facts = [_case_facts(evidence, c) for c in operands]
    built: dict[str, list] = {"rows": [], "row_evidence": [], "recoveries": [], "missing": [], "withheld": []}

    def withhold(name: str, reason: str) -> None:
        if not any(w["name"] == name for w in built["withheld"]):
            built["withheld"].append({"name": name, "reason": reason})
    support = {SUPPORT_COMPONENT, SUPPORT_FORCE, SUPPORT_MOMENT}
    order: list[tuple] = []
    seen: set = set()
    for _, rows in cases:
        for row in rows:
            kind = row.get("kind")
            if kind not in linear and kind not in recomputed or kind in support:
                continue
            key = key_of(row)
            if key is None:
                withhold(row["id"], MISSING_KEY_FIELDS)
                continue
            if key not in seen:
                seen.add(key)
                order.append((key, row))
    outcomes: dict = {}
    for key, template in order:
        if key[0] in recomputed and not expression.range:
            continue
        several = next((found for by_key, _ in cases if len(found := by_key.get(key, [])) > 1), None)
        if several is not None:
            reason = TIE_ROD_ROW if key[0] == TIE_ROD else MISSING_KEY_FIELDS
            for row in several:
                withhold(row["id"], reason)
            outcomes[key] = ("withheld",)
            continue
        if key[0] == TIE_ROD:
            withhold(template["id"], TIE_ROD_ROW)
            outcomes[key] = ("withheld",)
            continue
        sources, refs, values, missing, witnesses = [], [], [], [], []
        for (by_key, _), fact, case in zip(cases, facts, operands):
            found = by_key.get(key)
            if found:
                values.append(float(found[0]["value"]))
                sources.append({"load_case_id": case, "result_id": found[0]["id"]})
                refs.append(found[0]["id"])
                continue
            recovered = _recover(key, case, by_key, fact)
            if recovered is None:
                missing.append(case)
                continue
            value, witness = recovered
            values.append(value)
            sources.append({"load_case_id": case, "recovery_index": len(built["recoveries"]) + len(witnesses)})
            if witness["source_result_id"] is not None:
                refs.append(witness["source_result_id"])
            witnesses.append(witness)
        if missing:
            outcomes[key] = ("missing", missing)
            continue
        if key[1] not in UNITS:
            withhold(template["id"], DIMENSIONLESS)
            outcomes[key] = ("withheld",)
            continue
        value = expression.evaluate(values)
        if value is None:
            withhold(template["id"], ALGEBRA_REFUSED)
            outcomes[key] = ("withheld",)
            continue
        built["recoveries"].extend(witnesses)
        outcomes[key] = ("row", value, sources, refs)
    for key, template in order:
        if key[0] in recomputed and not expression.range:
            components = [(kind, key[1], key[2], "node", component, "global", "") for kind, component in DISPLACEMENT]
            vector, missing, refused = [0.0, 0.0, 0.0], [], False
            for slot, component in enumerate(components):
                outcome = outcomes.get(component)
                if outcome is not None and outcome[0] == "row":
                    vector[slot] = outcome[1]
                elif outcome is not None and outcome[0] == "withheld":
                    refused = True
                else:
                    for case in (outcome[1] if outcome is not None else operands):
                        if case not in missing:
                            missing.append(case)
            if missing:
                built["missing"].append({"source_result_id": template["id"], "missing_load_case_ids": missing})
                continue
            if refused:
                withhold(template["id"], ALGEBRA_REFUSED)
                continue
            row = _combined_row(combination, expression, template, key, norm3(*vector), [combined_id(combination, c) for c in components])
            built["row_evidence"].append({"result_id": row["id"], "sources": []})
            built["rows"].append(row)
            continue
        outcome = outcomes.pop(key, None)
        if outcome is None or outcome[0] == "withheld":
            continue
        if outcome[0] == "missing":
            built["missing"].append({"source_result_id": template["id"], "missing_load_case_ids": outcome[1]})
            continue
        _, value, sources, refs = outcome
        row = _combined_row(combination, expression, template, key, value, refs)
        built["row_evidence"].append({"result_id": row["id"], "sources": sources})
        built["rows"].append(row)
    supports: list[str] = []
    for _, rows in cases:
        for row in rows:
            if row.get("kind") == SUPPORT_COMPONENT and (row.get("metadata") or {}).get("component") == "Fx" and isinstance(row.get("entity_ref"), str):
                if row["entity_ref"] not in supports:
                    supports.append(row["entity_ref"])
    for entity in supports:
        def find(index, kind, component):
            return next((r for r in cases[index][1] if r.get("kind") == kind and r.get("entity_ref") == entity
                         and (r.get("metadata") or {}).get("component") == component), None)
        try:
            group = _support_group(combination, expression, operands, find)
        except _Withheld as reason:
            withhold(f"support-action:{len(entity.encode('utf-8'))}:{entity}", str(reason))
            continue
        for row, row_evidence in group:
            built["rows"].append(row)
            built["row_evidence"].append(row_evidence)
    return built


class _Withheld(Exception):
    pass


def _support_group(combination, expression, operands, find):
    out, combined, component_ids = [], [0.0] * 6, []
    for slot, component in enumerate(SUPPORT_COMPONENTS):
        sources = [find(i, SUPPORT_COMPONENT, component) for i in range(len(operands))]
        if any(s is None for s in sources):
            raise _Withheld(SUPPORT_SOURCE_MISSING)
        value = expression.evaluate([float(s["value"]) for s in sources])
        if value is None:
            raise _Withheld(ALGEBRA_REFUSED)
        combined[slot] = value
        key = key_of(sources[0])
        if key is None:
            raise _Withheld(SUPPORT_SOURCE_MISSING)
        refs = [s["id"] for s in sources]
        row = _combined_row(combination, expression, sources[0], key, value, refs)
        component_ids.append(row["id"])
        out.append((row, {"result_id": row["id"], "sources": [{"load_case_id": c, "result_id": r} for c, r in zip(operands, refs)]}))
    for kind, component, slots in ((SUPPORT_FORCE, "force_magnitude", slice(0, 3)), (SUPPORT_MOMENT, "moment_magnitude", slice(3, 6))):
        sources = [s for s in (find(i, kind, component) for i in range(len(operands))) if s is not None]
        if not sources:
            raise _Withheld(SUPPORT_SOURCE_MISSING)
        key = key_of(sources[0])
        if key is None:
            raise _Withheld(SUPPORT_SOURCE_MISSING)
        if expression.range:
            if len(sources) != len(operands):
                raise _Withheld(SUPPORT_SOURCE_MISSING)
            value = expression.evaluate([float(s["value"]) for s in sources])
            if value is None:
                raise _Withheld(ALGEBRA_REFUSED)
            refs = [s["id"] for s in sources]
            row_sources = [{"load_case_id": c, "result_id": r} for c, r in zip(operands, refs)]
        else:
            value, refs, row_sources = norm3(*combined[slots]), component_ids[slots], []
        row = _combined_row(combination, expression, sources[0], key, value, refs)
        out.append((row, {"result_id": row["id"], "sources": row_sources}))
    return out


# ----------------------------------------------------------------------------- the pass

def validate(source: Mapping[str, Any], table: Mapping[str, Any]) -> None:
    evidence = source["contract_evidence"]
    records = evidence.get("exact_combinations")
    _require(isinstance(records, list) and bool(records), "EVIDENCE_SHAPE")
    policy = table.get("combination_policy") or {}
    linear, recomputed = list(policy.get("linear_state_kinds") or []), list(policy.get("recomputed_magnitude_kinds") or [])
    _require(bool(linear) and bool(recomputed), "POLICY")
    results = source["results"]
    _require(isinstance(results, list), "ROWS")
    diagnostics = source["diagnostics"]
    _require(isinstance(diagnostics, list), "DIAGNOSTICS")
    quality = _items((source.get("numerical_quality") or {}).get("cases"))
    case_ids = {c.get("load_case_id") for c in _items(evidence.get("exact_cases")) if isinstance(c, Mapping)}
    has_maximum = any(_case_of(r) is not None and r.get("kind") in MAXIMUM_KINDS for r in results)
    has_intensified = any(_case_of(r) is not None and r.get("kind") in INTENSIFIED_KINDS for r in results)
    expected_rows, modulus_rows, expected_diagnostics, ids = [], [], [], []
    for record in records:
        _require(_keys(record, RECORD_KEYS), "RECORD_SHAPE")
        cid = _text(record["combination_id"], "RECORD_SHAPE")
        _require(cid not in ids and cid not in case_ids, "ID_DUPLICATE")
        ids.append(cid)
        basis = _text(record["basis"], "BASIS")
        expression = Expression(basis, record["expression"])
        _require(all(c in case_ids for c in expression.operands), "OPERAND_UNRESOLVED")
        operands = record["operands"]
        _require(isinstance(operands, list) and len(operands) == len(expression.operands), "OPERAND_SHAPE")
        for operand, case in zip(operands, expression.operands):
            _require(_keys(operand, OPERAND_KEYS) and operand["load_case_id"] == case, "OPERAND_SHAPE")
            _modulus_key(operand)
            stated = next((q for q in quality if isinstance(q, Mapping) and q.get("basis_ref") == {"ref_type": "load_case", "ref_id": case}), None)
            _require(stated is not None and bit_equal(stated, operand["numerical_quality"]), "OPERAND_STANDING")
        _require(record["standing"] == STANDING, "STANDING")
        _require(record["source_recovery"] == SOURCE_RECOVERY_EXCLUSION, "SOURCE_RECOVERY")
        facts = [_case_facts(evidence, c) for c in expression.operands]
        reason = _gate(expression, operands, facts, evidence.get("connector"))
        _require(bit_equal(record["gate"], {"withheld": reason is not None, "reason": reason}), "GATE")
        if basis != "mechanics":
            for case in expression.authored:
                label = next((r["metadata"]["basis"] for r in reversed(results) if r.get("kind") == "modulus_basis_record"
                              and r.get("entity_ref") == case and _case_of(r) is not None), "material_base_values")
                modulus_rows.append({"id": f"result:combination:{_stable_suffix(cid)}:modulus-basis:{_stable_suffix(case)}",
                                     "kind": "combination_modulus_basis_record", "value": 1.0, "unit": "record", "entity_ref": case,
                                     "basis_ref": {"ref_type": "combination", "ref_id": cid},
                                     "metadata": {"component": "material_modulus_basis", "coordinate_system": "not_applicable",
                                                  "location": cid, "basis": label, "sign_convention": MODULUS_SIGN}})
        if reason is not None:
            for member in ("rows", "recoveries", "missing_sources", "withheld"):
                _require(record[member] == [], "GATED_RECORD")
            expected_diagnostics.append((f"{PREFIX}gate:{identity([cid])}", reason, "warning"))
            continue
        built = _derive(cid, expression, results, evidence, linear, recomputed)
        _require(bit_equal(record["rows"], built["row_evidence"]), "ROW_EVIDENCE")
        _require(bit_equal(record["recoveries"], built["recoveries"]), "RECOVERY")
        _require(bit_equal(record["missing_sources"], built["missing"]), "MISSING_SOURCES")
        _require(bit_equal(record["withheld"], built["withheld"]), "WITHHELD")
        for missing in built["missing"]:
            expected_diagnostics.append((f"{PREFIX}source-missing:{identity([cid, missing['source_result_id']])}", SOURCE_MISSING, "warning"))
        if built["withheld"]:
            expected_diagnostics.append((f"{PREFIX}row-withheld:{identity([cid])}", ROW_WITHHELD, "warning"))
        expected_diagnostics.append((f"{PREFIX}derived:{identity([cid])}", DERIVED, "info"))
        if has_maximum:
            expected_diagnostics.append((f"{PREFIX}maximum:{identity([cid])}", MAXIMUM_UNAVAILABLE, "info"))
        if has_intensified:
            expected_diagnostics.append((f"{PREFIX}intensified:{identity([cid])}", INTENSIFIED_UNAVAILABLE, "info"))
        expected_rows.extend(built["rows"])
    expected_rows.extend(modulus_rows)
    published = [r for r in results if _is_combination(r)]
    _require(len(published) == len(expected_rows), "ROW_COVERAGE")
    for row, expected in zip(published, expected_rows):
        _require(bit_equal(row, expected), "ROW")
    stated = [d for d in diagnostics if isinstance(d, Mapping) and isinstance(d.get("id"), str) and d["id"].startswith(PREFIX)]
    _require(len(stated) == len(expected_diagnostics) + 1, "DIAGNOSTICS")
    for did, code, severity in expected_diagnostics:
        matching = [d for d in stated if d["id"] == did]
        _require(len(matching) == 1 and matching[0].get("code") == code and matching[0].get("severity") == severity, "DIAGNOSTICS")
    excluded = [d for d in stated if d["id"] == EXCLUDED_ID]
    _require(len(excluded) == 1 and excluded[0].get("code") == SOURCE_RECOVERY_EXCLUDED and excluded[0].get("severity") == "info"
             and excluded[0].get("affected_refs") == ids, "DIAGNOSTICS")


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """G2: one record per model combination, in model order, with the model's expression and operand selections."""
    if model.get("combinations") is None:
        return
    combinations = _items(model["combinations"])
    records = _items((source.get("contract_evidence") or {}).get("exact_combinations"))
    _require(len(records) == len(combinations), "MODEL_COVERAGE")
    cases = _items(model.get("load_cases"))
    for record, combination in zip(records, combinations):
        _require(record.get("combination_id") == combination.get("id") and record.get("basis") == combination.get("basis"), "MODEL_EXPRESSION")
        if combination.get("basis") == "mechanics":
            expression = {"terms": [{"load_case_id": t.get("load_case"), "factor": t.get("factor")} for t in _items(combination.get("terms"))]}
        elif combination.get("basis") == "result_state_subtraction":
            expression = {"minuend_id": combination.get("minuend_id"), "subtrahend_id": combination.get("subtrahend_id")}
        else:
            expression = {"operand_ids": combination.get("operand_ids"), "mode": combination.get("mode")}
        _require(bit_equal(record.get("expression"), expression), "MODEL_EXPRESSION")
        for operand in _items(record.get("operands")):
            case = next((c for c in cases if isinstance(c, Mapping) and c.get("id") == operand.get("load_case_id")), None)
            _require(case is not None, "MODEL_OPERAND")
            temperature = case.get("modulus_basis_temperature")
            temperature = None if temperature is None else {"value": temperature.get("value"), "unit": temperature.get("unit")}
            _require(bit_equal(operand.get("modulus_basis_ref"), case.get("modulus_basis_ref"))
                     and bit_equal(operand.get("modulus_basis_temperature"), temperature), "MODEL_OPERAND")
