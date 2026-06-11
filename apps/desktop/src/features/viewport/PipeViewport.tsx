import { Box, CircleDot, CirclePlus, GitBranch } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import type { EditorOperationIntent, EntityRef, MechanicsResult, PreviewModel, Vec3 } from "../../types";

type Props = {
  model: PreviewModel;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  queuedIntents?: EditorOperationIntent[];
  result?: MechanicsResult | null;
  selection: EntityRef;
};

type ViewportCommandType = "create_node" | "connect_pipe_run" | "insert_component_symbol";

type ViewportSelectionTarget = {
  ref: EntityRef;
  label: string;
  kind: "node" | "pipe" | "support" | "component";
  screen: { x: number; y: number };
};

type NodeDraft = {
  id: string;
  label: string;
  x: string;
  y: string;
  z: string;
};

type DeformationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  boundary: string;
  nodePositions: Map<string, Vec3>;
};

export function PipeViewport({ model, onQueueIntent, onSelect, queuedIntents = [], result = null, selection }: Props) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const [localIntents, setLocalIntents] = useState<EditorOperationIntent[]>([]);
  const [nodeDraft, setNodeDraft] = useState<NodeDraft>(() => emptyNodeDraft());
  const selectionTargets = useMemo(() => viewportSelectionTargets(model), [model]);
  const deformation = useMemo(() => buildDeformationOverlay(model, result), [model, result]);
  const visibleIntents = onQueueIntent ? viewportIntents(queuedIntents) : localIntents;
  const nodeDraftValid = isNodeDraftValid(nodeDraft);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    if (!hasWebGL()) {
      host.replaceChildren();
      const fallback = document.createElement("div");
      fallback.className = "viewport-fallback";
      fallback.textContent = "3D viewport requires WebGL; model data is still loaded in the tree.";
      host.appendChild(fallback);
      return;
    }

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xf6f7f4);
    const camera = new THREE.PerspectiveCamera(42, host.clientWidth / host.clientHeight, 0.1, 1000);
    camera.position.set(7.6, 7, 8);
    camera.lookAt(3.8, 1.2, 0.7);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setSize(host.clientWidth, host.clientHeight);
    host.replaceChildren(renderer.domElement);

    scene.add(new THREE.AmbientLight(0xffffff, 0.72));
    const key = new THREE.DirectionalLight(0xffffff, 1.2);
    key.position.set(4, 9, 7);
    scene.add(key);
    scene.add(grid());

    const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
    for (const segment of model.pipe_segments) {
      const from = nodeMap.get(segment.from);
      const to = nodeMap.get(segment.to);
      if (!from || !to) continue;
      scene.add(pipeMesh(from, to, selection.id === segment.id));
    }
    if (deformation.state === "available") {
      for (const segment of model.pipe_segments) {
        const from = deformation.nodePositions.get(segment.from);
        const to = deformation.nodePositions.get(segment.to);
        if (!from || !to) continue;
        scene.add(deformedPipeMesh(from, to, selection.id === segment.id));
      }
      for (const node of model.nodes) {
        const position = deformation.nodePositions.get(node.id);
        if (!position) continue;
        scene.add(deformationMarker(position, selection.id === node.id));
      }
    }
    for (const node of model.nodes) {
      scene.add(marker(node.position, selection.id === node.id ? 0xf08c22 : 0x2f6f73, 0.095));
    }
    for (const support of model.supports) {
      const node = nodeMap.get(support.node);
      if (!node) continue;
      scene.add(supportMesh(node, selection.id === support.id));
    }
    for (const component of model.components) {
      const node = nodeMap.get(component.node);
      if (!node) continue;
      scene.add(componentMesh(node, selection.id === component.id));
    }

    const resize = () => {
      const { clientWidth, clientHeight } = host;
      camera.aspect = clientWidth / clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(clientWidth, clientHeight);
      renderer.render(scene, camera);
    };
    resize();
    window.addEventListener("resize", resize);

    let frame = 0;
    const animate = () => {
      frame = requestAnimationFrame(animate);
      scene.rotation.y = Math.sin(Date.now() / 6000) * 0.05;
      renderer.render(scene, camera);
    };
    animate();

    return () => {
      cancelAnimationFrame(frame);
      window.removeEventListener("resize", resize);
      renderer.dispose();
      host.replaceChildren();
    };
  }, [model, selection, deformation]);

  function addIntent(commandType: ViewportCommandType) {
    const intent = buildIntent(model, commandType, queuedIntents.length + localIntents.length + 1);
    queueIntent(intent);
  }

  function addExplicitNodeIntent() {
    if (!nodeDraftValid) return;
    const intent = buildExplicitNodeIntent(model, nodeDraft, queuedIntents.length + localIntents.length + 1);
    queueIntent(intent);
    setNodeDraft(emptyNodeDraft());
  }

  function queueIntent(intent: EditorOperationIntent) {
    if (onQueueIntent) {
      onQueueIntent(intent);
      return;
    }
    setLocalIntents((current) => [intent, ...current].slice(0, 4));
  }

  function updateNodeDraft(field: keyof NodeDraft, value: string) {
    setNodeDraft((current) => ({ ...current, [field]: value }));
  }

  return (
    <div className="viewport-shell">
      <div className="viewport-toolbar">
        <span>3D Centerline</span>
        <span
          className={`viewport-deformation-status ${deformation.state}`}
          aria-label="Viewport deformation overlay status"
          data-testid="viewport-deformation-status"
        >
          <strong data-testid="viewport-deformation-summary">{deformation.summary}</strong>
          <small data-testid="viewport-deformation-boundary">{deformation.boundary}</small>
        </span>
        <span>Selected: {selection.id}</span>
      </div>
      <div className="viewport-frame">
        <div className="viewport-canvas" ref={hostRef} aria-label="Three.js pipe centerline viewport" />
        <div className="viewport-selection-layer" aria-label="Viewport entity selection" data-testid="viewport-selection-layer">
          {selectionTargets.map((target) => {
            const active = selection.id === target.ref.id;
            return (
              <button
                aria-label={`Select ${target.label} in viewport`}
                aria-pressed={active}
                className={`viewport-select-target ${target.kind} ${active ? "active" : ""}`}
                data-testid={`viewport-select-${target.ref.id}`}
                key={`${target.ref.type}:${target.ref.id}`}
                onClick={() => onSelect(target.ref)}
                style={{ left: `${target.screen.x}%`, top: `${target.screen.y}%` }}
                title={`${target.label} (${target.ref.id})`}
                type="button"
              >
                <ViewportTargetIcon kind={target.kind} />
                <span>{shortEntityToken(target.ref.id)}</span>
              </button>
            );
          })}
        </div>
      </div>
      <section className="viewport-intents" aria-label="Viewport editor intents">
        <div className="viewport-intent-controls">
          <div className="viewport-node-form" aria-label="Explicit node geometry">
            <label>
              <span>Node ID</span>
              <input
                aria-label="New node ID"
                data-testid="viewport-create-node-id"
                onChange={(event) => updateNodeDraft("id", event.target.value)}
                placeholder="node:N-3"
                value={nodeDraft.id}
              />
            </label>
            <label>
              <span>Label</span>
              <input
                aria-label="New node label"
                data-testid="viewport-create-node-label"
                onChange={(event) => updateNodeDraft("label", event.target.value)}
                placeholder="Node label"
                value={nodeDraft.label}
              />
            </label>
            <label>
              <span>X</span>
              <input
                aria-label="New node X coordinate"
                data-testid="viewport-create-node-x"
                inputMode="decimal"
                onChange={(event) => updateNodeDraft("x", event.target.value)}
                placeholder="0"
                value={nodeDraft.x}
              />
            </label>
            <label>
              <span>Y</span>
              <input
                aria-label="New node Y coordinate"
                data-testid="viewport-create-node-y"
                inputMode="decimal"
                onChange={(event) => updateNodeDraft("y", event.target.value)}
                placeholder="0"
                value={nodeDraft.y}
              />
            </label>
            <label>
              <span>Z</span>
              <input
                aria-label="New node Z coordinate"
                data-testid="viewport-create-node-z"
                inputMode="decimal"
                onChange={(event) => updateNodeDraft("z", event.target.value)}
                placeholder="0"
                value={nodeDraft.z}
              />
            </label>
            <button
              data-testid="queue-explicit-node-intent"
              disabled={!nodeDraftValid}
              onClick={addExplicitNodeIntent}
              title="Queue explicit node create intent"
              type="button"
            >
              <CirclePlus size={15} aria-hidden="true" />
              Queue node
            </button>
          </div>
          <div className="viewport-intent-actions">
            <button type="button" onClick={() => addIntent("create_node")}>
              <CirclePlus size={15} aria-hidden="true" />
              Node intent
            </button>
            <button type="button" onClick={() => addIntent("connect_pipe_run")}>
              <GitBranch size={15} aria-hidden="true" />
              Pipe-run intent
            </button>
            <button type="button" onClick={() => addIntent("insert_component_symbol")}>
              <Box size={15} aria-hidden="true" />
              Component intent
            </button>
          </div>
        </div>
        <div className="viewport-intent-list" data-testid="viewport-intent-list">
          {visibleIntents.length === 0 ? (
            <p data-testid="viewport-intent-empty">
              Editor gestures create pending service-validation intents; they do not mutate persisted project data directly.
            </p>
          ) : (
            visibleIntents.map((intent) => (
              <article key={intent.queue_id ?? intent.operation_id} data-testid={`viewport-intent-${intent.change.change_kind}`}>
                <strong>{intent.change.change_kind}</strong>
                <span>pending_service_validation</span>
                <small>unit_aware_domain_validation_required</small>
                <small>does_not_mutate_persisted_project_payload</small>
                <small>{intent.queue_id ?? "not_queued"}</small>
              </article>
            ))
          )}
        </div>
      </section>
    </div>
  );
}

