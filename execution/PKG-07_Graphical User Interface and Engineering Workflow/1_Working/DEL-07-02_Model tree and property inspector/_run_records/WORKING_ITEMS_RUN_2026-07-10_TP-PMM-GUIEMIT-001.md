# WORKING_ITEMS Run Record - TP-PMM-GUIEMIT-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded Type-2 implementation worker; owner-directed session)
Deliverable: DEL-07-02 - Model tree and property inspector
Package: PKG-07 - Graphical User Interface and Engineering Workflow
Tranche: TP-PMM-GUIEMIT-001
Target stage: R5 / physical-model mechanics program GUI-emission tranche
(rulings `DEC-068` and `DEC-070`, `execution/_Decomposition/SOFTWARE_DECOMP.md`
§12; plain residual named by the TP-PMM-P3-* run records: "GUI/editors do
not yet emit either")

## Deliverable Selection Basis

The direct precedent TP-R4-D4-EJSTIFF-001 recorded its GUI slice under
DEL-03-06 because it was expansion-joint-component-scoped. This tranche is
not component-scoped: it adds entry and emission surfaces across four slot
scopes (bend components, pipe spans, load cases twice) and touches only the
model tree / property inspector / model view editors. DEL-07-02 is the
deliverable that owns those editors and already carries every prior
property-inspector/model-tree GUI tranche (29 run records, e.g.
TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001), so the run record lives here.

## Scope

Desktop-GUI absorption of the four user-entered schema slots landed by the
TP-PMM P1-P3 engine tranches, following the EJSTIFF architecture (types,
PropertyInspector/ModelTree entry, model-view review rows, emission and App
tests). The GUI can now ENTER each slot (structured editor-operation
intents through the DEC-020 / ADR-0001 seam) and EMITS entered values
verbatim in the preview-model JSON it writes, omitting every slot the user
left empty:

1. `components[].geometry.bend_pipe_ref` on bend components (DEC-070
   curved-bend span mapping; expected shape per `core/product_physics`
   blocking diagnostics).
2. `pipe_segments[].section.mill_tolerance` (user-entered absolute
   thickness reduction; absence means no reduction, not zero).
3. `load_cases[].modulus_basis_ref` (DEC-068 item 1; exact selection, no
   interpolation — D-38 remains AWAITING_RULING and no interpolation UI
   semantics were added beyond naming the stored user-entered point id).
4. `load_cases[].equivalent_static` seismic/wind occasional-load-generation
   inputs (DEC-068 item 2: gravity acceleration, per-axis g-factors, wind
   pressure, shape factor, global-axis direction token, user-marked
   exposed spans).

All four slot groups start EMPTY/absent, carry no default, catalog value,
code coefficient, physical constant, or computed suggestion, and are
omitted from emitted JSON when un-entered.

## Files Touched

- `apps/desktop/src/types.ts` (`PreviewComponent.geometry.bend_pipe_ref`;
  pipe-section slot documentation; `load_cases[].modulus_basis_ref` and
  `equivalent_static`; new `EquivalentStaticGenerationInput` /
  `SeismicEquivalentStaticGenerationInput` /
  `WindEquivalentStaticGenerationInput` types mirroring the
  `core/product_physics` preview input surface)
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx` (editable
  intent fields: "Bend pipe" on bend components; "Mill tolerance" on pipe
  spans with explicit unit payload; "Modulus basis", seismic gravity +
  per-axis g-factors, wind pressure/shape factor/direction/exposed spans
  on load cases, all `update_load`)
- `apps/desktop/src/features/model-tree/ModelTree.tsx` (search keywords for
  the new slots; layout-grid columns: pipes "Mill tol.", components
  "Bend pipe", load cases "Modulus basis" + seismic/wind columns)
- `apps/desktop/src/features/model-workspace/modelView.ts` (review rows;
  optional-slot rows appear only when a value is present — an absent
  `bend_pipe_ref` is additionally surfaced as TBD when the component
  requests `curved_bend_macro_element` solver consumption, where its
  absence is engine-blocking)
- `apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx` (new;
  10 tests: empty-start pins, intent-emission per slot, review-row
  display/omission, canonical model-JSON emission/omission through the
  wasm engine's RFC 8785 serialization, and schema-binding checks that
  read `schemas/model.schema.yaml` / `schemas/section.schema.yaml` from
  disk)

## Recorded Design Choices

- **Entry goes through the structured-operation seam only.** Intents are
  queued/validated/applied via `core/model_operations/operation_applier`
  (DEC-020 / ADR-0001); no TypeScript-side model mutation was added. The
  applier's field-rule registry does not yet accept these paths (it
  answers `OP-FIELD-PATH-UNSUPPORTED`, the same honest blocked finding it
  returns today for every existing component-geometry inspector field such
  as `geometry.bend_radius.value` and the EJSTIFF fields). Extending the
  registry is `core/**` and out of this tranche's write fence; recorded as
  the core-side residual below.
- **Omission is the empty state.** Review rows for optional slots render
  only when a value exists, so the inspector's TBD-derived
  missing-field flags do not mark optional slots as missing.
  `bend_pipe_ref` is the one exception: when the component's
  `mechanics_interface.solver_consumption` is `curved_bend_macro_element`
  the row shows TBD (and flags), because the engine blocks on its absence
  in exactly that mode.
- **Preview-surface naming.** The emitted model JSON uses the
  preview-engine input names (`equivalent_static`, `g_factor_x/y/z`,
  `exposed_pipe_refs`, `direction: global_x|global_y|global_z`), which is
  what `core/product_physics` consumes. The canonical
  `schemas/model.schema.yaml` slots (`equivalent_static_generation`,
  `g_factors.x/y/z`, `exposed_element_refs`, `ForceDirection`) are the
  parallel surface (bend_pipe_ref precedent); the schema-binding tests pin
  the one-to-one vocabulary correspondence and validate the directly
  bindable constraints (Id pattern, dimension-token enum, section
  dimension-kind enum, quantity leaf constraints) against the real schema
  files.
- Display-unit metadata for empty quantity slots (e.g. `m/s^2` for the
  seismic gravity field, project pressure unit for wind pressure) is unit
  METADATA for the entry control, not a value default; the value stays
  TBD/absent until the user types one, and queueing is disabled until the
  value changes.

## Implemented Evidence

- Inspector entry: selecting the bend fixture component offers
  "Bend pipe" (before `TBD`, queue disabled until entry); queued intent
  emits `set_field` / `geometry.bend_pipe_ref` with the entered pipe ref
  verbatim. Pipe spans offer "Mill tolerance" with explicit
  `{value, unit}` JSON payload and length dimension. Load cases offer
  "Modulus basis" and the seven seismic/wind generation fields as
  `update_load` intents with acceleration/pressure/dimensionless
  dimension tokens.
- Review display: entered values echo verbatim in model-view rows
  (`Mill tolerance 0.0016 m`, `Bend pipe pipe:P-100`,
  `Seismic gravity 9.7 m/s^2`, wind rows, generation provenance); absent
  slots produce no rows; curved-bend mode surfaces absent `bend_pipe_ref`
  as TBD.
- Emission: the canonical model-JSON serialization (wasm engine RFC 8785
  `canonicalJsonString`, the same canonicalization used for model hashing
  and persistence payloads) carries every entered slot verbatim and omits
  every un-entered slot; the bundled fixture model contains none of the
  four slot keys (empty-start pin).
- Live GUI check on the vite dev server: bend component, pipe span, and
  load case selections each list the new entry fields in the intent
  editor; pipe property rows show no mill-tolerance row when absent.

## Checks

- `npm test` (`apps/desktop`, vitest): 20 files, 417 tests passed
  (407 prior + 10 new in `schemaSlotEmission.test.tsx`).
- `npm run build` (`tsc -b` + vite production build): passed; retained the
  pre-existing chunk-size warning.
- `npm run build:wasm`: passed (wasm-bindgen 0.2.123).
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep recorded on the clean head with this
  tranche (commit-bound summary under `validation/evidence/sweeps/`).

## Boundaries And Residuals

- **Apply-seam field rules are the core-side remainder.** The operation
  applier (`core/model_operations/operation_applier`, out of this
  tranche's write fence) does not yet list the new field paths in its
  registry, so applying a queued slot intent returns the explicit blocked
  finding `OP-FIELD-PATH-UNSUPPORTED` — identical treatment to every
  pre-existing component-geometry inspector field (bend radius/angle,
  EJSTIFF expansion-joint fields, rigid-component fields). The GUI entry,
  intent emission, review display, and model-JSON emission surfaces are
  landed; a core-scoped tranche should extend the registry (and the
  contract corpus) for these paths.
- Load-case creation shells (`create_load_case`) intentionally carry no
  new-slot payloads: the applier builds the canonical shell record itself,
  so slot values ride only on `update_load`/`set_field` intents and opened
  documents.
- Wind exposed-span marking is whole-span by pipe id (comma-separated
  entry), mirroring the engine input; sub-span marking remains the
  deliberately-uninvented extension recorded by TP-PMM-P3-OCCLOADGEN-001.
- No temperature-interpolation UI semantics (D-38 AWAITING_RULING): the
  modulus-basis field names a stored user-entered point id only.
- Canonical-vs-preview naming stays a parallel surface (bend_pipe_ref
  precedent); full canonical-document instance validation (Provenance
  blocks etc.) is not performed by the GUI and remains with the
  schema-owning deliverables.
- No mill-tolerance catalog fraction, seismic/wind code coefficient,
  exposure category, importance factor, gravity constant, or default
  ships; all values user-entered; unknowns remain TBD/absent.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
