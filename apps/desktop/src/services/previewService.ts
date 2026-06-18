import { invoke } from "@tauri-apps/api/core";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  MechanicsResult,
  ObjectRef,
  PreviewModel,
  PreviewComparison,
  SelectedReviewTarget
} from "../types";

async function invokeOrFixture<T>(command: string, fixture: () => Promise<T>, args?: Record<string, unknown>): Promise<T> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    return fixture();
  }
  try {
    return await invoke<T>(command, args);
  } catch {
    return fixture();
  }
}

export async function loadPreviewModel(): Promise<PreviewModel> {
  return invokeOrFixture("load_preview_model", loadModelFixture);
}

export async function loadDesignKnowledge(): Promise<DesignKnowledge> {
  return invokeOrFixture("load_design_knowledge", loadKnowledgeFixture);
}

export async function runPreviewMechanics(model?: PreviewModel | null): Promise<MechanicsResult> {
  return invokeOrFixture("run_preview_mechanics", () => runBrowserPreviewMechanics(model), model ? { model } : undefined);
}

export type SolveJobStartReceipt =
  | {
      mode: "backend_job";
      job_id: string;
      backend_cancellation_token: string;
      state: string;
      cancellation_scope: string;
    }
  | { mode: "browser_fixture_no_backend_job" };

export type BackendSolveJobStatus = {
  job_id: string;
  state: "queued" | "running" | "completed" | "cancelled" | "failed";
  cancellation_requested: boolean;
  cancellation_status: string;
  cancellation_scope: string;
  result: MechanicsResult | null;
  error_message: string | null;
};

export type BackendSolveJobCancellationReceipt = {
  job_id: string;
  accepted: boolean;
  cancellation_status: string;
  job_state: string;
  cancellation_scope: string;
  cancellation_success_claimed: boolean;
};

export async function startPreviewMechanicsJob(model?: PreviewModel | null): Promise<SolveJobStartReceipt> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    return { mode: "browser_fixture_no_backend_job" };
  }
  try {
    const receipt = await invoke<Omit<Extract<SolveJobStartReceipt, { mode: "backend_job" }>, "mode">>(
      "start_preview_mechanics_job",
      model ? { model } : undefined
    );
    return { mode: "backend_job", ...receipt };
  } catch {
    return { mode: "browser_fixture_no_backend_job" };
  }
}

export async function pollPreviewMechanicsJob(jobId: string): Promise<BackendSolveJobStatus> {
  return invoke<BackendSolveJobStatus>("poll_preview_mechanics_job", { jobId });
}

export async function cancelPreviewMechanicsJob(
  jobId: string,
  cancellationToken: string
): Promise<BackendSolveJobCancellationReceipt> {
  return invoke<BackendSolveJobCancellationReceipt>("cancel_preview_mechanics_job", {
    jobId,
    cancellationToken
  });
}

// The three frozen automatic rule-check statuses a GUI rule-check run may
// produce (mirrors core/runner/headless `analysis_status_for_rule_check`).
// Anything outside this set is NOT silently coerced (CONTRACT
// no-silent-defaults) — the analysis-run record falls back to the solve
// envelope's own rule_check rather than trusting an unrecognized aggregate.
const RULE_CHECK_RUN_STATUSES = new Set(["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"]);

// Resolve the rule-check status the app-held analysis-run record should carry.
// A recognized GUI rule-check aggregate (the worst-of over the checks the user
// ran against THIS solve) supersedes the solve envelope's `rule_check`, which a
// plain solve always leaves at RULE_INPUTS_INCOMPLETE because the solve runs no
// user rule checks. An absent or unrecognized aggregate falls back to the solve
// envelope's own rule_check — never a false pass.
export function appliedRuleCheckStatus(solveRuleCheck: string, ruleCheckAggregate?: string | null): string {
  if (ruleCheckAggregate && RULE_CHECK_RUN_STATUSES.has(ruleCheckAggregate)) {
    return ruleCheckAggregate;
  }
  return solveRuleCheck;
}

