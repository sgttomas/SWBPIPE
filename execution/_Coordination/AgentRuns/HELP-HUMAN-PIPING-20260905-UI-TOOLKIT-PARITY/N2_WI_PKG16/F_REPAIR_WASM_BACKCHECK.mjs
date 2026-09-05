import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import path from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const working = "projects/chirality-piping";
const evidence = `${working}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY`;
const instance = `${evidence}/N2_WI_PKG16`;
const reproductionPath = `${evidence}/instances/N7_FINAL_REVIEW/F1_REPRODUCTION.json`;
const reproduction = JSON.parse(readFileSync(reproductionPath, "utf8"));
const engineBase = `${working}/apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier`;
const engine = await import(pathToFileURL(path.resolve(root, `${engineBase}.js`)));
engine.initSync({ module: readFileSync(`${engineBase}_bg.wasm`) });
const artifacts = {};
for (const base of [engineBase, `${working}/apps/desktop/public/self-weight-engine/open_pipe_stress_self_weight_wasm`]) {
  for (const suffix of [".js", "_bg.wasm", ".d.ts", "_bg.wasm.d.ts"]) {
    artifacts[base + suffix] = createHash("sha256").update(readFileSync(base + suffix)).digest("hex");
  }
}
const originalModel = JSON.stringify(reproduction.model);
const outputs = [];
function invoke(label, batch, expectedApplied) {
  const output = JSON.parse(engine.apply_operation_batch_json(originalModel, JSON.stringify(batch), JSON.stringify(reproduction.claim)));
  assert.equal(output.application_route, "local_wasm_engine");
  assert.equal(output.input_model_unchanged, true);
  if (expectedApplied) {
    assert.equal(output.validation.application_status, "applied_to_session_model");
    assert.ok(output.applied_model);
    assert.ok(output.acceptance);
  } else {
    assert.equal(output.validation.application_status, "blocked");
    assert.equal(output.applied_model, null);
    assert.equal(output.acceptance, null);
    assert.equal(output.simulation_disposition, "rolled_back_no_model_published");
    assert.ok(output.diagnostics.some(item => item.severity === "blocking"));
  }
  outputs.push({ label, expected_applied: expectedApplied, output });
  return output;
}
invoke("N7_original_unequal_duplicate_stiffness", reproduction.batch, false);
for (const unit of ["lbf/in", "lbf/ft"]) {
  const batch = structuredClone(reproduction.batch);
  const support = JSON.parse(batch.operations[0].change.after);
  delete support.stiffness;
  support.hanger.stiffness.value = { value: 1, unit };
  batch.operations[0].change.after = JSON.stringify(support);
  let output = invoke(`single_US_${unit}`, batch, true);
  assert.deepEqual(output.applied_model.supports[0].hanger.stiffness.value, { value: 1, unit });
  const converted = JSON.parse(engine.convert_display_quantities_json(JSON.stringify({items:[{ id:"stiffness",value:1,from_unit:unit,to_unit:"N/m",dimension_id:"linear_stiffness" }]}))).items[0];
  assert.equal(converted.status, "converted");
  support.stiffness = { dof:"Uy", value:{value:converted.value,unit:"N/m"} };
  batch.operations[0].change.after = JSON.stringify(support);
  output = invoke(`equal_canonical_US_duplicate_${unit}`, batch, true);
  assert.deepEqual(output.applied_model.supports[0].stiffness, support.stiffness);
  assert.deepEqual(output.applied_model.supports[0].hanger.stiffness, support.hanger.stiffness);
}
assert.equal(JSON.stringify(reproduction.model), originalModel);
const result = { status:"PASS", runtime:"actual generated Wasm export under Node; not browser DOM", artifact_hashes:artifacts, original_reproduction_sha256:createHash("sha256").update(readFileSync(reproductionPath)).digest("hex"), cases:outputs, input_unchanged:true };
const resultPath = `${instance}/F_REPAIR_WASM_BACKCHECK_V1.json`;
writeFileSync(resultPath, JSON.stringify(result, null, 2) + "\n", {flag:"wx"});
console.log(JSON.stringify({status:result.status,cases:outputs.length,artifact_hashes:artifacts,result:resultPath}));
