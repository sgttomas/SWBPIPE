import type { RefObject } from "react";
import { ChevronDown, ChevronUp } from "lucide-react";
import { STAGE_TABS } from "../shellLayout";
import type { ShellStage, ShellStageTab, TableDrawerState } from "../shellLayout";
import { DisabledReason } from "./DisabledReason";
import type { WorkspaceSectionId } from "../workspaceSections";

/**
 * The table pane's tab strip: the stage's tabs and, where the pane is a drawer
 * (Model view; Both view below 1280 px), the collapse chevron at its right end,
 * disabled with its reason in the views where the pane is always open.
 * A tab is navigation: it hands `setActiveSection` the section that fills it
 * (null is the model tree). Until slice B4 rebuilds the tables, the Model
 * stage's second tab is today's Review changes and carries the pending count.
 */
export function StageTabStrip({
  stage,
  tab,
  pendingCount,
  drawer,
  toggleRef,
  onTab,
  onToggle
}: {
  stage: ShellStage;
  tab: ShellStageTab;
  pendingCount: number;
  drawer: TableDrawerState;
  toggleRef: RefObject<HTMLButtonElement | null>;
  onTab: (section: WorkspaceSectionId | null) => void;
  onToggle: () => void;
}) {
  return (
    <div className="shell-tab-strip" role="group" aria-label="Tables" data-testid="stage-tab-strip">
      {STAGE_TABS[stage].map((entry) => (
        <button
          type="button"
          key={entry.tab}
          className="shell-tab"
          data-testid={entry.tab === "review-changes" ? "workspace-review" : `stage-tab-${entry.tab}`}
          aria-pressed={entry.tab === tab}
          title={entry.label}
          onClick={() => onTab(entry.section)}
        >
          {entry.label}
          {entry.tab === "review-changes" && pendingCount > 0 ? <span className="workspace-count">{pendingCount}</span> : null}
        </button>
      ))}
      <span className="shell-tab-strip-fill" />
      <span className="shell-reason-anchor shell-drawer-chevron-anchor">
        <button
          type="button"
          className="shell-drawer-chevron"
          ref={toggleRef}
          data-testid="toggle-tree"
          aria-controls="shell-table-body"
          aria-expanded={drawer.expanded}
          aria-disabled={!drawer.collapsible || undefined}
          aria-describedby={drawer.collapsible ? undefined : "toggle-tree-reason"}
          aria-label={drawer.expanded ? "Collapse table drawer" : "Expand table drawer"}
          title={drawer.reason ?? (drawer.expanded ? "Collapse table drawer" : "Expand table drawer")}
          onClick={() => { if (drawer.collapsible) onToggle(); }}
        >
          {drawer.expanded ? <ChevronDown size={16} aria-hidden="true" /> : <ChevronUp size={16} aria-hidden="true" />}
        </button>
        {drawer.collapsible ? null : <DisabledReason id="toggle-tree-reason" text={drawer.reason ?? ""} />}
      </span>
    </div>
  );
}
