#!/usr/bin/env python3
"""U9 SOURCE_EQUALITY: prove the compact PR carries exactly the reviewed maintained source.

Read-only, standard library only. The only Git commands are reads, run with GIT_OPTIONAL_LOCKS=0:
`diff`, `ls-tree`, `cat-file`, `merge-base`, `log`, `rev-parse`, and `merge-file -p` on files in
--work. The script writes only --work and --json.

Usage:
  python3 source_equality.py --repo <checkout> --pr <PR head> --int <integration head> \
      [--main <main SHA>] --work <dir under WT/scratch> [--json <out.json>] \
      [--package <repo path of the PR's evidence package>]

  --pr    the PR head (any commit on the compact branch; the S blobs must be the same on each)
  --int   the reviewed integration head (the U7/NUM head the source was reviewed at)
  --main  the main commit the PR is cut from (default: git merge-base <pr> origin/main)

  B, the integration branch's base, is `git merge-base <int> <main>`.
  S, the source set, is `git diff --name-only B <int>`, minus execution paths.

Exit 0 only when all five checks pass. Each check prints PASS or FAIL with its evidence.

  1. The PR's non-execution paths equal S: `diff --name-only <main> <pr>` against S, as sorted lists.
  2. Every S path except the known resolutions has the same blob and mode at <int> and <pr>.
  3. Known resolutions (U9 decision 3). P/core/analysis_runs/compatibility.py is re-derived as a
     three-way merge-file (<int>, B, <main>). It must contain exactly one conflict: the
     `_same_canonical` helper added on both sides. Main's side must occur byte for byte inside the
     integration side, and the resolution keeps the integration side. The result must equal
     <pr>'s blob. Any other S path that main changed since B is a FAIL; it needs a ruling.
  4. The PR's execution paths lie only under the evidence package (when --package is given). The
     package's SHA256SUMS verify against the PR's blobs.
  5. The per-path table: blob at <int>, blob at <pr>, mode, equal flag and the last <int> commit
     touching the path. It is printed as a summary and written in full to --json.
"""
import argparse, hashlib, json, os, subprocess, sys

P = "projects/chirality-piping/"
EXCLUDE = [":!" + P + "execution", ":!execution"]
KNOWN_RESOLUTIONS = {P + "core/analysis_runs/compatibility.py": "decision 3: one _same_canonical, main's two call-site lines"}
HELPER = "def _same_canonical("

ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True)
ap.add_argument("--pr", required=True)
ap.add_argument("--int", dest="integ", required=True)
ap.add_argument("--main")
ap.add_argument("--work", required=True)
ap.add_argument("--json")
ap.add_argument("--package")
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")

def git(*args, ok=(0,), raw=False):
    r = subprocess.run(["git", "-C", a.repo, *args], capture_output=True, env=env)
    if r.returncode not in ok:
        sys.exit(f"git {' '.join(args)} failed: {r.stderr.decode(errors='replace')}")
    return r.stdout if raw else r.stdout.decode()

def rev(x): return git("rev-parse", "--verify", x + "^{commit}").strip()
PR, INT = rev(a.pr), rev(a.integ)
MAIN = rev(a.main) if a.main else git("merge-base", PR, "origin/main").strip()
B = git("merge-base", INT, MAIN).strip()
def names(x, y, *spec): return sorted(n for n in git("diff", "--name-only", x, y, "--", *spec).split("\n") if n)
NONEXEC = [".", *EXCLUDE]
S = names(B, INT, *NONEXEC)
results, ok_all = {}, True
def report(n, ok, msg):
    global ok_all
    ok_all &= ok
    results[f"check{n}"] = {"pass": ok, "detail": msg}
    print(f"[{'PASS' if ok else 'FAIL'}] check {n}: {msg if isinstance(msg, str) else json.dumps(msg)[:600]}")
print(f"PR {PR}\nINT {INT}\nMAIN {MAIN}\nB {B}\n|S| = {len(S)}")

# 1. PR's non-execution paths == S
prn = names(MAIN, PR, *NONEXEC)
extra, missing = sorted(set(prn) - set(S)), sorted(set(S) - set(prn))
report(1, not extra and not missing, "PR non-execution paths equal S" if not extra and not missing
       else {"in_pr_not_S": extra, "in_S_not_pr": missing})

# 2. blob and mode identity outside the known resolutions
def tree(x, paths):
    out = {}
    for i in range(0, len(paths), 200):
        for line in git("ls-tree", "-r", x, "--", *paths[i:i + 200]).split("\n"):
            if line:
                meta, path = line.split("\t", 1); mode, _, blob = meta.split()
                out[path] = (mode, blob)
    return out
