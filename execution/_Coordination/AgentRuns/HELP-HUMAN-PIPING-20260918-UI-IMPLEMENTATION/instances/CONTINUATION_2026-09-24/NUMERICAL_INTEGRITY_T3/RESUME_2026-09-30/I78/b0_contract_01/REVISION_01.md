# I78: B0 revision 01, the answer to RV105

TASK (Type 2), I78, for ROOT. 2026-10-06 UTC. Documents only: no source edits, no Git writes, no runs, nothing in the system temp directory.

**What this answers:**
- RV105's review `R/REVIEW_RV105/b0_01/REVIEW.md` (sha256 `d4807e9f16f3777b5e5616c71c3b743b77b635268e577eb3d7dd63aa47c2b008`, verified): FAIL as drafted, with 1 BLOCKING, 3 SHOULD-FIX and 11 NOTE;
- ROOT's ruling, RR "RV105 fails B0 as drafted on one local finding; I78 revises; RV101 confirms #1104's head", which accepts B-1 and S-1 to S-3 and assigns the notes.

**The revision:** `DESIGN_v2.md`, complete and self-contained. `DESIGN.md` (`25a07a66…`) and its `SHA256SUMS` are unchanged. In v2, `[r01: X]` marks the passage that answers finding X.

**Basis:** NUM `4e114af645`.
- The files the design cites (PP, RE `src`, FK, PY, RS, TS, SCHEMA, CORPUS) are byte-identical to revision 1's basis `ecb541d63f`.
- While I revised, NUM gained one records-only commit: the owner's decision that ROOT may raise M up to 6.0 GiB without asking. v2 applies it to §0 item 7, §6 steps 5–6, §8 decisions 16–17 and §9. It is not an RV105 finding; it is listed below because it changes two decisions.

## 1. Each finding, and where v2 answers it

