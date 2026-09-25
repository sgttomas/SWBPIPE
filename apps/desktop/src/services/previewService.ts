import { sourceContract } from '../features/results/numericalResultQuality';
import { invoke } from "@tauri-apps/api/core";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  CanonicalResultDimension,
  DesignKnowledge,
  MechanicsResult,
  ObjectRef,
  PreviewModel,
  PreviewComparison,
  SelectedReviewTarget,
} from "../types";
import {
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence,
} from "./inputManifestService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";

async function invokeOrFixture<T>(
  command: string,
  fixture: () => Promise<T>,
  args?: Record<string, unknown>,
): Promise<T> {
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

export async function runPreviewMechanics(
  model?: PreviewModel | null,
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<MechanicsResult> {
  assertPreviewSolverMode(solverMode);
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    throw await browserSolveUnavailable(model);
  }
  const capture = captureNativeInvocation(model, solverMode);
  const result = await invoke<MechanicsResult>(
    "run_preview_mechanics_with_solver_mode",
    model ? { model: capture?.invocation.request.model ?? model, solverMode } : { solverMode },
  );
  await validateCapturedSource(result, capture);
  return result; // Preserve received producer data; legacy projection is explicit.

}

export type PreviewSolverMode = "sparse_interactive" | "dense_scrutiny";

export type NativeMechanicsInvocation = {
  request: { model: PreviewModel; materials: [] };
  solver_mode: PreviewSolverMode;
};
type CapturedNativeInvocation = { invocation: NativeMechanicsInvocation; fingerprint: string; invalidated: boolean; terminalClaimed: boolean };
type NativeSourceRegistration = { capture: CapturedNativeInvocation; sourceFingerprint: string };
const nativeSourceInvocations = new WeakMap<MechanicsResult, NativeSourceRegistration>();
const jobInvocations = new Map<string, CapturedNativeInvocation>();

// Private equality witness only, not another public hash/canonicalization profile.
// The checked guard rejects lossy/non-JSON inputs, and the extra paths retain
// negative-zero identity that JSON text alone deliberately normalizes.
function nativeContentFingerprint(value: unknown): string {
  const checked = checkedJsonText(value);
  const negativeZeros: string[] = [];
  const visit = (item: unknown, path: string) => {
    if (typeof item === "number" && Object.is(item, -0)) negativeZeros.push(path);
    else if (item && typeof item === "object") {
      for (const [key, child] of Object.entries(item)) visit(child, `${path}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`);
    }
  };
  visit(value, "");
  return `${canonicalJson(JSON.parse(checked))}\nnegative_zero_paths=${JSON.stringify(negativeZeros.sort())}`;
}
function captureNativeInvocation(model: PreviewModel | null | undefined, solverMode: PreviewSolverMode): CapturedNativeInvocation | null {
  if (!model || typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return null;
  try {
    // Mirror the native API's actual {model, materials:[]} request and send this
    // captured JSON model, not a later caller mutation or a reconstructed result.
    const invocation = JSON.parse(checkedJsonText({ request: { model, materials: [] }, solver_mode: solverMode })) as NativeMechanicsInvocation;
    return { invocation, fingerprint: nativeContentFingerprint(invocation), invalidated: false, terminalClaimed: false };
  } catch { return null; } // Raw native diagnostics remain available; no registration.
}
async function validateCapturedSource(source: MechanicsResult, capture: CapturedNativeInvocation | null): Promise<void> {
  if (!capture || capture.invalidated) return;
  try {
    if (source.model_ref !== capture.invocation.request.model.project.id || nativeContentFingerprint(capture.invocation) !== capture.fingerprint) return;
    const sourceFingerprint = nativeContentFingerprint(source);
    // Registration is reachable only after actual direct IPC or a completed
    // known job. Method-specific source-block validation composes here when its
    // separately reviewed join is selected; none is inferred from these bytes.
    await canonicalSha256HexCheckedV1(source);
    if (capture.invalidated || nativeContentFingerprint(source) !== sourceFingerprint || nativeContentFingerprint(capture.invocation) !== capture.fingerprint) return;
    nativeSourceInvocations.set(source, { capture, sourceFingerprint });
  } catch { /* Preserve received source for inspection, without qualified standing. */ }
}
/** Query only: headers, hashes, clones and saved records cannot mint registration. */
export function hasNativeMechanicsInvocation(
  source: MechanicsResult | null | undefined,
  model: PreviewModel | null | undefined,
  solverMode?: string,
): boolean {
  if (!source || !model) return false;
  const registered = nativeSourceInvocations.get(source);
  if (!registered) return false;
  try {
    const invocation = registered.capture.invocation;
    return !registered.capture.invalidated
      && (solverMode === undefined || solverMode === invocation.solver_mode)
      && nativeContentFingerprint(invocation) === registered.capture.fingerprint
      && nativeContentFingerprint(source) === registered.sourceFingerprint
      && nativeContentFingerprint(JSON.parse(checkedJsonText(model))) === nativeContentFingerprint(invocation.request.model);
  } catch { return false; }
}
export function retainedNativeMechanicsInvocation(source: MechanicsResult, model: PreviewModel): NativeMechanicsInvocation | null {
  return hasNativeMechanicsInvocation(source, model)
    ? structuredClone(nativeSourceInvocations.get(source)!.capture.invocation) : null;
}

function assertPreviewSolverMode(value: unknown): asserts value is PreviewSolverMode {
  if (value !== "sparse_interactive" && value !== "dense_scrutiny") {
    throw new Error(`PREVIEW_SOLVER_MODE_UNSUPPORTED: ${String(value)}`);
  }
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

export async function startPreviewMechanicsJob(
  model?: PreviewModel | null,
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<SolveJobStartReceipt> {
  assertPreviewSolverMode(solverMode);
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    throw await browserSolveUnavailable(model);
  }
  const capture = captureNativeInvocation(model, solverMode);
  const receipt = await invoke<
      Omit<Extract<SolveJobStartReceipt, { mode: "backend_job" }>, "mode">
    >("start_preview_mechanics_job_with_solver_mode", model ? { model: capture?.invocation.request.model ?? model, solverMode } : { solverMode });
  if (capture && typeof receipt.job_id === "string" && receipt.job_id) jobInvocations.set(receipt.job_id, capture);
  return { mode: "backend_job", ...receipt };
}

export async function pollPreviewMechanicsJob(
  jobId: string,
): Promise<BackendSolveJobStatus> {
  const status = await invoke<BackendSolveJobStatus>("poll_preview_mechanics_job", { jobId });
  const capture = jobInvocations.get(jobId) ?? null;
  if (status.job_id !== jobId) {
    if (capture) capture.invalidated = true;
    jobInvocations.delete(jobId);
    throw new Error("SOLVE-JOB-IDENTITY-MISMATCH");
  }
  if (["completed", "cancelled", "failed"].includes(status.state)) {
    if (capture && !capture.terminalClaimed) {
      capture.terminalClaimed = true;
      try {
        if (status.state === "completed" && status.result) await validateCapturedSource(status.result, capture);
        else capture.invalidated = true;
      } finally {
        if (jobInvocations.get(jobId) === capture) jobInvocations.delete(jobId);
      }
    }
  }
  return status;
}

export async function cancelPreviewMechanicsJob(
  jobId: string,
  cancellationToken: string,
): Promise<BackendSolveJobCancellationReceipt> {
  const receipt = await invoke<BackendSolveJobCancellationReceipt>(
    "cancel_preview_mechanics_job",
    {
      jobId,
      cancellationToken,
    },
  );
  if (receipt.accepted) {
    const capture = jobInvocations.get(jobId);
    if (capture) capture.invalidated = true;
    jobInvocations.delete(jobId);
  }
  return receipt;
}

// The three frozen automatic rule-check statuses a GUI rule-check run may
// produce (mirrors core/runner/headless `analysis_status_for_rule_check`).
// Anything outside this set is NOT silently coerced (CONTRACT
// no-silent-defaults) — the analysis-run record falls back to the solve
// envelope's own rule_check rather than trusting an unrecognized aggregate.
const RULE_CHECK_RUN_STATUSES = new Set([
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED",
]);

// Resolve the rule-check status the app-held analysis-run record should carry.
// A recognized GUI rule-check aggregate (the worst-of over the checks the user
// ran against THIS solve) supersedes the solve envelope's `rule_check`, which a
// plain solve always leaves at RULE_INPUTS_INCOMPLETE because the solve runs no
// user rule checks. An absent or unrecognized aggregate falls back to the solve
// envelope's own rule_check — never a false pass.
export function appliedRuleCheckStatus(
  solveRuleCheck: string,
  ruleCheckAggregate?: string | null,
): string {
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
  {
    inputManifest,
    ruleCheckAggregate,
  }: {
    inputManifest: CurrentSessionInputManifestEvidence;
    ruleCheckAggregate?: string | null;
  },
): Promise<AnalysisRunEnvelope> {
  await verifyCurrentSessionInputManifest(inputManifest);
  const effective = appliedRuleCheckStatus(result.status.rule_check, ruleCheckAggregate);
  const route = sourceContract(result);
  if (route !== "precision" && route !== "physics") throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return buildAnalysisRunV03(result, inputManifest, effective, modelLoadBasisRefs(inputManifest.manifest.model_basis.model_payload));
}

export function bindSourceResultDimensions(
  result: MechanicsResult,
): MechanicsResult {
  // Precision carriers and unsupported historical headers retain exact received bytes.
  if (sourceContract(result) !== "legacy") return result;
  return {
    ...result,
    results: result.results.map((item) => {
      const declared = declaredSourceResultDimension(item);
      if (item.dimension && item.dimension !== declared) {
        throw new Error(
          `ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: ${item.id} declares ${item.dimension}; exact kind semantics require ${declared}.`,
        );
      }
      return {
        ...item,
        dimension: declared,
      };
    }),
  };
}

export function declaredSourceResultDimension(
  item: Omit<MechanicsResult["results"][number], "dimension"> &
    Partial<Pick<MechanicsResult["results"][number], "dimension">>,
): CanonicalResultDimension {
  const kind = item.kind;
  const component = item.metadata?.component;
  if (kind === "component_user_stiffness_macro_element_review") {
    if (
      component === "axial_user_stiffness" ||
      component === "lateral_user_stiffness"
    ) {
      return "linear_stiffness";
    }
    if (
      component === "angular_user_stiffness" ||
      component === "torsional_user_stiffness"
    ) {
      return "rotational_stiffness";
    }
  }
  if (kind === "constant_effort_user_input_review") {
    if (component === "constant_effort_support_constant_load") return "force";
    if (component === "constant_effort_support_travel_range") return "length";
  }
  if (kind === "spring_hanger_user_input_review") {
    if (component === "variable_spring_hanger_stiffness")
      return "linear_stiffness";
    if (
      component === "variable_spring_hanger_installed_load" ||
      component === "variable_spring_hanger_cold_load" ||
      component === "variable_spring_hanger_hot_load"
    ) {
      return "force";
    }
    if (component === "variable_spring_hanger_travel_range") return "length";
  }
  const declarations: Record<string, CanonicalResultDimension> = {
    component_user_stress_multiplier_review: "stress",
    displacement_magnitude: "length",
    element_local_axial_force: "force",
    element_local_axial_normal_stress: "stress",
    element_local_bending_moment_y: "moment",
    element_local_bending_moment_z: "moment",
    element_local_bending_normal_stress_y: "stress",
    element_local_bending_normal_stress_z: "stress",
    element_local_shear_force_y: "force",
    element_local_shear_force_z: "force",
    element_local_torsional_moment: "moment",
    element_local_torsional_shear_stress: "stress",
    expansion_joint_pressure_thrust_load_review: "force",
    global_nodal_displacement_x: "length",
    global_nodal_displacement_y: "length",
    global_nodal_displacement_z: "length",
    global_nodal_rotation_x: "angle",
    global_nodal_rotation_y: "angle",
    global_nodal_rotation_z: "angle",
    linear_solver_mode_basis: "dimensionless",
    nonlinear_support_active_set_converged_flag: "dimensionless",
    nonlinear_support_active_set_final_residual_count: "dimensionless",
    nonlinear_support_active_set_iteration_count: "dimensionless",
    nonlinear_support_active_set_state_code: "dimensionless",
    nonlinear_support_final_displacement: "length",
    nonlinear_support_final_reaction: "force",
    nonlinear_support_free_dof_work_residual: "moment",
    nonlinear_support_friction_normal_reaction_derived: "force",
    nonlinear_support_observed_free_dof_force_residual: "force",
    nonlinear_support_observed_free_dof_moment_residual: "moment",
    nonlinear_support_observed_max_force_reaction_delta: "force",
    nonlinear_support_observed_max_moment_reaction_delta: "moment",
    nonlinear_support_observed_max_rotation_delta: "angle",
    nonlinear_support_observed_max_translation_delta: "length",
    open_formula_stress_summary: "stress",
    pipe_section_pressure_hoop_stress: "stress",
    reaction_resultant: "force",
  };
  const declared = declarations[kind];
  if (!declared) {
    throw new Error(
      `ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED: ${item.id} (${item.kind}) has no source dimension declaration.`,
    );
  }
  return declared;
}

export function buildPreviewComparison({
  result,
  analysisRun,
  leftBasisRef = "load:L-100",
  rightBasisRef = "combination:C-OPER-ALT",
}: {
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  leftBasisRef?: string;
  rightBasisRef?: string;
}): PreviewComparison {
  const run = analysisRun.analysis_run;
  const resultIndex = new Map(result.results.map((item) => [item.id, item]));
  const leftResults = result.results.filter(
    (item) => item.basis_ref?.ref_id === leftBasisRef,
  );
  const rightResults = result.results.filter(
    (item) => item.basis_ref?.ref_id === rightBasisRef,
  );
  const matchedLeftIds = new Set<string>();
  const unmatchedRightRefs: string[] = [];
  const deltas = rightResults
    .flatMap((rightResult) => {
      const leftResultId = rightResult.source_result_refs?.find((sourceRef) =>
        resultIndex.has(sourceRef),
      );
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
          result_family: resultFamily(
            rightResult,
            rightResult.dimension ??
              declaredSourceResultDimension(rightResult),
          ),
          component: rightResult.metadata?.component ?? rightResult.kind,
          location: rightResult.metadata?.location ?? "summary",
          unit: rightResult.unit,
          left_value: leftResult.value,
          right_value: rightResult.value,
          raw_delta: rightResult.value - leftResult.value,
          absolute_delta: Math.abs(rightResult.value - leftResult.value),
          classification: "not_tolerance_checked" as const,
          classification_basis:
            "governed_tolerance_profile_TBD_no_default_engineering_threshold",
        },
      ];
    })
    .sort((left, right) => {
      const byDelta = right.absolute_delta - left.absolute_delta;
      return byDelta === 0
        ? left.right_result_id.localeCompare(right.right_result_id)
        : byDelta;
    });
  const unmatchedLeftRefs = leftResults
    .map((item) => item.id)
    .filter((id) => !matchedLeftIds.has(id));
  const diagnostics = comparisonDiagnostics({
    unmatchedLeftRefs,
    unmatchedRightRefs,
  });
  const matchedResultUnits = Array.from(
    new Set(deltas.map((item) => item.unit).filter(Boolean)),
  ).sort();

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
      result_count: leftResults.length,
    },
    right: {
      label: "User-defined combination result rows",
      basis_ref: ref("Combination", rightBasisRef),
      model_state_ref: run.model_state_ref,
      analysis_run_ref: ref("AnalysisRun", run.run_id),
      result_count: rightResults.length,
    },
    summary: {
      comparable_result_pairs: deltas.length,
      unmatched_left_results: unmatchedLeftRefs.length,
      unmatched_right_results: unmatchedRightRefs.length,
      mapping_basis:
        "stable result IDs plus explicit source_result_refs from the preview mechanics result envelope",
      tolerance_status: "not_tolerance_checked",
      tolerance_profile_ref: "TBD",
    },
    unit_policy_evidence: {
      evidence_id: "unit-policy-evidence:comparison-workspace-preview",
      unit_system_ref: ref("UnitSystem", "unit-system:dec-018-si-dual-display"),
      storage_convention: "entered_units_preserved",
      comparison_unit_policy:
        "compare_only_rows_with_equal_explicit_result_units",
      matching_policy:
        "stable_result_refs_must_match_and_units_must_be_equal_before_delta",
      matched_result_units: matchedResultUnits,
      unmatched_left_result_count: unmatchedLeftRefs.length,
      unmatched_right_result_count: unmatchedRightRefs.length,
      conversion_policy:
        "comparison_workspace_preserves_result_units_without_conversion",
      conversion_performed: false,
      tolerance_profile_ref: "TBD",
      tolerance_status: "not_tolerance_checked",
      decision_basis_refs: [
        ref("Decision", "DEC-018"),
        ref("Decision", "DEC-026"),
        ref("Deliverable", "DEL-14-05"),
      ],
      protected_content_included: false,
      private_payload_included: false,
    },
    result_deltas: deltas,
    diagnostics,
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false,
    },
  };
}


