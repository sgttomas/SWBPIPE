import type { PreviewModel } from "../../types";

/** The current pressure contract: all new pressure authoring writes it. */
export const PRESSURE_V3 = Object.freeze({ version: "3.0.0", mode: "exact_pressure_v3" });

type ContractModel = Pick<PreviewModel, "schema_version" | "pressure_contract">;

/** A document declaring `3.0.0/exact_pressure_v3`. */
export function isPressureV3(model: ContractModel): boolean {
  return model.pressure_contract?.version === PRESSURE_V3.version && model.pressure_contract.mode === PRESSURE_V3.mode;
}

/** A document on the exact pressure route (model 0.3.0 or 0.4.0): v3, or an
 * existing `2.0.0/exact_straight_pressure_v2` document, which stays solvable.
 * Materials there derive G from the selected E/nu pair. */
export function isExactPressureContract(model: ContractModel): boolean {
  if (model.schema_version !== "0.3.0" && model.schema_version !== "0.4.0") return false;
  return isPressureV3(model) || (model.pressure_contract?.version === "2.0.0" && model.pressure_contract.mode === "exact_straight_pressure_v2");
}

/** The profile the pressure-profile operation writes: v3, keeping a 0.4.0
 * document's version and writing 0.3.0 for every other document. */
export function pressureProfileTarget(model: ContractModel) {
  return { schema_version: model.schema_version === "0.4.0" ? "0.4.0" : "0.3.0", pressure_contract: { ...PRESSURE_V3 } };
}
