import { describe, expect, it } from "vitest";
import {
  AGENT_UNAVAILABLE_REASON,
  EMPTY_STAGE_VIEW_MEMORY,
  FIRST_OPEN_VIEWS,
  RAIL_FOOT_ITEMS,
  SECTION_HOMES,
  SHELL_REGIONS,
  SHELL_STAGES,
  STAGE_TABS,
  canvasAuthoringPanelActive,
  clampBothSplit,
  inspectorToggleState,
  railStageState,
  railStageStates,
  rememberStageView,
  runPresenceFromCells,
  sectionAfterPageClose,
  sectionForStage,
  shellGeometry,
  shellLocation,
  stageSurfaceAfter,
  stageViews,
  statusChipInputsFromCells,
  statusChipText,
  statusChips,
  tableDrawerState,
  viewForStage,
  viewMenuStageItems,
  viewSwitchItems
} from "./shellLayout";
import type { RunPresence, StatusChipInputs } from "./shellLayout";
import { REGISTERED_STATUS_LABELS } from "./statusLabels";
import { WORKSPACE_SECTIONS } from "./workspaceSections";
import type { WorkspaceSectionId } from "./workspaceSections";

describe("stages and views (§2.3)", () => {
  it("opens Model in Both, Loads in Table, Results in Both and Review in Table", () => {
    expect(SHELL_STAGES).toEqual(["model", "loads", "results", "review"]);
    expect(FIRST_OPEN_VIEWS).toEqual({ model: "both", loads: "table", results: "both", review: "table" });
    expect(SHELL_STAGES.map((stage) => viewForStage(EMPTY_STAGE_VIEW_MEMORY, stage))).toEqual([
      "both",
      "table",
      "both",
      "table"
    ]);
  });

  it("remembers each stage's view apart from every other stage's", () => {
    let memory = EMPTY_STAGE_VIEW_MEMORY;
    memory = rememberStageView(memory, "model", "table");
    expect(viewForStage(memory, "model")).toBe("table");
    expect(viewForStage(memory, "loads")).toBe("table");
    expect(viewForStage(memory, "results")).toBe("both");
    memory = rememberStageView(memory, "results", "model");
    memory = rememberStageView(memory, "loads", "both");
    // Leaving and re-entering a stage restores what it was left in.
    expect(SHELL_STAGES.map((stage) => viewForStage(memory, stage))).toEqual(["table", "both", "model", "table"]);
    expect(Object.keys(memory).sort()).toEqual(["loads", "model", "results"]);
  });

  it("returns the same memory when nothing changes, and never mutates", () => {
    const first = rememberStageView(EMPTY_STAGE_VIEW_MEMORY, "model", "model");
    expect(EMPTY_STAGE_VIEW_MEMORY).toEqual({});
    expect(rememberStageView(first, "model", "model")).toBe(first);
    const second = rememberStageView(first, "model", "both");
    expect(second).not.toBe(first);
    expect(first).toEqual({ model: "model" });
  });

  it("keeps Review in Table view: the Review page has no canvas", () => {
    expect(stageViews("review")).toEqual(["table"]);
    expect(stageViews("model")).toEqual(["table", "model", "both"]);
    const memory = rememberStageView(EMPTY_STAGE_VIEW_MEMORY, "review", "both");
    expect(memory).toBe(EMPTY_STAGE_VIEW_MEMORY);
    expect(viewForStage({ review: "both" }, "review")).toBe("table");
    expect(viewSwitchItems("review", EMPTY_STAGE_VIEW_MEMORY).map((item) => [item.view, item.enabled, item.pressed, item.reason])).toEqual([
      ["table", true, true, null],
      ["model", false, false, "Review has no canvas"],
      ["both", false, false, "Review has no canvas"]
    ]);
    expect(viewSwitchItems("model", EMPTY_STAGE_VIEW_MEMORY).every((item) => item.enabled && item.reason === null)).toBe(true);
  });

  it("names each view segment and then its key in parentheses", () => {
    const items = viewSwitchItems("model", EMPTY_STAGE_VIEW_MEMORY);
    expect(items.map((item) => item.tooltip)).toEqual(["Table view (⌘1)", "Model view (⌘2)", "Both view (⌘3)"]);
    expect(items.map((item) => item.label)).toEqual(["Table", "Model", "Both"]);
    expect(items.filter((item) => item.pressed).map((item) => item.view)).toEqual(["both"]);
  });

  it("lets the Inspector toggle act in Both view only, with its reason elsewhere", () => {
    expect(inspectorToggleState("both", true)).toEqual({ enabled: true, latched: true, tooltip: "Inspector (⌘I)", reason: null });
    expect(inspectorToggleState("both", false).latched).toBe(false);
    expect(inspectorToggleState("table", true)).toMatchObject({
      enabled: false,
      latched: false,
      reason: "Table view opens rows in place"
    });
    expect(inspectorToggleState("model", false)).toMatchObject({
      enabled: false,
      latched: true,
      reason: "The inspector is always docked in Model view"
    });
    expect(AGENT_UNAVAILABLE_REASON).toBe("Agent: not available yet");
  });

  it("collapses the table pane only where it is a drawer, and says why elsewhere", () => {
    expect(tableDrawerState("model", false, false)).toEqual({ collapsible: true, expanded: true, reason: null });
    expect(tableDrawerState("model", false, true)).toEqual({ collapsible: true, expanded: false, reason: null });
    // Below 1280 px of window width the Both view falls back to the drawer.
    expect(tableDrawerState("both", true, true)).toEqual({ collapsible: true, expanded: false, reason: null });
    // Elsewhere the pane is always open, whatever the collapse cell holds.
    expect(tableDrawerState("both", false, true)).toEqual({ collapsible: false, expanded: true, reason: "Both view keeps the tables open" });
    expect(tableDrawerState("table", true, true)).toEqual({ collapsible: false, expanded: true, reason: "Table view shows the tables at full width" });
  });
});

