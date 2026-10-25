"""B3's Python reader controls: B3a's dropped 0.3.0 legacy namespace (G8 refuses it) and B3b's `<physics-retained>` branch.

The bases are synthetic receipts derived from PP's pinned milestone successor (D-U6-5) and resealed by the 07e format
rule (preparation hashes, source identities, publication, receipt). They are reader controls, not producer execution or
native Current evidence. Since lane P's B3b-P landed, every B3b shape is also read on its producer-solved m3x
successors (`EXACT_PINNED`). B3a is dropped (RR "Owner decisions: the legacy pressure contract is retired
product-wide; …"): m3l has no producer fixture (PP refuses it at D1.3) and is a refusal witness here.
- m3l: the milestone's request with schema 0.3.0 and {"version": "1.0.0", "mode": "legacy_pressure_v1"}, as I99's
  m3l input (RR "I99's B3-W verified; …", ruling 4); its receipt differs only in the invocation digest.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys

import pytest

from tests.verifiers import retained_precision as rp
from tests.verifiers import compatibility as compatibility_verifier
from tests.verifiers import section_basis as sb
from tests.verifiers import u4b

if str(Path(__file__).resolve().parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).resolve().parent))
from retained_precision_corpus import load_corpus  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
# PP retained_facade_tests.rs `PINNED` pins the same bytes. W1's DEF-O meaning-hash move (DEL-04-08
# w1-member-load-certification.md §6.4) re-sealed only the DEF-O-bound preparation, source-identity and receipt
# hashes and replaced e3e0830d.../c24d4a1c....
PINNED = {"sparse_interactive": "67f146bd4411426b2811e5b6ce7cac7a4d22c6783b106d61566ee7454e2b59a9",
          "dense_scrutiny": "52ea3e9a29f200ada2bc7d2d73f6ee1e1eafdc5829b220c8b650f8e864e0ffc6"}
MODES = sorted(PINNED)
LEGACY = {"version": "1.0.0", "mode": "legacy_pressure_v1"}
INVOCATION = ("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")
PREPARATION = ("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH")


def milestone(mode):
    """D-U6-5: PP's pinned milestone successor bytes; returns (source, invocation)."""
    raw = (ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == PINNED[mode]
    doc = json.loads(raw)
    return doc["source"], doc["invocation"]


def reseal(source, invocation, definition_hash=rp.DEFINITION_HASH):
    """The 07e format rule on a copy: the invocation digest, each complete preparation hash with the route's
    definition hash (S-1; B3-D REVISION_01 §2), the selected cases' source identities, then publication and receipt."""
    source = deepcopy(source)
    body = source["retained_precision"]["body"]
    body["invocation"]["value"] = rp._hash("source_blocks_invocation_v1", invocation)
    for s in body["sources"]:
        prep = s["preparation"]
        if prep is not None:
            attempt = body["product_attempts"][int(prep["attempt_ref"])]
            if all(m["result"]["kind"] == "prepared" for m in attempt["preparation"]["members"]):
                prep["sha256"] = rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt, definition_hash))
    for case in body["cases"]:
        if case["status"] == "selected":
            case["source_identity_sha256"] = rp._source_hash(body["sources"][int(case["source_ref"])])
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return source


def outcome(source, invocation, transport=False):
    try:
        result = rp.validate_retained_precision_transport(deepcopy(source)) if transport else rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
        return ("pass", result["numerical_eligible"], result["standing"])
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)


def edited_invocation(invocation, change):
    invocation = deepcopy(invocation)
    change(invocation["request"]["model"])
    return invocation


# ---------------------------------------------------------------------------------------------------------------
# B3a dropped: G8 refuses 0.3.0 with the retired legacy_pressure_v1 contract (formerly B3-D §6.3; REVISION_01 §3, N-4);
# B3D-10's tightenings stay.


def m3l(mode):
    source, invocation = milestone(mode)
    invocation = edited_invocation(invocation, lambda m: m.update(schema_version="0.3.0", pressure_contract=dict(LEGACY)))
    return reseal(source, invocation), invocation


@pytest.mark.parametrize("mode", MODES)
def test_b3a_dropped_m3l_successor_is_refused(mode):
    """B3a is dropped: m3l's successor (the milestone's, resealed on the retired legacy contract) is refused at G8's
    namespace; the milestone itself, the control, stays eligible."""
    source, invocation = m3l(mode)
    assert outcome(source, invocation) == INVOCATION
    assert outcome(*milestone(mode)) == ("pass", True, "eligible")


def _element_pressure(magnitude):
    return lambda m: m["load_cases"][0]["primitive_loads"].append(
        {"id": "load:p", "category": "pressure", "dimension": "pressure", "direction": "radial",
         "magnitude": {"unit": "Pa", "value": magnitude}, "target": {"type": "element", "pipe": "M1"}, "provenance": "b3a_reader_control"})


