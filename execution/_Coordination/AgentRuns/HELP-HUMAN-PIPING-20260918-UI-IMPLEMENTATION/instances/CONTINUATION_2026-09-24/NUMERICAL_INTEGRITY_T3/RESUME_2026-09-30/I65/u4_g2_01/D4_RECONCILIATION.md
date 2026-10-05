# D-4 draft: C1 §2's upstream no-wrap premise and the accepted checked/sticky counter route

**Proposed reading:** for the first admission domain D1, C1 §2's pre-execution no-wrap premise is satisfied by the conjunction of four facts:
1. the accepted checked/sticky work custody;
2. the checked count and index sites, plus the D1 caps;
3. a bounded legacy ledger;
4. an exact-only projection in U1.

No global cumulative no-wrap theorem is needed. C1 §2 names this route as its own alternative, so the ruling applies an existing clause rather than changing the contract. Nothing in the accepted schema or the readers changes.

The draft is for ROOT's ruling at G2's return. It is a reading plus a list of obligations; it is not itself evidence that U1 meets them.

## 1. The governing text

C1 is `R/I32/f2a_wire_c1/WIRE_CONTRACT.md` §2. Lines 64 and 66 are quoted verbatim:

> 64: The kernel's Work/ SumWork/InvocationMeter totals saturate at u64::MAX, but upstream SumWork components, some aggregate expressions and stage accumulation use ordinary additions. Returned small, internally consistent counters and checked projection sums do not prove that an upstream counter never wrapped. Exact-charge reliance requires independently checked **upstream no-wrap evidence**, bound before execution to the actual admitted source/count/build profile. P1–P5 must cover cumulative component increments, aggregate and stage additions, maximum unguarded segments, whole attempts/invocations, reuse and every stopped/error path. The 20B/60B intermittent thresholds and provisional byte target alone supply no such proof. Without that established admission premise, decline W1 before execution and preserve the ordinary transaction; a retrospective equality check cannot grant it. A separately authorized checked/sticky-overflow seam is an alternative only after its own bounded design, maintained write-set and review, not an implicit change in this contract.

> 66: With that upstream premise established, the projector additionally requires nonsaturated, internally consistent counters and checked sums. An observed u64::MAX cannot distinguish exact equality from saturation; mark `saturation_not_excluded`, never invent lost counts. A checked inconsistency, max counter, sum beyond u64, or count beyond the safe JSON range prevents selected successor emission. Unexpected missing no-wrap evidence must never be labelled exact. Native arithmetic failures/panics are not repaired by JSON projection. The upstream premise is an open implementation-blocking proof obligation, not a result established by this draft.

**Nearby text:**
- **Line 62** keeps every successor integer in [0, 2^53−1] with checked arithmetic in all three readers.
- **Line 68** routes an unencodable or ambiguous run to the preserved ordinary base, with `RETAINED_PRECISION_UNAVAILABLE`, reason `receipt_encoding`, and detail `work_counter_range|work_counter_inconsistent|saturation_not_excluded`. That fallback "must be reserved before allocations".

**The record of how the alternative was taken up:**
- **RR:5024** (C1 no-wrap ruling) made the premise an implementation blocker and named the checked/sticky alternative as requiring "its own bounded design/source grant/review".
- **RR:5144** chose that alternative over extending the global static proof.
- **RR:5210** accepted I34's design. It said that does "not lift C1's exactness blocker". Still required at that point were "the concrete ownership/API/error/reset and caller coverage, source implementation, scalar admission, layout/profile and qualification".
- **RR:5260** selected the concrete API, I34 API-02 at `51d8d9fc1e`.
- **RR:5468** accepted the source `fdae294643b` after RV51's complete count/consumer trace, "on the stated 64-bit source/consumer scope".
- **I34 DESIGN.md:230–233** states that this mechanism "replaces the *unprovided global cumulative no-wrap bound* for receipts", and that "C1 §2 must explicitly accept this qualified mechanism as an alternative premise; until then its current pre-execution no-wrap requirement remains the governing blocker".
- **I34 API_PLAN.md:304–306** gives the form of that acceptance: "qualified checked-source/carry mechanism plus closed pre-execution scalar admission is the alternative to the unprovided global lifetime no-wrap theorem. Every Run/Build/Call amount must be E before projection, then satisfy the existing conservation and safe-JSON checks."
- **I52 reader_integration_04/RETURN.md:23–25**, released by ROOT at RR:7157, already treats "checked/sticky WorkTotal.exact()/status()" as superseding the historical global premise. This draft supplies the explicit C1 §2 acceptance that I34 asked for, scoped to D1.

