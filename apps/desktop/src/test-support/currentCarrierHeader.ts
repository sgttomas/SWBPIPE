import current from "../../../../fixtures/results/preview_physics_invented_sparse.json";
import type { MechanicsResult } from "../types";

/** b7-plan E2: renderer tests that classify synthetic rows need a readable header, since
 * the retired producer-less 0.1.0 carrier has no table. This is the actual preview-physics-2
 * header (version, producer, quality, formulation and evidence) of a committed producer
 * output; the header dispatch reads only these members, so the rows a test supplies are
 * classified by the preview-physics-2 table and never validated as that output's rows. */
export function currentCarrierHeader(): Pick<MechanicsResult, "schema_version" | "document_kind" | "producer" | "numerical_quality" | "formulation_basis" | "contract_evidence"> {
  const header = structuredClone(current) as unknown as MechanicsResult;
  return {
    schema_version: header.schema_version, document_kind: header.document_kind, producer: header.producer,
    numerical_quality: header.numerical_quality, formulation_basis: header.formulation_basis, contract_evidence: header.contract_evidence,
  };
}
