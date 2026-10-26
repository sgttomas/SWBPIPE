"""Closed formed-load premises: drift refusal, priority and positive owners."""
from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_formation_model.py"))
DOMAIN = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
RETIRED = runpy.run_path(str(PRICING / "lib/b7a_retired_carry.py"))


def source_copy(destination):
    for file in list(DOMAIN["SOURCE_FILES"]) + list(MODEL["SOURCE_FILES"]) + ["core/product_physics/src/retained_memory.rs"]:
        target = destination / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, target)
    return destination


def rules(tmp):
    inherited, _ = RETIRED["prepare"](SOURCE, PRICING / "base", tmp / "retired")
    widened, _ = DOMAIN["prepare"](SOURCE, PRICING / "base", tmp / "widened", inherited)
    inherited.update(widened)
    return inherited


def test_actual_guard_and_source_aggregate_bounds_are_positive(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    proof = MODEL["inspect"](SOURCE)
    assert len(proof["full_raw_source_fingerprints"]) == 21
    assert len(proof["formation_raw_fingerprints"]) == 6
    bound = MODEL["lengths"](32, 128, 128)
    assert bound == dict(N=192, q=1536, debug_total=1972608, fired_total=2149824,
                         reason=165057, fired=165659, joined=993964, sentence=994321)
    # Each source list has its own generation. Both may contain every term.
    assert 2 * bound["debug_total"] >= 2 * (1284 * bound["q"])
    capture = MODEL["finding_capture"](3, 32, 128, 128)
    assert capture["s(String)"] == 3 * 192
    assert capture["1"] == 3 * (bound["fired_total"] + bound["sentence"])
    construction = MODEL["finding_construction"](3, 32, 128, 128)
    assert construction["s(FormationRow)"] >= 3 * 192
    assert construction["s((usize,[f64;4]))"] > 0
    assert construction["s(String)"] > capture["s(String)"]


@pytest.mark.parametrize("file", list(MODEL["SOURCE_FILES"]))
def test_raw_formation_owners_crlf_and_missing_refuse(tmp_path, file):
    snapshot = source_copy(tmp_path)
    target = snapshot / file
    raw = target.read_bytes()
    assert b"\r\n" not in raw
    target.write_bytes(raw.replace(b"\n", b"\r\n"))
    with pytest.raises(ValueError, match="formation owner source changed"):
        MODEL["inspect"](snapshot)
    target.unlink()
    with pytest.raises(FileNotFoundError):
        MODEL["inspect"](snapshot)


@pytest.mark.parametrize("file,fragment", [
    ("core/solver/frame_kernel/src/load_ledger.rs", "let mut by_dof = vec![Vec::new(); size];"),
    ("core/product_physics/src/lib.rs", "finish_case_ledger(ledger, built.nodes.len())"),
    ("core/product_physics/src/formation_guard.rs", "for row in &rows"),
    ("core/product_physics/src/self_weight.rs", "if !recognized && !looks_generated"),
    ("core/product_physics/src/retained_memory.rs", "fn domain_clauses("),
    ("core/solver/frame_kernel/src/structural/formation_check.rs", "for p in source.loads.iter()"),
])
def test_constructor_census_guard_and_current_owner_drift_refuses(tmp_path, file, fragment):
    snapshot = source_copy(tmp_path)
    target = snapshot / file
    raw = target.read_bytes()
    needle = fragment.encode()
    assert needle in raw, (file, fragment)
    target.write_bytes(raw.replace(needle, needle + b"/* source changed */", 1))
    with pytest.raises(ValueError):
        MODEL["inspect"](snapshot)


def test_exact_copied_rule_transform_retains_every_unrelated_owner(tmp_path):
    inherited = rules(tmp_path)
    before = {name: Path(path).read_bytes() for name, path in inherited.items()}
    actual, report = MODEL["prepare"](SOURCE, inherited, tmp_path / "formed")
    old_loop = json.loads(before["loop_bounds.g4.json"])
    new_loop = json.loads(Path(actual["loop_bounds.g4.json"]).read_bytes())
    expected_loop = deepcopy(old_loop)
    expected_loop["loops"][256:257] = new_loop["loops"][256:258]
    for index, _, _, _, bound in MODEL["BASE_LOOP_PREIMAGES"]:
        current = index + (index > 256)
        expected_loop["loops"][current] = new_loop["loops"][current]
        assert expected_loop["loops"][current]["bound"] == bound
    for old, new in zip(expected_loop["fn_zero"], new_loop["fn_zero"]):
        if old["fn"].startswith("self_weight.rs:"):
            old["why"] = new["why"]
    assert new_loop == expected_loop
    assert len([r for r in new_loop["fn_zero"] if r["fn"].startswith("self_weight.rs:")]) == 17
    old_text = json.loads(before["text_args.g4.json"])
    new_text = json.loads(Path(actual["text_args.g4.json"]).read_bytes())
    expected_text = deepcopy(old_text)
    # RF-ID moves these two full owners to member_descriptors' independently
    # name-qualified sites; the unchanged member_ids clone remains carried.
    for key in ("core/product_physics/src/source_recovery.rs:1099",
                "core/product_physics/src/source_recovery.rs:1124"):
        assert expected_text["id_audit"].pop(key) == {"pipe.element_id": {"bytes": 128, "source": "IN128"}}
    for key in MODEL["ZERO_PREIMAGES"]:
        del expected_text["site_zero"][key]
    for key in MODEL["COPY_PREIMAGES"]:
        expected_text["site_total"][key] = new_text["site_total"][key]
    expected_text["site_from"]["structural.rs:1103"] = new_text["site_from"]["structural.rs:1103"]
    for key in MODEL["FIDELITY_PREIMAGES"]["aggregates"]:
        expected_text["site_aggregate"][key] = new_text["site_aggregate"][key]
    old_join = MODEL["FIDELITY_PREIMAGES"]["rows_join"]
    expected_text["join_rules"][expected_text["join_rules"].index(old_join)] = next(
        r for r in new_text["join_rules"] if r["re"] == "^rows$")
    expected_text["b7a_reformation_first_error"] = "FK895-v1"
    expected_text["b7a_owned_first_errors"] = "v1"
    assert new_text == expected_text
    assert {name: Path(path).read_bytes() for name, path in inherited.items()} == before
    assert all(hashlib.sha256(Path(actual[name]).read_bytes()).hexdigest() == digest
               for name, digest in report["prepared_files"].items())


@pytest.mark.parametrize("change", ["zero", "copy_bytes", "copy_reason", "loop_value", "loop_reason", "loop_order",
                                     "source_clone", "source_clone_caller", "fidelity_aggregate", "fidelity_join",
                                     "member_clone_removed", "member_clone_bytes"])
def test_exact_preimages_refuse_drift(tmp_path, change):
    inherited = rules(tmp_path)
    family = "loop_bounds.g4.json" if change.startswith("loop") else "text_args.g4.json"
    target = Path(inherited[family])
    data = json.loads(target.read_bytes())
    if change == "zero":
        data["site_zero"]["formation_guard.rs:315"] += " changed"
    elif change.startswith("copy"):
        field = "bytes" if change == "copy_bytes" else "why"
        data["site_total"]["load_ledger.rs:133"][field] += " changed"
    elif change == "source_clone":
        data["site_from"]["structural.rs:1103"]["per"] = "1*l"
    elif change == "source_clone_caller":
        data["site_from"]["structural.rs:1103"]["from"].pop()
    elif change == "fidelity_aggregate":
        data["site_aggregate"]["lib.rs:1273"]["bytes"] += "+1"
    elif change == "fidelity_join":
        next(r for r in data["join_rules"] if r["re"] == "^rows$")["bytes"] += "+1"
    elif change == "member_clone_removed":
        del data["id_audit"]["core/product_physics/src/source_recovery.rs:1099"]
    elif change == "member_clone_bytes":
        data["id_audit"]["core/product_physics/src/source_recovery.rs:1124"]["pipe.element_id"]["bytes"] = 1
    elif change == "loop_order":
        data["loops"][211], data["loops"][212] = data["loops"][212], data["loops"][211]
    else:
        data["loops"][211]["bound" if change == "loop_value" else "why"] += " changed"
    target.write_text(json.dumps(data))
    with pytest.raises(ValueError, match="preimage|zero changed|copy total"):
        MODEL["prepare"](SOURCE, inherited, tmp_path / "formed")


@pytest.mark.parametrize("change", ["missing", "duplicate", "value", "owner"])
def test_closed_o10_preimages_refuse_arbitrary_missing_or_changed_rule(change):
    rows = [(h, b, "O-10 (T4): " + why, list(owners))
            for h, b, why, owners in MODEL["O10_PREIMAGES"]]
    if change == "missing":
        rows.pop()
    elif change == "duplicate":
        rows.append(rows[0])
    elif change == "value":
        rows[0] = (rows[0][0], "1", rows[0][2], rows[0][3])
    else:
        rows[0] = (rows[0][0], rows[0][1], rows[0][2], ["foreign.rs:owner"])
    with pytest.raises(ValueError, match="exact formed O10 preimage"):
        MODEL["rebind_o10"](SOURCE, rows)


@pytest.mark.parametrize("route", ["L", "E"])
def test_real_prepared_rule_priority_keeps_q_and_function_qualified_n(tmp_path, route):
    inherited = rules(tmp_path)
    prepared, _ = MODEL["prepare"](SOURCE, inherited, tmp_path / "formed")
    inherited.update(prepared)
    prepared = inherited
    # Carry the actual base rules before SQ2, as the canonical driver does.
    # The source-local TEXT guards require current authoritative keys.
    repo = PRICING.parents[3]
    revision = os.environ.get("B7A_PHASE_TEST_REV")
    if SOURCE != PRICING.parents[1] and not revision:
        raise ValueError("archived test source requires B7A_PHASE_TEST_REV")
    target = subprocess.check_output(["git", "rev-parse", "--verify", (revision or "HEAD") + "^{commit}"],
                                     cwd=repo, text=True).strip()
    rule_files = ("callgraph_rules.g4.json", "loop_bounds.g4.json",
                  "text_args.g4.json", "sens.py", "premise_pins.json")
    mapped_rules = tmp_path / "map"
    environment = dict(os.environ,
        LINEMAP_CRATES=str(PRICING / "base/crate_dirs.txt"),
        LINEMAP_ROWS=str(PRICING / "data/basis_text_rows.json"),
        LINEMAP_RESOLUTIONS=str(PRICING / "rules/linemap_resolutions.json"))
    carried = subprocess.run([sys.executable, str(PRICING / "lib/linemap_rules.py"),
        "--prune-deleted", str(repo), "2243be380b", target, str(mapped_rules)]
        + [prepared.get(name, str(PRICING / "base" / name)) for name in rule_files],
        env=environment, text=True, capture_output=True)
    assert carried.returncode == 0, carried.stdout + carried.stderr
    chain = tmp_path / "chain"
    shutil.copytree(PRICING / "base", chain)
    for name in rule_files:
        shutil.copyfile(mapped_rules / name, chain / name)
    environment.update(SQ2_SNAP=str(SOURCE), SQ2_TARGET=target, SQ2_REPO=str(repo),
        SQ2_DEC_BASIS="2243be380b", SQ2_LM=str(mapped_rules / "g7_linemap.out.json"),
        SQ2_RESOLUTIONS=str(PRICING / "rules/linemap_resolutions.json"))
    mapper_api = runpy.run_path(str(PRICING / "lib/linemap_lib.py"))
    mapper = mapper_api["Mapper"](str(repo), "2243be380b", target,
        (PRICING / "base/crate_dirs.txt").read_text().split(),
        mapper_api["load_rows"]([str(PRICING / "data/basis_text_rows.json")]),
        mapper_api["load_resolutions"](str(PRICING / "rules/linemap_resolutions.json")))
    def current(key):
        result = mapper.map_key(key, "maintained priority control")
        assert not mapper.unmapped, mapper.unmapped
        return result
    decisions = PRICING / ("rules/sq2_decisions_L.py" if route == "L" else "rules/sq2_decisions_E.py")
    result = subprocess.run([sys.executable, str(PRICING / "lib/sq2_rules.py"),
                             str(chain), str(tmp_path / "actual"), str(decisions)],
                            env=environment, text=True, capture_output=True)
    assert result.returncode == 0, result.stdout + result.stderr
    loop = json.loads((tmp_path / "actual/loop_bounds.g4.json").read_bytes())
    text = json.loads((tmp_path / "actual/text_args.g4.json").read_bytes())
    def first(header):
        return next(row for row in loop["loops"] if re.search(row["re"], header))
    for header, _, _, _ in MODEL["O10_PREIMAGES"]:
        inner = header in ("for (j, expected) in manifest.iter().enumerate()",
                           "for p in source.loads.iter().filter(|p| p.term_index == i)")
        current_header = "for p in source.loads.iter()" if header == "for p in &source.loads" else header
        assert first(current_header)["bound"] == ("1" if inner else "12*l"), (route, current_header, first(current_header))
    for index, _, _, _, wanted in MODEL["BASE_LOOP_PREIMAGES"]:
        # Use actual representative headers, not a claim of rule presence.
        header = {211:"iter:terms.iter().map",225:"for t in &d.terms",
                  265:"for (i, t) in d.terms.iter().enumerate()",
                  276:"for &index in indices",222:'iter:" ":forces.iter().map',
                  226:"for product in &row.offset",219:"for &component in scaled_intended"}[index]
        assert first(header)["bound"] == wanted
    assert first("for f in forces")["bound"] == "12*l"
    assert first("iter:old.loads().iter().enumerate().map")["bound"] == "l"
    assert first("for (i, l) in source.loads().iter().enumerate()")["bound"] == "l"
    # A global header still has its old conservative P; only this exact
    # function's effective total replaces cases*P with cases*N.
    total = [r for r in loop["loop_total"] if r["fn"] == "formation_guard.rs:304:load_row_finding"]
    assert len(total) == 1 and total[0]["total"] == "cases*N"
    assert re.search(total[0]["loop"], "for row in &rows")
    assert first("for row in &rows")["bound"] == "P"
    for key in ("formation_guard.rs:315", "formation_guard.rs:320", "self_weight.rs:869", "self_weight.rs:1281",
                "self_weight.rs:1286"):
        assert key not in text["site_zero"]
    for key in ("formation_guard.rs:239", "formation_guard.rs:241", "formation_guard.rs:251", "formation_guard.rs:322"):
        assert key in text["site_total"]
    assert text["lex_site_size"]["formation_guard.rs:330"]["class"] == 993964
    assert text["lex_site_size"]["retained_product.rs:677"]["class"] == 994321   # T4-S5: 611 moved; O4 S5: 616 -> 677, unchanged text
    assert text["site_total"][current("load_ledger.rs:133")]["bytes"] == "5*cases*(12*l)*2*128"
    assert text["site_total"][current("load_ledger.rs:408")]["bytes"] == "4*cases*(12*l)*2*128"
    # Two independent lists are an occurrence multiplier; the final2 is
    # still the old String-capacity allowance, not the same factor twice.
    assert text["site_total"][current("load_ledger.rs:670")]["bytes"] == "10*cases*(12*l)*2*128"
    sf = text["site_from"][current("structural.rs:1103")]
    assert sf["per"] == "12*l"
    assert sf["from"] == [current(key) for key in MODEL["FIDELITY_PREIMAGES"]["source_clone"]["from"]]
    joined = next(row for row in text["join_rules"] if re.search(row["re"], "rows"))
    assert joined["re"] == "^rows$"
    assert "(12*l)*(2+10*128+2+2)" in joined["bytes"]
    # Exercise the actual text_budget AST branches in their real order,
    # with the complete prepared tables. This checks effective overridden
    # counts/costs, not only a new rule's presence in the JSON.
    import ast
    ast_nodes = ast.parse((PRICING / "base/text_budget.py").read_text()).body
    lex = next(node for node in ast_nodes if isinstance(node, ast.For)
               and isinstance(node.iter, ast.Name) and node.iter.id == "LEX")
    assign_sf = next(node for node in lex.body if isinstance(node, ast.Assign)
                     and any(isinstance(t, ast.Name) and t.id == "sf" for t in node.targets))
    if_sf = lex.body[lex.body.index(assign_sf) + 1]
    assert isinstance(if_sf, ast.If)
    counts = dict(N=192, l=128, cases=3)
    evaluate = lambda expression: int(eval(expression, {"__builtins__": {}, "max": max}, counts))
    environment = dict(TA=text, lkey=current("structural.rs:1103"), mult=10**9,
                       M={"core/solver/frame_kernel/src/" + sf["from"][0]: 3,
                          "core/solver/frame_kernel/src/" + sf["from"][1]: 6},
                       short=lambda key:key.split("/src/")[-1], ev=evaluate)
    exec(compile(ast.Module(body=[assign_sf, if_sf], type_ignores=[]), "real site_from branch", "exec"), environment)
    assert environment["mult"] == 9 * 12 * 128
    fmt = next(node for node in ast_nodes if isinstance(node, ast.For)
               and isinstance(node.iter, ast.Subscript)
               and isinstance(node.iter.value, ast.Name) and node.iter.value.id == "inv"
               and any(isinstance(x, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "sf" for t in x.targets)
                       for x in node.body))
    size_choice = next(node for node in fmt.body if isinstance(node, ast.If)
                       and isinstance(node.test, ast.Compare)
                       and isinstance(node.test.left, ast.Name) and node.test.left.id == "key"
                       and any(isinstance(t, ast.Name) and t.id == "AGG" for t in node.test.comparators))
    for key in MODEL["FIDELITY_PREIMAGES"]["aggregates"]:
        environment.update(key=current(key), AGG=text["site_aggregate"], ev=evaluate)
        exec(compile(ast.Module(body=[size_choice], type_ignores=[]), "real aggregate branch", "exec"), environment)
        assert environment["size"] == evaluate(text["site_aggregate"][current(key)]["bytes"])
        assert environment["size"] >= 12 * 128 * (10 * 128 + 4)
    # SITE_TOTAL follows and supersedes ordinary mult*capacity. Extract the
    # actual format branch: each source-list occurrence keeps capacity2.
    assign_st = next(node for node in fmt.body if isinstance(node, ast.Assign)
                     and any(isinstance(t, ast.Name) and t.id == "st" for t in node.targets))
    if_st = fmt.body[fmt.body.index(assign_st) + 1]
    for key, wanted in (("load_ledger.rs:133", 5*3*1536*2*128),
                        ("load_ledger.rs:408", 4*3*1536*2*128),
                        ("load_ledger.rs:670", 5*3*2*1536*2*128)):
        environment.update(key=current(key), TA=text, mult=10**9, req=10**12)
        exec(compile(ast.Module(body=[assign_st, if_st], type_ignores=[]), "real site_total branch", "exec"), environment)
        assert environment["req"] == wanted


def test_retained_and_independent_value_copies_are_consumed_in_actual_models():
    producer = (PRICING / "base/producer_caps.py").read_text()
    assert 'finding_capture(_finding_cases, n, l, ID)' in producer
    g4 = (PRICING / "base/g4_caps.py").read_text()
    assert 'ORDINARY += FINDING_VALUE' in g4
    # L3 removes only the independently built transient; the body owner and
    # named form remain. Both source cohorts execute the actual expression.
    import ast
    from types import SimpleNamespace
    import sys
    sys.path.insert(0, str(PRICING / "base"))
    from g3lib import X
    assignment = next(node for node in ast.parse(g4).body if isinstance(node, ast.Assign)
        and any(isinstance(t, ast.Name) and t.id == "FINDING_VALUE_BUILD" for t in node.targets))
    for reduced, expected in [(False, {"1": 22}), (True, {})]:
        scope = {"X": X, "SNAP": str(SOURCE),
                 "_w1_reductions_model": SimpleNamespace(inspect=lambda _: reduced),
                 "FINDING_VALUE": SimpleNamespace(tree=lambda: X({"1": 11}))}
        exec(compile(ast.Module(body=[assignment], type_ignores=[]), "actual L3 owner", "exec"), scope)
        assert scope["FINDING_VALUE_BUILD"] == expected
    assert 'form("FORMATION_VALUE_BUILD", FINDING_VALUE_BUILD)' in g4
    namespace = {}
    import ast
    count_assignment = next(node for node in ast.walk(ast.parse(producer))
        if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "_finding_cases" for t in node.targets))
    for branch, ordinary, success, expected in [("ordinary",3,2,3), ("success",3,2,2),
                                               ("ordinary",0,0,0), ("success",0,0,0)]:
        actual = {"_phase_context":{"branch":branch}, "_phase_ordinary":{"finding_cases":ordinary},
                  "_phase_success":{"finding_cases":success}}
        exec(compile(ast.Module(body=[count_assignment],type_ignores=[]), "actual scoped FindingSeed count", "exec"), actual)
        assert actual["_finding_cases"] == expected
    tree = ast.parse(g4)
    selected = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "phases_of"]
    assignment = next(n for n in tree.body if isinstance(n, ast.Assign)
                      and any(isinstance(t, ast.Name) and t.id == "EVERY" for t in n.targets))
    exec(compile(ast.Module(body=[assignment] + selected, type_ignores=[]), "g4 phase source", "exec"), namespace)
    # PU-X3 (§8.14): with and without the republication phase, every phase holds the formation Value build.
    namespace["_phase_model"] = SimpleNamespace(X3_NAME="X3 republication")
    for x3, count in ((False, 7), (True, 8)):
        namespace["_X3"] = x3
        for mode in ("sparse", "dense"):
            phases = namespace["phases_of"](mode)
            assert len(phases) == count
            for requested, _ in phases.values():
                assert "FORMATION_VALUE_BUILD" in requested
    native = (PRICING / "base/g5_profile.py").read_text()
    assert 'sz("open_pipe_stress_frame_kernel::load_ledger::FormationRow")' in native
    assert 'sz("(usize, [f64; 4])")' in native
    # Establish the actual dependency and public module/type edge too; a
    # copied binder spelling alone did not catch the earlier nonexistent alias.
    import tomllib
    dependencies = tomllib.loads((SOURCE / "core/product_physics/Cargo.toml").read_text())["dependencies"]
    assert "open_pipe_stress_frame_kernel" in dependencies
    assert "frame_kernel" not in dependencies
    assert "pub mod load_ledger;" in (SOURCE / "core/solver/frame_kernel/src/lib.rs").read_text()
    assert "pub struct FormationRow {" in (SOURCE / "core/solver/frame_kernel/src/load_ledger.rs").read_text()



