# RV113 (RV-R), round 1: independent review of B1's SR-RS slice (the Rust reader)

TASK (Type 2), RV113, an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path; I made no delegation. I am a fresh instance and wrote none of the change. 2026-10-07 UTC.

**Brief:** `R/BRIEFS/RV113_RVR_ROUND1.md`, sha256 `a753ea5b038762de275046b2275afa2abeb98d648977cca0d09277ba7bcf9cd4` (verified before use). I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md`, then the brief, `R/BRIEFS/B1_SR_RS.md` (`9c0bb8ac…70e2`) and `B1_COMMON.md`, PLAN_v2 §0–§2.5, §5 and §8 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…9be0`, verified), DESIGN_v2 §0, §1.2, §1.4, §2 and §3 (`R/I78/b0_contract_01/DESIGN_v2.md`, `5933b90b8c323199050caec4ce0f8bc178f4ea7a33a9808c8a72f769c62d1114`), I74's §1.5 and decision 9, RV101's NT-1 and A2-N3, RV97's R2-N-2, RV78's round-1 review (for its form), and RR's entries for I88/I90 and I90/RV113. I read I90's RETURN (`R/I90/b1_sr_rs_01/RETURN.md`, `29eb10a3…`, verified) **only after** writing my own D38 audit (`evidence/d38/`). No other role's instructions were consulted.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics`), RE (= `P/core/reporting/result_export`), RS (= `RE/src/retained_precision.rs`), T, R and VENV as in the dispatch. Code is cited by symbol.

**The candidate:** `codex/piping-t3-b1-r-20261007` at `cc81e78801` (three commits over I1 `262bd687f0`: `d7c76de43f`, `a4eab1dd01`, `cc81e78801`), 3 files, +483/−26. I reviewed my own `git archive` copies of I1 and of the head, plus a third copy of the head for mutants (`WT/rv113/{i1,head,mut}`, `projects/chirality-piping` without `execution/`); `diff -rq` of the I1 and head copies lists exactly the three files. Corpus 07m: `c21112fdbfad37dd4832c8dd64d066e1809dd70dce6c89cb920d1d6dd3d46807`, identical in both copies.

## Verdict: PASS

**Counts: 0 BLOCKING, 1 SHOULD-FIX, 5 NOTE.**

The reader does what DESIGN §2 and §3.2–§3.3 say:
- my own census over 07m finds **0 changes** in 339 entries and four verdicts each (R5 does not fire);
- (4b) is admitted exactly beside a registered source equal to the case's, with native failed and no Run, and every other shape I built is refused; (4a) is unchanged;
- G8 applies P1 to every case and P2–P4 per case; a dense b = 0 case without a parity row is admitted;
- G5's `not_required` rule is DESIGN's;
- c = 1 is byte-identical through precommit, and nothing in the layout stop fires;
- RE, PP, the runner and the pins match I1 test by test, except the six added tests.

I90's D38 list is complete. I accept the ten redundant (4b) conjuncts. The findings are test-coverage gaps (one SHOULD-FIX, which also bears on SC's 07n design) and notes.

## Findings

