import { HangerSelectionPanel } from "./features/hanger-selection";
import { SelfWeightPlanPanel } from "./features/self-weight-authoring";
import { OfflineProposalIntakePanel } from "./features/offline-proposal-intake";
import operationSchema from "../../../schemas/model_operation.schema.json";
import { GeometryToolsPanel } from "./features/geometry-tools/GeometryToolsPanel";
import { BoundaryAuthoringPanel } from "./features/boundary-authoring/BoundaryAuthoringPanel";
import { DisplayUnitsProvider, DisplayUnitSelector } from "./features/display-units";
import { applyOperationBatch, validateOperationBatch, type OperationBatch, type OperationBatchOutcome } from "./services/operationBatchService";
import { BatchReviewPanel, type QueuedBatch, type BatchReceipt } from "./features/toolkit/BatchReviewPanel";
import {
  Bot,
  ClipboardCheck,
  Crosshair,
  Database,
  FileWarning,
  FilePlus,
  FolderOpen,
  HardDrive,
  List,
  LockKeyhole,
  Play,
  Save,
  ShieldCheck,
  Sparkles
} from "lucide-react";
import type React from "react";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { AccessibilityBaselinePanel } from "./features/accessibility-baseline/AccessibilityBaselinePanel";
import { AdapterFrameworkPanel } from "./features/adapter-framework/AdapterFrameworkPanel";
import { AgentProposalPanel } from "./features/agent-proposals/AgentProposalPanel";
import { BuildReadinessPanel } from "./features/build-readiness/BuildReadinessPanel";
import { CaepipeExternalHarnessPanel } from "./features/caepipe-external/CaepipeExternalHarnessPanel";
import { CaepipeMbfExportPanel } from "./features/caepipe-mbf/CaepipeMbfExportPanel";
import { ComparisonPanel } from "./features/comparison/ComparisonPanel";
import { DiagnosticsPanel } from "./features/diagnostics/DiagnosticsPanel";
import { DesignWorkspacePanel } from "./features/design-workspace/DesignWorkspacePanel";
import { DiffPreviewPanel } from "./features/diff-preview/DiffPreviewPanel";
import { EditorContractPanel } from "./features/editor-contract/EditorContractPanel";
import { ExportAdapterSdkPanel } from "./features/export-adapter-sdk/ExportAdapterSdkPanel";
import { ExportReviewPanel } from "./features/export-review/ExportReviewPanel";
import { ExternalProverBoundaryPanel } from "./features/external-prover/ExternalProverBoundaryPanel";
import { RedactionExportControlsPanel } from "./features/redaction-controls/RedactionExportControlsPanel";
import { HandoffPanel } from "./features/handoff/HandoffPanel";
import { HeadlessRunnerPanel } from "./features/headless-runner/HeadlessRunnerPanel";
import { KnowledgePanel } from "./features/knowledge/KnowledgePanel";
import { LibraryManagerPanel } from "./features/library/LibraryManagerPanel";
import { LoadCaseManagerPanel } from "./features/load-cases/LoadCaseManagerPanel";
import { LocalFeaHandoffPanel } from "./features/local-fea-handoff/LocalFeaHandoffPanel";
import { MissingDataBlockingPanel, countMissingDataBlockers } from "./features/missing-data/MissingDataBlockingPanel";
import { defaultSelection } from "./features/model-workspace/modelView";
import { ModelTree } from "./features/model-tree/ModelTree";
import { NativePackagePanel } from "./features/native-package/NativePackagePanel";
import { intentKey, OperationApplyPanel } from "./features/operations/OperationApplyPanel";
import { OperationLedgerPanel } from "./features/operations/OperationLedgerPanel";
import { PcfExportPanel } from "./features/pcf-export/PcfExportPanel";
import { ProjectStorageAuditPanel } from "./features/project-storage/ProjectStorageAuditPanel";
import { ProjectValidationPanel } from "./features/project-validation/ProjectValidationPanel";
import { ToolkitPalette } from "./features/toolkit/ToolkitPalette";
import { toolkitCapabilities, capabilityAvailability, capabilityRoute, type ToolkitCapability } from "./features/toolkit/capabilityCatalog";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportLintPanel } from "./features/report-lint/ReportLintPanel";
import { ReportPanel } from "./features/report/ReportPanel";
import { RenderedReportPanel } from "./features/report/RenderedReportPanel";
import { buildReportPackageRequest } from "./features/report/reportPackageRequest";
import { controlReportPackageRequest } from "./features/report/reportRedactionProjector";
import { ResultExportPanel } from "./features/result-export/ResultExportPanel";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { resolveDiagnosticEntitySelection, resolveEntitySelection } from "./features/results/resultInterpretation";
import { ReviewGeometryPanel } from "./features/review-geometry/ReviewGeometryPanel";
import { RuleCheckPanel } from "./features/rule-check/RuleCheckPanel";
import { RuleCheckRunPanel } from "./features/rule-check/RuleCheckRunPanel";
import { RulePackManagerPanel } from "./features/rule-packs/RulePackManagerPanel";
import { RunAuditPanel } from "./features/run-audit/RunAuditPanel";
import { SecretPrivateLibraryPanel } from "./features/secret-private-library/SecretPrivateLibraryPanel";
import { SecurityThreatModelPanel } from "./features/security-threat-model/SecurityThreatModelPanel";
import { SolvePanel } from "./features/solve/SolvePanel";
import { StressNeutralExportPanel } from "./features/stress-neutral/StressNeutralExportPanel";
import { TelemetryBoundaryPanel } from "./features/telemetry/TelemetryBoundaryPanel";
import { ValidationEvidencePanel } from "./features/validation-evidence/ValidationEvidencePanel";
import { PipeViewport, type CreationTool } from "./features/viewport/PipeViewport";
import {
  buildAnalysisRunPreview,
  buildPreviewComparison,
  cancelPreviewMechanicsJob,
  loadDesignKnowledge,
  loadPreviewModel,
  loadSampleProposal,
  pollPreviewMechanicsJob,
  runPreviewMechanics,
  startPreviewMechanicsJob
} from "./services/previewService";
import type {
  BackendSolveJobCancellationReceipt,
  BackendSolveJobStatus,
  PreviewSolverMode,
  SolveJobStartReceipt
} from "./services/previewService";
import {
  applyModelOperation,
  initialOperationEngineStatus,
  validateModelOperation,
  warmupOperationEngine,
  type OperationEngineStatus
} from "./services/operationService";
import type { RuleCheckStatus } from "./services/ruleCheckService";
import {
  buildBlankLocalModelDocument,
  createLocalProject,
  getLocalStorageCapability,
  listLocalProjects,
  openLocalProject,
  saveLocalProject
} from "./services/projectService";
import { computeModelHash, computeProjectEnvelopeHash } from "./services/hashService";
import { isTauriRuntime } from "./services/nativeMenu";
import {
  saveReportPackage,
  type ReportPackageSaveRoute
} from "./services/reportPackageSaveService";
import {
  buildCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence
} from "./services/inputManifestService";
import type { ControlledRouteExport } from "./features/redaction-controls/redactionExportControls";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  AppliedOperationReceipt,
  DesignKnowledge,
  EditorOperationIntent,
  EntityRef,
  LocalProjectIndexEntry,
  LocalProjectSummary,
  LocalStorageCapability,
  MechanicsResult,
  ModelDocumentMigrationStatus,
  ModelHashEvidence,
  ModelHashIntegrityEvidence,
  ModelMigrationLedgerRecord,
  OperationOutcome,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence,
  SelectedReviewTarget,
  SolveJobAuditState
} from "./types";

type SessionModelCheckpoint = {
  checkpoint_id: string;
  operation_id: string;
  model: PreviewModel;
  selection: EntityRef;
};

export type SolveProofEvidence = {
  state: "completed";
  run_generation: number;
  job_id: string;
  backend_job_seam: SolveJobAuditState["backend_job_seam"];
  project_ref: string;
  model_sha256: string;
  input_manifest_sha256: string;
  result_run_id: string;
  result_model_ref: string;
  result_row_count: number;
};

// A run token is acquired synchronously before the first await. This closes the
// native-menu double-dispatch window and also lets model/open invalidation make
// every callback from an older solve inert, even when an adapter reuses a job
// identifier.
export class SolveRunGenerationGate {
  private generation = 0;
  private active: number | null = null;
  private cancelRequested = false;

  tryStart(): number | null {
    if (this.active !== null) return null;
    this.active = ++this.generation;
    this.cancelRequested = false;
    return this.active;
  }

  current(): number | null {
    return this.active;
  }

  isCurrent(token: number): boolean {
    return this.active === token;
  }

  requestCancellation(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.cancelRequested = true;
    return true;
  }

  isCancellationRequested(token: number): boolean {
    return this.isCurrent(token) && this.cancelRequested;
  }

  invalidate(): void {
    this.generation += 1;
    this.active = null;
    this.cancelRequested = false;
  }

  finish(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.active = null;
    this.cancelRequested = false;
    return true;
  }
}

export function commitModelAfterSolveInvalidation(
  gate: SolveRunGenerationGate,
  revision: { current: number },
  commit: () => void
): void {
  revision.current += 1;
  gate.invalidate();
  commit();
}

// TP-APP-R2-UXSHELL-001 workspace information architecture.
//
// PRD section 14.1 names the workspace surfaces; the A12 journey (SMOKE.md
// TP-MAC-141) orders them: model entities -> loads -> solve -> results ->
// report. The shell therefore keeps a persistent spatial core (model tree +
// 3D centerline viewport + property inspector, per PRD 14.1/14.3 and
// DEL-07-02) always on screen, and organizes every other panel behind this
// always-visible section navigation, listed in journey order. The rule-pack
// manager landed as Phase C2 slice 1 (TP-C2-EDITOR-001), placed between
// loads and solve because user rule checks consume authored loads and feed
// the solve/check journey (PRD §22.4). The private library manager landed as
// Phase C3 (TP-C3-LIBGUI-001, PRD §13/§14.6), placed immediately before the
// rule-pack manager because both are private local-only asset managers and
// rule packs reference imported library allowables.
type WorkspaceSectionId =
  | "operations"
  | "loads"
  | "libraries"
  | "rule-packs"
  | "solve"
  | "results"
  | "report"
  | "project"
  | "exports"
  | "evidence";

// CAD-shell menu model (TP-R3UX-CADSHELL). The in-DOM menu bar is the tested
// source of truth; the native macOS menu (Tauri) emits these same command ids.
type MenuId = "file" | "edit" | "view" | "insert" | "analyze";