# m3l is refused at G8's namespace step, so a mutation that keeps the legacy label is refused there first, before any
# load is read (the three load entries, PREPARATION while B3a admitted m3l); the others are outside the namespace on
# their own (0.3.0 without a contract, B3D-10; another contract on 0.3.0; a contract on 0.2.0; 0.4.0).
B3A_REFUSALS = {
    "contract mode changed": (lambda m: m["pressure_contract"].update(mode="exact_straight_pressure_v2"), INVOCATION),
    "contract version changed": (lambda m: m["pressure_contract"].update(version="1.0.1"), INVOCATION),
    "contract removed": (lambda m: m.pop("pressure_contract"), INVOCATION),
    "contract null": (lambda m: m.update(pressure_contract=None), INVOCATION),
    "contract with an extra key": (lambda m: m["pressure_contract"].update(extra="x"), INVOCATION),
    "contract the exact one": (lambda m: m.update(pressure_contract={"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), INVOCATION),
    "contract version as a number": (lambda m: m["pressure_contract"].update(version=1.0), INVOCATION),
    "schema 0.2.0 keeping the contract": (lambda m: m.update(schema_version="0.2.0"), INVOCATION),
    "schema 0.4.0 keeping the contract": (lambda m: m.update(schema_version="0.4.0"), INVOCATION),
    "a zero-magnitude element pressure load": (_element_pressure(0.0), INVOCATION),
    "a non-zero element pressure load": (_element_pressure(1000.0), INVOCATION),
    "a non-zero node force with category pressure": (lambda m: m["load_cases"][0]["primitive_loads"].append(
        {"id": "load:p", "category": "pressure", "dimension": "force", "direction": "UX", "magnitude": {"unit": "N", "value": 1.0},
         "target": {"type": "node", "node": "N1"}, "provenance": "b3a_reader_control"}), INVOCATION),
}


@pytest.mark.parametrize("label", sorted(B3A_REFUSALS))
@pytest.mark.parametrize("mode", MODES)
def test_b3a_m3l_mutations(mode, label):
    change, expected = B3A_REFUSALS[label]
    source, invocation = m3l(mode)
    invocation = edited_invocation(invocation, change)
    assert outcome(reseal(source, invocation), invocation) == expected


@pytest.mark.parametrize("value", [None, {}, False, [], "", 0, LEGACY])
def test_b3a_branch_l_admits_only_an_absent_contract(value):
    """N-4 on the current pressure-free 0.7.0: absent only (B7a/B7b, b7-plan c8d3cf3014: "The optional contract is
    absent only when its key is absent"; a present null is refused, as the Rust readers refuse it); every other value,
    including the retired legacy contract, stays refused."""
    source, invocation = milestone("sparse_interactive")
    for version in ("0.7.0",):
        fine = edited_invocation(invocation, lambda m: (m.update(schema_version=version), m.pop("pressure_contract", None)))
        assert outcome(reseal(source, fine), fine)[0] == "pass"
        bad = edited_invocation(invocation, lambda m: m.update(schema_version=version, pressure_contract=deepcopy(value)))
        assert outcome(reseal(source, bad), bad) == INVOCATION, (version, value)


@pytest.mark.parametrize("version", ["0.1.0", "0.2.0"])
@pytest.mark.parametrize("contract", [None, "absent"])
def test_preview_retired_versions_stay_refused(version, contract):
    """The format retirement does not restore a preview namespace without a contract."""
    source, invocation = milestone("sparse_interactive")
    retired = edited_invocation(invocation, lambda m: (m.update(schema_version=version), m.pop("pressure_contract", None) if contract == "absent" else m.update(pressure_contract=None)))
    assert outcome(reseal(source, retired), retired) == INVOCATION


def test_b3a_tightening_0_3_0_needs_the_legacy_contract():
    """B3D-10, as ruled for all three readers: 0.3.0 without a contract (which the producer cannot emit) is refused."""
    source, invocation = milestone("sparse_interactive")
    bare = edited_invocation(invocation, lambda m: m.update(schema_version="0.3.0"))
    assert "pressure_contract" not in bare["request"]["model"]
    assert outcome(reseal(source, bare), bare) == INVOCATION


# B3a dropped: the one table RS (`b3a_legacy_pressure_contract_namespace_at_g8`), TS (its "B3a (I101)" block) and PY
# pin alike, entry for entry: invocation edits on the shared corpus bases (07e's rule, `reseal`). Only branch L
# (current 0.7.0, pressure_contract absent or JSON null) passes. "L3" in the names is B3a's retired contract on 0.3.0,
# refused on every base and before any load is read (the former N-11 entries).
SHARED = {case["id"]: case for case in load_corpus()["cases"]}
B3A_DROPPED_BASES = ("ordinary_prepared_synthetic", "ordinary_prepared_dense_synthetic", "two_case_synthetic",
                     "u8_l0_isolated_node_sparse_interactive", "u8_l0_isolated_node_dense_scrutiny")
ORD, DENSE = B3A_DROPPED_BASES[:2]


def _schema(version):
    return lambda m: m.update(schema_version=version)


def _contract(value):
    return lambda m: m.update(pressure_contract=deepcopy(value))


def _no_contract(m):
    m.pop("pressure_contract", None)


def _zero_pressure(m):
    m["load_cases"][0]["primitive_loads"].append(
        {"id": "load:b3a-zero-pressure", "category": "pressure", "target": {"type": "element", "pipe": "pipe:fixture-span"},
         "magnitude": {"value": 0, "unit": "Pa"}, "dimension": "pressure", "provenance": "synthetic_integration_input_not_library_data"})


B3A_DROPPED_TABLE = [
    *[(f"L3 on {base}", base, [_schema("0.3.0"), _contract(LEGACY)], INVOCATION) for base in B3A_DROPPED_BASES],
    # B7a/B7b: a present null contract is authored presence, outside branch L (Rust "L: 0.7.0, contract null").
    ("L: 0.7.0, contract null", ORD, [_contract(None)], INVOCATION),
    ("L: 0.7.0, contract absent", ORD, [_schema("0.7.0"), _no_contract], "pass"),
    ("L3: mode exact_straight_pressure_v2, version 1.0.0", ORD, [_schema("0.3.0"), _contract({**LEGACY, "mode": "exact_straight_pressure_v2"})], INVOCATION),
    ("L3: version 1.0.1", ORD, [_schema("0.3.0"), _contract({**LEGACY, "version": "1.0.1"})], INVOCATION),
    ("L3: the exact contract 2.0.0", ORD, [_schema("0.3.0"), _contract({"version": "2.0.0", "mode": "exact_straight_pressure_v2"})], INVOCATION),
    ("0.3.0, contract null", ORD, [_schema("0.3.0"), _contract(None)], INVOCATION),
    ("0.3.0, contract absent", ORD, [_schema("0.3.0"), _no_contract], INVOCATION),
    ("L3: an extra key", ORD, [_schema("0.3.0"), _contract({**LEGACY, "extra": "x"})], INVOCATION),
    ("L3: an extra key valued null", ORD, [_schema("0.3.0"), _contract({**LEGACY, "extra": None})], INVOCATION),
    ("L3: version only", ORD, [_schema("0.3.0"), _contract({"version": "1.0.0"})], INVOCATION),
    ("L3: mode only", ORD, [_schema("0.3.0"), _contract({"mode": "legacy_pressure_v1"})], INVOCATION),
    ("L3: version a number", ORD, [_schema("0.3.0"), _contract({**LEGACY, "version": 1.0})], INVOCATION),
    ("L3: mode null", ORD, [_schema("0.3.0"), _contract({**LEGACY, "mode": None})], INVOCATION),
    ("0.3.0, contract {}", ORD, [_schema("0.3.0"), _contract({})], INVOCATION),
    ("0.7.0 keeping the L3 contract", ORD, [_contract(LEGACY)], INVOCATION),
    ("0.1.0 keeping the L3 contract", ORD, [_schema("0.1.0"), _contract(LEGACY)], INVOCATION),
    ("0.4.0 with the L3 contract", ORD, [_schema("0.4.0"), _contract(LEGACY)], INVOCATION),
    ("N-4: 0.7.0, contract {}", ORD, [_contract({})], INVOCATION),
    ("N-4: 0.7.0, contract false", ORD, [_contract(False)], INVOCATION),
    ("N-4: 0.7.0, contract []", ORD, [_contract([])], INVOCATION),
    ('N-4: 0.7.0, contract ""', ORD, [_contract("")], INVOCATION),
    ("N-4: 0.7.0, contract 0", ORD, [_contract(0)], INVOCATION),
    ("N-11: L3 with a zero-magnitude element pressure load", ORD, [_schema("0.3.0"), _contract(LEGACY), _zero_pressure], INVOCATION),
    ("N-11: L3 dense with a zero-magnitude element pressure load", DENSE, [_schema("0.3.0"), _contract(LEGACY), _zero_pressure], INVOCATION),
]


def test_b3a_dropped_namespace_table_has_the_rust_tests_29_entries():
    assert len(B3A_DROPPED_TABLE) == 29 and len({name for name, *_ in B3A_DROPPED_TABLE}) == 29


@pytest.mark.parametrize("name,base,changes,expected", B3A_DROPPED_TABLE, ids=[entry[0] for entry in B3A_DROPPED_TABLE])
def test_b3a_dropped_namespace_table(name, base, changes, expected):
    fixture = SHARED[base]
    invocation = edited_invocation(fixture["invocation"], lambda m: [change(m) for change in changes])
    got = outcome(reseal(fixture["source"], invocation), invocation)
    assert (got[0] if expected == "pass" else got) == expected, name


# ---------------------------------------------------------------------------------------------------------------
# B3b: the `<physics-retained>` branch (B3-D §6 with REVISION_01; RV116's rulings). The synthetic exact base is the
# milestone successor re-expressed on I99's m3x request (0ffbea35…), B7b's current exact document: model 0.7.0, the
# current contract {3.0.0, exact_pressure_v3, euler_bernoulli} with an authored zero corrosion_allowance on every pipe
# section, E and nu = 0.25 (so G_hat = E/2.5 = 80 GPa, the milestone's G, bit for bit), explicitly empty regions; the
# receipt with derived_e_nu, the exact route and DEF-E (prepared section bits unchanged, B3D-3); pressure-2's
# contract_evidence (D4 (a): profile_mode exact_pressure_v3) whose case entry states the prepared section (B3D-4); the
# preparation hash with DEF-E's H (S-1).

from core.analysis_runs import compatibility as c  # noqa: E402

EXACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
ZERO_ALLOWANCE = {"value": 0.0, "unit": "m"}
NU = 0.25
# D4 (a): the exact successor's base is pressure-2; its limitations are the committed pressure-2 envelope's.
PRESSURE_1_LIMITATIONS = json.loads((ROOT / "fixtures/results/pressure_connected_mechanics_sparse.json").read_text())["formulation_basis"]["limitations"]
UNSUPPORTED = ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
RECEIPT = ("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")
SECTION = ("G5b", "RETAINED_PRECISION_SECTION_MISMATCH")
BASE_G7 = ("G7", "SOURCE_PHYSICS_EVIDENCE_INVALID")


def m3x_invocation(invocation, nu=NU):
    def change(model):
        model.update(schema_version="0.7.0", pressure_contract=dict(EXACT))
        for pipe in model["pipe_segments"]:
            pipe["section"]["corrosion_allowance"] = dict(ZERO_ALLOWANCE)
        for material in model["materials"]:
            material.pop("shear_modulus", None)
            material.update(constitutive_basis="homogeneous_isotropic_E_nu_v1", poisson_ratio={"value": nu, "unit": "1"})
        for case in model["load_cases"]:
            case["pressure_regions"] = []
    return edited_invocation(invocation, change)


def _d5_id(row_id, basis):
    """§2.5 ids: `result:elastic-maximum:…` and `result:pressure-exact:…` end in the underscore basis, others in the
    hyphenated one."""
    underscore = row_id.startswith(("result:elastic-maximum:", "result:pressure-exact:"))
    return row_id + ":" + (basis if underscore else sb.hyphenated(basis))


def d5_rows(source):
    """T4-U6 (D-5, §2.2 and §2.5) on a preview base re-stated on the exact route, in place: each stress row is
    published on the nominal section and immediately followed by its reduced twin, with the same value (the bases
    coincide on this route); the receipt's references follow the nominal row (a reduced twin is outside the receipt's
    row universe, as W1-E stages it); the basis-free headline is null and each basis has its own."""
    renamed, rows, derived = {}, [], {}
    before = list(source["results"])
    for row in source["results"]:
        if row["kind"] in DERIVED_KINDS:
            # T4-U4b (U4B-RP): each derived not-covered row on both bases, nominal then reduced, each binding its own
            # position; σ± keep their values, τ takes the exact sign and the evidence's ν and basis annulus.
            for basis in sb.BASES:
                twin = deepcopy(row)
                twin["id"] = _d5_id(row["id"], basis)
                twin["metadata"]["basis"] = row["metadata"]["basis"] + sb.SEPARATOR + basis
                if row["kind"] == u4b.SHEAR_MAX:
                    twin["metadata"]["sign_convention"] = u4b.SHEAR_SIGN_EXACT
                    twin["value"] = _exact_tau(source, before, row)
                rows.append(twin)
            derived[row["id"]] = [_d5_id(row["id"], basis) for basis in sb.BASES]
            continue
        if row["kind"] not in sb.STRESS_KINDS:
            rows.append(row)
            continue
        for basis in sb.BASES:
            twin = deepcopy(row)
            twin["id"] = _d5_id(row["id"], basis)
            twin["metadata"]["basis"] = row["metadata"]["basis"] + sb.SEPARATOR + basis
            rows.append(twin)
        renamed[row["id"]] = _d5_id(row["id"], sb.NOMINAL)

    def rename(value):
        if isinstance(value, dict):
            return {key: rename(item) for key, item in value.items()}
        if isinstance(value, list):
            return [rename(item) for item in value]
        return renamed.get(value, value) if isinstance(value, str) else value

    source["results"] = rows
    source["retained_precision"]["body"] = rename(source["retained_precision"]["body"])
    body = source["retained_precision"]["body"]
    if derived:
        # Projection outcomes name bound rows of their case: a stress row's reduced twin shares its nominal twin's
        # row, each derived row binds its own, so the indices follow their rows.
        case_of = lambda r: (r.get("basis_ref") or {}).get("ref_id")
        bound = [r for r in rows if not (((r.get("metadata") or {}).get("basis") or "").endswith(sb.SEPARATOR + sb.REDUCED) and r["kind"] not in DERIVED_KINDS)]
        for attempt in body["product_attempts"]:
            if not isinstance(attempt, dict) or (attempt.get("owner_ref") or {}).get("kind") != "case" or not isinstance(attempt.get("proof"), dict):
                continue
            cid = body["cases"][attempt["owner_ref"]["index"]]["basis_ref"]["ref_id"]
            old_ids = [r["id"] for r in before if case_of(r) == cid]
            new_ids = [r["id"] for r in bound if case_of(r) == cid]
            for o in attempt["proof"].get("projection_outcomes") or []:
                i = o.get("row_index")
                if isinstance(i, int) and not isinstance(i, bool) and i < len(old_ids):
                    rid = old_ids[i]
                    o["row_index"] = new_ids.index(renamed.get(rid) or (derived[rid][0] if rid in derived else rid))
        for case in body["cases"]:
            selection = case.get("selection")
            if isinstance(selection, dict) and isinstance(selection.get("not_covered"), list):
                selection["not_covered"] = [x for i in selection["not_covered"] for x in derived.get(i, [i])]
    summary = {}
    for key, value in source["summary"].items():
        summary[key] = None if key == "max_open_formula_stress" else value
        if key == "max_open_formula_stress":
            for basis in sb.BASES:
                summary["max_open_formula_stress_" + basis] = None if value is None else dict(value, result_ref=_d5_id(value["result_ref"], basis))
    source["summary"] = summary


DERIVED_KINDS = (u4b.EXTREME, u4b.SHEAR_MAX)


def _exact_tau(source, rows, row):
    """τ on the exact re-statement: |V| c_b / 1e6, V from the row's case, member and station shear rows, c_b from the
    evidence's ν and the member's basis annulus (§2.4's fixed sequence)."""
    case, pipe, location = row["basis_ref"]["ref_id"], row["entity_ref"], row["metadata"]["location"]
    shear = lambda kind: next(r["value"] for r in rows if r["basis_ref"]["ref_id"] == case and r["entity_ref"] == pipe and r["kind"] == kind
                              and r["metadata"]["location"] == location)
    evidence = next(c for c in source["contract_evidence"]["exact_cases"] if c["load_case_id"] == case)
    section = next(x for x in evidence["pipe_sections"] if x["pipe_id"] == pipe)
    nu = next(x for x in evidence["pipe_materials"] if x["pipe_id"] == pipe)["nu"]
    factor = u4b.shear_factor(nu, section["ro_m"], section["nominal_section"]["ri_m"], section["nominal_section"]["I_m4"])
    return u4b.shear_maximum(shear("element_local_shear_force_y"), shear("element_local_shear_force_z"), factor)


def d5_extrema(extrema):
    """Each preview extremum on both coincident bases, nominal then reduced (§2.5)."""
    out = []
    for extremum in extrema:
        for basis in sb.BASES:
            out.append(dict(extremum, section_basis=basis, result_id=_d5_id(extremum["result_id"], basis)))
    return out


def exact_evidence(source, invocation, nu=NU):
    """pressure-2's closed contract_evidence for the synthetic base (D4 (a)): per case, the prepared section (B3D-4), E,
    nu and G_hat, the preview successor's regenerated extrema, complete coverage and the all-zero pressure RHS.
    T4-U6 (D-5): the section on both coincident bases with zero mill tolerance and allowance and Euler-Bernoulli, and
    the extrema and coverage per basis."""
    body = source["retained_precision"]["body"]
    model = invocation["request"]["model"]
    preview = {case["load_case_id"]: case for case in source["contract_evidence"]["preview_cases"]}
    materials = {m["id"]: m for m in model["materials"]}
    nodes = [n["id"] for n in model["nodes"]]
    entries = []
    for case in body["cases"]:
        cid = case["basis_ref"]["ref_id"]
        src = body["sources"][int(case["source_ref"])]
        sections, published = [], []
        for term, pipe in zip(src["section_terms"], model["pipe_segments"]):
            g = term["geometry"]
            od, wall = rp.from_bits(g["normalized_od"]), rp.from_bits(g["effective_wall"])
            ri = od * 0.5 - wall
            annulus = {"wall_m": wall, "ri_m": ri, "As_m2": rp.from_bits(term["area"]), "I_m4": rp.from_bits(g["actual_second_moment"]),
                       "J_m4": rp.from_bits(g["actual_polar_moment"]), "Z_m3": rp.from_bits(term["section_modulus"])}
            sections.append({"pipe_id": pipe["id"], "geometry_basis": sb.GEOMETRY_BASIS, "outside_diameter_m": od,
                             "ro_m": rp.from_bits(g["actual_radius"]), "wall_thickness_m": wall, "mill_tolerance_m": 0.0,
                             "corrosion_allowance_m": 0.0, "nominal_section": dict(annulus), "reduced_section": dict(annulus),
                             "pressure_bore": {"ri_m": ri, "Ai_m2": 3.141592653589793 * ri * ri}, "beam_theory": {"model": sb.EULER_BERNOULLI}})
            material = materials[pipe["material"]]
            e = material["elastic_modulus"]["value"]
            published.append({"pipe_id": pipe["id"], "material_id": material["id"], "E_pa": e, "nu": nu, "G_pa": e / (2.0 * (1.0 + nu)),
                              "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "thermal_consumed": False, "alpha_per_kelvin": None,
                              "provenance": material["provenance"]})
        zero = [0.0] * (6 * len(nodes))
        entries.append({"load_case_id": cid, "profile_mode": "exact_pressure_v3", "material_basis": "base_material_common_E_nu",
                        "pipe_materials": published, "pipe_sections": sections, "pipe_stress_extrema": d5_extrema(deepcopy(preview[cid]["pipe_stress_extrema"])),
                        "stress_maximum_coverage": [{"section_basis": basis, "complete": True, "unavailable_pipe_ids": []} for basis in sb.BASES],
                        "pressure_rhs_assembly": {"method": "source_factor_grouped_pressure_rhs_v1", "load_case_id": cid, "node_order": nodes,
                                                  "dof_order": ["Fx", "Fy", "Fz", "Mx", "My", "Mz"], "dof_units": ["N", "N", "N", "N*m", "N*m", "N*m"],
                                                  "assembled_pressure_rhs_global": list(zero), "groups": [], "rounded_cap_rhs_global": list(zero),
                                                  "rounded_poisson_rhs_global": list(zero), "rounded_cap_and_eigen_ledgers_are_observational": True,
                                                  "cancellation_screen": 0.0, "screen_limit": 1e-9, "screen_roundoff_multiplier": 32,
                                                  "screen_is_not_numerical_qualification": True}})
    return {"pressure": [], "connector": [], "exact_cases": entries}


def m3x(mode):
    return exact_successor(*milestone(mode))


def exact_successor(source, invocation, nu=NU):
    """A preview base (every case selected) re-stated on the exact route with `nu`, as `m3x` re-states the milestone."""
    invocation = m3x_invocation(invocation, nu)
    source = deepcopy(source)
    source["producer"]["semantic_contract_id"] = rp.EXACT_CONTRACT_ID
    source["formulation_basis"] = {"limitations": list(PRESSURE_1_LIMITATIONS), "profile_id": rp.EXACT_PROFILE}
    source["contract_evidence"] = exact_evidence(source, invocation, nu)
    d5_rows(source)
    body = source["retained_precision"]["body"]
    for basis in body["material_bases"]:
        for material in basis["materials"]:
            material["shear_origin"] = {"kind": "derived_e_nu", "poisson_ratio": rp.bits(nu), "constitutive_basis": "homogeneous_isotropic_E_nu_v1"}
    for src in body["sources"]:
        for term in src["section_terms"]:
            term["geometry"]["route"] = "exact"
    for attempt in body["product_attempts"]:
        attempt["definition_id"] = rp.EXACT_DEFINITION_ID
    return reseal(source, invocation, rp.EXACT_DEFINITION_HASH), invocation


def ulp(value, steps=1):
    import struct
    return struct.unpack(">d", (int(rp.bits(value), 16) + steps).to_bytes(8, "big"))[0]


def _evidence(path):
    """An edit of the owner (first) exact_cases entry, or of contract_evidence itself."""
    def apply(source, invocation):
        target = source["contract_evidence"]
        for key in path[:-1]:
            target = target[key]
        path[-1](target)
        return source, invocation
    return apply


def _receipt(change):
    def apply(source, invocation):
        change(source["retained_precision"]["body"])
        return source, invocation
    return apply


def _envelope(change):
    def apply(source, invocation):
        change(source)
        return source, invocation
    return apply


def _model(change):
    def apply(source, invocation):
        return source, edited_invocation(invocation, change)
    return apply


def _set_body(key, value):
    return _receipt(lambda b: b.update({key: value}))


# Corrosion-allowance edits of one pipe section (b7-plan "Exact-contract fields"): refused on an exact document, and
# zero in any length unit admitted. O4 (W1-E, o4-source-extension §4.6): a finite c_a >= 0 with c_m + c_a < t is inside
# DEF-E's namespace, so a nonzero allowance is no longer refused there; negative and wall-consuming ones are.
CA_REFUSED = {
    "negative -0.001 m": lambda s: s.update(corrosion_allowance={"value": -0.001, "unit": "m"}),
    "the whole wall (1 m)": lambda s: s.update(corrosion_allowance={"value": 1.0, "unit": "m"}),
    "absent": lambda s: s.pop("corrosion_allowance"),
    "null": lambda s: s.update(corrosion_allowance=None),
    "wrong dimension (0 Pa)": lambda s: s.update(corrosion_allowance={"value": 0.0, "unit": "Pa"}),
    "malformed value (string)": lambda s: s.update(corrosion_allowance={"value": "0", "unit": "m"}),
}
# O4: a nonzero allowance passes the namespace; the receipt prepared the member with none, so G8 refuses the
# successor at its preparation step (PREPARATION_MISMATCH), not at the namespace.
CA_NONZERO = {
    "nonzero 0.001 m": lambda s: s.update(corrosion_allowance={"value": 0.001, "unit": "m"}),
}
CA_ADMITTED = {
    "zero in mm": lambda s: s.update(corrosion_allowance={"value": 0.0, "unit": "mm"}),
    "negative zero in m": lambda s: s.update(corrosion_allowance={"value": -0.0, "unit": "m"}),
}


def _section(key, steps=1):
    """One ulp on the owner entry's first pipe section; a dotted key names a nested D-5 leaf (T4-U6)."""
    *nested, leaf = key.split(".")
    return _evidence(["exact_cases", 0, "pipe_sections", 0, *nested, lambda p: p.update({leaf: ulp(p[leaf], steps)})])


def _material(key, value):
    return _evidence(["exact_cases", 0, "pipe_materials", 0, lambda m: m.update({key: value(m[key])})])


def _receipt_material(change):
    return _receipt(lambda b: change(b["material_bases"][0]["materials"][0]))


# Each entry: the edit on the synthetic exact base (sealed again with DEF-E's H unless stated), then PY's first failure.
# Numbers 1 to 32 are B3-D REVISION_01 §4.3's list; x-entries are added shapes for the three-reader comparison.
B3B_REFUSALS = {
    "01 identity relabelled preview (profile kept)": (_envelope(lambda s: s["producer"].update(semantic_contract_id=rp.CONTRACT_ID)), UNSUPPORTED),
    "02 profile relabelled preview": (_envelope(lambda s: s["formulation_basis"].update(profile_id=rp.PROFILE)), UNSUPPORTED),
    "03 an attempt's definition_id ordinary": (_receipt(lambda b: b["product_attempts"][0].update(definition_id=rp.DEFINITION_ID)), UNSUPPORTED),
    "04 projection_policy changed": (_set_body("projection_policy", "RP-LOGICAL-ATTEMPTS-v2"), UNSUPPORTED),
    "05 work_policy changed": (_set_body("work_policy", "W1-LME-20B-60B-v2"), UNSUPPORTED),
    "06 canonicalization changed": (_set_body("canonicalization", "openpipestress_jcs_ijson_v2"), UNSUPPORTED),
    "07 work.case_limit changed": (_receipt(lambda b: b["work"].update(case_limit=20_000_000_001)), UNSUPPORTED),
    "08 work.invocation_limit changed": (_receipt(lambda b: b["work"].update(invocation_limit=60_000_000_001)), UNSUPPORTED),
    # T4-U6 (D-5): the leaves sit in the section bases; the shapes keep their names across the readers.
    "12 owner pipe_sections As_m2 one ulp": (_section("nominal_section.As_m2"), SECTION),
    "13 owner pipe_sections Z_m3 one ulp": (_section("nominal_section.Z_m3"), SECTION),
    "14 owner pipe_sections I_m4 one ulp": (_section("nominal_section.I_m4"), SECTION),
    "15 owner pipe_sections ro_m one ulp": (_section("ro_m"), SECTION),
    "16 connector non-empty": (_evidence([lambda e: e.update(connector=[{"id": "connector:x"}])]), BASE_G7),
    "17 pipe_materials G_pa three ulps": (_material("G_pa", lambda v: ulp(v, 3)), BASE_G7),
    "18 recovery_method added to an exact_cases entry": (_evidence(["exact_cases", 0, lambda e: e.update(recovery_method="retained_source_blocks_exact_v1")]), BASE_G7),
    "19 invocation contract legacy": (_model(lambda m: m.update(pressure_contract=dict(LEGACY))), INVOCATION),
    "20 invocation schema 0.4.0": (_model(lambda m: m.update(schema_version="0.4.0")), INVOCATION),
    "21 invocation pressure_contract false": (_model(lambda m: m.update(pressure_contract=False)), INVOCATION),
    # Entry 22, settled (B2 readers): the exact route admits no combination (B3b, z = 0), and B2-C's G8 equality of the
    # entries with the invocation's combinations would refuse it too, so the expectation stays G8 INVOCATION_MISMATCH.
    "22 a combination added to the invocation": (_model(lambda m: m.update(combinations=[{"id": "combination:x", "label": "x", "terms": [{"load_case": "case", "factor": 1.0}]}])), INVOCATION),
    "23 a case naming modulus_basis_ref": (_model(lambda m: m["load_cases"][0].update(modulus_basis_ref="point:x")), PREPARATION),
    "24 shear_origin explicit_g": (_receipt_material(lambda m: m.update(shear_origin={"kind": "explicit_g"})), PREPARATION),
    "25 receipt shear_modulus one ulp": (_receipt_material(lambda m: m.update(shear_modulus=rp.bits(ulp(rp.from_bits(m["shear_modulus"]))))), PREPARATION),
    "26 shear_origin.poisson_ratio bits changed": (_receipt_material(lambda m: m["shear_origin"].update(poisson_ratio=rp.bits(ulp(NU)))), PREPARATION),
    "27 authored nu changed in the invocation": (_model(lambda m: m["materials"][0]["poisson_ratio"].update(value=ulp(NU))), PREPARATION),
    "28 S-C only: an entry's pipe_materials nu one ulp": (_material("nu", ulp), PREPARATION),
    "29 N-6: an entry's pipe_materials G_pa one ulp": (_material("G_pa", ulp), PREPARATION),
    "30 a case's pressure_regions null": (_model(lambda m: m["load_cases"][0].update(pressure_regions=None)), PREPARATION),
    "31 a case's pressure_regions with one region": (_model(lambda m: m["load_cases"][0].update(pressure_regions=[{"id": "region:x", "members": ["M1"], "pressure": {"value": 0.0, "unit": "Pa"}}])), PREPARATION),
    "32 a member's geometry.route preview": (_receipt(lambda b: b["sources"][0]["section_terms"][0]["geometry"].update(route="preview")), PREPARATION),
    "x01 owner pipe_sections J_m4 one ulp": (_section("nominal_section.J_m4"), SECTION),
    "x02 owner pipe_sections outside_diameter_m one ulp": (_section("outside_diameter_m"), SECTION),
    # The effective wall (t - c_m, with zero allowance) is the reduced section's wall on this route.
    "x03 owner pipe_sections effective_wall_thickness_m one ulp": (_section("reduced_section.wall_m"), SECTION),
    "x04 the owner entry's load_case_id renamed": (_evidence(["exact_cases", 0, lambda e: e.update(load_case_id="case:other")]), SECTION),
    "x05 exact_cases empty": (_evidence([lambda e: e.update(exact_cases=[])]), SECTION),
    "x06 contract_evidence removed": (_envelope(lambda s: s.pop("contract_evidence")), SECTION),
    "x07 a case's pressure_regions removed": (_model(lambda m: m["load_cases"][0].pop("pressure_regions")), PREPARATION),
    "x08 a case with analysis_state": (_model(lambda m: m["load_cases"][0].update(analysis_state={"kind": "load_reference"})), PREPARATION),
    "x09 authored E one ulp": (_model(lambda m: m["materials"][0]["elastic_modulus"].update(value=ulp(m["materials"][0]["elastic_modulus"]["value"]))), PREPARATION),
    "x10 authored nu unit not 1": (_model(lambda m: m["materials"][0]["poisson_ratio"].update(unit="percent")), PREPARATION),
    "x11 authored nu 0.5": (_model(lambda m: m["materials"][0]["poisson_ratio"].update(value=0.5)), PREPARATION),
    "x12 authored constitutive_basis changed": (_model(lambda m: m["materials"][0].update(constitutive_basis="orthotropic_v1")), PREPARATION),
    "x13 invocation contract mode only": (_model(lambda m: m["pressure_contract"].update(mode="legacy_pressure_v1")), INVOCATION),
    "x14 invocation contract extra key": (_model(lambda m: m["pressure_contract"].update(extra="x")), INVOCATION),
    "x15 invocation contract removed": (_model(lambda m: m.pop("pressure_contract")), INVOCATION),
    "x16 invocation schema 0.2.0": (_model(lambda m: m.update(schema_version="0.2.0")), INVOCATION),
    "x18 a named point basis equal to the base, named in the receipt": (lambda s, i: _named_point(s, i), PREPARATION),
    "x19 the owner entry's pipe section listed twice": (_evidence(["exact_cases", 0, lambda e: e.update(pipe_sections=[e["pipe_sections"][0], deepcopy(e["pipe_sections"][0])])]), SECTION),
    "x20 the owner entry's As_m2 a string": (_evidence(["exact_cases", 0, "pipe_sections", 0, "nominal_section", lambda p: p.update(As_m2=repr(p["As_m2"]))]), SECTION),
    "x21 invocation contract version 3.0.1": (_model(lambda m: m["pressure_contract"].update(version="3.0.1")), INVOCATION),
    # B7b: the retired v2 contract and a current contract without its shear model are outside the exact namespace.
    "x33 invocation contract the retired v2": (_model(lambda m: m.update(pressure_contract={"version": "2.0.0", "mode": "exact_straight_pressure_v2"})), INVOCATION),
    "x34 invocation contract without shear_model": (_model(lambda m: m["pressure_contract"].pop("shear_model")), INVOCATION),
    "x35 invocation schema 0.3.0": (_model(lambda m: m.update(schema_version="0.3.0")), INVOCATION),
    # B7b review finding (DEF-E G8; b7-plan "Exact-contract fields"): every pipe section of the exact document declares an
    # explicit length-valued zero corrosion allowance; nonzero, absent, null, non-length or malformed is outside the
    # exact namespace (Rust `exact_namespace` with `section_allowances(model, true)`). The last section is edited.
    # O4 (W1-E): a nonzero allowance short of the wall is inside the namespace; x50 reads it at G8's preparation step.
    **{f"x{36 + i} invocation corrosion_allowance {name}": (_model(lambda m, change=change: change(m["pipe_segments"][-1]["section"])), INVOCATION)
       for i, (name, change) in enumerate(CA_REFUSED.items())},
    **{f"x{50 + i} invocation corrosion_allowance {name}": (_model(lambda m, change=change: change(m["pipe_segments"][-1]["section"])), PREPARATION)
       for i, (name, change) in enumerate(CA_NONZERO.items())},
    "x22 invocation contract with an extra null key": (_model(lambda m: m["pressure_contract"].update(extra=None)), INVOCATION),
    "x23 a component added to the invocation": (_model(lambda m: m.update(components=[{"id": "component:x"}])), INVOCATION),
    "x24 authored nu unit empty": (_model(lambda m: m["materials"][0]["poisson_ratio"].update(unit="")), PREPARATION),
    "x25 authored constitutive_basis removed": (_model(lambda m: m["materials"][0].pop("constitutive_basis")), PREPARATION),
    "x27 policy changed": (_set_body("policy", "M03-INTEGRITY-MP-v3"), UNSUPPORTED),
    "x28 facade_policy changed": (_set_body("facade_policy", "RP-FACADE-SI-v3"), UNSUPPORTED),
    "x29 receipt_version 2": (_set_body("receipt_version", 2), UNSUPPORTED),
    "x30 the owner's exact_cases entry listed twice": (_evidence([lambda e: e.update(exact_cases=[e["exact_cases"][0], deepcopy(e["exact_cases"][0])])]), SECTION),
    "x31 producer component_version 0.2.1": (_envelope(lambda s: s["producer"].update(component_version="0.2.1")), UNSUPPORTED),
    "x32 a material's selection named_point": (_receipt_material(lambda m: m.update(selection={"kind": "named_point", "point_id": "point:x"})), PREPARATION),
    "x26 G_hat one ulp high, every copy consistent": (lambda s, i: _g_hat_forged(s, i), PREPARATION),
    "x17 pressure-2 headline altered": (_envelope(lambda s: s["summary"]["max_open_formula_stress_nominal_section"].update(value=ulp(s["summary"]["max_open_formula_stress_nominal_section"]["value"]))), BASE_G7),
}
def _named_point(source, invocation):
    """Kills the base-only selection guards: a case naming a temperature point whose E and nu equal the base's, with the
    receipt's basis selector named accordingly; S-C's own selection then agrees, so only D1.5-exact refuses it."""
    def change(model):
        material = model["materials"][0]
        material["temperature_points"] = [{"id": "point:x", "temperature": {"value": 20.0, "unit": "degC"},
                                           "elastic_modulus": deepcopy(material["elastic_modulus"]), "poisson_ratio": deepcopy(material["poisson_ratio"])}]
        model["load_cases"][0]["modulus_basis_ref"] = "point:x"
    source["retained_precision"]["body"]["material_bases"][0]["selector"] = {"kind": "named", "id": "point:x"}
    return source, edited_invocation(invocation, change)


def _g_hat_forged(source, invocation):
    """Kills a G_hat binding that only compares E: the receipt's shear modulus one ulp above RN64(E/(2*RN64(1+nu))),
    with every copy made consistent (member G and the native source hashes, old_source, the operational inputs and
    torsional stiffnesses, both section echoes, and the published G_pa, within the base reader's 2-ulp tolerance)."""
    body = source["retained_precision"]["body"]
    g = rp.bits(ulp(rp.from_bits(body["material_bases"][0]["materials"][0]["shear_modulus"])))
    body["material_bases"][0]["materials"][0]["shear_modulus"] = g
    src, attempt = body["sources"][0], body["product_attempts"][0]
    src["id_maps"]["members"][0]["G"] = g
    attempt["preparation"]["members"][0]["old_source"][1] = g
    for side in ("old", "new"):
        record = attempt["operational"][side][0]
        record["inputs"][7] = g
        x = [rp.from_bits(v) for v in record["inputs"]]
        length = rp.from_bits(record["result"]["length"])
        record["result"]["torsional_stiffness"] = rp.bits((x[7] * x[9]) / length)
    torsion = attempt["operational"]["new"][0]["result"]["torsional_stiffness"]
    src["section_terms"][0]["torsional_stiffness"] = torsion
    body["cases"][0]["selection"]["section_terms"][0]["torsional_stiffness"] = torsion
    renamed = {}
    for include_loads, key in ((True, "kernel_source_sha256"), (False, "stiffness_sha256")):
        new = hashlib.sha256(rp._native_source_encoding(src, include_loads)).hexdigest()
        renamed[src[key]] = new
        src[key] = new

    def rename(value):  # the native groups and calls name the source by these hashes
        items = value.items() if isinstance(value, dict) else enumerate(value) if isinstance(value, list) else ()
        for k, v in list(items):
            if isinstance(v, str):
                value[k] = renamed.get(v, v)
            else:
                rename(v)
    rename(body)
    source["contract_evidence"]["exact_cases"][0]["pipe_materials"][0]["G_pa"] = rp.from_bits(g)
    return source, invocation


B3B_PASSES = {
    "p01 an authored redundant G in the invocation (ignored by the producer)": _model(lambda m: m["materials"][0].update(shear_modulus={"unit": "Pa", "value": 8.0e10})),
    "p02 a case-level pressure key (PP's typed case has none; addendum 01)": _model(lambda m: m["load_cases"][0].update(pressure={"value": 1000.0, "unit": "Pa"})),
    **{f"p{3 + i} invocation corrosion_allowance {name}": _model(lambda m, change=change: change(m["pipe_segments"][-1]["section"]))
       for i, (name, change) in enumerate(CA_ADMITTED.items())},
}


# Lane P's producer-solved m3x successors (B3b-P, b2 8d3419b542; PP retained_facade_tests.rs `EXACT_PINNED`):
# document and receipt sha256, as RS pins them. Every B3b shape is also read on each. B7b's W1 regeneration (the
# current 0.7.0 exact document; PP retained_facade_tests.rs and RS retained_precision_contract.rs `M3X` pin the same
# bytes) replaced 5ef83dd9.../57114af4... and ee20a337.../3b92b6ce.... T4-U6's regeneration on the D-5 producer
# (reduced stress twins, per-basis headlines and evidence) replaced 45e14e68.../f6bb7e7f... and 0f490c22.../0317aed6....
# O4 (W1-E): DEF-E's revision moved only the DEF-E-bound hashes; it replaced 3cfcde8d.../a5ac8236... and 2e69b43c.../0a357a3a....
EXACT_PINNED = {"sparse_interactive": ("08f9c9f654b4f18871cc9b08a23655fb8517c91fbfc5d9a3f24932f8694ace82", "a6f0815b30fa042a51552fa8d1881fe1af39479fffbc0382a866248e2276f531"),
                "dense_scrutiny": ("04ee8f329c19d19278b01be56365c3b76ee5fc8469bf8b9d488e4bd54b4aa535", "6aea076aaf67042cbd61bab4697a9da2d6fb873f331510643d0764a00cd6dafc")}
KINDS = ("synthetic", "producer")


def m3x_producer(mode):
    raw = (ROOT / f"fixtures/results/retained_precision_exact_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == EXACT_PINNED[mode][0]
    doc = json.loads(raw)
    assert doc["source"]["retained_precision"]["receipt_sha256"] == EXACT_PINNED[mode][1] and doc["invocation"]["solver_mode"] == mode
    return doc["source"], doc["invocation"]


def exact_base(mode, kind="synthetic"):
    return m3x(mode) if kind == "synthetic" else m3x_producer(mode)


def exact_shape(mode, label, kind="synthetic"):
    """(source, invocation) for one B3b shape on the synthetic or the producer's m3x base, sealed by the format rule
    with the route's H (DEF-O's for entry 11)."""
    source, invocation = exact_base(mode, kind)
    if label.startswith("09"):
        base, base_invocation = milestone(mode)
        base = deepcopy(base)
        base["producer"]["semantic_contract_id"] = rp.EXACT_CONTRACT_ID
        base["formulation_basis"]["profile_id"] = rp.EXACT_PROFILE
        return reseal(base, base_invocation), base_invocation
    if label.startswith("10"):
        source["producer"]["semantic_contract_id"] = rp.CONTRACT_ID
        source["formulation_basis"]["profile_id"] = rp.PROFILE
        return reseal(source, invocation, rp.EXACT_DEFINITION_HASH), invocation
    if label.startswith("11"):
        return reseal(source, invocation, rp.DEFINITION_HASH), invocation
    change = B3B_REFUSALS[label][0] if label in B3B_REFUSALS else B3B_PASSES[label]
    source, invocation = change(deepcopy(source), deepcopy(invocation))
    return reseal(source, invocation, rp.EXACT_DEFINITION_HASH), invocation


B3B_SPECIAL = {"09 the preview milestone successor relabelled physics-retained-1": UNSUPPORTED,
               "10 the exact successor relabelled preview-physics-retained-1": UNSUPPORTED,
               "11 S-1: the preparation hash over a payload with DEF-O's H": RECEIPT}


@pytest.mark.parametrize("mode", MODES)
def test_b3b_m3x_synthetic_base_is_eligible_and_classified_as_the_milestone(mode):
    source, invocation = m3x(mode)
    assert outcome(source, invocation) == ("pass", True, "eligible")
    assert outcome(source, None) == ("pass", False, "needs_recompute")
    assert outcome(source, None, transport=True) == ("pass", False, "needs_recompute")
    got = rp.validate_retained_precision(source, invocation)
    base_source, base_invocation = milestone(mode)
    want = rp.validate_retained_precision(base_source, base_invocation)
    assert got["numerical_eligible"] is True and got["standing"] == "eligible" and got["invocation_bound"] is True
    # T4-U6 (D-5): a stress row is classified on its nominal id, and its reduced twin takes the same class.
    stress = {row["id"] for row in base_source["results"] if row["kind"] in sb.STRESS_KINDS}
    derived = {row["id"] for row in base_source["results"] if row["kind"] in DERIVED_KINDS}
    published = {row["id"]: row for row in source["results"]}
    expected = []
    for cls in want["classifications"]:
        if cls["result_id"] in derived:
            # T4-U4b: each derived row's twins are not covered, at their own published values (τ on the exact basis).
            expected += [dict(cls, result_id=_d5_id(cls["result_id"], basis),
                              normalized_bits=rp.bits(rp._normalized(published[_d5_id(cls["result_id"], basis)])))
                         for basis in sb.BASES]
            continue
        if cls["result_id"] not in stress:
            expected.append(cls)
            continue
        expected += [dict(cls, result_id=_d5_id(cls["result_id"], basis)) for basis in sb.BASES]
    assert got["classifications"] == expected
    unbound = rp.validate_retained_precision(source)
    assert (unbound["invocation_bound"], unbound["numerical_eligible"]) == (False, False)
    transport = rp.validate_retained_precision_transport(source)
    assert (transport["numerical_eligible"], transport["standing"], transport["classifications"]) == (False, "needs_recompute", [])
    # D4 (a): the base's own pressure-2 reader accepts the projection (G7).
    assert dispatch(rp._project(source, rp.EXACT_ROUTE))[0] == "openpipestress.result_semantics/0.3.0/pressure-2"
    # The preview reader's path is not taken: G0 dispatches on the identity.
    assert rp.ROUTES[source["producer"]["semantic_contract_id"]] is rp.EXACT_ROUTE


@pytest.mark.parametrize("label", sorted(B3B_REFUSALS) + sorted(B3B_SPECIAL))
@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("kind", KINDS)
def test_b3b_m3x_mutations(kind, mode, label):
    expected = B3B_REFUSALS[label][1] if label in B3B_REFUSALS else B3B_SPECIAL[label]
    source, invocation = exact_shape(mode, label, kind)
    assert outcome(source, invocation) == expected


@pytest.mark.parametrize("label", sorted(B3B_PASSES))
@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("kind", KINDS)
def test_b3b_m3x_must_pass(kind, mode, label):
    source, invocation = exact_shape(mode, label, kind)
    assert outcome(source, invocation) == ("pass", True, "eligible")


CA_LABELS = sorted(label for label in [*B3B_REFUSALS, *B3B_PASSES] if "corrosion_allowance" in label)


def _rust_outcome(source, invocation):
    from core.analysis_runs import rust_reader
    try:
        result = rust_reader.retained_precision(source, invocation)
    except rust_reader.ReaderRefusal as error:
        return (error.gate, error.code)
    # Rust's outcome carries no standing token; a bound eligible read is "eligible", any other admitted read
    # "needs_recompute", as the verifier's `standing`.
    return ("pass", result["numerical_eligible"], "eligible" if result["invocation_bound"] and result["numerical_eligible"] else "needs_recompute")


@pytest.mark.parametrize("label", CA_LABELS)
@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("kind", KINDS)
def test_b3b_corrosion_allowance_is_bound_by_both_readers(kind, mode, label):
    """B7b review finding (DEF-E G8; b7-plan "Exact-contract fields"): a resealed exact successor whose invocation's
    allowance is nonzero, absent, null, non-length or malformed was read "pass, eligible" by both readers. Both now
    refuse it at G8 INVOCATION_MISMATCH; zero in any length unit stays eligible. O4 (W1-E): a nonzero allowance short
    of the wall passes the namespace and is refused at G8 PREPARATION_MISMATCH (the receipt prepared none)."""
    expected = B3B_REFUSALS[label][1] if label in B3B_REFUSALS else ("pass", True, "eligible")
    source, invocation = exact_shape(mode, label, kind)
    assert outcome(source, invocation) == expected
    assert _rust_outcome(source, invocation) == expected


@pytest.mark.parametrize("value", [{"value": 0.0, "unit": "m"}, {"value": 0.001, "unit": "m"}, None])
@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("name", ["milestone", "l0"])
def test_preview_successor_invocation_carrying_an_allowance_is_refused(name, mode, value):
    """The pressure-free half (b7-plan "Exact-contract fields": CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT): a
    preview successor whose invocation adds the key on one pipe section, zero and null included, is outside the
    preview namespace in both readers (Rust `preview_namespace` with `section_allowances(model, false)`)."""
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_{name}_successor_{mode}.json").read_text())
    source, invocation = doc["source"], doc["invocation"]
    control = reseal(source, invocation)
    assert outcome(control, invocation) == _rust_outcome(control, invocation) == ("pass", True, "eligible")
    changed = edited_invocation(invocation, lambda m: m["pipe_segments"][0]["section"].update(corrosion_allowance=deepcopy(value)))
    forged = reseal(source, changed)
    assert outcome(forged, changed) == _rust_outcome(forged, changed) == INVOCATION
    assert outcome(forged, None) == ("pass", False, "needs_recompute")


# B7b review N-1: every exact base above has one pipe and every preview edit above is on pipe 0, so none tells an
# every-section allowance check from a first-section one. These read a pipe other than index 0.
CA_SECOND_ZERO = "zero in m"


def _second_pipe(change):
    """Appends a second pipe segment to the invocation's model: a copy of the first under a new id, its section changed
    by `change`; the first keeps its explicit 0 m."""
    def apply(model):
        assert len(model["pipe_segments"]) == 1
        second = deepcopy(model["pipe_segments"][0])
        second["id"] = f"{second['id']}-second"
        change(second["section"])
        model["pipe_segments"].append(second)
    return apply


@pytest.mark.parametrize("label", sorted(CA_REFUSED) + [CA_SECOND_ZERO] + sorted(CA_NONZERO))
@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("kind", KINDS)
def test_b3b_allowance_on_an_appended_second_pipe_is_bound_by_both_readers(kind, mode, label):
    """On the synthetic and the producer's m3x bases (one pipe), a second pipe appended to the invocation with a nonzero,
    absent, null, non-length or malformed allowance is outside D1.3's exact namespace in both readers (G8
    INVOCATION_MISMATCH; the namespace runs before any G8 step that reads a pipe). With 0 m on the second pipe the
    namespace admits it and both refuse it later, G8 PREPARATION_MISMATCH (the receipt prepared one member), not at the
    namespace; a first-section-only check would give that code throughout. O4: a nonzero allowance is inside the
    namespace too and reads PREPARATION_MISMATCH like zero."""
    source, invocation = exact_base(mode, kind)
    change = {**CA_REFUSED, **CA_NONZERO}.get(label, lambda s: s.update(corrosion_allowance=dict(ZERO_ALLOWANCE)))
    invocation = edited_invocation(invocation, _second_pipe(change))
    assert invocation["request"]["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] == ZERO_ALLOWANCE
    source = reseal(source, invocation, rp.EXACT_DEFINITION_HASH)
    expected = INVOCATION if label in CA_REFUSED else PREPARATION
    assert outcome(source, invocation) == _rust_outcome(source, invocation) == expected
    assert outcome(source, None) == ("pass", False, "needs_recompute")


# Rust `retained_precision_contract.rs` EXACT_NU: the shared corpus's two-pipe synthetic bases have G = E / (2 (1 + nu)).
EXACT_NU = 0.2987012987012987


@pytest.mark.parametrize("label", sorted(CA_REFUSED) + sorted(CA_ADMITTED) + sorted(CA_NONZERO))
@pytest.mark.parametrize("base", ["two_body_synthetic", "p512_ladder_synthetic"])
def test_b3b_allowance_on_pipe_1_of_a_two_pipe_exact_successor(base, label):
    """Reader-local two-pipe exact successors (Rust `exact_successor` on the same shared bases): pipe 1's allowance
    nonzero, absent, null, non-length or malformed is refused at G8 INVOCATION_MISMATCH by both readers, and zero in any
    length unit on pipe 1 keeps the successor eligible. O4: a nonzero allowance on pipe 1 passes the namespace and is
    refused at G8 PREPARATION_MISMATCH (pipe 1 was prepared with none)."""
    source, invocation = exact_successor(SHARED[base]["source"], SHARED[base]["invocation"], EXACT_NU)
    assert len(invocation["request"]["model"]["pipe_segments"]) == 2
    assert outcome(source, invocation) == _rust_outcome(source, invocation) == ("pass", True, "eligible")
    change = {**CA_REFUSED, **CA_ADMITTED, **CA_NONZERO}[label]
    invocation = edited_invocation(invocation, lambda m: change(m["pipe_segments"][1]["section"]))
    source = reseal(source, invocation, rp.EXACT_DEFINITION_HASH)
    expected = INVOCATION if label in CA_REFUSED else PREPARATION if label in CA_NONZERO else ("pass", True, "eligible")
    assert outcome(source, invocation) == _rust_outcome(source, invocation) == expected


def _preview_base(name):
    if name in SHARED:
        return SHARED[name]["source"], SHARED[name]["invocation"]
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_{name}.json").read_text())
    return doc["source"], doc["invocation"]


def _rebind(source, invocation):
    """Only the invocation changes, so only its digest and the receipt hash over the body follow it: no preparation
    hash, source identity or publication hash reads the invocation. (`reseal` covers no CombinationSource.)"""
    source = deepcopy(source)
    body = source["retained_precision"]["body"]
    body["invocation"]["value"] = rp._hash("source_blocks_invocation_v1", invocation)
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return source


PREVIEW_MULTI_PIPE = [(base, pipe) for base in ("w_cb1_sparse_interactive", "w_cb1_dense_scrutiny") for pipe in (1, 31)] + [
    (base, 1) for base in ("w_c2_successor_sparse_interactive", "w_c2_successor_dense_scrutiny")]


@pytest.mark.parametrize("value", [{"value": 0.0, "unit": "m"}, {"value": 0.001, "unit": "m"}, None])
@pytest.mark.parametrize("name,pipe", PREVIEW_MULTI_PIPE)
def test_preview_allowance_on_a_pipe_other_than_the_first_is_refused(name, pipe, value):
    """The pressure-free half on multi-pipe bases (the shared corpus's 32-pipe W-CB1 bases and the committed two-pipe
    W-C2 successors, both modes): an allowance key on pipe 1 or the last pipe, zero and null included, is outside the
    preview namespace in both readers."""
    source, invocation = _preview_base(name)
    assert len(invocation["request"]["model"]["pipe_segments"]) > pipe > 0
    assert _rebind(source, invocation) == source
    assert outcome(source, invocation)[0] == _rust_outcome(source, invocation)[0] == "pass"
    changed = edited_invocation(invocation, lambda m: m["pipe_segments"][pipe]["section"].update(corrosion_allowance=deepcopy(value)))
    forged = _rebind(source, changed)
    assert outcome(forged, changed) == _rust_outcome(forged, changed) == INVOCATION
    assert outcome(forged, None) == ("pass", False, "needs_recompute")


@pytest.mark.parametrize("mode", MODES)
def test_b3b_producer_m3x_successor_pins(mode):
    """Lane P's m3x successors read eligible with their invocation (needs_recompute without it and on transport), in the
    milestone's classes, through the reader's exact route and the carriers' dispatch."""
    source, invocation = m3x_producer(mode)
    assert outcome(source, invocation) == ("pass", True, "eligible")
    assert outcome(source, None) == ("pass", False, "needs_recompute")
    assert outcome(source, None, transport=True) == ("pass", False, "needs_recompute")
    got = rp.validate_retained_precision(source, invocation)["classifications"]
    counts = [sum(1 for c in got if c["class"] == k) for k in ("relative_verified", "absolute_verified", "input_derived", "non_quantity")]
    # T4-U6 (D-5 §2.5): each of the member's 21 reduced stress twins takes its nominal row's class (5 relative,
    # 16 absolute), beside the milestone's 25, 69, 3 and 1.
    assert counts == [25 + 5, 69 + 16, 3, 1]
    # Every quantity row as the synthetic base classes it, bit for bit. E14 removed
    # the separate dense parity observation; the current mode header is non_quantity.
    synthetic = rp.validate_retained_precision(*m3x(mode))["classifications"]
    quantity = lambda classes: [(c["class"], c["normalized_bits"], c["scale_bits"], c["bound_bits"]) for c in classes if c["class"] != "non_quantity"]
    assert quantity(got) == quantity(synthetic)
    assert dispatch(source)[0] == c.PHYSICS_RETAINED_CONTRACT_ID
    assert c.numerical_use_standing(source, requested(invocation), invocation) == "numerically_eligible"


@pytest.mark.parametrize("label, s_c", [("x10 authored nu unit not 1", False), ("x24 authored nu unit empty", False), ("x11 authored nu 0.5", False),
                                        ("26 shear_origin.poisson_ratio bits changed", False), ("x26 G_hat one ulp high, every copy consistent", False),
                                        ("28 S-C only: an entry's pipe_materials nu one ulp", True), ("x12 authored constitutive_basis changed", True)])
def test_b3b_g8_material_binding_precedes_s_c(label, s_c):
    """REVISION_01 §4.2's G8 order on the exact branch: step 4 (the receipt's E, G_hat and nu against the authored
    material) before step 5 (S-C), whose own code is the detail of G8's PREPARATION_MISMATCH (B3D-13)."""
    source, invocation = exact_shape("sparse_interactive", label)
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision(source, invocation)
    assert (error.value.gate, error.value.code) == PREPARATION
    assert (error.value.detail or "").startswith("PHYSICS_SOURCE_") is s_c, error.value.detail


def test_b3b_s1_payload_carries_the_route_hash():
    """S-1 at G1 and G8: the exact base's preparation hash is computed over DEF-E's H; the same payload with DEF-O's H
    differs, and is refused at G1 (entry 11)."""
    source, invocation = m3x("sparse_interactive")
    body = source["retained_precision"]["body"]
    attempt = body["product_attempts"][0]
    prep = body["sources"][0]["preparation"]["sha256"]
    assert prep == rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt, rp.EXACT_DEFINITION_HASH))
    assert prep != rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt, rp.DEFINITION_HASH))
    # RV120 N1: the payload takes the route's hash from every caller; there is no DEF-O default to fall back on.
    with pytest.raises(TypeError):
        rp._preparation_payload(attempt)


def test_b3b_g5b_evidence_check_runs_only_on_the_exact_branch():
    """The preview branch has no exact evidence: its G5b is unchanged; the exact branch binds the owner entry."""
    source, invocation = milestone("sparse_interactive")
    assert "exact_cases" not in source["contract_evidence"]
    assert rp.validate_retained_precision(source, invocation)["numerical_eligible"] is True


@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("kind", KINDS)
def test_b3b_transport(kind, mode):
    source, _ = exact_base(mode, kind)
    assert outcome(source, None, transport=True) == ("pass", False, "needs_recompute")
    for label in ("16 connector non-empty", "18 recovery_method added to an exact_cases entry"):
        broken, _ = exact_shape(mode, label, kind)
        assert outcome(broken, None, transport=True) == BASE_G7, label
    for label in ("01 identity relabelled preview (profile kept)", "04 projection_policy changed"):
        broken, _ = exact_shape(mode, label, kind)
        assert outcome(broken, None, transport=True) == UNSUPPORTED, label
    broken, _ = exact_shape(mode, "11 S-1: the preparation hash over a payload with DEF-O's H", kind)
    assert outcome(broken, None, transport=True) == RECEIPT


def _packaged(tmp_path, monkeypatch, edit_table=None, edit_definition=None):
    """A test-only packaged copy of the exact route's three statics (B3-D §2.4; REV §2: a corpus entry cannot change a
    table), with the reader's ROOT pointed at it and its table constant pinned to the copy's bytes."""
    results = tmp_path / "fixtures/results"
    results.mkdir(parents=True)
    # D4 (a): the exact route's inherited table is pressure-2's.
    for name in ("semantic_contract_v0_3_physics_retained_1.json", "retained_precision_prepared_exact_v1.json", "semantic_contract_v0_3_pressure_2.json"):
        (results / name).write_bytes((ROOT / "fixtures/results" / name).read_bytes())
    table = json.loads((results / "semantic_contract_v0_3_physics_retained_1.json").read_text())
    if edit_table is not None:
        edit_table(table)
        raw = (json.dumps(table, indent=2, ensure_ascii=True) + "\n").encode()
        (results / "semantic_contract_v0_3_physics_retained_1.json").write_bytes(raw)
        monkeypatch.setattr(rp, "EXACT_TABLE_HASH", hashlib.sha256(raw).hexdigest())
    if edit_definition is not None:
        definition = json.loads((results / "retained_precision_prepared_exact_v1.json").read_text())
        edit_definition(definition)
        (results / "retained_precision_prepared_exact_v1.json").write_text(json.dumps(definition, sort_keys=True, separators=(",", ":")))
    monkeypatch.setattr(rp, "ROOT", tmp_path)


def _g0(source):
    try:
        rp._g0_exact(source["retained_precision"])
        return "pass"
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code, error.detail)


