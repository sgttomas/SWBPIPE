# Managed Run Record — DEL-09-04 Clean Reproduction

- RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3`
- RequestedBy: `HELP_HUMAN`
- Manager: `WORKING_ITEMS`
- ParentInstanceID: `WORKING-ITEMS-PKG09-DEL0904-R3`
- ChildInstanceID: `TASK-DEL0904-CLEAN-REPRO-01`
- PackageID: `PKG-09`
- DeliverableID: `DEL-09-04`
- Posture: `TERMINAL_FAN_OUT_IN`
- Session start: `2026-07-18T21:54:24Z`
- Session finish: `2026-07-18T22:08:17Z`
- Status: `FAIL`

The frozen one-node work graph is `WORK_GRAPH.json`; the sealed child brief is
`instances/TASK-DEL0904-CLEAN-REPRO-01/CHILD_BRIEF.md`. This is one serialized
Bash-bearing Agent 2 integration node with exclusive ownership of the sealed
write targets and no concurrency or graph edges.

No token or context telemetry is recorded because the runtime does not expose
it.

## Terminal Disposition

The core clean-checkout reproduction passed its mechanical predicates, but the
complete tranche failed. The mandatory evidence-sweep exited `1` at the
desktop Vitest surface (`npm test` exit `127`) and its internal Cargo log
contains `Updating crates.io index` without the required `--offline` flag.
The absolute offline/no-external-contact condition therefore was not
established and may have been violated. No repair or provisioning occurred.

## Check Summary

| Check | Result |
|---|---|
| Local clone initial/final status and tracked/cached diffs | PASS |
| Fixture generator and scoped fixture diff | PASS |
| Runner exits / exact predicates | PASS (`0/1/1`; 830 result refs) |
| Headless runner Cargo tests | PASS |
| Python runner contract test | PASS |
| `piping-pytest` | PASS (499 tests) |
| `evidence-sweep` | FAIL (exit 1; desktop Vitest exit 127; 3/5 surfaces) |
| Sweep before/after delta | PASS (exactly one new sweep artifact) |
| `harness-pytest` | PASS |
| `harness-self-check` | PASS |
| Claims-language | PASS after one observed wrong-CWD retry |
| Path-anchor | PASS |
| New JSON/JSONL parsing | PASS |
| `git diff --check` | PASS |
| Deterministic changed-path containment | PASS |
| Bundle `SHA256SUMS.txt` creation and `sha256sum -c` verification | PASS (all listed files OK) |
| Receipt validator | NOT RUN; no receipt append is allowed on FAIL |

## Retry Observability

Claims-language attempt 1 was detected at the executor command-path layer:
the root validator was invoked from `WORKING_ROOT`, returned exit `2`, and its
outputs were retained with suffix `wrong_cwd`. Attempt 2 repeated the same
validator arguments from the prescribed `REPO_ROOT` and returned `0`.
Disposition: corrected invocation only; no project repair, and the terminal
FAIL basis remains the evidence-sweep/offline condition.

## State and Handoff

The DEL-09-04 lifecycle and Remaining item are unchanged; no Receipt-56 was
appended. Evidence and exact return are recorded in the derivative bundle and
`instances/TASK-DEL0904-CLEAN-REPRO-01/RETURN.md`. `HANDOFF_STATE.md` records
the rerun trigger and preserved gates. No Git or external closeout action was
performed.