def test_newly_reached_identifier_audits_bind_exact_production_expressions():
    _, audits = MODEL["decisions"](SOURCE, lambda key:key)
    table = dict(audits)
    pp = "core/product_physics/src/"
    assert table[pp+"formation_guard.rs:322"]["dof_label(row.dof)"] == {"bytes":600,"source":"COMP"}
    assert table[pp+"formation_guard.rs:327"]["case_id"] == {"bytes":128,"source":"IN128"}
    # T4-U6/U8 snapshot lines, unchanged text (B7b: self_weight.rs 814; formation_check.rs 495..635 and 895);
    # W-15 slice b (§8.22): self_weight.rs 869 -> 1281; nested-record closure (§8.NR): 1281 -> 1286.
    assert table[pp+"self_weight.rs:1286"] == {
        "case.id":{"bytes":128,"source":"IN128"},
        "pipe":{"bytes":128,"source":"IN128"},
        "case.id.len()":{"bytes":20,"source":"NUM"},
        "pipe.len()":{"bytes":20,"source":"NUM"},
    }
    fc = "core/solver/frame_kernel/src/structural/formation_check.rs:"
    # T4-S5 (epoch 4): unchanged text, moved (T4-U6/U8: 509, 515, 525, 575, 588, 595; 627, 637, 643, 649).
    for line in (528,534,544,620,633,640):
        assert table[fc+str(line)] == {
            "terms[p.term_index].source":{"bytes":128,"source":"IN128"}}
    for line in (672,682,688,694,MODEL["FIRST_RETURN_LINE"]):
        assert table[fc+str(line)] == {
            "term.source":{"bytes":128,"source":"IN128"}}
    # These are literal producer/source edges under the existing raw guards,
    # not a class for arbitrary Foreign FormationSource identities.
    lib=(SOURCE/(pp+"lib.rs")).read_text()
    assert '|dof| integrity_dof_label(model, dof),' in lib
    assert '|node| format!("{}:{name}", node.id),' in lib
    assert '|| format!("global_dof={dof}"),' in lib
    assert 'ledger.push_formed(\n                        &load.load_id,' in lib
    cl=(SOURCE/(pp+"certified_loads.rs")).read_text()
    assert 'ledger.push(&load.load_id, load.global_dof, load.value);' in cl
    ledger=(SOURCE/"core/solver/frame_kernel/src/load_ledger.rs").read_text()
    assert 'source: source.into(),' in ledger
    assert 'source: term.source.clone(),' in ledger
    sw=(SOURCE/(pp+"self_weight.rs")).read_text()
    assert 'LoadTargetInput::Element { pipe }' in sw
    assert 'case.id.len(),' in sw and 'pipe.len(),' in sw
    # Current model's every-source/class/cap refusal proof is retained.
    proof=MODEL["inspect"](SOURCE)
    assert len(proof["full_raw_source_fingerprints"])+len(proof["formation_raw_fingerprints"])==27