def test_b3b_g0_reads_the_packaged_table(tmp_path, monkeypatch):
    source, _ = m3x("sparse_interactive")
    _packaged(tmp_path, monkeypatch)
    assert _g0(source) == "pass"


@pytest.mark.parametrize("label, edit", [
    ("receipt_bindings.projection_policy", lambda t: t["receipt_bindings"].update(projection_policy="RP-LOGICAL-ATTEMPTS-v2")),
    ("receipt_bindings.work.case_limit", lambda t: t["receipt_bindings"]["work"].update(case_limit=1)),
    ("receipt_bindings.method", lambda t: t["receipt_bindings"].update(method="other_method")),
    ("receipt_policy", lambda t: t.update(receipt_policy="M03-INTEGRITY-MP-v3")),
    ("accuracy_classification.policy", lambda t: t["accuracy_classification"].update(policy="RP-FACADE-SI-v3")),
])
def test_b3b_g0_table_constant_cross_check(tmp_path, monkeypatch, label, edit):
    """§2.4 step 6 (decision 31, N-12), reader-local: a test-only table whose bound value differs from the reader's
    constant is refused by the cross-check, even with the table's own hash pinned to it."""
    source, _ = m3x("sparse_interactive")
    _packaged(tmp_path, monkeypatch, edit_table=edit)
    assert _g0(source) == ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "table/constant cross-check"), label


