"""W1 source/lifetime bounds and shared U6-RB/L5 guard refuse drift."""
import ast
import os
from pathlib import Path
import re
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/w1_reductions_model.py"))
PHASE = runpy.run_path(str(PRICING / "base/b7a_phase_model.py"))


def snapshot(tmp_path):
    for relative in MODEL["REDUCED_PINS"]:
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    return tmp_path


def test_current_reduced_cohort_is_source_bound():
    assert MODEL["inspect"](SOURCE) is True


@pytest.mark.parametrize("relative", list(MODEL["REDUCED_PINS"]))
def test_every_producer_consumer_and_lifetime_pin_refuses_drift(tmp_path, relative):
    root = snapshot(tmp_path)
    target = root / relative
    target.write_text(target.read_text() + "\n// unreviewed owner change\n")
    with pytest.raises(ValueError, match="source/lifetime pin"):
        MODEL["inspect"](root)


class Resolved(str):
    pass


def rb_l5_block(root):
    """Execute the maintained rules' actual block and actual key resolvers."""
    path = PRICING / "rules/o10_rules.py"
    module = ast.parse(path.read_text())
    helpers = [n for n in module.body if isinstance(n, ast.FunctionDef)
               and n.name in ("fnkey", "site", "fn_lines", "_fn_text")]
    first = next(i for i, n in enumerate(module.body) if isinstance(n, ast.Assign)
                 and ast.unparse(n.targets[0]) == "(U6_RB_PATCHES, W1_L5_PATCHES, W1_L5_SITE_FROM)")
    body = module.body[first:first + 3]
    assert len(helpers) == 4 and isinstance(body[-1], ast.If)
    namespace = {"__file__": str(path), "_SNAP": str(root), "_os": os, "_re": re,
                 "Resolved": Resolved, "_LIB": MODEL["LIB"], "O10": "O-10 (T4): "}
    exec(compile(ast.Module(body=helpers + body, type_ignores=[]), "U6-RB/W1-L5", "exec"), namespace)
    return namespace


def test_u6_rb_and_l5_guard_pin_binds_the_current_text():
    state = rb_l5_block(SOURCE)
    assert len(state["U6_RB_PATCHES"]) == 1
    assert len(state["W1_L5_PATCHES"]) == 2
    assert len(state["W1_L5_SITE_FROM"]) == 1
    assert state["W1_L5_SITE_FROM"][0][1]["per"] == "min(z, 1) * cases * P"
    assert 'V("ResultItem", PA)' in state["U6_RB_PATCHES"][0][2]
    assert 'if _mc.get("z", 0)' in state["W1_L5_PATCHES"][0][2]
    assert 'if _mc.get("z", 0)' in state["W1_L5_PATCHES"][1][2]


@pytest.mark.parametrize("old,new", [
    (MODEL["GUARD"], "    let combining = preview.is_some();\n"),
    (MODEL["GUARD"], ""),
    (MODEL["INSERT"], MODEL["INSERT"].replace("if combining", "if preview.is_some()")),
    (MODEL["CALL"], MODEL["CALL"].replace("if combining", "if preview.is_some()")),
    (MODEL["SHADOW"], MODEL["SHADOW"].replace("#[cfg(test)]", "#[allow(dead_code)]")),
])
def test_u6_rb_missing_former_or_drifted_guard_refuses(tmp_path, old, new):
    root = snapshot(tmp_path)
    target = root / MODEL["LIB"]
    text = target.read_text()
    assert text.count(old) == 1
    target.write_text(text.replace(old, new))
    with pytest.raises((ValueError, SystemExit)):
        rb_l5_block(root)
    with pytest.raises(ValueError, match="U6-RB/L5"):
        MODEL["guard"](target.read_text())


def bodies(text, name):
    out = []
    for hit in re.finditer(r"^( *)(?:pub(?:\([a-z]+\))? )?fn " + name + r"\b", text, re.M):
        end = text.index("\n" + hit[1] + "}", hit.start())
        out.append(text[hit.start():end + len(hit[1]) + 2])
    return out


def test_force_scaled_owner_drops_at_the_scaled_evaluation_return():
    # Full file pins bind every caller and returned type as well as the two
    # entries. These lexical assertions state the reviewed ownership warrant:
    # only the nonzero branch constructs `owned`, borrowed only by the solve;
    # neither it nor its primitive/manifest children can escape either method.
    assert MODEL["inspect"](SOURCE)
    text = (SOURCE / MODEL["NI"]).read_text()
    entries = bodies(text, "solve_force_scaled_with_formation_check")
    assert len(entries) == 2
    for body in entries:
        assert body.count("let owned;") == 1
        assert "if self.force_scale.is_unscaled() {\n            f\n        } else {\n            owned = f" in body
        assert body.count(".force_scaled(self.force_scale)") == 1
        assert body.endswith("force_scaled_outcome(solved, self.force_scale, f)\n    }")
    caller, = bodies(text, "solve_with_force_scaling")
    first = caller.index("evaluate_force_scaled(case, ForceScale::UNSCALED)")
    second = caller.index("evaluate_force_scaled(case, force_scale)")
    assert first < second and caller.count("evaluate_force_scaled(") == 2
    outcome = text[text.index("pub struct ForceScalingOutcome {"):]
    outcome = outcome[:outcome.index("\n}")]
    assert "pub solution: ForceScaledSolution" in outcome and "pub stiffness: SparseStiffness" in outcome
    assert "AssembledForce" not in outcome


