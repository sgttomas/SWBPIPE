---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-06
package_id: PKG-16
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-069,SOW-070]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-06 Controlled model operation application

## Purpose and Objective Traceability

There is one way a model changes. An accepted, valid, previewed batch of structured operations is applied whole, by one engine, to the model state it was written for; anything else leaves the model exactly as it was and says so. This deliverable is that mutation seam, and it is also the embedded agent that works through it: an assistant in the desktop's conversation panel that reads the model, proposes changes, solves and reports through SWBPIPE's own capability catalog, and whose changes wait for the person. The result it contributes is that every edit, by a person or an agent, arrives at the model by the same checked route, and that a person can ask the agent to build and solve a system without giving up the decision on any change.

It serves SOW-069 and SOW-070 under OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-16, row DEL-16-06), with `docs/PRD.md` §5.3, §10.3 and FR-AGENT-001 to FR-AGENT-003. `docs/RELEASE_TARGET_v1.0.md` exit criterion 17 names this deliverable: from a person's request in the conversation panel, the embedded agent builds the reference system, solves it and reports its key results through SWBPIPE's capability catalog; every change goes through the one operation route and is applied on the person's acceptance or within autonomy the person granted; and the agent records no acceptance, approval or reliance.

- **OUT-001** — Application of accepted operations to the current model.
- **OUT-002** — Refusal without change: every rejected, invalid, blocked or stale request leaves the model as it was.
- **OUT-003** — The one operation engine, in its native build and its web build.
- **OUT-004** — The application outcome, handed to the operation history and to persistence.
- **OUT-005** — The embedded agent host and SWBPIPE's capability catalog.

## Deliverable Definition — Ontology

**Terms of application.**

- *Operation engine.* The single implementation the product uses to validate and apply operations. It is built natively for the desktop and as WebAssembly for the browser build. No other code applies an operation.
- *Apply.* A function of a model document, a batch and the model hash the batch claims. It returns a new model document with an outcome, or no document. It never alters the document it was given.
- *Whole or nothing.* Each operation of a batch applies to the document the previous one left. If any is refused, no document is returned.
- *Basis.* The model hash a batch was written and accepted against. Application requires it to be the hash of the document being changed.
- *Outcome.* What an application returns: the batch, each operation with its author, the diagnostics, the hash applied to, the resulting hash, the record of the person's acceptance of the change, and whether the result has been saved.
- *Publication.* The workspace session taking an applied document as its current model. It is the only way an edit of the current model's content reaches the session; opening, creating or replacing a project, and undo and redo of a checkpoint, are DEL-07-11's and DEL-02-05's session acts.
- *Person-entered value.* The bend flexibility factor k, the SIFs, the other component modifiers (stiffness scaling and stiffness) and their source. Users enter their own k and SIF.

**Terms of the embedded agent.**

- *Embedded agent.* An agent runtime the desktop starts on the person's explicit action and talks to over a private pipe. It reaches a model provider only through the host.
- *Host.* The desktop's native side of the agent: it starts and contains the runtime, validates every tool call, binds it to the session, and returns results.
- *Capability catalog.* SWBPIPE's list of the tools the agent has and the operations it may propose. It is the agent's whole reach into the product. The tools read the model summary, read nodes, read records, propose operations, observe a proposal, solve, and read results. The admitted operations are a closed list of operation kind, object type, change kind and field path; the kinds and object types are drawn from DEL-16-01's taxonomy.
- *Proposal.* A batch the agent submits with its declared assumptions. The host registers it in the review queue, unchangeable, stamped as proposed and agent-authored with the conversation, turn and call it came from. Registering applies nothing.
- *Proposal standing.* Pending review, rejected, withdrawn, applied or unknown.
- *Live basis and retained basis.* A read of the model, a new proposal and a solve are bound to the current model. Observing a proposal and reading a run's results are bound to the model state the proposal or the run was made on, and stay so when the model moves on.
- *Handle.* An opaque reference the host issues for a workspace, a proposal, a run or a page of results. It is valid for one running application and one workspace, is not a path and proves no identity.
- *Provider.* The model service the person configures: endpoint, model identifier, context window and whether a key is needed. The key is held in the operating system's keychain.
- *Relay.* The host's loopback service through which the agent runtime reaches the provider. The runtime holds a per-start token for the relay and never the provider's key.
- *Supplier.* The stock agent runtime binary, pinned by version and by size and hash for each supported architecture.

**Boundary.**

