import type { ReactNode } from "react";
import {
  Bot,
  Columns2,
  Flag,
  MousePointer2,
  PanelRight,
  Play,
  Redo2,
  SlidersHorizontal,
  Square,
  Table2,
  Undo2,
  Box
} from "lucide-react";
import { DisplayUnitSelector } from "../../display-units";
import { AGENT_UNAVAILABLE_REASON, inspectorToggleState, shellLocation, viewSwitchItems } from "../shellLayout";
import type { ShellView } from "../shellLayout";
import { updateUiPreferences } from "../uiPreferences";
import {
  useSessionChrome,
  useSessionModel,
  useSessionOperations,
  useSessionProject,
  useSessionResults
} from "../WorkspaceSessionContext";
import { DisabledReason } from "./DisabledReason";

const VIEW_ICONS: Record<ShellView, typeof Table2> = { table: Table2, model: Box, both: Columns2 };

/**
 * The toolbar band (specification §5.5): wordmark, project name, Undo and Redo,
 * the view switch, Run, the Issues count, the two panel toggles, the display
 * units and the palette field. Every control here maps to a session handler;
 * none mutates the model by itself. `children` is the one palette surface.
 */
export function ShellToolbar({ issueCount, children }: { issueCount: number; children: ReactNode }) {
  const { model } = useSessionModel();
  const { projectSummary } = useSessionProject();
  const { undoStack, redoStack, operationBusy, handleUndoSessionModelEdit, handleRedoSessionModelEdit } = useSessionOperations();
  const { running, handleRun, handleCancelRun } = useSessionResults();
  const {
    activeSection,
    stageSurface,
    stageViewMemory,
    chooseStageView,
    inspectorCollapsed,
    inspectorToggleRef,
    toggleWorkspaceRail,
    armedCreationTool,
    handleArmCreationTool,
    issuesDrawerOpen,
    setIssuesDrawerOpen,
    uiPreferences,
    setUiPreferences
  } = useSessionChrome();
  const location = shellLocation(activeSection, stageSurface);
  const segments = viewSwitchItems(location.stage, stageViewMemory);
  const view = segments.find((segment) => segment.pressed)?.view ?? "both";
  const inspector = inspectorToggleState(view, !inspectorCollapsed);
  const canUndo = undoStack.length > 0 && !operationBusy;
  const canRedo = redoStack.length > 0 && !operationBusy;
  // The save state is not shown: the session holds no cell that says whether the
  // model changed since the last save, so "saved" or "edited" could not be told truthfully.
  const projectName = projectSummary?.project_name ?? model?.project.name ?? "";

  return (
    <header className="shell-toolbar workspace-toolbar" data-testid="workspace-toolbar" aria-label="Toolbar">
      <div className="shell-toolbar-identity titlebar">
        <h1 className="shell-wordmark">SWBPIPE</h1>
        <p className="shell-project-name" data-testid="toolbar-project-name" title={projectName}>{projectName}</p>
      </div>
      <div className="shell-toolbar-group" role="group" aria-label="Editing tools">
        <button type="button" className="shell-icon-button" data-testid="workspace-undo" aria-label="Undo model edit" disabled={!canUndo} onClick={handleUndoSessionModelEdit} title={canUndo ? "Undo (⌘Z)" : "Nothing to undo"}><Undo2 size={16} aria-hidden="true" /></button>
        <button type="button" className="shell-icon-button" data-testid="workspace-redo" aria-label="Redo model edit" disabled={!canRedo} onClick={handleRedoSessionModelEdit} title={canRedo ? "Redo (⇧⌘Z)" : "Nothing to redo"}><Redo2 size={16} aria-hidden="true" /></button>
        <button type="button" className="shell-icon-button shell-select-tool" data-testid="workspace-select" aria-pressed={armedCreationTool === null} onClick={() => handleArmCreationTool(null)} title="Select (⎋)"><MousePointer2 size={16} aria-hidden="true" /><span className="shell-toolbar-label">Select</span></button>
      </div>
      <div className="shell-view-switch" role="group" aria-label="View">
        {segments.map((segment) => {
          const Icon = VIEW_ICONS[segment.view];
          const reasonId = `view-switch-reason-${segment.view}`;
          return (
            <span className="shell-reason-anchor" key={segment.view}>
              <button
                type="button"
                data-testid={`view-switch-${segment.view}`}
                aria-pressed={segment.pressed}
                aria-disabled={!segment.enabled || undefined}
                aria-describedby={segment.enabled ? undefined : reasonId}
                title={segment.enabled ? segment.tooltip : segment.reason ?? undefined}
                onClick={() => { if (segment.enabled) chooseStageView(segment.view); }}
              >
                <Icon size={15} aria-hidden="true" /><span>{segment.label}</span>
              </button>
              {segment.enabled ? null : <DisabledReason id={reasonId} text={segment.reason ?? ""} />}
            </span>
          );
        })}
      </div>
      <div className="shell-toolbar-centre">
        {running ? (
          <button type="button" className="shell-run" data-testid="toolbar-stop" onClick={() => void handleCancelRun()} title="Stop"><Square size={14} aria-hidden="true" /><span>Stop</span></button>
        ) : (
          <button type="button" className="shell-run" data-testid="toolbar-run" onClick={() => void handleRun()} title="Run"><Play size={15} aria-hidden="true" /><span>Run</span></button>
        )}
        <button
          type="button"
          className="shell-issues"
          data-testid="toolbar-issues"
          aria-expanded={issuesDrawerOpen}
          aria-label={`Issues, ${issueCount}`}
          title="Issues"
          onClick={() => setIssuesDrawerOpen((open) => !open)}
        ><Flag size={14} aria-hidden="true" /><span className="shell-toolbar-label">Issues</span><span>{issueCount}</span></button>
      </div>
      <div className="shell-toolbar-group shell-panel-toggles" role="group" aria-label="Panels">
        <span className="shell-reason-anchor">
          <button
            type="button"
            ref={inspectorToggleRef}
            className={inspector.latched ? "shell-panel-toggle latched" : "shell-panel-toggle"}
            data-testid="toggle-inspector"
            aria-label="Inspector"
            aria-controls="shell-inspector"
            aria-expanded={inspector.latched}
            aria-disabled={!inspector.enabled || undefined}
            aria-describedby={inspector.enabled ? undefined : "toggle-inspector-reason"}
            title={inspector.tooltip}
            onClick={() => { if (inspector.enabled) toggleWorkspaceRail("inspector"); }}
          ><PanelRight size={16} aria-hidden="true" /><span className="shell-toolbar-label">Inspector</span></button>
          {inspector.enabled ? null : <DisabledReason id="toggle-inspector-reason" text={inspector.reason ?? ""} />}
        </span>
        <span className="shell-reason-anchor">
          <button
            type="button"
            className="shell-panel-toggle"
            data-testid="toggle-agent"
            aria-label="Agent"
            aria-disabled="true"
            aria-describedby="toggle-agent-reason"
            title={AGENT_UNAVAILABLE_REASON}
          ><Bot size={16} aria-hidden="true" /><span className="shell-toolbar-label">Agent</span></button>
          <DisabledReason id="toggle-agent-reason" text={AGENT_UNAVAILABLE_REASON} />
        </span>
      </div>
      <div className="shell-toolbar-units" title="Display units"><DisplayUnitSelector compact /></div>
      <details className="shell-appearance" data-testid="toolbar-appearance">
        <summary aria-label="Appearance" title="Appearance"><SlidersHorizontal size={16} aria-hidden="true" /></summary>
        <div className="shell-appearance-popover">
          <label>
            <span>Theme</span>
            <select
              aria-label="Appearance theme"
              onChange={(event) => setUiPreferences((current) => updateUiPreferences(current, { theme: event.target.value as typeof current.theme }))}
              value={uiPreferences.theme}
            >
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </label>
          <label>
            <span>Density</span>
            <select
              aria-label="Workspace density"
              onChange={(event) => setUiPreferences((current) => updateUiPreferences(current, { density: event.target.value as typeof current.density }))}
              value={uiPreferences.density}
            >
              <option value="comfortable">Comfortable</option>
              <option value="compact">Compact</option>
            </select>
          </label>
        </div>
      </details>
      <div className="shell-toolbar-palette">{children}</div>
    </header>
  );
}
