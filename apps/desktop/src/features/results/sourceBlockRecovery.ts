import physicsSchema from '../../../../../schemas/results.v0.3.schema.yaml?raw';
import compositeSchema from '../../../../../schemas/physics_source_recovery.schema.json';
import compositeContract from '../../../../../fixtures/results/semantic_contract_v0_3_physics_source_1.json';
import { validatePhysicsSourceTransportMetadata, validatePhysicsSourceCase, validatePhysicsSourceDerived, PHYSICS_SOURCE_SUPPORT_SIGN } from './physicsSourceRecovery';
import { validatePhysicsSourceEvidence } from './physicsResultEvidence';
import receiptSchema from '../../../../../schemas/source_block_recovery.schema.json';
import contract from '../../../../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json';
import { canonicalSha256HexCheckedV1, checkedJsonText } from '../../services/hashService';
import { loadWasmEngine } from '../../services/wasmEngine/loadWasmEngine';
import type { MechanicsResult, PreviewModel } from '../../types';

export const SOURCE_BLOCKS_CONTRACT_ID = 'openpipestress.result_semantics/0.3.0/source-blocks-1';
export const SOURCE_BLOCKS_CONTRACT_SHA256 = '5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f';
export type SourceBlockInvocation = Readonly<{ request: unknown; solver_mode: 'dense_scrutiny' | 'sparse_interactive' }>;
type JsonObject = Record<string, any>;
export type SourceRecoveryValidation = { eligible: boolean; findings: string[] };
type Validation = SourceRecoveryValidation;
type Validated = Validation & { method: string } & { sourceText: string; sourceZeroSigns: string; invocationText: string; invocationZeroSigns: string; invocation: SourceBlockInvocation; callerModel?: PreviewModel; callerSnapshot?: PreviewModel };
// A registration is private and grants only receipt consistency. Current-use
// callers must still authenticate the actual model, producer/build and run.
const validated = new WeakMap<MechanicsResult, Validated>();
// Only this validator adds receipts after recursively freezing their already
// checked JSON graph. Mutable/untrusted identities never bypass shape checks.
const immutableReceiptShapes = new WeakSet<object>();
function sealValidatedReceipt(value: unknown, seen = new Set<object>()): void {
  if (!value || typeof value !== 'object' || seen.has(value)) return;
  seen.add(value);
  for (const descriptor of Object.values(Object.getOwnPropertyDescriptors(value))) {
    if (Object.hasOwn(descriptor,'value')) sealValidatedReceipt(descriptor.value, seen);
  }
  Object.freeze(value);
}
const fail = (condition: unknown, code: string): void => { if (!condition) throw new Error(`SOURCE_BLOCKS_${code}`); };
const unique = (values: unknown[]) => new Set(values).size === values.length;
const same = (a: unknown, b: unknown) => checkedJsonText(a) === checkedJsonText(b);

// JSON Schema const/enum compare JSON values structurally, with numeric -0
// equal to 0. This is deliberately separate from bit-sensitive source custody.
function schemaEqual(a: any, b: any): boolean {
  if (a === b) return true;
  if (!a || !b || typeof a !== 'object' || typeof b !== 'object' || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a);
  return keys.length === Object.keys(b).length && keys.every(key => Object.hasOwn(b, key) && schemaEqual(a[key], b[key]));
}

