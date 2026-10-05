# I62 review repair 07, checkpoint A: native and contract facts for ROOT

**Basis:**
- ROOT_RULINGS_V1 "Reader review RV78–RV81: consolidated ruling and the repair wave" (D1–D15);
- BRIEFS/I62_I64_REVIEW_REPAIR_07.md;
- the reviews REVIEW_RV78–RV81/reader_review_01.

**Status:** facts and proposals only. Nothing changed in READER, and there were no Git writes. The only writes are this file and its SHA256SUMS.

**Run window:** 2026-10-03T23:24:50Z (first tool call) to 23:36Z. The memory guard (PID 5387) was running.

**Sources read:**
- **READER at `6b607fd01f`** (the candidate, which carries snapshot 06d):
  - PY = P/core/analysis_runs/retained_precision.py;
  - RS = P/core/reporting/result_export/src/retained_precision.rs;
  - TS = P/apps/desktop/src/features/results/retainedPrecision.ts;
  - Python and the corpus in the READER working tree are byte-identical to `6b607fd01f`.
- **Native code at CODE/NUM `652ad0cc1f`** (the review basis):
  - FK = P/core/solver/frame_kernel/src/structural/retained;
  - PP = P/core/product_physics/src;
  - FC = FK/product_certificate/final_case.rs.
- **Producer projection status:** PP/retained_receipt.rs is the only `retained_receipt` module. Its first line reads "Private typed C3 seam. No serializer, public receipt, profile or selection." It has no ordinary, diagnostic, `source_ref` or `source_decline` handling at `652ad0cc1f` (138 lines) or at NUM `d566e487f4` (191 lines). **No producer projection of case-level or ordinary receipt JSON exists yet.** D6a and D9b therefore resolve from the contract alone.

**Probes** ran in memory with the venv Python against the 06d corpus at `6b607fd01f`. No file was written in READER.

---

## D1. Can a member inventory be empty in an emittable receipt?

**Finding: not established either way.** The conditional in D1 is therefore **not met**, and an empty CaseSource inventory should not fail G3 on this basis.

**What native code does and does not reject:**
- The native source constructor rejects a source with no nodes (`SourceError::NoNodes`, FK/source.rs:498). `SourceError` (FK/source.rs:190–) has **no** variant for an empty member list, and the member checks (duplicate id, repeated node, properties, FK/source.rs:~510–530) all pass for zero members.
- PP `prepare_owned_case` checks only that the old member, fact and operational counts are equal (PP/retained_product.rs:3157–3158). The member loop (PP:3199–) runs zero times on an empty list, and old coverage becomes `Complete` (PP:3165).
- I found no model-level rule requiring at least one member or pipe segment in PP/lib.rs, in the parts I searched. The search was not exhaustive.

**What remains open:** whether a zero-member model can reach W1 at all. That needs a nonsingular case with every node fully restrained and a numerical trigger. It is not provable from what I read.

**Consequence for D1's unsourced clause:** D1 requires an unsourced `old_coverage=complete` list to be non-empty at G3. That clause rests on the same unestablished fact. An emittable zero-member unsourced complete list cannot be excluded either. I flag this as a possible conflict with D1 (see "Conflicts" at the end).

**Proposed rule:**
- Keep G3's equality and prefix rules.
- Do not reject empty lists until a native rejection is cited.
- If ROOT keeps the non-empty clause as a contract choice, record it as such in the C3 clarification. It is not a native fact.

---

## D6a. The ordinary `diagnostic_refs` list

**Finding:** no producer code fills `ordinary_attempts[].diagnostic_refs`.
- The seam is specified at C2:168 ("private `solve_load_case` observer … retained_receipt owns the new closed projection").
- It is not built: see the producer projection status above.
- The contract requires only "exact existing diagnostics" (C1:100), "ordinary refs resolve" (C1:148, G5 row) and "exact diagnostic refs" (C2:166).
- Nothing requires an untyped listed diagnostic to name the case. The base envelopes carry model-level diagnostics whose `affected_refs` is null (for example `diagnostic:physics:rule-inputs-missing`, as RV81 notes).

**Proposed rule, all readers, G5 ATTEMPT (class 2):**
- `diagnostic_refs` entries are unique and each resolves to an existing diagnostic.
- Typed references stay strict, as already ruled at O2 (C2-2): the initial report or failure, the W2 refs, the formation refs and the legacy_source ref. Each must be listed and must name a diagnostic whose `affected_refs` contains the case.
- No reader requires an **untyped** listed ref to name the case. TypeScript drops that check (RV81 4a).

