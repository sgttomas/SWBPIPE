# I62 C2-4: product-level accounting causes (analysis only)

**Status:** analysis for ROOT's ruling. No shared or READER file was changed, and there were no Git writes. This file and its checksum are the only writes. The probes ran in memory against READER `c765e4f5b3` (snapshot 06c).

**Sources:**
- **Contracts:**
  - C1 §2, lines 56–69 (WIRE_CONTRACT.md);
  - C3 §3, lines 204–264, its error maps at 268–277 and the failure envelope at 376–388 (C3_DELTA.md);
  - F1:84–106 (ADDENDUM.md).
- **Native code** at CODE/NUM 652ad0cc1f:
  - PP = P/core/product_physics/src/retained_product.rs;
  - FK = P/core/solver/frame_kernel/src/structural/retained, including `work.rs`, `product_certificate.rs` and `product_certificate/final_case.rs` (FC below).

## 1. Does C1:66–68 reach product-level accounting causes?

**Scope.** C1 §2 is about kernel counters: it opens with "The kernel's Work/ SumWork/InvocationMeter totals" (C1:64). C1:68 keeps "exact safe counters in its `run`". C3 adds a separate representation for product work:
- a Count may be `{kind:"unavailable", fault}` (C3:207);
- "Ready requires exact statuses, no adapter fault and no lost scalar collection" (C3:232–233);
- "Scalar/adapter numeric arrays preserve their actual retained prefix even when their separate lost/fault bit is set" (C3:233–235);
- "Values above safe-U range cause existing whole-receipt receipt_encoding fallback; no truncation or invented zero" (C3:235–236). This is the same fallback as C1:68, which uses detail `work_counter_range`.

**So the rule is different at product level.**
- For the kernel, a non-exact status means no successor is emitted.
- For a product attempt, a non-exact status alone does not block emission. The receipt is emitted only if every value it must carry is the real retained value and is at most 2^53−1.
- C1:66 ("count beyond the safe JSON range prevents selected successor emission") reaches product causes only through that safe-range clause.

The causes fall into four classes.

### A. Adapter overflow: never emittable

This class covers `adapter.fault` and every `accounting{event}` cause in CaptureError and G5aError.

- **Only an overflow sets the fault, and it leaves the counter high.** `AdapterWork::enter` (PP:2896–2907) sets `Overflow(event)` only when `counts[event].checked_add(amount)` fails, and it leaves the counter unchanged. So at the fault, `counts[event] > 2^64−1−amount`.
- **Every amount is far too small to overflow a safe counter.** Each amount is one of:
  - a constant from 1 to 24, or a `size_of` (for example PP:149, 762, 1295, 3226–3241);
  - a length or capacity of a live allocation, at most isize::MAX = 2^63−1 (PP:509, 526, 788, 2948, 2956, 2964, 2980, 3089, 3095, 3098);
  - at PP:3548, `values.len() + 8·maxima.len() + 2`, which is a sum of live Vec lengths.

  So every amount is below 2^63 + 2^61, and the retained count at the fault is at least 2^62.
- **That count cannot be emitted.** C3:233–235 requires the retained prefix to be emitted, and C3:235–236 forbids truncation and falls back on values above the safe range.
- **Verdict:** no emitted receipt can carry `adapter.fault ≠ null`. Readers can check this publicly.

### B. Scalar trace loss: never emittable

This class covers `ScalarTrace.lost` and the OperationalError `accounting` cause.

- `ScalarWork::check` and `ScalarWork::operation` (PP:2310–2349) set `lost` only when `checks` or `entered` fails `checked_add(1)`. The retained counter is then exactly u64::MAX.
- The same retained-prefix and fallback rules apply (C3:233–236).

### C. Product work-status faults: representable, but none of the corpus uses is faithful

This class covers the ProductFailure `work_accounting{fault}` cause, the values cause `work_accounting`, and the SectionError `accounting` cause.

- **The fault comes from a trace the receipt carries.**
  - ProductFailure `Cause::Accounting(f)` is raised from the owning trace's status: FC:358–379, 428–436 and 1580 (`work.status().fault()`). FC:208 maps it to `work_accounting`.
  - NumericWork does the same (FK product_certificate.rs:186–197, 200–206, 222).
  - SectionPreparationError::Accounting comes from a non-exact status (FK product_certificate.rs:678), or from more conversion entries than the fixed outcome array holds (688).
  - So a faithful receipt must show the fault in that trace: a Count `unavailable{fault}` or a non-exact `sticky_status`. Every corpus use below leaves all statuses `exact`, so none is faithful as built.
- **Overflow** (FK work.rs:80–104) needs a WorkTotal amount beyond u64.
  - For counters that step by 1 inside NumericWork::begin, the overflowing value is never stored. The entry stays exactly u64::MAX (FK product_certificate.rs:190–197), which is unsafe, so the receipt falls back.
  - Accumulating totals such as `visits` (FC:358–364) store the overflow as unavailable, which is safe. But reaching 2^64 units without some correlated exact counter exceeding 2^53 is not established, and there is no producer witness.
  - Verdict: overflow is not established as emittable.
