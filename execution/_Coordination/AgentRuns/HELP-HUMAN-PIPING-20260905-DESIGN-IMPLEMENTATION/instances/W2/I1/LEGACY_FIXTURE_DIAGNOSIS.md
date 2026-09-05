# DEC-092 fixture metadata diagnosis

The unmodified crate baseline passed 124 tests. After coordinate normalization plus three new tests, 123 tests passed and four pre-existing DEC-092 tests failed; see baseline-cargo.log and after-cargo.log. New coordinate-equivalence, source-preservation, and invalid-metadata checks all passed. Initial failure history is retained.

The shared fixture `fixtures/product_preview/invented_dec092_temperature_g_request.json` has model.project.id but no units metadata. It uses an anchor at x=0 and tip at x=4. The existing public-original hand-calculation input table in `validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md` names `PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K` and explicitly declares L=4.0 m. The DEC-092 implementation record also requires explicit invented geometry/load/units. The four failures result from the new required-metadata check, not changed equations or expected mechanics values.

No explicit legacy implicit-SI solve contract was found. Current operation_applier code requires project.units.length for authored node coordinates. Proposed correction is the single metadata addition model.project.units.length=m using the already declared fixture-local metre basis. Numerical input values, expected results, source hand calculation, and prior immutable run evidence are unchanged. This addition was requested from the parent as an exact scope amendment; it is not authorized by the original sealed brief.

A separate self_weight module test deserializes a model without project units for load-intensity planning, not preview solve. Retaining raw/default JSON metadata preserves that deserialization behavior while enforcing metadata at the preview solve boundary.
