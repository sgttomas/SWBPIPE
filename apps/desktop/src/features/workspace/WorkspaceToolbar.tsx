import type { ReactNode } from "react";
import { Box, ChartNoAxesCombined, ClipboardCheck, FileText, Layers, MousePointer2, Play, ShieldCheck, Undo2, Redo2 } from "lucide-react";

type Section = "operations" | "loads" | "libraries" | "rule-packs" | "solve" | "results" | "report" | "project" | "exports" | "evidence";
type Props = {
  activeSection: Section | null;
  selecting: boolean;
  pendingCount: number;
  canUndo: boolean;
  canRedo: boolean;
  onSelect: () => void;
  onSection: (section: Section | null) => void;
  onUndo: () => void;
  onRedo: () => void;
  children: ReactNode;
};

/** Transient navigation only. Every editing command retains its existing service route. */
export function WorkspaceToolbar({ activeSection, selecting, pendingCount, canUndo, canRedo, onSelect, onSection, onUndo, onRedo, children }: Props) {
  return (
    <div className="workspace-toolbar" data-testid="workspace-toolbar">
      <nav className="workspace-task-nav" aria-label="Workspace tasks">
        {([
          [null, "Model", Box], ["loads", "Loads", Layers], ["solve", "Analyze", Play],
          ["results", "Results", ChartNoAxesCombined], ["rule-packs", "Rules", ShieldCheck], ["report", "Report", FileText]
        ] as const).map(([id, label, Icon]) => (
          <button key={label} type="button" aria-pressed={activeSection === id} data-testid={`workspace-task-${id ?? "model"}`} onClick={() => onSection(id)}>
            <Icon size={16} aria-hidden="true" />{label}
          </button>
        ))}
      </nav>
      <div className="workspace-edit-actions" role="group" aria-label="Editing tools">
        <button type="button" data-testid="workspace-select" aria-pressed={selecting} onClick={onSelect} title="Select geometry; cancel the active creation tool"><MousePointer2 size={16} aria-hidden="true" />Select</button>
        {children}
        <button type="button" data-testid="workspace-undo" aria-label="Undo model edit" disabled={!canUndo} onClick={onUndo} title="Undo model edit"><Undo2 size={16} aria-hidden="true" /></button>
        <button type="button" data-testid="workspace-redo" aria-label="Redo model edit" disabled={!canRedo} onClick={onRedo} title="Redo model edit"><Redo2 size={16} aria-hidden="true" /></button>
        <button type="button" data-testid="workspace-review" aria-pressed={activeSection === "operations"} onClick={() => onSection("operations")} title="Validate and apply queued changes"><ClipboardCheck size={16} aria-hidden="true" />Review{pendingCount > 0 ? <span className="workspace-count">{pendingCount}</span> : null}</button>
      </div>
    </div>
  );
}
