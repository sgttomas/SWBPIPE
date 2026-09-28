# RV10: independent review of the skew M03 pin (PR #1038)

**Verdict: PASS.** Findings: **0 BLOCKING, 2 SHOULD-FIX, 6 NOTE.**
- **The slice is tests only.** The in-test pre-K2a matrix is 134eefc24's `local_stiffness` bit for bit, and the outcomes are RV7's table. The exact errors re-derive independently. The parity really runs through K1's pattern path, and the guard is labelled as a documented limitation.
- **I9's product evidence reproduces byte for byte** from fresh archives. No trusted value exists: 0 of 864 runs are `checks_passed`, and none is eligible.
- **Both SHOULD-FIXes are records-only.**
  - S1: two run records name superseded test-file hashes.
  - S2: RETURN §3.4's "consequence for later slices" generalizes "6EI/L² is the limiting coefficient" beyond RV7's rows, where it fails.

## 0. Revisions, basis and delegation

- **Candidate:** PR #1038, branch `codex/piping-m03-skew-pin-20260928`, head `1d105d633` (verified after `git fetch`; the PR's `headRefOid` agrees).
  - Commits: `885f065e5` (tests), `39dfd69c7` (records), and `1d105d633`, a merge of main `6e18505e3`.
  - The merge message says `5a2b9d112`; the second parent is `6e18505e3`, and the PR body discloses the slip.
- **Slice diff reviewed in full:** `git diff eb52114e91 39dfd69c74`, 63 files. These are the two test files and 61 record files, including SHA256SUMS.
- **Basis read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md`, and "The Mac host" in `TASK_BRIEFS/I8R_K1_RESUME.md`;
  - the brief `TASK_BRIEFS/I9_M03_SKEW_PIN.md`;
  - `REVIEW/K2A_REVIEW.md` B1 and §8.3;
  - `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md` §1 to §1.4;
  - `REVIEW/_run_records/k2a/rv7_rotated_m03.rs.txt`;
  - `ROOT_RULINGS_V1.md`, "The skew M03 pin" and the K2b rulings that follow it;
  - `IMPLEMENTATION/K1/RETURN.md` §12.
- **Source read:**
  - FK `local_stiffness`, `set_symmetric`, `add_bending_z`, `add_bending_y` and `add_terms` at `134eefc24` (`lib.rs:699-744`, `:1167-1208`);
  - FK `transform_roundoff`, `checked_value`, `checked_product` and `gamma` at `134eefc24` and `eb52114e9`, which are byte-identical, as are `transformation_matrix`, `transform_global_stiffness`, `orientation` and `properties`;
  - FK `sparse.rs` `prepare_sparse_bound`, `factor_sparse_structural_profile` and `finish_sparse_structural`;
  - SA `EvidenceParts::new` and `element`, and NI `k1_tests.rs` helpers at `eb52114e9`;
  - PP `derive_pipe_section`;
  - result_export `numerical_use_standing_with_context`, and `source_blocks::validate` and `ordinary` at `134eefc24`.
- **Delegation:** RV10 is a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN), running as a background subagent of ROOT's session on the owner's Mac. It did not delegate.
- **What I wrote:** this file and `REVIEW/_run_records/m03_skew_pin_review/` (with its own `SHA256SUMS`), both uncommitted.
  - No Git write of any kind.
  - Nothing written in `<wt>/skewpin`; it was read only, and GEN-8 ran on an archive.
- **Host:**
  - Every build is from a `git archive` in `<wt>/scratch/rv10`, with targets under `<wt>/rv10-target`, pruned at the end.
  - Limits: cargo `-j 8` at most; `RUST_TEST_THREADS=4`; at most two cargo jobs of mine at once, except mutants, which ran three at once at `-j 4`. Every model has one member.
  - The memory guard log shows no kill.
  - Records: `_run_records/m03_skew_pin_review/README.txt` and `toolchain.txt`.

## 1. Tests only (check 1)

Record: `checks/merge_and_scope.txt`.

- **Outside the records folder, the slice changes two files:**
  - `frame_kernel/tests/m03_skew_scope.rs`, new, 947 lines;
  - `nonlinear_integration/src/structural_adapter/k1_tests.rs`, +150/−0.
  - There is no product source, fixture, reference, `Cargo.toml` or lockfile change.
- **`k1_tests.rs` is append-only.** The candidate's first 1409 lines hash to the base file's sha256 `ccd97e94…`. The file is `#[cfg(test)]` (`structural_adapter.rs:1985`).
- **Both test files are stable-rustfmt clean.** The committed bytes are `824fc637…` and `be78e978…`, identical at `885f065e5` and at the head.
- **The merge adds nothing of its own.** `git diff 6e18505e3 1d105d633` is byte-identical to `git diff eb52114e9 39dfd69c7` (sha256 `33875d4e…` for both).
- **Main's changes `eb52114e9..6e18505e3` (588 files):**
  - 392 are under `projects/chirality-app-v4/` and 1 is a `docs/governance_harness` tranche manifest;
  - 194 are T3 records and 1 is the piping work graph.
  - None is under root `tools/` or `.github/`, and none is under `projects/chirality-piping/` outside `execution/`, which also excludes that project's own `tools/` and `.github/`.
