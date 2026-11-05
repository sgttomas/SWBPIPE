// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §1.2, §4.2, §11, §12):
// the desktop's condition-set operations. Every edit is one EditorOperationIntent of
// S3a's wire, checked by the operation engine and queued for Review/Apply; nothing
// here writes the model, and nothing is defaulted: a record carries only what the
// person entered, and members the forms do not edit are carried as authored.
//
// Wire (core/model_operations/operation_applier/src/condition_set_authoring.rs):
// - create_condition_set / update_condition_set / delete_condition_set: object type
//   `ConditionSet`, field path `condition_sets`, unit `none`, dimension
//   `dimensionless`; `before` `not_present` (create) or the current record's canonical
//   JSON; `after` the whole record's JSON text, or `not_present` (delete);
// - assign_condition_set: `Element`, `condition_set_ref`; `before` `{"condition_set_ref":id}`
//   canonical or `not_present`; `after` `{"condition_set_ref":id}`, or `not_present` to clear;
// - set_load_case_condition_use / clear_load_case_condition_use: `Load`, `condition_use`;
//   `after` the whole ConditionUse object (set) or `not_present` (clear).
import type {
  ConditionDirection,
  ConditionSet,
  ConditionUse,
  EditorOperationIntent,
  OperatingCondition,
  PreviewModel,
  QuantityValue
} from "../../types";
import { modelDataIntent, stableJson, type Canonicalize } from "../model-data/modelDataIntents";
import { parseQuantities } from "../rich-authoring/formSupport";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";

export const NOT_PRESENT = "not_present";
/** §1.2: the visible statement on every member the current version's solve does not read
 * ("not used by the solve in 0.9.0"). The version is named through the one constant, so the
 * next version step re-checks each labelled member (the version-literal guard). */
export const NOT_USED_BY_SOLVE = `not used by the solve in ${MODEL_SCHEMA_VERSION}`;
/** An empty cell's text: nothing is prefilled. */
export const NOT_STATED = "not stated";
export const CONDITION_DIRECTIONS: readonly ConditionDirection[] = ["global_x", "global_y", "global_z"];
export const CONDITION_SET_SOURCE = {
  source_ref: "apps/desktop/src/features/condition-sets",
  source_channel: "local_desktop_preview",
  source_role: "gui_editor"
} as const;

/** The dimensions on main's unit catalog (`core/units`): absolute temperature is `temperature`. */
export const DIMENSIONS = {
  temperature: "temperature",
  pressure: "pressure",
  density: "density",
  forcePerLength: "force_per_length"
} as const;

export type QuantityDraft = { value: string; unit: string };
/** An exposure mark: not stated (absent), `true`, `false`, or "stored": a stored value the
 * engine refuses (for example `null`), shown as stored and kept unless the person replaces it. */
export type TriState = "" | "true" | "false" | "stored";
export type DirectionDraft = "" | ConditionDirection | "stored";
export type ConditionDraft = {
  key: string;
  label: string;
  temperature: QuantityDraft;
  pressure: QuantityDraft;
  contentsDensity: QuantityDraft;
  /** The index of the stored condition this row edits; absent for a row the person added. */
  origin?: number;
  /** Member names of the stored row the person explicitly removed (a refused stored value). */
  removed: string[];
};
export type ConditionSetDraft = {
  id: string;
  label: string;
  provenance: string;
  installationTemperature: QuantityDraft;
  conditions: ConditionDraft[];
  designTemperature: QuantityDraft;
  designPressure: QuantityDraft;
  peakTemperature: QuantityDraft;
  peakPressure: QuantityDraft;
  addedWeightMagnitude: QuantityDraft;
  addedWeightDirection: DirectionDraft;
  wind: TriState;
  snowIce: TriState;
  wave: TriState;
  /** Member paths of the stored set the person explicitly removed (a refused stored value). */
  removed: string[];
  /** An edit's basis: the stored record and the draft it gave. An update writes the stored
   * record with only the members whose draft differs from this one changed. */
  base?: { record: ConditionSet; draft: ConditionSetDraft };
};

