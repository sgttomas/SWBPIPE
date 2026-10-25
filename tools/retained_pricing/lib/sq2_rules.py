"""SQ2 G5: B2's (and B3's) TEXT rules, applied to a copy of the line-mapped SQ chain with N-5 (rn) (stdlib only).
- The count variable z (model combinations; G4_CAPS key "z", default 0) joins the TEXT counts. Branch X's TEXT variant
  is evaluated at z = 0: source recovery is not eligible with a combination (lib.rs source_eligible), so no X phase
  runs with z >= 1.
- The D1.4 combination zero rules are removed; each loop they bounded on the reached graph gets an exact-header rule
  (decisions file NEW, PREFIX), placed first. Further loop, edge and text_args rules come from the decisions file's
  optional lists (LOOPS_FIRST, EDGE_ZERO, EDGE_PER_CALL, FN_CAP, SITE_TOTAL, SITE_FROM, SITE_ZERO, ID_AUDIT, ARGS_FIRST,
  RECURSION, LOOP_TOTAL, EDGE_ONCE, FN_ZERO, FILE_ZERO_REMOVE, EDGE_ZERO_REMOVE, CG_RULES, O10_RECURSION_REVIEWED).
Every action is logged to SQ2_RULES.json in the output chain. Each removed pattern must match exactly one rule.
Usage: python3 sq2_rules.py <chain in> <chain out> <decisions.py>
"""
import json, os, re, shutil, sys, runpy
src, dst, dec_p = sys.argv[1:4]
if os.path.exists(dst): shutil.rmtree(dst)
shutil.copytree(src, dst)
D = runpy.run_path(dec_p)
LOG = {"decisions": os.path.basename(dec_p), "removed": [], "new_loop_rules": [], "other": []}
# O-10: the name-keyed rules of o10_rules.py (both routes) join their families here, after each route's own tables.
D = dict(D)
for _f in ("EDGE_ZERO", "SITE_FROM", "SITE_ZERO", "ID_AUDIT", "ARGS_FIRST"):
    if D.get("O10_" + _f):
        D[_f] = list(D.get(_f, [])) + list(D["O10_" + _f])
# O-10: the decision tables' line keys were written at SQ2's basis (2243be380b, where SQ2's chain was qualified; PP and
# FK production code is identical from c3cf970ad3 to there). They are carried to the target revision (env SQ2_TARGET)
# by the same line map as the rule tables (linemap_lib.Mapper: offset, else by function, else by the line's text); a key
# resolved by name at the snapshot (decisions' `Resolved`, from rs()/rs_line()) is already the target's. Any key that
# cannot be carried stops the run, unless the decisions' KEYMAP_ALLOW lists it with the kind the map reports and a reason
# (rule_ops.carry_decision_keys).
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rule_ops import carry_decision_keys, remove_caller_zero_edges
if os.environ.get("SQ2_TARGET"):
    from linemap_lib import Mapper, load_resolutions, load_rows
    _H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    _rows = load_rows([os.environ.get("SQ2_ROWS", os.path.join(_H, "data", "basis_text_rows.json"))])
    _M = Mapper(os.environ["SQ2_REPO"], os.environ.get("SQ2_DEC_BASIS", "2243be380b"), os.environ["SQ2_TARGET"],
                open(os.path.join(src, "crate_dirs.txt")).read().split(), _rows, load_resolutions(os.environ.get("SQ2_RESOLUTIONS")))
    _bad = carry_decision_keys(D, _M)
    LOG["decision_keymap"] = {"summary": _M.summary(), "report": _M.report}
    if _bad:
        json.dump(LOG, open(os.path.join(dst, "SQ2_RULES.json"), "w"), indent=1)
        raise SystemExit("DECISION KEYS NOT CARRIED: " + json.dumps(_bad))
def patch(fname, old, new):
    p = os.path.join(dst, fname); t = open(p).read()
    if t.count(old) != 1: raise SystemExit(f"PATCH FAILED {fname}: {old[:80]!r}")
    open(p, "w").write(t.replace(old, new)); LOG["other"].append(["patch", fname, old[:70]])
