# NOTES — DEL-00-04 Persistence and schema versioning architecture (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen source tree:
`main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding encoding set:
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13). Ledger:
`CLAIM_CONCORDANCE_DEL-00-04.csv` (17 rows, 20 columns, RFC-4180, CRLF).

DEL-00-04 is a PKG-00 architecture-runway (documentation-only) deliverable:
`DATA_MODEL_CHANGE` type, no product code. Its five requirements are
architecture-definition obligations realized as accepted architecture basis
`AB-00-04` (SOFTWARE_DECOMP architecture-basis table) via `SCA-001`
(PKG-00 SEMANTIC_READY content usable as basis) and `SCA-003` (MVP local
SQLite storage profile; canonical JSON / JSON Schema 2020-12 / JCS hashes).
Concrete persistence is implemented downstream (DEL-02-05, DEL-08-02,
DEL-14-01), not in this deliverable. Lifecycle IN_PROGRESS; the deliverable's
own Human Review Gate (architecture sufficiency) is still pending.

## Requirement-ID scheme mapping (non-self-identifying scheme REQ-04-*)

Per addendum 12 the ClaimID is fixed `DEL-00-04-<TYPE>-NNN`; the deliverable's
requirement IDs do not self-identify the deliverable, so the mapping is:

- `DEL-00-04-REQ-001` = REQ-04-01 (persistence deterministic/versioned/unit-aware/provenance)
- `DEL-00-04-REQ-002` = REQ-04-02 (schema version + migration status declared)
- `DEL-00-04-REQ-003` = REQ-04-03 (canonicalization prerequisite; algorithm TBD)
- `DEL-00-04-REQ-004` = REQ-04-04 (round-trip checks)
- `DEL-00-04-REQ-005` = REQ-04-05 (public/private schema separation)

ACC/EXC/DECL ClaimIDs are run-local (no source IDs); NormativeSource cites the
exact prose surface for each.

## Disposition histogram (recount from CSV)

- ALIGNED — 15
- STALE_SETUP_SPECIFICATION — 2

Total 17.

## ClaimType histogram (recount from CSV)

- REQUIREMENT — 5
- ACCEPTANCE — 3
- EXCLUSION — 3
- DECLARED_STATE — 6
- REMAINING_WORK — 0
- IMPLEMENTED_UNMAPPED — 0

Total 17.

Census rationale: the only `## Remaining` item is the seeded `(gated: D-41)`
bootstrap, recorded verbatim ONLY in the `_STATUS.md` surface row
(`DEL-00-04-DECL-005`) RecordedRemaining and excluded from residual/gate/
selectability analysis (addendum 2) — hence zero REMAINING_WORK rows
(R1 DELIVERABLE_INVENTORY confirms NonBootstrapItems=NONE). Zero
IMPLEMENTED_UNMAPPED rows: R1 IMPLEMENTATION_SURFACES.csv attributes no surface
to DEL-00-04; every persistence surface (docs/architecture/persistence_contract.md,
schemas/project_persistence.schema.yaml, core/project_persistence,
projectService.ts, hashService.ts) is already mapped to DEL-02-05/DEL-08-02/
DEL-14-01 — none unmapped in this deliverable's orbit.

## Self-flagged rows

- `DEL-00-04-REQ-001` (representative for REQ-001..005): SourceReliability=REVIEWED
  rests on SCA-001/SCA-003/AB-00-04 living in the human-accepted decomposition
  basis (revision 0.8, `current_basis`) rather than a separately-quoted
  per-record acceptance line; the SOFTWARE_DECOMP revision history states
  explicit human-project-authority acceptance for the v0.3 amendment and named
  DEC rows, and treats the SCA amendments as accepted amendments to that basis.
  A reviewer may prefer UNVERIFIED if a per-SCA acceptance line is required.
- `DEL-00-04-REQ-001..005` (lifecycle tension): AB-00-04 is in accepted
  downstream-injection use while the deliverable's own architecture-sufficiency
  Human Review Gate is still pending and lifecycle is IN_PROGRESS (SEMANTIC_READY
  was reverted by the D-40 administrative correction). Substance is aligned; the
  pending sufficiency review is a lifecycle/issuance gate, not a per-requirement
  defect — surfaced here for reviewer eyes, not routed per-row.
- `DEL-00-04-REQ-004`: the Specification enumerates "diagnostics" among
  round-trip subjects, but AB-00-04/SOW-050 enumerate models, units, loads,
  rule-pack references, provenance (not diagnostics). Judged a downstream
  realization detail, not a DEL-00-04 definition gap → ALIGNED at MEDIUM.
- `DEL-00-04-DECL-002`, `DEL-00-04-DECL-003`: STALE_SETUP_SPECIFICATION calls.
  Judgment that accepted SCA-003/AB-00-04 resolutions (storage backend, physical
  format, canonicalization/hash algorithm) overtake the still-listed TBD slots in
  the Datasheet "TBD and Human-Ruling Slots" and Guidance "Human-Ruling Queue".
  `_CONTEXT.md` (not a declared-state census surface) already carries the SCA-003
  injection, so the drift is isolated to these two kit surfaces. Migration
  framework/tooling remains a genuine surviving TBD.
- `DEL-00-04-DECL-002` also carries an anticipated-artifact reconciliation
  observation: `docs/architecture/persistence_versioning.md` does not exist; the
  architecture basis was realized as `AB-00-04` plus downstream
  `docs/architecture/persistence_contract.md` (owned by DEL-02-05). Folded into
  the STALE disposition rather than raised as a separate finding.
