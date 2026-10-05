# RV79 review: the Python retained-precision reader at READER `6b607fd01f`

RV79 is a TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) under `BRIEFS/RV78_RV81_READER_REVIEW.md`. ROOT is the return path. RV79 did not write any of the code, tests or corpus under review, did not delegate, and did not use the author's tests as oracles.

- **Run:** first tool call 2026-10-03T22:47:25Z; report frozen about 23:14Z, about 27 minutes into the 2-hour box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job. Every run used `VENV/bin/python` with both prebuilt helper binaries.
- **Paths** use the placeholders WT, P (= `projects/chirality-piping`), NUM, R (= the RESUME_2026-09-30 folder), T3 and VENV.

## Verdict: FAIL

| Severity | Count |
|---|---|
| BLOCKING | 2 |
| SHOULD-FIX | 5 |
| NOTE | 7 |

The arithmetic is exact and agrees bit for bit with an independent rational oracle. The I57 coverage rules are implemented as specified, and eligibility is closed on every path. The FAIL comes from two groups of false accepts. Python validates receipts that break explicit C1, C2, C3, F1 and S06 association and native-record rules. Eleven single-defect probes show this (B1 four, B2 seven), and two more items are confirmed by reading the code. Three further accepted probes are open contract readings (N4). Rust or TypeScript already enforce most of these rules (see the named questions). Five SHOULD-FIX items cover first-failure gates and order (G0, G3, within-G5 and the G5a/G5b label) and test isolation.

## Candidate and environment

| Item | Value |
|---|---|
| READER head | `6b607fd01f9819a3b6526dd9fde02cd3bc4db586`, tree `cd9938a5d3`; reviewed from `git archive` into WT/rv79 (deleted afterwards) |
| `P/core/analysis_runs/retained_precision.py` | `55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f` (last changed in `f1ff7ebddc`) |
| `P/tests/test_retained_precision_contract.py` | `354821ddf449b4bdd0304fd16bf5d78e596d71e96358204a30f09fa977a82de3` |
| `P/tests/test_retained_precision_schema.py` | `90bbd5660c504d3344ce19756d2b2250f80c0bd5d93b3ec123e5abf10523e504` |
| Corpus (input) | `d02701ed6afb82fdd8d925900119185412c3169276ed281423dff7dd726afdf4` (06d: 15 cases, 178 mutations, 18 must-pass) |
| Schema (input) | `f943ebd3511e31bcdfe09bac2788362d3a5734040bc3088fbe585361b571cd21` |
| Imported modules | `core.serialization.canonical_json.adapter` (`_hash`), `core.units.adapter` (G8), `core.analysis_runs.compatibility._source_contract` (G7; `4874b0e25b`). None differs from the merge base with main; only `retained_precision.py` changed in these packages. |
| Helpers | `openpipestress_jcs_ijson` `1ecf4f51bf`; `openpipestress_units` `813a0dd5f6` (WT/targets/i52-readers, used as is) |
| Python / pytest | 3.13.14 / 9.1.1 |
| Native code read | NUM `6a5b131b98`: adaptive.rs `6a2fc382bf`, verify.rs `66022cc78b`, PP retained_product.rs `d07383fc02` |

