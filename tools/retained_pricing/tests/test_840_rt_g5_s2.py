"""§8.40: model document 0.9.0 S2's pressure-free resolution on the retained route. D1.3
and D1.5 leave plan with an empty set map and every resolve_case returning Ok(None) at its first statement (S2-ZR);
judge's refused-document conversion owns one exact-size Vec<Diagnostic> (S2-JD); the moved S1 rules follow judge
(S2-RK); plan's by-name `as_str` has no crate target (S2-BN); the resolution's unreached format arguments keep their
classes (S2-ID). A changed module or premise stops the run, including a premise change under a restated pin."""
from pathlib import Path
import hashlib
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/"
RES = PP + "operating_conditions/resolution.rs"
OC = PP + "operating_conditions.rs"
CD = PP + "case_state/temperature/canonical_decimal.rs"
sys.path.insert(0, str(PRICING / "base"))


def decisions(monkeypatch, snapshot=SOURCE, route="L", tool=PRICING):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(tool / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def tool_with_pin(tmp_path, path, snapshot):
    """A scratch copy of the tool whose S2-PIN literal is restated to `snapshot`'s bytes of `path`."""
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("tests", "__pycache__"))
    rules = tool / "rules/o10_rules.py"
    old = hashlib.sha256((SOURCE / path).read_bytes()).hexdigest()
    new = hashlib.sha256((snapshot / path).read_bytes()).hexdigest()
    text = rules.read_text(encoding="utf-8")
    assert text.count(old) == 1
    rules.write_text(text.replace(old, new), encoding="utf-8")
    return tool


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_s2_rules_are_present_with_their_bounds(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zr = [(a, b) for a, b, why in d["O10_EDGE_ZERO"] if "S2-ZR" in why]
    assert [(a.rsplit(":", 1)[-1], b.rsplit(":", 1)[-1]) for a, b in zr] == [("plan", "resolve_case")]
    bn = [r for r in d["CG_RULES"] if r.get("added") == "S2-BN"]
    assert len(bn) == 1 and bn[0]["name"] == "as_str" and bn[0]["targets"] == [] and bn[0]["caller"].endswith(":plan")
    # S2-RK: the findings conversion keeps S1-L1's 4z, now in judge, placed before the broad base rules.
    first = [r for r in d["O10_LOOPS_FIRST"] if r[0] == "iter:diagnostics: found.into_vec().into_iter().map"]
    assert [(r[1], r[3]) for r in first] == [("4*z", ["operating_conditions.rs:judge"])]
    assert not any(r[0] == "for finding in found.into_vec()" for r in d["O10_LOOPS_FIRST"] + d["O10_LOOPS_LAST"])
    moved = {b.rsplit(":", 1)[-1] for a, b, why in d["O10_EDGE_ZERO"] if "S1-ZR" in why and a.endswith(":judge")}
    assert moved == {"interim_support_refusals", "interim_exact_combination_refusals"}
    # S2-ID: a JCS id or key and the longest static suffix.
    audits = {k.rsplit(":", 1)[0].split("/src/")[-1] + ":" + "|".join(sorted(v)): v for k, v in d["O10_ID_AUDIT"] if RES in k}
    jcs = [v for v in audits.values() if "jcs_string(set)" in v]
    assert len(jcs) == 2 and all(x["bytes"] == 2 + 6 * 128 for v in jcs for x in v.values())
    what = [v for v in audits.values() if "what" in v]
    assert len(what) == 1 and what[0]["what"]["bytes"] == len("installation-missing")
    # The closed wind wire is among the closed records SD-L1 counts.
    assert "WindGenerationWire" in d["_SD_RECORDS"]


@pytest.mark.parametrize("path, old, new, stop", [
    (RES, "pub const RESOLUTION_METHOD", "pub const  RESOLUTION_METHOD", "S2-PIN"),
    (OC, "pub(crate) fn judge(model: &PreviewModel) -> Judgement {", "pub(crate) fn judge(model: &PreviewModel)  -> Judgement {", "S1-PIN"),
    (CD, "fn bit_len(&self) -> u32 {", "fn bit_len(&self)  -> u32 {", "TIM-PF/RD/CD-ID"),
])
def test_a_changed_module_stops(monkeypatch, tmp_path, path, old, new, stop):
    with pytest.raises(SystemExit, match=stop):
        decisions(monkeypatch, snapshot_with(tmp_path, path, old, new))


@pytest.mark.parametrize("old, new", [
    # Work before the condition_use return.
    ("    let Some(using) = case.condition_use.as_ref() else {\n        return Ok(None);\n    };\n    let id = case.id.as_str();",
     "    let id = case.id.as_str();\n    let Some(using) = case.condition_use.as_ref() else {\n        return Ok(None);\n    };"),
    # A second caller of resolve_case.
    ("        match resolve_case(model, &sets, index, case) {",
     "        let _ = resolve_case(model, &sets, index, case);\n        match resolve_case(model, &sets, index, case) {"),
    # The plan's loop no longer over the authored cases.
    ("    for (index, case) in model.load_cases.iter().enumerate() {",
     "    for (index, case) in model.load_cases.iter().rev().enumerate() {"),
])
def test_a_changed_premise_stops_under_a_restated_pin(monkeypatch, tmp_path, old, new):
    changed = snapshot_with(tmp_path, RES, old, new)
    tool = tool_with_pin(tmp_path, RES, changed)
    with pytest.raises(SystemExit, match="S2-PIN: a premise"):
        decisions(monkeypatch, changed, tool=tool)


def test_the_restated_pin_alone_is_accepted(monkeypatch, tmp_path):
    changed = snapshot_with(tmp_path, RES, "/// The `method` of every", "/// The  `method` of every")
    tool = tool_with_pin(tmp_path, RES, changed)
    d = decisions(monkeypatch, changed, tool=tool)
    assert any("S2-ZR" in why for _, _, why in d["O10_EDGE_ZERO"])


def test_the_s2_owner_is_the_refused_documents_diagnostic_backing(tmp_path):
    import s1_owners_model as owners
    from g3lib import ASSUMED
    assert owners.inspect(str(SOURCE)) is True
    assert "s(Diagnostic)" not in owners.views_and_findings(32, 3, 32, 192, 0)
    for z in (1, 2):
        assert owners.views_and_findings(32, 3, 32, 192, z)["s(Diagnostic)"] == 4 * z
    assert owners.views_and_findings(32, 3, 32, 192, 2).ev(ASSUMED) > owners.views_and_findings(32, 3, 32, 192, 1).ev(ASSUMED)
    with pytest.raises(SystemExit, match="§8.40's owners"):
        owners.inspect(str(snapshot_with(tmp_path, RES, "pub const RESOLUTION_METHOD", "pub const  RESOLUTION_METHOD")))


def test_the_s2_owner_follows_the_judge_conversion(monkeypatch, tmp_path):
    import s1_owners_model as owners
    changed = snapshot_with(tmp_path, OC, ".map(|finding| finding.into_diagnostic(\"blocking\")).collect(),",
                            ".map(|finding| finding.into_diagnostic(\"blocking\")).collect::<Vec<_>>(),")
    restated = dict(owners.PINS)
    restated[OC] = hashlib.sha256((changed / OC).read_bytes()).hexdigest()
    monkeypatch.setattr(owners, "PINS", restated)
    with pytest.raises(SystemExit, match="S2-JD"):
        owners.inspect(str(changed))
