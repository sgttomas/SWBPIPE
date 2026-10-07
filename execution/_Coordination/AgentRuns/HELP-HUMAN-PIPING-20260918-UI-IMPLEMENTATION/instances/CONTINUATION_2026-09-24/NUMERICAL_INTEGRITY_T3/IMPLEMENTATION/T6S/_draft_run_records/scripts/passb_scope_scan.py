#!/usr/bin/env python3
"""I80: a read-only scope scan for Pass B (T6S). Standard library only; Git reads with GIT_OPTIONAL_LOCKS=0.

Usage: python3 passb_scope_scan.py --repo <checkout with NUM objects> --base <rev> --head <rev> --crate-dirs <crate_dirs.txt>

It does not run Pass B. It reads the diff base..head (maintained paths, execution excluded) and reports:
  1. each changed path, and whether it lies in a D1 crate directory (crate_dirs.txt's src dirs, or the
     crate root's Cargo.toml, Cargo.lock or build.rs);
  2. every include_str!/include_bytes!/include! literal in the D1 crate src dirs and PP's build.rs at
     head, resolved against its file, and whether any is a changed path;
  3. PP's REVIEWED_INPUTS (src/build_identity.rs) at head, resolved, and whether any is a changed path;
  4. tree equality of each D1 crate root between base and head, and blob equality of every resolved
     embedded file;
  5. every other text mention, in the D1 crate src dirs at head, of a changed path's file name;
  6. the named readers and carriers, base and head blobs;
  7. any Cargo.toml, Cargo.lock or build.rs among the changed paths, anywhere.
"""
import argparse, os, posixpath, re, subprocess, sys
ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True); ap.add_argument("--base", required=True); ap.add_argument("--head", required=True)
ap.add_argument("--crate-dirs", required=True)
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
P = "projects/chirality-piping/"
def git(*args, ok=(0,)):
    r = subprocess.run(["git", "-C", a.repo, *args], capture_output=True, env=env)
    if r.returncode not in ok: sys.exit(f"git {' '.join(args)}: {r.stderr.decode(errors='replace')}")
    return r.stdout.decode("utf-8", errors="replace")
def rp(rev, path): return git("rev-parse", "--verify", "--quiet", f"{rev}:{path}", ok=(0, 1, 128)).strip()
base, head = git("rev-parse", a.base).strip(), git("rev-parse", a.head).strip()
print(f"base {base}\nhead {head}")
changed = [n for n in git("diff", "--name-only", base, head, "--", ".", ":!" + P + "execution", ":!execution").split("\n") if n]
print(f"changed paths (maintained): {len(changed)}")
dirs = [P + d for d in open(a.crate_dirs).read().split()]
roots = sorted({d[: -len("/src")] for d in dirs})
print(f"D1 crate src dirs: {len(dirs)}; crate roots: {len(roots)}")
def in_d1(path):
    for d in dirs:
        if path.startswith(d + "/"): return f"in {d}"
    for r in roots:
        if path in (r + "/Cargo.toml", r + "/Cargo.lock", r + "/build.rs"): return f"crate file of {r}"
    return None
print("\n## 1. Changed paths")
hits1 = 0
for c in changed:
    w = in_d1(c); hits1 += bool(w)
    print(f"  {'D1  ' if w else 'out '} {c.replace(P, 'P/')}{'  <- ' + w if w else ''}")
print(f"  in a D1 crate dir or crate file: {hits1}")
print("\n## 2. Embedded literals in D1 crate src dirs and PP build.rs (at head)")
files = [f for f in git("ls-tree", "-r", "--name-only", head, "--", *dirs).split("\n") if f.endswith(".rs")]
files.append(P + "core/product_physics/build.rs")
rx = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*(?:concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*)?"([^"]+)"')
embeds, other = {}, 0
for f in files:
    text = git("cat-file", "blob", f"{head}:{f}", ok=(0, 128))
    for m in rx.finditer(text):
        if "//" in text[text.rfind("\n", 0, m.start()) + 1 : m.start()]: continue  # inside a line comment
        lit = m.group(1)
        if lit.startswith("/"):
            # concat!(env!("CARGO_MANIFEST_DIR"), "/…"): relative to the crate root
            root = next((r for r in roots if f.startswith(r + "/")), None)
            target = posixpath.normpath(posixpath.join(root, lit.lstrip("/"))) if root else None
        else:
            target = posixpath.normpath(posixpath.join(posixpath.dirname(f), lit))
        line = text[: m.start()].count("\n") + 1
        embeds.setdefault(target, []).append(f"{f.replace(P, 'P/')}:{line}")
