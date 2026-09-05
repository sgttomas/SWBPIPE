# B2 frozen UI interface v1
Parent acceptance AMENDMENT_B2.md. Implementation integrated; verification/review pending. All quantities remain explicit entered value/unit. Unknown nested fields reject.

Common envelope operation_kind modify, unit none, dimension dimensionless.
- Support target ref: change_kind update_support, field_path configuration. before RFC8785 canonical JSON projection of current family/restraints/stiffness/hanger/nonlinear/provenance keys, omit absent. after JSON object of same allowed keys. Omission removes optional config slots; identity/label/node unchanged. Existing create_support accepts rich family/stiffness/hanger/nonlinear fields using same validator.
- Material target ref: change_kind set_field, field_path temperature_points. before canonical current array or not_present when absent. after JSON array point objects with id, optional temperature, elastic_modulus, shear_modulus, thermal_expansion_coefficient, provenance. Incomplete unselected points remain explicit warning; used bases fail closed.
- Load target ref: change_kind update_load, field_path equivalent_static.wind.exposure. Existing equivalent_static.wind object required. before canonical {exposed_pipe_refs: existing-or-[], exposed_spans: existing-or-[]}. after same object requiring both arrays. Each span {pipe_ref,start_fraction:{value,unit},end_fraction:{value,unit}}. Fractions dimensionless; 0<=start<end<=1, no overlap or full/partial collision. Other wind fields preserved.

Exact rich support shapes and source validation references A2_RETURN.md; use canonical consumer labels for UI, preserve existing input aliases. Return warnings distinguish authoring from solve readiness. No provider/live runtime integration.
