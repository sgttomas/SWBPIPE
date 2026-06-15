---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREF-001
timestamp: 2026-06-14T20:40:00-0600
completed: 2026-06-14T21:20:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-SOLVERREF-001 — additive `solver_result_ref` schema member + authored-ref resolution (C4 solver-result-selector, backend slice)

## Tranche and authority basis

- Tranche: completion-plan **C4** named remaining scope (non-GUI) — "the future
  additive `solver-result-selector` schema member (a selector tying a
  `solver_result` input to a solved result row, retiring the caller-supplied
  binding; additive schema work, likely ratification-gated)". This is the
  **backend-resolution slice** (schema member + run-time resolution); GUI
  authoring and a run-panel picker are named follow-ups.
- Selection: the earliest unblocked item on the R3/Phase C dependency spine. C1
  is landed; C2/C3 *named* residuals are closed; C4's other half
  (`acceptability_relation`) landed as `TP-C4-ACCEPTREL-001`; the two R3-exit
  blocking residuals (F-4 packaged human journey; A3 authoring-journey
  usability) are human-gated and not agent-closable. This is the one
  genuinely-unblocked current-stage spine item: an additive,
  backward-compatible schema member built as a PROPOSAL following the
  established additive-member precedent (`library_value_ref` added by
  `TP-C3C4-LIBREF-001`, ratified by `DEC-038`).
- Problem: a `solver_result` required input's binding to a solved result row was
  **caller-supplied only**. The runner module doc said it plainly: "The schema
  carries no selector tying a `solver_result` required input to a solved result
  row, so the binding is caller-supplied." The desktop command resolved a
  run-time `[{input_id, result_id}]` selector against the solved envelope's
  `results[]`; the rule pack itself could not declare which result row each
  solver input reads. That selector is exactly the kind of authored, in-pack
  reference `library_value_ref` already established for private-library values.

## Design decision (additive schema member — awaiting human ratification)

`RequiredInput` is `additionalProperties: false`, so naming the result row
requires a new member. Added an **optional, additive** `solver_result_ref`
(object `{result_id}`, `additionalProperties: false`) to `RequiredInput` in
`schemas/rule_pack.schema.yaml`, mirroring `library_value_ref` exactly. It is
**labelled PROPOSAL** and surfaced here for human ratification (companion to
`DEC-038`); ratification will move the rule-pack `schema_version` 0.3.0 → 0.4.0
(or a later minor, after the `acceptability_relation` ratification) under the
`DEC-033` additive-minor policy — `RequiredInput` is `additionalProperties:
false`, so a strict validator predating the member rejects a pack that uses it
(the textbook minor-bump signal).

Decisions inside the slice, all surfaced (no silent choices):

- **Reuses the existing result-row addressing key.** `result_id` is the same
  stable, deterministic envelope id the caller-supplied selector already used
  (e.g. `result:stress:demo`, `result:disp:node-N-130:ux`); the `Id` pattern
  `^[A-Za-z][A-Za-z0-9_.:-]*$` admits the colon-delimited form, so the authored
  reference needs no new addressing model. No richer node/component/load-case
  selector is invented in this slice (a future candidate if authoring needs it).
