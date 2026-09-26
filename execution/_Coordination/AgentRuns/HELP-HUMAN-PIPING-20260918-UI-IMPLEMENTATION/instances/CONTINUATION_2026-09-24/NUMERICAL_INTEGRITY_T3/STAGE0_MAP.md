# T3 stage 0 — map of numerical integrity, precision and scale

WORKING_ITEMS manager for T3, 2026-09-26. Read-only on product source; this record and the stage-1 plan are the only writes. No build, test or probe was run for this map.

- **Basis.** T3 branch `codex/piping-numerical-integrity-20260926`, created from main `c61a540ea`. That main contains PR905 (`23aad15d6`) and T0R/PR952 (`82b43f9bd`). The only Piping product change after `82b43f9bd` is the opt-in WebKit Playwright lane (two desktop e2e files), so the product source is T0R's merged state.
- **T1 basis.** Branch `codex/piping-load-states-20260925`, candidate `f3270ea79`, merge base with main `82b43f9bd`. T1's own changes are taken from the three-dot diff `origin/main...origin/codex/piping-load-states-20260925`. The two-dot diff that ROOT's brief names also lists the two WebKit files as reverse changes, because they landed on main after T1's merge base; they are not T1 edits.
- **Paths.** `P/` is `projects/chirality-piping/`. `PP` is `P/core/product_physics/src/lib.rs`. Line numbers are at `c61a540ea` and will drift.
- **Records read.** Root `AGENTS.md`; `agents/AGENT_WORKING_ITEMS.md`; the work graph's current route, T3, COR-NUMERICS and the M03/M06/M32/M34 rows; `T0_REASSESSMENT/{RETURN,INDEPENDENT_CHECK}.md`; `CORRECTNESS_DESIGN/{NUMERICAL_IMPLEMENTATION,M03_RESIDUAL_SUCCESSOR_ADOPTION,NUMERICAL_REFERENCE}.md` and `CONTRIBUTION_PRECISION/{CONTRACT,RETURN,INDEPENDENT_REFUTATION}.md`; `DEFAULT_ROUTE_DESIGN/{DESIGN,ROOT_SELECTION,ROOT_RULINGS}.md`; `ENGINE_INTEGRATION/RETURN.md` (PHYS-R4, conversion scope); the solver-findings assessment rows ELEM-4, ELEM-13, VER-01, VER-07, STR-13 and SUP-17; `SCIENTIFIC_TRANSPORT_FOUNDATION/RETURN.md`; `P/validation/benchmarks/numerical_integrity/{README,COVERAGE}.md`; and from the T1 branch `LOAD_STATE_IMPLEMENTATION/{T1_WAVE1_RULINGS,CHECKPOINT_5,CP4_WIRE_ADDENDUM}.md` and `T1_WP1_JOINED_READERS/RETURN.md`.

These records are the accepted or historical basis. None of them is evidence that current code is correct; the facts below cite the source at `c61a540ea`.

## 1. Summary

No T3 group can close yet. The integrity policy, full-precision publication and the bounded exact recovery are merged. What remains is general accuracy, range and scale, plus a set of standing and binding items that other tranches routed here.

