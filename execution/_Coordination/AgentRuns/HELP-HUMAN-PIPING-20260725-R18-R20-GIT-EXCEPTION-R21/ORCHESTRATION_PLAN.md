---
doc_id: R21-R18-R20-GIT-EXCEPTION-PLAN
doc_kind: coordination.orchestration_plan
status: TERMINAL_VERIFIED_RECEIPT_INTEGRATED
created: 2026-07-25
version: 1
---

# R21 staged-whitespace exception plan

## Activation and authority

- Parent: HELP_HUMAN.
- Manager: HELPS_HUMANS.
- Owner authority: exact payload in `OWNER_DIRECTION.md`.
- Branch: `codex/piping-candidate-briefs-20260725`.
- Frozen source HEAD:
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
- Pre-governance index: 105 paths; ordered path-list SHA-256
  `3652393639a6d41dfef45325ccf7ac5f0bd945c7ce346990594b95080be26202`.

## Write fence

1. D-57 ruling record.
2. One D-57 register row.
3. One next-free `DEC-090` append.
4. This R21 managed subtree.
5. Receipt-75 only after accepted fresh Agent 2 verification.

No affected staged blob may be normalized or otherwise changed. No product,
configuration, deliverable lifecycle, DAG, build, packaging, release,
publication, push, merge, network, or other external effect is authorized.

## Work graph

| Node | Owner | Work | Gate |
|---|---|---|---|
| P0 | HELPS_HUMANS | freeze owner/index/findings/hash evidence | exact bindings |
| P1 | HELPS_HUMANS | D-57, register, DEC-090, R21 | scope and semantics |
| V1 | fresh Agent 2 | read-only independent verification | `PASS / COMMIT-SAFE` or `BLOCK` |
| I1 | HELPS_HUMANS | terminal handoff and Receipt-75 | accepted V1 only |

The staged check remains exit `2`; the only permissible closeout label is
`PASS_WITH_OWNER_EXCEPTION_DEC_090`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
