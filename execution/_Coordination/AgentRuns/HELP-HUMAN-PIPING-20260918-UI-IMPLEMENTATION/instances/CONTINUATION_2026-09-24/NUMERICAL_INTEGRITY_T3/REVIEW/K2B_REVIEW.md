# RV11: independent full-diff review of slice K2b

**Verdict: FAIL.** There is 1 BLOCKING finding, 3 SHOULD-FIX findings and 7 NOTEs.
- **The BLOCKING finding:** the step-5 reaction function publishes a wrong value, labelled exact, at the b the rule picks (RV11-1).
  - `SparseStiffness::force_scaled_reactions` forms K'·u at 2^b in unchecked binary64.
  - A reaction product that leaves the normal range at 2^b is flushed or truncated before unscaling.
  - It is then published with `Representability::Normal`.
  - The member-action recipe that RETURN §15 gives F1b has the same defect.
- **The rest of K2b holds:**
  - The solve side of the even-b derivation holds, and the kernel enforces it more strongly than the records say (RV11-3).
  - The kernel-only claim, the b = 0 identity, the b-rule, ruling B and the pins all check out.
  - Every re-run mutant is killed at I10's recorded sites.

## Reviewer, brief and delegation

- **Reviewer:** RV11, a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN). It ran as a background subagent of ROOT's session on the owner's Mac.
  - The brief is `TASK_BRIEFS/RV11_K2B_REVIEW.md`.
  - I did not design, implement or test K2b. I made no Git writes and did not delegate.
  - My writes are this file and `REVIEW/_run_records/k2b_review/**`, left uncommitted.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md` and `I8R_K1_RESUME.md` ("The Mac host").
  - `DESIGN.md` revision 5a.2: §4.7 in full, the K2b and F1 rows of §6, and §7.3.
  - `TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md`.
  - `ROOT_RULINGS_V1.md`, the four K2b sections of 2026-09-28.
  - The candidate's `IMPLEMENTATION/K2B/`: CHANGE_RECORD, RETURN and `_run_records/`.
  - The complete slice diff `git diff eb52114e9 ca20b9eca`, commit by commit, and the gate code it relies on (`FK/structural.rs`, `sparse.rs`, `exact_sum.rs`).
- **Placeholders:**
  - `<wt>` is the T3 worktrees root. My scratch is `<wt>/scratch/rv11` and my target `<wt>/rv11-target`.
  - `FK`, `SA`, `NI`, `SD` and `PP` are as in K2b's RETURN.
  - Line numbers are at the head `087b3a088`.

## Revisions reviewed (`_run_records/k2b_review/merge_check.txt`)

- **PR #1040** is open, on branch `codex/piping-k2b-20260928`, at head `087b3a088`. I verified this after a `git fetch`.
- **The slice is `eb52114e9..ca20b9eca`:** `6ce4d694b` (A), `70828d4d6` (rulings A–C), `8e6698282` (C), `b461dfc0f` (the pin) and `ca20b9eca` (the records).
  - Its 11 code files have exactly RETURN §2's line counts and sha256 prefixes at the head.
  - The pin row is 1,455 lines, `3b0c0afca9177f09`.
- **The merge `087b3a088` adds main's changes and nothing else. It conflicts with nothing.**
  - `git diff ca20b9eca 087b3a088` is byte-identical to `git diff eb52114e9 98b1723b1`.
  - `git diff 98b1723b1 087b3a088` is byte-identical to the slice diff.
  - No file is touched by both sides.
  - `git show --remerge-diff` is empty.
  - Main's code delta is test-only: `m03_skew_scope.rs` and SA's `k1_tests.rs`.
- **Everything I built** came from `git archive` copies under `<wt>/scratch/rv11`. The one read-only use of `<wt>/k2b` was GEN-8 (§8).

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV11-1 | **BLOCKING** | **Reactions:** `FK/structural/sparse.rs:393–430` (`force_scaled_reactions`). It sums `self.multiply(u)` at 2^b (`:419`; `multiply`, `:325`, forms plain binary64 products and sums) with the DOF's terms, then rounds once at 2^-b. **Actions:** the same defect in RETURN §15 step 4 (the member-action recipe for F1b) and the tests' `end_actions` helper. | **Probe F-A2** (`probes/probe_final.log`):<br>– A member W, N0→N1 along x, with L = 2^300 m, E = G = 2^200 Pa and A = I = J = 1. Only N0 RZ is free, loaded with a moment of 1.2345·2^-697 N·m.<br>– A separate, fully fixed member S (E = G = 2^1000 Pa, A = 2^30 m²) makes b = 0 fail with K2a's `EA/L: E*A`.<br>– The census span is [−697, 1030], so **the rule's b is −138**.<br>– `solve_with_force_scaling` gives **Passed** in both modes and both representations, with no records. θ = 5.950092146939021e-181, bit-identical to the same member without S solved today.<br>– `outcome.stiffness.force_scaled_reactions` publishes **R(N0 UY) = R(N1 UY) = 0, `Normal`**. The reactions are ±6EI/L²·θ = **±1.3825367244506685e-300 N**, and today's E12 gives exactly that on the member without S. At 2^b the product 6E′I/L²·θ ≈ 2^-1134.1 (2^-996.1 at b = 0) underflows inside `multiply`.<br>– With a moment 2^80 larger, the published value is 1.6713838193973253e-276, also `Normal`, **relative error 3.1e-7**.<br>– The §15 recipe publishes W's end shears as 0, `Normal`.<br>– **Forced-b grid (F-D):** on the member without S, 24 of 108 bit-identical solves in the census window publish wrong reactions and actions. On K-D5's 13 models (2,514 bit-identical solves) there are none.<br>– The census bounds K and f, not the reaction products K_rj·u_j of restrained rows. Step 5 says "never flushed", and RETURN §6 says "the exact value … rounded once". | Form each reaction as one exact sum of the products K′_rj·u_j (`ExactAccumulator::add_product` over the stored row) and the DOF's terms at 2^b, rounded once at 2^-b.<br>Or, keeping b = 0's `reactions` bits, check every product and partial sum at 2^b and refuse with `PublicationOutsideBinary64` when one leaves the normal range.<br>Give F1b a kernel function for member end actions, built the same way, and correct §15 steps 4–5 and RETURN §6.<br>Pin F-A2 in both modes and both representations. |
| RV11-2 | SHOULD-FIX | `FK/lib.rs:903–910`, `exact_normal_scaling`. It is used by `force_scaled_value` and by the ledger's `force_scaled_term` (`load_ledger.rs:373–402`). | It accepts a result that is `is_normal()` after rounding. An exact product in [2^-1022 − 2^-1075, 2^-1022) rounds up to 2^-1022, which is normal but inexact.<br>– `force_scaled_value(0x1.fffffffffffffp-1021, b = −2)` returns `Ok(2^-1022)`, not `NumericalRange` (`probes/float_boundary.txt`, F-B).<br>– **At the rule's b** (F-B: census [437, 1030], b = −706), the ledger term `Product(0x1.fffffffffffffp-317, 2^760)` has its x scaled inexactly, although scaling y (to 2^54) is exact. The scaled net is `0x2f90000000000000`; 2^b times the net is `0x2f8fffffffffffff`.<br>– The effect is 2^-53 relative: not a wrong value at 1e-9, but it falsifies "exact" in the function's doc, in L1 ("a term that cannot be scaled exactly and normally is ScaledEvaluation") and in §4.7.<br>– E, G, users, springs, curved entries and `Term` loads cannot reach it at the rule's b (they land at or above 2^-961). A `Product`'s single factor can, and `force_scaled_value` is public (F1b). | Accept only when the **exact** result is normal: `binary_exponent(value) + exponent` in [−1022, 1023]. Or check that the result times 2^-exponent is the value. Add the boundary case to `k2b_force_scaled_value_is_exact_…` and to the ledger test. |
| RV11-3 | SHOULD-FIX | RETURN §4 (premise P), §6 (reactions), §13.3; CHANGE_RECORD "Pending"; commit `ca20b9eca`'s message; ROOT's recorded sentence (`97000ab9f`) | **The premise is visible, but its scope is wrong.** It says the dense Cholesky factor, the triangular solves and the skyline LDLᵀ are unchecked. At the head all of them are range-checked with `checked_product`, `checked_value` and `checked_quotient`:<br>– `cholesky` (`structural.rs:1800–1841`); only the sqrt of a screened pivot is unchecked;<br>– `CholeskyFactor::solve` (`:1778–1799`);<br>– `ProfileFactor` factor and solve (`:1962–2025`), which the pattern path uses (`sparse.rs:1693`).<br>I10's own §13.3 table shows the check firing: "division overflow or underflow" at b = 416, dense.<br>**So on the solve side P is enforced:** a scaled evaluation either reproduces b = 0 bit for bit or fails with `Range`. My grids agree: 0 differences in 3,040 and in 2,514 comparisons (§2).<br>**What is really unchecked at 2^b** is the publication arithmetic (RV11-1), the RV11-2 boundary, and harmless sub-margin products (transformations, descriptive sums).<br>RETURN §6 also says reactions sum "the exact … K′·u". They sum `multiply`'s rounded output; the docstring's "the formed K*u" is accurate. | Rewrite the premise paragraph: list the checked stages and the unchecked ones, and tie the latter to RV11-1 and RV11-2. Fix §6's wording. ROOT may correct its recorded sentence. |
| RV11-4 | SHOULD-FIX | Test gap: the **census** of `Product` load terms (`FK/lib.rs` `load_term`) and of realized curved slots (SA `force_scale_census`) | Two of my mutants survive FK, SD and NI in full and PP's two tests, 361 of 361 passing (`mutations/MUTANTS.txt`):<br>– **RV11-CENSUS-PRODUCT-X:** a product is recorded at e(x), not e(x) + e(y);<br>– **RV11-CENSUS-NO-CURVED:** the curved matrices are left out of the census.<br>Neither is equivalent: each changes b whenever that input bounds the span. Their effect is availability only: the scaled formation of both inputs is checked, so neither can publish a wrong value. | Add census cases: a `Product` term with factors of very different exponents, and a curved slot's entries bounding the span. Show both mutants killed. |
| RV11-N1 | NOTE | `SA` evidence `Debug` | At b = 0 the derived `Debug` of `AssemblyEvidence` and `SparseAssemblyEvidence` now renders `force_scale: ForceScale { exponent: 0 }`, a design-placed field (§4.7).<br>– No product code formats either type. My lexer scan agrees, and PP's `Sources` derives no `Debug`.<br>– The b = 0 probe prints only `is_ok()` for evidences, as RETURN §3 discloses. | None. |
| RV11-N2 | NOTE | `solve_with_force_scaling`, `SA:1722–1746` | When the evaluation at the chosen b fails with a non-range error, the case returns that `Structural` error with **neither the step-1 trigger nor b**.<br>– In F-A and F-B, b = 0's refusal `EA/L: E*A` becomes "positive diagonal contribution absorbed by assembly".<br>– This is not a refusal "where no b fits", so ruling 7's letter holds. But F1b's diagnostics lose the fact that range scaling was attempted. | For F1b: carry the trigger and b on this path, or state why not. |
| RV11-N3 | NOTE | Ruling B, `FK/structural.rs:2433–2490` and `unscale_structural_solution` (`:2514`) | Implemented as ruled, and RETURN §6's field table matches the code field by field.<br>– The explicit outcome lives in `ForceScaledSolution::records`.<br>– `solution` carries the flushed zero or the infinity in the field itself.<br>– §15 step 2 says `records` "lists" them, but not that F1b must render each outcome with its field. Unrendered, the zero would be silent. | State it in F1b's interface. |
| RV11-N4 | NOTE | The force-scaled entries and `force_scaled_reactions` | They take the **unscaled** `AssembledForce`, and nothing in the type tells a scaled ledger from an unscaled one.<br>– A caller that passes a ledger already at 2^b gets it scaled twice. u is then off by 2^b.<br>– The gate still passes: its residual is self-consistent, and the load audit compares like with like. | For F1b: a marker type, or a documented invariant with a test. |
| RV11-N5 | NOTE | b = 0 physical residual records (`structural.rs:1366–1376`) | At b = 0 these fields are rounded **stepwise**, so they can round twice in the subnormal range. Their overflow is a `Range` error.<br>K2b's unscaling rounds once, and records an overflow as an outcome.<br>This is consistent with RETURN §4 step 13's condition ("every unscaled field normal"), and it is why an overflowing record is a step-1 trigger and a published outcome at b. | None (informational). |
| RV11-N6 | NOTE | Records disclosure | The raw logs' trailing blank lines (40 files, all `.log`, "new blank line at EOF"; nothing under `core/`) are disclosed in commit `ca20b9eca`'s message, not in RETURN §20. The disclosure is honest. | Optional: one line in RETURN §20. |
| RV11-N7 | NOTE | GEN-8 method | GEN-8's lint keeps only git-tracked files. On a `git archive` copy nested under the outer worktree (where the T3 worktrees directory is gitignored), it scans nothing and **passes vacuously**. My first run did this; I discarded it and re-ran in `<wt>/k2b`, which passes. | Run GEN-8 only in a git working tree of the candidate. |

## 1. The kernel-only claim

- **No product path calls a new entry.**
  - My lexer scan (`callers/lexscan.py.txt`) removes comments, strings and `#[cfg(test)]` items and skips test files. It covers all 36 new names.
  - At the head, every non-test hit is in `FK/lib.rs`, `FK/load_ledger.rs`, `FK/structural.rs`, `FK/structural/sparse.rs` and `SA`.
  - Within SA, the new entries are called only from new code: `evaluate_force_scaled`, the siblings and the orchestrator.
  - No module of PP, and no module of NI other than SA, names any of them. That includes the loop in `lib.rs`.
  - The base shows no hits.
  - PP never calls `assemble_sparse_stiffness`.
