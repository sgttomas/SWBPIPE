"""Build the audit table (for text_args.id_audit) and ID_CLASS_AUDIT.md from id_classified.json."""
import json, sys, re, collections
S, TBP, out_json, out_md = sys.argv[1:5]
# G6 repair: the text_args in force, to find site-level overrides (a format site priced by
# site_size/site_aggregate; a lexicon site priced by lex_site_size). There the expression's generic
# class never priced the site, so the entry is its source bound and the site keeps its override as
# a floor (text_budget.py: size = max(override, audited evaluation)).
TA = json.load(open(sys.argv[5])) if len(sys.argv) > 5 else {"site_size": {}, "site_aggregate": {}, "lex_site_size": {}, "classes": {}}
LEXK = {"to_string", "to_owned", "string_from", "into_text", "join", "replace", "push_str", "clone_text", "collect_string"}
def override(c):
    k = c["site"].split("/src/")[-1]
    if c["kind"] in LEXK:
        f, l = c["site"].rsplit(":", 1)
        lss = TA["lex_site_size"].get(k) or TA["lex_site_size"].get(f.split("core/")[-1].split("/", 1)[-1] + ":" + l)
        return ("lex", lss["class"]) if lss else None
    if k in TA["site_size"] or k in TA.get("site_aggregate", {}):
        return ("site", None)
    return None
sys.path.insert(0, S)
import importlib, id_rules
C = json.load(open(S + "/id_resolved.json"))
TB = json.load(open(TBP))
rows = collections.defaultdict(list)
for r in TB["rows"]:
    rows[(r["file"], r["line"], r["kind"])].append(r)
def fbytes(f, line):
    xs = [r["bytes"] for r in rows.get(("core/" + f, line, "format"), [])]
    if not xs: raise SystemExit(f"no format row {f}:{line}")
    return max(xs)
def maxrange(f, lo, hi):
    return max(r["bytes"] for k, v in rows.items() if k[0] == "core/" + f and lo <= k[1] <= hi and k[2] == "format" for r in v if r["mult"] > 0)
PP = "product_physics/src/"
TPLV = {
    "lib.rs:13044": (max(fbytes(PP+"lib.rs", 12881), fbytes(PP+"lib.rs", 12898), fbytes(PP+"lib.rs", 12913)), "lib.rs:12881/12898/12913"),
    "lib.rs:13074": (max(fbytes(PP+"lib.rs", 12985), fbytes(PP+"lib.rs", 13002), fbytes(PP+"lib.rs", 13017)), "lib.rs:12985/13002/13017"),
    "lib.rs:11617": (maxrange(PP+"lib.rs", 11370, 11420), "lib.rs:11370-11420"),
    "lib.rs:11649": (maxrange(PP+"lib.rs", 11540, 11585), "lib.rs:11540-11585"),
    "lib.rs:4624": (fbytes(PP+"lib.rs", 4609), "lib.rs:4609"),
    "lib.rs:5276": (fbytes(PP+"lib.rs", 5275), "lib.rs:5275"), "lib.rs:5281": (fbytes(PP+"lib.rs", 5275), "lib.rs:5275"),
    "lib.rs:5314": (fbytes(PP+"lib.rs", 5307), "lib.rs:5307"), "lib.rs:5329": (fbytes(PP+"lib.rs", 5307), "lib.rs:5307"),
    "lib.rs:5359": (fbytes(PP+"lib.rs", 5349), "lib.rs:5349"),
}
FID = fbytes(PP + "source_receipt/source.rs", 27)
ENTRY = maxrange(PP + "lib.rs", 1690, 1755)
BOUND = {"TPLLABEL": 131, "IN128": 128, "STATIC": 128, "FMTSTATIC": 512, "NUM": 20, "COMP": 600, "RES": 1024, "DIAG": 2330,
         "NOTID_OVR87": 87, "TPL145": 145, "TPL148": 148, "TPL_FID": FID, "TPL_ENTRY": ENTRY}
CLASSV = {"ident": 128, "ident_debug": 770, "result_id": 1024, "composite_id": 600, "int": 20, "f64_display": 327, "refs_vec": 608}
table, md = collections.defaultdict(dict), []
tot = collections.Counter()
for c in C:
    c = dict(c); c["short"] = c["site"].split("core/")[-1].replace(PP, ""); c["a"] = re.sub(r"\s", "", re.sub(r"(^|\s)//[^\n]*", r"\1", c["arg"])); c["b"] = c["binding"] or ""
    m = id_rules.MANUAL.get((c["short"], c["a"])) or id_rules.MANUAL.get((c["short"], "*"))
    if m: src, why = m
    else:
        for cls, pred, w in id_rules.RULES:
            try: ok = pred(c)
            except Exception: ok = False
            if ok: src, why = cls, w; break
        else: raise SystemExit("unclassified " + c["site"])
    if src == "TPLSITE":
        if c["a"] == "projection.projection_id":
            n, why = 11 + FID, f"Projection.projection_id = format!(\"projection:{{id}}\") with id = functional_id (<= {FID} B): 11 + {FID}"
        else:
            n, at = TPLV[c["short"]]; why = f"{why}: the defining format! sites {at} (the run's bound {n} B)"
    elif src == "NOTID":
        n = None
    else:
        n = BOUND[src]
    debug = "?" in c["spec"]
    cl = c["class"]
    if debug and cl == "ident": cl = "ident_debug"
    if debug and cl == "f64_display": cl = "f64_debug"
    old = cl if isinstance(cl, int) else {**CLASSV, "f64_debug": 24}.get(cl, 0)
    ov = override(c) if c.get("repair") else None
    if ov and ov[0] == "site":
        old = 0          # the generic class never priced this site; its override is the floor
    elif ov and ov[0] == "lex":
        old = ov[1] if isinstance(ov[1], int) else c["class"]   # the lex_site_size price the row had
    if n is None:
        new = old
    else:
        new = max(old, (2 + 6 * n) if debug else n)
    key = c["site"].split("chirality-piping/")[-1] if "chirality-piping/" in c["site"] else c["site"]
    table[key][c["a"] + ("|?" if debug else "")] = {"bytes": new, "source": src}
    tot[src] += 1
    md.append((c["site"].split("core/")[-1], c["kind"], c["arg"][-70:], src, n if n is not None else "-", old, new, c["mult"], why, c["b"][:110]))
json.dump(table, open(out_json, "w"), indent=0, sort_keys=True)
with open(out_md, "w") as fh:
    fh.write("| # | Site | Kind | Copied expression | Source | Bound (B) | Priced before | Priced now | Mult | Why (binding / provenance) |\n|---|---|---|---|---|---|---|---|---|---|\n")
    for i, (site, kind, arg, src, n, old, new, mult, why, b) in enumerate(sorted(md), 1):
        arg = arg.replace("|", "\\|"); why = why.replace("|", "\\|"); b = b.replace("|", "\\|").replace("`", "'")
        fh.write(f"| {i} | `{site}` | {kind} | `{arg}` | {src} | {n} | {old} | {new} | {mult:,} | {why}; binding `{b}` |\n")
print(json.dumps({"sites": len(table), "entries": len(md), "by_source": tot, "FID": FID, "ENTRY": ENTRY,
                  "raised": sum(1 for x in md if x[6] > x[5])}))
