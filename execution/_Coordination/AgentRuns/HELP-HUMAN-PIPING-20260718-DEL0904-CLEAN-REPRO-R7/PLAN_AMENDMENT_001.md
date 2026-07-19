# Plan Amendment 001 — Preflight Error Count Correction

**Disposition:** `RECORD`; non-consequential evidence-count correction. Objective, authority, write scope, acceptance criteria, and terminal `BLOCKED` handling are unchanged.

The frozen `ORCHESTRATION_PLAN.md` summarized the manager's exact registered preflight result as four missing project-local Node binaries plus fifteen incomplete locked/offline Cargo probes. The complete list actually contains four Node errors plus **sixteen** Cargo errors, for twenty prerequisite errors total, including `core/runner/headless/Cargo.toml`.

The Agent 2 independent revalidation also returned the exact twenty-error set. The bundle and all terminal records must use that complete revalidated list and surface this correction. The earlier plan remains preserved as the frozen plan version; this amendment prevents silent reinterpretation.

No new blocker class, scope, authority, risk, write target, acceptance criterion, or execution path was introduced. Candidate brief §§3.2, 3.6, and 7 still require terminal `BLOCKED` before the temporary clone, generator, runner, tests, evidence sweep, registered profile checks, or other reproduction execution.