| ID | Severity | Where | Finding | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | RS `g5_ordinary` (the `not_required` rule); `RE/tests/retained_precision_contract.rs` `b1_g5_not_required_admits_a_w2_published_case` | **The rule's first conjunct, `product_attempt_ref` null, is pinned by no test.** My mutant M20 (that conjunct dropped) survives RE's lib and contract tests. It is not equivalent: a `not_required` case whose `product_attempt_ref` names an attempt that the case owns passes G3, and RS refuses it at G5 `ATTEMPT_MISMATCH` (`nr_product_attempt_ref_with_attempt`); M20 admits it. The candidate's test names another case's attempt, so it stops at G3 `COVERAGE`. That is also the shape ROOT's ruling 3 routes to 07n for DESIGN §3.4's "`product_attempt_ref` set non-null" mutation. As planned, 07n would therefore pin G3 and leave this conjunct unpinned in all three readers. | Add a reader-local RS test with a product attempt owned by the `not_required` case (my probe's edits: 07j's must-pass entry without its `product_attempts/1` removal, with `cases/1/product_attempt_ref` = 1; expected G5 `ATTEMPT_MISMATCH`). For SC: give 07n's entry the same form on W-C2 case B (its own attempt, appended with the next id), and fix its first failure from the three readers. |
| N-1 | NOTE | RS `g5_products`, the (4b) branch | **(4a) has no positive witness at `validate`.** My mutant M02, which routes a native failure *with* a Run into (4b) and so refuses every (4a) receipt, survives RE's tests. It is not equivalent: my two (4a) probes (F_BASE's case 1 with an idle Run refused `ledger_unavailable`, the attempt native-failed or capture-failed) are admitted at I1 and at the head, and M02 refuses both. The contract test's own D30 note says no shared base has a non-selected native Run. W-C2's case C (Run `unresolved`/ceiling) is a (4a) case, so 07n's W-C2 must-pass entries will pin it as planned. | Confirm at SC that the W-C2 must-pass entries kill M02, or add a reader-local positive (4a) test now (my `a4_*` probe shape). |
| N-2 | NOTE | DESIGN §3.3's `not_required` rule (RS, PY, TS alike) | **The ruled rule admits two non-emittable ordinary shapes**, which RS refused at I1: an initial failure (`structural_failure`/range or `formation_failure`) beside W2 `not_triggered`, or beside W2 `failed`, with the verdict `checks_passed` (`nr_structural_failure_not_triggered`, `nr_w2_failed_checks_passed`, both admitted and eligible at the head). The producer cannot emit them: a range failure without a published W2 publishes no solve and blocks the envelope (T-6, T-11). PY and TS have exactly this rule, so parity holds, and the case's reliance still comes from the published verdict and rows, which the base reader checks. This is not a defect against DESIGN. | ROOT: keep the rule as ruled (my recommendation, for parity), or tighten it in all three readers to "initial a report with W2 `not_triggered`, or W2 `published`". |
| N-3 | NOTE | RS `g8`, P4 | **P4's exact predicate ("published", not "triggered") is unpinned.** My mutant M11 (a parity row refused unless W2 is `not_triggered`) survives RE's tests; my `dz_parity_case1_w2_failed` admits a parity row beside a *failed* W2 and kills it. On producer-emittable receipts M11 is equivalent: a failed W2 has no solve, hence no parity row. | Optional: one reader-local assertion, or a 07n entry. |
| N-4 | NOTE | F-1 text B's disclosed limit (DESIGN §3.2; RS's comment) | **A precision for the limit's wording.** Deleting a dense case's parity row and resealing only the hashes is **detected** at G3 `COVERAGE` when hull-projected rows follow it, because the attempt's `projection_outcomes` row indices shift (`limit_l0_dense_parity_deleted_indices_unshifted`). The limit holds for a deletion whose reseal also rewrites those indices (`limit_l0_dense_parity_deleted`, `limit_dense_synthetic_parity_deleted`: admitted and eligible), or for a case with no projected rows after it. The limit is real and as disclosed; only "after resealing" is broader than needed. | Optional, for DESIGN or SC's text: "after resealing the receipt, including its row indices". |
| N-5 | NOTE | `RE/src/source_blocks.rs` `rv95_n5_integer_tests::every_field_read_through_integer_refuses_two_to_53` | **The per-field loop exercises `integer` alone.** It places 2^53 at each of the 13 receipt paths and calls `integer` on that value, which is the same call 13 times; the path takes no part. The field-to-call-site binding comes from the test's census of the production text (exact argument set, with `failure["block_order"]`), which is sound and is what RV101 NT-1 asked for. The doc's "each refuses 2^53 as read" can read as more than this. | Optional wording: "each, read through `integer`, refuses 2^53; the census binds the fields to the call sites". |

## 1. The cascade census over 07m (R5)

**My harness** (`evidence/harness/rv113_census.rs`) is copied into my archives only. It is written from the snapshot format rules (SHARED_SNAPSHOT_06C `format_change`, SHARED_SNAPSHOT_07E `format_rule`), not from the contract test's helpers:
- edits use the strict index rule;
- `invocation_edits` rebind the invocation digest;
- `rehash: "all"` follows the rule's order: preparation, selected source identities, publication, receipt;
- `after_rehash` edits are applied last.

