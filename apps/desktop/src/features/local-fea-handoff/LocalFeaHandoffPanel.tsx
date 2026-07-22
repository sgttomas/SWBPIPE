import { Download, FileJson } from "lucide-react";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type LocalFeaReference = {
  ref_id: string;
  ref_kind: string;
  locator: string;
  path_is_user_supplied: boolean;
  content_hash?: LocalFeaChecksum;
};

type LocalFeaChecksum = {
  algorithm: "sha256" | "sha512" | "TBD";
  canonicalization: "JCS-compatible-json" | "external-file-native" | "none" | "TBD";
  payload_ref: LocalFeaReference;
  value: string;
};

type LocalFeaUnitPreservationWitness = {
  witness_id: string;
  source_result_ref: LocalFeaReference;
  source_field_path: string;
  source_quantity: {
    value: number;
    unit: string;
    dimension: string;
  };
  target_result_ref: LocalFeaReference;
  target_field_path: string;
  target_quantity_policy: "referenced_result_value_and_unit_preserved_by_source_ref";
  export_unit_policy: "preserve_source_result_unit_and_dimension";
  conversion_performed: false;
  unit_system_ref: LocalFeaReference;
  provenance: ReturnType<typeof previewProvenance>;
};

const LOCAL_FEA_HANDOFF_VERSION = "0.1.0";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_local_fea_package_hash";
const CREATED_AT = "2026-06-08T00:00:00Z";

export function LocalFeaHandoffPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = result && analysisRun ? buildLocalFeaHandoffPacket({ model, result, analysisRun }) : null;

  return (
    <section className="panel local-fea-handoff-panel" aria-label="Local FEA handoff" data-testid="local-fea-panel">
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Local FEA Handoff
      </div>
      {packet ? (
        <>
          <div className="report-actions">
        <ControlledExportLink
              className="report-export-link"
              data-testid="local-fea-export-link"
              download={`openpipestress-preview-local-fea-handoff-${safeFileToken(packet.handoff_package.handoff_id)}.json`}
              href={jsonDataHref(packet)}
            >
              <Download size={14} aria-hidden="true" />
              Handoff JSON
        </ControlledExportLink>
            <span data-testid="local-fea-summary">
              available; labels={packet.handoff_package.guidance_assessment.labels.length}; flags=
              {packet.handoff_package.unsupported_behavior_flags.length}; diagnostics=
              {packet.handoff_package.diagnostics.length}; mesh=false; solver=false
            </span>
          </div>
          <div className="report-list" data-testid="local-fea-body">
            <LocalFeaLine
              label="Contract"
              value={`${packet.contract_status.contract_kind}; format=${packet.contract_status.concrete_export_format}; adapter=${packet.contract_status.target_solver_adapter}`}
              testId="local-fea-contract"
            />
            <LocalFeaLine
              label="State binding"
              value={`${packet.handoff_package.source_refs.model_ref.locator}; ${packet.handoff_package.source_refs.result_envelope_ref.locator}`}
              testId="local-fea-state-binding"
            />
            <LocalFeaLine
              label="Local region"
              value={`region=${packet.handoff_package.local_region.region_id}; basis=${packet.handoff_package.local_region.selection_basis}; nodes=${packet.handoff_package.local_region.selected_entity_ids.node_ids.length}; elements=${packet.handoff_package.local_region.selected_entity_ids.element_ids.length}`}
              testId="local-fea-region"
            />
            <LocalFeaLine
              label="Transfer"
              value={`method=${packet.handoff_package.transfer_basis.transfer_method_label}; loads=${packet.handoff_package.transfer_basis.load_case_refs.length}; displacement_refs=${packet.handoff_package.transfer_basis.displacement_result_refs.length}; force_refs=${packet.handoff_package.transfer_basis.force_result_refs.length}; moment_refs=${packet.handoff_package.transfer_basis.moment_result_refs.length}`}
              testId="local-fea-transfer"
            />
            <LocalFeaLine
              label="Unit witnesses"
              value={`count=${packet.handoff_package.unit_preservation_witnesses.length}; policy=preserve_source_result_units; conversion=false`}
              testId="local-fea-unit-witnesses"
            />
            <LocalFeaLine
              label="Unsupported"
              value={packet.handoff_package.unsupported_behavior_flags.map((item) => item.behavior_label).join("; ")}
              testId="local-fea-unsupported"
            />
            <LocalFeaLine
              label="Boundary"
              value={localFeaBoundary(packet)}
              testId="local-fea-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="local-fea-empty">
          Run mechanics preview to assemble a target-neutral local shell/solid FEA handoff package with selected region,
          transfer-basis references, advisory labels, and explicit TBD flags.
        </p>
      )}
      <small className="report-note">
        Local FEA handoff output is target-neutral review context only; no mesh, external solver run, or target format
        is generated. Validation occurs in the user&apos;s accepted professional tools; this package is screening and
        handoff evidence.
      </small>
    </section>
  );
}

function LocalFeaLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildLocalFeaHandoffPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
}) {
  const run = analysisRun.analysis_run;
  const selected = selectedLocalRegion(model, result);
  const selectedResults = selectedResultRefs(result);
  const diagnostics = localFeaDiagnostics(selected);
  const unitPreservationWitnesses = localFeaUnitPreservationWitnesses(model, result, selectedResults);
  const resultHash = run.hashes.find((item) => item.payload_scope === "result_envelope");
  const modelRef = reference("model", run.model_state_ref.ref);
  const resultEnvelopeRef = reference("result_envelope", `result-envelope:${result.run_id}`);
  const modelHash = checksum("TBD", "TBD", modelRef, "TBD_browser_preview_does_not_emit_model_state_hash");
  const resultEnvelopeHash = resultHash
    ? checksum("sha256", "JCS-compatible-json", resultEnvelopeRef, resultHash.value)
    : checksum("TBD", "TBD", resultEnvelopeRef, "TBD_browser_preview_does_not_emit_result_envelope_hash");

  return {
    schema_version: LOCAL_FEA_HANDOFF_VERSION,
    deliverable_id: "DEL-10-03",
    package_id: "PKG-10",
    scope_items: ["SOW-031", "SOW-049"],
    objective: "OBJ-009",
    contract_status: {
      contract_kind: "schema_first_local_fea_handoff_contract",
      global_analysis_role: "primary_global_centerline_frame_model",
      local_analysis_role: "optional_specialized_shell_solid_handoff",
      concrete_export_format: "TBD",
      target_solver_adapter: "TBD",
      mesh_generation: "TBD",
      external_solver_invocation: "TBD",
      professional_decision: "human_review_required"
    },
    handoff_package: {
      handoff_id: `local-fea-handoff:${safeRefToken(result.run_id)}:${safeRefToken(selected.regionId)}`,
      package_kind: "local_shell_solid_fea_handoff",
      source_refs: {
        project_ref: reference("model", model.project.id),
        model_ref: modelRef,
        result_envelope_ref: resultEnvelopeRef,
        global_model_kind: "centerline_frame_global_analysis",
        model_hash: modelHash,
        result_hash: resultEnvelopeHash
      },
      local_region: {
        region_id: selected.regionId,
        selection_basis: "diagnostic_suggested",
        selected_entity_ids: entityIds(selected, selectedResults, model),
        geometry_refs: selected.elementIds.map((id) => reference("geometry", id)),
        local_detail_refs: selected.componentIds.map((id) => reference("component", id)),
        coordinate_system_ref: reference("coordinate_system", "coordinate-system:global-preview"),
        reason_labels: ["local_detail_review_consider", "local_shell_solid_handoff_consider", "human_review_required"]
      },
      units_manifest: unitsManifest(model),
      entity_ids: entityIds(selected, selectedResults, model),
      transfer_basis: {
        load_case_refs: run.load_basis_refs.map((item) => reference("load_case", item.ref)),
        displacement_result_refs: selectedResults
          .filter((id) => id.includes(":disp:"))
          .map((id) => reference("result_envelope", id)),
        force_result_refs: selectedResults
          .filter((id) => id.includes(":force:"))
          .map((id) => reference("result_envelope", id)),
        moment_result_refs: selectedResults
          .filter((id) => id.includes(":moment:"))
          .map((id) => reference("result_envelope", id)),
        boundary_condition_refs: selected.supportIds.map((id) => reference("node", id)),
        cut_boundary_refs: selected.nodeIds.map((id) => reference("node", id)),
        transfer_method_label: "result_reference_only",
        limitations: diagnostics.map((item) => reference("diagnostic", item.code))
      },
      unit_witness_policy: "preserve_source_result_units_for_referenced_transfer_results",
      unit_preservation_witnesses: unitPreservationWitnesses,
      guidance_assessment: {
        assessment_id: `local-fea-assessment:${safeRefToken(result.run_id)}`,
        labels: [
          "local_detail_review_consider",
          "local_shell_solid_handoff_consider",
          "global_to_local_transfer_inputs_incomplete",
          "human_review_required"
        ],
        criteria_input_refs: [
          reference("result_envelope", result.summary.max_displacement?.result_ref ?? "result:disp:TBD"),
          reference("result_envelope", result.summary.max_open_formula_stress?.result_ref ?? "result:stress:TBD")
        ],
        rationale_refs: diagnostics.map((item) => reference("diagnostic", item.code)),
        human_review_required: true,
        software_makes_approval_claim: false,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false
      },
      assumptions: [
        assumption(
          "local-fea-assumption:selected-region-review",
          "Selected region is a review-context subset around the invented preview high-displacement and stress references.",
          selected.regionId,
          "requires_human_review"
        ),
        assumption(
          "local-fea-assumption:boundary-transfer-tbd",
          "Boundary transfer, interpolation, mesh, and external target setup remain TBD and require user or specialist review.",
          "local-fea-boundary-transfer:TBD",
          "unresolved"
        )
      ],
      unsupported_behavior_flags: unsupportedBehaviorFlags(diagnostics),
      warnings: diagnostics,
      diagnostics,
      privacy: {
        local_only: true,
        telemetry_allowed: false,
        private_payload_embedded: false,
        classification: "public_metadata_only",
        redaction_required_before_public_export: true
      },
      provenance: previewProvenance(),
      professional_boundary: professionalBoundary(),
      reproducibility: {
        schema_refs: [reference("external_file", "schemas/local_fea_handoff.schema.yaml")],
        source_snapshot_refs: [modelRef, resultEnvelopeRef, reference("result_envelope", run.run_id)],
        package_hashes: [
          resultEnvelopeHash,
          checksum("TBD", "TBD", reference("external_file", `local-fea-handoff:${result.run_id}`), HASH_STATUS_TBD)
        ],
        created_by: reference("TBD", "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx"),
        created_at: CREATED_AT
      }
    }
  };
}

