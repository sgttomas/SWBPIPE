# Exact-route report sections and report provenance (W-11)

**Basis:** `origin/main` at `67e9e43cc5` (2026-10-24), after W-3 (#1553) and
model document 0.7.0 (#1564). Revision 1, design before implementation. W-18
(DEL-08-01 Design `report-package-current-results.md`) is the package's own
design; this design adds what the exact routes need on top of it.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **Ruling 10A (owner, 2026-10-21):** reports of exact-route results
  (`pressure-2`, `pressure-source-1`) stay refused until DEL-08-01 designs
  their limitation sections. DEL-04-07 Design `desktop-exact-outputs.md` §4.5:
  "Opening them needs DEL-08-01's own design of those sections and of the PRD
  §19.3 notice's place." Opening returns to the owner (completeness map §4.2).
  **This design builds the sections behind the existing gate. It opens
  nothing.**
- **FR-REP-001 (Must)**, PRD §19.2: results, assumptions, warnings, solver
  settings; FR-REP-002: mechanics, user-rule checks and human review kept
  distinct.
- **FR-REP-003 (Must):** "Report identifies sources or source notes for
  user-supplied data where available." Today the report carries the session,
  component and spring-hanger records only (completeness map, FR-REP-003 row).
- **Claims registry** (`docs/claims_registry.md` §1 BS-ACCEPT, §4):
  BS-ACCEPT text is on no product surface or report body (DEC-100, DEC-107).
  Where it is withdrawn, nothing is written in its place. Deliverable-specific
  exclusions are scope statements and stay ordinary prose. The PRD §19.3 notice
  is the report's composite anchor and is unchanged.

## 1. Where things stand

- **Refusals.** The output policy refuses `report`, `rendered-report` and
  `report-package` for `pressure` and `pressure_source` with
  `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` (`outputPolicy.ts`, `exactRouteSurfaces`).
  The rendered report checks it at `RenderedReportPanel.tsx:171`.
- **Limitations.** The renderable report has one fixed limitation, the
  technical-preview statement (`renderableReportInput.ts:547-562`). It carries
  none of the exact identity's published limitations.
- **What the exact producers publish.** `formulation_basis.limitations` are
  whole static strings (`core/product_physics/src/exact_admission.rs:72-115`,
  with `joint.rs:164` `CONNECTOR_LIMITATION`):
  - the identity sentence, with the admitted families and the D-5 section bases;
  - pressure (signed internal differential; "external-pressure stability and
    collapse are not assessed");
  - "Not modelled: …" (flow momentum and transient pressure, Bourdon effect,
    pressure stiffening, ovalization);
  - contribution ledgers, line loads, support attribution, the normal-stress
    maximum's enclosure;
  - the source paragraph;
  - five load/reference-state sentences, when present, and the connector
    limitation, when connectors are present.
- **Why they cannot simply be shown** (`desktop-exact-outputs.md` §5.3):
  - for the exact routes they are not bound to the semantic table; the reader
    checks only that they are non-empty strings (`semantic_contract.rs:605-611`);
  - the source paragraph ends with an acceptance-boundary sentence: "No code
    compliance or professional acceptance is produced, and actual numerical
    admission remains separate." The table's `supported_profile_limitations`
    carry similar sentences.
- **Headlines.** An exact result publishes one stress maximum per section
  basis (`max_open_formula_stress_nominal_section`, `…_reduced_section`;
  `ResultsPanel.tsx:219-240`). The report has no headline section; its result
  rows carry no basis label.
- **Provenance.** `provenance_notes` holds the session, component and
  spring-hanger records (`renderableReportInput.ts:537-545`). Materials, sections
  and library records are absent. Model document 0.7.0 gives materials,
  sections, components and supports an optional `provenance_record` with an
  optional `library_ref` (DEL-02-01 Design `model-document-0.7.0.md` §3.5).

## 2. Exact-route limitation sections

### 2.1 A closed statement table

`features/report/exactReportLimitations.ts` holds a closed table. Each entry is
**one producer limitation string, matched exactly, with its report statement**:
- **Scope statements are copied verbatim.** These are the identity, pressure,
  "Not modelled", ledger, line-load, attribution, enclosure, load/reference-state
  and connector strings. They are deliverable scope statements (registry §4).
- **The source paragraphs (`pressure-2` and `pressure-source-1`) omit their
  last sentence.** The sentence is "No code compliance or professional
  acceptance is produced, and actual numerical admission remains separate."
  Nothing replaces it (DEC-107). Numerical admission is stated by the report's
  standing line (§3.3).
- **Fail closed.** A result whose producer limitations include a string the
  table lacks is refused for every report surface, with
  `REPORT-EXACT-LIMITATION-UNREGISTERED`, naming the identity. A producer text
  change therefore needs a reviewed table change before it reaches a report.
- **Coverage.** A Rust test writes every limitation list the producer can emit
  to a committed fixture, `fixtures/reports/exact_route_limitations.json`:
  both identities × load/reference states × connectors, with the selected
  flag. It writes the lists through
  `pressure_v3_formulation_basis_with_source` and fails if the fixture is
  stale. A vitest test checks that the table covers the fixture exactly, with
  no extra entries.
- **Claims check.** The table is TypeScript product source, so the GEN-13 lint
  (`tools/validation/validate_claims_language.py`) scans it. A test also checks
  that no report statement contains a BS-ACCEPT text or short variant.

### 2.2 In the report

- Each statement becomes one `report_sections.limitations[]` entry:
  - `limitation_id` `limitation:<run>:exact:<n>`, in the producer's order;
  - `source` `{semantic_contract, <identity>}`;
  - `affected_scope` `{analysis_run, <run>}`;
  - the technical-preview entry's `effect`;
  - the session provenance.
- **A reference entry.** The run's semantic table is bound by id and sha256
  (`run.reproducibility.semantic_contract`): "The full profile limitations of
  <identity> are bound by reference: <table id>, sha256 <value>." Its
  `supported_profile_limitations` travel by that reference, as they already do
  for the exports (`desktop-exact-outputs.md` §5.3).
- The technical-preview limitation stays, first, on every route.
- **Notices.** The PRD §19.3 notice keeps its place; the renderer emits it
  (`report_renderer`, §19.3 section), and no exact-route text is added beside
  it. The local-private notice (W-3) is unchanged. There is no new notice.

## 3. Other exact-route sections

### 3.1 Headlines by section basis

- A new renderer input, `headlines[]`, renders as a "Stress headlines" section
  before the result rows. Each entry has a label, a quantity with its unit, a
  basis label and a source ref.
- For an exact result, there is one entry per section basis present in
  `summary`, with the `ResultsPanel` labels: "Nominal section (nominal wall)" and
  "Reduced section (wall minus mill tolerance and corrosion allowance)". No
  governing basis is chosen.
- A withheld headline is stated with the results panel's own withheld text,
  never as a value.
- For a pressure-free result, the single `max_open_formula_stress` headline is
  shown with the registered `N_HEADLINE` qualifier (`knownSemanticLimitations.ts:25`).
- `report_generator::RenderableReportInput` gains the optional field, and
  `schemas/report_generator.schema.yaml` its matching optional property. An
  absent field renders nothing, so the existing witnesses are unchanged.

### 3.2 Result rows

Exact result rows carry their section basis in the label: "(nominal section)"
or "(reduced section)", from the row metadata the reader classifies
(`readerRowSemantics`). A row the reader does not classify is refused for the
report, as the result export refuses it.

### 3.3 Standing and load/reference states

- **Standing.** One line gives the Rust reader's numerical standing in its
  registered display form (`numericalResultStanding`), as the results panel
  shows it. A report is built only at eligible standing with the live native
  invocation, the same admission the result export uses
  (`buildCurrentResultExport`).
- **Load/reference states.** A `pressure-2` result carrying resolved
  load/reference states gets the export's disclosure sentence
  (`N_LOAD_REFERENCE_EXPORT`, worded for the report): "This result includes
  resolved load/reference states (reference configuration …; load cases …;
  history …)." `pressure-source-1` with load/reference states is never
  eligible, so it never reaches a report.

## 4. Report provenance: materials, sections and library records (FR-REP-003, every route)

- **Materials and sections.**
  - Each material and section gets a `provenance_notes` record.
  - A record with a `provenance_record` maps its seven PRD §20.2 fields as they
    stand. A missing field is "not recorded", never invented.
  - Without one, the record is the session provenance with `source_location`
    `model:<entity id>` and the entity's `provenance` string as the source
    note.
  - The same holds for components and supports with a `provenance_record`,
    which today use the session record (`renderableReportInput.ts:177-193`).
- **Library records.** A new section lists each distinct `library_ref`:
  - `library_kind`, `library_id`, `library_version` and the entities that
    reference it;
  - the desktop's resolution warnings, `MODEL-LIBRARY-REF-UNRESOLVED` and
    `MODEL-LIBRARY-REF-VERSION-CHANGED`, as diagnostics.

  This is safe metadata only. No library payload enters the report (R-08-01-008,
  REXC-REQ-009).
- **Disposition.** A record with `protected_suspected`, `quarantined` or
  `rejected` is blocked by the redaction screener in every context, as for
  every report. `private_only` and `unknown` records render in a local-private
  report.
- **Dependency.** The slots exist on main (0.7.0), and the desktop sets
  `library_ref` when it applies a library record (W-13 S2b, #1566). Resolution
  and its warnings use the desktop's existing store resolver; the report adds no
  second resolution rule.

## 5. What opening would change (for the owner)

Opening is one policy edit, plus the Rust identity list of the W-18 package
boundary:
- `outputPolicy.ts` `exactRouteSurfaces` adds `rendered-report` and
  `report-package` to the opened surfaces for `pressure` and `pressure_source`;
- the W-18 package boundary admits `pressure-2` and `pressure-source-1`;
- the user guide and DEL-04-08 REQ-006 (c) text say so. That text is commitment
  text, so it takes the wording route.

What stays refused:
- The **report JSON packet** (`report`, `ReportPanel.tsx`) picks fixed demo ids
  and one headline (`selectedResultRefs`, `:290-300`). It stays refused on the
  exact routes. Its retirement or redesign is a separate item.
- The **handoff preview** (`handoff`) stays refused for exact results. The model
  handoff (W-6b) needs no result.

The evidence for the return: the coverage fixture and table test, the claims
check, a rendered local-private exact report (HTML and PDF) from the committed
exact fixtures, and a 2.0.0 package from the same inputs, built through the
test seam below the gate.

## 6. Verification

- **Rust:**
  - the limitation fixture is current with `exact_admission.rs`;
  - the renderer renders `headlines[]`, the library section and the extra
    provenance records;
  - an absent `headlines[]` leaves existing output byte-identical
    (`report_renderer`, `pdf_emitter`, `report_package` tests).
- **vitest:**
  - table coverage and the fail-closed refusal;
  - the source-paragraph omission;
  - no BS-ACCEPT text;
  - exact headlines and row labels;
  - the standing and load/reference lines;
  - the provenance records for materials, sections and libraries;
  - the policy still refusing every exact report surface.
- **pytest:** `tests/test_report_protected_content_linter.py`, the headless
  determinism test, and `schemas/report_generator.schema.yaml` validation.
- **GEN-13:** `tools/validation/validate_claims_language.py`.
- **Review:** an independent Codex round. The limitation text and the
  claims-language check get specific scrutiny.

## 7. Reconciliation

- **DEL-08-01:** the "Current implementation" paragraph (:81), this design, and
  `deliverable.yaml`.
- **DEL-08-03** (provenance section): the FR-REP-003 records; pointers.
- **DEL-04-07 Design §4.5:** a pointer to this design. Its refusal stands until
  the owner opens it.
- **The completeness map:** FR-REP-001 and FR-REP-003, gap 1, and the W-11 row.
