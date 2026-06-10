import {
  Activity,
  Database,
  FileWarning,
  FolderOpen,
  HardDrive,
  LockKeyhole,
  Save,
  ShieldCheck
} from "lucide-react";
import type React from "react";
import { useEffect, useMemo, useState } from "react";
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
import { LocalFeaHandoffPanel } from "./features/local-fea-handoff/LocalFeaHandoffPanel";
import { MissingDataBlockingPanel } from "./features/missing-data/MissingDataBlockingPanel";
import { defaultSelection } from "./features/model-workspace/modelView";
import { ModelTree } from "./features/model-tree/ModelTree";
import { NativePackagePanel } from "./features/native-package/NativePackagePanel";
import { OperationLedgerPanel } from "./features/operations/OperationLedgerPanel";
import { PcfExportPanel } from "./features/pcf-export/PcfExportPanel";
import { ProjectStorageAuditPanel } from "./features/project-storage/ProjectStorageAuditPanel";
import { ProjectValidationPanel } from "./features/project-validation/ProjectValidationPanel";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportLintPanel } from "./features/report-lint/ReportLintPanel";
import { ReportPanel } from "./features/report/ReportPanel";
import { ResultExportPanel } from "./features/result-export/ResultExportPanel";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { resolveDiagnosticEntitySelection, resolveEntitySelection } from "./features/results/resultInterpretation";
import { ReviewGeometryPanel } from "./features/review-geometry/ReviewGeometryPanel";
import { RuleCheckPanel } from "./features/rule-check/RuleCheckPanel";
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
  loadDesignKnowledge,
  loadPreviewModel,
  loadSampleProposal,
  runPreviewMechanics
} from "./services/previewService";
import {
  createLocalProject,
  getLocalStorageCapability,
  openLocalProject,
  saveLocalProject
} from "./services/projectService";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  EditorOperationIntent,
  EntityRef,
  LocalProjectSummary,
  LocalStorageCapability,
  MechanicsResult,
  PreviewModel,
  SelectedReviewTarget,
  SolveJobAuditState
} from "./types";