def first_return_fixture():
    site = MODEL["FIRST_RETURN_SITE"]
    table = {"b7a_reformation_first_error":"FK895-v1", "site_from": {
        site: dict(MODEL["FIRST_RETURN_RULE"], why="SQ2 G5: "+MODEL["FIRST_RETURN_WHY"])}}
    root,parent = MODEL["FIRST_RETURN_ROOT"],MODEL["FIRST_RETURN_PARENT"]
    graph = {"edges": {root: [],parent:[root]}}
    inventory = [{"file":MODEL["FIRST_RETURN_FILE"],"line":MODEL["FIRST_RETURN_LINE"],"kind":"format","fn":root}]
    return table,graph,inventory


def test_first_return_preserves_full_evaluate_counts_and_positive_content():
    table,graph,inventory=first_return_fixture()
    state=MODEL["first_return_state"](SOURCE,table,graph,inventory,[])
    f=MODEL["FIRST_RETURN_FILE"]
    for full in (672,448,224,672):
        counts={MODEL["FIRST_RETURN_ROOT"]:full}
        MODEL["first_return_witness"](state,f,MODEL["FIRST_RETURN_LINE"],"format",full*1536,counts)
        MODEL["first_return_witness"](state,f,MODEL["FIRST_RETURN_LINE"],"format",full,counts,rebound=True)
        assert 2*8334*full in (11200896,7467264,3733632)
    # The actual source-bound returning expression, not a visit cap.
    source=(SOURCE/f).read_text()
    assert 'fn evaluate<F, A, P>(' in source
    assert '.map_err(|e| Failure::Closure(format!("{}: reformation {e:?}", term.source)))?;' in source
    assert 'let value = v[p.local_row];' in source


