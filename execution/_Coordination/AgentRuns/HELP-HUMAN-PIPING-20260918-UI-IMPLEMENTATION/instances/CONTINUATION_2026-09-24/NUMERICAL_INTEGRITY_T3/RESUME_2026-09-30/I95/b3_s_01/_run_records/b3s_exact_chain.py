"""I95 B3-S: the exact-route variant of I82's multi-case chain, made in a scratch copy.

Input: I82's multi-case chain (b1_mc_chain.py's output over the line-mapped u4_g7_06 chain; argv[1]).
Output: a patched copy (argv[2]) and B3S_REBINDS.json beside it. No maintained file and no record is
changed. Every removal and every textual patch must match exactly the stated number of times, or this
script fails.

The route priced: model 0.3.0 with pressure_contract {2.0.0, exact_straight_pressure_v2} (PLAN section 1.4
item 3), every case's pressure_regions explicitly Some([]) ("D1.5-exact"), sections absent, D1.4/D1.6-D1.9
unchanged, no load-reference state (0.4.0 stays excluded). Under it `pressure_runtime::is_exact` is true,
so every TEXT rule whose reason is "is_exact is false (D1.3)" is rebound here:
  - removed, so its sites are priced at their real multiplicity; or
  - kept at zero with its reason restated against the exact route (explicitly empty regions); or
  - made branch-exclusive (TB_FN_ZERO of one branch, as sens.py's existing ZW).
Rules whose reason is the load-reference state (0.4.0), sections, D1.4, D1.6 or D1.7 are untouched: the
exact route keeps those clauses.

The census side (`--census <json>`, b3s_census.py's output): the exact route publishes no preview tree
(PP lib.rs:4673, `preview_record` only when !is_exact) and carries contract_evidence {connector,
exact_cases, pressure} instead (lib.rs:2841). Each per-case preview-tree Value form in the chain is
replaced by the componentwise maximum of the preview tree's facts and the exact evidence's facts at the
same caps (an upper bound of either; no credit is taken for the missing preview tree). P_final stays
7n + 51m + 8g + 3, which bounds the exact route's census (7n + 51m + 8g per case, plus at most the same
basis records).
Usage: python3 b3s_exact_chain.py <I82 mc chain dir> <out dir> [--census <census json>] [--credits]
       [--exact-only --edges <step 0's edges_base.json>]
"""
import json, os, shutil, sys

src, dst = sys.argv[1:3]
census = None
if "--census" in sys.argv:
    census = json.load(open(sys.argv[sys.argv.index("--census") + 1]))
if os.path.exists(dst):
    shutil.rmtree(dst)
shutil.copytree(src, dst)
LOG = {"removed": [], "restated": [], "branch_exclusive": [], "unchanged_d13": [], "added_rules": [], "patches": []}
EXACT = ("exact route (B3b): 0.3.0 with {2.0.0, exact_straight_pressure_v2}, so is_exact is true; every case's "
         "pressure_regions is explicitly Some([]) (D1.5-exact)")


def patch(fname, old, new, count=1):
    p = os.path.join(dst, fname)
    t = open(p).read()
    k = t.count(old)
    if k != count:
        raise SystemExit(f"PATCH FAILED {fname}: expected {count} match(es), found {k}: {old[:80]!r}")
    open(p, "w").write(t.replace(old, new))
    LOG["patches"].append((fname, old[:70].replace("\n", " "), k))


lbp = os.path.join(dst, "loop_bounds.g4.json")
LB = json.load(open(lbp))

# ---------------------------------------------------------------- 1. fn_zero
def take_fn_zero(key, action, why):
    hits = [e for e in LB["fn_zero"] if e["fn"] == key]
    assert len(hits) == 1, key
    if action == "remove":
        LB["fn_zero"].remove(hits[0])
        LB_removed = {"family": "fn_zero", "key": key, "old_why": hits[0]["why"], "new": why}
        LOG["removed"].append(LB_removed)
    else:
        old = hits[0]["why"]
        hits[0]["why"] = "I95 B3-S restated: " + why + " | was: " + old
        LOG["restated"].append({"family": "fn_zero", "key": key, "old_why": old, "new": why, "multiplicity": 0})

