# V1: delta check of S11G_GUARD revision 2.2 (the routing gate)

This resumes V1 under ROOT's **"S11-G ruling 3, revision 1"** (`T3/ROOT_RULINGS_V1.md`). `_COMMON.md` applies.

## Scope: the 2.1 → 2.2 diff only

Check only D1's change to the routing gate (§3.5 item 2), and the tests, forecasts and write-set rows that follow from it. **Do not reopen settled design.** A defect you notice elsewhere is a NOTE, unless it is blocking.

## What to check

1. **Path 2 is resolved.** After 2.2, I5's path-2 model is not refused. Take the model from I5's construction in `<wt>/s11g`, `T3/IMPLEMENTATION/S11G/`. Its guard-fired case is Sensitive, and **no receipt contract change** is needed. Check against the actual receipt code (`source_receipt.rs`, `finalize_for` and `OrdinaryAttempt::wire`) at the S11-G base.
2. **Soundness.** No guard-fired case can publish a Passed value, or a value presented as better than Sensitive, on any route: captured and typed, linear and nonlinear, source-selected or not. The guard's demotion and diagnostic apply whatever the retained-source outcome. The no-op rule and the byte layout are unchanged.
3. **D1's reconciliation with 2.1's SF-3 argument**, and its evidence on whether path 2 is introduced by S11-G or already refuses on main. **Attack it:** construct your own models.
4. **Path 1:** D1's claim that it is or is not removed. If it remains, the characterization test's assertions follow ruling 3's condition (b).
5. **The tests and mutations** that pin the new routing, including one that would reintroduce path 2, and **the forecast** (committed bytes and gate) as D1 states it.

## Return

- Write `T3/REVIEW/S11G_CHECK.md` **delta-2.2**, appended, with a new sha256, in `<wt>/numerics`. Make no Git writes.
- The verdict is **PASS** or **FAIL**, with findings (BLOCKING / SHOULD-FIX / NOTE).
- No cargo, unless the manager grants a slot.
- Send the manager a SendMessage summary.
