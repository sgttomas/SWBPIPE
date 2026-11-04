# D-01 — Engineering extension, 2026-11-04: `mass_length_per_rotation` and `time_squared`

| Field | Value |
|---|---|
| Record | Engineering extension of the unit catalog, beside [D-01](D-01_unit_catalog_acceptance.md). |
| Status | **Manager decisions under the engineering delegation**, confirmed by Agent 0 (2026-11-04) for DEL-04-09 I3a's modal result family. It is not an owner ruling: D-01, its sub-decisions D-01a–e and its ruling `DEC-018` are unchanged and are not reinterpreted. |
| Scope | Two routine family additions to `core/units`, made within D-01's accepted policy: an SI-coherent canonical basis (D-01a A2), exact definitional constants as governed f64 records (D-01b B1), and the two-tier witness policy (D-01e T1). No existing unit, dimension, symbol, factor or conversion changes. |
| Design | DEL-04-09 `execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3.5 |
| Change | Branch `claude/rt-dynamics-i3a-units`: `758716688` (`mass_length_per_rotation`), `194c1b9a8` (the modal result's use of it), `56a091408` (`time_squared`), `0307ba57d` (the modal result's use of it) |

## 1. The additions

| Dimension (`as_str`) | Vector (L, M, T, Θ, A) | Unit id | Symbol | To canonical | Provenance |
|---|---|---|---|---|---|
| `mass_length_per_rotation` | (1, 1, 0, 0, −1) | `unit:kilogram_meter_per_radian` | `kg*m/rad` | ×1 (canonical) | project-governed canonical binding |
| | | `unit:pound_mass_inch_per_radian` | `lb*in/rad` | ×(POUND_MASS_IN_KILOGRAMS · INCH_IN_METERS) = 0.45359237 · 0.0254 | exact public definition |
| | | `unit:pound_mass_foot_per_radian` | `lb*ft/rad` | ×(POUND_MASS_IN_KILOGRAMS · FOOT_IN_METERS) = 0.45359237 · 0.3048 | exact public definition |
| `time_squared` | (0, 0, 2, 0, 0) | `unit:square_second` | `s^2` | ×1 (canonical; no display unit) | SI canonical identity |

**Binding text.**
- `mass_length_per_rotation` is the translation–rotation coupling of a mass matrix whose rotations are in radians (kg·m·rad⁻¹). It is the unit of a consistent or rigid-body mass entry between a translational and a rotational degree of freedom, and of a rotational row of M·R_k.
- `time_squared` is the dimension of a diagonally scaled mass D·M·D, with D = diag(k_ii^−½) of the stiffness of the same pencil (s², entry by entry, whatever the degrees of freedom).
- Both are separate named dimensions, as model document 0.9.0's dynamics set is: checked by name and never matched by vector. Their conversions are definitional.

**Not added.** `angular_frequency_squared`. λ = ω² is not published, and ω carries the spectrum (Agent 0, 2026-11-04).

**History.** On the first pass the trace found the D·M·D magnitudes to be s², and this record held a `time_squared` family as an open item. Meanwhile those magnitudes were published as `catalog_family_absent`. Agent 0 chose option (a), the family, on 2026-11-04. The modal result now publishes them in `unit:square_second`, and the `catalog_family_absent` form is retired from modal-1.

## 2. Review basis

- **Dimensional trace** (Design §5.3.5, on the actual code).
  - Coordinates q_i are in m or rad. Through virtual work K_ij is in J/(q_i·q_j) and M_ij in J·s²/(q_i·q_j), so a translation–rotation mass entry is kg·m/rad.
  - Both the Sturm route (binary64) and the formation route (double-double) take D from the diagonal of the same formed K_ff. That is one normalization in units, d_i in J^−½·q_i, so D K D, D K̂ D and D(K − σM)D are dimensionless and D M D and D M̂ D are s².
  - λ and σ are in s⁻²; ω = √λ is published in rad/s through the catalog's radian convention, and f = ω/2π in Hz.
  - No published kind has mixed exponents, so modal-1's closed `equilibrated_basis` list is empty. The earlier rad^−½ rationale is kept in the Design as superseded provenance.
- **Conversion witnesses.**
  - `mass_length_per_rotation_units_have_definitional_conversion_witnesses`:
    - 1 lb·in/rad = 0.011521246198 kg·m/rad, and 1 lb·ft/rad = 0.138254954376 kg·m/rad, each converted both ways at the crate's tolerance;
    - the canonical unit, its id, symbol, provenance and vector;
    - refusal by name of `kg*m` for the dimension, of `kg*m/rad` for `mass` and `rotational_generalized_mass`, and of conversion to kg and to kg·m²/rad²;
    - three units in the dimension.
  - `time_squared_has_one_canonical_unit_and_is_matched_by_name`:
    - the canonical unit, its id, symbol, provenance and vector, and the identity conversion;
    - one unit in the dimension;
    - refusal by name of `s` for the dimension, of `s^2` for `time` and of `1` for the dimension, and of conversion to s and to 1.
  - The Python catalog (`tests/verifiers/units.py`) states the factors independently. `tests/test_verifier_independence.py` holds it to the crate's witness rows and to the catalog bit for bit (78 units, 36 canonical dimensions).
- **Mirrors.** `schemas/units.schema.yaml` `DimensionId`, `core/constraints/validation/engine.py` and `core/gui/pkg02_boundary.py` (`CANONICAL_DIMENSIONS`), and `tests/test_units_schema.py`.
- **Checks run.**
  - `core/units` tests (20);
  - `tests/test_units_schema.py` and `tests/test_verifier_independence.py`;
  - the qualification binding's units pin, `tools/validation/qualification_physics.py`, re-pinned deliberately for each addition.
- **Reviewers.**
  - Agent 0 confirmed the decisions (2026-11-04), and the dynamics manager issued the implementation briefs. The implementation is a TASK's.
  - **`mass_length_per_rotation` and the first trace, at `9b22b5a2b`:** an independent review commissioned by the dynamics manager found no blocking issue and one should-fix. The readers' empty-set rounding allowance did not cover a subnormal (2π·f_c)²; it was fixed in `0307ba57d`, with a producer-generated regression fixture.
  - **`time_squared`, the code-based trace and that fix:** independent review **pending**. It is recorded here when it completes.
- **D3.** `core/units/src` is priced. Its re-registration belongs to the I3a D3 turn (DEL-04-08), and this record registers nothing.

## 3. What this does not close

Units passing for the modal result's current documents closes no other condition. The mandatory Bessel relation (interim contract), the subject census, the production parameters and the other admission conditions of Design §5.3.9 stay open, and no current document is fully validated.
