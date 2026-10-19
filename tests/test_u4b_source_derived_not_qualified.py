"""T4-U4b §4.3 (S4): `derived_not_qualified` on genuine pressure-source-1 producer bytes.

A selected source case publishes its section normal-stress extremes and transverse-shear maxima beside the qualified
rows, formed from them, and not qualified. The receipt carries them as `derived_not_qualified`: no projection, one of
the two formula recipes and exactly the station rows the formula reads. These checks read the committed producer twins
(both families and modes) with the independent Python verifier and the receipt schema; every mutation is a resealed
negative statement, refused by name. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
"""
from copy import deepcopy
from functools import lru_cache
import json
from pathlib import Path

import pytest

from schema_validation import validate_instance
from tests.verifiers import physics_source as composite
from tests.verifiers import reader_verdict as verdict
from tests.verifiers.source_blocks import domain_hash

ROOT = Path(__file__).resolve().parents[1]
FAMILIES = ("physics_source", "load_reference_source")
MODES = ("sparse_interactive", "dense_scrutiny")
DERIVED = ("element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum")
RECIPES = {DERIVED[0]: "section_extremes_from_published_stress_rows_v1", DERIVED[1]: "transverse_shear_from_published_shear_rows_v1"}


@lru_cache(maxsize=None)
def schema():
    return json.loads((ROOT / "schemas/physics_source_recovery.schema.json").read_text())


def actual(family, mode, name="n05"):
    folder = ROOT / "fixtures/product_preview/pressure_source" / family
    return (json.loads((folder / f"{name}-{mode}.raw.json").read_text()),
            {"request": json.loads((folder / f"{name}.request.json").read_text()), "solver_mode": mode})


def reseal(source):
    """Recompute the receipt hash of a deliberately false statement; never a positive fixture."""
    receipt = source["source_block_recovery"]
    receipt["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", receipt["body"])


def kinds(source):
    return {row["id"]: row["kind"] for row in source["results"]}


@pytest.mark.parametrize("family", FAMILIES)
@pytest.mark.parametrize("mode", MODES)
def test_genuine_receipts_are_schema_valid_and_carry_the_treatment_for_the_two_kinds_only(family, mode):
    source, invocation = actual(family, mode)
    validate_instance(schema(), source["source_block_recovery"], instance_label="genuine pressure-source-1 receipt")
    assert composite.validate_physics_source(deepcopy(source), invocation) is (family == "physics_source")
    kind = kinds(source)
    rows = source["source_block_recovery"]["body"]["cases"][0]["rows"]
    treated = [t for t in rows if t["treatment"] == "derived_not_qualified"]
    members = len(invocation["request"]["model"]["pipe_segments"])
    assert len(treated) == 30 * members
    assert {kind[t["result_id"]] for t in treated} == set(DERIVED)
    assert all(kind[t["result_id"]] in DERIVED for t in treated)
    assert all(t["treatment"] == "derived_not_qualified" for t in rows if kind[t["result_id"]] in DERIVED)
    for t in treated:
        assert t["projection_id"] is None and t["recipe_id"] == RECIPES[kind[t["result_id"]]]
        expected = (["element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"]
                    if kind[t["result_id"]] == DERIVED[0] else ["element_local_shear_force_y", "element_local_shear_force_z"])
        assert [kind[i] for i in t["input_result_ids"]] == expected
    if family == "physics_source":
        standing = verdict.standing(source, invocation["request"]["model"], invocation)
        assert standing["eligible"] is True and standing["findings"] == [], standing


def mutated(source, index, edit):
    doc = deepcopy(source)
    edit(doc["source_block_recovery"]["body"]["cases"][0]["rows"][index])
    reseal(doc)
    return doc


