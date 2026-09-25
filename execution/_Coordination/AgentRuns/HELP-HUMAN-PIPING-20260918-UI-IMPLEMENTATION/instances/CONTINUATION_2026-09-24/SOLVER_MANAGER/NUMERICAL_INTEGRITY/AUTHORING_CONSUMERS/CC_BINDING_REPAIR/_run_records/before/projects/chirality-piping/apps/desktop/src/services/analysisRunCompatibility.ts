import { semanticContractForSource } from "../features/results/resultSemantics";
import { sourceContract, PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256 } from "../features/results/numericalResultQuality";
import type { AnalysisRunEnvelope, CanonicalResultDimension, MechanicsResult, ObjectRef } from "../types";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";

export const ANALYSIS_RUN_V02 = "0.2.0";
export const ANALYSIS_RUN_V03 = "0.3.0";
export const CHECKED_PROFILE_V1 = "openpipestress_jcs_ijson_v1";
export const SEMANTIC_CONTRACT_SHA256 = "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da";
type ManifestEvidence = { manifest_ref: ObjectRef; manifest_sha256: string; manifest: { model_basis: { model_ref: string }; solver_basis: { solver_name: string; solver_version: string; solver_build_ref: string } } };
const provenance = { source_name: "OpenPipeStress analysis record 0.2", source_location: "analysis_run.compatibility.v0.2", source_license: "project-governed", review_status: "pending", professional_claim: false };

