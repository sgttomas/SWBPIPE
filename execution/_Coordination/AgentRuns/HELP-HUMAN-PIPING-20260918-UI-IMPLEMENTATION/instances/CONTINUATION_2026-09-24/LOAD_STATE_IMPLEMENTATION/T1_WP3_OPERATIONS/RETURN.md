# T1_WP3_OPERATIONS — return

- **Role:** TASK (Type 2). No delegation.
- **Git:** no Git writes.
- **Checkout:** load-state worktree, branch `codex/piping-load-states-20260925`, base `d8f0dc4f7`.
- **Paths:** relative to WORKING_ROOT (`projects/chirality-piping/`).

## Result

The applier now has three closed, typed authoring operations for the model 0.4.0 load/reference-state records. They run through the existing Review/Apply, inverse (Undo/Redo) and atomic-batch seam.

| Object / field path | Kind | Payload | Validation |
|---|---|---|---|
| `Model` / `reference_configurations` | `set_field` | the whole array | `Vec<ReferenceConfigurationInput>`; configuration IDs unique; every `member_references[].pipe_ref` exists |
| `Material` / `expansion_laws` | `set_field` | the whole array for the target material | `Vec<ExpansionLawInput>`; law IDs unique |
| `Load` / `analysis_state` | `update_load` | the target case's `analysis_state` | `AnalysisStateInput`; every reference must resolve (list below) |

The `analysis_state` references checked are:
- `reference_configuration_ref`, against the model's `reference_configurations`;
- each element's `pipe_ref`, against the model's pipes;
- each `material_ref`, against the model's materials;
- each `point_ref`, against the selected material's `temperature_points`;
- each `expansion_law_ref`, against the selected material's `expansion_laws`;
- each `support_ref`, against the model's supports;
- each `load_sources[].source_ref`, against **that case's** `primitive_loads` IDs.

The typed parse uses the product's public closed DTOs (`open_pipe_stress_product_physics::{ReferenceConfigurationInput, ExpansionLawInput, AnalysisStateInput}`). It reads the authored text directly, and there is no second schema.

Rules shared by all three operations:
- The model must already be `schema_version` "0.4.0". Nothing sets or changes the version.
- The current complete model hash is required.
- The envelope is `unit: none`, `dimension: dimensionless`.
- Before/after displays are canonical JSON, or `not_present` for an absent key.

## Files changed

| File | sha256 before | sha256 after |
|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `7d18d0451bce010f2ad799aa4790e2c0480d6e4c907735e62c4b546874797278` | `e40dd7aeb8efca62e9fbacb3b2bc39bd2c1c7b176366c902030eeb4118ff52b1` |
| `core/model_operations/operation_applier/src/load_state_authoring.rs` | absent (new) | `2e122074797f8bae59111233d5e013cbd95e5fb2b6978cfa94f3a8047ef52896` |
| `core/model_operations/operation_applier/tests/load_state_authoring.rs` | absent (new) | `59600b37f8ecda34e32d7c4f0837d6dd555c1b9134ea689ba5b66cbf3cca935e` |

These are unchanged and were verified: `Cargo.toml`, `Cargo.lock` and `schemas/model_operation.schema.json` (`7bdb79be…f44`). No fixtures were added. The hash records are in `_run_records/sha256_before.txt` and `_run_records/sha256_after.txt`.

### What each change does

**`src/load_state_authoring.rs` (new).** Follows the `pressure_authoring.rs` pattern: `owns`, then `resolve`, returning a `RichEdit`.
- **Checks run in this order:**
  1. the envelope;
  2. `schema_version` = `LOAD_STATE_MODEL_VERSION`;
  3. the target: `Model` must be the current `project.id`, and a `Material`/`Load` ID must match exactly one entity;
  4. the before-value against the current display.
- **An `after` of `not_present`** writes a removal. This is the inverse used for Undo when the key was absent. It is the same after-value convention the delete operations use.
- **Any other `after`:**
  - it is parsed as JSON;
  - an explicit `null` is refused;
  - it is then typed-parsed from the text with the product DTO;
  - finally IDs and references are checked.
- **Reading inner references.** The selection and thermal unions (`MaterialSelectionInput`, `ThermalStateInput`) sit in a private module, so their variants cannot be named from outside the crate. Their inner references (`material_ref`, `point_ref`, `expansion_law_ref`) are therefore read from the same authored value, after the typed parse has shown it has exactly the closed shape.
- **Writes.** It writes exactly one key, `[path]`. An unchanged payload is a no-op.

**`src/lib.rs` (+22/−1).** Four additions, all in the existing dispatch:
- `mod load_state_authoring;`
- the three field paths registered in the `rich_kind` operation contract: `Model`/`reference_configurations` and `Material`/`expansion_laws` as `set_field`, and `Load`/`analysis_state` as `update_load`;
- `OP-LOAD-STATE-MODEL-HASH-REQUIRED` when the claim is absent or null, next to the pressure equivalent;
- resolver selection routing the three paths to `load_state_authoring::resolve`.

