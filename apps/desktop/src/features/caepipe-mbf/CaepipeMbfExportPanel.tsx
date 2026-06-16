import { Download, FileText } from "lucide-react";
import { usePackageHash, withCanonicalPackageHash } from "../../services/usePackageHash";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewModel } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

type CaePipeMbfReference = {
  object_type: string;
  ref: string;
};

type CaePipeMbfConversionWitness = {
  witness_id: string;
  source_ref: CaePipeMbfReference;
  field_path: string;
  source_quantity: {
    value: number;
    unit: string;
    dimension: "length";
  };
  target_quantity: {
    value: number;
    unit: "mm";
    target_field: string;
  };
  conversion_factor_to_target: number;
  conversion_status: "converted_to_target_unit" | "source_unit_matches_target_unit";
  decision_basis_refs: CaePipeMbfReference[];
  provenance: ReturnType<typeof previewProvenance>;
};

type CaePipeMbfLossCategory = "exported" | "omitted" | "approximated" | "delegated" | "unsupported" | "tbd";

const CAEPIPE_MBF_EXPORT_VERSION = "0.1.0";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";
const CAEPIPE_MBF_TARGET_LENGTH_UNIT = "mm";
const TARGET_VERSION_TBD = "TBD-17-01-001";
const RECORD_SUBSET_TBD = "TBD-17-01-002";
const DIRECT_STABLE_ID_TBD = "TBD-17-01-003";
const REQUIRED_LOSS_CATEGORIES: CaePipeMbfLossCategory[] = [
  "exported",
  "omitted",
  "approximated",
  "delegated",
  "unsupported",
  "tbd"
];

