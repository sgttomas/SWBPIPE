---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-05
package_id: PKG-10
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-054, SOW-032]
package_objective_refs: [OBJ-008, OBJ-009, OBJ-012]
---

# Scope of Work — DEL-10-05 Headless CLI and structured I/O analysis runner

## Purpose and Objective Traceability

This deliverable is SWBPIPE without its window: one command-line program that takes a JSON request, runs the same solve the desktop runs, and returns JSON. It exists so that solver verification, benchmarks, regression runs and automation do not depend on a person at the GUI. The result it contributes is that the product's solve, input validation, benchmark and regression suites and result export can be produced, repeated and compared by a script, and that a headless result is the product's result for the same request, not a second implementation's.

It serves SOW-054 and SOW-032 under OBJ-008, OBJ-009 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-10, row DEL-10-05, and architecture-basis rows AB-00-03 and AB-00-06), with `docs/PRD.md` §5.5, §5.6, §20.4, §20.5, §21.2, §22.1, §22.4 and §24 R0 (command-line input and output). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches criterion 2 through the benchmark suites it runs, and criterion 12 through the reproduction procedure of DEL-09-04, which is carried out with this runner.

- **OUT-001** — The headless runner: the program `openpipestress-runner` and its request and result contract `schemas/headless_runner.schema.yaml`.

## Deliverable Definition — Ontology

**Terms the runner implements.**

- *Verb.* What the runner is asked to do. The stable verbs are five, one for each runner operation: `solve`, `validate-input`, `export-results`, `run-benchmark` and `run-regression`.
- *Request.* A schema-first JSON document, read from standard input or from a file the caller names. It holds the runner request (the operation, references, privacy context and requested outputs) and, beside it, the payload its verb needs: a model document for `solve`, a suite and an optional list of cases for `run-benchmark` and `run-regression`, a report-package request for `export-results`.
- *Result.* Structured JSON on standard output: the validation of the request and of the result, the runner result, the mechanics envelope or suite report or exported package, or a hosted command's document, and diagnostics, under the local-private control's decisions, with the run evidence beside them. It is written to a file only at a path the caller gives.
- *Run evidence.* A member of every result the control passes, formed in the run from what the runner read and what it returns: the SHA-256 of the request's bytes as read; the SHA-256, over the canonical profile `openpipestress_jcs_ijson_v1`, of the request, of the model document it carries or a stated absence, and of the result less its run evidence; the command, solver mode and intent flag as invoked; the runner's and the product's versions and build reference; and one SHA-256 over all of these. Where the control would block the request itself, the request and model digests are withheld and stated as withheld, and the result digest is kept. Anyone holding the request, the model document and the result can check from them alone that they belong together, and can check where the result came from by running the same request on the build named and comparing. On a build that is not registered the build named is the components' versions and not a revision. It is not a signature: whoever can rewrite a result can rewrite its evidence, and whoever removes it leaves a result that reads as one made before run evidence existed, so its absence establishes nothing; only a consumer that requires it, as the unsealing of a capture does (DEL-17-05), closes that.
- *Solver mode.* `sparse_interactive`, the default, or `dense_scrutiny`. Only `solve` takes it.
- *Exit status.* `0`: completed with no blocking diagnostic. `1`: completed with a blocking diagnostic, or validation failed. `2`: wrong usage, unreadable input, malformed JSON or an unsupported operation, and no result was formed.
- *Process policy.* One local process in the foreground. No daemon, no separate solver process, no network listener or connection, no telemetry, no write the caller did not name, no scan of the user's home, no write of private data to a repository default location, and no SQL or direct access to a project store.
- *Local-private intent.* The caller's explicit statement, by a flag, that output holding private values is meant for a local private destination. Without it such output is blocked.
- *Suite.* A benchmark or regression suite owned by a validation deliverable: mechanics (DEL-09-01), stress (DEL-09-02) and nonlinear (DEL-09-03). A suite holds its cases, their recorded comparison values and the comparison that decides a match.
- *Hosted command.* A command of the same program that belongs to another deliverable and passes through the runner: `read`, which returns a product result reader's verdict (DEL-05-02, DEL-04-08); `size-hangers`, which runs hanger sizing (DEL-04-11); `modal`, which runs a modal request (DEL-04-09); and `spectrum`, which runs a response-spectrum request (DEL-04-10).

