# Datasheet: DEL-08-01 Calculation report generator

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-08-01 |
| Deliverable name | Calculation report generator |
| Package ID | PKG-08 |
| Package name | Reporting, Audit, and Reproducibility |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-024 |
| Objective | OBJ-007 |
| Context envelope | L |
| Current setup status | SEMANTIC_READY setup artifacts prepared |

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Production family | Calculation reports for OpenPipeStress analysis outputs. |
| Required report content | Inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations. |
| Anticipated downstream artifacts | Report renderer, report template, tests. |
| Setup-session implementation boundary | Renderer source, templates outside this deliverable, tests, schemas, and repo-level artifacts are out of write scope. |
| Architecture baseline | Rust core/application services, schema-first command/query/job/result envelopes, JSON Schema 2020-12 contracts, canonical JSON/JCS-compatible hash basis where JSON payload hashes are used. |
| Report authority boundary | Reports are decision-support artifacts and must not claim certification, sealing, approval, authentication, or automatic code compliance. |
| Protected-content boundary | Public templates/examples must not reproduce protected standards text, protected standards tables, protected figures, proprietary formulas, or protected code-derived content. |

## Conditions

| Condition | Handling requirement |
|---|---|
| Missing solve-required data | Preserve as explicit diagnostics or report findings; do not hide with silent defaults. |
| Missing rule-check-required data | Preserve as explicit rule-check-blocking diagnostics or findings; do not imply a completed check. |
| User-supplied code data | Record as user-supplied or private/provenance-tagged data; do not convert it into public defaults. |
| Rule-pack references | Report only identity, version, checksum, source/provenance notices, redistribution status, and user-provided report notices; do not reproduce protected rule text. |
| Professional reliance | Include a notice that competent human review is required before project reliance. |
| Unit-bearing values | Preserve units and dimensional context in generated report sections. |
| Private project data | Keep redaction/export handling visible as a requirement and defer detailed controls to PKG-12 where needed. |

## Construction

The calculation report generator deliverable is specified here as a bounded backend feature slice, not implemented in this setup session. Its future implementation should assemble report content from schema-first project/model/result envelopes, diagnostics, provenance metadata, and rule-pack references. The report generator must preserve the diagnostic/result-envelope boundary injected by AB-00-03 and AB-00-06: report content reflects mechanics outputs, user-rule-check outputs, warnings, assumptions, and limitations without promoting software output into professional approval.

The setup artifact family prepared for this session is:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

## References

| Source | SourceRef | Use |
|---|---|---|
| `_CONTEXT.md` | Scope Coverage; Architecture Basis Injection | Deliverable identity, scope, objective, accepted architecture basis, and write boundary. |
| `docs/_Registers/Deliverables.csv` | Row DEL-08-01 | Registered description, anticipated artifacts, scope, objective, context envelope, and risk note. |
| `docs/_Registers/ScopeLedger.csv` | Row SOW-024 | Report content scope and protected-content warning. |
| `docs/CONTRACT.md` | OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-RULE-1..3, OPS-K-AUTH-1, OPS-K-REPORT-1..2, OPS-K-PRIV-1..2, OPS-K-UNIT-1, OPS-K-AGENT-1..4 | Invariant constraints for report generation. |
| `docs/SPEC.md` | Sections 1, 3, 6, 7, 8, 9, 11 | Architecture, report domain object, rule-pack metadata, warning classes, reporting/audit content, report tests, and acceptance semantics. |
| `docs/TYPES.md` | Sections 4, 6, 8, 9 | Analysis-status vocabulary, professional boundary terms, report settings/report object definitions, and lifecycle meaning. |
| `docs/DIRECTIVE.md` | Sections 2, 3, 4, 5, 6 | Founding data/professional boundaries, stop rules, and auditability principles. |

