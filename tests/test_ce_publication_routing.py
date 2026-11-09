"""CE's two source-derived routing bindings on the integrated S6 predecessor.

A completed migration proof, frozen at the actual CE merge (`CE_STAGE`): the
complete routing inventory of that commit must equal S6's inventory with
exactly two leaves re-derived from that commit's blobs (the product-physics
Rust tree digest, by the maintained algorithm, and the CE reader's SHA-256).
That historical document is then projected back to S6 for its historical-stage
check. Neither membership nor caller/input, scan or routing rules are
normalized.

The live contract is separate: today's raw inventory, with today's manifest
discovery, must resolve under the unchanged maintained resolver
(`numerical_ci.source_read_dependencies`). Later source files are governed by
the current scan boundaries and that resolver, never by this frozen proof.
"""
from copy import deepcopy
from functools import cache
import hashlib
import importlib.util
import json
from pathlib import Path, PurePosixPath
import subprocess

import pytest

ROOT = Path(__file__).resolve().parents[1]
S6_PREDECESSOR = "b24b9a5669f573a62d23eaafe7e12503fe29e195"
ROUTING = "tools/hosted-ci-routing.json"
PP_MANIFEST = "core/product_physics/Cargo.toml"
CE_READER = "core/product_physics/tests/rt_a3_constant_effort_references.rs"
REFUSAL = "routing inventory changed outside its derivation"
# The actual CE merge (#193): the commit at which this derivation was proven.
CE_STAGE = "726ad406461e9b92791f2e0b83da31742793151e"


@cache
def _blob(rev, name):
    return subprocess.check_output(["git", "-C", str(ROOT), "show", f"{rev}:{name}"])


def stage_text(rev, name):
    return _blob(rev, name).decode()


@cache
def stage_files(rev, prefix):
    """Every tracked file at `rev` under the directory `prefix` ("" is the root)."""
    pathspec = ["--", prefix] if prefix else []
    listed = subprocess.check_output(
        ["git", "-C", str(ROOT), "ls-tree", "-r", "-z", "--name-only", rev, *pathspec])
    return tuple(name for name in listed.decode().split("\0") if name)


def historical_rust_tree_digest(rev, manifest):
    """`numerical_ci.rust_tree_digest` over the crate's blobs at `rev`: the same rows
    (crate-relative path, SHA-256 of the bytes), the same Path order and the same hash."""
    crate = PurePosixPath(manifest).parent
    rows = []
    for path in sorted(PurePosixPath(name) for name in stage_files(rev, crate.as_posix())):
        relative = path.relative_to(crate)
        if path.suffix == ".rs" and not {"target", ".git"} & set(relative.parts):
            rows.append((relative.as_posix(), hashlib.sha256(_blob(rev, path.as_posix())).hexdigest()))
    return hashlib.sha256(json.dumps(rows).encode()).hexdigest()


def _s6_text():
    return stage_text(S6_PREDECESSOR, ROUTING)


def historical_ce_routing():
    """The complete routing inventory as merged at `CE_STAGE`."""
    return json.loads(stage_text(CE_STAGE, ROUTING))


@cache
def _numerical():
    spec = importlib.util.spec_from_file_location("ce_publication_numerical", ROOT / "tools/ci/numerical_ci.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def derive_ce_routing():
    """The complete S6 inventory, with exactly two bindings derived from `CE_STAGE`'s source."""
    document = json.loads(_s6_text())
    runtime = document["piping_runtime_reads"]
    tree = historical_rust_tree_digest(CE_STAGE, PP_MANIFEST)
    reader = hashlib.sha256(_blob(CE_STAGE, CE_READER)).hexdigest()
    assert tree != runtime["source_trees"][PP_MANIFEST], "CE must bind its changed Rust tree"
    assert reader != runtime["reads"][CE_READER]["sha256"], "CE must bind its changed runtime reader"
    runtime["source_trees"][PP_MANIFEST] = tree
    runtime["reads"][CE_READER]["sha256"] = reader
    return document


def checked_s6_stage_routing(document):
    """Refuse an invalid historical overlay before restoring only S6's CE-reader SHA."""
    expected = derive_ce_routing()
    # Compare every raw value (including JSON types), list position and membership.
    assert json.dumps(document, sort_keys=True, allow_nan=False) == json.dumps(
        expected, sort_keys=True, allow_nan=False), REFUSAL
    stage = deepcopy(document)
    stage["piping_runtime_reads"]["reads"][CE_READER]["sha256"] = json.loads(
        _s6_text())["piping_runtime_reads"]["reads"][CE_READER]["sha256"]
    return stage


def check_live_runtime_reads():
    """The live contract: today's raw inventory under the unchanged maintained resolver."""
    numerical = _numerical()
    raw = json.loads((ROOT / ROUTING).read_text())
    return numerical.source_read_dependencies(
        ROOT, numerical.readiness.discover_cargo_manifests(ROOT), raw["piping_runtime_reads"])


def test_ce_overlay_is_complete_before_its_single_leaf_stage_projection():
    raw = historical_ce_routing()
    before = deepcopy(raw)
    stage = checked_s6_stage_routing(raw)
    predecessor = json.loads(_s6_text())
    assert raw == before, "checking/projection cannot mutate the raw inventory"
    assert stage["piping_runtime_reads"]["source_trees"][PP_MANIFEST] == historical_rust_tree_digest(CE_STAGE, PP_MANIFEST)
    stage["piping_runtime_reads"]["source_trees"][PP_MANIFEST] = predecessor["piping_runtime_reads"]["source_trees"][PP_MANIFEST]
    assert stage == predecessor, "only the two independently derived CE leaves may differ from integrated S6"


@pytest.mark.parametrize("mutation", [
    "forged_reader", "rolled_back_reader", "forged_tree", "rolled_back_tree",
    "membership", "missing_reader", "caller", "input", "scan", "routing_rule",
])
def test_ce_overlay_refuses_unbound_bindings_and_unrelated_changes(mutation):
    raw = historical_ce_routing()
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


def test_live_inventory_resolves_under_the_maintained_resolver(monkeypatch):
    numerical = _numerical()
    resolver = numerical.source_read_dependencies
    seen = []

    def spy(project, manifests, declarations=None):
        seen.append((project, manifests, declarations))
        return resolver(project, manifests, declarations)

    monkeypatch.setattr(numerical, "source_read_dependencies", spy)
    assert check_live_runtime_reads()
    raw = json.loads((ROOT / ROUTING).read_text())["piping_runtime_reads"]
    assert seen == [(ROOT, numerical.readiness.discover_cargo_manifests(ROOT), raw)], "the raw live inventory, not a projection"


def test_ce_overlay_keeps_the_independent_runtime_read_scanner(monkeypatch):
    def refused(*args, **kwargs):
        raise ValueError("independent runtime-read refusal")

    monkeypatch.setattr(_numerical(), "source_read_dependencies", refused)
    with pytest.raises(ValueError, match="independent runtime-read refusal"):
        check_live_runtime_reads()
