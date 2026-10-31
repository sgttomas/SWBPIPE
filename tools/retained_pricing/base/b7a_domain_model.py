"""B7a stage-1 ordinary member-load and named-basis positive owner bounds.

The guarded source admits no states/components/partial extents. Existing
positive families remain; these terms widen their storage/multiplicity basis.
"""
import hashlib
import json
import os
import re
import runpy

# B7b D3: the full-file pins are B7b's actual source (retained-pricing-rules.md, B7b section);
# pressure_runtime.rs's later change is inside its `mod tests` only (production prefix identical);
# physics_source.rs adds only the two resolved-branch requires (033626030c) and test include paths;
# ccb19ba0bf adds the readers' zero-corrosion-allowance checks and two guards (retained-pricing-rules.md, B7b-ZA).
# T4-U6/U8 D3 (retained-pricing-rules.md, "T4-U6/U8: actual-source D3"): the changed owner files are re-bound to the
# priced source after their diffs were read; the D-5 and Timoshenko changes add no owner this model prices.
# PP (503180f78a..20c6126a16): lib.rs, certified_loads.rs, retained_product.rs, retained_wire.rs (one hash constant),
# certified_publication.rs, formation_check.rs, certified_recovery.rs, straight_pipe, pressure_runtime.rs and
# structural_adapter.rs change for the Timoshenko admission, the D-5 exact mass and member-identity frame binding:
# frame_members copies are priced by U8-FM; no owner this model prices changes.
# Option A (433fc1f1e4): source_receipt.rs changes only `hash` (U6-SH: a borrowed wrapper through `hash_text`);
# physics_source.rs's actual_materials adds one Err-only `require(.., "ACTUAL_SHEAR_MODEL")` per pipe in its existing
# loop (18 B, inside the 40 B detail class) and a test module.
# Epoch addbdc030e (PP's realized-arc marker): lib.rs's `pipe_sections` moves into `exact_pipe_sections`, which with
# `realized_arc` and `mark_realized_arc` replaces the same expressions in the case solve, `assemble_pressure_case`
# (pressure_runtime.rs, region path only) and T25's `physical_source_built`; the marker is set only on a realized arc
# (none on D1: no components). No owner this model prices changes; the moved keys below are re-keyed.
# Final source 588de796a4 (REG2): lib.rs's withhold_certified_summaries also clears `max_stress_reduced` (PP4: a drop,
# no allocation) and a test; physics_source.rs compares its links and twins by `section_basis::bit_equal` (RD3, RD4:
# a recursive comparison that allocates nothing; `number` errs only on a non-number) and exposes
# `normalized_quantity` to reader_verdict.rs's pressure-1 model binding (unreached from the retained roots, B7b-ZA).
# No owner this model prices changes; no require site is added or removed.
# W1 stage2: the reviewed member capture/wire/reader changes bind the current
# cohort here; their additional owners are positive in w1_member_load_model.
# Current site keys are carried by their unchanged text/function identity.
# Historical BASE_PREIMAGES below remain keyed to the original basis.
# T4-S5 D3 (shared epoch 4: model 0.6.0, U4b, U5b, the W1 not-covered passthrough; retained-pricing-rules.md §8.8):
# the changed owner files are re-bound after their diffs were read. lib.rs adds the U4b station rows
# (u4b_rows::append_station_quantities, priced by U4B-ST), the published corrected displacement (U5B-PD), tie-rod
# reference loads and rows (components only: P3 keeps them empty) and arc maxima (realized arcs only: none on D1);
# retained_product.rs adds the passthrough capture, failed-manifest derived rows and prepared_not_covered (U4B-W1);
# retained_wire.rs moves two definition hashes; certified_publication.rs adds a tie-rod loop (P3: empty);
# exact_admission.rs, model_format.rs, physics_source.rs and semantic_contract.rs change identity/version tokens,
# texts and the -2 tables (the reader statics are re-bound to their -2 paths); pressure_runtime.rs drops the arc
# withheld policy; preview_physics.rs changes its texts and arc branches; FK lib.rs, formation_check.rs,
# certified_recovery.rs and structural_adapter.rs add the tie rod, contribution groups and certify_displacement.
# No admission, cap or census changes: the domain model's widened owners keep their bases.
# T4-S6 D3 (retained-pricing-rules.md §8.8, S6-HS): retained_wire.rs's domain_hash and source_receipt.rs's hash stream
# the checked canonical text into SHA-256 (domain_text, hash_text and both DomainPayload structs are deleted), and
# retained_precision.rs's G1 hashes the publication as the source without its receipt instead of a clone. No owner
# this model prices changes; the hash routes are priced by S6-HS in g4_caps.py and t25_g4.py.
# T4-S7: retained_wire.rs's domain_hash doc comment now cites S6-HS (comment only).
# O4 S5 D3 (retained-pricing-rules.md §8.9): the O4 epoch diffs of the changed owner files were read before re-binding.
# lib.rs: the selected case's reduced maximum takes a second composite_member_maximum call on the reduced section where
# the bases differ (O4-SR, priced in t25/ordinary staging) and SOURCE_MAXIMUM_BASES_DISTINCT is deleted;
# exact_admission.rs: the selected limitation literals are restated (TEXT); retained_product.rs: route E's member
# sidecars (exact_members, distinct_bases, timoshenko, reduced_facts), the reduced preparations and shear associations,
# the ReducedSectionSet, the reduced operand table and second maximum (O4-PS/PA/RS/NC, o4_owners_model);
# retained_wire.rs: the section terms' shear_area/reduced keys and preparation.members[].reduced (O4-WR) and one hash
# constant; source_receipt.rs: test-only items; physics_source.rs and retained_precision.rs: the readers admit either
# shear model and nonzero allowances (reader sets O4-RD, G8 rederivation scalars), MILL_TOLERANCE_DECLINE deleted;
# semantic_contract.rs: one table hash. No owner this model prices changes; the domain's widened owners keep their bases.
# Stack-headroom repair D3 (b7-plan 5a, retained-pricing-rules.md §8.10): read 27e4dfa972;
# freeze_case's closure is split into six phase functions, with unchanged calls, order, errors,
# drops and allocation. Only stack placement changes; no owner this model prices changes.
# W1 coverage repair D3 (retained-pricing-rules.md §8.11): read 3bbfdd5080; retained_wire.rs's selection borrows the
# certified proof and writes body_scales from its final scales (W1R-FS, W1R-L1). No owner this model prices changes.
# Pre-0.4 failure unification D3 (retained-pricing-rules.md §8.14): read b6cc3a792a; lib.rs republishes on both exact
# routes (run_linear_static_preview_captured and permitted_run take the request from the capture after the first
# envelope's fail-closed form drops; PU-R1, PU-X3), screens pre-0.4 selection, records the cause texts (PU-C, PU-CL)
# and states the S2 and missing-row causes (PU-ID, PU-S2); source_receipt.rs makes `requested` pub(super) (doc comment)
# and a test module pub(crate). No owner this model prices changes.
# W-8 topology warnings D3 (retained-pricing-rules.md §8.16): read 86836546e5; lib.rs declares `mod topology` and
# calls topology::append_topology_diagnostics once after the build in run_linear_static_preview_observed_inner (its
# owners: w8_topology_model, W8-TW; its texts: o10_rules W8-*). No owner this model prices changes.
# Silent-zero guard D3 (retained-pricing-rules.md §8.15): read 1cb6b275f6; lib.rs adds one call in
# run_linear_static_preview_observed_inner's load-state branch (refuse_all_reference_states, after
# refuse_resolved_span_strain), unreachable on D1 (B7b-LS, SZ-LS). No owner this model prices changes.
# Model document 0.7.0 D3 (§8.17): lib.rs adds model_settings' module, re-export and PreviewModel/wire members
# before every key below (+7 lines, unchanged text); no owner this model prices changes.
# Branch-warning statement D3 (§8.18): lib.rs's contract_evidence line also calls
# validation::state_applied_branch_factors (no line moves; its loop over model.components is 0 by P2, W8-BF). No owner
# this model prices changes.
# Silent-drop guard D3 (§8.19): lib.rs declares `mod model_fields;`, closes the wire records (declared-and-ignored
# members, closed quantities) and calls model_fields::unknown_field_refusal first in run_linear_static_preview_value_dispatch
# (every key below moves +59 before that call and +63 after it, unchanged text); preview_physics.rs's
# append_combination_results names withheld rows (SD-WH). No owner this model prices changes.
# W-16 slice a D3 (§8.23): lib.rs adds LoadTargetInput's support arm, the imposed-motion validation, the case solve's
# prescribed map and the primitive filter (every key below moves with unchanged text); preview_physics.rs shortens
# LIMITATIONS[1] (formulation_content 3,138 -> 3,069 B) and deletes the pre-parse refusal; retained_product.rs's
# captured-target match and RE retained_precision.rs's and semantic_contract.rs's table hashes follow. No owner this
# model prices changes beyond the formulation allowance.
# Unit factor-sum gate D3 (§8.24): re-read on the merge with main's W-16 slice a (§8.23); preview_physics.rs's
# gate_reason replaces its two binary64 factor folds by one call of load_case_algebra's factor_sum_within_unit_tolerance
# (an exact stack accumulator; no heap, UF-EX) and its tests add a gate test. RE retained_precision.rs's
# preview-physics-retained-1 digest and semantic_contract.rs's two preview table digests move with the tables' restated
# condition (constants). No owner this model prices changes.
# Nested-record closure D3 (§8.25), on the unit factor-sum gate (§8.24): lib.rs closes the nested wire records (node,
# pipe segment and its section, section, support, component, material and their member records: deny_unknown_fields,
# declared-and-ignored zero-size members, closed quantities and vectors), keeps the material record's flattened wire
# with its other keys collected and checked, and adds the new members to seven test literals; no allocation, format or
# `.min(`/`.max(` site. Every lib.rs key below moves +106 with unchanged text. No owner this model prices changes.
# Model document 0.8.0 D3 (§8.26), on the nested-record closure (§8.25): lib.rs gains ComponentGeometryInput's closed eccentric_offset member, its unit normalization in
# normalize_model_units and the two calls of the offset check (W15C-NO: each returns at its first statement on D1);
# the lib.rs keys move with unchanged text (+2 after line 142, +9 after 648, +18 after 2922, +21 after 3376, +36
# after 9714, main's numbering). model_format.rs,
# pressure_runtime.rs (two validate_profile texts format the shared constant, S1B-MV and M08-VZ), retained_wire.rs and RE
# retained_precision.rs (DEF-E's definition hash, the physics-retained-1 table hash), physics_source.rs and
# semantic_contract.rs change version tokens and table hashes; exact_admission.rs, retained_product.rs and
# preview_physics.rs comments or test literals only. retained_memory.rs's admission prefix and family_clauses change
# only comments; model_format.rs's version audit moves 26 -> 28 (its module comment). No owner this model prices changes.
# W-16 slices b and c D3 (§8.27): lib.rs gains `mod exact_combinations;`, the exact route's combination append and
# operand standing (W16B-CB: never on D1), the hoisted connector evidence (the same allocation, made before the
# combination append), the case evidence's equivalent_static record and the generator's exact-route mass, replaced-span,
# Timoshenko and id-collision checks (W16C-ES: never on D1); is_generated_equivalent_static_load takes over
# certified_loads.rs's two generated-id prefix formats (their audits move with them). pressure_runtime.rs's
# validate_profile, exact_admission.rs's load/reference-state limitation and refusal text, RE physics_source.rs's
# companion check and retained_precision.rs's and semantic_contract.rs's table hashes follow. The lib.rs keys move with
# unchanged text (+1 after 151, +28 after 3436, +33 after 7010, +193 after 10814, +198 after 14539, main's numbering).
# W-16 stage 2 D3 (§8.29): lib.rs threads the moved nonlinear grounds (R-b) and the spring bases (R-c) through the case
# solve, the force-scaled publication and the spring rows, and moves `before` above the spring-base map; certified_loads.rs
# adds spring_base_motion (no term on D1, W16S-SG); certified_publication.rs `support` and FK certify_spring_reaction /
# force_scaled_spring_action take each spring's base (stack values); FK formation_check.rs widens NodalProduct's doc
# comment; NI structural_adapter.rs passes the effective gap clearance; RE retained_precision.rs's and semantic_contract.rs's
# table hashes follow the preview tables. No owner this model prices changes. The lib.rs keys move with unchanged text
# (+2 after line 59, +8 after 2035, +24 after 5228, +41 after 5935, +55 after 7235, +106 after 14881; main's numbering).
# §8.30 D3: the lib.rs keys move with unchanged text (+1 after line 104, +23 after 3064, +2 after 10937; main's numbering).
# §8.30 D3 (the three corrections and W-16t): lib.rs adds the nonlinear dense-memory guard (behind a nonlinear
# support, D1.6) and drops the generator's Timoshenko partial-span refusal (W16C-ES); exact_admission.rs swaps that
# refusal's code and text for the shared-terminal junction's; pressure_runtime.rs adds refuse_compensated_shared_terminals;
# FK formation_check.rs and SP lib.rs re-form and bound a Timoshenko partial-span load. Re-pinned after review.
# Slice 1a D3 (§8.31): reviewed nonlinear operand producer/reader additions; D1.6 empties the producer
# records, OP1A-RD retains the reader scans and OP1A-OWN prices its borrowed gate vector.
# RT-A3 increment 1b D3 (§8.36): lib.rs adds the case solve's published-row index (a PublishedScope before the member
# loop, its `begin` per member and a MemberRows passed to two u4b_rows calls; RTA3B-PS, RTA3B-MR). No owner this model
# prices, `.min(`/`.max(` call, reduction owner or lifetime changes; keys after 6126 move +3 and after 6937 +4.
SOURCE_FILES = {'core/product_physics/src/lib.rs': 'b374e7cb228202c16d83bc4044c34b2bb35428a9f77735745e97cdb4928b64c2', 'core/product_physics/src/certified_loads.rs': 'da7d3decc3a0170da65b38670ec215aaebdb9d8d799e3c76e41879a80589bc73', 'core/product_physics/src/retained_product.rs': 'cf3a1240ee380d8bb4e6999c101e224fe740ea7eb50f24a54f531ab987b9908c', 'core/product_physics/src/retained_wire.rs': 'b457aff075c67f6eb93000dff2ca0cf4bab43119fe34db1316c78356bf1864b3', 'core/product_physics/src/certified_publication.rs': '89cdb23e172136b55ff980f7d4a4cd3643e43894fb2a9ad40c92fd3cd4adcf87', 'core/solver/frame_kernel/src/load_ledger.rs': '8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f', 'core/solver/frame_kernel/src/structural/formation_check.rs': 'ac56893815b552074dc4d3acc85ebe4b57579fecd311698729f2bbac225f954a', 'core/solver/frame_kernel/src/structural/certified_recovery.rs': '8f69bb9bea7c7696b1f78fc985998faa03a507dfc47be13cc6a2b6aa062980c9', 'core/loads/primitive_loads/src/lib.rs': '2273116392aa15f020000e1fe7815a2127fcc0548095a2f12dd034d0f7e37802', 'core/solver/straight_pipe/src/lib.rs': '9a4cb59ab52b6f4340fbd74bab9c83847b89691835d0e043a4971b261df452f7', 'core/product_physics/src/preview_physics.rs': '9f94323ee8d377563fdbde1b5ea31cb400cc924244075ad5a472db201467ae06', 'core/reporting/result_export/src/retained_precision.rs': '17f88ee2b5ae7de5fb0c22471f87387a066cbfd21e102b76f75b9bf54c164f25', 'core/product_physics/src/model_format.rs': '7313c52beb6fcd9cbfd774c6ec6a0280eece1aa46eb5a63056078dc310e8bec6', 'core/product_physics/src/pressure_runtime.rs': '8b4f390187e62d77b9f3316e3826bac0ca680b6d4decb9332e43e8a7126dacba', 'core/product_physics/src/source_receipt.rs': '9ee47bc14c5fd2885010f16a9de396f0085335481de75d1df1af2bc6eba241c5', 'core/product_physics/src/exact_admission.rs': '1174fa193e3d4e1d3e388737c7ada2386471177db7e185ded35a39943b161065', 'core/product_physics/src/case_state/mod.rs': '633b46de9fb4fdd5d213efbba1405e65afbc291ad8f9848096596fce27b06fab', 'core/solver/nonlinear_integration/src/structural_adapter.rs': '1ca609bda4e56138aab980f5ff4be8f9f664e3300b8bc51f808bc60b45e4d0a3', 'core/solver/frame_kernel/src/lib.rs': '680583b49167418c07ceb06827f3eab834681af78ab3d5981f92aab519c71ecc', 'core/reporting/result_export/src/semantic_contract.rs': '6417f7d1a889c96f4dcfd6cab21a10a2ca54b2a93e5c5591ce66cb3bf7c2059d', 'core/reporting/result_export/src/physics_source.rs': '1050e77c104a078ebb3b9abd571ac70d183934df9487178f72fe06ecdc41df24'}
CAPS_PIN = "e6759d84e480f0480f381933d4d36d9891f103158c35874fc7dd4acae7d9e71d"
# T4-U6/U8 (O4): family_clauses adds one refusal, a nonzero mill tolerance or corrosion allowance on branch E
# (FamilyFact::SectionAllowance), and namespace_branch refuses the Timoshenko exact contract (FamilyFact::ShearModel).
# Both only narrow admission; the caps, census and late admission bodies are unchanged.
# T4-S5 (epoch): family_clauses and the admission prefix change only in comments (the branch documents read 0.6.0
# for 0.5.0); the admission code, caps, census and late bodies are unchanged.
# O4 S5 D3 (retained-pricing-rules.md §8.9, O4-AD): O4 removes the two T4-U6/U8 narrowings. FamilyFact loses ShearModel
# and SectionAllowance; contract_is admits shear_model euler_bernoulli or timoshenko_annulus_energy_v1 (namespace_branch
# drops its Timoshenko arm), and family_clauses drops the branch-E nonzero mill tolerance/allowance refusal. Route E's
# D1 now holds Timoshenko documents and distinct section bases, whose owners O4's model prices
# (o4_owners_model.py); the caps module, census, late admission bodies and every other clause are byte-identical.
# W-15 slice b D3 (retained-pricing-rules.md §8.22, W15B-NS): family_clauses' node arm refuses a node load whose id
# starts with `load:self-weight:` (GeneratedLoadIdentity), the namespace inspect_applied_self_weight now treats as
# generated on a node target; it only narrows admission. The caps module, census and late admission bodies are
# byte-identical.
# W-16 slice a D3 (retained-pricing-rules.md §8.23, W16-NS): LoadTargetInput gains the Support arm; the census walk
# counts its two strings (support, dof) as it does every load's other texts, and family_clauses' D1.7 refuses it
# (C::Loads, F::LoadTarget) on both routes; it only narrows admission. The caps module and late admission bodies are
# byte-identical.
# W-16 slice b D3 (§8.27, W16B-CB): family_clauses' D1.4 exact clause states that the ordinary exact route evaluates
# combinations; it still refuses them on branch E. Comments only; the admission prefix changes only its comments too.
FAMILY_PIN = "72130bdb1b87bde04fd1746fb33f86bbdaf14c6d288ce3d3e273ea86f72711b4"
ADMISSION_MARKER = "// ---- D-6: the build status --------------------------------------------------"
ADMISSION_PREFIX_PIN = "7368b78e73a6737489507ee9d497d135a14d3a57a5a0b4a6b8a5cc7153935b82"
ADMISSION_LATE_PINS = {"law_order":"b0f9eae8c2d716f14cfda1ff895a65d7833b086383ec34154b501daa1ac344a3","admit":"67334c60ce1a606b5cd73e2715474cfc40150a11def9143752993af4c79904c8","admission":"7627646ef640f23a095bab2f355c7576e1ff79baeb2a9ea44be4550765f60003"}
MATERIALS = 4
OPERAND_WORDS = 26
MEMBER_SOURCE = 268
FAMILY_BYTES = 16

