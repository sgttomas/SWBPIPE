# Orchestration Plan — DEC-093 CI surface-4 tooling

- RunID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- Version: `1`
- SelectionAuthority: `HUMAN` (standing development-pressure steer applied to live ruled work)
- Posture: `TERMINAL_FAN_OUT_IN`
- ExaminedThrough: `219f695d348f1d83ba904ef4dd38781636b423a6`
- Branch: `codex/piping-dec093-ci-surface4`
- CurrentStage: `R5`
- AcceptedAuthority: `D-65`; `DEC-093`; Receipt 114; DEL-10-04 `## Remaining`

## Work graph

N1 is the only selected node. One PKG-10 WORKING_ITEMS instance owns the
bounded DEL-10-04 implementation. No concurrent node is selected and no shared
write occurs before fan-in. Integrated review and Git closeout follow the
validated WORKING_ITEMS return.

## Fan-in gate

Accept N1 only when its implementation and focused tests pass, write
containment is clean, DEL-10-04 state is reconciled, and the manager returns
the exact commands and evidence still required for the committed-HEAD DEC-025,
repository self-check, and full practitioner-harness closeout gates.

## Human decision points

None inside the adopted D-65/DEC-093 implementation fence. Any scope expansion,
release/publication act, lifecycle transition, new evidence semantics, or
macOS-only CI substitution returns to the owner and does not block lawful N1
work.