/** Closed, bounded interpreter for this maintained schema's small keyword set. */
function shape(value: any, rule: JsonObject | boolean, root: JsonObject, depth = 0): boolean {
  if (depth > 40 || rule === false || !rule) return false;
  if (rule === true) return true;
  if (rule.$ref) return shape(value, root.$defs[rule.$ref.replace('#/$defs/', '')], root, depth + 1);
  if (rule.anyOf && !rule.anyOf.some((r: JsonObject) => shape(value, r, root, depth + 1))) return false;
  if (rule.oneOf && rule.oneOf.filter((r: JsonObject) => shape(value, r, root, depth + 1)).length !== 1) return false;
  if (rule.allOf && !rule.allOf.every((r: JsonObject) => shape(value, r, root, depth + 1))) return false;
  if (Object.hasOwn(rule, 'const') && !schemaEqual(value, rule.const)) return false;
  if (rule.enum && !rule.enum.some((candidate: unknown) => schemaEqual(value, candidate))) return false;
  if (Array.isArray(rule.type)) return rule.type.some((type: string) => shape(value, { ...rule, type }, root, depth + 1));
  if (rule.type === 'null') return value === null;
  if (rule.type === 'boolean' && typeof value !== 'boolean') return false;
  if (rule.type === 'string' && (typeof value !== 'string' || (rule.minLength && value.length < rule.minLength) || (rule.pattern && !new RegExp(rule.pattern).test(value)))) return false;
  if (rule.type === 'number' || rule.type === 'integer') {
    if (typeof value !== 'number' || !Number.isFinite(value) || (rule.type === 'integer' && !Number.isSafeInteger(value))) return false;
    if (rule.minimum !== undefined && value < rule.minimum || rule.maximum !== undefined && value > rule.maximum
      || rule.exclusiveMinimum !== undefined && value <= rule.exclusiveMinimum || rule.exclusiveMaximum !== undefined && value >= rule.exclusiveMaximum) return false;
  }
  if (rule.type === 'array' && !Array.isArray(value)) return false;
  if (Array.isArray(value)) {
    if (value.length > (rule.maxItems ?? 16384) || value.length < (rule.minItems ?? 0)) return false;
    if (!value.every(v => shape(v, rule.items ?? true, root, depth + 1))) return false;
  }
  if (rule.type === 'object' && (!value || typeof value !== 'object' || Array.isArray(value))) return false;
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    if (rule.required?.some((key: string) => !Object.hasOwn(value, key))) return false;
    if (rule.additionalProperties === false && Object.keys(value).some(key => !Object.hasOwn(rule.properties ?? {}, key))) return false;
    if (Object.entries(rule.properties ?? {}).some(([key, r]) => Object.hasOwn(value, key) && !shape(value[key], r as JsonObject, root, depth + 1))) return false;
  }
  return true;
}
export function sourceBlockReceiptShape(value: unknown): boolean {
  if (value && typeof value === 'object' && immutableReceiptShapes.has(value) && (value as JsonObject).body?.policy === 'SOURCE-BLOCKS-1') return true;
  try { checkedJsonText(value); return shape(value, receiptSchema, receiptSchema); } catch { return false; }
}
/** Fixed composite schema context; callers cannot select or replace schema rules. */
export function physicsSourceReceiptShape(value: unknown): boolean {
  if (value && typeof value === 'object' && immutableReceiptShapes.has(value) && (value as JsonObject).body?.policy === 'PHYSICS-SOURCE-1') return true;
  try { checkedJsonText(value); return shape(value, compositeSchema, compositeSchema); } catch { return false; }
}
export function physicsSourcePhysicalShape(value: unknown): boolean {
  try { checkedJsonText(value); return shape(value, compositeSchema.$defs.physical_evidence, compositeSchema); } catch { return false; }
}
const physicsEvidenceSchema: JsonObject = JSON.parse(physicsSchema);
export function physicsEvidenceTransportShape(value: unknown): boolean {
  try { checkedJsonText(value); return shape(value, physicsEvidenceSchema.$defs.PhysicsContractEvidence, physicsEvidenceSchema); } catch { return false; }
}
/** Portable statement check only; does not populate the private registry. */
export async function validateSourceBlockTransportMetadata(source: Pick<MechanicsResult, 'producer' | 'source_block_recovery'>): Promise<void> {
  checkedJsonText(source);
  fail(same(source.producer, { component_name: 'open_pipe_stress_product_physics', component_version: '0.2.0', semantic_contract_id: SOURCE_BLOCKS_CONTRACT_ID }), 'PRODUCER');
  fail(!Object.hasOwn(source, 'contract_evidence') && !Object.hasOwn(source, 'carrier_evidence') && sourceBlockReceiptShape(source.source_block_recovery), 'TRANSPORT_SHAPE');
  const snapshot = structuredClone(source), receipt = snapshot.source_block_recovery as JsonObject;
  fail(receipt.receipt_sha256 === await domainHash('source_blocks_receipt_v1', receipt.body), 'RECEIPT_HASH');
  fail(sameModelData(source, snapshot), 'TRANSPORT_CHANGED_DURING_VALIDATION');
}
const bits = (value: number): string => { const b = new DataView(new ArrayBuffer(8)); b.setFloat64(0, value); return b.getBigUint64(0).toString(16).padStart(16, '0'); };
const domainHash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });

/** Validate actual wire evidence; this does not replay hidden exact arithmetic.
 * Capture invocation independently at dispatch, never from the returned receipt. */
