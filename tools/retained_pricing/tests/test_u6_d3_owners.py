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
SBK = "core/reporting/result_export/src/source_blocks.rs"
CJ = "core/serialization/canonical_json/src/lib.rs"
# The U6-DH/U6-SH borrowed-wrapper forms that preceded S6-HS (T4-S6), as the model reads them.
FORMER_DH = ("pub(super) fn domain_hash(domain: &str, payload: &Value) -> Option<String> {\n"
             "    let text = domain_text(domain, payload)?;\n"
             "    canonical_json_checked_v1_text(&text).ok().map(|canon| sha_hex(canon.as_bytes()))\n}\n"
             "pub(super) fn domain_text(domain: &str, payload: &Value) -> Option<String> {\n"
             "    serde_json::to_string(&DomainPayload { domain, payload }).ok()\n}\n"
             "#[derive(serde::Serialize)]\nstruct DomainPayload<'a> {\n    domain: &'a str,\n    payload: &'a Value,\n}\n")
FORMER_SH = ("fn hash(domain: &'static str, payload: &Value) -> Result<String, ReceiptError> {\n"
             "    let text = hash_text(domain, payload)?;\n"
             "    Ok(format!(\"{:x}\", Sha256::digest(text.as_bytes())))\n}\n"
             "fn hash_text(domain: &str, payload: &Value) -> Result<String, ReceiptError> {\n"
             "    let text = serde_json::to_string(&DomainPayload { domain, payload }).map_err(|e| bad(e.to_string()))?;\n"
             "    canonical_json_checked_v1_text(&text).map_err(bad)\n}\n"
             "#[derive(Serialize)]\nstruct DomainPayload<'a> {\n    domain: &'a str,\n    payload: &'a Value,\n}\n")
IMPORT_NOW = "use open_pipe_stress_canonical_json::{checked_v1_domain_canonical, ByteSink, DomainPayload};\n"
IMPORT_SR_NOW = ("use open_pipe_stress_canonical_json::{canonical_json_checked_v1_text, checked_v1_domain_canonical, ByteSink, "
                 "DomainPayload};\n")
IMPORT_FORMER = "use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;\n"
HASHES = [
    ("domain_hash_copies", "domain_hash_streams", RW, MODEL["STREAM_DH"], FORMER_DH,
     "    let text = domain_text(domain, payload)?;\n",
     '    let text = serde_json::to_string(&json!({"domain":domain,"payload":payload})).ok()?;\n'),
    ("receipt_hash_copies", "receipt_hash_streams", SR, MODEL["STREAM_SH"], FORMER_SH,
     "    let text = hash_text(domain, payload)?;\n",
     '    let text = checked(&json!({"domain":domain,"payload":payload}))?;\n'),
]


def test_current_hashes_stream():
    """S6-HS: every priced publication-hash family streams its digest from the borrowed payload."""
    assert MODEL["domain_hash_streams"](SOURCE, (SOURCE / RW).read_text()) is True
    assert MODEL["receipt_hash_streams"](SOURCE, (SOURCE / SR).read_text()) is True
    assert MODEL["reader_hash_streams"](SOURCE) == (True, True)


@pytest.mark.parametrize("copies,streams,file,streamed,former,borrow,copy", HASHES)
def test_former_forms_keep_their_prices_and_any_other_form_stops(copies, streams, file, streamed, former, borrow, copy):
    text = (SOURCE / file).read_text()
    assert text.count(streamed) == 1
    old = text.replace(streamed, former).replace(IMPORT_NOW, IMPORT_FORMER).replace(IMPORT_SR_NOW, IMPORT_FORMER)
    assert MODEL[streams](SOURCE, old) is False and MODEL[copies](old) is False
    assert MODEL[streams](SOURCE, old.replace(borrow, copy)) is False and MODEL[copies](old.replace(borrow, copy)) is True
    with pytest.raises(SystemExit):
        MODEL[streams](SOURCE, old.replace(borrow, borrow.replace("payload)?", "&payload.clone())?")))
    with pytest.raises(SystemExit):
        MODEL[copies](old + '\nfn other() { let _ = DomainPayload { domain: "", payload: &Value::Null }; }\n')
    # A streamed form beside a former wrapper, or a changed stream, stops.
    with pytest.raises(SystemExit):
        MODEL[streams](SOURCE, text + "\n#[derive(Serialize)]\nstruct DomainPayload<'a> {\n    domain: &'a str,\n}\n")
    with pytest.raises(SystemExit):
        MODEL[streams](SOURCE, text.replace(streamed, streamed.replace("Sha256::new()", "Sha256::default()")))


def reader_snapshot(tmp_path, edits=()):
    for relative in (SBK, MODEL["RPX"], CJ):
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    for relative, old, new in edits:
        text = (tmp_path / relative).read_text()
        assert text.count(old) == 1, old
        (tmp_path / relative).write_text(text.replace(old, new))
    return tmp_path


FORMER_RS = ("pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {\n"
             "    let value = json!({\"domain\":domain,\"payload\":payload});\n"
             "    let checked =\n        canonical_json_checked_v1_text(&serde_json::to_string(&value).map_err(|e| e.to_string())?)?;\n"
             "    Ok(format!(\"{:x}\", Sha256::digest(checked.as_bytes())))\n}\n")
