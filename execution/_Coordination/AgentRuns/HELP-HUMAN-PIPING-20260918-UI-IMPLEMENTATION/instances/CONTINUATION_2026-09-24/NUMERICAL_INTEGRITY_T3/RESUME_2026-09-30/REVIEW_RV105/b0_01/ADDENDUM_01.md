# RV105 ADDENDUM_01: confirmation of I78's B0 revision 01

**Reviewer:** RV105, TASK (Type 2), for ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. I made no delegation. 2026-10-06 UTC. This addendum confirms the revision against my review `REVIEW.md` (`d4807e9f…`). That review and its SHA256SUMS are unchanged.

**The candidate** (in `R/I78/b0_contract_01/`, committed on NUM `77bac7532f`):

| File | Sha256 | Check |
|---|---|---|
| `DESIGN_v2.md` | `5933b90b8c323199050caec4ce0f8bc178f4ea7a33a9808c8a72f769c62d1114` | Verified |
| `REVISION_01.md` | `79520623f889283cc85725f4c9825a67aa0c196e379214fbcfe867c4328b2983` | Verified |
| `SHA256SUMS.revision_01` | — | 2/2 OK |
| v1 `DESIGN.md` | `25a07a66…` | Unchanged (its SHA256SUMS 1/1 OK) |

**The rulings read:**
- RR "RV105 fails B0 as drafted on one local finding; I78 revises; RV101 confirms #1104's head" (RR:13367–13408);
- RR "Owner decision: ROOT may raise M up to 6.0 GiB without asking" (RR:13409ff);
- the work graph's amended owner-held list (WG:572–584).

**Basis.**
- NUM `77bac7532f`. Nothing outside `P/execution` differs from `f12fed9d69` (0 files), so every code fact in `REVIEW.md` still holds.

**Method.**
- I diffed v1 against v2 (100 `diff` hunks) and read v2 in full.
- I re-read the code behind each answer.
- I ran read-only checks with VENV and `git grep` against committed files (`evidence/addendum_01_checks.txt`).
- No cargo, vitest or native job, no Git write, and nothing in the system temp directory.

## Verdict: **CONFIRMED**

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 3 (all new, none gating) |

- **B-1 is resolved.**
- **S-1, S-2 and S-3 are resolved as ruled.**
- **N-1 to N-11 are each handled as RR assigns them.**
- **Decision 21:** AGREE.
- **Decisions 16 and 17** match the owner's decision.

I found no new defect introduced by the edits. The three notes below are wording.

## 1. B-1

