// The shell's rules, with no React: stages and views, the home of each of
// today's sections, the rail's states, the status-chip policy, the toolbar's
// view and inspector rules, and the regions' designed geometry.
//
// Basis: UX specification V1.2 §2.3 (stages, views, the per-stage memory, the
// rail's states), §2.6 (run standing), §5.4 (the status chip policy) and §5.5
// (toolbar and status bar); the lane's cut of slice B3, section 3 (where
// today's ten sections live until their own slice rebuilds them).
//
// Two things bind every function here. A stage or view switch is a
// presentation change: nothing in this module reads or writes a model,
// results, operations or project cell other than the few plain values its
// callers pass in. And the interface asserts no status of its own: a chip is a
// status the model document, the run record or the rule-check aggregate
// carries, shown through the one registered table (`statusLabels.ts`).

import type { SolveJobEvent } from "../../types";
import { professionalStatusToken, registeredStatusLabel } from "./statusLabels";
import type { RegisteredStatusToken, StatusAuthorityDomain } from "./statusLabels";
import type { WorkspaceSectionId } from "./workspaceSections";

// ---------------------------------------------------------------------------
// Stages and views (§2.3)

export type ShellStage = "model" | "loads" | "results" | "review";
export type ShellView = "table" | "model" | "both";

export const SHELL_STAGES: readonly ShellStage[] = ["model", "loads", "results", "review"];
export const SHELL_VIEWS: readonly ShellView[] = ["table", "model", "both"];

export const SHELL_STAGE_LABELS: Readonly<Record<ShellStage, string>> = Object.freeze({
  model: "Model",
  loads: "Loads",
  results: "Results",
  review: "Review"
});

/** The view a stage opens in before it has ever been left. */
export const FIRST_OPEN_VIEWS: Readonly<Record<ShellStage, ShellView>> = Object.freeze({
  model: "both",
  loads: "table",
  results: "both",
  review: "table"
});

/** The Review stage has no canvas (§2.3): it is the Review page, in Table view only. */
export function stageViews(stage: ShellStage): readonly ShellView[] {
  return stage === "review" ? ["table"] : SHELL_VIEWS;
}

/** What the per-stage memory holds: only the stages whose view was ever chosen. */
export type StageViewMemory = Readonly<Partial<Record<ShellStage, ShellView>>>;

export const EMPTY_STAGE_VIEW_MEMORY: StageViewMemory = Object.freeze({});

/** The view a stage shows: the one it was last left in, or its first-open default. */
export function viewForStage(memory: StageViewMemory, stage: ShellStage): ShellView {
  const remembered = memory[stage];
  return remembered && stageViews(stage).includes(remembered) ? remembered : FIRST_OPEN_VIEWS[stage];
}

/**
 * The memory after a view is chosen in a stage. Only that stage's entry
 * changes; a view the stage does not have is not remembered. Returns the same
 * object when nothing changes, so a state setter given this result re-renders
 * nothing.
 */
export function rememberStageView(memory: StageViewMemory, stage: ShellStage, view: ShellView): StageViewMemory {
  if (!stageViews(stage).includes(view)) return memory;
  if (memory[stage] === view) return memory;
  return { ...memory, [stage]: view };
}

export type ViewSwitchItem = {
  view: ShellView;
  label: string;
  tooltip: string;
  accelerator: string;
  pressed: boolean;
  enabled: boolean;
  /** Why the segment is disabled; null when it is enabled. */
  reason: string | null;
};

/** ROOT's wording for the Model and Both segments on the Review stage. */
export const REVIEW_HAS_NO_CANVAS_REASON = "Review has no canvas";

const VIEW_SWITCH_COPY: Readonly<Record<ShellView, { label: string; tooltip: string; accelerator: string }>> =
  Object.freeze({
    table: { label: "Table", tooltip: "Table view (⌘1)", accelerator: "⌘1" },
    model: { label: "Model", tooltip: "Model view (⌘2)", accelerator: "⌘2" },
    both: { label: "Both", tooltip: "Both view (⌘3)", accelerator: "⌘3" }
  });