**Boundary.** The runner is a caller of the product's services. It contains no solver, no unit conversion, no rule evaluator, no result format and no engineering default of its own.

- The solve it runs is DEL-04-07's; the diagnostics for a singular or incomplete model are DEL-04-06's.
- The result export is DEL-08-04's and the report package DEL-08-01's. A report released by a user is a person's act in the desktop, and the runner does not produce one.
- The control that blocks private values is DEL-12-02's.
- The suites, their recorded values and their tolerances are DEL-09-01's, DEL-09-02's and DEL-09-03's. A match is evidence about the solver's behaviour against those values and is no release threshold.
- The reproduction procedure that uses the runner is DEL-09-04's. The desktop panel that shows runner records is DEL-07-12's.
- Building and testing the runner's crate in CI is DEL-10-04's. The runner is not a public network API; the public transport is undecided under DEL-10-01.

**Not decided.**

- Whether the stable verb set grows beyond the five. The stable verbs are five; the hosted commands `read`, `size-hangers`, `modal` and `spectrum` are not among them.
- Whether the runner reads a persisted project. It takes a model document in its request and opens no project store.

## Completion and Reliance Basis — Epistemology

A headless result is relied on in two ways: as the product's own answer for a request, and as the observed side of a comparison. The first rests on the runner adding nothing to the solve; the second on the references the suites hold, which are the suites' to justify.

### Requirements

Requirement numbers 001 to 009 are those of the earlier `R-nn` series.

- **REQ-001** — The runner executes through the schema-first command, query, job and result-envelope boundary and has no path to the solver around it (decomposition AB-00-03).
- **REQ-002** — Structured input and output are unit-aware, deterministic and reproducible (SOW-054; OBJ-012; PRD §5.6).
- **REQ-003** — A missing solve-required or rule-check-required input, a missing verb payload and an unsupported input each produce a blocking diagnostic, and never a default (PRD §5.5).
- **REQ-004** — Result output is the product's result export and diagnostic envelope, so that regression comparison and report and result export consume it without another shape.
- **REQ-005** — A diagnostic keeps its code, class, severity, source, affected object, message, remediation and provenance, and states no compliance or certification (decomposition AB-00-06).
- **REQ-006** — The program is named `openpipestress-runner` and its stable verbs are the five above. Its canonical input is a schema-first JSON request from standard input or a named file, and its canonical output is structured JSON on standard output, with file output only at a path the caller gives.
- **REQ-007** — Public examples and fixtures of the runner use invented or otherwise permitted data and embed no protected standards content (PRD §20.1).
- **REQ-008** — The runner transmits no private project, material, component or rule-pack data, and output that holds a private value leaves the process only under explicit local-private intent (PRD §20.3 to §20.5).
- **REQ-009** — A runner result may state the mechanics-solve and user-rule-check statuses. It states no code compliance and no approval (PRD §4.3, §21.2).
- **REQ-010** — The runner keeps the process policy above on every verb and every hosted command.
- **REQ-011** — The runner exits with the statuses above and no others.
- **REQ-012** — `solve` runs the product's solve entry and nothing else: for the same request the headless result is the result the desktop gives, in each solver mode, and a model the product refuses is refused with the same named diagnostic.
- **REQ-013** — `run-benchmark` and `run-regression` run the named cases of a suite, or every case when none is named, and report for each case the recorded and the observed outcome and whether they match under the suite's own comparison. The runner holds no tolerance of its own. An unknown case, an unsupported suite and a case whose recorded basis cannot be used are blocked with a diagnostic, and no case is skipped silently.
- **REQ-014** — `export-results` returns the package its request describes, built by the owning deliverables' builders, with each member's hash. A request that carries a user-release marker is refused by name.
- **REQ-015** — A hosted command passes its input to the owning deliverable's entry and returns that entry's document unchanged. The runner supplies no engineering value to it, and the process policy, the local-private control and the exit statuses apply to it as to a verb.
- **REQ-016** — A model document a request carries is judged by the model-document format and the document validator (DEL-02-01) before any solve. A document that fails the format, or carries a member the validator refuses, is blocked with that finding's named code and JSON path, and nothing is solved, so that the runner is no way around the checks every entry path applies (DEL-02-01/REQ-010).
- **REQ-017** — Every result the runner writes for a verb or a hosted command that takes a request to the product carries run evidence formed in the same run from the request as read and the result as returned, so that a result paired with a request or a model document it did not answer fails a check that needs only the kept bytes. A digest a caller supplies is never taken as evidence. The `read` command is the exception: it runs no request, it returns a product reader's verdict on a statement handed to it, and that verdict's envelope is closed (DEL-05-02, DEL-04-08).

