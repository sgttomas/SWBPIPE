import { BookOpen, Box, ClipboardCheck, Flag, Layers, LineChart, Scale } from "lucide-react";
import { RAIL_FOOT_ITEMS, railStageStates, runPresenceFromCells, shellLocation } from "../shellLayout";
import type { ShellStage } from "../shellLayout";
import { useSessionChrome, useSessionResults } from "../WorkspaceSessionContext";
import { DisabledReason } from "./DisabledReason";

const STAGE_ICONS: Record<ShellStage, typeof Box> = { model: Box, loads: Layers, results: LineChart, review: ClipboardCheck };

/**
 * The stage rail (specification §2.3): Model, Loads, Results, Review; a
 * hairline; Libraries, Rules, Issues. It is navigation: every item calls
 * `setActiveSection` through the session, and its states come from
 * `shellLayout.ts`. A disabled item stays focusable so its reason can be read.
 */
export function StageRail({ issueCount }: { issueCount: number }) {
  const { result, historicalRun, solveJob } = useSessionResults();
  const { activeSection, stageSurface, enterStage, setActiveSection, closeShellPage, issuesDrawerOpen, setIssuesDrawerOpen } = useSessionChrome();
  const location = shellLocation(activeSection, stageSurface);
  const stages = railStageStates(runPresenceFromCells({ result, historicalRun, solveJob }));
  return (
    <nav className="shell-rail" aria-label="Stages" data-testid="stage-rail">
      <ul>
        {stages.map((state) => {
          const Icon = STAGE_ICONS[state.stage];
          const current = location.stage === state.stage && location.page === null;
          const reasonId = `rail-reason-${state.stage}`;
          return (
            <li key={state.stage} className="shell-reason-anchor">
              <button
                type="button"
                className="shell-rail-item"
                data-testid={`rail-stage-${state.stage}`}
                aria-current={location.stage === state.stage ? "page" : undefined}
                aria-pressed={current}
                aria-disabled={!state.enabled || undefined}
                aria-describedby={state.enabled ? undefined : reasonId}
                title={state.tooltip}
                onClick={() => { if (state.enabled) enterStage(state.stage); }}
              >
                <Icon size={20} aria-hidden="true" />
                <span className="shell-rail-label">{state.label}</span>
                {state.caption ? <span className="shell-rail-caption" data-testid={`rail-caption-${state.stage}`}>{state.caption}</span> : null}
              </button>
              {state.enabled ? null : <DisabledReason id={reasonId} text={state.reason ?? ""} />}
            </li>
          );
        })}
      </ul>
      <hr className="shell-rail-hairline" />
      <ul>
        {RAIL_FOOT_ITEMS.map((item) => {
          if (item.section === null) {
            return (
              <li key={item.id}>
                <button
                  type="button"
                  className="shell-rail-item"
                  data-testid="rail-issues"
                  aria-expanded={issuesDrawerOpen}
                  aria-label={`Issues, ${issueCount}`}
                  title={item.tooltip}
                  onClick={() => setIssuesDrawerOpen((open) => !open)}
                >
                  <Flag size={20} aria-hidden="true" />
                  <span className="shell-rail-label">{item.label}</span>
                  <span className="shell-rail-badge" aria-hidden="true">{issueCount}</span>
                </button>
              </li>
            );
          }
          const open = location.page === item.section;
          const Icon = item.id === "libraries" ? BookOpen : Scale;
          const section = item.section;
          return (
            <li key={item.id}>
              <button
                type="button"
                className="shell-rail-item"
                data-testid={`rail-page-${item.id}`}
                aria-expanded={open}
                title={item.tooltip}
                onClick={() => { if (open) closeShellPage(); else setActiveSection(section); }}
              >
                <Icon size={20} aria-hidden="true" />
                <span className="shell-rail-label">{item.label}</span>
              </button>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
