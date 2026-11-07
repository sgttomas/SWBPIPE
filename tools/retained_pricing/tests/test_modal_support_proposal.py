"""Provisional D3 guards for b1e2d1f97; these establish no final registration or compiled-source binding.

Threat model: source drift must reopen the receiver/argument warrants; new callers cannot borrow the literal-stage
bound; production diagnostic bytes stay classed, and a positive or missing retained site must stop O-10.
"""
import json
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODAL = "core/product_physics/src/modal/"


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


@pytest.mark.parametrize("route", ["L", "E"])
def test_stages_are_source_bound_and_both_production_rows_keep_their_bytes(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    support = [(str(k), v) for k, v in d["O10_ID_AUDIT"] if set(v) == {"error", "stage"}]
    assert len(support) == 1
    key, args = support[0]
    assert key.startswith(MODAL + "resource.rs:")
    assert {k: v["bytes"] for k, v in args.items()} == {"error": 8192, "stage": 31}
    guarded = [str(k) for k, why in d["O10_MULT0_SITES"] if "MODAL-ID support proposal" in why]
    assert len(guarded) == 2 and key.split("/src/")[-1] in guarded
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _, _ in d["O10_SITE_SIZE"]}
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _ in d.get("O10_SITE_ZERO", [])}


@pytest.mark.parametrize("file,old,new,refusal", [
    ("eigen.rs", "reported.max(1)", "reported.max(2)", "MODAL-FB: eigen.rs:modal_sturm_count changed"),
    ("resource.rs", "band_pairs: free.min(u128::from(budget.band_pair_allotment)),",
     "band_pairs: free.max(u128::from(budget.band_pair_allotment)),", "MODAL-FB: resource.rs:modal_resource_bound_dimensions changed"),
    ("eigen.rs", 'modal_support_resource_refusal("the dense eigensolve", &support)',
     'modal_support_resource_refusal("the dense eigensolve has a longer stage", &support)', "support refusal caller eigen.rs:solve_modal_pencil_within changed"),
    ("eigen.rs", 'modal_support_resource_refusal("the Sturm pencil\'s construction", &support)',
     'modal_support_resource_refusal("the Sturm pencil\'s construction is much longer", &support)', "support refusal caller eigen.rs:modal_sturm_pencil_refusal changed"),
    ("integrity.rs", 'modal_support_resource_refusal("the formation difference", &support)',
     'modal_support_resource_refusal("the formation difference is much longer", &support)', "support refusal caller integrity.rs:reform_modal_pencil changed"),
])
def test_changed_receivers_or_stage_bindings_stop(tmp_path, monkeypatch, file, old, new, refusal):
    snap = snapshot(tmp_path)
    path = snap / MODAL / file
    text = path.read_text()
    assert text.count(old) == 1
    path.write_text(text.replace(old, new))
    with pytest.raises(SystemExit, match=refusal):
        decisions(monkeypatch, snap)


@pytest.mark.parametrize("use", [
    'fn other() { super::resource::modal_support_resource_refusal("a new stage", &error); }',
    'use super::resource::modal_support_resource_refusal as alias; fn other() { alias("a new stage", &error); }',
])
def test_a_new_caller_or_import_cannot_reuse_the_stage_bound(tmp_path, monkeypatch, use):
    snap = snapshot(tmp_path)
    (snap / MODAL / "new_source.rs").write_text(use + "\n")
    with pytest.raises(SystemExit, match="support refusal helper callers or imports changed"):
        decisions(monkeypatch, snap)


def test_comments_literals_and_test_items_do_not_invent_callers(tmp_path, monkeypatch):
    snap = snapshot(tmp_path)
    (snap / MODAL / "new_source.rs").write_text('''// modal_support_resource_refusal
const NOTE: &str = "modal_support_resource_refusal";
#[cfg(test)] fn test() { modal_support_resource_refusal("test", &error); }
''')
    decisions(monkeypatch, snap)


def test_a_glob_import_cannot_compensate_a_new_call_in_the_same_file(tmp_path, monkeypatch):
    snap = snapshot(tmp_path)
    file = snap / MODAL / "eigen.rs"
    text = file.read_text()
    old = "use super::resource::modal_support_resource_refusal;"
    assert text.count(old) == 1
    text = text.replace(old, "use super::resource::*;")
    declaration = "pub(crate) fn modal_dense_options("
    assert text.count(declaration) == 1
    extra = '''fn extra(error: &open_pipe_stress_modal_eigen::SupportResourceError) {
    let _ = super::resource::modal_support_resource_refusal(&"x".repeat(100), error);
}

'''
    # One import identifier removed and one qualified call added; each pinned caller body is untouched.
    file.write_text(text.replace(declaration, extra + declaration))
    with pytest.raises(SystemExit, match="support refusal direct imports changed"):
        decisions(monkeypatch, snap)


@pytest.mark.parametrize("change", ["zero", "positive", "missing"])
@pytest.mark.parametrize("index", [0, 1])
def test_o10_rejects_a_positive_or_missing_production_site(tmp_path, monkeypatch, change, index):
    d = decisions(monkeypatch)
    keys = [str(k) for k, why in d["O10_MULT0_SITES"] if "MODAL-ID support proposal" in why]
    assert len(keys) == 2
    chain, run = tmp_path / "chain", tmp_path / "run"
    chain.mkdir()
    (run / "work").mkdir(parents=True)
    (chain / "SQ2_RULES.json").write_text(json.dumps({"o10_mult0_sites": keys}))
    (run / "looplog.json").write_text("[]")
    # Use exactly the graph's real row-path shape: no synthetic prefix that could mask a wrong full-path guard.
    rows = [{"file": "core/product_physics/src/" + k.rsplit(":", 1)[0], "line": int(k.rsplit(":", 1)[1]), "mult": 0} for k in keys]
    if change == "positive":
        rows[index]["mult"] = 1
    elif change == "missing":
        rows.pop(index)
    (run / "work/text_budget.caps.out.json").write_text(json.dumps({"rows": rows}))
    found = subprocess.run([sys.executable, str(PRICING / "lib/o10_check.py"), str(chain), str(run)], capture_output=True, text=True)
    assert found.returncode == (0 if change == "zero" else 3), found.stdout + found.stderr
