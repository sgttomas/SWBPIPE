"""DEL-04-09 I3a (retained-pricing-rules.md §8.52): the modal result document's format arguments (MODAL-ID).

No retained root reaches modal, so every document.rs and tests_document.rs format site is at multiplicity zero; the
audits classify the arguments no existing class covers and correct three by-name classes below the helpers' bound
(`b`, `id`: 20 and 128 B against a 135 B DOF id; `modal_doc_mode_ref(..)`: 20 B against a 25 B mode ref). The bounds
are read from document.rs's own helpers and constants; each mutation below re-prices or stops. These are source
warrants, not runtime allocation observations.
"""
from pathlib import Path
import re
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
DOC = "core/product_physics/src/modal/document.rs"
TDOC = "core/product_physics/src/modal/tests_document.rs"


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


def audits(d):
    """The I3a audits by (file, the site line's format text), each {arg: bytes}."""
    out = {}
    for key, value in d["O10_ID_AUDIT"]:
        key = str(key)
        if not key.startswith((DOC + ":", TDOC + ":")) or not any("MODAL-ID" in v["source"] for v in value.values()):
            continue
        path, line = key.rsplit(":", 1)
        text = (SOURCE / path).read_text(encoding="utf-8").split("\n")[int(line) - 1]
        out[(path.rsplit("/", 1)[-1], re.search(r'format!\("[^"]*"', text).group(0))] = {a: v["bytes"] for a, v in value.items()}
    return out


EXPECTED = {
    ("document.rs", 'format!("{prefix}:{ordinal}"'): {"ordinal": 20},
    ("document.rs", 'format!("modal-run:{}"'): {'run.get(..16).unwrap_or("")': 16},
    ("document.rs", 'format!("mass:{a}|{b}"'): {"a": 135, "b": 135},
    ("document.rs", 'format!("mass-total:{}"'): {"DIRECTIONS[k]": 1},
    ("document.rs", 'format!("shape:{}:{id}"'): {"modal_doc_mode_ref(mode.index)": 25, "id": 135},
    ("document.rs", 'format!("participation:{}:{}"'): {"modal_doc_mode_ref(mode.index)": 25, "DIRECTIONS[k]": 1},
    ("document.rs", 'format!("sturm-candidate:{}"'): {"j+1": 20},
    ("tests_document.rs", 'format!("/tables/mass_totals/rows/{k}/total_mass"'): {"k": 20},
    ("tests_document.rs", 'format!("/tables/modes/rows/{m}/generalized_mass"'): {"m": 20},
    ("tests_document.rs", 'format!("/tables/participation/rows/{}/{field}"'): {"3*m+k": 20},
}


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_document_audits_are_carried(monkeypatch, route):
    assert audits(decisions(monkeypatch, route=route)) == EXPECTED


def test_a_longer_component_token_reprices(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    edit(snap, DOC, 'const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];',
         'const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rzz"];')
    d = decisions(monkeypatch, snap)
    dof = [v for _, a in d["O10_ID_AUDIT"] for k, v in a.items() if k == "a" and "modal_doc_dof_id" in v["source"]]
    assert [v["bytes"] for v in dof] == [136]


@pytest.mark.parametrize("old, new", [
    # A computed token: the constant is no longer an all-literal [&str; n].
    ('const DIRECTIONS: [&str; 3] = ["x", "y", "z"];', 'const DIRECTIONS: [&str; 3] = ["x", "y", AXIS_Z];'),
    # The DOF id's or the mode ref's shape.
    ('    format!("dof:{}:{}", node_ids[dof / 6], COMPONENTS[dof % 6])\n',
     '    format!("dof:{}:{}:{}", node_ids[dof / 6], COMPONENTS[dof % 6], dof)\n'),
    ('    format!("mode:{index}")\n', '    format!("mode:{index}:{index}")\n'),
    # modal_doc_solved's bindings: the DOF id rebound to another value, or the node ids taken from elsewhere.
    ("            let id = modal_doc_dof_id(ids, dof);\n",
     "            let id = modal_doc_dof_id(ids, dof);\n            let id = format!(\"{id}{id}\");\n"),
    ("    let ids = &outcome.node_ids;\n", "    let ids = &outcome.node_ids.clone();\n"),
    # Review of ea68fc554: a rebinding after the first tuple element, and a closure rebinding.
    ("            let id = modal_doc_dof_id(ids, dof);\n",
     "            let id = modal_doc_dof_id(ids, dof);\n            let (_, id) = ((), id.repeat(2));\n"),
    ("            let id = modal_doc_dof_id(ids, dof);\n",
     "            let id = modal_doc_dof_id(ids, dof);\n            let id = (|id: String| id.repeat(2))(id);\n"),
])
def test_a_changed_helper_or_binding_stops(tmp_path, monkeypatch, old, new):
    snap = copy_core(tmp_path)
    edit(snap, DOC, old, new)
    with pytest.raises(SystemExit, match="MODAL-ID: document.rs's"):
        decisions(monkeypatch, snap)


def test_a_head_without_the_document_adds_nothing(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    edit(snap, DOC, "pub(crate) fn modal_result_document(", "pub(crate) fn modal_result_document_retired(")
    d = decisions(monkeypatch, snap)
    assert not [k for k, _ in d["O10_ID_AUDIT"] if str(k).startswith((DOC + ":", TDOC + ":"))]


def test_a_lengthening_edit_that_keeps_the_token_counts_stops(tmp_path, monkeypatch):
    # Narrow re-check of e98ad5add: token counts and the let list can be preserved while a value lengthens (e.g. a
    # compensated `for id in [..]` rebinding). Here `id` is doubled at its use with no a/b/id/ids token added, so only
    # the pinned text of modal_doc_solved can stop it.
    snap = copy_core(tmp_path)
    edit(snap, DOC, '"dof_ref": id', '"dof_ref": id.repeat(2)')
    with pytest.raises(SystemExit, match="modal_doc_solved changed"):
        decisions(monkeypatch, snap)