describe("the home of each of today's ten sections", () => {
  it("gives every section exactly one home", () => {
    expect(Object.keys(SECTION_HOMES).sort()).toEqual(WORKSPACE_SECTIONS.map((section) => section.id).sort());
    expect(Object.keys(SECTION_HOMES)).toHaveLength(10);
  });

  it("houses them as the cut of B3 gives", () => {
    const where = (section: WorkspaceSectionId | null) => {
      const { stage, tab, page, pageKind } = shellLocation(section, null);
      return page ? `${pageKind} page ${page}` : `${stage}/${tab}`;
    };
    expect(where(null)).toBe("model/model-tree");
    expect(where("operations")).toBe("model/review-changes");
    expect(where("loads")).toBe("loads/load-cases");
    expect(where("results")).toBe("results/results");
    expect(where("evidence")).toBe("results/evidence");
    expect(where("report")).toBe("review/report");
    expect(where("libraries")).toBe("rail page libraries");
    expect(where("rule-packs")).toBe("rail page rule-packs");
    expect(where("solve")).toBe("over-stage page solve");
    expect(where("project")).toBe("over-stage page project");
    expect(where("exports")).toBe("over-stage page exports");
  });

  it("lists each stage's tabs from the same homes", () => {
    for (const stage of SHELL_STAGES) {
      for (const entry of STAGE_TABS[stage]) {
        const location = shellLocation(entry.section, "exports");
        expect([location.stage, location.tab, location.page]).toEqual([stage, entry.tab, null]);
      }
    }
    expect(STAGE_TABS.model.map((entry) => entry.label)).toEqual(["Model", "Review changes"]);
    expect(STAGE_TABS.results.map((entry) => entry.label)).toEqual(["Results", "Evidence"]);
  });

  it("opens a page over the stage surface it found and leaves that surface alone", () => {
    let surface = stageSurfaceAfter(null, "evidence");
    expect(surface).toBe("evidence");
    surface = stageSurfaceAfter(surface, "project");
    expect(surface).toBe("evidence");
    expect(shellLocation("project", surface)).toEqual({
      stage: "results",
      tab: "evidence",
      surface: "evidence",
      page: "project",
      pageKind: "over-stage"
    });
    // A page opened from another page still sits over the same stage.
    surface = stageSurfaceAfter(surface, "rule-packs");
    expect(shellLocation("rule-packs", surface)).toMatchObject({ stage: "results", page: "rule-packs", pageKind: "rail" });
    // The close control returns to the surface underneath.
    expect(sectionAfterPageClose(surface)).toBe("evidence");
    // No section active is the Model stage's tree, whatever was underneath before.
    expect(stageSurfaceAfter(surface, null)).toBeNull();
    expect(shellLocation(null, "evidence")).toMatchObject({ stage: "model", tab: "model-tree", page: null });
  });

  it("never treats a page as a stage surface, even if handed one", () => {
    expect(shellLocation("project", "solve")).toMatchObject({ stage: "model", tab: "model-tree", page: "project" });
    expect(sectionAfterPageClose("solve")).toBeNull();
  });

  it("enters a stage through the one navigation primitive's argument", () => {
    expect(sectionForStage("model", "evidence")).toBeNull();
    expect(sectionForStage("loads", null)).toBe("loads");
    expect(sectionForStage("results", null)).toBe("results");
    expect(sectionForStage("review", null)).toBe("report");
    // Re-entering the stage underneath an open page returns to the tab that was left.
    expect(sectionForStage("results", "evidence")).toBe("evidence");
    expect(sectionForStage("model", "operations")).toBe("operations");
  });
});

