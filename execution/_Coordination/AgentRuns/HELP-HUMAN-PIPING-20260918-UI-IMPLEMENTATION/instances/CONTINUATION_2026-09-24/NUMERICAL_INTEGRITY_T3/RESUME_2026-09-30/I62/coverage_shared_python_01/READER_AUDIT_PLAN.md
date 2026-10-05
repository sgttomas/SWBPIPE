# Reader audit plan (I62 checkpoint C2-0)

**Status: plan only.** No shared file was edited and there were no Git writes. The only READER edit is the docstring ruled on: `tests/test_retained_precision_contract.py::test_old_operational_error_is_retained_independently_of_new_ready` now states that it tests reader logic only and that no native trigger is established.

**Run window:** 2026-10-03T21:06:05Z to the freeze below.

**Sources:**
- **Contracts** (R = RESUME_2026-09-30):
  - C1 = R/I32/f2a_wire_c1/WIRE_CONTRACT.md (sha256 c8ab2318);
  - C2 = R/I32/f2a_wire_c2/CONTRACT_DELTA.md;
  - C3 = R/I52/prepared_public_contract_02/C3_DELTA.md;
  - F1 = R/I52/prepared_public_contract_correction_03/ADDENDUM.md;
  - S06, S07, S08 = R/I52/reader_contract_seams_{06,07,08}/ADDENDUM.md.
- **Native code** at CODE/NUM 652ad0cc1f:
  - FK = P/core/solver/frame_kernel/src/structural/retained;
  - PP = P/core/product_physics/src.
- **Reader shorthand:** "Py" is `retained_precision.py` 3b12ca7511, after C1c. Rust and TypeScript statuses are marked *audit* unless I know them.

## Part 1: G5 obligation checklist (one list for all three readers)

**Within-gate precedence** (C3:294–297 and the G5 row at C3:304; C1 G5 row at C1:148):
1. **P1:** the native schedule and origin checks.
2. **P2:** C3 run/source/ordinary references and the allowed stage/lane sequence.
3. **P3:** typed check/result consistency.
4. **P4:** the work, status, conversion-prefix and merge equations.

Within each class, order is ascending attempt, then member, then lane, then row index. The first failure wins.

**Codes:** native schedule and graph failures use `ATTEMPT_MISMATCH`, native work uses `WORK_MISMATCH` (C1:148), C3 association uses `PRODUCT_ATTEMPT_MISMATCH`, and C3 work uses `WORK_MISMATCH` (C3 G5 row).

### A. Native schedule, terminal and reason domain (P1, ATTEMPT unless marked W)

