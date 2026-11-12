---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-07
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-055]
package_objective_refs: [OBJ-006, OBJ-007]
---

# Scope of Work — DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

## Purpose and Objective Traceability

This deliverable is the one path by which the desktop runs a solve and lets the engineer inspect how it went. The user starts a solve; it runs in the background in a separate local process with a memory cap; the desktop shows what the job itself reports, lets the user ask for cancellation, and shows the findings of the run with every field its producer supplied. When the job completes, the native host keeps the result and hands the display a summary and pages of it. Nothing on this path estimates, embellishes or decides: the job's states, the solver's findings and the result's identities are shown as they are. The result this deliverable contributes is that a solve in the desktop is observable, stoppable and traceable, and that the workspace stays usable while it runs.

It serves SOW-055 under OBJ-006 and OBJ-007 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-07), and PRD §14.1, §14.4, §12.8, FR-SOL-015 and FR-GUI-006. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 1, whose solve in the desktop runs through this path, and criterion 9, whose timed stages are the stages of this job.

- **OUT-001** — The solve job in the desktop: start, capped worker, progress, cancellation and refusal.
- **OUT-002** — The presentation of a run: the progress panel, the gate on the Solve command, the diagnostics panel and the run summary.
- **OUT-003** — The reading of a completed result: its summary and paged detail, its verdict, and the records formed from it.

## Deliverable Definition — Ontology

**Terms the solve path implements.**

- *Solve job.* One run of the product solve for one model, started by the user. The job reports one of the states queued, running, cancelled, completed, failed or refused, and a sequence of events. The display adds two readings of its own: "Starting" before the job first reports and "Cancelling" between the user's request and the job's next report.
- *Event.* A message the job reports while it runs. The *stage* shown is the latest event's message.
- *Capped worker.* A separate local process that does the engine's work for one job within the per-invocation memory budget. Ending the worker releases its memory.
- *Refusal.* A job, or a request for one, that ends without a result for a named reason, with an exact code and, where it applies, the quantity that exceeded a limit and the limit.
- *Gate.* The condition under which the Solve command is offered: no solve-blocking finding stands for the model.
- *Diagnostics panel.* The list of the findings a run supplied, filterable by class and navigable to the affected object.
- *Run summary.* The identities that tie a completed run to its inputs: model hash, input manifest, solver version, rule-pack version and checksum, warnings, assumptions and limitations.
- *Held result.* A completed job's result kept by the native host. The display receives a handle and a *summary* formed natively, and reads rows as *pages*: bounded runs of rows in source order. A consumer that reads whole rows (a report, a comparison, a rule check, a derivative export, a handoff panel) receives one copy through the *full-result gate*, only at or under the displayable limit; above it, the gate refuses by name with the size and the limit. Whether those consumers move to reading natively is not decided.
- *Historical run.* A saved or recorded run restored into native custody on reopening. It is held and paged as a current result is, and is marked historical.
- *Solve timeline.* The marks, each with a fixed code, that the solve path makes at its stage boundaries so that a harness can time them.

**Boundary.** This deliverable runs and shows. The following belong elsewhere:

