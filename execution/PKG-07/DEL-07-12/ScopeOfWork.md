---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-12
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-079]
package_objective_refs: [OBJ-006, OBJ-007, OBJ-018]
---

# Scope of Work — DEL-07-12 Interoperability and evidence-review workspace

## Purpose and Objective Traceability

This deliverable is the set of desktop panels through which a model and its evidence leave the product or are examined before they do. Each panel takes a record that another deliverable defines and produces (an export package, a handoff package, a file in an exchange format, a validation record), shows what it is and what it lost or could not carry, and, where the user asks, hands it off to a file the user chose. The panels add no exporter, no format and no permission. They exist so that an engineer can see exactly what is about to leave, tell an illustration from a record the product actually generated, and take the export deliberately. The result this deliverable contributes is a finite, inspectable route from the desktop to the tools and reviewers downstream of it.

It serves SOW-079 under OBJ-006, OBJ-007 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-12), and PRD §5.8, §5.9, §16.1, §16.2 and §20.3. `docs/RELEASE_TARGET_v1.0.md` names this deliverable, with DEL-17-04, an owner of exit criterion 6 (MBF round trip).

- **OUT-001** — The panels' statement of what a record is: its family, version and identity, validated against its owning contract.
- **OUT-002** — Preservation of findings, units, provenance, loss and unsupported-behaviour information through presentation and export.
- **OUT-003** — The visible distinction between illustrations, previews, references and historical records and actual current generated records.
- **OUT-004** — Explicit handoff: the routes by which a record is written out, the MBF export and import commands among them.

## Deliverable Definition — Ontology

**Terms the workspace implements.**

- *Record family.* A kind of record with an owning contract: a handoff package, an export package in one format, an external-run evidence set, a validation record. A record has a family, a version and an identity.
- *Owning contract.* The schema and validator of a family, held by the deliverable that owns the family. A panel calls it and adds no rule.
- *Actual current generated record.* A record the product generated in this session from the session's current model. Everything else a panel may show is an *illustrative fixture*, a *preview* (a rendering not written by the owning generator), a *reference* or a *historical record*, and is labelled as that.
- *Loss report.* The owning exporter's list of what a target format omitted, approximated or could not represent. *Unsupported-behaviour entry.* A stated behaviour of the model the target does not carry.
- *Explicit handoff.* Writing a record to a file the user chose in a native dialog, after the panel has shown what will be written.
- *The inventory.* The panel families this workspace has, and no others:
  - the native JSON package;
  - the `.mbf` model batch file: export and import;
  - review of evidence from an external run of a licensed program, from files the user supplies;
  - the stress-neutral, PCF and review-geometry exports;
  - the handoff, local-FEA handoff and external-prover boundary panels;
  - the adapter-framework, headless-runner and export-adapter SDK review panels;
  - validation-evidence review.
- *Identity file.* The file saved beside an exported `.mbf` that maps each exported entity to its identifier in the model, by which an import restores the same identities.

**Boundary.** The workspace inspects and hands off. The following belong elsewhere:

