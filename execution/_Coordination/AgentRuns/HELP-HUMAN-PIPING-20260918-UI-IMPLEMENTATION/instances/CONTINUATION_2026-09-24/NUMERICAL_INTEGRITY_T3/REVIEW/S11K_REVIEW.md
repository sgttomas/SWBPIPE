# RV1: independent full-diff review of slice S11-K

RV1 (Type 2 TASK), 2026-09-26. Brief: `T3/TASK_BRIEFS/RV1_S11K_REVIEW.md`, with its Addendum (C3), read with `T3/TASK_BRIEFS/_COMMON.md`. I did not design S11, review its design, or implement S11-K. I fixed nothing, made no Git write, and edited nothing in `<s11k-worktree>`. Every mutation and producer run happened in scratch copies under `<scratch>`.

## Verdict: PASS

There are no BLOCKING findings. There are **3 SHOULD-FIX** and **9 NOTE** findings.

- The code implements S11 revision 5a.2 §8.1 as selected. I found no 5a.x change that bears on S11-K and is missing.
- Every accumulator and E-site output I checked equals my own correctly rounded `Fraction` reference: 2176 cases, 0 mismatches.
- The committed fixture diff matches I1's pre-regeneration measurement file for file and leaf for leaf. The actual producers regenerate the committed bytes.
- The option (c) variants behave byte for byte like the merge-base code.
- Two findings concern how strongly option (c) is pinned (RV1-S1, RV1-S2). One is a records gap (RV1-S3).

## Revisions reviewed

