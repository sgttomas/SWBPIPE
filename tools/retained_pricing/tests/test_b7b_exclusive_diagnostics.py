"""B7b-DPS's source premise and B7b-DX's exclusive diagnostic group pass."""
import ast
import collections
import os
import re
from pathlib import Path
import runpy

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
LIB = "core/product_physics/src/lib.rs"


def decisions(monkeypatch, snapshot=SOURCE):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_E.py"))


class Resolved(str):
    pass


def ca_block(snapshot):
    """o10_rules.py's B7b-CA/B7b-DPS block, as written, with its own fnkey/site, over `snapshot`.
    (A changed lib.rs also stops the full decisions earlier, at its full-file pins.)"""
    module = ast.parse((PRICING / "rules/o10_rules.py").read_text())
    helpers = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name in ("fnkey", "site")]
    blocks = [node for node in module.body if isinstance(node, ast.If) and "derive_pipe_section" in ast.unparse(node.test)]
    assert len(helpers) == 2 and len(blocks) == 1
    namespace = {"_SNAP": str(snapshot), "_os": os, "_re": re, "Resolved": Resolved, "PP_": "core/product_physics/src/",
                 "IN": {"bytes": 128, "source": "IN128"}, "O10_ID_AUDIT": []}
    exec(compile(ast.Module(body=helpers + blocks, type_ignores=[]), "o10 B7b-CA/DPS", "exec"), namespace)
    return namespace


def with_lib(tmp_path, text):
    target = tmp_path / "snap" / LIB
    target.parent.mkdir(parents=True)
    target.write_text(text)
    return tmp_path / "snap"


def derive_pipe_section_sites():
    lines = (SOURCE / LIB).read_text().split("\n")
    start = next(i + 1 for i, line in enumerate(lines) if line.startswith("fn derive_pipe_section("))
    end = next(i + 1 for i in range(start, len(lines)) if re.match(r"(pub(\([a-z]+\))?\s+)?fn\s", lines[i]))
    return start, [i + 1 for i in range(start - 1, end - 1) if "diagnostics.push(diag(" in lines[i]]


def test_the_group_is_derive_pipe_sections_three_returning_diagnostics(monkeypatch, tmp_path):
    """U6-DPS: on D-5 source the nonzero-allowance refusal is gone; the dimension, mill-tolerance and invalid-allowance
    diagnostics remain one exclusive group (B7b's four-site form is the registered B7b source's)."""
    text = (SOURCE / LIB).read_text()
    assert "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED" not in text and "fn section_basis_row(" in text
    start, sites = derive_pipe_section_sites()
    assert len(sites) == 3
    expected = [("B7b-DPS", f"lib.rs:{start}:derive_pipe_section", [f"lib.rs:{n}" for n in sites])]
    groups = decisions(monkeypatch)["O10_DIAG_EXCLUSIVE"]
    assert [(name, fn, members) for name, fn, members, _ in groups] == expected
    block = ca_block(with_lib(tmp_path, text))
    assert [(name, fn, members) for name, fn, members, _ in block["O10_DIAG_EXCLUSIVE"]] == expected
    assert block["O10_DIAG_EXCLUSIVE"][0][3].startswith("U6-DPS (B7b-DPS on D-5 source)")
    assert len(block["O10_ID_AUDIT"]) == 4, "two unit-conversion and two section corrosion-allowance ID sites"


def test_u6_dps_refuses_a_lost_allowance_refusal_without_section_bases(tmp_path):
    """U6-DPS: only D-5 source (section_basis_row) may have derive_pipe_section without the nonzero refusal."""
    text = (SOURCE / LIB).read_text()
    assert text.count("fn section_basis_row(") == 1
    with pytest.raises(SystemExit, match="U6-DPS"):
        ca_block(with_lib(tmp_path, text.replace("fn section_basis_row(", "fn section_basis_row_renamed(")))


