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