| Item | Merged now | Open under the closure rule | Needs a product-semantics choice? |
|---|---|---|---|
| M03 | M03-INTEGRITY-v1 gate in both modes; M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1 residual; mapped mechanism diagnostics; Sensitive results withheld from Current; bounded strict-gap law | General accuracy (N05-class systems outside the bounded method); PHYS-R4 range; rigid-null witness for user-matrix and curved bodies; SUP-17 wording; VP-ROBUST | No (technical, ROOT) |
| M32 | Nothing; the sparse factorization is sparse, but assembly, reduction and reactions are dense | Direct sparse assembly, reduction and reactions, with parity and measured memory | No |
| M34 | No publication quantization; float round-trip; row-normalized residual; display-only rounding | Range (subnormal), cross-unit display representability, scientific transport adoption, a dimension-aware comparison policy for new cases | Possibly one (§5.3) |
| N05 ordinary accuracy | Correct only through bounded retained-source recovery | The ordinary (general) method still misses 1e-9 | No |
| General retained-source recovery | Order ≤ 2 free blocks, axis-aligned members, nodal loads only | Larger, skewed and weakly coupled systems; other load producers | No |
| Composite `SOURCE_BLOCKS_FINALIZATION_FAILED` | Fails closed with `Err` | The whole invocation loses its results | No; T1's SF-1 fallback is the precedent |
| Joined `load-reference-source-1` eligibility and binding | Readers accept and display; standing is always `needs_recompute` | Reader-side re-derivation; a canonical-document or binding route | No |
| Selected-UNAVAILABLE alignment | Joined reader refuses; physics-source-1 accepts | One rule for both | No, but ROOT picks the rule |
| T0R carries (found in this map) | Readers mark mixed source-blocks-1 `needs_recompute`; rule binding to its abs-sum summary is refused | Re-home source-blocks-1 semantics; the abs-sum summary | No |

## 2. Per-item map

### 2.1 M03 — mechanisms and solve integrity

**Requirement** (M-03, ELEM-4, VER-01, SUP-17): scaled stability and pivot diagnostics; per-component rigid-mode detection; positive definiteness distinct from LU pivot sign; a blocking original-system residual gate with node/DOF mapping in both modes; validation on skew mechanisms, disconnected parts, compliant legitimate systems and unit changes; no raw global threshold.

**Merged on main:**
- M03-INTEGRITY-v1 in `P/core/solver/frame_kernel/src/structural.rs` and `P/core/solver/sparse_direct/src/structural.rs`: contribution audit, symmetry evidence, positive Cholesky/LDL with a negative-energy witness, original-equation componentwise residual with bounded refinement. Both modes dispatch through `AssemblyEvidence::solve` (`P/core/solver/nonlinear_integration/src/structural_adapter.rs`).
- The successor residual M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1 (target `64 γ(m)`), with historical DEC-046 zero policies preserved.
- Mechanism, negative-energy, assembly-unresolved and sensitive outcomes publish diagnostics with a global DOF map and per-case `numerical_quality`. The legacy unscaled solves no longer select a solution.
- Sensitive results are withheld from Current, rule and qualified export in Rust, Python and TS (`numerical_use_standing*`).
- The bounded strict-gap law (pure-gap straight-frame systems, free blocks of order ≤ 2) is merged in `nonlinear_integration`; mixed gap, one-way, lift-off and friction systems are explicitly unqualified.

**Open (T3 unless marked):**
1. **Accuracy for general systems.** The ordinary binary64 method cannot preserve a small stabilizing contribution added to a large one (N05: `k/a ≈ 4.6e-11`). Main contains this by marking it Sensitive or rejected, and by replacing it with retained-source recovery only inside that method's scope (§2.5). T0 records the ordinary misses as dense −2.957918e-6 and sparse +1.698735e-6 against the unchanged 1e-9 criterion.
2. **PHYS-R4 range.** A subnormal transformation-error allowance (`7.53e-321` at `[UY,UY]`) is refused in `structural::transform_roundoff` before the solve. The public fixture therefore returns `NUMERICAL_INTEGRITY_UNRESOLVED`; the publication arithmetic repair is verified only by direct calls.
3. **Rigid-null witness** is unqualified for bodies containing user-matrix or curved elements (`structural_adapter.rs:264`).
4. **SUP-17 wording.** `PP:1362` still says "missing global rigid-body DOF classes" for directly restrained DOF classes. T0 notes that this alone keeps M03 open.
5. **VP-ROBUST coverage** does not exist: rotations (including `(1,2,2)/3`), unit sets, origin shifts, relabelling, larger, skewed and weakly coupled systems. `P/validation/benchmarks/numerical_integrity/COVERAGE.md` lists these as not yet executable.
6. Nonlinear mixed recovery (`NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`) and general mixed-device gap classification: **T5** (M06), not T3.