@pytest.mark.parametrize("mutation,message", [
    ("no_return", "not followed by `return None;`"),
    ("fifth", "diagnostic inventory changed"),
    ("other_push", "producer other than a pushed diag"),
    ("bare_diag", "producer other than a pushed diag"),
    ("callee", "uses `diagnostics` other than in a pushed diag"),
])
def test_a_changed_premise_refuses(tmp_path, mutation, message):
    text = (SOURCE / LIB).read_text()
    # U6-DPS: on D-5 source the last diagnostic is the invalid-allowance refusal (B7b's anchor, the nonzero one, is gone).
    nonzero = '"corrosion allowance must be a finite non-negative length that leaves a positive wall after mill tolerance",\n'
    assert text.count(nonzero) == 1
    at = text.index(nonzero)
    close = text.index("            ));\n            return None;\n", at)
    if mutation == "no_return":
        text = text[:close] + "            ));\n" + text[close + len("            ));\n            return None;\n"):]
    elif mutation == "fifth":
        extra = ('            diagnostics.push(diag("diagnostic:section:extra", "PIPE_DIMENSION_INVALID", "blocking", '
                 '"extra", vec![pipe_id.to_string()]));\n            return None;\n')
        text = text[:close] + "            ));\n            return None;\n" + extra + text[close + len("            ));\n            return None;\n"):]
    elif mutation == "other_push":
        anchor = "    let id = od - 2.0 * thickness;\n"
        assert text.count(anchor) == 1
        text = text.replace(anchor, "    diagnostics.push(diagnostics[0].clone());\n" + anchor, 1)
    elif mutation == "callee":
        anchor = "    let id = od - 2.0 * thickness;\n"
        text = text.replace(anchor, "    check_more(input, pipe_id, diagnostics);\n" + anchor, 1)
    else:
        anchor = "    let id = od - 2.0 * thickness;\n"
        text = text.replace(anchor, '    let _ = diag("x", "x", "info", "x", vec![]);\n' + anchor, 1)
    with pytest.raises(SystemExit, match=re.escape(message)):
        ca_block(with_lib(tmp_path, text))


def group_pass():
    """text_budget.py's B7b-DX group pass, as written (the one top-level loop over DIAG_EXCLUSIVE)."""
    module = ast.parse((PRICING / "base/text_budget.py").read_text())
    loops = [node for node in module.body if isinstance(node, ast.For)
             and ast.unparse(node.iter) == "DIAG_EXCLUSIVE.items()"]
    short = next(node for node in module.body if isinstance(node, ast.FunctionDef) and node.name == "short")
    assert len(loops) == 1
    code = compile(ast.Module(body=[short, loops[0]], type_ignores=[]), "text_budget B7b-DX", "exec")

    def run(seen, calls, sites=("lib.rs:10", "lib.rs:20")):
        groups = {"B7b-DPS": {"fn": "lib.rs:1:derive_pipe_section", "sites": list(sites)}}
        namespace = {"DIAG_EXCLUSIVE": groups, "DIAG_EXCLUSIVE_SEEN": seen, "M": calls,
                     "unclassified": collections.Counter(), "D": 7, "diag_bytes": 100}
        exec(code, namespace)
        return namespace["D"], namespace["diag_bytes"], dict(namespace["unclassified"])
    return run


def member(mult, size, fn="core/product_physics/src/lib.rs:1:derive_pipe_section", loops=()):
    return {"fn": fn, "loops": list(loops), "mult": mult, "bytes": size}


def test_one_diagnostic_per_owner_call_at_the_largest_member():
    run = group_pass()
    calls = {"core/product_physics/src/lib.rs:1:derive_pipe_section": 96}
    seen = {"lib.rs:10": member(96, 400), "lib.rs:20": member(96, 900)}
    assert run(seen, calls) == (7 + 96, 100 + 96 * 900, {})
    # An unreached owner with no reached member adds nothing.
    assert run({}, {}) == (7, 100, {})


@pytest.mark.parametrize("case", ["missing", "looped", "foreign", "unequal", "unreached_owner"])
def test_any_unproven_member_leaves_the_run_incomplete(case):
    run = group_pass()
    calls = {"core/product_physics/src/lib.rs:1:derive_pipe_section": 96}
    seen = {"lib.rs:10": member(96, 400), "lib.rs:20": member(96, 900)}
    if case == "missing":
        del seen["lib.rs:20"]
    elif case == "looped":
        seen["lib.rs:20"] = member(96, 900, loops=["for pipe in pipes"])
    elif case == "foreign":
        seen["lib.rs:20"] = member(96, 900, fn="core/product_physics/src/lib.rs:5:other")
    elif case == "unequal":
        seen["lib.rs:20"] = member(192, 900)
    else:
        calls = {}
    assert run(seen, calls) == (7, 100, {("diag-exclusive", "B7b-DPS"): 1})