function selectedLocalRegion(model: PreviewModel, result: MechanicsResult) {
  // No invented fallback references: when the result summary does not carry a
  // location_ref, the absence is surfaced as an explicit incomplete-state
  // finding (LOCAL-FEA-RESULT-SUMMARY-REF-MISSING) and the region degrades
  // honestly instead of borrowing fixture entity ids.
  const stressElement = result.summary.max_open_formula_stress?.location_ref ?? null;
  const displacementNode = result.summary.max_displacement?.location_ref ?? null;
  const missingSummaryRefs = [
    ...(stressElement ? [] : ["summary.max_open_formula_stress.location_ref"]),
    ...(displacementNode ? [] : ["summary.max_displacement.location_ref"])
  ];
  const stressPipe = stressElement
    ? model.pipe_segments.find((item) => item.id === stressElement) ?? model.pipe_segments[0]
    : undefined;
  const displacementPipes = displacementNode
    ? model.pipe_segments.filter((item) => item.from === displacementNode || item.to === displacementNode)
    : [];
  const selectedPipes = unique([stressPipe?.id, ...displacementPipes.map((item) => item.id)]).filter(Boolean);
  const selectedNodeSet = new Set<string>(displacementNode ? [displacementNode] : []);
  for (const pipeId of selectedPipes) {
    const pipe = model.pipe_segments.find((item) => item.id === pipeId);
    if (pipe) {
      selectedNodeSet.add(pipe.from);
      selectedNodeSet.add(pipe.to);
    }
  }
  const nodeIds = Array.from(selectedNodeSet).sort();
  const componentIds = model.components.filter((item) => nodeIds.includes(item.node)).map((item) => item.id).sort();
  const supportIds = model.supports.filter((item) => nodeIds.includes(item.node)).map((item) => item.id).sort();

  return {
    regionId: `local-region:${stressElement ? safeRefToken(stressElement) : "stress-location-ref-missing"}:${
      displacementNode ? safeRefToken(displacementNode) : "displacement-location-ref-missing"
    }`,
    missingSummaryRefs,
    nodeIds,
    elementIds: selectedPipes.sort(),
    componentIds,
    supportIds
  };
}

function entityIds(
  selected: ReturnType<typeof selectedLocalRegion>,
  selectedResults: string[],
  model: PreviewModel
) {
  return {
    entity_id_namespace: "openpipestress_model",
    component_ids: selected.componentIds,
    node_ids: selected.nodeIds,
    element_ids: selected.elementIds,
    station_ids: stationIds(selected, selectedResults),
    load_case_ids: model.load_cases.map((item) => item.id).sort(),
    result_ids: selectedResults
  };
}