@pytest.mark.parametrize("change",["remove_qmark","catch_continue","second_site","crlf"])
def test_first_return_source_change_refuses(tmp_path,change):
    snapshot=source_copy(tmp_path)
    p=snapshot/MODEL["FIRST_RETURN_FILE"]
    raw=p.read_bytes()
    needle=b'.map_err(|e| Failure::Closure(format!("{}: reformation {e:?}", term.source)))?;'
    assert raw.count(needle)==1
    if change=="crlf": raw=raw.replace(b"\n",b"\r\n")
    elif change=="second_site": raw+=b"\n"+needle+b"\n"
    else: raw=raw.replace(needle,needle.replace(b"?;",b";" if change=="remove_qmark" else b".unwrap_or_else(|_| continue);"))
    p.write_bytes(raw)
    table,graph,inventory=first_return_fixture()
    with pytest.raises(ValueError,match="owner source changed"):
        MODEL["first_return_state"](snapshot,table,graph,inventory,[])


@pytest.mark.parametrize("change",["guard","per","owner","why","zero","total","aggregate","size","lex_size","alias_from"])
def test_first_return_rule_or_override_drift_refuses(change):
    table,graph,inventory=first_return_fixture()
    key=MODEL["FIRST_RETURN_SITE"]
    if change=="guard": del table["b7a_reformation_first_error"]
    elif change in ("per","owner","why"):
        field={"per":"per","owner":"from","why":"why"}[change]
        table["site_from"][key][field]={"per":"0","owner":["unknown.rs:1:evaluate"],"why":"changed"}[change]
    else:
        family={"zero":"site_zero","total":"site_total","aggregate":"site_aggregate",
                "size":"site_size","lex_size":"lex_site_size","alias_from":"site_from"}[change]
        table.setdefault(family,{})[MODEL["FIRST_RETURN_FILE"]+":%d:format" % MODEL["FIRST_RETURN_LINE"]]={"bytes":0}
    with pytest.raises(ValueError,match="B7aFK895"):
        MODEL["first_return_state"](SOURCE,table,graph,inventory,[])


