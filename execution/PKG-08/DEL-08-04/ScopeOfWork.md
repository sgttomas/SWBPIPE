---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-04
package_id: PKG-08
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-046]
package_objective_refs: [OBJ-007, OBJ-009]
---

# Scope of Work — DEL-08-04 Result export format

## Purpose and Objective Traceability

A solved result can leave the application in a form that another person or program can check. This deliverable contributes the two forms. The first is the result export document: one JSON document that carries a result as the solve stated it, with its units, diagnostics, statuses and the identities of the model and run behind it, for review, regression comparison, the report package, the headless runner and downstream adapters. The second is a CSV file of any results table the desktop shows, whose numbers are the table's numbers, unrounded, with enough written above them to say what they are.

It serves SOW-046 under OBJ-007 and OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-08, row DEL-08-04; the stress-neutral packages and external-harness parsing that SOW-046 also names are DEL-17-05's and DEL-17-06's), and PRD §5.6 (unit safety), §19.2 (the result content a report draws on), §21.2 (prohibited claims) and §22.4 (regression of run reproducibility and comparison output). In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable of exit criterion 5: every results table in the desktop exports to CSV.

- **OUT-001** — The result export document: its schema in one current version, the result envelope, the assembly of a document from a received result, the reader-side form check, and the desktop's Export Result JSON (REQ-001 to REQ-013, REQ-015 to REQ-017).
- **OUT-002** — CSV export of every results table in the desktop, comparison tables included (REQ-014).

## Deliverable Definition — Ontology

The **result envelope** is the body of the export document. Its header names the schema version, the envelope, the model and run it belongs to and the solver version. It carries the result values with their units and dimensions, the diagnostics, the provenance and reproducibility references (model hash, input manifest, rule-pack checksums, source notes) and the analysis statuses.

A **received result** is what a solve published: its source statement, its rows, the model it solved and the request. An export document copies those members as received and never restates them. Each copied member is identified by the SHA-256 of its canonical JSON value, not by the bytes it arrived in; the desktop's native save keeps the exact received bytes.

The **analysis route** of a result is its semantic contract identifier, which says which solve route and semantics produced it: the pressure-free route, the exact pressure route in its two forms, and their retained-precision successors. The export preserves the class and receipt each row was verified under and re-derives neither.

A **results table** is a table of solved results or of a comparison shown in the desktop. Each has a **view model**, the data the panel renders from. A table's **adapter** reads that view model and hands rows to the one **serializer**, which writes the file. The **preamble** is the block of key and value rows at the top of a CSV file that says what the table is.

The report's display rule has an **exact step** (the value in the displayed unit, as exact decimal text) and a rounding step. A CSV file takes the exact step and stops there.

Boundary:

- **CLM-001** — This deliverable's export formats are JSON, as the result export document, and CSV of the desktop's results tables. Other formats, such as spreadsheet, HDF5, neutral-file and external-tool formats, are not this deliverable's; the stress-neutral CSV and JSON package is DEL-17-06's and the CAEPIPE formats are PKG-17's.
- **CLM-002** — What a result means belongs to its producers: the solve to PKG-04, the semantic tables, load and stress rows to DEL-05-02 and DEL-05-03, retained-precision verification to DEL-04-08, statuses to DEL-05-04. In the result-export crate this deliverable's part is the envelope and the derivative assembly; the evidence and reader modules beside them are their producers'.
- **CLM-003** — The results tables and their view models are DEL-07-05's, the comparison views' are DEL-07-08's, and the hanger table's is DEL-04-11's. The CSV adapters that sit beside them, the serializer, the export control and the save are this deliverable's. The report-wide display rule is DEL-08-01's; the display-unit converter is DEL-02-02's.
- **CLM-004** — The export route catalogue, export contexts and screening are DEL-12-02's. This deliverable's part is the native save of result JSON and CSV on the routes that catalogue admits. File names follow the desktop shell's export file-name convention (DEL-07-11).
- **CLM-005** — In the report-package crate, the form check of a result document and the schema evaluator it uses are this deliverable's; the package is DEL-08-01's.