- handoff packages, target mapping, the export workflow and prover metadata: DEL-15-01, DEL-15-02, DEL-15-03 and DEL-15-04;
- export packages and the export-unit contract: DEL-17-02. The native JSON package: DEL-17-03. The `.mbf` writer, importer, loss report and identity map: DEL-17-04. External-run evidence and its parser: DEL-17-05. Stress-neutral, PCF and review-geometry exporters: DEL-17-06, DEL-17-07 and DEL-17-08. The adapter SDK: DEL-17-09;
- the adapter framework, the local-FEA data contract and the headless runner: DEL-10-02, DEL-10-03 and DEL-10-05. Build readiness: DEL-10-04;
- validation records: PKG-09;
- redaction and export controls on private data: DEL-12-02;
- review of offline and agent proposals: DEL-07-08. Result tables and their CSV export: DEL-07-05 and DEL-08-04. Reports: DEL-08-01;
- the suggested file name of an export and the guard on replacing the working model: DEL-07-11.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — A panel shows the family, version and identity of the record it holds, and validates it against the owning contract before presenting it as an actual current generated record. A record that fails is shown with the contract's findings and is not presented as output.
- **REQ-002** — Findings, units, provenance, the loss report and unsupported-behaviour entries pass unchanged from the owning generator through the panel to the written file. A panel drops, merges or rewords none of them.
- **REQ-003** — An illustrative fixture, a preview, a reference and a historical record are each visibly labelled as such, and none can be mistaken for an actual current generated record.
- **REQ-004** — A record leaves the product only by explicit handoff, under the export controls on private data. The workspace starts no licensed external program and activates no plugin, provider or destination.
- **REQ-005** — The workspace has exactly the panel families of the inventory. It adds no exporter, schema, adapter family, private-data permission or acceptance workflow.
- **REQ-006** — *MBF in the desktop.* File › Export MBF… shows the writer's loss report before anything is saved, then saves the `.mbf` and its identity file through the native chooser. File › Import MBF… opens a `.mbf` with its identity file and adopts the imported model as the working model through the shell's guard. Closing either dialog, or a change of project, retires the export or import in progress, and a reply that is no longer current is discarded. Each dialog shows the engine's own review and acts only through the writer's and importer's native commands. The round trip of exit criterion 6 is made through these two commands.
- **REQ-007** — No product text of this workspace names the vendor or the program of a third-party format. The MBF export and import are described only as a `.mbf` model batch file.
- **REQ-008** — Every export panel states the unit basis of what it writes, under the export-unit contract, and shows by name the refusal an export gives when a project unit it needs is missing.
- **REQ-009** — No panel presents a record, a validation or evidence status, external-run evidence or prover metadata as code compliance, certification, approval, professional acceptance or verification by an external program. An evidence status is shown in the owning record's terms (PRD §5.9; SOW-079).

### Acceptance criteria

