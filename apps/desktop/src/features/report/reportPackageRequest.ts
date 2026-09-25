import { sourceContract } from "../results/numericalResultQuality";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { analysisResultHashScope } from "../results/analysisResultHashScope";
import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewComparison,
  PreviewModel
} from "../../types";
import { canonicalSha256Hex } from "../../services/hashService";
import {
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence
} from "../../services/inputManifestService";
import { resultSemantics, completeSourceMetadata } from "../results/resultSemantics";

import { buildRenderableReportInput } from "./renderableReportInput";
import { buildStateComparisonHandoffSections } from "./stateComparisonHandoffSections";

const UNIT_SYSTEM_REF = "unit-system:dec-018-si-dual-display";
const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false
};
const SHA256_HEX = /^[0-9a-f]{64}$/;

function reference(objectType: string, ref: string) {
  return { ref_type: objectType, ref_id: ref };
}

function privateProvenance(model: PreviewModel) {
  return {
    source_name: `OpenPipeStress desktop session (${model.project.name})`,
    source_location: "local desktop session",
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: "not_asserted",
    redistribution_status: "private_only",
    review_status: "pending"
  };
}

function safeId(value: string): string {
  return value.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "report-package";
}

function reportMetadata(item: MechanicsResult["results"][number]) {
  if (!completeSourceMetadata(item)) return null;
  const md = item.metadata!;
  // The unchanged report transport accepts the legacy metadata vocabulary.
  const component = ["axial_force", "shear_force_y", "shear_force_z", "torsional_moment", "bending_moment_y", "bending_moment_z", "nodal_force_x", "nodal_force_y", "nodal_force_z", "nodal_moment_x", "nodal_moment_y", "nodal_moment_z", "axial_normal_stress", "bending_normal_stress_y", "bending_normal_stress_z", "torsional_shear_stress", "pressure_hoop_stress", "pressure_longitudinal_stress", "section_area", "section_modulus_y", "section_modulus_z", "torsion_constant", "torsion_radius", "TBD"];
  const coordinate_system = ["global", "element_local", "pipe_section", "TBD"];
  const location = ["end_i", "end_j", "node", "quarter_1", "midspan", "quarter_3", "summary", "TBD"];
  const basis = ["recovered_from_local_element_stiffness", "assembled_solver_load_vector", "solved_from_global_linear_system", "recovered_from_open_mechanics_stress_components", "interpolated_from_endpoint_resultants", "derived_from_user_entered_section_geometry", "explicit_user_linear_combination", "explicit_user_result_state_subtraction", "explicit_user_range_envelope", "stress_recovery_summary", "rule_pack_evaluation", "TBD"];
  if (!component.includes(md.component) || !coordinate_system.includes(md.coordinate_system) || !location.includes(md.location) || !basis.includes(md.basis)) return null;
  return {component:md.component, coordinate_system:md.coordinate_system,location:md.location,basis:md.basis,sign_convention:md.sign_convention};
}

function diagnosticClass(severity: string): string {
  return severity === "blocking" || severity === "error" ? "solve_blocking" : "assumption_warning";
}

function checksum(payloadRef: { ref_type: string; ref_id: string }, value: string) {
  return { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: payloadRef, value };
}

function requireSha256(label: string, value: string): void {
  if (!SHA256_HEX.test(value)) {
    throw new Error(
      `REPORT-PACKAGE-SHA256-INVALID: ${label} must be bare lowercase 64-hex.`
    );
  }
}

// Called only after the manifest bytes and the run's manifest binding are checked.
// Source producer identity and recorded build identity are different evidence.
function reportSolverIdentity(result: MechanicsResult, analysisRun: AnalysisRunEnvelope, inputManifest: CurrentSessionInputManifestEvidence) {
  const manifestSolver = inputManifest.manifest.solver_basis;
  const recordedSolver = analysisRun.analysis_run.solver_version;
  const precision = analysisRun.schema_version === "0.3.0";
  if (!precision && (!(result.schema_version === "0.1.0" || result.schema_version === "0.2.0") ||
    ["producer", "numerical_quality", "formulation_basis"].some(key => Object.hasOwn(result, key)))) {
    throw new Error("REPORT-PACKAGE-SOURCE-CONTRACT-MISMATCH");
  }
  // Legacy 0.1 records may lack a solver-version field; the verified manifest
  // still supplies recorded identity. Strict records must carry both records.
  if ((!recordedSolver && analysisRun.schema_version !== "0.1.0") ||
    (recordedSolver && (recordedSolver.solver_name !== manifestSolver.solver_name ||
      recordedSolver.solver_version !== manifestSolver.solver_version ||
      recordedSolver.build_ref?.ref !== manifestSolver.solver_build_ref))) {
    throw new Error("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH: analysis record and verified manifest differ.");
  }
  if (precision && (sourceContract(result) !== "precision" ||
    result.producer!.component_name !== manifestSolver.solver_name ||
    result.producer!.component_version !== manifestSolver.solver_version)) {
    throw new Error("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH: recorded identity differs from the received producer.");
  }
  return {
    solver_name: precision ? result.producer!.component_name : manifestSolver.solver_name,
    solver_version: precision ? result.producer!.component_version : manifestSolver.solver_version,
    solver_build_ref: manifestSolver.solver_build_ref
  };
}