The applied candidate, diff row, hash evidence and batch replay all use the existing rich-edit path, unchanged.

**`tests/load_state_authoring.rs` (new, 7 tests).** Base models are the two witnesses: `fixtures/product_preview/load_reference_source/eigen_motion.request.json` and `fixtures/product_preview/load_reference/connected.request.json`. Every edit is invented.

**Refusal codes (new).** Each is blocking. The model is never mutated and no diff row is produced.

| Code | When |
|---|---|
| `OP-LOAD-STATE-MODEL-HASH-REQUIRED` | the model-hash claim is missing or `null` |
| `OP-LOAD-STATE-SCHEMA-VERSION-INVALID` | the model is not already 0.4.0 |
| `OP-LOAD-STATE-ENVELOPE-INVALID` | the unit or dimension is not `none`/`dimensionless` |
| `OP-LOAD-STATE-TARGET-INVALID` | the target is not the project ID, or is an unknown or ambiguous entity |
| `OP-LOAD-STATE-EXPLICIT-NULL` | the after-value is an explicit `null` |
| `OP-LOAD-STATE-PAYLOAD-INVALID` | the text is not JSON, or the product's typed boundary refuses it |
| `OP-LOAD-STATE-DUPLICATE-ID` | a configuration or law ID repeats |
| `OP-LOAD-STATE-REFERENCE-UNRESOLVED` | a reference does not resolve |

Two existing codes are reused: `OP-BEFORE-VALUE-MISMATCH` (a stale before-value) and `OP-RICH-KIND-INVALID` (the wrong change kind).

## Tests and what they prove

1. **`reference_configurations_apply_undo_redo_and_resolve_as_closed_form_predicts`** (connected):
   - **Edit:** the second member's `natural_length_change` goes from −1 to −3 mm.
   - **Round trip:** Review shows one row with the before/after display. Apply changes only that key, checked against the whole model's bytes. The inverse restores the exact prior bytes in both canonical and plain serialization. Redo equals the applied model.
   - **Absent key:** the same round trip on a copy with the key absent; the inverse restores absence.
   - **Solve:** the applied model is solved with `run_linear_static_preview_value_with_mode` in both modes. The middle node's UX in both cases equals the closed form below.
   - **Closed form:** the witness constants give two series members with equal E·A: u_m = (u_r + u_f + ε*₁L₁ − ε*₂L₂)/2, with ε* = λ_fit·λ_th − 1, λ_fit = 1 + ΔL/L and λ_th = 1 + α(T − T_install). For the cold case this is 1.75 mm, against the witness's 0.75 mm. The unedited witness is also checked against the same formula.
2. **`expansion_laws_apply_undo_redo_and_resolve_as_closed_form_predicts`** (connected):
   - **Edit:** the secant constant α goes from 1.2e-5 to 2.0e-5 1/K.
   - **Checks:** the same round trip, including the absent key. The hot-case middle UX equals the closed form, (0.5 mm + (1 + α·130)·1 mm)/2.
3. **`analysis_state_apply_undo_redo_and_boundary_motion_moves_the_tip_rigidly`** (eigen_motion):
   - **Edits:** anchor UY goes from 1 to 2.5 mm, anchor RZ from 1e-4 to 3e-4 rad, and the tip line-stop UX from 5e-5 to 1.2e-4 m.
   - **Round trip:** same as above, including the absent key.
   - **Expectation, from the witness's rigid kinematics:**
     - u_y(tip) = u_y(root) + θ_z(root)·L = 3.1 mm, against the base witness's 1.2 mm;
     - u_x(tip) = the entered stop motion, 0.12 mm.
   - Both values are checked in both solver modes. The joined witness still finalizes; the solve returns `Ok` with `MECHANICS_SOLVED`.
4. **`typed_refusals_match_the_product_typed_boundary`**. Each case below gets `OP-LOAD-STATE-PAYLOAD-INVALID`, and the same payload placed in the document is also refused by the product (`Err`). That is the parity check.
   - **Unknown fields:**
     - top-level;
     - inside a closed quantity;
     - an authored `projection_sha256`;
     - on a law;
     - on a `participation`.
   - **Unknown discriminants:**
     - a fit kind;
     - a law `definition`;
     - `history: continuation`.
   - **Other:**
     - a missing required `provenance`;
     - the wrong JSON shape.

   The unedited witness passes the product boundary. A duplicated key in the authored text, and text that is not JSON, are both refused.
5. **`explicit_null_wrong_version_missing_hash_and_envelope_are_refused`**. Run for each of the three operations; the unchanged payload is first admitted as a no-op.
   - **Explicit null:** `null` and ` null ` are refused.
   - **Schema version:** `0.3.0`, `0.4.1`, a null version and an absent `schema_version` are refused.
   - **Model hash:** a missing claim, a null claim and a stale claim (`OP-CLAIMED-MODEL-HASH-MISMATCH`) are refused.
   - **Before-value:** a stale before-value is refused.
   - **Envelope and kind:** a bad unit and the wrong change kind are refused.
   - **Target:** an unknown target and an ambiguous (duplicated) target entity are refused.
