# RV17: independent full-diff review of slice F1b

- **Reviewer:** RV17 (Type 2 TASK, independent reviewer).
- **PR:** #1052, `codex/piping-f1b-20260928`.
- **Head reviewed:** `f183e1fa9`, the final head. The brief named `9aeed9c22`; ROOT moved the head twice during the review:
  - `07bed2638`, the `pressure_thrust_load` pin, then `c4879c496`, a merge of main `1cdeae2c1` with K5;
  - `f183e1fa9`, records only (RETURN addendum 2).
  - `P/core` is identical at `c4879c496` and `f183e1fa9`, and `product_physics/src` is identical at `130445db2`, `9aeed9c22` and `c4879c496`.
- **Date:** 2026-09-29.
- **Verdict: PASS.** No BLOCKING finding. 4 SHOULD-FIX and 5 NOTEs.

**In short.**
- **The product change holds.** I found no published value that is wrong, and no byte change on anything main publishes. My checks:
  - the sparse wiring, W2's engagement order, admission, publication and refusals, read against the design and ROOT's Q1–Q14 and OQ rulings;
  - gate part 1 re-partitioned from the committed tables (C3: 832 runs, 0 differences), with 116 raw full envelopes re-hashed;
  - the 14 published C1 runs against R1's exact references, and my own exact-rational oracle for RF-RANGE-THIN-B;
  - the product head `c4879c496` built from an archive: NI 134 and PP 566 pass (only the known Mac `t13` fails), K5's `k5_curved_mechanism_runtime` included;
  - RETURN addendum 2, spot-verified: gate part 1 on `c4879c496` equals the `130445db2` gate in all 884 runs, and T9 is 112/112 (§4).
- **D10's step 4 is false (RV17-1).** Main's exact-block does select range-triggered cases. For two of my constructions, CX-F and CX-G, main publishes the selection with a receipt, so the coexistence test the brief requires can be built. D10's step 5 still holds: F1b's envelopes equal main's on all 10 selected runs, both on the recorded candidate probe and on my own build of the head.
- **Two test gaps let a mutant through (RV17-2, RV17-3):**
  - RV17-2: deferral on nonlinear invocations changes the bytes of the blocked envelope;
  - RV17-3: W2 publishes one spring's action for every spring.
  - Neither mutant is equivalent. Each is shown by a product probe.
- **A records gap (RV17-4).** 8 of the 14 published C1 runs lie outside R1's exact references by up to 1.7e-5 relative. They are Sensitive, and they match main's same-family Sensitive runs exactly. No record says so.
- **Re-runs, all killed at I13's sites:**
  - F1B-LANE-UNGUARDED, on both heads;
  - all 18 NI pin mutants on `c4879c496` (the three F1b product-half mutants and the 15 original ones). They are killed at exactly I13's NI sites, plus one site that K5 adds.

## Reviewer, brief and delegation

- **Reviewer.** RV17 is a Type 2 TASK, dispatched directly by ROOT (HELP_HUMAN) as a background subagent of ROOT's session. ROOT is the only return path.
  - I did not design, implement or test F1b. I delegated nothing.
  - I made no Git write and no index operation, with one caveat: I ran `git status --short` twice in `<wt>/f1b` and once in `<wt>/numerics`, which may refresh the index stat cache (disclosed under §8).
  - My writes are this file and `T3/REVIEW/_run_records/f1b_review/**`, left uncommitted.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `T3/TASK_BRIEFS/_COMMON.md`, and `I8R_K1_RESUME.md` ("The Mac host");
  - `TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md`, with Q1–Q14;
  - `ROOT_RULINGS_V1.md`, every F1b section: spawn, checkpoint 0, the A2 stop, A2, the heap-cap resume, and the gate re-run;
  - F1b's `CHANGE_RECORD.md`, `RETURN.md` (with addendum 1 at `c4879c496`) and the `_run_records/` I cite;
  - `REVIEW/K5_REVIEW.md` and `REVIEW/K2B_REVIEW.md`, for method;
  - the complete diff, and the code it relies on: SA's orchestrator, siblings and K5's W4; FK `structural.rs` (`prepare_bound`, `audit_contributions`, `Expansion`, `exact_radix`); FK `sparse.rs` assembly; SD's profile solver; PP's case loop, `source_recovery.rs`, `source_receipt.rs` finalization, `preview_physics.rs` and `pressure_runtime.rs`.
- **Abbreviations.**
  - `P/` = `projects/chirality-piping/`; `T3/` = the T3 folder; `R/` = `T3/REVIEW/_run_records/f1b_review/`.
  - PP = `P/core/product_physics/src/lib.rs`; SR = `P/core/product_physics/src/source_recovery.rs`; SA = `P/core/solver/nonlinear_integration/src/structural_adapter.rs`; FKS = `P/core/solver/frame_kernel/src/structural.rs`; SD = `P/core/solver/sparse_direct/src/lib.rs`; W2RT = `P/core/product_physics/tests/f1b_w2_runtime.rs`.
  - RETURN and CHANGE_RECORD are F1b's, at `f183e1fa9`. RETURN's line numbers are unchanged from `c4879c496`.
- **Line numbers** are at `c4879c496`. PP's are equal to RETURN's, because PP is unchanged since `130445db2`.

## Revisions reviewed (`R/revisions.txt`)

- **The slice** is `94e543a24` (A1), `e215c6007` (A2), `948e0bb99` (C) and `130445db2` (the lane guard): 14 files under `P/core`, +5,531 −188. Then come `9ecf2bdca` (records) and the merge `9aeed9c22`.
- **`9aeed9c22` adds exactly main's delta.**
  - The parents' merge base is `e7d930d49`, and `git diff e7d930d49 b37331092` equals `git diff 9ecf2bdca 9aeed9c22` byte for byte.
  - `git show --remerge-diff` is empty.
  - Main's delta touches only `P/../chirality-app-v4/` and `P/execution/`.