/** Pure received-data projection for inspection/compatibility. It returns no
 * package, audit manifest, readiness or native provenance and cannot authorize
 * current export. Historical numeric/metadata oracles remain inspectable here. */
export function projectReceivedReportResults(
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope,
  provenance = {source_name:"Received result reference",source_location:"reference inspection",source_license:"not_asserted",contributor:"not_asserted",contributor_certification:"not_asserted",redistribution_status:"private_only",review_status:"pending"},
) {
  const scope = analysisResultHashScope(analysisRun.schema_version);
  if (scope === null) throw new Error("REPORT-PACKAGE-ANALYSIS-VERSION-UNSUPPORTED");
  const strictAnalysis = scope === "received_result";
  const run = analysisRun.analysis_run;
  const resultDimensions = new Map(
    run.result_refs.map((item) => [
      item.result_ref.ref,
      item.source_dimension
    ])
  );
  const semanticDisclosures: Array<{id:string;reason:string}> = [];
  const groupedResults = new Map<string, MechanicsResult["results"]>();
  for (const item of result.results) {
    const semantics = resultSemantics(item, analysisRun.schema_version === "0.3.0" ? result : undefined); // known unit/component contradictions fail closed
    const declared = resultDimensions.get(item.id);
    const expectedSourceDimension = strictAnalysis
      ? semantics?.source_physical_semantic_dimension
      : item.dimension;
    if (!resultDimensions.has(item.id) || (expectedSourceDimension ?? null) !== (declared ?? null)) throw new Error(`REPORT-PACKAGE-SOURCE-DIMENSION-MISMATCH: ${item.id}`);
    if (!semantics || semantics.category !== "physical_quantity") {
      semanticDisclosures.push({id:item.id,reason:semantics?.category ?? "unsupported_source_kind"}); continue;
    }
    const requiredMetadata = ["force","moment","section_property"].includes(semantics.family ?? "");
    if (requiredMetadata && !reportMetadata(item)) {
      semanticDisclosures.push({id:item.id,reason:"legacy_report_metadata_unavailable"}); continue;
    }
    const basisKey = `${item.basis_ref?.ref_type ?? "analysis_run"}:${item.basis_ref?.ref_id ?? result.run_id}`;
    groupedResults.set(basisKey, [...(groupedResults.get(basisKey) ?? []), item]);
  }
  const resultSets = Array.from(groupedResults.entries()).map(([basisKey, values], index) => {
    const basis = values[0]?.basis_ref ?? { ref_type: "analysis_run", ref_id: result.run_id };
    return {
      set_id: `result-set:${result.run_id}:${index + 1}`,
      set_type: "mechanics",
      basis_ref: reference(basis.ref_type, basis.ref_id),
      values: values.map((item) => {
        const semantics = resultSemantics(item, analysisRun.schema_version === "0.3.0" ? result : undefined)!;
        const sourceDimension = resultDimensions.get(item.id);
        if (!sourceDimension) {
          throw new Error(
            `REPORT-PACKAGE-SOURCE-DIMENSION-MISSING: ${item.id} has no DEL-14-02 source declaration.`
          );
        }
        const expectedSourceDimension = strictAnalysis
          ? semantics?.source_physical_semantic_dimension
          : item.dimension;
        if (expectedSourceDimension && expectedSourceDimension !== sourceDimension) {
          throw new Error(
            `REPORT-PACKAGE-SOURCE-DIMENSION-MISMATCH: ${item.id} differs from its DEL-14-02 declaration.`
          );
        }
        const family = semantics.family;
        return {
        result_id: item.id,
        family,
        object_ref: reference("model_entity", item.entity_ref),
        basis_ref: reference(item.basis_ref?.ref_type ?? "analysis_run", item.basis_ref?.ref_id ?? result.run_id),
        station_ref: null,
        magnitude: item.value,
        unit: item.unit,
        dimension: semantics.derivative_target_dimension,
        metadata: reportMetadata(item),
        diagnostics: [],
        trace_chain: (item.source_result_refs ?? []).map((source, traceIndex) => ({
          trace_id: `trace:${item.id}:${traceIndex + 1}`,
          trace_type: "source_result",
          source_ref: reference("result", source),
          target_ref: reference("result", item.id),
          provenance,
          diagnostics: []
        })),
        provenance
        };
      })
    };
  });

  return { resultSets, semanticDisclosures };
}

