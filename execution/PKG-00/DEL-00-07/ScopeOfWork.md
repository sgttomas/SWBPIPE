---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-07
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-062]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-07 API boundary and adapter contract map

## Purpose and Objective Traceability

This deliverable is the map of the seams between SWBPIPE's layers and the rule for anything that crosses one from outside. It says which seams are internal contracts that may change freely and which may be offered to plugins, adapters and automation. It says what an adapter must check before data reaches the core, and what must be preserved at each handoff: to storage, to reports, to private libraries, to local FEA and to external automation. The result it contributes is that SWBPIPE can exchange models and results with other tools, and can be driven by automation, without any of them getting around unit checks, provenance, the rule sandbox, result envelopes or report controls.

It serves SOW-062 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-07, and architecture-basis row AB-00-07 in §8.1), with `docs/PRD.md` §5.3, §5.4, §5.8, §20.3 and §20.4 and `docs/SPEC.md` §1 and §4.5. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The boundary map, a document under `docs/architecture/`: one row for each pair of layers that call each other or pass data, naming the seam and marking it internal or public.
- **OUT-002** — The adapter and plugin rule, and the obligations at each of the five handoffs.

## Deliverable Definition — Ontology

**Terms the map implements.**

- *Seam.* A place where one layer calls another or data passes between them. The layers are those DEL-00-02 defines.
- *Internal seam.* A contract between first-party modules, which may change without notice to anyone outside the repository.
- *Public seam.* A contract that plugins, adapters or external automation may use. It is schema-first and takes the form of commands, queries, jobs and result envelopes.
- *Adapter.* A module that translates data between an external form and the domain contracts: an importer, an exporter, a handoff writer. *Plugin*: a third-party extension that asks for an entry point.
- *Denied by default.* A plugin's or adapter's declaration grants nothing. A capability exists only by an explicit grant, and a grant never lifts a control.
- *Boundary checks.* What an adapter establishes before data enters core workflows: units and dimensions, provenance, redistribution status, diagnostics for whatever is missing or wrong, and the public or private class of the data.
- *Controls that are never bypassed.* Domain and schema validation, rule-pack sandboxing, result envelopes, and report boundary controls.
- *Named format.* An import or export format that the decomposition, the release target or an owner ruling names. Those named are the project package (a multi-member archive, `.opsproj`), the native JSON export package, CAEPIPE MBF, the CAEPIPE result CSV, stress-neutral CSV and JSON, the conservative PCF subset, GLB/glTF review geometry, and CSV export of result tables.

**The five handoffs.**

| Handoff | What is preserved | What is never done |
|---|---|---|
| Storage | Schema version, canonical payload and hash metadata, provenance, diagnostics, units | Direct SQL or domain mutation; treating a sidecar as authoritative |
| Reports | References to governed results and reports, affected objects, assumptions, warnings, privacy class and professional-boundary notices | Creating an approval, compliance or reliance state |
| Private libraries | Local and private by default; provenance and redistribution metadata; explicit grants | Filesystem, network or publication access without a grant |
| Local FEA | Stable identifiers, units and dimensions, coordinate basis, source hashes, diagnostics and a loss report | A claim of compatibility with a target solver or of validation |
| External automation | Schema-first commands, queries and job results; correlation and idempotency; diagnostics; explicit runtime grants | Direct mutation of domain state; process, network or private-data access without a grant |

Automation reaches the product from outside through a local command-line runner: a single foreground process that takes a JSON request and returns JSON, with no daemon, network listener, telemetry or hidden write.

**Boundary.** This deliverable defines the map and the obligations; it implements no API, plugin, importer, exporter or format. Its own file is the map. AC-002 and AC-003 are observed on the code of the deliverables below, and a shortfall is repaired there.

- DEL-10-01 owns the public API and plugin boundary contract; DEL-02-04 owns what an extension may touch in the domain.
- DEL-10-02 owns the adapter framework; the deliverables of PKG-17 own each export format; DEL-10-03 owns the local FEA handoff contract; the deliverables of PKG-15 own the handoff package.
- DEL-10-05 owns the runner.
- DEL-00-02 owns where adapter modules sit in the module graph. This scope owns what is checked on the data that crosses.
- DEL-00-03 owns the request kinds, DEL-00-06 the envelope, DEL-00-04 the storage profile.

