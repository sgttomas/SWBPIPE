import {
  Activity,
  Database,
  FileWarning,
  FilePlus,
  FolderOpen,
  HardDrive,
  List,
  LockKeyhole,
  Save,
  ShieldCheck
} from "lucide-react";
import type React from "react";
import { useEffect, useMemo, useRef, useState } from "react";
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
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportLintPanel } from "./features/report-lint/ReportLintPanel";
import { ReportPanel } from "./features/report/ReportPanel";
import { RenderedReportPanel } from "./features/report/RenderedReportPanel";
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
import { PipeViewport } from "./features/viewport/PipeViewport";
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

type RibbonStopId = "model" | "loads" | "analyze" | "results" | "rules" | "report";

type RibbonStop = {
  id: RibbonStopId;
  label: string;
  sections: WorkspaceSectionId[];
  primarySection: WorkspaceSectionId;
  journeyStepIds: JourneyStepId[];
};

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

const RIBBON_STOPS: ReadonlyArray<RibbonStop> = [
  {
    id: "model",
    label: "Model",
    primarySection: "operations",
    sections: ["operations", "libraries", "project"],
    journeyStepIds: ["model", "private-libraries", "save-reopen"]
  },
  {
    id: "loads",
    label: "Loads",
    primarySection: "loads",
    sections: ["loads"],
    journeyStepIds: ["loads"]
  },
  {
    id: "analyze",
    label: "Analyze",
    primarySection: "solve",
    sections: ["solve"],
    journeyStepIds: ["solve-check"]
  },
  {
    id: "results",
    label: "Results",
    primarySection: "results",
    sections: ["results"],
    journeyStepIds: ["results"]
  },
  {
    id: "rules",
    label: "Rules",
    primarySection: "rule-packs",
    sections: ["rule-packs", "libraries"],
    journeyStepIds: ["rule-pack", "private-libraries"]
  },
  {
    id: "report",
    label: "Report",
    primarySection: "report",
    sections: ["report", "exports", "project"],
    journeyStepIds: ["report", "save-reopen"]
  }
];

type JourneyStepId =
  | "model"
  | "loads"
  | "private-libraries"
  | "rule-pack"
  | "solve-check"
  | "results"
  | "report"
  | "save-reopen";

type JourneyStep = {
  id: JourneyStepId;
  label: string;
  section: WorkspaceSectionId;
  description: string;
};

type A12FlowStepId =
  | "blank"
  | "nodes"
  | "material"
  | "section"
  | "pipe"
  | "support"
  | "load-case"
  | "primitive-load"
  | "combination"
  | "solve"
  | "report"
  | "save-reopen";

type A12FlowStep = {
  id: A12FlowStepId;
  label: string;
  section: WorkspaceSectionId;
  cue: string;
  complete: boolean;
  status: string;
};

type GuidedJourneyMode = "a12" | "r3";

type R3FlowStepId = "library" | "rule-pack" | "checksum-save" | "solve" | "bind" | "run";

type R3FlowStep = {
  id: R3FlowStepId;
  label: string;
  section: WorkspaceSectionId;
  cue: string;
  complete: boolean;
  status: string;
};

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

const JOURNEY_STEPS: ReadonlyArray<JourneyStep> = [
  {
    id: "model",
    label: "Model edits",
    section: "operations",
    description: "Select geometry, queue structured edits, then review and apply them"
  },
  {
    id: "loads",
    label: "Loads",
    section: "loads",
    description: "Create or edit load cases, primitive loads, and combinations"
  },
  {
    id: "private-libraries",
    label: "Libraries",
    section: "libraries",
    description: "Review private local library import status before rule-pack references"
  },
  {
    id: "rule-pack",
    label: "Rule pack",
    section: "rule-packs",
    description: "Draft, validate, checksum, and store a private non-code rule pack"
  },
  {
    id: "solve-check",
    label: "Solve/check",
    section: "solve",
    description: "Run mechanics, review missing inputs, then run rule checks"
  },
  {
    id: "results",
    label: "Results",
    section: "results",
    description: "Inspect solved results, comparison context, and design-authoring state"
  },
  {
    id: "report",
    label: "Report",
    section: "report",
    description: "Render the calculation report and report packet after solve/check evidence exists"
  },
  {
    id: "save-reopen",
    label: "Save/reopen",
    section: "project",
    description: "Save the local project and verify local project persistence"
  }
];

