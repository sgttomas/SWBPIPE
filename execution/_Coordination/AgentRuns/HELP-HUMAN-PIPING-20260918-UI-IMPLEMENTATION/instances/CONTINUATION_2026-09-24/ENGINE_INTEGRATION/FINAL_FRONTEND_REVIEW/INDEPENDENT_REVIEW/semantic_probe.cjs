const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root='/private/tmp/piping-engine-integration-20260925/projects/chirality-piping';
const ts=require(root+'/node_modules/typescript/lib/typescript.js');
const original=fs.readFileSync(path.join(__dirname,'ANALYSIS_ORIGINAL_FROZEN.ts'),'utf8');
const fn=original.slice(original.indexOf('export function analysisRowSemantics('),original.indexOf('const RULE_STATUSES'));
const js=ts.transpileModule(fn,{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022}}).outputText;
const tablePath=root+'/fixtures/results/semantic_contract_v0_3_physics_source_1.json';const table=JSON.parse(fs.readFileSync(tablePath));
const modules={};new Function('semanticContractForSource','exports',js)(source=>{if(source.producer.semantic_contract_id!==table.contract_id && source.producer.semantic_contract_id!=='openpipestress.result_semantics/0.3.0/physics-source-1')throw Error('unexpected fixture contract');return table;},modules);
const cases=[];const digest=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
for(const mode of ['sparse_interactive','dense_scrutiny']){
 const inputPath=root+'/fixtures/product_preview/physics_source/n05-'+mode+'.raw.json';const source=JSON.parse(fs.readFileSync(inputPath));
 const row=source.results.find(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2');
 const actual=modules.analysisRowSemantics(row,source).semantic;
 const expected=table.rows.find(s=>s.kind===row.kind&&s.unit===row.unit&&s.component===row.metadata.component&&s.source_basis===row.metadata.basis);
 if(actual.signature_id===expected.signature_id)throw Error('original failure not reproduced');
 const altered={...row,metadata:{...row.metadata,basis:'unrecognized-basis'}};let refused=false;try{modules.analysisRowSemantics(altered,source)}catch{refused=true}
 cases.push({mode,raw_sha256:digest(inputPath),row_id:row.id,row_basis:row.metadata.basis,expected_signature:expected.signature_id,actual_signature:actual.signature_id,unknown_basis_refused:refused});
}
const record={scope:'Actual frozen analysisRowSemantics function transpiled in memory; semantic table dependency fixed to the genuine composite fixture contract. Not complete native/validator execution or a new solve.',source_sha256:digest(path.join(__dirname,'ANALYSIS_ORIGINAL_FROZEN.ts')),table_sha256:digest(tablePath),typescript_version:ts.version,node:process.version,observations:cases};
fs.writeFileSync(path.join(__dirname,'SEMANTIC_PROBE_ORIGINAL.json'),JSON.stringify(record,null,2)+'\n');console.log(JSON.stringify(record,null,2));