| ID | Obligation | Clause | Native path for each permitted variant | Known reader status |
|---|---|---|---|---|
| N1 | ≤4 physical records indexed 0..n−1; ≤3 logical attempts; records empty iff attempts empty | C2:201; C1:105,108 | `FK/adaptive.rs:4520` (three candidate slots, four solves); pre-schedule refusals return empty attempts (C1:54) | Py: yes (C1a) |
| N2 | The first logical attempt is a fresh p128 in record 0 | C1:25–31, C1:148 | `adaptive.rs:4519–4521` | Py, Rust: yes |
| N3 | Failed candidate solve: Pivot/Condition/ResidualGate advances one slot (fresh next candidate); other stops are terminal | C1:31 | `adaptive.rs:4542–4552`; `escalates()` at 223–228 | Py: **no** (not checked). Rust: I59 says yes |
| N4 | Failed verification solve: candidate Rejected(VerificationFailed), verification Failed(stop); escalating stops advance two slots; the failed verification never becomes a candidate | C1:27,31 | `adaptive.rs:4576–4590` | Py: **no**. Rust: yes (I59) |
| N5 | Verification-pass failure is terminal | C1:31 | `adaptive.rs:4593–4611` | Py: no. Rust: audit |
| N6 | Rejected candidate → its verification becomes the next candidate (`verification_then_candidate`, `reused_verification` pointing at the immediately prior attempt), only if c+1<3 | C1:28–29; C1:108 | `adaptive.rs:4713–4735`, `4738–4758` | Py: partial (prior link, role). Rust: "immediate prior reuse" (I59) |
| N7 | Accepted + Verified ends at `finish_selected`; the last logical attempt is accepted, the last record is Verification/Verified, kernel_terminal selected | C1:30; C1:104 | `adaptive.rs:4696–4711` | Py: yes |
| N8 | Ceiling: rejected p512 with a completed p1024 verification gives Unresolved{ceiling}; never an accepted 1024 candidate | C1:30,49 | `adaptive.rs:4724–4766` | Py: not checked. Rust: audit |
| N9 | Work-status fault gives Unresolved{WorkAccounting{fault,prior}} (prior omitted when the stop itself is WorkAccounting); otherwise `terminal(stop)` gives Unresolved or Refused | C1:114 | `adaptive.rs:4769–4800`, `4356` | Py: no. Rust: audit |
| N10 | Invocation entry: meter fault → Unresolved{WorkAccounting}; exhausted (charged ≥ Li) → Unresolved{Budget(invocation)}, idle run, group null, zero attempts | C2:117, C2:219 item 3; C1:60 | `adaptive.rs:4994–5015`, `108–109` | Py: no. Rust: audit |
| N11 | Group refusal (preparation refused) gives a run with group null and refused terminal; no attempts | C2:209 | `solve_cases_projected` groups (`adaptive.rs:4989–4993`) | Py: no |
| N12 | Reason is a closed structural translation (no Debug, no `other`) of AttemptReason/Stop/Unresolved/Refusal | C1:114; C2 §2 (C2:17–75) | `adaptive.rs:145–211, 2477–2530, 2585–2630`; `bound.rs:222–290` | Shape at G1. Py: the G5 reason/outcome compatibility is partial |
| N13 | Physical role/outcome compatibility: candidate {accepted, rejected, failed}; verification {verified, solved, failed}; VtC keeps its later candidate outcome; logical outcome ∈ {accepted, rejected, failed} | C1:27–30,109 | `adaptive.rs:4543–4698` | Py: partial (candidate role check). Rust: yes (I59) |
| N14 | residual_basis = p+64 (1024 at p1024); limbs 4/4/8/16; corrections ≤3 | FK `adaptive.rs:1–14` doc; C1:105 | `adaptive.rs:4161` (run! table) | Py: residual and limbs yes, **corrections ≤3 no**. Rust: yes |
| N15 | Selected summaries equal the candidate record (pivot, rcond, residual_worst, corrections) and the verification record (resolution, theta, B non-null) | C1:148 ("selected terminal and summaries") | `finish_selected` | Py: yes |
| N16 (W) | Per record: own=wide+exact_sum=Σown_stages; stop_rule_lme=stage; verification_lme=scale+estimate+charge+bound+shift; D+Q≤O; Σshared_stages=S+V | C1:7–42 | `adaptive.rs:4645–4653` (latch/stages) | Py: yes |
| N17 (W) | Fragments: every B once, T on candidates only (zero on verification); per-attempt and run sums; invocation chaining; final guard of selected cases (`used > room`), honest unavailable overshoot accepted | C1:20–40, C1:60 | `budget.charge/retain` (`adaptive.rs:4654–4659`) | Py: sums yes; **overshoot rule and final-guard semantics not audited**. Rust: audit |

### B. Cache, build, call and group graph (P1)

| ID | Obligation | Clause | Native path | Status |
|---|---|---|---|---|
| C1 | Build per slot request: success, or a non-budget failure that is cached (later runs reuse it with the same build id, case charge and no invocation shared charge); a budget failure is not cached (a retry gets a new build id) | C1:42; C2:219 item 3 | `obtain` `adaptive.rs:3984–4026` (budget not cached at 4020) | Py: **no failure-state checks** (requires `state != budget_failure` for cache entries only) |
| C2 | Slot naming s{p}/v{2p}; build.work = Σstages; build origin (call, run, record, phase) = the first building record; reuse points backward | C2:115–117 | `obtain`/`obtain_verify` (3984, 4206) | Py: yes |
| C3 | cache_before/after are ordered (s128…s1024, v256…v1024), first occupied slot wins, and reference only earlier same-group builds | C2:117; C1:133 | GroupCache | Py: yes (order and origin) |
| C4 | Call: owner/source/run arrays correspond 1:1 in the submitted valid case-source slice; run ids consecutive across calls; Σ run increments = after−before; body.work.charged = final after | C2:119, C2:139 | `solve_cases_projected` (4981–) | Py: yes (case batch). Combination calls out of scope |
| C5 | Group: call-local; first-equality assignment by full K4STF bytes; separate calls keep separate groups; source_refs in first-seen order; preparation ready or refused | C2:143 | `adaptive.rs:4989–` (group linear search) | Py: stiffness equality and first_source only; **two-group order not tested** |
| C6 | execution_order enumerates every Run once in order (including exhausted-before-start idle runs); refused pre-source calls consume none | C2:117,139; C1:99 | `adaptive.rs:4994–5015` | Py: yes for case runs |