**Baseline:** from WT/rv79/P, `VENV/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **221 passed** in 22.9 s (`baseline_pytest.log`).

## Findings

Line numbers refer to `P/core/analysis_runs/retained_precision.py` at the reviewed head. "Probe" ids refer to `probes_final.jsonl`. Each probe is rehashed with the shared harness, and validated against the base invocation.

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **B1** | BLOCKING | `_g5_products` 769–846; `_g5_stages` 690–695 | **Product-attempt association false accepts.** Each probe below is a single contract defect that Python validates (PASS):<br>(a) `R4_source_preparation_null`: a sourced attempt whose CaseSource.preparation is null. Python checks the back-reference only on Ready (826–827); G1 and G8 skip a null preparation (1399–1406, 1331). Breaks C3:145–148.<br>(b) `R5_ordinary_material_basis`: the ordinary attempt names another material basis. Python never reads `ordinary_attempts[*].material_basis_ref`. Breaks C3:165 and I57:190 (binding chain).<br>(c) `R6c_preparation_error_with_selected_run`: error `preparation` with a selected Run, every pipeline stage completed, and the case reason rewritten to source_unavailable/preparation. Breaks S06:37 ("preparation stage failed; no native Run was entered"). `_g5_typed` checks only the stage→error direction (724–729).<br>(d) `P_run_ref_null_with_case_run`: run_ref null and native not_entered, but the case carries this attempt's selected Run. 771 and 691–694 check only one direction. Breaks C3:167 ("run_ref is null iff no native call happened") and S06:32.<br>(e) Code reading: a `native{run_ref}` error's run_ref is never compared with the attempt's Run (grep: no read of `error["run_ref"]`). Breaks S06:38 ("Non-null same Run"). | Add each relation in the C3 association pass (G5 PRODUCT_ATTEMPT, before typed and WORK): source_ref ⇒ that source's preparation = {attempt_ref: id}; a.material_basis_ref = ordinary attempt's; case.run present ⇔ run_ref non-null (and native stage entered); error `preparation` ⇒ preparation failed ∧ run null; error `native` ⇒ run_ref = run.id. Pin each with a shared mutation. |
| **B2** | BLOCKING | `_g5_native` 604–622; 539–553; `_g5_schedule` 406–436; `_g5_ordinary` 748–752; G4 1440–1444; G8 1349–1361 | **Native-record, graph and coverage false accepts.** Each is a single defect that Python validates:<br>(a) `R8_phantom_group`: an extra group whose `call` (7) does not exist. The C5 partition only looks at groups whose call matches a real call. Breaks C2:117 and C2:135.<br>(b) `T3_candidate_with_verification`: a candidate-role record carrying a verification summary. Only verification passes write one (adaptive.rs:4593–4600; C1:28, 105).<br>(c) `T2_stop_rule_foreign_quantity`: a stop-rule rejection naming node 99, which is absent from the layout. Breaks C1:114 (exact quantity/body/kind).<br>(d) `T1_failed_verification_pass_entered`: an escalating failed verification whose record shows verification-pass work (verification_lme 1). Python skips two slots. Natively a pass failure is terminal (adaptive.rs:4593–4611).<br>(e) `G4_orphan_selected_diagnostic`: an extra RETAINED_PRECISION_SELECTED diagnostic naming a non-requested case. Breaks C1:147 ("exactly one … per corresponding case, exact affected_refs").<br>(f) `R3b_sourceless_complete_old_empty`: an attempt with `old_coverage=complete` and **no** old entries (P′ case 1). Nothing checks the inventory without a source. Breaks F1:101 ("old ids exactly equal the full member inventory"). Probably a shared gap: Rust checks only sourced attempts.<br>(g) Code reading: a published W2's `force_scale_exponent` is never required to be nonzero (C2:158, "nonzero b"). No native base exists, so this is reader logic only.<br>(h) `N13_rejected_verification_failed_with_completed_verification`: a candidate rejected with `verification_failed` although its verification completed. Natively that reason is set only on a failed verification (adaptive.rs:4576–4590; C1:27). | Enforce each rule: every group's call exists and the groups equal the union of the call partitions; candidate records have verification null, no v-build and verification_lme 0; reason quantities resolve to a layout row with that body and kind; a failed verification with pass evidence must not escalate; retained diagnostics name only requested cases; complete old coverage equals 0..n−1, with n from the source or, for sourceless attempts, from the invocation's pipes at G8; a published W2 has a nonzero exponent; `verification_failed` only with a failed verification. Add shared mutations, and a reader-logic test for (g). |
| **S1** | SHOULD-FIX | 491–492; 780; G3 1408–1437 | **G3 obligations are checked at G5.** The execution-order bijection fails at G5 ATTEMPT (`R1a/b/c` probes); C1:146 puts "execution-order bijection" at G3, and C2:117 makes run.id equal its execution-order position. The complete-inventory length fails at G5 PRODUCT_ATTEMPT (`R3_sourced_complete_old_long`); F1:130 and C3:302 put it at G3. A single defect therefore gives a different first failure from Rust (I63 items 1 and 3). | Move both checks to G3 COVERAGE_MISMATCH, ahead of G4. Pin each with a shared single-defect mutation. |
| **S2** | SHOULD-FIX | 1446–1454 vs 847–850 | **Within-G5 class order.** The ordinary checks (listed diagnostics, `not_attempted`, report outcome = solve_quality, not_required), the selected `product_attempt_ref`/source hash and the `rcond_label` check all run **after** the deferred product WORK list. Dual-defect probes D1, D2 and D3 (R1 adapter fault plus one of these) give G5 WORK; C3:304 orders native, then ordinary/association, then typed, then work, which gives ATTEMPT. RETURN_C2_1 states the C3:304 order for Python, so the code contradicts its own record. TypeScript runs these checks in the ordinary pass (I64 item 4). | Move 1448–1451 into `_g5_ordinary`. Move `rcond_label` into the native selected-summary checks (585–597). Move 1453 into the association pass. Pin one dual defect. |
| **S3** | SHOULD-FIX | 1455; 1473–1475; 1181 | **G5b/G5c exceptions are labelled G5a.** `gate="G5a"` covers all of `_g5_numeric`. A zero section area or modulus (selection and source equal; probes A1, A2) raises ZeroDivisionError at 1181, inside G5b, and is reported as **G5a** SCALE_MISMATCH. C1:150 puts the stress-scale and section obligations at G5b. A reader that divides in binary64 gets an infinite scale and fails at G5b; RV80 and RV81 to confirm for Rust and TypeScript. | Guard the division (a nonpositive area or modulus fails G5b SECTION or SCALE explicitly), or advance the gate label per phase. Add a shared mutation. |
| **S4** | SHOULD-FIX | G0 1374–1388 | **G0 thresholds.** C1:143 puts "all registered policy ids/**thresholds**" at G0, but Python checks only the four policy ids there. `case_limit` and `invocation_limit` are schema constants, so a changed threshold fails **G1** RECEIPT_MISMATCH (probe `G0_case_limit_threshold`). No shared entry pins this. | Check both thresholds at G0 (SOURCE_PRODUCER_CONTRACT_UNSUPPORTED), or have ROOT rule that schema constants are G1. Pin it either way. |
| **S5** | SHOULD-FIX | tests; corpus pins | **Implemented rules that no test kills.** Of 22 RV79 mutants, 10 survive. Three survivors are the deferred bases (see N6). Seven are implemented rules:<br>M06: a passed G5a with null coverage. The pin `cert_failed_before_summary_g5a_passed` also breaks the observables/G5a pairing, so it still fails G5 PRODUCT_ATTEMPT without the rule (`probe_extra_run.txt`).<br>M09: a values failure must be `separate_failure`.<br>M11: the 2^-988 small-scale switch of the absolute bound (C1:158); the arithmetic vectors do not straddle it.<br>M14: the result error must match the first failed stage (P8/P9).<br>M18: the strict interpolation bracket. The at-point pins also leave E/G stale, so they fail for another reason.<br>M21: the 1+2^-40 sanity factor.<br>M22: the body-extent summation order (my oracle checks it; no test does).<br>The harness also rehashes with reader functions (`rp._hash`, `_preparation_payload`, `_source_hash`), so a mutant there is masked in mutation entries (base hashes still guard). | Add isolating controls: rebuild the M06 pin with observables completed/passed and a storage cause; add values-failed/merged, error-versus-stage and E/G-consistent at-point mutations; add arithmetic vectors at S = 2^-988 and the next value below; add an extent vector whose result depends on summation order. RV78 may prefer an independent rehash in the shared harness. |
| N1 | NOTE | 491–632 | **Order inside the native class** (Rust item 7). C1:148 lists ATTEMPT and WORK together, and C3:294–304 fixes no order between them inside native checks. Python interleaves them per call and run, and checks groups/builds/snapshots after the call loop; Rust checks groups/builds before it. Neither contradicts the text. A second reading of C3:304 ("then work … equations") would put native WORK in class 4; the approved audit plan puts it in class 1. | ROOT fixes one canonical order and pins a dual defect. |
| N2 | NOTE | 1374 | A list `producer` or string `formulation_basis` raises an untyped AttributeError at G0 (`probe_extra_run.txt` §2). This fails closed but is not the G0 code. | Type-check before `.get`. |
| N3 | NOTE | 1435 | Parity rule 2 depends on a non-empty inventory. With the body membership and coverage both emptied (`PR2_empty_coverage_and_empty_inventory`), Python fails at G5a, not G3. | Reject `summary_coverage == []` explicitly at G3. |
| N4 | NOTE | 1446–1450 | **Open contract readings (TypeScript stricter).** `diagnostic_refs` entries that exist but do not name the case, and duplicate entries, pass in Python (T4a, T4b). A selected case with `checks_passed` quality and report passes (T4c2); C1:101 makes not_required the ordinary-pass branch. Neither W2's trigger and error nor the initial failure is cross-checked. | ROOT rules on each; then pin it. |
| N5 | NOTE | 172–173 | An integral float (22.0) in a U field is accepted at G2 and hashes like 22 (`probe_extra_run.txt` §3). This is JSON-Schema-consistent, but a strict-integer parser (likely Rust serde) may reject it. The two are not shown to diverge. | RV78 confirms cross-reader handling of integral float literals. |
| N6 | NOTE | — | Expected survivors on deferred bases: M05 (N17 overshoot), M12 (a budget failure is never cached) and M17 (L = 0). The status table already marks N17 partial; C1's budget-not-cached half is untested too. | Carry these with the deferred producer-solved witnesses. |
| N7 | NOTE | 1453 | The selected `source_identity_sha256` recheck at G5 cannot fire: G1 (1397–1398) already checks it for every in-range `source_ref`, and an out-of-range one fails in `_g5_native` first. | Remove it, or document it as defensive. |

## 1. Gate order and first-failure codes

- **Gates G0→G8 run in order** (1372–1469). Every case's G5a precedes any case's G5b, and every G5b precedes G5c: `_g5_numeric` collects G5b state, then runs the deferred G5c checks after all cases (1144–1191). C3_DELTA §4 and the ROOT 05b ruling require this.
- **Inside G5,** the order is `_g5_native`, then `_g5_ordinary`, then `_g5_products`: per-attempt association, then typed for every attempt (847–848), then the deferred WORK list (849–850). That follows C3:304, apart from the trailing block (S2) and the native interleaving (N1).
- **G7** reports the bare leading base code as `code` and the full text as `detail` (1463–1467), as ruled. Python's base gives `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, which matches `expected_by_reader.python`.
- **Codes:** the generic handler maps any KeyError, IndexError, TypeError, ValueError, OverflowError, ZeroDivisionError or StopIteration to the current gate's code (1473–1475). That mapping is right except where the gate label lags the phase (S3). G5 exceptions default to PRODUCT_ATTEMPT. I found no reachable exception in `_g5_native` after G1/G2, and every index there goes through `_at` with an explicit code.
- **Deviations found:** S1 (G3 at G5), S3 (G5b labelled G5a), S4 (G0 thresholds at G1), and S2 and N1 for dual defects.