const SET_EDITED = new Set(["id", "label", "provenance", "installation_temperature", "conditions", "design", "peak", "added_weight", "exposure"]);
const CONDITION_EDITED = new Set(["key", "label", "temperature", "pressure", "contents_density"]);
const NESTED_EDITED: ReadonlyArray<readonly [string, ReadonlySet<string>]> = [
  ["design", new Set(["temperature", "pressure"])], ["peak", new Set(["temperature", "pressure"])],
  ["added_weight", new Set(["magnitude", "direction"])], ["exposure", new Set(["wind", "snow_ice", "wave"])]
];

type Json = Record<string, unknown>;
const isObject = (value: unknown): value is Json => Boolean(value) && typeof value === "object" && !Array.isArray(value);
const has = (record: unknown, key: string) => isObject(record) && Object.hasOwn(record, key);
/** A value the form can show and edit as a quantity: exactly `{value: finite number, unit: string}`. */
function isQuantity(value: unknown): value is QuantityValue {
  return isObject(value) && Object.keys(value).length === 2 && typeof value.value === "number" && Number.isFinite(value.value) && typeof value.unit === "string";
}
const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;
const same = (left: unknown, right: unknown) => stableJson(left) === stableJson(right);

const emptyQuantity = (): QuantityDraft => ({ value: "", unit: "" });
/** The text of a stored quantity; a stored value that is not a quantity shows no text (the
 * form lists it as stored and refused, and keeps it). */
export function quantityDraft(quantity: unknown): QuantityDraft {
  return isQuantity(quantity) ? { value: String(quantity.value), unit: quantity.unit } : emptyQuantity();
}
const text = (value: unknown) => (typeof value === "string" ? value : "");
function triState(record: unknown, key: string): TriState {
  if (!has(record, key)) return "";
  const value = (record as Json)[key];
  return value === true ? "true" : value === false ? "false" : "stored";
}

export function emptyConditionDraft(): ConditionDraft {
  return { key: "", label: "", temperature: emptyQuantity(), pressure: emptyQuantity(), contentsDensity: emptyQuantity(), removed: [] };
}

/** A blank draft: no member is prefilled. */
export function emptyConditionSetDraft(): ConditionSetDraft {
  return {
    id: "", label: "", provenance: "", installationTemperature: emptyQuantity(), conditions: [],
    designTemperature: emptyQuantity(), designPressure: emptyQuantity(), peakTemperature: emptyQuantity(), peakPressure: emptyQuantity(),
    addedWeightMagnitude: emptyQuantity(), addedWeightDirection: "", wind: "", snowIce: "", wave: "", removed: []
  };
}

/** The draft of a stored set: each value the form can show, as stored (text unchanged,
 * whitespace kept). The stored record is the draft's basis, so an untouched member keeps its
 * exact stored value and presence. */
export function draftFromSet(set: ConditionSet): ConditionSetDraft {
  const record = set as unknown as Json;
  const design = record.design, peak = record.peak, weight = record.added_weight, exposure = record.exposure;
  const direction = isObject(weight) && has(weight, "direction")
    ? ((CONDITION_DIRECTIONS as readonly unknown[]).includes(weight.direction) ? weight.direction as ConditionDirection : "stored")
    : "";
  const draft: ConditionSetDraft = {
    id: text(record.id),
    label: text(record.label),
    provenance: text(record.provenance),
    installationTemperature: quantityDraft(record.installation_temperature),
    conditions: (Array.isArray(record.conditions) ? record.conditions : []).map((condition: unknown, index: number) => ({
      key: text(isObject(condition) ? condition.key : undefined),
      label: text(isObject(condition) ? condition.label : undefined),
      temperature: quantityDraft(isObject(condition) ? condition.temperature : undefined),
      pressure: quantityDraft(isObject(condition) ? condition.pressure : undefined),
      contentsDensity: quantityDraft(isObject(condition) ? condition.contents_density : undefined),
      origin: index,
      removed: []
    })),
    designTemperature: quantityDraft(isObject(design) ? design.temperature : undefined),
    designPressure: quantityDraft(isObject(design) ? design.pressure : undefined),
    peakTemperature: quantityDraft(isObject(peak) ? peak.temperature : undefined),
    peakPressure: quantityDraft(isObject(peak) ? peak.pressure : undefined),
    addedWeightMagnitude: quantityDraft(isObject(weight) ? weight.magnitude : undefined),
    addedWeightDirection: direction,
    wind: triState(exposure, "wind"),
    snowIce: triState(exposure, "snow_ice"),
    wave: triState(exposure, "wave"),
    removed: []
  };
  return { ...draft, base: { record: set, draft } };
}

