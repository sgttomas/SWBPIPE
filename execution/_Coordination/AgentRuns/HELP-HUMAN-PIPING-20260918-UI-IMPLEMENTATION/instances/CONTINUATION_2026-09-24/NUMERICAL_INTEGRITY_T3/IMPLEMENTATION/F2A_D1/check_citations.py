#!/usr/bin/env python3
"""U9 citation check: every record citation the PR adds to maintained source resolves through the index.

Read-only, standard library only. The Git reads are `diff`, `cat-file` and `rev-parse`, run with
GIT_OPTIONAL_LOCKS=0. The script writes only --out.

Usage:
  python3 check_citations.py --repo <checkout> --base <main SHA> --head <PR head | WORKTREE> \
      [--index citations.json] [--package <local package dir>] [--out resolved.md] [--list]

The source set S is `git diff --name-only <base> <head>`, minus execution paths. The script scans
every added line of S's diff for these citation classes:
  record_path      R/I<n>[/...]                        e.g. R/I65/u4_g6_01/QUALIFICATION.md
  review_path      REVIEW_RV<n>[/...]                  e.g. REVIEW_RV77/coverage_producer_01/tests/rv77_pp.rs
  run              RV<n> <run>_<nn> | I<n> <run>_<nn>  e.g. RV92 u6f_01
  generator        <...>/_run_records/<file>           e.g. part2/_run_records/g5_profile.py
  generator_input  profile_tree.json
  rr_line          RR:<line>[–<line>]                  ROOT_RULINGS_V1.md line numbers
  rr_title         RR "<title>" | ROOT_RULINGS_V1 "<title>" (also JSON-escaped)

Each occurrence must match one index entry, by class and token. Each entry used is then verified
at the index's pinned NUM commit:
  - num_paths exist (relative to the index's records_root);
  - copies exist in --package with the recorded sha256;
  - an rr_line lies in the recorded section, whose heading text matches;
  - an rr_title starts the recorded heading.

Design documents (class "document"). The index's `documents` table maps each cited name
(C1, C2, C3, F1, S06, D1, D2, ROUTING, COMP, I57, I51 COMPOSITION, G4 NOTES and the U4 notes
DOMAIN.md, BUILD.md, API.md, API_G4.md, STACK_PLAN, STACK_INVENTORY.md, G2_AMENDMENTS,
QUALIFICATION.md, TRANSFER_COMPLETION.md, ADDENDUM_L128.md, NOTES_G4.md, COMPOSITION_G4.md) to one
location:
  - main, when the base holds the same blob at the same path;
  - else a copy in this package;
  - else a commit-pinned URL on NUM.
A name with several candidate files is resolved only by the context rule recorded for it (the
citing file's path prefix). Otherwise it is ambiguous, listed with its candidates. Bare short names
(C1, D1, ROUTING, ...) count only with an anchor; names ending in .md count with or without one.
Anchors are verified in the pinned version: each line number of `:n`, `:a-b` and `:a, b` must lie
within the document, and each section of `§x.y` must head a Markdown section.

Code line citations (class "code_line"). A bare FILE:line citation of code is forbidden: PP:2896,
FC:358, FK/adaptive.rs:4349, verify.rs:880 and the like. Code moves, and a stale line misleads; name
the symbol instead. The only exceptions are listed in the index's `code_anchors`. Each pins its
revision and records the anchor text of every cited line; the check verifies that text at that
revision. They are the 11 generated citations in retained_memory.rs's GENERATED PROFILE block (owned
by g5_profile.py) and the 3 in the hash-pinned JSON corpus, which is data, not comments.

Counts reported: resolved, ambiguous, unresolved. Exit 1 only on unresolved citations or failed
verifications. An ambiguous citation is listed for ROOT's ruling and does not fail.
"""
import argparse, hashlib, json, os, re, subprocess, sys

P = "projects/chirality-piping/"
ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True); ap.add_argument("--base", required=True); ap.add_argument("--head", required=True)
here = os.path.dirname(os.path.abspath(__file__))
ap.add_argument("--index", default=os.path.join(here, "citations.json"))
ap.add_argument("--package", default=here)
ap.add_argument("--out"); ap.add_argument("--list", action="store_true")
ap.add_argument("--suggest", help="write index entries for unresolved tokens to this JSON file (auto-resolved where unambiguous, else TODO)")
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def git(*args, ok=(0,)):
    r = subprocess.run(["git", "-C", a.repo, *args], capture_output=True, env=env)
    if r.returncode not in ok: sys.exit(f"git {' '.join(args)}: {r.stderr.decode(errors='replace')}")
    return r
