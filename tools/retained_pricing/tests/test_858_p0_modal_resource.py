"""DEL-04-09 O-4 P0 and S1 (retained-pricing-rules.md §8.58): the modal resource preflight's pricing rules.

No retained root reaches modal, so every resource.rs and tests_resource.rs site is at multiplicity zero. The rules:
- MODAL-ID re-keys run_modal_dense's three sites to run_modal_dense_with_budget, which now holds the body, and stops
  unless run_modal_dense is exactly its one-line delegation;
- MODAL-FB subtracts modal_resource_bound_dimensions' `.min(` (a u128 receiver) from certified_publication's private
  FallbackError, its body pinned;
- MODAL-ID audits resource.rs's refusal arguments (resource.rs pinned whole) and tests_resource.rs's, and sizes the two
  unboundable test Debug sites 0 under J3's multiplicity-0 guard.
These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
RS = "core/product_physics/src/modal/resource.rs"
TRS = "core/product_physics/src/modal/tests_resource.rs"
OUTCOME = "core/product_physics/src/modal/outcome.rs"
MODAL = "core/product_physics/src/modal.rs"


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


U128, U64 = 39, 20
ENTRY = 19 + 3 + (11 + 143)   # `{id} ({why})`: a 19 B id, ` ()`, `unbounded: ` and the longest reason
# In rule order: resource.rs's six sites, then tests_resource.rs's three.
EXPECTED_ARGS = [
    {"overflow.0": 19},
    {'term.modal_resource_unbounded_reason().unwrap_or("noboundatP0")': 143},
    {"count": U128},
    {"id": 19, "why": 154},
    {'ifinadmissible.len()==1{"aterm"}else{"theseterms"}': 11, 'stated.join(";")': 34 * ENTRY + 33 * 2},
    {"total": U128, "budget.work_budget": U128, "id": 19, "counts[dominantasusize]": U128,
     "budget.weights[dominantasusize].unwrap_or_default()": U64, "charges[dominantasusize]": U128},
    {"total": U128},
    {"ql_rotations": U64},
    {"hard_rejected": 5, "retained": 5},
]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_p0_audits_and_test_site_sizes_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    found = [{a: v["bytes"] for a, v in value.items()} for key, value in d["O10_ID_AUDIT"]
             if str(key).rsplit(":", 1)[0] in (RS, TRS)]
    assert found == EXPECTED_ARGS
    sizes = [(str(k), b) for k, b, why in d["O10_SITE_SIZE"] if "§8.58" in why]
    assert [b for _, b in sizes] == [0, 0]
    assert all(k.startswith("modal/tests_resource.rs:") for k, _ in sizes)


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_dense_entry_sites_follow_its_body(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    keys = [str(k) for k, v in d["O10_ID_AUDIT"] if str(k).startswith(OUTCOME + ":")
            and set(v) & {"i+1", "orthogonality.pair.map(|(a,b)|(a+1,b+1))|?"}]
    assert len(keys) == 3
    text = (SOURCE / OUTCOME).read_text(encoding="utf-8").split("\n")
    start = next(i for i, l in enumerate(text) if l.startswith("pub(crate) fn run_modal_dense_with_budget("))
    assert all(int(k.rsplit(":", 1)[1]) - 1 > start for k in keys)


def test_the_bound_dimensions_min_is_subtracted(monkeypatch):
    d = decisions(monkeypatch)
    rules = [r for r in d["CG_RULES"] if str(r["caller"]).startswith(RS.split("/src/")[0]) and "§8.58" in r["evidence"]]
    assert [(str(r["caller"]).rsplit(":", 1)[-1], r["name"]) for r in rules] == [("modal_resource_bound_dimensions", "min")]


@pytest.mark.parametrize("path, old, new, message", [
    # Any edit to resource.rs (here a longer unbounded reason) stops for re-review.
    (RS, 'Some("the confirmation evaluations have no source bound', 'Some("the confirmation evaluations have no source bound at all',
     "modal/resource.rs changed"),
    # The bound-dimensions body (its pinned MODAL-FB text) changes: the MODAL-FB pin stops first.
    (RS, "band_pairs: free.min(u128::from(budget.band_pair_allotment)),", "band_pairs: free.max(u128::from(budget.band_pair_allotment)),",
     "MODAL-FB: resource.rs:modal_resource_bound_dimensions changed"),
    # run_modal_dense no longer only delegates.
    (OUTCOME, "    run_modal_dense_with_budget(req, params, None)\n",
     "    let _ = format!(\"{}\", 1);\n    run_modal_dense_with_budget(req, params, None)\n",
     "run_modal_dense is not run_modal_dense_with_budget's delegation"),
    # tests_resource is no longer a test-only module.
    (MODAL, "#[cfg(test)] mod tests_resource;", "mod tests_resource;", "tests_resource is no longer a test-only module"),
])
def test_a_changed_resource_source_stops(tmp_path, monkeypatch, path, old, new, message):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)
