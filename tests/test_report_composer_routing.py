"""The report composer's source-derived routing bindings on integrated CE (#193).

The composer changes three reporting Rust trees and adds one compiled source and two
JSON fixtures inside two declared runtime-read scans. Validate the complete raw
inventory against CE's plus exactly those derived bindings, run the independent
scanner on it, then project those bindings back to CE's values for CE's own
stage check. No membership, caller, input, scan rule or routing rule is normalized.
"""
from copy import deepcopy
from functools import cache
import importlib.util
import json
from pathlib import Path
import subprocess

import pytest

ROOT = Path(__file__).resolve().parents[1]
CE_PREDECESSOR = "726ad406461e9b92791f2e0b83da31742793151e"
ROUTING = "tools/hosted-ci-routing.json"
REPORT_TREES = ("core/reporting/pdf_emitter/Cargo.toml",
                "core/reporting/report_package/Cargo.toml",
                "core/reporting/report_renderer/Cargo.toml")
# (declaring source, scan anchor, scan suffix) -> files the composer adds to that scan.
REPORT_SCANS = {
    ("core/reporting/result_export/tests/retained_precision_carriers.rs", "../../..", ".rs"):
        ("core/reporting/report_renderer/src/composer.rs",),
    ("core/reporting/result_export/tests/hash_corpus.rs", "../../../fixtures", ".json"):
        ("fixtures/reports/invented/report_composition_families.json",
         "fixtures/reports/invented/report_composition_vectors.json"),
}
REFUSAL = "routing inventory changed outside its derivation"


@cache
def _ce_text():
    return subprocess.check_output(
        ["git", "-C", str(ROOT), "show", f"{CE_PREDECESSOR}:{ROUTING}"], text=True)


@cache
def _numerical():
    spec = importlib.util.spec_from_file_location("report_composer_numerical", ROOT / "tools/ci/numerical_ci.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _scan_entry(runtime, key):
    source, anchor, suffix = key
    found = [e for e in runtime["reads"][source]["inputs"]
             if e.get("anchor") == anchor and (e.get("scan") or {}).get("suffix") == suffix]
    assert len(found) == 1, key
    return found[0]


def derive_report_routing():
    """CE's complete inventory with exactly the composer's actual-source bindings."""
    document = json.loads(_ce_text())
    runtime = document["piping_runtime_reads"]
    for manifest in REPORT_TREES:
        tree = _numerical().rust_tree_digest(ROOT, Path(manifest))
        assert tree != runtime["source_trees"][manifest], "the composer must bind its changed Rust tree"
        runtime["source_trees"][manifest] = tree
    for key, additions in REPORT_SCANS.items():
        files = _scan_entry(runtime, key)["files"]
        assert files == sorted(files), key
        for name in additions:
            assert name not in files and (ROOT / name).is_file(), name
        files[:] = sorted(files + list(additions))
    return document


def checked_ce_stage_routing(document):
    """Refuse an invalid raw inventory, then return it projected to CE's stage."""
    expected = derive_report_routing()
    # Compare every raw value (including JSON types), list position and membership.
    assert json.dumps(document, sort_keys=True, allow_nan=False) == json.dumps(
        expected, sort_keys=True, allow_nan=False), REFUSAL
    numerical = _numerical()
    numerical.source_read_dependencies(
        ROOT, numerical.readiness.discover_cargo_manifests(ROOT), document["piping_runtime_reads"])
    stage = deepcopy(document)
    runtime, ce = stage["piping_runtime_reads"], json.loads(_ce_text())["piping_runtime_reads"]
    for manifest in REPORT_TREES:
        runtime["source_trees"][manifest] = ce["source_trees"][manifest]
    for key, additions in REPORT_SCANS.items():
        files = _scan_entry(runtime, key)["files"]
        files[:] = [name for name in files if name not in additions]
    return stage


def test_report_overlay_is_complete_before_its_stage_projection():
    raw = json.loads((ROOT / ROUTING).read_text())
    before = deepcopy(raw)
    stage = checked_ce_stage_routing(raw)
    assert raw == before, "checking/projection cannot mutate the raw inventory"
    assert stage == json.loads(_ce_text()), "only the composer's derived leaves may differ from integrated CE"


@pytest.mark.parametrize("mutation", [
    "forged_tree", "rolled_back_tree", "missing_scan_addition", "extra_scan_file",
    "ce_leaf", "caller", "routing_rule",
])
def test_report_overlay_refuses_unbound_bindings_and_unrelated_changes(mutation):
    raw = json.loads((ROOT / ROUTING).read_text())
    runtime = raw["piping_runtime_reads"]
    ce = json.loads(_ce_text())["piping_runtime_reads"]
    scan = next(iter(REPORT_SCANS))
    if mutation == "forged_tree":
        runtime["source_trees"][REPORT_TREES[2]] = "0" * 64
    elif mutation == "rolled_back_tree":
        runtime["source_trees"][REPORT_TREES[1]] = ce["source_trees"][REPORT_TREES[1]]
    elif mutation == "missing_scan_addition":
        _scan_entry(runtime, scan)["files"].remove(REPORT_SCANS[scan][0])
    elif mutation == "extra_scan_file":
        _scan_entry(runtime, scan)["files"].append("core/reporting/report_renderer/src/not_compiled.rs")
    elif mutation == "ce_leaf":
        reader = "core/product_physics/tests/rt_a3_constant_effort_references.rs"
        runtime["reads"][reader]["sha256"] = "0" * 64
    elif mutation == "caller":
        runtime["reads"][scan[0]]["source_trees"].pop()
    else:
        raw["path_rules"].pop(0)
    with pytest.raises(AssertionError, match=REFUSAL):
        checked_ce_stage_routing(raw)


def test_report_overlay_keeps_the_independent_runtime_read_scanner(monkeypatch):
    raw = json.loads((ROOT / ROUTING).read_text())

    def refused(*args, **kwargs):
        raise ValueError("independent runtime-read refusal")

    monkeypatch.setattr(_numerical(), "source_read_dependencies", refused)
    with pytest.raises(ValueError, match="independent runtime-read refusal"):
        checked_ce_stage_routing(raw)
