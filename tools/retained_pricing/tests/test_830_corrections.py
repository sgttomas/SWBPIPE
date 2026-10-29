"""C30-JR, C30-ID, C30-DG and W16T-SP (retained-pricing-rules.md §8.30): the three corrections' and W-16t's new code is
zero on D1 under guarded premises (D1.5's empty regions, D1.6's absent nonlinear supports, D1.5's refusal of
equivalent_static); NI's two Display forms stay priced. A changed premise stops the run."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PR = "core/product_physics/src/pressure_runtime.rs"
LIB = "core/product_physics/src/lib.rs"
SP = "core/solver/straight_pipe/src/lib.rs"
PL = "core/loads/primitive_loads/src/lib.rs"


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


def short(key):
    return str(key).split(":")[-1]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_new_code_is_zero_on_d1(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {(r[0], r[1]) for r in d["O10_LOOPS_LAST"] if "C30-JR" in r[2] or "W16T-SP" in r[2]}
    assert loops == {("for (node, (ending, compensated)) in at_node", "0"), ("for spanned in uniform_loads.iter()", "0")}
    edges = {(short(a), short(b)) for a, b, why in d["O10_EDGE_ZERO"] if "C30-DG" in why or "W16T-SP" in why}
    assert edges == {("run_linear_static_preview_observed_inner", "dense_loop_memory_guard"),
                     ("equivalent_global_nodal_loads_with_spans_formed", "timoshenko_partial_bounds")}
    # The refusal diagnostic's row, its message format and its refs clone.
    zeros = [key for key, why in d["O10_SITE_ZERO"] if "C30-DG" in why]
    assert len(zeros) == len(set(zeros)) == 3
    args = {r["re"]: r["max"] for r in d["O10_ARGS_FIRST"] if "C30-" in r["why"]}
    assert args[r'^(ending|compensated)\.join\(", "\)$'] == 128
    assert args[r"^(estimated|self\.ceiling_bytes)$"] == 39
    text = (SOURCE / "core/product_physics/src/exact_admission.rs").read_text(encoding="utf-8")
    literal = text.split("SHARED_TERMINAL_JUNCTION_TEXT: &str =", 1)[1].split('"', 2)[1]
    assert args[r"^exact_admission::SHARED_TERMINAL_JUNCTION_TEXT$"] == len(literal.encode())


@pytest.mark.parametrize("path, old, new, stop", [
    # The junction map is filled outside the region loop.
    (PR, "    let mut at_node: BTreeMap<&str, (Vec<&str>, Vec<&str>)> = BTreeMap::new();\n",
     "    let mut at_node: BTreeMap<&str, (Vec<&str>, Vec<&str>)> = BTreeMap::new();\n    at_node.entry(case_id).or_default();\n",
     "C30-JR: the shared-terminal premise changed"),
    # The dense-memory guard runs without a nonlinear support.
    (LIB, "    if !built.nonlinear_supports.is_empty() {\n        if let Err(refusal) = dense_loop_memory_guard(",
     "    if true {\n        if let Err(refusal) = dense_loop_memory_guard(", "KD5C4: B7a E14 full-file source changed"),
    # The Timoshenko partial-span bound runs for every Timoshenko frame.
    (SP, "if self.section.beam_theory.is_timoshenko() && uniform_loads.iter().any(partial) {",
     "if self.section.beam_theory.is_timoshenko() {", "W1 reductions source/lifetime pin changed: core/solver/straight_pipe/src/lib.rs"),
    # A second span constructor in PP.
    (LIB, "        None => UniformLoadSpan::full(),\n", "        None => UniformLoadSpan::new(0.0, 1.0)?,\n", "KD5C4: B7a E14 full-file source changed"),
])
def test_a_changed_premise_stops(tmp_path, monkeypatch, path, old, new, stop):
    """PP lib.rs and SP lib.rs are under full-file pins, which stop the run first; C30-DG's and W16T-SP's guards on the
    same text stand behind them (as W16-NS's does behind KD5T2)."""
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, snapshot=snap)