// Build the app-held analysis-run envelope (DEL-14-02) for a solved result.
// When a GUI rule-check aggregate is supplied (TP-C4-APPAGG-001), the record's
// own `analysis_status` and its `analysis_run_record` hash honestly compose the
// mechanics solve with the rule-check run executed against it; the embedded
// `result_envelope` hash still binds the raw solve (which ran no rule checks),
// so the hash-bound solve envelope is never mutated. Omitting the aggregate
// reproduces the prior behavior byte-for-byte.
export async function buildAnalysisRunPreview(
  result: MechanicsResult,
  ruleCheckAggregate?: string | null
): Promise<AnalysisRunEnvelope> {
  const runRef = ref("AnalysisRun", result.run_id);
  const effectiveRuleCheck = appliedRuleCheckStatus(result.status.rule_check, ruleCheckAggregate);
  // The analysis-run record's own status carries the rule-check outcome; the
  // raw solve envelope (hashed separately below) is left exactly as solved.
  const recordStatus = { ...result.status, rule_check: effectiveRuleCheck };
  const resultEnvelopeRef = ref("ResultEnvelope", `result-envelope:${result.run_id}`);
  const resultRefs = await Promise.all(
    result.results
      .slice()
      .sort((left, right) => left.id.localeCompare(right.id))
      .map(async (item) => ({
        result_ref: ref("Result", item.id),
        result_family: resultFamily(item),
        hash_refs: [
          {
            algorithm: "sha256" as const,
            canonicalization: "rfc8785_jcs",
            payload_ref: ref("Result", item.id),
            payload_scope: "result_value",
            value: await sha256(canonicalJson(item))
          }
        ],
        privacy_classification: "invented_public_example"
      }))
  );
  const status = new Set([
    "HUMAN_REVIEW_REQUIRED",
    result.status.mechanics,
    effectiveRuleCheck
  ].filter(Boolean));
  const resultIds = result.results.map((item) => item.id).sort();
  const diagnosticIds = result.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort();
  const loadBasisRefs = loadBasisRefsFor(result);

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-14-02",
    package_id: "PKG-14",
    scope_item: "SOW-072",
    objectives: ["OBJ-016"],
    run_contract_status: {
      record_contract: "schema_first_analysis_run_records",
      model_state_binding: "schemas/model_state.schema.json",
      result_binding: "schemas/results.schema.yaml",
      physical_project_container: "TBD",
      external_validation_boundary: "reference_only_not_determined_by_software"
    },
    analysis_run: {
      run_id: result.run_id,
      run_name: `${result.run_id} preview mechanics run`,
      run_kind: "mechanics_solve",
      model_state_ref: ref("ModelState", `state:${result.model_ref}:preview`),
      load_basis_refs: loadBasisRefs,
      result_refs: resultRefs,
      hashes: [
        {
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: runRef,
          payload_scope: "analysis_run_record",
          value: await sha256(
            canonicalJson({
              run_id: result.run_id,
              model_ref: result.model_ref,
              status: recordStatus,
              load_basis_refs: loadBasisRefs,
              result_ids: resultIds,
              diagnostic_ids: diagnosticIds
            })
          )
        },
        {
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: resultEnvelopeRef,
          payload_scope: "result_envelope",
          value: await sha256(canonicalJson(result))
        }
      ],
      analysis_status: Array.from(status).sort(),
      reproducibility: {
        input_manifest_refs: [resultEnvelopeRef],
        determinism_notes: [
          "analysis run record was generated from an already computed invented preview mechanics result",
          "canonical hashes use stable JSON key ordering"
        ],
        unresolved_tbd: ["physical project container", "release-grade solver build provenance"]
      },
      immutability_policy: {
        run_record_is_read_only: true,
        mutation_policy: "changes_create_new_analysis_run",
        new_run_required_for_change: true,
        hash_invalidates_external_acceptance: true
      },
      professional_boundary: {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
        software_makes_authentication_claim: false
      }
    }
  };
}

