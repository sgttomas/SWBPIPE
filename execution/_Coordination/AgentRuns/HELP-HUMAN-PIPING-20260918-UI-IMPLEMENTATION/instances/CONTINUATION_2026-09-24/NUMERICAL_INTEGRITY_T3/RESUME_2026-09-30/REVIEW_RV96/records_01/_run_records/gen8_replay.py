#!/usr/bin/env python3
"""RV96 read-only GEN-8 diagnosis: replays the GEN-8 per-file classification
(cmd_self_check.py GEN-8 loop) for projects/chirality-piping over the git-tracked
file set of a commit, reading blobs from git. No self-check, no pytest.

usage: gen8_scan.py <harness_dir> <archive_root_for_policy> <git_dir> <commit> <out.tsv>
"""
import subprocess, sys
from pathlib import Path, PurePosixPath

harness, archive_root, git_dir, commit, out = sys.argv[1:6]
sys.path.insert(0, harness)
import surface_roles as sr  # noqa: E402

archive_root = Path(archive_root).resolve()
policy = sr.load_project_policy(archive_root, archive_root / "projects" / "chirality-piping")

names = subprocess.run(
    ["git", "-C", git_dir, "ls-tree", "-r", "--name-only", "-z", commit, "--",
     "projects/chirality-piping"], capture_output=True, text=True, check=True
).stdout.split("\0")
cands = []
for n in names:
    if not n:
        continue
    p = PurePosixPath(n)
    if p.suffix not in {".md", ".yaml", ".yml", ".json"}:
        continue
    if "_harness_generated" in p.parts or ".archive" in p.parts or "node_modules" in p.parts:
        continue
    c = sr.effective_role(n, policy)
    if not c.active or c.role is sr.SurfaceRole.EVIDENCE:
        continue
    if c.role is sr.SurfaceRole.CONTROL and sr.has_control_exception(n, policy):
        continue
    cands.append((n, c))

proc = subprocess.Popen(["git", "-C", git_dir, "cat-file", "--batch"],
                        stdin=subprocess.PIPE, stdout=subprocess.PIPE)
rows = []
for n, c in cands:
    proc.stdin.write(f"{commit}:{n}\n".encode()); proc.stdin.flush()
    hdr = proc.stdout.readline().split()
    size = int(hdr[2])
    data = proc.stdout.read(size); proc.stdout.read(1)
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        continue
    hits = list(sr.iter_machine_path_lines(text))
    if not hits:
        continue
    code = ("ABS_PATH_IN_UNCLASSIFIED_SURFACE" if c.role is sr.SurfaceRole.UNCLASSIFIED
            else "ABS_PATH_IN_PROJECT_SURFACE")
    rows.append((code, n, len(hits), hits[0], c.reason))
proc.stdin.close(); proc.wait()

with open(out, "w") as f:
    for r in rows:
        f.write("\t".join(map(str, r)) + "\n")
print(f"commit={commit} candidates={len(cands)} findings={len(rows)} "
      f"policy_issues={len(policy.issues)}")
for i in policy.issues:
    print("POLICY_ISSUE", i.code, i.message[:300])