patch("text_budget.py", '"cases", "att", "L", "Pall"):', '"cases", "att", "L", "Pall", "z"):')
patch("g3lib.py", '    c["cases"], c["att"] = c["c"], c["a"]\n', '    c["cases"], c["att"] = c["c"], c["a"]\n    c.setdefault("z", 0)                               # SQ2: model combinations (D1.4, B2-A)\n    if "lc" in c: c["cases"] = c["lc"]; c["att"] = min(c["a"], c["lc"])   # SQ2: the TEXT load cases and attempted cases at a shape (c, z); the typed forms keep c = a = C_eq (HIGH)\n')
_sp = os.path.join(dst, "sens.py"); _st = open(_sp).read()
_m = re.findall(r'    if variant == "_X": extra\["TB_FN_ZERO"\] = "(lib\.rs:\d+:retained_w1)"\n', _st)
if len(_m) != 1: raise SystemExit("PATCH FAILED sens.py _X")
_st = _st.replace(f'    if variant == "_X": extra["TB_FN_ZERO"] = "{_m[0]}"\n',
    f'    if variant == "_X":\n        extra["TB_FN_ZERO"] = "{_m[0]}"\n        extra["G4_CAPS"] = json.dumps(dict(json.loads(caps), z=0))   # SQ2: no X phase with a combination (source_eligible)\n')
open(_sp, "w").write(_st); LOG["other"].append(["patch", "sens.py", "_X at z = 0"])
for f in ("D_BASIS",):
    pass
lbp = os.path.join(dst, "loop_bounds.g4.json"); LB = json.load(open(lbp))
for pat, why in D.get("ZERO_REMOVED", []):
    hits = [i for i, r in enumerate(LB["loops"]) if r["re"] == pat]
    if len(hits) != 1: raise SystemExit(f"ZERO RULE NOT UNIQUE ({len(hits)}): {pat}")
    r = LB["loops"].pop(hits[0])
    LOG["removed"].append({"re": pat, "bound": r["bound"], "why": r["why"]})
first = []
for h, (b, why) in D.get("LOOPS_FIRST_RAW", {}).items():   # raw regexes
    first.append({"re": h, "bound": b, "why": "SQ2 G5: " + why})
for h, (b, why) in D.get("NEW", {}).items():
    first.append({"re": "^" + re.escape(h) + "$", "bound": b, "why": "SQ2 G5: " + why})
for h, (b, why) in D.get("PREFIX", {}).items():
    first.append({"re": "^" + re.escape(h), "bound": b, "why": "SQ2 G5: " + why})
# T4-U6 (O-10, placed first): an exact header that an earlier broad rule would otherwise capture; o10_check.py
# confines it to its listed functions exactly as an O10_LOOPS_LAST rule.
for h, b, why, fns in D.get("O10_LOOPS_FIRST", []):
    first.append({"re": "^" + re.escape(h) + "$", "bound": b, "why": "O-10 (placed first: a broader earlier rule would otherwise match): " + why})
    LOG.setdefault("o10_loops_last", []).append({"header": h, "bound": b, "fns": fns, "placed": "first"})
LOG["new_loop_rules"] = first
LB["loops"][0:0] = first
def add(fam, entry, key=None):
    LB.setdefault(fam, []).append(entry); LOG["other"].append([fam, entry])
for caller, callee, why in D.get("EDGE_ZERO", []):
    add("edge_zero", {"caller": caller, "callee": callee, "why": "SQ2 G5: " + why})
for caller, callee in D.get("EDGE_ZERO_REMOVE", []):
    hits = [i for i, e in enumerate(LB["edge_zero"]) if e["caller"] == caller and e["callee"] == callee]
    if len(hits) != 1: raise SystemExit(f"EDGE_ZERO NOT FOUND {caller} -> {callee}")
    LOG["other"].append(["edge_zero removed", LB["edge_zero"].pop(hits[0])])
