import { analysisResultHashScope } from "../results/analysisResultHashScope";
import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewComponent,
  PreviewModel
} from "../../types";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import {
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence
} from "../../services/inputManifestService";
import type { RulePackDocument } from "../../services/rulePackService";
import type { RuleCheckRunResult, RuleCheckStatus } from "../../services/ruleCheckService";
import desktopAppConfig from "../../../src-tauri/tauri.conf.json";
import type { NumericalHealthView } from "../results/numericalHealth";
import { reportLineText } from "../results/numericalHealthText";
import { reportResultRows } from "./reportResultRows";
import { checkExactReportAdmission, REPORT_EXACT_ADMISSION_BINDING_MISMATCH, type ExactReportAdmission } from "./exactReportAdmission";
import { exactLimitationEntries, exactRouteCore, isExactReportSource } from "./exactReportSections";

// DEC-021 (A7): assemble the renderer input document from session state.
// The frontend composes only references, disclosures, and display rows — the
// Rust renderer crate owns document composition, gating, and hashing. Every
// value here is either copied from session envelopes, computed locally with
// the same canonical-JSON/sha256 discipline as the run hashes, or an explicit
// `TBD` marker that stays visible in the rendered document.

const KNOWN_ANALYSIS_STATUSES = new Set([
  "MODEL_INCOMPLETE",
  "MECHANICS_SOLVED",
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED",
  "HUMAN_REVIEW_REQUIRED"
]);

const SECTION_KINDS = [
  "model_input_summary",
  "load_cases",
  "results",
  "warnings_assumptions_provenance",
  "audit_manifest",
  "rule_pack_references",
  "limitations",
  "professional_boundary_notice"
] as const;

const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false
};

export const DEC018_UNIT_SYSTEM_REF = {
  ref_type: "unit_system",
  ref_id: "unit-system:dec-018-si-dual-display"
};

type Ref = { ref_type: string; ref_id: string };

/** Export context the wrapper renders under. Set only by the wrapper's own
 * route control and intent; it records the outcome in the document and
 * grants no export authority (the redaction route projector decides). */
export type ReportExportContext = "public_report" | "local_private";

/** The rule-check run held by the session for the current result: the loaded
 * rule-pack document and the backend run outcome. Only safe metadata is read
 * from the document (identity, version, checksum, source notice,
 * redistribution status) and only identity, status and missing-input names
 * from the outcomes (DEL-08-01 R-08-01-008). */
export type SessionRuleCheckRun = {
  rulePackDocument: RulePackDocument;
  result: RuleCheckRunResult;
};

export type ReportRenderContext = {
  exportContext: ReportExportContext;
  /** Application clock reading when the render was requested; null when the
   * caller records none. The renderer never reads a clock. */
  generatedAt: string | null;
};

const NOT_RECORDED = null;
const REPORT_REDISTRIBUTION_STATUSES = new Set([
  "public_permissive",
  "private_only",
  "unknown",
  "protected_suspected",
  "invented_non_engineering_example",
  "TBD"
]);
const RULE_CHECK_STATUSES = new Set<RuleCheckStatus>([
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED"
]);

export type UnitDisplaySummary = {
  storage_convention: string;
  model_units: Record<string, string>;
  result_units: string[];
  quantity_display_policy: string;
  conversion_performed: boolean;
};

export function buildUnitDisplaySummary(model: PreviewModel, result: MechanicsResult): UnitDisplaySummary {
  return {
    storage_convention: "entered_units_preserved",
    model_units: Object.fromEntries(
      Object.entries(model.project.units).sort(([left], [right]) => left.localeCompare(right))
    ),
    result_units: Array.from(new Set(result.results.map((item) => item.unit).filter(Boolean))).sort(),
    quantity_display_policy: "display result-row values with their explicit units; no report-time conversion",
    conversion_performed: false
  };
}