## 2. The G5 checklist (42 IDs) against the code

I confirm the author's status (RETURN_C2_1 plus RETURN_C2_5) for N1–N4, N6–N12, N14–N16, C2–C4, C6, O1, O4, O5, P3–P5, P7, W1–W4 and R1–R3. Every row that is "checked" maps to code I read, and mutants M01–M04, M07, M08, M10, M13, M15, M16 and M20 are each killed by the cited entries. **I dispute these rows:**

| ID | Author status | RV79 status | Why |
|---|---|---|---|
| N5 | checked | partly | The terminal translation is exact (M15 is killed). But a failed verification is classified as a solve failure without the record's pass evidence, so a pass-entered escalating stop is accepted (B2d). |
| N13 | checked | partly | A candidate record's verification, v-build and verification_lme are unconstrained (B2b). A rejected outcome's reason is not range-checked when the verification completed: `verification_failed` with a completed verification is accepted (B2h). |
| C1 | checked | partly | The non-budget half is pinned. "A budget failure is not cached" is implemented at 476 but untested (M12 survives). |
| C5 | checked | partly | Groups outside every call partition are accepted (B2a). |
| C6 | checked | checked at the **wrong gate** | S1. |
| O2 | checked | partly | Typed references are strict (741). The `diagnostic_refs` list is checked only for existence. The selected-quality domain is open (N4). |
| O3 | checked | partly | Only the trigger precondition is checked (M13 is killed). A published W2's nonzero exponent is not checked (B2g). |
| P1 | checked | partly | B1a, B1b and B1d. |
| P6 | checked | partly | Implemented at 703–709, but the values-failed ⇒ separate_failure rule is untested (M09 survives). A pipeline with values completed and aliases not_entered leaves completion unchecked. |
| P8 | checked | partly | Code/phase mapping is correct, and Python is right on I63 6(a). Missing: preparation ⇒ no Run and preparation failed; native ⇒ same Run (B1c, B1e). |
| P9 | checked | partly | The check wrappers are pinned. The result-error-versus-first-failed-stage half is untested (M14 survives), and it says nothing when no pipeline stage failed (B1c). |
| P10 | checked | checked, not isolated | "A passed G5a requires coverage" is implemented (651) but not isolated by the corpus (M06, S5). |
| P11 | checked | partly | native completed ⇔ selected is checked. A case Run with a null run_ref is not (B1d). |
| N17, N11, N8, N10 | as stated | agree | Implemented. Shared bases are deferred, or reader-logic only. |

