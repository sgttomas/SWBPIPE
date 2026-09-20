import { HangerSelectionPanel } from "./features/hanger-selection";
import { SelfWeightPlanPanel } from "./features/self-weight-authoring";
import { OfflineProposalIntakePanel } from "./features/offline-proposal-intake";
import operationSchema from "../../../schemas/model_operation.schema.json";
import { GeometryToolsPanel } from "./features/geometry-tools/GeometryToolsPanel";
import { BoundaryAuthoringPanel } from "./features/boundary-authoring/BoundaryAuthoringPanel";
import { DisplayUnitsProvider } from "./features/display-units";
import { BatchReviewPanel } from "./features/toolkit/BatchReviewPanel";
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
  Sparkles,
  X
} from "lucide-react";
import { AgentStrip } from "./features/workspace/shell/AgentStrip";
import { ShellStatusBar } from "./features/workspace/shell/ShellStatusBar";
import { ShellToolbar } from "./features/workspace/shell/ShellToolbar";
import { StageRail } from "./features/workspace/shell/StageRail";
import { StageTabStrip } from "./features/workspace/shell/StageTabStrip";
import {
  SHELL_STAGE_LABELS,
  canvasAuthoringPanelActive,
  inspectorToggleState,
  railStageStates,
  runPresenceFromCells,
  shellLocation,
  tableDrawerState,
  viewForStage,
  viewSwitchItems
} from "./features/workspace/shellLayout";
import type { ShellStage, ShellView, TableDrawerState } from "./features/workspace/shellLayout";
import { DormantSection } from "./features/workspace/dormantSection";
import { BOTH_SPLIT_PCT_BOUNDS, TABLE_DRAWER_PX_BOUNDS } from "./features/workspace/uiPreferences";
import type { UiDensityPreference, UiThemePreference } from "./features/workspace/uiPreferences";
import type React from "react";
import { Fragment, useState } from "react";
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
import { professionalStatusToken, ruleCheckStatusToken, statusDisplay } from "./features/workspace/statusLabels";
import { setSelectionFocus } from "./features/workspace/selectionState";
import { solveProofStatus, type SolveProofEvidence } from "./features/workspace/solveProof";
import {
  WORKSPACE_SECTIONS,
  type WorkspaceSectionId
} from "./features/workspace/workspaceSections";
import {
  type MenuCommandId,
  type MenuId,
  type MenuItemSpec
} from "./features/workspace/menuCommands";
import { useWorkspaceSession } from "./features/workspace/workspaceSession";
import { WorkspaceSessionProvider } from "./features/workspace/WorkspaceSessionContext";
import { ModelTree } from "./features/model-tree/ModelTree";
import { NativePackagePanel } from "./features/native-package/NativePackagePanel";
import { OperationApplyPanel } from "./features/operations/OperationApplyPanel";
import { OperationLedgerPanel } from "./features/operations/OperationLedgerPanel";
import { PcfExportPanel } from "./features/pcf-export/PcfExportPanel";
import { ProjectStorageAuditPanel } from "./features/project-storage/ProjectStorageAuditPanel";
import { ProjectValidationPanel } from "./features/project-validation/ProjectValidationPanel";
import { ToolkitPalette, type PaletteShellCommand } from "./features/toolkit/ToolkitPalette";
import { toolkitCapabilities } from "./features/toolkit/capabilityCatalog";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ReportLintPanel } from "./features/report-lint/ReportLintPanel";
import { ReportPanel } from "./features/report/ReportPanel";
import { RenderedReportPanel } from "./features/report/RenderedReportPanel";
import { ResultExportPanel } from "./features/result-export/ResultExportPanel";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { ReviewGeometryPanel } from "./features/review-geometry/ReviewGeometryPanel";
import { RuleCheckPanel } from "./features/rule-check/RuleCheckPanel";
import { RuleCheckRunPanel } from "./features/rule-check/RuleCheckRunPanel";
import { RulePackManagerPanel } from "./features/rule-packs/RulePackManagerPanel";
import { RunAuditPanel } from "./features/run-audit/RunAuditPanel";
import { SecretPrivateLibraryPanel } from "./features/secret-private-library/SecretPrivateLibraryPanel";
import { SecurityThreatModelPanel } from "./features/security-threat-model/SecurityThreatModelPanel";
import { HistoricalRunPanel } from "./features/results/HistoricalRunContext";
import { SolvePanel } from "./features/solve/SolvePanel";
import { StressNeutralExportPanel } from "./features/stress-neutral/StressNeutralExportPanel";
import { TelemetryBoundaryPanel } from "./features/telemetry/TelemetryBoundaryPanel";
import { ValidationEvidencePanel } from "./features/validation-evidence/ValidationEvidencePanel";
import {
  PipeViewport,
  type CreationTool
} from "./features/viewport/PipeViewport";
import type { RuleCheckStatus } from "./services/ruleCheckService";
import { isTauriRuntime } from "./services/nativeMenu";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  EditorOperationIntent,
  EntityRef,
  LocalProjectSummary,
  LocalStorageCapability,
  MechanicsResult,
  ModelHashEvidence,
  OperationOutcome,
  PreviewModel,
  SelectedReviewTarget,
  SolveJobAuditState
} from "./types";