/** True when the draft edits nothing of its stored basis (no member changed or removed). */
export function draftUnchanged(draft: ConditionSetDraft): boolean {
  if (!draft.base) return false;
  const { base: _base, ...current } = draft;
  return same(current, draft.base.draft);
}

/** An entered quantity, or undefined when both cells are blank. A half-entered quantity
 * or a value that is not a finite number is refused by name; nothing is defaulted. */
export function quantityFromDraft(draft: QuantityDraft, label: string): QuantityValue | undefined {
  if (!draft.value.trim() && !draft.unit.trim()) return undefined;
  if (!draft.value.trim()) throw new Error(`${label} has a unit but no value; enter both or clear both.`);
  const parsed = parseQuantities({ value: draft.value.trim(), unit: draft.unit.trim() }, label) as QuantityValue;
  return { value: parsed.value, unit: parsed.unit };
}
function booleanFrom(state: TriState): boolean | undefined {
  return state === "true" ? true : state === "false" ? false : undefined;
}
/** Write `value`, or remove the member when it is undefined. An existing member keeps its place. */
function put(record: Json, key: string, value: unknown) {
  if (value === undefined) delete record[key];
  else record[key] = value;
}
const WEIGHT_MESSAGE = "Added weight needs both its magnitude and its direction; the direction is entered, never assumed.";

/** A new record: the entered members only (create). */
function newSet(draft: ConditionSetDraft): ConditionSet {
  const record: Json = { id: draft.id.trim() };
  if (draft.label.trim()) record.label = draft.label.trim();
  record.provenance = draft.provenance.trim();
  put(record, "installation_temperature", quantityFromDraft(draft.installationTemperature, "Installation temperature"));
  if (draft.conditions.length) record.conditions = draft.conditions.map((row, index) => newCondition(row, index));
  for (const [name, temperature, pressure] of [["design", draft.designTemperature, draft.designPressure], ["peak", draft.peakTemperature, draft.peakPressure]] as const) {
    const label = name === "design" ? "Design" : "Peak";
    const values: Json = {};
    put(values, "temperature", quantityFromDraft(temperature, `${label} temperature`));
    put(values, "pressure", quantityFromDraft(pressure, `${label} pressure`));
    if (Object.keys(values).length) record[name] = values;
  }
  const magnitude = quantityFromDraft(draft.addedWeightMagnitude, "Added weight magnitude");
  if (magnitude || draft.addedWeightDirection) {
    if (!magnitude || !draft.addedWeightDirection || draft.addedWeightDirection === "stored") throw new Error(WEIGHT_MESSAGE);
    record.added_weight = { magnitude, direction: draft.addedWeightDirection };
  }
  const exposure: Json = {};
  put(exposure, "wind", booleanFrom(draft.wind));
  put(exposure, "snow_ice", booleanFrom(draft.snowIce));
  put(exposure, "wave", booleanFrom(draft.wave));
  if (Object.keys(exposure).length) record.exposure = exposure;
  return record as ConditionSet;
}
function newCondition(row: ConditionDraft, index: number): OperatingCondition {
  const name = `Condition ${index + 1}`;
  const out: Json = { key: row.key.trim() };
  if (row.label.trim()) out.label = row.label.trim();
  put(out, "temperature", quantityFromDraft(row.temperature, `${name} temperature`));
  put(out, "pressure", quantityFromDraft(row.pressure, `${name} pressure`));
  put(out, "contents_density", quantityFromDraft(row.contentsDensity, `${name} contents density`));
  return out as OperatingCondition;
}