## 3. Arithmetic (independent oracle)

`rv79_oracle.py` uses only integers and `fractions.Fraction`. It computes RU64(q), the least binary64 ≥ q, from integer fields; fl(q) by correctly rounded Fraction conversion; and checks sqrt by exact rounding-interval tests. None of the reader's functions are used. The reader's results agreed bit for bit on 20,000 random cases per helper (seed 79), including subnormal, near-overflow and 2^-988-boundary operands, plus 13 targeted edges: **0 mismatches** (`oracle_run.txt`).

| Helper | Specification checked | Result |
|---|---|---|
| `upward_product` | RU64(a·b); overflow refused | 20,000 / 0 mismatches; MAX·(1+ε) refused |
| `upward_small_sum` | RU64(b0 + r + 2^-1074) | 20,000 / 0 |
| `_scaled_component` | RU64(v·2^-53), RU64(v·2^-64) | 40,000 / 0 |
| `absolute_bound` | C1:158: S=0 → 0; S < 2^-988 → RU64(RU64(2^-64 S)+RU64(2^-53\|n\|)+2^-1074); else RU64(2^-64 S) | 60,000 / 0; edges at S = 2^-988 and the next value below |
| `_phi_512` | Φ = RU64(2^-438 ê), verify.rs:365–376 | 20,000 / 0; edges 0, 2^-1074, MAX |
| `_e_hat` | verify.rs:323–334, nearest; L = 0 keeps E | 39,480 / 0 |
| `_coupled` | adaptive.rs:341–353 | 20,000 / 0 |
| `_extent` | adaptive.rs:321–339 order; correctly rounded sqrt | 20,000 / 0 |

