# TASK brief — T1_WP3_OPERATIONS (typed authoring operations for 0.4.0 load/reference state)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP3_OPERATIONS/`.

## Assignment

Add closed, typed authoring operations to `core/model_operations/operation_applier`, so that the 0.4.0 records can be authored through the existing Review/Apply, Undo/Redo and atomic-batch seam. Follow the `pressure_authoring.rs` pattern: ownership, the model-hash requirement, `resolve`, and before/after display. These are the model-side operations only. Desktop fields, TypeScript contracts and native persistence come later, after T0R.

| Object / field path | Kind | Payload | Validation |
|---|---|---|---|
| `Model` / `reference_configurations` | `set_field` | the whole array | typed parse as `Vec<ReferenceConfigurationInput>`; every `pipe_ref` exists; IDs are unique |
| `Material` / `expansion_laws` | `set_field` | the whole array for the target material | typed parse as `Vec<ExpansionLawInput>`; IDs are unique |
| `Load` / `analysis_state` | `update_load` | the target load case's `analysis_state` | typed parse as `AnalysisStateInput`. Every `reference_configuration_ref`, `pipe_ref`, `support_ref`, `material_ref`, `point_ref`, law ref and `load_sources[].source_ref` must resolve in the model: the source refs to that case's `primitive_loads` IDs |

**Typed parsing.** Use the product's public closed DTOs, re-exported from `open_pipe_stress_product_physics` (`case_state::input`). The operations then refuse exactly what the product's typed boundary refuses: unknown fields, explicit null on these keys, and unknown discriminants. Do not write a second, looser schema.

**Rules shared by all three operations:**

- The target model must already be `schema_version` "0.4.0" (D3). Refuse otherwise, with a targeted code. No operation sets or changes `schema_version`. Creating a new 0.4.0 model is later work in the desktop, after T0R.
- The current complete model hash is required, as for the pressure operations.
- Undo/Redo restores the exact prior bytes of the field, including its absence where the key was absent.
- The applied model re-solves: prove this in tests by running `open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode` on the applied model. Use the maintained witness `fixtures/product_preview/load_reference_source/eigen_motion.request.json` and `fixtures/product_preview/load_reference/connected.request.json` as base models, with invented edits.

Register the new field paths wherever the applier enumerates its operation contract. If `schemas/model_operation.schema.json` must change to admit them, include that edit (additive only) and say so.

**Tests.** Add them in the crate's `tests/`, in a new file:

- apply, undo and redo round-trip bytes for each operation;
- each refusal (unknown field, null, dangling reference, wrong schema version, missing hash, duplicate ID);
- an applied edit changes the solved result as the closed form predicts: for example, a changed `boundary_motion` value moves the tip by exactly that motion under the witness's rigid kinematics. Take the expectation from the witness constants, not from the producer.

## Write boundary

- `core/model_operations/operation_applier/**`.
- `schemas/model_operation.schema.json` (additive, only if needed).
- `fixtures/model_operations/load_reference_*` (new files, only if needed).
- Your return folder.

## Checks

- The operation_applier crate: `cargo test`. Baseline 176 plus yours.
- rustfmt clean on the files you touch.
- Mutants removing each new refusal.