- **Authored reference is canonical; no run-time override.** A `solver_result`
  input that carries a `solver_result_ref` is governed by it alone: the
  caller-supplied selector for that input is dropped. This matches the
  `library_value_ref` ruling ("run-time override of the authored reference was
  deliberately deferred behind a human design ruling — a non-goal, not a
  residual").
- **Unresolvable reference blocks — never a silent pass, never a caller
  rescue.** If the referenced row is absent, or lacks a numeric `value` / string
  `unit`, the input stays unsupplied and the check blocks at
  `RULE_INPUTS_INCOMPLETE`. An input that carries a reference blocks even when a
  caller-supplied selector for the same input *would* resolve (no-silent-defaults,
  CONTRACT; authored-canonical).
- **Backward compatible.** A pack with no `solver_result_ref` behaves exactly as
  before — every solver value still comes from the caller-supplied selectors. The
  member is not in `RequiredInput.required`, so every pre-member pack stays valid.
- **Runner unchanged / stays pure.** Resolution lives in the desktop command
  (the impure caller), exactly as the library path resolves from the local
  store. The `rule_check_runner` crate still consumes pre-resolved
  `SolverResultBinding`s and was not touched.

## Changes

### Schema — `schemas/rule_pack.schema.yaml`
- `RequiredInput.properties.solver_result_ref` (optional; `$ref SolverResultRef`).
- New `$defs.SolverResultRef` — `additionalProperties: false`, required
  `{result_id: Id}`, with a description of the addressing key, the authored-vs-
  caller precedence, the unresolvable→block rule, and the "only meaningful when
  source_kind is solver_result" constraint (parallel to `LibraryValueRef`). Not
  added to `RequiredInput.required` (backward compatible).

### Desktop command — `apps/desktop/src-tauri/src/lib.rs`
- New `solver_result_row_value(envelope, result_id) -> Option<(f64, String)>`:
  the shared envelope `results[]`-by-id lookup, reused by both binding paths so
  they address rows identically. `resolve_solver_result_bindings`
  (caller-supplied selectors) refactored onto it (behaviour unchanged).
- New `resolve_authored_solver_result_bindings(document, envelope) ->
  (Vec<SolverResultBinding>, HashSet<String>)`: resolves each `solver_result`
  input that carries a `solver_result_ref` against the envelope; returns the
  resolved bindings plus the set of input ids that carry a reference (resolvable
  or not), so the caller fallback can exclude them (authored-canonical).
- `run_rule_checks_core`: resolve authored references first, then extend with
  caller-supplied selectors **only for inputs without an authored reference**.
  `import std::collections::HashSet` added.

## Evidence

- `cargo test` (from `apps/desktop/src-tauri`): **61 pass** (57 baseline + 4
  new). New: `resolve_authored_solver_result_bindings_resolves_ref_and_records_input_id`
  and `…_records_input_id_but_omits_unresolvable_binding` (helper-level: ref
  resolves to a binding and records the input id; unresolvable ref records the id
  but yields no binding); `run_rule_checks_core_authored_solver_result_ref_resolves_without_caller_selector`
  (end-to-end pass with NO caller selector — actual 50 / limit 100 = 0.5 ≤ 1.0 →
  `USER_RULE_CHECKED`, and `bound_inputs[demo_actual_quantity].result_id ==
  result:stress:demo`); `run_rule_checks_core_authored_solver_result_ref_unresolvable_blocks_over_caller_selector`
  (authored ref to a missing row blocks at `RULE_INPUTS_INCOMPLETE` even though a
  caller selector for the same input would resolve — authored-canonical, no
  rescue). The existing caller-supplied-selector tests are unchanged → backward
  compatibility witnessed.
- `cargo fmt --check` (src-tauri): clean.
- `python3 -m pytest tests/test_rule_pack_schema.py`: **5 pass** (schema valid
  JSON with the additive member; demo/example fixtures conform — they omit it).
- `cargo test` (rule_check_runner crate): **18 pass** — unchanged (the runner
  was not touched; pure-binding consumer confirmed stable).
- Frontend unaffected (no TypeScript changed; the GUI does not yet author or
  consume `solver_result_ref`). No Playwright/Vitest extension is owed under the
  H4 evidence posture — there is no user-visible behaviour change in this slice.
- Five-surface DEC-025 sweep: **overall pass** —
  `validation/evidence/sweeps/SWEEP_20260615T033429Z_c161a7053cb5-dirty.json`
  (cargo 32 crates, pytest 359, desktop Vitest 367, Playwright e2e 10/10,
  desktop production build — all five surfaces pass). The summary binds to
  `c161a7053` because a concurrent monorepo agent committed external-scope
  `tools/retrieval` work on top of this tranche's `ad27dc4d7` during the first
  sweep run; `ad27dc4d7` is an ancestor of `c161a7053`, so the summary validates
  a superset that includes this change (the `c161a7053` delta is outside
  `chirality-piping`). The first sweep run was marked `fail` solely on a
  Playwright worker-teardown timeout ("worker did not exit within 300000ms after
  stop") — all 10 e2e tests passed; no test or code failure, an environmental
  flake under concurrent load — and was green on a clean re-run with no lingering
  processes. This tranche's `ad27dc4d7` was already on `origin/main` (carried by
  the concurrent agent's push); this evidence commit adds the green summary.

## Residuals and hand-offs

- **Schema ratification** — `solver_result_ref` is a PROPOSAL additive member
  awaiting a human `DEC` (companion to `DEC-038`); ratification bumps the
  rule-pack `schema_version` under the `DEC-033` additive-minor policy (the bump
  rides ratification, sequenced after the pending `acceptability_relation`
  ratification).
- **GUI authoring** — authoring `solver_result_ref` in the C2 declarations
  form-builder (a result-row picker for `solver_result` inputs) is the next
  slice, mirroring `TP-C3-LIBREFAUTHOR-001`. Until then the member is
  hand-authorable / runner-resolvable only.
- **Run-panel resolution preview** — a read-only "does this authored reference
  resolve in the solved envelope?" preview, mirroring `TP-C3-LIBREFPICKER-001`,
  is a later slice.
- **Richer result addressing** — a structured node/component/load-case selector
  (vs. the raw `result_id`) is a future candidate only if authoring ergonomics
  need it; not pursued now.
- **Headless runner** — `core/runner/headless` performs no solver-result binding
  resolution today, so it needs no authored-ref support in this slice.

## Boundary compliance

Local-only (in-memory resolution + invented test fixtures; no
network/daemon/telemetry/private-data writes). The authored reference carries
only a result-row id, never an embedded solver value (resolved at run time).
Status-vocabulary-only (`RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` /
`USER_RULE_FAILED`); no compliance/certification/sealing/authentication/approval/
code-compliance or professional-acceptance claim. Deliverables stay `CHECKING`.
Git/test evidence is source-control hygiene only, not lifecycle issuance.

## Open decisions awaiting human ruling

- **Schema ratification** of the additive `solver_result_ref` member (PROPOSAL;
  companion to `DEC-038`), and of the still-pending `acceptability_relation`
  member from `TP-C4-ACCEPTREL-001`.
- Pre-existing, unrelated: **D-06**, **D-10b**, **D-04b**, **D-05b**, **D-07b**,
  **D-11**, **D-12** remain `NOT_PREPARED` in the decision register. The two
  R3-exit blocking residuals (F-4 packaged human journey; A3 authoring-journey
  usability) remain human-gated.
