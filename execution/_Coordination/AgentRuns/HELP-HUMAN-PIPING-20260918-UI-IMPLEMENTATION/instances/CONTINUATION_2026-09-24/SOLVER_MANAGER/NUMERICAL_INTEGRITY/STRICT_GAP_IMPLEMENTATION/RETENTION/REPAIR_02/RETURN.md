# RET-01 repair02 — two-stage reservation

**Frozen source repair for same-reviewer backcheck; no execution clearance or runtime pass claimed.** TASK Type2 `/root/solver_manager/exact_gap_kernel`, actual parent `/root/solver_manager`, delegated-harness-native. Full TASK was reread; Root/Piping/LOOP instruction hashes remain recorded. Work waited until the retention REPAIR_01_BACKCHECK was sealed, then read its full RETURN (`805c1819bc8252a23b7b790b242fd570e2de9cd407515acf86b625d99eb6f1c0`). Repair01 and its failed backcheck remain unchanged.

Only nonlinear_integration/src/structural_adapter.rs and evidence in this directory changed. The exact-boundary kernel remains `6801610d0bb515eab33603163423812b81bf94946d8c23b99c127b655d996a7f`. Before/frozen adapter, full delta, origins and output hashes are under `_run_records`. No arithmetic, original oracle, 1e-9 criterion, source identity, private ownership, public caller/schema/manifest, nonpassing method status, Git, Cargo/build, native or Node change/execution occurred.

The complete size estimate introduced by repair01 is preserved. Its accounting is now explicitly two-stage:

1. Compute the fixed/container prefix using only the fixed number of O(1) length fields and checked arithmetic. If it does not fit, no dynamic traversal begins: charged=0, rejected=the denied prefix reservation.
2. Once the full prefix fits, retain it as an accepted charge before visiting label/error/ID lengths. This conservative prefix covers the complete sizing walk even if it stops early, plus fixed/container comparisons and contact-vector construction.
3. Accumulate the separate dynamic-byte comparison reservation against limit−prefix. A late size/overflow denial returns the accepted prefix in charged and only the denied dynamic remainder in rejected. No string contents have been compared. Overflow uses the existing explicitly declared usize::MAX saturated rejected-size marker.
4. Full admission returns prefix+dynamic as before. summaries_match runs only afterward, and nested exact replay receives limit−(prefix+dynamic). All later failures retain the accepted charges. Successful reservation magnitudes are unchanged by this accounting repair.

The maintained long-ID test still rejects the historical1088 budget before sizing with zero charge. Its full-comparison-charge-minus-one path now requires positive prefix charge and charged+rejected equal to the full comparison reservation. Its late4096-byte nested-error denial requires that same accepted prefix and a rejected remainder exceeding the remaining byte budget. Sufficient-budget success, one-below-total child-budget failure and short/long reservation growth remain. Only newly authored nonprotected work assertions changed; mechanical expected values are untouched.

Rustfmt exited0 and source/diff/hash inspection completed. Tests remain unexecuted under the parent's lane hold. The next step is the same independent backcheck, then parent-owned focused execution with actual recorded ledgers. No broader NGR02, storage/replay-wire, product or Current completion is implied.
