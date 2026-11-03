"""RT-U4 (order): connector_evidence walks the source's `results` array in place of the row-ID map's values. E3's Pall
row pass and U5B-L's Pall lookup follow the new headers; E2's zero for the retired `spans.keys()` loop is kept only on a
source that still has it (the coverage loop over `records` falls under the generic `record in records` rule)."""
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
    found = loops(decisions(monkeypatch))
    assert found["for row in published"] == ("Pall", ["connector_evidence.rs:validate"])
    assert found["iter:published.iter().find"] == ("Pall", ["connector_evidence.rs:validate_pressure"])
    assert not {"for row in rows.values()", "iter:rows.values().find", "for component in spans.keys()"} & set(found)


def test_a_source_with_the_map_scans_keeps_their_rules(tmp_path, monkeypatch):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / CE
    text = target.read_text(encoding="utf-8")
    assert text.count("for row in published {") == 1 and text.count("published.iter().find(") == 2
    target.write_text(text.replace("for row in published {", "for row in rows.values() {")
                      .replace("published.iter().find(", "rows.values().find("), encoding="utf-8")
    found = loops(decisions(monkeypatch, tmp_path))
    assert found["for row in rows.values()"] == ("Pall", ["connector_evidence.rs:validate"])
    assert found["iter:rows.values().find"] == ("Pall", ["connector_evidence.rs:validate_pressure"])
    assert found["for component in spans.keys()"] == ("0", ["connector_evidence.rs:validate"])
    assert "for row in published" not in found
