/** The hanger-sizing request the desktop assembles (REQ-001, AC-012): every
 * input is the person's own entry. The form starts with no choice made ([`EMPTY_HANGER_SIZING_FORM`]); nothing here
 * supplies a value. An incomplete or invalid form names each missing or invalid input and assembles nothing; the
 * native entry refuses again by name whatever reaches it. */

import type { PreviewModel } from "../../types";
import type { HangerSizingFamilyPolicy, HangerSizingRequest } from "../results/hangerSizingResult";
import type { HangerSizingInput } from "../../services/hangerSizingService";

/** The only admitted `weight_support_state`: shown as a fixed fact, not a choice. */
export const WEIGHT_SUPPORT_STATE = "own_solve" as const;

export type HangerSizingFormState = Readonly<{
  libraryId: string;
  /** Candidate supports in the order chosen, each with its family policy ("" until chosen). */
  candidates: readonly Readonly<{ support: string; policy: HangerSizingFamilyPolicy | "" }>[];
  weightCase: string;
  designOperatingCase: string;
  checkOperatingCases: readonly string[];
  /** The person stated that no further operating case is checked (§2: may be empty, but must be stated). */
  noCheckOperatingCases: boolean;
  loadDesign: "" | "hot" | "cold";
  variationReference: "" | "operating_load" | "design_load";
  /** As entered: a positive finite ratio. */
  variationLimit: string;
  /** As entered, in metres: a finite length >= 0. */
  travelAllowanceM: string;
  selectionRule: "" | "smallest_capacity" | "least_variation";
  solverMode: "" | "sparse_interactive" | "dense_scrutiny";
}>;

/** A new project's form: no library, no candidate, no case and no method choice. */
export const EMPTY_HANGER_SIZING_FORM: HangerSizingFormState = {
  libraryId: "",
  candidates: [],
  weightCase: "",
  designOperatingCase: "",
  checkOperatingCases: [],
  noCheckOperatingCases: false,
  loadDesign: "",
  variationReference: "",
  variationLimit: "",
  travelAllowanceM: "",
  selectionRule: "",
  solverMode: ""
};

export type AssembledHangerSizing =
  | Readonly<{ ok: true; input: HangerSizingInput }>
  | Readonly<{ ok: false; missing: readonly string[] }>;

/** The solve request a sizing request carries for `model`: the desktop solve's request, `materials: []`. The result
 * binds its hash, so the same assembly tells whether a result is still bound to the present model. */
export function hangerSizingSolveRequest(model: PreviewModel): HangerSizingInput["request"] {
  return { model, materials: [] };
}

/** A decimal number as typed, or null: no blank, sign-only or trailing text is read as a number. */
function enteredNumber(text: string): number | null {
  const trimmed = text.trim();
  if (!/^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$/.test(trimmed)) return null;
  const value = Number(trimmed);
  return Number.isFinite(value) ? value : null;
}

/** The headless input for `form`, the session's `model` and the opened `library` document, or every missing or
 * invalid input by name. `materials: []` matches the desktop solve's request. */
export function assembleHangerSizingInput(form: HangerSizingFormState, model: PreviewModel | null, library: Record<string, unknown> | null): AssembledHangerSizing {
  const missing: string[] = [];
  if (!model) missing.push("model: no model is open");
  if (!form.libraryId || !library) missing.push("library: choose a stored hanger library");
  if (form.candidates.length === 0) missing.push("candidates: choose at least one support");
  for (const candidate of form.candidates) {
    if (!candidate.policy) missing.push(`family policy of ${candidate.support}: choose variable, constant or variable else constant`);
  }
  if (!form.weightCase) missing.push("weight case: choose a load case");
  if (!form.designOperatingCase) missing.push("design operating case: choose a load case");
  if (form.checkOperatingCases.length === 0 && !form.noCheckOperatingCases) {
    missing.push("check operating cases: choose them, or state that there are none");
  }
  if (!form.loadDesign) missing.push("load design: choose hot or cold");
  if (!form.variationReference) missing.push("variation reference: choose the operating load or the design load");
  const variationLimit = enteredNumber(form.variationLimit);
  if (form.variationLimit.trim() === "") missing.push("variation limit: enter a ratio");
  else if (variationLimit === null || variationLimit <= 0) missing.push("variation limit: enter a positive finite ratio");
  const travelAllowance = enteredNumber(form.travelAllowanceM);
  if (form.travelAllowanceM.trim() === "") missing.push("travel allowance: enter a length in metres");
  else if (travelAllowance === null || travelAllowance < 0) missing.push("travel allowance: enter a finite length of 0 m or more");
  if (!form.selectionRule) missing.push("selection rule: choose smallest capacity or least variation");
  if (!form.solverMode) missing.push("solver mode: choose sparse interactive or dense scrutiny");
  if (missing.length > 0 || !model || !library || variationLimit === null || travelAllowance === null) return { ok: false, missing };
  const sizing: HangerSizingRequest = {
    weight_case: form.weightCase,
    weight_support_state: WEIGHT_SUPPORT_STATE,
    design_operating_case: form.designOperatingCase,
    check_operating_cases: form.noCheckOperatingCases ? [] : [...form.checkOperatingCases],
    candidates: form.candidates.map((candidate) => ({ support: candidate.support, family_policy: candidate.policy as HangerSizingFamilyPolicy })),
    method: {
      load_design: form.loadDesign as "hot" | "cold",
      variation_reference: form.variationReference as "operating_load" | "design_load",
      variation_limit: variationLimit,
      travel_allowance_m: travelAllowance,
      selection_rule: form.selectionRule as "smallest_capacity" | "least_variation"
    },
    solver_mode: form.solverMode as "sparse_interactive" | "dense_scrutiny"
  };
  return { ok: true, input: { request: hangerSizingSolveRequest(model), sizing, library } };
}