take_fn_zero("lib.rs:11705:append_exact_pressure_results", "restate",
             EXACT + ". Its only call (PP lib.rs:5373-5389) is under `exact_pressure...pipe_states.get(&pipe_index)` "
             "Some; build_pressure_case_with_members inserts a pipe state only for a member of a region "
             "(pressure_runtime.rs region loop), so with every region list empty pipe_states is empty and the "
             "call never happens: multiplicity 0")

# ---------------------------------------------------------------- 2. file_zero
def take_file_zero(path, why):
    hits = [e for e in LB["file_zero"] if e["file"] == path]
    assert len(hits) == 1, path
    LB["file_zero"].remove(hits[0])
    LOG["removed"].append({"family": "file_zero", "key": path, "old_why": hits[0]["why"], "new": why})

take_file_zero("core/product_physics/src/source_receipt/composite.rs",
               EXACT + ". composite_support_norms (lib.rs:4748-4750) and composite_member_maximum (lib.rs:5268-5270) "
               "run under is_exact with a selected source; validate_publication runs in finalize_for when "
               "`composite` = source_selected && is_exact (lib.rs:2838; source_receipt.rs:955). All three are "
               "priced at their multiplicity in the full and X runs; branch W excludes them (item 6)")
take_file_zero("core/product_physics/src/pressure_material.rs",
               EXACT + ". resolve_base (lib.rs:2434) no longer returns at its !is_exact check, and resolve_case "
               "(lib.rs:9634-9635) is called under is_exact: both priced at their multiplicity")
# physical_source_built keeps its own non-D1.3 reason (the file_zero's third clause): `physical` is Some
# only on the captured load-state replay (0.4.0, still excluded) or from composite::validate_publication's
# own replay; the latter is priced (it is reached only through finalize_for)

# ---------------------------------------------------------------- 3. edge_zero
bp = [e for e in LB["edge_zero"] if e["caller"] == "pressure_runtime.rs:427:build_pressure_case_with_members"]
assert len(bp) == 51, len(bp)
for e in bp:
    LB["edge_zero"].remove(e)
LOG["removed"].append({"family": "edge_zero", "key": "pressure_runtime.rs:427:build_pressure_case_with_members -> * (51 edges)",
                       "callees": sorted(e["callee"] for e in bp), "old_why": bp[0]["why"],
                       "new": EXACT + ". The function no longer returns at its !is_exact check (pressure_runtime.rs:436-438); "
                              "each callee is priced at its call sites' loop bounds. Callees inside the per-region loop "
                              "take that loop's bound (0: every region list is empty, loop rule 220 restated)"})
cs = [e for e in LB["edge_zero"] if e["caller"] == "pressure_runtime.rs:111:validate_profile"
      and e["callee"] == "pressure_runtime.rs:357:check_suffixes"]
assert len(cs) == 1
LB["edge_zero"].remove(cs[0])
LOG["removed"].append({"family": "edge_zero", "key": "pressure_runtime.rs:111:validate_profile -> pressure_runtime.rs:357:check_suffixes",
                       "old_why": cs[0]["why"],
                       "new": EXACT + ". The is_exact branch (pressure_runtime.rs:167-200) calls check_suffixes four times "
                              "(pipes, nodes, supports, load cases); its problem() sites are bounded by its id loop"})
# the precommit reader: physics-retained-1's G7 runs physics-1's base validator (PLAN section 1.4, readers)
pre = [e for e in LB["edge_zero"] if e["caller"] == "semantic_contract.rs:437:for_source"
       and e["callee"] in ("physics_evidence.rs:319:validate_physics_evidence", "semantic_contract.rs:490:forbid_load_reference_evidence")]
assert len(pre) == 2, len(pre)
for e in pre:
    LB["edge_zero"].remove(e)
    LOG["removed"].append({"family": "edge_zero", "key": f"{e['caller']} -> {e['callee']}", "old_why": e["why"],
                           "new": "exact route's precommit validator: physics-retained-1's G7 runs physics-1's base "
                                  "validator on the projection (PLAN section 1.4, D2 section 4.9.1), i.e. for_source's "
                                  "PHYSICS_ID arm (semantic_contract.rs:440-443: forbid_load_reference_evidence, then "
                                  "validate_physics_evidence). The PREVIEW_PHYSICS_ID arm stays priced (no credit)"})