- **Existing entries are unchanged at b = 0.** Read line by line:
  - The only changes to existing code are:
    - `binary_exponent` becomes `pub(crate)`;
    - `SparseAssemblyOptions` gains a field, and `new()` and `Default` stay unscaled;
    - `assemble_sparse_stiffness` gains a branch taken only when b ≠ 0;
    - `unscaled_evidence(self.force_scale)?` becomes the first statement of the 7 existing SA solve entries: dense `solve`, `solve_assembled`, `solve_assembled_with_formation_check` and `solve_binary64`; pattern `solve_assembled`, `solve_assembled_with_formation_check` and `solve`;
    - the evidence field and its `Debug` (RV11-N1).
  - `unscaled_evidence` returns `Ok(())` for every evidence `new` builds, and only `new_force_scaled` can build a scaled one.
- **The b = 0 probe, re-run in full rather than sampled** (`b0probe/`).
  - I10's probe source, unchanged, rebuilt at opt-level 0 from archives of `eb52114e9` and `087b3a088`.
  - **439 of 439 outputs are identical**, base against head.
  - Both hash lists also equal I10's recorded release-build lists, 439 of 439.
  - The categories match RETURN §12: 345 Ok, 56 Err, 206 Passed, 64 Sensitive, 28 formation demotions and 20 loop runs.

