# RV4: independent complete-diff review of slice S11-G

**Verdict: PASS**, for head `759dccf3530530be66feffc9aa1f65950e905746`. There are no BLOCKING findings. There are 4 SHOULD-FIX findings and 10 NOTEs.

**The main finding: the R-b′ path-1 residual is reachable.** It was previously disclosed as "not demonstrated reachable". It fails closed on every reachable path, so under ruling 3 and Addendum 3 it is not BLOCKING. Its disclosure and T20 need correcting (RV4-S1).

**Reviewer.** RV4 is a Type 2 TASK in the TASK role. The brief is `T3/TASK_BRIEFS/RV4_S11G_REVIEW.md` with Addenda 1–3, read with `_COMMON.md`.
- **Independence.** I did not design S11-G (D1), check it (V1) or implement it (I5). This is not owner review.
- **What I changed.** I fixed nothing. My only Git operations were to add and then remove my own detached scratch worktree, before I switched to archive extractions to save disk.

## 0. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | PR #1003, branch `codex/piping-s11g-20260927`, head `759dccf35` (`git ls-remote` confirmed it pushed) |
| Chain | `3d844fea4` (merge of main `72d5ff864`; tree identical to `72d5ff864`) → `b62e40d4d` (I5's S11-G) → `759dccf35` (the manager's GATE emptying) |
| Base (merge base) | `72d5ff86443345b6594942bb08139c5cec999015` |
| Diff | `git diff 72d5ff864..759dccf35`: 42 files, +7857 −142; product and test code 10 files, +4815 −37 |
| Trees built | `git archive` of `projects/chirality-piping` (excluding `execution/`) at `759dccf35` (tree `684bed0d…`) and at `72d5ff864` (tree `8f1a2e77…`) |
| Design basis | `DESIGN_NUMERICS/S11G_GUARD.md` revision 2.2 with the §0.2 erratum E-1 to E-4, sha256 `afe5f256…` (the erratum-bearing file; 2.2 as selected was `680fecdd…`, `3c80158e9`) |
| Check basis | `REVIEW/S11G_CHECK.md` sha256 `ec0efce7…` (V1's delta-2.2 PASS; D22-1 and N1–N4) |
| Rulings | `ROOT_RULINGS_V1.md`: every S11-G section, through "Selection: S11-G revision 2.2" and the note on T18–T20 and T6a |
| Records basis | `<wt>/numerics` at `37bdc2edd`. Its GATE files are identical to the candidate's |

## 1. Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| **RV4-S1** | **SHOULD-FIX** | Disclosure of the R-b′ path-1 residual: `IMPLEMENTATION/S11G/CHANGE_RECORD.md:105-108`, `RETURN.md:167-175` and `:252`, the PR #1003 body ("Disclosures"), and the T20 comment and test at `core/product_physics/src/s11g_tests.rs:2182-2240` | **The residual is reachable, through an ordinary construction (C1, §6).** It uses N05's cantilever with T19's per-case bases. Case A is N05's cancelling tip torques on the base basis: Sensitive, and selected. Case B is on the invented soft basis (E = 1 Pa, G = 0.4 Pa) and carries only nodal inputs: tip F_y = 1 N and tip M_z = 1e-7 N·m. **The two-case captured invocation returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` in both modes.** It does the same at m = 1e-9, and at F = 10 N with m = 1e-6. The preconditions were checked:<br>- Case B alone is Passed and published, and R-b′ demotes it (q = 9.99999993922529e-8, B = 1.42e-14, S\* = 2.0). That is a genuine error of 6.1e-9 relative, so this is a true catch.<br>- The control (m = 0.5, R-b′ silent) publishes with a receipt: case A `qualified`, case B `qualified` with ordinary outcome `checks_passed`.<br>**It fails closed** (§6): no envelope is published on any reachable path. So it is not BLOCKING (Addendum 3). But "not demonstrated reachable" is false. And T20 pins a non-refusing construction, while ruling 3(b) requires a test that pins the refusal | 1. Rewrite T20 on C1 (or an equivalent refusing construction), labelled a characterization of a known residual. It should assert: the pre-0.4 captured `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` with no envelope; case B alone demoted and not refused; the control publishing with a receipt; and a comment that 0.4.0 republishes.<br>2. Correct CHANGE_RECORD, RETURN and the PR body to "reachable through per-case modulus bases with ordinary nodal loads; fail-closed". Also update the work graph's reach wording.<br>3. No product change is needed under ruling 3. The owner stays T3's `SOURCE_BLOCKS_FINALIZATION_FAILED` item |
| **RV4-S2** | **SHOULD-FIX** | D21-1, the second test's ±12B: `formation_guard.rs:182-196` (`exceeds`), used at `:241` | Mutant **RV-M1** removes B from the second test, leaving \|A_net\| > 12·T0 instead of \|A_net\| + 12B > 12·T0. **It survives the whole product_physics suite (506 tests),** the site test and the FK and SP S11-G unit tests. No test has B > 0 with \|A_net\| in (12(T0 − B), 12·T0]. `ruling2_boundary_of_the_first_clause` (d) has A = 0 and B = T0, which the first clause decides. This is the clause V1 raised (D21-1) and ROOT carried into the brief. Without a test, a fail-open regression in it is invisible | Add a unit test at a row with one `Bounded` term (B > 0, B < T0) and an `Exact` term whose exact defect lies strictly between 12(T0 − B) and 12·T0 (after scaling by 12). It must fire, and must be silent with B = 0. It kills RV-M1 (`mutations/patches/RV-M1.patch`) |
| **RV4-S3** | **SHOULD-FIX** | The exact-pressure operand bound: `core/product_physics/src/lib.rs:2407-2414` (`exact_pressure_operand_bound`, γ₂₀·\|t\| plus 2⁻¹⁰⁷⁴ when subnormal) | Mutant **RV-M2** multiplies the bound by 0. **It survives all 506 product_physics tests.** T8 checks only that the token `exact_pressure_operand_bound(` is present. So the one `Bounded` family on a fresh-solve path can lose its bound unseen (fail-open). I checked k = 10 against the source (`pressure_exact.rs:84-92, 184-197, 422-424`; `pressure_runtime.rs:704-728, 845-880`; the terms are ±2ν and ±1, exact): γ₂₀ is sound | Add a unit test pinning `exact_pressure_operand_bound(v)` = RU(γ₂₀·\|v\|) for a normal v, and the subnormal branch. Preferably also add a product test in which an exact-pressure case's operand row carries B > 0 in `formation_rows`. It kills RV-M2 |
| **RV4-S4** | **SHOULD-FIX** | The curved thermal family: `lib.rs:9202-9215` (`RoundedProduct { k: K_rc, a: ε, b: chord[c] }`) | Mutant **RV-M3** takes b from the wrong chord axis. **It survives all 506 tests.** No S11-G test solves a realized curved span with a thermal load, and the existing curved-thermal tests (`lib.rs:21627`, `:21668`; `s11f_tests.rs:1658`, `:1696`) assert values, not the integrity code. The family is reachable: a thermal load on a realized curved span. A wrong operand record mis-states the defect, silently: false demotion here, and masking in other forms. Otherwise I confirmed by reading that the operands match the product (`curved_bend_free_expansion_displacements`, `lib.rs:9225-9234`) | Add a product test: a realized curved span with a thermal load and a Passed report. Its precondition is A_se ≠ 0 at a bend DOF, computed from `formation_rows`. It pins `CHECKS_PASSED`, so the self-equilibrated floor holds. Optionally add a unit check that each record's k·fl(a·b) equals the pushed term. It kills RV-M3 |
| RV4-N1 | NOTE | The S\* rule for restrained rows: `formation_guard.rs:284-288` | Mutant **RV-M10** gives restrained rows the free-row maxima. It survives all 506 tests. It errs in the fail-closed direction (smaller scale, more demotion), and the converse (M5) is killed | Optional: a test with a formed restrained row whose scale differs between the two rules |
| RV4-N2 | NOTE | E-1's condition: `lib.rs:2742` | Mutant **RV-M6** passes `attempt_err = false`. An ordinary-`Err` guard-fired case then gets the zero-work decline instead of main's real attempt. **Only T10b's source pin kills it.** No behavioural test exercises an ordinary-`Err` case with a load-row finding. T18 covers the already-Sensitive arm | Optional: a behavioural test of an ordinary-`Err` guard-fired case keeping main's attempt |
| RV4-N3 | NOTE | D22-1's ledger equality: `source_receipt.rs:880-881` and `:910-921` | The attempt ledger (`charged`, `failed_charged`, `attempts`, `rejected`) equals main's (T22; RV-M7 killed). In a multi-case invocation with a receipt, the publication reservation still grows by 32 + 12 × (the size of the added info diagnostic), plus the entry difference. V1 specified that diagnostic, and it matters only within a few kB of the 64e6 limit. T22 exercises a single-case invocation with no receipt | Record only |
| RV4-N4 | NOTE | T1's pin: `s11g_tests.rs:835-897` | The test checks the `SOURCE_BLOCK_*` and `NUMERICAL_INTEGRITY_*` codes, not the design's "code set equal to the unguarded run's except…". My base-against-candidate probe confirms the design form (§8): in both modes the only change is +1 `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) and CHECKS_PASSED → SENSITIVE, and `/results` is equal | Optional |
| RV4-N5 | NOTE | T6a's disclosure: `CHANGE_RECORD.md:94-97` | The curved pressure caps (`RoundedProduct`) and the curved wall vector (`CannotBound`, `lib.rs:9112-9139`) are just as unreachable on a fresh solve, by the same refusal (`pressure_runtime.rs:212-224`), and have no S11-G test. Only the straight thrust is named | Extend the disclosure |
| RV4-N6 | NOTE | Hygiene | PP `lib.rs` has 156 rustfmt diffs (stable rustfmt 1.8.0), against base's 147: the new code adds 9 (for example `lib.rs:2714`, `:9078-9079`, `:9121-9137`, `:9175-9176`). I5 disclosed the choice. Every other touched `.rs` file is clean. `git diff --check` flags trailing whitespace only in two records (`mutations/failing_tests_per_mutant.txt`, 32 lines; `reader/vitest.txt`, 1 line) | Optional |
| RV4-N7 | NOTE | Operand bounds: `lib.rs:8837-8858` | The γ₄ operand bound applies only to `{case}:generated:` loads (the equivalent-static generation). Applied self-weight loads (`load:self-weight:…`) get 0: they are stored inputs, behind the held-operand boundary. Design §3.2 says "γ₄ for generated intensities (self-weight)". The bound is additive only (fail-closed). γ₄ covers only q = m·g·f; m's own formation (which includes od² − id²) is behind N-3's boundary | Record the reading in CHANGE_RECORD |
| RV4-N8 | NOTE | 0.4.0 diagnostic text: `lib.rs:2797-2810` | For a Passed guard-fired 0.4.0 case, `retained_source_attempt` now reads `unavailable` (captured) or `not_eligible` (typed) instead of `not_required_ordinary_checks_passed`. This follows from G-2 and changes no committed byte | Record only |
| RV4-N9 | NOTE | R-b′'s floor at absurd scale: `formation_guard.rs:52-54`, `:430` | When S\*_moment < 2⁻⁹⁸⁸ (about 1.5e-298 N·m), the floor clause is false and R-b′ never fires. That is fail-open against §4's literal q ≥ 2⁻³⁴·S\*. It is not listed among the CHANGE_RECORD deviations | Record as a deviation |
| RV4-N10 | NOTE | T13: `s11g_tests.rs:1567-1591` | The design's precondition (R-b's clauses hold at end i) is not asserted; the comment says R-b′ is silent there. The no-op rule is pinned by T13b and by the ruling-1 test instead. CHANGE_RECORD's "The formation list" section (it "is to be emptied") is stale at `759dccf35`, which empties it | Optional wording |

## 2. Design conformance (brief check 1)

- **Formed and input tagging at every push site.** The T8 table matches S11-F's rule-5 producer list, and each flag matches §3.2:
  - nodal and constant-effort: input;
  - straight uniform: `Exact`;
  - curved uniform and curved wall: `CannotBound`;
  - straight thrust, thermal and eigen, curved caps and curved thermal: `RoundedProduct`, self-equilibrated;
  - exact-pressure: `Bounded` γ₂₀.

  I checked each record's operands against the value pushed:
  - thrust and thermal: a = `axial_load`, b = `local_x[axis]`, value = a·b;
  - caps: identical expressions;
  - curved thermal: a `Product(K, fl(ε·c))` term with a = ε, b = chord[c];
  - `Exact`: SP's formula over the same q_g, L, (a, b) and T. I checked the six polynomials against the fixed-end integrals (for a full span they give qL/2 and qL²/12).
- **Two exact accumulators.** `load_ledger.rs:470-575`.
  - `Exact` terms: 12·value − (12/scale)·Σc.
  - `RoundedProduct` terms: 3 × (−4k·lo).
  - The FMA condition is tested on fl(a·b) < 2⁻⁹⁶⁹. I checked that this equals the exact condition: an exact a·b just below 2⁻⁹⁶⁹ cannot round up to it, because two 53-bit mantissas have a product at least 2⁵⁴ below 2¹⁰⁶.
  - D21-2: γ₂·\|v\| + \|k\|·2⁻¹⁰⁷⁴.
  - Overflow gives a range failure, which fires.
- **The decision.** `formation_guard.rs:216-254`.
  - The first clause is `B > 0 && B ≥ T0` (ruling 2).
  - The second and third clauses are decided exactly in accumulator copies.
  - T0 and Tf are `product_downward(RD(1e-9), …)`; `CRITERION` is `1e-9.next_down()`.
  - `CannotBound` or a range failure fires.
  - S\* follows §3.4: free rows over the body's free rows, restrained rows over all its loaded rows, and the coupling rounded downward.
- **R-b′.** `formation_guard.rs:421-432`: B > RD(1e-9)·q exactly; q > 2¹⁰·B; q ≥ 2⁻³⁴·S\*_moment. B comes from SP's `bending_formation_bound` (exact sums rounded upward, then γ₁₆). S\*_moment comes from the case's published rows over the closed kind table.
- **The no-op rule.** `demote` changes only a `CHECKS_PASSED` record.
- **`source_eligible`.** It is main's predicate (G-1).
- **No new field, code or `Err`.** Every code used already exists, and the guard itself returns no `Err`.

## 3. The attack log (brief check 2: hiding a real formation defect)

| # | Attempt | Outcome |
|---|---|---|
| A1 | Defects that cancel within A_net | Not a hide. A_net is the row's exact net defect; if it cancels, the published row is exact (probe A; T5) |
| A2 | A formed term tagged as input | Caught by T8, and behaviourally by T1 and T2 (M2 is killed) |
| A3 | A formed term tagged as self-equilibrated, to get the floor | The straight thermal pairs flipped (RV-M4) are caught by T6, T6a, T6b and T8. Every family's flag matches §3.2 (T8) |
| A4 | A family whose record uses a different operand from the product | By reading, every family matches (§2). **Curved thermal is unpinned by any test (RV4-S4).** |
| A5 | A family whose bound is zeroed | **Exact-pressure: RV-M2 survives (RV4-S3)** |
| A6 | Hiding in the ±12B band of the second test | **RV-M1 survives (RV4-S2)**. The code itself is correct |
| A7 | A defect routed into A_se and hidden by the floor | Only the self-equilibrated families feed A_se, each defect ≤ u·\|t\|. The DN-4 residual is disclosed. M17 (floor on the whole row) is killed by T6b |
| A8 | Underflow and overflow edges | FMA underflow gives `Bounded` (RV-M9 is killed by the ledger unit test). 12× overflow and a non-finite bound give a range failure, which fires. A subnormal T0 fires any nonzero defect. An R-b′ bound of inf fires. The floor clause is false for S\* < 2⁻⁹⁸⁸ (RV4-N9, absurd scale) |
| A9 | A recovery row just under R-b′'s floor or ratio clauses | These are the design's accepted limits. T12 (LARGE-CONT, below the floor) stays Passed, and M13 is killed. X4 and X7 are killed at unit level. Dropping Mz from q (RV-M5) is killed by T11 |
| A10 | A demotion bypassed through routing (a selected guard-fired case) | G-3 declines the selection (T21; I5's M22). The E-1 zero-work decline is used only when main would not attempt. A selected case publishes exact projections, which R-b′ correctly skips |

## 4. R-b′ and the mutation table (brief checks 3 and 4)

Mutants run by `mutations/rv4_mutants.py`, which reuses I5's driver (an unchanged copy). Each run patches the scratch tree, runs the command, restores the bytes and checks the sha256. There was no compile error, and no killing command ran 0 tests. The full per-mutant failing tests are in `mutations/failing_tests_per_mutant.txt`.

| Mutant | Change | Killed by | Verdict |
|---|---|---|---|
| M1 | An `Exact` defect set to 0 | T1, T2 | KILLED |
| M2 | Straight uniform term pushed as an input | T1, T2, T8 | KILLED |
| M5 | Free-row S\* taken over all rows | T1, T2 | KILLED |
| M10 | Families combined in binary64 | `m10_families_are_combined_exactly` | KILLED |
| M12 | Floor taken from all formed terms | T1 | KILLED |
| M13 | R-b′'s floor clause dropped | T12 | KILLED |
| M14a | Signed sums in B | T11 | KILLED |
| M14b | \|Tu\| in B | T17 | KILLED |
| M17 | Floor applied to the whole row | T6b | KILLED |
| M18 | The binary64 literal 1e-9 | `m18_threshold_constant_is_rounded_down` | KILLED |
| M23 | The charged attempt restored (D22-1) | T22 | KILLED |
| MR2 | `B ≥ T0` restored | `ruling2_boundary…`, T6 | KILLED |
| MR2-T6 | The same, with T6 alone | **T6** | KILLED (Addendum 1) |
| RV-M1 | D21-1's +12B dropped | none, in 506 tests | **SURVIVED** (RV4-S2) |
| RV-M2 | Exact-pressure bound × 0 | none, in 506 tests | **SURVIVED** (RV4-S3) |
| RV-M3 | Curved thermal operand from the wrong axis | none, in 506 tests | **SURVIVED** (RV4-S4) |
| RV-M4 | Thermal pairs not self-equilibrated | T6, T6a, T6b, T8 | KILLED |
| RV-M5 | q from My only | T11, T12, T16, T20 | KILLED |
| RV-M6 | E-1 ignores `attempt_err` | T10b (source pin only) | KILLED (RV4-N2) |
| RV-M7 | The zero-work decline counts an attempt | T22, site rule 8 | KILLED |
| RV-M8 | The zero-work decline has limit 4e6 | T19, T23 | KILLED |
| RV-M9 | The FMA underflow fallback removed | `s11g_range_failures_fall_back_or_fire_and_never_err` | KILLED |
| RV-M10 | Restrained rows use the free-row S\* | none, in 506 tests | SURVIVED (RV4-N1) |

**R-b′ is load-bearing and well pinned.** Its clause, bound and scale mutants all fail behavioural tests:
- M13, M14a and M14b;
- I5's X1–X7 (from I5's record; I re-ran none of them);
- RV-M5.

Of the killed mutants, only RV-M6 dies by a source pin alone.

## 5. Pins (brief check 5)

- Every verdict pin I read asserts its paths-differ precondition inside the test: T1–T6b, T11, T12, T15, T16 and T18–T22. The exception is T13 (RV4-N10).
- T10 is behavioural and unit-level.
- T10b is a source pin. It is backed behaviourally by:
  - T18 for M19;
  - T19 for M20 and M21;
  - T21 for M22;
  - T22 for M23.

## 6. The receipt residual (Addendum 1 item 3; Addendum 2 item 4; Addendum 3)

**My construction.** C1 is described in RV4-S1, with the probe at `probe/rv4_probe.rs.txt` and its output at `probe/rv4_probe.stdout.txt`.
- **Pre-0.4 captured: refused, fail-closed.** `run_linear_static_preview_value_with_mode` returns `Err` with no envelope (`lib.rs:1583-1588`).
- **The consumers propagate the `Err`.**
  - The headless runner: `run_preview_with_producer`, `produce()?` (`runner/headless/src/lib.rs:867`).
  - The desktop: `solve_preview_mechanics_with_mode`, `…?` (`apps/desktop/src-tauri/src/lib.rs:1562`).
  - The examples use `?` too.
  - None of them builds an export or qualification from a refused envelope.
- **The typed entry** has no capture and no receipt. It publishes case B as Sensitive, which is correct.
- **0.4.0 captured.** From the code, not run: the finalization failure is recorded (`lib.rs:2154-2158`), and `run_linear_static_preview_captured` republishes on the ordinary route with every attempt withheld (`lib.rs:1605-1625`). R-b′ demotes again on the rerun, so the value is published as Sensitive, as ROOT ruled (E-3).
- **No reachable path publishes a value from the refused invocation.** So the residual is not BLOCKING. The disclosure is wrong, which is RV4-S1.

**The reach per slice (I5's condition (a)).** I agree with the code facts: S11-F's `LOAD_CONTRIBUTION_ABSORBED` and K-D5 act on the kernel report before routing (FK `structural.rs:1400-1404`), and G-2 routes the load-row finding. The R-b′ variant remains, as C1 now demonstrates.

## 7. The reader window (Addendum 1 item 2; my own enumeration)

The window runs from the appends at `lib.rs:2944` (linear) and `:2985` (nonlinear) to the amendment at `lib.rs:3783-3802`, all inside `solve_load_case`. I enumerated every reader of the integrity code, `solve_quality`, `numerical_quality` or the standing.

**None lies in the window.**
- `assessed_numerical_quality` (`lib.rs:1241`) is called at `:2103` and `:11932`, after every case has been solved.
- The case-level receipt finalization (`lib.rs:3864-3910`) and `qualify_source_case_rows` come after the amendment.
- `OrdinaryAttempt::wire` and the invocation receipt (`lib.rs:2143-2159`, `source_receipt.rs`) run after all cases.
- `preview_physics` rendering is at envelope build.
- The headless export and digests (`runner/headless/src/lib.rs:733-760`) and the result_export and desktop readers consume only the final envelope.

**What runs inside the window, and why none of it reads the verdict.**
- `append_load_contribution_absorbed` only pushes.
- `RECOVERY_BASIS_UNQUALIFIED` pushes are never CHECKS_PASSED.
- The support, constant-effort, element, stress, exact-pressure and component recovery calls only push rows and diagnostics. None of `append_expansion_joint_pressure_thrust_results`, `append_constant_effort_support_results`, `append_exact_pressure_results`, `append_component_stress_multiplier_results`, `append_signed_support_results`, `append_endpoint_stress_results` or `append_element_force_results` reads a diagnostic code.

**Tests.**
- R-b′ alone: T11 checks the final envelope, `numerical_quality` case and status, and the reason sentence exactly once.
- Both guards: `ruling1_both_guards_fire_and_the_load_row_sentence_stands` shows the load-row sentence stands and R-b′ is a no-op.

## 8. The gate (brief check 6; ROOT's condition)

**My re-run** is at `gate/rv4_gate.py` and `gate/rv4_gate_table.md`. It uses the base and candidate fixdiff probes, on every entry each case accepts, in both modes.

**All 14 rows go from CHECKS_PASSED to SENSITIVE:**
- UDL-W1e8: captured and typed, load-row guard;
- UDL-W1e80: typed, load-row guard;
- F- and M-INPLANE: typed, R-b′.

**No `/results` leaf differs, and the envelope status is not `checks_passed`.** On captured UDL-W1e8 the only diagnostic change is +1 `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info, "formation guard") and the integrity code (`gate/udl_w1e8_captured_codes.txt`). T16 passes in the suite.

