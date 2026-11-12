---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-03
package_id: PKG-08
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-024]
package_objective_refs: [OBJ-007, OBJ-011]
---

# Scope of Work — DEL-08-03 Warnings, assumptions, and provenance report section

## Purpose and Objective Traceability

A reviewer reading a calculation report finds, in one section, everything that qualifies the results: what was missing, what the software warned about, what was assumed, which values the user supplied, and where each of those values came from. Nothing in that section is the report's own judgment. It shows what the solver, the rule check, the model and the library store said, and it shows a gap as a gap.

This deliverable contributes the section's records and the rules for what the section discloses and when a record may appear at all. The calculation report (DEL-08-01) renders the section and applies its rules as report gates.

It serves SOW-024 under OBJ-007 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-08, row DEL-08-03), and PRD FR-REP-003 (a report identifies sources or source notes for user-supplied data), §19.1 (the warning and assumption report), §19.2 (source and provenance summary, warnings, unresolved assumptions), §20.2 (the provenance record) and §5.5 (no silent engineering defaults). It supports the release target's exit criterion 4, which DEL-08-01 owns.

- **OUT-001** — The warnings, assumptions and provenance section: its record definitions and schema, its validation, and the disclosure and disposition rules a report applies to it (REQ-001 to REQ-015).

## Deliverable Definition — Ontology

The section has six groups, each fed by records it does not create:

- **Missing data:** a solve-required or rule-required input that is absent.
- **Warnings:** the diagnostics of the solve, the rule check, the model and the library store.
- **Assumptions:** what the user or the model assumed and a reviewer must look at.
- **User-supplied values:** values that came from the user, a private library or a rule pack, marked as such.
- **Provenance notes:** the source record of each material, section, component and support that carries one.
- **Library records:** the libraries the model refers to.

A **diagnostic** is the record DEL-00-06 defines: code, class, severity, source, affected object, message, remediation and provenance. The product's six warning classes are solve blocking, rule-check blocking, provenance, assumption, nonlinear and IP boundary; a boundary may also declare local classes.

A **provenance record** has the seven fields of PRD §20.2 (source name, source location, source licence, contributor, contributor certification, redistribution status, review status) and a privacy classification. A field the source does not give reads "not recorded"; an absent redistribution status reads "unknown" and an absent review status "pending". Review status is the user's own review of the source data; the software never sets or advances it.

A **library record** entry is safe metadata only: the library's kind, identity and version, the record, and the model entities that refer to it. No library value or record content enters the section.

