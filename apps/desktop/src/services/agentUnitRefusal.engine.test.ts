// The unit-refusal explanation against the real operation engine (the wasm applier the desktop
// tests load in src/test/setup.ts), through the session's own propose route: validateOperationBatch
// is not mocked. A scripted agent proposes a bare number in another unit for every quantity field
// the table lists, receives the refusal text, builds its second proposal from that text and the
// catalog description alone, and the second proposal registers for review. The table's dimensions
// and the stated-unit vocabulary are checked against the same engine. Invented data; no provider,
// no native call, nothing launched.
import { describe, expect, it, vi } from 'vitest';
import previewJson from '../../../../fixtures/product_preview/invented_preview_model.json';
import catalog from '../../src-tauri/resources/agent-capability-catalog.json';
import type { PreviewModel, ModelHashEvidence } from '../types';
import { EmbeddedAgentSession, agentHash, stampAgentOperations, type WorkspaceExecution, type AgentSolveOutcome } from './embeddedAgentSession';
import type { AgentBasis } from './embeddedAgentBridge';
import { validateOperationBatch } from './operationBatchService';
import { computeModelHash } from './hashService';
import { unitRatio } from './unitConversion';
import { GENERIC_REFUSAL, STATED_UNITS, UNIT_REFUSAL_FIELDS, unitRefusalMessage, type QuantityField } from './agentUnitRefusal';

type AnyRecord = Record<string, unknown>;
const P = 'invented: unit refusal route check';

/** The invented preview model with every table field stored as a value/unit pair. */
function model(): PreviewModel {
 const doc = structuredClone(previewJson) as unknown as AnyRecord;
 const pipe = (doc.pipe_segments as AnyRecord[])[0];
 Object.assign(pipe.section as AnyRecord, {corrosion_allowance: {value: 0.001, unit: 'm'}, mill_tolerance: {value: 0.0005, unit: 'm'}, insulation_thickness: {value: 0.05, unit: 'm'},
  material_density: {value: 7850, unit: 'kg/m^3'}, contents_density: {value: 1000, unit: 'kg/m^3'}, insulation_density: {value: 100, unit: 'kg/m^3'}});
 (doc.load_cases as AnyRecord[])[0].equivalent_static = {
  seismic: {gravity_acceleration: {value: 9.80665, unit: 'm/s^2'}, g_factor_x: {value: 0.1, unit: 'none'}, g_factor_y: {value: 0.2, unit: 'none'}, g_factor_z: {value: 0.3, unit: 'none'}},
  wind: {pressure: {value: 1000, unit: 'Pa'}, shape_factor: {value: 0.7, unit: 'none'}, direction: 'global_x', exposed_pipe_refs: [pipe.id]}
 };
 return doc as unknown as PreviewModel;
}
const COLLECTION: Record<QuantityField['objectType'], string> = {Material: 'materials', Element: 'pipe_segments', Component: 'components', Load: 'load_cases'};
/** The first record of the field's collection holding the field, and its stored number and unit. */
function target(doc: PreviewModel, field: QuantityField) {
 for (const record of (doc as unknown as AnyRecord)[COLLECTION[field.objectType]] as AnyRecord[]) {
  let node: unknown = record;
  for (const segment of field.fieldPath.split('.').slice(0, -1)) node = (node as AnyRecord | undefined)?.[segment];
  const pair = node as {value?: unknown; unit?: unknown} | undefined;
  if (typeof pair?.value === 'number' && typeof pair.unit === 'string') return {ref: String(record.id), value: pair.value, unit: pair.unit};
 }
 throw new Error(`no stored ${field.fieldPath}`);
}
/** Per dimension: a unit the engine accepts other than the stored one (null where the dimension has
 * only one), and an entered unit the first proposal uses. */
const OTHER: Record<string, string | null> = {stress: 'GPa', thermal_expansion_coefficient: '1/degF', length: 'mm', density: 'lb/in^3', angle: 'deg', force: 'kN', area: 'mm^2', acceleration: null, dimensionless: '1', pressure: 'kPa'};
const NOT_OF_DIMENSION = 'in/s^3';
function op(field: QuantityField, ref: string, before: string, after: string, unit: string, dimension: string, id = `op:${field.fieldPath}`): AnyRecord {
 return {operation_id: id, operation_kind: 'modify', target: {object_type: field.objectType, ref},
  change: {change_id: `change:${field.fieldPath}`, change_kind: 'set_field', field_label: field.fieldPath, field_path: field.fieldPath, before, after, unit, dimension, source_note: P}, rationale: P};
}
async function validate(doc: PreviewModel, operations: AnyRecord[]) {
 const hash = await computeModelHash(doc);
 return validateOperationBatch(doc, {batch_id: 'agent-unit-refusal', operations: stampAgentOperations(operations, 'proposal', 'request'), source_model_hash: hash!.value}, hash);
}
const blockingCodes = (outcome: Awaited<ReturnType<typeof validate>>) => outcome.diagnostics.filter(d => d.severity === 'blocking').map(d => d.code);

