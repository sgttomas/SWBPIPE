# Specification: DEL-07-08 Design-authoring state and comparison workspace

## Scope

DEL-07-08 covers the GUI workspace slice for design-authoring and comparison workflows. The scope includes design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, operation/diff review, and graphical comparison overlays. Source: `_CONTEXT.md` / Description and Anticipated Artifacts; `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076.

This deliverable does not own the backend contracts for design knowledge, constraint validation, physical-to-analytical transformation, immutable model states, analysis runs, comparison engines, structured model operations, operation validation, or operation audit trails. Those are declared upstream dependencies in `Dependencies.csv` and are consumed by this GUI workspace.

This deliverable must not silently supply missing code data, protected engineering values, owner requirements, rule-pack values, or professional acceptance states. Source: `docs/CONTRACT.md` / OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` / Public repository must not contain.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| REQ-07-08-001 | The workspace shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076 |
| REQ-07-08-002 | The workspace shall consume existing GUI, design, transform, comparison, and operation contracts rather than inventing placeholder semantics. | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015 |
| REQ-07-08-003 | GUI-originated mutations shall route through application-service command intents and shall not directly mutate persisted project payloads. | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05 |
| REQ-07-08-004 | The operation/diff review surface shall preserve the distinction between proposed, validated, accepted, and audited model operations; exact UI state labels are TBD pending upstream operation contracts. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-069, SOW-070; `Dependencies.csv` rows DAG-002-E0851 through DAG-002-E0853 |
| REQ-07-08-005 | The state/run browser shall treat immutable model states, analysis runs, and deterministic comparisons as first-class review records; exact data contract shape is inherited from upstream PKG-14 deliverables. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-071 through SOW-073; `Dependencies.csv` rows DAG-002-E0847 through DAG-002-E0850 |
| REQ-07-08-006 | Constraint and warning presentation shall preserve structured diagnostic fields and the project warning classes where available. | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06 |
| REQ-07-08-007 | Comparison views shall be diagnostic and review-oriented; they shall not present deterministic comparisons as automatic external validation, professional approval, or code compliance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-073; `docs/CONTRACT.md` / OPS-K-AUTH-1 |
| REQ-07-08-008 | Missing solve-required, rule-check-required, provenance, assumption, nonlinear, or IP-boundary information shall remain visible as findings or warnings and shall not be silently defaulted by the GUI. | `docs/SPEC.md` / GUI requirements; `docs/CONTRACT.md` / OPS-K-DATA-2 |
| REQ-07-08-009 | The GUI-facing implementation shall follow the accepted architecture basis: Tauri 2 where desktop-shell-facing, TypeScript/React/Vite where GUI-facing, and Three.js where 3D viewport-facing. Exact package versions remain TBD. | `_CONTEXT.md` / Architecture Basis Injection |
| REQ-07-08-010 | Verification shall include GUI-appropriate automated checks where implementation exists, including layered GUI tests and rendering/interaction checks; exact test harness and acceptance thresholds are TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-08; `docs/SPEC.md` / GUI requirements |

## Standards

| Standard or governing basis | Applicability |
|---|---|
| `docs/CONTRACT.md` | Binding project invariants for data boundary, no silent defaults, professional boundary, agent limits, and review status. |
| `docs/SPEC.md` | Technical architecture and GUI diagnostic requirements. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content and private-data boundary for public artifacts. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 | Accepted decomposition basis for SOW-076, OBJ-015, OBJ-016, PKG-07, and architecture-basis constraints. |
| PRD v0.2 references | Mentioned by decomposition for SOW-076, SOW-069 through SOW-073, but the PRD source text was not locally read in this workflow. Clause-level requirements from PRD remain TBD unless source text is supplied. |

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-07-08-001 | Review implemented workspace surfaces against SOW-076 surface list; automated GUI smoke tests are TBD until implementation exists. |
| REQ-07-08-002 | Inspect data adapters and UI fixtures for dependency-backed contract use; flag placeholder semantics as TBD or implementation blocker. |
| REQ-07-08-003 | Test that user edits produce application-service command intents rather than direct project payload mutation. Exact command API is TBD pending upstream contracts. |
| REQ-07-08-004 | Test operation review states against upstream operation validation and audit records once those contracts are available. |
| REQ-07-08-005 | Test state/run browser queries and comparison navigation against upstream PKG-14 records once those contracts are available. |
| REQ-07-08-006 | Verify warning class display for `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` where such diagnostics are supplied. |
| REQ-07-08-007 | Review UI copy, status labels, and exported comparison affordances for professional-boundary violations. |
| REQ-07-08-008 | Inject missing-data and provenance-warning fixtures; confirm the GUI surfaces findings without inventing values. |
| REQ-07-08-009 | Inspect package/runtime configuration once implementation files exist. |
| REQ-07-08-010 | Run GUI unit/component tests and Playwright/rendering checks once a frontend scaffold exists. |

## Documentation

Required deliverable-local setup artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv` and `_DEPENDENCIES.md` preserved as approved DAG-002 mirror/evidence artifacts

Implementation-stage artifacts anticipated by `_CONTEXT.md`:

- GUI design knowledge panel
- operation diff review
- state/run browser
- comparison overlays

Exact implementation file paths and product code artifacts are TBD because this workflow does not create product code.