STATIC_PREIMAGE = ('schemas/physics_source_recovery.schema.json', 'schemas/retained_precision_mp_v2.schema.json', 'fixtures/results/retained_precision_prepared_ordinary_v1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json', 'fixtures/results/semantic_contract_v0_2.json', 'fixtures/results/semantic_contract_v0_3_physics_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_source_1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_1.json', 'fixtures/results/semantic_contract_v0_3_physics_source_1.json', 'fixtures/results/semantic_contract_v0_3_source_blocks_1.json', 'schemas/source_block_recovery.schema.json')
# B7b (A4/A5, O-10): physics_contract, load_reference_contract,
# load_reference_source_contract and physics_source_contract are deleted
# with their tables. pressure_rows_contract() (the ordinary pressure-1 table)
# and pressure_source_contract() (the pressure-source-1 table) are distinct
# resident OnceLock<Value> owners and are priced positively in their own right.
# B7b D3 review (b7-plan D2): load_reference::transport_schema()'s
# OnceLock<Result<Value, String>> over the pinned load_reference_state schema
# is a surviving independent process-lifetime owner, priced as its Ok Value.
# TIM-OWN (§8.32): the independent Timoshenko OnceLock stays resident in every phase,
# including a later Euler–Bernoulli invocation; SOURCE_FILES binds its loader.
STATIC_OWNERS = (('core/reporting/result_export/src/semantic_contract.rs:preview_physics_timoshenko_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json'), ('core/reporting/result_export/src/physics_source.rs:schema', 'schemas/physics_source_recovery.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:schema', 'schemas/retained_precision_mp_v2.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:definition', 'fixtures/results/retained_precision_prepared_ordinary_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:table', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_2.json'), ('core/reporting/result_export/src/retained_precision.rs:combination_definition', 'fixtures/results/retained_precision_prepared_combination_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_definition', 'fixtures/results/retained_precision_prepared_exact_v1.json'), ('core/reporting/result_export/src/semantic_contract.rs:physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_table', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:pressure_source_contract', 'fixtures/results/semantic_contract_v0_3_pressure_source_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:pressure_rows_contract', 'fixtures/results/semantic_contract_v0_3_pressure_2.json'), ('core/reporting/result_export/src/load_reference.rs:transport_schema', 'schemas/load_reference_state.schema.json'))


