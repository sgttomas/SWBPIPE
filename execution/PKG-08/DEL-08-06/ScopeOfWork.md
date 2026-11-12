---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-06
package_id: PKG-08
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-024]
package_objective_refs: [OBJ-007, OBJ-016, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-08-06 State, comparison, and handoff report sections

## Purpose and Objective Traceability

A report can say which saved model state and which analysis run it concerns, how two states or two runs differ, and what a handoff package to a downstream tool contains and cannot represent. Each is shown as a record a reviewer can trace back to its source by identity and hash, and none of them reads as validation, acceptance or approval of the design.

This deliverable contributes those three families of report section, assembled as one section set from records other deliverables own. It adds no comparison arithmetic and no handoff content of its own.

It serves SOW-024 under OBJ-007, OBJ-016, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-08, row DEL-08-06), and PRD FR-REP-001 (a report includes states and runs), FR-REP-004 (a user can include comparison summaries in a report), §19.1 (the model state comparison report, the analysis run comparison report and the handoff package manifest), §19.2 (model state identity and hash, run identity, comparison results where selected), FR-HAND-002 and FR-HAND-004 (what a handoff manifest states and flags) and §21.3 (a human acceptance record is separate from report generation). It supports the release target's exit criterion 4, which DEL-08-01 owns.

- **OUT-001** — The state, comparison and handoff section set: state and run sections, comparison sections and handoff sections, with their diagnostics, limitations and coverage statement, carried in the report package and available to the report (REQ-001 to REQ-016).

## Deliverable Definition — Ontology

A **model state** is an immutable saved state of the model with its name, tags, notes, hash, warnings and unresolved assumptions. An **analysis run** is the record of one solve of one state: solver version and settings, units, load cases, diagnostics, results, rule-pack and library references, and hashes. Both are PRD §15 records that PKG-14 owns.

A **comparison** is the deterministic difference of two states or two runs: entities matched by stable identifier or by a manual mapping, each classified, result differences in a common unit, judged against a tolerance profile. It requires no prover lifecycle.

A **handoff package manifest** states what a package for a downstream tool carries: model hash, units, entity identifiers, library and rule references, warnings, unresolved assumptions, the mapping to target fields, and what the target cannot represent. **External-prover metadata** is the user's names, tags, notes and references about work in another tool; it has no status vocabulary.

The **section set** is one record with an identifier naming the run it belongs to, where it belongs to one. Its **state and run sections** restate a state or a run by reference; its **comparison sections** restate a comparison; its **handoff sections** restate a manifest, its export workflow record and any external-prover metadata. The set also carries its own diagnostics, the limitations of its sources, and a **coverage statement**: for each SOW-024 content category (inputs, sources, warnings, assumptions, results, rule-pack checksums, limitations), whether the set holds it or it is unavailable.

A section **restates by reference**: it carries identifiers, hashes, source notes, privacy class and review state, and the non-private fields of the source record. It never carries a private payload.

Boundary:

- **CLM-001** — State and run records, the comparison engines and their mapping and tolerance contracts are PKG-14's; the handoff manifest, the export workflow and external-prover metadata are PKG-15's. This deliverable reads their records and computes none of their content.
- **CLM-002** — The report's assembly, rendering, layout, lint gate and package are DEL-08-01's. The report package checks the section set against its other members; what the set must hold is decided here. What a user-released package carries of the section set is DEL-08-01's rule. The warnings and provenance vocabulary is DEL-08-03's, hash records are DEL-08-02's, and export classification is DEL-12-02's.
- **CLM-003** — Outside the product: GUI presentation of comparisons, the headless runner's behaviour, solver execution, and any record of human acceptance.

## Completion and Reliance Basis — Epistemology

Not decided: how the external and non-JSON payloads of a state or a run are partitioned for hashing. Until it is, a section refers to such a payload by its asset hash where one exists and states it as unavailable where none does.

