import { Box, CircleDot, CirclePlus, GitBranch } from "lucide-react";
import { type PointerEvent as ReactPointerEvent, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import {
  describeUnitBasis,
  loadUnitCatalog,
  type UnitCatalogRoute,
  type UnitCatalogEntry,
  unitDimensionValidationStatus,
  unitEntryMatchesDimension
} from "../../services/unitCatalogService";
import type {
  EditorOperationIntent,
  EntityRef,
  MechanicsResult,
  PreviewComponent,
  PreviewModel,
  Vec3
} from "../../types";
import {
  buildCreateComponentIntent,
  componentDraftForKind,
  componentDraftForNode,
  creatableComponentKinds,
  defaultComponentDraft,
  isComponentDraftValid,
  type ComponentDraft,
  type CreatableComponentKind
} from "../component-creation/componentIntent";

type Props = {
  armedCreationTool?: CreationTool | null;
  model: PreviewModel;
  onArmCreationTool?: (tool: CreationTool | null) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  queuedIntents?: EditorOperationIntent[];
  result?: MechanicsResult | null;
  selection: EntityRef;
};

export type CreationTool = "node" | "pipe" | "support" | "component" | "load";
type ViewportCommandType = "create_node" | "connect_pipe_run" | "insert_component_symbol";
type ViewPreset = "iso" | "front" | "top";
const VIEWPORT_DIMENSIONLESS_UNIT_VALIDATION_STATUS = "not_required_dimensionless";
const VIEW_TARGET = { x: 3.8, y: 1.2, z: 0.7 } as const;
const GIZMO_SIZE = 96;

type ViewportSelectionTarget = {
  ref: EntityRef;
  label: string;
  kind: "node" | "pipe" | "support" | "component";
  screen: { x: number; y: number };
};

type NodeDraft = {
  id: string;
  label: string;
  coordinateUnit: string;
  x: string;
  y: string;
  z: string;
};

type PipeDraft = {
  id: string;
  label: string;
  from: string;
  to: string;
  material: string;
  lengthUnit: string;
  outsideDiameter: string;
  wallThickness: string;
  yReferenceX: string;
  yReferenceY: string;
  yReferenceZ: string;
  provenance: string;
};

type PipeEndpointPickMode = "from" | "to" | null;

type DraftProjector = (event: { clientX: number; clientY: number }) => Vec3 | null;

type UnitOption = Pick<UnitCatalogEntry, "symbol" | "unit_id">;

function ViewportComponentTextInput({
  label,
  onChange,
  testId,
  value
}: {
  label: string;
  onChange: (value: string) => void;
  testId: string;
  value: string;
}) {
  return (
    <label>
      <span>{label}</span>
      <input
        aria-label={`New component ${label.toLowerCase()}`}
        data-testid={testId}
        onChange={(event) => onChange(event.target.value)}
        value={value}
      />
    </label>
  );
}

function ViewportComponentUnitSelect({
  label,
  onChange,
  options,
  testId,
  value
}: {
  label: string;
  onChange: (value: string) => void;
  options: UnitOption[];
  testId: string;
  value: string;
}) {
  return (
    <label>
      <span>{label}</span>
      <select
        aria-label={`New component ${label.toLowerCase()}`}
        data-testid={testId}
        onChange={(event) => onChange(event.target.value)}
        value={value}
      >
        {options.map((option) => <option key={option.symbol} value={option.symbol}>{option.symbol}</option>)}
      </select>
    </label>
  );
}

type DeformationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  boundary: string;
  nodePositions: Map<string, Vec3>;
};

