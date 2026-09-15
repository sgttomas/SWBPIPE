import { QuantityReadout } from "../display-units";
import { Box, CircleDot, CirclePlus, GitBranch, MoveDown, Anchor } from "lucide-react";
import { type PointerEvent as ReactPointerEvent, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { convertDisplayQuantities } from "../../services/displayQuantityService";
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
import {
  buildNodeCreationSubmission,
  buildRouteSubmission,
  DraftReviewGate,
  routeEndNodeId,
  submissionOperationIds,
  type DraftSubmission,
  type FrozenDraftReview,
  type RouteDraft,
  type RouteEndMode
} from "./routeDraft";
import {
  applicableRoutingAxes,
  cancelPointerGesture,
  constrainRoutingPoint,
  convertedRoutingPoint,
  normalizeRoutingAxis,
  pointerGestureMayAuthor,
  resolveExistingRouteGhost,
  resolveNewRouteGhost,
  routingConversionRequests,
  RoutingPlacementGate,
  routingPlaneDefinition,
  startPointerGesture,
  updatePointerGesture,
  type PointerGesture,
  type RouteGhost,
  type RoutingAxisConstraint,
  type RoutingPlane,
  type RoutingPlaneDefinition
} from "./viewportRouting";
import { modelIndexFor, type Bounds3, type ModelIndex } from "../workspace/modelIndex";
import {
  entityKey,
  type EntityKey,
  type OrderedSelectionState
} from "../workspace/selectionState";
import {
  authoredToLocal,
  createRenderTransform,
  pointPickPrimitives,
  prioritizedLabelKeys
} from "./viewportSelection";
import { ViewportResource, type ViewportContextStatus } from "./viewportResource";
import {
  UI_DIAGNOSTICS_VERSION,
  hasUiDiagnosticsObserver,
  publishUiDiagnostics
} from "../workspace/uiDiagnostics";

type Props = {
  armedCreationTool?: CreationTool | null;
  model: PreviewModel;
  modelIndex?: ModelIndex;
  modelCommitToken?: string | null;
  onArmCreationTool?: (tool: CreationTool | null) => void;
  onAddDraft?: (submission: DraftSubmission, generation: number) => Promise<FrozenDraftReview | null>;
  onApplyDraft?: (review: FrozenDraftReview) => Promise<boolean>;
  onInvalidateDraft?: () => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef, modifiers?: { additive?: boolean; toggle?: boolean }) => void;
  queuedIntents?: EditorOperationIntent[];
  reservedIntents?: ReadonlyArray<unknown>;
  result?: MechanicsResult | null;
  selection: EntityRef;
  selectionState?: OrderedSelectionState;
};

export type CreationTool = "node" | "pipe" | "support" | "component" | "load";
type ViewportCommandType = "create_node" | "connect_pipe_run" | "insert_component_symbol";
type ViewPreset = "iso" | "front" | "top";
const VIEWPORT_DIMENSIONLESS_UNIT_VALIDATION_STATUS = "not_required_dimensionless";

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
  provenance: string;
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

type AppliedRouteContinuation = {
  commitToken: string;
  appliedEnd: string;
  continuePipe: boolean;
  pipeDraft: PipeDraft;
  newEndCoordinateUnit: string;
  routingAxis: RoutingAxisConstraint;
  routingPlane: RoutingPlane;
};

type DraftProjector = (
  event: { clientX: number; clientY: number },
  plane: RoutingPlaneDefinition
) => Vec3 | null;

type PointerGhost = {
  point: Vec3;
  provenance: "captured" | "hover";
};

type RoutingVisualState = {
  anchor: Vec3 | null;
  ghost: RouteGhost | null;
  plane: RoutingPlane;
  showGrid: boolean;
};

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
        {value === "" ? <option value="">Select unit</option> : null}
        {options.map((option) => <option key={option.symbol} value={option.symbol}>{option.symbol}</option>)}
      </select>
    </label>
  );
}

type DeformationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  maximum?: { value: number; unit: string };
  nodeCount?: number;
  boundary: string;
  nodePositions: Map<string, Vec3>;
};

