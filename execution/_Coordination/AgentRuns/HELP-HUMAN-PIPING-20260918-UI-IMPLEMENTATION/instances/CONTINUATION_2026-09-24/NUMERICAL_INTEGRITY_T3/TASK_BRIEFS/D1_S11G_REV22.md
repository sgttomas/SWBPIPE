# D1: S11G_GUARD revision 2.2, the routing-gate delta

This is a design TASK for D1 (resumed). `_COMMON.md` applies. It is commissioned by ROOT under **"S11-G ruling 3, revision 1"** in `T3/ROOT_RULINGS_V1.md`; read that first, with "S11-G implementation: I5 rulings".

## Scope: the routing gate only

Revise `T3/DESIGN_NUMERICS/S11G_GUARD.md` from **revision 2.1** (sha256 `7c052c9e…`) to **2.2**. Change only §3.5 item 2 (routing, `source_eligible … && load_row_finding.is_none()`), and whatever tests, forecasts and write-set rows follow from it. **Everything else in 2.1 is settled. Do not reopen it.** A defect you notice elsewhere is a NOTE, unless it is blocking.

## The problem

I5 reports, from the code on `3d844fea4` (the S11-G branch, which contains main at `72d5ff864`), a **path 2** to the receipt refusal:
1. An already-Sensitive case whose load-row guard fires gets no retained-source attempt, because of the routing gate.
2. It therefore has no receipt entry.
3. `FinalizedSourceBlockReceipt::finalize_for` (`source_receipt.rs:955-1005`) requires an entry for every case when any case is source-selected. So finalization fails, and PP pushes a blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`: the invocation is refused.

I5 reports this as reachable in ordinary models, for example an N05-type case beside a formation-noisy second case. I5's path-2 construction (the model, and file:line evidence) is in `<wt>/s11g` under `T3/IMPLEMENTATION/S11G/` (uncommitted). Read it there; ask the manager if it isn't there yet.

**This conflicts with 2.1 §3.5's SF-3 argument**, that the gate is unreachable end to end because retained-source recovery admits only nodal loads plus eigen terms, on which the guard cannot fire. Reconcile the two:
- Is the gate reached on a case that recovery would otherwise attempt?
- Or is source eligibility, or receipt coverage, decided at invocation level, so that a guard-fired case which is never itself recovery-eligible still breaks coverage?
- **Is path 2 introduced by S11-G, or does the same model already refuse on main (`72d5ff864`) without the guard?** Establish this with evidence (code reading and, if needed, a script or a request to the manager for a cargo slot). Don't presume it.

## What to answer

1. **Why** 2.1 withholds the retained-source attempt from an already-Sensitive case whose load-row guard fires. What the gate protects against, and whether that protection is needed given SF-3.
2. **Whether giving the case the attempt is sound**, with the guard's Sensitive demotion and its diagnostic applied **whatever the attempt's outcome**, so that receipt coverage holds **with no receipt contract change**. Show:
   - that no guard-fired case can publish a value that is Passed, or that is presented as better than Sensitive;
   - that the receipt's `OrdinaryAttempt::wire` equalities hold for such a case;
   - that the no-op rule and the diagnostic byte layout are unchanged.
3. **If that is unsound**, the smallest in-design alternative that avoids refusal. **A receipt contract change is out of scope.**
4. **Path 1** (a Passed case demoted beside a source-selected case, which needs per-case stiffness): does your 2.2 fix also remove it? That is preferred. If not, say so, and path 1 stays a disclosed residual under ruling 3.
5. **Updated tests:**
   - replace or restate T10 and T10b as needed;
   - add a test for path 2's model (no refusal after 2.2, and the guard-fired case Sensitive);
   - add the characterization test for any residual that remains;
   - add the mutations that would reintroduce path 2.
6. **Forecast:** confirm that the committed-byte forecast and the gate forecast of 2.1 are unchanged, or state exactly what changes.

## Return

- Write `S11G_GUARD.md` revision 2.2, with a change log against 2.1 and a new sha256. Put any scripts under `DESIGN_NUMERICS/_run_records/`, with SHA256SUMS.
- Write in `<wt>/numerics` only. Make no Git writes; the manager commits.
- No cargo, unless the manager grants a slot. The order is I3, then I5.
- Send the manager a SendMessage summary: the answers to 1–4, and the file's sha256.
