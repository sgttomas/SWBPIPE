# Independent review: load/reference-state checkpoint 2

**Verdict: FINDINGS.** There is one should-fix finding and six notes. Nothing is blocking.

The physics, the source ledger, the not-joined guard, backward compatibility for well-formed documents, and the SF1–SF3/N1–N5 repairs all hold under my own derivations, probes and mutants. The should-fix is at the typed boundary: several closed unions accept unknown fields without error, so some authored input can be silently dropped. That contradicts the wire and this checkpoint's own claims, and it should be repaired before a mergeable candidate.

## Scope and identity

- **Reviewer.** A Type 2 TASK acting as a fresh-context, non-author reviewer. ROOT (HELP_HUMAN) spawned it for the load-state WORKING_ITEMS manager (`a3675abb28ada0834`). It wrote none of the reviewed bytes, delegated nothing and made no Git writes.
- **Candidate.** A detached snapshot at `404cd9c6b` on `codex/piping-load-states-20260925`. The review diff is `be1b9294a..404cd9c6b`, which includes the CP1 review record `5511596af`.
- **Brief.** `LSI/REVIEW_CHECKPOINT_2/BRIEF.md`. I verified its sha256 as `3480b70df9f25ae244ea50778c175d869ad7c825732a094e42cc05c3c16e9388`, which matches ROOT's.
- **Basis.** Git `9e8a55d`, `CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/{DESIGN,INTERFACE,VERIFICATION,REFERENCES,HYDROSTATIC_CONTROL}.md`, read with `git show`. Their hashes match the fixture's `sources` block. I also read `_run_records/ROOT_SELECTION.json`.
- **Paths.** Paths are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` means `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.

**Files reviewed (numstat added/deleted)**

| File | sha256 prefix | +/− |
|---|---|---|
| `core/product_physics/src/lib.rs` (diff only) | `aae66623` | 428/52 |
| `core/product_physics/src/pressure_runtime.rs` (diff only) | `00da0892` | 44/6 |
| `core/product_physics/src/case_state/input.rs` | `602725e4` | 344/0 |
| `core/product_physics/src/case_state/resolve.rs` | `e47ca49a` | 1087/0 |
| `core/product_physics/src/case_state/temperature.rs` | `6f030d5a` | 214/0 |
| `core/product_physics/src/case_state/mod.rs` | `f18701f6` | 27/2 |
| `core/product_physics/src/case_state/tests.rs` | `f16af25e` | 711/0 |
| `core/product_physics/src/case_state/thermal.rs` (repairs and tests) | `b78074dd` | 201/46 |
| `core/product_physics/src/case_state/material.rs` (N3 test) | `dc84e62e` | 25/0 |
| `core/product_physics/tests/load_reference_state_runtime.rs` (TASK) | `d5be0bd8` | 3012/0 |
| `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` (TASK) | `4d7b7777` | 1360/0 |
| `core/product_physics/tests/fixtures/load_reference_states/README.md` (TASK) | `fe8149c0` | 91/0 |

I also read these records: `LSI/CHECKPOINT_2.md` (`5b2e02a2`), `LSI/CP2_WIRE.md` (`81a7adba`), `LSI/CP2_WIRE_ADDENDUM_1.md` (`c389f5e3`), `LSI/REVIEW_CHECKPOINT_1/RETURN.md`, `LSI/CP2_RUNTIME_TESTS/RETURN.md`, `LSI/ANALYTICAL_REFERENCE/CP2_EXTENSION/RETURN.md` and `LSI/_run_records/session2/`.

## Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| SF-A | should-fix | `core/product_physics/src/case_state/input.rs:26` (`AuthoredModelGeometry`), `:45` (`DirectStrainReference`), `:53` (`NoFit`), `:278–279` (`ActiveModelDevice`, `Inactive`), `:343` (`IndependentEquilibrium`); module doc `:3`; the shared `Quantity` at `core/product_physics/src/lib.rs:301` | **Unit variants of the internally tagged unions ignore unknown fields.** Serde does not apply `deny_unknown_fields` to a unit variant. Probe `p03` (`_run_records/probes_a.log`) solves every case below with `MECHANICS_SOLVED` and no diagnostic:<br>• `fit: {"kind":"none","length_change":{"value":-2,"unit":"mm"}}` publishes `fit_kind: "none"`, `total_eigenstrain: 0`. An authored 2 mm cut-short disappears. With VERIFICATION control 6 numbers, that drops a +40 kN installation force.<br>• `basis: {"kind":"direct_strain_reference","installation_temperature":…}` drops the temperature.<br>• `history: {"kind":"independent_equilibrium","predecessor_case_ref":…,"predecessor_state_hash":…}` silently discards a declared predecessor.<br>• `participation: {"kind":"active_model_device","components":[lock…]}` discards lock components.<br>• `geometry_ref` with a forged `projection_sha256` is accepted.<br>• `operating_temperature: {"value":20,"unit":"degC","basis":"guessed"}` is accepted, because `Quantity` is open.<br>Struct variants are rejected correctly; the control `fit_strain` + `length_change` returns `Err`. The defect contradicts INTERFACE ("Unknown discriminants or fields in the new namespace block … rather than disappearing"), DESIGN §1 ("Unknown versions/fields must not be silently discarded"), CP2_WIRE ("All new objects are closed"), ADDENDUM §1 ("an unknown field in any object" → `Err`) and CHECKPOINT_2 ("Unknown fields … are rejected at the typed boundary"). No runtime negative control covers a unit variant. | Make each unit variant an empty struct variant (`#[serde(rename = "none")] NoFit {}` and likewise for the others). Probe `b04` confirms the enum-level `deny_unknown_fields` then rejects extra keys (`unknown field length_change, there are no fields`), and a bare `{"kind":"none"}` still parses. Either give the new namespace a closed quantity type, or record `Quantity` as an explicit exception. Add runtime negative controls for each unit variant carrying an extra field. Correct the claim in CHECKPOINT_2, ADDENDUM §1 and the `input.rs` doc. |
| N-1 | note | `core/product_physics/src/lib.rs:2122–2135` (`observation_force`), `:2110–2119` (`reduced` with prescribed values); tests | **The DEC050/053 observation lanes are correct but unpinned under prescribed motion.** On the candidate, the published dense-parity row `sparse_live_path_dense_parity_relative_delta` is 0.0 for moving-root cases (probe `p02`). If the lane is reverted to the uncoupled `f` (mutant M02), that row becomes 1.0 in both cases, and every reviewed test and probe still passes. The same holds for a zero-boundary `reduced` (M03). Inspection evidence could therefore regress to a 100% mismatch unnoticed. | Assert in the runtime suite, or at least in the smoke tests, that the dense parity delta and the sparse residual stay at round-off level for a prescribed-motion case in `DenseScrutiny`. |
| N-2 | note | `core/product_physics/src/lib.rs:145`, `:176`, `:188`; `core/product_physics/src/case_state/resolve.rs:72–81`; `LinearStaticPreviewRequest.materials` | **Older documents are handled inconsistently when the new keys are null or malformed.** Probe `p04` on a 0.3.0 document:<br>• `reference_configurations: null` and `analysis_state: null` solve as if absent, with no `VERSION_MISMATCH`;<br>• `expansion_laws: null` returns a typed `Err`;<br>• `reference_configurations: {…}` of another shape returns a whole-document `Err`, not the targeted diagnostic;<br>• `expansion_laws: []` blocks correctly.<br>At `be1b9294a` all of these keys were ignored as unknown fields. Old documents that happen to use these names now fail outright instead of blocking with the targeted code. A request-level `materials[].expansion_laws` is still silently ignored on pre-0.4 requests. | Decide whether a present `null` counts as carrying the key, then apply the choice uniformly (block, or treat as absent) and pin it with a test. Record that a malformed legacy key is a typed `Err`. If a targeted block is wanted, capture those keys as raw `Value` for non-0.4 documents. |
| N-3 | note | `core/product_physics/src/lib.rs:170` (`materials: Vec<serde_json::Value>`), `:181–199` | **The text path now accepts duplicate keys inside a material record.** Because material records pass through a `serde_json::Value`, a duplicated key is resolved last-wins. Probe `p05` parses a JSON text directly into `LinearStaticPreviewRequest` with `elastic_modulus` repeated and takes 1 GPa. The same duplication in a pipe record still errors (`duplicate field material`), and before this change it errored for materials too. Material parse errors also lose their line and column. The product's own entries (`…_value_with_mode`, self_weight_wasm, operation_applier) already go through `Value`, so the practical exposure is narrow. The change nonetheless touches every document version. | Deserialize materials through a typed wire struct that carries `MaterialInput`'s fields plus `expansion_laws`, then split it. That keeps duplicate-field detection. Otherwise, record the change. |
| N-4 | note | `core/product_physics/tests/load_reference_state_runtime.rs`; `core/product_physics/src/case_state/tests.rs`; the fixture | **Gaps in the independent acceptance suite.** These are covered only by the implementer's in-module tests, or not at all:<br>• nonzero pressure with per-member E/ν (M06, M19 and M23 are killed only by `case_state::tests::pressure_recovery…`);<br>• the operating-temperature and order-check identity (M22 and M15 are killed in-module only);<br>• no test consumes the fixture's `temperature_unit_identity`, including its non-equal control, or `thermal_datum_ratio.verification_two_point`;<br>• a 1e-8 relative endpoint-snapping mutant (M24) survives every reviewed test and is killed only by my probe `b01`, which consumes that control;<br>• `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED` is not exercised through the public route (my probe `b02` shows it is reachable and targeted);<br>• two material points with the same exact temperature in different units (M14, canonicalizing material points, survives);<br>• the dilation datum-zero check being recorded as consulted rather than consumed (M17 survives; N1 is pinned only for secant tables). | Add runtime cases driven from the fixture: the non-equal control (fraction > 0, two consumed points, override required at the exact point), a `467.6 degF` / `515.1500000000001 K` refusal, a closed-region nonzero-pressure two-case check, duplicate-class material points (`…TEMPERATURE_AMBIGUOUS`), and a dilation `consulted_law_point_indices` assertion. |
| N-5 | note | `core/product_physics/src/lib.rs:1996` (`built.sections.get(&member.pipe_id)?` inside `filter_map`) | If a member's section were ever missing, its resolved eigenload would be dropped with no diagnostic. The present build makes this unreachable: every built pipe inserts its section or blocks. | Replace the silent `?` with an `expect` or a blocking diagnostic, matching `LOAD_STATE_MEMBER_MATERIAL_MISSING`. |
| N-6 | note | `LSI/CHECKPOINT_2.md` ("Old documents"); `LSI/CP2_WIRE.md` ("Branches known to the reviewed interface … parse and then block"); ADDENDUM §1 | **Some record statements are inaccurate.**<br>• The typed-boundary claims are inaccurate as described in SF-A.<br>• The reviewed INTERFACE branch `history: continuation` returns a typed `Err` (unknown variant) rather than "parse and block". ADDENDUM §1 records this as an example, but the CP2_WIRE sentence is not updated.<br>Every other CHECKPOINT_2 claim I checked is accurate; see below. | Correct these in the next addendum or checkpoint record. |

