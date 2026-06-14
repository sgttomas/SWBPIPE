---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C4-CHECKRUN-001
timestamp: 2026-06-13T22:00:00-0600
completed: 2026-06-13T22:45:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/core/rules/rule_check_runner
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-CHECKRUN-001 — end-to-end rule-check orchestration backend (C4 slice 1)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C4** ("End-to-end rule checks on
  authored models"), backend slice — the first of C4's slices toward the
  PRD §22.4 / R3 exit criterion ("user defines a private non-code rule pack
  and runs checks; software blocks pass/fail on missing inputs"). Selected as
  the earliest unblocked item on the Phase C dependency spine
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C4 row; `_COORDINATION.md`
  Application Integration And Issuance Loop step 3.1).
- Engines wired (previously built and tested but **orphaned** — not reachable
  from any Tauri command): `expression_evaluator` (DEL-06-02),
  `completeness_checker` (DEL-06-03), `rule_pack_document` codec (DEL-06-01).
- D-02b status at execution: AWAITING_RULING. It gates only a *writable
  expression text syntax*; it does **not** gate running checks on the
  already-authored declarative AST, so it did not block this slice.
- No human decision was taken. The two composition semantics below were
  **explicitly routed to C4 as wiring scope** by the DEL-06-01
  `WORKING_ITEMS_RUN_2026-06-12_TP-C2-SCHEMA-001.md` run record (§"Residuals
  and hand-offs"); the schema deliberately left them open.

## Design decisions defined by this slice (authority-grounded)

1. **Acceptability composition.** The schema's `acceptability_basis` is a
   provenance label, not an operator, and `CheckDefinition` carries no
   comparison member (`schemas/rule_pack.schema.yaml`). Per the C4 routing in
   the TP-C2-SCHEMA-001 run record and the PRD §12.5 ratio-vs-allowable shape,
   the runner: (a) uses a **boolean** formula result directly as the predicate;
   (b) for a **quantity** formula result (the demo's ratio), synthesizes
   `Compare(formula, <=, limit)` and evaluates it through the *same*
   `expression_evaluator` so unit/dimension safety is enforced in one place.
   Boolean `true → USER_RULE_CHECKED`, `false → USER_RULE_FAILED`; any
   evaluator/decoder finding keeps the status at `RULE_INPUTS_INCOMPLETE`.
   `<=` is the labelled v1 relation (correct for the demo `ratio_limit` slot
   and ratio/allowable checks); `>=`-style limits need a future additive
   `acceptability_relation` member — a separate decision, **not built here**.
2. **Solver-result binding.** No schema selector ties a `solver_result` input
   to a solved result row (the load-case/result mapping is itself routed to
   C4). The runner accepts a **caller-supplied** `input_id → result_id`
   selector plus the solved envelope, reading `results[].value`/`unit` by
   `ResultItem.id`. An unmatched selector leaves the input unsupplied so
   completeness blocks (`RULE_INPUTS_INCOMPLETE`) — never a silent pass.

## Changes

### Created — `core/rules/rule_check_runner/` (new crate `open_pipe_stress_rule_check_runner`)

- `Cargo.toml`: deps `expression_evaluator`, `completeness_checker`,
  `rule_pack_document`, serde/serde_json. No `product_physics` dep — the runner
  is solver-agnostic (takes already-resolved solver values).
- `src/lib.rs`: `run_rule_checks(&RuleCheckRunInput) -> RuleCheckRunResult`.
  Per check (document order): index `required_inputs`/`value_slots`/
  `formula_declarations`; build completeness inputs (supplied via
  `SuppliedInput::private_reviewed`, unsupplied left out so completeness
  blocks); `check_completeness`; on ready, `decode_expression` the formula AST,
  build `VariableBinding`s, `evaluate`; resolve the value-slot limit; synthesize
  the `<=` comparison and evaluate; map boolean to the per-check status; enforce
  the check's declared `result_statuses` envelope; aggregate worst-of
  (`USER_RULE_FAILED` > `RULE_INPUTS_INCOMPLETE` > `USER_RULE_CHECKED`). Output
  is JSON-serializable per-check outcomes (computed value, limit, bound inputs,
  findings, diagnostic codes) + aggregate + boundary notice. `AnalysisStatus`
  re-exported for consumers.
- `tests/invented_demo_run.rs`: end-to-end over
  `examples/rule_packs/invented_demo.yaml`.

### Modified — desktop backend

- `apps/desktop/src-tauri/Cargo.toml`: added the runner crate dep.
- `apps/desktop/src-tauri/src/lib.rs`: new `run_rule_checks` Tauri command
  (validates the pack first; obtains the solved envelope from a supplied
  `solved_envelope` or `run_preview_mechanics`; resolves the caller-supplied
  selectors against the envelope's `results[]`; calls the runner) plus two
  parse helpers; registered in `generate_handler!` after
  `delete_local_rule_pack`. Embedded command tests added.

## Evidence

- `cargo test --manifest-path core/rules/rule_check_runner/Cargo.toml`:
  **9 unit + 3 integration** pass (pass→`USER_RULE_CHECKED`,
  fail→`USER_RULE_FAILED`, missing solver input→`RULE_INPUTS_INCOMPLETE` with
  the formula not evaluated, missing/unit-mismatched limit→blocked,
  status-outside-declared downgrade, worst-of aggregate, `private_library_value`
  not silently passed, status-vocabulary-only).
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`: **48 pass**
  (43 prior + 5 new `run_rule_checks` command tests: passing/failing/missing/
  blocking-invalid-pack/status-vocabulary-only). No regression.
- Regression (no breakage from the new dependents): `expression_evaluator`
  32 pass, `completeness_checker` 12 pass, `rule_pack_document` 10 pass.
- `cargo fmt --check` clean on both touched crates.
- No UI/TS change in this slice → no Vitest/Playwright/SMOKE.md entry required
  (deliberate evidence posture; UI evidence lands with `TP-C4-CHECKGUI-001`).
- Five-surface evidence sweep (DEC-025) run at the committed HEAD; summary
  committed alongside.

## Residuals and hand-offs

- **`TP-C4-CHECKGUI-001`** — GUI "Run checks" action + per-check results panel
  (engine→GUI-true; PRD §14.5). The R3 exit criterion is GUI-true, so C4 is not
  complete until this lands.
- **C3 residual** — `private_library_value`-sourced input resolution
  (rule-pack ↔ library reference wiring). The runner treats it as unsupplied +
  a recorded note today.
- **Solve-envelope wiring** — driving `aggregate_status` into
  `MechanicsEnvelope.status.rule_check` (still hard-coded at
  `core/product_physics/src/lib.rs:492`) and emitting a `result_export`
  `UserRuleCheck` set (the demo's own `OD-DEL-06-05-002` is `deferred`).
- **Additive schema decision-candidates** — a per-check `acceptability_relation`
  (beyond `<=`) and an explicit solver-result-selector / load-case-mapping
  member on `CheckDefinition` (companions to the TP-C2-SCHEMA-001 residuals).

## Boundary compliance

Status-vocabulary-only (the three automatic rule-check statuses; no
compliance/certification/sealing/authentication/approval/code-compliance
language — asserted by tests). Pure in-memory orchestration; no network,
filesystem, process, or plugin surface; the only file read is the invented
example pack in tests. Invented fixtures only; no protected content; no schema,
example-pack, or `product_physics` change. Git/test evidence is source-control
hygiene, not a lifecycle, release, professional, or code-compliance claim.

## Open decisions awaiting human ruling

- **D-02b** — writable rule-expression text syntax (`AWAITING_RULING`); does
  not block this slice or C4 (checks run on the decoded AST).
