# Agent 2 Return — DEL-09-04 Clean Reproduction

## Disposition

`FAIL`

- Reproduction Run ID: `REPRO_DEL0904_20260718T215424Z_f14fa77518a`
- Source commit: `f14fa77518a06f112ae72a8fcce4de0fab958d47`
- Committed candidate SHA-256: `57b9cb228b970ce2d1c3d58f6b888bc57f4b81a5afd6c718dad7124c76b27aed`
- Bundle:
  `validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/`
- Sweep:
  `validation/evidence/sweeps/SWEEP_20260718T220018Z_f14fa77518a0-dirty.json`

## Execution Results

- Local clone/check-out/detach: exit `0`; exact source commit.
- Initial and final status: empty; tracked and cached diffs: exit `0`.
- Generator: exit `0`; three-fixture scoped diff: exit `0`.
- Solve: exit `0`; completed; request/result/top diagnostics empty; 830
  non-empty result references.
- Validate input: exit `1`; required missing-load-basis diagnostic; no solver
  result.
- Run benchmark: exit `1`; required downstream-payload diagnostic.
- Headless Cargo tests: exit `0` under explicit offline flags.
- Python contract test: exit `0`.

## Registered and Closeout Checks

- `piping-pytest`: exit `0`, 499 passed.
- `evidence-sweep`: exit `1`; desktop Vitest `npm test` exit `127`; 3/5
  surfaces ran. Exactly one sweep artifact was created. Its internal Cargo log
  contains `Updating crates.io index` without the required `--offline` flag,
  so the absolute offline/no-external-contact contract was not established and
  may have been violated.
- `harness-pytest`: exit `0`.
- `harness-self-check`: exit `0`.
- Claims-language: initial wrong-CWD exit `2` retained and classified at the
  executor command-path layer; one repo-root retry exited `0`. Disposition was
  corrected invocation only, not project repair.
- Path-anchor, JSON/JSONL parsing, `git diff --check`, and changed-path
  containment: pass.
- `SHA256SUMS.txt` was generated only after every other bundle file was final;
  `sha256sum -c SHA256SUMS.txt` then returned `0` with every listed file `OK`.
- Receipt: not appended; Receipt-55 remains latest. Receipt validator not run
  because the candidate requires it only after the PASS-only append.

## Exact Durable Changed Paths

1. Candidate activation/status fields:
   `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`.
2. Complete reproduction bundle:
   `validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/**`.
3. Single sweep artifact:
   `validation/evidence/sweeps/SWEEP_20260718T220018Z_f14fa77518a0-dirty.json`.
4. One deliverable run record:
   `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-18_DEL0904_CLEAN_REPRO_FAIL.md`.
5. Managed run tree:
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3/**`.

DEL-09-04 `_STATUS.md`, `MEMORY.md`, and `loop/LOOP_RECEIPTS.md` were not
changed.

## Preserved Gates and Rerun Trigger

The lifecycle remains `IN_PROGRESS`; the clean-checkout Remaining bullet stays
open. No reproduction-result acceptance, evidence promotion, lifecycle/stage/
issuance/release act, prover action, publication/external action, merge, D-45,
or D-06b act occurred. A rerun requires a new run ID after the registered
evidence-sweep prerequisites and its offline posture can satisfy the candidate;
all candidate §8 triggers remain controlling.

Handoff: WORKING_ITEMS should reject PASS closeout, preserve this immutable
failed evidence, and return the blocker/repair need to HELP_HUMAN for a fresh
lawful selection. No CHANGE closeout should commit this as a successful
reproduction claim.