6. **`duplicate_ids_and_dangling_references_are_refused`**:
   - **Duplicate IDs:** a duplicate configuration ID and a duplicate law ID.
   - **Dangling references:**
     - a member `pipe_ref`;
     - each of the eight `analysis_state` reference positions, including a `source_ref` naming a primitive stored in the *other* case;
     - a case naming a configuration when the model has none;
     - a law that exists only on a different material.
7. **`load_state_operations_apply_as_one_atomic_batch_or_not_at_all`**:
   - **Setup:** the witness with all four records removed.
   - **Forward batch:** configurations, then laws, then the two `analysis_state`s. It reviews as `passed`, and applies to exactly the witness bytes.
   - **Reversed batch:** a case would name a configuration not yet authored, so it is refused as a whole and returns no model.

## Checks

| Check | Result |
|---|---|
| Crate `cargo +1.97.1 test --locked --offline -j 2` | **183 passed**: baseline 176 (140+2+2+6+8+13+5) plus 7 new; 0 failed (`_run_records/baseline_cargo_test.log`, `_run_records/final_cargo_test.log`) |
| rustfmt (`rustfmt --edition 2021 --check`) on the two new files | clean |
| rustfmt on `src/lib.rs` | see below |
| Contract corpus (81 cases, cross-engine) | unchanged, passes |
| clippy | not run: no clippy component is installed for toolchain 1.97.1 |

**rustfmt on `src/lib.rs`.** The file was not rustfmt-clean at base: 12 pre-existing hunks (module order, the one-line `rich_kind` block, and others). After the edit there are still exactly 12 hunks. The added lines are written in rustfmt form and appear in the rustfmt output only as unchanged context, so this change adds no new formatting debt. I did not reformat the pre-existing hunks, to keep the diff minimal.

## Mutation evidence

- **Method:** a scratch copy from `git archive HEAD` of `core/`, `fixtures/` and `schemas/`, with the three candidate files overlaid. Each mutant makes one textual edit, then runs `cargo test --test load_state_authoring` and restores the file.
- **Records:** script `_run_records/mutants.py`, result `_run_records/mutants_result.json`.
- **Result:** 24/24 killed. No mutant failed to compile, and there were no survivors.

| Mutant (refusal removed) | Killed by |
|---|---|
| M01 envelope | explicit_null_wrong_version… |
| M02 schema version | explicit_null_wrong_version… |
| M03 Model target = project ID | explicit_null_wrong_version… |
| M04 before-value | explicit_null_wrong_version… |
| M05 explicit null | explicit_null_wrong_version… |
| M06 typed parse, reference_configurations | typed_refusals…, duplicate_ids… |
| M07 typed parse, expansion_laws | typed_refusals…, duplicate_ids… |
| M08 typed parse, analysis_state | typed_refusals…, duplicate_ids…, atomic_batch… |
| M09 typed parse via Value (loses duplicate-key refusal) | typed_refusals… |
| M10 unique configuration IDs | duplicate_ids… |
| M11 unique law IDs | duplicate_ids… |
| M12 member pipe_ref | duplicate_ids… |
| M13 reference_configuration_ref | duplicate_ids… |
| M14 element pipe_ref | duplicate_ids… |
| M15 material_ref | duplicate_ids… |
| M16 point_ref | duplicate_ids… |
| M17 expansion_law_ref | duplicate_ids… |
| M18 law resolved on any material | duplicate_ids… |
| M19 support_ref | duplicate_ids… |
| M20 source_ref | duplicate_ids… |
| M21 source_ref resolved on any case | duplicate_ids… |
| M22 model-hash requirement (lib.rs) | explicit_null_wrong_version… |
| M23 `not_present` inverse (restoring absence) | all three round-trip tests |
| M24 ambiguous target entity | explicit_null_wrong_version… |

The first run left M24 surviving. I added the ambiguous-target assertion and re-ran all 24; all were killed.

## Not done

- **Desktop side.** Desktop fields, TypeScript contracts, native persistence and the browser-mirror (TypeScript) engine are not done; they come later, after T0R, per the brief.
- **Contract corpus.** No cases were added to `fixtures/model_operations/contract_corpus/`. The corpus is executed by both engines, and a new case would need the TypeScript mirror, which is `apps/desktop/**` and outside this boundary.
- **Operation schema.** `schemas/model_operation.schema.json` did not need to change. It has no field-path enumeration, and it has no `Model`/`Material` target types either; the existing pressure operations on those objects are not listed there.

## Design questions for the manager