- The record and the taxonomy are DEL-16-01's. Validation and the preview are DEL-16-02's: they are the validate-only mode of this engine. The person's decision, the author stamp and the operation history are DEL-16-03's. What an agent's rationale must carry and what no agent surface may state are DEL-16-04's.
- The rules of a change kind belong to the deliverable that owns the records it changes, which also owns that kind's module in the engine: the model-data records, settings, condition sets and the document validator (DEL-02-01), project units (DEL-02-02), the document version check (DEL-02-05), library application (DEL-03-01), model checks (DEL-04-06) and pressure and bend authoring (DEL-04-07). This deliverable owns the engine that runs them and the operations no other deliverable owns.
- The workspace session, with its live model, revision, review queue and undo and redo, is DEL-07-11's. The review surface with its preview and Apply is DEL-07-08's. Saving is DEL-02-05's.
- The reference system, its key results and its independent references are DEL-09-01's. Building the supplier into the application and signing it are DEL-10-04's.
- What may be sent to a provider is set by `docs/IP_AND_DATA_BOUNDARY.md` §6.1. This deliverable adds no destination beyond the provider the person configured.

The embedded agent host is activated: it is part of the product, not held behind a gate awaiting a decision. It is to work on machines other than the one it was built on, which REQ-011 and AC-009 require and the activation does not establish.

**Not decided.** Whether the agent may ever apply a change itself, within autonomy the person granted, is undecided (DEL-16-03). Until it is decided the catalog has no tool that applies, accepts or grants. Whether an agent may propose person-entered values (k, the SIFs, the other component modifiers and their source, and bend realization) is undecided. Release limitation LIM-001 (`docs/RELEASE_LIMITATIONS_v1.0.md`), which refuses them and leaves the person to enter them during the journey of exit criterion 17, awaits the owner's acceptance.

## Completion and Reliance Basis — Epistemology

### Requirements

- **CLM-001** — Only an explicitly accepted, valid, previewed batch of structured operations is applied, and only to the model whose hash is the batch's basis (SOW-069; PRD §5.3).
- **CLM-002** — A rejected, invalid, blocked or stale request changes nothing, and its outcome never looks like a success (SOW-069; decomposition PKG-16, which allows no hidden model mutation).
- **CLM-003** — One engine applies operations, with the same behaviour in its native and its web build, and with the meaning DEL-16-01's taxonomy gives each kind.
- **CLM-004** — An applied batch returns a deterministic outcome that says what changed, on which state and into which state. The outcome goes to DEL-16-03's history and DEL-02-05's persistence, and neither becomes another way to change a model (SOW-070).
- **CLM-005** — The desktop embeds an agent that operates SWBPIPE through the capability catalog (`docs/RELEASE_TARGET_v1.0.md` exit criterion 17). The host validates every call against the catalog and binds it to the live session. Every agent change enters the review queue as a proposal and is applied only by the person's act, through this engine. The agent's attribution, the model basis of every call, the bounds on what it reads and the true standing of every proposal are preserved. The agent runtime has no shell, file, patch or web capability, and that is established by observing it, not by configuring it.
- **REQ-006** — A batch is applied whole or not at all (decomposition AB-00-03).
- **REQ-007** — An operation whose author is not a person cannot write a person-entered value, by any route: a field edit, a component created with modifiers, a bend realization or insertion, or a library record that copies modifier members (`docs/RELEASE_TARGET_v1.0.md`, Pressure effects: users enter their own k and SIF).
- **REQ-008** — Reconnecting a pipe end refuses a pipe that a component names (a bend span, a tee leg, a rigid component, an expansion joint or a connector span), a pipe with a component other than a terminal marker at its fixed end or at the node it leaves or joins, a pipe that is a member of a pressure region, and a move onto a pressure region's internal node. Each refusal names what blocks it. Reconnecting inside a pressure region while keeping membership is not provided. Deleting a pipe is refused while any record names it and never removes other records.
- **REQ-009** — An agent route that consumes solve or preview results tells native execution from fixture fallback. If the native call fails, the route fails; fixture output is never reported as the result of a run.
- **REQ-010** — From a person's request in the conversation panel, the agent builds the reference system from a blank model, solves it and reports its key results, through the catalog (exit criterion 17).
- **REQ-011** — The agent works on machines other than the one it was built on: the person configures the provider, the key is kept in the keychain, the supplier is bundled for the supported Mac architectures, and where isolation is not qualified the agent is unavailable and says why.

### Acceptance criteria