/** The toolbar's three segments for a stage. The keys accelerate the segments and nothing else. */
export function viewSwitchItems(stage: ShellStage, memory: StageViewMemory): ViewSwitchItem[] {
  const current = viewForStage(memory, stage);
  const available = stageViews(stage);
  return SHELL_VIEWS.map((view) => {
    const enabled = available.includes(view);
    return {
      view,
      ...VIEW_SWITCH_COPY[view],
      pressed: view === current,
      enabled,
      reason: enabled ? null : REVIEW_HAS_NO_CANVAS_REASON
    };
  });
}

export type InspectorToggleState = { enabled: boolean; latched: boolean; tooltip: string; reason: string | null };

/**
 * The toolbar's Inspector toggle (§2.3, §5.5). Table view has no inspector and
 * Model view always docks it, so the toggle acts in Both view only.
 */
export function inspectorToggleState(view: ShellView, inspectorOpen: boolean): InspectorToggleState {
  if (view === "table") {
    return { enabled: false, latched: false, tooltip: "Table view opens rows in place", reason: "Table view opens rows in place" };
  }
  if (view === "model") {
    return {
      enabled: false,
      latched: true,
      tooltip: "The inspector is always docked in Model view",
      reason: "The inspector is always docked in Model view"
    };
  }
  return { enabled: true, latched: inspectorOpen, tooltip: "Inspector (⌘I)", reason: null };
}

export type TableDrawerState = { collapsible: boolean; expanded: boolean; reason: string | null };

/**
 * The table pane's collapse chevron. The pane is a drawer in Model view, and in
 * Both view below 1280 px of window width (the narrow fallback); elsewhere it is
 * always open and the chevron is disabled with its reason.
 */
export function tableDrawerState(view: ShellView, narrowWindow: boolean, collapsed: boolean): TableDrawerState {
  if (view === "table") return { collapsible: false, expanded: true, reason: "Table view shows the tables at full width" };
  if (view === "both" && !narrowWindow) return { collapsible: false, expanded: true, reason: "Both view keeps the tables open" };
  return { collapsible: true, expanded: !collapsed, reason: null };
}

/** The Agent toggle and the agent strip: the column is a later slice and its live binding is the host gap G-19. */
export const AGENT_UNAVAILABLE_REASON = "Agent: not available yet";

// ---------------------------------------------------------------------------
// The home of each of today's sections (the cut of B3, section 3)

export type ShellStageTab = "model-tree" | "review-changes" | "load-cases" | "results" | "evidence" | "report";
export type ShellPageKind = "rail" | "over-stage";

export type SectionHome =
  | { kind: "stage"; stage: ShellStage; tab: ShellStageTab }
  | { kind: "page"; page: WorkspaceSectionId; pageKind: ShellPageKind };

/**
 * Where each of today's ten sections lives until its own slice rebuilds it.
 * The model tree is not a section: it is the Model stage's first tab, shown
 * when no section is active.
 */
export const SECTION_HOMES: Readonly<Record<WorkspaceSectionId, SectionHome>> = Object.freeze({
  operations: { kind: "stage", stage: "model", tab: "review-changes" },
  loads: { kind: "stage", stage: "loads", tab: "load-cases" },
  results: { kind: "stage", stage: "results", tab: "results" },
  evidence: { kind: "stage", stage: "results", tab: "evidence" },
  report: { kind: "stage", stage: "review", tab: "report" },
  libraries: { kind: "page", page: "libraries", pageKind: "rail" },
  "rule-packs": { kind: "page", page: "rule-packs", pageKind: "rail" },
  solve: { kind: "page", page: "solve", pageKind: "over-stage" },
  project: { kind: "page", page: "project", pageKind: "over-stage" },
  exports: { kind: "page", page: "exports", pageKind: "over-stage" }
});

