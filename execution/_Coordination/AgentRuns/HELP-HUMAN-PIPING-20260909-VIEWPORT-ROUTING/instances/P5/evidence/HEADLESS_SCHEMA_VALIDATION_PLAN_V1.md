# P5 complete straight/curved headless schema validation plan

Status: SUPERSEDED BY SEALED `FULL_ENVELOPE_SCHEMA_WITNESS_PLAN_V1.md`

Root subsequently sealed and released `FULL_ENVELOPE_SCHEMA_WITNESS_PLAN_V1.md`, SHA-256 `2ffbcf56f9a6dad5ca34b6c20a362a5d1aadacafada9ef692b83e94d0cd95dc9`. That plan expanded this draft to three full cases and is the executed authority. The resulting full-schema witness failed on pre-existing broader headless producer/schema mismatches; see `../schema_witness/VALIDATION.json`. A manager-side unreserved `cargo test ... -- --list` invocation remains an external tooling observation only.

The existing headless witness uses the committed straight invented preview model. It cannot establish the realized-curved-bend condition. The complete route therefore uses an evidence-only scratch Cargo harness that depends on the public `open_pipe_stress_headless_runner` and `open_pipe_stress_product_physics` crates, calls the real public `run_preview_in_memory` path for both inputs, independently rebuilds each document with `result_envelope_binding::build_result_export_document`, emits each complete wrapper document, and then uses the repository's qualified Python JSON-Schema validator against the exact `schemas/results.schema.yaml`.

Run from `{PROJECT_ROOT}` only after root grants the heavy-build slot. The reviewer should bind the run to source SHA `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58` and fixture SHA `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`.

## Scratch harness contract

Materialize an isolated crate at `${TMPDIR:-/tmp}/p5_endpoint_headless_schema`. Its `Cargo.toml` must contain:

```toml
[package]
name = "p5_endpoint_headless_schema"
version = "0.0.0"
edition = "2021"
publish = false

[workspace]

[dependencies]
open_pipe_stress_headless_runner = { path = "{PROJECT_ROOT}/core/runner/headless" }
open_pipe_stress_product_physics = { path = "{PROJECT_ROOT}/core/product_physics" }
serde_json = "1"
```

The harness `src/main.rs` must:

1. Load `fixtures/product_preview/invented_preview_model.json` as the straight input.
2. Derive a curved input in memory by retaining nodes N-100/N-110, setting N-110 to `[2,0,0]`, retaining pipe P-100 with `y_reference=[0,1,0]`, retaining only the six-DOF N-100 anchor, and retaining component C-110 with:
   - `geometry.bend_pipe_ref="pipe:P-100"`;
   - radius `sqrt(2) m`, angle `pi/2 rad`;
   - user SIF `1.15`, flexibility factor `2.0`;
   - `mechanics_interface.solver_consumption="mechanics_curved_bend_macro_element"`.
3. Retain load case L-100 and set its primitive loads to the exact P5 curved oracle combination: node N-110 global-y `1000 N`, pipe P-100 global-z `-190 N/m`, and genuine pipe P-100 pressure `2_000_000 Pa`; clear combinations.
4. For both straight and curved `LinearStaticPreviewRequest { model, materials: vec![] }`, construct a `RunnerRequest` with `RunnerOperation::Solve`, `PrivacyContext::local_first_public_metadata()`, `ProfessionalBoundary::project_default()`, `TbdDecisions::d33_local_cli_policy()`, complete invented references/provenance, and requested outputs `result_envelope`, `audit_manifest`, and `diagnostics`.
5. Call `run_preview_in_memory(request.clone(), preview)`. Require `MECHANICS_SOLVED`, no blocking runner diagnostic, and an attached `result_envelope_document`.
6. Call `result_envelope_binding::build_result_export_document(&request, &output.runner_result, output.mechanics_envelope.as_ref().unwrap())` and require byte-equivalent `serde_json::Value` to the attached document. This directly exercises the real public headless export binding rather than a locally copied transform.
7. For the curved mechanics envelope, require every mechanical stress row for pipe P-100 at `end_i`, `end_j`, `quarter_1`, `midspan`, and `quarter_3` to carry `coordinate_system=element_local`, `basis=recovered_from_local_element_stiffness`, and the exact frozen curved convention. Require the corresponding endpoint/station IDs to appear in the complete exported document.
8. Write complete wrapper documents to the two paths passed after the model argument: `straight.result-export.json` and `curved.result-export.json`.

## Exact commands after slot release

```sh
test "$(sha256sum core/product_physics/src/lib.rs | awk '{print $1}')" = dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58
test "$(sha256sum fixtures/product_preview/invented_mechanics_result.json | awk '{print $1}')" = fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="${TMPDIR:-/tmp}/p5-endpoint-headless-target" cargo run --offline --quiet --manifest-path "${TMPDIR:-/tmp}/p5_endpoint_headless_schema/Cargo.toml" -- fixtures/product_preview/invented_preview_model.json execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/evidence/headless-schema
python3 - execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/evidence/headless-schema/straight.result-export.json execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/evidence/headless-schema/curved.result-export.json <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, str(Path('tests').resolve()))
from schema_validation import load_schema, validate_instance, validate_schema_document
schema_path = Path('schemas/results.schema.yaml')
schema = load_schema(schema_path)
assert validate_schema_document(schema, schema_label=str(schema_path))
for name in sys.argv[1:]:
    import json
    document = json.loads(Path(name).read_text(encoding='utf-8'))
    assert validate_instance(schema, document, schema_label=str(schema_path), instance_label=name)
print('PASS: complete straight and curved headless result-export documents validate')
PY
python3 -m pytest -q tests/test_results_schema.py tests/test_analysis_run_records.py
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="${TMPDIR:-/tmp}/p5-endpoint-headless-target" cargo test --offline --manifest-path core/runner/headless/Cargo.toml
```

No subset result may be reported as the complete straight/curved envelope pass. If either complete document fails schema validation, return the actual failure without widening P5's source/schema scope.
