# Applying library records to model entities (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

**Proposed definitions** (for W-13 §3.5 and the S1b schema description): the `library_version` definition (`sha256:` and the SHA-256 of the record's RFC 8785 form, §2.5) and the warning `MODEL-LIBRARY-REF-UNRESOLVED` (§2.7).

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
     - Superseded for records without a stored G by the coordinator's decision
       (§7): insertion derives G = E / (2(1+ν)) and records the derivation in
       provenance. The implementer reconciles the modes in this section with §7.
   - **Apply to an existing material** keeps that material's basis by default.
     Changing the basis is a separate, explicit choice: the diff shows the
     `constitutive_basis`, `poisson_ratio` and `shear_modulus` members being
     added or removed.
   - **A missing member fails the mode.** If the chosen mode needs a member the
     record lacks, the apply is refused and names the member: G for E/G, unless
     the record holds E and ν and the user chose the E/ν route (§7); ν for E/ν.
     No other value is derived.

   **3b. Component source kinds.** Mapping is by an exhaustive table from source
   `field_kind` to a destination path and type. A library slot holds either
   `value` (a scalar `QuantityValue`) or `reference_value` (a typed
   `Reference`) (`schemas/component.schema.yaml:405-420,740-760`).

| Source `field_kind` | Destination | Type |
|---|---|---|
| `bend_radius`, `bend_angle`, `branch_run_size`, `branch_header_size`, `branch_connection_angle`, `branch_reinforcement_area`, `rigid_body_length`, `end_a_size`, `end_b_size`, `weight`, `effective_area`, `movement_limit` | `geometry.<same name>` | scalar `value` → `Quantity` |
| `sif_user_value`, `flexibility_factor_user_value` | `modifiers.<same name>` | scalar → `Quantity` |
| `sif_in_plane_user_value` and `sif_out_of_plane_user_value` | `modifiers.<same names>` | scalar → `Quantity`; a pair: both copy or neither does; realized curved bends only (below) |
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
   - **Source references are updated, never left stale.**
     - **Why it matters.** The solver requires a source reference for each
       component family's group: `bend_geometry_source_reference` with bend
       geometry, and `modifiers.source_reference` with bend modifiers and the
       directional SIF pair (`core/product_physics/src/validation.rs:1485-1507,1996-2017`).
       It also requires the branch and rigid references (`:1551,1586,1636,1670`).
       An unchanged string would credit the new values to the previous source.
     - **Groups and their replaceable members.** A group is a family's
       geometry block or `modifiers`. Its *replaceable members* are exactly
       the §2.3b destinations that lie in that group, and nothing else:
       - **bend geometry:** `bend_radius`, `bend_angle`;
       - **branch geometry:** `branch_run_size`, `branch_header_size`,
         `branch_connection_angle`, `branch_reinforcement_area`;
       - **rigid geometry:** `rigid_body_length`, `end_a_size`, `end_b_size`,
         `weight`;
       - **expansion-joint geometry:** `effective_area`, `movement_limit`;
       - **modifiers:** `sif_user_value`, `flexibility_factor_user_value`, the
         directional SIF pair, `linear_stiffness_user_value`,
         `rotational_stiffness_user_value`.
     - **Never removed.** Placement and topology references are never removed
       by any apply option: `node`, `bend_pipe_ref`, `branch_header_pipe_ref`,
       `branch_branch_pipe_ref`, `rigid_pipe_ref`, `expansion_joint_pipe_ref`,
       and every other string or reference slot. Removing `bend_pipe_ref`, for
       example, blocks a realized bend (`core/product_physics/src/lib.rs:8406-8414`).
       Non-replaceable values such as `center_of_gravity` and
       `stiffness_scaling_user_value` are never removed either.
     - **"Replace group."** This option removes only the group's replaceable
       members that the library does not supply, and the diff shows each
       removal.
       - **Required members are protected.** A removal of a member the solver
         requires for the component's family is refused, naming the member
         (`LIBRARY-APPLY-REQUIRED-MEMBER`). The required members are:
         - bend: `bend_radius`, `bend_angle` (unless realized),
           `sif_user_value`, `flexibility_factor_user_value`
           (`validation.rs:1484-1507`);
         - branch: the run size, header size and connection angle
           (`:1523-1550`), and `modifiers.flexibility_factor_user_value`
           (`:1582`; without it branch modifier generation returns `None`,
           `lib.rs:12858`), with a refusal test for removing it;
         - rigid: `rigid_body_length`, the end sizes and `weight`, and the
           linear and rotational stiffness modifiers (`:1606-1640,1660-1680`).
     - **The source-reference string.** When an apply writes any member of a
       group, it rewrites that group's string (the diff shows the change):
       - **every value in the group was written by this apply:** the string
         becomes `library:component:<library_id>#<record_id>`;
       - **the group still holds retained values** (such as a rigid
         element's `center_of_gravity`, or a required member the library does
         not supply): the string becomes `<previous reference>;
         library:component:<library_id>#<record_id>: <written members>`.
         Retained members keep their previous attribution and are not
         credited to the library, and written members are credited to it;
       - **retained values with no previous reference** refuse the apply
         (`LIBRARY-APPLY-RETAINED-UNSOURCED`, naming them). "Replace group" can
         remove such a value only if it is replaceable and not required.
   - **The directional SIF pair needs a realized bend.** It applies only when
     the target bend is realized as the curved-bend macro element
     (`is_curved_bend_macro_component`, `core/product_physics/src/lib.rs:8349-8353`).
     Otherwise the solver blocks the pair
     (`BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC`, `validation.rs:1996-2004`). On a
     geometry-only bend or any other component, the pair is listed as not
     copied, with that reason.
4. **Units.**
   - Values copy in their entered unit (DEC-018, entered units preserved).
     Nothing is converted.
   - A material `unit_ref.ref_id` resolves to its symbol through the desktop's
     unit catalog (`unit_id` → `symbol`). An id the catalog does not hold, such
     as the fixture's `unit.MPa`, refuses the apply with
     `LIBRARY-APPLY-UNIT-UNRESOLVED`.
     - The resolver is shared with the rule-binding fix (DEL-06-02 Design
       `rule-pack-unit-check.md` §2.7). This caller is stricter: library apply
       requires a recognised catalog id, while rule binding passes
       non-`unit:` strings through.
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
       and `review_status` are the most restrictive found across these levels:
       - the library metadata, the record, every copied slot and every copied
         value's own provenance;
       - **every level's `privacy_class`.** A class beginning `private` raises
         the combined `redistribution_status` to at least `private_only`. That
         is how the importer's public policy treats it, independently of the
         status fields (`library_import_document/src/lib.rs:381-392`), so a
         `private_user_supplied` record with `public_permissive` provenance
         never comes out public;
       - **for an apply to an existing entity,** the entity's previous
         `provenance_record`, when any governed value of the entity is kept.
         Examples: a retained `thermal_expansion_coefficient`, temperature
         points or expansion laws, each an independent member of the material
         (`core/product_physics/src/lib.rs:799-815`), or unwritten component
         slots. An entity with no previous record counts as `unknown` and
         `pending`. Downstream handling is decided only from recorded
         dispositions (`library_import_document/src/lib.rs:345-410`).
       - **Retained values under quarantine or protection are not allowed.**
         The apply is refused while any retained value of the entity carries
         `protected_suspected` or `quarantined`
         (`LIBRARY-APPLY-RETAINED-QUARANTINED`, naming the retained values).
         The combination has no state that could carry them, and the
         downstream policy depends on those dispositions
         (`library_import_document/src/lib.rs:345-410`). The user's alternative
         is "replace all values".
       - **Dropping the retained restrictions.** The previous restrictions drop
         out only when the user chooses "replace all values". That choice
         removes every governed member the apply does not write, and the diff
         shows each removal. For components it obeys the same limits as
         "replace group": it never removes references, non-replaceable values
         or required members, and it is refused if it would. If such a retained
         member is quarantined or suspected protected, the apply cannot
         proceed.
       - When retained values remain, `verification_note` records that the
         entity also keeps values from its previous source.

       Each level carries its own provenance
       (`schemas/material.schema.yaml:490-504`; the import walks nested
       values, `library_import_document/src/lib.rs:425-432`).
       - **Rejected and protected sources are never used.** A library or
         record with redistribution or review `rejected` refuses the apply. A
         slot or value with `rejected`, `protected_suspected` or `quarantined`
         is excluded and listed. This matters because the importer's nested
         walk checks only unit metadata and that value-level provenance exists,
         not dispositions (`library_import_document/src/lib.rs:425-458`).
       - **Normalization.** `TBD` or a missing redistribution status becomes
         `unknown`. `TBD` or a missing review status becomes `pending`.
       - **The combined result stays inside W-13 §3.5's vocabulary.**
         - Redistribution, most to least restrictive: `private_only`,
           `unknown`, `public_permissive`. That is three of W-13's four states
           (`model-document-0.7.0.md:331`); `protected_suspected` never
           arises, because those sources are excluded.
         - Review: `pending`, `accepted`. The library schema's `TBD`
           (`schemas/material.schema.yaml`, `ReviewStatus`) never reaches the
           model.
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
  - the §7 derivation: an E/ν record without G inserts G = E / (2(1+ν)) with
    provenance naming the derivation and its E and ν sources; a stored G is
    used and never overwritten;
  - basis kept on apply-to-existing;
  - a refused component kind mismatch;
  - `center_of_gravity` excluded;
  - a duplicate `field_kind` refused;
  - the in/out-of-plane SIF pair copied only together.
- **Vitest, provenance:**
  - conflicting dispositions between parent and child, each way: the combined
    status is the most restrictive;
  - a quarantined slot: a partial apply with the slot excluded;
  - a private retained `thermal_expansion_coefficient` with a public incoming
    E/G: the result stays `private_only`. With "replace all values", alpha is
    removed and the result is public;
  - a `private_user_supplied` record with `public_permissive` provenance: the
    result is `private_only`;
  - an entity whose previous record is `quarantined`, applied with values
    retained: refused, naming the retained values. With "replace all values"
    it proceeds, and the diff shows each removal;
  - a value-level `rejected` slot is excluded, and a record-level `rejected`
    refuses the apply;
  - `TBD` normalizes to `unknown` and `pending`.
- **Vitest, components:**
  - an absent or stale `modifiers.source_reference` is replaced by the library
    reference;
  - **a realized bend:**
    - apply a record supplying only `sif_user_value` with "replace group":
      `bend_pipe_ref` is kept and the removal of
      `flexibility_factor_user_value` is refused, naming it;
    - without "replace group": the modifiers string becomes the combined
      form, and `flexibility_factor_user_value` keeps its previous
      attribution;
  - **a rigid element:** apply `weight` and the end sizes. `center_of_gravity`
    is kept, and the geometry string attributes it to the previous reference
    and the written members to the library. A rigid element with no previous
    reference is refused (`LIBRARY-APPLY-RETAINED-UNSOURCED`);
  - the directional SIF pair is not copied onto a geometry-only bend, and is
    copied onto a realized curved bend.
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
`provenance_record`. S2's sentence is delivered by this item, as its own slice,
which keeps S2 bounded.

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

**Dependencies.**
- **Requires S1b.** `provenance_record` does not exist in 0.6.0, and the S1a
  schema's closed records refuse it.
- **A string-only interim** would leave drift undetectable, and the hanger fix
  would have to be done twice.
- **Requires S2's applier carriage** of `provenance_record`; `libraryApply.ts`
  and its tests are written against the S1b schema.

**Proposed definition** (W-13 §3.5 and the S1b schema description): "`library_version`
is `sha256:` and the SHA-256 of the applied record's RFC 8785 form. Library
documents carry no version member."

**Proposed W-13 §3.5 changes.** As of this design, W-13 §3.5's sentences are
unchanged. The proposed changes:
- **Digest comparison.** The library-version comparison becomes a record-digest
  comparison. `MODEL-LIBRARY-REF-VERSION-CHANGED` compares the digest of the
  stored record with `library_version`.
- **Narrow the payload sentence.** "The library payload never enters the
  document" becomes "Only the enumerated applied values (DEL-03-01 Design
  `apply-library-records.md` §2.3), their units and the combined
  `provenance_record` enter the document; no other library content does."

## 7. Decision: the E/ν route

The question was whether insertion must always use a stored G, or whether the
user may also choose the E/ν route. The coordinator decided on 2026-10-23:
"allow the explicit **E/ν route**. For an isotropic material record that
supplies E and ν but no G, derive G = E / (2(1+ν)), record in the inserted
record's provenance that G was derived (and from what), and never overwrite a
stored G. Records with a stored G use it. That's standard practice for
isotropic piping materials, and the provenance keeps it auditable."

So:
- The route is the user's explicit choice; nothing is inferred from which
  properties a record holds.
- A stored G is used and never overwritten.
- A derived G's provenance names the derivation and the record's E and ν
  values and units it came from.
- The implementation settles how §2.3a's E/G and E/ν forms carry
  this (in particular whether the inserted material also carries ν and the
  exact basis) against the material validator.

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
   refusal of duplicates, and the SIF pair (§2.3b). Source references were
   later made explicit per group (round 2, item 8; round 3, item 12).
4. **Record-level provenance could drop stricter value-level restrictions
   (P1).** Resolved by conservative combination across library, record, slot
   and value; no promotion; exclusion of quarantined slots; and tests (§2.5,
   §4).
5. **The W-13 §3.5 wording.** Reconciliation requested above; not yet
   completed.

Codex round 2 (on `63615afe87`):

6. **R2-1, partial apply could drop restrictions on retained data (P1).** The
   previous record's dispositions are kept whenever governed values remain,
   unless the user chooses "replace all values" (§2.5).
7. **R2-2, `privacy_class` was dropped (P1).** It now enters the combination
   and raises the result to at least `private_only` (§2.5).
8. **R2-3, stale component source references (P2).** Source references are now
   updated per group, a group that keeps members from an earlier source carries a
   combined attribution (§2.3b), and the directional SIF pair is
   gated on a realized bend (§2.3b).
9. **R2-4, `rejected` was outside W-13's vocabulary (P2).** Rejected and
   protected sources are excluded, `TBD` is normalized, and the result stays
   inside W-13 §3.5's states (§2.5).
10. **The shared unit resolver was not strict enough for library apply.**
    Library apply requires a recognised catalog id (§2.4).

Codex round 3 (on `cb49504802`):

11. **Retained quarantine dispositions had no defined result (P1).** The apply
    is now refused while a retained value is `protected_suspected` or
    `quarantined`, unless the user chooses "replace all values" (§2.5).
12. **"Replace group" could remove required geometry (P2).** Replaceable
    members are now listed per group; references and placement are never
    removed; removing a required member is refused; retained members keep
    their previous attribution (§2.3b).
13. **Item 3's wording** no longer says source references are not written.
