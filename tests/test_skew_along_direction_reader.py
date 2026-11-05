"""R-6/R-7 S-K1 (DEL-04-03 Work skew-restraints.md sections 7.1, 10.5 and 11.1 R-1): the Python verifier's
re-check of `support_reaction_along_direction_v1` on the committed product fixture
(`fixtures/results/skew_along_direction_preview.json`, written by PP's `the_reader_fixture_is_the_product_output`):
the document reads with its model, and each tampered document is refused by name."""
from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest

from tests.verifiers.preview_physics_evidence import validate_preview_physics_evidence

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = json.loads((ROOT / "fixtures/results/skew_along_direction_preview.json").read_text(encoding="utf-8"))
KIND = "support_reaction_along_direction_v1"


def _source() -> dict:
    return copy.deepcopy(FIXTURE["envelope"])


def _model() -> dict:
    return copy.deepcopy(FIXTURE["request"]["model"])


def _at(source: dict, kind: str, entity: str, component: str, ref_type: str = "load_case") -> int:
    return next(i for i, r in enumerate(source["results"]) if r["kind"] == kind and r["entity_ref"] == entity
                and r["metadata"]["component"] == component and r["basis_ref"]["ref_type"] == ref_type)


def test_the_fixture_reads_with_its_model():
    source = _source()
    along = [r for r in source["results"] if r["kind"] == KIND]
    # One row per skewed record and case, and the combination's.
    assert sorted((r["entity_ref"], r["basis_ref"]["ref_type"]) for r in along) == [
        ("spring_B", "combination"), ("spring_B", "load_case"), ("stop_B", "combination"), ("stop_B", "load_case")]
    validate_preview_physics_evidence(source, _model())


@pytest.mark.parametrize("entity", ["stop_B", "spring_B"])
def test_tampered_documents_are_refused_by_name(entity):
    model = _model()
    for tamper, code in [
        (lambda s: s["results"][_at(s, KIND, entity, "along_direction")].update(value=-s["results"][_at(s, KIND, entity, "along_direction")]["value"]),
         "ALONG_DIRECTION_INCONSISTENT"),
        (lambda s: s["results"][_at(s, KIND, entity, "along_direction")].update(value=s["results"][_at(s, KIND, entity, "along_direction")]["value"] * (1 + 1e-9)),
         "ALONG_DIRECTION_INCONSISTENT"),
        # The case row and the combination row that names it (a dangling source reference is refused first).
        (lambda s: (s["results"].pop(_at(s, KIND, entity, "along_direction", "combination")),
                    s["results"].pop(_at(s, KIND, entity, "along_direction"))), "ALONG_DIRECTION_MISSING"),
        (lambda s: (s["results"].pop(_at(s, KIND, entity, "along_direction", "combination")),
                    s["results"][_at(s, KIND, entity, "along_direction")].update(id="result:support-along-direction-v1:x")), "ALONG_DIRECTION_ROW_ID"),
    ]:
        source = _source()
        tamper(source)
        with pytest.raises(ValueError, match=code):
            validate_preview_physics_evidence(source, model)
    # An along row on a record that is not skewed.
    unskewed = _model()
    for support in unskewed["supports"]:
        if support["id"] == entity:
            del support["direction"]
    with pytest.raises(ValueError, match="ALONG_DIRECTION_UNEXPECTED"):
        validate_preview_physics_evidence(_source(), unskewed)


COMBINATIONS = json.loads((ROOT / "fixtures/results/skew_along_direction_combinations_preview.json").read_text(encoding="utf-8"))


def test_the_combinations_fixture_reads_with_its_model():
    """Two cases and one combination of each PF form (signed mechanics, absolute sum, SRSS, range envelope, subtraction):
    the document reads with its model; the combination verifier (``preview_combination_gates.check`` with the model)
    re-derives each combination's along-direction row in its support's group, as the producer forms it, and refuses an
    absolute sum's along row with a changed value."""
    from tests.verifiers import preview_combination_gates
    source, model = copy.deepcopy(COMBINATIONS["envelope"]), copy.deepcopy(COMBINATIONS["request"]["model"])
    validate_preview_physics_evidence(source, model)
    preview_combination_gates.check(source, model)
    along = [r for r in source["results"] if r["kind"] == KIND and r["basis_ref"]["ref_type"] == "combination"]
    assert sorted({r["basis_ref"]["ref_id"] for r in along}) == ["C_ABS", "C_DIFF", "C_ENV", "C_SIGNED", "C_SRSS"]
    assert len(along) == 10
    tampered = copy.deepcopy(COMBINATIONS["envelope"])
    row = next(r for r in tampered["results"] if r["kind"] == KIND and r["basis_ref"]["ref_id"] == "C_ABS")
    row["value"] = row["value"] * (1 + 1e-9)
    with pytest.raises(ValueError):
        preview_combination_gates.check(tampered, model)


def _synthetic(d, r, s):
    """One case's along row and its three component rows, and the model's one skewed stop (`_along_direction`)."""
    case, entity = "case:skew", "stop_B"
    md = {"component": "along_direction", "coordinate_system": "support_direction", "location": "node",
          "basis": "recovered_from_assembled_support_law", "sign_convention": "support_on_pipe_positive_along_authored_direction"}
    rows = [{"id": f"result:support-along-direction-v1:{len(case)}:{case}{len(entity)}:{entity}", "kind": KIND, "entity_ref": entity,
             "value": s, "unit": "N", "metadata": md, "basis_ref": {"ref_type": "load_case", "ref_id": case}}]
    rows += [{"id": f"r:{c}", "kind": "support_reaction_component_v2", "entity_ref": entity, "value": v, "metadata": {"component": c},
              "basis_ref": {"ref_type": "load_case", "ref_id": case}} for c, v in zip(("Fx", "Fy", "Fz"), r)]
    model = {"supports": [{"id": entity, "family": "line_stop", "direction": dict(zip("xyz", d))}]}
    return {"results": rows}, model


def test_a_large_finite_reaction_never_makes_the_guard_accept_a_wrong_value():
    """Astra B2: R = (8e307, 8e307, 0) made norm*sum|R_i| overflow, so the guard accepted s = 0. R and s are now scaled
    together by an exact power of two: the true value reads, a wrong one is refused."""
    import math
    from tests.verifiers.preview_physics_evidence import _along_direction
    d, r = (1.0, 1.0, 0.0), (8e307, 8e307, 0.0)
    true = 8e307 * math.sqrt(2.0)
    _along_direction(*_synthetic(d, r, true))
    for wrong in (0.0, -true, true * (1 + 1e-9)):
        with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
            _along_direction(*_synthetic(d, r, wrong))
    big = (1.7976931348623157e308, -1.7976931348623157e308, 1.7976931348623157e308)
    _along_direction(*_synthetic((1.0, 1.0, 1.0), big, 1.7976931348623157e308 / math.sqrt(3.0)))
    with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
        _along_direction(*_synthetic((1.0, 1.0, 1.0), big, 0.0))
