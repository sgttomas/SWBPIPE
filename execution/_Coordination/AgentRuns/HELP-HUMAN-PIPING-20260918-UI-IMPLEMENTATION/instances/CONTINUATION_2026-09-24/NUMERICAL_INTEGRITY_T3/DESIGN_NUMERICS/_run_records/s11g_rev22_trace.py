#!/usr/bin/env python3
"""D1 S11-G revision 2.2: source-trace pins for the routing-gate analysis (standard library; reads only).

Usage (from T3/):
  PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 DESIGN_NUMERICS/_run_records/s11g_rev22_trace.py <P> <out.json>

<P> is `projects/chirality-piping` (relative `../../../../../../..`) in a tree whose product code equals main
`72d5ff864` (the numerics worktree after its merge of main; `git diff 72d5ff864 HEAD -- projects/chirality-piping/core`
is empty). For each claim of S11G_GUARD.md revision 2.2 section 0.2 / 3.5 item 2, the script checks that the cited
line range contains the cited text, and records the file's sha256. Every check must be true.
"""
import hashlib
import json
import os
import sys

CHECKS = [
    # (claim, file under core/product_physics/src, first line, last line, required substrings)
    ("main routing predicate has no finding", "lib.rs", 2592, 2593,
     ["let source_eligible = capture.is_some() && built.nonlinear_supports.is_empty() && model.combinations.is_empty();",
      "if source_eligible && needs_source_recovery {"]),
    ("needs_source_recovery follows the report or an Err", "lib.rs", 2583, 2586,
     ["let needs_source_recovery = match &attempted_linear {", "SolveQuality::Sensitive", "Err(_) => true"]),
    ("a failed attempt records the failure and the UNAVAILABLE diagnostic", "lib.rs", 2625, 2640,
     ["Err(failure) =>", "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "source_failure = Some(failure);"]),
    ("per-case entry: failed when an attempt failed, else ordinary", "lib.rs", 3683, 3691,
     ["} else if let Some(failure) = &source_failure {", "FinalizedSourceBlockCase::failed(",
      "FinalizedSourceBlockCase::ordinary(capture, &load_case.id, ordinary_attempt, &qualified)"]),
    ("the ordinary entry refuses a non-checks-passed outcome", "source_receipt.rs", 710, 718,
     ["pub(super) fn ordinary(", "ordinary.outcome != \"checks_passed\"", "ordinary selection not p1 checks-passed"]),
    ("the failed entry has no outcome precondition", "source_receipt.rs", 752, 762,
     ["pub(super) fn failed(", "validate_case_rows(case_id, rows)?;"]),
    ("finalization requires an entry for every case", "source_receipt.rs", 955, 961,
     ["cases.len() != request.model.load_cases.len()", "invocation case coverage"]),
    ("the wire binds outcome to the published quality and code", "source_receipt.rs", 527, 558,
     ["fn wire(", "\"sensitive\" => quality == \"sensitive\"", "ordinary outcome changed", "ordinary diagnostic binding"]),
    ("OrdinaryAttempt::passed derives its outcome from the report only", "source_receipt.rs", 454, 476,
     ["pub(super) fn passed(", "report.quality == open_pipe_stress_frame_kernel::structural::SolveQuality::Sensitive"]),
    ("finalization failure is a blocking diagnostic", "lib.rs", 2055, 2058,
     ["SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED", "\"blocking\""]),
    ("the captured entry then returns Err", "lib.rs", 1483, 1488,
     ["result.source_block_recovery.is_none()", "return Err(\"SOURCE_BLOCKS_FINALIZATION_FAILED\".into());"]),
    ("retained scope refuses element uniform loads before charged work", "source_recovery.rs", 518, 528,
     ["!input.load_application.element_uniform_loads.is_empty()", "non-nodal load producer present"]),
    ("retained scope refuses non-nodal authored families", "source_recovery.rs", 590, 601,
     ["authored non-nodal load family, including zero-valued inputs"]),
    ("the existing decline pattern", "source_recovery.rs", 222, 231,
     ["pub fn decline_withheld(self) -> RecoveryFailure", "RecoveryError::Unsupported("]),
    ("Unsupported maps to the existing unsupported_family code", "source_receipt/source.rs", 409, 434,
     ["R::Unsupported(_) => (\"unsupported_family\", None)", "AttemptStage::SourceClosure => \"source_validation\""]),
    ("S11-F's F6: an element-load case beside a selected case finalizes on main", "s11f_tests.rs", 1236, 1276,
     ["fn f6_multi_case_pre_0_4_invocation_never_errs_on_cancelling_loads()", "uniform(\"udl:b\", \"pipe\", \"global_y\", 10.0)",
      "envelope.source_block_recovery.is_some()", "SOURCE_BLOCK_RECOVERY_SELECTED"]),
    ("readers bind the ordinary outcome as the wire does", "../../reporting/result_export/src/source_blocks.rs", 226, 252,
     ["\"checks_passed\" | \"sensitive\" => require!(", "q[\"solve_quality\"] == o[\"outcome\"]"]),
    ("an attempted case must record sensitive or rejected", "../../reporting/result_export/src/physics_source.rs", 613, 620,
     ["Some(\"sensitive\" | \"rejected\")", "SOURCE_FALLBACK_TRIGGER"]),
]


def main():
    root, outp = sys.argv[1], sys.argv[2]
    base = os.path.join(root, "core", "product_physics", "src")
    out = {"checks": [], "files": {}}
    ok = True
    for claim, f, a, b, needles in CHECKS:
        path = os.path.normpath(os.path.join(base, f))
        data = open(path, "rb").read()
        rel = os.path.relpath(path, root)
        out["files"][rel] = hashlib.sha256(data).hexdigest()
        text = "\n".join(data.decode().split("\n")[a - 1:b])
        found = {n: (n in text) for n in needles}
        good = all(found.values())
        ok = ok and good
        out["checks"].append({"claim": claim, "file": rel, "lines": [a, b], "all_found": good,
                              "missing": [n for n, v in found.items() if not v]})
    out["all_true"] = ok
    json.dump(out, open(outp, "w"), indent=1, sort_keys=True)
    print(json.dumps({"all_true": ok, "failed": [c["claim"] for c in out["checks"] if not c["all_found"]]}, indent=1))


if __name__ == "__main__":
    main()