export async function validateSourceBlockRecovery(source: MechanicsResult, invocation: SourceBlockInvocation, callerModel?: PreviewModel): Promise<Validation> {
  return validateKnownSourceRecovery(source, invocation, callerModel, false);
}
/** Explicit composite wrapper shares only the private receipt-consistency registry. */
export async function validatePhysicsSourceReceiptRecovery(source: MechanicsResult, invocation: SourceBlockInvocation, callerModel?: PreviewModel): Promise<Validation> {
  return validateKnownSourceRecovery(source, invocation, callerModel, true);
}
async function validateKnownSourceRecovery(source: MechanicsResult, invocation: SourceBlockInvocation, callerModel: PreviewModel | undefined, composite: boolean): Promise<Validation> {
  const method = composite ? 'openpipestress.result_semantics/0.3.0/physics-source-1' : SOURCE_BLOCKS_CONTRACT_ID;
  const semantic = (row: MechanicsResult['results'][number]) => (composite ? compositeContract.rows : contract.rows).find(s => s.kind === row.kind && s.unit === row.unit && (s.component === null || s.component === row.metadata?.component) && (!('source_basis' in s) || s.source_basis === row.metadata?.basis));
  const physical = (row: MechanicsResult['results'][number]) => !semantic(row) || semantic(row)!.category === 'physical_quantity';
  validated.delete(source);
  const sourceText = checkedJsonText(source), invocationText = checkedJsonText(invocation);
  // All awaits operate on snapshots. Neither a mutable caller nor a racing edit
  // can alter the bytes checked and then reuse the resulting registration.
  const raw = structuredClone(source);
  const sourceZeroSigns = negativeZeroPaths(source);
  // Optional context comes from the native boundary before serialization. It
  // adds an observed caller representation, never an inferred model from rows.
  if (callerModel) checkedJsonText(callerModel);
  const callerSnapshot = callerModel ? structuredClone(callerModel) : undefined;
  const captured = structuredClone(invocation);
  const invocationZeroSigns = negativeZeroPaths(invocation);
  fail(raw.schema_version === '0.2.0' && raw.producer?.semantic_contract_id === method, 'CONTRACT');
  const allowed = ['schema_version','producer','numerical_quality','formulation_basis','document_kind','run_id','model_ref','status','summary','results','diagnostics','professional_boundary','accepted_model_state_mutated','source_block_recovery'];
  if (composite) allowed.push('contract_evidence');
  fail(Object.keys(raw).every(k => allowed.includes(k)), 'ENVELOPE_FIELDS');
  if (composite) {
    fail(allowed.every(k => Object.hasOwn(raw, k)), 'ENVELOPE_FIELDS');
    fail(Object.keys(captured).length === 2 && Object.hasOwn(captured, 'request') && Object.hasOwn(captured, 'solver_mode'), 'INVOCATION_SHAPE');
    fail(raw.formulation_basis && Object.keys(raw.formulation_basis).length === 2 && raw.formulation_basis.profile_id === 'exact_straight_pressure_v2'
      && Array.isArray(raw.formulation_basis.limitations) && raw.formulation_basis.limitations.length && raw.formulation_basis.limitations.every(x => typeof x === 'string' && x.length), 'FORMULATION');
    fail(Array.isArray(raw.results) && raw.results.length <= 16384 && Array.isArray(raw.diagnostics) && raw.diagnostics.length <= 16384, 'ROWS');
    fail(raw.results.every(r => !r.metadata || Object.keys(r.metadata).length === 5 && ['component','coordinate_system','location','basis','sign_convention'].every(k => typeof (r.metadata as JsonObject)[k] === 'string' && (r.metadata as JsonObject)[k].length > 0)), 'ROW_METADATA');
    const q = raw.numerical_quality;
    fail(q && Object.keys(q).length === 5 && q.value_representation === 'finite_binary64' && q.publication_quantization === 'none' && q.integrity_policy === 'M03-INTEGRITY-v1' && Array.isArray(q.cases), 'ORDINARY_QUALITY');
    fail(q!.cases.every(c => Object.keys(c).length === 6 && ['checks_passed','sensitive','not_assessed','unresolved','failed'].includes(c.solve_quality)
      && ['passive_model_basis','physical_mechanism_witnessed','negative_energy_witnessed','numerically_unresolved'].includes(c.structural_status)
      && ['represented_equations_retained','assembly_loss_detected','assembly_uncertainty','not_assessed'].includes(c.model_matrix_fidelity)
      && ['not_claimed','reference_verified','unresolved'].includes(c.accuracy_evidence) && Array.isArray(c.evidence_refs)), 'ORDINARY_CASE');
  }
  fail((composite ? physicsSourceReceiptShape : sourceBlockReceiptShape)(raw.source_block_recovery), 'RECEIPT_SHAPE');
  const receipt = raw.source_block_recovery as JsonObject, body = receipt.body;
  const request = captured.request as JsonObject, model = request?.model as PreviewModel;
  fail(model && Array.isArray(model.load_cases) && Array.isArray(model.nodes) && model.nodes.length > 0 && Array.isArray(model.pipe_segments) && Array.isArray(model.supports), 'INVOCATION_MODEL_REQUIRED');
  fail(model.project?.id === raw.model_ref, 'MODEL_MISMATCH');
  if (composite) { await validatePhysicsSourceTransportMetadata(raw); validatePhysicsSourceEvidence(raw, model); }
  if (callerSnapshot) fail(sameModelData(JSON.parse(checkedJsonText(callerSnapshot)), model), 'CALLER_DISPATCH_NORMALIZATION_MISMATCH');
  for (const items of [model.nodes,model.pipe_segments,model.supports,model.load_cases]) fail(unique(items.map(x => x.id)) && items.every(x => typeof x.id === 'string' && x.id.length > 0), 'MODEL_IDENTITIES');
  fail(raw.results.every(row => Object.keys(row).every(k => ['id','kind','value','unit','dimension','entity_ref','basis_ref','source_result_refs','metadata'].includes(k)) && Number.isFinite(row.value) && typeof row.entity_ref === 'string' && row.entity_ref.length > 0), 'ROW_SHAPE');
  fail(raw.results.every(row => !Object.hasOwn(row,'dimension') || semantic(row)?.legacy_declared_dimension === row.dimension), 'ROW_DIMENSION');
  fail(['dense_scrutiny','sparse_interactive'].includes(captured.solver_mode), 'MODE');
  const withoutReceipt = { ...raw }; delete withoutReceipt.source_block_recovery;
  fail(body.invocation.value === await domainHash('source_blocks_invocation_v1', captured), 'INVOCATION_HASH');
  fail(body.publication_sha256 === await domainHash('source_blocks_publication_v1', withoutReceipt), 'PUBLICATION_HASH');
  fail(receipt.receipt_sha256 === await domainHash('source_blocks_receipt_v1', body), 'RECEIPT_HASH');
  const cases = body.cases as JsonObject[], requested = model.load_cases.map(c => c.id), q = raw.numerical_quality!;
  fail(requested.length && unique(requested) && cases.length === requested.length && q?.cases.length === requested.length, 'CASE_COVERAGE');
  const rowIds = raw.results.map(r => r.id), diagnosticIds = raw.diagnostics.map(d => d.id);
  fail([...rowIds,...diagnosticIds].every(id => typeof id === 'string' && id.length > 0) && unique([...rowIds,...diagnosticIds]), 'EVIDENCE_IDS');
  const byId = new Map(raw.results.map(r => [r.id, r])), accounted = new Set<string>();
  const findings: string[] = [];
  let chargedCases = 0;
  const qualityOrder = ['checks_passed','sensitive','not_assessed','unresolved','failed'];
  fail(q.status === qualityOrder[Math.max(...q.cases.map(c => qualityOrder.indexOf(c.solve_quality)))], 'ORDINARY_AGGREGATE');
  const diagnosticById = new Map(raw.diagnostics.map(d => [d.id,d]));
  const diag = (id: unknown) => typeof id === 'string' && diagnosticIds.includes(id);
  const account = (id: string) => { fail(byId.has(id) && !accounted.has(id), 'ROW_COVERAGE'); accounted.add(id); return byId.get(id)!; };
  for (const [index, c] of cases.entries()) {
    fail(c.basis_ref.ref_id === requested[index] && q.cases[index].basis_ref.ref_type === 'load_case' && q.cases[index].basis_ref.ref_id === requested[index], 'CASE_ORDER');
    const o = c.ordinary_attempt, qc = q.cases[index];
    fail(c.requested_mode === captured.solver_mode && o.requested_mode === captured.solver_mode && o.quality_case_index === index, 'MODE_QUALITY_BINDING');
    fail(o.structural_report_diagnostic_ref === null || diag(o.structural_report_diagnostic_ref), 'ORDINARY_REPORT');
    if (o.outcome === 'not_attempted') fail(o.structural_report_diagnostic_ref === null && o.failure === null && qc.solve_quality === 'not_assessed', 'ORDINARY_NOT_ATTEMPTED');
    if (o.outcome === 'checks_passed' || o.outcome === 'sensitive') fail(diag(o.structural_report_diagnostic_ref) && o.failure === null && qc.solve_quality === o.outcome && qc.evidence_refs.includes(o.structural_report_diagnostic_ref) && diagnosticById.get(o.structural_report_diagnostic_ref)?.code === (o.outcome === 'checks_passed' ? 'NUMERICAL_INTEGRITY_CHECKS_PASSED' : 'NUMERICAL_INTEGRITY_SENSITIVE'), 'ORDINARY_REPORT');
    if (o.outcome === 'rejected') fail(o.failure && diag(o.failure.diagnostic_ref) && ['failed','unresolved'].includes(qc.solve_quality) && qc.evidence_refs.includes(o.failure.diagnostic_ref) && ['NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM','NUMERICAL_INTEGRITY_NEGATIVE_ENERGY','NUMERICAL_INTEGRITY_FAILED','NUMERICAL_INTEGRITY_UNRESOLVED','NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED'].includes(diagnosticById.get(o.failure.diagnostic_ref)!.code), 'ORDINARY_FAILURE');
    const work = c.work;
    const caseCharge = work.charged + work.reserved_unobserved_failure;
    chargedCases += caseCharge;
    fail(Number.isSafeInteger(caseCharge) && caseCharge <= work.limit && work.limit <= (composite ? 8_000_000 : 4_000_000) && chargedCases <= 64_000_000 && (work.rejected_reservation.kind === 'overflow' ? work.rejected_reservation.amount === null : work.rejected_reservation.amount !== null), 'WORK');
    const exact = c.selected_method === 'retained_source_blocks_exact_v1', ordinary = c.selected_method === `ordinary_${captured.solver_mode === 'dense_scrutiny' ? 'dense' : 'sparse'}_structural_v1`;
    if (c.outcome === 'qualified') {
      fail(c.failure === null && (exact || ordinary), 'SELECTED_METHOD');
      if (composite) fail(work.rejected_reservation.kind === 'finite' && work.rejected_reservation.amount === 0 && work.reserved_unobserved_failure === 0, 'QUALIFIED_WORK');
      if (ordinary) fail(c.source === null && !c.projections.length && !c.supports.length && o.outcome === 'checks_passed' && qc.structural_status === 'passive_model_basis' && qc.solve_quality === 'checks_passed' && qc.model_matrix_fidelity === 'represented_equations_retained' && ['not_claimed','reference_verified'].includes(qc.accuracy_evidence) && qc.evidence_refs.length && qc.evidence_refs.every(id => byId.has(id) || diag(id)), 'ORDINARY_NOT_QUALIFIED');
      if (exact) fail(c.source !== null, 'SOURCE_REQUIRED');
    } else {
      fail(c.selected_method === null && c.failure && diag(c.failure.diagnostic_ref), 'FAILURE');
      fail(c.failure.code === 'unsupported_block' ? c.source === null && c.failure.block_order > 2 : c.failure.block_order === null, 'FAILURE_BLOCK');
      findings.push('SOURCE_BLOCKS_CASE_UNQUALIFIED');
    }
    if (c.source) {
      const s = c.source, map = [...s.free_dofs,...s.prescribed_dofs], blockDofs = s.free_blocks.flat();
      fail(s.dof_count === model.nodes.length * 6 && s.dof_count <= 256 && s.stiffness_term_count + s.force_term_count <= 16384 && s.functional_count >= c.projections.length && s.functional_count <= 16384, 'SOURCE_COUNTS');
      fail(map.length === s.dof_count && unique(map) && map.every(n => n < s.dof_count) && unique(blockDofs) && blockDofs.length === s.free_dofs.length && blockDofs.every((n: number) => s.free_dofs.includes(n)), 'SOURCE_MAP');
      fail(s.member_ids.length > 0 && unique(s.member_ids) && same(s.member_ids, model.pipe_segments.map(m => m.id)) && unique(s.support_ids) && same(s.support_ids, model.supports.map(s => s.id)), 'SOURCE_ENTITIES');
    }
    fail(unique(c.projections.map((p: JsonObject) => p.projection_id)) && unique(c.projections.map((p: JsonObject) => p.functional_id)) && unique(c.projections.map((p: JsonObject) => p.result_id)), 'PROJECTION_IDS');
    const usedProjections = new Set<string>();
    for (const treatment of c.rows as JsonObject[]) {
      const row = account(treatment.result_id);
      fail(row.basis_ref?.ref_type === 'load_case' && row.basis_ref.ref_id === requested[index], 'ROW_CASE');
      fail(unique(treatment.input_result_ids) && treatment.input_result_ids.every((id: string) => byId.get(id)?.basis_ref?.ref_type === 'load_case' && byId.get(id)?.basis_ref?.ref_id === requested[index] && id !== row.id), 'ROW_INPUTS');
      if (treatment.treatment === 'qualified_projection') {
        const p = c.projections.find((p: JsonObject) => p.projection_id === treatment.projection_id);
        fail(exact && p && p.result_id === row.id && !usedProjections.has(p.projection_id) && treatment.recipe_id === null && !treatment.input_result_ids.length, 'PROJECTION_BINDING');
        usedProjections.add(p.projection_id);
        fail(Object.is(p.value,row.value) && p.value_bits === bits(row.value) && p.unit === row.unit && semantic(row)?.category === 'physical_quantity', 'PROJECTION_VALUE');
        validateProjectionSemantics(p, row, model);
        fail(p.interval[0] <= p.value && p.value <= p.interval[1] && (p.value === 0 || p.interval[0] > 0 || p.interval[1] < 0), 'PROJECTION_INTERVAL');
        fail(p.value !== 0 || p.basis !== 'outward_interval', 'ZERO_PROJECTION');
        if (p.basis === 'exact_zero' || p.basis === 'exact_identity') fail(p.interval[0] === p.value && p.interval[1] === p.value && p.absolute_error_bound === 0 && p.relative_error_bound === 0 && (p.basis !== 'exact_zero' || p.value === 0), 'EXACT_PROJECTION');
        if (composite) {
          fail(p.basis !== 'exact_identity' || p.value !== 0, 'EXACT_IDENTITY');
          if (p.basis === 'outward_interval') fail(p.absolute_error_bound >= Math.max(Math.abs(p.value - p.interval[0]), Math.abs(p.interval[1] - p.value))
            && p.relative_error_bound >= p.absolute_error_bound / Math.min(Math.abs(p.interval[0]), Math.abs(p.interval[1])), 'PROJECTION_ERROR_BOUND');
        }
      } else if (treatment.treatment === (composite ? 'ordinary_physics_checked' : 'ordinary_checked')) {
        fail(ordinary && c.outcome === 'qualified' && semantic(row) !== undefined && (composite || row.kind !== 'support_reaction_component_v2') && treatment.projection_id === null && treatment.recipe_id === null, 'ORDINARY_ROW');
      } else if (treatment.treatment === 'checked_derived') {
        fail(exact && treatment.projection_id === null && treatment.recipe_id !== null, 'DERIVED_ROW');
        // Concrete recipe/dependency checks run after the complete row ledger.
        // A recipe label alone never qualifies a value.
      } else {
        fail(treatment.projection_id === null && treatment.recipe_id === null, 'INSPECTION_ROW');
        if (physical(row)) findings.push('SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY');
      }
    }
    fail(usedProjections.size === c.projections.length, 'UNACCOUNTED_PROJECTION');
    if (exact && c.outcome === 'qualified') { checkExpectedPhysicalRows(c, model, byId, composite); await checkDerivedRows(c, model, byId, composite ? raw : undefined); if (composite) await validatePhysicsSourceCase(raw, c, captured); }
    if (exact && c.outcome === 'qualified') {
      fail(c.supports.length === model.supports.length && unique(c.supports.map((s: JsonObject) => s.support_id)), 'SUPPORT_COVERAGE');
      for (const support of c.supports as JsonObject[]) {
        const authored = model.supports.find(s => s.id === support.support_id), nodeIndex = model.nodes.findIndex(n => n.id === support.node_id);
        fail(authored?.node === support.node_id && nodeIndex >= 0 && unique(support.components.map((p: JsonObject) => p.component)), 'SUPPORT_IDENTITY');
        for (const part of support.components as JsonObject[]) {
          const offset = ['Fx','Fy','Fz','Mx','My','Mz'].indexOf(part.component), row = byId.get(part.result_id), p = c.projections.find((p: JsonObject) => p.result_id === part.result_id);
          fail(row && row.metadata && row.kind === 'support_reaction_component_v2' && row.entity_ref === support.support_id && row.metadata?.component === part.component && row.metadata.coordinate_system === 'global' && row.metadata.location === 'node' && row.metadata.basis === 'recovered_from_assembled_support_law' && row.metadata.sign_convention === (composite ? PHYSICS_SOURCE_SUPPORT_SIGN : 'support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node') && row.unit === (offset < 3 ? 'N' : 'N*m') && p?.functional_id === part.functional_id && p.quantity === 'support_action_component', 'SUPPORT_COMPONENT');
          fail(part.action_terms.length && part.action_terms.every((t: JsonObject) => t.global_dof === nodeIndex * 6 + offset && t.source_id === support.support_id), 'SUPPORT_OWNERSHIP');
        }
      }
    } else fail(!c.supports.length, 'UNQUALIFIED_SUPPORT_CERTIFICATE');
  }
  for (const id of body.envelope_observation_result_ids) { const row = account(id); fail(!row.basis_ref && !physical(row), 'ENVELOPE_OBSERVATION'); }
  fail(accounted.size === raw.results.length, 'ROW_COVERAGE');
  const invocationWork = body.invocation_work;
  fail(invocationWork.limit <= 64_000_000 && invocationWork.charged <= invocationWork.limit && chargedCases + invocationWork.publication_charged === invocationWork.charged, 'INVOCATION_WORK');
  const count = cases.filter(c => c.outcome === 'qualified').length;
  fail(body.status === (count === cases.length ? 'qualified' : count ? 'partial' : 'unavailable'), 'AGGREGATE');
  if (body.status === 'qualified') fail(cases.some(c => c.selected_method === 'retained_source_blocks_exact_v1'), 'NO_SOURCE_METHOD');
  if (body.status !== 'qualified' || model.combinations?.length) findings.push('SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED');
  if (raw.status.mechanics !== 'MECHANICS_SOLVED') findings.push('MECHANICS_NOT_SOLVED');
  for (const [summary, kind] of [[raw.summary.max_displacement, 'displacement_magnitude'], [raw.summary.max_open_formula_stress, composite ? 'pipe_elastic_normal_stress_maximum_v2' : 'open_formula_stress_summary']] as const) {
    const candidates = raw.results.filter(r => r.kind === kind);
    if (body.status === 'qualified' && candidates.length) fail(summary, 'SUMMARY_REQUIRED');
    if (summary) {
      const row = byId.get(summary.result_ref);
      fail(row && row.kind === kind && row.value === summary.value && row.unit === summary.unit && row.entity_ref === summary.location_ref, 'SUMMARY_BINDING');
      if (body.status === 'qualified') fail(candidates.every(r => r.value <= summary.value), 'SUMMARY_MAXIMUM');
    }
  }
  fail(checkedJsonText(source) === sourceText && negativeZeroPaths(source) === sourceZeroSigns && checkedJsonText(invocation) === invocationText && negativeZeroPaths(invocation) === invocationZeroSigns && (!callerModel || (!!checkedJsonText(callerModel) && sameModelData(callerModel,callerSnapshot))), 'CHANGED_DURING_VALIDATION');
  // Receipt fields are immutable producer evidence. Seal only after complete
  // validation and race checks; this avoids quadratic schema work when each
  // semantic-row lookup dispatches the same authenticated receipt.
  sealValidatedReceipt(source.source_block_recovery);
  fail(checkedJsonText(source) === sourceText && negativeZeroPaths(source) === sourceZeroSigns && checkedJsonText(invocation) === invocationText && negativeZeroPaths(invocation) === invocationZeroSigns && (!callerModel || (!!checkedJsonText(callerModel) && sameModelData(callerModel,callerSnapshot))), 'CHANGED_DURING_VALIDATION');
  immutableReceiptShapes.add(source.source_block_recovery as object);
  const answer = { eligible: findings.length === 0, findings: [...new Set(findings)] };
  validated.set(source, { ...answer, method, sourceText, sourceZeroSigns, invocationText, invocationZeroSigns, invocation, callerModel, callerSnapshot });
  return answer;
}

