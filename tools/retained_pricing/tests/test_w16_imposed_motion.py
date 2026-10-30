"""W16-NS, W16-ID, W16-NI and W16-VF (retained-pricing-rules.md §8.23): W-16 slice a's imposed_motion.rs is priced only
where its code exists. Behind a support target, which D1.7 refuses on both routes, its sites and edges are zero; the
node/element refusal (route L) and case_prescribed's node index are priced positively; a changed premise, refusal or
owner stops the run.

W-16 stage 2 (§8.29): validate_pressure_free's support branch calls per-target helpers, the case resolvers classify
through target_acting, a spring's moved base forms a ledger term only for a nonzero ground (W16S-SG) and the
relative-displacement rows' loop is empty on D1 (W16S-L1); each premise is guarded and mutated below."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
IM = "core/product_physics/src/imposed_motion.rs"
RM = "core/product_physics/src/retained_memory.rs"
LIB = "core/product_physics/src/lib.rs"
SUPPORT_REFUSAL = "crate::LoadTargetInput::Support { .. } => {\n                    return refuse(C::Loads, F::LoadTarget);"


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
def test_the_support_branch_is_zero_and_refuse_is_bounded_per_load(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    edges = {(short(a), short(b)) for a, b, why in d["O10_EDGE_ZERO"] if "W16-NS" in why}
    assert ("validate", "refuse") in edges and ("case_prescribed", "refuse") in edges
    assert {callee for caller, callee in edges if caller == "validate_pressure_free"} == {
        "parse_dof", "is_constant_effort_support", "acting", "is_spring_family", "dimension_valid",
        "validate_spring_base", "validate_rigid_base", "validate_ground_motion", "support_id", "find"}
    assert {callee for caller, callee in edges if caller == "target_acting"} == {
        "parse_dof", "is_constant_effort_support", "acting", "find"}
    assert ("case_prescribed", "target_acting") in edges and ("case_ground_motion", "record") in edges
    assert {callee for caller, callee in edges if caller == "case_spring_ground"} == {"refuse", "next"}
    assert "refuse" not in {callee for caller, callee in edges if caller == "validate_pressure_free"}
    per_call = [(short(a), short(b), per) for a, b, per, why in d["EDGE_PER_CALL"] if "W16-NS" in why]
    assert per_call == [("validate_pressure_free", "refuse", "cases*l")]
    zeros = [key for key, why in d["O10_SITE_ZERO"] if "W16-NS" in why]
    # validate's exact-branch text; validate_pressure_free's six support-branch texts; case_prescribed's text;
    # case_spring_ground's two refusals; case_ground_motion's Nonlinear arm; the case solve's early-return clone.
    assert len(zeros) == len(set(zeros)) == 12
    audits = {str(key).split("/src/")[-1]: value for key, value in d["O10_ID_AUDIT"]
              if str(key).split("/src/")[-1].startswith("imposed_motion.rs:")}
    ident = next(v for v in audits.values() if "exact_source_identity(&[&case.id,&load.id,code])" in v)
    # Two IN128 ids as `128:{id}` and the longest code, EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED (48 B).
    assert ident["exact_source_identity(&[&case.id,&load.id,code])"]["bytes"] == 2 * 132 + 3 + 48


@pytest.mark.parametrize("route", ["L", "E"])
def test_stage_2_spring_base_and_relative_rows_are_zero(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    sg = {(short(a), short(b)) for a, b, why in d["O10_EDGE_ZERO"] if "W16S-SG" in why}
    assert sg == {("spring_base_motion", n) for n in ("summand", "attach_last", "push_formed", "global_index")}
    l1 = [why for key, why in d["O10_SITE_ZERO"] if "W16S-L1" in why]
    assert len(l1) == 1
    loops = [r for r in d["O10_LOOPS_LAST"] if r[0] == "for (support_id, ground) in ground_motion"]
    assert [(r[1], r[3]) for r in loops] == [("0", ["lib.rs:append_nonlinear_relative_displacements"])]


def test_route_e_never_runs_the_pressure_free_branch(monkeypatch):
    e = {(short(a), short(b)) for a, b, why in decisions(monkeypatch, route="E")["EDGE_ZERO"] if "W16-NS (route E)" in why}
    assert e == {("validate", "validate_pressure_free")}
    assert not [why for *_, why in decisions(monkeypatch, route="L")["EDGE_ZERO"] if "W16-NS (route E)" in why]


@pytest.mark.parametrize("path, old, new", [
    # D1.7 no longer refuses the support target.
    (RM, SUPPORT_REFUSAL, "crate::LoadTargetInput::Support { .. } => {\n                    if false { return refuse(C::Loads, F::LoadTarget); }"),
    # The pressure-free refusal reaches support targets too.
    (IM, "            let LoadTargetInput::Support { support: support_id, dof } = &load.target else {\n                if load.category == CATEGORY {",
     "            if load.category == CATEGORY {\n                refuse(diagnostics, case, load, TARGET_INVALID, String::new(), None);\n            }\n"
     "            let LoadTargetInput::Support { support: support_id, dof } = &load.target else {\n                if load.category == CATEGORY {"),
    # Motion is claimed before the let-else.
    (IM, "        for load in &case.primitive_loads {\n            let LoadTargetInput::Support { support: support_id",
     "        for load in &case.primitive_loads {\n            motion = true;\n            let LoadTargetInput::Support { support: support_id"),
    # case_prescribed inserts for every load.
    (IM, "    let mut any = false;\n", "    let mut any = false;\n    prescribed.insert(0, 0.0);\n"),
    # The exact branch iterates every load.
    (IM, "for load in case.primitive_loads.iter().filter(|load| is_support_target(load)) {", "for load in case.primitive_loads.iter() {"),
    # Stage 2: target_acting classifies before its let-else.
    (IM, "    let LoadTargetInput::Support { support, dof } = &load.target else { return None };\n",
     "    let _ = model.supports.first().map(|record| acting(record, FrameDof::Ux));\n"
     "    let LoadTargetInput::Support { support, dof } = &load.target else { return None };\n"),
    # case_spring_ground refuses before its let-else.
    (IM, "        let Some((record, dof, Acting::Spring)) = target_acting(model, load) else { continue };",
     "        refuse(diagnostics, case, load, SUPPORT_UNKNOWN, String::new(), None);\n"
     "        let Some((record, dof, Acting::Spring)) = target_acting(model, load) else { continue };"),
    # A per-target helper runs before validate_pressure_free's let-else.
    (IM, "            let LoadTargetInput::Support { support: support_id, dof } = &load.target else {\n                if load.category == CATEGORY {",
     "            dimension_valid(case, load, FrameDof::Ux, \"\", diagnostics);\n"
     "            let LoadTargetInput::Support { support: support_id, dof } = &load.target else {\n                if load.category == CATEGORY {"),
])
def test_a_changed_premise_or_refusal_stops(tmp_path, monkeypatch, path, old, new):
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises(SystemExit, match="W16-NS"):
        decisions(monkeypatch, snapshot=snap)


@pytest.mark.parametrize("path, old, new, stop", [
    # spring_base_motion forms a term for an unmoved base.
    ("core/product_physics/src/certified_loads.rs", "        if g == 0.0 {\n            continue;\n        }\n", "",
     "W1 reductions source/lifetime pin changed: core/product_physics/src/certified_loads.rs"),
    # The relative rows' flag is set outside the loop.
    (LIB, "    let mut appended = false;\n", "    let mut appended = false;\n    appended = true;\n",
     "KD5C4: B7a E14 full-file source changed"),
])
def test_a_changed_stage_2_premise_stops(tmp_path, monkeypatch, path, old, new, stop):
    """Both files are under full-file pins, which stop the run first; W16S's guard on the same text stands behind
    them (as W16-NS's does behind KD5T2 for the case solve's return)."""
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, snapshot=snap)


def test_a_changed_case_solve_return_stops(tmp_path, monkeypatch):
    """The early return no longer tied to case_prescribed's findings: KD5T2's body pin on the case solve stops the run
    first; W16-NS's guard on the same statement stands behind it."""
    snap = snapshot_with(tmp_path, LIB, "    if has_blocking(&diagnostics[before..]) {\n", "    if has_blocking(&diagnostics) {\n")
    with pytest.raises(ValueError, match="KD5T2: source pin changed: core/product_physics/src/lib.rs:solve_prepared_load_case"):
        decisions(monkeypatch, snapshot=snap)