type MenuCommandId =
  | "file.new-local"
  | "file.new-blank"
  | "file.open-local"
  | "file.list-local"
  | "file.save-local"
  | "file.save-report-package"
  | "edit.undo"
  | "edit.redo"
  | "view.tree"
  | "view.inspector"
  | "view.issues"
  | "view.audit"
  | "view.close-panels"
  | `view.section.${WorkspaceSectionId}`
  | "insert.node"
  | "insert.pipe"
  | "insert.support"
  | "insert.component"
  | "insert.load"
  | "analyze.run"
  | "analyze.cancel"
  | "analyze.rule-checks";

type MenuItemSpec =
  | { kind: "command"; id: MenuCommandId; label: string; disabled?: boolean; active?: boolean }
  | { kind: "separator" };

const WORKSPACE_SECTIONS: ReadonlyArray<{ id: WorkspaceSectionId; label: string; description: string }> = [
  {
    id: "operations",
    label: "Operation Apply",
    description: "Queued structured operations, apply/undo/redo, diffs, and the operation review ledger"
  },
  {
    id: "loads",
    label: "Load Cases",
    description: "Load-case manager: create load cases, primitive loads, and combinations"
  },
  {
    id: "libraries",
    label: "Libraries",
    description:
      "Private, local-only library manager: import material/section/component libraries with provenance, validation findings, and the local store"
  },
  {
    id: "rule-packs",
    label: "Rule Packs",
    description:
      "Private, local-only rule-pack manager: drafts, validation findings, checksum generation, and the local store"
  },
  {
    id: "solve",
    label: "Solve",
    description: "Run the mechanics preview, solve job audit, diagnostics, and missing-data review"
  },
  {
    id: "results",
    label: "Results",
    description: "Results browser, comparison workspace, and design-authoring state"
  },
  {
    id: "report",
    label: "Report",
    description: "Rendered calculation report, report packet, and report content lint"
  },
  {
    id: "project",
    label: "Project",
    description: "Local project storage audit and validation preflight"
  },
  {
    id: "exports",
    label: "Exports",
    description: "Result/geometry exports, exchange adapters, handoff packages, and export review"
  },
  {
    id: "evidence",
    label: "Audit & Boundaries",
    description: "Run audit, validation evidence, telemetry/privacy/security boundary reviews"
  }
];

const NATIVE_MENU_COMMAND_IDS: ReadonlySet<string> = new Set([
  "file.new-local",
  "file.new-blank",
  "file.open-local",
  "file.list-local",
  "file.save-local",
  "file.save-report-package",
  "edit.undo",
  "edit.redo",
  "view.tree",
  "view.inspector",
  "view.issues",
  "view.audit",
  "view.close-panels",
  ...WORKSPACE_SECTIONS.map((section) => `view.section.${section.id}`),
  "insert.node",
  "insert.pipe",
  "insert.support",
  "insert.component",
  "insert.load",
  "analyze.run",
  "analyze.cancel",
  "analyze.rule-checks"
]);

function isMenuCommandId(value: string): value is MenuCommandId {
  return NATIVE_MENU_COMMAND_IDS.has(value);
}

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

type R3JourneyEvent =
  | "library_template_loaded"
  | "library_validate_requested"
  | "library_save_requested"
  | "rule_pack_draft_created"
  | "rule_pack_validate_requested"
  | "rule_pack_checksum_requested"
  | "rule_pack_save_requested"
  | "rule_check_pack_loaded"
  | "rule_check_run_requested";

type R3JourneyState = Record<R3JourneyEvent, boolean>;

const INITIAL_R3_JOURNEY_STATE: R3JourneyState = {
  library_template_loaded: false,
  library_validate_requested: false,
  library_save_requested: false,
  rule_pack_draft_created: false,
  rule_pack_validate_requested: false,
  rule_pack_checksum_requested: false,
  rule_pack_save_requested: false,
  rule_check_pack_loaded: false,
  rule_check_run_requested: false
};

export function App() {
  return <DisplayUnitsProvider><AppSession /></DisplayUnitsProvider>;
}

