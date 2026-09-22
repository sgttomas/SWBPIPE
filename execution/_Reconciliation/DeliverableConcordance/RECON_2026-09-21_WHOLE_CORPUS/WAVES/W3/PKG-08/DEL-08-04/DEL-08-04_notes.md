# DEL-08-04 Result export format — worker notes (W3, PKG-08, worker G2)

Forward ledger sealed: `DEL-08-04_SEAL.txt`. Evidence read from the freeze
checkout at `00115c719` only. Agent judgments, not owner rulings.

## Path aliases

- Project-root evidence tokens (`schemas/…`, `core/…`, `apps/…`, `tests/…`,
  `fixtures/…`) resolve under `projects/chirality-piping/`.
- The deliverable folder is cited with its full spaced path
  (`projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/…`).
- `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…` parity
  records are repository-root paths.
- `tools/validation/validate_claims_language.py` is the repository-root file
  (no project copy exists).
- The desktop product writer is
  `apps/desktop/src/features/result-export/resultExportAdapter.ts`
  (`buildCurrentResultExport`); the Rust contract is
  `core/reporting/result_export/src/lib.rs` (crate `open_pipe_stress_result_export`).

## Judgment calls

- **Product caller.** The Rust crate has product callers (report package via
  the Tauri `save_report_package` command; headless runner binding). The
  desktop result JSON is written by the TypeScript writer, mounted in
  `App.tsx` Exports section. No row needed `PRODUCT_CALLER: NONE`.
- **FG-DEL-08-04-01 (R4, V-4):** the desktop writer never populates
  `rule_pack_refs`, although the analysis status it copies can be
  `USER_RULE_CHECKED`/`USER_RULE_FAILED` after a GUI rule-check run
  (`previewService.appliedRuleCheckStatus`). Judged PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE (MEDIUM): "where applicable" arguably applies once a rule
  check ran.
- **FG-DEL-08-04-02 (R5, CLM-025.r04):** the desktop writer sets every
  exported diagnostic's `class` to `ASSUMPTION_WARNING` and `remediation` to a
  fixed sentence, discarding the source diagnostic's optional
  `class`/`diagnostic_class`/`remediation`. Judged POSSIBLE_DEFECT (MEDIUM),
  R5 tier PROJECT_BASELINE (AB-00-06), AuthorityNeeded REVIEW. It may be a
  deliberate enum-safe mapping; a reviewer should decide.
- **V-3:** crate emits a missing-provenance diagnostic but no test asserts
  it → PARTIALLY_IMPLEMENTED.
- **V-7:** the DEC-081 claims validator scans desktop sources, docs and SOWs,
  not `fixtures/`; no protected-content scan over the public result fixtures
  was located → PARTIALLY_IMPLEMENTED · INVARIANT · IP_DATA;CLAIMS.
- **R9/R12 (no-bypass):** judged ALIGNED for the result-export contract and
  its consumers; other export adapters (PCF, CAEPIPE, SDK) consume
  `MechanicsResult` directly under their own deliverables' contracts. MEDIUM
  confidence; a verifier may read AB-00-07 more broadly.
- **CLM-006, AC-001:** judged against the schema contract, which is their
  subject; writer-level fidelity is carried on CLM-011.r04/r05.
- **Tables split:** CLM-003, CLM-004, CLM-011, CLM-012, CLM-020, CLM-025
  (rows differed). Unsplit: CLM-006, CLM-018, CLM-026, CLM-027 (rows share a
  disposition). CLM-005 (bulleted, no `.rNN`) uses `.s01`–`.s04`.
- **Sub-claims:** `SOW.s01` (front matter pin rev 0.8),
  `CONTEXT#architecture-basis-injection.s01` (PKG-00 SEMANTIC_READY),
  `MEMORY.s01` (undated Boundary Decisions / Remaining TBDs).
- **Remaining R01** (target-format unit-conversion witnesses): text accurate,
  open, no governing row carries it → takes PARTIALLY_IMPLEMENTED itself (F2).

## Canonical departures

None. CP-03 blocks carry CP-02 fields (CanonicalSituation written as CP-03).
CP-04 rename residue kept STALE_REVIEW_OR_EVIDENCE per CP-04 although the SOW
phrase is migration-era text (F3 would suggest STALE_SETUP_SPECIFICATION; the
rename, not the origin, made it stale). Flagged here as convention friction.

## Convention friction

- F3 versus CP-04 class for migration-era text made stale by a later rename
  (above).
- CP-04 variant for `openpipestress_jcs_ijson_v1` (frozen-contract variant):
  used by this deliverable's writer but owned by the hashing contract; noted on
  the SOW SURFACE row for R3, not keyed here.

## Rename residue observed (for R3)

SOW lines 167, 307; results schema `title`/`$id`; crate name
`open_pipe_stress_result_export`; download filename prefix
`openpipestress-preview-results-`; canonicalization id
`openpipestress_jcs_ijson_v1` (frozen contract, not owned here).

## UNKNOWN rows

None.

## Reverse pass

Read the routing file only after all three seals. It did not change my view of
any sealed row. Overlapping capabilities answered specifically (F5):
RC-08-0018, 0034, 0050, 0058, 0070, 0083, 0086, 0101, 0183, 0194, 0254, 0275,
0299. The native result save (RC-08-0080) was not cited in the forward ledger;
answered PARTIAL.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-08-04/05/06: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): Piping
selects work through owner-steered work graphs, not `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
