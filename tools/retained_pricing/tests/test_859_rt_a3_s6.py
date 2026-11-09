"""RT-A3 S6 (retained-pricing-rules.md §8.59): nonlinear supports on load/reference-state documents and beside
objective connectors.

D1.5 refuses every analysis_state, D1.6 every nonlinear support and D1.4 every component, so the resolved-case motion
paths are unreached on D1. The rules:
- S6-ME prices the loop's consumed motion that solve_prepared_load_case builds as the capture's argument (one u64 per
  spring per case), bound to its exact statements;
- RTA3S6-L bounds the resolved spring-base loop and the moved-ground map at 0, guarded by D1.5 and the call shape;
- RTA3S6-VP replaces RTA3NL-S6's per-call charge: validate_profile no longer forms the S6 context, so its one
  carries_fields call is inside `case.equivalent_static.is_some()` and the edge is zero again (W16C-ES);
- W16-NS excludes CaseMotionEvidence::resolve (one caller, the modal state case) from its lib.rs call counts.
These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
LIB = "core/product_physics/src/lib.rs"
PR = "core/product_physics/src/pressure_runtime.rs"
STATE = "core/product_physics/src/modal/state.rs"
RM = "core/product_physics/src/retained_memory.rs"


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


def repinned_tool(tmp_path, original, mutated):
    """A scratch copy of the tool whose source pins of `original`'s file and of every function body the mutation
    changed are moved to the mutated bytes (the README's scratch re-pin), so a mutation reaches the S6 guards behind
    those pins instead of the pins themselves. The committed tool is unchanged."""
    import hashlib
    import re
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("tests", "__pycache__"))
    kd5 = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
    o4 = runpy.run_path(str(PRICING / "base/o4_owners_model.py"))
    pairs = {}
    for a, b in ((original, mutated), (original.split("#[cfg(test)]", 1)[0], mutated.split("#[cfg(test)]", 1)[0])):
        pairs[hashlib.sha256(a.encode()).hexdigest()] = hashlib.sha256(b.encode()).hexdigest()
    for name in set(re.findall(r"fn\s+([A-Za-z_]\w*)", original)):
        for extract in (lambda s, n: kd5["_indented_function"](s, n)[0], o4["_body"], kd5["function_body"]):
            try:
                a, b = extract(original, name), extract(mutated, name)
            except BaseException:
                continue
            if a and b and a != b:
                pairs[hashlib.sha256(a.encode()).hexdigest()] = hashlib.sha256(b.encode()).hexdigest()
    for path in tool.rglob("*.py"):
        text = path.read_text(encoding="utf-8")
        moved = text
        for old, new in pairs.items():
            moved = moved.replace(old, new)
        if moved != text:
            path.write_text(moved, encoding="utf-8")
    return tool


def w16_model():
    import sys
    sys.path[:0] = [str(PRICING / "base"), str(PRICING / "lib")]
    try:
        return runpy.run_path(str(PRICING / "base/w16_owners_model.py"))
    finally:
        del sys.path[:2]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_resolved_motion_loops_are_zero_on_d1(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {header: (bound, fns) for header, bound, why, fns in d["O10_LOOPS_LAST"] if "RTA3S6-L" in why}
    assert loops == {
        "for (support_id, dof, value) in &state.spring_base": ("0", ["lib.rs:load_state_spring_ground"]),
        "iter:ground_motion: ground.iter().map": ("0", ["lib.rs:new"]),
    }


@pytest.mark.parametrize("route", ["L", "E"])
def test_validate_profile_carries_fields_is_zero_again(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zero = [(str(a), str(b)) for a, b, *_ in d["O10_EDGE_ZERO"]
            if str(a).endswith(":validate_profile") and str(b).endswith(":carries_fields")]
    assert len(zero) == 1
    assert not any(str(a).endswith(":validate_profile") and str(b).endswith(":carries_fields")
                   for a, b, *_ in d["EDGE_PER_CALL"])


def test_the_case_motion_owner_is_one_u64_per_spring():
    m = w16_model()
    assert m["inspect"](str(SOURCE)) is True
    assert m["case_motion"](32) == m["B"](256)


@pytest.mark.parametrize("path,old,new", [
    # An unconditional carries_fields call before the case loop (the S6 context restored in another form).
    (PR, "        for support in &model.supports {\n            if let Some(family) = support_family(support) {",
     "        let _ = super::case_state::carries_fields(model);\n"
     "        for support in &model.supports {\n            if let Some(family) = support_family(support) {"),
    # The nonlinear arm no longer judged by the table.
    ("core/product_physics/src/exact_admission.rs",
     "(ExactContract::PressureV3, ExactFamily::NonlinearSupport) => Admission::Admitted,",
     "(ExactContract::PressureV3, ExactFamily::NonlinearSupport) => Admission::Refused { code: FAMILY_NOT_ADMITTED, message: \"x\" },"),
    # D1.5 no longer refuses an analysis_state.
    (RM, "        if !matches!(case.analysis_state, Authored::Absent) {\n            return refuse(C::Case, F::AnalysisState);",
     "        if false && !matches!(case.analysis_state, Authored::Absent) {\n            return refuse(C::Case, F::AnalysisState);"),
])
def test_a_changed_s6_premise_stops(monkeypatch, tmp_path, path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit):
        decisions(monkeypatch, snapshot=snap)


@pytest.mark.parametrize("old,new", [
    # A fourth vector or a changed collect in the argument stops the owner model.
    ("            spring_ground: springs.iter().map(|value| value.to_bits()).collect(),",
     "            spring_ground: springs.iter().chain(springs).map(|value| value.to_bits()).collect(),"),
    ("        CaseMotionEvidence::new(base_motion, &ground_motion, &spring_ground),",
     "        CaseMotionEvidence::new(base_motion, &ground_motion, &spring_ground.repeat(2)),"),
])
def test_a_changed_case_motion_owner_stops(tmp_path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, LIB, old, new)
    with pytest.raises(SystemExit):
        w16_model()["inspect"](str(snap))


@pytest.mark.parametrize("old,new,guard", [
    # A further call of the resolved spring-base helper: RTA3S6-L's call census.
    ("                &load_state_spring_ground(state, spring_entries, &load_case.id, &mut scratch),\n",
     "                &load_state_spring_ground(state, spring_entries, &load_case.id, &mut scratch),\n"
     "            ).also(load_state_spring_ground(state, spring_entries, &load_case.id, &mut scratch)),\n", "RTA3S6-L"),
    # A second caller of the motion restatement outside the modal state case: W16-NS's exclusion premise.
    ("    let mut moved = vec![false; springs.len()];\n    for (support_id, dof, value) in &state.spring_base {",
     "    let mut moved = vec![false; springs.len()];\n"
     "    if false { let _ = CaseMotionEvidence::resolve(todo!(), todo!(), None, &[], springs); }\n"
     "    for (support_id, dof, value) in &state.spring_base {", "W16-NS"),
])
def test_a_changed_s6_call_premise_stops_at_its_guard(monkeypatch, tmp_path, old, new, guard):
    """lib.rs is pinned whole and by body; these mutations are run against a scratch tool re-pinned to the mutated
    bytes, so the stop is the S6 guard's own, not an enclosing pin's."""
    snap = copy_core(tmp_path / "snap")
    original = (snap / LIB).read_text(encoding="utf-8")
    edit(snap, LIB, old, new)
    tool = repinned_tool(tmp_path, original, (snap / LIB).read_text(encoding="utf-8"))
    monkeypatch.setenv("SQ2_SNAP", str(snap))
    with pytest.raises(SystemExit, match=guard):
        runpy.run_path(str(tool / "rules/sq2_decisions_L.py"))
    # The unmutated snapshot passes through the same scratch tool.
    clean = copy_core(tmp_path / "clean")
    monkeypatch.setenv("SQ2_SNAP", str(clean))
    runpy.run_path(str(repinned_tool(tmp_path / "c", original, original) / "rules/sq2_decisions_L.py"))

