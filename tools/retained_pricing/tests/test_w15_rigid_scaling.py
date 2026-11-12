"""W15-PM, W15-RB, W15-NZ, W15-L1 and W15-SK (retained-pricing-rules.md §8.21): W-15's working sets are priced only where
its code exists, positively, and a changed or partly present source stops the run."""
from pathlib import Path
import os
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
RC = "core/product_physics/src/rigid_component.rs"
CB = "core/reporting/result_export/src/component_body.rs"
sys.path.insert(0, str(PRICING / "base"))
import w15_owners_model  # noqa: E402


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def test_the_owners_are_present_and_positive():
    assert w15_owners_model.inspect(str(SOURCE))
    from g3lib import HashReq
    assert w15_owners_model.producer_maps(32) == HashReq("(&str,&T)", 32) * 2
    facts = w15_owners_model.extremum_facts()
    # The census extremum dominates the preview one except in its key bytes.
    assert facts == {"arr": 37, "obj": 10, "ent": 46, "strb": 4864, "keyb": 600}


def test_a_snapshot_without_w15_adds_nothing(tmp_path):
    snap = copy_core(tmp_path)
    for rel in (RC, CB):
        os.remove(snap / rel)
    for rel, needle, _ in w15_owners_model.STATEMENTS:
        path = snap / rel
        if path.exists():
            path.write_text(path.read_text(encoding="utf-8").replace(needle, ""), encoding="utf-8")
    assert w15_owners_model.inspect(str(snap)) is False


@pytest.mark.parametrize("change", ["module", "partial", "statement"])
def test_a_changed_or_partial_source_stops(tmp_path, change):
    snap = copy_core(tmp_path)
    if change == "module":
        path = snap / RC
        path.write_text(path.read_text(encoding="utf-8") + "\n// changed\n", encoding="utf-8")
    elif change == "partial":
        os.remove(snap / CB)
    else:
        path = snap / "core/reporting/result_export/src/section_basis.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(text.replace("crate::component_body::coverage_without_bodies(", "(", 1), encoding="utf-8")
    with pytest.raises(SystemExit, match="W15 owners"):
        w15_owners_model.inspect(str(snap))
