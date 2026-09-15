import { useState } from "react";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, ModelHashEvidence, ProjectEnvelopeHashEvidence } from "../../types";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
import { bindSourceResultDimensions } from "../../services/previewService";
import { ResultsPanel } from "./ResultsPanel";

// Transient evidence only. Saved run references do not contain the historical
// input-manifest payload, so reopening cannot establish a current solve basis.
export type HistoricalRunContext = {
  designation: "historical_saved_run";
  mechanicsResult: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  modelHash: ModelHashEvidence | null;
  envelopeHash: ProjectEnvelopeHashEvidence | null;
  envelopePayloadHash: string | null;
  findings: string[];
  runId: string;
};

function legacyDesktopJson(value: unknown): string {
  function sort(item: any): any {
    if (Array.isArray(item)) return item.map(sort);
    if (item && typeof item === "object") return Object.fromEntries(Object.entries(item).sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, sort(child)]));
    return item;
  }
  return JSON.stringify(sort(value));
}

async function legacyDesktopDigest(value: unknown): Promise<string> {
  const bytes = new TextEncoder().encode(legacyDesktopJson(value));
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function legacyPythonJson(value: unknown): string {
  function sort(item: any): any {
    if (Array.isArray(item)) return item.map(sort);
    if (item && typeof item === "object") return Object.fromEntries(Object.entries(item).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([key, child]) => [key, sort(child)]));
    return item;
  }
  return JSON.stringify(sort(value)).replace(/[^\x00-\x7f]/g, (character) => [...character].map((unit) => `\\u${unit.charCodeAt(0).toString(16).padStart(4, "0")}`).join(""));
}

async function legacyPythonDigest(value: unknown): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(legacyPythonJson(value)));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

type LegacyDesktopVerification = { result: "match" | "mismatch" | "unverifiable"; record: "match" | "mismatch" | "unverifiable" };

function exactKeys(value: unknown, keys: string[]): boolean {
  return Boolean(value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === keys.slice().sort().join("\0"));
}

