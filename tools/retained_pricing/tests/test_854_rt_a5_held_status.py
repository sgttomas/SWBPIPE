"""RT-A5 minuend_final, the held-status solve (retained-pricing-rules.md §8.54).

- RTA5: the held solve's rules are carried on both routes. D1.4 refuses every difference basis, so no combination is
  minuend_final: the held solve, the held source capture and its frame input, the held reader's record checks and the
  by-name false edges into the new modules are zero edges. The held closures' calls and the released notice's calls
  restate §8.53's per-call counts.
- RTA5-ID: the held key is the one positive-multiplicity new TEXT site (an IN128 combination id per combination).
- A changed held module, or a second held solve call, stops the run.
- bound_held_gate (the roster's whole-message exemption, reached from member_check) forms no text.

These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import re
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
LIB = PP + "lib.rs"
H_PP = PP + "held_status.rs"
H_RE = RE + "preview_held_status.rs"


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def _name(key):
    return str(key).rsplit(":", 1)[-1]


def _file(key):
    return str(key).split("/src/")[-1].split(":")[0]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_held_status_rules_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    calls = {(_name(e[0]), _name(e[1])): e[2] for e in d["EDGE_PER_CALL"] if "RTA5" in str(e[3])}
    # The held closures' calls restate §8.53's sites: three case solves per case, one render, one key per case.
    assert calls[("run_linear_static_preview_observed_inner", "solve_load_case_observed")] == "3*cases"
    assert calls[("run_linear_static_preview_observed_inner", "render")] == "1"
    assert calls[("run_linear_static_preview_observed_inner", "modulus_basis_key")] == "cases"
    # The released notice is a seventh notice site under a held operand; §8.53's six are restated.
    assert calls[("append_combination_results", "diag")] == calls[("append_combination_results", "identity")] == "5*z+z*P"
    zero = {(_file(a), _name(a), _file(b), _name(b)) for a, b, *_ in d["O10_EDGE_ZERO"]}
    for edge in [("lib.rs", "run_linear_static_preview_observed_inner", "held_status.rs", "solve_held_combinations"),
                 ("lib.rs", "solve_prepared_load_case", "held_status.rs", "capture_held_status_source"),
                 ("lib.rs", "solve_prepared_load_case", "lib.rs", "nonlinear_frame_input"),
                 ("preview_held_status.rs", "check", "preview_held_status.rs", "read"),
                 ("preview_physics.rs", "gate_reason", "retained_wire.rs", "refusal"),
                 ("held_status.rs", "solve_held_combinations", "source_recovery/j7_rf_census.rs", "run")]:
        assert edge in zero, edge


def test_the_held_key_is_the_one_priced_site_at_in128(monkeypatch):
    d = decisions(monkeypatch)
    line = next(i + 1 for i, text in enumerate((SOURCE / PP / "preview_physics.rs").read_text(encoding="utf-8").split("\n"))
                if 'let held_key = format!("{}\\u{0}held", combination.id);' in text)
    audits = {str(site): table for site, table in d["O10_ID_AUDIT"]}
    assert audits[f"{PP}preview_physics.rs:{line}"]["combination.id"]["bytes"] == 128
    # Every other held branch site is zero (RTA5-DZ), the held key's own line is not.
    zeros = {str(site) for site, *_ in d["O10_SITE_ZERO"]}
    assert f"preview_physics.rs:{line}" not in zeros
    assert f"preview_physics.rs:{line + 41}" in zeros   # the held rows' key copy inside `if let Some(operand)`


def test_a_changed_held_module_stops(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    edit(snap, H_PP, "pub(crate) fn is_minuend_final(", "pub(crate) fn  is_minuend_final(")
    with pytest.raises(SystemExit, match="RTA5-PIN"):
        decisions(monkeypatch, snap)


def test_a_changed_held_reader_stops(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    edit(snap, H_RE, "    if !claimed(source) {\n        return Ok(());\n    }\n",
         "    if !claimed(source) {\n        return Ok(());\n    }\n\n")
    with pytest.raises(SystemExit, match="RTA5-PIN"):
        decisions(monkeypatch, snap)


def test_a_second_held_solve_call_stops(tmp_path, monkeypatch):
    """lib.rs is pinned whole (KD5C4's B7a E14 pin runs first); RTA5-DZ's statement counts are the second line."""
    snap = copy_core(tmp_path)
    guard = "model.combinations.iter().any(held_status::is_minuend_final) {\n        held_status::solve_held_combinations("
    edit(snap, LIB, guard, "true {\n        held_status::solve_held_combinations(")
    with pytest.raises((SystemExit, ValueError), match="KD5C4|RTA5"):
        decisions(monkeypatch, snap)


def test_bound_held_gate_forms_no_text():
    text = (SOURCE / RE / "combination_certificate.rs").read_text(encoding="utf-8")
    start = text.index("\nfn bound_held_gate(")
    body = text[start:text.index("\n}\n", start)]
    assert not re.search(r"format!|to_string|to_owned|String::|collect|Vec|clone\(\)|join\(", body)
    assert "strip_prefix(\"Combination withheld: \")" in body and "strip_suffix('.')" in body
