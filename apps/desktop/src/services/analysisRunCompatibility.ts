import contract from "../../../../fixtures/results/semantic_contract_v0_2.json";
import type { AnalysisRunEnvelope, CanonicalResultDimension, MechanicsResult, ObjectRef } from "../types";
import { canonicalSha256HexCheckedV1 } from "./hashService";

export const ANALYSIS_RUN_V02 = "0.2.0";
export const CHECKED_PROFILE_V1 = "openpipestress_jcs_ijson_v1";
export const SEMANTIC_CONTRACT_SHA256 = "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da";
type ManifestEvidence = { manifest_ref: ObjectRef; manifest_sha256: string; manifest: { model_basis: { model_ref: string }; solver_basis: { solver_name: string; solver_version: string; solver_build_ref: string } } };
const provenance = { source_name: "OpenPipeStress analysis record 0.2", source_location: "analysis_run.compatibility.v0.2", source_license: "project-governed", review_status: "pending", professional_claim: false };

function semantics(row: MechanicsResult["results"][number]) {
  const byKind = contract.rows.filter((entry) => entry.kind === row.kind);
  if (!byKind.length) return { semantic: null, findings: ["SOURCE_SIGNATURE_UNKNOWN"] };
  const byUnit = byKind.filter((entry) => entry.unit === row.unit);
  if (!byUnit.length) throw new Error(`SOURCE_UNIT_CONTRADICTION: ${row.kind} / ${row.unit}`);
  const component = row.metadata?.component ?? null;
  const exact = byUnit.find((entry) => entry.component === component);
  if (exact) {
    if (row.dimension && exact.legacy_declared_dimension && row.dimension !== exact.legacy_declared_dimension) throw new Error(`ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: ${row.id}`);
    return { semantic: exact, findings: [] as string[] };
  }
  const generic = byUnit.find((entry) => entry.component === null);
  if (generic) {
    if (row.dimension && generic.legacy_declared_dimension && row.dimension !== generic.legacy_declared_dimension) throw new Error(`ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: ${row.id}`);
    return { semantic: generic, findings: component ? [] : ["OPTIONAL_SOURCE_METADATA_MISSING"] };
  }
  if (component) throw new Error(`SOURCE_COMPONENT_CONTRADICTION: ${row.kind} / ${component}`);
  return { semantic: null, findings: ["SOURCE_COMPONENT_MISSING_SEMANTICS_UNAVAILABLE"] };
}

export function analysisRecordProjection(record: AnalysisRunEnvelope): AnalysisRunEnvelope {
  const projected = structuredClone(record);
  const matches = projected.analysis_run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
  if (matches.length > 1) throw new Error("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE");
  projected.analysis_run.hashes = projected.analysis_run.hashes.filter((hash) => hash.payload_scope !== "analysis_run_record");
  return projected;
}