function isExactLegacyDesktopEnvelope(record: AnalysisRunEnvelope): boolean {
  const run: any = record.analysis_run, contract: any = record.run_contract_status;
  const boundary = run?.professional_boundary, immutable = run?.immutability_policy, reproducibility = run?.reproducibility;
  const isRef = (value: any) => exactKeys(value, ["object_type", "ref"]) && typeof value.object_type === "string" && value.object_type.length > 0 && typeof value.ref === "string" && value.ref.length > 0;
  const isHash = (value: any, scope?: string) => exactKeys(value, ["algorithm", "canonicalization", "payload_ref", "payload_scope", "value"]) && value.algorithm === "sha256" && value.canonicalization === "rfc8785_jcs" && isRef(value.payload_ref) && (!scope || value.payload_scope === scope) && /^[0-9a-f]{64}$/.test(value.value);
  const dimensions = ["dimensionless", "length", "angle", "force", "moment", "stress", "area", "section_modulus", "second_moment_area", "ratio", "time", "temperature", "pressure", "linear_stiffness", "rotational_stiffness"];
  return exactKeys(record, ["schema_version", "deliverable_id", "package_id", "scope_item", "objectives", "run_contract_status", "analysis_run"])
    && record.schema_version === "0.1.0" && record.deliverable_id === "DEL-14-02" && record.package_id === "PKG-14" && record.scope_item === "SOW-072"
    && JSON.stringify(record.objectives) === '["OBJ-016"]'
    && exactKeys(contract, ["record_contract", "model_state_binding", "result_binding", "physical_project_container", "external_validation_boundary"])
    && contract.record_contract === "schema_first_analysis_run_records" && contract.model_state_binding === "schemas/model_state.schema.json" && contract.result_binding === "schemas/results.schema.yaml" && contract.physical_project_container === "TBD" && contract.external_validation_boundary === "reference_only_not_determined_by_software"
    && exactKeys(run, ["run_id", "run_name", "run_kind", "model_state_ref", "load_basis_refs", "result_refs", "hashes", "analysis_status", "reproducibility", "immutability_policy", "professional_boundary"])
    && run.run_kind === "mechanics_solve" && typeof run.run_id === "string" && run.run_id.length > 0 && typeof run.run_name === "string" && run.run_name.length > 0
    && isRef(run.model_state_ref) && Array.isArray(run.load_basis_refs) && run.load_basis_refs.every(isRef)
    && Array.isArray(run.result_refs) && run.result_refs.every((item: any) => exactKeys(item, ["result_ref", "result_family", "source_dimension", "hash_refs", "privacy_classification"]) && isRef(item.result_ref) && typeof item.result_family === "string" && item.result_family.length > 0 && dimensions.includes(item.source_dimension) && Array.isArray(item.hash_refs) && item.hash_refs.length === 1 && isHash(item.hash_refs[0], "result_value") && item.privacy_classification === "invented_public_example")
    && Array.isArray(run.hashes) && run.hashes.length === 2 && run.hashes.every((item: any) => isHash(item)) && new Set(run.hashes.map((item: any) => item.payload_scope)).size === 2 && run.hashes.some((item: any) => item.payload_scope === "analysis_run_record") && run.hashes.some((item: any) => item.payload_scope === "result_envelope") && Array.isArray(run.analysis_status)
    && run.analysis_status.length >= 2 && new Set(run.analysis_status).size === run.analysis_status.length && run.analysis_status.includes("HUMAN_REVIEW_REQUIRED") && run.analysis_status.every((value: unknown) => typeof value === "string" && ["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"].includes(value))
    && exactKeys(reproducibility, ["input_manifest_refs", "input_manifest_hashes", "determinism_notes", "unresolved_tbd"])
    && Array.isArray(reproducibility.input_manifest_refs) && reproducibility.input_manifest_refs.length === 1 && reproducibility.input_manifest_refs.every(isRef) && Array.isArray(reproducibility.input_manifest_hashes) && reproducibility.input_manifest_hashes.length === 1 && isHash(reproducibility.input_manifest_hashes[0], "input_manifest") && Array.isArray(reproducibility.determinism_notes) && reproducibility.determinism_notes.every((item: unknown) => typeof item === "string") && Array.isArray(reproducibility.unresolved_tbd) && reproducibility.unresolved_tbd.every((item: unknown) => typeof item === "string")
    && exactKeys(immutable, ["run_record_is_read_only", "mutation_policy", "new_run_required_for_change", "hash_invalidates_external_acceptance"])
    && immutable.run_record_is_read_only === true && immutable.mutation_policy === "changes_create_new_analysis_run" && immutable.new_run_required_for_change === true && immutable.hash_invalidates_external_acceptance === true
    && exactKeys(boundary, ["human_review_required", "software_makes_compliance_claim", "software_makes_certification_claim", "software_makes_sealing_claim", "software_makes_approval_claim", "software_makes_authentication_claim"])
    && boundary.human_review_required === true && boundary.software_makes_compliance_claim === false && boundary.software_makes_certification_claim === false && boundary.software_makes_sealing_claim === false && boundary.software_makes_approval_claim === false && boundary.software_makes_authentication_claim === false;
}