**`FORMATION_EXCEPTIONS.json` at `759dccf35`** (`gate/check_emptied.py`, `.out.txt`):
- `rows` = [] and `counts` = 0/0;
- `emptied.pinned_before_s11g.rows_sha256` = `70c0ff53…`, which equals sha256(json.dumps(previous rows)) recomputed from `759dccf35~1` (`454bbc24…`);
- the owners, counts and source are equal to the previous file's.

`GATE/pin_s11_exceptions.py`, re-run in a scratch copy with Python 3.11.15, reproduces `FORMATION_EXCEPTIONS.json` (`0e110b4b…`) and `S11_EXCEPTIONS.json` (`138515b3…`) byte for byte. `GATE/SHA256SUMS` verifies. The numerics branch's GATE files are identical.

## 9. Zero committed-byte diff, T18 and D22-1 (brief check 7; Addendum 2)

**T9, re-run.** I5's fixdiff harness (behaviour unchanged) ran every committed request or model under `fixtures/`, `core/` and `validation/`, on the captured and typed entries, in both modes: 218 outputs per tree.
- `t9/base_outputs.sha256` equals `t9/candidate_outputs.sha256`, and both equal I5's manifests.
- 6 outputs are ERR on both trees.
- 0 outputs contain "S11-G".
- 88 carry SENSITIVE on both trees.

