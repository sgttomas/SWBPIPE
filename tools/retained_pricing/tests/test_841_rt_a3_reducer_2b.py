"""RT-A3 reducer 2b (§8.41, candidate): the W-15c pressure-step branch adds no owner on D1.

The new entries (pressure_step_run_evidence's Topology::of and axes, reducer_body_members' set over member_mappings)
are reached only behind D1-empty loops; W15's statements and RTA3R2B-NZ's premise bind that. Each mutation below
stops the run even though every full-file pin elsewhere is left as it is. These are source warrants, not runtime
allocation observations.
"""
from pathlib import Path
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/"
sys.path.insert(0, str(PRICING / "base"))
import w15_owners_model  # noqa: E402
import batch5_model  # noqa: E402


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, rel, old, new, count=1):
    path = snap / rel
    text = path.read_text(encoding="utf-8")
    assert text.count(old) >= 1, (rel, old)
    path.write_text(text.replace(old, new, count), encoding="utf-8")


def test_the_current_source_meets_the_premise():
    assert w15_owners_model.inspect(str(SOURCE))
    assert batch5_model.active(str(SOURCE))


CALL = "super::rigid_component::pressure_step_run_evidence(model, reducer.component)"
MEMBERS = "super::rigid_component::reducer_body_members(model)"


@pytest.mark.parametrize("mutation", [
    # The run-evidence call before the reducer loop: reached once per case on D1.
    ("pressure_runtime.rs", "    let mut refused = false;\n",
     "    let mut refused = false;\n    let _ = super::rigid_component::pressure_step_run_evidence(model, &model.components[0]);\n"),
    # The reducer loop renamed (another collection): the zero no longer follows from D1.4/D1.5.
    ("pressure_runtime.rs", "for reducer in exact_admission::reducers_in_regions(model, case) {",
     "for reducer in exact_admission::reducers_everywhere(model, case) {"),
    # The reducer-body set built before the region loop.
    ("exact_admission.rs", "    let mut reducer_bodies = None;\n",
     "    let mut reducer_bodies = Some(super::rigid_component::reducer_body_members(model));\n"),
    # A third Topology::of or member_mappings caller.
    ("rigid_component.rs", "pub(crate) fn reducer_body_members(model: &PreviewModel) -> HashSet<&str> {\n",
     "pub(crate) fn reducer_body_members(model: &PreviewModel) -> HashSet<&str> {\n    let _ = member_mappings(model);\n"),
])
def test_a_moved_or_added_pressure_step_entry_stops(tmp_path, mutation):
    rel, old, new = mutation
    snap = copy_core(tmp_path)
    edit(snap, PP + rel, old, new)
    if rel == "rigid_component.rs":
        # Restate the module pin so that only the statement/premise check can stop it.
        import hashlib
        w15_owners_model.PINS[w15_owners_model.RC], saved = (
            hashlib.sha256((snap / (PP + rel)).read_bytes()).hexdigest(), w15_owners_model.PINS[w15_owners_model.RC])
        try:
            with pytest.raises(SystemExit, match="W15 owners"):
                w15_owners_model.inspect(str(snap))
        finally:
            w15_owners_model.PINS[w15_owners_model.RC] = saved
    else:
        with pytest.raises(SystemExit, match="W15 owners"):
            w15_owners_model.inspect(str(snap))


def test_a_changed_pressure_prefix_stops(tmp_path):
    snap = copy_core(tmp_path)
    edit(snap, PP + "pressure_runtime.rs", "(body > 0.0 && neighbour < 0.0) || (body < 0.0 && neighbour > 0.0)",
         "(body >= 0.0 && neighbour <= 0.0) || (body <= 0.0 && neighbour >= 0.0)")
    with pytest.raises(ValueError, match="B5O1"):
        batch5_model.active(str(snap))


def test_the_scoped_problem_parents_follow_their_functions():
    import b7a_scoped_text_model as scoped
    text = (SOURCE / (PP + "pressure_runtime.rs")).read_text(encoding="utf-8").splitlines()
    # RT-A3 increment 4 (§8.49): both moved +11 after validate_profile's S6 context; RT-A3 S6 (§8.59) deletes the
    # context, so both move -10, with unchanged text.
    # RT-L2: both move +381 (the cancellation exception precedes them), unchanged text.
    # RT-L2 final source: both move +126 (the exception's restructured predicate and its probe), unchanged text.
    for key in ("2390:refuse_reducers_outside_step", "2467:collinear_run"):
        line, name = key.split(":")
        assert any(parent.endswith(key) for parent in scoped.PROBLEM_PARENTS), key
        assert text[int(line) - 1].lstrip().startswith(("fn " + name, "pub(crate) fn " + name)), key