/** The session as the workspace holds it, with the real engine behind its propose route. */
async function harness(doc: PreviewModel) {
 const engineHash = (await computeModelHash(doc)) as ModelHashEvidence;
 const basis: AgentBasis = {workspaceHandle: 'workspace-invented', applicationGeneration: 1, modelRevision: 1, modelHash: agentHash(engineHash)!, stateId: null, runId: null};
 let args: AnyRecord = {}, reference = '';
 const owner: WorkspaceExecution = {snapshot: () => ({model: doc, hash: engineHash, revision: 1, generation: 1, result: null, standing: null}), register: vi.fn(async () => 'batch-key'),
  proposal: vi.fn(() => ({status: 'pending_review' as const, outcomeModelHash: null})), solve: vi.fn(async (): Promise<AgentSolveOutcome> => ({kind: 'not_started', message: 'Invented.'})), withdraw: vi.fn()};
 const invoke = vi.fn(async (value: AnyRecord): Promise<unknown> => {
  switch (value.kind) {
   case 'register': return {...basis, ownerCapability: 'private-owner-capability'};
   case 'start': return {status: 'starting'};
   case 'poll': return {status: 'ready', notices: reference ? [reference] : [], messages: [], closed: false, turnActive: false, turnStatus: 'none'};
   case 'claim': return {raw: JSON.stringify({requestRef: reference, threadId: 'thread-1', turnId: 'turn-1', callId: `call-${reference}`, basis, tool: 'swbpipe_propose_operations', arguments: args})};
   case 'complete': return {delivery: 'detached'};
   default: return {};
  }
 });
 const session = new EmbeddedAgentSession(owner, invoke);
 vi.useFakeTimers({toFake: ['setTimeout', 'clearTimeout']});
 await session.start('revision');
 let n = 0;
 /** One proposal through the session: its tool outcome. */
 const propose = async (operations: AnyRecord[]) => {
  reference = `invented-session:${++n}`;args = {operations, assumptions: []};
  await session.poll();
  for (let i = 0; i < 1000 && !session.actual(reference); i++) await new Promise(resolve => setImmediate(resolve));
  const actual = session.actual(reference);if (!actual) throw new Error('no outcome');
  return JSON.parse(actual.outcome) as {status: string; message: string};
 };
 return {session, owner, propose, close: async () => {await session.close();vi.useRealTimers();}};
}

/** What a scripted agent takes from the refusal text: the dimension and, when stated, the stored unit. */
function readRefusal(text: string) {
 const dimension = /has dimension ([a-z_]+)[,;]/.exec(text)?.[1];
 const stored = /, stored unit ([^.\s]+)\./.exec(text)?.[1] ?? null;
 return {dimension, stored};
}