export function PipeViewport({
  armedCreationTool = null,
  model,
  modelIndex,
  modelCommitToken = null,
  onArmCreationTool = () => {},
  onAddDraft,
  onApplyDraft,
  onInvalidateDraft = () => {},
  onQueueIntent,
  onSelect,
  queuedIntents = [],
  reservedIntents = [],
  result = null,
  selection,
  selectionState
}: Props) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const viewportResourceRef = useRef<ViewportResource | null>(null);
  const assignmentStartedAtRef = useRef<number | null>(null);
  const mainFrameRef = useRef({ sequence: 0, submittedAt: null as number | null, opportunityAt: null as number | null });
  const draftProjectorRef = useRef<DraftProjector | null>(null);
  const cameraStateRef = useRef<{
    position: [number, number, number];
    target: [number, number, number];
  } | null>(null);
  const lastPresetRef = useRef<ViewPreset | null>(null);
  const fittedSessionGenerationRef = useRef<number | null>(null);
  const pickRef = useRef<((event: { clientX: number; clientY: number }) => EntityRef | null) | null>(null);
  const pointerGestureRef = useRef<{ gesture: PointerGesture; target: Element } | null>(null);
  const placementGenerationRef = useRef(new RoutingPlacementGate());
  const routingVisualUpdaterRef = useRef<((state: RoutingVisualState) => void) | null>(null);
  const routingVisualStateRef = useRef<RoutingVisualState>({ anchor: null, ghost: null, plane: "XZ", showGrid: false });
  const selectionLayerRef = useRef<HTMLDivElement | null>(null);
  const gizmoHostRef = useRef<HTMLDivElement | null>(null);
  const defaultLengthUnit = model.project.units.length ?? "TBD";
  const [localIntents, setLocalIntents] = useState<EditorOperationIntent[]>([]);
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const [nodeDraft, setNodeDraft] = useState<NodeDraft>(() => emptyNodeDraft(defaultLengthUnit));
  const [continuePipe, setContinuePipe] = useState(false);
  const [pipeDraft, setPipeDraft] = useState<PipeDraft>(() => emptyPipeDraft(defaultLengthUnit));
  const [routeEndMode, setRouteEndMode] = useState<RouteEndMode>("existing");
  const [newEndDraft, setNewEndDraft] = useState<NodeDraft>(() => emptyNodeDraft(defaultLengthUnit));
  const [draftReview, setDraftReview] = useState<FrozenDraftReview | null>(null);
  const [draftReviewBusy, setDraftReviewBusy] = useState(false);
  const [draftReviewMessage, setDraftReviewMessage] = useState<string | null>(null);
  const draftReviewGate = useRef(new DraftReviewGate());
  const pendingAppliedRoute = useRef<AppliedRouteContinuation | null>(null);
  const [componentDraft, setComponentDraft] = useState<ComponentDraft>(() =>
    defaultComponentDraft(model, selection, queuedIntents)
  );
  const [pipeEndpointPickMode, setPipeEndpointPickMode] = useState<PipeEndpointPickMode>(null);
  const [routingPlane, setRoutingPlane] = useState<RoutingPlane>("XZ");
  const [routingAxis, setRoutingAxis] = useState<RoutingAxisConstraint>("Free");
  const [pointerGhost, setPointerGhost] = useState<PointerGhost | null>(null);
  const [placementMessage, setPlacementMessage] = useState<string | null>(null);
  const [webglAvailable, setWebglAvailable] = useState<boolean | null>(null);
  const [viewportContextStatus, setViewportContextStatus] = useState<ViewportContextStatus>("ready");
  const [viewPreset, setViewPreset] = useState<ViewPreset>("iso");
  const [showLabels, setShowLabels] = useState(true);
  const [showLoads, setShowLoads] = useState(true);
  const [showGrid, setShowGrid] = useState(true);
  const fallbackModelIndex = useMemo(() => modelIndexFor(model, 0, 0), [model]);
  const activeModelIndex = modelIndex ?? fallbackModelIndex;
  const renderTransform = useMemo(
    () => createRenderTransform(activeModelIndex.geometryBounds),
    [activeModelIndex]
  );
  const orderedSelectionKeys = useMemo(
    () => selectionState?.orderedKeys ?? Object.freeze([entityKey(selection)]),
    [selection.id, selection.type, selectionState]
  );
  const primarySelectionKey = selectionState?.primaryKey ?? entityKey(selection);
  const labelKeys = useMemo(
    () => prioritizedLabelKeys(activeModelIndex, {
      primaryKey: primarySelectionKey,
      selectedKeys: orderedSelectionKeys
    }),
    [activeModelIndex, orderedSelectionKeys, primarySelectionKey]
  );
  const selectionTargets = useMemo(
    () => viewportSelectionTargets(activeModelIndex, labelKeys),
    [activeModelIndex, labelKeys]
  );
  const diagnosticsStateRef = useRef({
    index: activeModelIndex,
    orderedSelectionKeys,
    primarySelectionKey
  });
  diagnosticsStateRef.current = { index: activeModelIndex, orderedSelectionKeys, primarySelectionKey };
  const deformation = useMemo(() => buildDeformationOverlay(model, result), [model, result]);
  const visibleIntents = onQueueIntent ? viewportIntents(queuedIntents) : localIntents;
  const pendingViewportIntents = [...reservedIntents, ...queuedIntents, ...localIntents];
  const routeNodeMap = useMemo(
    () => new Map(model.nodes.map((node) => [node.id, node.position] as const)),
    [model]
  );
  const resolvedFrom = routeNodeMap.get(pipeDraft.from) ?? null;
  const routeGhost = armedCreationTool !== "pipe"
    ? null
    : routeEndMode === "existing"
      ? resolveExistingRouteGhost(routeNodeMap, pipeDraft.from, pipeDraft.to)
      : resolveNewRouteGhost(resolvedFrom, pointerGhost?.point ?? null, pointerGhost?.provenance ?? null);
  const routingControlsDisabled = draftReviewBusy || routeEndMode !== "new" || !resolvedFrom;
  const routingControlReason = routeEndMode !== "new"
    ? "Plane and axis apply only while authoring a new endpoint."
    : !resolvedFrom
      ? "Choose a resolved From node to enable the construction plane and axis."
      : draftReviewBusy
        ? "The frozen draft is being validated or applied."
        : "Plane and axis constrain later pointer placement; typed coordinates remain authoritative.";
  const pointerCaptureReason = webglAvailable === false
    ? "Pointer placement is unavailable because the visible 3D WebGL projection is unavailable. Enter coordinates manually."
    : routeEndMode !== "new"
      ? "Pointer placement is available only for a new endpoint. Existing routes use exact node IDs."
      : !resolvedFrom
        ? "Choose a resolved From node before pointer placement."
        : draftReviewBusy
          ? "Pointer placement is unavailable while the frozen draft is being validated or applied."
          : "Move over the visible 3D canvas to preview; click within 4 CSS pixels to capture.";
  const nodePointerReason = webglAvailable === false
    ? "Pointer node placement is unavailable because the visible 3D WebGL projection is unavailable. Enter coordinates manually."
    : draftReviewBusy
      ? "Pointer node placement is unavailable while the frozen draft is being validated or applied."
      : "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0.";
  routingVisualStateRef.current = {
    anchor: resolvedFrom,
    ghost: routeGhost,
    plane: routingPlane,
    showGrid: armedCreationTool === "pipe" && routeEndMode === "new" && Boolean(resolvedFrom) && webglAvailable === true
  };
  const nodeBuild = buildNodeCreationSubmission(
    model,
    pendingViewportIntents,
    nodeDraft,
    pendingViewportIntents.length + 1,
    `length=${unitDimensionValidationStatus(unitCatalogRoute, nodeDraft.coordinateUnit, "length")}`
  );
  const routeDraft = routeDraftFromState(pipeDraft, routeEndMode, newEndDraft);
  const routeBuild = buildRouteSubmission(
    model,
    pendingViewportIntents,
    routeDraft,
    pendingViewportIntents.length + 1,
    `length=${unitDimensionValidationStatus(unitCatalogRoute, pipeDraft.lengthUnit, "length")}`
  );
  const nodeDraftValid = nodeBuild.ok;
  const pipeDraftValid = routeBuild.ok;
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
  const componentAreaUnitOptions = unitOptions(unitCatalogRoute, "area", "m^2");
  const componentMovementUnitOptions = unitOptions(unitCatalogRoute, "length", defaultLengthUnit);
  const componentLinearStiffnessUnitOptions = unitOptions(unitCatalogRoute, "linear_stiffness", "N/m");
  const componentRotationalStiffnessUnitOptions = unitOptions(
    unitCatalogRoute,
    "rotational_stiffness",
    "N*m/rad"
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
    clearPointerPlacement();
    if (armedCreationTool === "pipe") {
      setPipeEndpointPickMode("from");
    } else {
      setPipeEndpointPickMode(null);
      setContinuePipe(false);
      setPipeDraft(emptyPipeDraft(defaultLengthUnit));
      setRouteEndMode("existing");
      setNewEndDraft(emptyNodeDraft(defaultLengthUnit));
      setRoutingPlane("XZ");
      setRoutingAxis("Free");
    }
  }, [armedCreationTool]);

  useEffect(() => {
    clearPointerPlacement();
    const continuation = pendingAppliedRoute.current;
    if (continuation && modelCommitToken === continuation.commitToken) {
      pendingAppliedRoute.current = null;
      invalidateDraftReview("Applied once through the structured operation service.");
      finishAppliedRoute(continuation);
      return;
    }
    pendingAppliedRoute.current = null;
    invalidateDraftReview("The model changed. Add again to review the current draft.");
    setContinuePipe(false);
    setPipeDraft(emptyPipeDraft(defaultLengthUnit));
    setRouteEndMode("existing");
    setNewEndDraft(emptyNodeDraft(defaultLengthUnit));
    setPipeEndpointPickMode(null);
    setRoutingPlane("XZ");
    setRoutingAxis("Free");
  }, [model, modelCommitToken]);

  useEffect(() => {
    clearPointerPlacement();
    invalidateDraftReview("The affected selection changed. Add again to review the current draft.");
  }, [selection.id, selection.type, selectionState?.preparationEpoch]);

  useEffect(() => {
    routingVisualUpdaterRef.current?.(routingVisualStateRef.current);
  }, [resolvedFrom, routeGhost, routingPlane, routeEndMode, armedCreationTool, webglAvailable]);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    assignmentStartedAtRef.current = performance.now();
    let resource: ViewportResource;
    try {
      resource = new ViewportResource(host, {
        onContextStatus: setViewportContextStatus,
        onRestore: () => viewportResourceRef.current?.invalidate(),
        shouldObservePaintOpportunity: hasUiDiagnosticsObserver,
        onAfterMainFrame: (current, submittedAt) => {
          mainFrameRef.current = {
            sequence: current.submissionSequence,
            submittedAt,
            opportunityAt: null
          };
          publishViewportDiagnostics(current);
        },
        onNextPaintOpportunity: (current, opportunityAt) => {
          mainFrameRef.current = { ...mainFrameRef.current, opportunityAt };
          publishViewportDiagnostics(current);
        }
      });
    } catch {
      setWebglAvailable(false);
      draftProjectorRef.current = null;
      routingVisualUpdaterRef.current = null;
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
    setWebglAvailable(true);
    viewportResourceRef.current = resource;
    resource.setOrigin(renderTransform.origin);
    fitViewportCamera(resource, viewPreset, renderTransform.localBounds);
    fittedSessionGenerationRef.current = activeModelIndex.sessionGeneration;
    lastPresetRef.current = viewPreset;
    draftProjectorRef.current = (event, plane) => resource.projectToAuthoredPlane(
      event,
      plane.normal,
      plane.fixedValue
    );
    pickRef.current = (event) => resource.pick(event);
    const persistCameraState = () => {
      cameraStateRef.current = {
        position: [resource.camera.position.x, resource.camera.position.y, resource.camera.position.z],
        target: [resource.controls.target.x, resource.controls.target.y, resource.controls.target.z]
      };
    };
    resource.controls.addEventListener("change", persistCameraState);
    resource.invalidate();

    return () => {
      resource.controls.removeEventListener("change", persistCameraState);
      pickRef.current = null;
      draftProjectorRef.current = null;
      routingVisualUpdaterRef.current = null;
      viewportResourceRef.current = null;
      resource.dispose();
    };
  }, []);

  useEffect(() => {
    assignmentStartedAtRef.current = performance.now();
    mainFrameRef.current = { sequence: 0, submittedAt: null, opportunityAt: null };
  }, [activeModelIndex.generation]);

  function publishViewportDiagnostics(resource: ViewportResource): void {
    publishUiDiagnostics(
      () => {
        const diagnostics = diagnosticsStateRef.current;
        const info = resource.renderer.info;
        const authoredCamera = resource.localToAuthored(resource.camera.position);
        const authoredTarget = resource.localToAuthored(resource.controls.target);
        return {
          version: UI_DIAGNOSTICS_VERSION,
          sessionGeneration: diagnostics.index.sessionGeneration,
          modelGeneration: diagnostics.index.generation,
          assignmentStartedAt: assignmentStartedAtRef.current,
          mainFrameSubmissionSequence: mainFrameRef.current.sequence,
          mainFrameSubmittedAt: mainFrameRef.current.submittedAt,
          nextPaintOpportunityAt: mainFrameRef.current.opportunityAt,
          selectionKeys: diagnostics.orderedSelectionKeys,
          primaryKey: diagnostics.primarySelectionKey,
          boxSelectionMode: false,
          entityFilter: "all" as const,
          camera: {
            position: [authoredCamera.x, authoredCamera.y, authoredCamera.z] as const,
            target: [authoredTarget.x, authoredTarget.y, authoredTarget.z] as const,
            up: [resource.camera.up.x, resource.camera.up.y, resource.camera.up.z] as const
          },
          renderOrigin: [resource.origin.x, resource.origin.y, resource.origin.z] as const,
          rendererInfo: {
            geometries: info.memory.geometries,
            textures: info.memory.textures,
            calls: info.render.calls,
            triangles: info.render.triangles,
            points: info.render.points,
            lines: info.render.lines
          },
          pendingAppOwnedRafCount: resource.pendingAppOwnedRafCount
        };
      },
      () => {
        resource.camera.updateMatrixWorld();
        const viewProjection = new THREE.Matrix4()
          .multiplyMatrices(resource.camera.projectionMatrix, resource.camera.matrixWorldInverse);
        const rect = resource.renderer.domElement.getBoundingClientRect();
        return {
          generation: diagnosticsStateRef.current.index.generation,
          renderOrigin: resource.origin,
          viewProjectionMatrix: viewProjection.toArray(),
          canvasRect: { left: rect.left, top: rect.top, width: rect.width, height: rect.height }
        };
      }
    );
  }

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    resource.setOrigin(renderTransform.origin);
    const localNodeMap = new Map(
      model.nodes
        .filter((node) => !activeModelIndex.invalidGeometry.has(entityKey({ type: "node", id: node.id })))
        .map((node) => [node.id, authoredToLocal(node.position, renderTransform.origin)] as const)
    );
    const labelKeySet = new Set(labelKeys);
    const modelObjects: THREE.Object3D[] = [];
    const resultObjects: THREE.Object3D[] = [];
    const loadObjects: THREE.Object3D[] = [];
    const pickables: THREE.Object3D[] = [];
    const anchorPositions: Array<{
      key: EntityKey;
      position: THREE.Vector3;
      offsetPct: number;
    }> = [];
    for (const key of labelKeySet) {
      const indexed = activeModelIndex.entities.get(key);
      if (!indexed?.anchor) continue;
      const position = authoredToLocal(indexed.anchor, renderTransform.origin);
      if (indexed.ref.type === "support") position.y -= 0.26;
      if (indexed.ref.type === "component") position.y += 0.2;
      anchorPositions.push({
        key,
        position: new THREE.Vector3(position.x, position.y, position.z),
        offsetPct: indexed.ref.type === "support" ? 8 : indexed.ref.type === "component" ? -8 : 0
      });
    }
    const pipeInstances = instancedPipeMesh(model, localNodeMap, orderedSelectionKeys);
    if (pipeInstances) modelObjects.push(pipeInstances);
    if (deformation.state === "available") {
      for (const segment of model.pipe_segments) {
        const authoredFrom = deformation.nodePositions.get(segment.from);
        const authoredTo = deformation.nodePositions.get(segment.to);
        if (!authoredFrom || !authoredTo) continue;
        const ref: EntityRef = { type: "pipe", id: segment.id };
        resultObjects.push(deformedPipeMesh(
          authoredToLocal(authoredFrom, renderTransform.origin),
          authoredToLocal(authoredTo, renderTransform.origin),
          selectionContainsKey(orderedSelectionKeys, ref)
        ));
      }
      for (const node of model.nodes) {
        const authoredPosition = deformation.nodePositions.get(node.id);
        if (!authoredPosition) continue;
        const ref: EntityRef = { type: "node", id: node.id };
        resultObjects.push(deformationMarker(
          authoredToLocal(authoredPosition, renderTransform.origin),
          selectionContainsKey(orderedSelectionKeys, ref)
        ));
      }
    }
    const nodeInstances = instancedNodeMesh(model, localNodeMap, orderedSelectionKeys);
    if (nodeInstances) modelObjects.push(nodeInstances);
    const supportInstances = instancedSupportMesh(model, localNodeMap, orderedSelectionKeys);
    if (supportInstances) modelObjects.push(supportInstances);
    for (const component of model.components) {
      const node = localNodeMap.get(component.node);
      if (!node) continue;
      const ref: EntityRef = { type: "component", id: component.id };
      const mesh = componentMesh(component, node, selectionContainsKey(orderedSelectionKeys, ref));
      mesh.userData.entityRef = ref;
      pickables.push(mesh);
      modelObjects.push(mesh);
    }
    if (showGrid) modelObjects.push(referenceGroundFromBounds(renderTransform.localBounds));
    if (showLoads) loadObjects.push(...buildLoadArrows(model, localNodeMap));

    const routingGrid = routeConstructionGridFromBounds(renderTransform.localBounds);
    const routingGhost = routeGhostLine();
    const routingMarker = marker({ x: 0, y: 0, z: 0 }, 0xf08c22, 0.105);
    routingGrid.visible = false;
    routingGhost.visible = false;
    routingMarker.visible = false;
    routingVisualUpdaterRef.current = (state) => {
      const localAnchor = state.anchor ? authoredToLocal(state.anchor, renderTransform.origin) : null;
      const localGhost = state.ghost ? {
        ...state.ghost,
        from: authoredToLocal(state.ghost.from, renderTransform.origin),
        to: authoredToLocal(state.ghost.to, renderTransform.origin)
      } : null;
      updateRouteConstructionGrid(routingGrid, localAnchor, state.plane, state.showGrid);
      updateRouteGhostObjects(routingGhost, routingMarker, localGhost);
      resource.invalidate();
    };
    routingVisualUpdaterRef.current(routingVisualStateRef.current);
    const updateLabelAnchors = () => {
      const layer = selectionLayerRef.current;
      if (!layer) return;
      const buttons = new Map(
        [...layer.querySelectorAll<HTMLElement>("[data-entity-key]")]
          .map((button) => [button.dataset.entityKey, button] as const)
      );
      for (const { key, position, offsetPct } of anchorPositions) {
        const button = buttons.get(key);
        if (!button) continue;
        const projected = position.clone().project(resource.camera);
        const behind = projected.z < -1 || projected.z > 1;
        button.style.display = behind ? "none" : "";
        if (behind) continue;
        button.style.left = `${clamp((projected.x * 0.5 + 0.5) * 100, 2, 98)}%`;
        button.style.top = `${clamp((-projected.y * 0.5 + 0.5) * 100 + offsetPct, 2, 98)}%`;
      }
    };
    resource.setPickables(pickables);
    resource.setPointPrimitives(activeModelIndex, pointPickPrimitives(activeModelIndex, model, renderTransform.origin));
    resource.setLabelUpdater(updateLabelAnchors);
    resource.replaceLayer(resource.modelLayer, modelObjects);
    resource.replaceLayer(resource.authoredLoadLayer, loadObjects);
    resource.replaceLayer(resource.resultLayer, resultObjects);
    resource.replaceLayer(resource.routingLayer, [routingGrid, routingGhost, routingMarker]);
    resource.invalidate();
    return () => resource.setLabelUpdater(null);
  }, [activeModelIndex, deformation, labelKeys, model, orderedSelectionKeys, renderTransform, showGrid, showLoads]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource || fittedSessionGenerationRef.current === activeModelIndex.sessionGeneration) return;
    fitViewportCamera(resource, viewPreset, renderTransform.localBounds);
    fittedSessionGenerationRef.current = activeModelIndex.sessionGeneration;
    lastPresetRef.current = viewPreset;
    resource.invalidate();
  }, [activeModelIndex.sessionGeneration, renderTransform.localBounds, viewPreset]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource || lastPresetRef.current === viewPreset) return;
    fitViewportCamera(resource, viewPreset, renderTransform.localBounds);
    lastPresetRef.current = viewPreset;
    resource.invalidate();
  }, [renderTransform.localBounds, viewPreset]);

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

  async function addExplicitNodeIntent() {
    if (!nodeBuild.ok) return;
    if (!onAddDraft) {
      queueIntent(nodeBuild.submission.kind === "single" ? nodeBuild.submission.intent : nodeBuild.submission.batch.operations[0]);
      setNodeDraft(emptyNodeDraft(nodeDraft.coordinateUnit || defaultLengthUnit));
      return;
    }
    await addDraftReview(nodeBuild.submission);
  }

  async function addExplicitPipeIntent() {
    if (!routeBuild.ok) return;
    if (!onAddDraft) {
      if (routeBuild.submission.kind === "single") queueIntent(routeBuild.submission.intent);
      else {
        setDraftReviewMessage("Atomic route review is unavailable. No operation was queued.");
        return;
      }
      finishAppliedRoute();
      return;
    }
    await addDraftReview(routeBuild.submission);
  }

  function cancelPipeDraft() {
    clearPointerPlacement();
    invalidateDraftReview("Route canceled. No reviewed operation can be applied.");
    setContinuePipe(false);
    setPipeDraft(emptyPipeDraft(defaultLengthUnit));
    setRouteEndMode("existing");
    setNewEndDraft(emptyNodeDraft(defaultLengthUnit));
    setPipeEndpointPickMode(null);
    setRoutingPlane("XZ");
    setRoutingAxis("Free");
    onArmCreationTool(null);
  }

  async function addDraftReview(submission: DraftSubmission) {
    if (!onAddDraft || draftReviewBusy) return;
    const generation = draftReviewGate.current.invalidate();
    setDraftReview(null);
    setDraftReviewMessage("Validating the frozen draft and generating its diff…");
    setDraftReviewBusy(true);
    try {
      const review = await onAddDraft(structuredClone(submission), generation);
      if (!draftReviewGate.current.isCurrent(generation)) return;
      if (!review) {
        setDraftReviewMessage("The draft was not validated. Review the current inputs and Add again.");
        return;
      }
      setDraftReview(review);
      setDraftReviewMessage("Validation passed. Review the exact operations and diff, then Apply.");
    } finally {
      if (draftReviewGate.current.isCurrent(generation)) setDraftReviewBusy(false);
    }
  }

  async function applyDraftReview() {
    if (!draftReview || !onApplyDraft || draftReviewBusy) return;
    const generation = draftReview.generation;
    setDraftReviewBusy(true);
    setDraftReviewMessage("Applying the reviewed operation once…");
    const submission = draftReview.submission;
    const routeContinuation: AppliedRouteContinuation | null =
      submissionOperationIds(submission).some((id) => id.includes("connect-pipe"))
        ? {
            commitToken: draftReview.reviewId,
            appliedEnd: routeEndNodeId(routeDraft),
            continuePipe,
            pipeDraft: structuredClone(pipeDraft),
            newEndCoordinateUnit: newEndDraft.coordinateUnit || defaultLengthUnit,
            routingAxis,
            routingPlane
          }
        : null;
    pendingAppliedRoute.current = routeContinuation;
    try {
      const applied = await onApplyDraft(draftReview);
      if (!draftReviewGate.current.isCurrent(generation)) return;
      if (!applied) {
        pendingAppliedRoute.current = null;
        setDraftReview(null);
        draftReviewGate.current.invalidate();
        setDraftReviewMessage("Apply did not publish a model. Update the draft and Add again.");
        return;
      }
      setDraftReview(null);
      draftReviewGate.current.invalidate();
      setDraftReviewMessage("Applied once through the structured operation service.");
      if (!routeContinuation) setNodeDraft(emptyNodeDraft(nodeDraft.coordinateUnit || defaultLengthUnit));
    } finally {
      setDraftReviewBusy(false);
    }
  }

  function finishAppliedRoute(continuation: AppliedRouteContinuation = {
    commitToken: "local_queued_route",
    appliedEnd: routeEndNodeId(routeDraft),
    continuePipe,
    pipeDraft,
    newEndCoordinateUnit: newEndDraft.coordinateUnit || defaultLengthUnit,
    routingAxis,
    routingPlane
  }) {
    clearPointerPlacement();
    setPipeDraft(continuation.continuePipe
      ? { ...continuation.pipeDraft, id: "", label: "", from: continuation.appliedEnd, to: "" }
      : emptyPipeDraft(continuation.pipeDraft.lengthUnit || defaultLengthUnit));
    setRouteEndMode(continuation.continuePipe ? "new" : "existing");
    setNewEndDraft(emptyNodeDraft(continuation.newEndCoordinateUnit));
    setPipeEndpointPickMode(null);
    setRoutingPlane(continuation.continuePipe ? continuation.routingPlane : "XZ");
    setRoutingAxis(continuation.continuePipe
      ? normalizeRoutingAxis(continuation.routingPlane, continuation.routingAxis)
      : "Free");
  }

  function invalidateDraftReview(message: string | null = null) {
    draftReviewGate.current.invalidate();
    setDraftReview(null);
    onInvalidateDraft();
    if (message) setDraftReviewMessage(message);
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
    if (field === "x" || field === "y" || field === "z" || field === "coordinateUnit") {
      clearPointerPlacement();
    } else {
      placementGenerationRef.current.invalidate();
    }
    invalidateDraftReview();
    setNodeDraft((current) => ({ ...current, [field]: value }));
  }

  function updatePipeDraft(field: keyof PipeDraft, value: string) {
    if (field === "from" || field === "to") clearPointerPlacement();
    invalidateDraftReview();
    setPipeDraft((current) => ({ ...current, [field]: value }));
    if (field === "from" || field === "to") {
      setPipeEndpointPickMode(null);
    }
  }

  function updateNewEndDraft(field: keyof NodeDraft, value: string) {
    if (field === "x" || field === "y" || field === "z" || field === "coordinateUnit") {
      clearPointerPlacement();
    } else {
      placementGenerationRef.current.invalidate();
    }
    invalidateDraftReview();
    setNewEndDraft((current) => ({ ...current, [field]: value }));
  }

  function updateComponentDraft<K extends keyof ComponentDraft>(field: K, value: ComponentDraft[K]) {
    setComponentDraft((current) => ({ ...current, [field]: value }));
  }

  function armPipeEndpointPick(mode: Exclude<PipeEndpointPickMode, null>) {
    setPipeEndpointPickMode((current) => (current === mode ? null : mode));
  }

  function chooseViewportTarget(
    target: ViewportSelectionTarget,
    modifiers: { additive?: boolean; toggle?: boolean } = {}
  ) {
    if (draftReviewBusy) return;
    if (pipeEndpointPickMode && target.kind === "node") {
      invalidateDraftReview();
      const mode = pipeEndpointPickMode;
      setPipeDraft((current) => nextPipeDraftWithEndpoint(current, mode, target.ref.id));
      setPipeEndpointPickMode(mode === "from" ? "to" : null);
    }
    if (armedCreationTool === "component" && target.kind === "node") {
      setComponentDraft((current) => componentDraftForNode(model, current, target.ref.id));
    }
    onSelect(target.ref, modifiers);
  }

  function clearPointerPlacement(message: string | null = null) {
    placementGenerationRef.current.invalidate();
    releasePointerCandidate();
    setPointerGhost(null);
    setPlacementMessage(message);
  }

  function handleViewportPointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (draftReviewBusy) return;
    if (event.button !== 0 && event.button !== undefined) return;
    if (event.isPrimary === false) return;
    releasePointerCandidate();
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    const clientX = finitePointerEventNumber(event.clientX, 0);
    const clientY = finitePointerEventNumber(event.clientY, 0);
    const target = event.target as Element & { setPointerCapture?: (pointerId: number) => void };
    pointerGestureRef.current = {
      gesture: startPointerGesture(pointerId, clientX, clientY),
      target
    };
    target.setPointerCapture?.(pointerId);
  }

  function handleViewportPointerMove(event: ReactPointerEvent<HTMLDivElement>) {
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    if (candidate) {
      candidate.gesture = updatePointerGesture(
        candidate.gesture,
        pointerId,
        finitePointerEventNumber(event.clientX, 0),
        finitePointerEventNumber(event.clientY, 0)
      );
    }
    if (
      draftReviewBusy ||
      armedCreationTool !== "pipe" ||
      routeEndMode !== "new" ||
      !resolvedFrom ||
      !(event.target instanceof HTMLCanvasElement)
    ) return;
    const projected = projectRoutePoint(event, resolvedFrom);
    if (!projected) {
      setPointerGhost(null);
      setPlacementMessage("The pointer ray has no finite intersection with the selected construction plane.");
      return;
    }
    setPointerGhost({ point: projected, provenance: "hover" });
    setPlacementMessage(null);
  }

  function handleViewportPointerLeave() {
    setPointerGhost((current) => current?.provenance === "hover" ? null : current);
  }

  function handleViewportPointerCancel(event: ReactPointerEvent<HTMLDivElement>) {
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    if (candidate && candidate.gesture.pointerId === pointerId) {
      candidate.gesture = cancelPointerGesture(candidate.gesture);
      releasePointerCandidate(pointerId);
      setPointerGhost((current) => current?.provenance === "hover" ? null : current);
      setPlacementMessage("Pointer placement canceled; coordinates were not changed.");
    }
  }

  function handleViewportLostPointerCapture(event: ReactPointerEvent<HTMLDivElement>) {
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    if (candidate && candidate.gesture.pointerId === pointerId) {
      pointerGestureRef.current = null;
      setPointerGhost((current) => current?.provenance === "hover" ? null : current);
      setPlacementMessage("Pointer capture was lost; coordinates were not changed.");
    }
  }

  function handleViewportPointerUp(event: ReactPointerEvent<HTMLDivElement>) {
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    const clientX = finitePointerEventNumber(event.clientX, 0);
    const clientY = finitePointerEventNumber(event.clientY, 0);
    const mayAuthor = pointerGestureMayAuthor(
      candidate?.gesture ?? null,
      pointerId,
      clientX,
      clientY
    );
    releasePointerCandidate(pointerId);
    if (!mayAuthor || draftReviewBusy) {
      if (candidate) setPlacementMessage("Pointer movement exceeded 4 CSS pixels; no selection or coordinate was captured.");
      return;
    }
    const picked = pickRef.current?.(event);
    if (picked) {
      if (armedCreationTool === "pipe" && pipeEndpointPickMode && picked.type === "node") {
        invalidateDraftReview();
        setPipeDraft((current) => nextPipeDraftWithEndpoint(current, pipeEndpointPickMode, picked.id));
        setPipeEndpointPickMode(pipeEndpointPickMode === "from" ? "to" : null);
      }
      if (armedCreationTool === "component" && picked.type === "node") {
        setComponentDraft((current) => componentDraftForNode(model, current, picked.id));
      }
      onSelect(picked, {
        additive: event.shiftKey,
        toggle: event.ctrlKey || event.metaKey
      });
      return;
    }
    if (armedCreationTool === "node") {
      const projected = draftProjectorRef.current?.(
        event,
        routingPlaneDefinition("XZ", { x: 0, y: 0, z: 0 })
      );
      if (!projected) {
        setPlacementMessage("Pointer node placement requires the visible WebGL projection; enter coordinates manually.");
        return;
      }
      void captureConvertedPoint(projected, nodeDraft.coordinateUnit || defaultLengthUnit, "node");
    } else if (armedCreationTool === "pipe" && routeEndMode === "new") {
      if (!resolvedFrom) {
        setPlacementMessage("Choose a resolved From node before pointer placement.");
        return;
      }
      const projected = projectRoutePoint(event, resolvedFrom);
      if (!projected) {
        setPlacementMessage("The pointer ray has no finite intersection with the selected construction plane.");
        return;
      }
      void captureConvertedPoint(projected, newEndDraft.coordinateUnit || defaultLengthUnit, "route");
    }
  }

  function releasePointerCandidate(pointerId?: number) {
    const candidate = pointerGestureRef.current;
    if (!candidate) return;
    const id = pointerId ?? candidate.gesture.pointerId;
    const target = candidate.target as Element & {
      hasPointerCapture?: (pointerId: number) => boolean;
      releasePointerCapture?: (pointerId: number) => void;
    };
    try {
      if (!target.hasPointerCapture || target.hasPointerCapture(id)) target.releasePointerCapture?.(id);
    } catch {
      // A browser may have already released capture while unmounting the canvas.
    }
    pointerGestureRef.current = null;
  }

  function projectRoutePoint(event: { clientX: number; clientY: number }, from: Vec3): Vec3 | null {
    const projected = draftProjectorRef.current?.(event, routingPlaneDefinition(routingPlane, from)) ?? null;
    return projected ? constrainRoutingPoint(projected, from, routingPlane, routingAxis) : null;
  }

  async function captureConvertedPoint(point: Vec3, coordinateUnit: string, target: "node" | "route") {
    const generation = placementGenerationRef.current.invalidate();
    setPlacementMessage(`Converting model coordinates from ${defaultLengthUnit} to ${coordinateUnit}…`);
    try {
      const results = await convertDisplayQuantities(
        routingConversionRequests(point, defaultLengthUnit, coordinateUnit)
      );
      if (!placementGenerationRef.current.isCurrent(generation)) return;
      const converted = convertedRoutingPoint(results, coordinateUnit);
      if (!converted) {
        setPlacementMessage("Pointer placement conversion failed: the engine did not return exactly one finite, matching result for X, Y, and Z.");
        return;
      }
      invalidateDraftReview();
      if (target === "node") {
        setNodeDraft(buildDraftNodeFromViewportPoint(
          model,
          [...queuedIntents, ...localIntents],
          converted,
          coordinateUnit
        ));
      } else {
        const identity = nextViewportNodeIdentity(model, [...queuedIntents, ...localIntents]);
        setNewEndDraft({
          id: identity.id,
          label: identity.label,
          coordinateUnit,
          x: formatDraftCoordinate(converted.x),
          y: formatDraftCoordinate(converted.y),
          z: formatDraftCoordinate(converted.z),
          provenance: ""
        });
        setPointerGhost({ point, provenance: "captured" });
      }
      setPlacementMessage(`Captured pointer coordinates in ${coordinateUnit}; provenance remains required.`);
    } catch (error: unknown) {
      if (!placementGenerationRef.current.isCurrent(generation)) return;
      setPlacementMessage(`Pointer placement conversion failed: ${error instanceof Error ? error.message : "conversion unavailable"}`);
    }
  }

  function changeRoutingPlane(nextPlane: RoutingPlane) {
    clearPointerPlacement();
    invalidateDraftReview();
    setRoutingPlane(nextPlane);
    setRoutingAxis((current) => normalizeRoutingAxis(nextPlane, current));
  }

  function changeRoutingAxis(nextAxis: RoutingAxisConstraint) {
    clearPointerPlacement();
    invalidateDraftReview();
    setRoutingAxis(normalizeRoutingAxis(routingPlane, nextAxis));
  }

  const canCancelPipeDraft = armedCreationTool === "pipe" || continuePipe || Boolean(pipeEndpointPickMode) ||
    routeEndMode === "new" || Boolean(draftReview) ||
    JSON.stringify(pipeDraft) !== JSON.stringify(emptyPipeDraft(defaultLengthUnit)) ||
    JSON.stringify(newEndDraft) !== JSON.stringify(emptyNodeDraft(defaultLengthUnit));
  const nodeToolActive = armedCreationTool === "node";
  const pipeToolActive = armedCreationTool === "pipe";
  const componentToolActive = armedCreationTool === "component";
  const viewportIntentPanelActive = nodeToolActive || pipeToolActive || componentToolActive || visibleIntents.length > 0;
  const nodeAddReason = draftReviewBusy
    ? "Add node is unavailable while the frozen draft is being validated or applied."
    : nodeBuild.ok
      ? "Ready to validate and freeze this explicit node create intent."
      : nodeBuild.errors.join("; ");
  const routeAddReason = draftReviewBusy
    ? "Add route is unavailable while the frozen draft is being validated or applied."
    : routeBuild.ok
      ? "Ready to validate and freeze this explicit straight route."
      : routeBuild.errors.join("; ");
  const constructionPlaneReadout = resolvedFrom
    ? `Construction plane: ${routingPlane} · ${routingPlaneDefinition(routingPlane, resolvedFrom).fixedAxis.toUpperCase()}=${formatDraftCoordinate(routingPlaneDefinition(routingPlane, resolvedFrom).fixedValue)} ${defaultLengthUnit} · through ${pipeDraft.from}`
    : "Construction plane: unavailable · choose a resolved From node";

  return (
    <div className="viewport-shell">
      <div className="viewport-toolbar">
        <span>3D Centerline</span>
        <details
          className={`viewport-deformation-status ${deformation.state}`}
          aria-label="Viewport deformation overlay status"
          data-testid="viewport-deformation-status"
        >
          <summary>{deformation.state === "available" ? "Deformation · normalized" : deformation.state === "blocked" ? "Deformation · blocked" : "Deformation · unavailable"}</summary>
          <div><strong data-testid="viewport-deformation-summary">{deformation.maximum ? <>available; nodes={deformation.nodeCount}; max=<QuantityReadout quantity={{ ...deformation.maximum, dimension_id: "length" }} /></> : deformation.summary}</strong>
          <small data-testid="viewport-deformation-boundary">{deformation.boundary}</small></div>
        </details>
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
        {viewportContextStatus !== "ready" ? (
          <span role="status" data-testid="viewport-context-status">
            {viewportContextStatus === "lost" ? "3D context lost; rendering paused." : "Restoring 3D resources…"}
          </span>
        ) : null}
        {activeModelIndex.invalidGeometry.size > 0 ? (
          <span role="status" data-testid="viewport-invalid-geometry">
            {activeModelIndex.invalidGeometry.size} item{activeModelIndex.invalidGeometry.size === 1 ? "" : "s"} excluded: {activeModelIndex.invalidGeometry.values().next().value}
          </span>
        ) : null}
      </div>
      <div className="viewport-frame">
        <div
          className="viewport-canvas"
          data-testid="viewport-canvas"
          onPointerDown={handleViewportPointerDown}
          onPointerMove={handleViewportPointerMove}
          onPointerLeave={handleViewportPointerLeave}
          onPointerUp={handleViewportPointerUp}
          onPointerCancel={handleViewportPointerCancel}
          onLostPointerCapture={handleViewportLostPointerCapture}
          ref={hostRef}
          aria-label="Three.js pipe centerline viewport"
          title={webglAvailable === false ? pointerCaptureReason : "Click a part to select it; drag to orbit, scroll to zoom"}
        />
        {showLabels ? (
          <div
            className="viewport-selection-layer"
            aria-label="Viewport entity selection"
            data-testid="viewport-selection-layer"
            ref={selectionLayerRef}
          >
            {selectionTargets.map((target) => {
              const active = selectionContainsKey(orderedSelectionKeys, target.ref);
              return (
                <button
                  aria-label={`Select ${target.label} in viewport`}
                  aria-pressed={active}
                  className={`viewport-select-target ${target.kind} ${active ? "active" : ""}`}
                  data-entity-key={entityKey(target.ref)}
                  data-testid={`viewport-select-${target.ref.id}`}
                  disabled={draftReviewBusy}
                  key={entityKey(target.ref)}
                  onClick={(event) => chooseViewportTarget(target, {
                    additive: event.shiftKey,
                    toggle: event.ctrlKey || event.metaKey
                  })}
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
          <QuantityReadout quantity={{ value: 1, unit: defaultLengthUnit, dimension_id: "length" }} />
        </div>
      </div>
      <section className="command-bar" aria-label="Command and selection bar" data-testid="command-bar">
        <div className="command-buttons" aria-label="Object creation tools">
          <button
            type="button"
            className={armedCreationTool === "node" ? "active" : ""}
            data-testid="command-node"
            disabled={draftReviewBusy}
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
            disabled={draftReviewBusy}
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
            disabled={draftReviewBusy}
            aria-pressed={armedCreationTool === "support"}
            onClick={() => armCreationTool("support")}
            title="Arm support creation in the Inspector"
          >
            <Anchor size={15} aria-hidden="true" />
            Support
          </button>
          <button
            type="button"
            className={armedCreationTool === "component" ? "active" : ""}
            data-testid="command-component"
            disabled={draftReviewBusy}
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
            disabled={draftReviewBusy}
            aria-pressed={armedCreationTool === "load"}
            onClick={() => armCreationTool("load")}
            title="Arm load creation in the Load Cases panel"
          >
            <MoveDown size={15} aria-hidden="true" />
            Load
          </button>
        </div>
        <span className="command-active-tool" data-testid="armed-creation-tool" title={creationToolStatusLabel(armedCreationTool)}>
          {armedCreationTool ? `${armedCreationTool.charAt(0).toUpperCase()}${armedCreationTool.slice(1)} tool armed` : "Select"}
        </span>
        <details className="command-context"><summary>Selection &amp; navigation</summary><div className="command-context-content">
        <button
          type="button"
          className="command-preview-button"
          data-testid="queue-armed-creation-intent"
          disabled={draftReviewBusy || !viewportCommandTypeForCreationTool(armedCreationTool)}
          onClick={queueArmedPreviewIntent}
          title="Queue a review-only preview intent for the armed viewport tool"
        >
          Queue preview
        </button>
        <span data-testid="command-selection-readout">
          Selected {selection.type}: {selection.id}; {visibleIntents.length} queued
        </span>
        <span className="command-hint" data-testid="viewport-orbit-hint">
          Drag to orbit · scroll to zoom · right-drag to pan
        </span>
        </div></details>
      </section>
      <section
        className={`viewport-intents${viewportIntentPanelActive ? " active" : " collapsed"}`}
        aria-label="Viewport editor intents"
        data-testid="viewport-editor-intents"
      >
        <h3 className="viewport-tool-heading">{nodeToolActive ? "Create node" : pipeToolActive ? "Create pipe" : componentToolActive ? "Insert component" : "Pending changes"}</h3>
        <fieldset className="viewport-intent-controls" disabled={draftReviewBusy} data-testid="viewport-draft-flight-controls">
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
            <small data-testid="viewport-node-construction-plane">Pointer plane: global XZ · Y=0 {defaultLengthUnit}</small>
            <small data-testid="viewport-node-pointer-status" title={nodePointerReason}>{placementMessage ?? nodePointerReason}</small>
            <label>
              <span>Provenance</span>
              <input
                aria-label="New node provenance"
                data-testid="viewport-create-node-provenance"
                onChange={(event) => updateNodeDraft("provenance", event.target.value)}
                value={nodeDraft.provenance}
              />
            </label>
            <button
              data-testid="queue-explicit-node-intent"
              disabled={!nodeDraftValid || draftReviewBusy}
              aria-describedby="viewport-node-add-reason"
              onClick={() => void addExplicitNodeIntent()}
              title={nodeAddReason}
              type="button"
            >
              <CirclePlus size={15} aria-hidden="true" />
              Add node
            </button>
            <small id="viewport-node-add-reason" data-testid="viewport-node-add-reason">{nodeAddReason}</small>
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
            <fieldset className="viewport-end-mode" data-testid="viewport-route-end-mode">
              <legend>End mode</legend>
              <label>
                <input type="radio" name="route-end-mode" value="existing" checked={routeEndMode === "existing"} onChange={() => { clearPointerPlacement(); invalidateDraftReview(); setRouteEndMode("existing"); }} />
                Existing node
              </label>
              <label>
                <input type="radio" name="route-end-mode" value="new" checked={routeEndMode === "new"} onChange={() => { clearPointerPlacement(); invalidateDraftReview(); setRouteEndMode("new"); setPipeEndpointPickMode(null); }} />
                New node
              </label>
            </fieldset>
            <div className="viewport-endpoint-field" hidden={routeEndMode !== "existing"}>
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
            <div className="viewport-new-endpoint" hidden={routeEndMode !== "new"} data-testid="viewport-route-new-endpoint">
              <strong>New endpoint</strong>
              <small>Typed coordinates are authoritative. Pointer placement is only a draft aid.</small>
              <fieldset
                className="viewport-routing-aids"
                data-testid="viewport-routing-aids"
                disabled={routingControlsDisabled}
                aria-describedby="viewport-routing-control-reason"
                title={routingControlReason}
              >
                <legend>Pointer routing aids</legend>
                <label>
                  <span>Plane</span>
                  <select
                    aria-label="Route construction plane"
                    data-testid="viewport-routing-plane"
                    value={routingPlane}
                    onChange={(event) => changeRoutingPlane(event.target.value as RoutingPlane)}
                    title={routingControlReason}
                  >
                    {(["XY", "XZ", "YZ"] as const).map((plane) => <option key={plane} value={plane}>{plane}</option>)}
                  </select>
                </label>
                <div className="viewport-routing-axis" role="group" aria-label="Route axis constraint">
                  {(["Free", "X", "Y", "Z"] as const).map((axis) => {
                    const applicable = applicableRoutingAxes(routingPlane).includes(axis);
                    const reason = applicable
                      ? `${axis} is available in the ${routingPlane} plane.`
                      : `${axis} is unavailable because it does not lie in the ${routingPlane} plane.`;
                    return <label key={axis} title={reason}>
                      <input
                        type="radio"
                        name="route-axis-constraint"
                        value={axis}
                        checked={routingAxis === axis}
                        disabled={!applicable}
                        onChange={() => changeRoutingAxis(axis)}
                        title={reason}
                      />
                      {axis}
                    </label>;
                  })}
                </div>
              </fieldset>
              <small id="viewport-routing-control-reason" data-testid="viewport-routing-control-reason">{routingControlReason}</small>
              <small data-testid="viewport-pointer-placement-status" title={pointerCaptureReason}>{placementMessage ?? pointerCaptureReason}</small>
              <label><span>Node ID</span><input aria-label="Route end node ID" data-testid="viewport-route-end-id" value={newEndDraft.id} onChange={(event) => updateNewEndDraft("id", event.target.value)} /></label>
              <label><span>Label</span><input aria-label="Route end node label" data-testid="viewport-route-end-label" value={newEndDraft.label} onChange={(event) => updateNewEndDraft("label", event.target.value)} /></label>
              <label><span>X</span><input aria-label="Route end X coordinate" data-testid="viewport-route-end-x" inputMode="decimal" value={newEndDraft.x} onChange={(event) => updateNewEndDraft("x", event.target.value)} /></label>
              <label><span>Y</span><input aria-label="Route end Y coordinate" data-testid="viewport-route-end-y" inputMode="decimal" value={newEndDraft.y} onChange={(event) => updateNewEndDraft("y", event.target.value)} /></label>
              <label><span>Z</span><input aria-label="Route end Z coordinate" data-testid="viewport-route-end-z" inputMode="decimal" value={newEndDraft.z} onChange={(event) => updateNewEndDraft("z", event.target.value)} /></label>
              <label><span>Coordinate unit</span><select aria-label="Route end coordinate unit" data-testid="viewport-route-end-unit" value={newEndDraft.coordinateUnit} onChange={(event) => updateNewEndDraft("coordinateUnit", event.target.value)}>{nodeLengthUnitOptions.map((option) => <option key={option.symbol} value={option.symbol}>{option.symbol}</option>)}</select></label>
              <label><span>Provenance</span><input aria-label="Route end provenance" data-testid="viewport-route-end-provenance" value={newEndDraft.provenance} onChange={(event) => updateNewEndDraft("provenance", event.target.value)} /></label>
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
            <small data-testid="viewport-construction-plane">{routeEndMode === "new" ? constructionPlaneReadout : "Construction plane inactive: existing endpoint uses exact node IDs."}</small>
            <small data-testid="viewport-route-ghost-status">
              {routeGhost
                ? `${routeGhost.provenance} route ghost: ${formatDraftCoordinate(routeGhost.from.x)}, ${formatDraftCoordinate(routeGhost.from.y)}, ${formatDraftCoordinate(routeGhost.from.z)} → ${formatDraftCoordinate(routeGhost.to.x)}, ${formatDraftCoordinate(routeGhost.to.y)}, ${formatDraftCoordinate(routeGhost.to.z)} ${defaultLengthUnit}`
                : "No route ghost is visible."}
            </small>
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
            <label>
              <input type="checkbox" data-testid="continue-pipe-after-queue" checked={continuePipe} onChange={(event) => { invalidateDraftReview(); setContinuePipe(event.target.checked); }} />
              Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
            </label>
            <button
              type="button"
              data-testid="cancel-pipe-draft"
              disabled={!canCancelPipeDraft}
              title={canCancelPipeDraft ? "Clear the pipe draft and stop continuation" : "No pipe draft to cancel"}
              onClick={cancelPipeDraft}
            >
              Cancel pipe draft
            </button>
            <button
              data-testid="queue-explicit-pipe-intent"
              disabled={!pipeDraftValid || draftReviewBusy}
              aria-describedby="viewport-route-add-reason"
              onClick={() => void addExplicitPipeIntent()}
              title={routeAddReason}
              type="button"
            >
              <GitBranch size={15} aria-hidden="true" />
              Add route
            </button>
            <small id="viewport-route-add-reason" data-testid="viewport-route-add-reason">{routeAddReason}</small>
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
            {componentDraft.kind === "expansion_joint" ? (
              <>
                <ViewportComponentTextInput label="Effective area" testId="viewport-create-component-effective-area" value={componentDraft.effectiveArea} onChange={(value) => updateComponentDraft("effectiveArea", value)} />
                <ViewportComponentUnitSelect label="Effective area unit" testId="viewport-create-component-area-unit" value={componentDraft.areaUnit} options={componentAreaUnitOptions} onChange={(value) => updateComponentDraft("areaUnit", value)} />
                <ViewportComponentTextInput label="Movement limit" testId="viewport-create-component-movement-limit" value={componentDraft.movementLimit} onChange={(value) => updateComponentDraft("movementLimit", value)} />
                <ViewportComponentUnitSelect label="Movement limit unit" testId="viewport-create-component-movement-unit" value={componentDraft.movementUnit} options={componentMovementUnitOptions} onChange={(value) => updateComponentDraft("movementUnit", value)} />
                <ViewportComponentTextInput label="Hardware reference" testId="viewport-create-component-hardware-ref" value={componentDraft.hardwareReference} onChange={(value) => updateComponentDraft("hardwareReference", value)} />
                <ViewportComponentTextInput label="Manufacturer reference" testId="viewport-create-component-manufacturer-ref" value={componentDraft.manufacturerReference} onChange={(value) => updateComponentDraft("manufacturerReference", value)} />
                <ViewportComponentTextInput label="Pressure thrust reference" testId="viewport-create-component-pressure-thrust-ref" value={componentDraft.pressureThrustReference} onChange={(value) => updateComponentDraft("pressureThrustReference", value)} />
                <ViewportComponentTextInput label="Axial stiffness" testId="viewport-create-component-axial-stiffness" value={componentDraft.axialStiffness} onChange={(value) => updateComponentDraft("axialStiffness", value)} />
                <ViewportComponentTextInput label="Lateral stiffness" testId="viewport-create-component-lateral-stiffness" value={componentDraft.lateralStiffness} onChange={(value) => updateComponentDraft("lateralStiffness", value)} />
                <ViewportComponentUnitSelect label="Linear stiffness unit" testId="viewport-create-component-linear-stiffness-unit" value={componentDraft.linearStiffnessUnit} options={componentLinearStiffnessUnitOptions} onChange={(value) => updateComponentDraft("linearStiffnessUnit", value)} />
                <ViewportComponentTextInput label="Angular stiffness" testId="viewport-create-component-angular-stiffness" value={componentDraft.angularStiffness} onChange={(value) => updateComponentDraft("angularStiffness", value)} />
                <ViewportComponentTextInput label="Torsional stiffness" testId="viewport-create-component-torsional-stiffness" value={componentDraft.torsionalStiffness} onChange={(value) => updateComponentDraft("torsionalStiffness", value)} />
                <ViewportComponentUnitSelect label="Rotational stiffness unit" testId="viewport-create-component-rotational-stiffness-unit" value={componentDraft.rotationalStiffnessUnit} options={componentRotationalStiffnessUnitOptions} onChange={(value) => updateComponentDraft("rotationalStiffnessUnit", value)} />
                <ViewportComponentTextInput label="Stiffness source reference" testId="viewport-create-component-stiffness-source" value={componentDraft.stiffnessSourceReference} onChange={(value) => updateComponentDraft("stiffnessSourceReference", value)} />
              </>
            ) : null}
            {componentDraft.kind !== "bend" && componentDraft.kind !== "tee" && componentDraft.kind !== "expansion_joint" ? (
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
            <label>
              <span>Provenance</span>
              <input
                aria-label="New component provenance"
                data-testid="viewport-create-component-provenance"
                onChange={(event) => updateComponentDraft("provenance", event.target.value)}
                value={componentDraft.provenance}
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
          {onAddDraft ? (
            <section className="viewport-draft-review" aria-label="Route review" data-testid="viewport-draft-review">
              <h4>Review and Apply</h4>
              {draftReviewMessage ? <p role="status" data-testid="viewport-draft-review-message">{draftReviewMessage}</p> : null}
              {draftReview ? <DraftReviewPreview review={draftReview} /> : <p className="muted">Add a complete node or route to generate the service validation and exact diff.</p>}
              <button
                type="button"
                data-testid="apply-reviewed-draft"
                disabled={!draftReview || draftReviewBusy || !onApplyDraft}
                onClick={() => void applyDraftReview()}
                title={
                  draftReviewBusy
                    ? "Wait for the current draft request to finish."
                    : !draftReview
                      ? "Add and review a draft before applying."
                      : !onApplyDraft
                        ? "Applying drafts is unavailable."
                        : undefined
                }
              >
                Apply
              </button>
            </section>
          ) : null}
          <details className="viewport-technical-details"><summary>Unit source</summary><small data-testid="viewport-unit-catalog-status">
            {unitCatalogRoute?.route === "tauri_unit_catalog"
              ? `DEC-018 unit catalog loaded; entries=${unitCatalogRoute.catalog.entry_count}`
              : "browser preview uses model metadata for viewport length units"}
          </small></details>
        </fieldset>
        <details className="viewport-technical-details"><summary>Pending changes ({visibleIntents.length})</summary>
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
        </div></details>
      </section>
    </div>
  );
}

function DraftReviewPreview({ review }: { review: FrozenDraftReview }) {
  const operationIds = submissionOperationIds(review.submission);
  const diffs = "operation_outcomes" in review.outcome
    ? review.outcome.operation_outcomes.flatMap((step) => step.diff_preview)
    : review.outcome.diff_preview;
  return (
    <div data-testid="viewport-draft-review-preview">
      <p><strong>{review.submission.kind === "batch" ? "Atomic batch" : "Single operation"}</strong>: {operationIds.join(" → ")}</p>
      <p>Validated model hash: {review.basisHash.value}</p>
      <ul>{diffs.map((diff, index) => <li key={`${index}:${diff.entity_ref}:${diff.field_path}`}>{diff.entity_ref} {diff.field_path}: {diff.before} → {diff.after} [{diff.unit}]</li>)}</ul>
    </div>
  );
}

function routeDraftFromState(pipe: PipeDraft, endMode: RouteEndMode, newEnd: NodeDraft): RouteDraft {
  return {
    startNodeId: pipe.from,
    endMode,
    existingEndNodeId: pipe.to,
    newEnd,
    pipe: {
      id: pipe.id,
      label: pipe.label,
      materialId: pipe.material,
      outsideDiameter: pipe.outsideDiameter,
      wallThickness: pipe.wallThickness,
      lengthUnit: pipe.lengthUnit,
      yReferenceX: pipe.yReferenceX,
      yReferenceY: pipe.yReferenceY,
      yReferenceZ: pipe.yReferenceZ,
      provenance: pipe.provenance
    }
  };
}

function emptyNodeDraft(lengthUnit: string): NodeDraft {
  return { id: "", label: "", coordinateUnit: lengthUnit, x: "", y: "", z: "", provenance: "" };
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
  if (tool === "component") return "Component tool armed: complete geometry and connectivity, then queue for validation, review and Apply.";
  if (tool === "load") return "Load tool armed: use the Load Cases panel to create load cases and primitive loads.";
  return "Model focus";
}

function fitViewportCamera(resource: ViewportResource, preset: ViewPreset, bounds: Bounds3 | null) {
  const center = bounds
    ? {
        x: (bounds.min.x + bounds.max.x) / 2,
        y: (bounds.min.y + bounds.max.y) / 2,
        z: (bounds.min.z + bounds.max.z) / 2
      }
    : { x: 0, y: 0, z: 0 };
  const extent = bounds
    ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1)
    : 4;
  const distance = Math.max(4, extent * 1.8);
  if (preset === "front") {
    resource.camera.position.set(center.x, center.y, center.z + distance);
  } else if (preset === "top") {
    resource.camera.position.set(center.x, center.y + distance, center.z);
  } else {
    const diagonal = distance / Math.sqrt(3);
    resource.camera.position.set(center.x + diagonal, center.y + diagonal, center.z + diagonal);
  }
  resource.controls.target.set(center.x, center.y, center.z);
  resource.camera.near = Math.max(0.001, distance / 100_000);
  resource.camera.far = Math.max(1_000, distance + extent * 8);
  resource.camera.lookAt(center.x, center.y, center.z);
  resource.camera.updateProjectionMatrix();
  resource.controls.update();
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
    provenance: ""
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
  if (draft.kind === "expansion_joint") {
    const area = unitDimensionValidationStatus(route, draft.areaUnit, "area");
    const movement = unitDimensionValidationStatus(route, draft.movementUnit, "length");
    const linearStiffness = unitDimensionValidationStatus(route, draft.linearStiffnessUnit, "linear_stiffness");
    const rotationalStiffness = unitDimensionValidationStatus(
      route,
      draft.rotationalStiffnessUnit,
      "rotational_stiffness"
    );
    return `area=${area}; movement=${movement}; linear_stiffness=${linearStiffness}; rotational_stiffness=${rotationalStiffness}`;
  }
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
  const identity = nextViewportNodeIdentity(model, queuedIntents);
  return {
    id: identity.id,
    label: identity.label,
    coordinateUnit,
    x: formatDraftCoordinate(point.x),
    y: formatDraftCoordinate(point.y),
    z: formatDraftCoordinate(point.z),
    provenance: ""
  };
}

function nextViewportNodeIdentity(
  model: PreviewModel,
  queuedIntents: EditorOperationIntent[]
): { id: string; label: string } {
  const id = nextViewportNodeId(model, queuedIntents);
  return { id, label: `Viewport node ${shortEntityToken(id)}` };
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
  camera: THREE.PerspectiveCamera,
  definition: RoutingPlaneDefinition
): Vec3 | null {
  const position = eventPositionFraction(canvas, event);
  const pointer = new THREE.Vector2(position.x * 2 - 1, -(position.y * 2 - 1));
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(pointer, camera);
  const normal = new THREE.Vector3(definition.normal.x, definition.normal.y, definition.normal.z);
  const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(
    normal,
    new THREE.Vector3(definition.anchor.x, definition.anchor.y, definition.anchor.z)
  );
  const intersection = new THREE.Vector3();
  if (!raycaster.ray.intersectPlane(plane, intersection)) return null;
  if (![intersection.x, intersection.y, intersection.z].every(Number.isFinite)) return null;
  return { x: intersection.x, y: intersection.y, z: intersection.z };
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

function viewportSelectionTargets(index: ModelIndex, keys: readonly EntityKey[]): ViewportSelectionTarget[] {
  const rawTargets = keys.flatMap((key) => {
    const entity = index.entities.get(key);
    if (!entity?.anchor || !["node", "pipe", "support", "component"].includes(entity.ref.type)) return [];
    const kind = entity.ref.type as ViewportSelectionTarget["kind"];
    return [{
      ref: entity.ref,
      label: entity.label,
      kind,
      position: entity.anchor as Vec3,
      offsetY: kind === "support" ? 8 : kind === "component" ? -8 : 0
    }];
  });
  const bounds = selectionBounds(rawTargets.map((target) => target.position));
  return rawTargets.map(({ position, offsetY, ...target }) => ({
    ...target,
    screen: projectToViewport(position, bounds, offsetY)
  }));
}

function selectionContainsKey(keys: readonly EntityKey[], ref: EntityRef): boolean {
  return keys.includes(entityKey(ref));
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

function instancedPipeMesh(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  selectedKeys: readonly EntityKey[]
): THREE.InstancedMesh | null {
  const valid = model.pipe_segments.flatMap((pipe) => {
    const from = nodes.get(pipe.from);
    const to = nodes.get(pipe.to);
    if (!from || !to) return [];
    const direction = new THREE.Vector3(to.x - from.x, to.y - from.y, to.z - from.z);
    const length = direction.length();
    return Number.isFinite(length) && length > 0 ? [{ pipe, from, to, direction, length }] : [];
  });
  if (valid.length === 0) return null;
  const geometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false);
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff, metalness: 0.2, roughness: 0.58 });
  const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
  const matrix = new THREE.Matrix4();
  const quaternion = new THREE.Quaternion();
  const up = new THREE.Vector3(0, 1, 0);
  const scale = new THREE.Vector3();
  const center = new THREE.Vector3();
  valid.forEach(({ pipe, from, to, direction, length }, index) => {
    quaternion.setFromUnitVectors(up, direction.normalize());
    center.set((from.x + to.x) / 2, (from.y + to.y) / 2, (from.z + to.z) / 2);
    scale.set(0.052, length, 0.052);
    matrix.compose(center, quaternion, scale);
    mesh.setMatrixAt(index, matrix);
    mesh.setColorAt(index, new THREE.Color(
      selectionContainsKey(selectedKeys, { type: "pipe", id: pipe.id }) ? 0xf08c22 : 0x4f6f73
    ));
  });
  mesh.instanceMatrix.needsUpdate = true;
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
  mesh.userData.instanceEntityKeys = valid.map(({ pipe }) => entityKey({ type: "pipe", id: pipe.id }));
  return mesh;
}

function instancedNodeMesh(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  selectedKeys: readonly EntityKey[]
): THREE.InstancedMesh | null {
  const valid = model.nodes.filter((node) => nodes.has(node.id));
  if (valid.length === 0) return null;
  const geometry = new THREE.SphereGeometry(0.095, 12, 8);
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.48 });
  const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
  const matrix = new THREE.Matrix4();
  valid.forEach((node, index) => {
    const position = nodes.get(node.id)!;
    matrix.makeTranslation(position.x, position.y, position.z);
    mesh.setMatrixAt(index, matrix);
    mesh.setColorAt(index, new THREE.Color(
      selectionContainsKey(selectedKeys, { type: "node", id: node.id }) ? 0xf08c22 : 0x2f6f73
    ));
  });
  mesh.instanceMatrix.needsUpdate = true;
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
  mesh.userData.instanceEntityKeys = valid.map((node) => entityKey({ type: "node", id: node.id }));
  return mesh;
}

function instancedSupportMesh(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  selectedKeys: readonly EntityKey[]
): THREE.InstancedMesh | null {
  const valid = model.supports.flatMap((support) => {
    const node = nodes.get(support.node);
    return node ? [{ support, node }] : [];
  });
  if (valid.length === 0) return null;
  const geometry = new THREE.ConeGeometry(0.18, 0.34, 4);
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.7 });
  const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
  const matrix = new THREE.Matrix4();
  const quaternion = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 1, 0), Math.PI / 4);
  const scale = new THREE.Vector3(1, 1, 1);
  valid.forEach(({ support, node }, index) => {
    matrix.compose(new THREE.Vector3(node.x, node.y - 0.26, node.z), quaternion, scale);
    mesh.setMatrixAt(index, matrix);
    mesh.setColorAt(index, new THREE.Color(
      selectionContainsKey(selectedKeys, { type: "support", id: support.id }) ? 0xf08c22 : 0x6b7d49
    ));
  });
  mesh.instanceMatrix.needsUpdate = true;
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
  mesh.userData.instanceEntityKeys = valid.map(({ support }) => entityKey({ type: "support", id: support.id }));
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

