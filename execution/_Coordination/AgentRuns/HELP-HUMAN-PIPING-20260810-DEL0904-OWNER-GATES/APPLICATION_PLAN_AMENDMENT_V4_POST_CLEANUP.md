# Application Plan Amendment V4 — Post-Cleanup Verification

- RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- Parent: `HELP_HUMAN` / Agent 0
- Manager: `WORKING_ITEMS` / Agent 1
- Accepted base and HEAD: `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
- Scope effect: none; this amendment resumes the already ruled application
  after separately authorized exact cleanup.

## Basis

The C-B, V-D, and MR-A author returns are frozen. O-B and the bounded
DEL-09-04 state fan-in are present. Separate managed cleanup children removed
only the exact V-D Cargo lockfile and the three exact first-verifier Cargo
target roots. Manager backcheck finds all four absent, their parents retained,
the ignored inventory empty, the index empty, and nonignored state unchanged.

The first application verifier was interrupted before WORKING_ITEMS accepted
a terminal child return. Its on-disk records are preserved as attempt evidence
but are not a terminal verification basis.

## Resumed node

Dispatch exactly one fresh non-delegating ephemeral Agent 2,
`A2-APPLICATION-VERIFY-POST-CLEANUP`. It may write only its sealed
`RETURN.md` and `STATUS.json`. It must start from zero ignored drift and must
send every temporary, build, target, bytecode, and cache output to a unique
directory outside the repository. Task Management federation and every Git
mutation are prohibited.

The verifier closes only if the entire ruled application, exact containment,
all non-effects, current-main caveat, and terminal zero ignored drift pass
without repository-side effects. Any new side effect or semantic defect is a
terminal hold; the verifier may not repair or delete it.