for e in LB["edge_zero"]:
    if e["caller"] == "semantic_contract.rs:437:for_source" or e["caller"] == "semantic_contract.rs:269:for_source_metadata":
        LOG["unchanged_d13"].append({"family": "edge_zero", "key": f"{e['caller']} -> {e['callee']}",
                                     "why_kept": "not physics-1's arm: load-reference (0.4.0, excluded), physics-source-1 "
                                                 "and source-blocks-1 (no precommit on the X branch), or the is_retained "
                                                 "branch (the projection is not retained)"})

# the composite publication replay keeps D1.3's load-state exclusion: physical_source takes its
# captured_load_state_case branch only for a 0.4.0 model (composite.rs:415-431), which stays excluded
LB["edge_zero"].append({"caller": "source_receipt/composite.rs:406:physical_source",
                        "callee": "source_receipt.rs:152:captured_load_state_case",
                        "why": "I95 B3-S (D1.3's load-state clause, unchanged on the exact route): called only under "
                               "case_state::is_load_state (composite.rs:415-417), false for 0.3.0"})
LOG["added_rules"].append({"family": "edge_zero", "key": "composite::physical_source -> captured_load_state_case",
                           "why": "D1.3's load-state exclusion kept (0.4.0 only)"})

# ---------------------------------------------------------------- 4. edge_per_call
ep = [e for e in LB["edge_per_call"] if e["caller"] == "pressure_runtime.rs:111:validate_profile"]
assert len(ep) == 1 and ep[0]["per_call"] == "L", ep
old = ep[0]["why"]
ep[0]["why"] = ("I95 B3-S restated (L unchanged): " + EXACT + ". Of validate_profile's direct problem() calls the "
                "exact arm makes, per call: components 0 (D1.4); nonlinear or constant-effort supports 0 (D1.6); "
                "combinations 0 (D1.4); per case equivalent_static 0 (D1.5), pressure_regions None 0 (D1.5-exact), "
                "the region loop 0 (empty); EXACT_PRESSURE_REQUIRES_REGION at most once per primitive load whose "
                "category or dimension is pressure: <= L over every case (pressure_runtime.rs:186-191). | was: " + old)
LOG["restated"].append({"family": "edge_per_call", "key": "pressure_runtime.rs:111:validate_profile -> pressure_runtime.rs:85:problem",
                        "old_why": old, "new": ep[0]["why"], "multiplicity": "L per call (unchanged)"})

# check_suffixes' id loop: the newly priced edge takes its loop's real collection, not the generic
# 'entity id iterator' rule (written for load-id lists; I82 rebound it to max(l, L))
LB["edge_per_call"].append({"caller": "pressure_runtime.rs:357:check_suffixes", "callee": "pressure_runtime.rs:85:problem",
                            "per_call": "max(n, m, g, cases)",
                            "why": "I95 B3-S (exact route, newly reached; the loop's real bound): check_suffixes is called on the "
                                   "pipe, node, support and load-case id lists (pressure_runtime.rs:175-194) and reports at most one "
                                   "collision per id (:363-367), so <= max(n, m, g, c) per call; the generic 'entity id iterator' "
                                   "rule would give max(l, L)"})
LOG["added_rules"].append({"family": "edge_per_call", "key": "check_suffixes -> problem", "per_call": "max(n, m, g, cases)"})

# ---------------------------------------------------------------- 5. loop rules
def restate_loop(rx, why):
    hits = [r for r in LB["loops"] if r["re"] == rx]
    assert len(hits) == 1, rx
    old = hits[0]["why"]
    assert str(hits[0]["bound"]) == "0"
    hits[0]["why"] = "I95 B3-S restated (0 kept): " + why + " | was: " + old
    LOG["restated"].append({"family": "loops", "key": rx, "old_why": old, "new": why, "multiplicity": 0})