// These moved to features/workspace. App.tsx keeps exporting them: tests import
// them from "./App".
export {
  SolveRunGenerationGate,
  RuleRevisionGenerationGate,
  commitModelAfterSolveInvalidation
} from "./features/workspace/solveGates";
export { isSupportedChangedModelPersistenceResponse } from "./features/workspace/projectPersistenceIntegrity";
export { solveProofStatus } from "./features/workspace/solveProof";
export type { SolveProofEvidence } from "./features/workspace/solveProof";

export function App() {
  return <DisplayUnitsProvider><AppSession /></DisplayUnitsProvider>;
}

function AppSession() {
  const session = useWorkspaceSession();
  const [routingPanelContainer, setRoutingPanelContainer] = useState<HTMLDivElement | null>(null);
  const {
    model,
    knowledge,
    modelHash,
    modelAssignment,
    activeModelIndex,
    projectSessionGeneration,
    uiModelRevision,
    modelRevision
  } = session.model;
  const {
    selection,
    orderedSelection,
    orderedSelectionRef,
    hiddenEntityKeys, setHiddenEntityKeys,
    isolateHiddenEntityKeys, setIsolateHiddenEntityKeys,
    effectiveHiddenKeys,
    selectedPipeRefs,
    treePublication,
    handleTreePublication,
    commitSelectionState,
    handleSelectEntity,
    handleBoxSelection
  } = session.selection;
  const {
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
    handleCancelRun,
    handleRuleCheckAggregate,
    handleProposal,
    handleSelectResult,
    handleSelectDiagnostic,
    handleSaveReportPackage
  } = session.results;
  const {
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
  } = session.operations;
  const {
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
  } = session.project;
  const {
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
    stageViewMemory,
    narrowWindow,
    closeShellPage,
    recordR3JourneyEvent,
    handleArmCreationTool,
    handleToolkitCommand,
    runMenuCommand,
    handleWorkspaceSplitterKeyDown,
    beginWorkspaceResize,
    handleNarrowDrawerKeyDown,
    exposeViewportForNarrowInteraction,
    toggleWorkspaceRail
  } = session.chrome;

  // In the packaged Tauri shell the OS-level menu bar is the single menu, so the
  // in-DOM menu bar is suppressed to avoid a redundant second row. In the
  // browser/Playwright preview there is no native menu, so the in-DOM bar
  // renders and remains the navigable, tested source of truth.
  const showInAppMenuBar = !isTauriRuntime();

  if (!model || !selection) {
    return <div className="loading-screen">Loading local SWBPIPE preview fixture.</div>;
  }

  // Where the shell is, derived from the one navigation cell (shellLayout.ts).
  const shell = shellLocation(activeSection, stageSurface);
  const stageView = viewForStage(stageViewMemory, shell.stage);
  const tableDrawer = tableDrawerState(stageView, narrowWindow, treeCollapsed);
  const issueCount = issueCountFor(model, knowledge, result, operationOutcomes);
  const pendingOperationCount = editorIntents.length + queuedBatches.reduce((count, entry) => count + (Array.isArray(entry.batch?.operations) ? entry.batch.operations.length : 1), 0);
  // The solve proof left the status bar (specification §5.5: no hashes, no proofs there).
  // It is read on the Results stage's Evidence tab, in the same words as before.
  const visibleSolveProof = solveProofStatus(model, modelHash, result, solveJob, solveProof);

  return (
    <WorkspaceSessionProvider session={session}>
    <main
      className={showInAppMenuBar ? "app-shell" : "app-shell native-menu"}
      data-density={uiPreferences.density}
      data-theme={resolvedTheme}
      data-theme-preference={uiPreferences.theme}
      data-testid="desktop-preview-shell"
      ref={workspaceShellRef}
    >
      {showInAppMenuBar ? (
        <MenuBar
          activeSection={activeSection}
          auditOpen={auditDrawerOpen}
          canRedo={redoStack.length > 0 && !operationBusy}
          canUndo={undoStack.length > 0 && !operationBusy}
          inspectorCollapsed={inspectorCollapsed}
          issuesOpen={issuesDrawerOpen}
          openMenu={openMenu}
          projectBusy={projectBusy}
          reportPackageReady={Boolean(currentSolvedResult && analysisRun && inputManifest) && !running && !reportPackageBusy}
          running={running}
          treeCollapsed={treeCollapsed}
          armedCreationTool={armedCreationTool}
          stage={shell.stage}
          stageViewMemory={stageViewMemory}
          pageOpen={shell.page !== null}
          tableDrawer={tableDrawer}
          run={runPresenceFromCells({ result, historicalRun, solveJob })}
          theme={uiPreferences.theme}
          density={uiPreferences.density}
          onCommand={runMenuCommand}
          onOpenMenu={setOpenMenu}
        />
      ) : null}

      <ShellToolbar issueCount={issueCount}>
          <ToolkitPalette
            context={{ selection, selectionCardinality: orderedSelection.orderedKeys.length, canUndo: undoStack.length > 0, canRedo: redoStack.length > 0, busy: operationRequest.current.busy || operationBusy, windConfigured: Boolean(selection.type === "load" && model.load_cases.find((load) => load.id === selection.id)?.equivalent_static?.wind) }}
            onChoose={handleToolkitCommand}
            shellCommands={shellPaletteCommands({
              stage: shell.stage,
              pageOpen: shell.page !== null,
              stageViewMemory,
              run: runPresenceFromCells({ result, historicalRun, solveJob }),
              theme: uiPreferences.theme,
              density: uiPreferences.density,
              onCommand: runMenuCommand
            })}
          />
      </ShellToolbar>

      <div className="shell-body">
        <StageRail issueCount={issueCount} />
        <div ref={workspaceBudgetRef} className="workspace shell-surfaces-frame">
        <section
          className={`modeling-workspace shell-surfaces${treeCollapsed ? " tree-collapsed" : ""}${
            inspectorCollapsed ? " inspector-collapsed" : ""
          }`}
          aria-label="Modeling workspace"
          inert={Boolean(shell.page)}
          data-testid="modeling-workspace"
          data-stage={shell.stage}
          data-view={stageView}
          data-narrow={narrowWindow ? "true" : undefined}
          data-canvas-authoring={canvasAuthoringPanelActive(armedCreationTool, editorIntents) ? "true" : undefined}
          style={{
            "--shell-both-split": `${uiPreferences.bothSplitPct}%`,
            "--shell-drawer-height": `${uiPreferences.tableDrawerPx}px`
          } as React.CSSProperties}
        >
          <div className="workspace-pane workspace-pane-tree shell-table-pane" onKeyDown={(event) => handleNarrowDrawerKeyDown(event, "tree")}>
            <StageTabStrip
              stage={shell.stage}
              tab={shell.tab}
              pendingCount={pendingOperationCount}
              drawer={tableDrawer}
              toggleRef={treeToggleRef}
              onTab={(section) => { setActiveSection(section); if (section === "operations") setOperationTab("review"); }}
              onToggle={() => toggleWorkspaceRail("tree")}
            />
            <div className="shell-table-body" id="shell-table-body">
            <div className={shell.tab === "model-tree" ? "shell-tree-host" : "shell-tree-host inactive"} data-testid="shell-tree-host">
            <ModelTree
              density={uiPreferences.density}
              hiddenKeys={effectiveHiddenKeys}
              model={model}
              modelIndex={activeModelIndex ?? undefined}
              projectSessionGeneration={projectSessionGeneration}
              selection={selection}
              selectionState={orderedSelection}
              onQueueIntent={handleQueueEditorIntent}
              onSelect={handleSelectEntity}
              onFocusChange={(key) => commitSelectionState(setSelectionFocus(orderedSelectionRef.current, key))}
              onFilterPublication={handleTreePublication}
            />
            </div>
          <div className={shell.tab === "model-tree" ? "workspace-dock-body shell-stage-sections inactive" : "workspace-dock-body shell-stage-sections"}>
            <section
              className={dockSectionClass("operations", activeSection)}
              aria-label="Operation Apply section"
              data-testid="workspace-section-operations"
              tabIndex={-1}
            >
              {activeSection === "operations" || activatedExpensiveSections.has("operations") ? (
              <DormantSection active={activeSection === "operations"} guardGeneration={requestEpoch} sessionGeneration={projectSessionGeneration}>
              <nav className="operation-tabs" aria-label="Editing and review tools">
                {[["review", "Review changes"], ["geometry", "Geometry"], ["supports", "Supports"], ["weight", "Self weight"], ["agent", "Agent"], ["details", "Details"]].map(([id, label]) => (
                  <button type="button" key={id} aria-pressed={operationTab === id} data-testid={`operation-tab-${id}`} onClick={() => setOperationTab(id)}>{label}</button>
                ))}
              </nav>
              <div className="operation-tool-page" hidden={operationTab !== "details"}>
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
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "supports"}>
              <HangerSelectionPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "weight"}>
              <SelfWeightPlanPanel
                model={model}
                selection={selection}
                selectedPipeRefs={selectedPipeRefs}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "agent"}>
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
              reviewActive={activeSection === "operations" && operationTab === "review"}
              onGenerateProposal={handleProposal}
              onOpenOperations={() => { setActiveSection("operations"); setOperationTab("review"); }}
              onOpenResults={() => setActiveSection("results")}
              onRunMechanics={handleRun}
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
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "geometry"}>
              <GeometryToolsPanel
                model={model}
                selection={selection}
                selectedPipeRefs={selectedPipeRefs}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "supports"}>
              <BoundaryAuthoringPanel
                model={model}
                selection={selection}
                onQueueBatch={handleQueueOperationBatch}
                busy={operationBusy}
                requestEpoch={requestEpoch}
              />
              </div>
              <div className="operation-tool-page review-tool-page" hidden={operationTab !== "review"}>
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
              </div>
              <div className="operation-tool-page" hidden={operationTab !== "details"}>
              <section
                className={reviewDetailsOpen ? "review-apply-drawer open" : "review-apply-drawer"}
                aria-label="Review and apply detail views"
                data-testid="review-apply-drawer"
              >
                <div className="review-apply-drawer-header">
                  <div>
                    <span>Detail views</span>
                    <h2>Review evidence</h2>
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
              </div>
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("loads", activeSection)}
              aria-label="Load Cases section"
              data-testid="workspace-section-loads"
            >
              {activeSection === "loads" || activatedExpensiveSections.has("loads") ? (
              <DormantSection active={activeSection === "loads"} guardGeneration={requestEpoch} sessionGeneration={projectSessionGeneration}>
              <LoadCaseManagerPanel
                model={model}
                onQueueIntent={handleQueueEditorIntent}
                onSelect={handleSelectEntity}
                selection={selection}
              />
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("results", activeSection)}
              aria-label="Results section"
              data-testid="workspace-section-results"
            >
              {activeSection === "results" || activatedExpensiveSections.has("results") ? (
              <DormantSection active={activeSection === "results"} sessionGeneration={projectSessionGeneration}>
              <Fragment key={`results:${dormantOutputBasis}`}>
              {historicalRun ? <HistoricalRunPanel key={historicalRun.runId} context={historicalRun} /> : <ResultsPanel
                result={result}
                knowledge={knowledge}
                analysisRun={analysisRun}
                selectedResultId={selectedReviewTarget?.target_type === "result" ? selectedReviewTarget.id : null}
                onSelectResult={handleSelectResult}
              />}
              <ComparisonPanel comparison={comparison} result={currentSolvedResult} onSelectResult={handleSelectResult} />
              <DesignWorkspacePanel
                model={model}
                knowledge={knowledge}
                result={currentSolvedResult}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
              />
              </Fragment>
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("evidence", activeSection)}
              aria-label="Audit and boundaries section"
              data-testid="workspace-section-evidence"
            >
              {visibleSolveProof ? (
                <StatusPill
                  label="Solve proof"
                  value={visibleSolveProof}
                  summaryText="Run identity matches"
                  testId="status-pill-solve-proof"
                />
              ) : null}
              {activeSection === "evidence" || activatedExpensiveSections.has("evidence") ? (
              <DormantSection active={activeSection === "evidence"} sessionGeneration={projectSessionGeneration}>
              <RunAuditPanel model={model} result={result} analysisRun={analysisRun} />
              <ValidationEvidencePanel model={model} />
              <BuildReadinessPanel model={model} />
              <TelemetryBoundaryPanel model={model} storageCapability={storageCapability} />
              <SecretPrivateLibraryPanel model={model} storageCapability={storageCapability} />
              <SecurityThreatModelPanel model={model} storageCapability={storageCapability} />
              <AccessibilityBaselinePanel model={model} />
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("report", activeSection)}
              aria-label="Report section"
              data-testid="workspace-section-report"
            >
              {activeSection === "report" || activatedExpensiveSections.has("report") ? (
              <DormantSection active={activeSection === "report"} sessionGeneration={projectSessionGeneration}>
              <Fragment key={`report:${dormantOutputBasis}`}>
              <RenderedReportPanel
                model={model}
                result={currentSolvedResult}
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
                result={currentSolvedResult}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                projectOperation={projectOperation}
                projectSummary={projectSummary}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
                storageCapability={storageCapability}
              />
              <ReportLintPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              </Fragment>
              </DormantSection>) : null}
            </section>
          </div>
            </div>
          </div>
          <button
            aria-label="Resize table and canvas"
            aria-orientation="vertical"
            aria-valuemax={BOTH_SPLIT_PCT_BOUNDS.max}
            aria-valuemin={BOTH_SPLIT_PCT_BOUNDS.min}
            aria-valuenow={uiPreferences.bothSplitPct}
            className="workspace-splitter workspace-splitter-tree shell-both-splitter"
            data-testid="resize-model-tree"
            onKeyDown={(event) => handleWorkspaceSplitterKeyDown(event, "both")}
            onPointerDown={(event) => beginWorkspaceResize(event, "both")}
            role="separator"
            tabIndex={0}
            type="button"
          />
          <div className="workspace-pane workspace-pane-viewport">
            <PipeViewport
              authoringPanelContainer={routingPanelContainer}
              presentationBottomInsetPx={treeCollapsed && (stageView === "model" || (stageView === "both" && narrowWindow)) ? 28 : 0}
              viewCommandRef={viewportViewCommandRef}
              armedCreationTool={armedCreationTool}
              assignment={modelAssignment}
              model={model}
              modelIdentityHash={modelAssignment?.identityHash ?? null}
              hiddenKeys={effectiveHiddenKeys}
              explicitHiddenKeys={hiddenEntityKeys}
              isolateHiddenKeys={isolateHiddenEntityKeys}
              modelIndex={activeModelIndex ?? undefined}
              modelCommitToken={directDraftCommitToken}
              onAddDraft={handleAddDraftReview}
              onApplyDraft={handleApplyDraftReview}
              onArmCreationTool={handleArmCreationTool}
              onBoxSelection={handleBoxSelection}
              onHiddenKeysChange={setHiddenEntityKeys}
              onIsolateKeysChange={setIsolateHiddenEntityKeys}
              onClearVisibility={() => { setHiddenEntityKeys(new Set()); setIsolateHiddenEntityKeys(new Set()); }}
              onViewportInteractionStart={exposeViewportForNarrowInteraction}
              onInvalidateDraft={invalidateDirectDraftContext}
              onQueueIntent={handleQueueEditorIntent}
              onSelect={handleSelectEntity}
              queuedIntents={editorIntents}
              reservedIntents={queuedBatches.flatMap((entry) => entry.batch.operations)}
              result={result}
              selection={selection}
              selectionState={orderedSelection}
              theme={resolvedTheme}
              treePublication={treePublication}
            />
          </div>
          <button
            aria-label="Resize table drawer"
            aria-orientation="horizontal"
            aria-valuemax={TABLE_DRAWER_PX_BOUNDS.max}
            aria-valuemin={TABLE_DRAWER_PX_BOUNDS.min}
            aria-valuenow={uiPreferences.tableDrawerPx}
            className="workspace-dock-splitter shell-drawer-splitter"
            data-testid="resize-task-dock"
            onKeyDown={(event) => handleWorkspaceSplitterKeyDown(event, "drawer")}
            onPointerDown={(event) => beginWorkspaceResize(event, "drawer")}
            role="separator"
            tabIndex={0}
            type="button"
          />
          <div id="shell-inspector" className="workspace-pane workspace-pane-inspector" onKeyDown={(event) => handleNarrowDrawerKeyDown(event, "inspector")}>
            {stageView === "both" ? (
              <button
                type="button"
                className="shell-inspector-close"
                data-testid="inspector-close"
                aria-label="Close inspector"
                title="Close (⎋)"
                onClick={() => toggleWorkspaceRail("inspector")}
              ><X size={14} aria-hidden="true" /></button>
            ) : null}
            <div id="shell-routing-panel" className="shell-routing-panel" ref={setRoutingPanelContainer} />
            <PropertyInspector
              getPreparationEpoch={getPreparationEpoch}
              model={model}
              onQueueIntent={handleQueueEditorIntent}
              onValidateIntent={handleValidateIntent}
              onApplyIntent={handleApplyIntent}
              operationBusy={operationBusy}
              operationOutcomes={operationOutcomes}
              projectSessionGeneration={projectSessionGeneration}
              queuedIntents={editorIntents}
              selection={selection}
              selectionState={orderedSelection}
              taskRequest={propertyTaskRequest}
            />
          </div>
        </section>

        <section
          className={shell.page ? "workspace-dock shell-page" : "workspace-dock shell-page collapsed"}
          aria-label="Workspace sections"
          data-testid="workspace-dock"
          data-page={shell.page ?? undefined}
        >
          <header className="workspace-dock-header" data-testid="workspace-dock-header">
            <h2>{shell.page ? WORKSPACE_SECTIONS.find((candidate) => candidate.id === shell.page)?.label ?? shell.page : ""}</h2>
            <div className={shell.page === "project" ? "titlebar-actions shell-project-controls" : "titlebar-actions shell-project-controls inactive"} aria-label="Local project controls">
          <button type="button" onClick={handleCreateProject} disabled={projectBusy}>
            <Database size={15} aria-hidden="true" />
            Create local
          </button>
          <button type="button" onClick={handleCreateBlankProject} disabled={projectBusy}>
            <FilePlus size={15} aria-hidden="true" />
            New blank
          </button>
          <button data-testid="open-local-project" type="button" onClick={() => handleOpenProject()} disabled={projectBusy}>
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
            {shell.page ? (
              <button type="button" data-testid="workspace-dock-close" title="Close (⎋)" onClick={closeShellPage}>
                <X size={14} aria-hidden="true" /> Close
              </button>
            ) : null}
          </header>
          <div className={shell.page === "project" ? "shell-project-strip" : "shell-project-strip inactive"}>
      <details className="project-strip" aria-label="Project summary">
        <summary><span data-testid="local-project-message" role="status">{projectMessage}</span><span className="project-details-label">Details</span></summary>
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
      </details>
          </div>
          <div className="workspace-dock-body shell-page-sections">
            <section
              className={dockSectionClass("libraries", activeSection)}
              aria-label="Libraries section"
              data-testid="workspace-section-libraries"
              tabIndex={-1}
            >
              {activeSection === "libraries" || activatedExpensiveSections.has("libraries") ? (
              <DormantSection active={activeSection === "libraries"} sessionGeneration={projectSessionGeneration}>
              <LibraryManagerPanel model={model} onR3JourneyEvent={recordR3JourneyEvent} />
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("rule-packs", activeSection)}
              aria-label="Rule Packs section"
              data-testid="workspace-section-rule-packs"
            >
              {activeSection === "rule-packs" || activatedExpensiveSections.has("rule-packs") ? (
              <DormantSection active={activeSection === "rule-packs"} sessionGeneration={projectSessionGeneration}>
              <RulePackManagerPanel model={model} onR3JourneyEvent={recordR3JourneyEvent} />
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("solve", activeSection)}
              aria-label="Solve section"
              data-testid="workspace-section-solve"
            >
              {activeSection === "solve" || activatedExpensiveSections.has("solve") ? (
              <DormantSection active={activeSection === "solve"} sessionGeneration={projectSessionGeneration}>
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
              <RuleCheckPanel model={model} result={currentSolvedResult} />
              <RuleCheckRunPanel
                basis={ruleCheckRunBasis}
                model={model}
                result={currentSolvedResult}
                onAggregateChange={(aggregate) => void handleRuleCheckAggregate(aggregate, {
                  projectSessionGeneration,
                  modelRevision: uiModelRevision,
                  result: currentSolvedResult,
                  inputManifest
                })}
                onR3JourneyEvent={recordR3JourneyEvent}
              />
              <KnowledgePanel knowledge={knowledge} result={currentSolvedResult} />
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("project", activeSection)}
              aria-label="Project section"
              data-testid="workspace-section-project"
            >
              {activeSection === "project" || activatedExpensiveSections.has("project") ? (
              <DormantSection active={activeSection === "project"} sessionGeneration={projectSessionGeneration}>
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
              </DormantSection>) : null}
            </section>

            <section
              className={dockSectionClass("exports", activeSection)}
              aria-label="Exports section"
              data-testid="workspace-section-exports"
            >
              {activeSection === "exports" || activatedExpensiveSections.has("exports") ? (
              <DormantSection active={activeSection === "exports"} sessionGeneration={projectSessionGeneration}>
              <Fragment key={`exports:${dormantOutputBasis}`}>
              <ResultExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} inputManifest={inputManifest} />
              <StressNeutralExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <PcfExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <CaepipeMbfExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <CaepipeExternalHarnessPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <ExportAdapterSdkPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <AdapterFrameworkPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <LocalFeaHandoffPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <ExternalProverBoundaryPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <ReviewGeometryPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />
              <HeadlessRunnerPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} solveJob={solveJob} />
              <NativePackagePanel
                model={model}
                modelHash={modelHash}
                result={currentSolvedResult}
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
                result={currentSolvedResult}
                analysisRun={analysisRun}
                comparison={comparison}
                editorIntents={editorIntents}
                proposal={proposal}
                selectedReviewTarget={selectedReviewTarget}
              />
              <ExportReviewPanel
                model={model}
                knowledge={knowledge}
                result={currentSolvedResult}
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
              </Fragment>
              </DormantSection>) : null}
            </section>
          </div>
        </section>
        </div>
        <AgentStrip />
      </div>

      <ShellStatusBar issueCount={issueCount} />

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
    </main>
    </WorkspaceSessionProvider>
  );
}

// The palette's View commands: the same commands, states and reasons as the
// in-app View menu, so the native runtime (whose menu lists none of them) still
// has a pointer home for each beside the toolbar and the rail.
function shellPaletteCommands({ stage, pageOpen, stageViewMemory, run, theme, density, onCommand }: {
  stage: ShellStage;
  pageOpen: boolean;
  stageViewMemory: Parameters<typeof viewSwitchItems>[1];
  run: Parameters<typeof railStageStates>[0];
  theme: UiThemePreference;
  density: UiDensityPreference;
  onCommand: (command: MenuCommandId) => void;
}): PaletteShellCommand[] {
  return [
    ...viewSwitchItems(stage, stageViewMemory).map((segment): PaletteShellCommand => ({
      id: `view.view.${segment.view}`, label: `${segment.label} view`, active: segment.pressed, disabled: !segment.enabled, reason: segment.reason,
      run: () => onCommand(`view.view.${segment.view}`)
    })),
    ...railStageStates(run).map((state): PaletteShellCommand => ({
      id: `view.stage.${state.stage}`, label: `${state.label} stage`, active: state.stage === stage && !pageOpen, disabled: !state.enabled, reason: state.reason,
      run: () => onCommand(`view.stage.${state.stage}`)
    })),
    ...(["light", "dark", "system"] as const).map((value): PaletteShellCommand => ({
      id: `view.theme.${value}`, label: `${{ light: "Light", dark: "Dark", system: "System" }[value]} theme`, active: theme === value,
      run: () => onCommand(`view.theme.${value}`)
    })),
    ...(["comfortable", "compact"] as const).map((value): PaletteShellCommand => ({
      id: `view.density.${value}`, label: `${{ comfortable: "Comfortable", compact: "Compact" }[value]} density`, active: density === value,
      run: () => onCommand(`view.density.${value}`)
    }))
  ];
}

// Once activated, sections stay mounted so form drafts and queue previews
// survive navigation; expensive sections defer their first model scan until
// the user opens them. CSS removes inactive sections from the accessibility tree.
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
  stage,
  stageViewMemory,
  pageOpen,
  tableDrawer,
  run,
  theme,
  density,
  onCommand,
  onOpenMenu
}: {
  stage: ShellStage;
  stageViewMemory: Parameters<typeof viewSwitchItems>[1];
  pageOpen: boolean;
  tableDrawer: TableDrawerState;
  run: Parameters<typeof railStageStates>[0];
  theme: UiThemePreference;
  density: UiDensityPreference;
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
  const segments = viewSwitchItems(stage, stageViewMemory);
  const currentView: ShellView = segments.find((segment) => segment.pressed)?.view ?? "both";
  const inspector = inspectorToggleState(currentView, !inspectorCollapsed);
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
        // The three views and the four stages: the same setters, states and
        // reasons as the toolbar's view switch and the rail (shellLayout.ts).
        ...segments.map((segment): MenuItemSpec => ({
          kind: "command",
          id: `view.view.${segment.view}`,
          label: segment.label,
          active: segment.pressed,
          disabled: !segment.enabled,
          reason: segment.reason
        })),
        { kind: "separator" },
        ...railStageStates(run).map((state): MenuItemSpec => ({
          kind: "command",
          id: `view.stage.${state.stage}`,
          label: SHELL_STAGE_LABELS[state.stage],
          active: state.stage === stage && !pageOpen,
          disabled: !state.enabled,
          reason: state.reason
        })),
        { kind: "separator" },
        {
          kind: "command",
          id: "view.tree",
          label: "Table Drawer",
          active: tableDrawer.expanded,
          disabled: !tableDrawer.collapsible,
          reason: tableDrawer.reason
        },
        { kind: "command", id: "view.inspector", label: "Inspector", active: inspector.latched, disabled: !inspector.enabled, reason: inspector.reason },
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
        ...(["light", "dark", "system"] as const).map((value): MenuItemSpec => ({
          kind: "command", id: `view.theme.${value}`, label: { light: "Light", dark: "Dark", system: "System" }[value], active: theme === value
        })),
        { kind: "separator" },
        ...(["comfortable", "compact"] as const).map((value): MenuItemSpec => ({
          kind: "command", id: `view.density.${value}`, label: { comfortable: "Comfortable", compact: "Compact" }[value], active: density === value
        })),
        { kind: "separator" },
        { kind: "command", id: "view.close-panels", label: "Close Page", disabled: !pageOpen, reason: pageOpen ? null : "No page is open" }
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
      <nav className={openMenu ? "app-menu-bar menu-open" : "app-menu-bar"} data-testid="app-menu-bar" aria-label="Application menu">
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
                      title={item.disabled && item.reason ? item.reason : undefined}
                      onClick={() => onCommand(item.id)}
                    >
                      {item.label}
                      {item.disabled && item.reason ? <small className="app-menu-reason">{item.reason}</small> : null}
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
  reviewActive,
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
  reviewActive: boolean;
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
        <button type="button" data-testid="agent-open-operations" aria-pressed={reviewActive} onClick={onOpenOperations}>
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

// The summary shows the registered display form with its authority domain
// (features/workspace/statusLabels.ts, DEC-102); the recorded token stays
// reachable in place in the pill's body.
function StatusPill({ label, value, testId, summaryText }: {
  label: string; value: string; testId: string; summaryText?: string;
}) {
  return (
    <details className="status-pill" data-testid={testId}>
      <summary><strong>{label}</strong> {summaryText ?? statusDisplay(value)}</summary>
      <div><strong>Recorded status</strong><code>{value}</code></div>
    </details>
  );
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
