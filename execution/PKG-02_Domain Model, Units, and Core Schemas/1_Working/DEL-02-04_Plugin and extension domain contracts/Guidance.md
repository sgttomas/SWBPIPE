---
doc_id: DEL-02-04-GUIDANCE
doc_kind: deliverable.guidance
status: draft
created: 2026-04-30
deliverable_id: DEL-02-04
package_id: PKG-02
---

# Guidance: Plugin and Extension Domain Contracts

## Purpose

DEL-02-04 exists so OpenPipeStress can be extensible without turning plugins or adapters into an escape route around the domain model, unit system, provenance model, diagnostics, report controls, or public/private data boundary. This is the PKG-02 domain/API contract for that boundary, not a plugin-loader implementation. SourcePath: `_CONTEXT.md`; SectionRef: Description. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: SOW-038.

## Principles

1. Deny-bypass is the governing principle. Plugins/adapters may extend behavior only through governed interfaces that preserve validation, diagnostics, envelopes, and report controls. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02 and AB-00-07.
2. Schema-first boundaries should be preferred for public manifests and interchange surfaces because SCA-001 selected JSON Schema 2020-12 and schema-first command/query/job/result envelopes. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.
3. Unit and provenance checks are not optional plugin features. They are part of the core data contract and must be preserved across imports, exports, reports, and rule-facing hooks. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1 and OPS-K-DATA-3.
4. Private or protected data must remain under user control. Public artifacts can provide schemas, empty templates, invented examples, and import mechanisms, but must not embed protected standards or proprietary data. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 2. Public repository may contain and 3. Public repository must not contain.
5. Diagnostics should be machine-readable as well as user-visible. Plugin/adapter failures need enough source, affected-object, remediation, and provenance detail to support audit and review. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06.
6. Software may show computed states and user-rule outcomes, but it must not claim professional approval or code compliance. SourcePath: `docs/TYPES.md`; SectionRef: 4. Analysis-status vocabulary. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1.

## Considerations

- Keep DEL-02-04 at the domain/API level. Public API transport, concrete import/export formats, concrete plugin loader behavior, and packaging/distribution are later implementation decisions unless separately authorized. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and DEC-012.
- Treat plugin capability grants as explicit and minimal. ASSUMPTION: a future permission model will need to distinguish read-only project access, governed project mutation, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration. Exact permission names and enforcement technology are TBD.
- Treat candidate extension point families as rationale-backed assumptions, not an approved registry. Import/export adapter candidates trace to the public API/import-export scope; report/output extension candidates trace to report-control obligations; validation-hook candidates trace to layered testing and protected-content/provenance gates; rule-pack integration hook candidates trace to sandboxed, unit-aware rule evaluation. Exact registry names, registry owner, and approval record are TBD.
- Prefer private-by-default behavior for user rule packs, component libraries, material data, project files, and calculation results. SourcePath: `docs/PRD.md`; SectionRef: 18.1 Local-First Design and 18.2 Telemetry.
- When a plugin imports external values, the contract should require provenance and redistribution metadata before those values can become public or reusable. Missing provenance should produce `PROVENANCE_WARNING` or a blocking result when required for reliance. SourcePath: `docs/SPEC.md`; SectionRef: 7. GUI requirements warning classes; SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields.
- If a plugin touches rule-pack evaluation, it should not run arbitrary executable code. Keep that work declarative, sandboxed, deterministic, unit-aware, and aligned with the rule-pack engine scope. SourcePath: `docs/PRD.md`; SectionRef: 12.3 Rule-Pack Evaluator.
- If a plugin hashes a JSON manifest or JSON data payload for reproducibility, use the accepted canonical JSON/JCS-compatible hash basis. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Extensibility vs. governance | Prefer stricter validation and explicit diagnostics over broad plugin authority. SOW-038 requires extensibility without bypass. |
| Schema-first contracts vs. idiomatic in-process APIs | Keep the public/domain contract schema-first where possible; implementation bindings can be generated or wrapped later. Exact code-generation tooling is TBD. |
| Sandboxing strength vs. plugin ergonomics | Start from least privilege and grant capabilities only when evidence, user intent, and review controls are clear. Exact sandbox mechanism is TBD. |
| Import/export flexibility vs. data boundary protection | Support adapters, but require provenance, redistribution status, unit checks, and protected-content gates before data leaves private control. |
| Report customization vs. protected content risk | Allow private report templates under user responsibility, but public templates and exports must not reproduce protected standards or proprietary formulas. |
| Plugin diagnostics volume vs. auditability | Favor structured diagnostics with source and remediation over terse failures; diagnostics are part of reproducibility and review. |

## Open Decision Table (for human ruling)

| Decision ID | Decision needed | Current bounded position | Source basis | Human ruling |
|---|---|---|---|---|
| OD-02-04-001 | Public API transport for plugin/adapter interfaces | TBD; keep schema-first envelope baseline only | `docs/_Decomposition/SOFTWARE_DECOMP.md` 8.2 Resolved architecture baseline; DEC-012 | TBD |
| OD-02-04-002 | Approved extension-point registry names and owner | TBD; candidate families remain ASSUMPTION | `docs/_Registers/ScopeLedger.csv` row SOW-038; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 | TBD |
| OD-02-04-003 | Permission taxonomy, sandbox mechanism, and approval path | TBD; deny-by-default and no-bypass controls remain governing | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 and DEC-012; `docs/CONTRACT.md` OPS-K-RULE-2 | TBD |
| OD-02-04-004 | Supported import/export formats | TBD; do not imply concrete format support | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-004 and DEC-012 | TBD |
| OD-02-04-005 | Whether plugins can ever receive telemetry-facing or private engineering data | TBD; default is no telemetry/private-data exposure | `docs/CONTRACT.md` OPS-K-PRIV-1 and OPS-K-PRIV-2; `docs/PRD.md` 18.2 Telemetry | TBD |

## Examples

The examples below are invented contract scenarios only. They do not define concrete schema syntax or provide engineering values.

| Scenario | Expected contract behavior |
|---|---|
| Invented CSV import adapter reads user-supplied component metadata from a private path. | Adapter declares private data access, maps fields to canonical schema, requires units for dimensional values, records source/provenance, and emits diagnostics for missing source/license fields. |
| Invented result exporter writes a machine-readable result package. | Exporter receives data through a governed query/result envelope, preserves units, includes warnings/assumptions, and does not claim code compliance or professional approval. |
| Invented rule-pack helper provides a custom interpolation table using user-owned values. | Helper remains sandboxed and deterministic, records redistribution status, rejects missing units, and does not access filesystem or network except through a granted, reviewed capability. |
| Invented report extension tries to include copied standards text. | Protected-content gate blocks the public artifact, classifies the issue for quarantine/human review, and emits an `IP_BOUNDARY_WARNING`. |

## Conflict Table (for human ruling)

No technical source conflict was identified for plugin-contract content during Pass 3. One metadata traceability conflict remains outside this run's write scope.

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| CONF-02-04-001 | Decomposition revision wording differs between current context and local reference index. | `_CONTEXT.md` - Decomposition Reference identifies accepted revision 0.4/current_basis. | `_REFERENCES.md` - Decomposition and Registers describes the decomposition as accepted v0.2. | Datasheet References; future metadata/reference cleanup. | Treat `_CONTEXT.md` and current `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 as the active basis for this run; route `_REFERENCES.md` cleanup to a later authorized metadata/reference pass. | TBD |