1. **Inbound references are not guarded by the operation seam.** Three kinds of operation can leave a case's `analysis_state` naming something that no longer exists:
   - replacing or removing `reference_configurations`, which can orphan a case's `reference_configuration_ref`;
   - replacing or removing `expansion_laws`, which can orphan an `expansion_law_ref`;
   - the existing delete operations (pipe, support, material, primitive load), which do not scan the new records. They scan `pressure_regions`, but not `analysis_state` or `reference_configurations`.

   Nothing is silent: the product blocks at solve, for example with `LOAD_STATE_REFERENCE_CONFIGURATION_UNRESOLVED` or `LOAD_STATE_SOURCE_UNRESOLVED`. The brief's validation table does not ask for inbound checks, so I did not add them.

   **Question:** should inbound-reference refusals be added, to these operations and to the delete operations? Adding them would make the Undo order strict (LIFO), which the checkpoint stack already gives.
2. **`not_present` as an after-value.** This is how the inverse restores an absent key. It mirrors the delete operations' convention, and it is distinct from the refused explicit `null`. Desktop Undo is checkpoint-based (whole-model snapshots), so this matters for operation-level and agent replay. Please confirm it is acceptable.
3. **Duplicated keys in the authored text are refused,** because the typed parse reads the text. The product reads a `Value`, in which a duplicated key would already be last-wins. Here the operation is stricter than the product, and in the safe direction.
4. **What the operations deliberately leave to the product's solve-time diagnostics,** because they are not in the brief's table:
   - the `contract` string;
   - element and support coverage;
   - member coverage and duplicates inside a configuration;
   - `material_ref` equal to the pipe's material;
   - predecessor references inside `locked_equivalent_support`;
   - quantity and unit admissibility.

   The law reference is resolved against the element's selected `material_ref`, not "anywhere in the model"; the product requires the law to be on the member's own material. Say if any of the others should be refused at authoring time.

## Scratch and disk

The cargo target and the scratch copy were deleted after the checks.

## Addendum 1 — inbound-reference refusals (manager follow-up, 2026-09-26)

This addendum covers the manager's follow-up on the design questions:

- **Q1:** add inbound-reference refusals. Done, as described below.
- **Q2:** `not_present` as the inverse is accepted.
- **Q3:** refusing duplicated keys is accepted.
- **Q4:** those checks stay at solve time.

The write boundary and the rules are unchanged. I made no Git writes. The text above this addendum is unchanged, except for the diff count and the cleanup note, which were corrected before the first report.

### Files (sha256 after this addendum)

| File | Before this addendum | After |
|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `e40dd7ae…` (base `7d18d045…`) | `4f088d67e4e62c1ce403c3e5555b43dad06244e573fe7deb1feef643b7d4234c` (+46/−2 against base) |
| `core/model_operations/operation_applier/src/load_state_authoring.rs` | `2e122074…` | `63223cc2d391235fe1523785dec0b2b145ba5d64175c77e895ad4118b293c654` |
| `core/model_operations/operation_applier/tests/load_state_authoring.rs` | `59600b37…` | `393ad349ca749bd44e8919134fc70ddd4be838d1f9947bfdcbcb56d3a60c92da` |
| `core/model_operations/operation_applier/tests/load_state_delete_control.rs` | absent (new) | `cf7c34feeca0b72f898218a64f89c56e70def5557d2c2883c6c1ad986598b880` |
| `fixtures/model_operations/load_reference_delete_control.json` | absent (new) | `1da7ef35cc20773643e3a0f08f7f76ecb6ec301a953c43acd8207ebf4186e16f` |

`Cargo.toml`, `Cargo.lock` and `schemas/model_operation.schema.json` are unchanged. The hashes are recorded in `_run_records/sha256_after_addendum1.txt`.

### What changed

**(a) Configurations.** Replacing or removing `Model/reference_configurations` is refused with `OP-LOAD-STATE-INBOUND-REFERENCE` when it would orphan a case's `analysis_state.reference_configuration_ref`. The message names each referring path, for example `case:cold.analysis_state.reference_configuration_ref (reference:installed)`.

**(b) Expansion laws.** Replacing or removing a material's `expansion_laws` is refused with the same code when it would orphan an element's `thermal_state.expansion_law_ref`. Only elements whose `material_selection.material_ref` is that material count. The message names, for example, `case:hot.analysis_state.element_states.1.thermal_state.expansion_law_ref (law:secant)`.

For both (a) and (b), "orphan" means the reference resolves before the edit and not after. A reference that was already unresolved is not created by the edit, so it stays a solve-time diagnostic, as Q4 directs. The check lives in `load_state_authoring::refuse_orphans`. It runs after the typed validation, and for `not_present` removals too.

**(c) Delete operations.** Each delete now refuses when its target is named by a load/reference-state record, in the same way it already refused for `pressure_regions`: the referring paths join the operation's existing reference list and code.

