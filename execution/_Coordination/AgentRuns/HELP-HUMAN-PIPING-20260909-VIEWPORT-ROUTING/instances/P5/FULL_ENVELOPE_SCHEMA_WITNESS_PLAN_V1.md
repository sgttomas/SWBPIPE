# P5 full straight and curved envelope schema witness plan V1

Status: `PLANNED — execution held for root Cargo slot`

## Evidence-only harness

Create the temporary evidence harness only under:

- `instances/P5/schema_witness/Cargo.toml`
- `instances/P5/schema_witness/src/main.rs`
- `instances/P5/schema_witness/outputs/straight_full_document.json`
- `instances/P5/schema_witness/outputs/curved_tip_weight_full_document.json`
- `instances/P5/schema_witness/outputs/curved_pressure_full_document.json`
- `instances/P5/schema_witness/VALIDATION.json`

The harness has path dependencies on `core/runner/headless` and `core/product_physics`, plus `serde_json`. It must not modify product source, the published fixture, schemas, or other evidence.

## Public production route

For each invented case, construct a public `open_pipe_stress_product_physics::LinearStaticPreviewRequest` from the checked-in invented preview model and call:

1. `open_pipe_stress_headless_runner::run_preview_in_memory(runner_request.clone(), preview_request)`;
2. require `mechanics_envelope.status.mechanics == "MECHANICS_SOLVED"`;
3. call `open_pipe_stress_headless_runner::result_envelope_binding::build_result_export_document(&runner_request, &output.runner_result, mechanics_envelope)`;
4. require that document to equal the `output.result_envelope_document` attached by the public runner;
5. serialize that complete wrapper document to the declared evidence output.

The `RunnerRequest` uses the same public invented request identity/provenance values as the established headless binding test (`request_id`, solve operation, project/model/unit/load/manifest references, requested result/audit/diagnostic outputs, local-first public metadata, invented provenance, project-default professional boundary, and the D33 local CLI decision set).

## Covered invented cases

- `P5-straight-full`: the checked-in `fixtures/product_preview/invented_preview_model.json` with nonlinear-bearing supports removed, matching the established real straight headless path. It exercises straight endpoints, interior stations, pressure components, primitive load cases, and combinations.
- `P5-curved-tip-weight-full`: the same invented model narrowed exactly as `curved_bend_span_request` to nodes N-100/N-110, pipe P-100, anchored N-100, a quarter-circle radius `sqrt(2) m` over the `2 m` chord, `pi/2` bend angle, y-reference `(0,1,0)`, user flexibility `2`, user SIF `1.15`, and `curved_bend_macro_element` solver consumption. Apply the existing invented `+1000 N global_y` tip force plus `-190 N/m global_z` uniform element load. This exercises both endpoints and all three curved stations with nontrivial six-component resultants in the actual arc frame.
- `P5-curved-pressure-full`: the same realized curved model with the existing invented `2 MPa` element pressure load. This exercises pressure thrust plus endpoint and station axial stress under the true arc tangent/radial frames.

All model construction copies checked-in invented test-fixture values; it introduces no engineering acceptance parameter.

## Exact schema validation

After the harness emits the three complete documents, use the repository-qualified Python with `jsonschema>=4,<5` to load the strict-JSON `schemas/results.schema.yaml`, run `Draft202012Validator.check_schema(schema)`, and call `Draft202012Validator(schema).iter_errors(document)` for every full wrapper document. Write all error paths/messages and per-case counts to `VALIDATION.json`.

Acceptance requires zero schema errors for all three documents, canonical endpoint and curved-station `coordinate_system=element_local` and `basis=recovered_from_local_element_stiffness`, frame/equilibrium detail retained in `sign_convention`, and no fabricated subset pass. Any actual failure is reported without editing source or schema.

## Execution sequence

1. Root assigns a Cargo slot after F4 reaches a stable cut.
2. Create the evidence-only harness and run it with `cargo run --manifest-path <P5 schema_witness Cargo.toml>`.
3. Run the exact Python Draft 2020-12 validation over all three emitted documents.
4. Hash the harness, emitted documents, schema input, source/fixture freeze, and validation result.
5. Treat the earlier unreserved `cargo test ... headless ... --list` as a tooling observation only; it is not validation and is not part of the acceptance evidence.