@pytest.mark.parametrize("change",["missing_node","missing_edge","duplicate_edge","extra_parent","missing_site","duplicate_site","wrong_owner"])
def test_first_return_exact_graph_inventory_refuses(change):
    table,graph,inventory=first_return_fixture()
    root,parent=MODEL["FIRST_RETURN_ROOT"],MODEL["FIRST_RETURN_PARENT"]
    if change=="missing_node": del graph["edges"][root]
    elif change=="missing_edge": graph["edges"][parent]=[]
    elif change=="duplicate_edge": graph["edges"][parent].append(root)
    elif change=="extra_parent": graph["edges"]["foreign.rs:1:evaluate"]=[root]
    elif change=="missing_site": inventory=[]
    elif change=="duplicate_site": inventory+=inventory[:]
    else: inventory[0]["fn"]=parent
    with pytest.raises(ValueError,match="B7aFK895"):
        MODEL["first_return_state"](SOURCE,table,graph,inventory,[])


@pytest.mark.parametrize("normal,post,count",[(0,672,672),(100,672,0),(100,671,672),(100,673,672)])
def test_first_return_pre_and_post_counts_never_erase_positive_owner(normal,post,count):
    table,graph,inventory=first_return_fixture()
    state=MODEL["first_return_state"](SOURCE,table,graph,inventory,[])
    counts={MODEL["FIRST_RETURN_ROOT"]:count}
    with pytest.raises(ValueError,match="KD5C6"):
        MODEL["first_return_witness"](state,MODEL["FIRST_RETURN_FILE"],MODEL["FIRST_RETURN_LINE"],"format",normal,counts)
        MODEL["first_return_witness"](state,MODEL["FIRST_RETURN_FILE"],MODEL["FIRST_RETURN_LINE"],"format",post,counts,rebound=True)


