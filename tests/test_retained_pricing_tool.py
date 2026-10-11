"""The retained pricing tool (tools/retained_pricing): the key carry (offset, by function, by text), the prune and its
stop on an ambiguous deletion, the decision-key stop and KEYMAP_ALLOW, the caller zero-edge count and the basis line.

A two-commit repository in a temporary directory stands in for the product; no product build or chain run is needed.
"""
import json
import os
from pathlib import Path
import subprocess
import sys

import pytest

TOOL = Path(__file__).resolve().parents[1] / "tools" / "retained_pricing"
P = "projects/chirality-piping/"
F = P + "core/x/src/lib.rs"

V1 = """fn a() {
    let s = format!("one");
}
fn b() {}
fn gone() {}
"""
# Two lines above (offset), one inside a() (a hunk below its definition), fn gone deleted.
V2 = """// one
// two
fn a() {
    let t = 1;
    let s = format!("one");
}
fn b() {}
"""
RULES = {
    "edge_zero": [{"caller": "lib.rs:1:a", "callee": "lib.rs:5:gone", "why": "lib.rs:5 is cited, not mapped"},
                  {"caller": "lib.rs:1:a", "callee": "lib.rs:4:b"}],
    "fn_cap": {"lib.rs:4:b": 1, "lib.rs:5:gone": 2},
    "site_total": {"lib.rs:2": 3},
}


def git(repo, *a):
    env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@example.invalid", GIT_COMMITTER_NAME="t",
               GIT_COMMITTER_EMAIL="t@example.invalid", GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-C", str(repo), *a], env=env, check=True, capture_output=True, text=True).stdout.strip()


def commit(repo, text):
    (repo / F).parent.mkdir(parents=True, exist_ok=True)
    (repo / F).write_text(text)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "v")
    return git(repo, "rev-parse", "HEAD")


@pytest.fixture()
def repo(tmp_path):
    r = tmp_path / "repo"
    r.mkdir()
    git(r, "init", "-q")
    return r


def carry(tmp_path, repo, old, new, rules, resolutions=None):
    (tmp_path / "crates.txt").write_text("core/x/src\n")
    (tmp_path / "rules.json").write_text(json.dumps(rules))
    out = tmp_path / "out"
    r = subprocess.run([sys.executable, str(TOOL / "lib" / "linemap_rules.py"), "--prune-deleted", str(repo), old, new, str(out),
                        str(tmp_path / "rules.json")],
                       env=dict(os.environ, LINEMAP_CRATES=str(tmp_path / "crates.txt"), LINEMAP_RESOLUTIONS=resolutions or ""),
                       capture_output=True, text=True)
    report = json.loads((out / "g7_linemap.out.json").read_text())
    return r.returncode, json.loads((out / "rules.json").read_text()), report


def test_keys_carry_by_offset_and_deleted_function_rules_are_pruned(tmp_path, repo):
    old, new = commit(repo, V1), commit(repo, V2)
    rc, rules, report = carry(tmp_path, repo, old, new, RULES)
    assert rc == 0, report["summary"]["unmapped"]
    # the deleted callee's list-form rule and dict-keyed rule are dropped; descriptive text is left as written
    assert rules["edge_zero"] == [{"caller": "lib.rs:3:a", "callee": "lib.rs:7:b"}]
    assert rules["fn_cap"] == {"lib.rs:7:b": 1}
    assert rules["site_total"] == {"lib.rs:5": 3}
    pruned = sorted(p["ctx"] for p in report["summary"]["pruned_deleted"])
    assert pruned == ["rules.json/edge_zero[0]", "rules.json/fn_cap/lib.rs:5:gone/key"]
    assert report["summary"]["unmapped"] == []


def test_a_function_whose_line_a_hunk_replaced_carries_by_its_definition(tmp_path, repo):
    old = commit(repo, V1)
    new = commit(repo, "fn b() {}\nfn a() {\n    let s = format!(\"one\");\n}\n")   # a and b swapped, gone deleted
    rc, rules, report = carry(tmp_path, repo, old, new, {"fn_cap": {"lib.rs:1:a": 1, "lib.rs:4:b": 2}})
    assert rc == 0
    assert rules["fn_cap"] == {"lib.rs:2:a": 1, "lib.rs:1:b": 2}
    # a() only shifts (offset, checked); b()'s old line lies in a changed hunk, so it carries by its definition
    assert {r[1]: r[3].split(":")[0] for r in report["remapped"]} == {"lib.rs:1:a": "offset", "lib.rs:4:b": "by function"}


def test_a_site_whose_text_changed_stops_the_carry(tmp_path, repo):
    old = commit(repo, V1)
    new = commit(repo, V1.replace('format!("one")', 'format!("two")'))
    rc, rules, report = carry(tmp_path, repo, old, new, {"site_total": {"lib.rs:2": 3}})
    assert rc == 1
    assert [u[1] for u in report["summary"]["unmapped"]] == ["lib.rs:2"]
    assert report["summary"]["unmapped"][0][3] == "replaced"


# The review's case: two `fn f`; B::f deleted and A::f's signature edited in the same change. The count falls from 2 to
# 1, which does not say which definition went, so nothing is pruned and the carry stops until the keys are resolved.
TWO_F = "impl A {\n    fn f(&self) {}\n}\nimpl B {\n    fn f(&self) {}\n}\n"
ONE_F = "impl A {\n    fn f(&self, extra: u8) {}\n}\n"
TWO_F_RULES = {"fn_cap": {"lib.rs:2:f": 1}, "edge_zero": [{"caller": "lib.rs:2:f", "callee": "lib.rs:5:f"}]}


