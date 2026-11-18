/** What swbpipe_propose_operations tells the agent when the operation engine refuses a proposal.
 *
 * A refusal for a unit or value/unit-pair reason on a known quantity field (issue 382: a bare
 * number in another unit is refused, not written beside the old unit) is explained in a closed
 * form: which operation by its position, which field, the field's dimension, its stored unit when
 * that validates, and the two corrective forms. Every other refusal, and anything that does not
 * validate, is the generic sentence, unchanged.
 *
 * Every word of the explanation is a literal of this module. The engine's outcome, the proposal
 * and the model only select literals: a finding code, an object type, a field path, a dimension
 * and a unit symbol reach the text only as the matching entry of the closed lists below, never as
 * the input string; positions and counts are integers. No engine message, remediation or affected
 * reference, no proposal text (identifier, rationale, label, after, target reference) and no
 * document string other than a validated unit symbol is read into the text. */

export const GENERIC_REFUSAL = 'The operation engine refused the proposal; nothing was queued.';

/** The catalog's message bound (agent-capability-catalog.json: every tool output's `message` is at
 * most 1000 characters). A composed explanation longer than this is replaced by the generic sentence. */
export const UNIT_REFUSAL_MAX_CHARS = 1000;
/** Refused operations described, in order; any further refused operations are counted, not described. */
export const UNIT_REFUSAL_STEPS = 2;
/** Bounds on the engine outcome read: the proposal's operation limit (embeddedAgentSession LIMITS) and
 * a cap on findings per list. An outcome past either is not read and the generic sentence is returned. */
const MAX_OPERATIONS = 16, MAX_FINDINGS = 64;

/** The unit and value/unit-pair refusal codes of the applier's quantity resolvers
 * (operation_applier lib.rs resolve_field, resolve_optional_quantity, stored_pair_edit_names_its_unit). */
export const UNIT_REFUSAL_CODES = Object.freeze(['OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE', 'OP-UNIT-DIMENSION-MISMATCH', 'OP-UNIT-DIMENSION-UNKNOWN', 'OP-UNIT-METADATA-MISSING', 'OP-QUANTITY-PAYLOAD-INVALID'] as const);

export type QuantityField = Readonly<{objectType: 'Material' | 'Element' | 'Component' | 'Load'; fieldPath: string; dimension: string}>;
/** The quantity fields the agent catalog admits through modify/<type>/set_field, each with the
 * dimension the applier's field_rules give it (FieldKind::Quantity or FieldKind::OptionalQuantity).
 * Paths carrying an identifier or index (a temperature point, a primitive load) are not here. */
export const UNIT_REFUSAL_FIELDS: readonly QuantityField[] = Object.freeze([
 {objectType: 'Material', fieldPath: 'elastic_modulus.value', dimension: 'stress'},
 {objectType: 'Material', fieldPath: 'shear_modulus.value', dimension: 'stress'},
 {objectType: 'Material', fieldPath: 'thermal_expansion_coefficient.value', dimension: 'thermal_expansion_coefficient'},
 {objectType: 'Element', fieldPath: 'section.outside_diameter.value', dimension: 'length'},
 {objectType: 'Element', fieldPath: 'section.wall_thickness.value', dimension: 'length'},
 {objectType: 'Element', fieldPath: 'section.corrosion_allowance.value', dimension: 'length'},
 {objectType: 'Element', fieldPath: 'section.mill_tolerance.value', dimension: 'length'},
 {objectType: 'Element', fieldPath: 'section.insulation_thickness.value', dimension: 'length'},
 {objectType: 'Element', fieldPath: 'section.material_density.value', dimension: 'density'},
 {objectType: 'Element', fieldPath: 'section.contents_density.value', dimension: 'density'},
 {objectType: 'Element', fieldPath: 'section.insulation_density.value', dimension: 'density'},
 {objectType: 'Component', fieldPath: 'geometry.bend_radius.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.bend_angle.value', dimension: 'angle'},
 {objectType: 'Component', fieldPath: 'geometry.branch_run_size.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.branch_header_size.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.branch_connection_angle.value', dimension: 'angle'},
 {objectType: 'Component', fieldPath: 'geometry.rigid_body_length.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.end_a_size.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.end_b_size.value', dimension: 'length'},
 {objectType: 'Component', fieldPath: 'geometry.weight.value', dimension: 'force'},
 {objectType: 'Component', fieldPath: 'geometry.effective_area.value', dimension: 'area'},
 {objectType: 'Component', fieldPath: 'geometry.movement_limit.value', dimension: 'length'},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.gravity_acceleration.value', dimension: 'acceleration'},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_x.value', dimension: 'dimensionless'},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_y.value', dimension: 'dimensionless'},
 {objectType: 'Load', fieldPath: 'equivalent_static.seismic.g_factor_z.value', dimension: 'dimensionless'},
 {objectType: 'Load', fieldPath: 'equivalent_static.wind.shape_factor.value', dimension: 'dimensionless'},
 {objectType: 'Load', fieldPath: 'equivalent_static.wind.pressure.value', dimension: 'pressure'}
].map(entry => Object.freeze(entry as QuantityField)));