| Delete | Referring records it now checks | Code |
|---|---|---|
| `delete_pipe_run` | `reference_configurations.<id>.member_references.<k>.pipe_ref`; `<case>.analysis_state.element_states.<i>.pipe_ref` | existing `OP-PIPE-DELETE-REFERENCED` |
| `delete_support` | `<case>.analysis_state.support_states.<i>.support_ref` | existing `OP-SUPPORT-DELETE-REFERENCED` |
| `delete_primitive_load` | the owning case's `analysis_state.load_sources.<i>.source_ref` | new: `OP-LOAD-STATE-INBOUND-REFERENCE` |
| `delete_material` | `material_ref`, and `point_ref` to its owned temperature points, through the generic `_ref` reference scan | existing `OP-ENTITY-DELETE-REFERENCED` |

- **Pipe and support.** The referring paths are appended to the operation's reference list. For support deletion they come after the existing nonlinear-support references.
- **Primitive load.** Deleting a primitive load had no reference refusal before, so there was no existing code to reuse; it uses the new code. The refusal names the case's path in `affected_refs`.
- **Material.** No code change was needed. The existing generic `_ref` scan already covers `material_ref`, and `point_ref` to its owned temperature points. A law reference always comes with a `material_ref` to the same material, so it is covered as well. A test now proves the `material_ref` case.

The scanners live in `load_state_authoring.rs` (`pipe_references`, `support_references`, `source_references`). In `lib.rs` there are three call sites and one new blocking push; no existing message text changed.

`lib.rs` still has exactly the 12 pre-existing rustfmt hunks, and none of the added lines appears in them. The new and changed test and source files are rustfmt-clean.

### Unchanged behaviour for models without the records (control)

- **Existing tests.** All 176 pre-existing tests pass unchanged. Among them are the 81-case cross-engine contract corpus, which covers the delete cases 48–57 at byte level, and the 0.3.0 exact-authoring tests.
- **New control.** `tests/load_state_delete_control.rs` runs 86 validate and apply outcomes over five models:
  - the fixture models `precision_connected_ui_model` (0.2.0), `exact_pressure_authoring_model` (0.3.0) and `physics_thermal_ui_model` (0.3.0);
  - the connected and eigen_motion witnesses (0.4.0), with every `reference_configurations`, `expansion_laws` and `analysis_state` removed.

  Every pipe, support, material and primitive-load delete is run on them. The test compares the sha256 of each serialized `OperationOutcome` to the golden file.
- **Golden file.** `fixtures/model_operations/load_reference_delete_control.json` was produced on a scratch copy of the **base revision** (`git archive HEAD`, with only the control test added), by running the test with `LOAD_REFERENCE_DELETE_CONTROL_BLESS=1`. The candidate then matches it exactly.
- **Coverage of the golden file.** It covers all four operations. It includes accepted outcomes with applied models, and refused outcomes, among them `OP-PIPE-DELETE-REFERENCED`, `OP-ENTITY-DELETE-REFERENCED`, the payload-invalid refusals and `OP-STALE-BEFORE-VALUE`.

### Tests (3 new in `load_state_authoring.rs`, 1 new control file)

- **`replacing_or_removing_owners_cannot_orphan_case_references`:**
  - **Refused:** a rename or removal of the configuration, naming both cases' paths; a rename or removal of the law, naming all four element paths; the earlier absent-key scenario, where the inverse removal is refused once the configuration makes the cases' references resolve.
  - **Admitted:** keeping the named configuration and adding another; a replacement while a reference is already unresolved; a second material with a same-ID law dropping its laws, because no element selects that material.
- **`delete_operations_refuse_targets_named_by_load_state_records`:**
  - **Pipe:** refused for members and element states together, and for either record alone.
  - **Support:** refused for support states.
  - **Material:** refused through `material_ref`.
  - **Primitive load:** refused in each case for a primitive its load sources name; the unnamed stored primitive can be deleted.
  - **Control:** with the records stripped, all five deletes apply.
- **`atomic_batch_removes_the_states_before_their_configuration_and_passes`:**
  - **Forward batch:** both `analysis_state` removals, then the laws, then the configurations, then the now-unnamed primitive. It reviews as passed and applies to exactly the expected bytes.
  - **Configurations first:** refused as a whole.
  - **Primitive first:** refused as a whole.
- **`delete_outcomes_without_load_state_records_are_byte_identical_to_the_base_revision`:** the control described in the previous section.

**Changes to earlier tests.** Two of this TASK's own earlier absent-key round trips (for `reference_configurations` and `expansion_laws`) started from an inconsistent base: the cases still named the removed records. Under the new rule the inverse removal in that base is correctly refused. Both round trips now start from the same model with every `analysis_state` removed. The round-trip property is still proven in full, and the old scenario is now asserted as a refusal in the first new test above. No assertion was removed or weakened.

### Checks

