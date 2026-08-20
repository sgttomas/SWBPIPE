# Sealed TASK Brief — N1 Implementation

- RequestedBy: WORKING_ITEMS `WI-PKG09-DEL0904-GUI-CLOSURE`
- RunID: `HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE`
- ParentInstanceID: `WI-PKG09-DEL0904-GUI-CLOSURE`
- ChildInstanceID: `TASK-DEL0904-GUI-IMPLEMENT-01`
- Role: TASK Agent 2
- TaskSkill: `software-bounded-implementation`
- Model: inherited runtime model
- PackageID: `PKG-09`
- DeliverableIDs: `DEL-09-04`
- ScopePath: `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton`
- PROFILE_PATH: `projects/chirality-piping/software-workflow.json`
- ApplyEdits: true
- AcceptedBasis: Git `57803893d1eb161f395e0574c256dd27920bf1d4`; live validation-manual index GUI checklist; existing two-viewport Playwright configuration and `gui-workflow-validation.spec.ts`; DEL-09-04 first Remaining item
- Objective: make the existing invented desktop workflow case a complete, fixture-bound GUI validation-manual case across both registered viewport projects, covering missing-data behavior, warning visibility, assumptions/boundaries, solve status, and result-state transitions; update the draft manual inventory and DEL-09-04 residual truthfully after focused evidence passes.
- Dependencies: accepted activation only
- DeclaredReads: root/project instructions already supplied; software workflow profile; DEL-09-04 intake; `docs/validation_manual/**`; desktop Playwright configuration/spec; smallest relevant desktop source/test surfaces and invented fixture data
- AllowedTools: repository-native read/edit commands plus registered `desktop-test` and `desktop-build` checks, Playwright list/focused execution, TypeScript checking, Git diff/status, and software-workflow scope/check selectors
- AllowedWriteTargets:
  - `projects/chirality-piping/apps/desktop/e2e/**`
  - `projects/chirality-piping/apps/desktop/src/**` only when the case demonstrates a bounded product defect
  - `projects/chirality-piping/apps/desktop/tests/**` if live
  - `projects/chirality-piping/docs/validation_manual/**`
  - `projects/chirality-piping/validation/**` only for invented fixtures/test support and evidence required by this node
  - `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`
  - `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/MEMORY.md`
  - `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/**`
  - `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/IMPLEMENTATION/**`
- AcceptanceCriteria:
  1. An invented, repository-local fixture binds every documented GUI workflow-validation check to visible controls and expected states.
  2. The focused Playwright case is discovered once in each of `chromium-desktop` and `chromium-compact` and passes under host capability.
  3. Missing data/warnings stay visible and distinct; solve and result transitions are asserted; assumptions/boundaries remain explicit; no external requests occur.
  4. Manual inventory moves only GUI workflow validation from `PLANNED/TBD` to truthful `DRAFT_EVIDENCE`, cites exact test/evidence, and selects no threshold or review promotion.
  5. DEL-09-04 Remaining removes only the closed GUI-workflow clause; `MAINTAINER_REVIEWED` and public-comparison owner gates remain open; lifecycle stays `IN_PROGRESS`.
  6. Focused/registered checks pass and changed paths are contained.
- EXCLUSIONS: no public benchmark value, case-page promotion, lifecycle/stage/release/publication/professional-reliance act; no root, workflow, register, DAG, decomposition, receipt, other package, or other project write; no network/telemetry/private data.
- ExpectedOutputs: exact changed files and behavior; focused check results; host command/result or exact pending command; containment; residual risks; readiness for manager fan-in.
- Escalation: return scope changes, owner decisions, external defects, or host refusal; do not create another node or delegate.
