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


@pytest.mark.parametrize("fixture", [FIXTURE, COMBINATIONS], ids=["one-case", "combinations"])
def test_a_subnormal_scaled_direction_reads_as_its_twin_through_the_reader_entry(fixture):
    """Astra B3: every skewed direction scaled by 2^-1026 (subnormal components, the same admitted canonical direction) reads
    through validate_preview_physics_evidence as its unscaled twin; the scale is never formed (2.0 ** 1026 overflowed and
    the validator reported malformed evidence). A wrong along value is refused under the scaled model too."""
    import math
    source, model = copy.deepcopy(fixture["envelope"]), copy.deepcopy(fixture["request"]["model"])
    scaled = copy.deepcopy(model)
    for support in scaled["supports"]:
        if "direction" in support:
            support["direction"] = {axis: math.ldexp(value, -1026) for axis, value in support["direction"].items()}
    assert scaled != model
    validate_preview_physics_evidence(source, model)
    validate_preview_physics_evidence(copy.deepcopy(source), scaled)
    wrong = copy.deepcopy(source)
    row = next(r for r in wrong["results"] if r["kind"] == KIND and r["basis_ref"]["ref_type"] == "load_case")
    row["value"] = -row["value"]
    for m in (model, scaled):
        with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
            validate_preview_physics_evidence(copy.deepcopy(wrong), m)


def test_an_honest_subnormal_reaction_reads_and_a_subnormal_tamper_is_refused():
    """Astra B2': the finite-kappa spring outputs for k = 2^-1074, u = (1, 1, 0) (R = (-m, -m, 0), s = -m) read; the
    allowance floor is the unscaled one carried through the joint scaling. A subnormal tamper beyond it is refused."""
    import math
    from tests.verifiers.preview_physics_evidence import _along_direction
    m, d = math.ldexp(1.0, -1074), (1.0, 1.0, 0.0)
    _along_direction(*_synthetic(d, (-m, -m, 0.0), -m))
    for r, s in (((-m, -m, 0.0), -100 * m), ((-1000 * m, -1000 * m, 0.0), 1000 * m)):
        with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
            _along_direction(*_synthetic(d, r, s))


def test_the_boundary_counterexample_is_refused():
    """Re-check 2: d = R = (1.0, 0.1, 0.6), s = 0x1.2ba3ebdc9a779p+0. Exactly, |s - d.R/|d|| exceeds 64*eps*sum|R_i| by
    about 6.55e-17; the RE reader refuses it, and the built-in (compensated from Python 3.12) `sum` accepted it here."""
    from tests.verifiers.preview_physics_evidence import _along_direction
    d = (1.0, 0.1, 0.6)
    with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
        _along_direction(*_synthetic(d, d, float.fromhex("0x1.2ba3ebdc9a779p+0")))


def test_the_shared_parity_sweep_reads_with_its_committed_verdicts():
    """Re-check 2: the boundary sweep the RE reader also reads (`along_direction_evidence`
    `the_shared_parity_sweep_reads_with_its_committed_verdicts`): each committed verdict, here."""
    import struct
    from tests.verifiers.preview_physics_evidence import _along_direction
    sweep = json.loads((ROOT / "fixtures/results/skew_along_direction_guard_parity.json").read_text(encoding="utf-8"))
    value = lambda h: struct.unpack(">d", bytes.fromhex(h))[0]  # noqa: E731
    verdicts = []
    for n, case in enumerate(sweep["cases"]):
        d, r, s = [value(x) for x in case["d"]], [value(x) for x in case["r"]], value(case["s"])
        if case["accept"]:
            _along_direction(*_synthetic(d, r, s))
        else:
            with pytest.raises(ValueError, match="ALONG_DIRECTION_INCONSISTENT"):
                _along_direction(*_synthetic(d, r, s))
        verdicts.append(case["accept"])
    assert (len(verdicts), sum(verdicts)) == (385, 192)


def test_a_subnormal_alteration_inside_the_floor_reads_by_design():
    """Re-check 2 (by design): a consistency check within a finite allowance, not an exact-integrity check. With
    m = 2^-1074, R = (-m, -m, 0) and s changed from -m to 0 lies inside the absolute floor and reads."""
    import math
    from tests.verifiers.preview_physics_evidence import _along_direction
    m = math.ldexp(1.0, -1074)
    _along_direction(*_synthetic((1.0, 1.0, 0.0), (-m, -m, 0.0), 0.0))


def test_json_integers_are_decoded_to_binary64_before_the_guard_scales(monkeypatch):
    """Re-check 3: the RE reader's serde_json decode makes every JSON number an f64 before its guard scales. A Python
    int kept as an int steered `_largest` differently: with I = 2^512 - 1, x = 0x1.4cccccccccccdp-562 and m = 2^-1074,
    d = (I, x, 0) canonicalized to d_y = m here but 2m in Rust, and R = (I, x, 0) joint-scaled to R_y = 0 here but m
    in Rust. The guard now converts first, so both intermediates are Rust's."""
    import math
    import tests.verifiers.preview_physics_evidence as verifier
    big, x, m = 2 ** 512 - 1, float.fromhex("0x1.4cccccccccccdp-562"), math.ldexp(1.0, -1074)
    seen: dict[str, list[float]] = {}
    canonical, joint = verifier._canonical_direction, verifier._joint_scale

    def record_canonical(d):
        assert all(type(v) is float for v in d)
        seen["d"] = canonical(d)
        return seen["d"]

    def record_joint(r, s):
        assert all(type(v) is float for v in r) and type(s) is float
        seen["r"] = joint(r, s)[0]
        return joint(r, s)

    monkeypatch.setattr(verifier, "_canonical_direction", record_canonical)
    monkeypatch.setattr(verifier, "_joint_scale", record_joint)
    assert verifier._along_guard((big, x, 0), (1.0, 0.0, 0.0), 1.0)
    assert seen["d"][1] == 2 * m
    assert verifier._along_guard((1.0, 1.0, 0.0), (big, x, 0), float.fromhex("0x1.6a09e667f3bccp+511"))
    assert seen["r"][1] == m
