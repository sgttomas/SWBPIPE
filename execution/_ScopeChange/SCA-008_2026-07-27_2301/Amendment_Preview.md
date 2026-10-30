# Piping SCA-008 Gate 3 — exact amendment preview

## State

`CANDIDATE — GATE 3 NOT APPROVED`

This is an exact, derivative candidate against durable `main` basis
`4f8448143608588b06e968cf1ff07054ee1da60c`. The inspected source tree is
`e1b1eab0cc8fbaa9f1213a28ba55ecf364fb9092`, byte-identical to the tree of
the merged basis. No repository, pointer, snapshot, notice, product, runtime,
lifecycle, release, reliance, dependency, estimate, schedule, or Git state
has changed.

The accepted Gate-2 impact is `Impact_Assessment.md` SHA-256
`c3f975a624bd52ead0914de44215b15fcbc558c4ef25b815097297ebb9f888f2`.

## Exact working-surface postimages

| Live path | Preimage SHA-256 | Exact postimage | Postimage SHA-256 |
|---|---|---|---|
| `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | `dbcbd4c5967662152bc8c1b7ba28196bfc0654773feb99ce05e15b24b32986c8` | `postimages/SOFTWARE_DECOMP.md` | `82b835b5fd36d0fd337da5b084dbf146caa29c18d0e1ef8f96a06fcfa4363a07` |
| `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md` | `632a0d418733f00b890e81a3ef117db5e6d5ffbfbada13ca72b45fa0fa9bfb62` | `postimages/DEL-16-04_STATUS.md` | `39af5d4b4ebfcf0c0c46b122f29d33f33b0774094be317201ac8a8269572b366` |

The complete exact diffs are:

- `Exact_Diff_SOFTWARE_DECOMP.patch`;
- `Exact_Diff_DEL-16-04_STATUS.patch`.

## Exact decomposition changes

1. Change frontmatter revision `0.10` to `0.11`.
2. Add one revision-history paragraph for SCA-008. It records D-58's current
   effect, preserves R7 and Piping's non-client posture, and states all
   structural and authority non-effects.
3. Add three D-58 records as current authority inputs and the root
   program-architecture evaluation as derivative evidence only.
4. Correct the existing context-envelope summary from the stale
   `S=9, M=70, L=22, XL=0` to the register-derived
   `S=9, M=69, L=23, XL=0`.
5. Correct the stale coverage revision from `0.7 — 2026-05-18` to
   `0.11 — 2026-07-27`.
6. Add exactly one next-free row, `DEC-091`, after `DEC-090`. It preserves
   D-30, D-31, D-58, DEC-041, and DEC-063 as historical acts; retires only
   current reliance on the App-era synchronized-consumption mechanism;
   leaves the automation-condition mechanism unresolved; preserves R7 and
   non-client status; and adopts no successor mechanism or downstream
   authority.
7. Advance the gate-posture language from SCA-007/revision 0.10 to
   SCA-008/revision 0.11 and add the current-effect fence.

## Exact DEL-16-04 change

Only the second bullet under `## Remaining` changes. The exact postimage:

- retains `Current State: IN_PROGRESS`;
- retains the R7 and App F3 live-binding gates;
- no longer treats the DEC-063 App-era mechanism as the complete current
  gate;
- adds the D-58 fact that the automation-condition mechanism remains
  unresolved after retirement of current reliance on that mechanism; and
- changes no other Remaining or History byte.

The sibling `MEMORY.md` remains read-only and unchanged.

## Exact preservation envelope

The candidate changes no PRD byte, including R7; no scope-ledger,
deliverables-register, context-budget, objective, requirement, dependency,
ScopeOfWork, source, schema, runtime, profile, implementation, lifecycle,
release, reliance, estimate, or schedule surface; and no historical D-30,
D-31, D-58, DEC-041, or DEC-063 byte.

Topology remains exactly:

- 76 scope items, all `IN`;
- 18 packages;
- 101 deliverables;
- 101 context-budget rows;
- 18 objectives;
- 0 unassigned scope items;
- 0 scope items without deliverable mappings; and
- 0 unmapped objectives.

The context-envelope distribution remains exactly
`S=9, M=69, L=23, XL=0`.

## Later propagation candidates — not approved or applied here

Gate 4 must present exact bytes for:

- `projects/chirality-piping/execution/_Decomposition/_LATEST.md`;
- `projects/chirality-piping/execution/_ScopeChange/_LATEST.md`;
- a complete immutable
  `projects/chirality-piping/execution/_ScopeChange/SCA-008_2026-07-27_{HHMM}/`
  snapshot using `Snapshot_Inventory.md`;
- `execution/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md`;
- `projects/chirality-app-dev/execution/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md`; and
- `_DomainEngines/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md`.

The timestamp is minted once during Gate-4 exact propagation preparation.
Notice receipt or acknowledgment is tracked but does not grant authority and
is not a closure veto.

## Gate-3 effect

Approval accepts these exact amendments for later application. It does not
apply them, approve Gate 4, advance a pointer, create the snapshot, send a
notice, or authorize implementation, repinning, lifecycle, release,
professional reliance, dependency, estimate, schedule, or Git work.
