---
run_id: TASK_RUN_2026-05-17_TP-STRESS-016
task: TP-STRESS-016 Governed Stress Section Inputs
deliverable_id: DEL-09-02
package_id: PKG-09
requested_by: WORKING_ITEMS orchestrator
execution_mode: parent_orchestrator_direct_task
date: 2026-05-17
---

# TP-STRESS-016 Governed Stress Section Inputs

## Loaded Truth Set

- Global: `AGENTS.md`, `agents/AGENT_WORKING_ITEMS.md`,
  `agents/AGENT_TASK.md`, `docs/CONTRACT.md`, `docs/SPEC.md`,
  `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  `execution/_Coordination/_COORDINATION.md`, `execution/_DAG/_LATEST.md`,
  and `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`.
- Deliverable-local: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`,
  `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary stress
  benchmark/hand-calculation artifacts.
- Adjacent evidence: TP-PHYS-015C and TP-VERIFY-012C run records,
  section-property calculator tests, result-export schema evidence, and
  mechanics benchmark TP-PHYS-015 result-boundary evidence.

## Decision Applied

- Stress recovery references governed section-property calculation evidence
  owned by `DEL-03-08`; it does not derive hidden section modulus values from
  fixture-local defaults.

## Changes

- Added `GovernedStressSectionEvidence` to the stress benchmark crate.
- Added evidence id
  `SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25` with explicit
  invented OD/wall inputs and calculated area, section modulus, torsion
  constant, and torsion radius.
- Routed the TP-PHYS-015 canonical resultant stress fixture through that
  governed evidence before constructing `StressSectionProperties`.
- Updated the TP-PHYS-015 stress hand calculation with governed section
  evidence and revised expected mechanics-only stress values.

## Validation

- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml`
- `python3 tests/test_results_schema.py`
- `python3 tests/test_headless_runner_contract.py`
- `python3 -m pytest tests/test_physical_to_analytical_transform.py tests/test_analytical_solver_boundary_adapter.py`
- `git diff --check`

## Gaps

- Public schema/runtime transport of section-property calculation evidence is
  not added here.
- Tolerance, release, CI, publication, and professional-reliance policy remain
  deferred to their owning deliverables and human authority.

## Boundaries Preserved

- No lifecycle/status changes.
- No dependency register, DAG, blocker, candidate-promotion, review-finding,
  release, or acceptance changes.
- No public API, CLI, GUI, report, persistence, or solver-behavior expansion.
- No protected standards content, private/proprietary data, allowables,
  SIF/flexibility data, fatigue/design-code checks, professional reliance
  claim, code-compliance claim, release statement, or human-acceptance
  statement.

## No-Claim Closeout

This record documents bounded mechanics benchmark evidence only. It is not an
acceptance record, release record, professional approval, certification,
sealing, authentication, or code-compliance claim.