### C. Ordinary and source_decline relationships (P2, ATTEMPT)

| ID | Obligation | Clause | Native path | Status |
|---|---|---|---|---|
| O1 | One ordinary_attempt per requested case, in request order; quality_binding present for every selected case | C2:149, C2:161 | PP ordinary capture (C2:165 seam, `lib.rs:3559, 3690–3785`) | Py: yes (G3/G5 partial) |
| O2 | initial ∈ report / structural_failure / formation_failure / not_attempted, consistent with status: selected and not_required need an attempted report | C2:151–156 | `lib.rs:3559, 3690–3785` | Py: partial (`not_attempted` vs status; report diagnostic) |
| O3 | w2 not_triggered / published / failed, preserving the initial trigger; no extra evaluation | C2:157 | same | Py: **no** |
| O4 | legacy_source disposition; private `abandoned_at_selection` never appears | C2:159–160 | source receipt seam | Py: **no** |
| O5 | source_decline (unavailable case without source): input_owner, constructor_counts, SourceError; a case-source rejection is a no-call decline | C2:115–119; S06:51 | `PP:3249` `PrimitiveSource::new` | Py: **no**. No faithful base yet (deferred) |

### D. C3 typed error, stage and failure-prefix precedence (P2/P3, PRODUCT_ATTEMPT)

| ID | Obligation | Clause | Native path | Status |
|---|---|---|---|---|
| P1 | Unique attempt per owner; owner/ordinary/material/source/run references agree with the case and Run origin; run_ref null iff no native call | C3:165–169 | `PP` prepare_case → native (3135–3260) | Py: yes |
| P2 | Stage values come from entered/returned transitions only: no `entered` state in a receipt; Ready ⇒ all stages completed and all checks passed; checked(passed=false) ⇒ stage failed and check failed | C3:196–201; `retained_receipt.rs:111–113` | `retained_receipt.rs:45–54` (enter/completed/checked/fail_entered) | Py: Ready yes; **general stage-transition legality partial** |
| P3 | Lanes are an ordered entered prefix of [K, Source]; failed lane last; no lane after a failed K; projection requires two completed lanes; proof_start completed ⇒ two completed lanes | C3:171–176 | `final_case.rs:1572–1577, 344` | Py: yes. Rust: yes (I59) |
| P4 | Projection outcomes are the entered prefix (complete when projection completed), in ascending row order over hull-projected rows only (excluding observed ancillary rows, support norms and maxima) | C3:178–190 | `final_case.rs:1700–1720` | Py: Ready exact set at **G5**. C3:302 (G3 row) says "row-index coverage" at **G3**, so there is a gate divergence to rule on (Part 2 item 8) |
| P5 | Conversion kinds: Normal is normal or ±0; Subnormal has nonzero subnormal bits plus relative-precision metadata; Underflow keeps sign and Ready needs +0; Overflow is impossible on Ready; preparation has 9 ordered conversions | C3:183–195 | `final_case.rs:1710` (project_hull), `PP:3215–3231` | Py: yes, except the subnormal metadata |
| P6 | Completion: not_entered / merged (after certify_final or abandon_values) / separate_failure (ProductValuesFailure) with its own visits and capacity; never double counted | C3:253–257 | `final_case.rs:1766–1774`; `PP:3476, 3481` | Py: partial. Rust: merged-before-values allowed (I59) |
| P7 | Failure-prefix table (old/members/new) per F1:86–96; captured_prefix ⇒ source/run null, unavailable | F1:84–100 | `PP:3141–3253` | Py: captured_prefix and prefix shapes yes; **evaluator-returned-then-failure row not tested** |
| P8 | PublicFailure ↔ case reason code/phase table (exhaustive) | S06:30–45 | `PP:3605–3634` typed_trace | Py: yes |
| P9 | Check/error consistency: certificate failed ⇒ proof{cause}; observables failed ⇒ observable{cause}; G5a failed ⇒ g5a{cause}; Numeric may carry a null cause | C3:279–287 | `PP:3492–3543`; `retained_receipt.rs:114–119` | Py: **no** |
| P10 | summary_coverage null/complete rules (I57 §3) | I57 §3–4 | `final_case.rs:1195` | Py, Rust, TypeScript: yes (05c) |
| P11 | Native stage completed ⇔ its selected Run; native nonselected ⇒ native failed and referenced | C3:200 | `PP:3614–3617` | Py: partial (via the reason table) |