idx = json.load(open(a.index, encoding="utf-8"))
NUM, R, RR = idx["num_commit"], idx["records_root"], idx["rr_path"]

CLASSES = [
    ("record_path", re.compile(r"(?<![\w/])R/I\d+(?:/[\w.\-]+)*")),
    ("review_path", re.compile(r"(?<![\w/])REVIEW_RV\d+(?:/[\w.\-]+)*")),
    ("run", re.compile(r"\b(?:RV|I)\d{2,3} [a-z][a-z0-9_]*_\d{2}[a-z]?\b")),
    ("generator", re.compile(r"(?:[\w.\-]+/)*_run_records/[\w.\-/]+")),
    ("generator_input", re.compile(r"(?<![\w/.])profile_tree\.json")),
    ("rr_line", re.compile(r"\bRR:\d+(?:[–-]\d+)?")),
    ("rr_title", re.compile(r'\bRR "([^"]+)"|ROOT_RULINGS_V1 \\"(.+?)\\"|ROOT_RULINGS_V1 "([^"\\]+)"')),
]
DOCS = idx.get("documents", {})
ALIAS = {al: name for name, d in DOCS.items() for al in d["aliases"]}
ANCH = r"(?::\d+(?:\s*[–\-/]\s*\d+)*(?:,\s*\d+(?:\s*[–\-]\s*\d+)*)*|\s§\s?\d[\d.]*(?:\s*[–\-/]\s*§?\s?\d[\d.]*)*)"
DOC_RX = re.compile(r"(?<![\w/.])(" + "|".join(re.escape(x) for x in sorted(ALIAS, key=len, reverse=True)) + r")(" + ANCH + r")?") if ALIAS else None
NUMS = r"\d+(?:\s*[–-]\s*\d+)?(?:(?:,\s*|/)\d+(?:\s*[–-]\s*\d+)?(?![\w.]))*"
CODE_LINE = re.compile(r"(?<![\w/])(?:(?:PP|FC|FK|SR)(?:/(?:[\w.]+/)*[\w.]+\.(?:rs|py|ts))?|(?:[\w-]+/)*[\w-]+\.(?:rs|py|ts|tsx)):" + NUMS)
ANCHORS = {(x["citing_file"], x["token"]): x for x in idx.get("code_anchors", [])}
def norm_title(t): return t.rstrip("…").rstrip(".").strip()

def tokens(line):
    for cls, rx in CLASSES:
        for m in rx.finditer(line):
            if cls == "rr_title":
                yield cls, norm_title(next(g for g in m.groups() if g))
            else:
                yield cls, m.group(0).rstrip(".")

REVS = [a.base] if a.head == "WORKTREE" else [a.base, a.head]   # WORKTREE: compare base with the checkout's working tree
S = [n for n in git("diff", "--name-only", *REVS, "--", ".", ":!" + P + "execution", ":!execution").stdout.decode().split("\n") if n]
diff = git("diff", "-U0", *REVS, "--", *S).stdout.decode("utf-8", errors="replace") if S else ""
occ, report_only, f, ln, docs, code = [], 0, None, 0, [], []
for line in diff.split("\n"):
    if line.startswith("+++ "): f = line[6:] if line.startswith("+++ b/") else None; continue
    m = re.match(r"@@ -\S+ \+(\d+)(?:,\d+)? @@", line)
    if m: ln = int(m.group(1)); continue
    if line.startswith("+") and f:
        for cls, tok in tokens(line[1:]): occ.append((f, ln, cls, tok))
        if DOC_RX:
            for m in DOC_RX.finditer(line[1:]):
                name, anchor = ALIAS[m.group(1)], (m.group(2) or "").strip().rstrip(".,;")
                if anchor or m.group(1).endswith(".md"): docs.append((f, ln, name, m.group(1), anchor))
        for m in CODE_LINE.finditer(line[1:]): code.append((f, ln, m.group(0)))
        ln += 1

entries = {(e["class"], e["token"]): e for e in idx["citations"]}
unresolved = [o for o in occ if (o[2], o[3]) not in entries]
used = {(o[2], o[3]) for o in occ} & set(entries)
rr_lines = None
def rr_text():
    global rr_lines
    if rr_lines is None: rr_lines = git("cat-file", "blob", f"{NUM}:{RR}").stdout.decode("utf-8").split("\n")
    return rr_lines