const NO_RUN: RunPresence = { hasResult: false, hasCurrentSolvedResult: false, historicalRunShown: false, solveJobState: "not_started" };
const SOLVED: RunPresence = { hasResult: true, hasCurrentSolvedResult: true, historicalRunShown: false, solveJobState: "completed" };
const FAILED: RunPresence = { ...NO_RUN, solveJobState: "failed" };
const STOPPED: RunPresence = { ...NO_RUN, solveJobState: "cancelled" };
const UNSOLVED_RESULT: RunPresence = { hasResult: true, hasCurrentSolvedResult: false, historicalRunShown: false, solveJobState: "completed" };
const HISTORICAL: RunPresence = { ...NO_RUN, historicalRunShown: true };

describe("the rail's states (§2.3, §2.6)", () => {
  const row = (stage: "results" | "review", run: RunPresence) => {
    const { enabled, reason, caption, tooltip } = railStageState(stage, run);
    return { enabled, reason, caption, tooltip };
  };

  it("always enables Model and Loads", () => {
    for (const run of [NO_RUN, SOLVED, FAILED, STOPPED, UNSOLVED_RESULT, HISTORICAL]) {
      expect(railStageState("model", run)).toMatchObject({ enabled: true, reason: null, caption: null, tooltip: "Model" });
      expect(railStageState("loads", run)).toMatchObject({ enabled: true, reason: null, caption: null, tooltip: "Loads" });
    }
  });

  it("before a run: Results and Review disabled with No run yet", () => {
    expect(row("results", NO_RUN)).toEqual({ enabled: false, reason: "No run yet", caption: null, tooltip: "No run yet" });
    expect(row("review", NO_RUN)).toEqual({ enabled: false, reason: "No run yet", caption: null, tooltip: "No run yet" });
    // While the first run is still going there is no run to show yet.
    for (const solveJobState of ["queued", "running", "cancelling"] as const) {
      expect(row("results", { ...NO_RUN, solveJobState }).enabled).toBe(false);
      expect(row("review", { ...NO_RUN, solveJobState }).reason).toBe("No run yet");
    }
  });

  it("after a solved run: both enabled, no caption", () => {
    expect(row("results", SOLVED)).toEqual({ enabled: true, reason: null, caption: null, tooltip: "Results" });
    expect(row("review", SOLVED)).toEqual({ enabled: true, reason: null, caption: null, tooltip: "Review" });
  });

  it("after a failed run with none solved: Results captioned Failed, Review No solved run", () => {
    expect(row("results", FAILED)).toEqual({ enabled: true, reason: null, caption: "Failed", tooltip: "Failed · Results" });
    expect(row("review", FAILED)).toEqual({ enabled: false, reason: "No solved run", caption: null, tooltip: "No solved run" });
  });

  it("after a stopped run, or a run that completed without solving: Results enabled, Review No solved run", () => {
    expect(row("results", STOPPED)).toMatchObject({ enabled: true, caption: null });
    expect(row("review", STOPPED)).toMatchObject({ enabled: false, reason: "No solved run" });
    expect(row("results", UNSOLVED_RESULT)).toMatchObject({ enabled: true, caption: null });
    expect(row("review", UNSOLVED_RESULT)).toMatchObject({ enabled: false, reason: "No solved run" });
  });

  it("with a reopened Historical run: Results captioned Historical, Review Needs a current run", () => {
    expect(row("results", HISTORICAL)).toEqual({ enabled: true, reason: null, caption: "Historical", tooltip: "Historical · Results" });
    expect(row("review", HISTORICAL)).toEqual({
      enabled: false,
      reason: "Needs a current run",
      caption: null,
      tooltip: "Needs a current run"
    });
    // A later run that fails leaves the Historical run on screen: it still names the caption and enables nothing.
    const historicalThenFailed: RunPresence = { ...HISTORICAL, solveJobState: "failed" };
    expect(row("results", historicalThenFailed).caption).toBe("Historical");
    expect(row("review", historicalThenFailed).reason).toBe("Needs a current run");
  });

  it("after a model change there is no run, so the rail reads as before a run; Stale is never drawn", () => {
    // The session clears the result and replaces the job record when a model commit lands.
    expect(row("results", NO_RUN).reason).toBe("No run yet");
    const captions = [NO_RUN, SOLVED, FAILED, STOPPED, UNSOLVED_RESULT, HISTORICAL].flatMap((run) =>
      railStageStates(run).map((state) => state.caption)
    );
    expect(new Set(captions)).toEqual(new Set([null, "Failed", "Historical"]));
  });

  it("gives the View menu the rail's states and reasons, stage for stage", () => {
    for (const run of [NO_RUN, SOLVED, FAILED, STOPPED, UNSOLVED_RESULT, HISTORICAL]) {
      const rail = railStageStates(run);
      const menu = viewMenuStageItems(run);
      expect(menu.map((item) => item.id)).toEqual([
        "view.stage.model",
        "view.stage.loads",
        "view.stage.results",
        "view.stage.review"
      ]);
      expect(menu.map((item) => [item.stage, item.label, item.disabled, item.reason])).toEqual(
        rail.map((state) => [state.stage, state.label, !state.enabled, state.reason])
      );
    }
  });

  it("puts Libraries, Rules and Issues at the rail's foot", () => {
    expect(RAIL_FOOT_ITEMS.map((item) => [item.id, item.label, item.section])).toEqual([
      ["libraries", "Libraries", "libraries"],
      ["rules", "Rules", "rule-packs"],
      ["issues", "Issues", null]
    ]);
  });

  it("reads run presence from the session's cells", () => {
    const solvedResult = { status: { mechanics: "MECHANICS_SOLVED" } };
    expect(runPresenceFromCells({ result: null, historicalRun: null, solveJob: { state: "not_started" } })).toEqual(NO_RUN);
    expect(runPresenceFromCells({ result: solvedResult, historicalRun: null, solveJob: { state: "completed" } })).toEqual(SOLVED);
    expect(
      runPresenceFromCells({ result: { status: { mechanics: "MODEL_INCOMPLETE" } }, historicalRun: null, solveJob: { state: "completed" } })
    ).toEqual(UNSOLVED_RESULT);
    expect(runPresenceFromCells({ result: null, historicalRun: {}, solveJob: { state: "not_started" } })).toEqual(HISTORICAL);
  });
});