def test_shared_illustrative_layouts_exist_in_fresh_process(tmp_path):
    code = (
        "import json,runpy; "
        "g=runpy.run_path(" + repr(str(PRICING / "base/g3lib.py")) + "); "
        "a=g['ASSUMED']; "
        "v=g['X']({'s(FormationRow)':2,'s((usize,[f64;4]))':4}).ev(a); "
        "print(json.dumps([a['s(FormationRow)'],a['s((usize,[f64;4]))'],v]))"
    )
    result = subprocess.run([sys.executable, "-c", code], cwd=tmp_path,
                            text=True, capture_output=True)
    assert result.returncode == 0, result.stderr
    assert json.loads(result.stdout) == [3 * 1104 + 128, 5 * 8, 7040]
    # Actual qualified binders remain the separately measured Rust types.
    generator = (PRICING / "base/g5_profile.py").read_text()
    assert '"s(FormationRow)": sz("open_pipe_stress_frame_kernel::load_ledger::FormationRow")' in generator
    assert '"s((usize,[f64;4]))": sz("(usize, [f64; 4])")' in generator


@pytest.mark.parametrize("missing", [
    "s(FormationRow)", "s((usize,[f64;4]))", "s(UnboundFormationAtom)"
])
def test_shared_illustrative_missing_atom_still_refuses(missing):
    g3 = runpy.run_path(str(PRICING / "base/g3lib.py"))
    values = dict(g3["ASSUMED"])
    values.pop(missing, None)
    with pytest.raises(KeyError, match="no ASSUMED value"):
        g3["X"]({missing: 1}).ev(values)