function semantics(row: MechanicsResult["results"][number], source?: MechanicsResult) {
  const contract = semanticContractForSource(source);
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

/** Explicit historical builder; never a fallback for precision admission. */
export async function buildAnalysisRunV02(result: MechanicsResult, inputManifest: ManifestEvidence, ruleCheckStatus?: string | null, loadBasisRefs: ObjectRef[] = []): Promise<AnalysisRunEnvelope> {
  if (!["0.1.0", "0.2.0"].includes(result.schema_version) || ["producer", "numerical_quality", "formulation_basis"].some(key => Object.hasOwn(result, key))) throw new Error("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
  return buildAnalysisRecord(result, inputManifest, "legacy", ruleCheckStatus, loadBasisRefs);
}
export async function buildAnalysisRunV03(result: MechanicsResult, inputManifest: ManifestEvidence, ruleCheckStatus?: string | null, loadBasisRefs: ObjectRef[] = []): Promise<AnalysisRunEnvelope> {
  if (sourceContract(result) !== "precision") throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return buildAnalysisRecord(result, inputManifest, "precision", ruleCheckStatus, loadBasisRefs);
}
async function buildAnalysisRecord(result: MechanicsResult, inputManifest: ManifestEvidence, route: "legacy" | "precision", ruleCheckStatus?: string | null, loadBasisRefs: ObjectRef[] = []): Promise<AnalysisRunEnvelope> {
  if (inputManifest.manifest.model_basis.model_ref !== result.model_ref) throw new Error("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  const semanticBinding = route === "precision" ? { id: PRECISION_CONTRACT_ID, sha256: PRECISION_CONTRACT_SHA256 } : { id: "openpipestress_result_semantics_v0_2", sha256: SEMANTIC_CONTRACT_SHA256 };
  const rawResult = structuredClone(result);
  const resultRefs = await Promise.all(rawResult.results.map(async (row, sourceRowIndex) => {
    const { semantic, findings } = semantics(row, route === "precision" ? result : undefined);
    return {
      result_ref: { object_type: "Result" as const, ref: row.id }, source_row_index: sourceRowIndex,
      category: semantic?.category ?? "unknown", source_dimension: (semantic?.source_physical_semantic_dimension ?? null) as CanonicalResultDimension | null,
      result_family: semantic?.family ?? null,
      semantic_contract: { ...semanticBinding, signature_id: semantic?.signature_id ?? null },
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
    schema_version: route === "precision" ? ANALYSIS_RUN_V03 : ANALYSIS_RUN_V02, deliverable_id: "DEL-14-02", package_id: "PKG-14", scope_item: "SOW-072", objectives: ["OBJ-016"],
    run_contract_status: { record_contract: route === "precision" ? "strict_analysis_run_v0_3" : "strict_analysis_run_v0_2", result_binding: "received_mechanics_result", external_validation_boundary: "reference_only_not_determined_by_software" },
    analysis_run: {
      run_id: rawResult.run_id, run_name: `${rawResult.run_id} analysis record`, run_kind: "mechanics_solve", created_at: null, model_state_ref: { object_type: "ModelState", ref: `state:${rawResult.model_ref}:preview` },
      solver_version: { solver_name: inputManifest.manifest.solver_basis.solver_name, solver_version: inputManifest.manifest.solver_basis.solver_version, build_ref: { object_type: "ExternalReference", ref: inputManifest.manifest.solver_basis.solver_build_ref } },
      settings_ref: { object_type: "SolverSettings", ref: `solver-settings:${inputManifest.manifest_ref.ref}:${inputManifest.manifest_sha256}` }, unit_system_ref: { object_type: "UnitSystem", ref: `unit-system:${rawResult.model_ref}:${inputManifest.manifest_sha256}` }, load_basis_refs: effectiveLoadBasisRefs,
      diagnostics: rawResult.diagnostics.map((item) => ({ source_annotation: structuredClone(item) })), rule_pack_refs: [], library_refs: [],
      result_refs: resultRefs, hashes: [{ algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: { object_type: "ResultEnvelope", ref: `result-envelope:${rawResult.run_id}` }, payload_scope: "received_result", value: await canonicalSha256HexCheckedV1(rawResult) }], analysis_status: statuses,
      reproducibility: { input_manifest_refs: [structuredClone(inputManifest.manifest_ref)], input_manifest_hashes: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: structuredClone(inputManifest.manifest_ref), payload_scope: "input_manifest", value: inputManifest.manifest_sha256 }], semantic_contract: semanticBinding, determinism_notes: ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"], unresolved_tbd: [] },
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
  if (route === "precision") await validateAnalysisRunV03(record, result);
  return record;
}

export async function verifyAnalysisRunRecord(record: AnalysisRunEnvelope): Promise<"match" | "mismatch" | "unverifiable"> {
  if (record.schema_version === "0.1.0") return "unverifiable";
  if (record.schema_version !== ANALYSIS_RUN_V02 && record.schema_version !== ANALYSIS_RUN_V03) throw new Error(`ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: ${record.schema_version}`);
  const hashes = record.analysis_run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
  if (hashes.length !== 1) return "unverifiable";
  const claim = hashes[0];
  if (claim.algorithm !== "sha256" || claim.canonicalization !== CHECKED_PROFILE_V1 || claim.payload_ref.object_type !== "AnalysisRun" || claim.payload_ref.ref !== record.analysis_run.run_id) return "mismatch";
  return claim.value === await canonicalSha256HexCheckedV1(analysisRecordProjection(record)) ? "match" : "mismatch";
}

/** Source interpretation validation is distinct from checksum verification and producer authentication. */
export async function validateAnalysisRunV03(record: AnalysisRunEnvelope, source: MechanicsResult): Promise<void> {
  const same = (a: unknown, b: unknown): boolean => {
    if (a === undefined || b === undefined) return a === b;
    const order = (v: any): any => Array.isArray(v) ? v.map(order) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, order(v[k])])) : v;
    return JSON.stringify(order(JSON.parse(checkedJsonText(a)))) === JSON.stringify(order(JSON.parse(checkedJsonText(b))));
  };
  if (sourceContract(source) !== "precision" || record.schema_version !== ANALYSIS_RUN_V03 || record.run_contract_status.record_contract !== "strict_analysis_run_v0_3") throw new Error("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
  const run = record.analysis_run;
  if (run.solver_version?.solver_name !== source.producer!.component_name || run.solver_version?.solver_version !== source.producer!.component_version) throw new Error("ANALYSIS_SOURCE_PRODUCER_MISMATCH");
  if (run.run_id !== source.run_id) throw new Error("ANALYSIS_SOURCE_RUN_MISMATCH");
  if (!same(run.model_state_ref, {object_type:"ModelState",ref:`state:${source.model_ref}:preview`})) throw new Error("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
  const contract = {id: PRECISION_CONTRACT_ID, sha256: PRECISION_CONTRACT_SHA256};
  if (!same(run.reproducibility.semantic_contract, contract)) throw new Error("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
  const checksum = async (scope: string, ref: ObjectRef, value: unknown) => ({algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: ref, payload_scope: scope, value: await canonicalSha256HexCheckedV1(value)});
  if (!same(run.hashes.filter(h => h.payload_scope === "received_result"), [await checksum("received_result", {object_type:"ResultEnvelope",ref:`result-envelope:${source.run_id}`}, source)])) throw new Error("ANALYSIS_RECEIVED_SOURCE_MISMATCH");
  if (source.results.length !== run.result_refs.length || new Set(source.results.map(r => r.id)).size !== source.results.length) throw new Error("ANALYSIS_ROW_ACCOUNTING_MISMATCH");
  for (const [index, row] of source.results.entries()) {
    const actual = run.result_refs[index], {semantic, findings} = semantics(row, source), ref = {object_type:"Result",ref:row.id};
    if (actual.source_row_index !== index || !same(actual.result_ref, ref) || !same(actual.semantic_contract, {...contract,signature_id:semantic?.signature_id ?? null})
      || !same(actual.hash_refs, [await checksum("result_row",ref,row)])) throw new Error("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH");
    if (actual.category !== (semantic?.category ?? "unknown") || actual.source_dimension !== (semantic?.source_physical_semantic_dimension ?? null) || actual.result_family !== (semantic?.family ?? null)
      || !same(actual.interpretation, {status:semantic?.canonical_disposition ?? "unavailable",findings})
      || !same(actual.source_annotation, {kind:row.kind,unit:row.unit,metadata:row.metadata ?? null})) throw new Error("ANALYSIS_ROW_INTERPRETATION_MISMATCH");
  }
  if (await verifyAnalysisRunRecord(record) !== "match") throw new Error("ANALYSIS_RECORD_CHECKSUM_MISMATCH");
}
