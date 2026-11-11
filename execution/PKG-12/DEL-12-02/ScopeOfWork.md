---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-02
package_id: PKG-12
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-040]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-02 Private data redaction and export controls

## Purpose and Objective Traceability

This deliverable decides what of a user's private and protected data may leave the product, and in what form. Every way out (a report on screen, a saved file, a print, a download, a handoff, a command-line output) is a catalogued route. Before a route exposes anything, its content is classified under one export context and each value is included, kept with a warning, redacted, omitted, or the whole export is blocked, and the decisions are returned where the user and the caller can see them. Its result is that a shared or public output never silently carries private values or suspected protected content, that a local private output carries them only on the user's explicit intent, that a user can share their own project data in one report by an explicit release for that report, and that none of this changes the source model.

It serves SOW-040 under OBJ-010 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-12, row DEL-12-02), PRD §20.3 (private data excluded from public examples and issue bundles by default), PRD §19.4 and FR-REP-005 (report prohibitions) and FR-RULE-004 (private rule packs excluded unless explicitly exported). Exit criterion 4 of `docs/RELEASE_TARGET_v1.0.md` names this deliverable with DEL-08-01: a user releases, saves and prints a report for results on either solve route, with the PRD §19.3 notice and a reproducible canonical hash. This deliverable's part of that criterion is the classification under which a user's own project data becomes shareable in one report, and the admission of the route that carries it. The report, its package, the record of the release, the notice and the hash are DEL-08-01's.

- **OUT-001** — The export control contract: `docs/security/redaction_export_controls.md` and `schemas/redaction_export_controls.schema.yaml`. Together they state the export contexts, the privacy classes, the actions, the decision order, the reading of item metadata, the findings and the controlled export.
- **OUT-002** — The classifier: the decision for one item and the redaction of one payload, in each language the product needs it in.
- **OUT-003** — The route catalogue and route admission: the governed routes, each route's fixed context, the intent its wrapper supplies, and the admission decision made from route metadata.
- **OUT-004** — The native screens on the desktop's saves: the desktop's own reading of the content it is asked to write, made before the save dialog.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Export.* Any exposure of project-derived content outside the session's working model: report markup, a render call, a preview, a saved file, a print, a download, standard output, an output file, a handoff to another tool.
- *Export context.* Exactly one per export, fixed by the route:

  | Context | Meaning |
  |---|---|
  | `public_report`, `public_example` | Public output. Public metadata and invented public examples are included; private and unresolved values are redacted; suspected protected content and professional claims block. Examples are invented or reviewed public-permissive content |
  | `shared_model` | A model shared with another party. Private project, material, component, rule-pack, owner-standard, design-basis, path and secret-like data are redacted unless metadata supports sharing |
  | `downstream_tool` | A handoff to another tool. Unresolved and private values are redacted unless metadata supports the handoff |
  | `local_private` | The user's own local output. Private values are kept only on the user's explicit intent, each with a warning finding |
  | `user_released_report` | One report of the user's own session in which the user releases their own project data for sharing, with their statement that they hold the redistribution right |

- *Item metadata.* What a value says about itself: field class, privacy class, redistribution status, review status and provenance. Decisions rest on this explicit metadata only. Sensitivity is never inferred from how a value looks, from a file name or from a text pattern.
- *Privacy classes.* Private: project data, material data, component data, rule-pack data, owner-standard data, company design-basis data, path data and secret-like data. Safe: public metadata and invented public examples. A *public redistribution status* is public permissive, or the status the product's schemas give an invented example. *Safe metadata* is what may stay visible when marked public: identifiers, versions, checksums, source notes, warning summaries, schema versions and provenance summaries.
- *Action.* `include`, `warning_only`, `redact_value`, `redact_field`, `omit_field` or `block_export`. In strictness, `include` < `warning_only` < `redact_value` < `block_export`.
- *Finding.* The machine-readable record of a decision that is not a plain inclusion. It carries the fields DEL-00-06 requires of a diagnostic (code, class, severity, source, affected object, message, remediation, and provenance where applicable), with the affected path as its affected object, and the action it records. The classes are `IP_BOUNDARY_WARNING`, `PRIVATE_DATA_WARNING`, `PROVENANCE_WARNING`, `STORAGE_BOUNDARY_WARNING` and `PROFESSIONAL_BOUNDARY_WARNING`.
- *Controlled export.* What every route returns before it exposes anything: the controlled payload or none, the decisions, the findings, whether the export is blocked, and a summary. The controlled payload is a copy; the source is untouched.
- *Route.* One final sink with an identifier and a fixed export context. A route's wrapper supplies the user's intent. Intent carried inside a payload is never read and grants nothing.
- *Route catalogue.* The set of governed route identifiers. An identifier outside it is refused. Some routes admit only `local_private`, the result-table CSV save and the MBF export among them. One route admits only `user_released_report`.
- *Route admission.* The decision on one route's own metadata (identifier, export context, storage context, intent), made with DEL-12-01's storage guard and returned as evidence with an action, a reason code and a blocked flag. It reads no payload.
- *Lossless route.* A route whose format cannot carry a redaction without corrupting its meaning. It is withheld whole when any value would be redacted, omitted or blocked.
- *Decision order.* In every context an item is decided in this order, and an earlier block is final:
  1. a professional claim blocks;
  2. a cloud or network reference blocks;
  3. direct SQL access blocks;
  4. a storage bypass blocks;
  5. secret material blocks;
  6. a contained payload blocks;
  7. a privacy class or redistribution status of suspected protected content blocks;
  8. a review status of quarantined or rejected blocks;
  9. the context's own rule applies;
  10. a concrete path is redacted, unless step 9 blocked.