def source_fn_key(name, f):
    """The rules' rs() over the actual source: `<src-relative file>:<line>:<name>`."""
    import re
    lines = (SOURCE / f).read_text().split("\n")
    hits = [i + 1 for i, line in enumerate(lines)
            if re.match(r"\s*(pub(\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", line)]
    assert len(hits) == 1, (name, f, hits)
    return f"{f.split('/src/')[-1]}:{hits[0]}:{name}"


def route_e_site_from(common):
    import ast
    source = (PRICING / "rules/sq2_decisions_E.py").read_text()
    body = ast.parse(source).body
    def assigned(name):
        found = [i for i, node in enumerate(body) if isinstance(node, ast.Assign)
                 and any(isinstance(t, ast.Name) and t.id == name for t in node.targets)]
        assert len(found) == 1, name
        return found[0]
    index = assigned("SITE_FROM")
    assert isinstance(body[index + 1], ast.If)
    # B7b-F: the local rules' entry (_PE_ENTRY/_PE_NAME) is decided just before them.
    entry = assigned("_PE_ENTRY")
    assert assigned("_PE_NAME") == entry + 1 == index - 1
    namespace = {"SITE_FROM": deepcopy(common), "B7B_EXACT_BASE": True,
                 "RE_FULL": "core/reporting/result_export/src/", "rs": lambda name, f: source_fn_key(name, f)}
    program = ast.Module(body=body[entry:index + 2], type_ignores=[])
    exec(compile(program, "route E SITE_FROM source", "exec"), namespace)
    return namespace["SITE_FROM"]


