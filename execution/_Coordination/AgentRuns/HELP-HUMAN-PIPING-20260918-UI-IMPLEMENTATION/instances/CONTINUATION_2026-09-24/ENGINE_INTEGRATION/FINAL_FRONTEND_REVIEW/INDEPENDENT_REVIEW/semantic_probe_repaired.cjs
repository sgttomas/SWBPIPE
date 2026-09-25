const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root='/private/tmp/piping-engine-integration-20260925/projects/chirality-piping';
const ts=require(root+'/node_modules/typescript/lib/typescript.js');
const sourcePath=root+'/apps/desktop/src/services/analysisRunCompatibility.ts';
const matcherPath=root+'/apps/desktop/src/features/results/resultSemantics.ts';
const text=fs.readFileSync(sourcePath,'utf8'),matching=fs.readFileSync(matcherPath,'utf8');
const original=fs.readFileSync(path.join(__dirname,'ANALYSIS_ORIGINAL_FROZEN.ts'),'utf8');
const fn=s=>s.slice(s.indexOf('export function analysisRowSemantics('),s.indexOf('const RULE_STATUSES'));
const predicate=matching.slice(matching.indexOf('export function semanticSourceBasisMatches('),matching.indexOf('export function semanticContractForSource('));
const digest=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const tables={}; for(const [key,file] of [['precision-1','precision'],['physics-1','physics'],['source-blocks-1','source_blocks'],['physics-source-1','physics_source']])tables[key]=JSON.parse(fs.readFileSync(root+'/fixtures/results/semantic_contract_v0_3_'+file+'_1.json'));
const tableFor=s=>tables[s.producer.semantic_contract_id.split('/').at(-1)];
function compile(code){const exports={};new Function('semanticContractForSource','exports',ts.transpileModule(code,{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022}}).outputText)(tableFor,exports);return exports.analysisRowSemantics;}
const current=compile(predicate+'\n'+fn(text)),prior=compile(fn(original));const records=[];
for(const name of ['n05','n06','mixed','fields','n05_units','mixed_units','n05_unicode'])for(const mode of ['sparse_interactive','dense_scrutiny']){
 const file=root+'/fixtures/product_preview/physics_source/'+name+'-'+mode+'.raw.json',s=JSON.parse(fs.readFileSync(file));const checks=[];
 for(const row of s.results.filter(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2')){
  const expected=row.metadata.basis==='retained_source_endpoint_normal_max_v1'?'physics-source-endpoint-normal-maximum':'supported-source-068';const got=current(row,s).semantic.signature_id;if(got!==expected)throw Error('signature');
  for(const bad of [undefined,'unrecognized_basis']){const r={...row,metadata:{...row.metadata,basis:bad}};let ok=false;try{current(r,s)}catch(e){ok=e.message.startsWith('SOURCE_BASIS_CONTRADICTION')};if(!ok)throw Error('basis not refused');}
  checks.push({id:row.id,expected,actual:got,missing_and_unknown_refused:true});
 }
 records.push({pair:name+'-'+mode,raw_sha256:digest(file),maxima:checks});
}
const compatibility=[];
for(const file of ['fixtures/results/precision_connected_ui_mechanics_sparse.json','fixtures/results/physics_connected_ui_mechanics_sparse.json','fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json']){
 const full=root+'/'+file,s=JSON.parse(fs.readFileSync(full));for(const row of s.results){const a=current(row,s),b=prior(row,s);if(JSON.stringify(a)!==JSON.stringify(b))throw Error('old method changed: '+row.id);}
 compatibility.push({path:file,sha256:digest(full),rows_unchanged:s.results.length});
}
const result={status:'PASS bounded actual-function repair backcheck',scope:'In-memory TypeScript transpilation of current function and shared predicate; actual pinned table and genuine captured rows. No full validator/native invocation/new solve.',node:process.version,typescript:ts.version,sources:[sourcePath,matcherPath].map(path=>({path,sha256:digest(path)})),pairs:records,old_method_compatibility:compatibility};
fs.writeFileSync(path.join(__dirname,'SEMANTIC_PROBE_REPAIRED.json'),JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify({status:result.status,pairs:records.length,old_methods:compatibility},null,2));