---

## D6b. `solve_quality` statuses that route to retained precision (I30)

**Native quality mapping** (`assessed_numerical_quality`, PP/lib.rs:1835–1900) maps the case's integrity diagnostic code:

| Integrity code | `solve_quality` |
|---|---|
| CHECKS_PASSED | `checks_passed` |
| SENSITIVE | `sensitive` |
| PHYSICAL_MECHANISM | `failed` |
| NEGATIVE_ENERGY | `failed` |
| RECOVERY_BASIS_UNQUALIFIED | `unresolved` |
| ASSEMBLY_UNRESOLVED | `unresolved` |
| FAILED | `failed` |
| any other code | `unresolved` |
| **no integrity diagnostic for the case** | **`not_assessed`** |

**Where the integrity diagnostic is emitted:** on the report (PP/lib.rs:1116), on a structural failure (1321), and on the range and other failure paths (1824, 3898/3904, 4040/4051, 4896). I did not trace each of those paths in full.

**How the source receipt binds the ordinary outcome** (PP/source_receipt.rs:546–550):

| Ordinary outcome | `solve_quality` |
|---|---|
| `not_attempted` | `not_assessed` |
| `checks_passed` | `checks_passed` |
| `sensitive` | `sensitive` |
| `rejected` | `failed` or `unresolved` |

**I30's routing** (I30/f2a_routing_02/ROUTING.md §3, 59–77) classifies on actual outcomes, not on quality strings. The W1 candidates are:
- an ordinary `Sensitive` result (including load loss and D5 demotion);
- the listed recoverable `NumericallyUnresolved` reasons;
- supported-family `NegativeEnergy`;
- `Range` after the one W2 failure.

A passed non-trigger stays ordinary. Validation and other errors are immediate terminals. C2:153 adds: "A not-attempted case never becomes not_required or selected by inference."

**Finding:** `not_assessed` means the case was not attempted (no integrity diagnostic). That never routes to W1. **A selected case's `solve_quality` is `sensitive`, `unresolved` or `failed`.**

**Proposed rule, all readers, G5 ATTEMPT (class 2):**
- A selected case requires `solve_quality ∈ {sensitive, unresolved, failed}`. This excludes `checks_passed`, already ruled, and `not_assessed`.
- **Caveat:** if ROOT wants certainty that every attempted outcome emits an integrity diagnostic, the paths at PP/lib.rs 1824–4896 should be traced in full. I traced the report and structural-failure sites only.

---

## D8. The accounting class

### Every schema shape carrying an accounting event, a fault or a lost flag (06d schema at `6b607fd01f`)

