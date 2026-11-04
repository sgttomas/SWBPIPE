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
import { modelDataIntent, type Canonicalize } from "../model-data/modelDataIntents";
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
export type TriState = "" | "true" | "false";
export type ConditionDraft = {
  key: string;
  label: string;
  temperature: QuantityDraft;
  pressure: QuantityDraft;
  contentsDensity: QuantityDraft;
  /** Members of the stored condition the form does not edit, carried as authored. */
  retained: Record<string, unknown>;
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
  addedWeightDirection: "" | ConditionDirection;
  wind: TriState;
  snowIce: TriState;
  wave: TriState;
  /** Members of the stored set the form does not edit (tags, notes, any other), carried as authored. */
  retained: Record<string, unknown>;
};

const SET_EDITED = new Set(["id", "label", "provenance", "installation_temperature", "conditions", "design", "peak", "added_weight", "exposure"]);
const CONDITION_EDITED = new Set(["key", "label", "temperature", "pressure", "contents_density"]);
const VALUES_EDITED = new Set(["temperature", "pressure"]);
const EXPOSURE_EDITED = new Set(["wind", "snow_ice", "wave"]);
const WEIGHT_EDITED = new Set(["magnitude", "direction"]);

const emptyQuantity = (): QuantityDraft => ({ value: "", unit: "" });
export function quantityDraft(quantity: QuantityValue | undefined): QuantityDraft {
  return quantity ? { value: String(quantity.value), unit: quantity.unit } : emptyQuantity();
}
function triState(value: unknown): TriState {
  return value === true ? "true" : value === false ? "false" : "";
}
function retainedMembers(record: object | undefined, edited: ReadonlySet<string>): Record<string, unknown> {
  return Object.fromEntries(Object.entries(record ?? {}).filter(([key]) => !edited.has(key)));
}

export function emptyConditionDraft(): ConditionDraft {
  return { key: "", label: "", temperature: emptyQuantity(), pressure: emptyQuantity(), contentsDensity: emptyQuantity(), retained: {} };
}

/** A blank draft: no member is prefilled. */
export function emptyConditionSetDraft(): ConditionSetDraft {
  return {
    id: "", label: "", provenance: "", installationTemperature: emptyQuantity(), conditions: [],
    designTemperature: emptyQuantity(), designPressure: emptyQuantity(), peakTemperature: emptyQuantity(), peakPressure: emptyQuantity(),
    addedWeightMagnitude: emptyQuantity(), addedWeightDirection: "", wind: "", snowIce: "", wave: "", retained: {}
  };
}

/** The draft of a stored set: every value as stored, every other member retained. */
export function draftFromSet(set: ConditionSet): ConditionSetDraft {
  const weightDirection = set.added_weight?.direction;
  const retained = retainedMembers(set, SET_EDITED);
  // Sub-records' unedited members are retained under their parent's name.
  const nested: Array<[string, object | undefined, ReadonlySet<string>]> = [
    ["design", set.design, VALUES_EDITED], ["peak", set.peak, VALUES_EDITED],
    ["exposure", set.exposure, EXPOSURE_EDITED], ["added_weight", set.added_weight, WEIGHT_EDITED]
  ];
  for (const [name, record, edited] of nested) {
    const extra = retainedMembers(record, edited);
    if (Object.keys(extra).length) retained[`${name}.`] = extra;
  }
  return {
    id: set.id,
    label: set.label ?? "",
    provenance: set.provenance ?? "",
    installationTemperature: quantityDraft(set.installation_temperature),
    conditions: (set.conditions ?? []).map((condition) => ({
      key: condition.key ?? "",
      label: condition.label ?? "",
      temperature: quantityDraft(condition.temperature),
      pressure: quantityDraft(condition.pressure),
      contentsDensity: quantityDraft(condition.contents_density),
      retained: retainedMembers(condition, CONDITION_EDITED)
    })),
    designTemperature: quantityDraft(set.design?.temperature),
    designPressure: quantityDraft(set.design?.pressure),
    peakTemperature: quantityDraft(set.peak?.temperature),
    peakPressure: quantityDraft(set.peak?.pressure),
    addedWeightMagnitude: quantityDraft(set.added_weight?.magnitude),
    addedWeightDirection: weightDirection && (CONDITION_DIRECTIONS as readonly string[]).includes(weightDirection) ? weightDirection : "",
    wind: triState(set.exposure?.wind),
    snowIce: triState(set.exposure?.snow_ice),
    wave: triState(set.exposure?.wave),
    retained
  };
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
function compact<T extends object>(record: T): T | undefined {
  const entries = Object.entries(record).filter(([, value]) => value !== undefined);
  return entries.length ? (Object.fromEntries(entries) as T) : undefined;
}

/** The record the draft states: entered members only, plus every retained member. */
export function setFromDraft(draft: ConditionSetDraft): ConditionSet {
  const nested = (name: string) => (draft.retained[`${name}.`] as Record<string, unknown> | undefined) ?? {};
  const record: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(draft.retained)) if (!key.endsWith(".")) record[key] = value;
  record.id = draft.id.trim();
  if (draft.label.trim()) record.label = draft.label.trim();
  record.provenance = draft.provenance.trim();
  const installation = quantityFromDraft(draft.installationTemperature, "Installation temperature");
  if (installation) record.installation_temperature = installation;
  if (draft.conditions.length) {
    record.conditions = draft.conditions.map((condition, index): OperatingCondition => {
      const name = `Condition ${index + 1}`;
      const out: Record<string, unknown> = { ...condition.retained, key: condition.key.trim() };
      if (condition.label.trim()) out.label = condition.label.trim();
      const temperature = quantityFromDraft(condition.temperature, `${name} temperature`);
      const pressure = quantityFromDraft(condition.pressure, `${name} pressure`);
      const contents = quantityFromDraft(condition.contentsDensity, `${name} contents density`);
      if (temperature) out.temperature = temperature;
      if (pressure) out.pressure = pressure;
      if (contents) out.contents_density = contents;
      return out as OperatingCondition;
    });
  }
  const design = compact({ ...nested("design"), temperature: quantityFromDraft(draft.designTemperature, "Design temperature"), pressure: quantityFromDraft(draft.designPressure, "Design pressure") });
  if (design) record.design = design;
  const peak = compact({ ...nested("peak"), temperature: quantityFromDraft(draft.peakTemperature, "Peak temperature"), pressure: quantityFromDraft(draft.peakPressure, "Peak pressure") });
  if (peak) record.peak = peak;
  const magnitude = quantityFromDraft(draft.addedWeightMagnitude, "Added weight magnitude");
  if (magnitude || draft.addedWeightDirection) {
    if (!magnitude || !draft.addedWeightDirection) throw new Error("Added weight needs both its magnitude and its direction; the direction is entered, never assumed.");
    record.added_weight = { ...nested("added_weight"), magnitude, direction: draft.addedWeightDirection };
  }
  const exposure = compact({ ...nested("exposure"), wind: booleanFrom(draft.wind), snow_ice: booleanFrom(draft.snowIce), wave: booleanFrom(draft.wave) });
  if (exposure) record.exposure = exposure;
  return record as ConditionSet;
}

/** Retained members as `name: JSON` lines, for the "Retained as authored" display. */
export function retainedLines(retained: Record<string, unknown>): string[] {
  return Object.entries(retained).map(([key, value]) => `${key.endsWith(".") ? key.slice(0, -1) + " (other members)" : key}: ${JSON.stringify(value)}`);
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