- Cross-cutting revision-pointer drift: the four kit documents, `_CONTEXT.md`,
  and `_DEPENDENCIES.md` cite SOFTWARE_DECOMP "revision 0.7" while the live and
  frozen basis is revision 0.8 (through DEC-072). Persistence architecture content
  is unchanged 0.7->0.8, so this was dispositioned as a minor kit-wide metadata
  refresh (ALIGNED on DECL-001/DECL-004, AuthorityNeeded=NO; folded into the R5
  repair RemainingWork on DECL-002) rather than REMAINING_STATE_MISMATCH per row.
  Flagged for reviewer confirmation that the measured (non-inflating) treatment
  is preferred over a per-surface metadata-mismatch call.
- ACC/EXC census grain: the 3 ACCEPTANCE + 3 EXCLUSION rows were derived from the
  Specification Acceptance Criteria bullets and the Datasheet/Guidance/_CONTEXT
  scope-boundary prose (acceptance bullets 2/4/5 folded into exclusion rows to
  avoid double-encoding). A reviewer may merge or re-split at a different grain;
  the six global Required Invariants (OPS-K-*) were folded into the governing
  REQ/EXC rows rather than given their own rows (not addendum-12 grain).

## Evidence-execution log

- Re-executed ONLY side-effect-free reads/inspection: `git status --porcelain`,
  `find`, `grep`, `ls`, `sed`/`head` over the frozen tree and CSV. No builds, no
  tests (the deliverable owns none — R1 VERIFICATION_INDEX carries no DEL-00-04
  rows), no CARGO/pytest invocation. No `content-identical` ancestor-diff
  qualifier used (not needed; all cited evidence is at the frozen SHA).
- Cited as recorded (not re-executed): none required an execution marker — all
  ledger evidence is direct inspection at frozen SHA 551f84ef6 of the deliverable
  kit, SOFTWARE_DECOMP (AB-00-04, SCA-001/003, SOW-059, revision 0.8),
  docs/architecture/persistence_contract.md (DEL-02-05), schemas/
  project_persistence.schema.yaml, D-40 ruling record, and the R1 index CSVs.
- Frozen-tree `git status --porcelain` verified EMPTY before all reads and again
  after the final CSV write and validation.

## Convention friction notes

- Addendum 6 binds "DECLARED_STATE prose rows always NOT_APPLICABLE" for
  SourceReliability; the format exemplar (R0B_CLAIM_CONCORDANCE_DEL-07-05.csv,
  which predates the adopted addenda) used REVIEWED on its `_STATUS.md` row. I
  followed the binding addendum: all six DECLARED_STATE rows are NOT_APPLICABLE,
  including `_STATUS.md` and MEMORY.
- SourceReliability for ACCEPTANCE/EXCLUSION rows is not explicitly fixed by the
  addenda. I used NOT_APPLICABLE, treating these as non-technical presence/
  boundary prose (R0b convention 5), rather than UNVERIFIED — the underlying
  facts are objective presence/absence checks, not technical evidence with a
  reliability posture.
- Documentation-deliverable disposition semantics: §6 "behavioral alignment
  requires implementation and reproducible verification" was adapted to a
  documentation runway — the "implementation" of a define/require obligation is
  the accepted architecture record (AB-00-04 + kit prose), and "verification" is
  document/acceptance review. The pending human sufficiency review is noted, not
  treated as a technical gap. ValidationEvidence is NOT_APPLICABLE with an
  in-cell reason on every requirement row (no MECHANICS/VALIDATION class here).
- Addendum 5 (declared bounded TBD): REQ-04-03's own "leave the concrete
  algorithm TBD" is a requirement-internal deferral that was subsequently
  resolved (JCS) rather than an unresolved residual, so it is ALIGNED, not
  ACCEPTED_DIVERGENCE — no named ruling was needed to permit an open state
  because the state is resolved.

## Boundary-compliance statement

- All writes confined to the two output files under `RUN/WAVES/W1/`
  (`CLAIM_CONCORDANCE_DEL-00-04.csv`, `NOTES_DEL-00-04.md`). The build script
  lives in the session scratchpad, outside both the frozen tree and the run
  folder product surfaces.
- Discovery was read-only outside those two files: no `_STATUS.md`, register,
  DAG, decomposition, or product-file edits; no lifecycle transition applied
  (`LIFECYCLE_REASSESSMENT_REQUIRED` not used); no DAG mutation; no cross-project
  edits.
- No frozen-tree writes (porcelain empty before and after, including no
  `target/`, `__pycache__`, or `.pytest_cache` — nothing was built/executed).
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim anywhere in the ledger or
  notes. The EXCLUSION rows record the deliverable's OWN boundary declarations
  (e.g. no protected data introduced), not any approval by the agent.
- All dispositions are agent judgments; the two STALE_SETUP_SPECIFICATION repair
  candidates and the SourceReliability provenance question route
  `AuthorityNeeded=OWNER` / are surfaced for reviewer eyes — none are phrased as
  owner or engineering rulings. No `DEFERRED_AGENT_WORKFLOW` implications arose.

## STOP-worthy contradiction

NONE. The frozen register shows D-41 `AWAITING_RULING`; per RUN_BASIS this is
ruling-after-freeze mechanics, not a conflict, and was not re-derived. No
`AUTHORITY_CONFLICT` encountered.
