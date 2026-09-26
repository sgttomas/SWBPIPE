"""Compare recorded native build inputs (JOINED_ENGINE_NATIVE inputs-pre.json)
with blob bytes at named revisions. Read-only: reads Git objects only.
Run from the repository root: python3 <this file> REV [REV ...]"""
import hashlib, json, subprocess, sys
REC = ("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/"
       "instances/CONTINUATION_2026-09-24/_run_records/JOINED_ENGINE_NATIVE/_run_records/inputs-pre.json")
def blob(rev, path):
    r = subprocess.run(["git", "cat-file", "blob", f"{rev}:{path}"], capture_output=True)
    return r.stdout if r.returncode == 0 else None
recorded = json.loads(subprocess.check_output(["git", "cat-file", "blob", f"6d9c0915f:{REC}"]))
print("recorded inputs:", len(recorded), "unique:", len({e['path'] for e in recorded}))
for rev in sys.argv[1:]:
    diffs = []
    for e in recorded:
        b = blob(rev, e["path"])
        h = hashlib.sha256(b).hexdigest() if b is not None else None
        if h != e["sha256"]:
            diffs.append((e["path"], "absent" if h is None else "changed"))
    print(f"{rev}: identical={len(recorded)-len(diffs)} differing={len(diffs)}")
    for p, k in diffs:
        print(f"   {k}: {p}")

# Set membership: build.py enumerated `git ls-files` over these roots at 8b982aa7.
ROOTS = ["projects/chirality-piping/apps/desktop", "projects/chirality-piping/core", "projects/chirality-piping/schemas",
         "projects/chirality-piping/fixtures", "projects/chirality-piping/package.json", "projects/chirality-piping/package-lock.json"]
rec = {e["path"] for e in recorded}
for rev in sys.argv[1:]:
    now = set(subprocess.check_output(["git", "ls-tree", "-r", "--name-only", rev, "--", *ROOTS], text=True).split())
    print(f"{rev}: tracked-under-roots={len(now)} added={sorted(now-rec)} removed={sorted(rec-now)}")