export function App() {
  const [model, setModel] = useState<PreviewModel | null>(null);
  const [knowledge, setKnowledge] = useState<DesignKnowledge | null>(null);
  const [selection, setSelection] = useState<EntityRef | null>(null);
  const [result, setResult] = useState<MechanicsResult | null>(null);
  const [analysisRun, setAnalysisRun] = useState<AnalysisRunEnvelope | null>(null);
  // Worst-of rule-check aggregate from the GUI run panel, lifted so it can be
  // recorded in the app-held analysis-run envelope (TP-C4-APPAGG-001).
  const [ruleCheckAggregate, setRuleCheckAggregate] = useState<RuleCheckStatus | null>(null);
  const [proposal, setProposal] = useState<AgentProposal | null>(null);
  const [editorIntents, setEditorIntents] = useState<EditorOperationIntent[]>([]);
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
  const [running, setRunning] = useState(false);
  const [projectBusy, setProjectBusy] = useState(false);
  const [operationOutcomes, setOperationOutcomes] = useState<Record<string, OperationOutcome>>({});
  const [appliedOperations, setAppliedOperations] = useState<AppliedOperationReceipt[]>([]);
  const [undoStack, setUndoStack] = useState<SessionModelCheckpoint[]>([]);
  const [redoStack, setRedoStack] = useState<SessionModelCheckpoint[]>([]);
  const [operationBusy, setOperationBusy] = useState(false);
  const [operationMessage, setOperationMessage] = useState<string | null>(null);
  const [operationEngineStatus, setOperationEngineStatus] = useState<OperationEngineStatus>(() =>
    initialOperationEngineStatus()
  );
  const [activeSection, setActiveSection] = useState<WorkspaceSectionId>("operations");
  const [activeA12StepId, setActiveA12StepId] = useState<A12FlowStepId | null>(null);
  const [guidedJourneyMode, setGuidedJourneyMode] = useState<GuidedJourneyMode>("a12");
  const [activeR3StepId, setActiveR3StepId] = useState<R3FlowStepId | null>(null);
  const [r3JourneyState, setR3JourneyState] = useState<R3JourneyState>(() => ({
    ...INITIAL_R3_JOURNEY_STATE
  }));
  const [reviewDetailsOpen, setReviewDetailsOpen] = useState(false);
  const [auditDrawerOpen, setAuditDrawerOpen] = useState(false);
  const [issuesDrawerOpen, setIssuesDrawerOpen] = useState(false);
  const intentSequence = useRef(0);
  const comparison = useMemo(
    () => (result && analysisRun ? buildPreviewComparison({ result, analysisRun }) : null),
    [analysisRun, result]
  );

  useEffect(() => {
    let active = true;
    Promise.all([loadPreviewModel(), loadDesignKnowledge(), getLocalStorageCapability()]).then(
      ([loadedModel, loadedKnowledge, loadedStorageCapability]) => {
        if (!active) return;
        setModel(loadedModel);
        setKnowledge(loadedKnowledge);
        setSelection(defaultSelection(loadedModel));
        setStorageCapability(loadedStorageCapability);
      }
    );
    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    let active = true;
    setModelHash(null);
    if (!model) return;
    computeModelHash(model).then((hash) => {
      if (active) setModelHash(hash);
    });
    return () => {
      active = false;
    };
  }, [model]);

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

  async function handleRun() {
    setRunning(true);
    setAnalysisRun(null);
    // A fresh solve invalidates any prior rule-check run against the old result.
    setRuleCheckAggregate(null);
    try {
      const startReceipt = await startPreviewMechanicsJob(model);
      setSolveJob(startSolveJob(model, startReceipt));
      let output: MechanicsResult;
      if (startReceipt.mode === "backend_job") {
        const terminal = await awaitBackendSolveJob(startReceipt.job_id);
        if (terminal.state === "cancelled") {
          setSolveJob((current) => cancelledSolveJob(current, terminal));
          return;
        }
        if (terminal.state !== "completed" || !terminal.result) {
          throw new Error(
            terminal.error_message ?? `backend solve job ${terminal.job_id} ended as ${terminal.state} without a result`
          );
        }
        output = terminal.result;
      } else {
        output = await runPreviewMechanics(model);
      }
      const runRecord = await buildAnalysisRunPreview(output);
      setSolveJob((current) => completeSolveJob(current, output, runRecord));
      setResult(output);
      setSelectedReviewTarget(null);
      setProposal(null);
      setAnalysisRun(runRecord);
    } catch (error) {
      setSolveJob((current) => failSolveJob(current, error));
    } finally {
      setRunning(false);
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
    if (!result) return;
    try {
      setAnalysisRun(await buildAnalysisRunPreview(result, aggregate));
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
    const activeJob = solveJob;
    setSolveJob((current) => requestSolveCancellation(current));
    if (activeJob.backend_job_seam === "tauri_backend_job" && activeJob.backend_job_id) {
      void cancelPreviewMechanicsJob(activeJob.backend_job_id, activeJob.backend_cancellation_token)
        .then((receipt) => setSolveJob((current) => recordBackendCancellationReceipt(current, receipt)))
        .catch((error) => setSolveJob((current) => recordBackendCancellationFailure(current, error)));
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
    setEditorIntents([]);
    setProposal(null);
    setOperationOutcomes({});
    setOperationMessage(null);
  }

  async function handleValidateIntent(intent: EditorOperationIntent) {
    if (!model) return;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const outcome = await validateModelOperation(model, intent, modelHash);
      setOperationOutcomes((current) => ({ ...current, [intentKey(intent)]: outcome }));
    } catch (error) {
      setOperationMessage(`Operation validation failed to run: ${String(error)}`);
    } finally {
      setOperationBusy(false);
    }
  }

  async function handleApplyIntent(intent: EditorOperationIntent) {
    if (!model) return;
    setOperationBusy(true);
    setOperationMessage(null);
    try {
      const outcome = await applyModelOperation(model, intent, modelHash);
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
      setModel(outcome.applied_model);
      const appliedSelection = selectionForOperationOutcome(outcome);
      if (
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
      setRuleCheckAggregate(null);
      setProposal(null);
      setSelectedReviewTarget(null);
      setSolveJob(modelChangedSolveJob(outcome));
      setOperationMessage(
        `Applied ${outcome.operation_id} to the session model; previous solve results were cleared. Run a new solve, then save the project to store the edited model locally.`
      );
    } catch (error) {
      setOperationMessage(`Operation apply failed to run: ${String(error)}`);
    } finally {
      setOperationBusy(false);
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
    setModel(clonePreviewModel(checkpoint.model));
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
    setModel(clonePreviewModel(checkpoint.model));
    setSelection(checkpoint.selection);
    clearComputedModelState(sessionHistoryChangedSolveJob("redo", checkpoint.operation_id));
    setOperationMessage(
      `Redid ${checkpoint.operation_id} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`
    );
  }

  function clearComputedModelState(nextSolveJob: SolveJobAuditState) {
    setResult(null);
    setAnalysisRun(null);
    setRuleCheckAggregate(null);
    setProposal(null);
    setSelectedReviewTarget(null);
    setSolveJob(nextSolveJob);
  }

  async function handleCreateProject() {
    if (!model) return;
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
      const envelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: editorIntents,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: result,
        analysis_run: analysisRun,
        model_hash: modelHash
      });
      const created = await createLocalProject(
        model,
        editorIntents,
        proposal,
        selectedReviewTarget,
        result,
        analysisRun,
        modelHash,
        envelopeHash
      );
      setProjectSummary(created.summary);
      setEditorIntents(created.editor_intents ?? []);
      setProposal(created.proposal ?? null);
      setSelectedReviewTarget(created.selected_review_target ?? null);
      setProjectEnvelopeHash(created.project_envelope_hash ?? null);
      setModelDocumentMigration(created.model_document_migration ?? null);
      setModelMigrationLedger(created.model_migration_ledger ?? []);
      setProjectMessage(created.summary.message);
      setProjectOperation("create");
    } catch (error) {
      setProjectMessage(`Create failed: ${String(error)}`);
      setProjectOperation("create_failed");
    } finally {
      setProjectBusy(false);
    }
  }

  async function handleCreateBlankProject() {
    const blankModel = buildBlankLocalModelDocument();
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
      const createdSummary = {
        ...created.summary,
        message: "Created blank local model document without fixture entities or external file copies."
      };
      setModel(created.model);
      setSelection(defaultSelection(created.model));
      setUndoStack([]);
      setRedoStack([]);
      setAppliedOperations([]);
      setOperationOutcomes({});
      setOperationMessage(null);
      setResult(null);
      setAnalysisRun(null);
      setRuleCheckAggregate(null);
      setProposal(null);
      setEditorIntents(created.editor_intents ?? []);
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
      setProjectMessage(`Blank create failed: ${String(error)}`);
      setProjectOperation("create_blank_failed");
    } finally {
      setProjectBusy(false);
    }
  }

  async function handleOpenProject(projectId: string | null = null) {
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
      const opened = await openLocalProject(projectId);
      if (!opened) {
        setProjectMessage(
          projectId ? `No local project snapshot found for ${projectId}.` : "No local project snapshot found."
        );
        setProjectOperation("open_missing");
        return;
      }
      const restoredResult = opened.mechanics_result ?? null;
      const restoredAnalysisRun = opened.analysis_run ?? null;
      setModel(opened.model);
      setSelection(defaultSelection(opened.model));
      setUndoStack([]);
      setRedoStack([]);
      setAppliedOperations([]);
      setResult(restoredResult);
      setAnalysisRun(restoredAnalysisRun);
      setRuleCheckAggregate(null);
      setProposal(opened.proposal ?? null);
      setEditorIntents(opened.editor_intents ?? []);
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
      setProjectEnvelopeHashIntegrity(
        deriveProjectEnvelopeHashIntegrity(
          opened.project_envelope_hash ?? null,
          recomputedEnvelopeHash,
          opened.model.project.id
        )
      );
    } catch (error) {
      setProjectMessage(`Open failed: ${String(error)}`);
      setProjectOperation("open_failed");
    } finally {
      setProjectBusy(false);
    }
  }

  async function handleSaveProject() {
    if (!model) return;
    setProjectBusy(true);
    setModelHashIntegrity(null);
    setProjectEnvelopeHashIntegrity(null);
    try {
      const envelopeHash = await computeProjectEnvelopeHash({
        model,
        editor_intents: editorIntents,
        proposal,
        selected_review_target: selectedReviewTarget,
        mechanics_result: result,
        analysis_run: analysisRun,
        model_hash: modelHash
      });
      const saved = await saveLocalProject(
        model,
        editorIntents,
        proposal,
        selectedReviewTarget,
        result,
        analysisRun,
        modelHash,
        envelopeHash,
        modelDocumentMigration
      );
      setProjectSummary(saved.summary);
      setEditorIntents(saved.editor_intents ?? []);
      setProposal(saved.proposal ?? null);
      setSelectedReviewTarget(saved.selected_review_target ?? null);
      setProjectEnvelopeHash(saved.project_envelope_hash ?? null);
      setModelDocumentMigration(saved.model_document_migration ?? null);
      setModelMigrationLedger(saved.model_migration_ledger ?? []);
      setProjectMessage(saved.summary.message);
      setProjectOperation("save");
    } catch (error) {
      setProjectMessage(`Save failed: ${String(error)}`);
      setProjectOperation("save_failed");
    } finally {
      setProjectBusy(false);
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

  if (!model || !selection) {
    return <div className="loading-screen">Loading local OpenPipeStress preview fixture.</div>;
  }

  return (
    <main className="app-shell" data-testid="desktop-preview-shell">
      <header className="titlebar">
        <div>
          <h1>OpenPipeStress</h1>
          <p>{projectSummary?.project_name ?? model.project.name}</p>
        </div>
        <div className="titlebar-actions" aria-label="Local project controls">
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

      <Ribbon
        activeSection={activeSection}
        analysisRun={analysisRun}
        appliedOperationCount={appliedOperations.length}
        editorIntents={editorIntents}
        projectMessage={projectMessage}
        projectSummary={projectSummary}
        result={result}
        ruleCheckAggregate={ruleCheckAggregate}
        onSelectSection={setActiveSection}
      />

      <div className="workspace">
        <section className="modeling-workspace" aria-label="Modeling workspace" data-testid="modeling-workspace">
          <div className="workspace-pane workspace-pane-tree">
            <ModelTree model={model} selection={selection} onQueueIntent={handleQueueEditorIntent} onSelect={setSelection} />
          </div>
          <div className="workspace-pane workspace-pane-viewport">
            <PipeViewport
              model={model}
              onQueueIntent={handleQueueEditorIntent}
              onSelect={setSelection}
              queuedIntents={editorIntents}
              result={result}
              selection={selection}
            />
          </div>
          <div className="workspace-pane workspace-pane-inspector">
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

        <section className="workspace-dock" aria-label="Workspace sections">
          <div className="workspace-dock-body">
            <section
              className={dockSectionClass("operations", activeSection)}
              aria-label="Operation Apply section"
              data-testid="workspace-section-operations"
            >
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
                onSelect={setSelection}
                selection={selection}
              />
            </section>

            <section
              className={dockSectionClass("libraries", activeSection)}
              aria-label="Libraries section"
              data-testid="workspace-section-libraries"
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
                onCancel={handleCancelRun}
                onRun={handleRun}
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
              <ComparisonPanel comparison={comparison} onSelectResult={handleSelectResult} />
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
        knowledge={knowledge}
        result={result}
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
        Technical preview only: no production-readiness, release-readiness, certification, sealing, code-compliance; no licensed engineering reliance claim.
      </footer>
    </main>
  );
}

// Inactive sections stay mounted (form drafts, queue previews, and audit
// state survive navigation) and are hidden with CSS only; display:none also
// removes them from the accessibility tree in a real browser.
function dockSectionClass(sectionId: WorkspaceSectionId, activeSection: WorkspaceSectionId): string {
  return sectionId === activeSection ? "workspace-dock-section" : "workspace-dock-section inactive";
}

function Ribbon({
  activeSection,
  analysisRun,
  appliedOperationCount,
  editorIntents,
  projectMessage,
  projectSummary,
  result,
  ruleCheckAggregate,
  onSelectSection
}: {
  activeSection: WorkspaceSectionId;
  analysisRun: AnalysisRunEnvelope | null;
  appliedOperationCount: number;
  editorIntents: EditorOperationIntent[];
  projectMessage: string;
  projectSummary: LocalProjectSummary | null;
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
  onSelectSection: (section: WorkspaceSectionId) => void;
}) {
  const activeStop = ribbonStopForSection(activeSection);
  const current = currentJourneyStep({
    activeSection,
    analysisRun,
    appliedOperationCount,
    editorIntents,
    projectMessage,
    projectSummary,
    result,
    ruleCheckAggregate
  });

  return (
    <section className="workflow-ribbon" aria-label="Workflow ribbon" data-testid="workflow-ribbon">
      <nav className="ribbon-stop-list" aria-label="Primary workflow">
        {RIBBON_STOPS.map((stop) => (
          <button
            key={stop.id}
            type="button"
            className={activeStop.id === stop.id ? "ribbon-stop active" : "ribbon-stop"}
            data-testid={`ribbon-stop-${stop.id}`}
            aria-pressed={activeStop.id === stop.id}
            onClick={() => onSelectSection(stop.primarySection)}
          >
            <span>{stop.label}</span>
            <small>{ribbonStopBadge(stop, activeSection, editorIntents, result, analysisRun, projectSummary)}</small>
          </button>
        ))}
      </nav>
      <article className="ribbon-current-step" data-testid="ribbon-current-step">
        <span>Current step</span>
        <strong>{current.title}</strong>
        <p>{current.body}</p>
        <div className="ribbon-section-switcher" aria-label="Ribbon section switcher">
          {activeStop.sections.map((section) => (
            <button
              key={section}
              type="button"
              data-testid={`ribbon-section-${section}`}
              aria-pressed={activeSection === section}
              onClick={() => onSelectSection(section)}
            >
              {WORKSPACE_SECTIONS.find((candidate) => candidate.id === section)?.label ?? section}
            </button>
          ))}
        </div>
      </article>
    </section>
  );
}

function StatusBar({
  model,
  knowledge,
  result,
  storageCapability,
  operationOutcomes,
  auditDrawerOpen,
  issuesDrawerOpen,
  onOpenAudit,
  onOpenIssues
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  storageCapability: LocalStorageCapability | null;
  operationOutcomes: Record<string, OperationOutcome>;
  auditDrawerOpen: boolean;
  issuesDrawerOpen: boolean;
  onOpenAudit: () => void;
  onOpenIssues: () => void;
}) {
  const status = result?.status ?? model.analysis_status;
  const issueCount = issueCountFor(model, knowledge, result, operationOutcomes);
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

function ribbonStopForSection(section: WorkspaceSectionId): RibbonStop {
  return RIBBON_STOPS.find((stop) => stop.sections.includes(section)) ?? RIBBON_STOPS[0];
}

function ribbonStopBadge(
  stop: RibbonStop,
  activeSection: WorkspaceSectionId,
  editorIntents: EditorOperationIntent[],
  result: MechanicsResult | null,
  analysisRun: AnalysisRunEnvelope | null,
  projectSummary: LocalProjectSummary | null
): string {
  const statuses = stop.journeyStepIds.map((id) => {
    const step = JOURNEY_STEPS.find((candidate) => candidate.id === id);
    return step
      ? journeyStepStatus({ step, activeSection, editorIntents, result, analysisRun, projectSummary })
      : "pending";
  });
  if (statuses.some((status) => status.includes("current"))) return "current";
  if (statuses.every((status) => status.includes("ready") || status.includes("solved") || status.includes("saved"))) return "ready";
  if (statuses.some((status) => status.includes("queued"))) return "queued";
  return statuses[0] ?? "pending";
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

function GuidedWorkbench({
  activeSection,
  analysisRun,
  appliedOperationCount,
  editorIntents,
  model,
  projectMessage,
  projectOperation,
  projectSummary,
  result,
  ruleCheckAggregate,
  activeA12StepId,
  activeR3StepId,
  guidedJourneyMode,
  operationBusy,
  r3JourneyState,
  onApplyNextQueuedIntent,
  onSelectA12Step,
  onSelectGuidedJourneyMode,
  onSelectR3Step,
  onSelectSection
}: {
  activeSection: WorkspaceSectionId;
  analysisRun: AnalysisRunEnvelope | null;
  appliedOperationCount: number;
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
  projectMessage: string;
  projectOperation: string;
  projectSummary: LocalProjectSummary | null;
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
  activeA12StepId: A12FlowStepId | null;
  activeR3StepId: R3FlowStepId | null;
  guidedJourneyMode: GuidedJourneyMode;
  operationBusy: boolean;
  r3JourneyState: R3JourneyState;
  onApplyNextQueuedIntent: () => void;
  onSelectA12Step: (stepId: A12FlowStepId) => void;
  onSelectGuidedJourneyMode: (mode: GuidedJourneyMode) => void;
  onSelectR3Step: (stepId: R3FlowStepId) => void;
  onSelectSection: (section: WorkspaceSectionId) => void;
}) {
  const current = currentJourneyStep({
    activeSection,
    analysisRun,
    appliedOperationCount,
    editorIntents,
    projectMessage,
    projectSummary,
    result,
    ruleCheckAggregate
  });

  return (
    <section className="guided-workbench" aria-label="Guided workbench" data-testid="guided-workbench">
      <nav className="journey-rail" aria-label="Guided journey steps">
        {JOURNEY_STEPS.map((step) => (
          <button
            key={step.id}
            type="button"
            className={activeSection === step.section ? "journey-step active" : "journey-step"}
            aria-pressed={activeSection === step.section}
            data-testid={`journey-step-${step.id}`}
            title={step.description}
            onClick={() => onSelectSection(step.section)}
          >
            {journeyStepIcon(step.id)}
            <span>{step.label}</span>
            <small data-testid={`journey-step-status-${step.id}`}>
              {journeyStepStatus({ step, activeSection, editorIntents, result, analysisRun, projectSummary })}
            </small>
          </button>
        ))}
      </nav>
      <article className="current-step-panel" data-testid="journey-current-step">
        <div>
          <span className="current-step-kicker">Current step</span>
          <h2>{current.title}</h2>
          <p>{current.body}</p>
        </div>
        <div className="current-step-facts" data-testid="journey-current-step-status">
          <span title={current.status}>{current.status}</span>
          <span
            data-testid="r3-exit-journey-status"
            title={r3ExitJourneyStatus({ result, ruleCheckAggregate, projectSummary })}
          >
            {r3ExitJourneyStatus({ result, ruleCheckAggregate, projectSummary })}
          </span>
        </div>
      </article>
      <div className="guided-journey-stack" data-testid="guided-journey-stack">
        <div className="guided-journey-tabs" role="tablist" aria-label="Guided journey selector">
          <button
            type="button"
            className={guidedJourneyMode === "a12" ? "guided-journey-tab active" : "guided-journey-tab"}
            data-testid="guided-journey-tab-a12"
            aria-pressed={guidedJourneyMode === "a12"}
            onClick={() => onSelectGuidedJourneyMode("a12")}
          >
            A12 authoring
          </button>
          <button
            type="button"
            className={guidedJourneyMode === "r3" ? "guided-journey-tab active" : "guided-journey-tab"}
            data-testid="guided-journey-tab-r3"
            aria-pressed={guidedJourneyMode === "r3"}
            onClick={() => onSelectGuidedJourneyMode("r3")}
          >
            R3 rule checks
          </button>
        </div>
        {guidedJourneyMode === "a12" ? (
          <A12AuthoringJourney
            analysisRun={analysisRun}
            activeA12StepId={activeA12StepId}
            editorIntents={editorIntents}
            model={model}
            operationBusy={operationBusy}
            projectOperation={projectOperation}
            result={result}
            onApplyNextQueuedIntent={onApplyNextQueuedIntent}
            onSelectA12Step={onSelectA12Step}
            onSelectSection={onSelectSection}
          />
        ) : (
          <R3GuidedJourney
            activeR3StepId={activeR3StepId}
            r3JourneyState={r3JourneyState}
            result={result}
            ruleCheckAggregate={ruleCheckAggregate}
            onSelectR3Step={onSelectR3Step}
            onSelectSection={onSelectSection}
          />
        )}
      </div>
    </section>
  );
}

function A12AuthoringJourney({
  analysisRun,
  activeA12StepId,
  editorIntents,
  model,
  operationBusy,
  projectOperation,
  result,
  onApplyNextQueuedIntent,
  onSelectA12Step,
  onSelectSection
}: {
  analysisRun: AnalysisRunEnvelope | null;
  activeA12StepId: A12FlowStepId | null;
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
  operationBusy: boolean;
  projectOperation: string;
  result: MechanicsResult | null;
  onApplyNextQueuedIntent: () => void;
  onSelectA12Step: (stepId: A12FlowStepId) => void;
  onSelectSection: (section: WorkspaceSectionId) => void;
}) {
  const journey = buildA12FlowState({ analysisRun, editorIntents, model, projectOperation, result });
  const queued = editorIntents.length;
  const actionSection = queued > 0 ? "operations" : journey.nextStep?.section ?? "operations";
  return (
    <section className="a12-authoring-journey" aria-label="A12 authoring journey" data-testid="a12-authoring-journey">
      <div className="a12-journey-header">
        <div>
          <span>A12 authoring</span>
          <h2 data-testid="a12-next-action">
            {queued > 0
              ? "Review/apply queued edits"
              : journey.nextStep
                ? journey.nextStep.cue
                : "A12 path is ready for packaged verification"}
          </h2>
        </div>
        <strong data-testid="a12-journey-progress">
          {journey.completedCount}/{journey.steps.length}
        </strong>
      </div>
      <div className="a12-journey-grid" aria-label="A12 journey steps">
        {journey.steps.map((step) => (
          <button
            key={step.id}
            type="button"
            className={a12StepClass(step, journey.nextStep?.id, activeA12StepId)}
            data-testid={`a12-journey-step-${step.id}`}
            data-status={step.complete ? "complete" : step.id === journey.nextStep?.id ? "next" : "pending"}
            title={step.cue}
            aria-pressed={activeA12StepId === step.id}
            aria-current={step.id === journey.nextStep?.id ? "step" : undefined}
            onClick={() => {
              onSelectA12Step(step.id);
              onSelectSection(step.section);
            }}
          >
            <span>{step.label}</span>
            <small>{step.status}</small>
          </button>
        ))}
      </div>
      <div className="a12-journey-actions">
        <span data-testid="a12-queue-status">{journey.queueStatus}</span>
        <button
          type="button"
          data-testid={queued > 0 ? "a12-review-apply-button" : "a12-next-action-button"}
          disabled={queued > 0 && operationBusy}
          onClick={() => {
            if (queued > 0) {
              onApplyNextQueuedIntent();
              return;
            }
            onSelectSection(actionSection);
          }}
        >
          {queued > 0
            ? operationBusy
              ? "Applying"
              : "Apply queued"
            : journey.nextStep
              ? `Go to ${journey.nextStep.label}`
              : "Review saved path"}
        </button>
      </div>
    </section>
  );
}

function a12StepClass(
  step: A12FlowStep,
  nextStepId: A12FlowStepId | undefined,
  activeA12StepId: A12FlowStepId | null
): string {
  const classes = ["a12-journey-step"];
  if (step.complete) classes.push("complete");
  if (step.id === nextStepId) classes.push("next");
  if (step.id === activeA12StepId) classes.push("selected");
  return classes.join(" ");
}

function buildA12FlowState({
  analysisRun,
  editorIntents,
  model,
  projectOperation,
  result
}: {
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
  projectOperation: string;
  result: MechanicsResult | null;
}): {
  completedCount: number;
  nextStep: A12FlowStep | null;
  queueStatus: string;
  steps: A12FlowStep[];
} {
  const materials = model.materials ?? [];
  const sections = model.sections ?? [];
  const combinations = model.combinations ?? [];
  const primitiveLoadCount = model.load_cases.reduce(
    (count, loadCase) => count + (loadCase.primitive_loads?.length ?? 0),
    0
  );
  const solved = Boolean(result && result.results.length > 0 && result.status.mechanics === "MECHANICS_SOLVED");
  const reportReady = Boolean(solved && analysisRun);
  const reopened = projectOperation === "open" || projectOperation === "open_by_id";
  const saved = projectOperation === "save" || reopened;
  const steps: A12FlowStep[] = [
    {
      id: "blank",
      label: "Blank",
      section: "project",
      cue: "Start with New blank",
      complete: isBlankAuthoringModel(model),
      status: isBlankAuthoringModel(model) ? "ready" : "start"
    },
    {
      id: "nodes",
      label: "Nodes",
      section: "operations",
      cue: "Add two nodes in the viewport",
      complete: model.nodes.length >= 2,
      status: countStatus(model.nodes.length, 2)
    },
    {
      id: "material",
      label: "Material",
      section: "operations",
      cue: "Add a material in the property inspector",
      complete: materials.length >= 1,
      status: countStatus(materials.length, 1)
    },
    {
      id: "section",
      label: "Section",
      section: "operations",
      cue: "Add a pipe section in the property inspector",
      complete: sections.length >= 1,
      status: countStatus(sections.length, 1)
    },
    {
      id: "pipe",
      label: "Pipe",
      section: "operations",
      cue: "Connect a straight pipe in the viewport",
      complete: model.pipe_segments.length >= 1,
      status: countStatus(model.pipe_segments.length, 1)
    },
    {
      id: "support",
      label: "Support",
      section: "operations",
      cue: "Add a support in the property inspector",
      complete: model.supports.length >= 1,
      status: countStatus(model.supports.length, 1)
    },
    {
      id: "load-case",
      label: "Load case",
      section: "loads",
      cue: "Create a load case",
      complete: model.load_cases.length >= 1,
      status: countStatus(model.load_cases.length, 1)
    },
    {
      id: "primitive-load",
      label: "Load",
      section: "loads",
      cue: "Add a primitive load",
      complete: primitiveLoadCount >= 1,
      status: countStatus(primitiveLoadCount, 1)
    },
    {
      id: "combination",
      label: "Combination",
      section: "loads",
      cue: "Create a mechanics combination",
      complete: combinations.length >= 1,
      status: countStatus(combinations.length, 1)
    },
    {
      id: "solve",
      label: "Solve",
      section: "solve",
      cue: "Run mechanics preview",
      complete: solved,
      status: solved ? `${result?.results.length ?? 0} rows` : "not run"
    },
    {
      id: "report",
      label: "Report",
      section: "report",
      cue: "Review the report packet",
      complete: reportReady,
      status: reportReady ? "ready" : "after solve"
    },
    {
      id: "save-reopen",
      label: "Save/reopen",
      section: "project",
      cue: saved ? "Reopen the saved local project" : "Save the local project",
      complete: reopened,
      status: reopened ? "opened" : saved ? "saved" : "pending"
    }
  ];
  const nextStep = steps.find((step) => !step.complete) ?? null;
  const nextQueuedIntent = editorIntents[0];
  return {
    completedCount: steps.filter((step) => step.complete).length,
    nextStep,
    queueStatus:
      editorIntents.length > 0 && nextQueuedIntent
        ? `${editorIntents.length} queued operation${editorIntents.length === 1 ? "" : "s"}: ${
            operationIntentDisplayRef(nextQueuedIntent)
          }`
        : "No queued operations",
    steps
  };
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

function R3GuidedJourney({
  activeR3StepId,
  r3JourneyState,
  result,
  ruleCheckAggregate,
  onSelectR3Step,
  onSelectSection
}: {
  activeR3StepId: R3FlowStepId | null;
  r3JourneyState: R3JourneyState;
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
  onSelectR3Step: (stepId: R3FlowStepId) => void;
  onSelectSection: (section: WorkspaceSectionId) => void;
}) {
  const journey = buildR3FlowState({ r3JourneyState, result, ruleCheckAggregate });
  const actionSection = journey.nextStep?.section ?? "evidence";
  return (
    <section className="r3-guided-flow" aria-label="R3 rule-pack guided flow" data-testid="r3-guided-flow">
      <div className="r3-flow-header">
        <div>
          <span>R3 rule-pack flow</span>
          <h2 data-testid="r3-flow-next-action">
            {journey.nextStep ? journey.nextStep.cue : "Review R3 guided evidence before packaged pass"}
          </h2>
        </div>
        <strong data-testid="r3-flow-progress">
          {journey.completedCount}/{journey.steps.length}
        </strong>
      </div>
      <div className="r3-flow-grid" aria-label="R3 journey steps">
        {journey.steps.map((step) => (
          <button
            key={step.id}
            type="button"
            className={r3StepClass(step, journey.nextStep?.id, activeR3StepId)}
            data-testid={`r3-flow-step-${step.id}`}
            data-status={step.complete ? "complete" : step.id === journey.nextStep?.id ? "next" : "pending"}
            title={step.cue}
            aria-pressed={activeR3StepId === step.id}
            aria-current={step.id === journey.nextStep?.id ? "step" : undefined}
            onClick={() => {
              onSelectR3Step(step.id);
              onSelectSection(step.section);
            }}
          >
            <span>{step.label}</span>
            <small>{step.status}</small>
          </button>
        ))}
      </div>
      <div className="r3-flow-blocker" data-testid="r3-flow-missing-input-blocker">
        {r3MissingInputBlockerText({ r3JourneyState, ruleCheckAggregate })}
      </div>
      <div className="r3-flow-actions">
        <span data-testid="r3-flow-status">{journey.status}</span>
        <button
          type="button"
          data-testid="r3-flow-next-action-button"
          onClick={() => onSelectSection(actionSection)}
        >
          {journey.nextStep ? `Go to ${journey.nextStep.label}` : "Review evidence"}
        </button>
      </div>
    </section>
  );
}

function buildR3FlowState({
  r3JourneyState,
  result,
  ruleCheckAggregate
}: {
  r3JourneyState: R3JourneyState;
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
}): {
  completedCount: number;
  nextStep: R3FlowStep | null;
  status: string;
  steps: R3FlowStep[];
} {
  const solved = Boolean(result && result.results.length > 0 && result.status.mechanics === "MECHANICS_SOLVED");
  const libraryTouched =
    r3JourneyState.library_template_loaded &&
    (r3JourneyState.library_validate_requested || r3JourneyState.library_save_requested);
  const packValidated = r3JourneyState.rule_pack_draft_created && r3JourneyState.rule_pack_validate_requested;
  const packSaved = r3JourneyState.rule_pack_checksum_requested && r3JourneyState.rule_pack_save_requested;
  const runAttempted = Boolean(ruleCheckAggregate || r3JourneyState.rule_check_run_requested);
  const steps: R3FlowStep[] = [
    {
      id: "library",
      label: "Library",
      section: "libraries",
      cue: "Load and validate a private local library template",
      complete: libraryTouched,
      status: r3JourneyState.library_save_requested
        ? "save requested"
        : r3JourneyState.library_validate_requested
          ? "validate requested"
          : r3JourneyState.library_template_loaded
            ? "template loaded"
            : "not started"
    },
    {
      id: "rule-pack",
      label: "Rule pack",
      section: "rule-packs",
      cue: "Create and validate a private non-code rule-pack draft",
      complete: packValidated,
      status: r3JourneyState.rule_pack_validate_requested
        ? "validate requested"
        : r3JourneyState.rule_pack_draft_created
          ? "draft ready"
          : "not started"
    },
    {
      id: "checksum-save",
      label: "Checksum/save",
      section: "rule-packs",
      cue: "Compute the checksum and request local save",
      complete: packSaved,
      status: r3JourneyState.rule_pack_save_requested
        ? "save requested"
        : r3JourneyState.rule_pack_checksum_requested
          ? "checksum requested"
          : "pending"
    },
    {
      id: "solve",
      label: "Solve",
      section: "solve",
      cue: "Run mechanics before rule checks",
      complete: solved,
      status: solved ? `${result?.results.length ?? 0} rows` : "not run"
    },
    {
      id: "bind",
      label: "Bind",
      section: "solve",
      cue: "Load a rule pack for checking and review required bindings",
      complete: r3JourneyState.rule_check_pack_loaded,
      status: r3JourneyState.rule_check_pack_loaded ? "binding plan visible" : "load pack"
    },
    {
      id: "run",
      label: "Run checks",
      section: "solve",
      cue: "Run rule checks and confirm missing inputs block pass/fail",
      complete: runAttempted,
      status: ruleCheckAggregate ?? (r3JourneyState.rule_check_run_requested ? "run requested" : "not run")
    }
  ];
  const nextStep = steps.find((step) => !step.complete) ?? null;
  return {
    completedCount: steps.filter((step) => step.complete).length,
    nextStep,
    status: `private library=${steps[0].status}; rule_pack=${steps[1].status}; checksum_save=${
      steps[2].status
    }; rule_check=${
      ruleCheckAggregate ?? (runAttempted ? "requested" : "not run")
    }`,
    steps
  };
}

function r3StepClass(
  step: R3FlowStep,
  nextStepId: R3FlowStepId | undefined,
  activeR3StepId: R3FlowStepId | null
): string {
  const classes = ["r3-flow-step"];
  if (step.complete) classes.push("complete");
  if (step.id === nextStepId) classes.push("next");
  if (step.id === activeR3StepId) classes.push("selected");
  return classes.join(" ");
}

function r3MissingInputBlockerText({
  r3JourneyState,
  ruleCheckAggregate
}: {
  r3JourneyState: R3JourneyState;
  ruleCheckAggregate: RuleCheckStatus | null;
}): string {
  if (ruleCheckAggregate === "RULE_INPUTS_INCOMPLETE") {
    return "Missing inputs block pass/fail: aggregate=RULE_INPUTS_INCOMPLETE.";
  }
  if (!r3JourneyState.rule_check_pack_loaded) {
    return "Missing-input gate pending: load a rule pack, then bind required values before pass/fail.";
  }
  if (!r3JourneyState.rule_check_run_requested && !ruleCheckAggregate) {
    return "Binding review active: required missing values remain a blocker until supplied.";
  }
  if (!ruleCheckAggregate) {
    return "Run requested; browser preview keeps pass/fail blocked until the desktop checker returns complete inputs.";
  }
  return `Rule-check aggregate=${ruleCheckAggregate}; software finding only, no code-compliance claim.`;
}

function journeyStepIcon(stepId: JourneyStepId): React.ReactNode {
  switch (stepId) {
    case "model":
      return <List size={15} aria-hidden="true" />;
    case "loads":
      return <Activity size={15} aria-hidden="true" />;
    case "private-libraries":
      return <LockKeyhole size={15} aria-hidden="true" />;
    case "rule-pack":
      return <FileWarning size={15} aria-hidden="true" />;
    case "solve-check":
      return <ShieldCheck size={15} aria-hidden="true" />;
    case "results":
      return <Database size={15} aria-hidden="true" />;
    case "report":
      return <FilePlus size={15} aria-hidden="true" />;
    case "save-reopen":
      return <Save size={15} aria-hidden="true" />;
  }
}

function journeyStepStatus({
  step,
  activeSection,
  editorIntents,
  result,
  analysisRun,
  projectSummary
}: {
  step: JourneyStep;
  activeSection: WorkspaceSectionId;
  editorIntents: EditorOperationIntent[];
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  projectSummary: LocalProjectSummary | null;
}): string {
  if (activeSection === step.section) return "current";
  if (step.id === "model") return editorIntents.length > 0 ? `${editorIntents.length} queued` : "ready";
  if (step.id === "solve-check") return result ? "solved" : "not run";
  if (step.id === "results") return result ? `${result.results.length} rows` : "after solve";
  if (step.id === "report") return analysisRun ? "ready" : "after solve";
  if (step.id === "save-reopen") return projectSummary ? "local" : "not saved";
  return "available";
}

function currentJourneyStep({
  activeSection,
  analysisRun,
  appliedOperationCount,
  editorIntents,
  projectMessage,
  projectSummary,
  result,
  ruleCheckAggregate
}: {
  activeSection: WorkspaceSectionId;
  analysisRun: AnalysisRunEnvelope | null;
  appliedOperationCount: number;
  editorIntents: EditorOperationIntent[];
  projectMessage: string;
  projectSummary: LocalProjectSummary | null;
  result: MechanicsResult | null;
  ruleCheckAggregate: RuleCheckStatus | null;
}): { title: string; body: string; status: string } {
  switch (activeSection) {
    case "operations":
      return editorIntents.length > 0
        ? {
            title: "Review and apply queued model edits",
            body: "Validate queued operations, apply accepted edits to the local session model, then save when ready.",
            status: `${editorIntents.length} queued; ${appliedOperationCount} applied this session`
          }
        : {
            title: "Queue a model edit",
            body: "Select in the tree or viewport, queue an inspector or viewport edit, then review/apply here.",
            status: "No queued operations"
          };
    case "loads":
      return {
        title: "Author load cases and combinations",
        body: "Use the load manager to create or edit loads before solving. Proposed edits still pass through Operation Apply before they mutate the local session model.",
        status: `${editorIntents.length} queued operation${editorIntents.length === 1 ? "" : "s"}`
      };
    case "libraries":
      return {
        title: "Prepare private local libraries",
        body: "Import or inspect private library records locally. Public fixtures remain invented and private payloads stay out of the repository.",
        status: "Private-library route available"
      };
    case "rule-packs":
      return {
        title: "Draft the private non-code rule pack",
        body: "Create, validate, checksum, and save rule-pack drafts through the structured composer. Writable expression text remains out of scope under DEC-037.",
        status: "AST composer only; no text parser"
      };
    case "solve":
      return {
        title: "Run solve and rule checks",
        body: "Run mechanics first, then review missing rule-check inputs and check results. Missing inputs block pass/fail instead of being guessed.",
        status: result
          ? `Solved ${result.results.length} rows; rule_check=${ruleCheckAggregate ?? "not run"}`
          : "Solve not run in this session"
      };
    case "results":
      return {
        title: "Review results and model-state context",
        body: "Inspect result rows, diagnostics, deformed shape status, comparison context, and selected-review targets before reporting.",
        status: result ? `${result.results.length} result rows available` : "Results populate after solve"
      };
    case "report":
      return {
        title: "Render the calculation report",
        body: "Generate the hash-bound report surface after solving. Reports disclose assumptions, warnings, provenance, and boundary status without professional approval claims.",
        status: analysisRun ? `analysis_run=${analysisRun.analysis_run.run_id}` : "Report evidence waits for solve"
      };
    case "project":
      return {
        title: "Save and reopen the local project",
        body: "Persist the current local session model and verify the local store. User-created models remain local project data and are not committed to the repository.",
        status: projectSummary ? projectMessage : "No local project summary yet"
      };
    case "exports":
      return {
        title: "Use export and handoff details deliberately",
        body: "Export surfaces are available for review evidence and local handoff packages after the main journey state is ready.",
        status: "Detail route; no external solver invocation"
      };
    case "evidence":
      return {
        title: "Audit boundaries and evidence",
        body: "Inspect validation evidence, build readiness, telemetry/privacy, private-library, security, and accessibility boundary panels.",
        status: "Audit details only; no release or professional claim"
      };
  }
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

function Badge({ icon, label }: { icon: React.ReactNode; label: string }) {
  return (
    <span className="badge">
      {icon}
      {label}
    </span>
  );
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

function sessionHistoryChangedSolveJob(action: "undo" | "redo", operationId: string): SolveJobAuditState {
  const verb = action === "undo" ? "Undid" : "Redid";
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
