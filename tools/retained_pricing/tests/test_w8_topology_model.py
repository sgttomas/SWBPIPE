"""W-8 topology warnings D3 (retained-pricing-rules.md §8.16).

W8-TW, W8-ND: the topology checks' transient working set and validation's node-degree map are priced positively in the
ordinary rows where W-8's source exists; a snapshot without it adds nothing, and a changed or partly present owner
stops."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
OWNERS = runpy.run_path(str(PRICING / "base/w8_topology_model.py"))
FILES = (OWNERS["TOP"], OWNERS["VAL"], OWNERS["LIB"], OWNERS["FG"])


def copy(tmp_path, rel=None, old=None, new=None, drop=None):
    for path in FILES:
        if path == drop:
            continue
        target = tmp_path / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / path, target)
    if rel:
        text = (tmp_path / rel).read_text()
        assert text.count(old) == 1
        (tmp_path / rel).write_text(text.replace(old, new, 1))
    return str(tmp_path)


def test_the_w8_owners_are_active_and_positive_at_the_caps():
    assert OWNERS["inspect"](str(SOURCE)) is True
    workspace = dict(OWNERS["topology_workspace"](32, 32, 32))
    # The pair samples hold <= NAMED (capacity 8, previous 4); the per-node found buffer <= n - 1 = 31
    # (capacity 32, previous 16). No quadratic pair term remains.
    assert workspace["s(Option<f64>)"] == 32 + 16
    assert workspace["s((usize,usize))"] < 1000
    assert all(v > 0 for v in workspace.values())
    assert dict(OWNERS["node_degrees"](32))["s((&str,usize))"] > 64


def test_a_snapshot_without_w8_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert OWNERS["inspect"](str(tmp_path)) is False


def test_a_partly_present_w8_stops(tmp_path):
    with pytest.raises(SystemExit, match="partly present"):
        OWNERS["inspect"](copy(tmp_path, drop=OWNERS["TOP"]))


@pytest.mark.parametrize("rel, old, new, match", [
    (OWNERS["TOP"], "let mut near = Vec::new();", "let mut near = Vec::with_capacity(1);", "topology.rs changed"),
    (OWNERS["VAL"], "*node_degree.entry(pipe.to.as_str()).or_default() += 1;",
     "*node_degree.entry(pipe.id.as_str()).or_default() += 1;", "validate_components"),
    (OWNERS["LIB"], "    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);\n",
     "    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);\n"
     "    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);\n", "not one site"),
])
def test_a_changed_owner_stops(tmp_path, rel, old, new, match):
    with pytest.raises(SystemExit, match=match):
        OWNERS["inspect"](copy(tmp_path, rel, old, new))