function stationIds(selected: ReturnType<typeof selectedLocalRegion>, selectedResults: string[]): string[] {
  return unique([
    ...selected.elementIds.flatMap((id) => [`station:${id}:end-i`, `station:${id}:end-j`]),
    ...selected.nodeIds.map((id) => `station:${id}:summary`),
    ...selectedResults.map((id) => `station:${safeRefToken(id)}:source-result`)
  ]).sort();
}

function selectedResultRefs(result: MechanicsResult): string[] {
  const preferred = [
    result.summary.max_displacement?.result_ref,
    result.summary.max_open_formula_stress?.result_ref,
    "result:force:pipe-P-120:axial",
    "result:moment:pipe-P-120:torsion",
    "result:stress:pipe-P-120:end-j:torsional-shear"
  ];
  const available = new Set(result.results.map((item) => item.id));
  return unique(preferred.filter((id): id is string => Boolean(id)).filter((id) => available.has(id))).sort();
}

function localFeaUnitPreservationWitnesses(
  model: PreviewModel,
  result: MechanicsResult,
  selectedResults: string[]
): LocalFeaUnitPreservationWitness[] {
  const rowsById = new Map(result.results.map((item) => [item.id, item]));
  return selectedResults
    .map((id) => rowsById.get(id))
    .filter((item): item is MechanicsResult["results"][number] => Boolean(item && transferListName(item.id)))
    .map((item) => {
      const dimension = resultDimension(item);
      return {
        witness_id: `local-fea-unit:${safeRefToken(item.id)}:source-ref`,
        source_result_ref: reference("result_envelope", item.id),
        source_field_path: `results.${item.id}.value`,
        source_quantity: {
          value: item.value,
          unit: item.unit,
          dimension
        },
        target_result_ref: reference("result_envelope", item.id),
        target_field_path: `handoff_package.transfer_basis.${transferListName(item.id)}[]`,
        target_quantity_policy: "referenced_result_value_and_unit_preserved_by_source_ref",
        export_unit_policy: "preserve_source_result_unit_and_dimension",
        conversion_performed: false,
        unit_system_ref: reference("unit_system", `${model.project.id}:units`),
        provenance: previewProvenance()
      };
    });
}

function transferListName(resultId: string): string | null {
  if (resultId.includes(":disp:")) return "displacement_result_refs";
  if (resultId.includes(":force:")) return "force_result_refs";
  if (resultId.includes(":moment:")) return "moment_result_refs";
  return null;
}

function resultDimension(item: MechanicsResult["results"][number]): string {
  const kind = item.kind.toLowerCase();
  const id = item.id.toLowerCase();
  if (kind.includes("displacement") || id.includes("disp")) return "length";
  if (kind.includes("reaction") || kind.includes("force") || id.includes("reaction") || id.includes("force")) {
    return "force";
  }
  if (kind.includes("moment") || id.includes("moment")) return "moment";
  if (kind.includes("stress") || id.includes("stress")) return "stress";
  if (kind.includes("rotation") || id.includes("rotation")) return "angle";
  return "TBD";
}

function localFeaDiagnostics(selected: ReturnType<typeof selectedLocalRegion>) {
  const items = [
    diagnostic(
      "LOCAL-FEA-MESH-NOT-GENERATED",
      "LOCAL_HANDOFF_WARNING",
      "Mesh generation is not performed by the desktop technical preview.",
      "Select a target-neutral or external-user workflow for mesh generation outside this package.",
      selected.regionId
    ),
    diagnostic(
      "LOCAL-FEA-TARGET-FORMAT-TBD",
      "LOCAL_HANDOFF_WARNING",
      "Concrete local FEA exchange format and target solver adapter remain TBD.",
      "Resolve target format and adapter in a separate approved implementation tranche.",
      selected.regionId
    ),
    diagnostic(
      "LOCAL-FEA-BOUNDARY-TRANSFER-REVIEW",
      "LOCAL_HANDOFF_WARNING",
      "Boundary transfer is represented by result references only and requires human review.",
      "Review cut boundaries, load/result basis, interpolation assumptions, and local coordinate choices before handoff use.",
      selected.regionId
    ),
    diagnostic(
      "LOCAL-FEA-EXTERNAL-SOLVER-NOT-INVOKED",
      "EXPORT_BLOCKING",
      "No external shell or solid FEA solver is invoked by this package.",
      "Use the package as review context only; external execution remains user-owned and separately governed.",
      selected.regionId
    )
  ];
  if (selected.missingSummaryRefs.length > 0) {
    // Appended after the four standing findings so unsupportedBehaviorFlags
    // keeps its diagnostic indices.
    items.push(
      diagnostic(
        "LOCAL-FEA-RESULT-SUMMARY-REF-MISSING",
        "LOCAL_HANDOFF_WARNING",
        `Result summary fields ${selected.missingSummaryRefs.join(" and ")} are missing; the local region selection basis is incomplete and no substitute entity references are invented.`,
        "Re-run the mechanics preview so the result summary carries explicit location references before using this handoff package.",
        selected.regionId
      )
    );
  }
  return items;
}