The checked counters use unbounded Python ints. Every U is ≤ 2^53−1 at G2, all sums and partitions are compared exactly, `current ≤ SAFE` (584), and bool is rejected (C1:62). The native G5a sanity factor 1+2^-40 (PP:2636) and the 2^-59/2^-60 lower-test constants (PP:2715–2744) match lines 1123 and 1141–1142.

## 4. Coverage (I57)

Each item below was checked against I57 §§1–4 and native `final_case.rs`/PP `validate_summary_shape` (PP:2759–2842):
- **G3 roster** (1432–1435): ascending 0..n−1, equal to the attempt's source inventory. Mutant M10 is killed.
- **G5 null/complete implications and the stage, lane and binding requirements** (`_g5_coverage` 645–660) are exact to I57 §3, including "a failed certificate may carry null" and "complete never implies a certificate pass".
- **Feasibility** (1019–1029): it enumerates the 16 A-vectors, drops A[k] without a non-input row, uses L = 0 versus coupled positivity, ORs in the floor after coupling and outside it, and has D = false. Before that, the canonical layout is rebuilt from the source maps and must equal the receipt layout (996). M01 and M08 are killed.
- **Estimate and charge** (1030–1035): hats are ORed when L ≠ 0; at p512 the charge is the stop flags, otherwise the estimate. M02 is killed.
- **Rosters** (1041–1044) are compared in body-major then kind order. That is the native TrackerSet `(test, body, kind)` order (adaptive.rs:2326, 2459–2466), and stricter than the order-free count check in PP. I found no genuine receipt it rejects.
- **Record bound/theta/data_blocks and the direct data facts** (1045–1058) are as specified, including the cancelled ±x loads.
- **Unavailable attempts that keep complete coverage** (1076–1078): the same source, feasibility, record and data checks, with floor positivity rederived from the record when p = 512, and no Selection rosters.
- **Gaps:** only N3 and the M06 isolation gap (S5).

