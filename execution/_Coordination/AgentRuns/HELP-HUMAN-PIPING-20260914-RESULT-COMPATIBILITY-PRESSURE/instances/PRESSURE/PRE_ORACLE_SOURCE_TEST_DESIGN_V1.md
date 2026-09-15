# Private exact-pressure kernel — pre-oracle source/test design V1

Status: `READY_FOR_ORACLE`; provisional until checked against the independent root-frozen oracle and followed by an explicit source release. No product files have been edited.

## Source shape

The product patch will contain one private declaration, `mod pressure_exact;`, in `core/product_physics/src/lib.rs` and one new `core/product_physics/src/pressure_exact.rs`. The module will have no serde surface, DTOs, calls into the preview runner, normalizer, solver, recovery, or legacy pressure helper. All maintained tests will be in `pressure_exact.rs` under `#[cfg(test)]`.

The module will define crate-private validated values:

- `ExactAnnulus { ri_m, ro_m, ai_m2, as_m2 }`; `from_radii` requires finite `0 < ri < ro`, computes `Ai = PI*ri*ri` and the cancellation-resistant `As = PI*(ro-ri)*(ro+ri)`, then rejects zero, negative, or non-finite areas.
- `IsotropicENu { e_pa, nu, g_pa }`; `new` requires finite `E > 0` and `-1 < nu < 0.5`, derives `G = E/(2*(1+nu))`, and rejects non-positive or non-finite `G`.
- `InternalDifferentialPressure { p_pa }`; `new` accepts any finite signed increment, including zero and negative pressure, with the external reference fixed at zero by the type meaning.
- `RadialHoopStressPa { radial_pa, hoop_pa }` and `AxialState { wall_force_n, effective_force_n, axial_membrane_pa }` as crate-private return records.
- A small crate-private `ExactPressureError` enum separating invalid geometry/material/pressure/radius/strain inputs from non-representable derived geometry, material, load, or stress. Tests will assert error categories rather than strings.

`lame_at_radius` will require a finite radius in the exact closed interval `[ri, ro]`. It will evaluate `P=p*Ai`, `A=P/As`, and an algebraically equivalent cancellation-resistant radial expression using factored radius differences; hoop follows from the exact constant-sum identity. It introduces no thin-wall, radius-ratio, or topology tolerance. Every pair of distinct positive finite adjacent representable radii whose areas remain positive and representable is valid. Non-finite derived load or stress results reject. The returned values preserve `sigma_r=A-B/r^2`, `sigma_h=A+B/r^2`, inner traction `-p`, outer traction zero, and constant sum `2P/As`.

`axial_state` will require finite `thermal_strain` and `epsilon_z`, form a finite positive `EAs`, finite `P=pAi`, and then calculate exactly once:

```text
Nw = EAs*(epsilon_z-thermal_strain) + 2*nu*P
S  = Nw-P
sigma_z = Nw/As
```

It will reject every non-finite derived force or stress. Positive `Nw` and `sigma_z` mean tension.

`eigenload_pair(annulus, material, pressure, thermal_strain)` will calculate `epsilon0=thermal_strain-2*nu*P/(EAs)` and return `[-EAs*epsilon0,+EAs*epsilon0]`. `cap_pair(annulus,pressure)` will return `[-P,+P]`. Both are local node-on-element mathematical pairs, validate every derived value, and do not infer transfer topology. The pair helpers return `Result<[f64;2],ExactPressureError>` so arithmetic failure cannot appear as a solved load.

No function will subtract a cap pair from recovered wall force. No function will produce the legacy longitudinal-pressure term or use a mean-radius pressure area.

## Co-located test inventory