The **report context** is the export context the report is rendered under (DEL-12-02's vocabulary). The rules below distinguish a local-private report from every other context.

Boundary:

- **CLM-001** — The report's assembly, rendering, gating and notice are DEL-08-01's. In the renderer and in the desktop's report input builder, this deliverable's part is the content of this section: which records are built for it and what each says. The audit manifest is DEL-08-02's, the protected-content lint DEL-08-05's, and the classification of export contexts DEL-12-02's.
- **CLM-002** — The section creates no engineering meaning. It does not classify, grade or resolve a finding, infer a missing message, source or severity, or state a disposition a person has not recorded.

## Completion and Reliance Basis — Epistemology

- **REQ-001** — Missing solve-required and rule-required data appear as explicit findings (also cited as DEL-08-03-REQ-001). Source: PRD §5.5, §17.5.
- **REQ-002** — The section renders the warning class each diagnostic arrives with, each as its own class: the six warning classes (solve blocking, rule-check blocking, provenance, assumption, nonlinear and IP boundary) and any local class its schema declares (also cited as DEL-08-03-REQ-002). Source: decomposition AB-00-06; DEL-00-06/REQ-002.
- **REQ-003** — Each warning and assumption keeps the trace data supplied with it: code, class, severity, source, affected object, message, remediation and provenance (also cited as DEL-08-03-REQ-003). Source: decomposition AB-00-06.
- **REQ-004** — User-supplied values and rule-pack references are identified as user-supplied or private, with their source notes, and with the rule pack's identity and checksum where the rule check supplies them (also cited as DEL-08-03-REQ-004). Source: PRD §5.4, §19.2.
- **REQ-005** — The section is rendered only inside a report that carries the PRD §19.3 notice, and any boundary wording in the section is a registered statement of `docs/claims_registry.md`, word for word (also cited as DEL-08-03-REQ-005). Source: PRD §19.3; DEL-01-04/REQ-011.
- **REQ-006** — No public template or example of the section reproduces protected code text, standards tables, copyrighted examples, copied formulas, material allowables, SIF or flexibility tables, protected dimensional tables or proprietary commercial data (also cited as DEL-08-03-REQ-006). Source: PRD §19.4.
- **REQ-007** — A unit-bearing value in the section is shown with its unit (also cited as DEL-08-03-REQ-007). Source: PRD §5.6.
- **REQ-008** — The section keeps mechanics solved, user-rule checked, incomplete data and human review required apart, and shows none of them as code compliance (also cited as DEL-08-03-REQ-008). Source: PRD FR-REP-002, §21.2.
- **REQ-009** — The section's records refer to the run they qualify by the model hash, the software and solver versions and the rule-pack checksum that the audit manifest supplies (also cited as DEL-08-03-REQ-009). Source: SOW-024; PRD §19.2.
- **REQ-010** — A missing source or provenance is itself reported, as a provenance warning, and is never left out (also cited as DEL-08-03-REQ-010). Source: PRD §5.5, §20.2.
- **REQ-011** — If automated protected-content lint is unavailable, a change to the section remains subject to explicit human review, and a lint that did not run is never counted as passed (also cited as DEL-08-03-REQ-011).
- **REQ-012** — No text the section renders claims certification, approval, endorsement, sealing, authentication or automatic code compliance (also cited as DEL-08-03-REQ-012). Source: PRD §21.2.
- **REQ-013** — On every solve route, the section carries a provenance record for each material, section, component and support whose model record gives a source, and a library record entry for each library the model refers to. The library store's findings on those references (a reference that does not resolve, a library whose version has changed) appear as provenance warnings naming the entity. Source: PRD FR-REP-003, §20.2.
- **REQ-014** — A record whose redistribution status is protected-suspected, or whose review status is quarantined or rejected, is refused in every context. A private record is rendered only in a local-private report, except that a record classed as the user's own project data may appear in a user-released report under DEL-08-01's user release (DEL-12-02's `user_released_report` context); every other private class blocks there, and REQ-015 still governs authored text. A record whose redistribution, review or privacy status is unknown or undecided is rendered in a local-private report with a non-blocking warning that names the record and the unresolved field and says the source's status is not recorded; in every other context it blocks the report. Source: `docs/IP_AND_DATA_BOUNDARY.md` §5, §6.
- **REQ-015** — Text the user authored in a provenance record, a path typed as a source location included, appears as written in local-private outputs. No other context carries these records, and a render refused for its provenance records returns no document body.

- **AC-001** — Every diagnostic and every assumption supplied to the section appears in it with its code, class, severity, source, affected object, message, remediation and provenance equal to those supplied. None is reclassified, merged or dropped, and each of the six classes is shown as its own class. Verify with VER-001.
- **AC-002** — A missing solve-required value and a missing rule-required value each appear as a finding of their own class with no value in place of the missing one, and a value with no source appears with a provenance warning. No rule outcome is shown for a check whose inputs are missing. Verify with VER-002.
- **AC-003** — For an invented model with materials, sections, components, supports and library references, on the pressure-free route and on the exact route, the section holds one provenance record per sourced entity with the seven fields and the privacy classification, "not recorded" for each absent field, and one library record entry per distinct library reference with no library value in it. An unresolved reference and a changed library version each appear as a provenance warning naming the entity. Each user-supplied value and rule-pack reference is marked user-supplied or private with its source note, with the rule pack's identity and checksum where the rule check supplies them, and no rule-pack content. Verify with VER-003.
- **AC-004** — The same input rendered under each report context gives the dispositions of REQ-014: the three refused statuses refuse in every context; an unknown or undecided status gives a named, non-blocking warning in a local-private report and a blocked report elsewhere, a user-released report included; a resolved private record renders in a local-private report and blocks elsewhere, except that under a user release a record classed as the user's own project data renders and one of any other private class blocks; and a blocked render returns no document body. Authored record text in a local-private report equals the text entered. Verify with VER-004.
- **AC-005** — A report holding each analysis status shows the four states of REQ-008 as distinct states, shows every unit-bearing value of the section with its unit, refers to its run by the manifest's identities, and contains no claim REQ-012 names; and every boundary statement in the section equals a registered statement of `docs/claims_registry.md`. Verify with VER-005.
- **AC-006** — Every record the product places in the section validates against the section's schema; the schema requires the trace fields of REQ-003; and the schema admits each of the six warning classes. Verify with VER-006.
- **AC-007** — The public template and examples of the section contain none of the content REQ-006 names. Verify with VER-007.

## Production and Verification Method — Praxeology

Choices that define the product:

- The section is a consumer. Its records are built from what the producing layer said, and a field the producer left out stays out, shown as not recorded.
- The disposition of a provenance record is decided from the record's own statuses and the report's context, at the point where the section is validated. The webview's screening is not the authority; the validating layer is.
- Review status is copied from the record. Writing "pending" for every record would hide a rejected or quarantined source, so it is not done.
- Library references are resolved by the library store's own check. The section adds no second resolution rule.

- **VER-001** — Executable check (observes the section's diagnostics and assumptions against a supplied set covering each class and each trace field): the supplied set is an invented input the check writes, and the expected result is equality with it.
- **VER-002** — Executable check (observes the findings for an invented run with one missing solve input, one missing rule input and one value with no source, and the absence of any substituted value or rule outcome).
- **VER-003** — Executable check (observes the provenance records and library entries built from an invented model on both routes, the warnings for an unresolved and a version-changed reference, and the marking, source note and rule-pack identity and checksum of each user-supplied value and rule-pack reference, for a private pack seeded with marker content): expected records are written by hand from the model's own entries and PRD §20.2's field list.
- **VER-004** — Executable check (observes, for each report context, a user release included, and each status and privacy class, whether the record is refused, warned or rendered, whether the report is blocked, and whether a blocked render returns a body; compares authored text with the text entered).
- **VER-005** — Executable check (observes the status display, unit display and manifest references in a rendered section, and compares each boundary statement in it with the registry's text) and judgment by a reviewer who did not write the section (reads its rendered text against `docs/claims_registry.md` and PRD §21.2).
- **VER-006** — Executable check (validates the section records of every committed report input against the section's schema with a JSON Schema validator that is not the product's, and observes that the schema's class list includes the six warning classes).
- **VER-007** — Executable check (runs the protected-content linter over the section's public template and examples) and judgment by a reviewer who did not write them (reads them for the content REQ-006 names). The linter is heuristic; the review is what decides.

## Governing Values and Decisions — Axiology

- **AX-001** — *Findings over defaults.* Better is fewer qualifying facts that a reader of the results could miss: a missing input, an unsourced value, an unresolved library reference. A cleaner or shorter section is never bought by omitting one, and a value is never supplied to close a gap.
- **AX-002** — *The producer's words.* Better is less difference between what a producing layer reported and what the section shows. Readability may group and order; it may never reclassify, soften or summarise away a diagnostic's code, class, severity or affected object.
- **AX-003** — *Private stays local.* Better is more of the user's own source record visible to the user in their own local report. That may never carry a private or unresolved record into a shared or public context beyond what the user has released for that one report; there the report blocks.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-024 OBJ-007 | REQ-002, REQ-003, CLM-002 | AC-001 | VER-001 | The required trace fields and classes come from the decomposition's architecture basis AB-00-06. The comparison is with the invented set of diagnostics and assumptions the check itself supplies, which the product did not write. |
| OUT-001 | SOW-024 OBJ-007 OBJ-011 | REQ-001, REQ-010 | AC-002 | VER-002 | What must be shown as missing comes from PRD §5.5 and §17.5; the missing inputs are constructed in an invented run. |
| OUT-001 | SOW-024 OBJ-007 | REQ-004, REQ-013 | AC-003 | VER-003 | The record's fields come from PRD §20.2 and the duty to show sources from FR-REP-003. Expected records are written by hand from the invented model's entries, not taken from a previous render. |
| OUT-001 | SOW-024 OBJ-011 | REQ-014, REQ-015 | AC-004 | VER-004 | The dispositions are REQ-014's and REQ-015's, with the refused statuses following the quarantine rule of `docs/IP_AND_DATA_BOUNDARY.md` §5; each status, privacy class and context is constructed in an invented input, and the expected outcome is the one those requirements state. |
| OUT-001 | SOW-024 OBJ-011 | REQ-005, REQ-007, REQ-008, REQ-009, REQ-012 | AC-005 | VER-005 | The status vocabulary and the prohibited claims come from PRD FR-REP-002 and §21.2, and the registered wording from `docs/claims_registry.md`. |
| OUT-001 | SOW-024 OBJ-007 | REQ-002, REQ-003 | AC-006 | VER-006 | The schema is this deliverable's own definition; conformance is decided by a JSON Schema validator the product did not write, and the class list is checked to include the six classes AB-00-06 names. |
| OUT-001 | SOW-024 OBJ-011 | REQ-006, REQ-011 | AC-007 | VER-007 | The prohibited content comes from PRD §19.4. The linter's result is heuristic and only supports the reviewer's reading, which is the ground. |