function chipInputs(run: RunPresence, extra: Partial<StatusChipInputs> = {}): StatusChipInputs {
  return {
    ...run,
    modelMechanicsStatus: "NOT_RUN",
    resultMechanicsStatus: run.hasResult ? (run.hasCurrentSolvedResult ? "MECHANICS_SOLVED" : "MODEL_INCOMPLETE") : null,
    resultProfessionalStatus: run.hasResult ? "NOT_PROVIDED" : null,
    rulePackStatus: null,
    ...extra
  };
}

describe("the status chip policy (§5.4)", () => {
  const faces = (inputs: StatusChipInputs, stage: Parameters<typeof statusChips>[1], pageOpen = false) =>
    statusChips(inputs, stage, pageOpen).map(statusChipText);

  it("rule 1: one chip, Solver · Model incomplete, while the model document records it", () => {
    for (const stage of SHELL_STAGES) {
      expect(statusChips(chipInputs(NO_RUN, { modelMechanicsStatus: "MODEL_INCOMPLETE" }), stage)).toEqual([
        { token: "MODEL_INCOMPLETE", label: "Model incomplete", domain: "Solver" }
      ]);
    }
  });

  it("rule 2: no chip when the model is complete and no run has solved it", () => {
    for (const stage of SHELL_STAGES) {
      expect(statusChips(chipInputs(NO_RUN, { modelMechanicsStatus: "NOT_RUN" }), stage)).toEqual([]);
      expect(statusChips(chipInputs(NO_RUN, { modelMechanicsStatus: "ready" }), stage)).toEqual([]);
      expect(statusChips(chipInputs(NO_RUN, { modelMechanicsStatus: null }), stage)).toEqual([]);
    }
    // The model document never lights a solved chip: only a run does.
    expect(statusChips(chipInputs(NO_RUN, { modelMechanicsStatus: "MECHANICS_SOLVED" }), "model")).toEqual([]);
  });

  it("rule 3: after a solved run, off Review, the Solver chip and the rule-pack status when a pack is set", () => {
    for (const stage of ["model", "loads", "results"] as const) {
      expect(faces(chipInputs(SOLVED), stage)).toEqual(["Solver · Mechanics solved"]);
      for (const [rulePackStatus, face] of [
        ["RULE_INPUTS_INCOMPLETE", "Rule pack · Rule inputs incomplete"],
        ["USER_RULE_CHECKED", "Rule pack · User-rule checked"],
        ["USER_RULE_FAILED", "Rule pack · User-rule failed"]
      ] as const) {
        expect(faces(chipInputs(SOLVED, { rulePackStatus }), stage)).toEqual(["Solver · Mechanics solved", face]);
      }
    }
  });

  it("rule 4: on Review, three chips with a rule pack and Solver and Human with none", () => {
    expect(faces(chipInputs(SOLVED, { rulePackStatus: "USER_RULE_CHECKED" }), "review")).toEqual([
      "Solver · Mechanics solved",
      "Rule pack · User-rule checked",
      "Human · Human review required"
    ]);
    expect(faces(chipInputs(SOLVED), "review")).toEqual(["Solver · Mechanics solved", "Human · Human review required"]);
    // Human belongs to the Review page: a page open over the Review stage is another page.
    expect(faces(chipInputs(SOLVED), "review", true)).toEqual(["Solver · Mechanics solved"]);
    // The chip is the status the run record carries, never the interface's own.
    expect(faces(chipInputs(SOLVED, { resultProfessionalStatus: "HUMAN_REVIEW_REQUIRED" }), "review")).toContain(
      "Human · Human review required"
    );
    expect(faces(chipInputs(SOLVED, { resultProfessionalStatus: "SOMETHING_ELSE" }), "review")).toEqual([
      "Solver · Mechanics solved"
    ]);
  });

  it("rule 5, as this tranche reads it: after a model change the chips are rule 1's or rule 2's", () => {
    // The session has cleared the result, the aggregate and the job record.
    expect(faces(chipInputs(NO_RUN, { modelMechanicsStatus: "MODEL_INCOMPLETE" }), "results")).toEqual([
      "Solver · Model incomplete"
    ]);
    expect(faces(chipInputs(NO_RUN), "results")).toEqual([]);
  });

  it("rule 6: a Historical run lights no chip, on any stage, whatever its record or the model carries", () => {
    for (const stage of SHELL_STAGES) {
      expect(
        statusChips(
          chipInputs(HISTORICAL, {
            modelMechanicsStatus: "MODEL_INCOMPLETE",
            resultMechanicsStatus: "MECHANICS_SOLVED",
            resultProfessionalStatus: "NOT_PROVIDED",
            rulePackStatus: "USER_RULE_CHECKED"
          }),
          stage
        )
      ).toEqual([]);
    }
  });

  it("rule 7: after a failed or stopped run, only what the run record carries", () => {
    // No status on the record: empty, and Model incomplete is not carried over from before the run.
    expect(statusChips(chipInputs(FAILED, { modelMechanicsStatus: "MODEL_INCOMPLETE" }), "model")).toEqual([]);
    expect(statusChips(chipInputs(STOPPED, { modelMechanicsStatus: "MODEL_INCOMPLETE" }), "results")).toEqual([]);
    // A run that completed without solving carries its Solver status, and lights nothing else.
    expect(faces(chipInputs(UNSOLVED_RESULT, { rulePackStatus: "USER_RULE_CHECKED" }), "review")).toEqual([
      "Solver · Model incomplete"
    ]);
    expect(faces(chipInputs(UNSOLVED_RESULT, { resultMechanicsStatus: "not_a_status" }), "model")).toEqual([]);
  });

  it("draws labels from the one registered table only, never an evidence label, never more than the rules allow", () => {
    const everyInput = [NO_RUN, SOLVED, FAILED, STOPPED, UNSOLVED_RESULT, HISTORICAL].flatMap((run) =>
      [null, "USER_RULE_FAILED", "INTERNALLY_VERIFIED", "made_up"].flatMap((rulePackStatus) =>
        ["MODEL_INCOMPLETE", "NOT_RUN", "PROVER_CORRELATED"].map((modelMechanicsStatus) =>
          chipInputs(run, { rulePackStatus, modelMechanicsStatus })
        )
      )
    );
    for (const inputs of everyInput) {
      for (const stage of SHELL_STAGES) {
        const chips = statusChips(inputs, stage);
        expect(chips.length).toBeLessThanOrEqual(stage === "review" ? 3 : 2);
        expect(new Set(chips.map((chip) => chip.domain)).size).toBe(chips.length);
        for (const chip of chips) {
          const row = REGISTERED_STATUS_LABELS[chip.token];
          expect(row.kind).toBe("status");
          expect(chip).toEqual({ token: row.token, label: row.label, domain: row.domain });
          if (chip.domain === "Human") expect(stage).toBe("review");
        }
      }
    }
  });

  it("reads its inputs from the session's cells", () => {
    expect(
      statusChipInputsFromCells({
        model: { analysis_status: { mechanics: "MODEL_INCOMPLETE" } },
        result: { status: { mechanics: "MECHANICS_SOLVED", professional_acceptance: "NOT_PROVIDED" } },
        historicalRun: null,
        solveJob: { state: "completed" },
        ruleCheckAggregate: "USER_RULE_CHECKED"
      })
    ).toEqual({
      ...SOLVED,
      modelMechanicsStatus: "MODEL_INCOMPLETE",
      resultMechanicsStatus: "MECHANICS_SOLVED",
      resultProfessionalStatus: "NOT_PROVIDED",
      rulePackStatus: "USER_RULE_CHECKED"
    });
    expect(
      statusChipInputsFromCells({ model: null, result: null, historicalRun: null, solveJob: { state: "not_started" }, ruleCheckAggregate: null })
    ).toEqual({ ...NO_RUN, modelMechanicsStatus: null, resultMechanicsStatus: null, resultProfessionalStatus: null, rulePackStatus: null });
  });
});

