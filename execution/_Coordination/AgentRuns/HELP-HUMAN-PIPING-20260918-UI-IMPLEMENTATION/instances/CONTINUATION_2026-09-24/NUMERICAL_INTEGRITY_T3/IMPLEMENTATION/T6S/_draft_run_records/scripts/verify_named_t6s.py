#!/usr/bin/env python3
"""I80: check the `named_references` of T6S's citations.json, which main's check_citations.py ignores.

Adapted from I77's verify_named.py (R/I77/u8_package_01/_run_records/scripts/). Read-only, standard
library only; Git reads with GIT_OPTIONAL_LOCKS=0 (`cat-file`, `rev-parse`, `log`, `merge-base
--is-ancestor`, `branch -r --contains`). Writes nothing.

Usage: python3 verify_named_t6s.py --repo <checkout with NUM objects> --index citations.json [--main <main>]

For every entry:
  - each cited_at site (relative to projects/chirality-piping/, at the index's source_basis) matches
    the entry's site_regex, so the index describes the source it claims to;
  - each resolves_to target holds its `contains` text at the stated line: num_path (under records_root)
    and rr_line (in rr_path, also inside the stated heading) at num_commit, repo_path at its rev;
  - a commit exists, has the stated subject, is reachable from the stated remote branch, and is or is
    not an ancestor of num_commit and of main as the entry states.
Exit 1 on any failure.
"""
import argparse, json, os, re, subprocess, sys
ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True); ap.add_argument("--index", required=True)
ap.add_argument("--main", default="f8ed4f055126cf19315d2d8b06d7d11796786a82")
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def git(*args, ok=(0,)):
    r = subprocess.run(["git", "-C", a.repo, *args], capture_output=True, env=env)
    if r.returncode not in ok: sys.exit(f"git {' '.join(args)}: {r.stderr.decode(errors='replace')}")
    return r
idx = json.load(open(a.index, encoding="utf-8"))
NUM, HEAD, R, RR = idx["num_commit"], idx["source_basis"], idx["records_root"], idx["rr_path"]
P = "projects/chirality-piping/"
cache = {}
def lines(rev, path):
    if (rev, path) not in cache:
        r = git("cat-file", "blob", f"{rev}:{path}", ok=(0, 128))
        cache[(rev, path)] = r.stdout.decode("utf-8", errors="replace").split("\n") if r.returncode == 0 else None
    return cache[(rev, path)]
fails, checks = [], 0
def fail(e, why): fails.append(f"{e['token']}: {why}")
def at(text, n): return text[n - 1] if text and 0 < n <= len(text) else ""
for e in idx["named_references"]:
    rx = re.compile(e["site_regex"])
    for site in e["cited_at"]:
        path, n = site.rsplit(":", 1)
        checks += 1
        text = lines(HEAD, P + path)
        if text is None: fail(e, f"no file {path} at {HEAD[:10]}"); continue
        if not rx.search(at(text, int(n))): fail(e, f"{site} does not match {e['site_regex']!r}")
    for t in e["resolves_to"]:
        checks += 1
        if "num_path" in t:
            text = lines(NUM, f"{R}/{t['num_path']}")
            if text is None: fail(e, f"missing at NUM: {t['num_path']}"); continue
            if t["contains"] not in at(text, t["line"]): fail(e, f"{t['num_path']}:{t['line']} lacks {t['contains']!r}")
        elif "rr_line" in t:
            text = lines(NUM, RR)
            if t["contains"] not in at(text, t["rr_line"]): fail(e, f"RR:{t['rr_line']} lacks {t['contains']!r}")
            hl = max(i + 1 for i in range(t["rr_line"]) if text[i].startswith("## "))
            if hl != t["heading_line"]: fail(e, f"RR:{t['rr_line']} is under {hl}, not {t['heading_line']}")
        elif "repo_path" in t:
            text = lines(t["rev"], t["repo_path"])
            if text is None or t["contains"] not in at(text, t["line"]): fail(e, f"{t['repo_path']}:{t['line']} at {t['rev'][:10]} lacks {t['contains']!r}")
            mb = git("rev-parse", "--verify", "--quiet", f"{a.main}:{t['repo_path']}", ok=(0, 1, 128)).stdout.decode().strip()
            hb = git("rev-parse", f"{t['rev']}:{t['repo_path']}").stdout.decode().strip()
            print(f"  {e['token']}: line {t['line']} blob at rev {hb[:12]}, at main {mb[:12] or '-'}")
        elif "commit" in t:
            c = t["commit"]
            subj = git("log", "-1", "--format=%s", c, ok=(0, 128)).stdout.decode().strip()
            if subj != t["subject"]: fail(e, f"commit {c[:10]} subject {subj!r}")
            remotes = git("branch", "-r", "--contains", c).stdout.decode().split()
            in_num = git("merge-base", "--is-ancestor", c, NUM, ok=(0, 1)).returncode == 0
            in_main = git("merge-base", "--is-ancestor", c, a.main, ok=(0, 1)).returncode == 0
            print(f"  {e['token']}: remote branches {remotes}; ancestor of NUM {in_num}; of main {in_main}")
            if t["reachable_from"] not in remotes or in_num != t["ancestor_of_num"] or in_main != t["ancestor_of_main"]:
                fail(e, "reachability differs from the index")
        else:
            fail(e, f"unknown target {t}")
for f in fails: print("FAILED", f)
print(f"named_references: {len(idx['named_references'])} entries, {sum(len(e['cited_at']) for e in idx['named_references'])} sites, {checks} checks, {len(fails)} failed")
print("RESULT", "FAIL" if fails else "PASS")
sys.exit(1 if fails else 0)
