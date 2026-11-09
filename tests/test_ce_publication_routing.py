"""CE's two source-derived routing bindings on the integrated S6 predecessor.

Validate the complete current overlay before projecting the one CE-reader leaf
back to S6 for its historical-stage check. Neither membership nor caller/input,
scan or routing rules are normalized.
"""
from copy import deepcopy
from functools import cache
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess

import pytest

ROOT = Path(__file__).resolve().parents[1]
S6_PREDECESSOR = "b24b9a5669f573a62d23eaafe7e12503fe29e195"
ROUTING = "tools/hosted-ci-routing.json"
PP_MANIFEST = "core/product_physics/Cargo.toml"
CE_READER = "core/product_physics/tests/rt_a3_constant_effort_references.rs"
REFUSAL = "routing inventory changed outside its derivation"


@cache
def _s6_text():
    return subprocess.check_output(
        ["git", "-C", str(ROOT), "show", f"{S6_PREDECESSOR}:{ROUTING}"], text=True)


@cache
def _numerical():
    spec = importlib.util.spec_from_file_location("ce_publication_numerical", ROOT / "tools/ci/numerical_ci.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def derive_ce_routing():
    """The complete S6 inventory, with exactly two actual-source bindings."""
    document = json.loads(_s6_text())
    runtime = document["piping_runtime_reads"]
    tree = _numerical().rust_tree_digest(ROOT, Path(PP_MANIFEST))
    reader = hashlib.sha256((ROOT / CE_READER).read_bytes()).hexdigest()
    assert tree != runtime["source_trees"][PP_MANIFEST], "CE must bind its changed Rust tree"
    assert reader != runtime["reads"][CE_READER]["sha256"], "CE must bind its changed runtime reader"
    runtime["source_trees"][PP_MANIFEST] = tree
    runtime["reads"][CE_READER]["sha256"] = reader
    return document


def checked_s6_stage_routing(document):
    """Refuse an invalid raw overlay before restoring only S6's CE-reader SHA."""
    expected = derive_ce_routing()
    # Compare every raw value (including JSON types), list position and membership.
    assert json.dumps(document, sort_keys=True, allow_nan=False) == json.dumps(
        expected, sort_keys=True, allow_nan=False), REFUSAL
    numerical = _numerical()
    numerical.source_read_dependencies(
        ROOT, numerical.readiness.discover_cargo_manifests(ROOT), document["piping_runtime_reads"])
    stage = deepcopy(document)
    stage["piping_runtime_reads"]["reads"][CE_READER]["sha256"] = json.loads(
        _s6_text())["piping_runtime_reads"]["reads"][CE_READER]["sha256"]
    return stage


def test_ce_overlay_is_complete_before_its_single_leaf_stage_projection():
    raw = json.loads((ROOT / ROUTING).read_text())
    before = deepcopy(raw)
    stage = checked_s6_stage_routing(raw)
    predecessor = json.loads(_s6_text())
    assert raw == before, "checking/projection cannot mutate the raw inventory"
    assert stage["piping_runtime_reads"]["source_trees"][PP_MANIFEST] == _numerical().rust_tree_digest(ROOT, Path(PP_MANIFEST))
    stage["piping_runtime_reads"]["source_trees"][PP_MANIFEST] = predecessor["piping_runtime_reads"]["source_trees"][PP_MANIFEST]
    assert stage == predecessor, "only the two independently derived CE leaves may differ from integrated S6"


@pytest.mark.parametrize("mutation", [
    "forged_reader", "rolled_back_reader", "forged_tree", "rolled_back_tree",
    "membership", "missing_reader", "caller", "input", "scan", "routing_rule",
])
def test_ce_overlay_refuses_unbound_bindings_and_unrelated_changes(mutation):
    raw = json.loads((ROOT / ROUTING).read_text())
    runtime = raw["piping_runtime_reads"]
    predecessor = json.loads(_s6_text())["piping_runtime_reads"]
    if mutation == "forged_reader":
        runtime["reads"][CE_READER]["sha256"] = "0" * 64
    elif mutation == "rolled_back_reader":
        runtime["reads"][CE_READER]["sha256"] = predecessor["reads"][CE_READER]["sha256"]
    elif mutation == "forged_tree":
        runtime["source_trees"][PP_MANIFEST] = "0" * 64
    elif mutation == "rolled_back_tree":
        runtime["source_trees"][PP_MANIFEST] = predecessor["source_trees"][PP_MANIFEST]
    elif mutation == "membership":
        runtime["source_trees"]["core/not-a-crate/Cargo.toml"] = "0" * 64
    elif mutation == "missing_reader":
        runtime["reads"].pop(CE_READER)
    elif mutation == "caller":
        runtime["reads"][CE_READER]["source_trees"].pop()
    elif mutation == "input":
        runtime["reads"][CE_READER]["inputs"].pop()
    elif mutation == "scan":
        source = "core/solver/nonlinear_integration/src/s11k_tests.rs"
        entry = next(e for e in runtime["reads"][source]["inputs"] if e.get("scan"))
        entry["scan"].setdefault("exclude", []).append("src")
    else:
        raw["path_rules"].pop(0)
    with pytest.raises(AssertionError, match=REFUSAL):
        checked_s6_stage_routing(raw)


def test_ce_overlay_keeps_the_independent_runtime_read_scanner(monkeypatch):
    raw = json.loads((ROOT / ROUTING).read_text())

    def refused(*args, **kwargs):
        raise ValueError("independent runtime-read refusal")

    monkeypatch.setattr(_numerical(), "source_read_dependencies", refused)
    with pytest.raises(ValueError, match="independent runtime-read refusal"):
        checked_s6_stage_routing(raw)
