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
