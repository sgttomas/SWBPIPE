---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-01
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-056]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-01 Architecture decision record baseline

## Purpose and Objective Traceability

This deliverable is the record of SWBPIPE's architecture decisions. It is the one place where the choices every other deliverable builds on are written down: the language and runtime, the desktop shell and GUI framework, the solver strategy, and the packaging target. Each is recorded with the owner's ruling, the packages it binds and the conditions that would reopen it. The result it contributes is a settled baseline that can be cited, so that no deliverable makes a stack choice of its own and a change of baseline is a visible decision and not drift.

It serves SOW-056 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-01, and architecture-basis row AB-00-01 in §8.1), and answers `docs/PRD.md` §27 questions 3 and 4. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. Criterion 15 (a signed and notarized build) is DEL-10-04's and rests on the packaging target recorded here.

- **OUT-001** — The decision record surface under `docs/architecture/adr/`: an index, a template, and one record per architecture decision.
- **OUT-002** — The architecture baseline: the decision in force for each of the five choice areas of SOW-056 and for each other choice the baseline lists.

## Deliverable Definition — Ontology

**Terms the record implements.**

- *Architecture decision.* A choice that binds more than one deliverable: a language, runtime, framework, library strategy, packaging or storage target, or a seam between modules.
- *Decision record (ADR).* One file per decision, named `ADR-NNNN_<slug>.md`, numbered with four digits from `ADR-0001`.
- *Status.* One of four. *Proposed*: framed for review, no ruling cited, the choice still open. *Accepted*: the owner's ruling is cited and the decision is the current design basis. *Superseded*: replaced by a later record, which it names. *Rejected*: declined by the owner's ruling.
- *Record fields.* Context, alternatives considered, decision, consequences, affected packages, reconsideration triggers, source references, and the human authority for the status.
- *Reconsideration trigger.* A concrete condition under which the decision must be reopened and taken back to the owner.
- *Decision log.* `execution/_Decomposition/SOFTWARE_DECOMP.md` §12. Decisions accepted there stay in force; a record here cross-references its row and does not replace it.

**The baseline in force.**

- *Runtime and GUI.* Rust for the core and application services; a Tauri 2 desktop shell; a TypeScript, React and Vite GUI; a Three.js viewport.
- *Solver library.* An in-repository sparse skyline (profile) direct solver, single-threaded with a fixed operation order, and no third-party numerical dependency.
- *Schemas.* JSON Schema 2020-12 for every public schema and interchange document. Requests and results are defined schema-first.
- *Operation engine.* The WebAssembly build of the operation applier is the only operation engine in the browser; there is no second implementation in TypeScript (ADR-0001).
- *Packaging target.* A macOS desktop application on Apple Silicon first, signed and notarized for v1.0, with Windows taken up after it. A project travels as a multi-member archive with the extension `.opsproj`.
- *Persistence.* Versioned, schema-governed JSON as domain truth, with JCS-compatible canonical hashing; a local SQLite project store and index with rebuildable FTS5 retrieval sidecars; no hosted database.
- *Test baseline.* Cargo tests, Vitest and Playwright, with validation, protected-content and provenance gates.

These seven choices are the baseline. The rules that follow from them are stated in DEL-00-02 to DEL-00-08.

**Boundary.** This deliverable records decisions; the owner makes them. Its own files are the records under `docs/architecture/adr/`.

- DEL-00-02 to DEL-00-08 state the rule that follows from a decision about their own subject.
- DEL-10-04 owns the build, the package and its signing; the solver deliverables of PKG-04 own the solver.
- Exact dependency versions, and the component and state-management libraries inside the baseline, are choices of the deliverable that uses them and are not architecture decisions.

