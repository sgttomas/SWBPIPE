---
run-id: TASK_RUN_diagnosis_2026-09-08_0502
timestamp: 2026-09-08T05:02:16Z
run-status: SUCCESS
control-surface: FILE
scope-path: /Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/diagnosis
task-profile: NONE
task-skill: software-defect-diagnosis
resolved-skill-path: /Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908/skills/software-defect-diagnosis
resolved-skill-version: "1"
resolved-task-profile-requirement: NONE
companion-files:
  - BRIEF_SCHEMA.md (found)
  - TOOL_POLICY.md (found)
  - QA_CHECKS.md (found)
allowed-tools:
  - "python3 tools/software_workflow/discover_repository.py:*"
  - "python3 tools/software_workflow/select_affected_checks.py:*"
  - "python3 tools/software_workflow/run_registered_checks.py:*"
write-authorization: ALLOWED_WRITE_TARGETS
runtime-overrides: {}
---

## Requested Tasks

- Reproduce and diagnose the unchanged M1-N-008 coupled-derived-normal witness against source `779dedb8670625b36af07b89fc5557470e47c50e`.
- Record original witness bytes and hashes before copying.
- Run both existing seeds and both existing modes in an isolated temporary copy with a unique `CARGO_TARGET_DIR`.
- In a separate temporary-only diagnostic extension, mirror the fixture for both applied-force signs and run both seeds and modes.
- Preserve raw stdout and stderr losslessly and identify the current-normal discrepancy, earliest divergent state, minimal repair surface, and verification recommendations.

## Expected Outputs

- `RETURN.md` in the D1 coordination directory.
- Lean machine-readable evidence binding source SHA, witness/copy hashes, commands, isolated target, exit statuses, cases, raw log encoding/paths, diagnosis, alternatives, surfaces, and unknowns.

## Tools Used

- `python3 tools/software_workflow/discover_repository.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- Rust/Cargo and deterministic local shell probes expressly declared by the sealed brief

## Tool Policy Compliance

PASS. Repository discovery and affected-check selection used the skill tools; the narrower witness reproduction used the sealed brief's express Rust/Cargo and deterministic-shell authorization. No network or broad suite was used.

## Write Authorization

ALLOWED_WRITE_TARGETS: D1 coordination subtree, diagnosis subtree, and a freshly allocated system temporary directory removed after evidence capture.

## Outputs Produced

- D1 `RETURN.md`
- `evidence/diagnosis.json`
- Lossless raw replay/extension stdout and stderr, exit-status files, environment record, extension diff, and pre/copy/post hash manifests

## Missing

none

## Needs Human Ruling

- Selection of the repair algorithm and any required nonzero force/current-normal closure policy remains outside this diagnostic authority.

## Dependency Notes

- Evidence-stage only; no PROJECT_SETUP disposition was assumed and no dependency truth was mutated.

## Applied Changes

- Added only authorized D1 return and diagnosis/run-record evidence.
- Removed the freshly allocated `/tmp/chirality-f4-d1.EflygK` workspace and both unique Cargo targets after evidence capture.

## Proposed Changes

- In `core/solver/nonlinear_integration/src/lib.rs`, prevent state-only convergence while the applied sliding force is inconsistent with the returned current normal, or solve the fixed sliding relation simultaneously.
- Retain `core/solver/nonlinear_supports/src/lib.rs` unless the selected algorithm requires a change to its Sliding anti-chatter rule.
