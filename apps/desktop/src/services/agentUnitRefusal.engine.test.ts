// The unit-refusal explanation against the real operation engine (the wasm applier the desktop
// tests load in src/test/setup.ts), through the session's own propose route: validateOperationBatch
// is not mocked. A scripted agent proposes a refused edit, receives the explanation, and for every
// form the explanation offers sends a second proposal built from the explanation's own words (the
// dimension, the stated unit, the listed units, before TBD where it says so); each is registered
// for review. The table's dimensions and optional flags and the unit lists are checked against the
// same engine. Invented data; no provider, no native call, nothing launched.
//
// Not exercised here: the native host's admission of the tool arguments against the catalog
// schema (src-tauri embedded_agent; the scripted claim hands the arguments straight to the
// session; the retries are only checked against the catalog's admitted operations and its unit and
// dimension members below); a model reading the text; and the members the explanation does not
// state, which the script takes as an agent would from swbpipe_read_records: the target reference,
// the before value of a stored quantity, the number itself (each chosen to satisfy the field's
// value constraints) and the free-text members.
import { describe, expect, it, vi } from 'vitest';
import previewJson from '../../../../fixtures/product_preview/invented_preview_model.json';
import catalog from '../../src-tauri/resources/agent-capability-catalog.json';
import type { PreviewModel, ModelHashEvidence } from '../types';
import { EmbeddedAgentSession, agentHash, stampAgentOperations, type WorkspaceExecution, type AgentSolveOutcome } from './embeddedAgentSession';
import type { AgentBasis } from './embeddedAgentBridge';
import { validateOperationBatch } from './operationBatchService';
import { computeModelHash } from './hashService';
import { GENERIC_REFUSAL, UNIT_REFUSAL_FIELDS, UNIT_REFUSAL_UNITS, unitRefusalMessage, type QuantityField } from './agentUnitRefusal';

type AnyRecord = Record<string, unknown>;
const P = 'invented: unit refusal route check';
const MISMATCH = 'OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE';

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
const E = UNIT_REFUSAL_FIELDS.find(field => field.fieldPath === 'elastic_modulus.value')!, G = UNIT_REFUSAL_FIELDS.find(field => field.fieldPath === 'shear_modulus.value')!;
/** The first record of the field's collection holding the field: the record, the quantity's parent and key, and its stored number and unit. */
function target(doc: PreviewModel, field: QuantityField) {
 const path = field.fieldPath.split('.').slice(0, -1);
 for (const record of (doc as unknown as AnyRecord)[COLLECTION[field.objectType]] as AnyRecord[]) {
  let parent: AnyRecord | undefined = record;
  for (const segment of path.slice(0, -1)) parent = parent?.[segment] as AnyRecord | undefined;
  const pair = parent?.[path[path.length - 1]] as {value?: unknown; unit?: unknown} | undefined;
  if (typeof pair?.value === 'number' && typeof pair.unit === 'string') return {record, parent: parent!, key: path[path.length - 1], ref: String(record.id), value: pair.value, unit: pair.unit};
 }
 throw new Error(`no stored ${field.fieldPath}`);
}
/** Per dimension, an entered unit the first proposal uses with a bare number: another accepted unit where the dimension has one. */
const OTHER: Record<string, string | null> = {stress: 'GPa', thermal_expansion_coefficient: '1/degF', length: 'mm', density: 'lb/in^3', angle: 'deg', force: 'kN', area: 'mm^2', acceleration: null, dimensionless: '1', pressure: 'kPa'};
const NOT_OF_DIMENSION = 'in/s^3';
function op(field: QuantityField, ref: string, before: string, after: string, unit: string, dimension: string, id = `op:${field.fieldPath}`): AnyRecord {
 return {operation_id: id, operation_kind: 'modify', target: {object_type: field.objectType, ref},
  change: {change_id: `change:${id}`, change_kind: 'set_field', field_label: field.fieldPath, field_path: field.fieldPath, before, after, unit, dimension, source_note: P}, rationale: P};
}
async function validate(doc: PreviewModel, operations: AnyRecord[]) {
 const hash = await computeModelHash(doc);
 return validateOperationBatch(doc, {batch_id: 'agent-unit-refusal', operations: stampAgentOperations(operations, 'proposal', 'request'), source_model_hash: hash!.value}, hash);
}
const blockingCodes = (outcome: Awaited<ReturnType<typeof validate>>) => outcome.diagnostics.filter(d => d.severity === 'blocking').map(d => d.code);
const passes = async (doc: PreviewModel, operations: AnyRecord[], label: string) => {
 const outcome = await validate(doc, operations);
 expect(outcome.validation.batch_validation_status, `${label}: ${blockingCodes(outcome).join(' | ')}`).toBe('passed');
};

