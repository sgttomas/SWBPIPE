# OD7-G1 Piping Record and Notice Candidates

> **Status: CANDIDATE — NOT APPROVED OR APPLIED.** This package changes no
> live Piping decision, receipt, pointer, decomposition, contract, scope, or
> product surface.

| Field | Value |
|---|---|
| Candidate basis | `main@fb16e32ed60bb4f384cf1e07a83c4a14ff63bbae` |
| Parent batch | `OD7-G1` |
| Candidates | `OD7-G1-C03`, `OD7-G1-C04`, `OD7-G1-C06` |
| Prepared by | `HELPS_HUMANS` (Agent 1), managed by `HELP_HUMAN` |
| Artifact class | Piping coordination proposal |
| Date | 2026-07-27 |

## C03 — D-30 publication identity

Git proves that commit
`712df44816cf5253223b449fec0f10b48abd585c` first added both the D-30
ruling and its consumption JSON, is an ancestor of the candidate basis, and
that the ruling bytes remain unchanged. The candidate changes only the
`Ruling SHA` field under the established publication-SHA backfill convention.

## C04 — decomposition currency correction after rescan

The accepted OD-7 evaluation proposed `_LATEST.md` `0.6 → 0.9`, subject to
execution-time rescan. That rescan found the target stale:

- accepted SCA-007 and its `_LATEST.md` record say decomposition revision
  `0.10`;
- `SOFTWARE_DECOMP.md` line 37 records revision `v0.10`;
- the accepted SCA-007 A023/A024 actions and acceptance records require
  `0.10`; and
- SCA-007 landed in ancestor
  `ab0e6cfc4d4871989cf887fdd45e3c2d3168b41f`.

C04 therefore proposes:

1. `_Decomposition/_LATEST.md`: `0.6 → 0.10`; and
2. the newly exposed `SOFTWARE_DECOMP.md` frontmatter residue:
   `0.9 → 0.10`.

The second edit is separately visible in `PIPING_C03_C04.patch` and requires
explicit owner approval within C04. It is not silently absorbed. It transcribes
already accepted SCA-007 current-basis truth and changes no scope.

## C06 — factual D-30 mismatch notice

The combined validator currently exits `1` with:

`ERROR: consumption source.commitSha mismatch`

Piping D-30 names source commit `ee290e22…` and registry `v6`; the current
D-APP-48 record names `55a066fd…` and registry `v14`. This proves record
drift, not semantic incompatibility. The notice blocks only reliance on
D-30's named synchronized-consumption claim and authorizes no repin,
successor, or Piping runtime-client scope.

## Candidate IDs

Current scan results:

- next Piping decision candidate: `D-58`;
- next decomposition decision-log candidate: `DEC-091`;
- next receipt candidate: `Receipt-76`; and
- the dated notice filename is unique.

None is reserved. C03/C04/C06 require no new D-* or DEC-* ID. `D-58` and
`DEC-091` remain for later substantive retire-or-replace work if still free.
The receipt ID and cursor must be rescanned immediately before application; a
disagreement returns a refreshed exact candidate.

## Exact application surfaces

Only the five paths in `DIRECT_WRITE_MAP.csv` are proposed. The receipt
fragment is valid only if all three C03/C04/C06 candidates are approved; a
subset requires a narrowed reissue.

## Authority route

Exact owner approval → Piping record/pointer owner application → receipt and
Piping checks → `CHANGE`. No SCOPE_CHANGE is needed for the clerical
application of already accepted SCA-007 revision metadata. The later
retire-versus-replace disposition and any decomposition semantic change remain
OD7-G5 and Piping SCOPE_CHANGE work.

## Exclusions

- D-30 JSON, D-31, DEC-063, and the decision register remain byte-identical;
- no D-APP-48 or D-30 repin;
- no successor contract or compatibility claim;
- no inference of semantic incompatibility from byte/record drift;
- no runtime dependency, Root-runtime client scope, product code, package,
  deliverable, lifecycle, release, publication, or professional-reliance
  effect; and
- no D-58, DEC-091, SCOPE_CHANGE gate, or owner decision is pre-approved.

## Rollback

Before merge, restore the three modified lines and remove the proposed notice
and receipt if any check fails. After merge, use additive correction where a
receipt or notice is involved; do not rewrite accepted receipt history. A
rollback does not make the D-30 synchronized-consumption claim reliable.
