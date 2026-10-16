"""T4-U6/U8 D3: the hash-wrapper forms (U6-DH, U6-SH) and the D-5/D-6 owners (U6-ES, U6-WS, U6-RS, U8-FM) are
read from the source they price; any other form stops the run rather than choosing a price."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/u6_owners_model.py"))
RW = "core/product_physics/src/retained_wire.rs"
SR = "core/product_physics/src/source_receipt.rs"
HASHES = [
    ("domain_hash_copies", RW, "    let text = domain_text(domain, payload)?;\n",
     '    let text = serde_json::to_string(&json!({"domain":domain,"payload":payload})).ok()?;\n'),
    ("receipt_hash_copies", SR, "    let text = hash_text(domain, payload)?;\n",
     '    let text = checked(&json!({"domain":domain,"payload":payload}))?;\n'),
]


def test_current_hashes_serialize_borrowed_wrappers():
    assert MODEL["domain_hash_copies"]((SOURCE / RW).read_text()) is False
    assert MODEL["receipt_hash_copies"]((SOURCE / SR).read_text()) is False


@pytest.mark.parametrize("function,file,borrow,copy", HASHES)
def test_the_json_copy_is_priced_and_any_other_form_stops(function, file, borrow, copy):
    text = (SOURCE / file).read_text()
    assert text.count(borrow) == 1
    assert MODEL[function](text.replace(borrow, copy)) is True
    with pytest.raises(SystemExit):
        MODEL[function](text.replace(borrow, borrow.replace("payload)?", "&payload.clone())?")))
    with pytest.raises(SystemExit):
        MODEL[function](text + '\nfn other() { let _ = DomainPayload { domain: "", payload: &Value::Null }; }\n')


def snapshot(tmp_path, file=None, old=None, new=None):
    for relative in (MODEL["LIB"], MODEL["FC"], MODEL["RPX"]):
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    if file:
        text = (tmp_path / file).read_text()
        assert text.count(old) == 1
        (tmp_path / file).write_text(text.replace(old, new))
    return tmp_path


def test_current_source_has_every_owner(tmp_path):
    assert MODEL["inspect"](snapshot(tmp_path)) == {
        "exact_sections": True, "reduced_workspace": True, "frame_members": True, "all_rows": True}


@pytest.mark.parametrize("file,old,new", [
    (MODEL["LIB"], "        let mut stress_sites = vec![\n", "        let mut stress_sites = Vec::from([\n"),
    (MODEL["LIB"], "    exact_sections: HashMap<String, pressure_exact::ExactSectionBases>,\n",
     "    exact_sections: HashMap<String, pressure_exact::ExactSectionBases>,\n"
     "    exact_sections: HashMap<String, pressure_exact::ExactSectionBases>,\n"),
    (MODEL["LIB"], "members.push(member_prefix(&pipe.element_id))", "members.push(pipe.element_id.clone())"),
    (MODEL["RPX"], "last = Some(class.clone());", "last = Some(class.clone()); last = last.clone();"),
])
def test_a_changed_owner_source_stops(tmp_path, file, old, new):
    with pytest.raises((SystemExit, ValueError)):
        MODEL["inspect"](snapshot(tmp_path, file, old, new))
