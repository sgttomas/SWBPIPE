"""C5 source-boundary refusals and exact candidate subtraction controls."""
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
PIPING = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
CPP = "core/product_physics/src/certified_publication.rs"


def snapshot_copy(tmp_path):
    files = {CPP} | {"core/product_physics/src/" + row[0] for row in MODEL["FALLBACK_OUTSIDE_CALLERS"]}
    for file in files:
        target = tmp_path / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PIPING / file, target)
    return tmp_path


def test_current_exact_inventory():
    rules = MODEL["inspect_fallback_boundary"](PIPING)
    # The current source has U2's arc pressure rows and the join's J7 census
    # caller after B7a E11's unused interior Lamé removal: 16 + 1 + 2
    # caller/name pairs (KD5C5, JC5, KD5C5b) from 15 + 1 + 2 callers.
    assert len(rules) == 19
    assert len({r["caller"] for r in rules}) == 18
    assert {r["exclude_targets"][0] for r in rules} == {
        CPP + ":746:add", CPP + ":749:mul", CPP + ":765:divided"}
    assert all(not r["caller"].startswith(CPP + ":") for r in rules)


@pytest.mark.parametrize("old,new", [
    ("struct FallbackError {", "pub(crate) struct FallbackError {"),
    ("    fn add(", "    pub(crate) fn add("),
    ("impl FallbackError {", "impl FallbackError {\n    pub(crate) fn escape() -> Self { Self::ZERO }"),
    ("FallbackError", "RenamedFallbackError"),
    ("fn divided(", "fn quotient("),
])
def test_private_or_escape_boundary_change_refuses(tmp_path, old, new):
    snapshot = snapshot_copy(tmp_path)
    file = snapshot / CPP
    original = file.read_text()
    assert old in original
    file.write_text(original.replace(old, new))
    with pytest.raises(ValueError, match="boundary changed"):
        MODEL["inspect_fallback_boundary"](snapshot)


def test_outside_receiver_change_refuses(tmp_path):
    snapshot = snapshot_copy(tmp_path)
    file = snapshot / "core/product_physics/src/annulus_geometry.rs"
    original = file.read_text()
    body, _ = MODEL["_indented_function"](original, "source_wall_area_scaled")
    file.write_text(original.replace(body, body.replace(".mul(", ".add(")))
    assert file.read_text() != original
    with pytest.raises(ValueError, match="outside caller/receiver body changed"):
        MODEL["inspect_fallback_boundary"](snapshot)


@pytest.fixture
def tiny_graph(tmp_path):
    source = tmp_path / "core/product_physics/src"
    source.mkdir(parents=True)
    methods = """    fn add(self, other: Self) -> Self { self }
    fn mul(self, other: Self) -> Self { self }
    fn divided(self, denominator: f64) -> Self { self }
"""
    (source / "certified_publication.rs").write_text(
        "#[derive(Clone, Copy)]\nstruct FallbackError;\nimpl FallbackError {\n" + methods + "}\n"
        "fn inside(x: FallbackError) {\n    let get = || x;\n    get().add(x);\n    get().mul(x);\n    get().divided(1.0);\n}\n")
    (source / "outside.rs").write_text(
        "#[derive(Clone, Copy)]\nstruct Other;\nimpl Other {\n" + methods + "}\n"
        "fn outside(x: Other) {\n    let get = || x;\n    get().add(x);\n    get().mul(x);\n    get().divided(1.0);\n}\n")
    def run(rules=None):
        env = dict(os.environ, CG_EDGES_OUT=str(tmp_path / "edges.json"))
        env.pop("CG_RULES", None)
        if rules is not None:
            rule_file = tmp_path / "rules.json"
            rule_file.write_text(json.dumps({"rules": rules}))
            env["CG_RULES"] = str(rule_file)
        result = subprocess.run([sys.executable, str(PRICING / "base/callgraph_g5.py"),
            str(tmp_path), str(tmp_path / "graph.json"), "outside,inside", "core/product_physics/src"],
            env=env, capture_output=True, text=True)
        edges = json.loads((tmp_path / "edges.json").read_text())["edges"] if result.returncode == 0 else None
        return result, edges
    return run


def test_subtract_only_qualified_outside_targets(tiny_graph):
    result, before = tiny_graph()
    assert result.returncode == 0, result.stderr
    caller = next(k for k in before if k.endswith(":outside"))
    inside = next(k for k in before if k.endswith(":inside"))
    targets = [k for k in before if k.startswith(CPP + ":") and k.rsplit(":", 1)[1] in ("add", "mul", "divided")]
    assert len(targets) == 3 and set(targets) <= set(before[caller]) & set(before[inside])
    rules = [{"caller": caller, "name": t.rsplit(":", 1)[1], "exclude_targets": [t]} for t in targets]
    result, after = tiny_graph(rules)
    assert result.returncode == 0, result.stderr
    assert set(after[caller]) == set(before[caller]) - set(targets)
    assert len(after[caller]) >= 3  # other owners remain conservative candidates
    assert {k: v for k, v in after.items() if k != caller} == {k: v for k, v in before.items() if k != caller}
    # An existing explicit target list also precedes, rather than bypasses, subtraction.
    for rule in rules:
        rule["targets"] = [k for k in before[caller] if k.rsplit(":", 1)[1] == rule["name"]]
    result, with_replacement = tiny_graph(rules)
    assert result.returncode == 0, result.stderr
    assert with_replacement == after


@pytest.mark.parametrize("kind", ["missing_caller", "missing_target", "unwitnessed"])
def test_changed_exclusion_inventory_refuses(tiny_graph, kind):
    result, before = tiny_graph()
    assert result.returncode == 0
    caller = next(k for k in before if k.endswith(":outside"))
    target = next(k for k in before if k.startswith(CPP + ":") and k.endswith(":add"))
    rule = {"caller": caller, "name": "add", "exclude_targets": [target]}
    if kind == "missing_caller":
        rule["caller"] += "_missing"
    elif kind == "missing_target":
        rule["exclude_targets"] = [target + "_missing"]
    else:
        rule["name"] = "never_called"
    result, _ = tiny_graph([rule])
    assert result.returncode != 0
    assert "exclusion" in result.stderr


def test_no_exclusions_keeps_original_graph(tiny_graph):
    result, before = tiny_graph()
    assert result.returncode == 0
    result, after = tiny_graph([])
    assert result.returncode == 0 and after == before
