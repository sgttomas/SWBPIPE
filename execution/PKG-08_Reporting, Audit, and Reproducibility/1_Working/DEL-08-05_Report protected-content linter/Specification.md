# Specification: DEL-08-05 Report protected-content linter

## Scope

This deliverable specifies setup requirements for a protected-content linter that will guard public report templates and examples against accidental inclusion of protected standards/code text, copied standards tables, proprietary formulas, private rule-pack content, or misleading professional-authority claims.

In scope for future implementation:

- checks over public report templates/examples and other explicitly configured public report surfaces;
- heuristic detection of protected-content risk categories without embedding protected examples as fixtures;
- diagnostics that route suspected protected/private content to human review;
- checks that public report language does not claim software certification, sealing, approval, authentication, or automatic code compliance;
- CI guard integration when separately authorized by implementation scope.

Out of scope for this setup session:

- writing linter source code;
- adding or modifying CI guards;
- adding tests or protected-content fixtures;
- editing report templates or report generator source;
- making legal sufficiency, compliance, certification, or professional-reliance claims;
- moving the deliverable to `ISSUED`.

## Requirements

| Requirement ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-08-05-REQ-001 | The future linter shall scan only authorized public report-template/example surfaces by default, not user-private templates or private project/rule data. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 6-7; `docs/CONTRACT.md` OPS-K-PRIV-1 | Future unit/integration tests verify public-surface targeting and private-surface opt-in behavior. |
| DEL-08-05-REQ-002 | The future linter shall flag suspected protected code text, copied standards tables, protected figures/examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data in public templates/examples. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3; `docs/SPEC.md` Section 8 | Future linter fixtures use invented/synthetic trigger placeholders and safe negative fixtures; no protected examples are embedded. |
| DEL-08-05-REQ-003 | The future linter shall not treat a clean heuristic scan as legal clearance or professional approval. | `docs/CONTRACT.md` OPS-K-AGENT-4; `docs/_Registers/ContextBudgetQA.csv` row DEL-08-05 | Future report and CI output includes review-required wording; review checklist confirms no legal-sufficiency claim. |
| DEL-08-05-REQ-004 | The future linter shall route suspected protected content to quarantine/review workflow signals rather than accepting, paraphrasing, or normalizing the content into public artifacts. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Section 5 | Future tests verify suspected-content diagnostics and review routing. |
| DEL-08-05-REQ-005 | The future linter shall permit safe report metadata such as rule-pack ID, version, checksum, source notice, redistribution status, and required-input status when the metadata does not reproduce protected formulas or tables. | `docs/SPEC.md` Sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` Section 7; `docs/CONTRACT.md` OPS-K-RULE-3 | Future fixtures verify metadata allowance and protected rule-content rejection. |
| DEL-08-05-REQ-006 | The future linter shall flag public report wording that claims the software certifies, seals, approves, authenticates, or declares engineering code compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Sections 4 and 8; `docs/DIRECTIVE.md` Sections 4-6 | Future text fixtures verify prohibited-claim detection while permitting decision-support wording. |
| DEL-08-05-REQ-007 | The future linter shall surface findings using diagnostics compatible with report/result warning contracts, including `IP_BOUNDARY_WARNING` where applicable. | `docs/SPEC.md` Section 7; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Future diagnostic output tests verify code/class/severity/source/remediation fields where available. |
| DEL-08-05-REQ-008 | Public examples used to exercise the linter shall use invented non-code values or synthetic markers and shall clearly avoid standards-body/vendor copied content. | `docs/CONTRACT.md` OPS-K-RULE-1; `docs/SPEC.md` Section 9 | Future fixture review confirms invented-only content and source/provenance notes. |
| DEL-08-05-REQ-009 | The future linter shall be deterministic for the same input files, configuration, and version so that CI and review evidence are reproducible. | `docs/SPEC.md` Section 9; `docs/CONTRACT.md` OPS-K-REPORT-1 | Future tests compare stable machine-readable finding output. |
| DEL-08-05-REQ-010 | The future CI guard shall fail or warn according to a documented severity policy, with suspected protected content treated as review-blocking until human/legal disposition is recorded. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` Sections 4-5 | Future CI tests verify severity mapping and review handoff behavior. |
| DEL-08-05-REQ-011 | The future linter shall not transmit private project, material, component, or rule-pack data by default. | `docs/CONTRACT.md` OPS-K-PRIV-1 and OPS-K-PRIV-2 | Future security/privacy tests verify no telemetry or external transfer by default. |
| DEL-08-05-REQ-012 | The future implementation shall preserve schema-first result-envelope and no-bypass adapter/plugin constraints from the architecture basis where linter output integrates with reports, APIs, or CI. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-06, AB-00-07, AB-00-08 | Architecture review verifies integration boundaries before implementation acceptance. |

## Standards

No protected standards text, standards tables, protected examples, or proprietary formulas are needed or authorized for this deliverable. Applicable public project standards are the OpenPipeStress governance artifacts listed in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/TYPES.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md`.

## Verification

Future implementation work should provide at least these checks:

| Check | Purpose |
|---|---|
| Public-surface scan fixture | Verifies only configured public report-template/example paths are scanned by default. |
| Synthetic protected-risk fixture | Verifies invented markers trigger protected-content findings without copying protected content. |
| Safe metadata fixture | Verifies rule-pack identity/version/checksum/source notice can appear without embedding protected rule content. |
| Prohibited-claim fixture | Verifies public report language does not claim certification, sealing, approval, authentication, or automatic code compliance. |
| Diagnostic output fixture | Verifies findings carry stable code/class/severity/source/remediation fields where available. |
| Determinism check | Verifies stable output for the same inputs/config/version. |
| CI policy check | Verifies severity-to-fail/warn mapping and review-blocking behavior. |
| Privacy check | Verifies no private data transmission by default. |
| Human review checklist | Confirms the linter is treated as heuristic support, not sole legal control. |

## Documentation

Required setup artifacts for this run:

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

Future production artifacts anticipated by the register:

- report linter;
- CI guard.

## Acceptance Criteria For This Setup Session

- No file outside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/` is edited.
- No linter source, CI guard, tests, report templates, docs outside this deliverable, or repo-level artifacts are modified.
- Setup artifacts are grounded in local governance, decomposition, register, and context files.
- No protected standards content, proprietary examples, private project/rule data, or certification/compliance claim is introduced.
- `Dependencies.csv` validates against v3.1 schema and active rows contain evidence.
- `_STATUS.md` reports `SEMANTIC_READY` only after the four-document kit, semantic artifacts, dependency register, and validation checks are complete.

