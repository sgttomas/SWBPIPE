# INIT-TASK — DEL-09-04 R8 Exception Predicate Verification R9

## Normalized task contract

- `ScopePath`: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-R8-GIT-EXCEPTION-R9`
- `WorkingRoot`: `projects/chirality-piping`
- `TaskSkill`: none; use the TASK base contract as an ephemeral bounded
  generalist.
- `ApplyEdits`: false.
- `AllowedWriteTargets`: none. The runtime-owned LAUNCH/STATUS/RETURN files are
  the durable record.
- `AllowedTools`: read-only shell commands and Python standard library only.
- `Delegation`: forbidden. Agent 2 must not delegate.
- `ExpectedOutput`: `COMMIT-SAFE` or `BLOCK`, with exact recomputed evidence.

## Objective

Independently verify every predicate of the proposed one-off R8 Git-closeout
exception. Do not rely on the parent manager's asserted measurements. Recompute
from current bytes, Git index, and protected live state.

## Required checks

1. Resolve repository root; prove branch
   `codex/piping-del0904-clean-repro-20260718-r8` and HEAD
   `89a93d7ca21d64c57cc344955d17deef709fd685`.
2. Recompute the staged inventory: exactly 93 paths, all and only the R8
   terminal-FAIL tranche described by the candidate brief and R8 manager
   RETURN (83 fresh bundle files including checksum, one sweep, one
   deliverable run record, and eight R8 managed-run files). Report the
   SHA-256 of the bytewise-sorted newline-delimited staged path list.
3. Run `git diff --cached --check` without masking its status. Confirm exit
   `2` with exactly one finding and exact text:
   `projects/chirality-piping/validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/stdout/cargo_test.txt:42: new blank line at EOF.`
4. Recompute raw stdout size/hash: 2,095 bytes and SHA-256
   `625969f5f2c2dcf8a450818ce1e370e0aa8f991a70b88ee04a515068829e489e`.
5. Recompute `SHA256SUMS.txt` size/hash: 7,617 bytes and SHA-256
   `003db610d9bfee47ea73349c81dae79bc196af8ee56aaeb0fb1b85a54bcf7c77`;
   verify exactly 82 payload entries, every entry, no extras/duplicates, and
   the binding of the raw stdout hash.
6. Recompute sweep size/hash: 4,316 bytes and SHA-256
   `9994c20815dd1d5a64d90cf93f9956f61085b896ed93ec9b07017a1da0d3dd64`;
   parse it and verify all five surfaces pass.
7. Parse the R8 harness evidence and verify exit `1`, `264 passed`, `2 failed`;
   confirm the objective remains terminal `FAIL` in R8 RETURN, HANDOFF, and
   STATUS.
8. Verify DEL-09-04 remains `IN_PROGRESS`, the clean-reproduction Remaining
   item remains open, its MEMORY is unchanged from HEAD, Receipt 56 remains
   the latest ledger entry, and Receipt 57 is absent.
9. Verify no staged or unstaged changes touch the R8 bundle/sweep bytes beyond
   their already staged additions; no change touches R3, R7, P1, DEL-09-04
   `_STATUS.md`/`MEMORY.md`, or `loop/LOOP_RECEIPTS.md`. Report any other
   unstaged path separately; R9 governance files are expected but must not be
   mistaken for the 93-file staged inventory.
10. Recompute the exact 28-byte, no-newline UTF-8 owner phrase
    `You may proceed accordingly.` as SHA-256
    `91bcd18370d6004c9d9c4a2b7b7de1fc65b5ad71078c9ccb6bbdf854d34cd54d`.

Any second whitespace finding, size/hash/count/checksum failure, staged-scope
drift, protected-state drift, receipt/status drift, or inability to prove a
predicate returns `BLOCK`. The command remains truthfully exit `2`; a passing
verdict means only that the proposed closeout gate may be
`PASS_WITH_OWNER_EXCEPTION_DEC_088`.

## Exclusions

Do not write, stage, unstage, commit, mutate bundle/sweep bytes, rerun any
reproduction/test/sweep, repair a baseline, append a receipt, update any
deliverable state, contact the network, delegate, or cause external effect.