| Finding | Sev. | What v2 does | Where in v2 |
|---|---|---|---|
| **B-1** | BLOCKING | T-4 is keyed on the published verdict: `not_required` ⇔ `numerical_quality.cases[i].solve_quality == checks_passed`. The seed-shape text and the "same fact … the readers already check the two agree" sentence are struck. A W2-published Passed case is `not_required`. RS's `not_required` rule is aligned to PY and TS (drop the `initial`/`outcome`/`w2` conjuncts; keep each reader's report-outcome equality, which is equivalent under D6c). It is pinned by W-C2 case B (producer-solved, W2-published `not_required`) as a must-pass, with three mutations, and re-qualified once with D38 and F-1. §0 F0-1 and F0-2, §1.3, §1.4 and decision 1's rationale are restated on this keying. The three-case W-C2 still stands: A selected (W2-published Sensitive), B `not_required` (W2-published Passed), C to be established by the probe | §0 items 1, 2, 4; §1.2 T-4; §1.3; §1.4; §3.3 (G5); §3.4; §8 decisions 1, 2, 9 |
| **S-1** | SHOULD-FIX | B1's probe records, before T-4 lands, the published verdict and the seed's `initial` and `w2` of every QUAL §4 witness input (W1–W7, W2-deep, headroom; W2 and W2b included) and every `attempted_examples` input, and re-bases any that become `not_required`. W2 and W2b are added to §6 step 7's re-based witnesses where they apply | §1.3 (table and "B1's probe records first"); §6 step 7; §7 (B1 row); §8 decision 2 |
| **S-2** | SHOULD-FIX | T-7: a preparation-stage failure publishes `source_ref: null` and emits no `CaseSource`. This matches PY `_g5_stages`, TS `productAttempts` and the producer's existing fail-closed behaviour. C3a's refused record follows the same rule | §1.2 T-7; §2 (S-2 reading, last bullet); §4.2; §8 decision 4 |
| **S-3** | SHOULD-FIX | T-11 defines the record point as the attempt's terminal stage, drops the monotonicity claim in start order, and says no reader checks an order among snapshots. C3a's `adapter` follows the same rule | §1.2 T-11 ("Cumulative snapshots"); §4.2; §8 decision 4 |
| **N-1** | NOTE | T-3 (e)'s attempt fact counts requested cases: exactly one seed per requested load case, each with `initial` set. Today's `ordinary_solve_attempted` misses seedless later cases | §1.2 T-3 (e); §6 cap table (G-C row); §7 (B1 row) |
| **N-2** | NOTE | A new decision 21. **Recommendation: apply DN §4.3's exclusion.** A case whose seed is `structural_failure` with tag `mechanism`, `asymmetric` or `invalid_input`, and no W2 publication, is not in A: no attempt, no notice. If A becomes empty, the result is `NoTriggeredCase`. The alternative is to keep D1's notice. ROOT rules | §1.2 T-4; §1.3 (probe records the tags); §8 decision 21 |
| **N-3** | NOTE | The outcome table lists T-1 refusal, `StackReservation`, `PermitUnbound` (unreachable) and `NoTriggeredCase`. T-11's impossible no-call branches are removed | §1.2 outcome table; T-11 |
| **N-4** | NOTE | The detail's scope (only a serializer failure whose check is a C1:68 detail or `publication_hash_range`) and its placement (on cases selected at abandonment; it names the invocation's cause) are stated. B1 extends `u3_r2_base_readers_accept_the_unavailable_notice` to several notices | §1.2 T-12; §7 (B1 row); §8 decision 6 |
| **N-5** | NOTE | R-b′'s `recovery_demoted` on any case abandons the successor (`ordinary_value` refuses `Untranslated`). This is recorded as a known B1 limit, fail-safe, with the wire member left to a later contract question | §1.2 T-11 ("A known B1 limit"); §7 (B1 row); §8 decision 5 |
| **N-6** | NOTE | Three statements are added: (4b) requires `case.source_ref == a.source_ref`; the test-hook D38 shape (source withdrawn, `source_ref: null`) stays refused by design; the pin's derivation keeps the shared group and case A's builds, and removes only case C's Run, execution entry, C-originated builds and source references. Mutation m8 is added | §2 (4b), the "test-hook" paragraph and the pin; §8 decision 11 |
| **N-7** | NOTE | §3.4 uses CORPUS's producer-solved L = 0 bases (dense L = 0 has a parity row) instead of milestone bases, which CORPUS lacks. Milestone D-U6-5 copies are optional | §3.4; §8 decision 9 |
| **N-8** | NOTE | B3's inheritance states two things. The zero-pressure `legacy_pressure_v1` route needs a three-reader G8 widening for `pressure_contract {1.0.0, legacy_pressure_v1}`, not only D1.3. And on the `reserved_inactive_successors` precedent (physics-1 and load-reference-1 list some successors; the retained precedent left preview-physics-1's list unchanged), the recommendation is not to edit physics-1's table | §5 items 1 and 7; §7 (B3 row); §8 decisions 14 and 15 |
| **N-9** | NOTE | The empty-A fallback is named `NoTriggeredCase` (0 hits at `b75069e6de`), distinct from `LegacySeed::NotRequired`. Reservation rechecks use the full `openpipestress.result_semantics/0.3.0/physics-retained-1` form; the bare form has one comment hit in T6S's `outputPolicy.ts` | §0 item 6; §1.2 T-4; §5 (the reservation); §8 decisions 1, 14 |
| **N-10** | NOTE | §6 and §9 carry the memory contingency: |A| Runs and frozen candidates are live at W3, and c ≥ 2 is unlikely to fit at today's M. With the owner's decision of 2026-10-06, the probable path is a ROOT selection of M ≤ 6.0 GiB under D-7. The owner is reached only above 6.0 GiB. B1's estimate adds B-1's RS change, S-1's audit and the M selection record: about +7–14 h agent and +2–3 h review | §0 item 7; §6 steps 2, 5, 6; §9; §8 decisions 16 and 17 |
| **N-11** | NOTE | The citations are corrected. C1:101 and D2 §4.9.2 define `not_required`; DN §4.3 and RR:2190 warrant the trigger; C2:164 is D6b's quality-binding warrant. §6 step 2 classifies prepared case sources as per attempted case | §0 item 1; §1.2 T-4; §6 step 2 |

## 2. The decisions that changed (DESIGN_v2 §8)