def heading_for(n):
    hs = [(i + 1, l[3:]) for i, l in enumerate(rr_text()[:n]) if l.startswith("## ")]
    return hs[-1] if hs else (None, None)
def url(path):
    t = git("cat-file", "-t", f"{NUM}:{path}", ok=(0, 128)).stdout.decode().strip()
    return (f"{idx['github']}/{'tree' if t == 'tree' else 'blob'}/{NUM}/{path}", t)
failed, rows = [], []
for key in sorted(used):
    e = entries[key]; out = []
    for p in e.get("num_paths", []):
        u, t = url(f"{R}/{p}")
        if t not in ("blob", "tree"): failed.append((key, f"missing at NUM: {p}"))
        out.append(u)
    if e.get("copy"):
        fp = os.path.join(a.package, e["copy"])
        if not os.path.isfile(fp) or hashlib.sha256(open(fp, "rb").read()).hexdigest() != e["sha256"]:
            failed.append((key, f"copy missing or sha256 mismatch: {e['copy']}"))
        out.append(f"package: {e['copy']}")
    if key[0] in ("rr_line", "rr_title"):
        hl = e["heading_line"]; text = rr_text()
        if not text[hl - 1].startswith("## ") or text[hl - 1][3:] != e["heading"]:
            failed.append((key, f"heading at RR:{hl} differs"))
        if key[0] == "rr_line":
            lo = int(re.findall(r"\d+", key[1])[0])
            if heading_for(lo)[0] != hl: failed.append((key, f"RR line {lo} is not in section {hl}"))
        elif not e["heading"].startswith(key[1]):
            failed.append((key, "title is not the heading's prefix"))
        out.append(f"{idx['github']}/blob/{NUM}/{RR}#L{hl} — \"{e['heading']}\"")
    rows.append((key, out))
# design documents: location, context rules, anchors
doc_text, doc_res = {}, {"resolved": [], "ambiguous": [], "unresolved": []}
def lines_of(path):
    if path not in doc_text:
        r = git("cat-file", "blob", f"{NUM}:{path}", ok=(0, 128))
        doc_text[path] = r.stdout.decode("utf-8", errors="replace").split("\n") if r.returncode == 0 else None
    return doc_text[path]
def where(path, d):
    nb = git("rev-parse", f"{NUM}:{path}", ok=(0, 128)).stdout.decode().strip()
    bb = git("rev-parse", f"{a.base}:{path}", ok=(0, 128)).stdout.decode().strip()
    if nb and nb == bb: return f"main: {path}"
    if d.get("copy"): return f"package: {d['copy']} (and {idx['github']}/blob/{NUM}/{path})"
    return f"{idx['github']}/blob/{NUM}/{path}"
def anchor_ok(text, anchor):
    if anchor.startswith(":"):
        return all(1 <= int(n) <= len(text) for n in re.findall(r"\d+", anchor))
    for sec in re.findall(r"\d+(?:\.\d+)*", anchor):
        if not any(re.match(r"^#+\s+(?:§\s*)?" + re.escape(sec) + r"(?:[.\s):]|$)", l) for l in text): return False
    return True
def label(alias, anchor): return alias + (anchor if anchor.startswith(":") or not anchor else " " + anchor)
for f_, l_, name, alias, anchor in docs:
    d = DOCS[name]; path = d.get("path")
    if d.get("candidates"):
        rule = next((r for r in d.get("rules", []) if f_.startswith(r["file_prefix"])), None)
        if not rule:
            doc_res["ambiguous"].append((f_, l_, label(alias, anchor), d["candidates"])); continue
        path = rule["resolve"]
    text = lines_of(path)
    if text is None or (anchor and not anchor_ok(text, anchor)):
        doc_res["unresolved"].append((f_, l_, label(alias, anchor), path)); continue
    doc_res["resolved"].append((f_, l_, label(alias, anchor), where(path, d)))
code_res = {"pinned": [], "unresolved": []}
for f_, l_, tok in code:
    an = ANCHORS.get((f_, tok))
    if not an: code_res["unresolved"].append((f_, l_, tok, "bare code line citation: name the symbol")); continue
    body = git("cat-file", "blob", f"{an['rev']}:{an['file']}", ok=(0, 128)).stdout.decode("utf-8", errors="replace").split("\n")
    bad = [x["n"] for x in an["lines"] if not (0 < x["n"] <= len(body)) or body[x["n"] - 1] != x["text"]]
    (code_res["unresolved"] if bad else code_res["pinned"]).append((f_, l_, tok, f"anchor text differs at {an['rev'][:10]} lines {bad}" if bad else an["rev"][:10]))
