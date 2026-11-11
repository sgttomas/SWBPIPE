"""T4-U4b review repairs: tampers the model-holding
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


# §5.1's shear operand rule (RT-U4): a component-free shear slot is one operand when its rows agree bit for bit (or
# none carries a number) and is refused otherwise, in every row order. Rust: `u4b_evidence_any_tests.rs`.
def _shear(component, value, *, case="case:a", entity="pipe:p", kind="element_local_shear_force_y", location="midspan"):
    row = {"basis_ref": {"ref_type": "load_case", "ref_id": case}, "entity_ref": entity, "kind": kind,
           "metadata": {"component": component, "location": location, "basis": "recovered_from_local_element_stiffness"}}
    if value is not _ABSENT:
        row["value"] = value
    return row


_ABSENT = object()
_CONTEXT = [_shear("shear_force_z", 3.0, kind="element_local_shear_force_z"), _shear("shear_force_y", 7.0, entity="pipe:q"),
            _shear("", 9.0, case="case:b")]


def _orders(variants):
    import itertools
    for order in itertools.permutations(list(variants) + _CONTEXT[:2]):
        yield list(order) + _CONTEXT[2:]


@pytest.mark.parametrize("variants", [
    [_shear("shear_force_y", 2.0), _shear("", 1002.0)],
    [_shear("shear_force_y", 2.0), _shear("", None)],
    [_shear("shear_force_y", 2.0), _shear("other", "2.0")],
    [_shear("shear_force_y", 2.0), _shear("", _ABSENT)],
    [_shear("shear_force_y", 2.0), _shear("", float("inf"))],
    [_shear("shear_force_y", 2.0), _shear("", float("nan"))],
    [_shear("shear_force_y", 2.0), _shear("shear_force_y", 4.0)],
    [_shear("shear_force_y", 0.0), _shear("", -0.0)],
    [_shear("shear_force_y", 2.0), _shear("", 2.0), _shear("x", 5.0)],
], ids=["values", "null", "string", "absent", "inf", "nan", "same-component", "signed-zero", "third"])
def test_conflicting_shear_operands_are_refused_in_every_order(variants):
    codes = []
    for order in _orders(variants):
        with pytest.raises(ValueError, match="shear operand rows of one station disagree"):
            u4b.Rows(order)

        def fail(ok, code):
            if not ok:
                raise ValueError(f"SOURCE_PHYSICS_{code}")
        with pytest.raises(ValueError) as refused:
            u4b.Rows(order, fail)
        codes.append(str(refused.value))
        assert u4b.Rows(order, operands=False).operands == {}  # the row index alone reads no operand
    assert set(codes) == {"SOURCE_PHYSICS_SHEAR_OPERAND_AMBIGUOUS"} and len(codes) >= 24


@pytest.mark.parametrize("variants,want", [
    ([_shear("shear_force_y", 2.0), _shear("", 2.0)], 2.0),
    ([_shear("shear_force_y", -2.5), _shear("shear_force_y", -2.5), _shear("x", -2.5)], -2.5),
    ([_shear("shear_force_y", 2), _shear("", 2.0)], 2.0),
    ([_shear("shear_force_y", None), _shear("", "2.0"), _shear("x", _ABSENT)], None),
], ids=["components", "repeat", "integer", "no-number"])
def test_equivalent_shear_operands_are_one(variants, want):
    for order in _orders(variants):
        rows = u4b.Rows(order)
        got = rows.any("case:a", "pipe:p", "element_local_shear_force_y", "midspan")
        assert (None if got is None else u4b.bits(got)) == (None if want is None else u4b.bits(want))
        assert rows.any("case:a", "pipe:p", "element_local_shear_force_z", "midspan") == 3.0
        assert rows.any("case:b", "pipe:p", "element_local_shear_force_y", "midspan") == 9.0
        assert rows.any("case:a", "pipe:p", "element_local_shear_force_y", "end_i") is None


@pytest.mark.parametrize("entry", [EXACT, PREVIEW], ids=["pressure-2", "preview-physics-2"])
@pytest.mark.parametrize("first", [True, False], ids=["first", "last"])
def test_an_ambiguous_shear_slot_is_refused_by_the_readers(entry, first):
    doc, model, reader, _ = _load(entry)
    original = next(r for r in doc["results"] if r.get("kind") == "element_local_shear_force_y" and r["metadata"]["location"] == "midspan")

    def variant(delta):
        changed = copy.deepcopy(doc)
        extra = copy.deepcopy(original)
        extra["id"] += ":variant"
        del extra["metadata"]["component"]
        extra["value"] += delta
        changed["results"].insert(0 if first else len(changed["results"]), extra)
        return changed
    for with_model in (None, model):
        refused = rv.validate(reader, variant(1000.0), with_model)
        assert refused["ok"] is False and "shear operand rows of one station disagree" in _code(refused), _code(refused)
        same = rv.validate(reader, variant(0.0), with_model)
        assert same["ok"] is True, _code(same)


# Review F1 (#104): the verifier's slot is Rust `Index::new`'s, field by field: a non-string location or component
# falls back to "", a basis is read only from a string, and a row with a non-string case, entity or kind is skipped.
# Two component-free shear rows whose fields Rust maps to one slot, with different values, are refused by both, and
# with equal values read as the source does; the verifier and the Rust runner agree on each envelope, in both
# insertion orders. Rust: `u4b_evidence_any_tests.rs`.
_MISSING = object()
_SAME_SLOT = [  # (location, basis) of the two rows
    ((None, _MISSING), ("", _MISSING)),
    ((5, _MISSING), (_MISSING, _MISSING)),
    (({"at": "midspan"}, _MISSING), ([], _MISSING)),
    (("", 7), ("", _MISSING)),
    (("", {"b": 1}), ("", "recovered_from_local_element_stiffness")),
    (("", None), ("", "token; section_basis=")),
]
_OTHER_SLOT = [  # Rust skips the second row, or reads it at another slot
    (("", _MISSING), ("", "token; section_basis=nominal_section")),
]


def _malformed(doc, fields, values, first, skip=None):
    changed = copy.deepcopy(doc)
    original = next(r for r in changed["results"] if r.get("kind") == "element_local_shear_force_y")
    rows = []
    for i, ((location, basis), value) in enumerate(zip(fields, values)):
        row = copy.deepcopy(original)
        row["id"] = f"{original['id']}:malformed-{i}"
        row["value"] = value
        metadata = row["metadata"]
        del metadata["component"]
        for key, field in (("location", location), ("basis", basis)):
            if field is _MISSING:
                metadata.pop(key, None)
            else:
                metadata[key] = field
        rows.append(row)
    if skip is not None:
        skip(rows[1])
    for row in rows:
        changed["results"].insert(0 if first else len(changed["results"]), row)
    return changed


def _verdicts(source, model=None):
    """The verifier's verdict, and the Rust runner's: its preview-physics reader without the model, and with it the
    dispatch standing (the runner's model-bound read; the clean fixture is eligible)."""
    from core.analysis_runs import rust_reader
    python = rv.validate("preview-physics", source, model)
    if model is not None:
        standing = rust_reader.dispatch(source, model)["standing"]
        return python, (standing["eligible"], (standing["findings"] or [None])[0])
    try:
        rust_reader.validate("preview-physics", source)
        return python, (True, None)
    except rust_reader.ReaderRefusal as refusal:
        return python, (False, refusal.code)


@pytest.mark.parametrize("first", [True, False], ids=["first", "last"])
@pytest.mark.parametrize("fields", _SAME_SLOT + _OTHER_SLOT, ids=[f"slot{i}" for i in range(len(_SAME_SLOT))] + ["other-basis"])
def test_malformed_slot_fields_agree_with_the_rust_runner(fields, first):
    doc, model, _, _ = _load(PREVIEW)
    same = fields in _SAME_SLOT
    for values, refused in (((2.0, 3.0), same), ((2.0, 2.0), False)):
        source = _malformed(doc, fields, values, first)
        for with_model in (None, model):
            python, (rust_ok, rust_code) = _verdicts(source, with_model)
            assert python["ok"] is rust_ok, (_code(python), rust_code)
            if refused:
                assert rust_code == "SOURCE_PREVIEW_PHYSICS_SHEAR_OPERAND_AMBIGUOUS"
                assert "shear operand rows of one station disagree" in _code(python), _code(python)
            else:
                # A control is accepted by both, not merely refused alike.
                assert rust_ok and python["ok"], (_code(python), rust_code)


@pytest.mark.parametrize("skip", [
    lambda row: row.__setitem__("entity_ref", 7),
    lambda row: row.__setitem__("kind", None),
    lambda row: row["basis_ref"].__setitem__("ref_id", 3),
    lambda row: row["basis_ref"].__setitem__("ref_type", "load_combination"),
], ids=["entity", "kind", "case", "scope"])
def test_a_row_rust_cannot_slot_is_skipped_by_both(skip):
    doc, model, _, _ = _load(PREVIEW)
    source = _malformed(doc, (("", _MISSING), ("", _MISSING)), (2.0, 3.0), False, skip)
    for with_model in (None, model):
        python, (rust_ok, rust_code) = _verdicts(source, with_model)
        assert python["ok"] is rust_ok, (_code(python), rust_code)
        assert rust_code != "SOURCE_PREVIEW_PHYSICS_SHEAR_OPERAND_AMBIGUOUS"


def test_malformed_slot_fields_index_as_rust_indexes_them():
    """The verifier's index on Rust's `malformed_slot_fields_read_as_rust_reads_them` pairs, plus a non-object metadata,
    in both orders: one slot each, refused with different values and one operand with equal ones."""
    def shear(location, basis, value, metadata=_MISSING):
        row = _shear("", value, location="")
        del row["metadata"]["component"]
        for key, field in (("location", location), ("basis", basis)):
            if field is _MISSING:
                row["metadata"].pop(key, None)
            else:
                row["metadata"][key] = field
        if metadata is not _MISSING:
            row["metadata"] = metadata
        return row
    pairs = [[shear(*a, 2.0), shear(*b, 3.0)] for a, b in _SAME_SLOT]
    pairs.append([shear("", _MISSING, 2.0, ["location", ""]), shear("", _MISSING, 3.0)])
    for pair in pairs:
        for order in (pair, pair[::-1]):
            with pytest.raises(ValueError, match="shear operand rows of one station disagree"):
                u4b.Rows(copy.deepcopy(order))
            equal = copy.deepcopy(order)
            equal[1]["value"] = equal[0]["value"]
            rows = u4b.Rows(equal)
            assert len(rows.operands) == 1 and rows.any("case:a", "pipe:p", "element_local_shear_force_y", "") == equal[0]["value"]
