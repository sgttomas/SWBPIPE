# Sealed TASK Brief — N1 Integrated Review 03

- RequestedBy: WORKING_ITEMS `WI-PKG09-DEL0904-GUI-CLOSURE`
- RunID: `HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE`
- ParentInstanceID: `WI-PKG09-DEL0904-GUI-CLOSURE`
- ChildInstanceID: `TASK-DEL0904-GUI-REVIEW-03`
- Role: fresh TASK Agent 2, read-only
- TaskSkill: `software-code-review`
- ScopePath: DEL-09-04 deliverable folder
- PROFILE_PATH: `projects/chirality-piping/software-workflow.json`
- ApplyEdits: false
- AcceptedBasis: Git `57803893d1eb161f395e0574c256dd27920bf1d4`; V2 product acceptance; CHANGE Phase 1 whitespace failure and exact bounded remediation
- DiffBasis: `FROZEN_DIFF_MANIFEST_V3.md`, 24 SHA-256-bound paths
- VerificationEvidence: implementation/manager check JSON, `REMEDIATION.md`, Review 01/02 returns, and clean tracked-plus-untracked diff-check result
- AllowedWriteTargets: none; runtime coordination may persist only this instance's `RETURN.md` and `STATUS.json`
- Objective: independently review 100% of the V3 integrated diff after EOF normalization. Verify all 24 hashes twice; confirm exactly the identified whitespace class changed in the ten repaired files except for the appended truthful remediation section; confirm no semantic regression, stale hash claim, scope drift, evidence gap, or owner-gate change; independently run/check the complete worktree whitespace gate including untracked files.
- Required verdict: `PASS` only with zero actionable findings. Return exact 24-path coverage, two hash-pass results, whitespace-gate result, semantic-delta assessment, containment, residual risk, and CHANGE fan-in validity.
- EXCLUSIONS: no edits, repairs, acceptance, owner decisions, Git operations, network, or delegation.
