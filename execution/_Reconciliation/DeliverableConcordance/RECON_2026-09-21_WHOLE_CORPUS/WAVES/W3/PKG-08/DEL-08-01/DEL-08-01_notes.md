# DEL-08-01 Calculation report generator — worker notes (W3, PKG-08, worker G1)

Forward ledger `DEL-08-01_forward.csv` (106 rows: 75 required keys, `.rNN`
splits of CLM-004, CLM-005, CLM-028 and CLM-030, and `.sNN` sub-claims
SOW.s01, CLM-006.s01, CLM-013.s01, CLM-029.s01,
CONTEXT#architecture-basis-injection.s01/.s02). Sealed in `DEL-08-01_SEAL.txt`.
The shared notebook is `../_WORKER_DEL-08-01_NOTES.md`.

## Path aliases

- Deliverable folder:
  `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/`.
  Cited with spaces, as the W2 validator change allows.
- Code, schemas, tests and fixtures are cited project-root (`core/…`, `apps/…`,
  `schemas/…`, `tests/…`, `fixtures/…`). Project documents are cited as
  `projects/chirality-piping/docs/…`.
- SOW parity and claim-map records live at the repository root
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`), as
  `EVIDENCE_MAP.csv` lists them. They are not under the project folder.
- Crate names in code (`open_pipe_stress_report_renderer` and others) are the
  former-name identifiers of the `core/reporting/*` crates.

## Judgment calls

- **Rule-pack references (FG-DEL-08-01-01).** The renderer prints rule-pack
  id, version, checksum, notice and completeness. The product adapter
  (`renderableReportInput.ts`) always sends `rule_pack_refs: []`. The package
  request (`reportPackageRequest.ts`) throws
  REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE unless the rule check is
  `RULE_INPUTS_INCOMPLETE`. Claims that reports *include* rule-pack metadata
  are PARTIALLY_IMPLEMENTED under F7. Constraint claims about *what* may be
  shown are ALIGNED on the engine with `PRODUCT_CALLER: NONE`.
- **Versions (FG-DEL-08-01-02).** No software/application version exists on
  any report surface. The solver stamp is only in the package's
  audit-manifest member, not in the rendered HTML/PDF. R-08-01-002 and
  CLM-028.r06 are PARTIALLY_IMPLEMENTED, tier PROJECT_BASELINE (SPEC §9;
  OPS-K-REPORT-1 named in Notes).
- **Diagnostic classes (FG-DEL-08-01-03).** The rendered diagnostics table
  shows no class column. The product derives class from severity. R-08-01-007
  is PARTIALLY_IMPLEMENTED here; the stronger "do not reclassify" finding is
  recorded in DEL-08-03.
- **Remaining R01** (.opsproj compatibility window) is accurate and open. No
  SOW row in this ledger carries the work, so under F2 the Remaining row itself
  is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · PROJECT_BASELINE. DEL-02-05 may
  own it.
- **The OUT-001 purpose statement and AC-001** are judged on the SOW contract
  text (ALIGNED). The OUT-001 matrix row and VER-001 follow CP-09.
- **R-08-01-003** (protected content) is ALIGNED. The "human review before
  public release" in its verification column is a release-time gate that has
  not yet been reached.
- **R-08-01-009** (schema-first boundary) is ALIGNED at MEDIUM confidence; see
  the reverse-pass note below.

## Canonical departures

None. CS rows follow `CANONICAL_ASSIGNMENTS.csv` exactly. The CP-03 D-41 rows
follow CP-02 (STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE).

## Convention friction

- The shifted SPEC section references are a common defect across the
  requirement SourceRef column. They are recorded once as a minted `SOW.s01`
  SUBCLAIM of the SURFACE key rather than in SURFACE Notes, because the SURFACE
  row already carries CP-04. The validator accepts `.sNN` on a SURFACE key.
- There is no "implementation" divergence layer. Code gaps use VALIDATION
  (provenance/reproducibility disclosure) or BASELINE (accepted-baseline
  structure). This mapping is recorded in the notebook.
- CP-09 is applied to VER-001, which W2 recorded as a contested cluster. This
  follows the G1 reading.
- The STATUS SURFACE row needs CP-04 (identity-rename variant, for `.opsproj`
  in Remaining). CP-05 does not apply (Last Updated 2026-08-20 equals the
  latest History entry).

## UNKNOWN rows

None.

## Reverse pass (after sealing; no edits made)

- The reverse pass confirmed that DEL-08-01 carries the report-package
  container (RC-08-0183), native save (RC-08-0130) and save controls
  (RC-08-0193), but no issued key covers them. They are answered UNKEYED,
  anchored at CLM-030.r01 / CLM-028.r07. This agrees with the forward CLM-030.r01
  row (container built here under DEC-028/DEC-057).
- The routing note on RC-08-0224 says only the contract test reads
  `report_generator.schema.yaml`. Runtime inputs are typed serde structs
  mirroring the schema, not schema-validated. A verifier reading
  "schema-first" as runtime schema validation might move R-08-01-009 and
  CLM-004.r05 (sealed ALIGNED, MEDIUM) to PARTIALLY_IMPLEMENTED. I would
  keep them ALIGNED, because the serde types are pinned to the schema by the
  fixture tests. I flag it for the verifier.
- The routing note on RC-08-0296 ("no direct application caller") does not
  change F7. `validate_report` runs in the product path through the
  renderer's `render_calculation_report`, reached from the Tauri command.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed forward ledgers
(DEL-08-01/02/03): **PASS, 0 findings**.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
