---
doc_id: R16-DEL0801-ORCHESTRATION-PLAN
doc_kind: coordination.orchestration_plan
status: active_pre_effect
created: 2026-07-22
---

# DEL-08-01 report-package seam — work graph v1

## Activation

- Parent: HELP_HUMAN.
- Manager: WORKING_ITEMS instance `W-PKG08`.
- Package: `PKG-08` — Reporting, Audit, and Reproducibility.
- Selected deliverable: `DEL-08-01` only.
- Frozen Git basis: `8698b0338ac82556fee583dd3f85bb62d0b74f85` on branch
  `codex/piping-pkg08-report-package`.
- Dependency authority: owner-accepted and active `DAG-008`.
- Selection authority: the owner's adopted serialized next-tranche plan,
  applied by HELP_HUMAN after DAG-008 activation.
- Posture: `MIXED` — parallel disjoint read-only reconnaissance, then serial
  candidate/refutation, implementation, verification, and closeout gates.

## Objective and fence

Bind the existing report-package producer to the existing desktop menu and
implement caller-owned atomic on-disk save using temporary-file, write, and
rename semantics. Preserve the established report/package schema, sanitized
redaction output, manifest evidence, diagnostics, provenance, units,
sandboxing, claim-state distinctions, and professional-boundary notices.

No runner verb is added or altered. The DEL-10-05 `export-results` stub is not
consumed or changed. No DEL-10-05, PKG-07, PKG-11, lifecycle, release, issuance,
or merge effect is in scope. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081).

## Graph

| Node | Owner | Dependencies | Concurrency | Writes | Return gate |
|---|---|---|---|---|---|
| N1 producer reconnaissance | fresh Agent 2 generalist | frozen basis | parallel with N2 | N1 evidence only | bounded observed contract map |
| N2 desktop/native reconnaissance | fresh Agent 2 generalist | frozen basis | parallel with N1 | N2 evidence only | bounded route/save/test map |
| W1 candidate synthesis | WORKING_ITEMS | accepted N1+N2 | serial | candidate, matrices, run records | complete scope/route/write/test freeze |
| N3 independent refutation | fresh Agent 2 generalist | W1 candidate | serial | N3 evidence only | `COMMIT-SAFE` or `BLOCK` |
| G1 candidate adoption | human owner unless exact standing adoption is demonstrably sufficient | N3 `COMMIT-SAFE` | stop gate | governed adoption record | adopted brief |
| N4 implementation | one serialized Agent 2 integration owner | adopted brief | serial | exact adopted write matrix | terminal implementation return |
| N5 fresh verification | fresh Agent 2 generalist | accepted N4 return | serial | N5 evidence only | `COMMIT-SAFE` or `BLOCK` |
| W3 DEL-08-01 closeout | WORKING_ITEMS | N5 `COMMIT-SAFE` | serial | DEL-08-01 state/memory/run record only | package return to HELP_HUMAN |

Git closeout belongs to CHANGE. Exactly one acceptance-eligible DEC-025 sweep
is permitted for each terminal implementation attempt; superseded failed
attempt evidence remains preserved and non-acceptance-eligible.

## Human and escalation gates

- Any material scope, contract, ownership, risk, or acceptance change returns
  to HELP_HUMAN and the owner.
- Candidate adoption remains a stop unless a current accepted authority is
  found that expressly adopts the exact candidate rather than only directing
  its preparation.
- N3 or N5 `BLOCK` holds all dependent nodes.

