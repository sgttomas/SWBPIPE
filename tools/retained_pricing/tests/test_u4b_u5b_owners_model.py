"""T4-S5 D3 (retained-pricing-rules.md §8.8): the T4-U4b/U5b positive owners are read from the function bodies that
allocate them. The current source activates every owner; a snapshot without the functions adds nothing; a changed
body or a half-present owner stops the run rather than choosing a price."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/u4b_u5b_owners_model.py"))
FILES = sorted({rel for rel, _ in MODEL["BODY_PINS"]} | {MODEL["FKS"], MODEL["PREV"], MODEL["NI"]})
OWNERS = ("published_displacements", "station_quantities", "absorption", "not_covered", "readers", "not_qualified",
          "contribution_groups", "published_index")


def snapshot(tmp_path, file=None, old=None, new=None, drop=()):
    for relative in FILES:
        if relative in drop:
            continue
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    if file:
        target = tmp_path / file
        text = target.read_text()
        assert text.count(old) == 1, old
        target.write_text(text.replace(old, new))
    return tmp_path


def test_current_source_activates_every_owner():
    assert MODEL["inspect"](SOURCE) == {name: True for name in OWNERS}


def test_a_snapshot_without_the_owners_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert MODEL["inspect"](tmp_path) == {name: False for name in OWNERS}


@pytest.mark.parametrize("file,old,new", [
    (MODEL["LIB"], ".unwrap_or_else(|| displacements.to_vec())", ".unwrap_or_else(|| displacements.repeat(2))"),
    (MODEL["U4B"], "    let mut out = StationOutputs::default();\n",
     "    let mut out = StationOutputs::default();\n    out.directional_records.reserve(64);\n"),
    (MODEL["FKS"], "        let mut free = vec![false; n];\n", "        let mut free = vec![false; 2 * n];\n"),
    (MODEL["RP"], "let mut out_of_domain=self.adapter.reserve::<Option<f64>>(self.members.len())?;",
     "let mut out_of_domain=self.adapter.reserve::<Option<f64>>(2*self.members.len())?;"),
    (MODEL["EVID"], "        let mut map = HashMap::new();\n", "        let mut map = HashMap::with_capacity(rows.len() * 2);\n"),
    (MODEL["ROWS"], "            input_result_ids: inputs.iter().map(|r| r.id.clone()).collect(),\n",
     "            input_result_ids: inputs.iter().chain(inputs.iter()).map(|r| r.id.clone()).collect(),\n"),
    # U5B-CG (T4-S6): a third group push, or a cloned group vector, is not the reviewed owner.
    (MODEL["NI"], "        self.groups.push(self.contributions.len());\n",
     "        self.groups.push(self.contributions.len());\n        self.groups.push(0);\n"),
    (MODEL["NI"], "            contribution_groups: parts.groups,\n            absolute_roundoff: parts.store.absolute_roundoff,\n",
     "            contribution_groups: parts.groups.clone(),\n            absolute_roundoff: parts.store.absolute_roundoff,\n"),
])
def test_a_changed_allocating_body_stops(tmp_path, file, old, new):
    with pytest.raises(SystemExit):
        MODEL["inspect"](snapshot(tmp_path, file, old, new))


def test_a_half_present_owner_stops(tmp_path):
    # U4B-ST's two functions without member_bases: neither absent nor reviewed.
    text = (SOURCE / MODEL["U4B"]).read_text()
    body = MODEL["_body"](text, "member_bases")
    with pytest.raises(SystemExit):
        MODEL["inspect"](snapshot(tmp_path, MODEL["U4B"], body, ""))


def test_owner_forms_are_positive():
    from g3lib import ASSUMED
    for form in (MODEL["ordinary_published_displacements"](192), MODEL["ordinary_station_quantities"](),
                 MODEL["absorption_flags"](192), MODEL["producer_not_covered"](32),
                 MODEL["failed_manifest_additions"](32, 28_799), MODEL["receipt_not_qualified"](32)):
        assert form and all(coefficient > 0 for coefficient in form.values())
        assert form.ev(dict(ASSUMED)) > 0


def test_u4b_w1_grows_the_exact_clone_by_doubling():
    """T4-S6: the diagnostics clone has exactly d_env slots, so its first push doubles it (Rust's amortized growth)."""
    grown = MODEL["grown_from"]
    assert grown(28_799, 32) == 57_598 and grown(25_367, 32) == 50_734
    assert grown(3, 10) == 24 and grown(0, 5) == 8 and grown(7, 0) == 7
    from g3lib import ASSUMED
    atoms = dict(ASSUMED, **{"s(Diagnostic)": 144, "s(Option<f64>)": 16})
    old = 16 * 32 + 144 * (32_768 + 16_384)            # S5's pushcap(d_env + m) and half of it, at route E's D_env
    assert MODEL["failed_manifest_additions"](32, 28_799).ev(atoms) - old == 3_648_672 // 3
    assert MODEL["failed_manifest_additions"](0, 28_799).ev(atoms) == 0


def test_u5b_cg_charges_one_group_vector_per_evidence():
    assert MODEL["contribution_groups"](32, 32) == {"1": 512}
    assert MODEL["contribution_groups"](1, 0) == {"1": 32}