For each entry it records the input's sha256 and four verdicts:
- `validate` with the invocation: the gate and code, or the eligibility with the classification count and a digest of the full `Validation.classifications`;
- `validate` without the invocation;
- `validate_transport_metadata`;
- `semantic_contract::numerical_use_standing_with_context`.

**Runs:** I1 and the head, each in its own archive and target, inside RE's suite (jobs `re_i1`, `re_head`).

**Result: 0 changes in 339 entries** (17 bases, 294 mutations, 28 must-pass):
- the 339 materialized inputs are byte-identical between the two runs;
- all four verdicts are identical entry by entry, including the classification digests of the 45 admitted statements;
- on both sides my harness reproduces the corpus's own Rust expectations: all 294 first failures (`expected_by_reader.rust`, else `expected`); all 28 must-pass eligibilities and standings (the corpus's `eligible` is the API's `numerically_eligible`); all 17 bases' eligibility.

Evidence: `evidence/census/CENSUS_07M.json` (the comparison), `census_table.tsv` (one line per entry), and `census_i1.jsonl` and `census_head.jsonl` (the raw verdicts). **R5 does not fire.** This agrees with I90's census, which used the contract test's printed outcomes.

None of my 22 mutants changes any 07m verdict either (§8): 07m does not exercise B1's rules, as I90 and I84 said; 07n will.

## 2. R-D38 (4b)

### 2.1 Admitted where DESIGN §2 admits it; m1–m8 and the rest refused

**My own (4b) derivation** (`evidence/harness/gen_probes.py`, probe `d38_base`) is on `two_case_two_groups_synthetic`. This is a different base from I90's: there case 1 has its **own group and three Builds**, so DESIGN's derivation runs in full:
- case 1 (selected) becomes unavailable: cause `prepared_product_failure` naming attempt 1, reason (`source_unavailable`, `preparation`), its source kept, its Run removed;
- its group (group 1) and the Builds its Run originated (3–5) are removed;
- the call keeps case 0's position only, and `charged` and the call's after-value are recomputed (17);
- attempt 1 has `run_ref` null, proof null, stages done(1, [failed]), and result `capture` with `CaptureError::Origin(capacity)`;
- the selected case's artefacts become an unavailable case's: `RETAINED_PRECISION_UNAVAILABLE`, rows without `recovery_method`, and no selection, method or identity digest.

I also derived the shape on I90's base independently (`f38_base`).

**Results:** 82 probes, each with my expectation written at both I1 and the head. **0 mismatches.** `evidence/probes/PROBE_TABLE.json`; raw verdicts are in `probes_i1.jsonl` and `probes_head.jsonl`.

| Probe | Head | I1 |
|---|---|---|
| `d38_base`, `f38_base`: (4b) beside a selected case | admitted, not eligible; standing `needs_recompute` | G5 PRODUCT_ATTEMPT |
| m1: error `{native, run_ref: 1}` | G5 PRODUCT_ATTEMPT | same |
| m2: native completed; m3: `run_ref` with no Run; m5: `proof_start` completed; m6: the attempt's source null | G5 PRODUCT_ATTEMPT | same |
| m4: `execution_order` lists the case | G3 COVERAGE | same |
| m7 (five forms): the source in the call's `source_refs`; in group 0's; its own group kept; the call's original two positions; position 0 naming source 1 | G5 ATTEMPT | same |
| m8: the case's source 0; the case's source null | G5 PRODUCT_ATTEMPT | same |
| the removed Run's Builds kept; `charged` or the call's after-value not recomputed | G5 WORK | same |
| the case's Run kept | G3 COVERAGE | same |
| cause `CaptureError::accounting{event}` | G5 WORK (R1′, as for any attempt) | G5 PRODUCT_ATTEMPT |
| cause `native_unavailable` | admitted, not eligible | G5 PRODUCT_ATTEMPT |
| preparation failed; the test-hook shape (no source anywhere, DESIGN §2); the cause naming attempt 0; reason code `kernel_unresolved`; phase `kernel`; cause `receipt_failure`; proof kept; observables/G5a or values entered; result ready; attempt and case on case 0's source; native `not_entered` with a capture error | G5 PRODUCT_ATTEMPT | same |
| `RETAINED_PRECISION_SELECTED` kept | G4 DIAGNOSTIC | same |
| `recovery_method` kept on the case's rows | G6 ROW_METHOD | G5 PRODUCT_ATTEMPT |

