# P5 Python metadata oracle amendment V1

Status: SEALED FOR TEST-ONLY IMPLEMENTATION

## Trigger and diagnosis

DEC-025 G1 at candidate `ab1016e653afd7db6e0d8243b83492c4a54ede31` passed 1009 Python tests and failed three assertions in two files. Two assertions expected an endpoint torsional stress row to retain the old open-mechanics basis. The third required mechanical section-equilibrium language on a pressure-hoop station row.

The frozen producer is correct. Endpoint mechanical stresses now derive from the repaired section cut and use `recovered_from_local_element_stiffness` with j-side, element-local, section-equilibrium wording. Straight interior mechanical stresses retain the open-mechanics stress-component basis while recording the section-cut correction. Pressure membrane rows are a distinct `pipe_section` category with the open-mechanics stress-component basis and an exact explicit-pressure sign convention; they do not claim section-action semantics.

## Authorized repair

Edit only the affected assertions in `tests/product_preview/test_product_preview_service.py` and `tests/test_results_schema.py`, whose sealed pre-edit SHA-256 values are `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` and `64854f2e43c74349b695f7f8a1d5fb6609d3d3dd7fa78ab9ca3abdfa70c4d553`.

- Assert the endpoint torsional row's local-stiffness basis and its section-cut sign semantics.
- Assert exact endpoint and station pressure-hoop basis, coordinate system, component, and explicit-pressure convention, including absence of section-action wording.
- In complete station coverage, require section-equilibrium and distributed-load correction wording only for mechanical rows; preserve all required-field, enum, full component/location coverage, and invalid-basis rejection checks.

Do not change production code, fixtures, schemas, result families, numeric values, tolerances, or the open 196-error full-envelope contract disposition.

Run the three focused failures, then both complete Python modules with the qualified environment. Freeze the two-file diff for the root-routed combined test-only review.