def test_route_e_preserves_every_common_and_original_site_from_rule():
    common = MODEL["first_return_rules"](SOURCE, lambda key: key)
    before = deepcopy(common)
    combined = route_e_site_from(common)
    assert common == before
    assert combined[:len(common)] == common
    local = combined[len(common):]
    assert [key for key, _ in local] == ["physics_evidence.rs:" + str(n) for n in (38, 48, 53, 57)]
    # B7b-F: route E's reached entry is validate_pressure_evidence (validate_physics_evidence is deleted).
    entry = source_fn_key("validate_pressure_evidence", "core/reporting/result_export/src/physics_evidence.rs")
    assert all(rule["from"] == [entry] and rule["per"] == "1" for _, rule in local)
    assert len({key for key, _ in combined}) == len(combined)
    table, graph, inventory = first_return_fixture()
    table["site_from"][MODEL["FIRST_RETURN_SITE"]] = dict(
        combined[0][1], why="SQ2 G5: " + combined[0][1]["why"])
    MODEL["first_return_state"](SOURCE, table, graph, inventory, [])


@pytest.mark.parametrize("common", [
    [("physics_evidence.rs:38", {"from": [], "per": "0"})],
    [("same.rs:1", {"per": "1"}), ("same.rs:1", {"per": "2"})],
])
def test_route_e_conflicting_or_duplicate_site_from_refuses(common):
    with pytest.raises(ValueError, match="duplicate/conflicting"):
        route_e_site_from(common)