/** No token is serialized or copied to a cloned/imported historical carrier. */
export function sourceBlockStanding(source: MechanicsResult, model?: Pick<PreviewModel, 'load_cases'> | null): Validation {
  return knownSourceStanding(source, model, SOURCE_BLOCKS_CONTRACT_ID);
}
export function physicsSourceReceiptStanding(source: MechanicsResult, model?: Pick<PreviewModel, 'load_cases'> | null): Validation {
  return knownSourceStanding(source, model, 'openpipestress.result_semantics/0.3.0/physics-source-1');
}
function knownSourceStanding(source: MechanicsResult, model: Pick<PreviewModel, 'load_cases'> | null | undefined, method: string): Validation {
  const token = validated.get(source);
  try {
    if (!token || token.method !== method || checkedJsonText(source) !== token.sourceText || negativeZeroPaths(source) !== token.sourceZeroSigns || checkedJsonText(token.invocation) !== token.invocationText || negativeZeroPaths(token.invocation) !== token.invocationZeroSigns || !model || !checkedJsonText(model)
      || (token.callerModel && (!checkedJsonText(token.callerModel) || !sameModelData(token.callerModel,token.callerSnapshot)))
      || !(sameModelData(model,(token.invocation.request as {model:PreviewModel}).model) || (token.callerSnapshot && sameModelData(model,token.callerSnapshot)))) return { eligible: false, findings: ['SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED'] };
    return { eligible: token.eligible, findings: [...token.findings] };
  } catch { return { eligible: false, findings: ['SOURCE_BLOCKS_VALIDATED_SOURCE_CHANGED'] }; }
}