def test_b7b_jr_guards_pressure_findings_diagnostic_at_multiplicity_zero(monkeypatch):
    """B7b-JR: the fourth-ref pressure finding is guarded (o10_check's multiplicity-0 list), not zeroed or resized."""
    lines = (SOURCE / "core/product_physics/src/joint.rs").read_text().split("\n")
    start = next(i for i, line in enumerate(lines) if line.startswith("fn pressure_finding("))
    push = next(i + 1 for i in range(start, len(lines)) if "diagnostics.push(diag(" in lines[i])
    assert "refs.extend(extra.map(str::to_string));" in lines[push - 2]
    d = decisions(monkeypatch)
    # W8-BT (§8.16) adds validate_components' branch-topology warning and its id, also guarded at multiplicity 0.
    assert [key for key, _ in d["O10_MULT0_SITES"] if key.startswith("joint.rs:")] == [f"joint.rs:{push}"]
    # SZ-LS (§8.15) guards load-state resolution's sites in case_state/resolve.rs (test_silent_zero_guard.py).
    assert [key.split(":")[0] for key, _ in d["O10_MULT0_SITES"]
            if not key.startswith(("joint.rs:", "case_state/resolve.rs:"))] == ["validation.rs"] * 2
    assert not any(str(key).startswith("joint.rs:") and key == f"joint.rs:{push}" for key, _, _ in d.get("O10_SITE_SIZE", []))
    assert not any(key == f"joint.rs:{push}" for key, _ in d.get("SITE_ZERO", []))


def test_b7b_j3_drops_only_its_own_deleted_census_sites(monkeypatch):
    """B7b-J3: J3's filter touches only J3's entries; the one dropped census function is the deleted leaf_diff."""
    d = decisions(monkeypatch)
    assert all(key for key, _, _ in d["O10_SITE_SIZE"]) and all(key for key, _ in d["O10_ID_AUDIT"])
    census = "source_recovery/j7_rf_census.rs"
    assert "fn leaf_diff(" not in (SOURCE / "core/product_physics/src" / census).read_text()
    sizes = [key for key, _, why in d["O10_SITE_SIZE"] if "J3 (test code" in why]
    # RF-J3 adds the two CANCEL class-label assertions, under the same
    # test-only module and guarded multiplicity-zero rule. U4B-J3 (T4-S5) adds the capability test's two report
    # lines, in the same module at J3's multiplicity zero.
    assert len(sizes) == 15 and all(str(key).startswith(census + ":") for key in sizes)


RP = "core/reporting/result_export/src/retained_precision.rs"


def za_block(snapshot):
    """o10_rules.py's B7b-ZA block, as written, with its own fnkey, over `snapshot`."""
    module = ast.parse((PRICING / "rules/o10_rules.py").read_text())
    helpers = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name == "fnkey"]
    assign = [node for node in module.body if isinstance(node, ast.Assign) and ast.unparse(node.targets[0]) == "_RP"]
    blocks = [node for node in module.body if isinstance(node, ast.If) and "section_allowances" in ast.unparse(node.test)]
    assert len(helpers) == len(assign) == len(blocks) == 1
    namespace = {"_SNAP": str(snapshot), "_os": os, "_re": re, "Resolved": Resolved, "RE_": "core/reporting/result_export/src/",
                 "O10": "O-10 (T4): ", "O10_LOOPS_LAST": []}
    exec(compile(ast.Module(body=helpers + assign + blocks, type_ignores=[]), "o10 B7b-ZA", "exec"), namespace)
    return namespace["O10_LOOPS_LAST"]


@pytest.mark.parametrize("mutation", [None, "loop", "second_conversion"])
def test_b7b_za_bounds_section_allowances_by_m_and_refuses_drift(tmp_path, mutation):
    text = (SOURCE / RP).read_text()
    # O4 S5: the exact arm converts the allowance, the wall and the mill tolerance (three unit_value sites).
    anchor = '            let allowance = unit_value(&section["corrosion_allowance"], Dimension::Length);\n'
    assert text.count(anchor) == 1
    if mutation == "loop":
        text = text.replace(anchor, "            for _ in 0..1 {}\n" + anchor, 1)
    elif mutation == "second_conversion":
        text = text.replace(anchor, anchor + '            let _od = unit_value(&section["outside_diameter"], Dimension::Length);\n', 1)
    target = tmp_path / "snap" / RP
    target.parent.mkdir(parents=True)
    target.write_text(text)
    if mutation is None:
        assert za_block(tmp_path / "snap") == [('iter:list(&model[" "]).iter().all', "m", za_block(tmp_path / "snap")[0][2],
                                                ["retained_precision.rs:section_allowances"])]
    else:
        with pytest.raises(SystemExit, match="B7b-ZA"):
            za_block(tmp_path / "snap")