## 2. The four facts, item by item against RR:5210's list

| RR:5210 item | Where it stands now | Evidence |
|---|---|---|
| Ownership, API, error, reset | Selected: I34 API-02 (RR:5260) | `FK/work.rs`: `WorkTotal{amount,status}`; `add`/`mul`/`remainder` are checked and latch Overflow or Inconsistent; `exact()` returns `Err(fault)` whenever any fault bit is set (FK/work.rs:56–120) |
| Source implementation | Accepted: `fdae294643b` (RR:5468, RV51) | RV51's SOURCE_TRACE; I37 RETURN:147 ("No known accounting-loss escape remains in the converted instrumented schedule") |
| Caller coverage | Accepted for the kernel consumers RV51 traced. **Projection coverage is U1's obligation (§3)** | the checked accessors listed in §3 exist at NUM |
| Scalar admission | **Closed for D1 by the caps** (RESIDUALS.md T22) | every named site (6n, offsets, residual m/64m, tracker sequences, n·n) is a checked operation with a typed `CountRange` stop, and its cap-evaluated value is far inside its width |
| Layout/profile, qualification | U4 G5/G6 | not a no-wrap matter; listed because RR:5210 lists it |

**The legacy exact-source ledger** (the `legacy_source_work` that T1 (a) projects) is outside the retained kernel, and its no-wrap argument is direct:
- **Exact-boundary `Work::charge`** uses `checked_add` and refuses beyond `operations` (FKS/exact_boundary.rs:80–87). So `charged` ≤ its limit.
- **PP's `SourceRecoveryBudget`** adds only after the guard `amount <= invocation_limit − charged` (PP/lib.rs:883–896). Its limits are 4,000,000 per case and 64,000,000 per invocation (PP/lib.rs:837–839), far below 2^53.
- **`rejected` uses `saturating_add`** (FKS/exact_boundary.rs:83). It is exact unless it equals `usize::MAX`, which lies outside the safe JSON range. So `saturation_not_excluded` keeps its literal C1 meaning for this one counter.

## 3. What U1's exact-only projection must guarantee

These are obligations on U1's serializer. U1's review should check each one, and U4's G4 records conformance under design-to-budget.

**Already adopted.** ROOT's disposition "Checked work custody in U1" (RR, NUM `21fe3e923d`) adopted items 1, 2 and 4 for U1 during this grant, independent of how D-4 is ruled. RV82's brief checks them (item 6). That disposition uses a typed `receipt_failure` for an abandoned projection, and item 2 here should be read the same way. Item 3 (D-4b) and items 5–6 remain as written.

1. **Read every work amount through a checked accessor.**
   - Run, Build, Call, attempt, stage, lane and invocation amounts are taken only from the checked views:
     - `AttemptRecord::checked_own_work`, `checked_shared_work`, `checked_verification_work`, `checked_verification_shared_work`, `checked_stop_rule_work`, `checked_case_charge` and `checked_invocation_increment` (FK/adaptive.rs:2788–2815);
     - `StageWork::checked_total` (FK/adaptive.rs:1182);
     - `RunWork::{case, invocation_before, invocation_increment, invocation_after}` (FK/adaptive.rs:4385–4401);
     - `InvocationMeter::checked_charged` (FK/adaptive.rs:111);
     - `checked_lme` on the sums (FK/wide_sum.rs:104; FK/wide/multi.rs:1142).
   - **Never use the legacy `u64` compatibility fields** (`shared_work`, `stop_rule_work`, `verification_work`, `verification_shared_work`, the per-stage slots, `InvocationMeter::charged()`). These hold `legacy_saturated()`, which is `u64::MAX` on any fault (FK/work.rs:117–119), so a fault would reach projection disguised as a number.
   - Experiment 02's reference emitter reads several of these legacy fields: `r.shared_work`, `r.stop_rule_work`, `r.verification_work`, `r.verification_shared_work` and `inv.meter().charged()` (R/I61/receipt_experiment_02/_run_records/emitter_i61_receipt_probe.rs:313–314, 326, 511). **U1 must replace them.**
   - A per-stage slot may be emitted only when its `StageWork` status is exact. `StageWork::set` latches before storing (FK/adaptive.rs:1236–1241).