1. `annulus_accepts_exact_reference_geometry_and_adjacent_radii`: `ri=1, ro=2` gives `Ai=PI`, `As=3*PI`; `ro` as the immediate representable successor of `ri=1` is admitted and verifies the stable positive area expression without a minimum thickness ratio.
2. `annulus_rejects_invalid_and_nonrepresentable_geometry`: zero/negative/reversed/equal/non-finite radii plus underflowed `Ai` and overflowed `As` reject deterministically.
3. `material_derives_single_g_authority`: `E=120, nu=1/4` gives `G=48`; invalid bounds/non-finite values and a derived-G overflow reject. This kills an independent-G authority in this kernel.
4. `pressure_accepts_finite_signed_values_only`: positive, zero, and negative values pass; NaN and infinities reject.
5. `lame_matches_inner_outer_and_constant_sum_oracle`: reference inner `(-3,5)`, outer `(0,2)`, and an interior point satisfy `sigma_r+sigma_h=2`; radius below/above the wall and non-finite radius reject. A second adjacent-radii case checks inner and outer traction residuals against an applied-pressure scale, rather than the much larger `A/B` or hoop-stress scale, so cancellation cannot hide a bad boundary value.
6. `lame_is_linear_for_zero_and_signed_pressure`: zero produces zero and sign reversal negates both stresses.
7. `p1_free_transfer_matches_oracle`: `thermal=0`, `epsilon_z=1/240` yields `Nw=3PI`, `S=0`, axial stress `1`, eigen pair `[3PI/2,-3PI/2]`, and cap pair `[-3PI,3PI]`.
8. `p2_restrained_transfer_matches_oracle`: zero strain gives `Nw=3PI/2`, `S=-3PI/2`, axial stress `1/2`; distinguishes correct `2nuP` from omission, sign flip, or doubling.
9. `p3_free_separate_closures_matches_oracle`: `epsilon_z=-1/240` gives `Nw=0`, `S=-3PI`, axial stress zero while cap remains a separate mathematical pair; kills cap subtraction from wall recovery and omission of the `S` fluid term.
10. `p4_mixed_thermal_pressure_matches_oracle`: `thermal=1/1000`; free `epsilon_z=31/6000` gives `(Nw,S,sigma)=(3PI,0,1)` and restrained zero strain gives `(57PI/50,-93PI/50,19/50)`; eigen pair is `[57PI/50,-57PI/50]`.
11. `axial_reductions_and_linear_scaling_hold`: explicit `p=0`, `nu=0`, `thermal=0`, signed pressure, signed thermal strain, and additive linear-superposition checks.
12. `wall_and_effective_force_identities_are_independent`: checks `S=Nw-P`, `sigma_z=Nw/As` exactly once, and positive-tension sign; kills double-longitudinal and mean-radius-area mutations.
13. `local_pair_signs_and_conservation_hold`: each pair sums to zero, signed pressure reverses cap signs, and reference pair orientations match the contract.
14. `derived_load_and_stress_overflow_rejects`: finite constructors combined with extreme finite pressure/strain inputs must return errors for non-representable `P`, `Nw`, `S`, eigen/cap pair, or Lamé stress.

Numerical comparisons will use exact equality where the result is exactly representable and a local machine-epsilon assertion elsewhere. General identity checks use the magnitude of their expected physical quantity. Inner/outer traction checks specifically use applied-pressure scale, including a small absolute machine floor for zero pressure, and never scale by large Lamé coefficients or hoop stress. These assertions are test instrumentation only and do not introduce a product tolerance, physical applicability cutoff, or engineering acceptance threshold.

## Release checks after oracle acceptance

- Bind the independent oracle path and SHA-256 into the context manifest, then reconcile every oracle predicate and mutation against this inventory before editing source.
- Re-hash `lib.rs`, `Cargo.toml`, and all supplied contract/oracle inputs immediately before source effect; stop on unexplained drift.
- Implement only the two product paths in the sealed fence.
- Run formatting check and the focused locked `product_physics` test target with a unique isolated `CARGO_TARGET_DIR`; retain exact commands, exits, and raw outputs.
- Search for `pressure_exact` references outside the module declaration and co-located tests and verify no invocation from `run_linear_static_preview`, normalization, recovery, or load assembly.
- Freeze product diff, source hashes, test evidence, and known limits for root integration and independent full-diff review.

## Resolved routine API choice

Root accepted `eigenload_pair(annulus, material, pressure, thermal_strain)` and `cap_pair(annulus, pressure)`. Validated input fields remain private behind checked constructors; crate-private read-only accessors and crate-private return records are the accepted private/dormant shape.