- **REQ-001** — The product provides report sections for model states, analysis runs, comparisons and handoff manifests (also cited as DEL-08-06-R1). Source: SOW-024; PRD §19.1.
- **REQ-002** — The sections cover SOW-024's content: inputs, sources, warnings, assumptions, results, rule-pack checksums and limitations. A category the sources do not supply is stated as unavailable (also cited as DEL-08-06-R2). Source: SOW-024.
- **REQ-003** — No section declares code compliance, certification, sealing, approval, authentication, endorsement or professional reliance (also cited as DEL-08-06-R3). Source: PRD §21.2.
- **REQ-004** — The analysis statuses a section shows are the software's own. A status of human approval for a project, or any equivalent, is never among them (also cited as DEL-08-06-R4). Source: PRD §21.3.
- **REQ-005** — A missing solve-required or rule-required value remains an explicit finding with its diagnostic and provenance (also cited as DEL-08-06-R5). Source: PRD §5.5.
- **REQ-006** — A numeric value in a section carries its unit and dimension from the source record, or a diagnostic saying that it has none (also cited as DEL-08-06-R6). Source: PRD §5.6.
- **REQ-007** — Sections preserve the stable references, checksums, source notes, privacy classification, review state and provenance of the records they restate (also cited as DEL-08-06-R7). Source: SOW-024; OBJ-016.
- **REQ-008** — Public sections, templates, fixtures and examples copy no protected standards text, tables or examples, proprietary formulas or engineering values, private project data, private rule-pack payloads, private library content or real secrets (also cited as DEL-08-06-R8). Source: PRD §19.4.
- **REQ-009** — A private rule pack, library, owner requirement or project value is referred to only by identifier, checksum, source note, privacy class and review state. A comparison section states the stored values of the fields and results that differ (REQ-011). They are the user's project data; a section set holding them is produced only under the local-private context, and no rule-pack or library payload is stated under any context (also cited as DEL-08-06-R9). Source: PRD §5.4; OBJ-018.
- **REQ-010** — A state or run section gives the record's stable identity, hashes, warnings, assumptions, diagnostics, solver version, settings, units, load cases, results, rule and library references and result hashes, as far as the record holds them (also cited as DEL-08-06-R10). Source: PRD §15.1, §15.2.
- **REQ-011** — A comparison section states the comparison as computed and bounds what it restates of the results. It states both sides by identifier and hash; the comparison's settings, its manual mappings and its tolerance profile reference; every difference in inputs: each added and each removed entity by stable identifier, and each changed entity with every changed field's path, both stored values and their units; the diagnostics and settings that differ; and the hash of the complete comparison record. Of the result differences it states the counts, for the comparison and for each result family: pairs compared, pairs that differ, pairs not compared by reason, and rows unmatched by side and reason. For each result family and unit it states the ten largest differences by absolute value, at most one for any one object, each with both source values, the difference, their common unit and where it occurs: the object, the load case or combination, the component and the location. They are ordered by absolute difference, largest first, and then by result identifier. Differences are ranked only among rows of one family in one unit. A pair not compared and a row present on one side only are never dropped for want of a number: each is counted per family and reason, and the first ten of each in result identifier order are listed with their reasons. A row the reader reads on one side and not on the other is listed as unmatched with the identity fields that differ. The section states each side's standing as the comparison holds it, for the whole run, and states what the summary covers and what it omits: the families summarised, the number of rows not listed, and where they are. It states that the comparison table's CSV holds every row, with its row count, and the CSV states the hash of the same comparison record (also cited as DEL-08-06-R11). Source: PRD §15.3 to §15.5, FR-REP-004, FR-CMP-010.
- **REQ-012** — A handoff section preserves the manifest's model hash, units manifest, entity identifiers, library and rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags and external-prover metadata (also cited as DEL-08-06-R12). Source: PRD FR-HAND-002 to FR-HAND-004, §16.3.
- **REQ-013** — Sections are assembled from records obtained through the product's service boundaries and go around no validation, diagnostic, privacy, protected-content or report control (also cited as DEL-08-06-R13). Source: decomposition AB-00-03, AB-00-07.
- **REQ-014** — A section set passes the report's protected-content and provenance gates before it appears in a report (also cited as DEL-08-06-R14). Source: SOW-043.
- **REQ-015** — The section set is produced wherever its source records exist: for the desktop's current state, run and comparison; for a solver and rule-check run; and for a handoff package and its export workflow. It reaches the report package and, where the user selects them, the rendered report (also cited as DEL-08-06:1). Source: PRD FR-REP-001, FR-REP-004.
- **REQ-016** — Sections can be assembled from the model-state and analysis-run records a persisted project holds. Doing so changes nothing in the project and runs no solve. Source: OBJ-016; PRD §15.1.

