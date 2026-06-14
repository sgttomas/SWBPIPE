---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFSECCOMP-001
timestamp: 2026-06-14T09:30:00-0600
completed: 2026-06-14T10:25:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-LIBREFSECCOMP-001 — section/component library-reference resolution (C3 residual)

## Tranche and authority basis

- Tranche: the completion-plan **C3** residual **"section/component slot
  resolution"** — the first-listed residual carried by both
  `TP-C3C4-LIBREF-001` (run-time resolution half) and `TP-C3-LIBREFAUTHOR-001`
  (authoring half). It is an unblocked item on the current R3/Phase C dependency
  spine, so it is pre-approved for execution under the Application Integration
  And Issuance Loop (`_COORDINATION.md` step 3.1).
- Problem: a rule-pack `private_library_value` required input can already
  **author** a `library_value_ref` pointing at any of the three library kinds
  (`material`/`section`/`component` — the `DeclarationsEditor` selector landed in
  `TP-C3-LIBREFAUTHOR-001`), but the run-time resolver
  `extract_library_slot_value` resolved **material allowable slots only**. A
  section- or component-kind reference therefore never resolved and the input
  silently blocked, with no path to pass/fail from a user-entered section
  dimension/property or component field value.
- Scope: backend resolver only (single file:
  `apps/desktop/src-tauri/src/lib.rs`). No schema change, no runner change
  (`LibraryValueBinding` is kind-agnostic), no frontend change (authoring and the
  read-only run-panel reference display already exist; the store layer
  `load_library`/`upsert_library` is already keyed by `(project_id,
  library_kind, library_id)`).
- D-02b status: AWAITING_RULING; unrelated (it gates writable expression text
  syntax, not library references).

## Design decision (schema-grounded; no invention)

The resolver now dispatches by `library_kind` to the record/slot/value shapes
defined in the authoritative library schemas — read verbatim from the schemas,
nothing invented:

| kind | record array / id (`schemas/*.schema.yaml`) | slot array(s) / id | value unit field |
|---|---|---|---|
| material | `material_records` / `material_id` | `allowables` / `allowable_id` | `value.unit_ref.ref_id` (a unit reference) |
| section | `section_records` / `section_id` | `dimensions` / `dimension_id`, `properties` / `property_id` | `value.unit` (plain string) |
| component | `component_records` / `component_id` | `fields` / `field_id` | `value.unit` (plain string) |

Material `QuantityValue` carries `unit_ref` + `dimension_id`; section and
component `QuantityValue` carry a plain `unit` string + `dimension`. The two
unit conventions are mutually exclusive per the schemas (`additionalProperties:
false`), so the resolver reads `value.unit_ref.ref_id` first and falls back to
`value.unit`. Material behaviour is byte-for-byte unchanged (same record/slot
keys, same `unit_ref.ref_id` read, same number-or-string magnitude parse).

**IP boundary (unchanged, decisive):** the rule pack still carries the
*reference only*; every kind's private value is read at run time from the local
store and is **never embedded** in the rule-pack document. Unresolvable
references (unknown kind, missing record/slot/value, non-numeric magnitude) are
omitted, so the input blocks — never a silent pass (CONTRACT no-silent-defaults).

## Changes

### Desktop command — `apps/desktop/src-tauri/src/lib.rs` (only)
- `extract_library_slot_value` refactored from a material-only body into a
  `match library_kind` that dispatches material/section/component to the schema
  shapes above (unknown kind → `None`). Doc comment rewritten to document all
  three kinds and the two unit conventions.
- New helper `find_library_slot_value(document, records_member, record_id_key,
  record_id, &[(slot_array, slot_id_key)], slot_id) -> Option<&Value>`: finds the
  record by id, then the first matching slot across the listed slot arrays (so a
  section slot id resolves whether it is a dimension or a property), returning its
  `value` object.
- New helper `parse_quantity_magnitude(Option<&Value>) -> Option<f64>` (extracts
  the existing number-or-string magnitude parse, reused across kinds).
- The resolve driver (`resolve_library_value_bindings_with_connection`), the
  runner (`rule_check_runner`), the schema, and the frontend are unchanged.

## Evidence

- `cargo test` (`apps/desktop/src-tauri`): **57 pass** (+4 over the 53 baseline):
  `extract_library_slot_value_reads_section_dimension_and_property`,
  `…reads_component_field`, `…returns_none_for_unknown_kind`, and
  `library_value_ref_resolves_section_from_local_store` (full store path:
  upsert a section library → resolve a `prop:area` reference →
  `{value: 2300.0, unit: "mm2", library_kind: "section"}`). The renamed material
  test `extract_library_slot_value_reads_material_allowable` still asserts the
  unchanged material read and that a section lookup over a material document
  finds nothing.
- `cargo fmt --check` (`apps/desktop/src-tauri`): clean.
- Five-surface DEC-025 sweep: see the committed sweep summary (cargo crate sweep
  re-runs all crates including `rule_check_runner`; pytest, desktop Vitest +
  wasm, Playwright e2e ×2, and the production build are unaffected — the change
  is isolated to the Tauri-only `src-tauri` crate, which is not in the
  frontend/wasm/pytest build graphs).

### UI evidence posture (H4 amendment)
No Playwright spec extension. Library-reference resolution is **desktop-store
only** — the local private-library SQLite store is reachable only through the
Tauri command path, not the browser preview (which has the honest
store-unavailable seam). The behaviour therefore lives at, and is exercised by,
the Rust command/store tests (the same posture `TP-C3C4-LIBREF-001` used for the
material resolution it shipped). No new user-visible UI surface lands here: the
authoring selector (`TP-C3-LIBREFAUTHOR-001`) and the read-only run-panel
reference display (`TP-C3C4-LIBREF-001`) already cover the section/component
kinds; this slice only makes their references resolve at run time.

## Residuals and hand-offs

- **Richer C4 GUI** — a library/record/slot picker in the run panel (today the
  reference is authored in the rule-pack editor; the run panel surfaces it
  read-only). Carried from `TP-C3C4-LIBREF-001`.
- **Schema ratification** — `library_value_ref` remains a PROPOSAL additive
  member awaiting a human `DEC` ratification (companion to `DEC-031`). This slice
  reads the existing member; it adds no schema surface.
- The C3/C4 dependency-spine residual "section/component slot resolution" is now
  **closed**: all three library kinds resolve at run time.

## Boundary compliance

Local-only (Tauri command + local SQLite store + invented test fixtures; no
network/daemon/telemetry). Private library values (all kinds) are resolved at run
time and never embedded in the rule pack or committed. Status-vocabulary-only; no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim. Deliverables stay `CHECKING`. Git/test evidence is
source-control hygiene only.

## Open decisions awaiting human ruling

- **Schema ratification** of the additive `library_value_ref` member (PROPOSAL;
  companion to `DEC-031`).
- **D-02b** — writable rule-expression text syntax (`AWAITING_RULING`);
  unrelated to this slice.