export function buildPreviewComparison({
  result,
  analysisRun,
  leftBasisRef = "load:L-100",
  rightBasisRef = "combination:C-OPER-ALT"
}: {
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  leftBasisRef?: string;
  rightBasisRef?: string;
}): PreviewComparison {
  const run = analysisRun.analysis_run;
  const resultIndex = new Map(result.results.map((item) => [item.id, item]));
  const leftResults = result.results.filter((item) => item.basis_ref?.ref_id === leftBasisRef);
  const rightResults = result.results.filter((item) => item.basis_ref?.ref_id === rightBasisRef);
  const matchedLeftIds = new Set<string>();
  const unmatchedRightRefs: string[] = [];
  const deltas = rightResults
    .flatMap((rightResult) => {
      const leftResultId = rightResult.source_result_refs?.find((sourceRef) => resultIndex.has(sourceRef));
      if (!leftResultId) {
        unmatchedRightRefs.push(rightResult.id);
        return [];
      }
      const leftResult = resultIndex.get(leftResultId);
      if (!leftResult || leftResult.unit !== rightResult.unit) {
        unmatchedRightRefs.push(rightResult.id);
        return [];
      }
      matchedLeftIds.add(leftResult.id);
      return [
        {
          mapping_id: `mapping:${leftResult.id}->${rightResult.id}`,
          left_result_id: leftResult.id,
          right_result_id: rightResult.id,
          entity_ref: rightResult.entity_ref,
          result_family: resultFamily(rightResult),
          component: rightResult.metadata?.component ?? rightResult.kind,
          location: rightResult.metadata?.location ?? "summary",
          unit: rightResult.unit,
          left_value: leftResult.value,
          right_value: rightResult.value,
          raw_delta: rightResult.value - leftResult.value,
          absolute_delta: Math.abs(rightResult.value - leftResult.value),
          classification: "not_tolerance_checked" as const,
          classification_basis: "governed_tolerance_profile_TBD_no_default_engineering_threshold"
        }
      ];
    })
    .sort((left, right) => {
      const byDelta = right.absolute_delta - left.absolute_delta;
      return byDelta === 0 ? left.right_result_id.localeCompare(right.right_result_id) : byDelta;
    });
  const unmatchedLeftRefs = leftResults.map((item) => item.id).filter((id) => !matchedLeftIds.has(id));
  const diagnostics = comparisonDiagnostics({ unmatchedLeftRefs, unmatchedRightRefs });
  const matchedResultUnits = Array.from(new Set(deltas.map((item) => item.unit).filter(Boolean))).sort();

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.comparison",
    deliverable_id: "DEL-14-04",
    package_id: "PKG-14",
    scope_items: ["SOW-073"],
    objectives: ["OBJ-016"],
    comparison_id: `comparison:${result.run_id}:${safeComparisonToken(leftBasisRef)}-to-${safeComparisonToken(rightBasisRef)}`,
    comparison_kind: "single_run_load_basis_review",
    left: {
      label: "Reference load-case result rows",
      basis_ref: ref("LoadCase", leftBasisRef),
      model_state_ref: run.model_state_ref,
      analysis_run_ref: ref("AnalysisRun", run.run_id),
      result_count: leftResults.length
    },
    right: {
      label: "User-defined combination result rows",
      basis_ref: ref("Combination", rightBasisRef),
      model_state_ref: run.model_state_ref,
      analysis_run_ref: ref("AnalysisRun", run.run_id),
      result_count: rightResults.length
    },
    summary: {
      comparable_result_pairs: deltas.length,
      unmatched_left_results: unmatchedLeftRefs.length,
      unmatched_right_results: unmatchedRightRefs.length,
      mapping_basis: "stable result IDs plus explicit source_result_refs from the preview mechanics result envelope",
      tolerance_status: "not_tolerance_checked",
      tolerance_profile_ref: "TBD"
    },
    unit_policy_evidence: {
      evidence_id: "unit-policy-evidence:comparison-workspace-preview",
      unit_system_ref: ref("UnitSystem", "unit-system:dec-018-si-dual-display"),
      storage_convention: "entered_units_preserved",
      comparison_unit_policy: "compare_only_rows_with_equal_explicit_result_units",
      matching_policy: "stable_result_refs_must_match_and_units_must_be_equal_before_delta",
      matched_result_units: matchedResultUnits,
      unmatched_left_result_count: unmatchedLeftRefs.length,
      unmatched_right_result_count: unmatchedRightRefs.length,
      conversion_policy: "comparison_workspace_preserves_result_units_without_conversion",
      conversion_performed: false,
      tolerance_profile_ref: "TBD",
      tolerance_status: "not_tolerance_checked",
      decision_basis_refs: [ref("Decision", "DEC-018"), ref("Decision", "DEC-026"), ref("Deliverable", "DEL-14-05")],
      protected_content_included: false,
      private_payload_included: false
    },
    result_deltas: deltas,
    diagnostics,
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    }
  };
}

