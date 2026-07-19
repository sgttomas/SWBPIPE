# HELPS_HUMANS Orchestration Plan — DEL-09-04 R8 Git Exception R9

## Objective

Codify the human owner's 2026-07-19 adoption by reference of a new,
case-specific R8 raw-evidence whitespace exception, then return the bounded
closeout state to HELP_HUMAN and CHANGE. This run is managed by HELPS_HUMANS
under HELP_HUMAN.

## Accepted authority and frozen state

- Human owner response, verbatim: `You may proceed accordingly.`
- Context: HELP_HUMAN had explained that D-53/`DEC-086` cannot be reused, and
  proposed a new R8-specific exception bound to the sole whitespace finding,
  raw/checksum hashes, exact staged inventory, truthful exit `2`, and all
  preserved terminal-FAIL boundaries.
- Canonical SHA-256 of the exact 28 UTF-8 bytes above, without newline:
  `91bcd18370d6004c9d9c4a2b7b7de1fc65b5ad71078c9ccb6bbdf854d34cd54d`.
- Branch: `codex/piping-del0904-clean-repro-20260718-r8`.
- HEAD/source commit:
  `89a93d7ca21d64c57cc344955d17deef709fd685`.
- The exact 93-file staged R8 inventory is the pre-governance closeout
  tranche. No file may be staged, unstaged, or committed by this run.

## Work graph

1. Freeze the governance brief and dispatch exactly one serialized,
   read-only ephemeral Agent 2 verifier under the TASK base contract.
2. Require that verifier to independently recompute every exception predicate
   and return `COMMIT-SAFE` or `BLOCK` without delegation.
3. On `COMMIT-SAFE` only, write D-55, its register row, `DEC-088`, the minimum
   R8 manager RETURN/HANDOFF/STATUS annotations, and this R9 managed record.
4. Re-run deterministic validation and exact protected-state checks. Any
   boundary drift changes the closeout disposition to `HOLD`.

## Write scope

Only:

- `execution/_Coordination/_DECISIONS/D-55_r8_terminal_fail_raw_evidence_whitespace_exception.md`;
- the D-55 row in `execution/_Coordination/_DECISIONS/_REGISTER.md`;
- the `DEC-088` row in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12;
- R8 manager `RETURN.md`, `HANDOFF_STATE.md`, and `STATUS.json`;
- this R9 managed record.

## Exclusions

Do not modify the R8 bundle or sweep; rerun reproduction, tests, or sweep;
repair harness baselines; append Receipt 57; change DEL-09-04 status, memory,
lifecycle, or Remaining; alter R3/R7/P1; accept reproduction; promote evidence;
advance stage/lifecycle/release; push, merge, publish, invoke a prover, or cause
external effect. No S5 review applies because this is a case-specific owner
ruling, not an instrument amendment.
