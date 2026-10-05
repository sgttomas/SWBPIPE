"""I65 U4 G7 Pass B (RV89 G7 S-1(a)): the production-delta inventory between two revisions, every hunk
classified explicitly, fail-closed (stdlib only; Git reads only, GIT_OPTIONAL_LOCKS=0).

Classes (each with its reason):
  test        a test file, a #[cfg(test)] module file, or a hunk wholly inside #[cfg(test)] items;
  cfg-test-stmt  `#[cfg(test)] <statement>;` added to production lines (compiled into the lib test
              binary only): needs a reviewed entry with evidence;
  generated   inside the GENERATED PROFILE markers (checked by the FORMS gate);
  no-code     comments and blank lines only;
  unreachable the enclosing fns are not on the lexical D1 graph, or are reached only through
              edge_zero rules (not live);
  live        an enclosing fn is live on D1: needs a reviewed pricing entry;
  qualification-test  a hunk in the qualification's own evidence (QUAL_TESTS: the law, witness and
              challenge test files; RV89 ADDENDUM_01 N-5): needs a reviewed entry, else exit 6;
  item        outside any fn (const, static, type, use, impl header): needs a reviewed entry;
  not-d1      a file outside the D1 crates' sources and not embedded by them (apps, Python, other
              crates, documentation);
  data        a data file: embedded (include_str!/include_bytes!) by D1 production code, by a fn whose
              reachability decides as above, or not embedded (not-d1). A reviewed input is gated by
              the identity check.
A hunk needing a reviewed entry is looked up by its fingerprint (sha256 of the file path and its
removed/added lines, whitespace-stripped) in the reviewed table; a missing entry makes the verdict
`unreviewed` and the script exits 5; an unreviewed qualification-test hunk alone exits 6 (a delta to read).
Usage: python3 delta_inventory2.py <repo> <OLD> <NEW> <tree projects/chirality-piping> <crate_dirs.txt>
       <edges json> <loop_bounds json> <reviewed json> <out json>"""
import hashlib, json, os, re, subprocess, sys
repo, old, new, W, crates_p, edges_p, lb_p, reviewed_p, out_p = sys.argv[1:10]
P = "projects/chirality-piping/"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
git = lambda *a: subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True, env=env, check=True).stdout
CRATES = open(crates_p).read().split()
REVIEWED = {e["fingerprint"]: e for e in json.load(open(reviewed_p)).get("entries", [])}
REVIEWED_INPUTS = ["Cargo.lock", "schemas/physics_source_recovery.schema.json", "schemas/retained_precision_mp_v2.schema.json",
                   "fixtures/results/retained_precision_prepared_ordinary_v1.json", "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
                   "fixtures/results/semantic_contract_v0_2.json", "fixtures/results/semantic_contract_v0_3_precision_1.json",
                   "fixtures/results/semantic_contract_v0_3_physics_1.json", "fixtures/results/semantic_contract_v0_3_load_reference_1.json",
                   "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json", "fixtures/results/semantic_contract_v0_3_preview_physics_1.json",
                   "fixtures/results/semantic_contract_v0_3_physics_source_1.json", "fixtures/results/semantic_contract_v0_3_source_blocks_1.json",
                   "schemas/source_block_recovery.schema.json"]
E = json.load(open(edges_p))["edges"]
EZ = {(e["caller"], e["callee"]) for e in json.load(open(lb_p)).get("edge_zero", [])}
sh = lambda k: k.split("/src/")[-1]
root = [k for k in E if k.endswith(":run_linear_static_preview_value_with_retained_direct")]
def bfs(skip):
    s, t = set(root), list(root)
    while t:
        v = t.pop()
        for w in E.get(v, []):
            if w not in s and not (skip and (sh(v), sh(w)) in EZ): s.add(w); t.append(w)
    return s
REACH, LIVE = bfs(False), bfs(True)
FN = re.compile(r"^\s*(pub(\([^)]*\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?(extern\s+\"[^\"]*\"\s+)?fn\s+(\w+)")
CFGT = re.compile(r"\s*#\[cfg\((test|any\(test)")
def code_of(l):
    l = re.sub(r'"(\\.|[^"\\])*"', '""', l)
    l = re.sub(r"'(\\.|[^'\\])'", "''", l)
    return l.split("//")[0]
def spans(L):
    """fn spans (start, end, name) and #[cfg(test)] item spans, by brace matching on code text."""
    fns, tests = [], []
    for i, l in enumerate(L):
        m = FN.match(l); c = CFGT.match(l)
        if not (m or c): continue
        depth, started, j = 0, False, i
        while j < len(L):
            cl = code_of(L[j])
            depth += cl.count("{") - cl.count("}")
            if "{" in cl: started = True
            if started and depth <= 0: break
            if not started and cl.rstrip().endswith(";") and (j > i or not c): break
            j += 1
        (fns.append((i + 1, j + 1, m.group(7))) if m else tests.append((i + 1, j + 1)))
    return fns, tests
def in_any(spans_, ln):
    return any(a <= ln <= b for a, b in spans_)
