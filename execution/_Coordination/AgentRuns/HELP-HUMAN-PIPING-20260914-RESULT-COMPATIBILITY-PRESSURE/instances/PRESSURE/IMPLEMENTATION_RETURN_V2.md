# PRESSURE representability repair return V2

Status: `SOURCE_FROZEN_FOCUSED_TESTS_PASS`; ready for root-owned independent V2 oracle refutation.

The repair remains inside the original two-file product fence. `lib.rs` is byte-for-byte unchanged from V1. `pressure_exact.rs` retains the private API and now preserves representable final stress/load values across the independently confirmed intermediate underflow, upward-rounded subnormal product, force-term overflow/cancellation, and finite-strain subtraction-overflow families.

## Repaired formulas

- Lamé radial stress uses the dimensionless factored radius expression `-p*(ri/r)^2*((ro-r)/(ro-ri))*((ro+r)/(ro+ri))`. The constant transverse trace uses `2p/[d/ri*(2+d/ri)]` for walls through `ro<=2ri`, and the equivalent bounded `2p*(ri/ro)^2/[1-(ri/ro)^2]` form for thicker walls. Neither path reconstructs normal stress from rounded area or fluid force.
- Axial and applied-eigen terms cancel in stress space with fused multiply-add before multiplication by wall area. A direct-force identity remains only as a subnormal-stress fallback, so it can recover a representable load without making separately overflowing terms fatal.
- Three-factor products and scaled ratios select the first association whose intermediate binary exponent is closest to zero. The V1 maximum-absolute-result heuristic, which preferred an upward-rounded subnormal partial product, is removed.
- When two finite strains overflow in direct subtraction, the mechanical strain is normalized by their maximum magnitude before multiplying by modulus. The routed case now returns `sigma_z=2^24`, `Nw=S=3*pi*2^24`, and the unchanged applied-RHS pair `[+3*pi*2^23,-3*pi*2^23]`. Root confirmed the initially routed negative i-entry was a transcription error, not a sign-contract change.

## Retained failures and regression result

The V1-derived failures were retained before repair: two lost-stress cases, two upward-rounded three-factor cases, two force-term overflow/cancellation cases, and the later finite-strain subtraction-overflow case. Exact commands, observed values/errors, the sign correction, and two authoring-wrapper failures are recorded in `_run_records/V2_PRE_REPAIR_FAILURES.txt`.

The final locked private-module run passes all 24 co-located tests with 149 unrelated tests filtered out. This includes the 17 V1 tests and all seven V2 regressions. `rustfmt --check` passes. The isolated Cargo target remains `/tmp/chirality-pressure-cargo-target-20260914` and is excluded from handoff.

## Frozen source

- `core/product_physics/src/lib.rs`: `aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15`
- `core/product_physics/src/pressure_exact.rs`: `8533622eaa7d87c102791d60bc9eb3607d6daf119a2da41cbfe866e89e4d283f`

The machine-readable inventory is `SOURCE_FREEZE_V2.json`. The canonical focused output is `_run_records/V2_FINAL_FOCUSED_TEST_OUTPUT.txt` with SHA256 `930282e145d478a8ee7956baffa0b61569cff34f15a27347698866434e95a93b`.

## Handoff boundary

This remains a dormant pure kernel. No runtime call, topology inference, solver assembly/recovery, DTO/schema/version, public units/material path, native/browser/network behavior, or Git state was changed. Root owns source copying, independent V2 oracle-to-product refutation, broader integration checks, and final acceptance.
