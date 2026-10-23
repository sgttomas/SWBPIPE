# Applying library records to model entities (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

**Status** (coordinator, 2026-10-23): the model-and-states manager implements this, as part of W-13 S2 or right after it; W-13 §3.5 assigns the work there. Two items are proposals relayed to that manager: the `library_version` definition (`sha256:` and the SHA-256 of the record's RFC 8785 form, §2.5) and the warning `MODEL-LIBRARY-REF-UNRESOLVED` (§2.7).

Requirements: FR-RULE-005 (Must) and FR-RULE-006 (Should). The map rows read
"Library records cannot be applied to model materials". Related: the W-13 design
(DEL-02-01 `Design/model-document-0.7.0.md`), §3.5 `provenance_record.library_ref`
and §6 authoring.

## 1. Where things stand

- **The store.** The private library store holds one document per
  `(project_id, library_kind, library_id)`, for the kinds material, section,
  component and hanger (`apps/desktop/src-tauri/src/lib.rs:868-885,3709-3717`).
  Only an accepted private import is stored (`:3916-3971`).
- **The Library Manager** imports, validates, stores and lists libraries. It
  never edits the model. Its only prop is `model`
  (`features/library/LibraryManagerPanel.tsx:185-191`; mounted at
  `App.tsx:905`).
- **Rule checks** read a stored record's slot value at run time through
  `library_value_ref` (`src-tauri/src/lib.rs:3016-3175`). That is the only
  current consumer of stored values.
- **A precedent for applying records:** `features/hanger-selection/hangerSelection.ts`.
  - It maps a hanger record into a `create_support` or `update_support` batch.
  - It writes the whole library metadata block and the record snapshot into
    the support's `provenance` string and the change's `source_note`
    (`:36-40,52,64`). That puts library payload in the model document.
- **The library schemas carry no version member.** Their metadata members are
  `library_id`, `name`, `library_scope`, `privacy_class`, `provenance` and
  `review_status` (`schemas/material.schema.yaml`, `section.schema.yaml` and
  `component.schema.yaml` `LibraryMetadata`; the hanger schema adds
  `redistribution_status`). W-13 §3.5's `library_ref.library_version`, and its
  `MODEL-LIBRARY-REF-VERSION-CHANGED` comparison, therefore have no source
  today.
- **Unit forms differ.**
  - Material values carry `unit_ref.ref_id`, a catalog unit id. The panel
    writes ids such as `unit:pascal` (`LibraryManagerPanel.tsx:140-161`).
  - The invented fixture carries `unit.MPa`, which is not a catalog id
    (`fixtures/material/invented_material_library_valid.json:54`).
  - Section, component and hanger values carry a plain `unit` symbol.

## 2. Decisions

1. **Every apply is an applier operation batch,** built by the desktop from a
   stored document after native validation, like the hanger path. The batch is
   validated, then shown as a diff, then applied. One undo checkpoint is
   recorded (`workspaceSession.ts:1192-1223`). Nothing writes the model
   directly.
2. **Insert or apply.**
   - **Insert** creates a new material (`create_material`, applier
     `lib.rs:4277`), a new section (`create_section`, `:4061`), or a new hanger
     support (`create_support`).
   - **Apply to an existing entity** uses one new change kind,
     `apply_library_record`, on Material, Section or Component. It writes the
     mapped members, creating absent ones, and carries the replaced members as
     `before`, as `update_support` does.
   - A new kind is needed because `set_field` cannot author an absent quantity
     (`OP-QUANTITY-OBJECT-MISSING`, `lib.rs:6845-6856`). Hangers keep
     `update_support`.
   - Components are apply-only. A record carries no node or pipe placement,
     and a component's `component_type` must match the target's `kind`; for
     branch records the matching kinds are `branch`, `tee` and
     `branch_connection`.
3. **The fields that copy.** Copying is fixed by table. Anything else is listed
   to the user as "not copied".

| Kind | Copies | Not copied (reason) |
|---|---|---|
| Material | `name`→`label`; `elastic_modulus`, `shear_modulus`, `poisson_ratio` and `thermal_expansion_coefficient` properties that have no `temperature_ref` and whose `value_status` is `provided` | `density` (the model holds density per pipe, `section.material_density`); strengths, `allowable_stress` and allowables (rule checks read them from the library at run time); temperature-referenced properties (library temperature points carry no temperature value); `specific_heat`, `thermal_conductivity`, `user_defined` (no model slot) |
| Section | `name`; `section_type` `pipe` only; the `outside_diameter` and `wall_thickness` dimensions → `properties` | Every other dimension or property. A bound section must hold only those two properties, or the solver refuses it (`core/product_physics/src/lib.rs:8751-8758`). Allowances and insulation are pipe fields. |
| Component | Field kinds with an identical model slot: a `geometry` key of the same name (e.g. `bend_radius`, `rigid_body_length`, `weight`, `effective_area`), or `modifiers.<kind>` / `modifiers.<kind>_user_value` (e.g. `sif_user_value`, `linear_stiffness`) | `outside_diameter`, `wall_thickness`, `bend_centerline_radius`, `bend_included_angle`, `bend_tangent_length`, the end orientations, `branch_size`, `hardware_flag`, `user_defined` (no slot of the same meaning) |
| Hanger | As `hangerSelection.ts:42-51` maps today | Unchanged |

   - **No derived value.** A material without `shear_modulus` is refused for
     insert, naming the field; G is never computed from E and ν. The same holds
     for any required member.
4. **Units.**
   - Values copy in their entered unit (DEC-018, entered units preserved).
     Nothing is converted.
   - A material `unit_ref.ref_id` resolves to its symbol through the desktop's
     unit catalog (`unit_id` → `symbol`). An id the catalog does not hold, such
     as the fixture's `unit.MPa`, refuses the apply with
     `LIBRARY-APPLY-UNIT-UNRESOLVED`.
   - A string magnitude must parse to a finite number.
   - The applier then checks each slot's dimension and positivity as it does
     today, along with the project-unit preconditions.
5. **Provenance.** Each written entity carries:
   - **`provenance`,** a short string `library:<kind>:<library_id>#<record_id>`;
   - **`provenance_record`** (W-13 §3.5):
     - the record's seven provenance fields, copied, with the record's own
       `redistribution_status` and `review_status` taking precedence;
     - `library_ref` = `{library_kind, library_id, library_version, record_id}`.
   - **`library_version`** is `sha256:` followed by the SHA-256 of the applied
     record's RFC 8785 form (`hashService.ts:95`). It is a record digest because
     the library has no version member (§1), and drift in other records of the
     same library does not flag this entity.
6. **The private-data boundary.**
   - **What enters the model document,** which is the user's private data
     (W-13 §3.4): the copied values and units, the label, the provenance string
     and `provenance_record`.
   - **What never enters it:** the library metadata block, other records,
     allowables, non-copied slots, completeness blocks or any record snapshot.
     The same rule holds for the change's `source_note` and rationale, which are
     persisted with editor intents.
   - **The hanger path moves to this rule.** It drops `library_metadata` and
     `record_snapshot` from `provenance` and `source_note`.
   - **Export.** Every export applies W-13 §3.5's disposition policy
     (`library_import_document/src/lib.rs:345-410`).
7. **Re-apply and version drift.**
   - **Checks.** On project load and on library-list refresh, the desktop's
     Rust side reads the store for each `library_ref` in the document and
     compares record digests. This needs the store, so it is not part of the
     store-free S1b validator.
   - **Warnings.** A changed digest reports `MODEL-LIBRARY-REF-VERSION-CHANGED`
     (W-13). A missing library or record reports `MODEL-LIBRARY-REF-UNRESOLVED`
     (new). Both are warnings, and the model's values stand.
   - **Re-apply.** "Re-apply from library" builds `apply_library_record` with
     the full diff and the new digest. It is never automatic. Deleting a library
     changes no model value.
   - **New project id.** The store is keyed by project id, so after "Save as
     new project" the references read unresolved until the library is
     re-imported into the new project. The warning says so.

## 3. Desktop UX

- The Library Manager gets an "Apply to model" section for an opened stored
  library: a record list, then Insert as new, or Apply to the selected entity.
  It shows a preview with two columns: copied (value, unit) and not copied
  (reason).
- Insert and Apply call the existing batch review. The panel gains an
  `onApplyBatch` prop, wired in `App.tsx`.
- The inspector's Provenance record section (W-13 S2) shows the library
  reference, "Source review" (never acceptance, F-PIP-2), and the drift or
  unresolved warning with "Re-apply from library".

## 4. Tests

- **Vitest, mapping tables.** One table per kind over the invented fixtures:
  copied and not-copied sets, refusal of `unit.MPa`, a string magnitude, a
  refused insert with no G, and a refused component kind mismatch.
- **Vitest, batch.** Validate, apply, then undo restores the previous model.
  Re-apply updates `library_version`.
- **Applier.** `apply_library_record`:
  - it authors absent members;
  - it refuses a dimension mismatch, a stale `before` and a kind mismatch;
  - `create_material` and `create_section` accept `provenance_record`.
- **Privacy.** After an apply, the saved document contains no allowable id, no
  non-copied slot id and no library metadata name, except within
  `library_ref`. The hanger batch passes the same check.
- **Drift.** A digest change raises the version warning. A deleted record
  raises the unresolved warning.
- **W-13 checks.** W-13 §5.4's round trip and §5.2's emission golden each
  include one library-applied entity.

## 5. Relationship to W-13 S2

W-13 §3.5 says "The desktop sets it when it inserts a library record (S2)". S2's
scope (§6, §8.3) adds only generic `provenance_record` carriage on the existing
change kinds. It designs no insertion flow, field mapping, record digest or drift
check. **This item is the work that sentence assigns to S2.** It depends on S1b,
which adds the slot to the format, and on S2's applier carriage of
`provenance_record`. Recommendation: S2 records the sentence as delivered by this
item, and this item lands as its own slice right after S2. That keeps S2 bounded.

## 6. Files, overlap and sequencing

| File | This item | W-13 S1b / S2 | Collision |
|---|---|---|---|
| `features/library/LibraryManagerPanel.tsx`, new `features/library/libraryApply.ts` | apply UI and mapping | none | no |
| `features/hanger-selection/hangerSelection.ts` | short provenance plus `provenance_record` | none | no |
| `apps/desktop/src/App.tsx` | `onApplyBatch` wiring at `:905` | S2 panels | low, textual |
| `apps/desktop/src-tauri/src/lib.rs` | store-side reference and digest check | S1b: self-test model (`:4482`), open-path validator | same file, different code; no |
| `core/model_operations/operation_applier/src/lib.rs` | `apply_library_record`; `provenance_record` on create kinds | S2: `provenance_record` on the same change kinds; S1b: version literals and validator | **yes (S2)** |
| `schemas/model.schema.yaml` | `library_version` meaning (§2.5) | S1b owns the members | **yes: text for S1b** |
| `fixtures/model_document/desktop_emitted.json` | one entry | S2 creates the golden | yes (S2) |
| D3-pinned files | none | S1b's D3 chain | no |

**Sequencing.**
- **Not before S1b.** `provenance_record` does not exist in 0.6.0, and the S1a
  schema's closed records refuse it.
- **A string-only interim** would leave drift undetectable, and the hanger fix
  would have to be done twice.
- **Before S2 merges,** `libraryApply.ts` and its tests can be written against
  the S1b schema, but they land after S2.

**Proposal relayed to the model-and-states manager** (W-13 §3.5 and the S1b schema description): "`library_version`
is `sha256:` and the SHA-256 of the applied record's RFC 8785 form. Library
documents carry no version member."