function validateProjectionSemantics(p: JsonObject, row: MechanicsResult['results'][number], model: PreviewModel) {
  const m = row.metadata;
  fail(m && ['component','coordinate_system','location','basis','sign_convention'].every(k => typeof (m as JsonObject)[k] === 'string' && (m as JsonObject)[k].length > 0), 'PROJECTION_METADATA');
  const md = m!;
  if (p.quantity === 'nodal_translation' || p.quantity === 'nodal_rotation') {
    const rotation = p.quantity === 'nodal_rotation';
    fail(model.nodes.some(n => n.id === row.entity_ref) && md.coordinate_system === 'global' && md.location === 'node'
      && md.basis === 'solved_from_global_linear_system' && row.unit === (rotation ? 'rad' : 'mm')
      && ['x','y','z'].some(axis => row.kind === `global_nodal_${rotation ? 'rotation' : 'displacement'}_${axis}` && md.component === `nodal_${rotation ? 'rotation' : 'displacement'}_${axis}`), 'NODAL_PROJECTION');
  } else if (p.quantity === 'member_end_action' || p.quantity === 'member_station_action') {
    const end = p.quantity === 'member_end_action';
    const components = ['axial_force','shear_force_y','shear_force_z','torsional_moment','bending_moment_y','bending_moment_z'];
    const component = components.indexOf(md.component);
    fail(model.pipe_segments.some(m => m.id === row.entity_ref) && md.coordinate_system === 'element_local' && component >= 0
      && row.kind === `element_local_${md.component}` && row.unit === (component < 3 ? 'N' : 'N*m')
      && (end ? ['end_i','end_j'].includes(md.location) : ['quarter_1','midspan','quarter_3'].includes(md.location)), 'MEMBER_PROJECTION');
  } else fail(p.quantity === 'support_action_component' && row.kind === 'support_reaction_component_v2', 'UNIMPLEMENTED_PROJECTION');
}

