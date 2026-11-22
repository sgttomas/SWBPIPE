import type { PreviewModel } from "../../types";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";

/** The two admitted straight-member shear models of the exact contract (T4-U8, D-6/O3). */
export type ShearModel = "euler_bernoulli" | "timoshenko_annulus_energy_v1";
/** From T4-U8 on, authoring writes Timoshenko by default; Euler-Bernoulli remains an
 * explicit choice. Realized arcs are Euler-Bernoulli under either model. */
export const DEFAULT_SHEAR_MODEL: ShearModel = "timoshenko_annulus_energy_v1";
/** The label and choices of shear deformation in straight members, in the order offered. The
 * Model settings control, this panel's selector and its status line all read these. */
export const SHEAR_MODEL_LABEL = "Shear deformation in straight members";
export const SHEAR_MODEL_CHOICES: readonly { value: ShearModel; label: string }[] = Object.freeze([
  { value: "timoshenko_annulus_energy_v1", label: "Included (Timoshenko)" },
  { value: "euler_bernoulli", label: "Not included (Euler–Bernoulli)" },
]);
const lowerFirst = (text: string) => text.charAt(0).toLowerCase() + text.slice(1);
/** A status line's clause for a selection, in the control's own words: "shear deformation in
 * straight members is included (Timoshenko)" or "... is not included (Euler–Bernoulli)". */
export function shearModelClause(model: ShearModel): string {
  return `${lowerFirst(SHEAR_MODEL_LABEL)} is ${lowerFirst(SHEAR_MODEL_CHOICES.find(choice => choice.value === model)!.label)}`;
}

/** The current pressure contract: all new pressure authoring writes it, with the
 * default shear model unless the document or the user selects another. */
export const PRESSURE_V3 = Object.freeze({ version: "3.0.0", mode: "exact_pressure_v3", shear_model: DEFAULT_SHEAR_MODEL });

type ContractModel = Pick<PreviewModel, "schema_version" | "pressure_contract" | "model_settings">;

export function isShearModel(value: unknown): value is ShearModel {
  return value === "euler_bernoulli" || value === "timoshenko_annulus_energy_v1";
}

/** A document declaring `3.0.0/exact_pressure_v3`. */
export function isPressureV3(model: ContractModel): boolean {
  return model.pressure_contract?.version === PRESSURE_V3.version && model.pressure_contract.mode === PRESSURE_V3.mode;
}

/** A current document on the exact pressure route.
 * Materials there derive G from the selected E/nu pair. */
export function isExactPressureContract(model: ContractModel): boolean {
  if (model.schema_version !== MODEL_SCHEMA_VERSION || !isPressureV3(model)) return false;
  const contract = model.pressure_contract;
  return !!contract && Object.keys(contract).length === 3 && isShearModel(contract.shear_model);
}

/** The shear model a document's v3 contract already selects, or null when it has none. */
export function documentShearModel(model: ContractModel): ShearModel | null {
  const selected = isPressureV3(model) ? model.pressure_contract?.shear_model : undefined;
  return isShearModel(selected) ? selected : null;
}

/** The selection pressure activation carries: the v3 contract's, else the pressure-free
 * route's `model_settings.shear_model`, which the operation engine moves into the contract
 * (and out of the model settings) on activation; null when the document states neither. */
export function carriedShearModel(model: ContractModel): ShearModel | null {
  const free = model.model_settings?.shear_model;
  return documentShearModel(model) ?? (isShearModel(free) ? free : null);
}

/** The current profile; local user section inputs are never manufactured. The shear
 * model is the user's explicit choice when given, else the document's existing
 * selection (never rewritten), else the default. */
export function pressureProfileTarget(model: ContractModel, selected?: ShearModel) {
  const shear_model = selected ?? carriedShearModel(model) ?? DEFAULT_SHEAR_MODEL;
  return { schema_version: MODEL_SCHEMA_VERSION, pressure_contract: { version: PRESSURE_V3.version, mode: PRESSURE_V3.mode, shear_model } };
}
