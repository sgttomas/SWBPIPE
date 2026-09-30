#!/usr/bin/env python3
"""KF1 mutants (addendum 1: clean copies of BASE_REV plus the candidate's changes): one clean copy of the candidate FK tree and one fresh target per
mutant, exact string edits with their match counts checked, serialized (one
cargo job at -j 4, RUST_TEST_THREADS=2); NONE first. Kill = a failing test of
`cargo test --lib kf1_`. Each target is deleted afterwards."""
import json, os, shutil, subprocess, sys, time
WT = sys.argv[1]            # <wt>
OUT = sys.argv[2]           # results file
SRC = os.path.join(WT, "kf1/projects/chirality-piping/core/solver/frame_kernel")
REL = "src/structural/retained/adaptive.rs"
M = [
 ("NONE", "no edit", []),
 ("KF1-M1", "the prune ranks ratios in the wrong direction (keeps the least extreme)",
  [("                Direction::Up => a > b,\n                Direction::Down => a < b,\n            },\n        }\n    }\n}",
    "                Direction::Up => a < b,\n                Direction::Down => a > b,\n            },\n        }\n    }\n}", 1)]),
 ("KF1-M2", "the collapse drops the evaluated entries",
  [("            self.table.push((*key, outcome));", "            let _ = (key, outcome);", 1)]),
 ("KF1-M3", "the collapse evaluates with the opposite rounding",
  [("            let outcome = match directed_ratio(ctx16, num, den, self.direction) {",
    "            let opposite = match self.direction {\n                Direction::Up => Direction::Down,\n                Direction::Down => Direction::Up,\n            };\n            let outcome = match directed_ratio(ctx16, num, den, opposite) {", 1)]),
 ("KF1-M4", "a new best does not prune the table",
  [("            self.table.retain(|entry| in_window(entry.0, key));\n", "", 1)]),
 ("KF1-M5", "dominance by the farther key (the prune sorts farthest first)",
  [("                Direction::Up => b.0.cmp(&a.0),\n                Direction::Down => a.0.cmp(&b.0),",
    "                Direction::Up => a.0.cmp(&b.0),\n                Direction::Down => b.0.cmp(&a.0),", 1)]),
 ("KF1-M6", "a refusal met at a collapse is swallowed (recorded as +inf)",
  [("                Err(stop) => Evaluated::Refused { seq: *seq, stop },",
    "                Err(_) => Evaluated::Ratio(f64::INFINITY),", 1)]),
 ("KF1-M6b", "a refusal met at a collapse survives the window (returned whatever follows)",
  [("            self.table.retain(|entry| in_window(entry.0, key));",
    "            self.table.retain(|entry| {\n                in_window(entry.0, key) || matches!(entry.1, Evaluated::Refused { .. })\n            });", 1)]),
 ("KF1-M7", "no collapse (T never reached)",
  [("            if self.lazy.len() >= self.limit {", "            if self.lazy.len() >= usize::MAX {", 1)]),
 ("KF1-M8", "the shared cap G is ignored",
  [("        if self.held > self.limit {", "        if false && self.held > self.limit {", 1)]),
 ("KF1-M9", "the table's window is one ulp narrower (< instead of <=)",
  [("            self.table.retain(|entry| in_window(entry.0, key));",
    "            self.table.retain(|entry| entry.0.abs_diff(key) < WINDOW_ULPS);", 1)]),
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
