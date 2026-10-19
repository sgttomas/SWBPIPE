"""O4 (DEL-04-08 Design ``o4-source-extension.md`` §3.4, §6): the independent Python verifier, beside the Rust reader
(``openpipestress-runner read``), on genuine Timoshenko and dual-basis pressure-source-1 publications.

The controls are the test-only fixture ``fixtures/product_preview/pressure_source/o4/o4_source_controls.json``, which
the producer test ``source_receipt::o4_source_tests::o4_source_controls_are_current`` holds current: in-memory
re-authorings of the committed n05 and fields requests (a Timoshenko twin, a nonzero mill tolerance and allowance, both,
and a two-member document with one distinct-basis and one bit-equal member). Each mutation below is made in memory on a
genuine publication and resealed, so only the named statement is wrong; both readers refuse it by the same name.
"""
from __future__ import annotations

from copy import deepcopy
import json
import math
from pathlib import Path

import pytest

from core.analysis_runs import rust_reader
from tests.verifiers import physics_source as ps
from tests.verifiers import reader_verdict as verdict
from tests.verifiers import source_blocks as blocks

ROOT = Path(__file__).resolve().parents[1]
CONTROLS = ROOT / "fixtures/product_preview/pressure_source/o4/o4_source_controls.json"
EXACT = "retained_source_blocks_exact_v1"


def controls():
    return json.loads(CONTROLS.read_text())["entries"]


def control(control_id, mode="sparse_interactive"):
    entry = next(e for e in controls() if e["id"] == control_id and e["solver_mode"] == mode)
    return deepcopy(entry["source"]), deepcopy(entry["invocation"])


def reseal(source):
    """The physical case commitments, the publication commitment and the receipt, over the (mutated) statement."""
    evidence = source["contract_evidence"]
    body = source["source_block_recovery"]["body"]
    for index, case in enumerate(body["cases"]):
        exact = evidence["exact_cases"][index]
        proof = {"exact_case": exact, "pressure": [p for p in evidence["pressure"] if p["load_case_id"] == exact["load_case_id"]]}
        case["physical_evidence_sha256"] = blocks.domain_hash("pressure_source_case_evidence_v1", proof)
    publication = {k: v for k, v in source.items() if k != "source_block_recovery"}
    body["publication_sha256"] = blocks.domain_hash("source_blocks_publication_v1", publication)
    source["source_block_recovery"]["receipt_sha256"] = blocks.domain_hash("source_blocks_receipt_v1", body)
    return source


# The Python base verifier names a twin-rule failure by its own text (``physics_evidence.py``), not by Rust's code.
PYTHON_NAMES = {"SOURCE_PHYSICS_ROW_BASIS_SECTION": "SOURCE_PHYSICS_EVIDENCE_INVALID: stress basis section"}


def refused_by_both(source, invocation, code):
    python = PYTHON_NAMES.get(code, code)
    with pytest.raises(ValueError) as refused:
        ps.validate_physics_source(deepcopy(source), deepcopy(invocation))
    assert str(refused.value) == python
    with pytest.raises(rust_reader.ReaderRefusal) as rust:
        rust_reader.validate("pressure-source", source, invocation=invocation)
    assert rust.value.code == code


def is_reduced(row_id):
    return row_id.endswith(":reduced-section") or row_id.endswith(":reduced_section")


def twin(row_id):
    for reduced, nominal in ((":reduced-section", ":nominal-section"), (":reduced_section", ":nominal_section")):
        if row_id.endswith(reduced):
            return row_id[: -len(reduced)] + nominal
    raise AssertionError(row_id)


def selected(source):
    return next(c for c in source["source_block_recovery"]["body"]["cases"] if c["selected_method"] == EXACT)


def distinct(section):
    return any(section["nominal_section"][k] != section["reduced_section"][k] for k in ("As_m2", "Z_m3", "J_m4"))


@pytest.mark.parametrize("entry", controls(), ids=lambda e: f"{e['id']}-{e['solver_mode']}")
def test_o4_controls_read_eligible_in_both_readers(entry):
    source, invocation = deepcopy(entry["source"]), deepcopy(entry["invocation"])
    assert source["producer"]["semantic_contract_id"] == ps.PRESSURE_CONTRACT_ID
    assert ps.validate_physics_source(deepcopy(source), deepcopy(invocation)) is True
    ps.validate_transport_metadata(deepcopy(source))
    assert rust_reader.validate("pressure-source", source, invocation=invocation)["eligible"] is True
    python = verdict.validate("pressure-source", deepcopy(source), invocation["request"]["model"], deepcopy(invocation))
    assert python["ok"] is True and python["eligible"] is True, python