## 5. Fail-closed behaviour and API exposure

- `validate_retained_precision` raises G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED while `_IMPLEMENTATION_COMPLETE = False` (1366).
- `_validate_draft` computes `numerical_eligible = _IMPLEMENTATION_COMPLETE and …` (1470). It returns success only after G0–G8, or G0–G7 when no invocation is given, which C1:162 allows and which is never eligible.
- Flipping the flag (M19) is killed by 10 tests.
- **Exposure:** nothing outside the two test files imports the module. It is not in `core.analysis_runs.__all__`. The base `_source_contract` refuses the retained contract id (compatibility.py:240), so no public path reaches the reader.
- Every malformed top-level input I tried fails at a typed gate, except N2, which fails closed but untyped.
- I found no path that reports success while skipping a gate that applies.
- **The only fail-open behaviour is the false accepts of B1 and B2.** Eligibility is closed, but they must be fixed before the reader can be accepted.

## 6. Mutation testing

`rv79_mutants.py` applies 22 single-edit mutants. Each starts from the pristine bytes (sha256 checked), makes exactly one replacement, runs both test files and restores the file. The copy was verified as `55736ea65a` at the end. Full results are in `mutants.json`; the per-mutant logs are in WT/scratch/rv79_reader_review/mutants/.

| Killed (12) | Survived (10) |
|---|---|
| M01 floor positivity; M02 p512 charge; M03 failed-verification skip; M04 R3; M07 G5b Φ; M08 layout input-derived; M10 G3 roster; M13 W2 precondition; M15 N5 translation; M16 N10 exhaustion; M19 completeness flag; M20 R2 | M05 N17 (deferred); M06 null coverage on passed G5a (pin not isolating); M09 values ⇒ separate_failure; M11 2^-988 switch; M12 budget not cached (deferred); M14 error versus first failed stage; M17 L = 0 (deferred); M18 strict bracket (pin not isolating); M21 sanity factor; M22 extent order |

## Named questions: the known differences from Python

"Right" means consistent with C1/C2/C3/F1/S06 and the native code at NUM. Where Python is wrong, the finding is listed.

**Rust (I63 `reader_align_06d/RETURN.md`):**