### Acceptance criteria

- **AC-001** — The program accepts the five verbs and the hosted commands, with an input path, an output path, the local-private intent flag and, for `solve` only, a solver mode. Any other argument, a flag missing its value, an unknown solver-mode value, or a solver mode on a verb other than `solve` exits `2`, names the problem and forms no result.
- **AC-002** — For each verb: a run with no blocking diagnostic exits `0`; a run with one, or a request that fails validation, exits `1` with the diagnostic in its result; and unreadable input, malformed JSON and an unsupported verb exit `2`. A run that exits `2` runs no solver, suite or exporter, and anything it writes to standard output is only a blocking diagnostic that names the problem, with no runner result, mechanics envelope, suite report or package.
- **AC-003** — While it runs any verb or hosted command, the process opens no network connection, starts no other process, and creates or changes no file other than the output path the caller gave. A run with no output path changes no file.
- **AC-004** — A request and a result each validate against the runner contract. A request that fails it, or that lacks the payload its verb needs, is blocked with a diagnostic naming the member, and the solver, suite or exporter is not run. A request quantity with an unknown unit, a unit of the wrong dimension or no unit on a unit-bearing member is refused with its unit finding before any solve, and every quantity in a result carries its unit. A model document that fails the model-document format, or carries a member the document validator refuses, is blocked with that finding's code and JSON path before any solve.
- **AC-005** — The same model document through `solve` and through the desktop's solve entry gives byte-identical published results and diagnostics in each solver mode, and a model the product refuses is refused by both with the same code and affected object.
- **AC-006** — The same request run twice gives byte-identical standard output, and every hash in a result equals SHA-256 over the RFC 8785 canonical form of what it names, as computed by an implementation that imports no product code.
- **AC-007** — A whole-suite run reports every case of the suite's inventory. A case whose observed value departs from its recorded value under the suite's comparison is reported as a mismatch and the run exits `1`. An unknown case and an unsupported suite are blocked by name, and a regression suite is not accepted by `run-benchmark` nor a benchmark suite by `run-regression`.
- **AC-008** — The document `export-results` returns validates against the schemas of the result export and the report package, and each member's stated hash is the hash of that member. A request with a user-release marker returns a named refusal and no package.
- **AC-009** — Output that holds a value classed private is blocked without the local-private intent flag: the run exits `1`, the payload is absent and no file is written. With the flag the payload is returned with a warning.
- **AC-010** — The analysis status of every result is one of the six automatic statuses of DEL-05-04, a rule-check outcome maps only to the three user-rule statuses, every diagnostic has the members of REQ-005, and no output contains a statement of compliance, certification, sealing or approval.
- **AC-011** — For each hosted command, the payload the runner returns equals, by canonical value, the document the owning deliverable's entry returns for the same input, with nothing added to it or removed from it except the local-private control's decisions. Its exit status is `0` when that entry accepted or passed, `1` when it refused or the control blocked, and `2` when the input was unusable.
- **AC-012** — Every example and fixture of the runner is invented, carries its provenance note and contains no protected or private content.
- **AC-013** — For each verb, and for each hosted command other than `read`, admitted and refused, the digests of a result's run evidence equal those computed, by an implementation that imports no product code, from the request bytes the runner read, the model document the request carries and the result less its run evidence. Checked against another request, against a model document changed in any member, or after any change of its own, the result fails by the named digest. The same request run again on the build the evidence names gives the same result digest. A request or result outside the canonical profile is stated as such and never hashed in another form. A blocked result and a run that exits `2` carry no run evidence. Where the control would block the request itself, a result it passes carries the result digest and states the request and model digests as withheld, member by member, so that it reads neither as complete nor as a result made before run evidence existed; on no verb or hosted command, with or without local-private intent, admitted or refused, does a digest of content the control classes as private leave the process in a result the control did not block. Evidence with a withheld binding, and a result without run evidence, bind nothing: a check that requires the binding fails on both.