/** These finite binary64 recipes are separate from exact affine projection.
 * Recompute from the authenticated invocation and same-case projected rows;
 * neither a digest nor the name of a recipe establishes these relationships. */
async function checkDerivedRows(c: JsonObject, model: PreviewModel, rows: Map<string, MechanicsResult['results'][number]>, compositeSource?: MechanicsResult) {
  const treatments = new Map<string, JsonObject>(c.rows.map((t: JsonObject) => [t.result_id, t]));
  if ((c.rows as JsonObject[]).some(t => t.treatment === 'checked_derived' && ['straight_open_stress_v1','reviewed_stress_summary_v1', ...(compositeSource ? ['retained_source_straight_stress_v1'] : [])].includes(t.recipe_id))) {
    const recipeBound = 128*Number.EPSILON;
    fail(c.projections.length > 0 && Math.max(...c.projections.map((p: JsonObject) => p.relative_error_bound)) <= (1e-9-recipeBound)/(1+recipeBound), 'STRESS_ERROR_BOUND');
  }
  const stressPa = new Map<string, number>();
  const sections = new Map<string, { area: number; z: number; j: number; r: number }>();
  async function section(id: string) {
    if (sections.has(id)) return sections.get(id)!;
    const member = model.pipe_segments.find(m => m.id === id);
    fail(member, 'DERIVED_MEMBER');
    const input = member!.section as JsonObject;
    const fields = ['outside_diameter','wall_thickness', ...(input.mill_tolerance ? ['mill_tolerance'] : [])];
    const converted = JSON.parse((await loadWasmEngine()).convertDisplayQuantitiesJson(checkedJsonText({ items: fields.map(name => ({ id:name,value:input[name].value,from_unit:input[name].unit,to_unit:'m',dimension_id:'length' })) })));
    fail(!converted.error && converted.items?.length === fields.length && converted.items.every((x: JsonObject, i: number) => x.id === fields[i] && x.status === 'converted' && x.unit === 'm' && Number.isFinite(x.value)), 'DERIVED_GEOMETRY_UNITS');
    const D = converted.items[0].value, t = converted.items[1].value - (fields.length === 3 ? converted.items[2].value : 0), inner = D - 2*t;
    fail(D > 0 && t > 0 && 2*t < D, 'DERIVED_GEOMETRY');
    // Match this source method's selected section formation. Physics-1's
    // authored-OD/effective-wall implementation is a separate reviewed join.
    const D2 = D*D, inner2 = inner*inner;
    const area = Math.PI*(D2-inner2)/4, I = Math.PI*(D2*D2-inner2*inner2)/64;
    const result = { area, z:I/(D/2), j:2*I, r:D/2 };
    fail(Object.values(result).every(v => Number.isFinite(v) && v > 0), 'DERIVED_GEOMETRY_RANGE');
    sections.set(id,result); return result;
  }
  const projected = (id: string) => {
    fail(treatments.get(id)?.treatment === 'qualified_projection', 'DERIVED_PROJECTED_INPUT');
    const row = rows.get(id); fail(row, 'DERIVED_INPUT'); return row!;
  };
  async function stress(id: string): Promise<number> {
    if (stressPa.has(id)) return stressPa.get(id)!;
    const row = rows.get(id)!, treatment = treatments.get(id);
    fail(row && treatment?.treatment === 'checked_derived' && treatment.recipe_id === 'straight_open_stress_v1' && treatment.input_result_ids.length === 1, 'STRESS_RECIPE');
    const force = projected(treatment!.input_result_ids[0]), md = row.metadata;
    const mapping: Record<string,string> = { axial_normal_stress:'axial_force', bending_normal_stress_y:'bending_moment_y', bending_normal_stress_z:'bending_moment_z', torsional_shear_stress:'torsional_moment' };
    fail(md && Object.hasOwn(mapping,md.component) && row.kind === `element_local_${md.component}` && row.unit === 'MPa'
      && force.entity_ref === row.entity_ref && force.metadata?.component === mapping[md.component] && force.metadata.location === md.location
      && md.coordinate_system === 'element_local' && md.basis === (['end_i','end_j'].includes(md.location) ? 'recovered_from_local_element_stiffness' : 'recovered_from_open_mechanics_stress_components'), 'STRESS_INPUT_BINDING');
    const sec = await section(row.entity_ref), action = force.value * (md!.location === 'end_i' ? -1 : 1);
    const torsion = md!.component === 'torsional_shear_stress';
    const numerator = torsion ? action*sec.r : action;
    if (torsion) fail(action === 0 ? numerator === 0 : normalFinite(numerator), 'STRESS_TORSION_PRODUCT_RANGE');
    const pa = numerator/(md!.component === 'axial_normal_stress' ? sec.area : torsion ? sec.j : sec.z);
    const mpa = pa/1e6;
    fail(action === 0 ? mpa === 0 : normalFinite(mpa), 'STRESS_MPA_RANGE');
    fail(action === 0 ? pa === 0 : normalFinite(pa), 'STRESS_PA_RANGE');
    fail(Object.is(row.value,mpa), `STRESS_VALUE:${row.id}:${bits(row.value)}:${bits(mpa)}`);
    stressPa.set(id,pa);return pa;
  }
  for (const treatment of c.rows as JsonObject[]) {
    if (treatment.treatment !== 'checked_derived') continue;
    const row = rows.get(treatment.result_id)!;
    if (compositeSource && treatment.recipe_id !== 'translation_norm_scaled_v1') { validatePhysicsSourceDerived(compositeSource, c, treatment, row, treatment.input_result_ids.map(projected)); continue; }
    if (treatment.recipe_id === 'translation_norm_scaled_v1' || treatment.recipe_id === 'support_force_norm_scaled_v1') {
      const translation = treatment.recipe_id === 'translation_norm_scaled_v1', inputs = treatment.input_result_ids.map(projected);
      const components = translation ? ['nodal_displacement_x','nodal_displacement_y','nodal_displacement_z'] : ['Fx','Fy','Fz'];
      fail(inputs.length === 3 && inputs.every((x: MechanicsResult['results'][number], i: number) => x.entity_ref === row.entity_ref && x.metadata?.component === components[i]
        && x.kind === (translation ? `global_${components[i]}` : 'support_reaction_component_v2') && x.unit === (translation ? 'mm' : 'N'))
        && row.kind === (translation ? 'displacement_magnitude' : 'reaction_resultant') && row.unit === (translation ? 'mm' : 'N'), 'NORM_INPUT_BINDING');
      const inputBound = Math.max(...treatment.input_result_ids.map((id: string) => c.projections.find((p: JsonObject) => p.result_id === id).relative_error_bound));
      const recipeBound = 64*Number.EPSILON;
      fail(inputBound <= (1e-9-recipeBound)/(1+recipeBound), 'NORM_ERROR_BOUND');
      const v = inputs.map((r: MechanicsResult['results'][number]) => r.value);
      fail(v.every(Number.isFinite), 'NORM_VALUE');
      const m = Math.max(Math.abs(v[0]),Math.abs(v[1]),Math.abs(v[2]));
      const nx = m === 0 ? 0 : v[0]/m, ny = m === 0 ? 0 : v[1]/m, nz = m === 0 ? 0 : v[2]/m;
      const sum = (nx*nx+ny*ny)+nz*nz;
      const value = m === 0 ? 0 : m*Math.sqrt(sum);
      fail([m,nx,ny,nz,sum,value].every(Number.isFinite) && (m === 0 || value >= 2 ** -1022) && Object.is(row.value,value), 'NORM_VALUE');
    } else if (treatment.recipe_id === 'straight_open_stress_v1') await stress(row.id);
    else if (treatment.recipe_id === 'reviewed_stress_summary_v1') {
      fail(row.kind === 'open_formula_stress_summary' && row.unit === 'MPa' && treatment.input_result_ids.length === 20, 'STRESS_SUMMARY_INPUTS');
      const componentNames = ['axial_normal_stress','bending_normal_stress_y','bending_normal_stress_z','torsional_shear_stress'];
      let expected = 0;
      for (const location of ['end_i','quarter_1','midspan','quarter_3','end_j']) {
        const values: number[] = [];
        for (const component of componentNames) {
          const matching = treatment.input_result_ids.filter((id: string) => { const input = rows.get(id);return input?.entity_ref === row.entity_ref && input.metadata?.location === location && input.metadata.component === component; });
          fail(matching.length === 1, 'STRESS_SUMMARY_COVERAGE'); values.push(await stress(matching[0]));
        }
        const axial = values[0]+0, bending = Math.abs(values[1])+Math.abs(values[2]);
        const positive = axial+bending, negative = axial-bending;
        const maximumPa = Math.max(Math.abs(positive),Math.abs(negative)), maximumMpa = maximumPa/1e6;
        const zero = values[0] === 0 && values[1] === 0 && values[2] === 0;
        fail([axial,bending,positive,negative].every(Number.isFinite) && (zero ? maximumPa === 0 && maximumMpa === 0 : normalFinite(maximumPa) && normalFinite(maximumMpa)), 'STRESS_SUMMARY_RANGE');
        expected = Math.max(expected, maximumMpa);
      }
      fail(Number.isFinite(expected) && Object.is(row.value,expected), 'STRESS_SUMMARY_VALUE');
    } else fail(false, 'DERIVED_RECIPE_NOT_IMPLEMENTED');
  }
}

