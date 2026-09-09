# Endpoint section-cut repair metadata refinement V1

Status: **SUPPLEMENTAL ACCEPTANCE CONDITION** to `ENDPOINT_SECTION_CUT_REPAIR_BRIEF_V1.md`; the frozen mechanics algorithm is unchanged.

`schemas/results.schema.yaml:600-607,622-637` admits `element_local` and `recovered_from_local_element_stiffness`, but does not admit the product-runtime station strings `arc_section_frame` or `arc_section_equilibrium_from_assembled_end_forces`. The headless result-envelope binding currently copies metadata strings verbatim (`core/runner/headless/src/result_envelope_binding.rs:216-223`) rather than normalizing them. New endpoint rows must not extend that existing vocabulary mismatch.

For the bounded sign repair, keep the established public metadata enums: emit `coordinate_system=element_local` and `basis=recovered_from_local_element_stiffness` for corrected endpoint stress rows, including realized bends. Put the exact bend convention — local x is the endpoint arc tangent toward j, local z is the bend-plane normal, and local y is `z cross x` toward the arc center — in the required free-text `sign_convention`. This truthfully treats the arc section frame as the bend element's local section frame without a schema change made solely for descriptive granularity.

Focused acceptance must pass the existing headless result-envelope validation for a realized curved-bend case and assert that no endpoint result row introduces an unmapped metadata enum. The pre-existing curved-station metadata mismatch is outside this repair; record it as an existing limitation rather than copying it into new endpoint metadata or silently broadening the schema.