/** A stage's tabs in strip order, each with the section that fills it (null is the model tree). */
export const STAGE_TABS: Readonly<Record<ShellStage, ReadonlyArray<{ tab: ShellStageTab; label: string; section: WorkspaceSectionId | null }>>> =
  Object.freeze({
    model: [
      { tab: "model-tree", label: "Model", section: null },
      { tab: "review-changes", label: "Review changes", section: "operations" }
    ],
    loads: [{ tab: "load-cases", label: "Load Cases", section: "loads" }],
    results: [
      { tab: "results", label: "Results", section: "results" },
      { tab: "evidence", label: "Evidence", section: "evidence" }
    ],
    review: [{ tab: "report", label: "Report", section: "report" }]
  });

/**
 * A stage surface is what `activeSection` holds while no page is open: null
 * (the model tree) or a section whose home is a stage.
 */
export type StageSurface = WorkspaceSectionId | null;

export function isStageSurface(section: WorkspaceSectionId | null): boolean {
  return section === null || SECTION_HOMES[section].kind === "stage";
}

/** The stage a stage surface belongs to. No section active is the Model stage's tree. */
export function stageOfSurface(surface: StageSurface): ShellStage {
  if (surface === null) return "model";
  const home = SECTION_HOMES[surface];
  return home.kind === "stage" ? home.stage : "model";
}

/**
 * The stage surface to remember after `activeSection` becomes `next`: a page
 * opens over the surface it found and leaves it alone; anything else is the
 * new surface.
 */
export function stageSurfaceAfter(previous: StageSurface, next: WorkspaceSectionId | null): StageSurface {
  return isStageSurface(next) ? next : previous;
}

export type ShellLocation = {
  stage: ShellStage;
  tab: ShellStageTab;
  /** The section drawn in the stage's table pane; null is the model tree. */
  surface: StageSurface;
  /** The page open over the stage's surfaces, or null. */
  page: WorkspaceSectionId | null;
  pageKind: ShellPageKind | null;
};

/**
 * Section → stage, tab or page. `setActiveSection` stays the one navigation
 * primitive: the shell never stores where it is, it derives it from the active
 * section and, while a page is open, from the stage surface the page was
 * opened over.
 */
export function shellLocation(activeSection: WorkspaceSectionId | null, stageSurface: StageSurface): ShellLocation {
  const surface: StageSurface = isStageSurface(activeSection) ? activeSection : isStageSurface(stageSurface) ? stageSurface : null;
  const stage = stageOfSurface(surface);
  const tab = surface === null ? "model-tree" : (SECTION_HOMES[surface] as Extract<SectionHome, { kind: "stage" }>).tab;
  if (activeSection !== null && !isStageSurface(activeSection)) {
    const home = SECTION_HOMES[activeSection] as Extract<SectionHome, { kind: "page" }>;
    return { stage, tab, surface, page: home.page, pageKind: home.pageKind };
  }
  return { stage, tab, surface, page: null, pageKind: null };
}

/**
 * What the rail passes to `setActiveSection` to enter a stage. Re-entering the
 * stage already underneath returns to the surface that was left (so closing a
 * page and clicking the current stage agree); another stage opens on its first tab.
 */
export function sectionForStage(stage: ShellStage, stageSurface: StageSurface): WorkspaceSectionId | null {
  if (isStageSurface(stageSurface) && stageOfSurface(stageSurface) === stage) return stageSurface;
  return STAGE_TABS[stage][0].section;
}

/** What a page's close control passes to `setActiveSection`: the surface the page was opened over. */
export function sectionAfterPageClose(stageSurface: StageSurface): WorkspaceSectionId | null {
  return isStageSurface(stageSurface) ? stageSurface : null;
}

// ---------------------------------------------------------------------------
// The rail's states (§2.3, §2.6). The Stale caption is not drawn in this
// tranche (G-11): after a model change there is no run to show, as today.

export type RunPresence = {
  /** A result of the current session's run is held (`result !== null`), solved or not. */
  hasResult: boolean;
  /** That result is a solved one (`currentSolvedResult !== null`): the Current solved run. */
  hasCurrentSolvedResult: boolean;
  /** A reopened saved run is shown (`historicalRun !== null`). */
  historicalRunShown: boolean;
  /** `solveJob.state`. */
  solveJobState: SolveJobEvent["state"];
};