function AppSession() {
  const [model, setModel] = useState<PreviewModel | null>(null);
  const [knowledge, setKnowledge] = useState<DesignKnowledge | null>(null);
  const [selection, setSelection] = useState<EntityRef | null>(null);
  const [result, setResult] = useState<MechanicsResult | null>(null);
  const [analysisRun, setAnalysisRun] = useState<AnalysisRunEnvelope | null>(null);
  const [inputManifest, setInputManifest] =
    useState<CurrentSessionInputManifestEvidence | null>(null);
  // Worst-of rule-check aggregate from the GUI run panel, lifted so it can be
  // recorded in the app-held analysis-run envelope (TP-C4-APPAGG-001).
  const [ruleCheckAggregate, setRuleCheckAggregate] = useState<RuleCheckStatus | null>(null);
  const [proposal, setProposal] = useState<AgentProposal | null>(null);
  const [editorIntents, setEditorIntents] = useState<EditorOperationIntent[]>([]);
  const [retainedReviewContext, setRetainedReviewContext] = useState<EditorOperationIntent[]>([]);
  const projectRequest = useRef(0);
  const [selectedReviewTarget, setSelectedReviewTarget] = useState<SelectedReviewTarget | null>(null);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  const [projectSummary, setProjectSummary] = useState<LocalProjectSummary | null>(null);
  const [projectIndex, setProjectIndex] = useState<LocalProjectIndexEntry[] | null>(null);
  const [modelHash, setModelHash] = useState<ModelHashEvidence | null>(null);
  const [modelHashIntegrity, setModelHashIntegrity] = useState<ModelHashIntegrityEvidence | null>(null);
  const [projectEnvelopeHash, setProjectEnvelopeHash] = useState<ProjectEnvelopeHashEvidence | null>(null);
  const [modelDocumentMigration, setModelDocumentMigration] = useState<ModelDocumentMigrationStatus | null>(null);
  const [modelMigrationLedger, setModelMigrationLedger] = useState<ModelMigrationLedgerRecord[]>([]);
  const [projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity] =
    useState<ProjectEnvelopeHashIntegrityEvidence | null>(null);
  const [projectMessage, setProjectMessage] = useState("Local project store not opened.");
  const [projectOperation, setProjectOperation] = useState("not_started");
  const [solveJob, setSolveJob] = useState<SolveJobAuditState>(() => initialSolveJob());
  const [solveProof, setSolveProof] = useState<SolveProofEvidence | null>(null);
  const [running, setRunning] = useState(false);
  const [solverMode, setSolverMode] = useState<PreviewSolverMode>("sparse_interactive");
  const [projectBusy, setProjectBusy] = useState(false);
  const [reportPackagePrivateIntent, setReportPackagePrivateIntent] = useState(false);
  const [reportPackageBusy, setReportPackageBusy] = useState(false);
  const [reportPackageRedaction, setReportPackageRedaction] = useState<ControlledRouteExport | null>(null);
  const [reportPackageRoute, setReportPackageRoute] = useState<ReportPackageSaveRoute | null>(null);
  const [operationOutcomes, setOperationOutcomes] = useState<Record<string, OperationOutcome>>({});
  const [appliedOperations, setAppliedOperations] = useState<AppliedOperationReceipt[]>([]);
  const [undoStack, setUndoStack] = useState<SessionModelCheckpoint[]>([]);
  const [redoStack, setRedoStack] = useState<SessionModelCheckpoint[]>([]);
  const [queuedBatches, setQueuedBatches] = useState<QueuedBatch[]>([]);
  const [batchOutcomes, setBatchOutcomes] = useState<Record<string, OperationBatchOutcome>>({});
  const [batchReceipts, setBatchReceipts] = useState<BatchReceipt[]>([]);
  const [batchMessage, setBatchMessage] = useState<string | null>(null);
  const [requestEpoch, setRequestEpoch] = useState(0);
  const requestEpochRef = useRef(0);
  const batchSequence = useRef(0);
  const [operationBusy, setOperationBusy] = useState(false);
  const [operationMessage, setOperationMessage] = useState<string | null>(null);
  const [operationEngineStatus, setOperationEngineStatus] = useState<OperationEngineStatus>(() =>
    initialOperationEngineStatus()
  );
  // CAD-shell IA (TP-R3UX-CADSHELL): the dock starts collapsed so the spatial
  // core (model tree | 3D viewport | inspector) owns the surface; workspace
  // sections are summoned from the View menu and dismissed back to the viewport.
  const [activeSection, setActiveSection] = useState<WorkspaceSectionId | null>(null);
  const [toolkitFocus, setToolkitFocus] = useState<{ testId: string; elementId?: string } | null>(null);
  useLayoutEffect(() => {
    if (!toolkitFocus) return;
    const target = toolkitFocus.elementId ? document.getElementById(toolkitFocus.elementId) : document.querySelector<HTMLElement>(`[data-testid="${toolkitFocus.testId}"]`);
    target?.focus();
    target?.scrollIntoView?.({ block: "nearest" });
  }, [toolkitFocus]);
  const [openMenu, setOpenMenu] = useState<MenuId | null>(null);
  const [armedCreationTool, setArmedCreationTool] = useState<CreationTool | null>(null);
  // Viewport-first agent-mediated shell (TP-R3UX-AGENTSHELL-001): the detailed
  // tree and property inspector start tucked away so the primary screen is the
  // 3D model plus a local review-only agent workbench. The detailed rails remain
  // available from View for targeted investigation.
  const [treeCollapsed, setTreeCollapsed] = useState(true);
  const [inspectorCollapsed, setInspectorCollapsed] = useState(true);
  const [r3JourneyState, setR3JourneyState] = useState<R3JourneyState>(() => ({
    ...INITIAL_R3_JOURNEY_STATE
  }));
  const [reviewDetailsOpen, setReviewDetailsOpen] = useState(false);
  const [auditDrawerOpen, setAuditDrawerOpen] = useState(false);
  const [issuesDrawerOpen, setIssuesDrawerOpen] = useState(false);
  const intentSequence = useRef(0);
  const modelRevision = useRef(0);
  const currentModel = useRef<PreviewModel | null>(null);
  const operationRequest = useRef({ sequence: 0, busy: false });
  const solveRunGate = useRef(new SolveRunGenerationGate());
  const activeSolveJob = useRef<{
    generation: number;
    job: SolveJobAuditState;
    cancellation_dispatched: boolean;
  } | null>(null);
  const solveCancellationTombstones = useRef(new Map<number, {
    requested: boolean;
    dispatched: boolean;
  }>());
  const comparison = useMemo(
    () => (result && analysisRun ? buildPreviewComparison({ result, analysisRun }) : null),
    [analysisRun, result]
  );

  useEffect(() => {
    let active = true;
    Promise.all([loadPreviewModel(), loadDesignKnowledge(), getLocalStorageCapability()]).then(
      ([loadedModel, loadedKnowledge, loadedStorageCapability]) => {
        if (!active) return;
        commitModel(loadedModel);
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
    modelRevision.current += 1;
    solveRunGate.current.invalidate();
    activeSolveJob.current = null;
    setRunning(false);
    setSolveProof(null);
    setModelHash(null);
    if (!model) return;
    computeModelHash(model).then((hash) => {
      if (active) setModelHash(hash);
    });
    return () => {
      active = false;
    };
  }, [model]);

  function commitModel(nextModel: PreviewModel) {
    requestEpochRef.current += 1;
    setRequestEpoch(requestEpochRef.current);
    currentModel.current = nextModel;
    operationRequest.current.sequence += 1;
    operationRequest.current.busy = false;
    setOperationBusy(false);
    commitModelAfterSolveInvalidation(solveRunGate.current, modelRevision, () => {
      activeSolveJob.current = null;
      setRunning(false);
      setSolveProof(null);
      setModelHash(null);
      setModel(nextModel);
    });
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

  async function handleRun() {
    const runGeneration = solveRunGate.current.tryStart();
    if (runGeneration === null) return;
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
        output = await runPreviewMechanics(solveModel, solverMode);
        if (solveRunGate.current.isCancellationRequested(runGeneration)) return;
      }
      const manifest = await buildCurrentSessionInputManifest({
        model: solveModel,
        solver: {
          solver_name: "open_pipe_stress_product_physics",
          solver_version: "0.1.0",
          solver_build_ref: "open_pipe_stress_product_physics@0.1.0",
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
  async function handleRuleCheckAggregate(aggregate: RuleCheckStatus | null) {
    if (aggregate === ruleCheckAggregate) return;
    setRuleCheckAggregate(aggregate);
    if (!result || !inputManifest) return;
    try {
      setAnalysisRun(
        await buildAnalysisRunPreview(result, {
          inputManifest,
          ruleCheckAggregate: aggregate
        })
      );
    } catch {
      // Recording the aggregate failed (e.g. hashing unavailable); keep the
      // solve-time analysis-run envelope rather than surfacing a false outcome.
      setRuleCheckAggregate(null);
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
      setSelection(defaultSelection(outcome.applied_model));
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

  async function handleApplyIntent(intent: EditorOperationIntent) {
    if (!model || operationRequest.current.busy) return;
    const revision = modelRevision.current;
    const request = ++operationRequest.current.sequence;
    operationRequest.current.busy = true;
    const stillCurrent = () => operationRequest.current.sequence === request && modelRevision.current === revision;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const initialHash = await computeModelHash(model);
      if (!stillCurrent() || !initialHash) return;
      const outcome = await applyModelOperation(model, intent, initialHash);
      if (!stillCurrent() || !currentModel.current) return;
      const currentHash = await computeModelHash(currentModel.current);
      // The hash computation is asynchronous too; generation must still match
      // after it resolves before any outcome, receipt or checkpoint is published.
      if (!stillCurrent() || currentHash?.value !== initialHash.value) return;
      setOperationOutcomes((current) => ({ ...current, [intentKey(intent)]: outcome }));
      if (outcome.validation.application_status !== "applied_to_session_model" || !outcome.applied_model) {
        setOperationMessage(
          `Operation ${outcome.operation_id} was not applied (${outcome.validation.application_status}); see its diagnostics.`
        );
        return;
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
      if (
        outcome.change_kind === "delete_material" ||
        outcome.change_kind === "delete_section" ||
        outcome.change_kind === "delete_component" ||
        outcome.change_kind === "delete_support" ||
        outcome.change_kind === "delete_pipe_run" ||
        outcome.change_kind === "delete_node"
      ) {
        setSelection(defaultSelection(outcome.applied_model));
      } else if (appliedSelection) {
        setSelection(appliedSelection);
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
    } catch (error) {
      if (stillCurrent()) setOperationMessage(`Operation apply failed to run: ${String(error)}`);
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
    void handleApplyIntent(nextIntent);
  }

  function handleUndoSessionModelEdit() {
    if (!model || !selection || undoStack.length === 0) return;
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
    if (!model || !selection || redoStack.length === 0) return;
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
    setResult(null);
    setAnalysisRun(null);
    setInputManifest(null);
    setRuleCheckAggregate(null);
    setProposal(null);
    setSelectedReviewTarget(null);
    setSolveJob(nextSolveJob);
  }

  async function handleCreateProject() {
    if (!model) return;
    const request = ++projectRequest.current;
    const epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    const combinedContext = structuredClone([...retainedReviewContext, ...editorIntents, ...queuedBatches.flatMap((entry) => entry.batch.operations)]);
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
      const snapshotModelHash = await computeModelHash(model);
      if (!stillCurrent()) return;
      const envelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: combinedContext,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: result,
        analysis_run: analysisRun,
        model_hash: snapshotModelHash
      });
      if (!stillCurrent()) return;
      const created = await createLocalProject(
        model,
        combinedContext,
        proposal,
        selectedReviewTarget,
        result,
        analysisRun,
        snapshotModelHash,
        envelopeHash
      );
      if (!stillCurrent()) return;
      setProjectSummary(created.summary);
      setProposal(created.proposal ?? null);
      setSelectedReviewTarget(created.selected_review_target ?? null);
      setProjectEnvelopeHash(created.project_envelope_hash ?? null);
      setModelDocumentMigration(created.model_document_migration ?? null);
      setModelMigrationLedger(created.model_migration_ledger ?? []);
      setProjectMessage(created.summary.message);
      setProjectOperation("create");
    } catch (error) {
      if (!stillCurrent()) return;
      setProjectMessage(`Create failed: ${String(error)}`);
      setProjectOperation("create_failed");
    } finally {
      if (request === projectRequest.current) setProjectBusy(false);
    }
  }

  async function handleCreateBlankProject() {
    const blankModel = buildBlankLocalModelDocument();
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
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
      const createdSummary = {
        ...created.summary,
        message: "Created blank local model document without fixture entities or external file copies."
      };
      commitModel(created.model);
      epoch = requestEpochRef.current;
      setSelection(defaultSelection(created.model));
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
      if (request === projectRequest.current) setProjectBusy(false);
    }
  }

  async function handleOpenProject(projectId: string | null = null) {
    const request = ++projectRequest.current;
    let epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
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
      const restoredResult = opened.mechanics_result ?? null;
      const restoredAnalysisRun = opened.analysis_run ?? null;
      commitModel(opened.model);
      epoch = requestEpochRef.current;
      setSelection(defaultSelection(opened.model));
      setUndoStack([]);
      setRedoStack([]);
      setAppliedOperations([]);
      setQueuedBatches([]);
      setBatchOutcomes({});
      setBatchReceipts([]);
      setBatchMessage(null);
      setResult(restoredResult);
      setAnalysisRun(restoredAnalysisRun);
      // Persisted analysis-run refs do not include the exact current-session
      // manifest payload needed to recompute its hash. A new solve is required
      // before report-package save can become ready.
      setInputManifest(null);
      setRuleCheckAggregate(null);
      setProposal(opened.proposal ?? null);
      setRetainedReviewContext(opened.editor_intents ?? []);
      setEditorIntents([]);
      setSelectedReviewTarget(opened.selected_review_target ?? null);
      setSolveJob(
        restoredResult && restoredAnalysisRun
          ? restoredSolveJob(restoredResult, restoredAnalysisRun)
          : initialSolveJob()
      );
      setProjectSummary(opened.summary);
      setProjectEnvelopeHash(opened.project_envelope_hash ?? null);
      setModelDocumentMigration(opened.model_document_migration ?? null);
      setModelMigrationLedger(opened.model_migration_ledger ?? []);
      setProjectMessage(opened.summary.message);
      setProjectOperation(projectId ? "open_by_id" : "open");
      const recomputedHash = await computeModelHash(opened.model);
      if (!stillCurrent()) return;
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
      if (!stillCurrent()) return;
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
      if (request === projectRequest.current) setProjectBusy(false);
    }
  }

  async function handleSaveProject() {
    if (!model) return;
    const request = ++projectRequest.current;
    const epoch = requestEpochRef.current;
    const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;
    const combinedContext = structuredClone([...retainedReviewContext, ...editorIntents, ...queuedBatches.flatMap((entry) => entry.batch.operations)]);
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
      const snapshotModelHash = await computeModelHash(model);
      if (!stillCurrent()) return;
      const envelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: combinedContext,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: result,
        analysis_run: analysisRun,
        model_hash: snapshotModelHash
      });
      if (!stillCurrent()) return;
      const saved = await saveLocalProject(
        model,
        combinedContext,
        proposal,
        selectedReviewTarget,
        result,
        analysisRun,
        snapshotModelHash,
        envelopeHash,
        modelDocumentMigration
      );
      if (!stillCurrent()) return;
      setProjectSummary(saved.summary);
      setProposal(saved.proposal ?? null);
      setSelectedReviewTarget(saved.selected_review_target ?? null);
      setProjectEnvelopeHash(saved.project_envelope_hash ?? null);
      setModelDocumentMigration(saved.model_document_migration ?? null);
      setModelMigrationLedger(saved.model_migration_ledger ?? []);
      setProjectMessage(saved.summary.message);
      setProjectOperation("save");
    } catch (error) {
      if (!stillCurrent()) return;
      setProjectMessage(`Save failed: ${String(error)}`);
      setProjectOperation("save_failed");
    } finally {
      if (request === projectRequest.current) setProjectBusy(false);
    }
  }

  async function handleListProjects() {
    setProjectBusy(true);
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
      setProjectBusy(false);
    }
  }

  function handleSelectResult(resultId: string) {
    setSelectedReviewTarget({ target_type: "result", id: resultId });
    const item = result?.results.find((candidate) => candidate.id === resultId);
    if (!item || !model) return;
    const entitySelection = resolveEntitySelection(model, item.entity_ref);
    if (entitySelection) {
      setSelection(entitySelection);
    }
  }

  function handleSelectDiagnostic(diagnosticId: string) {
    setSelectedReviewTarget({ target_type: "diagnostic", id: diagnosticId });
    if (!model) return;
    const entitySelection = resolveDiagnosticEntitySelection({ model, result, knowledge, diagnosticId });
    if (entitySelection) {
      setSelection(entitySelection);
    }
  }

  function handleSelectEntity(entity: EntityRef) {
    setSelection(entity);
  }

  function handleArmCreationTool(tool: CreationTool | null) {
    setArmedCreationTool(tool);
    if (!tool) return;
    if (tool === "load") {
      setActiveSection("loads");
      return;
    }
    setActiveSection(null);
    if (tool === "support") {
      setInspectorCollapsed(false);
    }
  }

  function handleToolkitCommand(capability: ToolkitCapability) {
    if (!selection) return;
    const context = { selection, canUndo: undoStack.length > 0, canRedo: redoStack.length > 0, busy: operationBusy, windConfigured: Boolean(selection.type === "load" && model?.load_cases.find((load) => load.id === selection.id)?.equivalent_static?.wind) };
    if (!capabilityAvailability(capability, context).enabled) return;
    if (capability.history) {
      if (capability.history === "undo") handleUndoSessionModelEdit();
      else handleRedoSessionModelEdit();
      setToolkitFocus({ testId: "toolkit-entry" });
      return;
    }
    const route = capabilityRoute(capability, context);
    if (!route) return;
    setArmedCreationTool(route.tool ?? null);
    if (route.surface === "inspector") { setInspectorCollapsed(false); setActiveSection(null); }
    else if (route.surface === "tree") { setTreeCollapsed(false); setActiveSection(null); }
    else if (route.surface === "viewport") setActiveSection(null);
    else setActiveSection(route.surface);
    setToolkitFocus({ testId: route.focusTestId, elementId: route.elementId });
  }

  async function handleSaveReportPackage() {
    if (!model || !result || !analysisRun || !inputManifest || running || reportPackageBusy) return;
    setReportPackageBusy(true);
    setReportPackageRoute(null);
    try {
      const request = await buildReportPackageRequest({
        model,
        result,
        analysisRun,
        inputManifest,
        projectSummary,
        comparison,
        ruleCheckAggregate
      });
      const controlled = controlReportPackageRequest(request, reportPackagePrivateIntent);
      setReportPackageRedaction(controlled);
      if (controlled.blocked || controlled.payload === null) {
        setReportPackageRoute({
          route: "redaction_blocked",
          diagnostic: `REPORT-PACKAGE-REDACTION-BLOCKED: ${controlled.findings.length} finding(s) prevent package assembly or persistence.`
        });
        return;
      }
      setReportPackageRoute(await saveReportPackage(controlled));
    } catch (error) {
      setReportPackageRedaction(null);
      setReportPackageRoute({
        route: "redaction_blocked",
        diagnostic: formatPackageSaveError(error)
      });
    } finally {
      setReportPackageBusy(false);
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
        setActiveSection(null);
        break;
      case "view.tree":
        setTreeCollapsed((collapsed) => !collapsed);
        break;
      case "view.inspector":
        setInspectorCollapsed((collapsed) => !collapsed);
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
          setActiveSection((current) => (current === sectionId ? null : sectionId));
        }
        break;
      }
    }
  }

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

  // In the packaged Tauri shell the OS-level menu bar is the single menu, so the
  // in-DOM menu bar is suppressed to avoid a redundant second row. In the
  // browser/Playwright preview there is no native menu, so the in-DOM bar
  // renders and remains the navigable, tested source of truth.
  const showInAppMenuBar = !isTauriRuntime();

  if (!model || !selection) {
    return <div className="loading-screen">Loading local OpenPipeStress preview fixture.</div>;
  }

  return (
    <main
      className={showInAppMenuBar ? "app-shell" : "app-shell native-menu"}
      data-testid="desktop-preview-shell"
    >
      <header className="titlebar">
        <div>
          <h1>OpenPipeStress</h1>
          <p>{projectSummary?.project_name ?? model.project.name}</p>
        </div>
        <div className="titlebar-actions" aria-label="Local project controls">
          <div className="display-preference-control"><DisplayUnitSelector /></div>
          <span className="titlebar-project-name">{projectSummary?.project_name ?? model.project.name}</span>
          <button type="button" onClick={handleCreateProject} disabled={projectBusy}>
            <Database size={15} aria-hidden="true" />
            Create local
          </button>
          <button type="button" onClick={handleCreateBlankProject} disabled={projectBusy}>
            <FilePlus size={15} aria-hidden="true" />
            New blank
          </button>
          <button type="button" onClick={() => handleOpenProject()} disabled={projectBusy}>
            <FolderOpen size={15} aria-hidden="true" />
            Open local
          </button>
          <button type="button" onClick={handleListProjects} disabled={projectBusy}>
            <List size={15} aria-hidden="true" />
            List local
          </button>
          <button type="button" onClick={handleSaveProject} disabled={projectBusy}>
            <Save size={15} aria-hidden="true" />
            Save local
          </button>
        </div>
      </header>

      <section className="project-strip" aria-label="Project summary">
        <span data-testid="local-project-message">{projectMessage}</span>
        <span data-testid="local-project-review-context">{projectReviewContext(editorIntents, proposal, appliedOperations.length)}</span>
        {projectIndex && projectIndex.length > 0 ? (
          <div className="project-index-picker" data-testid="project-index-picker" aria-label="Open listed project by id">
            {projectIndex.map((entry) => (
              <button
                key={entry.project_id}
                type="button"
                data-testid={`project-index-open-${entry.project_id}`}
                onClick={() => handleOpenProject(entry.project_id)}
                disabled={projectBusy}
              >
                <FolderOpen size={15} aria-hidden="true" />
                {entry.project_name} ({entry.project_id})
              </button>
            ))}
          </div>
        ) : null}
      </section>

      {showInAppMenuBar ? (
        <MenuBar
          activeSection={activeSection}
          auditOpen={auditDrawerOpen}
          canRedo={redoStack.length > 0}
          canUndo={undoStack.length > 0}
          inspectorCollapsed={inspectorCollapsed}
          issuesOpen={issuesDrawerOpen}
          openMenu={openMenu}
          projectBusy={projectBusy}
          reportPackageReady={Boolean(result && analysisRun && inputManifest) && !running && !reportPackageBusy}
          running={running}
          treeCollapsed={treeCollapsed}
          armedCreationTool={armedCreationTool}
          onCommand={runMenuCommand}
          onOpenMenu={setOpenMenu}
        />
      ) : null}

      <div className={activeSection ? "workspace" : "workspace dock-collapsed"}>
        <section
          className={`modeling-workspace${treeCollapsed ? " tree-collapsed" : ""}${
            inspectorCollapsed ? " inspector-collapsed" : ""
          }`}
          aria-label="Modeling workspace"
          data-testid="modeling-workspace"
        >
          <div className="workspace-pane workspace-pane-tree">
            <button
              type="button"
              className="workspace-pane-toggle"
              data-testid="toggle-tree"
              aria-expanded={!treeCollapsed}
              aria-label={treeCollapsed ? "Expand model tree" : "Collapse model tree"}
              title={treeCollapsed ? "Expand model tree" : "Collapse model tree"}
              onClick={() => setTreeCollapsed((collapsed) => !collapsed)}
            >
              <span className="workspace-pane-toggle-label">Model Tree</span>
              <span className="workspace-pane-toggle-icon" aria-hidden="true">
                {treeCollapsed ? "›" : "‹"}
              </span>
            </button>
            <ModelTree model={model} selection={selection} onQueueIntent={handleQueueEditorIntent} onSelect={handleSelectEntity} />
          </div>
          <div className="workspace-pane workspace-pane-viewport">
            <ToolkitPalette
              context={{ selection, canUndo: undoStack.length > 0, canRedo: redoStack.length > 0, busy: operationBusy, windConfigured: Boolean(selection.type === "load" && model.load_cases.find((load) => load.id === selection.id)?.equivalent_static?.wind) }}
              onChoose={handleToolkitCommand}
            />
            <PipeViewport
              armedCreationTool={armedCreationTool}
              model={model}
              onArmCreationTool={handleArmCreationTool}
              onQueueIntent={handleQueueEditorIntent}
              onSelect={handleSelectEntity}
              queuedIntents={editorIntents}
              result={result}
              selection={selection}
            />
          </div>
          <div className="workspace-pane workspace-pane-agent">
            <AgentWorkbenchPanel
              appliedOperationCount={appliedOperations.length}
              mechanicsReady={Boolean(result)}
              model={model}
              proposal={proposal}
              queuedIntentCount={editorIntents.length}
              running={running}
              selection={selection}
              selectedReviewTarget={selectedReviewTarget}
              statusText={r3ExitJourneyStatus({ result, ruleCheckAggregate, projectSummary })}
              onGenerateProposal={handleProposal}
              onOpenOperations={() => setActiveSection("operations")}
              onOpenResults={() => setActiveSection("results")}
              onRunMechanics={handleRun}
            />
          </div>
          <div className="workspace-pane workspace-pane-inspector">
            <button
              type="button"
              className="workspace-pane-toggle"
              data-testid="toggle-inspector"
              aria-expanded={!inspectorCollapsed}
              aria-label={inspectorCollapsed ? "Expand inspector" : "Collapse inspector"}
              title={inspectorCollapsed ? "Expand inspector" : "Collapse inspector"}
              onClick={() => setInspectorCollapsed((collapsed) => !collapsed)}
            >
              <span className="workspace-pane-toggle-label">Inspector</span>
              <span className="workspace-pane-toggle-icon" aria-hidden="true">
                {inspectorCollapsed ? "‹" : "›"}
              </span>
            </button>
            <PropertyInspector
              model={model}
              onQueueIntent={handleQueueEditorIntent}
              onValidateIntent={handleValidateIntent}
              operationBusy={operationBusy}
              operationOutcomes={operationOutcomes}
              queuedIntents={editorIntents}
              selection={selection}
            />
          </div>
        </section>

        <section
          className={activeSection ? "workspace-dock" : "workspace-dock collapsed"}
          aria-label="Workspace sections"
          data-testid="workspace-dock"
        >
          {activeSection ? (
            <header className="workspace-dock-header" data-testid="workspace-dock-header">
              <h2>{WORKSPACE_SECTIONS.find((candidate) => candidate.id === activeSection)?.label ?? activeSection}</h2>
              <button type="button" data-testid="workspace-dock-close" onClick={() => setActiveSection(null)}>
                Close panel
              </button>
            </header>
          ) : null}
          <div className="workspace-dock-body">
            <section
              className={dockSectionClass("operations", activeSection)}
              aria-label="Operation Apply section"
              data-testid="workspace-section-operations"
              tabIndex={-1}
            >
              <section className="panel" aria-label="Stored proposed review context">
                <h3>Stored proposed review context</h3>
                <p data-testid="retained-context-summary">{retainedReviewContext.length} retained operation records. Acceptance unknown after reopening; these records do not prove the current model contains a change. Saved member metadata does not restore batch grouping, receipts or undo checkpoints.</p>
                {retainedReviewContext.map((intent, index) => (
                  <details key={index}>
                    <summary>{String(intent?.operation_id ?? "Unrecognized operation record")} — proposed context; acceptance unknown</summary>
                    <pre>{JSON.stringify(intent, null, 2)}</pre>
                    <button type="button" data-testid={`requeue-context-${index}`} disabled={operationBusy} title={operationBusy ? "Wait for the current operation to finish." : undefined}
                      onClick={() => void handleQueueOperationBatch({ batch_id: `requeue-${index}`, operations: [intent] })}>
                      Requeue for fresh validation
                    </button>
                  </details>
                ))}
              </section>
              <HangerSelectionPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              <SelfWeightPlanPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              <OfflineProposalIntakePanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
                capabilityReference={{
                  capabilities: toolkitCapabilities,
                  operation_schema: operationSchema,
                  provider_status: "Live provider is held; offline proposals require explicit human review."
                }}
              />
              <GeometryToolsPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              <BoundaryAuthoringPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              <BatchReviewPanel
                onClear={handleClearReviewQueue}
                batches={queuedBatches} outcomes={batchOutcomes} receipts={batchReceipts}
                revision={modelRevision.current} busy={operationBusy} message={batchMessage}
                onValidate={(entry) => void handleRunOperationBatch(entry, false)}
                onApply={(entry) => void handleRunOperationBatch(entry, true)}
              />
              <OperationApplyPanel
                queuedIntents={editorIntents}
                outcomes={operationOutcomes}
                appliedOperations={appliedOperations}
                undoCount={undoStack.length}
                redoCount={redoStack.length}
                busy={operationBusy}
                message={operationMessage}
                engineStatus={operationEngineStatus}
                onValidate={handleValidateIntent}
                onApply={handleApplyIntent}
                onUndo={handleUndoSessionModelEdit}
                onRedo={handleRedoSessionModelEdit}
              />
              <section
                className={reviewDetailsOpen ? "review-apply-drawer open" : "review-apply-drawer"}
                aria-label="Review and apply detail views"
                data-testid="review-apply-drawer"
              >
                <div className="review-apply-drawer-header">
                  <div>
                    <span>Detail views</span>
                    <h2>Review evidence</h2>
                    <p>
                      Editor contract, operation diffs, review ledger, and agent proposal context stay available here
                      without owning the default work surface.
                    </p>
                  </div>
                  <button
                    type="button"
                    aria-expanded={reviewDetailsOpen}
                    data-testid="review-apply-drawer-toggle"
                    onClick={() => setReviewDetailsOpen((open) => !open)}
                  >
                    {reviewDetailsOpen ? "Hide details" : "Show details"}
                  </button>
                </div>
                <div className="review-apply-detail-grid">
                  <EditorContractPanel editorIntents={editorIntents} model={model} />
                  <DiffPreviewPanel
                    model={model}
                    analysisRun={analysisRun}
                    editorIntents={editorIntents}
                    proposal={proposal}
                    selectedReviewTarget={selectedReviewTarget}
                  />
                  <OperationLedgerPanel
                    model={model}
                    analysisRun={analysisRun}
                    editorIntents={editorIntents}
                    proposal={proposal}
                    selectedReviewTarget={selectedReviewTarget}
                    onClearReviewQueue={handleClearReviewQueue}
                  />
                  <AgentProposalPanel
                    proposal={proposal}
                    mechanicsReady={Boolean(result)}
                    selectedReviewTarget={selectedReviewTarget}
                    onLoad={handleProposal}
                  />
                </div>
              </section>
            </section>

            <section
              className={dockSectionClass("loads", activeSection)}
              aria-label="Load Cases section"
              data-testid="workspace-section-loads"
            >
              <LoadCaseManagerPanel
                model={model}
                onQueueIntent={handleQueueEditorIntent}
                onSelect={handleSelectEntity}
                selection={selection}
              />
            </section>

            <section
              className={dockSectionClass("libraries", activeSection)}
              aria-label="Libraries section"
              data-testid="workspace-section-libraries"
              tabIndex={-1}
            >
              <LibraryManagerPanel model={model} onR3JourneyEvent={recordR3JourneyEvent} />
            </section>

            <section
              className={dockSectionClass("rule-packs", activeSection)}
              aria-label="Rule Packs section"
              data-testid="workspace-section-rule-packs"
            >
              <RulePackManagerPanel model={model} onR3JourneyEvent={recordR3JourneyEvent} />
            </section>

            <section
              className={dockSectionClass("solve", activeSection)}
              aria-label="Solve section"
              data-testid="workspace-section-solve"
            >
              <SolvePanel
                analysisRun={analysisRun}
                model={model}
                result={result}
                running={running}
                solveJob={solveJob}
                solverMode={solverMode}
                onCancel={handleCancelRun}
                onRun={handleRun}
                onSolverModeChange={setSolverMode}
              />
              <RuleCheckPanel model={model} result={result} />
              <RuleCheckRunPanel
                model={model}
                result={result}
                onAggregateChange={handleRuleCheckAggregate}
                onR3JourneyEvent={recordR3JourneyEvent}
              />
              <KnowledgePanel knowledge={knowledge} result={result} />
            </section>

            <section
              className={dockSectionClass("results", activeSection)}
              aria-label="Results section"
              data-testid="workspace-section-results"
            >
              <ResultsPanel
                result={result}
                knowledge={knowledge}
                analysisRun={analysisRun}
                selectedResultId={selectedReviewTarget?.target_type === "result" ? selectedReviewTarget.id : null}
                onSelectResult={handleSelectResult}
              />
              <ComparisonPanel comparison={comparison} result={result} onSelectResult={handleSelectResult} />
              <DesignWorkspacePanel
                model={model}
                knowledge={knowledge}
                result={result}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
              />
            </section>

            <section
              className={dockSectionClass("report", activeSection)}
              aria-label="Report section"
              data-testid="workspace-section-report"
            >
              <RenderedReportPanel
                model={model}
                result={result}
                analysisRun={analysisRun}
                projectSummary={projectSummary}
                packagePrivateIntent={reportPackagePrivateIntent}
                packageBusy={reportPackageBusy}
                packageRedaction={reportPackageRedaction}
                packageRoute={reportPackageRoute}
                onPackagePrivateIntentChange={setReportPackagePrivateIntent}
                onSaveReportPackage={() => void handleSaveReportPackage()}
              />
              <ReportPanel
                model={model}
                knowledge={knowledge}
                result={result}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                projectOperation={projectOperation}
                projectSummary={projectSummary}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
                storageCapability={storageCapability}
              />
              <ReportLintPanel model={model} result={result} analysisRun={analysisRun} />
            </section>

            <section
              className={dockSectionClass("project", activeSection)}
              aria-label="Project section"
              data-testid="workspace-section-project"
            >
              <ProjectStorageAuditPanel
                model={model}
                storageCapability={storageCapability}
                projectSummary={projectSummary}
                projectIndex={projectIndex}
                projectMessage={projectMessage}
                projectOperation={projectOperation}
                editorIntents={editorIntents}
                proposal={proposal}
                modelHashIntegrity={modelHashIntegrity}
              />
              <ProjectValidationPanel
                model={model}
                storageCapability={storageCapability}
                projectSummary={projectSummary}
                projectOperation={projectOperation}
                editorIntents={editorIntents}
                proposal={proposal}
                modelHash={modelHash}
                modelHashIntegrity={modelHashIntegrity}
                projectEnvelopeHash={projectEnvelopeHash}
                projectEnvelopeHashIntegrity={projectEnvelopeHashIntegrity}
                modelDocumentMigration={modelDocumentMigration}
                modelMigrationLedger={modelMigrationLedger}
              />
            </section>

            <section
              className={dockSectionClass("exports", activeSection)}
              aria-label="Exports section"
              data-testid="workspace-section-exports"
            >
              <ResultExportPanel model={model} result={result} analysisRun={analysisRun} />
              <StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />
              <PcfExportPanel model={model} result={result} analysisRun={analysisRun} />
              <CaepipeMbfExportPanel model={model} result={result} analysisRun={analysisRun} />
              <CaepipeExternalHarnessPanel model={model} result={result} analysisRun={analysisRun} />
              <ExportAdapterSdkPanel model={model} result={result} analysisRun={analysisRun} />
              <AdapterFrameworkPanel model={model} result={result} analysisRun={analysisRun} />
              <LocalFeaHandoffPanel model={model} result={result} analysisRun={analysisRun} />
              <ExternalProverBoundaryPanel model={model} result={result} analysisRun={analysisRun} />
              <ReviewGeometryPanel model={model} result={result} analysisRun={analysisRun} />
              <HeadlessRunnerPanel model={model} result={result} analysisRun={analysisRun} solveJob={solveJob} />
              <NativePackagePanel
                model={model}
                modelHash={modelHash}
                result={result}
                analysisRun={analysisRun}
                editorIntents={editorIntents}
                projectSummary={projectSummary}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
                storageCapability={storageCapability}
              />
              <HandoffPanel
                model={model}
                knowledge={knowledge}
                result={result}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
              />
              <ExportReviewPanel
                model={model}
                knowledge={knowledge}
                result={result}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                projectOperation={projectOperation}
                projectSummary={projectSummary}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
                storageCapability={storageCapability}
              />
              <RedactionExportControlsPanel model={model} />
            </section>

            <section
              className={dockSectionClass("evidence", activeSection)}
              aria-label="Audit and boundaries section"
              data-testid="workspace-section-evidence"
            >
              <RunAuditPanel model={model} result={result} analysisRun={analysisRun} />
              <ValidationEvidencePanel model={model} />
              <BuildReadinessPanel model={model} />
              <TelemetryBoundaryPanel model={model} storageCapability={storageCapability} />
              <SecretPrivateLibraryPanel model={model} storageCapability={storageCapability} />
              <SecurityThreatModelPanel model={model} storageCapability={storageCapability} />
              <AccessibilityBaselinePanel model={model} />
            </section>
          </div>
        </section>
      </div>

      <StatusBar
        model={model}
        modelHash={modelHash}
        knowledge={knowledge}
        result={result}
        solveJob={solveJob}
        solveProof={solveProof}
        storageCapability={storageCapability}
        operationOutcomes={operationOutcomes}
        auditDrawerOpen={auditDrawerOpen}
        issuesDrawerOpen={issuesDrawerOpen}
        onOpenAudit={() => setAuditDrawerOpen((open) => !open)}
        onOpenIssues={() => setIssuesDrawerOpen((open) => !open)}
      />

      {auditDrawerOpen ? (
        <AuditBoundaryDrawer
          model={model}
          result={result}
          analysisRun={analysisRun}
          storageCapability={storageCapability}
          onClose={() => setAuditDrawerOpen(false)}
        />
      ) : null}

      {issuesDrawerOpen ? (
        <IssuesHome
          model={model}
          knowledge={knowledge}
          result={result}
          operationOutcomes={operationOutcomes}
          selectedDiagnosticId={selectedReviewTarget?.target_type === "diagnostic" ? selectedReviewTarget.id : null}
          onClose={() => setIssuesDrawerOpen(false)}
          onSelectDiagnostic={handleSelectDiagnostic}
        />
      ) : null}

      <footer className="app-footer">
        Technical preview — not a released product. Acceptance and professional judgment remain with the responsible engineer.
      </footer>
    </main>
  );
}