restate_loop("exact\\.assembled_operands",
             EXACT + ". assembled_operands is filled only by add_factor_term inside the per-region traversal "
             "(pressure_runtime.rs:794); with no region it is empty")
restate_loop("traversal|terminals|\\(dof, pbits, hi, lo, _\\), group\\) in groups",
             EXACT + ". traversals, terminals and factor groups exist only per region; every region list is empty")
restate_loop("pressure|region|aggregates",
             EXACT + ". Every loop this rule zeroes on the exact graph iterates a region list, a region's members "
                     "or terminals, pipe_states (filled only per region member) or pressure-thrust aggregates "
                     "(expansion-joint components, D1.4); each is empty. b3s_zero_review.py lists every header it "
                     "matches on the exact graph for review")

# ---------------------------------------------------------------- 6. branch W: X-only composite entries
patch("sens.py", '''ZW = "source_receipt.rs:636:exact,source_receipt.rs:839:finalize,source_receipt.rs:853:finalize_composite,source_receipt.rs:867:finalize_for"''',
      '''ZW = "source_receipt.rs:636:exact,source_receipt.rs:839:finalize,source_receipt.rs:853:finalize_composite,source_receipt.rs:867:finalize_for"
# I95 B3-S: on the exact route the ordinary path calls two composite entries only with a selected source
# (lib.rs:4748-4750, :5268-5270); under T-3 (c) a selected case publishes the exact ordinary bytes, so W1's
# branch W never has one: they are branch-X-exclusive, like finalize_composite
ZW += ",source_receipt/composite.rs:353:composite_support_norms,source_receipt/composite.rs:165:composite_member_maximum,source_receipt/composite.rs:721:composite_exact"''')
LOG["branch_exclusive"].append({"variant": "_W (TB_FN_ZERO)", "keys": ["composite_support_norms", "composite_member_maximum", "composite_exact"],
                                "why": "called only with selected_source Some (lib.rs:4748-4750, :5268-5270, :5490-5496); T-3 (c): W1 "
                                       "publishes only when no case is source-selected. The unselected exact case takes "
                                       "FinalizedSourceBlockCase::ordinary_physics (lib.rs:5505-5508), which stays priced in W"})

