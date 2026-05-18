# Procedure: DEL-04-05 Sparse solver performance harness

## Purpose

Describe how a future TASK worker should produce or use the sparse-solver performance harness once implementation is authorized, while preserving the current setup-only boundary.

## Prerequisites

- Accepted solver/kernel interfaces from PKG-04 implementation deliverables.
- Accepted unit-system and schema contracts for model fixtures and result envelopes.
- Human-approved sparse solver/library and diagnostic policy, if implementation requires one.
- Invented, public-permissive, or otherwise lawful benchmark fixtures with provenance.
- Human-approved performance/conditioning threshold policy; current value is `TBD`.

## Steps

1. Confirm the sealed brief authorizes implementation and lists exact write targets.
2. Read the accepted solver interface, result-envelope schema, diagnostics contract, and unit-system contract.
3. Select or create benchmark fixtures only from invented, public-permissive, or otherwise lawful data.
4. Record fixture provenance, model/hash basis where available, unit basis, solver version, and harness settings.
5. Run repeated deterministic cases through the solver interface without changing solver logic.
6. Capture performance observations, conditioning diagnostics, warnings, assumptions, and limitations.
7. Compare repeated outputs using the approved tolerance/threshold policy; if no policy exists, record `TBD` rather than failing or passing on invented criteria.
8. Emit run records suitable for regression review and release-gate evidence.
9. Stop and escalate if protected data, proprietary benchmark content, missing engineering values, or compliance/certification wording appears.

## Verification

| Check | Expected evidence |
|---|---|
| Scope boundary | Harness code/tests do not modify solver logic or repo-level artifacts unless explicitly authorized. |
| Determinism | Repeat-run comparison evidence is present; tolerance policy is cited or marked `TBD`. |
| Fixture provenance | Each fixture records source/provenance and redistribution status. |
| Unit safety | Fixture inputs and outputs pass accepted dimensional checks. |
| Diagnostic/reporting boundary | Results include warnings/assumptions/limitations and avoid certification or compliance claims. |

## Records

- Performance/regression run records.
- Fixture provenance index.
- Solver version and settings record.
- Conditioning diagnostic summary.
- Review notes for threshold changes and human rulings.
