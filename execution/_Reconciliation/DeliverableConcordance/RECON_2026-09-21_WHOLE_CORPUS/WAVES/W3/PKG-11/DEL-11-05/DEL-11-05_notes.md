# DEL-11-05 notes — Contributor tutorial and onboarding (W3, PKG-11, worker G2)

Forward ledger sealed at `64f4c972cea6696b4eb7595e4cf0a72d27b7de86a5206da9994de2f3a1d0e43b`
(114 rows: 73 required keys, 39 optional `.rNN` rows for the split blocks
CLM-003, CLM-004, CLM-005, CLM-007 and CLM-032, and 2 `.sNN` sub-claims).
Validator with `--notes-gap`: PASS, 0 findings (one F4 hit was reworded
before sealing, with no change of disposition). Not an R0 pilot. All values
are agent judgments, not owner rulings.

## Path aliases

- "The tutorial draft" in the SOW means the published
  `docs/contributor_guide/index.md`. Tranche M created it (`bfb3931`,
  2026-05-09) together with a link in `CONTRIBUTING.md`. The 2026-06-07 TASK
  run re-created `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` and refreshed both.
  The kit-resident draft went away at the SOW migration.
- The scan records (`_run_records/TASK_RUN_2026-05-09_type2_implementation.md`,
  `TASK_RUN_2026-06-07_1711.md`, `TASK_RUN_2026-06-07_1812_*`) are older than
  the guide edits of 2026-07-17 (`8fac6631a`) and 2026-09-18 (`8143645ea`,
  `bb7c7a2e8`). The guide was read at the freeze. The scans were not rerun.
- Project `INIT.md` was removed by `9c4caf8fd` on 2026-07-04. The sealed brief
  named in MEMORY existed at `7bee9ae41` and was removed the same day by
  `9b608c3f3`.

## Judgment calls

- **INIT.md dead link.** Step 1 of the guide's First-Hour Path links
  `../../INIT.md`, which does not exist at the freeze. REQ-11-05-01 and
  CLM-028 are therefore `PARTIALLY_IMPLEMENTED` · BASIS_POINTER_STALE, because
  the reading path is structurally right but starts at a dead link. The SOW's
  own INIT.md pointers (CLM-004.r05, CLM-007.r01) are
  `STALE_SETUP_SPECIFICATION` · BASIS_POINTER_STALE. All five share
  FG-DEL-11-05-04.
- **REQ-11-05-07 is `DOCUMENTED_UNIMPLEMENTED` (MEDIUM).** Neither the guide
  nor the workflow map mentions architecture-basis IDs, PKG-00 or the
  dispatch-context rule. D-43 later consolidated PKG-00. A catch-up could
  either add the guidance or narrow the requirement.
- **CLM-031 is `STALE_REVIEW_OR_EVIDENCE` · SCOPE_REDIRECTED_BY_RULING.** Its
  premise (PKG-00 at SEMANTIC_READY under SCA-001) is overtaken by D-43 and
  by PKG-00's IN_PROGRESS state. The tutorial gap itself is disposed on
  REQ-11-05-07.
- **Kit-bound verification columns (F8).** REQ-11-05-02, 03, 04, 05 and 08
  are met in substance by the guide. Their verification column still checks
  Datasheet/Guidance/Procedure. Following F8 (only a record clause is stale)
  they are CP-01 `STALE_SETUP_SPECIFICATION` · REPRESENTATION_MIGRATED ·
  LOCAL_DESIGN (FG-DEL-11-05-05). REQ-11-05-06's verification is a text scan,
  so that row is ALIGNED.
- **CONTEXT#anticipated-artifacts is `STALE_SETUP_SPECIFICATION` ·
  BASIS_POINTER_STALE.** Both named artifacts exist. However, `cc1b91c5b`
  (2026-06-03) changed the Deliverables.csv row to name
  `execution/_Coordination/_COORDINATION.md` as the second artifact, and the
  CONTEXT list was not updated.
- **Setup-era statements** follow the same F3 treatment as DEL-11-04
  (FG-DEL-11-05-02). These are CLM-004.r02/r04, CLM-005.r04, CLM-010,
  CLM-016, CLM-019, CLM-020, CLM-022, CLM-032.r01 and CLM-036. CLM-003.r08
  (lifecycle target SEMANTIC_READY) is a readiness state and so is
  `STALE_REVIEW_OR_EVIDENCE`.
- **MEMORY is HISTORY · ALIGNED.** Every section is dated and accurate, and
  there is no undated current declaration. DEL-11-04's MEMORY differs: it
  carries a D-41 current declaration.
- **STATUS#remaining** is pre-typed NON_NORMATIVE and empty (0 items).

## Canonical departures

- CLM-004.r05 and CLM-007.r01 (INIT.md pointers) use CP-02 with
  `STALE_SETUP_SPECIFICATION` by F3, each with `CANONICAL_DEPARTURE:`. This is
  the same treatment as DEL-11-04.
- No CS assignment was departed from (7 keyed rows inherited).

## Convention friction

- F8's record-clause rule makes requirement rows non-aligned when only their
  verification column names removed kit documents. The requirement substance
  is met. R3 may want to cluster FG-DEL-11-05-05 with other CP-01 residue.
- CP-02 versus F3 for removed-file pointers, as in DEL-11-04.

## UNKNOWN rows

None.

## Reverse pass

267 capabilities answered. One is `CLAIMED_BY`: RC-11-0186, the contributor
guide. Two are `PARTIAL`: RC-11-0085 (CONTRIBUTING.md, where DEL-11-05 owns
the onboarding link and governance owns the policy) and RC-11-0131
(AGENTIC_DEVELOPMENT_WORKFLOW.md, which DEL-11-05 re-created, while the
authority of the process it maps sits with coordination). The other 264 are
`NOT_MINE`. Thirteen capabilities overlap cited paths (10 answered
`NOT_MINE`), and each has a specific reason (F5).

The reverse pass did not change my view of any sealed row. RC-11-0131
supports the anticipated-artifacts reading: the register moved the second
artifact to `_COORDINATION.md`, yet DEL-11-05 still produced the workflow map.

## Batch consistency

`--batch` over DEL-11-04 and DEL-11-05 forward ledgers: PASS, 0 consistency
findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping
selects work through owner-steered work graphs, not `## Remaining`.

## Protected checks, invariants, ISSUED, authority conflicts

- No protected check, ISSUED artifact or AUTHORITY_CONFLICT row is involved.
  The professional-boundary rows (CLM-004.r06, REQ-11-05-06, OPS-K-AUTH-1,
  CLM-030) are ALIGNED on a reading of the guide at the freeze.
- Possible defect for the owner: the contributor guide's first reading step
  links the removed `INIT.md`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