# ---------------------------------------------------------------- 7. loop rules for the newly reached loops
# Every loop header the rebinding newly reaches, and no existing rule matches, gets an anchored rule. Bounds are the published projection's counts (Pall rows, cases, m members, g supports,
# D diagnostics) or fixed tables. Loops nested in a region loop are reached at multiplicity 0 (rule 220).
NEW_LOOPS = [
    # physics_evidence.rs: physics-1's base validator, run by physics-retained-1's G7 on the projection
    ('^for diagnostic in array\\(&source\\[" "\\]\\)\\?$', "D", "physics_evidence.rs:341: the projection's diagnostics <= D"),
    ('^for reference in array\\(&case\\[" "\\]\\)\\?$', "D", "physics_evidence.rs:365: a numerical case's evidence_refs name envelope diagnostics <= D"),
    ('^for \\(case_id, case\\) in &cases$', "cases", "physics_evidence.rs:385, :810: one entry per load case"),
    ('^for s in sections\\.values\\(\\)$', "m", "physics_evidence.rs:424: one section per member"),
    ('^for \\(id, row\\) in &rows$', "Pall", "physics_evidence.rs:514, :805: every published row"),
    ('^for components in support_components\\.values\\(\\)$', "g", "physics_evidence.rs:564: one entry per support"),
    ('^for \\(member, duplicate\\) in duplicates$', "m", "physics_evidence.rs:655 (inside the region loop): per member"),
    ('^for item in applied\\.values\\(\\)$', "N", "physics_evidence.rs:676 (inside the region loop): applied loads per DOF"),
    ('^for \\(reaction, closure\\) in array\\(&terminal\\[" "\\]\\)\\? \\.iter\\(\\) \\.zip\\(array\\(&terminal\\[" "\\]\\)\\?\\)$', "3",
     "physics_evidence.rs:690 (inside the region loop): three axes"),
    ('^for id in actual$', "Pall", "physics_evidence.rs:762 (inside the region loop): bound result ids <= rows"),
    ('^iter:rows \\.iter\\(\\)\\.filter$', "Pall", "physics_evidence.rs:752 (inside the region loop): every published row"),
    ('^for component in components$', "6", "physics_evidence.rs:794 (inside the region loop): <= 2 components per kind; 6 bounds every reached table of this header"),
    ('^for station in stations$', "5", "physics_evidence.rs:795: STATIONS (5) or its first 2"),
    # composite.rs: the selected composite finalization and support/member norms (branch X)
    ('^for \\(i, v\\) in values\\.into_iter\\(\\)\\.enumerate\\(\\)$', "3", "composite.rs:214, physics_source.rs:254: a [f64; 3]"),
    ('^for row in actual$', "Pall", "composite.rs:604: a case's published rows <= every row"),
    ('^for mut row in expected$', "Pall", "composite.rs:704: expected rows <= every row"),
    ('^for id in member_ids$', "m", "composite.rs:780: member ids"),
    ('^for \\(group, mut row\\) in generated\\.into_iter\\(\\)\\.skip\\(6\\)\\.enumerate\\(\\)$', "2",
     "composite.rs:816: append_signed_support_results makes 8 rows; skip(6) leaves the 2 magnitudes"),
    ('^for row in actual\\.iter\\(\\)\\.filter\\(\\|r\\| r\\.entity_ref == member\\.member_id\\)$', "Pall", "composite.rs:842: every row"),
    ('^iter:" ":source_recovery::STATIONS\\.iter\\(\\)\\.enumerate\\(\\)\\.map$', "5", "composite.rs:914: STATIONS has 5 entries"),
    ('^iter:rows: actual \\.iter\\(\\)\\.map$', "Pall", "composite.rs:966: every row"),
    ('^for \\(index, \\(case, physical\\)\\) in cases\\.iter\\(\\)\\.zip\\(physical\\)\\.enumerate\\(\\)$', "cases", "composite.rs:1042: one per case"),
    ('^iter:inputs: \\(0\\.\\.3\\)\\.map$', "3", "composite.rs:825: three inputs"),
    # physics_evidence.rs helpers and assembly() (reached once the PHYSICS_ID arm is priced)
    ('^for item in a$', "Pall", "physics_evidence.rs:62 strings(): an id list (node_order n, pipe ids m, result ids <= rows)"),
    ('^for x in array\\(v\\)\\?$', "N", "physics_evidence.rs:98 vector(): length 2, 3 or 6n"),
    ('^for k in &GEOMETRY_KEYS\\[2\\.\\.\\]$', "9", "physics_evidence.rs:144: GEOMETRY_KEYS has 11 entries"),
    ('^for \\(i, n\\) in array\\(&v\\[values\\]\\)\\?\\.iter\\(\\)\\.enumerate\\(\\)$', "N", "physics_evidence.rs:892: an RHS vector of 6n"),
    ('^for group in array\\(&v\\[" "\\]\\)\\?$', "0",
     "physics_evidence.rs:899 assembly(): `groups` holds one entry per pressure source group, made only inside a "
     "region (finish_source_groups' group loop); every region list is empty (D1.5-exact)"),
    ('^for term in array\\(&group\\[" "\\]\\)\\?$', "m", "physics_evidence.rs:951: terms of one group <= members"),
    # physics_source.rs: physics-source-1's validator, reached from the composite finalization (branch X)
    ('^for \\(endpoint, \\(end, claim\\)\\) in ends\\.iter\\(\\)\\.zip\\(claimed\\)\\.enumerate\\(\\)$', "2", "physics_source.rs:375: two endpoints (required at :370-373)"),
    ('^for \\(\\(index, id\\), action\\) in array\\(&end\\[" "\\]\\)\\? \\.iter\\(\\) \\.zip\\(array\\(&end\\[" "\\]\\)\\?\\) \\.zip\\(array\\(&end\\[" "\\]\\)\\?\\)$',
     "3", "physics_source.rs:388: three functionals per endpoint"),
    ('^for end in ends$', "2", "physics_source.rs:432: two endpoints"),
]
# appended LAST: each applies only where no existing rule matches (every header here was unmapped), so no
# existing site's bound changes
for rx, bound, why in NEW_LOOPS:
    LB["loops"].append({"re": rx, "bound": bound, "why": "I95 B3-S (exact route, newly reached): " + why})
    LOG["added_rules"].append({"family": "loops", "re": rx, "bound": bound, "why": why})