def test_module_file(rel):
    """a child module file whose every `mod <stem>;` declaration in its crate lies in #[cfg(test)]"""
    stem = os.path.splitext(os.path.basename(rel))[0]
    if stem in ("lib", "mod", "main"): return False
    src = rel.split("/src/")[0] + "/src"
    decls = []
    for d, _, fs in os.walk(os.path.join(W, src)):
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(d, f); L = open(p, encoding="utf-8").read().split("\n")
            hits = [i + 1 for i, l in enumerate(L) if re.match(r"\s*(pub(\([^)]*\))?\s+)?mod\s+" + re.escape(stem) + r"\s*;", code_of(l))
                    or re.search(r"\}\s*mod\s+" + re.escape(stem) + r"\s*;", code_of(l))]
            if hits:
                _, tests = spans(L)
                decls += [in_any(tests, h) for h in hits]
    return bool(decls) and all(decls)
def fp(rel, minus, plus):
    h = hashlib.sha256((rel + "\n" + "\n".join("-" + x.strip() for x in minus) + "\n" + "\n".join("+" + x.strip() for x in plus)).encode())
    return h.hexdigest()
def nocode(lines):
    return all(not code_of(x).strip() for x in lines)
STMT = re.compile(r"#\[cfg\(test\)\]\s*[^;{}]*;")
rows, need, need_q = [], [], []
QUAL_TESTS = {"core/product_physics/src/retained_memory_law_tests.rs", "core/product_physics/src/retained_memory_witness_tests.rs",
              "core/product_physics/tests/retained_memory_challenge.rs"}
def hunks_of(f):
    out, cur = [], None
    for line in git("diff", "-U0", old, new, "--", f).split("\n"):
        m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
        if m:
            cur = {"o": int(m.group(1)), "oc": int(m.group(2) or 1), "n": int(m.group(3)), "nc": int(m.group(4) or 1), "minus": [], "plus": []}
            out.append(cur); continue
        if cur is not None and line.startswith("-") and not line.startswith("---"): cur["minus"].append(line[1:])
        elif cur is not None and line.startswith("+") and not line.startswith("+++"): cur["plus"].append(line[1:])
    return out