function routeConstructionGridFromBounds(bounds: Bounds3 | null): THREE.GridHelper {
  const span = bounds
    ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z)
    : 0;
  const size = Math.max(8, Math.ceil(span * 1.5));
  const grid = new THREE.GridHelper(size, Math.max(8, Math.min(40, Math.round(size * 2))), 0x2f6f73, 0x9bb7b4);
  const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
  for (const material of materials) {
    material.transparent = true;
    material.opacity = 0.44;
    material.depthWrite = false;
  }
  grid.renderOrder = 1;
  return grid;
}

function routeConstructionGrid(model: PreviewModel): THREE.GridHelper {
  const positions = model.nodes.map((node) => node.position);
  const coordinates = positions.flatMap((point) => [point.x, point.y, point.z]);
  const span = coordinates.length ? Math.max(...coordinates) - Math.min(...coordinates) : 0;
  const size = Math.max(8, Math.ceil(span * 1.5));
  const grid = new THREE.GridHelper(size, Math.max(8, Math.min(40, size * 2)), 0x2f6f73, 0x9bb7b4);
  const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
  for (const material of materials) {
    material.transparent = true;
    material.opacity = 0.44;
    material.depthWrite = false;
  }
  grid.renderOrder = 1;
  return grid;
}

function updateRouteConstructionGrid(
  grid: THREE.GridHelper,
  anchor: Vec3 | null,
  plane: RoutingPlane,
  visible: boolean
) {
  grid.visible = Boolean(anchor) && visible;
  if (!anchor) return;
  grid.position.set(anchor.x, anchor.y, anchor.z);
  grid.rotation.set(0, 0, 0);
  if (plane === "XY") grid.rotation.x = Math.PI / 2;
  if (plane === "YZ") grid.rotation.z = Math.PI / 2;
}