- **`07bed2638`** changes only W2RT under `P/core` (+59 −2), plus F1b's records (addendum 1).
- **`c4879c496` adds exactly main's delta (K5, PR #1044).**
  - The merge base is `b37331092`; the diff equality holds; the remerge diff is empty.
  - Outside records, main brings FK `rigid_body.rs` and its tests, SA (+253 −5), SA's `k5_tests.rs`, and PP `tests/k5_curved_mechanism_runtime.rs`.
- **The combination is sound (§1.7).**
- **`f183e1fa9`** changes 73 files, all under `T3/IMPLEMENTATION/F1B/` (addendum 2 and its records). `P/core` is unchanged.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV17-1 | SHOULD-FIX | RETURN.md:794-808 (D10 step 4 and "The test"); W2RT:1059 (the candidate-side test only). The brief's required test D, "Coexistence with exact-block". ROOT's A2 ruling (c) and its condition. | **D10 step 4 is false.** Main's exact-block selects range-triggered cases (`R/coex/`).<br>**The constructions:** one member of 1 m on the captured entry, free only in N1's UX (CX-A/B/F/G) or UY (CX-D), with a short-mantissa tip load (one or two significant bits). The ordinary attempt on main range-triggers:<br>– CX-A, CX-B, CX-D: `Range("radix scaling loses normal range")`;<br>– CX-F, CX-G: `Range("division overflow or underflow")`.<br>**What main does:**<br>– on CX-A/B/D, exact-block selects (`SOURCE_BLOCK_RECOVERY_SELECTED`), and then the receipt refuses the MPa stress (`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`);<br>– on **CX-F and CX-G** (OD 0.02 m, wall 0.002 m, EA/L ≈ 2^16 N/m, tip load 3·2^-1016 N or 2^-1015 N, every published value normal in its unit), main publishes: `MECHANICS_SOLVED`, the selected response, and a receipt (`R/coex/selected_excerpt.txt`, `R/coex/envelopes/`).<br>**Step 5 holds.** F1b's full envelope equals main's on all 10 selected runs, both on the recorded candidate probe and on my archive build of `c4879c496` (`R/coex/head_compare.txt`).<br>**Why step 4 fails.** M03's scaled right-hand side on a decoupled DOF is about f/√K, that is √(f·u). Exact-block needs only exact products and a projection within 1e-9. A one-bit load makes K·f exact, and \|u\| above about 2^-1044 passes projection. I13's candidates (1e-310 N, 2.5e-308 N, …) have full mantissas, so their products lose bits at `exact_radix`. My controls CX-C (1e-310 N) and CX-E (1e-307 N) reproduce that failure. | Correct D10: step 4 is refuted, and steps 2, 3 and 5 stand. Add the brief's positive coexistence test with CX-F or CX-G, captured, in both modes. It should assert:<br>– `SOURCE_BLOCK_RECOVERY_SELECTED`;<br>– the receipt;<br>– the info-severity "Rejected ordinary attempt … Range(…)" record;<br>– no `range_scaling` text;<br>– and, as addendum 1 does, main's full-envelope sha256 (`d953a683…` sparse, `10d312a1…` dense, for CX-F).<br>F1B-M2 is then also killed at a positive assertion. **ROOT should re-rule A2 (c)**, since its condition (the reviewer checks the derivation) is not met. |
| RV17-2 | SHOULD-FIX | PP:2367 (`let linear = …`), PP:2857-2870 (`form_basis_stiffness`), PP:3767 (the `(None, Formation)` arm). Test side: `tests/k2a_formation_range_runtime.rs:137` (`assert_refused_by_name`). | **Mutant RV17-M1:** `let linear = true;`. The basis's range refusal is then deferred on nonlinear invocations too. W2 still never engages there, because the arm keeps its own nonlinear filter.<br>– It **survives** NI and PP on both heads. Only the baseline `t13` fails (`R/mutants/MUTANTS.txt`).<br>– **Probe m1**, K2a's partial-underflow member with its open gap: main, clean `9aeed9c22` and clean `c4879c496` all give the same bytes (`a29e29f2…`), a blocked envelope with one `SOLVER_SYSTEM_BLOCKED`. The mutant gives `97ef552b…` on both entries and in both modes: the case loop runs first and adds `LOAD_CATEGORY_PREVIEW_MAPPED` (`R/probes/probe_compare.txt`).<br>– The brief requires these invocations to publish "byte for byte as main". The K2a nonlinear variants assert only the one named blocking diagnostic. | Pin the nonlinear variants' blocked envelope in full:<br>– the exact diagnostic list (one diagnostic, `diagnostic:physics:solver`);<br>– or, as in addendum 1, the full-envelope sha256 against main's.<br>Show RV17-M1 killed. |
| RV17-3 | SHOULD-FIX | PP:4084 (`publication.spring_actions[spring_index]`). Test side: every W2-publishing model in W2RT, `f1b_tests.rs` and the K2a tests has at most one spring. | **Mutant RV17-M2:** `spring_actions[0]` for every spring.<br>– It **survives** NI and PP on `c4879c496`.<br>– **Probe m2a:** K2a's exact-zero member, free in UY and UZ, springs of 1e-289 and 3e-289 N/m. W2 publishes it at b = 734. The clean head publishes spring:N1:UZ Fz = −1.8363222504158902e-289 N. The mutant publishes −4.363400364938612e-290 N, which is the UY spring's action: a wrong value, `MECHANICS_SOLVED`, and no test fails (`R/probes/`). | Add a two-spring W2 publication test (m2a, both entries and both modes). It should assert each spring's component: its checked value, or within 1e-9 of −k·u from the published u. Show RV17-M2 killed. |
| RV17-4 | SHOULD-FIX (records) | RETURN.md:390-411 (§9.3 and the C1 table); CHANGE_RECORD.md "Results". The brief's PASS text: "published within the criterion or refused by name". ROOT's A2 ruling: "0 trusted breaches (the published range cases are checked against the references)". | P1's predicate over every published observation, trusted or not (`R/gate/c1_references.out`):<br>– CHAIN-E-1000 ×4 and CHAIN-LEF-large ×2: 17 observations outside 1e-9 (12 keys), worst 9.7e-8 dense and 1.7e-8 sparse;<br>– SKEW-LEF-large ×2: 16 and 13 outside (14 and 12 keys), worst 1.3e-5 and 1.7e-5.<br>They are Sensitive and `needs_recompute`, so `gate_check` does not count them. Main's same-family Sensitive runs at b = 0 (CHAIN-SIM-b, CHAIN-L+240, SKEW-SIM-b) show the same counts and worst ratios to four digits. W2 is therefore no worse than main.<br>THIN-B ×4 and CONT-LEF-large ×2 are within 8e-16. My own exact-rational THIN-B oracle agrees to within 6.7e-16. | State in the C1 table and in CHANGE_RECORD which published C1 runs lie outside the references, with the counts and worst ratios, and that they equal main's same-family Sensitive pattern. ROOT should confirm that the PASS text's "within the criterion" means trusted publications only. |
| RV17-N1 | NOTE | PP:3455-3461 (the finiteness scan's stiffness half; RETURN D6). | **Mutant RV17-M4:** the scan reads only the force. It **survives** NI and PP.<br>The stiffness half is reachable: FK's assembly checks finiteness after frames and users, but before blocks and springs are added. **Probe m4** (typed only: EA/L ≈ 6e304 N/m plus an `f64::MAX` spring) shows:<br>– main and the clean head: "computed mechanics must be finite, got inf";<br>– the mutant: "matrix/vector entry must be finite, got inf".<br>The case is still refused, with a different text on a main-refused input. | Add a test on the m4 model asserting main's refusal text in both modes. |
| RV17-N2 | NOTE | PP:2950-2958 and CHANGE_RECORD.md:30 ("24 bytes per profile entry, counted in the code"); RETURN.md:238; SD:716-724, SD:386. | The lane's estimate 24P bounds only the identity-order build.<br>– SD keeps `original_profile` (capacity below 2P, so under 16P bytes) alive while it builds the RCM-ordered profile and while `factorize_ldlt` clones it.<br>– So the lane's peak is about 16P + 24P′ + O(n + nnz), with P′ the ordered profile. That exceeds 24P whenever P′ > P/3.<br>– D12 is unaffected: it uses 24P > C only as the firing condition, and main's live bytes only as a lower bound. | Say "bounds the identity-order build, not the lane's peak" in CHANGE_RECORD and RETURN §6.2. |
| RV17-N3 | NOTE | PP:2871-2882 (96 bytes per n² entry), RETURN §6.1. | I counted the n² buffers alive at once in the linear dense-scrutiny attempt:<br>– `to_dense` (8, SA:708) and `dense_symmetry_view` (8 + 8, SA:617-620);<br>– `prepare_bound`'s `a` (8, FKS:1260);<br>– `contribution_sums` (32, FKS:775) and `contribution_differences` (32, FKS:796). `Expansion` is `Vec<f64>` plus `usize`, 32 bytes (FKS:675).<br>That is 96, and the later phases hold 72 (`l`, FKS:1822, with the completion's sums). The constant is right **as the n² coefficient**. It is not a bound on total heap:<br>– the O(n) row headers and the per-nonzero `Expansion` term heaps are extra;<br>– the largest admitted model (1,365 nodes, 8,190 DOFs) is estimated at 6,439,305,600 B, only 3.1 MB under the ceiling. The O(nnz) expansion heaps alone (about 150 B per nonzero, twice) exceed that margin, so such a model would still exceed the gate's 6 GiB heap cap;<br>– the nonlinear loop's own dense work (Q1's remainder) is outside the estimate, and sparse-mode nonlinear invocations are unguarded, as on main. | Disclose. V-P and K6 set the margin. |
| RV17-N4 | NOTE | `07bed2638`: W2RT:832 (`f1b_w2_admission_refuses_a_zero_legacy_pressure_as_pressure_thrust`); RETURN addendum 1 §A1.2. | **The pin asserts what it must** (§7).<br>**The platform argument holds on inspection.**<br>– The twin's `hypot` sites (PP:4124, :4320, :11041, :11047, :12676; `preview_physics.rs:634`, `:966-967`; `stress_recovery/src/elastic_section.rs:105`) each have at most one nonzero operand for this cantilever.<br>– Otherwise only `sqrt` is reached, which is correctly rounded.<br>– Hosted Linux CI is the arbiter.<br>**Brittleness.** The full-envelope hash will need re-pinning whenever a later slice legitimately changes this model's envelope. | None required. The test's comment could name that re-pin duty. |
| RV17-N5 | NOTE | Gate part 2 (`_run_records/gate2_130445db2/part2/`); RETURN §9.3. | All 8 dense 1,000-member runs time out on both sides. Dense-mode byte identity at 1,000 members is therefore unobserved at the gate, and B1–B3's attempt parity reached 606 DOFs. This is pre-existing (K6's timeouts). | None for F1b. |