- **Inconsistent** leaves amount 0 with the fault (FK work.rs:105–113, 170–175), which is representable with safe values. It arises only from failed invariants:
  - a remainder underflow or a cross-stream delta (FK work.rs:105–113, 170–175);
  - explicit checks: FK product_certificate.rs:297, FC:507, source_residual.rs:228, wide_sum.rs:348–377, wide/multi.rs:1078, directed/certificate.rs:52;
  - SectionError at FK product_certificate.rs:688.

  A correct producer never reaches these. It is emittable under C3 only as a producer-defect state, not as a faithful trigger.

### D. Storage (allocator refusal): emittable

This class covers CaptureError `storage{detail}` and ProductFailure `storage`.

- **Native sites:**
  - CaptureError: PP:522 ("observation text"), 778 ("support vector"), 2941 ("adapter vector"), 2962 ("identity copy"), 2975 ("adapter growth"), 3086 ("prepared vector"), 3096 ("prepared string");
  - ProductFailure: FC:443–451 (`reserve` → `Cause::Storage`).
- **Why it is emittable:** a refusal needs no large counter, and any correct producer can hit it under memory pressure. It fits the standing synthetic-trigger rule (a resource fault) and C3's typed `storage` variants (C3:270–275).

## 2. Every corpus entry with an accounting trigger

I applied each entry to its base in memory and scanned every product attempt for these triggers:
- `adapter.fault`;
- an `accounting` or `work_accounting` cause;
- `lost` set to true;
- an unavailable Count;
- a non-exact `sticky_status`.

That gives 2 bases, 13 must-pass entries and 24 mutations. Kernel-terminal WorkAccounting (06c) is out of scope.

### Bases (both class A, not emittable)

| Base | Trigger | Verdict | Proposed replacement |
|---|---|---|---|
| F `two_case_facade_after_certificate_synthetic` | case 1: `capture{accounting{map_write}}` and `adapter.fault` `map_write`, at the commit's MapWrite after observables and G5a (PP:3548–3549) | Not emittable. Only adapter accounting can fail at that point. | **F′:** the post-certificate verdict-copy allocation is refused. The result is `capture{cause:{kind:"storage",detail:"adapter vector"}}` (PP:3362–3365 → 2940–2941), with `adapter.fault` null. Stage shape: certificate completed and its check passed; observables and G5a not entered. This is a new base that changes the stage shape, so recheck every derived entry when it is built. |
| P `two_case_preparation_failure_synthetic` | case 1: preparation `capture{accounting{rust_capacity_bytes}}` (PP:3089) | Not emittable | **P′:** the same `prepared_reserve` call is refused one step earlier, giving `capture{storage{"prepared vector"}}` (PP:3086), with `adapter.fault` null. The F1 prefix-table row is unchanged. |

### Must-pass entries (13 with accounting triggers; the two lane `storage` entries are listed for contrast)

| Entry | Base | Trigger (class) | Verdict | Replacement |
|---|---|---|---|---|
| `prefix_captured` | P | `capture accounting{allocation_request}` (A) | not emittable | `capture{storage{"identity copy"}}`, the same `copy()` call refused at its reserve (PP:2953–2962); fault null |
| `prefix_unequal_helper_new` | P | `accounting{requested_copy_bytes}` at `prepared_string` (A) | not emittable | `storage{"prepared string"}` at PP:3096, the same call |
| `prefix_after_new_evaluator` | P | `accounting{map_write}` after the new evaluator (A) | not emittable | **Retire.** After the evaluator, only MapWrite accounting can fail before `preparations.push` (PP:3241–3245). The F1 row stays covered by the contract but has no witness. |
| `prefix_helper_refused` | P | SectionError `accounting`; member refused `accounting` (C) | not faithful (statuses exact; 678/688 need an overflow or a broken invariant) | **Defer.** A faithful refusal (`invalid_geometry` or `primitive_range`) needs a different member input. This is now expressible with `invocation_edits`, but it needs a producer-derived refusal, so I did not approximate it. |
| `prefix_unattached_old_operand_attested` | P | inherits P's adapter fault (A) | not emittable as based | Faithful once rebased on P′ (its edits only change the old operand) |
| `cert_failed_before_summary` | F | proof `work_accounting{overflow}` (C); fault cleared | not faithful | Proof cause `{kind:"storage"}` from a certify-stage reserve before summary coverage forms. FK `certify` reserves at FC:955–1045, and coverage forms at FC:1195. The exact reserve and the `certify_final`→`certify` chain need confirming at build. |
| `cert_failed_after_summary_accounting` | F | same (C) | not faithful | Same, but only if a reserve follows FC:1195. I found none in this box; if none exists, **retire**. |
| `values_failed_separate_completion` | F | values `work_accounting{overflow}` (C) plus F's adapter fault (A) | not emittable | Values cause `{kind:"storage"}` from `complete_maxima`'s `reserve::<bool>` (FC:1644, via FC:443–451); fault null; completion `separate_failure` unchanged |
| `maxima_abandoned` | F | abandoned `accounting{map_write}` (A) | not emittable | Abandoned cause `storage{"adapter vector"}`; `prepared_maxima` allocates via `adapter.reserve` (PP:3388–3442) |
| `aliases_abandoned` | F | same (A) | not emittable | Abandoned cause `storage{"identity copy"}`; `prepared_alias` copies (PP:3443–3455 → 2962) |
| `bind_rows_abandoned` | F | same (A) | not emittable | Abandoned cause `storage{"adapter vector"}`; `bind_rows_view` reserves (PP:1587–1660) |
| `observables_failed_after_certificate` | F | observable `accounting{map_write}` (A) | not emittable | **Retire or defer.** `observables_view` (PP:1822–1900) allocates nothing, so no storage replacement exists; its other causes depend on the data and need a producer witness. |
| `g5a_failed_after_certificate` | F | G5a `accounting{map_write}` (A) | not emittable | **Retire or defer.** `g5a` (PP:2522–2640) allocates nothing, and G5aError has no storage variant (C3:276). |
| `lane_k_failed`, `lane_source_failed` | F | lane `storage` (D); fault cleared | faithful | none needed |

