#!/usr/bin/env python3
"""I67 U6d fresh collision check for the new names this unit introduces beyond
D-U6-4's 25 reserved names (records only). Read-only: `git grep -F` against
committed trees with GIT_OPTIONAL_LOCKS=0; no working tree, index or ref is
written. Run from NUM: GIT_OPTIONAL_LOCKS=0 python3 collision_check.py <out.json>"""
import json, os, subprocess, sys
P = "projects/chirality-piping"
SCOPES = [f"{P}/{d}" for d in ("core", "apps", "schemas", "fixtures", "tests", "validation", "tools")]
TREES = ["HEAD", "origin/main", "codex/piping-f2a-facade-20261004", "844448112f"]
NAMES = [
    "PREVIEW_PHYSICS_RETAINED_CONTRACT_ID", "PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256", "PREVIEW_PHYSICS_RETAINED_PROFILE",
    "retainedPrecisionDowngrade", "ordinaryCaseEligible",
    "registerRetainedPrecision", "retainedPrecisionRegistration", "retainedStandingFrom", "retainedPrecisionInvocation",
    "retainedRowClasses", "classificationSummaryFrom", "retainedPrecisionStandingText", "RetainedStandingToken", "RetainedClassificationSummary",
    "N_RP_UNVALIDATED", "retained-precision-unvalidated", "upwardBoundText", "retainedAbsoluteNotice", "classBindingRefusal", "retainedRowClassLabel",
    "RETAINED_PRECISION_OUTPUT_REFUSAL", "N_RETAINED_PRECISION_OUTPUT", "RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE", "isRetainedPrecisionRoute", "sameSavedReceipt",
]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
out = {"trees": {}, "names": {}}
for t in TREES:
    out["trees"][t] = subprocess.run(["git", "rev-parse", t], capture_output=True, text=True, env=env).stdout.strip()
for n in NAMES:
    out["names"][n] = {}
    for t in TREES:
        p = subprocess.run(["git", "grep", "-F", "-l", "-e", n, t, "--", *SCOPES], capture_output=True, text=True, env=env)
        files = [l.split(":", 1)[1] for l in p.stdout.splitlines()]
        out["names"][n][t] = {"rc": p.returncode, "files": files}
out["all_absent"] = all(v["rc"] == 1 for n in out["names"].values() for v in n.values())
json.dump(out, open(sys.argv[1], "w"), indent=1)
print("all_absent", out["all_absent"], [n for n, v in out["names"].items() if any(x["rc"] != 1 for x in v.values())])