// Inactive sections stay mounted (form drafts, queue previews, and audit
// state survive navigation) and are hidden with CSS only; display:none also
// removes them from the accessibility tree in a real browser.
function dockSectionClass(sectionId: WorkspaceSectionId, activeSection: WorkspaceSectionId | null): string {
  return sectionId === activeSection ? "workspace-dock-section" : "workspace-dock-section inactive";
}

function MenuBar({
  activeSection,
  auditOpen,
  canRedo,
  canUndo,
  inspectorCollapsed,
  issuesOpen,
  openMenu,
  projectBusy,
  reportPackageReady,
  running,
  treeCollapsed,
  armedCreationTool,
  onCommand,
  onOpenMenu
}: {
  activeSection: WorkspaceSectionId | null;
  auditOpen: boolean;
  canRedo: boolean;
  canUndo: boolean;
  inspectorCollapsed: boolean;
  issuesOpen: boolean;
  openMenu: MenuId | null;
  projectBusy: boolean;
  reportPackageReady: boolean;
  running: boolean;
  treeCollapsed: boolean;
  armedCreationTool: CreationTool | null;
  onCommand: (command: MenuCommandId) => void;
  onOpenMenu: (menu: MenuId | null) => void;
}) {
  const menus: ReadonlyArray<{ id: MenuId; label: string; items: MenuItemSpec[] }> = [
    {
      id: "file",
      label: "File",
      items: [
        { kind: "command", id: "file.new-local", label: "New Local Project", disabled: projectBusy },
        { kind: "command", id: "file.new-blank", label: "New Blank Project", disabled: projectBusy },
        { kind: "separator" },
        { kind: "command", id: "file.open-local", label: "Open Local Project…", disabled: projectBusy },
        { kind: "command", id: "file.list-local", label: "List Local Projects", disabled: projectBusy },
        { kind: "separator" },
        { kind: "command", id: "file.save-local", label: "Save Local Project", disabled: projectBusy },
        { kind: "command", id: "file.save-report-package", label: "Save Report Package…", disabled: !reportPackageReady }
      ]
    },
    {
      id: "edit",
      label: "Edit",
      items: [
        { kind: "command", id: "edit.undo", label: "Undo Model Edit", disabled: !canUndo },
        { kind: "command", id: "edit.redo", label: "Redo Model Edit", disabled: !canRedo }
      ]
    },
    {
      id: "view",
      label: "View",
      items: [
        { kind: "command", id: "view.tree", label: "Model Tree", active: !treeCollapsed },
        { kind: "command", id: "view.inspector", label: "Inspector", active: !inspectorCollapsed },
        { kind: "separator" },
        ...WORKSPACE_SECTIONS.map(
          (section): MenuItemSpec => ({
            kind: "command",
            id: `view.section.${section.id}`,
            label: section.label,
            active: activeSection === section.id
          })
        ),
        { kind: "separator" },
        { kind: "command", id: "view.issues", label: "Issues", active: issuesOpen },
        { kind: "command", id: "view.audit", label: "Audit & Boundaries", active: auditOpen },
        { kind: "separator" },
        { kind: "command", id: "view.close-panels", label: "Close Panel (show viewport)", disabled: !activeSection }
      ]
    },
    {
      id: "insert",
      label: "Insert",
      items: [
        { kind: "command", id: "insert.node", label: "Node", active: armedCreationTool === "node" },
        { kind: "command", id: "insert.pipe", label: "Pipe Run", active: armedCreationTool === "pipe" },
        { kind: "command", id: "insert.support", label: "Support", active: armedCreationTool === "support" },
        { kind: "command", id: "insert.component", label: "Component", active: armedCreationTool === "component" },
        { kind: "separator" },
        { kind: "command", id: "insert.load", label: "Load Case", active: armedCreationTool === "load" }
      ]
    },
    {
      id: "analyze",
      label: "Analyze",
      items: [
        { kind: "command", id: "analyze.run", label: "Run Mechanics Preview", disabled: running },
        { kind: "command", id: "analyze.cancel", label: "Cancel Run", disabled: !running },
        { kind: "separator" },
        { kind: "command", id: "analyze.rule-checks", label: "Rule Checks" }
      ]
    }
  ];

  return (
    <>
      {openMenu ? (
        <div className="app-menu-backdrop" data-testid="app-menu-backdrop" onClick={() => onOpenMenu(null)} />
      ) : null}
      <nav className="app-menu-bar" data-testid="app-menu-bar" aria-label="Application menu">
        {menus.map((menu) => (
          <div className="app-menu" key={menu.id}>
            <button
              type="button"
              className={openMenu === menu.id ? "app-menu-trigger open" : "app-menu-trigger"}
              data-testid={`menu-${menu.id}`}
              aria-haspopup="menu"
              aria-expanded={openMenu === menu.id}
              onClick={() => onOpenMenu(openMenu === menu.id ? null : menu.id)}
            >
              {menu.label}
            </button>
            {openMenu === menu.id ? (
              <div className="app-menu-dropdown" role="menu" data-testid={`menu-dropdown-${menu.id}`}>
                {menu.items.map((item, index) =>
                  item.kind === "separator" ? (
                    <div key={`sep-${index}`} className="app-menu-separator" role="separator" />
                  ) : (
                    <button
                      key={item.id}
                      type="button"
                      role="menuitem"
                      className={item.active ? "app-menu-item active" : "app-menu-item"}
                      data-testid={`menu-item-${item.id}`}
                      aria-pressed={item.active ?? undefined}
                      disabled={item.disabled}
                      onClick={() => onCommand(item.id)}
                    >
                      {item.label}
                    </button>
                  )
                )}
              </div>
            ) : null}
          </div>
        ))}
      </nav>
    </>
  );
}

