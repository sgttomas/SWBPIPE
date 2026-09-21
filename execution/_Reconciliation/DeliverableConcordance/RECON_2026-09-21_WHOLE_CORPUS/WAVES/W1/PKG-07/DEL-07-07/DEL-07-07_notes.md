# DEL-07-07 Solve execution UX — worker notes (W1, PKG-07, worker G3)

Forward ledger `DEL-07-07_forward.csv`: 125 rows (82 required keys, 7 canonical
assignments inherited without departure, `.rNN` splits of CLM-003, CLM-004,
CLM-005, CLM-007 and CLM-020, three `.s01` sub-claims). Sealed in
`DEL-07-07_SEAL.txt`. Evidence read from the frozen checkout at `00115c719`
only. Not an R0 pilot. Shared situation rules are in
`../_WORKER_DEL-07-05_NOTES.md`.

## Path aliases

- Deliverable-local files are cited in `ContextRefs` because their path
  contains spaces.
- Parity records: root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-07-07/`.
- The SOW's "DEL-00-03/05/06 `Specification.md`" references: PKG-00
  deliverables carry `ArchitectureBasis.md`, and the AB rows are in
  SOFTWARE_DECOMP rev 0.12.
- Backend job seam: `apps/desktop/src-tauri/src/lib.rs`
  (`start/poll/cancel_preview_mechanics_job`, from L1777).
- `docs/SPEC.md` section numbers in the SOW are offset (see the DEL-07-05
  notes). Recorded in Notes; the disposition follows the substance.

## Judgment calls

- Most of this SOW is setup-era text that says the slice is not yet
  implemented, but it has landed: the Solve panel, the Tauri background job
  with cooperative cancellation (`cancellation_success_claimed=false`), the
  event-state progress record (`percentages_synthesized=false`), and
  diagnostic class/remediation/provenance carriage. Such rows are
  STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE (CLM-004.r07, CLM-010,
  VER-07-07-002, VER-07-07-007) or CP-01 where the text names the four-document kit.
- Requirements REQ-07-07-001..011 are all ALIGNED on substance. Where a
  requirement also says "exact phases/enums TBD", I noted the TBD clause as
  overtaken rather than changing the disposition.
- OI-07-07-001..003 → STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE, not CP-10:
  the SOW itself delegated each choice to a future implementation or GUI task,
  and that work landed. DEL-07-05 CLM-005.r02 is different (no delegation;
  CP-10).
- **FG-DEL-07-07-01, producer-side diagnostic carriage.** The GUI shows
  class, remediation and provenance when supplied and labels absence "not
  supplied by producer". Upstream producers still emit the reduced shape.
  CLM-016 and STATUS R01 → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE; AuthorityNeeded
  OWNER because the producers belong to other deliverables.
- CLM-015 (what SEMANTIC_READY means) is a definition consistent with TYPES
  §9 → ALIGNED.
- The STATUS SURFACE is ALIGNED: Last Updated 2026-09-13 equals the latest
  history entry. The 2026-09-13 entry's "Git publication remains pending" was
  true then; PR #785 merged on 2026-09-14 (context only).
- CLM-012 is ALIGNED on its exclusion substance. `CLM-012.s01` (controlling
  references "listed in … `Datasheet.md`") is CP-01.
- For R3 (no forward key): `SolvePanel.tsx` still emits the former-name
  identifiers `openpipestress-preview-solve-job-*.json` and
  `openpipestress.technical_preview.solve_job_audit`. The SOW does not name
  them, so no CP-04 row applies here. The CP-04 finding on the SOW SURFACE row
  is for CLM-012's "OpenPipeStress governance" wording.

## Canonical departures

None. CS-04 carries `.s01` for the PKG-00 SEMANTIC_READY statement, the same
as DEL-07-05.

## Convention friction

- There is no named pattern for a SOW that reads as a pre-implementation setup
  plan after implementation landed. I used STALE_SETUP_SPECIFICATION ·
  DOC_BEHIND_CODE for origin text that denies implementation, and CP-01 where
  the four-document kit is named.
- Evidence columns cannot hold deliverable-local paths (spaces).

## UNKNOWN rows

None.

## Reverse pass

487 routed capabilities: CLAIMED_BY 3 (SolvePanel, solveJobAudit,
`core/gui/solve_execution`), PARTIAL 3 (backend solve-job seam added in a
DEL-07-07 tranche, Diagnostics panel, diagnostic interpretation), COVERS 5
(missing-data panel, status labels, status bar chips, toolbar run/stop, r2
smoke journeys), NOT_MINE 476. The reverse pass did not change my view of any
sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over my three forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These
dispositions are agent judgments, not owner rulings, and make no release,
approval, compliance or certification claim.
