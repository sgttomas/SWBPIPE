"""Carry the TEXT chain's line-keyed rules from OLD to NEW. O-10's copy of RV132's g7_linemap_crates_rows3.py (I65 U4 G7 /
I104 SQ / I107 lineage). Changes, each for O-10:
- every key goes through linemap_lib.Mapper: a key in a changed hunk is re-keyed by function (fn keys) or by its
  line's text (site keys) instead of stopping the run; what cannot be re-keyed is still reported unmapped;
- the KEY pattern has a left boundary `(?<![A-Za-z0-9_])`, so `preview_physics_evidence.rs:N` is never read as
  `physics_evidence.rs:N` (T4-I26's finding);
- with --prune-deleted, a rule whose key names a deleted function (no `fn name` left in its file) or a deleted line is dropped, the list-form rules
  (loop_bounds edge_zero, edge_per_call, ...) included, and every drop is reported (RV132's prune_deleted.py dropped
  only dict-keyed rules).
Keys in descriptive fields (why, evidence, about, reason, note, source) are citations and are left as written.
Usage: python3 linemap_rules.py [--prune-deleted] <repo> <OLD> <NEW> <out dir> <rules json|py> ...
Env: LINEMAP_CRATES (crate_dirs.txt), LINEMAP_ROWS (the basis TEXT rows, comma-separated: data/basis_text_rows.json or a
TEXT run's text_budget json/json.gz; optional), LINEMAP_RESOLUTIONS (rules/linemap_resolutions.json: keys whose function
lost a definition, resolved by review; optional).
"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from linemap_lib import Mapper, load_resolutions, load_rows
args = sys.argv[1:]
prune = "--prune-deleted" in args
args = [a for a in args if a != "--prune-deleted"]
repo, old, new, outdir = args[:4]
srcs = args[4:]
crates = open(os.environ["LINEMAP_CRATES"]).read().split()
rows = load_rows(filter(None, os.environ.get("LINEMAP_ROWS", "").split(",")))
M = Mapper(repo, old, new, crates, rows, load_resolutions(os.environ.get("LINEMAP_RESOLUTIONS")))
KEY = re.compile(r"(?<![A-Za-z0-9_/])(W/)?((?:core/[a-z_/]+/src/)?(?:[a-z_0-9]+/)*[a-z_0-9]+\.rs):(\d+)(?::([A-Za-z_][A-Za-z_0-9]*))?")
DELETED = []    # (ctx) of each deleted key
DICT_PRUNED = set()  # deleted dictionary keys discarded before mapped-key insertion


def fix(s, W, ctx):
    def sub(m):
        whole, wpre, path, ln, name = m.group(0), m.group(1), m.group(2), m.group(3), m.group(4)
        key = (W + "/" + path if wpre else path) + ":" + ln + (":" + name if name else "")
        n_un = len(M.unmapped)
        out = M.map_key(key, ctx)
        if len(M.unmapped) > n_un and M.unmapped[-1][-1] == "deleted":
            DELETED.append(ctx)
        if wpre:
            out = "W/" + out[len(W) + 1:]
        return out
    return KEY.sub(sub, s)


DESCR = {"why", "evidence", "about", "reason", "note", "source"}


def walk(v, W, ctx, descr=False):
    if isinstance(v, str):
        return v if descr else fix(v, W, ctx)
    if isinstance(v, list):
        return [walk(x, W, f"{ctx}[{i}]") for i, x in enumerate(v)]
    if isinstance(v, dict):
        out = {}
        for k, x in v.items():
            key_ctx = ctx + "/" + k + "/key"
            mapped = fix(k, W, key_ctx)
            value = walk(x, W, ctx + "/" + k, k in DESCR)
            if prune and ctx.count("/") == 1 and key_ctx in DELETED:
                # Discard this *original* deleted rule before inserting any
                # mapped key. Its old spelling can equal a different live
                # rule's mapped key; pruning afterward would overwrite/drop
                # that live rule (W1: old deleted lib.rs6382 vs live5811->6382).
                base, family = ctx.split("/", 1)
                pruned.append({"file": base, "family": family, "key": k,
                               "ctx": key_ctx, "rule": value})
                DICT_PRUNED.add(key_ctx)
                continue
            out[mapped] = value
        return out
    return v


os.makedirs(outdir, exist_ok=True)
pruned = []
for src in srcs:
    base = os.path.basename(src)
    if src.endswith(".py"):
        lines = open(src).read().split("\n")
        lines = [fix(l, "", base) if re.match(r'\s*(ZW|ENVZERO)\s*=|.*TB_FN_ZERO"\]\s*=\s*"', l) else l for l in lines]
        open(os.path.join(outdir, base), "w").write("\n".join(lines))
        continue
    data = json.load(open(src))
    W = data.get("W", "") if isinstance(data, dict) else ""
    out = walk(data, W, base)
    if prune and isinstance(out, dict):
        mine = sorted({c for c in DELETED if c.startswith(base + "/") and c not in DICT_PRUNED}, reverse=True)
        for c in mine:
            m = re.match(re.escape(base) + r"/([^/\[]+)(?:\[(\d+)\]|/(.*?)/key$|/(.*))?", c)
            fam = m.group(1)
            if m.group(2) is not None and isinstance(out.get(fam), list):        # a list-form rule: the whole entry
                i = int(m.group(2))
                if i < len(out[fam]) and out[fam][i] is not None:
                    pruned.append({"file": base, "family": fam, "index": i, "ctx": f"{base}/{fam}[{i}]", "rule": out[fam][i]}); out[fam][i] = None
            elif m.group(3) is not None and isinstance(out.get(fam), dict):     # a dict rule keyed by the deleted key
                k = m.group(3)
                for kk in list(out[fam]):
                    if kk == k:
                        pruned.append({"file": base, "family": fam, "key": kk, "ctx": c, "rule": out[fam].pop(kk)})
        for fam, v in out.items():
            if isinstance(v, list):
                out[fam] = [x for x in v if x is not None]
    json.dump(out, open(os.path.join(outdir, base), "w"), indent=1)
summary = M.summary()
if prune:
    summary["pruned_deleted"] = pruned
    summary["unmapped_before_prune"] = list(summary["unmapped"])
    summary["unmapped"] = [u for u in summary["unmapped"] if not (len(u) > 3 and u[3] == "deleted" and any(u[0] == p["ctx"] or u[0].startswith(p["ctx"] + "/") for p in pruned))]
json.dump({"summary": summary, "remapped": M.report}, open(os.path.join(outdir, "g7_linemap.out.json"), "w"), indent=1)
print(json.dumps({k: summary[k] for k in ("old", "new", "remapped", "unmapped")} | {"pruned": len(pruned), "rekeyed_by_fn_or_text": [r for r in M.report if r[3] != "offset"][:60]}, indent=1))
sys.exit(1 if summary["unmapped"] else 0)
