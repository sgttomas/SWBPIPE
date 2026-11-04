"""RT-U4 (order): connector_evidence walks the source's `results` array in place of the row-ID map's values. E3's Pall
row pass and U5B-L's Pall lookup follow the new headers; E2's zero for the retired `spans.keys()` loop is kept only on a
source that still has it (the coverage loop over `records` falls under the generic `record in records` rule). The
ordered row pass's header, `for row in source_rows`, is its own: RT-A4's `for row in published` (P+8*g) is
preview_combination_gates::derived_rows', and a header rule matches by header alone."""
from pathlib import Path
import runpy
import shutil

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
CE = "core/reporting/result_export/src/connector_evidence.rs"


def decisions(monkeypatch, snapshot=SOURCE):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_E.py"))


def loops(d):
    return {h: (b, fns) for h, b, _why, fns in d["O10_LOOPS_LAST"] if any(f.startswith("connector_evidence.rs:") for f in fns)}


def test_the_ordered_scans_are_priced_as_the_map_scans_were(monkeypatch):
    d = decisions(monkeypatch)
    found = loops(d)
    assert found["for row in source_rows"] == ("Pall", ["connector_evidence.rs:validate"])
    assert found["iter:source_rows.iter().find"] == ("Pall", ["connector_evidence.rs:validate_pressure"])
    assert not {"for row in rows.values()", "iter:rows.values().find", "for component in spans.keys()"} & set(found)
    # The region binding's counting pass keeps the map's header under its own function.
    headers = [(h, b, fns) for h, b, _why, fns in d["O10_LOOPS_LAST"]]
    assert [x for x in headers if x[0] == "for row in rows.values()"] == [
        ("for row in rows.values()", "Pall", ["physics_evidence.rs:validate_physics_evidence_in"])]
    # Each re-keyed header is one rule's: no other function's rule shares it.
    for header in ("for row in source_rows", "iter:source_rows.iter().find"):
        assert [h for h, _b, _fns in headers].count(header) == 1, header
    assert [(b, fns) for h, b, fns in headers if h == "for row in published"] == [("P+8*g", ["preview_combination_gates.rs:derived_rows"])]


def test_a_source_with_the_map_scans_keeps_their_rules(tmp_path, monkeypatch):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / CE
    text = target.read_text(encoding="utf-8")
    assert text.count("for row in source_rows {") == 1 and text.count("source_rows.iter().find(") == 2
    target.write_text(text.replace("for row in source_rows {", "for row in rows.values() {")
                      .replace("source_rows.iter().find(", "rows.values().find("), encoding="utf-8")
    found = loops(decisions(monkeypatch, tmp_path))
    assert found["for row in rows.values()"] == ("Pall", ["connector_evidence.rs:validate", "physics_evidence.rs:validate_physics_evidence_in"])
    assert found["iter:rows.values().find"] == ("Pall", ["connector_evidence.rs:validate_pressure"])
    assert found["for component in spans.keys()"] == ("0", ["connector_evidence.rs:validate"])
    assert "for row in source_rows" not in found
