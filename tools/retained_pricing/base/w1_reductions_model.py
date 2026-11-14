"""W1 reductions: source-bound allocation and lifetime premises through stage 2.

The RF source and the reduced source are explicit cohorts. The full raw pins bind
all producer/consumer paths for L1a/b/c, L2, L3 and L5, including the cfg(test)
shadow of the aggregation guard. A half change or a changed lifetime stops;
marker absence never silently selects historical prices. Pins are refreshed only
after the source diff and its allocation/lifetime consequences are reviewed.
Stage2's member wire/diagnostic additions preserve the L3 get_mut Value moves;
their separate positive owners are bound by w1_member_load_model.
"""
# RT-A3 increment 1b D3 (§8.37), after S1: the four reviewed index call sites add no error path, record,
# label, TEXT, reduction owner or lifetime. RTA3B-PS/MR price the new index storage. Live keys below follow
# unchanged source text; historical preimages retain their original identities.
import hashlib
from pathlib import Path

FC = "core/solver/frame_kernel/src/structural/formation_check.rs"
LIB = "core/product_physics/src/lib.rs"
RW = "core/product_physics/src/retained_wire.rs"
NI = "core/solver/nonlinear_integration/src/structural_adapter.rs"
GUARD = "    let combining = preview.is_some() && !model.combinations.is_empty();\n"
SHADOW = ("    #[cfg(test)]\n"
          "    let combining = combining || (l5_tests::former_aggregation() && preview.is_some());\n")
INSERT = ("            if combining {\n                rows_by_base_id\n"
          "                    .entry(base_id)\n                    .or_default()\n"
          "                    .insert(load_case_id.clone(), result.clone());\n            }\n")
# RT-A5 minuend_final (§8.54): the guarded call also passes the run's held operands, an empty map on D1 (D1.4 refuses
# every difference basis, so no combination is `minuend_final` and `solve_held_combinations` is never called).
CALL = ("    let preview_gates = if combining {\n"
        "        preview_physics::append_combination_results(&model, preview.as_ref().expect(\"combining requires preview\"), &rows_by_base_id, &held_operands, &mut results, &mut diagnostics)\n"
        "    } else {\n        Vec::new()\n    };\n")