- **AC-001** — For invented state, run, comparison and handoff records, the section set holds one section per record, and each field REQ-010, REQ-011 and REQ-012 names equals the source record's. The coverage statement marks each SOW-024 category as held or unavailable in agreement with what the records supply. For a comparison, the counts and the largest differences equal those formed by hand from the invented records, in the stated order, and a record with ten times the result rows gives a section no larger. Verify with VER-001.
- **AC-002** — The same source records give byte-identical section sets on repeated assembly, in an order that does not depend on the order the records were supplied in. Every implementation of the assembler in the product gives byte-identical section sets for the same records. A result difference with no unit metadata appears with a diagnostic and not as a bare number. Verify with VER-002.
- **AC-003** — An unsupported-target flag, an unresolved assumption and a warning in a manifest each appear in the handoff section, and removing any of them from the source removes exactly that entry. External-prover metadata appears as the user's names, tags and notes, and a source record that attempts to give it an execution or validation status produces a finding. Verify with VER-003.
- **AC-004** — A source record that lacks a field REQ-010, REQ-011 or REQ-012 names for its kind yields a finding naming the record and the field, and no value is supplied for it. Verify with VER-003.
- **AC-005** — A source record carrying an approval, certification or compliance status, a prohibited authority term, or a boundary flag set to claim one produces a diagnostic, and the status is not carried into the set. The set's own statuses are all in the software's analysis-status vocabulary, and its boundary flags all read as no claim. Verify with VER-004.
- **AC-006** — A source record holding a private payload gives a section with the payload absent and its identifier, checksum, source note, privacy class and review state present. A comparison section's stored values appear only in a section set produced under the local-private context, and a request whose section set holds them under any other context is refused by name. A changed field whose value came from a library record applied to the model is stated as the model value it now is, with its origin reference beside it and nothing else of the record. Verify with VER-004.
- **AC-007** — Every public fixture and example of the section set is invented, carries its provenance note and contains none of the content REQ-008 names (also cited as DEL-08-06:2). Verify with VER-005.
- **AC-008** — Assembling from a persisted project returns a section for each state and run the project holds, with their warnings, assumptions, limitations, solver context and provenance; the project's bytes are unchanged and no solve runs. Verify with VER-006.
- **AC-009** — A report package saved from the desktop for a solved model holds the section set for the current state and run and each comparison the user has included; the set's identifier names the run, and its state and run sections carry the current state's and run's identities and hashes; a solver and rule-check run and a handoff package each yield their sections; and a user can include state, run, comparison and handoff sections in a rendered report, where they pass the report's gates. Verify with VER-007.

## Production and Verification Method — Praxeology

Choices that define the product:

- Sections are built from structured records. Nothing is scraped from rendered text, and nothing is recomputed that a source record already states.
- Ordering depends only on the records, never on the order in which they were supplied, so assembly is deterministic.
- An authority claim in a source record is reported and dropped, not passed through and not silently removed.
- Where the section set is assembled in more than one language, each implementation gives the same set for the same records.
- The bound on restated result differences was chosen by Agent 0 because a report is read for the consequence of a change, which the largest differences in each family show, and because a section that restates every row grows with the result until the package can no longer be saved.

