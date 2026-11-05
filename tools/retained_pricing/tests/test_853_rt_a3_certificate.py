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
