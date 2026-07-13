# Procedure: DEL-09-01 Mechanics benchmark suite

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-09-01-DECL-004`.

## Purpose

Describe how a future TASK worker should produce or use the mechanics benchmark suite once implementation is authorized, while preserving the current setup-only boundary.

## Prerequisites

- Accepted frame-stiffness and coordinate-transform interfaces from PKG-04 implementation deliverables.
- Accepted straight-element mechanics behavior for cantilever-style cases.
- Accepted support, restraint, and imposed-displacement semantics for boundary-condition cases.
- Accepted load/thermal input semantics where thermal-growth benchmarks require them.
- Accepted unit-system, diagnostics, and result-envelope contracts for benchmark inputs and outputs.
- Original, public-domain, public-permissive, or otherwise lawfully redistributable benchmark sources with provenance.
- Human-approved numerical tolerance and release-gate policy; current value is `TBD`.

## Steps

1. Confirm the sealed brief authorizes benchmark implementation and lists exact write targets.
2. Read the accepted solver interfaces, unit-system contract, diagnostics/result-envelope contract, and applicable validation strategy.
3. Select or create benchmark fixtures only from original, public-domain, public-permissive, or otherwise documented lawful sources.
4. For each fixture, record benchmark family, source/provenance, redistribution status, assumptions, units, model inputs, expected result fields, and derivation notes.
5. Keep protected standards examples, proprietary commercial benchmarks, vendor-private data, and copied code-derived formulas out of public fixtures.
6. Run benchmark cases through the authorized solver or headless runner without changing solver logic.
7. Compare outputs using the approved comparison/tolerance policy; if no approved policy exists, record `TBD` rather than inventing pass/fail criteria.
8. Capture solver version, result-envelope fields, warnings, diagnostics, assumptions, limitations, and comparison records.
9. Stop and escalate if fixture provenance is missing, protected content is suspected, numerical tolerances are unsupported, or output wording implies certification, code compliance, or professional approval.

## Verification

| Check | Expected evidence |
|---|---|
| Scope boundary | Benchmark work stays in authorized validation/test locations and does not modify solver logic unless separately authorized. |
| Family coverage | Fixture inventory covers cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms or records an explicit `TBD`. |
| Fixture provenance | Every public fixture records source, license/redistribution status, and review disposition. |
| Unit safety | Inputs, expected values, and solver outputs pass accepted dimensional checks. |
| Tolerance authority | Comparison tolerances cite an approved source or remain `TBD`. |
| Diagnostics/reporting boundary | Results preserve warnings/assumptions/limitations and avoid certification or compliance claims. |

## Records

- Benchmark fixture inventory and provenance index.
- Hand-calculation or derivation notes for each case.
- Solver version and benchmark runner settings.
- Unit-check and result-envelope comparison records.
- Review notes for tolerance policy, fixture approval, and human rulings.

## D-41 R5 T2B PDU-013 Check

Record fixture-local unit identifiers and canonical dimensions exactly. Do not describe those checks as project-unit-system acceptance until the upstream catalog and conversions are accepted and the cases are rerun through that basis. Keep comparison tolerances and pass/fail authority unchanged.
