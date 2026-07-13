# Procedure: DEL-04-02 Straight pipe element

## Purpose

Define the bounded maintenance and verification procedure for the implemented straight pipe element without expanding its mechanics scope.

## Prerequisites

- Confirm the task names `DEL-04-02` and its authorized write scope.
- Confirm upstream architecture basis constraints from `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, and `AB-00-08`.
- Confirm the current `core/solver/straight_pipe` unit/domain and frame-kernel interfaces; unresolved integration items remain `TBD`.
- Confirm all example dimensions, material values, and fixtures are synthetic, public-domain, or otherwise cleared.

## Steps

1. Re-read `_CONTEXT.md`, `Specification.md`, and `_DEPENDENCIES.md`.
2. Identify the accepted module boundary for the straight pipe element and its relation to the global frame kernel.
3. Identify required input contracts for section properties, material/mechanical values, units, weight hooks, diagnostics, and result envelopes.
4. Maintain only the authorized straight-pipe local element behavior; do not broaden into stress-code or professional-acceptance logic.
5. Add explicit findings for missing solve-required values and unit mismatches; do not apply silent defaults.
6. Add deterministic solver tests with synthetic or cleared inputs.
7. Verify recovered element forces remain mechanical results and do not claim code compliance.

## Verification

| Check | Expected evidence |
|---|---|
| Scope boundary | Implementation changes are limited to the approved implementation scope for DEL-04-02. |
| Unit behavior | Unit-aware tests pass and dimensional mismatches fail explicitly. |
| Missing input behavior | Missing solve-required properties produce findings. |
| Solver result behavior | Element force recovery is deterministic for accepted synthetic/cleared cases. |
| IP/data boundary | No protected tables, formulas, examples, material allowables, or proprietary data are added. |

## Records

- Implementation or maintenance run records for authorized code work.
- Solver test results.
- Fixture provenance notes.
- Protected-content review evidence where applicable.
- Any human rulings for `TBD` items.
