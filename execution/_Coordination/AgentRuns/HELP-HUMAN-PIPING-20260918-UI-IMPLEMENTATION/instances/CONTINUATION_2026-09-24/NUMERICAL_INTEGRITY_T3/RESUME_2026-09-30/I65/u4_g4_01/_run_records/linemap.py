"""I65 U4 G4: carry line-keyed rules between NUM revisions (G3 basis 5ae5fe4f0f -> 3260d7809e -> G4 basis b1f80234dc) (stdlib only;
Git reads only, GIT_OPTIONAL_LOCKS=0).

G3's rule files key functions as `<file>:<line>:<name>` at NUM 5ae5fe4f0f. U1's merge changed
four non-test source files (PP lib.rs, retained_product.rs, retained_receipt.rs and FK
final_case.rs). For each, `git diff -U0 OLD NEW` gives the hunks; an unchanged old line maps to
its new line by the cumulative offset. A key whose line lies inside a changed hunk is NOT
mapped: it is reported, and the run stops, so no rule is silently re-pointed. Every mapped key
is checked: the new line must still define `fn <name>`.
Usage: python3 linemap.py <numerics repo root> <OLD> <NEW> <in rules json> <out rules json>
"""
import json, os, re, subprocess, sys

repo, old, new, src, dst = sys.argv[1:6]
P = "projects/chirality-piping/"
CHANGED = ["core/product_physics/src/lib.rs", "core/product_physics/src/retained_product.rs",
           "core/product_physics/src/retained_receipt.rs", "core/product_physics/src/retained_memory.rs",
           "core/product_physics/src/retained_wire.rs",
           "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs"]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")

def git(*a):
    return subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True, env=env, check=True).stdout

maps = {}
for f in CHANGED:
    hunks = []
    for line in git("diff", "-U0", old, new, "--", P + f).split("\n"):
        m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
        if m:
            o, oc = int(m.group(1)), int(m.group(2) or 1)
            n, nc = int(m.group(3)), int(m.group(4) or 1)
            hunks.append((o, oc, n, nc))
    maps[f] = hunks
newtext = {f: git("show", f"{new}:{P}{f}").split("\n") for f in CHANGED}
oldtext = {f: git("show", f"{old}:{P}{f}").split("\n") for f in CHANGED}

def remap(f, ln):
    off = 0
    for o, oc, n, nc in maps[f]:
        start = o if oc > 0 else o + 1          # -o,0 means insertion after line o
        if oc > 0 and o <= ln < o + oc:
            return None                          # the line itself changed
        if ln >= start:
            off += nc - oc
    return ln + off

def short_to_full(short):
    hits = [f for f in CHANGED if f.endswith("/" + short)]
    return hits[0] if len(hits) == 1 else None

KEY = re.compile(r"(?:(W/)|(?:core/[a-z_/]+/src/))?([a-z_/]*?(?:lib|retained_product|retained_receipt|retained_memory|retained_wire|final_case)\.rs):(\d+):([A-Za-z_0-9]+)")
report = []
def fix(s, W):
    def sub(m):
        whole = m.group(0)
        rel = m.group(2)
        full = None
        if m.group(1):
            full = (W + "/" + rel)
        elif whole.startswith("core/"):
            full = whole.rsplit(":", 2)[0]
        else:
            full = short_to_full(rel)
        if full is None or full not in maps:
            return whole
        if rel == "lib.rs" and not (whole.startswith("core/product_physics") or (not whole.startswith("core/") and not m.group(1))):
            return whole
        ln = int(m.group(3))
        name = m.group(4)
        # a short key (`lib.rs:L:name`) names whichever crate's file defines fn name at L at OLD;
        # only a definition in a changed file is re-pointed
        if not re.search(r"\bfn\s+" + re.escape(name) + r"\b", oldtext[full][ln - 1] if ln <= len(oldtext[full]) else ""):
            return whole
        nl = remap(full, ln)
        if nl is None or not re.search(r"\bfn\s+" + re.escape(name) + r"\b", newtext[full][nl - 1]):
            raise SystemExit(f"UNMAPPABLE {whole}: new line {nl} does not define fn {name}")
        out = whole.replace(f":{ln}:{name}", f":{nl}:{name}")
        if out != whole:
            report.append([whole, out])
        return out
    return KEY.sub(sub, s)

data = json.load(open(src))
W = data.get("W", "")
def walk(v):
    if isinstance(v, str):
        return fix(v, W)
    if isinstance(v, list):
        return [walk(x) for x in v]
    if isinstance(v, dict):
        return {k: walk(x) for k, x in v.items()}
    return v
out = walk(data)
json.dump(out, open(dst, "w"), indent=1)
print(json.dumps({"in": os.path.basename(src), "out": os.path.basename(dst), "old": old, "new": new,
                  "remapped": report, "hunks": {f: len(h) for f, h in maps.items()}}, indent=1))

# ---- site keys (`<file>:<line>` without a fn name, as in text_args.json) ----
# A key `lib.rs:N` is re-pointed only when the OLD line N of the changed file carries a text
# token (format!/write!/diag(/to_string/clone/into/...) and the NEW line has the same text;
# otherwise it names an unchanged file of another crate and is kept.
if os.environ.get("LINEMAP_SITES"):
    SITE = re.compile(r"^((?:[a-z_]+/src/)?)((?:[a-z_/]*?)(?:lib|retained_product|retained_receipt|retained_memory|retained_wire|final_case)\.rs):(\d+)$")
    TOKS = ("format!", "write!", "writeln!", "diag(", ".to_string", ".clone", ".into", "String::from", ".to_owned", "Diagnostic")
    site_report = []
    def fix_site(k):
        m = SITE.match(k)
        if not m:
            return k
        prefix, rel, ln = m.group(1), m.group(2), int(m.group(3))
        cands = [f for f in CHANGED if f.endswith("/" + rel) and (not prefix or f.split("/src/")[0].endswith(prefix.split("/src/")[0]))]
        if len(cands) != 1:
            return k
        full = cands[0]
        old_line = oldtext[full][ln - 1] if ln <= len(oldtext[full]) else ""
        if not any(t in old_line for t in TOKS):
            return k
        nl = remap(full, ln)
        if nl is None or newtext[full][nl - 1].strip() != old_line.strip():
            raise SystemExit(f"UNMAPPABLE site {k}")
        out = f"{prefix}{rel}:{nl}"
        if out != k:
            site_report.append([k, out])
        return out
    def walk_keys(v):
        if isinstance(v, dict):
            return {fix_site(k): walk_keys(x) for k, x in v.items()}
        if isinstance(v, list):
            return [walk_keys(x) for x in v]
        return v
    out2 = walk_keys(out)
    json.dump(out2, open(dst, "w"), indent=1)
    print(json.dumps({"site_keys_remapped": site_report}, indent=1))
