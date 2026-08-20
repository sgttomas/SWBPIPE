# Work Graph v1

- SelectionAuthority: `AGENT_0`
- Posture: `MIXED`
- Package: `PKG-09`
- Deliverable: `DEL-09-04`
- Concurrency: none; one product node

| Node | Role / method | Depends on | Write owner | Expected return | Fan-in gate |
|---|---|---|---|---|---|
| `N1-IMPLEMENT` | TASK + `software-bounded-implementation` | accepted activation | implementation child | fixture-backed Playwright workflow evidence, necessary bounded repairs, docs/state delta, focused results, containment | manager validates objective, scope, and checks |
| `N1-REVIEW` | fresh TASK + `software-code-review`, read-only | frozen integrated N1 diff | none | 100% diff review with terminal verdict | PASS with zero actionable findings |
| `N1-HOST` | manager host-capability execution | accepted implementation and any required review | evidence only | both registered viewport projects pass | exit 0, no skipped viewport |

Edges: `N1-IMPLEMENT -> N1-REVIEW -> N1-HOST`. CHANGE commit and committed-head DEC-025 proof are downstream Agent-0 fan-in stages, not additional product nodes.

Escalate only for scope/authority changes, owner-gated judgments, host execution refusal, or a defect outside N1. Discoveries outside N1 are registered in the handoff and do not become nodes.