### E. Source-bound work, status and completion (P4, WORK)

| ID | Obligation | Clause | Native path | Status |
|---|---|---|---|---|
| W1 | Count exact/unavailable; status joins counts and sticky_status; Ready ⇒ exact, no adapter fault, no lost scalar | C3:229–236 | `NumericWork.status`; `PP` adapter | Py: Ready only |
| W2 | Lane correction calls ≤1; lane data_capacity = view data_capacity; projection_conversions = outcome count when exact | C3:171–176, 180–181 | `final_case.rs:344–` | Py: yes |
| W3 | Preparation conversions count = conversions length | C3:193–195 | `PP:3215–3216` | Py: yes |
| W4 | Adapter counts are the cumulative prefix of the owning ProductCapture; never summed as a budget | C3:237–242 | `PP` capture_entry | Py: no (attested) |

## Part 2: snapshot-06 plan

**Rules for every item:**
- Mutations use `rehash:"all"`. A new base rehashes everything and rebinds the invocation digest and K4SRC/K4STF when its inputs change.
- The expected first gate and code are identical in all three readers.
- All bases are labelled synthetic.
- An item that needs a producer-solved witness is **deferred, not approximated**.

| # | Item | Native path / why faithful | Expected first failure | Rehash |
|---|---|---|---|---|
| 1 | **Failed-candidate skip base:** p128 candidate Failed(Pivot) → fresh p256 candidate accepted, v512 verified | `adaptive.rs:4542–4551`. A single-case receipt has no identical-input contradiction; the Pivot trigger is a synthetic private attestation | positive. Mutations: next attempt reused instead of fresh → G5 ATTEMPT (N3); skipped slot without the failed record → G5 ATTEMPT (N2/N3) | all |
| 2 | **Failed-verification skip base:** p128 candidate Rejected(VerificationFailed), v256 Failed(Condition) → fresh p512 candidate accepted, v1024 verified | `adaptive.rs:4576–4590` | positive. Mutation: the failed v256 reused as candidate → G5 ATTEMPT (N4) | all |
| 3 | **Shared non-budget failed cache across two cases with identical inputs:** case 0's s128 build fails non-budget (Pivot) → escalation; case 1 reuses the cached failed slot with the same build id and no shared invocation charge | `obtain` `adaptive.rs:4001–4023`. Identical inputs give the *same* failure, so this is consistent (C2 control 3) | positive. Mutations: a reused failed slot given a new build id → G5 WORK (C1); a budget-failed slot cached → G5 WORK (C1) | all |
| 4 | **Two groups in one call:** case 1 selects a material basis with E and G ×2 (named or interpolated point) | Stiffness differs → a separate group (C2:143). With linear statics, a power-of-two E/G scaling gives exactly 2^-1× displacements/rotations and identical forces, so the rows are exactly derivable. **Also serves G8 named/interpolated material** | positive. Mutations: both sources in one group → G5 ATTEMPT (C5); group order swapped → G5 ATTEMPT; interpolation target at a bracket point (strict-bracket rule) → G8 PREPARATION | all |
| 5 | **Units:** promote the Python-only mm normalization (positions ×1000, units mm) to a shared must-pass; a single un-normalized coordinate → G8 PREPARATION; missing alpha / duplicate temperature on the interpolated base → G8 | Existing Python controls; `lib.rs` unit conversion (G8 mirror) | as stated | invocation, then all |
| 6 | **Conversion encodings** on the base: kind `subnormal` with normal bits; `normal` with subnormal bits; Ready with `underflow` on a nonzero row; Ready with `overflow`; missing subnormal metadata | C3 conversion rules (P5) | G5 PRODUCT_ATTEMPT for each; G2 ENCODING for an invalid metadata encoding | receipt |
| 6d | **Positive subnormal/underflow rows** | Needs solved subnormal final values (scaling into the subnormal range is not exact) | **deferred** (producer witness) | |
| 7 | **Decisive G7 mutation:** `contract_evidence.preview_cases[0].pipe_stress_extrema[0].value_upper_pa` raised by one ulp (only G7 reads enclosures) | `preview_physics_evidence.py:300` | G7 with the base code `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`. **Divergence:** Python currently reports `"<code>: <detail>"` as the code; all readers must emit the bare base code (ROOT to confirm) | publication + receipt |
| 8 | **Exact final-row coverage:** drop one hull-projected outcome; add a support-norm row index; swap two outcomes | C3 P4 | **Ruling needed:** C3's G3 row ("row-index coverage") versus Python's Ready check at G5. Proposal: missing/foreign/unsorted index → G3 COVERAGE; a valid sorted subset on Ready → G5 PRODUCT_ATTEMPT | receipt |
| 9 | **Exact native coverage:** layout and maps already pinned (05a); add id_maps `support_ids` count ≠ model supports; member node swap | G8 maps (`_g8`) | G8 PREPARATION | all |
| 10 | **G8 failed-prefix association:** on the 05a P/prefix bases, old operational inputs not bound to the invocation (old J changed) | F1:105, G8 old/new tuple rule | G8 PREPARATION | all |
| 11 | **Post-evaluator failure (F1 row 6):** helper and new evaluator returned; the later map-write/push fails | `PP:3242–3245` (evaluate, then capture_entry/push). Synthetic accounting trigger (defensive, as ruled) | positive. Mutation: new entry dropped → G5 PRODUCT_ATTEMPT (prefix rule) | all |
| 12 | **Maxima-failed, merged-before-values positive** | Already a 05a must-pass (`maxima_abandoned`). Add mutation: completion `separate_failure` there → G5 PRODUCT_ATTEMPT (P6) | as stated | receipt |
| 13 | **Typed check consistency (P9):** certificate failed with an `observable` cause; G5a failed with a `proof` cause | `retained_receipt.rs:114–119` | G5 PRODUCT_ATTEMPT | receipt |
| 14 | **Observable/G5a failures after a passed certificate,** on the F template | `PP:3529–3543`. Synthetic accounting triggers (G5aError/CaptureError accounting) | positive (two must-pass entries) | receipt |
| 15 | **Values failure (separate_failure) and alias/bind-rows abandon,** on F | `PP:3480–3491`. Synthetic accounting triggers | positive (must-pass) | receipt |
| 16 | **WorkAccounting native terminal:** Unresolved{WorkAccounting} with a run whose record status is inconsistent | `adaptive.rs:4769–4790`. Resource trigger | positive unavailable row on a two-case template (case 1) | all |

