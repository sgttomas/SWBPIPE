---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-03
package_id: PKG-12
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-037]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-03 Telemetry off-by-default design

## Purpose and Objective Traceability

This deliverable makes "telemetry is off" a property of the product and not a setting someone must remember. It states the telemetry policy, and it supplies the guard that resolves every configuration it cannot positively read as an approved opt-in to disabled, drops or rejects an event attempt before any payload exists, and never starts a transport. Its result is that a user's private engineering data cannot leave the machine by telemetry, and that the user can see in the desktop that telemetry is off and why.

It serves SOW-037 under OBJ-010 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-12, row DEL-12-03, and open issue OI-008), PRD §20.5 (telemetry), PRD §20.3 and FR-RULE-004 (private data and private rule packs excluded from telemetry) and `docs/CONTRACT.md` OPS-K-PRIV-2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The telemetry policy, `docs/security/telemetry_policy.md`: the default, the conditions any telemetry would have to meet, the content telemetry may never carry, and the surfaces that cannot enable it.
- **OUT-002** — The telemetry guard: the resolution of a telemetry configuration and the decision on an event attempt, both made from metadata before any payload is built, in the Python core and in the desktop.
- **OUT-003** — The desktop's telemetry panel, which shows the resolved state and the reason for it.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Telemetry.* Any collection of events or measurements about use of the product that is stored for sending, or sent, to anyone other than the user. A file the user saves, a report the user exports and the embedded agent's model-provider channel are not telemetry.
- *Configuration resolution.* The reading of whatever telemetry configuration exists into one of two states, enabled or disabled, with a reason. Disabled is the result for a configuration that is absent, unset, empty, unknown, unsupported, malformed or incomplete.
- *Opt-in.* A user's affirmative act whose only meaning is to turn telemetry on, made on a consent surface approved for that purpose. Installing the product, accepting general terms, opening a project, importing data, running a solve, exporting a report and loading a plugin are not opt-in.
- *Event allowlist.* The list of events that may be collected, approved by the owner for an exact product version. An entry names the event, its permitted fields and each field's sensitivity. An allowlist admits only low-sensitivity operational metadata.
- *Event attempt.* A description of an event that some part of the product would like to record: its name, version, source surface, field names and field classes. It carries no values.
- *Decision.* For one attempt: dropped because telemetry is disabled; rejected because the event, a field, a field class or the shape of the attempt is not permitted; or eligible. A decision is made before any payload is constructed, and no decision starts a transport.
- *Forbidden content.* Private project models and results; code-specific rule data; private rule packs; private material and component libraries; generated reports and exports; hashes that can fingerprint private work; local file paths; secrets and credentials; protected standards content; and any claim of certification, approval or code compliance.
- *Runtime surface.* An endpoint, vendor client, network transport, upload queue, upload job, background job or telemetry persistence.

**Boundary.**

- No telemetry design is approved: no endpoint, vendor, transport, retention rule, event schema, event allowlist, consent surface or stored setting is selected. Telemetry stays a no-op until the owner approves one (decomposition OI-008).
- The panel's request action records, for the life of the panel, that the user asked for telemetry to be reviewed. It is not opt-in, enables nothing and writes no configuration.
- DEL-12-02 owns exports, including the local file the panel can save. DEL-12-04 owns how private-library and credential references are kept out of any release. DEL-12-05 owns the threat model's account of telemetry. DEL-16-06 owns the model-provider channel, which `docs/CONTRACT.md` OPS-K-PRIV-1 governs and this deliverable does not.
- The guard and the panel are not a telemetry implementation and authorize none.

**Not decided.**

- Whether a user may ever select private data for a telemetry payload. PRD §20.5 bars it "without explicit user action" and the threat model allows a user-chosen payload, while `docs/CONTRACT.md` OPS-K-PRIV-2 bars private engineering and code data from telemetry outright. Until the owner decides, the stricter rule governs: REQ-004 has no exception.
- Where an opt-in setting would be stored and what its schema is. REQ-005 fixes only that it is local and false by default.

## Completion and Reliance Basis — Epistemology

The deliverable is adequate when no configuration, action or module of the product can cause telemetry to be collected or sent, and this can be shown without trusting any setting.

### Requirements

