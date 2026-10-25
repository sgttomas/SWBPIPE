import { refuseSurfaceOutput, refuseSurfaceRoute } from '../results/outputPolicy';
import { RETAINED_ABSOLUTE_VERIFIED, RETAINED_NOT_COVERED, retainedClassDisclosure, retainedRowClassesFromReader } from '../results/retainedPrecisionDisclosure';
import { readerRowSemantics, readerValidate, sourceContract, sourceSemanticBinding } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { isFreshSemanticResult } from "../results/knownSemanticLimitations";
import { RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, isRetainedRoute } from '../results/numericalResultQuality';
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from '../../types';
import { canonicalJsonString, canonicalSha256Hex, canonicalSha256HexCheckedV1, checkedJsonText } from '../../services/hashService';
import { verifyAnalysisRunRecord, validateAnalysisRunV03, modelLoadBasisRefs } from '../../services/analysisRunCompatibility';
import { hasNativeMechanicsInvocation } from '../../services/previewService';
import { verifyCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from '../../services/inputManifestService';
import { recordRulePackRefAnnotations, rulePackRefAnnotationsToAdd, type RulePackRefDiagnosticAnnotation } from '../../services/rulePackRefDiagnostics';
// JSON assembly is independent of saved analysis/persistence types. Qualification
// below owns Current authenticity; the pure projection never authenticates shape.
export type JsonObject = Record<string, any>;
const canonicalization = 'openpipestress_jcs_ijson_v1';
export const ref = (ref_type:string, ref_id:string) => ({ref_type,ref_id});
function validUnicode(s:string): boolean {
  for(let i=0;i<s.length;i++){const c=s.charCodeAt(i);if(c>=0xd800&&c<=0xdbff){const next=s.charCodeAt(++i);if(!(next>=0xdc00&&next<=0xdfff))return false;}else if(c>=0xdc00&&c<=0xdfff)return false;}return true;
}
export function guardResultJson(value:unknown,seen=new Set<object>()):void {
  if(value===null||typeof value==='boolean')return;
  if(typeof value==='string'){if(!validUnicode(value))throw new Error('MALFORMED_UNICODE');return;}
  if(typeof value==='number'){if(!Number.isFinite(value)||(Number.isInteger(value)&&!Number.isSafeInteger(value)))throw new Error('UNSAFE_JSON_NUMBER');return;}
  if(typeof value!=='object')throw new Error('UNSUPPORTED_JSON_VALUE');
  if(seen.has(value))throw new Error('CYCLIC_JSON');
  if(!Array.isArray(value)&&Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)throw new Error('NONPLAIN_JSON_OBJECT');
  if(Object.getOwnPropertySymbols(value).length)throw new Error('SYMBOL_JSON_KEY');
  seen.add(value);
  for(const [key,descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))){if(!validUnicode(key)||descriptor.get||descriptor.set)throw new Error('UNSUPPORTED_JSON_PROPERTY');if(Array.isArray(value)&&key==='length')continue;if(!descriptor.enumerable)throw new Error('NONENUMERABLE_JSON_PROPERTY');if(Array.isArray(value)&&(!/^(0|[1-9][0-9]*)$/.test(key)||Number(key)>=value.length))throw new Error('EXTRA_ARRAY_PROPERTY');guardResultJson(descriptor.value,seen);}
  if(Array.isArray(value))for(let i=0;i<value.length;i++)if(!Object.hasOwn(value,i))throw new Error('SPARSE_JSON_ARRAY');
  seen.delete(value);
}
export async function resultDigest(value:unknown):Promise<string>{guardResultJson(value);return canonicalSha256Hex(value);}
async function scopedChecksum(value:unknown,payload_scope:string,payload_ref:JsonObject){return {algorithm:'sha256',canonicalization,payload_scope,payload_ref,value:await resultDigest(value)};}
export const derivativeProvenance = {source_name:'local qualified result derivative',source_location:'apps/desktop/src/features/result-export/resultExportAdapter.ts',source_license:'project-local',contributor:'SWBPIPE',contributor_certification:'local desktop session; human review required',redistribution_status:'private_only',review_status:'pending'};
/** One source diagnostic in the document, mapped as Rust's derivative maps it
 * (`core/runner/headless/src/result_envelope_binding.rs`, `export_mechanics_diagnostic`):
 * the affected object is the diagnostic itself, by id, which traces to the bound raw
 * source and every affected ref it keeps, and the class follows Rust's rules (DEL-08-04:
 * diagnostics are not dropped by exporters or adapters; DEL-04-07 REQ-004). Provenance
 * stays the desktop derivative's own. */
function documentDiagnostic(x:MechanicsResult['diagnostics'][number]):JsonObject{
  if(typeof x.id!=='string'||!x.id)throw new Error(`RESULT_EXPORT_DIAGNOSTIC_ID_MISSING: ${x.code}`);
  const severity=x.severity==='info'||x.severity==='warning'?x.severity:'blocking';
  return {code:x.code,class:diagnosticClass(x.code,x.severity),severity,source:ref('solver_component',x.source??'core/product_physics'),
    affected_object:ref('preview_diagnostic',x.id),message:x.message,
    remediation:'Review the carried solve diagnostic before relying on this exported result envelope.',provenance:derivativeProvenance};
}
/** A run record's rule-pack reference diagnostic (a model reference the solve left out of the
 * input manifest's `active_rule_packs`: unresolved, or a checksum mismatch) in the document,
 * mapped as `documentDiagnostic` maps a source diagnostic: code, message and severity copied,
 * the class by Rust's rules (a `RULE` warning is an ASSUMPTION_WARNING). Its source is the
 * desktop's solve-time resolution, its affected object the excluded reference by identity, and
 * provenance the derivative's own. A record without exclusions adds nothing. */
