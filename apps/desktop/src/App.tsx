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
import { AgentProposalPanel } from "./features/agent-proposals/AgentProposalPanel";
import { ComparisonPanel } from "./features/comparison/ComparisonPanel";
import { DiagnosticsPanel } from "./features/diagnostics/DiagnosticsPanel";
import { ExportReviewPanel } from "./features/export-review/ExportReviewPanel";
import { HandoffPanel } from "./features/handoff/HandoffPanel";
import { KnowledgePanel } from "./features/knowledge/KnowledgePanel";
import { defaultSelection } from "./features/model-workspace/modelView";
import { ModelTree } from "./features/model-tree/ModelTree";
import { OperationLedgerPanel } from "./features/operations/OperationLedgerPanel";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportPanel } from "./features/report/ReportPanel";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { resolveDiagnosticEntitySelection, resolveEntitySelection } from "./features/results/resultInterpretation";
import { RunAuditPanel } from "./features/run-audit/RunAuditPanel";
import { SolvePanel } from "./features/solve/SolvePanel";
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
  SelectedReviewTarget
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
    try {
      const output = await runPreviewMechanics(model);
      setResult(output);
      setSelectedReviewTarget(null);
      setProposal(null);
      const runRecord = await buildAnalysisRunPreview(output);
      setAnalysisRun(runRecord);
    } finally {
      setRunning(false);
    }
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

  async function handleCreateProject() {
    if (!model) return;
    setProjectBusy(true);
    try {
      const created = await createLocalProject(model, editorIntents);
      setProjectSummary(created.summary);
      setEditorIntents(created.editor_intents ?? []);
      setProjectMessage(created.summary.message);
    } catch (error) {
      setProjectMessage(`Create failed: ${String(error)}`);
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
        return;
      }
      setModel(opened.model);
      setSelection(defaultSelection(opened.model));
      setResult(null);
      setAnalysisRun(null);
      setProposal(null);
      setEditorIntents(opened.editor_intents ?? []);
      setSelectedReviewTarget(null);
      setProjectSummary(opened.summary);
      setProjectMessage(opened.summary.message);
    } catch (error) {
      setProjectMessage(`Open failed: ${String(error)}`);
    } finally {
      setProjectBusy(false);
    }
  }

  async function handleSaveProject() {
    if (!model) return;
    setProjectBusy(true);
    try {
      const saved = await saveLocalProject(model, editorIntents);
      setProjectSummary(saved.summary);
      setEditorIntents(saved.editor_intents ?? []);
      setProjectMessage(saved.summary.message);
    } catch (error) {
      setProjectMessage(`Save failed: ${String(error)}`);
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
          <span data-testid="local-project-review-context"> {projectReviewContext(editorIntents)}</span>
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
        </aside>

        <section className="center-stage">
          <PipeViewport model={model} selection={selection} />
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
          <SolvePanel model={model} result={result} running={running} onRun={handleRun} />
          <RunAuditPanel model={model} result={result} analysisRun={analysisRun} />
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
          <OperationLedgerPanel
            model={model}
            analysisRun={analysisRun}
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
            proposal={proposal}
            selectedReviewTarget={selectedReviewTarget}
          />
          <ReportPanel
            model={model}
            knowledge={knowledge}
            result={result}
            analysisRun={analysisRun}
            comparison={comparison}
            editorIntents={editorIntents}
            proposal={proposal}
            selectedReviewTarget={selectedReviewTarget}
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

function projectReviewContext(editorIntents: EditorOperationIntent[]): string {
  const label = editorIntents.length === 1 ? "operation" : "operations";
  return ` Review context: ${editorIntents.length} pending ${label}; applied=false.`;
}