| Shape (schema `$defs`) | Where it can appear | Native emission condition | Emittable? |
|---|---|---|---|
| `ProductAttempt.adapter.fault {kind:"overflow",event}` | every product attempt | Set only when `counts[event] + amount` overflows; the counter is left unchanged (PP:2896–2907). Every amount is under 2^63 + 2^61 (constants, `size_of`, live allocation lengths). So the retained count is at least 2^62 > 2^53−1. | **Never** (C3:233–236; C1:66, 68) |
| `CaptureError {kind:"accounting",event}` | PublicError preparation/capture/abandoned/observable causes | Built only from the sticky adapter fault (PP:2910–2912, 245–267, 617) | **Never** (needs the fault above) |
| `G5aError {kind:"accounting",event}` | PublicError g5a; check errors | Built from the adapter fault (PP:2527, 2586) | **Never** |
| `ScalarTrace.lost` (`ProductAttempt.overlay_work`, `g5a_work`; `MemberOperational.work`) | attempts; old/new operational entries | Set only when `entered` or `checks` fails `checked_add(1)`, leaving u64::MAX (PP:2310–2349) | **Never** |
| `OperationalError {kind:"accounting"}` | `MemberOperational.result.error`; CaptureError `prepared_arithmetic.cause`; G5aError `operational`/`arithmetic` causes | Returned only when the owning ScalarWork is `lost` (PP:2311–2347) | **Never** (needs `lost`) |
| `SectionError {kind:"accounting"}` | `PreparedMember.result.error`; PublicError `preparation.section` | Non-exact PreparationWork status (FK/product_certificate.rs:676–679), or more conversion entries than the fixed outcome array holds (688), which is a broken invariant | Only with a non-exact status of that member's PreparationWork (688 cannot be represented: at most 9 conversions) |
| `ProductError {kind:"work_accounting",fault}` | PublicError proof/values/abandoned proof; check errors | `Cause::Accounting(f)` from the owning work's status: FC:358–379 (visits and f64 ops), FC:428–436, and FC:1580, 1616, 1744, 1747 and 1792 (`work.status().fault()`), `values_visit` (FC:1668) | Only with that fault in the owner's emitted statuses (Count `unavailable` or `sticky_status`) |
| `Stop {space:"stop",tag:"work_accounting",fault}` nested in a product NumericError/HelperError (`ProductError.numeric`, `numeric_helper`; SectionError `arithmetic`; BridgeError `numeric`) | product attempts | `NumericError::Arithmetic(AttemptStop::WorkAccounting(f))` from a NumericWork status (FK/product_certificate.rs:186–206; bridge.rs:85; source_residual.rs:159, 707) | Same as the row above |
| `ViewIssue {kind:"work",fault}` (BridgeError `view`) | lane errors; ProductError `native_source` | View work status (FK/adaptive.rs:5279, 5468, 5572) | Same as the row above |
| `Count {kind:"unavailable",fault}` | NumericTrace, PrepWork, LaneWork, ProofTrace | Overflow with u64::MAX stored as unavailable, or `inconsistent` (FK/work.rs:80–113, 170–175) | This is a status, not a cause. Representable (C3:207) |
| Kernel `Stop`/`Unresolved` `work_accounting{fault}` (run records, attempts, builds, terminal) | runs | Any non-exact attempt work means `finish_terminal`, giving a WorkAccounting terminal (adaptive.rs:4545, 4777, 4996) | **Never** (C1:66–68; 06c terminal rule) |
| `Refusal work_accounting{fault}`, `count_range{name}` | Reason (refusal) | None: see D9a | **Never** (remove at G1) |

### Proposed rules

All of these are G5 `RETAINED_PRECISION_WORK_MISMATCH`, in class 4 (C3:304), on every product attempt:
- **R1′ (adapter, unchanged meaning):** fail if `adapter.fault ≠ null`, or if any CaptureError or G5aError `{kind:"accounting",event}` cause appears.
- **R2′ (lost and operational accounting):** fail if any ScalarTrace has `lost = true`, or if any **OperationalError** `{kind:"accounting"}` appears. That includes a G5aError `operational`/`arithmetic` cause and a CaptureError `prepared_arithmetic` cause. Match by location, because SectionError uses the same `{kind:"accounting"}` spelling.
- **R3′ (fault-bearing causes, any spelling):** every fault-bearing cause must have its fault contained in its owner's emitted statuses. The spellings are:
  - `{kind:"work_accounting",fault}`;
  - `{space:"stop",tag:"work_accounting",fault}`;
  - `{kind:"work",fault}`.

  For containment, `both` means overflow plus inconsistent.
- **R4 (SectionError accounting):** a SectionError `{kind:"accounting"}` on a PreparedMember, or as PublicError `preparation.section` (owner: the last, refused member), requires that member's PreparationWork to contain a non-exact status: a Count that is `unavailable`, or a non-exact `sticky_status`.
- **Kernel spelling (class 1, G5 ATTEMPT, not WORK):** a `work_accounting` Stop or Unresolved reason anywhere in a run's records, attempts or builds fails. It extends the 06c terminal rule, because a single non-exact attempt forces the WorkAccounting terminal (adaptive.rs:4545). The 06c mutations already cover the terminal.

### Can R3 bind to the owning trace? Yes, at four owner scopes

The native fault is the status of the work that raised it. The receipt carries that work at these scopes:

| Cause location | Owning trace |
|---|---|
| a PreparedMember error, or PublicError `preparation.section` | that member's `work` (PrepWork) (FK/product_certificate.rs:676–679) |
| `proof.lanes[i].error` | `lanes[i].work` (BridgeError and source residual: bridge.rs:85; source_residual.rs:159, 707) |
| PublicError `values.cause` | `proof.completion` (`separate_failure.visits`), via `values_visit` (FC:1668) |
| PublicError `proof.cause`, the certificate check error, or `abandoned.proof` | the whole ProofTrace: numeric, counts, lanes and completion. `certify_final` joins its rebound work, which retains the lanes (FC:1576–1580, 1792) |