### Mutations (24)

- **Seventeen pins carrying F's or P's class-A trigger** (two, the `maxima_abandoned_*` pair, restate it as their own cause): `unavailable_*` (8, at G3 or G5a), `cross_case_gate_order_unavailable`, `native_stage_disagrees_with_run`, `certificate_stage_check_disagree`, `stage_entered_after_failure`, `maxima_abandoned_with_coverage`, `maxima_abandoned_separate_failure`, and three idle mutations.
  - Their pinned defects are unrelated to the trigger, so each pin stays valid.
  - The base must be rebased to F′ or P′, and each expectation rechecked at build. F′ changes the stage shape, which could move the stage-sequence pins.
- **Five with their own accounting cause:** `cert_failed_before_summary_g5a_passed`, `certificate_check_wrong_wrapper` (class C), `prefix_captured_with_members` (A, G3), `refused_member_conversion_kind_bits` (C, PA), and `prefix_attached_old_input_unbound` (C, G8).
  - Each pinned defect is P2 or earlier, or at G3, so it would still be the first failure under the reader rules proposed below (which sit at P4).
  - For faithfulness, move each onto its sibling's replacement cause.
- **Two that set `lost` on a Ready attempt:** `product_work_only` (WORK) and `coverage_null_and_product_work` (PA).
  - Their pinned defect is C3:232, so they are valid.
  - They are also class B (not emittable), which agrees with what they pin.

### Proposed reader rules (for ROOT; none applied)

All three sit at G5 P4 and use `RETAINED_PRECISION_WORK_MISMATCH`, per the C3:304 work/status equations:
- **R1:** `adapter.fault ≠ null` rejects (class A).
- **R2:** any `ScalarTrace.lost = true` rejects (class B).
- **R3:** a `work_accounting{fault}` cause requires the owning trace's status to contain that fault (class C; FC:358–379, 1580).

R1 and R2 also make the G3 and G5a pins on F and P fail at G5 first. So rebase F and P before adopting them.

## 3. Tightened sibling of `idle_budget_below_invocation_limit` (proposed; not built)

**The 06b entry does not discriminate.** It keeps case 1's source in group 0 while setting group null.
- A reader without the exhaustion rule still fails it at the C5 group partition (`_g5_native`, ATTEMPT).
- In-memory probe: with the exhaustion clause removed from the current reader, it still gives G5 ATTEMPT.

**Proposed entry:** `idle_budget_not_exhausted_no_group`, on base F, `rehash:"all"`. Paths are under `retained_precision.body`:
- `cases[1].run.records` = `[]`
- `cases[1].run.attempts` = `[]`
- `cases[1].run.case_charge` = 0
- `cases[1].run.invocation_increment` = 0
- `cases[1].run.invocation_after` = 17
- `calls[0].invocation_after` = 17
- `work.charged` = 17
- `cases[1].run.origin.group` = null
- `cases[1].run.kernel_terminal` = `{"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}}`
- `groups[0].source_refs` = `[0]`
- `cases[1].run.cache_before` = `[]`
- `cases[1].run.cache_after` = `[]`

These are `idle_work_accounting_run`'s edits with the Budget(invocation) terminal; 17 is case 1's `invocation_before`.

**Expected outcome:** G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`, from the exhaustion rule (`invocation_before` 17 < Li = 60,000,000,000; `_g5_schedule`).

**In-memory probe:**

| Reader | Result |
|---|---|
| current reader | ATTEMPT, at the exhaustion check |
| same reader with the exhaustion clause removed | G5 PRODUCT_ATTEMPT, from case 1's covered proof (`_g5_coverage`) |

So the entry fails for the intended reason, and a reader without the rule gives a different code. Keep the 06b entry byte-identical, or retire it once the sibling is in.