**Not decided.** Whether decisions accepted in the decision log before this surface existed are restated as records here or stay as cross-references. What the project's policy is on the licences of third-party dependencies beyond the solver's choice to have none.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The index lists every decision record with its status, and the status is one of proposed, accepted, superseded and rejected.
- **REQ-002** — Every record follows the template and carries context, alternatives, decision, status, consequences, source references and human authority.
- **REQ-003** — A stack, runtime, framework, library or packaging choice is recorded as accepted only where the owner's ruling is cited. Without a ruling it is recorded as proposed and the choice stays open.
- **REQ-004** — Every accepted decision names the packages it affects and at least one reconsideration trigger.
- **REQ-005** — Records are code-neutral and hold no protected standards text, tables, formulas or proprietary values (`docs/CONTRACT.md` OPS-K-IP-1).
- **REQ-006** — Each choice in the baseline in force has a decision that a reader can reach from the index. The baseline covers at least the five choice areas of SOW-056: implementation stack, runtime, GUI framework, solver library and packaging target.

### Acceptance criteria

- **AC-001** — Every record file in the directory appears in the index exactly once with the same identifier, title and status; every index row resolves to a file; and every status is one of the four.
- **AC-002** — Every record fills each field the Ontology lists under Record fields.
- **AC-003** — Every accepted or rejected record cites the owner's ruling: who ruled, in role terms, and where the ruling is recorded. A record with no such citation has the status proposed.
- **AC-004** — Every accepted record names its affected packages and at least one reconsideration trigger, and every superseded record names its successor.
- **AC-005** — No record, index or template contains protected standards content or proprietary values.
- **AC-006** — For each of the five choice areas, and for each other choice the baseline lists, the index leads to the decision in force, as a record here or as the decision-log row it cross-references.

## Production and Verification Method — Praxeology

**Method choices that define the record.**

- The index is the entry point. A decision that cannot be reached from it is not part of the baseline.
- A record's status changes only on a recorded owner ruling. Superseding a decision adds a new record and marks the old one; the old decision text is not rewritten.
- The decision log stays the record of decisions accepted before this surface, and the two are joined by cross-reference.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002 and AC-004, executable check: observes the record directory against the index (identifiers, titles, statuses, files) and each record's fields.
- **VER-002** — AC-003 and AC-005, judgment by a reviewer who did not write the record: examines that each cited ruling exists where cited and says what the record says, with the protected-content scan run over the record directory.
- **VER-003** — AC-006, judgment by a reviewer who did not write the record: follows each choice in the baseline from the index to its decision in force.

## Governing Values and Decisions — Axiology

- **AX-001** — *One baseline, cited.* Better is fewer architecture choices in force that cannot be followed from the index to an owner ruling; the target is none. This is never bought by recording a proposal as accepted.
- **AX-002** — *Decisions stay reopenable.* Better is more accepted decisions whose reconsideration triggers can be checked against the code or a measurement. A sharper trigger never alters what was ruled.
- **AX-003** — *Few records, each binding.* Better is fewer records for choices a single deliverable could make alone. This is never bought by leaving unrecorded a choice that binds more than one deliverable.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-056 OBJ-013 | REQ-001 | AC-001 | VER-001 | The scope's definition of the four statuses (Ontology, Status) says what must hold; the comparison is the directory's files against the index |
| OUT-001 | SOW-056 OBJ-013 | REQ-002 | AC-002 | VER-001 | The scope's Record fields list says what must be present; the comparison is each record against that list. The template is examined against the same list |
| OUT-001 | SOW-056 OBJ-013 | REQ-003 | AC-003 | VER-002 | Each cited ruling, read where it is recorded and not in the record that cites it; the comparison is the record's status and decision against that ruling |
| OUT-001 | SOW-056 OBJ-013 | REQ-004 | AC-004 | VER-001 | Decomposition AB-00-01 states what an accepted decision names; the comparison is each accepted record's affected packages and triggers, and each superseded record's named successor |
| OUT-001 | SOW-056 OBJ-013 | REQ-005 | AC-005 | VER-002 | `docs/IP_AND_DATA_BOUNDARY.md` and `docs/CONTRACT.md` OPS-K-IP-1 say what is protected; the protected-content scan over the record directory and the reviewer's reading of the records against them |
| OUT-002 | SOW-056 OBJ-013 | REQ-006 | AC-006 | VER-003 | SOW-056 names the five areas and decomposition §8.2 the baseline; the comparison is each choice against the decision the index leads to, in the decision log (§12) or a record |
