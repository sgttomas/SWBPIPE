"""PR-A, the integrity-diagnostic repair (§8.62).

- PRA-IO: the blocked rewrite's loop is bounded by the integrity diagnostics that can carry CHECKS_PASSED or SENSITIVE
  (2*cases); its message, code and constants are site-audited; the old and new messages are both charged.
- PRA-CF: the corrected displacement's failure texts are charged positively (no zero premise): the Option chain runs
  once, the Debug text at most once per call, at a bound derived from the kernel's error types and literals.
- PRA-TR, PRA-NL: the trigger's prior read and route E's direct require are bound by their exact text.
- U5B-PD-RN: the owner's two reviewed shapes, one present; anything else stops.
- OWN-PRES and O10-GATE: an owner or a rule target that a current source loses, renames or moves stops the run; it is
  never priced as older source.

These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
FK = "core/solver/frame_kernel/src/"
LIB = PP + "lib.rs"
U45 = runpy.run_path(str(PRICING / "base/u4b_u5b_owners_model.py"))
CF = runpy.run_path(str(PRICING / "base/correction_failure_model.py"))
PRES = runpy.run_path(str(PRICING / "base/owner_presence.py"))
G = runpy.run_path(str(PRICING / "base/pra_guards.py"))


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


def _key(entry):
    return str(entry).split("/src/")[-1]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_pr_a_rules_are_positive_bounds(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {h: b for h, b, why, fns in d["O10_LOOPS_LAST"] if "PRA-" in why}
    assert loops == {
        "for diagnostic in diagnostics.iter_mut().filter(|d| d.id.starts_with(INTEGRITY_DIAGNOSTIC_PREFIX))": "2*cases",
        "iter:linear_bound .or(confirmed_candidate.map(|state| &state.member_bound)) "
        ".filter(|_| displacements.iter().all(|value| value.is_finite())).map": "1",
        "iter:corrected_displacement_tests::FORCED_CORRECTION_FAILURE.with(|forced| forced.get()) { return Err("
        "DisplacementCorrectionFailure { dof, kind, error: \" \".to_string() }); } (0..displacements.len()).map": "N",
    }
    froms = [entry for key, entry in d["O10_SITE_FROM"] if "PRA-CF" in entry["why"]]
    assert len(froms) == 1 and froms[0]["per"] == "1"
    assert [str(x).rsplit(":", 1)[-1] for x in froms[0]["from"]] == ["corrected_nodal_displacements"]
    audits = [(k, v) for k, v in d["O10_ID_AUDIT"] if any("PRA-" in str(x.get("source", "")) for x in v.values())]
    # Every correction and rewrite text is site-audited with a positive bound; none is zero: the rewrite, the Debug
    # text, the failed clause, the sentence, the warning message (with its IN128 case id) and route E's require. The
    # warning id, its ref and the test probe take IN128 (source "IN128").
    assert len(audits) == 6
    in128 = [v for k, v in d["O10_ID_AUDIT"] if v in ({"case_id": {"bytes": 128, "source": "IN128"}},
                                                     {"load_case.id": {"bytes": 128, "source": "IN128"}})]
    assert len(in128) >= 3
    assert all(x["bytes"] > 0 for _, v in audits for x in v.values())
    bounds = CF["inspect"](SOURCE)
    debug = [v["error|?"]["bytes"] for k, v in audits if "error|?" in v]
    assert debug == [bounds["debug"]] and bounds["debug"] >= bounds["static_literal"]
    template = "the corrected displacement of {} could not be {step} ({})"
    assert bounds["failed"] == CF["literal_bytes"](template) + bounds["dof_label"] + bounds["step"] + bounds["debug"]


def test_correction_failure_bounds_follow_the_kernel_source(tmp_path):
    base = CF["inspect"](SOURCE)
    assert 0 < base["debug"] < base["failed"] < base["sentence"]
    # The Debug bound is derived from the kernel's literals, not assumed: a longer literal anywhere in the kernel's
    # production files raises it by exactly the difference.
    snap = copy_core(tmp_path)
    longer = base["static_literal"] - 2 + 100
    target = snap / FK / "connector.rs"
    target.write_text(target.read_text(encoding="utf-8") + f'\npub const PRA_CF_TEST_SITE: &str = "{"z" * longer}";\n',
                      encoding="utf-8")
    grown = CF["inspect"](snap)
    assert grown["static_literal"] == longer + 2 and grown["debug"] - base["debug"] == grown["static_literal"] - base["static_literal"]
    # A kernel that builds &'static str at run time cannot be bounded by its literals.
    target.write_text(target.read_text(encoding="utf-8") + "\nfn leaked(s: String) -> &'static str { s.leak() }\n", encoding="utf-8")
    with pytest.raises(SystemExit, match="leak"):
        CF["inspect"](snap)


@pytest.mark.parametrize("path,old,new", [
    # CF-DBG: the error type gains a field the bound does not read.
    (FK + "structural/arc_certificate.rs", "    AtanArgumentNotPositive,\n", "    AtanArgumentNotPositive,\n    Named(String),\n"),
    (FK + "structural/retained/wide.rs", "    NonFinite,\n", "    NonFinite,\n    Detail(String),\n"),
    (FK + "structural/formation_check.rs", "    pub lambda_sigma: f64,\n", "    pub lambda_sigma: f64,\n    pub site: &'static str,\n"),
    # CF-CLONE: a second owned String in the cloned failure.
    (LIB, "    error: String,\n}\n", "    error: String,\n    context: String,\n}\n"),
    # The Debug text, the step literals, the templates and the warning's ref.
    (LIB, 'error: format!("{error:?}")', 'error: format!("{error:#?}")'),
    (LIB, 'DisplacementFailureKind::Represent => "represented in binary64",', 'DisplacementFailureKind::Represent => "not represented",'),
    (LIB, "could not be {step} ({})", "could not be {step} ({}) at all"),
    (LIB, "vec![case_id.to_string()],\n    )\n}\n", "vec![case_id.to_string(), case_id.to_string()],\n    )\n}\n"),
])
def test_a_changed_failure_text_or_type_stops(tmp_path, path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit):
        CF["inspect"](snap)


def _lib():
    return (SOURCE / LIB).read_text(encoding="utf-8")


def _mutant(text, old, new):
    assert text.count(old) == 1, old
    return text.replace(old, new)


CENSUS = {("lib.rs", "diagnostic:numerical-integrity:"): 2}


def test_the_pr_a_guards_hold_at_the_current_source(monkeypatch):
    d = decisions(monkeypatch)   # the census is computed by the rules from the whole product crate
    assert G["io_failures"](_lib(), G["IO_CENSUS"]) == []
    assert G["cf_failures"](_lib()) == [] and G["parts_failures"]({"a": True}, "corrected") == []


@pytest.mark.parametrize("label,old,new", [
    # PRA-IO: the early return, the loop, a second push or another code in append_integrity_report, a passing failure code.
    ("IO-BODY", '    if !matches!(diagnostic.code.as_str(), "NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE") {\n        return;\n    }\n', ""),
    ("IO-LOOP", "        inspection_only(diagnostic);\n    }\n", "        inspection_only(diagnostic);\n        inspection_only(diagnostic);\n    }\n"),
    ("IO-REPORT-PUSH", "    if let (Some(finding), Some(record)) = (formation, diagnostics.last_mut()) {\n",
     "    diagnostics.push(diagnostics[0].clone());\n    if let (Some(finding), Some(record)) = (formation, diagnostics.last_mut()) {\n"),
    ("IO-REPORT-CODE", '        "NUMERICAL_INTEGRITY_SENSITIVE"\n    } else {\n', '        "NUMERICAL_INTEGRITY_UNRESOLVED"\n    } else {\n'),
    ("IO-FAILURE-CODES", "fn integrity_failure_code(error: &StructuralError) -> &'static str {\n",
     "fn integrity_failure_code(error: &StructuralError) -> &'static str {\n    if false { return \"NUMERICAL_INTEGRITY_CHECKS_PASSED\"; }\n"),
])
def test_a_changed_rewrite_text_stops(label, old, new):
    failed = G["io_failures"](_mutant(_lib(), old, new), G["IO_CENSUS"])
    assert any(f.startswith(label) for f in failed), failed


def test_a_further_integrity_producer_stops_the_census():
    census = dict(G["IO_CENSUS"])
    census[("lib.rs", "integrity_diagnostic_id(")] += 1
    assert any(f.startswith("IO-CENSUS") for f in G["io_failures"](_lib(), census))
    census = dict(G["IO_CENSUS"])
    census[("pressure_runtime.rs", "\"NUMERICAL_INTEGRITY_SENSITIVE\"")] = 1
    assert any(f.startswith("IO-CENSUS") for f in G["io_failures"](_lib(), census))


def test_the_rules_census_reads_every_product_file(monkeypatch, tmp_path):
    # A new producer of a passed integrity diagnostic in another product file stops the rules by the census.
    snap = copy_core(tmp_path)
    target = snap / PP / "validation/model_checks.rs"
    target.write_text(target.read_text(encoding="utf-8") + '\nconst _PRA_TEST: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";\n',
                      encoding="utf-8")
    with pytest.raises(SystemExit, match="PRA-IO.*IO-CENSUS"):
        decisions(monkeypatch, snap)


@pytest.mark.parametrize("old,new", [
    ("        Some(Err(failure)) => Some(failure.clone()),\n", "        Some(Err(failure)) => Some(failure.clone().clone()),\n"),
    ("&& selected_source.is_none()).and_then(|solve| solve.member_bound.as_ref());\n",
     "&& selected_source.is_none()).and_then(|solve| solve.member_bound.as_ref()).cloned().as_ref();\n"),
    ("    let confirmed_candidate = confirmed_state.as_deref()", "    let confirmed_candidate = confirmed_state.clone().as_deref()"),
])
def test_a_changed_correction_chain_stops(old, new):
    assert G["cf_failures"](_mutant(_lib(), old, new))


@pytest.mark.parametrize("old,new,label", [
    ("    let rest = evaluation.message.strip_prefix(INSPECTION_ONLY_LEAD)?;\n",
     "    let rest = evaluation.message.clone();\n    let rest = rest.strip_prefix(INSPECTION_ONLY_LEAD)?;\n", "TR-BODY"),
])
def test_a_changed_trigger_read_stops(old, new, label):
    lib = _mutant(_lib(), old, new)
    assert any(f.startswith(label) for f in G["tr_failures"](lib, lib))
    code = _lib() + "\nfn extra() { case_triggers(a, b, c, d); }\n"
    assert any(f.startswith("TR-CALLS") for f in G["tr_failures"](_lib(), code.replace("//", "")))


def test_a_partly_present_pr_a_stops():
    parts = {"inspection_only": True, "corrected_nodal_displacements": False}
    assert G["parts_failures"](parts, "corrected") and G["parts_failures"](parts, None)
    assert G["parts_failures"]({"inspection_only": False}, None)            # neither PR-A nor U5B-PD's earlier shape
    assert G["parts_failures"]({"inspection_only": False}, "published") == []
    assert G["parts_failures"]({"inspection_only": True}, "published")       # PR-A without its corrected owner


@pytest.mark.parametrize("guard,label", [("io_failures", "PRA-IO"), ("cf_failures", "PRA-CF"), ("tr_failures", "PRA-TR"),
                                         ("parts_failures", "PRA:")])
def test_the_chain_stops_by_name_when_a_pr_a_guard_fails(tmp_path, monkeypatch, guard, label):
    tool = tmp_path / "retained_pricing"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("__pycache__", "tests"))
    guards = tool / "base/pra_guards.py"
    guards.write_text(guards.read_text(encoding="utf-8") + f"\n\ndef {guard}(*args):\n    return ['{guard} (mutant)']\n",
                      encoding="utf-8")
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    with pytest.raises(SystemExit, match=label):
        runpy.run_path(str(tool / "rules/sq2_decisions_L.py"))


def test_route_e_require_message_must_be_literal(monkeypatch, tmp_path):
    snap = copy_core(tmp_path)
    edit(snap, RE + "nonlinear_support_evidence.rs", '"ROWS_IN_UNSOLVED_DOCUMENT")?;', '&format!("ROWS_{}", 1))?;')
    with pytest.raises(SystemExit):
        decisions(monkeypatch, snap, route="E")


# ---- the class: an omitted pinned target, or the correction-failure work, cannot drop out of the count.

def test_current_source_has_one_published_displacement_shape():
    assert U45["published_displacement_shape"](SOURCE, (SOURCE / LIB).read_text(encoding="utf-8")) == "corrected"
    assert U45["inspect"](SOURCE)["published_displacements"] is True


@pytest.mark.parametrize("old,new", [
    ("    use open_pipe_stress_frame_kernel::structural::certify_displacement_step;\n",
     "    use open_pipe_stress_frame_kernel::structural::certify_displacement_step;\n    let _extra = displacements.to_vec();\n"),
    ("        .map(|bound| corrected_nodal_displacements(&displacements, bound));\n",
     "        .map(|bound| corrected_nodal_displacements(&displacements, bound).map(|v| v.clone()));\n"),
])
def test_a_changed_published_displacement_owner_stops(tmp_path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, LIB, old, new)
    with pytest.raises(SystemExit):
        U45["inspect"](snap)


def _omit_corrected_owner(snap):
    """Rename the corrected owner and its one call: the source keeps the work under a name no model reviewed."""
    text = (snap / LIB).read_text(encoding="utf-8")
    assert text.count("corrected_nodal_displacements(") == 2
    (snap / LIB).write_text(text.replace("corrected_nodal_displacements(", "renamed_nodal_displacements("), encoding="utf-8")
    return snap


def test_an_omitted_published_displacement_owner_stops_every_reader(monkeypatch, tmp_path):
    snap = _omit_corrected_owner(copy_core(tmp_path))
    # The model alone reads it as absent; the registry and the failure model refuse that reading.
    reading = U45["inspect"](snap)
    assert reading["published_displacements"] is False and reading["station_quantities"] is True
    model = str(PRICING / "base/u4b_u5b_owners_model.py")
    monkeypatch.setitem(PRES["check"].__globals__, "OWNERS", (
        (70, "U5B-PD published displacements", "u4b_u5b_owners_model.py", "inspect", "published_displacements"),
        (70, "U4B-ST station quantities", "u4b_u5b_owners_model.py", "inspect", "station_quantities")))
    with pytest.raises(SystemExit, match="OWN-PRES.*U5B-PD"):
        PRES["check"](snap)
    with pytest.raises(SystemExit, match="PRA-CF"):
        CF["inspect"](snap)


def test_omitting_the_correction_failure_work_stops(tmp_path):
    snap = copy_core(tmp_path)
    text = (snap / LIB).read_text(encoding="utf-8")
    (snap / LIB).write_text(text.replace("fn displacement_correction_warning(", "fn renamed_correction_warning(")
                            .replace("diagnostics.push(displacement_correction_warning(", "diagnostics.push(renamed_correction_warning("),
                            encoding="utf-8")
    # The failure model refuses the source, and the rules see PR-A partly present.
    with pytest.raises(SystemExit, match="PRA-CF"):
        CF["inspect"](snap)
    parts = {"inspection_only": True, "inspection_only_prior": True, "corrected_nodal_displacements": True,
             "displacement_correction_warning": False}
    assert G["parts_failures"](parts, "corrected")
    # The registry requires the failure texts' owner wherever U5B-PD's epoch and later owners are present.
    assert next(e for e in PRES["OWNERS"] if e[1] == "PRA-CF correction failure texts")[0] == max(e[0] for e in PRES["OWNERS"])


def test_the_registry_requires_every_owner_up_to_the_latest_present_epoch(tmp_path, monkeypatch):
    for name, value in (("early.py", "False"), ("late.py", "True")):
        (tmp_path / name).write_text(f"def inspect(snapshot):\n    return {value}\n", encoding="utf-8")
    monkeypatch.setitem(PRES["check"].__globals__, "HERE", str(tmp_path))
    monkeypatch.setitem(PRES["check"].__globals__, "OWNERS", ((1, "early owner", "early.py", "inspect", None),
                                                             (2, "late owner", "late.py", "inspect", None)))
    with pytest.raises(SystemExit, match="early owner"):
        PRES["check"]("unused")
    # An owner of a later epoch than any present one is not yet owed (an earlier source).
    monkeypatch.setitem(PRES["check"].__globals__, "OWNERS", ((3, "early owner", "early.py", "inspect", None),
                                                             (2, "late owner", "late.py", "inspect", None)))
    assert PRES["check"]("unused") == {"early owner": False, "late owner": True}


def test_the_current_source_holds_every_registered_owner():
    assert all(PRES["check"](SOURCE).values())


def test_an_unreviewed_absent_rule_target_stops():
    reviewed = {("gone", "a.rs"): (None, "historical"), ("old", "a.rs"): (("new", "a.rs"), "renamed")}
    present = lambda name, f: name == "new"
    assert G["gate_failures"]({("gone", "a.rs"), ("old", "a.rs")}, reviewed, present) == []
    assert G["gate_failures"]({("lost", "a.rs")}, reviewed, present) == ["GATE-UNREVIEWED ('lost', 'a.rs')"]
    assert G["gate_failures"]({("old", "a.rs")}, reviewed, lambda name, f: False)[0].startswith("GATE-ALTERNATIVE")


def test_the_chain_stops_when_a_reviewed_absence_is_withdrawn(tmp_path, monkeypatch):
    # The current source lacks `row_basis` (a reviewed historical absence); without its entry the run stops.
    tool = tmp_path / "retained_pricing"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("__pycache__", "tests"))
    rules = tool / "rules/o10_rules.py"
    text = rules.read_text(encoding="utf-8")
    entry = '    ("row_basis", RE_ + "connector_evidence.rs"): (None, "E4\'s connector row basis, deleted with its zero edge and audited site"),\n'
    assert text.count(entry) == 1
    rules.write_text(text.replace(entry, ""), encoding="utf-8")
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    with pytest.raises(SystemExit, match="O10-GATE.*row_basis"):
        runpy.run_path(str(tool / "rules/sq2_decisions_L.py"))


def test_the_reviewed_absent_targets_are_absent_now(monkeypatch):
    d = decisions(monkeypatch)
    assert set(d["O10_GATE_ABSENT"]) <= set(d["O10_ABSENT_REVIEWED"])
