# Authoring speed and completeness (RT-G2, RT-G3; exit criterion 7)

This note records the desktop changes that close the authoring gaps in the reference-system run
sheet (DEL-09-01 `Work/reference-system-run-sheet.md` §6). Desktop paths are relative to
`apps/desktop/src/`. Every change still produces intents through the one operation route and
applies only on the person's explicit Apply (DEC-020).

## 1. Material-level thermal expansion coefficient (G5)

**Gap.** No GUI route created an E/ν material carrying a material-level
`thermal_expansion_coefficient`. The E/ν form had no field for it, and **New material** requires a
shear modulus and writes neither basis nor ν. The reference system's RS-M1 carries one, so it
could not be entered as recorded.

**Change.** The E/ν create form ("Create explicit E/nu material",
`features/pressure-authoring/PressureAuthoringPanel.tsx`) has an optional **Thermal expansion
coefficient** quantity. The engine already admitted the member on `create_material` and checks the
project temperature unit and the coefficient's unit (`operation_applier/src/lib.rs`,
`resolve_create_material`); no operation, schema member or engine rule changed.

- Left blank (value and unit both empty), the member is absent. Nothing is prefilled or defaulted.
- A value without a unit, or a unit without a value, is refused before queueing; it is never
  completed.
- The coefficient is recorded on the material. Thermal states that name an expansion law or their
  own coefficient use those, as before.

**Evidence.** `features/pressure-authoring/PressureAuthoringPanel.test.tsx` (entered, blank and
half-entered coefficient) and `createExactMaterial.engine.test.tsx`, which creates RS-M1 from the
form on a blank load/reference model, applies it through the wasm operation engine and compares the
material with `fixtures/reference_system/reference_system.json`.

## 2. The coefficient on an existing material (G5b)

**Gap.** An existing E/ν material's coefficient could not be added, changed or removed. The
constitutive projection (`operation_applier/src/pressure_authoring.rs` `MATERIAL_KEYS`) excluded it,
and `set_field thermal_expansion_coefficient.value` refuses a material that lacks the quantity
(`OP-QUANTITY-OBJECT-MISSING`). No design text, pricing registration or provenance rule gave a
reason for the exclusion; `create_material` and the library apply route already admit the member.

**Change.** The constitutive projection, in the engine and in the desktop's existing-material editor,
includes `thermal_expansion_coefficient`. The editor shows the field with E, ν and provenance;
clearing it and queueing removes the member, explicitly.

- When the coefficient is present, the engine requires the project temperature unit to be stated and
  to be a temperature unit (`OP-UNIT-METADATA-MISSING`, `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`),
  and the coefficient to be a finite quantity in an accepted thermal-expansion unit, as on
  `create_material`. Its sign is the user's. Nothing is converted or completed.
- A caller whose projection still omits the coefficient is refused as stale
  (`OP-BEFORE-VALUE-MISMATCH`) on a material that carries one. It is never read as a removal.

**Evidence.** `operation_applier/tests/exact_authoring.rs`
`constitutive_properties_add_change_and_remove_the_thermal_expansion_coefficient` (add, change,
unit and value refusals, project-unit refusals, the stale-projection refusal and removal);
`l_line_authoring.rs` and the desktop `lLineAuthoring.test.tsx` now use the full projection; the
desktop form test and the wasm round trip that adds RS-M1's coefficient to the existing material
(`createExactMaterial.engine.test.tsx`).

## 3. Fill and copy for per-member and per-case lists (G7, G8)

**Gap.** Reference-configuration members, element states, support states and load sources were
entered one row at a time, and each case's analysis state and pressure regions were entered again
for every case. On the reference system that was about 1,480 entries (run sheet §3.8, §3.10,
§3.11).

**Change.** Draft helpers beside the existing editors (`features/load-cases/bulkFill.ts`,
`BulkFillControls.tsx`):

| Control | Where | What it adds to the draft |
|---|---|---|
| Fill ⟨rows⟩ from a template | reference-configuration members, element states, support states | A copy of the chosen row for each target the list does not yet name: every pipe (or support) without one, an inclusive range in model order, or chosen targets. Existing rows stay unchanged and in place. |
| Add every primitive of this case | load sources | `{source_ref, factor}` for each of the case's primitives not yet listed, with the factor the person enters. |
| Copy analysis state from another case | analysis state | The other case's applied analysis state. Load sources naming primitives this case does not hold are left out and reported, never redirected. |
| Copy pressure regions from another case | pressure regions | The other case's applied regions. Where the source case's id token is a whole segment of a region id, it becomes this case's (`region:RS-SUS-RG-1` gives `region:RS-OPE-RG-1`); other ids are kept. |

The draft rules are unchanged:
- Every control edits only the draft. The person reviews it, and the editor still submits the whole
  record as one intent, applied only through Review and Apply. The load/reference-state editor has
  the engine validate that intent before queueing it. The pressure-region editor queues after its
  own form checks, and the engine validates the intent at Review and Apply.
- No operation kind or engine rule changed, so the embedded agent's catalog is unaffected.
- Nothing is prefilled. A fill copies only what the template row holds, so a temperature is present
  only where the person entered one in the template. Under condition use (model document 0.9.0
  §1.6–§1.7), a case that takes its temperature from conditions leaves `operating_temperature` absent
  in its template, and the fill keeps it absent.
- Every fill control starts unchosen, and its outcome is announced in a status line.
- A row's own members that the editor keeps as authored without showing them are never copied to
  another row or case: `analysis_basis_override` and `mass_state_ref` on element states, and
  `base_motion` and `device_reference` on support states. A support state with locked equivalent
  participation is not a template and is left out of a case copy. Each omission is named in the
  status line.
- The template is the chosen row itself. If the row is edited or removed, or the list resets, the
  choice lapses and must be made again.
- A copy whose source case is no longer offered, has no regions, or would give two regions one id
  is refused with its reason, and the draft is left unchanged.

**Evidence.**
- `features/load-cases/bulkFill.test.ts` covers the helpers, and `BulkFillControls.test.tsx` covers
  the lapsing template and the refused stale or failed copy.
- `referenceSystemFills.engine.test.tsx` enters the reference system's load/reference state with
  these controls and applies each intent through the wasm operation engine:
  - one reference member typed through the editor and 25 filled;
  - RS-SUS with one element state and one support state typed and filled, every primitive at
    factor 1, and RS-P180's provenance edited;
  - RS-SAM copied from RS-SUS, with its own primitives and RS-C's two motions added;
  - RS-OPE with one operating element state filled, and RS-P180 edited to its replaced-span state;
  - RS-SUS's five regions copied to RS-OPE and RS-SAM.

  The resulting document equals `fixtures/reference_system/reference_system.json`.

**Re-estimate (entry counts; the run sheet's owner converts them to minutes).** §3.8 goes from about
187 entries to about 14, §3.11 from about 1,047 to about 68, and §3.10 from about 246 to about 86
(RS-SUS's five regions typed once, two copies). Scrolling a 26-row list to the one row that differs
is not counted as an entry.
