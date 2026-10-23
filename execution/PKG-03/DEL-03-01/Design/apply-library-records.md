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
   - **Applying to a shared section updates its bound pipes.** Pipes bound by
     `section_ref` cache the section's diameter and wall thickness, and the
     solver refuses a stale cache (`SECTION_REFERENCE_CACHE_STALE`,
     `core/product_physics/src/lib.rs:8793-8802`).
     - On Section, `apply_library_record` reuses the existing propagation: it
       calls `section_bindings::propagate` (`section_bindings.rs:187`), as the
       Section `set_field` path does (applier `lib.rs:1580-1596`).
     - Each bound pipe's retained supplements are validated by that propagation
       (mill tolerance and corrosion allowance are rechecked against the new
       geometry, `section_bindings.rs:70-85,131-140`).
       One failure refuses the whole change.
     - The preview lists every dependent pipe row, and one undo restores the
       section and all its pipes together.
3. **The fields that copy.** Copying is fixed by table. Anything else is listed
   to the user as "not copied".

| Kind | Copies | Not copied (reason) |
|---|---|---|
| Material | `name`→`label`; per the chosen mode (§2.3a): `elastic_modulus`, `shear_modulus` or `poisson_ratio`, and `thermal_expansion_coefficient`. Only properties with no `temperature_ref` and a `value_status` of `provided` or `private_user_supplied` | the property not used by the chosen mode; `density` (the model holds density per pipe, `section.material_density`); strengths, `allowable_stress` and allowables (rule checks read them from the library at run time); temperature-referenced properties (library temperature points carry no temperature value); `specific_heat`, `thermal_conductivity`, `user_defined` (no model slot) |
| Section | `name`; `section_type` `pipe` only; the `outside_diameter` and `wall_thickness` dimensions → `properties` | Every other dimension or property. A bound section must hold only those two properties, or the solver refuses it (`core/product_physics/src/lib.rs:8751-8758`). Allowances and insulation are pipe fields. |
| Component | Only the source kinds in table §2.3b | Every other kind (§2.3b) |
| Hanger | As `hangerSelection.ts:42-51` maps today | Unchanged |

   **3a. Material modes.** The user chooses one mode, and nothing is inferred
   from which properties a record happens to hold.
   - **E/G mode** (the default) writes E and G with no `constitutive_basis` and
     no `poisson_ratio`; ν is listed as not copied. Both create_material and
     the material validator require exactly this form: a material carrying ν or
     a basis is treated as exact (applier `lib.rs:4410`), and the legacy form
     requires G (`pressure_authoring.rs:66-91`).
   - **E/ν mode,** chosen explicitly, writes E, ν (unit `1`) and
     `constitutive_basis: "homogeneous_isotropic_E_nu_v1"`, the exact-material
     form that `validate_create_material` requires (`pressure_authoring.rs:93-107`,
     through `validate_material` `:71-84`).
     - G is written only if the record holds it, as a retained value. On the
       exact route the solver derives G from the selected E/ν pair and warns
       that a retained G is ignored (`core/product_physics/src/pressure_material.rs:61-92`, warning at `:86`).
     - That derivation is the solver's, on an explicitly selected basis. The
       apply itself never computes G.
   - **Apply to an existing material** keeps that material's basis by default.
     Changing the basis is a separate, explicit choice: the diff shows the
     `constitutive_basis`, `poisson_ratio` and `shear_modulus` members being
     added or removed.
   - **A missing member fails the mode.** If the chosen mode needs a member the
     record lacks (G for E/G; ν for E/ν), the apply is refused and names the
     member. No value is derived.

   **3b. Component source kinds.** Mapping is by an exhaustive table from source
   `field_kind` to a destination path and type. A library slot holds either
   `value` (a scalar `QuantityValue`) or `reference_value` (a typed
   `Reference`) (`schemas/component.schema.yaml:405-420,740-760`).

| Source `field_kind` | Destination | Type |
|---|---|---|
| `bend_radius`, `bend_angle`, `branch_run_size`, `branch_header_size`, `branch_connection_angle`, `branch_reinforcement_area`, `rigid_body_length`, `end_a_size`, `end_b_size`, `weight`, `effective_area`, `movement_limit` | `geometry.<same name>` | scalar `value` → `Quantity` |
| `sif_user_value`, `flexibility_factor_user_value` | `modifiers.<same name>` | scalar → `Quantity` |
| `sif_in_plane_user_value` and `sif_out_of_plane_user_value` | `modifiers.<same names>` | scalar → `Quantity`; a pair: both copy or neither does |
| `linear_stiffness`, `rotational_stiffness` | `modifiers.linear_stiffness_user_value`, `modifiers.rotational_stiffness_user_value` | scalar → `Quantity` |

   - **Excluded, with the reason shown:**
     - `center_of_gravity`: the library holds a scalar; the model holds a vector
       `{x, y, z, unit}` (`core/product_physics/src/lib.rs:507`).
     - The string slots (`bend_plane_orientation`, `branch_connection_type`,
       every `*_reference` and `*_source_reference`, `hardware_reference`,
       `manufacturer_reference`): the library's form is a typed `Reference` or
       a scalar, while the model's is free text of another meaning.
     - `outside_diameter`, `wall_thickness` (pipe-section fields).
     - `bend_centerline_radius`, `bend_included_angle`, `bend_tangent_length`,
       the end orientations, `branch_size`, `hardware_flag`, `user_defined`,
       `TBD`.
   - **Eligible slots** have `value_status` `user_supplied`,
     `private_user_supplied`, `public_permissive_reviewed` or
     `invented_non_engineering_example`. `missing`, `protected_suspected` and
     `TBD` are never copied. Section dimensions use the same statuses; the
     section-only `calculated` is not copied.
   - **Ambiguous sources are refused.** Two slots of the same `field_kind` in
     one record refuse the whole apply (`LIBRARY-APPLY-AMBIGUOUS-SOURCE`,
     naming the kind).
   - **No source references are written.** The entity's `provenance_record`
     covers its modifiers (W-13 §3.5), so `modifiers.source_reference` and the
     `*_source_reference` strings are left as they are.
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
     - **source fields** (`source_name`, `source_location`, `source_license`,
       `contributor`, `contributor_certification`) come from the record's
       provenance;
     - **dispositions are combined conservatively.** `redistribution_status`
       and `review_status` are the most restrictive found across four levels:
       the library metadata, the record, every copied slot and every copied
       value's own provenance. Each level carries its own provenance
       (`schemas/material.schema.yaml:490-504`; the import walks nested
       values, `library_import_document/src/lib.rs:425-432`).
       - Redistribution, most to least restrictive: `protected_suspected`,
         `rejected`, `private_only`, `unknown`/`TBD`/missing,
         `public_permissive`.
       - Review: `quarantined`, `rejected`, `pending`/missing, `accepted`.
       - A child's status never promotes the parent, or the reverse;
     - **source differences are noted.** When a copied slot's source fields
       differ from the record's, `verification_note` names those slot ids, so
       they can be traced in the library;
     - `library_ref` = `{library_kind, library_id, library_version, record_id}`.
   - **A slot quarantined or protected at slot or value level is never
     copied.** It is listed as excluded, and the rest of the apply proceeds
     (partial application), with the combined disposition taken over what was
     copied plus the library and the record.
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
- For a material, the user chooses the mode, E/G or E/ν (§2.3a), before the
  preview. For a section, the preview lists every bound pipe the change updates.
