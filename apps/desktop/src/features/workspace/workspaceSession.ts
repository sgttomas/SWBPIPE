import { hasCurrentSourceContract } from "../results/numericalResultQuality";
import type React from "react";
import { isTauriRuntime, syncNativeShellState } from "../../services/nativeMenu";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import {
  canonicalSha256Hex,
  checkedJsonText,
  computeModelHash,
  computeProjectEnvelopeHash
} from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { applyOperationBatch, validateOperationBatch } from "../../services/operationBatchService";
import type { OperationBatch } from "../../services/operationBatchService";
import {
  applyModelOperation,
  validateModelOperation,
  warmupOperationEngine
} from "../../services/operationService";
import {
  buildAnalysisRunPreview,
  cancelPreviewMechanicsJob,
  loadDesignKnowledge,
  loadBundledMechanicsReference,
  loadPreviewModel,
  loadSampleProposal,
  hasNativeMechanicsInvocation,
  startPreviewMechanicsJob
} from "../../services/previewService";
import {
  buildBlankLocalModelDocument,
  createLocalProject,
  getLocalStorageCapability,
  listLocalProjects,
  openLocalProject,
  saveLocalProject
} from "../../services/projectService";
import { saveReportPackage } from "../../services/reportPackageSaveService";
import type { RuleCheckStatus } from "../../services/ruleCheckService";
import type {
  AnalysisRunEnvelope,
  AppliedOperationReceipt,
  OperationOutcome,
  EditorOperationIntent,
  EntityRef,
  LocalProjectEnvelope,
  MechanicsResult,
  ModelHashEvidence,
  ProjectEnvelopeHashEvidence,
  PreviewModel,
  SolveJobAuditState
} from "../../types";
import { defaultSelection } from "../model-workspace/modelView";
import { intentKey } from "../operations/OperationApplyPanel";
import { buildReportPackageRequest } from "../report/reportPackageRequest";
import { controlReportPackageRequest } from "../report/reportRedactionProjector";
import { buildBundledReferenceContext, buildHistoricalRunContext } from "../results/HistoricalRunContext";
import type { HistoricalRunContext } from "../results/HistoricalRunContext";
import {
  resolveDiagnosticEntitySelection,
  resolveEntitySelection
} from "../results/resultInterpretation";
import type { RuleCheckRunBasis } from "../rule-check/RuleCheckRunPanel";
import type { QueuedBatch } from "../toolkit/BatchReviewPanel";
import { capabilityAvailability, capabilityRoute } from "../toolkit/capabilityCatalog";
import type { ToolkitCapability } from "../toolkit/capabilityCatalog";
import type { CreationTool, ViewportExposureInteraction } from "../viewport/PipeViewport";
import {
  applyResultMatchesSubmission,
  sameSubmission,
  validationMatchesSubmission
} from "../viewport/routeDraft";
import type { DraftSubmission, FrozenDraftReview } from "../viewport/routeDraft";
import { deriveViewportVisibility } from "../viewport/viewportVisibility";
import { useChromeSessionState } from "./chromeSessionState";
import type { R3JourneyEvent } from "./chromeSessionState";
import { isMenuCommandId } from "./menuCommands";
import type { MenuCommandId } from "./menuCommands";
import { modelIndexFor } from "./modelIndex";
import { useModelSessionState } from "./modelSessionState";
import { useOperationsSessionState } from "./operationsSessionState";
import {
  canPublishPersistenceObservation,
  ownsOpenPersistenceObservation,
  verifiedWriteBasis, isLocalModelEdited,
  deriveModelHashIntegrity,
  deriveProjectEnvelopeHashIntegrity,
  isSupportedChangedModelPersistenceResponse
} from "./projectPersistenceIntegrity";
import { persistenceHashesMatch } from "./savedModelBasis";
import { useProjectSessionState } from "./projectSessionState";
import { useResultsSessionState } from "./resultsSessionState";
import { useSelectionSessionState } from "./selectionSessionState";
import {
  applyBoxSelection,
  applyDisplayedRange,
  applySelection,
  emptySelection,
  primarySelection,
  pruneSelection,
  sameSelection,
  singletonSelection
} from "./selectionState";
import type { EntityKey, OrderedSelectionState, SelectionModifiers } from "./selectionState";
import {
  clonePreviewModel,
  selectionForOperationOutcome,
  uiModelIdentityHash
} from "./sessionModel";
import { commitModelAfterSolveInvalidation } from "./solveGates";
import {
  awaitBackendSolveJob,
  blankProjectCreatedSolveJob,
  cancelledBeforeBackendStartSolveJob,
  cancelledSolveJob,
  cancelledWithoutBackendSolveJob,
  completeSolveJob,
  failSolveJob,
  initialSolveJob,
  modelChangedSolveJob,
  pendingBackendStartCancellationSolveJob,
  recordBackendCancellationFailure,
  recordBackendCancellationReceipt,
  requestSolveCancellation,
  sessionHistoryChangedSolveJob,
  startSolveJob
} from "./solveJobAudit";
import { publishUiModelAssignmentStarted } from "./uiDiagnostics";
import {
  EMPTY_STAGE_VIEW_MEMORY,
  SHELL_STAGES,
  SHELL_VIEWS,
  canvasAuthoringPanelActive,
  isStageSurface,
  railStageState,
  rememberStageView,
  runPresenceFromCells,
  sectionAfterPageClose,
  sectionForStage,
  shellLocation,
  stageSurfaceAfter,
  viewForStage
} from "./shellLayout";
import type { ShellStage, ShellView } from "./shellLayout";
import type { UiDensityPreference, UiThemePreference } from "./uiPreferences";
import { updateUiPreferences, writeUiPreferences } from "./uiPreferences";
import { observeWorkspaceCanvasBudget } from "./workspaceCanvasBudget";
import { EXPENSIVE_LIFECYCLE_SECTIONS } from "./workspaceSections";
import type { WorkspaceSectionId } from "./workspaceSections";

// Diagnostics generations must not repeat across real same-page App remounts.
let nextUiModelPublicationGeneration = 0;

function formatPackageSaveError(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (error && typeof error === "object") {
    try {
      return JSON.stringify(error);
    } catch {
      return "REPORT-PACKAGE-SAVE-FAILED: native error was not serializable";
    }
  }
  return String(error);
}

/**
 * The workspace session: the six state hooks, all thirteen effects and every
 * handler, in one closure and in the order `AppSession` declared them.
 * `AppSession` in `App.tsx` calls it once and renders what it returns; nothing
 * else calls it.
 *
 * The order is kept because it carries meaning. The results hook bumps the two
 * basis refs during render, before `ruleCheckRunBasis` and `dormantOutputBasis`
 * read them. The layout effect on `[model]` advances `modelRevision` and
 * invalidates the solve gate. The mount effect keeps the first render's closure
 * on purpose. `runMenuCommandRef` is assigned on every render, so the native
 * menu listener, registered once, runs the current `runMenuCommand`.
 * `nextUiModelPublicationGeneration` is module state beside `commitModel`, so a
 * generation does not repeat across remounts.
 *
 * It returns six slices: `model`, `selection`, `results`, `operations`,
 * `project` and `chrome`. Each holds only what the view reads. Apart from the
 * chrome setters, the setters handed out are the two visibility sets
 * (`setHiddenEntityKeys`, `setIsolationSelectionKeys`) and the two run options
 * (`setSolverMode`, `setReportPackagePrivateIntent`). No setter of the model,
 * of a result, of an operations cell or of a project cell leaves the session:
 * those change only in its effects and handlers, where the stale-response
 * guards and the paths that clear computed state sit together.
 */