def test_a_function_that_lost_one_of_two_definitions_stops_the_carry(tmp_path, repo):
    old, new = commit(repo, TWO_F), commit(repo, ONE_F)
    rc, rules, report = carry(tmp_path, repo, old, new, TWO_F_RULES)
    assert rc == 1
    assert rules == TWO_F_RULES                      # nothing pruned
    assert report["summary"]["pruned_deleted"] == []
    assert sorted((u[1], u[3]) for u in report["summary"]["unmapped"]) == [
        ("lib.rs:2:f", "fewer_definitions"), ("lib.rs:2:f", "fewer_definitions"), ("lib.rs:5:f", "fewer_definitions")]


def test_reviewed_resolutions_carry_the_survivor_and_prune_the_deleted_definition(tmp_path, repo):
    old, new = commit(repo, TWO_F), commit(repo, ONE_F)
    res = tmp_path / "resolutions.json"
    res.write_text(json.dumps({"keys": {"lib.rs:2:f": {"as": 1, "why": "A::f kept, its signature edited"},
                                        "lib.rs:5:f": {"as": "deleted", "why": "B::f removed"}}}))
    rc, rules, report = carry(tmp_path, repo, old, new, TWO_F_RULES, resolutions=str(res))
    assert rc == 0, report["summary"]["unmapped"]
    assert rules == {"fn_cap": {"lib.rs:2:f": 1}, "edge_zero": []}
    assert [p["ctx"] for p in report["summary"]["pruned_deleted"]] == ["rules.json/edge_zero[0]"]


def lib():
    sys.path.insert(0, str(TOOL / "lib"))
    import linemap_lib, rule_ops
    return linemap_lib, rule_ops


class Resolved(str):
    pass


def test_decision_keys_stop_unless_allowed_as_deleted(tmp_path, repo):
    linemap_lib, rule_ops = lib()
    old, new = commit(repo, V1), commit(repo, V1.replace('format!("one")', 'format!("two")').replace("fn gone() {}\n", ""))

    def run(allow):
        M = linemap_lib.Mapper(str(repo), old, new, ["core/x/src"])
        D = {"FN_ZERO": [("lib.rs:5:gone", "why"), (Resolved("lib.rs:9:named"), "why")], "SITE_TOTAL": [("lib.rs:4:b", 1)],
             "SITE_ZERO": [("lib.rs:2", "why")], "KEYMAP_ALLOW": allow}
        return D, rule_ops.carry_decision_keys(D, M)

    D, bad = run({})
    assert sorted((u[1], u[-1]) for u in bad) == [("lib.rs:2", "replaced"), ("lib.rs:5:gone", "deleted")]
    assert D["FN_ZERO"][1][0] == "lib.rs:9:named"            # resolved at the snapshot: left as resolved
    D, bad = run({"lib.rs:5:gone": ("deleted", "fn gone is deleted")})
    assert [u[1] for u in bad] == ["lib.rs:2"]
    D, bad = run({"lib.rs:5:gone": ("deleted", "gone"), "lib.rs:2": ("deleted", "a replaced line is not a deletion")})
    assert [u[1] for u in bad] == ["lib.rs:2"]                 # the allowed kind must be the kind the map reported
    with pytest.raises(SystemExit):
        run({"lib.rs:2": ("replaced", "never allowed")})


def test_a_caller_s_zero_edges_count_is_reduced_by_exactly_the_pruned_edges():
    _, rule_ops = lib()
    edges = [{"caller": "x", "callee": "a"}, {"caller": "x", "callee": "b"}, {"caller": "y", "callee": "a"}]
    pruned = [{"family": "edge_zero", "rule": {"caller": "x", "callee": "gone"}}]
    log = rule_ops.remove_caller_zero_edges(edges, [("x", 3, "why")], pruned)
    assert edges == [{"caller": "y", "callee": "a"}]
    assert log[0][2] == ["gone"] and log[1][2] == ["a", "b"]
    with pytest.raises(SystemExit):
        rule_ops.remove_caller_zero_edges([{"caller": "x", "callee": "a"}], [("x", 3, "why")], pruned)


def test_the_generator_wraps_a_basis_that_continues_the_source_sentence():
    import ast
    src = (TOOL / "base" / "g5_profile.py").read_text()
    fn = next(n for n in ast.parse(src).body if isinstance(n, ast.FunctionDef) and n.name == "source_clause")
    ns = {}
    exec(compile(ast.Module(body=[fn], type_ignores=[]), "g5_profile.py", "exec"), ns)
    head = "/// candidates) is taken here, in the build. Source: the G4 chain with RV84/RV87's corrections"
    assert ns["source_clause"]("SQ2 G5 at x: y") == [head + " at", "/// SQ2 G5 at x: y."]
    lines = ns["source_clause"](", re-run as SQ2 G5 at f78f52b7e2: the maximum over routes L (C_eq <= 3, worst shape c = 3, "
                                "z = 0) and E (c = 3), under SQ2's rules; text at l <= 128.")
    assert lines == [head + ",",
                     "/// re-run as SQ2 G5 at f78f52b7e2: the maximum over routes L (C_eq <= 3, worst shape c = 3, z = 0)",
                     "/// and E (c = 3), under SQ2's rules; text at l <= 128."]
    assert all(len(l) <= 100 for l in lines)