**Evidence:** frame_kernel 86 tests, sparse_direct 25, product library 268 plus 1 ignored, integration 62 (T0 logs at `eb56e1083`); the independent `source_block_recovery.rs` N05/N06 × sign × rotation × mode test; the native N05/N06 witnesses from PR905. Most M03 unit tests are implementer-authored. The frozen N01–N09, R01–R07 and NP-A–NP-D references are in `P/validation/benchmarks/numerical_integrity/fixtures.json`.

### 2.2 M32 — direct sparse assembly

**Requirement** (M-32, ELEM-13): one sparse assembly, reduction and reaction path, with bounded scrutiny and fallback behaviour; sparse/dense parity on small controls, several modulus bases and nonlinear paths; measured peak memory and runtime on sealed large models; the fallback is kept.

**On main (unchanged since the assessment):** `assemble_global_stiffness_with_user_elements` allocates `vec![vec![0.0; n]; n]` (`frame_kernel/src/lib.rs:762-768`). `reduce_system` builds a dense reduced matrix (`:867`). The facade assembles the dense global matrix at `PP:1378`, again per modulus basis at `PP:1452`, and reduces densely at `PP:1855`. The sparse backend receives entries (`solve_symmetric_system_from_entries`) after that dense assembly. The exact helper caps systems at 256 DOFs (`exact_boundary.rs:292`).

**Open:** everything in the requirement. No memory observation exists; `P/core/solver/performance_harness` is the natural home.

### 2.3 M34 — stored precision and dimension-aware accuracy

**Requirement** (M-34, STR-13, VER-07): full precision stored and rounded only for display; precision carried through export, rule inputs and persistence; dimension-aware residual and comparison rules with near-zero floors; scale and unit invariance tests; protected criteria kept unless an evidenced change is approved.

**Merged on main:** `round6` is test-only; envelopes declare `value_representation: finite_binary64` and `publication_quantization: none`; `serde_json` uses `float_roundtrip`; the residual is row-normalized; the desktop rounds only at presentation (`apps/desktop/src/services/unitConversion.ts`).

**Open:**
1. Range: the PHYS-R4 subnormal refusal (shared with M03).
2. Cross-unit display: `QuantityReadout → displayQuantityService → operation_applier/display_units → units::convert_for_dimension` checks only finiteness; a subnormal or zero projection has no relative representability bound (`ENGINE_INTEGRATION/RETURN.md`, "Conversion scope").
3. Scientific transport: the PR901 profile `openpipestress_jcs_binary64_v1` is standalone; no product, runner or desktop path consumes it. T1 also notes that the canonical carrier `openpipestress_jcs_ijson_v1` refuses integral magnitudes above 2^53 − 1 at materialization, and calls a change there a transport-profile matter for T3 or T6.
4. Comparison policy: the protected benchmark predicate is absolute 1e-9 for most fixtures (`P/validation/benchmarks/mechanics/src/lib.rs:1437-1441`). A dimension-aware policy for new cases is open (T3, with T9).
5. T6, not T3: persistence of manifest bodies (`HISTORICAL_INPUT_MANIFEST_MISSING`) and regeneration of the precision-1 fixture pair. Held separately: the DEL-10-05 witness inputs at `product_physics` 0.1.0.

**Evidence:** `precision_signed_subquantum_torsion_both_modes_matches_independent_annulus`, `precision_same_unit_raw_quantity_bits_and_numeric_strings_roundtrip`, `precision_nonfinite_values_are_rejected_not_serialized_as_null`, the mixed-unit normalization tests and the source-recovery scaled-norm test (T0).

### 2.4 N05 ordinary accuracy

N05 is a stable 2 m torsion member with a soft root spring `k = 1e-4 N·m/rad` and tip torque `1e-8 N·m`. The ordinary method misses the unchanged 1e-9 criterion because `a + k` is rounded when assembled (`a = 687800π`): even an exact solve of the stored matrix is off by about 1.7e-6. N06 (`k = 1e-12`) loses `k` altogether. This is an assembly-resolution defect, not a physical mechanism.