So RS admits (4b) exactly when native failed with no Run, beside a registered prepared source equal to the case's. Every other shape I built is refused. The relaxation changes only verdicts that were G5 PRODUCT_ATTEMPT at the old D38 check. The m-forms' first failures agree with I90's table and with ROOT's ruling 3 (m4 at G3, m7 at G5 ATTEMPT, m1 with the whole error value).

**(4a) is unchanged.** F_BASE's case 1 Run was made idle in its ready group (refused `ledger_unavailable`), with the attempt native-failed or capture-failed. Both are admitted, not eligible, at I1 and at the head. The same receipt with (4b)'s reason is refused at both. See N-1.

### 2.2 D38's audit: my list against I90's 18

I wrote my list before reading I90's RETURN (`evidence/d38/D38_AUDIT_RV113_BEFORE_RETURN.md`). The two lists agree on every check that assumes a prepared source has a Call or a Run:

| Mine | I90 # | Check | Disposition (agreed) |
|---|---|---|---|
| A1 | 1 | native entered ⇒ `run_ref` | relaxed to (4b) |
| A2, A3 | 2 | the source equality bound only through a Run; `run_ref` null ⇒ the case's Run null | (4b) adds the equality (m8); A3 is (4b)'s first conjunct |
| A4 | 4 | native completed ⇔ Run selected | holds (both false) |
| A5 | 3 | proof ⇒ `run_ref` and a selected Run | does not apply |
| A6 | 5 | coverage ⇒ own Run (`g5_coverage`) | does not apply |
| A7 | 6 | `reason_table` | already admits a capture without a Run |
| A8 | 8 | `g5_native`: Call positions, groups, C5, Builds, meter | no source needs a Call; these enforce (4b)'s last clause (m7 and the WORK probes) |
| A9 | 9 | `g3`: Run ids and `execution_order` | enforces the `execution_order` clause (m4) |
| A10 | 13 | `g5a` on an unavailable case's coverage | does not apply |
| A11 | 14 | the selected-only checks | do not apply |
| — | 7, 10–12, 15–18 | `error_stages`; `g3`'s member and body checks; D29; `g1`; `g8`'s source and attempt loops; O5; eligibility | no Call or Run assumption |

**I90's list is complete.** I add two items that touch a Run-less prepared attempt without assuming a Call or a Run; they are not omissions:
- `g5_stages`: preparation completed ⇔ a source. This is what makes (4b)'s non-null-source conjunct redundant.
- `accounting_rules` R3′: a fault-bearing cause's owner defaults to the proof, so with proof null a fault-bearing capture cause is refused at G5 WORK. (4b)'s cause is a native-stage capture failure (`Origin`, which has no fault), so this does not bear on (4b).

### 2.3 The ten redundant (4b) conjuncts: accepted

Each of the ten repeats a later G5 PRODUCT_ATTEMPT check in the same attempt's iteration: the proof-null rule, `g5_stages`, D19, D4c, `reason_table` or `error_stages`. So the first failure is unchanged, by argument and by I90's ten survivors. My M05 replaces the whole predicate by the source equality alone, I90's "minimal form". It survives every test, my 82 probes and the census, so the ten are redundant **jointly**, not only one at a time.

I see no reason for the minimal form:
- the ten state (4b) at the one place the relaxation is granted, as DESIGN §2 writes it;
- the checks that would otherwise carry them were written for other rules (P2/P6, D19, D4c, D4d, D37), so an unrelated later edit to one of them would widen (4b) silently;
- the cost is a constant scan of eight stage names on a branch that only a Run-less native failure reaches, plus ten equivalent mutants.

ROOT's ruling 2 stands.

## 3. G8 per case: F-1 text B, P1–P4

