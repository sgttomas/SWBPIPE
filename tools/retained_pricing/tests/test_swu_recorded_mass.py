"""SWU-RM: certified_loads::recorded_mass (the self-weight units repair) clones and deserialises a generated record's
normalized_dependencies. The retained route charges the edge intensity -> recorded_mass zero only because D1.10
(retained_memory.rs provenance_clause) refuses every object provenance, so intensity's GENERATED_METHOD arm never runs
on an admitted model. A widened D1.10, a dropped call of it, or a call of recorded_mass outside that arm stops the
pricing run until the allocations are priced."""
from pathlib import Path
import hashlib
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
CL = "core/product_physics/src/certified_loads.rs"
RM = "core/product_physics/src/retained_memory.rs"


def decisions(monkeypatch, snapshot=SOURCE, route="L", pricing=PRICING):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(pricing / f"rules/sq2_decisions_{route}.py"))


def repinned_tool(tmp_path, snapshot, path):
    """A copy of this tool whose full-file pins on `path` name the snapshot's bytes: a re-pin made without re-reading
    the change. The rule's own guard must still stop a moved call."""
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("__pycache__", "tests"))
    old = hashlib.sha256((SOURCE / path).read_bytes()).hexdigest()
    new = hashlib.sha256((snapshot / path).read_bytes()).hexdigest()
    moved = 0
    for model in (tool / "base").glob("*.py"):
        text = model.read_text(encoding="utf-8")
        if old in text:
            model.write_text(text.replace(old, new), encoding="utf-8")
            moved += 1
    assert moved
    return tool


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_recorded_mass_edge_is_zero_on_both_routes(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    edges = [(a, b) for a, b, why in d["O10_EDGE_ZERO"] if "SWU-RM" in why]
    assert [(str(a).split(":")[-1], str(b).split(":")[-1]) for a, b in edges] == [("intensity", "recorded_mass")]


@pytest.mark.parametrize("path, old, new", [
    # D1.10 no longer refuses an object provenance.
    (RM, "        if load.provenance.as_deref().is_some_and(object) {\n",
     "        if false && load.provenance.as_deref().is_some_and(object) {\n"),
    # D1.10 is no longer among the domain clauses.
    (RM, "    provenance_clause(request)?;\n", "    let _ = provenance_clause;\n"),
])
def test_a_changed_premise_stops(tmp_path, monkeypatch, path, old, new):
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises(SystemExit, match="SWU-RM"):
        decisions(monkeypatch, snapshot=snap)


BRANCH = '                if evidence["method"] == self_weight::GENERATED_METHOD {\n                    // This source'


def test_a_moved_call_stops_at_the_full_file_pin_and_after_a_bare_re_pin(tmp_path, monkeypatch):
    # recorded_mass reached outside the GENERATED_METHOD arm: certified_loads.rs's full-file pin stops it first, and a
    # re-pin of that hash alone still stops at this rule.
    snap = snapshot_with(tmp_path, CL, BRANCH, BRANCH.replace(
        'evidence["method"] == self_weight::GENERATED_METHOD', 'evidence["method"] != serde_json::Value::Null'))
    with pytest.raises(ValueError, match="certified_loads.rs"):
        decisions(monkeypatch, snapshot=snap)
    with pytest.raises(SystemExit, match="SWU-RM"):
        decisions(monkeypatch, snapshot=snap, pricing=repinned_tool(tmp_path, snap, CL))
