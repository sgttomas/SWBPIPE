"""RT-A6 H1 (PROVISIONAL proposal; no governed section or registration yet): the explicit contents selection's pricing
rules (rules/o10_rules.py, "RT-A6 H1").

D1.5 refuses every case's condition_use and D1.3 every condition set and pipe set reference, so on D1:
- H1-ZR: judge's contents_use_findings returns an unallocated Vec::new() at its first statement and lower's
  RetainedContents::take returns None after an empty filter_map collect; restore runs only under Some. The heap edges are
  zero; each call and its frame stay live (stack is AC-004's, not this tool's).
- H1-NS: inspect_applied_self_weight's of_case_in_place and origin read sit inside the lineage closure after the
  `continue` every admitted load takes (W15B-NS).
- H1-BN/H1-TK: by-name false edges are subtracted only where the pinned body or a receiverless associated fn excludes
  the candidate; the exclusions are fatal unless witnessed.
- H1-ID/H1-FV: every new placeholder is classed from source; from_value's two unboundable request Display sites are
  sized 0 under the multiplicity-0 guard.
The mutation controls re-pin a scratch copy of the tool to the mutated bytes, so each premise check, not the full-file
hash, is what refuses the drift. These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import hashlib
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/"
CT = PP + "operating_conditions/contents.rs"
OC = PP + "operating_conditions.rs"
SW = PP + "self_weight.rs"
RM = PP + "retained_memory.rs"
IN = {"bytes": 128, "source": "IN128"}


def decisions(monkeypatch, snapshot=SOURCE, route="L", pricing=PRICING):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(pricing / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def repinned_tool(tmp_path, snap, paths):
    """A scratch copy of the tool whose full-file pins of `paths` name the mutated bytes, so only the premise checks
    can refuse the mutation."""
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("__pycache__", "tests"))
    rules = tool / "rules/o10_rules.py"
    text = rules.read_text(encoding="utf-8")
    for path in paths:
        old, new = sha(SOURCE / path), sha(snap / path)
        assert text.count(old) == 1, path
        text = text.replace(old, new)
    rules.write_text(text, encoding="utf-8")
    return tool


def h1(d, family):
    return [entry for entry in d[family] if "H1-" in str(entry)]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_h1_zeros_loops_and_sizes_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zeros = sorted((str(c).rsplit(":", 1)[-1], str(t).rsplit(":", 1)[-1]) for c, t, why in d["O10_EDGE_ZERO"] if "H1-" in why)
    assert zeros == [("inspect_applied_self_weight", "case_id"), ("inspect_applied_self_weight", "of_case_in_place"),
                     ("judge", "contents_use_findings"), ("lower", "restore"), ("lower", "take")]
    # The call and frame stay live: every H1-ZR warrant says so, the restore zero says why it is never entered.
    assert all("frame stay live" in why for c, t, why in d["O10_EDGE_ZERO"] if "H1-ZR" in why and not str(t).endswith(":restore"))
    assert [str(k).split(":")[0] for k, why in d["O10_SITE_ZERO"] if "H1-NS" in why] == ["self_weight.rs"]
    loops = [(h, b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "H1-L1" in why]
    assert loops == [("iter:model .condition_sets .iter().map", "0", ["operating_conditions/contents.rs:take"]),
                     ("iter:conditions .iter().map", "0", ["operating_conditions/contents.rs:take"])]
    sizes = [(str(k), b) for k, b, why in d["O10_SITE_SIZE"] if "H1-FV" in why]
    assert [b for _, b in sizes] == [0, 0] and all(k.startswith("self_weight.rs:") for k, _ in sizes)


def test_the_by_name_subtractions_are_exact(monkeypatch):
    d = decisions(monkeypatch)
    rules = [r for r in d["CG_RULES"] if r.get("added") == "H1-BN"]
    replaced = sorted((str(r["caller"]).rsplit(":", 1)[-1], r["name"]) for r in rules if "targets" in r)
    assert replaced == [("capture", "index"), ("capture", "is_empty"), ("capture", "next"), ("case_contents_density", "next"),
                        ("contents_evidence_quantity", "get"), ("contents_evidence_quantity", "is_null"),
                        ("contents_evidence_quantity", "len")]
    assert all(r["targets"] == [] for r in rules if "targets" in r)
    excluded = [r for r in rules if "exclude_targets" in r]
    take = str(next(r for r in excluded if str(r["caller"]).endswith(":lower"))["exclude_targets"][0])
    assert take.startswith(RM + ":") and take.endswith(":take")          # lower keeps contents' take, drops the helper
    contents_take = [r for r in excluded if [str(t) for t in r["exclude_targets"]] == [str(r2) for r2 in
                     next(x for x in excluded if not str(x["caller"]).endswith((":lower", ":generate_operations")))["exclude_targets"]]]
    assert len(contents_take) == 35 and all(str(r["exclude_targets"][0]).startswith(CT + ":") for r in contents_take)
    assert not any(str(r["caller"]).endswith(":lower") for r in contents_take)
    go = next(r for r in excluded if str(r["caller"]).endswith(":generate_operations"))
    assert go["name"] == "key" and str(go["exclude_targets"][0]).endswith("functionals.rs:" + str(go["exclude_targets"][0]).rsplit(":", 2)[1] + ":key")


def test_the_new_placeholders_are_classed_from_source(monkeypatch):
    d = decisions(monkeypatch)
    audits = {}
    for key, value in d["O10_ID_AUDIT"]:
        if str(key).startswith((CT + ":", SW + ":")) and any("H1-ID" in str(v) or v == IN for v in value.values()):
            audits.setdefault(str(key), {}).update(value)
    flat = {a: v["bytes"] for value in audits.values() for a, v in value.items()}
    # A contents_diag suffix: at(what) = a stable_suffix'd IN128 pipe id, ':' and "set-unresolved" (14 B).
    assert flat["suffix"] == 128 + 1 + 14 and flat["what"] == 14 and flat['at("normalize")'] == 128 + 1 + 9
    for arg in ("case_id", "pipe", "key", "set", "set_id", "id", "authored.unit", "selection.context.key()", "c.case_id()"):
        assert flat[arg] == 128, arg
    assert sum(1 for k in audits if k.startswith(CT + ":")) == 36


def test_the_b7a_deterministic_identity_audit_follows_its_unchanged_text(monkeypatch):
    model = runpy.run_path(str(PRICING / "base/b7a_formation_model.py"))
    _, audits = model["decisions"](SOURCE, lambda key: key)
    keys = [k for k, _ in audits if k.startswith(SW + ":")]
    assert keys == [SW + ":1519"]
    line = (SOURCE / SW).read_text(encoding="utf-8").split("\n")[1518]
    assert line.strip() == "== format!("


@pytest.mark.parametrize("path, old, new, premise", [
    # contents_use_findings allocates before its no-use return.
    (CT, "pub(crate) fn contents_use_findings(model: &PreviewModel, case: &PreviewLoadCase) -> Vec<Diagnostic> {\n",
     "pub(crate) fn contents_use_findings(model: &PreviewModel, case: &PreviewLoadCase) -> Vec<Diagnostic> {\n    let _probe = case.id.clone();\n",
     "cuf-first"),
    # take builds the sets before its empty-use return.
    (CT, "        if uses.is_empty() {\n            return None;\n        }\n        let sets = model",
     "        let sets = model", "take-first"),
    # lower restores unconditionally.
    (OC, "    if let Some(retained) = retained {\n        retained.restore(model);\n    }\n",
     "    retained.unwrap().restore(model);\n", "lower"),
    # the origin is read before the lineage closure (before the `continue`).
    (SW, "            let looks_generated = claimed_method_family || deterministic_identity;\n",
     "            let looks_generated = claimed_method_family || deterministic_identity;\n"
     "            let _early = CaseContentsContext::of_case_in_place(model, case);\n", "ns"),
    # D1.5 no longer refuses a condition use.
    (RM, "        if case.condition_use.is_some() {\n            return refuse(C::Case, F::ConditionUse);\n        }\n", "", "D1.5"),
])
def test_a_drifted_premise_stops_even_with_its_pin_restated(tmp_path, monkeypatch, path, old, new, premise):
    snap = copy_core(tmp_path / "snap")
    edit(snap, path, old, new)
    tool = repinned_tool(tmp_path, snap, [path] if path in (CT, OC) else [])
    with pytest.raises(SystemExit, match="H1: a premise changed.*" + premise):
        decisions(monkeypatch, snap, pricing=tool)


@pytest.mark.parametrize("probe, premise", [
    # A priced caller of the receiverless take appears: its edge would be real, so the subtraction must stop.
    ("pub(crate) fn h1_probe(m: &crate::PreviewModel) { let _ = crate::operating_conditions::contents::RetainedContents::take(m); }\n",
     "tk-names"),
    # A priced caller of the request reader appears.
    ("pub(crate) fn h1_probe(v: &serde_json::Value) { let _ = crate::self_weight::ContentsAwareSelfWeightRequest::from_value(v); }\n",
     "fv"),
])
def test_a_new_priced_caller_stops(tmp_path, monkeypatch, probe, premise):
    snap = copy_core(tmp_path / "snap")
    (snap / PP / "h1_probe.rs").write_text(probe, encoding="utf-8")
    with pytest.raises(SystemExit, match="H1: a premise changed.*" + premise):
        decisions(monkeypatch, snap)


def test_a_changed_contents_module_stops_at_its_pin(tmp_path, monkeypatch):
    snap = copy_core(tmp_path / "snap")
    edit(snap, CT, '"source-missing"', '"source-absent"')
    with pytest.raises(SystemExit, match="H1-PIN"):
        decisions(monkeypatch, snap)


def test_a_longer_contents_suffix_stops_the_class_even_with_its_pin_restated(tmp_path, monkeypatch):
    snap = copy_core(tmp_path / "snap")
    edit(snap, CT, 'at("set-unresolved")', 'at("set-unresolved-anywhere")')
    tool = repinned_tool(tmp_path, snap, [CT])
    with pytest.raises(SystemExit, match="H1-ID: contents_diag's suffixes changed"):
        decisions(monkeypatch, snap, pricing=tool)


def test_a_take_caller_naming_the_type_stops(tmp_path, monkeypatch):
    # topology.rs's near_duplicate_nodes is one of the 35 subtracted callers; naming RetainedContents in its body
    # (here in a comment-free statement) withdraws the receiverless-fn warrant.
    snap = copy_core(tmp_path / "snap")
    text = (snap / PP / "topology.rs").read_text(encoding="utf-8")
    head = text.index("fn near_duplicate_nodes(")
    brace = text.index("{\n", head) + 2
    (snap / PP / "topology.rs").write_text(text[:brace] + "    let _h1 = std::any::type_name::<crate::operating_conditions::contents::RetainedContents>();\n" + text[brace:], encoding="utf-8")
    with pytest.raises(SystemExit, match="H1"):
        decisions(monkeypatch, snap)
