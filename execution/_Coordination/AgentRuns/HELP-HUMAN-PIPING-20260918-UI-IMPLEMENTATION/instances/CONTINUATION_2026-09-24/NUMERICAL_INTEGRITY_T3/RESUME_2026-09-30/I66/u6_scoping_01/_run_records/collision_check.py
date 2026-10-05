#!/usr/bin/env python3
"""I66 fresh collision check for U6's proposed new names (records only).

Read-only: `git grep -F` and `git cat-file -e` against committed trees, run with
GIT_OPTIONAL_LOCKS=0. No working tree, index or ref is written. Usage, from NUM:

    GIT_OPTIONAL_LOCKS=0 python3 <this file> <out.json>

Trees searched: NUM HEAD, origin/main, and the U3 branch head as committed
(codex/piping-f2a-facade-20261004). Paths: P/{core,apps,schemas,fixtures,tests,
validation,tools}; execution records are excluded (they discuss the names).
"""
import json
import os
import subprocess
import sys

P = "projects/chirality-piping"
SCOPES = [f"{P}/{d}" for d in ("core", "apps", "schemas", "fixtures", "tests", "validation", "tools")]
TREES = ["HEAD", "origin/main", "codex/piping-f2a-facade-20261004"]

NAMES = [
    # Binding refusal codes designed in D2 4.9.9 (revision 4, N-5); not in RV69's reservation list.
    "RULE_QUANTITY_BELOW_VERIFIED_FLOOR",
    "RULE_QUANTITY_NOT_COVERED",
    # Carrier binding codes proposed by I66 (patterns: SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH,
    # SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN, ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH).
    "RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH",
    "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN",
    "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH",
    "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN",
    # Derivative row-disclosure reason codes (decision D-U6-2, option A).
    "retained_precision_absolute_verified",
    "retained_precision_not_covered",
    # TypeScript route token, standing finding and desktop output refusal.
    # (First choice `preview_physics_retained` was dropped: it is a substring of the
    # existing fixture name semantic_contract_v0_3_preview_physics_retained_1.json.)
    "retained_preview_physics",
    "RETAINED_PRECISION_VALIDATION_REQUIRED",
    "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE",
    "RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE",
    # Notice constants and ids (D2 4.9.9 texts).
    "N_RP_ABSOLUTE",
    "N_RP_NOT_COVERED",
    "retained-precision-absolute",
    "retained-precision-not-covered",
    # Per-language summary function (D2 4.9.9).
    "classification_summary",
    "classificationSummary",
    # New file stems.
    "retained_precision_milestone_successor",
    "retained_precision_carrier_cases",
    "retained_precision_carriers",
    "retainedPrecisionStanding",
    "retainedPrecisionIntegration",
    "retainedPrecisionAnalysisRun",
    "retainedPrecisionOutputRefusal",
]

PATHS = [
    f"{P}/fixtures/results/retained_precision_milestone_successor_sparse_interactive.json",
    f"{P}/fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json",
    f"{P}/fixtures/results/retained_precision_carrier_cases.json",
    f"{P}/core/reporting/result_export/tests/retained_precision_carriers.rs",
    f"{P}/tests/test_retained_precision_carriers.py",
    f"{P}/apps/desktop/src/features/results/retainedPrecisionStanding.ts",
    f"{P}/apps/desktop/src/features/results/retainedPrecisionIntegration.test.tsx",
    f"{P}/apps/desktop/src/services/retainedPrecisionAnalysisRun.test.ts",
    f"{P}/apps/desktop/src/features/results/retainedPrecisionOutputRefusal.test.tsx",
]


def git(*args):
    env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
    return subprocess.run(["git", *args], capture_output=True, text=True, env=env)


def main(out):
    report = {"trees": {}, "names": {}, "paths": {}}
    for tree in TREES:
        rev = git("rev-parse", tree)
        report["trees"][tree] = rev.stdout.strip() if rev.returncode == 0 else None
    for name in NAMES:
        per = {}
        for tree, sha in report["trees"].items():
            if sha is None:
                per[tree] = {"error": "unresolved"}
                continue
            res = git("grep", "-n", "-F", "-e", name, sha, "--", *SCOPES)
            # git grep: 0 = match, 1 = no match, other = error.
            lines = [l for l in res.stdout.splitlines() if l]
            per[tree] = {"rc": res.returncode, "matches": len(lines), "first": lines[:3]}
        report["names"][name] = per
    for path in PATHS:
        per = {}
        for tree, sha in report["trees"].items():
            if sha is None:
                per[tree] = None
                continue
            per[tree] = git("cat-file", "-e", f"{sha}:{path}").returncode == 0
        report["paths"][path] = per
    clean = all(v["rc"] == 1 for n in report["names"].values() for v in n.values() if "rc" in v)
    report["all_names_absent"] = clean
    report["all_paths_absent"] = not any(v for p in report["paths"].values() for v in p.values())
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(report, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(json.dumps({"all_names_absent": report["all_names_absent"], "all_paths_absent": report["all_paths_absent"], "trees": report["trees"]}))


if __name__ == "__main__":
    main(sys.argv[1])