## 1. Correctness of the product change

### 1.1 The sparse wiring

- **Per-basis assembly.** `assemble_basis_stiffness` (PP:2816) calls FK `assemble_sparse_stiffness` with the frames and users, the realized bends as blocks, and the springs in the dense order.
  - FK sums frames and users, checks finiteness, then adds blocks and springs (`sparse.rs` assembly).
  - Main's dense path has the same order: `assemble_global_stiffness_with_user_elements`, then `add_curved_bend_stiffness_contributions`, then the spring loop.
- **Formation errors.** `form_basis_stiffness` (PP:2857) defers only `NumericalRange`, and only when `linear`. Every other error returns main's `solver_blocked`.
- **Reads of K** go to stored values with an absent entry read as +0.0:
  - the finiteness scan (PP:3455, D6);
  - the partition (PP:3508);
  - the observation force (PP:3261);
  - the attempt (`SparseAssemblyEvidence`, PP:5364);
  - E12's reactions (PP:3297).
- **Dense views remain only:**
  - inside SA's DenseScrutiny branches, and in PP's protected lanes after the attempt (PP:3829-3866). The dense guard bounds both;
  - for retained-source recovery at n ≤ 256 (PP:3583-3591). RV17-M3, which drops the size condition, is killed by C-SPARSE's captured-peak assertion (`tests/f1b_sparse_pattern_memory.rs:161`; 5,004,804,259 B against 1 GiB).