Nothing legitimate is demoted, including the 221 through S11-F's pins in the PP suite, UDL-W1e5 (T3), probe A (T5), and the collinear and pressure runs (T6a).

**T18.** The envelopes are byte-identical between base and candidate (dense `832319d7…`, sparse `5cf0c071…`), equal to I5's.

**D22-1.**
- The zero-work entry (`source_recovery.rs:70-88`) is used only when main would not attempt (`lib.rs:2742`). Already-Sensitive cases keep the real attempt (T18 is byte-equal to base).
- The invocation ledger equals main's (T22). M23 and RV-M7 are killed.
- The readers accept the entry:
  - result_export `source_blocks::validate`, with and without the invocation (T23 passes);
  - the desktop `validateSourceBlockRecovery`, a scratch vitest run of I5's test on my regenerated T23 and T18 envelopes (hashes equal to I5's; 2 of 2 pass; the standing equals main's `unsupported` shape; base is eligible).
- The WORK_LEDGER checks (`result_export/src/source_blocks.rs:933`; `sourceBlockRecovery.ts:185-187`) and FAILURE_CATEGORY (`source_blocks.rs:986`) hold for {0, 0, 0}.

**G-1 to G-3.** Present as specified. `OrdinaryAttempt::passed` gains only a constructor argument (`source_receipt.rs:454-470`), and `wire()` and the finalize bodies are unchanged. `decline_formation` is at `source_recovery.rs:254-265`.

