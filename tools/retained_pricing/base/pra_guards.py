"""§8.62 (PR-A) source guards, by exact text and name. rules/o10_rules.py adds the PRA rules only after these return no
failure; each function returns the names of the checks that fail (an empty list: all hold). The arguments for the
rules they warrant stand beside the rules in o10_rules.py and in retained-pricing-rules.md §8.62.

- parts_failures: PR-A's source is wholly present (with U5B-PD's corrected shape) or wholly absent (with its earlier
  published shape); a part without the rest stops.
- io_failures (PRA-IO): the blocked rewrite's body and loop, append_integrity_report's one push of a CHECKS_PASSED or
  SENSITIVE integrity diagnostic and its two calls in the case solve, append_integrity_failure's failure codes, and
  the product crate's census of every integrity id, code literal and rewrite call.
- cf_failures (PRA-CF): the correction's Option chain, its operands' Option forms and the failure's one clone.
- tr_failures (PRA-TR): the trigger's prior read and its two calls.
- gate_failures (O10-GATE): every absent rule target is reviewed, and its reviewed alternative is present.
"""
import re

IO_BODY = (
    "fn inspection_only(diagnostic: &mut Diagnostic) {\n"
    "    if !matches!(diagnostic.code.as_str(), \"NUMERICAL_INTEGRITY_CHECKS_PASSED\" | \"NUMERICAL_INTEGRITY_SENSITIVE\") {\n"
    "        return;\n"
    "    }\n"
    "    diagnostic.message = format!(\"{INSPECTION_ONLY_LEAD}{}{INSPECTION_ONLY_TAIL}{}\", diagnostic.code, diagnostic.message);\n"
    "    diagnostic.code = INSPECTION_ONLY_CODE.to_string();\n"
    "    diagnostic.severity = \"info\".to_string();\n"
    "}\n")
IO_LOOP = ("    for diagnostic in diagnostics.iter_mut().filter(|d| d.id.starts_with(INTEGRITY_DIAGNOSTIC_PREFIX)) {\n"
           "        inspection_only(diagnostic);\n"
           "    }\n")
IO_LOOP_HEADER = "for diagnostic in diagnostics.iter_mut().filter(|d| d.id.starts_with(INTEGRITY_DIAGNOSTIC_PREFIX))"
IO_REPORT_CODE = ('    let code = if report.quality == SolveQuality::Sensitive {\n        "NUMERICAL_INTEGRITY_SENSITIVE"\n'
                  '    } else {\n        "NUMERICAL_INTEGRITY_CHECKS_PASSED"\n    };\n')
IO_REPORT_PUSH = "diagnostics.push(diag(&integrity_diagnostic_id(case_id), code, "
CENSUS_NEEDLES = ("diagnostic:numerical-integrity:", "integrity_diagnostic_id(", "append_integrity_report(",
                  "\"NUMERICAL_INTEGRITY_CHECKS_PASSED\"", "\"NUMERICAL_INTEGRITY_SENSITIVE\"", "inspection_only(",
                  "SENSITIVE.to_string()")
IO_CENSUS = {
    # The prefix: its constant and integrity_diagnostic_id's format, nowhere else.
    ("lib.rs", "diagnostic:numerical-integrity:"): 2,
    # The id: its definition, the five creating pushes (append_integrity_report, append_integrity_failure, the range
    # refusal, the two recovery-basis warnings), two ordinary-attempt references and eight lookups by id.
    ("lib.rs", "integrity_diagnostic_id("): 16,
    # The definition and the two calls in the case solve.
    ("lib.rs", "append_integrity_report("): 3,
    # The two codes as literals: the rewrite's test, append_integrity_report's choice, the quality arms, the blocked
    # quality's debug assertion, the prior read's table and (CHECKS_PASSED only) the case solve's pre-guard comparison.
    ("lib.rs", "\"NUMERICAL_INTEGRITY_CHECKS_PASSED\""): 6, ("lib.rs", "\"NUMERICAL_INTEGRITY_SENSITIVE\""): 5,
    # The rewrite's definition and its one call.
    ("lib.rs", "inspection_only("): 2,
    # formation_guard's two constants (demote moves PASSED to SENSITIVE and creates nothing) and its one write.
    ("formation_guard.rs", "\"NUMERICAL_INTEGRITY_CHECKS_PASSED\""): 1, ("formation_guard.rs", "\"NUMERICAL_INTEGRITY_SENSITIVE\""): 1,
    ("formation_guard.rs", "SENSITIVE.to_string()"): 1,
    # retained_wire's and source_receipt's code-to-token tables and expected codes (no Diagnostic is made there).
    ("retained_wire.rs", "\"NUMERICAL_INTEGRITY_CHECKS_PASSED\""): 1, ("retained_wire.rs", "\"NUMERICAL_INTEGRITY_SENSITIVE\""): 1,
    ("source_receipt.rs", "\"NUMERICAL_INTEGRITY_CHECKS_PASSED\""): 1, ("source_receipt.rs", "\"NUMERICAL_INTEGRITY_SENSITIVE\""): 1,
}
CF_DECLS = (
    "    let linear_bound = linear_solve.as_ref().filter(|solve| pre_guard_passed && load_row_finding.is_none() "
    "&& built.nonlinear_supports.is_empty() && solve.structural_report.quality == SolveQuality::Passed "
    "&& selected_source.is_none()).and_then(|solve| solve.member_bound.as_ref());\n",
    "    let confirmed_candidate = confirmed_state.as_deref().filter(|_| pre_guard_passed && load_row_finding.is_none() "
    "&& !built.nonlinear_supports.is_empty());\n",
    "    let correction_failure = match &correction {\n        Some(Err(failure)) => Some(failure.clone()),\n        _ => None,\n    };\n",
)
CF_OPTION_HEADER = ("iter:linear_bound .or(confirmed_candidate.map(|state| &state.member_bound)) "
                    ".filter(|_| displacements.iter().all(|value| value.is_finite())).map")
