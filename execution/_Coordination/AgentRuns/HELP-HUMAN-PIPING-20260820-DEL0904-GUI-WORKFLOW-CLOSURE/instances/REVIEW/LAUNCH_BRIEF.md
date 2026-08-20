# Sealed TASK Brief — N1 Integrated Review

- RequestedBy: WORKING_ITEMS `WI-PKG09-DEL0904-GUI-CLOSURE`
- RunID: `HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE`
- ParentInstanceID: `WI-PKG09-DEL0904-GUI-CLOSURE`
- ChildInstanceID: `TASK-DEL0904-GUI-REVIEW-01`
- Role: fresh TASK Agent 2, read-only
- TaskSkill: `software-code-review`
- Model: inherited runtime model
- PackageID: `PKG-09`
- ScopePath: `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton`
- PROFILE_PATH: `projects/chirality-piping/software-workflow.json`
- ApplyEdits: false
- ImplementationBrief: `../IMPLEMENTATION/LAUNCH_BRIEF.md`
- AcceptedBasis: Git `57803893d1eb161f395e0574c256dd27920bf1d4`; live validation-manual GUI checklist; DEL-09-04 first Remaining item; Agent-0 review clarification
- DiffBasis: `FROZEN_DIFF_MANIFEST.md`, nine SHA-256-bound implementation files, compared with base `57803893d1eb161f395e0574c256dd27920bf1d4`
- VerificationEvidence: `../IMPLEMENTATION/REGISTERED_CHECKS.json` and TASK run record named in the frozen manifest
- AllowedWriteTargets: none; runtime coordination may persist only this instance's `RETURN.md` and `STATUS.json`
- Objective: independently review 100% of the frozen integrated N1 diff for correctness, fixture integrity, visible-workflow semantics, viewport coverage, test robustness, manual/status claim calibration, authority preservation, scope containment, and verification sufficiency.
- Required checks:
  1. Recompute and verify all nine frozen hashes before and after review.
  2. Review every changed line/content against the base, including untracked records.
  3. Trace every new Playwright selector and expectation to live desktop behavior and invented fixture structure; look for false positives, project-specific fragility, hidden direct state manipulation, network gaps, and assertion ordering defects.
  4. Confirm the command actually ran both registered projects and the evidence supports 2/2 PASS.
  5. Confirm the manual moves only GUI evidence to `DRAFT_EVIDENCE`, no case page to `MAINTAINER_REVIEWED`, and no threshold/release/professional-reliance decision is implied.
  6. Confirm `_STATUS.md` removes only the GUI-workflow clause and preserves lifecycle plus owner-gated residuals.
  7. Confirm changed-path containment and identify any missing relevant check.
- EXCLUSIONS: no edits, repairs, lifecycle acceptance, release/publication judgment, owner decisions, Git operations, network, delegation, or new scope.
- ExpectedReturn: terminal `PASS` only if zero actionable finding remains; otherwise `FAIL` with each blocking/non-blocking actionable finding, exact location, impact, evidence, and remediation; include 100% coverage statement, hash verification, residual risk, and manager fan-in validity.