## Production and Verification Method — Praxeology

**Method choices that define the runner.**

- *A thin caller.* Each verb decodes its request, validates it, calls one entry of the owning deliverable and wraps what comes back. Anything more belongs to the owner of that entry.
- *JSON in, JSON out, one process.* No state survives a run, so a run is reproduced by repeating its request.
- *The payload beside the request.* A verb's payload sits next to the runner request under the verb's name, so that the request contract stays the same for every verb.
- *Suites decide matches.* A case's recorded values and its comparison are read from the suite; the runner reports and never judges.
- *Blocked, not degraded.* A run that cannot do exactly what was asked returns a blocking diagnostic and does nothing partial.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the program's arguments, exit status and output): invokes it with each verb and hosted command, with valid, invalid and missing arguments, payloads and inputs, with request quantities whose unit is unknown, of the wrong dimension or absent, and with model documents that fail the format or carry a member the document validator refuses, and reads the status, the diagnostics, whether a result was formed and the unit of each result quantity.
- **VER-002** — Executable check (observes the process from outside it): runs each verb and hosted command under an operating-system trace of network, process and file activity, with and without an output path.
- **VER-003** — Comparison with a named reference: the published result and diagnostics of the desktop's solve entry for the same model documents, admitted and refused, in each solver mode.
- **VER-004** — Comparison with a named reference: each hash in a result against SHA-256 over RFC 8785 canonical JSON computed by an implementation that imports no product code; and the standard output of two runs of one request against each other.
- **VER-005** — Executable check (observes suite reports): runs named cases and whole suites, a case altered so that its observed value departs from the recorded one, an unknown case and a suite given to the wrong verb, and reads the per-case report and the exit status.
- **VER-006** — Executable check (observes exported packages and the private-value control): validates each exported document against the result-export and report-package schemas with a validator that is not the runner's, recomputes member hashes, presents a user-release marker, and runs requests holding private values with and without the intent flag.
- **VER-007** — Comparison with a named reference: for each hosted command, the document the owning deliverable's entry returns for the same input.
- **VER-008** — Judgment by a named role (an independent reviewer who did not write the runner): reads the contract, the examples and the fixtures against PRD §20.1 and §21.2 and reports any protected or private content and any prohibited claim.
- **VER-009** — Comparison with a named reference: each digest of the run evidence against SHA-256 over the `openpipestress_jcs_ijson_v1` canonical text computed by the test-only verifier that imports no product code, for the invented requests of VER-001 and for substituted, edited and re-paired requests, model documents and results; and the same request run twice giving the same result digest.

## Governing Values and Decisions — Axiology