export function PipeViewport({
  armedCreationTool = null,
  model,
  onArmCreationTool = () => {},
  onQueueIntent,
  onSelect,
  queuedIntents = [],
  result = null,
  selection
}: Props) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const draftProjectorRef = useRef<DraftProjector | null>(null);
  const cameraStateRef = useRef<{
    position: [number, number, number];
    target: [number, number, number];
  } | null>(null);
  const lastPresetRef = useRef<ViewPreset | null>(null);
  const pickRef = useRef<((event: { clientX: number; clientY: number }) => EntityRef | null) | null>(null);
  const selectionLayerRef = useRef<HTMLDivElement | null>(null);
  const gizmoHostRef = useRef<HTMLDivElement | null>(null);
  const defaultLengthUnit = model.project.units.length ?? "TBD";
  const [localIntents, setLocalIntents] = useState<EditorOperationIntent[]>([]);
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const [nodeDraft, setNodeDraft] = useState<NodeDraft>(() => emptyNodeDraft(defaultLengthUnit));
  const [pipeDraft, setPipeDraft] = useState<PipeDraft>(() => emptyPipeDraft(defaultLengthUnit));
  const [componentDraft, setComponentDraft] = useState<ComponentDraft>(() =>
    defaultComponentDraft(model, selection, queuedIntents)
  );
  const [pipeEndpointPickMode, setPipeEndpointPickMode] = useState<PipeEndpointPickMode>(null);
  const [viewPreset, setViewPreset] = useState<ViewPreset>("iso");
  const [showLabels, setShowLabels] = useState(true);
  const [showLoads, setShowLoads] = useState(true);
  const [showGrid, setShowGrid] = useState(true);
  const selectionTargets = useMemo(() => viewportSelectionTargets(model), [model]);
  const deformation = useMemo(() => buildDeformationOverlay(model, result), [model, result]);
  const visibleIntents = onQueueIntent ? viewportIntents(queuedIntents) : localIntents;
  const nodeDraftValid = isNodeDraftValid(nodeDraft);
  const pipeDraftValid = isPipeDraftValid(pipeDraft);
  const componentDraftValid = isComponentDraftValid(model, componentDraft, [
    ...queuedIntents,
    ...localIntents
  ]);
  const nodeUnitBasis = describeUnitBasis(unitCatalogRoute, nodeDraft.coordinateUnit, "length");
  const pipeUnitBasis = describeUnitBasis(unitCatalogRoute, pipeDraft.lengthUnit, "length");
  const nodeLengthUnitOptions = unitOptions(unitCatalogRoute, "length", nodeDraft.coordinateUnit || defaultLengthUnit);
  const pipeLengthUnitOptions = unitOptions(unitCatalogRoute, "length", pipeDraft.lengthUnit || defaultLengthUnit);
  const componentLengthUnitOptions = unitOptions(
    unitCatalogRoute,
    "length",
    componentDraft.lengthUnit || defaultLengthUnit
  );
  const componentAngleUnitOptions = unitOptions(
    unitCatalogRoute,
    "angle",
    componentDraft.angleUnit || model.project.units.angle || "rad"
  );
  const componentForceUnitOptions = unitOptions(
    unitCatalogRoute,
    "force",
    componentDraft.forceUnit || model.project.units.force || "TBD"
  );

  useEffect(() => {
    let cancelled = false;
    loadUnitCatalog()
      .then((route) => {
        if (!cancelled) setUnitCatalogRoute(route);
      })
      .catch((error: unknown) => {
        if (cancelled) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: error instanceof Error ? error.message : "UNIT-CATALOG-UNAVAILABLE"
        });
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (armedCreationTool === "pipe") {
      setPipeEndpointPickMode("from");
    } else {
      setPipeEndpointPickMode(null);
    }
  }, [armedCreationTool]);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    if (!hasWebGL()) {
      draftProjectorRef.current = (event) => fallbackDraftPointFromHostEvent(host, event, model);
      host.replaceChildren();
      const fallback = document.createElement("div");
      fallback.className = "viewport-fallback";
      fallback.textContent = "3D viewport requires WebGL; model data is still loaded in the tree.";
      host.appendChild(fallback);
      return () => {
        draftProjectorRef.current = null;
        host.replaceChildren();
      };
    }

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xf6f7f4);
    const camera = new THREE.PerspectiveCamera(42, host.clientWidth / host.clientHeight, 0.1, 1000);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setSize(host.clientWidth, host.clientHeight);
    host.replaceChildren(renderer.domElement);
    draftProjectorRef.current = (event) => raycastDraftPoint(event, renderer.domElement, camera);

    // Interactive orbit/pan/zoom. Camera state is preserved across the scene
    // rebuilds that fire on model/selection/deformation changes, so picking an
    // entity no longer snaps the view back; clicking a view-preset button (which
    // changes viewPreset) deliberately re-frames the model.
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;
    const presetChanged = lastPresetRef.current !== viewPreset;
    if (presetChanged || !cameraStateRef.current) {
      applyViewPreset(camera, viewPreset);
      controls.target.set(VIEW_TARGET.x, VIEW_TARGET.y, VIEW_TARGET.z);
    } else {
      const saved = cameraStateRef.current;
      camera.position.set(saved.position[0], saved.position[1], saved.position[2]);
      controls.target.set(saved.target[0], saved.target[1], saved.target[2]);
    }
    lastPresetRef.current = viewPreset;
    controls.update();
    const persistCameraState = () => {
      cameraStateRef.current = {
        position: [camera.position.x, camera.position.y, camera.position.z],
        target: [controls.target.x, controls.target.y, controls.target.z]
      };
    };
    controls.addEventListener("change", persistCameraState);

    scene.add(new THREE.AmbientLight(0xffffff, 0.72));
    const key = new THREE.DirectionalLight(0xffffff, 1.2);
    key.position.set(4, 9, 7);
    scene.add(key);

    const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
    if (showGrid) scene.add(referenceGround(model));

    // Pickable meshes (raycast click-to-select) and the 3D anchor positions used
    // to keep the entity labels pinned to their part as the camera orbits.
    const pickables: THREE.Object3D[] = [];
    const anchorPositions: Array<{
      id: string;
      position: THREE.Vector3;
      offsetPct: number;
    }> = [];
    const tag = (object: THREE.Object3D, ref: EntityRef, position: Vec3) => {
      object.userData.entityRef = ref;
      pickables.push(object);
      // Co-located entities (a support/component sits on its node) would stack
      // their labels at the same screen point; nudge them apart vertically.
      const offsetPct = ref.type === "support" ? 8 : ref.type === "component" ? -8 : 0;
      anchorPositions.push({
        id: ref.id,
        position: new THREE.Vector3(position.x, position.y, position.z),
        offsetPct
      });
    };

    for (const segment of model.pipe_segments) {
      const from = nodeMap.get(segment.from);
      const to = nodeMap.get(segment.to);
      if (!from || !to) continue;
      const mesh = pipeMesh(from, to, selection.id === segment.id);
      tag(mesh, { type: "pipe", id: segment.id }, midpoint(from, to));
      scene.add(mesh);
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
      const mesh = marker(node.position, selection.id === node.id ? 0xf08c22 : 0x2f6f73, 0.095);
      tag(mesh, { type: "node", id: node.id }, node.position);
      scene.add(mesh);
    }
    for (const support of model.supports) {
      const node = nodeMap.get(support.node);
      if (!node) continue;
      const mesh = supportMesh(node, selection.id === support.id);
      tag(mesh, { type: "support", id: support.id }, { x: node.x, y: node.y - 0.26, z: node.z });
      scene.add(mesh);
    }
    for (const component of model.components) {
      const node = nodeMap.get(component.node);
      if (!node) continue;
      const mesh = componentMesh(component, node, selection.id === component.id);
      tag(mesh, { type: "component", id: component.id }, { x: node.x, y: node.y + 0.2, z: node.z });
      scene.add(mesh);
    }
    if (showLoads) {
      for (const arrow of buildLoadArrows(model, nodeMap)) scene.add(arrow);
    }

    // Raycast picking: clicking a mesh selects its entity (primary selection).
    const raycaster = new THREE.Raycaster();
    pickRef.current = (event) => {
      const fraction = eventPositionFraction(renderer.domElement, event);
      const pointer = new THREE.Vector2(fraction.x * 2 - 1, -(fraction.y * 2 - 1));
      raycaster.setFromCamera(pointer, camera);
      for (const hit of raycaster.intersectObjects(pickables, true)) {
        let object: THREE.Object3D | null = hit.object;
        while (object) {
          const ref = object.userData?.entityRef as EntityRef | undefined;
          if (ref) return ref;
          object = object.parent;
        }
      }
      return null;
    };

    // Orientation gizmo: a small second scene showing the world X/Y/Z axes,
    // viewed from the same direction as the main camera so it rotates with the
    // orbit. Rendered into its own corner canvas.
    const gizmoHost = gizmoHostRef.current;
    let gizmoRenderer: THREE.WebGLRenderer | null = null;
    let gizmoScene: THREE.Scene | null = null;
    let gizmoCamera: THREE.PerspectiveCamera | null = null;
    if (gizmoHost) {
      gizmoRenderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
      gizmoRenderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      gizmoRenderer.setSize(GIZMO_SIZE, GIZMO_SIZE);
      gizmoHost.replaceChildren(gizmoRenderer.domElement);
      gizmoScene = new THREE.Scene();
      gizmoScene.add(buildOrientationGizmo());
      gizmoCamera = new THREE.PerspectiveCamera(45, 1, 0.1, 100);
    }

    const renderGizmo = () => {
      if (!gizmoRenderer || !gizmoScene || !gizmoCamera) return;
      const offset = camera.position.clone().sub(controls.target);
      if (offset.lengthSq() === 0) offset.set(0, 0, 1);
      gizmoCamera.position.copy(offset.normalize().multiplyScalar(3.2));
      gizmoCamera.up.copy(camera.up);
      gizmoCamera.lookAt(0, 0, 0);
      gizmoRenderer.render(gizmoScene, gizmoCamera);
    };

    // Keep the (toggleable) entity labels pinned to their part: project each
    // anchor to screen space every frame so the label tracks the 3D position
    // through orbit/pan/zoom instead of floating at a fixed screen spot.
    const updateLabelAnchors = () => {
      const layer = selectionLayerRef.current;
      if (!layer) return;
      for (const { id, position, offsetPct } of anchorPositions) {
        const button = layer.querySelector<HTMLElement>(`[data-testid="viewport-select-${id}"]`);
        if (!button) continue;
        const projected = position.clone().project(camera);
        const behind = projected.z > 1;
        button.style.display = behind ? "none" : "";
        if (behind) continue;
        button.style.left = `${clamp((projected.x * 0.5 + 0.5) * 100, 2, 98)}%`;
        button.style.top = `${clamp((-projected.y * 0.5 + 0.5) * 100 + offsetPct, 2, 98)}%`;
      }
    };

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
      controls.update();
      renderer.render(scene, camera);
      renderGizmo();
      updateLabelAnchors();
    };
    animate();

    return () => {
      cancelAnimationFrame(frame);
      window.removeEventListener("resize", resize);
      controls.removeEventListener("change", persistCameraState);
      controls.dispose();
      pickRef.current = null;
      draftProjectorRef.current = null;
      renderer.dispose();
      gizmoRenderer?.dispose();
      if (gizmoHost) gizmoHost.replaceChildren();
      host.replaceChildren();
    };
  }, [model, selection, deformation, viewPreset, showLoads, showGrid]);

  function addIntent(commandType: ViewportCommandType) {
    const intent = buildIntent(
      model,
      commandType,
      queuedIntents.length + localIntents.length + 1,
      unitCatalogRoute,
      defaultLengthUnit
    );
    queueIntent(intent);
  }

  function queueArmedPreviewIntent() {
    const commandType = viewportCommandTypeForCreationTool(armedCreationTool);
    if (!commandType) return;
    addIntent(commandType);
  }

  function armCreationTool(tool: CreationTool) {
    onArmCreationTool(armedCreationTool === tool ? null : tool);
  }

  function addExplicitNodeIntent() {
    if (!nodeDraftValid) return;
    const intent = buildExplicitNodeIntent(
      model,
      nodeDraft,
      unitCatalogRoute,
      queuedIntents.length + localIntents.length + 1
    );
    queueIntent(intent);
    setNodeDraft(emptyNodeDraft(nodeDraft.coordinateUnit || defaultLengthUnit));
  }

  function addExplicitPipeIntent() {
    if (!pipeDraftValid) return;
    const intent = buildExplicitPipeIntent(
      model,
      pipeDraft,
      unitCatalogRoute,
      queuedIntents.length + localIntents.length + 1
    );
    queueIntent(intent);
    setPipeDraft(emptyPipeDraft(pipeDraft.lengthUnit || defaultLengthUnit));
    setPipeEndpointPickMode(null);
  }

  function addExplicitComponentIntent() {
    if (!componentDraftValid) return;
    const intent = buildCreateComponentIntent({
      draft: componentDraft,
      sourceRef: "apps/desktop/src/features/viewport/PipeViewport.tsx",
      sourceRole: "viewport_editor",
      sequence: queuedIntents.length + localIntents.length + 1,
      unitValidation: componentUnitValidation(componentDraft, unitCatalogRoute)
    });
    queueIntent(intent);
    setComponentDraft(
      defaultComponentDraft(model, selection, [...queuedIntents, ...localIntents, intent], componentDraft.kind)
    );
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

  function updatePipeDraft(field: keyof PipeDraft, value: string) {
    setPipeDraft((current) => ({ ...current, [field]: value }));
    if (field === "from" || field === "to") {
      setPipeEndpointPickMode(null);
    }
  }

  function updateComponentDraft<K extends keyof ComponentDraft>(field: K, value: ComponentDraft[K]) {
    setComponentDraft((current) => ({ ...current, [field]: value }));
  }

  function armPipeEndpointPick(mode: Exclude<PipeEndpointPickMode, null>) {
    setPipeEndpointPickMode((current) => (current === mode ? null : mode));
  }

  function chooseViewportTarget(target: ViewportSelectionTarget) {
    if (pipeEndpointPickMode && target.kind === "node") {
      const mode = pipeEndpointPickMode;
      setPipeDraft((current) => nextPipeDraftWithEndpoint(current, mode, target.ref.id));
      setPipeEndpointPickMode(mode === "from" ? "to" : null);
    }
    if (armedCreationTool === "component" && target.kind === "node") {
      setComponentDraft((current) => componentDraftForNode(model, current, target.ref.id));
    }
    onSelect(target.ref);
  }

  function captureNodeDraftFromViewport(event: ReactPointerEvent<HTMLDivElement>) {
    if (event.button !== 0 && event.button !== undefined) return;
    const projected =
      draftProjectorRef.current?.(event) ?? fallbackDraftPointFromHostEvent(event.currentTarget, event, model);
    if (!projected) return;
    setNodeDraft((current) =>
      buildDraftNodeFromViewportPoint(
        model,
        [...queuedIntents, ...localIntents],
        projected,
        current.coordinateUnit || defaultLengthUnit
      )
    );
  }

  function handleViewportPointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (event.button !== 0 && event.button !== undefined) return;
    const picked = pickRef.current?.(event);
    if (picked) {
      if (armedCreationTool === "pipe" && pipeEndpointPickMode && picked.type === "node") {
        setPipeDraft((current) => nextPipeDraftWithEndpoint(current, pipeEndpointPickMode, picked.id));
        setPipeEndpointPickMode(pipeEndpointPickMode === "from" ? "to" : null);
      }
      if (armedCreationTool === "component" && picked.type === "node") {
        setComponentDraft((current) => componentDraftForNode(model, current, picked.id));
      }
      onSelect(picked);
      return;
    }
    if (armedCreationTool === "node") {
      captureNodeDraftFromViewport(event);
    }
  }

  const nodeToolActive = armedCreationTool === "node";
  const pipeToolActive = armedCreationTool === "pipe";
  const componentToolActive = armedCreationTool === "component";
  const viewportIntentPanelActive = nodeToolActive || pipeToolActive || componentToolActive || visibleIntents.length > 0;

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
        <div className="viewport-display-toggles" role="group" aria-label="Viewport display toggles">
          <button
            type="button"
            data-testid="toggle-viewport-labels"
            aria-pressed={showLabels}
            className={showLabels ? "active" : ""}
            onClick={() => setShowLabels((value) => !value)}
            title="Show or hide entity labels"
          >
            Labels
          </button>
          <button
            type="button"
            data-testid="toggle-viewport-loads"
            aria-pressed={showLoads}
            className={showLoads ? "active" : ""}
            onClick={() => setShowLoads((value) => !value)}
            title="Show or hide load arrows"
          >
            Loads
          </button>
          <button
            type="button"
            data-testid="toggle-viewport-grid"
            aria-pressed={showGrid}
            className={showGrid ? "active" : ""}
            onClick={() => setShowGrid((value) => !value)}
            title="Show or hide the ground grid"
          >
            Grid
          </button>
        </div>
        <span>Selected: {selection.id}</span>
      </div>
      <div className="viewport-frame">
        <div
          className="viewport-canvas"
          data-testid="viewport-canvas"
          onPointerDown={handleViewportPointerDown}
          ref={hostRef}
          aria-label="Three.js pipe centerline viewport"
          title="Click a part to select it; drag to orbit, scroll to zoom"
        />
        {showLabels ? (
          <div
            className="viewport-selection-layer"
            aria-label="Viewport entity selection"
            data-testid="viewport-selection-layer"
            ref={selectionLayerRef}
          >
            {selectionTargets.map((target) => {
              const active = selection.id === target.ref.id;
              return (
                <button
                  aria-label={`Select ${target.label} in viewport`}
                  aria-pressed={active}
                  className={`viewport-select-target ${target.kind} ${active ? "active" : ""}`}
                  data-testid={`viewport-select-${target.ref.id}`}
                  key={`${target.ref.type}:${target.ref.id}`}
                  onClick={() => chooseViewportTarget(target)}
                  style={{
                    left: `${target.screen.x}%`,
                    top: `${target.screen.y}%`
                  }}
                  title={`${target.label} (${target.ref.id})`}
                  type="button"
                >
                  <ViewportTargetIcon kind={target.kind} />
                  <span>{shortEntityToken(target.ref.id)}</span>
                </button>
              );
            })}
          </div>
        ) : null}
        <div className="viewport-axis-triad" aria-label="Orientation gizmo" data-testid="viewport-axis-triad">
          <div className="viewport-gizmo-host" ref={gizmoHostRef} aria-hidden="true" />
        </div>
        <div className="viewport-view-cube" aria-label="View controls" data-testid="viewport-view-cube">
          <button type="button" aria-pressed={viewPreset === "front"} onClick={() => setViewPreset("front")}>
            Front
          </button>
          <button type="button" aria-pressed={viewPreset === "top"} onClick={() => setViewPreset("top")}>
            Top
          </button>
          <button type="button" aria-pressed={viewPreset === "iso"} onClick={() => setViewPreset("iso")}>
            Iso
          </button>
        </div>
        <div className="viewport-scale-bar" data-testid="viewport-scale-bar">
          1 {defaultLengthUnit}
        </div>
      </div>
      <section className="command-bar" aria-label="Command and selection bar" data-testid="command-bar">
        <div className="command-buttons" aria-label="Object creation tools">
          <button
            type="button"
            className={armedCreationTool === "node" ? "active" : ""}
            data-testid="command-node"
            aria-pressed={armedCreationTool === "node"}
            onClick={() => armCreationTool("node")}
            title="Arm node creation"
          >
            <CirclePlus size={15} aria-hidden="true" />
            Node
          </button>
          <button
            type="button"
            className={armedCreationTool === "pipe" ? "active" : ""}
            data-testid="command-pipe"
            aria-pressed={armedCreationTool === "pipe"}
            onClick={() => armCreationTool("pipe")}
            title="Arm pipe-run creation"
          >
            <GitBranch size={15} aria-hidden="true" />
            Pipe
          </button>
          <button
            type="button"
            className={armedCreationTool === "support" ? "active" : ""}
            data-testid="command-support"
            aria-pressed={armedCreationTool === "support"}
            onClick={() => armCreationTool("support")}
            title="Arm support creation in the Inspector"
          >
            <CircleDot size={15} aria-hidden="true" />
            Support
          </button>
          <button
            type="button"
            className={armedCreationTool === "component" ? "active" : ""}
            data-testid="command-component"
            aria-pressed={armedCreationTool === "component"}
            onClick={() => armCreationTool("component")}
            title="Arm component-symbol insertion"
          >
            <Box size={15} aria-hidden="true" />
            Component
          </button>
          <button
            type="button"
            className={armedCreationTool === "load" ? "active" : ""}
            data-testid="command-load"
            aria-pressed={armedCreationTool === "load"}
            onClick={() => armCreationTool("load")}
            title="Arm load creation in the Load Cases panel"
          >
            <CirclePlus size={15} aria-hidden="true" />
            Load
          </button>
        </div>
        <button
          type="button"
          className="command-preview-button"
          data-testid="queue-armed-creation-intent"
          disabled={!viewportCommandTypeForCreationTool(armedCreationTool)}
          onClick={queueArmedPreviewIntent}
          title="Queue a review-only preview intent for the armed viewport tool"
        >
          Queue preview
        </button>
        <span className="command-active-tool" data-testid="armed-creation-tool">
          {creationToolStatusLabel(armedCreationTool)}
        </span>
        <span data-testid="command-selection-readout">
          Selected {selection.type}: {selection.id}; {visibleIntents.length} queued
        </span>
        <span className="command-hint" data-testid="viewport-orbit-hint">
          Drag to orbit · scroll to zoom · right-drag to pan
        </span>
      </section>
      <section
        className={`viewport-intents${viewportIntentPanelActive ? " active" : " collapsed"}`}
        aria-label="Viewport editor intents"
        data-testid="viewport-editor-intents"
      >
        <div className="viewport-intent-controls">
          <div className={`viewport-node-form${nodeToolActive ? " active" : ""}`} aria-label="Explicit node geometry">
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
            <label>
              <span>Coordinate unit</span>
              <select
                aria-label="New node coordinate unit"
                data-testid="viewport-create-node-unit"
                onChange={(event) => updateNodeDraft("coordinateUnit", event.target.value)}
                value={nodeDraft.coordinateUnit}
              >
                {nodeLengthUnitOptions.map((option) => (
                  <option key={option.symbol} value={option.symbol}>
                    {option.symbol}
                  </option>
                ))}
              </select>
            </label>
            <small data-testid="viewport-create-node-unit-basis">Coordinates: {nodeUnitBasis.label}</small>
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
          <div
            className={`viewport-pipe-form${pipeToolActive ? " active" : ""}`}
            aria-label="Explicit straight pipe connectivity"
          >
            <label>
              <span>Pipe ID</span>
              <input
                aria-label="New pipe ID"
                data-testid="viewport-create-pipe-id"
                onChange={(event) => updatePipeDraft("id", event.target.value)}
                placeholder="pipe:P-2"
                value={pipeDraft.id}
              />
            </label>
            <label>
              <span>Label</span>
              <input
                aria-label="New pipe label"
                data-testid="viewport-create-pipe-label"
                onChange={(event) => updatePipeDraft("label", event.target.value)}
                placeholder="Pipe label"
                value={pipeDraft.label}
              />
            </label>
            <div className="viewport-endpoint-field">
              <div className="viewport-field-heading">
                <span>From</span>
                <button
                  aria-pressed={pipeEndpointPickMode === "from"}
                  className="viewport-endpoint-pick"
                  data-testid="viewport-pick-pipe-from"
                  onClick={() => armPipeEndpointPick("from")}
                  title="Pick pipe from-node in viewport"
                  type="button"
                >
                  <CircleDot size={12} aria-hidden="true" />
                  Pick
                </button>
              </div>
              <select
                aria-label="New pipe from node"
                data-testid="viewport-create-pipe-from"
                onChange={(event) => updatePipeDraft("from", event.target.value)}
                value={pipeDraft.from}
              >
                <option value="">From node</option>
                {model.nodes.map((node) => (
                  <option key={node.id} value={node.id}>
                    {node.label} ({node.id})
                  </option>
                ))}
              </select>
            </div>
            <div className="viewport-endpoint-field">
              <div className="viewport-field-heading">
                <span>To</span>
                <button
                  aria-pressed={pipeEndpointPickMode === "to"}
                  className="viewport-endpoint-pick"
                  data-testid="viewport-pick-pipe-to"
                  onClick={() => armPipeEndpointPick("to")}
                  title="Pick pipe to-node in viewport"
                  type="button"
                >
                  <CircleDot size={12} aria-hidden="true" />
                  Pick
                </button>
              </div>
              <select
                aria-label="New pipe to node"
                data-testid="viewport-create-pipe-to"
                onChange={(event) => updatePipeDraft("to", event.target.value)}
                value={pipeDraft.to}
              >
                <option value="">To node</option>
                {model.nodes.map((node) => (
                  <option key={node.id} value={node.id}>
                    {node.label} ({node.id})
                  </option>
                ))}
              </select>
            </div>
            <label>
              <span>Material</span>
              <select
                aria-label="New pipe material"
                data-testid="viewport-create-pipe-material"
                onChange={(event) => updatePipeDraft("material", event.target.value)}
                value={pipeDraft.material}
              >
                <option value="">Material</option>
                {(model.materials ?? []).map((material) => (
                  <option key={material.id} value={material.id}>
                    {material.label} ({material.id})
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>OD</span>
              <input
                aria-label="New pipe outside diameter"
                data-testid="viewport-create-pipe-od"
                inputMode="decimal"
                onChange={(event) => updatePipeDraft("outsideDiameter", event.target.value)}
                placeholder="0.114"
                value={pipeDraft.outsideDiameter}
              />
            </label>
            <label>
              <span>Wall</span>
              <input
                aria-label="New pipe wall thickness"
                data-testid="viewport-create-pipe-wall"
                inputMode="decimal"
                onChange={(event) => updatePipeDraft("wallThickness", event.target.value)}
                placeholder="0.006"
                value={pipeDraft.wallThickness}
              />
            </label>
            <label>
              <span>Length unit</span>
              <select
                aria-label="New pipe length unit"
                data-testid="viewport-create-pipe-length-unit"
                onChange={(event) => updatePipeDraft("lengthUnit", event.target.value)}
                value={pipeDraft.lengthUnit}
              >
                {pipeLengthUnitOptions.map((option) => (
                  <option key={option.symbol} value={option.symbol}>
                    {option.symbol}
                  </option>
                ))}
              </select>
            </label>
            <small data-testid="viewport-create-pipe-unit-basis">Pipe geometry: {pipeUnitBasis.label}</small>
            <label>
              <span>Yref X</span>
              <input
                aria-label="New pipe y-reference X"
                data-testid="viewport-create-pipe-yref-x"
                inputMode="decimal"
                onChange={(event) => updatePipeDraft("yReferenceX", event.target.value)}
                placeholder="0"
                value={pipeDraft.yReferenceX}
              />
            </label>
            <label>
              <span>Yref Y</span>
              <input
                aria-label="New pipe y-reference Y"
                data-testid="viewport-create-pipe-yref-y"
                inputMode="decimal"
                onChange={(event) => updatePipeDraft("yReferenceY", event.target.value)}
                placeholder="0"
                value={pipeDraft.yReferenceY}
              />
            </label>
            <label>
              <span>Yref Z</span>
              <input
                aria-label="New pipe y-reference Z"
                data-testid="viewport-create-pipe-yref-z"
                inputMode="decimal"
                onChange={(event) => updatePipeDraft("yReferenceZ", event.target.value)}
                placeholder="1"
                value={pipeDraft.yReferenceZ}
              />
            </label>
            <label>
              <span>Provenance</span>
              <input
                aria-label="New pipe provenance"
                data-testid="viewport-create-pipe-provenance"
                onChange={(event) => updatePipeDraft("provenance", event.target.value)}
                value={pipeDraft.provenance}
              />
            </label>
            <button
              data-testid="queue-explicit-pipe-intent"
              disabled={!pipeDraftValid}
              onClick={addExplicitPipeIntent}
              title="Queue explicit straight-pipe connect intent"
              type="button"
            >
              <GitBranch size={15} aria-hidden="true" />
              Queue pipe
            </button>
          </div>
          <div
            className={`viewport-pipe-form${componentToolActive ? " active" : ""}`}
            aria-label="Explicit component geometry and connectivity"
            data-testid="viewport-create-component-form"
          >
            <label>
              <span>Component ID</span>
              <input
                aria-label="New component ID"
                data-testid="viewport-create-component-id"
                onChange={(event) => updateComponentDraft("id", event.target.value)}
                value={componentDraft.id}
              />
            </label>
            <label>
              <span>Label</span>
              <input
                aria-label="New component label"
                data-testid="viewport-create-component-label"
                onChange={(event) => updateComponentDraft("label", event.target.value)}
                value={componentDraft.label}
              />
            </label>
            <label>
              <span>Kind</span>
              <select
                aria-label="New component kind"
                data-testid="viewport-create-component-kind"
                value={componentDraft.kind}
                onChange={(event) =>
                  setComponentDraft((current) =>
                    componentDraftForKind(model, current, event.target.value as CreatableComponentKind)
                  )
                }
              >
                {creatableComponentKinds.map((kind) => <option key={kind} value={kind}>{kind}</option>)}
              </select>
            </label>
            <label>
              <span>Node</span>
              <select
                aria-label="New component node"
                data-testid="viewport-create-component-node"
                onChange={(event) =>
                  setComponentDraft((current) => componentDraftForNode(model, current, event.target.value))
                }
                value={componentDraft.node}
              >
                {model.nodes.map((node) => (
                  <option key={node.id} value={node.id}>{node.label} ({node.id})</option>
                ))}
              </select>
            </label>
            <label>
              <span>{componentDraft.kind === "tee" ? "Header pipe" : "Realized pipe"}</span>
              <select
                aria-label={componentDraft.kind === "tee" ? "New tee header pipe" : "New component realized pipe"}
                data-testid="viewport-create-component-pipe"
                onChange={(event) => updateComponentDraft("primaryPipeRef", event.target.value)}
                value={componentDraft.primaryPipeRef}
              >
                <option value="">Select connected pipe</option>
                {model.pipe_segments
                  .filter((pipe) => pipe.from === componentDraft.node || pipe.to === componentDraft.node)
                  .map((pipe) => <option key={pipe.id} value={pipe.id}>{pipe.label} ({pipe.id})</option>)}
              </select>
            </label>
            {componentDraft.kind === "tee" ? (
              <label>
                <span>Branch pipe</span>
                <select
                  aria-label="New tee branch pipe"
                  data-testid="viewport-create-component-secondary-pipe"
                  onChange={(event) => updateComponentDraft("secondaryPipeRef", event.target.value)}
                  value={componentDraft.secondaryPipeRef}
                >
                  <option value="">Select connected branch pipe</option>
                  {model.pipe_segments
                    .filter((pipe) => pipe.from === componentDraft.node || pipe.to === componentDraft.node)
                    .map((pipe) => <option key={pipe.id} value={pipe.id}>{pipe.label} ({pipe.id})</option>)}
                </select>
              </label>
            ) : null}
            {componentDraft.kind === "bend" ? (
              <>
                <ViewportComponentTextInput label="Radius" testId="viewport-create-component-radius" value={componentDraft.bendRadius} onChange={(value) => updateComponentDraft("bendRadius", value)} />
                <ViewportComponentUnitSelect label="Radius unit" testId="viewport-create-component-radius-unit" value={componentDraft.lengthUnit} options={componentLengthUnitOptions} onChange={(value) => updateComponentDraft("lengthUnit", value)} />
                <ViewportComponentTextInput label="Angle" testId="viewport-create-component-angle" value={componentDraft.bendAngle} onChange={(value) => updateComponentDraft("bendAngle", value)} />
                <ViewportComponentUnitSelect label="Angle unit" testId="viewport-create-component-angle-unit" value={componentDraft.angleUnit} options={componentAngleUnitOptions} onChange={(value) => updateComponentDraft("angleUnit", value)} />
                <ViewportComponentTextInput label="Plane orientation" testId="viewport-create-component-plane" value={componentDraft.bendPlaneOrientation} onChange={(value) => updateComponentDraft("bendPlaneOrientation", value)} />
              </>
            ) : null}
            {componentDraft.kind === "tee" ? (
              <>
                <ViewportComponentTextInput label="Run size" testId="viewport-create-component-run-size" value={componentDraft.branchRunSize} onChange={(value) => updateComponentDraft("branchRunSize", value)} />
                <ViewportComponentTextInput label="Header size" testId="viewport-create-component-header-size" value={componentDraft.branchHeaderSize} onChange={(value) => updateComponentDraft("branchHeaderSize", value)} />
                <ViewportComponentUnitSelect label="Size unit" testId="viewport-create-component-length-unit" value={componentDraft.lengthUnit} options={componentLengthUnitOptions} onChange={(value) => updateComponentDraft("lengthUnit", value)} />
                <ViewportComponentTextInput label="Connection angle" testId="viewport-create-component-connection-angle" value={componentDraft.branchConnectionAngle} onChange={(value) => updateComponentDraft("branchConnectionAngle", value)} />
                <ViewportComponentUnitSelect label="Angle unit" testId="viewport-create-component-angle-unit" value={componentDraft.angleUnit} options={componentAngleUnitOptions} onChange={(value) => updateComponentDraft("angleUnit", value)} />
                <ViewportComponentTextInput label="Connection type" testId="viewport-create-component-connection-type" value={componentDraft.branchConnectionType} onChange={(value) => updateComponentDraft("branchConnectionType", value)} />
                <ViewportComponentTextInput label="Reinforcement reference" testId="viewport-create-component-reinforcement" value={componentDraft.branchReinforcementReference} onChange={(value) => updateComponentDraft("branchReinforcementReference", value)} />
              </>
            ) : null}
            {componentDraft.kind !== "bend" && componentDraft.kind !== "tee" ? (
              <>
                <ViewportComponentTextInput label="Body length" testId="viewport-create-component-body-length" value={componentDraft.rigidBodyLength} onChange={(value) => updateComponentDraft("rigidBodyLength", value)} />
                <ViewportComponentTextInput label="End A size" testId="viewport-create-component-end-a-size" value={componentDraft.endASize} onChange={(value) => updateComponentDraft("endASize", value)} />
                <ViewportComponentTextInput label="End B size" testId="viewport-create-component-end-b-size" value={componentDraft.endBSize} onChange={(value) => updateComponentDraft("endBSize", value)} />
                <ViewportComponentUnitSelect label="Length unit" testId="viewport-create-component-length-unit" value={componentDraft.lengthUnit} options={componentLengthUnitOptions} onChange={(value) => updateComponentDraft("lengthUnit", value)} />
                <ViewportComponentTextInput label="Weight" testId="viewport-create-component-weight" value={componentDraft.weight} onChange={(value) => updateComponentDraft("weight", value)} />
                <ViewportComponentUnitSelect label="Weight unit" testId="viewport-create-component-force-unit" value={componentDraft.forceUnit} options={componentForceUnitOptions} onChange={(value) => updateComponentDraft("forceUnit", value)} />
                <ViewportComponentTextInput label="COG X" testId="viewport-create-component-cog-x" value={componentDraft.centerOfGravityX} onChange={(value) => updateComponentDraft("centerOfGravityX", value)} />
                <ViewportComponentTextInput label="COG Y" testId="viewport-create-component-cog-y" value={componentDraft.centerOfGravityY} onChange={(value) => updateComponentDraft("centerOfGravityY", value)} />
                <ViewportComponentTextInput label="COG Z" testId="viewport-create-component-cog-z" value={componentDraft.centerOfGravityZ} onChange={(value) => updateComponentDraft("centerOfGravityZ", value)} />
                <ViewportComponentTextInput label="End A reference" testId="viewport-create-component-end-a-ref" value={componentDraft.connectionEndAReference} onChange={(value) => updateComponentDraft("connectionEndAReference", value)} />
                <ViewportComponentTextInput label="End B reference" testId="viewport-create-component-end-b-ref" value={componentDraft.connectionEndBReference} onChange={(value) => updateComponentDraft("connectionEndBReference", value)} />
                <ViewportComponentTextInput label="Stiffness behavior reference" testId="viewport-create-component-stiffness-ref" value={componentDraft.stiffnessBehaviorReference} onChange={(value) => updateComponentDraft("stiffnessBehaviorReference", value)} />
              </>
            ) : null}
            <label>
              <span>Geometry source</span>
              <input
                aria-label="New component geometry source"
                data-testid="viewport-create-component-source"
                onChange={(event) => updateComponentDraft("geometrySourceReference", event.target.value)}
                value={componentDraft.geometrySourceReference}
              />
            </label>
            <button
              data-testid="queue-explicit-component-intent"
              disabled={!componentDraftValid}
              onClick={addExplicitComponentIntent}
              title={`Queue explicit ${componentDraft.kind} creation intent`}
              type="button"
            >
              <Box size={15} aria-hidden="true" />
              Queue {componentDraft.kind}
            </button>
          </div>
          <small data-testid="viewport-unit-catalog-status">
            {unitCatalogRoute?.route === "tauri_unit_catalog"
              ? `DEC-018 unit catalog loaded; entries=${unitCatalogRoute.catalog.entry_count}`
              : "browser preview uses model metadata for viewport length units"}
          </small>
        </div>
        <div className="viewport-intent-list" data-testid="viewport-intent-list">
          {visibleIntents.length === 0 ? (
            <p data-testid="viewport-intent-empty">
              Editor gestures create pending service-validation intents; they do not mutate persisted project data
              directly.
            </p>
          ) : (
            visibleIntents.map((intent) => (
              <article
                key={intent.queue_id ?? intent.operation_id}
                data-testid={`viewport-intent-${intent.change.change_kind}`}
              >
                <strong>{intent.change.change_kind}</strong>
                <span>pending_service_validation</span>
                <small data-testid={`viewport-intent-unit-validation-${intent.change.change_kind}`}>
                  unit_validation={intent.validation.unit_validation}
                </small>
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

function emptyNodeDraft(lengthUnit: string): NodeDraft {
  return { id: "", label: "", coordinateUnit: lengthUnit, x: "", y: "", z: "" };
}

function viewportCommandTypeForCreationTool(tool: CreationTool | null): ViewportCommandType | null {
  if (tool === "node") return "create_node";
  if (tool === "pipe") return "connect_pipe_run";
  if (tool === "component") return "insert_component_symbol";
  return null;
}

function creationToolStatusLabel(tool: CreationTool | null): string {
  if (tool === "node") return "Node tool armed: click empty canvas to fill coordinates, then queue node.";
  if (tool === "pipe") return "Pipe tool armed: pick from/to nodes or complete the pipe form.";
  if (tool === "support") return "Support tool armed: select a node, then complete Create support in the Inspector.";
  if (tool === "component") return "Component tool armed: queue a review-only component-symbol preview intent.";
  if (tool === "load") return "Load tool armed: use the Load Cases panel to create load cases and primitive loads.";
  return "Model focus";
}

function applyViewPreset(camera: THREE.PerspectiveCamera, preset: ViewPreset) {
  if (preset === "front") {
    camera.position.set(3.8, 1.2, 11);
  } else if (preset === "top") {
    camera.position.set(3.8, 12, 0.7);
  } else {
    camera.position.set(7.6, 7, 8);
  }
  camera.lookAt(3.8, 1.2, 0.7);
  camera.updateProjectionMatrix();
}

function emptyPipeDraft(lengthUnit: string): PipeDraft {
  return {
    id: "",
    label: "",
    from: "",
    to: "",
    material: "",
    lengthUnit,
    outsideDiameter: "",
    wallThickness: "",
    yReferenceX: "",
    yReferenceY: "",
    yReferenceZ: "",
    provenance: "user_entered_local_preview"
  };
}

function nextPipeDraftWithEndpoint(
  current: PipeDraft,
  mode: Exclude<PipeEndpointPickMode, null>,
  nodeId: string
): PipeDraft {
  if (mode === "from") {
    return {
      ...current,
      from: nodeId,
      to: current.to === nodeId ? "" : current.to
    };
  }
  return {
    ...current,
    to: nodeId,
    from: current.from === nodeId ? "" : current.from
  };
}

function isNodeDraftValid(draft: NodeDraft): boolean {
  return (
    Boolean(draft.id.trim() && draft.label.trim() && validUnitSymbol(draft.coordinateUnit)) &&
    [draft.x, draft.y, draft.z].every(isFiniteInput)
  );
}

function isPipeDraftValid(draft: PipeDraft): boolean {
  return (
    Boolean(
      draft.id.trim() &&
      draft.label.trim() &&
      draft.from.trim() &&
      draft.to.trim() &&
      draft.from !== draft.to &&
      draft.material.trim() &&
      validUnitSymbol(draft.lengthUnit) &&
      draft.provenance.trim()
    ) &&
    [draft.outsideDiameter, draft.wallThickness].every(isPositiveInput) &&
    [draft.yReferenceX, draft.yReferenceY, draft.yReferenceZ].every(isFiniteInput) &&
    [draft.yReferenceX, draft.yReferenceY, draft.yReferenceZ].some((value) => Number(value) !== 0)
  );
}

function isFiniteInput(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value));
}

function isPositiveInput(value: string): boolean {
  return isFiniteInput(value) && Number(value) > 0;
}

function validUnitSymbol(value: string): boolean {
  const unit = value.trim();
  return Boolean(unit && unit !== "TBD");
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, fallbackSymbol: string): UnitOption[] {
  const fallback = {
    symbol: fallbackSymbol.trim() || "TBD",
    unit_id: "current"
  };
  if (route?.route !== "tauri_unit_catalog") return [fallback];
  const options = route.catalog.entries
    .filter((entry) => entry.review_status === "accepted")
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .map((entry) => ({ symbol: entry.symbol, unit_id: entry.unit_id }));
  if (!options.some((option) => option.symbol === fallback.symbol)) options.unshift(fallback);
  return options;
}

function componentUnitValidation(draft: ComponentDraft, route: UnitCatalogRoute | null): string {
  const length = unitDimensionValidationStatus(route, draft.lengthUnit, "length");
  if (draft.kind === "bend" || draft.kind === "tee") {
    const angle = unitDimensionValidationStatus(route, draft.angleUnit, "angle");
    return `length=${length}; angle=${angle}`;
  }
  const force = unitDimensionValidationStatus(route, draft.forceUnit, "force");
  return `length=${length}; force=${force}`;
}

function buildDraftNodeFromViewportPoint(
  model: PreviewModel,
  queuedIntents: EditorOperationIntent[],
  point: Vec3,
  coordinateUnit: string
): NodeDraft {
  const id = nextViewportNodeId(model, queuedIntents);
  return {
    id,
    label: `Viewport node ${shortEntityToken(id)}`,
    coordinateUnit,
    x: formatDraftCoordinate(point.x),
    y: formatDraftCoordinate(point.y),
    z: formatDraftCoordinate(point.z)
  };
}

function nextViewportNodeId(model: PreviewModel, queuedIntents: EditorOperationIntent[]): string {
  const reserved = new Set(model.nodes.map((node) => node.id));
  for (const intent of queuedIntents) {
    if (intent.change.change_kind === "create_node") {
      reserved.add(intent.target.ref);
    }
  }
  for (let index = 1; index < 100000; index += 1) {
    const candidate = `node:V-${index.toString().padStart(3, "0")}`;
    if (!reserved.has(candidate)) return candidate;
  }
  return "node:V-TBD";
}

function formatDraftCoordinate(value: number): string {
  const rounded = Math.round(value * 1000) / 1000;
  return String(Object.is(rounded, -0) ? 0 : rounded);
}

function raycastDraftPoint(
  event: { clientX: number; clientY: number },
  canvas: HTMLCanvasElement,
  camera: THREE.PerspectiveCamera
): Vec3 | null {
  const position = eventPositionFraction(canvas, event);
  const pointer = new THREE.Vector2(position.x * 2 - 1, -(position.y * 2 - 1));
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(pointer, camera);
  const plane = new THREE.Plane(new THREE.Vector3(0, 1, 0), 0);
  const intersection = new THREE.Vector3();
  if (!raycaster.ray.intersectPlane(plane, intersection)) return null;
  if (![intersection.x, intersection.y, intersection.z].every(Number.isFinite)) return null;
  return { x: intersection.x, y: intersection.y, z: intersection.z };
}

function fallbackDraftPointFromHostEvent(
  host: HTMLElement,
  event: { clientX: number; clientY: number },
  model: PreviewModel
): Vec3 {
  const position = eventPositionFraction(host, event);
  const bounds = selectionBounds(model.nodes.map((node) => node.position));
  const xPercent = clamp(position.x * 100, 12, 88);
  const depthPercent = clamp(position.y * 100, 20, 78);
  return {
    x: unscale(xPercent, bounds.minX, bounds.maxX, 12, 88),
    y: 0,
    z: unscale(depthPercent, bounds.minDepth, bounds.maxDepth, 78, 20)
  };
}

function eventPositionFraction(
  element: HTMLElement,
  event: { clientX: number; clientY: number }
): { x: number; y: number } {
  const rect = element.getBoundingClientRect();
  const width = rect.width || element.clientWidth || 600;
  const height = rect.height || element.clientHeight || 320;
  const left = rect.width ? rect.left : 0;
  const top = rect.height ? rect.top : 0;
  return {
    x: clamp((event.clientX - left) / width, 0, 1),
    y: clamp((event.clientY - top) / height, 0, 1)
  };
}

function ViewportTargetIcon({ kind }: { kind: ViewportSelectionTarget["kind"] }) {
  if (kind === "node") return <CircleDot size={13} aria-hidden="true" />;
  if (kind === "pipe") return <GitBranch size={13} aria-hidden="true" />;
  return <Box size={13} aria-hidden="true" />;
}

function viewportSelectionTargets(model: PreviewModel): ViewportSelectionTarget[] {
  const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
  const rawTargets: Array<
    Omit<ViewportSelectionTarget, "screen"> & {
      position: Vec3;
      offsetY: number;
    }
  > = [
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

function unscale(value: number, min: number, max: number, low: number, high: number): number {
  if (!Number.isFinite(value) || max === min || low === high) return (min + max) / 2;
  const fraction = (value - low) / (high - low);
  return min + fraction * (max - min);
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function shortEntityToken(value: string): string {
  const parts = value.split(":");
  return parts[parts.length - 1] || value;
}

function buildIntent(
  model: PreviewModel,
  commandType: ViewportCommandType,
  sequence: number,
  unitCatalogRoute: UnitCatalogRoute | null,
  defaultLengthUnit: string
): EditorOperationIntent {
  const operationToken = `${safeToken(commandType)}-${sequence.toString().padStart(3, "0")}`;
  const nodeRefs = model.nodes.slice(0, 2).map((node) => node.id);
  const firstComponent = model.components[0]?.id ?? "TBD";
  const lengthUnit = defaultLengthUnit.trim() || "TBD";
  const target = viewportTarget(commandType, nodeRefs, firstComponent);
  const change = viewportChange(commandType, target.ref, nodeRefs, firstComponent, lengthUnit);

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
      unit_validation: viewportCommandUnitValidationStatus(commandType, unitCatalogRoute, lengthUnit),
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

function buildExplicitNodeIntent(
  _model: PreviewModel,
  draft: NodeDraft,
  unitCatalogRoute: UnitCatalogRoute | null,
  sequence: number
): EditorOperationIntent {
  const nodeId = draft.id.trim();
  const label = draft.label.trim();
  const lengthUnit = draft.coordinateUnit.trim() || "TBD";
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
      unit_validation: `length=${unitDimensionValidationStatus(unitCatalogRoute, lengthUnit, "length")}`,
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

function buildExplicitPipeIntent(
  _model: PreviewModel,
  draft: PipeDraft,
  unitCatalogRoute: UnitCatalogRoute | null,
  sequence: number
): EditorOperationIntent {
  const pipeId = draft.id.trim();
  const label = draft.label.trim();
  const lengthUnit = draft.lengthUnit.trim() || "TBD";
  const payload = {
    id: pipeId,
    label,
    from: draft.from.trim(),
    to: draft.to.trim(),
    section: {
      outside_diameter: {
        value: Number(draft.outsideDiameter),
        unit: lengthUnit
      },
      wall_thickness: { value: Number(draft.wallThickness), unit: lengthUnit }
    },
    material: draft.material.trim(),
    y_reference: {
      x: Number(draft.yReferenceX),
      y: Number(draft.yReferenceY),
      z: Number(draft.yReferenceZ)
    },
    provenance: draft.provenance.trim()
  };

  return {
    operation_id: `op:viewport-connect-pipe-${safeToken(pipeId)}-${sequence.toString().padStart(3, "0")}`,
    operation_kind: "connect",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/viewport/PipeViewport.tsx",
      source_channel: "local_desktop_preview",
      source_role: "viewport_editor"
    },
    target: { object_type: "Element", ref: pipeId },
    change: {
      change_id: `change:viewport:connect-pipe:${safeToken(pipeId)}`,
      change_kind: "connect_pipe_run",
      field_label: "Explicit straight pipe connectivity",
      field_path: "pipe_segments",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: lengthUnit,
      dimension: "length",
      source_note: "explicit user-entered straight-pipe connectivity and section geometry"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: `length=${unitDimensionValidationStatus(unitCatalogRoute, lengthUnit, "length")}`,
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
    rationale:
      "explicit user-entered straight-pipe connectivity and section geometry; requires service validation before durable model change."
  };
}

function viewportIntents(intents: EditorOperationIntent[]): EditorOperationIntent[] {
  return intents
    .filter(
      (intent) =>
        intent.source?.source_role === "viewport_editor" || intent.operation_id.startsWith("op:viewport-intent-")
    )
    .slice(0, 4);
}

function viewportOperationKind(commandType: ViewportCommandType): EditorOperationIntent["operation_kind"] {
  if (commandType === "create_node") return "create";
  if (commandType === "connect_pipe_run") return "connect";
  return "insert";
}

function viewportCommandUnitValidationStatus(
  commandType: ViewportCommandType,
  unitCatalogRoute: UnitCatalogRoute | null,
  lengthUnit: string
): string {
  if (commandType === "insert_component_symbol") return VIEWPORT_DIMENSIONLESS_UNIT_VALIDATION_STATUS;
  return `length=${unitDimensionValidationStatus(unitCatalogRoute, lengthUnit, "length")}`;
}

function viewportTarget(
  commandType: ViewportCommandType,
  nodeRefs: string[],
  firstComponent: string
): EditorOperationIntent["target"] {
  if (commandType === "create_node") {
    return { object_type: "Node", ref: "node:viewport-preview-created" };
  }
  if (commandType === "connect_pipe_run") {
    return {
      object_type: "Element",
      ref: `pipe:viewport-preview:${safeToken(nodeRefs.join("-to-"))}`
    };
  }
  return {
    object_type: "Component",
    ref: `component:viewport-preview:${safeToken(firstComponent)}`
  };
}

function viewportChange(
  commandType: ViewportCommandType,
  targetRef: string,
  nodeRefs: string[],
  firstComponent: string,
  lengthUnit: string
): EditorOperationIntent["change"] {
  const isLengthBearing = commandType !== "insert_component_symbol";
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
    unit: isLengthBearing ? lengthUnit : "none",
    dimension: isLengthBearing ? "length" : "dimensionless",
    source_note: isLengthBearing
      ? "viewport editor gesture review intent; length unit must be service-validated; pending_service_validation"
      : "viewport editor gesture review intent; pending_service_validation"
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
  )
    .translateX(position.x)
    .translateY(position.y)
    .translateZ(position.z);
}

function supportMesh(position: Vec3, active: boolean) {
  const group = new THREE.Group();
  const cone = new THREE.Mesh(
    new THREE.ConeGeometry(0.18, 0.34, 4),
    new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x6b7d49,
      roughness: 0.7
    })
  );
  cone.position.set(position.x, position.y - 0.26, position.z);
  cone.rotation.y = Math.PI / 4;
  group.add(cone);
  return group;
}

function componentMesh(component: PreviewComponent, position: Vec3, active: boolean) {
  if (isBendComponent(component)) {
    const group = new THREE.Group();
    const material = new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x1f6f73,
      metalness: 0.18,
      roughness: 0.5
    });
    const arc = new THREE.Mesh(new THREE.TorusGeometry(0.24, active ? 0.035 : 0.027, 10, 32, Math.PI * 0.75), material);
    arc.position.set(position.x, position.y + 0.2, position.z);
    arc.rotation.x = Math.PI / 2;
    arc.rotation.z = Math.PI / 4;
    group.add(arc);

    const hub = new THREE.Mesh(
      new THREE.SphereGeometry(active ? 0.08 : 0.06, 18, 12),
      new THREE.MeshStandardMaterial({
        color: active ? 0xf08c22 : 0x2f6f73,
        roughness: 0.48
      })
    );
    hub.position.set(position.x, position.y + 0.2, position.z);
    group.add(hub);
    return group;
  }
  if (isBranchComponent(component)) {
    const group = new THREE.Group();
    const material = new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x24705a,
      metalness: 0.12,
      roughness: 0.54
    });
    const header = new THREE.Mesh(new THREE.CylinderGeometry(0.035, 0.035, 0.42, 16), material);
    header.position.set(position.x, position.y + 0.2, position.z);
    header.rotation.z = Math.PI / 2;
    group.add(header);

    const branch = new THREE.Mesh(new THREE.CylinderGeometry(0.032, 0.032, 0.32, 16), material);
    branch.position.set(position.x, position.y + 0.36, position.z);
    group.add(branch);

    const hub = new THREE.Mesh(
      new THREE.SphereGeometry(active ? 0.085 : 0.065, 18, 12),
      new THREE.MeshStandardMaterial({
        color: active ? 0xf08c22 : 0x1f5c4c,
        roughness: 0.48
      })
    );
    hub.position.set(position.x, position.y + 0.2, position.z);
    group.add(hub);
    return group;
  }
  if (isRigidComponent(component)) {
    const group = new THREE.Group();
    const material = new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x33485f,
      metalness: 0.2,
      roughness: 0.46
    });
    const body = new THREE.Mesh(new THREE.CylinderGeometry(0.055, 0.055, 0.34, 18), material);
    body.position.set(position.x, position.y + 0.2, position.z);
    body.rotation.z = Math.PI / 2;
    group.add(body);

    const bonnet = new THREE.Mesh(
      new THREE.BoxGeometry(0.12, 0.1, 0.12),
      new THREE.MeshStandardMaterial({
        color: active ? 0xf08c22 : 0x4c5d72,
        roughness: 0.5
      })
    );
    bonnet.position.set(position.x, position.y + 0.31, position.z);
    group.add(bonnet);

    const handwheel = new THREE.Mesh(
      new THREE.TorusGeometry(0.07, 0.011, 8, 18),
      new THREE.MeshStandardMaterial({
        color: active ? 0xf08c22 : 0x273344,
        roughness: 0.44
      })
    );
    handwheel.position.set(position.x, position.y + 0.4, position.z);
    handwheel.rotation.x = Math.PI / 2;
    group.add(handwheel);
    return group;
  }
  if (isExpansionJointComponent(component)) {
    const group = new THREE.Group();
    const material = new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x7d5f2c,
      metalness: 0.16,
      roughness: 0.52
    });
    const axis = new THREE.Mesh(new THREE.CylinderGeometry(0.028, 0.028, 0.42, 18), material);
    axis.position.set(position.x, position.y + 0.2, position.z);
    axis.rotation.z = Math.PI / 2;
    group.add(axis);

    const leftRing = new THREE.Mesh(new THREE.TorusGeometry(0.082, 0.014, 8, 20), material);
    leftRing.position.set(position.x - 0.11, position.y + 0.2, position.z);
    leftRing.rotation.y = Math.PI / 2;
    group.add(leftRing);

    const rightRing = new THREE.Mesh(new THREE.TorusGeometry(0.082, 0.014, 8, 20), material);
    rightRing.position.set(position.x + 0.11, position.y + 0.2, position.z);
    rightRing.rotation.y = Math.PI / 2;
    group.add(rightRing);

    const bellows = new THREE.Mesh(
      new THREE.TorusGeometry(active ? 0.075 : 0.064, 0.01, 8, 16),
      new THREE.MeshStandardMaterial({
        color: active ? 0xf08c22 : 0x9d7830,
        roughness: 0.48
      })
    );
    bellows.position.set(position.x, position.y + 0.2, position.z);
    bellows.rotation.y = Math.PI / 2;
    group.add(bellows);
    return group;
  }

  const box = new THREE.Mesh(
    new THREE.BoxGeometry(0.24, 0.24, 0.24),
    new THREE.MeshStandardMaterial({
      color: active ? 0xf08c22 : 0x874c62,
      roughness: 0.52
    })
  );
  box.position.set(position.x, position.y + 0.2, position.z);
  return box;
}