export type RailCaption = "Failed" | "Historical";

export type RailStageState = {
  stage: ShellStage;
  label: string;
  enabled: boolean;
  /** Why the stage is disabled; shown as the tooltip and reachable by keyboard focus. Null when enabled. */
  reason: "No run yet" | "No solved run" | "Needs a current run" | null;
  /** A state name under the label, not a status. */
  caption: RailCaption | null;
  tooltip: string;
};

function runEnded(state: SolveJobEvent["state"]): boolean {
  return state === "failed" || state === "cancelled";
}

export function railStageState(stage: ShellStage, run: RunPresence): RailStageState {
  const label = SHELL_STAGE_LABELS[stage];
  let enabled = true;
  let reason: RailStageState["reason"] = null;
  let caption: RailCaption | null = null;
  if (stage === "results") {
    // There is a run to show once a run has completed, stopped or failed, or a saved run is reopened.
    enabled = run.hasResult || run.historicalRunShown || runEnded(run.solveJobState);
    if (!enabled) reason = "No run yet";
    // What is shown names the caption: a reopened saved run stays on screen while a later run fails.
    else if (run.historicalRunShown) caption = "Historical";
    else if (!run.hasResult && run.solveJobState === "failed") caption = "Failed";
  } else if (stage === "review") {
    // Only a Current solved run enables Review; a Historical run enables nothing.
    enabled = run.hasCurrentSolvedResult && !run.historicalRunShown;
    if (!enabled) {
      if (run.historicalRunShown) reason = "Needs a current run";
      else if (run.hasResult || runEnded(run.solveJobState)) reason = "No solved run";
      else reason = "No run yet";
    }
  }
  // A tooltip that also summarises content puts the summary first and the control last (§2.8).
  const tooltip = reason ?? (caption ? `${caption} · ${label}` : label);
  return { stage, label, enabled, reason, caption, tooltip };
}

export function railStageStates(run: RunPresence): RailStageState[] {
  return SHELL_STAGES.map((stage) => railStageState(stage, run));
}

export type StageCommandState = { id: string; stage: ShellStage; label: string; disabled: boolean; reason: string | null };

/**
 * The View menu's four stage items. They are the rail's states and nothing
 * else, so a stage the rail disables is disabled in the menu with the same reason.
 */
export function viewMenuStageItems(run: RunPresence): StageCommandState[] {
  return railStageStates(run).map((state) => ({
    id: `view.stage.${state.stage}`,
    stage: state.stage,
    label: state.label,
    disabled: !state.enabled,
    reason: state.reason
  }));
}

export type RailFootItem =
  | { id: "libraries" | "rules"; label: string; tooltip: string; section: WorkspaceSectionId }
  | { id: "issues"; label: string; tooltip: string; section: null };

/** Under the hairline: two pages and the issues drawer. The Issues item also carries the issue count. */
export const RAIL_FOOT_ITEMS: readonly RailFootItem[] = Object.freeze([
  { id: "libraries", label: "Libraries", tooltip: "Libraries", section: "libraries" },
  { id: "rules", label: "Rules", tooltip: "Rules", section: "rule-packs" },
  { id: "issues", label: "Issues", tooltip: "Issues", section: null }
] as const);

// ---------------------------------------------------------------------------
// The status chip policy (§5.4). The status bar speaks for the current model
// and its Current run, and for nothing else.

export type StatusChip = { token: RegisteredStatusToken; label: string; domain: StatusAuthorityDomain };

export type StatusChipInputs = RunPresence & {
  /** `model.analysis_status.mechanics`: the status the model document records; null with no model. */
  modelMechanicsStatus: string | null;
  /** `result.status.mechanics` of the held result; null with none. */
  resultMechanicsStatus: string | null;
  /** `result.status.professional_acceptance` of the held result; null with none. */
  resultProfessionalStatus: string | null;
  /**
   * `ruleCheckAggregate`: the status of the rule check run against the loaded
   * rule pack for the current solved result. Null means no rule pack is set.
   */
  rulePackStatus: string | null;
};