- **The archive trees agree:** `projects/chirality-piping` without `execution/`, at the head against `eb52114e9`, differs in exactly the two test files.

## 2. The pin is correct and not vacuous (check 2)

### 2.1 The replica is 134eefc24's formation

- **Code comparison.** `pre_k2a_coefficients` and `pre_k2a_local` (`m03_skew_scope.rs:219-290`) and the adapter's `i9_pre_k2a_local` (`k1_tests.rs:1429-1487`) match `134eefc24`'s `local_stiffness` operation for operation.
  - The coefficients: L² = L·L and L³ = L²·L; then (E·A)/L, (G·J)/L and ((k·E)·I)/Lⁿ.
  - The layout: the axial and torsion `set_symmetric` off-diagonals first, then the diagonals.
  - Then `add_bending_z` (indices UY, RZ, UY+6, RZ+6) before `add_bending_y` (UZ, RY, UZ+6, RY+6), with the same sign pattern and `+=` from zero.
  - The validations change no value.
- **Behavioural confirmation.** Both replicas were included verbatim in a probe run against a `git archive` of `134eefc24`'s own FK (`pin/rv10_probe.rs.txt`, part A).
  - They equal `134eefc24`'s `local_stiffness` bit for bit on all 18 case–member pairs and on a normal section, 3024 entries in all.
  - `transform_global_stiffness` of the replica equals `134eefc24`'s `global_stiffness`.
- **K2a's precondition.** The candidate's precondition test also shows that the replica equals K2a's formation on normal inputs, and that K2a refuses all 18 formations as `12EIy/L^3: (12*E)*Iy`.

### 2.2 Orientations and outcomes

- **Orientations.** Every orientation comes from FK's own `FrameElement::orientation().transformation_matrix()` and `transform_global_stiffness`. Every member's length is exactly 2^-39 m (asserted).
- **Outcomes on pre-K2a main.** On `134eefc24`'s own matrix and M03 (probe part B), the outcomes are RV7's table exactly:
  - the axis member is refused in all six rows;
  - both skew members are accepted from 2^-1030 to 2^-1050, and refused at 2^-1055 and in S6a;
  - every refusal is `Range("arithmetic outside normal range")`.
- **Outcomes in the candidate's tests.** Re-run from the archive, they give the same outcomes and print every figure of I9's log (`pin/fk_m03_cand.log`, `pin/ni_i9_cand.log`).
  - Only the interleaving of three stderr lines differs from I9's log.

### 2.3 Exact-reference errors, re-derived with Fraction

`pin/rv10_exact.py.txt` takes E, I, L and the formed coefficients as bits from 134eefc24's own matrix, and computes k·E·I/Lⁿ in Fractions.

| (12E)I | 2EI/L | 4EI/L | Candidate's `exact_errors` (5 digits) | RV7 |
|---|---|---|---|---|
| 2^-1030 | 1.137801e-13 | 5.675013e-14 | agree | 1.1e-13, 5.7e-14 |
| 2^-1040 | 1.164154e-10 | 5.820757e-11 | agree | 1.16e-10, 5.8e-11 |
| 2^-1045 | 3.725290e-9 | 1.862645e-9 | agree | 3.73e-9, 1.86e-9 |
| 2^-1050 | 1.192093e-7 | 5.960464e-8 | agree | 1.19e-7, 5.96e-8 |