## What I verified, and how

1. **Build and full suite at the frozen commit.** I ran `cargo +1.97.1 test --locked --offline -j 1` on the whole crate, with an external target that was deleted afterwards.
   - Results: lib 299 passed (1 pre-existing ignored), `load_reference_state_runtime` 20/20, other integration tests 62, doc tests 0.
   - The sources were hash-stable: my pre- and post-run hashes match, and the manager's gate-2 prehash agrees with every file it lists.
   - The 268 non-case_state lib tests are identical by name to the `be1b9294a` baseline. Every hunk in `lib.rs` and `pressure_runtime.rs` lies above the test modules, and no other integration test changed.
   - The reviewed modules add no compiler warnings (log: `snapshot_all_tests.log`).
   - This is also the repeated independent acceptance run of the runtime TASK's binary on the frozen commit, which CHECKPOINT_2 requested.
2. **Physics, against my own derivation.** Probes use only the public API: `run_linear_static_preview_value_with_mode`, in both modes, with a relative tolerance of 1e-9.
   - **Probe `p01`.** An axial chain: E1 200 GPa carrying an explicit strain of 4e-4, E2 100 GPa, root prescribed +0.3 mm, a middle force of 5 kN included at factor 2.5, and an unreferenced 7 kN force. I solved it with my own oracle: u_b = (k1·g + E1Aε1 − E2Aε2 + P)/(k1+k2), N_i = k_i·Δu − E_iAε_i, R_a = −N1, R_c = N2. The product matches u_a = g, u_b, R_a and R_c. The excluded force appears only in `excluded_sources`.
   - **Probe `p02`.** Two cases with swapped E per member and different root motions each rebuild their own stiffness and match independently. Diagnostic IDs are unique; so are result IDs (probe `b03`: 148 and 150 rows, all unique).
   - **Code reading.**
     - Per-member E/ν with derived G reaches `build_model_for_members`, `build_pressure_case_with_members` (`state.material`), `exact_cases.pipe_materials` and `pressure[].materials`.
     - The eigenstrain enters `ThermalElementLoad` once, as `E_member·A_s·ε*` with the exact annulus `A_s` that the stiffness also uses. `corrected_local_forces_for_axial_effects` removes it once.
     - Prescribed tuples reach `AssemblyEvidence::solve`. The kernel applies `f_f − K_fc·g_c` and checks the full residual rows (`core/solver/frame_kernel/src/structural.rs:603–606`, `:705–707`).
     - `displacements` includes g, and reactions are `K·u − f` from the full matrix, springs included.
     - A parallel spring on a moving rigid DOF is attributed correctly: rigid = K_total·u − f, spring = −k·g.
     - Two supports cannot claim the same DOF, because `prepare_boundary` returns `RepeatedRestrainedDof`.
   - **Mutants.** M01, M04, M05, M07, M10 and M13 are all killed (table below).