The selected design basis is `CONTRIBUTION_PRECISION/CONTRACT.md`: preserve source contributions, assemble, reduce, factor and solve at higher precision (adaptive 128/256/512 bits, 1024-bit verification ceiling), and recover actions from the retained state before rounding once to binary64. The relative-coordinate and double-double alternatives are recorded there. That design was reviewed and refuted as feasible, but it was **not implemented**. What was implemented is the bounded retained-source method (§2.5). The ordinary path remains contained, not repaired. The native N05/N06 witnesses pass only through retained-source recovery.

### 2.5 General retained-source recovery

**Merged:** `source-blocks-1` (legacy route) and `physics-source-1` (exact route) use exact recovery on complete retained-source free blocks. The scope limits on main are:
- every free connected component has order ≤ 2 (`exact_boundary.rs:4, 419-420`);
- at most 256 DOFs (`exact_boundary.rs:292`);
- every member transform is a signed permutation, so only axis-aligned members qualify (`source_recovery.rs:300-320`);
- straight frames only: no nonlinear or contact supports, components, curved bends, user matrices or releases (`:417-437`);
- nodal loads only: no element, thermal, pressure-thrust, imposed-displacement, equivalent-static or constant-effort loads, and an explicitly empty pressure-region list on the exact route (`:438-471`);
- no combinations; a captured invocation is required;
- work budgets of 4M per case for source-blocks-1 and 8M per case / 64M per invocation for physics-source-1.

**Open:** larger (order > 2), skewed and weakly coupled systems, and the excluded load producers. General recovery is the same problem as §2.4; one method should serve both.

### 2.6 The composite `SOURCE_BLOCKS_FINALIZATION_FAILED` finding

`run_linear_static_preview_value_with_mode` returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` when the producer claims `source-blocks-1` or `physics-source-1` but has no `source_block_recovery` (`PP:1249-1253`). The recorded trigger is T1's review probe P12 (and P3): one case selects retained-source recovery while another case in the same invocation is Sensitive with its own recovery unavailable. The whole invocation then loses its results, including the case that was recovered.

T1's SF-1 fallback (`LOAD_STATE_IMPLEMENTATION/CP4_WIRE_ADDENDUM.md` §1.2) repairs the equivalent 0.4.0 case: it republishes the invocation on the ordinary route in a fresh ledger, declines every successful attempt with "invocation join withheld", and each case keeps its own attempt facts and ordinary numerical standing. T1 deliberately leaves the pre-0.4 outcome unchanged and pins it with the characterization test `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback`. On T1's branch the same `Err` line also names `LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID`.

**Open:** the pre-0.4 fix. It sits in `PP` and `source_receipt*`, all of which T1 edits, so it waits for T1's merge. The T1 characterization test must then be replaced by a behavioural test, with the replacement recorded, not deleted silently.

### 2.7 Items T1 routed to T3

1. **Joined eligibility** (T1 ruling §7). Joined `load-reference-source-1` results stay `needs_recompute`. physics-source-1 grants `numerically_eligible` only with a captured invocation, from which the reader re-derives E, ν and α (`actual_materials`). For 0.4.0 the reader would have to re-derive the resolved case (thermal laws, fit, material selection) in Rust and Python, captured like physics-source-1. T1 classes this as retained-source recovery qualification.
2. **Binding route** (T1 ruling §11). `result_envelope_binding` builds a canonical document only for a numerically eligible result, so a joined solve yields no canonical document and no `QualifiedPreviewEvidence` (`CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`). The committed joined carriers come from the test-side `derive_document`. A binding route belongs with eligibility.
3. **Selected-UNAVAILABLE alignment** (T1 WP1 N-2). The joined reader's check S13 refuses an envelope whose selected case also carries a `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic. physics-source-1 accepts the same fact. One rule should govern both. Aligning in either direction changes a reader of a family that T1 or PR905 has already qualified, so ROOT chooses the direction from the design's evidence.
4. Desktop export of both load-reference identities is **T6** (T1 ruling §12), not T3.