/** The collection each table object type's records are in (operation_applier lib.rs collection_for). */
const COLLECTIONS: Readonly<Record<QuantityField['objectType'], string>> = Object.freeze({Material: 'materials', Element: 'pipe_segments', Component: 'components', Load: 'load_cases'});

/** Unit symbols a stored unit may be stated as, by dimension: the symbols of the desktop's own
 * closed display table (unitConversion.ts, reached synchronously, with no native call) that the
 * operation engine also accepts for that dimension (both checked by agentUnitRefusal.engine.test.ts). A
 * dimension not listed has no closed vocabulary here, and its stored unit is not stated. */
export const STATED_UNITS: Readonly<Record<string, readonly string[]>> = Object.freeze({
 stress: Object.freeze(['Pa', 'kPa', 'MPa', 'GPa', 'bar', 'psi', 'ksi']),
 pressure: Object.freeze(['Pa', 'kPa', 'MPa', 'GPa', 'bar', 'psi', 'ksi']),
 length: Object.freeze(['m', 'mm', 'in', 'ft']),
 force: Object.freeze(['N', 'kN', 'lbf']),
 angle: Object.freeze(['rad', 'deg'])
});

const plain = (x: unknown): x is Record<string, unknown> => !!x && typeof x === 'object' && !Array.isArray(x);
const own = (record: Record<string, unknown>, key: string): unknown => Object.hasOwn(record, key) ? record[key] : undefined;
const boundedList = (x: unknown, max: number): unknown[] | null => Array.isArray(x) && x.length <= max ? x : null;
const blocking = (finding: unknown) => plain(finding) && own(finding, 'severity') === 'blocking';
/** The recognised code literal equal to a finding's code, or null. */
const recognised = (finding: unknown) => plain(finding) ? UNIT_REFUSAL_CODES.find(code => code === own(finding, 'code')) ?? null : null;

/** The stored unit of a table field on the targeted record, as the matching literal of its
 * dimension's closed vocabulary; null when absent, not a string, or not in that vocabulary. */
function statedUnit(model: unknown, field: QuantityField, ref: unknown): string | null {
 const vocabulary = STATED_UNITS[field.dimension];
 if (!vocabulary || typeof ref !== 'string' || !plain(model)) return null;
 const records = boundedList(own(model, COLLECTIONS[field.objectType]), 100_000);
 const record = records?.find(item => plain(item) && own(item, 'id') === ref);
 let node: unknown = record;
 for (const segment of [...field.fieldPath.split('.').slice(0, -1), 'unit']) node = plain(node) ? own(node, segment) : undefined;
 return vocabulary.find(symbol => symbol === node) ?? null;
}

type Explained = {position: number; code: typeof UNIT_REFUSAL_CODES[number]; field: QuantityField; unit: string | null};