- **No automatic fallback.** `dense_fallback_message` is `None` on every path, including W2's.

### 1.2 Engagement order (Q2)

In `solve_load_case`, in order:
1. The ordinary attempt (PP:3544), or `OrdinaryFailure::Formation` for a deferred basis.
2. Exact-block where eligible, or OQ2's zero-work decline for a deferred basis (PP:3599-3666).
3. The outcome match (PP:3685):
   - `Ok`;
   - the selected arm (PP:3690);
   - the contact-seed arm;
   - then the final arm (PP:3713). W2 runs there only if `ordinary_range_trigger` is `Some` **and** `built.nonlinear_supports.is_empty()` (PP:3718-3719).

So W2 comes after both the ordinary attempt and exact-block, on linear invocations only, and the orchestrator is its only entry (`solve_with_force_scaling` at PP:1451 only; the NI pin `F1B_PRODUCT_SITES`). The receipt's `OrdinaryAttempt` is formed after the match (PP:3798):
- on main's paths it is formed from the same value as on main;
- it is read only at finalization (PP:4871-4885).

### 1.3 Admission (Q3) and D11

The nine checks at PP:1521-1581 match RETURN §7, and the order approved at A2.

**Check 1, `user_stiffness_element`: unreachable. I agree.**
- A user element is realized only from an expansion joint with all four stiffnesses (PP:6746).
- `UserStiffnessElement::new` refuses a lateral stiffness that is not positive.
- M07 (`preview_physics.rs:110-146`) refuses every joint with a nonzero lateral over non-coincident nodes, before the case loop.
- Coincident nodes fail pipe validation.

**Check 8, `non_nodal_load_term`: unreachable. I agree.**
- `case_force_ledger` (PP:3138) has six producers. Checks 3–7 cover five of them.
- `push_nodal_loads` (PP:3124) pushes one term per nodal load, sourced by the primitive's id.
- `build_load_case_primitive_loads` creates node or element loads only. The generated equivalent-static loads are element loads, and support-targeted primitives become imposed displacements (`primitive_loads/src/lib.rs:2507`).

**Check 4, `pressure_thrust_load`: reachable. I agree.**
- The legacy refusal fires only for `magnitude.value != 0.0` (`pressure_runtime.rs:214-216`).
- `build_pressure_thrust_loads` (PP:9968) still pushes a zero thrust.
- I13's product run is consistent with this, and `07bed2638` now pins it (§7).

### 1.4 Publication at b ≠ 0

- **Displacements** come from the orchestrator's unscaled solution (PP:1478-1490). They are never scaled.
- **Reactions** come through `force_scaled_reactions` on the orchestrator's stiffness with the unscaled ledger (PP:1601-1620). `reaction_values` places them at the restrained DOFs only (PP:1366), which are the only DOFs the product reads at b ≠ 0 (PP:4062-4070).
- **Spring actions** come through `force_scaled_spring_action` (PP:1621-1632), and **end actions** through `force_scaled_end_actions` (PP:1633-1646). A `PublicationOutsideBinary64` refusal is published by name, never flushed.
- **What is admitted.** Nothing else force-valued is formed at scale:
  - admission leaves only straight frames and nodal loads, so `straight_loads`, thermal loads and pressure thrusts are empty;
  - `exact_straight_end_forces` receives the helpers' values;
  - derived rows are today's binary64 functions (OQ6).
- **The one gap** in this chain is test coverage, not code: RV17-3.

### 1.5 Refusals and templates

