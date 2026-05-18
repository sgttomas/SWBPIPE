import { Activity, Database, FileWarning, FolderOpen, HardDrive, Save } from "lucide-react";
import type React from "react";
import { useEffect, useState } from "react";
import { AgentProposalPanel } from "./features/agent-proposals/AgentProposalPanel";
import { DiagnosticsPanel } from "./features/diagnostics/DiagnosticsPanel";
import { KnowledgePanel } from "./features/knowledge/KnowledgePanel";
import { defaultSelection } from "./features/model-workspace/modelView";
import { ModelTree } from "./features/model-tree/ModelTree";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportPanel } from "./features/report/ReportPanel";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { resolveDiagnosticEntitySelection, resolveEntitySelection } from "./features/results/resultInterpretation";
import { SolvePanel } from "./features/solve/SolvePanel";
import { PipeViewport } from "./features/viewport/PipeViewport";
import {
  buildAnalysisRunPreview,
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
  const [selectedReviewTarget, setSelectedReviewTarget] = useState<SelectedReviewTarget | null>(null);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  const [projectSummary, setProjectSummary] = useState<LocalProjectSummary | null>(null);
  const [projectMessage, setProjectMessage] = useState("Local project store not opened.");
  const [running, setRunning] = useState(false);
  const [projectBusy, setProjectBusy] = useState(false);

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

  async function handleCreateProject() {
    if (!model) return;
    setProjectBusy(true);
    try {
      const created = await createLocalProject(model);
      setProjectSummary(created.summary);
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
      const saved = await saveLocalProject(model);
      setProjectSummary(saved.summary);
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
            label={storageCapability?.fts5_available ? "SQLite + FTS5" : "SQLite check pending"}
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

      <section className="workspace-grid">
        <aside className="left-rail">
          <ModelTree model={model} selection={selection} onSelect={setSelection} />
          <PropertyInspector model={model} selection={selection} />
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
          <ReportPanel model={model} result={result} analysisRun={analysisRun} proposal={proposal} />
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