| # | Change | Driver |
|---|---|---|
| 1 | Re-keyed to the published verdict; `NoTriggeredCase`; the rationale restated; W6's PHYS-R4 geometry named for the owner's information | B-1, N-9, ROOT's ruling |
| 2 | Conditional on RS's `not_required` alignment; the verdict audit of every QUAL §4 witness and every `attempted_examples` input added | B-1, S-1 |
| 4 | Preparation failure publishes `source_ref: null` and no `CaseSource`; the snapshot record point is the terminal stage, with no order | S-2, S-3 |
| 5 | R-b′'s whole-invocation abandonment recorded as a known limit | N-5 |
| 6 | The receipt-encoding detail's scope and placement stated | N-4 |
| 9 | RS's `not_required` (G5) alignment added to the three-reader change; corpus on the L = 0 bases | B-1, N-7 |
| 11 | `case.source_ref`, the refused test-hook shape, and keeping shared builds in the pin | N-6 |
| 14 | Full-form recheck; physics-1's `reserved_inactive_successors` left unchanged, following the retained precedent | N-8, N-9 |
| 15 | B3's zero-pressure route also needs a three-reader G8 widening | N-8 |
| 16 | Per-attempted-case sources, |A| live at W3, and M ≤ 6.0 GiB as a ROOT lever under D-7 | N-10, N-11, the owner's decision of 2026-10-06 |
| 17 | Owner-held now only above 6.0 GiB, or for a supported-machine statement | The owner's decision of 2026-10-06 |
| **21 (new)** | DN §4.3's exclusion for Mechanism, Asymmetric and InvalidInput. Recommendation: apply it, so these get no attempt and no notice | N-2 |

**Unchanged:** decisions 3, 7, 8, 10, 12, 13, 18, 19 and 20.

## 3. For ROOT to rule on

1. **Decision 21 (new):** apply DN §4.3's exclusion (recommended) or keep D1's notice for those failures.
2. **Selection of decisions 1, 2, 9 and 11 on the corrected keying,** after RV105 confirms this revision.
3. **Decision 16 now includes M ≤ 6.0 GiB,** under the owner's decision of 2026-10-06. B0 selects no M and no cap value; the B1 study proposes them.
4. **Nothing owner-held needs a decision now.** Decision 17 (M above 6.0 GiB, or a supported-machine statement) and decisions 18 and 19 stay prepared, not decided. The owner's information on decision 1 is already in ROOT's ruling.

## 4. Checks I made for this revision

- **Collision checks** (`git grep -F`, outside `P/execution`, at `b75069e6de`; `4e114af645` is records-only beyond it):
  - `NoTriggeredCase`, `OperandPreparation`, `operand_preparations`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `RP-PREPARED-EXACT-DUAL-v1`, the full `openpipestress.result_semantics/0.3.0/physics-retained-1` and `exact_straight_retained_w1a_v2`: 0 lines each;
  - `combination_operand`: 7 lines, all inside the two unrelated identifiers that decision 12 names;
  - the bare `physics-retained-1` outside the preview successor: 1 comment line (`outputPolicy.ts`).
- **The code and record facts** each finding relies on were re-read at the basis:
  - RS, PY and TS `not_required` and report rules (RV105's `reader_rules.txt`; TS `ordinaryAttempts`);
  - `ordinary_value`'s `recovery_demoted`;
  - `ordinary_solve_attempted`;
  - the W2, W2b and W4 witnesses and `attempted_examples`;
  - RR:2186–2194;
  - CORPUS's bases (RV105's `corpus_bases.log`: 17 bases, all Sensitive with a report and no W2; L = 0 dense has a parity row).
- **The 0.9 M arithmetic at 6.0 GiB:** 0.9 × 6,442,450,944 = 5,798,205,849 B. That is 2,202,717,115 B above QUAL §3's dense E_mov,max + R, and 2,222,427,563 B above the sparse one.

## 5. Limits

- **Nothing was run.** W-C2's case C, and the verdicts of W2, W2b and K2a's deferred formation, remain for B1's probe.
- **That two-body B and the one-body pair are W2-published** is RV105's inference from records and code. W6's is recorded in QUAL.
- **Decision 21's effect on committed tests** is unknown until the probe records the error tags.
