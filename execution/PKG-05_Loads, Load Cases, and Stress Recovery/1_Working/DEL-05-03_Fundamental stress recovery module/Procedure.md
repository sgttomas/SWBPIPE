# Procedure: DEL-05-03 Fundamental stress recovery module

## Purpose

Define the bounded procedure for maintaining, reviewing, and extending the implemented fundamental stress recovery module without expanding beyond the approved mechanics-only deliverable boundary.

## Prerequisites

- Confirm the sealed brief names `DEL-05-03` and write scope is this deliverable folder or a separately approved implementation scope.
- Confirm upstream architecture basis constraints from `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-06`, and `AB-00-08`.
- Confirm the current implementation evidence in `core/loads/stress_recovery/README.md` and `core/loads/stress_recovery/src/lib.rs`.
- Confirm unit metadata, result-boundary, station-resultant, and result-envelope interfaces to be used; unresolved production integration items remain `TBD`.
- Confirm all example force resultants, section properties, pressure values, and fixtures are synthetic, public-domain, or otherwise cleared.

## Steps

1. Re-read `_CONTEXT.md`, `Specification.md`, and `_DEPENDENCIES.md`.
2. Re-read `core/loads/stress_recovery/README.md` and `core/loads/stress_recovery/src/lib.rs` before changing deliverable docs or code.
3. Identify the accepted module boundary for mechanics stress recovery and its relation to solver result recovery, load cases, rule packs, reports, GUI, and local FEA handoff.
4. For maintenance work, preserve the implemented mechanics-only API behavior unless the sealed brief explicitly authorizes a bounded change.
5. For new behavior, identify required input contracts for axial force, bending moments, torsion, pressure, section properties, station resultants, units, diagnostics, ranges, and result envelopes.
6. Add or preserve explicit findings for missing solve-required values, invalid/non-finite values, non-positive properties, unit metadata defects, incomplete statuses, blocked states, and status-boundary violations; do not apply silent defaults.
7. Add deterministic tests with synthetic or cleared inputs for any changed behavior.
8. Verify recovered stresses remain mechanical results and do not claim code compliance, release acceptance, or professional approval.

## Verification

| Check | Expected evidence |
|---|---|
| Scope boundary | Implementation changes are limited to the approved implementation scope for DEL-05-03. |
| Unit behavior | Unit metadata tests pass and missing/wrong dimensions fail explicitly; conversion constants remain outside this crate. |
| Missing input behavior | Missing solve-required force, pressure, section-property, status, range, or unit metadata inputs produce findings. |
| Stress result behavior | Recovered mechanics stress components are deterministic for accepted synthetic/cleared cases. |
| Station behavior | Station recovery preserves station identity and fraction; station sweeps preserve caller order and deterministic indexed IDs. |
| Range behavior | Mechanics-only ranges are component-wise deltas between two unblocked stress states and do not imply fatigue/code acceptance. |
| IP/data boundary | No protected tables, code formulas, examples, material allowables, or proprietary data are added. |

## Records

- Implementation notes or pull request summary when code work is authorized.
- Cargo test output for `core/loads/stress_recovery`.
- Hand-calc or benchmark test results where applicable.
- Fixture provenance notes.
- Protected-content review evidence where applicable.
- Any human rulings for `TBD` items.
