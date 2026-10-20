"""Actual W1 member-source owners and complete reader/search caps stay positive."""
from pathlib import Path
import os
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/w1_member_load_model.py"))


def test_complete_reader_and_search_inventory_fit_their_independent_caps():
    reader = MODEL["reader_inventory"]()
    assert reader["arithmetic_slots"] == 64
    assert reader["encoded_bytes"] == 56554
    assert reader["encoding_with_moving_and_stack"] == 333502
    assert reader["upper"] == 457728 < reader["cap"] == 1048576
    assert reader["encoding_with_moving_and_stack"] < reader["recompute_with_stack"]
    loaded = MODEL["loaded_inventory"]()
    assert loaded["backing"] == 702464
    # W1R-LM (W1 coverage repair, retained-pricing-rules.md §8.11): the driven search's four more endpoint slots.
    assert loaded["driven_slots"] == 4
    assert loaded["upper"] == 1022528 < loaded["cap"] == 1048576


def test_actual_member_cohort_has_all_authored_cases_and_no_preparation_credit():
    proof = MODEL["inspect"](SOURCE)
    assert proof["stage"] == 2 and proof["success_findings"] == "lc"
    assert len(proof["source_sha256"]) >= 20


def test_new_text_rules_are_positive_and_keep_fixed_array_bounds():
    loops, audits = MODEL["text_decisions"](SOURCE, str)
    assert len(loops) == 11
    assert all(bound in {"l", "3", "16"} and len(owners) == 1
               for _, bound, _, owners in loops)
    # O4 S5: CasePrep::new_with_shear's nodal arm repeats new_member's PrimitiveSource clone, audited alike.
    assert len(audits) == 5
    clone_audits = [a for k, a in audits if "structural/retained/adaptive.rs:" in k]
    assert clone_audits == [{"source.source()": {"bytes": 128, "source": "STATIC"}}] * 2
    key, audit = MODEL["case_clone_audit"](SOURCE)
    assert key.startswith("core/solver/frame_kernel/src/structural/retained/adaptive.rs:")
    assert audit == {"source": {"bytes": 128, "source": "STATIC"}}


def test_production_encoding_warrant_follows_guard_and_exact_occurrence_binding():
    # This ordering makes the tighter joint128/ID128 bound applicable to
    # native_bytes; its standalone1024/separate-count helper is a different
    # control. Full raw pins above bind the guard and both binding bodies.
    MODEL["inspect"](SOURCE)
    source = (SOURCE / "core/reporting/result_export/src/retained_precision.rs").read_text()
    g8 = source[source.index("fn g8("):source.index("struct MemberExpected")]
    guard = g8.index("guard_member_g8(source,inv,route.exact)?;")
    assert guard < g8.index('"source_blocks_invocation_v1"')
    assert guard < g8.index("verify_member_terms(s, case, pipes,")
    assert g8.index("verify_member_terms(s, case, pipes,") < g8.index("verify_native_source_hashes(s)?;")


def test_member_identifier_class_is_scoped_and_keeps_all_copy_work():
    proof = MODEL["inspect"](SOURCE)["member_identifier_warrant"]
    assert proof["class"] == 128
    assert proof["productive_calls"] == 3 and proof["test_calls"] == 1
    rules = MODEL["identifier_lex_sizes"](SOURCE, str)
    assert len(rules) == 1 and rules[0][1]["class"] == 128
    assert rules[0][0] == proof["site"].split("/src/")[-1]
    helpers = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
    source = (SOURCE / "core/product_physics/src/retained_product.rs").read_text()
    generic, line = helpers["_indented_function"](source, "copy")
    generic_site = line + next(i for i, row in enumerate(generic.splitlines()) if "out.push_str(s);" in row)
    # The generic2914B family and its nodal/diagnostic callers are untouched.
    assert rules[0][0] != "retained_product.rs:" + str(generic_site)


def identifier_snapshot(destination):
    domain = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
    files = set(MODEL["SOURCE_FILES"]) | set(domain["SOURCE_FILES"]) | {
        "core/product_physics/src/retained_memory.rs"}
    for relative in files:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    return destination


@pytest.mark.parametrize("change", ["helper", "capture_caller", "local_caller", "test_shadow",
                                   "text_cap", "member_parser", "nodal_parser"])
def test_member_identifier_and_parser_warrants_refuse_source_drift(tmp_path, change):
    root = identifier_snapshot(tmp_path)
    product = root / "core/product_physics/src/retained_product.rs"
    text = product.read_text()
    if change == "text_cap":
        target = root / "core/product_physics/src/retained_memory.rs"
        source = target.read_text()
        old = "const TEXT_BYTES: usize = 128;"
        assert source.count(old) == 1
        target.write_text(source.replace(old, "const TEXT_BYTES: usize = 256;"))
    else:
        if change == "helper":
            start = text.index("fn copy_member_id(")
            text = text[:start] + text[start:].replace("AdapterEvent::RequestedCopyBytes, n)",
                                                      "AdapterEvent::RequestedCopyBytes, n + 1)", 1)
        elif change == "capture_caller":
            text = text.replace(".copy_member_id(&load.load_id)", ".copy_member_id(&self.case_id)", 1)
        elif change == "local_caller":
            text = text.replace(".copy_member_id(&load.source_id)", ".copy(&load.source_id)", 1)
        elif change == "test_shadow":
            text = text.replace("member.copy_member_id(&value)", "member.copy(&value)", 1)
        elif change == "member_parser":
            text = text.replace('matches!((p.direction.as_str(),axis),("global_x",0)|("global_y",1)|("global_z",2))',
                                "parse_direction(&p.direction).is_ok_and(|d|d.dof_index()==axis)", 1)
        else:
            text = text.replace("parse_direction(&p.direction).is_ok_and(|d|d.dof_index()==l.global_dof%6)",
                                "true", 1)
        assert text != product.read_text()
        product.write_text(text)
    with pytest.raises(ValueError):
        MODEL["identifier_lex_sizes"](root, str)


@pytest.mark.parametrize("relative", list(MODEL["SOURCE_FILES"]))
def test_every_new_owner_and_complete_call_chain_pin_refuses_drift(tmp_path, relative):
    for path in MODEL["SOURCE_FILES"]:
        target = tmp_path / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / path, target)
    target = tmp_path / relative
    target.write_text(target.read_text() + "\n// unreviewed owner change\n")
    with pytest.raises(ValueError, match="source/owner pin"):
        MODEL["inspect"](tmp_path)


def test_member_owners_pay_full_dof_ledger_and_distinct_capture_clones(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    rows = MODEL["producer_additions"](32, 96, 128, 3, 128)
    full = rows["T12.13 complete declared ledger beside the nodal ledger"]
    # Full N is additional to the surviving nodal ledger; never an N-v credit.
    assert full["s(DeclaredMemberLoad)"] == 128
    assert full["s((usize,LedgerNet))"] == 192
    assert full["s(u64)"] == 128 * 192
    assert rows["T12.12 prepared/rebuilt operand member clone"]["s(UniformMemberLoad)"] == 128
    assert rows["T11.12 invocation member-only custody"]["s(UniformMemberLoad)"] == 3 * 128
    assert rows["T14.5 loaded search, evidence, PP spans and values completion"]["1"] >= 1048576


def test_reader_surcharge_is_separate_from_existing_source_identity_owners():
    source = (PRICING / "base/g4_caps.py").read_text()
    assert '"W1_MEMBER_READER"' in source
    assert "G8_VALUES" in source and "G8_BYTES" in source
    assert "2 * CC * _w1_member_model.READER_CASE_CAP" in source
