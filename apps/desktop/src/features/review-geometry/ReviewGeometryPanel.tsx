import { Boxes, Download } from "lucide-react";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewModel, Vec3 } from "../../types";

type GltfPositionGroup = {
  name: string;
  mode: 0 | 1;
  positions: number[];
  extras: Record<string, unknown>;
};

type StableIdMapEntry = {
  stable_id: string;
  canonical_family: string;
  export_status: "emitted" | "approximated" | "metadata_only" | "omitted";
  gltf_ref: Record<string, unknown> | null;
  review_note: string;
};

export function ReviewGeometryPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildReviewGeometryPacket({ model, result, analysisRun });

  return (
    <section className="panel review-geometry-panel" aria-label="Review geometry export" data-testid="review-geometry-panel">
      <div className="panel-title">
        <Boxes size={16} aria-hidden="true" />
        Review Geometry Export
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="review-geometry-export-link"
          download={`openpipestress-preview-review-geometry-${safeFileToken(model.project.id)}.gltf.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          glTF JSON
        </a>
        <span data-testid="review-geometry-summary">
          available; format={packet.review_geometry_profile.target_artifact}; segments=
          {packet.geometry_summary.pipe_segment_count}; nodes={packet.geometry_summary.node_count}; stable_ids=
          {packet.geometry_summary.stable_id_count}
        </span>
      </div>
      <div className="report-list" data-testid="review-geometry-body">
        <ReviewGeometryLine
          label="Format"
          value={`asset=${packet.gltf_asset.asset.version}; target=${packet.review_geometry_profile.target_artifact}; units=${packet.review_geometry_profile.linear_unit}`}
          testId="review-geometry-format"
        />
        <ReviewGeometryLine
          label="State binding"
          value={`${packet.analysis_run_ref.ref}; ${packet.model_state_ref.ref}`}
          testId="review-geometry-state-binding"
        />
        <ReviewGeometryLine
          label="Coverage"
          value={`emitted=${packet.coverage.emitted_families.join(",")}; approximated=${packet.coverage.approximated_families.join(",")}; omitted=${packet.coverage.omitted_families.join(",")}`}
          testId="review-geometry-coverage"
        />
        <ReviewGeometryLine
          label="Stable IDs"
          value={`map=${packet.sidecar_id_map.entries.length}; direct_metadata=${packet.review_geometry_profile.direct_metadata_policy}; sidecar=required`}
          testId="review-geometry-stable-ids"
        />
        <ReviewGeometryLine
          label="Boundary"
          value={reviewGeometryBoundary(packet)}
          testId="review-geometry-boundary"
        />
      </div>
      <small className="report-note">
        Review geometry is visual context only; it is not solver geometry, target compatibility evidence, or professional
        validation.
      </small>
    </section>
  );
}

function ReviewGeometryLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildReviewGeometryPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const stableIdMap = stableIdEntries(model);
  const gltfGroups = buildGltfPositionGroups(model);
  const gltfAsset = buildGltfAsset({ model, groups: gltfGroups });
  const modelStateRef = run?.model_state_ref ?? reference("ModelState", "state:TBD");
  const analysisRunRef = run ? reference("AnalysisRun", run.run_id) : reference("AnalysisRun", "not generated");

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.review_geometry_export",
    deliverable_id: "DEL-17-08",
    package_id: "PKG-17",
    scope_items: ["SOW-030", "SOW-074"],
    objectives: ["OBJ-009", "OBJ-017"],
    export_scope: "local_browser_download_preview",
    review_geometry_status: "visual_review_geometry_only",
    source_basis_refs: [
      "DEL-17-01",
      "DEL-17-02",
      "GLTF-2.0",
      "fixtures/product_preview/invented_preview_model.json"
    ],
    model_ref: reference("Model", model.project.id),
    model_state_ref: modelStateRef,
    analysis_run_ref: analysisRunRef,
    result_ref: result ? reference("MechanicsResult", result.run_id) : reference("MechanicsResult", "not generated"),
    review_geometry_profile: {
      profile_id: "review-geometry:gltf-json:centerline-points:preview",
      target_family: "GLB/glTF review geometry",
      gltf_version_basis: "2.0",
      target_artifact: "glTF_2_0_json_preview",
      target_file_extension: ".gltf.json",
      glb_binary_writer_status: "TBD",
      linear_unit: model.project.units.length,
      source_coordinate_basis: "OpenPipeStress preview fixture coordinates; source vertical axis treated as +Z",
      target_coordinate_basis: "glTF right-handed +Y-up meters",
      transform_policy: "preview_z_up_to_gltf_y_up_rotation_x_minus_90",
      primitive_topology: {
        pipe_segments: "LINES",
        nodes: "POINTS",
        supports: "POINTS_approximated_marker",
        components: "POINTS_approximated_marker"
      },
      direct_metadata_policy: "name_and_extras_candidate_only",
      sidecar_id_map_policy: "required_auditable_fallback",
      extension_policy: "no_required_extensions",
      viewer_compatibility: "TBD"
    },
    geometry_summary: {
      node_count: model.nodes.length,
      pipe_segment_count: model.pipe_segments.length,
      support_marker_count: model.supports.length,
      component_marker_count: model.components.length,
      line_primitive_count: model.pipe_segments.length,
      point_marker_count: model.nodes.length + model.supports.length + model.components.length,
      stable_id_count: stableIdMap.length,
      buffer_byte_length: gltfAsset.buffers[0]?.byteLength ?? 0,
      generated_from_model_roles: ["pipe_segments", "nodes", "supports", "components"]
    },
    coverage: {
      emitted_families: ["pipe_segments", "nodes"],
      approximated_families: ["supports", "components"],
      omitted_families: ["materials", "load_cases", "combinations"],
      tbd_families: ["bends_as_curved_surfaces", "equipment_interfaces", "annotations", "load_case_visual_overlays"]
    },
    package_members: [
      {
        member_id: "member:review-geometry-gltf-json",
        filename: `openpipestress-preview-review-geometry-${safeFileToken(model.project.id)}.gltf.json`,
        document_kind: "model/gltf+json",
        hash_status: "TBD_browser_preview_does_not_emit_canonical_package_hash",
        payload_embedded_in_download: true
      },
      {
        member_id: "member:review-geometry-id-map",
        filename: "id_map.json",
        document_kind: "openpipestress.review_geometry_id_map",
        hash_status: "TBD_browser_preview_does_not_emit_canonical_package_hash",
        payload_embedded_in_download: true
      },
      {
        member_id: "member:review-geometry-loss-report",
        filename: "loss_report.json",
        document_kind: "openpipestress.review_geometry_loss_report",
        hash_status: "TBD_browser_preview_does_not_emit_canonical_package_hash",
        payload_embedded_in_download: true
      }
    ],
    gltf_asset: gltfAsset,
    sidecar_id_map: {
      schema_version: "0.1.0",
      map_id: `review-geometry-id-map:${safeFileToken(model.project.id)}`,
      canonical_id_authority: "OpenPipeStress preview model stable IDs",
      entries: stableIdMap
    },
    loss_report: reviewGeometryLossReport(model),
    diagnostics: reviewGeometryDiagnostics(model),
    unresolved_tbd: [
      "binary GLB writer and deterministic package-member hashing",
      "viewer-specific metadata stripping and selection behavior",
      "surface or tube geometry profile",
      "target-specific material, section, support, component, annotation, and load overlay encodings"
    ],
    data_boundary: {
      public_examples_policy: model.data_boundary.public_examples_policy,
      private_payload_included: false,
      protected_content_included: false,
      proprietary_target_example_included: false,
      copied_commercial_file_included: false
    },
    professional_boundary: professionalBoundary(),
    solver_geometry_equivalence_claim: false,
    professional_validation_claim: false,
    target_compatibility_claim: false,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function buildGltfPositionGroups(model: PreviewModel): GltfPositionGroup[] {
  const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
  const segmentPositions: number[] = [];
  const segmentRefs: Array<Record<string, unknown>> = [];

  for (const segment of model.pipe_segments) {
    const from = nodeMap.get(segment.from);
    const to = nodeMap.get(segment.to);
    if (!from || !to) continue;
    const vertexStart = segmentPositions.length / 3;
    segmentPositions.push(...gltfPosition(from), ...gltfPosition(to));
    segmentRefs.push({
      stable_id: segment.id,
      from: segment.from,
      to: segment.to,
      vertex_start: vertexStart,
      vertex_count: 2
    });
  }

  const groups: GltfPositionGroup[] = [
    {
      name: "OpenPipeStress pipe centerline review geometry",
      mode: 1,
      positions: segmentPositions,
      extras: {
        canonical_family: "pipe_segments",
        topology: "LINES",
        segment_refs: segmentRefs,
        review_status: "visual_centerline_only"
      }
    },
    {
      name: "OpenPipeStress node review markers",
      mode: 0,
      positions: model.nodes.flatMap((node) => gltfPosition(node.position)),
      extras: {
        canonical_family: "nodes",
        topology: "POINTS",
        node_refs: model.nodes.map((node, index) => ({ stable_id: node.id, vertex_index: index }))
      }
    },
    {
      name: "OpenPipeStress support review markers",
      mode: 0,
      positions: model.supports.flatMap((support) => gltfPosition(nodeMap.get(support.node) ?? origin())),
      extras: {
        canonical_family: "supports",
        topology: "POINTS",
        approximation: "point_marker_at_support_node",
        support_refs: model.supports.map((support, index) => ({
          stable_id: support.id,
          node_ref: support.node,
          vertex_index: index
        }))
      }
    },
    {
      name: "OpenPipeStress component review markers",
      mode: 0,
      positions: model.components.flatMap((component) => gltfPosition(nodeMap.get(component.node) ?? origin())),
      extras: {
        canonical_family: "components",
        topology: "POINTS",
        approximation: "point_marker_at_component_node",
        component_refs: model.components.map((component, index) => ({
          stable_id: component.id,
          node_ref: component.node,
          kind: component.kind,
          vertex_index: index
        }))
      }
    }
  ];
  return groups.filter((group) => group.positions.length > 0);
}

function buildGltfAsset({ model, groups }: { model: PreviewModel; groups: GltfPositionGroup[] }) {
  const allPositions = groups.flatMap((group) => group.positions);
  const floatBuffer = new Float32Array(allPositions);
  const byteBuffer = new Uint8Array(floatBuffer.buffer);
  let byteOffset = 0;
  const bufferViews = groups.map((group, index) => {
    const byteLength = group.positions.length * Float32Array.BYTES_PER_ELEMENT;
    const view = {
      buffer: 0,
      byteOffset,
      byteLength,
      target: 34962,
      name: `${group.name} bufferView`
    };
    byteOffset += byteLength;
    return view;
  });

  const accessors = groups.map((group, index) => ({
    bufferView: index,
    byteOffset: 0,
    componentType: 5126,
    count: group.positions.length / 3,
    type: "VEC3",
    min: vectorMin(group.positions),
    max: vectorMax(group.positions),
    name: `${group.name} positions`
  }));

  return {
    asset: {
      version: "2.0",
      generator: "OpenPipeStress desktop technical preview review geometry export"
    },
    scene: 0,
    scenes: [
      {
        name: "OpenPipeStress preview review geometry",
        nodes: groups.map((_group, index) => index)
      }
    ],
    nodes: groups.map((group, index) => ({
      name: group.name,
      mesh: index,
      extras: {
        openpipestress_project_ref: model.project.id,
        review_geometry_status: "visual_review_geometry_only",
        stable_id_sidecar_required: true,
        ...group.extras
      }
    })),
    meshes: groups.map((group, index) => ({
      name: group.name,
      primitives: [
        {
          mode: group.mode,
          attributes: {
            POSITION: index
          },
          extras: group.extras
        }
      ]
    })),
    buffers: [
      {
        uri: `data:application/octet-stream;base64,${bytesToBase64(byteBuffer)}`,
        byteLength: byteBuffer.byteLength
      }
    ],
    bufferViews,
    accessors,
    extras: {
      document_kind: "openpipestress.technical_preview.review_geometry_export",
      gltf_boundary: "visual_review_only_not_solver_geometry",
      source_units: model.project.units,
      transform_policy: "preview_z_up_to_gltf_y_up_rotation_x_minus_90"
    }
  };
}

function stableIdEntries(model: PreviewModel): StableIdMapEntry[] {
  const entries: StableIdMapEntry[] = [
    {
      stable_id: model.project.id,
      canonical_family: "project",
      export_status: "metadata_only",
      gltf_ref: null,
      review_note: "Project identity is carried in packet metadata, not as visual geometry."
    },
    ...(model.materials ?? []).map((material): StableIdMapEntry => ({
      stable_id: material.id,
      canonical_family: "materials",
      export_status: "omitted",
      gltf_ref: null,
      review_note: "Material properties are not emitted into the visual-review geometry preview."
    })),
    ...model.nodes.map((node, index): StableIdMapEntry => ({
      stable_id: node.id,
      canonical_family: "nodes",
      export_status: "emitted",
      gltf_ref: { node_index: 1, mesh_index: 1, primitive_index: 0, vertex_index: index },
      review_note: "Node emitted as a glTF POINTS marker."
    })),
    ...model.pipe_segments.map((segment, index): StableIdMapEntry => ({
      stable_id: segment.id,
      canonical_family: "pipe_segments",
      export_status: "emitted",
      gltf_ref: { node_index: 0, mesh_index: 0, primitive_index: 0, vertex_start: index * 2, vertex_count: 2 },
      review_note: "Pipe segment emitted as a glTF LINES centerline segment."
    })),
    ...model.supports.map((support, index): StableIdMapEntry => ({
      stable_id: support.id,
      canonical_family: "supports",
      export_status: "approximated",
      gltf_ref: { node_index: 2, mesh_index: 2, primitive_index: 0, vertex_index: index, source_node_ref: support.node },
      review_note: "Support emitted as an approximate point marker at the support node."
    })),
    ...model.components.map((component, index): StableIdMapEntry => ({
      stable_id: component.id,
      canonical_family: "components",
      export_status: "approximated",
      gltf_ref: { node_index: 3, mesh_index: 3, primitive_index: 0, vertex_index: index, source_node_ref: component.node },
      review_note: "Component emitted as an approximate point marker at the component node."
    })),
    ...model.load_cases.map((loadCase): StableIdMapEntry => ({
      stable_id: loadCase.id,
      canonical_family: "load_cases",
      export_status: "omitted",
      gltf_ref: null,
      review_note: "Load-case overlays remain TBD and are not emitted into the visual-review geometry preview."
    })),
    ...(model.combinations ?? []).map((combination): StableIdMapEntry => ({
      stable_id: combination.id,
      canonical_family: "combinations",
      export_status: "omitted",
      gltf_ref: null,
      review_note: "Load-combination overlays remain TBD and are not emitted into the visual-review geometry preview."
    }))
  ];
  return entries;
}

function reviewGeometryLossReport(model: PreviewModel) {
  return {
    schema_version: "0.1.0",
    report_id: `review-geometry-loss:${safeFileToken(model.project.id)}`,
    entries: [
      {
        loss_id: "loss:pipe-segments-centerline-only",
        category: "approximated",
        severity: "info",
        affected_refs: model.pipe_segments.map((segment) => segment.id),
        downstream_implication: "Pipe segments are centerline review geometry only; wall, bend, and surface fidelity are not represented."
      },
      {
        loss_id: "loss:supports-point-markers",
        category: "approximated",
        severity: "info",
        affected_refs: model.supports.map((support) => support.id),
        downstream_implication: "Support restraint behavior is not encoded as solver or target-tool semantics."
      },
      {
        loss_id: "loss:components-point-markers",
        category: "approximated",
        severity: "info",
        affected_refs: model.components.map((component) => component.id),
        downstream_implication: "Component symbols are location markers only; detailed component geometry remains TBD."
      },
      {
        loss_id: "loss:loads-and-combinations-omitted",
        category: "omitted",
        severity: "warning",
        affected_refs: [
          ...model.load_cases.map((loadCase) => loadCase.id),
          ...(model.combinations ?? []).map((combination) => combination.id)
        ],
        downstream_implication: "Load and combination visual overlays remain TBD and are absent from the glTF preview."
      },
      {
        loss_id: "loss:glb-and-viewer-behavior-tbd",
        category: "TBD",
        severity: "warning",
        affected_refs: [model.project.id],
        downstream_implication: "Binary GLB packaging and viewer metadata behavior require later source-grounded checks."
      }
    ]
  };
}

function reviewGeometryDiagnostics(model: PreviewModel) {
  const unitDiagnostic =
    model.project.units.length === "m"
      ? {
          code: "REVIEW-GEOMETRY-UNITS-METERED",
          severity: "info",
          message: "Preview review geometry positions are emitted in meters for the invented fixture.",
          affected_refs: [model.project.id]
        }
      : {
          code: "REVIEW-GEOMETRY-UNITS-TBD",
          severity: "blocking",
          message: "Preview review geometry requires meter-resolved linear positions before target writing.",
          affected_refs: [model.project.id]
        };
  return [
    unitDiagnostic,
    {
      code: "REVIEW-GEOMETRY-VISUAL-ONLY",
      severity: "warning",
      message: "glTF review geometry is visual context only and is not solver geometry or professional validation.",
      affected_refs: [model.project.id]
    }
  ];
}

function reviewGeometryBoundary(packet: ReturnType<typeof buildReviewGeometryPacket>): string {
  return [
    packet.review_geometry_status,
    `solver_geometry_equivalence=${String(packet.solver_geometry_equivalence_claim)}`,
    `professional_validation=${String(packet.professional_validation_claim)}`,
    `target_compatibility=${String(packet.target_compatibility_claim)}`
  ].join("; ");
}

function gltfPosition(position: Vec3): [number, number, number] {
  return [round(position.x), round(position.z), round(-position.y)];
}

function origin(): Vec3 {
  return { x: 0, y: 0, z: 0 };
}

function vectorMin(values: number[]): [number, number, number] {
  return [0, 1, 2].map((offset) => Math.min(...values.filter((_value, index) => index % 3 === offset))) as [
    number,
    number,
    number
  ];
}

function vectorMax(values: number[]): [number, number, number] {
  return [0, 1, 2].map((offset) => Math.max(...values.filter((_value, index) => index % 3 === offset))) as [
    number,
    number,
    number
  ];
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) {
    binary += String.fromCharCode(byte);
  }
  return btoa(binary);
}

function round(value: number): number {
  return Number(value.toFixed(6));
}

function reference(objectType: string, ref: string): ObjectRef {
  return {
    object_type: objectType,
    ref
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_solver_geometry_equivalence_claim: false,
    software_makes_target_compatibility_claim: false,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
