// File > Save Report Package… request (DEL-08-01 Design
// `report-package-current-results.md` §2.1-§2.5; FR-REP-001). The request is
// the shared producer's version 2.0.0 request (`core/reporting/report_package/src/wire.rs`):
// the rendered report input, the audit manifest, the DEL-08-04 0.3.0 result
// document that Export Result JSON saves, the received result it derives from,
// and the state/run and comparison sections. The native command screens and
// re-checks it before the save dialog (`src-tauri/src/report_package_save.rs`),
// and the shared producer binds its members (`report_package::bindings`).
//
// Only a Current pressure-free (preview-physics-2) result is admitted. The
// exact routes stay refused by the output policy's `report-package` cell
// (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21), by the
// identity check against `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`, and by the
// producer's Rust boundary, which admits only preview-physics-2.
//
// W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.1, §3.5, §5.5):
// `buildReportPackageRequest` is the cell check followed by the assembly with the
// production admitted set. Behind them, an exact source is assembled from its Export
// Result JSON admission (whose document is the result member, so the package and the
// report name one admission), and `reportPackageExactSubmission` re-checks it
// immediately before IPC and yields the texts the native save binds. Tests reach the
// assembly below the gate through `assembleReportPackageRequestBelowOutputGate`; the
// Rust boundary still refuses whatever it produces.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { refuseSurfaceOutput, surfaceRouteRefusal } from "../results/outputPolicy";
import { PREVIEW_PHYSICS_CONTRACT_ID } from "../results/numericalResultQuality";
import { buildCurrentResultExport, type JsonObject as ResultDocument } from "../result-export/resultExportAdapter";
import {
  admitExactReportSource,
  exactSourceSubmission,
  type ExactReportAdmission,
  type ExactReportAdmissionInputs,
  type ExactSourceSubmission
} from "./exactReportAdmission";
import { isExactReportSource } from "./exactReportSections";
import type { PreviewComparisonUnavailable } from "../comparison/withinRunComparison";
import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewComparison,
  PreviewModel
} from "../../types";
import { canonicalSha256Hex } from "../../services/hashService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { buildRenderableReportInput, type SessionRuleCheckRun } from "./renderableReportInput";
import { reportResultRows } from "./reportResultRows";
import { buildStateComparisonHandoffSections } from "./stateComparisonHandoffSections";
import { currentLibraryRefFindings } from "../library/libraryRefFindings";

/** The desktop's export profile (`report_package::bindings::DESKTOP_PROFILE`). */
export const REPORT_PACKAGE_DESKTOP_PROFILE = "desktop_local_private_report_package_1";
/** The result document's identity is not the package's admitted one (preview-physics-2). */
export const REPORT_PACKAGE_ROUTE_NOT_ADMITTED = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";
/** The semantic contracts the package admits: Rust's `ADMITTED_SEMANTIC_CONTRACTS`
 * (`report_package/src/document_form.rs`), kept equal by the exact-section parity vectors.
 * Exact identities are added only when the owner opens exact-route packages (§5.1). */
export const REPORT_PACKAGE_ADMITTED_CONTRACT_IDS: readonly string[] = Object.freeze([PREVIEW_PHYSICS_CONTRACT_ID]);
/** The rule-check aggregate, the rule-check run and the analysis record do not name one rule check. */
export const REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH = "REPORT-PACKAGE-RULE-CHECK-BINDING-MISMATCH";
/** A rule-pack checksum whose declared pair has no audit basis the package can name (§2.4). */
export const REPORT_PACKAGE_RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED = "REPORT-PACKAGE-RULE-PACK-CHECKSUM-BASIS-UNSUPPORTED";
/** The package's rule packs (the rule-check run's) are not the packs the solve recorded (§2.5). */
export const REPORT_PACKAGE_RULE_PACKS_DISAGREE = "REPORT-PACKAGE-RULE-PACKS-DISAGREE";
/** The report's result rows are not the source rows' projection (§2.3 item 6). */
export const REPORT_PACKAGE_REPORT_ROWS_MISMATCH = "REPORT-PACKAGE-REPORT-ROWS-MISMATCH";

const RULE_TOKENS = new Set(["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"]);
const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false
};

