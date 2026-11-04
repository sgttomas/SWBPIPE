"""RT-U4 (order): the W-7 global end coverage loop's first refusal is the first member in (load case ID, member ID)
order, bytewise in Rust (``global_end_evidence::coverage`` walks its ``BTreeMap``) and by code point in the verifier
(``tests/verifiers/global_end.py`` walks ``sorted(members)``); UTF-8 byte order is code point order. A row whose load
case ID or member is not a string is no key in either reader.

The unit cases mirror ``global_end_evidence.rs``'s tests (same rows, same codes); the verifier's codes are also checked
under several string hash seeds. The reader cases solve invented models with the headless preview runner (RT-A4's
linear cantilever with its absolute-sum, SRSS and nested combinations, and a two-member variant) and compare the
runner's ``read preview-physics`` with the verifier.
"""
from __future__ import annotations

import copy
import json
import os
import subprocess
import sys
from pathlib import Path

import pytest

from tests.test_rt_a4_pf_reader_tampers import LINEAR, cli, linear_cantilever, mirror, solve  # noqa: F401 (fixture)
from tests.verifiers import global_end as ge

PROJECT = Path(__file__).resolve().parents[1]
CODE = "SOURCE_GLOBAL_END_"
SEEDS = range(8)


# Unit rows, as global_end_evidence.rs's tests build them.
def local(case, member, kind, end, value):
    return {"id": f"{case}:{member}:{kind}:{end}", "kind": kind, "value": value, "unit": "N", "entity_ref": member,
            "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "metadata": {"component": "c", "coordinate_system": "element_local", "location": end,
                         "basis": "recovered_from_local_element_stiffness", "sign_convention": "s"}}


def glob(case, member, end, name, value, basis=ge.BASIS):
    force = name.startswith("F")
    return {"id": f"{case}:{member}:{end}:{name}", "kind": ge.FORCE if force else ge.MOMENT, "value": value,
            "unit": "N" if force else "N*m", "entity_ref": member, "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "source_result_refs": [],
            "metadata": {"component": name, "coordinate_system": "global", "location": end, "basis": basis, "sign_convention": ge.SIGN}}


def member_in(case, member):
    rows = []
    for end in ge.ENDS:
        rows += [local(case, member, kind, end, value) for kind, value in zip(ge.LOCAL_KINDS, (3.0, 4.0, 12.0, 1.0, 2.0, 2.0))]
        rows += [glob(case, member, end, name, value) for name, value in
                 (("Fx", -4.0), ("Fy", 3.0), ("Fz", 12.0), ("Mx", -2.0), ("My", 1.0), ("Mz", 2.0))]
    return rows


def missing(case, member):
    return [r for r in member_in(case, member)
            if not (r["kind"] == ge.MOMENT and r["metadata"]["location"] == "end_j" and r["metadata"]["component"] == "My")]


def off_norm(case, member):
    rows = member_in(case, member)
    next(r for r in rows if r["kind"] == ge.FORCE and r["metadata"]["component"] == "Fz")["value"] = 12.5
    return rows


# (first, second) in (case, member) order: the competing and non-ASCII pairs of global_end_evidence.rs.
PAIRS = [(("c", "pipe:a"), ("c", "pipe:b")), (("c1", "pipe:z"), ("c2", "pipe:a")), (("c", "pipe:a"), ("c", "pipe:a b")),
         (("c", "pipe:z"), ("c", "pipe:é")), (("z", "pipe:a"), ("é", "pipe:a"))]


def unit_cases():
    """(name, rows, expected code): each pair, defect arrangement and row order; the first key's defect is refused."""
    for (c1, m1), (c2, m2) in PAIRS:
        for first, second, code in ((missing, off_norm, "COVERAGE"), (off_norm, missing, "ROTATION_NORM")):
            for order, rows in (("forward", first(c1, m1) + second(c2, m2)), ("reversed", second(c2, m2) + first(c1, m1))):
                yield f"{c1}/{m1} {first.__name__}; {c2}/{m2} {second.__name__}; {order}", rows, CODE + code


def unit_codes():
    """Each unit case's first refusal code from the verifier, in this process."""
    out = {}
    for name, rows, _ in unit_cases():
        with pytest.raises(ValueError) as refused:
            ge.coverage(rows)
        out[name] = str(refused.value)
    return out


def test_pairs_are_in_byte_and_code_point_order():
    for (c1, m1), (c2, m2) in PAIRS:
        assert (c1, m1) < (c2, m2)
        assert (c1.encode(), m1.encode()) < (c2.encode(), m2.encode())


def _in_seeds(expression):
    """The JSON value of a ``tests.test_rt_u4_global_end_order`` expression under each hash seed."""
    script = f"import json; from tests import test_rt_u4_global_end_order as t; print(json.dumps({expression}))"
    for seed in SEEDS:
        run = subprocess.run([sys.executable, "-c", script], cwd=PROJECT, capture_output=True, text=True, check=True,
                             env=dict(os.environ, PYTHONHASHSEED=str(seed)))
        yield seed, json.loads(run.stdout.strip().splitlines()[-1])