| Check | Result |
|---|---|
| Crate `cargo +1.97.1 test --locked --offline -j 2` | **187 passed**, 0 failed: base 176, plus 10 in `load_state_authoring`, plus 1 control (`_run_records/addendum1_cargo_test.log`) |
| rustfmt | new and changed files clean; `lib.rs` unchanged at the 12 pre-existing hunks |

### Mutation evidence

- **Method:** the same runner (`_run_records/mutants.py`), extended with N01–N09, on a fresh `git archive` scratch copy with the candidate files overlaid.
- **Result:** all 33 mutants were killed, M01–M24 re-run plus N01–N09 (`_run_records/mutants_result_addendum1.json`).

| Mutant (refusal removed) | Killed by |
|---|---|
| N01 orphan check for `reference_configuration_ref` | replacing_or_removing_owners… |
| N02 orphan check for `expansion_law_ref` | replacing_or_removing_owners… |
| N03 "only newly unresolved" (refuse already-unresolved too) | replacing_or_removing_owners… |
| N04 law orphan scoped to elements selecting the material | replacing_or_removing_owners… |
| N05 pipe delete: all load/reference-state references | delete_operations_refuse… |
| N06 pipe delete: configuration members | delete_operations_refuse… |
| N07 pipe delete: element states | delete_operations_refuse… |
| N08 support delete: support states | delete_operations_refuse… |
| N09 primitive delete: load sources | delete_operations_refuse…, atomic_batch_removes… |

`delete_material` gained no new code, so it has no mutant. Its `material_ref` refusal comes from the existing generic scan and is asserted by a test.

### Notes

- **Geometry operations.** `split_pipe_run` and `transform_pipe_run` keep the original pipe ID, and like `pressure_regions`, they do not update load/reference-state records. After a split, the new pipe has no configuration member or element state. The product blocks this at solve time with its coverage diagnostics. I did not change it; it is outside this follow-up.
- **Cleanup.** The cargo target and scratch copy were recreated for this addendum and deleted again afterwards.

## Addendum 2 — review B repairs (F1, F2, F5, F6, F7, F9)

- **Request:** the manager's repair request after `T1_WAVE1_REVIEW_B/RETURN.md`.
- **Scope:** the same write boundary (`operation_applier/**`, new `fixtures/model_operations/load_reference_*`, this folder) and the same rules.
- **Base:** branch head `203396e4d`, where the Addendum-1 bytes are committed.
- **Git:** no Git writes.
- **Earlier text:** unchanged.
- **Outside this addendum:** F3, F4 and F8 belong to other TASKs' files.

### Files (sha256; "before" = `203396e4d`)

| File | Before | After |
|---|---|---|
| `src/pressure_authoring.rs` | `b1017d66…` | `947e296145062b017dace877ae97f3ecdf79c3bba2daadd366f893ac0eaf5391` (+11) |
| `src/rich_authoring.rs` | `3c21a44c…` | `168ec01dbc84826d81d066e9f2690ecb09c0f1ab4b55f7219b7b0304de555c03` (+25/−10) |
| `src/load_state_authoring.rs` | `63223cc2…` | `6f53188b599b37a277c09c34404b04722552e7f7d7d6b45840590851408a170e` (+50) |
| `tests/load_state_authoring.rs` | `393ad349…` | `e32bf54fdfc81960b04e649b82d57294e0cf6a8ed2c4e1c586a26f7743e4c1df` (+286/−12) |
| `tests/load_state_delete_control.rs` | `cf7c34fe…` | `35d128020f56ef35853721ffa43c044df2d24649c1fb90a132b76afa5791b610` (+164) |
| `fixtures/model_operations/load_reference_authoring_control.json` | absent (new) | `2e21291cf07dbf900cc12ed6136a8b6a1b3a65295c25dbf859e803334bc2ade4` |

These are unchanged:
- `src/lib.rs` (`4f088d67…`);
- `fixtures/model_operations/load_reference_delete_control.json` (`1da7ef35…`; re-blessed from base, byte-identical);
- `Cargo.toml` and `Cargo.lock`.

Crate paths are under `core/model_operations/operation_applier/`. The hash records are in `_run_records/sha256_after_addendum2.txt`.

### Repairs

**F1 (pressure profile on 0.4.0).** `pressure_authoring::resolve` now refuses `Model/pressure_profile` whenever the current model's `schema_version` is 0.4.0 (`LOAD_STATE_MODEL_VERSION`).
- **Code:** a new pressure-specific code, **`OP-PRESSURE-PROFILE-SCHEMA-VERSION-LOCKED`**. I did not reuse `OP-LOAD-STATE-SCHEMA-VERSION-INVALID`: that code means "a load-state operation needs 0.4.0", which is the opposite condition, and reusing it would make the two refusals indistinguishable in diagnostics.
- **When it fires:** before the target and before-value checks, for every after-value, including an unchanged profile. Models before 0.4.0 never reach it.

