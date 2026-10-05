"""I65 U4 G7: carry the TEXT chain's line-keyed rules from the G6 text basis (1e323058f3) to the
integrated tree (stdlib only; Git reads only, GIT_OPTIONAL_LOCKS=0).

Every production file whose text changed between OLD and NEW is listed (CHANGED; the run fails if
`git diff --name-only` names another non-test .rs file under core/). For each, `git diff -U0`
gives the hunks; an unchanged OLD line maps to its NEW line by the cumulative offset. A key whose
line lies inside a changed hunk is NOT mapped: it is reported, and the script exits non-zero, so
no rule is silently re-pointed. A `file:line:fn` key must still define `fn` at the new line; a
`file:line` site key must carry the same text at the new line. Keys in descriptive fields (why,
evidence, about, reason) are citations and are left as written.
Usage: python3 g7_linemap.py <repo root> <OLD> <NEW> <out dir> <rules json> [<rules json> ...]
"""
import json, os, re, subprocess, sys
repo, old, new, outdir = sys.argv[1:5]
srcs = sys.argv[5:]
P = "projects/chirality-piping/"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def git(*a):
    return subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True, env=env, check=True).stdout
changed_all = [f[len(P):] for f in git("diff", "--name-only", old, new, "--", P + "core").split("\n") if f.endswith(".rs")]
CHANGED = [f for f in changed_all if "/tests/" not in f and not f.endswith("_tests.rs")]
maps, oldtext, newtext = {}, {}, {}
for f in CHANGED:
    hunks = []
    for line in git("diff", "-U0", old, new, "--", P + f).split("\n"):
        m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
        if m:
            hunks.append((int(m.group(1)), int(m.group(2) or 1), int(m.group(3)), int(m.group(4) or 1)))
    maps[f] = hunks
    try:
        oldtext[f] = git("show", f"{old}:{P}{f}").split("\n")
    except subprocess.CalledProcessError:
        oldtext[f] = []                                     # a file new since OLD: no OLD key can name it
    newtext[f] = git("show", f"{new}:{P}{f}").split("\n")
def remap(f, ln):
    off = 0
    for o, oc, n, nc in maps[f]:
        start = o if oc > 0 else o + 1
        if oc > 0 and o <= ln < o + oc:
            return None
        if ln >= start:
            off += nc - oc
    return ln + off
BASES = sorted({os.path.basename(f) for f in CHANGED})
KEY = re.compile(r"((?:W/)|(?:core/[a-z_/]+/src/)|(?:[a-z_]+/src/))?((?:[a-z_]+/)*(?:" + "|".join(re.escape(b) for b in BASES) + r")):(\d+)(?::([A-Za-z_0-9]+))?")
TOKS = ("format!", "write!", "writeln!", "diag(", ".to_string", ".clone", ".into", "String::from", ".to_owned", "Diagnostic", ".push_str", ".join", ".replace", "ok_or", "map_err", ".collect")
report, unmapped = [], []
def candidates(prefix, rel, W):
    if prefix == "W/":
        full = W + "/" + rel
        return [full[len("core/"):] if False else full] if full in maps or ("core/" + full) in maps else []
    if prefix and prefix.startswith("core/"):
        f = (prefix + rel)
        return [f] if f in maps else []
    tail = (prefix or "") + rel
    return [f for f in CHANGED if f.endswith("/" + tail) or f == tail]
def fix(s, W, ctx):
    def sub(m):
        whole, prefix, rel, ln, name = m.group(0), m.group(1), m.group(2), int(m.group(3)), m.group(4)
        c = candidates(prefix, rel, W)
        if prefix == "W/":
            c = [x for x in [W + "/" + rel] if x in maps]
        hits = []
        for f in c:
            ol = oldtext[f][ln - 1] if ln <= len(oldtext[f]) else ""
            if name:
                if re.search(r"\bfn\s+" + re.escape(name) + r"\b", ol):
                    hits.append(f)
            else:
                hits.append(f)
        if not hits:
            return whole                                   # names an unchanged file (another crate's same basename)
        if len(hits) > 1:
            unmapped.append([ctx, whole, "ambiguous: " + ", ".join(hits)]); return whole
        f = hits[0]
        nl = remap(f, ln)
        if nl is None:
            unmapped.append([ctx, whole, f"line {ln} of {f} lies in a changed hunk"]); return whole
        nline = newtext[f][nl - 1]
        if name and not re.search(r"\bfn\s+" + re.escape(name) + r"\b", nline):
            unmapped.append([ctx, whole, f"new line {nl} does not define fn {name}"]); return whole
        if not name and oldtext[f][ln - 1].strip() != nline.strip():
            unmapped.append([ctx, whole, f"new line {nl} differs"]); return whole
        out = whole.replace(f":{ln}", f":{nl}", 1)
        if out != whole:
            report.append([ctx, whole, out])
        return out
    return KEY.sub(sub, s)
DESCR = {"why", "evidence", "about", "reason", "note", "source"}
def walk(v, W, ctx, descr=False):
    if isinstance(v, str):
        return v if descr else fix(v, W, ctx)
    if isinstance(v, list):
        return [walk(x, W, ctx) for x in v]
    if isinstance(v, dict):
        return {fix(k, W, ctx + "/key"): walk(x, W, ctx + "/" + k[:40], k in DESCR) for k, x in v.items()}
    return v
os.makedirs(outdir, exist_ok=True)
for src in srcs:
    if src.endswith(".py"):
        # a script's hard-coded fn keys (sens.py's TB_FN_ZERO / branch keys): only its key lines
        lines = open(src).read().split("\n")
        lines = [fix(l, "", os.path.basename(src)) if re.match(r'\s*(ZW|ENVZERO)\s*=|.*TB_FN_ZERO"\]\s*=\s*"', l) else l for l in lines]
        open(os.path.join(outdir, os.path.basename(src)), "w").write("\n".join(lines))
        continue
    data = json.load(open(src))
    W = data.get("W", "") if isinstance(data, dict) else ""
    out = walk(data, W, os.path.basename(src))
    json.dump(out, open(os.path.join(outdir, os.path.basename(src)), "w"), indent=1)
summary = {"old": old, "new": new, "changed_production_rs": CHANGED, "hunks": {f: len(h) for f, h in maps.items()},
           "remapped": len(report), "unmapped": unmapped}
json.dump({"summary": summary, "remapped": report}, open(os.path.join(outdir, "g7_linemap.out.json"), "w"), indent=1)
print(json.dumps(summary, indent=1))
sys.exit(1 if unmapped else 0)