| # | Difference | Python's behaviour (probe) | Contract and native | Who is right |
|---|---|---|---|---|
| 1 | Run-id contiguity and execution_order gate | G5 ATTEMPT (R1a/b/c) | C1:146 G3 "execution-order bijection"; C2:117 run.id = position | **Rust** (S1) |
| 2 | G3 member indices | old/prepared/new must be exactly 0..len at G3 | Native ids are model ordinals in order (PP:1238, 1296); the complete prelude requires that order (PP:3161–3163); F1:82, 130; C3:302 | **Python**, stricter and faithful; Rust's unique-only G3 defers the failure |
| 3 | Old coverage against the source inventory | length only, only when complete, at G5 (R3); sourceless not checked at all (R3b) | F1:101, 130: complete = exactly M, at G3 | **Rust** on the gate (S1); both miss sourceless attempts (B2f) |
| 4 | Preparation back-reference on every sourced attempt | Ready only; null accepted (R4); foreign caught only at G8 (R4b) | C3:145–148 | **Rust** (B1a) |
| 5 | Material basis against the ordinary attempt | not checked (R5) | C3:165; I57:190 | **Rust** (B1b) |
| 6a | `native` error with a selected Run | rejected at P8 (841) | S06:38 "native selected is invalid for this error" | **Python**; Rust's facade branch is wrong unless another check rejects it (RV80 to confirm) |
| 6b | `error.run_ref == run.id` | not checked | S06:38 "Non-null same Run" | **Rust** (B1e) |
| 6c | `preparation` error ⇒ no Run and preparation failed | not checked (R6c) | S06:37 | **Rust** (B1c) |
| 7 | Order inside native G5 | per-run interleaving; groups/builds after | not fixed by C1:148 or C3:294–304 | neither; ROOT convention (N1) |
| 8 | Group call exists, sources unique and listed | phantom group accepted (R8) | C2:117, 135 | **Rust** (B2a) |

**TypeScript (I64 `reader_align_06d/RETURN.md`):**

| # | Difference | Python's behaviour (probe) | Contract and native | Who is right |
|---|---|---|---|---|
| 1 | Failed verification: solve versus pass | accepts an escalating stop with pass evidence and skips two slots (T1) | adaptive.rs:4576–4611: only a solve failure escalates; a pass failure is terminal | **TypeScript** (B2d) |
| 2 | Stop-rule reason locator | accepts a foreign quantity (T2) | C1:114 exact quantity/body/kind; native reasons name layout rows | **TypeScript** (B2c) |
| 3 | Candidate record shape | accepts a verification summary on a candidate (T3) | C1:28, 105; the pass writes only verification records | **TypeScript** (B2b) |
| 4 | Ordinary extras and order | order wrong (D1–D3); listed refs only need to exist (T4a/b); selected with checks_passed accepted (T4c2); nonzero W2 exponent not checked | C3:304 order; C2:158 "nonzero b"; C1:101 | **TypeScript** on order (S2) and the exponent (B2g); the list exactness and quality domain need a ROOT ruling (N4); Python's source-hash recheck is dead code (N7) |
| 5 | Structural only | — | — | equivalent; no action |

## For ROOT to rule on

1. B1 and B2: confirm that each listed relation is contract, and have all three readers enforce it with shared mutations. RV80 and RV81 should check B1c, B1d, B2e and B2f, which neither author listed.
2. N1: the canonical order of ATTEMPT and WORK checks inside native G5, and whether native WORK is class 1 (the approved audit plan) or class 4 (an alternative reading of C3:304).
3. S4: thresholds at G0 (C1:143), or schema constants at G1.
4. N4: whether `diagnostic_refs` must be exact (each names the case, no duplicates); whether a selected case may carry checks_passed quality; whether W2's trigger must match the initial failure.
5. B2f: for sourceless attempts, complete old coverage is checkable only against the invocation (G8). Confirm that gate.

## Evidence (this folder; SHA256SUMS covers each file)