function isExactLegacyPythonEnvelope(record: AnalysisRunEnvelope): boolean {
  const run: any = record.analysis_run, contract: any = record.run_contract_status;
  const boundary = run?.professional_boundary, immutable = run?.immutability_policy, reproducibility = run?.reproducibility;
  const fullRunKeys = ["run_id", "run_name", "run_kind", "created_at", "model_state_ref", "solver_version", "settings_ref", "unit_system_ref", "load_basis_refs", "diagnostics", "result_refs", "rule_pack_refs", "library_refs", "hashes", "analysis_status", "reproducibility", "immutability_policy", "professional_boundary", "provenance"];
  const provenanceKeys = ["source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status", "review_status", "privacy_classification"];
  const objectTypes = ["Project", "Model", "ModelState", "AnalysisRun", "SolverSettings", "UnitSystem", "LoadCase", "LoadCombination", "Result", "ResultEnvelope", "InputManifest", "RulePack", "Library", "AuditManifest", "Diagnostic", "ExternalReference", "TBD"];
  const isRef = (value: any) => (exactKeys(value, ["object_type", "ref"]) || exactKeys(value, ["object_type", "ref", "label"])) && objectTypes.includes(value.object_type) && typeof value.ref === "string" && value.ref.length > 0 && (!("label" in value) || typeof value.label === "string");
  const isProvenance = (value: any) => exactKeys(value, provenanceKeys) && provenanceKeys.every((key) => typeof value[key] === "string" && value[key].length > 0);
  const isHash = (value: any, scopes: string[]) => exactKeys(value, ["algorithm", "canonicalization", "payload_ref", "payload_scope", "value"]) && value.algorithm === "sha256" && value.canonicalization === "SORTED_COMPACT_JSON" && isRef(value.payload_ref) && scopes.includes(value.payload_scope) && typeof value.value === "string" && value.value.length > 0;
  const physical = contract?.physical_project_container;
  const dimensions = ["dimensionless", "length", "angle", "force", "moment", "stress", "area", "section_modulus", "second_moment_area", "ratio", "time", "temperature", "pressure", "linear_stiffness", "rotational_stiffness"];
  const statuses = ["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED", "TBD"];
  return exactKeys(record, ["schema_version", "deliverable_id", "package_id", "scope_item", "objectives", "run_contract_status", "analysis_run"])
    && record.schema_version === "0.1.0" && record.deliverable_id === "DEL-14-02" && record.package_id === "PKG-14" && record.scope_item === "SOW-072" && JSON.stringify(record.objectives) === '["OBJ-016"]'
    && exactKeys(contract, ["record_contract", "model_state_binding", "result_binding", "physical_project_container", "external_validation_boundary"]) && contract.record_contract === "schema_first_analysis_run_records" && contract.model_state_binding === "schemas/model_state.schema.json" && contract.result_binding === "schemas/results.schema.yaml" && contract.external_validation_boundary === "reference_only_not_determined_by_software"
    && exactKeys(physical, ["profile", "decision_ref", "storage_role", "canonical_truth", "sql_public_contract", "direct_sql_access_allowed", "hosted_db_allowed", "network_required", "sidecars_rebuildable"]) && physical.profile === "sqlite_local_project_store" && physical.decision_ref === "SCA-003" && physical.storage_role === "local_store_index_projection" && ["sorted_compact_json_payload", "canonical_json_jcs_payload"].includes(physical.canonical_truth) && physical.sql_public_contract === false && physical.direct_sql_access_allowed === false && physical.hosted_db_allowed === false && physical.network_required === false && physical.sidecars_rebuildable === true
    && exactKeys(run, fullRunKeys) && typeof run.run_id === "string" && run.run_id.length > 0 && typeof run.run_name === "string" && run.run_name.length > 0 && typeof run.created_at === "string" && run.created_at.length > 0 && run.run_kind === "mechanics_solve"
    && isRef(run.model_state_ref) && isRef(run.settings_ref) && isRef(run.unit_system_ref) && Array.isArray(run.load_basis_refs) && run.load_basis_refs.every(isRef)
    && Array.isArray(run.diagnostics) && run.diagnostics.every((item: any) => exactKeys(item, ["code", "class", "severity", "source", "affected_object", "message", "remediation", "provenance"]) && typeof item.code === "string" && typeof item.class === "string" && ["info", "warning", "blocking"].includes(item.severity) && isRef(item.source) && isRef(item.affected_object) && typeof item.message === "string" && item.message.length > 0 && typeof item.remediation === "string" && item.remediation.length > 0 && isProvenance(item.provenance))
    && Array.isArray(run.result_refs) && run.result_refs.every((item: any) => exactKeys(item, ["result_ref", "result_family", "source_dimension", "hash_refs", "privacy_classification", "provenance"]) && isRef(item.result_ref) && typeof item.result_family === "string" && item.result_family.length > 0 && dimensions.includes(item.source_dimension) && Array.isArray(item.hash_refs) && item.hash_refs.length === 1 && isHash(item.hash_refs[0], ["result_value"]) && typeof item.privacy_classification === "string" && isProvenance(item.provenance))
    && Array.isArray(run.rule_pack_refs) && run.rule_pack_refs.length === 0 && Array.isArray(run.library_refs) && run.library_refs.length === 0
    && Array.isArray(run.hashes) && run.hashes.length === 2 && run.hashes.every((item: any) => isHash(item, ["analysis_run_record", "result_envelope"])) && run.hashes.some((item: any) => item.payload_scope === "analysis_run_record") && run.hashes.some((item: any) => item.payload_scope === "result_envelope")
    && Array.isArray(run.analysis_status) && run.analysis_status.includes("HUMAN_REVIEW_REQUIRED") && run.analysis_status.every((item: unknown) => typeof item === "string" && statuses.includes(item))
    && exactKeys(run.solver_version, ["solver_name", "solver_version", "build_ref", "provenance"]) && typeof run.solver_version.solver_name === "string" && run.solver_version.solver_name.length > 0 && typeof run.solver_version.solver_version === "string" && run.solver_version.solver_version.length > 0 && isRef(run.solver_version.build_ref) && isProvenance(run.solver_version.provenance) && isProvenance(run.provenance)
    && exactKeys(reproducibility, ["input_manifest_refs", "input_manifest_hashes", "environment_refs", "determinism_notes", "unresolved_tbd"]) && Array.isArray(reproducibility.input_manifest_refs) && reproducibility.input_manifest_refs.every(isRef) && Array.isArray(reproducibility.input_manifest_hashes) && reproducibility.input_manifest_hashes.every((item: any) => exactKeys(item, ["algorithm", "canonicalization", "payload_ref", "payload_scope", "value"]) && item.algorithm === "sha256" && item.canonicalization === "JCS" && isRef(item.payload_ref) && item.payload_scope === "input_manifest" && /^[0-9a-f]{64}$/.test(item.value)) && Array.isArray(reproducibility.environment_refs) && reproducibility.environment_refs.every(isRef) && Array.isArray(reproducibility.determinism_notes) && reproducibility.determinism_notes.every((item: unknown) => typeof item === "string") && Array.isArray(reproducibility.unresolved_tbd) && reproducibility.unresolved_tbd.every((item: unknown) => typeof item === "string")
    && exactKeys(immutable, ["run_record_is_read_only", "mutation_policy", "new_run_required_for_change", "hash_invalidates_external_acceptance"]) && immutable.run_record_is_read_only === true && immutable.mutation_policy === "changes_create_new_analysis_run" && immutable.new_run_required_for_change === true && immutable.hash_invalidates_external_acceptance === true
    && exactKeys(boundary, ["human_review_required", "software_makes_compliance_claim", "software_makes_certification_claim", "software_makes_sealing_claim", "software_makes_approval_claim", "software_makes_authentication_claim"]) && boundary.human_review_required === true && boundary.software_makes_compliance_claim === false && boundary.software_makes_certification_claim === false && boundary.software_makes_sealing_claim === false && boundary.software_makes_approval_claim === false && boundary.software_makes_authentication_claim === false;
}

