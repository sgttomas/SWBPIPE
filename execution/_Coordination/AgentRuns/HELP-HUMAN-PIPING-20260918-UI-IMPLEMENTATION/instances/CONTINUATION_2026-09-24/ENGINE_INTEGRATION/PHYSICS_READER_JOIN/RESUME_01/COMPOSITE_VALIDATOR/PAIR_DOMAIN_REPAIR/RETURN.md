# CR01b material pair-domain repair

Bounded Type 2 TASK return to `/root/physics_resume/physics_reader_join`. No delegation. Only the two composite validator modules and their owned tests were edited; evidence is confined to this directory. No producer, unit authority, original fixture, old validator, semantic table, resource policy or numerical tolerance changed. No build, Cargo, npm, browser, native or Git operation was launched.

## Preserved red and supplied basis

`BEFORE_FREEZE_CHECK.json` verifies all 36 paths matched the prior `REPAIR_SUPPLEMENT/MANIFEST.json` before editing. `BEFORE_0` through `BEFORE_3` preserve owned preimages; `BASIS.json` records actual role/brief and producer-source hashes. The original reviewer finding and rehashed false-copy script are preserved. `RED_REPLAY.json` reproduces the public reader accepting a lower bracket with normalized E = the smallest positive binary64 value and nu = 0.25, whose producer-derived G is zero, because the final interpolated E remained 2000 Pa. This is deliberately false copied evidence, never a real producer invocation or native proof.

## Repair

Both validators now check the producer `pressure_exact::IsotropicENu::new` domain for the base pair, each consumed lower/upper interpolation pair and the final selected pair. Diagnostics retain the responsible source stage: `base`, `lower_bracket`, `upper_bracket`, or `selected`.

The helper uses normalized binary64 operands supplied by the unchanged unit authority. Within -1 < nu < 0.5, the ordered `2 * (1 + nu)` denominator is positive normal and has the same binary64 value as the producer's Scaled add/multiply sequence. E and the denominator are decomposed into mantissa in [0.5,1) and exponent; the mantissa quotient is normalized again. Python uses exact `frexp`, while Rust uses the producer's bit decomposition, including subnormal fraction normalization.

Publication exactly preserves `Scaled::to_f64`: exponent below -1074 yields zero, exponent above 1024 yields infinity, exponent 1024 uses the split `(mantissa * 2^1023) * 2`, and the remaining range multiplies by an exactly represented power of two. Final G must be finite and positive. A surviving positive subnormal G remains admissible; the check does not impose a normal-only material range or use a naive direct `E / denominator > 0` test.

`GREEN_REPLAY.json` now rejects the original false copy at `ACTUAL_DERIVED_SHEAR_RANGE:lower_bracket`. No source/result label was rewritten and no interpolation endpoint was defaulted.

## Verification and limits

- `python-tests.log`: 73 focused composite tests pass in 7.43 seconds. They retain the actual producer/unit positives, add adjacent underflow/tie/overflow boundaries and positive-subnormal controls, test every consumed source stage, and reject rehashed false lower/upper bracket copies whose final interpolated E remains finite.
- `SCALED_SEQUENCE_COMPARISON.json`: a scalar diagnostic compares the helper to the literal general Scaled add/multiply/divide/publication sequence for 1,010 deterministic nu inputs and six E boundary/ordinary values (6,060 pairs), with zero mismatches. This is not a producer execution or independent review claim; maintained adjacent-boundary tests are the regression contract.
- Python syntax, Rust parser/format and scoped whitespace checks pass. Rust compilation/tests were not run because parent owns the build lane.
- `REPAIR.diff` and `SOURCE_HASHES.json` bind the four-file repair. Final Python validator SHA256 is `81487a87d5e170b98b1569a8d2e7c42943b9c1be565f3a71c5b5a18879c4aba1`; Rust is `c4e380ee7c01eb1581930013e8fc77f8be37c68e1978dedbb78ab5d3cd656f37`.

Source is ready for parent integration, allocated Rust checks and independent backcheck. These outstanding checks are not closed by the Python pass or scalar diagnostic. The parent owns TS parity and the next frozen supplement.