## 10. T6a (Addendum 3)

**I5 took option (b).** I checked each piece:
- **Unreachability** at `pressure_runtime.rs:207-224` (legacy nonzero pressure refused) and `:236-241` (exact profile). `historical_pressure_reference` is `#[cfg(test)]` (`lib.rs:97-98`).
- **The unit test:** `load_ledger::tests::s11g_rounded_product_defect_is_exact`.
- **The disclosure:** CHANGE_RECORD deviation 4.

**My independent attempt to reach the family** went through every entry:
- the captured and typed entries, the headless runner and the desktop, which all pass `validate_profile` at `lib.rs:1645`;
- 0.4.0 load states: `effective_case` scales an authored magnitude that was already validated nonzero-refused (`case_state/resolve.rs:1063-1078`), and 0 × factor stays 0;
- equivalent-static generation, which produces only uniform loads;
- expansion-joint effective area, which uses the same pressure primitive;
- the exact profile, which refuses pressure primitives, even zero-valued ones;
- values of −0.0 and subnormals, which are refused (≠ 0) or stay exactly 0.

`build_pressure_thrust_loads` (`lib.rs:8934-8971`) is the only producer, and it needs a pressure-category, pressure-dimension primitive. **It is unreachable, so option (b) holds** (see RV4-N5 for the curved analogues).

