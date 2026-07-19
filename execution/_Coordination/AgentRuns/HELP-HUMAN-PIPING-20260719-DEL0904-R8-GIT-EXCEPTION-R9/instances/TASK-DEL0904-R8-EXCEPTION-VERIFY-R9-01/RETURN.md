# TASK Return — R8 Exception Predicate Verification R9

`RUN_STATUS: SUCCESS`
`Verdict: COMMIT-SAFE`
`ControlSurface: FILE`
`TaskProfile: NONE`
`TaskSkill: NONE`
`ToolPolicyCompliance: PASS`
`WriteAuthorization: NONE — managed read-only runtime record`
`Delegation: none`

The proposed one-off R8 closeout gate may be recorded as
`PASS_WITH_OWNER_EXCEPTION_DEC_088`. This does not turn
`git diff --cached --check` into PASS and does not alter R8's terminal `FAIL`.

## Recomputed predicates

- Branch `codex/piping-del0904-clean-repro-20260718-r8`; HEAD
  `89a93d7ca21d64c57cc344955d17deef709fd685`.
- Exactly 93 staged added paths: 83 bundle, one sweep, one deliverable record,
  eight R8 managed-run files; zero unclassified. Bytewise-sorted,
  newline-delimited path-list SHA-256:
  `9f2e0415cbfdbc815d4af6609b7b12fd7091a5af39fd6f335d07c18d355cc012`.
- `git diff --cached --check`: exit `2`, exactly one finding at the bound R8
  `stdout/cargo_test.txt:42`; no second finding.
- Raw stdout: 2,095 bytes; SHA-256
  `625969f5f2c2dcf8a450818ce1e370e0aa8f991a70b88ee04a515068829e489e`.
- `SHA256SUMS.txt`: 7,617 bytes; SHA-256
  `003db610d9bfee47ea73349c81dae79bc196af8ee56aaeb0fb1b85a54bcf7c77`;
  82 unique valid payload entries, zero malformed/duplicate/missing/extra or
  mismatch, raw binding exactly once.
- Sweep: 4,316 bytes; SHA-256
  `9994c20815dd1d5a64d90cf93f9956f61085b896ed93ec9b07017a1da0d3dd64`;
  exactly five surfaces, all `pass`, overall `pass`.
- Harness evidence: exit `1`, 264 passed, two failed. R8 RETURN/HANDOFF/STATUS
  each preserve terminal `FAIL`.
- DEL-09-04 `_STATUS.md` and MEMORY match HEAD at `e1ed7b6d...` and
  `cee73c9c...`; lifecycle `IN_PROGRESS`; reproduction Remaining item open.
- Receipt ledger matches HEAD at `2ad81234...`; Receipt 56 latest and Receipt
  57 absent.
- Protected aggregates match: R3 run `f530f329...`, R3 bundle `81e9ee96...`,
  R7 run `306eed9a...`, R7 bundle `df565da8...`, P1 run `bcb1324e...`; no
  protected staged, unstaged, or untracked change.
- All 84 staged bundle/sweep paths equal their staged-index bytes; zero
  unstaged tracked paths. Only the five expected R9 governance files existed
  untracked during verification.
- Owner phrase is exactly 28 no-newline UTF-8 bytes; SHA-256
  `91bcd18370d6004c9d9c4a2b7b7de1fc65b5ad71078c9ccb6bbdf854d34cd54d`.

Discrepancies, missing predicates, human rulings, and dependency notes: none.
No file write, stage, unstage, commit, test, reproduction, sweep, network, or
delegation occurred.
