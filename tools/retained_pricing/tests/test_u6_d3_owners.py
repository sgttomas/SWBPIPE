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


RE = "core/reporting/result_export/src/"
TEST_MOD = '#[cfg(test)]\n#[path = "arc_marker_tests.rs"]\nmod arc_marker_tests;\n'


class Resolved(str):
    pass


def j5_block(root):
    """o10_rules.py's J5 block, as written, with its own fnkey/site, over a snapshot at `root`."""
    import ast
    import os
    import re
    module = ast.parse((PRICING / "rules/o10_rules.py").read_text())
    helpers = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name in ("fnkey", "site")]
    assign = [node for node in module.body if isinstance(node, ast.Assign) and ast.unparse(node.targets[0]) == "_PE"]
    blocks = [node for node in module.body if isinstance(node, ast.If) and "validate_physics_evidence_in" in ast.unparse(node.test)]
    assert len(helpers) == 2 and len(assign) == len(blocks) == 1
    namespace = {"_SNAP": str(root), "_os": os, "_re": re, "Resolved": Resolved, "RE_": RE, "O10": "O-10 (T4): ",
                 "B7B_EXACT_BASE": True, "O10_EDGE_ZERO": []}
    exec(compile(ast.Module(body=helpers + assign + blocks, type_ignores=[]), "o10 J5", "exec"), namespace)
    return namespace["O10_EDGE_ZERO"]


def result_export(tmp_path, old=None, new=None, extra=None):
    shutil.copytree(SOURCE / RE, tmp_path / RE)
    if old is not None:
        text = (tmp_path / RE / "physics_evidence.rs").read_text()
        assert text.count(old) == 1
        (tmp_path / RE / "physics_evidence.rs").write_text(text.replace(old, new))
    if extra:
        (tmp_path / RE / extra[0]).write_text(extra[1])
    return tmp_path


def test_j5_t_test_only_modules_are_not_product_callers(tmp_path):
    """J5-T: arc_marker_tests.rs's composite = true call is test code; the product inventory is unchanged."""
    root = result_export(tmp_path)
    assert "validate_physics_evidence_in(&doc, true" in (root / RE / "arc_marker_tests.rs").read_text()
    assert len(j5_block(root)) == 1


@pytest.mark.parametrize("old,new,extra", [
    (TEST_MOD, TEST_MOD.replace("#[cfg(test)]\n", ""), None),
    (TEST_MOD, TEST_MOD.replace("#[cfg(test)]", "#[allow(dead_code)]"), None),
    (None, None, ("lib_twin.rs", "mod arc_marker_tests;\n")),
])
def test_j5_t_a_product_inclusion_of_a_test_file_stops(tmp_path, old, new, extra):
    root = result_export(tmp_path, old, new, extra)
    with pytest.raises(SystemExit, match="J5"):
        j5_block(root)