function isBendComponent(component: PreviewComponent): boolean {
  return component.kind === "bend" || component.kind === "elbow";
}

function isBranchComponent(component: PreviewComponent): boolean {
  return component.kind === "branch" || component.kind === "tee" || component.kind === "branch_connection";
}

function isRigidComponent(component: PreviewComponent): boolean {
  return ["valve", "flange", "reducer", "rigid", "specialty"].includes(component.kind);
}

function isExpansionJointComponent(component: PreviewComponent): boolean {
  return component.kind === "expansion_joint";
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
  )
    .translateX(position.x)
    .translateY(position.y)
    .translateZ(position.z);
}

// Ground reference grid on the global XZ plane, centred under the model bounds
// and sized to the model — replaces the old fixed-size grid that was rotated
// into a vertical plane and offset from a hard-coded point.
function referenceGround(model: PreviewModel): THREE.GridHelper {
  const xs = model.nodes.map((node) => node.position.x);
  const ys = model.nodes.map((node) => node.position.y);
  const zs = model.nodes.map((node) => node.position.z);
  const minX = xs.length ? Math.min(...xs) : 0;
  const maxX = xs.length ? Math.max(...xs) : 1;
  const minY = ys.length ? Math.min(...ys) : 0;
  const minZ = zs.length ? Math.min(...zs) : 0;
  const maxZ = zs.length ? Math.max(...zs) : 1;
  const size = Math.max(maxX - minX, maxZ - minZ, 1) * 1.6;
  const divisions = Math.max(4, Math.min(20, Math.round(size)));
  const helper = new THREE.GridHelper(size, divisions, 0xb6bfb9, 0xdce1db);
  helper.position.set((minX + maxX) / 2, minY - 0.02, (minZ + maxZ) / 2);
  const material = helper.material as THREE.Material & { opacity: number };
  material.transparent = true;
  material.opacity = 0.55;
  return helper;
}