export function useWorkspaceSession() {
  const {
    uiPreferences, setUiPreferences,
    setSystemDark,
    resolvedTheme,
    activeSection, setActiveSection,
    activatedExpensiveSections, setActivatedExpensiveSections,
    toolkitFocus, setToolkitFocus,
    propertyTaskRequestSequenceRef,
    propertyTaskRequest, setPropertyTaskRequest,
    openMenu, setOpenMenu,
    armedCreationTool, setArmedCreationTool,
    viewportViewCommandRef,
    workspaceShellRef,
    workspaceBudgetRef,
    narrowWindow, setNarrowWindow,
    treeCollapsed, setTreeCollapsed,
    inspectorCollapsed, setInspectorCollapsed,
    treeToggleRef,
    inspectorToggleRef,
    activeResizeCleanupRef,
    operationTab, setOperationTab,
    setR3JourneyState,
    reviewDetailsOpen, setReviewDetailsOpen,
    auditDrawerOpen, setAuditDrawerOpen,
    issuesDrawerOpen, setIssuesDrawerOpen,
    stageSurface, setStageSurface,
    stageViewMemory, setStageViewMemory
  } = useChromeSessionState();
  const {
    model, setModel,
    knowledge, setKnowledge,
    projectSessionGeneration, setProjectSessionGeneration,
    projectSessionGenerationRef,
    uiModelRevision, setUiModelRevision,
    uiModelRevisionRef,
    modelPublicationGenerationRef,
    modelAssignment, setModelAssignment,
    modelHash, setModelHash,
    modelRevision,
    currentModel,
    activeModelIndex
  } = useModelSessionState();
  // Chrome-only canonical comparison. A revision shortcut is used only for a
  // verified snapshot, or the untouched initial loaded-source snapshot (which
  // makes no claim of a file save). Later revisions need a current owned hash.
  const [savedModelBasis, setSavedModelBasis] = useState<{
    generation: number; revision: number; hash: string | null;
    source: "loaded-source" | "open" | "create" | "save";
  } | null>(null);
  const savedBasisSequence = useRef(0);
  // Service fulfillment is our observable landed-write order, before canonical
  // verification can complete out of order. This is not a backend commit clock.
  const persistedWriteObservation = useRef(0);
  const latestVerifiedWriteObservation = useRef(0);
  const modelHashOwner = useRef<{ generation: number; revision: number } | null>(null);
  const currentHashOwned = modelHashOwner.current?.generation === projectSessionGeneration &&
    modelHashOwner.current?.revision === uiModelRevision;
  const modelEdited = Boolean(model && isLocalModelEdited(savedModelBasis, projectSessionGeneration,
    uiModelRevision, currentHashOwned, modelHash));

  const {
    selection, setPrimarySelection,
    orderedSelection, setOrderedSelection,
    orderedSelectionRef,
    hiddenEntityKeys, setHiddenEntityKeys,
    isolationSelectionKeys, setIsolationSelectionKeys,
    treePublication,
    handleTreePublication,
    selectedPipeRefs
  } = useSelectionSessionState();
  const {
    result, setResult,
    historicalRun, setHistoricalRun,
    currentSolvedResult,
    analysisRun, setAnalysisRun,
    resultBasisRef,
    analysisBasisRef,
    inputManifest, setInputManifest,
    ruleCheckAggregate, setRuleCheckAggregate,
    proposal, setProposal,
    selectedReviewTarget, setSelectedReviewTarget,
    solveJob, setSolveJob,
    solveProof, setSolveProof,
    running, setRunning,
    solverMode, setSolverMode,
    reportPackagePrivateIntent, setReportPackagePrivateIntent,
    reportPackageBusy, setReportPackageBusy,
    reportPackageRedaction, setReportPackageRedaction,
    reportPackageRoute, setReportPackageRoute,
    reportPackageRequestGenerationRef,
    reportPackageBusyGenerationRef,
    solveRunGate,
    ruleRevisionGate,
    currentSolvedResultRef,
    currentInputManifestRef,
    activeSolveJob,
    solveCancellationTombstones,
    comparison
  } = useResultsSessionState();
  // Reference inspection is a transient view. Keep genuine persistence bytes
  // separate, scoped to the unchanged model/session, without retaining Current.
  const referencePersistence = useRef<{
    model: PreviewModel;
    generation: number;
    modelContent: string;
    history: HistoricalRunContext | null;
    result: MechanicsResult | null;
    analysisRun: AnalysisRunEnvelope | null;
  } | null>(null);
  function retainedReferencePersistence(requestModel: PreviewModel) {
    const retained = referencePersistence.current;
    try {
      return retained?.model === requestModel && retained.generation === projectSessionGenerationRef.current
        && retained.modelContent === checkedJsonText(requestModel) ? retained : null;
    } catch { return null; }
  }
  const {
    editorIntents, setEditorIntents,
    retainedReviewContext, setRetainedReviewContext,
    operationOutcomes, setOperationOutcomes,
    appliedOperations, setAppliedOperations,
    undoStack, setUndoStack,
    redoStack, setRedoStack,
    queuedBatches, setQueuedBatches,
    batchOutcomes, setBatchOutcomes,
    batchReceipts, setBatchReceipts,
    batchMessage, setBatchMessage,
    requestEpoch, setRequestEpoch,
    requestEpochRef,
    getPreparationEpoch,
    directDraftCommitToken, setDirectDraftCommitToken,
    batchSequence,
    operationBusy, setOperationBusy,
    operationMessage, setOperationMessage,
    operationEngineStatus, setOperationEngineStatus,
    intentSequence,
    operationRequest,
    directDraftReviews,
    directDraftReviewSequence
  } = useOperationsSessionState();
  const {
    projectRequest, projectOperationOwner,
    storageCapability, setStorageCapability,
    projectSummary, setProjectSummary,
    projectIndex, setProjectIndex,
    modelHashIntegrity, setModelHashIntegrity,
    projectEnvelopeHash, setProjectEnvelopeHash,
    modelDocumentMigration, setModelDocumentMigration,
    modelMigrationLedger, setModelMigrationLedger,
    projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity,
    projectMessage, setProjectMessage,
    projectOperation, setProjectOperation,
    projectBusy, setProjectBusy
  } = useProjectSessionState();

  const ruleCheckRunBasis: RuleCheckRunBasis = {
    projectSessionGeneration,
    modelRevision: uiModelRevision,
    resultSequence: resultBasisRef.current.sequence
  };
  // Dormant output panels receive this key only when they reconcile again.
  // Thus accepted model/result changes do not wake an inactive full-model scan,
  // while the next activation remounts local packet/route state before paint.
  const dormantOutputBasis = `${uiModelRevision}:${resultBasisRef.current.sequence}:${analysisBasisRef.current.sequence}`;
  useEffect(() => {
    if (!activeSection || !EXPENSIVE_LIFECYCLE_SECTIONS.has(activeSection)) return;
    setActivatedExpensiveSections((current) => {
      if (current.has(activeSection)) return current;
      return new Set([...current, activeSection]);
    });
  }, [activeSection]);
  // The stage surface follows the one navigation cell: a page leaves it alone,
  // anything else becomes it. A presentation cell only; it reads and writes
  // nothing of the model, the results, the operations or the project.
  useEffect(() => {
    setStageSurface((current) => stageSurfaceAfter(current, activeSection));
    // A section summoned into the table pane is meant to be read: where the pane
    // is a collapsed drawer, summoning one opens the drawer.
    if (activeSection !== null && isStageSurface(activeSection)) setTreeCollapsed(false);
  }, [activeSection]);
  // Where the shell is: derived on every render, never stored. The ref serves
  // the listeners that are registered once (the keys, the native menu).
  const shellNow = shellLocation(activeSection, stageSurface);
  const stageViewNow = viewForStage(stageViewMemory, shellNow.stage);
  // Routing temporarily borrows the inspector without changing its remembered
  // open/closed state. A queued viewport intent keeps the same routing surface.
  const routingPanelActive = canvasAuthoringPanelActive(armedCreationTool, editorIntents);
  const routingInspectorRestore = useRef<boolean | null>(null);
  useLayoutEffect(() => {
    if (routingPanelActive) {
      if (routingInspectorRestore.current === null) routingInspectorRestore.current = inspectorCollapsed;
      setInspectorCollapsed(false);
    } else if (routingInspectorRestore.current !== null) {
      const restoreCollapsed = routingInspectorRestore.current;
      routingInspectorRestore.current = null;
      if (restoreCollapsed && document.getElementById("shell-inspector")?.contains(document.activeElement)) {
        workspaceShellRef.current?.querySelector<HTMLButtonElement>('[data-testid="workspace-select"]')?.focus();
      }
      setInspectorCollapsed(restoreCollapsed);
    }
  }, [routingPanelActive]);
  const shellNowRef = useRef({ location: shellNow, view: stageViewNow, stageSurface });
  shellNowRef.current = { location: shellNow, view: stageViewNow, stageSurface };
  const pageReturnFocusRef = useRef<HTMLElement | null>(null);
  const previousPageRef = useRef(shellNow.page);
  useLayoutEffect(() => {
    const wasOpen = previousPageRef.current !== null;
    previousPageRef.current = shellNow.page;
    if (!wasOpen || shellNow.page) return;
    const shell = workspaceShellRef.current;
    if (!shell) return;
    const available = (element: HTMLElement | null): element is HTMLElement => {
      if (!element?.isConnected || element.closest('[inert], [hidden], [aria-hidden="true"]') ||
          element.matches(":disabled") || element.getAttribute("aria-disabled") === "true") return false;
      const box = element.getBoundingClientRect();
      return box.width > 0 && box.height > 0 && getComputedStyle(element).visibility !== "hidden";
    };
    // A deliberate navigation control may already own visible focus. Otherwise
    // restore the pre-page opener, or a persistent visible stage/Select control.
    const focused = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    if (focused !== document.body && available(focused)) return;
    const preferred = pageReturnFocusRef.current;
    const stage = shell.querySelector<HTMLElement>(`[data-testid="rail-stage-${shellNow.stage}"]`);
    const select = shell.querySelector<HTMLElement>('[data-testid="workspace-select"]');
    (available(preferred) ? preferred : available(stage) ? stage : select)?.focus();
  }, [shellNow.page, shellNow.stage]);
  useEffect(() => {
    const update = () => setNarrowWindow(window.innerWidth < 1280);
    window.addEventListener("resize", update);
    return () => window.removeEventListener("resize", update);
  }, []);
  useLayoutEffect(() => {
    if (!toolkitFocus) return;
    const target = toolkitFocus.elementId ? document.getElementById(toolkitFocus.elementId) : document.querySelector<HTMLElement>(`[data-testid="${toolkitFocus.testId}"]`);
    // Reveal contextual details before transferring focus from command search.
    for (let parent = target?.parentElement; parent; parent = parent.parentElement) {
      if (parent instanceof HTMLDetailsElement) parent.open = true;
    }
    target?.focus();
    target?.scrollIntoView?.({ block: "nearest" });
  }, [toolkitFocus]);
  useLayoutEffect(() => {
    const workspace = workspaceBudgetRef.current;
    if (!model || !workspace) return;
    return observeWorkspaceCanvasBudget(workspace);
  }, [Boolean(model)]);
  const viewportVisibility = useMemo(
    () => activeModelIndex
      ? deriveViewportVisibility(activeModelIndex, hiddenEntityKeys, isolationSelectionKeys)
      : { hiddenKeys: new Set<EntityKey>(), dimmedKeys: new Set<EntityKey>(), hiddenCount: 0, isolationActive: isolationSelectionKeys !== null },
    [activeModelIndex, hiddenEntityKeys, isolationSelectionKeys]
  );

  function handleClearVisibility() {
    setHiddenEntityKeys(new Set());
    setIsolationSelectionKeys(null);
  }

  useEffect(() => {
    if (!activeModelIndex) return;
    setHiddenEntityKeys((current) => {
      const next = new Set([...current].filter((key) => activeModelIndex.entities.has(key)));
      return next.size === current.size ? current : next;
    });
    setIsolationSelectionKeys((current) => {
      if (current === null) return current;
      const next = new Set([...current].filter((key) => activeModelIndex.entities.has(key)));
      return next.size === current.size ? current : next;
    });
  }, [activeModelIndex]);

  useEffect(() => {
    writeUiPreferences(uiPreferences);
  }, [uiPreferences]);

  useEffect(() => () => activeResizeCleanupRef.current?.(), []);

  // The design tokens (tokens.css) are scoped on :root[data-theme]; the document root follows the
  // resolved theme so they hold for the shell and for anything rendered outside it.
  useLayoutEffect(() => {
    const root = document.documentElement;
    const previous = root.getAttribute("data-theme");
    root.setAttribute("data-theme", resolvedTheme);
    return () => {
      if (previous === null) root.removeAttribute("data-theme");
      else root.setAttribute("data-theme", previous);
    };
  }, [resolvedTheme]);

  useEffect(() => {
    if (typeof window.matchMedia !== "function") return;
    const query = window.matchMedia("(prefers-color-scheme: dark)");
    const update = () => setSystemDark(query.matches);
    query.addEventListener?.("change", update);
    return () => query.removeEventListener?.("change", update);
  }, []);

  function commitSelectionState(next: OrderedSelectionState, fallbackModel: PreviewModel | null = model): boolean {
    const previous = orderedSelectionRef.current;
    const membershipChanged = !sameSelection(previous, next);
    if (!membershipChanged && previous.focusKey === next.focusKey && previous.rangeAnchorKey === next.rangeAnchorKey) return false;
    orderedSelectionRef.current = next;
    setOrderedSelection(next);
    if (membershipChanged) {
      invalidateDirectDraftContext();
      if (fallbackModel) setPrimarySelection(primarySelection(next, defaultSelection(fallbackModel)));
      else setPrimarySelection(next.primaryKey ? primarySelection(next, { type: "project", id: "" }) : null);
    }
    return membershipChanged;
  }

  function setSelection(next: EntityRef | null): void {
    if (!next) {
      const previous = orderedSelectionRef.current;
      const empty = emptySelection(orderedSelectionRef.current.preparationEpoch + 1);
      orderedSelectionRef.current = empty;
      setOrderedSelection(empty);
      setPrimarySelection(null);
      if (!sameSelection(previous, empty)) invalidateDirectDraftContext();
      return;
    }
    commitSelectionState(singletonSelection(next, orderedSelectionRef.current));
  }

  useEffect(() => {
    let active = true;
    Promise.all([loadPreviewModel(), loadDesignKnowledge(), getLocalStorageCapability()]).then(
      ([loadedModel, loadedKnowledge, loadedStorageCapability]) => {
        if (!active) return;
        advanceProjectSession();
        commitModel(loadedModel);
        const generation = projectSessionGenerationRef.current;
        const revision = uiModelRevisionRef.current;
        const sequence = ++savedBasisSequence.current;
        setSavedModelBasis({ generation, revision, hash: null, source: "loaded-source" });
        computeModelHash(loadedModel).then((hash) => {
          if (active && generation === projectSessionGenerationRef.current && sequence === savedBasisSequence.current) {
            setSavedModelBasis({ generation, revision, hash: hash?.value ?? null, source: "loaded-source" });
          }
        }).catch(() => {
          // Keep only the untouched loaded-source revision. No saved-file claim
          // or canonical equality for a later revision follows from this failure.
        });
        setKnowledge(loadedKnowledge);
        setSelection(defaultSelection(loadedModel));
        setStorageCapability(loadedStorageCapability);
      }
    );
    return () => {
      active = false;
    };
  }, []);

  useLayoutEffect(() => {
    let active = true;
    const retained = referencePersistence.current;
    if (retained && (retained.model !== model || retained.generation !== projectSessionGenerationRef.current)) referencePersistence.current = null;
    modelRevision.current += 1;
    solveRunGate.current.invalidate();
    activeSolveJob.current = null;
    setRunning(false);
    setSolveProof(null);
    setModelHash(null);
    if (!model) return;
    const generation = projectSessionGenerationRef.current;
    const revision = uiModelRevisionRef.current;
    computeModelHash(model).then((hash) => {
      if (active && currentModel.current === model && generation === projectSessionGenerationRef.current && revision === uiModelRevisionRef.current) {
        modelHashOwner.current = { generation, revision };
        setModelHash(hash);
      }
    }).catch(() => { /* An unavailable current hash cannot prove a clean comparison. */ });
    return () => {
      active = false;
    };
  }, [model]);

  function commitModel(nextModel: PreviewModel, directDraftToken: string | null = null) {
    referencePersistence.current = null;
    modelHashOwner.current = null;
    viewportViewCommandRef.current?.({ type: "retire-box-gesture" });
    invalidateReportPackageComputedState();
    const assignmentStartedAt = performance.now();
    const modelPublicationGeneration = ++nextUiModelPublicationGeneration;
    publishUiModelAssignmentStarted(modelPublicationGeneration, assignmentStartedAt);
    const nextUiModelRevision = uiModelRevisionRef.current + 1;
    const indexGeneration = `${projectSessionGenerationRef.current}:${nextUiModelRevision}`;
    const identityHash = uiModelIdentityHash(nextModel);
    setModelAssignment({
      status: "started",
      generation: modelPublicationGeneration,
      indexGeneration,
      identityHash,
      startedAt: assignmentStartedAt,
      committedAt: null
    });
    requestEpochRef.current += 1;
    setRequestEpoch(requestEpochRef.current);
    setDirectDraftCommitToken(directDraftToken);
    uiModelRevisionRef.current = nextUiModelRevision;
    setUiModelRevision(nextUiModelRevision);
    currentModel.current = nextModel;
    setHistoricalRun(null);
    operationRequest.current.sequence += 1;
    operationRequest.current.busy = false;
    directDraftReviews.current.clear();
    setOperationBusy(false);
    commitModelAfterSolveInvalidation(solveRunGate.current, modelRevision, () => {
      modelPublicationGenerationRef.current = modelPublicationGeneration;
      activeSolveJob.current = null;
      setRunning(false);
      setSolveProof(null);
      setModelHash(null);
      setModelAssignment({
        status: "committed",
        generation: modelPublicationGeneration,
        indexGeneration,
        identityHash,
        startedAt: assignmentStartedAt,
        committedAt: performance.now()
      });
      setModel(nextModel);
    });
  }

  function preserveAndPruneSelection(nextModel: PreviewModel) {
    const nextIndex = modelIndexFor(
      nextModel,
      projectSessionGenerationRef.current,
      uiModelRevisionRef.current
    );
    commitSelectionState(
      pruneSelection(
        orderedSelectionRef.current,
        new Set(nextIndex.entities.keys()),
        { type: "project", id: nextModel.project.id }
      ),
      nextModel
    );
  }

  function advanceProjectSession(): void {
    viewportViewCommandRef.current?.({ type: "cancel-box-selection" });
    invalidateReportPackageComputedState();
    const nextGeneration = ++projectSessionGenerationRef.current;
    setProjectSessionGeneration(nextGeneration);
    setHiddenEntityKeys(new Set());
    setIsolationSelectionKeys(null);
    setPropertyTaskRequest(null);
  }

  // Warm up the operation engine and report its honest route/readiness.
  // Browser mode lazily loads the wasm32 operation_applier build; an absent
  // artifact surfaces as an explicit unavailable status, never a fallback.
  useEffect(() => {
    let active = true;
    warmupOperationEngine().then((status) => {
      if (active) setOperationEngineStatus(status);
    });
    return () => {
      active = false;
    };
  }, []);

  function dispatchActiveSolveCancellation(runGeneration: number) {
    const active = activeSolveJob.current;
    const tombstone = solveCancellationTombstones.current.get(runGeneration);
    if (
      !active ||
      active.generation !== runGeneration ||
      active.cancellation_dispatched ||
      !tombstone?.requested ||
      tombstone.dispatched ||
      !solveRunGate.current.isCurrent(runGeneration) ||
      active.job.backend_job_seam !== "tauri_backend_job" ||
      !active.job.backend_job_id
    ) return;
    active.cancellation_dispatched = true;
    tombstone.dispatched = true;
    void cancelPreviewMechanicsJob(
      active.job.backend_job_id,
      active.job.backend_cancellation_token
    )
      .then((receipt) => {
        if (
          !solveRunGate.current.isCurrent(runGeneration) ||
          receipt.job_id !== active.job.job_id
        ) return;
        setSolveJob((current) =>
          current.job_id === active.job.job_id
            ? recordBackendCancellationReceipt(current, receipt)
            : current
        );
      })
      .catch((error) => {
        if (!solveRunGate.current.isCurrent(runGeneration)) return;
        setSolveJob((current) =>
          current.job_id === active.job.job_id
            ? recordBackendCancellationFailure(current, error)
            : current
        );
      });
  }

  async function handleInspectBundledReference() {
    if (!model) return;
    if (result || analysisRun || historicalRun?.designation === "historical_saved_run"
      || !retainedReferencePersistence(model)) {
      referencePersistence.current = {
        model, generation: projectSessionGenerationRef.current, modelContent: checkedJsonText(model),
        history: historicalRun?.designation === "historical_saved_run" ? structuredClone(historicalRun) : null,
        result: result ? structuredClone(result) : null,
        analysisRun: analysisRun ? structuredClone(analysisRun) : null,
      };
    }
    // Retire any pending backend publication before loading the separate example.
    handleCancelRun();
    solveRunGate.current.invalidate();
    activeSolveJob.current = null;
    const referenceGeneration = solveRunGate.current.tryStart()!;
    setRunning(false);
    setSolveProof(null);
    invalidateReportPackageComputedState();
    ruleRevisionGate.current.invalidate();
    setHistoricalRun(null);
    setResult(null); setAnalysisRun(null); setInputManifest(null);
    setRuleCheckAggregate(null); setSolveJob(initialSolveJob());
    // Review proposals and selected review context remain the user's work;
    // only current mechanics qualification is retired by reference inspection.
    try {
      const reference = await loadBundledMechanicsReference(solverMode);
      if (!solveRunGate.current.isCurrent(referenceGeneration)) return;
      setHistoricalRun(buildBundledReferenceContext(reference));
      setActiveSection("results");
    } finally {
      solveRunGate.current.finish(referenceGeneration);
    }
  }

  async function handleRun() {
    const runGeneration = solveRunGate.current.tryStart();
    if (runGeneration === null) return;
    invalidateReportPackageComputedState();
    ruleRevisionGate.current.invalidate();
    solveCancellationTombstones.current.set(runGeneration, {
      requested: false,
      dispatched: false
    });
    setRunning(true);
    setSolveProof(null);
    setResult(null);
    setAnalysisRun(null);
    setInputManifest(null);
    // A fresh solve invalidates any prior rule-check run against the old result.
    setRuleCheckAggregate(null);
    let startedJob: SolveJobAuditState | null = null;
    try {
      if (!model) {
        throw new Error(
          "INPUT-MANIFEST-MODEL-INCOMPLETE: a current session model is required before solve."
        );
      }
      const solveModel = clonePreviewModel(model);
      const solveModelRevision = modelRevision.current;
      const solveModelHash = await computeModelHash(solveModel);
      if (
        !solveModelHash ||
        modelRevision.current !== solveModelRevision ||
        !solveRunGate.current.isCurrent(runGeneration)
      ) return;
      if (
        solveRunGate.current.isCancellationRequested(runGeneration) ||
        solveCancellationTombstones.current.get(runGeneration)?.requested
      ) {
        setSolveJob(cancelledBeforeBackendStartSolveJob(solveModel));
        return;
      }
      const startReceipt = await startPreviewMechanicsJob(solveModel, solverMode);
      // Reference records never create a fresh solve. Only a real backend job
      // can publish current result/manifest/proof cells through this path.
      if (startReceipt.mode !== "backend_job") {
        throw new Error("SOLVE_BACKEND_INVOCATION_REQUIRED: a reference-only receipt cannot publish Current mechanics.");
      }
      const cancellationTombstone = solveCancellationTombstones.current.get(runGeneration);
      if (
        modelRevision.current !== solveModelRevision ||
        !solveRunGate.current.isCurrent(runGeneration)
      ) {
        if (
          cancellationTombstone?.requested &&
          !cancellationTombstone.dispatched &&
          startReceipt.mode === "backend_job"
        ) {
          cancellationTombstone.dispatched = true;
          void cancelPreviewMechanicsJob(
            startReceipt.job_id,
            startReceipt.backend_cancellation_token
          ).catch(() => {
            // The originating generation is already detached from visible UI.
            // A failed best-effort cancellation must not resurrect stale state.
          });
        }
        return;
      }
      startedJob = startSolveJob(solveModel, startReceipt);
      const cancellationRequested =
        solveRunGate.current.isCancellationRequested(runGeneration) ||
        cancellationTombstone?.requested === true;
      const visibleStartedJob = cancellationRequested
        ? requestSolveCancellation(startedJob)
        : startedJob;
      activeSolveJob.current = {
        generation: runGeneration,
        job: visibleStartedJob,
        cancellation_dispatched: false
      };
      setSolveJob(visibleStartedJob);
      if (cancellationRequested) {
        if (startReceipt.mode === "backend_job") {
          dispatchActiveSolveCancellation(runGeneration);
        } else {
          setSolveJob(cancelledWithoutBackendSolveJob(visibleStartedJob));
          return;
        }
      }
      let output: MechanicsResult;
      if (startReceipt.mode === "backend_job") {
        const terminal = await awaitBackendSolveJob(startReceipt.job_id);
        if (!solveRunGate.current.isCurrent(runGeneration)) return;
        if (terminal.job_id !== startedJob.job_id) {
          throw new Error(
            `SOLVE-JOB-IDENTITY-MISMATCH: expected ${startedJob.job_id}; received ${terminal.job_id}`
          );
        }
        if (terminal.state === "cancelled") {
          setSolveJob((current) =>
            current.job_id === startedJob?.job_id
              ? cancelledSolveJob(current, terminal)
              : current
          );
          return;
        }
        if (terminal.state !== "completed" || !terminal.result) {
          throw new Error(
            terminal.error_message ?? `backend solve job ${terminal.job_id} ended as ${terminal.state} without a result`
          );
        }
        output = terminal.result;
      } else {
        throw new Error("SOLVE_BACKEND_INVOCATION_REQUIRED: browser references are not completed solve results.");
      }
      if (!hasNativeMechanicsInvocation(output, solveModel, solverMode)) {
        throw new Error("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED: the received result is not bound to this actual native model/mode invocation.");
      }
      if (!hasCurrentSourceContract(output) || !output.producer) {
        throw new Error("SOLVE-PRODUCER-CONTRACT-UNSUPPORTED: a fresh result requires a recognized current producer contract; historical and unsupported carriers remain available only through their inspection routes.");
      }
      const manifest = await buildCurrentSessionInputManifest({
        model: solveModel,
        solver: {
          solver_name: output.producer.component_name,
          solver_version: output.producer.component_version,
          solver_build_ref: `${output.producer.component_name}@${output.producer.component_version}`,
          solver_mode: solverMode,
          settings: {
            nonlinear_iteration_policy:
              "DEC-046-CV-B-product-preview-active-set-count-v1",
            sparse_evidence_lane: solverMode === "sparse_interactive"
          }
        },
        active_rule_packs: [],
        external_assets: []
      });
      if (!solveRunGate.current.isCurrent(runGeneration)) return;
      const runRecord = await buildAnalysisRunPreview(output, {
        inputManifest: manifest
      });
      if (
        modelRevision.current !== solveModelRevision ||
        !solveRunGate.current.isCurrent(runGeneration)
      ) return;
      if (output.model_ref !== solveModel.project.id) {
        throw new Error(
          `SOLVE-RESULT-MODEL-MISMATCH: expected ${solveModel.project.id}; received ${output.model_ref}`
        );
      }
      setSolveJob((current) =>
        current.job_id === startedJob?.job_id
          ? completeSolveJob(current, output, runRecord)
          : current
      );
      referencePersistence.current = null;
      setHistoricalRun(null);
      setResult(output);
      setSelectedReviewTarget(null);
      setProposal(null);
      setInputManifest(manifest);
      setAnalysisRun(runRecord);
      setModelHash(solveModelHash);
      setSolveProof({
        state: "completed",
        run_generation: runGeneration,
        job_id: startedJob.job_id,
        backend_job_seam: startedJob.backend_job_seam,
        project_ref: solveModel.project.id,
        model_sha256: solveModelHash.value,
        input_manifest_sha256: manifest.manifest_sha256,
        result_run_id: output.run_id,
        result_model_ref: output.model_ref,
        result_row_count: output.results.length
      });
    } catch (error) {
      if (solveRunGate.current.isCurrent(runGeneration)) {
        setSolveJob((current) =>
          !startedJob || current.job_id === startedJob.job_id
            ? failSolveJob(current, error)
            : current
        );
      }
    } finally {
      solveCancellationTombstones.current.delete(runGeneration);
      if (activeSolveJob.current?.generation === runGeneration) {
        activeSolveJob.current = null;
      }
      if (solveRunGate.current.finish(runGeneration)) setRunning(false);
    }
  }

  // Record (or clear) the GUI rule-check aggregate in the app-held analysis-run
  // envelope (TP-C4-APPAGG-001). Rebuilding re-derives only the
  // analysis_run_record status/hash and analysis_status; the embedded
  // result_envelope hash still binds the raw solve, so the hash-bound solve
  // envelope is never mutated. With no solved result there is no app-held
  // envelope to annotate.
  async function handleRuleCheckAggregate(
    aggregate: RuleCheckStatus | null,
    renderedBasis: Readonly<{
      projectSessionGeneration: number;
      modelRevision: number;
      result: MechanicsResult | null;
      inputManifest: CurrentSessionInputManifestEvidence | null;
    }>
  ) {
    const renderedBasisIsCurrent = () =>
      projectSessionGenerationRef.current === renderedBasis.projectSessionGeneration &&
      uiModelRevisionRef.current === renderedBasis.modelRevision &&
      currentSolvedResultRef.current === renderedBasis.result &&
      currentInputManifestRef.current === renderedBasis.inputManifest;
    if (!renderedBasisIsCurrent()) return;
    if (aggregate === ruleCheckAggregate) return;
    const previousAggregate = ruleCheckAggregate;
    const revision = ruleRevisionGate.current.start();
    // A different aggregate immediately makes the previously assembled report
    // package stale. Clear its computed route before publishing the new rule
    // state; the private-package intent remains an independent user choice.
    invalidateReportPackageComputedState();
    setRuleCheckAggregate(aggregate);
    if (!currentSolvedResult || !inputManifest) return;
    const capturedResult = currentSolvedResult;
    const capturedManifest = inputManifest;
    const capturedModelRevision = modelRevision.current;
    const capturedSolveGeneration = solveRunGate.current.current();
    const stillCurrent = () =>
      ruleRevisionGate.current.isCurrent(revision) &&
      modelRevision.current === capturedModelRevision &&
      solveRunGate.current.current() === capturedSolveGeneration &&
      currentSolvedResultRef.current === capturedResult &&
      currentInputManifestRef.current === capturedManifest &&
      currentModel.current?.project.id === capturedResult.model_ref &&
      renderedBasisIsCurrent();
    try {
      const revisedRecord = await buildAnalysisRunPreview(capturedResult, {
          inputManifest: capturedManifest,
          ruleCheckAggregate: aggregate
      });
      if (!stillCurrent()) return;
      setAnalysisRun(revisedRecord);
    } catch {
      if (!stillCurrent()) return;
      // Recording the aggregate failed (e.g. hashing unavailable); keep the
      // solve-time analysis-run envelope rather than surfacing a false outcome.
      setRuleCheckAggregate(previousAggregate);
    }
  }

  function recordR3JourneyEvent(event: R3JourneyEvent) {
    setR3JourneyState((current) => {
      if (current[event]) return current;
      return { ...current, [event]: true };
    });
  }

  function handleCancelRun() {
    const runGeneration = solveRunGate.current.current();
    if (runGeneration === null) return;
    if (!solveRunGate.current.requestCancellation(runGeneration)) return;
    const tombstone = solveCancellationTombstones.current.get(runGeneration);
    if (tombstone) tombstone.requested = true;
    const active = activeSolveJob.current;
    if (!active || active.generation !== runGeneration) {
      setSolveJob((current) => pendingBackendStartCancellationSolveJob(current));
      return;
    }
    active.job = requestSolveCancellation(active.job);
    setSolveJob((current) =>
      current.job_id === active.job.job_id
        ? requestSolveCancellation(current)
        : current
    );
    if (active.job.backend_job_seam === "tauri_backend_job") {
      dispatchActiveSolveCancellation(runGeneration);
    } else {
      setSolveJob(cancelledWithoutBackendSolveJob(active.job));
    }
  }

  async function handleProposal() {
    setProposal(await loadSampleProposal(result, selectedReviewTarget));
  }

  function handleQueueEditorIntent(intent: EditorOperationIntent) {
    intentSequence.current += 1;
    setEditorIntents((current) => [
      {
        ...intent,
        queue_id: `editor-intent-${intentSequence.current}`
      },
      ...current
    ]);
    setActiveSection("operations");
    setOperationTab("review");
  }

  function handleClearReviewQueue() {
    requestEpochRef.current += 1;
    setRequestEpoch(requestEpochRef.current);
    setQueuedBatches([]);
    setBatchOutcomes({});
    setBatchMessage("Pending batches cleared. Running requests will not change this session.");
    // Withdrawal invalidates callbacks synchronously, including the hash waits
    // in apply. An older response or finally block cannot revive this queue.
    operationRequest.current.sequence += 1;
    operationRequest.current.busy = false;
    setOperationBusy(false);
    setEditorIntents([]);
    setProposal(null);
    setOperationOutcomes({});
    setOperationMessage("Pending changes cleared. Requests already running will not change this session.");
  }

  async function handleQueueOperationBatch(batch: OperationBatch) {
    if (!model) return;
    const revision = modelRevision.current;
    const epoch = requestEpochRef.current;
    const basisModel = clonePreviewModel(model);
    const submitted = structuredClone(batch);
    try {
      const basisHash = await computeModelHash(basisModel);
      if (revision !== modelRevision.current || epoch !== requestEpochRef.current) return;
      if (!basisHash) throw new Error("The current model hash is unavailable.");
      const key = `operation-batch-${++batchSequence.current}`;
      setQueuedBatches((current) => [...current, { key, batch: submitted, basisModel, basisHash, basisRevision: revision }]);
      setBatchMessage("Batch queued for validation and explicit application.");
      setActiveSection("operations");
      setOperationTab("review");
      setToolkitFocus({ testId: "", elementId: "batch-review" });
    } catch (error) {
      if (revision === modelRevision.current && epoch === requestEpochRef.current) {
        setBatchMessage(`Batch could not be queued: ${String(error)}`);
      }
    }
  }

  async function handleRunOperationBatch(entry: QueuedBatch, apply: boolean) {
    if (operationRequest.current.busy || entry.basisRevision !== modelRevision.current) return;
    const revision = modelRevision.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    const stillCurrent = () => operationRequest.current.sequence === request && modelRevision.current === revision;
    setOperationBusy(true);
    setBatchMessage(null);
    try {
      if (!currentModel.current) return;
      const initialHash = await computeModelHash(currentModel.current);
      if (!stillCurrent()) return;
      if (!initialHash || initialHash.value !== entry.basisHash.value) throw new Error("The model changed. Prepare a new batch.");
      const outcome = await (apply ? applyOperationBatch : validateOperationBatch)(entry.basisModel, entry.batch, entry.basisHash);
      if (!stillCurrent() || !currentModel.current) return;
      const currentHash = await computeModelHash(currentModel.current);
      if (!stillCurrent()) return;
      if (
        currentHash?.value !== initialHash.value ||
        outcome.initial_model_hash?.value !== initialHash.value ||
        outcome.batch_id !== entry.batch.batch_id
      ) {
        throw new Error("The batch result does not match the original model basis; it was discarded.");
      }
      if (!apply && (outcome.acceptance || outcome.applied_model || outcome.validation.application_status === "applied_to_session_model")) {
        throw new Error("Validation returned an application result; it was discarded.");
      }
      setBatchOutcomes((current) => ({ ...current, [entry.key]: outcome }));
      if (!apply || outcome.validation.application_status !== "applied_to_session_model") return;
      if (!outcome.applied_model || !outcome.acceptance || outcome.simulation_disposition !== "committed_as_one_batch") {
        throw new Error("The batch did not return a complete application receipt; the model was not changed.");
      }
      setUndoStack((current) => [{
        checkpoint_id: `undo-${entry.key}`,
        operation_id: entry.batch.batch_id,
        model: clonePreviewModel(entry.basisModel),
        selection: selection ?? defaultSelection(entry.basisModel)
      }, ...current].slice(0, 25));
      setRedoStack([]);
      setBatchReceipts((current) => [...current, { batch: entry.batch, outcome }]);
      setRetainedReviewContext((current) => [...current, ...structuredClone(entry.batch.operations)]);
      setAppliedOperations((current) => [...current, {
        receipt_id: `applied-${entry.key}`,
        sequence: current.length + 1,
        operation_id: entry.batch.batch_id,
        change_id: entry.batch.batch_id,
        target_object_type: "Model",
        target_ref: entry.basisModel.project.id,
        field_path: "batch",
        before: entry.basisHash.value,
        after: outcome.applied_model_backend_hash ?? "not_reported",
        application_route: outcome.application_route === "local_wasm_engine" ? "local_wasm_engine" : "tauri_backend_apply",
        applied_model_hash: outcome.applied_model_backend_hash ?? "not_reported",
        acceptance: outcome.acceptance!,
        diagnostics: outcome.diagnostics,
        professional_boundary: outcome.professional_boundary
      }]);
      commitModel(outcome.applied_model);
      preserveAndPruneSelection(outcome.applied_model);
      setQueuedBatches((current) => current.filter((candidate) => candidate.key !== entry.key));
      clearComputedModelState(sessionHistoryChangedSolveJob("batch", entry.batch.batch_id));
      setProposal(null);
      setSelectedReviewTarget(null);
      setBatchMessage("Batch applied once. Previous solve results were cleared; one undo checkpoint is available.");
    } catch (error) {
      if (stillCurrent()) setBatchMessage(`Batch request failed: ${String(error)}`);
    } finally {
      if (operationRequest.current.sequence === request) {
        operationRequest.current.busy = false;
        setOperationBusy(false);
      }
    }
  }

  async function handleAddDraftReview(
    submission: DraftSubmission,
    generation: number
  ): Promise<FrozenDraftReview | null> {
    if (!model || operationRequest.current.busy) return null;
    const revision = modelRevision.current;
    const epoch = requestEpochRef.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    const basisModel = clonePreviewModel(model);
    const frozenSubmission = structuredClone(submission);
    const stillCurrent = () =>
      operationRequest.current.sequence === request &&
      modelRevision.current === revision &&
      requestEpochRef.current === epoch;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const basisHash = await computeModelHash(basisModel);
      if (!stillCurrent() || !basisHash) return null;
      const outcome = frozenSubmission.kind === "single"
        ? await validateModelOperation(basisModel, frozenSubmission.intent, basisHash)
        : await validateOperationBatch(basisModel, frozenSubmission.batch, basisHash);
      if (!stillCurrent() || !currentModel.current) return null;
      const currentHash = await computeModelHash(currentModel.current);
      if (!stillCurrent() || currentHash?.value !== basisHash.value) return null;
      if (!validationMatchesSubmission(frozenSubmission, outcome)) {
        setOperationMessage("Draft validation did not produce a clean, matching diff. Update the draft and Add again.");
        return null;
      }
      const review: FrozenDraftReview = {
        reviewId: `viewport-draft-review-${++directDraftReviewSequence.current}`,
        generation,
        basisRevision: revision,
        basisEpoch: epoch,
        basisHash,
        submission: frozenSubmission,
        outcome
      };
      directDraftReviews.current.clear();
      directDraftReviews.current.set(review.reviewId, review);
      return structuredClone(review);
    } catch (error) {
      if (stillCurrent()) setOperationMessage(`Draft validation failed to run: ${String(error)}`);
      return null;
    } finally {
      if (operationRequest.current.sequence === request) {
        operationRequest.current.busy = false;
        setOperationBusy(false);
      }
    }
  }

  async function handleApplyDraftReview(review: FrozenDraftReview): Promise<boolean> {
    if (!model || operationRequest.current.busy) return false;
    const registered = directDraftReviews.current.get(review.reviewId);
    if (
      !registered ||
      registered.basisRevision !== modelRevision.current ||
      registered.basisEpoch !== requestEpochRef.current ||
      registered.basisHash.value !== review.basisHash.value ||
      !sameSubmission(registered.submission, review.submission) ||
      !validationMatchesSubmission(registered.submission, registered.outcome)
    ) return false;

    const revision = modelRevision.current;
    const epoch = requestEpochRef.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    directDraftReviews.current.delete(review.reviewId);
    const basisModel = clonePreviewModel(model);
    const frozenSubmission = structuredClone(registered.submission);
    const stillCurrent = () =>
      operationRequest.current.sequence === request &&
      modelRevision.current === revision &&
      requestEpochRef.current === epoch;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const currentHash = await computeModelHash(basisModel);
      if (!stillCurrent() || currentHash?.value !== registered.basisHash.value) return false;
      if (frozenSubmission.kind === "batch") {
        const outcome = await applyOperationBatch(basisModel, frozenSubmission.batch, registered.basisHash);
        if (!stillCurrent() || !currentModel.current) return false;
        const afterWaitHash = await computeModelHash(currentModel.current);
        if (!stillCurrent() || afterWaitHash?.value !== registered.basisHash.value) return false;
        if (!applyResultMatchesSubmission(frozenSubmission, outcome, registered.basisHash)) return false;
        const [appliedHash, batchHash] = await Promise.all([
          computeModelHash(outcome.applied_model!),
          canonicalSha256Hex(frozenSubmission.batch)
        ]);
        if (
          !stillCurrent() ||
          appliedHash?.value !== outcome.applied_model_backend_hash ||
          `sha256:${batchHash}` !== outcome.batch_hash
        ) return false;
        setUndoStack((current) => [{
          checkpoint_id: `undo-${review.reviewId}`,
          operation_id: frozenSubmission.batch.batch_id,
          model: basisModel,
          selection: selection ?? defaultSelection(basisModel)
        }, ...current].slice(0, 25));
        setRedoStack([]);
        setBatchReceipts((current) => [...current, { batch: frozenSubmission.batch, outcome }]);
        setRetainedReviewContext((current) => [...current, ...structuredClone(frozenSubmission.batch.operations)]);
        setAppliedOperations((current) => [...current, {
          receipt_id: `applied-${review.reviewId}`,
          sequence: current.length + 1,
          operation_id: frozenSubmission.batch.batch_id,
          change_id: frozenSubmission.batch.batch_id,
          target_object_type: "Model",
          target_ref: basisModel.project.id,
          field_path: "batch",
          before: registered.basisHash.value,
          after: outcome.applied_model_backend_hash ?? "not_reported",
          application_route: outcome.application_route === "local_wasm_engine" ? "local_wasm_engine" : "tauri_backend_apply",
          applied_model_hash: outcome.applied_model_backend_hash ?? "not_reported",
          acceptance: outcome.acceptance!,
          diagnostics: outcome.diagnostics,
          professional_boundary: outcome.professional_boundary
        }]);
        commitModel(outcome.applied_model!, review.reviewId);
        const endpoint = frozenSubmission.batch.operations.at(-1)?.change.after;
        const endpointId = endpoint ? (JSON.parse(endpoint) as { to?: string }).to : null;
        setSelection(endpointId ? { type: "node", id: endpointId } : defaultSelection(outcome.applied_model!));
        clearComputedModelState(sessionHistoryChangedSolveJob("batch", frozenSubmission.batch.batch_id));
        setOperationMessage("Applied the reviewed route as one atomic batch; one undo checkpoint is available.");
        return true;
      }

      const intent = frozenSubmission.intent;
      const outcome = await applyModelOperation(basisModel, intent, registered.basisHash);
      if (!stillCurrent() || !currentModel.current) return false;
      const afterWaitHash = await computeModelHash(currentModel.current);
      if (
        !stillCurrent() ||
        afterWaitHash?.value !== registered.basisHash.value ||
        !applyResultMatchesSubmission(frozenSubmission, outcome, registered.basisHash)
      ) return false;
      const appliedHash = await computeModelHash(outcome.applied_model!);
      if (!stillCurrent() || appliedHash?.value !== outcome.applied_model_backend_hash) return false;
      const receipt: AppliedOperationReceipt = {
        receipt_id: `applied-${review.reviewId}`,
        sequence: appliedOperations.length + 1,
        operation_id: outcome.operation_id,
        change_id: outcome.change_id,
        target_object_type: outcome.target_object_type,
        target_ref: outcome.target_ref,
        field_path: intent.change.field_path,
        before: intent.change.before,
        after: intent.change.after,
        application_route: outcome.application_route,
        applied_model_hash: outcome.applied_model_backend_hash ?? "not_reported",
        acceptance: outcome.acceptance,
        diagnostics: outcome.diagnostics,
        professional_boundary: outcome.professional_boundary
      };
      setAppliedOperations((current) => [receipt, ...current]);
      setRetainedReviewContext((current) => [...current, structuredClone(intent)]);
      setUndoStack((current) => [{
        checkpoint_id: `undo-${review.reviewId}`,
        operation_id: outcome.operation_id,
        model: basisModel,
        selection: selection ?? defaultSelection(basisModel)
      }, ...current].slice(0, 25));
      setRedoStack([]);
      commitModel(outcome.applied_model!, review.reviewId);
      setSelection(selectionForOperationOutcome(outcome) ?? defaultSelection(outcome.applied_model!));
      clearComputedModelState(modelChangedSolveJob(outcome));
      setOperationMessage(`Applied reviewed ${outcome.operation_id}; previous solve results were cleared.`);
      return true;
    } catch (error) {
      if (stillCurrent()) setOperationMessage(`Draft apply failed to run: ${String(error)}`);
      return false;
    } finally {
      if (operationRequest.current.sequence === request) {
        operationRequest.current.busy = false;
        setOperationBusy(false);
      }
    }
  }

  async function handleValidateIntent(intent: EditorOperationIntent) {
    if (!model || operationRequest.current.busy) return;
    const revision = modelRevision.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    const stillCurrent = () => operationRequest.current.sequence === request && modelRevision.current === revision;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const outcome = await validateModelOperation(model, intent, modelHash);
      if (!stillCurrent()) return;
      setOperationOutcomes((current) => ({ ...current, [intentKey(intent)]: outcome }));
    } catch (error) {
      if (stillCurrent()) setOperationMessage(`Operation validation failed to run: ${String(error)}`);
    } finally {
      if (operationRequest.current.sequence === request) {
        operationRequest.current.busy = false;
        setOperationBusy(false);
      }
    }
  }

  async function handleApplyIntent(intent: EditorOperationIntent, ownedOutcome?: { current?: Readonly<OperationOutcome> }): Promise<boolean> {
    if (!model || operationRequest.current.busy) return false;
    const revision = modelRevision.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    const stillCurrent = () => operationRequest.current.sequence === request && modelRevision.current === revision;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const initialHash = await computeModelHash(model);
      if (!stillCurrent() || !initialHash) return false;
      const outcome = await applyModelOperation(model, intent, initialHash);
      if (!stillCurrent() || !currentModel.current) return false;
      const currentHash = await computeModelHash(currentModel.current);
      // The hash computation is asynchronous too; generation must still match
      // after it resolves before any outcome, receipt or checkpoint is published.
      if (!stillCurrent() || currentHash?.value !== initialHash.value) return false;
      setOperationOutcomes((current) => ({ ...current, [intentKey(intent)]: outcome }));
      // Internal owned result only; this is not acknowledgement of a React publication.
      if (ownedOutcome) ownedOutcome.current = outcome;
      if (outcome.validation.application_status !== "applied_to_session_model" || !outcome.applied_model) {
        setOperationMessage(
          `Operation ${outcome.operation_id} was not applied (${outcome.validation.application_status}); see its diagnostics.`
        );
        return false;
      }
      const receipt: AppliedOperationReceipt = {
        receipt_id: `applied-${appliedOperations.length + 1}-${intentKey(intent)}`,
        sequence: appliedOperations.length + 1,
        operation_id: outcome.operation_id,
        change_id: outcome.change_id,
        target_object_type: outcome.target_object_type,
        target_ref: outcome.target_ref,
        field_path: intent.change.field_path,
        before: intent.change.before,
        after: intent.change.after,
        application_route: outcome.application_route,
        applied_model_hash: outcome.applied_model_backend_hash ?? "not_reported",
        acceptance: outcome.acceptance,
        diagnostics: outcome.diagnostics,
        professional_boundary: outcome.professional_boundary
      };
      setAppliedOperations((current) => [receipt, ...current]);
      setRetainedReviewContext((current) => [...current, structuredClone(intent)]);
      setUndoStack((current) =>
        [
          {
            checkpoint_id: `undo-${appliedOperations.length + 1}-${intentKey(intent)}`,
            operation_id: outcome.operation_id,
            model: clonePreviewModel(model),
            selection: selection ?? defaultSelection(model)
          },
          ...current
        ].slice(0, 25)
      );
      setRedoStack([]);
      commitModel(outcome.applied_model);
      const appliedSelection = selectionForOperationOutcome(outcome);
      // Explicit creation deliberately focuses the authored entity. Ordinary
      // edits and deletion preserve ordered membership and prune only refs the
      // accepted model no longer contains.
      if (appliedSelection && /^(create_|insert_|connect_)/.test(outcome.change_kind)) {
        setSelection(appliedSelection);
      } else {
        preserveAndPruneSelection(outcome.applied_model);
      }
      setEditorIntents((current) => current.filter((queued) => intentKey(queued) !== intentKey(intent)));
      // Earlier solve output no longer describes the edited model document;
      // keeping it visible would overstate what was computed.
      setResult(null);
      setAnalysisRun(null);
      setInputManifest(null);
      setRuleCheckAggregate(null);
      setProposal(null);
      setSelectedReviewTarget(null);
      setSolveJob(modelChangedSolveJob(outcome));
      setOperationMessage(
        `Applied ${outcome.operation_id} to the session model; previous solve results were cleared. Run a new solve, then save the project to store the edited model locally.`
      );
      return true;
    } catch (error) {
      if (stillCurrent()) setOperationMessage(`Operation apply failed to run: ${String(error)}`);
      return false;
    } finally {
      if (operationRequest.current.sequence === request) {
        operationRequest.current.busy = false;
        setOperationBusy(false);
      }
    }
  }

  function handleApplyNextQueuedIntent() {
    if (operationBusy) return;
    const [nextIntent] = editorIntents;
    if (!nextIntent) return;
    setActiveSection("operations");
    setOperationTab("review");
    void handleApplyIntent(nextIntent);
  }

  function handleUndoSessionModelEdit() {
    if (operationRequest.current.busy || !model || !selection || undoStack.length === 0) return;
    const [checkpoint, ...remainingUndo] = undoStack;
    setUndoStack(remainingUndo);
    setRedoStack((current) =>
      [
        {
          checkpoint_id: `redo-${checkpoint.checkpoint_id}`,
          operation_id: checkpoint.operation_id,
          model: clonePreviewModel(model),
          selection
        },
        ...current
      ].slice(0, 25)
    );
    commitModel(clonePreviewModel(checkpoint.model));
    setSelection(checkpoint.selection);
    clearComputedModelState(sessionHistoryChangedSolveJob("undo", checkpoint.operation_id));
    setOperationMessage(
      `Undid ${checkpoint.operation_id} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`
    );
  }

  function handleRedoSessionModelEdit() {
    if (operationRequest.current.busy || !model || !selection || redoStack.length === 0) return;
    const [checkpoint, ...remainingRedo] = redoStack;
    setRedoStack(remainingRedo);
    setUndoStack((current) =>
      [
        {
          checkpoint_id: `undo-${checkpoint.checkpoint_id}`,
          operation_id: checkpoint.operation_id,
          model: clonePreviewModel(model),
          selection
        },
        ...current
      ].slice(0, 25)
    );
    commitModel(clonePreviewModel(checkpoint.model));
    setSelection(checkpoint.selection);
    clearComputedModelState(sessionHistoryChangedSolveJob("redo", checkpoint.operation_id));
    setOperationMessage(
      `Redid ${checkpoint.operation_id} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`
    );
  }

  function clearComputedModelState(nextSolveJob: SolveJobAuditState) {
    invalidateReportPackageComputedState();
    ruleRevisionGate.current.invalidate();
    setHistoricalRun(null);
    setResult(null);
    setAnalysisRun(null);
    setInputManifest(null);
    setRuleCheckAggregate(null);
    setProposal(null);
    setSelectedReviewTarget(null);
    setSolveJob(nextSolveJob);
  }

  function invalidateReportPackageComputedState() {
    reportPackageRequestGenerationRef.current += 1;
    reportPackageBusyGenerationRef.current = null;
    setReportPackageBusy(false);
    setReportPackageRedaction(null);
    setReportPackageRoute(null);
  }

  function adoptNormalizedPersistenceModel(
    envelope: LocalProjectEnvelope,
    returnedHistory: HistoricalRunContext | null,
    returnedModelHash: ModelHashEvidence | null
  ) {
    commitModel(envelope.model);
    const normalizedIndex = modelIndexFor(
      envelope.model,
      projectSessionGenerationRef.current,
      uiModelRevisionRef.current
    );
    commitSelectionState(
      pruneSelection(
        orderedSelectionRef.current,
        new Set(normalizedIndex.entities.keys()),
        { type: "project", id: envelope.model.project.id }
      ),
      envelope.model
    );
    setUndoStack([]);
    setRedoStack([]);
    setAppliedOperations([]);
    setQueuedBatches([]);
    setBatchOutcomes({});
    setBatchReceipts([]);
    setBatchMessage(null);
    setOperationOutcomes({});
    setOperationMessage(null);
    setResult(null);
    setAnalysisRun(null);
    setInputManifest(null);
    setRuleCheckAggregate(null);
    setHistoricalRun(returnedHistory);
    setRetainedReviewContext(envelope.editor_intents ?? []);
    setEditorIntents([]);
    setSolveJob(initialSolveJob());
    setModelHash(envelope.model_hash ?? returnedModelHash);
  }

  const latestIntegrityObservation = useRef(0);
  function recordPersistenceObservation(envelope: LocalProjectEnvelope, modelHash: ModelHashEvidence | null,
    envelopeHash: ProjectEnvelopeHashEvidence | null, source: "save" | "create", generation: number,
    observation: number, responseValid: boolean, payloadRef: string, retainedHistoricalCarrier = false) {
    if (!canPublishPersistenceObservation(generation, projectSessionGenerationRef.current, observation, latestIntegrityObservation.current)) return;
    latestIntegrityObservation.current = observation;
    const observedAt = new Date().toISOString();
    setModelHashIntegrity({ ...deriveModelHashIntegrity(envelope.model_hash ?? null, modelHash, payloadRef, source, observedAt, responseValid),
      claim_standing: retainedHistoricalCarrier ? "retained_historical_carrier" : "canonical_payload" });
    setProjectEnvelopeHashIntegrity(deriveProjectEnvelopeHashIntegrity(envelope.project_envelope_hash ?? null, envelopeHash, payloadRef, source, observedAt, responseValid));
  }

  function acquireProjectOperation(): symbol | null {
    if (projectOperationOwner.current !== null) return null;
    const owner = Symbol("project operation");
    projectOperationOwner.current = owner;
    setProjectBusy(true);
    return owner;
  }

  function releaseProjectOperation(owner: symbol) {
    if (projectOperationOwner.current !== owner) return;
    projectOperationOwner.current = null;
    setProjectBusy(false);
  }

  async function handleCreateProject() {
    if (!model) return;
    const owner = acquireProjectOperation();
    if (!owner) return;
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    const requestModel = model;
    const generation = projectSessionGenerationRef.current;
    const requestRevision = uiModelRevisionRef.current;
    const sameProjectSession = () => generation === projectSessionGenerationRef.current && currentModel.current?.project.id === requestModel.project.id;
    const retainedRun = retainedReferencePersistence(requestModel);
    const requestHistoricalRun = historicalRun?.designation === "historical_saved_run" ? historicalRun : retainedRun?.history ?? null;
    const requestMigrationLedgerCount = modelMigrationLedger.length;
    let pendingObservation: { envelope: LocalProjectEnvelope; ordinal: number } | null = null;
    try {
      const combinedContext = structuredClone([...retainedReviewContext, ...editorIntents, ...queuedBatches.flatMap((entry) => entry.batch.operations)]);
      const actualRequestModelHash = await computeModelHash(requestModel);
      const snapshotModelHash = requestHistoricalRun ? requestHistoricalRun.modelHash : actualRequestModelHash;
      const snapshotResult = requestHistoricalRun ? requestHistoricalRun.rawMechanicsResult as MechanicsResult | null : retainedRun?.result ?? result;
      const snapshotAnalysisRun = requestHistoricalRun ? requestHistoricalRun.rawAnalysisRun as AnalysisRunEnvelope | null : retainedRun?.analysisRun ?? analysisRun;
      if (!stillCurrent()) return;
      const computedEnvelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: combinedContext,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: snapshotResult,
        analysis_run: snapshotAnalysisRun,
        model_hash: snapshotModelHash
      });
      const envelopeHash = requestHistoricalRun && computedEnvelopeHash?.value === requestHistoricalRun.envelopePayloadHash
        ? requestHistoricalRun.envelopeHash : computedEnvelopeHash;
      if (!stillCurrent()) return;
      const created = await createLocalProject(
        model,
        combinedContext,
        proposal,
        selectedReviewTarget,
        snapshotResult,
        snapshotAnalysisRun,
        snapshotModelHash,
        envelopeHash
      );
      const writeObservation = ++persistedWriteObservation.current;
      if (!sameProjectSession()) return;
      pendingObservation = { envelope: created, ordinal: writeObservation };
      // Retire the old observation while verification of rewritten bytes is pending.
      setModelHashIntegrity(null);
      setProjectEnvelopeHashIntegrity(null);
      const returnedModelHash = await computeModelHash(created.model);
      if (!sameProjectSession()) return;
      const recomputedReturnedEnvelopeHash = await computeProjectEnvelopeHash({
        model: created.model,
        editor_intents: created.editor_intents,
        proposal: created.proposal,
        selected_review_target: created.selected_review_target,
        mechanics_result: created.mechanics_result,
        analysis_run: created.analysis_run,
        model_hash: created.model_hash
      });
      if (!sameProjectSession()) return;
      const responseModelChanged = returnedModelHash?.value !== actualRequestModelHash?.value;
      const modelChanged = isSupportedChangedModelPersistenceResponse(
        created,
        actualRequestModelHash,
        returnedModelHash,
        recomputedReturnedEnvelopeHash,
        requestMigrationLedgerCount
      );
      const verifiedResponse = Boolean(actualRequestModelHash && returnedModelHash &&
        created.model.project.id === requestModel.project.id &&
        created.summary.project_id === requestModel.project.id &&
        persistenceHashesMatch(created, returnedModelHash, recomputedReturnedEnvelopeHash, !responseModelChanged ? requestHistoricalRun?.modelHash ?? null : null) &&
        (!responseModelChanged || modelChanged));
      recordPersistenceObservation(created, returnedModelHash, recomputedReturnedEnvelopeHash, "create", generation, writeObservation, verifiedResponse, requestModel.project.id, Boolean(verifiedResponse && requestHistoricalRun && !responseModelChanged));
      pendingObservation = null;
      if (created.model.project.id !== requestModel.project.id || created.summary.project_id !== requestModel.project.id) {
        if (!stillCurrent()) return;
        setProjectMessage("Create failed: PROJECT-PERSISTENCE-RESPONSE-INTEGRITY: returned project identity does not match the requested project.");
        setProjectOperation("create_failed");
        return;
      }
      if (responseModelChanged && !modelChanged) {
        if (!stillCurrent()) return;
        setProjectMessage("Create failed: PROJECT-PERSISTENCE-RESPONSE-INTEGRITY: returned model change is not bound to the supported persisted normalization transition.");
        setProjectOperation("create_failed");
        return;
      }
      // Failed/missing later requests do not retire an actual same-session
      // write. Only a newer verified fulfillment may supersede this observation;
      // current-only metadata/model/history adoption keeps its stricter gate.
      const verifiedBasis = verifiedWriteBasis(verifiedResponse, generation, projectSessionGenerationRef.current,
        writeObservation, latestVerifiedWriteObservation.current, responseModelChanged ? -1 : requestRevision, returnedModelHash, "create");
      if (verifiedBasis) {
        latestVerifiedWriteObservation.current = writeObservation;
        savedBasisSequence.current += 1;
        setSavedModelBasis(verifiedBasis);
      }
      if (!stillCurrent()) return;
      const returnedHistory = modelChanged || requestHistoricalRun
        ? await buildHistoricalRunContext(created)
        : null;
      if (!stillCurrent()) return;
      if (modelChanged) {
        adoptNormalizedPersistenceModel(created, returnedHistory, returnedModelHash);
        epoch = requestEpochRef.current;
        if (verifiedResponse && writeObservation === latestVerifiedWriteObservation.current) setSavedModelBasis({ generation, revision: uiModelRevisionRef.current, hash: returnedModelHash!.value, source: "create" });
      } else if (requestHistoricalRun) {
        setHistoricalRun((current) => current === requestHistoricalRun ? returnedHistory : current);
      }
      setProjectSummary(created.summary);
      setProposal(created.proposal ?? null);
      setSelectedReviewTarget(created.selected_review_target ?? null);
      setProjectEnvelopeHash(created.project_envelope_hash ?? null);
      setModelDocumentMigration(created.model_document_migration ?? null);
      setModelMigrationLedger(created.model_migration_ledger ?? []);
      setProjectMessage(created.summary.message);
      setProjectOperation("create");
    } catch (error) {
      if (pendingObservation && sameProjectSession() &&
        pendingObservation.envelope?.model?.project?.id === requestModel.project.id &&
        pendingObservation.envelope?.summary?.project_id === requestModel.project.id) {
        recordPersistenceObservation(pendingObservation.envelope, null, null, "create", generation,
          pendingObservation.ordinal, false, requestModel.project.id);
      }
      if (!stillCurrent()) return;
      setProjectMessage(`Create failed: ${String(error)}`);
      setProjectOperation("create_failed");
    } finally {
      releaseProjectOperation(owner);
    }
  }

  async function handleCreateBlankProject() {
    const owner = acquireProjectOperation();
    if (!owner) return;
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    try {
      const blankModel = buildBlankLocalModelDocument();
      const blankModelHash = await computeModelHash(blankModel);
      const envelopeHash = await computeProjectEnvelopeHash({
        model: blankModel,
        editor_intents: [],
        proposal: null,
        selected_review_target: null,
        mechanics_result: null,
        analysis_run: null,
        model_hash: blankModelHash
      });
      const created = await createLocalProject(blankModel, [], null, null, null, null, blankModelHash, envelopeHash);
      if (!stillCurrent()) return;
      const returnedHash = await computeModelHash(created.model);
      const returnedEnvelopeHash = await computeProjectEnvelopeHash({
        model: created.model, editor_intents: created.editor_intents,
        proposal: created.proposal, selected_review_target: created.selected_review_target,
        mechanics_result: created.mechanics_result, analysis_run: created.analysis_run,
        model_hash: created.model_hash
      });
      if (!stillCurrent()) return;
      const verifiedBlankResponse = Boolean(blankModelHash && returnedHash &&
        created.model.project.id === blankModel.project.id &&
        created.summary.project_id === blankModel.project.id &&
        persistenceHashesMatch(created, returnedHash, returnedEnvelopeHash) &&
        (returnedHash.value === blankModelHash.value || isSupportedChangedModelPersistenceResponse(created, blankModelHash, returnedHash, returnedEnvelopeHash, 0)));
      const createdSummary = {
        ...created.summary,
        message: "Created blank local model document without fixture entities or external file copies."
      };
      // The create commits here. A create that fails leaves the open project's
      // rule revision gate and integrity cells as they were.
      ruleRevisionGate.current.invalidate();
      setModelHashIntegrity(null);
      setProjectEnvelopeHashIntegrity(null);
      advanceProjectSession();
      recordPersistenceObservation(created, returnedHash, returnedEnvelopeHash, "create", projectSessionGenerationRef.current, ++persistedWriteObservation.current, verifiedBlankResponse, blankModel.project.id);
      // A successful New Blank starts the first-open shell context. Failed or
      // superseded creates never reach this commit boundary or change chrome.
      routingInspectorRestore.current = null;
      pageReturnFocusRef.current = null;
      setArmedCreationTool(null);
      setToolkitFocus(null);
      setOpenMenu(null);
      setActiveSection(null);
      setStageSurface(null);
      setStageViewMemory(EMPTY_STAGE_VIEW_MEMORY);
      setTreeCollapsed(window.innerWidth < 1280);
      setInspectorCollapsed(true);
      setReviewDetailsOpen(false);
      setAuditDrawerOpen(false);
      setIssuesDrawerOpen(false);
      setOperationTab("review");
      commitModel(created.model);
      if (verifiedBlankResponse) setSavedModelBasis({ generation: projectSessionGenerationRef.current, revision: uiModelRevisionRef.current, hash: returnedHash!.value, source: "create" });
      setSelection(defaultSelection(created.model));
      epoch = requestEpochRef.current;
      setUndoStack([]);
      setRedoStack([]);
      setAppliedOperations([]);
      setQueuedBatches([]);
      setBatchOutcomes({});
      setBatchReceipts([]);
      setBatchMessage(null);
      setOperationOutcomes({});
      setOperationMessage(null);
      setResult(null);
      setAnalysisRun(null);
      setInputManifest(null);
      setRuleCheckAggregate(null);
      setProposal(null);
      setRetainedReviewContext(created.editor_intents ?? []);
      setEditorIntents([]);
      setSelectedReviewTarget(null);
      setSolveJob(blankProjectCreatedSolveJob(created.model));
      setProjectSummary(createdSummary);
      setProjectEnvelopeHash(created.project_envelope_hash ?? null);
      setModelHash(created.model_hash ?? blankModelHash);
      setModelDocumentMigration(created.model_document_migration ?? null);
      setModelMigrationLedger(created.model_migration_ledger ?? []);
      setProjectMessage(createdSummary.message);
      setProjectOperation("create_blank");
    } catch (error) {
      if (!stillCurrent()) return;
      setProjectMessage(`Blank create failed: ${String(error)}`);
      setProjectOperation("create_blank_failed");
    } finally {
      releaseProjectOperation(owner);
    }
  }

  async function handleOpenProject(projectId: string | null = null) {
    const owner = acquireProjectOperation();
    if (!owner) return;
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    try {
      const opened = await openLocalProject(projectId);
      if (!stillCurrent()) return;
      if (!opened) {
        setProjectMessage(
          projectId ? `No local project snapshot found for ${projectId}.` : "No local project snapshot found."
        );
        setProjectOperation("open_missing");
        return;
      }
      const restoredHistory = await buildHistoricalRunContext(opened);
      if (!stillCurrent()) return;
      // The open commits here. An open that finds nothing, fails or is
      // superseded leaves the open project's rule revision gate and integrity
      // cells as they were.
      ruleRevisionGate.current.invalidate();
      setModelHashIntegrity(null);
      setProjectEnvelopeHashIntegrity(null);
      advanceProjectSession();
      commitModel(opened.model);
      const openedGeneration = projectSessionGenerationRef.current;
      const openedRevision = uiModelRevisionRef.current;
      const openedBasisSequence = ++savedBasisSequence.current;
      const observationOwner = { request, generation: openedGeneration, projectId: opened.model.project.id, basisSequence: openedBasisSequence };
      const stillOwnsOpenedObservation = () => ownsOpenPersistenceObservation(observationOwner, {
        request: projectRequest.current, generation: projectSessionGenerationRef.current,
        projectId: currentModel.current?.project.id, basisSequence: savedBasisSequence.current
      });
      setSelection(defaultSelection(opened.model));
      epoch = requestEpochRef.current;
      setUndoStack([]);
      setRedoStack([]);
      setAppliedOperations([]);
      setQueuedBatches([]);
      setBatchOutcomes({});
      setBatchReceipts([]);
      setBatchMessage(null);
      setOperationOutcomes({});
      setOperationMessage(null);
      setHistoricalRun(restoredHistory);
      setResult(null);
      setAnalysisRun(null);
      // Persisted analysis-run refs do not include the exact current-session
      // manifest payload needed to recompute its hash. A new solve is required
      // before report-package save can become ready.
      setInputManifest(null);
      setRuleCheckAggregate(null);
      setProposal(opened.proposal ?? null);
      setRetainedReviewContext(opened.editor_intents ?? []);
      setEditorIntents([]);
      setSelectedReviewTarget(opened.selected_review_target ?? null);
      setSolveJob(initialSolveJob());
      setProjectSummary(opened.summary);
      setProjectEnvelopeHash(opened.project_envelope_hash ?? null);
      setModelDocumentMigration(opened.model_document_migration ?? null);
      setModelMigrationLedger(opened.model_migration_ledger ?? []);
      setProjectMessage(opened.summary.message);
      setProjectOperation(projectId ? "open_by_id" : "open");
      setActiveSection(restoredHistory ? "results" : null);
      const recomputedHash = await computeModelHash(opened.model);
      if (recomputedHash && openedGeneration === projectSessionGenerationRef.current && openedBasisSequence === savedBasisSequence.current) {
        setSavedModelBasis({ generation: openedGeneration, revision: openedRevision, hash: recomputedHash.value, source: "open" });
      }
      if (!stillOwnsOpenedObservation()) return;
      setModelHashIntegrity(deriveModelHashIntegrity(opened.model_hash ?? null, recomputedHash, opened.model.project.id));
      const recomputedEnvelopeHash = await computeProjectEnvelopeHash({
        model: opened.model,
        editor_intents: opened.editor_intents ?? [],
        proposal: opened.proposal ?? null,
        selected_review_target: opened.selected_review_target ?? null,
        mechanics_result: opened.mechanics_result ?? null,
        analysis_run: opened.analysis_run ?? null,
        model_hash: opened.model_hash ?? null
      });
      if (!stillOwnsOpenedObservation()) return;
      setProjectEnvelopeHashIntegrity(
        deriveProjectEnvelopeHashIntegrity(
          opened.project_envelope_hash ?? null,
          recomputedEnvelopeHash,
          opened.model.project.id
        )
      );
    } catch (error) {
      if (!stillCurrent()) return;
      setProjectMessage(`Open failed: ${String(error)}`);
      setProjectOperation("open_failed");
    } finally {
      releaseProjectOperation(owner);
    }
  }

  async function handleSaveProject() {
    if (!model) return;
    const owner = acquireProjectOperation();
    if (!owner) return;
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    const requestModel = model;
    const generation = projectSessionGenerationRef.current;
    const requestRevision = uiModelRevisionRef.current;
    const sameProjectSession = () => generation === projectSessionGenerationRef.current && currentModel.current?.project.id === requestModel.project.id;
    const retainedRun = retainedReferencePersistence(requestModel);
    const requestHistoricalRun = historicalRun?.designation === "historical_saved_run" ? historicalRun : retainedRun?.history ?? null;
    const requestMigrationLedgerCount = modelMigrationLedger.length;
    let pendingObservation: { envelope: LocalProjectEnvelope; ordinal: number } | null = null;
    try {
      const combinedContext = structuredClone([...retainedReviewContext, ...editorIntents, ...queuedBatches.flatMap((entry) => entry.batch.operations)]);
      const actualRequestModelHash = await computeModelHash(requestModel);
      const snapshotModelHash = requestHistoricalRun ? requestHistoricalRun.modelHash : actualRequestModelHash;
      const snapshotResult = requestHistoricalRun ? requestHistoricalRun.rawMechanicsResult as MechanicsResult | null : retainedRun?.result ?? result;
      const snapshotAnalysisRun = requestHistoricalRun ? requestHistoricalRun.rawAnalysisRun as AnalysisRunEnvelope | null : retainedRun?.analysisRun ?? analysisRun;
      if (!stillCurrent()) return;
      const computedEnvelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: combinedContext,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: snapshotResult,
        analysis_run: snapshotAnalysisRun,
        model_hash: snapshotModelHash
      });
      const envelopeHash = requestHistoricalRun && computedEnvelopeHash?.value === requestHistoricalRun.envelopePayloadHash
        ? requestHistoricalRun.envelopeHash : computedEnvelopeHash;
      if (!stillCurrent()) return;
      const saved = await saveLocalProject(
        model,
        combinedContext,
        proposal,
        selectedReviewTarget,
        snapshotResult,
        snapshotAnalysisRun,
        snapshotModelHash,
        envelopeHash,
        modelDocumentMigration
      );
      const writeObservation = ++persistedWriteObservation.current;
      if (!sameProjectSession()) return;
      pendingObservation = { envelope: saved, ordinal: writeObservation };
      // Retire the old observation while verification of rewritten bytes is pending.
      setModelHashIntegrity(null);
      setProjectEnvelopeHashIntegrity(null);
      const returnedModelHash = await computeModelHash(saved.model);
      if (!sameProjectSession()) return;
      const recomputedReturnedEnvelopeHash = await computeProjectEnvelopeHash({
        model: saved.model,
        editor_intents: saved.editor_intents,
        proposal: saved.proposal,
        selected_review_target: saved.selected_review_target,
        mechanics_result: saved.mechanics_result,
        analysis_run: saved.analysis_run,
        model_hash: saved.model_hash
      });
      if (!sameProjectSession()) return;
      const responseModelChanged = returnedModelHash?.value !== actualRequestModelHash?.value;
      const modelChanged = isSupportedChangedModelPersistenceResponse(
        saved,
        actualRequestModelHash,
        returnedModelHash,
        recomputedReturnedEnvelopeHash,
        requestMigrationLedgerCount
      );
      const verifiedResponse = Boolean(actualRequestModelHash && returnedModelHash &&
        saved.model.project.id === requestModel.project.id &&
        saved.summary.project_id === requestModel.project.id &&
        persistenceHashesMatch(saved, returnedModelHash, recomputedReturnedEnvelopeHash, !responseModelChanged ? requestHistoricalRun?.modelHash ?? null : null) &&
        (!responseModelChanged || modelChanged));
      recordPersistenceObservation(saved, returnedModelHash, recomputedReturnedEnvelopeHash, "save", generation, writeObservation, verifiedResponse, requestModel.project.id, Boolean(verifiedResponse && requestHistoricalRun && !responseModelChanged));
      pendingObservation = null;
      if (saved.model.project.id !== requestModel.project.id || saved.summary.project_id !== requestModel.project.id) {
        if (!stillCurrent()) return;
        setProjectMessage("Save failed: PROJECT-PERSISTENCE-RESPONSE-INTEGRITY: returned project identity does not match the requested project.");
        setProjectOperation("save_failed");
        return;
      }
      if (responseModelChanged && !modelChanged) {
        if (!stillCurrent()) return;
        setProjectMessage("Save failed: PROJECT-PERSISTENCE-RESPONSE-INTEGRITY: returned model change is not bound to the supported persisted normalization transition.");
        setProjectOperation("save_failed");
        return;
      }
      // Failed/missing later requests do not retire an actual same-session
      // write. Only a newer verified fulfillment may supersede this observation;
      // current-only metadata/model/history adoption keeps its stricter gate.
      const verifiedBasis = verifiedWriteBasis(verifiedResponse, generation, projectSessionGenerationRef.current,
        writeObservation, latestVerifiedWriteObservation.current, responseModelChanged ? -1 : requestRevision, returnedModelHash, "save");
      if (verifiedBasis) {
        latestVerifiedWriteObservation.current = writeObservation;
        savedBasisSequence.current += 1;
        setSavedModelBasis(verifiedBasis);
      }
      if (!stillCurrent()) return;
      const returnedHistory = modelChanged || requestHistoricalRun
        ? await buildHistoricalRunContext(saved)
        : null;
      if (!stillCurrent()) return;
      if (modelChanged) {
        adoptNormalizedPersistenceModel(saved, returnedHistory, returnedModelHash);
        epoch = requestEpochRef.current;
        if (verifiedResponse && writeObservation === latestVerifiedWriteObservation.current) setSavedModelBasis({ generation, revision: uiModelRevisionRef.current, hash: returnedModelHash!.value, source: "save" });
      } else if (requestHistoricalRun) {
        // Preserve a fresh Current solve that completed while the native save
        // was pending. Only the still-owned Historical snapshot may refresh.
        setHistoricalRun((current) => current === requestHistoricalRun ? returnedHistory : current);
      }
      setProjectSummary(saved.summary);
      setProposal(saved.proposal ?? null);
      setSelectedReviewTarget(saved.selected_review_target ?? null);
      setProjectEnvelopeHash(saved.project_envelope_hash ?? null);
      setModelDocumentMigration(saved.model_document_migration ?? null);
      setModelMigrationLedger(saved.model_migration_ledger ?? []);
      setProjectMessage(saved.summary.message);
      setProjectOperation("save");
    } catch (error) {
      if (pendingObservation && sameProjectSession() &&
        pendingObservation.envelope?.model?.project?.id === requestModel.project.id &&
        pendingObservation.envelope?.summary?.project_id === requestModel.project.id) {
        recordPersistenceObservation(pendingObservation.envelope, null, null, "save", generation,
          pendingObservation.ordinal, false, requestModel.project.id);
      }
      if (!stillCurrent()) return;
      setProjectMessage(`Save failed: ${String(error)}`);
      setProjectOperation("save_failed");
    } finally {
      releaseProjectOperation(owner);
    }
  }

  async function handleListProjects() {
    const owner = acquireProjectOperation();
    if (!owner) return;
    try {
      const listed = await listLocalProjects();
      setProjectIndex(listed);
      setProjectMessage(
        `Listed ${listed.length} local project snapshot${listed.length === 1 ? "" : "s"} from the local store index.`
      );
      setProjectOperation("list");
    } catch (error) {
      setProjectMessage(`List failed: ${String(error)}`);
      setProjectOperation("list_failed");
    } finally {
      releaseProjectOperation(owner);
    }
  }

  function handleSelectResult(resultId: string) {
    setSelectedReviewTarget({ target_type: "result", id: resultId });
    const item = result?.results.find((candidate) => candidate.id === resultId);
    if (!item || !model) return;
    const entitySelection = resolveEntitySelection(model, item.entity_ref);
    if (entitySelection) {
      handleSelectEntity(entitySelection);
    }
  }

  function handleSelectDiagnostic(diagnosticId: string) {
    setSelectedReviewTarget({ target_type: "diagnostic", id: diagnosticId });
    if (!model) return;
    const entitySelection = resolveDiagnosticEntitySelection({ model, result, knowledge, diagnosticId });
    if (entitySelection) {
      handleSelectEntity(entitySelection);
    }
  }

  function handleSelectEntity(
    entity: EntityRef,
    modifiers: SelectionModifiers & Readonly<{ range?: boolean }> = {},
    displayedOrder: readonly EntityKey[] = []
  ): OrderedSelectionState {
    const next = modifiers.range
      ? applyDisplayedRange(orderedSelectionRef.current, entity, displayedOrder)
      : applySelection(orderedSelectionRef.current, entity, modifiers);
    commitSelectionState(next);
    return orderedSelectionRef.current;
  }

  function handleBoxSelection(
    keys: readonly EntityKey[],
    modifiers: SelectionModifiers
  ): OrderedSelectionState {
    const next = applyBoxSelection(orderedSelectionRef.current, keys, modifiers);
    commitSelectionState(next);
    return next;
  }

  function invalidateDirectDraftContext() {
    requestEpochRef.current += 1;
    setRequestEpoch(requestEpochRef.current);
    directDraftReviews.current.clear();
  }

  function handleArmCreationTool(tool: CreationTool | null) {
    viewportViewCommandRef.current?.({ type: "cancel-box-selection" });
    if ((tool === "node" || tool === "pipe" || tool === "component") && routingInspectorRestore.current === null) {
      routingInspectorRestore.current = inspectorCollapsed;
    }
    setArmedCreationTool(tool);
    if (!tool) return;
    if (tool === "load") {
      setActiveSection("loads");
      return;
    }
    setActiveSection(null);
    revealModelStageCanvas();
    if (tool === "support") {
      openWorkspaceRail("inspector");
      if (selection) setPropertyTaskRequest({
        sequence: ++propertyTaskRequestSequenceRef.current,
        target: { ...selection },
        view: "properties",
        focusTestId: "create-support-id"
      });
    } else {
      exposeViewportForNarrowInteraction(`authoring-${tool}`);
      // Re-selecting an armed routing tool also reveals its inspector home.
      setInspectorCollapsed(false);
    }
  }

  function handleToolkitCommand(capability: ToolkitCapability) {
    if (!selection) return;
    const context = { selection, selectionCardinality: orderedSelectionRef.current.orderedKeys.length, canUndo: undoStack.length > 0, canRedo: redoStack.length > 0, busy: operationRequest.current.busy || operationBusy, windConfigured: Boolean(selection.type === "load" && model?.load_cases.find((load) => load.id === selection.id)?.equivalent_static?.wind) };
    if (!capabilityAvailability(capability, context).enabled) return;
    if (capability.history) {
      if (capability.history === "undo") handleUndoSessionModelEdit();
      else handleRedoSessionModelEdit();
      setToolkitFocus({ testId: "toolkit-entry" });
      return;
    }
    const route = capabilityRoute(capability, context);
    if (!route) return;
    viewportViewCommandRef.current?.({ type: "cancel-box-selection" });
    if ((route.tool === "node" || route.tool === "pipe" || route.tool === "component") && routingInspectorRestore.current === null) {
      routingInspectorRestore.current = inspectorCollapsed;
    }
    setArmedCreationTool(route.tool ?? null);
    if (route.tool && route.surface === "viewport") {
      exposeViewportForNarrowInteraction(`authoring-${route.tool}`);
      // Commit visibility with the focus request. A later auto-open effect is
      // too late for a browser to focus a control in the hidden inspector.
      if (route.tool === "node" || route.tool === "pipe" || route.tool === "component") setInspectorCollapsed(false);
    }
    if (route.surface === "operations") {
      const tabs: Record<string, string> = { "geometry-tools": "geometry", "boundary-authoring": "supports", "hanger-selection": "supports", "self-weight-plan": "weight", "offline-proposal-intake": "agent" };
      setOperationTab(tabs[route.elementId ?? ""] ?? "review");
    }
    if (route.surface === "inspector") {
      setPropertyTaskRequest({
        sequence: ++propertyTaskRequestSequenceRef.current,
        target: { ...selection },
        view: route.inspectorView ?? "properties",
        focusTestId: route.focusTestId,
        elementId: route.elementId
      });
      revealModelStageCanvas();
      openWorkspaceRail("inspector");
      setActiveSection(null);
    }
    else if (route.surface === "tree") { openWorkspaceRail("tree"); setActiveSection(null); }
    else if (route.surface === "viewport") { revealModelStageCanvas(); setActiveSection(null); }
    else setActiveSection(route.surface);
    if (route.surface !== "inspector") setToolkitFocus({ testId: route.focusTestId, elementId: route.elementId });
  }

  async function handleSaveReportPackage() {
    if (!model || !currentSolvedResult || !analysisRun || !inputManifest || running || reportPackageBusy) return;
    const generation = ++reportPackageRequestGenerationRef.current;
    const basis = {
      projectSessionGeneration: projectSessionGenerationRef.current,
      modelRevision: uiModelRevisionRef.current,
      model,
      result: currentSolvedResult,
      inputManifest,
      analysisRun,
      comparison,
      projectSummary,
      ruleCheckAggregate,
      privateIntent: reportPackagePrivateIntent
    } as const;
    const isCurrent = () =>
      reportPackageRequestGenerationRef.current === generation &&
      reportPackageBusyGenerationRef.current === generation &&
      projectSessionGenerationRef.current === basis.projectSessionGeneration &&
      uiModelRevisionRef.current === basis.modelRevision &&
      currentModel.current === basis.model &&
      currentSolvedResultRef.current === basis.result &&
      currentInputManifestRef.current === basis.inputManifest &&
      analysisBasisRef.current.value === basis.analysisRun;
    reportPackageBusyGenerationRef.current = generation;
    setReportPackageBusy(true);
    setReportPackageRoute(null);
    try {
      const request = await buildReportPackageRequest({
        model: basis.model,
        result: basis.result,
        analysisRun: basis.analysisRun,
        inputManifest: basis.inputManifest,
        projectSummary: basis.projectSummary,
        comparison: basis.comparison,
        ruleCheckAggregate: basis.ruleCheckAggregate
      });
      if (!isCurrent()) return;
      const controlled = controlReportPackageRequest(request, basis.privateIntent);
      setReportPackageRedaction(controlled);
      if (controlled.blocked || controlled.payload === null) {
        setReportPackageRoute({
          route: "redaction_blocked",
          diagnostic: `REPORT-PACKAGE-REDACTION-BLOCKED: ${controlled.findings.length} finding(s) prevent package assembly or persistence.`
        });
        return;
      }
      const saved = await saveReportPackage(controlled);
      if (!isCurrent()) return;
      setReportPackageRoute(saved);
    } catch (error) {
      if (!isCurrent()) return;
      setReportPackageRedaction(null);
      setReportPackageRoute({
        route: "redaction_blocked",
        diagnostic: formatPackageSaveError(error)
      });
    } finally {
      if (isCurrent()) {
        reportPackageBusyGenerationRef.current = null;
        setReportPackageBusy(false);
      }
    }
  }

  // Single command sink for both the in-DOM menu bar (tested) and the native
  // macOS menu bar (Tauri shell only). View commands summon/dismiss workspace
  // sections and toggle the tree/inspector rails; the spatial core (tree |
  // viewport | inspector) is always present, so Insert commands arm the same
  // creation tools exposed in the command bar instead of acting as navigation.
  function runMenuCommand(command: MenuCommandId) {
    setOpenMenu(null);
    switch (command) {
      case "file.new-local":
        void handleCreateProject();
        break;
      case "file.new-blank":
        void handleCreateBlankProject();
        break;
      case "file.open-local":
        void handleOpenProject();
        break;
      case "file.list-local":
        void handleListProjects();
        break;
      case "file.save-local":
        void handleSaveProject();
        break;
      case "file.save-report-package":
        setActiveSection("report");
        void handleSaveReportPackage();
        break;
      case "edit.undo":
        handleUndoSessionModelEdit();
        break;
      case "edit.redo":
        handleRedoSessionModelEdit();
        break;
      case "view.issues":
        setIssuesDrawerOpen((open) => !open);
        break;
      case "view.audit":
        setAuditDrawerOpen((open) => !open);
        break;
      case "view.close-panels":
        closeShellPage();
        break;
      case "view.tree":
        // The table pane collapses only where it is a drawer (Model view; Both view below 1280 px).
        if (shellNowRef.current.view === "model" || (shellNowRef.current.view === "both" && window.innerWidth < 1280)) toggleWorkspaceRail("tree");
        break;
      case "view.inspector":
        // As the toolbar's Inspector toggle: it acts in Both view only.
        if (shellNowRef.current.view === "both") toggleWorkspaceRail("inspector");
        break;
      case "insert.load":
        handleArmCreationTool("load");
        break;
      case "insert.node":
        handleArmCreationTool("node");
        break;
      case "insert.pipe":
        handleArmCreationTool("pipe");
        break;
      case "insert.support":
        handleArmCreationTool("support");
        break;
      case "insert.component":
        handleArmCreationTool("component");
        break;
      case "analyze.run":
        setActiveSection("solve");
        void handleRun();
        break;
      case "analyze.cancel":
        void handleCancelRun();
        break;
      case "analyze.rule-checks":
        setActiveSection("solve");
        break;
      default: {
        const prefix = "view.section.";
        if (command.startsWith(prefix)) {
          const sectionId = command.slice(prefix.length) as WorkspaceSectionId;
          // Choosing the open page again closes it onto the stage underneath;
          // choosing the current stage surface again returns to the model tree, as before.
          setActiveSection((current) => current !== sectionId
            ? sectionId
            : isStageSurface(sectionId) ? null : sectionAfterPageClose(shellNowRef.current.stageSurface));
        } else if (command.startsWith("view.view.")) {
          chooseStageView(command.slice("view.view.".length) as ShellView);
        } else if (command.startsWith("view.stage.")) {
          enterStage(command.slice("view.stage.".length) as ShellStage);
        } else if (command.startsWith("view.theme.")) {
          const theme = command.slice("view.theme.".length) as UiThemePreference;
          setUiPreferences((current) => updateUiPreferences(current, { theme }));
        } else if (command.startsWith("view.density.")) {
          const density = command.slice("view.density.".length) as UiDensityPreference;
          setUiPreferences((current) => updateUiPreferences(current, { density }));
        }
        break;
      }
    }
  }

  const nativeRunPresence = runPresenceFromCells({ result, historicalRun, solveJob, hasQualifiedCurrentResult: currentSolvedResult !== null });
  const nativeResultsEnabled = railStageState("results", nativeRunPresence).enabled;
  const nativeReviewEnabled = railStageState("review", nativeRunPresence).enabled;
  const nativeProjectName = projectSummary?.project_name ?? model?.project.name ?? null;
  useEffect(() => {
    if (!isTauriRuntime()) return;
    void syncNativeShellState({
      projectName: nativeProjectName,
      modelEdited,
      projectBusy,
      stage: shellNow.stage,
      view: stageViewNow,
      theme: uiPreferences.theme,
      density: uiPreferences.density,
      resultsStageEnabled: nativeResultsEnabled,
      reviewStageEnabled: nativeReviewEnabled,
      inspectorOpen: stageViewNow === "model" || (stageViewNow === "both" && !inspectorCollapsed),
      canUndo: undoStack.length > 0 && !operationBusy,
      canRedo: redoStack.length > 0 && !operationBusy,
      canRun: !running,
      canCancel: running
    }).catch((error: unknown) => console.warn("Native shell state sync failed", error));
  }, [Boolean(model), nativeProjectName, modelEdited, projectBusy, shellNow.stage, stageViewNow, uiPreferences.theme, uiPreferences.density,
    nativeResultsEnabled, nativeReviewEnabled, inspectorCollapsed, undoStack.length, redoStack.length, operationBusy, running]);

  // Latest-closure ref so native menu events (registered once) always dispatch
  // against current state rather than the first render. The Rust menu handler
  // injects this DOM event directly into the main webview; this avoids relying
  // on a frontend Tauri event subscription that is not available in the
  // packaged capability set.
  const runMenuCommandRef = useRef(runMenuCommand);
  runMenuCommandRef.current = runMenuCommand;
  useEffect(() => {
    const handleNativeMenuCommand = (event: Event) => {
      if (
        !(event instanceof CustomEvent) ||
        typeof event.detail !== "string" ||
        !isMenuCommandId(event.detail)
      ) {
        return;
      }
      runMenuCommandRef.current(event.detail);
    };
    window.addEventListener("openpipestress-native-menu-command", handleNativeMenuCommand);
    return () => {
      window.removeEventListener("openpipestress-native-menu-command", handleNativeMenuCommand);
    };
  }, []);

  // Bubble after local React handlers so drawers and the palette can consume
  // Escape, including when the unconsumed key originates outside the shell.
  useEffect(() => {
    function handleWorkspaceEscape(event: KeyboardEvent) {
      const shell = workspaceShellRef.current;
      if (!shell || event.key !== "Escape" || event.defaultPrevented) return;
      // Inspect the DOM home, including portalled routing controls, after child
      // controls have had their cancellation opportunity. Model stays docked.
      const target = event.target instanceof Node ? event.target : document.activeElement;
      const inspector = document.getElementById("shell-inspector");
      const surfaces = shell.querySelector(".shell-surfaces");
      if (shellNowRef.current.view === "both" && !shellNowRef.current.location.page &&
          target && inspector?.contains(target) && surfaces && !surfaces.classList.contains("inspector-collapsed")) {
        event.preventDefault();
        runMenuCommandRef.current("view.inspector");
        inspectorToggleRef.current?.focus();
        return;
      }
      viewportViewCommandRef.current?.({ type: "cancel-box-selection" });
      setOpenMenu(null);
      setArmedCreationTool(null);
      // ⎋ accelerates the open page's close control. It never leaves a stage.
      setActiveSection((current) => isStageSurface(current) ? current : sectionAfterPageClose(shellNowRef.current.stageSurface));
      setAuditDrawerOpen(false);
      setIssuesDrawerOpen(false);
      shell.querySelector<HTMLButtonElement>('[data-testid="workspace-select"]')?.focus();
    }
    window.addEventListener("keydown", handleWorkspaceEscape);
    return () => window.removeEventListener("keydown", handleWorkspaceEscape);
  }, []);

  // The shell's two splitters: the Both view's split (the table pane's share of
  // the surface) and the Model view's table drawer. Both are presentation
  // preferences; neither touches the model.
  function handleWorkspaceSplitterKeyDown(
    event: React.KeyboardEvent<HTMLButtonElement>,
    target: "both" | "drawer"
  ) {
    if (target === "both") {
      if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
      event.preventDefault();
      const delta = event.key === "ArrowLeft" ? -2 : 2;
      setUiPreferences((current) => updateUiPreferences(current, { bothSplitPct: current.bothSplitPct + delta }));
      return;
    }
    if (event.key !== "ArrowUp" && event.key !== "ArrowDown") return;
    event.preventDefault();
    const delta = event.key === "ArrowUp" ? 16 : -16;
    setUiPreferences((current) => updateUiPreferences(current, { tableDrawerPx: current.tableDrawerPx + delta }));
  }

  function beginWorkspaceResize(
    event: React.PointerEvent<HTMLButtonElement>,
    target: "both" | "drawer"
  ) {
    event.preventDefault();
    activeResizeCleanupRef.current?.();
    const startX = event.clientX;
    const startY = event.clientY;
    const initial = uiPreferences;
    const surfaceWidth = workspaceBudgetRef.current?.getBoundingClientRect().width ?? 0;
    const move = (pointer: PointerEvent) => {
      const patch = target === "both"
        ? (surfaceWidth > 0 ? { bothSplitPct: initial.bothSplitPct + ((pointer.clientX - startX) / surfaceWidth) * 100 } : {})
        : { tableDrawerPx: initial.tableDrawerPx + startY - pointer.clientY };
      setUiPreferences((current) => updateUiPreferences(current, patch));
    };
    const finish = () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", finish);
      window.removeEventListener("pointercancel", finish);
      if (activeResizeCleanupRef.current === finish) activeResizeCleanupRef.current = null;
    };
    activeResizeCleanupRef.current = finish;
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", finish);
    window.addEventListener("pointercancel", finish);
  }

  function handleNarrowDrawerKeyDown(
    event: React.KeyboardEvent<HTMLDivElement>,
    side: "tree" | "inspector"
  ) {
    if (event.defaultPrevented) return;
    if (side === "inspector" && event.key === "Escape" && shellNowRef.current.view !== "both") return;
    if (window.innerWidth >= 1280 || (side === "tree" ? treeCollapsed : inspectorCollapsed)) return;
    const pane = event.currentTarget;
    if (event.key === "Escape") {
      event.preventDefault();
      if (side === "inspector") routingInspectorRestore.current = null;
      closeWorkspaceRail(side, true);
      return;
    }
    if (event.key !== "Tab") return;
    const focusable = [...pane.querySelectorAll<HTMLElement>(
      'button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [href], [tabindex]:not([tabindex="-1"])'
    )].filter((element) => !element.hidden && !element.closest("[hidden]"));
    if (focusable.length === 0) return;
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }

  function closeWorkspaceRail(side: "tree" | "inspector", restoreFocus = false) {
    const toggle = side === "tree" ? treeToggleRef.current : inspectorToggleRef.current;
    const focused = document.activeElement;
    const pane = side === "tree" ? toggle?.closest(".workspace-pane") : document.getElementById("shell-inspector");
    // Move focus to the persistent opener only when the closing drawer owns
    // focus. A viewport command that closes an overlapping drawer keeps focus
    // on its already-visible initiating control.
    if (restoreFocus || (focused instanceof HTMLElement && pane?.contains(focused))) toggle?.focus();
    if (side === "tree") setTreeCollapsed(true);
    else setInspectorCollapsed(true);
  }

  function exposeViewportForNarrowInteraction(
    _interaction: ViewportExposureInteraction | `authoring-${CreationTool}`
  ) {
    if (window.innerWidth >= 1280) return;
    // Below 1280 px the table drawer is in flow and the inspector slides over
    // the canvas. Mounted children and drafts remain alive as exposure changes.
    // A page over the stage closes onto it; the stage itself is never left.
    closeWorkspaceRail("tree");
    closeWorkspaceRail("inspector");
    closeShellPage();
  }

  function openWorkspaceRail(side: "tree" | "inspector") {
    // Explicit navigation into a property editor supersedes routing's temporary
    // borrowing; its destination must stay open and retain the requested focus.
    if (side === "inspector" && routingPanelActive) routingInspectorRestore.current = null;
    const openingToggle = side === "tree" ? treeToggleRef.current : inspectorToggleRef.current;
    const invokedFromToggle = document.activeElement === openingToggle;
    if (window.innerWidth < 1280) {
      if (side === "tree" && !inspectorCollapsed) closeWorkspaceRail("inspector");
      if (side === "inspector" && !treeCollapsed) closeWorkspaceRail("tree");
      // A page would cover the pane being opened; close it onto its stage,
      // retaining its mounted draft state.
      closeShellPage();
    }
    if (side === "tree") setTreeCollapsed(false);
    else setInspectorCollapsed(false);
    if (invokedFromToggle) openingToggle?.focus();
  }

  function toggleWorkspaceRail(side: "tree" | "inspector") {
    if (side === "inspector" && routingPanelActive) routingInspectorRestore.current = null;
    const collapsed = side === "tree" ? treeCollapsed : inspectorCollapsed;
    if (collapsed) openWorkspaceRail(side);
    else closeWorkspaceRail(side);
  }

  // --- The shell's presentation handlers (slice B3). Each one writes chrome
  // cells only; `setActiveSection` stays the one navigation primitive.

  /** The toolbar's view switch, the View menu and ⌘1 ⌘2 ⌘3: the current stage's view. */
  function chooseStageView(view: ShellView) {
    const stage = shellNowRef.current.location.stage;
    setStageViewMemory((memory) => rememberStageView(memory, stage, view));
  }

  /** The rail and the View menu: enter a stage, unless the rail disables it. */
  function enterStage(stage: ShellStage) {
    if (!railStageState(stage, runPresenceFromCells({ result, historicalRun, solveJob, hasQualifiedCurrentResult: currentSolvedResult !== null })).enabled) return;
    setActiveSection(sectionForStage(stage, shellNowRef.current.stageSurface));
  }

  function rememberShellFocus(event: React.SyntheticEvent<HTMLElement>) {
    if (shellNowRef.current.location.page || !(event.target instanceof Element)) return;
    const target = event.target.closest<HTMLElement>("button, a[href], input, select, textarea, [tabindex], [contenteditable]");
    if (target) pageReturnFocusRef.current = target;
  }

  /** A page's close control (⎋ accelerates it): back to the stage surface it was opened over. */
  function closeShellPage() {
    setActiveSection((current) => isStageSurface(current) ? current : sectionAfterPageClose(shellNowRef.current.stageSurface));
  }

  /**
   * A command that needs the canvas or the inspector is about to return to the
   * Model stage: if that stage was left in Table view, show it in Both view.
   */
  function revealModelStageCanvas() {
    setStageViewMemory((memory) => viewForStage(memory, "model") === "table" ? rememberStageView(memory, "model", "both") : memory);
  }

  // ⌘1 ⌘2 ⌘3 accelerate the view switch, ⌘I the Inspector toggle, and
  // ⌘Z / ⇧⌘Z the same model history handlers as the toolbar and menu.
  // Each key does what its visible control does, and nothing when that control is disabled.
  const shellKeyHandlers = {
    chooseStageView, toggleWorkspaceRail,
    canUndo: undoStack.length > 0 && !operationBusy,
    canRedo: redoStack.length > 0 && !operationBusy
  };
  const shellKeyHandlersRef = useRef(shellKeyHandlers);
  shellKeyHandlersRef.current = shellKeyHandlers;
  useEffect(() => {
    function handleShellAccelerator(event: KeyboardEvent) {
      if ((!event.metaKey && !event.ctrlKey) || event.altKey || event.defaultPrevented) return;
      const key = event.key.toLowerCase();
      if (key === "z") {
        const target = event.target instanceof HTMLElement ? event.target : document.activeElement;
        if (target instanceof HTMLElement && (target.isContentEditable || target.closest('input, textarea, [contenteditable="true"], [contenteditable=""]'))) return;
        const handlers = shellKeyHandlersRef.current;
        if (event.shiftKey ? handlers.canRedo : handlers.canUndo) {
          event.preventDefault();
          runMenuCommandRef.current(event.shiftKey ? "edit.redo" : "edit.undo");
        }
        return;
      }
      // AppKit owns the remaining shell keys. History stays in the webview in
      // both runtimes so editable controls retain their normal text undo.
      if (isTauriRuntime() || event.shiftKey) return;
      const now = shellNowRef.current;
      if (key === "1" || key === "2" || key === "3") {
        const view = SHELL_VIEWS[Number(key) - 1];
        event.preventDefault();
        if (SHELL_STAGES.includes(now.location.stage) && (now.location.stage !== "review" || view === "table")) {
          shellKeyHandlersRef.current.chooseStageView(view);
        }
      } else if (key === "i") {
        event.preventDefault();
        if (now.view === "both") shellKeyHandlersRef.current.toggleWorkspaceRail("inspector");
      }
    }
    window.addEventListener("keydown", handleShellAccelerator);
    return () => window.removeEventListener("keydown", handleShellAccelerator);
  }, []);

  return {
    model: {
      model,
      knowledge,
      modelHash,
      modelAssignment,
      activeModelIndex,
      projectSessionGeneration,
      uiModelRevision,
      modelRevision
    },
    selection: {
      selection,
      orderedSelection,
      orderedSelectionRef,
      hiddenEntityKeys, setHiddenEntityKeys,
      isolationSelectionKeys, setIsolationSelectionKeys,
      effectiveHiddenKeys: viewportVisibility.hiddenKeys,
      dimmedKeys: viewportVisibility.dimmedKeys,
      hiddenCount: viewportVisibility.hiddenCount,
      isolationActive: viewportVisibility.isolationActive,
      handleClearVisibility,
      selectedPipeRefs,
      treePublication,
      handleTreePublication,
      commitSelectionState,
      handleSelectEntity,
      handleBoxSelection
    },
    results: {
      result,
      currentSolvedResult,
      historicalRun,
      analysisRun,
      inputManifest,
      ruleCheckAggregate,
      ruleCheckRunBasis,
      dormantOutputBasis,
      comparison,
      proposal,
      selectedReviewTarget,
      solveJob,
      solveProof,
      running,
      solverMode, setSolverMode,
      reportPackagePrivateIntent, setReportPackagePrivateIntent,
      reportPackageBusy,
      reportPackageRedaction,
      reportPackageRoute,
      handleRun,
      handleInspectBundledReference,
      handleCancelRun,
      handleRuleCheckAggregate,
      handleProposal,
      handleSelectResult,
      handleSelectDiagnostic,
      handleSaveReportPackage
    },
    operations: {
      editorIntents,
      retainedReviewContext,
      operationOutcomes,
      appliedOperations,
      undoStack,
      redoStack,
      queuedBatches,
      batchOutcomes,
      batchReceipts,
      batchMessage,
      requestEpoch,
      getPreparationEpoch,
      directDraftCommitToken,
      operationBusy,
      operationMessage,
      operationEngineStatus,
      operationRequest,
      handleQueueEditorIntent,
      handleClearReviewQueue,
      handleQueueOperationBatch,
      handleRunOperationBatch,
      handleAddDraftReview,
      handleApplyDraftReview,
      handleValidateIntent,
      handleApplyIntent,
      handleUndoSessionModelEdit,
      handleRedoSessionModelEdit,
      invalidateDirectDraftContext
    },
    project: {
      modelEdited,
      storageCapability,
      projectSummary,
      projectIndex,
      modelHashIntegrity,
      projectEnvelopeHash,
      projectEnvelopeHashIntegrity,
      modelDocumentMigration,
      modelMigrationLedger,
      projectMessage,
      projectOperation,
      projectBusy,
      handleCreateProject,
      handleCreateBlankProject,
      handleOpenProject,
      handleSaveProject,
      handleListProjects
    },
    chrome: {
      uiPreferences, setUiPreferences,
      resolvedTheme,
      activeSection, setActiveSection,
      activatedExpensiveSections,
      propertyTaskRequest,
      openMenu, setOpenMenu,
      armedCreationTool,
      viewportViewCommandRef,
      workspaceShellRef,
      workspaceBudgetRef,
      treeCollapsed,
      inspectorCollapsed,
      treeToggleRef,
      inspectorToggleRef,
      operationTab, setOperationTab,
      reviewDetailsOpen, setReviewDetailsOpen,
      auditDrawerOpen, setAuditDrawerOpen,
      issuesDrawerOpen, setIssuesDrawerOpen,
      stageSurface,
      stageViewMemory, setStageViewMemory,
      narrowWindow,
      chooseStageView,
      enterStage,
      closeShellPage,
      rememberShellFocus,
      recordR3JourneyEvent,
      handleArmCreationTool,
      handleToolkitCommand,
      runMenuCommand,
      handleWorkspaceSplitterKeyDown,
      beginWorkspaceResize,
      handleNarrowDrawerKeyDown,
      exposeViewportForNarrowInteraction,
      toggleWorkspaceRail
    }
  };
}

/** What the view receives: the six slices `useWorkspaceSession` returns. */
export type WorkspaceSession = ReturnType<typeof useWorkspaceSession>;