def test_the_first_unit_refusal_is_the_first_key_under_every_seed():
    expected = {name: code for name, _, code in unit_cases()}
    assert len(expected) == 4 * len(PAIRS) and unit_codes() == expected
    for seed, seen in _in_seeds("t.unit_codes()"):
        assert seen == expected, seed


def int_member():
    """A whole member under an integer load case ID."""
    whole = member_in("c", "pipe:c")
    for row in whole:
        row["basis_ref"]["ref_id"] = 7
    return whole


def strays():
    """global_end_evidence.rs `non_string_keys_are_skipped`'s rows: none is a key."""
    int_case = glob("c", "pipe:b", "end_i", "Fx", 0.0)
    int_case["basis_ref"]["ref_id"] = 7
    null_member = dict(glob("c", "pipe:a", "end_i", "Fx", 0.0), entity_ref=None)
    null_local = dict(local("c", "pipe:a", "element_local_shear_force_y", "end_i", 4.0), entity_ref=None)
    int_local = local("c", "pipe:a", "element_local_shear_force_y", "end_i", 4.0)
    int_local["basis_ref"]["ref_id"] = 7
    return [int_case, null_member, null_local, int_local] + int_member()


def test_non_string_keys_are_skipped_as_rust_skips_them():
    # Rust: coverage(member + strays) is Ok, and the member's own defect is the only refusal.
    # Alone beside a valid member, an integer-case member reaches the ordered walk: as a key it would not sort.
    assert ge.coverage(member_in("c", "pipe:a") + int_member()) is None
    assert ge.coverage(member_in("c", "pipe:a") + strays()) is None
    with pytest.raises(ValueError, match="^SOURCE_GLOBAL_END_ROTATION_NORM$"):
        ge.coverage(off_norm("c", "pipe:a") + strays())
    # Rust `case_of`: an integer load case ID is no case, so the row needs a combination basis.
    with pytest.raises(ValueError, match="^SOURCE_GLOBAL_END_BASIS$"):
        ge.row_semantics(strays()[0], True)


# The readers, on solved preview-physics-2 documents.
def _rows(source, kind, case, member):
    return [r for r in source["results"] if r["kind"] == kind and r.get("basis_ref") == {"ref_type": "load_case", "ref_id": case}
            and r["entity_ref"] == member]


def drop_row(source, case, member):
    """COVERAGE: the member's end_j My row in the case is removed."""
    (row,) = [r for r in _rows(source, ge.MOMENT, case, member) if r["metadata"]["location"] == "end_j" and r["metadata"]["component"] == "My"]
    source["results"].remove(row)


def turn_row(source, case, member):
    """ROTATION_NORM: the member's end_i Fz in the case no longer rotates its local force."""
    (row,) = [r for r in _rows(source, ge.FORCE, case, member) if r["metadata"]["location"] == "end_i" and r["metadata"]["component"] == "Fz"]
    row["value"] = row["value"] * 1.5 + 1000.0


def competing(source, first, second):
    """(name, tampered document, expected code): each arrangement of the two defects on the two keys."""
    out = []
    for a, b, code in ((drop_row, turn_row, "COVERAGE"), (turn_row, drop_row, "ROTATION_NORM")):
        tampered = copy.deepcopy(source)
        a(tampered, *first)
        b(tampered, *second)
        out.append((f"{first} {a.__name__}; {second} {b.__name__}", tampered, CODE + code))
    return out


def mirror_in_seeds(tmp_path, documents):
    """The verifier's code for each (source, model) document under each hash seed."""
    path = tmp_path / "documents.json"
    path.write_text(json.dumps(documents, ensure_ascii=False), encoding="utf-8")
    expression = f"[t.mirror(s, m) for s, m in json.load(open({str(path)!r}, encoding='utf-8'))]"
    return dict(_in_seeds(expression))


def check(tmp_path, model, source, first, second):
    for given in (None, model):
        assert (cli(source, given), mirror(source, given)) == (None, None)  # the accepting verdict is unchanged
    cases = competing(source, first, second)
    documents = []
    for name, tampered, code in cases:
        for given in (None, model):
            assert (cli(tampered, given), mirror(tampered, given)) == (code, code), (name, given is not None)
            documents.append([tampered, given])
    expected = [code for _name, _doc, code in cases for _ in (None, model)]
    for seed, seen in mirror_in_seeds(tmp_path, documents).items():
        assert seen == expected, seed