- **P1 for every case.** On F_BASE's unavailable case, each of the following is refused at G8 `PREPARATION` at the head and was admitted (not eligible) at I1: mode code 2 in sparse; mode code 3; two mode rows; the mode row deleted, with the case's projection indices shifted as a faithful reseal would. The same holds on a (4b) case.
- **Dense b = 0 without a parity row is admitted.** I made the two-group base dense, with both cases selected.
  - No parity rows: admitted and eligible at the head; G8 at I1.
  - One on case 0 only: the same.
  - One on each case: admitted at both.
- **P2–P4 per case.** Each of these is refused at G8:
  - two parity rows on case 1 (P2);
  - a parity row on a W2-published case 1, or on a W2-published case 0 (P4);
  - on F_BASE's dense unavailable case, two rows (P2), or one row beside W2 published (P4).

  One parity row on a dense b = 0 **unavailable** case is admitted, since text B allows at most one on any case.
- **A parity row on a non-selected case** is refused with the right first failure:
  - in `sparse_interactive`, G8 `PREPARATION` (P3), on an unavailable case and on a (4b) case;
  - carrying `recovery_method`, G6 `ROW_METHOD` first (as ROOT's ruling 3 notes);
  - beside W2 published, G8 (P4);
  - at dense b = 0, one row is admitted, as text B says.
- **P4 is exactly `published`.** A parity row beside a *failed* W2 is admitted (N-3).
- **The disclosed limit holds, and P5 is not implemented.**
  - Deleting the parity row of the producer-solved `u8_l0_isolated_node_dense_scrutiny`, or of `ordinary_prepared_dense_synthetic`, and resealing is admitted and eligible at the head (G8 at I1). That is the disclosed limit, which RS's comment states. For its wording, see N-4.
  - A W2-published dense case whose mode row still states the observation fields (`original_profile_entries=…`) is admitted (`dz_no_parity_case1_w2_published`). P5 is deferred (decision 10).
- **Order.** RS runs the per-case loop before the material-basis loop. Every `INVOCATION_MISMATCH` in `g8` precedes the loop, and every check after it is G8 `PREPARATION`, so the first failure's gate and code are DESIGN's in either order (I90 §3.2).
- **The scope of P1 is safe for producer receipts.** PP emits the mode row for every case with a linear solve. A case without one (a structural failure with no W2 publication) blocks the envelope (T-6, T-11), so it never reaches a successor. Mode code 3 needs `dense_fallback_message`, which PP never sets.

## 4. G5's `not_required` rule

The rule is now `product_attempt_ref` null and verdict `checks_passed`, with the existing `initial != not_attempted` check above it, as DESIGN §3.3 writes it. PY's `_g5_ordinary` and TS's `ordinaryAttempts` read the same at I1 (I read both), so RS's comment is accurate.
- **W2-published `not_required` cases are admitted and eligible,** with evaluation and formation triggers, alone and in a dense statement without a parity row. I1 refused them at G5 `ATTEMPT`.
- **Still refused at G5 `ATTEMPT`:**
  - a report whose outcome differs from the verdict;
  - `initial` `not_attempted`;
  - the verdicts `sensitive` and `not_assessed`;
  - a W2-published case with the verdict `sensitive`;
  - a report beside a published W2 (D6c).
- **A non-null `product_attempt_ref`** is refused at G5 `ATTEMPT` when the attempt exists and names the case, and at G3 when it names another case's attempt (S-1).
- **The report-outcome equality's W2 guard** is equivalent under D6c (M22 survives everything; a W2 other than `not_triggered` requires a non-report initial, and both checks are G5 `ATTEMPT`).
- **Admitted now, refused at I1:** an initial failure with W2 `not_triggered` or `failed`, beside `checks_passed` (N-2).

## 5. c = 1 byte identity through precommit

PP compiles RS for precommit. I ran PP's whole suite at I1 and at the head (jobs `pp_i1`, `pp_head`), with the pins' own output variables set (`I61_U1_OUT`, `I61_U3_OUT`, `I61_U3G2_OUT`, `I68_U8_OUT`).
- **The pins pass on both sides:**
  - `u1_milestone_successor_both_modes`;
  - `u3_permitted_path_publishes_the_pinned_successor`, `u3_r1_carrier_names_exactly_one_publication`, `u3_r2_notice_bytes_are_pinned`;
  - `u3g2_direct_entry_publishes_the_pinned_successor`, `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
  - `u8_l0_isolated_node_publishes_pinned_successor`, `u8_d_u6_5_l0_fixtures_are_the_live_successors`.
- **The 14 documents the pin tests wrote are byte-identical at I1 and the head** (`evidence/pins/PINS.txt`):
  - the milestone successor in both modes, from U1's serializer, from U3's private driver and from U3G2's Direct entry (the last two through precommit);
  - the L = 0 successors in both modes (Direct, through precommit);
  - U1's translation corpus and five U1G2 receipts.
- **The successors equal the committed fixtures:**

| Document | sha256 (I1 = head = fixture) |
|---|---|
| milestone, `sparse_interactive` (U1, U3, U3G2) | `ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc` |
| milestone, `dense_scrutiny` (U1, U3, U3G2) | `6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5` |
| L = 0, `sparse_interactive` (U8) | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` |
| L = 0, `dense_scrutiny` (U8) | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` |

These are I90's and I85's hashes. **c = 1 byte identity holds; no stop.**

## 6. The layout stop

My checks on the archive copies are in `evidence/static/layout_and_n5_static.txt`.
- **File hashes, I1 → head:**
  - RS: `4722b505…3f93b69` → `c6965da0…780d0f5`;
  - `RE/src/source_blocks.rs`: `a9ee998a…3c96e534` → `e388416b…0a76a13e`.

  Both are as I90 states.
- **The four definitions are byte-identical at I1 and the head:** `ValidationError`, `AccuracyClass`, `RowClassification` and `Validation`, with their derives.
- **No visibility change:** every `pub`/`pub(…)` line of RS (35) and of `source_blocks.rs` (8) is identical at I1 and the head.
  - The one new RS item, `d38_capture_before_run`, is private.
  - N-5's module reaches the private `integer`, `source_plan` and `summary` as a child module (`use super::*`).
- **PP's binding tests pass on both sides:** `bindings_need_witnesses_inputs_and_reader_layouts` and `reviewed_inputs_bind_the_lock_and_the_reader_statics`.

**No stop.**

## 7. N-5's direct test

- **Wholly `#[cfg(test)]`.** The head's `source_blocks.rs` is I1's bytes followed by 6,381 appended bytes: a doc comment, then `#[cfg(test)] mod rv95_n5_integer_tests { … }`. No production line changed.
- **What it pins.** I74 decision 9 asks for a direct unit test that kills RV95's S1 and rides with PR-B1. RV101 NT-1 and A2-N3 ask for `failure.block_order` to be included beside I76's fields, which makes 13 receipt fields, not 12.
  - `integer_admits_two_to_53_minus_1_and_refuses_two_to_53` kills S1 (bound removed, M30) and the off-by-one (M31).
  - `every_field_read_through_integer_refuses_two_to_53` takes a census of the production text's `integer(&…)` arguments and asserts exact set equality, so a call site removed or added fails it.
    - My own extraction agrees: 21 calls with 15 distinct arguments, namely the 14 receipt-field expressions (13 fields, since `dof_count` is read twice) and `summary[key]`.
    - `failure["block_order"]` is in the set, and `/failure/block_order` is among the 13 fields.
  - Everything after the file's first `#[cfg(test)]` is test modules (`norm_tests`, `stress_range_tests`, `rv95_n5_integer_tests`), so the census's "production" slice is the whole production text.
  - For the per-field loop, see N-5.

## 8. Mutants

**The method.** I used a mutant schema in my third copy (`evidence/mutants/mutant_schema_*.diff`). Each mutant is a source edit guarded by `mx("<id>")`, true only when `RV113_MUT` names it, so one build serves all of them. The control (`RV113_MUT` unset) passes RE's whole suite and gives the head's census and probe verdicts, byte for byte. For each mutant I ran these already-built binaries:
- RE's lib unit tests;
- the contract test;
- the public `source_blocks` test (for the N-5 mutants);
- my probes;
- the 07m census.

**"Killed"** below means a failing assertion in the **candidate's** tests. Raw results: `MUTANT_RESULTS.json` and `run_logs.txt`.

| ID | Mutant | Candidate's tests | My probes | Disposition |
|---|---|---|---|---|
| M01 | (4b) branch without `native == failed` | survives | none change | **equivalent**: a native-`completed` attempt with no Run is refused earlier (completed ⇒ selected Run) |
| M02 | (4b) branch's "and" made "or" (a Run-bearing native failure routed to (4b)) | survives | `a4_*` (2) | **not equivalent** (N-1) |
| M03 | the relaxed check restored (I90's D38-1) | killed: `b1_d38_…` | 7 | killed |
| M04 | (4b)'s predicate → `true` | killed: `b1_d38_…` (m8) | 2 | killed |
| M05 | (4b)'s predicate → the source equality alone | survives | none | **equivalent** (§2.3) |
| M06 | (4b)'s equality → `case.source_ref` non-null | killed: `b1_d38_…` | 1 | killed |
| M10 | P4 reads case 0's ordinary attempt | killed: `b1_g8_parity_…` | 3 | killed |
| M11 | P4 stricter: parity only beside W2 `not_triggered` | survives | `dz_parity_case1_w2_failed` | equivalent on emittable receipts (N-3) |
| M12 | P4 for selected cases only | killed: `b1_g8_parity_…` | 2 | killed |
| M13 | P2 → at most two | killed: `b1_g8_parity_…` | 2 | killed |
| M14 | P1 for selected cases only | killed: `b1_g8_mode_row_…` | 5 | killed |
| M15 | P2–P4 for selected cases only | killed: `b1_g8_parity_…` | 5 | killed |
| M16 | P2–P4 count case 0's rows (rows mis-indexed in the loop) | killed: `b1_g8_parity_…` | 6 | killed |
| M18 | the old "exactly one iff dense" kept for selected cases | killed: `b1_g8_parity_…` | 7 | killed |
| M20 | `not_required`: `product_attempt_ref` null dropped | survives | `nr_product_attempt_ref_with_attempt` | **not equivalent** (S-1) |
| M21 | `not_required`: verdict `checks_passed` dropped | killed: `b1_g5_…` | 1 | killed |
| M22 | report-outcome equality: its W2 guard dropped | survives | none | **equivalent** under D6c (§4) |
| M23 | report-outcome equality dropped | killed: `b1_g5_…` | 1 | killed |
| M24 | `not_required`: a W2-published case refused | killed: `b1_g5_…`, `b1_g8_parity_…` | 4 | killed |
| M25 | the `initial != not_attempted` check skipped for `not_required` (now that rule's only guard against `not_attempted`) | killed: `b1_g5_…` | 1 | killed |
| M30 | `integer`'s bound removed (S1) | killed: both N-5 tests | — | killed |
| M31 | `integer`'s bound off by one | killed: both N-5 tests and the public-API test's control | — | killed |

**Totals: 22 mutants.**
- 16 are killed by the candidate's assertions.
- 3 are equivalent (M01, M05, M22).
- 1 is equivalent on producer-emittable receipts and pinned only by my probe (M11, N-3).
- 2 are not equivalent and pinned only by my probes: M20 (S-1) and M02 (N-1).

**Replication of I90's 25.** I replicated I90's D38-1 (M03), S1 (M30) and S1b (M31), and I90's G8 and G5 families in substance (M10–M18, M21, M23–M25); all are killed, as I90 reports. I90's ten redundant-conjunct survivors are covered jointly by M05.

**The corpus.** No mutant changes a 07m verdict.

## 9. Suites against I1, test by test

Every suite ran through `WT/tools/t3_cargo.sh` with `test --locked --offline --no-fail-fast`, in my own archives and fresh targets (`WT/targets/rv113-{i1,head}`); PP and the runner also had `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`. `evidence/suites/SUITES.json` holds the per-test outcomes, and the logs are beside it with paths reduced to placeholders.

| Suite | I1 | Head | Per-test difference |
|---|---|---|---|
| RE (all targets) | 182 ok | 188 ok | **+6 new (ok):** the four `b1_*` contract tests and the two N-5 tests. Nothing else changed. (Both counts include my two harness tests; without them, 180 → 186, as I90 reports) |
| PP (all targets) | 712 ok, 1 failed, 11 ignored | the same | none. The one failure is `s11g_tests::t13_committed_fallback_uz_is_byte_identical` (the known Mac `t13`), with the same panic text on both sides |
| runner/headless | 85 ok, 2 failed | the same | none. The failures are the known `load_reference` pair (`load_reference_one_actual_solve_…`, `cli_load_reference_one_both_modes_…`), with the same panic text on both sides |
| pins | ok | ok | none; byte-identical outputs (§5) |
| source-text guards that read RE | ok | ok | RE's carrier test (17 of 17), PP's `s11f_site_test` and `retained_precision_admission`, and the two PP law tests (§6) |

## Host and limits

- **Cargo.** Every cargo ran through `WT/tools/t3_cargo.sh` (`--locked --offline`): seven jobs (`re_i1`, `re_head`, `re_mut_control`, `pp_i1`, `pp_head`, `runner_i1`, `runner_head`). They are listed in `evidence/host/cargo_jobs_rv113.log`, extracted from `WT/guard/cargo_jobs.log`. Each had one wait (its own background job), which ended when the job's process ended. None of my waits remain. I killed no job.
- **The mutant and probe runs** executed already-built test binaries directly, at nice 10 with `RUST_TEST_THREADS=2`. They used no cargo and were light, so they ran outside the lock. The census binary also ran directly for probe reruns.
- **A recorded slip, repaired.** On this case-insensitive filesystem, one comparison output (`PROBES_V3.json`) overwrote my probe input (`probes_v3.json`) while the mutant batch was starting.
  - I regenerated the input from `gen_probes.py` (deterministic; sha256 `d30b8383ad157f59016cf149076c2932b2f34c8228d746a26f542da10c1a855d`) and reran I1's probes; the output was byte-identical to the first run.
  - I reran the probes for the two runs that had read the bad file (the control and M01).
  - The control's probe verdicts equal the head's.
  - The comparison output is now named `PROBE_TABLE.json`. The 7 MB probe file is not copied here; it is regenerated from the generator and 07m.
- **Copies and targets.**
  - `WT/rv113/{i1,head,mut}` and `WT/targets/rv113-{i1,head,mut}` were deleted after the runs.
  - My scratch, `WT/scratch/rv113_rvr_01/` (tools, census, probes, mutant runs, logs and pin outputs), is kept for SR-PY, SR-TS and SC.
  - `TMPDIR` was in scratch, and nothing went to the system temp directory.
- **Not done, and not allowed:** no DEC-025, no installs, no Git writes (Git reads used `GIT_OPTIONAL_LOCKS=0`; `git archive` only), and no record folder named `build`. Every write used an absolute path. This folder carries placeholder paths only.
- **Not run:** PY and TS (SR-PY's and SR-TS's lanes); the src-tauri suite and the full 40-manifest suite (ROOT's).

## For ROOT

1. **R5 does not fire.** My independent census agrees with I90's: 0 changes.
2. **S-1 (SHOULD-FIX)** needs a ruling on where it closes:
   - a reader-local RS test in an SR-RS repair round, which RV113 would confirm;
   - and SC's 07n entry built with W-C2 case B's **own** product attempt. As ruling 3 now routes it, the shared mutation stops at G3 and pins nothing of the G5 rule.
3. **N-1:** confirm that 07n's W-C2 must-pass entries (case C with its ceiling Run) are the (4a) pin, which RV113 checks at SC; or ask for a reader-local (4a) test now.
4. **N-2 (contract):** keep DESIGN §3.3's `not_required` rule (my recommendation, for three-reader parity), or tighten it in all three readers.
5. **The ten (4b) conjuncts:** I accept them; ruling 2 stands.
6. N-3 to N-5 are optional.

## For the next rounds (SR-PY, SR-TS, SC)

My scratch keeps:
- the harness `tools/rv113_census.rs`, the generator `tools/gen_probes.py` and the comparator `tools/compare.py`;
- the mutant tools `tools/make_mutants.py`, `run_mutants.sh`, `run_mutant_probes.sh` and `mutant_table.py`;
- the raw verdicts.

The 82 probes are written in the shared entry format (base, edits, `invocation_edits`, `rehash: "all"`) with RS's verdicts at I1 and the head. They can be fed unchanged to PY's and TS's harnesses for first-failure parity, and the (4b), (4a), F-1 and `not_required` shapes can seed my check of 07n.
