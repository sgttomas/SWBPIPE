import {
  Activity,
  Database,
  FileWarning,
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
import { LocalFeaHandoffPanel } from "./features/local-fea-handoff/LocalFeaHandoffPanel";
import { MissingDataBlockingPanel } from "./features/missing-data/MissingDataBlockingPanel";
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
import { applyModelOperation, validateModelOperation } from "./services/operationService";
import {
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
  const [operationBusy, setOperationBusy] = useState(false);
  const [operationMessage, setOperationMessage] = useState<string | null>(null);
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

  async function handleRun() {
    setRunning(true);
    setAnalysisRun(null);
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
      setModel(outcome.applied_model);
      setEditorIntents((current) => current.filter((queued) => intentKey(queued) !== intentKey(intent)));
      // Earlier solve output no longer describes the edited model document;
      // keeping it visible would overstate what was computed.
      setResult(null);
      setAnalysisRun(null);
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
          <span data-testid="local-project-review-context"> {projectReviewContext(editorIntents, proposal, appliedOperations.length)}</span>
        </div>
        <div className="project-toolbar-actions">
          <button type="button" onClick={handleCreateProject} disabled={projectBusy}>
            <Database size={15} aria-hidden="true" />
            Create local
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
        {projectIndex && projectIndex.length > 0 ? (
          <div className="project-toolbar-actions" data-testid="project-index-picker" aria-label="Open listed project by id">
            {projectIndex.map((entry) => (
              <button
                key={entry.project_id}
                type="button"
                data-testid={`project-index-open-${entry.project_id}`}
                onClick={() => handleOpenProject(entry.project_id)}
                disabled={projectBusy}
              >
                <FolderOpen size={15} aria-hidden="true" />
                Open {entry.project_name} ({entry.project_id})
              </button>
            ))}
          </div>
        ) : null}
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
          <OperationApplyPanel
            queuedIntents={editorIntents}
            outcomes={operationOutcomes}
            appliedOperations={appliedOperations}
            busy={operationBusy}
            message={operationMessage}
            onValidate={handleValidateIntent}
            onApply={handleApplyIntent}
          />
        </aside>

        <section className="center-stage">
          <PipeViewport
            model={model}
            onQueueIntent={handleQueueEditorIntent}
            onSelect={setSelection}
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
            modelHash={modelHash}
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
