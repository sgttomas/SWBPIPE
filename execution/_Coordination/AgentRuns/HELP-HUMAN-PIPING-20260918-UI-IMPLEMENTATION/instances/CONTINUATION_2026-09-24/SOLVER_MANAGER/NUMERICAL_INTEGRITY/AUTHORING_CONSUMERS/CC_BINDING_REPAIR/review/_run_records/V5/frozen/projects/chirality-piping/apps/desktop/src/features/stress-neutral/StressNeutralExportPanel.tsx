import { sourceContract, numericalResultStanding, PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256 } from "../results/numericalResultQuality";
import { verifyAnalysisRunRecord, validateAnalysisRunV03, analysisRowSemantics, sourceBasisReference, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { semanticFamily, semanticDimension, semanticCategory, resultSemantics } from "../results/resultSemantics";
import { Download, FileJson } from "lucide-react";
import { useEffect, useState } from "react";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, ResultBasisRef, ObjectRef } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

const STRESS_NEUTRAL_EXPORT_VERSION = "0.2.0";
const STRESS_NEUTRAL_EXPORT_PROFILE = "ops.stress_neutral.v2";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";
const CSV_COLUMNS = [
  "result_id",
  "canonical_ref",
  "row_kind",
  "result_family",
  "load_case_ref",
  "station_ref",
  "component_ref",
  "value",
  "unit",
  "dimension",
  "correlation_status"
] as const;

type CsvColumn = (typeof CSV_COLUMNS)[number];

type StressNeutralRef = {
  object_type: string;
  ref: string;
};

type StressNeutralRow = {
  result_id: string;
  canonical_ref: StressNeutralRef;
  row_kind: "result_value";
  result_family: string;
  load_case_ref: StressNeutralRef;
  station_ref: StressNeutralRef;
  component_ref: StressNeutralRef;
  value: number;
  unit: string;
  dimension: string;
  correlation_status: "canonical_id_map" | "unit_or_dimension_blocking_review_required";
  source_result_ref: StressNeutralRef;
  provenance: ReturnType<typeof previewProvenance>;
};

type StressNeutralUnitPreservationWitness = {
  witness_id: string;
  source_result_ref: StressNeutralRef;
  source_field_path: string;
  source_quantity: {
    value: number;
    unit: string;
    dimension: string;
  };
  target_row_ref: StressNeutralRef;
  target_field_path: string;
  target_quantity: {
    value: number;
    unit: string;
    dimension: string;
  };
  export_unit_policy: "preserve_source_result_unit_and_dimension";
  conversion_performed: false;
  decision_basis_refs: StressNeutralRef[];
  provenance: ReturnType<typeof previewProvenance>;
};

export function StressNeutralExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const [packet, setPacket] = useState<Awaited<ReturnType<typeof buildStressNeutralExportPacket>> | null>(null);
  const [binding, setBinding] = useState<[PreviewModel, MechanicsResult, AnalysisRunEnvelope] | null>(null);
  const currentPacket = binding?.[0] === model && binding?.[1] === result && binding?.[2] === analysisRun
    ? packet
    : null;
  useEffect(() => {
    let current = true;
    setPacket(null);
    setBinding(null);
    if (result && analysisRun) buildStressNeutralExportPacket({ model, result, analysisRun }).then((built) => {
      if (current) {
        setPacket(built);
        setBinding([model, result, analysisRun]);
      }
    }).catch(() => { if (current) { setPacket(null); setBinding(null); } });
    return () => { current = false; };
  }, [model, result, analysisRun]);
  return (
    <section
      className="panel stress-neutral-export-panel"
      aria-label="Stress-neutral CSV JSON export"
      data-testid="stress-neutral-panel"
    >
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Stress-neutral CSV/JSON
      </div>
      {currentPacket ? (
        <>
          <div className="report-actions">
        <ControlledExportLink
              className="report-export-link"
              data-testid="stress-neutral-export-link"
              nativeCurrentBinding={binding}
              download={`openpipestress-preview-stress-neutral-${safeFileToken(currentPacket.source_result_ref.ref)}.json`}
              href={jsonDataHref(currentPacket)}
              validateDecodedPayload={(payload) => validateStressNeutralExportPacket(payload, currentPacket.schema_version === "0.3.0" ? binding?.[1] : undefined, currentPacket.schema_version === "0.3.0" ? binding?.[2] : undefined, binding && currentPacket.schema_version === "0.3.0" ? modelLoadBasisRefs(binding[0]) : undefined)}
            >
              <Download size={14} aria-hidden="true" />
              Package JSON
        </ControlledExportLink>
        <ControlledExportLink
              className="report-export-link"
              data-testid="stress-neutral-csv-link"
              download={`openpipestress-preview-stress-neutral-${safeFileToken(currentPacket.source_result_ref.ref)}.csv`}
              href={csvDataHref(currentPacket.csv_text)}
            >
              <Download size={14} aria-hidden="true" />
              CSV
        </ControlledExportLink>
            <span data-testid="stress-neutral-summary">
              available; rows={currentPacket.result_rows.length}; csv_columns={currentPacket.export_profile.csv_columns.length};
              diagnostics={currentPacket.diagnostics.length}
            </span>
          </div>
          <div className="report-list" data-testid="stress-neutral-body">
            <StressNeutralLine
              label="Format"
              value={`${currentPacket.export_profile.target_family}; profile=${currentPacket.export_profile.profile_id}; schema=${currentPacket.schema_version}`}
              testId="stress-neutral-format"
            />
            <StressNeutralLine
              label="State binding"
              value={`${currentPacket.source_model_ref.ref}; ${currentPacket.source_run_ref.ref}; ${currentPacket.source_result_ref.ref}`}
              testId="stress-neutral-state-binding"
            />
            <StressNeutralLine
              label="Units"
              value={`${unitCount(currentPacket)} explicit units; dimensions=${dimensionSummary(
                currentPacket
              )}; ${unitDisclosureSummary(currentPacket.unit_system_disclosure)}`}
              testId="stress-neutral-units"
            />
            <StressNeutralLine
              label="Unit witnesses"
              value={`count=${currentPacket.unit_preservation_witnesses.length}; policy=preserve_source_result_units; conversion=false`}
              testId="stress-neutral-unit-witnesses"
            />
            <StressNeutralLine
              label="Package"
              value={`members=${currentPacket.manifest.package_members.length}; stable_ids=${currentPacket.stable_id_map.length}; loss_entries=${currentPacket.loss_report.length}; validation=${currentPacket.validation_report.validation_status}; package_hash=checked`}
              testId="stress-neutral-package"
            />
            <StressNeutralLine
              label="Boundary"
              value={`vendor_format=${String(packet.professional_boundary.software_makes_external_compatibility_claim)}; solver_validation=${String(
                packet.professional_boundary.software_makes_solver_validation_claim
              )}; code_compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; professional_reliance=${String(
                packet.professional_boundary.software_creates_professional_reliance_record
              )}`}
              testId="stress-neutral-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="stress-neutral-empty">
          Run mechanics preview to assemble a stress-neutral CSV/JSON package for local review and downstream adapter
          development.
        </p>
      )}
      <small className="report-note">
        Stress-neutral export is a local review package only; it is not a vendor format or solver input deck. Validation
        occurs in the user's accepted professional tools; this package is screening and handoff evidence.
      </small>
    </section>
  );
}

function StressNeutralLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildStressNeutralExportPacketV01({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
}) {
  const run = analysisRun.analysis_run;
  const resultRows = result.results
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((item) => sourceContract(result) === "precision" ? precisionStressRow(item, result) : stressNeutralRow(item, run.run_id, result));
  const csvText = renderCsv(resultRows);
  const unitPreservationWitnesses = stressNeutralUnitPreservationWitnesses(resultRows, result.results);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: {},
    conversionPolicy: "result_row_units_preserved_no_export_time_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx"
  });
  const stableIdMap = resultRows.map((row, index) => ({
    map_id: `stress-neutral-map:${sourceContract(result) === "precision" ? index : safeFileToken(row.result_id)}`,
    canonical_ref: row.canonical_ref,
    export_ref: reference("StressNeutralResultRow", `stress-neutral-row:${sourceContract(result) === "precision" ? index : safeFileToken(row.result_id)}`),
    mapping_status: "mapped",
    loss_category: "exported",
    row_index: index,
    provenance: previewProvenance()
  }));
  const diagnostics: Array<ReturnType<typeof stressNeutralDiagnostics>[number] | {code:string;class:string;severity:string;source:StressNeutralRef;affected_object:StressNeutralRef;message:string;remediation:string;provenance:ReturnType<typeof previewProvenance>}> = stressNeutralDiagnostics(resultRows);
  if(unitPreservationWitnesses.length!==resultRows.length)diagnostics.push({class:"export_blocking",source:reference("ExportConsumer","DEL-17-06"),affected_object:reference("StressNeutralResultRows","stress-neutral:result-rows"),remediation:"Review received declarations and canonical0.2 source/target semantics; unavailable preservation witnesses are withheld.",code:"SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE",severity:"blocking",message:`${resultRows.length-unitPreservationWitnesses.length} received carrier dimension declarations are unavailable or differ from independently interpreted semantics; numerical rows and units retained, preservation witnesses withheld.`,provenance:previewProvenance()});
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const validationStatus = blockingCount === 0 ? "passed" : "blocked";

  return {
    schema_version: STRESS_NEUTRAL_EXPORT_VERSION,
    document_kind: "openpipestress.technical_preview.stress_neutral_csv_json_package",
    deliverable_id: "DEL-17-06",
    package_id: "PKG-17",
    scope_items: ["SOW-046", "SOW-074"],
    objectives: ["OBJ-007", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    export_id: `stress-neutral:${safeFileToken(result.run_id)}`,
    package_status: "stress_neutral_export_package",
    source_model_ref: reference("Model", model.project.id),
    source_run_ref: reference("AnalysisRun", run.run_id),
    source_result_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
    source_basis_refs: [
      { deliverable_id: "DEL-08-04", status: "source_result_envelope" },
      { deliverable_id: "DEL-14-02", status: "source_analysis_run" },
      { deliverable_id: "DEL-14-05", status: "source_hash_boundary" },
      { deliverable_id: "DEL-17-02", status: "source_native_json_boundary" }
    ],
    source_hashes: run.hashes.map((item) => ({
      ...item,
      hash_role: "source_run_evidence_not_package_member_checksum"
    })),
    export_profile: {
      profile_id: "ops.stress_neutral.v1",
      target_family: "stress_neutral_csv_json",
      csv_columns: CSV_COLUMNS,
      identity_policy: "canonical_ref_per_row_plus_stable_id_map",
      unit_policy: "unit_and_dimension_required_per_row",
      loss_report_policy: "mandatory_for_every_package",
      comparison_semantics: "diagnostic_export_only_no_pass_fail"
    },
    unit_system_disclosure: unitSystemDisclosure,
    unit_preservation_witnesses: unitPreservationWitnesses,
    result_rows: resultRows,
    csv_text: csvText,
    stable_id_map: stableIdMap,
    loss_report: {
      report_id: `loss-report:${safeFileToken(result.run_id)}:stress-neutral`,
      entries: [
        {
          loss_id: "loss:desktop-preview:stress-neutral-exported",
          category: "exported",
          severity: "info",
          affected_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
          reason: "Received numerical rows/units/source refs are retained; dimensions are independently interpreted. Missing/incompatible declaration witnesses are withheld and validation blocked.",
          downstream_implication: "Local review evidence only; validation/loss findings require review before downstream reliance."
        },
        {
          loss_id: "loss:desktop-preview:comparison-semantics-tbd",
          category: "tbd",
          severity: "warning",
          affected_ref: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
          reason: "Pass/fail, tolerances, and external target interpretation are outside this preview package.",
          downstream_implication: "Consumers must not infer acceptance, equivalence, or compliance from this export."
        },
        {
          loss_id: "loss:desktop-preview:private-rule-payload-redacted",
          category: "omitted",
          severity: "warning",
          affected_ref: reference("RulePack", "rule-pack:user-supplied:not-loaded"),
          reason: "Private rule criteria and compliance inputs are not bundled in the public build.",
          downstream_implication: "Completeness and code-rule interpretation require separate governed review."
        }
      ],
      provenance: previewProvenance()
    },
    manifest: {
      manifest_id: `manifest:${safeFileToken(result.run_id)}:stress-neutral`,
      package_members: [
        member("manifest", "manifest.json", "json", 1),
        member("csv_text", "stress_neutral_results.csv", "csv", resultRows.length),
        member("result_rows", "result_rows.json", "json", resultRows.length),
        member("unit_system_disclosure", "unit_system_disclosure.json", "json", 1),
        member("unit_preservation_witnesses", "unit_preservation_witnesses.json", "json", unitPreservationWitnesses.length),
        member("stable_id_map", "stable_id_map.json", "json", stableIdMap.length),
        member("loss_report", "loss_report.json", "json", 3),
        member("validation_report", "validation_report.json", "json", 1),
        member("diagnostics", "diagnostics.json", "json", diagnostics.length)
      ],
      schema_ref: "schemas/stress_neutral_export.schema.json",
      provenance: previewProvenance()
    },
    validation_report: {
      validation_id: `validation:${safeFileToken(result.run_id)}:stress-neutral`,
      validation_status: validationStatus,
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      checks: [
        check("csv_json_row_sync", resultRows.length === csvText.trimEnd().split("\n").length - 1),
        check("canonical_ref_per_row", resultRows.every((row) => Boolean(row.canonical_ref.ref))),
        check("unit_and_dimension_per_row", resultRows.every((row) => Boolean(row.unit && row.dimension && row.dimension !== "TBD"))),
        check("unit_preservation_witness_per_row", unitPreservationWitnesses.length === resultRows.length),
        check(
          "unit_preservation_witnesses_match_rows",
          unitPreservationWitnesses.every((witness) => witnessMatchesRow(witness, resultRows.find(row=>row.result_id===witness.source_result_ref.ref)))
        ),
        check("stable_id_map_per_row", stableIdMap.length === resultRows.length),
        check("loss_report_present", true)
      ],
      blocking_diagnostic_count: blockingCount,
      provenance: previewProvenance()
    },
    diagnostics,
    boundary_notes: [
      "Local browser preview does not emit canonical package member hashes.",
      "The package is not a vendor format, solver input deck, or external target compatibility claim.",
      "Comparison pass/fail and tolerances remain outside this export; validation occurs in the user's accepted professional tools, and this package is screening and handoff evidence."
    ],
    privacy: {
      privacy_classification: "public_metadata_and_invented_results",
      private_payload_embedded: false,
      protected_content_embedded: false,
      source_model_mutated: false
    },
    professional_boundary: professionalBoundary(),
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    vendor_format_claim: false,
    solver_input_deck_claim: false,
    target_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false
  };
}