`append_force_scaling_refusal` (PP:1766) produces exactly the checkpoint-0 template:
- b is printed as `none` for steps 2–3, and omitted for step 4 and for non-range failures (OQ1 c2);
- the code map follows OQ4 (`integrity_failure_code`, PP:1288, is main's match moved into a function);
- the evidence line (PP:1670) follows Q7, bounded by `NAMED`.

W2RT and `f1b_tests.rs` pin both texts exactly.

### 1.6 `source_recovery.rs`

- `solve_ordinary` (SR:128) refuses an input at b ≠ 0 with zero work.
- Its one non-test product caller passes `ForceScale::UNSCALED` (PP:3615), so D3 holds.
- `prepare_sources` refuses n > 256 by budget (SR:520) before any read of `stiffness`, so the empty view changes neither the charge nor the bytes.

### 1.7 The merge with K5 (`c4879c496`)

K5 adds W4 (`constrained_geometry`) to the four selected branches, dense and sparse, and to the force-scaled siblings (SA:146, SA:575, and their callers).

- **F1b's ordinary attempt** uses the sparse sibling of the formation-checked entry (SA:688). The orchestrator's step 1 uses `solve_force_scaled_with_formation_check` (SA:868). At `UNSCALED` both run the same `constrained_geometry` with the same curved sources, so D1's equality persists.
- **The observation lane** and W2's publication do not touch W4.
- **The suites.** On my archive build of `c4879c496`:
  - PP: 566 passed, 1 failed (the known Mac `t13`), 1 ignored. That includes K5's four `k5_curved_mechanism_runtime` tests, now running against F1b's pattern evidence, and F1b's D-CLASS test;
  - NI: 134 passed.

## 2. RETURN's derivations

**D1, PP's classification equals the orchestrator's step 1: holds.**
- **The assembly.** SA's `evaluate_force_scaled` at `UNSCALED` (SA:1850) forms K with the same function and arguments. Its blocks come from the same macro elements' `global_stiffness`.
- **The evidence.** `new_force_scaled` returns `Self::new` at `UNSCALED`.
- **The solve.** `solve_force_scaled_with_formation_check` uses `f` itself and `formation_source` at `UNSCALED` (SA:1685).
- **The classification.** SA maps `NumericalRange` to `Formation` and `Structural(Range)` to `Evaluation`. SA's third arm needs a scaled ledger or a scaled formation, and neither exists at b = 0.

**D3, the scaled-evidence refusal is unreachable: holds** (§1.6).

**D4, byte identity wherever main publishes: holds. My own derivation:**
1. Every stored K value is main's dense entry bit for bit, with the same first formation error. This is K1 at the product's order; B1–B3 covered 311 cases.
2. Every consumer of K on main's path is replaced by a function of the same values (§1.1): the scan (D6), the partition and force (K1), the observation force (`get`), the attempt (K1's `Debug`-equal `StructuralSolution`, now with K5's W4 on both representations), the dense lanes (`to_dense`), reactions (K1), and the recovery input (the view at n ≤ 256).
3. Every new branch lands where main publishes nothing, or in the two provisional classes:
   - `RangeDeferred`: main's `solver_blocked` blocks the invocation before any case;
   - the W2 arm: after main's blocking `append_integrity_failure`, and the caller's `has_blocking` returns a blocked envelope after each `solve_load_case` (PP:2452, :2481, :2538);
   - the OQ2 decline: only under `RangeDeferred`;
   - the dense guard: an estimate above 6 GiB;
   - the lane guard: 24P above 6 GiB (D12).
4. The receipt's `OrdinaryAttempt` equals main's on main's paths (§1.2).
- **Independent evidence:**
  - gate C3: 832 runs, 0 differences, recomputed by me;
  - my 10 exact-block-selected CX runs, identical on the head;
  - probes m1 and m4, where the clean head equals main.

**D10, coexistence: step 4 fails.**
- Step 2 (formation triggers) holds: `prepare_sources` re-forms the frames.
- Step 3 (the captured entry refuses integral floats above 2^53 − 1) holds as a bound on large magnitudes.
- **Step 4 is refuted (RV17-1).**
- **Step 5 holds, and matters most.** The selected arm (PP:3690) precedes the W2 arm (PP:3713), so any selection publishes main's arm.

**D11: holds** (§1.3), including the correction that check 4 is reachable.

**D12, the lane guard: holds.**
- When the lane runs on main, `basis_solve_states` holds the dense K (8n²) and `reduced` holds the dense reduced K (8n_f²).
- P ≤ n_f(n_f + 1)/2 gives live bytes of at least 40P − 16n_f.
- If 24P > C, main needs more than (5/3)C − 16n_f > C for every n_f < C/24. Above that, the dense K alone exceeds C.
- Step 6's disclosure (an uncapped host) is correct.

**The receipt claim: holds.**
- A receipt is finalized only when a case is selected (PP:2717). A selection needs n ≤ 256 (SR:520), and every case of an invocation has the same node count.
- So P ≤ 32,896, and 24P ≤ 789,504 B, which is far below C.
- The lane guard therefore never fires in an invocation that has a receipt.

**D13, the dense-mode lanes:** 24P ≤ 12n² + 12n < 96n² ≤ C. It holds. The parity lane's true peak (RV17-N2) is at most 20n_f² + O(n), which is still under C there.

## 3. The guards

**The dense-scrutiny guard.**
- **Placement.** The guard (PP:2375-2385) runs after basis 0's sparse formation and before the case loop. Nothing n²-sized exists before it: formation is O(nnz), and the captured parse and model build are O(model).
- **Gate evidence.** The 12 dense 10,000-member runs are refused at a peak RSS of 182–524 MB, where a dense K alone would be 28.8 GB.
- **The constant.** The 96 bytes are right as the n² coefficient (RV17-N3).

**The lane guard.**
- **The estimate.** `observation_lane_profile` (PP:2966) mirrors SD's `from_entries_with_order` (SD:215-270) exactly:
  - each row's first column starts at the diagonal;
  - exact zeros are skipped, and −0.0 == 0.0;
  - the count is Σ(row − first + 1) and the half-bandwidth is max(row − first).
- **The cost.** It is O(nnz), with one first-column array of n_f entries: the disclosed deviation. It stores no profile.
- **Its use** as a firing condition is sound. As a peak bound it is incomplete (RV17-N2).

**ROOT's condition 1 (no case main publishes can exceed the lane ceiling):** derived above (D12), with the uncapped-host disclosure.

## 4. Evidence

**Committed records** (`R/records_checks.txt`).
- `SHA256SUMS` verifies 356 of 356 at `9aeed9c22`, 379 of 379 at `c4879c496`, and 449 of 449 at `f183e1fa9`.
- Trailing whitespace is exactly the disclosed files, with equal per-file line counts: 22 files, then 23 with addendum 1's `MUTANTS.txt`. CHANGE_RECORD and RETURN have none, and each ends with one newline.
- There are no machine paths and no model identifiers.

**The uncommitted raw gate files.**
- The five hashes in `uncommitted_sha256.txt` match, including gate 1's `runs.jsonl` (260 MB) and gate 2's (606 MB).
- `gate2/SHA256SUMS` verifies in full: 3,456 of 3,456 files OK.

**Gate part 1** (`R/gate/c3_check.out`), from the committed tables only (G1's full-envelope base TSV and gate 2's candidate TSV):
- C3: 832 runs, 0 differences in outcome, ok, exit, full sha256, summary sha256 and error hash;
- C1: exactly the 28 runs ROOT ruled, all changed. The table matches the ruled list: 14 published, 6 refused, 8 in the new class;
- C2: 24 runs;
- nothing changed outside C1 and C2.
- The raw full envelopes of 116 sampled candidate runs match the committed hashes (`R/gate/raw_full_check.out`).
- Gate 1 differs from gate 2 in exactly the 4 CONT n10000 sparse runs.
- **Consistent with ROOT's acceptance.**

**Gate part 2.** 8 of 8 runs time out on both sides, with loads recorded (RV17-N5).

**RETURN addendum 2** (`f183e1fa9`; the re-run on `c4879c496`), spot-verified in `R/k5m_checks.txt`:
- **Gate part 1**, from its committed tables: C3 has 832 runs and 0 differences against G1's base; C1 is the ruled 28; C2 is 24. All 884 runs equal the `130445db2` gate's in the six fields.
- The uncommitted `k5m/part1/runs.jsonl` matches its recorded hash (`61b5405d…`).
- **T9:** the 112 output hashes equal the `PLATFORM_CALIBRATION_MAC` hashes.
- **Suites:** PP 566, FK 267 and NI 134, with only the three known Mac failures. My own H-NONE run gives the same PP and NI counts.
- **Consistent with I13's report.** Part 2 was not re-run, by ROOT's direction; K5 cannot shorten a dense run.

**An exact-rational solve of a subset of C1** (the brief's oracle).
- My own closed form for RF-RANGE-THIN-B: u_y = FL³/(3EI), θ_z = FL²/(2EI), θ_x = TL/(GJ), from the binary64 inputs as rationals and π to 50 digits. All four published runs agree to within 6.7e-16 (`R/gate/thin_b_exact.out`).
- R1's exact references for all 14 published C1 runs (RV17-4).

## 5. Tests and mutants

All runs are from clean archives, `--no-fail-fast` (`R/mutants/`):
- NI in full for every run;
- PP in full for NONE, H-NONE, the lane mutant and RV17-M1 to M4;
- PP `cargo check` only for the three F1b pin mutants;
- NI only for the 15 original pin mutants, whose kill sites are NI's.

| Run | Tree | Result | Kill site |
|---|---|---|---|
| NONE | `9aeed9c22` | FK 249, SD 30, NI 121 and PP 561 passed; PP `t13` failed (the known Mac test); 1 ignored | — |
| H-NONE | `c4879c496` | NI 134 and PP 566 passed; `t13` failed; 1 ignored | — |
| F1B-LANE-UNGUARDED | both | **killed** | `f1b_tests.rs:1018` (the mode-row assertion), as in I13's record |
| F1B-PIN-SECOND, -HELPER | `c4879c496` | **killed**; PP `cargo check` clean | `s11k_tests.rs:1651` (the `F1B_PRODUCT_SITES` table) |
| F1B-PIN-NEVER | `c4879c496` | **killed** | `s11k_tests.rs:1631` (`F1B_PRODUCT_NEVER`) |
| The 15 original NI pin mutants (K2B-PIN-LOOP, -PLUMBING, -THIRD; KD5-M32a, -M32b, -E4; S11K-RV-OPT1, -OPT3, -OPT4, -PUB; K1-PIN-BINARY64, -B; K1-PIN-THIRD; K1-PIN-LOOP, -B) | `c4879c496` | **all killed** | the same NI sites as I13's record, 0 lost; one added, K5's `k5_nonlinear_loop_keeps_todays_geometry` for K1-PIN-LOOP-B (`R/mutants/compare_ni_sites.out`) |
| RV17-M1 (deferral on nonlinear invocations) | both | **survives** | RV17-2 |
| RV17-M2 (every spring gets the first spring's action) | `c4879c496` | **survives** | RV17-3 |
| RV17-M3 (the dense recovery view at any n) | `c4879c496` | killed | C-SPARSE's captured peak, `tests/f1b_sparse_pattern_memory.rs:161` |
| RV17-M4 (the finiteness scan reads only the force) | `c4879c496` | **survives** | RV17-N1 |

- **I13's patchers** were reused byte for byte (`R/mutants/patchers_provenance.txt`). Every patch applies once on both heads.
- **None of the three survivors is equivalent.** For each, a product probe shows a byte difference against the clean tree, whose bytes equal main's (`R/probes/probe_compare.txt`).
- **Admission and publication otherwise look well covered.** Each reachable family is pinned by its exact token, and F-A2 and F-S are pinned by name. M6, M7, M8 and DES9 are killed. The LEF-large analogue compares 192 rows bit for bit across 5 members.

## 6. The merges

Both merges add exactly main's delta, with an empty remerge diff (§ Revisions). `c4879c496`'s product combination is §1.7.

## 7. The `pressure_thrust_load` pin (`07bed2638`)

**What the pin must assert, and does** (W2RT:832):
- the range model with a 0 Pa legacy pressure element load is refused on both entries and in both modes;
- the refusal has no rows and exactly one blocking diagnostic, `NUMERICAL_INTEGRITY_UNRESOLVED`;
- the refusal names `pressure_thrust_load`, not `uniform_element_load`, so a mutant that drops or reorders check 4 is killed;
- b = 516 and the step-1 trigger `Evaluation(Range("arithmetic outside normal range"))` are asserted;
- implicitly, the legacy `PRESSURE_MODEL_REAUTHOR_REQUIRED` is absent: it would add a second blocking diagnostic and block the twin;
- the b = 0 twin is `MECHANICS_SOLVED` with main's full-envelope sha256 on both entries.

**Evidence.** I13 reports F1B-M22 killed. On my H-NONE run the new test passes. The platform argument is RV17-N4.

## 8. What I did not do, and disclosures

**Not done:**
- DEC-025 and hosted CI; T9 and the gate on `c4879c496` are I13's addendum 2, which I spot-verified rather than re-ran;
- the src-tauri suite;
- a GEN-8 run (ROOT runs it before the records commit). This file and `R/` give 0 hits under GEN-8's own `MACHINE_ABS_PATH_RE` (`tools/practitioner_harness/surface_roles.py`);
- a re-run of F1b's 32 own mutants, other than those named above;
- a re-run of F1B-M22-PRESSURE-THRUST.

**The recorded gate probes.** For the CX constructions and the m1/m2/m4 main baselines I used the recorded gate probes `577b10d4…` (main) and `f4535939…` (`130445db2`), after checking their hashes. I then confirmed every CX result on my own archive build of the head.

**Procedural disclosures:**
- **`git status`.** I ran `git status --short` twice in `<wt>/f1b`: once at the start, and once later, which revealed I13's uncommitted pin work. At the end I ran it once more, in `<wt>/numerics`, restricted to `T3/REVIEW`, to confirm my files are the only new ones there. It may refresh the index stat cache. Otherwise I used only `git show`, `git diff`, `git archive`, `git log` and `git merge-base`.
- **The system temporary directory.** I wrote one scan file to `<tmp>` instead of my scratch directory, and deleted it at once.
- **The shared scratchpad.** I wrote two diffs (`lib.diff`, `sr.diff`) to the shared session scratchpad, then moved them to `<wt>/scratch/rv17`. I cannot rule out that files of the same names were there before.
- **Stopped runs.** An old-head RV17-M2 run was stopped when the head moved, and left no result. Its tree and target were deleted, including one orphaned test process of mine that I stopped.
- **Workspace.** Everything under `<wt>/rv17` and `<wt>/rv17-target` is deleted at the end (README).

## Delta check at 6fa422979

- **Head checked:** `6fa42297926e7a218a1dc55187895a6784474aa1`, one commit after `f183e1fa9`: tests and records only.
- **Date:** 2026-09-29.
- **Verdict: PASS.**
  - RV17-1 to RV17-4 are resolved. No new BLOCKING or SHOULD-FIX finding.
  - One new NOTE: RV17-D1, a line-citation slip.
- **Records:** `R/delta/` (README, records, platform check, runs); `R/SHA256SUMS` is regenerated over the whole folder.

### D.1 What changed (`R/delta/records_delta.txt`)

- **Under `P/core`,** only W2RT changes: +337 −9, 1,431 lines, sha256 `c350af2e30d40bdb…`, with `git diff --check` clean.
  - `product_physics/src` and `solver/` are unchanged.
  - No file changes outside `P/core` and `T3/IMPLEMENTATION/F1B/`.
- **Records.** F1b's `SHA256SUMS` verifies 470 of 470.
  - Trailing whitespace is exactly the disclosed 24 files, and CHANGE_RECORD and RETURN have none.
  - GEN-8's `MACHINE_ABS_PATH_RE` finds nothing, and there are no model identifiers.
  - I13's three request files (`cx_f`, `m1`, `m2a`) equal mine as parsed JSON.

### D.2 My runs

Each run is from a clean `git archive 6fa422979`, one cargo job at a time, `-j 4`, `RUST_TEST_THREADS=4`, `--no-fail-fast`, with NI and PP in full (`R/delta/mutants/`). The memory guard log is unchanged, and all trees and targets are deleted.

| Run | Result | Kill site |
|---|---|---|
| D-NONE | NI 134 and PP 569 passed; only the known Mac `t13` failed (`s11g_tests.rs:1666`); 1 ignored | — |
| D-F1B-M2 (W2 before exact-block; I13's patcher) | **killed** | W2RT:1274, the positive test's `MECHANICS_SOLVED` assertion: the captured CX-F run is W2-refused instead of selected. W2RT:1098, the unselected side's count: no exact-block attempt, 0 against 1. W2RT:463, the mixed invocation's captured entry returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, an `unwrap` panic; the two assertion kills suffice |
| D-RV17-M1 (`let linear = true;`) | **killed** | W2RT:1350: the diagnostic list, captured and sparse first, gains `LOAD_CATEGORY_PREVIEW_MAPPED` |
| D-RV17-M2 (every spring action the first spring's) | **killed** | W2RT:1422: `spring:N1:UZ` is not −k·u of its own DOF |

- **The three new tests and the pressure pin pass on my build.** So all three full-envelope pins equal Mac main's bytes on an independent archive build of the head: CX-F sparse `d953a683…` and dense `10d312a1…`, m1 `a29e29f2…`, and the pressure twin.
- **Everything reproduces I13's addendum 3:** the PP count, the three kill sites for F1B-M2, and one kill site each for RV17-M1 and RV17-M2.
- **Totals.** F1b now has 41 added tests and 53 counted mutants, all killed. RV17-M4 remains the recorded NOTE survivor.

### D.3 RV17-1: D10 now stands

**D10 (RETURN.md:783-810)** now claims only what is true.
- **Its claim** is byte identity wherever exact-block selects, proven by the arm order: the selected arm precedes W2's arm.
- **The withdrawn step 4** is removed, and the stated reason is the right one: short-mantissa loads make the exact products representable, and M03's scaled right-hand side is about √(f·u).
- **Steps 1–3 remain as context.** Step 3 is labelled a bound sketch, and no conclusion depends on it.

**The new test**, `f1b_w2_exact_block_selection_of_a_range_triggered_case_publishes_mains_bytes` (W2RT:1261-1321), is the brief's positive coexistence test.
- It is built field for field from my CX-F.
- **On the captured entry, in both modes,** it asserts:
  - `MECHANICS_SOLVED` and exactly one `SOURCE_BLOCK_RECOVERY_SELECTED`;
  - the receipt;
  - the info-severity rejected-attempt record with its `Range("division overflow or underflow")` trigger;
  - no `range_scaling` text;
  - main's full-envelope sha256.

**The typed-entry deviation from ROOT's "both entries" wording is right.**
- **The positive property is structural.** Exact-block is eligible only on the captured entry: `source_eligible` needs a capture.
- **On the typed entry the two sides must differ.** Main refuses CX-F with its blocking ordinary Range record (`ca8eda41…`). F1b's W2 then refuses it by name (`853fd2e7…`; `R/coex/head_compare.txt`), which is exactly the C1 kind: a range refusal on a linear invocation that exact-block did not recover. So byte equality on the typed entry is impossible by design, and it is not part of coexistence.
- **The typed half** asserts the W2 refusal's reason and step-1 trigger, which match my runs. The brief's own wording of the test is "a captured, source-eligible case".

**The unselected-side test's** doc comment no longer states A2's general claim, and its assertions are unchanged.

### D.4 RV17-2 and RV17-3

**RV17-2.** `f1b_w2_nonlinear_formation_range_invocation_is_mains_blocked_envelope` (W2RT:1331-1367) pins m1 on both entries and in both modes:
- the exact one-entry diagnostic list;
- no results;
- main's full-envelope sha256.

It kills RV17-M1 at a direct assertion.

**RV17-3: the two-spring test's 1e-9 comparison is sound.** `f1b_w2_two_spring_publication_gives_each_spring_its_own_action` (W2RT:1375-1431) compares each spring's action within 1e-9 of −k·u, not bit for bit. That is sound, for three reasons:
- **Why no exact oracle exists in the test.** The checked helper publishes fl(−k·u) of the solver's own u: k·2^b is exact, there is one rounding at scale, and the unscaling is exact in the normal range. But the test can read u only from the published mm row divided by 1000, which adds two roundings.
- **Why sparse and dense differ.** Their solves return u one ulp apart (skyline LDL against Cholesky; within the DEC-053 basis), so their UY actions differ by one ulp (−4.363400364938612e-290 against −4.3634003649386115e-290, as in my probes).
- **Why the tolerance cannot hide the mutant.** The predicate is the brief's unchanged `|obs − exp| ≤ 1e-9·|exp|`. The defect it guards against is a wrong DOF or a wrong spring, which is off by a factor of about 4.2 here (−4.36e-290 against −1.84e-289 N), far outside 1e-9. The test also asserts that the two actions differ.
- **The checked-helper bits themselves** remain pinned by the earlier M8 tie test.

### D.5 RV17-4, the NOTEs, and the platform independence of the new pins

- **RV17-4** is disclosed under RETURN §9.3's C1 table (RETURN.md:402-408), with my numbers and ROOT's "trusted publications only" ruling. CHANGE_RECORD.md:88 repeats it.
- **N1–N5** are recorded as ruled (addendum 3 §A3.6). RETURN §6.2 (RETURN.md:239) and CHANGE_RECORD.md:32 now say that 24P bounds the identity-order build, not the lane's peak.
- **CX-F is platform-independent on inspection** (`R/delta/platform_check.txt`).
  - The exact-block publication path calls no function of unspecified precision: the only hits are `powi` constants inside `exact_boundary.rs`'s test module, which starts at line 1664.
  - `source_receipt::scaled_norm` uses only `sqrt` and basic operations.
  - Every nonzero row of the CX-F envelope is either one component or a magnitude with exactly one nonzero component (N1 UX, the anchor's Fx, the axial stress), so `hypot` is exact.
  - There is no thermal load.
- **m1** is a blocked envelope with no computed value.
- **Hosted Linux CI remains the arbiter.** Both pins name their re-pin duty in their doc comments.

### D.6 New NOTE

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV17-D1 | NOTE | RETURN.md:804 and :1228 (and the earlier :194, :902, :906): the arm citations `:3689` and `:3714` | At `6fa422979` (and at `130445db2`), the selected arm `Err(OrdinaryFailure::Structural(error)) if selected_source.is_some()` is PP:3690 and the W2 arm `Err(failure) =>` is PP:3713. Each citation is off by one. The argument is unaffected. | Correct the citations when the records are next touched. |

### D.7 Not done, and disclosures

**Not done:**
- FK and SD, which are unchanged since `c4879c496`;
- T9 and the gate: no product file changed;
- hosted CI, DEC-025 and GEN-8.

**Disclosures:**
- I made no Git write. I compared my committed review with the working file by `git show`, not `git status`.
- One cargo job at a time throughout.
