---
doc_id: DEL-02-04-DATASHEET
doc_kind: deliverable.datasheet
status: draft
created: 2026-04-30
deliverable_id: DEL-02-04
package_id: PKG-02
---

# Datasheet: Plugin and Extension Domain Contracts

## Identification

| Field | Value | Evidence |
|---|---|---|
| Deliverable ID | DEL-02-04 | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04 |
| Name | Plugin and extension domain contracts | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04 |
| Package | PKG-02 - Domain Model, Units, and Core Schemas | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
| Type | API_CONTRACT | SourcePath: `_CONTEXT.md`; SectionRef: Type |
| Scope item | SOW-038 | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 |
| Objective | OBJ-009 | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: Objective-to-deliverable mapping, OBJ-009 |
| Anticipated artifacts | plugin interface spec; sandbox/permission model notes | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts |
| Decomposition basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 | SourcePath: `_CONTEXT.md`; SectionRef: Decomposition Reference |
| Discipline/domain | API and domain-contract governance for plugins/adapters | ASSUMPTION from Type=`API_CONTRACT`, package scope, and SOW-038 |
| Responsible party | TBD | Not stated in accessible sources |
| Governance/ruling owner | TBD | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AGENT-4. No project authority is assigned in the accessible sources for plugin-contract rulings. |

## Attributes

| Attribute | Value | Evidence |
|---|---|---|
| Primary contract subject | Extension points and governance constraints for plugins/adapters | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| Package boundary | Domain entities, unit system, persistence/serialization contracts, and extensibility boundaries; excludes numerical solving and GUI views | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
| Core governance intent | Extensibility must not bypass governance, unit safety, or data-boundary constraints | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 |
| Architecture basis IDs | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
| Schema/API baseline | JSON Schema 2020-12 for public schemas/interchange; schema-first command/query/job/result envelopes | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
| Hash basis when JSON payloads are hashed | Canonical JSON with JCS-compatible canonicalization | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
| Mandatory boundary checks | Units, provenance, redistribution status, diagnostics, public/private data boundary, report controls | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-07 |
| Diagnostic payload basis | Diagnostic/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-06 |
| External API transport | TBD | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
| Concrete import/export formats | TBD | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 |
| Detailed permission taxonomy and sandbox implementation | TBD | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 notes |

## Conditions

- Public artifacts must not include protected standards text, copied tables, protected examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1.
- Suspected protected content must be quarantined and escalated rather than paraphrased into public data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-3.
- Code-specific values and proprietary data are user-supplied or lawfully imported private data, not bundled public defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-1.
- All plugin or adapter ingress, egress, and rule-related values must remain unit-aware and dimensionally checked where numerical or dimensional data is involved. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1.
- Plugin and adapter outputs must not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1.
- Any rule-pack or expression-facing extension must remain sandboxed and unable to execute arbitrary code. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2.
- Telemetry, if ever exposed to plugins, is off by default and must not transmit private engineering/code data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-2.

## Construction

The DEL-02-04 contract kit is constructed from the following conservative parts:

| Part | Status | Notes |
|---|---|---|
| Plugin interface spec | Draft target | Must define extension point identity, lifecycle status, schema version, capability declarations, diagnostic envelope use, and validation obligations. Exact schema file layout is TBD. |
| Sandbox/permission model notes | Draft target | Must express deny-bypass constraints for units, provenance, data boundary, diagnostics, report controls, and rule sandboxing. Exact runtime mechanism is TBD. |
| Extension point registry | ASSUMPTION / TBD approval record | Candidate families are import adapter, export adapter, report/output extension, validation hook, and rule-pack integration hook. Approved registry names, registry ownership, and a decision/open-issue pointer are TBD. Concrete public API transport and format support remain TBD. |
| Capability/permission declarations | ASSUMPTION / TBD approval record | Candidate classes for later review are governed project mutation, read-only query access, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration. Exact permission names, grant records, enforcement implementation, and approval path remain TBD. |
| Human-ruling log pointer | TBD | Needed before exact extension registry, permission taxonomy, sandbox mechanism, transport binding, or telemetry/private-data exposure can be treated as approved. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-012 and OI-004. |
| Provenance and redistribution metadata | Required concept | Public data records require source, source location, license, contributor certification, redistribution status, and review status. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. |
| Verification hooks | Required concept | Future implementation should support schema validation, unit checks, protected-content/provenance gates, and adapter/plugin tests. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-08. |

## References

- `_CONTEXT.md`, revision 0.4 accepted basis for DEL-02-04.
- `_REFERENCES.md`, local reference index for DEL-02-04.
- `_DEPENDENCIES.md`, human-owned dependency declarations for DEL-02-04.
- `docs/_Registers/Deliverables.csv`, row DEL-02-04.
- `docs/_Registers/ScopeLedger.csv`, row SOW-038.
- `docs/_Registers/ContextBudgetQA.csv`, row DEL-02-04.
- `docs/_Decomposition/SOFTWARE_DECOMP.md`, revision 0.4, especially PKG-02, DEL-02-04, OBJ-009, SOW-038, and SCA-001 architecture basis rows.
- Traceability note: `_CONTEXT.md` identifies `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 as the accepted basis, while `_REFERENCES.md` still describes the decomposition reference as accepted v0.2. `_REFERENCES.md` is outside this run's write scope; see `Guidance.md` Conflict Table.
- `docs/CONTRACT.md`, invariant catalog.
- `docs/TYPES.md`, deliverable types, status vocabulary, epistemic labels, and data provenance labels.
- `docs/SPEC.md`, sections 1, 6, 7, 8, 10, and 11.
- `docs/IP_AND_DATA_BOUNDARY.md`, public/private data policy.
- `docs/DIRECTIVE.md`, product boundaries and stop rules.
- `docs/PRD.md`, sections 6.2, 6.3, 6.6, 12.3, 12.4, 15.2, 15.3, 18, and 19.
