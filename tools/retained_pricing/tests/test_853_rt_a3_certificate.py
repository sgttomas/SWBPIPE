"""RT-A3 P and the §4 combination certificate (retained-pricing-rules.md §8.53).

- The line map's reviewed text resolution re-keys a site line rewritten in place only when the OLD text is gone, the
  named NEW text occurs exactly once and the named function encloses it; any other case stays unmapped (the run stops).
- RTA3C: the certificate's rules are carried on both routes, its D1 premises are guarded, and a changed module stops.
- RTA3C-OWN (rta3c_owners_model): the current source activates the owners, a snapshot without the producer adds nothing,
  a moved owning statement stops, and at z = 0 only the case-id vector is priced.

These are source warrants, not runtime allocation observations.
"""
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "lib"))
sys.path.insert(0, str(PRICING / "base"))
import linemap_lib  # noqa: E402

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
LIB = PP + "lib.rs"
C3_PP = PP + "combination_certificate.rs"
C3_RE = RE + "combination_certificate.rs"
P = "projects/chirality-piping/"
F = "core/x/src/lib.rs"


# ---- the line map's text resolution -------------------------------------------------------------------------------

def _repo(tmp_path, monkeypatch, old_text, new_text):
    repo = tmp_path / "repo"
    repo.mkdir()
    env = dict(GIT_AUTHOR_NAME="test", GIT_AUTHOR_EMAIL="test@example.invalid", GIT_COMMITTER_NAME="test",
               GIT_COMMITTER_EMAIL="test@example.invalid", GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1")
    for k, v in env.items():
        monkeypatch.setenv(k, v)

    def git(*args):
        return subprocess.check_output(["git", "-C", str(repo), *args], text=True).strip()
    git("init", "-q")
    source = repo / P / F
    source.parent.mkdir(parents=True)

    def commit(text):
        source.write_text(text)
        git("add", ".")
        git("commit", "-qm", "fixture")
        return git("rev-parse", "HEAD")
    return repo, commit(old_text), commit(new_text)


OLD = "fn outer() {\n    let x = read(a);\n    done(x);\n}\n"
NEW = "// one\nfn outer() {\n    let x = wrap(|| read(a));\n    done(x);\n}\n"
TEXT = "let x = wrap(|| read(a));"


def _map(tmp_path, monkeypatch, resolution, old=OLD, new=NEW):
    repo, a, b = _repo(tmp_path, monkeypatch, old, new)
    m = linemap_lib.Mapper(str(repo), a, b, ["core/x/src"], resolutions={F + ":2": resolution} if resolution else None)
    return m.map_key(F + ":2"), m


def test_a_reviewed_text_resolution_rekeys_the_rewritten_line(tmp_path, monkeypatch):
    out, m = _map(tmp_path, monkeypatch, {"as": "text", "text": TEXT, "fn": "outer", "why": "fixture"})
    assert out == F + ":3" and not m.unmapped and len(m.resolutions_used) == 1


@pytest.mark.parametrize("resolution, new", [
    (None, NEW),                                                                     # no review: unmapped
    ({"as": "text", "text": TEXT, "fn": "other", "why": "fixture"}, NEW),            # another enclosing fn
    ({"as": "text", "text": "let x = absent();", "fn": "outer", "why": "fixture"}, NEW),   # the text is not at NEW
    ({"as": "text", "text": TEXT, "fn": "outer", "why": "fixture"},                  # the text twice at NEW
     "fn outer() {\n    let x = wrap(|| read(a));\n    let x = wrap(|| read(a));\n}\n"),
    ({"as": "text", "text": TEXT, "fn": "outer", "why": "fixture"},                  # the OLD text still at NEW (twice;
     "fn outer() {\n    let x = wrap(|| read(a));\n    done(x);\n}\nfn later() {\n    let x = read(a);\n"   # once it maps
     "    let x = read(a);\n}\n"),                                                   # there by text)
])
def test_any_other_case_stays_unmapped(tmp_path, monkeypatch, resolution, new):
    out, m = _map(tmp_path, monkeypatch, resolution, new=new)
    assert out == F + ":2" and m.unmapped and not m.resolutions_used


@pytest.mark.parametrize("entry", [
    {"as": "text", "text": TEXT, "why": "no fn"},
    {"as": "text", "text": TEXT, "fn": "outer"},
    {"as": "text", "text": " " + TEXT, "fn": "outer", "why": "not stripped"},
    {"as": "text", "text": "", "fn": "outer", "why": "empty"},
])
def test_an_incomplete_text_resolution_is_refused(tmp_path, entry):
    path = tmp_path / "resolutions.json"
    path.write_text(json.dumps({"keys": {F + ":2": entry}}))
    with pytest.raises(SystemExit):
        linemap_lib.load_resolutions(str(path))


def test_the_reviewed_g7_resolution_names_the_wrapped_read_once():
    keys = json.loads((PRICING / "rules/linemap_resolutions.json").read_text())["keys"]
    entry = keys[RE + "retained_precision.rs:5860"]
    assert entry["as"] == "text" and entry["fn"] == "gates_after_g0"
    production = (SOURCE / RE / "retained_precision.rs").read_text(encoding="utf-8")
    assert [line.strip() for line in production.split("\n")].count(entry["text"]) == 1


# ---- RTA3C rules ---------------------------------------------------------------------------------------------------

def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def _short(key):
    return str(key).split("/src/")[-1]


def _per_call(d):
    return {(_short(e[0]), _short(e[1])): e[2] for e in d["EDGE_PER_CALL"] if "RTA3C" in str(e[3])}


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_certificate_rules_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    calls = {(a.split(":")[-1], b.split(":")[-1]): n for (a, b), n in _per_call(d).items()}
    # The producer runs only with a combination; the reader's member work only with a member (both at most z).
    assert calls[("run_linear_static_preview_observed_inner", "publish")] == "min(1,z)"
    assert calls[("solve_prepared_load_case", "operand")] == "min(1,z)"
    assert calls[("member_check", "new")] == "min(1,z)"
    assert calls[("member_check", "bit_equal")] == "min(cases,3*z)*28*(m+g)+3*z"
    # The roster is unreached on the retained graph (RTA3C-RB).
    zero = [(_short(a), _short(b)) for a, b, *_ in d["O10_EDGE_ZERO"]]
    assert any(a.endswith(":check_with") and b.endswith(":roster") for a, b in zero)


@pytest.mark.parametrize("old, new", [
    # The producer's early return goes: publish would allocate without a combination.
    ("    if combinations.is_empty() || envelope.status.mechanics != \"MECHANICS_SOLVED\" {\n        return;\n    }\n", ""),
])
def test_a_changed_certificate_module_stops(tmp_path, monkeypatch, old, new):
    snap = copy_core(tmp_path)
    edit(snap, C3_PP, old, new)
    with pytest.raises(SystemExit, match="RTA3C-PIN"):
        decisions(monkeypatch, snap)


def test_a_second_publish_call_stops(tmp_path, monkeypatch):
    """lib.rs is pinned whole (KD5C4's B7a E14 pin runs first); RTA3C's statement counts are the second line."""
    snap = copy_core(tmp_path)
    call = "combination_certificate::publish(&model.combinations, &case_ids, &mut envelope, &certificate_operands);"
    edit(snap, LIB, call, call + "\n    " + call)
    with pytest.raises((SystemExit, ValueError), match="KD5C4|RTA3C"):
        decisions(monkeypatch, snap)


# ---- RTA3C-OWN -----------------------------------------------------------------------------------------------------

MODEL = runpy.run_path(str(PRICING / "base/rta3c_owners_model.py"))
OWN_FILES = sorted(set(MODEL["PINS"]) | {rel for rel, _ in MODEL["STATEMENTS"]})


def owners_snapshot(tmp_path, file=None, old=None, new=None, drop=()):
    for relative in OWN_FILES:
        if relative in drop:
            continue
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    if file:
        edit(tmp_path, file, old, new)
    return tmp_path


def test_the_current_source_activates_the_owners():
    assert MODEL["inspect"](SOURCE) is True


def test_a_snapshot_without_the_producer_adds_nothing(tmp_path):
    assert MODEL["inspect"](owners_snapshot(tmp_path, drop=(C3_PP,))) is False


def test_a_moved_owning_statement_stops(tmp_path):
    statement = "let case_ids: Vec<&str> = model.load_cases.iter().map(|case| case.id.as_str()).collect();"
    snap = owners_snapshot(tmp_path, LIB, statement, statement.replace("let case_ids", "let mut case_ids"))
    with pytest.raises(SystemExit, match="RT-A3 §4 owners"):
        MODEL["inspect"](snap)


def test_a_changed_module_stops(tmp_path):
    snap = owners_snapshot(tmp_path, C3_RE, "let rows = Rows::new(results);", "let rows = Rows::new(results); ")
    with pytest.raises(SystemExit, match="re-review"):
        MODEL["inspect"](snap)


def test_at_z_zero_only_the_case_ids_are_priced():
    tree = lambda *a: MODEL["B"](sum(a))   # noqa: E731 (any Value-tree form)
    for c in (1, 3, 7):
        assert MODEL["operands"](c, 0, 5, 2) == MODEL["X"]()
        assert MODEL["publish"](c, 0, 5, 2, 100, 100, tree) == MODEL["X"]()
        assert MODEL["reader"](c, 0, 5, 2, 100, 100, tree) == MODEL["X"]()
        assert MODEL["member_value"](c, 0, 5, 2, 64) == (0, 0, 0, 0, 0, 0)
        assert MODEL["case_ids"](c) == MODEL["s"]("&str") * c


# ---- RTA3C-OWN's Value-tree census (§8.53 S1) ----------------------------------------------------------------------
#
# The member's counts follow the producer's json! literals (RTA3C-PIN binds the module): at the maximal shape every
# array-slot, object and number count equals the census (arr carries a fixed slack of 2 and ent one of 2 per record,
# for the expression's alternatives) and key bytes are bounded; every registered successor and corpus document with a
# member is bounded component by component.

def _walk(value, row_ids=()):
    """Array slots, objects, entries, string bytes (less the row ids'), key bytes and numbers of a JSON tree."""
    c = dict(arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, rid=0)

    def w(x, rows=False):
        if isinstance(x, list):
            c["arr"] += len(x)
            for y in x:
                w(y, rows)
        elif isinstance(x, dict):
            c["obj"] += 1
            c["ent"] += len(x)
            for k, y in x.items():
                c["keyb"] += len(k.encode())
                w(y, rows or k == "rows")
        elif isinstance(x, str):
            c["rid" if rows else "strb"] += len(x.encode())
        elif isinstance(x, (int, float)) and not isinstance(x, bool):
            c["nums"] += 1
    w(value)
    return c


def _maximal_member(used, z, G):
    """The producer's member at its largest shape (combination_certificate.rs publish's json! literals)."""
    rid = lambda i: "r%d" % i   # noqa: E731
    group = {"entity": "e", "location": "l", "group": "g", "moment": False, "rows": [[rid(0)] * 3, [rid(1)] * 3],
             "q": 0.0, "bound": 0.0, "floor_scale": 0.0}
    derived = {"entity": "e", "location": "wall_l", "group": "pipe_wall", "row_set": 0, "rows": [rid(2)] * 3,
               "factors_bound": 0.0, "rounding_error": 0.0, "bound": 0.0, "q": 0.0, "q_hat": 0.0, "floor_scale": 0.0,
               "verdict": "not_established", "reason": "bound_exceeds_criterion",
               "equal_pressure": {"elastic_group": {"entity": "e", "location": "l", "group": "axial", "row_set": 1}}}
    record = {"combination_id": "c", "basis": "mechanics",
              "expression": {"terms": [{"combination_id": "x", "factor": 1.0}] * 3}, "subtrahend_support_state": None,
              "assessment": "assessed", "reason": None, "groups": [derived] * (2 * G), "uncovered_row_count": 0}
    return {"operand_cases": [{"load_case_id": "k", "groups": [group] * G}] * used, "combinations": [record] * z}


@pytest.mark.parametrize("used,z,G", [(1, 1, 1), (2, 1, 5), (3, 2, 7), (6, 2, 3)])
def test_the_member_census_is_the_producers_maximal_topology(used, z, G):
    actual = _walk(_maximal_member(used, z, G))
    arr, obj, ent, strb, keyb, nums = MODEL["member_counts"](used, z, G, 64)
    assert (arr, obj, ent) == (actual["arr"] + 2, actual["obj"], actual["ent"] + 2 * z)
    assert nums >= actual["nums"] and keyb >= actual["keyb"]
    # The row ids are the chain's TEXT (RTA3C-RS) in the resident member; their slots are the census's.
    assert MODEL["_row_id_bytes"](used, z, G) == 1024 * (z * 2 * G * 3 + used * G * 6)


def test_one_derived_group_is_its_literal():
    d = MODEL["_derived_value"](1, 64)
    one = _walk({"groups": [_maximal_member(0, 1, 1)["combinations"][0]["groups"][0]]})
    assert (d["arr"], d["obj"], d["ent"], d["nums"]) == (2 * one["arr"], 2 * (one["obj"] - 1), 2 * (one["ent"] - 1), 2 * one["nums"])
    assert d["keyb"] == 2 * (one["keyb"] - len("groups"))


def _members():
    # The registered successors: the lossless corpus storage, expanded by its own reader.
    import importlib.util
    spec = importlib.util.spec_from_file_location("rp_corpus_storage", SOURCE / "tools/fixtures/retained_precision_corpus.py")
    storage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(storage)
    cases = storage.expand(json.loads((SOURCE / "fixtures/results/retained_precision_cases.json").read_bytes()))["cases"]
    for case in cases:
        member = case["source"].get("contract_evidence", {}).get("combination_certificate")
        if member:
            yield case["id"], member
    for i, doc in enumerate(json.loads((SOURCE / "fixtures/results/combination_certificate_corpus.json").read_text())["documents"]):
        member = doc["document"].get("contract_evidence", {}).get("combination_certificate")
        if member:
            yield f"corpus[{i}]", member


def test_the_member_census_bounds_every_registered_member():
    seen = {}
    for name, member in _members():
        used, z = len(member["operand_cases"]), len(member["combinations"])
        G = max([len(o["groups"]) for o in member["operand_cases"]] + [0])
        derived = [g for r in member["combinations"] for g in r["groups"]]
        assert all(len(r["groups"]) <= 2 * G for r in member["combinations"]), name
        assert all(len(g["rows"]) <= 3 for g in derived), name
        assert all(len(s_) <= 2 and all(r is None or len(r) <= 3 for r in s_) for o in member["operand_cases"] for s_ in
                   (g["rows"] for g in o["groups"])), name
        actual = _walk(member)
        arr, obj, ent, strb, keyb, nums = MODEL["member_counts"](used, z, G, 64)
        assert all(a <= b for a, b in zip((actual["arr"], actual["obj"], actual["ent"], actual["keyb"], actual["nums"], actual["strb"]),
                                         (arr, obj, ent, keyb, nums, strb))), (name, actual)
        assert actual["rid"] <= MODEL["_row_id_bytes"](used, z, G), name
        seen[name] = (len(derived), sum(len(g["rows"]) for g in derived))
    # W-CB5 (the assessed registered successors): 40 derived groups, 70 row-id slots.
    assert seen["w_cb5_sparse_interactive"] == seen["w_cb5_dense_scrutiny"] == (40, 70)
    assert len(seen) >= 20


def _slots(a, o, e, sb, kb):
    return MODEL["X"]({"arr": a, "obj": o, "ent": e, "1": sb + kb})


@pytest.mark.parametrize("c,z", [(2, 1), (1, 2), (3, 2)])
def test_the_value_copies_are_charged(c, z):
    m, g, p, pa = 32, 32, 2115, 3 * 2115
    G, used, rows = 28 * (m + g), min(c, 3 * z), p + 8 * g + 1
    d = MODEL["_derived_value"](G, 64)
    member = MODEL["member_counts"](used, z, G, 64)
    publish = MODEL["publish"](c, z, m, g, p, pa, _slots)
    # quality twice, the derived groups twice, the member copy, one case's groups and one group's rows.
    assert publish["arr"] == 2 * 2 * (c + z) + 2 * d["arr"] + member[0] + G * 9 + 2 * 4
    assert publish["obj"] == 2 * 4 * (c + z) + 2 * d["obj"] + member[1] + G
    reader = MODEL["reader"](c, z, m, g, p, pa, lambda *a: _slots(*a))
    d128 = MODEL["_derived_value"](G, 128)
    # one group's expected rows and its copy, the derived trees three times, the expected diagnostics.
    assert reader["arr"] == 2 * 8 + 3 * d128["arr"] + z
    for x in (publish, reader):
        assert x["s(usize)"] == 2 * G * 3 + __import__("g3lib").buckets(min(2 * G * 3, rows)), x
        assert x["s((String,f64))"] == 3


# ---- the point atoms (§8.53 B1) -------------------------------------------------------------------------------------

LAW = SOURCE / PP / "retained_memory_law_tests.rs"


def _tree(atoms, assumed=None):
    return {"forms": {"F": {a: 1 for a in atoms} | {"1": 3}}, "assumed": assumed or {a: 8 for a in atoms}, "text_atoms": {}}


def _point_block(tmp_path, merged, points):
    law = tmp_path / "law.rs"
    law.write_text("before\n// ---- BEGIN GENERATED POINT ATOMS\n// ---- END GENERATED POINT ATOMS ----\nafter\n")
    paths = []
    for i, t in enumerate([merged] + points):
        paths.append(tmp_path / f"t{i}.json")
        paths[-1].write_text(json.dumps(t))
    r = subprocess.run([sys.executable, str(PRICING / "base/g5_profile.py"), "--point-atoms", str(paths[0]), str(law),
                        *map(str, paths[1:])], capture_output=True, text=True)
    return r, law.read_text()


def test_the_point_atoms_are_the_points_atoms_beyond_the_merged_trees(tmp_path):
    r, text = _point_block(tmp_path, _tree(["s(String)", "Text(x)"]),
                           [_tree(["s(String)"]), _tree(["s(String)", "s(C3Derived)", "Text(y)"]), _tree(["s(Target)"])])
    assert r.returncode == 0, r.stderr
    assert text.startswith("before\n// ---- BEGIN GENERATED POINT ATOMS (") and text.endswith("// ---- END GENERATED POINT ATOMS ----\nafter\n")
    assert '        "s(C3Derived)",\n        "s(Target)",\n    ];' in text and '"s(String)"' not in text and "Text(" not in text
    assert "(size_of::<crate::combination_plan::Target<'static>>()) as u64, // s(Target)" in text
    assert "pub(crate) const ATOMS: usize = 2;" in text


def test_an_unbound_point_atom_stops(tmp_path):
    r, text = _point_block(tmp_path, _tree([]), [_tree(["s(NoSuchAtom)"])])
    assert r.returncode != 0 and "unbound point atoms: s(NoSuchAtom)" in r.stderr
    assert "const ATOMS" not in text


def test_the_committed_point_atoms_are_the_generators():
    """The committed block regenerates from its own atom names and strides by the generator's current bindings (the
    run-tree check is `price.py point-atoms <run> <rs> --check`)."""
    text = LAW.read_text()
    block = text[text.index("// ---- BEGIN GENERATED POINT ATOMS"):text.index("// ---- END GENERATED POINT ATOMS ----")]
    names = json.loads("[" + block.split("ATOM_NAMES: [&str; ATOMS] = [", 1)[1].split("];", 1)[0].strip().rstrip(",") + "]")
    assumed = [int(x) for x in block.split("ATOM_ASSUMED: [u64; ATOMS] = [", 1)[1].split("];", 1)[0].replace(",", " ").split()]
    assert names == sorted(names) and len(names) == len(assumed) >= 30
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        r, regenerated = _point_block(Path(td), _tree([]), [_tree(names, dict(zip(names, assumed)))])
    assert r.returncode == 0, r.stderr
    new = regenerated[regenerated.index("// ---- BEGIN GENERATED POINT ATOMS"):regenerated.index("// ---- END GENERATED POINT ATOMS ----")]
    assert new == block
    # The law test prints them beside the profile's own, under the `profile_` filter.
    assert "fn profile_point_atoms_in_build() {" in text and 'println!("I65_G5_ATOM {:?}\\t{}\\t{}\\t{}", ATOM_BINDINGS[i], ATOM_NAMES[i]' in text
