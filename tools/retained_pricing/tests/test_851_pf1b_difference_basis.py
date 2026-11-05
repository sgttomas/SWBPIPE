"""Operating-case slice 1b (retained-pricing-rules.md §8.51): the declared operating-minus-sustained difference.

D1.4 refuses every combination with a difference basis, so the declared arms, the derivation's records and the model-free
reader's claims are zero on D1 (PF1B-DZ, PF1B-FZ); the reader's model arm is model-bound (PF1B-RM); the by-name call
graph's false in-edges into the modules' private functions are zeroed (PF1B-BN). Each mutation below stops the run or
breaks the premise. These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
DB = "core/product_physics/src/difference_basis.rs"
RD = "core/reporting/result_export/src/preview_difference_basis.rs"


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


def short(key):
    return str(key).split("/src/")[-1]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_zeros_and_bounds_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zeros = {(short(a).rsplit(":", 1)[-1], short(b).rsplit(":", 1)[-1]): why for a, b, why in d["O10_EDGE_ZERO"]
             if "PF1B" in why}
    assert {("gate_reason", "gate"), ("derivation", "contents"), ("free_check", "read"), ("check", "model_check"),
            ("read_recorded", "read"), ("findings", "case"), ("json_findings", "items")} <= set(zeros)
    loops = {header: bound for header, bound, why, *_ in d["O10_LOOPS_LAST"] if "PF1B" in why}
    assert loops["for record in &records"] == "0" and loops["for entry in list"] == "l"
    assert [r for r in d["CG_RULES"] if r.get("added") == "PF1B-BN"] and all(
        r["targets"] == [] for r in d["CG_RULES"] if r.get("added") == "PF1B-BN")


def test_the_reader_returns_before_any_allocation_without_a_claim():
    """PF1B-FZ's premise: free_check's no-claim return precedes read() and every loop."""
    text = (SOURCE / RD).read_text(encoding="utf-8")
    body = text[text.index("fn free_check(source: &Value) -> Check {"):]
    body = body[:body.index("\n}\n")]
    at = body.index("    if !claimed {\n        return Ok(());\n    }\n")
    assert at < body.index("read(source)") and at < body.index("    for ")


@pytest.mark.parametrize("path, old, new", [
    # PF1B-PIN: either module's bytes.
    (DB, "        .filter(|c| declared(c))\n", "        .filter(|_| true)\n"),
    (RD, "    if !claimed {\n        return Ok(());\n    }\n", "    if false {\n        return Ok(());\n    }\n"),
])
def test_a_changed_module_stops(tmp_path, monkeypatch, path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit, match="PF1B-PIN"):
        decisions(monkeypatch, snap)