function chipFor(value: string | null, domain: StatusAuthorityDomain): StatusChip | null {
  if (value === null) return null;
  const row = registeredStatusLabel(value);
  // Evidence labels are never status-bar chips, and a value outside the table has no chip.
  if (!row || row.kind !== "status" || row.domain !== domain) return null;
  return { token: row.token, label: row.label, domain: row.domain };
}

/**
 * The chips of the status bar's left end, from the session's cells and where
 * the shell is. `pageOpen` is true while a page covers the stage: Human ·
 * Human review required belongs to the Review page and to no other.
 *
 * Rule 5 reads, for this tranche: after a model change there is no run (the
 * session clears it), so the chips are those of rule 1 or rule 2.
 */
export function statusChips(inputs: StatusChipInputs, stage: ShellStage, pageOpen = false): StatusChip[] {
  // Rule 6: a Historical run lights no chip. A chip beside it would read as
  // the current model's standing, which a Historical record never supplies.
  if (inputs.historicalRunShown) return [];
  if (inputs.hasResult) {
    const solver = chipFor(inputs.resultMechanicsStatus, "Solver");
    // Rule 7, first row: a run that did not solve shows whatever Solver status its record carries.
    if (!inputs.hasCurrentSolvedResult) return solver ? [solver] : [];
    // Rules 3 and 4: one chip per authority domain for the Current solved run.
    const chips: StatusChip[] = [];
    if (solver) chips.push(solver);
    const rulePack = chipFor(inputs.rulePackStatus, "Rule pack");
    if (rulePack) chips.push(rulePack);
    if (stage === "review" && !pageOpen) {
      const human = chipFor(
        inputs.resultProfessionalStatus === null ? null : professionalStatusToken(inputs.resultProfessionalStatus),
        "Human"
      );
      if (human) chips.push(human);
    }
    return chips;
  }
  // Rule 7, second row: a stopped or failed run whose record carries no status
  // leaves the bar empty; Model incomplete is not carried over from before the run.
  if (runEnded(inputs.solveJobState)) return [];
  // Rule 1: the model document records that it cannot be solved. Rule 2: otherwise no chip.
  const incomplete = chipFor(inputs.modelMechanicsStatus, "Solver");
  return incomplete && incomplete.token === "MODEL_INCOMPLETE" ? [incomplete] : [];
}

/** "Solver · Mechanics solved": the chip's face. The raw token is its tooltip and its popover's monospace line. */
export function statusChipText(chip: StatusChip): string {
  return `${chip.domain} · ${chip.label}`;
}

/** The cells the two rules read, taken from the session's slices as plain values. */
export function runPresenceFromCells(cells: {
  result: { status: { mechanics: string } } | null;
  historicalRun: object | null;
  solveJob: { state: SolveJobEvent["state"] };
}): RunPresence {
  return {
    hasResult: cells.result !== null,
    hasCurrentSolvedResult: cells.result?.status.mechanics === "MECHANICS_SOLVED",
    historicalRunShown: cells.historicalRun !== null,
    solveJobState: cells.solveJob.state
  };
}

export function statusChipInputsFromCells(cells: {
  model: { analysis_status: { mechanics: string } } | null;
  result: { status: { mechanics: string; professional_acceptance?: string } } | null;
  historicalRun: object | null;
  solveJob: { state: SolveJobEvent["state"] };
  ruleCheckAggregate: string | null;
}): StatusChipInputs {
  return {
    ...runPresenceFromCells(cells),
    modelMechanicsStatus: cells.model?.analysis_status.mechanics ?? null,
    resultMechanicsStatus: cells.result?.status.mechanics ?? null,
    resultProfessionalStatus: cells.result?.status.professional_acceptance ?? null,
    rulePackStatus: cells.ruleCheckAggregate
  };
}

// ---------------------------------------------------------------------------
// The regions' designed geometry (design system §5.2, §5.3; D-72's canvases).
// jsdom lays nothing out, so the built shell is measured by Playwright; this
// is the arithmetic the stylesheet is written to, kept where a unit test holds it.

