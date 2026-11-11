---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-05
package_id: PKG-12
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-040]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-05 Security threat model

## Purpose and Objective Traceability

This deliverable is the one document that says what the product must protect, where data crosses a trust boundary, what can go wrong at each crossing, and which control or open decision answers each threat. It is kept current as the product's architecture changes. Its result is that every way a user's private data, protected content or a secret can leave the user's control or be corrupted is written down, with the control that answers it or the decision still to be made, so that the deliverables that build a surface know what they must defend and a reviewer can see what is not defended.

It serves SOW-040 under OBJ-010 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-12, row DEL-12-05: document now, update as the architecture matures), PRD §20 (IP, data, security and privacy) and the PRD §26 risk of private data leaking in reports or bug bundles. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The threat model, `docs/security/threat_model.md`: the assets, the trust boundaries, the threat inventory with the control or open decision for each threat, the rules that are never bypassed, the open decisions and the review triggers.

## Deliverable Definition — Ontology

**Terms the document implements.**

- *Asset.* Something whose disclosure or corruption harms the user or the project. The asset classes are: public mechanics code; project model data and the project store; rule packs; material and component libraries; reports and result exports; diagnostics, logs and bug reports; plugin and adapter declarations; import, export and handoff data, including export packages and their manifests; secrets and credentials; and build and release artifacts. Each class has a sensitivity (private by default, provenance-gated, disclosure-sensitive, secret, or supply-chain-sensitive) and a boundary expectation.
- *Trust boundary.* A place where data passes between parties or levels of trust. The boundaries are: the local filesystem; the public repository; report, export and sharing; export packages for another tool, and the execution of an external tool the user owns; bug reports and diagnostics; telemetry; plugins, importers, exporters and the local FEA handoff; the rule evaluator; the embedded agent and its model-provider channel; and the build and release supply chain.
- *Threat.* One way an asset is harmed at a boundary. A threat has a stable identifier, the surface it concerns, a statement of the harm, an initial risk (High or Medium/High) and the controls it requires or the decisions that are still open.
- *Control.* A property of the product or a rule of the project that answers a threat. The control themes are: local-first by default; disclosure by channel; redaction and warning before export; provenance and redistribution status; sandboxed rule evaluation; no bypass by plugins and adapters; and evidence that binds hashes, checksums, versions and warnings to what was reviewed.
- *Disclosure by channel.* Each way out has its own rule. Public contribution and exported artifacts require the user's intent, documented rights, warnings and redaction. The model-provider channel of the embedded agent carries the user's own private data to the provider the user configured, with no further guard in the product (`docs/CONTRACT.md` OPS-K-PRIV-1); it leaves telemetry off by default and leaves the public-repository and third-party content prohibitions intact. Telemetry carries nothing by default.
- *Open decision.* A choice a threat's control depends on that no one has made. It is listed once, with what is undecided.
- *Review trigger.* A change that requires the model to be reviewed and updated.

**Boundary.** The document states threats and required controls. The controls are built and proved elsewhere: storage and path classes by DEL-12-01, redaction and export control by DEL-12-02, the telemetry guard by DEL-12-03, secret and private-library handling by DEL-12-04, the content boundary policy by DEL-01-02, the extension contract by DEL-02-04, the adapter gate by DEL-10-02, the rule sandbox by DEL-06-02, the protected-content linter and release scan by DEL-08-05, the embedded agent's isolation and credential custody by DEL-16-06, and build and release integrity by PKG-10. The document is a planning document. It is not a penetration test, a security certification, a legal opinion or an assurance that the product is secure.

**Not decided.** Whether telemetry may ever carry private data a user explicitly selects. REQ-003 states the PRD's rule; `docs/CONTRACT.md` OPS-K-PRIV-2 is stricter, and DEL-12-03 applies the stricter rule until the owner decides.

## Completion and Reliance Basis — Epistemology

The threat model is adequate when a reader who holds only the document can name, for any surface the product has, the assets at risk there, the threats, and for each threat either the control it requires or the decision that is open, and when nothing in it describes a control the product does not have.

### Requirements