RF_PINS = {
    "core/product_physics/src/lib.rs": "6c48546bbea452c7f26a3490892d4b441aff25ed7c1c1a9e2b10ac8eb4d91bb4",
    "core/product_physics/src/retained_wire.rs": "7e4326e5b294f6db2ed0995eeca1a8bae3e4ddd487bb0f70436f82426eaa6f3a",
    "core/product_physics/src/certified_publication.rs": "1c6de84c0f7c4cd0799a5b2c7f4bacf3d5d950bb062a816066aaabdd45b489c5",
    "core/product_physics/src/certified_loads.rs": "da7d3decc3a0170da65b38670ec215aaebdb9d8d799e3c76e41879a80589bc73",
    "core/product_physics/src/preview_physics.rs": "61b2fb2f3f9bc3aa9803bf6bf5c95cf5e51c841289ef5c415bf86fe00f33b468",
    "core/solver/frame_kernel/src/structural/formation_check.rs": "0d83c8a61b890d757b96529d255e38c42b35793ae5c998871331278c8cbaa384",
    "core/solver/frame_kernel/src/structural/certified_recovery.rs": "1766a0d94cf02b909b4875cae756b6df5d0bb81489c359281109b7cb248999dc",
    "core/solver/frame_kernel/src/load_ledger.rs": "8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f",
    "core/solver/nonlinear_integration/src/structural_adapter.rs": "bf42cfa532f0f36823fb78128ad175d6bb9a5765e8fd7374599da89df4232a8b",
    "core/solver/straight_pipe/src/lib.rs": "9a4cb59ab52b6f4340fbd74bab9c83847b89691835d0e043a4971b261df452f7"
}
# T4-S5 D3 (epoch; retained-pricing-rules.md §8.8): the reduced cohort is re-bound after its diffs were read.
# FormationSource.loads stays the Cow (L1a); load_closure only adds the TieRodReference arm and the reformation adds the
# connectors' tie rods (P3: none on D1), so L1b/L1c are unchanged; the L2 constructors, the L3 Value moves and the L5
# guard/shadow/insert/call texts (checked below) are unchanged. lib.rs's U4b rows, preview_physics.rs's kind predicates
# and texts, retained_wire.rs's two hashes, certified_publication.rs's tie-rod groups and certified_recovery.rs's
# tie-rod/displacement additions change no reduction's owner or lifetime.
# T4-S6 D3 (§8.8, S6-HS): retained_wire.rs's domain_hash streams its digest (domain_text and DomainPayload deleted);
# no reduction's owner or lifetime changes. T4-S7: retained_wire.rs's domain_hash doc comment only.
# W1 coverage repair D3 (§8.11): retained_wire.rs's selection borrows the certified proof for its final scales;
# no reduction's owner or lifetime changes.
# Pre-0.4 failure unification D3 (§8.14): lib.rs's republication, screen and cause texts change no reduction's owner
# or lifetime (the L5 guard, shadow, insert and call texts are unchanged).
# W-8 topology warnings D3 (§8.16): lib.rs's `mod topology;` and its one call change no reduction's owner or lifetime.
# Silent-zero guard D3 (§8.15): lib.rs's one load-state call (refuse_all_reference_states) changes no reduction's owner
# or lifetime (the L5 guard, shadow, insert and call texts are unchanged).
# Model document 0.7.0 D3 (§8.17): lib.rs's model_settings lines and retained_wire.rs's DEF-E hash constant change no
# reduction's owner or lifetime.
# Branch-warning statement D3 (§8.18): lib.rs's contract_evidence line also calls validation::state_applied_branch_factors
# (no line moves); no reduction's owner or lifetime changes.
# Silent-drop guard D3 (§8.19): lib.rs's closed wire records and the value entry's first call
# (model_fields::unknown_field_refusal) change no reduction's owner or lifetime; preview_physics.rs's
# append_combination_results keeps the L5 callee's inputs (rows_by_base_id, case_rows) and names its withheld rows; the
# L5 guard, shadow, insert and call texts are unchanged.
# W-16 slice a D3 (§8.23): lib.rs's support target, the imposed-motion validation and prescribed map and the primitive
# filter, and preview_physics.rs's shorter LIMITATIONS[1] and deleted pre-parse refusal, change no reduction's owner
# or lifetime; the L5 guard, shadow, insert and call texts are unchanged.
# Unit factor-sum gate D3 (§8.24): preview_physics.rs's gate_reason calls factor_sum_within_unit_tolerance in place
# of its two folds; the L5 callee's inputs and the L5 guard, shadow, insert and call texts are unchanged; no reduction's
# owner or lifetime changes.
# Nested-record closure D3 (§8.25): lib.rs's closed nested wire records and material wire change no reduction's owner or
# lifetime; the L5 guard, shadow, insert and call texts are unchanged.
# Model document 0.8.0 D3 (§8.26): lib.rs's eccentric_offset member and offset-check calls, retained_wire.rs's DEF-E hash
# and preview_physics.rs's test literal change no reduction's owner or lifetime; the L5 texts are unchanged.
# W-16 slices b and c D3 (§8.27): lib.rs's exact combination append (W16B-CB) and equivalent-static record and checks
# (W16C-ES), and certified_loads.rs's generated-id test and exact mass operands, change no reduction's owner or lifetime;
# the L5 guard, shadow, insert and call texts are unchanged.
# §8.30 D3: lib.rs's dense-memory guard and dropped Timoshenko partial-span refusal, FK formation_check.rs's and SP
# lib.rs's Timoshenko partial-span formation change no reduction's owner or lifetime; the L5 texts are unchanged.
# Slice 1a D3 (§8.31): reviewed nonlinear operand producer/reader additions; D1.6 empties the producer
# records, OP1A-RD retains the reader scans and OP1A-OWN prices its borrowed gate vector.
# Model document 0.9.0 D3 (§8.36): lib.rs's new members, closed term wire, lowering and withholding calls, the snubber
# token, retained_wire.rs's DEF-E hash and preview_physics.rs's test literal change no reduction's owner or lifetime;
# the L5 guard, shadow, insert and call texts are unchanged.
# DEL-04-09 dynamics D3 (read 393a11ee7..0468d2ca2): formation_check.rs's only hunks separate frame_matrix's and
# curved_matrix's section leaves (frame_matrix_with/curved_matrix_with and the FrameLeaves/CurvedLeaves exact-lift
# impls, B7a LEAF_SEAM). FormationSource.loads stays the Cow (L1a); load_closure, evaluate and the scaled-force
# children are untouched (L1b, L1c); no Value move, constructor or guard changes (L2, L3, L5). No reduction's owner
# or lifetime changes.
# RT-A4 D3 (§8.46): lib.rs's and preview_physics.rs's combination rewrite keeps the L5 callee's inputs (rows_by_base_id,
# case_rows); the L5 guard, shadow, insert and call texts are unchanged; no reduction's owner or lifetime changes.
REDUCED_PINS = {
    # DEL-04-09 I2a D3 (§8.43): lib.rs only appends `mod modal;`; no reduction's owner or lifetime changes.
    # DEL-04-09 I2b-2 D3 (§8.43): the decision-B seam (LoadCaseSolve.nonlinear_state, the qualification element,
    # capture_nonlinear_state_evidence) changes no reduction's owner or lifetime; the L5 texts are unchanged.
    # I2b-2 layout (§8.43, cc160d28d; Agent 0): ordinary layout of the same hunks (token-equal but for block-wrapped
    # arms and trailing commas); no reduction's owner or lifetime changes, the L5 texts are unchanged.
    # I2b-2 on RT-U4's candidate (§8.48, merge of 41cf15b95): lib.rs carries both of the above; no reduction's owner
    # or lifetime changes, and the L5 texts are unchanged.
    # Operating-case slice 1b (§8.51, merge of 41142f4a3): lib.rs's three 1b items are folded onto existing lines and
    # preview_physics.rs adds the declared arms; no reduction's owner or lifetime changes, the L5 texts are unchanged.
    # RT-A3 S6 D3 (§8.59): lib.rs re-read: no reduction's owner or lifetime changes; L5 texts unchanged.
    # RT-L2 D3: lib.rs, retained_wire.rs and certified_publication.rs re-read (b7a_domain_model's RT-L2 note): no
    # reduction's owner or lifetime changes; the L5 texts are unchanged.
    # RT-L2 final source: lib.rs re-read: a test-module declaration and blocked_envelope's in-place retain of the
    # exception warning; no reduction's owner or lifetime changes, the L5 texts are unchanged.
    # Self-weight units repair (12f46a797, 0051e8a1a): lib.rs re-read: one intensity unit test in `mod tests` restates
    # its evidence literal (test-only); certified_loads.rs adds recorded_mass and its doc comment, called from intensity
    # in place of the closure over mass_inputs. No reduction's owner or lifetime changes; the L5 texts are unchanged.
    # One-way friction normal source (DEL-04-04 REQ-012): lib.rs re-read: the nonlinear friction source admission and the
    # derived normal's publication, inside the loops over nonlinear supports; no reduction's owner or lifetime changes.
    "core/product_physics/src/lib.rs": "5f03bfccaac2d013d4db8bfdb072f3c97eca2538f1a6e3bfdd8c4a3e0ef12e8f",
    "core/product_physics/src/retained_wire.rs": "ca982a664d0c3f30dd510182c402785cb11baee1ef8d567c522351332892e9f7",
    "core/product_physics/src/certified_publication.rs": "088fbf78c977abbcc3c28c7cb38d74bf14b7f7adfac48c21f8010a4020d0cfc2",
    "core/product_physics/src/certified_loads.rs": "6a1e5f45998fe2c6360bbd661bf58afe04cac39a40f106b39559614865e501ac",
    "core/product_physics/src/preview_physics.rs": "19a3134e33454831a961efbf822428650fc23df84ed4d9da07c548c3c607c4c4",
    "core/solver/frame_kernel/src/structural/formation_check.rs": "96b35424bd76fd7733b781241dd328aca91556372876df2a053cbdf2c7377e10",
    "core/solver/frame_kernel/src/structural/certified_recovery.rs": "ef58e6c2ec74bad85b991c344a2127a78145af2a22d8656bbeb699ad097c1e49",
    "core/solver/frame_kernel/src/load_ledger.rs": "8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f",
    "core/solver/nonlinear_integration/src/structural_adapter.rs": "72c93deecae3a0a377a02b08d91ab99d18a44d80d9ddb662b1cf1becd782b093",
    "core/solver/straight_pipe/src/lib.rs": "9a4cb59ab52b6f4340fbd74bab9c83847b89691835d0e043a4971b261df452f7"
}


