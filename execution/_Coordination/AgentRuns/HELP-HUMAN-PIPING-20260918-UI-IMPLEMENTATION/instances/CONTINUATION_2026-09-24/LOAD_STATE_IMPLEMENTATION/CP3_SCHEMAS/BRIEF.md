# TASK — 0.4.0 load/reference-state schemas

Parent of record: the session-2 WORKING_ITEMS load-state manager (SendMessage id
`a3675abb28ada0834`). ROOT spawns this TASK on the manager's request. Report to
the manager by SendMessage and send your final report to ROOT. Do not delegate
further. Paths are WORKING_ROOT-relative, and the checkout is the manager's
load-state worktree. `LSI` =
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.

## Objective

Publish JSON Schemas (draft 2020-12, the repository's existing style) for the
connected load/reference-state route, from the frozen wire. You are not the
producer implementer.

1. **New `schemas/load_reference_state.schema.json`.** Freeze this file early.
   A parallel reader TASK consumes its `$defs` once the manager relays your hash.
   - **Authored namespace, closed.** `additionalProperties: false` throughout,
     tagged unions via `oneOf` + `const` discriminants. Define:
     - `ReferenceConfiguration`, `MemberReference`, `ReferenceBasis`, `FitReference`
     - `ExpansionLaw`: four definitions with their data unions
     - `AnalysisState`, `ElementState`, `MaterialSelection`, `ThermalState`,
       `AnalysisBasisOverride`
     - `SupportState`, `Participation` (active, inactive, locked with components
       and position sources), `Motion`, `DeviceReference`
     - `LoadSource`, `History`
     - `MaterialExpansionLaws`, for the material-owned `expansion_laws` array

     Quantities are `{value: number, unit: string}`. Required and optional fields
     must match `CP2_WIRE.md`. The schema describes only shape. Admission rules
     remain producer diagnostics: coverage, unit dimension, and "parses but
     blocks". Document that distinction in `description` fields.
   - **Raw producer evidence, closed.**
     - `LoadReferenceStateRecord`: exactly the §3 keys of `CP2_WIRE_ADDENDUM_1.md`,
       including member, support-component, contribution and excluded-source item
       shapes, segment `use` enums, and nullable optionals as `null`.
     - `LoadReferenceContractEvidence`: `{pressure, connector, exact_cases, load_reference_states}`.
       - `pressure` reuses the existing physics pressure item shape by `$ref`, if
         you can without changing it.
       - `exact_cases` items are the physics shape with these differences:
         `material_basis` const `resolved_per_member_load_reference_state_v1`;
         `pipe_materials` items with the extra keys `material_selection_kind` and
         `resolved_eigenstrain` (read the actual shape from the frozen raw
         envelopes).
2. **Additive, closed load-reference-1 branches** in the three v0.3 carrier
   schemas:
   - `schemas/results.v0.3.schema.yaml`
   - `schemas/analysis_run.v0.3.schema.json`
   - `schemas/stress_neutral_export.v0.3.schema.json`

   Each branch adds semantic ID `openpipestress.result_semantics/0.3.0/load-reference-1`,
   table sha256 `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d`,
   and profile `resolved_straight_load_state_v1`, with `contract_evidence` →
   `load_reference_state.schema.json#/$defs/LoadReferenceContractEvidence`. It
   forbids `source_block_recovery`. Every existing branch, `$def` and enum member
   keeps its meaning. Mirror how physics-1 is expressed in each file.
   - The reader TASK derives carriers by preserving raw metadata and
     `contract_evidence` verbatim, following the physics-1 pattern.
   - Build carrier-branch tests from the existing physics-1 carrier fixtures,
     transformed. Where those are unavailable, use minimal hand-built instances.
   - The manager will validate the reader TASK's actual carriers against your
     schemas at integration.
3. **Tests in the new `tests/test_load_reference_schema.py`**, using the
   existing `tests/schema_validation.py` helper style:
   - Positive checks:
     - the authored namespace objects inside both frozen requests validate
       (`reference_configurations[]`, each material's `expansion_laws`, each
       case's `analysis_state`);
     - every raw envelope's `contract_evidence` validates as
       `LoadReferenceContractEvidence`.
   - Negative checks, which must be rejected:
     - an unknown key per object level;
     - a missing required key;
     - an unknown discriminant (`kind`/`definition`/`meaning`/history);
     - a fit with both `length_change` and `strain`;
     - a wrong semantic ID, hash or profile in a carrier branch;
     - `source_block_recovery` present under load-reference-1;
     - a physics-1 carrier carrying `load_reference_states`.
   - The existing schema tests must still pass unchanged. Run at least
     `tests/test_results_schema.py`, `tests/test_analysis_run_schema.py`,
     `tests/test_stress_neutral_*.py` and `tests/test_source_block_schema_contract.py`.

## Frozen inputs (read-only; sha256)

| Input | sha256 |
|---|---|
| `LSI/CP2_WIRE.md` | `81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996` |
| `LSI/CP2_WIRE_ADDENDUM_1.md` | `c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760` |
| `fixtures/results/semantic_contract_v0_3_load_reference_1.json` | `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d` |
| `fixtures/product_preview/load_reference/connected.request.json` | `ff0ce8724d7734c2cd8d10ce866085d80741adadc0da529521084dc8382453a2` |
| `fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json` | `6c1dd176e5600bdbec08066345372a26459746ed34d665fa4767569594dd03bf` |
| `fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json` | `333515b39d15e1fc270b40f267d6ff47c5de24acd733ee52660d4488a3c31c11` |
| `fixtures/product_preview/load_reference/pressure.request.json` | `52375ad60d05074ed6064b72d78d9c24d19b1a53915bcc46c2b424be7c39ef69` |
| `fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json` | `20802f97f3e00fe7043d3adf5d78e926270f0e0f9c80f4b724ef4d122fc9ab75` |
| `fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json` | `8215cad29a51e0e9d59f90b5db014256787592cf514b82ccef56fd814ce94be3` |

The typed Rust DTOs are the implementation of the wire:
`core/product_physics/src/case_state/input.rs` (read-only). A mismatch between that
file and the wire documents is a finding to report, not something to paper over.

## Exclusive write boundary

- new `schemas/load_reference_state.schema.json`
- `schemas/results.v0.3.schema.yaml`, `schemas/analysis_run.v0.3.schema.json`,
  `schemas/stress_neutral_export.v0.3.schema.json`: additive only
- new `tests/test_load_reference_schema.py`
- evidence under `LSI/CP3_SCHEMAS/` (RETURN.md, `_run_records/`)

The reader TASK concurrently reads `results.v0.3.schema.yaml` at test time.
Write every schema file atomically: write a temporary file in the same
directory, then rename it. Keep existing bytes of `$defs` you do not extend
unchanged. Make no other writes: no `core/**`, fixtures, desktop or Git.

## Acceptance and return

- A strict JSON parse of every schema passes.
- The new tests pass.
- The listed pre-existing schema tests pass unchanged.
- Before editing, record the sha256 of the three carrier schemas. After editing,
  run a path-preservation check proving the change is additive: every
  pre-existing JSON pointer keeps its value, except the enum arrays and
  `oneOf`/`anyOf` lists you extend.

Freeze `schemas/load_reference_state.schema.json` first and SendMessage the manager
its sha256 as soon as its `$defs` are stable. The reader TASK depends on it.
The return lists:

- files with their sha256;
- commands run, with raw logs;
- pass/fail;
- the preservation result;
- any wire/DTO ambiguity.

Write RETURN.md in `LSI/CP3_SCHEMAS/`.

## Resources

- Python only: `python3 -m pytest -q <specific files>` (no `-n auto`).
  Use a Python interpreter with `requirements-dev.txt` installed. The system
  `python3` lacks `jsonschema`, so the manager supplies a local environment path
  in the spawn message. Record the interpreter you used in `_run_records/`.
- No Cargo, browser, native, npm or UI.
- Disk is limited; keep scratch files small.
- Machine-specific paths only in `_run_records/`.