- **AC-001** — By each product route (a single edit, a batch in review, an agent proposal, an imported proposal), a batch is applied only when it passed validation, its preview was generated, the person accepted it and its basis is the current model's hash. The applied document passes the document validator, differs from the input by exactly the previewed rows with every member the batch does not name unchanged bit for bit, and has the hash the outcome reports.
- **AC-002** — For each of a contract-invalid record, a blocking validation finding, a missing acceptance, a stale basis, a stale before value and a refusal part-way through a batch: no document is returned, the input document is byte for byte the same with the same hash, the outcome says not applied and carries the named diagnostic, and it carries no acceptance record.
- **AC-003** — Every case of a shared set of operations gives the same outcome (validation states, preview rows, diagnostic codes with severity and affected references, boundary records) and the same canonical hash of the applied document in the native build and in the web build. No product route reaches a module outside the engine that applies or validates an operation.
- **AC-004** — For an applied batch the outcome names the batch, each operation with its author, the hash applied to and the resulting hash, and is the same for the same inputs. It says the result is in the session only until it is saved. After save and reopen the stored model's hash is the outcome's resulting hash. No edit of the model's content reaches the session except through an application.
- **AC-005** — With the pinned supplier and a model served on the person's own machine, in the native application, six things are observed. (a) Round trip: the model makes a tool call, the host validates it in full against the catalog whatever the provider did, answers with a value the model could not have known, and the model's reply is grounded in that value. (b) Containment: attempts by the runtime to run a shell command, read or write a file outside its private directory, patch a file, fetch from the web, reach any address but the relay, or start another process each fail, and built-in tool activity by the runtime ends the session. (c) Review: a proposal is registered in batch review with its operations, assumptions and attribution and is applied only by the person's Apply there, and a rejected proposal is observed as rejected. (d) Binding: a call on a stale basis is refused by name; a completion that is unknown, repeated, from another session or from an earlier run of the application is refused and nothing is dispatched again; a turn interrupted while a proposal is pending withdraws that proposal, one interrupted after application began reports the actual result, and an outcome without evidence is reported as unknown, never as success or rejection. (e) Reads: a read is bounded at the catalog's limits before it is collected, keeps the document's stored units, and returns a withheld value as withheld with no substitute number. (f) Custody: the provider key is absent from the runtime's environment, arguments and files, from the webview and from every log.
- **AC-006** — On every route, an operation whose author is not a person and that writes k, another component modifier or the modifier source, realizes or inserts a bend, creates a component with modifiers or applies a library record copying modifier members is refused by `OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED` or `OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED`. The catalog admits none of these. The agent's model summary lists the components that await the person's modifiers, and a solve report names every bend that is not realized, so no result is reported as if it were.
- **AC-007** — Each refusal of REQ-008 occurs, names the blocking component, region or record, and leaves the model unchanged; a reconnect none of them covers is applied; and deleting a pipe that a record names is refused with a list of the records.
- **AC-008** — In the packaged desktop, from requests typed in the conversation panel, the agent builds DEL-09-01's reference system from a blank model through proposals that a person applies, solves it on the exact pressure route and reports the named key results. The model built has every entity of the reference system's specification and no other; each reported value equals the value the results view shows for the same run; nothing was applied except by the person's Apply; and afterwards no record of acceptance, approval or reliance by the agent exists.
- **AC-009** — On a Mac of each supported architecture that holds no developer configuration, the person enters a provider in the panel, the key goes only to the keychain, the agent starts only when the person starts it, and a tool round trip completes. The supplier that runs is the bundled binary whose size and hash equal the pin for that architecture; no other binary is searched for. A locked keychain or a missing key gives an explicit unavailable state and no file is used in its place. On Windows and Linux the panel says isolation is not qualified and no supplier process starts.
- **AC-010** — With the native solve made to fail, the agent's solve and result tools report the failure, and no fixture value appears in a result or in the agent's report.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The engine is the Rust operation applier, compiled natively and to WebAssembly. TypeScript holds the user interface, the builders of operations and a thin routing adapter, and no validation or application logic (`docs/architecture/adr/ADR-0001_operation_seam_engine_unification.md`).
- Apply is pure: it returns a new document and writes nothing. Publishing to the session and saving are separate acts.
- The agent runtime is a stock Codex app-server, release 0.160.0 of `openai/codex`, run as a child process over private standard input and output. The catalog is offered to it as dynamic tools. The runtime is used as published and is not reimplemented.
- `apps/desktop/src-tauri/resources/agent-capability-catalog.json` is the one catalog; there is no second copy to edit.
- The host, not the agent, stamps status, author, attribution, the validation states and the boundary records; arguments that supply them are refused. Numbers the agent writes are kept as the text it wrote.
- The runtime reaches local and hosted providers alike through the host's loopback relay. A provider that is not on the loopback address is reached over HTTPS only.
- The runtime is contained by the operating system: a private home and working directory, one loopback port, and no start of another process. It inherits no developer configuration, instructions or credentials. The supplier's remote control, by which another device could drive the runtime, is switched off by the supplier's own disabling variable in the environment the host writes (the supplier's feature key for it is removed and has no effect), and a reported remote control status other than disabled ends the session.
- The supplier is bundled per architecture and verified against its pinned size and hash before it runs. The pin manifest also names the supplier's licence and notice files.

**How each criterion is examined.**

- **VER-001** — AC-001, executable check: observes the returned document and outcome on each product route for an accepted current batch, a contract-invalid record, a blocking finding, an absent acceptance and a stale hash, and compares the applied document with the previewed rows using a differ that imports no engine code.
- **VER-002** — AC-002, executable check: observes the input document's bytes and hash before and after each refused path, and the outcome each returns.
- **VER-003** — AC-003, comparison between the native and the web build of the engine: the same operation corpus is run through both and their contract-visible outcomes and canonical hashes are compared; with an executable check that no product route reaches a module outside the engine that applies or validates.
- **VER-004** — AC-004, executable check: observes batch, basis and result identity from the application's outcome through publication, save and reopen in the native application, and that each edit of the session's model follows an application.
- **VER-005** — AC-005, executable check with judgment by a named role (a reviewer who did not write the host): runs the pinned supplier and a locally served model in the native application through the round trip, the denied-operation probes, the review of a proposal, and controlled protocol cases for each binding and interruption outcome; the reviewer examines the native commands and permissions the host adds.
- **VER-006** — AC-006, executable check: observes the engine's answer to each write path of a person-entered value under each author type, the catalog's admission of the same paths, and the summary and solve report on a model with unrealized bends.
- **VER-007** — AC-007, executable check: observes each reconnect refusal and its named blocker, an admitted reconnect, and the deletion of a pipe that records name.
- **VER-008** — AC-008, judgment by a named role (the owner, or a person the owner names, applying each proposal in the packaged application), with an executable check that judges the model tree, the results view and the agent's report against DEL-09-01's specification and key-result names. A scripted provider and a stand-in for the person examine the route only and do not settle this criterion.
- **VER-009** — AC-009, executable check: on a clean account of each supported architecture, observes provider entry, the keychain, the binary that runs and a round trip; then a locked keychain and a missing key; and on Windows and Linux the panel and the process list.
- **VER-010** — AC-010, executable check: forces the native solve to fail and observes the agent's tool results and report.

## Governing Values and Decisions — Axiology

- **AX-001** — *One route.* Better is fewer code paths that can produce a changed model; the target is one. No caller, whether an agent, an import or a bulk edit, gets a shorter path that skips validation, the preview or the person's decision.
- **AX-002** — *Truthful outcomes.* Better is a smaller share of outcomes that end as unknown, reached by keeping the evidence of what happened. It is never reached by guessing: an uncertain result stays unknown, and nothing that might apply twice is retried.
- **AX-003** — *An agent that gets the work done.* Better is fewer turns, fewer refused proposals and a smaller prompt for the agent to build and solve the reference system. The weight is on the agent operating the application effectively, ahead of workflow machinery around it. Effectiveness never costs the person's Apply or a person-entered value.
- **AX-004** — *A contained agent.* Better is a smaller set of files, addresses and secrets the agent runtime can reach. The set is never widened to make a provider, a model or a platform work; where containment is not established the agent is unavailable.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | CLM-001 | AC-001 | VER-001 | The applied document is compared with the preview by a differ that imports no engine code |
| OUT-002 | SOW-069 OBJ-015 | CLM-002 REQ-006 | AC-002 | VER-002 | The criterion states the expected result: the same bytes and hash, compared with a copy taken before the request |
| OUT-003 | SOW-069 OBJ-015 | CLM-003 | AC-003 | VER-003 | The two builds are compared with each other on the shared corpus; agreement shows they are consistent, not that either is correct. The corpus's recorded outcomes are the engine's own earlier output and only guard against change. A corpus case is relied on only once an independent reviewer, who did not write the engine, has examined that case, and the cases are shown to the owner with the request for the verdict; a review of earlier cases never covers a case added or changed later |
| OUT-004 | SOW-070 OBJ-015 | CLM-004 | AC-004 | VER-004 | The stored hash is recomputed by an RFC 8785 and SHA-256 implementation that shares no code with the product (DEL-08-02) |
| OUT-005 | SOW-069 OBJ-015 | CLM-005 | AC-005 | VER-005 | The probes are observed at the operating system, the answered value is invented by the host, and the protocol cases state what must be refused |
| OUT-001 | SOW-069 OBJ-015 | REQ-007 | AC-006 | VER-006 | Each write path and author type states the refusal code expected; the catalog's admitted list is read against the same paths |
| OUT-001 | SOW-069 OBJ-015 | REQ-008 | AC-007 | VER-007 | The invented models state which blocker each must name |
| OUT-005 | SOW-069 SOW-070 OBJ-015 | REQ-010 | AC-008 | VER-008 | Exit criterion 17 defines the journey. The specification and the key-result names are DEL-09-01's; the built model is compared entity by entity with that specification; the applying is done by a person |
| OUT-005 | SOW-069 OBJ-015 | REQ-011 | AC-009 | VER-009 | The supplier's size and hash are the publisher's release values recorded in the pin manifest |
| OUT-005 | SOW-069 OBJ-015 | REQ-009 | AC-010 | VER-010 | The forced failure states what must be reported |
