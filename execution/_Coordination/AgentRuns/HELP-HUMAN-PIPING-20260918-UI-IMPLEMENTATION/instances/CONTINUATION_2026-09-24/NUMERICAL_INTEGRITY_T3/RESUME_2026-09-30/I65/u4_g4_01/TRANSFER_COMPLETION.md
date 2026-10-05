# T18 (transfer, fallback, the refusal reserve) and T19 (Direct completion), and the budgets U3 must meet

**Basis:** NUM `b1f80234dc`, where U3 grant 1 is committed (`bee3dc07ca`), and the ROOT rulings on U3: carrier A, notice N1 and locks R-3.

**This is design-to-budget.** Each budget below is a cap-priced bound at the D1 caps. U3's committed code is checked against it where it exists; grant 1b and grant 2 must meet the rest.

## 1. T18.1: the fallback copy (D-b), as U3 implements it

Experiment 03 kept a clone of the ordinary envelope for the fallback, about 68–69 KB serialized at the milestone. U3 inverts that:
- the **ordinary owner is never mutated**;
- `FrozenCandidate::staged_envelope` (retained_product.rs:3788–3792) clones it and applies the frozen overlay to the copy;
- the serializer reads the copy (`serialize_frozen`, retained_wire.rs:1474);
- the copy is dropped right after serialization (lib.rs:2976–2978).

So the "fallback copy" is the **staged typed copy**, live only during T16 (phase W3).

**The budget:** `MechanicsEnvelope::clone` with exact capacities:

| Part | Formula |
|---|---|
| Rows | s(ResultItem)·P + P·Text(row) |
| Diagnostics | s(Diagnostic)·D_env + Text(diag_env) |
| Preview evidence tree | as G3's PREVIEW |
| Fixed members | ≤ 64 KiB |
| **Total** | **105,778,121 B at the caps**; 4,912,685 B at the milestone's facts |

The overlay replaces numbers and two `LocatedQuantity` aliases. That adds nothing beyond the clone's bound: the alias strings are ids within Text(row).

## 2. T18.2: the refusal reserve (N1; RR "U3 grant 1 verified", R-2)

Once W1 work has run, every fallback appends exactly one info `RETAINED_PRECISION_UNAVAILABLE` diagnostic. Its space is reserved before W1 starts.

**The budget:**
- **The pre-built Diagnostic:**
  - id ≤ 2·(30 + ID + 12);
  - code 30;
  - severity 8;
  - message ≤ 2·(232 + 64): the fixed text plus C1:68's reason and detail token from `ReceiptFailure::check.wire()`;
  - source 24;
  - one affected ref (the case id).
- **The diagnostics `Vec` reserved to D_env + 1:** at most s(Diagnostic)·(PushCap(D_env + 1) − D_env) beyond today's backing.
- **The total** is **757,746 B at the caps** and 14,922 B at the milestone.
- **The moving term:** the old diagnostics backing during that one reserve, ≤ s(Diagnostic)·D_env = 1,734,016 B.

**When the reserve is live.** It is held from the reservation (before preparation) until the transfer, in phases W2 to W5. X2 also carries it, conservatively: on branch X, `retained_w1` returns at its coexistence check (lib.rs:2955), and whether the reservation precedes that check is grant 1b's choice.

**When no W1 work ran** (G-A, G-B, G-C, stack spawn, domain, coexistence), N1 keeps the ordinary bytes exactly. Nothing is appended, and the reserve is released unused.

## 3. The budgets U3 must meet

