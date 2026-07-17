import { Download, FileText } from "lucide-react";
import { usePackageHash, withCanonicalPackageHash } from "../../services/usePackageHash";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewModel } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

type PcfReference = {
  object_type: string;
  ref: string;
};

type PcfDiagnostic = {
  code: string;
  class: string;
  severity: "info" | "warning" | "blocking";
  source: PcfReference;
  affected_object: PcfReference;
  message: string;
  remediation: string;
  provenance: ReturnType<typeof previewProvenance>;
};

type PcfPayloadNode = {
  node_id: string;
  x: number;
  y: number;
  z: number;
};

type PcfPayloadSegment = {
  element_id: string;
  from_node: string;
  to_node: string;
  target_component_identifier: string;
  nominal_size: "TBD_SOURCE_REQUIRED";
  outside_diameter: string;
  wall_thickness: string;
  item_code: string;
  material_label: string;
};

type PcfConversionWitness = {
  witness_id: string;
  source_ref: PcfReference;
  field_path: string;
  source_quantity: {
    value: number;
    unit: string;
    dimension: "length";
  };
  target_quantity: {
    value: number;
    unit: "MM";
    target_field: string;
  };
  conversion_factor_to_target: number;
  conversion_status: "converted_to_target_unit" | "source_unit_matches_target_unit";
  decision_basis_refs: PcfReference[];
  provenance: ReturnType<typeof previewProvenance>;
};

const PCF_EXPORT_VERSION = "0.1.0";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";