export function App() {
  const [model, setModel] = useState<PreviewModel | null>(null);
  const [knowledge, setKnowledge] = useState<DesignKnowledge | null>(null);
  const [selection, setSelection] = useState<EntityRef | null>(null);
  const [result, setResult] = useState<MechanicsResult | null>(null);
  const [analysisRun, setAnalysisRun] = useState<AnalysisRunEnvelope | null>(null);
  const [proposal, setProposal] = useState<AgentProposal | null>(null);
  const [editorIntents, setEditorIntents] = useState<EditorOperationIntent[]>([]);
  const [selectedReviewTarget, setSelectedReviewTarget] = useState<SelectedReviewTarget | null>(null);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  const [projectSummary, setProjectSummary] = useState<LocalProjectSummary | null>(null);
  const [projectMessage, setProjectMessage] = useState("Local project store not opened.");
  const [projectOperation, setProjectOperation] = useState("not_started");
  const [solveJob, setSolveJob] = useState<SolveJobAuditState>(() => initialSolveJob());
  const [running, setRunning] = useState(false);
  const [projectBusy, setProjectBusy] = useState(false);
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

  async function handleRun() {
    setRunning(true);
    setAnalysisRun(null);
    setSolveJob(startSolveJob(model));
    try {
      const output = await runPreviewMechanics(model);
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

  function handleCancelRun() {
    setSolveJob((current) => requestSolveCancellation(current));
  }

  async function handleProposal() {
    setProposal(await loadSampleProposal(result, selectedReviewTarget));
  }

  function handleQueueEditorIntent(intent: EditorOperationIntent) {
    setEditorIntents((current) => [
      {
        ...intent,
        queue_id: `editor-intent-${current.length + 1}`
      },
      ...current
    ]);
  }

  function handleClearReviewQueue() {
    setEditorIntents([]);
    setProposal(null);
  }

  async function handleCreateProject() {
    if (!model) return;
    setProjectBusy(true);
    try {
      const created = await createLocalProject(model, editorIntents, proposal, selectedReviewTarget, result, analysisRun);
      setProjectSummary(created.summary);
      setEditorIntents(created.editor_intents ?? []);
      setProposal(created.proposal ?? null);
      setSelectedReviewTarget(created.selected_review_target ?? null);
      setProjectMessage(created.summary.message);
      setProjectOperation("create");
    } catch (error) {
      setProjectMessage(`Create failed: ${String(error)}`);
      setProjectOperation("create_failed");
    } finally {
      setProjectBusy(false);
    }
  }

  async function handleOpenProject() {
    setProjectBusy(true);
    try {
      const opened = await openLocalProject();
      if (!opened) {
        setProjectMessage("No local project snapshot found.");
        setProjectOperation("open_missing");
        return;
      }
      const restoredResult = opened.mechanics_result ?? null;
      const restoredAnalysisRun = opened.analysis_run ?? null;
      setModel(opened.model);
      setSelection(defaultSelection(opened.model));
      setResult(restoredResult);
      setAnalysisRun(restoredAnalysisRun);
      setProposal(opened.proposal ?? null);
      setEditorIntents(opened.editor_intents ?? []);
      setSelectedReviewTarget(opened.selected_review_target ?? null);
      setSolveJob(
        restoredResult && restoredAnalysisRun
          ? restoredSolveJob(restoredResult, restoredAnalysisRun)
          : initialSolveJob()
      );
      setProjectSummary(opened.summary);
      setProjectMessage(opened.summary.message);
      setProjectOperation("open");
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
    try {
      const saved = await saveLocalProject(model, editorIntents, proposal, selectedReviewTarget, result, analysisRun);
      setProjectSummary(saved.summary);
      setEditorIntents(saved.editor_intents ?? []);
      setProposal(saved.proposal ?? null);
      setSelectedReviewTarget(saved.selected_review_target ?? null);
      setProjectMessage(saved.summary.message);
      setProjectOperation("save");
    } catch (error) {
      setProjectMessage(`Save failed: ${String(error)}`);
      setProjectOperation("save_failed");
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
      <header className="topbar">
        <div>
          <h1>OpenPipeStress Technical Preview</h1>
          <p>{model.project.description}</p>
        </div>
        <div className="topbar-actions" aria-label="Preview status">
          <Badge icon={<Database size={16} />} label="Invented data" />
          <Badge icon={<Activity size={16} />} label="Local execution" />
          <Badge
            icon={<HardDrive size={16} />}
            label={storageCapability ? storageBadgeLabel(storageCapability) : "Storage check pending"}
          />
          <Badge icon={<FileWarning size={16} />} label="Human review required" />
        </div>
      </header>

      <section className="project-toolbar" aria-label="Local project controls">
        <div>
          <strong>{projectSummary?.project_name ?? model.project.name}</strong>
          <span data-testid="local-project-status">
            {storageCapability
              ? ` ${storageCapability.engine} local store; network=${String(storageCapability.network_required)}; daemon=${String(
                  storageCapability.daemon_required
                )}; telemetry=${String(storageCapability.telemetry_enabled)}; FTS5=${String(
                  storageCapability.fts5_available
                )}.`
              : " Checking local storage."}
          </span>
          <span data-testid="local-project-message"> {projectMessage}</span>
          <span data-testid="local-project-review-context"> {projectReviewContext(editorIntents, proposal)}</span>
        </div>
        <div className="project-toolbar-actions">
          <button type="button" onClick={handleCreateProject} disabled={projectBusy}>
            <Database size={15} aria-hidden="true" />
            Create local
          </button>
          <button type="button" onClick={handleOpenProject} disabled={projectBusy}>
            <FolderOpen size={15} aria-hidden="true" />
            Open local
          </button>
          <button type="button" onClick={handleSaveProject} disabled={projectBusy}>
            <Save size={15} aria-hidden="true" />
            Save local
          </button>
        </div>
      </section>

      <section className="boundary-strip" aria-label="Preview boundary" data-testid="preview-boundary-strip">
        <BoundaryItem
          icon={<Database size={15} aria-hidden="true" />}
          label="Public data"
          value={boundaryValue(model, "public_examples_policy")}
        />
        <BoundaryItem
          icon={<ShieldCheck size={15} aria-hidden="true" />}
          label="Protected content"
          value={boundaryValue(model, "protected_source_policy")}
        />
        <BoundaryItem
          icon={<LockKeyhole size={15} aria-hidden="true" />}
          label="Private data"
          value={boundaryValue(model, "private_data_policy")}
        />
        <BoundaryItem
          icon={<FileWarning size={15} aria-hidden="true" />}
          label="Professional boundary"
          value={boundaryValue(model, "professional_boundary")}
        />
      </section>

      <section className="workspace-grid">
        <aside className="left-rail">
          <ModelTree model={model} selection={selection} onSelect={setSelection} />
          <PropertyInspector
            model={model}
            onQueueIntent={handleQueueEditorIntent}
            queuedIntents={editorIntents}
            selection={selection}
          />
          <EditorContractPanel editorIntents={editorIntents} model={model} />
        </aside>

        <section className="center-stage">
          <PipeViewport
            model={model}
            onQueueIntent={handleQueueEditorIntent}
            queuedIntents={editorIntents}
            selection={selection}
          />
          <div className="bottom-panels">
            <KnowledgePanel knowledge={knowledge} result={result} />
            <DiagnosticsPanel
              model={model}
              knowledge={knowledge}
              result={result}
              selectedDiagnosticId={selectedReviewTarget?.target_type === "diagnostic" ? selectedReviewTarget.id : null}
              onSelectDiagnostic={handleSelectDiagnostic}
            />
          </div>
        </section>

        <aside className="right-rail">
          <ProjectStorageAuditPanel
            model={model}
            storageCapability={storageCapability}
            projectSummary={projectSummary}
            projectMessage={projectMessage}
            projectOperation={projectOperation}
            editorIntents={editorIntents}
            proposal={proposal}
          />
          <ProjectValidationPanel
            model={model}
            storageCapability={storageCapability}
            projectSummary={projectSummary}
            projectOperation={projectOperation}
            editorIntents={editorIntents}
            proposal={proposal}
          />
          <TelemetryBoundaryPanel model={model} storageCapability={storageCapability} />
          <SecretPrivateLibraryPanel model={model} storageCapability={storageCapability} />
          <SecurityThreatModelPanel model={model} storageCapability={storageCapability} />
          <BuildReadinessPanel model={model} />
          <ValidationEvidencePanel model={model} />
          <SolvePanel
            analysisRun={analysisRun}
            model={model}
            result={result}
            running={running}
            solveJob={solveJob}
            onCancel={handleCancelRun}
            onRun={handleRun}
          />
          <MissingDataBlockingPanel model={model} result={result} />
          <AccessibilityBaselinePanel model={model} />
          <RuleCheckPanel model={model} result={result} />
          <RunAuditPanel model={model} result={result} analysisRun={analysisRun} />
          <ResultExportPanel model={model} result={result} analysisRun={analysisRun} />
          <StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />
          <HeadlessRunnerPanel model={model} result={result} analysisRun={analysisRun} solveJob={solveJob} />
          <AdapterFrameworkPanel model={model} result={result} analysisRun={analysisRun} />
          <LocalFeaHandoffPanel model={model} result={result} analysisRun={analysisRun} />
          <ExternalProverBoundaryPanel model={model} result={result} analysisRun={analysisRun} />
          <ReviewGeometryPanel model={model} result={result} analysisRun={analysisRun} />
          <PcfExportPanel model={model} result={result} analysisRun={analysisRun} />
          <CaepipeMbfExportPanel model={model} result={result} analysisRun={analysisRun} />
          <CaepipeExternalHarnessPanel model={model} result={result} analysisRun={analysisRun} />
          <ExportAdapterSdkPanel model={model} result={result} analysisRun={analysisRun} />
          <ReportLintPanel model={model} result={result} analysisRun={analysisRun} />
          <NativePackagePanel
            model={model}
            result={result}
            analysisRun={analysisRun}
            editorIntents={editorIntents}
            projectSummary={projectSummary}
            proposal={proposal}
            selectedReviewTarget={selectedReviewTarget}
            storageCapability={storageCapability}
          />
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
          <ComparisonPanel comparison={comparison} onSelectResult={handleSelectResult} />
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
          <ResultsPanel
            result={result}
            knowledge={knowledge}
            analysisRun={analysisRun}
            selectedResultId={selectedReviewTarget?.target_type === "result" ? selectedReviewTarget.id : null}
            onSelectResult={handleSelectResult}
          />
          <AgentProposalPanel
            proposal={proposal}
            mechanicsReady={Boolean(result)}
            selectedReviewTarget={selectedReviewTarget}
            onLoad={handleProposal}
          />
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
        </aside>
      </section>

      <footer className="app-footer">
        Technical preview only: no production-readiness, release-readiness, certification, sealing, code-compliance; no licensed engineering reliance claim.
      </footer>
    </main>
  );
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
      <code>{value}</code>
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

function projectReviewContext(editorIntents: EditorOperationIntent[], proposal: AgentProposal | null): string {
  const proposalCount = proposal ? 1 : 0;
  const total = editorIntents.length + proposalCount;
  const label = total === 1 ? "operation" : "operations";
  return ` Review context: ${total} pending ${label}; applied=false; editor_intents=${editorIntents.length}; agent_proposals=${proposalCount}.`;
}

function initialSolveJob(): SolveJobAuditState {
  return {
    job_id: "job:preview-linear-static:not-started",
    state: "not_started",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_cancellation_token: "TBD",
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

function startSolveJob(model: PreviewModel | null): SolveJobAuditState {
  const modelRef = model?.project.id ?? "project:unknown";
  return {
    job_id: `job:preview-linear-static:${safeJobToken(modelRef)}`,
    state: "running",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_cancellation_token: "TBD",
    events: [
      {
        event_id: "solve-preview-queued",
        state: "queued",
        message: "Preview mechanics command queued through the application service boundary.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      },
      {
        event_id: "solve-preview-running",
        state: "running",
        message: "Preview mechanics command is executing; this service path does not stream percentage progress.",
        result_available: false,
        diagnostic_count: model?.diagnostics.length ?? 0,
        result_row_count: 0,
        analysis_status: model ? [model.analysis_status.mechanics, model.analysis_status.rule_check] : []
      }
    ],
    error_message: null
  };
}

function completeSolveJob(
  current: SolveJobAuditState,
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope
): SolveJobAuditState {
  const cancellationStatus = current.cancellation_requested
    ? "request_recorded_run_completed_before_backend_cancelled"
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
    backend_cancellation_token: "TBD",
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
  return {
    ...current,
    state: "cancelling",
    cancellation_requested: true,
    cancellation_status: "request_recorded_backend_token_tbd",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-requested",
        state: "cancelling",
        message: "Cancellation request recorded at the UI boundary; backend cancellation token remains TBD.",
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