- **REQ-001** — The model identifies private project, code, rule-pack, material, component, report, diagnostic, plugin and import, and supply-chain assets, and marks each protected or private-sensitive where it is (OBJ-010; PRD §20.3).
- **REQ-002** — The model keeps the local-first default and assumes no cloud operation unless one is approved as DEL-12-01/REQ-003 provides (PRD §20.4).
- **REQ-003** — The telemetry threats assume telemetry disabled by default and opt-in if ever implemented, with no private engineering or code data transmitted without explicit user action (PRD §20.5).
- **REQ-004** — The export, report, shared-model and bug-report threats include redaction and explicit warning controls for protected and private values (SOW-040; PRD §20.3).
- **REQ-005** — The model requires that public report templates and examples reproduce no protected standards content, proprietary formula, protected table or private project data (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-REPORT-2).
- **REQ-006** — The rule-pack threats include sandboxing, checksums, versioning, source notices, redistribution status and private or public marking (`docs/CONTRACT.md` OPS-K-RULE-2, OPS-K-RULE-3).
- **REQ-007** — The plugin, adapter, import, export and FEA-handoff threats keep the no-bypass boundary for unit checks, provenance, diagnostics, validation, sandboxing and report controls.
- **REQ-008** — The import and public-contribution threats include provenance, redistribution status, suspicion of protected content, quarantine and the path to human or legal review (`docs/CONTRACT.md` OPS-K-IP-2, OPS-K-IP-3).
- **REQ-009** — Where the model refers to diagnostics and result envelopes it uses the product's warning classes, the provenance, assumption and IP-boundary classes among them.
- **REQ-010** — The model keeps mechanics solved, user-rule checked and human-approved distinct, and neither states nor implies code compliance, professional approval or security certification (PRD §21.2). Any boundary language in the model is a statement of the claims registry, word for word, and the model composes no prohibition list of its own.
- **REQ-011** — The supply-chain threats include the provenance of dependencies and plugin packages, the integrity of build and release artifacts, open-source licence review and protected-content review gates.
- **REQ-012** — An unknown implementation choice is listed as an open decision and is never invented. The model describes a control as in place only where the product has it, and says so where a boundary has no interception.
- **REQ-013** — The model is reviewed and updated when any of these changes: the public API transport, the plugin permission model or the list of import and export formats; storage locations, store migration, the portable package, encryption or key management; the rule expression grammar, parser or sandbox; telemetry or its event schema; report or export fields, templates, copy paths, screenshots or shared-model behaviour; export packages, target profiles or the execution of an external tool; private-library storage, secret handling or a credential workflow; the embedded agent or its model-provider channel; build, packaging, signing, dependency review or CI policy; and any protected-content, private-data, secret or supply-chain incident.
- **REQ-014** — The model contains no protected standards content, no real private project data and no real secret, and states once, as a scope statement in plain prose, what it is not: a penetration test, a security certification, a legal opinion.

### Acceptance criteria

- **AC-001** — The document has an asset table with a row, a sensitivity and a boundary expectation for every asset class of the definition; a trust-boundary table with a row, a risk and a required posture for every boundary of the definition; and a threat inventory in which every boundary has at least one threat. Every threat has an identifier, a surface, a statement, an initial risk and its controls or open decisions.
- **AC-002** — The inventory includes at least these threats: private data stored in a public or example path or committed by accident; a report, shared model, export, screenshot or copied table disclosing private values or protected content; a bug report or diagnostic bundle carrying private files, paths or results; telemetry transmitting private data; a plugin, importer, exporter or handoff bypassing a control; imported data without provenance or copied from a protected source; a rule expression executing arbitrary code or leaking rule content; a public example built from protected or private values; a secret in a project file, report, log, example or test; a checksum or source notice that does not bind to the content reviewed; a compromised or provenance-weak dependency or release artifact; an overclaim of compliance, approval or certification; an export package that hides loss or claims target support beyond its evidence; an external tool run without the user's configuration; and private data or a credential leaving through the embedded agent beyond the provider the user configured.
- **AC-003** — For every threat, each control named is one the product has, one a named deliverable's scope commits to, or one a project policy states. Anything else, and each dependency on a choice no one has made, appears among the open decisions. No row describes as present a control the product lacks, and a boundary with no interception is described as having none.
- **AC-004** — The rows for reports, exports, shared models and bug reports name redaction, warning and the protected-content check. The rows for rule packs name the sandbox, checksums, versioning, source notices, redistribution status and private or public marking. The rows for imports and contributions name provenance, redistribution status, quarantine and human or legal review. The rows for the supply chain name dependency and package provenance, licence review, artifact integrity and the protected-content gate. Diagnostics are named by the product's warning classes.
- **AC-005** — The document states the local-first default with cloud operation absent; telemetry off by default and opt-in; disclosure by channel, the model-provider channel with its limits included; and the list of controls no plugin, adapter, importer, exporter, report generator or automation runner may bypass, direct access to the project store among them.
- **AC-006** — Every choice a control depends on that is undecided is listed once as an open decision, and no choice that the owner or an owning deliverable has made is still listed as open.
- **AC-007** — The document lists the review triggers of REQ-013, and it covers the product as it is: every family of export route, every kind of place the product stores private data, the telemetry state, the adapter gate, the embedded agent and the custody of its credential that exist in the product appear in the assets, boundaries or threats.
- **AC-008** — The document distinguishes mechanics solved, user-rule checked and human-approved; states what it is not; and contains no claim of compliance, approval or certification, no protected standards content, no real private data and no real secret. Any boundary statement it carries is a registered statement used word for word, and it carries no prohibition list of its own.