## Completion and Reliance Basis — Epistemology

Not decided: the version of DEL-14-05's common comparison-result interface that this export maps. Until one is accepted, no conformance of a comparison export to that interface is claimed.

The rows of a derivative document are classified by the product's ordinary result reader and its base semantic table: each row by its signature (category, target dimension, disposition), with the reader's verdict giving its kind, unit, component and basis and disclosing source rows it cannot map. No deliverable currently commits to criteria for that reader or table; DEL-05-02 holds their code.

- **REQ-001** — The result export is a schema-first JSON result envelope. It has one current version, 0.3.0; a reader admits that version only and refuses any other by name (also cited as DEL-08-04-R1). Source: SOW-046; decomposition AB-00-03.
- **REQ-002** — This deliverable produces no export format other than those of CLM-001 (also cited as DEL-08-04-R2). Source: SOW-046; decomposition OI-004.
- **REQ-003** — Every exported value carries its unit and dimension. Nothing relies on a unit default (also cited as DEL-08-04-R3). Source: PRD §5.6.
- **REQ-004** — The envelope carries or refers to the model and run identity, the solver version, the rule-pack checksum where a rule pack took part, warnings, assumptions and source notes (also cited as DEL-08-04-R4). Source: SOW-046; decomposition AB-00-04.
- **REQ-005** — Diagnostics are carried as structured records with code, class, severity, source, affected object, message, remediation and provenance (also cited as DEL-08-04-R5). Source: decomposition AB-00-06.
- **REQ-006** — Exported statuses keep mechanics solved, user-rule checked, rule inputs incomplete and human review required apart. Where a human acceptance record is referred to, it is distinct from every status the software emits (also cited as DEL-08-04-R6). Source: PRD §5.9, §21.3.
- **REQ-007** — No export claims certification, sealing, professional approval, authentication or automatic code compliance (also cited as DEL-08-04-R7). Source: PRD §21.2.
- **REQ-008** — Exports and public examples embed no protected standards text, copied standards table, proprietary formula, private rule-pack payload or private project data by default. A rule pack appears by identifier, version and checksum only (also cited as DEL-08-04-R8). Source: PRD §19.4, §20.1.
- **REQ-009** — An exporter or adapter goes around no validation, unit check, diagnostic, provenance handling, report control or data-boundary check (also cited as DEL-08-04-R9). Source: decomposition AB-00-07.
- **REQ-010** — Rows keep stable identifiers and a deterministic order, so that equivalent result sets compare (also cited as DEL-08-04-R10). Source: SOW-046; PRD §22.4.
- **REQ-011** — Missing solve-required, rule-required, provenance or unit data is represented as a diagnostic and never filled with a default (also cited as DEL-08-04-R11). Source: PRD §5.5.
- **REQ-012** — Report generation, the results viewer, the headless runner and import and export adapters consume the same document, and none of them receives results by a route that bypasses it (also cited as DEL-08-04-R12). Source: decomposition AB-00-03, AB-00-07.
- **REQ-013** — The export has a defined vocabulary for the nonlinear active-set observations DEL-04-04 supplies: iteration count, flags, support state codes and residuals. It preserves the producer's meaning, unit and status context and states what was omitted; it invents no name and no threshold (also cited as DEL-08-04-R13). Source: SOW-046.
- **REQ-014** — Every results table in the desktop, comparison tables included, exports to CSV (also cited as DEL-08-04-R14). Source: release target exit criterion 5.
  - **One serializer.** Every table goes through one serializer, fed by an adapter that reads the view model the table renders. An adapter recomputes nothing and reads nothing from the page, and export adapters and the save keep REQ-003 to REQ-013.
  - **Preamble.** Each file begins with key and value rows in a fixed order: the notice (the registry's validation-posture statement, word for word); the export context, which is local private; a statement that the file is not a released report; the table kind; the software version; the model hash; the result hash; the analysis route; the case or filter; the unit basis; and one row per rule pack, by identifier, version and checksum, where the table has rule outcomes. A two-sided comparison states the hashes and routes of each side. A hash or route the result does not record is written as not recorded, and one that does not apply to the table as not applicable; an export is not refused for either. Then one empty row, then the table.
  - **Columns.** Every numeric column states its unit; a column whose rows mix units has a unit column beside it; a dimensionless value is labelled as such. Rows keep their existing stable identifiers beside their labels.
  - **Values.** Each numeric value is the unrounded value the table's own conversion function returns for the column's displayed unit, written as the shortest decimal that parses back to that value exactly. The two tables of the main results panel write the session display converter's value; the others take the exact step of the report's display rule; comparison tables write stored values in the rows' own units.
  - **Absent values.** An absent value is never a blank number. Its cell carries the table's own stated status, and missing values and metadata keep their diagnostics.
  - **Rule outcomes.** These use the table's own wording, under REQ-006 and REQ-007.
  - **Encoding.** UTF-8 with a byte-order mark and CRLF line endings. A text cell that a spreadsheet would read as a formula is neutralized.
  - **Determinism.** Identical source data, provenance and table settings give identical bytes. The file carries no timestamp.
  - **Coverage.** A file holds every row the table's filters admit, across all pages, whatever the size of the result.
  - **Saving.** Saving requires the user's explicit intent on the governed local-private CSV route, which is admitted for local-private output only, with the same identity and policy in each of the product's route catalogues. The CSV is not a released report and is not a shareable output.
- **REQ-015** — General result exports of comparisons map DEL-14-05's common comparison-result interface. The mapping and its round trips are bound to the accepted version of that interface before any conformance is claimed, and they preserve source identity, diagnostics, units, raw rows, unmatched records and hashes. Comparison arithmetic and tolerance policy are not this deliverable's (also cited as DEL-08-04-SCA011-REQ-001, DEL-08-04-SCA011-VER-001 and DEL-08-04:1). Source: SOW-046; decomposition SCA-011.
- **REQ-016** — Each member an export document copies from a received result is identified by the SHA-256 of its canonical JSON value (also cited as DEL-08-04-SCA011-HASH-001).
- **REQ-017** — Every unit conversion made at an export boundary is witnessed: for each target format and each conversion it performs there is a check of the converted value (also cited as DEL-08-04:2). Source: PRD §22.4.

- **AC-001** — An export document built from an invented solved model on each route validates against the schema. Every value has its unit and dimension, the diagnostics equal the received result's field for field, the statuses of REQ-006 appear as distinct values, rows carry stable identifiers in a deterministic order, the reproducibility references of REQ-004 are present, and exporting the same result twice gives byte-identical documents. Verify with VER-001.
- **AC-002** — Every results table in the desktop, comparison tables included, exports to CSV as REQ-014 requires. A table without CSV export is detected as a failure. Parsing each exported numeric field returns exactly the unrounded value the table's conversion function produces for the displayed column unit. Repeated exports with identical source data, provenance and table settings are byte-identical. The preamble and table layout, units and identifiers, absent-value handling, byte-order mark, line endings and formula neutralization are as REQ-014 states. Adapters preserve AC-001 and the no-bypass boundary of REQ-009 and REQ-012. The CSV route is admitted only for local-private output with explicit user intent, with matching policy in each route catalogue, and is refused without that intent and for every other audience. Verify with VER-002.
- **AC-003** — A document whose schema version is not 0.3.0 is refused by every reader with a diagnostic naming the version, and nothing is read from it. Verify with VER-003.
- **AC-004** — For each member an export document copies, the recorded hash equals SHA-256 of the member's RFC 8785 canonical form as computed by an implementation that shares no code with the product, and a copy of the member re-serialized with different number spelling or key order reads as the same member. Verify with VER-004.
- **AC-005** — An export of a run with a missing rule input, a missing source note and a value without a unit carries a diagnostic for each and no substituted value. A run with a rule pack carries the pack's identifier, version and checksum and none of its content. No export field or example states a claim REQ-007 names. Verify with VER-005.
- **AC-006** — Each consumer REQ-012 names takes the result export document from the one assembly, and none receives result values by another route. For one received result, the members copied from it and the result content of the document each consumer receives equal those of the document Export Result JSON saves, by canonical value; a member that differs by run is named, and those members are the only exclusion. Verify with VER-006.
- **AC-007** — For a nonlinear result, the export states the iteration count, the flags, each support's state code and the residuals as DEL-04-04's record gives them, with their units and status context, and names any observation it omits. Verify with VER-007.
- **AC-008** — A comparison export maps every field of the accepted comparison-result interface, and a round trip returns the source identity, diagnostics, units, raw rows, unmatched records and hashes unchanged. Verify with VER-008.
- **AC-009** — For each export format and each unit conversion it performs, a converted value agrees with the value formed from the units' public definitions at the witness tolerance of its kind, and a conversion the product states as exact (a length, the pascal-to-megapascal shift) equals it exactly. Verify with VER-009.
- **AC-010** — Export Result JSON is saved only with the user's explicit intent on its governed local-private route and is refused without it. Verify with VER-002.
- **AC-011** — A document that fails the schema, one lacking a unit, the diagnostics or a status among them, or whose recorded hash differs from the hash of the content it covers, is refused by the form check with a named diagnostic, and nothing is read from it. Verify with VER-010.

## Production and Verification Method — Praxeology

Choices that define the product:

- The schema comes first. A document is valid because it satisfies the published schema, and every reader checks the form before it reads a value.
- One version. There is no migration and no compatibility window; an older document is refused by name.
- Copy, do not restate. A received result's members are carried whole and identified by value hash, so a consumer can tell that what it holds is what the solve published.
- One serializer for CSV, beside one adapter per table that reads the table's own view model. A CSV number and a screen number therefore come from the same function.
- Labels are untrusted text. They can come from an imported file, so the serializer neutralizes anything a spreadsheet would execute.

- **VER-001** — Executable check (validates export documents of invented models on each route against the schema with a JSON Schema validator that is not the product's; observes units, the diagnostics against the received result, statuses, identifiers and order; compares two exports byte for byte).
- **VER-002** — Executable check (observes, for every table, the serializer's output against REQ-014; a guard that lists every table under the results and comparison features and fails on one without CSV export; per-table round trips of each numeric field; route-catalogue parity; and the native save's admission with explicit local-private intent and its refusal without it or for another audience). The round trip compares the file with the table's own conversion function, which shows that file and screen agree and nothing more.
- **VER-003** — Executable check (presents documents of other versions to each reader and observes the named refusal).
- **VER-004** — Comparison with a named reference (canonical bytes and SHA-256 from a JCS implementation that imports no product code) with an executable check (observes that respelled and reordered copies of a member read as equal).
- **VER-005** — Executable check (observes the diagnostics of an export seeded with each missing item, the rule-pack reference fields, and the claims-language check over export fields and examples).
- **VER-006** — Executable check (compares by canonical value the copied members and result content of the documents produced for one received result by the desktop export, the report package and the headless runner, and of the document an adapter is handed, with the run-specific members named; and observes that each consumer's result values come from the assembled document).
- **VER-007** — Executable check (observes the nonlinear vocabulary of an export against the active-set record of an invented nonlinear case, and the statement of any omitted observation).
- **VER-008** — Comparison with a named reference (the accepted version of DEL-14-05's comparison-result interface and its schema) with an executable check (round trip of an invented comparison).
- **VER-009** — Comparison with a named reference (the public definitions of the units involved, for example the exact definition of the inch and of the pound-force) for each conversion at each export boundary, compared at DEL-02-02's witness tolerances and exactly where the product states the conversion as exact.
- **VER-010** — Executable check (presents constructed invalid documents to the form check: each schema defect in turn, and a document with an altered member under an unchanged recorded hash; observes the named refusal and that no value is read). The expected verdicts come from a JSON Schema validator that is not the product's and from the outside hash implementation of VER-004.

## Governing Values and Decisions — Axiology

- **AX-001** — *What left is what was solved.* Better is fewer differences between a result as the solve published it and the result a consumer reads from an export. Compactness, convenience of a target format and display rounding are never allowed to alter, drop or relabel a value, unit, diagnostic or status on the way out.
- **AX-002** — *Units are data.* Better is fewer exported numbers whose unit a reader must infer; the target is none. A number is never written without its unit to make a file simpler, and a missing unit is a diagnostic, not a guess.
- **AX-003** — *Comparable tomorrow.* Better is a larger share of two exports of equivalent results that compare field for field by stable identifier. Human-readable labels may be added beside identifiers; they never replace them.
- **AX-004** — *File and screen agree.* Better is less distance between a CSV number and the number the table shows, down to display rounding only. That agreement is never gained by a second conversion path for the file.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-046 OBJ-007 OBJ-009 | REQ-001, REQ-003, REQ-004, REQ-005, REQ-006, REQ-010 | AC-001 | VER-001 | What the envelope must carry comes from SOW-046 and the decomposition's architecture basis (AB-00-03, AB-00-04, AB-00-06). Conformance is decided by a JSON Schema validator the product did not write. The values themselves are the solve's; their correctness is the producing deliverables' criterion, and this criterion claims only that they are carried unchanged. |
| OUT-002 | SOW-046 OBJ-007 OBJ-009 | REQ-014, CLM-003, CLM-004 | AC-002 | VER-002 | Release target exit criterion 5 and REQ-014 fix the form. The shortest-decimal rule is a closed form of binary64 arithmetic, and the pascal-to-megapascal step is a decimal shift. Agreement of a field with the table's own conversion function rests on the product's own output and guards only that file and screen agree; the correctness of unit factors is DEL-02-02's criterion. |
| OUT-001 | SOW-046 OBJ-009 | REQ-001 | AC-003 | VER-003 | REQ-001 states the expected outcome: one version, and a document of any other version refused by name with no migration; each other version is presented as a constructed document. |
| OUT-001 | SOW-046 OBJ-007 | REQ-016 | AC-004 | VER-004 | REQ-016 requires members to be hashed by value; the expected digests come from RFC 8785 and a JCS and SHA-256 implementation that imports no product code. |
| OUT-001 | SOW-046 OBJ-007 | REQ-007, REQ-008, REQ-011 | AC-005 | VER-005 | The no-default rule comes from PRD §5.5 and the prohibited content and claims from PRD §19.4, §20.1 and §21.2; each missing item is seeded in an invented run. |
| OUT-001 | SOW-046 OBJ-009 | REQ-009, REQ-012, CLM-005 | AC-006 | VER-006 | The no-bypass rule comes from the decomposition's architecture basis AB-00-07. The comparison is between documents produced on different routes for one received result, by canonical value; it rests on the product's own output and guards only that the routes agree. |
| OUT-001 | SOW-046 OBJ-007 | REQ-013 | AC-007 | VER-007 | The observations to carry are those of DEL-04-04's active-set record, which this deliverable did not write; the export is compared with that record for an invented case. |
| OUT-001 | SOW-046 OBJ-009 | REQ-015 | AC-008 | VER-008 | The reference is the accepted version of DEL-14-05's comparison-result interface, which this deliverable does not own. The version to map is not decided. |
| OUT-001 | SOW-046 OBJ-007 | REQ-003, REQ-017 | AC-009 | VER-009 | Outside source: the public definitions of the units, which fix every conversion factor exactly. The tolerance at which a binary64 conversion is judged is DEL-02-02's witness tolerance of its kind. |
| OUT-001 | SOW-046 OBJ-009 | REQ-009, CLM-004 | AC-010 | VER-002 | The admission rule comes from DEL-12-02's export controls and PRD §20.3; refusal without intent is observed at the native save. |
| OUT-001 | SOW-046 OBJ-009 | REQ-001, REQ-016, CLM-005 | AC-011 | VER-010 | The published schema and the constructed invalid documents fix the expected refusals. Schema verdicts are taken from a JSON Schema validator the product did not write, and hashes from a JCS and SHA-256 implementation that imports no product code. |
