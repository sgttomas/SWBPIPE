"""Exact route-E retirement preimages and ordinary mapper kind controls."""
from copy import deepcopy
from pathlib import Path
import re
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


# Mapper.map_key's key spelling (lib/linemap_lib.py), checked against its source below.
LINE_KEY = r"((?:core/[a-z_/]+/src/)?(?:[a-z_0-9]+/)*[a-z_0-9]+\.rs):(\d+)(?::([A-Za-z_][A-Za-z_0-9]*))?"


def consumed_tables():
    """Every upper-case name a driver in lib/ reads from the decisions, plus the O10_ families sq2_rules folds in."""
    names = set()
    for path in (PRICING / "lib").glob("*.py"):
        names |= set(re.findall(r'"([A-Z][A-Z0-9_]*)"', path.read_text()))
    return names | {"O10_" + f for f in ("EDGE_ZERO", "SITE_FROM", "SITE_ZERO", "ID_AUDIT", "ARGS_FIRST")}


def stray_line_keys(d, consumed):
    """Upper-case bindings no driver reads that still hold a carriable line key (not Resolved): the carry would map
    them as if written at the basis, though they are snapshot state (a model's inspect result, a namespace)."""
    def keys(v, seen):
        if id(v) in seen or type(v).__name__ == "Resolved":
            return []
        seen.add(id(v))
        if isinstance(v, str):
            return [v] if re.fullmatch(LINE_KEY, v) else []
        if isinstance(v, dict):
            return [k for kk, x in v.items() for k in keys(kk, seen) + keys(x, seen)]
        if isinstance(v, (list, tuple, set, frozenset)):
            return [k for x in v for k in keys(x, seen)]
        return []
    return {k: found for k in d if k.isupper() and k != "KEYMAP_ALLOW" and k not in consumed
            for found in [keys(d[k], set())] if found}


@pytest.mark.parametrize("route", ["L", "E"])
def test_no_model_state_is_bound_as_a_decision_table(monkeypatch, route):
    """The same for any snapshot state, whatever form binds it (assignment, dict(...), unpacking, a helper's return):
    in the evaluated decisions, no upper-case binding that no driver reads holds a carriable line key."""
    assert LINE_KEY in (PRICING / "lib/linemap_lib.py").read_text()
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    d = runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))
    consumed = consumed_tables()
    assert stray_line_keys(d, consumed) == {}
    # Control: C8's inspected state bound upper-case through dict(...) is found (round 4's escaping form).
    state = dict(d["_c8_model"]["inspect"](str(SOURCE))) if route == "E" else {"site": "core/product_physics/src/lib.rs:1"}
    assert list(stray_line_keys({"C8_MEMBER_STATE": state}, consumed)) == ["C8_MEMBER_STATE"]