function loadBasisRefsFor(result: MechanicsResult): ObjectRef[] {
  const seen = new Set<string>();
  const refs: ObjectRef[] = [];
  for (const item of result.results) {
    const basis = item.basis_ref;
    if (!basis) continue;
    const objectType = basis.ref_type === "combination" ? "Combination" : "LoadCase";
    const key = `${objectType}:${basis.ref_id}`;
    if (seen.has(key)) continue;
    seen.add(key);
    refs.push(ref(objectType, basis.ref_id));
  }
  return refs;
}

function comparisonDiagnostics({
  unmatchedLeftRefs,
  unmatchedRightRefs
}: {
  unmatchedLeftRefs: string[];
  unmatchedRightRefs: string[];
}) {
  const diagnostics = [];
  if (unmatchedLeftRefs.length > 0) {
    diagnostics.push({
      id: "diagnostic:comparison:reference-unmatched",
      code: "COMPARISON_REFERENCE_ROWS_UNMATCHED",
      severity: "info" as const,
      message:
        "Some reference load-case result rows do not have a matching user-defined combination row in the preview comparison.",
      source: "apps/desktop/src/services/previewService.ts",
      affected_refs: unmatchedLeftRefs.slice(0, 12)
    });
  }
  if (unmatchedRightRefs.length > 0) {
    diagnostics.push({
      id: "diagnostic:comparison:target-unmatched",
      code: "COMPARISON_TARGET_ROWS_UNMATCHED",
      severity: "warning" as const,
      message:
        "Some user-defined combination result rows could not be mapped back to source result rows for preview comparison.",
      source: "apps/desktop/src/services/previewService.ts",
      affected_refs: unmatchedRightRefs.slice(0, 12)
    });
  }
  return diagnostics;
}

function safeComparisonToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "basis";
}

export async function loadSampleProposal(
  mechanicsResult?: MechanicsResult | null,
  selectedTarget?: SelectedReviewTarget | null
): Promise<AgentProposal> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    try {
      return (
        await invoke<{ proposal: AgentProposal }>("sample_agent_proposal", {
          mechanicsResult,
          selectedTarget
        })
      ).proposal;
    } catch {
      // Fall through to the local fixture-backed proposal below.
    }
  }
  return buildProposalFromMechanics(
    mechanicsResult ?? (await loadMechanicsFixture()),
    await loadAgentProposalFixture(),
    selectedTarget
  );
}

async function loadModelFixture(): Promise<PreviewModel> {
  return (await import("../../../../fixtures/product_preview/invented_preview_model.json")).default as PreviewModel;
}

async function loadKnowledgeFixture(): Promise<DesignKnowledge> {
  return (await import("../../../../fixtures/product_preview/invented_design_knowledge.json")).default as DesignKnowledge;
}

async function loadMechanicsFixture(): Promise<MechanicsResult> {
  return (await import("../../../../fixtures/product_preview/invented_mechanics_result.json")).default as MechanicsResult;
}

async function runBrowserPreviewMechanics(model?: PreviewModel | null): Promise<MechanicsResult> {
  if (!model) return loadMechanicsFixture();
  const fixtureModel = await loadModelFixture();
  if (canonicalJson(model) === canonicalJson(fixtureModel)) return loadMechanicsFixture();
  return blockedBrowserEditedModelResult(model);
}