- **AC-001** — For each family of the inventory: a valid record is shown with its family, version and identity; an invalid record and one of another version are shown with the owning validator's findings and are not presented as output.
- **AC-002** — For each family that exports: the findings, units, provenance, loss report and unsupported-behaviour entries in the written file equal those the owning generator returned for the same model, and the panel shows each of them before the file is written.
- **AC-003** — A fixture, a preview, a reference and a historical record each carry their label in every panel that can show one, and a record is labelled actual and current only where its model identity equals the session's current model.
- **AC-004** — In the native desktop: every export writes only to the file the user chose; a cancelled dialog and a denied path write nothing; a payload the export controls block is not written; no external program is started by any panel; and every byte a panel writes is the owning generator's output for the session's model, so that no preview or panel-built record is written.
- **AC-005** — The panel families the workspace mounts are exactly those of the inventory.
- **AC-006** — In the native desktop, with the release target's reference system: Export MBF shows a loss report in which no family of exit criterion 1's reference system, combinations apart, is omitted or unsupported, and saves the `.mbf` with its identity file; Import MBF of that pair gives a working model whose entities carry the identities of the model exported.
- **AC-007** — No text any panel or dialog of the workspace can show contains the vendor's or the program's name.
- **AC-008** — For each family of the inventory, no text a panel shows for a record states or implies certification, code compliance, approval, professional acceptance or verification by an external program, and each evidence status shown equals the one in the owning record.
- **AC-009** — Every export panel shows the unit disclosure of the package it holds (the model's units, the units written and any conversion performed), equal to the package's own disclosure. For a project that lacks a unit an export needs, the panel shows that refusal by its name and nothing is written.

## Production and Verification Method — Praxeology

**Method choices.** Each panel is a view over one owning service and holds no generator of its own. The MBF dialogs are the only two commands of the workspace in the File menu, and each is one dialog over the writer's and importer's native commands. Unit disclosure is one shared helper every export panel calls.

**How each criterion is examined.**

- **VER-001** — Executable check (observes each panel with records of its family): supplies a valid record, an invalid record and one of another version to each panel, and compares what is shown with the owning validator's verdict and findings.
- **VER-002** — Comparison with a named reference (each owning generator's output for the same model): exports from each exporting panel, reads the written file, and compares its findings, units, provenance, loss report and unsupported-behaviour entries with the generator's output and with what the panel showed.
- **VER-003** — Executable check (observes each panel in its fixture, preview, reference, historical and current states): reads the label shown in each state and the model identity behind a record labelled current. An image of a panel does not establish a record's identity.
- **VER-004** — Executable check (observes the native desktop, the file system and the process list): exports to a chosen file, cancels a dialog, requests a denied path and exports a payload the export controls block, reads what was written and which processes ran, and compares the written bytes with the owning generator's output for the session's model.
- **VER-005** — Executable check (observes the mounted workspace): lists the panel families mounted and compares them with the inventory.
- **VER-006** — Executable check (observes the native desktop with the reference system): exports the reference system, reads the loss report and the saved pair, imports the pair, and compares the imported entities' identities with those of the model exported.
- **VER-007** — Executable check (observes every text the workspace can show): scans the panels', dialogs' and messages' texts for the vendor's and the program's names.
- **VER-008** — Comparison with a named reference (the prohibited claims of PRD §21.2 as DEL-15-04's authority-wording screen finds them, and each owning record's evidence status): reads the texts each panel shows for a record of each family and compares them with the screen and the statuses with the record.
- **VER-009** — Comparison with a named reference (the unit disclosure in each package and the named refusal for a project that lacks a needed unit): reads the disclosure each export panel shows and compares it with the package's, then exports from such a project and reads the refusal shown and what was written.

## Governing Values and Decisions — Axiology

- **AX-001** — A panel shows what the owner of the record said. Better is a finding, a loss and a limit shown in the generator's own terms, measured by the written file equalling the generator's output, never a summary that reads better than the record.
- **AX-002** — An illustration is never mistaken for output. Better is a label on everything that is not an actual current generated record, never a preview that looks like the file that will be written.
- **AX-003** — Leaving the product is the user's act. Better is one explicit handoff to a chosen file, measured by nothing written anywhere else and no program started, never an export that happens as a side effect.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-079 OBJ-006 OBJ-018 | REQ-001 | AC-001 | VER-001 | Each family's owning contract and validator are the reference; this deliverable writes none of them. |
| OUT-002 | SOW-079 OBJ-007 OBJ-018 | REQ-002 | AC-002 | VER-002 | Each owning generator's output for the same model is the reference. This shows the panel alters nothing; whether the export is right is the generator's to show. |
| OUT-003 | SOW-079 OBJ-006 OBJ-007 | REQ-003 | AC-003 | VER-003 | The requirement states the labels; the session's current model identity is the reference for a record labelled current. |
| OUT-004 | SOW-079 OBJ-018 | REQ-004 REQ-005 | AC-004 | VER-004 | Owner ruling: PRD §20.3. The file system and the process list are read directly, the block is DEL-12-02's export control applied to a payload the check constructs, and the written bytes are compared with the owning generator's output. |
| OUT-001 | SOW-079 OBJ-006 | REQ-005 | AC-005 | VER-005 | Owner ruling: the inventory stated above is finite. |
| OUT-004 | SOW-079 OBJ-007 OBJ-018 | REQ-006 | AC-006 | VER-006 | Owner ruling: exit criterion 6 of the release target. The reference system is DEL-09-01's fixture and the families are criterion 1's; whether the `.mbf` content is right is DEL-17-04's to show. |
| OUT-004 | SOW-079 OBJ-018 | REQ-007 | AC-007 | VER-007 | Owner ruling: the vendor's name appears nowhere in the product. The names scanned for are fixed strings. |
| OUT-003 | SOW-079 OBJ-018 | REQ-009 | AC-008 | VER-008 | Owner ruling: PRD §5.9 and SOW-079. The prohibited wording is what DEL-15-04's authority-wording screen finds, and the statuses are the owning record's; this deliverable writes neither. |
| OUT-002 | SOW-079 OBJ-007 | REQ-008 | AC-009 | VER-009 | The package's own unit disclosure (DEL-17-02's contract) and DEL-02-02's named refusal for the same model are the reference; the panel writes neither. |