## 11. Disclosure and the K-D5 boundary (brief checks 8 and 9)

**Disclosure.** CHANGE_RECORD states:
- the CannotBound availability loss;
- DN-4, routed to W1/F2;
- the formation list (the wording is stale: RV4-N10);
- no in-band marker;
- deviations 1–5.

The residual wording is wrong (RV4-S1).

**The K-D5 boundary.** `git diff 72d5ff864..759dccf35` shows no change to SA, FK `structural.rs`, `solve_preview_reduced_system` (PP `lib.rs:4332`, outside every hunk), CB, `pressure_runtime`, `serialization` or `units`.

## 12. Hygiene (brief check 10)

- **Machine paths.** A GEN-8-style scan of the S11-G records and GATE finds none. My records use placeholders.
- **rustfmt and `git diff --check`:** RV4-N6.
- **I5's records.** `SHA256SUMS` verifies all 28 entries. `generators/gen_rb_controls.py`, re-run, reproduces `rb_controls.json` (`c12b3cb9…`) byte for byte.
- **The authority targets** were built in my copy with the two tools; nothing else was touched.

## 13. What I ran

Environment:
- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `CARGO_NET_OFFLINE=true`, `--offline --locked`;
- my own target `<scratch>/target`, with debuginfo off, pruned after each job;
- one cargo job at a time, inside the slots the manager granted;
- Python `<venv>` 3.11.15; Node 24.21.0.