// Real 3D load arrows anchored to the loaded node or element midpoint and
// oriented along the load's global direction, so they move with the model.
function buildLoadArrows(model: PreviewModel, nodeMap: Map<string, Vec3>): THREE.Object3D[] {
  const pipeMidpoints = new Map<string, Vec3>();
  for (const segment of model.pipe_segments) {
    const from = nodeMap.get(segment.from);
    const to = nodeMap.get(segment.to);
    if (from && to) pipeMidpoints.set(segment.id, midpoint(from, to));
  }
  const arrows: THREE.Object3D[] = [];
  for (const loadCase of model.load_cases) {
    for (const primitive of loadCase.primitive_loads ?? []) {
      const record = primitive as Record<string, unknown>;
      const anchor = loadAnchor(record, nodeMap, pipeMidpoints);
      const direction = globalDirectionVector(record);
      if (!anchor || !direction) continue;
      const isMoment = String(record.dimension ?? "").includes("moment");
      const color = isMoment ? 0x7b4ea3 : 0xd9822b;
      const origin = new THREE.Vector3(anchor.x, anchor.y, anchor.z);
      arrows.push(new THREE.ArrowHelper(direction, origin, 0.9, color, 0.28, 0.16));
    }
  }
  return arrows;
}

function loadAnchor(
  primitive: Record<string, unknown>,
  nodeMap: Map<string, Vec3>,
  pipeMidpoints: Map<string, Vec3>
): Vec3 | null {
  const target = primitive.target as Record<string, unknown> | undefined;
  if (!target) return null;
  if (target.type === "node" && typeof target.node === "string") return nodeMap.get(target.node) ?? null;
  if (target.type === "element" && typeof target.pipe === "string") return pipeMidpoints.get(target.pipe) ?? null;
  return null;
}