/** One stored condition with only the edited members changed. A row the form cannot read
 * (not an object) is written back as stored. */
function editedCondition(stored: unknown, row: ConditionDraft, base: ConditionDraft, index: number): unknown {
  if (!isObject(stored)) return clone(stored);
  const name = `Condition ${index + 1}`;
  const out = clone(stored);
  for (const member of row.removed) delete out[member];
  if (row.key !== base.key) out.key = row.key.trim();
  if (row.label !== base.label) put(out, "label", row.label.trim() || undefined);
  const quantities: Array<[string, keyof ConditionDraft, string]> = [["temperature", "temperature", "temperature"], ["pressure", "pressure", "pressure"], ["contents_density", "contentsDensity", "contents density"]];
  for (const [member, field, label] of quantities) {
    if (!same(row[field], base[field])) put(out, member, quantityFromDraft(row[field] as QuantityDraft, `${name} ${label}`));
  }
  return out;
}

/** The record the draft states. A create writes the entered members only. An update starts
 * from the stored record and changes only the members the person edited or removed; every
 * other member keeps its exact stored value and presence (empty strings and collections,
 * whitespace, `null` and members the form does not edit included). */
export function setFromDraft(draft: ConditionSetDraft): ConditionSet {
  if (!draft.base) return newSet(draft);
  const base = draft.base.draft;
  const out = clone(draft.base.record) as unknown as Json;
  for (const path of draft.removed) {
    const [head, tail] = path.split(".");
    if (tail === undefined) delete out[head];
    else if (isObject(out[head])) delete (out[head] as Json)[tail];
  }
  if (draft.label !== base.label) put(out, "label", draft.label.trim() || undefined);
  if (draft.provenance !== base.provenance) out.provenance = draft.provenance.trim();
  if (!same(draft.installationTemperature, base.installationTemperature)) put(out, "installation_temperature", quantityFromDraft(draft.installationTemperature, "Installation temperature"));

  const rows = Array.isArray(draft.base.record.conditions) ? draft.base.record.conditions as unknown[] : [];
  const rowsUntouched = draft.conditions.length === base.conditions.length && draft.conditions.every((row, index) => same(row, base.conditions[index]));
  if (!rowsUntouched) {
    out.conditions = draft.conditions.map((row, index) => row.origin === undefined
      ? newCondition(row, index)
      : editedCondition(rows[row.origin], row, base.conditions[row.origin], index));
  }

  for (const [name, temperature, pressure, baseTemperature, basePressure] of [
    ["design", draft.designTemperature, draft.designPressure, base.designTemperature, base.designPressure],
    ["peak", draft.peakTemperature, draft.peakPressure, base.peakTemperature, base.peakPressure]
  ] as const) {
    const label = name === "design" ? "Design" : "Peak";
    const editedTemperature = !same(temperature, baseTemperature), editedPressure = !same(pressure, basePressure);
    if (!editedTemperature && !editedPressure) continue;
    const values: Json = isObject(out[name]) ? out[name] as Json : {};
    if (editedTemperature) put(values, "temperature", quantityFromDraft(temperature, `${label} temperature`));
    if (editedPressure) put(values, "pressure", quantityFromDraft(pressure, `${label} pressure`));
    out[name] = values;
  }

  const editedMagnitude = !same(draft.addedWeightMagnitude, base.addedWeightMagnitude);
  const editedDirection = draft.addedWeightDirection !== base.addedWeightDirection;
  if (editedMagnitude || editedDirection) {
    const weight: Json = isObject(out.added_weight) ? out.added_weight as Json : {};
    if (editedMagnitude) put(weight, "magnitude", quantityFromDraft(draft.addedWeightMagnitude, "Added weight magnitude"));
    if (editedDirection) put(weight, "direction", draft.addedWeightDirection || undefined);
    if (Object.keys(weight).length === 0) delete out.added_weight;
    else if (!has(weight, "magnitude") || !has(weight, "direction")) throw new Error(WEIGHT_MESSAGE);
    else out.added_weight = weight;
  }

  const marks: Array<[string, TriState, TriState]> = [["wind", draft.wind, base.wind], ["snow_ice", draft.snowIce, base.snowIce], ["wave", draft.wave, base.wave]];
  if (marks.some(([, now, was]) => now !== was)) {
    const exposure: Json = isObject(out.exposure) ? out.exposure as Json : {};
    for (const [member, now, was] of marks) if (now !== was) put(exposure, member, booleanFrom(now));
    out.exposure = exposure;
  }
  return out as unknown as ConditionSet;
}