function AgentWorkbenchPanel({
  appliedOperationCount,
  mechanicsReady,
  model,
  proposal,
  queuedIntentCount,
  running,
  selection,
  selectedReviewTarget,
  statusText,
  onGenerateProposal,
  onOpenOperations,
  onOpenResults,
  onRunMechanics
}: {
  appliedOperationCount: number;
  mechanicsReady: boolean;
  model: PreviewModel;
  proposal: AgentProposal | null;
  queuedIntentCount: number;
  running: boolean;
  selection: EntityRef;
  selectedReviewTarget: SelectedReviewTarget | null;
  statusText: string;
  onGenerateProposal: () => void;
  onOpenOperations: () => void;
  onOpenResults: () => void;
  onRunMechanics: () => void;
}) {
  const selectedTarget = selectedReviewTarget ? `${selectedReviewTarget.target_type}: ${selectedReviewTarget.id}` : "model";
  return (
    <section className="panel agent-workbench-panel" aria-label="Design agent workbench" data-testid="agent-workbench-panel">
      <div className="agent-workbench-title">
        <span className="agent-workbench-icon" aria-hidden="true">
          <Bot size={18} />
        </span>
        <div>
          <div className="panel-title">Design Agent</div>
          <p data-testid="agent-workbench-status">{statusText}</p>
        </div>
      </div>

      <div className="agent-focus-grid" aria-label="Agent focus">
        <AgentFocusFact label="Selection" value={`${selection.type}: ${selection.id}`} testId="agent-focus-selection" />
        <AgentFocusFact label="Target" value={selectedTarget} testId="agent-focus-target" />
        <AgentFocusFact label="Queue" value={`${queuedIntentCount} queued / ${appliedOperationCount} applied`} testId="agent-focus-queue" />
        <AgentFocusFact
          label="Boundary"
          value={boundaryValue(model, "professional_boundary")}
          testId="agent-focus-boundary"
        />
      </div>

      <div className="agent-action-grid" aria-label="Agent actions">
        <button type="button" data-testid="agent-run-mechanics" onClick={onRunMechanics} disabled={running}>
          <Play size={15} aria-hidden="true" />
          Run
        </button>
        <button
          type="button"
          data-testid="agent-generate-proposal"
          onClick={onGenerateProposal}
          disabled={!mechanicsReady}
          title={mechanicsReady ? undefined : "Run mechanics before generating a review proposal."}
        >
          <Sparkles size={15} aria-hidden="true" />
          Propose
        </button>
        <button type="button" data-testid="agent-open-operations" onClick={onOpenOperations}>
          <ClipboardCheck size={15} aria-hidden="true" />
          Review
        </button>
        <button type="button" data-testid="agent-open-results" onClick={onOpenResults}>
          <Crosshair size={15} aria-hidden="true" />
          Inspect
        </button>
      </div>

      <div className="agent-proposal-summary" data-testid="agent-proposal-summary">
        {proposal ? (
          <>
            <strong>{proposal.proposal_id}</strong>
            <span>{proposal.validation.application_status ?? "not_applied"}</span>
            <small>{proposal.audit_boundary.requires_user_acceptance ? "requires_user_acceptance=true" : "requires_user_acceptance=false"}</small>
          </>
        ) : (
          <>
            <strong>No active proposal</strong>
            <span>review_only_local_preview</span>
            <small>accepted_model_state_mutated=false</small>
          </>
        )}
      </div>
    </section>
  );
}