function routeGhostLine(): THREE.Line<THREE.BufferGeometry, THREE.LineDashedMaterial> {
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute([0, 0, 0, 0, 0, 0], 3));
  const line = new THREE.Line(
    geometry,
    new THREE.LineDashedMaterial({ color: 0xf08c22, dashSize: 0.18, gapSize: 0.1, depthTest: false })
  );
  line.renderOrder = 3;
  return line;
}

function updateRouteGhostObjects(
  line: THREE.Line<THREE.BufferGeometry, THREE.LineDashedMaterial>,
  endpoint: THREE.Mesh,
  ghost: RouteGhost | null
) {
  line.visible = Boolean(ghost);
  endpoint.visible = Boolean(ghost);
  if (!ghost) return;
  const position = line.geometry.getAttribute("position") as THREE.BufferAttribute;
  position.setXYZ(0, ghost.from.x, ghost.from.y, ghost.from.z);
  position.setXYZ(1, ghost.to.x, ghost.to.y, ghost.to.z);
  position.needsUpdate = true;
  line.computeLineDistances();
  line.geometry.computeBoundingSphere();
  endpoint.position.set(ghost.to.x, ghost.to.y, ghost.to.z);
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

function referenceGroundFromBounds(bounds: Bounds3 | null): THREE.GridHelper {
  const minX = bounds?.min.x ?? 0;
  const maxX = bounds?.max.x ?? 1;
  const minY = bounds?.min.y ?? 0;
  const minZ = bounds?.min.z ?? 0;
  const maxZ = bounds?.max.z ?? 1;
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
    maximum: { value: maxValue, unit },
    nodeCount: nodeValues.size,
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

function finitePointerEventNumber(value: number, fallback: number): number {
  return Number.isFinite(value) ? value : fallback;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