| Run | Result |
|---|---|
| Authority builds (`build_checked_json.py` for both profiles; `build_units_authority.py`) | exit 0 |
| `cargo test --no-fail-fast`: FK, SP, PP, headless | 144, 42, 505 (+1 ignored, pre-existing) and 84 passed; 0 failed (`suites/`) |
| `rv4_probe` (C1) | §6 |
| Mutations (23 plus 4 whole-suite survivor runs) | §4 (`mutations/`) |
| T9 fixdiff, base and candidate (release) | §9 (`t9/`) |
| Gate, T18 and T23 probes | §8, §9 (`gate/`, `reader/`) |
| Desktop reader vitest (node_modules hardlinked from the engine worktree; the wasm artifact copied, `cb40d116…`) | 2 of 2 passed (`reader/vitest.txt`) |
| The GATE generator and `rb_controls` generator re-runs | byte-identical |
| rustfmt `--check` (stable 1.8.0, nothing installed) | RV4-N6 |

## 14. What I did not check

- Crates other than FK, SP, PP and headless, and the Python consumer groups. Their inputs are unchanged, and T9 shows zero byte change.
- src-tauri tests.
- I5's X1–X7 and M3, M4, M6, M8, M9, M11, M15, M16 and M19–M22. I relied on I5's records and my reading of the tests.
- A 0.4.0 run of C1 (the republication follows from the code).
- The frozen-reference false-demotion forecast (LFRAME, WEAK) on the product.
- CI logs and the DEC-025 sweep result, which are the manager's.