- **REQ-001** — Telemetry is disabled by default. An absent, unset, empty, unknown, unsupported, malformed or incomplete telemetry setting resolves to disabled (SOW-037; PRD §20.5; `docs/CONTRACT.md` OPS-K-PRIV-2).
- **REQ-002** — No network transport, background upload job, queue or telemetry persistence is initialized unless the user has explicitly opted in.
- **REQ-003** — No event is collected until the owner has approved the event allowlist for the product version (decomposition OI-008).
- **REQ-004** — Telemetry never collects or transmits forbidden content (PRD §20.3, §20.5; FR-RULE-004; `docs/CONTRACT.md` OPS-K-PRIV-2, OPS-K-IP-1).
- **REQ-005** — Opt-in is a local configuration value whose default is false.
- **REQ-006** — Turning telemetry on requires an affirmative user action distinct from accepting general terms, installing the software, opening a project or running a solve.
- **REQ-007** — Telemetry settings and diagnostics keep mechanics solved, user-rule checked and human-approved distinct, and telemetry creates no professional approval or compliance claim (PRD §21.2).
- **REQ-008** — A telemetry diagnostic uses the product's diagnostic record and contains no private payload content.
- **REQ-009** — No plugin, adapter, import or export path, report or private-library mechanism bypasses opt-in or the content rules.
- **REQ-010** — While telemetry is unimplemented the product's behaviour is a no-op, and no outbound telemetry behaviour is reachable by default.

### Acceptance criteria

- **AC-001** — The policy states the default; the conditions that must all hold before any runtime surface is initialized (opt-in on an approved surface, an approved allowlist for the version, filtering before construction, the content checks available, and no bypass); each class of forbidden content; the acts that are not opt-in; and the surfaces that cannot enable telemetry. It names no endpoint, vendor, transport or event.
- **AC-002** — The guard resolves an absent, unset, empty, unknown, unsupported, malformed and incomplete configuration each to disabled, with a diagnostic that names the fault for the malformed and incomplete ones. It resolves to enabled only a configuration whose opt-in was made on a consent surface the owner has approved and whose allowlist is one the owner has approved for the product version. A configuration's own statement that a surface or an allowlist is approved is not approval. While the owner has approved none, no configuration of the product resolves to enabled. The desktop resolves to disabled whatever is requested of it.
- **AC-003** — With telemetry disabled every event attempt is dropped. With telemetry enabled, an attempt is eligible only when its event and every one of its fields are on the allowlist and every field is of a low-sensitivity operational class; an unknown event, an unlisted field or a field of unclear class is rejected. In every case no payload is constructed.
- **AC-004** — An attempt that names any class of forbidden content, by field class, field name or event name, is rejected under every configuration, enabled with a matching allowlist entry included. An attempt that carries a value, a payload member or a runtime-surface member is rejected for its shape.
- **AC-005** — In the product, under every configuration and after every user action, no runtime surface is initialized: the desktop opens no telemetry connection, starts no upload job and writes no telemetry store, and the guard's decisions and the desktop's reported state say so.
- **AC-006** — Installing, accepting terms, opening a project, importing, solving, exporting and loading a plugin leave the resolved state disabled. After the panel's request action the request is recorded, and telemetry is still disabled, opt-in is false, no configuration is written, no payload is constructed and no runtime surface is initialized. This carries commitment DEL-12-03:2.
- **AC-007** — Every telemetry diagnostic carries a code, class, severity, source, affected object, message and remediation; it names the rejected event or field and never a value; and no diagnostic, setting or panel text states or implies approval, certification or code compliance, or merges mechanics solved, user-rule checked and human-approved.
- **AC-008** — No module of the product other than the guard and the panel names a telemetry event, endpoint, vendor or transport, or records an event; and no plugin manifest, adapter declaration, command-line option, imported file, report template or rule pack changes the resolved state. This carries commitment DEL-12-03:1.
- **AC-009** — The panel shows the resolved state, the reason code and whether any payload or runtime surface exists, each equal to what the guard returns. The file it can save is a local export of that state through a route DEL-12-02 controls, and saving it sends nothing.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *Decide on metadata, before construction.* The guard sees names and classes, never values. A payload that is never built cannot leak.
- *Allowlist, not redaction.* Permission comes from an approved list of events and fields. Filtering a broad payload after the fact is at most a second check and is never the permission.
- *Fail closed.* Anything the guard cannot positively read as approved resolves to disabled or rejected. Consent is never inferred from ambiguity, legacy configuration, an environment default or an error.
- *No transport in the product.* Until a design is approved the product contains no telemetry client to switch on.