/** The catalog's admission of an operation, as far as this test can read it: an admitted kind and
 * field path, and a unit and dimension the operation schema admits. */
function admitted(operation: AnyRecord): boolean {
 const target = operation.target as AnyRecord, change = operation.change as AnyRecord;
 const kinds = catalog.admittedOperations as unknown as {operationKind: string; objectType: string; changeKind: string; fieldPaths: string[]}[];
 const schema = (catalog.definitions as unknown as {operationIntent: {properties: {change: {properties: {unit: {maxLength: number}; dimension: {enum: string[]}}}}}}).operationIntent.properties.change.properties;
 return kinds.some(kind => kind.operationKind === operation.operation_kind && kind.objectType === target.object_type && kind.changeKind === change.change_kind && kind.fieldPaths.includes(String(change.field_path)))
  && typeof change.unit === 'string' && change.unit.length >= 1 && change.unit.length <= schema.unit.maxLength && schema.dimension.enum.includes(String(change.dimension));
}

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

/** What a scripted agent takes from the explanation: only its fixed-word slots. */
function readRefusal(text: string) {
 const dimension = /has dimension ([a-z_]+)[,.]/.exec(text)?.[1] ?? null;
 const bare = /Resend it as a bare number with change\.unit (\S+) and change\.dimension ([a-z_]+), read in the stored unit and never converted/.exec(text);
 const pair = /after the JSON text \{"value": number, "unit": unit\} with change\.unit that unit and change\.dimension ([a-z_]+), unit one of (.+?)\.(?: Operations after|$)/.exec(text);
 const create = /Create it with before (TBD) and after the JSON text/.exec(text);
 return {dimension, bare: bare && {unit: bare[1], dimension: bare[2]}, pair: pair && {dimension: pair[1], units: pair[2].split(', ')}, createBefore: create?.[1] ?? null, earlier: text.includes('An earlier operation of this proposal may have changed what is stored')};
}
/** The retries the explanation offers for one operation, built from its words and the agent's own
 * knowledge of the target, before value and number. */
function retries(text: string, field: QuantityField, ref: string, before: string, value: number, id: string) {
 const read = readRefusal(text), out: {form: string; operation: AnyRecord}[] = [];
 if (read.bare) out.push({form: 'bare', operation: op(field, ref, before, String(value), read.bare.unit, read.bare.dimension, `${id}:bare`)});
 if (read.pair) {
  const unit = read.pair.units[read.pair.units.length - 1];
  out.push({form: read.createBefore ? 'create' : 'pair', operation: op(field, ref, read.createBefore ?? before, JSON.stringify({value, unit}), unit, read.pair.dimension, `${id}:pair`)});
 }
 return out;
}

