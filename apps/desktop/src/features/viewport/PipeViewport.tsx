import { CompactSelect } from "../workspace/CompactSelect";
import { SelectionPresentationBinding } from "./selectionPresentationBinding";
import { QuantityReadout, useDisplayQuantity } from "../display-units";
import { Box, CircleDot, CirclePlus, GitBranch, MoveDown, Anchor } from "lucide-react";
import { type CSSProperties, type PointerEvent as ReactPointerEvent, type ReactNode, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
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
import {
  convertedSectionEnvelopeIsConsistent,
  modelIndexFor,
  type Bounds3,
  type ModelIndex
} from "../workspace/modelIndex";
import {
  entityKey,
  entityRefFromKey,
  type EntityKey,
  type OrderedSelectionState
} from "../workspace/selectionState";
import {
  authoredToLocal,
  applyMeasurementTargetCommand,
  boxSelectEntityKeys,
  boxGestureContextIsCurrent,
  claimBoxSelectionPointerDown,
  createRenderTransform,
  displayedBoundsForEntityKeys,
  formatMeasurementDisplayValue,
  fittedViewportDistance,
  hideSelectionVisibility,
  isolateSelectionVisibility,
  pointPickPrimitives,
  prioritizedLabelKeys,
  spatialEntityKeyGroups,
  visibilityEligibleSelectionKeys,
  type BoxGestureContext,
  type BoxSelectionDirection,
  type BoxSelectionFilter,
  type MeasurementSource,
  type ViewportViewCommand
} from "./viewportSelection";
import {
  registerGridPaletteRoles,
  registerInstancedRolePresentation,
  registerPaletteRole,
  ViewportResource,
  type ViewportContextStatus,
  type ViewportRendererInfo
} from "./viewportResource";
import { createFigureMaterial, figureEdgeOutlineFor } from "./viewportFigureMaterial";
import type { ViewportPaletteRole } from "./viewportPalette";
import {
  clearUiDiagnosticsPublisher,
  publishUiDiagnostics,
  refreshUiDiagnostics,
  type UiDiagnosticsPublication
} from "../workspace/uiDiagnostics";
import { VirtualTargetPicker } from "../workspace/VirtualTargetPicker";
import { statusDisplayWithToken } from "../workspace/statusLabels";

function ViewportMeasurementQuantityReadout({ label, unit, value }: { label: string; unit: string; value: number }) {
  const display = useDisplayQuantity({ value, unit, dimension_id: "length" });
  const rawValue = String(display.value);
  const presentedValue = typeof display.value === "number"
    ? formatMeasurementDisplayValue(display.value)
    : rawValue;
  const exactLabel = `${label}: ${presentedValue} ${display.unit}; full-precision numeric value ${rawValue} ${display.unit}.`;
  return (
    <span
      aria-label={exactLabel}
      data-display-status={display.status}
      data-measurement-label={label}
      data-raw-unit={display.unit}
      data-raw-value={rawValue}
      title={`${label} full precision: ${rawValue} ${display.unit}`}
    >
      {label} {presentedValue} {display.unit}
      {display.notice ? <small role="status"> ({display.notice})</small> : null}
    </span>
  );
}

type Props = {
  /** Optional shell-owned home for the existing authoring panel. */
  authoringPanelContainer?: HTMLElement | null;
  armedCreationTool?: CreationTool | null;
  assignment?: {
    status: "started" | "committed";
    generation: number;
    indexGeneration: string;
    identityHash: string;
    startedAt: number;
    committedAt: number | null;
  } | null;
  model: PreviewModel;
  modelIdentityHash?: string | null;
  hiddenKeys?: ReadonlySet<EntityKey>;
  explicitHiddenKeys?: ReadonlySet<EntityKey>;
  isolateHiddenKeys?: ReadonlySet<EntityKey>;
  modelIndex?: ModelIndex;
  modelCommitToken?: string | null;
  viewCommandRef?: { current: ((command: ViewportViewCommand) => void) | null };
  onArmCreationTool?: (tool: CreationTool | null) => void;
  onAddDraft?: (submission: DraftSubmission, generation: number) => Promise<FrozenDraftReview | null>;
  onApplyDraft?: (review: FrozenDraftReview) => Promise<boolean>;
  onInvalidateDraft?: () => void;
  onHiddenKeysChange?: (keys: ReadonlySet<EntityKey>) => void;
  onIsolateKeysChange?: (keys: ReadonlySet<EntityKey>) => void;
  onClearVisibility?: () => void;
  onViewportInteractionStart?: (interaction: ViewportExposureInteraction) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onBoxSelection?: (
    keys: readonly EntityKey[],
    modifiers: { additive?: boolean; toggle?: boolean },
    detail: { direction: BoxSelectionDirection; filter: BoxSelectionFilter; pointerDownAt: number }
  ) => OrderedSelectionState;
  onSelect: (selection: EntityRef, modifiers?: { additive?: boolean; toggle?: boolean }) => OrderedSelectionState;
  queuedIntents?: EditorOperationIntent[];
  reservedIntents?: ReadonlyArray<unknown>;
  result?: MechanicsResult | null;
  selection: EntityRef;
  selectionState?: OrderedSelectionState;
  /** Presentation-only clearance below viewport furniture and the painted orientation frame. */
  presentationBottomInsetPx?: number;
  theme?: "light" | "dark";
  treePublication?: { actionSequence: number; publicationSequence: number; query: string; visibleCount: number; inputAt: number | null; inputEventTimeStamp: number | null; publishedAt: number } | null;
};

function OptionalPortal({ children, container }: { children: ReactNode; container?: HTMLElement | null }) {
  return container ? createPortal(children, container) : children;
}

function normalizedPresentationInset(value: number | undefined): number {
  return typeof value === "number" && Number.isFinite(value) ? Math.max(0, Math.round(value)) : 0;
}

export type CreationTool = "node" | "pipe" | "support" | "component" | "load";
export type ViewportExposureInteraction = "measurement" | "box-selection";
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

type ActiveViewportDiagnostics = Exclude<UiDiagnosticsPublication["viewport"], { status: "unavailable" }>;
type BoxPublication = Readonly<{
  actionSequence: number;
  direction: BoxSelectionDirection;
  filter: BoxSelectionFilter;
  orderedKeys: readonly EntityKey[];
  primaryKey: EntityKey | null;
  pointerDownAt: number;
  publishedAt: number;
  afterSubmissionSequence: number;
  renderSubmissionSequence: number;
}>;

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
      <CompactSelect
        aria-label={`New component ${label.toLowerCase()}`}
        data-testid={testId}
        onValueChange={(value) => onChange(value)}
        value={value}
        options={[...(value === "" ? [{ value: "", label: "Select unit" }] : []), ...options.map((option) => ({ value: option.symbol, label: option.symbol }))]}
      />
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
  authoringPanelContainer = null,
  armedCreationTool = null,
  assignment = null,
  model,
  modelIdentityHash = null,
  hiddenKeys = new Set(),
  explicitHiddenKeys = hiddenKeys,
  isolateHiddenKeys = new Set(),
  modelIndex,
  modelCommitToken = null,
  viewCommandRef,
  onArmCreationTool = () => {},
  onAddDraft,
  onApplyDraft,
  onInvalidateDraft = () => {},
  onHiddenKeysChange = () => {},
  onIsolateKeysChange = () => {},
  onClearVisibility = () => {},
  onViewportInteractionStart = () => {},
  onQueueIntent,
  onBoxSelection = () => Object.freeze({ orderedKeys: [], primaryKey: null, rangeAnchorKey: null, focusKey: null, preparationEpoch: 0 }),
  onSelect,
  queuedIntents = [],
  reservedIntents = [],
  result = null,
  selection,
  selectionState,
  presentationBottomInsetPx = 0,
  theme = "light",
  treePublication = null
}: Props) {
  const presentationBottomInset = normalizedPresentationInset(presentationBottomInsetPx);
  const hostRef = useRef<HTMLDivElement | null>(null);
  const viewportResourceRef = useRef<ViewportResource | null>(null);
  const selectionPresentationBindingRef = useRef(new SelectionPresentationBinding());
  const mainFrameRef = useRef({
    generation: null as number | null,
    sequence: 0,
    submittedAt: null as number | null,
    opportunityAt: null as number | null,
    rendererInfo: null as ViewportRendererInfo | null
  });
  const diagnosticsSelectionRef = useRef<{
    actionSequence: number;
    generation: number;
    selection: ActiveViewportDiagnostics["selection"];
    inspector: ActiveViewportDiagnostics["inspector"];
  } | null>(null);
  const diagnosticsBoxRef = useRef<{
    generation: number;
    publication: BoxPublication;
    box: Exclude<ActiveViewportDiagnostics["box"], { status: "unavailable" }>;
  } | null>(null);
  const diagnosticsInspectorPublicationSequenceRef = useRef(0);
  const diagnosticsPublisherRegisteredRef = useRef(false);
  const draftProjectorRef = useRef<DraftProjector | null>(null);
  const cameraStateRef = useRef<{
    position: [number, number, number];
    target: [number, number, number];
  } | null>(null);
  const lastPresetRef = useRef<ViewPreset | null>(null);
  const fittedSessionGenerationRef = useRef<number | null>(null);
  const pickRef = useRef<((event: { clientX: number; clientY: number }) => EntityRef | null) | null>(null);
  const pointerGestureRef = useRef<{ gesture: PointerGesture; target: Element; pointerDownAt: number } | null>(null);
  const boxGestureRef = useRef<{
    context: BoxGestureContext;
    pointerId: number;
    startX: number;
    startY: number;
    currentX: number;
    currentY: number;
    pointerDownAt: number;
    filter: BoxSelectionFilter;
    modifiers: { additive?: boolean; toggle?: boolean };
    target: Element;
  } | null>(null);
  const boxActionSequenceRef = useRef(0);
  const selectionActionSequenceRef = useRef(0);
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
  const [boxSelectActive, setBoxSelectActive] = useState(false);
  const [boxSelectionFilter, setBoxSelectionFilter] = useState<BoxSelectionFilter>("all");
  const [boxRect, setBoxRect] = useState<{ left: number; top: number; right: number; bottom: number } | null>(null);
  const [boxPublication, setBoxPublication] = useState<BoxPublication | null>(null);
  const [measurementActive, setMeasurementActive] = useState(false);
  const [measurementSource, setMeasurementSource] = useState<MeasurementSource | null>(null);
  const measurementGateRef = useRef(0);
  const [measurementReadout, setMeasurementReadout] = useState<{
    status: "idle" | "pending" | "available" | "unavailable";
    values: ReadonlyMap<string, number>;
    reason: string;
  }>({ status: "idle", values: new Map(), reason: "Choose two authored nodes or one authored pipe." });
  const [geometryMode, setGeometryMode] = useState<"schematic" | "actual-od">("schematic");
  const [viewCommandStatus, setViewCommandStatus] = useState("No view command dispatched.");
  const [odGeneration, setOdGeneration] = useState(0);
  const odGenerationRef = useRef(0);
  const odRequestGateRef = useRef(0);
  const actualOdCacheRef = useRef<{
    key: string;
    value: { radii: ReadonlyMap<EntityKey, number>; status: "available" | "partial-unavailable" | "unavailable"; reason: string };
  } | null>(null);
  const [actualOd, setActualOd] = useState<{
    radii: ReadonlyMap<EntityKey, number>;
    status: "not-requested" | "pending" | "available" | "partial-unavailable" | "unavailable";
    reason: string;
  }>({ radii: new Map(), status: "not-requested", reason: "Actual OD has not been requested." });
  const activeModelIndex = useMemo(
    () => modelIndex ?? modelIndexFor(model, 0, 0),
    [model, modelIndex]
  );
  const renderTransformResolution = useMemo(() => {
    try {
      return { transform: createRenderTransform(activeModelIndex.geometryBounds), issue: null as string | null };
    } catch (error: unknown) {
      return {
        transform: createRenderTransform(null),
        issue: error instanceof Error ? error.message : "Viewport geometry cannot be represented safely."
      };
    }
  }, [activeModelIndex]);
  const renderTransform = renderTransformResolution.transform;
  const orderedSelectionKeys = useMemo(
    () => selectionState?.orderedKeys ?? Object.freeze([entityKey(selection)]),
    [selection.id, selection.type, selectionState]
  );
  const orderedSelectionKeySet = useMemo(
    () => new Set(orderedSelectionKeys),
    [orderedSelectionKeys]
  );
  const visibilitySelectionKeys = useMemo(
    () => visibilityEligibleSelectionKeys(activeModelIndex, orderedSelectionKeys),
    [activeModelIndex, orderedSelectionKeys]
  );
  const visibilitySelectionReason = visibilitySelectionKeys.length > 0
    ? null
    : "Select at least one visible, valid node, pipe, support, or component.";
  const [hoveredEntityKey, setHoveredEntityKey] = useState<EntityKey | null>(null);
  useEffect(() => {
    setHoveredEntityKey((current) => current && !hiddenKeys.has(current) ? current : null);
  }, [activeModelIndex.sessionGeneration, hiddenKeys]);
  const selectedHiddenCount = useMemo(
    () => orderedSelectionKeys.reduce((count, key) => count + (hiddenKeys.has(key) ? 1 : 0), 0),
    [hiddenKeys, orderedSelectionKeys]
  );
  const primarySelectionKey = selectionState?.primaryKey ?? entityKey(selection);
  // Labels retain their project fallback; explicit empty ordered selection does not.
  const orderedPrimarySelectionKey = selectionState === undefined
    ? entityKey(selection)
    : selectionState.primaryKey;
  const [selectionPublication, setSelectionPublication] = useState<{
    actionSequence: number;
    inputKind: "programmatic" | "external" | "pointer" | "label";
    pointerDownAt: number | null;
    orderedKeys: readonly EntityKey[];
    primaryKey: EntityKey | null;
    publishedAt: number;
    afterSubmissionSequence: number;
  }>(() => ({
    actionSequence: ++selectionActionSequenceRef.current,
    inputKind: "programmatic",
    pointerDownAt: null as number | null,
    orderedKeys: orderedSelectionKeys,
    primaryKey: orderedPrimarySelectionKey,
    publishedAt: performance.now(),
    afterSubmissionSequence: viewportResourceRef.current?.submissionSequence ?? 0
  }));
  useEffect(() => {
    if (selectionPublication.primaryKey === orderedPrimarySelectionKey &&
        selectionPublication.orderedKeys.length === orderedSelectionKeys.length &&
        selectionPublication.orderedKeys.every((key, index) => key === orderedSelectionKeys[index])) return;
    const publication = {
      actionSequence: ++selectionActionSequenceRef.current,
      inputKind: "external" as const,
      pointerDownAt: null,
      orderedKeys: orderedSelectionKeys,
      primaryKey: orderedPrimarySelectionKey,
      publishedAt: performance.now(),
      afterSubmissionSequence: viewportResourceRef.current?.submissionSequence ?? 0
    };
    diagnosticsStateRef.current.selectionPublication = publication;
    setSelectionPublication(publication);
  }, [orderedSelectionKeys, orderedPrimarySelectionKey, selectionPublication]);
  const labelKeys = useMemo(
    () => prioritizedLabelKeys(activeModelIndex, {
      primaryKey: primarySelectionKey,
      hoverKey: hoveredEntityKey,
      selectedKeys: orderedSelectionKeys,
      hiddenKeys
    }),
    [activeModelIndex, hiddenKeys, hoveredEntityKey, orderedSelectionKeys, primarySelectionKey]
  );
  const selectionTargets = useMemo(
    () => viewportSelectionTargets(activeModelIndex, labelKeys),
    [activeModelIndex, labelKeys]
  );
  const diagnosticsStateRef = useRef({
    assignment,
    identityHash: modelIdentityHash,
    index: activeModelIndex,
    projectId: model.project.id,
    labelsEnabled: showLabels,
    labelCount: labelKeys.length,
    boxPublication,
    geometryMode,
    odGeneration,
    odStatus: actualOd.status,
    treePublication,
    orderedSelectionKeys,
    primarySelectionKey,
    inspectorSelection: selection,
    selectionPublication
  });
  diagnosticsStateRef.current = {
    assignment,
    identityHash: modelIdentityHash,
    index: activeModelIndex,
    projectId: model.project.id,
    labelsEnabled: showLabels,
    labelCount: labelKeys.length,
    boxPublication,
    geometryMode,
    odGeneration,
    odStatus: actualOd.status,
    treePublication,
    orderedSelectionKeys,
    primarySelectionKey,
    inspectorSelection: selection,
    selectionPublication
  };
  const deformation = useMemo(() => buildDeformationOverlay(model, result), [model, result]);
  const localNodeMap = useMemo(
    () => new Map(
      model.nodes
        .filter((node) => !activeModelIndex.invalidGeometry.has(entityKey({ type: "node", id: node.id })))
        .map((node) => [node.id, authoredToLocal(node.position, renderTransform.origin)] as const)
    ),
    [activeModelIndex, model, renderTransform]
  );
  const visibleIntents = onQueueIntent ? viewportIntents(queuedIntents) : localIntents;
  const pendingViewportIntents = [...reservedIntents, ...queuedIntents, ...localIntents];
  const routeNodeMap = useMemo(
    () => new Map(model.nodes.map((node) => [node.id, node.position] as const)),
    [model]
  );
  const nodeTargetOptions = useMemo(() => model.nodes.map((node) => ({
    value: node.id,
    label: node.label || node.id
  })), [model.nodes]);
  const materialTargetOptions = useMemo(() => (model.materials ?? []).map((material) => ({
    value: material.id,
    label: material.label || material.id
  })), [model.materials]);
  const connectedPipeTargetOptions = useMemo(() => model.pipe_segments
    .filter((pipe) => pipe.from === componentDraft.node || pipe.to === componentDraft.node)
    .map((pipe) => ({ value: pipe.id, label: pipe.label || pipe.id, keywords: [pipe.from, pipe.to] })),
  [componentDraft.node, model.pipe_segments]);
  const measurementPoints = useMemo<readonly Vec3[]>(() => {
    if (!measurementSource) return [];
    if (measurementSource.kind === "nodes") {
      return measurementSource.keys.flatMap((key) => {
        const indexed = activeModelIndex.entities.get(key);
        return indexed?.ref.type === "node" && indexed.anchor && !indexed.geometryIssue ? [indexed.anchor as Vec3] : [];
      });
    }
    const indexed = activeModelIndex.entities.get(measurementSource.key);
    if (indexed?.ref.type !== "pipe" || indexed.geometryIssue) return [];
    const pipe = indexed.record as PreviewModel["pipe_segments"][number];
    const from = routeNodeMap.get(pipe.from);
    const to = routeNodeMap.get(pipe.to);
    return from && to ? [from, to] : [];
  }, [activeModelIndex, measurementSource, routeNodeMap]);
  const measurementTargetSummary = measurementSource?.kind === "pipe"
    ? `pipe: ${entityRefFromKey(measurementSource.key)?.id ?? "unavailable"}`
    : measurementSource?.kind === "nodes"
      ? measurementSource.keys.map((key) => {
          const ref = entityRefFromKey(key);
          return ref ? `${ref.type}: ${ref.id}` : "unavailable";
        }).join(" → ")
      : "No authored targets";
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
    viewportResourceRef.current?.setPresentationBottomInsetPx(presentationBottomInset);
  }, [presentationBottomInset]);

  useEffect(() => {
    if (geometryMode !== "actual-od") return;
    const cacheKey = `${activeModelIndex.generation}:${defaultLengthUnit}`;
    if (actualOdCacheRef.current?.key === cacheKey) {
      setActualOd(actualOdCacheRef.current.value);
      return;
    }
    const requestToken = ++odRequestGateRef.current;
    const generation = ++odGenerationRef.current;
    setOdGeneration(generation);
    const plan = actualOdConversionPlan(activeModelIndex, model, defaultLengthUnit);
    if (plan.requests.length === 0) {
      const value = { radii: new Map<EntityKey, number>(), status: "unavailable" as const, reason: plan.reason };
      actualOdCacheRef.current = { key: cacheKey, value };
      setActualOd(value);
      return;
    }
    setActualOd({ radii: new Map(), status: "pending", reason: "Validating pipe outside diameters through the unit service…" });
    convertDisplayQuantities(plan.requests).then((results) => {
      if (odRequestGateRef.current !== requestToken) return;
      const converted = new Map(results.flatMap((result) =>
        result.status === "converted" && result.unit === defaultLengthUnit && Number.isFinite(result.value) && result.value > 0
          ? [[result.id, result.value] as const]
          : []
      ));
      const radii = new Map<EntityKey, number>();
      let rejected = plan.invalidCount;
      for (const [key, requestIds] of plan.requestByPipe) {
        const diameter = converted.get(requestIds.outsideDiameter);
        const wall = converted.get(requestIds.wallThickness);
        const inlineDiameter = converted.get(requestIds.inlineOutsideDiameter);
        const inlineWall = converted.get(requestIds.inlineWallThickness);
        if (diameter === undefined || wall === undefined || inlineDiameter === undefined || inlineWall === undefined ||
            !convertedSectionEnvelopeIsConsistent(diameter, wall, inlineDiameter, inlineWall)) rejected += 1;
        else radii.set(key, diameter / 2);
      }
      const value = {
        radii,
        status: (radii.size === 0 ? "unavailable" : rejected > 0 ? "partial-unavailable" : "available") as "available" | "partial-unavailable" | "unavailable",
        reason: rejected > 0
          ? `${rejected} pipe span${rejected === 1 ? "" : "s"} retained centerline because OD conversion or binding validation was unavailable.`
          : `${radii.size} straight pipe span${radii.size === 1 ? "" : "s"} uses validated outside diameter.`
      };
      actualOdCacheRef.current = { key: cacheKey, value };
      setActualOd(value);
    }).catch((error: unknown) => {
      if (odRequestGateRef.current !== requestToken) return;
      const value = {
        radii: new Map(),
        status: "unavailable" as const,
        reason: error instanceof Error ? `Outside-diameter conversion unavailable: ${error.message}` : "Outside-diameter conversion unavailable."
      };
      actualOdCacheRef.current = { key: cacheKey, value };
      setActualOd(value);
    });
    return () => {
      odRequestGateRef.current += 1;
    };
  }, [activeModelIndex, defaultLengthUnit, geometryMode, model]);

  useEffect(() => {
    const requestToken = ++measurementGateRef.current;
    if (measurementPoints.length !== 2) {
      setMeasurementReadout({
        status: measurementSource ? "unavailable" : "idle",
        values: new Map(),
        reason: measurementSource
          ? "Measurement source is deleted, ambiguous, or no longer has valid authored endpoints."
          : "Choose two authored nodes or one authored pipe."
      });
      return;
    }
    const [from, to] = measurementPoints;
    const values = {
      distance: Math.hypot(to.x - from.x, to.y - from.y, to.z - from.z),
      dx: to.x - from.x,
      dy: to.y - from.y,
      dz: to.z - from.z
    };
    setMeasurementReadout({ status: "pending", values: new Map(), reason: "Converting authored measurement through the unit service…" });
    convertDisplayQuantities(Object.entries(values).map(([id, value]) => ({
      id: `viewport-measurement-${id}`,
      value,
      from_unit: defaultLengthUnit,
      to_unit: defaultLengthUnit,
      dimension_id: "length"
    }))).then((results) => {
      if (measurementGateRef.current !== requestToken) return;
      const converted = new Map(results.flatMap((result) =>
        result.status === "converted" && result.unit === defaultLengthUnit && Number.isFinite(result.value)
          ? [[result.id.replace("viewport-measurement-", ""), result.value] as const]
          : []
      ));
      if (["distance", "dx", "dy", "dz"].some((id) => !converted.has(id))) {
        setMeasurementReadout({ status: "unavailable", values: new Map(), reason: "Measurement conversion returned an incomplete or mismatched unit result." });
        return;
      }
      setMeasurementReadout({ status: "available", values: converted, reason: "View-only measurement from authored coordinates." });
    }).catch((error: unknown) => {
      if (measurementGateRef.current !== requestToken) return;
      setMeasurementReadout({ status: "unavailable", values: new Map(), reason: error instanceof Error ? error.message : "Measurement conversion unavailable." });
    });
    return () => { measurementGateRef.current += 1; };
  }, [defaultLengthUnit, measurementPoints, measurementSource]);

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
    let resource: ViewportResource;
    try {
      resource = new ViewportResource(host, {
        onContextStatus: setViewportContextStatus,
        onRestore: () => viewportResourceRef.current?.invalidate(),
        onCameraChange: (current) => {
          cameraStateRef.current = {
            position: [current.camera.position.x, current.camera.position.y, current.camera.position.z],
            target: [current.controls.target.x, current.controls.target.y, current.controls.target.z]
          };
        },
        onAfterMainFrame: (current, submittedAt, submissionSequence, rendererInfo) => {
          const diagnostics = diagnosticsStateRef.current;
          const generation = diagnostics.assignment?.status === "committed" &&
              diagnostics.assignment.indexGeneration === diagnostics.index.generation
            ? diagnostics.assignment.generation
            : null;
          selectionPresentationBindingRef.current.afterFrame(current.contextSnapshot.generation, generation, submissionSequence);
          mainFrameRef.current = {
            generation,
            sequence: submissionSequence,
            submittedAt,
            opportunityAt: null,
            rendererInfo
          };
          publishViewportDiagnostics(current);
        },
        onNextPaintOpportunity: (current, opportunityAt, submissionSequence) => {
          const frame = mainFrameRef.current;
          const diagnostics = diagnosticsStateRef.current;
          const currentAssignment = diagnostics.assignment;
          if (currentAssignment?.status !== "committed" || frame.sequence !== submissionSequence || frame.generation === null ||
              frame.generation !== currentAssignment.generation ||
              currentAssignment.indexGeneration !== diagnostics.index.generation) return;
          mainFrameRef.current = { ...frame, opportunityAt };
          publishViewportDiagnostics(current);
        },
        onOwnedRafCountChange: (current) => {
          publishViewportDiagnostics(current);
        },
        onResourceStateChange: (current, retired) => publishViewportDiagnostics(current, !retired)
      });
    } catch {
      releaseViewportDiagnosticsPublisher();
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
        releaseViewportDiagnosticsPublisher();
      };
    }
    setWebglAvailable(true);
    resource.setPresentationBottomInsetPx(presentationBottomInset);
    viewportResourceRef.current = resource;
    // Register stable lazy factories once. Product callbacks update only
    // bounded refs; diagnostics materialization happens on an explicit pull.
    publishViewportDiagnostics(resource);
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
    resource.invalidate();

    return () => {
      pickRef.current = null;
      draftProjectorRef.current = null;
      routingVisualUpdaterRef.current = null;
      viewportResourceRef.current = null;
      try {
        resource.dispose();
      } finally {
        releaseViewportDiagnosticsPublisher();
      }
    };
  }, []);

  useEffect(() => {
    selectionPresentationBindingRef.current.clear();
    mainFrameRef.current = {
      generation: assignment?.generation ?? null,
      sequence: 0,
      submittedAt: null,
      opportunityAt: null,
      rendererInfo: null
    };
  }, [activeModelIndex.generation, assignment?.generation]);

  function releaseViewportDiagnosticsPublisher(): void {
    selectionPresentationBindingRef.current.clear();
    diagnosticsPublisherRegisteredRef.current = false;
    clearUiDiagnosticsPublisher();
  }

  function publishViewportDiagnostics(resource: ViewportResource, includeProjection = true): void {
    if (!diagnosticsPublisherRegisteredRef.current) {
      diagnosticsPublisherRegisteredRef.current = true;
      publishUiDiagnostics(
      () => {
        const diagnostics = diagnosticsStateRef.current;
        const currentAssignment = diagnostics.assignment;
        if (currentAssignment?.status !== "committed" ||
            currentAssignment.indexGeneration !== diagnostics.index.generation) {
          return {
            model: {
              projectId: diagnostics.projectId,
              generation: null,
              indexGeneration: diagnostics.index.generation,
              projectSessionGeneration: diagnostics.index.sessionGeneration,
              identityHash: diagnostics.identityHash,
              assignment: { status: "idle" as const, generation: null, startedAt: null, committedAt: null }
            },
            tree: { status: "unavailable" as const },
            viewport: { status: "unavailable" as const }
          };
        }
        const generation = currentAssignment.generation;
        const now = performance.now();
        if (diagnosticsSelectionRef.current?.generation !== generation ||
            diagnosticsSelectionRef.current.actionSequence !== diagnostics.selectionPublication.actionSequence ||
            diagnosticsSelectionRef.current.inspector.ref?.type !== diagnostics.inspectorSelection.type ||
            diagnosticsSelectionRef.current.inspector.ref?.id !== diagnostics.inspectorSelection.id) {
          const orderedRefs = Object.freeze(diagnostics.selectionPublication.orderedKeys.flatMap((key) => {
            const ref = entityRefFromKey(key);
            return ref ? [Object.freeze({ type: ref.type, id: ref.id })] : [];
          }));
          const primary = diagnostics.selectionPublication.primaryKey
            ? entityRefFromKey(diagnostics.selectionPublication.primaryKey)
            : null;
          const publishedAt = now;
          diagnosticsSelectionRef.current = {
            actionSequence: diagnostics.selectionPublication.actionSequence,
            generation,
            selection: Object.freeze({
              actionSequence: diagnostics.selectionPublication.actionSequence,
              generation,
              inputKind: diagnostics.selectionPublication.inputKind,
              pointerDownAt: diagnostics.selectionPublication.pointerDownAt,
              orderedRefs,
              primaryRef: primary ? Object.freeze({ type: primary.type, id: primary.id }) : null,
              publishedAt: diagnostics.selectionPublication.publishedAt,
              renderSubmissionSequence: 0
            }),
            inspector: Object.freeze({
              generation,
              ref: Object.freeze({
                type: diagnostics.inspectorSelection.type,
                id: diagnostics.inspectorSelection.id
              }),
              publicationSequence: ++diagnosticsInspectorPublicationSequenceRef.current,
              publishedAt
            })
          };
        }
        const selectionEvidence = diagnosticsSelectionRef.current!;
        const currentFrame = mainFrameRef.current;
        const resourceGeneration = resource.contextSnapshot.generation;
        const renderedSelection = selectionPresentationBindingRef.current.current(resourceGeneration, generation, currentFrame.sequence);
        const selectionSubmission = currentFrame.generation === generation
          ? selectionPresentationBindingRef.current.qualifyingSubmission(resourceGeneration, generation, currentFrame.sequence,
              diagnostics.selectionPublication.afterSubmissionSequence, diagnostics.selectionPublication.orderedKeys)
          : 0;
        if (selectionEvidence.selection.renderSubmissionSequence !== selectionSubmission) {
          selectionEvidence.selection = Object.freeze({ ...selectionEvidence.selection, renderSubmissionSequence: selectionSubmission });
        }
        const authoredCamera = resource.localToAuthored(resource.camera.position);
        const authoredTarget = resource.localToAuthored(resource.controls.target);
        const rect = resource.renderer.domElement.getBoundingClientRect();
        const rendererInfo = currentFrame.rendererInfo ?? Object.freeze({
          geometries: 0, textures: 0, calls: 0, triangles: 0, points: 0, lines: 0
        });
        if (diagnostics.boxPublication &&
            (diagnosticsBoxRef.current?.generation !== generation ||
              diagnosticsBoxRef.current.publication !== diagnostics.boxPublication)) {
          const orderedRefs = Object.freeze(diagnostics.boxPublication.orderedKeys.flatMap((key) => {
            const ref = entityRefFromKey(key);
            return ref ? [Object.freeze({ type: ref.type, id: ref.id })] : [];
          }));
          const primary = diagnostics.boxPublication.primaryKey
            ? entityRefFromKey(diagnostics.boxPublication.primaryKey)
            : null;
          diagnosticsBoxRef.current = {
            generation,
            publication: diagnostics.boxPublication,
            box: Object.freeze({
              actionSequence: diagnostics.boxPublication.actionSequence,
              generation,
              direction: diagnostics.boxPublication.direction,
              filter: diagnostics.boxPublication.filter,
              orderedRefs,
              primaryRef: primary ? Object.freeze({ type: primary.type, id: primary.id }) : null,
              publishedAt: diagnostics.boxPublication.publishedAt,
              renderSubmissionSequence: 0
            })
          };
        }
        const boxEvidence = diagnostics.boxPublication ? diagnosticsBoxRef.current!.box : null;
        const boxSubmission = diagnostics.boxPublication && currentFrame.generation === generation
          ? selectionPresentationBindingRef.current.qualifyingSubmission(resourceGeneration, generation, currentFrame.sequence,
              diagnostics.boxPublication.afterSubmissionSequence, diagnostics.boxPublication.orderedKeys)
          : 0;
        const currentBoxEvidence = boxEvidence && boxEvidence.renderSubmissionSequence !== boxSubmission
          ? Object.freeze({ ...boxEvidence, renderSubmissionSequence: boxSubmission }) : boxEvidence;
        if (currentBoxEvidence && diagnosticsBoxRef.current && currentBoxEvidence !== boxEvidence) {
          diagnosticsBoxRef.current.box = currentBoxEvidence;
        }
        return {
          model: {
            projectId: diagnostics.projectId,
            generation,
            indexGeneration: diagnostics.index.generation,
            projectSessionGeneration: diagnostics.index.sessionGeneration,
            identityHash: currentAssignment.identityHash,
            assignment: {
              status: "committed" as const,
              generation,
              startedAt: currentAssignment.startedAt,
              committedAt: currentAssignment.committedAt!
            }
          },
          tree: diagnostics.treePublication ? {
            generation,
            publicationSequence: diagnostics.treePublication.publicationSequence,
            query: diagnostics.treePublication.query,
            visibleCount: diagnostics.treePublication.visibleCount,
            publishedAt: diagnostics.treePublication.publishedAt
          } : { status: "unavailable" as const },
          viewport: {
            generation,
            canvas: {
              cssLeft: rect.left,
              cssTop: rect.top,
              cssWidth: rect.width,
              cssHeight: rect.height,
              dpr: resource.renderer.getPixelRatio(),
              bufferWidth: resource.renderer.domElement.width,
              bufferHeight: resource.renderer.domElement.height
            },
            camera: {
              sequence: resource.cameraSequence,
              kind: "perspective" as const,
              position: [authoredCamera.x, authoredCamera.y, authoredCamera.z] as const,
              target: [authoredTarget.x, authoredTarget.y, authoredTarget.z] as const,
              up: [resource.camera.up.x, resource.camera.up.y, resource.camera.up.z] as const,
              fovDegrees: resource.camera.fov,
              near: resource.camera.near,
              far: resource.camera.far,
              aspect: resource.camera.aspect,
              localRenderOrigin: [resource.origin.x, resource.origin.y, resource.origin.z] as const
            },
            mainRender: {
              submissionSequence: currentFrame.sequence,
              generation,
              reason: currentFrame.sequence === 0 ? "pending" : "invalidation",
              submittedAt: currentFrame.submittedAt,
              selectionPresentation: renderedSelection ? Object.freeze({
                resourceGeneration: renderedSelection.resourceGeneration,
                modelGeneration: renderedSelection.modelGeneration,
                revision: renderedSelection.revision,
                appliedAfterSubmissionSequence: renderedSelection.appliedAfterSubmissionSequence,
                renderedSubmissionSequence: renderedSelection.renderedSubmissionSequence,
                orderedRefs: Object.freeze(renderedSelection.orderedKeys.flatMap((key) => {
                  const ref = entityRefFromKey(key);
                  return ref ? [Object.freeze({ type: ref.type, id: ref.id })] : [];
                }))
              }) : null,
              nextPaintOpportunity: currentFrame.opportunityAt === null
                ? null
                : {
                    submissionSequence: currentFrame.sequence,
                    generation,
                    at: currentFrame.opportunityAt
                  }
            },
            selection: selectionEvidence.selection,
            inspector: selectionEvidence.inspector,
            filter: diagnostics.treePublication ? {
              actionSequence: diagnostics.treePublication.actionSequence,
              generation,
              query: diagnostics.treePublication.query,
              visibleCount: diagnostics.treePublication.visibleCount,
              inputAt: diagnostics.treePublication.inputAt,
              inputEventTimeStamp: diagnostics.treePublication.inputEventTimeStamp,
              publishedAt: diagnostics.treePublication.publishedAt,
              renderSubmissionSequence: currentFrame.generation === generation && currentFrame.submittedAt !== null &&
                currentFrame.submittedAt >= diagnostics.treePublication.publishedAt ? currentFrame.sequence : 0
            } : { status: "unavailable" as const },
            box: currentBoxEvidence ?? { status: "unavailable" as const },
            labels: {
              enabled: diagnostics.labelsEnabled,
              renderedCount: diagnostics.labelsEnabled ? diagnostics.labelCount : 0,
              budget: 80
            },
            geometry: { mode: diagnostics.geometryMode, odGeneration: diagnostics.odGeneration, odStatus: diagnostics.odStatus },
            resources: {
              rendererInfo,
              ownedPendingRafCount: resource.pendingAppOwnedRafCount,
              owned: resource.ownershipSnapshot,
              context: resource.contextSnapshot
            }
          },
        };
      },
      () => {
        const diagnostics = diagnosticsStateRef.current;
        const currentAssignment = diagnostics.assignment;
        if (currentAssignment?.status !== "committed" ||
            currentAssignment.indexGeneration !== diagnostics.index.generation) {
          throw new Error("UI diagnostics projection requested before a committed model assignment.");
        }
        if (!resource.projectionAvailable) {
          throw new Error("UI diagnostics projection requested without a current viewport context.");
        }
        resource.camera.updateMatrixWorld();
        const viewProjection = new THREE.Matrix4()
          .multiplyMatrices(resource.camera.projectionMatrix, resource.camera.matrixWorldInverse);
        const rect = resource.renderer.domElement.getBoundingClientRect();
        return {
          modelGeneration: currentAssignment.generation,
          cameraSequence: resource.cameraSequence,
          renderOrigin: resource.origin,
          viewProjectionMatrix: viewProjection.toArray(),
          canvasCss: { left: rect.left, top: rect.top, width: rect.width, height: rect.height },
          canvasDevice: { width: resource.renderer.domElement.width, height: resource.renderer.domElement.height }
        };
      }
      );
    }
    const diagnosticState = diagnosticsStateRef.current;
    const currentAssignment = diagnosticState.assignment;
    if (currentAssignment?.status !== "committed" ||
        currentAssignment.indexGeneration !== diagnosticState.index.generation) return;
    refreshUiDiagnostics(includeProjection, currentAssignment.generation);
  }

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    resource.setOrigin(renderTransform.origin);
    const modelObjects: THREE.Object3D[] = [];
    const pickables: THREE.Object3D[] = [];
    const pipeInstances = instancedPipeMeshes(
      model,
      localNodeMap,
      activeModelIndex,
      geometryMode === "actual-od" ? actualOd.radii : new Map()
    );
    modelObjects.push(...pipeInstances);
    modelObjects.push(...instancedNodeMeshes(model, localNodeMap, activeModelIndex));
    modelObjects.push(...instancedSupportMeshes(model, localNodeMap, activeModelIndex));
    modelObjects.push(...instancedComponentMeshes(model, localNodeMap, activeModelIndex));
    const referenceGround = referenceGroundFromBounds(renderTransform.localBounds);
    referenceGround.name = "viewport-reference-ground";
    modelObjects.push(referenceGround);
    resource.setPickables(pickables);
    resource.setPointPrimitives(activeModelIndex, pointPickPrimitives(activeModelIndex, model, renderTransform.origin));
    resource.setActualOdRadiusByPipe(geometryMode === "actual-od" ? actualOd.radii : new Map());
    resource.replaceLayer(resource.modelLayer, modelObjects);
    applySelectionPresentation(resource, orderedSelectionKeys);
  }, [activeModelIndex, actualOd, geometryMode, localNodeMap, model, renderTransform]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    const resultObjects: THREE.Object3D[] = [];
    if (deformation.state === "available") {
      const deformedPipes = instancedDeformedPipeMesh(model, deformation.nodePositions, renderTransform.origin, activeModelIndex);
      resultObjects.push(...deformedPipes);
      const deformedNodes = instancedDeformationMarkerMesh(model, deformation.nodePositions, renderTransform.origin, activeModelIndex);
      resultObjects.push(...deformedNodes);
    }
    resource.replaceLayer(resource.resultLayer, resultObjects);
    applySelectionPresentation(resource, orderedSelectionKeys);
  }, [activeModelIndex, deformation, model, renderTransform]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    resource.replaceLayer(resource.authoredLoadLayer, buildLoadArrows(model, localNodeMap, activeModelIndex));
  }, [activeModelIndex, localNodeMap, model]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    resource.setAuxiliaryVisibility(showGrid, showLoads);
  }, [showGrid, showLoads]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;

    const routingGrid = routeConstructionGridFromBounds(renderTransform.localBounds);
    const routingGhost = routeGhostLine();
    const routingMarker = marker({ x: 0, y: 0, z: 0 }, "routeDraft", 0.105);
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
    resource.replaceLayer(resource.routingLayer, [routingGrid, routingGhost, routingMarker]);
  }, [renderTransform]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (!resource) return;
    const anchorPositions: Array<{
      key: EntityKey;
      position: THREE.Vector3;
      offsetPct: number;
    }> = [];
    for (const key of labelKeys) {
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
    const updateLabelAnchors = () => {
      const layer = selectionLayerRef.current;
      if (!layer) {
        diagnosticsStateRef.current.labelCount = 0;
        return;
      }
      const buttons = new Map(
        [...layer.querySelectorAll<HTMLElement>("[data-entity-key]")]
          .map((button) => [button.dataset.entityKey, button] as const)
      );
      const width = Math.max(1, layer.clientWidth || resource.renderer.domElement.clientWidth);
      const height = Math.max(1, layer.clientHeight || resource.renderer.domElement.clientHeight);
      const occupied: Array<{ left: number; right: number; top: number; bottom: number }> = [];
      let visibleCount = 0;
      for (const { key, position, offsetPct } of anchorPositions) {
        const button = buttons.get(key);
        if (!button) continue;
        const projected = position.clone().project(resource.camera);
        const outsideFrustum = projected.x < -1 || projected.x > 1 || projected.y < -1 || projected.y > 1 ||
          projected.z < -1 || projected.z > 1;
        if (outsideFrustum) {
          button.style.display = "none";
          continue;
        }
        const x = clamp((projected.x * 0.5 + 0.5) * width, 29, Math.max(29, width - 29));
        const y = clamp((-projected.y * 0.5 + 0.5) * height + offsetPct * height / 100, 13, Math.max(13, height - 13));
        const box = { left: x - 31, right: x + 31, top: y - 15, bottom: y + 15 };
        const overlaps = occupied.some((placed) =>
          box.left < placed.right && box.right > placed.left && box.top < placed.bottom && box.bottom > placed.top
        );
        button.style.display = overlaps ? "none" : "";
        if (overlaps) continue;
        occupied.push(box);
        visibleCount += 1;
        button.style.left = `${x}px`;
        button.style.top = `${y}px`;
      }
      diagnosticsStateRef.current.labelCount = visibleCount;
    };
    resource.setLabelUpdater(updateLabelAnchors);
    updateLabelAnchors();
    resource.invalidate();
    return () => resource.setLabelUpdater(null);
  }, [activeModelIndex, labelKeys, renderTransform, showLabels]);

  useEffect(() => {
    const resource = viewportResourceRef.current;
    if (resource) applySelectionPresentation(resource, orderedSelectionKeys);
  }, [orderedSelectionKeys]);

  // The hover halo (design system 6.6). The hovered element stays React state, as it was; the
  // resource is told only when it changes, and does nothing when told the same key again.
  useEffect(() => {
    viewportResourceRef.current?.setHoverPresentation(hoveredEntityKey);
  }, [hoveredEntityKey]);

  useEffect(() => {
    viewportResourceRef.current?.setVisibilityPresentation(hiddenKeys);
  }, [activeModelIndex.generation, hiddenKeys]);

  useLayoutEffect(() => {
    if (!viewCommandRef) return;
    viewCommandRef.current = dispatchViewportViewCommand;
    return () => { viewCommandRef.current = null; };
  });

  useLayoutEffect(() => () => {
    if (boxGestureRef.current) releaseBoxCandidate(boxGestureRef.current.pointerId);
  }, []);

  useLayoutEffect(() => {
    retireBoxGesture();
  }, [model, activeModelIndex.generation]);

  useLayoutEffect(() => {
    cancelBoxSelection();
  }, [activeModelIndex.sessionGeneration]);

  useEffect(() => {
    setMeasurementSource(null);
    setMeasurementActive(false);
    setBoxSelectActive(false);
    setBoxRect(null);
    setGeometryMode("schematic");
  }, [activeModelIndex.sessionGeneration]);

  useLayoutEffect(() => {
    if (armedCreationTool !== null || draftReviewBusy) cancelBoxSelection();
  }, [armedCreationTool, draftReviewBusy]);

  useEffect(() => {
    if (armedCreationTool !== null) setMeasurementActive(false);
  }, [armedCreationTool]);

  useEffect(() => {
    viewportResourceRef.current?.setThemePresentation(theme);
  }, [theme]);

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
    if (measurementActive && !(armedCreationTool === "pipe" && pipeEndpointPickMode)) {
      dispatchViewportViewCommand({ type: "apply-measurement-target", ref: target.ref });
      return;
    }
    if (armedCreationTool === "component" && target.kind === "node") {
      setComponentDraft((current) => componentDraftForNode(model, current, target.ref.id));
    }
    const next = onSelect(target.ref, modifiers);
    publishSelectionAction(next, "label", null);
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
    setHoveredEntityKey(null);
    releasePointerCandidate();
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    const clientX = finitePointerEventNumber(event.clientX, 0);
    const clientY = finitePointerEventNumber(event.clientY, 0);
    const target = event.target as Element & { setPointerCapture?: (pointerId: number) => void };
    pointerGestureRef.current = {
      gesture: startPointerGesture(pointerId, clientX, clientY),
      target,
      pointerDownAt: performance.now()
    };
    target.setPointerCapture?.(pointerId);
  }

  function handleViewportPointerDownCapture(event: ReactPointerEvent<HTMLDivElement>) {
    if (draftReviewBusy) return;
    const canvasTarget = event.target instanceof HTMLCanvasElement;
    if (!claimBoxSelectionPointerDown({
      button: event.button,
      isPrimary: event.isPrimary,
      canvasTarget,
      preventDefault: () => event.preventDefault(),
      stopPropagation: () => event.stopPropagation()
    }, {
      active: boxSelectActive,
      authoringActive: armedCreationTool !== null
    })) return;

    // Box owns this primary gesture. Retire any sub-EPS OrbitControls tail
    // before the selection rectangle can cause a presentation redraw.
    viewportResourceRef.current?.cancelNavigation();
    setHoveredEntityKey(null);
    releasePointerCandidate();
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    const clientX = finitePointerEventNumber(event.clientX, 0);
    const clientY = finitePointerEventNumber(event.clientY, 0);
    const target = event.target as HTMLCanvasElement & { setPointerCapture?: (pointerId: number) => void };
    const rect = target.getBoundingClientRect();
    const startX = clientX - rect.left;
    const startY = clientY - rect.top;
    boxGestureRef.current = {
      context: { model, indexGeneration: activeModelIndex.generation, sessionGeneration: activeModelIndex.sessionGeneration },
      pointerId,
      startX,
      startY,
      currentX: startX,
      currentY: startY,
      pointerDownAt: performance.now(),
      filter: boxSelectionFilter,
      modifiers: { additive: event.shiftKey, toggle: event.ctrlKey || event.metaKey },
      target
    };
    setBoxRect({ left: startX, top: startY, right: startX, bottom: startY });
    target.setPointerCapture?.(pointerId);
  }

  function handleViewportPointerMove(event: ReactPointerEvent<HTMLDivElement>) {
    const boxCandidate = boxGestureRef.current;
    if (boxCandidate && boxCandidate.pointerId === finitePointerEventNumber(event.pointerId, 1)) {
      if (!boxCandidateIsCurrent(boxCandidate.context)) {
        retireBoxGesture();
        return;
      }
      const canvas = viewportResourceRef.current?.renderer.domElement;
      const rect = canvas?.getBoundingClientRect();
      if (rect) {
        boxCandidate.currentX = finitePointerEventNumber(event.clientX, rect.left) - rect.left;
        boxCandidate.currentY = finitePointerEventNumber(event.clientY, rect.top) - rect.top;
        setBoxRect({ left: boxCandidate.startX, top: boxCandidate.startY, right: boxCandidate.currentX, bottom: boxCandidate.currentY });
      }
      return;
    }
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    if (candidate) {
      setHoveredEntityKey(null);
      candidate.gesture = updatePointerGesture(
        candidate.gesture,
        pointerId,
        finitePointerEventNumber(event.clientX, 0),
        finitePointerEventNumber(event.clientY, 0)
      );
    }
    if (!candidate && armedCreationTool === null && event.target instanceof HTMLCanvasElement) {
      const hovered = pickRef.current?.(event) ?? null;
      const nextKey = hovered ? entityKey(hovered) : null;
      setHoveredEntityKey((current) => current === nextKey ? current : nextKey);
      viewportResourceRef.current?.invalidate();
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
    setHoveredEntityKey(null);
    setPointerGhost((current) => current?.provenance === "hover" ? null : current);
  }

  function handleViewportPointerCancel(event: ReactPointerEvent<HTMLDivElement>) {
    if (boxGestureRef.current?.pointerId === finitePointerEventNumber(event.pointerId, 1)) {
      releaseBoxCandidate(boxGestureRef.current.pointerId);
      setBoxRect(null);
      return;
    }
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
    if (boxGestureRef.current?.pointerId === finitePointerEventNumber(event.pointerId, 1)) {
      boxGestureRef.current = null;
      setBoxRect(null);
      return;
    }
    const candidate = pointerGestureRef.current;
    const pointerId = finitePointerEventNumber(event.pointerId, 1);
    if (candidate && candidate.gesture.pointerId === pointerId) {
      pointerGestureRef.current = null;
      setPointerGhost((current) => current?.provenance === "hover" ? null : current);
      setPlacementMessage("Pointer capture was lost; coordinates were not changed.");
    }
  }

  function handleViewportPointerUp(event: ReactPointerEvent<HTMLDivElement>) {
    const boxCandidate = boxGestureRef.current;
    if (boxCandidate && boxCandidate.pointerId === finitePointerEventNumber(event.pointerId, 1)) {
      if (!boxCandidateIsCurrent(boxCandidate.context)) {
        retireBoxGesture();
        return;
      }
      const resource = viewportResourceRef.current;
      const endX = boxCandidate.currentX;
      const endY = boxCandidate.currentY;
      const direction: BoxSelectionDirection = endX >= boxCandidate.startX ? "left-to-right" : "right-to-left";
      const hits = resource ? boxSelectEntityKeys(activeModelIndex, model, {
        camera: resource.camera,
        canvas: resource.renderer.domElement,
        renderOrigin: renderTransform.origin,
        rect: { left: boxCandidate.startX, top: boxCandidate.startY, right: endX, bottom: endY },
        direction,
        filter: boxCandidate.filter,
        hiddenKeys
      }) : Object.freeze([]);
      const next = onBoxSelection(hits, boxCandidate.modifiers, {
        direction,
        filter: boxCandidate.filter,
        pointerDownAt: boxCandidate.pointerDownAt
      });
      publishSelectionAction(next, "pointer", boxCandidate.pointerDownAt, false);
      const publication = {
        actionSequence: ++boxActionSequenceRef.current,
        direction,
        filter: boxCandidate.filter,
        orderedKeys: next.orderedKeys,
        primaryKey: next.primaryKey,
        pointerDownAt: boxCandidate.pointerDownAt,
        publishedAt: performance.now(),
        afterSubmissionSequence: resource?.submissionSequence ?? 0,
        renderSubmissionSequence: 0
      };
      diagnosticsStateRef.current.boxPublication = publication;
      setBoxPublication(publication);
      releaseBoxCandidate(boxCandidate.pointerId);
      setBoxRect(null);
      resource?.invalidate();
      return;
    }
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
      if (measurementActive && !(armedCreationTool === "pipe" && pipeEndpointPickMode)) {
        dispatchViewportViewCommand({ type: "apply-measurement-target", ref: picked });
        return;
      }
      if (armedCreationTool === "pipe" && pipeEndpointPickMode && picked.type === "node") {
        invalidateDraftReview();
        setPipeDraft((current) => nextPipeDraftWithEndpoint(current, pipeEndpointPickMode, picked.id));
        setPipeEndpointPickMode(pipeEndpointPickMode === "from" ? "to" : null);
      }
      if (armedCreationTool === "component" && picked.type === "node") {
        setComponentDraft((current) => componentDraftForNode(model, current, picked.id));
      }
      const next = onSelect(picked, {
        additive: event.shiftKey,
        toggle: event.ctrlKey || event.metaKey
      });
      publishSelectionAction(next, "pointer", candidate?.pointerDownAt ?? performance.now());
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

  function applySelectionPresentation(resource: ViewportResource, keys: readonly EntityKey[]): void {
    const diagnostics = diagnosticsStateRef.current;
    const currentAssignment = diagnostics.assignment;
    const generation = currentAssignment?.status === "committed" && currentAssignment.indexGeneration === diagnostics.index.generation
      ? currentAssignment.generation : null;
    selectionPresentationBindingRef.current.apply(resource, generation, keys);
  }

  function publishSelectionAction(
    next: OrderedSelectionState,
    inputKind: "pointer" | "label",
    pointerDownAt: number | null,
    invalidate = true
  ) {
    const publication = {
      actionSequence: ++selectionActionSequenceRef.current,
      inputKind,
      pointerDownAt,
      orderedKeys: next.orderedKeys,
      primaryKey: next.primaryKey,
      publishedAt: performance.now(),
      afterSubmissionSequence: viewportResourceRef.current?.submissionSequence ?? 0
    };
    diagnosticsStateRef.current.selectionPublication = publication;
    setSelectionPublication(publication);
    if (invalidate) viewportResourceRef.current?.invalidate();
  }

  function boxCandidateIsCurrent(context: BoxGestureContext): boolean {
    return boxSelectActive && !draftReviewBusy && armedCreationTool === null &&
      boxGestureContextIsCurrent(context, {
        model, indexGeneration: activeModelIndex.generation, sessionGeneration: activeModelIndex.sessionGeneration
      });
  }

  function retireBoxGesture(): void {
    if (boxGestureRef.current) releaseBoxCandidate(boxGestureRef.current.pointerId);
    setBoxRect(null);
  }

  function cancelBoxSelection(): void {
    retireBoxGesture();
    setBoxSelectActive(false);
  }

  function releaseBoxCandidate(pointerId: number) {
    const candidate = boxGestureRef.current;
    if (!candidate) return;
    boxGestureRef.current = null;
    const target = candidate.target as Element & { hasPointerCapture?: (id: number) => boolean; releasePointerCapture?: (id: number) => void };
    try {
      if (!target.hasPointerCapture || target.hasPointerCapture(pointerId)) target.releasePointerCapture?.(pointerId);
    } catch {
      // Pointer capture may already be released by the browser.
    }
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
  const fitSelectionBounds = displayedBoundsForEntityKeys(
    activeModelIndex,
    orderedSelectionKeys,
    hiddenKeys,
    geometryMode === "actual-od" ? actualOd.radii : new Map()
  );

  function fitDisplayedBounds(label: string, authoredBounds: Bounds3 | null) {
    const localBounds = authoredBounds ? {
      min: authoredToLocal(authoredBounds.min, renderTransform.origin),
      max: authoredToLocal(authoredBounds.max, renderTransform.origin)
    } : null;
    const resource = viewportResourceRef.current;
    if (resource) {
      fitViewportCamera(resource, viewPreset, localBounds);
      resource.invalidate();
    }
    setViewCommandStatus(resource
      ? `${label} applied to the displayed geometry envelope.`
      : `${label} requested; the 3D renderer is unavailable.`);
  }

  function dispatchViewportViewCommand(command: ViewportViewCommand) {
    if (command.type === "retire-box-gesture") {
      retireBoxGesture();
      return;
    }
    if (command.type === "cancel-box-selection") {
      cancelBoxSelection();
      return;
    }
    if (command.type === "set-box-select") {
      if (!command.active) cancelBoxSelection();
      if (command.active) onViewportInteractionStart("box-selection");
      if (command.active) viewportResourceRef.current?.cancelNavigation();
      setBoxSelectActive(command.active);
      if (command.active) setMeasurementActive(false);
      setViewCommandStatus(`Box Select ${command.active ? "enabled" : "disabled"}.`);
      return;
    }
    if (command.type === "set-selection-filter") {
      setBoxSelectionFilter(command.filter);
      setViewCommandStatus(`Selection filter set to ${command.filter}.`);
      return;
    }
    if (command.type === "set-geometry-mode") {
      setGeometryMode(command.mode);
      setViewCommandStatus(command.mode === "actual-od" ? "Actual OD geometry requested." : "Schematic geometry enabled.");
      return;
    }
    if (command.type === "set-measurement") {
      if (command.active) onViewportInteractionStart("measurement");
      setMeasurementActive(command.active);
      if (command.active) cancelBoxSelection();
      setViewCommandStatus(`Measurement ${command.active ? "enabled" : "disabled"}.`);
      return;
    }
    if (command.type === "apply-measurement-target") {
      setMeasurementSource((current) => {
        const result = applyMeasurementTargetCommand(activeModelIndex, current, command.ref);
        setPlacementMessage(result.message);
        setViewCommandStatus(result.message);
        return result.source;
      });
      return;
    }
    if (command.type === "hide-selection") {
      if (visibilitySelectionReason) { setViewCommandStatus(visibilitySelectionReason); return; }
      onHiddenKeysChange(hideSelectionVisibility(explicitHiddenKeys, visibilitySelectionKeys));
      setViewCommandStatus(`${visibilitySelectionKeys.length} selected item${visibilitySelectionKeys.length === 1 ? "" : "s"} hidden.`);
      return;
    }
    if (command.type === "isolate-selection") {
      if (visibilitySelectionReason) { setViewCommandStatus(visibilitySelectionReason); return; }
      onIsolateKeysChange(isolateSelectionVisibility(activeModelIndex, visibilitySelectionKeys));
      setViewCommandStatus(`${visibilitySelectionKeys.length} selected item${visibilitySelectionKeys.length === 1 ? "" : "s"} isolated with authored context.`);
      return;
    }
    if (command.type === "show-all") {
      onClearVisibility();
      setViewCommandStatus("All eligible model geometry shown.");
      return;
    }
    const radii = geometryMode === "actual-od" ? actualOd.radii : new Map<EntityKey, number>();
    const keys = command.type === "fit-selection" ? orderedSelectionKeys : activeModelIndex.visibilityEligibleKeys;
    const mask = command.type === "fit-model" ? new Set<EntityKey>() : hiddenKeys;
    fitDisplayedBounds(
      command.type === "fit-model" ? "Fit Model" : command.type === "fit-selection" ? "Fit Selection" : "Fit Visible",
      displayedBoundsForEntityKeys(activeModelIndex, keys, mask, radii)
    );
  }

  return (
    <div
      className="viewport-shell"
      style={{ "--viewport-presentation-bottom-inset": `${presentationBottomInset}px` } as CSSProperties}
    >
      <div className="viewport-toolbar">
        <div className="viewport-toolbar-controls" role="group" aria-label="Viewport controls">
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
        <div className="viewport-selection-tools" role="group" aria-label="Viewport selection tools">
          <button
            aria-pressed={boxSelectActive}
            className={boxSelectActive ? "active" : ""}
            data-testid="viewport-box-select"
            onClick={() => dispatchViewportViewCommand({ type: "set-box-select", active: !boxSelectActive })}
            type="button"
          >Box Select</button>
          <label><span className="visually-hidden">Selection filter</span>
            <select
              aria-label="Selection filter"
              data-testid="viewport-selection-filter"
              onChange={(event) => dispatchViewportViewCommand({ type: "set-selection-filter", filter: event.target.value as BoxSelectionFilter })}
              value={boxSelectionFilter}
            >
              <option value="all">All</option><option value="pipes">Pipes</option><option value="nodes">Nodes</option>
              <option value="supports">Supports</option><option value="components">Components</option>
            </select>
          </label>
          <button onClick={() => dispatchViewportViewCommand({ type: "hide-selection" })} disabled={Boolean(visibilitySelectionReason)} title={visibilitySelectionReason ?? "Hide selected geometry."} type="button">Hide</button>
          <button onClick={() => dispatchViewportViewCommand({ type: "isolate-selection" })} disabled={Boolean(visibilitySelectionReason)} title={visibilitySelectionReason ?? "Isolate selected geometry with authored context."} type="button">Isolate</button>
          <button onClick={() => dispatchViewportViewCommand({ type: "show-all" })} disabled={explicitHiddenKeys.size === 0 && isolateHiddenKeys.size === 0} title={explicitHiddenKeys.size === 0 && isolateHiddenKeys.size === 0 ? "All eligible entities are already shown." : "Show every eligible entity."} type="button">Show All</button>
          <button data-testid="viewport-fit-model" onClick={() => dispatchViewportViewCommand({ type: "fit-model" })} type="button">Fit Model</button>
          <button onClick={() => dispatchViewportViewCommand({ type: "fit-visible" })} type="button">Fit Visible</button>
          <button data-testid="viewport-fit-selection" onClick={() => dispatchViewportViewCommand({ type: "fit-selection" })} disabled={!fitSelectionBounds} title={fitSelectionBounds ? "Fit currently visible selected geometry." : "No visible valid selected geometry can be fitted."} type="button">Fit Selection</button>
        </div>
        <div className="viewport-geometry-tools" role="group" aria-label="Viewport geometry">
          <button aria-pressed={geometryMode === "schematic"} data-testid="viewport-geometry-schematic" onClick={() => dispatchViewportViewCommand({ type: "set-geometry-mode", mode: "schematic" })} type="button">Schematic</button>
          <button aria-pressed={geometryMode === "actual-od"} data-testid="viewport-geometry-actual-od" onClick={() => dispatchViewportViewCommand({ type: "set-geometry-mode", mode: "actual-od" })} type="button">Actual OD</button>
          <button aria-pressed={measurementActive} onClick={() => dispatchViewportViewCommand({ type: "set-measurement", active: !measurementActive })} type="button">Measure</button>
        </div>
        </div>
        <div className="viewport-toolbar-status-strip" aria-label="Viewport status">
          <span
            className="viewport-toolbar-selection-status"
            title={`Selected ${selection.type}: ${selection.id}`}
          >Selected: {selection.id}</span>
          {selectedHiddenCount > 0 ? <span role="status" title={`${selectedHiddenCount} selected item${selectedHiddenCount === 1 ? " is" : "s are"} hidden`}>{selectedHiddenCount} selected item{selectedHiddenCount === 1 ? " is" : "s are"} hidden</span> : null}
          <span role="status" data-testid="viewport-od-status" title={geometryMode === "actual-od" ? actualOd.reason : "Schematic centerline geometry"}>{geometryMode === "actual-od" ? actualOd.reason : "Schematic centerline geometry"}</span>
          <span role="status" aria-label="View command status" data-testid="viewport-view-command-status" title={viewCommandStatus}>{viewCommandStatus}</span>
          {viewportContextStatus !== "ready" ? (
            <span role="status" data-testid="viewport-context-status" title={viewportContextStatus === "lost" ? "3D context lost; rendering paused." : "Restoring 3D resources…"}>
              {viewportContextStatus === "lost" ? "3D context lost; rendering paused." : "Restoring 3D resources…"}
            </span>
          ) : null}
          {activeModelIndex.invalidGeometry.size > 0 ? (
            <span role="status" data-testid="viewport-invalid-geometry" title={`${activeModelIndex.invalidGeometry.size} item${activeModelIndex.invalidGeometry.size === 1 ? "" : "s"} excluded: ${activeModelIndex.invalidGeometry.values().next().value}`}>
              {activeModelIndex.invalidGeometry.size} item{activeModelIndex.invalidGeometry.size === 1 ? "" : "s"} excluded: {activeModelIndex.invalidGeometry.values().next().value}
            </span>
          ) : null}
          {renderTransformResolution.issue ? (
            <span role="status" data-testid="viewport-render-transform-status" title={`Viewport geometry excluded: ${renderTransformResolution.issue}`}>
              Viewport geometry excluded: {renderTransformResolution.issue}
            </span>
          ) : null}
        </div>
      </div>
      <div className={`viewport-frame${boxSelectActive ? " box-select-active" : ""}`}>
        <div
          className="viewport-canvas"
          data-testid="viewport-canvas"
          onPointerDownCapture={handleViewportPointerDownCapture}
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
              const active = selectionContainsKey(orderedSelectionKeySet, target.ref);
              return (
                <button
                  aria-label={`Select ${target.label} in viewport`}
                  aria-pressed={active}
                  className={`viewport-select-target ${target.kind} ${active ? "active" : ""}`}
                  data-entity-key={entityKey(target.ref)}
                  data-testid={`viewport-select-${target.ref.id}`}
                  disabled={draftReviewBusy}
                  key={entityKey(target.ref)}
                  onPointerEnter={() => setHoveredEntityKey(entityKey(target.ref))}
                  onPointerLeave={() => setHoveredEntityKey(null)}
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
        {boxRect ? (
          <div
            aria-hidden="true"
            className="viewport-box-rect"
            style={{
              left: Math.min(boxRect.left, boxRect.right),
              top: Math.min(boxRect.top, boxRect.bottom),
              width: Math.abs(boxRect.right - boxRect.left),
              height: Math.abs(boxRect.bottom - boxRect.top)
            }}
          />
        ) : null}
        <div className="viewport-axis-triad" aria-label="Orientation gizmo showing X, Y, Z axes" data-testid="viewport-axis-triad" role="img">
          <div className="viewport-gizmo-host" ref={gizmoHostRef} aria-hidden="true" />
        </div>
        <div className="viewport-view-cube" aria-label="View controls" data-testid="viewport-view-cube">
          <button type="button" aria-pressed={viewPreset === "front"} onClick={() => setViewPreset("front")}>
            Front
          </button>
          <button type="button" aria-pressed={viewPreset === "top"} onClick={() => setViewPreset("top")}>
            Top
          </button>
          <button type="button" data-testid="viewport-view-isometric" aria-pressed={viewPreset === "iso"} onClick={() => setViewPreset("iso")}>
            Isometric
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
        <span aria-label="Selection summary" role="status" data-testid="command-selection-readout">
          Selected {selection.type}: {selection.id}; {visibleIntents.length} queued
        </span>
        <span className="command-hint" data-testid="viewport-orbit-hint">
          Drag to orbit · scroll to zoom · right-drag to pan
        </span>
        </div></details>
      </section>
      {measurementPoints.length > 0 || measurementReadout.status === "unavailable" ? (
        <div className="viewport-measurement-strip" role="status" data-testid="viewport-measurement-readout">
          <strong>Measure · {measurementTargetSummary}</strong>
          <span>{measurementReadout.status === "available"
            ? <>
                <ViewportMeasurementQuantityReadout label="Distance" unit={defaultLengthUnit} value={measurementReadout.values.get("distance")!} /> ·
                <ViewportMeasurementQuantityReadout label="ΔX" unit={defaultLengthUnit} value={measurementReadout.values.get("dx")!} /> ·
                <ViewportMeasurementQuantityReadout label="ΔY" unit={defaultLengthUnit} value={measurementReadout.values.get("dy")!} /> ·
                <ViewportMeasurementQuantityReadout label="ΔZ" unit={defaultLengthUnit} value={measurementReadout.values.get("dz")!} />
              </>
            : measurementReadout.reason}</span>
        </div>
      ) : null}
      <OptionalPortal container={authoringPanelContainer}>
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
              <CompactSelect
                aria-label="New node coordinate unit"
                data-testid="viewport-create-node-unit"
                onValueChange={(value) => updateNodeDraft("coordinateUnit", value)}
                value={nodeDraft.coordinateUnit}
                options={nodeLengthUnitOptions.map((option) => ({ value: option.symbol, label: option.symbol }))}
              />
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
              <VirtualTargetPicker label="New pipe from node" testId="viewport-create-pipe-from" options={nodeTargetOptions} value={pipeDraft.from} onChange={(value) => updatePipeDraft("from", value)} />
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
              <VirtualTargetPicker label="New pipe to node" testId="viewport-create-pipe-to" options={nodeTargetOptions} value={pipeDraft.to} onChange={(value) => updatePipeDraft("to", value)} />
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
                  <CompactSelect
                    aria-label="Route construction plane"
                    data-testid="viewport-routing-plane"
                    value={routingPlane}
                    onValueChange={(value) => changeRoutingPlane(value as RoutingPlane)}
                    title={routingControlReason}
                    options={(["XY", "XZ", "YZ"] as const).map((plane) => ({ value: plane, label: plane }))}
                  />
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
              <label><span>Coordinate unit</span><CompactSelect
                aria-label="Route end coordinate unit"
                data-testid="viewport-route-end-unit"
                value={newEndDraft.coordinateUnit}
                onValueChange={(value) => updateNewEndDraft("coordinateUnit", value)}
                options={nodeLengthUnitOptions.map((option) => ({ value: option.symbol, label: option.symbol }))}
              /></label>
              <label><span>Provenance</span><input aria-label="Route end provenance" data-testid="viewport-route-end-provenance" value={newEndDraft.provenance} onChange={(event) => updateNewEndDraft("provenance", event.target.value)} /></label>
            </div>
            <VirtualTargetPicker label="New pipe material" testId="viewport-create-pipe-material" options={materialTargetOptions} value={pipeDraft.material} onChange={(value) => updatePipeDraft("material", value)} />
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
              <CompactSelect
                aria-label="New pipe length unit"
                data-testid="viewport-create-pipe-length-unit"
                onValueChange={(value) => updatePipeDraft("lengthUnit", value)}
                value={pipeDraft.lengthUnit}
                options={pipeLengthUnitOptions.map((option) => ({ value: option.symbol, label: option.symbol }))}
              />
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
              <CompactSelect
                aria-label="New component kind"
                data-testid="viewport-create-component-kind"
                value={componentDraft.kind}
                onValueChange={(value) =>
                  setComponentDraft((current) =>
                    componentDraftForKind(model, current, value as CreatableComponentKind)
                  )
                }
                options={creatableComponentKinds.map((kind) => ({ value: kind, label: kind }))}
              />
            </label>
            <VirtualTargetPicker label="New component node" testId="viewport-create-component-node" options={nodeTargetOptions} value={componentDraft.node} onChange={(value) => setComponentDraft((current) => componentDraftForNode(model, current, value))} />
            <VirtualTargetPicker label={componentDraft.kind === "tee" ? "New tee header pipe" : "New component realized pipe"} testId="viewport-create-component-pipe" options={connectedPipeTargetOptions} value={componentDraft.primaryPipeRef} onChange={(value) => updateComponentDraft("primaryPipeRef", value)} />
            {componentDraft.kind === "tee" ? (
              <VirtualTargetPicker label="New tee branch pipe" testId="viewport-create-component-secondary-pipe" options={connectedPipeTargetOptions} value={componentDraft.secondaryPipeRef} onChange={(value) => updateComponentDraft("secondaryPipeRef", value)} />
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
      </OptionalPortal>
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
  // Programmatic camera commands establish a new pose; retained user-input
  // damping must not be applied on top of it.
  resource.cancelNavigation();
  const center = bounds
    ? {
        x: bounds.min.x / 2 + bounds.max.x / 2,
        y: bounds.min.y / 2 + bounds.max.y / 2,
        z: bounds.min.z / 2 + bounds.max.z / 2
      }
    : { x: 0, y: 0, z: 0 };
  const extent = bounds
    ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1)
    : 4;
  const distance = fittedViewportDistance(bounds, preset, resource.camera.fov, resource.camera.aspect);
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
  resource.markCameraProjectionChanged();
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

function actualOdConversionPlan(
  index: ModelIndex,
  model: PreviewModel,
  modelLengthUnit: string
): {
  requests: Array<{ id: string; value: number; from_unit: string; to_unit: string; dimension_id: string }>;
  requestByPipe: ReadonlyMap<EntityKey, {
    outsideDiameter: string;
    wallThickness: string;
    inlineOutsideDiameter: string;
    inlineWallThickness: string;
  }>;
  invalidCount: number;
  reason: string;
} {
  const requests: Array<{ id: string; value: number; from_unit: string; to_unit: string; dimension_id: string }> = [];
  const requestByQuantity = new Map<string, string>();
  const requestByPipe = new Map<EntityKey, {
    outsideDiameter: string;
    wallThickness: string;
    inlineOutsideDiameter: string;
    inlineWallThickness: string;
  }>();
  let invalidCount = 0;
  const requestId = (value: number, fromUnit: string) => {
    const signature = JSON.stringify([value, fromUnit, modelLengthUnit]);
    const existing = requestByQuantity.get(signature);
    if (existing) return existing;
    const id = `viewport-actual-od-${requestByQuantity.size + 1}`;
    requestByQuantity.set(signature, id);
    requests.push({ id, value, from_unit: fromUnit, to_unit: modelLengthUnit, dimension_id: "length" });
    return id;
  };
  for (const pipe of model.pipe_segments) {
    const key = entityKey({ type: "pipe", id: pipe.id });
    const binding = index.sectionBindings.get(key);
    if (!binding || binding.issue || index.invalidGeometry.has(key)) {
      invalidCount += 1;
      continue;
    }
    const outsideDiameter = binding.record.outside_diameter;
    const wallThickness = binding.record.wall_thickness;
    const inlineOutsideDiameter = pipe.section.outside_diameter;
    const inlineWallThickness = pipe.section.wall_thickness;
    if (!outsideDiameter || !wallThickness || !(outsideDiameter.value > 0) || !(wallThickness.value > 0) ||
        !inlineOutsideDiameter || !inlineWallThickness || !(inlineOutsideDiameter.value > 0) || !(inlineWallThickness.value > 0) ||
        !Number.isFinite(outsideDiameter.value) || !Number.isFinite(wallThickness.value) ||
        !Number.isFinite(inlineOutsideDiameter.value) || !Number.isFinite(inlineWallThickness.value) ||
        !outsideDiameter.unit.trim() || !wallThickness.unit.trim() ||
        !inlineOutsideDiameter.unit.trim() || !inlineWallThickness.unit.trim() ||
        !modelLengthUnit.trim() || modelLengthUnit === "TBD") {
      invalidCount += 1;
      continue;
    }
    requestByPipe.set(key, {
      outsideDiameter: requestId(outsideDiameter.value, outsideDiameter.unit),
      wallThickness: requestId(wallThickness.value, wallThickness.unit),
      inlineOutsideDiameter: requestId(inlineOutsideDiameter.value, inlineOutsideDiameter.unit),
      inlineWallThickness: requestId(inlineWallThickness.value, inlineWallThickness.unit)
    });
  }
  return {
    requests,
    requestByPipe,
    invalidCount,
    reason: invalidCount > 0
      ? `${invalidCount} pipe span${invalidCount === 1 ? "" : "s"} has no consistent explicit OD/wall binding.`
      : "No pipe spans expose an explicit OD/wall binding."
  };
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

function selectionContainsKey(keys: ReadonlySet<EntityKey>, ref: EntityRef): boolean {
  return keys.has(entityKey(ref));
}

function midpoint(from: Vec3, to: Vec3): Vec3 {
  return {
    x: from.x / 2 + to.x / 2,
    y: from.y / 2 + to.y / 2,
    z: from.z / 2 + to.z / 2
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

function spatialGroups<T>(index: ModelIndex, entries: ReadonlyMap<EntityKey, T>): T[][] {
  const groups: T[][] = [];
  for (const chunkKeys of index.spatialChunks.map((chunk) => chunk.entityKeys)) {
    const group = chunkKeys.flatMap((key) => {
      const entry = entries.get(key);
      return entry === undefined ? [] : [entry];
    });
    if (group.length > 0) groups.push(group);
  }
  return groups;
}

function spatialMultiGroups<T>(index: ModelIndex, entries: ReadonlyMap<EntityKey, readonly T[]>): T[][] {
  const groups: T[][] = [];
  const entityTypes = new Set([...entries.keys()].flatMap((key) => {
    const indexed = index.entities.get(key);
    return indexed && ["pipe", "node", "support", "component"].includes(indexed.ref.type)
      ? [indexed.ref.type as "pipe" | "node" | "support" | "component"]
      : [];
  }));
  const chunkGroups = [...entityTypes].flatMap((type) => spatialEntityKeyGroups(index, type));
  for (const chunkKeys of chunkGroups) {
    const group = chunkKeys.flatMap((key) => entries.get(key) ?? []);
    if (group.length > 0) groups.push(group);
  }
  return groups;
}

function instancedPipeMeshes(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  modelIndex: ModelIndex,
  actualRadii: ReadonlyMap<EntityKey, number>
): THREE.InstancedMesh[] {
  const validByKey = new Map(model.pipe_segments.flatMap((pipe) => {
    const from = nodes.get(pipe.from);
    const to = nodes.get(pipe.to);
    const key = entityKey({ type: "pipe", id: pipe.id });
    if (!from || !to || modelIndex.invalidGeometry.has(key)) return [];
    const direction = new THREE.Vector3(to.x - from.x, to.y - from.y, to.z - from.z);
    const length = direction.length();
    return Number.isFinite(length) && length > 0 ? [[key, { pipe, from, to, direction, length }] as const] : [];
  }));
  if (validByKey.size === 0) return [];
  const geometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false);
  // The tube carries the edge line (design system 6.2); its outline is read from the geometry.
  const material = createFigureMaterial({ edge: figureEdgeOutlineFor(geometry) });
  return spatialGroups(modelIndex, validByKey).map((valid) => {
    const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
    const matrix = new THREE.Matrix4();
    const quaternion = new THREE.Quaternion();
    const up = new THREE.Vector3(0, 1, 0);
    const scale = new THREE.Vector3();
    const center = new THREE.Vector3();
    valid.forEach(({ pipe, from, to, direction, length }, instance) => {
      quaternion.setFromUnitVectors(up, direction.normalize());
      center.set(from.x / 2 + to.x / 2, from.y / 2 + to.y / 2, from.z / 2 + to.z / 2);
      const radius = actualRadii.get(entityKey({ type: "pipe", id: pipe.id })) ?? 0.052;
      scale.set(radius, length, radius);
      matrix.compose(center, quaternion, scale);
      mesh.setMatrixAt(instance, matrix);
    });
    mesh.instanceMatrix.needsUpdate = true;
    registerInstancedRolePresentation(
      mesh,
      valid.map(({ pipe }) => entityKey({ type: "pipe", id: pipe.id })),
      "pipe"
    );
    return mesh;
  });
}

function instancedNodeMeshes(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  index: ModelIndex
): THREE.InstancedMesh[] {
  const validByKey = new Map(model.nodes.flatMap((node) => {
    const key = entityKey({ type: "node", id: node.id });
    return nodes.has(node.id) && !index.invalidGeometry.has(key) ? [[key, node] as const] : [];
  }));
  if (validByKey.size === 0) return [];
  const geometry = new THREE.SphereGeometry(0.095, 12, 8);
  const material = createFigureMaterial();
  return spatialGroups(index, validByKey).map((valid) => {
    const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
    const matrix = new THREE.Matrix4();
    valid.forEach((node, instance) => {
      const position = nodes.get(node.id)!;
      matrix.makeTranslation(position.x, position.y, position.z);
      mesh.setMatrixAt(instance, matrix);
    });
    mesh.instanceMatrix.needsUpdate = true;
    registerInstancedRolePresentation(mesh, valid.map((node) => entityKey({ type: "node", id: node.id })), "node");
    return mesh;
  });
}

function instancedSupportMeshes(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  index: ModelIndex
): THREE.InstancedMesh[] {
  const validByKey = new Map(model.supports.flatMap((support) => {
    const key = entityKey({ type: "support", id: support.id });
    if (index.invalidGeometry.has(key)) return [];
    const node = nodes.get(support.node);
    return node ? [[key, { support, node }] as const] : [];
  }));
  if (validByKey.size === 0) return [];
  const geometry = new THREE.ConeGeometry(0.18, 0.34, 4);
  const material = createFigureMaterial();
  const quaternion = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 1, 0), Math.PI / 4);
  const scale = new THREE.Vector3(1, 1, 1);
  return spatialGroups(index, validByKey).map((valid) => {
    const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
    const matrix = new THREE.Matrix4();
    valid.forEach(({ support, node }, instance) => {
      matrix.compose(new THREE.Vector3(node.x, node.y - 0.26, node.z), quaternion, scale);
      mesh.setMatrixAt(instance, matrix);
    });
    mesh.instanceMatrix.needsUpdate = true;
    registerInstancedRolePresentation(mesh, valid.map(({ support }) => entityKey({ type: "support", id: support.id })), "support");
    return mesh;
  });
}

function instancedComponentMeshes(
  model: PreviewModel,
  nodes: ReadonlyMap<string, Vec3>,
  index: ModelIndex
): THREE.InstancedMesh[] {
  const groups = new Map<string, Map<EntityKey, PreviewComponent>>();
  for (const component of model.components) {
    const key = entityKey({ type: "component", id: component.id });
    if (!nodes.has(component.node) || index.invalidGeometry.has(key)) continue;
    const kind = isBendComponent(component) ? "bend" : isBranchComponent(component) ? "branch" : isExpansionJointComponent(component) ? "expansion" : "rigid";
    const kindEntries = groups.get(kind) ?? new Map<EntityKey, PreviewComponent>();
    kindEntries.set(key, component);
    groups.set(kind, kindEntries);
  }
  const output: THREE.InstancedMesh[] = [];
  for (const [kind, entries] of groups) {
    const geometry = kind === "bend"
      ? new THREE.TorusGeometry(0.24, 0.027, 8, 18, Math.PI * 0.75)
      : kind === "branch"
        ? new THREE.CylinderGeometry(0.055, 0.055, 0.44, 10)
        : kind === "expansion"
          ? new THREE.CylinderGeometry(0.11, 0.11, 0.34, 12)
          : new THREE.BoxGeometry(0.24, 0.24, 0.24);
    // The figure is neutral: a fitting takes the tube's colour and is told apart by its shape.
    const role: ViewportPaletteRole = kind === "bend"
      ? "componentBend"
      : kind === "branch"
        ? "componentBranch"
        : kind === "expansion"
          ? "componentExpansion"
          : "componentRigid";
    // Every placeholder takes the pipe's edge line (design system 6.3), along its own shape's outline.
    const material = createFigureMaterial({ edge: figureEdgeOutlineFor(geometry) });
    for (const components of spatialGroups(index, entries)) {
      const mesh = new THREE.InstancedMesh(geometry, material, components.length);
      const matrix = new THREE.Matrix4();
      components.forEach((component, instance) => {
        const node = nodes.get(component.node)!;
        matrix.makeTranslation(node.x, node.y + 0.2, node.z);
        mesh.setMatrixAt(instance, matrix);
      });
      mesh.instanceMatrix.needsUpdate = true;
      registerInstancedRolePresentation(
        mesh,
        components.map((component) => entityKey({ type: "component", id: component.id })),
        role
      );
      output.push(mesh);
    }
  }
  return output;
}

function instancedDeformedPipeMesh(
  model: PreviewModel,
  positions: ReadonlyMap<string, Vec3>,
  origin: Readonly<Vec3>,
  index: ModelIndex
): THREE.InstancedMesh[] {
  const validByKey = new Map(model.pipe_segments.flatMap((pipe) => {
    const key = entityKey({ type: "pipe", id: pipe.id });
    const from = positions.get(pipe.from);
    const to = positions.get(pipe.to);
    if (!from || !to || index.invalidGeometry.has(key)) return [];
    const localFrom = authoredToLocal(from, origin);
    const localTo = authoredToLocal(to, origin);
    const direction = new THREE.Vector3(localTo.x - localFrom.x, localTo.y - localFrom.y, localTo.z - localFrom.z);
    const length = direction.length();
    return length > 0 && Number.isFinite(length)
      ? [[key, { pipe, from: localFrom, to: localTo, direction, length }] as const]
      : [];
  }));
  if (!validByKey.size) return [];
  const geometry = new THREE.CylinderGeometry(1, 1, 1, 10);
  const material = createFigureMaterial({ opacity: 0.82, transparent: true });
  return spatialGroups(index, validByKey).map((valid) => {
    const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
    const matrix = new THREE.Matrix4();
    const quaternion = new THREE.Quaternion();
    const up = new THREE.Vector3(0, 1, 0);
    valid.forEach(({ from, to, direction, length }, instance) => {
      quaternion.setFromUnitVectors(up, direction.normalize());
      matrix.compose(
        new THREE.Vector3(from.x / 2 + to.x / 2, from.y / 2 + to.y / 2, from.z / 2 + to.z / 2),
        quaternion,
        new THREE.Vector3(0.032, length, 0.032)
      );
      mesh.setMatrixAt(instance, matrix);
    });
    registerInstancedRolePresentation(mesh, valid.map(({ pipe }) => entityKey({ type: "pipe", id: pipe.id })), "deformedShape");
    return mesh;
  });
}

function instancedDeformationMarkerMesh(
  model: PreviewModel,
  positions: ReadonlyMap<string, Vec3>,
  origin: Readonly<Vec3>,
  index: ModelIndex
): THREE.InstancedMesh[] {
  const validByKey = new Map(model.nodes.flatMap((node) => {
    const key = entityKey({ type: "node", id: node.id });
    return positions.has(node.id) && !index.invalidGeometry.has(key) ? [[key, node] as const] : [];
  }));
  if (!validByKey.size) return [];
  const geometry = new THREE.SphereGeometry(0.055, 10, 7);
  const material = createFigureMaterial({ opacity: 0.86, transparent: true });
  return spatialGroups(index, validByKey).map((valid) => {
    const mesh = new THREE.InstancedMesh(geometry, material, valid.length);
    const matrix = new THREE.Matrix4();
    valid.forEach((node, instance) => {
      const position = authoredToLocal(positions.get(node.id)!, origin);
      matrix.makeTranslation(position.x, position.y, position.z);
      mesh.setMatrixAt(instance, matrix);
    });
    registerInstancedRolePresentation(mesh, valid.map((node) => entityKey({ type: "node", id: node.id })), "deformedShape");
    return mesh;
  });
}

function marker(position: Vec3, role: ViewportPaletteRole, radius: number) {
  const mesh = new THREE.Mesh(new THREE.SphereGeometry(radius, 24, 16), createFigureMaterial());
  registerPaletteRole(mesh, role);
  return mesh
    .translateX(position.x)
    .translateY(position.y)
    .translateZ(position.z);
}

function routeConstructionGridFromBounds(bounds: Bounds3 | null): THREE.GridHelper {
  const span = bounds
    ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z)
    : 0;
  const size = Math.max(8, Math.ceil(span * 1.5));
  const grid = new THREE.GridHelper(size, Math.max(8, Math.min(40, Math.round(size * 2))));
  registerGridPaletteRoles(grid, "routeGridAxis", "routeGridLine");
  const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
  for (const material of materials) {
    // The grid tokens are already stepped for the ground, so the lines draw at full opacity.
    // The material stays in the transparent pass, so the draw order is what it was.
    material.transparent = true;
    material.opacity = 1;
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
    new THREE.LineDashedMaterial({ dashSize: 0.18, gapSize: 0.1, depthTest: false })
  );
  // The routing draft has its own token; it is never the selection's colour.
  registerPaletteRole(line, "routeDraft");
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

function isBendComponent(component: PreviewComponent): boolean {
  return component.kind === "bend" || component.kind === "elbow";
}

function isBranchComponent(component: PreviewComponent): boolean {
  return component.kind === "branch" || component.kind === "tee" || component.kind === "branch_connection";
}

function isExpansionJointComponent(component: PreviewComponent): boolean {
  return component.kind === "expansion_joint";
}

// Ground reference grid on the global XZ plane, centred under the model bounds
// and sized to the model — replaces the old fixed-size grid that was rotated
// into a vertical plane and offset from a hard-coded point.
function referenceGroundFromBounds(bounds: Bounds3 | null): THREE.GridHelper {
  const minX = bounds?.min.x ?? 0;
  const maxX = bounds?.max.x ?? 1;
  const minY = bounds?.min.y ?? 0;
  const minZ = bounds?.min.z ?? 0;
  const maxZ = bounds?.max.z ?? 1;
  const size = Math.max(maxX - minX, maxZ - minZ, 1) * 1.6;
  const divisions = Math.max(4, Math.min(20, Math.round(size)));
  const helper = new THREE.GridHelper(size, divisions);
  registerGridPaletteRoles(helper, "groundGridMajor", "groundGridMinor");
  helper.position.set((minX + maxX) / 2, minY - 0.02, (minZ + maxZ) / 2);
  const material = helper.material as THREE.Material & { opacity: number };
  // The grid tokens are already stepped for the ground, so the lines draw at full opacity.
  // The material stays in the transparent pass, so the draw order is what it was.
  material.transparent = true;
  material.opacity = 1;
  return helper;
}

// A moment is told from every other load, and no further. Moments are built first.
type LoadArrowRole = Extract<ViewportPaletteRole, "loadMoment" | "loadForce">;
const LOAD_ARROW_ROLES: readonly LoadArrowRole[] = ["loadMoment", "loadForce"];

// Real 3D load arrows anchored to the loaded node or element midpoint and
// oriented along the load's global direction, so they move with the model.
function buildLoadArrows(
  model: PreviewModel,
  nodeMap: Map<string, Vec3>,
  index: ModelIndex
): THREE.Object3D[] {
  const pipeMidpoints = new Map<string, Vec3>();
  for (const segment of model.pipe_segments) {
    const from = nodeMap.get(segment.from);
    const to = nodeMap.get(segment.to);
    if (from && to) pipeMidpoints.set(segment.id, midpoint(from, to));
  }
  const supportNodes = new Map(model.supports.map((support) => [support.id, support.node] as const));
  const arrows: Array<{ anchor: Vec3; direction: THREE.Vector3; role: LoadArrowRole; ownerKey: EntityKey }> = [];
  for (const loadCase of model.load_cases) {
    for (const primitive of loadCase.primitive_loads ?? []) {
      const record = primitive as Record<string, unknown>;
      const ownedAnchor = loadAnchor(record, nodeMap, pipeMidpoints, supportNodes);
      const direction = globalDirectionVector(record);
      if (!ownedAnchor || !direction || index.invalidGeometry.has(ownedAnchor.ownerKey)) continue;
      const isMoment = String(record.dimension ?? "").includes("moment");
      const role: LoadArrowRole = isMoment ? "loadMoment" : "loadForce";
      arrows.push({ anchor: ownedAnchor.anchor, direction, role, ownerKey: ownedAnchor.ownerKey });
    }
  }
  const arrowsByOwner = new Map<EntityKey, typeof arrows>();
  for (const arrow of arrows) arrowsByOwner.set(arrow.ownerKey, [...(arrowsByOwner.get(arrow.ownerKey) ?? []), arrow]);
  const spatialArrowGroups = spatialMultiGroups(index, arrowsByOwner);
  const output: THREE.Object3D[] = [];
  for (const role of LOAD_ARROW_ROLES) {
    if (!arrows.some((arrow) => arrow.role === role)) continue;
    const shaftGeometry = new THREE.CylinderGeometry(0.025, 0.025, 1, 8);
    const headGeometry = new THREE.ConeGeometry(0.12, 1, 8);
    // Unlit and white: the instance colour alone is the drawn colour, so an arrow draws at its
    // token and a selected arrow at the selected colour.
    const material = new THREE.MeshBasicMaterial();
    for (const group of spatialArrowGroups) {
      const entries = group.filter((arrow) => arrow.role === role);
      if (!entries.length) continue;
      const shaft = new THREE.InstancedMesh(shaftGeometry, material, entries.length);
      const head = new THREE.InstancedMesh(headGeometry, material, entries.length);
      const matrix = new THREE.Matrix4();
      const quaternion = new THREE.Quaternion();
      const up = new THREE.Vector3(0, 1, 0);
      entries.forEach(({ anchor, direction }, instance) => {
        quaternion.setFromUnitVectors(up, direction);
        matrix.compose(
          new THREE.Vector3(anchor.x, anchor.y, anchor.z).addScaledVector(direction, 0.3),
          quaternion,
          new THREE.Vector3(1, 0.6, 1)
        );
        shaft.setMatrixAt(instance, matrix);
        matrix.compose(
          new THREE.Vector3(anchor.x, anchor.y, anchor.z).addScaledVector(direction, 0.76),
          quaternion,
          new THREE.Vector3(1, 0.28, 1)
        );
        head.setMatrixAt(instance, matrix);
      });
      shaft.instanceMatrix.needsUpdate = true;
      head.instanceMatrix.needsUpdate = true;
      const ownershipKeys = entries.map((entry) => entry.ownerKey);
      registerInstancedRolePresentation(shaft, ownershipKeys, role);
      registerInstancedRolePresentation(head, ownershipKeys, role);
      output.push(shaft, head);
    }
  }
  return output;
}

function loadAnchor(
  primitive: Record<string, unknown>,
  nodeMap: Map<string, Vec3>,
  pipeMidpoints: Map<string, Vec3>,
  supportNodes: ReadonlyMap<string, string>
): { anchor: Vec3; ownerKey: EntityKey } | null {
  const target = primitive.target as Record<string, unknown> | undefined;
  if (!target) return null;
  if (target.type === "node" && typeof target.node === "string") {
    const anchor = nodeMap.get(target.node);
    return anchor ? { anchor, ownerKey: entityKey({ type: "node", id: target.node }) } : null;
  }
  if (target.type === "element" && typeof target.pipe === "string") {
    const anchor = pipeMidpoints.get(target.pipe);
    return anchor ? { anchor, ownerKey: entityKey({ type: "pipe", id: target.pipe }) } : null;
  }
  if (target.type === "support" && typeof target.support === "string") {
    const nodeId = supportNodes.get(target.support);
    const anchor = nodeId ? nodeMap.get(nodeId) : null;
    return anchor ? { anchor, ownerKey: entityKey({ type: "support", id: target.support }) } : null;
  }
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
      summary: `blocked; mechanics=${statusDisplayWithToken(result.status.mechanics)}; rows=${result.results.length}`,
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

function formatNumber(value: number): string {
  return value.toFixed(6).replace(/0+$/u, "").replace(/\.$/u, "");
}

function finitePointerEventNumber(value: number, fallback: number): number {
  return Number.isFinite(value) ? value : fallback;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
