# Guidance: DEL-08-01 Calculation report generator

## Purpose

The calculation report generator exists to make analysis outputs reviewable and reproducible without converting software output into professional approval. It should help a competent reviewer see what was analyzed, what data was supplied, what assumptions or gaps remain, what warnings were raised, what rule-pack metadata was used, and what limitations apply.

## Principles

| Principle | Guidance |
|---|---|
| Auditability first | Report content should be traceable to model inputs, result envelopes, diagnostics, provenance records, report settings, and rule-pack references. |
| Boundary clarity | The report should separate mechanics solve output, user-rule-check output, incomplete inputs, and human review. |
| No protected content | Public report templates and examples should contain placeholders, invented values, or safe metadata, not protected code text/tables/formulas. |
| No professional overclaim | Report language should say what the software computed and what data was supplied, not that the work is certified, sealed, approved, or code-compliant for reliance. |
| Unit visibility | Unit-bearing values should display units and keep dimensional context visible. |
| Reproducibility | Reports should bind to versions, hashes, manifests, warnings, and rule-pack checksums so reviewers can reproduce or compare results. |
| Local-first privacy | Private project/rule/material/component data should remain user-controlled unless intentionally exported with documented rights. |

## Considerations

The report generator will depend on upstream model/result/rule-pack envelopes that are not implemented in this setup session. Where the current source set does not provide exact field names, template structure, renderer API, or output format, those details remain `TBD` for future sealed implementation briefs.

The report generator should be conservative about rule-pack content. A report can identify a rule pack, version, checksum, required-input state, source notice, redistribution status, and user-provided notice. It should not copy protected rule text or proprietary formulas into public templates/examples.

The report generator should expose warnings as findings, not as decoration. Missing solve-required values, rule-check-required values, weak provenance, assumptions, nonlinear uncertainty, and IP-boundary concerns should be report-visible when present in the input envelopes.

## Trade-offs

| Topic | Conservative direction |
|---|---|
| Human-readable PDF/HTML vs structured output | This deliverable can define report assembly behavior, but result export format is a separate DEL-08-04 concern. Avoid binding a public transport/output format here beyond setup requirements. |
| Rich template language vs protected-content control | Prefer constrained placeholders and safe metadata over a template model that encourages copying protected code text into public examples. |
| User-private templates vs public templates | User-private templates may contain user-owned content under user responsibility; public templates must stay protected-content-free. |
| Completion language vs professional boundary | Use "generated", "computed", "reported", "checked by user rule pack" and "human review required"; avoid compliance/certification language. |

## Examples

Safe report section labels may include:

- Model input summary
- Source and provenance summary
- Load case and combination summary
- Result summary
- Warnings and missing data
- Assumptions and limitations
- Rule-pack references and checksums
- Reproducibility manifest
- Professional review notice

These labels are illustrative only and do not define a renderer template in this setup session.

## Conflict Table (for human ruling)

No source conflicts were identified in the local reference set for this setup run.

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | None identified | N/A | N/A | N/A | N/A | N/A |

## Open Questions

| ID | Question | Current disposition |
|---|---|---|
| OQ-08-01-001 | Exact renderer library, file format, and report template language. | TBD; outside setup-session write scope. |
| OQ-08-01-002 | Exact report schema fields and result-envelope field names. | TBD; downstream schema/service deliverables must define before implementation. |
| OQ-08-01-003 | Redaction/export configuration for private project data in shared reports. | TBD; coordinate with PKG-12 in future sealed work. |
| OQ-08-01-004 | Human approval workflow and acceptance-record binding to report hashes. | TBD; outside software authority unless human governance defines a record process. |

