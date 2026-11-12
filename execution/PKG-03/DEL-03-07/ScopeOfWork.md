---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-07
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-019,SOW-044]
package_objective_refs: [OBJ-002, OBJ-004]
---

# Scope of Work — DEL-03-07 Public/private library import provenance checker

## Purpose and Objective Traceability

This deliverable is the gate every library passes on its way into the product. Given a material, section, component or hanger library and whether it is meant to be public or private, it decides one of five outcomes, and says why in findings a user can act on. Its effect is that data enters the public project only with a documented source and documented redistribution rights, that a user's private library stays local, and that content suspected of being protected is held and never stored or republished.

It serves SOW-019 and SOW-044 under OBJ-002 and OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-07), and PRD §18.4 (import warnings), §20.2 (provenance of public data) and §20.3 (private data handling) with FR-RULE-005 and FR-RULE-006. Hanger libraries are a user-imported library class. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable; it reaches exit criterion 1's hanger sizing through DEL-04-11, which sizes from an imported private catalogue.

- **OUT-001** — The library import boundary for material, section and component libraries: the check of an already-parsed library document, its outcome and findings, and the rule for what the local library store keeps.
- **OUT-002** — The same boundary for hanger libraries, in formats 1.0.0 and 1.1.0.

## Deliverable Definition — Ontology

**Terms the boundary implements.**

- *Import.* The presentation of one library document of a stated kind (material, section, component or hanger) with a stated intended visibility, public or private. The document is already parsed; the boundary reads no external file format.
- *Outcome.* Exactly one of: accepted as public; accepted as private and local only; review required; rejected; quarantined. Where findings of several kinds are present the gravest governs, in the order quarantined, rejected, review required, accepted. Only the two accepted outcomes are acceptance.
- *Finding.* One reason for an outcome: a code, a severity, the path of the member it concerns, a message and a remediation, projected into a diagnostic with code, class, severity, source, affected object, message, remediation and provenance.
- *Provenance check.* On the library and on each record: the seven members of PRD §20.2 (source name, source location, source licence, contributor, contributor certification, redistribution status, review status), each present and non-empty. On each numeric value: its unit and dimension, and its own provenance.
- *Public disposition.* A public import is accepted only when its redistribution status is public permissive and its review status is accepted. Private data, an unknown or missing redistribution status and any other status block it; a review status not yet accepted leaves it at review required.
- *Quarantine.* A library, record or value whose redistribution status says protected content is suspected, or whose review status is quarantined, quarantines the whole import, whatever its intended visibility.
- *Unreadable disposition.* A redistribution or review status that is not text, or a provenance that is not an object, anywhere in a library. No outcome can be read from it, so the import is rejected by name.
- *Local library store.* The store on the user's machine that holds one document per project, library kind and library identifier.

**Boundary.** This deliverable decides admission. The formats it checks against are DEL-03-01's (material) and DEL-03-02's (section, component, hanger). It makes no legal determination: whether a licence in fact grants redistribution is for the project's human authority, and the boundary records and gates on what the document states. DEL-04-11 owns what a hanger's working load range means in sizing. DEL-07-03 owns the Library Manager in which a user imports. DEL-12-04 owns how private libraries are kept out of exports, bug reports and telemetry. This deliverable reads no external file format. The decomposition selects no external library format; an adapter that parses one is admitted through DEL-10-02 and presents its parsed document to this boundary (REQ-010).

**Not decided.** PRD §18.4 requires an import to flag values outside user-defined reasonableness ranges. Where a user defines those ranges, and whether this boundary or the adapter framework (DEL-10-02/REQ-004) applies them, is not decided; this scope states no criterion for it. PRD §18.4 also requires a flag on a value that appears to be protected standards data. How the boundary recognizes such a value when its metadata does not say so is not decided; AC-003 covers content whose metadata states the suspicion.

## Completion and Reliance Basis — Epistemology

An accepted import means the document states what the boundary requires. It is a software finding about metadata. It does not establish that the stated rights exist, that the values are correct, or that a record is ready to solve.

### Requirements