function emptyNodeDraft(): NodeDraft {
  return { id: "", label: "", x: "", y: "", z: "" };
}

function isNodeDraftValid(draft: NodeDraft): boolean {
  return Boolean(draft.id.trim() && draft.label.trim()) && [draft.x, draft.y, draft.z].every(isFiniteInput);
}

function isFiniteInput(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value));
}

function ViewportTargetIcon({ kind }: { kind: ViewportSelectionTarget["kind"] }) {
  if (kind === "node") return <CircleDot size={13} aria-hidden="true" />;
  if (kind === "pipe") return <GitBranch size={13} aria-hidden="true" />;
  return <Box size={13} aria-hidden="true" />;
}

function viewportSelectionTargets(model: PreviewModel): ViewportSelectionTarget[] {
  const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
  const rawTargets: Array<Omit<ViewportSelectionTarget, "screen"> & { position: Vec3; offsetY: number }> = [
    ...model.nodes.map((node) => ({
      ref: { type: "node" as const, id: node.id },
      label: node.label,
      kind: "node" as const,
      position: node.position,
      offsetY: 0
    })),
    ...model.pipe_segments.flatMap((pipe) => {
      const from = nodeMap.get(pipe.from);
      const to = nodeMap.get(pipe.to);
      if (!from || !to) return [];
      return [
        {
          ref: { type: "pipe" as const, id: pipe.id },
          label: pipe.label,
          kind: "pipe" as const,
          position: midpoint(from, to),
          offsetY: 0
        }
      ];
    }),
    ...model.supports.flatMap((support) => {
      const node = nodeMap.get(support.node);
      if (!node) return [];
      return [
        {
          ref: { type: "support" as const, id: support.id },
          label: support.label,
          kind: "support" as const,
          position: node,
          offsetY: 8
        }
      ];
    }),
    ...model.components.flatMap((component) => {
      const node = nodeMap.get(component.node);
      if (!node) return [];
      return [
        {
          ref: { type: "component" as const, id: component.id },
          label: component.label,
          kind: "component" as const,
          position: node,
          offsetY: -8
        }
      ];
    })
  ];

  const bounds = selectionBounds(rawTargets.map((target) => target.position));
  return rawTargets.map(({ position, offsetY, ...target }) => ({
    ...target,
    screen: projectToViewport(position, bounds, offsetY)
  }));
}

