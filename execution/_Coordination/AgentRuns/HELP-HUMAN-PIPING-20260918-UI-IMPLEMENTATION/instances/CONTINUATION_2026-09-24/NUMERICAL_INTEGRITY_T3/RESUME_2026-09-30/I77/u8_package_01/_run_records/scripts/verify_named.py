#!/usr/bin/env python3
"""I77: check the `named_references` of U8's citations.json, which main's check_citations.py ignores.

Read-only, standard library only; Git reads with GIT_OPTIONAL_LOCKS=0 (`cat-file`, `rev-parse`,
`merge-base --is-ancestor`, `branch -r --contains`, `log`). Writes nothing.

Usage: python3 verify_named.py --repo <checkout with NUM and the U8 branch> --index citations.json

For every entry:
  - each cited_at site (relative to projects/chirality-piping/, at the index's source_basis) holds one
    of the entry's site tokens, so the index describes the source it claims to;
  - each resolves_to target holds its `contains` text at the stated line, at the index's num_commit
    (num_path, under records_root; rr_line, in rr_path, also inside the stated heading), or at the
    stated rev (repo_path); a commit exists, has the stated subject, is reachable from the stated
    remote branch, and is an ancestor of neither num_commit nor main (`--main`, default c1bfc460fc).
Exit 1 on any failure.
"""
import argparse, json, os, re, subprocess, sys

ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True); ap.add_argument("--index", required=True); ap.add_argument("--main", default="c1bfc460fc")
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
    k = (rev, path)
    if k not in cache:
        r = git("cat-file", "blob", f"{rev}:{path}", ok=(0, 128))
        cache[k] = r.stdout.decode("utf-8", errors="replace").split("\n") if r.returncode == 0 else None
    return cache[k]
SITE = {
    "R/I68/u8_probe_01 §3": ["§3"], "RV93 N-5": ["RV93 N-5"], "D-U6-5": ["D-U6-5"], "decision 6": ["decision 6", "Decision 6"],
    "W-C1, W-C2": ["W-C1", "W-C2"], "U8-0 probe; I68": ["U8-0", "I68"], "I69": ["I69"],
    "W6's body; PHYS-R4's cantilever; retained_memory_witness_tests.rs :181–199": ["retained_memory_witness_tests.rs", ":181–199"],
    "U5 criterion": ["U5"], "RV94 N-3": ["RV94 N-3"], "D-U7-2": ["D-U7-2"], "RV90 S1, N1, N2, N4": ["RV90"], "C04": ["C04"],
    "D11, D37": ["D11", "D37"],
}
fails, checks = [], 0
def fail(e, why): fails.append(f"{e['token']}: {why}")
for e in idx["named_references"]:
    tokens = SITE.get(e["token"]) or ([e["token"].split()[0]] if e["kind"] == "commit" else None)
    if not tokens: fail(e, "no site tokens"); continue
    path = None
    for site in e["cited_at"]:
        m = re.match(r"^(?:(.+?))?:(\d+)(?:–(\d+))?$", site)
        if m.group(1): path = m.group(1)
        lo, hi = int(m.group(2)), int(m.group(3) or m.group(2))
        text = lines(HEAD, P + path)
        checks += 1
        if text is None: fail(e, f"no file {path} at {HEAD[:10]}"); continue
        if not any(t in text[n - 1] for n in range(lo, hi + 1) for t in tokens): fail(e, f"{path}:{lo}–{hi} holds none of {tokens}")
    for t in e["resolves_to"]:
        checks += 1
        if "num_path" in t:
            text = lines(NUM, f"{R}/{t['num_path']}")
            if text is None: fail(e, f"missing at NUM: {t['num_path']}"); continue
            if "line" in t and t["contains"] not in text[t["line"] - 1]: fail(e, f"{t['num_path']}:{t['line']} lacks {t['contains']!r}")
        elif "rr_line" in t:
            text = lines(NUM, RR)
            if t["contains"] not in text[t["rr_line"] - 1]: fail(e, f"RR:{t['rr_line']} lacks {t['contains']!r}")
            hl = max(i + 1 for i in range(t["rr_line"]) if text[i].startswith("## "))
            if hl != t["heading_line"]: fail(e, f"RR:{t['rr_line']} is under {hl}, not {t['heading_line']}")
        elif "repo_path" in t:
            text = lines(t["rev"], t["repo_path"])
            if text is None or t["contains"] not in text[t["line"] - 1]: fail(e, f"{t['repo_path']}:{t['line']} at {t['rev'][:10]} lacks {t['contains']!r}")
            mb = git("rev-parse", f"{a.main}:{t['repo_path']}", ok=(0, 128)).stdout.decode().strip()
            hb = git("rev-parse", f"{t['rev']}:{t['repo_path']}").stdout.decode().strip()
            print(f"  {e['token']}: {t['repo_path'].replace(P, '')}:{t['line']} blob at source {hb[:12]}, at main {mb[:12] or '-'}")
        elif "commit" in t:
            c = t["commit"]
            subj = git("log", "-1", "--format=%s", c, ok=(0, 128)).stdout.decode().strip()
            if subj != t["subject"]: fail(e, f"commit {c[:10]} subject {subj!r}")
            remotes = git("branch", "-r", "--contains", c).stdout.decode().split()
            in_num = git("merge-base", "--is-ancestor", c, NUM, ok=(0, 1)).returncode == 0
            in_main = git("merge-base", "--is-ancestor", c, a.main, ok=(0, 1)).returncode == 0
            print(f"  {e['token'][:20]}…: remote branches {remotes}; ancestor of NUM {in_num}; of main {in_main}")
            want = t["reachable_from"].split()[0]
            if want not in remotes or in_num or in_main: fail(e, "reachability differs from the index")
        else:
            fail(e, f"unknown target {t}")
for f in fails: print("FAILED", f)
print(f"named_references: {len(idx['named_references'])} entries, {checks} checks, {len(fails)} failed")
print("RESULT", "FAIL" if fails else "PASS")
sys.exit(1 if fails else 0)
