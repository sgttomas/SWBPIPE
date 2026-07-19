# Orchestration Plan — DISCRETION-REFINEMENT-R5

**RunID:** `HELP-HUMAN-PIPING-20260718-DISCRETION-REFINEMENT-R5`
**Agent 0:** HELP_HUMAN
**Integration owner:** HELPS_HUMANS
**Instance:** `HELPS-HUMANS-PIPING-DISCRETION-REFINEMENT-01`
**Posture:** serialized draft, then independent two-node verification fan-out,
then fan-in; repeat S5 was owner-curtailed as unnecessary

## Objective

Record D-54/DEC-087 as a prospective piping-local reasoned-selection
refinement, re-mint a guarded candidate-successor workplan under the
third-lineage rule, preserve all history and DEL-09-04, and return ready for
durable landing after fresh v7 independent local verifiers return
`COMMIT-SAFE`.

## Write Scope

- D-54;
- decision register;
- SOFTWARE_DECOMP DEC-087;
- R5 `WORKPLAN_CANDIDATE_2026-07-18_piping_loop.md`;
- `loop/LOOP_INIT.md` for the owner-selected committed-HEAD-only fail-stop
  loader;
- this R5 AgentRuns directory.

No other path is writable. In particular D-52/DEC-085 and history,
Shared-Block v1, old workplans, receipts, and all DEL-09-04 surfaces are
read-only.

## Fan-In

The first actual S5 review returned `BLOCK`; its return is preserved. Every
subsequent embedded-guard BLOCK, interruption, late return, and tool-error
record remains preserved truthfully. The owner superseded that architecture;
v6 was interrupted without returns. Current architecture isolates the
candidate outside `loop/WORKPLAN_*.md`, makes committed `HEAD` the sole plan
selection/read source, fails stopped before Step 0 with no older-plan fallback,
and uses the closed M1–M12 matrix plus atomic promotion. Fresh v7 semantic and
carry-forward verifiers each returned `COMMIT-SAFE`; deterministic validation
passed. The owner curtailed repeat S5 because no Shared-Block or app-dev change
occurred; the interrupted attempt has no verdict. The run is
`READY_FOR_DURABLE_LANDING`, and operational effect requires durable Git
landing. No refined-discretion exercise occurs here.