3. **Source ledger.**
   - `effective_case` holds exactly the declared primitives, scaled by their factors, in declaration order. Pressure regions pass through unchanged.
   - A duplicate `source_ref` gives `LOAD_STATE_SOURCE_DUPLICATE`, and a zero or non-finite factor blocks. A referenced legacy thermal primitive blocks, and M18 shows the guard is load-bearing.
   - I traced every post-resolution reader of the case:
     - `build_load_case_primitive_loads`, `build_pressure_thrust_loads`, `pressure_for_pipe`, the legacy thermal branch (not taken) and the evidence all read the effective case;
     - `validate_profile`, `validate_model_inputs`, `self_weight::validate_applied_self_weight` and the ID-only readers (`assessed_numerical_quality`, `qualify_source_case_rows`, component-modifier lookups) read the unfiltered model but only validate or look up IDs; they never apply loads;
     - no path re-applies an excluded source.
   - Mutants M08, M09 and M11 are killed.
4. **No inherited eligibility.** For 0.4.0:
   - `selected_source` is always `None` (M12 is killed by the one-per-case NOT_JOINED assertion);
   - `source_case` is `None`;
   - envelope-level `source_selected` is false, so neither `finalize` nor `finalize_composite` runs;
   - `check_input_with_physical`, which rebuilds with the old methods, is reachable only through those paths;
   - the Sensitive gate is preserved: a Sensitive ordinary solve publishes with its integrity diagnostic, and an ordinary `Err` blocks;
   - the not-joined record, the evidence `solve.recovery_method` and the formulation limitation are truthful.