- the product solve entry, the per-invocation memory budget, the shared memory pool, its leases and the pricing of a parse: DEL-04-07;
- what the solver's findings and the model checks say: DEL-04-06 and the engine deliverables that emit them. A field a producer omits is the producer's to supply;
- the job, command and result-envelope rules: DEL-00-03. The fields and classes of a diagnostic: DEL-00-06;
- how a finding's class and fields are shown: DEL-07-04, whose rules the diagnostics panel follows;
- what each status means: DEL-05-04. The product result reader, which classifies a published result and gives its verdict, and REQ-015's reference: DEL-05-02/REQ-018 to DEL-05-02/REQ-020; the reader for retained-precision results is DEL-04-08's;
- the tables and drawings that show a result: DEL-07-05;
- the form of an analysis-run record and of a stored project: DEL-14-02 and DEL-02-05. CSV streaming of a held result: DEL-08-04;
- which marks a performance harness keeps and what it times: DEL-04-05;
- the shell that mounts the panels and the refusal of a solve with no model: DEL-07-11.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — A solve is started only as a job through the solve command. No surface changes solver state or the model directly.
- **REQ-002** — A solve runs in the background, can be cancelled, reports progress, and records the reproducibility metadata the job contract defines. The workspace stays operable while it runs.
- **REQ-003** — Progress shows only what the job reports: its state, its events, its counts and its messages. The display invents no percentage, estimate or time the job did not report.
- **REQ-004** — Cancellation is a request. After it the display reads "Cancelling" and claims nothing. The final state shown is the one the job reports, including completed where the job finished first.
- **REQ-005** — A finding of a run is shown with its code, class, severity, source, affected object, message, remediation and provenance. A field the producer did not supply is labelled as not supplied and is never composed by the display (also cited as DEL-07-07:1).
- **REQ-006** — The six warning classes are preserved through the job, the host and the display.
- **REQ-007** — Invalid input, incomplete model, mechanics solved, rule-check result and human review needed are shown as distinct outcomes of a run. No surface of the solve path shows or implies code compliance, certification, approval, sealing or professional authentication (PRD §21.2).
- **REQ-008** — Missing solve-required data and missing rule-check-required data are kept apart. The Solve command is withheld, naming the findings, only for the first. The solve path supplies no value the model lacks (PRD §5.5).
- **REQ-009** — The run summary of a completed solve shows its model hash, input manifest, solver version, rule-pack version and checksum, warnings, assumptions and limitations, as the result and its run record carry them.
- **REQ-010** — A solve is local. No model, result or finding leaves the machine on this path (PRD §20.4).
- **REQ-012** — *Capped worker.* A solve job's engine work runs in a capped local worker process within the per-invocation memory budget. A request priced over the budget is refused by name before a worker starts. Cancelling ends the worker and releases its memory. The worker's peak physical footprint is measured and reported with the job. Memory for the result is taken from the shared pool under a lease.
- **REQ-013** — *Stages.* A solve job's worker reads and parses the model, calls the product solve entry, serializes the result and verifies it. The host then takes the result into custody, and the display reads its summary and pages and paints them. Each stage boundary is marked on the solve timeline with its fixed code. The marks change no behaviour, keep only timings, counts and codes, and can be read only where the desktop's UI diagnostics global is installed (DEL-04-05).
- **REQ-014** — *Progress panel.* The panel opens when a run starts and stays open after the job ends until the user closes it. It shows the state in a polite live region, the stage, the job's identifier, the queue reason with its exact code, the worker's memory, the cancellation status, the counts of result rows and findings on completion, and for a refusal its reading, the way to retry and its exact code. Cancel is a native button in the tab order; after the request it is disabled with that state exposed, and focus moves to the state line. A record of another job, or of none, closes the panel.
- **REQ-015** — *Result reading.* The summary, each page of detail, the reader's verdict, the analysis-run record, Record Run and Save Project, whichever implementation forms them, equal what the product result reader and the run-record form give for the same result. Until a held result is disposed of, the host keeps with it the digests of the model text the job received and of the result it delivered, so that a later request can show whether the model and result it submits are those this job received and delivered. Disposing of a held result ends any read of it in progress. A result or stored record larger than the displayable limit is refused by name with its size and the limit before its bytes reach the display. A reopened saved or recorded run is restored into custody as a historical run.

### Acceptance criteria

- **AC-001** — For a scripted job that reports a fixed sequence of states and events: the panel shows "Starting", then each reported state and stage in order, then the completion counts the job reported; a queued job shows its queue reason with its exact code, and a refused one its reading, the way to retry and its exact code; a model command issued while the job runs completes; the panel stays open until closed; and no surface of the run shows a percentage, an estimate or a time that the job did not report.
- **AC-002** — For a running job that is asked to cancel: the display reads "Cancelling" until the job reports, with Cancel disabled, that state exposed and focus on the state line; when the job reports cancelled the worker process has ended and its memory is back in the pool; and where the job reports completed instead, completed is what is shown. The worker's memory is shown, and the job reports the worker's peak footprint.
- **AC-003** — A request priced over the per-invocation memory budget is refused with its named code, its price and the budget, no worker starts, and the model document is unchanged.
- **AC-004** — For supplied findings of each class, with and without the optional fields: every supplied field and class is shown, each absent field reads as not supplied, and activating a finding selects its affected object.
- **AC-005** — With a solve-blocking finding standing, the Solve command is withheld and names the finding; with only a rule-check-blocking finding standing, the solve runs and its outcome is shown as mechanics solved with rule inputs incomplete. One supplied case each of invalid input, incomplete model and human review needed is shown as its own outcome. The solve request carries no value absent from the model document, and no text of the panel, the run summary or an outcome states or implies a claim PRD §21.2 prohibits.
- **AC-006** — The run summary of a completed solve shows the model hash, input manifest, solver version, rule-pack version and checksum, warnings, assumptions and limitations that the published result and its run record carry.
- **AC-007** — For each case of a committed set of results: the summary, every page of detail, the verdict and the analysis-run record equal those the product result reader and the run-record form give for the same published result text, and paging returns every row once in source order.
- **AC-008** — A stored record larger than the displayable limit is refused by name with its size and the limit; while a result is held, a submitted model text and result equal to those the job received and delivered match the kept digests, and a substituted or altered text of either does not; disposing of a held result ends a read in progress and leaves nothing to match; and a reopened run is shown as historical.
- **AC-009** — The timeline of one solve carries each stage mark of REQ-013 once, in order, and no build that does not install the diagnostics global exposes a timeline reading.
- **AC-010** — During a solve, from start to the held result, neither the desktop nor the worker opens a network connection.

## Production and Verification Method — Praxeology