describe("the regions' designed geometry", () => {
  it("holds the 48 / 56 / 44 / 24 px regions and the docked widths", () => {
    expect(SHELL_REGIONS).toMatchObject({
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
      toggleLabelsMinWindowPx: 1360
    });
  });

  it("gives D-72's canvases at 1440 × 900", () => {
    const both = shellGeometry({ width: 1440, height: 900 }, "both");
    expect(both.surfaces).toEqual({ x: 56, y: 48, width: 1340, height: 828 });
    expect(both.tablePane).toEqual({ x: 56, y: 48, width: 737, height: 828 });
    expect(both.canvas).toEqual({ x: 793, y: 48, width: 603, height: 828 });
    expect(both.inspector).toBeNull();
    const model = shellGeometry({ width: 1440, height: 900 }, "model", { drawerCollapsed: true });
    expect(model.canvas).toEqual({ x: 56, y: 48, width: 1000, height: 828 });
    expect(model.inspector).toEqual({ x: 1056, y: 48, width: 340, height: 828 });
    expect(model.tablePane).toEqual({ x: 56, y: 848, width: 1000, height: 28 });
  });

  it("borrows the Both view's inspector from the table and preserves the default canvas", () => {
    const closed = shellGeometry({ width: 1440, height: 900 }, "both");
    const open = shellGeometry({ width: 1440, height: 900 }, "both", { inspectorOpen: true });
    expect(open.tablePane).toEqual({ x: 56, y: 48, width: 437, height: 828 });
    expect(open.canvas).toEqual({ x: 493, y: 48, width: 603, height: 828 });
    expect(open.inspector).toEqual({ x: 1096, y: 48, width: 300, height: 828 });
    expect(open.canvas?.width).toBe(closed.canvas?.width);
    expect(open.canvas?.x).not.toBe(closed.canvas?.x);
  });

  it("opens the Model view's drawer under the canvas and beside the inspector", () => {
    const model = shellGeometry({ width: 1440, height: 900 }, "model");
    expect(model.canvas).toEqual({ x: 56, y: 48, width: 1000, height: 548 });
    expect(model.tablePane).toEqual({ x: 56, y: 596, width: 1000, height: 280 });
    expect(model.inspector?.height).toBe(828);
  });

  it("gives Table view the whole surface and no canvas or inspector", () => {
    const table = shellGeometry({ width: 1440, height: 920 }, "table");
    expect(table.tablePane).toEqual({ x: 56, y: 48, width: 1340, height: 848 });
    expect(table.canvas).toBeNull();
    expect(table.inspector).toBeNull();
  });

  it("always lends the docked inspector from the table before consuming canvas width", () => {
    for (const [width, height, table, canvas] of [[1440, 900, 437, 603], [1280, 800, 349, 531]] as const) {
      const lent = shellGeometry({ width, height }, "both", { inspectorOpen: true });
      expect([lent.tablePane.width, lent.canvas?.width, lent.inspector?.width]).toEqual([table, canvas, 300]);
      expect(shellGeometry({ width, height }, "both").tablePane.width).toBe(table + 300);
    }
    // The table pane lends down to 320 px and no further; the canvas gives the rest.
    const tight = shellGeometry({ width: 1440, height: 900 }, "both", { bothSplit: 0.3, inspectorOpen: true });
    expect([tight.tablePane.width, tight.canvas?.width]).toEqual([320, 720]);
    const tighter = shellGeometry({ width: 1440, height: 900 }, "both", { bothSplit: 0.2, inspectorOpen: true });
    expect([tighter.tablePane.width, tighter.canvas?.width]).toEqual([268, 772]);
    expect(SHELL_REGIONS.tablePaneLendingMinPx).toBe(320);
  });

  it("knows when the routing editor borrows the inspector from the session's cells", () => {
    expect(canvasAuthoringPanelActive(null, [])).toBe(false);
    for (const tool of ["node", "pipe", "component"]) expect(canvasAuthoringPanelActive(tool, [])).toBe(true);
    // Support opens the inspector and Load opens the Loads stage: neither shows the canvas's panel.
    for (const tool of ["support", "load"]) expect(canvasAuthoringPanelActive(tool, [])).toBe(false);
    expect(canvasAuthoringPanelActive(null, [{ operation_id: "op:inspector-1" }])).toBe(false);
    expect(canvasAuthoringPanelActive(null, [{ operation_id: "op:viewport-intent-3" }])).toBe(true);
    expect(canvasAuthoringPanelActive(null, [{ operation_id: "op:x", source: { source_role: "viewport_editor" } }])).toBe(true);
  });

  it("stops the splitter where the canvas would fall under 220 px", () => {
    expect(clampBothSplit(0.55, 1340)).toBe(0.55);
    expect(clampBothSplit(0.99, 1340)).toBeCloseTo((1340 - 220) / 1340, 10);
    expect(clampBothSplit(-1, 1340)).toBe(0);
    expect(clampBothSplit(Number.NaN, 1340)).toBe(0.55);
    const narrow = shellGeometry({ width: 1440, height: 900 }, "both", { bothSplit: 0.95, inspectorOpen: true });
    expect(narrow.canvas?.width).toBe(220);
    expect(narrow.inspector?.width).toBe(300);
  });
});