/** The route gate (§2.1): the output policy's `report-package` cell. Synchronous, for
 * the menu and the report panel. Null admits the route to the builder's own checks:
 * `preview_physics` is ungated, the exact routes and both retained successors are
 * refused with their routes' reasons, and an unsupported statement falls to the
 * builder's contract check. */
export function reportPackageUnavailableReason(result: MechanicsResult | null | undefined): string | null {
  return surfaceRouteRefusal(result, "report-package");
}

export type ReportPackageRequestInput = {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  inputManifest: CurrentSessionInputManifestEvidence;
  projectSummary: LocalProjectSummary | null;
  comparison: PreviewComparison | null;
  /** W-12 S3c: the record of a chosen within-run comparison the builder refused;
   * carried beside the sections only when present. */
  comparisonUnavailable?: PreviewComparisonUnavailable | null;
  ruleCheckAggregate: string | null;
  /** The session's rule-check run, exposed only while its aggregate is current. */
  ruleCheckRun: SessionRuleCheckRun | null;
};

type JsonRecord = Record<string, any>;

function reference(refType: string, refId: string) {
  return { ref_type: refType, ref_id: refId };
}

function safeId(value: string): string {
  return value.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "report-package";
}

function ruleCheckMismatch(what: string): Error {
  return new Error(`${REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH}: ${what}`);
}

type RulePackIdentity = { rule_pack_id: string; version: string; checksum: string };

function identityText(set: readonly RulePackIdentity[]): string {
  return set.length === 0
    ? "none"
    : set.map((pack) => `${pack.rule_pack_id}@${pack.version} (sha256 ${pack.checksum})`).join(", ");
}

/** Solve and rule-check rule packs agree (§2.5, decided): the package's set (the
 * rule-check run's pack, else empty) equals the input manifest's `active_rule_packs`,
 * compared as sets of `{rule_pack_id, version, checksum value}`. This is the early
 * refusal; the native command re-checks the same rule against the input-manifest
 * text bound to the audit input-manifest hash (`report_package_save.rs`). */
export function checkRulePacksAgree(
  packageRefs: ReadonlyArray<{ rule_pack_id: string; version: string; checksum?: { value?: string } | null }>,
  activeRulePacks: ReadonlyArray<{ rule_pack_id: string; version: string; checksum_sha256: string }>
): void {
  const packageSet = packageRefs.map((pack) => ({ rule_pack_id: pack.rule_pack_id, version: pack.version, checksum: pack.checksum?.value ?? "not recorded" }));
  const solveSet = activeRulePacks.map((pack) => ({ rule_pack_id: pack.rule_pack_id, version: pack.version, checksum: pack.checksum_sha256 }));
  const key = (pack: RulePackIdentity) => JSON.stringify([pack.rule_pack_id, pack.version, pack.checksum]);
  const packageKeys = new Set(packageSet.map(key));
  const solveKeys = new Set(solveSet.map(key));
  const equal = packageKeys.size === solveKeys.size && [...packageKeys].every((item) => solveKeys.has(item));
  if (!equal) {
    throw new Error(
      `${REPORT_PACKAGE_RULE_PACKS_DISAGREE}: the package's rule packs (the rule check's: ${identityText(packageSet)}) are not the packs the solve recorded (${identityText(solveSet)}). Run the rule check with the model's pack, or change the model's rule-pack references and solve again.`
    );
  }
}

/** The safe-reference projection of the report's rule-pack reference to the audit
 * manifest's `RulePackAuditRef` (§2.4). Any other declared checksum pair refuses. */
