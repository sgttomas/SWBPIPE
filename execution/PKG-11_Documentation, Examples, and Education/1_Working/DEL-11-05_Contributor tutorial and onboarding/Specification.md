# Specification: DEL-11-05 Contributor tutorial and onboarding

## Scope

This deliverable defines a contributor onboarding tutorial draft inside the DEL-11-05 working folder. It covers how contributors should orient to OpenPipeStress governance, the package/deliverable decomposition, sealed Type 2 execution, evidence production, review handoff, and protected-data boundaries.

This setup run does not edit repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, documentation outside this deliverable, source code, examples, schemas, or release artifacts. Future publication of the tutorial into repo-level onboarding surfaces requires a separate approved task or human action.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-11-05-01 | The tutorial draft shall start contributors from root bootstrap and governance reading before any implementation or documentation work. | INIT.md required reading order; docs/README.md document map | Procedure includes onboarding read sequence. |
| REQ-11-05-02 | The tutorial draft shall explain the flat package/deliverable hierarchy and stable ID model used by OpenPipeStress. | docs/TYPES.md sections 1-2; SOFTWARE_DECOMP revision 0.4 | Datasheet and Procedure use PKG/DEL identity consistently. |
| REQ-11-05-03 | The tutorial draft shall describe Type 1 and Type 2 agent responsibilities without expanding TASK authority beyond a sealed deliverable. | AGENTS.md primary agents and dispatch rule; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md sections 1-4 | Guidance and Procedure distinguish routing, execution, review, and human authority. |
| REQ-11-05-04 | The tutorial draft shall require contributors to preserve protected standards, vendor IP, private data, and provenance controls. | docs/CONTRACT.md OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-PRIV-1; docs/IP_AND_DATA_BOUNDARY.md sections 2-6 | Guidance includes stop rules; dependencies record governing constraints. |
| REQ-11-05-05 | The tutorial draft shall preserve the rule-pack boundary: public examples use invented non-code values and user/code data remains user supplied. | docs/CONTRACT.md OPS-K-RULE-1 and OPS-K-RULE-3; docs/SPEC.md section 6; ScopeLedger.csv SOW-033 | Guidance and Procedure prohibit protected or private rule-pack examples. |
| REQ-11-05-06 | The tutorial draft shall preserve professional responsibility boundaries and avoid certification, code-compliance, approval, sealing, or reliance claims by agents/software. | docs/CONTRACT.md OPS-K-AUTH-1; docs/TYPES.md section 4; docs/DIRECTIVE.md sections 3 and 6 | Text scan confirms no software/agent compliance certification claim. |
| REQ-11-05-07 | The tutorial draft shall tell contributors to use architecture-basis constraints only as dispatch context and not as ISSUED product authority. | _CONTEXT.md Architecture Basis Injection; SCA-001 Handoff_State.md Explicit Holds | Procedure includes architecture-basis handling. |
| REQ-11-05-08 | The tutorial draft shall include evidence expectations: changed paths, validation commands/results, warnings, open issues, and review handoff. | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md sections 4-5; docs/SPEC.md section 11 | Procedure includes final handoff checklist. |
| REQ-11-05-09 | The setup artifacts shall include four documents, semantic matrix, lensing register, dependency register, run records, and final status only if setup gates pass. | Human brief; skills/four-documents/SKILL.md; skills/semantic-matrix-build/SKILL.md; skills/lens-register/SKILL.md; skills/dependency-extract/SKILL.md | Local validation commands pass before final `_STATUS.md` is set to SEMANTIC_READY. |

## Standards

No engineering design standard content is used as source material for this contributor tutorial. Governing standards for this deliverable are the project governance and workflow documents listed in `_REFERENCES.md`. If a future contributor wants to include any standard, vendor, owner, or commercial software material in onboarding examples, the correct action is to stop and route the material through protected-content/provenance review rather than copy or paraphrase it into public documentation.

## Verification

| Check | Expected result |
|---|---|
| Four-document presence | `tools/validation/check_four_documents.sh <deliverable>` returns PASS. |
| Minimum metadata presence | `tools/validation/check_min_viable_fileset.sh <deliverable>` returns PASS. |
| Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` returns VALID. |
| Lifecycle enum | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` returns VALID. |
| Protected-data boundary scan | Local review finds no protected standards tables, code examples, proprietary vendor data, private rule packs, or commercial software examples introduced. |
| Professional-claims scan | Local review finds no automatic compliance, certification, approval, sealing, or professional reliance claim by software or agents. |
| Semantic setup | `_SEMANTIC.md` has Audit Result PASS and `_SEMANTIC_LENSING.md` includes coverage rows for matrices A, B, C, F, D, X, and E. |

## Documentation

Required setup outputs for this deliverable are:

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

Repo-level publication targets remain references only in this setup session.

## Required Invariants

| Invariant | Application |
|---|---|
| OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3 | Public onboarding content must exclude protected standards/vendor/commercial/private material and must require provenance review for public data. |
| OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3 | Code-specific and proprietary engineering data are user supplied or private; missing values stay explicit. |
| OPS-K-RULE-1, OPS-K-RULE-3 | Public rule-pack examples are invented/non-code and rule packs carry version, checksum, source notes, and public/private markers where referenced. |
| OPS-K-AUTH-1 | No software or agent certification, approval, sealing, authentication, or code-compliance reliance claim. |
| OPS-K-PRIV-1 and OPS-K-PRIV-2 | Private project/code/rule/component data and telemetry boundaries stay visible. |
| OPS-K-AGENT-1 through OPS-K-AGENT-4 | Contributors and agents do not invent facts; conflicts are surfaced; Type 2 work is sealed; outputs are drafts until accepted. |
| No-bypass adapter/API baseline | Contributors touching plugins/adapters/API topics must preserve units, provenance, diagnostics, rule sandboxing, report controls, and data-boundary checks. |

## Acceptance Criteria

This setup deliverable is ready for review when the listed setup artifacts exist, validation commands pass, no protected-data or certification-claim warning is found in the local artifacts, and `_STATUS.md` records `Current State: SEMANTIC_READY`.