/** A stored value the form cannot show as an entry (for example `null`, a malformed quantity
 * or an unknown direction): the engine refuses the document by name. It is listed as stored,
 * written back unchanged, and removed only when the person removes it. */
export type RefusedMember = { path: string; value: unknown; row?: number; member?: string };
export function refusedMembers(set: ConditionSet): RefusedMember[] {
  const record = set as unknown as Json;
  const found: RefusedMember[] = [];
  const check = (path: string, present: boolean, value: unknown, ok: boolean) => { if (present && !ok) found.push({ path, value }); };
  check("label", has(record, "label"), record.label, typeof record.label === "string");
  check("provenance", has(record, "provenance"), record.provenance, typeof record.provenance === "string");
  check("installation_temperature", has(record, "installation_temperature"), record.installation_temperature, isQuantity(record.installation_temperature));
  check("conditions", has(record, "conditions"), record.conditions, Array.isArray(record.conditions));
  for (const [name, members] of NESTED_EDITED) {
    const nested = record[name];
    check(name, has(record, name), nested, isObject(nested));
    if (!isObject(nested)) continue;
    for (const member of members) {
      const value = nested[member];
      const ok = member === "direction" ? (CONDITION_DIRECTIONS as readonly unknown[]).includes(value)
        : name === "exposure" ? typeof value === "boolean" : isQuantity(value);
      check(`${name}.${member}`, has(nested, member), value, ok);
    }
  }
  (Array.isArray(record.conditions) ? record.conditions : []).forEach((condition: unknown, row: number) => {
    if (!isObject(condition)) { found.push({ path: `conditions[${row}]`, value: condition, row }); return; }
    for (const member of ["key", "label", "temperature", "pressure", "contents_density"]) {
      const value = condition[member];
      const ok = member === "key" || member === "label" ? typeof value === "string" : isQuantity(value);
      if (has(condition, member) && !ok) found.push({ path: `conditions[${row}].${member}`, value, row, member });
    }
  });
  return found;
}

/** Members the form does not edit, as `path: JSON` lines, for "Retained as authored". */
export function uneditedMembers(set: ConditionSet): string[] {
  const record = set as unknown as Json;
  const lines: string[] = [];
  for (const [key, value] of Object.entries(record)) if (!SET_EDITED.has(key)) lines.push(`${key}: ${JSON.stringify(value)}`);
  for (const [name, members] of NESTED_EDITED) {
    const nested = record[name];
    if (isObject(nested)) for (const [key, value] of Object.entries(nested)) if (!members.has(key)) lines.push(`${name}.${key}: ${JSON.stringify(value)}`);
  }
  (Array.isArray(record.conditions) ? record.conditions : []).forEach((condition: unknown, row: number) => {
    if (isObject(condition)) for (const [key, value] of Object.entries(condition)) if (!CONDITION_EDITED.has(key)) lines.push(`conditions[${row}].${key}: ${JSON.stringify(value)}`);
  });
  return lines;
}