const STRICT_MEMBER_NAMES = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"] as const;

async function rawTextSha256(text: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function checksumRef(filename: string) {
  return { object_type: "StressNeutralMember", ref: filename };
}

async function checkedMemberChecksum(filename: string, value: unknown, payloadScope = "member_payload") {
  return { algorithm: "sha256" as const, canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: payloadScope, payload_ref: checksumRef(filename), value: await canonicalSha256HexCheckedV1(value) };
}

function strictManifestSeed(packet: any, checksums: any[]) {
  return {
    manifest_id: packet.manifest.manifest_id,
    source_result_ref: packet.source_result_ref,
    source_run_ref: packet.source_run_ref,
    source_model_ref: packet.source_model_ref,
    received_source_checksums: packet.received_source_checksums,
    unresolved_assumption_refs: packet.unresolved_assumption_refs,
    reproducibility_refs: packet.reproducibility_refs,
    export_profile_ref: packet.manifest.export_profile_ref,
    boundary_notes: packet.manifest.boundary_notes,
    member_checksums: checksums,
    diagnostics: packet.diagnostics,
  };
}

type StrictWitnessDisposition = "eligible" | "diagnostic_work" | "unknown_semantic" | "missing_semantic" | "contradiction";

const WITHHOLDING_CODES: Record<Exclude<StrictWitnessDisposition, "eligible">, string> = {
  diagnostic_work: "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK",
  unknown_semantic: "SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC",
  missing_semantic: "SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC",
  contradiction: "SN-UNIT-WITNESS-WITHHELD-CONTRADICTION"
};

function strictWitnessDisposition(source: MechanicsResult["results"][number], carrier: MechanicsResult): { disposition: StrictWitnessDisposition; dimension: string | null } {
  try {
    const semantic = sourceContract(carrier) === "precision" ? analysisRowSemantics(source, carrier).semantic : resultSemantics(source, carrier);
    if (!semantic) return { disposition: "unknown_semantic", dimension: null };
    if (semantic.category === "diagnostic_work") return { disposition: "diagnostic_work", dimension: semantic.derivative_target_dimension };
    if (semantic.component !== null && (!source.metadata || typeof source.metadata.component !== "string" || source.metadata.component.length === 0)) {
      return { disposition: "missing_semantic", dimension: semantic.derivative_target_dimension };
    }
    if (!semantic.derivative_target_dimension) return { disposition: "missing_semantic", dimension: null };
    return { disposition: "eligible", dimension: semantic.derivative_target_dimension };
  } catch {
    return { disposition: "contradiction", dimension: null };
  }
}

function strictWithholdingDiagnostic(disposition: Exclude<StrictWitnessDisposition, "eligible">, row: StressNeutralRow, rowIndex: number, packetProvenance = previewProvenance(), precision = false) {
  const diagnosticWork = disposition === "diagnostic_work";
  return {
    code: WITHHOLDING_CODES[disposition], class: "unit_preservation_witness", severity: diagnosticWork ? "info" : "blocking",
    source: reference("StressNeutralResultRow", row.result_id), affected_object: reference("StressNeutralUnitWitness", `unit-witness:${rowIndex}`),
    message: diagnosticWork
      ? "Diagnostic work evidence is retained as a row but has no physical unit-preservation witness."
      : `${disposition.replaceAll("_", " ")} prevents a unit-preservation witness; the received row remains retained without a physical interpretation claim.`,
    remediation: diagnosticWork
      ? "Review diagnostic work separately from physical quantity witnesses."
      : precision ? "Resolve the source family, unit, dimension and correlation evidence before downstream physical interpretation." : "Resolve the source kind, unit, component and semantic-contract evidence before downstream physical interpretation.",
    provenance: structuredClone(packetProvenance)
  };
}

export async function buildStressNeutralExportPacket(args: { model: PreviewModel; result: MechanicsResult; analysisRun: AnalysisRunEnvelope }) {
  const route = sourceContract(args.result);
  if (route === "unsupported") throw new Error("SN-SOURCE-CONTRACT-UNSUPPORTED");
  const precision = route === "precision";
  const version = precision ? "0.3.0" : STRESS_NEUTRAL_EXPORT_VERSION;
  const profile = precision ? "ops.stress_neutral.v3" : STRESS_NEUTRAL_EXPORT_PROFILE;
  let sourceCarrierChecksum;
  if (precision) {
    if (!numericalResultStanding(args.result, args.model).eligible) throw new Error("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    if (args.analysisRun.schema_version !== "0.3.0" || args.analysisRun.analysis_run.run_id !== args.result.run_id || args.model.project.id !== args.result.model_ref
      || args.analysisRun.analysis_run.solver_version?.solver_name !== args.result.producer!.component_name
      || args.analysisRun.analysis_run.solver_version?.solver_version !== args.result.producer!.component_version
      || !args.analysisRun.analysis_run.solver_version?.build_ref.ref
      || args.analysisRun.analysis_run.reproducibility.semantic_contract?.id !== PRECISION_CONTRACT_ID
      || args.analysisRun.analysis_run.reproducibility.semantic_contract?.sha256 !== PRECISION_CONTRACT_SHA256
      || await verifyAnalysisRunRecord(args.analysisRun) !== "match") throw new Error("SN-ANALYSIS-SOURCE-BINDING-MISMATCH");
    const hashes = args.analysisRun.analysis_run.hashes.filter(h => h.payload_scope === "received_result");
    if (hashes.length !== 1 || hashes[0].algorithm !== "sha256" || hashes[0].canonicalization !== "openpipestress_jcs_ijson_v1"
      || hashes[0].payload_ref.object_type !== "ResultEnvelope" || hashes[0].payload_ref.ref !== `result-envelope:${args.result.run_id}`
      || hashes[0].value !== await canonicalSha256HexCheckedV1(args.result)) throw new Error("SN-SOURCE-CARRIER-HASH-MISMATCH");
    await validateAnalysisRunV03(args.analysisRun, args.result, modelLoadBasisRefs(args.model));
    sourceCarrierChecksum = structuredClone(hashes[0]);
  }
  const legacy = buildStressNeutralExportPacketV01(args);
  const strictBoundaryNotes = legacy.boundary_notes.filter((note) =>
    !note.includes("does not emit canonical package member hashes")
  );
  strictBoundaryNotes.push("Result-row dimensions and witness eligibility are interpreted from the accepted semantic contract and bound analysis run; received numerical values, units, rows and source hashes remain unchanged, and no absent raw dimension is claimed as received evidence.");
  const dispositions = legacy.result_rows.map((row, rowIndex) => {
    const source = args.result.results.find((candidate) => candidate.id === row.result_id);
    const interpreted = source ? strictWitnessDisposition(source, args.result) : { disposition: "unknown_semantic" as const, dimension: null };
    return { row, rowIndex, source, ...interpreted };
  });
  const strictWitnesses = dispositions.filter((item) => item.disposition === "eligible").map(({ row, rowIndex }) => ({
    witness_id: precision ? `unit-witness:${rowIndex}` : `stress-neutral-unit:${safeFileToken(row.result_id)}`,
    source_row_index: rowIndex,
    result_id: row.result_id,
    source_quantity: { value: row.value, unit: row.unit, dimension: row.dimension },
    target_quantity: { value: row.value, unit: row.unit, dimension: row.dimension },
    conversion_performed: false,
    policy: "preserve_received_value_and_unit"
  }));
  const withheld = dispositions.filter((item): item is typeof item & { disposition: Exclude<StrictWitnessDisposition, "eligible"> } => item.disposition !== "eligible");
  const withheldDiagnostics = withheld.map(({ disposition, row, rowIndex }) => strictWithholdingDiagnostic(disposition, row, rowIndex, legacy.provenance, precision));
  const aggregateWithholdingDiagnostic = withheld.length ? [{
    code: "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", class: "export_blocking", severity: "blocking",
    source: reference("ExportConsumer", "DEL-17-06"), affected_object: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
    message: `${withheld.length} retained rows are explicitly categorized as ineligible for unit-preservation witnesses; ${strictWitnesses.length} rows have accepted semantic-contract interpretations and exact value/unit witnesses.`,
    remediation: "Review every explicit witness-withholding finding before downstream use.", provenance: previewProvenance()
  }] : [];
  const diagnostics = [
    ...legacy.diagnostics.filter((item) => !["SN-DESKTOP-PREVIEW-HASH-TBD", "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", "SN-UNIT-DIMENSION-MISSING"].includes(item.code)).map((item) => ({ code: item.code, class: item.class ?? "stress_neutral_export", severity: item.severity, source: item.source ?? reference("StressNeutralExportPackage", legacy.export_id), affected_object: item.affected_object ?? ("affected_ref" in item ? item.affected_ref : undefined) ?? reference("StressNeutralExportPackage", legacy.export_id), message: item.message, remediation: item.remediation ?? "Review stress-neutral handoff evidence before use.", provenance: item.provenance })),
    ...withheldDiagnostics,
    ...aggregateWithholdingDiagnostic
  ];
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const semanticContractRef = reference("ExternalReference", precision ? "fixtures/results/semantic_contract_v0_3_precision_1.json" : "fixtures/results/semantic_contract_v0_2.json");
  const decisionBasisRefs = [...structuredClone(legacy.unit_system_disclosure.decision_basis_refs), semanticContractRef, legacy.source_run_ref];
  const packet: any = {
    schema_version: version,
    deliverable_id: "DEL-17-06", package_id: "PKG-17", scope_items: ["SOW-046", "SOW-074"], objectives: ["OBJ-007", "OBJ-017", "OBJ-018"],
    export_id: legacy.export_id, package_status: "stress_neutral_export_package", schema_conformant: true,
    validation_ready: blockingCount === 0,
    source_result_ref: legacy.source_result_ref, source_run_ref: legacy.source_run_ref, source_model_ref: legacy.source_model_ref,
    received_source_checksums: structuredClone(args.analysisRun.analysis_run.hashes),
    unresolved_assumption_refs: [], reproducibility_refs: [legacy.source_run_ref], export_profile: { ...legacy.export_profile, profile_id: profile, profile_version: version, boundary_notes: strictBoundaryNotes, source_basis_refs: [legacy.source_run_ref, semanticContractRef] },
    manifest: { manifest_id: legacy.manifest.manifest_id, export_profile_ref: reference("StressNeutralExportProfile", profile), boundary_notes: strictBoundaryNotes, package_members: [] as any[], checksums: [] as any[] },
    csv_text: legacy.csv_text, result_rows: legacy.result_rows, unit_system_disclosure: { ...legacy.unit_system_disclosure, decision_basis_refs: decisionBasisRefs },
    unit_preservation_witnesses: strictWitnesses,
    stable_id_map: legacy.stable_id_map.map(({ canonical_ref, export_ref, mapping_status, loss_category, provenance }) => ({ canonical_ref, export_ref, mapping_status, loss_category, provenance })),
    loss_report: legacy.loss_report.entries.map((entry) => ({ loss_id: entry.loss_id, category: entry.category, severity: entry.severity, affected_refs: [entry.affected_ref], target_artifact_ref: reference("StressNeutralExportPackage", legacy.export_id), reason: entry.category === "exported" ? `All ${legacy.result_rows.length} received numerical rows and units are retained unchanged; ${strictWitnesses.length} rows have accepted semantic-contract dimension witnesses and ${withheld.length} rows have explicit witness-withholding findings.` : entry.reason, source_basis_ref: reference("Deliverable", "DEL-17-06"), downstream_implication: entry.downstream_implication, human_review_required: true, provenance: previewProvenance() })),
    validation_report: { validation_status: blockingCount ? "blocked" : "passed", checks: legacy.validation_report.checks.map((check) => check.check_id === "unit_preservation_witness_per_row"
      ? ({ check_id: check.check_id, check_status: withheld.length ? "blocking" : "passed", blocking_count: withheld.length, diagnostic_count: withheldDiagnostics.length, provenance: previewProvenance() })
      : ({ check_id: check.check_id, check_status: check.status, blocking_count: check.blocking ? 1 : 0, diagnostic_count: 0, provenance: previewProvenance() })), human_review_required: true, provenance: previewProvenance() },
    diagnostics,
    privacy: { classification: legacy.privacy.privacy_classification, commercial_tool_payload_embedded: false, local_only: true, private_payload_embedded: false, protected_payload_embedded: false, redaction_refs: [], telemetry_allowed: false },
    provenance: previewProvenance(),
    professional_boundary: {
      human_review_required: true,
      supports_review: true,
      supports_regression_comparison_input: true,
      supports_downstream_tooling: true,
      software_makes_release_claim: false,
      software_makes_external_compatibility_claim: false,
      software_makes_solver_validation_claim: false,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_creates_professional_reliance_record: false
    },
  };
  if (precision) Object.assign(packet, {
    producer: structuredClone(args.result.producer), numerical_quality: structuredClone(args.result.numerical_quality), formulation_basis: structuredClone(args.result.formulation_basis),
    semantic_contract: { id: PRECISION_CONTRACT_ID, sha256: PRECISION_CONTRACT_SHA256 },
    semantic_contract_ref: { ref_type: "semantic_contract", ref_id: PRECISION_CONTRACT_ID }, source_carrier_checksum: sourceCarrierChecksum
  });
  const payloads: Record<string, unknown> = {
    "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure,
    "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report,
    "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics,
  };
  const checksums = [];
  for (const filename of STRICT_MEMBER_NAMES.slice(1)) {
    const value = payloads[filename];
    checksums.push(filename.endsWith(".csv")
      ? { algorithm: "sha256" as const, canonicalization: "normalized_ascii_lf_text", payload_scope: "member_bytes", payload_ref: checksumRef(filename), value: await rawTextSha256(value as string) }
      : await checkedMemberChecksum(filename, value));
  }
  const manifestChecksum = await checkedMemberChecksum("manifest.json", strictManifestSeed(packet, checksums), "manifest_seed");
  packet.manifest.checksums = [...checksums, manifestChecksum];
  packet.manifest.package_members = STRICT_MEMBER_NAMES.map((filename) => ({ filename, checksum: packet.manifest.checksums.find((item: any) => item.payload_ref.ref === filename) }));
  packet.package_checksum = { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "complete_package_excluding_self_checksum", payload_ref: { object_type: "StressNeutralExportPackage", ref: packet.export_id }, value: await canonicalSha256HexCheckedV1(packet) };
  await validateStressNeutralExportPacket(packet, precision ? args.result : undefined, precision ? args.analysisRun : undefined, precision ? modelLoadBasisRefs(args.model) : undefined);
  return packet;
}

export async function validateStressNeutralExportPacket(packet: any, source?: MechanicsResult, analysisRun?: AnalysisRunEnvelope, expectedBasisRefs?: ObjectRef[]): Promise<void> {
  const precision = packet.schema_version === "0.3.0";
  if (precision && analysisRun !== undefined && source === undefined) throw new Error("SN-PRECISION-ANALYSIS-SOURCE-REQUIRED");
  if (source !== undefined) {
    if (!precision || sourceContract(source) !== "precision") throw new Error("SN-PRECISION-SOURCE-BINDING-REQUIRED");
    for (const value of [source.run_id, source.model_ref, ...source.results.map(r => r.id)]) if (typeof value !== "string" || !value) throw new Error("ANALYSIS_SOURCE_REFERENCE_INVALID");
    if (analysisRun !== undefined) await validateAnalysisRunV03(analysisRun, source, expectedBasisRefs);
    const same = async (a: unknown, b: unknown) => a !== undefined && b !== undefined && await canonicalSha256HexCheckedV1(a) === await canonicalSha256HexCheckedV1(b);
    for (const key of ["producer", "numerical_quality", "formulation_basis"] as const) if (!await same(packet[key], source[key])) throw new Error("SN-PRECISION-SOURCE-METADATA-MISMATCH");
    const received = { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_result", payload_ref: reference("ResultEnvelope", `result-envelope:${source.run_id}`), value: await canonicalSha256HexCheckedV1(source) };
    if (!await same(packet.source_model_ref, reference("Model", source.model_ref)) || !await same(packet.source_run_ref, reference("AnalysisRun", source.run_id)) || !await same(packet.source_result_ref, received.payload_ref) || !await same(packet.source_carrier_checksum, received) || !await same(packet.received_source_checksums?.filter((h: any) => h.payload_scope === "received_result"), [received])) throw new Error("SN-PRECISION-SOURCE-IDENTITY-MISMATCH");
    const rawById = new Map(source.results.map(row => [row.id, row]));
    const rows = packet.result_rows;
    if (!Array.isArray(rows) || rows.length !== source.results.length || rawById.size !== source.results.length || new Set(rows.map((r: any) => r?.result_id)).size !== rows.length) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
    const expectedWitnesses = [], expectedFindings = [];
    for (const [index, row] of rows.entries()) {
      const raw = rawById.get(row?.result_id);
      if (!raw) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      // Provenance belongs to the exporting package, not the source mechanics row.
      const { provenance: _provenance, ...expected } = precisionStressRow(raw, source);
      if (Object.keys(expected).some(key => !Object.hasOwn(row, key))) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      const actual = Object.fromEntries(Object.keys(expected).map(key => [key, row[key]]));
      if (!await same(actual, expected)) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      const { disposition } = strictWitnessDisposition(raw, source);
      const quantity = { value: row.value, unit: row.unit, dimension: row.dimension };
      if (disposition === "eligible") expectedWitnesses.push({ witness_id: `unit-witness:${index}`, source_row_index: index, result_id: row.result_id, source_quantity: quantity, target_quantity: quantity, conversion_performed: false, policy: "preserve_received_value_and_unit" });
      else expectedFindings.push(strictWithholdingDiagnostic(disposition, row, index, packet.provenance, true));
    }
    if (expectedFindings.length) expectedFindings.push({
      code: "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", class: "export_blocking", severity: "blocking",
      source: reference("ExportConsumer", "DEL-17-06"), affected_object: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
      message: `${expectedFindings.length} retained rows are explicitly categorized as ineligible for unit-preservation witnesses; ${expectedWitnesses.length} rows have accepted semantic-contract interpretations and exact value/unit witnesses.`,
      remediation: "Review every explicit witness-withholding finding before downstream use.", provenance: structuredClone(packet.provenance)
    });
    const codes = [...Object.values(WITHHOLDING_CODES), "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE"];
    if (!await same(packet.unit_preservation_witnesses, expectedWitnesses) || !await same(packet.diagnostics.filter((d: any) => codes.includes(d.code)), expectedFindings)) throw new Error("SN-PRECISION-WITNESS-BINDING-MISMATCH");
  }
  const version = precision ? "0.3.0" : STRESS_NEUTRAL_EXPORT_VERSION;
  const profile = precision ? "ops.stress_neutral.v3" : STRESS_NEUTRAL_EXPORT_PROFILE;
  const semanticPath = precision ? "fixtures/results/semantic_contract_v0_3_precision_1.json" : "fixtures/results/semantic_contract_v0_2.json";
  if (precision) {
    if (sourceContract({ schema_version: "0.2.0", producer: packet.producer, numerical_quality: packet.numerical_quality, formulation_basis: packet.formulation_basis } as MechanicsResult) !== "precision"
      || packet.semantic_contract?.id !== PRECISION_CONTRACT_ID || packet.semantic_contract?.sha256 !== PRECISION_CONTRACT_SHA256
      || packet.semantic_contract_ref?.ref_type !== "semantic_contract" || packet.semantic_contract_ref?.ref_id !== PRECISION_CONTRACT_ID) throw new Error("SN-PRECISION-CONTRACT-MISMATCH");
    const claim = packet.source_carrier_checksum;
    if (!claim || claim.algorithm !== "sha256" || claim.canonicalization !== "openpipestress_jcs_ijson_v1" || claim.payload_scope !== "received_result"
      || JSON.stringify(claim.payload_ref) !== JSON.stringify(packet.source_result_ref)
      || !packet.received_source_checksums?.some((h: unknown) => JSON.stringify(h) === JSON.stringify(claim))) throw new Error("SN-SOURCE-CARRIER-CHECKSUM-UNBOUND");
  }
  if ((!precision && packet.schema_version !== "0.2.0") || !Array.isArray(packet.manifest?.package_members) || !Array.isArray(packet.manifest?.checksums) || packet.manifest.package_members.length !== 9 || packet.manifest.checksums.length !== 9) throw new Error("SN-STRICT-INVENTORY-MISMATCH");
  if (packet.schema_conformant !== true) throw new Error("SN-SCHEMA-CONFORMANCE-CLAIM-INVALID");
  if (packet.export_profile?.profile_id !== profile || packet.export_profile?.profile_version !== version
    || packet.manifest?.export_profile_ref?.object_type !== "StressNeutralExportProfile" || packet.manifest?.export_profile_ref?.ref !== profile) throw new Error("SN-PROFILE-IDENTITY-MISMATCH");
  const exactNames = [...STRICT_MEMBER_NAMES];
  const memberNames = packet.manifest.package_members.map((item: any) => item.filename);
  const checksumNames = packet.manifest.checksums.map((item: any) => item?.payload_ref?.ref);
  if (JSON.stringify(memberNames) !== JSON.stringify(exactNames)) throw new Error("SN-MEMBER-ORDER-MISMATCH");
  if (new Set(memberNames).size !== 9 || new Set(checksumNames).size !== 9 || exactNames.some((name) => !checksumNames.includes(name))) throw new Error("SN-MEMBER-CHECKSUM-BIJECTION-MISMATCH");
  const metadata = (filename: string) => filename === "manifest.json"
    ? ["sha256", "openpipestress_jcs_ijson_v1", "manifest_seed", "StressNeutralMember", filename]
    : filename.endsWith(".csv")
      ? ["sha256", "normalized_ascii_lf_text", "member_bytes", "StressNeutralMember", filename]
      : ["sha256", "openpipestress_jcs_ijson_v1", "member_payload", "StressNeutralMember", filename];
  for (const [index, filename] of exactNames.entries()) {
    const claim = packet.manifest.checksums.find((item: any) => item?.payload_ref?.ref === filename);
    if (!claim || JSON.stringify([claim.algorithm, claim.canonicalization, claim.payload_scope, claim.payload_ref?.object_type, claim.payload_ref?.ref]) !== JSON.stringify(metadata(filename))) throw new Error(`SN-CHECKSUM-METADATA-MISMATCH: ${filename}`);
    if (packet.manifest.package_members[index]?.filename !== filename || JSON.stringify(packet.manifest.package_members[index]?.checksum) !== JSON.stringify(claim)) throw new Error(`SN-MEMBER-CHECKSUM-BINDING-MISMATCH: ${filename}`);
  }
  const packageMetadata = [packet.package_checksum?.algorithm, packet.package_checksum?.canonicalization, packet.package_checksum?.payload_scope, packet.package_checksum?.payload_ref?.object_type, packet.package_checksum?.payload_ref?.ref];
  if (JSON.stringify(packageMetadata) !== JSON.stringify(["sha256", "openpipestress_jcs_ijson_v1", "complete_package_excluding_self_checksum", "StressNeutralExportPackage", packet.export_id])) throw new Error("SN-PACKAGE-CHECKSUM-METADATA-MISMATCH");
  const received = new Set(packet.received_source_checksums.map((item: any) => JSON.stringify(item)));
  if (packet.manifest.checksums.some((item: any) => received.has(JSON.stringify(item)))) throw new Error("SN-RECEIVED-CHECKSUM-RELABEL");
  const payloads: Record<string, unknown> = { "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure, "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics };
  const nonManifest = packet.manifest.checksums.filter((item: any) => item.payload_ref.ref !== "manifest.json");
  for (const claim of nonManifest) {
    const expected = claim.payload_ref.ref.endsWith(".csv") ? await rawTextSha256(payloads[claim.payload_ref.ref] as string) : await canonicalSha256HexCheckedV1(payloads[claim.payload_ref.ref]);
    if (claim.value !== expected) throw new Error(`SN-MEMBER-CHECKSUM-MISMATCH: ${claim.payload_ref.ref}`);
  }
  const manifest = packet.manifest.checksums.find((item: any) => item.payload_ref.ref === "manifest.json");
  if (!manifest || manifest.value !== await canonicalSha256HexCheckedV1(strictManifestSeed(packet, nonManifest))) throw new Error("SN-MANIFEST-SEED-CHECKSUM-MISMATCH");
  const rows = packet.result_rows;
  if (!Array.isArray(rows) || new Set(rows.map((row: any) => row?.result_id)).size !== rows.length
    || rows.some((row: any) => typeof row?.result_id !== "string" || row?.canonical_ref?.ref !== row.result_id || row?.source_result_ref?.ref !== row.result_id)) throw new Error("SN-RESULT-ROW-IDENTITY-MISMATCH");
  if (packet.csv_text !== renderCsv(rows)) throw new Error("SN-CSV-ROW-BINDING-MISMATCH");
  if (!Array.isArray(packet.stable_id_map) || packet.stable_id_map.length !== rows.length
    || new Set(packet.stable_id_map.map((item: any) => item?.canonical_ref?.ref)).size !== rows.length
    || new Set(packet.stable_id_map.map((item: any) => precision ? JSON.stringify([item?.export_ref?.object_type, item?.export_ref?.ref]) : item?.export_ref?.ref)).size !== rows.length
    || packet.stable_id_map.some((item: any) => !rows.some((row: any) => row.result_id === item?.canonical_ref?.ref)
      || item?.mapping_status !== "mapped" || item?.loss_category !== "exported")) throw new Error("SN-STABLE-ID-MAP-BINDING-MISMATCH");
  if (precision && packet.stable_id_map.some((item: any) => item.canonical_ref?.object_type !== "Result" || Object.keys(item.canonical_ref).length !== 2 || !item.export_ref || typeof item.export_ref !== "object" || Array.isArray(item.export_ref) || Object.keys(item.export_ref).length !== 2 || typeof item.export_ref.object_type !== "string" || !item.export_ref.object_type || typeof item.export_ref.ref !== "string" || !item.export_ref.ref)) throw new Error("SN-STABLE-ID-MAP-BINDING-MISMATCH");
  if (!packet.received_source_checksums.some((item: any) => JSON.stringify(item.payload_ref) === JSON.stringify(packet.source_result_ref))) throw new Error("SN-SOURCE-RESULT-REF-UNBOUND");
  if (!packet.reproducibility_refs.some((item: any) => JSON.stringify(item) === JSON.stringify(packet.source_run_ref))) throw new Error("SN-SOURCE-RUN-REF-UNBOUND");
  const seenWitnesses = new Set<string>();
  const seenWitnessIds = new Set<string>();
  for (const witness of packet.unit_preservation_witnesses) {
    if (precision && (!Number.isSafeInteger(witness.source_row_index) || witness.source_row_index < 0 || witness.witness_id !== `unit-witness:${witness.source_row_index}` || seenWitnessIds.has(witness.witness_id))) throw new Error("SN-UNIT-WITNESS-BINDING-MISMATCH");
    seenWitnessIds.add(witness.witness_id);
    const row = rows[witness.source_row_index];
    if (!row || seenWitnesses.has(witness.result_id) || witness.result_id !== row.result_id || JSON.stringify(witness.source_quantity) !== JSON.stringify({ value: row.value, unit: row.unit, dimension: row.dimension }) || JSON.stringify(witness.target_quantity) !== JSON.stringify(witness.source_quantity) || witness.conversion_performed !== false || witness.policy !== "preserve_received_value_and_unit") throw new Error("SN-UNIT-WITNESS-BINDING-MISMATCH");
    seenWitnesses.add(witness.result_id);
  }
  for (const entry of packet.loss_report) if (entry.target_artifact_ref?.ref !== packet.export_id || entry.human_review_required !== true) throw new Error("SN-LOSS-REPORT-BINDING-MISMATCH");
  const withholdingCodes = new Map(Object.entries(WITHHOLDING_CODES).map(([category, code]) => [code, category]));
  const categorized = new Map<string, string>();
  for (const item of packet.diagnostics) {
    const category = withholdingCodes.get(item.code);
    if (!category) continue;
    const resultId = item.source?.ref;
    const expectedSeverity = category === "diagnostic_work" ? "info" : "blocking";
    if (typeof resultId !== "string" || !rows.some((row: any) => row.result_id === resultId)
      || categorized.has(resultId) || item.severity !== expectedSeverity) throw new Error("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH");
    categorized.set(resultId, category);
  }
  if ([...seenWitnesses].some((id) => categorized.has(id)) || seenWitnesses.size + categorized.size !== rows.length
    || rows.some((row: any) => !seenWitnesses.has(row.result_id) && !categorized.has(row.result_id))) throw new Error("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH");
  if (!packet.export_profile?.source_basis_refs?.some((item: any) => item?.ref === semanticPath)
    || !packet.unit_system_disclosure?.decision_basis_refs?.some((item: any) => item?.ref === semanticPath)) throw new Error("SN-SEMANTIC-CONTRACT-BINDING-MISSING");
  const blockingDiagnostics = packet.diagnostics.filter((item: any) => item.severity === "blocking").length;
  const checks = packet.validation_report?.checks;
  const checksConsistent = Array.isArray(checks) && checks.every((check: any) => Number.isSafeInteger(check.blocking_count) && check.blocking_count >= 0
    && ((check.check_status === "blocking" && check.blocking_count > 0) || (check.check_status === "passed" && check.blocking_count === 0)));
  if (!checksConsistent || packet.validation_report?.validation_status !== (blockingDiagnostics > 0 ? "blocked" : "passed")
    || checks.some((check: any) => check.check_status === "blocking") !== (blockingDiagnostics > 0)
    || packet.validation_ready !== (blockingDiagnostics === 0)) throw new Error("SN-VALIDATION-STATUS-BINDING-MISMATCH");
  const privacy = packet.privacy;
  if (!privacy || privacy.local_only !== true || privacy.commercial_tool_payload_embedded !== false || privacy.private_payload_embedded !== false
    || privacy.protected_payload_embedded !== false || privacy.telemetry_allowed !== false) throw new Error("SN-PRIVACY-BOUNDARY-VIOLATION");
  const boundary = packet.professional_boundary;
  if (!boundary || boundary.human_review_required !== true || [
    "software_makes_release_claim",
    "software_makes_external_compatibility_claim",
    "software_makes_solver_validation_claim",
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_creates_professional_reliance_record"
  ].some((key) => boundary[key] !== false)) throw new Error("SN-PROFESSIONAL-BOUNDARY-VIOLATION");
  const projection = structuredClone(packet); delete projection.package_checksum;
  if (packet.package_checksum.value !== await canonicalSha256HexCheckedV1(projection)) throw new Error("SN-PACKAGE-CHECKSUM-MISMATCH");
}

export function precisionStressRow(item: MechanicsResult["results"][number], source: MechanicsResult): StressNeutralRow {
  const { semantic } = analysisRowSemantics(item, source);
  const location = item.metadata?.location;
  if (location !== undefined && location !== null && (typeof location !== "string" || !location)) throw new Error("SN-PRECISION-ROW-LOCATION-INVALID");
  if (typeof item.entity_ref !== "string" || !item.entity_ref) throw new Error("SN-PRECISION-ROW-ENTITY-MISSING");
  const dimension = semantic?.derivative_target_dimension ?? "TBD";
  return { result_id: item.id, canonical_ref: reference("Result", item.id), row_kind: "result_value", result_family: semantic?.category === "physical_quantity" ? semantic.family ?? "other" : "other",
    load_case_ref: sourceBasisReference(item.basis_ref) ?? reference("AnalysisRun", source.run_id), station_ref: reference("Station", location ?? "summary"), component_ref: entityReference(item.entity_ref), value: item.value, unit: item.unit, dimension,
    correlation_status: item.unit && dimension !== "TBD" ? "canonical_id_map" : "unit_or_dimension_blocking_review_required", source_result_ref: reference("Result", item.id), provenance: previewProvenance() };
}

function stressNeutralRow(item: MechanicsResult["results"][number], runId: string, source: MechanicsResult): StressNeutralRow {
  const family = semanticFamily(item, source);
  const dimension = semanticDimension(item, source) ?? "TBD";
  return {
    result_id: item.id,
    canonical_ref: reference("Result", item.id),
    row_kind: "result_value",
    result_family: family,
    load_case_ref: basisReference(item.basis_ref, runId),
    station_ref: reference("Station", item.metadata?.location ?? "summary"),
    component_ref: entityReference(item.entity_ref),
    value: item.value,
    unit: item.unit,
    dimension,
    correlation_status: item.unit && dimension && dimension !== "TBD" ? "canonical_id_map" : "unit_or_dimension_blocking_review_required",
    source_result_ref: reference("Result", item.id),
    provenance: previewProvenance()
  };
}

function stressNeutralUnitPreservationWitnesses(rows: StressNeutralRow[], received: MechanicsResult["results"]): StressNeutralUnitPreservationWitness[] {
  return rows.filter(row => {const source=received.find(x=>x.id===row.result_id);return typeof source?.dimension === "string" && source.dimension === row.dimension;}).map((row) => ({
    witness_id: `stress-neutral-unit:${safeFileToken(row.result_id)}`,
    source_result_ref: row.source_result_ref,
    source_field_path: `results.${row.result_id}.value`,
    source_quantity: {
      value: row.value,
      unit: row.unit,
      dimension: row.dimension
    },
    target_row_ref: reference("StressNeutralResultRow", `stress-neutral-row:${safeFileToken(row.result_id)}`),
    target_field_path: `result_rows.${row.result_id}.value`,
    target_quantity: {
      value: row.value,
      unit: row.unit,
      dimension: row.dimension
    },
    export_unit_policy: "preserve_source_result_unit_and_dimension",
    conversion_performed: false,
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-17-06")
    ],
    provenance: previewProvenance()
  }));
}

function witnessMatchesRow(witness: StressNeutralUnitPreservationWitness, row: StressNeutralRow | undefined): boolean {
  return Boolean(
    row &&
      witness.source_result_ref.ref === row.source_result_ref.ref &&
      witness.target_row_ref.ref === `stress-neutral-row:${safeFileToken(row.result_id)}` &&
      witness.source_quantity.value === row.value &&
      witness.source_quantity.unit === row.unit &&
      witness.source_quantity.dimension === row.dimension &&
      witness.target_quantity.value === row.value &&
      witness.target_quantity.unit === row.unit &&
      witness.target_quantity.dimension === row.dimension &&
      witness.conversion_performed === false
  );
}

function member(role: string, path: string, format: string, recordCount: number) {
  return {
    role,
    path,
    format,
    record_count: recordCount,
    hash_status: HASH_STATUS_TBD
  };
}

function check(checkId: string, passed: boolean) {
  return {
    check_id: checkId,
    status: passed ? "passed" : "blocking",
    blocking: !passed
  };
}

type StressNeutralDiagnostic = {code:string;class?:string;severity:string;source?:StressNeutralRef;affected_object?:StressNeutralRef;message:string;remediation?:string;provenance:ReturnType<typeof previewProvenance>;diagnostic_id?:string;affected_ref?:StressNeutralRef};
function stressNeutralDiagnostics(rows: StressNeutralRow[]): StressNeutralDiagnostic[] {
  const missingRows = rows.filter((row) => !row.unit || !row.dimension || row.dimension === "TBD");
  const diagnostics: StressNeutralDiagnostic[] = [
    {
      diagnostic_id: "diagnostic:stress-neutral:hash-tbd",
      code: "SN-DESKTOP-PREVIEW-HASH-TBD",
      severity: "warning",
      message: "Browser preview records source hashes but does not emit canonical package member hashes.",
      affected_ref: reference("StressNeutralExportPackage", "stress-neutral:browser-preview"),
      provenance: previewProvenance()
    },
    {
      diagnostic_id: "diagnostic:stress-neutral:comparison-semantics-tbd",
      code: "SN-DESKTOP-PREVIEW-COMPARISON-TBD",
      severity: "warning",
      message: "Comparison pass/fail semantics and tolerance profiles are not defined by this preview export.",
      affected_ref: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
      provenance: previewProvenance()
    }
  ];
  if (missingRows.length > 0) {
    diagnostics.push({
      class: "export_blocking",
      source: reference("ExportConsumer", "DEL-17-06"),
      affected_object: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
      remediation: "Review bound canonical source semantics before interpreting unavailable physical dimensions.",
      code: "SN-UNIT-DIMENSION-MISSING",
      severity: "blocking",
      message: `${missingRows.length} result rows are missing unit or dimensional metadata.`,
      provenance: previewProvenance()
    });
  }
  return diagnostics;
}

function renderCsv(rows: StressNeutralRow[]): string {
  const body = rows.map((row) => CSV_COLUMNS.map((column) => csvValue(csvColumnValue(row, column))).join(","));
  return `${CSV_COLUMNS.join(",")}\n${body.join("\n")}\n`;
}

function csvColumnValue(row: StressNeutralRow, column: CsvColumn): string | number | StressNeutralRef {
  return row[column];
}

function csvValue(value: string | number | StressNeutralRef): string {
  const text = typeof value === "object" ? value.ref : String(value);
  if (/[",\n\r]/.test(text)) return `"${text.replaceAll('"', '""')}"`;
  return text;
}

function basisReference(item: ResultBasisRef | undefined, runId: string) {
  if (!item) return reference("AnalysisRun", runId);
  if (item.ref_type === "load_case") return reference("LoadCase", item.ref_id);
  if (item.ref_type === "combination") return reference("Combination", item.ref_id);
  return reference("ResultBasis", item.ref_id);
}

function entityReference(value: string) {
  if (value.startsWith("pipe:")) return reference("PipeElement", value);
  if (value.startsWith("node:")) return reference("Node", value);
  if (value.startsWith("support:")) return reference("Support", value);
  if (value.startsWith("component:")) return reference("Component", value);
  if (value.startsWith("material:")) return reference("Material", value);
  return reference("CanonicalObject", value);
}


function unitCount(packet: Awaited<ReturnType<typeof buildStressNeutralExportPacket>>): number {
  return new Set(packet.result_rows.map((row: any) => row.unit)).size;
}

function dimensionSummary(packet: Awaited<ReturnType<typeof buildStressNeutralExportPacket>>): string {
  return Array.from(new Set<string>(packet.result_rows.map((row: any) => row.dimension))).sort().join(", ");
}

function professionalBoundary() {
  return {
    supports_review: true,
    supports_regression_comparison_input: true,
    supports_downstream_tooling: true,
    supports_release_readiness: false,
    supports_external_target_compatibility: false,
    supports_solver_validation: false,
    supports_code_compliance: false,
    supports_certification: false,
    supports_sealing: false,
    supports_approval: false,
    supports_professional_reliance_record: false,
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function reference(objectType: string, ref: string): StressNeutralRef {
  return {
    object_type: objectType,
    ref
  };
}

function previewProvenance() {
  return {
    source_name: "SWBPIPE desktop",
    source_location: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
    source_license: "MIT project license context",
    contributor: "SWBPIPE app integration tranche",
    contributor_certification: "Invented preview metadata only; no protected standards or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function csvDataHref(csvText: string): string {
  return `data:text/csv;charset=utf-8,${encodeURIComponent(csvText)}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
