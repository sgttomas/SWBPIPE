import fs from "node:fs";
import crypto from "node:crypto";
import { pathToFileURL } from "node:url";
const [project,rowPath,outputPath]=process.argv.slice(2);
const gluePath=project+"/apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier.js";
const wasmPath=project+"/apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier_bg.wasm";
const glue=await import(pathToFileURL(gluePath).href);
await glue.default({module_or_path:new Uint8Array(fs.readFileSync(wasmPath))});
const row=JSON.parse(fs.readFileSync(rowPath,"utf8"));
const model=JSON.parse(row.model_json);
const mechanics=JSON.parse(row.mechanics_result_json);
const analysis=JSON.parse(row.analysis_run_json);
const modelHash=JSON.parse(row.model_hash_json);
const envelopeHash=JSON.parse(row.project_envelope_hash_json);
const payload={
  model,
  editor_intents:JSON.parse(row.editor_intents_json),
  proposal:JSON.parse(row.proposal_json),
  selected_review_target:JSON.parse(row.selected_review_target_json),
  mechanics_result:mechanics,
  analysis_run:analysis,
  model_hash:modelHash,
};
const modelDigest=glue.canonical_sha256_hex(JSON.stringify(model));
const envelopeDigest=glue.canonical_sha256_hex(JSON.stringify(payload));
const result={
 status:(modelHash.value===`sha256:${modelDigest}`&&envelopeHash.value===`sha256:${envelopeDigest}`)?"PASS":"MISMATCH",
 authority:{
  profile:"rfc8785_jcs",
  implementation:"built exact-candidate open_pipe_stress_operation_applier WASM canonical_sha256_hex export used by apps/desktop/src/services/hashService.ts",
  glue_path:gluePath,
  glue_sha256:crypto.createHash("sha256").update(fs.readFileSync(gluePath)).digest("hex"),
  wasm_path:wasmPath,
  wasm_sha256:crypto.createHash("sha256").update(fs.readFileSync(wasmPath)).digest("hex"),
 },
 model_hash:{
  owning_scope:"model_payload",
  projection:"exact parsed stored model_json",
  stored_claim:modelHash,
  recomputed_value:`sha256:${modelDigest}`,
  match:modelHash.value===`sha256:${modelDigest}`&&modelHash.payload_ref===model.project.id,
  finding_if_mismatch:"HISTORICAL_MODEL_HASH_MISMATCH",
 },
 project_envelope_hash:{
  owning_scope:"project_envelope_payload",
  projection_keys:Object.keys(payload),
  excluded:"storage_summary_and_envelope_hash_carrier_fields",
  stored_claim:envelopeHash,
  recomputed_value:`sha256:${envelopeDigest}`,
  match:envelopeHash.value===`sha256:${envelopeDigest}`&&envelopeHash.payload_ref===model.project.id,
  finding_if_mismatch:"HISTORICAL_ENVELOPE_HASH_MISMATCH",
 },
 note:"These two persisted claims use the existing rfc8785_jcs WASM authority, not the checked JSON profile used by analysis 0.2."
};
fs.writeFileSync(outputPath,JSON.stringify(result,null,2)+"\n");
console.log(JSON.stringify({status:result.status,model_match:result.model_hash.match,envelope_match:result.project_envelope_hash.match,model_recomputed:result.model_hash.recomputed_value,envelope_recomputed:result.project_envelope_hash.recomputed_value}));

