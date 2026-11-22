import type { ReportComposition } from "./reportComposition";
// File > Save Report Package… request (DEL-08-01 Design
// `report-package-current-results.md` §2.1-§2.5; FR-REP-001). The request is
// the shared producer's version 2.0.0 request (`core/reporting/report_package/src/wire.rs`):
// the rendered report input, the audit manifest, the DEL-08-04 0.3.0 result
// document that Export Result JSON saves, the received result it derives from,
// and the state/run and comparison sections. The native command screens and
// re-checks it before the save dialog (`src-tauri/src/report_package_save.rs`),
// and the shared producer binds its members (`report_package::bindings`).
//
// A Current pressure-free (preview-physics-2) result is admitted, and, since the owner
// opened exact-route packages (W-11, DEL-08-01 Design `exact-route-report-sections.md`
// §5.1; owner, 2026-10-26), a pressure-2 or pressure-source-1 result the reader reads with
// its mechanics solved, whatever its numerical standing, which the package's report states
// (DEL-08-01 REQ-024, "Package and user release"): the
// output policy's `report-package` cell admits the three routes, the identity check
// admits `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`, and the producer's Rust boundary admits
// the same three identities. The retained successors stay refused by the cell.
//
// W-11 (§3.1, §3.5, §5.5): `buildReportPackageRequest` is the cell check followed by the
// assembly with the production admitted set. An exact source is assembled from its
// Export Result JSON admission (whose document is the result member, so the package and
// the report name one admission), and `reportPackageExactSubmission` re-checks it
// immediately before IPC and yields the texts the native save binds.
// Standard claim fence applies (claims taxonomy per DEC-081).
import type { SessionAnalysisRun, SessionResult } from "../../services/nativeResult";
import { refuseSurfaceOutput, surfaceRouteRefusal } from "../results/outputPolicy";
import { PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import { buildCurrentResultExport, type JsonObject as ResultDocument } from "../result-export/resultExportAdapter";
import { reportStanding } from "../results/resultStanding";
import {
  admitExactReportSource,
  exactSourceSubmission,
  type ExactReportAdmission,
  type ExactReportAdmissionInputs,
  type ExactSourceSubmission
} from "./exactReportAdmission";
import { isExactReportSource } from "./exactReportSections";
import { isPfTimoshenkoReportSource } from "./pfTimoshenkoReport";
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
import { buildRenderableReportInput, userReleasedRenderableReportInput, type SessionRuleCheckRun, type UserReleaseRecordValue } from "./renderableReportInput";
import { reportResultRows } from "./reportResultRows";
import { buildStateComparisonHandoffSections } from "./stateComparisonHandoffSections";
import { currentLibraryRefFindings } from "../library/libraryRefFindings";

/** The desktop's export profile (`report_package::bindings::DESKTOP_PROFILE`). */
export const REPORT_PACKAGE_DESKTOP_PROFILE = "desktop_local_private_report_package_1";
/** The user-released export profile (W-18b, DEL-08-01 Design `report-user-release.md` §2.1;
 * `report_package::user_release::USER_RELEASED_DESKTOP_PROFILE`). */
export const REPORT_PACKAGE_USER_RELEASED_PROFILE = "desktop_user_released_report_package_1";
/** The result document's identity is not one the package admits. */
export const REPORT_PACKAGE_ROUTE_NOT_ADMITTED = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";
/** The semantic contracts the package admits: Rust's `ADMITTED_SEMANTIC_CONTRACTS`
 * (`report_package/src/document_form.rs`), kept equal by the exact-section parity vectors.
 * The exact identities were added when the owner opened exact-route packages (§5.1;
 * owner, 2026-10-26). */
export const REPORT_PACKAGE_ADMITTED_CONTRACT_IDS: readonly string[] = Object.freeze([
  PREVIEW_PHYSICS_CONTRACT_ID,
  // DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity, with its projection.
  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
  PRESSURE_SOURCE_CONTRACT_ID,
]);
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
 * `preview_physics` is ungated, the exact routes are admitted to the builder's
 * Export Result JSON admission (W-11 §5.1), both retained successors are refused with
 * their routes' reasons, and an unsupported statement falls to the builder's contract
 * check. */
export function reportPackageUnavailableReason(result: SessionResult | null | undefined): string | null {
  return surfaceRouteRefusal(result, "report-package");
}

/** The code of a package request whose pressure-free source is not numerically eligible (the
 * result-export builder's, `buildCurrentResultExport`; LIM-016). */
export const CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE = "CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE";

/** Why a solved result that is current in every other respect is not numerically eligible, for
 * the package refusals' words (DEL-08-01 REQ-024; LIM-016): "health" where some load case's solve
 * quality is not checks passed, which the report's numerical health states; "record" where every
 * load case reads checks passed and a case's record is still not the eligible one (its evidence
 * references, `reader_verdict.rs` `ordinary_case_eligible`), which the report states only in the
 * reader's finding. Read from the result's own numerical quality, so it is meaningful only for a
 * result already known not to be eligible. */
export type PackageStandingGround = "health" | "record";
export function packageStandingGround(result: Pick<MechanicsResult, "numerical_quality">): PackageStandingGround {
  const quality = result.numerical_quality;
  const cases = quality?.cases ?? [];
  return quality?.status !== "checks_passed" || cases.some((item) => item.solve_quality !== "checks_passed") ? "health" : "record";
}

/** What a person reads, by ground (one constant each, so that a word can change). */
export const PACKAGE_STANDING_WORDS: Readonly<Record<PackageStandingGround, Readonly<{ offered: string; reading: string; local: string }>>> = Object.freeze({
  health: Object.freeze({
    /** The report panel, in place of the two package buttons. */
    offered: "No report package for this result: its numerical health does not allow one. The report states it and can be printed or saved as HTML.",
    /** A refusal's reading, without its lead, full stop or tail. */
    reading: "this result's numerical health does not allow one",
    /** The shareable refusal's last sentence. */
    local: "A local private package can carry it when its numerical health allows."
  }),
  record: Object.freeze({
    offered: "No report package for this result: a load case's numerical record is incomplete. The report states it and can be printed or saved as HTML.",
    reading: "a load case's numerical record is incomplete",
    local: "A local private package can carry it when every load case's numerical record is complete."
  })
});
/** What a person can do instead, where the rendered report takes the result. */
export const REPORT_PACKAGE_INSTEAD_RENDERED = "The report can be printed or saved as HTML.";
/** The File menu's package save for a solve output the rendered report does not take and whose
 * ground is not known (not solved, not readable, an identity that is not Current, a standing not
 * yet read). */
export const REPORT_PACKAGE_NOT_CURRENT_ELIGIBLE = "Report package not saved: no report is available for this solve. Nothing was written.";
/** The File menu's package save where the model has no current result. */
export const REPORT_PACKAGE_NO_CURRENT_RESULT = "Report package not saved: the model has no current result. Solve it, then save. Nothing was written.";

/** A package request refused because its pressure-free source is not numerically eligible. The
 * refusal is data: `code` (`CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`), `ground`, `reading`
 * (the reason alone) and `instead` (what the person can do, or null); `message` is for logs. A
 * surface composes its own sentence from these (`reportPackageNotSavedText`). */
export class ReportPackageStandingRefusal extends Error {
  readonly code = CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE;
  readonly reading: string;
  constructor(readonly ground: PackageStandingGround, readonly instead: string | null) {
    super(`${CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE}: ${PACKAGE_STANDING_WORDS[ground].reading}`);
    this.name = "ReportPackageStandingRefusal";
    this.reading = PACKAGE_STANDING_WORDS[ground].reading;
  }
}

/** The package save status line's sentence for a standing refusal. */
export function reportPackageNotSavedText(refusal: Pick<ReportPackageStandingRefusal, "reading" | "instead">): string {
  return `Report package not saved: ${refusal.reading}. Nothing was written.${refusal.instead ? ` ${refusal.instead}` : ""}`;
}

/** The package controls' refusal for the rendered report's `source` (synchronous, for the menu
 * and the report panel): the route gate (`reportPackageUnavailableReason`), else, for a source
 * that is not `current` (the Current result) and is not an exact-route result (a pressure-free
 * Timoshenko result the rendered report takes, which is not eligible), its ground's words
 * (LIM-016), else null. */
export function reportPackageSourceUnavailableReason(source: SessionResult | null | undefined, current: SessionResult | null | undefined): string | null {
  const refused = reportPackageUnavailableReason(source);
  if (refused) return refused;
  if (source && source !== current && !isExactReportSource(source as MechanicsResult)) return PACKAGE_STANDING_WORDS[packageStandingGround(source)].offered;
  return null;
}

/** The identities a shareable (user-released, route `DREP-PACKAGE-USER-RELEASE-010`) package
 * admits: a pressure-free `preview-physics-2` result (record 1.0.0), and, since D-6 (owner,
 * 2026-10-29; DEL-08-01 Design `report-user-release.md` §11), a `pressure-2` or
 * `pressure-source-1` result (record 1.2.0) the reader reads with its mechanics solved,
 * whatever its numerical standing (DEL-08-01 REQ-024). Native admission and the derivation
 * (`report_package::user_release`) admit the same three and refuse the pressure-free
 * Timoshenko identity, which the local-private package admits. */
export const USER_RELEASED_ADMITTED_CONTRACT_IDS: readonly string[] = Object.freeze([
  PREVIEW_PHYSICS_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
  PRESSURE_SOURCE_CONTRACT_ID
]);

/** The route-010 cell (§2.2): synchronous, for the dialog. The package cell's refusal (the
 * retained routes), else a result whose identity a shareable package does not admit
 * (`USER_RELEASED_ADMITTED_CONTRACT_IDS`), else null. An exact-route result is admitted here;
 * its readability, solved mechanics and live custody are the builder's Export Result JSON
 * admission's and native admission's to decide. `ground` is the session's reason a result that is
 * not eligible is not (`packageStandingGround`), or null; its last sentence follows it. */
export function userReleasedReportPackageUnavailableReason(result: SessionResult | null | undefined, ground: PackageStandingGround | null = null): string | null {
  const refused = reportPackageUnavailableReason(result);
  if (refused) return refused;
  const identity = result?.producer?.semantic_contract_id;
  if (typeof identity === "string" && !USER_RELEASED_ADMITTED_CONTRACT_IDS.includes(identity)) {
    return `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: a shareable report package carries pressure-free (preview-physics-2) results and exact pressure route (pressure-3, pressure-source-1) results; this result is ${identity}. ${PACKAGE_STANDING_WORDS[ground ?? "health"].local}`;
  }
  return null;
}

export type ReportPackageRequestInput = {
  reportComposition?: ReportComposition | null;
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

/** The result document of a pressure-free source: the result-export builder's, whose standing
 * must be eligible. Where it refuses a pressure-free Timoshenko result that the rendered report
 * takes (solved and readable: `reportStanding`, as the builder's "report" purpose reads it), the
 * refusal is a `ReportPackageStandingRefusal` with its ground; any other refusal is unchanged. */
async function pressureFreeResultDocument(inputs: Pick<ExactReportAdmissionInputs, "model" | "result" | "analysisRun" | "inputManifest">): Promise<ResultDocument> {
  try {
    return await buildCurrentResultExport(inputs);
  } catch (error) {
    if (error instanceof Error && error.message === CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE && isPfTimoshenkoReportSource(inputs.result) && reportedSolvedAndReadable(inputs)) {
      throw new ReportPackageStandingRefusal(packageStandingGround(inputs.result), REPORT_PACKAGE_INSTEAD_RENDERED);
    }
    throw error;
  }
}

function reportedSolvedAndReadable({ model, result }: Pick<ExactReportAdmissionInputs, "model" | "result">): boolean {
  try {
    const { readability } = reportStanding(result, model);
    return readability.readable && readability.solved;
  } catch {
    return false;
  }
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

  // An exact source takes the Export Result JSON admission for a report (any solved result
  // the reader reads; DEL-08-01 REQ-024); its document is the result member. A pressure-free
  // source keeps the result-export builder's document and its eligible standing.
  const exactInputs: ExactReportAdmissionInputs | null = isExactReportSource(result) ? { model, result, analysisRun, inputManifest, purpose: "report" } : null;
  const admission = exactInputs ? await admitExactReportSource(exactInputs) : null;
  const document: ResultDocument = admission ? admission.document : await pressureFreeResultDocument({ model, result, analysisRun, inputManifest });
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
    libraryRefFindings: currentLibraryRefFindings(model),
    reportComposition: input.reportComposition
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


/** The inputs of a user-released request (W-18b, DEL-08-01 Design `report-user-release.md`
 * §2.3 "Session", §2.6): the local-private basis without the comparison, project summary,
 * rule-check aggregate and run, and the solve's own transient run record. */
export type UserReleasedReportPackageRequestInput = {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  inputManifest: CurrentSessionInputManifestEvidence;
  userRelease: UserReleaseRecordValue;
  /** The session composition C (slice C4), or none for the full report. */
  reportComposition?: ReportComposition | null;
};

/** The user-released request's assembly (§2.6; scope R, §2.1): the Export Result JSON
 * document of an eligible pressure-free result or of an exact result the reader reads with
 * its mechanics solved, whatever its numerical standing (DEL-08-01 REQ-024), the minimal renderer input, an
 * audit manifest with no rule-pack reference or asset, and no state/comparison/handoff
 * record. The run record carries the solve's own rule status. The native host derives the
 * same request from its bound basis and requires canonical equality
 * (`report_package::user_release::derive`; vectors
 * `fixtures/reports/invented/user_release_derivation_vectors.json`). This is the assembly
 * only: it grants nothing. The route's own cell is `userReleasedReportPackageUnavailableReason`,
 * and `buildUserReleasedReportPackageRequest` is the cell, the early model check and this
 * assembly with the record bound to the session (`UserReleaseDialog.tsx` asks for it). */
export async function assembleUserReleasedReportPackageRequest(input: UserReleasedReportPackageRequestInput) {
  const { userRelease, ...basis } = input;
  return assembleUserReleased(basis, () => userRelease);
}

async function assembleUserReleased(
  input: Omit<UserReleasedReportPackageRequestInput, "userRelease">,
  recordFor: (document: ResultDocument) => Promise<UserReleaseRecordValue> | UserReleaseRecordValue
) {
  const { model, result, analysisRun, inputManifest, reportComposition = null } = input;
  const exactInputs: ExactReportAdmissionInputs | null = isExactReportSource(result) ? { model, result, analysisRun, inputManifest, purpose: "report" } : null;
  const admission = exactInputs ? await admitExactReportSource(exactInputs) : null;
  const document: ResultDocument = admission ? admission.document : await pressureFreeResultDocument({ model, result, analysisRun, inputManifest });
  const envelope = document.result_envelope as JsonRecord;
  const identity = envelope?.producer?.semantic_contract_id;
  if (!USER_RELEASED_ADMITTED_CONTRACT_IDS.includes(identity)) {
    throw new Error(
      `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: a shareable report package admits ${USER_RELEASED_ADMITTED_CONTRACT_IDS.join(", ")}; this document is ${String(identity ?? "unidentified")}.`
    );
  }
  const userRelease = await recordFor(document);
  const run = analysisRun.analysis_run;
  const recordedTokens = run.analysis_status.filter((status) => RULE_TOKENS.has(status));
  if (recordedTokens.length !== 1 || recordedTokens[0] !== result.status.rule_check) {
    throw ruleCheckMismatch(`the analysis record's rule status (${recordedTokens.join(", ") || "none"}) is not the solve's (${result.status.rule_check}).`);
  }
  const report = await userReleasedRenderableReportInput({ model, result, analysisRun, inputManifest, userRelease, exactAdmission: admission, reportComposition });
  if (JSON.stringify(report.result_rows) !== JSON.stringify(reportResultRows(result))) {
    throw new Error(`${REPORT_PACKAGE_REPORT_ROWS_MISMATCH}: the report's result rows are not the source rows' projection.`);
  }
  const modelHash = await canonicalSha256Hex(model);
  if (envelope.reproducibility?.model_hash?.value !== modelHash) {
    throw new Error("REPORT-PACKAGE-MODEL-HASH-MISMATCH: the result document's model hash is not the session model's.");
  }
  const solver = envelope.solver_version as { solver_name: string; solver_version: string; solver_build_ref: string };
  const loadBasisRefs = (envelope.load_basis_refs as Array<{ ref_type: string; ref_id: string }>).map((item) => reference(item.ref_type, item.ref_id));
  const manifestRef = inputManifest.manifest_ref.ref;
  const request = {
    package_id: `desktop-user-released-report-${safeId(result.run_id)}`,
    export_profile_id: REPORT_PACKAGE_USER_RELEASED_PROFILE,
    source_model_ref: reference("model", model.project.id),
    source_basis_refs: [reference("analysis_run", run.run_id), ...loadBasisRefs],
    report,
    audit_manifest: {
      manifest_id: manifestRef,
      model_hash: { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_kind: "model_json", payload_ref: model.project.id, value: modelHash },
      input_manifest_hash: { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_kind: "input_manifest_json", payload_ref: manifestRef, value: inputManifest.manifest_sha256 },
      solver_version: { solver_name: solver.solver_name, solver_version: solver.solver_version, solver_build_ref: solver.solver_build_ref },
      unit_system_ref: envelope.unit_system_ref.ref_id as string,
      rule_pack_refs: [],
      assets: [],
      professional_boundary: PROFESSIONAL_BOUNDARY
    },
    result_documents: [document],
    source_results: [result],
    state_comparison_handoff_records: [],
    rule_check_aggregate: null,
    solve_rule_check_status: result.status.rule_check
  };
  if (admission && exactInputs) {
    exactRequests.set(request, { admission, inputs: exactInputs });
    exactSourceSubmission(admission, exactInputs);
  }
  return request;
}

/** A model with a detectable library origin (§2.7, rules 1-3). */
export const REPORT_USER_RELEASE_LIBRARY_REFERENCED = "REPORT-USER-RELEASE-LIBRARY-REFERENCED";
/** A model with a rule-pack reference, or a run whose input manifest records an active pack (§2.7). */
export const REPORT_USER_RELEASE_RULE_PACK_REFERENCED = "REPORT-USER-RELEASE-RULE-PACK-REFERENCED";
/** The confirmation time is not of the record's form (§2.3, `YYYY-MM-DDTHH:MM:SSZ`). */
export const REPORT_USER_RELEASE_RECORD_INVALID = "REPORT-USER-RELEASE-RECORD-INVALID";

const LIBRARY_PROVENANCE_PREFIXES = ["library:material:", "library:section:", "library:component:", "library:hanger:"];
const LEGACY_HANGER_SELECTION = "user_imported_hanger_selection";
const LEGACY_EVIDENCE_MEMBERS = ["library_kind", "library_id", "library_metadata", "record_snapshot"];

function jsonPointerToken(key: string): string {
  return key.replaceAll("~", "~0").replaceAll("/", "~1");
}

/** The first site of `found` in a document-order walk of `value`, or null. */
function firstSite(value: unknown, found: (key: string | null, child: unknown) => boolean, pointer = ""): string | null {
  if (Array.isArray(value)) {
    for (let index = 0; index < value.length; index += 1) {
      const at = `${pointer}/${index}`;
      if (found(null, value[index])) return at;
      const inner = firstSite(value[index], found, at);
      if (inner !== null) return inner;
    }
    return null;
  }
  if (value === null || typeof value !== "object") return null;
  for (const [key, child] of Object.entries(value)) {
    const at = `${pointer}/${jsonPointerToken(key)}`;
    if (found(key, child)) return at;
    const inner = firstSite(child, found, at);
    if (inner !== null) return inner;
  }
  return null;
}

const holdsLibraryRefMember = (value: unknown) => firstSite(value, (key) => key === "library_ref") !== null;
const libraryProvenanceText = (text: string) => LIBRARY_PROVENANCE_PREFIXES.some((prefix) => text.includes(prefix));
const textSite = (value: unknown, test: (text: string) => boolean) => firstSite(value, (_, child) => typeof child === "string" && test(child));

/** Rule 3 (b): a string that, trimmed of JSON whitespace, begins with `{` and parses to an
 * object holding legacy evidence members or a rule 1, 2 or 3 (a) site, one level deep. */
function embeddedLegacyEvidence(text: string): boolean {
  const trimmed = text.replace(/^[ \t\n\r]+|[ \t\n\r]+$/g, "");
  if (!trimmed.startsWith("{")) return false;
  let parsed: unknown;
  try { parsed = JSON.parse(trimmed); } catch { return false; }
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) return false;
  return LEGACY_EVIDENCE_MEMBERS.some((member) => Object.hasOwn(parsed as object, member))
    || holdsLibraryRefMember(parsed)
    || textSite(parsed, (inner) => libraryProvenanceText(inner) || inner.includes(LEGACY_HANGER_SELECTION)) !== null;
}

/** The webview's early model check for a shareable package (§2.2, §2.7): the first library
 * site (rule 1, then rule 2, then rule 3, each in document order), else the first rule-pack
 * reference in the model or an active pack in the run's input manifest, else null. Native
 * admission decides; this only lets the dialog say early which models cannot be shared yet. */
export function userReleaseModelRefusal(model: unknown, inputManifest?: CurrentSessionInputManifestEvidence | null): string | null {
  const library =
    firstSite(model, (key) => key === "library_ref") ??
    textSite(model, libraryProvenanceText) ??
    textSite(model, (text) => text.includes(LEGACY_HANGER_SELECTION) || embeddedLegacyEvidence(text));
  if (library !== null) {
    return `${REPORT_USER_RELEASE_LIBRARY_REFERENCED}: the model carries a library reference at ${library || "/"}; a shareable package does not yet carry library record values.`;
  }
  const rulePack = firstSite(model, (key, child) => key === "rule_pack_refs" && Array.isArray(child) && child.length > 0);
  if (rulePack !== null || (inputManifest?.manifest.active_rule_packs ?? []).length > 0) {
    return `${REPORT_USER_RELEASE_RULE_PACK_REFERENCED}: ${rulePack !== null ? `the model carries a rule-pack reference at ${rulePack}` : "the run's input manifest records an active rule pack"}; a shareable package does not yet carry rule-pack checks.`;
  }
  return null;
}

/** The record's constants (§2.3), as `report_renderer`'s `USER_RELEASE_*` constants. */
export const USER_RELEASE_STATEMENT = "I hold the right to share this project's data, and I release my project data in this report for sharing.";
const USER_RELEASE_STATEMENT_BASIS = "docs/IP_AND_DATA_BOUNDARY.md §6: a user intentionally exports their own data with documented redistribution rights";
/** What a user-released package does not carry, on every record version. None names a result
 * as not carried on the ground of its numerical standing (DEL-08-01 REQ-024). */
const USER_RELEASE_NOT_CARRIED = ["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"];
/** Record 1.0.0 (pressure-free) and the exact-route record 1.2.0 (D-6, §11; widened by DEL-08-01
 * REQ-024 to every exact-route result the rendered report admits). */
const USER_RELEASE_RECORD_VERSION = "1.0.0";
const USER_RELEASE_EXACT_RECORD_VERSION = "1.2.0";

/** The application clock's reading as the record's `user_released_at` (UTC, whole seconds). */
export function userReleasedAtOf(date: Date): string {
  return date.toISOString().replace(/\.\d{3}Z$/, "Z");
}

/** The user's record U (§2.3) for one report, bound to the session's identities and the
 * result document the package carries. Record 1.2.0 for an exact-route document (§11), else
 * 1.0.0. Native admission re-derives and binds every member but `user_released_at`. */
export async function userReleaseRecord(args: {
  model: PreviewModel;
  result: MechanicsResult;
  inputManifest: CurrentSessionInputManifestEvidence;
  document: ResultDocument;
  userReleasedAt: string;
}): Promise<UserReleaseRecordValue> {
  const { model, result, inputManifest, document, userReleasedAt } = args;
  if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/.test(userReleasedAt)) {
    throw new Error(`${REPORT_USER_RELEASE_RECORD_INVALID}: user_released_at is not YYYY-MM-DDTHH:MM:SSZ.`);
  }
  const envelope = document.result_envelope as JsonRecord;
  const exact = isExactReportSource(result);
  const run = result.run_id;
  return {
    record_kind: "user_release_of_own_project_data",
    record_version: exact ? USER_RELEASE_EXACT_RECORD_VERSION : USER_RELEASE_RECORD_VERSION,
    statement: USER_RELEASE_STATEMENT,
    statement_basis: USER_RELEASE_STATEMENT_BASIS,
    standing: "user_statement_of_data_rights_only",
    scope: {
      scope_kind: "this_report_only",
      report_id: `report:${run}`,
      package_id: `desktop-user-released-report-${safeId(run)}`,
      user_released_data_class: "private_project_data",
      not_carried: [...USER_RELEASE_NOT_CARRIED]
    },
    bound_to: {
      model_ref: model.project.id,
      model_document_sha256: await canonicalSha256Hex(model),
      analysis_run_ref: run,
      input_manifest_ref: inputManifest.manifest_ref.ref,
      input_manifest_sha256: inputManifest.manifest_sha256,
      result_envelope_id: envelope.envelope_id,
      result_document_derivative_sha256: envelope.reproducibility.derivative_hash.value
    },
    user_released_at: userReleasedAt
  };
}

/** The inputs of `buildUserReleasedReportPackageRequest`: the user-released basis (§2.3
 * "Session") and the confirmation time; the record is bound here, not by the caller. */
export type UserReleasedReportPackageBuildInput = Omit<UserReleasedReportPackageRequestInput, "userRelease"> & {
  userReleasedAt: string;
};

/** Builds the route-010 request (§2.2, §2.6): the user-release cell, the early model check,
 * the package surface's route check, then the assembly with the record bound to the
 * session and the document it carries. Refusals throw with their own codes; nothing here
 * grants export authority (native admission derives the whole request and decides). */
export async function buildUserReleasedReportPackageRequest(input: UserReleasedReportPackageBuildInput) {
  const { model, result, inputManifest, userReleasedAt, ...rest } = input;
  const unavailable = userReleasedReportPackageUnavailableReason(result);
  if (unavailable) throw new Error(unavailable);
  const modelRefusal = userReleaseModelRefusal(model, inputManifest);
  if (modelRefusal) throw new Error(modelRefusal);
  refuseSurfaceOutput(result, model, "report-package");
  return assembleUserReleased({ model, result, inputManifest, ...rest }, (document) =>
    userReleaseRecord({ model, result, inputManifest, document, userReleasedAt })
  );
}
