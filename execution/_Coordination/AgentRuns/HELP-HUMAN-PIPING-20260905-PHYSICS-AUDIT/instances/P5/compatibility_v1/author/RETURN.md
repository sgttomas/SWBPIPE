# P5 metadata compatibility bounded return

RUN_STATUS: SUCCESS — ready for fresh review; whole-package closure remains parent/root-owned.
ControlSurface: MERGED; executed AUTHOR_BRIEF V1, continuing portability successor AUTHOR_BRIEF_V2 SHA256 1a7454a04b93f99de6b9fa73aa81851e35d5e5d70bebdb0f63cb4d926b8f9ba3 acknowledged. RuntimeOverrides.INSTRUCTION_ROOT resolves REPO_ROOT. Role TASK Agent2, software-bounded-implementation v1, no children; delegated-harness-native non-delegation instruction+config asserted, model unknown when unexposed.

## Frozen source and containment

Product before93d182ee3504db7114058ff7de0aaa6f096728398869ca72a39d6ad78b18b2cc; aftere757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903.
Headless binding before0ab28ce33829e198ac4f91e2136a1dca122f35a8cb024af276683c200e3697cd; after2ed810b577d5db34824e65a36e9d696d3758dd14d8c4bdd0941c1cb797cf9c85.
SOURCE_BINDINGS.json records paths through WORKING_ROOT and verifies headless production prefix is byte-identical. Only added headless test module code; no adapter production change. Product changes only metadata categories/descriptions and inline assertions. No numerical expression, result ID, public key, schema, fixture or unrelated family changes. Prior implementation/evaluation packets immutable. Source frozen and compile slot released before packaging.

## Behavior

- Straight station forces/moments use existing recovered_from_local_element_stiffness category. The existing sign_convention now explicitly names j-side section action, element-local frame, section equilibrium from stiffness-recovered end actions and consistent distributed-load fixed-end correction.
- Straight station stresses use existing recovered_from_open_mechanics_stress_components category. Freeform sign text describes section-equilibrium resultants rather than interpolation and carries the same detailed cut/recovery context. Pressure membrane text remains alongside the detailed context.
- Curved station categories/conventions remain untouched. Arithmetic combination basis/sign semantics retain existing explicit_user_linear_combination and source-reference meaning. No canonical enum extension or silent adapter omission was introduced.

## Checks and actual canonical evidence

Product full offline lib check:137PASS, sole known FAIL generated_result_surface_matches_fallback_fixture_force_metadata. Fixture remains unchanged, assertion remains enabled and expects valid categories. F1 owns regeneration. Numerical product regressions continue passing; parent independently compares all numerical results before/after.
Headless final full offline lib check:39PASS,0FAIL. New straight_station_library_document_metadata_uses_canonical_schema_categories runs linear, nonlinear and zero-pressure scenarios. It consumes actual library result_envelope_document, not CLI serialization (that field is serde-skipped), and checks schema-derived required properties, string values, closed enums, no additional fields, truthful recovery text and primitive/combination categories.
Actual document evidence: linear_canonical_document.json, nonlinear_canonical_document.json, zero_pressure_canonical_document.json. These are produced directly by the library in the test, not reconstructed from preview output. They are not claimed wholly schema-valid: unrelated pre-existing invalid exported metadata remains parent-diagnosed and out of scope.

Reproduce bounded test evidence with CARGO_TARGET_DIR={ScopePath}/target and PIPING_P5_SCHEMA_WITNESS_DIR={ScopePath}, then cargo test --manifest-path {WORKING_ROOT}/core/runner/headless/Cargo.toml --offline --lib straight_station_library_document_metadata_uses_canonical_schema_categories. Normal tests without the environment variable write no documents. Root explicitly authorized isolated product/headless tests; all were serialized. Heavy/global/native checks remain root-owned.

## Existing export boundary found and preserved

Zero-pressure scenario exercises station pressure_longitudinal_stress. Its preview kind pipe_section_pressure_longitudinal_stress is already absent from the export mapping table. The first expanded regression correctly failed when it demanded this row in the canonical document. Final regression requires the existing explicit vocabulary-boundary disclosure naming the row, validates its changed preview metadata against canonical ResultMetadata, and requires all mapped changed families to reach the real canonical document. This is a distinct limitation, not a newly introduced omission. No unsupported family was coerced, added or dropped to obtain a test pass. All six force/moment and six stress component categories are exercised, including the separately disclosed longitudinal-pressure family.

## Durable evidence and next gates

Predecessor/final source copies, base64 patches with decoded SHA256, exact encoded red/green logs, selected-check proposal, scope validation and manifest supplied. No target/cache/binary included. Shell/editor/formatter writes stayed within two named source paths and own ScopePath. Headless formatter was applied only to the new test function. Method qualification: check selector ran after targeted implementation reads/edits; no additional permission or scope was inferred.
Fresh complete-diff review, parent numerical identity/schema diagnosis, F1 fixture/Python updates, and root final checks remain required. This derivative package consumes accepted source/briefs; it is not authoritative decomposition truth, engineering acceptance or lifecycle issuance. D01–D06 and broader canonical-export repairs remain held.