export async function buildAnalysisRunV02(result: MechanicsResult, inputManifest: ManifestEvidence, ruleCheckStatus?: string | null, loadBasisRefs: ObjectRef[] = []): Promise<AnalysisRunEnvelope> {
  if (inputManifest.manifest.model_basis.model_ref !== result.model_ref) throw new Error("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  const rawResult = structuredClone(result);
  const resultRefs = await Promise.all(rawResult.results.map(async (row, sourceRowIndex) => {
    const { semantic, findings } = semantics(row);
    return {
      result_ref: { object_type: "Result" as const, ref: row.id }, source_row_index: sourceRowIndex,
      category: semantic?.category ?? "unknown", source_dimension: (semantic?.source_physical_semantic_dimension ?? null) as CanonicalResultDimension | null,
      result_family: semantic?.family ?? null,
      semantic_contract: { id: "openpipestress_result_semantics_v0_2", sha256: SEMANTIC_CONTRACT_SHA256, signature_id: semantic?.signature_id ?? null },
      interpretation: { status: semantic?.canonical_disposition ?? "unavailable", findings },
      source_annotation: { kind: row.kind, unit: row.unit, metadata: structuredClone(row.metadata ?? null) },
      hash_refs: [{ algorithm: "sha256" as const, canonicalization: CHECKED_PROFILE_V1, payload_ref: { object_type: "Result" as const, ref: row.id }, payload_scope: "result_row", value: await canonicalSha256HexCheckedV1(row) }],
      privacy_classification: "source_evidence",
      provenance,
    };
  }));
  const effectiveRule = ruleCheckStatus ?? rawResult.status.rule_check;
  const effectiveLoadBasisRefs = loadBasisRefs.length ? loadBasisRefs : Array.from(new Map(rawResult.results.flatMap((row) => row.basis_ref ? [[`${row.basis_ref.ref_type}:${row.basis_ref.ref_id}`, { object_type: row.basis_ref.ref_type === "combination" ? "Combination" : "LoadCase", ref: row.basis_ref.ref_id } as ObjectRef]] : [])).values());
  const statuses = Array.from(new Set(["HUMAN_REVIEW_REQUIRED", rawResult.status.mechanics, effectiveRule].filter(Boolean))).sort();
  const runRef = { object_type: "AnalysisRun", ref: rawResult.run_id } as ObjectRef;
  const record: AnalysisRunEnvelope = {
    schema_version: ANALYSIS_RUN_V02, deliverable_id: "DEL-14-02", package_id: "PKG-14", scope_item: "SOW-072", objectives: ["OBJ-016"],
    run_contract_status: { record_contract: "strict_analysis_run_v0_2", result_binding: "received_mechanics_result", external_validation_boundary: "reference_only_not_determined_by_software" },
    analysis_run: {
      run_id: rawResult.run_id, run_name: `${rawResult.run_id} analysis record`, run_kind: "mechanics_solve", created_at: null, model_state_ref: { object_type: "ModelState", ref: `state:${rawResult.model_ref}:preview` },
      solver_version: { solver_name: inputManifest.manifest.solver_basis.solver_name, solver_version: inputManifest.manifest.solver_basis.solver_version, build_ref: { object_type: "ExternalReference", ref: inputManifest.manifest.solver_basis.solver_build_ref } },
      settings_ref: { object_type: "SolverSettings", ref: `solver-settings:${inputManifest.manifest_ref.ref}:${inputManifest.manifest_sha256}` }, unit_system_ref: { object_type: "UnitSystem", ref: `unit-system:${rawResult.model_ref}:${inputManifest.manifest_sha256}` }, load_basis_refs: effectiveLoadBasisRefs,
      diagnostics: rawResult.diagnostics.map((item) => ({ source_annotation: structuredClone(item) })), rule_pack_refs: [], library_refs: [],
      result_refs: resultRefs, hashes: [{ algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: { object_type: "ResultEnvelope", ref: `result-envelope:${rawResult.run_id}` }, payload_scope: "received_result", value: await canonicalSha256HexCheckedV1(rawResult) }], analysis_status: statuses,
      reproducibility: { input_manifest_refs: [structuredClone(inputManifest.manifest_ref)], input_manifest_hashes: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: structuredClone(inputManifest.manifest_ref), payload_scope: "input_manifest", value: inputManifest.manifest_sha256 }], semantic_contract: { id: "openpipestress_result_semantics_v0_2", sha256: SEMANTIC_CONTRACT_SHA256 }, determinism_notes: ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"], unresolved_tbd: [] },
      immutability_policy: { run_record_is_read_only: true, mutation_policy: "changes_create_new_immutable_record_revision", new_mechanics_run_required_for_record_revision: false, record_revision_identity: "analysis_run_record_sha256", hash_invalidates_external_acceptance: true },
      professional_boundary: {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
        software_makes_authentication_claim: false
      },
      provenance,
    },
  };
  record.analysis_run.hashes.unshift({ algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: runRef, payload_scope: "analysis_run_record", value: await canonicalSha256HexCheckedV1(analysisRecordProjection(record)) });
  return record;
}

export async function verifyAnalysisRunRecord(record: AnalysisRunEnvelope): Promise<"match" | "mismatch" | "unverifiable"> {
  if (record.schema_version === "0.1.0") return "unverifiable";
  if (record.schema_version !== ANALYSIS_RUN_V02) throw new Error(`ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: ${record.schema_version}`);
  const hashes = record.analysis_run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
  if (hashes.length !== 1) return "unverifiable";
  const claim = hashes[0];
  if (claim.algorithm !== "sha256" || claim.canonicalization !== CHECKED_PROFILE_V1 || claim.payload_ref.object_type !== "AnalysisRun" || claim.payload_ref.ref !== record.analysis_run.run_id) return "mismatch";
  return claim.value === await canonicalSha256HexCheckedV1(analysisRecordProjection(record)) ? "match" : "mismatch";
}