def test_uniform_constructor_staging_has_one_sequential_call_and_attach_term(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    monkeypatch.setenv("RP_SNAPSHOT", str(SOURCE))
    context = {"route": "L", "branch": "ordinary"}
    price = lambda count, owner: PHASE["formed_owner"](context, 3, 3, count, 128, owner=owner, branch="ordinary")
    # The bound is per case, independent of a positive load count. Returned
    # formation expansions and primitive/manifest owners still grow with q.
    assert price(1, "uniform_constructor") == price(128, "uniform_constructor")
    assert price(1, "typed_source_children") == price(128, "typed_source_children")
    assert not price(128, "recovery_source_children")
    assert not price(0, "uniform_constructor")
    assert price(128, "assembled_children") != price(1, "assembled_children")
    text = (SOURCE / MODEL["LIB"]).read_text()
    body, = bodies(text, "add_uniform_element_loads")
    assert body.count(".equivalent_global_nodal_loads_with_spans_formed(&[global])") == 1
    assert ".zip(formations)" in body
    assert body.index(".zip(formations)") < body.index('certified_loads::attach_last(ledger, certified_loads::summand("uniform_straight"')


@pytest.mark.parametrize("replacement", ["", "    let combining = preview.is_some();\n"])
def test_missing_or_former_guard_is_driver_exit_4(tmp_path, monkeypatch, capsys, replacement):
    """The real guard's refusal passes through cmd_run's rules-stage exit 4.

    Stub successful archive/key-carry/inventory stages, without a full pricing
    run; execute the actual maintained U6-RB/L5 block at the rules stage.
    """
    import importlib.util
    import json
    import subprocess
    from types import SimpleNamespace
    monkeypatch.syspath_prepend(str(PRICING))
    spec = importlib.util.spec_from_file_location("w1_price_driver_test", PRICING / "price.py")
    driver = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(driver)
    root = snapshot(tmp_path / "source")
    target = root / MODEL["LIB"]
    target.write_text(target.read_text().replace(MODEL["GUARD"], replacement))
    output = tmp_path / "price"
    monkeypatch.setattr(driver, "git", lambda *args, **kw: "0" * 40 if kw.get("text") else b"")
    monkeypatch.setattr(driver, "resolve_revision", lambda repo, rev: "0" * 40)
    monkeypatch.setattr(driver, "snapshot", lambda *args: str(root))
    monkeypatch.setattr(driver, "prepare_b7a_retired_carry", lambda *args: ({}, None))
    monkeypatch.setattr(driver, "template_inventory", lambda *args: None)
    def step(args, **kwargs):
        if str(args[0]).endswith("linemap_rules.py"):
            mapped = output / "map"
            mapped.mkdir()
            for relative in driver.RULE_FILES:
                shutil.copyfile(PRICING / "base" / relative, mapped / relative)
            (mapped / "premise_pins.json").write_text(json.dumps({"pins": []}))
            return subprocess.CompletedProcess(args, 0, "", "")
        assert str(args[0]).endswith("sq2_rules.py")
        try:
            rb_l5_block(root)
        except (ValueError, SystemExit) as error:
            return subprocess.CompletedProcess(args, 1, "", str(error))
        pytest.fail("the mutated guard reached the pricing points")
    monkeypatch.setattr(driver, "py", step)
    with pytest.raises(SystemExit) as result:
        driver.cmd_run(SimpleNamespace(repo=str(SOURCE), rev="bad-guard", out=str(output)))
    assert result.value.code == 4
    assert '"code": 4' in capsys.readouterr().out


def test_carried_first_return_keys_name_the_actual_functions_and_format():
    model = runpy.run_path(str(PRICING / "base/b7a_formation_model.py"))
    lines = (SOURCE / model["FIRST_RETURN_FILE"]).read_text().splitlines()
    for key in (model["FIRST_RETURN_ROOT"], model["FIRST_RETURN_PARENT"]):
        _, line, name = key.rsplit(":", 2)
        assert re.search(r"\bfn " + name + r"\b", lines[int(line) - 1])
    assert "format!(" in lines[model["FIRST_RETURN_LINE"] - 1]


def test_carried_owned_error_sites_and_callers_keep_their_source_identity():
    model = runpy.run_path(str(PRICING / "base/b7a_returning_errors.py"))
    for site in model["SITES"]:
        for key in [site["owner"]] + site["parents"]:
            relative, line, name = key.rsplit(":", 2)
            text = (SOURCE / relative).read_text().splitlines()
            assert re.search(r"\bfn " + name + r"\b", text[int(line) - 1])
        line = (SOURCE / site["file"]).read_text().splitlines()[site["line"] - 1]
        token = {"format": "format!(", "into_text": ".into(", "to_string": ".to_string("}[site["kind"]]
        assert token in line


def test_carried_helper_allocations_are_at_the_actual_source_sites():
    model = runpy.run_path(str(PRICING / "base/b7a_owner_choices.py"))
    for row in model["PREIMAGES"]["helper_rows"]:
        relative, line, name = row["fn"].rsplit(":", 2)
        lines = (SOURCE / relative).read_text().splitlines()
        assert re.search(r"\bfn " + name + r"\b", lines[int(line) - 1])
        token = {"format": "format!(", "replace": ".replace("}[row["kind"]]
        assert token in lines[row["line"] - 1]