# ---------------------------------------------------------------- 7b. physics_evidence error-path sites
# As I65's G4 rule for the preview validator (text_args site_from preview_physics_evidence.rs:90, :100):
# require()/text()/number()/array() allocate only on failure, and every failure returns through `?` to
# validate_physics_evidence's caller; the one `.ok()` that swallows an Err from these helpers
# (physics_evidence.rs:755, case_basis(row).ok()) is inside the region loop (multiplicity 0 here).
tap = os.path.join(dst, "text_args.g4.json")
TA = json.load(open(tap))
for line in (38, 48, 53, 57):
    key = f"physics_evidence.rs:{line}"
    assert key not in TA["site_from"], key
    TA["site_from"][key] = {"from": ["physics_evidence.rs:319:validate_physics_evidence"], "per": "1",
                            "why": "I95 B3-S (exact route, newly reached; as G4's preview_physics_evidence.rs:90/:100): an error-path "
                                   "allocation; require()/text()/number()/array() allocate only on failure and every failure returns "
                                   "through `?` (physics_evidence.rs:34-58); the only swallowing `.ok()` (:755) is inside the region "
                                   "loop, empty on this route; so <= 1 per validate_physics_evidence call"}
    LOG["added_rules"].append({"family": "text_args.site_from", "key": key, "from": "validate_physics_evidence", "per": "1"})
json.dump(TA, open(tap, "w"), indent=1)

# ---------------------------------------------------------------- 8. sens.py: evaluate every variant
# The chain's sens.py stops evaluating variants after the first incomplete one (`complete and tb(..)`).
# This only reorders that boolean so every variant runs; completeness is still reported.
patch("sens.py", '''    complete = complete and tb(v, D)["complete"]''', '''    complete = tb(v, D)["complete"] and complete     # I95: evaluate every variant''')

# ---------------------------------------------------------------- 8b. the identifier audit of the new sites
# The newly reached sites carry identifier-bearing placeholders that I65's audit table (text_args id_audit)
# has no entry for ("id-unaudited"). text_budget still prices each at its argument rule's class value; only
# the audit is missing. t08_closure would refuse such a run, so here it accepts a run whose ONLY open items
# are id-unaudited, and sens.py reports them (text_complete stays false). G5's audit replaces this.
patch("t08_closure.py", '''assert tbe["complete"], "envelope text budget incomplete"''',
      '''def _only_id(t):                                    # I95 B3-S: only identifier-audit items open
    return t["complete"] or (not t["unmapped_loop_headers"] and all(u[0][0] == "id-unaudited" for u in t["unclassified_args"]))
assert _only_id(tbe), "envelope text budget incomplete"''')
patch("t08_closure.py", '''assert tb["complete"], "text budget incomplete"''', '''assert _only_id(tb), "text budget incomplete"''')

