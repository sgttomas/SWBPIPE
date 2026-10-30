# Local FEA handoff: manifest result units (W-17 slice c)

**Basis:** `b9fa486209` (2026-10-26). The governing design is DEL-02-02
`Work/project-unit-selection.md` §7.4 and §7.7. Standard claim fence applies
(F-PIP-2; claims taxonomy per DEC-081).

## What the desktop builder does

The desktop handoff builder is `buildLocalFeaHandoffPacket` in
`apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx`. It refers
to result rows and preserves their units. It carries no stress values.

**Before this change**
- `stress_unit` and `force_unit` came from the project keys.
- In the bundled demo, the handoff referenced the governing summary maximum
  `result:elastic-maximum:10:load:L-100:10:pipe:P-100` (Pa) and
  `result:stress:pipe-P-120:end-j:torsional-shear` (MPa), and declared
  `stress_unit: MPa`. The label disagreed with the referenced governing row.

**Since then (Agent 0's B-c, 2026-10-26)**
- Each of `stress_unit` and `force_unit` is the unit of the rows it labels.
  - The governing row decides, which for stress is the summary maximum.
  - With no governing row, the candidates' shared unit is used.
- Only rows in the declared unit are referenced.
- A candidate in another unit is left out and named in a non-blocking
  `LOCAL-FEA-RESULT-ROW-UNIT-OMITTED` diagnostic, with its id, its unit and the
  declared unit. Nothing is converted.
- If no governing row decides and the candidates disagree, the handoff is
  refused (`LOCAL-FEA-RESULT-UNIT-INCONSISTENT`).
- With no row of a family referenced, the member is the project key. A missing
  key refuses the handoff (`LOCAL-FEA-PROJECT-UNIT-MISSING`).
- The demo now declares `stress_unit: Pa` and no longer references the MPa
  torsional-shear row. The diagnostic names that row.
- The schema, `selectedResultRefs`' candidate list, the witnesses and their
  policy are unchanged. `moment_unit`, `displacement_unit` and `rotation_unit`
  remain literals equal to the referenced rows' units on the routes the panel
  is open to.

## Deferred improvements

- **B-a: stress witnesses with converted values.** Each referenced stress row
  would get a witness that keeps its source value and unit and adds the value
  converted to the declared `stress_unit`. Rows in other units, such as the
  demo's torsional-shear row, could then be referenced instead of omitted.
  - This needs a `local_fea_handoff.schema.yaml` change, a new witness family
    and its policy. It belongs to this deliverable.
  - The owner's ruling of 2026-10-26 permits routine conversion: "units
    conversions that are routinely done in engineering applications are
    permissible as a solution."
- **B2: every result-unit member from the referenced rows.** This covers
  `moment_unit`, `displacement_unit` and `rotation_unit`, which are literals
  today.
