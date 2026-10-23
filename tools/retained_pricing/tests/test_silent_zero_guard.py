"""SZ-TX, SZ-ID and SZ-LS (retained-pricing-rules.md §8.15): the silent-zero guard's rules apply only where its code
exists, price its per-case validation diagnostic's identifiers, keep load-state resolution's findings guarded at
multiplicity 0, and stop on a changed diagnostic."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
VALIDATION = "core/product_physics/src/validation.rs"
RESOLVE = "core/product_physics/src/case_state/resolve.rs"
IN = {"bytes": 128, "source": "IN128"}


def decisions(monkeypatch, snapshot=SOURCE):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_E.py"))


def line_of(path, text):
    hits = [i + 1 for i, line in enumerate((SOURCE / path).read_text(encoding="utf-8").split("\n")) if text in line]
    assert len(hits) == 1, (text, hits)
    return hits[0]


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    for crate in ("core",):
        shutil.copytree(SOURCE / crate, tmp_path / crate, ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_current_source_audits_the_guard_ids_and_guards_load_state_at_zero(monkeypatch):
    d = decisions(monkeypatch)
    audit = dict((str(k), v) for k, v in d["O10_ID_AUDIT"])
    fmt = line_of(VALIDATION, '&format!("diagnostic:load-case:{}:applies-no-action", stable_suffix(&case.id)),')
    refs = line_of(VALIDATION, "vec![case.id.clone()],")
    assert audit[f"{VALIDATION}:{fmt}"] == {"stable_suffix(&case.id)": IN}
    assert audit[f"{VALIDATION}:{refs}"] == {"case.id": IN}
    mult0 = [str(k) for k, why in d["O10_MULT0_SITES"] if "SZ-LS" in why]
    excluded = line_of(RESOLVE, '"LOAD_STATE_SOURCE_EXCLUDED",')
    refusal = line_of(RESOLVE, "the load case applies no action: none of its")
    named = line_of(RESOLVE, "let mut text = ids.iter().take(crate::formation_guard::NAMED)")
    lines = (SOURCE / RESOLVE).read_text(encoding="utf-8").split("\n")
    assert "diag(" in lines[excluded - 3] and "format!(" in lines[refusal - 2]
    reference = line_of(RESOLVE, '"no load case applies an action: every case is a reference state')
    assert "diag(" in lines[reference - 5]
    assert mult0 == [f"case_state/resolve.rs:{excluded - 2}", f"case_state/resolve.rs:{refusal - 1}",
                     f"case_state/resolve.rs:{named}", f"case_state/resolve.rs:{reference - 4}"]


def test_a_changed_guard_diagnostic_stops(monkeypatch, tmp_path):
    snap = snapshot_with(tmp_path, VALIDATION, "vec![case.id.clone()],", "vec![case.id.clone(), case.id.clone()],")
    with pytest.raises(SystemExit, match="SZ-TX"):
        decisions(monkeypatch, snap)


def test_connector_preloads_edge_is_zeroed_by_p2(monkeypatch):
    """SZ-CP: model_loads_every_case's one call of connector_preloads is over model.components, empty by P2."""
    d = decisions(monkeypatch)
    edges = [(str(a), str(b)) for a, b, why in d["O10_EDGE_ZERO"] if "SZ-CP" in why]
    assert len(edges) == 1
    assert edges[0][0].endswith(":model_loads_every_case") and edges[0][1].endswith(":connector_preloads")


def test_a_changed_connector_preloads_call_stops(monkeypatch, tmp_path):
    snap = snapshot_with(tmp_path, VALIDATION, "model.components.iter().any(connector_preloads)",
                         "model.components.iter().chain(model.components.iter()).any(connector_preloads)")
    with pytest.raises(SystemExit, match="SZ-CP"):
        decisions(monkeypatch, snap)