def inspect(snapshot):
    fingerprints = {}
    for file, expected in SOURCE_FILES.items():
        with open(os.path.join(snapshot, file), "rb") as stream:
            raw = stream.read()
        fingerprints[file] = hashlib.sha256(raw).hexdigest()
        if fingerprints[file] != expected:
            raise ValueError("B7a positive-domain owner source changed: " + file)
    with open(os.path.join(snapshot, "core/product_physics/src/retained_memory.rs"), "rb") as stream:
        source = stream.read().decode("utf-8")
    # The whole raw nongenerated prefix binds actual census, capacity and
    # domain enforcement. Mutable registration/generated arrays are excluded.
    if source.count(ADMISSION_MARKER) != 1:
        raise ValueError("B7a positive-domain admission boundary changed")
    if hashlib.sha256(source.split(ADMISSION_MARKER)[0].encode()).hexdigest() != ADMISSION_PREFIX_PIN:
        raise ValueError("B7a positive-domain census/enforcement source changed")
    kd5 = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
    function_body = kd5["function_body"]
    for name, digest in ADMISSION_LATE_PINS.items():
        body, _ = kd5["_indented_function"](source, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError("B7a positive-domain late admission source changed: " + name)
    caps = re.search(r"^pub\(super\) mod caps \{.*?^\}", source, re.S | re.M)
    if caps is None or hashlib.sha256(caps.group().encode()).hexdigest() != CAPS_PIN:
        raise ValueError("B7a positive-domain admission caps changed")
    if hashlib.sha256(function_body(source, "family_clauses").encode()).hexdigest() != FAMILY_PIN:
        raise ValueError("B7a positive-domain family clauses changed")
    return {"full_raw_source_fingerprints": fingerprints,
            "admission_caps_sha256": CAPS_PIN, "family_clauses_sha256": FAMILY_PIN,
            "admission_prefix_sha256": ADMISSION_PREFIX_PIN, "admission_late_sha256": ADMISSION_LATE_PINS,
            "model_sha256": hashlib.sha256(open(__file__, "rb").read()).hexdigest(),
            "formed_terms_per_input": 12, "manifest_operand_words": OPERAND_WORDS,
            "basis_material_count": MATERIALS, "named_basis_count": 3,
            "reader_static_owners": STATIC_OWNERS,
            "resident_json_inputs": list(dict.fromkeys(path for _, path in STATIC_OWNERS))}


def basis_lengths(ident):
    provenance = 161 + 4 * ident
    joined = MATERIALS * provenance + 3 * (MATERIALS - 1)
    record = 85 + ident + joined
    return provenance, joined, record


def primitive_copy(terms):
    """One complete owned FormationSource.loads/ledger primitive copy."""
    from g3lib import V, B, textcap
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    return V("LoadPrimitive", terms) + B(terms * identity)


def manifest_copy(terms):
    """One independent one-entry manifest for each formed term, including children."""
    from g3lib import s, B, pushcap, textcap
    # Child arrays each have one entry: keep the existing 4q+2x upper
    # for q=terms, x=terms, rather than treating them as one shared Vec.
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    return s("LoadClosureEntry") * (6 * terms) + B(
        terms * (identity + 8 * pushcap(OPERAND_WORDS)))


def formed_constructor(terms):
    """Conservative sum over attach's source, expected and supplied staging."""
    from g3lib import V, B, textcap
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    # These constructors are sequential per term. Reserving all terms'
    # source/supplied/temp primitive backings together is an upper, not a
    # claim that they coexist or that move elision removes storage.
    return (V("CertifiedSummand", terms) + V("LoadPrimitive", terms) * 2
            + B(3 * terms * identity))


def custody(named, ident):
    """Invocation-owned exact-growth bases/selections, with no per-case cloning."""
    from g3lib import s, Xc, B
    _, _, record = basis_lengths(ident)
    requested = (Xc("BasisCustody", named)
                 + (Xc("MaterialSelection", MATERIALS)
                    + B(MATERIALS * 5 * ident + 3 * ident + record)) * named)
    # try_reserve_exact(1), not the geometric Vec.push law: each old
    # exact backing can have n-1 elements while the new backing is live.
    moving = (s("BasisCustody") * max(named - 1, 0)
              + s("MaterialSelection") * (MATERIALS - 1))
    return requested, moving


def cached_basis_headers(named, ident):
    from g3lib import V, s, B, pushcap, textcap
    _, _, record = basis_lengths(ident)
    # BASE is retained too; every named state owns its cloned key and
    # formatted record beside its materials/BuiltModel/BasisStiffness.
    count = 1 + named
    requested = V("BasisState", count) + B(
        named * (textcap(6 + ident) + textcap(record)))
    moving = s("BasisState") * (pushcap(count) // 2)
    return requested, moving


def resolver_scratch(members, ident):
    from g3lib import HashReq, V, B, s, textcap
    provenance, joined, record = basis_lengths(ident)
    # HashSet collect sees every pipe in its size_hint, even when only
    # four distinct materials remain. One resolver/build runs at a time.
    requested = (HashReq("&str", members) + V("String", MATERIALS)
                 + B(MATERIALS * textcap(provenance) + textcap(joined)
                     + textcap(6 + ident)))
    moving = (hash_moving("&str", members)
              + s("String") * (pushcap_for_materials() // 2)
              + B(max(textcap(provenance), textcap(joined), textcap(record))))
    return requested, moving


def pushcap_for_materials():
    from g3lib import pushcap
    return pushcap(MATERIALS)


def record_map(cases, ident):
    from g3lib import HashReq, B
    _, _, record = basis_lengths(ident)
    return (HashReq("(String,String)", cases) + B(cases * (ident + record)),
            hash_moving("(String,String)", cases))


def hash_moving(kind, count):
    from g3lib import X, s, B, buckets
    previous = buckets(count) // 2 if buckets(count) > 4 else 0
    return s(kind) * previous + B(previous + 8 + 15) if previous else X()



def formation_children(terms):
    """Original force retains each moved Formation::Exact f64 child."""
    from g3lib import B, pushcap
    return B(8 * terms * pushcap(80))


def formation_constructor():
    """One straight uniform call: conservative sum of distinct construction owners."""
    from g3lib import Xc, B, pushcap
    # All63 expansion_components outputs per single uniform call plus the
    # two initial variable children. Counting them together conservatively
    # covers requested/moving allocation, not just a pair's coexistence.
    # Keeping returnedRows here as well as in resident q is conservative.
    children = 63 + 2
    requested = (Xc("ExactAccumulator", 14)
                 + Xc("Vec", 2 + 8 + 6 + 12 + 12 + 3)
                 + B(children * 8 * pushcap(80))
                 + Xc("Option<FormationRecord>", 12)
                 + Xc("SpannedUniformLocalLoad", 3) + B(3 * 12 * 8))
    # Expansion growth is geometric; at most the old64-f64 backing can
    # remain beside a new128-f64 child. Sequential calls do not multiply it.
    moving = B(8 * (pushcap(80) // 2))
    return requested, moving



TEXT_LOOPS = [

    ("for (local_dof, &global_dof) in dof_map.iter().enumerate()", "12",
     ["lib.rs:add_uniform_element_loads"]),
    ("for ((slot, global), formation) in element_dof_map(pipe.node_i.index, pipe.node_j.index) .iter() .enumerate() .zip(formations)", "12",
     ["lib.rs:add_uniform_element_loads"]),
    ("for load in uniform_loads", "1", ["lib.rs:exact_scaled_intended"]),
    ("iter:variables .iter().map", "2", ["lib.rs:exact_scaled_intended"]),
    ("iter:self.early_sources.iter().filter_map(Option::as_ref).filter_map", "cases",
     ["retained_product.rs:pending_error_bytes"]),
    ("for entry in quality.cases.iter_mut().filter(|entry|entry.basis_ref.ref_id==case)", "cases",
     ["retained_product.rs:failed_manifest"]),
    ("for (index, selector) in selectors.into_iter().enumerate()", "cases",
     ["retained_wire.rs:material_basis"]),
    ("for &i in &used", "4", ["retained_wire.rs:material_basis"]),
    ("for location in FAILED_STATIONS", "3", ["retained_product.rs:failed_manifest"]),
    ("for (index, mut row) in old.into_iter().enumerate()", "Pall", ["preview_physics.rs:render"]),
    ("for term in &self.terms", "l", ["retained_product.rs:check_early_occurrences"]),
    ('for o in list(&b[" "])', "cases", ["retained_precision.rs:g5_requested_mode"]),
    ('for d in list(&source[" "])', "D", ["retained_precision.rs:g5_requested_mode"]),
    ("for (index, (seed, refs)) in pc.ordinary.iter().zip(diagnostic_refs).enumerate()", "cases",
     ["retained_wire.rs:serialize_cases_with"]),
    ("for index in 0..self.cases_seen()", "cases", ["retained_product.rs:settle_ordinary_terminal"]),
    ("for selection in &custody.selections", "4", ["retained_product.rs:validate_selected_case"]),
    ("for count in counts.iter()", "cases", ["retained_product.rs:bind_observations_by_case"]),
    ("for row in &mut rows", "P", ["retained_product.rs:failed_manifest"]),
]


def text_decisions(snapshot, resolved):
    """Site-specific positive classes and O10's existing exact owner checks."""
    inspect(snapshot)
    # Full raw owner guards above bind the functions named by each exact
    # header; O10's standard post-run match check refuses another owner.
    loops = [(header, bound, "B7a source-bound stage-1 owner: " + ", ".join(owners), owners)
             for header, bound, owners in TEXT_LOOPS]
    pp = "core/product_physics/src/"
    re_ = "core/reporting/result_export/src/"
    inp = {"bytes": 128, "source": "IN128"}
    static = {"bytes": 64, "source": "STATIC"}
    audits = [

        # B7b: snapshot keys follow B7b's unchanged lines (B7a 10627.. +5).
        # T4-U6/U8 (PP head 20c6126a16): every key below moved with unchanged text (B7b: lib.rs
        # add_uniform_element_loads 10632.., 1054, 3064, 5299, 5970; certified_loads.rs 233/256; retained_product.rs
        # failed_manifest 4298..; retained_precision.rs 5555/5637/5661). Epoch addbdc030e: the lib.rs keys +36
        # (the realized-arc marker's three helpers after `straight_section_resultants`); final source 588de796a4:
        # every lib.rs key after withhold_certified_summaries +5 (PP4's doc lines and reduced-headline clear).
        # Pre-0.4 failure unification D3 (§8.14): every lib.rs key below moved with unchanged text (+26 to +103: the
        # screen hook, the republication, the fail-closed form and the support-norm cause precede them).
        # W-8 topology warnings D3 (§8.16): every lib.rs key below moved with unchanged text (+1 before the topology
        # call, +2 after it: `mod topology;` and the call).
        # Silent-zero guard D3 (§8.15): the lib.rs keys after 2719 move +1 more (one load-state call), unchanged text.
        # W-15 D3 (§8.21): every lib.rs key below moved again with unchanged text (+3 before W-15's ResultItem record, +49/
        # +60/+64 through the build and case solve, +103 after it); retained_product.rs's rows are edited in place by
        # `component_body: None` with the same audited arguments, so their keys stand.
        # W-7 D3 (§8.20): every key below moved with unchanged text (lib.rs +5/+8/+37: the aggregation's retain, the
        # case solve's member row position and global end block; retained_product.rs +29/+30/+36: the global end
        # completion and the failed manifest's placeholders; retained_precision.rs +10: the global end kind).
        # RT-A3 increment 1 D3 (§8.33): every lib.rs key below 2900 stands; later keys move +2 (the junction warnings'
        # call), +7 (the branch model diagnostics) and +12 (the case solve's branch rows), unchanged text.
        # RT-A3 increment 1b D3 (§8.36): lib.rs keys after 6126 move +3 (the published scope and its `begin`) and after
        # 6937 +4 (the member's MemberRows), unchanged text; earlier keys stand.
        (pp + "lib.rs:12119", {"load_case_id": inp}),
        (pp + "lib.rs:12133", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:12140", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:12145", {"load.load_id": inp}),
        (pp + "lib.rs:12146", {"bend.pipe_id": inp}),
        (pp + "lib.rs:12147", {"bend.component_id": inp}),
        (pp + "lib.rs:12148", {"load_case_id": inp}),
        (pp + "lib.rs:12207", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:12214", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:12219", {"load.load_id": inp}),
        (pp + "lib.rs:12220", {"bend.pipe_id": inp}),
        (pp + "lib.rs:12221", {"bend.component_id": inp}),
        (pp + "lib.rs:12222", {"load_case_id": inp}),
        (pp + "lib.rs:12261", {"stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:12268", {"load.load_id": inp, "load_case_id": inp}),
        # W-16 slice c (§8.27): certified_loads' two generated-id prefix formats move into lib.rs'
        # is_generated_equivalent_static_load (the kind arm and the any-kind arm) with the same input id; both are
        # reached only from an admitted equivalent-static case (W16C-ES: none on D1).
        (pp + "lib.rs:10685", {"load_case.id": inp}),
        (pp + "lib.rs:10686", {"load_case.id": inp}),
        (pp + 'retained_product.rs:4773', {"stable_suffix(id)": inp}),
        (pp + 'retained_product.rs:4774', {"id": inp}),
        (pp + 'retained_product.rs:4782', {"id": inp, "id.len()": inp, "component": static}),
        (pp + 'retained_product.rs:4784', {"id": inp}),
        (pp + 'retained_product.rs:4785', {"component": static}),
        (pp + 'retained_product.rs:4803', {"member.id": inp, "member.id.len()": inp}),
        (pp + 'retained_product.rs:4808', {"member.id": inp}),
        (pp + 'retained_product.rs:4850', {"ordinary.document_kind": static}),
        (pp + 'retained_product.rs:4851', {"ordinary.model_ref": inp}),
        (pp + "lib.rs:1299", {"semantic_contract_id": static}),
        (pp + "lib.rs:3529", {"context.semantic_id()": static}),
        (pp + "lib.rs:6098", {"support.id": inp}),
        (pp + "lib.rs:6847", {"pipe.element_id": inp}),
        (pp + "model_format.rs:28", {"version": static}),  # model document 0.8.0 (§8.26): +2 (the module comment), same text
        # ccb19ba0bf: +17 lines (section_allowances), same text.
        # W1 G8 scope: +6 lines before these unchanged requested-mode bodies.
        # T4-S5 (epoch): every key above and below moved with unchanged text (lib.rs +314/+15/+16/+195/+189;
        # retained_product.rs +66/+76; retained_precision.rs +16). T4-S6 (S6-HS: G1's hash_without): retained_precision.rs
        # +1, unchanged text. O4 S5 (§8.9): every key moved with unchanged text (lib.rs +3/+3/+3/+4, 0 after the case
        # solve; retained_product.rs +167; retained_precision.rs +189).
        (re_ + 'retained_precision.rs:6064', {"mode": static}),
        (re_ + 'retained_precision.rs:6146', {"text(cid)": inp}),
        (re_ + 'retained_precision.rs:6170', {"text(cid)": inp}),
    ]
    lex = [('retained_product.rs:4848', {"class": 64, "why": "source-bound mechanics schema literal"}),
           # T4-S6 (Codex correction, §8.8): the formulation content upper is read from the current literals (3,138 B
           # at T4-S6; 2,914 B was the B7a basis record class, below the epoch's preview-physics limitations).
           ('retained_product.rs:4850', {"class": max(2914, formulation_content(snapshot)[1]), "why": "positive formulation content upper (current literals); actual struct/Vec storage is separate"}),
           ('retained_product.rs:4854', {"class": 2914, "why": "positive summary-owned text upper; inline header and LocatedQuantity children stay separate"})]
    member = runpy.run_path(os.path.join(os.path.dirname(__file__), "w1_member_load_model.py"))
    audits.append(member["case_clone_audit"](snapshot))
    return loops, [(resolved(key), values) for key, values in audits], [(resolved(key), value) for key, value in lex]


_LIT = re.compile(r'"((?:[^"\\]|\\.)*)"')


def _literals(text, start, stop, label):
    if text.count(start) != 1:
        raise ValueError("B7a failed-envelope formulation source changed: " + label)
    a = text.index(start)
    e = text.find(stop, a)
    if e < 0:
        raise ValueError("B7a failed-envelope formulation source changed: " + label)
    return [len(x.encode()) for x in _LIT.findall(text[a:e])]


def formulation_content(snapshot):
    """T4-S6 (Codex correction, §8.8): a failed Preview case's formulation text, read from the current literals as
    (limitation strings, bytes) for the union of its two forms: preview_physics::formulation_basis (its profile id,
    LIMITATIONS and two recovery sentences; "PF9") and lib.rs preview_formulation_basis with
    formulation_basis_for_model_with_source's two appended sentences ("blocked8"). Raw literal bytes bound the decoded
    text. At the B7a basis these were 9 strings / 2,035 B and 8 / 1,649 B."""
    with open(os.path.join(snapshot, "core/product_physics/src/preview_physics.rs"), encoding="utf-8") as stream:
        pp = stream.read()
    with open(os.path.join(snapshot, "core/product_physics/src/lib.rs"), encoding="utf-8") as stream:
        lib = stream.read()
    if pp.count("pub(crate) const LIMITATIONS: [&str; 7] = [") != 1:
        raise ValueError("B7a failed-envelope formulation source changed: LIMITATIONS")
    limits = _literals(pp, "pub(crate) const LIMITATIONS: [&str; 7] = [", "\n];\n", "LIMITATIONS")
    pf = limits + _literals(pp, "pub(crate) fn formulation_basis() -> FormulationBasis {", "\n}\n", "PF")
    blocked = (_literals(lib, "pub fn preview_formulation_basis() -> FormulationBasis {", "\n}\n", "blocked")
               + _literals(lib, "    basis.limitations.extend([\n", "    ]);\n", "appended"))
    if len(limits) != 7 or len(pf) != 10 or len(blocked) != 9:
        raise ValueError("B7a failed-envelope formulation source changed: literal counts")
    return max(len(pf), len(blocked)) - 1, max(sum(pf), sum(blocked))


def failed_envelope_copy(cases, ident, snapshot=None):
    from g3lib import s, V, B
    # A failed Preview case is the only admitted owner. Keep the PF9-limit
    # union rather than the tighter blocked8-limit bytes (2035 and 1649 B at the
    # B7a basis; T4-S6: read from the current literals when the snapshot is given).
    # The complete real envelope atom includes inline basis/Summary headers.
    strings, content = formulation_content(snapshot) if snapshot else (9, 2035)
    return (s("MechanicsEnvelope") + V("String", strings) + B(content)
            + B(2 * (2 * ident + 1024))) * cases

BASE_PREIMAGES = {'loop_bounds_sha256': 'cd0273bc94e521142cc56121171055f9020c167b97caa8efe7fa6ac36c69ba4c', 'loop_rows': {'116': {'re': '^for\\ \\(mi,\\ mb\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', 'bound': '1', 'why': "I104 SQ B1 re-checked, kept (1): RS reader (:747 g3, :3616 g8): body.material_bases: the producer emits one material basis (D1.5's one default basis) whose case_indices name every case (retained_wire.rs material_basis). Was: G4 (T17 precommit reader at b1f80234dc): a receipt body array other than builds: cases, sources, material_bases, calls, groups, ordinary_attempts, product_attempts and legacy_source_work each hold <= 1 entry in D1 (one case, no combinations, one native call and run; retained_wire.rs one_case/one_run, finish)"}, '218': {'re': '^for term in force_terms$|^for force in force_terms \\.iter\\(\\) \\.filter', 'bound': 'l', 'why': 'G4 (R-1, newly reached on the repaired graph): force terms: one per nodal load primitive <= l (D1.7)'}, '313': {'re': 'span\\.start_fraction|uniform_intensities', 'bound': '0', 'why': 'D1.7: no element (distributed) loads, so no load spans (PP lib.rs:9999-10015 at b1f80234dc). G4 (RV84 S-2): bare `intensity` removed (it matched the 3-accumulator zip)'}}, 'fn_zero': {'fn': 'lib.rs:10631:add_uniform_element_loads', 'why': 'D1.7: iterates load_application.element_uniform_loads (lib.rs:3223-3226), empty for nodal-only loads; every text site is inside that loop'}, 'edge_zero': {'caller': 'formation_guard.rs:297:load_row_finding', 'callee': 'formation_guard.rs:216:decide_row', 'why': 'D1.7 nodal-only loads: every formation slot is None, so load_row_finding returns at its has_formation_records() check (formation_guard.rs:303-305; FK load_ledger.rs:307-309; I54 correction ADDENDUM)'}, 'record_audit': {'bytes': 600, 'source': 'COMP'}, 'provenance_join': {'re': '^provenance_records$', 'bytes': '8*(128+200)+8*3', 'why': 'composite_text provenance_join'}, 'copy_lex': {'class': 'result_id', 'why': "adapter copy(): identifiers, units, entity refs, result ids and the basis text (retained_product.rs:247-3452), each <= result_id (G6 repair, RV87 SF-2: re-keyed from b1f80234dc's line 2963 to copy()'s push_str at 1e323058f3, retained_product.rs:3125)"}}

def prepare(snapshot, base, output, inherited):
    """Replay positive source-premise changes; never edit the historical base."""
    proof = inspect(snapshot)
    loop_path = os.path.join(base, "loop_bounds.g4.json")
    raw = open(loop_path, "rb").read()
    if hashlib.sha256(raw).hexdigest() != BASE_PREIMAGES["loop_bounds_sha256"]:
        raise ValueError("B7a positive-domain loop-rule base changed")
    loops = json.loads(raw)
    alterations = []
    for index, bound, why in ((116, "cases", "one material basis per distinct requested selector"),
                              (218, "12*l", "at most twelve formed terms per admitted full-span uniform input"),
                              (313, "l", "at most one full-span uniform contribution per admitted input")):
        expected = BASE_PREIMAGES["loop_rows"][str(index)]
        assert loops["loops"][index] == expected
        changed = dict(expected, bound=bound, why="B7a stage 1: " + why)
        loops["loops"][index] = changed
        alterations.append({"family": "loops", "index": index, "before": expected, "after": changed})
    for family, index in (("fn_zero", 19), ("edge_zero", 0)):
        expected = BASE_PREIMAGES[family]
        if loops[family][index] != expected or loops[family].count(expected) != 1:
            raise ValueError("B7a positive-domain exact zero preimage changed")
        loops[family].pop(index)
        alterations.append({"family": family, "index": index, "before": expected,
                            "why": "already-admitted full-span uniform loads execute this owner"})
    text_path = inherited.get("text_args.g4.json", os.path.join(base, "text_args.g4.json"))
    text_raw = open(text_path, "rb").read()
    text = json.loads(text_raw)
    # W1 extracts the submitted-source loop into solve_sources_projected.
    # Keep the complete historical audit preimage, then re-install it at the
    # actual clone statement after line carry. The existing sources.iter()
    # rule still bounds the moved loop by att; no owner or visit is deleted.
    member = runpy.run_path(os.path.join(os.path.dirname(__file__), "w1_member_load_model.py"))
    member["inspect"](snapshot)
    clone_key = "core/solver/frame_kernel/src/structural/retained/adaptive.rs:5082"
    clone_audit = {"source": {"bytes": 128, "source": "STATIC"}}
    if text["id_audit"].get(clone_key) != clone_audit:
        raise ValueError("W1 stage2: historical source-clone audit preimage changed")
    text["id_audit"].pop(clone_key)
    alterations.append({"family": "id_audit", "key": clone_key,
                        "after": member["case_clone_audit"](snapshot)[0],
                        "why": "same source clone in the extracted batch body; full audit retained"})
    key = "core/product_physics/src/lib.rs:10230"
    if text["id_audit"][key]["basis_record"] != BASE_PREIMAGES["record_audit"]:
        raise ValueError("B7a positive-domain record audit preimage changed")
    text["id_audit"][key]["basis_record"] = {"bytes": 2914, "source": "COMP"}
    old_join = BASE_PREIMAGES["provenance_join"]
    if text["join_rules"].count(old_join) != 1:
        raise ValueError("B7a positive-domain provenance join preimage changed")
    new_join = dict(old_join, bytes="2701", why="B7a named point: four 673-byte records plus three 3-byte separators")
    text["join_rules"][text["join_rules"].index(old_join)] = new_join
    lex = "retained_product.rs:3775"
    if text["lex_site_size"][lex] != BASE_PREIMAGES["copy_lex"]:
        raise ValueError("B7a positive-domain adapter-copy preimage changed")
    text["lex_site_size"][lex] = {"class": 2914, "why": "B7a named basis record; other original copied identifiers remain bounded"}
    text["args"].insert(0, {"re": r"^(basis_record|metadata\.basis|basis_label)$", "max": 2914,
                            "why": "B7a named point aggregate record: 85+128+4*(161+4*128)+9 bytes"})
    alterations += [{"family": "id_audit", "key": key, "field": "basis_record", "bytes": 2914},
                    {"family": "join_rules", "before": old_join, "after": new_join},
                    {"family": "lex_site_size", "key": lex, "class": 2914},
                    {"family": "args_first", "rule": text["args"][0]}]
    os.makedirs(output, exist_ok=True)
    paths = {}
    for name, data in (("loop_bounds.g4.json", loops), ("text_args.g4.json", text)):
        path = os.path.join(output, name)
        with open(path, "w", encoding="utf-8") as stream:
            json.dump(data, stream, indent=1)
        paths[name] = path
    proof.update({"base_loop_sha256": hashlib.sha256(raw).hexdigest(),
                  "input_text_rules_sha256": hashlib.sha256(text_raw).hexdigest(),
                  "prepared_files": {name: hashlib.sha256(open(path, "rb").read()).hexdigest()
                                     for name, path in paths.items()}, "alterations": alterations})
    return paths, proof

def reader_static_files(snapshot, historical):
    """Closed actual loader ownership, including two independently cached duplicates."""
    inspect(snapshot)
    if tuple(historical) != STATIC_PREIMAGE:
        raise ValueError("B7a historical reader static inventory changed")
    # TIM-OWN: twelve distinct inputs, fourteen actual OnceLock allocations
    # (thirteen OnceLock<Value> and transport_schema's OnceLock<Result<Value, String>>).
    # Never deduplicate by path or skip an absent file: g4's unchanged
    # static_tree parser must account every listed allocation positively.
    return [path for _, path in STATIC_OWNERS]