@pytest.mark.parametrize("mode", MODES)
def test_mutations_refuse_by_name(mode):
    source, invocation = actual("physics_source", mode)
    kind = kinds(source)
    rows = source["source_block_recovery"]["body"]["cases"][0]["rows"]
    sigma = next(i for i, t in enumerate(rows) if kind[t["result_id"]] == DERIVED[0])
    tau = next(i for i, t in enumerate(rows) if kind[t["result_id"]] == DERIVED[1])
    other = next(i for i, t in enumerate(rows) if kind[t["result_id"]] == "support_reaction_component_v2")
    tau_inputs = list(rows[tau]["input_result_ids"])

    def set_(**fields):
        return lambda t: t.update(fields)

    cases = [
        ("wrong recipe", sigma, set_(recipe_id=RECIPES[DERIVED[1]]), "SOURCE_BLOCKS_NOT_QUALIFIED_RECIPE"),
        ("unregistered recipe", tau, set_(recipe_id="retained_source_straight_stress_v1"), "SOURCE_BLOCKS_NOT_QUALIFIED_RECIPE"),
        ("treatment checked_derived", sigma, set_(treatment="checked_derived"), "SOURCE_BLOCKS_NOT_QUALIFIED_TREATMENT"),
        ("treatment inspection_only", tau, set_(treatment="inspection_only", recipe_id=None, input_result_ids=[]), "SOURCE_BLOCKS_NOT_QUALIFIED_TREATMENT"),
        ("projection named", sigma, set_(projection_id="projection:none"), "SOURCE_BLOCKS_NOT_QUALIFIED_ROW"),
        ("wrong kind", other, set_(treatment="derived_not_qualified", projection_id=None, recipe_id=RECIPES[DERIVED[0]]), "SOURCE_BLOCKS_NOT_QUALIFIED_KIND"),
        ("inputs reordered", sigma, lambda t: t.update(input_result_ids=list(reversed(t["input_result_ids"]))), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
        ("inputs of tau on sigma", sigma, set_(input_result_ids=tau_inputs), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
        ("input missing", tau, lambda t: t.update(input_result_ids=t["input_result_ids"][:1]), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
    ]
    for name, index, edit, code in cases:
        doc = mutated(source, index, edit)
        with pytest.raises(ValueError, match=code):
            composite.validate_physics_source(doc, invocation)


@pytest.mark.parametrize("mode", MODES)
def test_the_neutral_standing_is_for_the_two_kinds_only(mode):
    source, invocation = actual("physics_source", mode)
    kind = kinds(source)
    rows = source["source_block_recovery"]["body"]["cases"][0]["rows"]
    other = next(i for i, t in enumerate(rows) if kind[t["result_id"]] == "support_reaction_component_v2")
    doc = mutated(source, other, lambda t: t.update(treatment="derived_not_qualified", projection_id=None, recipe_id=None))
    assert "SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY" in verdict.source_findings(doc, invocation)
    assert verdict.source_findings(source, invocation) == []


# S4 round 2 (§4.3, §5.1): the composite reader recomputes every straight member's derived rows on every case. Each
# change below is made on both bases (end i), so the bit-equal twin rule of these coincident sections passes and the
# derived-row checks are reached; the publication and receipt are resealed.
def reseal_publication(source):
    publication = {key: value for key, value in source.items() if key != "source_block_recovery"}
    source["source_block_recovery"]["body"]["publication_sha256"] = domain_hash("source_blocks_publication_v1", publication)
    reseal(source)


def end_i(source, case, kind, component):
    found = [i for i, row in enumerate(source["results"])
             if row["basis_ref"]["ref_id"] == case and row["kind"] == kind and row["metadata"]["component"] == component
             and row["metadata"]["location"] == "end_i"]
    assert len(found) == 2 and {source["results"][i]["metadata"]["basis"].rsplit("=", 1)[1] for i in found} == {"nominal_section", "reduced_section"}
    return found


def case_ids(source):
    cases = source["source_block_recovery"]["body"]["cases"]
    selected = next(c["basis_ref"]["ref_id"] for c in cases if c["selected_method"] == "retained_source_blocks_exact_v1")
    ordinary = next(c["basis_ref"]["ref_id"] for c in cases if c["selected_method"] != "retained_source_blocks_exact_v1")
    return selected, ordinary


def nudged(source, case, kind, component):
    import math
    doc = deepcopy(source)
    for i in end_i(doc, case, kind, component):
        doc["results"][i]["value"] = math.nextafter(doc["results"][i]["value"], math.inf)
    reseal_publication(doc)
    return doc


@pytest.mark.parametrize("family", FAMILIES)
@pytest.mark.parametrize("mode", MODES)
def test_composite_recomputes_derived_rows_on_selected_and_ordinary_cases(family, mode):
    source, invocation = actual(family, mode, "mixed")
    selected, ordinary = case_ids(source)
    for doc, code in [
        (nudged(source, selected, DERIVED[1], "tau_v_max"), "SOURCE_PHYSICS_SHEAR_VALUE"),
        (nudged(source, ordinary, DERIVED[0], "sigma_plus"), "SOURCE_PHYSICS_EXTREME_VALUE"),
        (nudged(source, ordinary, DERIVED[0], "sigma_minus"), "SOURCE_PHYSICS_EXTREME_VALUE"),
        (nudged(source, ordinary, DERIVED[1], "tau_v_max"), "SOURCE_PHYSICS_SHEAR_VALUE"),
    ]:
        with pytest.raises(ValueError, match=f"^{code}$"):
            composite.validate_physics_source(doc, invocation)
        assert verdict.validate("pressure-source", doc, invocation["request"]["model"], invocation)["error"]["code"] == code


@pytest.mark.parametrize("family", FAMILIES)
@pytest.mark.parametrize("mode", MODES)
def test_deleting_derived_rows_with_their_treatments_is_a_coverage_refusal(family, mode):
    source, invocation = actual(family, mode, "mixed")
    selected, ordinary = case_ids(source)
    for case in (selected, ordinary):
        doc = deepcopy(source)
        ids = {doc["results"][i]["id"] for kind, component in ((DERIVED[0], "sigma_plus"), (DERIVED[0], "sigma_minus"), (DERIVED[1], "tau_v_max"))
               for i in end_i(doc, case, kind, component)}
        doc["results"] = [row for row in doc["results"] if row["id"] not in ids]
        removed = 0
        for receipt_case in doc["source_block_recovery"]["body"]["cases"]:
            before = len(receipt_case["rows"])
            receipt_case["rows"] = [t for t in receipt_case["rows"] if t["result_id"] not in ids]
            removed += before - len(receipt_case["rows"])
        assert removed == 6
        reseal_publication(doc)
        with pytest.raises(ValueError, match="^SOURCE_PHYSICS_EXTREME_COVERAGE$"):
            composite.validate_physics_source(doc, invocation)


# S4 round 3: the composite's ν is always in domain, so a TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN warning can never
# exempt one of its members. A false warning appended for a member whose ten τ rows (five stations, both bases) are
# deleted with their treatments, resealed, is refused by name on each recovery method's case.
@pytest.mark.parametrize("family", FAMILIES)
@pytest.mark.parametrize("mode", MODES)
def test_a_false_poisson_warning_cannot_exempt_deleted_tau_rows(family, mode):
    source, invocation = actual(family, mode, "mixed")
    for case in case_ids(source):
        doc = deepcopy(source)
        member = next(row["entity_ref"] for row in doc["results"] if row["basis_ref"]["ref_id"] == case and row["kind"] == DERIVED[1])
        ids = {row["id"] for row in doc["results"] if row["basis_ref"]["ref_id"] == case and row["kind"] == DERIVED[1] and row["entity_ref"] == member}
        assert len(ids) == 10
        doc["results"] = [row for row in doc["results"] if row["id"] not in ids]
        for receipt_case in doc["source_block_recovery"]["body"]["cases"]:
            receipt_case["rows"] = [t for t in receipt_case["rows"] if t["result_id"] not in ids]
        doc["diagnostics"].append({"id": f"diagnostic:transverse-shear:{case}:{member}:poisson", "code": "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN",
                                   "severity": "warning", "message": f"member {member} has nu = 0.75 outside -1 < nu < 1/2; no transverse-shear maximum is published for it in {case}",
                                   "source": "core/product_physics", "affected_refs": [case, member]})
        reseal_publication(doc)
        with pytest.raises(ValueError, match="^SOURCE_PHYSICS_POISSON_DOMAIN_WARNING$"):
            composite.validate_physics_source(doc, invocation)
        assert verdict.validate("pressure-source", doc, invocation["request"]["model"], invocation)["error"]["code"] == "SOURCE_PHYSICS_POISSON_DOMAIN_WARNING"


# Integrated-head review (T4): D-5's row order (section-bases-and-timoshenko.md §2.5) on the composite. The end-i σ⁺
# reduced row and σ⁻ nominal row exchange positions, values and ids unchanged, publication and receipt resealed: every
# twin and value still agrees, so only Rust's ROW_BASIS_TWIN_ORDER, which the composite verifier now also checks, finds
# the reordering, on each recovery method's case.
@pytest.mark.parametrize("family", FAMILIES)
@pytest.mark.parametrize("mode", MODES)
def test_a_broken_sigma_twin_order_is_refused_by_name(family, mode):
    source, invocation = actual(family, mode, "mixed")
    for case in case_ids(source):
        doc = deepcopy(source)
        plus = sorted(end_i(doc, case, DERIVED[0], "sigma_plus"))
        minus = sorted(end_i(doc, case, DERIVED[0], "sigma_minus"))
        assert doc["results"][plus[1]]["metadata"]["basis"].endswith("reduced_section") and plus[1] + 1 == minus[0]
        doc["results"][plus[1]], doc["results"][minus[0]] = doc["results"][minus[0]], doc["results"][plus[1]]
        reseal_publication(doc)
        with pytest.raises(ValueError, match="^SOURCE_PHYSICS_ROW_BASIS_TWIN_ORDER$"):
            composite.validate_physics_source(doc, invocation)
        assert verdict.validate("pressure-source", doc, invocation["request"]["model"], invocation)["error"]["code"] == "SOURCE_PHYSICS_ROW_BASIS_TWIN_ORDER"
