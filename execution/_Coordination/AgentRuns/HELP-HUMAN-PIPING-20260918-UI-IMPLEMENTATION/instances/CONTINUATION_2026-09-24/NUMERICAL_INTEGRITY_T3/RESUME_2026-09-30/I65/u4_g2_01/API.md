# The U3/U4 interface at `retained_memory.rs` (D-5)

**Under D-5, U4 owns `PP/retained_memory.rs` and U3 owns the dispatch and the permit call sites. I61 is the single integration owner for `PP/lib.rs`.**

This file proposes the interface between them so both can proceed: U3 under design-to-budget, U4 to G5. It is a proposal for review, not code. The existing signatures stay as they are; one entry point and two gate checks are added.

## 1. Kept as they are

These exist at NUM `a2c26cc885` (PP/retained_memory.rs):
- `Entry::{Direct, Headless}` (:308–311);
- `assess(capture, request, entry) -> RetainedAdmissionReport` (:312–343), which keeps its census role;
- `RetainedAdmissionReport` with `required_unknown_terms()` and `census_complete()` (:269–295). The unknown-terms list shrinks to empty only when G6 qualifies;
- `ProfileStatus::{Missing, Stale}` (:230–234). G5 adds `Registered`;
- `CapturePermit` stays `pub(super)` and keeps a `&'static RegisteredProfile` (:299–302). No public constructor and no test values exist in maintained code (decision 7).

## 2. Added by U4 (G5)

```rust
// G-A: census + D1 predicate + build status + cap-priced bound check, allocation-free.
pub(super) fn admit(capture: &CapturedInvocation, request: &LinearStaticPreviewRequest,
                    entry: Entry<'_>) -> Result<CapturePermit, RetainedAdmissionReport>;

impl CapturePermit {
    /// R from STACK_PLAN.md; the dispatch spawns the scoped thread with it.
    pub(super) fn reserved_stack_bytes(&self) -> usize;
    /// G-B, immediately before the late old-source capture (I51 COMPOSITION §2).
    pub(super) fn check_late(&self, facts: &LateFacts) -> Result<(), PhaseRefusal>;
    /// G-C, after the complete ordinary owner returns.
    pub(super) fn check_complete(&self, facts: &CompleteFacts) -> Result<(), PhaseRefusal>;
    /// Design-to-budget figures U1/U3 must meet (set by G4): precommit reader, staging, transfer.
    pub(super) fn budgets(&self) -> &'static PhaseBudgets;
}
```

**The supporting types.**
- `LateFacts` and `CompleteFacts` are borrowed, allocation-free fact records read at the gate site: lengths and capacities of the live ordinary owners named in I51 COMPOSITION §2. G3 fixes their exact fields. For example, `CompleteFacts` carries the result-row count against P_final and the diagnostics count, plus text bytes and capacities against the G3 text budget.
- `PhaseRefusal { gate, fact, observed, cap }` is the refusal record.
- `admit`'s refusal is the existing report, extended with the first failing D1 clause (DOMAIN.md §3).

## 3. Owned by U3

U3 (I61) writes all of the following in `PP/lib.rs`:
1. **At the retained dispatch** (PP/lib.rs:2199–2229), call `admit`. On `Err` keep today's ordinary route exactly.
2. **On `Ok`**, run the observed ordinary run and every W1 phase on a scoped thread with `reserved_stack_bytes()`. A spawn failure falls back as in STACK_PLAN.md §1.
3. **Call `check_late`** at the late old-source capture site, and **`check_complete`** after the ordinary owner returns. On `Err`, skip W1 as I51's gates prescribe, never rerun or block the ordinary solve, and keep the ordinary bytes.
4. **Map each refusal** to the precondition kind in DOMAIN.md §3, and decide the public `RETAINED_PRECISION_UNAVAILABLE` behaviour under C1 §2.
5. **Meet `budgets()`.** U1/U3 reviews check conformance, G4 records it, and G5's allocation-challenge tests test it.

## 4. Not in this interface

- **Headless** (outside D1, per D-2).
- **Native callers** (excluded by construction; plan §4).
- **Any test-only permit** in maintained code.