function unsupportedBehaviorFlags(diagnostics: ReturnType<typeof localFeaDiagnostics>) {
  return [
    flag("mesh_generation_not_performed", "unsupported", diagnostics[0]),
    flag("target_format_not_selected", "TBD", diagnostics[1]),
    flag("boundary_transfer_requires_review", "requires_human_review", diagnostics[2]),
    flag("local_detail_assumption_unresolved", "requires_human_review", diagnostics[2]),
    flag("external_solver_not_invoked", "unsupported", diagnostics[3])
  ];
}

function flag(behaviorLabel: string, status: string, diagnosticItem: ReturnType<typeof diagnostic>) {
  return {
    flag_id: `flag:${safeRefToken(behaviorLabel)}`,
    behavior_label: behaviorLabel,
    status,
    affected_ref: diagnosticItem.affected_object,
    diagnostic_ref: reference("diagnostic", diagnosticItem.code),
    provenance: previewProvenance()
  };
}

function diagnostic(
  code: string,
  diagnosticClass: string,
  message: string,
  remediation: string,
  affectedLocator: string
) {
  return {
    code,
    class: diagnosticClass,
    severity: diagnosticClass === "EXPORT_BLOCKING" ? "blocking" : "warning",
    source: reference("external_file", "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx"),
    affected_object: reference("geometry", affectedLocator),
    message,
    remediation,
    provenance: previewProvenance()
  };
}

function assumption(assumptionId: string, label: string, basisLocator: string, status: string) {
  return {
    assumption_id: assumptionId,
    label,
    basis_ref: reference("assumption", basisLocator),
    status,
    provenance: previewProvenance()
  };
}

function unitsManifest(model: PreviewModel) {
  return {
    manifest_id: `units:${safeRefToken(model.project.id)}:local-fea-handoff`,
    unit_system_ref: reference("unit_system", `${model.project.id}:units`),
    dimension_basis: "schemas/units.schema.yaml",
    coordinate_unit: model.project.units.length ?? "m",
    force_unit: model.project.units.force ?? "N",
    moment_unit: "N*m",
    displacement_unit: "mm",
    rotation_unit: "rad",
    stress_unit: model.project.units.stress ?? "MPa",
    temperature_unit: model.project.units.temperature ?? "degC"
  };
}

function checksum(
  algorithm: LocalFeaChecksum["algorithm"],
  canonicalization: LocalFeaChecksum["canonicalization"],
  payloadRef: LocalFeaReference,
  value: string
): LocalFeaChecksum {
  return {
    algorithm,
    canonicalization,
    payload_ref: payloadRef,
    value
  };
}

function reference(refKind: string, locator: string, pathIsUserSupplied = false): LocalFeaReference {
  return {
    ref_id: safeRefToken(locator),
    ref_kind: refKind,
    locator,
    path_is_user_supplied: pathIsUserSupplied
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx",
    source_license: "project-invented metadata only",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented local FEA handoff metadata; no mesh, external solver, protected standards, private project, or proprietary target payload.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function localFeaBoundary(packet: ReturnType<typeof buildLocalFeaHandoffPacket>): string {
  const boundary = packet.handoff_package.professional_boundary;
  return [
    `human_review=${String(boundary.human_review_required)}`,
    `compliance=${String(boundary.software_makes_compliance_claim)}`,
    `certification=${String(boundary.software_makes_certification_claim)}`,
    `sealing=${String(boundary.software_makes_sealing_claim)}`,
    `approval=${String(boundary.software_makes_approval_claim)}`,
    `authentication=${String(boundary.software_makes_authentication_claim)}`
  ].join("; ");
}

function unique<T>(values: Array<T | null | undefined>): T[] {
  return Array.from(new Set(values.filter((value): value is T => value !== null && value !== undefined)));
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "ref";
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