| File | Content |
|---|---|
| `REVIEW.md` | this report |
| `baseline_pytest.log` | the brief's command on the archived head: 221 passed |
| `rv79_oracle.py`, `oracle_run.txt` | independent rational oracle and its run |
| `rv79_probes.py`, `probes_final.jsonl` | 27 single- and dual-defect probes, each with RV79's contract reading |
| `rv79_probe_extra.py`, `probe_extra_run.txt` | M06 isolation, malformed top-level inputs, unsafe and integral-float U |
| `rv79_mutants.py`, `mutants.json` | 22 mutants and their outcomes |

**Rerun:** from a fresh `git archive 6b607fd01f` of READER, with both BIN variables set, run each script with `VENV/bin/python` from P. `rv79_mutants.py` takes `<log_dir> VENV/bin/python`. The probes and the oracle take no arguments. Bulk logs are in WT/scratch/rv79_reader_review/.

## Basis read (sha256)

| sha256 | File |
|---|---|
| 48af25ae74b0352bf38c7bda52725b1c3b1233ee20c583358402462b6b0d600d | R/BRIEFS/RV78_RV81_READER_REVIEW.md |
| f35e36359d6e948c33935862d7141fa6d48b066530615d14762e1db2b90d6c73 | T3/ROOT_RULINGS_V1.md (7388 to the end) |
| c8ab2318457bd3897e207888af80f54f1a889071bf630ff1815939d922a567e3 | R/I32/f2a_wire_c1/WIRE_CONTRACT.md |
| 923da0b97eb5becad7e7c1373c5a6f362568dc28ac8eab029c9170677c890869 | R/I32/f2a_wire_c2/CONTRACT_DELTA.md (§§4–5, the group and ordinary rules) |
| fd00d2c1e8f4f7e2077f304560a63830e2b7b61cb5b1d47aff994c7874ed292e | R/I52/prepared_public_contract_02/C3_DELTA.md (140–330) |
| 6b8ebea5b83c3033211df144f69add8fcb753855cae570ec66afdd6684ddbf36 | R/I52/prepared_public_contract_correction_03/ADDENDUM.md (60–140) |
| 031b2a150545df4d80da8d8078b956e98760e0df757a75f98bd39b914dec42ab | R/I52/reader_contract_seams_06/ADDENDUM.md (25–60) |
| 2e9bfb7abd8e94f097613877d643127397958a107a5eb13fd05ce3ae8242cdbe | R/I52/reader_contract_seams_g4_08/ADDENDUM.md |
| 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 | R/I57/summary_coverage_01/ADDENDUM.md (§§1–4) |
| ee3cc5918cec2a789116702fd4bdbce78dd0a243dde388b21b2b6828954a7790 | R/I62/coverage_shared_python_01/READER_AUDIT_PLAN.md |
| 06419414e214e0675110b9ffd2d4370130991d83bf79a77d9b3e9a346ca15935 | R/I62/coverage_shared_python_01/RETURN_C2_1.md |
| 1bfa35c514c302bf16b5cd00f8e34627609c01b4ab5a83dcfa8ea83affbffbea | R/I62/coverage_shared_python_01/RETURN_C2_5.md |
| d562af562ddf057b55a3e790ded7056172ff607ae0dfb87be3a0020da14fb628 | R/I62/coverage_shared_python_01/ACCOUNTING_CAUSES.md (via RETURN_C2_5 and the ruling) |
| 9ece528f917dc9741fb56a89b24b35fd3027828fe35c1321c29b8b074fa8212b | R/I63/reader_align_06d/RETURN.md |
| 272fc5fe9195e1760962b07c5164ff8e3f64e40307c1e8def432afa683f39618 | R/I64/reader_align_06d/RETURN.md |

The section of `reader_contract_seams_correction_07` I needed was read through the 08 addendum and the grep hits cited there. Not done in the time box: a line-by-line read of the remaining G8 material and units helpers beyond the cited checks, and any reading of the Rust and TypeScript source (left to RV80 and RV81).
