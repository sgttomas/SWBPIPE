# Sealed Launch Brief — CHANGE-CLOSEOUT

- RequestedBy: `HELP_HUMAN`
- RunID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- ParentInstanceID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- ChildInstanceID: `CHANGE-CLOSEOUT`
- Role: `CHANGE` (Agent 1)
- Objective: perform routine scoped Git closeout for the single DEC-093
  engineering node, then remain available for the later fan-in receipt commit,
  push, and PR after committed-HEAD proof gates pass.
- AcceptedBasis: validated WORKING_ITEMS return; TASK-REVIEW-005 PASS; focused
  96/96; full Piping pytest 597/597; practitioner harness 349/349; repository
  self-check PASS; `git diff --check` PASS.
- CurrentBranch: `codex/piping-dec093-ci-surface4`
- BaseSHA: `219f695d348f1d83ba904ef4dd38781636b423a6`
- Phase1AllowedActions: inspect state; verify only tranche paths are dirty;
  stage all current tracked/untracked paths under `projects/chirality-piping/**`
  belonging to the node and this run; commit once with message
  `feat(piping): bind DEC-025 surface 4 to CI`; report SHA and clean state.
- Phase1Exclusions: no receipt write; no evidence-sweep execution; no push;
  no PR; no merge; no reset/rebase/amend/cleanup; no file-content edits except
  CHANGE-owned status/return records under this instance if needed.
- Phase2: only on explicit follow-up from HELP_HUMAN after proof and fan-in;
  stage the receipt/handoff closeout, commit, fetch/verify upstream, push the
  branch, and open one PR if the available authenticated Git interface permits.
- Escalation: stop on any unrelated dirty path, base divergence, in-progress
  Git operation, scope ambiguity, failed staging containment, or non-fast-
  forward push condition.
- ModelAttribution: inherited GPT-5-based Codex runtime; no substitution
  planned.