/** Return a fresh copy only from a still-valid live registration. Serialized
 * invocation is binding input for the backend, never a replacement for origin. */
export function retainedSourceBlockInvocation(source: MechanicsResult, model: PreviewModel): SourceBlockInvocation | null {
  if (!sourceBlockStanding(source,model).eligible) return null;
  return structuredClone(validated.get(source)!.invocation);
}

function checkExpectedPhysicalRows(c: JsonObject, model: PreviewModel, byId: Map<string, MechanicsResult['results'][number]>, composite = false) {
  const actual = c.rows.map((t: JsonObject) => byId.get(t.result_id)!);
  const required = (entity: string, kind: string, component?: string, location?: string) => fail(actual.filter((r: MechanicsResult['results'][number]) => r.entity_ref === entity && r.kind === kind && (component === undefined || r.metadata?.component === component) && (location === undefined || r.metadata?.location === location)).length === 1, 'REQUIRED_PHYSICAL_ROW_COVERAGE');
  for (const node of model.nodes) {
    for (const family of ['displacement','rotation']) for (const axis of ['x','y','z']) required(node.id, `global_nodal_${family}_${axis}`, `nodal_${family}_${axis}`, 'node');
    required(node.id,'displacement_magnitude');
  }
  for (const member of model.pipe_segments) {
    for (const location of ['end_i','quarter_1','midspan','quarter_3','end_j']) {
      for (const component of ['axial_force','shear_force_y','shear_force_z','torsional_moment','bending_moment_y','bending_moment_z']) required(member.id,`element_local_${component}`,component,location);
      for (const component of ['axial_normal_stress','bending_normal_stress_y','bending_normal_stress_z','torsional_shear_stress']) required(member.id,`element_local_${component}`,component,location);
    }
    required(member.id,composite ? 'pipe_elastic_normal_stress_maximum_v2' : 'open_formula_stress_summary');
  }
  for (const support of model.supports) {
    for (const component of ['Fx','Fy','Fz','Mx','My','Mz']) required(support.id,'support_reaction_component_v2',component,'node');
    required(support.id,composite ? 'support_reaction_force_magnitude_v2' : 'reaction_resultant');
    if (composite) required(support.id,'support_reaction_moment_magnitude_v2');
  }
}
export function sourceBlockModeMatches(source: MechanicsResult, model: PreviewModel, solverMode: string): boolean {
  const context = retainedSourceBlockInvocation(source, model);
  return context !== null && context.solver_mode === solverMode;
}

