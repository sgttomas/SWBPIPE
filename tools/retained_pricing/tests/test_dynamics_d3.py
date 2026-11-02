"""DEL-04-09 dynamics D3: the separated formation leaves stay exact lifts with one retained instantiation.

frame_matrix and curved_matrix delegate to frame_matrix_with::<FrameSection> and
curved_matrix_with::<CurvedFormation>; the retained leaves are the exact lift of each binary64 field, so
the audited retained formation is unchanged and adds no owner. The supplementary guard checks the
listed lexical leaf/impl/caller/delegation forms; full-file pins also protect other spellings, such as
explicit generic callers. Restating a pin requires a fresh source audit, not reliance on this guard alone.
"""
import os
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
FC = "core/solver/frame_kernel/src/structural/formation_check.rs"


def current():
    return (SOURCE / FC).read_text(encoding="utf-8")


def test_current_retained_leaves_are_the_exact_lifts():
    assert MODEL["leaf_seam"](current()) is True


def test_the_domain_proof_records_the_seam(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    assert MODEL["inspect"](SOURCE)["formation_leaf_seam"] is True


def test_a_source_without_the_seam_takes_no_seam_premise():
    text = current()
    for token in ("trait FrameLeaves", "trait CurvedLeaves"):
        text = text.replace(token, "trait Removed")
    assert MODEL["leaf_seam"](text) is False


@pytest.mark.parametrize("old,new", [
    # a retained leaf that is no longer the exact lift of its own field
    ("        lift(self.area)\n    }\n    fn torsion_constant", "        lift(self.area * 2.0)\n    }\n    fn torsion_constant"),
    ("        lift(self.second_moment_z)\n", "        lift(self.second_moment_y)\n"),
    ("            BeamTheory::Timoshenko { shear_area } => Some(lift(shear_area)),\n",
     "            BeamTheory::Timoshenko { shear_area } => Some(lift(shear_area * 2.0)),\n"),
    ("        lift(self.out_of_plane_flexibility_factor)\n", "        lift(self.in_plane_flexibility_factor)\n"),
    # a delegation that supplies other leaves or geometry
    ("    frame_matrix_with(a, e.node_i.coordinates, e.node_j.coordinates, e.y_reference, &e.section)\n",
     "    frame_matrix_with(a, e.node_j.coordinates, e.node_i.coordinates, e.y_reference, &e.section)\n"),
    ("    curved_matrix_with(a, e, e)\n", "    curved_matrix_with(a, e, &e.clone())\n"),
])
def test_a_changed_retained_leaf_or_delegation_stops(old, new):
    text = current()
    assert text.count(old) == 1
    with pytest.raises(ValueError, match="leaf seam"):
        MODEL["leaf_seam"](text.replace(old, new))


@pytest.mark.parametrize("extra", [
    "\nstruct Other;\nimpl FrameLeaves for Other {}\n",
    "\nstruct Other;\nimpl CurvedLeaves for Other {}\n",
    "\nfn another(a: &mut WideArith, e: &FrameElement) {\n    let _ = frame_matrix_with(a, [0.0; 3], [0.0; 3], [0.0; 3], &e.section);\n}\n",
    "\nfn another(a: &mut WideArith, e: &CurvedFormation) {\n    let _ = curved_matrix_with(a, e, e);\n}\n",
])
def test_another_retained_impl_or_caller_stops(extra):
    with pytest.raises(ValueError, match="leaf seam"):
        MODEL["leaf_seam"](current() + extra)


def test_the_domain_guard_refuses_the_changed_seam_after_the_file_pin(tmp_path, monkeypatch):
    """The leaf guard is reached on the pinned file and stops on a changed one even when the pin is restated."""
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    for file in list(MODEL["SOURCE_FILES"]) + ["core/product_physics/src/retained_memory.rs"]:
        target = tmp_path / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, target)
    changed = current().replace("    curved_matrix_with(a, e, e)\n", "    curved_matrix_with(a, e, &e.clone())\n")
    (tmp_path / FC).write_text(changed, encoding="utf-8")
    with pytest.raises(ValueError, match="positive-domain owner source changed: " + FC):
        MODEL["inspect"](tmp_path)
    import hashlib
    restated = dict(MODEL["inspect"].__globals__["SOURCE_FILES"])
    restated[FC] = hashlib.sha256(changed.encode("utf-8")).hexdigest()
    monkeypatch.setitem(MODEL["inspect"].__globals__, "SOURCE_FILES", restated)
    with pytest.raises(ValueError, match="leaf seam"):
        MODEL["inspect"](tmp_path)
