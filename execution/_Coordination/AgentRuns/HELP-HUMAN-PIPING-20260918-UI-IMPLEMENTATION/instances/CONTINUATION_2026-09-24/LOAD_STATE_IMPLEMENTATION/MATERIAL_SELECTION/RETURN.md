# Per-member material selection — implementation return

Implemented the private pure selector in `core/product_physics/src/case_state/material.rs`, WORKING_ROOT-relative. Actual TASK `/root/physics_resume/exact_authoring`, parent `/root/physics_resume`, delegated-harness-native reused handle with this new bounded assignment; no descendants. This work used the selected immutable load/reference-state design at `9e8a55daecdeb9669131fd3e53c0e0303ee550d6`. It did not change the old case-wide resolver, model/material map, public schema/facade, another checkout or any Git state.

Private API:

```rust
select_for_member(
    material: &MaterialInput,
    selection: &MaterialSelection,
    operating_temperature: Option<&Quantity>,
    analysis_basis_override: Option<&AnalysisBasisOverride>,
) -> Result<ResolvedMemberMaterial, MaterialSelectionError>
```

All declared API types/functions are `pub(crate)`; they are not public DTOs. `MaterialSelection` has the agreed three closed variants: `ExplicitBaseProperties {material_ref, applicability_reference}`, `ExactPoint {material_ref, point_ref}`, and `TemperatureInterpolation {material_ref, temperature}`. The interpolation variant means piecewise linear with extrapolation forbidden. Parent's public adapter must reject unknown kinds/policies before mapping to it; no private unknown-kind fallback exists. `AnalysisBasisOverride` retains `reason` and `provenance`, both nonempty when supplied.

The returned record retains the actual material ID and provenance, selection kind, `pair: IsotropicENu`, separate `operating_temperature_k` and `selection_temperature_k`, consumed point records, interpolation fraction, applicability reference, override and `retained_g_ignored`. A consumed point retains its actual ID, normalized optional temperature, validated E/nu/derived-G pair, provenance and ignored-G flag. The caller retains the original authored input record for raw-unit evidence. No alias is synthesized and no map entry or input is mutated; two calls with the same material may return different per-member pairs.

Inputs may carry authored units. Conversion uses only the existing units crate (`unit_by_symbol`, `canonical_unit`, `convert_for_dimension`): E→Pa, absolute T→K. Finite checks follow conversion; negative absolute kelvin fails. Nu requires explicit unit `1`. Every consumed base/point/bracket endpoint and the interpolated result pass the existing `IsotropicENu` constructor, retaining its full representability domain. G is derived from the selected E/nu pair, never independently selected or interpolated. Recorded G and alpha are not used as substitutes.

Exact table temperatures—including endpoints—select that one explicit point. Interior requests consume the strict adjacent normalized bracket; incomplete pairs are rejected rather than skipped to a wider bracket. Duplicate normalized temperatures, ambiguous point IDs, unknown point/material references, missing pairs, extrapolation, invalid units and nonrepresentable pairs return typed errors. Only consumed property pairs are required; unrelated missing E/nu records do not block a valid exact point/endpoint.

Known actual and selected temperatures must agree exactly after existing normalization or carry the explicit override. Per parent's clarified design alignment, an undated exact point remains a deliberately selected fixed basis with selection temperature `None`; it does not require an override merely because its temperature is absent and does not prove applicability at actual T. Selected temperatures never manufacture an actual operating temperature. Explicit base selection always requires its applicability reference.

Checks: rustfmt/parser check passed; ten meaningful in-module tests are prepared. They cover cold/hot 200/100 GPa selections for one unchanged material, exact/interior source order, derived-G versus interpolated-G discrimination, equivalent units, duplicate T, missing consumed pairs, no extrapolation, wrong/unknown/duplicate point identity, actual/selected temperature distinction and override, undated points, fixed applicability, ignored recorded G, invalid units/basis/override, adjacent overflow/underflow limits, invalid endpoints hidden by an otherwise admissible midpoint, and unconsumed incomplete property data. Numerical assertions retain relative `1e-9`; exact binary/domain controls use exact comparisons. These tests have **not** been compiled or run by this TASK.

Source SHA-256: `1ae8ac998fce43bcdb2f2b1fdb8b0f03569fbe688fa28f420d5f578d37c239bd`. Machine-specific checkout/commands, source hash and actual parser output are in [`_run_records/CHECKS.json`](_run_records/CHECKS.json) and [`_run_records/PARSER_CHECK.log`](_run_records/PARSER_CHECK.log). Instruction and selected immutable design origins/hashes are in [`_run_records/STARTUP.json`](_run_records/STARTUP.json).

Parent integration: declare `material` under the owned `case_state` module; map the strict public selection and policy; pass the actual member material and operating T; use the returned pair consistently in member stiffness, pressure recovery and evidence. Parent owns coordinated compilation/runtime checks, actual per-member solve integration, unknown public DTO controls, source-recovery integration and independent review. This private selector and parser check do not establish connected/native qualification or completion of those capabilities.
