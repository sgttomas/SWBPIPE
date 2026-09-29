# K5 change record: W4, the constrained-body witness and the curved rule

This is the draft PR record for slice K5 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I14 (TASK). The details are in `RETURN.md`.

- **Branch:** `codex/piping-k5-20260928`, from main `24dea2dae`, whose product tree equals `e7d930d49`'s.
- **Commits (made by ROOT):**
  - `0c732061b`: checkpoint A1, the FK function;
  - `6bf64f5a9`: checkpoint A2, the SA wiring and the tests;
  - `416b0d456`: checkpoint B, records;
  - `f89662e1f`: checkpoint C, the mutation table and two tests;
  - `b379e5b27`: checkpoint D, records (reviewed by RV14: PASS, 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs).
- **Proposed next commit:** the addendum for RV14's review: FK `publish`'s exactness check (RV14-4), four tests and two vector files, RETURN §16, `_run_records/rv14/`, this record and SHA256SUMS. ROOT then merges main (`df6d59e3c`, app-v4 only; the piping tree is unchanged).
- **Size:** at `b379e5b27`, 11 product and test files, +5,654 −5; the addendum changes 7 of them (+423 −24) and adds 2 vector files (+316). Records are under `T3/IMPLEMENTATION/K5/`.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.9 (W4), §2.5, §4.1.3, §4.3, and the K5 row of §6;
  - `ROOT_RULINGS_V1.md`: "K5: spawn and rulings" and "K5: rulings on I14's checkpoint-0 plan" (Q7 reversed to (a));
  - the I14 brief (`TASK_BRIEFS/I14_K5_IMPLEMENTATION.md`, `a2f61531…`, with ROOT's rulings on Q1–Q11) and `_COMMON.md` (`6bb845bf…`);
  - R5-4 (`DESIGN_NUMERICS/R5_4_CURVED.md` §2, §5).
- **Platform:** Mac (`aarch64-apple-darwin`), rustc 1.97.1. T9, gate part 1 and the product runs are Mac-only comparisons against Mac main.

## What changes

**New in FK (`rigid_body.rs`; lines 1–249 byte-identical).**
- `assess_constrained_bodies(coordinates, sub_bodies, ties, grounds)` assesses one connected body. The body is made of objective sub-bodies (frames and qualified curved elements), joined by user-element ties and held by ground rows (`Dof`, and Q6's `Directional`).
- **Exact tree reduction.** It reduces each body to six unknowns: u(x) = t + θ × v(x), with exact virtual positions v, and three rows per cycle.
- **Libm-free screen.** L is a power of two; the Jacobi uses `sqrt` forms in place of `hypot`; the rank screen is τ_B.
- **Exact witness.** A witness is verified exactly with `Expansion`, and published only as its canonical representative (r = k·p/p_j, k ≤ 64) with exactly representable node motions. Otherwise the status is `NumericallyUnresolved`, never a rounded direction.
- `user_element_tie` (Q5(a)) and `objective_sub_bodies` are added.
- `assess_rigid_body` and `original_rigid_witness` are unchanged.

**SA (`structural_adapter.rs`, +253 −5).**
- `BodyEvidence` gains `w4: Option<W4Context>`.
- The four `selected` bodies of the formation-checked entries (dense and sparse, unscaled and force-scaled) call a private `constrained_geometry` with their `curved_sources`.
- There, a body with a user or curved element is assessed by W4. Its users must be ties. Its curved slots must be matched to their macro source (K-D5's predicate, at 2^b in the force-scaled branches), with consistent coordinates (Q2(b)).
- The outcome is `Mechanism { direction }`, `NumericallyUnresolved("constrained-body rank unresolved")`, or, for an unqualified body, the matrix gate as today.
- Every other `geometry()` path passes `w4: None`, and is today's code.
- No public signature changes. The basis text, `qualified_passive_family`, the edges' objective flags and exact-block eligibility are unchanged.

**Tests.**
- FK: `k5_constrained_bodies.rs` (14 tests; a standard-library generator with `--check`, 1,000 committed B1 records, and 27 constructed cases) and `k5_scale.rs` (10,000 sub-bodies, with a capped allocator).
- NI: `structural_adapter/k5_tests.rs` (11 tests), including the loop pin, the wiring pins and the frame-only witness pin.
- PP (tests only): `k5_curved_mechanism_runtime.rs` (3 tests).

## What differs from the design's letter (ROOT's rulings)

- **Only the four selected branches (Q1(b)).** Every invocation with a nonlinear support, and the nonlinear loop, keep today's geometry, so the contact-seed guard stays closed (RETURN §2.5).
- **Qualification by source, not a screen (Q2(b)).** The curved screen of §4.9 is replaced by qualification through the matched source. RETURN §2.3 derives that a K5-C1 refusal never refuses a model the intended physics restrains. The screen ratio is recorded as evidence only.
- **The basis text is unchanged (Q7(a), after S1).** It now under-claims on the selected branches. This is on the T3-close list and F2a's input list.
- **Also ruled:** T4's confirmation is not a prerequisite (Q3(a)); directional rows are in the FK API (Q6(a)); the new reason string is ROOT-approved (P2, K5-C2).

## Results

- **Product changes** (Mac, base against candidate, 152 runs of a curved corpus through both entries and both modes).
  - 128 unchanged.
  - 16 K5-C1: `NUMERICAL_INTEGRITY_UNRESOLVED` (a pivot failure) becomes `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM` with an exact direction.
  - 8 K5-C2: unresolved stays unresolved, with only the reason text changed. The witness is exact but not representable.
  - **0 outside K5-C1 and K5-C2, and 0 changes on a nonlinear-support invocation.** No published result is removed.
- **Nothing existing moves.**
  - RF-MECH's nine models through SA's geometry and the selected entries: 133 output lines byte-identical, with the expected 8 mechanisms and 1 restrained.
  - 39-manifest suites: only K5's added tests change. The only failures are the three known Mac platform tests, and their failure blocks are byte-identical.
  - **T9 (Mac-only): 112 of 112 byte-identical.**
  - **Gate part 1, full-envelope: 884 of 884 runs identical to G1's Mac baseline**, and `gate_check` PASS.
- **Mutations:**
  - 33 of 33 killed at test assertions: the design's items 12 and 7, K5-M1 to M19 with M5 not applicable under Q2(b), the original K1 and K-D5 pins' mutants, and K5-M9b.
  - K5-M9 survived its first run and was not equivalent: a free translation separates today's frame witness from W4's. The new pin `k5_frame_only_bodies_keep_todays_witness` kills it.
  - The prefilter's removal is derived equivalent (RETURN §2.8), and the D run is consistent with that.
- **Derived in RETURN §2:**
  - the energy zero sets;
  - the tie reduction, cross-checked against the unreduced stacked map on 4,000 generated cases;
  - the Q2 condition;
  - frame-only byte identity;
  - no nonlinear-support change;
  - platform independence;
  - the unreachability of the `Geometry` branch from built evidence (P6);
  - the prefilter's equivalence.

## Limits

- **Coverage:** curved bends are realized by no committed request, gate request or desktop path (R5-4), and user elements by no product build. K5's product reach is the curved-model corpus in RETURN §4.4.
- **What W4's outcomes mean:**
  - W4's `Restrained` is no restraint proof: the matrix gate still runs.
  - A body W4 does not witness keeps today's outcome.
- **Routed to other slices and loops** (RETURN §9–§10):
  - T4: the user tie rule and the curved rule;
  - T5: the loop's adoption of W4;
  - K4: the directional-row wiring;
  - F2a: the reason carrier and the basis text;
  - F3 and W1c: geometry first.
- **Merge order with F1b:** whichever merges second merges main and re-runs its suites, T9, gate part 1 and the curved product-run table.
- **Pending the independent reviewer's check:**
  - RETURN §2's derivations, including §2.8;
  - the curved rule on the corpus;
  - an exact rational null-space oracle for the constructed cases, independent of K5's generator;
  - a re-run of the loop pin's and K1's pins' original mutants.

## RV14's review and the addendum (`RETURN.md` §16)

RV14's independent review at `b379e5b27` (`REVIEW/K5_REVIEW.md`, numerics `3a17799e4`) was **PASS**: no false witness, no missed mechanism, and byte identity, the Q2 condition, P6 and the prefilter equivalence all held. Its four SHOULD-FIX items are resolved as ROOT directed.

- **RV14-4 (an FK API defect).**
  - The defect: `rigid_parameters` = [t/L, θ] published +∞ at subnormal spans.
  - The fix: `WitnessContext::publish` now publishes only when each t_i/L is finite and scales back to t_i exactly. Otherwise the witness is refused as `NumericallyUnresolved`, with the new `ConstrainedAssessment::unresolved` = "constrained-body witness parameters not representable".
  - `unresolved` is `Some` exactly when the status is unresolved; its other reason is "constrained-body rank unresolved".
  - The control is RV14's tiny-coordinate sweep, ported bit for bit into the generator: 301 records committed, 1,501 via `K5_SUBNORMAL_VECTORS`, with a new expectation `P`.
  - On RV14's whole 4,226-case corpus, exactly its 349 non-finite witnesses become unresolved, every other result is byte-identical, and RV14's oracle reports 0 findings.
  - SA's mapping and published text are unchanged. From SA-built evidence the refusal needs |t_i| > 2^952, or a rounding t_i/L, which product-scale coordinates do not reach.
- **Three test gaps, now pinned:**

| Finding | New test |
|---|---|
| RV14-1: a cycle in the τ_B band | two FK cases from RV14's construction, in `k5_b4_cycles` |
| RV14-2: two curved sources that disagree at a curved-only node | SA `k5_curved_sources_agree_at_a_curved_only_node` |
| RV14-3: K5-C2's published reason on `constructed_mechanism_r0.2_o0`, both entries and both modes | PP `k5_curved_mechanism_without_a_representable_witness_is_unresolved` |

- **NOTEs:**
  - N1: §6's line numbers renumbered to `b379e5b27`, and §2.5 cites both the base and the head;
  - N2: the spring-ground assertion added (`k5_positive_springs_ground_w4_bodies`);
  - N3, N4: no action;
  - N5: ROOT merges main.
- **Mutations**, each from a clean `git archive b379e5b27` plus the addendum's files, after a NONE control:
  - RV14-M1, M2 and M4 are killed at the new tests' assertions;
  - RV14-M3 is now killed at an assertion;
  - the new K5-M20 (the exactness check removed) is killed.
- **Tests** (targeted):
  - FK: 8 + 15 + 1, and `s11_site_table` 3;
  - NI: 129;
  - PP: 4;
  - `gen_k5_vectors.py --check` OK;
  - NI's non-test build has no warnings.
- **Not re-run:** T9, gate part 1, the 39-manifest suites and the product-run table. SA and PP sources are unchanged, and W4 runs in no T9 or gate run.

## RV14's delta check at `28517eaaa` (`RETURN.md` §16.7)

RV14's delta check was **PASS**, with 0 BLOCKING (`REVIEW/K5_REVIEW.md`, "Delta check at 28517eaaa", `f3e50948b`). ROOT's rulings are at `9b13481fa`.

- **RV14-D1 (SHOULD-FIX, fixed before merge).**
  - RV14-4's exactness clause, fl(t_i·L) = r_i, had no test: every refusal was an overflow.
  - `k5_b5_parameters_are_exact_or_refused` now carries RV14's `D_huge_underflow_L2e1023_r2e-60` as a `P` record, which must be refused because 2^-60/L underflows. It also carries RV14's exact control at 2^-40, which must publish with t_y/L = 2^-1063 exactly.
  - Tests only: 51 lines inserted inside that test.
  - Mutant RV14-M5 (the finiteness test only) is RV14's text, kept as a patch file.
- **RV14-D2 (NOTE, accepted as-is).** A refused witness ends the candidate search with `NumericallyUnresolved`. This is conservative, reachable only through the FK API, and recorded in RETURN §9.1 and §16.7. It is on the T3-close list as a candidate refinement.
- **Stage 2 ran** after ROOT released the host (`_run_records/rv14/delta/`):
  - FK's K5 tests pass on the working tree: 8 + 15 + 1, the full subnormal set included.
  - The NONE control, from a clean `git archive 28517eaaa` plus the test file, passes.
  - **RV14-M5 is killed** at `check()`'s `P` assertion (`k5_constrained_bodies.rs:269`) on the new record.
  - The memory guard was unchanged.