**Deferred (need producer-solved witnesses; not approximated):**
- the Ceiling and any case-dependent numeric rejection in a two-case receipt;
- L = 0 (memberless-node admission);
- source-construction failure / source_decline (O5);
- old-Err/new-Ready;
- positive subnormal/underflow final rows (6d);
- budget overshoot and exhausted-before-start (N10): these need ≥20B/60B of actual work, which a synthetic small model cannot honestly attest;
- ordinary W2 published/failed and Formation/Structural initial failures (O2/O3): these need actual ordinary solver failures or force scaling;
- combination calls (out of the case-only C3 scope).

**Effort:** about 6–8 hours across C2-1 and C2-2. Readers then align against Part 1.
- **C2-1 (about 3 h):** items 1, 2, 3, 6, 7, 8 (after the ruling), 9, 10, 12, 13.
- **C2-2 (about 3–4 h):** items 4 and 5 (material/units, two groups), 11, 14, 15, 16.

**Order if time runs short:** 6, 7, 13, 12, 1, 2, 3, then the rest.

**Rulings needed:**
- (a) the G7 code form: bare base code;
- (b) the row-index coverage gate (G3 versus G5, item 8);
- (c) whether the item-3 and item-16 synthetic resource/numeric triggers are acceptable on two-case templates. Item 3 is consistent because identical inputs fail identically.

## Freeze

Frozen at 2026-10-03T21:10:00Z. The C3 line numbers were resolved by phrase search on the sealed C3_DELTA.md.
