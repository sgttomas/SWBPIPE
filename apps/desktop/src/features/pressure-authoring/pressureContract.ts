import type { PreviewModel } from "../../types";

/** The current pressure contract: all new pressure authoring writes it. */
export const PRESSURE_V3 = Object.freeze({ version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" as const });

type ContractModel = Pick<PreviewModel, "schema_version" | "pressure_contract">;

/** A document declaring `3.0.0/exact_pressure_v3`. */
export function isPressureV3(model: ContractModel): boolean {
  return model.pressure_contract?.version === PRESSURE_V3.version && model.pressure_contract.mode === PRESSURE_V3.mode;
}

/** A current document on the exact pressure route.
 * Materials there derive G from the selected E/nu pair. */
export function isExactPressureContract(model: ContractModel): boolean {
  if (model.schema_version !== "0.5.0" || !isPressureV3(model)) return false;
  const contract = model.pressure_contract;
  return !!contract && Object.keys(contract).length === 3
    && (contract.shear_model === "euler_bernoulli" || contract.shear_model === "timoshenko_annulus_energy_v1");
}

/** The current profile; local user section inputs are never manufactured. */
export function pressureProfileTarget(_model: ContractModel) {
  return { schema_version: "0.5.0", pressure_contract: { ...PRESSURE_V3 } };
}
