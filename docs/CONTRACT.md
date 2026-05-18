---
doc_id: OPS-CONTRACT
doc_kind: governance.contract
status: draft
created: 2026-04-30
refs:
  - rel: depends_on
    to: OPS-TYPES
  - rel: informs
    to: OPS-SPEC
---

# CONTRACT — OpenPipeStress Invariant Catalog

This document defines binding invariants for agentic development of OpenPipeStress. Invariants are intended to become enforceable through schemas, tests, linters, review gates, and human governance.

## 1. Invariant index

|ID|Invariant|Enforcement|
|---|---|---|
|OPS-K-HIER-1|Packages are flat work domains; deliverables are the smallest production units.|SOFTWARE_DECOMP coverage audit; human review|
|OPS-K-ID-1|Package, deliverable, scope, objective, decision, and issue IDs are stable and not reused.|Decomposition registers; review|
|OPS-K-IP-1|The public repository must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data.|Contributor review; report linter; legal review|
|OPS-K-IP-2|Every public data contribution must include source, provenance, license/redistribution status, contributor certification, and review disposition.|Data import validator; contribution checklist|
|OPS-K-IP-3|Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data.|Stop rule; human/legal review|
|OPS-K-DATA-1|Code-specific values are user-supplied or lawfully imported private data, not bundled public defaults.|Rule-pack completeness checker; library governance|
|OPS-K-DATA-2|Missing solve-required or rule-check-required values are explicit findings, never silent defaults.|GUI warnings; solver/rule engine validation|
|OPS-K-DATA-3|Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields.|Schema validation|
|OPS-K-AUTH-1|Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance.|Report templates; product-claims policy; review|
|OPS-K-AUTH-2|Human acceptance records, if used, bind to specific model/rule/report hashes and do not survive content changes without re-review.|Audit manifest; hash checks|
|OPS-K-MECH-1|The primary global analysis model is a 3D centerline/frame model; shell/solid FEA is a local-analysis handoff path.|Architecture review; docs|
|OPS-K-MECH-2|The solver computes mechanics; rule packs evaluate user-defined acceptability; professional compliance remains human judgment.|Analysis status model; reports|
|OPS-K-UNIT-1|All calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked.|Unit tests; schema validation; rule evaluator|
|OPS-K-SOLVER-1|Solver changes require deterministic verification tests before release.|CI; V&V gate|
|OPS-K-SOLVER-2|Nonlinear support behavior must report convergence, active-set state, and unresolved non-convergence.|Solver tests; result schema|
|OPS-K-RULE-1|Public rule-pack examples must use invented non-code values and clear non-engineering notices.|Contribution review; report linter|
|OPS-K-RULE-2|The expression evaluator is sandboxed; rule packs cannot execute arbitrary code.|Security tests; architecture review|
|OPS-K-RULE-3|Rule packs are versioned, checksummed, source-noted, and marked public/private.|Rule-pack schema validation|
|OPS-K-REPORT-1|Reports must disclose model version, solver version, rule-pack checksum, source/provenance notes, warnings, assumptions, and limitations.|Report generator tests|
|OPS-K-REPORT-2|Public report templates and examples must not reproduce protected standards content.|Report protected-content linter; human review|
|OPS-K-PRIV-1|Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default.|Storage policy; telemetry config; CI checks|
|OPS-K-PRIV-2|Telemetry is off by default and cannot include private engineering/code data.|Security review; telemetry tests|
|OPS-K-GOV-1|The project is intended to be free and open source, but the exact license remains `TBD` until recorded by the human project authority.|Maintainer policy; release review|
|OPS-K-GOV-2|Maintainer authority, release authority, and public-governance decisions must be recorded in public governance artifacts before they are treated as project policy.|Maintainer policy; decision records|
|OPS-K-GOV-3|Public releases must disclose scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations.|Release checklist; human review gate|
|OPS-K-GOV-4|Maintainers must apply IP, provenance, privacy, and protected-content review before accepting public repository contributions.|Contribution review; protected-content gate|
|OPS-K-AGENT-1|Agents must not invent engineering values, scope, source citations, or legal conclusions; unknowns become `TBD`.|Agent instructions; review|
|OPS-K-AGENT-2|Agents must surface conflicts and gaps rather than silently resolving them.|Review; conflict/open issue registers|
|OPS-K-AGENT-3|Type 2 execution requires sealed context and explicit deliverable scope.|Agentic workflow; orchestration|
|OPS-K-AGENT-4|Agent outputs are drafts/proposals until accepted by a human gate.|Review; lifecycle state|

## 2. Enforcement map

| Enforcement point | Invariants checked |
|---|---|
| Decomposition review | OPS-K-HIER-1, OPS-K-ID-1, OPS-K-AGENT-3 |
| Schema validation | OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-3 |
| CI/test suite | OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-RULE-2, OPS-K-REPORT-2 |
| Contributor review | OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-RULE-1 |
| Report generation | OPS-K-AUTH-1, OPS-K-MECH-2, OPS-K-REPORT-1, OPS-K-REPORT-2 |
| Human review gate | OPS-K-AUTH-1, OPS-K-AUTH-2, OPS-K-AGENT-4 |
| Security/privacy review | OPS-K-PRIV-1, OPS-K-PRIV-2, OPS-K-RULE-2 |
| Maintainer/release governance | OPS-K-GOV-1, OPS-K-GOV-2, OPS-K-GOV-3, OPS-K-GOV-4, OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-AUTH-1 |

## 3. Retired invariants

None.