function comparisonDiagnostics({
  unmatchedLeftRefs,
  unmatchedRightRefs,
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
      affected_refs: unmatchedLeftRefs.slice(0, 12),
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
      affected_refs: unmatchedRightRefs.slice(0, 12),
    });
  }
  return diagnostics;
}

function safeComparisonToken(value: string): string {
  return (
    value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "basis"
  );
}

export async function loadSampleProposal(
  mechanicsResult?: MechanicsResult | null,
  selectedTarget?: SelectedReviewTarget | null,
): Promise<AgentProposal> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    try {
      return (
        await invoke<{ proposal: AgentProposal }>("sample_agent_proposal", {
          mechanicsResult,
          selectedTarget,
        })
      ).proposal;
    } catch {
      // Fall through to the local fixture-backed proposal below.
    }
  }
  return buildProposalFromMechanics(
    mechanicsResult ?? (await loadMechanicsFixture()),
    await loadAgentProposalFixture(),
    selectedTarget,
  );
}

async function loadModelFixture(): Promise<PreviewModel> {
  return structuredClone((
    await import("../../../../fixtures/product_preview/invented_preview_model.json")
  ).default) as PreviewModel;
}

async function loadKnowledgeFixture(): Promise<DesignKnowledge> {
  return (
    await import("../../../../fixtures/product_preview/invented_design_knowledge.json")
  ).default as DesignKnowledge;
}

