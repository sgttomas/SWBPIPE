#!/usr/bin/env python3
"""I89 redaction 01 (ROOT's request): re-sanitize, from the kept raw sources, every committed
file under R/I89/b1_sa_01/_run_records/ that carries a home-relative path form, with the
corrected sanitizer (WT four levels up). For each file it first proves provenance: the first
sanitizer (sanitize_v1.py, as run) applied to the raw source reproduces the committed bytes
exactly. Then it writes the corrected sanitizer's output to a staging folder, and checks that
every changed line differs only in its path forms. --install copies the staged files in place.

Usage: redact_01.py RECORDS_DIR STAGE_DIR [--install]
"""
import hashlib, json, os, re, subprocess, sys
REC, STAGE = sys.argv[1:3]
install = "--install" in sys.argv
WT = "WT"
S = WT + "/scratch/i89_b1_sa"
PY = "VENV/bin/python"
V1, V2 = S + "/scripts/sanitize_v1.py", S + "/scripts/sanitize.py"
BAD = re.compile("|".join(map(re.escape, ["~" + "/", "/" + "Users/", "/" + "private/", ".claude" + "/worktrees", "swbpipe" + "-control-layer"])))
def source(rel):
    d, f = os.path.split(rel)
    if rel == "cargo_jobs_i89.log": return S + "/tmp/cargo_jobs_i89.log", False
    if d == "scripts": return S + "/scripts/" + f, False
    if d == "mutants": return S + "/logs/mutants/" + f.replace(".filtered.log", ".log"), True
    if f.endswith(".filtered.log"): return S + "/logs/" + f.replace(".filtered.log", ".log"), True
    return S + "/logs/" + f, False
def run(san, src, filt, dst):
    subprocess.run([PY, san] + (["--filter"] if filt else []) + [src, dst], check=True)
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
# The v1 forms and what they stand for (to check that only path forms changed).
HOME = os.path.expanduser("~")
HOME_T3 = "~" + WT[len(HOME):]                                     # the first sanitizer's form of WT
HOME_VENV = "~" + os.path.dirname(os.path.dirname(PY))[len(HOME):]  # and of VENV
SCRATCH_NAMES = sorted(os.listdir(WT + "/scratch"), key=len, reverse=True)
def normalize_v1(line):
    line = line.replace(HOME_VENV, "VENV").replace(HOME_T3, "WTROOT")
    for n in SCRATCH_NAMES:  # v1's "WT" was WT/scratch
        line = re.sub(r"(?<![A-Za-z0-9_])WT/" + re.escape(n) + r"(?![A-Za-z0-9_.-])", "WTROOT/scratch/" + n, line)
    return line.replace("WTROOT", "WT")
affected = []
for root, _, files in os.walk(os.path.join(REC, "_run_records")):
    for f in files:
        p = os.path.join(root, f)
        rel = os.path.relpath(p, os.path.join(REC, "_run_records"))
        if rel.startswith("followup_01/"): continue
        if BAD.search(open(p, encoding="utf-8", errors="replace").read()): affected.append(rel)
affected.sort()
report = []
os.makedirs(STAGE, exist_ok=True)
for rel in affected:
    src, filt = source(rel)
    old = os.path.join(REC, "_run_records", rel)
    v1 = os.path.join(STAGE, "v1_" + rel.replace("/", "__"))
    new = os.path.join(STAGE, rel.replace("/", "__"))
    run(V1, src, filt, v1)
    provenance = open(v1, "rb").read() == open(old, "rb").read()
    run(V2, src, filt, new)
    a = open(old, encoding="utf-8").read().split("\n"); b = open(new, encoding="utf-8").read().split("\n")
    same_count = len(a) == len(b)
    changed = [(x, y) for x, y in zip(a, b) if x != y]
    path_only = [normalize_v1(x) == y for x, y in changed]
    cut = [("[... cut:" in x or "[... cut:" in y) and not ok for (x, y), ok in zip(changed, path_only)]
    other = [i for i, ok in enumerate(path_only) if not ok and not cut[i]]
    clean = not BAD.search(open(new, encoding="utf-8").read())
    report.append({"file": rel, "source": os.path.relpath(src, S), "filter": filt, "provenance_v1_reproduces_committed": provenance,
                   "lines": len(a), "same_line_count": same_count, "changed_lines": len(changed),
                   "changed_only_in_path_forms": sum(path_only), "cut_line_digests_changed": sum(cut),
                   "other_changes": len(other), "new_clean": clean,
                   "old_sha256": sha(old), "new_sha256": sha(new)})
    print(json.dumps({k: report[-1][k] for k in ("file", "provenance_v1_reproduces_committed", "same_line_count", "changed_lines", "changed_only_in_path_forms", "cut_line_digests_changed", "other_changes", "new_clean")}))
json.dump(report, open(os.path.join(STAGE, "report.json"), "w"), indent=1)
ok = all(r["provenance_v1_reproduces_committed"] and r["same_line_count"] and r["other_changes"] == 0 and r["new_clean"] for r in report)
print("files:", len(report), "all checks:", ok)
if install and ok:
    for rel in affected:
        open(os.path.join(REC, "_run_records", rel), "wb").write(open(os.path.join(STAGE, rel.replace("/", "__")), "rb").read())
    print("installed", len(affected))