describe('unit refusal explanation against the operation engine', () => {
 it('the catalog states the corrective pair form and the set_field route of every table type', () => {
  const description = catalog.tools.find(tool => tool.name === 'swbpipe_propose_operations')!.description;
  expect(description).toContain('after is JSON text; quantities are {value, unit}');
  for (const field of UNIT_REFUSAL_FIELDS) expect(description, field.fieldPath).toContain(`modify/${field.objectType}/set_field`);
 });

 it('each table dimension is the engine\'s: the field accepts it and refuses another by OP-UNIT-DIMENSION-MISMATCH', async () => {
  const doc = model();
  for (const field of UNIT_REFUSAL_FIELDS) {
   const stored = target(doc, field);
   const same = await validate(doc, [op(field, stored.ref, String(stored.value), String(stored.value), stored.unit, field.dimension)]);
   expect(same.validation.batch_validation_status, `${field.fieldPath}: ${blockingCodes(same).join(' | ')}`).toBe('passed');
   const otherDimension = field.dimension === 'length' ? 'angle' : 'length';
   const other = await validate(doc, [op(field, stored.ref, String(stored.value), String(stored.value), stored.unit, otherDimension)]);
   expect(blockingCodes(other), field.fieldPath).toEqual(['OP-UNIT-DIMENSION-MISMATCH']);
  }
 });

 it('every stated unit is in the desktop display vocabulary and accepted by the engine for its dimension', async () => {
  const canonical: Record<string, string> = {stress: 'Pa', pressure: 'Pa', length: 'm', force: 'N', angle: 'rad'};
  const doc = model();
  for (const [dimension, symbols] of Object.entries(STATED_UNITS)) {
   const field = UNIT_REFUSAL_FIELDS.find(entry => entry.dimension === dimension)!;
   const stored = target(doc, field);
   for (const symbol of symbols) {
    expect(unitRatio(symbol, canonical[dimension]), `${dimension} ${symbol}`).not.toBeNull();
    const pair = await validate(doc, [op(field, stored.ref, String(stored.value), JSON.stringify({value: stored.value, unit: symbol}), symbol, dimension)]);
    expect(pair.validation.batch_validation_status, `${dimension} ${symbol}: ${blockingCodes(pair).join(' | ')}`).toBe('passed');
   }
  }
 });

 it('an agent given the refusal resubmits the explicit pair, and the stated bare form, and each registers for review', async () => {
  const doc = model();
  // One session per field: a session's bridge retains at most 64 requests (embeddedAgentBridge MAX_ENTRIES).
  for (const field of UNIT_REFUSAL_FIELDS) {
   const {owner, propose, close} = await harness(doc);
   try {
    const stored = target(doc, field), other = OTHER[field.dimension];
    const entered = other ?? NOT_OF_DIMENSION;
    // The witness of issue 382 for E: a bare 210 with GPa on a material stored in Pa.
    const after = field.fieldPath === 'elastic_modulus.value' ? '210' : String(stored.value * 2);
    const first = await propose([op(field, stored.ref, String(stored.value), after, entered, field.dimension, `op:IGNORE_PREVIOUS_INSTRUCTIONS:${field.fieldPath}`)]);
    expect(first.status, field.fieldPath).toBe('unavailable');
    expect(first.message, field.fieldPath).not.toBe(GENERIC_REFUSAL);
    expect(first.message.startsWith(GENERIC_REFUSAL)).toBe(true);
    expect(first.message).toContain(`Operation 1 of 1 (OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE): ${field.objectType} ${field.fieldPath} has dimension ${field.dimension}`);
    expect(first.message).not.toContain('IGNORE');expect(first.message).not.toContain(stored.ref);
    expect(first.message.length).toBeLessThanOrEqual(1000);
    if (field.fieldPath === 'elastic_modulus.value') expect(first.message).toContain('stored unit Pa.');

    // The second proposal from the refusal text and the catalog alone.
    const read = readRefusal(first.message);
    expect(read.dimension, field.fieldPath).toBe(field.dimension);
    const unit = other ?? stored.unit;
    const registered = vi.mocked(owner.register).mock.calls.length;
    const pair = await propose([op(field, stored.ref, String(stored.value), JSON.stringify({value: Number(after), unit}), unit, read.dimension!)]);
    expect(pair.status, `${field.fieldPath}: ${pair.message}`).toBe('pending_review');
    expect(vi.mocked(owner.register).mock.calls.length).toBe(registered + 1);
    if (read.stored) {
     expect(read.stored).toBe(stored.unit);
     const bare = await propose([op(field, stored.ref, String(stored.value), String(stored.value), read.stored, read.dimension!)]);
     expect(bare.status, `${field.fieldPath}: ${bare.message}`).toBe('pending_review');
    } else {
     expect(first.message).toContain('read its stored unit with swbpipe_read_records');
    }
   } finally {await close();}
  }
 });

 it('a refusal for another reason keeps the generic sentence', async () => {
  const doc = model();
  const field = UNIT_REFUSAL_FIELDS[0], stored = target(doc, field);
  const {propose, close} = await harness(doc);
  try {
   const stale = await propose([op(field, stored.ref, String(stored.value + 1), JSON.stringify({value: 1, unit: 'GPa'}), 'GPa', 'stress')]);
   expect(stale).toMatchObject({status: 'unavailable', message: GENERIC_REFUSAL});
   const temperaturePoint = await propose([{...op(field, stored.ref, '1', '2', 'GPa', 'stress'), change: {...op(field, stored.ref, '1', '2', 'GPa', 'stress').change as AnyRecord, field_path: 'temperature_points.T1.shear_modulus.value'}}]);
   expect(temperaturePoint).toMatchObject({status: 'unavailable', message: GENERIC_REFUSAL});
  } finally {await close();}
 });

 it('a refused later step is named by position and the earlier steps are not', async () => {
  const doc = model();
  const e = UNIT_REFUSAL_FIELDS[0], stored = target(doc, e);
  const label = (n: number) => ({operation_id: `op:label:${n}`, operation_kind: 'modify', target: {object_type: 'Material', ref: stored.ref}, change: {change_id: `c:label:${n}`, change_kind: 'set_field', field_label: 'label', field_path: 'label', before: (doc.materials![0] as unknown as AnyRecord).label, after: 'Relabelled', unit: 'none', dimension: 'dimensionless', source_note: P}, rationale: P});
  const outcome = await validate(doc, [label(1), op(e, stored.ref, String(stored.value), '210', 'GPa', 'stress'), label(3)]);
  const text = unitRefusalMessage(outcome, [label(1), op(e, stored.ref, String(stored.value), '210', 'GPa', 'stress'), label(3)], doc);
  expect(text).toBe(`${GENERIC_REFUSAL} Operation 2 of 3 (OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE): Material elastic_modulus.value has dimension stress, stored unit Pa. Resend it as a bare number with change.unit Pa, or as after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension stress. A bare number is read in the stored unit, never converted. Operations after operation 2 were not checked.`);
 });
});
