"""SQ2 G5, route E: D1.3's branch E (B3b; schema 0.3.0 with {2.0.0, exact_straight_pressure_v2}, every case's
pressure_regions explicitly Some([]) with capacity 0, sections absent; D1.4's exact clause refuses every combination, so
z = 0 here). I95's B3-S rebinding (R/I95/b3_s_01: STUDY section 2; RR "I95's B3-S ...", rulings 1, 2 and 5), carried to
the real code at c3cf970ad3 (keys re-derived, each premise re-read), with B3b-P's own exact W1 and the RS reader's exact
branch live, and the mirror of route L: the preview-only W1 functions and branches zero."""
import os, json, runpy
H = os.path.dirname(os.path.abspath(__file__))
globals().update({k: v for k, v in runpy.run_path(os.path.join(H, "sq2_decisions_common.py")).items() if k.isupper() or k in ("rs", "rs_any", "Resolved")})
EXACT = ("route E (B3b): 0.3.0 with {2.0.0, exact_straight_pressure_v2}, so pressure_runtime::is_exact is true; every case's "
         "pressure_regions is explicitly Some([]) (D1.5's exact clause)")
FN_ZERO_RESTATE = [
    ("lib.rs:11552:append_exact_pressure_results", EXACT + ". Its only call (lib.rs:5595-5599) is under `exact_pressure...pipe_states.get(&pipe_index)` Some; "
     "build_pressure_case_with_members inserts a pipe state only for a member of a region (its region loop, pressure_runtime.rs:474ff.), so with every region "
     "list empty pipe_states is empty and the call never happens"),
]
FILE_ZERO_REMOVE = [
    ("core/product_physics/src/source_receipt/composite.rs", EXACT + ": composite_support_norms (lib.rs:4995-4996) and composite_member_maximum (lib.rs:5490-5491) "
     "run under is_exact with a selected source; validate_publication in finalize_composite (`composite` = source_selected && is_exact, lib.rs:2873); "
     "ordinary_physics for an unselected exact case (lib.rs:5726-5729). Priced at their multiplicity; branch W excludes the selected-source entries (ZW_ADD)"),
    ("core/product_physics/src/pressure_material.rs", EXACT + ": resolve_base no longer returns at its !is_exact check and resolve_case runs under is_exact"),
]
EDGE_ZERO_REMOVE_CALLER = [
    ("pressure_runtime.rs:426:build_pressure_case_with_members", 51, EXACT + ": it no longer returns at `!is_exact` (pressure_runtime.rs:435-437); each callee is priced "
     "at its call sites' loop bounds; callees inside the region loop take that loop's bound (0: every region list is empty)"),
]
EDGE_ZERO_REMOVE = [
    ("pressure_runtime.rs:115:validate_profile", "pressure_runtime.rs:356:check_suffixes"),
    ("semantic_contract.rs:480:for_source", "physics_evidence.rs:319:validate_physics_evidence"),
    ("semantic_contract.rs:480:for_source", "semantic_contract.rs:533:forbid_load_reference_evidence"),
]
EDGE_ZERO = EDGE_ZERO + [
    ("source_receipt/composite.rs:406:physical_source", "source_receipt.rs:152:captured_load_state_case",
     "D1.3's load-state clause holds on route E: called only under case_state::is_load_state (0.4.0), false for 0.3.0 (I95)"),
    # I95's credits (RR ruling 2, provisional; re-read on c3cf970ad3: SQ2's written argument)
    ("pressure_runtime.rs:835:finish_source_groups", "pressure_runtime.rs:89:problem",
     "D1.5-exact credit (I95 C-2), re-read at c3cf970ad3 (pressure_runtime.rs:835-920): `groups` is empty (factor groups are made only by add_factor_term "
     "inside build_pressure_case_with_members' region loop), so its two in-loop problem() sites never run; every by_dof list is empty, so exact_sum "
     "returns Ok(+0.0) (exact_rounded_sum over no value: the accumulator's exact zero) and the per-DOF `Err(_) => problem` never runs; every "
     "assembled load is +0.0, so max_rhs = 0, no value is nonzero, maximum_ratio stays 0, screen = 0 and the cancellation problem() never runs"),
    # the mirror (I95 section 2.4): legacy-only branches cannot run with is_exact true
    (rs_any("solve_prepared_load_case", "solve_load_case_observed", f="core/product_physics/src/lib.rs"), "source_receipt.rs:634:exact",
     "a selected exact case takes FinalizedSourceBlockCase::composite_exact (lib.rs:5714-5717); `exact` is the !is_exact arm (lib.rs:5718-5719)"),
    (rs_any("solve_prepared_load_case", "solve_load_case_observed", f="core/product_physics/src/lib.rs"), "source_receipt.rs:717:ordinary",
     "an unselected exact case takes ordinary_physics (lib.rs:5726-5729); `ordinary` is the !is_exact arm (lib.rs:5730-5731)"),
    ("lib.rs:2354:run_linear_static_preview_observed", "source_receipt.rs:836:finalize",
     "`composite` = source_selected && is_exact, so a selected exact invocation takes finalize_composite (lib.rs:2872-2875)"),
    ("lib.rs:2354:run_linear_static_preview_observed", "preview_physics.rs:593:render",
     "`preview` exists only when !is_exact && !source_selected (lib.rs:2693-2695)"),
    ("lib.rs:2354:run_linear_static_preview_observed", "preview_physics.rs:248:evidence",
     "the preview evidence is rendered only from `preview` (lib.rs:2860-2864), None on route E"),
    ("semantic_contract.rs:480:for_source", "preview_physics_evidence.rs:244:validate_preview_physics_evidence",
     "physics-retained-1's G7 validates project(successor), whose base id on route E is physics-1 (RS retained_precision.rs:4720, Route EXACT), so "
     "for_source takes the PHYSICS_ID arm, never the preview arm"),
]
FN_ZERO = [
    ("source_receipt.rs:152:captured_load_state_case", "the 0.4.0 load-state replay; D1.3's load-state clause holds on route E (I95)"),
    ("source_receipt.rs:197:check_load_state_input", "the 0.4.0 load-state replay; D1.3's load-state clause holds on route E (I95)"),
    # B3b-P/B3b-R: the preview-only W1 and reader functions (the mirror of route L's exact-only list)
    ("retained_product.rs:2562:preview_case_evidence", "route E: observables_view's Preview arm only (retained_product.rs:2461)"),
    ("retained_wire.rs:937:explicit_g_origin", "route E: material_basis' Preview arm only (retained_wire.rs:927)"),
    (rs_any("preview_namespace", "legacy_namespace"), "route E: RS g8's `if route.exact {..} else { preview_namespace }` (named legacy_namespace before #1251)"),
    (rs("selected_material"), "route E: RS g8 reaches it only after the exact branch's `continue` (retained_precision.rs:3901-3914), i.e. on the preview route"),
]
EDGE_PER_CALL = EDGE_PER_CALL + [
    ("pressure_runtime.rs:356:check_suffixes", "pressure_runtime.rs:89:problem", "max(n, m, g, cases)",
     "route E (I95): check_suffixes is called on the pipe, node, support and load-case id lists (pressure_runtime.rs:183-202) and reports at most one collision per id"),
    (rs_any("assemble_pressure_case", "build_pressure_case_with_members", f="core/product_physics/src/pressure_runtime.rs"), "pressure_runtime.rs:89:problem", "mats",
     "D1.5-exact credit (I95 C-1), re-read at c3cf970ad3 (pressure_runtime.rs:426-785): of its direct problem() sites only EXACT_PRESSURE_MATERIAL_AMBIGUOUS "
     "can fire, once per selected material (the materials loop); pressure_regions is Some, so EXACT_PRESSURE_REGIONS_REQUIRED never fires; the region loop "
     "is empty, so none of its sites runs; cap_terms and eigen_terms are empty for every DOF, so both exact_sums return Ok(+0.0) and the per-DOF `_ => "
     "problem` never runs; every vector stays +0.0, so EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE never fires (validate_profile's own sites keep their rule)"),
]
ZW_ADD = [
    ("source_receipt/composite.rs:353:composite_support_norms", "called only with selected_source Some (lib.rs:4994-4996): branch X only (T-3 (c): W1 publishes only when no case is source-selected)"),
    ("source_receipt/composite.rs:165:composite_member_maximum", "called only with selected_source Some (lib.rs:5490-5493): branch X only"),
    ("source_receipt/composite.rs:721:composite_exact", "called only with a selected source (lib.rs:5711-5717): branch X only"),
]
LOOP_ZERO_RESTATE = [
    ("exact\\.assembled_operands", EXACT + ". assembled_operands is filled only in finish_source_groups' group loop (pressure_runtime.rs:892), empty here"),
    ("traversal|terminals|\\(dof, pbits, hi, lo, _\\), group\\) in groups", EXACT + ". traversals, terminals and factor groups exist only per region; every region list is empty"),
    ("pressure|region|aggregates", EXACT + ". Every loop this rule zeroes on route E's graph iterates a region list, a region's members or terminals, pipe_states "
     "(filled only per region member) or pressure-thrust aggregates (expansion-joint components, D1.4); each header it matches is listed in zero_hits_E.json"),
]
LOOPS_LAST = [(rx, b, "I95 B3-S (route E, newly reached): " + why) for rx, b, why in json.load(open(os.path.join(H, "i95_new_loops.json")))]
SITE_FROM = [("physics_evidence.rs:%d" % ln, {"from": ["physics_evidence.rs:319:validate_physics_evidence"], "per": "1",
    "why": "route E (I95, as G4's preview_physics_evidence.rs:90/:100): require()/text()/number()/array() (physics_evidence.rs:34-58) allocate only on "
           "failure and every failure returns through `?`; the one swallowing `.ok()` is inside the region loop (empty): <= 1 per validate_physics_evidence call"})
    for ln in (38, 48, 53, 57)]