5. **Backward compatibility.**
   - Every behavioural change is gated on `is_load_state` or `load_state.is_some()`. The legacy branches were checked line by line:
     - `build_pressure_case(None)` keeps the old material, `thermal_consumed` and `temperature_basis`;
     - zero prescribed tuples reproduce the old boundary, observation force, displacements and dense observation;
     - `pipe_materials` is unchanged;
     - `source_case` is unchanged;
     - `blocked_envelope` and the producer/profile are unchanged;
     - `is_exact` and `validate_profile` change only the 0.4.0 arm and the connector check.
   - The `solve_preview_reduced_system` signature change has one caller, and zero prescriptions produce identical `free` and `prescribed`.
   - The manual `PreviewModel` deserializer is otherwise equivalent. `float_roundtrip` is on and `arbitrary_precision` is not enabled anywhere in `core/`. The exceptions are N-2 and N-3.
   - Old documents carrying the namespace block (M21 is killed).
   - Downstream crates, run at the snapshot beyond `--no-run`: operation_applier 176/176, self_weight_wasm 14/14, headless 64/64.
6. **Typed boundary.** Struct variants and unknown discriminants are rejected; the TASK's observed codes match the targeted ones. See SF-A for unit variants and `Quantity`. I found no ambient temperature, no synthesized material identity, no read of legacy α on the 0.4.0 path, and no silent default elsewhere.
7. **Repairs.**
   - **SF1.** I re-derived by hand, and in `review2_fixture_rederive.py`, 5/1601, −5/1606, expm1(±1/320), 4/2001 and −4/2005, together with the split and discriminator values. The kernel test and the product-route test reproduce them.
   - **SF2.** For the quadratic secant, λ = 0.001t² − 0.5t + 61 has roots at about 211.3 K and 288.7 K, so [300, 400] K is admissible and 240 K is refused. The control-4 two-point table gives 43/25009. M16 is killed.
   - **SF3.**
     - `{:e}` is Rust's shortest round-trip formatting.
     - The affine definitions are exact: K; °C + 27315/100; (°F + 45967/100)·5/9; °R·5/9.
     - Every i128 operation is checked, and overflow refuses rather than wraps (1e300 K is refused).
     - Points always register before other temperatures, and one representative per exact class feeds both the selector and the kernel.
     - `check_order` refuses a set whose representatives are not strictly increasing, and no tolerance is applied anywhere. Probe `b01` shows −49.999999 °C against 223.15 K interpolates with fraction 1.3e-8 and needs an override as an exact point. Probe `b02` shows the refusal is reachable. M22 and M15 are killed.
     - I found no remaining decision on unregistered temperatures: an ExactPoint selection compares only its own point and the actual temperature.
   - **N1.** The secant test pins consumed versus consulted; for dilation see N-4.
   - **N2.** The vacuous assertions were removed and the unit-equivalence test was moved to the adapter.
   - **N3.** The order [120, 20, 70] is meaningful: an unsorted "first containing pair" gives 125 GPa rather than 140 GPa, and an ascending-pair search finds no bracket.
   - **N4/N5.** The module-wide allow is gone, the doc is updated, and the reviewed modules add no warnings.
   - **N7.** The runtime thermal variants author 1/degF coefficients and degF/K absolute temperatures.
