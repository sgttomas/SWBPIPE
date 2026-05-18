# Datasheet: DEL-08-05 Report protected-content linter

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-08-05 |
| Deliverable Name | Report protected-content linter |
| Package ID | PKG-08 |
| Package Name | Reporting, Audit, and Reproducibility |
| Deliverable Type | TEST_SUITE |
| Scope Item | SOW-043 |
| Objectives | OBJ-002, OBJ-007 |
| Context Envelope | M |
| Current Setup Boundary | Document/setup artifacts only; no linter source, CI guard, tests, report templates, or repo-level artifacts are implemented in this session. |

## Attributes

| Attribute | Value |
|---|---|
| Primary purpose | Define setup expectations for checks that prevent public report templates/examples from automatically embedding protected code text, copied standards tables, proprietary formulas, or private/proprietary engineering content. |
| Public/private boundary | Public templates and examples must stay protected-content-free. User-private report templates remain user responsibility. |
| Review posture | Heuristic plus review; this linter cannot be the sole legal/IP control. |
| Anticipated future artifacts | `report linter`; `CI guard` |
| Authorized current artifacts | Four-document setup kit, semantic artifacts, dependency register, run records, status update. |
| Explicit exclusions | No linter source, no CI workflow edits, no test fixtures embedding protected examples, no legal-sufficiency claim, no `ISSUED` movement. |

## Conditions

| Condition | SourceRef | Disposition |
|---|---|---|
| Public repository must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3 | Binding guardrail for future linter scope. |
| Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Section 5 | Linter findings must route to review rather than automatic acceptance. |
| Public report templates and examples must not reproduce protected standards content. | `docs/CONTRACT.md` OPS-K-REPORT-2; `docs/SPEC.md` Section 8 | Direct requirement for this deliverable. |
| Reports must not claim certification, sealing, approval, authentication, or code compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Section 4 | Linter scope may include prohibited public-claim language, but human review remains required. |
| Reports may reference user rule-pack IDs, versions, checksums, and source notes without embedding protected formulas or tables. | `docs/IP_AND_DATA_BOUNDARY.md` Section 7; `docs/SPEC.md` Sections 6 and 8 | Safe metadata is allowed; rule-pack content remains user/private unless rights are documented. |
| Telemetry/private data boundaries must be preserved. | `docs/CONTRACT.md` OPS-K-PRIV-1; OPS-K-PRIV-2 | Future linter should not transmit or collect private project/rule data. |

## Construction

The future linter should be framed as a conservative protected-content and public-report guard. It should inspect only authorized public report-template/example surfaces unless a user explicitly runs it against private material. It should flag suspected protected or private content for human review, not decide legal sufficiency or professional acceptability.

Expected future check categories:

| Category | Intended behavior | Boundary |
|---|---|---|
| Protected text/table/formula indicators | Detect likely copied standards/code/vendor content in public templates/examples using heuristic patterns and allowlisted safe placeholders. | No protected examples may be embedded as fixtures. |
| Public/private marker checks | Require explicit public/private or redistribution-status metadata where template/example content may expose rule-pack or provenance text. | Unknown status is a finding, not a silent pass. |
| Prohibited public claims | Flag public report language that claims software certification, sealing, approval, authentication, or automatic code compliance. | The check supports OPS-K-AUTH-1 but does not replace human review. |
| Safe metadata allowance | Permit rule-pack identity, version, checksum, source notice, redistribution status, and user-supplied notices when represented as metadata. | Do not reproduce protected rule formulas or tables in public artifacts. |
| Review routing | Emit actionable diagnostics such as `IP_BOUNDARY_WARNING` and review-required findings. | Do not auto-quarantine outside future authorized implementation scope. |

## References

| Reference | Use |
|---|---|
| `INIT.md` | Root bootstrap and agent constraints. |
| `AGENTS.md` | TASK dispatch and package role constraints. |
| `docs/CONTRACT.md` | Invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-RULE-1/3, OPS-K-AUTH-1, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4, OPS-K-REPORT-2. |
| `docs/SPEC.md` | Reporting/audit requirements, warning classes, rule-pack metadata boundary, V&V expectations. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, quarantine rule, report boundary. |
| `docs/TYPES.md` | Analysis status vocabulary, protected-content vocabulary, report object boundary. |
| `docs/DIRECTIVE.md` | Founding stop rules and professional/IP responsibility constraints. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-08 and DEL-08-05 decomposition basis, SOW-043, OBJ-002, OBJ-007, architecture-basis IDs. |
| `docs/_Registers/Deliverables.csv` | DEL-08-05 register row. |
| `docs/_Registers/ScopeLedger.csv` | SOW-043 register row. |