CF_DOF_HEADER = ("iter:corrected_displacement_tests::FORCED_CORRECTION_FAILURE.with(|forced| forced.get()) { return Err("
                 "DisplacementCorrectionFailure { dof, kind, error: \" \".to_string() }); } (0..displacements.len()).map")
TR_BODY = (
    "fn inspection_only_prior(diagnostics: &[Diagnostic], case_id: &str) -> Option<NumericalQualityStatus> {\n"
    "    let wanted_id = integrity_diagnostic_id(case_id);\n"
    "    let evaluation = only_one(diagnostics.iter().filter(|d| d.id == wanted_id))?;\n"
    "    if evaluation.code != INSPECTION_ONLY_CODE {\n"
    "        return None;\n"
    "    }\n"
    "    let rest = evaluation.message.strip_prefix(INSPECTION_ONLY_LEAD)?;\n"
    "    [\n"
    "        (\"NUMERICAL_INTEGRITY_CHECKS_PASSED\", NumericalQualityStatus::ChecksPassed),\n"
    "        (\"NUMERICAL_INTEGRITY_SENSITIVE\", NumericalQualityStatus::Sensitive),\n"
    "    ]\n"
    "    .into_iter()\n"
    "    .find_map(|(code, prior)| rest.strip_prefix(code)?.starts_with(INSPECTION_ONLY_TAIL).then_some(prior))\n"
    "}\n")
TR_CALL = "case_triggers(&ordinary.numerical_quality, &ordinary.diagnostics, &observer.ordinary, cases.ids())"


def fn_text(text, name):
    """A top-level `fn name`'s exact text, from its line to its first closing `}` in column 0; None if not one."""
    hits = list(re.finditer(r"^(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        return None
    end = text.find("\n}\n", hits[0].start())
    return text[hits[0].start():end + 3] if end >= 0 else None


def parts_failures(parts, shape):
    if any(parts.values()) and not (all(parts.values()) and shape == "corrected"):
        return [f"PARTS (PR-A's source is partly present: {parts}, U5B-PD shape {shape})"]
    if not any(parts.values()) and shape != "published":
        return [f"PARTS (neither PR-A's source nor U5B-PD's earlier shape: {shape})"]
    return []


def io_failures(lib, census):
    case = fn_text(lib, "solve_prepared_load_case") or ""
    report = fn_text(lib, "append_integrity_report") or ""
    checks = {
        "IO-BODY (inspection_only's exact body)": fn_text(lib, "inspection_only") == IO_BODY,
        "IO-LOOP (blocked_envelope's loop, once)": (fn_text(lib, "blocked_envelope") or "").count(IO_LOOP) == 1,
        "IO-CENSUS (the integrity diagnostics' producers and the two codes)": census == IO_CENSUS,
        "IO-REPORT-CALLS (append_integrity_report: two calls, both in the case solve)": case.count("append_integrity_report(") == 2,
        "IO-REPORT-PUSH (append_integrity_report pushes one integrity diagnostic)":
            report.count(IO_REPORT_PUSH) == 1 and report.count("diagnostics.push(") == 1,
        "IO-REPORT-CODE (its code is one of the two literals)": report.count(IO_REPORT_CODE) == 1,
        "IO-FAILURE-CODES (append_integrity_failure's codes are failures)":
            not re.search(r"CHECKS_PASSED|INTEGRITY_SENSITIVE", fn_text(lib, "integrity_failure_code") or "CHECKS_PASSED"),
    }
    return [name for name, ok in checks.items() if not ok]


def cf_failures(lib):
    case = fn_text(lib, "solve_prepared_load_case") or ""
    return [f"CF-DECL ({decl.strip().split(' =')[0]})" for decl in CF_DECLS if case.count(decl) != 1]


def tr_failures(lib, code):
    """`code` is lib.rs's production text without comments (the calls are counted there)."""
    failed = []
    if fn_text(lib, "inspection_only_prior") != TR_BODY:
        failed.append("TR-BODY (inspection_only_prior's exact body)")
    if (fn_text(lib, "retained_w1") or "").count(TR_CALL) != 2 or code.count("case_triggers(") != 2:
        failed.append("TR-CALLS (case_triggers: two calls, both in retained_w1)")
    if code.count("inspection_only_prior(") != 2:
        failed.append("TR-PRIOR (inspection_only_prior: one call)")
    return failed


def gate_failures(absent, reviewed, present):
    """O10-GATE: `absent` the (name, file) lookups that found no function; `reviewed` maps each allowed absence to
    (alternative or None, why); `present(name, file)` says whether an alternative exists."""
    failed = [f"GATE-UNREVIEWED {k}" for k in sorted(absent) if k not in reviewed]
    failed += [f"GATE-ALTERNATIVE {k} needs {reviewed[k][0]}" for k in sorted(absent)
               if k in reviewed and reviewed[k][0] and not present(*reviewed[k][0])]
    return failed
