/** What swbpipe_propose_operations tells the agent when the operation engine refuses a proposal.
 *
 * A refusal for a unit or value/unit-pair reason on a known quantity field (issue 382: a bare
 * number in another unit is refused, not written beside the old unit) is explained in a closed
 * form: which operation by its position, which field, the field's dimension, and only the
 * corrective forms the engine would accept in the field's state. Every form names the field's
 * dimension as change.dimension and the units it may carry.
 *
 * The engine checks each step against the temporary document the earlier steps produced and stops
 * at the first refused step (operation_applier atomic_batch.rs). The session's document is
 * therefore what the engine evaluated only for the first step. When the first step is refused,
 * the field's state in the session's document selects the forms: a stored number with a unit
 * beside it gets the explicit pair form, and the bare-number form too when the stored unit is a
 * literal of the field's unit list (it is then stated); an absent optional quantity the engine
 * can create gets the creating form alone. When a later step is refused, no stored unit is
 * stated and only the explicit pair form is offered, in fixed words saying that an earlier
 * operation may have changed what is stored, and only when every earlier operation on the same
 * record is a set_field on a field of the table below or on the record's label or provenance.
 * A pipe with a section binding is not explained. Every other refusal, every other state,
 * and anything that does not validate, is the generic sentence, unchanged.
 *
 * The outcome must fit the engine's sequence: one step per operation up to the refused one, each
 * bound to the submitted operation at its position by its identities (operation_id, change_id,
 * target object type and reference, change_kind), every earlier step validated, and exactly one
 * refused step, the last one present. Only own enumerable data properties are read; every array
 * is taken through Array.isArray with its length read once as a checked safe integer within the
 * bounds below.
 *
 * Every word of the explanation is a literal of this module. The engine's outcome, the proposal
 * and the model only select literals: a finding code, an object type, a field path, a dimension
 * and a unit symbol reach the text only as the matching entry of the closed lists below, never as
 * the input string; positions and counts are integers this module computed, never values read
 * from an input. No engine message, remediation or affected reference, no proposal text
 * (identifier, rationale, label, before, after, target reference) and no document string other
 * than a stored unit equal to a literal of the field's unit list is read into the text. */

export const GENERIC_REFUSAL = 'The operation engine refused the proposal; nothing was queued.';

/** The catalog's message bound (agent-capability-catalog.json: every tool output's `message` is at
 * most 1000 characters). A composed explanation longer than this is replaced by the generic sentence. */
export const UNIT_REFUSAL_MAX_CHARS = 1000;
/** Bounds on what is read: the proposal's operation limit (embeddedAgentSession LIMITS), a cap on
 * findings per list and on records per collection. An input past any of them is not read and the
 * generic sentence is returned. */
const MAX_OPERATIONS = 16, MAX_FINDINGS = 64, MAX_RECORDS = 100_000;

/** The unit and value/unit-pair refusal codes of the applier's quantity resolvers
 * (operation_applier lib.rs resolve_field, resolve_optional_quantity, stored_pair_edit_names_its_unit)
 * that the corrective forms below answer. OP-UNIT-METADATA-MISSING is not among them: a stored
 * quantity with no unit is refused whichever form is sent, so naming those forms would mislead. */
export const UNIT_REFUSAL_CODES = Object.freeze(['OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE', 'OP-UNIT-DIMENSION-MISMATCH', 'OP-UNIT-DIMENSION-UNKNOWN', 'OP-QUANTITY-PAYLOAD-INVALID'] as const);

export type QuantityField = Readonly<{objectType: 'Material' | 'Element' | 'Component' | 'Load'; fieldPath: string; dimension: string; optional: boolean}>;
/** The quantity fields the agent catalog admits through modify/<type>/set_field, each with the
 * dimension the applier's field_rules give it, and whether the rule is FieldKind::OptionalQuantity
 * (an absent value can be created) or FieldKind::Quantity (an absent value is refused,
 * OP-QUANTITY-OBJECT-MISSING). Paths carrying an identifier or index (a temperature point, a
 * primitive load) are not here. */
