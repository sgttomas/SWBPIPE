---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-03
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-058, SOW-062]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-03 Application service command-query-job model

## Purpose and Objective Traceability

This deliverable is the application-service model: the form in which the GUI, the headless runner, adapters and the embedded agent ask the core to do something. It defines three kinds of request. A command changes durable state. A query reads. A job runs long work such as a solve, a report or an export, shows its progress and can be stopped. Every result comes back as an envelope with its diagnostics and never as a bare success. The result it contributes is that every caller gets the same validated behaviour, a long solve can be watched and cancelled, and each result carries what is needed to reproduce it.

It serves SOW-058 and SOW-062 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-03, and architecture-basis row AB-00-03 in §8.1), with `docs/PRD.md` §5.3, §5.7 and §5.9. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 17, under which every change goes through the one operation route, through DEL-16-06.

- **OUT-001** — The command, query and job model: the three request kinds, what each returns, and the transaction boundary of each kind of operation.

## Deliverable Definition — Ontology

**Terms the model implements.**

- *Application service.* The layer between a caller (GUI, headless runner, adapter, embedded agent) and the domain core, solver, storage and reports.
- *Command.* A request that changes durable state: a model operation, a save, an import. It applies whole or not at all.
- *Query.* A request that reads and changes nothing durable.
- *Job.* A long-running request (solve, report generation, export, validation run) that does not block its caller. It reports progress, can be cancelled, ends in a result envelope and records reproducibility metadata.
- *Result envelope.* The structured return defined by DEL-00-06, carrying the outcome and its diagnostics.
- *Transaction boundary.* The extent of one all-or-nothing effect. A model edit is one operation or one atomic batch, and on failure the model is unchanged. A save writes a complete project state or leaves the stored one intact.
- *Reproducibility metadata.* What binds a job's result to its inputs: the model state, solver version, analysis settings, units, load cases, and rule-pack and library references (`docs/PRD.md` §5.7).
- *Result states.* Mechanics solved, user-rule checked, and human-approved are three different things. The software produces the first two. The third is a person's act and no service produces or records it as its own.

**Boundary.** This deliverable defines the kinds of request and what they guarantee; it implements no service. Its criteria are observed on the services the deliverables below own, and a shortfall is repaired in the owning deliverable's code.

- DEL-00-02 owns the layer map and which module may depend on which.
- DEL-00-06 owns the content of the result envelope and the diagnostic record.
- The deliverables of PKG-16 own model operations: their schema, validation and preview, acceptance, and application.
- DEL-07-07 owns how the desktop shows solve progress and cancellation; DEL-07-11 owns the desktop host that runs jobs.
- DEL-10-05 owns the headless runner, a second caller of the same services.
- DEL-10-01 owns the public API contract's expression of commands, queries and jobs; DEL-00-07 owns which seams are public.
- DEL-02-05 owns the persistence service; DEL-14-02 owns the analysis-run record that holds the reproducibility metadata.
- The interface language of a service and the shape of its cancellation handle are choices of the implementing deliverable.

**Not yet defined.** The transaction boundary of a solve run, of report generation and of an adapter call. Defining each is part of REQ-003.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Mutating commands are separate from read-only queries and from long-running jobs (decomposition AB-00-03).
- **REQ-002** — Every command and job returns a result envelope with diagnostics and makes no bare claim of success. This holds for storage, rule-pack and library operations as for the rest.
- **REQ-003** — Transaction boundaries are defined for model edits, solve runs, persistence operations, report generation and adapter calls.
- **REQ-004** — Background solve, report and export jobs report progress, can be cancelled, and record reproducibility metadata.
- **REQ-005** — The distinction between mechanics solved, user-rule checked and human-approved is preserved (`docs/PRD.md` §5.9; `docs/CONTRACT.md` OPS-K-MECH-2 and OPS-K-AUTH-1).

### Acceptance criteria

