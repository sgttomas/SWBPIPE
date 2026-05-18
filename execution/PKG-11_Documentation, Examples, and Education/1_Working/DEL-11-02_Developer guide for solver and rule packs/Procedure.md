# Procedure: DEL-11-02 Developer guide for solver and rule packs

## Purpose

This procedure defines how a future authorized session should produce or refresh the developer guide artifact for solver and rule-pack contributors while preserving OpenPipeStress architecture, IP, data, unit, test, and professional-responsibility boundaries.

## Prerequisites

| Prerequisite | Source or status |
|---|---|
| Sealed DEL-11-02 brief and explicit write scope | Required before editing the final guide artifact. |
| Current decomposition and registers | `docs/_Decomposition/SOFTWARE_DECOMP.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`. |
| Governing invariants | `docs/CONTRACT.md`, especially IP, data, unit, rule-pack, privacy, authority, and agent invariants. |
| Architecture basis | AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08. |
| Source references | `INIT.md`, `docs/DIRECTIVE.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/VALIDATION_STRATEGY.md`, and workflow docs. |
| Human authority for unresolved decisions | Required for license/contributor certification, solver numerical library, rule expression grammar, dependency versions, CI thresholds, and other `TBD` decisions. |

## Steps

1. Confirm the active write scope.
   - For this setup session, do not edit `docs/developer_guide/index.md`.
   - For a future guide-authoring session, verify the human explicitly authorizes that path.

2. Build the guide outline from the required content groups.
   - Architecture map.
   - Solver architecture.
   - Rule-pack architecture.
   - Test discipline.
   - Contribution boundaries.
   - Review and acceptance.

3. Source every non-trivial requirement from local governing material.
   - Prefer `docs/SPEC.md` for technical architecture and baseline mechanics.
   - Prefer `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` for boundary constraints.
   - Prefer `docs/VALIDATION_STRATEGY.md` for test families and release gates.
   - Prefer SOFTWARE_DECOMP revision 0.4 for scope, objectives, and architecture basis.

4. Draft solver sections conservatively.
   - Explain centerline/frame mechanics, six degree-of-freedom nodes, loads, stress recovery, diagnostics, and test hooks.
   - Do not copy protected standards formulas, protected examples, code tables, or proprietary commercial examples.
   - Keep exact tolerances, solver numerical library, and performance thresholds as `TBD` unless resolved by approved source.

5. Draft rule-pack sections conservatively.
   - Explain schema shape, required inputs, variables, checks, provenance, redistribution status, checksums, report notices, and missing-input behavior.
   - Explain sandboxing and unit-aware deterministic evaluation.
   - Do not provide protected code equations, allowables, SIF/flexibility factors, protected tables, or private rule-pack content.

6. Draft test-discipline sections.
   - Map solver and rule-pack changes to deterministic evidence families.
   - Include protected-content and provenance gates for public examples, fixtures, docs, reports, and templates.
   - Distinguish mechanics verification from validation and professional reliance.

7. Draft contribution-boundary sections.
   - Define permitted public contributions and prohibited public content.
   - Include quarantine/escalation steps for suspected protected or private data.
   - Require `TBD`, `ASSUMPTION`, and `PROPOSAL` labels where warranted.

8. Run local review checks before proposing the guide for review.
   - Check scope and anticipated artifact path.
   - Scan for protected-content risk and certification/compliance overclaims.
   - Confirm unit/provenance/test/professional boundaries are visible.
   - Confirm unresolved implementation choices remain `TBD`.

9. Record evidence.
   - Keep setup and drafting evidence in the deliverable folder.
   - Record validation commands and results.
   - Do not move anything to `ISSUED` without human review.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Scope check | Documentation-only setup; no source code, examples, repo-level docs, or final guide artifact edited in this session. |
| Boundary check | No protected standards text/tables/examples/formulas/proprietary data; no professional approval claims. |
| Coverage check | Solver architecture, rule-pack schema, test discipline, and contribution boundaries are all addressed. |
| TBD check | Unresolved implementation choices are marked `TBD`. |
| Dependency check | `Dependencies.csv` is v3.1 schema-valid and `_DEPENDENCIES.md` counts match. |
| Semantic setup | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and are lens-only evidence. |

## Records

Maintain these records in this deliverable folder:

- four-document setup kit;
- semantic matrix file and lensing register;
- dependency register and dependency index;
- `_STATUS.md` lifecycle history;
- `_run_records/*` for the five required setup invocations;
- validation command results in the final TASK response.