## 2. The even-b derivation (RETURN §4), checked step by step

| Step | Check against the code at the head | Holds? |
|---|---|---|
| 1 Formation | `local_stiffness` puts 2^b on exactly one factor (E or G) in each K2a intermediate. K2a checks every intermediate, so each is exact or a refusal. The transformation and assembly order are unchanged. The FK test checks the stored values. | Yes, under P. Transformation products are unchecked, but one that leaves the normal range at 2^b is below 2^-1022, while every stiffness diagonal is at least about 2^-961 there. It is therefore below the diagonal's rounding: this is the 64-bit margin's purpose. |
| 2 Force | L1 scales each term, and each net is one exact sum rounded once. | Yes, except the RV11-2 boundary. |
| 3 Exponents | `:1258` computes s = −e(d).div_euclid(2). For even b, ⌊(e+b)/2⌋ = ⌊e/2⌋ + b/2. | Yes. |
| 4 A′ = A | `:1264` uses `radix_scale`: same-sign steps, each checked. The skew and allowances (`:1306–1316`) are invariant. | Yes. |
| 5 rhs′ = 2^(b/2)·rhs | `exact_scaled_rhs` (`:1203`) takes one exact sum and rounds it once, rejecting subnormal or zero results. The binary64 branch (`:1285`) is checked. | Yes. |
| 6 Audit | `audit_contributions` (`:804`) uses `exact_radix` of differences at s′+s′, and prescribed deltas against rhs, both ratios. The absorption screen compares rounded sums. | Yes. |
| 7 Factor, screens | A′ = A, so the pivots, rcond (`gate_rcond`), the amplification and the witness are identical. | Yes. |
| 8 y′ = 2^(b/2)·y | The solves are **checked** (RV11-3). So y′ is exactly scaled, or the solve fails with `Range`; nothing in between is silent. | Yes. |
| 9 u invariant | `:1676`: `radix_scale(y′, s − b/2)`. | Yes. |
| 10 Residual, refinement | The row exponent gains b. `normalized_product` and the exact KS3 numerator are invariant. The correction (`:1759`) scales like the rhs. | Yes. |
| 11 K-D5, load audit | ρ and the record are scale-free. The fidelity bits are republished from the unscaled ledger. | Yes. The forced-b tests and my grids include F122's demotion. |
| 12 Odd b | A′ = ΔAΔ with Δ = 2^(±1/2). The square roots in the dense factor then differ. | Yes; K2B-ODD-MIDPOINT is killed. |
| 13 Report | Equal whenever P holds and every unscaled field is normal. | Yes (see RV11-N5). |