### 2.8 T0R carries found in this map

T0R's selection routes two source-blocks-1 items to T3 ("Re-homing source-blocks-1 goes to T3"; DESIGN §5.7 and §11, rulings R-1 and R-2). These items are not in the graph's T3 row, and ROOT should confirm them:
- A selected-plus-ordinary source-blocks-1 envelope is read as `needs_recompute` with `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS` "until T3", because its ordinary cases keep precision-1 semantics (norm-only reactions, abs-sum summary).
- An all-selected source-blocks-1 envelope stays Current, but its abs-sum summary is refused for rule binding (`RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`) "until T3". The user-facing texts `N_SB` and `N_SB_MIXED` (`apps/desktop/src/features/results/knownSemanticLimitations.ts:20-21`) promise T3.

These are the M14 remainder that T0R names for T3. Re-homing means publishing source-blocks-1 cases under preview-physics-1 semantics (signed six-component reactions and the circular maximum) or retiring the identity for fresh solves in favour of a successor.

### 2.9 VP-ORACLES and VP-ROBUST

- **VP-ORACLES**, "near-zero and finite-accuracy budgets": every frozen reference used by T3 must state its own accuracy and, for each zero-valued expectation, a derived zero scale. T0R already uses this form (`|obs − exp| ≤ 1e-9 · max(|exp|, scale)`). No new tolerance is implied.
- **VP-ROBUST**: true coordinate rotations, unit changes, relabelling and refinement; larger, skewed and weakly coupled systems; thin sections and extreme admitted scales; memory and runtime observations; seeded faults. Nothing exists yet; the graph says bounded order-2 signed-permutation recovery is not general coverage.

## 3. Files T3 would touch, and the T1 overlap

"T1" means the file is changed in `origin/main...origin/codex/piping-load-states-20260925` at `f3270ea79`.