**F2 (temperature points orphaning a `point_ref`).** The rich `Material/temperature_points` path now calls `load_state_authoring::refuse_point_orphans` after its existing validation. It refuses with `OP-LOAD-STATE-INBOUND-REFERENCE` when an `exact_point` `point_ref` is orphaned. It applies the `refuse_orphans` rule: the point resolves before the edit and not after. It counts only elements whose `material_selection.material_ref` is the target material. The message names each case path, for example `case:cold.analysis_state.element_states.0.material_selection.point_ref (point:cold)`. A model without `analysis_state` is unaffected.

**F5 (review mutants R01, R02, R04).** One targeted test for each; none was equivalent.
- **R01 (unchanged payload must not write).** `an_unchanged_payload_is_a_true_no_op_with_no_write`. Each of the three payloads is re-entered with the integer `20` written as `20.0`. The text is canonically unchanged, so nothing may be written. The applied model must keep its exact plain serialization, including the integer. Under R01 the write stores `20.0`, which changes the bytes.
- **R02 (`point_ref` only on the selected material).** `point_ref_resolves_only_on_the_selected_material`. A point that exists only on a second material is named with `material_ref` = the first material, and is refused.
- **R04 (support scan across every case).** `support_deletion_scans_every_case_not_only_the_first`. `support:far` is named only in the second case's `support_states`, and deleting it is refused naming `case:hot…`.

**F6 (solver modes and closed form).**
- `displacement_mm` now returns the value from both modes, sparse then dense. Every call site checks **both** against the prediction (`close_both`).
- `connected_middle_ux_mm` now uses the general series form, u_m = (u_r/L₁ + u_f/L₂ + ε*₁ − ε*₂)/(1/L₁ + 1/L₂). It also asserts that the far anchor has no entered motion (u_f = 0).
- The earlier RETURN sentence "both values are checked in both solver modes" is true only from this addendum on.

**F7 (explicit-null prior).** A doc comment at `NOT_PRESENT` in `load_state_authoring.rs` now records that the operation-level inverse cannot restore an explicit-null prior. Such a prior is not admissible in 0.4.0: the product blocks it with `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`. Desktop Undo restores checkpoints, so it is unaffected.

**F9 (spurious warnings on 0.4.0).** In `validate_temperature_points`, a 0.4.0 model with the exact pressure contract is now the exact profile, so the modulus pair is `poisson_ratio`.

This goes one step beyond the review's wording. For 0.4.0 exact models only, a point's `thermal_expansion_coefficient` is also no longer demanded for completeness. In 0.4.0 the thermal definition lives in the load/reference state (expansion laws or explicit interval states), and legacy thermal primitives are refused by the product. Without this step the spurious warning would remain for every 0.4.0 point.

For 0.2.0 and 0.3.0 models the logic is unchanged. I also formatted the three affected `let` lines to rustfmt style, which removes one pre-existing rustfmt hunk (22 → 21).

### Byte-identity for models before 0.4.0

- **Existing tests.** They pass unchanged, including the 81-case contract corpus and `exact_authoring`, which carries the 0.3.0 pressure-profile route and the legacy-to-exact authoring sequence.
- **Delete control.** `load_reference_delete_control.json` was re-blessed from a `git archive` of `203396e4d` and is byte-identical (`1da7ef35…`). The candidate matches it.
- **New authoring control.** `pressure_profile_and_temperature_point_outcomes_before_0_4_0_are_byte_identical_to_the_base_revision` runs 54 validate and apply outcomes on the 0.2.0 and two 0.3.0 fixture models:
  - pressure profile: to exact 0.3.0, to 0.4.0 (refused), and unchanged;
  - temperature points: author from absent, remove, rename, add an incomplete point, unchanged, and empty.

  The sha256 of each serialized `OperationOutcome` is compared with `load_reference_authoring_control.json`, which was blessed from `203396e4d`. The golden file covers accepted and refused outcomes, and it covers the 0.2.0 and 0.3.0 differences in `OP-RICH-NOT-SOLVE-READY`.
- **0.4.0 models are excluded from this control,** because F9 deliberately changes their warnings.

### Tests

**Added to `load_state_authoring.rs` (5 new):**
- **`pressure_profile_cannot_change_a_0_4_0_model`:**
  - the downgrade the review found, and an unchanged profile, are refused on `connected`;
  - an atomic batch (an admitted `analysis_state` edit, then the downgrade) is refused as a whole in both review and apply, with no model;
  - the edit alone applies and stays 0.4.0.
- **`temperature_points_replacement_cannot_orphan_an_exact_point`:**
  - **Refused:** removing or renaming `point:cold`, and removing `point:hot`; all four element paths are named.
  - **Admitted, no warning (F9):** a complete new E/nu point.
  - **One warning:** an incomplete point, only for its missing pair.
  - **Controls, admitted:** without the records, removal and rename; a second material with the same point IDs dropping its points; a replacement when the references were already unresolved.
- **The three F5 tests described above.**

**Changed:** the F6 helpers and all six solve call sites.