function globalDirectionVector(primitive: Record<string, unknown>): THREE.Vector3 | null {
  const direction = String(primitive.direction ?? "");
  const magnitude = primitive.magnitude as { value?: number } | undefined;
  const sign = (magnitude?.value ?? 1) < 0 ? -1 : 1;
  const axis = direction.includes("_x")
    ? new THREE.Vector3(1, 0, 0)
    : direction.includes("_y")
      ? new THREE.Vector3(0, 1, 0)
      : direction.includes("_z")
        ? new THREE.Vector3(0, 0, 1)
        : null;
  return axis ? axis.multiplyScalar(sign).normalize() : null;
}

// X/Y/Z orientation gizmo: coloured world axes plus letter sprites.
function buildOrientationGizmo(): THREE.Object3D {
  const group = new THREE.Group();
  group.add(new THREE.AxesHelper(1));
  group.add(axisLabelSprite("X", "#b9462f", new THREE.Vector3(1.3, 0, 0)));
  group.add(axisLabelSprite("Y", "#347b46", new THREE.Vector3(0, 1.3, 0)));
  group.add(axisLabelSprite("Z", "#2e638f", new THREE.Vector3(0, 0, 1.3)));
  return group;
}

function axisLabelSprite(text: string, color: string, position: THREE.Vector3): THREE.Sprite {
  const canvas = document.createElement("canvas");
  canvas.width = 64;
  canvas.height = 64;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.fillStyle = color;
    ctx.font = "bold 48px sans-serif";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillText(text, 32, 36);
  }
  const sprite = new THREE.Sprite(
    new THREE.SpriteMaterial({
      map: new THREE.CanvasTexture(canvas),
      transparent: true
    })
  );
  sprite.position.copy(position);
  sprite.scale.set(0.55, 0.55, 0.55);
  return sprite;
}