- **REQ-001** — A public import is accepted only with recorded source, provenance, licence, contributor certification, review status and redistribution status (SOW-019, SOW-044; `docs/CONTRACT.md` OPS-K-IP-2; PRD §20.2).
- **REQ-002** — An import with missing provenance or missing redistribution status is flagged or rejected. Nothing is defaulted on the way in (SOW-044; `docs/CONTRACT.md` OPS-K-DATA-2; PRD §18.4).
- **REQ-003** — Public and private handling are separate. Private data is never accepted as public, and a private library is not transmitted or published by importing it (`docs/CONTRACT.md` OPS-K-PRIV-1, OPS-K-DATA-1; PRD §20.3).
- **REQ-004** — Content suspected of being protected is quarantined and escalated. It is not paraphrased, transformed or accepted, and a finding never reproduces it (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3; PRD §20.2).
- **REQ-005** — The unit and dimension of every imported numeric value are required and preserved. The boundary converts and assumes no unit (`docs/CONTRACT.md` OPS-K-UNIT-1; PRD §18.4).
- **REQ-006** — Every finding is a diagnostic with code, class, severity, source, affected object, message, remediation and provenance (the decomposition's architecture basis for DEL-00-06).
- **REQ-007** — The boundary makes no legal acceptance. An unresolved licence or rights question remains a finding for human review (`docs/CONTRACT.md` OPS-K-GOV-4).
- **REQ-008** — Imported data is flagged where a required field is missing, a unit is missing or inconsistent, provenance is missing, redistribution status is unclear, or a value appears to be protected standards data (PRD §18.4).
- **REQ-009** — The local library store keeps only an import accepted as private and local. A quarantined, rejected or review-required import is refused: its findings are returned and nothing is written. A stored library document is returned as it was saved: the whole document, with its units, values and provenance (DEL-00-04/REQ-001).
- **REQ-010** — No import, export or adapter path brings a library into the product around this boundary (the decomposition's architecture basis for DEL-00-07).
- **REQ-011** — A hanger library is imported in format 1.0.0 or 1.1.0. A 1.0.0 document imports unchanged. A working load range in a 1.0.0 document, a range whose ends differ in unit, and a range whose minimum is not below its maximum are each refused by name. An import fills in no absent engineering value (PRD §12.4).
- **REQ-012** — Wherever the boundary is implemented more than once, every implementation returns the same outcome and the same findings for the same document.
- **REQ-013** — Every fixture and test case of the boundary is invented, labelled as invented, and carries no protected standards content, vendor data or real rights or provenance record (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1).

### Acceptance criteria

- **AC-001** — A material, section or component library presented as public is accepted as public only when the library and every record carry all seven provenance members, the redistribution status is public permissive and the review status is accepted, and every numeric value carries its unit, dimension and own provenance. With a provenance member missing it is rejected with a finding that names the member; with private data, or a redistribution status that is unknown, missing or anything but public permissive, it is rejected; with a review status not yet accepted and nothing graver, it is review required.
- **AC-002** — The same library presented as private, complete in its provenance and units, is accepted as private and local only. It is never returned as accepted as public, whatever its content.
- **AC-003** — A library in which the library, a record or a value at any depth states that protected content is suspected, or a quarantined review status, is quarantined under either visibility, and a rejected source is rejected. No finding quotes the content it concerns.
- **AC-004** — A numeric value without its unit and dimension, or without its own provenance, rejects the import with a finding at that value's path. A library document saved to the local library store and opened again equals the saved document in canonical JSON form (RFC 8785), every unit, value and provenance as it was saved.
- **AC-005** — Every finding of every outcome is a diagnostic with all eight members of REQ-006, and its affected object identifies the library and, where it concerns one, the record.
- **AC-006** — A library whose licence or rights are unresolved is never returned as accepted as public. No outcome, finding or message states that redistribution is lawful or that a licence has been approved.
- **AC-007** — A library document that lacks a member its format requires, or carries a redistribution or review status that is not text or a provenance that is not an object, is not accepted, and a finding names the member. A value whose unit is not a unit of its stated dimension in the unit catalog is rejected with a finding at its path. A private import whose redistribution status is unknown is accepted as private and local only, with a finding that names the status.
- **AC-008** — Saving to the local library store writes the document only when the import, checked as private, is accepted as private and local. For a quarantined, rejected or review-required import the save returns the findings and reports that nothing was stored, and the store's contents are unchanged. A library reaches the store by no other path.
- **AC-009** — A hanger library in format 1.0.0 imports with the outcome and findings it had before 1.1.0 existed. A 1.1.0 library with a well-formed working load range imports. A working load range in a 1.0.0 document, a range with a missing end or an extra member, a range whose ends differ in unit, and a range whose minimum is not below its maximum when both are read as binary64 numbers, are each rejected with their named finding. A quantity that is not finite and a repeated record identifier are rejected. After a successful import every optional value that was absent is still absent.
- **AC-010** — For every case of a shared set of invented valid and invalid libraries, each implementation of the boundary returns the same outcome and the same ordered findings by code, path and severity.
- **AC-011** — The shared cases and every test library of the boundary are invented and labelled so, and contain nothing REQ-013 excludes.

## Production and Verification Method — Praxeology

**Method choices that define the boundary.**

- The boundary reads an already-parsed document and nothing else: no file format, no network, no catalogue.
- Strict by default. An absent, unknown or unreadable standing is never read as permission.
- Every nested provenance is checked. A standing stated on a record or a library cannot conceal a rejected or protected provenance beneath it.
- The hanger rules are read from the hanger schema itself by each implementation, with the cross-member rules a schema cannot state applied after the shape check and only to a range that passes it.
- The desktop's runtime boundary and a reference checker outside it implement the same rules, and are held together by shared cases.
- The store re-checks at save. A document is not stored on the strength of an earlier check.

**How each criterion is examined.**

- **VER-001** — Executable check: presents invented libraries of each kind, complete and with each provenance member, status and unit removed or altered in turn, under each visibility; observes the outcome and the finding with its path for each, the precedence among findings, and that an accepted public outcome never arises from private, unknown or unresolved rights.
- **VER-002** — Executable check: observes all eight diagnostic members on every finding produced by the cases of VER-001, and that no finding's message contains the content of the member it concerns.
- **VER-003** — Executable check: presents documents that omit a member their format requires, documents with an unreadable disposition, documents with a value whose unit is not a unit of its stated dimension, and private documents whose redistribution status is unknown; observes that none of the first three is accepted and that a finding names the member or the value's path, and that the last is accepted as private and local only with a finding that names the status.
- **VER-004** — Executable check: saves accepted, quarantined, rejected and review-required imports to the local library store and observes what the store holds before and after each, and the returned findings; opens each stored library and compares its canonical form (RFC 8785), and every unit, with the document that was saved.
- **VER-005** — Executable check: runs the shared invented hanger cases, 1.0.0 and 1.1.0, valid and invalid; observes each outcome and named finding, and compares an imported document with its input for filled-in values.
- **VER-006** — Executable check: runs every shared case through each implementation and compares the outcomes and the ordered code, path and severity of the findings.
- **VER-007** — Judgment by an independent reviewer: a reviewer who did not write the boundary reads its outcomes, messages and documentation against `docs/claims_registry.md` for any statement of legal acceptance, and reads its fixtures against `docs/IP_AND_DATA_BOUNDARY.md` §3 for protected or vendor content.

## Governing Values and Decisions — Axiology

- **AX-001** — *When in doubt, do not accept.* Better is fewer imports accepted on metadata that is missing, unknown or unreadable; the target is none. A smoother import is never bought by reading silence as permission.
- **AX-002** — *Private use stays possible.* Better is fewer complete, properly sourced private libraries that a user cannot bring in. This is never bought by storing or passing on content suspected of being protected.
- **AX-003** — *Findings a user can act on.* Better is more refusals that name the member and say what to supply. A finding never states a legal conclusion and never repeats suspect content.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-019 SOW-044 OBJ-002 OBJ-004 | REQ-001 REQ-002 | AC-001 | VER-001 | PRD §20.2 and `docs/CONTRACT.md` OPS-K-IP-2 state what a public record must carry; the criterion states the expected outcomes |
| OUT-001 | SOW-019 SOW-044 OBJ-002 OBJ-004 | REQ-003 | AC-002 | VER-001 | PRD §20.3 and `docs/CONTRACT.md` OPS-K-PRIV-1 |
| OUT-001 | SOW-019 SOW-044 OBJ-002 OBJ-004 | REQ-004 | AC-003 | VER-001 VER-002 | PRD §20.2 (unclear or suspicious data is quarantined) and `docs/CONTRACT.md` OPS-K-IP-3 |
| OUT-001 | SOW-044 OBJ-004 | REQ-005 REQ-009 | AC-004 | VER-001 VER-004 | PRD §18.4 and `docs/CONTRACT.md` OPS-K-UNIT-1; the stored and reopened library, its units included, is compared in the canonical form an outside standard defines (RFC 8785) with the document that was saved |
| OUT-001 | SOW-044 OBJ-004 | REQ-006 | AC-005 | VER-002 | The criterion states the expected members (decomposition architecture basis for DEL-00-06) |
| OUT-001 | SOW-019 OBJ-002 | REQ-007 | AC-006 | VER-001 VER-007 | `docs/CONTRACT.md` OPS-K-GOV-4 and `docs/claims_registry.md`; independent review of the texts |
| OUT-001 | SOW-044 OBJ-004 | REQ-008 | AC-007 | VER-003 | PRD §18.4 lists the conditions; the required members are those of the formats of DEL-03-01 and DEL-03-02, and the units of a dimension are those of the unit catalog (DEL-02-02), none of which this deliverable writes |
| OUT-001 | SOW-044 OBJ-002 OBJ-004 | REQ-009 REQ-010 | AC-008 | VER-004 | The criterion states what is stored and what is refused; the store's contents are observed before and after each save |
| OUT-002 | SOW-044 OBJ-004 | REQ-011 | AC-009 | VER-005 | PRD §12.4; the shape is that of DEL-03-02's hanger format. The 1.0.0 outcomes are the boundary's own earlier output, which only guards against change |
| OUT-001 | SOW-019 OBJ-002 | REQ-013 | AC-011 | VER-007 | PRD §20.1 and `docs/CONTRACT.md` OPS-K-IP-1; independent review against `docs/IP_AND_DATA_BOUNDARY.md` §3 |
| OUT-002 | SOW-044 OBJ-004 | REQ-012 | AC-010 | VER-006 | Two implementations of the product compared with each other, which only guards against drift. Correctness of the outcomes rests on AC-001 to AC-009; on agreement alone there are no independent grounds |
