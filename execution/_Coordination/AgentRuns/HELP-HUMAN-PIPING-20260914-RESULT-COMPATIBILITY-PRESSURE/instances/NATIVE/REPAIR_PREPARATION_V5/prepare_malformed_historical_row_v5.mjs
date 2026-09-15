#!/usr/bin/env node
// Preparation only: derive a self-consistent synthetic persisted row without erasing a malformed JSON carrier.
import fs from "node:fs";
import crypto from "node:crypto";
import { pathToFileURL } from "node:url";
const argv=process.argv.slice(2), arg=(name)=>{const i=argv.indexOf(name);if(i<0||!argv[i+1])throw new Error(`MISSING:${name}`);return argv[i+1];};
const baselinePath=arg("--baseline-row"),casesPath=arg("--cases"),caseId=arg("--case-id"),gluePath=arg("--wasm-glue"),wasmPath=arg("--wasm"),outputPath=arg("--output");
const glue=await import(pathToFileURL(gluePath).href);await glue.default({module_or_path:new Uint8Array(fs.readFileSync(wasmPath))});
const baseline=JSON.parse(fs.readFileSync(baselinePath,"utf8")), cases=JSON.parse(fs.readFileSync(casesPath,"utf8"));
const spec=cases.cases.find((item)=>item.case_id===caseId);if(!spec)throw new Error(`UNKNOWN_CASE:${caseId}`);
const row=structuredClone(baseline), carrierJson=JSON.stringify(spec.mechanics_result), analysisJson=JSON.stringify(cases.analysis_run);
row.mechanics_result_json=carrierJson;row.analysis_run_json=analysisJson;
const model=JSON.parse(row.model_json), modelHash=JSON.parse(row.model_hash_json);
const envelopePayload={model,editor_intents:JSON.parse(row.editor_intents_json),proposal:JSON.parse(row.proposal_json),selected_review_target:JSON.parse(row.selected_review_target_json),mechanics_result:JSON.parse(carrierJson),analysis_run:JSON.parse(analysisJson),model_hash:modelHash};
const digest=glue.canonical_sha256_hex(JSON.stringify(envelopePayload)), oldClaim=JSON.parse(row.project_envelope_hash_json);
row.project_envelope_hash_json=JSON.stringify({...oldClaim,payload_ref:model.project.id,value:`sha256:${digest}`});
const sha=(value)=>crypto.createHash("sha256").update(value).digest("hex");
const result={schema:"prepared-malformed-historical-row-v5",case_id:caseId,source_basis:spec.source_basis,row,expectations:{mechanics_result_json:carrierJson,mechanics_result_json_sha256:sha(carrierJson),analysis_run_json:analysisJson,project_envelope_hash:JSON.parse(row.project_envelope_hash_json),raw_carrier_must_survive_open_and_unchanged_save:true,current_must_remain_unset:true,malformed_finding_code:"PENDING_FINAL_WRITER_CODE_BINDING"},authority:{profile:"rfc8785_jcs",wasm_glue:gluePath,wasm:wasmPath,wasm_sha256:sha(fs.readFileSync(wasmPath))}};
fs.writeFileSync(outputPath,JSON.stringify(result,null,2)+"\n");
console.log(JSON.stringify({status:"PREPARED",case_id:caseId,mechanics_result_json:carrierJson,envelope_hash:result.expectations.project_envelope_hash.value}));