/** One refused operation's sentence: fixed words, slots filled only from the literals above. */
function stepText(step: Explained, total: number): string {
 const {field, unit} = step;
 const stored = unit ? `, stored unit ${unit}` : '; read its stored unit with swbpipe_read_records';
 return ` Operation ${step.position} of ${total} (${step.code}): ${field.objectType} ${field.fieldPath} has dimension ${field.dimension}${stored}.`
  + ` Resend it as a bare number with change.unit ${unit ?? 'its stored unit'}, or as after the JSON text {"value": number, "unit": unit} with change.unit that unit and change.dimension ${field.dimension}.`;
}

/** The message for a refused proposal: the closed explanation when every refusal is a unit or
 * value/unit-pair refusal on a table field, otherwise the generic sentence. `outcome` is the
 * engine's batch outcome, `operations` the proposal's operations and `model` the session's model
 * the proposal was validated against; all three are read as untrusted. */
export function unitRefusalMessage(outcome: unknown, operations: unknown, model: unknown): string {
 try {
  const ops = boundedList(operations, MAX_OPERATIONS);
  if (!ops?.length || !plain(outcome) || !plain(outcome.validation) || outcome.validation.batch_validation_status !== 'blocked') return GENERIC_REFUSAL;
  const findings = boundedList(own(outcome, 'diagnostics'), MAX_FINDINGS), steps = boundedList(own(outcome, 'operation_outcomes'), ops.length);
  if (!findings || !steps) return GENERIC_REFUSAL;
  // Every blocking finding of the batch must be a recognised unit refusal.
  const blockingFindings = findings.filter(blocking);
  if (!blockingFindings.length || blockingFindings.some(finding => !recognised(finding))) return GENERIC_REFUSAL;
  const refused: Explained[] = [];
  for (let i = 0; i < steps.length; i++) {
   const step = steps[i];
   if (!plain(step) || own(step, 'index') !== i) return GENERIC_REFUSAL;
   const status = own(step, 'simulation_status');
   if (status === 'validated_on_temporary_state') continue;
   if (status !== 'blocked') return GENERIC_REFUSAL;
   const stepFindings = boundedList(own(step, 'diagnostics'), MAX_FINDINGS)?.filter(blocking);
   const code = stepFindings?.length ? recognised(stepFindings[0]) : null;
   if (!code || stepFindings!.some(finding => !recognised(finding))) return GENERIC_REFUSAL;
   const operation = ops[i];
   const target = plain(operation) ? own(operation, 'target') : undefined, change = plain(operation) ? own(operation, 'change') : undefined;
   if (!plain(operation) || !plain(target) || !plain(change) || own(operation, 'operation_kind') !== 'modify' || own(change, 'change_kind') !== 'set_field' || own(step, 'change_kind') !== 'set_field') return GENERIC_REFUSAL;
   const field = UNIT_REFUSAL_FIELDS.find(entry => entry.objectType === own(target, 'object_type') && entry.objectType === own(step, 'target_object_type') && entry.fieldPath === own(change, 'field_path'));
   if (!field) return GENERIC_REFUSAL;
   refused.push({position: i + 1, code, field, unit: statedUnit(model, field, own(target, 'ref'))});
  }
  if (!refused.length) return GENERIC_REFUSAL;
  const described = refused.slice(0, UNIT_REFUSAL_STEPS), more = refused.length - described.length;
  const last = refused[refused.length - 1].position;
  const text = GENERIC_REFUSAL + described.map(step => stepText(step, ops.length)).join('')
   + ' A bare number is read in the stored unit, never converted.'
   + (more ? ` ${more} more operation${more === 1 ? ' was' : 's were'} refused.` : '')
   + (last < ops.length ? ` Operations after operation ${last} were not checked.` : '');
  return text.length <= UNIT_REFUSAL_MAX_CHARS ? text : GENERIC_REFUSAL;
 } catch {
  return GENERIC_REFUSAL;
 }
}
