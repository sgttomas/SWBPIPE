"""DEL-04-09 completed-set boundary (§8.63, MODAL-CS): the pricing rules for PR 390's fallible typed outcome storage.

No retained root reaches modal, so every new site is at multiplicity zero. The rules:
- MODAL-CS-RK re-keys the dense entry's three sites to run_modal_dense_within, which now holds the body, and stops
  unless run_modal_dense_with_budget is exactly its one-line delegation; the sites' TEXT rows are `format_args!(`.
- resource.rs is re-pinned whole; MODAL-CS-ID audits the boundary's unclassified arguments at source bounds, with
  MODAL-CS-ST reading every storage request's structure name and MODAL-CS-RR the premise that no product build holds a
  caller limit (so a response-row name is empty there).
- MODAL-CS-LI stops if a priced source other than skew_pencil names the frame kernel's new lift_into.
- MODAL-CS-M0 guards every boundary row, the three moved rows and the audited test rows at multiplicity 0; J3's pattern
  sizes the four test Debug sites of a whole ModalOutcome 0.
These are source warrants, not runtime allocation observations.
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
RS, TRS, OUTCOME = MODAL + "resource.rs", MODAL + "tests_resource.rs", MODAL + "outcome.rs"


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


U = 20                     # a usize or u64 decimal
EXPR = 14 + 2 * U          # `r × ρ = {r} × {}` or `{count} entries × {element} B`: 14 literal bytes and two decimals
BOUND = 62 + U             # `the bytes exceed the addressable size, at most isize::MAX = {} B`
STRUCT = 35                # `cumulative effective-mass fractions`, the longest structure constant
LIMIT = 87 + U             # `the caller's limit on it is {limit} and was not exceeded: ...`
EXPECTED_ARGS = [
    (RS, {"expression": EXPR, "limit": U}),
    (RS, {"expression": EXPR, "bound": BOUND}),
    (RS, {"u64::MAX": U}),
    (RS, {"computed": U}),
    (RS, {"completed": U, "width": U, "usize::MAX": U}),
    (RS, {"element": U}),
    (RS, {"isize::MAX": U}),
    (RS, {"r": U}),
    (RS, {"r": U, "sizes.coordinates": U}),
    (RS, {"r": U, "sizes.global": U}),
    (RS, {"r": U, "product.rows_per_mode": U}),
    (RS, {"u64::MAX": U}),
    (RS, {"limit": U}),
    (RS, {"structure": STRUCT, "elements": U, "element_size": U, "isize::MAX": U}),
    (RS, {"bytes": U, "elements": U, "structure": STRUCT, "limit": LIMIT}),
    (TRS, {"actual-1": U}),
    (TRS, {"u64::MAX": U}),
    (TRS, {"usize::MAX": U}),
    (TRS, {"usize::MAX/4": U}),
    (TRS, {"structure": STRUCT}),
]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_boundary_audits_sizes_and_guards_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    found = [(str(k).rsplit(":", 1)[0], {a: v["bytes"] for a, v in value.items()}) for k, value in d["O10_ID_AUDIT"]
             if "MODAL-CS" in next(iter(value.values()))["source"]]
    assert found == EXPECTED_ARGS
    sizes = [(str(k), b) for k, b, why in d["O10_SITE_SIZE"] if "§8.63" in why]
    assert len(sizes) == 4 and all(k.startswith("modal/tests_resource.rs:") and b == 0 for k, b in sizes)
    guarded = [str(k) for k, why in d["O10_MULT0_SITES"] if "MODAL-CS-M0" in why]
    assert len(guarded) == 33 and len(set(guarded)) == 33
    assert sum(k.startswith("modal/resource.rs:") for k in guarded) == 25
    assert sum(k.startswith("modal/outcome.rs:") for k in guarded) == 3
    # Every audited site is guarded, and no guarded site is zeroed or resized (the guard must see its own multiplicity).
    assert {str(k).split("/src/")[-1] for k, v in d["O10_ID_AUDIT"] if "MODAL-CS" in next(iter(v.values()))["source"]} <= set(guarded)
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _, _ in d["O10_SITE_SIZE"]}
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _ in d.get("O10_SITE_ZERO", [])}


def test_the_dense_entry_sites_are_the_format_args_rows_of_the_body(monkeypatch):
    d = decisions(monkeypatch)
    keys = [str(k) for k, v in d["O10_ID_AUDIT"] if str(k).startswith(OUTCOME + ":")
            and set(v) & {"i+1", "orthogonality.pair.map(|(a,b)|(a+1,b+1))|?"}]
    assert len(keys) == 3
    text = (SOURCE / OUTCOME).read_text(encoding="utf-8").split("\n")
    start = next(i for i, l in enumerate(text) if l.startswith("pub(crate) fn run_modal_dense_within("))
    lines = [int(k.rsplit(":", 1)[1]) for k in keys]
    assert all(n - 1 > start for n in lines)
    assert all("format_args!(" in text[n - 1] for n in lines)


@pytest.mark.parametrize("path, old, new, message", [
    # run_modal_dense_with_budget no longer only delegates to run_modal_dense_within.
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n",
     "    let _ = format!(\"{}\", 1);\n    run_modal_dense_within(req, params, budget, None)\n",
     "run_modal_dense_with_budget is not run_modal_dense_within's delegation"),
    # It delegates with limits, so a product build would pass a caller's limits.
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n", "    run_modal_dense_within(req, params, budget, limits())\n",
     "run_modal_dense_with_budget is not run_modal_dense_within's delegation"),
    # Any edit to resource.rs (here a longer boundary refusal) stops for re-review.
    (RS, "over the caller's limit of {limit}; the whole set", "over the caller's stated limit of {limit}; the whole set",
     "modal/resource.rs changed"),
    # A storage request whose structure is not a named literal constant.
    (OUTCOME, "storage.modal_completed_set_rows(total, MODAL_SET_COMPUTED_EIGENVALUES)?;",
     "storage.modal_completed_set_rows(total, req.name())?;", "MODAL-CS-ST"),
    (MODAL + "skew_pencil.rs", "storage.modal_shape_reserve(&mut v, dimension, SKEW_LIFT_SCATTER)?;",
     "let structure = SKEW_LIFT_SCATTER;\n        storage.modal_shape_reserve(&mut v, dimension, structure)?;", "MODAL-CS-ST"),
    ("core/solver/modal_eigen/src/modal_quantities.rs",
     "storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;",
     "storage.modal_shape_reserve(&mut out, modes.len(), \"a literal structure\")?;", "MODAL-CS-ST"),
    # The modal document tests are no longer a test-only module.
    ("core/product_physics/src/modal.rs", "#[cfg(test)]\nmod tests_document;", "mod tests_document;",
     "the modal test modules are not test-only"),
])
def test_a_changed_boundary_source_stops(tmp_path, monkeypatch, path, old, new, message):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)


@pytest.mark.parametrize("path, source, message", [
    # A product build that constructs a caller limit or a response-row product breaks the response-row premise.
    (MODAL + "new_source.rs", "fn rows() -> super::resource::ModalResponseRowProduct {\n"
     "    super::resource::ModalResponseRowProduct { name: \"rows\", rows_per_mode: 1, limit: 1 }\n}\n", "MODAL-CS-RR"),
    (MODAL + "new_source.rs", "fn limits() -> super::resource::ModalCompletedSetLimits {\n    super::resource::ModalCompletedSetLimits "
     "{ computed_modes: 1, completed_modes: 1, normalized_entries: 1, published_entries: 1, response_rows: vec![] }\n}\n",
     "MODAL-CS-RR"),
    # The variant named outside resource.rs.
    (MODAL + "new_source.rs", "fn dimension() -> super::resource::ModalCompletedSetDimension {\n"
     "    super::resource::ModalCompletedSetDimension::ResponseRows(\"rows\")\n}\n", "MODAL-CS-RR"),
    # A priced caller of the frame kernel's lift_into (by name, the graph would reach it).
    ("core/solver/frame_kernel/src/new_source.rs", "fn twice(b: &RestraintBasis, v: &[f64], out: &mut [f64]) {\n"
     "    let _ = b.lift_into(v, out);\n}\n", "MODAL-CS-LI"),
])
def test_a_new_production_source_stops(tmp_path, monkeypatch, path, source, message):
    snap = copy_core(tmp_path)
    (snap / path).write_text(source, encoding="utf-8")
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)


def test_test_code_comments_and_literals_do_not_trip_the_scans(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    (snap / MODAL / "new_source.rs").write_text(
        "// lift_into, ModalResponseRowProduct { .. }, modal_shape_reserve(&mut v, n, x)\n"
        "const NOTE: &str = \"self.lift_into(v, out); ModalCompletedSetLimits { }\";\n"
        "#[cfg(test)] fn test() { let _ = storage.modal_completed_set_rows(1, \"test\"); }\n", encoding="utf-8")
    decisions(monkeypatch, snap)


@pytest.mark.parametrize("change", ["zero", "positive", "missing"])
@pytest.mark.parametrize("index", [0, 24, 30, 32])
def test_o10_rejects_a_positive_or_missing_boundary_site(tmp_path, monkeypatch, change, index):
    d = decisions(monkeypatch)
    keys = [str(k) for k, why in d["O10_MULT0_SITES"] if "MODAL-CS-M0" in why]
    assert len(keys) == 33
    chain, run = tmp_path / "chain", tmp_path / "run"
    chain.mkdir()
    (run / "work").mkdir(parents=True)
    (chain / "SQ2_RULES.json").write_text(json.dumps({"o10_mult0_sites": keys}))
    (run / "looplog.json").write_text("[]")
    rows = [{"file": "core/product_physics/src/" + k.rsplit(":", 1)[0], "line": int(k.rsplit(":", 1)[1]), "mult": 0} for k in keys]
    if change == "positive":
        rows[index]["mult"] = 1
    elif change == "missing":
        rows.pop(index)
    (run / "work/text_budget.caps.out.json").write_text(json.dumps({"rows": rows}))
    found = subprocess.run([sys.executable, str(PRICING / "lib/o10_check.py"), str(chain), str(run)], capture_output=True, text=True)
    assert found.returncode == (0 if change == "zero" else 3), found.stdout + found.stderr