- Insert and Apply call the existing batch review. The panel gains an
  `onApplyBatch` prop, wired in `App.tsx`.
- The inspector's Provenance record section (W-13 S2) shows the library
  reference, "Source review" (never acceptance, F-PIP-2), and the drift or
  unresolved warning with "Re-apply from library".

## 4. Tests

- **Vitest, mapping tables.** One table per kind over the invented fixtures:
  - copied and not-copied sets, refusal of `unit.MPa`, a string magnitude;
  - E/G and E/ν insert (the E/ν payload carries the basis and ν with unit `1`);
  - a refused E/G insert with no G, and a refused E/ν insert with no ν;
  - basis kept on apply-to-existing;
  - a refused component kind mismatch;
  - `center_of_gravity` excluded;
  - a duplicate `field_kind` refused;
  - the in/out-of-plane SIF pair copied only together.
- **Vitest, provenance.** Conflicting dispositions between parent and child,
  each way: the combined status is the most restrictive. A quarantined slot
  gives a partial apply with the slot excluded.
- **Vitest, batch.** Validate, apply, then undo restores the previous model.
  Re-apply updates `library_version`.
- **Applier, Section.** `apply_library_record` on a section bound by two pipes:
  - both caches update;
  - an allowance that fails the new wall thickness refuses the whole change;
  - one undo restores the section and both pipes;
  - the solver reports no `SECTION_REFERENCE_CACHE_STALE`.
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
| `core/model_operations/operation_applier/src/section_bindings.rs` | reused (`propagate`); no edit expected | none named in W-13 §6 | no |
| `core/model_operations/operation_applier/src/pressure_authoring.rs` | E/ν create path, used unchanged | S1b: version literals (`:149,285`) | no |
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

**W-13 §3.5 reconciliation,** requested from the model-and-states manager:
- **Digest comparison.** The library-version comparison becomes a record-digest
  comparison. `MODEL-LIBRARY-REF-VERSION-CHANGED` compares the digest of the
  stored record with `library_version`.
- **Narrow the payload sentence.** "The library payload never enters the
  document" becomes "Only the enumerated applied values (DEL-03-01 Design
  `apply-library-records.md` §2.3), their units and the combined
  `provenance_record` enter the document; no other library content does."
- **Assign the work explicitly.** "The desktop sets it when it inserts a library
  record (S2)" names the work: the model-and-states manager implements
  insertion, apply, re-apply and drift checks per this design, as part of S2 or
  right after it.

## 7. Owner question

**For insertion, should a stored G always be required, or may the user also
choose the E/ν route?**
- **G always required:** only E/G-mode insertion is offered. Records without G
  cannot be inserted, though ν stays visible in the library.
- **The E/ν route as well:** the user may choose E/ν mode. The material is then
  exact-basis (`homogeneous_isotropic_E_nu_v1`), and on the exact route the
  solver derives G and ignores any retained G.

Either way the basis is written explicitly and never inferred. The design
supports both. Until the owner answers, the implementation offers E/G only, and
E/ν waits.

## 8. Review findings resolved

The Codex review of `1f1b0d7fa9` found these, and the design above resolves
them:
1. **ν implied exact authoring without a basis (P1).** Resolved by the
   explicit E/G and E/ν modes, the basis kept on apply, and no inference
   (§2.3a, §7).
2. **Shared-section apply left bound-pipe caches stale (P1).** Resolved by
   reusing `section_bindings::propagate`, validating each pipe's supplements, a
   preview of every dependent row, and atomic undo (§2.2).
3. **Component mapping by field name (P2).** Resolved by the exhaustive
   source-to-destination table, explicit exclusions, eligible statuses,
   refusal of duplicates, the SIF pair, and no source references written
   (§2.3b).
4. **Record-level provenance could drop stricter value-level restrictions
   (P1).** Resolved by conservative combination across library, record, slot
   and value; no promotion; exclusion of quarantined slots; and tests (§2.5,
   §4).
5. **The W-13 §3.5 wording.** Reconciliation requested above.
