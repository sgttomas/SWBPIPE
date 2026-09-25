#!/usr/bin/env python3
"""Compare the 1,092 recorded native build inputs (JOINED_ENGINE_NATIVE inputs-pre.json, the
8b982aa7 executable's record) with Git blobs at each named revision. Read-only. Run from repo root."""
import hashlib, json, subprocess, sys
REC = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/_run_records/JOINED_ENGINE_NATIVE/_run_records/inputs-pre.json"
pre = json.load(open(REC)); rec = {r["path"]: r["sha256"] for r in pre}
def sha(rev, p):
    r = subprocess.run(["git", "show", f"{rev}:{p}"], capture_output=True)
    return hashlib.sha256(r.stdout).hexdigest() if r.returncode == 0 else None
for rev in sys.argv[1:]:
    full = subprocess.check_output(["git", "rev-parse", rev], text=True).strip()
    diff = sorted(p for p, s in rec.items() if sha(full, p) != s)
    print(json.dumps({"rev": full, "recorded": len(rec), "identical": len(rec) - len(diff), "differ": diff}))