def test_an_rt_a4_combination_document_refuses_in_case_order(solve, tmp_path):
    """RT-A4's linear cantilever (SRSS, absolute sum of a subtraction, a nested sum): defects in W1 and W3."""
    model = linear_cantilever(LINEAR)
    source = solve(model)
    assert {r["basis_ref"]["ref_id"] for r in source["results"] if r["kind"] in ge.KINDS and r["basis_ref"]["ref_type"] == "combination"} \
        == {"srss-w", "d1", "abs-d", "sus+srss"}
    check(tmp_path, model, source, ("W1", "pipe:a-b"), ("W3", "pipe:a-b"))


def _renamed_cases(combinations):
    """LINEAR with load cases W1 -> "z" and W3 -> "é" ("z" < "é" by byte and code point)."""
    return json.loads(json.dumps(combinations).replace('"W1"', '"z"').replace('"W3"', '"\\u00e9"'))


def test_non_ascii_case_ids_refuse_in_byte_order(solve, tmp_path):
    model = linear_cantilever(_renamed_cases(LINEAR))
    model["project"]["id"] = "project:rt-u4-order-cases"
    for case in model["load_cases"]:
        case["id"] = {"W1": "z", "W3": "é"}.get(case["id"], case["id"])
    source = solve(model)
    check(tmp_path, model, source, ("z", "pipe:a-b"), ("é", "pipe:a-b"))


def test_non_ascii_member_ids_refuse_in_byte_order(solve, tmp_path):
    """Two members, "pipe:z" then "pipe:é", along the cantilever."""
    model = linear_cantilever(LINEAR)
    model["project"]["id"] = "project:rt-u4-order-members"
    provenance = model["nodes"][0]["provenance"]
    model["nodes"].insert(1, {"id": "node:m", "position": {"x": 0.5, "y": 0.0, "z": 0.0}, "provenance": provenance})
    pipe = model["pipe_segments"][0]
    model["pipe_segments"] = [dict(pipe, id="pipe:z", to="node:m"), dict(pipe, id="pipe:é", **{"from": "node:m"})]
    source = solve(model)
    check(tmp_path, model, source, ("W1", "pipe:z"), ("W1", "pipe:é"))


def test_an_integer_case_id_row_is_refused_alike(solve):
    """On preview-physics-2 a load-case `ref_id` is not type-checked before W-7; both readers read an integer one as no
    case's and refuse its case basis (before RT-U4 the verifier read it as case 7's and refused MEMBER_UNRESOLVED)."""
    model = linear_cantilever(LINEAR)
    source = solve(model)
    row = copy.deepcopy(next(r for r in source["results"] if r["kind"] == ge.FORCE and r["basis_ref"]["ref_type"] == "load_case"))
    row["id"], row["basis_ref"]["ref_id"] = "result:rt-u4:integer-case", 7
    source["results"].append(row)
    for given in (None, model):
        assert (cli(source, given), mirror(source, given)) == (CODE + "BASIS", CODE + "BASIS")


def _int_case_copy(row, basis=None):
    row = copy.deepcopy(row)
    row["id"], row["basis_ref"]["ref_id"] = row["id"] + ":rt-u4:int-case", 7
    if basis is not None:
        row["metadata"]["basis"] = basis
    return row


def test_an_integer_case_id_copy_of_a_whole_case_is_refused_alike(solve):
    """Review of #105: W1's element-local and global end rows copied under integer load case ID 7, keeping their case
    bases. Before RT-U4 the verifier read them as case 7's complete member set and accepted; both readers now read
    them as no case's and refuse the case basis (Python acceptance changes on this malformed input; Rust's does not)."""
    model = linear_cantilever(LINEAR)
    source = solve(model)
    copies = [_int_case_copy(r) for r in source["results"] if r["kind"] in (*ge.LOCAL_KINDS, *ge.KINDS)
              and r["basis_ref"] == {"ref_type": "load_case", "ref_id": "W1"}]
    assert any(r["kind"] in ge.KINDS for r in copies) and any(r["kind"] in ge.LOCAL_KINDS for r in copies)
    source["results"] += copies
    for given in (None, model):
        assert (cli(source, given), mirror(source, given)) == (CODE + "BASIS", CODE + "BASIS")


def test_an_integer_case_id_row_with_a_combination_basis_is_accepted_alike(solve):
    """Review of #105: an integer-ID load-case global row carrying a combination basis. Before RT-U4 the verifier read it
    as case 7's and refused BASIS; both readers now read it as no case's, admit the combination basis and skip it in
    coverage, so both accept (Python acceptance changes on this malformed input; Rust's does not)."""
    model = linear_cantilever(LINEAR)
    source = solve(model)
    row = next(r for r in source["results"] if r["kind"] == ge.FORCE and r["basis_ref"]["ref_type"] == "load_case")
    source["results"].append(_int_case_copy(row, "explicit_user_linear_combination"))
    for given in (None, model):
        assert (cli(source, given), mirror(source, given)) == (None, None)
