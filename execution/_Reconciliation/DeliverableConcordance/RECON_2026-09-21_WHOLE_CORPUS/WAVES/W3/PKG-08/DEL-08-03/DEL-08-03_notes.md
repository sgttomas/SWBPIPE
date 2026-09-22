# DEL-08-03 Warnings, assumptions, and provenance report section — worker notes (W3, PKG-08, worker G1)

Forward ledger `DEL-08-03_forward.csv` (94 rows): 75 required keys, `.rNN`
splits of CLM-006, CLM-026 and CLM-029, and `.sNN` sub-claims SOW.s01 and
CONTEXT#architecture-basis-injection.s01. Sealed in `DEL-08-03_SEAL.txt`.
The shared notebook is `../_WORKER_DEL-08-01_NOTES.md`.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section/`.
- Section crate `core/reporting/report_sections/`
  (`open_pipe_stress_report_sections`) and schema
  `schemas/report_sections.schema.yaml`.
- The rendered section is the `warnings_assumptions_provenance` block of
  `core/reporting/report_renderer/src/lib.rs::assemble_report_sections`.
- The product projection is
  `apps/desktop/src/features/report/renderableReportInput.ts`
  (`diagnosticsForSections`, component and hanger provenance builders).

## Judgment calls

- **Adapter-derived diagnostic class and remediation (FG-DEL-08-03-01), possible defect.**
  - Product-physics `Diagnostic` has no class and no remediation.
  - The desktop adapter derives the class from severity: blocking/error
    becomes SOLVE_BLOCKING and everything else ASSUMPTION_WARNING. It also
    inserts a fixed remediation sentence.
  - `reportPackageRequest.ts::diagnosticClass` does the same for the package
    result envelope.
  - The SOW forbids reclassification without evidence and requires TBD rather
    than inference (CLM-006, CLM-006.r02, CLM-020 step 2, CLM-026.r03,
    CLM-027, REQ-002).
  - These rows are IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
    PROJECT_BASELINE · BASELINE · OWNER, at MEDIUM confidence. It is an R4
    code-change candidate.
- **Rendered-field omission (FG-DEL-08-03-02).**
  - The section record carries every trace field and validation enforces
    them.
  - The rendered diagnostics table omits class, source, affected object and
    provenance.
  - The provenance-notes table omits redistribution and review status.
  - REQ-003, CLM-006.r05 and CLM-029.r05 are PARTIALLY_IMPLEMENTED.
- **Rule packs (FG-DEL-08-03-03).** The product binds no rule-pack
  references or values. REQ-004, CLM-006.r04 and CLM-029.r04 are
  PARTIALLY_IMPLEMENTED under F7.
- **Versions (FG-DEL-08-03-04).** REQ-009: software/solver versions are
  neither referenced nor marked TBD.
- **Assumptions (FG-DEL-08-03-05).** The engine and renderer support
  assumptions, but the product adapter passes `assumptions: []`: the product
  model has no assumption register. CLM-006.r03, CLM-029.r03 and the
  CONTEXT description are PARTIALLY_IMPLEMENTED (F7, LOCAL_DESIGN).
- **REQ-011** (fallback if the lint gate is unavailable) is ALIGNED. The
  contingency was never triggered, because the linter exists and gates the
  section.
- **CLM-004** (attributes) and **CLM-014** (verification list) are assessed
  directly rather than split. CLM-004's rows are all accurate. CLM-014's
  per-class fixture and version-reference checks were not located.
- **Scope of the Still-TBD list.** The CONTEXT architecture-basis Still-TBD
  list names nothing this section depends on, so no `.s02` row was minted
  (unlike DEL-08-01/02, which touch the container).

## Canonical departures

None.

## Convention friction

- The SPEC pointer drift (Section 8 → 9, Section 7 → 8) is minted as `SOW.s01`
  on the SURFACE key, as in DEL-08-01/02.
- Rows forbidding inference (IMPLEMENTED_DIFFERENTLY) and rows about rendering
  fewer fields (PARTIALLY_IMPLEMENTED) touch the same code seam. I kept them in
  separate finding groups so R3 can cluster the defect apart from the display
  gap.

## UNKNOWN rows

None.

## Reverse pass (after sealing; no edits made)

- The routing file lists `report_sections` (RC-08-0115) and its schema
  (RC-08-0111) with the note "Consumed by renderer, PDF emitter and package
  crates only". This agrees with the forward view that the section reaches
  the product only through the DEL-08-01 renderer.
- I answered RC-08-0053 (renderer input builder) and RC-08-0160 (renderer)
  PARTIAL for the section-record and section-rendering parts. The forward
  rows would not change.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-08-01/02/03: **PASS, 0 findings**.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Piping
selects work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