for caller, callee, per, why in D.get("EDGE_PER_CALL", []):
    hits = [e for e in LB["edge_per_call"] if e["caller"] == caller and e["callee"] == callee]
    if hits:
        LOG["other"].append(["edge_per_call rebound", dict(hits[0]), per]); hits[0]["per_call"] = per; hits[0]["why"] = "SQ2 G5: " + why + " Was: " + hits[0]["why"]
    else:
        add("edge_per_call", {"caller": caller, "callee": callee, "per_call": per, "why": "SQ2 G5: " + why})
for caller, callee, why in D.get("EDGE_ONCE", []):
    add("edge_once", {"caller": caller, "callee": callee, "why": "SQ2 G5: " + why})
for fn, cap, why in D.get("FN_CAP", []):
    add("fn_cap", {"fn": fn, "cap": cap, "why": "SQ2 G5: " + why})
for fn, why in D.get("FN_ZERO", []):
    add("fn_zero", {"fn": fn, "why": "SQ2 G5: " + why})
for fn, why in D.get("FN_ZERO_REMOVE", []):
    hits = [i for i, e in enumerate(LB["fn_zero"]) if e["fn"] == fn]
    if len(hits) != 1: raise SystemExit(f"FN_ZERO NOT FOUND {fn}")
    LOG["other"].append(["fn_zero removed", LB["fn_zero"].pop(hits[0]), why])
for f, why in D.get("FILE_ZERO_REMOVE", []):
    hits = [i for i, e in enumerate(LB["file_zero"]) if e["file"] == f]
    if len(hits) != 1: raise SystemExit(f"FILE_ZERO NOT FOUND {f}")
    LOG["other"].append(["file_zero removed", LB["file_zero"].pop(hits[0]), why])
for fn, loop, total, why in D.get("LOOP_TOTAL", []):
    add("loop_total", {"fn": fn, "loop": loop, "total": total, "why": "SQ2 G5: " + why})
for name, factor, why in D.get("RECURSION", []):
    LB["recursion_factor"][name] = factor; LOG["other"].append(["recursion_factor", name, factor, why])
for pat, b, why in D.get("REBIND", []):
    hits = [r for r in LB["loops"] if r["re"] == pat]
    if len(hits) != 1: raise SystemExit(f"REBIND NOT UNIQUE ({len(hits)}): {pat}")
    LOG["other"].append(["rebind", pat, hits[0]["bound"], b, why]); hits[0]["why"] = f"SQ2 G5 rebind ({hits[0]['bound']} -> {b}): {why}. Was: " + hits[0]["why"]; hits[0]["bound"] = b
# O-10: the line map's prune may have dropped zero edges of a caller whose callee the head deleted; the expected count
# (written at the basis) is reduced by exactly those (env SQ2_LM: the line map's g7_linemap.out.json).
_PRUNED = json.load(open(os.environ["SQ2_LM"]))["summary"].get("pruned_deleted", []) if os.environ.get("SQ2_LM") else []
LOG["other"] += remove_caller_zero_edges(LB["edge_zero"], D.get("EDGE_ZERO_REMOVE_CALLER", []), _PRUNED)
# B5K1: after route-E's exact-count removal, migrate surviving route-L
# edge bounds to the extracted body. No bound is pruned or replaced by zero.
for old, new, why in D.get("B5_FUNCTION_REKEY", []):
    for entry in LB["edge_zero"]:
        if entry["caller"] == old:
            entry["caller"] = new
            LOG["other"].append(["source body re-key", old, new, entry["callee"], why])
for pat, b, why in D.get("LOOPS_LAST", []):
    LB["loops"].append({"re": pat, "bound": b, "why": "SQ2 G5 (appended last: only headers no earlier rule matches): " + why})
    LOG["other"].append(["loop last", pat, b])
for h, b, why, fns in D.get("O10_LOOPS_LAST", []):
    LB["loops"].append({"re": "^" + re.escape(h) + "$", "bound": b, "why": "O-10 (appended last: only headers no earlier rule matches): " + why})
    LOG.setdefault("o10_loops_last", []).append({"header": h, "bound": b, "fns": fns})
