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
