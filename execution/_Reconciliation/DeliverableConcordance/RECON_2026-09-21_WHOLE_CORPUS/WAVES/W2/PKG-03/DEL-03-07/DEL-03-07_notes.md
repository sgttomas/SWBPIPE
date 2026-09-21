# DEL-03-07 notes — gate wave W2, PKG-03, worker G3

Deliverable: Public/private library import provenance checker. Forward ledger
103 rows (53 required keys, 7 canonical assignments inherited). Sealed forward
SHA-256 `e339f3da16356e76f10c36469e516a5d8cfadf47b2a6b482bc20dc3fc5b01c1d`.
Not an R0 pilot, so no calibration repairs applied.

## Path aliases

- Code, tests and fixtures are cited project-root (`core/…`, `apps/…`,
  `tests/…`, `fixtures/…`).
- The SOW parity records listed in `EVIDENCE_MAP.csv` live under the
  repository-root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`,
  not under the project. They are cited in that root form.
- Deliverable-folder files (`Review_Findings.csv`, `MEMORY.md`, `_STATUS.md`,
  run records) have spaces in their path. Part D bars spaces in evidence
  columns, so they are cited in `ContextRefs` and described in Notes even where
  they act as evidence (the review-finding state in FG-DEL-03-07-03).
- The runtime crate is `core/library_import/library_import_document`
  (package `open_pipe_stress_library_import_document`). The desktop command is
  `validate_library_import` in `apps/desktop/src-tauri/src/lib.rs`.

## Judgment calls

- **FG-DEL-03-07-01: runtime port not in the implementation listings**
  (CLM-005, CLM-012, CLM-023; `IMPLEMENTED_UNDOCUMENTED · DOC_BEHIND_CODE ·
  LOCAL_DESIGN`). The SOW was reconciled on 2026-06-05. The 2026-06-13 tranche
  (run records in the deliverable folder, MEMORY) ported the contract to Rust,
  added 11 unit and 8 parity tests (gate log), and exposed it through the
  desktop command and the private-store gate. None of that is in the SOW.
  I chose `DOC_BEHIND_CODE` over `SCOPE_GREW_BY_DIRECTION` because the port
  realizes the same contract rather than extending it.
- **FG-DEL-03-07-02: unit check covers only magnitude objects** (CLM-003.r06,
  CLM-009.r05; `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT ·
  BASELINE`, MEDIUM). `_validate_nested_values` and the Rust mirror inspect
  only dicts that carry a `magnitude` key. A bare numeric field is never
  checked, and no schema validation runs for material, section or component
  payloads, so an accepted record can hold a unitless number. I chose tier
  INVARIANT because the rows restate OPS-K-UNIT-1. The unit invariant has no
  dedicated layer, so I used BASELINE. The verifier may prefer
  `PROJECT_BASELINE`; DEL-03-08 RQ-003 uses the same choice.
- **FG-DEL-03-07-03: review findings declared pending** (CLM-005.s01,
  CLM-012.s01, CLM-017.r07; `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT`).
  `Review_Findings.csv` has both findings `ACCEPT_AS_IS / RESOLVED` (human
  Gate A, 2026-06-05). The text was first declared on 2026-06-04, so it takes
  the later-declaration class.
- **CLM-009.r03 (R3)**: `PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE ·
  LOCAL_DESIGN`. R3 lists rule-pack data, but the checker has no rule-pack
  kind. The privacy boundary is not shown breached, so under F8 the tier
  follows the record gap. I name OPS-K-PRIV-1 in Notes.
- **CLM-009.r06 (R6)**: `ALIGNED`, MEDIUM. The named Python test covers the
  diagnostic fields. The "no bypass" element is shown by the desktop store
  tests (save and open both re-validate) rather than by the named test.
- **Hanger kind**: the module accepts `library_kind="hanger"`. DEC-103 records
  that extension in the DEL-07-09 coverage ledger, so I kept the three-kind
  statements (CLM-003.r01, CLM-008, CLM-016) `ALIGNED`.
- **Desktop Libraries panel**: its run record cites DEL-03-07 as authority,
  but the SOW excludes GUI work (CLM-008). The forward pass leaves that
  exclusion `ALIGNED`, and the reverse pass answers RC-03-0288 `COVERS`.
- **MEMORY.s01**: the undated "Open Items" list reads as a current declaration
  (C1). Its "UI/editor presentation … future GUI work" item was overtaken
  on 2026-06-13. The text was first present at `7bee9ae41`, so it takes
  `STALE_SETUP_SPECIFICATION` (F3).
- **CONTEXT#architecture-basis-injection**: CS-04 covers the pin. I added
  two sub-claims, `.s01` (PKG-00 `SEMANTIC_READY`; all PKG-00 deliverables are
  IN_PROGRESS, and D-43 made PKG-00 reference context) and `.s02` (the "Still
  TBD" list: package/container superseded by DEC-017, import/export formats
  decomposed through PKG-17 per OI-004). Both use F3 setup origin. The body is
  shared by 8 PKG-03 deliverables (SharedTextCount 8), so the other PKG-03
  workers' treatment should be compared.
- **SOW surface**: the frontmatter `decomposition_basis` pins commit
  `69ac259a`, whose decomposition is revision 0.8, so the row is CP-02. The
  SOW text has no former-name residue.
- **SourceReliability** is `NOT_APPLICABLE` throughout. The evidence is tests
  and run records, and no validation asset is cited.

## Canonical departures

None. All 7 CS rows are inherited unchanged. CP-01, CP-02 and CP-09 are applied
as written.

## Convention friction

- Spaces in deliverable-folder paths keep review and run records out of the
  evidence columns (see Path aliases).
- F7 marker: several `ALIGNED` rows cite only the Python module
  (`provenance_checker.py`), which has no product caller of its own. Examples
  are CLM-002.r07, CLM-011, CLM-014, CLM-015, CLM-024.r01, CLM-024.r02,
  CLM-026.r01, CLM-026.r02, AC-001 and CONTEXT#anticipated-artifacts. Its
  runtime port does have product callers (desktop command and store), but
  those rows omit `PRODUCT_CALLER: NONE`. The claims are about the checker
  contract, and the port carries that contract into the product. I note the
  omission here because the sealed file cannot change.
- Unit-invariant layer: C5 has no layer for units, so I used BASELINE (see
  FG-02).

## UNKNOWN rows

None.

## Reverse pass

Answers for 376 routed capabilities: 4 `CLAIMED_BY` (RC-03-0107 Python
checker, RC-03-0072 runtime crate and command, RC-03-0256 checker README,
RC-03-0121 crate README), 1 `PARTIAL` (RC-03-0198 hanger path: shared
disposition rules only; hanger family under DEL-07-09 per DEC-103), and 3
`COVERS` (RC-03-0288 Libraries panel; RC-03-0109 and RC-03-0162 fixture
folders used by the provenance tests). The rest are `NOT_MINE`. Every
capability whose EntryPoints hit a path the forward ledger cites has a specific
reason (F5): the nine other `src-tauri/src/lib.rs` commands, CONTRACT,
registers, and RC-03-0106 (the entry-drafting panel).

Did the reverse pass change my view of anything sealed? Mostly no. Two
observations:
- The RC-03-0072 routing row confirms FG-DEL-03-07-01, and the reverse answer
  claims the crate.
- The routing note on RC-03-0198 says the Rust port has no hanger path. The
  frozen `library_import_document/src/lib.rs` does contain
  `validate_hanger_import` (from about line 797). This does not affect any
  sealed row. It is recorded for R3.

## Rename residue observation (for R3)

Active code identifiers carrying the former name sit in this deliverable's
code: the crate package `open_pipe_stress_library_import_document`, the desktop
envelope `document_kind` `openpipestress.library_import.validation`, and the
Python module docstring. No DEL-03-07 surface names them except MEMORY
history, which is accurate history, so CP-04 has no surface row here. They are
code-change candidates for the R4 rename class.

## Batch consistency

`--batch` over DEL-03-07 and DEL-03-08: PASS, 0 consistency findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` (C9). Since 2026-09-19 Piping
selects work through owner-steered work graphs, not `## Remaining`.

These dispositions are agent judgments, not owner rulings. No release,
approval, compliance or certification claim is made or implied.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