for fn, why in D.get("FN_ZERO_RESTATE", []):
    hits = [e for e in LB["fn_zero"] if e["fn"] == fn]
    if len(hits) != 1: raise SystemExit(f"FN_ZERO_RESTATE NOT FOUND {fn}")
    hits[0]["why"] = "SQ2 G5 restated (0 kept): " + why + " | was: " + hits[0]["why"]; LOG["other"].append(["fn_zero restated", fn, why])
for pat, why in D.get("LOOP_ZERO_RESTATE", []):
    hits = [r for r in LB["loops"] if r["re"] == pat]
    if len(hits) != 1 or str(hits[0]["bound"]) != "0": raise SystemExit(f"LOOP_ZERO_RESTATE {pat}")
    hits[0]["why"] = "SQ2 G5 restated (0 kept): " + why + " | was: " + hits[0]["why"]; LOG["other"].append(["loop zero restated", pat, why])
# O-10: a reviewed bounded self-recursion whose body moved to a function the basis does not have (o10_rules.py, by name).
for fn, why in D.get("O10_RECURSION_REVIEWED", []):
    if any(e["fn"] == fn for e in LB.get("recursion_reviewed", [])): raise SystemExit(f"O10_RECURSION_REVIEWED DUPLICATE {fn}")
    LB.setdefault("recursion_reviewed", []).append({"fn": fn, "why": "O-10: " + why}); LOG["other"].append(["recursion_reviewed", fn, why])
json.dump(LB, open(lbp, "w"), indent=1)
tap = os.path.join(dst, "text_args.g4.json"); TA = json.load(open(tap))
for key, bytes_, why in D.get("SITE_TOTAL", []):
    TA["site_total"][key] = {"bytes": bytes_, "why": "SQ2 G5: " + why}; LOG["other"].append(["site_total", key, bytes_])
if D.get("FIRST_ERROR_GUARD"):
    TA["first_error_guard"] = D["FIRST_ERROR_GUARD"]
for key, entry in D.get("SITE_FROM", []):
    entry = dict(entry); entry["why"] = "SQ2 G5: " + entry["why"]; TA["site_from"][key] = entry; LOG["other"].append(["site_from", key, entry])
for key, per, why in D.get("SITE_FROM_REBIND", []):
    e = TA["site_from"][key]; LOG["other"].append(["site_from rebound", key, e["per"], per])
    e["why"] = f"SQ2 G5 rebind ({e['per']} -> {per}): {why}. Was: " + e["why"]; e["per"] = per
for key, bytes_, why in D.get("O10_SITE_SIZE", []):
    TA["site_size"][key] = {"bytes": bytes_, "why": "O-10: " + why}; LOG["other"].append(["site_size", key, bytes_])
    LOG.setdefault("o10_mult0_sites", []).append(key)
# B7b-JR: sites whose own multiplicity (no size, zero or other rule) must stay 0; o10_check.py fails the run otherwise.
for key, why in D.get("O10_MULT0_SITES", []):
    if not key:
        raise SystemExit(f"O10_MULT0_SITES: unresolved site ({why[:60]})")
    LOG.setdefault("o10_mult0_sites", []).append(key); LOG["other"].append(["mult0_guard", key, why])
for key, why in D.get("SITE_ZERO", []):
    TA["site_zero"][key] = "SQ2 G5: " + why; LOG["other"].append(["site_zero", key])
for key, value in D.get("ID_AUDIT", []):
    TA["id_audit"].setdefault(key, {}).update(value); LOG["other"].append(["id_audit", key, value])
for rule in reversed(D.get("ARGS_FIRST", [])):
    rule = dict(rule); rule["why"] = "SQ2 G5: " + rule["why"]; TA["args"].insert(0, rule); LOG["other"].append(["args", rule])
# W8-RF (retained-pricing-rules.md §8.16): a diagnostic's affected-refs expression and its bound, placed before the
# inherited refs rules (o10_rules.py's O10_REFS_FIRST).
for rule in reversed(D.get("O10_REFS_FIRST", [])):
    rule = dict(rule); rule["why"] = "O-10: " + rule["why"]; TA.setdefault("refs_rules", []).insert(0, rule)
    LOG["other"].append(["refs_first", rule])