function auditRulePackRef(pack: JsonRecord) {
  const checksum = pack.checksum ?? {};
  const id = pack.rule_pack_id as string;
  const algorithm = typeof checksum.algorithm === "string" ? checksum.algorithm : "not recorded";
  const basis = typeof checksum.canonicalization === "string" ? checksum.canonicalization : "not recorded";
  if (checksum.payload_ref?.ref_id !== id) {
    throw ruleCheckMismatch(`the rule pack's checksum names another payload than rule pack ${id}.`);
  }
  let projected: { canonicalization: string; payload_ref: string };
  if (algorithm === "sha256" && basis === "JCS" && checksum.payload_ref.ref_type === "rule_pack") {
    projected = { canonicalization: "rule_pack_payload_jcs", payload_ref: id };
  } else if (algorithm === "sha256" && basis === "rfc8785_jcs" && checksum.payload_ref.ref_type === "rule_pack_document") {
    projected = { canonicalization: "rfc8785_jcs", payload_ref: `rule_pack_document:${id}` };
  } else {
    throw new Error(
      `${REPORT_PACKAGE_RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED}: the rule pack's checksum ${algorithm}/${basis} has no audit basis the package can name.`
    );
  }
  return {
    rule_pack_id: id,
    rule_pack_version: pack.version,
    source_notice: pack.source_notice,
    redistribution_status: pack.redistribution_status,
    checksum: {
      algorithm: "sha256",
      canonicalization: projected.canonicalization,
      payload_kind: "rule_pack_reference",
      payload_ref: projected.payload_ref,
      value: checksum.value
    },
    private_payload_redacted: true
  };
}

/** Requests assembled from an exact source, with the admission each was built from
 * and its inputs (module-private: only the assembly records one). */
const exactRequests = new WeakMap<object, { admission: ExactReportAdmission; inputs: ExactReportAdmissionInputs }>();

/** Builds the version 2.0.0 request (§2.1 admission, §2.3, §2.4, §2.5): the
 * `report-package` cell, then the assembly with the production admitted set. Refusals
 * throw with their own codes; nothing here grants export authority (the route control
 * and the native admission decide). */
export async function buildReportPackageRequest(input: ReportPackageRequestInput) {
  const { model, result } = input;
  const unavailable = reportPackageUnavailableReason(result);
  if (unavailable) throw new Error(unavailable);
  // The result-export builder gates on its own cell, which admits the exact routes,
  // so the package's own cell is applied first.
  refuseSurfaceOutput(result, model, "report-package");
  return assembleReportPackageRequest(input, REPORT_PACKAGE_ADMITTED_CONTRACT_IDS);
}

/** The pre-IPC re-check of a request assembled from an exact source (§3.1, §3.6):
 * synchronous, immediately before `save_report_package`; returns the texts the native
 * save re-reads, or null for a request from a pressure-free source. Throws once the
 * admission no longer holds (`REPORT-EXACT-ADMISSION-STALE`). */
export function reportPackageExactSubmission(request: object): ExactSourceSubmission | null {
  const exact = exactRequests.get(request);
  return exact ? exactSourceSubmission(exact.admission, exact.inputs) : null;
}

/** The assembly after the cell (§5.5): `admittedContractIds` is the identity check's
 * set, `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` in production. */
