# DEL-07-08 Design-authoring state and comparison workspace — worker notes (W1, PKG-07, worker G3)

Forward ledger `DEL-07-08_forward.csv`: 96 rows (78 required keys, 9 canonical
assignments inherited without departure, `.rNN` splits of CLM-004 and CLM-005,
the required per-requirement verification keys under CLM-013, three `.s01`
sub-claims). Sealed in `DEL-07-08_SEAL.txt`. Evidence read from the frozen
checkout at `00115c719` only. Not an R0 pilot. Shared situation rules are in
`../_WORKER_DEL-07-05_NOTES.md`.

## Path aliases

- Deliverable-local files are cited in `ContextRefs` because their path
  contains spaces.
- The SOW cites `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` /
  DAG2-RD-015. That file does not exist. The review is at
  `projects/chirality-piping/execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md`
  (line 64 carries DAG2-RD-015).
- Parity records: root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-07-08/`.
- `core_contract_evidence` block: `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx`
  near L203.

## Judgment calls

- **FG-DEL-07-08-01, unimplemented SOW-076 surfaces.** These exist: the
  knowledge panel, Diagnostics and missing-data panels, ComparisonPanel,
  DiffPreviewPanel and OperationLedgerPanel. Graphical comparison overlays are
  descriptor-only: the packet says `descriptor_only_available`, and the
  viewport draws no comparison overlay. The state/run browser shows only the
  current run (count 0 or 1) plus reopened historical-run evidence. So
  REQ-07-08-001/005, CLM-004.r01/r02, CLM-006, CLM-010, CLM-013/REQ-07-08-005
  and the CONTEXT description and anticipated artifacts are
  PARTIALLY_IMPLEMENTED · PARTIAL_SLICE.
- **FG-DEL-07-08-02, possible defect (verifier attention).** The workspace
  packet embeds a fixed `core_contract_evidence` block (records=2, warnings=1,
  states=2, runs=2, overlays=5, …) copied from the Python contract fixture.
  The "Core contract" line shows these values for every project, next to live
  values. This contradicts REQ-07-08-002 and CLM-025 principle 1 ("consume
  upstream contracts … rather than inventing placeholder semantics"). Both rows
  are IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · LOCAL_DESIGN · AuthorityNeeded
  REVIEW, confidence MEDIUM. A benign reading is that the block is labelled
  fixture evidence; it is not labelled that way in the UI.
- CLM-005.r01 → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT: the SOW says 21 rows
  of the DAG-006 mirror, all ACTIVE. The frozen register has 29 rows: 27 ACTIVE
  (4 anchors, 7 architecture-basis, 14 DAG-002, 2 PKG-02) and 2 RETIRED. The
  approved graph is DAG-010.
- CLM-006.s01 and CLM-026 (state library and similar TBDs) → CP-10, the same
  as DEL-07-05 CLM-005.r02. CLM-004.r04 (exact dependency versions TBD) stays
  ALIGNED, because DEC-012 keeps versions an implementation-level TBD.
- The SOW SURFACE row is STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE. The
  surface is written from the setup workflow's viewpoint ("setup documents
  only", "no product code", "no implementation records exist"), but
  implementation has existed since 2026-05-09. There is no rename residue on
  this SOW. The frontmatter pin is `SOW.s01` (CP-02).
- CLM-012 and CLM-018 → CP-02 (rev 0.7 pin, PRD v0.2, DAG-006 approval record).
- STATUS SURFACE → CP-05 (two 2026-09-19 history entries after Last Updated
  2026-09-13). `STATUS#remaining` is pre-typed NON_NORMATIVE (empty, shared
  body) and I kept the pre-type. Observation, not relied on (A4): no residual
  is recorded although FG-01 and FG-02 are open.
- For R3 (no forward key): `DesignWorkspacePanel.tsx` emits former-name
  identifiers (`openpipestress-preview-design-workspace-*.json`,
  `openpipestress.technical_preview.design_authoring_comparison_workspace`).

## Canonical departures

None. CS-03 (SCA-002 refresh note) and CS-02 are inherited. CS-04 carries
`.s01` as for DEL-07-05.

## Convention friction

- `STATUS#remaining` is pre-typed NON_NORMATIVE, so an empty section cannot be
  judged even when real gaps exist; C6(c) limits REMAINING_STATE_MISMATCH to
  `#remaining/` items. The gap is recorded in Notes only.
- CLM-013's rows are issued as required keys named after the requirements
  (`CLM-013/REQ-07-08-00n`). They are verification approaches, typed ACCEPTANCE.

## UNKNOWN rows

None.

## Reverse pass

487 routed capabilities: CLAIMED_BY 5 (DesignWorkspacePanel, its JSON
export, `core/gui/design_workspace`, ComparisonPanel, KnowledgePanel), PARTIAL 3
(diff preview, operation ledger, historical-run display), COVERS 7, NOT_MINE 472.

The reverse pass sharpened one observation without changing a sealed row.
DEL-07-08's STATUS history and MEMORY record work on the workspace session
extraction, the project-handler hash-mismatch repairs, the undo/redo guard and
saved-run verification (capabilities 0143, 0161, 0042, 0374, 0417). No SOW
claim covers these. I answered COVERS against `STATUS#history`/`MEMORY`, not
UNKEYED, because SOW-076 does not describe session state or project
persistence. R3 should decide whether this work was filed under the wrong
deliverable or is undocumented scope.

## Batch consistency

`validate_ledger_v2.py --batch` over my three forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These
dispositions are agent judgments, not owner rulings, and make no release,
approval, compliance or certification claim.
