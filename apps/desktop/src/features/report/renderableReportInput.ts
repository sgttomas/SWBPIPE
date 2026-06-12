import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewModel
} from "../../types";
import { canonicalJson } from "../../services/hashService";

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

type Ref = { ref_type: string; ref_id: string };

function sessionProvenance(model: PreviewModel) {
  return {
    source_name: `OpenPipeStress desktop session (${model.project.name})`,
    source_location: "local desktop session (invented preview / user-local data)",
    source_license: "project_fixture",
    contributor: "OpenPipeStress",
    contributor_certification:
      "Invented preview or user-local non-engineering data only; no protected standards content.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "accepted",
    privacy_classification: "invented_public_example"
  };
}

async function sha256Hex(payload: string): Promise<string> {
  const subtle = globalThis.crypto?.subtle;
  if (!subtle) return "TBD";
  const digest = await subtle.digest("SHA-256", new TextEncoder().encode(payload));
  return Array.from(new Uint8Array(digest))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
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

export async function buildRenderableReportInput({
  model,
  result,
  analysisRun,
  projectSummary
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  projectSummary: LocalProjectSummary | null;
}) {
  const run = analysisRun.analysis_run;
  const provenance = sessionProvenance(model);
  const modelHashValue = await sha256Hex(canonicalJson(model));
  const runRecordHash = run.hashes.find((item) => item.payload_scope === "analysis_run_record");
  const resultEnvelopeHash = run.hashes.find((item) => item.payload_scope === "result_envelope");

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
    diagnostics: diagnosticsForSections(result, provenance),
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
    provenance_notes: [provenance],
    user_supplied_values: [],
    assumptions: [],
    limitations: [
      {
        limitation_id: `limitation:${result.run_id}:technical-preview`,
        source: { ref_type: "report_renderer", ref_id: `report:${result.run_id}` },
        affected_scope: { ref_type: "model", ref_id: result.model_ref },
        statement:
          "Technical-preview output over invented or user-local data; not validated engineering output.",
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

  const sectionsChecksumValue = await sha256Hex(canonicalJson(reportSections));

  const checksum = (payloadRef: Ref, value: string | undefined) => ({
    algorithm: "sha256",
    canonicalization: "JCS",
    payload_ref: payloadRef,
    value: value || "TBD"
  });

  const envelope = (ref: Ref, schemaId: string, checksumValue: string | undefined) => ({
    ref,
    schema_ref: { ref_type: "schema", ref_id: schemaId },
    checksum: checksum(ref, checksumValue),
    privacy_classification: "invented_public_example",
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
      unit_system_ref: { ref_type: "unit_system", ref_id: "preview-display-label-set" },
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
    rule_pack_refs: [],
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
    privacy_classification: "invented_public_example",
    unresolved_runtime_tbds: run.reproducibility.unresolved_tbd.map((description, index) => ({
      tbd_id: `tbd:${result.run_id}:${index + 1}`,
      topic: "analysis run reproducibility",
      affected_scope: { ref_type: "analysis_run", ref_id: result.run_id },
      description,
      review_needed: true
    }))
  };

  return {
    report_title: `${model.project.name} — Calculation Report (Technical Preview)`,
    calculation_report: calculationReport,
    report_sections: reportSections,
    result_rows: result.results.map((item) => ({
      row_id: `row:${item.id}`,
      label: item.metadata ? `${item.kind} (${item.metadata.component})` : item.kind,
      case_ref: item.basis_ref ? `${item.basis_ref.ref_type}:${item.basis_ref.ref_id}` : "TBD",
      quantity_display: `${item.value} ${item.unit}`,
      source_ref: `result:${item.id}`
    }))
  };
}
