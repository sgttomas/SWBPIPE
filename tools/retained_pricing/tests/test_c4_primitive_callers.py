"""E14 caller retirement keeps exact source bindings and current C4 targets."""
import ast
import os
from pathlib import Path
import re
import runpy

import pytest

PRICING = Path(__file__).resolve().parents[1]
PIPING = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
LIB = "core/product_physics/src/lib.rs"


def c4_rules(snapshot, override=None):
    source = (PRICING / "rules/o10_rules.py").read_text()
    namespace = {"_SNAP": str(snapshot), "_os": os, "_re": re,
                 "Resolved": lambda key: key, "PP_": "core/product_physics/src/",
                 "FK_": "core/solver/frame_kernel/src/", "_CP": "core/product_physics/src/certified_publication.rs",
                 "_labels_model": MODEL, "CG_RULES": []}
    for node in ast.parse(source).body:
        if isinstance(node, ast.FunctionDef) and node.name in ("fnkey", "_fn_n"):
            exec(ast.get_source_segment(source, node), namespace)
    if override:
        original = namespace["fnkey"]
        namespace["fnkey"] = lambda name, file: None if name == override else original(name, file)
    start = source.index("# KD5C4:")
    end = source.index("# KD5C5:", start)
    exec(source[start:end], namespace)
    return namespace["CG_RULES"]


def test_only_retired_observation_caller_leaves_and_all_28_current_callers_stay():
    assert MODEL["primitive_lib_minmax_callers"](PIPING) == [
        "build_curved_bend_macro_elements", "straight_summary_extrema"]
    rules = c4_rules(PIPING)
    # RF's upfront unit limit adds prepare_sources' intrinsic usize min; W-8 (W8-CG, §8.16) adds
    # topology.rs's near_duplicate_nodes (f64 box bounds and usize node indices) and scaled_norm (f64).
    assert len(rules) == 60
    assert len({rule["caller"] for rule in rules}) == 30
    assert {rule["name"] for rule in rules if "/topology.rs:" in rule["caller"]} == {"min", "max"}
    assert all(not rule["targets"] for rule in rules if "/topology.rs:" in rule["caller"])
    assert any(rule["caller"].endswith(":prepare_sources") and rule["name"] == "min" for rule in rules)
    assert all("observation_lane_profile" not in rule["caller"] for rule in rules)
    assert all(rule["name"] in ("min", "max") for rule in rules)
    assert all("fallback_section_errors" not in rule["caller"] for rule in rules)
    # The two prior conservative max-target sets are retained.
    exceptions = [rule for rule in rules if rule["targets"]]
    assert len(exceptions) == 2
    assert all(rule["name"] == "max" and len(rule["targets"]) == 3 for rule in exceptions)


@pytest.mark.parametrize("change", ["comment", "crlf", "retired_definition", "current_owner", "missing_current"])
def test_any_full_file_drift_or_old_helper_restoration_refuses(tmp_path, change):
    file = tmp_path / LIB
    file.parent.mkdir(parents=True)
    raw = (PIPING / LIB).read_bytes()
    if change == "comment":
        raw += b"\n// changed source\n"
    elif change == "crlf":
        assert b"\r\n" not in raw
        raw = raw.replace(b"\n", b"\r\n")
    elif change == "retired_definition":
        raw += b"\nfn observation_lane_profile() {}\n"
    else:
        name = b"fn build_curved_bend_macro_elements("
        assert raw.count(name) == 1
        raw = raw.replace(name, b"fn changed_curved_owner(", 1) if change == "current_owner" else raw.replace(name, b"// deleted function(", 1)
    file.write_bytes(raw)
    with pytest.raises(ValueError):
        MODEL["primitive_lib_minmax_callers"](tmp_path)


def test_a_missing_surviving_primitive_caller_still_stops():
    with pytest.raises(SystemExit, match="KD5C4: primitive caller is not unique: .*:row_scales"):
        c4_rules(PIPING, override="row_scales")
