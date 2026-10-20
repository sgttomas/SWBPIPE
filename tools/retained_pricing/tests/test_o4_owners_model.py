"""O4 S5 D3 (retained-pricing-rules.md §8.9): O4's positive owners are read from the function bodies and spans that
allocate them. The current source activates them; a snapshot without the O4 functions adds nothing; a changed body,
a moved allocating statement or a half-present owner stops the run rather than choosing a price."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/o4_owners_model.py"))
FILES = sorted({rel for rel, _ in MODEL["BODY_PINS"]} | {MODEL["FKS"], MODEL["FKA"], MODEL["FKF"]})


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


def test_current_source_activates_the_owners():
    assert MODEL["inspect"](SOURCE) is True


def test_a_snapshot_without_the_owners_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert MODEL["inspect"](tmp_path) is False


@pytest.mark.parametrize("file,old,new", [
    # O4-PS / O4-PA: a doubled capture or preparation reservation is not the reviewed owner.
    (MODEL["RP"], "let mut leaves:Vec<k::TimoshenkoLeaf>=if timoshenko>0 {self.adapter.reserve(timoshenko)?} else {Vec::new()};",
     "let mut leaves:Vec<k::TimoshenkoLeaf>=if timoshenko>0 {self.adapter.reserve(2*timoshenko)?} else {Vec::new()};"),
    # O4-RS: a wider row-index reservation per reduced section.
    (MODEL["RP"], "let mut indices=self.adapter.reserve::<u32>(21)?;", "let mut indices=self.adapter.reserve::<u32>(42)?;"),
    # O4-NC: the bound-position bitmap at twice the rows.
    (MODEL["RP"], "reduced_bound=self.adapter.reserve(rows.len())?;reduced_bound.resize(rows.len(),false);",
     "reduced_bound=self.adapter.reserve(2*rows.len())?;reduced_bound.resize(rows.len(),false);"),
    # O4-KS: the sidecar's encoded block, a second CasePrep clone, a wider reduced map.
    (MODEL["FKS"], "            put_f64(out, leaf.shear_area);\n", "            put_f64(out, leaf.shear_area);\n            put_f64(out, leaf.shear_area);\n"),
    (MODEL["FKA"], "            prep.timoshenko = Some(set.clone());\n", "            prep.timoshenko = Some(set.clone().clone());\n"),
    (MODEL["FKF"], "    let mut map=reserve::<u32>(n)?;", "    let mut map=reserve::<u32>(2*n)?;"),
    # O4-WR: one more key in each reduced preparation record.
    (MODEL["RW"], '    json!({"result":result,"work":{"numeric":numeric(e,&w.numeric_trace(&mut copies)),',
     '    json!({"extra":result.clone(),"result":result,"work":{"numeric":numeric(e,&w.numeric_trace(&mut copies)),'),
    # O4-RD: the reader's Timoshenko leaves collected twice.
    (MODEL["RPX"], '            .filter_map(|st| st.get("shear_area").map(|a| (u(&st["member"]), a)))\n',
     '            .filter_map(|st| st.get("shear_area").map(|a| (u(&st["member"]), a)))\n            .chain(std::iter::empty())\n'),
    # O4-SP: three components per station.
    (MODEL["ROWS"], "            let mut components = Vec::with_capacity(2);\n", "            let mut components = Vec::with_capacity(3);\n"),
])
def test_a_changed_allocating_body_stops(tmp_path, file, old, new):
    with pytest.raises(SystemExit):
        MODEL["inspect"](snapshot(tmp_path, file, old, new))


def test_a_half_present_owner_stops(tmp_path):
    # The kernel sidecar span without the product's reduced_section_set: neither absent nor reviewed.
    text = (SOURCE / MODEL["RP"]).read_text()
    body = MODEL["_body"](text, "reduced_section_set")
    with pytest.raises(SystemExit):
        MODEL["inspect"](snapshot(tmp_path, MODEL["RP"], body, ""))


def test_owner_forms_are_positive():
    from g3lib import ASSUMED
    atoms = dict(ASSUMED, **{"s(NotCoveredOperands)": 400, "s(MaximumE)": 512, "s(PreparedMemberEvent)": 1024,
                             "s(ConversionEvent)": 64, "s(String)": 24, "s(&str)": 16, "Node(&str,())": 288})
    for form in (MODEL["capture_sidecars"](32, 128), MODEL["prepare_sidecars"](32, 40_000),
                 MODEL["proof_owners"](32, 3_747, 128), MODEL["trace_owners"](32),
                 MODEL["reader_sets"](32, 3), MODEL["derived_components"]()):
        assert form and all(coefficient > 0 for coefficient in form.values())
        assert form.ev(atoms) > 0


def test_identity_block_is_the_kernel_and_reader_framing():
    # tag `TSH\x01` + u32 count + (u32 member, u64 bits) per leaf, as FK encode_block and RS native_bytes write it.
    assert MODEL["identity_block"](32) == 4 + 4 + 12 * 32 == MODEL["reader_block_bytes"](32)
    text = (SOURCE / MODEL["FKS"]).read_text()
    assert 'pub const TIMOSHENKO_BLOCK_TAG: &[u8; 4] = b"TSH\\x01";' in text


def test_maximum_evidence_is_one_census_entry():
    assert MODEL["maximum_evidence_facts"](MODEL["CENSUS"]) == MODEL["MAXIMUM_EVIDENCE"]


def test_receipt_and_source_facts_scale_with_members():
    facts = MODEL["receipt_facts"](32)
    assert facts["class_entries"]["cnt"] == 21 * 32 and facts["preparation_members"] == 32
    assert MODEL["source_payload_facts"](32) == {"obj": 32, "ent": 192, "strb": 5 * 16 * 32}
    assert MODEL["beam_theory_evidence"](32) == {"ent": 64, "keyb": 25 * 32, "nums": 64}
