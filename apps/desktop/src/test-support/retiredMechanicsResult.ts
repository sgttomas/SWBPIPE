import current from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import type { MechanicsResult } from "../types";

/** b7-plan E2: the retired producer-less 0.1.0 mechanics-result carrier, for refusal
 * tests only. No such carrier is committed; this one is the bundled demo's current
 * output with its producer header removed and the retired version label. */
export function retiredMechanicsResult(): MechanicsResult {
  const result = structuredClone(current) as unknown as Record<string, unknown>;
  result.schema_version = "0.1.0";
  for (const key of ["producer", "numerical_quality", "formulation_basis", "contract_evidence"]) delete result[key];
  return result as unknown as MechanicsResult;
}