names = [f for f in git("diff", "--name-only", old, new, "--", P).split("\n") if f and "/execution/" not in f]
for f in names:
    rel = f[len(P):]
    in_crate = any(rel.startswith(c + "/") for c in CRATES)
    if not rel.endswith(".rs"):
        if rel in REVIEWED_INPUTS:
            rows.append({"file": rel, "class": "data", "reason": "a reviewed input: the identity gate (law tests) refuses a changed one"}); continue
        base = os.path.basename(rel); embedders = []
        for c in CRATES:
            for d, _, fs in os.walk(os.path.join(W, c)):
                if "/tests" in d: continue
                for ff in fs:
                    if not ff.endswith(".rs"): continue
                    p = os.path.join(d, ff); L = open(p, encoding="utf-8").read().split("\n"); fns, tests = spans(L)
                    for i, l in enumerate(L):
                        if base in l and re.search(r"include_(str|bytes)!", " ".join(L[max(0, i - 2):i + 1])) and not in_any(tests, i + 1):
                            enc = [(a, n) for a, b, n in fns if a <= i + 1 <= b]
                            key = f"{os.path.relpath(p, W)}:{enc[-1][0]}:{enc[-1][1]}" if enc else None
                            embedders.append({"at": f"{os.path.relpath(p, W)}:{i + 1}", "fn": key and sh(key),
                                              "live": bool(key) and key in LIVE})
        if not embedders:
            rows.append({"file": rel, "class": "not-d1", "reason": "not embedded by D1 production code" if (rel.startswith("fixtures/") or rel.startswith("schemas/")) else "not compiled into the D1 crates (apps, Python, documentation, other crates)"})
        elif any(e["live"] or e["fn"] is None for e in embedders):
            key = fp(rel, [], [git("rev-parse", f"{new}:{f}").strip()])
            r = {"file": rel, "class": "data-live", "embedders": embedders, "fingerprint": key}
            r["reviewed"] = REVIEWED.get(key); rows.append(r)
            if not r["reviewed"]: need.append(r)
        else:
            rows.append({"file": rel, "class": "unreachable", "reason": "data embedded only by fns not live on D1", "embedders": embedders})
        continue
    if rel in QUAL_TESTS:
        for h in hunks_of(f):
            r = {"file": rel, "new_lines": f"{h['n']}-{h['n'] + h['nc'] - 1}" if h["nc"] else f"after {h['n']}", "removed": len(h["minus"]), "added": len(h["plus"]),
                 "fingerprint": fp(rel, h["minus"], h["plus"]), "class": "qualification-test",
                 "reason": "the qualification's own evidence (PINNED_RECORD, the registered law tests, the witnesses' stacks and outcomes, the challenge's bounds): a weakened assertion can keep its outcome ok"}
            r["reviewed"] = REVIEWED.get(r["fingerprint"])
            if not r["reviewed"]: need_q.append(r)
            rows.append(r)
        continue
    if not in_crate:
        rows.append({"file": rel, "class": "not-d1", "reason": "not in a D1 crate's sources (crate_dirs.txt)"}); continue
    if "/tests/" in rel or rel.endswith("_tests.rs") or rel.endswith("/tests.rs"):
        rows.append({"file": rel, "class": "test", "reason": "a test file"}); continue
    if test_module_file(rel):
        rows.append({"file": rel, "class": "test", "reason": "a module file declared only inside #[cfg(test)]"}); continue
    NL = open(os.path.join(W, rel), encoding="utf-8").read().split("\n")
    try:
        OL = git("show", f"{old}:{f}").split("\n")
    except subprocess.CalledProcessError:
        OL = []
    nf, nt = spans(NL); of, ot = spans(OL)
    gen = [i + 1 for i, l in enumerate(NL) if "GENERATED PROFILE" in l]
    diff = git("diff", "-U0", old, new, "--", f).split("\n")
    hunks, cur = [], None
    for line in diff:
        m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
        if m:
            cur = {"o": int(m.group(1)), "oc": int(m.group(2) or 1), "n": int(m.group(3)), "nc": int(m.group(4) or 1), "minus": [], "plus": []}
            hunks.append(cur); continue
        if cur is not None and line.startswith("-") and not line.startswith("---"): cur["minus"].append(line[1:])
        elif cur is not None and line.startswith("+") and not line.startswith("+++"): cur["plus"].append(line[1:])
    for h in hunks:
        nlines = list(range(h["n"], h["n"] + h["nc"])); olines = list(range(h["o"], h["o"] + h["oc"]))
        r = {"file": rel, "new_lines": f"{h['n']}-{h['n'] + h['nc'] - 1}" if h["nc"] else f"after {h['n']}", "removed": len(h["minus"]), "added": len(h["plus"]),
             "fingerprint": fp(rel, h["minus"], h["plus"])}
        if all(in_any(nt, x) for x in nlines if h["nc"]) and all(in_any(ot, x) for x in olines if h["oc"]):
            r.update({"class": "test", "reason": "wholly inside #[cfg(test)] items"})
        elif len(gen) >= 2 and all(gen[0] < x < gen[-1] for x in nlines) and h["nc"]:
            r.update({"class": "generated", "reason": "inside the GENERATED PROFILE markers: gated by FORMS (equal to regeneration from G7's profile tree)"})
        elif nocode(h["minus"]) and nocode(h["plus"]):
            r.update({"class": "no-code", "reason": "comments and blank lines only"})
        elif [code_of(STMT.sub("", x)).strip() for x in h["plus"] if code_of(STMT.sub("", x)).strip()] == [code_of(x).strip() for x in h["minus"] if code_of(x).strip()] and any(STMT.search(x) for x in h["plus"]):
            r.update({"class": "cfg-test-stmt", "reason": "only `#[cfg(test)] <statement>;` added to production lines: compiled into the lib test binary only",
                      "statements": [m.group(0) for x in h["plus"] for m in STMT.finditer(x)]})
        else:
            enc = sorted({(a, n) for a, b, n in nf for x in nlines if a <= x <= b} | {(a, n) for a, b, n in of for x in olines if a <= x <= b and h["oc"] and False})
            keys = [f"{rel}:{a}:{n}" for a, n in enc]
            if not keys:
                r.update({"class": "item", "reason": "outside any fn"})
            else:
                info = [{"fn": sh(k), "reached": k in REACH, "live": k in LIVE} for k in keys]
                r["fns"] = info
                if any(i["live"] for i in info):
                    r.update({"class": "live", "reason": "an enclosing fn is live on D1"})
                else:
                    r.update({"class": "unreachable", "reason": "enclosing fns not live on D1: " + ", ".join(f"{i['fn']} ({'reached only through edge_zero' if i['reached'] else 'not on the lexical D1 graph'})" for i in info)})
        if r["class"] in ("cfg-test-stmt", "live", "item"):
            r["reviewed"] = REVIEWED.get(r["fingerprint"])
            if not r["reviewed"]: need.append(r)
        rows.append(r)
out = {"old": old, "new": new, "files": len(names), "hunks_and_files": len(rows), "classes": {}, "unreviewed": need, "unreviewed_qualification_tests": need_q,
       "verdict": (f"STOP: {len(need)} hunk(s) need a reviewed entry" if need else
                   f"DELTAS TO READ: {len(need_q)} unreviewed hunk(s) in the qualification's own test files" if need_q else
                   "PASS: every hunk classified; every live, item, cfg(test)-statement or qualification-test hunk has a reviewed entry"), "rows": rows}
for r in rows: out["classes"][r["class"]] = out["classes"].get(r["class"], 0) + 1
json.dump(out, open(out_p, "w"), indent=1)
print(json.dumps({k: out[k] for k in ("files", "hunks_and_files", "classes", "verdict")}))
for r in need + need_q: print("UNREVIEWED", r["file"], r.get("new_lines", ""), r["class"], r["fingerprint"][:16])
sys.exit(5 if need else 6 if need_q else 0)