---

## Delta check: 759dccf35..e6f45d30f (the RV4 repair)

**Verdict: PASS**, for head **`e6f45d30ff1593d9032146fe32528cafccda6b2a`**. There are no BLOCKING and no SHOULD-FIX findings; there are 2 NOTEs.
- The delta contains tests, records and layout only.
- PP `lib.rs` is token-identical to `b62e40d4d`.
- **No product behaviour changes, so the no-re-sweep condition holds.**

**What was checked.** The brief was ROOT's delta request (items 1–5), covering:
- **Range:** `759dccf35..e6f45d30f`, which is two commits.
  - `6d6d31923`: I5's repair.
  - `e6f45d30f`: records only. It restores the hash-bound mutation driver and keeps the updated driver as `s11g_mutants_rv4_repair.py`.
- **How:**
  - I read the diff from the pushed head (`git ls-remote` confirmed `e6f45d30f`).
  - I built and tested a fresh `git archive` of `projects/chirality-piping` (excluding `execution/`), with my own target.
  - Outside `execution/`, `6d6d31923` and `e6f45d30f` are byte-identical (`git diff --quiet`; the `core` tree is `7bd0ca35…` in both).
- **Records:** in `_run_records/s11g_review/delta_e6f45d30f/`.

### D1. Scope and layout: the no-re-sweep condition (ROOT item 4)

**Which files changed.** Outside the S11G records, `git diff --name-only 759dccf35 e6f45d30f` shows exactly two files: `core/product_physics/src/s11g_tests.rs` and `core/product_physics/src/lib.rs`.
- **No other product, test, fixture or GATE file changed.** That includes FK, SP, `formation_guard.rs`, `source_recovery.rs`, `source_receipt.rs`, the site test, headless, the schemas and GATE.

**PP `lib.rs` is layout-only.** I checked this independently with `delta_e6f45d30f/layout_check.py` and `layout_check.out.txt`, which walk both files with all whitespace removed.
- **Result:** against `b62e40d4d` (which equals `759dccf35` for this file), the only differences are **10 commas added before `)` or `}`**. There are no other differences, and both texts are consumed completely.
- **So no token, comment or string changed.**
- **rustfmt `--check`** (skip_children, stable 1.8.0), counting Diff-in blocks:

  | Revision | Diff-in count |
  |---|---|
  | Repair head | 78 |
  | Base `72d5ff864` | 78 |
  | `b62e40d4d` | 87 |

  So RV4-N6 is closed.
- **`git diff --check 759dccf35 e6f45d30f`:** clean.

### D2. S1: T20 on C1 and the corrected disclosures (ROOT item 1)

**The test.** `t20_characterization_rb_prime_residual_c1` is at `s11g_tests.rs:2261-2326`, with helpers `c1_case_b` and `c1_request`. It is my construction C1 (F = 1 N, m = 1e-7 N·m). In both modes it pins:
- **the typed entry:**
  - no receipt;
  - case B's report is Passed, and R-b′ fires at the tip;
  - the tip row's relative error exceeds the criterion, computed in the test against the exact |m|;
  - case B is published demoted (`assert_demoted`, which covers the code, the sentence, `numerical_quality` and "never refused");
- **case B alone, captured:** demoted and not refused;
- **the two-case captured invocation:** `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`. The captured entry returns no envelope on `Err`;
- **the control at m = 0.5:** Ok with a receipt; cases A and B are both `qualified`; case B is `CHECKS_PASSED` with no S11-G text.

**The labelling** is right: it is labelled a characterization of a known residual, and its comment names the owner, the pre-0.4 scope and the 0.4.0 republication.

**The disclosures are corrected.** Each now says reachable and fail-closed, and "not demonstrated reachable" is withdrawn:
- CHANGE_RECORD "Residuals";
- RETURN §6a and §12;
- the test comment;
- the PR #1003 body.

T20 passes.

### D3. S2–S4, RV-M6 and RV-M10: my patches, re-applied (ROOT item 2)

**How.** I re-applied the same patches, using my driver `mutations/rv4_mutants.py` on an unchanged copy of I5's `b62e40d4d` driver. All five anchors are unique on the repair tree. Each mutant ran the S11-G surface: every `s11g_tests::` test, the site test, and the FK and SP S11-G unit tests. Results are in `delta_e6f45d30f/kills.txt` and `mutations_results.json`.

**Every mutant is killed by a behavioural assertion**, with no compile error and no command that ran 0 tests:

