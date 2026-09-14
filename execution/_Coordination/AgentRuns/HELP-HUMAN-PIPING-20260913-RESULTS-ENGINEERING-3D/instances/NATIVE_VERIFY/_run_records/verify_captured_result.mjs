// Lease-only check of captured native output against independently retained input.
// Does not import or invoke the product result adapter.
import fs from 'node:fs';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
const [capture,snapshot,engineDir]=process.argv.slice(2);
assert(capture&&snapshot&&engineDir);
const bytes=fs.readFileSync(capture),text=bytes.toString('utf8');
const actual=text.startsWith('data:')?decodeURIComponent(text.replace(/\r?\n$/,'').slice(text.indexOf(',')+1)):text;
const doc=JSON.parse(actual),saved=JSON.parse(fs.readFileSync(snapshot,'utf8'));
const model=JSON.parse(saved.model_json),source=JSON.parse(saved.mechanics_result_json);
const gluePath=path.join(engineDir,'open_pipe_stress_operation_applier.js');
const wasmPath=path.join(engineDir,'open_pipe_stress_operation_applier_bg.wasm');
const engine=await import(pathToFileURL(gluePath).href);
await engine.default({module_or_path:new Uint8Array(fs.readFileSync(wasmPath))});
const digest=v=>engine.canonical_sha256_hex(JSON.stringify(v));
const point=p=>p.slice(1).split('/').reduce((a,k)=>a[k],doc);
const e=doc.result_envelope;assert.equal(doc.schema_version,'0.2.0');assert.equal(e.schema_version,'0.2.0');assert.equal(doc.export_format_status.additional_formats,'TBD');
assert.equal(source.status.mechanics,'MECHANICS_SOLVED');assert.equal(e.model_ref.ref_id,model.project.id);assert.equal(source.model_ref,model.project.id);assert.equal(e.run_ref.ref_id,source.run_id);
assert.equal(e.reproducibility.model_hash.value,digest(model));
const origin=e.reproducibility.source_origin_bindings[0];assert.equal(origin.authentic_producer_available,false);assert.equal(origin.original_producer_checksum,null);assert.equal(origin.received_carrier_checksum.value,digest(source));
const absent=source.results.every(r=>!Object.hasOwn(r,'dimension'));assert(absent);assert.equal(origin.received_carrier_checksum.payload_scope,'received_current_dimension_absent_carrier');
assert.equal(e.row_accounting.length,source.results.length);assert.equal(e.source_annotations.length,source.results.length);
const targets=new Set();let numerical=0,disclosed=0;
for(const [i,r] of source.results.entries()){
 const a=e.row_accounting[i],ann=e.source_annotations[i];assert.equal(a.source_row_index,i);assert.equal(a.source_result_id,r.id);assert.equal(a.source_kind,r.kind);assert.equal(a.source_field_path,`/results/${i}`);assert.equal(a.received_carrier_row_checksum.value,digest(r));assert.equal(a.received_carrier_row_checksum.payload_scope,'received_current_dimension_absent_row');assert.equal(a.original_producer_row_checksum,null);assert(!targets.has(a.target_field_path));targets.add(a.target_field_path);
 assert.deepEqual(ann.metadata,r.metadata??null);assert.deepEqual(ann.basis_ref,r.basis_ref??null);assert.deepEqual(ann.source_result_refs,r.source_result_refs??[]);assert.deepEqual(ann.observed_carrier_dimension,{present:false,value:null});
 const t=point(a.target_field_path);if(a.disposition==='disclosed'){assert.equal(t.source_value,r.value);assert.equal(t.source_unit,r.unit);disclosed++;}else{assert.equal(t.magnitude,r.value);assert.equal(t.unit,r.unit);numerical++;}
 if(r.kind.startsWith('global_nodal_rotation_')){assert.equal(t.dimension,'angle');assert.equal(t.unit,'rad');assert.equal(t.family,'rotation');}
 if(r.kind==='linear_solver_mode_basis'){assert.equal(a.disposition,'disclosed');assert.equal(t.semantic_category,'solver_mode');assert.equal(t.reason_code,'discrete_evidence_not_ratio');}
}
assert.equal(e.unit_preservation_witnesses.length,numerical);
for(const w of e.unit_preservation_witnesses){const r=source.results[w.source_row_index],t=point(w.target_field_path);assert.deepEqual(w.source_quantity,{value:r.value,unit:r.unit,observed_dimension:{present:false,value:null}});assert.equal(w.target_quantity.value,r.value);assert.equal(w.target_quantity.unit,r.unit);assert.equal(w.original_producer_row_checksum,null);assert.equal(w.received_carrier_row_checksum.value,digest(r));assert.equal(w.target_row_checksum.value,digest(t));assert.equal(w.conversion_performed,false);}
const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;assert.equal(e.reproducibility.derivative_hash.value,digest(payload));assert.equal(e.reproducibility.derivative_hash.payload_scope,'derivative_document_excludes_own_hash');
console.log(JSON.stringify({status:'PASS_CAPTURED_NATIVE_SOURCE_BINDING_ONLY_SCHEMA_CHECK_SEPARATE',capture_sha256:crypto.createHash('sha256').update(bytes).digest('hex'),actual_decoded_utf8_sha256:crypto.createHash('sha256').update(actual).digest('hex'),native_received_rows:source.results.length,numerical,disclosed,original_producer_unavailable:true,engine_glue_sha256:crypto.createHash('sha256').update(fs.readFileSync(gluePath)).digest('hex'),engine_wasm_sha256:crypto.createHash('sha256').update(fs.readFileSync(wasmPath)).digest('hex')},null,2));