CENSUS = {"patcher": os.path.join(os.path.dirname(H), "lib", "census_patch.py"), "json": os.path.join(os.path.dirname(H), "data", "census_c3cf.json")}
# ---- route E batch 2: B3b's exact loops (PP W1, RS reader, RE physics-1 base validator) and the identifier audit
NEW.update({
    'for v in assembly[key].as_array().ok_or(" ")?': ("N", "PP retained_product.rs exact_case_evidence: one assembled pressure RHS vector (6n entries; the producer's own evidence)"),
    "for s in sections": ("m", "PP retained_product.rs exact_case_evidence: pipe_sections, checked equal in length to the members (retained_product.rs:2518)"),
    "for (i,x) in sections.iter().enumerate()": ("m", "PP retained_product.rs prepared_sections: pipe_sections, checked equal in length to the members (retained_product.rs:4823)"),
    "for key in PREPARED_SECTION_KEYS": ("4", "PP retained_product.rs prepared_sections: PREPARED_SECTION_KEYS has 4 entries (retained_product.rs:4626)"),
    'for row in array(&source[" "])?': ("Pall", "RE physics_evidence.rs (physics-1's base validator on route E's projection): the published rows, at most the envelope's"),
    'for ex in array(&case[" "])?': ("m", "RE physics_evidence.rs: an exact case's pipe_stress_extrema, one per member"),
    'for m in array(&case[" "])?': ("m", "RE physics_evidence.rs: an exact case's pipe_materials, one per member"),
    'for s in array(&case[" "])?': ("m", "RE physics_evidence.rs: an exact case's pipe_sections, one per member"),
    "for entry in evidence": ("cases", "RS retained_precision.rs g8 (`if route.exact`): contract_evidence.exact_cases, one entry per load case"),
    'for pm in list(&entry[" "])': ("m", "RS retained_precision.rs g8 (`if route.exact`): one exact case's pipe_materials, one per member"),
    'for st in list(&c.source[" "])': ("m", "RS retained_precision.rs g5b_exact_evidence: one case source's section_terms, one per member"),
})
C_ = "core/product_physics/src/source_receipt/composite.rs:"; PM_ = "core/product_physics/src/pressure_material.rs:"
MAT_CLONE = {"bytes": 11264, "source": "COMP"}
ID_AUDIT = ID_AUDIT + [
    (C_ + "97", {"member_id": IN}), (C_ + "322", {"id": IN}),
    (C_ + "349", {"input.load_case.id": IN, "id": IN, "input.load_case.id.len()": NUM_, "id.len()": NUM_}),
    (C_ + "609", {"row.kind": ST}), (C_ + "777", {"m.member_id": IN}),
    (C_ + "781", {"input.load_case.id": IN, "id": IN, "input.load_case.id.len()": NUM_, "id.len()": NUM_}),
    (C_ + "798", {"id": IN, "input.load_case.id": IN}),
    (C_ + "828", {"primary[&support.functional_indices[group*3+c]].id": RS_}),
    (C_ + "925", {"input.load_case.id": IN}), (C_ + "961", {"case_id": IN}), (C_ + "968", {"r.id": RS_}),
    (PM_ + "7", {"stable_suffix(material)": IN, "code": ST}), (PM_ + "15", {"material": IN}),
    (PM_ + "85", {"stable_suffix(&material.id)": IN}), (PM_ + "86", {"material.id": IN}),
    (PM_ + "116", {"material": MAT_CLONE}), (PM_ + "234", {"material": MAT_CLONE}),
    (PM_ + "146", {"id": IN}), (PM_ + "205", {"a.unit": IN}), (PM_ + "212", {"p0.id": IN, "p1.id": IN}), (PM_ + "248", {"material.id": IN}),
]
# C8: source-bound IN128 metadata survives; only owned syntax allocates.
_C8_MODEL = runpy.run_path(os.path.join(H, "..", "base", "member_comparison_model.py"))
C8_MEMBER_STATE = _C8_MODEL["inspect"](os.environ["SQ2_SNAP"])
C8_INPUT_METADATA = [(Resolved(C8_MEMBER_STATE["site"]), {"member.member_id": IN})]
if not C8_MEMBER_STATE["borrowed"]:
    ID_AUDIT = ID_AUDIT + C8_INPUT_METADATA

ID_AUDIT_WHY_E = ("SQ2 G5 route E (RR I95 ruling 5: the 32 newly reached identifier placeholders of b3s_open_items.out.json, at c3cf970ad3 lines): "
    "ids (member, support, load case, material, temperature point) and input units are IN128 (D1.9 TEXT_BYTES); a row kind is STATIC; a result row "
    "id is RES (1024); a MaterialInput clone (pressure_material.rs:116, :234) copies its id, constitutive basis, provenance and four quantity units "
    "(7 strings) and <= 16 temperature points of an id and four units each (80 strings), each <= 128 B: <= 87 * 128 = 11,136 <= 11,264 B")
ID_AUDIT = ID_AUDIT + [
    ("core/reporting/result_export/src/physics_source.rs:777", {"stage": ST}),
    ("core/reporting/result_export/src/physics_source.rs:805", {"stage": ST}),
]
# physics_source.rs checked_material_pair's `stage`: every caller passes a literal ("base", "lower_bracket", "upper_bracket", "selected", "boundary"; physics_source.rs:812-1280): STATIC
