# NOTICE 2026-09-22 — DEC-046 translation-delta unit is off by 1000

Status: raised for the owner. No code, record or threshold was changed.

## Finding

`validation/benchmarks/nonlinear/src/lib.rs`,
`force_displacement_residual_observation_with_policy`, reports

```rust
max_abs_translation_delta_from_previous: final_residuals
    .max_abs_translation_delta_from_previous
    .map(|value| value * 1000.0),
translation_delta_unit: "mm",
```

The nonlinear fixtures are already in fixture-local millimetres
(`NONLINEAR_FIXTURE_UNIT_BASIS.support_displacement_unit = "mm"`,
`status = fixture-local-explicit-units-no-conversions`). In
`NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL`, `EA / L = 100` N/mm and the clearance is
`0.05` mm: the free trial displacement is `0.1` mm and the closed state is
`0.05` mm, so the final translation delta is `0.05` mm. The observation reports
`50.0 mm`. The force-reaction deltas are unscaled and consistent with the
stiffness (`5.0` N for `0.05` mm at `100` N/mm).

Measured at base `b3e2ce4ec` (`assembled_force_displacement_residual_observations()`):

| Fixture | Reported translation delta | Force-reaction delta | Physical delta |
|---|---:|---:|---:|
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | 100.0 "mm" | 10.0 N | 0.1 mm |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | 50.0 "mm" | 5.0 N | 0.05 mm |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | 100.0 "mm" | 10.0 N | 0.1 mm |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | 40.0 "mm" | 4.0 N | 0.04 mm |
| `NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL` | 100.0 "mm" | 10.0 N | 0.1 mm |
| `NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL` | 30.0 "mm" | 3.0 N | 0.03 mm |
| `NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL` | 30.0 "mm" | 3.0 N | 0.03 mm |

`NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` and
`NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` converge in one iteration and
report no delta.

Separately, `convergence_observations.md` records `100.0 mm` / `10.0 N` for
`NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL`; the current measurement is `40.0` /
`4.0 N` (inside the limits). That ledger row appears stale.

## Where the figures are governed

- `validation/hand_calcs/nonlinear/convergence_observations.md` (observation
  table and seed thresholds `100.0` / `50.0` mm).
- `validation/benchmarks/nonlinear/displacement_reaction_delta_policy.dec046.json`
  (`translation_delta_absolute_limit` `100.0` / `50.0`, unit `mm`) and its
  observation record.
- `validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_policy.dec046.json`
  (`100.0`, unit `mm`) and its observation record.
- The release-scope record `release_convergence_policy.dec046.c-b.json` covers
  the active-set count only and does not contain these delta limits.

The product preview has its own DEC-046 records
(`DEC-046-CV-B-product-preview-displacement-reaction-delta-*`, limit `50.0` mm
in `core/product_physics/src/lib.rs`) and also multiplies the translation delta
by 1000 before labelling it `mm`. There the solve is in SI metres, so that
conversion appears correct. It was not assessed further and is not part of
this finding.

## Why it is held

The limits were set from the observed envelope, and the observation and the
limit carry the same factor of 1000, so every check is internally consistent
and passes. The defect is the unit: the enforced envelope is `0.1` / `0.05` mm
of physical displacement, while the records state `100.0` / `50.0` mm. Anyone
who applies the governed limits at face value to unscaled millimetre deltas
would allow 1000 times the observed envelope. Removing the
scaling, relabelling the unit or restating the limits changes protected DEC-046
criteria, which needs an owner decision (`AGENTS.md`, "Never weaken a protected
test, tolerance, oracle or limit"; the records' own loosening rule).

## Options for the owner

1. Remove the `* 1000.0` and restate the governed limits in true mm
   (`0.1` / `0.05`), same physical envelope, as a new DEC-046 governance event.
2. Keep the scaled numbers and relabel the unit `um` in the observation code
   and all governed records.
3. Leave as is and record the unit discrepancy as a known limitation.

No swbpipe.org page or validation-manual case record states these figures.
