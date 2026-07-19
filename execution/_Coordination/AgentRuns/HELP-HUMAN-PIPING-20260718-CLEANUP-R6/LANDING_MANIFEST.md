# Cleanup R6 Landing Manifest

**Status:** `COMMIT-SAFE / READY_FOR_ATOMIC_GIT_LANDING`

## Scope

- this complete Cleanup R6 AgentRuns directory;
- `loop/WORKPLAN_2026-07-18b_piping_loop.md`, byte-identical to the reviewed
  candidate;
- `tools/release/run_evidence_sweep.py`;
- `tools/release/check_release_readiness.py`;
- `apps/desktop/scripts/build-wasm-engine.mjs`; and
- the two focused Python test modules for those tools.

`LOOP_INIT.md`, prior workplans, decision records/codifications, receipts,
app-dev, Shared-Block v1, and every DEL-09-04/R3 surface are excluded.

## Preconditions

- first independent verifier BLOCK preserved;
- corrected independent V2 verifier `COMMIT-SAFE`;
- 504 piping tests pass;
- receipt validator passes;
- root self-check completed with only pre-existing findings outside this
  tranche;
- focused Python tests, Python compilation, Node syntax check, and
  `git diff --check` pass; and
- local/offline prerequisite preflight passes without installation/download.

After this tranche lands on a clean commit, run the registered evidence sweep
once outside DEL-09-04. Only a passing commit-bound sweep permits the cleanup
receipt to be appended. No reproduction or acceptance effect follows.