describe('unit refusal explanation against the operation engine', () => {
 it('the catalog states the corrective pair form and admits the set_field route of every table field', () => {
  const description = catalog.tools.find(tool => tool.name === 'swbpipe_propose_operations')!.description;
  expect(description).toContain('after is JSON text; quantities are {value, unit}');
  for (const field of UNIT_REFUSAL_FIELDS) {
   expect(description, field.fieldPath).toContain(`modify/${field.objectType}/set_field`);
   expect(admitted(op(field, 'r', '1', '1', UNIT_REFUSAL_UNITS[field.dimension][0], field.dimension)), field.fieldPath).toBe(true);
  }
 });

 it('each table dimension and optional flag is the engine\'s', async () => {
  for (const field of UNIT_REFUSAL_FIELDS) {
   const doc = model(), stored = target(doc, field);
   await passes(doc, [op(field, stored.ref, String(stored.value), String(stored.value), stored.unit, field.dimension)], field.fieldPath);
   const otherDimension = field.dimension === 'length' ? 'angle' : 'length';
   const other = await validate(doc, [op(field, stored.ref, String(stored.value), String(stored.value), stored.unit, otherDimension)]);
   expect(blockingCodes(other), field.fieldPath).toEqual(['OP-UNIT-DIMENSION-MISMATCH']);
   // Absent: an optional quantity is created from before TBD and an explicit pair; a required one is refused.
   delete stored.parent[stored.key];
   const create = await validate(doc, [op(field, stored.ref, 'TBD', JSON.stringify({value: stored.value, unit: stored.unit}), stored.unit, field.dimension)]);
   if (field.optional) expect(create.validation.batch_validation_status, `${field.fieldPath}: ${blockingCodes(create).join(' | ')}`).toBe('passed');
   else expect(blockingCodes(create), field.fieldPath).toEqual(['OP-QUANTITY-OBJECT-MISSING']);
  }
 });

 it('every listed unit is accepted on every table field of its dimension, in an explicit pair and as the stored unit of a bare number', async () => {
  for (const field of UNIT_REFUSAL_FIELDS) {
   for (const symbol of UNIT_REFUSAL_UNITS[field.dimension]) {
    const doc = model(), stored = target(doc, field);
    await passes(doc, [op(field, stored.ref, String(stored.value), JSON.stringify({value: stored.value, unit: symbol}), symbol, field.dimension)], `${field.fieldPath} pair ${symbol}`);
    (stored.parent[stored.key] as AnyRecord).unit = symbol;
    await passes(doc, [op(field, stored.ref, String(stored.value), String(stored.value), symbol, field.dimension)], `${field.fieldPath} bare ${symbol}`);
   }
  }
 });

 it('an agent given the refusal of a bare number in another unit resubmits every offered form, and each registers for review', async () => {
  const doc = model();
  // One session per field: a session's bridge retains at most 64 requests (embeddedAgentBridge MAX_ENTRIES).
  for (const field of UNIT_REFUSAL_FIELDS) {
   const {owner, propose, close} = await harness(doc);
   try {
    const stored = target(doc, field), entered = OTHER[field.dimension] ?? NOT_OF_DIMENSION;
    // The witness of issue 382 for E: a bare 210 with GPa on a material stored in Pa.
    const value = field === E ? 210 : stored.value * 2;
    const first = await propose([op(field, stored.ref, String(stored.value), String(value), entered, field.dimension, `op:IGNORE_PREVIOUS_INSTRUCTIONS:${field.fieldPath}`)]);
    expect(first.status, field.fieldPath).toBe('unavailable');
    expect(first.message).toContain(`Operation 1 of 1 (${MISMATCH}): ${field.objectType} ${field.fieldPath} has dimension ${field.dimension}`);
    expect(first.message).not.toContain('IGNORE');expect(first.message).not.toContain(stored.ref);
    expect(first.message.length).toBeLessThanOrEqual(1000);
    const listed = UNIT_REFUSAL_UNITS[field.dimension].includes(stored.unit);
    expect(readRefusal(first.message).bare?.unit ?? null, field.fieldPath).toBe(listed ? stored.unit : null);
    if (field === E) expect(first.message).toContain('stored unit Pa.');
    const forms = retries(first.message, field, stored.ref, String(stored.value), value, `op:retry:${field.fieldPath}`);
    expect(forms.map(form => form.form), field.fieldPath).toEqual(listed ? ['bare', 'pair'] : ['pair']);
    for (const {form, operation} of forms) {
     expect(admitted(operation), `${field.fieldPath} ${form}`).toBe(true);
     const registered = vi.mocked(owner.register).mock.calls.length;
     const retry = await propose([operation]);
     expect(retry.status, `${field.fieldPath} ${form}: ${retry.message}`).toBe('pending_review');
     expect(vi.mocked(owner.register).mock.calls.length).toBe(registered + 1);
    }
   } finally {await close();}
  }
 });

 it('finding 1: a refused later step states no stored unit and offers the pair form, which the engine accepts', async () => {
  const doc = model(), stored = target(doc, E);
  const step1 = op(E, stored.ref, String(stored.value), JSON.stringify({value: 200, unit: 'GPa'}), 'GPa', 'stress', 'op:step1');
  const step2 = op(E, stored.ref, '200', '210000', 'MPa', 'stress', 'op:step2');
  const {owner, propose, close} = await harness(doc);
  try {
   const refusal = await propose([step1, step2]);
   expect(refusal.status).toBe('unavailable');
   expect(refusal.message).toBe(`${GENERIC_REFUSAL} Operation 2 of 2 (${MISMATCH}): Material elastic_modulus.value has dimension stress. An earlier operation of this proposal may have changed what is stored, so no stored unit is stated. Resend it as after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension stress, unit one of Pa, kPa, MPa, GPa, bar, psi, ksi.`);
   // The defect repaired: the session's document says Pa, but the engine judges step 2 against GPa,
   // so the bare form in Pa that the earlier wording offered is refused.
   expect(blockingCodes(await validate(doc, [step1, op(E, stored.ref, '200', '210000000000', 'Pa', 'stress', 'op:step2')]))).toEqual([MISMATCH]);
   const forms = retries(refusal.message, E, stored.ref, '200', 210, 'op:step2');
   expect(forms.map(form => form.form)).toEqual(['pair']);
   expect(admitted(forms[0].operation)).toBe(true);
   const retry = await propose([step1, forms[0].operation]);
   expect(retry.status, retry.message).toBe('pending_review');
   expect(vi.mocked(owner.register)).toHaveBeenCalledTimes(1);
  } finally {await close();}
 });

 it('finding 2a: an absent optional quantity is explained with the creating form alone, which the engine accepts', async () => {
  const doc = model(), stored = target(doc, G);
  delete stored.parent[stored.key];
  const {propose, close} = await harness(doc);
  try {
   for (const [unit, dimension, code] of [[NOT_OF_DIMENSION, 'stress', MISMATCH], ['GPa', 'pressure', 'OP-UNIT-DIMENSION-MISMATCH']]) {
    const refusal = await propose([op(G, stored.ref, 'TBD', '80', unit, dimension, `op:g:${dimension}`)]);
    expect(refusal.message).toBe(`${GENERIC_REFUSAL} Operation 1 of 1 (${code}): Material shear_modulus.value has dimension stress, and no value is stored. Create it with before TBD and after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension stress, unit one of Pa, kPa, MPa, GPa, bar, psi, ksi.`);
    expect(refusal.message).not.toContain('swbpipe_read_records');expect(refusal.message).not.toContain('bare number');
    const forms = retries(refusal.message, G, stored.ref, 'not used', 80, `op:g:${dimension}:retry`);
    expect(forms.map(form => form.form)).toEqual(['create']);
    expect(admitted(forms[0].operation)).toBe(true);
    const retry = await propose([forms[0].operation]);
    expect(retry.status, retry.message).toBe('pending_review');
   }
  } finally {await close();}
 });

 it('finding 2a on a material on the E/nu basis: the creating form the explanation offers is accepted there too', async () => {
  const doc = model(), stored = target(doc, G);
  delete stored.parent[stored.key];
  Object.assign(stored.record, {constitutive_basis: 'homogeneous_isotropic_E_nu_v1', poisson_ratio: {value: 0.3, unit: '1'}});
  const refused = await validate(doc, [op(G, stored.ref, 'TBD', '80', NOT_OF_DIMENSION, 'stress')]);
  const text = unitRefusalMessage(refused, [op(G, stored.ref, 'TBD', '80', NOT_OF_DIMENSION, 'stress')], doc);
  const forms = retries(text, G, stored.ref, 'not used', 80, 'op:g:enu');
  expect(forms.map(form => form.form)).toEqual(['create']);
  await passes(doc, [forms[0].operation], 'G created on an E/nu material');
 });

 it('finding 2b: a wrong dimension is corrected by both forms, each naming change.dimension', async () => {
  const doc = model(), stored = target(doc, E);
  const {propose, close} = await harness(doc);
  try {
   const refusal = await propose([op(E, stored.ref, String(stored.value), '210000000000', 'Pa', 'pressure', 'op:e:pressure')]);
   expect(refusal.message).toBe(`${GENERIC_REFUSAL} Operation 1 of 1 (OP-UNIT-DIMENSION-MISMATCH): Material elastic_modulus.value has dimension stress, stored unit Pa. Resend it as a bare number with change.unit Pa and change.dimension stress, read in the stored unit and never converted, or as after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension stress, unit one of Pa, kPa, MPa, GPa, bar, psi, ksi.`);
   const forms = retries(refusal.message, E, stored.ref, String(stored.value), 210e9, 'op:e:retry');
   expect(forms.map(form => form.form)).toEqual(['bare', 'pair']);
   for (const {form, operation} of forms) {
    expect((operation.change as AnyRecord).dimension).toBe('stress');
    const retry = await propose([operation]);
    expect(retry.status, `${form}: ${retry.message}`).toBe('pending_review');
   }
  } finally {await close();}
 });

 it('finding 2c: a stored number with no unit, sent with a wrong dimension, keeps the generic sentence because no form succeeds', async () => {
  const doc = model(), stored = target(doc, E);
  delete (stored.parent[stored.key] as AnyRecord).unit;
  const sent = [op(E, stored.ref, String(stored.value), '210000000000', 'Pa', 'pressure')];
  const outcome = await validate(doc, sent);
  expect(blockingCodes(outcome)).toEqual(['OP-UNIT-DIMENSION-MISMATCH']);
  expect(unitRefusalMessage(outcome, sent, doc)).toBe(GENERIC_REFUSAL);
  // Neither corrected form would be accepted: the engine wants the document's unit repaired first.
  expect(blockingCodes(await validate(doc, [op(E, stored.ref, String(stored.value), '210000000000', 'Pa', 'stress')]))).toEqual(['OP-UNIT-METADATA-MISSING']);
  expect(blockingCodes(await validate(doc, [op(E, stored.ref, String(stored.value), JSON.stringify({value: 210, unit: 'GPa'}), 'GPa', 'stress')]))).toEqual(['OP-UNIT-METADATA-MISSING']);
  const {propose, close} = await harness(doc);
  try {
   expect(await propose(sent)).toMatchObject({status: 'unavailable', message: GENERIC_REFUSAL});
  } finally {await close();}
 });

 it('a refusal for another reason keeps the generic sentence', async () => {
  const doc = model();
  const stored = target(doc, E);
  const {propose, close} = await harness(doc);
  try {
   const stale = await propose([op(E, stored.ref, String(stored.value + 1), JSON.stringify({value: 1, unit: 'GPa'}), 'GPa', 'stress')]);
   expect(stale).toMatchObject({status: 'unavailable', message: GENERIC_REFUSAL});
   const temperaturePoint = await propose([{...op(E, stored.ref, '1', '2', 'GPa', 'stress'), change: {...op(E, stored.ref, '1', '2', 'GPa', 'stress').change as AnyRecord, field_path: 'temperature_points.T1.shear_modulus.value'}}]);
   expect(temperaturePoint).toMatchObject({status: 'unavailable', message: GENERIC_REFUSAL});
  } finally {await close();}
 });

 it('a refused later step after a label edit on the same record is named by position, and the earlier steps are not', async () => {
  const doc = model();
  const stored = target(doc, E);
  const label = (n: number) => ({operation_id: `op:label:${n}`, operation_kind: 'modify', target: {object_type: 'Material', ref: stored.ref}, change: {change_id: `c:label:${n}`, change_kind: 'set_field', field_label: 'label', field_path: 'label', before: stored.record.label, after: 'Relabelled', unit: 'none', dimension: 'dimensionless', source_note: P}, rationale: P});
  const operations = [label(1), op(E, stored.ref, String(stored.value), '210', 'GPa', 'stress'), label(3)];
  const text = unitRefusalMessage(await validate(doc, operations), operations, doc);
  expect(text).toBe(`${GENERIC_REFUSAL} Operation 2 of 3 (${MISMATCH}): Material elastic_modulus.value has dimension stress. An earlier operation of this proposal may have changed what is stored, so no stored unit is stated. Resend it as after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension stress, unit one of Pa, kPa, MPa, GPa, bar, psi, ksi. Operations after operation 2 were not checked.`);
 });
});