function midpoint(from: Vec3, to: Vec3): Vec3 {
  return {
    x: (from.x + to.x) / 2,
    y: (from.y + to.y) / 2,
    z: (from.z + to.z) / 2
  };
}

function selectionBounds(positions: Vec3[]) {
  if (!positions.length) {
    return { minX: 0, maxX: 0, minDepth: 0, maxDepth: 0 };
  }
  const depths = positions.map((position) => depthAxis(position));
  return {
    minX: Math.min(...positions.map((position) => position.x)),
    maxX: Math.max(...positions.map((position) => position.x)),
    minDepth: Math.min(...depths),
    maxDepth: Math.max(...depths)
  };
}

function projectToViewport(
  position: Vec3,
  bounds: ReturnType<typeof selectionBounds>,
  offsetY: number
): ViewportSelectionTarget["screen"] {
  return {
    x: scale(position.x, bounds.minX, bounds.maxX, 12, 88),
    y: clamp(scale(depthAxis(position), bounds.minDepth, bounds.maxDepth, 78, 20) + offsetY, 14, 86)
  };
}

function depthAxis(position: Vec3): number {
  return position.z + position.y * 0.45;
}

function scale(value: number, min: number, max: number, low: number, high: number): number {
  if (!Number.isFinite(value) || max === min) return (low + high) / 2;
  const fraction = (value - min) / (max - min);
  return low + fraction * (high - low);
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function shortEntityToken(value: string): string {
  const parts = value.split(":");
  return parts[parts.length - 1] || value;
}

function buildIntent(model: PreviewModel, commandType: ViewportCommandType, sequence: number): EditorOperationIntent {
  const operationToken = `${safeToken(commandType)}-${sequence.toString().padStart(3, "0")}`;
  const nodeRefs = model.nodes.slice(0, 2).map((node) => node.id);
  const firstComponent = model.components[0]?.id ?? "TBD";
  const target = viewportTarget(commandType, nodeRefs, firstComponent);
  const change = viewportChange(commandType, target.ref, nodeRefs, firstComponent);

  return {
    operation_id: `op:viewport-intent-${operationToken}`,
    operation_kind: viewportOperationKind(commandType),
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/viewport/PipeViewport.tsx",
      source_channel: "local_desktop_preview",
      source_role: "viewport_editor"
    },
    target,
    change,
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `${commandType} viewport gesture requires application-service validation before any durable model change.`
  };
}

