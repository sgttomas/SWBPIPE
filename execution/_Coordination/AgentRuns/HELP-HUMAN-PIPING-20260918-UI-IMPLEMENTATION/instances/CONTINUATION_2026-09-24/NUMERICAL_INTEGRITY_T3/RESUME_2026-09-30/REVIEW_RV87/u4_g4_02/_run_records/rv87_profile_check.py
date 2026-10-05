"""RV87 (u4_g4_02): check that the part-2 profile tree carries RV87's S-1, N-1(a)-(e) and N-2 terms,
by rebuilding those forms with RV87's own G4 model (REVIEW_RV87/u4_g4_01/_run_records/rv87_g4.py,
model part only) at the part-2 text atoms, and comparing them atom by atom with profile_tree.json.
Then check the generated Rust FORMS table transcribes profile_tree.json (retained_memory.rs at the
part-2 commit, read from a file given on the command line).
Usage: python3 rv87_profile_check.py <rv87_g4.py> <part2 _run_records> <retained_memory.rs copy>
Stdlib only; writes nothing but stdout."""
import json, os, re, sys, tempfile
G4PY, P2, RS = sys.argv[1], sys.argv[2], sys.argv[3]
tree = json.load(open(os.path.join(P2, "text_p2", "profile_tree.json")))
TA = tree["text_atoms"]; LP = tree["L"]["L_PUB"]
# The model part of rv87_g4.py reads text_closure / composite_text / the ASSUMED dicts from its RR
# argument; feed it the part-2 atoms through a mapping open() (no files are written).
src = open(G4PY).read()
model = src.split("# ----------------------------------------------------------------------------- inputs from the chain")[0]
import builtins, io
_open = builtins.open
fake = {"text_closure.caps.l128.json": json.dumps({"atoms": TA}),
        "composite_text.caps.l128.json": json.dumps({"classes": {"message_int": LP}})}
def myopen(p, *a, **k):
    base = os.path.basename(p)
    if base in fake:
        return io.StringIO(fake[base])
    if base in ("g3lib.py", "g4_caps.py"):
        return _open(os.path.join(P2, base), *a, **k)
    return _open(p, *a, **k)
builtins.open = myopen
SNAP = os.environ.get("SNAP", ".")
sys.argv = ["rv87_g4.py", "RR", SNAP, "128"]
g = {"__name__": "rv87_model"}
try:
    exec(compile(model.replace('STATIC_FILES = [', 'STATIC_FILES = [] and [', 1), "rv87_model", "exec"), g)
finally:
    builtins.open = _open
def norm(x):
    return {k: v for k, v in sorted(x.items()) if v}
def cmp(name, mine):
    theirs = {k: v for k, v in tree["forms"][name].items() if v}
    m = norm(mine)
    keys = sorted(set(m) | set(theirs))
    diff = {k: [m.get(k, 0), theirs.get(k, 0)] for k in keys if m.get(k, 0) != theirs.get(k, 0)}
    return {"equal": not diff, "diff_rv87_vs_tree": diff}
b16 = g["build16"]()
b17 = g["build17"]()
res = {
    "T16_P2 (S-1 THIRD, N-1a, N-1c, N-1d)": cmp("T16_P2", b16["P2"]),
    "T16_P3": cmp("T16_P3", b16["P3"]),
    "T16_P4": cmp("T16_P4", b16["P4"]),
    "T16_P1": cmp("T16_P1", b16["P1"]),
    "STAGED (N-1b)": cmp("STAGED", g["STAGED"]),
    "NOTICE (N-1e)": cmp("NOTICE", g["NOTICE_RV87"]),
    "NOTICE_moving": cmp("NOTICE_moving", g["NOTICE_MOV"]),
    "BODY (N-1d)": cmp("BODY", g["BODY"].tree()),
    "SUCC": cmp("SUCC", g["succ_of"](False).tree()),
    "T17_V2_hash": cmp("T17_V2_hash", b17["V2_hash"]),
    "T17_V4 (N-2, without the walkers)": cmp("T17_V4", b17["V4"]),
    "T16_moving": {"rv87": g["last_growth"](g["ENV_S"]), "tree": tree["forms"]["T16_moving"].get("1")},
}
# The THIRD term alone, and whether the tree's P2 minus RV87's P2-without-THIRD equals it.
third = norm(g["THIRD"])
res["THIRD_form"] = third
# ---- the generated Rust table against the tree
rs = open(RS).read()
gen = rs.split("BEGIN GENERATED PROFILE")[1].split("END GENERATED PROFILE")[0]
names_blk = re.search(r"ATOM_NAMES\s*:\s*\[&str;\s*\w+\]\s*=\s*\[(.*?)\];", gen, re.S)
atom_names = re.findall(r'"([^"]*)"', names_blk.group(1)) if names_blk else None
rust_forms = {}
for mm in re.finditer(r'Form \{ name: "([^"]+)", constant: (\d+), terms: &\[(.*?)\] \}', gen):
    terms = {int(i): int(c) for i, c in re.findall(r"\((\d+), (\d+)\)", mm.group(3))}
    rust_forms[mm.group(1)] = (int(mm.group(2)), terms)
trans = {}
if atom_names:
    for name, (const, terms) in rust_forms.items():
        f = {atom_names[i]: c for i, c in terms.items()}
        if const: f["1"] = const
        t = {k: v for k, v in tree["forms"].get(name, {}).items() if v}
        trans[name] = (f == t)
res["rust_atom_names_found"] = bool(atom_names)
res["rust_forms"] = len(rust_forms)
res["rust_transcribes_tree"] = {"all_equal": all(trans.values()) if trans else None,
                                "unequal": [k for k, v in trans.items() if not v],
                                "tree_forms_missing_in_rust": sorted(set(tree["forms"]) - set(rust_forms))}
print(json.dumps(res, indent=1))
