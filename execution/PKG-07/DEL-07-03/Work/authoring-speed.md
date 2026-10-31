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

**Remaining (G5b).** Adding or changing the coefficient on an existing E/ν material that lacks
one follows separately.