export async function buildReportPackageRequest({
  model,
  result,
  analysisRun,
  inputManifest,
  projectSummary,
  comparison,
  ruleCheckAggregate
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  inputManifest: CurrentSessionInputManifestEvidence;
  projectSummary: LocalProjectSummary | null;
  comparison: PreviewComparison | null;
  ruleCheckAggregate: string | null;
}) {
  const resultHashScope = analysisResultHashScope(analysisRun.schema_version);
  const strictAnalysis = resultHashScope === "received_result";
  if (resultHashScope === null) {
    throw new Error("REPORT-PACKAGE-ANALYSIS-VERSION-UNSUPPORTED");
  }
  if (sourceContract(result) === "physics") {
    throw new Error("REPORT-PACKAGE-PHYSICS-PROJECTION-UNAVAILABLE: this legacy report transport does not preserve the exact physical metadata/evidence; use canonical result export.");
  }
  if (analysisRun.schema_version === "0.3.0" && sourceContract(result) !== "precision") {
    throw new Error("REPORT-PACKAGE-SOURCE-CONTRACT-MISMATCH");
  }
  await verifyCurrentSessionInputManifest(inputManifest);
  if (
    inputManifest.manifest.model_basis.model_ref !== model.project.id ||
    result.model_ref !== model.project.id
  ) {
    throw new Error(
      "REPORT-PACKAGE-INPUT-MANIFEST-MODEL-MISMATCH: manifest, model, and result refs must match."
    );
  }
  const [modelHash, manifestModelHash] = await Promise.all([
    canonicalSha256Hex(model),
    canonicalSha256Hex(inputManifest.manifest.model_basis.model_payload)
  ]);
  requireSha256("model_hash", modelHash);
  requireSha256("input_manifest.model_basis.model_payload_hash", manifestModelHash);
  if (modelHash !== manifestModelHash) {
    throw new Error(
      "REPORT-PACKAGE-INPUT-MANIFEST-MODEL-PAYLOAD-MISMATCH: supplied model must canonically equal the verified input-manifest model payload."
    );
  }
  if (ruleCheckAggregate !== null || result.status.rule_check !== "RULE_INPUTS_INCOMPLETE") {
    throw new Error(
      "REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE: active rule-check metadata is not owned at the report-package boundary."
    );
  }
  if (!Number.isFinite(result.results.length) || result.results.some((item) => !Number.isFinite(item.value))) {
    throw new Error("REPORT-PACKAGE-NON-FINITE-RESULT: all result values must be finite.");
  }

  const report = await buildRenderableReportInput({ model, result, analysisRun, projectSummary });
  const run = analysisRun.analysis_run;
  const runHash = run.hashes.find((item) => item.payload_scope === "analysis_run_record");
  const resultEnvelopeHash = run.hashes.find(
    (item) => item.payload_scope === resultHashScope
  );
  const [inputManifestRef] = run.reproducibility.input_manifest_refs;
  const [inputHash] = run.reproducibility.input_manifest_hashes;
  if (
    run.reproducibility.input_manifest_refs.length !== 1 ||
    run.reproducibility.input_manifest_hashes.length !== 1 ||
    !runHash?.value ||
    !resultEnvelopeHash?.value ||
    !inputHash?.value
  ) {
    throw new Error("REPORT-PACKAGE-HASH-BINDING-INCOMPLETE: current model/input/run hashes are required.");
  }
  for (const hash of run.hashes) {
    if (hash.algorithm === "sha256") {
      requireSha256(`analysis_run.hashes[${hash.payload_scope}]`, hash.value);
    }
  }
  for (const resultRef of run.result_refs) {
    for (const hash of resultRef.hash_refs) {
      if (hash.algorithm === "sha256") {
        requireSha256(`analysis_run.result_refs[${resultRef.result_ref.ref}]`, hash.value);
      }
    }
  }
  if (inputHash.algorithm !== "sha256") {
    throw new Error(
      "REPORT-PACKAGE-HASH-BINDING-INCOMPLETE: input manifest must declare SHA-256."
    );
  }
  requireSha256("input_manifest_hash", inputHash.value);
  if (
    inputManifestRef.object_type !== "InputManifest" ||
    inputManifestRef.object_type !== inputHash.payload_ref.object_type ||
    inputManifestRef.ref !== inputHash.payload_ref.ref ||
    inputManifestRef.ref !== inputManifest.manifest_ref.ref ||
    inputHash.value !== inputManifest.manifest_sha256
  ) {
    throw new Error(
      "REPORT-PACKAGE-INPUT-MANIFEST-BINDING-MISMATCH: analysis run and current-session manifest evidence differ."
    );
  }
  if (inputHash.value === resultEnvelopeHash.value) {
    throw new Error(
      "REPORT-PACKAGE-INPUT-MANIFEST-RESULT-SUBSTITUTION: result-envelope evidence cannot stand in for the input manifest."
    );
  }

  const solverIdentity = reportSolverIdentity(result, analysisRun, inputManifest);
  const provenance = privateProvenance(model);
  const { resultSets, semanticDisclosures } = projectReceivedReportResults(result, analysisRun, provenance);

  const resultEnvelope = {
    envelope_id: `result-envelope:${result.run_id}`,
    schema_version: "1.0.0",
    model_ref: reference("model", result.model_ref),
    run_ref: reference("analysis_run", result.run_id),
    ...solverIdentity,
    unit_system_ref: reference("unit_system", UNIT_SYSTEM_REF),
    load_basis_refs: run.load_basis_refs.map((item) => reference(item.object_type, item.ref)),
    result_sets: resultSets,
    diagnostics: [...result.diagnostics.map((item, index) => ({
      code: item.code,
      class: diagnosticClass(item.severity),
      severity: item.severity === "blocking" || item.severity === "error" ? "blocking" : "warning",
      source: reference("diagnostic_source", item.source ?? "core/product_physics"),
      affected_object: reference("affected_refs", item.affected_refs?.join(",") || item.id || `diagnostic-${index}`),
      message: item.message,
      remediation: item.remediation ?? "Human review required before reliance.",
      provenance
    })), ...semanticDisclosures.map(item => ({
      code:"REPORT_SOURCE_EVIDENCE_DISCLOSED",class:"assumption_warning",severity:"warning",
      source:reference("source_result",item.id),affected_object:reference("result",item.id),
      message:`${item.id}: ${item.reason}; received numerical evidence remains in the bound source result envelope and legacy analysis-run references.`,
      remediation:"Review the bound source result evidence; this unchanged report transport cannot represent it as a physical quantity.",provenance
    }))],
    provenance,
    reproducibility: {
      model_hash: checksum(reference("model", model.project.id), modelHash),
      run_hashes: run.hashes.map((item) =>
        checksum(reference(item.payload_ref.object_type, item.payload_ref.ref), item.value)
      ),
      audit_manifest_ref: reference("audit_manifest", `audit-manifest:${result.run_id}`),
      deterministic_ordering: true
    },
    analysis_status: Array.from(new Set([...run.analysis_status, "HUMAN_REVIEW_REQUIRED"])).sort(),
    rule_pack_refs: [],
    professional_boundary: PROFESSIONAL_BOUNDARY
  };

  if (!hasNativeMechanicsInvocation(result, model, inputManifest.manifest.solver_basis.solver_mode)) {
    throw new Error("REPORT-PACKAGE-NATIVE-INVOCATION-REQUIRED: reference/history data cannot produce a qualified current package.");
  }
  return {
    package_id: `desktop-report-${safeId(result.run_id)}`,
    export_profile_id: "desktop_local_private_report_package_1",
    source_model_ref: reference("model", result.model_ref),
    source_basis_refs: [reference("analysis_run", result.run_id), ...resultEnvelope.load_basis_refs],
    report,
    audit_manifest: {
      manifest_id: `audit-manifest:${result.run_id}`,
      model_hash: {
        algorithm: "sha256",
        canonicalization: "project_local_deterministic_json",
        payload_kind: "model_json",
        payload_ref: model.project.id,
        value: modelHash
      },
      input_manifest_hash: {
        algorithm: "sha256",
        canonicalization: "project_local_deterministic_json",
        payload_kind: "input_manifest_json",
        payload_ref: inputManifestRef.ref,
        value: inputHash.value
      },
      solver_version: { ...solverIdentity },
      unit_system_ref: UNIT_SYSTEM_REF,
      rule_pack_refs: [],
      assets: [],
      professional_boundary: PROFESSIONAL_BOUNDARY
    },
    result_envelopes: [resultEnvelope],
    state_comparison_handoff_records: [
      buildStateComparisonHandoffSections({ model, result, analysisRun, comparison, modelHash })
    ],
    rule_check_aggregate: ruleCheckAggregate,
    solve_rule_check_status: result.status.rule_check
  };
}
