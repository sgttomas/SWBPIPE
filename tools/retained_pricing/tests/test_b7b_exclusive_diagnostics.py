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


def test_the_group_is_derive_pipe_sections_four_returning_diagnostics(monkeypatch, tmp_path):
    start, sites = derive_pipe_section_sites()
    assert len(sites) == 4
    expected = [("B7b-DPS", f"lib.rs:{start}:derive_pipe_section", [f"lib.rs:{n}" for n in sites])]
    groups = decisions(monkeypatch)["O10_DIAG_EXCLUSIVE"]
    assert [(name, fn, members) for name, fn, members, _ in groups] == expected
    block = ca_block(with_lib(tmp_path, (SOURCE / LIB).read_text()))
    assert [(name, fn, members) for name, fn, members, _ in block["O10_DIAG_EXCLUSIVE"]] == expected
    assert len(block["O10_ID_AUDIT"]) == 6, "two unit-conversion and four section corrosion-allowance ID sites"


@pytest.mark.parametrize("mutation,message", [
    ("no_return", "not followed by `return None;`"),
    ("fifth", "diagnostic inventory changed"),
    ("other_push", "producer other than a pushed diag"),
    ("bare_diag", "producer other than a pushed diag"),
    ("callee", "uses `diagnostics` other than in a pushed diag"),
])
def test_a_changed_premise_refuses(tmp_path, mutation, message):
    text = (SOURCE / LIB).read_text()
    nonzero = '"nonzero corrosion allowance is not yet supported",\n'
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
    assert [key for key, _ in d["O10_MULT0_SITES"]] == [f"joint.rs:{push}"]
    assert not any(str(key).startswith("joint.rs:") and key == f"joint.rs:{push}" for key, _, _ in d.get("O10_SITE_SIZE", []))
    assert not any(key == f"joint.rs:{push}" for key, _ in d.get("SITE_ZERO", []))


def test_b7b_j3_drops_only_its_own_deleted_census_sites(monkeypatch):
    """B7b-J3: J3's filter touches only J3's entries; the one dropped census function is the deleted leaf_diff."""
    d = decisions(monkeypatch)
    assert all(key for key, _, _ in d["O10_SITE_SIZE"]) and all(key for key, _ in d["O10_ID_AUDIT"])
    census = "source_recovery/j7_rf_census.rs"
    assert "fn leaf_diff(" not in (SOURCE / "core/product_physics/src" / census).read_text()
    sizes = [key for key, _, why in d["O10_SITE_SIZE"] if "J3 (test code" in why]
    assert len(sizes) == 11 and all(str(key).startswith(census + ":") for key in sizes)