| Check | Result | Evidence |
|---|---|---|
| T-4 is keyed only on the verdict | **Yes** | T-4 reads: "`not_required` exactly when `numerical_quality.cases[i].solve_quality == checks_passed`". A W2-published Passed case is `not_required`. The seed-shape text and the "same fact … readers already check" sentence are gone. `grep` of v2 for `not_triggered`, `same fact` and the seed form finds them only where RS's current rule is described (§0 item 2, §3.3) and in decision 21's own tag condition |
| RS's `not_required` alignment is specified for B1, with its pin and mutations | **Yes** | **§3.3 (G5):** RS drops `initial.kind == "report"`, `initial.outcome == "checks_passed"` and `w2.kind == "not_triggered"`. It keeps PY and TS's rule: null attempt, `initial` not `not_attempted`, verdict `checks_passed`. **§3.4 must-pass:** W-C2 case B, W2-published `not_required`, in both modes. **Three mutations:** verdict `sensitive`; `initial` `not_attempted`; a non-null `product_attempt_ref`. **Re-qualification:** one, together with D38 and F-1.<br>**I confirmed in code** that RS's own `w2` check requires an initial failure whenever `w2` is not `not_triggered`. So dropping the `w2` guard on the report-outcome equality is equivalent, as v2 says. **No existing first failure moves:** no current CORPUS mutation edits the `initial` or `w2` of a `not_required` case (`corpus_bases.log`) |
| §0 F0-2, §1.3, §1.4 and decision 1 are restated consistently | **Yes** | **§0 item 2:** case B is W2-published Passed, therefore `not_required`, and RS refuses it until aligned. **§1.3:** W-C1 and W6 change to `NoTriggeredCase`; W2 and W2b are "changes if `checks_passed`". **§1.4:** case B's row says "needs RS's `not_required` alignment". **Decision 1:** carries the DN §4.3 and RR:2190 warrant |
| The three-case W-C2 is reader-legal in all three readers under the aligned rule | **Yes, with Text B for dense case A** | **Case B** (`not_required`; `initial structural_failure/range`; `w2 published`; verdict `checks_passed`; legacy disposition `unavailable` with its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`):<ul><li>PY and TS admit it today, and RS will after the alignment;</li><li>D6c's trigger and initial-error checks pass;</li><li>G4 forbids the legacy diagnostic only on **selected** cases (RS `g4`; PY; TS), and `legacy_source` carries no status constraint;</li><li>G8 P4 needs no parity row at b ≠ 0, which the producer also emits.</li></ul>**Case A** (W2-published Sensitive, selected) passed all three readers in sparse (PROBE §4). Dense needs Text B (§3.2).<br>**Case C** (unavailable `kernel_unresolved` with a Run) is the D4d shape |
| No other passage relies on the seed shape | **None found** | §1.2–§1.4, §2, §3.3–§3.4, §4, §6, §8 and §9 all key on the verdict |

## 2. S-1, S-2 and S-3

| Finding | Result | Where in v2 |
|---|---|---|
| **S-1** | **Resolved.** Before T-4 lands, B1's probe records the verdict, `initial` and `w2` of every QUAL §4 witness input (W1–W7, W2-deep, headroom; W2 and W2b included) and of every `attempted_examples` input, plus decision 21's error tags. W2 and W2b are added to §6 step 7's re-based witnesses where needed. The `attempted_examples` row ("the milestone and the `rejected_stress_range` pair are Sensitive") agrees with the committed `rejected_stress_range/*.raw.json`, whose `solve_quality` is `sensitive` | §1.3 table and "B1's probe records first"; §6 step 7; §7 (B1 row); decision 2 |
| **S-2** | **Resolved.** T-7: a preparation-stage failure publishes `source_ref: null`, `run: null` and no `CaseSource`. This matches PY `_g5_stages` and TS `productAttempts`.<br>**It is also consistent with C2 §3 and the code.** In `prepare_owned_case`, `PrimitiveSource::new` and `check_support_source` (the map validation) are followed directly by `self.source = Some(new)` and `trace.completed(Preparation)`, with no fallible step between them. So a registered source always means preparation completed. C3a's refused record follows the same rule (§4.2) | §1.2 T-7; §2 (last S-2 bullet); §4.2; decision 4 |
| **S-3** | **Resolved.** The record point is the attempt's terminal stage: preparation for T-7, native for T-8, the last proof stage for T-9. The snapshots are "not ordered in `product_attempts[]` (start) order", and "no reader checks an order among snapshots, and nothing sums them" (C3 §3). C3a's `adapter` follows the same rule | §1.2 T-11 "Cumulative snapshots"; §4.2; decision 4 |

## 3. N-1 to N-11

| Note | Handled as RR assigns? | Where |
|---|---|---|
| N-1 | Yes: one seed per requested load case, each with `initial` set. The pitfall in `ordinary_solve_attempted` is stated | T-3 (e); §6 (G-C row); §7 |
| N-2 | Yes: decision 21, with a recommendation and ROOT as decider (see §4 below) | T-4; §1.3; decision 21 |
| N-3 | Yes: the outcome table adds T-1, `StackReservation`, `PermitUnbound` and `NoTriggeredCase`; T-11's no-call branches are removed | §1.2 outcome table; T-11 |
| N-4 | Yes: the scope (only C1:68 details or `publication_hash_range`, which matches PP `receipt_encoding_detail`), the placement (selected-at-abandonment cases, naming the invocation's cause), and the base-reader test extended to several notices | T-12; §7; decision 6 |
| N-5 | Yes: recorded as a known, fail-safe B1 limit. One citation slip (A1-N1) | T-11; §7; decision 5 |
| N-6 | Yes: (4b) requires `case.source_ref == a.source_ref`; the test-hook shape is refused by design; the pin keeps the shared group and case A's builds and removes only C-originated items; m8 is added | §2; decision 11 |
| N-7 | Yes: the L = 0 producer-solved bases replace the absent milestone bases; milestone D-U6-5 copies are optional | §3.4; decision 9 |
| N-8 | Yes: B3's three-reader G8 widening for `pressure_contract {1.0.0, legacy_pressure_v1}`, and the `reserved_inactive_successors` precedent (physics-1 left unedited, because editing it would change the inherited hash) | §5 items 1 and 7; §7; decisions 14 and 15 |
| N-9 | Yes: `NoTriggeredCase` is distinct from `LegacySeed::NotRequired` and the disposition, and rechecks use the full form. **My recheck at `77bac7532f`:**<ul><li>0 hits each: `NoTriggeredCase`, the full `openpipestress.result_semantics/0.3.0/physics-retained-1`, `exact_straight_retained_w1a_v2`, `OperandPreparation`, `operand_preparations`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `RP-PREPARED-EXACT-DUAL-v1`;</li><li>`combination_operand`: 7 lines, in the two identifiers named;</li><li>the bare form: 1 comment hit.</li></ul> | §0 item 6; T-4; §5; decisions 1, 12 and 14 |
| N-10 | Yes: the memory contingency, with \|A\| live at W3. The probable path is now ROOT's choice of M ≤ 6.0 GiB. The estimate adds 7–14 h agent and 2–3 h review; the arithmetic checks (30–45 + 7–14 = 37–59; 10–14 + 2–3 = 12–17) | §0 item 7; §6 steps 2, 5 and 6; §9; decisions 16 and 17 |
| N-11 | Yes: C1:101 and D2 §4.9.2 define `not_required`; DN §4.3 and RR:2190 warrant the trigger; C2:164 is D6b's warrant; prepared sources are per attempted case | §0 item 1; T-4; §6 step 2 |

## 4. Decision 21 (new): **AGREE**

**The rule.** A case whose seed's `initial` is `structural_failure` with tag `mechanism`, `asymmetric` or `invalid_input` (and no W2 publication) is not in A: no attempt and no notice. If A becomes empty, the result is `NoTriggeredCase`.

**Checked against DN §4.3.** DN says the trigger is "Never triggered by `Mechanism`, `Asymmetric` or `InvalidInput`". The three tags exist in SCHEMA's `StructuralError` (`invalid_input`, `range`, `asymmetric`, `numerically_unresolved`, `negative_energy`, `mechanism`). The rule leaves DN's triggers in A:
- `NegativeEnergy` (in the supported family);
- `NumericallyUnresolved` (for example `tiny_spring`, whose tag is `numerically_unresolved`);
- a Range error that W2 could not resolve.

**Checked against RR:2190.** "W1 is selected only for Sensitive and D-5-routed cases" restricts selection. Excluding these failures from the attempt is consistent with it, and so is G6 ruling 2(a) / ROUTING:98, which allows a notice only for W1 work that ran.

**Checked against the readers.** No reader is affected:
- an excluded case's failure is pushed as a `blocking` integrity diagnostic (PP `append_integrity_failure`);
- `has_blocking` then returns `blocked_envelope`;
- so the envelope is never `MECHANICS_SOLVED`, and such a case can never sit in a successor, as v2's T-11 says.

The base readers see only plain ordinary bytes, which they already accept.

**The effect.** It changes D1's output only by removing a notice on a blocked envelope:
- at c = 1, a one-case Mechanism, Asymmetric or InvalidInput input publishes exact bytes instead of bytes plus one notice;
- with c ≥ 2, the excluded case gets no notice, while the other cases in A keep theirs.

**The alternative,** keeping D1's notice, is also coherent. But it publishes a notice implying a recovery route that DN rules out.

## 5. Decisions 16 and 17: the M text matches the owner's decision

- **ROOT may select M up to 6.0 GiB.** Decision 16 and §6 step 5 say ROOT "may also select a new M ≤ 6.0 GiB (6,442,450,944 B) under D-7". The new M is chosen "by measurement and the 0.9 M margin, as a ruling with re-registration". That matches RR:13409ff ("selected the way D-7 selects … recorded as a ruling … with the re-qualification its unit requires").
- **The owner keeps the rest.** Decision 17 and §6 step 6 leave M above 6.0 GiB and any supported-machine statement owner-held, as the owner's decision and WG:578 do.
- **B0 selects nothing.** §6 step 5 says "B0 selects no M and no cap values".
- **The arithmetic checks.** 0.9 × 6,442,450,944 = 5,798,205,849 B, which is 2,202,717,115 B above dense E_mov,max + R and 2,222,427,563 B above sparse.
- **The new threshold has a home.** `REGISTERED_PROFILES[…].threshold_bytes` (today `4_026_531_840`) is a real field, so v2's "new `threshold_bytes` … part of B1's registration diff" is accurate.

## 6. Nothing new is broken

All 100 diff hunks map to a `[r01: X]` answer, the owner's M decision, or header and basis updates. In the edited passages:
- **§3.1:** condensed without loss (the conclusion stays in §0 item 3).
- **Text A:** its defect line is shortened; the substance is unchanged.
- **P4:** gains the producer-site sentence, which matches the code.
- **§4.2:** gains the S-2 and S-3 rules.
- **§5:** gains the full-form recheck and the N-8 items.
- **§7:** gains B1 rows for S-1, N-1, N-4 and N-5, and NT-1's 13th field.
- **§9:** gains the itemized estimate, which checks.

Unedited sections (T-1, T-2, T-5, T-8 to T-10, T-13, R-D38 (1)–(4a), C3a's rules, §6's cap table) are identical in substance.

## Notes (new; none gating)

| ID | Where | Note | Suggested remedy |
|---|---|---|---|
| A1-N1 | T-11 "A known B1 limit" | The R-b′ wire-representation ruling (F3, "fail-closed, held") is at **RR:9013**, not RR:9019 | Correct the citation in ROOT's selection text |
| A1-N2 | T-4 exclusion; decision 21 | **The `w2` clause is vacuous.** "and its `w2` is not `published`" can never fail: W2 is entered only on a Range or formation `NumericalRange` trigger (PP `ordinary_range_trigger`), and D6c binds the trigger to the initial error. A `mechanism`, `asymmetric` or `invalid_input` initial therefore always has `w2` `not_triggered`. **The effect is unchanged.** A Range initial whose W2 then fails stays in A, as DN's "Range error that W2 scaling could not resolve" requires | Keep or drop the clause. If kept, read it as "`w2` is `not_triggered`" |
| A1-N3 | T-4 ("for c = 1, T-4 is the only behaviour change"); §1.3; owner information | **Decision 21 is a second c = 1 change** (§4 above), and §1.3's table has no row for it. Only the probe bullet covers it. Its effect on committed tests is unknown until the probe records the error tags. For example, W2's ordinary outcome is unrecorded | If ROOT adopts decision 21, add it to the "for the owner's information" line beside decision 1's (dev/test build only; no public change), and have B1's probe report any committed test it moves |

## 7. The decisions ROOT asked about

| # | Verdict | Reason |
|---|---|---|
| 1 | **AGREE** | Keyed on the verdict, as DN §4.3 and RR:2190 require. `NoTriggeredCase` has no collisions. The owner information covers W6's PHYS-R4 geometry. B-1 is resolved |
| 2 | **AGREE** | The three-case W-C2 is reader-legal under the aligned rule, with Text B for dense A. The S-1 audit precedes T-4. The stop rule is kept |
| 4 | **AGREE** | Per-case C3 attempts; S-2 is consistent with C2 §3 and the code order; S-3 has a defined record point and no order claim |
| 5 | **AGREE** | The abandonment list is unchanged. N-5's limit is recorded and fail-safe (citation per A1-N1) |
| 6 | **AGREE** | The detail's scope matches `receipt_encoding_detail`; it is placed on selected-at-abandonment cases and names the invocation's cause |
| 9 | **AGREE** | The G8 alignment as before, plus RS's G5 `not_required` alignment. It is equivalent under D6c, and no existing first failure moves. The corpus moves to the L = 0 bases |
| 11 | **AGREE** | (4b) with `case.source_ref == a.source_ref` (TS already requires it). The test-hook shape is refused by design. The pin keeps shared builds; m8 is added |
| 14 | **AGREE** | The full-form recheck is clean at `77bac7532f`. physics-1 is left unedited, following the retained precedent, and the inherited-hash reason is sound |
| 15 | **AGREE** | `preview-physics-retained-1` (the route publishes preview-physics-1 today), with the three-reader G8 widening stated |
| 16 | **AGREE** | Scopes and method; per-attempted-case sources; \|A\| live at W3. M ≤ 6.0 GiB is ROOT's under D-7, per the owner. No value is selected |
| 21 | **AGREE** | Faithful to DN §4.3; consistent with RR:2190 and G6 ruling 2(a); invisible to the readers (blocked envelope). Note A1-N2 and A1-N3 |

## 8. For ROOT

- **Nothing blocks selection** of decisions 1–16 and 20–21 on v2.
- **If decision 21 is adopted:** add it to the owner-information line (A1-N3), and fix A1-N1's citation in the selection text.
- **Nothing owner-held needs a decision now.** Decision 17 is narrowed to M above 6.0 GiB or a supported-machine statement; decisions 18 and 19 are unchanged.

**Limits:** I ran nothing. W-C2's case C, and the verdicts and error tags of W2, W2b and K2a's deferred formation, remain for B1's probe, as v2 states.
