"""T4-U4b review repairs (DEL-04-07 Design u4b-u5b-implementation.md §2.4, §2.6, §5.1): tampers the model-holding
readers must refuse, on the shared corpus's two arc envelopes, read by the independent verifier
(``tests/verifiers``) with and without the captured model.

The Rust readers carry the same controls in ``core/product_physics/src/u4b_tests.rs``.
"""
from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest

from tests.verifiers import reader_verdict as rv
from tests.verifiers import u4b

PROJECT = Path(__file__).resolve().parents[1]
EXACT = (PROJECT / "fixtures/results/pressure_u4b_arc_sparse.json", PROJECT / "core/product_physics/tests/fixtures/u4b_l90_p_model.json", "pressure", True)
PREVIEW = (PROJECT / "fixtures/results/preview_physics_u4b_arc_sparse.json", PROJECT / "core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json", "preview-physics", False)


def _load(entry):
    envelope, model, reader, exact = entry
    return json.loads(envelope.read_text()), json.loads(model.read_text()), reader, exact


def _code(outcome) -> str:
    error = outcome.get("error") or {}
    return f"{error.get('code')}: {error.get('detail', '')}"


def _directional(row) -> bool:
    return row.get("kind") in (u4b.DIRECTIONAL, u4b.DIRECTIONAL_MAX)


def _records(doc, exact):
    return doc["contract_evidence"]["exact_cases" if exact else "preview_cases"]


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2"])
def test_the_arc_envelopes_pass_with_and_without_the_model(entry):
    doc, model, reader, _ = _load(entry)
    assert rv.validate(reader, doc)["ok"] is True, _code(rv.validate(reader, doc))
    assert rv.validate(reader, doc, model)["ok"] is True, _code(rv.validate(reader, doc, model))


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2"])
def test_a_dropped_directional_family_is_refused_only_with_the_model(entry):
    doc, model, reader, exact = _load(entry)
    dropped = copy.deepcopy(doc)
    dropped["results"] = [row for row in dropped["results"] if not _directional(row)]
    dropped["diagnostics"] = [d for d in dropped["diagnostics"] if d.get("code") != u4b.DIRECTIONAL_MAX_UNAVAILABLE]
    for case in _records(dropped, exact):
        case["intensified_measures"] = [r for r in case.get("intensified_measures", []) if "i_in_plane" not in r]
        if exact and not case["intensified_measures"]:
            del case["intensified_measures"]
    assert rv.validate(reader, dropped)["ok"] is True, _code(rv.validate(reader, dropped))
    refused = rv.validate(reader, dropped, model)
    assert refused["ok"] is False and "directional station coverage" in _code(refused), _code(refused)


def test_one_dropped_basis_is_refused_on_pressure_2():
    doc, model, reader, exact = _load(EXACT)
    reduced = lambda row: (row.get("metadata") or {}).get("basis", "").endswith("; section_basis=reduced_section")
    one = copy.deepcopy(doc)
    one["results"] = [row for row in one["results"] if not (_directional(row) and reduced(row))]
    for case in _records(one, exact):
        case["intensified_measures"] = [r for r in case.get("intensified_measures", []) if r.get("section_basis") != "reduced_section"]
    assert rv.validate(reader, one)["ok"] is False
    assert rv.validate(reader, one, model)["ok"] is False


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2"])
def test_consistently_changed_factors_are_refused_only_with_the_model(entry):
    doc, model, reader, exact = _load(entry)
    halved = copy.deepcopy(doc)
    factors = {}
    for case in _records(halved, exact):
        for record in case.get("intensified_measures", []):
            if "i_in_plane" in record:
                record["i_in_plane"] /= 2.0
                record["i_out_of_plane"] /= 2.0
                factors[record["result_id"]] = (record["i_in_plane"], record["i_out_of_plane"])
    index = u4b.Rows(halved["results"])
    changed = 0
    for row in halved["results"]:
        if row.get("kind") == u4b.DIRECTIONAL:
            i_ip, i_op = factors[row["id"]]
            md = row["metadata"]
            basis = u4b._basis(row)
            y = index.value(u4b._case(row), row["entity_ref"], "element_local_bending_normal_stress_y", "bending_normal_stress_y", md["location"], basis)
            z = index.value(u4b._case(row), row["entity_ref"], "element_local_bending_normal_stress_z", "bending_normal_stress_z", md["location"], basis)
            row["value"] = u4b.directional(i_ip, i_op, y, z)
            changed += 1
    assert changed >= 5
    assert rv.validate(reader, halved)["ok"] is True, _code(rv.validate(reader, halved))
    refused = rv.validate(reader, halved, model)
    assert refused["ok"] is False and "directional factors differ from the model" in _code(refused), _code(refused)


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2"])
def test_directional_rows_on_a_member_without_the_models_pair_are_refused(entry):
    doc, model, reader, _ = _load(entry)
    stripped = copy.deepcopy(model)
    for component in stripped.get("components", []):
        (component.get("modifiers") or {}).pop("sif_in_plane_user_value", None)
        (component.get("modifiers") or {}).pop("sif_out_of_plane_user_value", None)
    refused = rv.validate(reader, doc, stripped)
    assert refused["ok"] is False and "outside the model's arcs" in _code(refused), _code(refused)


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2-in-domain"])
def test_a_forged_poisson_warning_with_deleted_shear_rows_is_refused(entry):
    """Review item 3: pressure-2 admits only -1 < nu < 1/2 and refuses the exemption outright; on preview in the
    domain a model-free read keeps custody only (as designed) and the model-holding read refuses it."""
    doc, model, reader, exact = _load(entry)
    case_id = _records(doc, exact)[0]["load_case_id"]
    forged = copy.deepcopy(doc)
    forged["results"] = [row for row in forged["results"] if not (row.get("kind") == u4b.SHEAR_MAX and row.get("entity_ref") == "pipe:S1")]
    forged["diagnostics"].append({"id": "diagnostic:transverse-shear:forged:poisson", "code": u4b.POISSON_OUT_OF_DOMAIN,
                                  "severity": "warning", "message": "forged", "affected_refs": [case_id, "pipe:S1"]})
    free = rv.validate(reader, forged)
    if exact:
        assert free["ok"] is False and "Poisson exemption on pressure-2" in _code(free), _code(free)
    else:
        assert free["ok"] is True, _code(free)
        refused = rv.validate(reader, forged, model)
        assert refused["ok"] is False and "Poisson warning does not match" in _code(refused), _code(refused)