**How each criterion is examined.**

- **VER-001** — AC-001, judgment by a named role (a reviewer who did not write the policy): reads the policy against PRD §20.3 and §20.5, `docs/CONTRACT.md` OPS-K-PRIV-2 and this scope; reports each default, condition, forbidden class or non-consent act that is missing, added or weakened, and any endpoint, vendor, transport or event it names.
- **VER-002** — AC-002, AC-003 and AC-004, executable check: submits each kind of configuration and invented event attempts of every permitted and forbidden kind, under each configuration, to the guard in the Python core and in the desktop, and observes the resolution, the decision, its reason code and that no payload was constructed. The enabled resolution is examined with an invented approval that the check supplies to the guard, and a configuration that only asserts its own approval is observed to resolve to disabled.
- **VER-003** — AC-005 and AC-006, executable check: runs the desktop through each named user action and the panel's request action with network connections, background jobs and written files observed, and reads the resolved state after each.
- **VER-004** — AC-007 and AC-009, executable check: observes the fields and text of every diagnostic the guard returns for attempts seeded with sentinel values, compares the panel's displayed state with the guard's, and observes the save of the panel's file with network connections observed.
- **VER-005** — AC-008, judgment by a named role (a reviewer who did not write the guard), with an executable check: the check searches the product's modules for telemetry events, endpoints, vendors and transports outside the guard and the panel, and offers a plugin manifest, an adapter declaration, a command-line option, an imported file, a report template and a rule pack that each ask for telemetry; the reviewer examines every hit.

## Governing Values and Decisions — Axiology

- **AX-001** — *Off means off.* Better is fewer code paths that could initialize a runtime surface; while no design is approved the target is none. Insight into how the product is used never buys a default-on path or a path that starts before opt-in.
- **AX-002** — *Permission by list.* Better is a smaller set of events and fields that could ever be eligible, each of plainly low sensitivity. A wider allowlist is never traded for the promise of redaction afterwards.
- **AX-003** — *Consent is never inferred.* Better is a larger share of ambiguous configurations and actions that resolve to disabled with a stated reason. Convenience at first run never buys an opt-in the user did not give as its own act.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-037 OBJ-010 | REQ-001 REQ-003 REQ-004 REQ-006 | AC-001 | VER-001 | PRD §20.3 and §20.5, `docs/CONTRACT.md` OPS-K-PRIV-2 and decomposition OI-008; a reviewer who did not write the policy compares it with them |
| OUT-002 | SOW-037 OBJ-010 | REQ-001 REQ-005 | AC-002 | VER-002 | This scope's definitions of configuration resolution, opt-in and the event allowlist say what each configuration must resolve to and whose approval counts; the expected states are written from them, not taken from the guard |
| OUT-002 | SOW-037 OBJ-010 | REQ-003 | AC-003 | VER-002 | This scope's definitions of the allowlist and the decision say what each invented attempt must receive |
| OUT-002 | SOW-037 OBJ-010 | REQ-004 | AC-004 | VER-002 | The forbidden content of PRD §20.3 and §20.5 and `docs/CONTRACT.md` OPS-K-PRIV-2 |
| OUT-002 | SOW-037 OBJ-010 | REQ-002 REQ-010 | AC-005 | VER-003 | The scenario says nothing may start; connections, jobs and files are observed from outside the product |
| OUT-003 | SOW-037 OBJ-010 | REQ-005 REQ-006 | AC-006 | VER-003 | PRD §20.5 requires opt-in and this scope lists the acts that are not opt-in; the state after each act is observed |
| OUT-002 | SOW-037 OBJ-010 | REQ-007 REQ-008 | AC-007 | VER-004 | DEL-00-06's diagnostic contract fixes the fields; PRD §21.2 lists the prohibited claims; the sentinel values are known to the check and must be absent |
| OUT-002 | SOW-037 OBJ-010 | REQ-009 REQ-010 | AC-008 | VER-005 | The scenario says no other module may emit and no declaration may enable; a reviewer who did not write the guard examines the search results |
| OUT-003 | SOW-037 OBJ-010 | REQ-001 REQ-008 | AC-009 | VER-004 | The criterion determines the result: the panel must equal the guard, and the save must open no connection |