- **AX-001** — *The product's answer, not another.* Better is fewer places where the runner could differ from the desktop for the same request, measured by the code between a request and the owning entry that is not decoding, validation or wrapping. A convenience for automation is never a reason to add a default, a tolerance or a solver path here.
- **AX-002** — *Repeatable by a stranger.* Better is a run that someone else can repeat from its request alone and get the same bytes, measured by the output that differs between two runs of one request, which is none. Richer output is never bought with a timestamp, a host detail or an ordering that varies.
- **AX-003** — *Quiet process.* Better is fewer outcomes that a script has to tell apart by reading message text: every outcome is told from the exit status and the structured diagnostics. No capability is added by opening a connection, starting a process or writing somewhere the caller did not name.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-054 OBJ-009 | REQ-006 | AC-001 | VER-001 | REQ-006 states the program name, the five verbs and the input and output forms; each invented invocation states whether it must be accepted |
| OUT-001 | SOW-054 OBJ-008 | REQ-003 REQ-011 | AC-002 | VER-001 | The three exit statuses and what each means; each invented invocation states the status it must get |
| OUT-001 | SOW-054 OBJ-009 | REQ-008 REQ-010 | AC-003 | VER-002 | The process policy says what the process may do. What the process did is read from an operating-system trace, which the runner did not write |
| OUT-001 | SOW-054 OBJ-012 | REQ-001 REQ-002 REQ-003 REQ-016 | AC-004 | VER-001 | The runner contract says which requests are valid; each invented faulty request states the member its diagnostic must name. The expected unit finding comes from DEL-02-02's unit catalog, which says which symbols exist and the dimension of each, and the expected document finding from DEL-02-01's format and validator rules |
| OUT-001 | SOW-054 OBJ-008 OBJ-012 | REQ-012 | AC-005 | VER-003 | Equivalence only: the runner's result is compared with the desktop entry's for the same request. That the values are right is DEL-04-07's criterion, against references that import no product code |
| OUT-001 | SOW-054 OBJ-012 | REQ-002 | AC-006 | VER-004 | Outside source: RFC 8785 and SHA-256, computed by an implementation that imports no product code. Determinism is one run compared with another |
| OUT-001 | SOW-054 SOW-032 OBJ-008 | REQ-013 | AC-007 | VER-005 | The suites' recorded values say what each case must give; where those values come from is the criterion of DEL-09-01, DEL-09-02 and DEL-09-03. The nonlinear regression values are the product's own earlier output and only guard against change. The altered case states the mismatch the runner must report |
| OUT-001 | SOW-054 OBJ-009 | REQ-004 REQ-014 | AC-008 | VER-006 | The schemas of DEL-08-04 and DEL-08-01 say what a valid document is, applied by a validator that is not the runner's; member hashes are recomputed outside the runner |
| OUT-001 | SOW-054 OBJ-009 | REQ-008 | AC-009 | VER-006 | PRD §20.3 keeps private data from leaving unexpectedly; each invented request states whether its output must be blocked |
| OUT-001 | SOW-054 OBJ-008 | REQ-005 REQ-009 | AC-010 | VER-001 VER-008 | PRD §4.3 and §21.2 list the prohibited statements. The status and field sets are DEL-05-04's and DEL-00-06's, not the runner's |
| OUT-001 | SOW-054 OBJ-009 | REQ-015 | AC-011 | VER-007 | The owning deliverable's entry says what the document must be; the hosted command's payload is compared with it by canonical value |
| OUT-001 | SOW-054 OBJ-009 | REQ-007 | AC-012 | VER-008 | PRD §20.1 lists what public content may not contain; the reviewer compares |
| OUT-001 | SOW-054 OBJ-008 OBJ-012 | REQ-017 | AC-013 | VER-009 | Outside source: RFC 8785 with the I-JSON profile, and SHA-256, computed by an implementation that imports no product code; each substitution states the digest that must differ |