export function CaepipeMbfExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const basePacket = buildCaePipeMbfExportPacket({ model, result, analysisRun });
  const packageHash = usePackageHash(
    basePacket.manifest.manifest_id,
    basePacket,
    "manifest_and_validation_report_package_hash_carrier_fields"
  );
  const packet = withCanonicalPackageHash(basePacket, packageHash);

  return (
    <section className="panel caepipe-mbf-panel" aria-label="CAEPIPE MBF export" data-testid="caepipe-mbf-panel">
      <div className="panel-title">
        <FileText size={16} aria-hidden="true" />
        CAEPIPE MBF Export
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="caepipe-mbf-export-link"
          download={`openpipestress-preview-caepipe-mbf-package-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Package JSON
        </a>
        <a
          className="report-export-link"
          data-testid="caepipe-mbf-text-link"
          download={`openpipestress-preview-${safeFileToken(model.project.id)}.mbf`}
          href={textDataHref(packet.mbf_text)}
        >
          <Download size={14} aria-hidden="true" />
          MBF text
        </a>
        <span data-testid="caepipe-mbf-summary">
          available; nodes={packet.model_payload.nodes.length}; pipes={packet.model_payload.elements.length};
          supports={packet.model_payload.supports.length}; validation={packet.validation_report.validation_status};
          losses={packet.loss_report.length}
        </span>
      </div>
      <div className="report-list" data-testid="caepipe-mbf-body">
        <CaePipeMbfLine
          label="Profile"
          value={`${packet.export_profile.target_family}; profile=${packet.export_profile.profile_id}; target_version=${packet.export_profile.target_version_basis}; subset=${packet.export_profile.record_subset_basis}`}
          testId="caepipe-mbf-profile"
        />
        <CaePipeMbfLine
          label="State binding"
          value={`${packet.source_model_ref.ref}; ${packet.model_state_ref.ref}; ${packet.analysis_run_ref.ref}`}
          testId="caepipe-mbf-state-binding"
        />
        <CaePipeMbfLine
          label="Coverage"
          value="emitted=nodes,pipes,supports,load_cases; omitted=components; unsupported=boundary_conditions; tbd=target_version,record_subset,direct_stable_id"
          testId="caepipe-mbf-coverage"
        />
        <CaePipeMbfLine
          label="Units"
          value={unitDisclosureSummary(packet.unit_system_disclosure)}
          testId="caepipe-mbf-units"
        />
        <CaePipeMbfLine
          label="Conversion witnesses"
          value={`count=${packet.conversion_witnesses.length}; scope=node.coordinates; target_length=${packet.model_payload.units.length}`}
          testId="caepipe-mbf-conversion-witnesses"
        />
        <CaePipeMbfLine
          label="Package"
          value={`members=${packet.manifest.package_members.length}; stable_ids=${packet.stable_id_map.length}; diagnostics=${packet.diagnostics.length}; mbf_lines=${packet.manifest.mbf_artifact.line_count}; package_hash=${packet.manifest.canonical_package_hash_status}`}
          testId="caepipe-mbf-package"
        />
        <CaePipeMbfLine
          label="Boundary"
          value={`caepipe_compatibility=${String(packet.professional_boundary.software_makes_caepipe_compatibility_claim)}; external_tool_invoked=${String(packet.professional_boundary.external_tool_invoked)}; solver_validation=${String(packet.professional_boundary.software_makes_solver_validation_claim)}; code_compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; professional_reliance=${String(packet.professional_boundary.software_creates_professional_reliance_record)}`}
          testId="caepipe-mbf-boundary"
        />
      </div>
      <small className="report-note">
        CAEPIPE MBF export is a local invented smoke-subset package only; target version, record subset, direct stable-ID
        carrying, external execution, and target result interpretation remain governed TBDs.
      </small>
    </section>
  );
}

function CaePipeMbfLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildCaePipeMbfExportPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const sourceModelRef = reference("Model", model.project.id);
  const modelStateRef = run?.model_state_ref ?? reference("ModelState", "state:TBD");
  const analysisRunRef = run ? reference("AnalysisRun", run.run_id) : reference("AnalysisRun", "not generated");
  const payload = caepipeMbfPayload(model);
  const exportProfile = caepipeMbfExportProfile();
  const mbfText = renderCaePipeMbfText(payload, exportProfile);
  const conversionWitnesses = caepipeMbfConversionWitnesses(model);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: payload.units,
    conversionPolicy: "caepipe_mbf_smoke_subset_converts_node_coordinates_to_millimeters_with_source_unit_witnesses",
    conversionPerformed: true,
    conversionScope: ["node.coordinates"],
    sourceLocation: "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx"
  });
  const stableIdMap = caepipeMbfStableIdMap(payload);
  const lossReport = caepipeMbfLossReport({ model, result, analysisRunRef });
  const diagnostics = caepipeMbfDiagnostics({ model, payload, stableIdMap, lossReport, exportProfile, mbfText });
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const validationStatus = blockingCount === 0 ? "boundary_checked" : "blocked";

  return {
    schema_version: CAEPIPE_MBF_EXPORT_VERSION,
    document_kind: "openpipestress.technical_preview.caepipe_mbf_export_package",
    deliverable_id: "DEL-17-04",
    package_id: "PKG-17",
    scope_items: ["SOW-030", "SOW-074", "SOW-075"],
    objectives: ["OBJ-009", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    export_id: `caepipe-mbf:${safeFileToken(model.project.id)}`,
    package_status: "caepipe_mbf_export_foundation",
    source_model_ref: sourceModelRef,
    source_model_hash_status: "TBD_browser_preview_does_not_emit_canonical_model_hash",
    model_state_ref: modelStateRef,
    analysis_run_ref: analysisRunRef,
    result_ref: result ? reference("MechanicsResult", result.run_id) : reference("MechanicsResult", "not generated"),
    export_profile: exportProfile,
    unit_system_disclosure: unitSystemDisclosure,
    conversion_witnesses: conversionWitnesses,
    manifest: {
      manifest_id: `caepipe-mbf:${safeFileToken(model.project.id)}:manifest`,
      source_model_ref: sourceModelRef,
      source_model_hash_status: "TBD_browser_preview_does_not_emit_canonical_model_hash",
      export_profile_ref: reference("ExportProfile", exportProfile.profile_id),
      source_basis_refs: exportProfile.source_basis_refs,
      carried_tbd_refs: exportProfile.carried_tbd_refs,
      mbf_artifact: {
        artifact_id: `caepipe-mbf:${safeFileToken(model.project.id)}:model.mbf`,
        artifact_kind: "caepipe_mbf_text",
        path: "model.mbf",
        encoding: "ascii",
        line_count: mbfText.trimEnd().split("\n").length,
        hash_status: HASH_STATUS_TBD
      },
      package_members: [
        member("manifest", "manifest.json", "json", 1),
        member("mbf_text", "model.mbf", "text/plain", mbfText.trimEnd().split("\n").length),
        member("unit_system_disclosure", "unit_system_disclosure.json", "json", 1),
        member("conversion_witnesses", "conversion_witnesses.json", "json", conversionWitnesses.length),
        member("stable_id_map", "stable_id_map.json", "json", stableIdMap.length),
        member("loss_report", "loss_report.json", "json", lossReport.length),
        member("validation_report", "validation_report.json", "json", 1),
        member("diagnostics", "diagnostics.json", "json", diagnostics.length)
      ],
      boundary_notes: exportProfile.boundary_notes,
      diagnostics
    },
    mbf_text: mbfText,
    model_payload: payload,
    stable_id_map: stableIdMap,
    loss_report: lossReport,
    validation_report: {
      validation_status: validationStatus,
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      blocking_count: blockingCount,
      checks: [
        check("mbf_text_has_terminal_record", mbfText.endsWith("END\n")),
        check("smoke_subset_has_nodes_and_pipe_elements", payload.nodes.length > 0 && payload.elements.length > 0),
        check(
          "conversion_witness_per_node_coordinate_field",
          conversionWitnesses.length === expectedCaePipeMbfConversionWitnessCount(model)
        ),
        check(
          "conversion_witnesses_target_millimeters",
          conversionWitnesses.every((witness) => witness.target_quantity.unit === CAEPIPE_MBF_TARGET_LENGTH_UNIT)
        ),
        check("sidecar_id_map_present", stableIdMap.length > 0),
        check("loss_report_has_required_taxonomy", requiredLossCategoriesPresent(lossReport)),
        check("profile_tbd_refs_carried", REQUIRED_LOSS_CATEGORIES.length === 6 && exportProfile.carried_tbd_refs.includes(DIRECT_STABLE_ID_TBD)),
        check("external_execution_not_invoked", exportProfile.external_execution_policy === "not_invoked_by_this_package")
      ],
      human_review_required: true,
      provenance: previewProvenance()
    },
    diagnostics,
    privacy: {
      classification: "invented_public_example",
      local_only: true,
      telemetry_allowed: false,
      private_payload_embedded: false,
      protected_payload_embedded: false,
      commercial_tool_payload_embedded: false,
      redaction_refs: []
    },
    data_boundary: {
      public_examples_policy: model.data_boundary.public_examples_policy,
      private_payload_included: false,
      protected_content_included: false,
      commercial_tool_payload_embedded: false,
      copied_commercial_solver_file_included: false
    },
    provenance: previewProvenance(),
    professional_boundary: {
      human_review_required: true,
      supports_interchange_smoke_testing: true,
      supports_export_review: true,
      external_tool_invoked: false,
      commercial_result_payload_ingested: false,
      software_makes_release_claim: false,
      software_makes_caepipe_compatibility_claim: false,
      software_makes_solver_validation_claim: false,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_creates_professional_reliance_record: false
    },
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    caepipe_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false
  };
}

function caepipeMbfExportProfile() {
  return {
    profile_id: "ops.caepipe_mbf.smoke_tbd",
    profile_version: CAEPIPE_MBF_EXPORT_VERSION,
    target_family: "caepipe_mbf",
    target_version_basis: TARGET_VERSION_TBD,
    record_subset_basis: RECORD_SUBSET_TBD,
    stable_id_policy: "sidecar_mapping_until_direct_mbf_carrier_confirmed",
    unit_policy: "explicit_units_record_required",
    loss_report_policy: "mandatory_for_every_package",
    external_execution_policy: "not_invoked_by_this_package",
    source_basis_refs: [
      reference("Deliverable", "DEL-17-01"),
      reference("Deliverable", "DEL-17-02"),
      reference("SourceID", "CAEPIPE-IMPORT-MBF"),
      reference("SourceID", "CAEPIPE-EXPORT-MBF")
    ],
    carried_tbd_refs: [TARGET_VERSION_TBD, RECORD_SUBSET_TBD, DIRECT_STABLE_ID_TBD],
    boundary_notes: [
      "CAEPIPE MBF output is a bounded desktop smoke-subset package.",
      "Target version, record subset closure, and direct MBF stable-ID carrying remain TBD.",
      "Sidecar stable-ID mapping is the conservative default for this preview.",
      "The package does not assert CAEPIPE compatibility, solver validation, release readiness, code compliance, or professional reliance."
    ]
  };
}

function caepipeMbfPayload(model: PreviewModel) {
  const nodeTargetById = new Map<string, string>();
  const sourceLengthUnit = model.project.units.length ?? "m";
  const nodes = model.nodes
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((node, index) => {
      const nodeTargetId = targetId("N", index);
      nodeTargetById.set(node.id, nodeTargetId);
      return {
        node_id: node.id,
        target_id: nodeTargetId,
        x: convertLengthToMillimeters(node.position.x, sourceLengthUnit).value,
        y: convertLengthToMillimeters(node.position.y, sourceLengthUnit).value,
        z: convertLengthToMillimeters(node.position.z, sourceLengthUnit).value
      };
    });

  const elements = model.pipe_segments
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((segment, index) => ({
      element_id: segment.id,
      target_id: targetId("P", index),
      from_node: nodeTargetById.get(segment.from) ?? "N-TBD",
      to_node: nodeTargetById.get(segment.to) ?? "N-TBD",
      section_ref: `section:${segment.id}`,
      material_ref: segment.material
    }));

  const supports = model.supports
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((support, index) => ({
      support_id: support.id,
      target_id: targetId("S", index),
      node: nodeTargetById.get(support.node) ?? "N-TBD",
      support_kind: supportKind(support.restraints)
    }));

  const loadCases = model.load_cases
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((loadCase, index) => ({
      load_case_id: loadCase.id,
      target_id: targetId("L", index),
      load_kind: safeMbfField(loadCase.kind)
    }));

  return {
    payload_kind: "caepipe_mbf_smoke_subset_payload",
    units: {
      length: CAEPIPE_MBF_TARGET_LENGTH_UNIT,
      force: model.project.units.force ?? "N",
      temperature: model.project.units.temperature ?? "C"
    },
    nodes,
    elements,
    supports,
    load_cases: loadCases,
    unsupported_entities: [reference("BoundaryCondition", "free_end_and_equipment_nozzle_semantics")],
    provenance: previewProvenance()
  };
}

function caepipeMbfConversionWitnesses(model: PreviewModel): CaePipeMbfConversionWitness[] {
  return model.nodes
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .flatMap((node) =>
      (["x", "y", "z"] as const).map((axis) =>
        conversionWitness({
          witnessId: `caepipe-mbf-conversion:${safeFileToken(node.id)}:position:${axis}`,
          sourceRef: reference("Node", node.id),
          fieldPath: `nodes.${node.id}.position.${axis}`,
          sourceValue: node.position[axis],
          sourceUnit: model.project.units.length ?? "m",
          targetField: `model_payload.nodes.${node.id}.${axis}`
        })
      )
    );
}

function conversionWitness({
  witnessId,
  sourceRef,
  fieldPath,
  sourceValue,
  sourceUnit,
  targetField
}: {
  witnessId: string;
  sourceRef: CaePipeMbfReference;
  fieldPath: string;
  sourceValue: number;
  sourceUnit: string;
  targetField: string;
}): CaePipeMbfConversionWitness {
  const conversion = convertLengthToMillimeters(sourceValue, sourceUnit);
  return {
    witness_id: witnessId,
    source_ref: sourceRef,
    field_path: fieldPath,
    source_quantity: {
      value: sourceValue,
      unit: sourceUnit,
      dimension: "length"
    },
    target_quantity: {
      value: conversion.value,
      unit: CAEPIPE_MBF_TARGET_LENGTH_UNIT,
      target_field: targetField
    },
    conversion_factor_to_target: conversion.factor,
    conversion_status: conversion.factor === 1 ? "source_unit_matches_target_unit" : "converted_to_target_unit",
    decision_basis_refs: [reference("Decision", "DEC-018"), reference("Deliverable", "DEL-02-02")],
    provenance: previewProvenance()
  };
}

function expectedCaePipeMbfConversionWitnessCount(model: PreviewModel): number {
  return model.nodes.length * 3;
}

function renderCaePipeMbfText(
  payload: ReturnType<typeof caepipeMbfPayload>,
  exportProfile: ReturnType<typeof caepipeMbfExportProfile>
): string {
  const lines = [
    "$ OpenPipeStress invented CAEPIPE MBF smoke subset",
    "$ Source basis: DEL-17-01 / DEL-17-02 / DEL-17-04",
    `$ Target version basis: ${exportProfile.target_version_basis}`,
    "$ Canonical IDs are preserved in sidecar mapping, not asserted as direct MBF fields",
    "UNITS",
    record("UNIT", [payload.units.length, payload.units.force, payload.units.temperature]),
    "NODES",
    ...payload.nodes.map((node) => record("NODE", [node.target_id, formatNumber(node.x), formatNumber(node.y), formatNumber(node.z)])),
    "ELEMENTS",
    ...payload.elements.map((element) =>
      record("PIPE", [element.target_id, element.from_node, element.to_node, element.section_ref, element.material_ref])
    ),
    "SUPPORTS",
    ...payload.supports.map((support) => record("SUPPORT", [support.target_id, support.node, support.support_kind])),
    "LOAD_CASES",
    ...payload.load_cases.map((loadCase) => record("LOADCASE", [loadCase.target_id, loadCase.load_kind])),
    "END"
  ];
  return `${lines.join("\n")}\n`;
}

function caepipeMbfStableIdMap(payload: ReturnType<typeof caepipeMbfPayload>) {
  return [
    ...payload.nodes.map((node) => stableIdEntry(reference("Node", node.node_id), node.target_id)),
    ...payload.elements.map((element) => stableIdEntry(reference("PipeElement", element.element_id), element.target_id)),
    ...payload.supports.map((support) => stableIdEntry(reference("Support", support.support_id), support.target_id)),
    ...payload.load_cases.map((loadCase) => stableIdEntry(reference("LoadCase", loadCase.load_case_id), loadCase.target_id))
  ];
}

function caepipeMbfLossReport({
  model,
  result,
  analysisRunRef
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRunRef: ObjectRef;
}) {
  const targetArtifactRef = reference("CaePipeMbfMember", `caepipe-mbf:${safeFileToken(model.project.id)}:mbf_text`);
  return [
    loss(
      "loss:desktop-preview:exported-smoke-subset",
      "exported",
      "info",
      [...model.nodes.map((node) => reference("Node", node.id)), ...model.pipe_segments.map((segment) => reference("PipeElement", segment.id))],
      targetArtifactRef,
      "Invented node and straight pipe records were emitted for deterministic browser package review.",
      reference("Deliverable", "DEL-17-02"),
      RECORD_SUBSET_TBD,
      "Does not close broad MBF record-family coverage."
    ),
    loss(
      "loss:desktop-preview:components-omitted",
      "omitted",
      "warning",
      model.components.map((component) => reference("Component", component.id)),
      targetArtifactRef,
      "Invented component markers are not emitted as CAEPIPE MBF target component records in this smoke subset.",
      reference("Deliverable", "DEL-17-04"),
      RECORD_SUBSET_TBD,
      "A later source-confirmed profile tranche must classify component records before target use."
    ),
    loss(
      "loss:desktop-preview:supports-approximated",
      "approximated",
      "warning",
      model.supports.map((support) => reference("Support", support.id)),
      targetArtifactRef,
      "Invented supports are represented only as minimal smoke-subset support records.",
      reference("Deliverable", "DEL-17-04"),
      RECORD_SUBSET_TBD,
      "Detailed support target behavior remains source-gated and must not be inferred from this preview."
    ),
    loss(
      "loss:desktop-preview:external-execution-delegated",
      "delegated",
      "warning",
      [analysisRunRef],
      reference("CaePipeMbfMember", `caepipe-mbf:${safeFileToken(model.project.id)}:manifest`),
      "External CAEPIPE execution and target-side options are not invoked or interpreted by this desktop package.",
      reference("Deliverable", "DEL-17-04"),
      "TBD-17-04-004",
      result
        ? "Mechanics result context exists locally, but external execution and target result interpretation remain later guarded work."
        : "Run-independent export context exists locally, but external execution and target result interpretation remain later guarded work."
    ),
    loss(
      "loss:desktop-preview:unsupported-boundary-semantics",
      "unsupported",
      "warning",
      [reference("BoundaryCondition", "free_end_and_equipment_nozzle_semantics")],
      targetArtifactRef,
      "Boundary-condition and equipment-connection semantics are not supported by the first CAEPIPE MBF smoke subset.",
      reference("Deliverable", "DEL-17-04"),
      "TBD-17-04-004",
      "Unsupported boundary semantics must remain visible until a later profile tranche classifies support."
    ),
    loss(
      "loss:desktop-preview:profile-basis-tbd",
      "tbd",
      "warning",
      [
        reference("TargetVersion", TARGET_VERSION_TBD),
        reference("RecordSubset", RECORD_SUBSET_TBD),
        reference("StableIdCarrier", DIRECT_STABLE_ID_TBD)
      ],
      reference("CaePipeMbfMember", `caepipe-mbf:${safeFileToken(model.project.id)}:stable_id_map`),
      "CAEPIPE target version, MBF record-family subset, and direct stable-ID carrying remain open TBDs.",
      reference("Deliverable", "DEL-17-01"),
      DIRECT_STABLE_ID_TBD,
      "Downstream MBF work must keep sidecar mapping and source-basis TBDs until admitted evidence closes them."
    )
  ] as const;
}

function caepipeMbfDiagnostics({
  model,
  payload,
  stableIdMap,
  lossReport,
  exportProfile,
  mbfText
}: {
  model: PreviewModel;
  payload: ReturnType<typeof caepipeMbfPayload>;
  stableIdMap: ReturnType<typeof caepipeMbfStableIdMap>;
  lossReport: ReturnType<typeof caepipeMbfLossReport>;
  exportProfile: ReturnType<typeof caepipeMbfExportProfile>;
  mbfText: string;
}) {
  const diagnostics = [];
  if (payload.nodes.length === 0 || payload.elements.length === 0) {
    diagnostics.push(diagnostic("MBF-SMOKE-SUBSET-MISSING", "blocking", "EXPORT_BLOCKING", "CAEPIPE MBF smoke subset requires at least one node and one pipe element.", "Provide invented node and pipe element records or record an explicit omitted/unsupported loss.", reference("ExportProfile", exportProfile.profile_id)));
  }
  if (!isAscii(mbfText)) {
    diagnostics.push(diagnostic("MBF-TEXT-NON-ASCII", "blocking", "EXPORT_BLOCKING", "Rendered MBF text contains non-ASCII characters.", "Use ASCII-safe invented labels for MBF text output.", reference("ExportProfile", exportProfile.profile_id)));
  }
  if (!isSupportedMillimeterConversionUnit(model.project.units.length ?? "m")) {
    diagnostics.push(
      diagnostic(
        "MBF-NODE-COORDINATE-UNIT-TBD",
        "blocking",
        "UNIT_WARNING",
        `Preview model length unit ${model.project.units.length} is not mapped to the CAEPIPE MBF millimeter node-coordinate policy.`,
        "Select an explicit coordinate conversion policy before exporting CAEPIPE MBF text.",
        reference("Project", model.project.id)
      )
    );
  }
  if (stableIdMap.length === 0) {
    diagnostics.push(diagnostic("MBF-STABLE-ID-SIDECAR-MISSING", "blocking", "TARGET_MAPPING_WARNING", "CAEPIPE MBF package has no sidecar stable ID map entries.", "Use sidecar mapping while direct MBF stable-ID carrying remains TBD-17-01-003.", reference("ExportProfile", exportProfile.profile_id)));
  }
  if (!requiredLossCategoriesPresent(lossReport)) {
    diagnostics.push(diagnostic("MBF-LOSS-REPORT-MISSING", "blocking", "UNSUPPORTED_BEHAVIOR_WARNING", "CAEPIPE MBF package does not carry every required loss-report category.", "Record exported, omitted, approximated, delegated, unsupported, and tbd behavior explicitly.", reference("ExportProfile", exportProfile.profile_id)));
  }
  return diagnostics;
}

function stableIdEntry(canonicalRef: CaePipeMbfReference, targetRefId: string) {
  return {
    canonical_ref: canonicalRef,
    target_ref: reference("CaePipeMbfRecord", targetRefId),
    carrier_mode: "sidecar_mapping",
    mapping_status: "mapped_sidecar",
    loss_category: "exported",
    provenance: previewProvenance()
  };
}

function loss(
  lossId: string,
  category: CaePipeMbfLossCategory,
  severity: "info" | "warning" | "blocking",
  affectedRefs: CaePipeMbfReference[],
  targetArtifactRef: CaePipeMbfReference,
  reason: string,
  sourceBasisRef: CaePipeMbfReference,
  governingTbdId: string,
  downstreamImplication: string
) {
  return {
    loss_id: lossId,
    category,
    severity,
    affected_refs: affectedRefs,
    target_artifact_ref: targetArtifactRef,
    reason,
    source_basis_ref: sourceBasisRef,
    governing_tbd_id: governingTbdId,
    downstream_implication: downstreamImplication,
    human_review_required: true,
    provenance: previewProvenance()
  };
}

function requiredLossCategoriesPresent(lossReport: ReturnType<typeof caepipeMbfLossReport>): boolean {
  const categories = new Set(lossReport.map((entry) => entry.category));
  return REQUIRED_LOSS_CATEGORIES.every((category) => categories.has(category));
}

function member(memberRole: string, path: string, contentKind: "json" | "text/plain", recordCount: number) {
  return {
    member_name: `caepipe-mbf:${memberRole}`,
    member_role: memberRole,
    path,
    content_kind: contentKind,
    record_count: recordCount,
    hash_status: HASH_STATUS_TBD,
    payload_embedded_in_download: true
  };
}

function check(checkId: string, passed: boolean) {
  return {
    check_id: checkId,
    check_status: passed ? "passed" : "blocked",
    provenance: previewProvenance()
  };
}

function diagnostic(
  code: string,
  severity: "info" | "warning" | "blocking",
  diagnosticClass: string,
  message: string,
  remediation: string,
  affectedObject: CaePipeMbfReference
) {
  return {
    code,
    class: diagnosticClass,
    severity,
    source: reference("Deliverable", "DEL-17-04"),
    affected_object: affectedObject,
    message,
    remediation,
    provenance: previewProvenance()
  };
}

function supportKind(restraints: string[]): string {
  if (["UX", "UY", "UZ", "RX", "RY", "RZ"].every((restraint) => restraints.includes(restraint))) {
    return "ANCHOR";
  }
  if (restraints.length > 0) return "GUIDE";
  return "SUPPORT";
}

function targetId(prefix: string, index: number): string {
  return `${prefix}${String(index + 1).padStart(3, "0")}`;
}

function record(name: string, fields: string[]): string {
  return [name, ...fields].map(safeMbfField).join(",");
}

function reference(objectType: string, ref: string): CaePipeMbfReference {
  return { object_type: objectType, ref };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop CAEPIPE MBF export preview",
    source_location: "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
    source_license: "project-governed",
    contributor: "OpenPipeStress desktop technical preview",
    contributor_certification: "invented-public-preview-no-professional-claim",
    redistribution_status: "public_permissive",
    review_status: "desktop_preview",
    privacy_classification: "public_metadata"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function textDataHref(value: string): string {
  return `data:text/plain;charset=utf-8,${encodeURIComponent(value)}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

function safeMbfField(value: string): string {
  return value.replace(/[,\n\r]/g, "_").replace(/[^\x20-\x7E]/g, "_");
}

function convertLengthToMillimeters(value: number, unit: string): { value: number; factor: number } {
  const factor = unit === "m" ? 1000 : 1;
  return {
    value: Number((value * factor).toFixed(6)),
    factor
  };
}

function isSupportedMillimeterConversionUnit(unit: string): boolean {
  return ["m", CAEPIPE_MBF_TARGET_LENGTH_UNIT].includes(unit);
}

function formatNumber(value: number): string {
  if (Number.isInteger(value)) return String(value);
  return value.toFixed(6).replace(/0+$/g, "").replace(/\.$/, "");
}

function isAscii(value: string): boolean {
  return /^[\x00-\x7F]*$/.test(value);
}