| # | Budget | Bound at the caps | Status at `b1f80234dc` |
|---|---|---|---|
| B-1 (RV84 S-7) | **Exactly one ordinary run per invocation** | `run_linear_static_preview_observed` runs ≤ 1 time (TEXT `fn_cap`) | **Met by structure.** The Direct dispatch either returns `permitted_dispatch` or runs `ordinary_dispatch` (lib.rs:2256–2263). `permitted_dispatch` runs `ordinary_dispatch` only when the reserved thread did not run (the slot is still `Some`; :2887–2902). `permitted_run` runs either `ordinary_dispatch` (Domain) or its own observed run (:2921–2944). Grant 2 pins it with a test on the permitted path, using `ordinary_dispatch_entered` |
| B-2 | Staged copy | ≤ 105,778,121 B, live only from staging to the end of serialization | met (:2976–2978) |
| B-3 | Successor carrier | one successor Value, ≤ 190,637,407 B, moved into the carrier and the output, never cloned or re-serialized in the facade | met at grant 1 (`RetainedSuccessor(successor)`, :2997). **Carrier A** (`successor()`, `into_publication()`, grant 1b) must hand it out by reference or by move |
| B-4 | Precommit invocation Value | ≤ 15,782,080 B (the raw request's tree plus the mode), dropped right after `validate` | met (:2989–2995) |
| B-5 | Precommit reader | T17 ≤ 1,275,668,333 B at its peak, plus 5,397,696 B of statics | the accepted reader, called once (:2992) |
| B-6 | N1 reserve | ≤ 757,746 B, reserved before W1 starts and consumed by moves only | **grant 1b**: met at `a634ac8b53` (§7) |
| B-7 | Transfer | moves only; **no fallible allocation after the first mutation** (§4) | met at grant 1; grant 1b must preserve it when it adds N1: met at `a634ac8b53` (§7) |
| B-8 | Reserved-stack thread | heap ≤ 8 KiB plus s(output) (illustrative 2,048 B; §5); stack R = 64 MiB | met (:2907–2919) |
| B-9 (RV82 N9) | One parse per invocation | the typed request and the captured invocation come from one `CapturedInvocation::parse` | met: parsed once at :2251 and passed down |
| B-10 | Fallback values | `W1Fallback` allocates nothing new. Every variant holds enum facts or `&'static str`, except `Precommit { gate, code: String }`, which **moves** the reader's already-allocated error code (≤ one error text; it is priced in T08 by the reader's error-path `site_from` rule) | met (:2197–2214) |

## 4. The no-fallible-allocation-after-first-mutation property

**The property.**
1. Every allocation the W1 path needs to finish is made before the first mutation of any owner that the publication returns. Here that means the output's `envelope` and `retained` fields, and the diagnostics `Vec` under N1.
2. After that first mutation, only moves, drops and pushes into already-reserved capacity occur.

So a failure can never leave a half-mutated publication. Rust's allocation failure is an abort, never a recoverable error, so the "fallible" steps are every step that can return `Err`.

**At `b1f80234dc`:**
- The last fallible step is `validate` (:2992). Its `Err` returns `(frozen.into_ordinary(), Err(Precommit{..}))`, which moves the untouched ordinary owner.
- On success, `drop(invocation)` (:2995) and `(frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))` (:2997) are moves.
- `RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) }` (:2942) is a move.
- The thread result moves through `Packet` and `join` (:2909–2913).

**Grant 1b must keep this.**
- The N1 diagnostic is fully built, and the diagnostics `Vec`'s capacity checked at ≥ len + 1, **before** preparation.
- The append is `push` into reserved capacity: no reallocation, no `format!`, no `?`.
- Carrier A's `into_publication()` must be a move.

**A test that would pin this:** a reserve-capacity assertion (`capacity() > len()`) at the append site, under the fault controls.

## 5. T19: Direct completion

**`on_reserved_stack`** (lib.rs:2907–2919) runs `std::thread::scope` and `Builder::stack_size(R).spawn_scoped`. Its heap:
- `Arc<ScopeData>`;
- the thread handle's `Arc<Inner>` (no name is set);
- the result `Packet` `Arc`, which holds the moved output;
- the boxed main closure, which captures references, the `Copy` permit and the mode;
- the child's TLS destructor registrations.

The output (the typed envelope plus the successor carrier) **moves** through the packet and `join`, back to `permitted_dispatch` and then to the caller. The panic path re-raises the original payload (`resume_unwind`, :2912).

**Budget: 8 KiB + s(output) = 10,240 B (illustrative).** The stack is R, counted separately.

**s(output) is illustrative 2,048 B**, not G4's first 1,024. `RetainedPreviewOutput` holds `Option<RetainedAdmissionReport>` inline. That is a `Copy` struct of about 70 words (`BorrowedValueFacts` twice inside the Headless option, `BorrowedRequestFacts`, and the capacity facts). It takes its full size whether it is `Some` or `None`, so it was already in s(output) at `b1f80234dc`. Adding s(MechanicsEnvelope), the retained `Option<Result<…>>` and the packet's discriminants gives about 1.3 KB, rounded up here. G5 measures it.