async function verifyLegacyDesktopAnalysis(record: AnalysisRunEnvelope, received: MechanicsResult): Promise<LegacyDesktopVerification> {
  try {
    const run = record.analysis_run;
    const canonicalizations = [
      ...(run.hashes ?? []),
      ...(run.result_refs ?? []).flatMap((item) => item.hash_refs ?? []),
    ].map((claim) => claim.canonicalization);
    const isDesktopProfile =
      canonicalizations.length > 0 &&
      canonicalizations.every((label) => label === "rfc8785_jcs") &&
      !("created_at" in run) && !("solver_version" in run) && !("provenance" in run) && isExactLegacyDesktopEnvelope(record);
    const isPythonProfile =
      canonicalizations.length > 0 &&
      canonicalizations.every((label) => label === "SORTED_COMPACT_JSON") &&
      "created_at" in run && "solver_version" in run && "provenance" in run && isExactLegacyPythonEnvelope(record);
    // The full Python 0.1 producer used sorted compact Python JSON. Once its
    // numbers have crossed JSON into JavaScript, integer-vs-float spelling
    // (1 versus 1.0) is irrecoverable, so it must not be replayed as desktop.
    if (isPythonProfile) {
      const recordClaim = run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
      const manifestRefs = run.reproducibility?.input_manifest_refs ?? [];
      const manifestHashes = run.reproducibility?.input_manifest_hashes ?? [];
      if (recordClaim.length !== 1 || manifestRefs.length !== 1 || manifestHashes.length !== 1 || run.result_refs.length !== received.results.length) return { result: "unverifiable", record: "unverifiable" };
      const claim = recordClaim[0], manifestClaim = manifestHashes[0];
      if (claim.algorithm !== "sha256" || claim.canonicalization !== "SORTED_COMPACT_JSON" || claim.payload_scope !== "analysis_run_record" || claim.payload_ref?.object_type !== "AnalysisRun" || claim.payload_ref?.ref !== run.run_id || !/^[0-9a-f]{64}$/.test(claim.value)) return { result: "unverifiable", record: "mismatch" };
      if (manifestClaim.payload_ref?.ref !== manifestRefs[0].ref || manifestClaim.payload_scope !== "input_manifest" || typeof manifestClaim.value !== "string") return { result: "unverifiable", record: "unverifiable" };
      const sortedReceived = received.results.slice().sort((left, right) => left.id < right.id ? -1 : left.id > right.id ? 1 : 0);
      const sortedRefs = run.result_refs.slice().sort((left, right) => left.result_ref.ref < right.result_ref.ref ? -1 : 1);
      if (sortedReceived.some((row, index) => row.id !== sortedRefs[index]?.result_ref.ref)) return { result: "unverifiable", record: "unverifiable" };
      const payload = {
        run_id: received.run_id, model_ref: received.model_ref, status: received.status,
        result_ids: sortedReceived.map((row) => row.id),
        result_dimensions: sortedRefs.map((item) => ({ result_id: item.result_ref.ref, source_dimension: item.source_dimension })),
        diagnostic_ids: received.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort(),
        input_manifest_ref: manifestRefs[0], input_manifest_hash: manifestClaim.value,
      };
      return { result: "unverifiable", record: claim.value === await legacyPythonDigest(payload) ? "match" : "mismatch" };
    }
    if (!isDesktopProfile) return { result: "unverifiable", record: "unverifiable" };
    const enriched = bindSourceResultDimensions(received);
    const resultClaim = run.hashes.filter((hash) => hash.payload_scope === "result_envelope");
    const recordClaim = run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
    const manifestRefs = run.reproducibility?.input_manifest_refs ?? [];
    const manifestHashes = run.reproducibility?.input_manifest_hashes ?? [];
    const ruleStatus = run.analysis_status.find((status) => ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"].includes(status));
    let resultStatus: LegacyDesktopVerification["result"] = "match";
    let recordStatus: LegacyDesktopVerification["record"] = "match";
    const validClaim = (claim: any, scope: string, type: string, ref: string) => claim.algorithm === "sha256" && claim.canonicalization === "rfc8785_jcs" && claim.payload_scope === scope && claim.payload_ref?.object_type === type && claim.payload_ref?.ref === ref && /^[0-9a-f]{64}$/.test(claim.value);
    if (resultClaim.length !== 1 || run.result_refs.length !== enriched.results.length) resultStatus = "unverifiable";
    else if (!validClaim(resultClaim[0], "result_envelope", "ResultEnvelope", `result-envelope:${received.run_id}`) || resultClaim[0].value !== await legacyDesktopDigest(enriched)) resultStatus = "unverifiable";
    if (resultStatus === "match") {
      const sortedRows = enriched.results.slice().sort((left, right) => left.id.localeCompare(right.id));
      for (const [index, row] of sortedRows.entries()) {
        const ref = run.result_refs[index];
        const claim = ref?.hash_refs?.[0];
        if (!claim || ref.result_ref?.ref !== row.id || !validClaim(claim, "result_value", "Result", row.id) || claim.value !== await legacyDesktopDigest(row)) { resultStatus = "unverifiable"; break; }
      }
    }
    if (recordClaim.length !== 1 || manifestRefs.length !== 1 || manifestHashes.length !== 1 || !ruleStatus) return { result: resultStatus, record: "unverifiable" };
    if (!validClaim(recordClaim[0], "analysis_run_record", "AnalysisRun", run.run_id)) recordStatus = "mismatch";
    const manifestClaim = manifestHashes[0];
    if (manifestClaim.payload_ref?.ref !== manifestRefs[0].ref || manifestClaim.payload_scope !== "input_manifest" || typeof manifestClaim.value !== "string") return { result: resultStatus, record: "unverifiable" };
    const payload = {
      run_id: received.run_id,
      model_ref: received.model_ref,
      status: { ...received.status, rule_check: ruleStatus },
      load_basis_refs: run.load_basis_refs,
      result_ids: enriched.results.map((row) => row.id).sort(),
      diagnostic_ids: enriched.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort(),
      input_manifest_ref: manifestRefs[0],
      input_manifest_sha256: manifestClaim.value,
      result_dimensions: enriched.results.map((row) => ({ result_id: row.id, dimension: row.dimension })).sort((left, right) => left.result_id.localeCompare(right.result_id)),
    };
    if (recordStatus === "match" && recordClaim[0].value !== await legacyDesktopDigest(payload)) recordStatus = "unverifiable";
    return { result: resultStatus, record: recordStatus };
  } catch {
    return { result: "unverifiable", record: "unverifiable" };
  }
}

export async function buildHistoricalRunContext(opened: LocalProjectEnvelope): Promise<HistoricalRunContext | null> {
  const mechanicsResult = opened.mechanics_result ?? null;
  const analysisRun = opened.analysis_run ?? null;
  if (!mechanicsResult && !analysisRun) return null;
  const findings = ["HISTORICAL_INPUT_MANIFEST_MISSING"];
  const record = analysisRun && typeof analysisRun === "object" && analysisRun.analysis_run && typeof analysisRun.analysis_run === "object" ? analysisRun.analysis_run : null;
  if (analysisRun && (!record || !Array.isArray(record.hashes) || !Array.isArray(record.result_refs) || typeof record.run_id !== "string" || typeof record.model_state_ref?.ref !== "string")) findings.push("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
  if (mechanicsResult && mechanicsResult.model_ref !== opened.model.project.id) findings.push("HISTORICAL_MODEL_REF_MISMATCH");
  const expectedStateRef = `state:${opened.model.project.id}:preview`;
  if (record && record.model_state_ref?.ref !== expectedStateRef) findings.push("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  if (!mechanicsResult || !record) findings.push("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
  if (mechanicsResult && record && mechanicsResult.run_id !== record.run_id) findings.push("HISTORICAL_RUN_REF_MISMATCH");
  const modelHash = opened.model_hash ?? null;
  if (!modelHash) findings.push("HISTORICAL_MODEL_HASH_MISSING");
  if (!opened.project_envelope_hash) findings.push("HISTORICAL_ENVELOPE_HASH_MISSING");
  let envelopePayloadHash: string | null = null;
  let legacyVerification: LegacyDesktopVerification | null = null;
  try {
    const recomputedModel = await computeModelHash(opened.model);
    if (modelHash && (modelHash.value !== recomputedModel?.value || modelHash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_MODEL_HASH_MISMATCH");
    const recomputedEnvelope = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: opened.editor_intents ?? [], proposal: opened.proposal ?? null, selected_review_target: opened.selected_review_target ?? null, mechanics_result: mechanicsResult, analysis_run: analysisRun, model_hash: modelHash });
    envelopePayloadHash = recomputedEnvelope?.value ?? null;
    if (opened.project_envelope_hash && (opened.project_envelope_hash.value !== recomputedEnvelope?.value || opened.project_envelope_hash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    if (mechanicsResult) {
      const receivedScope = analysisRun?.schema_version === "0.2.0" ? "received_result" : "result_envelope";
      const storedResultHash = Array.isArray(record?.hashes) ? record.hashes.find((hash) => hash && typeof hash === "object" && hash.payload_scope === receivedScope) : null;
      if (!storedResultHash) findings.push("HISTORICAL_RESULT_HASH_MISSING");
      else if (analysisRun?.schema_version === "0.2.0" && ((typeof storedResultHash.value === "string" ? storedResultHash.value.replace(/^sha256:/, "") : null) !== await canonicalSha256HexCheckedV1(mechanicsResult) || storedResultHash.payload_ref?.ref !== `result-envelope:${mechanicsResult.run_id}`)) findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
      else if (analysisRun?.schema_version === "0.1.0") {
        legacyVerification = await verifyLegacyDesktopAnalysis(analysisRun, mechanicsResult);
        if (legacyVerification.result === "mismatch") findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
        if (legacyVerification.result === "unverifiable") findings.push("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
      }
    }
    if (analysisRun?.schema_version === "0.2.0") {
      const verification = await verifyAnalysisRunRecord(analysisRun);
      if (verification === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (verification === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE");
    } else if (analysisRun?.schema_version === "0.1.0" && mechanicsResult) {
      legacyVerification ??= await verifyLegacyDesktopAnalysis(analysisRun, mechanicsResult);
      if (legacyVerification.record === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (legacyVerification.record === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    }
  } catch {
    findings.push("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
  }
  return { designation: "historical_saved_run", mechanicsResult, analysisRun, modelHash, envelopeHash: opened.project_envelope_hash ?? null, envelopePayloadHash, findings, runId: mechanicsResult?.run_id ?? (typeof record?.run_id === "string" ? record.run_id : "unknown saved run") };
}

export function HistoricalRunPanel({ context }: { context: HistoricalRunContext }) {
  const [selectedResultId, setSelectedResultId] = useState<string | null>(null);
  return <section className="panel" aria-label="Historical saved run" data-testid="historical-run-context">
    <div className="panel-title">Historical saved run</div>
    <p>Saved evidence for {context.runId}; mechanics={context.mechanicsResult?.status.mechanics ?? "missing"}. Run a fresh solve to establish current results.</p>
    <p>Historical results cannot drive current overlays, rule checks, comparisons or report readiness.</p>
    <ul>{context.findings.map((finding) => <li key={finding}>{finding}</li>)}</ul>
    <ResultsPanel result={context.mechanicsResult} knowledge={null} analysisRun={null} selectedResultId={selectedResultId} onSelectResult={setSelectedResultId} />
  </section>;
}