- **Other RETURN §3.3 figures also hold:**
  - 6EI/L² and 12EI/L³ carry a relative error of 9.33e-17;
  - fl(12E)·I is exactly 2^t·(1 − 1.851e-16);
  - in S6a it is 2.5·2^-1075·(1 − 3.818e-17), rounding to 2^-1074, with (4E)·I = (2E)·I = 0.
- **The in-test integer routine** (`exact_relative_error`, `:318`) is exact up to its final quotient. Its `u128` products stay below 2^110.

### 2.4 "6EI/L² is the limiting coefficient" in RV7's rows

- **The counterfactual test holds.** Moving only 6EI/L² across the floor by a factor of 8 flips every skew row, except that 2^-1055 on (1,2,2) then refuses with a product underflow.
- **An independent sweep agrees** (probe part D). It runs 513 points of (12E)·I from 2^-1049 to 2^-1057 in steps of 1/64 binade, on 134eefc24's own formation.
  - On both skew members, acceptance of the full matrix equals acceptance of the matrix with every entry except the 6EI/L² couplings zeroed, at 513 of 513 points.
  - So in RV7's configuration the coupling block, that is 6EI/L², decides throughout.
- **The threshold is orientation-dependent,** not the axis floor 2^-974.585:
  - (1,1,1) is last accepted at 6EI/L² = 2^-973.781;
  - (1,2,2) is last accepted at 2^-973.000, which is RV7's 2^-1050 row itself (N5).
- **Outside RV7's configuration it fails** (S2).

### 2.5 The parity really exercises K1's pattern path

- **FK level.** `pattern_m03` (`:784`) runs K1's `assemble_sparse_stiffness` with a `StiffnessBlock`, one allowance per pattern entry, `SparseStructuralSystem::new`, `prepare_sparse_structural`, `factor_sparse_structural_profile` and `finish_sparse_structural`.
  - The allowance arithmetic equals SA's `EvidenceParts`: bound + gamma(1)·|value|, with count 48 + 1.
- **NI level.** The appended test runs `SparseAssemblyEvidence::solve_assembled` against `AssemblyEvidence` on the dense view, in both modes.
- **Refusal parity is by construction,** as both tests' comments say. `transform_roundoff` runs before either store, exactly as in SA's shared `EvidenceParts::new`, so refused rows never reach a representation. This faithfully mirrors the product.
- **Behavioural proof.** My pattern-only mutant `RV10-K1-PATTERN-SCALE` makes `prepare_sparse_bound` scale values by the row exponent twice, and leaves dense untouched. Both parity assertions kill it:
  - FK `:855`, where dense gives `NumericallyUnresolved { … Some(10) }` and the pattern gives `Asymmetric { row: 9, col: 7, … }`;
  - NI `k1_tests.rs:1549`.