STREAM_RS = "".join(MODEL["STREAM_RS"])
FORMER_G1 = ("        let mut public = source.clone();\n        public\n            .as_object_mut()\n"
             "            .ok_or_else(|| error(\"G1\", \"RECEIPT_MISMATCH\"))?\n            .remove(\"retained_precision\");\n"
             "        need(\n            hash(\n                \"retained_precision_publication_mp_v2\",\n                &public,\n")


def test_the_reader_hash_forms(tmp_path):
    rp = MODEL["RPX"]
    assert MODEL["reader_hash_streams"](reader_snapshot(tmp_path / "now")) == (True, True)
    sb_text = (SOURCE / SBK).read_text()
    streamed = sb_text[sb_text.index(MODEL["STREAM_RS"][0]):sb_text.index(MODEL["STREAM_RS"][2]) + len(MODEL["STREAM_RS"][2])]
    borrow = MODEL["BORROW_G1"]
    former_sb = (SBK, streamed, FORMER_RS)
    former_g1 = (rp, borrow, FORMER_G1)
    no_helper = (rp, MODEL["HASH_WITHOUT"], "")
    assert MODEL["reader_hash_streams"](reader_snapshot(tmp_path / "old", [former_sb, former_g1, no_helper])) == (False, False)
    assert MODEL["reader_hash_streams"](reader_snapshot(tmp_path / "half", [former_g1, no_helper])) == (True, False)
    with pytest.raises(SystemExit):     # a borrowed publication beside the former copying hash
        MODEL["reader_hash_streams"](reader_snapshot(tmp_path / "mixed", [former_sb]))
    with pytest.raises(SystemExit):     # a changed canonical writer
        MODEL["reader_hash_streams"](reader_snapshot(tmp_path / "cj", [(CJ, "Vec::with_capacity(map.len())", "Vec::new()")]))


def test_stream_route_holds_only_the_string_temporary_frames_and_key_vectors():
    sys_path = str(PRICING / "base")
    import sys
    sys.path.insert(0, sys_path)
    from g3lib import ASSUMED

    class VF:
        depth, ent = 6, 1000
    route = MODEL["stream_route"](VF, 1_000_000, 2)
    assert route.ev(dict(ASSUMED)) == 2 * (2 * 1_000_000 + 2) + 6 * 128 + 272 + 16 * 1000
    assert MODEL["stream_last_growth"](1_000_000, 2).ev(dict(ASSUMED)) == 2 * 1_000_000 + 2
    assert MODEL["stream_route"](VF, 10, 2).ev(dict(ASSUMED)) == 128 + 6 * 128 + 272 + 16 * 1000


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


SB = RE + "section_basis.rs"


def u6_be_block(root):
    """o10_rules.py's U6-BE block, as written, with its own fnkey/fn_lines/_fn_text, over a snapshot at `root`."""
    import ast
    import os
    import re
    module = ast.parse((PRICING / "rules/o10_rules.py").read_text())
    helpers = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name in ("fnkey", "fn_lines", "_fn_text")]
    assign = [node for node in module.body if isinstance(node, ast.Assign) and ast.unparse(node.targets[0]) == "_SBR_BE"]
    blocks = [node for node in module.body if isinstance(node, ast.If) and 'fnkey("bit_equal"' in ast.unparse(node.test).replace("'", '"')]
    assert len(helpers) == 3 and len(assign) == len(blocks) == 1
    namespace = {"_SNAP": str(root), "_os": os, "_re": re, "Resolved": Resolved, "RE_": RE}
    exec(compile(ast.Module(body=helpers + assign + blocks, type_ignores=[]), "o10 U6-BE", "exec"), namespace)
    return namespace.get("O10_RECURSION_REVIEWED")


def section_basis(tmp_path, old=None, new=None):
    target = tmp_path / SB
    target.parent.mkdir(parents=True, exist_ok=True)
    text = (SOURCE / SB).read_text()
    if old is not None:
        assert text.count(old) == 1
        text = text.replace(old, new)
    target.write_text(text)
    return tmp_path


def test_u6_be_reviews_bit_equals_recursion(tmp_path):
    """U6-BE: bit_equal's recursion is reviewed (its only text, number's static Err, counts once per external call)."""
    import re
    reviewed = u6_be_block(section_basis(tmp_path))
    assert len(reviewed) == 1 and re.fullmatch(r"section_basis\.rs:\d+:bit_equal", reviewed[0][0])
    assert reviewed[0][1].startswith("U6-BE:")


@pytest.mark.parametrize("old,new", [
    ("        _ => a == b,\n    }\n}\n", "        _ => a.to_string() == b.to_string(),\n    }\n}\n"),
    ('.ok_or_else(|| "SOURCE_PHYSICS_NUMBER_INVALID".into())', '.ok_or_else(|| format!("SOURCE_PHYSICS_NUMBER_INVALID {v}"))'),
])
def test_u6_be_a_changed_bit_equal_or_number_stops(tmp_path, old, new):
    with pytest.raises(SystemExit, match="U6-BE"):
        u6_be_block(section_basis(tmp_path, old, new))