ti, tp = tree(INT, S), tree(PR, S)
plain = [s for s in S if s not in KNOWN_RESOLUTIONS]
diff2 = [s for s in plain if ti.get(s) != tp.get(s)]
report(2, not diff2, f"{len(plain)} paths identical in blob and mode" if not diff2 else {"differ": diff2})

# 3. the known resolutions, and no other S path moved on main
moved = set(names(B, MAIN, "."))
unruled = sorted((moved & set(S)) - set(KNOWN_RESOLUTIONS))
detail3, ok3 = {}, not unruled
if unruled: detail3["main_moved_unruled_S_paths"] = unruled
os.makedirs(a.work, exist_ok=True)
for path, why in KNOWN_RESOLUTIONS.items():
    if path not in moved:          # main no longer differs here: plain identity is required instead
        same = ti.get(path) == tp.get(path); ok3 &= same
        detail3[path] = "main unchanged since B; identical" if same else "main unchanged since B; DIFFERS"; continue
    f = {}
    for tag, c in (("int", INT), ("base", B), ("main", MAIN)):
        f[tag] = os.path.join(a.work, f"compat_{tag}.py")
        open(f[tag], "wb").write(git("cat-file", "blob", f"{c}:{path}", raw=True))
    r = subprocess.run(["git", "merge-file", "-p", f["int"], f["base"], f["main"]], capture_output=True, env=env)
    text = r.stdout.decode()
    out, mode, conflicts, ours, theirs = [], 0, 0, [], []
    for line in text.splitlines(keepends=True):
        if line.startswith("<<<<<<< "): mode, conflicts = 1, conflicts + 1; continue
        if line.startswith("||||||| ") and mode: mode = 2; continue
        if line.rstrip("\n") == "=======" and mode: mode = 3; continue
        if line.startswith(">>>>>>> ") and mode: mode = 0; continue
        if mode == 1: ours.append(line)
        if mode == 3: theirs.append(line)
        if mode in (0, 1): out.append(line)
    resolved = "".join(out).encode()
    o, t = "".join(ours), "".join(theirs)
    helper_ok = conflicts == 1 and HELPER in t and t.strip() in o
    blob = hashlib.sha1(b"blob %d\0" % len(resolved) + resolved).hexdigest()
    pr_blob = tp.get(path, (None, None))[1]
    good = helper_ok and blob == pr_blob
    ok3 &= good
    open(os.path.join(a.work, "compat_expected.py"), "wb").write(resolved)
    detail3[path] = {"rule": why, "conflicts": conflicts, "main_helper_inside_int_side": helper_ok,
                     "expected_blob": blob, "pr_blob": pr_blob, "equal": blob == pr_blob}
report(3, ok3, detail3)

# 4. execution paths only inside the package; the package's SHA256SUMS verify on the PR's blobs
exe = names(MAIN, PR, P + "execution")
if a.package:
    pkg = a.package.rstrip("/") + "/"
    outside = [n for n in exe if not n.startswith(pkg)]
    bad = []
    try:
        sums = git("cat-file", "blob", f"{PR}:{pkg}SHA256SUMS")
        for line in sums.split("\n"):
            if line.strip():
                h, name = line.split(None, 1); name = name.lstrip("*").strip()
                data = git("cat-file", "blob", f"{PR}:{pkg}{name}", raw=True)
                if hashlib.sha256(data).hexdigest() != h: bad.append(name)
    except SystemExit:
        bad.append("SHA256SUMS unreadable")
    report(4, not outside and not bad, {"execution_files": len(exe), "outside_package": outside, "sha256_mismatch": bad})
else:
    report(4, False, {"execution_files": len(exe), "note": "no --package given; list not verified", "paths": exe[:20]})

# 5. per-path table
def last(path): return git("log", "-1", "--format=%H", INT, "--", path).strip()
table = [{"path": s, "int_blob": ti.get(s, (None, None))[1], "pr_blob": tp.get(s, (None, None))[1],
          "mode": tp.get(s, (None, None))[0], "equal": ti.get(s) == tp.get(s),
          "resolution": KNOWN_RESOLUTIONS.get(s), "int_last_commit": last(s)} for s in S]
unexplained = [r["path"] for r in table if not r["equal"] and not r["resolution"]]
report(5, not unexplained, f"{len(table)} rows; {sum(r['equal'] for r in table)} equal; "
       f"{sum(1 for r in table if r['resolution'])} known resolution(s); unexplained {unexplained}")
if a.json:
    json.dump({"pr": PR, "int": INT, "main": MAIN, "base": B, "checks": results, "table": table},
              open(a.json, "w"), indent=1, sort_keys=True)
print("RESULT", "PASS" if ok_all else "FAIL")
sys.exit(0 if ok_all else 1)
