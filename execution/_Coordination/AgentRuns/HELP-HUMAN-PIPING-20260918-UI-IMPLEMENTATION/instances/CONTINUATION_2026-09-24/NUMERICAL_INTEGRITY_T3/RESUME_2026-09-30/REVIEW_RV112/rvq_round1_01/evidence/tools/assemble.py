#!/usr/bin/env python3
"""RV112: assemble the review's evidence/ from scratch, placeholder paths only (sanitize.py).

Usage: assemble.py <records dir>   (writes <records dir>/evidence/...; REVIEW.md is written separately)
Logs are filtered to what the review cites: job markers, `test … ok|FAILED|ignored` lines, `test result`
lines, panics with their messages, and the RV112_/I65_G5_/I89_ lines the tests print.
"""
import glob, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import sanitize  # noqa: E402

S = os.path.dirname(HERE)
OUT = os.path.join(sys.argv[1], "evidence")
KEEP = re.compile(r"(RV112_JOB|^test |test result|panicked at|assertion|left:|right:|RV112_|I65_G5_|I89_B1_SA|^\s+Running |warning: unused|^failures:|^    [a-z_:]+$)")


def put(rel, text):
    path = os.path.join(OUT, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(sanitize.clean(text))


def copy(src, rel):
    put(rel, open(src, encoding="utf-8", errors="replace").read())


def filtered(src, rel):
    lines = open(src, encoding="utf-8", errors="replace").read().splitlines()
    keep, i = [], 0
    while i < len(lines):
        line = lines[i]
        if KEEP.search(line):
            keep.append(line)
            if "panicked at" in line:
                for nxt in lines[i + 1:i + 6]:
                    if nxt.startswith("note:") or nxt.startswith("test ") or nxt.startswith("thread '"):
                        break
                    keep.append(nxt)
        i += 1
    keep = [l if len(l) <= 2000 else l[:2000] + f" ...[{len(l)} chars; truncated by RV112]" for l in keep]
    put(rel, "\n".join(keep) + "\n")


def main():
    if os.path.exists(OUT):
        shutil.rmtree(OUT)
    for f in glob.glob(f"{S}/evidence/basis/*") + glob.glob(f"{S}/evidence/identity/*") + glob.glob(f"{S}/evidence/oracle/*") \
            + glob.glob(f"{S}/evidence/record/*"):
        copy(f, os.path.relpath(f, f"{S}/evidence"))
    copy(f"{S}/oracle/gate_bounds_s3.py", "oracle/gate_bounds_s3.py")
    for f in sorted(glob.glob(f"{S}/logs/*.log")):
        filtered(f, "suites/" + os.path.basename(f).replace(".log", ".filtered.log"))
    for f in sorted(glob.glob(f"{S}/logs/*.rc")) + sorted(glob.glob(f"{S}/logs/*.done")) + sorted(glob.glob(f"{S}/logs/*.steps")):
        copy(f, "suites/" + os.path.basename(f))
    for f in sorted(glob.glob(f"{S}/identity/*")):
        if f.endswith(".log"):
            filtered(f, "identity/run_" + os.path.basename(f).replace(".log", ".filtered.log"))
        else:
            copy(f, "identity/run_" + os.path.basename(f))
    for f in sorted(glob.glob(f"{S}/mutants/*")):
        b = os.path.basename(f)
        if b.endswith(".log"):
            filtered(f, "mutants/" + b.replace(".log", ".filtered.log"))
        elif b.endswith(".rs"):
            continue  # the full schemata source is reproducible from mut_vs_head.diff
        else:
            copy(f, "mutants/" + b)
    for f in sorted(glob.glob(f"{S}/final/*")):
        if f.endswith(".log"):
            filtered(f, "probe/" + os.path.basename(f).replace(".log", ".filtered.log"))
        else:
            copy(f, "probe/" + os.path.basename(f))
    copy(f"{S}/mut_vs_head.diff", "mutants/mut_vs_head.diff")
    for f in sorted(glob.glob(f"{S}/derived/*")):
        copy(f, "derived/" + os.path.basename(f))
    for f in sorted(glob.glob(f"{HERE}/*")):
        copy(f, "tools/" + os.path.basename(f))
    jobs = [l for l in open(os.path.join(sanitize.WT, "guard/cargo_jobs.log"), encoding="utf-8") if "/rv112/" in l or "rv112_rvq_01" in l]
    put("cargo_jobs_rv112.log", "".join(jobs))


if __name__ == "__main__":
    main()
