"""W7-RL, W7-RP, W7-NC, W7-QF, W7-RD, W7-L1 and W7-ID (§8.20): W-7's global end rows are
priced only where their code exists, raise each route's rows per member by twelve, charge their owners positively and
stop on a changed source."""
from tools.repository_history import source_spec as _source_spec
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
LIB = "core/product_physics/src/lib.rs"
RP = "core/product_physics/src/retained_product.rs"
sys.path.insert(0, str(PRICING / "base"))
import w7_owners_model  # noqa: E402


def decisions(monkeypatch, snapshot=SOURCE, route="E"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_owners_are_present_and_positive():
    assert w7_owners_model.inspect(str(SOURCE))
    assert w7_owners_model.end_action_operands(32) == w7_owners_model.end_action_operands(1) * 32
    assert w7_owners_model.source_filter(2_755) and w7_owners_model.reader_coverage(96)


def test_each_route_prices_twelve_more_rows_per_member(monkeypatch):
    """W7-RL: route L's typed patches write 78 (FAILED_MEMBER_ROWS); W7-RP: route E's census adds 12 to its 102."""
    route_l = decisions(monkeypatch, route="L")
    assert any("78 * m + 8 * g + 3" in new for _file, _old, new, _why in route_l["O10_TYPED_PATCHES"] if "W7-RL" in _why)
    route_e = decisions(monkeypatch, route="E")
    assert route_e["CENSUS"]["global_end_rows_per_member"] == 12


def test_the_loops_and_texts_are_bound_in_their_own_functions(monkeypatch):
    d = decisions(monkeypatch, route="L")
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "W7-L1" in why}
    assert loops["for row in results.get(from..).unwrap_or(&[])"] == ("102", ["lib.rs:append_element_global_end_results"])
    assert loops["for key in global.keys()"] == ("cases*m", ["global_end_evidence.rs:coverage"])
    assert sum(1 for _key, audit in d["O10_ID_AUDIT"] if "W7-ID" in str(audit)) >= 4


def test_a_source_without_w7_adds_nothing(tmp_path):
    """The silent-drop guard's registered source (main 2653f87392) has none of W-7's statements."""
    import subprocess
    for path in (LIB, RP):
        text = subprocess.run(["git", "show", _source_spec(Path(__file__).resolve().parents[3], "2653f8739262251d9fa2cc339f03422f8fea9052", path)],
                              cwd=SOURCE, check=True, capture_output=True, text=True).stdout
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / path).write_text(text, encoding="utf-8")
    assert not w7_owners_model.inspect(str(tmp_path))


def test_a_partly_present_source_stops(tmp_path):
    snap = snapshot_with(tmp_path, LIB, "fn append_element_global_end_results(", "fn append_element_global_end_rows_removed(")
    with pytest.raises(SystemExit, match="W7 owners"):
        w7_owners_model.inspect(str(snap))


@pytest.mark.parametrize("path, old, new", [
    (RP, "let mut end_actions=self.adapter.reserve::<[[Option<usize>;6];2]>(self.members.len())?;",
     "let mut end_actions=self.adapter.reserve::<[[Option<usize>;6];2]>(2*self.members.len())?;"),
    (LIB, "let rows: Vec<&ResultItem> = rows.iter().filter(|row| !is_global_end_kind(&row.kind)).collect();",
     "let rows: Vec<&ResultItem> = rows.iter().collect();"),
    (LIB, "        let member_rows_from = results.len();\n", "        let member_rows_from = 0;\n"),
], ids=["end-action-table", "source-filter", "row-window"])
def test_a_changed_owner_stops(tmp_path, path, old, new):
    with pytest.raises(SystemExit, match="W7 owners"):
        w7_owners_model.inspect(str(snapshot_with(tmp_path, path, old, new)))


# RT-U4 (order): W7-RD's member HashSet is charged only while `coverage` collects it.
GE = "core/reporting/result_export/src/global_end_evidence.rs"
WALK = """    let member = |ends: &[[Option<f64>; 6]; 2]| ends.iter().all(|e| e[1].is_some() && e[2].is_some());
    for key in global.keys() {
        require(local.get(key).is_some_and(member), "MEMBER_UNRESOLVED")?;
    }
"""
HASHED = """    let members: HashSet<Key<'_>> = local
        .iter()
        .filter(|(_, ends)| ends.iter().all(|e| e[1].is_some() && e[2].is_some()))
        .map(|(key, _)| *key)
        .collect();
    for key in global.keys() {
        require(members.contains(key), "MEMBER_UNRESOLVED")?;
    }
"""


def test_the_ordered_walk_drops_only_the_member_set(tmp_path):
    sys.path.insert(0, str(PRICING / "base"))
    from g3lib import HashMov
    assert w7_owners_model.reader_member_set(str(SOURCE)) is False
    assert w7_owners_model.reader_coverage(96, True) == w7_owners_model.reader_coverage(96, False) + HashMov("PairBorrowed", 96)
    snapshot = snapshot_with(tmp_path, GE, WALK, HASHED)
    target = snapshot / GE
    text = target.read_text(encoding="utf-8")
    old = "    for (key, locals) in local.iter().filter(|(_, ends)| member(ends)) {\n"
    assert text.count(old) == 1
    target.write_text(text.replace(old, "    for key in &members {\n        let locals = &local[key];\n"), encoding="utf-8")
    assert w7_owners_model.reader_member_set(str(snapshot)) is True


def test_a_changed_member_walk_stops(tmp_path):
    old = "for (key, locals) in local.iter().filter(|(_, ends)| member(ends)) {"
    snapshot = snapshot_with(tmp_path, GE, old, old.replace("local.iter()", "local.iter().rev()"))
    with pytest.raises(SystemExit, match="W7 owners"):
        w7_owners_model.reader_member_set(str(snapshot))