- **No stop was needed.** The pattern path carried the pre-K2a matrix without a product change (the brief's stop condition did not arise).

### 2.6 The guard is labelled correctly

- **Name and doc comment:** `m03_skew_scope_guard_documented_limitation_not_desired_behaviour` (`:913`). The doc comment (`:900-911`) says the accepted rows are "today's **documented limitation**, not … desired behaviour" and cites K2a `RETURN_ADDENDUM_1.md` §1.4.
- **What it asserts:**
  - the exact list of 8 accepted pairs;
  - 18 outcomes in all;
  - every refusal is `Range("arithmetic outside normal range")`.
- **Its reach** is narrower than its comment's last clause (N2).

## 3. Mutations (check 3)

Records: `mutations/`.
- Each mutant ran on a fresh `git archive` of `1d105d633` with its own target; both were deleted afterwards.
- NONE ran first, alone; the rest ran three at a time at `-j 4`.
- I9's four were applied from I9's committed `mutate_i9.py.txt`, unchanged.

| Mutant | Change | FK kill sites (`m03_skew_scope.rs`) | NI | Result |
|---|---|---|---|---|
| NONE | – | 5/5 pass | pass | control |
| M-AXIS-BOUND (I9, required) | axis-aligned per-entry bound | :562, :869, :637, **:931** | :1525 | killed, as I9 records |
| M-FLOOR-UP (I9, required) | floor ×16 | :444, :637, :562, **:931**, :869 | :1525 | killed, as I9 records |
| M-NO-COUPLING (I9, required) | coupling block unbounded | :637, :551, **:921**, :869 | survives | killed, as I9 records |
| M-CLOSE-GAP (I9) | refuse every sub-floor coefficient | :562, :869, :637, **:931** | :1525 | killed, as I9 records |
| RV10-MAX-NOT-SUM | the bound's sums become maxima (unchanged on axis members) | :380 (figure), **:931**, :869 | :1525 | killed at behavioural assertions |
| RV10-TRANSPOSE | \|T_ik\| for \|T_ki\| in the bound (identical on axis members) | :380 (figure), **:931** ((1,2,2) now refused), :869 | :1525 | killed at behavioural assertions |
| RV10-NO-TORSION-MIX | the torsion rows dropped from the bound (no GJ/L lift) | :562, :869, :637, **:931** | :1525 | killed |
| RV10-DEEP-REFUSE-1016 | refuse any nonzero local coefficient below 2^-1016 | :637 only | pass | killed, by the counterfactual alone |
| RV10-DEEP-REFUSE-1019 | refuse any nonzero local coefficient below 2^-1019 | none | pass | **survives** (N2) |
| RV10-K1-PATTERN-SCALE | K1 pattern path only (§2.5) | :855 | :1549 | killed by the parity assertions |

- **The four re-kills match I9's kill-site table** (RETURN §7) site for site.
- **The two figure-first kills** (`:380`, `close_log2`) are also killed by the guard's outcome assertion `:931`.
- **The RV10-DEEP-REFUSE pair shows** that the counterfactual test adds real coverage beyond RV7's rows, and where the pin's reach ends.

## 4. I9's product evidence (check 4)

Records: `product/`.

- **The re-run reproduces byte for byte.** I9's probe was re-run unchanged on fresh archives of `134eefc24` and `eb52114e9`. For both trees, the I9PROBE, I9GEOM and I9SECTION lines are identical to I9's committed raw logs (sha256 `aa90378a…` and `fe9c555f…`), 432 runs each.
- **Statuses:**
  - pre-K2a main: 380 `unresolved` and 52 `sensitive`, with case quality `passive_model_basis` / `sensitive` / `not_claimed`;
  - current main: 432 `not_assessed`;
  - no run anywhere carries `checks_passed`.
- **The sensitive publications, independently solved.** RV10's own Fraction solve of the free node (T^T K T at the intended coefficients, with the probe's represented inputs and FK's axes) covers all 52 published runs.
  - It gives a minimum error of 7.28e-12, a maximum of 1.87e-4, and 42 runs over 1e-9.
  - These agree with I9's `product_runs.jsonl` (the same minimum, maximum and count). Every cell's worst error agrees with I9's table: 5.96e-8, 1.19e-7, 1.16e-10, 7.63e-6, 6.10e-5, 1.22e-4, 1.87e-4 and 1.91e-6.
  - **Pre-K2a main does publish skew cases as Sensitive at intermediate G/E (2^30 to 2^40)**, wrong by up to 1.9e-4. None of them is trusted.
- **No trusted wrong value exists in the committed records.** `product_runs.jsonl` holds 864 records; none contains `checks_passed` or `numerically_eligible`.
  - The 26 captured Sensitive runs carry `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`.
  - The 26 typed runs have no source blocks.
  - So neither branch of `numerical_use_standing_with_context` can return `numerically_eligible` (see N3 for the branch the RETURN's wording omits).
- **The stop rule was correctly not triggered.**
- **"6EI/L² is the limiting coefficient"** holds in RV7's rows (§2.4). The RETURN's general consequence does not hold beyond them (S2).
- **Not re-derived:** the formation-only error figures of RETURN §6.3 item 3 (2.3e-13, 2.3e-10, 7.45e-9).

## 5. Records hygiene (check 5)

Records: `checks/records_hygiene.txt` and `checks/gen8.txt`.

- **SHA256SUMS:** 60 of 60 OK. The listed and present file sets are identical.
- **GEN-8 on the head passes** (a full `git archive` of `1d105d633`; 1 passed, 10 deselected).
- **Machine paths** (`/usr/bin/grep -E` over all 63 files): no absolute machine path, and no owner or host name.
  - The one textual hit is the part-built scratchpad regex in `assemble_run_records.py.txt:17`, which names no path.
  - Two relative fragments remain (N4).
- **No model identifier** appears.
- **T9 is stated as Mac-only** in RETURN §8, CHANGE_RECORD and `t9_summary.txt`.
- **The suites claim matches the logs,** and it reproduces. From archives, `eb52114e9` gives FK 179, SD 30 and NI 101, all passing. The head gives FK 184, SD 30 and NI 102, all passing. The per-test difference is exactly the 6 added tests (`suites/summary.txt`).
- **Hosted CI** on `1d105d633` is all success at the time of review (observed, not re-run).
- **Stale hashes:** two records name superseded test-file hashes (S1).

## 6. Findings

### S1 (SHOULD-FIX, records only): two run records name superseded test-file bytes

- **Sites:**
  - `_run_records/trees.txt:5-7` gives `m03_skew_scope.rs` sha256 `39276069…`, `k1_tests.rs` `4f34c248…` and "+149 lines appended";
  - `_run_records/t9/t9_summary.txt:2-3` gives the same two hashes.
- **The committed bytes** are `824fc637…` and `be78e978…`, with +150 lines, as RETURN §2 correctly states.
- **The runs themselves used the final bytes:**
  - In I9's scratch (host local time), the candidate files are dated 00:30:12, and T9's candidate ran at 00:30:13–15, the suites at 00:30:22–35 and the mutants at 00:30:54–00:31:16. `trees.txt` (00:27) and `t9_summary.txt` (00:27:32) predate them.
  - Every kill-site line number matches the final file.
  - My re-runs from the head reproduce the tests, suites and product evidence.
- **Fix:** correct the hashes and the line count in both files, and regenerate `SHA256SUMS`.

### S2 (SHOULD-FIX, records only): RETURN §3.4's "consequence for later slices" generalizes beyond RV7's rows

**The site** is `RETURN.md:116`: "M03's skew acceptance of subnormal-derived 1/L coefficients ends either at the 6EI/L² floor or where 2EI/L·|T|² falls below 2^-1022, whichever comes first." It is offered for K2b's scaling and F1b. All of the following ran on `134eefc24`'s own formation and M03 (`pin/`).

(a) **The 6EI/L² threshold on a skew member is not the floor.** The least coupling bound is about 2g·s·6EI/L², so refusal starts at 6EI/L² ≈ 2^-973.8 on (1,1,1) and ≈ 2^-973.0 on (1,2,2). That is up to 1.6 binades above 2^-974.585 (§2.4). Addendum §1.3's "6EI/L² against the floor" has the same imprecision; that is a K2a record, for ROOT.

(b) **With Iy ≠ Iz, 6EI/L² does not limit.** Iy ≠ Iz is FK's API; PP always forms Iy = Iz (`derive_pipe_section`).
- The case: a member along (1,2,3) with y-reference (0.3, 0.5, 1), so the rotation is dense, with (12E)·Iy = 2^-1030 and Iz = Iy·2^-23 (`rv10_probe_f`).
- M03 **accepts** 6EIz/L² = 2^-976, below the floor, with subnormal-derived 4EIz/L and 2EIz/L. It refuses only at Iz·2^-25, with a product underflow.
- On RV7's two skew members the same section is refused: each has a zero in its local z axis, which blocks the lift.

(c) **"1/L coefficients" includes GJ/L, which 6EI/L² does not govern.** The case: normal bending (E = 2e11 Pa) with G·J rounding to about 2^-1050, so GJ/L = 2^-1011, subnormal-derived and below the floor (`rv10_probe_g`).
- M03 refuses it on the axis member and **accepts it on both skew members,** lifted by 4EI/L.
- At GJ/L = 2^-1020 it still accepts on (1,1,1). On (1,2,2) it refuses by GJ/L's own product underflow, not by the 2EI/L term the sentence names.

**Why SHOULD-FIX and not BLOCKING:**
- The pin's tests and their scope are correct.
- No product-reach or benefit statement rests on the sentence.
- I9 routed §3.4 to ROOT rather than into any accepted record.
- But ROOT's standing lesson applies: a general claim must be derived and independently checked, and later slices are told to lean on this one.

**Fix:** scope the sentence to RV7's configuration: a pipe section (Iy = Iz), the bending coefficients, and these members at L = 2^-39. State (a) to (c) as outside it, or restate it as "acceptance is decided by the least mixed bound in each block; no single-coefficient threshold holds in general". The test module's doc (`:9-18`) needs no change for (b) and (c), since it speaks only of RV7's cases.

### NOTEs

- **N1: the "second limit" wording.** The module doc `:16-18` and the RETURN summary say "Below those cases a second limit appears".
  - In natural formation at RV7's geometry, the coupling bound decides acceptance throughout, down to 2^-1057 (§2.4).
  - The second-stage product limit decides only in the counterfactual, where 6EI/L² is raised.
  - The first-stage product underflow (2EI/L·|T| < 2^-1022) only changes (1,2,2)'s refusal text, to `product overflow or underflow`, below about 2^-1056.34.
- **N2: the guard's reach.** The guard's last clause, "so that any change to M03's scope on skew members is deliberate and visible" (`:902-903`), echoes the brief but is broader than the 18 pinned pairs.
  - `RV10-DEEP-REFUSE-1019` passes all six tests.
  - Yet it changes M03's outcome on a pipe-section input: GJ/L = 2^-1020 on (1,1,1) is accepted by pre-K2a M03 (S2(c)), and the mutant refuses it by construction.
  - The guard meets the brief's operative requirement. If K2b, K5 or F1b lean on M03's skew scope beyond RV7's rows, carry forward adding torsion, near-threshold and (if kernel callers matter) Iy ≠ Iz rows.
- **N3: the eligibility wording in RETURN §6.3 item 1.** It says "result_export's `numerically_eligible` requires `checks_passed`". That holds for the ordinary branch.
  - In the source-blocks/physics-source branch, a case can qualify through an EXACT-method source recovery without ordinary `checks_passed` (`semantic_contract.rs:480-503`, `source_blocks.rs:935-957` at `134eefc24`).
  - Here recovery was unavailable (captured) or absent (typed), so the conclusion stands.
- **N4: relative path fragments in two logs.** `_run_records/fk_m03_skew_scope.log:2` and `ni_i9_adapter.log:2` keep a relative worktree-layout prefix ending in `t3/skewpin-target/suites_cand/`.
  - Cargo printed it relative to the working directory, so the `<wt>` substitution missed it.
  - It is not absolute, and GEN-8 is unaffected. Tidy optionally, to `<wt>/skewpin-target/…`.
- **N5: RV7's 2^-1050 row on (1,2,2) sits on a knife edge.** Its least bound is 2^-1022·(1 + 1.8e-14), 81 ulps above the normal threshold (`pin/rv10_probe_h`); (1,1,1) has a margin of 1.73×.
  - The reason: 2·gamma(24)/3 is about 2^-49, and 6EI/L² is exactly 2^-973.
  - The computation is deterministic: IEEE operations, correctly rounded `sqrt`, and hosted Linux CI passes.
  - A change to the bound's reserve or summation order at the 1e-14 level can flip that one pair and fire the guard without any change of scope in B1's sense. Recorded for whoever next touches `transform_roundoff`.
- **N6: a torsion analogue of B1, for ROOT.** On skew members, pre-K2a M03 accepts a subnormal-derived GJ/L lifted by normal bending (S2(c)).
  - Addendum §1.3 lists bending and the unprobed EA/L with 12EI/L³, not torsion.
  - K2a's formation refusal at `GJ/L: G*J` covers it on current main, which is consistent with §1.4's "every zero or subnormal-derived coefficient … on every orientation".
  - The case is not probed on the product route.

## 7. What I did not do

- **T9 was not re-run.** The piping archives of base and head differ only in two test-only files: an integration test, and a `#[cfg(test)]` module outside the harness's release build. I9's base and candidate outputs are identical, 112 of 112.
- **Not run:** PP and the other 36 manifests; DEC-025; a re-run of hosted CI.
- **The "4×" relation and formation-only errors** of RETURN §6.3 item 3 were not re-derived.
- **Not probed:** the translational EA/L with 12EI/L³ case (addendum §1.3).
- **No edits:** I edited no candidate file, record, ruling or work graph.

## 8. Run records

`REVIEW/_run_records/m03_skew_pin_review/` holds `README.txt`, `toolchain.txt`, `checks/`, `pin/`, `mutations/`, `product/`, `suites/`, `build_records.py.txt` and `SHA256SUMS` (every file there except itself). Machine paths are replaced by `<wt>`, `<VENV>`, `{REPO_ROOT}`, `<home>` and `<scratch>`.