2. **Exact or abandon.** Each amount goes through `exact()`.
   - `Ok(v)` with v ≤ 2^53−1 is emitted.
   - Any `Err` abandons successor finalization for the invocation. That means the ordinary base, the unchanged spent ledger and the pre-reserved `RETAINED_PRECISION_UNAVAILABLE` notice (C1:68; I34 API_PLAN.md:304–310).
   - **No panic.** The experiment emitter's `exact(...).unwrap_or_else(panic!)` (emitter line 30) is not acceptable in maintained code.
3. **The fault vocabulary.** Two vocabularies are in play.
   - **The typed cause.** ROOT's disposition (RR "Checked work custody in U1") requires an abandoned projection to carry a typed `receipt_failure`. Its `check` enum is accepted in the schema (`schemas/retained_precision_mp_v2.schema.json:6505–6514`): `work_counter_range`, `work_counter_inconsistent`, `saturation_not_excluded` and others. Using that enum needs no schema or reader change. The mapping is:
     - an exact value above 2^53−1 → `work_counter_range`;
     - O → `work_counter_range` (the true total exceeds u64, hence the safe range);
     - I or OI → `work_counter_inconsistent`;
     - a saturated legacy `rejected` → `saturation_not_excluded`.
   - **The detail tokens.** I34 API-02 (API_PLAN.md:312–315, under RR:5260) also selected `work_counter_overflow`, `work_counter_unknown` and so on as diagnostic detail text on the ordinary fallback. `work_counter_overflow` is **not** in the accepted `check` enum, so a typed cause cannot carry it without a schema change. **Decision D-4b below.**
   - In D1 (one case), a fault abandons the whole successor; no emitted successor ever contains an unavailable case for a work fault.
4. **Legacy ledger.** Project `charged` and `limit` as exact.
   - Project `rejected` only when it is ≤ 2^53−1. It cannot be `usize::MAX` without lying outside that range; otherwise the typed cause is `receipt_failure` with check `saturation_not_excluded`.
   - The values come from the typed G-l capture (U1), never from parsed diagnostic text.
5. **Conservation still applies.** The existing checks remain, unchanged: own = W+K, stages, D+Q ≤ O, shared stages = S+V, B/T conservation, build flags, execution order and before/after (I34 DESIGN.md:235–236; C1 §1).
6. **The readers already enforce the successor side.** The accepted Rust reader rejects any `work_accounting` cause anywhere in a Run, build or group (P/core/reporting/result_export/src/retained_precision.rs:904–917), so a faulted run cannot be laundered into a selected successor. U1 adds no reader change.

## 4. What the pre-execution side guarantees for D1

- **The domain predicate is checked at G-A, before any W1 owner exists** (DOMAIN.md). Inside D1, every named count/index scalar is representable (RESIDUALS.md T22). An out-of-domain fact declines W1 before execution with the unavailable precondition `resource_admission` or `source_family`. A missing checked view would be `upstream_no_wrap`. In D1 none exists, because every amount above has a checked accessor.
- **No work counter needs a pre-execution bound.** Overflow is detected and latched rather than lost, so the honest result is "unavailable", never "wrong".
- **The 20B/60B thresholds keep their C1 meaning.** They are stop thresholds, not the proof.

## 5. Proposed ruling text (for ROOT)

> For the first admission domain D1 (RR U4 plan ruling D-1), C1 §2's upstream no-wrap premise is satisfied by the accepted checked/sticky work custody (I34 API-02, RR:5260; source `fdae294643b`, RR:5468), the checked count/index sites with D1's caps (I65 G2 RESIDUALS T22), the guarded legacy ledger, and U1's exact-only projection (I65 G2 D4_RECONCILIATION §3, items 1–6). This applies C1 §2's own named alternative and changes no schema, reader or public meaning. A work fault abandons the successor to the preserved ordinary base with the pre-reserved unavailable notice. U1's review checks items 1–6; I65 G4 records conformance.

**Decision D-4b:** the overflow vocabulary.
- **Recommended:** the typed `receipt_failure` uses the accepted `check` enum, with O → `work_counter_range`. This needs no schema or reader change. Any human-readable text stays U1's fixed product text (G-a).
- **Alternative:** adopt I34's `work_counter_overflow`. That needs a schema enum addition and a reader round, so it is not recommended for the milestone.

## 6. Limits

- This is a reading and an obligation list. It does not test U1, and it adds no proof beyond RV51's accepted trace and the D1 caps.
- It covers D1 only (one case, no combinations). Combination and multi-case projection, including unavailable cases inside an emitted successor, remain wider F2a work.