**Trying to break the invariance.** Forced even b over each model's whole census window, in both modes and both representations:
- **F-C:** a coarse grid plus the edges on K-D5's 13 models. **3,040 comparisons show identical u bits and an identical unscaled `Debug`**, and 72 edge cases fail with an error. There is no case where b = 0 fails and the scaled solve passes.
- **F-D:** every tenth b on the 13 models. There are 193 or 194 bit-identical solves per model, 2,514 in all, and their reactions and actions also match.
- **Only reactions and actions break.** On the long member (RV11-1), u and the report stay invariant, but the reactions and actions are wrong at 24 of the 108 b values.

**Premise P.** P is stated where a reader sees it, and it is adequate for the StructuralSolution: the kernel enforces it there. Its scope, however, is misstated (RV11-3). It is not adequate for step 5's reactions and actions, which step 13's "published report" does not cover, and which RETURN §6 and §15 describe as exact (RV11-1).

## 3. The b-rule

- **The census** matches ruling 2:
  - E, G and the 24 predicted exponents of each frame. Any subnormal operand, including A, I, J or L, is refused.
  - Exact exponents for users, springs and the 144 entries of each curved slot.
  - e(x) for a `Term`, and e(x) + e(y) for a `Product`.
  - **The census gaps are RV11-4.**
