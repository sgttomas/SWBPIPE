# Procedure: DEL-08-05 Report protected-content linter

## Purpose

This procedure records how DEL-08-05 setup artifacts are produced and how future implementation work should approach the protected-content linter without exceeding the public IP/data boundary.

## Prerequisites

| Prerequisite | Source | Status |
|---|---|---|
| Sealed deliverable context for DEL-08-05 with explicit write scope. | User brief; `_CONTEXT.md` | Available for this setup run. |
| Local governance references for IP/data, report boundary, professional boundary, and agent constraints. | `_REFERENCES.md`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/TYPES.md`; `docs/DIRECTIVE.md` | Available. |
| Decomposition/register scope for SOW-043, OBJ-002, OBJ-007. | `docs/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/*.csv` | Available. |
| Future report template/example locations. | DEL-08-01 and future implementation work | TBD; not required for setup artifacts. |
| Future CI guard location and policy. | DEL-10-04 or later authorized implementation work | TBD; not modified in this setup session. |

## Steps

### Setup sequence used in this session

1. Read the sealed context and required governance/decomposition files.
2. Run `four-documents` with `RUN_PASSES=P1_P2` by drafting `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from accessible local sources.
3. Run `semantic-matrix-build` by replacing the placeholder `_SEMANTIC.md` with deliverable-bound semantic matrices and recording audit PASS.
4. Run `lens-register` by creating `_SEMANTIC_LENSING.md` with complete matrix-cell coverage.
5. Run `four-documents` with `RUN_PASSES=P3_ONLY` by applying warranted lensing findings or recording that no document rewrite was warranted.
6. Run `dependency-extract` by creating `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md`.
7. Validate setup gates and set `_STATUS.md` to `SEMANTIC_READY` only after checks pass.

### Future implementation procedure

1. Confirm authorized scan surfaces.
   - Enumerate public report template/example paths.
   - Keep private/user template paths out of default scanning.
   - Record opt-in behavior for local private scans if supported.
2. Define safe linter finding categories.
   - Protected content risk.
   - Private data risk.
   - Unknown/weak redistribution status.
   - Prohibited professional-authority claim.
   - Review-required or quarantine-required disposition.
3. Build fixtures without protected content.
   - Use invented markers, placeholders, and synthetic table/formula shapes.
   - Do not copy standards text, standards tables, proprietary formulas, vendor tables, or commercial examples into fixtures.
4. Implement deterministic scan behavior.
   - Stable inputs/config/version produce stable findings.
   - Findings cite file and best-effort location.
   - Unknowns become review-required findings.
5. Integrate diagnostics and CI policy.
   - Emit diagnostic classes compatible with the project result-envelope/warning model.
   - Map severity to fail/warn/review-required behavior only after policy approval.
6. Verify boundaries.
   - Check no private data transmission occurs by default.
   - Check public templates/examples remain protected-content-free.
   - Check public report language avoids certification, sealing, approval, authentication, and automatic code-compliance claims.
7. Prepare review evidence.
   - Include test results, fixture provenance, known limitations, and residual risk notes.
   - State that heuristic linting is not sole legal control.

## Verification

Setup verification for this session:

- Four production documents exist and contain the default schema sections.
- `_SEMANTIC.md` contains canonical matrices A/B and derived matrices C/F/D/K/G/X/T/E with audit PASS.
- `_SEMANTIC_LENSING.md` covers matrices A/B/C/F/D/X/E.
- `Dependencies.csv` validates against the v3.1 required-column schema.
- Dependency enum values validate against the local enum validator.
- `_STATUS.md` reports `SEMANTIC_READY`.
- No protected examples, proprietary formulas, private data, linter source, CI edits, report-template edits, or repo-level artifacts are introduced.

## Records

| Record | Purpose |
|---|---|
| `Datasheet.md` | Descriptive setup facts and boundary conditions. |
| `Specification.md` | Normative setup requirements and future verification targets. |
| `Guidance.md` | Rationale, principles, trade-offs, examples, and open questions. |
| `Procedure.md` | Setup and future implementation workflow. |
| `_SEMANTIC.md` | Semantic matrix setup lens. |
| `_SEMANTIC_LENSING.md` | Coverage-complete lensing register. |
| `Dependencies.csv` | Machine-readable dependency register v3.1. |
| `_DEPENDENCIES.md` | Human-readable dependency summary and run notes. |
| `_run_records/*` | Setup run evidence for each required step. |
| `_STATUS.md` | Lifecycle state and history. |