- Binding to a single counter is not derivable from public data.
- **Proposal:** bind to the owner scope above. Fall back to the whole attempt only for a cause with no locatable owner (none in the current schema).

### Corpus entries each rule would move (in-memory probe, all 06d entries)

| Rule | Entries it fails | Effect |
|---|---|---|
| R1′ | none beyond today's R1 pins | — |
| R2′ | none | — |
| R3′ | `cert_failed_before_summary_g5a_passed`, `certificate_check_wrong_wrapper` | stay G5 PRODUCT_ATTEMPT (class 2/3 first) |
| R3′ | `work_accounting_cause_exact_status` | stays G5 WORK |
| R4 | `refused_member_conversion_kind_bits` | stays G5 PRODUCT_ATTEMPT |
| R4 | **`prefix_attached_old_input_unbound`** | **moves from G8 PREPARATION to G5 WORK** |

No base and no must-pass entry fails any of the rules.

**Rebase for `prefix_attached_old_input_unbound` (probed):**
- **New base:** F′ (`two_case_facade_after_certificate_synthetic`). Its case-1 attempt has an attached, prepared member and an emittable allocator-refusal context.
- **New edits:** the entry's single old-input edit only, `retained_precision.body.product_attempts[1].operational.old[0].inputs[9] = "3ff0000000000000"`, with `rehash:"all"`. The helper-refused member, result and fault edits are dropped.
- **Observed:** G8 `RETAINED_PRECISION_PREPARATION_MISMATCH`, at the same `_g8` old-input binding check as today.
- **What is lost:** the unsourced attached-member variant. An unsourced attempt with a PreparedMember needs a helper refusal or a post-helper accounting or association failure. None of those is an allocator refusal (PP:3215–3245), so that variant stays deferred.
- **Also on the non-emittable SectionError context:** `refused_member_conversion_kind_bits`. Its pin (P5 on a refused member) needs a refused member, so it cannot move to an allocator-refusal base. I propose keeping it as is (pinned at PRODUCT_ATTEMPT before R4) and listing it as context-deferred.

---

## D9a. Refusal variants `work_accounting{fault}` and `count_range{name}`

**Finding: neither has a contract or native source.**
- Native `Refusal` has exactly five variants: `MechanismWitnessed`, `GeometryUnavailable`, `NegativeEnergy`, `LedgerUnavailable` and `Structure` (FK/adaptive.rs:2860–2875).
- `terminal()` maps `AttemptStop::CountRange` and `AttemptStop::WorkAccounting` to **UnresolvedReason**, not to Refusal (adaptive.rs:4349–4355). Only `NegativeEnergy` and `Structure` produce a Refusal (4375–4376).
- C2:41–45 lists the same five refusals.
- C3:261–263 imports `CountRange{name}` and `WorkAccounting{fault}` as **AttemptStop**. They are therefore Stop and Unresolved shapes, which the schema already carries, not Refusal shapes.

**Proposed rule:**
- Remove both variants from the schema's `Refusal`.
- Add a G1 `RECEIPT_MISMATCH` mutation for each: a group preparation refusal, or a refused terminal, carrying `refusal/work_accounting` and `refusal/count_range`.
- No 06d entry uses either variant.

---

## D9b. "No source" on an unavailable case

**Finding:**
- No producer projection emits case JSON yet (see the producer projection status above).
- The only contract statement of a no-source unavailable shape is C2:133: "For `retained_unavailable` caused by pre_source_refusal, `run:null`, `source_ref:null`". That is explicit null.
- C3 uses required `null|U` members throughout (C3:65–66: `product_attempt_ref`, `preparation`).
- C1:92 requires absent and null to be "distinguished as stated".
- **The corpus uses "absent" once:** P′ case 1 omits `source_ref` and has `run: null`. No mutation edits a case `source_ref`.

**Proposed rule:**
- `source_ref` is **required** on every unavailable case, as `null|U`, with null meaning no source.
- An absent `source_ref` fails G1 `RETAINED_PRECISION_RECEIPT_MISMATCH`.
- `source_decline` may appear only when `source_ref` is null. Any other combination fails G1.
- **Corpus effect (phase B):**
  - P′ case 1 gains `source_ref: null` (base rehash);
  - two G1 mutations: `source_ref` absent, and `source_decline` together with a non-null `source_ref`.