- **The ±3 bound, re-derived.**
  - The numerator mantissa lies in [1, 6), or [1, 4) when k is 4 or 2.
  - The mantissa of Lⁿ, rounded twice, lies in [1, 8).
  - So the true exponent lies in [p − 3, p + 2]. Scaled values stay within [2^-961, 2^1018), which the 64- and 8-bit margins absorb.
- **The window, floor and parity** follow ruling 1 exactly: `div_euclid`, then m, m − 1 or m + 1, and a single odd point is refused. K2B-ODD-MIDPOINT is re-killed at `k2b_force_scaling.rs:243`.
- **I10's counterexample pin** is correct and labelled.
  - Its doc comment says "A DOCUMENTED LIMITATION OF THE b-RULE, NOT DESIRED BEHAVIOUR" and cites the ruling.
  - It asserts the span, the window, b = 312, the named refusal with its trigger, the forced-b = 312 `Range`, and a Passed solve within 1e-9 at b = 500, in both modes and both representations.
  - K2B-THIRD-ATTEMPT is re-killed at `k2b_tests.rs:1030`.
- **A case worse than refused exists: RV11-1.**
  - The solve side cannot publish a wrong Passed u: its stages are checked, and the gate's free rows are measured exactly.
  - The reactions and actions can.

## 4. The unscaling outcomes and the step-5 departure

- **Residual records (ruling B).**
  - Each physical field is re-formed from its normalized value at the unscaled exponent and rounded once.
  - The outcome is normal (unlisted), subnormal with 2^-1075/|v| rounded up, underflow (a signed zero) or overflow (a signed infinity). None refuses the case, and the sentinel row is untouched.
  - RETURN §6's field-by-field table matches the code:
    - scale exponents +b/2; the pivots, rcond, estimates and skew unchanged;
    - `contribution_rounding` unscaled descriptively;
    - the load-fidelity bits recomputed from the unscaled ledger, as b = 0 computes them (`unaudited_row`, `audit_load_row`);
    - the formation check unchanged;
    - error payloads: the stored-diagonal `NegativeEnergy` × 2^-b, the witness direction × 2^(b/2), and `Mechanism` geometric.
  - **The departure is sound and explicit** (see RV11-N3 for F1b).
- **Actions and reactions.**
  - `unscale_for_publication` and `publication_outcome` are correct for their input: normal exact, subnormal with its precision, and underflow or overflow refused.
  - **The input is the problem:** a value formed at 2^b by unchecked arithmetic (RV11-1).

## 5. The interactions under b

- **K-D5:** demotion parity holds. F122 stays Sensitive with `Estimate` at every forced b. `Debug` is equal across 13 models × both modes × both representations × the whole window. The curved slots are matched by bits and fail closed.
- **S11-K and L1:** a `Product` scales one factor, or splits b; the split's bounds are correct. Nets are rounded once, and nothing is pushed to a ledger. RV11-FIDELITY-AT-SCALE is killed (`k2b_tests.rs:1356`).
- **K1:** dense and pattern are `Debug`-equal at each b in the tests, and F-D's grids ran on the pattern path.
- **K2a:** its names survive where no b fits. The window and subnormal refusals carry `GJ/L: G*J`. The non-range path is RV11-N2.
- **The loop:** it reaches no scaled entry. The source pin (`k2b_force_scaled_entries_…`), the behavioural pin (`k2b_nonlinear_loop_reaches_no_force_scaled_entry`) and my lexer scan all show this, and K2B-PIN-LOOP is re-killed at the same 8 sites.

## 6. Mutations (`mutations/`; clean archives of the head, NONE first)

**NONE is clean:** 361 passed (FK 198, SD 30, NI 113 + 4 doc tests, PP 11 + 5).