async function assembleReportPackageRequest(input: ReportPackageRequestInput, admittedContractIds: readonly string[]) {
  const { model, result, analysisRun, inputManifest, projectSummary, comparison, comparisonUnavailable = null, ruleCheckAggregate, ruleCheckRun } = input;

  // Rule-check threading (§2.5): aggregate present exactly when the run is, and the
  // run's aggregate is the session's.
  if ((ruleCheckAggregate === null) !== (ruleCheckRun === null)) {
    throw ruleCheckMismatch("the rule-check aggregate and the rule-check run must be present together.");
  }
  if (ruleCheckRun && ruleCheckRun.result.aggregate_status !== ruleCheckAggregate) {
    throw ruleCheckMismatch("the rule-check run's aggregate is not the session's aggregate.");
  }

  // An exact source takes the Export Result JSON admission; its document is the result
  // member. A pressure-free source keeps the result-export builder's document.
  const exactInputs = isExactReportSource(result) ? { model, result, analysisRun, inputManifest } : null;
  const admission = exactInputs ? await admitExactReportSource(exactInputs) : null;
  const document: ResultDocument = admission ? admission.document : await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
  const envelope = document.result_envelope as JsonRecord;
  const identity = envelope?.producer?.semantic_contract_id;
  if (typeof identity !== "string" || !admittedContractIds.includes(identity)) {
    throw new Error(
      `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: the report package admits only ${admittedContractIds.join(", ")} results; this document is ${String(identity ?? "unidentified")}.`
    );
  }
  const run = analysisRun.analysis_run;
  const recordedTokens = run.analysis_status.filter((status) => RULE_TOKENS.has(status));
  const expectedToken = ruleCheckAggregate ?? result.status.rule_check;
  if (recordedTokens.length !== 1 || recordedTokens[0] !== expectedToken) {
    throw ruleCheckMismatch(
      `the analysis record's rule status (${recordedTokens.join(", ") || "none"}) is not ${ruleCheckAggregate === null ? "the solve's" : "the rule-check aggregate"} (${expectedToken}).`
    );
  }

  // The package route is always local-private (DREP-PACKAGE-SAVE-009); its members
  // carry no generation time, so identical inputs stay byte-identical.
  const report = await buildRenderableReportInput({
    model,
    result,
    analysisRun,
    projectSummary,
    inputManifest,
    ruleCheckRun,
    renderContext: { exportContext: "local_private", generatedAt: null },
    exactAdmission: admission,
    libraryRefFindings: currentLibraryRefFindings(model)
  });
  if (JSON.stringify(report.result_rows) !== JSON.stringify(reportResultRows(result))) {
    throw new Error(`${REPORT_PACKAGE_REPORT_ROWS_MISMATCH}: the report's result rows are not the source rows' projection.`);
  }
  const reportPacks = report.calculation_report.rule_pack_refs as JsonRecord[];
  checkRulePacksAgree(reportPacks as never, inputManifest.manifest.active_rule_packs ?? []);
  const auditRulePacks = reportPacks.map(auditRulePackRef);

  const modelHash = await canonicalSha256Hex(model);
  if (envelope.reproducibility?.model_hash?.value !== modelHash) {
    throw new Error("REPORT-PACKAGE-MODEL-HASH-MISMATCH: the result document's model hash is not the session model's.");
  }
  const solver = envelope.solver_version as { solver_name: string; solver_version: string; solver_build_ref: string };
  const loadBasisRefs = (envelope.load_basis_refs as Array<{ ref_type: string; ref_id: string }>).map((item) => reference(item.ref_type, item.ref_id));
  const manifestRef = inputManifest.manifest_ref.ref;

  const request = {
    package_id: `desktop-report-${safeId(result.run_id)}`,
    export_profile_id: REPORT_PACKAGE_DESKTOP_PROFILE,
    source_model_ref: reference("model", model.project.id),
    source_basis_refs: [reference("analysis_run", run.run_id), ...loadBasisRefs],
    report,
    audit_manifest: {
      // §2.4: named by the run's input-manifest reference, as the result document's
      // `audit_manifest_ref` names it.
      manifest_id: manifestRef,
      model_hash: {
        algorithm: "sha256",
        canonicalization: "rfc8785_jcs",
        payload_kind: "model_json",
        payload_ref: model.project.id,
        value: modelHash
      },
      input_manifest_hash: {
        algorithm: "sha256",
        canonicalization: "rfc8785_jcs",
        payload_kind: "input_manifest_json",
        payload_ref: manifestRef,
        value: inputManifest.manifest_sha256
      },
      solver_version: { solver_name: solver.solver_name, solver_version: solver.solver_version, solver_build_ref: solver.solver_build_ref },
      unit_system_ref: envelope.unit_system_ref.ref_id as string,
      rule_pack_refs: auditRulePacks,
      assets: [],
      professional_boundary: PROFESSIONAL_BOUNDARY
    },
    result_documents: [document],
    source_results: [result],
    state_comparison_handoff_records: [
      buildStateComparisonHandoffSections({ model, result, analysisRun, comparison, comparisonUnavailable, modelHash })
    ],
    rule_check_aggregate: ruleCheckAggregate,
    solve_rule_check_status: result.status.rule_check
  };
  if (admission && exactInputs) {
    exactRequests.set(request, { admission, inputs: exactInputs });
    // After the last await: the admission still holds.
    exactSourceSubmission(admission, exactInputs);
  }
  return request;
}

/** Test-only name of the assembly, which production reaches only after the
 * `report-package` cell (never for an exact result before opening). */
export const assembleReportPackageRequestBelowOutputGate = assembleReportPackageRequest;