**Not decided.** The transport protocol and endpoint syntax of a public API. The plugin runtime: how a plugin is loaded, signed and isolated, and how a permission grant persists.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The map says which boundaries are internal implementation contracts and which may become public extension APIs (decomposition AB-00-07).
- **REQ-002** — Adapters validate units, provenance, redistribution status, diagnostics, and the private or public class of data before that data enters core workflows.
- **REQ-003** — Plugins and adapters do not bypass domain validation, rule-pack sandboxing, result envelopes or report boundary controls (`docs/SPEC.md` §4.5).
- **REQ-004** — An import or export format is part of the map only where the decomposition or an owner ruling names it. A choice no one has made is stated as undecided.
- **REQ-005** — The obligations at the storage, report, private-library, local FEA and external-automation handoffs are defined, each as a check its owning boundary applies. The map selects no concrete format, target compatibility, runtime grant or validation outcome, and none is to be inferred from it; meeting the obligations is the work of the deliverables that own each boundary.

### Acceptance criteria

- **AC-001** — The map lists every seam between the layers and marks each internal or public. Every operation the public API contract exposes falls under a seam the map marks public, and nothing the map marks internal is exposed there.
- **AC-002** — Data offered to any adapter with a missing or incompatible unit, missing provenance, unknown redistribution status or private content bound for a public destination is stopped at the adapter with a diagnostic that names the item, and none of it enters the model.
- **AC-003** — No plugin or adapter can cause a model change, a rule evaluation, a result or a report that did not pass domain validation, the rule sandbox, a result envelope and the report controls. A declaration that requests a bypass, or a capability that has not been granted, is refused.
- **AC-004** — The map and the public API contract name as an import or export format only one the decomposition, the release target or an owner ruling names, and no adapter is declared for any other. Each undecided format choice appears as one statement of what is undecided.
- **AC-005** — For each of the five handoffs the map names the owning boundary and its deliverable, and states what that boundary preserves and what it never does, as in the table above. No handoff obligation selects a concrete format, target compatibility, runtime grant or validation outcome.

## Production and Verification Method — Praxeology

**Method choices that define the map.**

- A public seam is defined by its schema before any transport is chosen.
- An adapter translates. It decides no validation policy, and the same controls apply to its path as to first-party code.
- A format enters by decision and never by an adapter being written for it.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-004, executable check with judgment by a reviewer who did not write the map: observes the public API contract's operations and format names against the map, and the reviewer examines that the map covers every seam between the layers.
- **VER-002** — AC-002, executable check: offers each adapter inputs with a missing unit, an incompatible dimension, missing provenance, unknown redistribution status and private content marked for public export, and observes the named diagnostic and an unchanged model.
- **VER-003** — AC-003, executable check: submits plugin and adapter declarations that request each bypass and each ungranted capability and observes the refusal.
- **VER-004** — AC-005, judgment by a reviewer who did not write the map: examines each handoff's row against the table above and the owning deliverable's scope, and that no row selects a format, compatibility, grant or validation outcome.

## Governing Values and Decisions — Axiology

- **AX-001** — *Denied by default.* Better is fewer capabilities a plugin or adapter can reach without an explicit grant; the target is none. Ease of integration never buys a bypass of a control.
- **AX-002** — *Small public surface.* Better is fewer public seams, each defined by a schema. A seam is not made public to serve a single integration.
- **AX-003** — *Loss is reported.* Better is a larger share of what the local FEA target cannot represent appearing in its loss report. A handoff never claims compatibility with a target solver or validation.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-062 OBJ-013 | REQ-001 | AC-001 | VER-001 | Decomposition AB-00-07 and SOW-062; the comparison is the public API contract against the map, and a reviewer who did not write the map |
| OUT-002 | SOW-062 OBJ-013 | REQ-002 | AC-002 | VER-002 | Decomposition AB-00-07, `docs/PRD.md` §5.5 and §5.6; each scenario states what must be refused |
| OUT-002 | SOW-062 OBJ-013 | REQ-003 | AC-003 | VER-003 | Decomposition AB-00-07 and `docs/SPEC.md` §4.5; each scenario states what must be refused |
| OUT-001 | SOW-062 OBJ-013 | REQ-004 | AC-004 | VER-001 | The formats the decomposition names for PKG-17, exit criteria 5 and 6 of the release target, and the project package of the decision log; the comparison is each format the map and the public API contract name against them |
| OUT-002 | SOW-062 OBJ-013 | REQ-005 | AC-005 | VER-004 | The five-handoff table; the comparison is each handoff's row in the map against it and against the owning deliverable's scope. Whether a handoff holds in the product is established by the owning deliverable's own criteria, not by this row |
