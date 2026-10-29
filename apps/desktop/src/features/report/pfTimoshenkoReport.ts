/** The pressure-free Timoshenko report projection (DEL-05-03 Design `pf-timoshenko.md` §4,
 * "Source binding on PF"). A report of a `preview-physics-timoshenko-1` result carries the
 * identity's limitation 0 as one entry after the technical-preview entry, and the result's
 * diagnostics (among them `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI`) as its first
 * Diagnostics entries (`diagnosticsForSections`). A rendered report also carries the
 * top-level `pf_timoshenko` section: the identity and the Export Result JSON admission
 * binding, which the native render binds to the delivered result before it renders. The
 * Rust mirror is `report_package::pf_timoshenko`; Euler–Bernoulli reports take neither
 * path, so their bytes are unchanged. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import type { MechanicsResult } from "../../types";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import timoshenkoTable from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
import type { ExactAdmissionBinding } from "./exactReportAdmission";

export const PF_TIMOSHENKO_SECTION = "pf_timoshenko";
export const REALIZED_ARCS_DIAGNOSTIC = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";
export const REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH = "REPORT-PACKAGE-PF-TIMOSHENKO-SECTIONS-MISMATCH";

/** Whether `result` names the pressure-free Timoshenko identity. */
export function isPfTimoshenkoReportSource(result: MechanicsResult | null | undefined): boolean {
  return result?.producer?.semantic_contract_id === PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID;
}

/** The identity's limitation 0 entry for `run`, without the `effect` and `provenance` it
 * shares with the technical-preview entry: the source's published limitation 0, which must
 * be the table's (`report_package::pf_timoshenko::expected_limitation`). */
export function pfTimoshenkoLimitationEntry(result: MechanicsResult, run: string) {
  const published = result.formulation_basis?.limitations?.[0];
  const stated = timoshenkoTable.supported_profile_limitations[0];
  if (!isPfTimoshenkoReportSource(result) || published !== stated) {
    throw new Error(`${REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH}: the source's limitation 0 is not the identity's`);
  }
  return {
    limitation_id: `limitation:${run}:pf-timoshenko:1`,
    source: { ref_type: "semantic_contract", ref_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID },
    affected_scope: { ref_type: "analysis_run", ref_id: run },
    statement: stated
  };
}

/** The rendered report's section: the identity and its admission binding. */
export function pfTimoshenkoSection(binding: Readonly<ExactAdmissionBinding>) {
  return { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, admission: { ...binding } };
}
