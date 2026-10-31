"""W1 reductions: source-bound allocation and lifetime premises through stage 2.

The RF source and the reduced source are explicit cohorts. The full raw pins bind
all producer/consumer paths for L1a/b/c, L2, L3 and L5, including the cfg(test)
shadow of the aggregation guard. A half change or a changed lifetime stops;
marker absence never silently selects historical prices. Pins are refreshed only
after the source diff and its allocation/lifetime consequences are reviewed.
Stage2's member wire/diagnostic additions preserve the L3 get_mut Value moves;
their separate positive owners are bound by w1_member_load_model.
"""
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
CALL = ("    let preview_gates = if combining {\n"
        "        preview_physics::append_combination_results(&model, preview.as_ref().expect(\"combining requires preview\"), &rows_by_base_id, &mut results, &mut diagnostics)\n"
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
REDUCED_PINS = {
    "core/product_physics/src/lib.rs": "56ea5ca6cee2fbf3163d0d0840f32b13a8e482c805b9eb7b620ac60a4b6b4f13",
    "core/product_physics/src/retained_wire.rs": "b457aff075c67f6eb93000dff2ca0cf4bab43119fe34db1316c78356bf1864b3",
    "core/product_physics/src/certified_publication.rs": "89cdb23e172136b55ff980f7d4a4cd3643e43894fb2a9ad40c92fd3cd4adcf87",
    "core/product_physics/src/certified_loads.rs": "da7d3decc3a0170da65b38670ec215aaebdb9d8d799e3c76e41879a80589bc73",
    "core/product_physics/src/preview_physics.rs": "9f94323ee8d377563fdbde1b5ea31cb400cc924244075ad5a472db201467ae06",
    "core/solver/frame_kernel/src/structural/formation_check.rs": "ac56893815b552074dc4d3acc85ebe4b57579fecd311698729f2bbac225f954a",
    "core/solver/frame_kernel/src/structural/certified_recovery.rs": "8f69bb9bea7c7696b1f78fc985998faa03a507dfc47be13cc6a2b6aa062980c9",
    "core/solver/frame_kernel/src/load_ledger.rs": "8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f",
    "core/solver/nonlinear_integration/src/structural_adapter.rs": "1ca609bda4e56138aab980f5ff4be8f9f664e3300b8bc51f808bc60b45e4d0a3",
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
    for relative, digest in expected.items():
        if hashlib.sha256((root / relative).read_bytes()).hexdigest() != digest:
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