function toVector(position: Vec3) {
  return new THREE.Vector3(position.x, position.y, position.z);
}

export function buildDeformationOverlay(model: PreviewModel, result: MechanicsResult | null): DeformationOverlay {
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
  const nodeValues = new Map<string, { value: number; unit: string; basisKey: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    const current = nodeValues.get(row.entity_ref);
    if (!current || Math.abs(row.value) > Math.abs(current.value)) {
      nodeValues.set(row.entity_ref, {
        value: row.value,
        unit: row.unit,
        basisKey: rowBasisKey(row)
      });
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

  // Signed global-cartesian translation components (ux/uy/uz rows) grouped by
  // node and result basis, so each node's direction comes from the same load
  // case or combination as its governing magnitude row.
  const nodeComponentVectors = new Map<string, Map<string, Partial<Record<"x" | "y" | "z", number>>>>();
  for (const row of result.results) {
    const axis = DISPLACEMENT_COMPONENT_AXES[row.kind];
    if (!axis || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    const perBasis: Map<string, Partial<Record<"x" | "y" | "z", number>>> = nodeComponentVectors.get(row.entity_ref) ??
    new Map();
    const vector = perBasis.get(rowBasisKey(row)) ?? {};
    vector[axis] = row.value;
    perBasis.set(rowBasisKey(row), vector);
    nodeComponentVectors.set(row.entity_ref, perBasis);
  }
  const nodeDirections = new Map<string, Vec3>();
  let directional = true;
  for (const [nodeId, info] of nodeValues) {
    const vector = nodeComponentVectors.get(nodeId)?.get(info.basisKey);
    if (!vector || vector.x === undefined || vector.y === undefined || vector.z === undefined) {
      directional = false;
      break;
    }
    nodeDirections.set(nodeId, unitVector({ x: vector.x, y: vector.y, z: vector.z }));
  }

  const values = Array.from(nodeValues.values());
  const maxValue = Math.max(...values.map((item) => Math.abs(item.value)));
  const units = Array.from(new Set(values.map((item) => item.unit))).sort();
  const unit = units.length === 1 ? units[0] : "mixed";
  const displayOffset = 0.65;
  const fallbackDirection: Vec3 = { x: 0, y: 1, z: 0 };
  const nodePositions = new Map(
    model.nodes.map((node) => {
      const value = nodeValues.get(node.id)?.value ?? 0;
      const normalizedOffset = maxValue > 0 ? (Math.abs(value) / maxValue) * displayOffset : 0;
      const direction = directional ? (nodeDirections.get(node.id) ?? fallbackDirection) : fallbackDirection;
      return [
        node.id,
        {
          x: node.position.x + direction.x * normalizedOffset,
          y: node.position.y + direction.y * normalizedOffset,
          z: node.position.z + direction.z * normalizedOffset
        }
      ] as const;
    })
  );

  const vectorDirection = directional
    ? "global_cartesian_displacement_components"
    : "vertical_display_axis_fallback_component_rows_unavailable";
  return {
    state: "available",
    summary: `available; nodes=${nodeValues.size}; max=${formatNumber(maxValue)} ${unit}`,
    boundary: `scale=normalized_display_offset_not_physical_length; vector_direction=${vectorDirection}; unit_basis=${unit}; professional_claim=false`,
    nodePositions
  };
}

const DISPLACEMENT_COMPONENT_AXES: Record<string, "x" | "y" | "z" | undefined> = {
  global_nodal_displacement_x: "x",
  global_nodal_displacement_y: "y",
  global_nodal_displacement_z: "z"
};

function rowBasisKey(row: MechanicsResult["results"][number]): string {
  return row.basis_ref ? `${row.basis_ref.ref_type}:${row.basis_ref.ref_id}` : "unspecified";
}

function unitVector(vector: Vec3): Vec3 {
  const length = Math.hypot(vector.x, vector.y, vector.z);
  if (!Number.isFinite(length) || length === 0) {
    return { x: 0, y: 0, z: 0 };
  }
  return { x: vector.x / length, y: vector.y / length, z: vector.z / length };
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