def test_o4_reduced_checks_bind_the_reduced_section():
    """§3.2/§3.4: a reduced check's parameters are the reduced section's and its action its nominal twin's; mutation
    ``reduced_check_nominal_parameters`` (the checks alone) is refused STRESS_SECTION_PARAMETERS, and with the reduced rows
    then carrying the nominal values the twin rule refuses it first."""
    source, invocation = control("dual-basis-fields")
    case = selected(source)
    section = source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
    assert distinct(section)
    checks = {c["result_id"]: c for c in case["section_stress_checks"]}
    reduced = [c for c in case["section_stress_checks"] if is_reduced(c["result_id"])]
    assert len(reduced) == 20
    for check in reduced:
        nominal = checks[twin(check["result_id"])]
        assert all(check[k] == nominal[k] for k in ("functional_index", "functional_id", "action"))
        assert check["parameters"] == {"area_m2": section["reduced_section"]["As_m2"], "section_modulus_m3": section["reduced_section"]["Z_m3"],
                                       "torsion_radius_m": section["ro_m"], "torsion_constant_m4": section["reduced_section"]["J_m4"], "pa_per_mpa": 1e6}
    mutated = deepcopy(source)
    for check in selected(mutated)["section_stress_checks"]:
        if is_reduced(check["result_id"]):
            check["parameters"] = deepcopy(checks[twin(check["result_id"])]["parameters"])
    refused_by_both(reseal(mutated), invocation, "PHYSICS_SOURCE_STRESS_SECTION_PARAMETERS")
    values = {r["id"]: r["value"] for r in source["results"]}
    for row in mutated["results"]:
        if row["id"] in checks and is_reduced(row["id"]):
            row["value"] = values[twin(row["id"])]
    refused_by_both(reseal(mutated), invocation, "SOURCE_PHYSICS_ROW_BASIS_SECTION")


def test_o4_maximum_is_per_basis():
    """§3.2: each basis's maximum binds its own section's A and Z; mutation ``reduced_maximum_copied`` (the nominal
    maximum, value and evidence, published as the reduced one) is refused MAXIMUM_SECTION."""
    source, invocation = control("dual-basis-fields")
    exact = source["contract_evidence"]["exact_cases"][0]
    section = exact["pipe_sections"][0]
    extrema = {e["section_basis"]: e for e in exact["pipe_stress_extrema"]}
    for basis, extremum in extrema.items():
        assert extremum["area_m2"] == section[basis]["As_m2"] and extremum["section_modulus_m3"] == section[basis]["Z_m3"]
    assert extrema["reduced_section"]["value_pa"] > extrema["nominal_section"]["value_pa"]
    copied = deepcopy(source)
    target = next(i for i, e in enumerate(copied["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]) if e["section_basis"] == "reduced_section")
    replacement = {**deepcopy(extrema["nominal_section"]), "section_basis": "reduced_section", "result_id": extrema["reduced_section"]["result_id"]}
    copied["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][target] = replacement
    for row in copied["results"]:
        if row["id"] == replacement["result_id"]:
            row["value"] = replacement["value_pa"]
    headline = copied["summary"]["max_open_formula_stress_reduced_section"]
    if headline and headline["result_ref"] == replacement["result_id"]:
        headline["value"] = replacement["value_pa"]
    refused_by_both(reseal(copied), invocation, "PHYSICS_SOURCE_MAXIMUM_SECTION")


@pytest.mark.parametrize("key", ["shear_area_m2", "shear_factor"])
def test_o4_shear_area_and_kappa_mutations_are_refused(key):
    """§3.4: ``shear_area_changed`` and ``kappa_changed`` (one ulp, resealed) fail the transport rederivation."""
    source, invocation = control("timoshenko-n05")
    theory = source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"]
    theory[key] = math.nextafter(theory[key], math.inf)
    reseal(source)
    refused_by_both(source, invocation, "PHYSICS_SOURCE_SECTION_BASES")
    with pytest.raises(ValueError, match="^PHYSICS_SOURCE_SECTION_BASES$"):
        ps.validate_transport_metadata(deepcopy(source))


def test_o4_a_dropped_timoshenko_leaf_is_refused_by_the_contract_selection():
    source, invocation = control("timoshenko-n05")
    source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"] = {"model": "euler_bernoulli"}
    reseal(source)
    ps.validate_transport_metadata(deepcopy(source))
    refused_by_both(source, invocation, "PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL")


@pytest.mark.parametrize("pipe,bases,code", [
    ("independent-member", ("nominal_section",), "SOURCE_PHYSICS_SHEAR_VALUE"),
    ("independent-member", ("reduced_section",), "SOURCE_PHYSICS_SHEAR_VALUE"),
    ("independent-member-2", ("nominal_section", "reduced_section"), "SOURCE_PHYSICS_SHEAR_VALUE"),
    ("independent-member-2", ("reduced_section",), "SOURCE_PHYSICS_ROW_BASIS_SECTION"),
])
def test_o4_one_basis_tamper_on_a_distinct_basis_member(pipe, bases, code):
    """§6 (MJ9's both-bases tamper, revised): on the distinct-basis member a one-ulp tau_v,max on one basis alone is
    refused by that basis's own shear check; on the bit-equal member the both-bases tamper keeps its outcome and a
    one-basis tamper meets the coincident twin rule."""
    source, invocation = control("mixed-two-member")
    sections = {s["pipe_id"]: s for s in source["contract_evidence"]["exact_cases"][0]["pipe_sections"]}
    assert distinct(sections["independent-member"]) and not distinct(sections["independent-member-2"])
    changed = 0
    for row in source["results"]:
        if (row["entity_ref"] == pipe and row["kind"] == "element_local_transverse_shear_stress_maximum"
                and row["metadata"]["location"] == "end_i" and any(row["metadata"]["basis"].endswith(f"; section_basis={b}") for b in bases)):
            row["value"] = math.nextafter(row["value"], math.inf)
            changed += 1
    assert changed == len(bases)
    refused_by_both(reseal(source), invocation, code)
