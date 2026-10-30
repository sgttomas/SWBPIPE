"""§8.31: nonlinear-only zero tails; the reached reader scans and gate Vec remain.

Mutations threaten the source/dataflow premises, rather than changing outputs to
match a price. The historical registration is exercised separately by replay.
"""
from pathlib import Path
import runpy
import shutil
import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/preview_physics.rs"
RE = "core/reporting/result_export/src/preview_combination_operands.rs"


def decisions(monkeypatch, source=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(source))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


@pytest.mark.parametrize("route", ["L", "E"])
def test_only_nonlinear_tails_are_zero(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    edges = [(str(a).rsplit(":", 1)[-1], str(b).rsplit(":", 1)[-1])
             for a, b, why in d["O10_EDGE_ZERO"] if "OP1A-PF:" in why]
    assert edges == [("run_linear_static_preview_observed_inner", "append_operand_standing"),
                     ("run_linear_static_preview_observed_inner", "state_operand_standing")]
    assert not any("preview_combination_operands.rs" in str(b) and str(b).endswith(":check")
                   for a, b, why in d["O10_EDGE_ZERO"])
    loops = {h: b for h, b, why, fns in d["O10_LOOPS_LAST"] if "OP1A-RD:" in why}
    assert loops["for (record, gate_id) in records.iter().zip(&admitted)"] == "0"
    assert loops["for notice in &expected"] == "0"
    owner = [p for p in d["O10_TYPED_PATCHES"] if "OP1A-OWN:" in p[3]]
    assert len(owner) == 1 and 's("&str")' in owner[0][2]
    assert 'pushcap(CAPS.get("z", 0)) // 2' in owner[0][2]
    # The current base identity reaches this reader through for_source.
    retained = (SOURCE / "core/reporting/result_export/src/retained_precision.rs").read_text()
    assert 'base_id: "openpipestress.result_semantics/0.3.0/preview-physics-2"' in retained


@pytest.mark.parametrize("path,old,new,stop", [
    (PP, "if !rendered.nonlinear {", "if false {", "W1 reductions source/lifetime pin changed"),
    (RE, 'let nonlinear = rows.iter().any(|r| r["kind"] == STATE_CODE);',
     'let nonlinear = true;', "OP1A-PF/RD: reviewed source changed"),
    (RE, 'let mut expected: Vec<Value> = Vec::new();',
     'let mut expected: Vec<Value> = vec![Value::Null];', "OP1A-PF/RD: reviewed source changed"),
    (RE, '.filter(|g| g["withheld"] == false)', '.filter(|_| true)', "OP1A-PF/RD: reviewed source changed"),
    (RE, 'fn text(v: &Value)', 'pub(crate) fn text(v: &Value)', "OP1A-PF/RD: reviewed source changed"),
    # OP1A-ER: swallowing the first error invalidates the one-returning-error cap.
    (RE, 'require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, "SHAPE")?;',
     'let _ = require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, "SHAPE");',
     "OP1A-PF/RD: reviewed source changed"),
    # OP1A-ID: a dynamic suffix no longer fits the reviewed literal class.
    (RE, 'require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, "SHAPE")?;',
     'require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, &"X".repeat(129))?;',
     "OP1A-PF/RD: reviewed source changed"),
])
def test_changed_premise_stops(tmp_path, monkeypatch, path, old, new, stop):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    p = tmp_path / path
    text = p.read_text()
    assert text.count(old) == 1
    p.write_text(text.replace(old, new))
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, tmp_path)
