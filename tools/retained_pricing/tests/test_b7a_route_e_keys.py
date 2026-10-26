"""Exact route-E retirement preimages and ordinary mapper kind controls."""
from copy import deepcopy
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "lib/b7a_retired_carry.py"))
OPS = runpy.run_path(str(PRICING / "lib/rule_ops.py"))


def decisions(monkeypatch):
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_E.py"))


def test_only_three_exact_deleted_allowances_and_all_edges_preserved(monkeypatch):
    d = decisions(monkeypatch)
    edges = deepcopy(d["EDGE_ZERO"])
    # The common allowances inherited before route E's: E13's, and B7b-K1's deleted callee.
    inherited = {key: d["KEYMAP_ALLOW"][key] for key in
                 ("lib.rs:13102:append_combination_results", "load_reference_source.rs:46:code")}
    before = dict(inherited)
    result = MODEL["route_e_keymap_allow"](SOURCE, edges, inherited)
    assert result == d["KEYMAP_ALLOW"]
    assert inherited == before and edges == d["EDGE_ZERO"]
    assert set(result) - set(inherited) == {edge[1] for _, edge in MODEL["ROUTE_E_RETIRED_EDGES"]}
    assert all(result[edge[1]][0] == "deleted" for _, edge in MODEL["ROUTE_E_RETIRED_EDGES"])


@pytest.mark.parametrize("change", ["key", "caller", "reason", "order", "missing", "duplicate", "overlap"])
def test_complete_preimage_order_and_allowance_drift_refuse(monkeypatch, change):
    d = decisions(monkeypatch)
    edges = deepcopy(d["EDGE_ZERO"])
    inherited = {}
    if change in ("key", "caller", "reason"):
        field = {"caller": 0, "key": 1, "reason": 2}[change]
        edge = list(edges[13])
        edge[field] += "_changed"
        edges[13] = tuple(edge)
    elif change == "order":
        edges[13], edges[14] = edges[14], edges[13]
    elif change == "missing":
        edges.pop(13)
    elif change == "duplicate":
        edges.append(edges[13])
    else:
        inherited[edges[13][1]] = ("deleted", "different allowance")
    with pytest.raises(ValueError):
        MODEL["route_e_keymap_allow"](SOURCE, edges, inherited)


@pytest.mark.parametrize("change", ["rows", "crlf", "exact", "ordinary", "finalize", "live_finalizer"])
def test_raw_source_and_restored_constructor_drift_refuse(tmp_path, monkeypatch, change):
    d = decisions(monkeypatch)
    for file in MODEL["SOURCE_FILES"]:
        target = tmp_path / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, target)
    file = MODEL["ROW_FILE"] if change == "rows" else "core/product_physics/src/source_receipt.rs"
    target = tmp_path / file
    raw = target.read_bytes()
    if change == "crlf":
        assert b"\r\n" not in raw
        raw = raw.replace(b"\n", b"\r\n")
    elif change in ("exact", "ordinary", "finalize"):
        raw += ("\n    pub(super) fn " + change + "() {}\n").encode()
    elif change == "live_finalizer":
        raw = raw.replace(b"fn finalize_composite(", b"fn changed_composite(", 1)
    else:
        raw += b"\n// current row owner changed\n"
    target.write_bytes(raw)
    with pytest.raises(ValueError):
        MODEL["route_e_keymap_allow"](tmp_path, d["EDGE_ZERO"], {})


@pytest.mark.parametrize("kind", ["deleted", "replaced", "fewer_definitions"])
def test_original_strict_mapper_accepts_only_exact_deleted_kind(monkeypatch, kind):
    d = decisions(monkeypatch)
    keys = {edge[1] for _, edge in MODEL["ROUTE_E_RETIRED_EDGES"]}

    class Mapper:
        unmapped = []

        def map_key(self, value, context):
            if value in keys:
                self.unmapped.append((context, value, kind))
            return value

    mapper = Mapper()
    mapper.unmapped = []
    tables = {"EDGE_ZERO": deepcopy(d["EDGE_ZERO"]), "KEYMAP_ALLOW": dict(d["KEYMAP_ALLOW"])}
    bad = OPS["carry_decision_keys"](tables, mapper)
    assert len(mapper.unmapped) == 3
    assert (bad == []) == (kind == "deleted")


def test_unknown_deleted_key_still_refuses(monkeypatch):
    d = decisions(monkeypatch)
    d["FN_ZERO"].append(("unknown.rs:1:owner", "not covered"))

    class Mapper:
        unmapped = []

        def map_key(self, value, context):
            if value == "unknown.rs:1:owner":
                self.unmapped.append((context, value, "deleted"))
            return value

    mapper = Mapper()
    mapper.unmapped = []
    bad = OPS["carry_decision_keys"](d, mapper)
    assert len(bad) == 1 and bad[0][1:] == ("unknown.rs:1:owner", "deleted")


def test_no_module_namespace_is_a_decision_table(monkeypatch):
    """Every upper-case name the decisions bind is carried as a decision table (rule_ops.carry_decision_keys). A module
    namespace held under one, such as the owner choices' (whose PREIMAGES keys are written at the snapshot), would
    have its snapshot keys carried as if written at the basis."""
    d = decisions(monkeypatch)
    namespaces = [k for k, v in d.items() if k.isupper() and isinstance(v, dict) and "__file__" in v]
    assert namespaces == []
