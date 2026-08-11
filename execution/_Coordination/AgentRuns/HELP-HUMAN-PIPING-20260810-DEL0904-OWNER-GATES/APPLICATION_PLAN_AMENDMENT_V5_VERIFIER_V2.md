# Application Plan Amendment V5 — Tokenized Post-Cleanup Verifier V2

- RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- Parent: `HELP_HUMAN` / Agent 0
- Manager: `WORKING_ITEMS` / Agent 1
- Accepted base and HEAD: `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
- Scope effect: none; this amendment cures only the verifier-control surface.

## Preserved V1 result

`A2-APPLICATION-VERIFY-POST-CLEANUP` returned terminal `HOLD` because its
manager-authored sealed brief contained two machine-local absolute-path
anchors. The V1 brief, status, and return remain unchanged as process
evidence. V1 independently passed the ruled application semantics and found
zero ignored drift, but its return is not the terminal acceptance basis.

## V2 control-surface cure

Dispatch exactly one fresh non-delegating ephemeral Agent 2,
`A2-APPLICATION-VERIFY-POST-CLEANUP-V2`, under a new sealed brief. The brief
derives `REPO_ROOT` with `git rev-parse --show-toplevel`, derives
`WORKING_ROOT` from that root, and creates any scratch root with `mktemp -d`.
It contains no hard-coded machine-local workspace or scratch path.

Before dispatch, validate the V2 brief itself with the path-anchor validator
in an isolated external validation root, validate candidate whitespace on the
actual V2 instance paths, and confirm the live repository scan introduces no
finding beyond the two frozen V1 findings.

The V2 verifier repeats the full semantic, containment, non-effect,
current-main, and zero-side-effect checks against current bytes. It may write
only its own `RETURN.md` and `STATUS.json`. A new defect or side effect is a
terminal hold without repair or deletion.