async function loadMechanicsFixture(
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<MechanicsResult> {
  assertPreviewSolverMode(solverMode);
  const fixture = solverMode === "sparse_interactive"
    ? (await import("../../../../fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json")).default
    : (await import("../../../../fixtures/product_preview/invented_mechanics_result_precision_1_dense.json")).default;
  // Return a copy so one caller cannot change the imported producer evidence for
  // subsequent invocations. No header, row, dimension or quality is manufactured.
  return structuredClone(fixture) as MechanicsResult;
}

/** Validate mode evidence of a selected bundled producer result, not authenticate
 * an arbitrary caller's source/build or upgrade its numerical quality. */
export function validateBrowserMechanicsFixture(
  result: MechanicsResult,
  solverMode: PreviewSolverMode,
  fixtureModel: PreviewModel,
): MechanicsResult {
  assertPreviewSolverMode(solverMode);
  if (sourceContract(result) !== "precision") {
    throw new Error("BROWSER_FIXTURE_PRODUCER_CONTRACT_UNSUPPORTED");
  }
  if (result.model_ref !== fixtureModel.project.id) {
    throw new Error("BROWSER_FIXTURE_MODEL_BINDING_MISMATCH");
  }
  // A genuine blocked producer return can have no execution rows. Preserve that
  // failure as received; it supplies no completed mode/solve evidence to promote.
  if (result.status.mechanics !== "MECHANICS_SOLVED" && result.results.length === 0) return result;
  const cases = fixtureModel.load_cases.map(item => item.id);
  const rows = result.results.filter(item => item.kind === "linear_solver_mode_basis");
  const expectedValue = solverMode === "sparse_interactive" ? 1 : 2;
  const expectedBasis = solverMode === "sparse_interactive"
    ? "sparse_structural_integrity_primary" : "dense_structural_integrity_primary";
  const field = (basis: string, key: string) => basis.split(";")
    .map(part => part.trim()).filter(part => part.startsWith(`${key}=`));
  if (!cases.length || new Set(cases).size !== cases.length || rows.length !== cases.length
    || cases.some(id => rows.filter(row => row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === id).length !== 1)
    || rows.some(row => row.value !== expectedValue || row.unit !== "mode_code"
      || row.metadata?.component !== "linear_solver_mode"
      || row.metadata.coordinate_system !== "reduced_system"
      || row.metadata.location !== row.basis_ref?.ref_id
      || typeof row.metadata.basis !== "string"
      || field(row.metadata.basis, "solver_mode").join() !== `solver_mode=${solverMode}`
      || field(row.metadata.basis, "solution_basis").join() !== `solution_basis=${expectedBasis}`)) {
    throw new Error(`BROWSER_FIXTURE_SOLVER_MODE_BINDING_MISMATCH: ${solverMode}`);
  }
  return result;
}

export type BundledMechanicsReference = {
  standing: "reference_only";
  model: PreviewModel;
  source: MechanicsResult;
  provenance: {
    origin: "preserved_bundled_producer_record";
    fixture_ref: string;
    recorded_solver_mode: PreviewSolverMode;
    fresh_invocation_performed: false;
    current_use_eligible: false;
    notice: string;
  };
};

/** Explicit reference inspection, separate from the fresh solve/job APIs.
 * Provenance stays outside the unchanged producer data. Reading this record
 * does not mint a live solve invocation, Current, rule or export eligibility. */
export async function loadBundledMechanicsReference(
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<BundledMechanicsReference> {
  assertPreviewSolverMode(solverMode);
  const model = await loadModelFixture();
  const source = validateBrowserMechanicsFixture(await loadMechanicsFixture(solverMode), solverMode, model);
  return {
    standing: "reference_only",
    model,
    source,
    provenance: {
      origin: "preserved_bundled_producer_record",
      fixture_ref: solverMode === "sparse_interactive"
        ? "fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json"
        : "fixtures/product_preview/invented_mechanics_result_precision_1_dense.json",
      recorded_solver_mode: solverMode,
      fresh_invocation_performed: false,
      current_use_eligible: false,
      notice: "Preserved reference data; no solve was performed. This record is unavailable for Current, rule checks or qualified export.",
    },
  };
}

export const BROWSER_REFERENCE_SOLVE_DIAGNOSTIC = "BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY: Browser mechanics requires a real solver backend. Bundled records are available only through reference inspection; use the native application to solve.";
async function browserSolveUnavailable(model?: PreviewModel | null): Promise<Error> {
  if (model && canonicalJson(model) !== canonicalJson(await loadModelFixture())) {
    return new Error("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL: Browser reference data cannot solve an edited model; use the native backend for model-bound mechanics results.");
  }
  return new Error(BROWSER_REFERENCE_SOLVE_DIAGNOSTIC);
}

async function loadAgentProposalFixture(): Promise<AgentProposal> {
  return (
    await import("../../../../fixtures/product_preview/invented_agent_proposal.json")
  ).default as AgentProposal;
}

function ref(objectType: string, value: string): ObjectRef {
  return { object_type: objectType, ref: value };
}

function resultFamily(
  result: MechanicsResult["results"][number],
  sourceDimension: CanonicalResultDimension,
): string {
  const expectedDimension = declaredSourceResultDimension(result);
  if (sourceDimension !== expectedDimension) {
    throw new Error(
      `ANALYSIS-RUN-RESULT-FAMILY-DIMENSION-MISMATCH: ${result.id} declares ${sourceDimension}; exact kind semantics require ${expectedDimension}.`,
    );
  }
  if (
    sourceDimension === "force" &&
    (
      result.kind === "reaction_resultant" ||
      result.kind === "nonlinear_support_final_reaction" ||
      result.kind === "nonlinear_support_friction_normal_reaction_derived" ||
      result.kind === "nonlinear_support_observed_max_force_reaction_delta"
    )
  ) {
    return "reaction";
  }
  if (
    sourceDimension === "moment" &&
    result.kind === "nonlinear_support_observed_max_moment_reaction_delta"
  ) {
    return "reaction";
  }
  const families: Partial<Record<CanonicalResultDimension, string>> = {
    length: "displacement",
    angle: "rotation",
    force: "force",
    moment: "moment",
    stress: "stress",
    pressure: "stress",
    ratio: "ratio",
    dimensionless: "ratio",
  };
  return families[sourceDimension] ?? "TBD";
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
        .map(([key, nested]) => [key, sortJson(nested)]),
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
  selectedTarget?: SelectedReviewTarget | null,
): AgentProposal {
  const forceResult =
    result.results.find(
      (item) => item.id === "result:force:pipe-P-120:axial",
    ) ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  const primaryDiagnostic =
    result.diagnostics.find(
      (item) => item.severity === "warning" || item.severity === "blocking",
    ) ?? result.diagnostics[0];
  const targetRef =
    selectedTarget?.id ??
    forceResult?.id ??
    primaryDiagnostic?.id ??
    primaryDiagnostic?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";
  const targetKind =
    selectedTarget?.target_type.replaceAll("_", " ") ?? "computed mechanics";

  return {
    ...agentProposalFixture,
    proposal_id: "proposal:physics-diagnostic-review",
    prompt:
      "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
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
          after: `Attach review note referencing the current computed preview ${targetKind} context.`,
        },
      ],
    },
    rationale: `Generated from current preview mechanics context; selected review reference is ${targetRef}. This narrative is review-only and does not mutate accepted model state.`,
    validation: {
      ...agentProposalFixture.validation,
      unit_validation: "not_required_metadata_review_only",
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context",
    },
  };
}
