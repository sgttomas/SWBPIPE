"""§8.63, MODAL-CS, continued: the reviewed absences, the unreached-function check, the forwarder body pins, the
driver's selection checks, the test-module ground and the boundary row predicates.

test_863_modal_completed_set.py holds the rules' summary and the shared helpers and constants. The tests of
§8.63 are in six files so that a hosted run, which keeps one file on one worker, spreads them over its workers.
"""
import json
import re
import subprocess

import pytest

from test_863_modal_completed_set import (
    MAIN, MAIN_RS_PIN, MQ, OUTCOME, PP, RB, RS, SKEW, SOURCE, check, copy_core, decisions, edit, o10_check_rules,
    point, sha, tool_with, tool_with_pin,
)


@pytest.mark.parametrize("path, old, new, name", [
    (OUTCOME, "run_modal_dense_within", "run_modal_dense_bounded", "outcome.rs:run_modal_dense_within"),
    (RS, "modal_completed_set_judge", "modal_completed_set_admit", "resource.rs:modal_completed_set_judge"),
    (RB, "pub fn lift_into(", "pub fn lift_into_exact(", "restraint_basis.rs:lift_into"),
])
def test_a_rename_that_keeps_the_boundary_stops_at_its_reviewed_absence(tmp_path, monkeypatch, path, old, new, name):
    snap = copy_core(tmp_path)
    target = snap / path
    target.write_text(target.read_text(encoding="utf-8").replace(old, new), encoding="utf-8")
    with pytest.raises(SystemExit, match=r"O10-GATE \(MODAL-CS, §8\.63\): .*" + name + " is absent"):
        decisions(monkeypatch, snap)


def main_source(tmp_path):
    """Main before the boundary (MAIN), archived from this repository's history; skipped where history lacks it."""
    if subprocess.run(["git", "-C", str(SOURCE), "cat-file", "-e", MAIN + "^{commit}"], capture_output=True).returncode:
        pytest.skip(f"{MAIN} is not in this clone's history")
    snap = tmp_path / "main"
    snap.mkdir()
    archive = subprocess.run(["git", "-C", str(SOURCE), "archive", MAIN, "core"], capture_output=True, check=True).stdout
    subprocess.run(["tar", "-x", "-C", str(snap)], input=archive, check=True)
    return snap


MAIN_LIB_PIN = "3f7e7e84f8cd9e8c0aa3019b430584e6e31d2bf73b744381b041af10f648cfb1"   # the tool's lib.rs binding at MAIN
LIB_PIN_FILES = ("base/b7a_domain_model.py", "base/kd5_text_model.py", "base/w1_reductions_model.py")


def main_tool(tmp_path):
    """The scratch tool for MAIN's source (the README's step-5 restoration): its resource.rs pin, and the whole-file
    binding of core/product_physics/src/lib.rs in each file that holds it, restored to MAIN's. A source later than
    MAIN may have moved that binding; MAIN's own lib.rs must still meet the value it was priced under."""
    tool = tool_with_pin(tmp_path, MAIN_RS_PIN)
    held = re.search(r"^B7A_PRIMITIVE_LIB_SOURCE_PIN = '([0-9a-f]{64})'$",
                     (tool / LIB_PIN_FILES[1]).read_text(encoding="utf-8"), re.M).group(1)
    for rel in LIB_PIN_FILES:
        text = (tool / rel).read_text(encoding="utf-8")
        assert text.count(held) == 1, (rel, held)
        (tool / rel).write_text(text.replace(held, MAIN_LIB_PIN), encoding="utf-8")
    return tool


def test_the_historical_absence_is_admitted_only_for_its_source_state(tmp_path, monkeypatch):
    snap = main_source(tmp_path)
    tool = main_tool(tmp_path)
    d = decisions(monkeypatch, snap, tool=tool)
    assert [(u["file"], u["name"], u["state"]) for u in d["O10_UNREACHED_FNS"]] == [(RB, "lift_into", "reviewed-absent")]
    # The same source with the function's text present (here only in a comment) is not that state.
    (snap / RB).write_text((snap / RB).read_text(encoding="utf-8") + "// lift_into\n", encoding="utf-8")
    with pytest.raises(SystemExit, match=r"O10-GATE \(MODAL-CS, §8\.63\): .*restraint_basis\.rs:lift_into is absent"):
        decisions(monkeypatch, snap, tool=tool)


# The node form: callgraph_g5 keys a function `file:line:name`. A real call `x.lift_into(..)`, resolved by name or by the
# receiver's known type, is an edge from the caller's node to lift_into's node; the governing run's graph holds exactly
# one such edge, core/product_physics/src/modal/skew_pencil.rs:51:modal_lift_within -> restraint_basis.rs:1265:lift_into.
ROOT, FN = PP + "lib.rs:1:root", RB + ":1265:lift_into"


OTHER = "core/solver/frame_kernel/src/structural.rs:10:solve"