## Production and Verification Method — Praxeology

**Method choices that define the document.**

- *Assets, boundaries, threats, controls.* The model is organized by what crosses where. A threat without a boundary, or a boundary without a threat, is a gap to resolve.
- *State what is, not what is planned.* The model describes the product at the time of writing. A control the model requires and the product lacks is stated as required, never as present.
- *Stable identifiers.* A threat keeps its identifier for as long as it is in the model, so other documents can cite it.
- *Invented examples only.* No scenario in the model uses real project data, a real credential or protected content.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002, AC-004 and AC-005, judgment by a named role (a reviewer who did not write the document), with an executable check: the reviewer reads the document against the definitions and requirements of this scope and reports each asset class, boundary, threat, control or posture that is missing or weakened. The executable check observes only that the terms naming the required export workflows and controls are present in the document, which is a proxy for coverage and not a judgment of it.
- **VER-002** — AC-003, AC-006 and AC-007, judgment by a named role (a reviewer who did not write the document): reads the document against the product, the scopes of the deliverables it names and the decision register, and reports each control described as present that the product lacks, each control named that the product does not have, no named deliverable's scope commits to and no project policy states, each decided choice still listed as open, each undecided choice not listed, and each surface of the product the document does not cover.
- **VER-003** — AC-008, executable check with judgment by a named role (the same reviewer): searches the document for prohibited claim terms, credential-like strings and protected-content signatures, and the reviewer examines every hit and the statement of what the document is not, and compares each boundary statement in the document with the text the claims registry gives for it.

## Governing Values and Decisions — Axiology

- **AX-001** — *Cover the product that exists.* Better is fewer surfaces of the product through which private data, protected content or a secret can cross a boundary without a threat that names it; the target is none. Coverage is never claimed by listing a surface the product does not have as though it were defended.
- **AX-002** — *Honest about controls.* Better is a larger share of threats whose control is real or whose open decision is stated. A control is never described as present to make the model look complete, and an unresolved threat is never removed to make it look finished.
- **AX-003** — *Current.* Better is a shorter time between a review trigger and the model's update. Speed never buys a change that drops a threat's identifier or rewrites what an earlier threat said without stating the change.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 OBJ-010 | REQ-001 REQ-002 | AC-001 | VER-001 | This scope's definitions list the asset classes and boundaries, from PRD §20 and the decomposition; a reviewer who did not write the document compares it with them |
| OUT-001 | SOW-040 OBJ-010 | REQ-003 REQ-004 REQ-005 REQ-006 REQ-007 REQ-008 REQ-011 | AC-002 | VER-001 | PRD §20 and §26 and `docs/CONTRACT.md`, OPS-K-PRIV-1 for the model-provider channel included, say what must be protected; the list of threats is this scope's; the reviewer compares the inventory with it |
| OUT-001 | SOW-040 OBJ-010 | REQ-012 | AC-003 | VER-002 | The product, the scopes of the deliverables that build the controls and the project's policies, none of which the document's author wrote, say which controls exist or are committed |
| OUT-001 | SOW-040 OBJ-010 | REQ-004 REQ-006 REQ-008 REQ-009 REQ-011 | AC-004 | VER-001 | `docs/CONTRACT.md` OPS-K-IP, OPS-K-RULE and OPS-K-REPORT invariants name the controls; DEL-00-06 names the warning classes |
| OUT-001 | SOW-040 OBJ-010 | REQ-002 REQ-003 REQ-007 | AC-005 | VER-001 | PRD §20.4 and §20.5, the local storage profile DEL-00-04 states, and `docs/CONTRACT.md` OPS-K-PRIV-1 and `docs/IP_AND_DATA_BOUNDARY.md` §6.1 for the model-provider channel; the reviewer compares the document's statements with them |
| OUT-001 | SOW-040 OBJ-010 | REQ-012 | AC-006 | VER-002 | The decision register and the owning deliverables' scopes say what has been decided; the reviewer compares the open decisions with them |
| OUT-001 | SOW-040 OBJ-010 | REQ-013 | AC-007 | VER-002 | The product as built says which surfaces exist; a reviewer who did not write the document compares the two |
| OUT-001 | SOW-040 OBJ-010 | REQ-010 REQ-014 | AC-008 | VER-003 | PRD §21.2 lists the prohibited claims and the claims registry holds the boundary statements, each read from its source and not from the document; the search terms are known to the check independently of the document |