- **VER-001** — Executable check (observes the fields of each section against the invented source records it was built from, and the coverage statement against the categories those records supply): the source records are written for the check, and the expected result is equality with them.
- **VER-002** — Executable check (assembles the same records twice and in permuted order and compares bytes; assembles the same non-empty invented records with each implementation of the assembler and compares bytes; observes the diagnostic for a difference without unit metadata).
- **VER-003** — Executable check (observes the handoff section for a manifest with each disclosure present and removed, the finding for external-prover metadata given a status, and the finding for each field of REQ-010, REQ-011 and REQ-012 removed from a record of its kind).
- **VER-004** — Executable check (observes the diagnostics and the resulting set for source records seeded with each authority status, term and flag, and for a record holding a private payload).
- **VER-005** — Judgment by a reviewer who did not write the fixtures (reads each public fixture and example of the section set for the content REQ-008 names), with the protected-content linter run over them as a heuristic aid.
- **VER-006** — Executable check (assembles from an invented persisted project, compares the project's bytes before and after, and observes that no solver entry is called).
- **VER-007** — Executable check (observes the section set inside a package saved from the desktop, with its identifier and its state and run identities and hashes compared with the package's other members, the sections from an invented solver and rule-check run and from an invented handoff package, and the sections rendered in a report with the report's gates applied).

## Governing Values and Decisions — Axiology

- **AX-001** — *Traceable, not persuasive.* Better is a reader able to get from more of a section's statements to the source record by identity and hash. A fuller or smoother summary is never bought by dropping a reference, a warning, an unmatched entity or an unsupported-target flag.
- **AX-002** — *A comparison shows change, nothing more.* Better is a clearer account of what differs and what could not be matched. No wording, ordering or omission may make a comparison or a handoff read as external validation or acceptance.
- **AX-003** — *Refer, do not copy.* Better is more reproducibility evidence in a section. It never includes a private payload; where the two conflict the section keeps the identifier and the hash and says the content is private.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-024 OBJ-007 OBJ-016 | REQ-001, REQ-002, REQ-007, REQ-010, REQ-011, REQ-012 | AC-001 | VER-001 | What each section must hold comes from PRD §15.1 to §15.3, FR-HAND-002 and SOW-024. The comparison is with invented source records the check writes; the section computes no value of its own, so no numerical ground is involved. |
| OUT-001 | SOW-024 OBJ-007 OBJ-016 | REQ-006, REQ-011, REQ-015 | AC-002 | VER-002 | The criterion determines the expected result: repeated and permuted assembly, and each implementation, must give the same bytes; reproducible reports are OBJ-007. Agreement between implementations rests on the product's own output and guards only against their drifting apart. The unit rule comes from PRD §5.6. |
| OUT-001 | SOW-024 OBJ-017 | REQ-005, REQ-012, CLM-001 | AC-003 | VER-003 | The disclosures a handoff must keep come from PRD FR-HAND-002 and FR-HAND-004, and the absence of a prover status from §16.3; each case is seeded in an invented manifest. |
| OUT-001 | SOW-024 OBJ-007 | REQ-002, REQ-005, REQ-010, REQ-011, REQ-012 | AC-004 | VER-003 | The rule that missing data is shown and never defaulted comes from PRD §5.5, and the fields are those REQ-010, REQ-011 and REQ-012 name from PRD §15 and FR-HAND-002; each missing field is constructed by removal from a complete invented record. |
| OUT-001 | SOW-024 OBJ-018 | REQ-003, REQ-004 | AC-005 | VER-004 | The prohibited claims come from PRD §21.2 and the separation of human acceptance from §21.3; the offending statuses, terms and flags are seeded by the check. |
| OUT-001 | SOW-024 OBJ-018 | REQ-008, REQ-009 | AC-006 | VER-004 | The private-data rule comes from PRD §5.4 and §19.4; the private payload is an invented marker the check plants and then looks for. |
| OUT-001 | SOW-024 OBJ-018 | REQ-008, REQ-014 | AC-007 | VER-005 | The prohibited content comes from PRD §19.4. The ground is the reading by someone who did not write the fixtures; the linter is heuristic. |
| OUT-001 | SOW-024 OBJ-016 | REQ-013, REQ-016 | AC-008 | VER-006 | The immutability of states and runs comes from PRD §15.1; the unchanged project is observed by comparing its bytes. |
| OUT-001 | SOW-024 OBJ-007 OBJ-017 | REQ-001, REQ-013, REQ-014, REQ-015, CLM-002 | AC-009 | VER-007 | Where the sections must reach comes from PRD FR-REP-001 and FR-REP-004 and §19.1; their presence is observed in a saved package and a rendered report built from invented records. |