// JCS intentionally serializes -0 as 0. Projection value_bits does not: retain
// the signed-zero positions separately so the private registration cannot be
// reused after a bit-changing mutation that leaves canonical JSON unchanged.
function negativeZeroPaths(value: unknown): string {
  const paths: string[] = [];
  const visit = (item: unknown, path: string[]) => {
    if (Object.is(item,-0)) paths.push(JSON.stringify(path));
    else if (item && typeof item === 'object') for (const [key, child] of Object.entries(item)) visit(child,[...path,key]);
  };
  visit(value,[]); return JSON.stringify(paths);
}

function sameModelData(a: any, b: any): boolean {
  if (Object.is(a,b)) return true;
  if (!a || !b || typeof a !== 'object' || typeof b !== 'object' || Array.isArray(a)!==Array.isArray(b)) return false;
  const keys=Object.keys(a);return keys.length===Object.keys(b).length && keys.every(k=>Object.hasOwn(b,k)&&sameModelData(a[k],b[k]));
}

function normalFinite(value: number): boolean { return Number.isFinite(value) && Math.abs(value) >= 2 ** -1022; }

export function retainedPhysicsSourceReceiptInvocation(source: MechanicsResult, model: PreviewModel): SourceBlockInvocation | null {
  if (!physicsSourceReceiptStanding(source, model).eligible) return null;
  return structuredClone(validated.get(source)!.invocation);
}
