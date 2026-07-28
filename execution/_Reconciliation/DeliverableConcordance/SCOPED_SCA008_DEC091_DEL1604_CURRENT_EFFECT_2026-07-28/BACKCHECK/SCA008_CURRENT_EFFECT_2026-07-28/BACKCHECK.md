# SCA-008 current-effect backcheck

## Result

`PASS — CURRENT_EFFECT_RECONCILED / CLOSED_WITH_RELIANCE_HOLD`

## Exact accounting

| Population | Expected | Observed | Result |
|---|---:|---:|---|
| Authority rows | 2 | 2 | PASS |
| Claim rows | 2 | 2 | PASS |
| Unique claim IDs | 2 | 2 | PASS |
| In-scope deliverables in Remaining census | 1 | 1 | PASS |
| DEL-16-04 Remaining rows | 3 | 3 | PASS |
| Selectable rows | 0 | 0 | PASS |
| Historical package tracked members | 294 | 294 | PASS |

The scoped census covers 1/1 activated deliverables. D-60 does not activate a
corpus-wide rerun, so other deliverables are intentionally outside this
derivative and receive no new disposition.

## Authority and claims

- `DEC-063` remains `HISTORICAL_ACCEPTED / NOT_CURRENT_RELIANCE`.
- `DEC-091` is `CURRENT_ACCEPTED / GOVERNING_CURRENT_EFFECT`.
- `DEL-16-04-REQ-009` remains
  `PARTIALLY_IMPLEMENTED / MEDIUM / OWNER / NO`.
- `DEL-16-04-DECL-005` remains `ALIGNED / HIGH / NO`.
- App F3 remains necessary.
- The automation-condition mechanism remains unresolved.
- DEL-16-04 remains `IN_PROGRESS` with its exact three Remaining items.

## Preservation

The historical 2026-07-11 package remains at Git tree
`79914bd8b4a9bc183994e5a63746ef0f316ed3a2`, with 294 tracked members and
ordered tree-list SHA-256
`c547becd5eecae05cd6a2a5c64708223c136aa3c147ea55399f6b8cb8ce64f02`.
No historical file was edited.

No product, implementation, runtime, client, dependency, DAG, lifecycle,
repin, release, estimate, schedule, or professional-reliance effect is
created. The reliance hold is the closure condition, not a defect omitted
from closure.