for key, value in D.get("LEX_SITE_SIZE", []):
    TA["lex_site_size"][key] = value; LOG["other"].append(["lex_site_size", key, value])
# B7b-DX: exclusive diagnostic groups (o10_rules.py, by name at the snapshot): text_budget.py counts each group's
# diagnostics once per owner call and verifies every member against its owner.
for name, fn, sites, why in D.get("O10_DIAG_EXCLUSIVE", []):
    if name in TA.setdefault("diag_exclusive", {}) or any(s in g["sites"] for g in TA["diag_exclusive"].values() for s in sites):
        raise SystemExit(f"O10_DIAG_EXCLUSIVE duplicate group or member: {name}")
    TA["diag_exclusive"][name] = {"fn": fn, "sites": list(sites), "why": "O-10: " + why}
    LOG["other"].append(["diag_exclusive", name, fn, list(sites)])
if D.get("B7A_SCOPED_TEXT"):
    _scoped = runpy.run_path(D["B7A_SCOPED_TEXT"]["model"])
    TA, _scope_proof = _scoped["transform"](D["B7A_SCOPED_TEXT"]["snapshot"], LB, TA)
    LOG["b7a_scoped_text"] = _scope_proof
if D.get("B7A_OWNER_CHOICES"):
    _choices = runpy.run_path(os.path.join(dst, "b7a_owner_choices.py"))
    TA, _choices_proof = _choices["transform"](os.environ["SQ2_SNAP"], TA)
    LOG["b7a_owner_choices"] = _choices_proof
if D.get("B7A_STAGE1_ROUTE"):
    _phase = runpy.run_path(os.path.join(dst, "b7a_phase_model.py"))
    LB, TA, _phase_proof = _phase["transform"](
        os.environ["SQ2_SNAP"], D["B7A_STAGE1_ROUTE"], LB, TA)
    LOG["w1_stage2_phase"] = _phase_proof
    json.dump(LB, open(lbp, "w"), indent=1)
json.dump(TA, open(tap, "w"), indent=1)
cgp = os.path.join(dst, "callgraph_rules.g4.json"); CG = json.load(open(cgp))
for r in D.get("CG_RULES", []):
    CG["rules"].append(r); LOG["other"].append(["callgraph rule", r])
json.dump(CG, open(cgp, "w"), indent=1)
if D.get("ZW_ADD"):
    _sp = os.path.join(dst, "sens.py"); _st = open(_sp).read()
    _m = re.findall(r'^ZW = "[^"]*"$', _st, re.M)
    if len(_m) != 1: raise SystemExit("ZW not found")
    _st = _st.replace(_m[0], _m[0] + "\nZW += \"," + ",".join(k for k, _ in D["ZW_ADD"]) + "\"   # SQ2: branch-X-exclusive on this route")
    open(_sp, "w").write(_st); LOG["other"].append(["ZW add", D["ZW_ADD"]])
if D.get("CENSUS"):
    import importlib.util
    _spec = importlib.util.spec_from_file_location("census_patch", D["CENSUS"]["patcher"]); _cp = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(_cp)
    _census = json.load(open(D["CENSUS"]["json"]))
    if D["CENSUS"].get("global_end_rows_per_member"):
        # W7-RP (retained-pricing-rules.md §8.20): rows the ordinary envelope carries and the census's source fixtures omit.
        _census = dict(_census, global_end_rows_per_member=D["CENSUS"]["global_end_rows_per_member"])
    LOG["census"] = _cp.apply(dst, _census, patch)
# T4-U6 (O-10): a route's typed-chain substitution, each by exact text with its argued reason (o10_rules.py).
for fname, old, new, why in D.get("O10_TYPED_PATCHES", []):
    patch(fname, old, new); LOG.setdefault("o10_typed_patches", []).append({"file": fname, "why": why})
json.dump(LOG, open(os.path.join(dst, "SQ2_RULES.json"), "w"), indent=1)
print(json.dumps({"removed": len(LOG["removed"]), "new_loop_rules": len(LOG["new_loop_rules"]), "other": len(LOG["other"])}))