| Mutant | Result | Kill sites (compared with I10's record) |
|---|---|---|
| K2B-ODD-MIDPOINT | killed | `k2b_force_scaling.rs:243`, `k2b_tests.rs:773`; same |
| K2B-UNSCALE-STEPWISE | killed | `k2b_force_scaling.rs:730`; same |
| K2B-RECORD-REFUSES | killed | `k2b_tests.rs:1231`, `:772`; same test (I10's `:1153` + the pin's 78 lines) |
| K2B-RECORD-SILENT | killed | `k2b_force_scaling.rs:875`, `k2b_tests.rs:809`; same |
| K2B-THIRD-ATTEMPT | killed | `k2b_tests.rs:1030`, `s11_site_table.rs:529`; same |
| K2B-PIN-LOOP (pin) | killed | 8 sites; same |
| KD5-M32a (original pin) | killed | 8 sites; identical to K1's and I10's tables |
| RV11-REACT-TERMS-UNSCALED (mine: reactions with unscaled terms) | killed | `k2b_force_scaling.rs:769`, `k2b_tests.rs:523` |
| RV11-FIDELITY-AT-SCALE (mine: fidelity bits left at 2^b) | killed | `k2b_tests.rs:1356` |
| RV11-CENSUS-PRODUCT-X (mine) | **survives** | RV11-4 |
| RV11-CENSUS-NO-CURVED (mine) | **survives** | RV11-4 |

## 7. The pins and site tables

- **The extensions are additive.**
  - They add +124 lines to `s11k_tests.rs` (the new siblings' bodies blanked by impl, the new definition counts, the new product and loop test), +5 rows to `s11_site_table.rs` and +17 lines to `s11f_site_test.rs`.
  - No existing row, count or assertion changes.
- **The original mutants' kill sites are unchanged.** `kill_site_comparison.txt` shows 15 of 15 with 0 lost and 0 added, and my re-run of KD5-M32a reproduces its 8 sites exactly.

## 8. Records and hygiene (`records_checks.txt`, `gen8.txt`)

- **SHA256SUMS:** 171 of 171 verify, with no unlisted or missing file.
- **GEN-8 passes on the head.** It ran read-only in `<wt>/k2b`, whose working tree is clean at `087b3a088` (RV11-N7).
- **No machine paths and no model identifiers** in `IMPLEMENTATION/K2B/` (`/usr/bin/grep`).
- **T9 is stated Mac-only.** Its records are consistent: 112 of 112 base and candidate hashes, equal to ROOT's calibration list.
- **The suites claim matches the logs:** FK 193, NI 115, 0 changed, 0 removed, 28 added, and 3 Mac platform failures whose blocks are byte-identical.
- **The disclosures are honest:**
  - the overwritten checkpoint-A logs (`checkpoint_a/README.txt`);
  - the probe's coverage (RETURN §3; RV11-N1);
  - the trailing blank lines (RV11-N6).
- **The F1b interface (RETURN §15):** every signature is exact at the head (checked mechanically). The recipe's semantics are wrong in steps 3–5 (RV11-1).

## What I ran

- **Host:** aarch64-apple-darwin, rustc and cargo 1.97.1, opt-level 0, `-j 8` for single jobs, `-j 4` for mutants (three at once), `RUST_TEST_THREADS=4`.
- **Head tests:** FK, SD and NI in full, and PP `s11f_site_test` and `formation_check_runtime`. All pass.
- **The b = 0 probe:** base and head.
- **My probe module:** F-A, F-A2, F-B, F-C and F-D, in a scratch copy of the head only.
- **Mutations:** 12 runs.
- **Scans:** the lexer scan on the base and the head; GEN-8.
- **The memory guard never fired:** `<wt>/guard/memguard.log` has only its start lines.
- **Mutant targets** were deleted after each run.

## What I did not do

- I did not re-run T9, the 39-manifest suites or the both-entry gate (not run for K2b). I cross-checked their records instead.
- Hosted CI (Linux) and the DEC-025 sweep are outside my run.
- I re-ran 7 of I10's 32 counted mutants (the brief's sample), not all of them.
- I made no timing or memory claims.
- I did not assess realistic reach beyond this: RV11-1 needs reaction products below about 2^-1022 at 2^b (in F-A2, a 2^300 m member and 1e-300 N reactions). Its reach in realistic models is nil, as with the pinned limitation, but it fails unsafe, not by refusal.

## Delta check at f385a8bc8

**Verdict: PASS.** No BLOCKING finding remains unresolved.
- **RV11-1 is resolved in the kernel.** Reactions and member actions at scale now fail closed, and none of my probes finds a wrong published value.
- **RV11-2, RV11-3 and RV11-4 are resolved.**
- **The check found 2 new SHOULD-FIX findings and 3 NOTEs:**
  - RETURN §15 step 5's spring-action recipe still has RV11-1's defect (RV11D-1);
  - two of the new member-action checks are untested (RV11D-2).

**Scope.** ROOT resumed me for this check at PR #1040's head `f385a8bc8`, which I verified after a fetch. It is three commits on my reviewed head `087b3a088`:
- `bf4647c21`, the fix;
- `14f9b093f`, the tests;
- `f385a8bc8`, RETURN addendum 1 and `_run_records/rv11_fixes/`.

The basis adds `ROOT_RULINGS_V1.md`, "K2b: rulings on RV11's review (ROOT)" (numerics `4ec82a9b3`), and ROOT's two decisions stated at resume. The evidence is in `_run_records/k2b_review/delta_f385a8bc8/`.

### Findings (delta)

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV11-1 | resolved (kernel) | `sparse.rs` `force_scaled_reactions` with `row_product_stays_normal`; `lib.rs` `FrameElement::force_scaled_end_actions` | **F-A2** (`probes/delta_probe.log`): at the rule's b = −138, in both modes and both representations, R(N0 UY) and R(N1 UY) are **refused** with `PublicationOutsideBinary64`, as are W's end actions. The moment reaction R(N1 RZ) is published `Normal` with today's bits. The 2^80 variant is also refused. Without S (b = 0), all are published with today's bits.<br>**F-D:** forced even b over the census window of 13 K-D5 models and 5 long-member variants gives **146,602 published values, 0 wrong**, and 204 refusals (all on the long members).<br>**F-E:** on K2b's own cases (reach_zero, reach_lef, partial underflow, PHYS-R4, three LEF-large, both modes), there are **0 refusals at the rule's b**, and every value published across each window equals the value at the rule's b (0 wrong). | None. |
| RV11-2 | resolved | `lib.rs` `exact_normal_scaling` | F-B: `force_scaled_value(0x1.fffffffffffffp-1021, b = −2)` is now `NumericalRange`. At the rule's b = −706 the ledger scales y (to 2^54), and the scaled net is exactly 2^b times the net. The acceptance test `binary_exponent(value) + exponent ∈ [−1022, 1023]` is exact for normal inputs. | None. |
| RV11-3 | resolved | RETURN §4, §6 and §15 (steps 3–4 and the signatures); CHANGE_RECORD "Pending"; ROOT_RULINGS_V1.md:1084 | Read in place (`records_checks.txt` item 8). The superseded sentence is kept and marked wrong. The code docs match. | None. (§15 step 5 is RV11D-1.) |
| RV11-4 | resolved | the census tests | RV11-CENSUS-PRODUCT-X is killed at `k2b_force_scaling.rs:1023` and RV11-CENSUS-NO-CURVED at `k2b_tests.rs:1631`, the sites I10 recorded. | None. |
| RV11D-1 | SHOULD-FIX | RETURN §15 step 5 (line 819), unchanged by addendum 1: "Spring actions: −(k·2^b)·u, through `unscale_for_publication`" | **F-S** (`probes/delta_probe_spring.log`): W with N0 UY free and a spring of 2^-600 N/m to ground there, plus S.<br>– At the rule's b = −138 the solve is Passed in both modes and both representations, and u_y is bit-identical to today's.<br>– The recipe's product at 2^b is 0, and `unscale_for_publication` publishes **0.0, `Normal`**. Today's −k·u is **1.3825367244506685e-300 N**.<br>– This is RV11-1's defect in the one step 5 output the fix left. My first review's resolution named "§15 steps 4–5".<br>– No K2b code publishes a spring action, so K2b itself publishes nothing wrong. But §15 is the interface F1b will follow. | Before F1b wires spring actions, correct step 5: the product of nonzero operands must be normal at 2^b, or the value is refused with `PublicationOutsideBinary64`. Better, give F1b a kernel helper built like `force_scaled_end_actions`, and pin F-S. |
| RV11D-2 | SHOULD-FIX | Test gap: `force_scaled_end_actions` (`lib.rs:986–1047`) | Two of my mutants survive FK, SD and NI in full and PP's two tests (367 of 367 pass):<br>– **RV11D-ACTIONS-LOCAL-UNCHECKED:** the check on the local stage T·u_e removed.<br>– **RV11D-ACTIONS-B0-EXEMPT:** every check skipped at b = 0, although the doc says "the check applies at every b, b = 0 included".<br>The tests exercise only the stiffness stage, at b = −138.<br>The local stage guards the same wrong-`Normal` class. If a T·u_e term is subnormal (a small direction-cosine component times a u near 2^-1022), the local displacement loses bits, and K′ at a large scale can lift the result back to a normal value that is published as exact. | Add an action case whose T·u_e has a subnormal product or partial sum, refused at b = 0 and at a b ≠ 0. Show both mutants killed. |
| RV11D-N1 | NOTE | ROOT's decision 2: the pin list (`s11k_tests.rs` `FORCE_SCALED_ENTRY_POINTS`) | "Covered indirectly through the pinned `ForceScale` token" is not complete.<br>– **RV11D-PIN-ACTIONS-EVASION** survives every test. It adds a non-test function to the loop's module (NI `lib.rs`) that calls `frame.force_scaled_end_actions(u, Default::default())`. That names no pinned token, because `token_indices` matches text and the scale's type is inferred.<br>– The same evasion reaches `unscale_for_publication` and the other functions that take a `ForceScale`.<br>– At b = 0 the new function returns the straight pipe's values or refuses, so the behavioural risk is small. | Add `"force_scaled_end_actions"` to `FORCE_SCALED_ENTRY_POINTS` (one line; it is also I10's A1.6 suggestion). Record the `Default::default()` limit beside RV8-N4's text-pin limits. |
| RV11D-N2 | NOTE | ROOT's decision 1: `force_scaled_reactions` at b = 0 | **Verified in the code and pinned.**<br>– `row_product_stays_normal` runs unconditionally, and **RV11D-REACT-B0-EXEMPT** is killed at `k2b_force_scaling.rs:1058`.<br>– A value it publishes at b = 0 has `reactions`' bits: the FK test, F-A2 without S, and F-D's b = 0 comparisons all show this.<br>**The check is stricter than flushing requires.** It refuses when any single product is subnormal, even one far below the rounding of a normal sum, which today's E12 would give accurately to within an ulp. That costs availability only. F-D and F-E show no such refusal on K-D5's or K2b's models. | F1b's gate measures it, as ROOT ruled. |
| RV11D-N3 | NOTE | `force_scaled_end_actions`' doc and RETURN §15 step 4: "a caller adds its load terms (fixed-end actions) at the same scale" | The function returns actions that are already unscaled, so a caller has no scaled value to add to.<br>– Adding b = 0 fixed-end actions to the published values is today's two-rounding order, and is fine when those loads are in range.<br>– A fixed-end action that must be formed under b (RETURN §14's F1b note) cannot be combined at scale through this interface. | For F1b: a variant that returns the scaled actions, or one that takes the load terms and rounds once. |

### ROOT's two decisions, against the code

1. **The reactions check applies at every b, b = 0 included.**
   - This holds: `force_scaled_reactions` calls `row_product_stays_normal` with no b guard, and the FK test asserts the refusals at b = 0 and b = 2 (RV11D-REACT-B0-EXEMPT is killed).
   - The row check mirrors `multiply`'s left fold exactly: the same stored-entry order, and appended signed zeros that cannot change a magnitude.
   - "Normal products and partial sums" does imply "2^b times the unbounded row". Binary64 rounding commutes with 2^b inside the normal range, and a partial sum rounds to zero only when its exact sum is zero.
   - Existing `reactions` and `multiply` are untouched (RV11D-N2).
2. **`force_scaled_end_actions` is not named in the pin list.**
   - It has no non-test caller outside FK `lib.rs` (my lexer scan, `callers/`).
   - The indirect coverage has a demonstrated gap (RV11D-N1).

### The other checks ROOT asked for

- **The write set** (`commits_check.txt`):
  - The fix commits change 5 code paths, all in K2b's declared write set: FK `lib.rs`, `structural/sparse.rs`, the declared `s11_site_table.rs` rows, and K2b's two test files.
  - Under `IMPLEMENTATION/K2B/`, CHANGE_RECORD, RETURN and SHA256SUMS are modified, and 147 files are added under `_run_records/rv11_fixes/`.
  - Nothing else changes. No pre-K2b function is touched, and no empty blob is added.
- **I10's index write** (`index_check.txt`, read-only): it left no trace.
  - In `<wt>/k2b`, at `f385a8bc8`, which equals its upstream, the index equals HEAD's tree entry for entry: 53,402 entries, same modes and blobs.
  - No entry carries a nonzero flag, so no intent-to-add entry remains. The working tree is clean.
  - The 147 `rv11_fixes` entries equal the committed blobs.
  - The reflog shows only the merge and ROOT's three commits.
- **b = 0 is unchanged:**
  - The **b = 0 probe gives 439 of 439 identical** at `f385a8bc8` against `eb52114e9`, against `98b1723b1` and against `087b3a088`. It equals I10's `rv11_fixes` release lists.
  - **F-C:** 3,040 forced-b comparisons on K-D5's 13 models, with identical u and unscaled `Debug`.
  - **Head tests pass:** FK 202, SD 30, NI 115 + 4 doc, and PP 11 + 5. There are no new warnings; PP's 10 are pre-existing.
- **Mutations** (`mutations/`, clean archives of `f385a8bc8`, NONE first, 367 passing):
  - I10's K2B-REACT-UNCHECKED, K2B-REACT-PARTIAL, K2B-ACTIONS-UNCHECKED and K2B-EXACT-ROUNDUP are killed at exactly the sites A1.7 records.
  - My RV11-CENSUS-PRODUCT-X and RV11-CENSUS-NO-CURVED are killed (RV11-4).
  - Of my four new mutants:
    - RV11D-REACT-B0-EXEMPT is killed;
    - RV11D-ACTIONS-LOCAL-UNCHECKED and RV11D-ACTIONS-B0-EXEMPT survive (RV11D-2);
    - RV11D-PIN-ACTIONS-EVASION survives (RV11D-N1).
- **Records** (`records_checks.txt`, `gen8.txt`):
  - SHA256SUMS: 318 of 318 verify.
  - There are no machine paths and no model identifiers.
  - A1.1's line counts and hashes match the head.
  - The A1.7 suites claim matches the logs: FK 202, NI 119, 0 changed, 0 removed, 35 added, and the same 3 Mac failures with byte-identical blocks.
  - T9's records show 112 of 112, equal to the calibration list. T9 is stated Mac-only.
  - GEN-8 passes, run read-only in `<wt>/k2b` at `f385a8bc8`.

### Not done in the delta check

- **Not re-run:** T9, the 39-manifest suites, or I10's other 51 re-run mutants. I cross-checked their records instead.
- **No timing or memory claims.**
- **Host:** the memory guard never fired, and the mutant targets were deleted after each run.
