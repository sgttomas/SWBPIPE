# RV68 local accounting observations

These observations apply to the frozen FIRST_AMENDED source. They were sent to
ROOT while I51 was completing C4. They are local prerequisites under the selected
contract, not demands for public debit/RSS/caller qualification or exact pricing of
named auxiliary library internals.

1. **Projection conversion entries/outcomes (RV68-3):** final_case.rs project_hull
   executes `to_binary64()` with no separate entered conversion count/outcome.
   Final error comparisons remain sound, but the actual conversion history is not
   retained, particularly the nonzero-to-zero Underflow branch.
2. **Verdict allocation overlap:** row_scales runs twice; replacement allocation
   for `spent.verdicts` precedes drop of its original Q-capacity buffer. Preserve
   both successful reserves and failure prefixes/overlap, or remove the redundant
   first/second reserve. A single overwritten capacity slot is insufficient as
   the concrete full-route allocation account.
3. **Two retained data mask capacities:** LaneReadouts moves SourceBridgeView's
   `Vec<bool>` into each lane. ResidualWork's nine recorded capacities cover
   center, eta, two residuals, alpha, epsilon, midpoint, rows and reaction;
   SourceBridgeViewWork has only visits/f64 operations. No field records the two
   actual data capacities before final drop. The existing private data_capacity
   accessor provides the narrow extraction seam without adaptive algorithm edits.
4. **First-lane failure correction total:** source_correction_calls starts with
   `self.native.as_ref().map(...)`. If K executes its correction and fails later,
   native_k is Some and native is None; the aggregate accessor returns None rather
   than the entered K count. Native_k's detailed work is retained internally, so
   this is misleading aggregate/capture reporting, not complete work loss. Sum
   whichever lane records exist and retain None only when neither exists; verify
   a first-lane post-correction refusal prefix.

The REENTRY_BOUNDARY source already removes ordinary support/G5a temporary Vec
collections, enters maximum midpoint operations and Number constructions, and
makes diagnostic formatting test-only. That bounded progress was inspected in
REENTRY_BACKCHECK.md. The final measured owner/capacity/prefix schedule and its
exact source-associated controls remain pending.

## Narrow failed-view dependency consultation

At 2026-10-03T12:25:13.989736+00:00, ROOT requested review of a proposed exact scope extension: two usize capacity fields in SourceBridgeViewWork, assigned immediately after the existing prescribed/data Vec allocations, residual propagation before success/error matching, and only ..Default::default() in the two existing source_bridge_tests literals. Frozen adaptive source shows failures after each allocation; these fields close a real prefix gap. Immutable symbol search confirms exactly the two test literals at 591/600. No objection was returned for that exact scope, with numeric/allocation behavior unchanged and affected failure/layout controls required. This consultation itself grants no edits; ROOT owns the exact grant and final diff check.
