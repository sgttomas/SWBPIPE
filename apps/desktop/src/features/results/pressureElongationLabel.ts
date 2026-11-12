/** RT-L2 (DEL-04-07): each load case's result
 * header states whether the case includes pressure elongation, from the case's own pressure
 * evidence (`contract_evidence.pressure[]`, one record per pressure region of the case, each
 * with `pressure_elongation: "included" | "not_included"`). A combination states nothing of its
 * own (its rows are formed from its cases' rows), and a case without pressure regions has no
 * pressure evidence to state it from.
 *
 * Pure presentation: the reader has already checked that the regions of a case agree and match
 * their coefficients; anything else here states nothing rather than a guess. */

import type { ResultBasisRef } from "../../types";

export const PRESSURE_ELONGATION_INCLUDED_LABEL = "Pressure elongation included";
export const PRESSURE_ELONGATION_NOT_INCLUDED_LABEL = "Pressure elongation not included";

const LABELS: Readonly<Record<string, string>> = Object.freeze({
  included: PRESSURE_ELONGATION_INCLUDED_LABEL,
  not_included: PRESSURE_ELONGATION_NOT_INCLUDED_LABEL
});

/** The label for the load case `basis` names, or null: a combination or other basis, a case
 * with no pressure region, or regions that do not state one agreed value. */
export function pressureElongationLabel(contractEvidence: unknown, basis: ResultBasisRef | null): string | null {
  if (!basis || basis.ref_type !== "load_case") return null;
  const regions = contractEvidence && typeof contractEvidence === "object"
    ? (contractEvidence as Record<string, unknown>).pressure
    : undefined;
  if (!Array.isArray(regions)) return null;
  const stated = new Set<unknown>();
  for (const region of regions) {
    if (region && typeof region === "object" && (region as Record<string, unknown>).load_case_id === basis.ref_id) {
      stated.add((region as Record<string, unknown>).pressure_elongation);
    }
  }
  if (stated.size !== 1) return null;
  const [value] = [...stated];
  return typeof value === "string" && Object.hasOwn(LABELS, value) ? LABELS[value] : null;
}