# ---------------------------------------------------------------- 9. exact-route zero rules (--credits)
# Sites that the exact route reaches but cannot execute when every region list is empty. The old D1.3
# rule zeroed build_pressure_case_with_members whole; these keep only what is still unreachable.
if "--credits" in sys.argv:
    LB["edge_per_call"].append({"caller": "pressure_runtime.rs:427:build_pressure_case_with_members",
                                "callee": "pressure_runtime.rs:85:problem", "per_call": "mats",
                                "why": "I95 B3-S (D1.5-exact credit): per call, of its problem() sites only "
                                       "EXACT_PRESSURE_MATERIAL_AMBIGUOUS can fire, once per selected material "
                                       "(pressure_runtime.rs:461-468): pressure_regions is Some (no REGIONS_REQUIRED, :439-446); "
                                       "the region loop is empty (:474); every cap/eigen term list is empty, so exact_sum "
                                       "returns Ok(+0.0) (pressure_sum.rs:16-22; :748-761 never reaches `_ => problem`); "
                                       "every assembled vector is +0.0, so the nonfinite check (:764-775) never fires"})
    # finish_source_groups still runs (it builds assembly_evidence); only its problem() sites are credited
    LB["edge_zero"].append({"caller": "pressure_runtime.rs:836:finish_source_groups",
                            "callee": "pressure_runtime.rs:85:problem",
                            "why": "I95 B3-S (D1.5-exact credit): groups is empty (no region, no add_factor_term), so its "
                                   "two in-loop problem() sites never run; every by_dof list is empty, so exact_sum returns "
                                   "Ok(+0.0) and the per-DOF `Err(_) => problem` never runs; max_rhs = 0 with every list "
                                   "empty leaves maximum_ratio 0, so screen = 0 and the cancellation problem() never runs "
                                   "(pressure_runtime.rs:845-907)"})
    LOG["added_rules"].append({"family": "edge_per_call (credit)", "key": "build_pressure_case_with_members -> problem", "per_call": "mats"})
    LOG["added_rules"].append({"family": "edge_zero (credit)", "key": "finish_source_groups -> problem"})

# ---------------------------------------------------------------- 9b. route-exclusive forms (--exact-only)
# The mirror of the D1.3 rules: with is_exact true, these legacy-route branches cannot run. Without this flag
# the chain prices the union of both routes (one form set for both); with it, the exact route's own forms
# (PLAN section 3.2 item 1: cap_priced_maximum then takes the maximum over the route-specific forms).
if "--exact-only" in sys.argv:
    E_ = json.load(open(sys.argv[sys.argv.index("--edges") + 1]))["edges"]
    MIRROR = [
        ("lib.rs:3879:solve_load_case_observed", "source_receipt.rs:636:exact",
         "a selected exact case takes FinalizedSourceBlockCase::composite_exact (lib.rs:5493-5496); `exact` is the !is_exact arm (:5497-5498)"),
        ("lib.rs:3879:solve_load_case_observed", "source_receipt.rs:720:ordinary",
         "an unselected exact case takes ordinary_physics (lib.rs:5505-5508); `ordinary` is the legacy arm (:5509-5510)"),
        ("lib.rs:2372:run_linear_static_preview_observed", "source_receipt.rs:839:finalize",
         "`composite` = source_selected && is_exact (lib.rs:2838), so a selected exact invocation takes finalize_composite (:2890-2891)"),
        ("lib.rs:2372:run_linear_static_preview_observed", "preview_physics.rs:528:render",
         "`preview` exists only when !is_exact && !source_selected (lib.rs:2711)"),
        ("lib.rs:2372:run_linear_static_preview_observed", "preview_physics.rs:183:evidence",
         "the preview tree is rendered only from `preview` (lib.rs:2878-2881), None on the exact route"),
        ("lib.rs:3879:solve_load_case_observed", "preview_physics.rs:183:evidence",
         "preview_record is None when is_exact (lib.rs:4673-4674)"),
        ("semantic_contract.rs:437:for_source", "preview_physics_evidence.rs:244:validate_preview_physics_evidence",
         "physics-retained-1's G7 runs physics-1's base validator (the PHYSICS_ID arm, semantic_contract.rs:440-443), not the preview arm"),
    ]
    for a_, b_, why in MIRROR:
        fa = [k for k in E_ if k.endswith("/" + a_)]; fb = [k for k in E_ if k.endswith("/" + b_)]
        assert len(fa) == 1 and len(fb) == 1 and fb[0] in E_[fa[0]], (a_, b_)
        LB["edge_zero"].append({"caller": a_, "callee": b_, "why": "I95 B3-S (exact-route-only forms): " + why})
        LOG["added_rules"].append({"family": "edge_zero (exact-only)", "key": f"{a_} -> {b_}", "why": why})
    for k_, why in (("source_receipt.rs:152:captured_load_state_case", "the 0.4.0 load-state replay; D1.3's load-state clause holds on the exact route (0.3.0 only)"),
                    ("source_receipt.rs:197:check_load_state_input", "the 0.4.0 load-state replay; D1.3's load-state clause holds on the exact route (0.3.0 only)")):
        LB["fn_zero"].append({"fn": k_, "why": "I95 B3-S (exact-route-only forms): " + why})
        LOG["added_rules"].append({"family": "fn_zero (exact-only)", "key": k_, "why": why})

