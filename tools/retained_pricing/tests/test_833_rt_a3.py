"""RTA3-JW, RTA3-ID and RTA3-BB (retained-pricing-rules.md §8.33): RT-A3 increment 1's junction warnings and branch
rows are zero on D1 under guarded premises (D1.5's empty regions, D1.4's refusal of every component); their texts keep
classes. The template inventory reads Rust's `\\u{...}` escape as its one character. A changed premise stops the run."""
from pathlib import Path
import json
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PR = "core/product_physics/src/pressure_runtime.rs"
U4R = "core/product_physics/src/u4b_rows.rs"
LIB = "core/product_physics/src/lib.rs"
EA = "core/product_physics/src/exact_admission.rs"


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def const_len(name):
    text = (SOURCE / EA).read_text(encoding="utf-8")
    return len(text.split(f"{name}: &str =", 1)[1].split('"', 2)[1].encode())


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_new_code_is_zero_on_d1(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {(r[0], r[1], tuple(r[3])) for r in d["O10_LOOPS_LAST"] if "RTA3-" in r[2]}
    assert loops == {
        ("for (node, ending) in at_node.into_iter().filter(|(_, ending)| ending.len() >= 2)", "0",
         ("pressure_runtime.rs:case_junction_closure_warnings",)),
        ("iter:ending.iter().map", "0", ("pressure_runtime.rs:case_junction_closure_warnings",)),
        ("for b in bases", "2", ("u4b_rows.rs:append_branch_intensified",)),
    }
    audits = [audit for _, audit in d["O10_ID_AUDIT"] if any("RTA3-ID" in v["source"] for v in audit.values())]
    merged = {name: v["bytes"] for audit in audits for name, v in audit.items()}
    assert len(audits) == 4
    assert merged["authored"] == 456 and merged["listed"] == 585 and merged["regions"] == 128
    assert merged["scalar_string(q.value)"] == 327 and merged["q.unit"] == 128
    assert merged["exact_admission::JUNCTION_PRESSURES_DIFFER_TEXT"] == const_len("JUNCTION_PRESSURES_DIFFER_TEXT")
    assert merged["exact_admission::JUNCTION_PIPE_OUTSIDE_REGIONS_TEXT"] == const_len("JUNCTION_PIPE_OUTSIDE_REGIONS_TEXT")


@pytest.mark.parametrize("path, old, new, stop", [
    # The junction warnings run without an exact contract.
    (PR, "fn junction_closure_warnings(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {\n"
         "    if exact_contract(model).is_none() {\n        return;\n    }\n",
     "fn junction_closure_warnings(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {\n",
     "RTA3-JW: the junction-warning premise changed"),
    # The junction map is filled outside the region loop.
    (PR, "    let mut at_node: BTreeMap<&str, Vec<&PressureRegionInput>> = BTreeMap::new();\n",
     "    let mut at_node: BTreeMap<&str, Vec<&PressureRegionInput>> = BTreeMap::new();\n    at_node.entry(case_id).or_default();\n",
     "RTA3-JW: the junction-warning premise changed"),
    # A branch row is formed outside the association loop.
    (U4R, "    for (location, node) in [(\"end_i\", &pipe.from), (\"end_j\", &pipe.to)] {\n"
          "        for association in super::preview_physics::associations(model, node, pipe_id) {\n",
     "    for (location, node) in [(\"end_i\", &pipe.from), (\"end_j\", &pipe.to)] {\n"
     "        for association in super::preview_physics::associations(model, node, pipe_id).into_iter().chain(None) {\n",
     "RTA3-BB: the branch-row premise changed"),
    # The case solve appends branch rows on the pressure-free route too (the case solve's body pin stops first).
    (LIB, "            if exact && macro_bend.is_none() {\n                directional_measures.extend(u4b_rows::append_branch_intensified(",
     "            if macro_bend.is_none() {\n                directional_measures.extend(u4b_rows::append_branch_intensified(",
     "KD5T2: source pin changed: core/product_physics/src/lib.rs:solve_prepared_load_case"),
])
def test_a_changed_premise_stops(tmp_path, monkeypatch, path, old, new, stop):
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, snapshot=snap)


def test_the_template_inventory_reads_a_rust_unicode_escape(tmp_path):
    src = tmp_path / "a.rs"
    src.write_text('fn f(a: &str, b: &str) -> String {\n    format!("{a}\\u{0}{b}:x")\n}\n', encoding="utf-8")
    out = subprocess.run([sys.executable, str(PRICING / "base/template_inventory.py"), str(tmp_path), "a.rs"],
                         capture_output=True, text=True, check=True).stdout
    row = json.loads(out)["rows"][0]
    assert row["literal_bytes"] == 3
    assert [p["arg"] for p in row["placeholders"]] == ["a", "b"]