export const SHELL_REGIONS = Object.freeze({
  toolbarPx: 48,
  railPx: 56,
  agentStripPx: 44,
  statusBarPx: 24,
  inspectorBothPx: 300,
  inspectorModelPx: 340,
  tableDrawerPx: 280,
  tableDrawerTabStripPx: 28,
  bothSplitDefault: 0.55,
  canvasMinPx: 220,
  /** Below this window width the two panel toggles drop their labels and keep their tooltips. */
  toggleLabelsMinWindowPx: 1360
});

export type ShellRect = { x: number; y: number; width: number; height: number };

export type ShellGeometry = {
  surfaces: ShellRect;
  /** The stage's table pane: the full surface, the Both view's left part, or the Model view's drawer. Null never. */
  tablePane: ShellRect;
  canvas: ShellRect | null;
  inspector: ShellRect | null;
};

/** The largest table share of the Both view's split that still leaves the canvas its minimum. */
export function clampBothSplit(split: number, surfaceWidth: number, inspectorOpen: boolean): number {
  const reserved = SHELL_REGIONS.canvasMinPx + (inspectorOpen ? SHELL_REGIONS.inspectorBothPx : 0);
  const max = surfaceWidth > 0 ? Math.max(0, (surfaceWidth - reserved) / surfaceWidth) : 0;
  if (!Number.isFinite(split)) return Math.min(SHELL_REGIONS.bothSplitDefault, max);
  return Math.min(Math.max(split, 0), max);
}

/**
 * Where the surfaces sit in a window. In Both view the inspector takes its
 * width from the canvas, never from the table. In Model view the drawer spans
 * the canvas column; collapsed, its 28 px tab strip lies over the foot of the
 * canvas's box, so the canvas keeps D-72's full height.
 */
export function shellGeometry(
  window: { width: number; height: number },
  view: ShellView,
  options: { bothSplit?: number; inspectorOpen?: boolean; drawerCollapsed?: boolean; drawerPx?: number } = {}
): ShellGeometry {
  const r = SHELL_REGIONS;
  const surfaces: ShellRect = {
    x: r.railPx,
    y: r.toolbarPx,
    width: Math.max(0, window.width - r.railPx - r.agentStripPx),
    height: Math.max(0, window.height - r.toolbarPx - r.statusBarPx)
  };
  if (view === "table") return { surfaces, tablePane: { ...surfaces }, canvas: null, inspector: null };
  if (view === "both") {
    const inspectorOpen = options.inspectorOpen ?? false;
    const split = clampBothSplit(options.bothSplit ?? r.bothSplitDefault, surfaces.width, inspectorOpen);
    const tableWidth = Math.round(surfaces.width * split);
    const right = surfaces.width - tableWidth;
    const inspectorWidth = inspectorOpen ? r.inspectorBothPx : 0;
    return {
      surfaces,
      tablePane: { x: surfaces.x, y: surfaces.y, width: tableWidth, height: surfaces.height },
      canvas: { x: surfaces.x + tableWidth, y: surfaces.y, width: right - inspectorWidth, height: surfaces.height },
      inspector: inspectorOpen
        ? { x: surfaces.x + surfaces.width - inspectorWidth, y: surfaces.y, width: inspectorWidth, height: surfaces.height }
        : null
    };
  }
  const canvasWidth = Math.max(0, surfaces.width - r.inspectorModelPx);
  const collapsed = options.drawerCollapsed ?? false;
  const drawerHeight = collapsed ? r.tableDrawerTabStripPx : options.drawerPx ?? r.tableDrawerPx;
  return {
    surfaces,
    tablePane: { x: surfaces.x, y: surfaces.y + surfaces.height - drawerHeight, width: canvasWidth, height: drawerHeight },
    canvas: { x: surfaces.x, y: surfaces.y, width: canvasWidth, height: collapsed ? surfaces.height : surfaces.height - drawerHeight },
    inspector: { x: surfaces.x + canvasWidth, y: surfaces.y, width: r.inspectorModelPx, height: surfaces.height }
  };
}