**Caller-side completion** (Direct only, D-2): `run_linear_static_preview_value_with_retained_direct` returns the output by move. Today's `envelope()` and `into_parts()` are unchanged; Carrier A adds `successor()` and `into_publication()`. **No caller-side serialization is part of the invocation's budget.** The Direct API returns typed owners, and any JSON rendering the caller does afterwards is outside the permit, as it is today for the ordinary output.

## 6. Totals used in the composition

| Item | Caps | Milestone |
|---|---|---|
| T18.1 staged copy | 105,778,121 | 4,912,685 |
| T18.2 N1 reserve | 757,746 | 14,922 |
| T18.3 transfer and fallback | 48 (moves; `W1Fallback`; a precommit code String is moved, not allocated) | 48 |
| T19 | 10,240 | 10,240 |

## 7. U3 grants 1b and 1c against these budgets (NUM `a634ac8b53`)

ROOT merged grants 1b (`4b31bbf23a`) and 1c (`886bef131a`) into NUM at `a634ac8b53` during G4. This section checks those commits against B-1 to B-10.
- **What it does not change:** G4's phase arithmetic stays pinned at `b1f80234dc`.
- **What it adds:** a conformance reading only.
- **What it rests on:** the committed code alone (`git show`). I61's working tree and grant 1d were not read.
- **Line numbers** below are lib.rs at `a634ac8b53` unless named.

| Budget | Grant 1b/1c code | Verdict |
|---|---|---|
| B-1, one ordinary run | `permitted_run` (:2971–3006) now checks Domain, coexistence (:2994) and G-B's refusal before G-C. Each arm returns the one ordinary owner; there is no second run | **met** |
| B-3, Carrier A | `successor()` borrows (:2235); `into_publication()` moves (:2243). `RetainedPublication` derives `Clone`, but nothing in the facade calls it | **met** |
| B-6, N1 reserve | `ReservedNotice::reserve` (:3034–3051) runs after the coexistence and G-B checks and before `prepare_case` (:3103), so it is live in W2–W5 only.<br>**What it reserves:**<br>– it builds the id with `format!` (≤ 2·(30 + ID + 12));<br>– it calls `try_reserve_exact(1)` on the ordinary diagnostics (:3039). Growth is ≤ s(Diagnostic), and the old backing moves (≤ s(Diagnostic)·D_env, the priced moving term);<br>– it reserves the message at 135 + 35 + 25 + 1 = 196 B (:3041; the bound is 592);<br>– code 30, severity 4, source 20 and one case-id ref: each within T18.2's classes.<br>**On X**, `permitted_run` returns at coexistence before `retained_w1`, so X2's reserve term is conservative | **met**: T18.2 = 757,746 B bounds it |
| B-7, no fallible allocation after the first mutation | Every fallback after the reserve goes through `publish` (:3054–3068), which pushes `push_str` into reserved capacity. The detail tokens are at most 25 B. There is a `cfg(test)` assertion of the reserve capacity (:3064), which is §4's suggested test. The reservation itself can fail (`NoticeReservation`, :3104), but that happens before any W1 work and changes only capacity, not bytes. The success path drops the notice (:3152) and moves | **met** |
| B-10, fallback values | The new variants `Staging(StagingFault(&'static str))`, `NoticeReservation` and `PermitUnbound` (:2219–2228) allocate nothing | **met** |
| T18.1, staged copy | `staged_envelope` is now fallible (retained_product.rs:3799). On a `StagingFault`, the partly overlaid copy drops inside it, so its peak is still the clone | **unchanged** |
| T19 | `admit` now returns `(permit, report)`. `permitted_run` keeps `admission: Some(report)` (:3005). The report is `Copy` and inline. The reserved-thread closure captures it by value (about 0.6 KB inside the 8 KiB allowance). s(output) is unchanged by 1b, because the field was already inline (§5) | **unchanged** (10,240 B) |
| G-B and G-C (API_G4.md §3) | The permit is now linear and owned by the observer. G-C reads it with `observer.permit()` (:2999), and G-B with `self.permit.as_ref()` (retained_product.rs:3244–3246). S-6(c)'s `capture: &observer` and `capture: &*self` fields are shared borrows beside those, so the hook change stays a field addition | compatible |

**Text model.** Grant 1b's new strings are the notice's id, code, severity, source, ref and message. They are N1's, priced in T18.2, not in TAV, so nothing is counted twice or omitted. `w1_case_id` (:3072–3078) reads the borrowed raw request without allocating. `carry_test_hooks` is the identity outside tests (:2959).