**Method choices.** The job registry and the worker live in the native host, and the display polls the job and renders its reports; no state is inferred in the display. The panel is a plain function of the job's latest report. Custody keeps the result in the host, and consumers read it through the same summary and page queries, or through the full-result gate where they read whole rows. Where the host and the display each implement a reading of a result, they are held to equality on a committed set of cases. The gate on the Solve command reads the model checks (DEL-04-06): the session's last run where it is current for the model value, or a run made on the Solve request where none is; while Solve is withheld, the checks run again after each change of the model value. Only the solve's own checks withhold it.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the progress panel against a scripted job): feeds the fixed sequence of reports and reads the panel's state, stage, codes, counts, focus and live-region text after each, issues a model command while the job runs, and scans every surface for a percentage, an estimate or a time the job did not report.
- **VER-002** — Executable check (observes the native job registry and the worker process): starts a job, requests cancellation, and reads the job's reported state, the Cancel control, the worker's process, its shown memory, the peak footprint the job reports and the pool's balance; then prices a request over the budget and reads the refusal and the model document's bytes.
- **VER-003** — Executable check (observes the diagnostics panel and the Solve command with supplied findings): renders findings of each class with and without optional fields, activates one, and reads the fields shown, the session's selection, the command's state under each kind of blocking finding, the outcome shown for each supplied case, and the solve request against the model document.
- **VER-004** — Comparison with a named reference (the published result and its analysis-run record): reads the run summary of a completed solve and compares each identity with the one the result and the record carry.
- **VER-005** — Comparison with a named reference (the product result reader and the run-record form applied to the published result text of each committed case): compares the summary, every page, the verdict and the record with them, byte for byte where the form is bytes.
- **VER-006** — Executable check (observes native custody): stores a record over the limit and reads the refusal; submits the received model text and delivered result of a held job, and substituted texts of each, and reads whether each matches the kept digests; disposes of a held result during a read and submits its texts again; and reopens a saved run and reads its marking.
- **VER-007** — Executable check (observes the solve timeline and the builds): runs one solve where the diagnostics global is installed and reads the marks in order, and scans a build that does not install it for an exposed timeline reading.
- **VER-008** — Comparison with a named reference (the prohibited claims of PRD §21.2 and the claims registry's display forms): reads every text the progress panel, the run summary and the outcome display can show and compares it with them.
- **VER-009** — Executable check (observes the network activity of the desktop and the worker): runs a solve under a network monitor and reads the connections opened.

## Governing Values and Decisions — Axiology

- **AX-001** — The display reports the job; it does not predict it. Better is a state and a stage the job stated, measured by every displayed reading tracing to a report, never a progress bar, an estimate or a claim of cancellation the job has not made.
- **AX-002** — A run that cannot proceed says why and what to change. Better is a refusal with its exact code, the quantity and the limit, never a spinner that ends silently or a result cut short.
- **AX-003** — One result, one copy, one reading. Better is every surface reading the held result through the same queries, measured by native and reference readings being equal on every committed case and by fewer consumers that take a whole copy, never a copy in the display that can drift from the held result.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-002 | SOW-055 OBJ-006 | REQ-002 REQ-003 REQ-014 | AC-001 | VER-001 | The scripted sequence of reports is the scenario, written by hand, and the panel is expected to show exactly it and nothing it did not report. |
| OUT-001 | SOW-055 OBJ-006 OBJ-007 | REQ-001 REQ-004 REQ-012 | AC-002 | VER-002 | The scenario states the expected readings. The worker's end and the pool's balance are read from the operating system and the pool, which this deliverable does not write. |
| OUT-001 | SOW-055 OBJ-007 | REQ-012 | AC-003 | VER-002 | The budget and the price are DEL-04-07's; the expected refusal is theirs applied to the request the check constructs. |
| OUT-002 | SOW-055 OBJ-006 | REQ-005 REQ-006 | AC-004 | VER-003 | Outside source: the diagnostic field list and six classes of DEL-00-06. The expected display is the finding as the check supplies it. |
| OUT-002 | SOW-055 OBJ-006 | REQ-007 REQ-008 | AC-005 | VER-003 VER-008 | PRD §5.5 and §17.5 separate what blocks a solve from what blocks a rule check, and PRD §21.2 lists the prohibited claims. The findings and cases are supplied by the check, and the model document is the reference for the solve request. |
| OUT-002 | SOW-055 OBJ-007 | REQ-009 | AC-006 | VER-004 | Outside this deliverable: the published result (DEL-04-07) carries the identities. The record's form is DEL-14-02's; that the natively formed record equals it is examined by AC-007. |
| OUT-003 | SOW-055 OBJ-007 | REQ-015 | AC-007 | VER-005 | The product result reader (DEL-05-02) and the run-record form (DEL-14-02), applied to the published result text, are the references. This shows custody changes nothing a consumer reads; whether the result is right is the solver's to show. |
| OUT-003 | SOW-055 OBJ-006 | REQ-015 | AC-008 | VER-006 | The scenario states the expected refusal, matches, ended read and marking; the job's received and delivered bytes are the reference for the digests; the size and the limit are read from the store and the host. |
| OUT-001 | SOW-055 OBJ-007 | REQ-013 | AC-009 | VER-007 | The stages are those DEL-04-05's harness times separately; each build is scanned as built. |
| OUT-001 | SOW-055 OBJ-006 | REQ-010 | AC-010 | VER-009 | PRD §20.4 makes solving local. The connections are read from a network monitor outside the product, and the expected count is zero. |