export const UNIT_REFUSAL_FIELDS: readonly QuantityField[] = Object.freeze([
 {objectType: 'Material', fieldPath: 'elastic_modulus.value', dimension: 'stress', optional: false},
 {objectType: 'Material', fieldPath: 'shear_modulus.value', dimension: 'stress', optional: true},
 {objectType: 'Material', fieldPath: 'thermal_expansion_coefficient.value', dimension: 'thermal_expansion_coefficient', optional: false},
 {objectType: 'Element', fieldPath: 'section.outside_diameter.value', dimension: 'length', optional: false},
 {objectType: 'Element', fieldPath: 'section.wall_thickness.value', dimension: 'length', optional: false},
 {objectType: 'Element', fieldPath: 'section.corrosion_allowance.value', dimension: 'length', optional: true},
 {objectType: 'Element', fieldPath: 'section.mill_tolerance.value', dimension: 'length', optional: true},
 {objectType: 'Element', fieldPath: 'section.insulation_thickness.value', dimension: 'length', optional: true},
 {objectType: 'Element', fieldPath: 'section.material_density.value', dimension: 'density', optional: true},
 {objectType: 'Element', fieldPath: 'section.contents_density.value', dimension: 'density', optional: true},
 {objectType: 'Element', fieldPath: 'section.insulation_density.value', dimension: 'density', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.bend_radius.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.bend_angle.value', dimension: 'angle', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.branch_run_size.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.branch_header_size.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.branch_connection_angle.value', dimension: 'angle', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.rigid_body_length.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.end_a_size.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.end_b_size.value', dimension: 'length', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.weight.value', dimension: 'force', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.effective_area.value', dimension: 'area', optional: true},
 {objectType: 'Component', fieldPath: 'geometry.movement_limit.value', dimension: 'length', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.gravity_acceleration.value', dimension: 'acceleration', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_x.value', dimension: 'dimensionless', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_y.value', dimension: 'dimensionless', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_z.value', dimension: 'dimensionless', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.wind.shape_factor.value', dimension: 'dimensionless', optional: true},
 {objectType: 'Load', fieldPath: 'equivalent_static.wind.pressure.value', dimension: 'pressure', optional: true}
].map(entry => Object.freeze(entry as QuantityField)));

/** The collection each table object type's records are in (operation_applier lib.rs collection_for). */
const COLLECTIONS: Readonly<Record<QuantityField['objectType'], string>> = Object.freeze({Material: 'materials', Element: 'pipe_segments', Component: 'components', Load: 'load_cases'});

/** The units a form may name for each table dimension, and the only symbols a stored unit may be
 * stated as: symbols of the engine's unit catalog (core/units) for that dimension, plus `none`,
 * which the applier accepts for its optional dimensionless slots. agentUnitRefusal.engine.test.ts
 * checks that the engine accepts every symbol in an explicit pair, and as the stored unit of a bare
 * number, on every table field of its dimension. */
export const UNIT_REFUSAL_UNITS: Readonly<Record<string, readonly string[]>> = Object.freeze({
 stress: Object.freeze(['Pa', 'kPa', 'MPa', 'GPa', 'bar', 'psi', 'ksi']),
 pressure: Object.freeze(['Pa', 'kPa', 'MPa', 'GPa', 'bar', 'psi', 'ksi']),
 length: Object.freeze(['m', 'mm', 'in', 'ft']),
 force: Object.freeze(['N', 'kN', 'lbf']),
 angle: Object.freeze(['rad', 'deg']),
 density: Object.freeze(['kg/m^3', 'lb/in^3']),
 area: Object.freeze(['m^2', 'mm^2', 'in^2']),
 acceleration: Object.freeze(['m/s^2']),
 thermal_expansion_coefficient: Object.freeze(['1/K', '1/degC', '1/degF']),
 dimensionless: Object.freeze(['none', '1'])
});

/** Anything that does not fit is thrown and becomes the generic sentence. */
class Unfit extends Error {}
const unfit = (): never => {throw new Unfit();};

/** A plain data object as JSON.stringify would carry it to the engine: no array, no prototype
 * other than Object.prototype or null, no own toJSON. */
function plain(x: unknown): x is Record<string, unknown> {
 if (!x || typeof x !== 'object' || Array.isArray(x)) return false;
 const proto = Object.getPrototypeOf(x);
 return (proto === Object.prototype || proto === null) && !Object.getOwnPropertyDescriptor(x, 'toJSON');
}
/** The own enumerable data property `key` of a plain object; undefined when absent. An accessor or
 * non-enumerable member, or a record that is not a plain object, does not fit. */
function own(record: unknown, key: string): unknown {
 if (!plain(record)) return unfit();
 const slot = Object.getOwnPropertyDescriptor(record, key);
 if (!slot) return undefined;
 if (!('value' in slot) || !slot.enumerable) return unfit();
 return slot.value;
}
/** A copy of an array of at most `max` elements, its length read once as a checked safe integer and
 * every element an own enumerable data property. */
function list(x: unknown, max: number): unknown[] {
 if (!Array.isArray(x) || Object.getPrototypeOf(x) !== Array.prototype) return unfit();
 const length = Object.getOwnPropertyDescriptor(x, 'length');
 const n: unknown = length && 'value' in length ? length.value : undefined;
 if (typeof n !== 'number' || !Number.isSafeInteger(n) || n < 0 || n > max) return unfit();
 const copy: unknown[] = [];
 for (let i = 0; i < n; i++) {
  const slot = Object.getOwnPropertyDescriptor(x, String(i));
  if (!slot || !('value' in slot) || !slot.enumerable) return unfit();
  copy.push(slot.value);
 }
 return copy;
}
const blocking = (finding: unknown) => own(finding, 'severity') === 'blocking';
/** The recognised code literal equal to a finding's code, or null. */
const recognised = (finding: unknown) => {const code = own(finding, 'code');return UNIT_REFUSAL_CODES.find(literal => literal === code) ?? null;};
const tableField = (objectType: unknown, fieldPath: unknown) => UNIT_REFUSAL_FIELDS.find(entry => entry.objectType === objectType && entry.fieldPath === fieldPath);

/** The field's state on the targeted record of the session's document, as the engine reads it:
 * `pair` (a finite number with a unit string beside it; `unit` is the matching literal of the
 * field's unit list, or null), `absent` (an optional quantity with no number, every enclosing
 * member an object or absent, so the engine creates it), or null for every other state. */
type FieldState = {kind: 'pair'; unit: string | null} | {kind: 'absent'};
function fieldState(model: unknown, field: QuantityField, ref: unknown): FieldState | null {
 if (typeof ref !== 'string') return null;
 const matches = list(own(model, COLLECTIONS[field.objectType]), MAX_RECORDS).filter(record => own(record, 'id') === ref);
 if (matches.length !== 1) return null;
 const record = matches[0];
 // A pipe bound to a section refuses a local diameter or wall edit, and a malformed binding
 // refuses every edit of the pipe (operation_applier section_bindings.rs validate_local).
 if (field.objectType === 'Element' && own(record, 'section_ref') !== undefined) return null;
 let node: unknown = record;
 for (const segment of field.fieldPath.split('.').slice(0, -1)) {
  if (node === undefined) break;
  if (!plain(node)) return null;
  node = own(node, segment);
 }
 if (node === undefined) return field.optional ? {kind: 'absent'} : null;
 if (!plain(node)) return null;
 const value = own(node, 'value'), unit = own(node, 'unit');
 if (typeof value === 'number') {
  if (!Number.isFinite(value) || typeof unit !== 'string') return null;
  return {kind: 'pair', unit: UNIT_REFUSAL_UNITS[field.dimension].find(symbol => symbol === unit) ?? null};
 }
 return field.optional ? {kind: 'absent'} : null;
}

/** The explanation of the refused step: fixed words, slots filled only from the literals above and
 * integers computed here. */
function stepText(code: string, field: QuantityField, state: FieldState, position: number, total: number): string {
 const {objectType, fieldPath, dimension} = field;
 const pair = `after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension ${dimension}, unit one of ${UNIT_REFUSAL_UNITS[dimension].join(', ')}`;
 const head = `${GENERIC_REFUSAL} Operation ${position} of ${total} (${code}): ${objectType} ${fieldPath} has dimension ${dimension}`;
 let body: string;
 if (position > 1) body = `. An earlier operation of this proposal may have changed what is stored, so no stored unit is stated. Resend it as ${pair}.`;
 else if (state.kind === 'absent') body = `, and no value is stored. Create it with before TBD and ${pair}.`;
 else if (state.unit) body = `, stored unit ${state.unit}. Resend it as a bare number with change.unit ${state.unit} and change.dimension ${dimension}, read in the stored unit and never converted, or as ${pair}.`;
 else body = `. Resend it as ${pair}.`;
 return head + body + (position < total ? ` Operations after operation ${position} were not checked.` : '');
}

function explain(outcome: unknown, operations: unknown, model: unknown): string {
 const ops = list(operations, MAX_OPERATIONS), total = ops.length;
 if (!total || own(own(outcome, 'validation'), 'batch_validation_status') !== 'blocked') return unfit();
 // Every blocking finding of the batch must be a recognised unit refusal.
 const batchBlocking = list(own(outcome, 'diagnostics'), MAX_FINDINGS).filter(blocking);
 if (!batchBlocking.length || batchBlocking.some(finding => !recognised(finding))) return unfit();
 const steps = list(own(outcome, 'operation_outcomes'), total), last = steps.length - 1;
 if (last < 0) return unfit();
 for (let i = 0; i <= last; i++) {
  const step = steps[i], operation = ops[i];
  const target = own(operation, 'target'), change = own(operation, 'change');
  if (own(step, 'index') !== i) return unfit();
  // The step is the engine's account of the submitted operation at the same position.
  const identities: [unknown, unknown][] = [[own(step, 'operation_id'), own(operation, 'operation_id')], [own(step, 'change_id'), own(change, 'change_id')],
   [own(step, 'target_object_type'), own(target, 'object_type')], [own(step, 'target_ref'), own(target, 'ref')], [own(step, 'change_kind'), own(change, 'change_kind')]];
  if (identities.some(([engine, submitted]) => typeof engine !== 'string' || engine !== submitted)) return unfit();
  const stepBlocking = list(own(step, 'diagnostics'), MAX_FINDINGS).filter(blocking);
  // The engine's sequence: every earlier step validated with no blocking finding, the last one refused.
  if (i < last ? own(step, 'simulation_status') !== 'validated_on_temporary_state' || stepBlocking.length : own(step, 'simulation_status') !== 'blocked' || !stepBlocking.length) return unfit();
  if (i === last && stepBlocking.some(finding => !recognised(finding))) return unfit();
 }
 const refused = ops[last], target = own(refused, 'target'), change = own(refused, 'change');
 const code = recognised(list(own(steps[last], 'diagnostics'), MAX_FINDINGS).filter(blocking)[0]);
 if (!code || own(refused, 'operation_kind') !== 'modify' || own(change, 'change_kind') !== 'set_field') return unfit();
 const field = tableField(own(target, 'object_type'), own(change, 'field_path'));
 if (!field) return unfit();
 const ref = own(target, 'ref'), state = fieldState(model, field, ref);
 if (!state) return unfit();
 // A later step's form holds only where every earlier operation on the same record kept the
 // field a value/unit pair or an absent slot and left the record's other state alone: a set_field
 // on a field of the table, or on its label or provenance text.
 for (let i = 0; i < last; i++) {
  const earlier = ops[i], earlierTarget = own(earlier, 'target'), earlierChange = own(earlier, 'change');
  if (own(earlierTarget, 'object_type') !== field.objectType || own(earlierTarget, 'ref') !== ref) continue;
  const path = own(earlierChange, 'field_path');
  if (own(earlier, 'operation_kind') !== 'modify' || own(earlierChange, 'change_kind') !== 'set_field' || !(path === 'label' || path === 'provenance' || tableField(field.objectType, path))) return unfit();
 }
 const text = stepText(code, field, state, last + 1, total);
 return text.length <= UNIT_REFUSAL_MAX_CHARS ? text : unfit();
}

/** The message for a refused proposal: the closed explanation when the engine's outcome fits its
 * sequence and its one refused step is a unit or value/unit-pair refusal on a table field in a state
 * some corrective form answers, otherwise the generic sentence. `outcome` is the engine's batch
 * outcome, `operations` the operations submitted to the engine and `model` the session's model the
 * proposal was validated against; all three are read as untrusted. */
export function unitRefusalMessage(outcome: unknown, operations: unknown, model: unknown): string {
 try {
  return explain(outcome, operations, model);
 } catch {
  return GENERIC_REFUSAL;
 }
}