export function PcfExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const basePacket = buildPcfExportPacket({ model, result, analysisRun });
  const packageHash = usePackageHash(
    basePacket.manifest.manifest_id,
    basePacket,
    "manifest_and_validation_report_package_hash_carrier_fields"
  );
  const packet = withCanonicalPackageHash(basePacket, packageHash);

  return (
    <section className="panel pcf-export-panel" aria-label="Conservative PCF export" data-testid="pcf-export-panel">
      <div className="panel-title">
        <FileText size={16} aria-hidden="true" />
        Conservative PCF Export
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="pcf-export-link"
          download={`openpipestress-preview-pcf-package-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Package JSON
        </a>
        <a
          className="report-export-link"
          data-testid="pcf-text-link"
          download={`openpipestress-preview-${safeFileToken(model.project.id)}.pcf`}
          href={textDataHref(packet.pcf_text)}
        >
          <Download size={14} aria-hidden="true" />
          PCF text
        </a>
        <span data-testid="pcf-export-summary">
          available; segments={packet.pcf_payload.pipe_segments.length}; nodes={packet.pcf_payload.nodes.length};
          validation={packet.validation_report.validation_status}; losses={packet.loss_report.length}
        </span>
      </div>
      <div className="report-list" data-testid="pcf-export-body">
        <PcfLine
          label="Profile"
          value={`${packet.export_profile.target_family}; profile=${packet.export_profile.profile_id}; target_version=${packet.export_profile.target_profile_version_basis}`}
          testId="pcf-export-profile"
        />
        <PcfLine
          label="State binding"
          value={`${packet.source_model_ref.ref}; ${packet.model_state_ref.ref}; ${packet.analysis_run_ref.ref}`}
          testId="pcf-export-state-binding"
        />
        <PcfLine
          label="Coverage"
          value={`emitted=pipe_segments; omitted=supports; approximated=components; tbd=nominal_size,target_profile`}
          testId="pcf-export-coverage"
        />
        <PcfLine
          label="Units"
          value={unitDisclosureSummary(packet.unit_system_disclosure)}
          testId="pcf-export-units"
        />
        <PcfLine
          label="Conversion witnesses"
          value={`count=${packet.conversion_witnesses.length}; scope=node.coordinates,pipe_segments.outside_diameter,pipe_segments.wall_thickness; target_length=${packet.pcf_payload.units.coordinates}`}
          testId="pcf-export-conversion-witnesses"
        />
        <PcfLine
          label="Package"
          value={`members=${packet.manifest.package_members.length}; stable_ids=${packet.stable_id_map.length}; diagnostics=${packet.diagnostics.length}; pcf_lines=${packet.manifest.pcf_artifact.line_count}; package_hash=${packet.manifest.canonical_package_hash_status}`}
          testId="pcf-export-package"
        />
        <PcfLine
          label="Boundary"
          value={`target_compatibility=${String(packet.professional_boundary.software_makes_target_compatibility_claim)}; solver_validation=${String(packet.professional_boundary.software_makes_solver_validation_claim)}; code_compliance=${String(packet.professional_boundary.software_makes_code_compliance_claim)}; professional_reliance=${String(packet.professional_boundary.software_creates_professional_reliance_record)}`}
          testId="pcf-export-boundary"
        />
      </div>
      <small className="report-note">
        PCF export is a conservative local review package only; unresolved target profile, nominal size, support,
        component, and translator-default behavior remain visible in diagnostics and loss reporting.
      </small>
    </section>
  );
}

function PcfLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildPcfExportPacket({
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
  const payload = pcfPayload(model);
  const pcfText = renderPcfText(payload);
  const conversionWitnesses = pcfConversionWitnesses(model);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: payload.units,
    conversionPolicy: "pcf_text_uses_millimeter_coordinate_and_pipe_geometry_fields_with_source_unit_disclosure",
    conversionPerformed: true,
    conversionScope: ["node.coordinates", "pipe_segments.outside_diameter", "pipe_segments.wall_thickness"],
    sourceLocation: "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx"
  });
  const stableIdMap = payload.pipe_segments.map((segment) => ({
    canonical_ref: reference("PipeSegment", segment.element_id),
    pcf_ref: reference("PcfRecord", `PIPE:${segment.target_component_identifier}`),
    metadata_carrier: "sidecar_id_map",
    mapping_status: "mapped",
    loss_category: "exported",
    provenance: previewProvenance()
  }));
  const lossReport = pcfLossReport({ model, result, analysisRunRef });
  const diagnostics = pcfDiagnostics({ model, payload });
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const validationStatus = blockingCount === 0 ? "passed" : "blocked_missing_explicit_pcf_target_fields";

  return {
    schema_version: PCF_EXPORT_VERSION,
    document_kind: "openpipestress.technical_preview.conservative_pcf_export_package",
    deliverable_id: "DEL-17-07",
    package_id: "PKG-17",
    scope_items: ["SOW-030", "SOW-074"],
    objectives: ["OBJ-009", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    export_id: `pcf-export:${safeFileToken(model.project.id)}`,
    package_status: "conservative_pcf_export_foundation",
    source_model_ref: sourceModelRef,
    source_model_hash_status: "TBD_browser_preview_does_not_emit_canonical_model_hash",
    model_state_ref: modelStateRef,
    analysis_run_ref: analysisRunRef,
    result_ref: result ? reference("MechanicsResult", result.run_id) : reference("MechanicsResult", "not generated"),
    export_profile: {
      profile_id: "ops.pcf.conservative_subset",
      profile_version: PCF_EXPORT_VERSION,
      target_family: "pcf",
      target_profile_version_basis: "TBD",
      artifact_format: "ascii_pcf_text",
      subset_scope: "invented_straight_pipe_segments_only",
      unit_policy: "explicit_units_required_no_hidden_defaults",
      coordinate_policy: "project_xyz_millimeters_only",
      identity_policy: "authoritative_sidecar_id_map",
      loss_report_policy: "mandatory_for_every_package",
      translator_default_policy: "hidden_defaults_blocked_or_loss_reported",
      support_restraint_policy: "unsupported_or_tbd_until_source_confirmed",
      source_basis_refs: [
        reference("Deliverable", "DEL-17-01"),
        reference("Deliverable", "DEL-17-02"),
        reference("Document", "CAEPIPE-PCF"),
        reference("Document", "PLAN-EXPORT-INTEROP")
      ],
      boundary_notes: [
        "PCF export is a conservative desktop preview package only.",
        "The package does not assert PCF completeness or downstream import compatibility; validation occurs in the user's accepted professional tools, and this package is screening and handoff evidence.",
        "Nominal size, first target profile/version, support/restraint preservation, hidden translator defaults, and downstream import behavior remain TBD or loss-reported.",
        "The sidecar ID map is authoritative for audit correlation."
      ]
    },
    unit_system_disclosure: unitSystemDisclosure,
    conversion_witnesses: conversionWitnesses,
    pcf_payload: payload,
    pcf_text: pcfText,
    stable_id_map: stableIdMap,
    loss_report: lossReport,
    manifest: {
      manifest_id: `pcf-manifest:${safeFileToken(model.project.id)}`,
      source_model_ref: sourceModelRef,
      source_model_hash_status: "TBD_browser_preview_does_not_emit_canonical_model_hash",
      export_profile_ref: reference("ExportProfile", "ops.pcf.conservative_subset"),
      pcf_artifact: {
        artifact_id: `pcf-export:${safeFileToken(model.project.id)}:model.pcf`,
        artifact_kind: "pcf_text",
        path: "model.pcf",
        encoding: "ascii",
        line_count: pcfText.trimEnd().split("\n").length,
        hash_status: HASH_STATUS_TBD
      },
      package_members: [
        member("manifest", "manifest.json", "manifest and package inventory", 1),
        member("model_pcf", "model.pcf", "deterministic ASCII PCF review text", pcfText.trimEnd().split("\n").length),
        member("unit_system_disclosure", "unit_system_disclosure.json", "DEC-018 source and target unit disclosure", 1),
        member(
          "conversion_witnesses",
          "conversion_witnesses.json",
          "auditable source-to-PCF millimeter conversion witnesses",
          conversionWitnesses.length
        ),
        member("stable_id_map", "id_map.json", "authoritative canonical-to-PCF sidecar", stableIdMap.length),
        member("loss_report", "loss_report.json", "unsupported, approximated, delegated, omitted, exported, and TBD behavior", lossReport.length),
        member("validation_report", "validation_report.json", "desktop preview validation and blocking diagnostics", 1),
        member("diagnostics", "diagnostics.json", "diagnostics generated from preview model fields", diagnostics.length)
      ],
      boundary_notes: [
        "browser preview package is shape-aligned with DEL-17-07 but not runtime JSON Schema validated",
        "target import behavior is not tested or claimed"
      ]
    },
    validation_report: {
      validation_status: validationStatus,
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      blocking_count: blockingCount,
      checks: [
        check("pcf_text_has_terminal_record", pcfText.endsWith("END-ISOGEN\n")),
        check("sidecar_id_map_per_pipe_segment", stableIdMap.length === payload.pipe_segments.length),
        check(
          "conversion_witness_per_converted_length_field",
          conversionWitnesses.length === expectedPcfConversionWitnessCount(model)
        ),
        check(
          "conversion_witnesses_target_millimeters",
          conversionWitnesses.every((witness) => witness.target_quantity.unit === "MM")
        ),
        check("loss_report_has_required_taxonomy", requiredLossCategoriesPresent(lossReport)),
        check("target_profile_version_remains_tbd", true),
        check("nominal_size_not_silently_invented", payload.pipe_segments.every((segment) => segment.nominal_size === "TBD_SOURCE_REQUIRED"))
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
      proprietary_target_example_included: false,
      copied_commercial_file_included: false
    },
    provenance: previewProvenance(),
    professional_boundary: {
      human_review_required: true,
      supports_export_review: true,
      supports_identity_correlation: true,
      software_makes_release_claim: false,
      software_makes_target_compatibility_claim: false,
      software_makes_solver_validation_claim: false,
      software_makes_code_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_creates_professional_reliance_record: false
    },
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    target_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false
  };
}

function pcfPayload(model: PreviewModel) {
  const materialLabelById = new Map((model.materials ?? []).map((item) => [item.id, item.label]));
  const nodes = model.nodes
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((node): PcfPayloadNode => {
      const position = toMillimeters(node.position, model.project.units.length);
      return {
        node_id: node.id,
        x: position.x,
        y: position.y,
        z: position.z
      };
    });
  const pipeSegments = model.pipe_segments
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((segment): PcfPayloadSegment => ({
      element_id: segment.id,
      from_node: segment.from,
      to_node: segment.to,
      target_component_identifier: `OPS-${safePcfToken(segment.id)}`,
      nominal_size: "TBD_SOURCE_REQUIRED",
      outside_diameter: formatQuantityAsMillimeters(segment.section.outside_diameter, "OD"),
      wall_thickness: formatQuantityAsMillimeters(segment.section.wall_thickness, "WALL"),
      item_code: `PIPE-${safePcfToken(segment.id)}`,
      material_label: safePcfToken(materialLabelById.get(segment.material) ?? segment.material)
    }));

  return {
    payload_kind: "conservative_pcf_pipe_subset",
    pipeline_reference: `OPS-${safePcfToken(model.project.id)}`,
    units: {
      bore: "MM",
      coordinates: "MM",
      bolt_diameter: "MM",
      bolt_length: "MM",
      weight: "KG"
    },
    coordinate_basis: "project_xyz_millimeters",
    nodes,
    pipe_segments: pipeSegments,
    omitted_entities: model.supports.map((support) => reference("Support", support.id)),
    unsupported_entities: [
      reference("BoundaryCondition", "free_end_and_equipment_nozzle_semantics"),
      ...model.supports.map((support) => reference("SupportRestraintSemantics", support.id))
    ],
    tbd_entities: [
      reference("TargetProfile", "pcf-target-version"),
      ...model.pipe_segments.map((segment) => reference("NominalSize", segment.id))
    ],
    provenance: previewProvenance()
  };
}

function pcfConversionWitnesses(model: PreviewModel): PcfConversionWitness[] {
  const nodeWitnesses = model.nodes
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .flatMap((node) =>
      (["x", "y", "z"] as const).map((axis) =>
        conversionWitness({
          witnessId: `pcf-conversion:${safeFileToken(node.id)}:position:${axis}`,
          sourceRef: reference("Node", node.id),
          fieldPath: `nodes.${node.id}.position.${axis}`,
          sourceValue: node.position[axis],
          sourceUnit: model.project.units.length ?? "m",
          targetField: `pcf_payload.nodes.${node.id}.${axis}`
        })
      )
    );

  const pipeSegmentWitnesses = model.pipe_segments
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .flatMap((segment) =>
      ([
        ["outside_diameter", "outside_diameter"],
        ["wall_thickness", "wall_thickness"]
      ] as const).flatMap(([field, targetField]) => {
        const quantity = segment.section[field];
        if (!quantity) return [];
        return [
          conversionWitness({
            witnessId: `pcf-conversion:${safeFileToken(segment.id)}:${field}`,
            sourceRef: reference("PipeSegment", segment.id),
            fieldPath: `pipe_segments.${segment.id}.section.${field}`,
            sourceValue: quantity.value,
            sourceUnit: quantity.unit,
            targetField: `pcf_payload.pipe_segments.${segment.id}.${targetField}`
          })
        ];
      })
    );

  return [...nodeWitnesses, ...pipeSegmentWitnesses];
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
  sourceRef: PcfReference;
  fieldPath: string;
  sourceValue: number;
  sourceUnit: string;
  targetField: string;
}): PcfConversionWitness {
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
      unit: "MM",
      target_field: targetField
    },
    conversion_factor_to_target: conversion.factor,
    conversion_status: conversion.factor === 1 ? "source_unit_matches_target_unit" : "converted_to_target_unit",
    decision_basis_refs: [reference("Decision", "DEC-018"), reference("Deliverable", "DEL-02-02")],
    provenance: previewProvenance()
  };
}

function expectedPcfConversionWitnessCount(model: PreviewModel): number {
  const nodeCoordinateFields = model.nodes.length * 3;
  const pipeSectionFields = model.pipe_segments.reduce(
    (count, segment) =>
      count + (segment.section.outside_diameter ? 1 : 0) + (segment.section.wall_thickness ? 1 : 0),
    0
  );
  return nodeCoordinateFields + pipeSectionFields;
}

function renderPcfText(payload: ReturnType<typeof pcfPayload>): string {
  const nodes = new Map(payload.nodes.map((node) => [node.node_id, node]));
  const lines = [
    "ISOGEN-FILES ISOGEN.FLS",
    `PIPELINE-REFERENCE ${payload.pipeline_reference}`,
    `UNITS-BORE ${payload.units.bore}`,
    `UNITS-CO-ORDS ${payload.units.coordinates}`,
    `UNITS-BOLT-DIA ${payload.units.bolt_diameter}`,
    `UNITS-BOLT-LENGTH ${payload.units.bolt_length}`,
    `UNITS-WEIGHT ${payload.units.weight}`,
    "STATUS FOR-REVIEW"
  ];

  for (const segment of payload.pipe_segments) {
    const start = nodes.get(segment.from_node);
    const end = nodes.get(segment.to_node);
    lines.push(
      "PIPE",
      `    COMPONENT-IDENTIFIER ${segment.target_component_identifier}`,
      `    END-POINT ${formatPoint(start)} ${segment.nominal_size}`,
      `    END-POINT ${formatPoint(end)} ${segment.nominal_size}`,
      `    ITEM-CODE ${segment.item_code}`,
      `    MATERIAL ${segment.material_label}`,
      `    PIPELINE-REFERENCE ${payload.pipeline_reference}`,
      `    OUTSIDE-DIAMETER ${segment.outside_diameter}`,
      `    WALL-THICKNESS ${segment.wall_thickness}`
    );
  }

  lines.push("END-ISOGEN");
  return `${lines.join("\n")}\n`;
}

function pcfDiagnostics({ model, payload }: { model: PreviewModel; payload: ReturnType<typeof pcfPayload> }): PcfDiagnostic[] {
  const diagnostics: PcfDiagnostic[] = [];
  if (!["m", "mm"].includes(model.project.units.length)) {
    diagnostics.push(
      diagnostic(
        "PCF-COORDINATE-UNIT-TBD",
        "blocking",
        "UNIT_WARNING",
        `Preview model length unit ${model.project.units.length} is not mapped to the PCF millimeter coordinate policy.`,
        "Select an explicit coordinate conversion policy before exporting PCF text.",
        reference("Project", model.project.id)
      )
    );
  }
  if (payload.pipe_segments.some((segment) => segment.nominal_size === "TBD_SOURCE_REQUIRED")) {
    diagnostics.push(
      diagnostic(
        "PCF-NOMINAL-SIZE-TBD",
        "blocking",
        "EXPORT_BLOCKING",
        "Preview model pipe segments do not carry source-owned nominal size records required by the conservative PCF profile.",
        "Provide source-owned nominal size data or keep the PCF output as blocked review evidence.",
        reference("ExportProfile", "ops.pcf.conservative_subset")
      )
    );
  }
  if (model.supports.length > 0) {
    diagnostics.push(
      diagnostic(
        "PCF-SUPPORT-SEMANTICS-UNSUPPORTED",
        "warning",
        "UNSUPPORTED_BEHAVIOR_WARNING",
        "Support and restraint semantics are not reliably preserved by this desktop PCF preview.",
        "Review the loss report before using the package as downstream handoff context.",
        reference("Support", model.supports[0].id)
      )
    );
  }
  if (model.components.length > 0) {
    diagnostics.push(
      diagnostic(
        "PCF-COMPONENT-COVERAGE-APPROXIMATED",
        "warning",
        "UNSUPPORTED_BEHAVIOR_WARNING",
        "Component markers are not emitted as target-specific PCF component records in this preview.",
        "Keep component behavior in the sidecar loss report until a source-confirmed profile supports it.",
        reference("Component", model.components[0].id)
      )
    );
  }
  diagnostics.push(
    diagnostic(
      "PCF-TARGET-PROFILE-TBD",
      "warning",
      "PROVENANCE_WARNING",
      "First supported PCF target profile/version remains TBD.",
      "Do not infer downstream import compatibility from this browser preview package.",
      reference("TargetProfile", "pcf-target-version")
    )
  );
  return diagnostics;
}

function pcfLossReport({
  model,
  result,
  analysisRunRef
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRunRef: ObjectRef;
}) {
  const targetArtifactRef = reference("PcfExportMember", `pcf-export:${safeFileToken(model.project.id)}:model_pcf`);
  return [
    {
      loss_id: "loss:desktop-preview:pipe-segments-exported",
      category: "exported",
      severity: "info",
      affected_refs: model.pipe_segments.map((segment) => reference("PipeSegment", segment.id)),
      target_artifact_ref: targetArtifactRef,
      reason: "Invented straight pipe segment geometry was emitted as conservative PCF review text.",
      source_basis_ref: reference("Deliverable", "DEL-17-02"),
      downstream_implication: "PCF text is export-review evidence only; no downstream import or solver behavior is implied.",
      human_review_required: true,
      provenance: previewProvenance()
    },
    {
      loss_id: "loss:desktop-preview:supports-omitted",
      category: "omitted",
      severity: "warning",
      affected_refs: model.supports.map((support) => reference("Support", support.id)),
      target_artifact_ref: targetArtifactRef,
      reason: "Support transfer is outside the conservative PCF browser preview.",
      source_basis_ref: reference("Deliverable", "DEL-17-07"),
      downstream_implication: "Support behavior remains visible only through source records and the loss report.",
      human_review_required: true,
      provenance: previewProvenance()
    },
    {
      loss_id: "loss:desktop-preview:components-approximated",
      category: "approximated",
      severity: "warning",
      affected_refs: model.components.map((component) => reference("Component", component.id)),
      target_artifact_ref: targetArtifactRef,
      reason: "Component markers are represented only by pipe geometry context and sidecar metadata in this tranche.",
      source_basis_ref: reference("Deliverable", "DEL-17-07"),
      downstream_implication: "No target-side component stiffness, weight, SIF, or flexibility behavior may be inferred.",
      human_review_required: true,
      provenance: previewProvenance()
    },
    {
      loss_id: "loss:desktop-preview:translator-defaults-delegated",
      category: "delegated",
      severity: "warning",
      affected_refs: [reference("TranslatorBehavior", "hidden-defaults")],
      target_artifact_ref: targetArtifactRef,
      reason: "Hidden downstream translator defaults are not interpreted locally.",
      source_basis_ref: reference("Document", "CAEPIPE-PCF"),
      downstream_implication: "Any downstream translator default remains user-owned and non-authoritative.",
      human_review_required: true,
      provenance: previewProvenance()
    },
    {
      loss_id: "loss:desktop-preview:boundary-conditions-unsupported",
      category: "unsupported",
      severity: "warning",
      affected_refs: [reference("BoundaryCondition", "free_end_and_equipment_nozzle_semantics")],
      target_artifact_ref: targetArtifactRef,
      reason: "Boundary-condition and equipment-connection semantics are not source-confirmed for the preview profile.",
      source_basis_ref: reference("Deliverable", "DEL-17-07"),
      downstream_implication: "The exported PCF text must not be treated as solver-ready boundary condition evidence.",
      human_review_required: true,
      provenance: previewProvenance()
    },
    {
      loss_id: "loss:desktop-preview:target-profile-and-nominal-size-tbd",
      category: "tbd",
      severity: "blocking",
      affected_refs: [
        reference("TargetProfile", "pcf-target-version"),
        ...model.pipe_segments.map((segment) => reference("NominalSize", segment.id)),
        analysisRunRef
      ],
      target_artifact_ref: targetArtifactRef,
      reason: "PCF target profile/version and source-owned nominal size records remain unresolved.",
      source_basis_ref: reference("Deliverable", "DEL-17-01"),
      downstream_implication: result
        ? "Mechanics result context exists, but PCF package remains blocked for target-compatibility use."
        : "Run-independent model export context exists, but PCF package remains blocked for target-compatibility use.",
      human_review_required: true,
      provenance: previewProvenance()
    }
  ] as const;
}

function requiredLossCategoriesPresent(lossReport: ReturnType<typeof pcfLossReport>): boolean {
  const categories = new Set(lossReport.map((entry) => entry.category));
  return (["exported", "omitted", "approximated", "delegated", "unsupported", "tbd"] as const).every((category) =>
    categories.has(category)
  );
}

function toMillimeters(point: PreviewModel["nodes"][number]["position"], unit: string) {
  return {
    x: convertLengthToMillimeters(point.x, unit).value,
    y: convertLengthToMillimeters(point.y, unit).value,
    z: convertLengthToMillimeters(point.z, unit).value
  };
}

function formatQuantityAsMillimeters(quantity: { value: number; unit: string } | undefined, fallback: string): string {
  if (!quantity) return `TBD_${fallback}`;
  return formatNumber(convertLengthToMillimeters(quantity.value, quantity.unit).value);
}

function convertLengthToMillimeters(value: number, unit: string): { value: number; factor: number } {
  const factor = unit === "m" ? 1000 : 1;
  return {
    value: Number((value * factor).toFixed(6)),
    factor
  };
}

function formatPoint(point: PcfPayloadNode | undefined): string {
  if (!point) return "0 0 0";
  return `${formatNumber(point.x)} ${formatNumber(point.y)} ${formatNumber(point.z)}`;
}

function formatNumber(value: number): string {
  if (Number.isInteger(value)) return String(value);
  return value.toFixed(6).replace(/0+$/g, "").replace(/\.$/, "");
}

function member(memberRole: string, path: string, description: string, recordCount: number) {
  return {
    member_id: `member:pcf:${memberRole}`,
    member_role: memberRole,
    path,
    description,
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
  severity: PcfDiagnostic["severity"],
  diagnosticClass: string,
  message: string,
  remediation: string,
  affectedObject: PcfReference
): PcfDiagnostic {
  return {
    code,
    class: diagnosticClass,
    severity,
    source: reference("Deliverable", "DEL-17-07"),
    affected_object: affectedObject,
    message,
    remediation,
    provenance: previewProvenance()
  };
}

function reference(objectType: string, ref: string): PcfReference {
  return { object_type: objectType, ref };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop PCF export preview",
    source_location: "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
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

function safePcfToken(value: string): string {
  return value
    .replace(/[^a-z0-9]+/gi, "-")
    .replace(/^-+|-+$/g, "")
    .toUpperCase();
}
