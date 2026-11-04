# D-01 — Engineering extension, 2026-11-04: `mass_length_per_rotation`

| Field | Value |
|---|---|
| Record | Engineering extension of the unit catalog, beside [D-01](D-01_unit_catalog_acceptance.md). |
| Status | **Manager decision under the engineering delegation**, confirmed by Agent 0 (2026-11-04) for DEL-04-09 I3a's modal result family. It is not an owner ruling: D-01, its sub-decisions D-01a–e and its ruling `DEC-018` are unchanged and are not reinterpreted. |
| Scope | One routine family addition to `core/units`, made within D-01's accepted policy: an SI-coherent canonical basis (D-01a A2), exact definitional constants as governed f64 records (D-01b B1), and the two-tier witness policy (D-01e T1). No existing unit, dimension, symbol, factor or conversion changes. |
| Design | DEL-04-09 `execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3.5 |
| Change | Branch `claude/rt-dynamics-i3a-units`, commits `758716688` (catalog) and `194c1b9a8` (the modal result's use of it) |

## 1. The addition

| Dimension (`as_str`) | Vector (L, M, T, Θ, A) | Unit id | Symbol | To canonical | Provenance |
|---|---|---|---|---|---|
| `mass_length_per_rotation` | (1, 1, 0, 0, −1) | `unit:kilogram_meter_per_radian` | `kg*m/rad` | ×1 (canonical) | project-governed canonical binding |
| | | `unit:pound_mass_inch_per_radian` | `lb*in/rad` | ×(POUND_MASS_IN_KILOGRAMS · INCH_IN_METERS) = 0.45359237 · 0.0254 | exact public definition |
| | | `unit:pound_mass_foot_per_radian` | `lb*ft/rad` | ×(POUND_MASS_IN_KILOGRAMS · FOOT_IN_METERS) = 0.45359237 · 0.3048 | exact public definition |

**Binding text.** `mass_length_per_rotation` is the translation–rotation coupling of a mass matrix whose rotations are in radians (kg·m·rad⁻¹). It is the unit of a consistent or rigid-body mass entry between a translational and a rotational degree of freedom, and of a rotational row of M·R_k. It is a separate named dimension, as model document 0.9.0's dynamics set is: checked by name and never matched by vector. Its conversions are definitional products of reviewed constants.

**Not added.** `angular_frequency_squared`. λ = ω² is not published, and ω carries the spectrum (Agent 0, 2026-11-04).

## 2. Review basis

- **Dimensional trace** (Design §5.3.5).
  - In modal-1's generalized-coordinate convention, M_ij is in kg·m²/(q_i·q_j), with each q_i in m or rad. A translation–rotation entry is therefore kg·m/rad.
  - The trace also covers the modal result's former `equilibrated_basis` magnitudes. Those on D(K − σM)D and DKD, with D = diag(|k_ii|^−½), are dimensionless (`unit:one`); those on DMD are time squared, s².
  - No published kind has mixed exponents, so modal-1's closed `equilibrated_basis` list is empty.
- **Conversion witnesses.** `core/units` test `mass_length_per_rotation_units_have_definitional_conversion_witnesses`:
  - 1 lb·in/rad = 0.011521246198 kg·m/rad, and 1 lb·ft/rad = 0.138254954376 kg·m/rad, each converted both ways at the crate's tolerance;
  - the canonical unit, its id, symbol, provenance and vector;
  - refusal by name of `kg*m` for the dimension, of `kg*m/rad` for `mass` and `rotational_generalized_mass`, and of conversion to kg and to kg·m²/rad²;
  - three units in the dimension.

  The Python catalog (`tests/verifiers/units.py`) states the same three factors independently. `tests/test_verifier_independence.py` holds it to the crate's witness rows (two more) and to the catalog bit for bit (77 units, 35 canonical dimensions).
- **Mirrors.** `schemas/units.schema.yaml` `DimensionId`, `core/constraints/validation/engine.py` and `core/gui/pkg02_boundary.py` (`CANONICAL_DIMENSIONS`), and `tests/test_units_schema.py`.
- **Checks run.** `core/units` tests (19); `tests/test_units_schema.py` and `tests/test_verifier_independence.py`; and the qualification binding's units pin, `tools/validation/qualification_physics.py`, re-pinned deliberately in this slice.
- **Reviewers.**
  - Agent 0 confirmed the decisions (2026-11-04), and the dynamics manager issued the implementation brief.
  - The implementation is a TASK's, under that brief.
  - **Independent review: pending.** The dynamics manager's review before this branch merges into `claude/rt-dynamics-i3a` is recorded here when it completes. Nothing above is review evidence.
- **D3.** `core/units/src` is priced. Its re-registration belongs to the I3a D3 turn (DEL-04-08), and this record registers nothing.

## 3. Open item (not decided here)

The two mass-side formation magnitudes of the modal result, ‖D(M − M̂)D‖_∞ and ‖DM̂D‖_∞, are time squared (s²), a dimension the catalog has no family for. They are published as `catalog_family_absent` with exponents (0, 0, 2, 0, 0). A further routine family, `time_squared` (`unit:square_second`, `s^2`, ×1 canonical), would be the same kind of manager decision under the engineering delegation, recorded the same way. Design §5.3.5 recommends it.