This is a contract choice by analogy with C2:133, not producer evidence.

---

## D3. The native G5 check table

**Location of the native checks:**
- PY: `_g5_schedule` 354–459, `_g5_cache` 460–486, `_g5_native` 487–634;
- RS: `g5_native` 786–1265, `g5_schedule` 1298–;
- TS: `nativeRuns` 332–474.

| Family (checklist) | PY | RS | TS | Code in all three |
|---|---|---|---|---|
| Run ids contiguous; execution order = runs (C6, R-1) | 491–492 | G3 | G3 | **ATTEMPT in PY, G3 in RS/TS.** This is the R-1 difference; D1 moves PY to G3. |
| Call ids; `invocation_before` chaining; run and call `invocation_after`; `work.charged` (C4) | 495, 504, 584, 598, 600 | 819, 845 | 341, 348, 450, 452 | WORK |
| Call arrays 1:1; run origin = call/position/source/owner; case↔run↔source (C4, P1) | 496–503 | in G3 (D1 moves to class 1) | in G3 | ATTEMPT (RS: G3 today, the D1 move) |
| Records ≤4, indices; attempts ≤3; fresh p128 first; precision order (N1, N2) | 506–512 | g5_schedule | nativeRuns | ATTEMPT |
| Schedule replay: slots, reuse, verification phase, outcomes, terminal (N3–N9, N13) | 354–459 | 1298– | nativeRuns | ATTEMPT |
| Idle and pre-schedule rules; exhaustion (N10, N11) | 370–385, 574 | g5_schedule | nativeRuns | ATTEMPT |
| Cache: built slot not already cached; reuse is the same build; state↔reason; `cache_after` derived (C1, C3) | 475–484 | 937–1001 | 382–400 | WORK |
| Failed build ⇒ requesting record failed with the same stop (C1) | 483 (ATTEMPT) | g5_native | nativeRuns | ATTEMPT |
| `cache_before`/`after` slot order; entries in the same group, non-budget, earlier origin (C3) | 629–632 | 872, 1001 | 365, 400 | WORK |
| Build id; `build.work` = Σ stages (C2) | 624 | 813 | 473– | WORK |
| Build origin = first building record; reuse points backward; build group, slot, cost (C2) | 530–536 | 941–955 | 384, 390 | WORK |
| Record `residual_basis`, limbs, corrections ≤3 (N14) | 518–519, 364 | g5_native | nativeRuns | ATTEMPT |
| Record work partition: own, stop_rule, verification and shared lme; shared-stage projection (W/native) | 521–537, 569 | 906–998 | 372–397 | WORK |
| Attempt↔record pairing and verification phase; fragment set (native) | 541–562, 567 | 1114–1124 | 403–420 | ATTEMPT (fragment set) / WORK (sums) |
| Attempt and run charge/debit; run `invocation_after` (native) | 565, 571–572, 584 | 1124– | 422–429 | WORK |
| Budget scope test, case first (N17) | 580–582 | g5_native | 446 | WORK |
| Selected terminal ⇒ last accepted and verified, verification complete (N7) | 586–588 | g5_native | nativeRuns | ATTEMPT |
| Selected final guard: charge ≤ Lc and after ≤ Li | 589 | g5_native | 436 | WORK |
| Selected summaries = candidate and verification records (N15) | 592–597 | g5_native | nativeRuns | ATTEMPT |
| Call run_refs flatten to 0..n−1 (C4) | 599 | g5_native | nativeRuns | ATTEMPT |
| Group partition: first-seen stiffness, call-local; run group; refused group ⇒ refused terminal (C5, N11) | 614–622 | g5_native (adds call exists; sources unique and in call, R-8) | nativeRuns | ATTEMPT (R-8: RS-only checks; D5e adds them everywhere) |
| Group ids; non-empty sources; same stiffness (C5) | 621–622 | g5_native | nativeRuns | ATTEMPT |

**How this table was checked:**
- **Single defects:** RV78's parity run (211 entries, all agreeing; R-1 through R-8) finds no other ATTEMPT/WORK disagreement. The table above is read from the code at the cited lines.
- **I did not exhaustively diff the three readers line by line.** I63 and I64 must confirm their columns while implementing D3, as the brief requires.

