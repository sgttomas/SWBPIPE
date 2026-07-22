# N5C Return — Fresh attempt-3 terminal verification

**Verdict:** `BLOCK`

Material findings:

1. Python and TypeScript still propagate a record's public basis recursively
   into nested unmetadataed records/leaves, exceeding the exact record-local
   boundary (`route_control.py` around lines 195–202/292–293;
   `redactionExportControls.ts` around lines 743–756/805–813).
2. TypeScript PCF/MBF structural projection uses broad token/suffix matching
   across arbitrary descendant paths rather than exact bounded paths. An
   opaque nested value under a matching key can become public.
3. Report projection infers `invented_public_example` from user-controlled
   provenance text containing “invented” or “cleared”
   (`renderableReportInput.ts` around lines 131–147/240–256), violating the
   no-value-text-inference rule.
4. Exact containment is false: Playwright generated untracked
   `projects/chirality-piping/test-results/.last-run.json` outside candidate
   §6. Attempts 2 and 3 sweeps record `test-results/` in `dirty_paths`, while
   the terminal change-scope evidence omitted it.

Sweep checksums, portability-policy binding, protected/release no-change, and
state/receipt gates otherwise reconcile. No edits, sweep, state, receipt, or
Git effect was performed by N5C. W3 remains held.

