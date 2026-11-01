"""§8.36 (retained-pricing-rules.md): model document 0.9.0's S1 rules and owners. The rules bound the lowering's and the
withholding's loops and texts on D1 under guarded premises (the module pins, D1.3-D1.6), give the by-name false edges
their actual targets, price the remaining owners (S1-VW, S1-WH) and the typed-string census (S1-TS). A changed premise
stops the run."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
DM = "core/product_physics/src/document_members.rs"
LIB = "core/product_physics/src/lib.rs"
sys.path.insert(0, str(PRICING / "base"))


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_s1_rules_are_present_with_their_bounds(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {(r[0], r[1]) for r in d["O10_LOOPS_LAST"] + d["O10_LOOPS_FIRST"] if "S1-" in r[2]}
    assert ("for (index, set) in sets.iter().enumerate()", "0") in loops
    assert ("for (index, spectrum) in spectra.iter().enumerate()", "0") in loops
    assert ("for finding in found.into_vec()", "4*z") in loops
    assert ("for (code, message) in &reasons", "4") in loops
    assert not any(h == 'for key in [" ", " "]' for h, _ in loops), "case_observables shares this header"
    bn = [r for r in d["CG_RULES"] if r.get("added") == "S1-BN"]
    assert len(bn) == 27 and all(r["targets"] in ([],) or len(r["targets"]) in (1, 2) for r in bn)
    push = [r for r in bn if r["name"] == "push"]
    assert len(push) == 1 and push[0]["targets"] == [] and push[0]["caller"].endswith(":push")
    assert sum("S1-ZR" in why or "S1-BN" in why for _, _, why in d["O10_EDGE_ZERO"]) == 17
    assert sum("S1-ZR" in why for _, why in d["O10_SITE_ZERO"]) >= 30


@pytest.mark.parametrize("path, old, new, stop", [
    (DM, "pub const OPERATING_MINUS_SUSTAINED", "pub const  OPERATING_MINUS_SUSTAINED", "S1-PIN"),
    ("core/product_physics/src/model_fields.rs", 'const SPECTRUM_QUANTITIES: [&str; 1] = ["g_value"];',
     'const SPECTRUM_QUANTITIES: [&str; 2] = ["g_value", "x"];', "SD-NR"),
])
def test_a_changed_premise_stops(monkeypatch, tmp_path, path, old, new, stop):
    with pytest.raises(SystemExit, match=stop):
        decisions(monkeypatch, snapshot_with(tmp_path, path, old, new))


def test_the_typed_string_census_reads_thirteen_pipe_strings_and_four_model_strings(tmp_path):
    import typed_strings_model as ts
    assert ts.inspect(str(SOURCE)) == {"pipe": 13, "model": 4}
    assert ts.inspect(None) is None
    added = snapshot_with(tmp_path, LIB, "    pub condition_set_ref: Option<String>,\n    pub material: String,",
                          "    pub condition_set_ref: Option<String>,\n    pub tag_text: String,\n    pub material: String,")
    with pytest.raises(SystemExit, match="S1-TS"):
        ts.inspect(str(added))


def test_the_s1_owners_bind_the_modules_and_price_positively(tmp_path):
    import s1_owners_model as owners
    from g3lib import ASSUMED
    assert owners.inspect(str(SOURCE)) is True
    flat = owners.views_and_findings(32, 3, 32, 192, 0)
    with_combinations = owners.views_and_findings(32, 3, 32, 192, 2)
    assert with_combinations.ev(ASSUMED) > flat.ev(ASSUMED) > 0
    assert owners.withholding(3, 2).ev(ASSUMED) > owners.withholding(3, 0).ev(ASSUMED) > 0
    changed = snapshot_with(tmp_path, LIB, "    document_members::withhold_interim_combinations(&mut model, &mut diagnostics);\n",
                            "    document_members::withhold_interim_combinations(&mut model, &mut diagnostics);\n    let _ = 0;\n")
    assert owners.inspect(str(changed)) is True
    moved = snapshot_with(tmp_path / "b", DM, "fn reachable(edges", "fn reachable_from(edges")
    with pytest.raises(SystemExit, match="S1 owners"):
        owners.inspect(str(moved))


def test_flattened_collect_capacity_and_support_refs_backing_are_priced():
    import s1_owners_model as owners
    # The pinned combination_view source collects through flatten(), whose
    # zero lower size hint allocates four slots for one to three operands.
    assert owners._view_children()["s(&str)"] == 4
    assert owners._view_children()["s(TermView)"] == 3
    # supports_findings allocates this one-entry Vec even without combinations;
    # its String payload is TEXT, but its String header backing is a heap owner.
    assert owners.views_and_findings(32, 3, 32, 192, 0)["s(String)"] == 1
