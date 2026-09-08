# M9 work graph V1

Status: FROZEN_BEFORE_DISPATCH. Selection authority: parent launch plus human-selected recommended M node. Posture: MIXED.

| Node | Owner | Depends on | Writes | Return / gate |
|---|---|---|---|---|
| M9-I | WORKING_ITEMS M9 | C0 | M9 controls and DEL-09-01 run record | Freeze exact fixture identity, case boundary, method independence and acceptance checks. |
| M9-C1 | Ephemeral Agent 2 | M9-I | `instances/M9/children/C1/**` | Independent 3D frame/consistent-load/contact/friction derivation and review; no production comparison; no delegation. |
| M9-A | WORKING_ITEMS M9 | M9-I | M9 evidence and DEL-09-01 run record | Author standalone calculation, frictionless controls and frozen actual expected results without production output. |
| M9-F | WORKING_ITEMS M9 | M9-C1, M9-A | M9 return/handoff and DEL-09-01 run record | Validate child independence and numerical agreement, then compare frozen reference to production if a lawful exact route exists; report errors and retained forks. |

M9-C1 and M9-A may run concurrently because their write targets are disjoint. M9-F is the sole integration owner. There are no cross-package writes. The held DAG unit/kernel/consumer rows are context and remain unsatisfied by this investigation.
