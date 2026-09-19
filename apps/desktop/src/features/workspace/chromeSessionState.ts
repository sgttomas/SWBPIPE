import { useRef, useState } from "react";
import type { EntityRef } from "../../types";
import type { CreationTool } from "../viewport/PipeViewport";
import type { ViewportViewCommand } from "../viewport/viewportSelection";
import type { MenuId } from "./menuCommands";
import { EMPTY_STAGE_VIEW_MEMORY } from "./shellLayout";
import type { StageSurface, StageViewMemory } from "./shellLayout";
import { readUiPreferences, resolvedUiTheme } from "./uiPreferences";
import type { WorkspaceSectionId } from "./workspaceSections";

export type R3JourneyEvent =
  | "library_template_loaded"
  | "library_validate_requested"
  | "library_save_requested"
  | "rule_pack_draft_created"
  | "rule_pack_validate_requested"
  | "rule_pack_checksum_requested"
  | "rule_pack_save_requested"
  | "rule_check_pack_loaded"
  | "rule_check_run_requested";

type R3JourneyState = Record<R3JourneyEvent, boolean>;

const INITIAL_R3_JOURNEY_STATE: R3JourneyState = {
  library_template_loaded: false,
  library_validate_requested: false,
  library_save_requested: false,
  rule_pack_draft_created: false,
  rule_pack_validate_requested: false,
  rule_pack_checksum_requested: false,
  rule_pack_save_requested: false,
  rule_check_pack_loaded: false,
  rule_check_run_requested: false
};

/**
 * The chrome cells of the workspace session: the appearance preferences and the
 * resolved theme, the active section and the sections already activated, the
 * toolkit focus and the property-task request, the open menu, the armed creation
 * tool, the rails and drawers, the dock's tab, the stage surface and the per-stage
 * view memory, and the refs the shell's elements attach to. It declares no effect and no handler.
 *
 * Called only by the session, which is `useWorkspaceSession` in
 * `workspaceSession.ts`.
 */
export function useChromeSessionState() {
  const [uiPreferences, setUiPreferences] = useState(readUiPreferences);
  const [systemDark, setSystemDark] = useState(() =>
    typeof window.matchMedia === "function" && window.matchMedia("(prefers-color-scheme: dark)").matches
  );
  const resolvedTheme = resolvedUiTheme(uiPreferences.theme, systemDark);
  // CAD-shell IA (TP-R3UX-CADSHELL): the dock starts collapsed so the spatial
  // core (model tree | 3D viewport | inspector) owns the surface; workspace
  // sections are summoned from the View menu and dismissed back to the viewport.
  const [activeSection, setActiveSection] = useState<WorkspaceSectionId | null>(null);
  const [activatedExpensiveSections, setActivatedExpensiveSections] = useState<ReadonlySet<WorkspaceSectionId>>(
    () => new Set()
  );
  const [toolkitFocus, setToolkitFocus] = useState<{ testId: string; elementId?: string } | null>(null);
  const propertyTaskRequestSequenceRef = useRef(0);
  const [propertyTaskRequest, setPropertyTaskRequest] = useState<{
    sequence: number;
    target: EntityRef;
    view: "properties" | "task";
    focusTestId: string;
    elementId?: string;
  } | null>(null);
  const [openMenu, setOpenMenu] = useState<MenuId | null>(null);
  const [armedCreationTool, setArmedCreationTool] = useState<CreationTool | null>(null);
  const viewportViewCommandRef = useRef<((command: ViewportViewCommand) => void) | null>(null);
  const workspaceShellRef = useRef<HTMLElement | null>(null);
  const workspaceBudgetRef = useRef<HTMLDivElement | null>(null);
  // Viewport-first agent-mediated shell (TP-R3UX-AGENTSHELL-001): the detailed
  // tree and property inspector start tucked away so the primary screen is the
  // 3D model plus a local review-only agent workbench. The detailed rails remain
  // available from View for targeted investigation.
  const [treeCollapsed, setTreeCollapsed] = useState(() => window.innerWidth < 1280);
  const [inspectorCollapsed, setInspectorCollapsed] = useState(() => window.innerWidth < 1280);
  const treeToggleRef = useRef<HTMLButtonElement | null>(null);
  const inspectorToggleRef = useRef<HTMLButtonElement | null>(null);
  const activeResizeCleanupRef = useRef<(() => void) | null>(null);
  const [operationTab, setOperationTab] = useState("review");
  const [r3JourneyState, setR3JourneyState] = useState<R3JourneyState>(() => ({
    ...INITIAL_R3_JOURNEY_STATE
  }));
  const [reviewDetailsOpen, setReviewDetailsOpen] = useState(false);
  const [auditDrawerOpen, setAuditDrawerOpen] = useState(false);
  const [issuesDrawerOpen, setIssuesDrawerOpen] = useState(false);
  // The shell's presentation cells (slice B3); their rules are in `shellLayout.ts`.
  // `activeSection` stays the one navigation cell. `stageSurface` follows it: the
  // last stage surface shown (null is the model tree), which is the stage a page
  // opens over and returns to. The stage itself is derived, never stored.
  const [stageSurface, setStageSurface] = useState<StageSurface>(null);
  // The view each stage was last left in. It is session state: saving it with
  // the project's interface state is gap G-17 (typed interface), outside this
  // tranche, so it starts from the first-open defaults in every session and is
  // never written to the model, the project or `uiPreferences`.
  const [stageViewMemory, setStageViewMemory] = useState<StageViewMemory>(EMPTY_STAGE_VIEW_MEMORY);
  return {
    uiPreferences, setUiPreferences,
    systemDark, setSystemDark,
    resolvedTheme,
    activeSection, setActiveSection,
    activatedExpensiveSections, setActivatedExpensiveSections,
    toolkitFocus, setToolkitFocus,
    propertyTaskRequestSequenceRef,
    propertyTaskRequest, setPropertyTaskRequest,
    openMenu, setOpenMenu,
    armedCreationTool, setArmedCreationTool,
    viewportViewCommandRef,
    workspaceShellRef,
    workspaceBudgetRef,
    treeCollapsed, setTreeCollapsed,
    inspectorCollapsed, setInspectorCollapsed,
    treeToggleRef,
    inspectorToggleRef,
    activeResizeCleanupRef,
    operationTab, setOperationTab,
    r3JourneyState, setR3JourneyState,
    reviewDetailsOpen, setReviewDetailsOpen,
    auditDrawerOpen, setAuditDrawerOpen,
    issuesDrawerOpen, setIssuesDrawerOpen,
    stageSurface, setStageSurface,
    stageViewMemory, setStageViewMemory
  };
}