- *The context's own rule.* In the public, shared and downstream contexts: a safe class with a public redistribution status is included; a private class is redacted; missing or unknown metadata is redacted. In `local_private`: a private class is kept with a warning when the wrapper states the user's intent and blocks without it; missing or unknown metadata is kept with a warning. In `user_released_report`: a private class other than project data is redacted whatever its redistribution status; private project data is kept with a warning when the wrapper states the user's release and blocks without it; public metadata and invented public examples are included; anything else, missing and unknown metadata included, blocks.
- *Disposition member.* A member named `redistribution_status`, `review_status`, `privacy_class`, `privacy_classification`, `classification` or `lifecycle_status`.
- *Blocked disposition.* A redistribution or review status of suspected protected content, quarantined or rejected, as DEL-03-07 decides on import.

**Reading of item metadata.** The contract document states these rules in full, and every implementation follows them:

- a privacy or redistribution label that is neither text nor null matches no class, and is not treated as unknown, safe or absent; a null or absent label falls back to the item's provenance;
- the `classification` alias is read only when `privacy_classification` is absent, never when it is present and null;
- an object carrying its own `__proto__` member gets no public basis, and a decision that would have been an inclusion becomes the unknown-label decision; every other decision stands;
- an input that carries one of the exporter's reserved internal names is blocked whole;
- a record holding a value with metadata but no label takes the stricter of its decision as one item and its decision member by member, and a restriction covers the whole subtree beneath it;
- a blocking flag is set by `true`, a non-zero number or a recognized word; an intent flag is set only by `true` or a recognized word;
- `professional_claim` blocks unless it is absent, null or `false`;
- the desktop's entry points accept only a plain JSON data graph, read once, and refuse anything else whole.

**Boundary.**

- DEL-08-01 owns the report: its assembly, rendering, saving, printing and package, the record of a user release with its binding to the model and result, and the notices. DEL-08-03 and DEL-08-06 own report sections, DEL-08-04 the result JSON and CSV exports, DEL-08-05 the protected-content linter and the release scan. DEL-15-01 and PKG-17 own the handoff and the export formats. Each brings its outputs out through a catalogued route.
- DEL-12-01 owns the path classes and the storage guard. DEL-12-04 owns the release of private-library and credential references. DEL-12-03 owns telemetry.
- DEL-03-07 decides whether a library is admitted, and DEL-06-04 keeps a rule pack's identity, checksum and privacy standing. This deliverable reads the status they record.
- The embedded agent's model-provider channel (DEL-16-06) is runtime handling of the user's own data, not an export.
- The control moves no quarantined material, transmits nothing and decides no legal right. A clean result is review evidence, not legal or professional clearance.

**Not decided.**

- Whether a user can save a redaction profile with a project, and where it would be kept. The schema defines only a profile's form.
- What is done with content the control blocks as suspected protected, beyond blocking and reporting it.

## Completion and Reliance Basis — Epistemology

The control is adequate when, for any content offered to any route, the outcome can be predicted from the rules above, is observed before anything is exposed, and is the same wherever the rules are implemented.

### Requirements