function AgentFocusFact({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="agent-focus-fact" data-testid={testId} title={value}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function StatusBar({
  model,
  modelHash,
  knowledge,
  result,
  solveJob,
  solveProof,
  storageCapability,
  operationOutcomes,
  auditDrawerOpen,
  issuesDrawerOpen,
  onOpenAudit,
  onOpenIssues
}: {
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  solveJob: SolveJobAuditState;
  solveProof: SolveProofEvidence | null;
  storageCapability: LocalStorageCapability | null;
  operationOutcomes: Record<string, OperationOutcome>;
  auditDrawerOpen: boolean;
  issuesDrawerOpen: boolean;
  onOpenAudit: () => void;
  onOpenIssues: () => void;
}) {
  const status = result?.status ?? model.analysis_status;
  const issueCount = issueCountFor(model, knowledge, result, operationOutcomes);
  const visibleSolveProof = solveProofStatus(model, modelHash, result, solveJob, solveProof);
  return (
    <section className="status-bar" aria-label="Workspace status" data-testid="workspace-status-bar">
      <div className="status-pill-group" aria-label="Analysis statuses">
        <StatusPill label="Mechanics" value={status.mechanics} testId="status-pill-mechanics" />
        <StatusPill label="Rule check" value={ruleCheckStatusLabel(status.rule_check)} testId="status-pill-rule-check" />
        <StatusPill
          label="Professional"
          value={professionalStatusLabel(status.professional_acceptance)}
          testId="status-pill-professional"
        />
        {visibleSolveProof ? (
          <StatusPill
            label="Solve proof"
            value={visibleSolveProof}
            testId="status-pill-solve-proof"
          />
        ) : null}
      </div>
      <div className="status-bar-actions">
        <button
          type="button"
          className="status-drawer-button"
          data-testid="audit-drawer-toggle"
          aria-expanded={auditDrawerOpen}
          onClick={onOpenAudit}
        >
          Local · no network · no telemetry
          <small>{storageCapability ? storageBadgeLabel(storageCapability) : "checking storage"}</small>
        </button>
        <button
          type="button"
          className="status-drawer-button issue-button"
          data-testid="issues-drawer-toggle"
          aria-expanded={issuesDrawerOpen}
          onClick={onOpenIssues}
        >
          ⚑ {issueCount} Issues
        </button>
      </div>
    </section>
  );
}

function AuditBoundaryDrawer({
  model,
  result,
  analysisRun,
  storageCapability,
  onClose
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  storageCapability: LocalStorageCapability | null;
  onClose: () => void;
}) {
  return (
    <aside className="workspace-drawer audit-drawer" aria-label="Audit and boundaries drawer" data-testid="audit-boundary-drawer">
      <div className="drawer-header">
        <div>
          <span>Audit & boundaries</span>
          <h2>Local preview evidence</h2>
        </div>
        <button type="button" onClick={onClose}>
          Close
        </button>
      </div>
      <section className="boundary-grid" aria-label="Preview boundary details">
        <BoundaryItem icon={<Database size={15} aria-hidden="true" />} label="Public data" value={boundaryValue(model, "public_examples_policy")} />
        <BoundaryItem
          icon={<ShieldCheck size={15} aria-hidden="true" />}
          label="Protected content"
          value={boundaryValue(model, "protected_source_policy")}
        />
        <BoundaryItem icon={<LockKeyhole size={15} aria-hidden="true" />} label="Private data" value={boundaryValue(model, "private_data_policy")} />
        <BoundaryItem
          icon={<FileWarning size={15} aria-hidden="true" />}
          label="Professional boundary"
          value={boundaryValue(model, "professional_boundary")}
        />
      </section>
      <section className="storage-capability" data-testid="local-project-status">
        <strong>{storageCapability ? storageBadgeLabel(storageCapability) : "Storage check pending"}</strong>
        <span>
          {storageCapability
            ? `${storageCapability.engine} local store; network=${String(storageCapability.network_required)}; daemon=${String(
                storageCapability.daemon_required
              )}; telemetry=${String(storageCapability.telemetry_enabled)}; FTS5=${String(storageCapability.fts5_available)}.`
            : "Checking local storage."}
        </span>
      </section>
      <div className="drawer-panel-grid">
        <RunAuditPanel model={model} result={result} analysisRun={analysisRun} />
        <ValidationEvidencePanel model={model} />
        <BuildReadinessPanel model={model} />
        <TelemetryBoundaryPanel model={model} storageCapability={storageCapability} />
        <SecretPrivateLibraryPanel model={model} storageCapability={storageCapability} />
        <SecurityThreatModelPanel model={model} storageCapability={storageCapability} />
        <AccessibilityBaselinePanel model={model} />
      </div>
    </aside>
  );
}

function IssuesHome({
  model,
  knowledge,
  result,
  operationOutcomes,
  selectedDiagnosticId,
  onClose,
  onSelectDiagnostic
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  operationOutcomes: Record<string, OperationOutcome>;
  selectedDiagnosticId: string | null;
  onClose: () => void;
  onSelectDiagnostic: (diagnosticId: string) => void;
}) {
  const operationDiagnostics = Object.values(operationOutcomes).flatMap((outcome) => outcome.diagnostics);
  return (
    <aside className="workspace-drawer issues-drawer" aria-label="Issues home" data-testid="issues-home">
      <div className="drawer-header">
        <div>
          <span>Issues</span>
          <h2>Diagnostics and required inputs</h2>
        </div>
        <button type="button" onClick={onClose}>
          Close
        </button>
      </div>
      {operationDiagnostics.length > 0 ? (
        <section className="panel issues-operation-list" aria-label="Operation diagnostics">
          <div className="panel-title">Operation diagnostics</div>
          {operationDiagnostics.map((diagnostic) => (
            <article className={`issue-row ${diagnostic.severity}`} key={diagnostic.id}>
              <strong>{diagnostic.code}</strong>
              <p>{diagnostic.message}</p>
            </article>
          ))}
        </section>
      ) : null}
      <DiagnosticsPanel
        model={model}
        knowledge={knowledge}
        result={result}
        selectedDiagnosticId={selectedDiagnosticId}
        onSelectDiagnostic={onSelectDiagnostic}
      />
      <MissingDataBlockingPanel model={model} result={result} />
    </aside>
  );
}

function StatusPill({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <span className="status-pill" data-testid={testId} title={value}>
      <strong>{label}</strong>
      <code>{value}</code>
    </span>
  );
}

function professionalStatusLabel(value: string) {
  if (value.toLowerCase() === "not_provided") {
    return "HUMAN_REVIEW_REQUIRED";
  }
  return value;
}

function ruleCheckStatusLabel(value: string) {
  if (value.toLowerCase() === "not_performed_user_rule_inputs_missing") {
    return "RULE_INPUTS_INCOMPLETE";
  }
  return value;
}

export function solveProofStatus(
  model: PreviewModel,
  modelHash: ModelHashEvidence | null,
  result: MechanicsResult | null,
  solveJob: SolveJobAuditState,
  proof: SolveProofEvidence | null
): string | null {
  if (
    !modelHash ||
    !result ||
    !proof ||
    solveJob.state !== "completed" ||
    proof.state !== "completed" ||
    solveJob.job_id !== proof.job_id ||
    solveJob.backend_job_seam !== proof.backend_job_seam ||
    model.project.id !== proof.project_ref ||
    modelHash.value !== proof.model_sha256 ||
    result.run_id !== proof.result_run_id ||
    result.model_ref !== proof.result_model_ref ||
    result.results.length !== proof.result_row_count ||
    result.model_ref !== model.project.id
  ) {
    return null;
  }
  return [
    `seam=${proof.backend_job_seam}`,
    `project=${model.project.id}`,
    `result_model=${result.model_ref}`,
    "identity=match",
    `rows=${result.results.length}`,
    `generation=${proof.run_generation}`,
    `job=${proof.job_id}`,
    `model_sha256=${proof.model_sha256}`,
    `input_manifest_sha256=${proof.input_manifest_sha256}`
  ].join("; ");
}

function issueCountFor(
  model: PreviewModel,
  knowledge: DesignKnowledge | null,
  result: MechanicsResult | null,
  operationOutcomes: Record<string, OperationOutcome>
): number {
  const operationDiagnosticCount = Object.values(operationOutcomes).reduce(
    (count, outcome) => count + outcome.diagnostics.length,
    0
  );
  return (
    model.diagnostics.length +
    (knowledge?.diagnostics.length ?? 0) +
    (result?.diagnostics.length ?? 0) +
    operationDiagnosticCount +
    countMissingDataBlockers({ model, result })
  );
}

function operationIntentDisplayRef(intent: EditorOperationIntent): string {
  try {
    const afterPayload = JSON.parse(intent.change.after) as { id?: unknown };
    if (typeof afterPayload.id === "string" && afterPayload.id.trim().length > 0) {
      return afterPayload.id;
    }
  } catch {
    // Non-JSON after-values fall back to the target reference.
  }
  return intent.target.ref;
}

function isBlankAuthoringModel(model: PreviewModel): boolean {
  return model.data_boundary.public_examples_policy.includes("blank_user_created_local_document");
}

function countStatus(actual: number, required: number): string {
  return actual >= required ? "done" : `${actual}/${required}`;
}

function r3ExitJourneyStatus({
  result,
  ruleCheckAggregate,
  projectSummary
}: {
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
  projectSummary: LocalProjectSummary | null;
}): string {
  const solveStatus = result && ruleCheckAggregate && projectSummary ? "journey evidence started" : "journey evidence incomplete";
  return `Packaged A12/R3 human pass not recorded; ${solveStatus}; R3 exit review not started.`;
}

function BoundaryItem({ icon, label, value }: { icon: React.ReactNode; label: string; value: string }) {
  return (
    <span className="boundary-item">
      {icon}
      <strong>{label}</strong>
      {/* title carries the full value when the strip ellipsizes at narrow widths */}
      <code title={value}>{value}</code>
    </span>
  );
}

function boundaryValue(model: PreviewModel, key: string): string {
  return model.data_boundary[key] ?? "TBD";
}

function storageBadgeLabel(storageCapability: LocalStorageCapability): string {
  if (storageCapability.fts5_available) return "SQLite + FTS5";
  if (storageCapability.engine.toLowerCase().includes("browser")) return "Local preview store";
  return "Local store";
}

function projectReviewContext(
  editorIntents: EditorOperationIntent[],
  proposal: AgentProposal | null,
  appliedOperationCount: number
): string {
  const proposalCount = proposal ? 1 : 0;
  const total = editorIntents.length + proposalCount;
  const label = total === 1 ? "operation" : "operations";
  return ` Review context: ${total} pending ${label}; applied_operations=${appliedOperationCount}; editor_intents=${editorIntents.length}; agent_proposals=${proposalCount}.`;
}

function selectionForOperationOutcome(outcome: OperationOutcome): EntityRef | null {
  const selectionTypeByObjectType: Record<string, EntityRef["type"]> = {
    Material: "material",
    Section: "section",
    Node: "node",
    Element: "pipe",
    Component: "component",
    Support: "support",
    Load: "load",
    Combination: "combination"
  };
  const type = selectionTypeByObjectType[outcome.target_object_type];
  return type ? { type, id: outcome.target_ref } : null;
}

function clonePreviewModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

const NO_BACKEND_JOB_TOKEN = "none_no_active_backend_job";

function initialSolveJob(): SolveJobAuditState {
  return {
    job_id: "job:preview-linear-static:not-started",
    state: "not_started",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: "no_job_started",
    backend_job_id: null,
    backend_cancellation_token: NO_BACKEND_JOB_TOKEN,
    events: [
      {
        event_id: "solve-preview-not-started",
        state: "not_started",
        message: "Preview mechanics has not been requested in this session.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: null
  };
}

function startSolveJob(model: PreviewModel | null, startReceipt: SolveJobStartReceipt): SolveJobAuditState {
  const modelRef = model?.project.id ?? "project:unknown";
  const backendJob = startReceipt.mode === "backend_job";
  const runningMessage = backendJob
    ? `Backend solve job ${startReceipt.job_id} is executing; cancellation is cooperative at backend checkpoints (${startReceipt.cancellation_scope}); this service path does not stream percentage progress.`
    : "Preview mechanics command is executing in browser fixture mode without a backend job; this service path does not stream percentage progress.";
  return {
    job_id: backendJob ? startReceipt.job_id : `job:preview-linear-static:${safeJobToken(modelRef)}`,
    state: "running",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: backendJob ? "tauri_backend_job" : "browser_fixture_no_backend_job",
    backend_job_id: backendJob ? startReceipt.job_id : null,
    backend_cancellation_token: backendJob
      ? startReceipt.backend_cancellation_token
      : "unavailable_no_backend_job_browser_fixture_mode",
    events: [
      {
        event_id: "solve-preview-queued",
        state: "queued",
        message: backendJob
          ? `Backend solve job ${startReceipt.job_id} queued with a backend cancellation token through the application service boundary.`
          : "Preview mechanics command queued through the application service boundary.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      },
      {
        event_id: "solve-preview-running",
        state: "running",
        message: runningMessage,
        result_available: false,
        diagnostic_count: model?.diagnostics.length ?? 0,
        result_row_count: 0,
        analysis_status: model ? [model.analysis_status.mechanics, model.analysis_status.rule_check] : []
      }
    ],
    error_message: null
  };
}

function modelChangedSolveJob(outcome: OperationOutcome): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: "job:preview-linear-static:model-changed",
    events: [
      {
        event_id: "solve-preview-model-changed",
        state: "not_started",
        message: `Model changed by applied structured operation ${outcome.operation_id}; previous mechanics results were cleared because they no longer describe the edited model. Run a new solve.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function blankProjectCreatedSolveJob(model: PreviewModel): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:blank:${safeJobToken(model.project.id)}`,
    events: [
      {
        event_id: "solve-preview-blank-project-created",
        state: "not_started",
        message:
          "Blank local model document created as the authoring target; no mechanics results exist until explicit entities and loads are added and a solve is run.",
        result_available: false,
        diagnostic_count: model.diagnostics.length,
        result_row_count: 0,
        analysis_status: [model.analysis_status.mechanics, model.analysis_status.rule_check]
      }
    ]
  };
}

function sessionHistoryChangedSolveJob(action: "undo" | "redo" | "batch", operationId: string): SolveJobAuditState {
  const verb = action === "undo" ? "Undid" : action === "redo" ? "Redid" : "Applied batch";
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:session-${action}`,
    events: [
      {
        event_id: `solve-preview-session-${action}`,
        state: "not_started",
        message: `${verb} local session model operation ${operationId}; previous mechanics results were cleared because they no longer describe the current model. Run a new solve.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

async function awaitBackendSolveJob(jobId: string): Promise<BackendSolveJobStatus> {
  let status = await pollPreviewMechanicsJob(jobId);
  while (status.state === "queued" || status.state === "running") {
    await new Promise((resolve) => setTimeout(resolve, 50));
    status = await pollPreviewMechanicsJob(jobId);
  }
  return status;
}

function cancelledSolveJob(current: SolveJobAuditState, status: BackendSolveJobStatus): SolveJobAuditState {
  return {
    ...current,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: status.cancellation_status,
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancelled",
        state: "cancelled",
        message: `Backend solve job ${status.job_id} stopped at a cooperative checkpoint (${status.cancellation_status}); no result was published and no cancellation-success guarantee is claimed.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: status.error_message
  };
}

function recordBackendCancellationReceipt(
  current: SolveJobAuditState,
  receipt: BackendSolveJobCancellationReceipt
): SolveJobAuditState {
  return {
    ...current,
    cancellation_status: receipt.cancellation_status,
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-receipt",
        state: current.state,
        message: `Backend cancellation receipt for ${receipt.job_id}: accepted=${String(receipt.accepted)}; status=${receipt.cancellation_status}; job_state=${receipt.job_state}; no cancellation success is claimed.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function recordBackendCancellationFailure(current: SolveJobAuditState, error: unknown): SolveJobAuditState {
  return {
    ...current,
    cancellation_status: "backend_cancellation_request_failed",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-request-failed",
        state: current.state,
        message: `Backend cancellation request failed to reach the job registry: ${String(error)}. The solve job continues under its own state reporting.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function completeSolveJob(
  current: SolveJobAuditState,
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope
): SolveJobAuditState {
  const cancellationStatus = current.cancellation_requested
    ? "request_recorded_run_completed_before_cancellation_took_effect"
    : "not_requested";
  return {
    ...current,
    state: "completed",
    cancellation_status: cancellationStatus,
    events: [
      ...current.events,
      {
        event_id: "solve-preview-completed",
        state: "completed",
        message: `Preview mechanics completed with ${result.results.length} result rows bound to ${analysisRun.analysis_run.run_id}.`,
        result_available: true,
        diagnostic_count: result.diagnostics.length,
        result_row_count: result.results.length,
        analysis_status: analysisRun.analysis_run.analysis_status
      }
    ],
    error_message: null
  };
}

function restoredSolveJob(result: MechanicsResult, analysisRun: AnalysisRunEnvelope): SolveJobAuditState {
  return {
    job_id: `job:preview-linear-static:restored:${safeJobToken(analysisRun.analysis_run.run_id)}`,
    state: "completed",
    progress_basis: "restored_persisted_run_record_no_new_solve_executed",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: "restored_persisted_run_no_new_solve",
    backend_job_id: null,
    backend_cancellation_token: NO_BACKEND_JOB_TOKEN,
    events: [
      {
        event_id: "solve-preview-restored",
        state: "completed",
        message: `Restored persisted preview mechanics run ${analysisRun.analysis_run.run_id} from the local project store; no new solve was executed in this session.`,
        result_available: true,
        diagnostic_count: result.diagnostics.length,
        result_row_count: result.results.length,
        analysis_status: analysisRun.analysis_run.analysis_status
      }
    ],
    error_message: null
  };
}

function failSolveJob(current: SolveJobAuditState, error: unknown): SolveJobAuditState {
  return {
    ...current,
    state: "failed",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-failed",
        state: "failed",
        message: `Preview mechanics failed: ${String(error)}`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"]
      }
    ],
    error_message: String(error)
  };
}

function requestSolveCancellation(current: SolveJobAuditState): SolveJobAuditState {
  if (current.state !== "running") return current;
  const backendJob = current.backend_job_seam === "tauri_backend_job";
  return {
    ...current,
    state: "cancelling",
    cancellation_requested: true,
    cancellation_status: backendJob
      ? "request_sent_to_backend_job_awaiting_receipt"
      : "request_recorded_no_backend_job_in_browser_fixture_mode",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-requested",
        state: "cancelling",
        message: backendJob
          ? `Cancellation requested for backend solve job ${current.backend_job_id} using its backend cancellation token; cancellation is cooperative at backend checkpoints and success is not guaranteed.`
          : "Cancellation request recorded at the UI boundary; no backend job exists in browser fixture mode, so the in-flight fixture run cannot be interrupted.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function pendingBackendStartCancellationSolveJob(current: SolveJobAuditState): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: "job:preview-linear-static:pending-start-cancel",
    state: "cancelling",
    cancellation_requested: true,
    cancellation_status: "request_recorded_awaiting_backend_job_start",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-requested-before-backend-start",
        state: "cancelling",
        message:
          "Cancellation requested before backend job creation completed; the active solve generation will stop before start or dispatch exactly one cooperative cancellation when its backend receipt arrives.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function cancelledBeforeBackendStartSolveJob(model: PreviewModel): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:cancelled-before-start:${safeJobToken(model.project.id)}`,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: "cancelled_before_backend_job_start",
    events: [
      {
        event_id: "solve-preview-cancelled-before-backend-start",
        state: "cancelled",
        message:
          "The active solve generation was cancelled before any backend job was started; no result was computed or published.",
        result_available: false,
        diagnostic_count: model.diagnostics.length,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function cancelledWithoutBackendSolveJob(current: SolveJobAuditState): SolveJobAuditState {
  return {
    ...current,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: "cancelled_before_browser_fixture_result_publication",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-browser-fixture-cancelled",
        state: "cancelled",
        message:
          "The browser fixture solve generation was cancelled before result publication; no backend cancellation-success claim is made.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function safeJobToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

function deriveModelHashIntegrity(
  storedHash: ModelHashEvidence | null,
  recomputedHash: ModelHashEvidence | null,
  payloadRef: string
): ModelHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_model"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_model"
    };
  }
  return {
    integrity_status: storedHash.value === recomputedHash.value ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: "recomputed_on_open_from_restored_model"
  };
}

function deriveProjectEnvelopeHashIntegrity(
  storedHash: ProjectEnvelopeHashEvidence | null,
  recomputedHash: ProjectEnvelopeHashEvidence | null,
  payloadRef: string
): ProjectEnvelopeHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_envelope_payload"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_envelope_payload"
    };
  }
  return {
    integrity_status: storedHash.value === recomputedHash.value ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: "recomputed_on_open_from_restored_envelope_payload"
  };
}
