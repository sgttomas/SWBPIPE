"""SD-NF, SD-L1, SD-ID, SD-EV, SD-WT, SD-DT, SD-RK and the SD owners (retained-pricing-rules.md §8.19): the silent-drop
guard's rules apply only where its code exists, zero the closed-record scan's finding path by its place before
admission, bound its loops and texts, re-key two replaced combination audit keys by name, and stop on a changed
source."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
FIELDS = "core/product_physics/src/model_fields.rs"
LIB = "core/product_physics/src/lib.rs"
PREVIEW = "core/product_physics/src/preview_physics.rs"
VALIDATION = "core/product_physics/src/validation.rs"
sys.path.insert(0, str(PRICING / "base"))
import sd_owners_model  # noqa: E402


def decisions(monkeypatch, snapshot=SOURCE, route="E"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def line_of(path, text):
    hits = [i + 1 for i, line in enumerate((SOURCE / path).read_text(encoding="utf-8").split("\n")) if text in line]
    assert len(hits) == 1, (text, hits)
    return hits[0]


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_scan_finding_path_is_zeroed_and_its_loops_bound(monkeypatch):
    d = decisions(monkeypatch)
    zeros = {str(k) for k, why in d["O10_SITE_ZERO"] if "SD-NF" in why}
    assert zeros == {f"model_fields.rs:{line_of(FIELDS, t)}" for t in (
        "findings.push(Finding { field: key.clone(), record: name(), path: path.to_string() });",
        '.map(|f| format!("unknown field `{}` in {} ({})", f.field, f.record, f.path))', '.join("; ");',
        'named_findings.push_str(&format!("; and {} more", findings.len() - NAMED_LIMIT));', "    Some(format!(")}
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "SD-L1" in why}
    assert loops == {
        'for (i, case) in objects(model, " ")': ("cases", ["model_fields.rs:unknown_field_refusal"]),
        'for (j, load) in objects(case, " ")': ("l", ["model_fields.rs:unknown_field_refusal"]),
        'for (i, combination) in objects(model, " ")': ("z", ["model_fields.rs:unknown_field_refusal"]),
        'for (j, term) in objects(combination, " ")': ("3", ["model_fields.rs:unknown_field_refusal"]),
        "for key in record.keys()": ("23", ["model_fields.rs:check"]),
    }
    edges = [(str(a), str(b)) for a, b, why in d["O10_EDGE_ZERO"] if "SD-EV" in why]
    assert len(edges) == 1 and edges[0][0].endswith(":check") and ":push" in edges[0][1] and "load_ledger.rs" in edges[0][1]


def test_the_names_paths_and_withheld_texts_are_audited(monkeypatch):
    d = decisions(monkeypatch, route="L")
    audit = {}
    for k, v in list(d["ID_AUDIT"]) + list(d["O10_ID_AUDIT"]):
        audit.setdefault(str(k), {}).update(v)
    load_name = audit[f"{FIELDS}:{line_of(FIELDS, 'let load_name = format!(')}"]
    assert load_name["case_name"]["bytes"] == 140 and load_name['named("primitiveload",id_of(load),j)']["bytes"] == 145
    assert audit[f"{FIELDS}:{line_of(FIELDS, '{load_path}.magnitude')}"]["load_name"]["bytes"] == 289
    support = audit[f"{PREVIEW}:{line_of(PREVIEW, 'Err(reason) => withheld.push(format!(')}"]
    assert support["reason"]["bytes"] == len("source support row missing in an operand case")
    named = audit[f"{PREVIEW}:{line_of(PREVIEW, 'withholds {} combined row(s) that could not be formed')}"]["named"]["bytes"]
    assert named == 8 * 1052 + 7 * 2 + len(", and  more") + 20
    # SD-RK: the two replaced basis keys resolve by name to the same sites.
    assert f"{PREVIEW}:{line_of(PREVIEW, 'rows.push(combination_row(combination, &expression, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, sources.iter()')}" in audit
    assert "core/product_physics/src/preview_physics.rs:990" not in audit and "core/product_physics/src/preview_physics.rs:1022" not in audit


def test_the_owners_are_present_and_positive():
    assert sd_owners_model.inspect(str(SOURCE))
    assert sd_owners_model.withheld_names(7785, 32) and sd_owners_model.seen_terms() and sd_owners_model.probe_error()


# lib.rs and preview_physics.rs are also under full-file pins, which stop first.
@pytest.mark.parametrize("path, old, new, stop", [
    (LIB, "    if let Some(refusal) = model_fields::unknown_field_refusal(&actual_request) {\n        return Err(refusal);\n    }\n", "", "SD-NF|KD5C4|pin changed"),
    (FIELDS, "    if findings.is_empty() {\n        return None;\n    }\n", "", "SD-NF"),
    (FIELDS, 'let case_path = format!("model.load_cases[{i}]");', 'let case_path = format!("model.load_cases.{i}");', "SD-ID"),
    (PREVIEW, "const NAMED: usize = 8;", "const NAMED: usize = 16;", "SD-WT|KD5C4|pin changed"),
])
def test_a_changed_guard_source_stops(monkeypatch, tmp_path, path, old, new, stop):
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, snapshot_with(tmp_path, path, old, new))


def test_a_changed_owner_stops(tmp_path):
    snap = snapshot_with(tmp_path, VALIDATION, "if !seen.insert(term.load_case.as_str()) {", "if !seen.insert(term.load_case.as_str()) && true {")
    with pytest.raises(SystemExit, match="SD owners"):
        sd_owners_model.inspect(str(snap))
