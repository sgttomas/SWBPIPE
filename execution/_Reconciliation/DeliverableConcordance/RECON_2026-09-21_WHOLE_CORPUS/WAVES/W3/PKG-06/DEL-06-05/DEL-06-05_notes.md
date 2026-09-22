# DEL-06-05 notes — Invented non-code example rule pack

Wave W3, PKG-06, worker G2. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed first (124 rows: 70 required keys, 51 optional `.rNN`
rows for 8 split blocks, 3 `.sNN` sub-claims); reverse file written after both
of this worker's seals.

## Path aliases

- `EX` = `examples/rule_packs/invented_demo.yaml` (this deliverable's artifact,
  commit `73506b7`, later upgraded by `d3885f4ba`, `e200e47a3`, `f737985ab`).
- `NOTICE` = `projects/chirality-piping/docs/_Examples/rule_pack_notice.md`.
- Example verification: `tests/test_rule_pack_schema.py`,
  `core/rules/rule_pack_document/tests/invented_demo_document.rs`, plus the
  gate sweep `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/`. No suite was rerun.
- Parity: root-level `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-06-05/`.

## Judgment calls

- **Setup framing versus an implemented example (FG-02, 12 rows).** The SOW
  still describes a setup-only deliverable. Its lines say "does not create
  repo-level examples", "write boundary: this folder only", "no repo-level
  example edited" and "local setup only". But this deliverable committed the
  example and notice on 2026-05-02. All such text was first present at
  `7bee9ae41`, so it is `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE` (F3). The
  D-41 declaration says the implemented slice "supersedes" that wording. Under
  C1 and CP-03 that blanket declaration does not change sibling dispositions.
- **Conditional rows.** Some rows set conditions that are now satisfied, such
  as "TBD until a concrete payload exists" and "after schema and grammar are
  authorized". Examples are REQ-05, CLM-006.r04, CLM-026, CLM-027 and AC-001.
  These are `ALIGNED`: the example followed the condition (grammar after
  DEC-022; checksum stamped once the payload existed). Rows whose trigger can
  no longer arise and that read as live instructions are stale setup text:
  CLM-005.r03 (future artifact) and CLM-005.r04 (checksum before payload).
  Both are MEDIUM.
- **D-41 declarations (CLM-002/009/016/023, FG-01).** Each pins SOFTWARE_DECOMP
  revision 0.8 and DAG-007 (frozen: 0.12 and DAG-010), so CP-03 directs the
  CP-02 fields. Their delegation to `## Remaining` points at an empty section
  and is not relied on (A4).
- **REQ-07 and CLM-003.r08.** These are write-boundary constraints scoped to
  the setup run. I read them as current requirements the SOW still presents,
  so they are stale (MEDIUM). A verifier could read REQ-07 as accurate history
  of the setup run instead.
- **CLM-012.** Assessed on its own sentence (`ALIGNED`) with rows split. r03 is
  `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING` (grammar TBD, since
  ruled by DEC-022). r04 is `DOC_BEHIND_CODE` (the schema path now exists).
- **MEMORY.** The undated "Open Items" block still reads as current. It lists
  checksum generation, private storage, GUI, API transport and
  completeness-checker behaviour as TBD, and all of these now exist. Result:
  `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`, matching DEL-06-04's MEMORY
  row. The dated 2026-06-12 entry names the grammar marker of that time; that
  is history, not residue.

## Canonical departures

- **CP-04 (SOW SURFACE).** The SOW names "OpenPipeStress governance and
  invariant documents" (CLM-012.r01). The example it names carries
  `rule_pack_kind: open_pipe_stress_rule_pack` and OpenPipeStress provenance
  labels, and the notice mentions the "OpenPipeStress rule-pack schema
  surface". The row uses the CP-04 default variant (`LOCAL_DESIGN · NONE ·
  RECORD · OWNER`) and records the residue once. The front-matter revision
  0.8 pin moved to `SOW.s01` (CP-02) so the SURFACE row can carry CP-04.
- **CP-02 → F3.** CLM-007.r03 and CLM-018.r02 (INIT.md) use
  `STALE_SETUP_SPECIFICATION`, as DEL-06-04 does; each row says
  `CANONICAL_DEPARTURE:`.
- **CP-03 rows** carry `CANONICAL_DEPARTURE:` explaining why they differ from
  DEL-06-04's CP-03 rows (pins versus none).
- **CS rows** keep their assigned fields. `.s01` and `.s02` are handled as
  for DEL-06-04.

## Convention friction

- A declaration can delegate residuals to a `## Remaining` section that is
  empty. A4 says not to rely on the delegation, and there is nothing to audit;
  that is recorded only in the Notes.
- `STATUS#remaining` is pre-typed NON_NORMATIVE (an empty section) and is
  disposed `NOT_ASSESSED`.

## UNKNOWN rows

None.

## Reverse pass

- CLAIMED_BY: RC-06-0200 (the example).
- PARTIAL: RC-06-0179 (the notice is mine; the `_Examples` README has no
  DEL-06-05 key).
- COVERS: RC-06-0167 (preview demo pack derived from the example) and
  RC-06-0099 (schema the example conforms to).
- All 24 path-overlap rows carry capability-specific reasons (F5).
- The reverse pass did not change my view of anything sealed.

## Batch consistency

`--batch` over the DEL-06-04 and DEL-06-05 forward ledgers: **PASS, 0
findings**.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
