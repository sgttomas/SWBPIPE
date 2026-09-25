// File-only independent JCS-style hash/reference checks; no product execution.
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
const root="/private/tmp/piping-engine-integration-20260925";
const here=path.dirname(new URL(import.meta.url).pathname);
const review=path.dirname(here), join=path.dirname(review);
const manifest=JSON.parse(fs.readFileSync(path.join(review,"MANIFEST.json"),"utf8"));
const sha=b=>createHash("sha256").update(b).digest("hex");
let assertions=0, numbers=0;
function check(x,label){assertions++;if(!x)throw new Error(label);}
function canonical(v){
 if(v===null||typeof v==="boolean"||typeof v==="string")return JSON.stringify(v);
 if(typeof v==="number"){numbers++;check(Number.isFinite(v),"finite JSON number");check(!Number.isInteger(v)||Math.abs(v)<=Number.MAX_SAFE_INTEGER,"IJSON safe integer");return JSON.stringify(v);}
 if(Array.isArray(v))return "["+v.map(canonical).join(",")+"]";
 return "{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canonical(v[k])).join(",")+"}";
}
function domain(d,p){return sha(canonical({domain:d,payload:p}));}
function inspect(v){if(typeof v==="number"){check(Number.isFinite(v),"finite fixture number");}else if(v&&typeof v==="object")for(const x of Object.values(v))inspect(x);}
const read=p=>JSON.parse(fs.readFileSync(path.join(root,p),"utf8"));
const files=new Map();
for(const item of manifest.paths.filter(p=>p.path.endsWith(".json"))){
 const b=fs.readFileSync(path.join(root,item.path));check(sha(b)===item.sha256,"frozen JSON bytes "+item.path);
 const value=JSON.parse(b);inspect(value);files.set(item.path,value);
}
const bindings=new Map();
for(const name of ["ACTUAL_CAPTURE_02.json","ACTUAL_UNIT_CAPTURE.json","ACTUAL_UNICODE_CAPTURE.json"]){
 const record=JSON.parse(fs.readFileSync(path.join(join,"COMPOSITE_RECEIPT",name),"utf8"));
 for(const run of record.runs){
  const input=run.input??run.command.at(-1).slice(root.length+1);
  check(sha(fs.readFileSync(path.join(root,input)))===(run.input_sha256??record.input_sha256),"actual capture input");
  check(sha(fs.readFileSync(path.join(root,run.output)))===run.output_sha256,"actual capture output");
  check(run.exit_code===0,"actual capture status");bindings.set(run.output,{input,record:name});
 }
}
const oldroot="projects/chirality-piping/fixtures/product_preview/source_blocks/";
const generation=files.get(oldroot+"generation.json");
for(const f of generation.files)check(sha(fs.readFileSync(path.join(root,oldroot,f.path)))===f.sha256,"original generation bytes");
const negroot=oldroot+"rejected_stress_range/";
const negative=files.get(negroot+"CAPTURE.json");
check(negative.classification.includes("negative reader fixtures")&&negative.classification.includes("never positive"),"negative truthfulness");
for(const f of negative.files)check(sha(fs.readFileSync(path.join(root,negroot,f.path)))===f.sha256,"negative captured bytes");
const rows=[];
const tables={};
for(const [key,file] of [["source","semantic_contract_v0_3_source_blocks_1.json"],["composite","semantic_contract_v0_3_physics_source_1.json"]]){
 tables[key]=read("projects/chirality-piping/fixtures/results/"+file).rows;
}
for(const [filename,raw] of files){
 if(!filename.endsWith(".raw.json"))continue;
 const composite=filename.includes("/physics_source/");
 const bad=filename.includes("/rejected_stress_range/");
 let input,mode;
 if(composite){check(bindings.has(filename),"genuine composite capture");input=bindings.get(filename).input;mode=filename.includes("-dense_scrutiny.")?"dense_scrutiny":"sparse_interactive";}
 else {input=filename.replace(".raw.json",".request.json");mode=filename.includes("dense_scrutiny.")?"dense_scrutiny":"sparse_interactive";}
 const request=files.get(input)??read(input), receipt=raw.source_block_recovery, body=receipt.body;
 const publication={...raw};delete publication.source_block_recovery;
 check(body.invocation.value===domain("source_blocks_invocation_v1",{request,solver_mode:mode}),"actual invocation hash "+filename);
 check(body.publication_sha256===domain("source_blocks_publication_v1",publication),"publication hash "+filename);
 check(receipt.receipt_sha256===domain("source_blocks_receipt_v1",body),"receipt hash "+filename);
 check(raw.model_ref===request.model.project.id,"model identity");
 check(raw.producer.semantic_contract_id==="openpipestress.result_semantics/0.3.0/"+(composite?"physics-source-1":"source-blocks-1"),"namespace");
 check(body.policy===(composite?"PHYSICS-SOURCE-1":"SOURCE-BLOCKS-1"),"policy");
 check(body.status==="qualified","retained recorded status"); // bad captures intentionally retain their old false claim.
 const actual=new Map(raw.results.map(r=>[r.id,r]));
 check(actual.size===raw.results.length,"unique rows");
 for(const r of raw.results)check(tables[composite?"composite":"source"].some(s=>s.kind===r.kind&&s.unit===r.unit&&(s.component??null)===(r.metadata?.component??null)&&(!("source_basis" in s)||s.source_basis===r.metadata?.basis)),"registered immutable row signature "+r.kind);
 const caseIds=request.model.load_cases.map(c=>c.id);
 check(JSON.stringify(body.cases.map(c=>c.basis_ref.ref_id))===JSON.stringify(caseIds),"ordered case coverage");
 check(JSON.stringify(raw.numerical_quality.cases.map(c=>c.basis_ref.ref_id))===JSON.stringify(caseIds),"ordinary case coverage");
 let accounted=new Set(body.envelope_observation_result_ids);
 let charged=body.invocation_work.publication_charged;
 for(const c of body.cases){
  check(c.requested_mode===mode,"actual mode");check(c.work.limit<=(composite?8000000:4000000),"method-specific cap");
  charged+=c.work.charged+c.work.reserved_unobserved_failure;
  for(const r of c.rows){check(actual.has(r.result_id),"row reference");check(!accounted.has(r.result_id),"single accounting");accounted.add(r.result_id);}
  for(const p of c.projections){
   const r=actual.get(p.result_id);check(r&&r.value===p.value&&r.unit===p.unit,"projection row bits/unit");
   check(p.interval[0]<=p.value&&p.value<=p.interval[1],"projection interval");
   check(p.relative_error_bound<=1e-9&&p.relative_limit===1e-9,"protected projection criterion");
  }
  if(composite){
   const exact=raw.contract_evidence.exact_cases.find(e=>e.load_case_id===c.basis_ref.ref_id);
   const pressure=raw.contract_evidence.pressure.filter(e=>e.load_case_id===c.basis_ref.ref_id);
   check(exact.recovery_method===c.selected_method,"physical selected method");
   check(c.physical_evidence_sha256===domain("physics_source_case_evidence_v1",{exact_case:exact,pressure}),"physical/source case hash");
   for(const m of exact.pipe_stress_extrema)check(actual.has(m.result_id)&&actual.get(m.result_id).entity_ref===m.pipe_id,"physical maximum binding");
  }
 }
 check(accounted.size===actual.size&&[...actual.keys()].every(k=>accounted.has(k)),"whole row coverage");
 check(charged===body.invocation_work.charged&&charged<=body.invocation_work.limit&&body.invocation_work.limit===64000000,"invocation debit sum");
 for(const h of [raw.summary.max_displacement,raw.summary.max_open_formula_stress])if(h)check(actual.get(h.result_ref)?.value===h.value&&actual.get(h.result_ref)?.entity_ref===h.location_ref,"headline source reference");
 if(bad){
  check(raw.results.some(r=>r.unit==="MPa"&&r.value!==0&&Math.abs(r.value)<2.2250738585072014e-308),"known-bad subnormal stress retained");
  check(!generation.files.some(f=>oldroot+f.path===filename),"negative excluded from positive generation inventory");
 }
 rows.push({path:filename,input,classification:bad?"known-bad retained negative":composite?"genuine composite capture":"genuine inherited source capture",result_count:raw.results.length,case_count:body.cases.length,invocation_charged:charged});
}
const synthetic=files.get("projects/chirality-piping/core/reporting/result_export/tests/fixtures/source_blocks_statement_control.json");
check(synthetic.description.includes("Synthetic")&&synthetic.description.includes("never used as physical/runtime"),"synthetic explicit scope");
check(synthetic.source.run_id==="synthetic-run","synthetic distinct identity");
for(const mode of ["dense_scrutiny","sparse_interactive"]){
 for(const base of ["n05","mixed"]){
  const a=read("projects/chirality-piping/fixtures/product_preview/physics_source/"+base+"-"+mode+".raw.json");
  const b=read("projects/chirality-piping/fixtures/product_preview/physics_source/"+base+"_units-"+mode+".raw.json");
  const values=raw=>raw.results.filter(r=>r.kind!=="load_case_modulus_basis").map(r=>[r.id,r.kind,r.value,r.unit]);
  check(JSON.stringify(values(a))===JSON.stringify(values(b)),"authored equivalent units preserve emitted values");
  for(let i=0;i<a.contract_evidence.exact_cases.length;i++)check(JSON.stringify(a.contract_evidence.exact_cases[i].pipe_materials)===JSON.stringify(b.contract_evidence.exact_cases[i].pipe_materials),"actual normalized E/nu/G equivalence");
 }
 const a=read("projects/chirality-piping/fixtures/product_preview/physics_source/n05-"+mode+".raw.json");
 const b=read("projects/chirality-piping/fixtures/product_preview/physics_source/n05_unicode-"+mode+".raw.json");
 check(JSON.stringify(a.results.map(r=>[r.kind,r.value,r.unit,r.metadata?.component]))===JSON.stringify(b.results.map(r=>[r.kind,r.value,r.unit,r.metadata?.component])),"Unicode identity edits preserve physical fields");
 check(a.source_block_recovery.body.invocation.value!==b.source_block_recovery.body.invocation.value,"Unicode actual invocation differs");
}
fs.writeFileSync(path.join(here,"CAPTURE_VERIFICATION.json"),JSON.stringify({runtime:process.version,method:"File-only canonical hashes and structural/reference inventory; no solver or production reader execution",assertions,number_visits:numbers,json_files:files.size,raw_packets:rows.length,records:rows},null,2)+"\n");
process.stdout.write(JSON.stringify({assertions,json_files:files.size,raw_packets:rows.length,number_visits:numbers})+"\n");