def test_b3b_g0_constant_drift_is_refused_by_the_cross_check(monkeypatch):
    source, _ = m3x("sparse_interactive")
    monkeypatch.setattr(rp, "RECEIPT_BINDINGS", dict(rp.RECEIPT_BINDINGS, work_policy="W1-LME-20B-60B-v2"))
    assert _g0(source) == ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "table/constant cross-check")


@pytest.mark.parametrize("label, edit, expected", [
    ("identity", lambda t: t.update(semantic_contract_id="openpipestress.result_semantics/0.3.0/physics-retained-2"), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
    ("profile", lambda t: t.update(formulation_profile_id="exact_straight_retained_w1a_v3"), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
    ("bound definition", lambda t: t["product_formation_definitions"][0].update(sha256="0" * 64), "RETAINED_PRECISION_FORMATION_MISMATCH"),
    ("a second definition", lambda t: t["product_formation_definitions"].append({"id": "RP-PREPARED-ORDINARY-DUAL-v1", "sha256": rp.DEFINITION_HASH}), "RETAINED_PRECISION_FORMATION_MISMATCH"),
    ("inherited hash", lambda t: t.update(inherited_semantic_contract_sha256="0" * 64), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
])
def test_b3b_g0_table_steps_3_to_5(tmp_path, monkeypatch, label, edit, expected):
    source, _ = m3x("sparse_interactive")
    _packaged(tmp_path, monkeypatch, edit_table=edit)
    assert _g0(source)[:2] == ("G0", expected), label


def test_b3b_g0_table_bytes_and_definition_hash(tmp_path, monkeypatch):
    source, _ = m3x("sparse_interactive")
    _packaged(tmp_path, monkeypatch, edit_definition=lambda d: d.update(version=2))
    assert _g0(source)[:2] == ("G0", "RETAINED_PRECISION_FORMATION_MISMATCH")
    monkeypatch.setattr(rp, "ROOT", ROOT)
    monkeypatch.setattr(rp, "EXACT_TABLE_HASH", "0" * 64)
    assert _g0(source)[:2] == UNSUPPORTED


# ---------------------------------------------------------------------------------------------------------------
# The exact successor through the Python carriers (compatibility.py): dispatch, standing, classes, binding refusal
# and the AnalysisRun record, which carries the receipt and, as its pressure-2 base's, no contract_evidence.


def dispatch(source, check_receipt=True):
    try:
        return c._source_contract(source, check_receipt=check_receipt)
    except ValueError as error:
        return str(error)


def built(source):
    """The AnalysisRun record, or the builder's refusal text (the builder validates what it built)."""
    try:
        return c.build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:b3b"}, input_manifest_hash="1" * 64)
    except ValueError as error:
        return str(error)


def requested(invocation):
    return [{"ref_type": "load_case", "ref_id": case["id"]} for case in invocation["request"]["model"]["load_cases"]]


@pytest.mark.parametrize("mode", MODES)
def test_b3b_carriers(mode):
    source, invocation = m3x(mode)
    assert dispatch(source) == (c.PHYSICS_RETAINED_CONTRACT_ID, c.PHYSICS_RETAINED_CONTRACT_SHA256, c._PHYSICS_RETAINED_CONTRACT_PATH)
    assert dispatch(source, check_receipt=False)[0] == c.PHYSICS_RETAINED_CONTRACT_ID
    assert hashlib.sha256(c._PHYSICS_RETAINED_CONTRACT_PATH.read_bytes()).hexdigest() == c.PHYSICS_RETAINED_CONTRACT_SHA256 == rp.EXACT_TABLE_HASH
    assert c.PHYSICS_RETAINED_CONTRACT_ID in c.CURRENT_RECORD_CONTRACT_IDS
    refs = requested(invocation)
    assert c.numerical_use_standing(source, refs) == "needs_recompute"
    assert c.numerical_use_standing(source, refs, invocation) == "numerically_eligible"
    assert c.numerical_use_standing(source, refs, m3l(mode)[1]) == "unsupported"
    classes = rp.validate_retained_precision(source, invocation)["classifications"]
    summary = c.classification_summary(source, invocation, refs)
    assert len(summary) == 1 and summary[0]["relative_verified"] == sum(1 for x in classes if x["class"] == "relative_verified")
    rows = {row["id"]: row for row in source["results"]}
    for item in classes:
        assert c.rule_binding_refusal(source, rows[item["result_id"]]) == compatibility_verifier._class_binding_refusal(item["class"])
    record = built(source)
    assert isinstance(record, dict), record
    run = record["analysis_run"]
    assert run.get("retained_precision") == source["retained_precision"] and "contract_evidence" not in run
    assert run["reproducibility"]["semantic_contract"] == {"id": c.PHYSICS_RETAINED_CONTRACT_ID, "sha256": c.PHYSICS_RETAINED_CONTRACT_SHA256}
    c.validate_analysis_run_v0_3(record, source)
    dropped = deepcopy(record)
    del dropped["analysis_run"]["retained_precision"]
    with pytest.raises(ValueError) as error:
        c.validate_analysis_run_v0_3(dropped, source)
    assert str(error.value) == c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH
    # F-5: pressure-2 carrying the receipt is a downgrade; the Rust reader's G7 text comes back on a refused
    # successor (its own base code, where the verifier reports SOURCE_PHYSICS_EVIDENCE_INVALID).
    downgraded = rp._project(source, rp.EXACT_ROUTE)
    downgraded["retained_precision"] = deepcopy(source["retained_precision"])
    assert dispatch(downgraded) == c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN
    # D4 (a): the projection is read by the pressure-2 reader, which admits objective connector records (T4-U3), so
    # the invented record is refused by its record shape (formerly physics-1's CONNECTOR_UNSUPPORTED).
    broken, _ = exact_shape(mode, "16 connector non-empty")
    assert dispatch(broken) == "SOURCE_PHYSICS_CONNECTOR_RECORD_SHAPE"
    with pytest.raises(rp.RetainedPrecisionError) as verifier:
        rp.validate_retained_precision(broken)
    assert (verifier.value.gate, verifier.value.code) == BASE_G7


@pytest.mark.parametrize("mode", MODES)
def test_b3b_stress_neutral_packager_refuses_the_exact_successor(mode):
    """As the preview successor (T6's refusal stands in the Python packager; lane T's carriers are TS and schemas)."""
    from core.handoff.stress_neutral import package as sn
    from tests.test_stress_neutral_physics_source import arguments
    source, _ = m3x(mode)
    record = built(source)
    assert isinstance(record, dict), record
    try:
        sn.build_stress_neutral_export_package(source_envelope=source, analysis_record=record, **arguments(source, record))
        refused = None
    except ValueError as error:
        refused = str(error)
    assert refused == "SN-SOURCE-METHOD-UNSUPPORTED"


# ---------------------------------------------------------------------------------------------------------------
# Addendum 01: G8's sourced-case check on the preview route, aligned in the three readers (DOMAIN D1.5; C1's G8 row,
# "no 0.4 extension"; B3D-11). Each value is set on the milestone's sourced case and the receipt is resealed, so the
# statement passes G0-G7 and reaches G8 (I100 B3 addendum 01, reachability).

SOURCED_CASE = [
    ("analysis_state", {"kind": "load_reference_state"}, PREPARATION),
    ("analysis_state", None, PREPARATION),
    ("analysis_state", {}, PREPARATION),
    ("pressure", {"value": 1000.0, "unit": "Pa"}, None),
    ("pressure", None, None),
    ("pressure", 0, None),
    ("pressure_regions", "x", PREPARATION),
    ("pressure_regions", {}, PREPARATION),
    ("pressure_regions", {"id": "region:x"}, PREPARATION),
    ("pressure_regions", 0, PREPARATION),
    ("pressure_regions", 1, PREPARATION),
    ("pressure_regions", True, PREPARATION),
    ("pressure_regions", False, PREPARATION),
    ("pressure_regions", "", PREPARATION),
    ("pressure_regions", [], None),
    ("pressure_regions", None, None),
    ("pressure_regions", [{"id": "region:x", "member_pipe_ids": ["M1"]}], PREPARATION),
    ("equivalent_static", {}, PREPARATION),
    ("equivalent_static", False, PREPARATION),
    ("equivalent_static", 0, PREPARATION),
    ("equivalent_static", None, None),
    ("notes", "free text", None),
]


@pytest.mark.parametrize("key, value, expected", SOURCED_CASE, ids=[f"{k}={json.dumps(v)}" for k, v, _ in SOURCED_CASE])
@pytest.mark.parametrize("mode", MODES)
def test_addendum01_sourced_case_on_the_preview_route(mode, key, value, expected):
    """Refused: an analysis_state member (null included); pressure_regions other than absent, null or []; equivalent_static
    other than absent or null. Admitted: a key PP's typed load case does not have, a case-level `pressure` included (serde
    ignores it, and the invocation digest is over the raw request, so a producer-emitted successor can carry it)."""
    source, invocation = milestone(mode)
    invocation = edited_invocation(invocation, lambda m: m["load_cases"][0].update({key: deepcopy(value)}))
    sealed = reseal(source, invocation)
    assert outcome(sealed, invocation) == (expected or ("pass", True, "eligible"))
    # Unbound and on transport G8 does not run: each reads as the base does.
    assert outcome(sealed, None) == ("pass", False, "needs_recompute")
    assert outcome(sealed, None, transport=True) == ("pass", False, "needs_recompute")


# ---------------------------------------------------------------------------------------------------------------
# Repair 01 (RV120, RV-R2): the preserved mutations use current bases and DEF-E.
# input_sha256 pins Python sorted-compact [source, invocation]; the full constructed values also bind to native R1's JCS inputs.
# - F1: on the exact route G5b runs the shared checks over every selected case first, then each case's exact
#   evidence (DESIGN §6.2's G5b row), as RS and TS do. With a fault in case 0's evidence and one in case 1's shared
#   checks, the shared one is read.
# - F2: E or G-hat moved by one ulp in all five receipt copies, with the derived stiffness (and for G-hat the evidence
#   G_pa) moved with it and the native hashes resealed. G8 step 4's bits are what anchor the receipt to the authored
#   material, so each forgery is refused there.

RV120 = json.loads((ROOT / "fixtures/results/retained_precision_rv120_b3_inputs.json").read_text())
_RV120_CORPUS = {}


def _rv120_base(shape):
    kind, key = shape["base"]
    if kind == "corpus":
        if not _RV120_CORPUS:
            cases = load_corpus()["cases"]
            _RV120_CORPUS.update((c["id"], c) for c in cases)
        return deepcopy(_RV120_CORPUS[key]["source"]), deepcopy(_RV120_CORPUS[key]["invocation"])
    assert kind == "m3x_producer"
    return m3x_producer(key)


def _rv120_materialized(shape):
    source, invocation = _rv120_base(shape)
    for value, edits in ((source, shape["edits"]), (invocation, shape["invocation_edits"])):
        for edit in edits:
            at = value
            for k in edit["path"][:-1]:
                at = at[k]
            if edit["op"] == "remove":
                del at[edit["path"][-1]]
            else:
                at[edit["path"][-1]] = deepcopy(edit["value"])
    return reseal(source, invocation, rp.EXACT_DEFINITION_HASH), invocation


def _rv120_digest(source, invocation):
    return hashlib.sha256(json.dumps([source, invocation], sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def rv120_input(shape):
    source, invocation = _rv120_materialized(shape)
    assert _rv120_digest(source, invocation) == shape["input_sha256"], shape["name"]
    return source, invocation


# T4-U6: RV120's shapes are the Rust reader's B3b shapes (`retained_precision_contract.rs` `rv120_forgeries`,
# `rv120_probes`, `repair03_probes`) stated as edits from this file's bases. Regeneration only:
# `RV120_INPUTS_WRITE=<B3B_INPUTS_OUT's JSON lines>` re-derives each shape's edits from that materialization, checks that
# the edits rebuild it exactly through `_rv120_materialized` and that the Rust shape's expected first failure is the
# shape's documented gate, and rewrites the file (names, wants and bases unchanged); otherwise a no-op.
RV120_RUST_SHAPES = {
    "X G5b control: case 0's evidence As_m2 one ulp [two_case_synthetic]": ("RV120 F1 control: case 0's evidence As_m2 one ulp", "two_case_synthetic"),
    "X G5b control: case 1's body_scales force one ulp [two_case_synthetic]": ("RV120 F1 control: case 1's body_scales force one ulp", "two_case_synthetic"),
    "X G5b order: case 0's evidence As_m2 and case 1's body_scales force [two_case_synthetic]": ("RV120 F1 order: case 0's evidence As_m2 and case 1's body_scales force", "two_case_synthetic"),
    "X G5b order: case 0's evidence As_m2 and case 1's section term area [two_case_synthetic]": ("RV120 F1 order: case 0's evidence As_m2 and case 1's section term area", "two_case_synthetic"),
    "forge E-hat+1ulp, every copy, native hashes resealed [ordinary_prepared_synthetic]": ("RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", "ordinary_prepared_synthetic"),
    "forge G-hat+1ulp, every copy, native hashes resealed [ordinary_prepared_synthetic]": ("RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", "ordinary_prepared_synthetic"),
    "forge E-hat+1ulp, every copy, native hashes resealed [m3x_sparse_interactive]": ("RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", "m3x_sparse_interactive"),
    "forge G-hat+1ulp, every copy, native hashes resealed [m3x_sparse_interactive]": ("RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", "m3x_sparse_interactive"),
    "X G8: an exact_cases entry for a case not in the invocation [ordinary_prepared_synthetic]": ("RV120 N2: an exact_cases entry for a case not in the invocation", "ordinary_prepared_synthetic"),
    "X G8: an exact_cases entry for a case not in the invocation [m3x_sparse_interactive]": ("RV120 N2: an exact_cases entry for a case not in the invocation", "m3x_sparse_interactive"),
    "N2b: entry 0's profile_mode and entry 1's material_basis [two_case_synthetic]": ("N2b: entry 0's profile_mode and entry 1's material_basis", "two_case_synthetic"),
    "N2b: entry 0's material_basis and entry 1's profile_mode [two_case_synthetic]": ("N2b: entry 0's material_basis and entry 1's profile_mode", "two_case_synthetic"),
}
# The 07e rule recomputes these members; edits leave them to it.
_RV120_RESEALED = (("retained_precision", "receipt_sha256"), ("retained_precision", "body", "publication_sha256"),
                   ("retained_precision", "body", "invocation", "value"))


def _rv120_edits(base, target, path, out):
    """Edits in this file's grammar turning `base` into `target`: removed and added keys, changed members of objects
    and of equal-length arrays, any other change set whole."""
    canonical = lambda v: json.dumps(v, sort_keys=True)  # noqa: E731
    if isinstance(base, dict) and isinstance(target, dict):
        out.extend({"op": "remove", "path": path + [k]} for k in base if k not in target)
        for k, v in target.items():
            if k not in base:
                out.append({"op": "set", "path": path + [k], "value": v})
            elif canonical(base[k]) != canonical(v):
                _rv120_edits(base[k], v, path + [k], out)
    elif isinstance(base, list) and isinstance(target, list) and len(base) == len(target):
        for i, (x, y) in enumerate(zip(base, target)):
            if canonical(x) != canonical(y):
                _rv120_edits(x, y, path + [i], out)
    else:
        out.append({"op": "set", "path": path, "value": target})


def _rv120_resealed(path):
    return (tuple(path) in _RV120_RESEALED
            or (len(path) == 6 and path[:3] == ["retained_precision", "body", "cases"] and path[4:] == ["source_identity_sha256"])
            or (len(path) == 6 and path[:3] == ["retained_precision", "body", "sources"] and path[4:] == ["preparation", "sha256"]))


def test_rv120_inputs_write():
    import os
    inputs_path = os.environ.get("RV120_INPUTS_WRITE")
    if not inputs_path:
        return
    rust = {}
    for line in Path(inputs_path).read_text().splitlines():
        entry = json.loads(line)
        rust[(entry["name"], entry["base"])] = entry
    path = ROOT / "fixtures/results/retained_precision_rv120_b3_inputs.json"
    document = json.loads(path.read_text())
    assert sorted(s["name"] for s in document["shapes"]) == sorted(RV120_RUST_SHAPES)
    for shape in document["shapes"]:
        target = rust[RV120_RUST_SHAPES[shape["name"]]]
        assert shape["want"].split()[0] == target["expected_bound"]["gate"], shape["name"]
        base_source, base_invocation = _rv120_base(shape)
        edits, invocation_edits = [], []
        _rv120_edits(base_source, target["source"], [], edits)
        _rv120_edits(base_invocation, target["invocation"], [], invocation_edits)
        shape["edits"] = [e for e in edits if not _rv120_resealed(e["path"])]
        shape["invocation_edits"] = invocation_edits
        source, invocation = _rv120_materialized(shape)
        assert (source, invocation) == (target["source"], target["invocation"]), shape["name"]
        shape["input_sha256"] = _rv120_digest(source, invocation)
    path.write_text(json.dumps(document, indent=1) + "\n")


def _rv120_want(text):
    gate, code = text.split()
    return (gate, "RETAINED_PRECISION_" + code)


RV120_G5B = [s for s in RV120["shapes"] if s["name"].startswith("X G5b")]
RV120_FORGERIES = [s for s in RV120["shapes"] if s["name"].startswith("forge")]
RV120_N2 = [s for s in RV120["shapes"] if s["name"].startswith("X G8: an exact_cases entry")]
REPAIR03_N2B = [s for s in RV120["shapes"] if s["name"].startswith("N2b: ")]


@pytest.mark.parametrize("shape", RV120_G5B, ids=[s["name"] for s in RV120_G5B])
def test_repair01_g5b_shared_checks_precede_the_exact_evidence(shape):
    """F1: the two-fault order probe (case 0's evidence As_m2 and case 1's body_scales force, each one ulp) reads
    G5b SCALE_MISMATCH, as RS and TS read it; each fault alone, and the other order probe, read as before."""
    assert len(RV120_G5B) == 4
    source, invocation = rv120_input(shape)
    assert outcome(source, invocation) == _rv120_want(shape["want"])
    assert outcome(source, None) == _rv120_want(shape["want"])
    # T4-U6 (D-5 §2.3, §5): case 0's evidence As_m2 is now its nominal section's leaf; with coincident walls the
    # reduced leaf no longer equals it, a section stated twice, so transport refuses the statement too (RS:
    # G7 SOURCE_PHYSICS_SECTION_BASES); the force-only control still reads as its base.
    as_edit = "case 0's evidence As_m2" in shape["name"]
    expected = BASE_G7 if as_edit else ("pass", False, "needs_recompute")
    assert outcome(source, None, transport=True) == expected


@pytest.mark.parametrize("shape", RV120_FORGERIES, ids=[s["name"] for s in RV120_FORGERIES])
def test_repair01_rv120_forgeries_are_refused_at_g8(shape):
    """F2: RV120's four E and G-hat forgeries (on its exact ordinary_prepared_synthetic and lane P's m3x successor)
    are refused at G8 PREPARATION_MISMATCH; unbound and on transport G8 does not run, so each reads as its base."""
    assert len(RV120_FORGERIES) == 4 and shape["want"] == "G8 PREPARATION_MISMATCH"
    source, invocation = rv120_input(shape)
    assert outcome(source, invocation) == PREPARATION
    assert outcome(source, None) == ("pass", False, "needs_recompute")
    assert outcome(source, None, transport=True) == ("pass", False, "needs_recompute")


@pytest.mark.parametrize("shape", RV120_N2, ids=[s["name"] for s in RV120_N2])
def test_repair02_rv120_n2_reads_the_cases_in_array_order(shape):
    """N2 (I101 repair 02): an exact_cases entry for a case not in the invocation (entry 0 copied, renamed), on RV120's
    synthetic exact base and lane P's m3x successor. Bound and unbound read G7's case coverage. On transport the cases
    are read in array order, so entry 0 passes and the copy's repeated maximum result id is read, run after run, as RS
    (array-ordered since repair 02) and TS read it."""
    assert len(RV120_N2) == 2
    source, invocation = rv120_input(shape)

    def read(bound, transport):
        try:
            if transport:
                rp.validate_retained_precision_transport(deepcopy(source))
            else:
                rp.validate_retained_precision(deepcopy(source), deepcopy(invocation) if bound else None)
        except rp.RetainedPrecisionError as error:
            return (error.gate, error.code, error.detail)
        return None

    coverage = ("G7", "SOURCE_PHYSICS_EVIDENCE_INVALID", "SOURCE_PHYSICS_EVIDENCE_INVALID: case coverage")
    for _ in range(8):
        assert read(True, False) == coverage
        assert read(False, False) == coverage
        assert read(False, True) == ("G7", "SOURCE_PHYSICS_EVIDENCE_INVALID", "SOURCE_PHYSICS_EVIDENCE_INVALID: transport maximum result ID")


@pytest.mark.parametrize("shape", REPAIR03_N2B, ids=[s["name"] for s in REPAIR03_N2B])
def test_repair03_n2b_reads_the_cases_in_array_order(shape):
    """N2b (I101 repair 03): two faults in the two exact_cases entries of the exact two_case_synthetic (one entry's
    profile_mode "x", the other's material_basis not a string), and the swap, on the Rust reader's materialized input
    (input_sha256). PY reads its cases in array order (a dict's insertion order), and both faults fail its one case check,
    so bound and unbound read G7 "case profile/material basis", run after run; transport refuses the evidence shape."""
    assert len(REPAIR03_N2B) == 2
    source, invocation = rv120_input(shape)

    def read(bound, transport):
        try:
            if transport:
                rp.validate_retained_precision_transport(deepcopy(source))
            else:
                rp.validate_retained_precision(deepcopy(source), deepcopy(invocation) if bound else None)
        except rp.RetainedPrecisionError as error:
            return (error.gate, error.code, error.detail)
        return None

    case = ("G7", "SOURCE_PHYSICS_EVIDENCE_INVALID", "SOURCE_PHYSICS_EVIDENCE_INVALID: case profile/material basis")
    for _ in range(8):
        assert read(True, False) == case
        assert read(False, False) == case
        assert read(False, True) == ("G7", "SOURCE_PHYSICS_EVIDENCE_INVALID", "SOURCE_PHYSICS_EVIDENCE_INVALID: transport evidence shape")


def test_b7a_preview_namespace_keys_on_the_contracts_absence_not_the_version():
    """B7a (b7-plan "Model format, pressure-free half"): G8's preview namespace is the current model version 0.7.0
    without a pressure contract. Keying it on the version string alone (a 0.7.0 model with a contract would pass) or
    on the retired 0.1.0/0.2.0 fails here."""
    def model(version, **extra):
        return {"schema_version": version, "project": {"id": "p"}, **extra}
    assert rp._preview_namespace(model("0.7.0"))
    assert not rp._preview_namespace(model("0.7.0", pressure_contract=None))
    for contract in ({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}, {}, False):
        assert not rp._preview_namespace(model("0.7.0", pressure_contract=contract)), contract
        assert not rp._exact_namespace(model("0.7.0", pressure_contract=contract)), contract
    for version in ("0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.5.1", "0.6.0"):
        assert not rp._preview_namespace(model(version)), version
    assert not rp._preview_namespace({"project": {"id": "p"}})
    exact = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
    assert rp._exact_namespace(model("0.7.0", pressure_contract=exact))
    for value in [None, "euler_bernoulli", False, 1, [], {}]:
        assert not rp._preview_namespace(model("0.7.0", shear_model=value))
        assert not rp._exact_namespace(model("0.7.0", pressure_contract=exact, shear_model=value))