function withSource(intent: EditorOperationIntent, note: string): EditorOperationIntent {
  return { ...intent, source: { ...CONDITION_SET_SOURCE }, change: { ...intent.change, source_note: note } };
}
const RECORD_NOTE = "User-entered operating-condition set; only the entered members are written and nothing is defaulted.";

export function createConditionSetIntent(record: ConditionSet): EditorOperationIntent {
  return withSource(modelDataIntent({ objectType: "ConditionSet", ref: record.id, changeKind: "create_condition_set", fieldPath: "condition_sets", before: NOT_PRESENT, after: JSON.stringify(record), label: "Add condition set" }), RECORD_NOTE);
}

export async function updateConditionSetIntent(canonicalize: Canonicalize, current: ConditionSet, next: ConditionSet): Promise<EditorOperationIntent> {
  return withSource(modelDataIntent({ objectType: "ConditionSet", ref: current.id, changeKind: "update_condition_set", fieldPath: "condition_sets", before: await canonicalize(current), after: JSON.stringify(next), label: "Edit condition set" }), RECORD_NOTE);
}

export async function deleteConditionSetIntent(canonicalize: Canonicalize, current: ConditionSet): Promise<EditorOperationIntent> {
  return withSource(modelDataIntent({ objectType: "ConditionSet", ref: current.id, changeKind: "delete_condition_set", fieldPath: "condition_sets", before: await canonicalize(current), after: NOT_PRESENT, label: "Remove condition set" }), RECORD_NOTE);
}

type Pipe = PreviewModel["pipe_segments"][number];
type LoadCase = PreviewModel["load_cases"][number];

/** Assign (`next` an id) or clear (`next` null) a pipe's condition set. */
export async function assignConditionSetIntent(canonicalize: Canonicalize, pipe: Pipe, next: string | null): Promise<EditorOperationIntent> {
  const before = pipe.condition_set_ref === undefined ? NOT_PRESENT : await canonicalize({ condition_set_ref: pipe.condition_set_ref });
  const after = next === null ? NOT_PRESENT : JSON.stringify({ condition_set_ref: next });
  return withSource(modelDataIntent({ objectType: "Element", ref: pipe.id, changeKind: "assign_condition_set", fieldPath: "condition_set_ref", before, after, label: next === null ? "Clear condition set" : "Assign condition set" }),
    "User-chosen condition set of this pipe; no set is suggested.");
}

/** Set (`next` the whole use) or clear (`next` null) a load case's condition use. */
export async function conditionUseIntent(canonicalize: Canonicalize, loadCase: LoadCase, next: ConditionUse | null): Promise<EditorOperationIntent> {
  const before = loadCase.condition_use === undefined ? NOT_PRESENT : await canonicalize(loadCase.condition_use);
  return withSource(modelDataIntent({
    objectType: "Load", ref: loadCase.id, changeKind: next === null ? "clear_load_case_condition_use" : "set_load_case_condition_use", fieldPath: "condition_use",
    before, after: next === null ? NOT_PRESENT : JSON.stringify(next), label: next === null ? "Clear condition use" : "Set condition use"
  }), "User-chosen condition use of this load case; absent members are not used and nothing is defaulted.");
}

/** Every pipe whose `condition_set_ref` names the set, in model order. */
export function pipesUsingSet(model: PreviewModel, setId: string): string[] {
  return model.pipe_segments.filter((pipe) => pipe.condition_set_ref === setId).map((pipe) => pipe.id);
}

/** The condition keys present in the model's sets, in first-appearance order. */
export function conditionKeys(model: PreviewModel): string[] {
  const keys: string[] = [];
  for (const set of model.condition_sets ?? []) for (const condition of set.conditions ?? []) {
    if (typeof condition.key === "string" && condition.key && !keys.includes(condition.key)) keys.push(condition.key);
  }
  return keys;
}

/** A document without `pressure_contract` is pressure-free (the applier's own test). */
export function isPressureFree(model: Pick<PreviewModel, "pressure_contract">): boolean {
  return model.pressure_contract === undefined || model.pressure_contract === null;
}