**Added to `load_state_delete_control.rs`:** the authoring-control test described above.

### Checks

| Check | Result |
|---|---|
| Crate `cargo +1.97.1 test --locked --offline -j 2` | **193 passed**, 0 failed: 140 + 2 + 2 (corpus) + 6 + 8 (`exact_authoring`) + 13 + 15 (`load_state_authoring`) + 2 (controls) + 5 (`_run_records/addendum2_cargo_test.log`) |
| rustfmt, touched files | `pressure_authoring.rs`, `load_state_authoring.rs` and both test files clean |
| rustfmt, `rich_authoring.rs` | 21 pre-existing hunks (was 22), none containing added lines |
| rustfmt, `lib.rs` | untouched, 12 pre-existing hunks |

### Mutation evidence

- **Method:** the same runner, on a fresh `git archive` scratch copy of `203396e4d` with the candidate files overlaid.
- **Result:** all **42 killed**, 0 survived (`_run_records/mutants_result_addendum2.json`).
- **Re-run:** M01–M24 and N01–N09, still killed.
- **New:** review B's R01, R02 and R04, run with their exact text, plus P01–P06.

| Mutant | Killed by |
|---|---|
| R01 unchanged payload still writes | an_unchanged_payload_is_a_true_no_op_with_no_write |
| R02 `point_ref` resolved on any material | point_ref_resolves_only_on_the_selected_material |
| R04 support scan limited to the first case | support_deletion_scans_every_case_not_only_the_first |
| P01 pressure-profile version lock removed (F1) | pressure_profile_cannot_change_a_0_4_0_model |
| P02 temperature-points orphan check removed (F2) | temperature_points_replacement_cannot_orphan_an_exact_point |
| P03 point orphan not scoped to the selecting material | temperature_points_replacement_cannot_orphan_an_exact_point |
| P04 point orphan counts already-unresolved references | temperature_points_replacement_cannot_orphan_an_exact_point |
| P05 exact profile not extended to 0.4.0 (F9) | temperature_points_replacement_cannot_orphan_an_exact_point |
| P06 0.4.0 points still demand the coefficient (F9) | temperature_points_replacement_cannot_orphan_an_exact_point |

Review B's R03, R05 and R06 were already killed in its own run, so I did not repeat them. F6 and F7 are test and documentation changes, so they have no mutant.

### Cleanup

The scratch copy and cargo target were deleted after the checks.

## Addendum 3 — backcheck L-1 (the F9 relaxation is 0.4.0-only)

- **Request:** the manager's follow-up to `T1_WAVE1_REVIEW_B/BACKCHECK.md`, Addendum 1, note L-1.
- **Scope:** the same write boundary and rules.
- **Base:** head `8fe3e5a5a`.
- **Git:** no Git writes.
- **Earlier text:** unchanged.

**The gap.** The backcheck's mutant B09 changes the F9 rule in `rich_authoring.rs` from `load_state && exact_profile` to `exact_profile`. That would stop 0.3.0 exact models from demanding `thermal_expansion_coefficient` on points. The code was correct, but no test pinned the 0.3.0 side.

**The change.** One new test in `tests/load_state_authoring.rs` (+48 lines; `e32bf54f…` → `27680766af9424e3cf37962d25eff2b602341ba38264a00bc5bda77228cb370b`): `exact_0_3_0_points_still_demand_the_expansion_coefficient`.
- On `exact_pressure_authoring_model` (0.3.0, exact contract), it authors two invented temperature points, both complete for E/ν.
- The point without α must raise exactly one `OP-RICH-NOT-SOLVE-READY` warning, which names it.
- The point with α must raise none.

No source file changed. The goldens were not re-blessed, and both still match unchanged: `load_reference_delete_control.json` `1da7ef35…` and `load_reference_authoring_control.json` `2e21291c…`.

**Checks:**
- **Crate suite:** `CARGO_INCREMENTAL=0`, the shared target `ls-vp-target` reused with no new target, and 13–14 GB free before the builds. **194 passed**, 0 failed: 140 + 2 + 2 + 6 + 8 + 13 + 16 + 2 + 5 (`_run_records/addendum3_cargo_test.log`).
- **rustfmt:** the test file is clean.

**Mutants.** Run on a `git archive` scratch copy of `8fe3e5a5a`, with the same runner and the new entry B09 (`_run_records/mutants_result_addendum3.json`):

| Mutant | Committed tests | With the new test |
|---|---|---|
| B09 F9 relaxation not limited to 0.4.0 | SURVIVED (reproduces L-1) | **KILLED** by `exact_0_3_0_points_still_demand_the_expansion_coefficient` |
| P05 exact profile not extended to 0.4.0 (re-check) | — | KILLED |
| P06 0.4.0 points still demand α (re-check) | — | KILLED |

**Cleanup:** the scratch copy is deleted. The shared target `ls-vp-target` is left in place, as instructed.