| Patch | Killing test | Failing assertion |
|---|---|---|
| RV-M1 (D21-1's +12B dropped) | `d21_1_second_test_adds_the_bound_exactly` | `s11g_tests.rs:700`: `fires` (mutant: `fires: false` at net ratio 1.15) |
| RV-M2 (exact-pressure bound × 0) | `exact_pressure_operand_bound_is_gamma_20`; `exact_pressure_operand_rows_carry_their_bound` | `:716`, the value pin (left 0.0); `:2372`, a product operand row with bound 0 (`case:cold-pressure`, dof 0) |
| RV-M3 (curved-thermal operand from the wrong axis) | `curved_thermal_records_name_the_pushed_products` | `:2465`: A_se ≠ the independently formed −12·Σ K_rc·lo(ε, c) (row 0) |
| RV-M6 (E-1 ignores `attempt_err`) | `e1_ordinary_err_guard_fired_case_keeps_mains_attempt` (also T10b) | `:2532`, the **captured pin** ("main's real attempt, not the zero-work decline"; the mutant's message is `stage: "formation guard"`). This comes **after** the typed-entry precondition, which passes under the mutant: the typed entry has no retained-source route |
| RV-M10 (restrained rows use the free-row S\*) | `restrained_rows_take_the_all_rows_scale` | `:2595`: root RZ is silent under its own scale (the mutant fires at net ratio 34.1); the product pin follows |

**The new tests are sound.**
- **S2:** T0 is independent of B and of the defect (the intended net is 1.5 N). B = T0/4 and d = 0.9·T0, so d lies in (T0 − B, T0].
- **S3:** the product test uses the committed exact-pressure request and the product's own builder and producer, and decides B > 20u·Σ|t| exactly.
- **S4:** the chord (1.2, 1.6, 0) m makes the two axes' lo values differ, and A_se ≠ 0 is asserted.
- **RV-M6:** uses N06's model, whose ordinary attempt errs at assembly.
- **RV-M10:** the precondition shows the free-row scale fires on the same row.

### D4. The NOTEs taken (ROOT item 3)

- **N10: T13's precondition.** T13 now asserts that R-b's two clauses hold at end i (`s11g_tests.rs:1657-1664`).
- **N5: curved pressure unreachability.** Disclosed as CHANGE_RECORD deviation 5, with the same refusal cited (`pressure_runtime.rs:207-224`). The `CannotBound` record is covered by T15, and the cap record by the ledger unit test.
- **The stale formation-list wording.** CHANGE_RECORD "The formation list" now says the list is empty at `759dccf35` (`37bdc2edd` on T3).
- **N3: reservation growth.** Stated in CHANGE_RECORD "What firing does".
- **N9: 2⁻⁹⁸⁸.** Deviation 6.
- **Also:** N7 is recorded as deviation 8, and N6 as deviation 7 (see D1). N4 and N8 are left, as ROOT ruled.

### D5. Records (ROOT item 5)

- **SHA256SUMS** (`IMPLEMENTATION/S11G/SHA256SUMS` at `e6f45d30f`): 47 entries for 47 files, all verify.
- **Hash-bound evidence from `b62e40d4d` is byte-unchanged.** Of the files committed at `b62e40d4d`, only CHANGE_RECORD.md, RETURN.md and SHA256SUMS changed. The run evidence all has its `b62e40d4d` bytes, including:
  - `mutations/s11g_mutants.py`, restored at `e6f45d30f` to `0d091feb…`;
  - `mutations/results.json`, `0ee4bd39…`;
  - `mutations/failing_tests_per_mutant.txt`, `5a4190dd…`;
  - every fixture-diff, gate, reader, suites and callers record.
- **Earlier finding closed.** The in-place edit of `s11g_mutants.py` that I raised on `6d6d31923` is fixed by `e6f45d30f`: the updated driver is the new `s11g_mutants_rv4_repair.py`.
- **Machine paths.** A scan of the S11G records for home, temp and root paths finds none.
- **GEN-8** passes (below).

### D6. Runs

Environment as in §13: toolchain 1.97.1, `--offline --locked`, my own target with debuginfo off, pruned afterwards. It ran in the manager's cargo slot.

| Run | Result |
|---|---|
| Authority builds | exit 0 |
| `cargo test --offline --locked --no-fail-fast`, product_physics (repair tree) | **511 passed, 0 failed, 1 ignored** (pre-existing). All new tests, T13 and T20 pass |
| RV-M1, M2, M3, M6 and M10 | all KILLED (D3) |
| Layout check and rustfmt counts | D1 |
| `pytest tools/practitioner_harness/test_live_baseline.py -k gen8` (from `<wt>/numerics`) | pass |

**Why not re-run T9, the gate or other crates.** Only `s11g_tests.rs` and a token-identical PP `lib.rs` changed, so the product binary's behaviour is unchanged. The T9, gate, T18 and T23 evidence of §8–§9 carries over.

### Delta findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV4-D1 | NOTE | PR #1003 body | The status line reads "Status on head `6d6d31923`"; the head is now `e6f45d30f`. The content is otherwise current | Update at the next body edit |
| RV4-D2 | NOTE | `s11g_tests.rs:2595` | The kill is at the assertion that the root row is silent under its own scale. That scale comes from the product's `row_scales`, so it is a behavioural assertion on the guard's decision. The product-level pin (not demoted) follows it. Acceptable. Recorded because the assertion is written with the precondition | None |
