# AMENDMENT 8 — exact sweep-evidence containment reconciliation

Version: 8

Authority: HELP_HUMAN standing direction to amend discoveries within N1;
reviewer 7 terminal `FAIL / ACTIONABLE_FINDINGS`.

Reason: `AMENDMENT_7_SWEEP_PATHS_MISSING_FROM_GRAPH_WRITE_TARGETS`.

Reviewer 7 verified 14/14 frozen hashes, exact membership, exact six-check
profile selection, and 100% substantive coverage. It found no product CSS or
Playwright defect. It rejected containment because Amendment 7 authorized and
the freeze preserved these two exact sweep summaries, while WORK_GRAPH v8 did
not carry their exact paths in `write_targets`:

- `validation/evidence/sweeps/SWEEP_20260820T214752Z_a9b1fbef90f3.json`
- `validation/evidence/sweeps/SWEEP_20260820T214858Z_a9b1fbef90f3.json`

Authorized correction is limited to adding those two exact paths to the graph,
recording reviewer 7's preserved FAIL and this containment-only remediation in
the existing R8/DEL run controls, refreezing the complete adjacent tree, and
dispatching a different fresh read-only non-delegating reviewer 8. Do not
broaden the write root to `validation/**` or another directory. Do not change
product code, tests, either sweep summary, check evidence, proof predicates,
or the commit-bound evidence-sweep handoff. No product/test/package/host rerun
is required because the correction is control metadata only.