UNRESOLVED = {"caller": OTHER, "line": "core/solver/frame_kernel/src/structural.rs:12", "name": "lift_into",
              "reason": "receiver typed Rigidity: no method of that name on it or a scanned trait", "context": "r.lift_into("}


@pytest.mark.parametrize("edges, graph, state, stops", [
    ({ROOT: [OTHER], OTHER: [], FN: []}, True, "present", None),                                   # unreached: passes
    # A retained caller's edge into it, real or a false same-name edge: either stops.
    ({ROOT: [OTHER], OTHER: [FN], FN: []}, True, "present", "reached from a retained root"),
    # A call naming it that the graph left unresolved (its audit row) stops even though no edge reaches it.
    ({ROOT: [OTHER], OTHER: [], FN: [], "audit": [UNRESOLVED]}, True, "present", "a call naming it produced no edge"),
    ({ROOT: [OTHER], OTHER: []}, True, "present", "missing from the graph"),
    ({ROOT: [OTHER], OTHER: [], FN: [], RB + ":1300:lift_into": []}, True, "present", "ambiguous in the graph"),
    ({ROOT: []}, True, "present", "the function's crate is not in the graph"),
    (None, False, "present", "the point's graph or roots are missing"),                             # a point not evaluated
    ({ROOT: [OTHER], OTHER: []}, True, "reviewed-absent", None),                                    # the historical state
    ({ROOT: [OTHER], OTHER: [], FN: []}, True, "reviewed-absent", "reviewed absent, but the graph has it"),
])
def test_the_unreached_function_check(tmp_path, edges, graph, state, stops):
    rules = o10_check_rules({"O10_UNREACHED_FNS": [{"file": RB, "name": "lift_into", "state": state, "why": "test"}]})
    good = point(tmp_path, "L_30", {ROOT: [OTHER], OTHER: [], **({FN: []} if state == "present" else {})})
    edges = dict(edges or {}) if graph else None
    audit = edges.pop("audit", ()) if graph else ()
    code, out = check(tmp_path, rules, [good, point(tmp_path, "L_21", edges, graph=graph, audit=audit)])
    assert code == (3 if stops else 0), out
    if stops:
        assert "O10 CHECK FAILED" in out and stops in out


def test_the_unreached_function_check_on_the_candidates_rules(tmp_path, monkeypatch):
    """The candidate's own emitted entry finds lift_into by file and name and stops on a retained caller's path."""
    rules = o10_check_rules(decisions(monkeypatch))
    spec = rules["o10_unreached_fns"]
    assert [(s["file"], s["name"], s["state"]) for s in spec] == [(RB, "lift_into", "present")]
    rules = {"o10_unreached_fns": spec}
    mlw = SKEW + ":51:modal_lift_within"
    today = {ROOT: [OTHER], OTHER: [], mlw: [FN], FN: []}           # the governing run's form: modal unreached
    assert check(tmp_path, rules, [point(tmp_path, "E_30", today)])[0] == 0
    # A retained function gaining the real call: its node's edge to lift_into's node, as the graph resolves it.
    code, out = check(tmp_path, rules, [point(tmp_path, "L_30", {**today, OTHER: [FN]})])
    assert code == 3 and "reached from a retained root" in out
    # Reaching modal_lift_within, whose real call reaches lift_into, stops the same way.
    code, out = check(tmp_path, rules, [point(tmp_path, "L_12", {**today, OTHER: [mlw]})])
    assert code == 3 and "reached from a retained root" in out


# ---- negative controls for the remaining stops (each through a scratch tool or snapshot copy)

FWD = [("modal_completed_set_rows#1", "94faf840e8c5256c5ef8bc2bf5838c6d0e31309642d5e596436240f79d85c9d8"),
       ("modal_shape_reserve#1", "9a2ac2cba339e4af78d90ec1df12993476014a7ddff67a9a9933182fe2f1020b"),
       ("modal_completed_set_text_room#1", "f550243ad823913d017b67b8131a39f03dc7dd5c0b862f1e276d8149e40600fc"),
       ("modal_completed_set_format#1", "e5b12b9ada9a70b11a5276fbf83fc17ed8c979483cfe8aaa1c08433a5bfba90d"),
       ("modal_completed_set_copy_text#1", "a15d88778468512f380594418c7b0f1d99ac546c699ed7bacf2c32d5d47a981f"),
       ("modal_shape_reserve#1", "31e2b71ebf7dc4b28d94866b8d20dd8f049292b9431bf06c1fdac0fabc240df5"),
       ("reserve_exactly#1", "1ec173384fccea2408263b0c7e9ab1227728adabe8489c5c4d3e5ed3a0ad2f2c"),
       ("requested_bytes#1", "b5fafc356977136df5f2771a5373e3cc1b9b43959f69e5fcb95daa12909e27ff")]


