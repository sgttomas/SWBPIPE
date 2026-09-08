# Sealed launch brief V1 — F4-D1

RequestedBy: `/root/friction_execution`
RunID: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`
ParentInstanceID: `F4`
ChildInstanceID: `F4-D1`
Role: `TASK` Agent 2; role/nondelegation instruction+config asserted
TaskSkill: `software-defect-diagnosis`
RequestedModel: `gpt-5.6-sol`
RequestedReasoning: `high`
PackageID: `PKG-04`
DeliverableID: `DEL-04-04`
ScopePath: `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/diagnosis`
AcceptedBasis: source `779dedb8670625b36af07b89fc5557470e47c50e`; D-35 / DEC-067; accepted preparation friction `DECISION_BRIEF.md`; root F4 activation V1
Dependencies: evidence-stage only; no PROJECT_SETUP disposition assumed

## Objective

Reproduce and diagnose the unchanged M1-N-008 coupled-derived-normal witness against current source. Record original witness bytes/hashes before copying. Use an isolated temporary copy and a unique `CARGO_TARGET_DIR`; do not edit the historical witness or production source. Run both existing seeds and both existing modes. Then, in a separate temporary-only diagnostic extension, mirror the same fixture for both applied-force signs and run both seeds/modes. Freeze raw stdout/stderr losslessly and return the exact observed current-normal discrepancy, earliest divergent state, likely minimal repair surface, and verification recommendations.

## Declared reads and tools

- root/project instructions, `agents/AGENT_TASK.md`, `skills/software-defect-diagnosis/SKILL.md`
- F4 activation/graph and this brief
- original M1-N-008 `supplement_v2` witness/evidence
- `core/solver/nonlinear_integration/**`, `core/solver/nonlinear_supports/**`, `core/solver/frame_kernel/**`, `core/solver/diagnostics/**`
- Rust/Cargo and deterministic local shell probes; no network

## Allowed write targets

- `{RUN_ROOT}/instances/F4/children/D1/**`
- `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/diagnosis/**`
- ephemeral files under a freshly allocated system temporary directory, removed after evidence capture

## Exclusions

No product/source/test edit; no historical evidence edit; no decision/dependency/lifecycle/pointer mutation; no new reference/history/direction/tolerance choice; no delegation; no network; no release claim.

## Expected return and acceptance

Return `RETURN.md` and lean machine-readable evidence that bind source SHA, original witness SHA256, copy SHA256, commands, isolated target, exit status, both seeds/modes/signs, raw log encoding/path, causal chain, alternatives considered, exact likely surfaces, and remaining unknowns. Failure output must be preserved as failure. Escalate any witness drift, inability to isolate the target, compilation failure, source mismatch, or needed scope expansion.
