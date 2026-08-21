# Sealed instance launch brief — N1 SCA-009 instruments

- RequestedBy: `HELP_HUMAN`
- ParentInstanceID: `HELP_HUMAN`
- ExecutorInstanceID: `PROJECT-SETUP-SCA009-INSTRUMENTS`
- Role: `PROJECT_SETUP` (Agent 1)
- Node: `N1`
- Model: current `GPT-5.6` Codex runtime; no substitution authorized.
- Basis: branch `codex/piping-sca009-ci-support-20260821` at `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`; SOFTWARE_DECOMP revision 0.12; accepted SCA-009.
- Objective: execute exactly the post-closure mechanical obligations in SCA-009 `Propagation_Plan.md` and `Handoff_State.md`.
- DeclaredReads: root and project AGENTS; PROJECT_SETUP instructions; SCA-009 snapshot; accepted decomposition/registers; current DAG snapshot and validators; dependency-extract schema/tooling; current Reconciliation snapshots/conventions; existing PKG-07 scaffold conventions.
- AllowedWriteTargets: this AgentRuns root; the new DEL-07-09 scaffold under PKG-07; a new immutable DAG rebuild snapshot and its owning `_LATEST.md` pointer; DEL-07-09 `Dependencies.csv` and its directly required human-readable dependency view/run evidence; a new targeted Reconciliation current-authority derivative snapshot and its owning pointer only if convention requires it.
- Acceptance: minimum viable DEL-07-09 scaffold grounded in revision 0.12 and `OPEN`; topology contains `+DEL-07-09`, `+SOW-077`, and exactly three additive DEL-07-09 outgoing edges to DEL-16-01/DEL-07-01/DEL-07-02; canonical schema-v3.1 dependency rows; targeted current-authority refresh covers only DEL-07-09, DEL-07-03, DEL-07-01/02, DEL-16-01 and R-005/R-006 binding; focused validators and `git diff --check` pass; exact containment proven.
- EXCLUSIONS: no product code; no lifecycle beyond scaffold `OPEN`; no estimate/schedule recomputation; no existing-edge mutation; no formal AUDIT_DECOMP unless a clearly applicable in-session runner exists; no shared closeout surfaces; no commit/push/PR; no parked owner holds; no `artifact-proof` label.
- Return: files, exact checks, authoritative-vs-derivative status, named parked audit, blockers, and fan-in handoff.