def test_owners_are_priced_where_the_module_exists(tmp_path):
    sys.path.insert(0, str(PRICING / "base"))
    try:
        model = runpy.run_path(str(PRICING / "base/w16_owners_model.py"))
    finally:
        sys.path.remove(str(PRICING / "base"))
    assert model["inspect"](str(SOURCE)) is True
    from g3lib import X
    assert model["node_index"](32) != X() and model["validation_findings"](384) != X()
    # An earlier revision without the module adds nothing.
    shutil.copytree(SOURCE / "core/product_physics/src", tmp_path / "core/product_physics/src",
                    ignore=shutil.ignore_patterns("target"))
    (tmp_path / IM).unlink()
    lib = (tmp_path / LIB).read_text(encoding="utf-8")
    (tmp_path / LIB).write_text(lib.replace("imposed_motion::", "imposed_motion_absent::"), encoding="utf-8")
    assert model["inspect"](str(tmp_path)) is False
    # The module without its pin, or a call without the module, stops.
    (tmp_path / IM).write_text((SOURCE / IM).read_text(encoding="utf-8") + "\n", encoding="utf-8")
    with pytest.raises(SystemExit, match="W16 owners"):
        model["inspect"](str(tmp_path))
    (tmp_path / IM).unlink()
    (tmp_path / LIB).write_text(lib, encoding="utf-8")
    with pytest.raises(SystemExit, match="W16 owners"):
        model["inspect"](str(tmp_path))


def test_a_changed_owning_statement_stops(tmp_path):
    sys.path.insert(0, str(PRICING / "base"))
    try:
        model = runpy.run_path(str(PRICING / "base/w16_owners_model.py"))
    finally:
        sys.path.remove(str(PRICING / "base"))
    snap = snapshot_with(tmp_path, LIB, "None => imposed_motion::case_prescribed(model, load_case, restrained_dofs, diagnostics),",
                         "None => { let _ = model.nodes.clone(); imposed_motion::case_prescribed(model, load_case, restrained_dofs, diagnostics) }")
    with pytest.raises(SystemExit, match="W16 owners"):
        model["inspect"](str(snap))
