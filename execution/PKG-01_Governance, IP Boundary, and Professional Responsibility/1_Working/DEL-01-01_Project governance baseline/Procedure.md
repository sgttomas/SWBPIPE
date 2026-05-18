# Procedure: DEL-01-01 Project governance baseline

## Purpose

This procedure describes how to produce and review the DEL-01-01 governance baseline artifacts inside the deliverable-local setup folder.

## Prerequisites

- Sealed DEL-01-01 brief and write scope.
- `_CONTEXT.md`, `_REFERENCES.md`, and decomposition/register rows for DEL-01-01.
- Access to `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/TYPES.md`, and `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`.
- No protected standards/code data or proprietary contribution data.

## Steps

1. Confirm the deliverable identity: DEL-01-01, PKG-01, SOW-001, SOW-048, OBJ-001, and OBJ-002.
2. Read the applicable contract invariants for hierarchy, IDs, IP boundary, professional authority, governance, and agent execution.
3. Draft deliverable-local `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`; do not edit repo-level governance targets.
4. Mark unresolved governance choices as `TBD`, including license, maintainer roster, quorum, release signing, release authority, and legal review process.
5. Check that draft language does not assert certification, sealing, endorsement, code compliance, legal opinion, or professional approval.
6. Generate `_SEMANTIC.md` as a question-shaping semantic lens after the four documents exist.
7. Generate `_SEMANTIC_LENSING.md` from `_SEMANTIC.md` and the four production documents.
8. Apply P3 enrichment only where the lensing register identifies warranted additions supported by local sources.
9. Generate `Dependencies.csv` and refresh `_DEPENDENCIES.md` using conservative dependency extraction.
10. Persist `_run_records` evidence for the setup sequence and leave `_STATUS.md` below `ISSUED`.

## Verification

| Check | Expected result |
|---|---|
| Scope boundary | Only the DEL-01-01 deliverable folder is written. |
| Repo-level targets | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `governance/MAINTAINERS.md` are not edited in this setup run. |
| Protected content | No protected standards/code text, tables, examples, or proprietary data are reproduced. |
| Professional boundary | No certification, approval, sealing, authentication, or compliance-for-reliance claim appears. |
| Unknowns | Unresolved policy values are marked `TBD`. |
| Setup artifacts | Four documents, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, and `_run_records` exist. |

Release validation disclosures are communication controls. They must not be treated as professional reliance approval, code compliance, certification, sealing, endorsement, or authentication.

The durable run record should explicitly state whether repo-level artifacts were edited, whether protected standards/code content was reproduced, and which unresolved governance decisions remain `TBD`.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
