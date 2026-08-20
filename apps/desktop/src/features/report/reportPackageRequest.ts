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
import { declaredSourceResultDimension } from "../../services/previewService";
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

function familyFor(
  item: MechanicsResult["results"][number],
  sourceDimension: MechanicsResult["results"][number]["dimension"]
): string {
  if (!sourceDimension) {
    throw new Error(
      `REPORT-PACKAGE-SOURCE-DIMENSION-MISSING: ${item.id} has no DEL-14-02 source declaration.`
    );
  }
  const expectedDimension = declaredSourceResultDimension(item);
  if (sourceDimension !== expectedDimension) {
    throw new Error(
      `REPORT-PACKAGE-UNSUPPORTED-RESULT-FAMILY: ${item.id} declares ${sourceDimension}; exact kind semantics require ${expectedDimension}.`
    );
  }
  if (
    sourceDimension === "force" &&
    (
      item.kind === "reaction_resultant" ||
      item.kind === "nonlinear_support_final_reaction" ||
      item.kind === "nonlinear_support_friction_normal_reaction_derived" ||
      item.kind === "nonlinear_support_observed_max_force_reaction_delta"
    )
  ) {
    return "reaction";
  }
  if (
    sourceDimension === "moment" &&
    item.kind === "nonlinear_support_observed_max_moment_reaction_delta"
  ) {
    return "reaction";
  }
  const families: Record<string, string> = {
    length: "displacement",
    angle: "rotation",
    force: "force",
    moment: "moment",
    stress: "stress",
    pressure: "stress",
    linear_stiffness: "section_property",
    rotational_stiffness: "section_property",
    area: "section_property",
    section_modulus: "section_property",
    second_moment_area: "section_property",
    ratio: "ratio",
    dimensionless: "ratio"
  };
  const family = families[sourceDimension];
  if (!family) {
    throw new Error(
      `REPORT-PACKAGE-UNSUPPORTED-RESULT-FAMILY: ${item.id} uses unsupported explicit dimension ${sourceDimension}.`
    );
  }
  return family;
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
    (item) => item.payload_scope === "result_envelope"
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

  const provenance = privateProvenance(model);
  const resultDimensions = new Map(
    run.result_refs.map((item) => [
      item.result_ref.ref,
      item.source_dimension
    ])
  );
  const groupedResults = new Map<string, MechanicsResult["results"]>();
  for (const item of result.results) {
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
        const sourceDimension = resultDimensions.get(item.id);
        if (!sourceDimension) {
          throw new Error(
            `REPORT-PACKAGE-SOURCE-DIMENSION-MISSING: ${item.id} has no DEL-14-02 source declaration.`
          );
        }
        if (item.dimension && item.dimension !== sourceDimension) {
          throw new Error(
            `REPORT-PACKAGE-SOURCE-DIMENSION-MISMATCH: ${item.id} differs from its DEL-14-02 declaration.`
          );
        }
        const family = familyFor(item, sourceDimension);
        return {
        result_id: item.id,
        family,
        object_ref: reference("model_entity", item.entity_ref),
        basis_ref: reference(item.basis_ref?.ref_type ?? "analysis_run", item.basis_ref?.ref_id ?? result.run_id),
        station_ref: null,
        magnitude: item.value,
        unit: item.unit,
        dimension: sourceDimension,
        metadata:
          item.metadata ??
          (["force", "moment", "section_property"].includes(family)
            ? {
                component: item.kind,
                coordinate_system: "source_result_coordinate_system_not_expanded",
                location: item.entity_ref,
                basis: item.basis_ref?.ref_id ?? result.run_id,
                sign_convention: "source scalar sign preserved from the mechanics-result envelope"
              }
            : null),
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

  const resultEnvelope = {
    envelope_id: `result-envelope:${result.run_id}`,
    schema_version: "1.0.0",
    model_ref: reference("model", result.model_ref),
    run_ref: reference("analysis_run", result.run_id),
    solver_name: "open_pipe_stress_product_physics",
    solver_version: "0.1.0",
    solver_build_ref: "open_pipe_stress_product_physics@0.1.0",
    unit_system_ref: reference("unit_system", UNIT_SYSTEM_REF),
    load_basis_refs: run.load_basis_refs.map((item) => reference(item.object_type, item.ref)),
    result_sets: resultSets,
    diagnostics: result.diagnostics.map((item, index) => ({
      code: item.code,
      class: diagnosticClass(item.severity),
      severity: item.severity === "blocking" || item.severity === "error" ? "blocking" : "warning",
      source: reference("diagnostic_source", item.source ?? "core/product_physics"),
      affected_object: reference("affected_refs", item.affected_refs?.join(",") || item.id || `diagnostic-${index}`),
      message: item.message,
      remediation: item.remediation ?? "Human review required before reliance.",
      provenance
    })),
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
      solver_version: {
        solver_name: "open_pipe_stress_product_physics",
        solver_version: "0.1.0",
        solver_build_ref: "open_pipe_stress_product_physics@0.1.0"
      },
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