# ---------------------------------------------------------------- 10. the census: the per-case evidence tree
# Each per-case preview-tree Value form becomes the componentwise maximum of the preview tree's facts and the
# exact evidence's facts, both as functions of (n, m, g) (the exact parts measured by b3s_census.py).
if census is not None:
    parts = census["exact_evidence_case_parts"]
    FIELDS = ("arr", "obj", "ent", "strb", "keyb", "nums")
    def coef(k):
        return (parts["rest"][k], parts["per_pipe"][k] + parts["per_unavailable_pipe"][k], 6 * parts["per_dof"][k] + parts["per_node"][k])
    helper = ("\n# I95 B3-S census: per-case evidence facts = max(preview tree, exact evidence) at (n, m, g)\n"
              "_EXC = " + repr({k: coef(k) for k in FIELDS}) + "\n"
              "def _exmax(n, m, g):\n"
              "    pv = dict(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,\n"
              "              strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),\n"
              "              keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m)\n"
              "    return {k: max(pv[k], _EXC[k][0] + _EXC[k][1] * m + _EXC[k][2] * n) for k in pv}\n")
    # ordinary_caps: PREVIEW (used by two families)
    patch("ordinary_caps.py", '''PREVIEW = value_tree(3 * m + 2 * g, 3 + m + g, 9 + 15 * m + 2 * g,
                     m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
                     (9 + 15 * m + 2 * g) * 40)''', helper + '''_PX = _exmax(n, m, g)
PREVIEW = value_tree(_PX["arr"], _PX["obj"], _PX["ent"], _PX["strb"], _PX["keyb"])   # I95: max(preview, exact)''')
    # g4_caps: ENV's per-case tree and PREVIEW_T
    patch("g4_caps.py", '''       + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
            strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64), keyb=(9 + 15 * m + 2 * g) * 40,
            nums=10 * m, depth=6).scale(CC)''', '''       + VF(depth=7, **_exmax(n, m, g)).scale(CC)                      # I95: max(preview, exact)''')
    patch("g4_caps.py", '''rows = {}
# =========================================================================== envelope facts''', helper + '''rows = {}
# =========================================================================== envelope facts''')
    patch("g4_caps.py", '''PREVIEW_T = (s("Value") * (3 * m + 2 * g) + X({"Node(String,Value)": 3 + m + g + (9 + 15 * m + 2 * g) // 5})
             + B(m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64) + (9 + 15 * m + 2 * g) * 40))''',
          '''_PX = _exmax(n, m, g)                                                                # I95: max(preview, exact)
PREVIEW_T = (s("Value") * _PX["arr"] + X({"Node(String,Value)": _PX["obj"] + _PX["ent"] // 5}) + B(_PX["strb"] + _PX["keyb"]))''')
    # t25_g4: env_vf's per-case tree
    patch("t25_g4.py", '''          + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
               strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
               keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m, depth=6).scale(CC)''', '''          + VF(depth=7, **_exmax(n, m, g)).scale(CC)                 # I95: max(preview, exact)''')
    patch("t25_g4.py", '''# ---------------------------------------------------------------- per-invocation finalize''',
          helper + '''# ---------------------------------------------------------------- per-invocation finalize''')
    LOG["census"] = {"per_case_evidence_coefficients": {k: coef(k) for k in FIELDS},
                     "at_caps": census["componentwise_max"], "sites": ["ordinary_caps PREVIEW", "g4_caps ENV", "g4_caps PREVIEW_T", "t25_g4 env_vf"]}

json.dump(LB, open(lbp, "w"), indent=1)
json.dump(LOG, open(os.path.join(dst, "B3S_REBINDS.json"), "w"), indent=1)
print(json.dumps({k: len(v) for k, v in LOG.items()}))
