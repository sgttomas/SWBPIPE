#!/usr/bin/env python3
"""KF1 mutants (addendum 2, RV20's M4 and M5 with RV20's exact edits; clean copies of BASE_REV plus the candidate's changes): one clean copy of the candidate FK tree and one fresh target per
mutant, exact string edits with their match counts checked, serialized (one
cargo job at -j 4, RUST_TEST_THREADS=2); NONE first. Kill = a failing test of
`cargo test --lib kf1_`. Each target is deleted afterwards."""
import json, os, shutil, subprocess, sys, time
WT = sys.argv[1]            # <wt>
OUT = sys.argv[2]           # results file
SRC = os.path.join(WT, "kf1/projects/chirality-piping/core/solver/frame_kernel")
REL = "src/structural/retained/adaptive.rs"
THROWAWAY = '&mut WideContext::<16>::new(1024).expect("p")'
M = [
 ("NONE", "no edit", []),
 ("RV20-M4", "the shared cap's collapse work is not charged (throwaway ctx16) [RV20's edit]",
  [("                    t.collapse(ctx16);", "                    t.collapse(" + THROWAWAY + ");", 1)]),
 ("RV20-M5", "RuleTest ordered (d), (a), (b): trackers finished in another order [RV20's edit]",
  [("enum RuleTest {\n    /// (a): (|\u0394| + V)/M.\n    Disagreement,\n    /// (b): \u0174/V.\n    Estimate,\n"
    "    /// (d): C over its allowance.\n    Charge,\n}",
    "enum RuleTest {\n    /// (d): C over its allowance.\n    Charge,\n    /// (a): (|\u0394| + V)/M.\n"
    "    Disagreement,\n    /// (b): \u0174/V.\n    Estimate,\n}", 1)]),
]
only = sys.argv[3].split(",") if len(sys.argv) > 3 else None
BASE_REV = sys.argv[4] if len(sys.argv) > 4 else "HEAD"
env = dict(os.environ, RUSTUP_TOOLCHAIN="1.97.1", RUSTUP_AUTO_INSTALL="0",
           CARGO_INCREMENTAL="0", RUST_TEST_THREADS="2")
for name, what, edits in M:
    if only and name not in only:
        continue
    root = os.path.join(WT, "kf1-mut", name)
    target = os.path.join(WT, "kf1-mut", name + "-target")
    shutil.rmtree(root, ignore_errors=True); shutil.rmtree(target, ignore_errors=True)
    # A clean copy of BASE_REV's FK tree, with the candidate's changed files
    # (`git diff --name-only BASE_REV` under FK) overlaid from the worktree.
    os.makedirs(root)
    fk_rel = "projects/chirality-piping/core/solver/frame_kernel"
    repo = os.path.join(WT, "kf1")
    archive = subprocess.run(["git", "-C", repo, "archive", BASE_REV, fk_rel],
                             capture_output=True, check=True).stdout
    subprocess.run(["tar", "-x", "-C", root], input=archive, check=True)
    shutil.move(os.path.join(root, fk_rel), os.path.join(root, "frame_kernel"))
    shutil.rmtree(os.path.join(root, "projects"))
    changed = subprocess.run(["git", "-C", repo, "diff", "--name-only", BASE_REV, "--", fk_rel],
                             capture_output=True, text=True, check=True).stdout.split()
    changed += subprocess.run(["git", "-C", repo, "ls-files", "--others", "--exclude-standard", "--", fk_rel],
                              capture_output=True, text=True, check=True).stdout.split()
    for path in changed:
        rel = os.path.relpath(path, fk_rel)
        shutil.copyfile(os.path.join(repo, path), os.path.join(root, "frame_kernel", rel))
    print(json.dumps({"mutant": name, "overlaid": sorted(os.path.relpath(p, fk_rel) for p in changed)}), flush=True)
    path = os.path.join(root, "frame_kernel", REL)
    text = open(path).read()
    for old, new, count in edits:
        n = text.count(old)
        assert n == count, (name, old[:60], n)
        text = text.replace(old, new)
    open(path, "w").write(text)
    t0 = time.time()
    p = subprocess.run(["cargo", "test", "--offline", "--locked", "-j", "4", "--lib", "kf1_"],
                       cwd=os.path.join(root, "frame_kernel"),
                       env=dict(env, CARGO_TARGET_DIR=target),
                       capture_output=True, text=True)
    log = p.stdout + p.stderr
    open(os.path.join(WT, "kf1-mut", name + ".log"), "w").write(log)
    failed = [l.split()[1] for l in log.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    built = "could not compile" not in log
    rec = {"mutant": name, "edit": what, "exit": p.returncode, "built": built,
           "failed": failed, "killed": built and p.returncode != 0,
           "seconds": round(time.time() - t0, 1)}
    print(json.dumps(rec), flush=True)
    open(OUT, "a").write(json.dumps(rec) + "\n")
    shutil.rmtree(target, ignore_errors=True)