| Item | Revision |
|---|---|
| Candidate (PR branch `codex/piping-s11k-pr-20260926`) | `4912dc6368be87636334162acae6e64ca7427ac2` |
| Merge base (origin/main at the cut) | `6bb3ee4903977ed316bcf872c4f04a461349d2fe` |
| Diff reviewed | `git diff 6bb3ee490..4912dc636`: 95 files, 52 code/lockfile/test/fixture files and 43 records under `T3/IMPLEMENTATION/S11K/` |
| Development branch `codex/piping-s11k-20260926`: head and cherry-pick sources | head `65f3dcdef`. The PR commits `8167f7926`, `201b11bfc`, `9c8fcbe48`, `6717b9e1d` and `412469de0` carry `-x` trailers for `14354efdb`, `e517fd9b6`, `41bfcab85`, `9be58ea95` and `65f3dcdef`. `4912dc636` is new (a note on the base and a sanity check). `git diff 65f3dcdef 412469de0` over the 95 paths is empty. |
| Development base | `163cd44ab`. `git diff 163cd44ab 6bb3ee490 -- projects/chirality-piping ':!…/execution'` is empty, so I1's measurements carry over to the PR base. T1's `e43412a9b` is an ancestor of `6bb3ee490`. |
| New PR head (ROOT's administrative note) | `f766432351b224115014a2d2451ba320cfabba40`, a merge of origin/main `efe938506` into `4912dc636`. **`git diff 4912dc636 f76643235 -- projects/chirality-piping` is empty** (checked with `git diff --quiet`). The object was already present locally, so no fetch was needed. |
| Basis | T3 branch at `3b2fbe022`. `S11_CONTAINMENT.md` revision 5a.2: sha256 `e6507587f3e97f6f28ecea213d5a21636235c66822c69e967c283248a3272ef4`, identical to its blob at `932698d7a`. |

## Findings

| ID | Severity | Site | Evidence | What would resolve it |
|---|---|---|---|---|
| RV1-S1 | SHOULD-FIX | `nonlinear_integration/src/s11k_tests.rs` `option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path` (the option (c) loop pin) | The pin is a substring check over the raw body text of `solve_linearized_system_evidence`. Comments are not stripped, and calls made through helpers or aliases are not seen. **Mutant RV-OPT4** routes the loop's assembly solve through a helper `exact_solve(…)` (exact KS1/KS3) and keeps `// assembly.solve_binary64(` as a comment. The pin **passes**. Only two unrelated existing gap-inspection tests fail. **RV-OPT1** (loop reduction switched to the exact KS2) and **RV-OPT3** (the no-assembly dense and sparse loop solves switched to the exact solves) are killed **only** by this source pin: `validation/benchmarks/nonlinear` (DEC-046) and every other nonlinear test pass. So the same comment-and-helper workaround on those targets would pass every test. | (a) Harden the source pin: strip comments and string literals first (reuse `lex` from `frame_kernel/tests/s11_site_table.rs`), and require the four `_binary64` targets as real calls. Then forbid every exact kernel entry point anywhere in `nonlinear_integration/src/lib.rs` outside `#[cfg(test)]`: `reduce_system_with_prescribed_displacements(`, `.solve(` and `.solve_assembled(` on `AssemblyEvidence`, `solve_structural_dense(`, `solve_structural_sparse(`, `prepare_structural(`, `prepare_assembled_structural(`, `solve_assembled_structural_dense(`, `evaluate_assembled_original_residual(`, `*_with_force_terms(`. Scanning only one function body is not enough. (b) **Add a behavioural pin.** In `nonlinear_integration`, run an active-set case with a closed gap (nonzero prescribed displacement) whose binary64 fold of `f − ΣK·g` differs from the exact value. The DEC-046 two-span opposing-gaps inventory case, or a probe-P-like invented model, would do. Run it dense and sparse, with and without `AssemblyEvidence`. Assert the precondition (exact ≠ binary64). Then assert that the loop's reduced force, displacements and residual rows equal the binary64 legacy values bit for bit, taken from an in-test fold or a frozen expected. |
| RV1-S2 | SHOULD-FIX | `frame_kernel/src/structural/s11k_tests.rs` `option_c_public_original_residual_stays_binary64_on_coupled_rows` | This is the named pin for the public binary64 `evaluate_original_residual`, and it is vacuous. **Mutant RV-PUB** switches the public function to the exact KS3 numerator (`ForceBinding::Legacy`), and **all 116 frame_kernel tests pass**. At the test's `u`, the binary64 expression and the exact rounded numerator are bitwise equal, so the test breaks the precondition rule (S11B-6). The behaviour itself is protected elsewhere: RV-PUB fails DEC-046's `multisupport_acceptance_inventory_uses_narrow_dec_046_policy` and two nonlinear gap-inspection tests. That is why this is not BLOCKING. | Choose the test's `u`, for example by perturbing `u[11]` or `u[7]` by a few ulp or using a different g, so that on row 11 the binary64 expression differs from the exact numerator. Add `assert_ne!(r.to_bits(), acc.round_scaled(-e).unwrap().to_bits())` inside the test before asserting that the public row equals `r`. Re-run RV-PUB to confirm the test now fails. |
| RV1-S3 | SHOULD-FIX (records) | `T3/ROOT_RULINGS_V1.md` at `3b2fbe022`; `CHANGE_RECORD.md` "write set extended by ROOT" | The brief and the Addendum cite ROOT's 2026-09-26 regeneration approval and hash-pin approval in `ROOT_RULINGS_V1.md`. **No such entry exists** there or anywhere in T3's records: I searched for "hash pin", "go-ahead" and "regenerat… ROOT". The candidate's commit `9c8fcbe48` says "(ROOT-approved)", and `CHANGE_RECORD.md` asserts the write-set extension without citing a record. I verified the substance independently: only producers regenerated; the two pins equal the new raws' sha256 and are the only test edits; no `node_modules` or scratch output is committed. | The manager records ROOT's 2026-09-26 approvals verbatim in `ROOT_RULINGS_V1.md`: regeneration by the actual producers only; the write set extended to the two hash-pin constants; the local TS run with the `node_modules` links removed before commit. Then `CHANGE_RECORD.md` cites that record by commit. |
| RV1-N1 | NOTE | `frame_kernel/tests/s11_site_table.rs` header and its zero-count rows | The scanner matches only `+=`, `-=`, `.sum(`, `.sum::<` and `fold(`, which is exactly the design's rule 8 pattern. A fold restored as `s = s + v` or `b = checked_value(b - …)` is invisible to it. Mutants RV-M1a, RV-M1c and RV-M1d (E1, E3 and E4/E6 folds written that way) **pass the site table**, and the numeric K4 tests kill them. The header claim "a restored binary64 fold there fails this test as well", and the zero rows for `prepare_bound` and `evaluate_original_residual_bound`, overstate what the scan can see. KS1's and KS3's legacy folds are written in exactly that non-compound form. | Reword the header to state the limit. Optionally extend the pattern to self-assignment (`x = x ± …`, `x = checked_value(x ± …)`) and re-baseline the counts. |
| RV1-N2 | NOTE | site table rows `FK/lib.rs reduced_right_hand_side` and `FK/structural.rs prepare_bound` | The dispositions say "b − (±0) is exact" and "legacy rows without nonzero prescribed product". The same code is also the option (c) `Binary64` variant, which folds nonzero products on purpose. | Add "and the option (c) binary64 legacy variant (nonlinear loop)" to both dispositions. |
| RV1-N3 | NOTE | `RETURN.md` §2 and §4, `CHANGE_RECORD.md`, the mutation records | The M1 labels differ from the selected text. 5a.2 §9 uses M1e = E6 and M1f = E5; I1 uses M1f = E6. I1's M1d (E4) and M1f (E6) are one SP function split by component, force versus moment. Revision 5a.2 does note that E6's sum is in SP. | Relabel M1f to M1e in the records, and state that E4 and E6 share `station_resultants_from_i_end_with_spans`. |
| RV1-N4 | NOTE | `CHANGE_RECORD.md` header; `_run_records/run_suites.sh.txt` | The header names the development branch, base `163cd44ab` and S11 revision 3. The selected basis is revision 5a.2 (C3), and the PR branch and base appear only in the last section. "Checks run" says `--no-fail-fast`, but the recorded script runs `cargo test --offline --locked` without it. Every suite passed, so the flag makes no difference to the result. | Put the PR branch, base `6bb3ee490` and basis 5a.2 (`e6507587`) in the header, and record the command actually run. |
| RV1-N5 | NOTE (dormant; for S11-F) | `structural.rs` `audit_load_fidelity`, called with `?` from `finish_checked_factor` on the `Assembled` and `AuditTerms` bindings | The audit rescales each term with `exact_radix(…, −exponent)`, which returns `Range("exact radix loses represented bits")` when a term is more than about 2^1000 below the row's largest term or product, for example a ledger row (1e80, −1e80, 1e-300). That turns a solve into `Err`, against §6's "never a refusal". It is unreachable from the product in S11-K. | Before S11-F wires the typed path, make the audit non-fatal: accumulate with `ExactAccumulator` instead of radix-scaled expansions, or map an audit range error to "Sensitive, unaudited row". |
| RV1-N6 | NOTE | `structural.rs` naming | The unsuffixed `evaluate_original_residual` is the binary64 variant, while `reduce_system_with_prescribed_displacements`, `prepare_structural` and `solve_structural_dense` are exact and their `_binary64` twins are the legacy ones. A future linear caller of the public residual would silently get binary64. The doc comment states this. | Consider `evaluate_original_residual_binary64` plus an exact default, or keep the name and rely on RV1-S2's repaired pin. |
| RV1-N7 | NOTE | `provenance/build-artifacts/core__loads__load_case_algebra__Cargo.lock` | This snapshot equalled the real lockfile at the base and now lacks the new `open_pipe_stress_frame_kernel` line. Nothing outside `execution/` references these snapshots, and most of them already differ from their lockfiles at the base. | None needed, unless a loop maintains these snapshots. |
| RV1-N8 | NOTE | `compile_fail` doctests in `structural_adapter.rs` (`solve_assembled`) and `nonlinear_integration/src/lib.rs` (`solve_active_set_frame_assembled`) | Unlike FK's, these have no positive twin, so they cannot show that the compile failure is the refused `Vec<f64>` and not some other error. I read both, and the only mismatch I can see is the force type. | Add a positive twin that compiles with an `AssembledForce`. |
| RV1-N9 | NOTE (disclosure) | `SP` `station_resultants_from_i_end_with_spans` (E4/E6) | An incoming −0.0 i-end N, Vy, Vz, My or Mz at a load-free station now publishes +0.0 (the E-site zero rule). T stays a passthrough. This agrees with §4.1.3, and no committed fixture changes. PP's E6 path passes published end forces, which today can be −0.0 through sign conventions. | Optionally add one sentence to the change record: the E-sites now publish +0.0 for any zero, including a −0.0 carried in from an end action. |

## 1. §8.1 items

- **`exact_sum.rs`.** I read `project` line by line.
  - Limbs and range: 68 limbs, `QUANTUM_EXPONENT = −2148`. The largest product sits at bit 4195. The significand is a 106-bit `u128`.
  - `round_scaled` places the binade quantum at `max(e − 52, −1074)` and rounds once, ties to even, with a sticky bit below. There is no copy-bits shortcut.
  - It returns +0.0 for an exact zero and for any underflowed nonzero value, and `NonRepresentable` above the range.
  - The accumulator overflow check is `get_mut(word)`.
  - `add(−0.0)` and zero products are skipped. The sign is split correctly for products.
  - My exact check agrees (§5).
- **`load_ledger.rs`.**
  - `AssembledForce`: private fields, derives only `Debug`, no public constructor, no `Clone`, `From` or `Default`, no `&mut`.
  - `ReducedForce::from_exact_rows` is `pub(crate)`.
  - `LedgerEvidence::underflowed_dofs` is recorded when `value == 0.0 && !is_zero()`.
- **Typed seams beside today's.**
  - `reduce_assembled_system*`, `StructuralSystem::assembled` → `AssembledStructuralSystem` (the manager's decision 2a), `prepare_assembled_structural`, `solve_assembled_structural_dense`, `evaluate_assembled_original_residual`, `AssemblyEvidence::solve_assembled`, and the three nonlinear `*_assembled` siblings.
  - The nonlinear siblings compare every bit, so −0.0 and +0.0 are treated as different; on success they delegate.
  - No `&[f64]` entry point was removed.
- **KS1–KS3.**
  - KS2: `reduced_right_hand_side`. KS1: `exact_scaled_rhs` via `prepare_bound`. KS3: the `exact_numerator` branch of `evaluate_original_residual_bound`.
  - On `Legacy`, a row takes the exact path only when some prescribed pair has K ≠ 0 and g ≠ 0. Uncoupled rows run today's expression, so the sign of zero is unchanged (pin `legacy_all_zero_prescribed_rows_keep_todays_zero_sign`).
  - `Assembled` rows are always exact.
- **E1–E4, E6 (SP).**
  - `exact_array_sum` covers E1, E2 and E3 (local − each load term − each axial term, in one rounding).
  - `exact_terms_sum` covers E4/E6: M_i, fl(V·d) and each load's station term. T stays a passthrough.
  - For one load, E1 equals today's term bit for bit (K4 test, and my reasoning: 0.0 + t = t, and −0.0 becomes +0.0 in both).
  - The only folds left in SP outside tests are the three transforms (`:1462`, `:1490`, `:1500`), which are declared formation.
- **E13.** `add_product(c, q)` per term, rounded once. A non-finite operand keeps today's fold. An out-of-range net becomes ±∞ by the exact sign.
- **CB.** `arc_section_resultant_terms` (dormant). Its thrust formula is identical to `arc_section_resultants_with_radial_pressure`'s.
- **`pressure_sum`.** Now a wrapper only. Its operands are multiples of 2^-1074, so the old projection, including its `highest < 52` path, gives the same results. Its tests are unchanged.
- **Zero witness.** The only `-0.0` literal in the added non-test code is at `audit_intended_action` (the base's `:554`), guarded by `Expansion::is_zero()`. `Expansion` removes zero terms, so an empty expansion is exactly the exact-zero case.
- **`Expansion::rounded()`** is removed and `round()` is used at all three FK sites.
- **`exact_boundary`** `:361` and `:387` also accept `exact_rounded_sum` of the same terms.
- **PP.** Only `pressure_sum.rs` changes in `product_physics`. PP compiles unchanged: product_physics passes 448 tests with 1 ignored in source.
- **Write set.** Every changed path is in I1's write set, or is one of the approved regenerated fixtures (20 files) or the two hash-pin constants (`result_export/tests/load_reference_contract.rs` and `tests/test_load_reference_readers.py`).

## 2. The R3 conditions

- **R3-1.** Every changed fixture is pre-registered (A, B2) or ROOT-accepted (B1), listed in §7. Every other committed request is byte-identical on I1's evidence. I re-derived the list of changed files from the diff; I did not re-run the whole-corpus fixture diff.
- **R3-2.** `k4_axial_effect_case_kills_e2_and_the_axial_half_of_e3` kills I1's M1b and M1c at G = 1e8 and at 1e80 separately. It also kills my RV-M1c, which keeps two roundings: exact local-minus-loads, then a binary64 subtraction of the axial pairs.
- **R3-3.** The site-table test is present and passes. Its table covers FK (lib, structural, exact_boundary, load_ledger, exact_sum), SA, `nonlinear_integration/lib.rs`, SP, CB and `load_case_algebra`, keyed by (file, function, exact count).
  - The unit-force row is justified: it covers `solve_iteration_with_sliding_friction_evidence`, count 2, with `unit_force[…] += 1.0` at `:1399` and an integer coupling pattern at `:1444`. It is T5's, as ROOT allowed.
  - The limits are in RV1-N1 and RV1-N2.
- **R3-4.** KS1 and KS3 both call `round_scaled(exponent)` before their single rounding. My 80 `SCALED` cases, with exponents −1100 to +1000 across the subnormal boundary, all match the reference.

## 3. R5-3: callers of the KS-affected functions

**Search.** A `grep` of every `.rs` under `projects/chirality-piping` outside `execution/` for `reduce_system_with_prescribed_displacements`, `reduce_system(`, `prepare_structural`, `solve_structural_dense`, `solve_structural_sparse`, `finish_structural`, `evaluate_original_residual`, `factor_structural_*`, `negative_pair_witness`, `AssemblyEvidence` and `apply_linear_supports`. I separated test code by each file's `#[cfg(test)]` boundaries and traced each non-test hit to its caller.

**My independent list** (non-test code, candidate line numbers):

| Caller | Calls | Class | Path after S11-K |
|---|---|---|---|
| PP `solve_load_case` `:2333` | `reduce_system_with_prescribed_displacements` (load_state present) | linear | exact KS2, live on T1 support motion |
| PP `solve_load_case` `:2340` | `reduce_system` | linear, zero prescribed | legacy expression, bit-identical |
| PP `solve_preview_reduced_system` `:3965` (reached from `:2360`) | `AssemblyEvidence::solve` → `solve_structural_dense` or `sparse_direct::…::solve_structural_sparse` → `prepare_structural` + `finish_structural` | linear | exact KS1/KS3 on coupled rows |
| `linear_supports::apply_linear_supports` `:467` | `reduce_system_with_prescribed_displacements` | linear | exact KS2 |
| `linear_supports::validate_global_system` `:487` | the same, with no prescribed DOFs | linear | bit-identical |
| `sparse_direct/src/structural.rs` `solve_structural_sparse` `:21`, `:26` | `prepare_structural`, `finish_structural` | linear (SA `solve`) | exact |
| SA `solve` `:297`, `:301`, `:303` | `_with_force_terms` / dense / sparse | linear (PP) and a dormant audit seam | exact |
| SA `solve_binary64` `:334`, `solve_structural_sparse_binary64` `:382`, `solve_prepared` `:392–401` | `prepare_structural_binary64`, factor, `finish_structural` | nonlinear | legacy |
| SA `scrutinize_gaps` `:1146` (`pub(crate)`, called only from `nonlinear_integration/src/lib.rs:690`) | `product_equilibrium::evaluate` → public `evaluate_original_residual` | nonlinear | legacy binary64 |
| `product_equilibrium::evaluate` `:56` | public `evaluate_original_residual` | nonlinear (callers: lib `:2025`, SA `:1146`) | legacy binary64 |
| `nonlinear_integration` `solve_linearized_system_evidence` `:1968`, `:1981`, `:1999`, `:2004`, `:2025` (its callers `:1367`, `:1400`, `:1483`) | the four `_binary64` targets and `product_equilibrium::evaluate` | nonlinear | legacy |
| PP `:3639` | `solve_active_set_frame_with_mode_and_springs` (reaches the loop) | nonlinear | legacy |
| `validation/benchmarks/nonlinear` (12 references to `nonlinear_integration`) | the loop | nonlinear | legacy |
| `validation/benchmarks/mechanics` | `reduce_system` ×10 (`:1679` … `:4156`); `apply_linear_supports` ×8 (`:4480`, `:4558`, `:4632`, `:4751`, `:4861`, `:4980`, `:5221`, `:5457`) | linear | exact on coupled rows (suite passes 41/41) |
| `performance_harness` `:773`, `:959`, `:966` | `reduce_system` | linear, zero prescribed | bit-identical |
| FK `structural.rs` `:1510`, `:1517`, `:1531`, `:1540` | internal composition | — | — |

**Test-only callers, excluded:**
- PP `:13444` and `:20766`;
- CB `:1188` and `:1626`;
- `rigid_body.rs:314`;
- `linear_supports:950`;
- nonlinear lib `:5473`;
- `sparse_direct/src/lib.rs` `:848` and `:888`;
- `performance_harness` `:1912` and `:1994`;
- `product_equilibrium` `:122–153`.

`source_recovery.rs` builds `AssemblyEvidence` (`:836`) but never calls `solve`. headless, `result_export` and src-tauri call none of these functions directly.

**Comparison.** My list agrees with I1's `ks_callers.txt` and `PRE_REGENERATION_REPORT.md` §2, and with S11 revision 5a.2 §8.1, including `product_equilibrium::evaluate` and `linear_supports::apply_linear_supports`. **No caller is missing and none is misclassified.**

## 4. The option (c) pins

- **Reach.** Every KS call in the nonlinear loop goes through `solve_linearized_system_evidence`, which calls only `reduce_system_with_prescribed_displacements_binary64`, `assembly.solve_binary64`, `solve_structural_dense_binary64`, `structural_adapter::solve_structural_sparse_binary64` and `product_equilibrium::evaluate`. `product_equilibrium::evaluate` and SA `scrutinize_gaps` reach only the public binary64 `evaluate_original_residual`.
- **Byte identity with the base source,** compared against `6bb3ee490`, not only against the tests:
  - The `ForceRows::Binary64` branch of `reduced_right_hand_side` is the base loop verbatim. The rest of `reduce_system_for_boundary` is unchanged apart from its return type.
  - The `Binary64` branch of `prepare_bound` is the base KS1 expression verbatim: `checked_value(b − checked_product(K, u))`, then `radix_scale`. Nothing else in `prepare` changed except the new `force_binding` field.
  - `finish_checked_factor` on `Binary64` evaluates the base residual (`exact_numerator` is false), runs no audit, and sets `load_fidelity: None`. `quality` is as before.
  - SA `solve_prepared` and FK `solve_prepared_dense` match the base `solve_structural_dense` (cholesky, then negative-pair witness, then finish) and the base `sparse_direct::structural::solve_structural_sparse` (`factor_structural_ldlt`, then witness, then finish) step for step.
  - `AssemblyEvidence::solve_binary64` builds the same `StructuralSystem` as the base `solve`, and `symmetry_basis()` returns the base's format string byte for byte.
  - The public `evaluate_original_residual` binds `Binary64`, which is the base expression.
- **Textual workaround and behavioural backing.** RV1-S1. The source pin can be satisfied by a trivial textual workaround (RV-OPT4), and for the reduction and the no-assembly solves the source pin is the only thing that fails (RV-OPT1, RV-OPT3). **A behavioural pin should back it.** The public-residual pin is vacuous (RV1-S2).
- **Protected benchmarks.** `validation/benchmarks/nonlinear` and the DEC-046 limits are untouched: no file under `validation/` changes except two lockfiles. The suite passes 19/19 on the candidate.

## 5. Exactness spot check (standard-library `fractions`)

**Method.** A scratch probe crate (`rv1_probe_main.rs.txt`) links the candidate's `frame_kernel`, `straight_pipe` and `load_case_algebra` and runs the code under review on bit-exact inputs. `rv1_exact_check.py.txt` builds each reference from `Fraction` with its own round-half-even to binary64, and applies the S11 zero rule: an exact zero or an underflowed value is +0.0. I self-tested that rounding against `float(Fraction)` on 20,000 random rationals, including the subnormal range: 0 disagreements.

**Results.**
- Accumulator and E13: **2071 cases, 0 mismatches.** The binary64 fold differs from the correct value in 315 of them, so the check is not vacuous.
- `straight_pipe` E1, E2, E3 and E4/E6: **105 cases, 0 mismatches.** The fold differs in 76.

**Coverage.**
- Cancellation at G = 1e7, 1e8, 1e80 (and 1e300) in the orders (G, n, −G), (n, G, −G) and (G, −G, n). Also V1's probe D and the thermal pair.
- Subnormal nets: 60 random subnormal sums, and 40 dot products in the underflow range.
- Probe X, as products: 3·2^-1076 → 2^-1074; a tie at 2^-1075 → +0.0; 2^-1075 + 2^-1100 → 2^-1074; −2^-1080 → +0.0; (1.7e308)² − (1.7e308)² → +0.0. Also max-cancellation and overflow (an error).
- Signed zero: [−0], [−0, −0], [], [1e8, −1e8] and [−1e80, 1e80, −0] all give +0.0 bits.
- **Order independence:** each sum and dot case also ran reversed and in 2–3 random shuffles, with identical bits every time.
- `round_scaled` exponents from −1100 to +1000.
- E13, including non-unit factors.
- E1 over X/Y/Z uniform, partial-span and point loads.
- E3 through the `recover_end_resultants_with_spans_and_axial_effects` route at both ends.
- E4 at stations 0.25, 0.5, 0.75 and 1.0, with Y, Z and X uniform loads, partial spans and point loads, plus zero-sign stations.

**Precondition rule.** Every fold-killing S11-K test asserts that the binary64 fold differs from the correctly rounded net: K4 (three tests), P1's probe, K6, K7 (`differs_somewhere`), K8, K9, K11 (`precondition()`), and `option_c_binary64_variants…` (`assert_ne!` of exact against legacy). **The one exception is `option_c_public_original_residual_stays_binary64_on_coupled_rows`** (RV1-S2). The P1 probe's precondition only asserts that the fold differs (`> 0`), not that it breaches 1e-9. That is acceptable: G = 1e7 is informative, not a kill requirement.

## 6. Mutations

**I1's records.** I read `mutate.py.txt` and `mutation_results_final.json`. Every listed mutant fails at least one test, with M1a–M1f and M1m run at G = 1e8 and G = 1e80 separately. The site table also fails for I1's compound-assignment M1 mutants, except M1m.

**My sample,** run in a scratch copy with `--no-fail-fast` (`rv1_mutation_results.json`):

| Mutant | Mutation | Killed by |
|---|---|---|
| RV-M1a | E1 fold as `a[d] = a[d] + t[d]` | `k4_e1_single_load_is_bit_identical_and_cancelling_loads_are_exact`, at G = 1e8 and G = 1e80 each. The site table passes (RV1-N1). |
| RV-M1c | E3 in two roundings (axial half in binary64) | `k4_axial_effect_case_kills_e2_and_the_axial_half_of_e3`, at 1e8 and 1e80 each |
| RV-M1d | E4/E6 moment-z fold as `s = s + v` | `k4_probe_a_end_forces_and_stations_are_exact_at_the_kill_set`, at 1e8 and 1e80 each. The site table passes (RV1-N1). |
| RV-M1m | E13 publishes the fold | the three K6 tests, at 1e8 and 1e80 each |
| RV-M13 | KS2 `if true` (binary64 on every legacy row) | `k11_reduced_right_hand_side_is_correctly_rounded`, `option_c_binary64_variants_keep_todays_fold_on_probe_p` |
| RV-M14 | KS3 numerator binary64 | `k11_forced_refinement_step_stays_exact` |
| RV-M12L | KS1 binary64 on legacy coupled rows only | `k11_dense_profile_and_typed_structural_paths_are_exact`, `option_c_binary64_variants…` |
| RV-OPT1 | loop reduction → exact | **only** the source pin `option_c_nonlinear_loop_is_pinned…`. DEC-046 passes. |
| RV-OPT2 | loop `assembly.solve_binary64` → `solve` | the source pin, plus two existing `unsupported_gap_inspection_*` tests. DEC-046 passes. |
| RV-OPT3 | loop dense and sparse (no assembly) → exact | **only** the source pin |
| RV-OPT4 | exact solve through a helper, required text kept in a comment | **the source pin passes.** Only the two `unsupported_gap_inspection_*` tests fail. |
| RV-PUB | public `evaluate_original_residual` → exact | **no FK test.** The two `unsupported_gap_inspection_*` tests and DEC-046 `multisupport_acceptance_inventory_uses_narrow_dec_046_policy` fail. |

Every E-site and KS mutant is killed by the named behavioural test. The findings are RV-OPT1, RV-OPT3 and RV-OPT4 (RV1-S1) and RV-PUB (RV1-S2).

## 7. Fixtures and regeneration

**Committed diff against I1's measurement.** `rv1_leafdiff.stdout.txt` covers the 20 changed JSON files. Byte counts and changed-leaf counts match `PRE_REGENERATION_REPORT.md` §5, `RETURN.md` §5 and the `CHANGE_RECORD.md` table exactly, for example connected-dense 246086 → 246182 bytes and 39 leaves, and the connected-dense document 240 leaves. The key sets are unchanged in every file.

**Producers re-run** in a scratch copy of the candidate, built from `git archive 4912dc636`. Results are in `rv1_producer_compare.txt`.

| Kind | Producer | Outputs checked | Result |
|---|---|---|---|
| raws | release example `physics_source_connected` | connected sparse and dense; eigen_motion dense and sparse; fallback_uz sparse | identical to the committed bytes |
| B1 | release example `preview_physics_capture` | dense and sparse | identical to the committed bytes |
| documents | `result_export` `LOAD_REFERENCE{,_SOURCE}_WRITE_FIXTURES=1 cargo test --test load_reference_contract --test load_reference_source_contract` | connected dense and sparse, eigen_motion sparse | I first reset these to their base bytes. The writer regenerated the committed bytes. |
| analysis runs | pytest `tests/test_load_reference_{,source_}readers.py` with the write flags (390 passed, 2 skipped) | connected dense, eigen_motion sparse | reset to base first, then regenerated to the committed bytes |
| stress-neutral | T1's `cp3_stress_neutral_outputs.py` and `t1_joined_stress_neutral_outputs.py` | connected dense, eigen_motion sparse | reset to base first, then regenerated to the committed bytes |

After all of this, all 20 committed files are byte-identical in the scratch tree.

**Frozen references and historical raws.** None changed. The diff touches nothing under `validation/` except two lockfiles, nothing in `numerical_integrity` N01–N09, R01–R07 or NP-A–NP-D, no T0R or T1 reference, and nothing in `execution/` outside `IMPLEMENTATION/S11K/`.

**Hash pins.** The new constants equal the sha256 of the committed raws: connected-sparse `89bbc3f6…20c7` and connected-dense `187a6d8d…b575`. The old constants equal the base raws: `915965a4…b74c` and `3824035c…b2bf`. `CHANGE_RECORD.md` states old → new for both files.

**Run records.** I1's `SHA256SUMS` verifies: 39 entries.

## 8. Disclosure (`CHANGE_RECORD.md`)

It states:
- which quantities change bits, and why;
- the non-cancelling bound: at most the fold's own rounding error, about one rounding of the gross;
- that no case changes status unless it was absorbing a load;
- every size: bytes, changed leaves and the largest relative change per kind, satisfying N-4;
- the eigen_motion sparse `invocation_work` / `publication_charged` −12, as 12 units per byte of diagnostic text times a net −1 byte. I confirmed the factor `saturating_mul(12)` at `source_receipt.rs:909` and the 7617107 → 7617095 and 276924 → 276912 changes in the raw;
- that there is no in-band marker.

It follows `.agents/skills/chirality-change/SKILL.md`: one concise record with the checked revisions, results and limits. Minor gaps are in RV1-N4 and RV1-N9.

## 9. Hygiene

- The added lines contain no machine-specific absolute paths; they use placeholders (`<s11k-worktree>`, `<dec025-venv>`, `<T1 LSI>`).
- No `node_modules`, `target` or scratch output is committed.
- `git diff --check 6bb3ee490..4912dc636` is clean (exit 0).
- **rustfmt.** I used the host's rustfmt 1.8.0; the 1.97.1 toolchain has no rustfmt. Every changed `.rs` file has zero diff hunks, except three:
  - `FK/lib.rs`: one pre-existing module-order hunk, also present at the base;
  - `nonlinear_integration/src/lib.rs`: 16 hunks, all present at the base;
  - `k1_reference.rs`: a generated table, marked `#[rustfmt::skip]`.
- **Lockfiles.** 8 lockfiles each gain the single line `"open_pipe_stress_frame_kernel",` in `load_case_algebra`'s dependency list. There are no registry changes. These 8 are exactly the lockfiles outside `execution/` and `provenance/` that contain `load_case_algebra` (RV1-N7 covers the provenance snapshot).

## Addendum (C3) items

- **5a.x changes bearing on S11-K are all in the code:**
  - R5-3: the caller classification (§3) and the pins, with the pin-strength findings RV1-S1 and RV1-S2;
  - `linear_supports` on the exact path;
  - P1's S11-PROBE-A kernel test `straight_pipe` `p1_probe_a_recovery_is_exact_at_g1e7_and_g1e8`. Its section data (OD 0.2 m, wall 0.01 m, E 200 GPa, G = E/2.6) and its expected values (the exact dyadic −2w for root shear and moment; the tip deflection wL⁴/8EI) are derived in the test, not copied from P1.
  - Revision 4's R3-N2 (`round_scaled`), revision 5's option (c), the B1/B2 registrations and F13 (the loop pin plus the DEC-046 test passing unchanged) are present.
- **No missing 5a.x change** was found, so there is no BLOCKING finding under C3.
- **`GATE/S11_EXCEPTIONS.json`:** nothing in S11-K's code or records contradicts it. S11-K touches no gate or PP producer.
- **ROOT's regeneration approvals:** the substance is verified in §7 and §9. The record is missing (RV1-S3). I1 reports the local TS run (vitest: 134 files, 2822 tests) with the link removed, and no `node_modules` entry is committed.

## What I ran

All runs used `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>` and `--offline --locked`, one cargo job at a time (checked with `pgrep -x cargo`), in `<scratch>/cand`, a `git archive` of `4912dc636`.

**Suites** (`cargo test --offline --locked --no-fail-fast`; `rv1_suites_SUMMARY.txt`). All passed, with nothing skipped or filtered:

| Crate | Passed |
|---|---|
| frame_kernel | 116 |
| straight_pipe | 39 |
| curved_bend | 25 |
| load_case_algebra | 21 |
| primitive_loads | 49 |
| nonlinear_integration | 69 |
| sparse_direct | 25 |
| linear_supports | 15 |
| benchmarks/nonlinear | 19 |
| benchmarks/mechanics | 41 |
| nonlinear_supports | 22 |
| diagnostics | 24 |
| performance_harness | 25 |
| stress_recovery | 48 |
| user_loads | 28 |
| benchmarks/stress | 23 |
| benchmarks/numerical_integrity | 0 tests |
| product_physics | 448 (1 ignored in source) |
| result_export | 91 |
| runner/headless | 83 |

**Other runs.**
- The exactness probe and checker (§5): `rv1_exact_check.py.txt`, output in `rv1_exact_check.stdout.json`.
- The mutation sample (§6): `rv1_mutate.py.txt`, `rv1_mutation_results.json`, `rv1_mutation_run.log.txt`.
- The producers (§7): `rv1_producer_compare.txt`.
- The leaf diff: `rv1_leafdiff.py.txt`, output in `rv1_leafdiff.stdout.txt`.
- rustfmt, `git diff --check`, the sha256 checks, and the lockfile and caller greps.

Records are in `T3/REVIEW/_run_records/s11k_review/`, with their own `SHA256SUMS`. REVIEW's existing `SHA256SUMS` is untouched.

## What I did not check

- **Skipped for disk headroom:** `apps/desktop/src-tauri` tests, the desktop vitest and `npm run build`, and `npm run build:wasm`. Free disk was 7.6–9.7 GB, and src-tauri alone needs about 3.4 GB. I rely on I1's records (114 src-tauri tests; vitest 134 files and 2822 tests) and on hosted CI.
- **Python:** only the load-reference reader suites (390 passed, 2 skipped). I did not run I1's full set of 1464 tests.
- **Not re-run by me:** the whole-corpus fixture diff across every committed request (I verified the diff that is committed, and re-ran producers per kind), the mechanics-benchmark byte comparison of base against candidate (the suite passes), and the dense `fallback_uz` raw and the eigen_motion dense and connected sparse derived documents through a reset-to-base.
- **Not my review:** the DEC-025 sweep and hosted CI, including the surface-4 dual-viewport dispatch.
- **Not verified:** the exact −1 byte arithmetic of each residual rendering behind the −12 (I checked the factor and the totals), and the root cause of each `compile_fail` doctest beyond reading them (RV1-N8).