@pytest.mark.parametrize("encl, digest", FWD)
def test_each_structure_forwarders_body_pin_is_compared(tmp_path, monkeypatch, encl, digest):
    """Each of the eight forwarders' body pins is checked: a different reviewed digest stops at that function."""
    tool = tool_with_pin(tmp_path, "0" * 64, digest)
    with pytest.raises(SystemExit, match="MODAL-CS-ST: .*:" + re.escape(encl) + " changed"):
        decisions(monkeypatch, tool=tool)


def test_the_forwarder_set_is_exact(tmp_path, monkeypatch):
    line = '                (_MQ, "requested_bytes#1"): "b5fafc356977136df5f2771a5373e3cc1b9b43959f69e5fcb95daa12909e27ff",\n'
    tool = tool_with(tmp_path, {"rules/o10_rules.py": [(line, "")]})
    with pytest.raises(SystemExit, match="MODAL-CS-ST: the structure forwarders changed"):
        decisions(monkeypatch, tool=tool)


@pytest.mark.parametrize("rel, old, new, message", [
    ("price.py", "CORRECT_NORM = (", "CORRECT_NORM = CORRECT_NORM_AGAIN = (", "the driver's chain_crates or CORRECT_NORM is not unique"),
    ("base/crate_dirs.txt", " core/solver/frame_kernel/src", "", "the driver's crate selection no longer holds the frame kernel"),
])
def test_the_driver_selection_sanity_checks_stop(tmp_path, monkeypatch, rel, old, new, message):
    with pytest.raises(SystemExit, match="MODAL-CS: " + message):
        decisions(monkeypatch, tool=tool_with(tmp_path, {rel: [(old, new)]}))


def test_a_test_entry_outside_its_inline_test_module_stops(tmp_path, monkeypatch):
    """MODAL-CS-TM's containment: an inventoried modal_quantities.rs entry marked test code must lie in its tests module."""
    tool = tool_with(tmp_path, {})
    path = tool / "rules/modal_cs_inventory.json"
    inv = json.loads(path.read_text(encoding="utf-8"))
    entry = next(e for e in inv["storage"] if e[0] == MQ and e[4] == "production")
    entry[4] = "test"
    path.write_text(json.dumps(inv, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    with pytest.raises(SystemExit, match="MODAL-CS-TM: .* is marked test code but is not inside a reviewed test module"):
        decisions(monkeypatch, tool=tool)


MARKER = '// The completed-set boundary (module docs, "The completed-set boundary").'


@pytest.mark.parametrize("change, message", [
    ("marker", "MODAL-CS-M0: the boundary's marker is not unique"),
    ("added row", "MODAL-CS-M0: the boundary's TEXT rows changed"),
])
def test_the_boundary_row_predicates_stop(tmp_path, monkeypatch, change, message):
    """Past the whole-file pin: a second boundary marker, and an added TEXT row in the boundary (the aggregate count)."""
    snap = copy_core(tmp_path / "src")
    if change == "marker":
        (snap / RS).write_text((snap / RS).read_text(encoding="utf-8") + "\n" + MARKER + "\n", encoding="utf-8")
    else:
        edit(snap, RS, MARKER + "\n", MARKER + '\nfn completed_set_probe() -> String {\n    format!("probe")\n}\n')
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap, tool=tool_with_pin(tmp_path, sha(snap, RS)))


def test_two_dense_rows_on_one_line_stop(tmp_path, monkeypatch):
    """MODAL-CS-M0's distinct-line predicate: five dense rows, but two of them on one line and a cluster refusal gone."""
    snap = copy_core(tmp_path)
    edit(snap, OUTCOME, 'format!("the computed eigenvalues cannot be clustered ({error})")',
         'String::from("the computed eigenvalues cannot be clustered")')
    edit(snap, OUTCOME, 'format!("the cluster error estimates cannot be formed ({error:?})"), vec!["model".into()])]',
         'format!("the cluster error estimates cannot be formed ({error:?})"), vec![format!("model")])]')
    with pytest.raises(SystemExit, match="MODAL-CS-M0: the dense body's TEXT rows changed"):
        decisions(monkeypatch, snap)


def test_the_unreached_function_check_needs_points_and_roots(tmp_path):
    """Part A's two branches: no point at all, and a point whose roots are missing or empty."""
    rules = o10_check_rules({"O10_UNREACHED_FNS": [{"file": RB, "name": "lift_into", "state": "present", "why": "test"}]})
    code, out = check(tmp_path, rules, [])
    assert code == 3 and "no point to check" in out
    graph = {ROOT: [OTHER], OTHER: [], FN: []}
    code, out = check(tmp_path, rules, [point(tmp_path, "L_30", graph, roots=None)])
    assert code == 3 and "the point's graph or roots are missing" in out
    code, out = check(tmp_path, rules, [point(tmp_path, "L_21", graph, roots=())])
    assert code == 3 and "no roots, or the function's crate is not in the graph" in out