function rulePackRefDocumentDiagnostic(x:RulePackRefDiagnosticAnnotation):JsonObject{
  return {code:x.code,class:diagnosticClass(x.code,x.severity),severity:x.severity,source:ref('desktop_component',x.source),
    affected_object:ref('rule_pack_ref',x.affected_refs[0]),message:x.message,
    remediation:'Review the excluded rule-pack reference before relying on this exported result envelope; it is not among this run\'s rule-pack references.',provenance:derivativeProvenance};
}
/** The document's diagnostics: each source diagnostic (`documentDiagnostic`), then each of the
 * run record's rule-pack reference diagnostics that none of them already carries. */
function documentDiagnostics(result:MechanicsResult,runDiagnostics:readonly unknown[]|undefined):JsonObject[]{
  const carried=result.diagnostics.map(documentDiagnostic);
  const added=rulePackRefAnnotationsToAdd(recordRulePackRefAnnotations(runDiagnostics,result.diagnostics.length),[...result.diagnostics,...carried]);
  return [...carried,...added.map(rulePackRefDocumentDiagnostic)];
}
/** Rust's `export_diagnostic_class`, rule for rule. */
function diagnosticClass(code:string,severity:string):string{
  if(code.includes('RULE'))return severity==='blocking'?'RULE_CHECK_BLOCKING':'ASSUMPTION_WARNING';
  if(code.includes('UNIT'))return 'UNIT_WARNING';
  if(code.includes('PROVENANCE'))return 'PROVENANCE_WARNING';
  if(severity!=='info'&&severity!=='warning')return 'SOLVE_BLOCKING';
  if(code.startsWith('NONLINEAR'))return 'NONLINEAR_WARNING';
  return 'ASSUMPTION_WARNING';
}
/** The one result-export version (b7-plan E2). */
export const RESULT_EXPORT_VERSION = '0.3.0';

/** Pure projection for already qualified evidence. It does not mint Current or
 * authentic producer proof and is not the product's canonical export entrypoint. */
