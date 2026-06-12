# WORKING_ITEMS Run Record - TP-APP-R2-SAVEDPROJECT-SMOKE-001

- Agent: WORKING_ITEMS (Type 1 persona)
- Date: 2026-06-12
- Tranche: `TP-APP-R2-SAVEDPROJECT-SMOKE-001`
- Plan item: Phase A8 GUI test harness
- Deliverable context: `DEL-00-08` Layered software test and acceptance strategy
- Authority basis: `execution/_Coordination/_COORDINATION.md`, `plans/PLAN_2026-06-10_prd_completion.md`, PRD §22.3 R2 GUI MVP, `DEC-025` five-surface local sweep posture

## Objective

Close the A8 packaged-Tauri/backend saved-project smoke residual by proving
that the A12-authored model can be persisted into the desktop local project
store, reopened, solved through the backend mechanics command, and rendered
through the backend report renderer.

## Scope

- Updated `apps/desktop/src-tauri/src/lib.rs`.
- Reused the existing A12 rehearsal helpers and invented fixture.
- Did not add new runtime dependencies, external services, network paths,
  protected data, or lifecycle-state changes.

## Changes

- Added Tauri regression
  `r2_from_blank_saved_project_opens_solves_and_renders_report`.
- The test applies the A12 from-blank rehearsal steps to build the authored
  model, saves the authored model into an in-memory SQLite local project
  store through the same store helper used by the desktop commands, reloads
  it by project id, and verifies no mechanics/result payload was silently
  persisted before the solve.
- The reloaded model then solves through `run_preview_mechanics` with
  `MECHANICS_SOLVED`, renders through `render_calculation_report`, and
  records a local project summary with persisted mechanics and analysis-run
  references.

## Validation

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml r2_from_blank_saved_project_opens_solves_and_renders_report`
  - passed, 1/1 focused test.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - passed, 31/31 Tauri Rust tests.

## Boundary Review

- The saved project is an in-memory SQLite test store over invented A12 data.
- The test does not write user project data to the repository or copy external
  files.
- No cloud, daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was
  introduced.

## Residuals

- A8 still needs broader SMOKE checklist parity if the project wants every
  manual smoke row represented by automated assertions.

## Lifecycle

- `DEL-00-08` remains `CHECKING`.
- This run record is implementation/test evidence only; it is not lifecycle
  issuance, release readiness, professional approval, certification, sealing,
  authentication, or code-compliance acceptance.