- **REQ-001** — Every export is classified under one export context before project, rule-pack, material, component, owner or design-basis data is exposed (SOW-040; `docs/CONTRACT.md` OPS-K-PRIV-1).
- **REQ-002** — A shared or public export never silently includes private project data, private rule-pack values, private material or component values, protected standards content, copied formulas, proprietary templates or vendor data without documented rights (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-REPORT-2).
- **REQ-003** — Public report templates and public examples contain no protected formula, standards table, copied code text, proprietary report template or private project example (PRD §19.4, FR-REP-005).
- **REQ-004** — The control supports at least warning, value redaction, field redaction, export blocking and an explicit allowance for local private export.
- **REQ-005** — An unknown or insufficiently established redistribution status leads to a warning, a redaction or a block, never to silent public inclusion (`docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-006** — A local private export keeps private values only when the context is `local_private` and the user's intent is explicit. The export's record keeps a warning for each value kept.
- **REQ-007** — Redaction works on a copy made for the export. It never changes the source project model, a private library or a rule pack.
- **REQ-008** — A redacted export keeps the evidence that is safe to keep: model and report hashes, solver and report versions, input-manifest identifiers, warning summaries, rule-pack identity, version and checksum, and provenance summaries (`docs/CONTRACT.md` OPS-K-REPORT-1).
- **REQ-009** — In a public or shared report, a rule pack appears only as safe metadata (identifier, version, checksum, source note) unless the user has documented rights to include more (`docs/CONTRACT.md` OPS-K-RULE-3; FR-RULE-004).
- **REQ-010** — Material, component, section, stress-intensification and flexibility values, allowables, vendor values and design-basis fields bring their provenance and their privacy and redistribution status into the decision (`docs/CONTRACT.md` OPS-K-DATA-3).
- **REQ-011** — Every redaction, protected-content and private-data decision that is not a plain inclusion is a machine-readable finding, with `IP_BOUNDARY_WARNING` where content may cross the public boundary. The contract's other finding classes are local classes of this boundary (DEL-00-06).
- **REQ-012** — No adapter, plugin, command-line export, report preview or export, or downstream handoff exposes content around the control. A route outside the catalogue exposes nothing.
- **REQ-013** — The control keeps mechanics solved, user-rule checked and human-approved distinct, and blocks any content that claims certification, sealing, approval, authentication or code compliance (PRD §21.2).
- **REQ-014** — The control is examined for public report export, local private export, shared model export, downstream-tool export, adapter and plugin routes, preservation of safe evidence, unknown provenance and non-mutation of the source.
- **REQ-015** — A route, export format or transport that no decision has admitted is not in the catalogue, and the control invents no policy for it.
- **REQ-016** — A shareable report may include the user's own private project data only after the user's explicit release of that data for that one report, with the user's statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). The export context is `user_released_report`, admitted only on its own route and only with the release its wrapper states. The release is not kept for later reports. Private rule-pack and library values stay at safe metadata. Protected, quarantined and rejected content still blocks. Every other export context is unchanged.
- **REQ-017** — Wherever the classifier, the catalogue or route admission is implemented more than once, every implementation gives the same decision for the same input, and each route has one identity and one policy.
- **REQ-018** — A native save decides for itself. The evidence it is handed must name, by hash, the exact content it received; the desktop screens the disposition members of that content itself, at every depth, and refuses a blocked disposition and any disposition member whose value is a list or an object. A null, boolean or number disposition is not refused and is never read as public. Caller evidence alone admits nothing.
- **REQ-019** — An issue, crash or support bundle exposes project-derived content only as a catalogued route, and excludes private data by default (PRD §20.3).

### Acceptance criteria

- **AC-001** — The contract document and the schema state every export context, privacy class, action, finding class and reason code, the decision order and each rule for reading item metadata, and say which finding classes are local to this boundary. Every decision, finding and controlled export the implementations emit validates against the schema.
- **AC-002** — For an invented item of each privacy class and each redistribution and review status, with and without intent, in each of the six contexts, the decision is the one the decision order and the context's rule give. In particular: private values in a public, shared or downstream context are redacted; missing or unknown metadata is redacted there and kept with a warning in `local_private`; safe metadata with a public redistribution status is included.
- **AC-003** — A professional claim, a cloud or network reference, direct SQL access, a storage bypass, secret material, a contained payload, suspected protected content and a quarantined or rejected review status each block in every context, `local_private` with intent included. A concrete path never turns a block into a redaction.
- **AC-004** — On a `local_private` route, private values are present in the output only when the route's wrapper states the user's intent, and each carries a warning finding in the returned decisions and a warning in the export's record; without that intent the export is blocked. A payload member named `local_private_intent`, `explicit_local_private_intent` or `user_intent`, at any depth, is removed from the controlled copy and changes no decision.
- **AC-005** — In `user_released_report`, private project data is kept with its warning finding only with the wrapper's release; a private class other than project data is redacted; missing, unknown or unrecognized metadata blocks and is never warning-only. Every enclosing object is decided before any value beneath it, so beneath a labelled object no protected, quarantined, rejected or non-project private descendant, and no descendant carrying a blocking marker, is exposed; and an object with a stated but unrecognized label blocks whatever its descendants state. The context is refused on every route but its own, a release stated with any other context is refused, and no decision in any other context differs with or without a stated release.
- **AC-006** — After any export, redacted, blocked or admitted, the source model, the private libraries and the rule packs are equal, byte for byte in canonical form, to what they were before.
- **AC-007** — Every route returns its controlled export before exposure. When it is blocked there is no output file or directory, no report markup, preview or render call, no save or print action, no payload on standard output and no fallback path. A lossless route with any value redacted, omitted or blocked is withheld whole. On the desktop this holds at the native save and the native print, observed there and not only in the webview. This carries commitment DEL-12-02:1.
- **AC-008** — A route identifier outside the catalogue is refused by route admission. A `local_private`-only route is refused in any other context. Route admission reads only route metadata, and its evidence carries the route identifier, both contexts, the action, the reason code, the blocked flag and the intent.
- **AC-009** — A redacted public or shared export of a report that used a private rule pack and a private library still contains the model and report hashes, the solver and report versions, the input-manifest identifier, the warning summary, the pack's identifier, version, checksum and source note and the provenance summaries, and contains none of the pack's formulas or values and none of the library's values.
- **AC-010** — Every finding carries the fields DEL-00-06 requires of a diagnostic (code, class, severity, source, affected object, message, remediation, and provenance where applicable), with the affected path as its affected object and the action it records; it names the path and never the value; and states or implies no certification, approval or code compliance.
- **AC-011** — For every case of the shared invented corpus, each implementation gives the decision this scope's rules give. For every case of the input-reading sweeps, the implementations give one and the same decision, equal to the recorded decision for the case, and none decides a case less strictly than its recorded earlier decision. The catalogue of routes, and each route's contexts and policy, are equal wherever they are held.
- **AC-012** — A native save handed evidence that does not name the hash of the content it received refuses. Handed content with a blocked disposition, or with a disposition member that is a list or an object at any depth, it refuses before the save dialog, whatever the evidence says. A stored rule pack's own top-level `classification` object in the handoff save is the one admitted object: each of its text members is screened and any list or object inside it is refused.
- **AC-013** — Every sink through which the product exposes project-derived content is reached only through a catalogued route's wrapper: none builds its output from the uncontrolled source.
- **AC-014** — The public report templates and the public examples the product ships carry only invented or reviewed public-permissive content, and each passes the control in `public_report` or `public_example` with nothing blocked and nothing redacted.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *Explicit metadata only.* A value is decided by what it and its provenance state. Nothing is guessed from content, so a decision can be predicted and repeated.
- *Fixed context, wrapper-owned intent.* A route cannot choose its own policy and a payload cannot grant itself exposure.
- *Decide before exposure, on a copy.* The controlled export exists before any sink is touched, and the source is never the thing redacted.
- *Block is final.* The decision order puts every block ahead of any redaction or inclusion.
- *Withhold, do not corrupt.* A format that cannot carry a redaction is not written at all.
- *One contract, several implementations.* The classifier exists in the Python core, in the desktop's TypeScript and in a Rust mirror. One invented corpus and one set of sweeps hold them to the contract and to one another.
- *Defence in depth at the native boundary.* The webview's evidence is checked, never trusted, by the native side that writes the file.
- *A release is a context, not a class.* The user's release is stated by the route's wrapper. A privacy class carried in a payload could grant its own exposure; a context cannot.

**How each criterion is examined.**

- **VER-001** — AC-001, executable check with judgment by a named role (a reviewer who did not write the contract): validates emitted decisions, findings and controlled exports against the schema; the reviewer reads the contract document against this scope and reports each context, class, action, step or reading rule that is missing, added or weakened.
- **VER-002** — AC-002, AC-003, AC-004 and AC-005, executable check: classifies the shared invented corpus in every context with and without each intent and observes each decision and reason code against the decision this scope's rules give for the case; and for a `local_private` export made with intent, observes the warning for each kept value in the export's record.
- **VER-003** — AC-006 and AC-007, executable check: for each route, compares the canonical source before and after an admitted, a redacted and a blocked export, and observes the file system, the report markup, the render and save calls and standard output on a blocked export and on a lossless route with a redaction.
- **VER-004** — AC-008 and AC-012, executable check: offers route admission unknown identifiers, each route in each context and each intent setting; offers each native save evidence that names another hash, content with each blocked disposition and content with list and object dispositions at several depths, and observes the refusal and the absence of a save dialog.
- **VER-005** — AC-009 and AC-010, executable check: exports a report built on a private rule pack and a private library holding sentinel values in each public and shared context and observes the kept evidence, the absent sentinels and the fields and wording of every finding.
- **VER-006** — AC-011, executable check and comparison with a named reference: the check observes each implementation's decision on every corpus case against the decision this scope's rules give; the reference for the sweeps is the recorded decision per case and each implementation's recorded earlier decision, against which every implementation is compared for equality and for strictness; and the catalogues are compared member for member.
- **VER-007** — AC-013 and AC-014, judgment by a named role (a reviewer who did not write the routes), with executable checks: the reviewer traces every sink that exposes project-derived content to its route wrapper; a check observes that no sink is reachable without one and runs the shipped public templates and examples through the control.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing private leaves unnoticed.* Better is fewer inputs, routes and readings on which a private or protected value reaches a shared or public output; the target is none. Usefulness of a richer output never buys an inclusion that the metadata does not support.
- **AX-002** — *Stricter, never looser.* When two readings of an input differ, the stricter decision is taken. Better is a larger set of anomalous inputs decided by a stated rule. Making an existing case less strict is a change of the export policy, never a repair, and is decided as one.
- **AX-003** — *A redacted output is still useful.* Better is more safe evidence kept (hashes, versions, identifiers, provenance summaries) for the same redaction. This is never bought by keeping a value, a path or a private checksum.
- **AX-004** — *The user decides about the user's own data.* A local private output and a user release cost the user one explicit act each and no more. Neither act is ever inferred, remembered or read from content.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 OBJ-010 | REQ-001 REQ-004 REQ-011 | AC-001 | VER-001 | This scope's vocabulary and decision order, which the contract must state; a reviewer who did not write the contract compares them. The schema is the contract's own and is not independent grounds for it |
| OUT-002 | SOW-040 OBJ-010 | REQ-002 REQ-005 REQ-009 REQ-010 | AC-002 | VER-002 | The decision order and context rules of this scope say what each invented case must decide; expected decisions are written from the rules, not captured from the classifier |
| OUT-002 | SOW-040 OBJ-010 | REQ-002 REQ-013 | AC-003 | VER-002 | The decision order, and PRD §21.2 for the prohibited claims |
| OUT-003 | SOW-040 OBJ-010 | REQ-006 | AC-004 | VER-002 | Intent comes from the route's wrapper only; the scenario says what must be present and absent, in the output and in the export's record |
| OUT-002 | SOW-040 OBJ-010 | REQ-016 | AC-005 | VER-002 | The user release of the user's own project data for one report, and its decision rule in this scope |
| OUT-002 | SOW-040 OBJ-010 | REQ-007 | AC-006 | VER-003 | The criterion determines the result: the source after must equal the source before |
| OUT-003 | SOW-040 OBJ-010 | REQ-001 REQ-012 | AC-007 | VER-003 | The scenario says what must not exist on a block; the file system, markup and calls are observed from outside the control |
| OUT-003 | SOW-040 OBJ-010 | REQ-012 REQ-015 | AC-008 | VER-004 | This scope's definition of the catalogue and of route admission; DEL-12-01's guard gives the storage decision |
| OUT-002 | SOW-040 OBJ-010 | REQ-008 REQ-009 | AC-009 | VER-005 | The criterion determines the result: the sentinel values are known to the check and must be absent, and the evidence listed must be present |
| OUT-002 | SOW-040 OBJ-010 | REQ-011 REQ-013 | AC-010 | VER-005 | DEL-00-06's diagnostic contract fixes what a finding carries; PRD §21.2 lists the prohibited claims |
| OUT-002 | SOW-040 OBJ-010 | REQ-014 REQ-017 | AC-011 | VER-006 | For the corpus, the expected decision is written per case from this scope's rules. For the sweeps, the recorded decisions are the product's own earlier output: they guard only against change and against a case becoming less strict, and agreement between implementations is not grounds for correctness |
| OUT-004 | SOW-040 OBJ-010 | REQ-012 REQ-018 | AC-012 | VER-004 | This scope's rule for a native save says what must refuse; which dispositions are blocked is what DEL-03-07's scope commits to quarantine or reject (DEL-03-07/AC-003), written into the check from that scope and not taken from the import's code |
| OUT-003 | SOW-040 OBJ-010 | REQ-012 REQ-014 REQ-019 | AC-013 | VER-007 | The catalogue says which sinks exist; a reviewer who did not write the routes traces each sink |
| OUT-001 | SOW-040 OBJ-010 | REQ-003 | AC-014 | VER-007 | PRD §19.4 and FR-REP-005. Whether content is protected is DEL-08-05's lint and the owner's release scan, not this control |
