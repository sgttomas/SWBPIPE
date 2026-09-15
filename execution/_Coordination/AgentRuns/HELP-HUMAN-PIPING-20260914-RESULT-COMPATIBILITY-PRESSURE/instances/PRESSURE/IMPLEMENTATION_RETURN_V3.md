# PRESSURE scaled-arithmetic repair return V3

Status: `SOURCE_FROZEN_FOCUSED_TESTS_PASS`; ready for root-owned independent V3 oracle refutation.

V3 replaces the V2 local association heuristics and rounded-area fallbacks with one private `Scaled` representation. Each value carries a signed normalized binary64 mantissa and an integer binary exponent through geometry derivation, products, quotients, affine sums, and fused multiply-add. Conversion to binary64 occurs only for constructor admission/getters and each requested final output.

## Resulting mechanics path

- `ExactAnnulus` still exposes the same rounded positive area getters and applies the same positive-finite admission predicate. It also retains private scaled `Ai` and `As` derived from the validated radii. Later mechanics use those scaled values rather than the rounded getters.
- Lamé radial stress evaluates the factored squared-radius identity entirely in scaled arithmetic. The `ri/r` square therefore remains available when it is subnormal before multiplication by large pressure. Transverse trace is the scaled identity `2P/As` using precise scaled `P` and `As`.
- Axial stress subtracts strains, adds the Poisson term, and performs cancellation in the scaled representation. Wall force, fluid force, and effective force remain scaled until each output conversion.
- Applied eigen and cap pairs use scaled `P`, `EAs`, thermal strain, and Poisson terms directly. The applied-RHS signs are unchanged.
- Final conversion uses exact power-of-two construction. It returns finite normal/subnormal values, including rounding upward to the minimum subnormal, preserves signed zero when a smaller result rounds away, and lets the existing operation error reject only final infinity.

The representation retains binary64 significand precision; it is not arbitrary precision. Its purpose is to prevent premature exponent-range loss and rounded getter reuse while preserving the frozen binary64 tolerance and domain.

## Retained failures and tests

The frozen V2 failures were reproduced before repair. For the tiny annulus, V2 returned cap magnitude `2.381798647991063e-263` instead of `2.4942137116196084e-263`. For the wide exponent-span annulus, V2 returned radial `-0` instead of `-1.472728039589318e-90`. Commands, residuals, and frozen limits are recorded in `_run_records/V3_PRE_REPAIR_FAILURES.txt`.

The final locked module run passes all 27 co-located tests: the complete V1/V2 set, both V3 independent families, both pressure signs, adjacent pressure-exponent variants, and final gradual-underflow/signed-zero conversion. Result: 27 passed, 0 failed, 0 ignored, 149 filtered out. `rustfmt --check` passes.

## Frozen source

- `core/product_physics/src/lib.rs`: `aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15`
- `core/product_physics/src/pressure_exact.rs`: `baa83a5a62ea34d0677a5c5c5d6cd2fd2f977b6b62a5e2d4ddd06fa18951abc0`

The machine-readable inventory is `SOURCE_FREEZE_V3.json`. The canonical focused output is `_run_records/V3_FINAL_FOCUSED_TEST_OUTPUT.txt`, SHA256 `0db411ee6393e628888283a7e2787da0a7ca72d59bcc21198cee8c7cdf58cfeb`.

## Handoff boundary

`lib.rs` remains byte-identical to V1/V2 and the module remains dormant. No runtime call, topology inference, solver assembly/recovery, public DTO/schema/version, material migration, dependency, native/WASM/browser/network action, or Git state was changed. Root owns source copying, independent V3 oracle-to-product refutation, broader checks, and acceptance.
