"""PR-A, the integrity-diagnostic repair (§8.62).

- PRA-IO: the blocked rewrite's loop is bounded by the integrity diagnostics that can carry CHECKS_PASSED or SENSITIVE
  (2*cases); its message, code and constants are site-audited; the old and new messages are both charged.
- PRA-CF: the corrected displacement's failure texts are charged positively (no zero premise): the Option chain runs
  once, the Debug text at most once per call, at a bound derived from the kernel's error types and literals.
- PRA-TR, PRA-NL: the trigger's prior read and route E's direct require are bound by their exact text.
- U5B-PD-RN: the owner's two reviewed shapes, one present; anything else stops.
- OWN-PRES and O10-GATE: an owner or a rule target that a current source loses, renames or moves stops the run; it is
  never priced as older source.
- CFG-PROD: the production extraction and the chain scanner end a test element where Rust ends it, so no production
  after it is hidden.

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
    reviewed = {("gone", "a.rs"): (("text-absent", "a.rs", "its own site"), "historical"),
                ("old", "a.rs"): (("fn", "new", "a.rs"), "renamed")}
    holds = lambda fact: fact[0] == "text-absent" or fact[1] == "new"
    assert G["gate_failures"]({("gone", "a.rs"), ("old", "a.rs")}, reviewed, holds) == []
    assert G["gate_failures"]({("lost", "a.rs")}, reviewed, holds) == ["GATE-UNREVIEWED ('lost', 'a.rs')"]
    assert G["gate_failures"]({("old", "a.rs")}, reviewed, lambda fact: False)[0].startswith("GATE-FACT")


def test_the_chain_stops_when_a_reviewed_absence_is_withdrawn(tmp_path, monkeypatch):
    # The current source lacks `row_basis` (a reviewed historical absence); without its entry the run stops.
    tool = tmp_path / "retained_pricing"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("__pycache__", "tests"))
    rules = tool / "rules/o10_rules.py"
    text = rules.read_text(encoding="utf-8")
    entry = text[text.index('    ("row_basis", RE_ + "connector_evidence.rs"): '):]
    entry = entry[:entry.index('audited site"),\n') + len('audited site"),\n')]
    assert text.count(entry) == 1
    rules.write_text(text.replace(entry, ""), encoding="utf-8")
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    with pytest.raises(SystemExit, match="O10-GATE.*row_basis"):
        runpy.run_path(str(tool / "rules/sq2_decisions_L.py"))


def test_the_reviewed_absent_targets_are_absent_now(monkeypatch):
    d = decisions(monkeypatch)
    assert set(d["O10_GATE_ABSENT"]) <= set(d["O10_ABSENT_REVIEWED"])


# ---- CFG-PROD: the production extraction keeps production after any test element, and removes only test elements.
CFG = runpy.run_path(str(PRICING / "base/cfg_test_items.py"))


def test_the_recovery_branch_after_the_kd5_probe_is_production():
    text = _lib()
    keep = "    if !recovery_records.is_empty() {"
    assert text.count(keep) == 1
    assert CFG["production"](text, LIB).count(keep) == 1
    assert CFG["legacy_production"](text, LIB).count(keep) == 0        # the defect this repairs
    assert CFG["production"](text, LIB).count("\n") == text.count("\n")


@pytest.mark.parametrize("source,kept,removed", [
    # a test statement whose closure holds blocks, inside a function, before production
    ("fn f() {\n    #[cfg(test)]\n    P.with(|p| { if a { b(); } });\n    if keep_me { x(); }\n}\n", ["if keep_me { x(); }"], ["P.with"]),
    # nested cfg(test) inside a test module, then production
    ("#[cfg(test)]\nmod t {\n    #[cfg(test)]\n    fn inner() {}\n    fn other() {}\n}\nfn prod() {}\n", ["fn prod() {}"], ["inner", "other"]),
    # a test let with if/else ends at its `;`
    ("fn f() {\n    #[cfg(test)]\n    let a = if c { 1 } else { 2 };\n    let keep = 3;\n}\n", ["let keep = 3;"], ["else { 2 }"]),
    # cfg(not(test)) and other attributes are kept
    ("#[cfg(not(test))]\nfn prod() {}\n#[cfg(test)]\nfn t() {}\n", ["#[cfg(not(test))]", "fn prod() {}"], ["fn t()"]),
    ("#[cfg(all(test, feature = \"x\"))]\nfn kept_here() {}\n", ["fn kept_here() {}"], []),
    # attribute stacks: after the cfg(test) they are removed with the item; before it they stay
    ("#[cfg(test)]\n#[path = \"x.rs\"]\nmod x;\nfn prod() {}\n", ["fn prod() {}"], ["#[path", "mod x;"]),
    ("#[allow(dead_code)]\n#[cfg(test)]\nfn t() { }\nfn prod() {}\n", ["#[allow(dead_code)]", "fn prod() {}"], ["fn t()"]),
    # a field, a last arm without a comma, a block expression with a method
    ("struct S {\n    #[cfg(test)]\n    probe: u8,\n    keep: u8,\n}\n", ["keep: u8,"], ["probe"]),
    ("fn f(x: u8) -> u8 {\n    match x {\n        0 => 1,\n        #[cfg(test)]\n        9 => 2\n    }\n}\n", ["0 => 1,", "}\n}"], ["9 => 2"]),
    ("fn f() {\n    #[cfg(test)]\n    { a() }.touch();\n    keep();\n}\n", ["keep();"], ["touch"]),
    # commas inside a field's generics or an item's generics and parameters do not end it
    ("struct S {\n    #[cfg(test)]\n    probe: Map<A, B>,\n    keep: u8,\n}\n", ["keep: u8,"], ["probe", "B>"]),
    ("#[cfg(test)]\nfn t<A, B>(a: A, b: B) -> (A, B) { (a, b) }\nfn prod() {}\n", ["fn prod() {}"], ["fn t", "(a, b)"]),
    # an attribute inside a literal or comment is not one
    ("fn f() {\n    let s = \"#[cfg(test)] {\";\n    // #[cfg(test)] fn no() {}\n    keep();\n}\n", ["let s = \"#[cfg(test)] {\";", "keep();"], []),
])
def test_cfg_test_elements_end_where_rust_ends_them(source, kept, removed):
    out = CFG["production"](source, "case.rs")
    assert out.count("\n") == source.count("\n")
    for text in kept:
        assert text in out, (text, out)
    for text in removed:
        assert text not in out, (text, out)


def test_an_unterminated_test_element_stops():
    with pytest.raises(ValueError, match="CFG-PROD"):
        CFG["production"]("fn f() {\n    #[cfg(test)]\n    { open(\n", "case.rs")


SCAN = runpy.run_path(str(PRICING / "base/loopscan.py"))


def _scanned(raw):
    return SCAN["blank_test_items"](SCAN["blank_rust"](raw), raw)


def test_the_scanner_keeps_production_after_a_test_field():
    # The scanner ran a test-only field on to the next depth-0 `;`, hiding the later fields, the `}` and the next item.
    raw = ("struct S {\n    #[cfg(test)]\n    probe: Option<u8>,\n    keep: [usize; 2],\n}\n"
           "const K: &str = \"k\";\nfn prod() { call(); }\n")
    out = _scanned(raw)
    assert out.count("\n") == raw.count("\n")
    assert "probe" not in out
    for text in ("keep: [usize; 2],", "}\nconst K: &str", "fn prod() { call(); }"):
        assert text in out, (text, out)


def test_the_scanner_blanks_whole_test_statements():
    raw = "fn f() {\n    #[cfg(test)]\n    let g = if hook() { a() } else { g };\n    keep();\n}\n"
    out = _scanned(raw)
    assert "hook" not in out and "else" not in out and "keep();" in out


def test_the_scanner_keeps_its_off_feature_rule():
    raw = '#[cfg(feature = "census-probe")]\nfn probe() { a(); }\nfn prod() {}\n'
    out = _scanned(raw)
    assert "probe()" not in out and "fn prod() {}" in out
    with pytest.raises(ValueError, match="W1R-FG"):
        _scanned('fn f() {\n    match x {\n        #[cfg(feature = "census-probe")]\n        1 => a(),\n        _ => b(),\n    }\n}\n')


def test_the_scanner_sees_product_capture_after_its_test_field():
    text = (SOURCE / PP / "retained_product.rs").read_text(encoding="utf-8")
    out = _scanned(text)
    for line in ("    pub prepared_capacity_bytes: [usize; 16],",
                 'pub(super) const REQUESTED_MODE_KIND: &str = "',
                 "    pub combination_rows: Vec<std::ops::Range<usize>>,"):
        assert text.count(line) == 1 and out.count(line) == 1, line
    assert "trace_fault:Option" not in out


def test_every_production_helper_uses_the_corrected_extraction(monkeypatch):
    # o10's _production, batch 5's production, RTL2's and U6's _production all read base/cfg_test_items.py.
    for path in ("rules/o10_rules.py", "base/batch5_model.py", "base/rtl2_owners_model.py", "base/u6_owners_model.py"):
        assert "cfg_test_items.py" in (PRICING / path).read_text(encoding="utf-8"), path
    batch5 = runpy.run_path(str(PRICING / "base/batch5_model.py"))
    assert batch5["production"](_lib(), LIB) == CFG["production"](_lib(), LIB)


# ---- FN-VIS and the grounded absences.

def test_a_path_scoped_function_is_found(monkeypatch):
    d = decisions(monkeypatch)
    edges = [(str(a), str(b)) for a, b, why in d["O10_EDGE_ZERO"] if "SKW" in str(why) and "certify_support_reaction_with_basis" in str(a)]
    assert ("structural/certified_recovery.rs:1474:certify_support_reaction_with_basis",
            "structural/formation_check/skew.rs:36:skew_spring_matrix") in edges
    assert all(b != "None" for _, b in edges)
    assert not any(name == "skew_spring_matrix" for name, _ in d["O10_GATE_ABSENT"])


def test_every_reviewed_absence_names_a_source_fact(monkeypatch):
    d = decisions(monkeypatch)
    assert all(fact and fact[0] in ("fn", "text-absent", "file-absent") for fact, why in d["O10_ABSENT_REVIEWED"].values())
    holds = lambda fact: False
    assert G["gate_failures"]({("row_basis", "x.rs")}, {("row_basis", "x.rs"): (("text-absent", "x.rs", "t"), "w")}, holds) \
        == ["GATE-FACT ('row_basis', 'x.rs') needs ('text-absent', 'x.rs', 't')"]
    assert G["gate_failures"]({("row_basis", "x.rs")}, {("row_basis", "x.rs"): (None, "w")}, lambda fact: True)


def test_a_historical_function_kept_under_another_name_stops(monkeypatch, tmp_path):
    snap = copy_core(tmp_path)
    target = snap / RE / "connector_evidence.rs"
    target.write_text(target.read_text(encoding="utf-8")
                      + '\nfn renamed_row_basis(span: &str) -> String {\n    format!("objective_connector_v1;replaces_span={span}")\n}\n',
                      encoding="utf-8")
    with pytest.raises((SystemExit, ValueError), match="GATE-FACT.*row_basis|connector_evidence"):
        decisions(monkeypatch, snap)
