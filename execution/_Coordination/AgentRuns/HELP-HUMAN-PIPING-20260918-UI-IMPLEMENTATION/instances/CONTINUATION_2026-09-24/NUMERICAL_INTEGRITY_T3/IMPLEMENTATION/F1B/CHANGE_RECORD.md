# F1b change record: the product's sparse wiring, the dense-scrutiny guard, and W2 at formation in the product

This is the draft PR record for facade slice F1b of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I13 (TASK). The full account, with every derivation, is `RETURN.md`; the run evidence is `_run_records/`.

- **Branch:** `codex/piping-f1b-20260928`, from main `e7d930d49` (product tree `2b44cdf06…`, equal to K2b's final head `33e33c723`'s).
- **Commits (made by ROOT):**
  - `94e543a24`: checkpoint A1, the sparse wiring and the dense-scrutiny guard (a pure refactor at b = 0);
  - `e215c6007`: checkpoint A2, W2 at formation in the product;
  - `948e0bb99`: checkpoint C, two tests that kill the first-round survivors (tests only);
  - `130445db2`: the DEC-050/053 observation-lane guard (ROOT's ruling on the gate's heap-cap finding);
  - `9ecf2bdca`: these records (checkpoint D), in `T3/IMPLEMENTATION/F1B/` on this branch;
  - `9aeed9c22`: main `b37331092` merged in. The product tree is unchanged: `git diff 130445db2 9aeed9c22 -- P/core P/fixtures P/validation` is empty.
  - `07bed2638`: the `pressure_thrust_load` product pin (tests only) and records addendum 1;
  - `c4879c496`: main `1cdeae2c1` merged in (K5, PR #1044): K5's 13 files only, no F1b file. Re-run on it (`RETURN.md` addendum 2): the 39 manifests (PP 566, FK 267, NI 134; only the three known Mac failures, identical blocks), T9 112/112, and gate part 1 PASS, 884 of 884 runs identical to the `130445db2` gate. Part 2 was not re-run: every dense run times out on both sides, and K5 cannot shorten one.
- **PR:** #1052. **Product candidate:** the branch head `c4879c496`: F1b's product tree of `130445db2` plus K5's merged files. **Size:** 14 files, +5,531 −188 against `e7d930d49` (`RETURN.md` §2).
- **Uncommitted, for ROOT to commit:** this revision of the records (addendum 2).
- **Basis:**
  - the I13 brief (`TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md`, `57cce4f6…`) with ROOT's rulings Q1–Q14 in it;
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`; hash-pinned, not edited): §1, §2.1, §4.7, §4.8, §5 items 5a–8, §6's F1 row;
  - `ROOT_RULINGS_V1.md`, the F1b sections: spawn (`28bb8dfa8`), checkpoint 0 (`2612c3c86`), the A2 stop (`2f48511b7`), A2 (`52ead31ed`), the heap-cap finding (`9fa4d1b59`), and the gate re-run (`74850700d`);
  - K1's F1b interface (`IMPLEMENTATION/K1/RETURN.md` §12) and K2b's (`IMPLEMENTATION/K2B/RETURN.md` §15 with addenda 1–2).
- **Platform:** Mac, arm64 (macOS 26.6.2), aarch64-apple-darwin, rustc 1.97.1. **T9 and the gate are Mac-only comparisons against Mac main.** Nothing was run on Linux or on hosted CI.

## What changes

**In the product (`PP` = `projects/chirality-piping/core/product_physics`) only.** No kernel file changes (FK, SA, SD, SP, CB), and the nonlinear loop is untouched (Q1).

1. **The sparse wiring (W3 at the facade, §4.8).** Each modulus basis and each 0.4.0 resolved case is assembled on the kernel's pattern (`assemble_basis_stiffness` → FK `assemble_sparse_stiffness`), with the curved bends as blocks and the springs in today's order. The partition is `reduce_assembled_sparse_system`, the legacy observation force reads `SparseStiffness::get`, E12's reactions are `SparseStiffness::reactions`, and the linear attempt runs on `SparseAssemblyEvidence` in both modes. No dense matrix is held across cases or bases. Dense views remain only in dense scrutiny (SA's own view, the protected LU lane and the parity rows) and for retained-source recovery at n ≤ 256 (Q9). PP's `multiply_matrix_vector` lost its caller and is deleted.
2. **The dense-scrutiny resource guard (§4.8, Q8, provisional).** Dense scrutiny refuses, once per invocation and before any n² allocation, a model whose estimated dense-path peak (96 bytes per n² entry, counted in the code) exceeds 6 GiB: `SOLVER_SYSTEM_BLOCKED`, naming the estimate. That admits at most 8,192 global DOFs (1,365 nodes). This is a new refusal class for very large dense-scrutiny models (ROOT's provisional product decision; the owner is told).
3. **The DEC-050/053 observation-lane guard (ROOT, `9fa4d1b59`, provisional).** In sparse mode, the legacy observation lane's identity-order profile is estimated in O(nnz) (24 bytes per profile entry, counted in the code). Above the same 6 GiB ceiling the lane is not run: the mode row's seven lane fields are published as `not_observed`, with one new **info** diagnostic, `SPARSE_OBSERVATION_LANE_NOT_RUN`. No result row changes. Within the provisional budget no case main publishes can reach it (`RETURN.md` §15, D12).
4. **W2 at formation in the product (§4.7, K2b §15).** On a linear invocation, K2a's formation `NumericalRange` is deferred from the basis to its cases. A case whose b = 0 attempt range-triggers, and that exact-block did not recover, goes through K2b's orchestrator `solve_with_force_scaling` (the only W2 entry):
   - **admission (Q3):** at b ≠ 0 only straight frames, ground springs, rigid restraints, prescribed motion and authored nonzero nodal loads; any other family is refused by name;
   - **publication:** reactions, spring actions and member end actions through K2b's checked helpers only; displacements are never scaled;
   - **evidence:** a `range_scaling:` line in the integrity diagnostic (Q7, bounded by S11-G's `NAMED`);
   - **refusals:** per case, `NUMERICAL_INTEGRITY_UNRESOLVED` (blocking), under the template fixed at checkpoint 0, carrying the step-1 trigger so K2a's names survive (Q6, OQ1 c2, OQ4).
5. **`source_recovery.rs`:** the shared `DENSE_SOURCE_DOF_LIMIT` (256); `solve_ordinary(input, limits, ForceScale)` refuses force-scaled evidence before any work is charged (OQ3; a defensive guard, unreachable in the product by D3); `range_formation_decline_without_attempt()` declines a formation-range case with zero work (OQ2 option B).
6. **Declared edits outside the new files:**
   - `formation_guard.rs`: `NAMED` becomes `pub(crate)` (OQ8; visibility only);
   - PP `tests/s11f_site_test.rs`: rule-8 counts and rows (Q11), and t10b's anchor (OQ3);
   - PP `tests/k2a_formation_range_runtime.rs`: the linear variants restated (Q10);
   - NI `s11k_tests.rs`: the K2b pin's product half becomes the declared table `F1B_PRODUCT_SITES` plus `F1B_PRODUCT_NEVER` (Q11); the loop halves are unchanged;
   - call-site-only updates in `f1a_tests.rs`, `s11f_tests.rs` and two `source_receipt/*_tests.rs` files (OQ9).
7. **F1a's follow-ups:** N1's comment at the unreachable `row=none` branch; N2 is disclosed below.

**The one approved assertion change** (ROOT, F1b A2, `2f48511b7`): `lib.rs` `tests::audit_nonfinite_computed_mechanics_never_publishes_solved_rows`, third assertion only. Its linear 1e308 N model now range-triggers, W2 publishes at b = −494, and the derived stresses overflow. So the invocation is refused by today's `SOLVER_SYSTEM_BLOCKED` ("computed mechanics must be finite, got inf") with `ELEMENT_FORCE_RECOVERY_FAILED`, and the integrity record carries the `range_scaling:` line. The no-solved-rows assertions are unchanged.

## What differs from the design's letter (ROOT's rulings)

- **The loop stays dense and binary64** (Q1): its base assembly is an M32 remainder owned by T5.
- **W2 runs after the ordinary attempt and after exact-block,** on linear invocations only (Q2). The design does not order them.
- **Admission at b ≠ 0 is narrower than §4.7** (Q3): loads PP forms at b = 0 are not exact inputs. Every exactly-zero nodal term is refused, because the authored value is not available at admission (OQ13 narrowed, disclosed).
- **R-b′ fails closed at b ≠ 0** (Q5(a)): W2-published straight-member cases publish Sensitive at product level.
- **PHYS-R4 is a named refusal, not "passes the evidence stage"** (Q10): the fixture's exact-pressure end-cap operand is subnormal at formation (confirmed by product run: `SubnormalAtFormation`). Without pressure it publishes at b = 536.
- **"LEF-small and LEF-large solved"** is restated per entry: LEF-small stays a model-build refusal; LEF-large is refused at capture on the captured entry and published at b = −702, Sensitive, on the typed entry, bit-related to its base cases by exact powers of two.
- **b in the refusal text (OQ1 c2):** printed only where the orchestrator's outcome carries it; `none` at steps 2–3; omitted at step 4 and for a non-range failure. The b there waits for an SA change (K5 or F2a).
- **The DEC-050/053 lanes are not run at b ≠ 0** (OQ5), and the sparse lane is guarded above the ceiling (the lane guard).
- **The ceilings are provisional** (Q8 and the lane guard): 6 GiB, the gate's heap cap on this Mac, revisited from K6's and V-P's measurements.

## Results (all at `130445db2` unless stated)

- **The both-entry gate: PASS** (ROOT, `74850700d`), against G1's full-envelope Mac base of `e7d930d49`:
  - **part 1 (884 runs):** 0 trusted breaches; C3 832 runs with 0 differences in full-envelope sha256, outcome, exit and standing; C1 exactly the ruled 28 runs; nothing changed outside C1 and C2; C2: all 12 dense 10,000-member runs get the guard's refusal, 8 sparse get named M03 refusals, and the 4 CONT n10000 sparse runs, which aborted at the heap cap on `948e0bb99`, now publish (`MECHANICS_SOLVED`, Sensitive, `needs_recompute`, 620,016 rows, the lane not run, peak RSS 4.82–5.17 GiB);
  - **part 2:** all 8 dense 1,000-member runs time out at 1,800 s on both sides, interleaved base then candidate; no mismatch.
- **The first gate, on `948e0bb99`, FAILED its own condition:** the 4 CONT n10000 sparse runs aborted at the heap cap, in main's unchanged DEC-050/053 lane (an 8 GiB `Vec` growth in `SymmetricProfileMatrix::from_entries`). ROOT's lane guard resolved it; the re-run passed. Everything else in that part 1 equalled the re-run (880 of 884 runs identical).
- **T9 (Mac-only): 112 of 112 committed outputs byte-identical** to main (A1, A2, the lane guard, and re-run on `130445db2` as committed), equal to the Mac calibration hashes; the extra corpus 16 of 16.
- **Suites:** at A2, the 39 manifests `--no-fail-fast` against the K2b Mac baseline: 37 manifests identical; PP 525 → 556 and NI 120 → 121 (new tests only); the only failures are the three known Mac platform tests, byte-identical. At `130445db2`: PP 561 passed, 1 failed (the known `t13`), 1 ignored; NI 121; FK 249; SD 30.
- **Product runs (A2):** every Scope §6 prediction confirmed. RF-RANGE 128 runs: 100 full-envelope identical, 28 changed = the C1 list (14 published, 6 refused by name, 8 published at b ≠ 0 and then refused by today's derived-row non-finite check). The extra corpus: 220 runs, the 116 changed all refused on main.
- **The b-rule (Q12):** of 38 `ScaledEvaluation` refusals probed at every even b in [−1100, 1100], only K2b's documented limitation chain has a solving b. It stays a documented limitation.
- **RV11D-N2 (the b = 0 availability cost):** 0 refusals and 0 bit mismatches in 766 runs (3,666 reactions, 251 spring actions, 3,456 end-action sets), plus 8 of the 13 gate cases of 1,000 or more members, including CONT n10000 (15,006 reactions and 10,000 end-action sets each). The other 5 publish nothing at b = 0.
- **Mutations:** 32 F1b mutants and 18 original pin mutants, NONE first, from clean archives: all killed at behavioural or pin assertions. Two first-round survivors were killed on re-run by two new tests (`948e0bb99`). The original pins lose 0 kill sites. The lane mutant `F1B-LANE-UNGUARDED` is killed, re-run on `130445db2` as committed.
- **Memory:** sparse mode is pattern-only (C-SPARSE: 4,000 members, peak 825 MB against a 1 GiB bound; the dense view alone is 4.61 GB). Of the sparse path's about 206 KB per member, the solver holds at most about 27 KB; the rest is result-row publication (`RETURN.md` §12).

## Limits and disclosures

- **New refusal classes (provisional):** dense scrutiny above 6 GiB estimated; and the lane's fields `not_observed` above 6 GiB (on an uncapped host, a model main could publish, like CONT n10000 at about 45 GB on main).
- **New info code** `SPARSE_OBSERVATION_LANE_NOT_RUN` (ROOT-accepted). No consumer enumerates a closed code set that would reject it (`RETURN.md` §16).
- **Deviation from "no allocation"** in the lane estimate: one first-column index per reduced DOF (8 bytes per DOF), accepted.
- **Correction to A2's "three admission checks are unreachable":** two are (`user_stiffness_element`, `non_nodal_load_term`). `pressure_thrust_load` is reachable, through a zero-valued pressure element load on the legacy route: refused by name on both entries and both modes, where main refuses the same runs; its b = 0 twin is byte-identical (`RETURN.md` §15, D11). No product change. ROOT approved a product-level pin after D: `f1b_w2_admission_refuses_a_zero_legacy_pressure_as_pressure_thrust` (tests only), with the mutant `F1B-M22-PRESSURE-THRUST`, killed (`RETURN.md` addendum 1). With it, PP passes 562 (1 known Mac failure), NI 121, FK 249, SD 30 (NONE, from a clean archive).
- **C-SPARSE departure (approved):** 4,000 members, a 1 GiB bound and a 6 GiB cap, instead of 1,000 members and 64 MiB (`_run_records/checkpoint_a1/c_sparse_scaling.txt`).
- **F1a N2:** `source_receipt.rs:914-921` reserves 12× the bytes of every envelope diagnostic. F1b's new text reaches a receipt only in a captured invocation that selects one case and publishes another at b ≠ 0 (the mixed invocation: +1 `range_scaling:` line, at most about 600 bytes). W2 refusals block, and the guard refusals are blocked envelopes, so neither reaches a receipt; the lane diagnostic cannot co-occur with a selection (n ≤ 256).
- **Pre-existing NOTE for T3's close list:** the mode row's format string on main has a double space after `legacy_unscaled_DEC050_DEC053;` (`lib.rs:4502` on `e7d930d49`, `:5459` on `130445db2`). F1b leaves it unchanged.
- **Not re-run on `130445db2`:** the 38 manifests other than PP (and NI, FK, SD, which were re-run). C and the lane guard changed only PP. DEC-025's Mac sweep covers the rest.
- **Left to others:** the loop's pattern move (T5); a scale-aware R-b′ bound, PHYS-R4 with pressure, W2 beyond nodal loads, and the b-rule refinement (T3-close list); b on the step-4 and non-range refusal paths (K5 or F2a); the ceilings (K6, V-P).
- **Procedural disclosures:** `/tmp` used twice briefly (deleted); a short three-cargo-job breach at C; NONE and the lane mutant run concurrently on the pre-format bytes (re-run properly on `130445db2`); one `git status`.
- **Pending the reviewer's check:** D1 (classification equality), D4 (byte identity by construction), the coexistence derivation (D10, the M31b lesson), the unreachability derivations (D3, D11), and the lane-guard derivation (D12).
