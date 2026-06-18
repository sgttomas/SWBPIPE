# WORKING_ITEMS RUN - TP-UNITS-BTAIL-COMPTOLCORPUS-001

- **Date:** 2026-06-17
- **Persona:** WORKING_ITEMS
- **Supporting deliverable:** DEL-02-02 Unit system and dimensional-analysis core contract
- **Primary deliverables:** DEL-14-04, DEL-14-05
- **Tranche:** TP-UNITS-BTAIL-COMPTOLCORPUS-001
- **SMOKE:** TP-MAC-194

## Supporting Scope

Support the comparison mixed-unit tolerance corpus by recording unit-system
evidence for explicit conversion-factor use, missing-conversion diagnostics,
and DEC-026 relative+absolute tolerance-pair behavior.

## Changes

- The analysis-run comparison corpus now proves stress normalization from
  `kPa` to `Pa`, force normalization from `lbf` to `N`, and a blocking
  diagnostic when a required conversion is omitted.
- The tolerance profile remains caller/governance supplied; no unit catalog
  constant, conversion API default, or hidden conversion path was added.

## Validation

- `python3 tests/test_analysis_run_comparison.py` passed.
- `python3 tests/test_comparison_contracts.py` passed.
- Adjacent comparison pytest subset passed 23/23.
- Full Python suite passed 360/360.
- `git diff --check` passed.

## Boundary

Supporting unit evidence only. No DEC-018 catalog constant change, unit
conversion API defaulting, hidden conversion path, release threshold, solver
convergence policy, protected standards content, private data, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## Handoff

- Primary owner: DEL-14-04.
- Contract owner: DEL-14-05.
- DEL-02-02 role: unit-system and dimensional-analysis support evidence.
