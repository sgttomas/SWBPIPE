---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C3C4-LIBREF-001
timestamp: 2026-06-14T01:00:00-0600
completed: 2026-06-14T02:05:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3C4-LIBREF-001 — rule-pack ↔ library reference resolution (C3/C4 coupling, backend slice)

## Tranche and authority basis

- Tranche: the coupled completion-plan residuals **C3** ("rule-pack ↔ library
  reference wiring (couples C2/C3; its own slice)") and **C4**
  ("`private_library_value`-sourced input resolution (couples the C3
  residual)"). The human directed "Proceed to … the C3↔C4 library-reference
  wiring slice." Scoped as a **backend-resolution slice** (mirroring the C4
  backend/GUI split): the reference mechanism + run-time resolution land here;
  authoring the reference in the C2 form-builder and a richer C4 library picker
  are named follow-ups.
- Problem: a rule-pack required input can declare `source_kind:
  private_library_value`, but the schema had **no member** to say *which*
  library value it references, so the C4 runner treated such inputs as
  unsupplied with a deferred note. This slice defines the reference and resolves
  it from the local private-library store (C3, store v11).
- D-02b status: AWAITING_RULING; unrelated (it gates writable expression text
  syntax, not library references).

## Design decision (additive schema member — awaiting human ratification)

`RequiredInput` is `additionalProperties: false`, so referencing a library value
requires a new member. Added an **optional, additive** `library_value_ref`
(`LibraryValueRef` $def: `library_kind` ∈ {material, section, component},
`library_id`, `record_id`, `slot_id`) to `RequiredInput` in
`schemas/rule_pack.schema.yaml`. This follows the established precedent for
bounded additive rule-pack-schema members (`TP-C2-SCHEMA-001`, ratified by
`DEC-031`) and the additive-minor policy (`DEC-033`). It is **labelled
PROPOSAL** and is surfaced here for human ratification; it changes nothing for
packs that do not use it (optional; the hand-rolled validator is Value-based and
ignores it; the JSON-Schema conformance test passes because the demo pack omits
it).

**IP boundary (decisive):** the rule pack carries the *reference only*; the
private library value is read at run time from the local store and **never
embedded** in the rule-pack document, so a rule pack (private-by-default, but
potentially shared) never redistributes a private library value (PRD §13/§17.3,
IP_AND_DATA_BOUNDARY).

## Changes

### Schema — `schemas/rule_pack.schema.yaml`
- `RequiredInput.properties.library_value_ref` (optional) + new `LibraryValueRef`
  definition.

### Runner — `core/rules/rule_check_runner/src/lib.rs`
- New `LibraryValueBinding { input_id, value, unit, library_kind, library_id,
  record_id, slot_id }` + `RuleCheckRunInput.library_values`.
- `private_library_value` resolution: when a matching `LibraryValueBinding`
  exists, bind `{value, unit}` (dimension comes from the input's
  `quantity_intent`, as for solver bindings) with a provenance note citing the
  library reference; otherwise stay unsupplied with a "no resolved library
  reference" note (never a silent pass). Doc comment updated.

### Desktop command — `apps/desktop/src-tauri/src/lib.rs`
- Refactored the `run_rule_checks` Tauri command into a store-free
  `run_rule_checks_core(...)` (now also takes `library_value_bindings`) plus a
  thin `#[tauri::command]` wrapper that takes `app: AppHandle` + `project_id`,
  resolves library references from the local store, and calls the core.
- `resolve_rule_check_project_id` (explicit `project_id` → model `/project/id` →
  envelope `/model_ref`); `resolve_library_value_bindings` /
  `…_with_connection` (open store; per `private_library_value` input with a
  `library_value_ref`, `load_library` and `extract_library_slot_value`);
  `extract_library_slot_value` (**material allowable slots** — `material_records`
  → `allowables` → `value.magnitude`/`value.unit_ref.ref_id`; section/component
  is a named follow-up); `parse_library_value_bindings`. Unresolvable references
  are omitted, so the input blocks. Existing command tests retargeted to
  `run_rule_checks_core(..., None)` (behaviour unchanged for the demo pack).

### Frontend — `apps/desktop/src/services/ruleCheckService.ts`, `features/rule-check/RuleCheckRunPanel.tsx`
- Service: `runRuleChecks` accepts `projectId` and forwards it; `LibraryValueRef`
  type; `deriveRuleCheckBindingPlan` surfaces each library input's
  `library_value_ref`.
- Panel: passes `projectId` (`model.project.id`); the library-input row now
  shows the reference (`kind:id → record → slot`) with an honest "resolves from
  the local private library store at run time (desktop); the value stays in the
  library and is never embedded in the rule pack" note (replacing the deferred
  note). The run result's `bound_inputs` show whether each library input
  resolved.

## Evidence

- `cargo test -p open_pipe_stress_rule_check_runner`: **11 unit + 3 integration
  pass** (+2: library binding resolves→pass, failing library value→fail).
- `cargo test` (`apps/desktop/src-tauri`): **53 pass** (+5: `extract_library_slot_value`
  material/none; `resolve_library_value_bindings_with_connection` with an
  in-memory store; no-stored-library→no-binding; `run_rule_checks_core` threads a
  library binding→`USER_RULE_CHECKED`; no binding→`RULE_INPUTS_INCOMPLETE`).
- `cargo fmt --check` clean (both touched crates).
- `pytest tests/test_rule_pack_schema.py`: **5 pass** (schema valid + demo pack
  conforms with the additive member present).
- `npm test --workspace apps/desktop` (Vitest): **340 pass** (+1 panel
  library-reference render; service tests extended for `projectId` +
  `library_value_ref` in the plan).
- `npm run build --workspace apps/desktop`: clean.
- `npx playwright test`: **10/10** (two viewports; the run-checks browser-seam
  spec is unaffected — the demo pack has no library inputs).
- Five-surface DEC-025 sweep: see the committed sweep summary.

## Residuals and hand-offs

- **C2 authoring** — author `library_value_ref` in the rule-pack declarations
  form-builder (today it is set via the raw document JSON).
- **Section/component slot resolution** — `extract_library_slot_value` resolves
  material allowable slots only; section/component value slots are a follow-up.
- **Richer C4 GUI** — a library/record/slot picker in the run panel (today the
  reference is authored in the pack; the panel surfaces it read-only).
- **Schema ratification** — `library_value_ref` is a PROPOSAL additive member
  awaiting a human `DEC` ratification (companion to `DEC-031`).

## Boundary compliance

Local-only (Tauri command + local SQLite store + invented test fixtures; no
network/daemon/telemetry). Private library values are resolved at run time and
never embedded in the rule pack or committed. Status-vocabulary-only; no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim. Deliverables stay `CHECKING`. Git/test evidence
is source-control hygiene only.

## Open decisions awaiting human ruling

- **Schema ratification** of the additive `library_value_ref` member (PROPOSAL).
- **D-02b** — writable rule-expression text syntax (`AWAITING_RULING`);
  unrelated to this slice.