def inspect(snapshot):
    root = Path(snapshot)
    fc = (root / FC).read_text()
    if "    pub loads: Cow<'a, [LoadPrimitive]>,\n" in fc:
        reduced = True
        expected = REDUCED_PINS
    elif "    pub loads: Vec<LoadPrimitive>,\n" in fc:
        reduced = False
        expected = RF_PINS
    else:
        raise ValueError("W1 reductions L1a: FormationSource.loads changed; re-review")
    # RT-S3 CE fallback (§8.61, CEF-PJ): lib.rs is compared without the exact reviewed CE delta. The branch transfers
    # nothing to a reduction: FormationSource, its Cow loads and the L5 guard, shadow, insert and call are unchanged.
    import runpy
    ce = runpy.run_path(str(Path(__file__).resolve().parent / "ce_fallback_model.py"))
    for relative, digest in expected.items():
        raw = (root / relative).read_bytes()
        if relative == ce["LIB"]:
            raw = ce["s6_bytes"](raw)
        if hashlib.sha256(raw).hexdigest() != digest:
            raise ValueError("W1 reductions source/lifetime pin changed: " + relative)
    if reduced:
        guard((root / LIB).read_text())
    return reduced


def guard(text):
    """L5 and U6-RB share this fail-closed production/test guard warrant."""
    if (text.count(GUARD) != 1 or text.count(SHADOW) != 1 or text.count(INSERT) != 1
            or text.count(CALL) != 1):
        raise ValueError("U6-RB/L5: guarded combination index/call or test shadow changed; re-review")
    return True