8. **Tests and deliverables.**
   - **Runtime suite.** All 20 tests loop over both modes. The expectations come from the fixture at test time, and three derivations are disclosed. The criterion is 1e-9 relative with named zero floors; nothing was loosened. The in-module helpers also use 1e-9, and no pre-existing criterion changed.
   - **Fixture.** `review2_fixture_rederive.py` is my own standard-library script (fractions, decimal, Machin π). It does not read the generator, its checker or the manager's checker. It ran 2214 checks with 0 failures and detected 5/5 mutations of its own.
     - It checks every quantity in the three added sections: `exact`, `decimal` to 80 or more digits, and `value` correctly rounded.
     - It verifies that all 1816 CP1 leaf paths are byte-equal to `git show be1b9294a` (sha256 `5478bba8`).
     - It confirms the control-4 inputs against the VERIFICATION text.
     - It checks the binary64 discriminators with IEEE arithmetic.
   - **README.** Accurate. All values are labelled invented, and there are no library or code-rule terms.
9. **Records.** The CHECKPOINT_2 claims hold, apart from N-6:
   - hashes (`d5be0bd8`, `4d7b7777`, `fe8149c0`);
   - test counts (299 = 268 + 31, runtime 20, integration 62);
   - 1360/0 numstat;
   - the retained implementer failures;
   - the manager's four repair mutants;
   - the dependent `--no-run` results and the ENOSPC reruns;
   - the inherited numerical_integrity lock issue;
   - the mechanism.

   Machine paths appear only in `_run_records/`.

## Mutation summary

All mutants ran on a scratch copy of `core/`. Each was restored and sha256-verified, and the scratch copy was diffed identical to the snapshot afterwards. The suites were `--lib`, `load_reference_state_runtime` and my probes (`mutations.log`, `mutations_followup.log`, `mutant_logs/`).

| Result | Mutants |
|---|---|
| Killed by the reviewed tests (19) | M01 complete u omits g; M04 eigenload doubled; M05 eigenload not removed; M06 pressure ignores pairs; M07 stiffness reused from case 0; M08 unfiltered case; M09 factor ignored; M10 G not derived; M11 duplicate source admitted; M12 not-joined guard removed; M13 prescribed sign; M15 order check off; M16 secant datum coverage restored; M18 legacy-thermal guard removed; M19 evidence pipe_materials from base; M20 fit ignored; M21 version guard removed; M22 operating temperature not canonicalized; M23 pressure E from base |
| Killed only by my probes (1) | M24, 1e-8 endpoint snapping (by `b01`) |
| Survive every suite (4) | M02 and M03, observation lanes (N-1); M14, material points not canonicalized (equivalent unless two points share an exact class); M17, dilation datum counted as consumed (N-4) |

The first mutation run did not attribute failures, because a parser bug concatenated stdout and stderr. It was interrupted with SIGINT, the scratch copy was verified restored, and the run was repeated; the first log is retained as `mutations_attempt1_parser_bug.log`.

## Raw logs

Everything is in `LSI/REVIEW_CHECKPOINT_2/_run_records/`:

- `RUNS.json`
- `snapshot_all_tests.log`, `prehash_snapshot.txt`, `posthash_snapshot.txt`
- `review2_fixture_rederive.py`, `fixture_rederive.log`, `fixture_rederive_mutations.log`
- `review2_probes.rs.txt`, `probes_a.log`, `review2_probes_b.rs.txt`, `probes_b.log`
- `review2_mutations.py`, `mutations.log`, `mutations_followup.log`, `mutations_attempt1_parser_bug.log`, `mutant_logs/`
- `dependent_{operation_applier,self_weight_wasm,headless}_tests.log`

## Limits

- **Not run.** Piping pytest, the evidence sweep, the numerical_integrity and physics_audit_regression dependents, and the browser, native, desktop, UI and npm lanes. None of these was in scope.
- **Scope.** Coverage is limited to the listed files and their direct call paths. I read the CP1-reviewed material selector and thermal kernels only where the repairs or integration touched them.
- **Probes.** The probes and mutants used scratch-only copies, which were deleted after the review. Reviewed bytes were never edited. The snapshot changed only inside this directory.
- **Status.** This review closes no M10, M16 or M29 finding and is not acceptance.