for o in unresolved: print(f"UNRESOLVED {o[0]}:{o[1]} [{o[2]}] {o[3]}")
for o in code_res["unresolved"]: print(f"UNRESOLVED {o[0]}:{o[1]} [code_line] {o[2]} ({o[3]})")
for o in doc_res["unresolved"]: print(f"UNRESOLVED {o[0]}:{o[1]} [document] {o[2]} (anchor not found in {o[3]})")
for o in doc_res["ambiguous"]: print(f"AMBIGUOUS {o[0]}:{o[1]} [document] {o[2]} candidates {o[3]}")
if a.suggest:
    sug, seen = [], set()
    heads = [(i + 1, l[3:]) for i, l in enumerate(rr_text()) if l.startswith("## ")]
    def exists(p): return git("cat-file", "-t", f"{NUM}:{R}/{p}", ok=(0, 128)).stdout.decode().strip() in ("blob", "tree")
    for f_, l_, c, t in unresolved:
        if (c, t) in seen: continue
        seen.add((c, t)); e = {"class": c, "token": t, "first_seen": f"{f_}:{l_}"}
        cand = None
        if c == "record_path": cand = [t[2:]]
        elif c == "review_path": cand = [t]
        elif c == "run":
            who, run = t.split(); cand = [("REVIEW_" + who if who.startswith("RV") else who) + "/" + run]
        if cand and all(exists(p) for p in cand): e["num_paths"] = cand
        elif c == "rr_line":
            hl, h = heading_for(int(re.findall(r"\d+", t)[0])); e.update(heading_line=hl, heading=h)
        elif c == "rr_title" and len([h for h in heads if h[1].startswith(t)]) == 1:
            hl, h = next(h for h in heads if h[1].startswith(t)); e.update(heading_line=hl, heading=h)
        else: e["TODO"] = "resolve by hand"
        sug.append(e)
    json.dump(sug, open(a.suggest, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print(f"suggested {len(sug)} entries, {sum('TODO' in e for e in sug)} TODO -> {a.suggest}")
for k, why in failed: print(f"FAILED {k[0]} {k[1]!r}: {why}")
unused = sorted(set(entries) - used)
n_res = len(occ) - len(unresolved) + len(doc_res["resolved"]) + len(code_res["pinned"])
print(f"COUNTS resolved {n_res}; ambiguous {len(doc_res['ambiguous'])}; unresolved {len(unresolved) + len(doc_res['unresolved']) + len(code_res['unresolved'])}")
print(f"  records/RR: occurrences {len(occ)}, distinct {len({(o[2], o[3]) for o in occ})}, unresolved {len(unresolved)}; "
      f"documents: {len(docs)} citations ({len(doc_res['resolved'])} resolved, {len(doc_res['ambiguous'])} ambiguous, "
      f"{len(doc_res['unresolved'])} unresolved); verification failures {len(failed)}; unused index entries {len(unused)}; "
      f"code lines: {len(code)} ({len(code_res['pinned'])} pinned in code_anchors, {len(code_res['unresolved'])} unresolved)")
for k in unused: print(f"  unused: {k[0]} {k[1]!r}")
if a.list:
    for o in occ: print(f"  {o[0]}:{o[1]}\t{o[2]}\t{o[3]}")
    for o in doc_res["resolved"]: print(f"  {o[0]}:{o[1]}\tdocument\t{o[2]}\t{o[3]}")
if a.out:
    with open(a.out, "w", encoding="utf-8") as w:
        w.write(f"# Resolved citations (NUM {NUM})\n\n| Class | Token | Resolves to |\n|---|---|---|\n")
        for (c, t), out in rows: w.write(f"| {c} | `{t}` | {'<br>'.join(out)} |\n")
        seen = {}
        for f_, l_, tok, loc in doc_res["resolved"]: seen.setdefault((tok, loc), f"{f_}:{l_}")
        for (tok, loc), first in sorted(seen.items()): w.write(f"| document | `{tok}` | {loc} |\n")
        for f_, l_, tok, cands in doc_res["ambiguous"]: w.write(f"| document (AMBIGUOUS) | `{tok}` at {f_}:{l_} | {' / '.join(cands)} |\n")
bad = unresolved or failed or doc_res["unresolved"] or code_res["unresolved"]
print("RESULT", "FAIL" if bad else ("PASS (ambiguous citations listed for ROOT)" if doc_res["ambiguous"] else "PASS"))
sys.exit(1 if bad else 0)