function blockedBrowserEditedModelResult(model: PreviewModel): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: `run:preview-linear-static-browser-blocked:${safeComparisonToken(model.project.id)}`,
    model_ref: model.project.id,
    status: {
      mechanics: "MODEL_INCOMPLETE",
      rule_check: "RULE_INPUTS_INCOMPLETE",
      professional_acceptance: "NOT_PROVIDED"
    },
    summary: {
      node_count: model.nodes.length,
      segment_count: model.pipe_segments.length,
      support_count: model.supports.length,
      load_case_count: model.load_cases.length,
      max_displacement: null,
      max_open_formula_stress: null
    },
    results: [],
    diagnostics: [
      {
        id: "diagnostic:browser-solve:edited-model-backend-required",
        code: "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL",
        severity: "blocking",
        source: "apps/desktop/src/services/previewService.ts",
        affected_refs: [model.project.id],
        message:
          "Browser fixture mode will not reuse bundled solved-result rows for an edited model; run through the Tauri backend solve path for model-bound mechanics results."
      }
    ]
  };
}

async function loadAgentProposalFixture(): Promise<AgentProposal> {
  return (await import("../../../../fixtures/product_preview/invented_agent_proposal.json")).default as AgentProposal;
}

function ref(objectType: string, value: string): ObjectRef {
  return { object_type: objectType, ref: value };
}

function resultFamily(result: MechanicsResult["results"][number]): string {
  const kind = result.kind.toLowerCase();
  const id = result.id.toLowerCase();
  if (kind.includes("displacement") || id.includes("disp")) return "displacement";
  if (kind.includes("reaction") || id.includes("reaction")) return "reaction";
  if (kind.includes("force") || id.includes("force")) return "force";
  if (kind.includes("moment") || id.includes("moment")) return "moment";
  if (kind.includes("stress") || id.includes("stress")) return "stress";
  if (kind.includes("ratio") || id.includes("ratio")) return "ratio";
  return "TBD";
}

function canonicalJson(value: unknown): string {
  return JSON.stringify(sortJson(value));
}

function sortJson(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(sortJson);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, nested]) => [key, sortJson(nested)])
    );
  }
  return value;
}

async function sha256(payload: string): Promise<string> {
  if (globalThis.crypto?.subtle) {
    const bytes = new TextEncoder().encode(payload);
    const digest = await globalThis.crypto.subtle.digest("SHA-256", bytes);
    return Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
  let hash = 5381;
  for (let index = 0; index < payload.length; index += 1) {
    hash = (hash * 33) ^ payload.charCodeAt(index);
  }
  return `sha256-unavailable-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

function buildProposalFromMechanics(
  result: MechanicsResult,
  agentProposalFixture: AgentProposal,
  selectedTarget?: SelectedReviewTarget | null
): AgentProposal {
  const forceResult =
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial") ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  const primaryDiagnostic =
    result.diagnostics.find((item) => item.severity === "warning" || item.severity === "blocking") ??
    result.diagnostics[0];
  const targetRef =
    selectedTarget?.id ??
    forceResult?.id ??
    primaryDiagnostic?.id ??
    primaryDiagnostic?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";
  const targetKind = selectedTarget?.target_type.replaceAll("_", " ") ?? "computed mechanics";

  return {
    ...agentProposalFixture,
    proposal_id: "proposal:physics-diagnostic-review",
    prompt: "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
    operation: {
      ...agentProposalFixture.operation,
      operation_id: "op:review-computed-diagnostic",
      operation_kind: "attach_design_knowledge",
      affected_entity_ids: [targetRef],
      changes: [
        {
          change_id: "change:add-review-note",
          change_kind: "attach_design_knowledge",
          target_ref: targetRef,
          before: `No review note attached for the selected ${targetKind} context.`,
          after: `Attach review note referencing the current computed preview ${targetKind} context.`
        }
      ]
    },
    rationale: `Generated from current preview mechanics context; selected review reference is ${targetRef}. This narrative is review-only and does not mutate accepted model state.`,
    validation: {
      ...agentProposalFixture.validation,
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context"
    }
  };
}