export async function deriveResultDocument(base:JsonObject,model:PreviewModel,source:MechanicsResult,origin:JsonObject,request:unknown=null):Promise<JsonObject>{
  guardResultJson(base);guardResultJson(model);guardResultJson(source);guardResultJson(origin);
  if(['source_block_recovery','carrier_evidence'].some(key=>Object.hasOwn(base.result_envelope,key)))throw new Error('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
  if(source.status.mechanics!=='MECHANICS_SOLVED'||!source.results.length)throw new Error('SOURCE_NOT_SOLVED');
  if(model.project.id!==source.model_ref)throw new Error('SOURCE_MODEL_IDENTITY_MISMATCH');
  if(origin.received_carrier_checksum.value!==await resultDigest(source))throw new Error('SOURCE_CARRIER_HASH_MISMATCH');
  if(!origin.authentic_producer_available&&origin.original_producer_checksum!==null)throw new Error('UNAVAILABLE_PRODUCER_HASH');
  // b7-plan E2: a dimension-absent carrier declares no row dimension (no enriched carrier remains).
  if(origin.received_carrier_checksum.payload_scope==='received_current_dimension_absent_carrier'&&source.results.some(row=>Object.hasOwn(row,'dimension')))throw new Error('CARRIER_DIMENSION_PRESENT');
  // T6S-3/T6S-4: this surface's policy entry, route level (a pure projection reads no standing).
  const route=sourceContract(source);if(route==='unsupported')throw new Error('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');refuseSurfaceRoute(source,'result-export');const version=RESULT_EXPORT_VERSION;
  // T6S-4 (D-U6-2; Rust `retained_row_classes`): the successor's validated classes, from the
  // accepted reader run on these bytes without an invocation; null for every other identity.
  const classes=await retainedRowClassesFromReader(source);
  // Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.2): the exact routes read
  // the full Rust reader, as Rust's `derive_document` does: pressure-2 with the model,
  // pressure-source-1 through its raw reader without an invocation (never the shallow
  // source-recovery check). The retired v2 arms went with v2.
  if(route==='preview_physics')readerValidate('preview-physics',source,model);
  else if(route==='pressure')readerValidate('pressure',source,model);
  else if(route==='pressure_source')readerValidate('pressure-source',source);
  const doc=structuredClone(base);doc.schema_version=version;const e=doc.result_envelope;
  e.producer=structuredClone(source.producer);e.numerical_quality=structuredClone(source.numerical_quality);e.formulation_basis=structuredClone(source.formulation_basis);e.semantic_contract_ref=ref('semantic_contract',sourceSemanticBinding(source).id);
  if(Object.hasOwn(e,'source_block_recovery'))throw new Error('SOURCE_RECOVERY_METADATA_CONTRADICTION');
  // b7-plan MJ8: pressure-source-1's received receipt travels whole, as in Rust's derivative.
  if(route==='pressure_source')e.source_block_recovery=structuredClone(source.source_block_recovery);
  if(route==='preview_physics'||route==='pressure'||route==='pressure_source'||isRetainedRoute(route))e.contract_evidence=structuredClone(source.contract_evidence);
  else if(Object.hasOwn(e,'contract_evidence'))throw new Error('SOURCE_PHYSICAL_METADATA_MISMATCH');
  // D2 4.9.7: the successor's receipt travels with the document, whole. A receipt on any
  // other identity's document is refused by the validation that ends this function.
  if(isRetainedRoute(route))e.retained_precision=structuredClone(source.retained_precision);
  e.result_sets=[e.result_sets[0]];
  e.schema_version=version;e.model_ref=ref('model_payload',source.model_ref);
  const source_origin_ref=ref('source_origin_binding',origin.origin_id);
  const values:JsonObject[]=[],reviews:JsonObject[]=[],disclosures:JsonObject[]=[],annotations:JsonObject[]=[],accounts:JsonObject[]=[],witnesses:JsonObject[]=[];const ids=new Set<string>();
  const rowSemantics=readerRowSemantics(source);
  for(const [index,row] of source.results.entries()){
    if(!row.id||ids.has(row.id))throw new Error('DUPLICATE_SOURCE_ID');ids.add(row.id);
    if(!Number.isFinite(row.value)||!row.unit||!row.entity_ref)throw new Error('SOURCE_ROW_INVALID');
    const r=rowSemantics(row),s=r.signature,category=s?.category??'unknown',dimension=s?.derivative_target_dimension??null,math=s?.source_physical_semantic_dimension??null;
    const md=r.canonical_metadata,mandatory=['force','moment','section_property'].includes(s?.family??'');
    const reviewMissing=s?.canonical_disposition==='exported_review'&&!r.complete_metadata,physicalMissing=s?.canonical_disposition==='exported_quantity'&&mandatory&&!md;
    // D-U6-2 (Rust `class_disclosure`): an absolute_verified or not_covered row is disclosed, not valued.
    const classDisclosure=retainedClassDisclosure(row.kind,row.unit,classes?.get(row.id));
    const disposition=reviewMissing||physicalMissing||classDisclosure?'disclosed':s?.canonical_disposition??'disclosed';
    const observed={present:Object.hasOwn(row,'dimension'),value:row.dimension??null},sourcePath=`/results/${index}`;
    const rowScope=origin.received_carrier_checksum.payload_scope==='received_current_dimension_absent_carrier'?'received_current_dimension_absent_row':'raw_source_row';
    const rowHash=await scopedChecksum(row,rowScope,origin.received_carrier_checksum.payload_ref),original=origin.authentic_producer_available?await scopedChecksum(row,'raw_source_row',origin.original_producer_checksum.payload_ref):null;
    const annotation_ref=ref('source_annotation',`source-annotation:${index}`),object_ref=ref('preview_entity',row.entity_ref);
    const basis_ref=row.basis_ref??(disposition==='exported_review'?ref('source_basis',row.metadata!.basis):e.run_ref);
    // W-15 slice d (DEL-03-05 Design §5): a component body's record travels verbatim.
    const annotation:JsonObject={annotation_id:annotation_ref.ref_id,source_row_index:index,source_result_id:row.id,metadata:row.metadata??null,basis_ref:row.basis_ref??null,source_result_refs:row.source_result_refs??[],observed_carrier_dimension:observed,source_physical_semantic_dimension:math,derivative_target_dimension:dimension,source_origin_ref};
    if(row.component_body!==undefined)annotation.component_body=structuredClone(row.component_body);
    annotations.push(annotation);
    let target:JsonObject,targetPath:string,targetType:string,targetScope:string;
    if(disposition==='exported_quantity'){
      target={result_id:row.id,family:s!.family,object_ref,basis_ref,magnitude:row.value,unit:row.unit,dimension,provenance:e.provenance,source_kind:row.kind,source_row_index:index,source_annotation_ref:annotation_ref};if(md)target.metadata=md;if(row.component_body!==undefined)target.component_body=structuredClone(row.component_body);
      targetPath=`/result_envelope/result_sets/0/values/${values.length}`;values.push(target);targetType='quantity_result';targetScope='derived_quantity_row';
    }else if(disposition==='exported_review'){
      target={evidence_id:row.id,source_row_index:index,source_result_id:row.id,source_kind:row.kind,evidence_kind:category==='assembled_load_review'?'assembled_load_review':'user_input_review',magnitude:row.value,unit:row.unit,dimension,object_ref,basis_ref,location_ref:ref('source_location',row.metadata!.location),source_annotation_ref:annotation_ref,provenance:e.provenance};
      targetPath=`/result_envelope/review_evidence/${reviews.length}`;reviews.push(target);targetType='review_evidence';targetScope='derived_review_row';
    }else{
      const reason=classDisclosure?classDisclosure.code:reviewMissing?'review_metadata_incomplete':physicalMissing?'physical_metadata_incomplete':!s?'unsupported_source_kind':category==='basis_record'?'basis_annotation_not_quantity':category==='diagnostic_relative_ratio'?'diagnostic_relative_ratio_non_governing':['count','state','flag','solver_mode'].includes(category)?'discrete_evidence_not_ratio':'diagnostic_evidence_not_physical_quantity';
      target={source_row_index:index,source_result_id:row.id,source_kind:row.kind,source_value:row.value,source_unit:row.unit,source_dimension_present:observed.present,source_dimension:observed.value,declared_semantic_dimension:dimension,source_physical_semantic_dimension:math,semantic_category:category,reason_code:reason,object_ref,source_field_path:sourcePath,source_annotation_ref:annotation_ref,received_carrier_row_checksum:rowHash,original_producer_row_checksum:original,message:classDisclosure?classDisclosure.message:`${row.kind}: ${reason}; source value/unit and annotation retained; non-governing evidence`};
      targetPath=`/result_envelope/row_disclosures/${disclosures.length}`;disclosures.push(target);targetType='row_disclosure';targetScope='derived_disclosure_row';
    }
    const target_ref=ref(targetType,row.id);accounts.push({source_row_index:index,source_result_id:row.id,source_kind:row.kind,source_field_path:sourcePath,received_carrier_row_checksum:rowHash,original_producer_row_checksum:original,disposition,target_ref,target_field_path:targetPath});
    if(disposition!=='disclosed')witnesses.push({witness_id:`value-unit-witness:${index}`,source_result_ref:ref('received_result',row.id),source_field_path:sourcePath,source_quantity:{value:row.value,unit:row.unit,observed_dimension:observed},target_result_ref:target_ref,target_field_path:targetPath,target_quantity:{value:row.value,unit:row.unit,dimension},target_quantity_policy:'preserve_received_value_and_unit_with_independent_semantic_dimension',export_unit_policy:'no_numerical_conversion_preserve_received_unit',conversion_performed:false,unit_system_ref:e.unit_system_ref,provenance:e.provenance,source_row_index:index,received_carrier_row_checksum:rowHash,original_producer_row_checksum:original,source_physical_semantic_dimension:math,source_origin_ref,target_row_checksum:await scopedChecksum(target,targetScope,ref('derivative_document',e.envelope_id))});
  }
  e.result_sets[0].values=values;e.review_evidence=reviews;e.row_disclosures=disclosures;e.source_annotations=annotations;e.row_accounting=accounts;e.unit_preservation_witnesses=witnesses;e.unit_witness_policy='preserve_received_value_and_unit_with_independent_semantic_dimension';
  e.reproducibility.model_hash=await scopedChecksum(model,'model_payload',ref('model_payload',source.model_ref));e.reproducibility.raw_source_hashes=origin.authentic_producer_available?[structuredClone(origin.original_producer_checksum)]:[];e.reproducibility.source_origin_bindings=[origin];e.reproducibility.request_hash=request===null?null:await scopedChecksum(request,'request_payload',ref('runner_request','request'));
  e.reproducibility.derivative_hash_excludes='result_envelope.reproducibility.derivative_hash';delete e.reproducibility.derivative_hash;e.reproducibility.derivative_hash=await scopedChecksum(doc,'derivative_document_excludes_own_hash',ref('derivative_document',e.envelope_id));await validateResultDocument(doc,source);return doc;
}

function equalJson(a:any,b:any):boolean {
  if(a===b)return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object")return false;
  const keys=Object.keys(a),other=Object.keys(b);return Array.isArray(a)===Array.isArray(b)&&keys.length===other.length&&keys.every(k=>Object.hasOwn(b,k)&&equalJson(a[k],b[k]));
}
function requireEqual(a:any,b:any,code:string){if(!equalJson(a,b))throw new Error(code);}
function targetAt(doc:JsonObject,path:unknown):JsonObject {
  if(typeof path!=="string"||!path.startsWith("/result_envelope/"))throw new Error("TARGET_POINTER_INVALID");
  const target=path.slice(1).split('/').reduce((value:any,key)=>value?.[key],doc);if(!target||typeof target!=="object")throw new Error("TARGET_POINTER_UNRESOLVED");return target;
}
/** Validates source accounting and scoped references; it does not authenticate
 * the origin. Only the Current/source-solve entrypoints supply that evidence. */
export async function validateResultDocument(doc:JsonObject,source:MechanicsResult):Promise<void>{
  guardResultJson(doc);guardResultJson(source);if(Object.hasOwn(doc.result_envelope,'carrier_evidence'))throw new Error('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');const route=sourceContract(source);if(route==='unsupported')throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");refuseSurfaceRoute(source,'result-export');
  // T6S-4 (Rust `validate_document`): the successor's classes from the accepted reader (no invocation).
  const classes=await retainedRowClassesFromReader(source);
  // D2 4.9.7: the copied receipt equals the source's, whole; no other identity's document carries one.
  if(isRetainedRoute(route))requireEqual(doc.result_envelope.retained_precision,source.retained_precision,'RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH');
  else if(Object.hasOwn(doc.result_envelope,'retained_precision'))throw new Error(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
  if(resultSchemaVersion(doc)!==RESULT_EXPORT_VERSION)throw new Error("DERIVATIVE_VERSION_MISMATCH");
  for(const key of ['producer','numerical_quality','formulation_basis'] as const)requireEqual(doc.result_envelope[key],source[key],'SOURCE_NUMERICAL_METADATA_MISMATCH');requireEqual(doc.result_envelope.semantic_contract_ref,ref('semantic_contract',sourceSemanticBinding(source).id),'SEMANTIC_CONTRACT_MISMATCH');
  // Epoch 8 (Rust `validate_document`): pressure-source-1's receipt equals the source's,
  // whole; a pressure-2 document carries none (Rust's downgrade code); any other
  // identity's keeps the desktop's existing code.
  if(route==='pressure_source'){if(!Object.hasOwn(doc.result_envelope,'source_block_recovery'))throw new Error('SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH');requireEqual(doc.result_envelope.source_block_recovery,source.source_block_recovery,'SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH');}
  else if(Object.hasOwn(doc.result_envelope,'source_block_recovery'))throw new Error(route==='pressure'?'SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN':'SOURCE_RECOVERY_METADATA_CONTRADICTION');
  if(route==='preview_physics'||route==='pressure'||route==='pressure_source'||isRetainedRoute(route)){
    if(route==='preview_physics')readerValidate('preview-physics',source);
    else if(route==='pressure')readerValidate('pressure',source);
    else if(route==='pressure_source')readerValidate('pressure-source',source);
    requireEqual(doc.result_envelope.contract_evidence,source.contract_evidence,'SOURCE_PHYSICAL_METADATA_MISMATCH');
  }
  else if(Object.hasOwn(doc.result_envelope,'contract_evidence'))throw new Error('SOURCE_PHYSICAL_METADATA_MISMATCH');
  const e=doc.result_envelope,accounts=e.row_accounting,annotations=e.source_annotations,witnesses=e.unit_preservation_witnesses;
  if(!Array.isArray(accounts)||!Array.isArray(annotations)||accounts.length!==source.results.length||annotations.length!==source.results.length)throw new Error("ROW_ACCOUNTING_CARDINALITY");
  if(!Array.isArray(e.result_sets)||!e.result_sets.every((set:any)=>Array.isArray(set.values))||!Array.isArray(e.review_evidence)||!Array.isArray(e.row_disclosures)||!Array.isArray(witnesses))throw new Error("TARGET_ARRAYS_INVALID");
  if(e.result_sets.reduce((n:number,set:any)=>n+set.values.length,0)+e.review_evidence.length+e.row_disclosures.length!==source.results.length)throw new Error("TARGET_ACCOUNTING_CARDINALITY");
  const origins=e.reproducibility.source_origin_bindings;if(!Array.isArray(origins)||origins.length!==1)throw new Error("ORIGIN_CARDINALITY");const origin=origins[0],originRef=ref('source_origin_binding',origin.origin_id);
  requireEqual(origin.actual_model_ref,ref('model_payload',source.model_ref),'ORIGIN_IDENTITY');requireEqual(e.model_ref,origin.actual_model_ref,'ORIGIN_IDENTITY');requireEqual(origin.mechanics_run_ref,ref('mechanics_run',source.run_id),'ORIGIN_IDENTITY');
  const scope=origin.received_carrier_checksum.payload_scope;
  const rowScope=scope==='received_current_dimension_absent_carrier'?'received_current_dimension_absent_row':scope==='attested_headless_producer_carrier'?'raw_source_row':null;if(!rowScope)throw new Error('ORIGIN_SCOPE');
  requireEqual(origin.received_carrier_checksum,await scopedChecksum(source,scope,origin.received_carrier_checksum.payload_ref),'CARRIER_HASH');
  if(scope==='received_current_dimension_absent_carrier'&&source.results.some(row=>Object.hasOwn(row,'dimension')))throw new Error('CARRIER_DIMENSION_PRESENT');
  if(origin.authentic_producer_available)requireEqual(origin.original_producer_checksum,origin.received_carrier_checksum,'PRODUCER_HASH_BINDING');else if(origin.original_producer_checksum!==null)throw new Error('UNAVAILABLE_PRODUCER_HASH');
  const pointers=new Set<string>(),ids=new Set<string>();let wi=0;const rowSemantics=readerRowSemantics(source);
  for(const [i,row] of source.results.entries()){
    const a=accounts[i],ann=annotations[i],r=rowSemantics(row),s=r.signature,md=r.canonical_metadata,dimension=s?.derivative_target_dimension??null,math=s?.source_physical_semantic_dimension??null;
    let disposition=s?.canonical_disposition??'disclosed';if(disposition==='exported_review'&&!r.complete_metadata||disposition==='exported_quantity'&&['force','moment','section_property'].includes(s?.family??'')&&!md)disposition='disclosed';
    const classDisclosure=retainedClassDisclosure(row.kind,row.unit,classes?.get(row.id));if(classDisclosure)disposition='disclosed';
    if(ids.has(row.id))throw new Error('DUPLICATE_SOURCE_ID');ids.add(row.id);
    requireEqual([a.source_row_index,a.source_result_id,a.source_kind,a.source_field_path,a.disposition],[i,row.id,row.kind,`/results/${i}`,disposition],'SOURCE_ACCOUNTING_IDENTITY');
    const target=targetAt(doc,a.target_field_path);if(pointers.has(a.target_field_path))throw new Error('TARGET_DUPLICATE');pointers.add(a.target_field_path);
    const type=disposition==='exported_quantity'?'quantity_result':disposition==='exported_review'?'review_evidence':'row_disclosure',prefix=disposition==='exported_quantity'?'/result_envelope/result_sets/0/values/':disposition==='exported_review'?'/result_envelope/review_evidence/':'/result_envelope/row_disclosures/';
    if(!a.target_field_path.startsWith(prefix)||!/^\d+$/.test(a.target_field_path.slice(prefix.length)))throw new Error('TARGET_POINTER_INVALID');
    requireEqual(a.target_ref,ref(type,row.id),'TARGET_REF');requireEqual([target.source_row_index,target.source_kind,target.result_id??target.evidence_id??target.source_result_id],[i,row.kind,row.id],'TARGET_IDENTITY');
    requireEqual(target.object_ref,ref('preview_entity',row.entity_ref),'TARGET_OBJECT');
    const observed={present:Object.hasOwn(row,'dimension'),value:row.dimension??null},annRef=ref('source_annotation',`source-annotation:${i}`);
    requireEqual(ann,{annotation_id:annRef.ref_id,source_row_index:i,source_result_id:row.id,metadata:row.metadata??null,basis_ref:row.basis_ref??null,source_result_refs:row.source_result_refs??[],observed_carrier_dimension:observed,source_physical_semantic_dimension:math,derivative_target_dimension:dimension,source_origin_ref:originRef,...(row.component_body!==undefined?{component_body:row.component_body}:{})},'SOURCE_ANNOTATION');requireEqual(target.source_annotation_ref,annRef,'ANNOTATION_REF');
    // W-15 slice d (Rust `derivative_agrees`): a quantity carries its row's record verbatim; no other target carries one;
    // no target carries the coverage-only body list (the annotation is held whole above).
    if(Object.hasOwn(target,'component_body_pipe_ids')||(disposition==='exported_quantity'?!equalJson(target.component_body,row.component_body):Object.hasOwn(target,'component_body')))throw new Error('SOURCE_COMPONENT_BODY_MISMATCH');
    const rowHash=await scopedChecksum(row,rowScope,origin.received_carrier_checksum.payload_ref),original=origin.authentic_producer_available?await scopedChecksum(row,'raw_source_row',origin.original_producer_checksum.payload_ref):null;
    requireEqual(a.received_carrier_row_checksum,rowHash,'SOURCE_ROW_HASH');requireEqual(a.original_producer_row_checksum,original,'SOURCE_ROW_HASH');
    requireEqual(disposition==='disclosed'?[target.source_value,target.source_unit]:[target.magnitude,target.unit],[row.value,row.unit],'SOURCE_TARGET_VALUE');
    if(disposition==='disclosed'){
      requireEqual([target.declared_semantic_dimension,target.source_physical_semantic_dimension,target.semantic_category,target.source_dimension_present,target.source_dimension,target.received_carrier_row_checksum,target.original_producer_row_checksum],[dimension,math,s?.category??'unknown',observed.present,observed.value,rowHash,original],'DISCLOSURE_SEMANTICS');
      // D-U6-2: a class disclosure states exactly its class code and message; no other
      // disclosure of a successor claims a class code (Rust `validate_document`).
      if(classes){const claimed=target.reason_code===RETAINED_ABSOLUTE_VERIFIED||target.reason_code===RETAINED_NOT_COVERED;if(classDisclosure?target.reason_code!==classDisclosure.code||target.message!==classDisclosure.message:claimed)throw new Error('DISCLOSURE_SEMANTICS');}
    }else{
      requireEqual(target.dimension,dimension,'TARGET_DIMENSION');requireEqual(target.basis_ref,row.basis_ref??(disposition==='exported_review'?ref('source_basis',row.metadata!.basis):e.run_ref),'TARGET_BASIS');requireEqual(target.provenance,e.provenance,'TARGET_PROVENANCE');if(disposition==='exported_review'){requireEqual(target.source_result_id,row.id,'REVIEW_ID');requireEqual(target.location_ref,ref('source_location',row.metadata!.location),'REVIEW_LOCATION');requireEqual(target.evidence_kind,s?.category==='assembled_load_review'?'assembled_load_review':'user_input_review','REVIEW_KIND');}if(disposition==='exported_quantity'){requireEqual(target.family,s!.family,'TARGET_FAMILY');requireEqual(target.metadata??null,md,'TARGET_METADATA');}
      const w=witnesses[wi++];if(!w)throw new Error('WITNESS_CARDINALITY');requireEqual(w.unit_system_ref,e.unit_system_ref,'WITNESS_UNIT_SYSTEM');requireEqual(w.provenance,e.provenance,'WITNESS_PROVENANCE');
      requireEqual([w.witness_id,w.source_row_index,w.source_result_ref,w.source_field_path,w.source_quantity,w.target_result_ref,w.target_field_path,w.target_quantity,w.received_carrier_row_checksum,w.original_producer_row_checksum,w.source_physical_semantic_dimension,w.source_origin_ref,w.conversion_performed,w.target_quantity_policy,w.export_unit_policy],[`value-unit-witness:${i}`,i,ref('received_result',row.id),a.source_field_path,{value:row.value,unit:row.unit,observed_dimension:observed},a.target_ref,a.target_field_path,{value:row.value,unit:row.unit,dimension},rowHash,original,math,originRef,false,'preserve_received_value_and_unit_with_independent_semantic_dimension','no_numerical_conversion_preserve_received_unit'],'WITNESS_BINDING');
      requireEqual(w.target_row_checksum,await scopedChecksum(target,disposition==='exported_quantity'?'derived_quantity_row':'derived_review_row',ref('derivative_document',e.envelope_id)),'WITNESS_TARGET_HASH');
    }
  }
  if(wi!==witnesses.length)throw new Error('WITNESS_CARDINALITY');const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;
  requireEqual(e.reproducibility.derivative_hash,await scopedChecksum(payload,'derivative_document_excludes_own_hash',ref('derivative_document',e.envelope_id)),'DERIVATIVE_HASH');
}

/** The origin limit of a qualified Current received carrier (the builder's own text). */
export const CURRENT_ORIGIN_LIMIT='Qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation';
/** The received-carrier origin that `buildCurrentResultExport` binds, from its qualified inputs.
 * Exported, with the base below, so that the T6S golden test builds the same desktop-shaped origin
 * and base from fixed stand-ins (I76's inputs, with a test-labelled limit). Neither function
 * qualifies, authenticates or admits anything: only the Current builder's checks do. */
export async function currentReceivedOrigin(model:PreviewModel,result:MechanicsResult,manifestRef:string,originLimit:string):Promise<JsonObject>{
  const scope='received_current_dimension_absent_carrier';
  return {origin_id:'source-origin:current-received',origin_class:'received_current_dimension_absent',qualification_ref:ref('current_manifest',manifestRef),authentic_producer_available:false,received_carrier_checksum:await scopedChecksum(result,scope,ref('received_current_carrier',result.run_id)),original_producer_checksum:null,origin_limit:originLimit,actual_model_ref:ref('model_payload',model.project.id),mechanics_run_ref:ref('mechanics_run',result.run_id),request_model_ref:null,request_run_ref:null,request_alias_disclosure:null};
}
/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.2, §6.2): an exact-route
 * document names its load bases `load_case`, as Rust's derivative does. Other identities
 * keep the record's `object_type`, so their documents are unchanged (§10). */
function exactLoadCase(result:MechanicsResult,objectType:string):boolean{
  if(objectType!=='LoadCase')return false;
  try{const route=sourceContract(result);return route==='pressure'||route==='pressure_source';}catch{return false;}
}
type CurrentDocumentRun=Pick<AnalysisRunEnvelope['analysis_run'],'run_id'|'load_basis_refs'|'hashes'|'analysis_status'|'professional_boundary'>&Partial<Pick<AnalysisRunEnvelope['analysis_run'],'diagnostics'>>;
/** The document base that `buildCurrentResultExport` derives from (see `currentReceivedOrigin`). */
export function currentResultDocumentBase(model:PreviewModel,result:MechanicsResult,run:CurrentDocumentRun,manifestRef:string,solverBasis:{solver_name:string;solver_version:string;solver_build_ref:string}):JsonObject{
  const provenance=derivativeProvenance;
  return {schema_version:RESULT_EXPORT_VERSION,deliverable_id:'DEL-08-04',package_id:'PKG-08',scope_item:'SOW-046',objectives:['OBJ-007','OBJ-009'],export_format_status:{baseline_format:'schema_first_json_result_envelope',additional_formats:'TBD',public_transport_protocol:'TBD',local_fea_package_format:'TBD',external_adapter_formats:'TBD'},result_envelope:{schema_version:RESULT_EXPORT_VERSION,envelope_id:`result-envelope:${result.run_id}`,model_ref:ref('model_payload',model.project.id),run_ref:ref('analysis_run',run.run_id),solver_version:{solver_name:solverBasis.solver_name,solver_version:solverBasis.solver_version,solver_build_ref:solverBasis.solver_build_ref},unit_system_ref:ref('unit_system',`${model.project.id}:units`),load_basis_refs:run.load_basis_refs.map(x=>exactLoadCase(result,x.object_type)?ref('load_case',x.ref):ref(x.object_type,x.ref)),result_sets:[{set_id:`result-set:${result.run_id}:mechanics`,set_type:'mechanics',basis_ref:ref('analysis_run',run.run_id),values:[]}],diagnostics:documentDiagnostics(result,run.diagnostics),provenance,reproducibility:{model_hash:null,run_hashes:run.hashes.map(x=>({algorithm:x.algorithm,canonicalization:x.canonicalization,payload_ref:ref(x.payload_ref.object_type,x.payload_ref.ref),value:x.value})),audit_manifest_ref:ref('audit_manifest',manifestRef),deterministic_ordering:true},analysis_status:run.analysis_status,professional_boundary:run.professional_boundary,downstream_use:{review:true,regression_comparison:true,report_consumption:true,headless_automation:true,governed_downstream_tooling:true,additional_export_formats:'TBD'}}};
}
export async function buildCurrentResultExport({model,result,analysisRun,inputManifest}:{model:PreviewModel;result:MechanicsResult;analysisRun:AnalysisRunEnvelope;inputManifest:CurrentSessionInputManifestEvidence|null|undefined}):Promise<JsonObject>{
  guardResultJson(model);guardResultJson(result);guardResultJson(analysisRun);
  // T6S-3: this surface's policy entry; a retained-precision successor is admitted only at
  // numerically eligible standing with the live native capture. Epoch 8: the exact routes
  // (load/reference-state results included) add no policy refusal here; the standing
  // check below takes Rust's verdict for them.
  refuseSurfaceOutput(result,model,'result-export');
  if(!numericalResultStanding(result,model).eligible)throw new Error('CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE');
  if(!isFreshSemanticResult(result))throw new Error('CURRENT_SOURCE_IDENTITY_NOT_FRESH');
  if(!inputManifest)throw new Error('CURRENT_INPUT_MANIFEST_UNAVAILABLE');guardResultJson(inputManifest);
  const currentBindingText=checkedJsonText({model,result,analysisRun,inputManifest});
  await verifyCurrentSessionInputManifest(inputManifest);
  if(await canonicalJsonString(model)!==await canonicalJsonString(inputManifest.manifest.model_basis.model_payload)||result.model_ref!==model.project.id)throw new Error('CURRENT_MODEL_PAYLOAD_MISMATCH');
  if(analysisRun.schema_version!=='0.3.0')throw new Error('CURRENT_ANALYSIS_VERSION_UNSUPPORTED');
  const proofChecksum=(h:any,scope:string,type:string,id:string,profile:string='rfc8785_jcs')=>h?.algorithm==='sha256'&&h.canonicalization===profile&&h.payload_scope===scope&&h.payload_ref?.object_type===type&&h.payload_ref.ref===id&&/^[0-9a-f]{64}$/.test(h.value);
  const run=analysisRun.analysis_run;
  requireEqual(run.reproducibility.semantic_contract,sourceSemanticBinding(result),'CURRENT_SEMANTIC_CONTRACT_MISMATCH');
  const solver=inputManifest.manifest.solver_basis;
  if(solver.solver_name!==result.producer!.component_name||solver.solver_version!==result.producer!.component_version)throw new Error('CURRENT_PRODUCER_BINDING_MISMATCH');
  requireEqual(run.solver_version,{solver_name:solver.solver_name,solver_version:solver.solver_version,build_ref:{object_type:'ExternalReference',ref:solver.solver_build_ref}},'CURRENT_SOLVER_BINDING_MISMATCH');
  const expectedState=`state:${result.model_ref}:preview`;
  if(analysisRun.deliverable_id!=='DEL-14-02'||run.model_state_ref.object_type!=='ModelState'||run.model_state_ref.ref!==expectedState)throw new Error('CURRENT_MODEL_STATE_BINDING_MISMATCH');if(run.run_id!==result.run_id||!run.analysis_status.includes('MECHANICS_SOLVED')||!run.analysis_status.includes('HUMAN_REVIEW_REQUIRED')||result.status.mechanics!=='MECHANICS_SOLVED')throw new Error('CURRENT_RUN_BINDING_MISMATCH');
  const hashes=run.reproducibility.input_manifest_hashes,refs=run.reproducibility.input_manifest_refs;
  if(hashes.length!==1||refs.length!==1||refs[0].object_type!=='InputManifest'||!proofChecksum(hashes[0],'input_manifest','InputManifest',inputManifest.manifest_ref.ref)||hashes[0].value!==inputManifest.manifest_sha256||hashes[0].payload_ref.ref!==inputManifest.manifest_ref.ref||refs[0].ref!==inputManifest.manifest_ref.ref)throw new Error('CURRENT_MANIFEST_BINDING_MISMATCH');
  // b7-plan E2: a current producer carrier declares no row dimension; a carrier that does is not current.
  if(result.results.some(row=>Object.hasOwn(row,'dimension')))throw new Error('CURRENT_CARRIER_DIMENSION_PRESENT');
  if(run.hashes.length!==2)throw new Error('CURRENT_HASH_INVENTORY_MISMATCH');
  const envelopeScope='received_result';
  const envelopeHashes=run.hashes.filter(h=>h.payload_scope===envelopeScope);
  const expectedEnvelope=await canonicalSha256HexCheckedV1(result);
  if(envelopeHashes.length!==1||!proofChecksum(envelopeHashes[0],envelopeScope,'ResultEnvelope',`result-envelope:${result.run_id}`,canonicalization)||envelopeHashes[0].value!==expectedEnvelope||run.result_refs.length!==result.results.length)throw new Error('CURRENT_RESULT_BINDING_MISMATCH');
  const seen=new Set<string>();for(const row of result.results){const rr=run.result_refs.find(x=>x.result_ref.ref===row.id);const expectedRow=await canonicalSha256HexCheckedV1(row);if(seen.has(row.id)||!rr||rr.result_ref.object_type!=='Result'||!proofChecksum(rr.hash_refs[0],'result_row','Result',row.id,canonicalization)||rr.hash_refs.length!==1||rr.hash_refs[0].value!==expectedRow)throw new Error('CURRENT_ROW_BINDING_MISMATCH');seen.add(row.id);}
  const ruleStatus=run.analysis_status.find(s=>['RULE_INPUTS_INCOMPLETE','USER_RULE_CHECKED','USER_RULE_FAILED'].includes(s));if(!ruleStatus||JSON.stringify([...run.analysis_status].sort())!==JSON.stringify(['HUMAN_REVIEW_REQUIRED','MECHANICS_SOLVED',ruleStatus].sort()))throw new Error('CURRENT_STATUS_BINDING_MISMATCH');
  if(await verifyAnalysisRunRecord(analysisRun)!=='match')throw new Error('CURRENT_ANALYSIS_RECORD_MISMATCH');
  await validateAnalysisRunV03(analysisRun,result,modelLoadBasisRefs(model));
  if(run.professional_boundary.human_review_required!==true||Object.entries(run.professional_boundary).some(([k,v])=>k!=='human_review_required'&&v!==false))throw new Error('CURRENT_BOUNDARY_MISMATCH');
  if(!hasNativeMechanicsInvocation(result,model,solver.solver_mode))throw new Error('CURRENT_NATIVE_INVOCATION_REQUIRED');
  const origin=await currentReceivedOrigin(model,result,inputManifest.manifest_ref.ref,CURRENT_ORIGIN_LIMIT);
  const base=currentResultDocumentBase(model,result,run,inputManifest.manifest_ref.ref,inputManifest.manifest.solver_basis);
  if(!hasNativeMechanicsInvocation(result,model,solver.solver_mode))throw new Error('CURRENT_NATIVE_INVOCATION_UNAVAILABLE');
  const document=await deriveResultDocument(base,model,result,origin);
  if(checkedJsonText({model,result,analysisRun,inputManifest})!==currentBindingText || !hasNativeMechanicsInvocation(result,model,solver.solver_mode) || !numericalResultStanding(result,model).eligible)throw new Error('CURRENT_BINDING_CHANGED_DURING_EXPORT');
  return document;
}
export function resultSchemaVersion(value:JsonObject):typeof RESULT_EXPORT_VERSION{const v=value.schema_version;if(v!==RESULT_EXPORT_VERSION||value.result_envelope?.schema_version!==v)throw new Error('RESULT_VERSION_UNSUPPORTED_OR_MIXED');return v;}