function sessionProvenance(model: PreviewModel) {
  return {
    source_name: `OpenPipeStress desktop session (${model.project.name})`,
    source_location: "local desktop session",
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: "not_asserted",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function mapObjectRef(ref: { object_type: string; ref: string } | undefined, fallbackType: string): Ref {
  if (!ref) return { ref_type: fallbackType, ref_id: "TBD" };
  return { ref_type: ref.object_type, ref_id: ref.ref };
}

function diagnosticsForSections(result: MechanicsResult, provenance: ReturnType<typeof sessionProvenance>) {
  return result.diagnostics.map((diagnostic, index) => ({
    code: diagnostic.code,
    class:
      diagnostic.severity === "blocking" || diagnostic.severity === "error"
        ? "SOLVE_BLOCKING"
        : "ASSUMPTION_WARNING",
    severity:
      diagnostic.severity === "blocking" || diagnostic.severity === "error"
        ? "blocking"
        : diagnostic.severity,
    source: { ref_type: "diagnostic_source", ref_id: diagnostic.source ?? "core/product_physics" },
    affected_object: {
      ref_type: "affected_refs",
      ref_id: diagnostic.affected_refs?.join(", ") || diagnostic.id || `diagnostic-${index}`
    },
    message: diagnostic.message,
    remediation:
      "Human review required: evaluate this diagnostic against its referenced results before any reliance.",
    provenance
  }));
}

function componentSourceLocation(component: PreviewComponent): string {
  const refs: string[] = [];
  if (component.provenance?.trim()) {
    refs.push(`component.provenance=${component.provenance.trim()}`);
  }
  for (const [key, value] of Object.entries(component.geometry ?? {})) {
    if (typeof value === "string" && key.endsWith("_reference") && value.trim()) {
      refs.push(`geometry.${key}=${value.trim()}`);
    }
  }
  if (component.modifiers?.source_reference?.trim()) {
    refs.push(`modifiers.source_reference=${component.modifiers.source_reference.trim()}`);
  }
  return refs.join("; ") || "component provenance missing";
}

function componentProvenanceRecord(
  component: PreviewComponent,
  base: ReturnType<typeof sessionProvenance>
) {
  const label = component.label?.trim() || component.id;
  return {
    source_name: `${label} component provenance`,
    source_location: componentSourceLocation(component),
    source_license: "user_supplied_or_private",
    contributor: base.contributor,
    contributor_certification:
      "User-local component provenance metadata; redistribution remains private or pending until separately cleared.",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function componentHasProvenance(component: PreviewComponent): boolean {
  return Boolean(component.provenance?.trim());
}

function componentProvenanceValues(
  model: PreviewModel,
  provenance: ReturnType<typeof sessionProvenance>
) {
  return model.components.map((component) => {
    const record = componentProvenanceRecord(component, provenance);
    return {
      value_id: `component-provenance:${component.id}`,
      value_category: `component_provenance:${component.kind || "TBD"}`,
      source: { ref_type: "component", ref_id: component.id },
      quantity: null,
      provenance: record,
      privacy_classification: record.privacy_classification,
      required_for: ["reporting", "human_review"],
      review_status: record.review_status,
      missing_data_finding: !componentHasProvenance(component)
    };
  });
}

function componentProvenanceDiagnostics(
  model: PreviewModel,
  provenance: ReturnType<typeof sessionProvenance>
) {
  return model.components
    .filter((component) => !componentHasProvenance(component))
    .map((component) => ({
      code: "COMPONENT_PROVENANCE_MISSING",
      class: "PROVENANCE_WARNING",
      severity: "warning",
      source: { ref_type: "component", ref_id: component.id },
      affected_object: { ref_type: "component", ref_id: component.id },
      message: `Component ${component.label || component.id} has no component.provenance field; report provenance is incomplete for this component.`,
      remediation: "Enter component source/provenance before relying on this report.",
      provenance
    }));
}

function springHangerSupports(model: PreviewModel) {
  return model.supports.filter(
    (support) =>
      support.family === "variable_spring_hanger" ||
      support.family === "constant_effort_support" ||
      support.hanger?.hanger_type === "variable_spring_hanger" ||
      support.hanger?.hanger_type === "constant_effort_support"
  );
}

function springHangerSourceLocation(support: PreviewModel["supports"][number]): string {
  const refs: string[] = [];
  if (support.provenance?.trim()) refs.push(`support.provenance=${support.provenance.trim()}`);
  if (support.family?.trim()) refs.push(`support.family=${support.family.trim()}`);
  for (const [key, value] of Object.entries(support.hanger ?? {})) {
    if (typeof value === "string" && value.trim()) refs.push(`hanger.${key}=${value.trim()}`);
  }
  return refs.join("; ") || "spring hanger provenance missing";
}

function springHangerProvenanceRecord(
  support: PreviewModel["supports"][number],
  base: ReturnType<typeof sessionProvenance>
) {
  return {
    source_name: `${support.label || support.id} spring-hanger provenance`,
    source_location: springHangerSourceLocation(support),
    source_license: "user_supplied_or_private",
    contributor: base.contributor,
    contributor_certification:
      "User-local spring-hanger provenance metadata; redistribution remains private or pending until separately cleared.",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function springHangerReportQuantity(support: PreviewModel["supports"][number]) {
  const force =
    support.hanger?.constant_load ??
    support.hanger?.hot_load ??
    support.hanger?.cold_load ??
    support.hanger?.installed_load ??
    null;
  if (!force) return null;
  return {
    magnitude: force.value,
    unit: force.unit,
    dimension: "force" as const
  };
}

function springHangerValues(model: PreviewModel, provenance: ReturnType<typeof sessionProvenance>) {
  return springHangerSupports(model).map((support) => {
    const record = springHangerProvenanceRecord(support, provenance);
    const quantity = springHangerReportQuantity(support);
    return {
      value_id: `spring-hanger:${support.id}`,
      value_category: `spring_hanger:${support.hanger?.hanger_type || support.family || "TBD"}`,
      source: { ref_type: "support", ref_id: support.id },
      quantity,
      provenance: record,
      privacy_classification: record.privacy_classification,
      required_for: ["reporting", "human_review"],
      review_status: record.review_status,
      missing_data_finding:
        !support.provenance?.trim() || !support.hanger?.source_reference?.trim() || quantity === null
    };
  });
}

function springHangerDiagnostics(model: PreviewModel, provenance: ReturnType<typeof sessionProvenance>) {
  return springHangerSupports(model)
    .filter((support) => !support.provenance?.trim() || !support.hanger?.source_reference?.trim())
    .map((support) => ({
      code: "SPRING_HANGER_PROVENANCE_MISSING",
      class: "PROVENANCE_WARNING",
      severity: "warning",
      source: { ref_type: "support", ref_id: support.id },
      affected_object: { ref_type: "support", ref_id: support.id },
      message: `Spring hanger ${support.label || support.id} is missing support provenance or hanger.source_reference.`,
      remediation: "Enter spring-hanger source/provenance before relying on this report.",
      provenance
    }));
}

function record(value: unknown): Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function text(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value.trim() : null;
}

/** The input manifest is used only when it is the one the analysis run binds
 * (ref and SHA-256) and its canonical bytes verify; otherwise its settings are
 * reported as not recorded. */
async function boundInputManifest(
  analysisRun: AnalysisRunEnvelope,
  inputManifest: CurrentSessionInputManifestEvidence | null | undefined
): Promise<CurrentSessionInputManifestEvidence | null> {
  if (!inputManifest) return null;
  const reproducibility = analysisRun.analysis_run.reproducibility;
  const refs = reproducibility.input_manifest_refs ?? [];
  const hashes = reproducibility.input_manifest_hashes ?? [];
  if (
    refs.length !== 1 ||
    hashes.length !== 1 ||
    refs[0].ref !== inputManifest.manifest_ref.ref ||
    hashes[0].value !== inputManifest.manifest_sha256
  ) {
    return null;
  }
  try {
    await verifyCurrentSessionInputManifest(inputManifest);
  } catch {
    return null;
  }
  return inputManifest;
}

async function buildReportContext(
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope,
  renderContext: ReportRenderContext,
  inputManifest: CurrentSessionInputManifestEvidence | null | undefined
) {
  const run = analysisRun.analysis_run;
  const manifest = await boundInputManifest(analysisRun, inputManifest);
  const solverBasis = manifest?.manifest.solver_basis ?? null;
  const settings = solverBasis
    ? await Promise.all(
        Object.keys(solverBasis.settings)
          .sort()
          .map(async (key) => ({ key, value: await canonicalJsonString(solverBasis.settings[key]) }))
      )
    : [];
  return {
    export_context: renderContext.exportContext,
    generated_at: renderContext.generatedAt,
    software_name: text(desktopAppConfig.productName) ?? "SWBPIPE desktop",
    software_version: text(desktopAppConfig.version),
    // The desktop build records no build/release identifier.
    build_release_identifier: NOT_RECORDED,
    solver_name: text(run.solver_version?.solver_name),
    solver_version: text(run.solver_version?.solver_version),
    solver_build_ref: text(run.solver_version?.build_ref?.ref),
    analysis_run_name: text(run.run_name),
    analysis_run_recorded_at: text(run.created_at),
    solver_mode: text(solverBasis?.solver_mode),
    solver_settings_ref: text(run.settings_ref?.ref),
    solver_settings: settings,
    // The preview model records no model-level coordinate system.
    model_coordinate_system: NOT_RECORDED,
    result_coordinate_systems: Array.from(
      new Set(
        result.results
          .map((item) => item.metadata?.coordinate_system)
          .filter((value): value is string => typeof value === "string" && value.trim() !== "" && value !== "TBD")
      )
    ).sort()
  };
}

async function rulePackReference(run: SessionRuleCheckRun) {
  const document = record(run.rulePackDocument);
  const metadata = record(document.metadata);
  const classification = record(document.classification);
  const declared = record(record(document.checksums).rule_pack_checksum);
  const declaredValue = text(declared.value);
  const ruleId = run.result.rule_pack_id;
  if (!text(ruleId) || text(metadata.rule_pack_id) !== ruleId) {
    throw new Error("REPORT-RULE-CHECK-PACK-MISMATCH: the rule-check run and the loaded rule pack differ.");
  }
  const redistribution = text(classification.redistribution_status);
  const name = text(metadata.name);
  return {
    rule_pack_id: ruleId,
    // Safe display name (PRD §19.2); omitted when the pack records none, which
    // the renderer discloses as "not recorded".
    ...(name ? { name } : {}),
    version: text(metadata.rule_pack_version) ?? "not recorded",
    checksum: declaredValue
      ? {
          algorithm: text(declared.algorithm) ?? "sha256",
          canonicalization: text(declared.canonicalization) ?? "not recorded",
          payload_ref: { ref_type: "rule_pack", ref_id: ruleId },
          value: declaredValue
        }
      : {
          // No declared checksum: the canonical SHA-256 of the loaded document.
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: { ref_type: "rule_pack_document", ref_id: ruleId },
          value: await canonicalSha256Hex(run.rulePackDocument)
        },
    source_notice: text(metadata.source_notice) ?? "not recorded",
    redistribution_status:
      redistribution && REPORT_REDISTRIBUTION_STATUSES.has(redistribution) ? redistribution : "unknown",
    // Required-input status: incomplete when any check lacks inputs. The
    // aggregate ranks a failed check above an incomplete one, so it cannot be
    // the basis here.
    completeness_status:
      run.result.aggregate_status === "RULE_INPUTS_INCOMPLETE" ||
      run.result.checks.some((check) => check.status === "RULE_INPUTS_INCOMPLETE")
        ? "incomplete"
        : "complete",
    private_payload_redacted: true
  };
}

function userRuleChecks(run: SessionRuleCheckRun) {
  const status = (value: RuleCheckStatus) => {
    if (!RULE_CHECK_STATUSES.has(value)) throw new Error(`REPORT-RULE-CHECK-STATUS-UNKNOWN: ${String(value)}`);
    return value;
  };
  return {
    rule_pack_id: run.result.rule_pack_id,
    grammar_version: run.result.grammar_version,
    aggregate_status: status(run.result.aggregate_status),
    // One outcome per check and load case or combination (rule-pack format
    // 0.5.0; DEL-06-01 Design `rule-pack-0.5.0.md` §2.7, §6): each carries
    // the basis it was evaluated on. An outcome without one (a check the pack
    // gives no basis) omits the member, and the report shows "load case not
    // recorded".
    checks: run.result.checks.map((check) => ({
      check_id: check.check_id,
      ...(check.basis_ref ? { basis_ref: { ref_type: check.basis_ref.ref_type, ref_id: check.basis_ref.ref_id } } : {}),
      status: status(check.status),
      missing_inputs: Array.from(
        new Set(check.bound_inputs.filter((input) => !input.supplied).map((input) => input.input_id))
      ).sort()
    }))
  };
}

export async function buildRenderableReportInput({
  model,
  result,
  analysisRun,
  projectSummary,
  renderContext,
  inputManifest = null,
  ruleCheckRun = null,
  numericalHealth = null,
  exactAdmission = null
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  projectSummary: LocalProjectSummary | null;
  renderContext: ReportRenderContext;
  inputManifest?: CurrentSessionInputManifestEvidence | null;
  ruleCheckRun?: SessionRuleCheckRun | null;
  /** DEL-07-05 numerical health §5: the rendered report's one line per load case,
   * placed after the result rows and so before the diagnostics section, on the generic
   * route only. Absent, the input is unchanged (the report package passes none). No
   * figure is exported. */
  numericalHealth?: NumericalHealthView | null;
  /** W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.1): the recorded Export
   * Result JSON admission of an exact-route source (`admitExactReportSource`), required
   * for an exact source and refused for any other. A pressure-free source takes none, so
   * its input is unchanged. */
  exactAdmission?: ExactReportAdmission | null;
}) {
  const exact = isExactReportSource(result);
  if (!exact && exactAdmission !== null) {
    throw new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`);
  }
  const run = analysisRun.analysis_run;
  // The exact path's admission is checked at the start and again after the last await
  // (`checkExactReportAdmission`): retirement, an edit or a substitute refuses.
  const exactInputs = exact ? { model, result, analysisRun, inputManifest: inputManifest as CurrentSessionInputManifestEvidence } : null;
  if (exactInputs) checkExactReportAdmission(exactAdmission, exactInputs);
  const provenance = sessionProvenance(model);
  const modelHashValue = await canonicalSha256Hex(model);
  const runRecordHash = run.hashes.find((item) => item.payload_scope === "analysis_run_record");
  const receivedResultScope = analysisResultHashScope(analysisRun.schema_version);
  if (receivedResultScope === null) throw new Error("REPORT-ANALYSIS-VERSION-UNSUPPORTED");
  const resultEnvelopeHash = run.hashes.find((item) => item.payload_scope === receivedResultScope);

  const analysisStatus = Array.from(
    new Set(
      [...run.analysis_status, "HUMAN_REVIEW_REQUIRED"].filter((status) =>
        KNOWN_ANALYSIS_STATUSES.has(status)
      )
    )
  ).sort();

  const reportSections = {
    report_section_id: `sections:${result.run_id}`,
    model_ref: { ref_type: "model", ref_id: result.model_ref },
    run_ref: { ref_type: "analysis_run", ref_id: result.run_id },
	    diagnostics: [
	      ...diagnosticsForSections(result, provenance),
	      ...componentProvenanceDiagnostics(model, provenance),
	      ...springHangerDiagnostics(model, provenance)
	    ],
    analysis_status_disclosures: [
      ...analysisStatus
        .filter((status) => status !== "HUMAN_REVIEW_REQUIRED")
        .map((status) => ({
          status,
          source: { ref_type: "analysis_run", ref_id: result.run_id },
          affected_object: { ref_type: "model", ref_id: result.model_ref },
          explanation: `Automatic analysis status reported by the run record: ${status}.`,
          human_review_required: true,
          human_acceptance_ref: null
        })),
      {
        status: "HUMAN_REVIEW_REQUIRED",
        source: { ref_type: "report_renderer", ref_id: `report:${result.run_id}` },
        affected_object: { ref_type: "report", ref_id: `report:${result.run_id}` },
        explanation: "Human professional review is required before any reliance on this report.",
        human_review_required: true,
        human_acceptance_ref: null
      }
    ],
	    provenance_notes: [
	      provenance,
	      ...model.components.map((component) => componentProvenanceRecord(component, provenance)),
	      ...springHangerSupports(model).map((support) => springHangerProvenanceRecord(support, provenance))
	    ],
	    user_supplied_values: [
	      ...componentProvenanceValues(model, provenance),
	      ...springHangerValues(model, provenance)
	    ],
    assumptions: [],
    limitations: [
      {
        limitation_id: `limitation:${result.run_id}:technical-preview`,
        source: { ref_type: "report_renderer", ref_id: `report:${result.run_id}` },
        affected_scope: { ref_type: "model", ref_id: result.model_ref },
        statement:
          "Output over invented or user-local data; not validated engineering output.",
        effect: {
          mechanics_solve_qualified: true,
          user_rule_check_qualified: false,
          report_completeness: "qualified",
          human_review_required: true
        },
        provenance
      }
    ],
    unresolved_tbds: run.reproducibility.unresolved_tbd.map((description, index) => ({
      tbd_id: `tbd:${result.run_id}:${index + 1}`,
      affected_scope: { ref_type: "analysis_run", ref_id: result.run_id },
      description,
      review_needed: true
    })),
    professional_boundary: PROFESSIONAL_BOUNDARY
  };

  // W-11 (§2.4): an exact source's limitation entries, one per published producer
  // string with the closed table's statement, then the profile reference, after the
  // technical-preview entry (which stays first), sharing its effect and provenance.
  if (exactInputs) {
    const semanticContract = run.reproducibility.semantic_contract;
    if (!semanticContract) throw new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`);
    const [preview] = reportSections.limitations;
    const entries = await exactLimitationEntries(result, result.run_id, { id: semanticContract.id, sha256: semanticContract.sha256 });
    reportSections.limitations.push(...entries.map((entry) => ({ ...entry, effect: preview.effect, provenance: preview.provenance })) as never[]);
  }

  const sectionsChecksumValue = await canonicalSha256Hex(reportSections);

  const checksum = (payloadRef: Ref, value: string | undefined) => ({
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
    payload_ref: payloadRef,
    value: value || "TBD"
  });

  const envelope = (ref: Ref, schemaId: string, checksumValue: string | undefined) => ({
    ref,
    schema_ref: { ref_type: "schema", ref_id: schemaId },
    checksum: checksum(ref, checksumValue),
    privacy_classification: "private_project_data",
    provenance
  });

  const calculationReport = {
    report_id: `report:${result.run_id}`,
    model_input_summary: {
      project_ref: { ref_type: "project", ref_id: model.project.id },
      model_ref: { ref_type: "model", ref_id: result.model_ref },
      persistence_ref: {
        ref_type: "project_persistence",
        ref_id: projectSummary ? `local_sqlite:${projectSummary.project_id}` : "TBD"
      },
      unit_system_ref: DEC018_UNIT_SYSTEM_REF,
      unit_display_summary: buildUnitDisplaySummary(model, result),
      model_hash: checksum({ ref_type: "model", ref_id: model.project.id }, modelHashValue),
      input_manifest_ref: mapObjectRef(run.reproducibility.input_manifest_refs[0], "audit_manifest"),
      provenance
    },
    load_case_summary: model.load_cases.map((loadCase) => ({
      load_ref: { ref_type: "load_case", ref_id: loadCase.id },
      label: loadCase.label,
      basis: loadCase.kind,
      source: { ref_type: "model", ref_id: model.project.id },
      provenance
    })),
    result_export_refs: run.result_refs.map((item) =>
      envelope(
        { ref_type: item.result_ref.object_type, ref_id: item.result_ref.ref },
        "schemas/results.schema.yaml",
        item.hash_refs[0]?.value
      )
    ),
    audit_manifest_refs: [
      envelope(
        { ref_type: "analysis_run", ref_id: result.run_id },
        "schemas/analysis_run.schema.yaml",
        runRecordHash?.value
      ),
      envelope(
        { ref_type: "result_envelope", ref_id: `result-envelope:${result.run_id}` },
        "schemas/results.schema.yaml",
        resultEnvelopeHash?.value
      )
    ],
    report_section_refs: [
      envelope(
        { ref_type: "report_sections", ref_id: reportSections.report_section_id },
        "schemas/report_sections.schema.yaml",
        sectionsChecksumValue
      )
    ],
    rule_pack_refs: ruleCheckRun ? [await rulePackReference(ruleCheckRun)] : [],
    diagnostics: [],
    template_slots: SECTION_KINDS.map((kind, index) => ({
      slot_id: `slot-${kind}`,
      required: true,
      section_kind: kind,
      source_contract: { ref_type: "contract", ref_id: "DEL-08-01" },
      ordering_index: index + 1
    })),
    rendered_sections: SECTION_KINDS.map((kind) => ({
      section_id: `section-${kind}`,
      slot_id: `slot-${kind}`,
      section_kind: kind,
      title: kind.replaceAll("_", " "),
      source_refs: [{ ref_type: "model", ref_id: model.project.id }],
      content_status: "rendered"
    })),
    analysis_status: analysisStatus,
    professional_boundary: PROFESSIONAL_BOUNDARY,
    provenance,
    privacy_classification: "private_project_data",
    unresolved_runtime_tbds: run.reproducibility.unresolved_tbd.map((description, index) => ({
      tbd_id: `tbd:${result.run_id}:${index + 1}`,
      topic: "analysis run reproducibility",
      affected_scope: { ref_type: "analysis_run", ref_id: result.run_id },
      description,
      review_needed: true
    }))
  };

  const reportContext = await buildReportContext(result, analysisRun, renderContext, inputManifest);
  // W-11 (§3.2-§3.3): the exact-route result block: the identity, the admission binding
  // (the renderer composes the standing line from it), the load/reference line and the
  // headlines, nominal then reduced.
  const exactRoute = exactInputs
    ? await (async () => {
        const core = await exactRouteCore(result);
        return {
          semantic_contract_id: core.semantic_contract_id,
          admission: { ...exactAdmission!.binding },
          load_reference_states: core.load_reference_states,
          headlines: core.headlines
        };
      })()
    : null;
  // After the last await: the admission still holds for these live objects.
  if (exactInputs) checkExactReportAdmission(exactAdmission, exactInputs);

  return {
    report_title: `${model.project.name} — Calculation Report`,
    calculation_report: calculationReport,
    report_sections: reportSections,
    // W-11 (DEL-08-01 Design `exact-route-report-sections.md` §6.1): an exact-route
    // report carries no numerical-health line until the native render can re-derive it
    // from the delivered result; its rows are exactly the source rows' projection.
    result_rows: reportResultRows(result).concat((numericalHealth?.route === "generic" && !exactInputs ? numericalHealth.cases : []).map((item) => ({
      row_id: `numerical-health:${item.caseId}`,
      label: reportLineText(item.state),
      case_ref: `load_case:${item.caseId}`,
      quantity_display: "",
      source_ref: item.integrityDiagnosticId ?? "numerical_quality"
    }))),
    report_context: reportContext,
    user_rule_checks: ruleCheckRun ? userRuleChecks(ruleCheckRun) : null,
    ...(exactRoute ? { exact_route: exactRoute } : {})
  };
}
