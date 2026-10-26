"""W15B-NS (retained-pricing-rules.md §8.22): W-15 slice b's component-weight lineage functions are reached only after
inspect_applied_self_weight's `continue`, which D1.10 and D1.7 (now also refusing a node load in the generated
self-weight namespace) make every admitted load take. The edge into generation_basis is zeroed only where the code
exists (`fnkey`), and a changed premise or refusal stops the run. An earlier self_weight.rs stops at
b7a_formation_model's full-file pin before any rule applies."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
SW = "core/product_physics/src/self_weight.rs"
RM = "core/product_physics/src/retained_memory.rs"
NODE_REFUSAL = 'if load.id.starts_with("load:self-weight:") {\n                        return refuse(C::Loads, F::GeneratedLoadIdentity);'


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


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_lineage_edge_and_key_site_are_zero_on_both_routes(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    edges = [(a, b) for a, b, why in d["O10_EDGE_ZERO"] if "W15B-NS" in why]
    assert [(str(a).split(":")[-1], str(b).split(":")[-1]) for a, b in edges] == [("inspect_applied_self_weight", "generation_basis")]
    assert sum(1 for _key, why in d["O10_SITE_ZERO"] if "W15B-NS" in why) == 1


@pytest.mark.parametrize("path, old, new", [
    (RM, NODE_REFUSAL, "if false {\n                        return refuse(C::Loads, F::GeneratedLoadIdentity);"),
    (SW, 'super::LoadTargetInput::Node { .. } => load.id.starts_with("load:self-weight:"),',
     'super::LoadTargetInput::Node { .. } => load.id.contains("self-weight"),'),
    (SW, "if !recognized && !looks_generated {\n                continue;", "if !recognized {\n                continue;"),
    (SW, "let row = generated.source_evidence.iter().position(|row| {", "let _ = generation_basis(evidence, &case.id);\n                let row = generated.source_evidence.iter().position(|row| {"),
])
def test_a_changed_premise_or_refusal_stops(tmp_path, monkeypatch, path, old, new):
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises(SystemExit, match="W15B-NS"):
        decisions(monkeypatch, snapshot=snap)