function buildExplicitNodeIntent(model: PreviewModel, draft: NodeDraft, sequence: number): EditorOperationIntent {
  const nodeId = draft.id.trim();
  const label = draft.label.trim();
  const lengthUnit = model.project.units.length ?? "TBD";
  const payload = {
    id: nodeId,
    label,
    position: {
      x: Number(draft.x),
      y: Number(draft.y),
      z: Number(draft.z)
    },
    provenance: "user_entered_local_preview"
  };

  return {
    operation_id: `op:viewport-create-node-${safeToken(nodeId)}-${sequence.toString().padStart(3, "0")}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/viewport/PipeViewport.tsx",
      source_channel: "local_desktop_preview",
      source_role: "viewport_editor"
    },
    target: { object_type: "Node", ref: nodeId },
    change: {
      change_id: `change:viewport:create-node:${safeToken(nodeId)}`,
      change_kind: "create_node",
      field_label: "Explicit node geometry",
      field_path: "nodes",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: lengthUnit,
      dimension: "length",
      source_note: "explicit user-entered viewport node geometry"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: "explicit user-entered node geometry; requires service validation before durable model change."
  };
}

function viewportIntents(intents: EditorOperationIntent[]): EditorOperationIntent[] {
  return intents
    .filter((intent) => intent.source?.source_role === "viewport_editor" || intent.operation_id.startsWith("op:viewport-intent-"))
    .slice(0, 4);
}

function viewportOperationKind(commandType: ViewportCommandType): EditorOperationIntent["operation_kind"] {
  if (commandType === "create_node") return "create";
  if (commandType === "connect_pipe_run") return "connect";
  return "insert";
}

function viewportTarget(commandType: ViewportCommandType, nodeRefs: string[], firstComponent: string): EditorOperationIntent["target"] {
  if (commandType === "create_node") {
    return { object_type: "Node", ref: "node:viewport-preview-created" };
  }
  if (commandType === "connect_pipe_run") {
    return { object_type: "Element", ref: `pipe:viewport-preview:${safeToken(nodeRefs.join("-to-"))}` };
  }
  return { object_type: "Component", ref: `component:viewport-preview:${safeToken(firstComponent)}` };
}

function viewportChange(
  commandType: ViewportCommandType,
  targetRef: string,
  nodeRefs: string[],
  firstComponent: string
): EditorOperationIntent["change"] {
  const after =
    commandType === "connect_pipe_run"
      ? nodeRefs.join(" -> ")
      : commandType === "insert_component_symbol"
        ? firstComponent
        : targetRef;
  return {
    change_id: `change:viewport:${safeToken(commandType)}:${safeToken(targetRef)}`,
    change_kind: commandType,
    field_label: "Viewport command",
    field_path: `viewport.${commandType}`,
    before: "not_present",
    after,
    unit: "none",
    dimension: "dimensionless",
    source_note: "viewport editor gesture review intent; pending_service_validation"
  };
}

function pipeMesh(from: Vec3, to: Vec3, active: boolean) {
  const start = toVector(from);
  const end = toVector(to);
  const direction = end.clone().sub(start);
  const length = direction.length();
  const geometry = new THREE.CylinderGeometry(active ? 0.07 : 0.052, active ? 0.07 : 0.052, length, 18);
  const material = new THREE.MeshStandardMaterial({
    color: active ? 0xf08c22 : 0x4f6f73,
    metalness: 0.2,
    roughness: 0.58
  });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.position.copy(start.clone().add(end).multiplyScalar(0.5));
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction.normalize());
  return mesh;
}

function marker(position: Vec3, color: number, radius: number) {
  return new THREE.Mesh(
    new THREE.SphereGeometry(radius, 24, 16),
    new THREE.MeshStandardMaterial({ color, roughness: 0.48 })
  ).translateX(position.x).translateY(position.y).translateZ(position.z);
}

function supportMesh(position: Vec3, active: boolean) {
  const group = new THREE.Group();
  const cone = new THREE.Mesh(
    new THREE.ConeGeometry(0.18, 0.34, 4),
    new THREE.MeshStandardMaterial({ color: active ? 0xf08c22 : 0x6b7d49, roughness: 0.7 })
  );
  cone.position.set(position.x, position.y - 0.26, position.z);
  cone.rotation.y = Math.PI / 4;
  group.add(cone);
  return group;
}

function componentMesh(position: Vec3, active: boolean) {
  const box = new THREE.Mesh(
    new THREE.BoxGeometry(0.24, 0.24, 0.24),
    new THREE.MeshStandardMaterial({ color: active ? 0xf08c22 : 0x874c62, roughness: 0.52 })
  );
  box.position.set(position.x, position.y + 0.2, position.z);
  return box;
}

function deformedPipeMesh(from: Vec3, to: Vec3, active: boolean) {
  const start = toVector(from);
  const end = toVector(to);
  const direction = end.clone().sub(start);
  const length = direction.length();
  const geometry = new THREE.CylinderGeometry(active ? 0.045 : 0.032, active ? 0.045 : 0.032, length, 18);
  const material = new THREE.MeshStandardMaterial({
    color: active ? 0xf08c22 : 0x0f8f85,
    emissive: active ? 0x4c2500 : 0x03433f,
    metalness: 0.1,
    opacity: 0.82,
    roughness: 0.42,
    transparent: true
  });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.position.copy(start.clone().add(end).multiplyScalar(0.5));
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction.normalize());
  return mesh;
}

function deformationMarker(position: Vec3, active: boolean) {
  return new THREE.Mesh(
    new THREE.SphereGeometry(active ? 0.075 : 0.055, 18, 12),
    new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x0f8f85,
      emissive: active ? 0x4c2500 : 0x03433f,
      opacity: 0.86,
      roughness: 0.44,
      transparent: true
    })
  ).translateX(position.x).translateY(position.y).translateZ(position.z);
}

function grid() {
  const helper = new THREE.GridHelper(10, 10, 0x8b9490, 0xd6dbd4);
  helper.position.set(3.8, -0.02, 1.1);
  helper.rotation.x = Math.PI / 2;
  return helper;
}

function toVector(position: Vec3) {
  return new THREE.Vector3(position.x, position.y, position.z);
}

function buildDeformationOverlay(model: PreviewModel, result: MechanicsResult | null): DeformationOverlay {
  if (!result) {
    return {
      state: "not_started",
      summary: "not started; result rows=0",
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }
  if (result.status.mechanics !== "MECHANICS_SOLVED") {
    return {
      state: "blocked",
      summary: `blocked; mechanics=${formatStatus(result.status.mechanics)}; rows=${result.results.length}`,
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }

  const nodeIds = new Set(model.nodes.map((node) => node.id));
  const nodeValues = new Map<string, { value: number; unit: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    const current = nodeValues.get(row.entity_ref);
    if (!current || Math.abs(row.value) > Math.abs(current.value)) {
      nodeValues.set(row.entity_ref, { value: row.value, unit: row.unit });
    }
  }
  if (nodeValues.size === 0) {
    return {
      state: "unavailable",
      summary: "unavailable; displacement rows=0",
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }

  const values = Array.from(nodeValues.values());
  const maxValue = Math.max(...values.map((item) => Math.abs(item.value)));
  const units = Array.from(new Set(values.map((item) => item.unit))).sort();
  const unit = units.length === 1 ? units[0] : "mixed";
  const displayOffset = 0.65;
  const nodePositions = new Map(
    model.nodes.map((node) => {
      const value = nodeValues.get(node.id)?.value ?? 0;
      const normalizedOffset = maxValue > 0 ? (Math.abs(value) / maxValue) * displayOffset : 0;
      return [
        node.id,
        {
          x: node.position.x,
          y: node.position.y + normalizedOffset,
          z: node.position.z
        }
      ] as const;
    })
  );

  return {
    state: "available",
    summary: `available; nodes=${nodeValues.size}; max=${formatNumber(maxValue)} ${unit}`,
    boundary: `scale=normalized_display_offset_not_physical_length; vector_direction=TBD; unit_basis=${unit}; professional_claim=false`,
    nodePositions
  };
}

function formatStatus(value: string): string {
  return value.replaceAll("_", " ").toLowerCase();
}

function formatNumber(value: number): string {
  return value.toFixed(6).replace(/0+$/u, "").replace(/\.$/u, "");
}

function hasWebGL() {
  if (navigator.userAgent.toLowerCase().includes("jsdom")) {
    return false;
  }
  try {
    const canvas = document.createElement("canvas");
    return Boolean(canvas.getContext("webgl") || canvas.getContext("experimental-webgl"));
  } catch {
    return false;
  }
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