| Area | Files | T1? | T3 items |
|---|---|---|---|
| Kernel | `P/core/solver/frame_kernel/src/{lib.rs, structural.rs, rigid_body.rs, structural/exact_boundary.rs, structural/exact_boundary/functionals.rs}` | disjoint | M03 general accuracy, range scaling, M32 sparse assembly API |
| Kernel | `P/core/solver/sparse_direct/src/{lib.rs, structural.rs}` | disjoint | M32 |
| Kernel | `P/core/solver/nonlinear_integration/src/{structural_adapter.rs, lib.rs, product_equilibrium.rs}` | disjoint | rigid-null witness, new solve dispatch |
| Harness | `P/core/solver/performance_harness/**` | disjoint | M32 memory and runtime observations |
| Facade | `PP` | **T1** (hunks throughout the entry, case loop, `solve_load_case`, `solve_preview_reduced_system`, envelope) | all product-side wiring; composite finalization; SUP-17 wording |
| Facade | `P/core/product_physics/src/{source_recovery.rs, source_receipt.rs, source_receipt/{composite,source,rows}.rs, pressure_runtime.rs}` | **T1** | general recovery; composite finalization |
| Facade | `P/core/product_physics/src/{membrane_publication_range.rs, preview_physics.rs, validation.rs}` | disjoint | PHYS-R4 controls; source-blocks re-homing |
| Facade tests | `P/core/product_physics/tests/{source_block_recovery.rs, physics_source_runtime.rs, pressure_membrane_range.rs}`; new T3 test files | disjoint | runtime tests |
| Readers | `P/core/reporting/result_export/src/{semantic_contract.rs, derivative.rs, lib.rs}` | **T1** | standing, identities |
| Readers | `P/core/reporting/result_export/src/{source_blocks.rs, physics_source.rs}` | disjoint | selected-UNAVAILABLE alignment, re-homing |
| Readers | `P/core/reporting/result_export/src/load_reference_source.rs` | **T1** (new file) | joined eligibility |
| Headless | `P/core/runner/headless/src/result_envelope_binding.rs` | disjoint | joined binding route |
| Headless | `P/core/runner/headless/src/lib.rs` | **T1** | only if a route is added |
| Python | `P/core/analysis_runs/{source_blocks.py, physics_source.py, __init__.py}` | disjoint | alignment, re-homing |
| Python | `P/core/analysis_runs/{compatibility.py, load_reference_source.py}` | **T1** | standing, joined eligibility |
| TS | `apps/desktop/src/features/results/{sourceBlockRecovery.ts, physicsSourceRecovery.ts}`, `apps/desktop/src/services/{unitConversion.ts, displayQuantityService.ts, ruleCheckService.ts}` | disjoint | re-homing; display representability |
| TS | `apps/desktop/src/features/results/{numericalResultQuality.ts, loadReferenceSourceEvidence.ts, knownSemanticLimitations.ts}`, `apps/desktop/src/types.ts` | **T1** | standing parity; joined eligibility; re-homing notices (`N_SB`, `N_SB_MIXED`). Correction after D2: `knownSemanticLimitations.ts` and its test are T1-touched; the first version of this map listed them as disjoint |
| Native | `apps/desktop/src-tauri/src/lib.rs` | **T1** | binding, rule gate, only if needed |
| Units, transport | `P/core/units/**`, `P/core/serialization/canonical_json/**`, `P/tools/serialization/**` | disjoint | scientific profile adoption; display range |
| Schemas | `P/schemas/results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json`, `stress_neutral_export.v0.3.schema.json` | **T1** | any new evidence field |
| Validation | `P/validation/benchmarks/numerical_integrity/**`, `P/validation/benchmarks/mechanics/**`, `P/validation/qualification/**` (except T1's `fixtures/load_reference`), `P/validation/hand_calcs/**` | disjoint | VP-ORACLES and VP-ROBUST |
| Python tests | `P/tests/{test_source_blocks_validation.py, test_physics_consumer_contract.py, test_precision_consumer_contract.py}` | disjoint | reader parity |
| Python tests | `P/tests/{test_preview_physics_consumer_contract.py, test_source_block_schema_contract.py}` | **T1** | standing pins |

**Consequences for sequencing:**
- Before T1 merges, T3 may implement only in the disjoint rows. The kernel crates, the performance harness, the validation assets, the source-blocks and physics-source readers, the units and transport crates and the display services are all disjoint. That is enough for a kernel-first slice (general accuracy and sparse assembly APIs with kernel-level tests), the VP-ORACLES and VP-ROBUST assets, and the display-range repair.
- All product wiring, the composite finalization fix, joined eligibility and binding, standing changes and schema changes wait for T1's merge, followed by a merge of main into the T3 branch.
- Each crate has its own `Cargo.lock` (there is no workspace lock). T1 changes none of the kernel lockfiles. A new arithmetic dependency would change the kernel lockfile and every dependent crate's lockfile, including `P/core/product_physics/Cargo.lock`. It needs a crate that resolves offline and locked, and it is a design question (§5.2).

## 4. Existing evidence T3 can reuse

- The frozen N01–N09, R01–R07 and NP-A–NP-D references with their reference review (`P/validation/benchmarks/numerical_integrity/`).
- The N05/N06 intended-versus-stored distinction and the prototype results in `CONTRIBUTION_PRECISION/_run_records/precision_refutation/`.
- The T0R frozen references (`DEFAULT_ROUTE_DESIGN/references.py`), for its straight-member and reaction cases.
- T1's load-state analytical references and its VP-STATIC run, for any joined-eligibility re-derivation.
- DEC-050 and DEC-053 observations and policies in `P/validation/qualification/`. They are protected and stay unchanged.

## 5. Decisions the design must settle

### 5.1 Technical (ROOT)

1. The general accuracy method: the selected contribution-preserving extended-precision path, a relative-coordinate or basic-deformation reformulation, or a generalized exact-block method. One method should serve N05 ordinary accuracy and general retained-source recovery.
2. How PHYS-R4 range is handled. Exact power-of-two scaling makes the scaled problem normal-range without rounding; T3 recommends it before any refusal.
3. The shape of the sparse assembly path and its scrutiny and fallback rules; the memory measurement protocol.
4. The composite finalization fix: port T1's SF-1 republication to pre-0.4, or publish a mixed envelope.
5. The direction of the selected-UNAVAILABLE alignment.
6. Source-blocks-1 re-homing: republish under preview-physics-1 semantics, or retire the identity for fresh solves with a successor.

### 5.2 Environment question (ROOT)

An arbitrary-precision backend (MPFR through `rug` needs GMP/MPFR system libraries; pure-Rust alternatives exist) must build offline and locked on CI and on the owner's Mac. The design must check what is vendorable before choosing. This is a build fact, not an owner choice.

### 5.3 Possible owner decision

**The comparison policy for new cases.** M34 asks for "dimension-aware ... comparison rules with near-zero floors" and says "preserve protected criteria until an evidenced change is approved". If the design proposes changing any protected predicate (the absolute 1e-9 in the mechanics benchmark, DEC-026, DEC-050/053), that needs the approval the finding names, and the approving instrument is not clear from the records. T3's recommendation is to avoid the question: leave every protected predicate unchanged and apply the existing relative form `1e-9 · max(|exp|, scale)`, with stated zero scales, only to new T3 cases. If the design finds that a protected predicate is itself wrong, T3 will bring concrete options to ROOT.

No other owner decision is foreseen at stage 0.

## 6. Findings added after stage 0

These findings were found during stage 1 and are recorded here so the map stays complete. The owning records are cited.

| Finding | Where | Severity | Owner and route |
|---|---|---|---|
| V1-S11: cancelled load contributions folded in binary64 and published as Passed | `primitive_loads/src/lib.rs:1401-1409`, `PP:1810` and the other force producers; recovery-side folds (`S11_CONTAINMENT.md` §2.2, E1–E14) | Silent wrong, bounded to about half an ulp of the gross load | T3: S11-K, then S11-F (`MANAGER_NOTES/S11_MAP.md`, `ROOT_RULINGS_V1.md`) |
| V1-S5: invocation capture refuses any request containing a finite \|x\| ≥ 2^53 | `PP:1239-1240` → `source_receipt.rs:64-73` | Refusal (capability hole) | T3 capture step (D2 S-H); T6 export carriers |
| N-S11-R: the M03 intended-action residual is published with a naive sum; an exact zero shows as -0.0 | `FK/structural.rs:554`, rendered at `PP:883, 2046, 2057` | Low: imprecise diagnostic value | T3, D-S11-2 |
| Formation silent zero: an underflowed stiffness-coefficient product becomes 0 with no error | `FK/lib.rs:717-726` | Silent input change | T3, K2 (checked formation), next after S11-K |
| Friction forces added after the ledger in the nonlinear loop, with no in-loop load guard | `nonlinear_integration/src/lib.rs:1642-1659` | Bounded | T5 (open) |
| F-P2: a rejected, unrecoverable case blocks the whole invocation (fails closed) | M03 blocking rule; D2 probe PR-2 | Coverage (no wrong value) | T3 D1 F2 (the general method recovers such cases; probe PR-2b); whole-invocation blocking with T6 |
| F-P7: binary64 renders 1e16 and 1e20 as integral literals that checked-profile readers refuse | D2 probe PR-7 | Migration hazard | T6 carrier-migration notes: every consumer switches together |
| P1-SKEW (provisional): RF-SKEW-T-CANT-OFF-122-r1e-04 (ordinary magnitudes, k/a ≈ 1e-4, skew member, no cancellation) is published Passed and Current-eligible on main, with rotations about 2.4e-9 relative off (dense ratio 2.43, sparse 1.21 against the unchanged criterion). The 345 sibling passes | Ordinary binary64 solve on the ordinary route; M03-INTEGRITY-v1 passes it (P1 early report) | Silent, small (about 8e-14 rad absolute) | T3 M03 general accuracy: D1's W1 trigger (D-5) must be revisited, since a Passed case can miss 1e-9. Pending ROOT |