**06d entries whose code would change under the D3 convention** (ATTEMPT wins in class 1; native WORK deferred to the end of class 1):
- **Python: none.** I probed in memory with `_g5_native` and `_g5_cache` WORK predicates collected and raised only after every class-1 ATTEMPT check. All 178 mutations and 18 must-pass entries keep their observed outcome.
- **Rust and TypeScript:** not runnable in this grant. Neither reports a dual-defect 06d entry (RV78 R-7: "undetermined", no corpus case). I63 and I64 confirm during their phase 1.

---

## D12. Checklist corrections

**N11 corrected:**

> Group preparation refusal: the Run is recorded with **its group index** (`Some(index)`, FK/adaptive.rs:5043, `RunPhase::GroupPreparation`), a refused terminal carrying that group's refusal, and no attempts (5034–5052). Group null belongs only to the exhausted-before-start Run and the invocation-entry return (C2:209 item 3; adaptive.rs:4994–5015).

Python already implements this. The old checklist text ("group null") was wrong; the code was not.

**Count:** the checklist has **43 IDs**, not 42:
- N1–N17 (17);
- C1–C6 (6);
- O1–O5 (5);
- P1–P11 (11);
- W1–W4 (4).

R1–R3 (and R1′–R4 if ruled) are separate class rows, not checklist IDs.

**Status of the 13 IDs RV79 disputes.** I accept RV79's status for each. Each row now records the decision that repairs it.

| ID | Status now (Python at `6b607fd01f`) | Repair (decision) |
|---|---|---|
| N5 | partly: exact translation pinned; a pass-entered escalating stop is accepted (B2d) | D5b |
| N13 | partly: candidate-record verification, v-build and `verification_lme` unconstrained (B2b); `verification_failed` with a completed verification accepted (B2h) | D5a, D5c; D11 N13 pins (including rejected + `verified`) |
| C1 | partly: non-budget half pinned; budget-not-cached implemented but untested (M12) | Deferred base (≥20B/60B); reader-local pin under D13 if feasible |
| C5 | partly: groups outside every call partition accepted (B2a) | D5e |
| C6 | checked at the wrong gate (G5, not G3) | D1 (move to G3); D11 C6 pin |
| O2 | partly: typed refs strict; list refs checked for existence only; selected-quality domain open | D6a (unique + resolve), D6b |
| O3 | partly: published W2's nonzero exponent unchecked (B2g) | D6c |
| P1 | partly: B1a, B1b, B1d | D4a, D4b, D4e |
| P6 | partly: values-failed ⇒ `separate_failure` untested (M09); completion unchecked when values completed and aliases not_entered | D11 shared pins |
| P8 | partly: preparation ⇒ no Run and preparation failed; native ⇒ same Run (B1c, B1e) | D4d; D11 P8 pins on F′ and P′ |
| P9 | partly: result error versus first failed stage untested (M14); silent when no stage failed (B1c) | D4d; D11 pin |
| P10 | checked, not isolated (M06) | D11 pin isolating "passed G5a requires coverage" |
| P11 | partly: a case Run with a null `run_ref` is accepted (B1d) | D4e |

All other rows are unchanged: RV79 confirms them.

---

## Conflicts with the ruling, or open points

1. **D1 (unsourced non-empty, and the empty-CaseSource conditional).** No native rejection of a zero-member inventory was found (FK/source.rs:190–530; PP:3157–3165). The conditional is not met. D1's unsourced "non-empty" clause is likewise unsupported natively: ROOT should either cite a source or record it as a contract choice.
2. **D9b.** "Whatever the producer's retained_receipt projection emits" cannot be determined, because the projection is not built (PP/retained_receipt.rs:1). I propose explicit null by analogy with C2:133.
3. **D6a.** The producer seam that fills the list (C2:168) is not built. The conditional resolves to "no reader enforces names-the-case for untyped refs".
4. **D8.**
   - R4 moves `prefix_attached_old_input_unbound`. The rebase onto F′ preserves the sourced P7 pin only; the unsourced attached variant becomes deferred.
   - `refused_member_conversion_kind_bits` has no emittable context and stays context-deferred.
5. **D3.**
   - The Python convention probe changes no 06d outcome.
   - The Rust and TypeScript columns are code-read, not run. Their confirmation belongs to I63 and I64's phase 1.