print(f"  literals: {sum(len(v) for v in embeds.values())}, distinct targets: {len(embeds)}")
hits2 = [t for t in embeds if t in changed]
for t in sorted(embeds, key=str):
    tb, bb = rp(head, t) if t else "", rp(base, t) if t else ""
    same = "same blob" if tb and tb == bb else ("ABSENT" if not tb else "BLOB DIFFERS")
    print(f"  {'CHANGED ' if t in changed else ''}{(t or '?').replace(P, 'P/')}  [{same}]  x{len(embeds[t])}")
print(f"  embedded targets among the changed paths: {len(hits2)}")
print("\n## 3. PP REVIEWED_INPUTS (src/build_identity.rs, at head)")
bi = git("cat-file", "blob", f"{head}:{P}core/product_physics/src/build_identity.rs")
blk = re.search(r"REVIEWED_INPUTS: \[&str; (\d+)\] = \[(.*?)\];", bi, re.S)
ri = [posixpath.normpath(posixpath.join(P + "core/product_physics", x)) for x in re.findall(r'"([^"]+)"', blk.group(2))]
hits3 = [x for x in ri if x in changed]
for x in ri: print(f"  {x.replace(P, 'P/')}  [{'same blob' if rp(head, x) == rp(base, x) else 'BLOB DIFFERS'}]")
print(f"  declared {blk.group(1)}, read {len(ri)}; among the changed paths: {len(hits3)}")
print("\n## 4. D1 crate roots: tree equality base..head")
diff_roots = 0
for r in roots:
    same = rp(head, r) == rp(base, r); diff_roots += not same
    print(f"  {r.replace(P, 'P/')}  {'same tree' if same else 'TREE DIFFERS'}")
print(f"  roots whose tree differs: {diff_roots}")
diff_src = 0
for d in dirs:
    same = rp(head, d) == rp(base, d); diff_src += not same
    if not same: print(f"  src dir {d.replace(P, 'P/')} TREE DIFFERS")
print(f"  D1 crate src dirs whose tree differs: {diff_src} of {len(dirs)}")
for r in roots:
    if rp(head, r) != rp(base, r):
        inside = [c.replace(P, "P/") for c in changed if c.startswith(r + "/")]
        print(f"  changed paths inside {r.replace(P, 'P/')}: {inside}")
        for x in ("Cargo.toml", "Cargo.lock", "build.rs"):
            hb, bb = rp(head, f"{r}/{x}"), rp(base, f"{r}/{x}")
            print(f"    {x}: {'absent' if not hb and not bb else ('same blob' if hb == bb else 'DIFFERS')}")
print("\n## 5. Other mentions of a changed file's name in the D1 crate src dirs (at head)")
names = sorted({posixpath.basename(c) for c in changed})
mention = 0
for n in names:
    out = git("grep", "-n", "-F", n, head, "--", *dirs, ok=(0, 1)).strip()
    for l in out.split("\n") if out else []:
        mention += 1; print("  " + l.replace(head + ":", "").replace(P, "P/")[:220])
print(f"  mentions: {mention}")
print("\n## 6. Readers and carriers named in T6S_COMMON's never-touched list")
readers = [P + "core/reporting/result_export/src/retained_precision.rs", P + "core/reporting/result_export/src/derivative.rs",
           P + "core/reporting/result_export/src/semantic_contract.rs", P + "core/reporting/result_export/src/source_blocks.rs",
           P + "apps/desktop/src/features/results/retainedPrecision.ts", P + "apps/desktop/src/features/results/retainedPrecisionStanding.ts",
           P + "apps/desktop/src/features/results/knownSemanticLimitations.ts"]
readers += [f for f in git("ls-tree", "-r", "--name-only", head, "--", P + "core/analysis_runs", P + "core/handoff/stress_neutral").split("\n") if f.endswith(".py")]
for x in readers:
    print(f"  {x.replace(P, 'P/')}  [{'same blob' if rp(head, x) == rp(base, x) else 'CHANGED'}]")
print("\n## 7. Manifests, locks and build scripts among the changed paths, anywhere")
mf = [c for c in changed if posixpath.basename(c) in ("Cargo.toml", "Cargo.lock", "build.rs", "package.json", "package-lock.json", "pnpm-lock.yaml", "requirements.txt", "pyproject.toml")]
print(f"  {len(mf)}" + "".join(f"\n  {c}" for c in mf))
print("\nSUMMARY changed_in_d1_dirs={} embedded_changed={} reviewed_inputs_changed={} d1_src_dirs_differ={} d1_roots_differ={} name_mentions={} manifests_changed={}".format(
    hits1, len(hits2), len(hits3), diff_src, diff_roots, mention, len(mf)))
