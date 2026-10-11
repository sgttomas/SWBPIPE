"""The retained pricing tool's key carry (tools/retained_pricing/lib): offset, by function, by text, and the prune.

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


def carry(tmp_path, repo, old, new, rules):
    (tmp_path / "crates.txt").write_text("core/x/src\n")
    (tmp_path / "rules.json").write_text(json.dumps(rules))
    out = tmp_path / "out"
    r = subprocess.run([sys.executable, str(TOOL / "lib" / "linemap_rules.py"), "--prune-deleted", str(repo), old, new, str(out),
                        str(tmp_path / "rules.json")], env=dict(os.environ, LINEMAP_CRATES=str(tmp_path / "crates.txt")),
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
