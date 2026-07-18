# Sealed Child Brief — DEL-09-04 Clean Reproduction

- RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3`
- ParentInstanceID: `WORKING-ITEMS-PKG09-DEL0904-R3`
- ChildInstanceID: `TASK-DEL0904-CLEAN-REPRO-01`
- PackageID: `PKG-09`
- DeliverableID: `DEL-09-04`
- RequestedBy: `HELP_HUMAN`
- Source commit: `f14fa77518a06f112ae72a8fcce4de0fab958d47`
- Committed candidate SHA-256: `57b9cb228b970ce2d1c3d58f6b888bc57f4b81a5afd6c718dad7124c76b27aed`
- Reproduction Run ID: `REPRO_DEL0904_20260718T215424Z_f14fa77518a`
- Session start: `2026-07-18T21:54:24Z`

## Objective

Execute exactly one actor-neutral local clean-checkout reproduction of the
three DEL-09-04 E1 runner cases under the committed candidate brief. Return a
truthful `PASS`, `FAIL`, or `BLOCKED` with an immutable derivative evidence
bundle. Do not repair any observed defect.

## Accepted Basis

Governance commit `f14fa77518a06f112ae72a8fcce4de0fab958d47`, the committed
candidate brief, DEC-080, DEC-085, approved DAG-007, the documented E1
procedure, `software-workflow.json`, DEL-09-04 status/context/memory, and the
project/root governance named by the candidate. The owner standing approval,
HELPS_HUMANS classification, local verifier, and DEC-083 S5 review were
durably landed before dispatch.

## Execution Contract

Execute candidate §§3–4 and test candidate §6 literally. Work offline and
without installation. Use one local filesystem clone pinned to the source
commit, `CARGO_NET_OFFLINE=true`, and Cargo `--offline`. Capture expected
nonzero runner cases independently. Missing prerequisites produce `BLOCKED`,
not provisioning. Preserve evidence and do not claim acceptance, release,
lifecycle change, prover correlation, or professional reliance.

## Exclusive Durable Write Targets

1. Candidate brief activation/status fields only.
2. `validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/**`.
3. Exactly one tool-emitted `validation/evidence/sweeps/SWEEP_*.json`.
4. DEL-09-04 `_STATUS.md`, `MEMORY.md`, and one new
   `_run_records/WORKING_ITEMS_RUN_*.md`.
5. This managed run tree only.
6. Append-only `loop/LOOP_RECEIPTS.md`, exactly one Receipt-56 on PASS.

No other durable writes are authorized. No commit, push, PR, merge, fetch,
publication, release, lifecycle promotion, prover action, or external state
change is authorized.

## Expected Return and Fan-In Gates

Return terminal disposition, exact changed paths, evidence/sweep paths,
reproduction ID, source and candidate hashes, command exits and predicates,
registered/closeout checks, receipt status, preserved gates, blockers/rerun
triggers, and a handoff for HELP_HUMAN/CHANGE. Fan-in requires exact path
containment, complete evidence, truthful claim calibration, and no excluded
action.

No child delegation is permitted.