- **AC-001** — Every service entry point is exactly one of command, query or job. A query leaves the model's canonical hash and the stored project unchanged.
- **AC-002** — Every command and job returns a result envelope carrying its diagnostics. No entry point reports its outcome by a bare boolean, an exception text or an exit status alone. This includes persistence, rule-pack and library operations.
- **AC-003** — For each kind of operation in REQ-003 whose transaction boundary is defined, as the Ontology establishes it (a model edit and a save), an operation that fails at any step within that boundary leaves the model and the stored project as they were before it. A solve run, report generation and an adapter call are examined in the same way once each boundary is defined (Ontology, Not yet defined).
- **AC-004** — A solve, report or export job reports progress while it runs. A cancel request stops it and returns an envelope that says it was cancelled. A completed job's result records the reproducibility metadata.
- **AC-005** — No envelope, status or record that a service produces states or implies that engineering work is approved, certified, sealed, authenticated or code-compliant (`docs/PRD.md` §21.2). A record of a person's acceptance is kept separate from solver, rule-pack, comparison and report outputs and is never presented as professional approval (`docs/PRD.md` §21.3). Mechanics solved and user-rule checked are separate values that no single flag merges.

AC-002, AC-003 and AC-004 carry commitment DEL-00-03:1.

## Production and Verification Method — Praxeology

**Method choices that define the model.**

- Schema first: each entry point's request and result have a schema contract, in the dialect the baseline names (DEL-00-01).
- A change to the model is a structured model operation. The command surface for model edits is the operation route, the same for a person's edit and an agent's proposal (`docs/PRD.md` §5.3).

**How each criterion is examined.**

- **VER-001** — AC-001, executable check: observes each entry point's declared kind, and the model hash and stored project before and after each query.
- **VER-002** — AC-002 and AC-003, executable check: forces a failure at each step of each kind of operation whose transaction boundary is defined, commands and jobs, and observes the returned envelope, its diagnostic, and the unchanged model and stored project.
- **VER-003** — AC-004, executable check: starts each kind of job on an input large enough to show progress, cancels it, and observes the cancelled envelope; observes each reproducibility field on a completed result.
- **VER-004** — AC-005, judgment by a reviewer who did not write the services: examines the envelope and status vocabularies against `docs/claims_registry.md`, with an executable check that no schema enumerates a professional-approval or compliance value as an outcome of computation.

## Governing Values and Decisions — Axiology

- **AX-001** — *Whole or nothing.* Better is fewer operations that can leave the model or the stored project partly changed; the target is none. A faster edit or save is never bought with a partial write.
- **AX-002** — *Results explain themselves.* Better is a larger share of failures that reach the caller as a diagnostic naming the object and the remedy. An envelope never claims more than was computed.
- **AX-003** — *Long work stays interruptible.* Better is a shorter time from a cancel request to the job stopping, and finer progress. Progress reporting and cancellation never change the numbers of a completed result.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-058 SOW-062 OBJ-013 | REQ-001 | AC-001 | VER-001 | Decomposition AB-00-03; the criterion states the expected result, an unchanged hash and store |
| OUT-001 | SOW-058 OBJ-013 | REQ-002 | AC-002 | VER-002 | Decomposition AB-00-03 and AB-00-06; the forced failures state what must be returned |
| OUT-001 | SOW-058 SOW-062 OBJ-013 | REQ-003 | AC-003 | VER-002 | Decomposition AB-00-03 and `docs/PRD.md` §5.3; the comparison is the model and store before the failed operation |
| OUT-001 | SOW-058 OBJ-013 | REQ-004 | AC-004 | VER-003 | Decomposition AB-00-03 and `docs/PRD.md` §5.7 name the metadata; the cancel scenario states what must happen |
| OUT-001 | SOW-058 OBJ-013 | REQ-005 | AC-005 | VER-004 | `docs/PRD.md` §5.9, §21.2 and §21.3 and `docs/claims_registry.md` say what may not be claimed; the comparison is the envelope and status vocabularies against them |
