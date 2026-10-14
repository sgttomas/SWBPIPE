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
SOURCE = PRICING.parents[1]
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
    assert bound == dict(N=192, q=1536, debug_total=1972608, fired_total=3664128,
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
    ("core/solver/frame_kernel/src/structural/formation_check.rs", "for p in &source.loads"),
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
    assert new_text == expected_text
    assert {name: Path(path).read_bytes() for name, path in inherited.items()} == before
    assert all(hashlib.sha256(Path(actual[name]).read_bytes()).hexdigest() == digest
               for name, digest in report["prepared_files"].items())


@pytest.mark.parametrize("change", ["zero", "copy_bytes", "copy_reason", "loop_value", "loop_reason", "loop_order",
                                     "source_clone", "source_clone_caller", "fidelity_aggregate", "fidelity_join"])
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
    chain = tmp_path / "chain"
    shutil.copytree(PRICING / "base", chain)
    for name, path in prepared.items():
        shutil.copyfile(path, chain / name)
    environment = dict(os.environ, SQ2_SNAP=str(SOURCE))
    # These are the actual decisions and SQ2 preparation implementation.
    # No line-map exception is substituted by this local priority control.
    for name in ("SQ2_TARGET", "SQ2_LM"):
        environment.pop(name, None)
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
        assert first(header)["bound"] == ("1" if inner else "12*l"), (route, header, first(header))
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
    for key in ("formation_guard.rs:315", "formation_guard.rs:320", "self_weight.rs:814"):
        assert key not in text["site_zero"]
    for key in ("formation_guard.rs:239", "formation_guard.rs:241", "formation_guard.rs:251", "formation_guard.rs:322"):
        assert key in text["site_total"]
    assert text["lex_site_size"]["formation_guard.rs:330"]["class"] == 993964
    assert text["lex_site_size"]["retained_product.rs:545"]["class"] == 994321
    assert text["site_total"]["load_ledger.rs:133"]["bytes"] == "5*cases*(12*l)*2*128"
    assert text["site_total"]["load_ledger.rs:408"]["bytes"] == "4*cases*(12*l)*2*128"
    # Two independent lists are an occurrence multiplier; the final2 is
    # still the old String-capacity allowance, not the same factor twice.
    assert text["site_total"]["load_ledger.rs:670"]["bytes"] == "10*cases*(12*l)*2*128"
    sf = text["site_from"]["structural.rs:1103"]
    assert sf["per"] == "24*l"
    assert sf["from"] == MODEL["FIDELITY_PREIMAGES"]["source_clone"]["from"]
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
    environment = dict(TA=text, lkey="structural.rs:1103", mult=10**9,
                       M={"core/solver/frame_kernel/src/" + sf["from"][0]: 3,
                          "core/solver/frame_kernel/src/" + sf["from"][1]: 6},
                       short=lambda key:key.split("/src/")[-1], ev=evaluate)
    exec(compile(ast.Module(body=[assign_sf, if_sf], type_ignores=[]), "real site_from branch", "exec"), environment)
    assert environment["mult"] == 9 * 24 * 128
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
        environment.update(key=key, AGG=text["site_aggregate"], ev=evaluate)
        exec(compile(ast.Module(body=[size_choice], type_ignores=[]), "real aggregate branch", "exec"), environment)
        assert environment["size"] == evaluate(text["site_aggregate"][key]["bytes"])
        assert environment["size"] >= 12 * 128 * (10 * 128 + 4)
    # SITE_TOTAL follows and supersedes ordinary mult*capacity. Extract the
    # actual format branch: each source-list occurrence keeps capacity2.
    assign_st = next(node for node in fmt.body if isinstance(node, ast.Assign)
                     and any(isinstance(t, ast.Name) and t.id == "st" for t in node.targets))
    if_st = fmt.body[fmt.body.index(assign_st) + 1]
    for key, wanted in (("load_ledger.rs:133", 5*3*1536*2*128),
                        ("load_ledger.rs:408", 4*3*1536*2*128),
                        ("load_ledger.rs:670", 5*3*2*1536*2*128)):
        environment.update(key=key, TA=text, mult=10**9, req=10**12)
        exec(compile(ast.Module(body=[assign_st, if_st], type_ignores=[]), "real site_total branch", "exec"), environment)
        assert environment["req"] == wanted


def test_retained_and_independent_value_copies_are_consumed_in_actual_models():
    producer = (PRICING / "base/producer_caps.py").read_text()
    assert 'finding_capture(CC, n, l, ID)' in producer
    g4 = (PRICING / "base/g4_caps.py").read_text()
    assert 'ORDINARY += FINDING_VALUE' in g4
    assert 'FINDING_VALUE_BUILD = FINDING_VALUE.tree() * 2' in g4
    assert 'form("FORMATION_VALUE_BUILD", FINDING_VALUE_BUILD)' in g4
    namespace = {}
    import ast
    tree = ast.parse(g4)
    selected = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "phases_of"]
    assignment = next(n for n in tree.body if isinstance(n, ast.Assign)
                      and any(isinstance(t, ast.Name) and t.id == "EVERY" for t in n.targets))
    exec(compile(ast.Module(body=[assignment] + selected, type_ignores=[]), "g4 phase source", "exec"), namespace)
    for mode in ("sparse", "dense"):
        for requested, _ in namespace["phases_of"](mode).values():
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
