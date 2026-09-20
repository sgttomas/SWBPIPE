import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { App } from "./App";
import { buildCurrentSessionInputManifest } from "./services/inputManifestService";
import { buildAnalysisRunPreview, loadPreviewModel, runPreviewMechanics } from "./services/previewService";
import type { LocalProjectEnvelope, PreviewModel } from "./types";

// Slice B3: the shell through the product. The rules themselves are unit-tested
// in features/workspace/shellLayout.test.ts; these tests hold that the shell's
// controls follow them, that every control reaches a session handler, and that
// no presentation change disturbs a mounted panel.

const initialInnerWidth = window.innerWidth;

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  Object.defineProperty(window, "innerWidth", { configurable: true, value: initialInnerWidth });
  window.localStorage.clear();
});

function setTauriRuntime(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

async function renderShell(width = 1440) {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  return screen.getByTestId("modeling-workspace");
}

async function solve() {
  fireEvent.click(screen.getByTestId("toolbar-run"));
  await waitFor(() => expect(chipFaces()).toContain("Solver · Mechanics solved"), { timeout: 30_000 });
}

function chipFaces(): string[] {
  return within(screen.getByTestId("status-chips")).queryAllByRole("button").map((chip) => chip.textContent ?? "");
}

function rail(stage: "model" | "loads" | "results" | "review") {
  return screen.getByTestId(`rail-stage-${stage}`);
}

function railState(stage: "results" | "review") {
  const item = rail(stage);
  const reasonId = item.getAttribute("aria-describedby");
  return {
    disabled: item.getAttribute("aria-disabled") === "true",
    tooltip: item.getAttribute("title"),
    reason: reasonId ? document.getElementById(reasonId)?.textContent ?? null : null,
    caption: screen.queryByTestId(`rail-caption-${stage}`)?.textContent ?? null
  };
}

function inventedOpenEnvelope(model: PreviewModel): LocalProjectEnvelope {
  const openedModel = JSON.parse(JSON.stringify(model)) as PreviewModel;
  openedModel.project.id = "project:invented-shell-open";
  openedModel.project.name = "Invented shell open";
  return {
    summary: {
      project_id: openedModel.project.id,
      project_name: openedModel.project.name,
      database_path: "/invented/local/shell-open.sqlite",
      storage_mode: "local_sqlite",
      migration_status: "current",
      migration_framework: "application_service_separate_db_and_product_schema",
      store_schema_version: 1,
      store_schema_target_version: 1,
      migrations_applied_on_open: [],
      fts_indexed: true,
      copied_external_files: false,
      editor_intent_count: 0,
      proposal_count: 0,
      selected_review_target_count: 0,
      selected_review_target_ref: "none",
      persisted_mechanics_result_count: 1,
      persisted_analysis_run_count: 1,
      persisted_analysis_run_ref: "none",
      persisted_model_hash_count: 0,
      persisted_model_hash_ref: "none",
      persisted_project_envelope_hash_count: 0,
      persisted_project_envelope_hash_ref: "none",
      unit_round_trip_status: "not_checked_in_invented_test",
      unit_round_trip_checked_ref_count: 0,
      unit_round_trip_signature: "none",
      message: "Opened invented shell project.",
    },
    model: openedModel,
    editor_intents: [],
    proposal: null,
    selected_review_target: null,
    mechanics_result: null,
    analysis_run: null,
    model_hash: null,
    project_envelope_hash: null,
    model_document_migration: null,
    model_migration_ledger: [],
  };
}

async function openHistoricalRun() {
  const envelope = inventedOpenEnvelope(await loadPreviewModel());
  const result = structuredClone(await runPreviewMechanics());
  result.model_ref = envelope.model.project.id;
  const inputManifest = await buildCurrentSessionInputManifest({
    model: envelope.model,
    solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.1.0", solver_build_ref: "open_pipe_stress_product_physics@0.1.0", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [], external_assets: []
  });
  envelope.mechanics_result = result;
  envelope.analysis_run = await buildAnalysisRunPreview(result, { inputManifest });
  invokeMock.mockImplementation((command: string) =>
    command === "open_local_project" ? Promise.resolve(envelope) : Promise.reject(new Error(`Unexpected command ${command}`)));
  setTauriRuntime(true);
  act(() => nativeMenuCommand("file.open-local"));
  await waitFor(() => expect(screen.getByTestId("historical-run-context")).toHaveTextContent(result.run_id));
}

describe("the rail's states through the product", () => {
  it("before a run: Results and Review are disabled with No run yet, reachable by focus, and do nothing", async () => {
    await renderShell();
    for (const stage of ["results", "review"] as const) {
      expect(railState(stage)).toEqual({ disabled: true, tooltip: "No run yet", reason: "No run yet", caption: null });
      // aria-disabled, never disabled: the item takes focus so its reason can be read.
      expect(rail(stage)).not.toBeDisabled();
      rail(stage).focus();
      expect(rail(stage)).toHaveFocus();
      fireEvent.click(rail(stage));
      expect(rail("model")).toHaveAttribute("aria-current", "page");
    }
    expect(rail("model")).not.toHaveAttribute("aria-disabled");
    expect(rail("loads")).not.toHaveAttribute("aria-disabled");
    // The View menu agrees with the rail, stage for stage, with the same reason.
    fireEvent.click(screen.getByTestId("menu-view"));
    expect(screen.getByTestId("menu-item-view.stage.results")).toBeDisabled();
    expect(screen.getByTestId("menu-item-view.stage.results")).toHaveTextContent("No run yet");
    expect(screen.getByTestId("menu-item-view.stage.review")).toBeDisabled();
    expect(screen.getByTestId("menu-item-view.stage.review")).toHaveTextContent("No run yet");
    expect(screen.getByTestId("menu-item-view.stage.loads")).toBeEnabled();
    // The section commands stay enabled even for a stage the rail disables.
    expect(screen.getByTestId("menu-item-view.section.results")).toBeEnabled();
  });

  it("after a solved run: both stages open, with no caption", async () => {
    await renderShell();
    await solve();
    for (const stage of ["results", "review"] as const) {
      expect(railState(stage)).toMatchObject({ disabled: false, reason: null, caption: null });
    }
    fireEvent.click(rail("results"));
    expect(rail("results")).toHaveAttribute("aria-current", "page");
    expect(screen.getByTestId("workspace-section-results")).not.toHaveClass("inactive");
    fireEvent.click(rail("review"));
    expect(rail("review")).toHaveAttribute("aria-current", "page");
    expect(screen.getByTestId("workspace-section-report")).not.toHaveClass("inactive");
    fireEvent.click(screen.getByTestId("menu-view"));
    expect(screen.getByTestId("menu-item-view.stage.review")).toBeEnabled();
    expect(screen.getByTestId("menu-item-view.stage.review")).toHaveAttribute("aria-pressed", "true");
  });

  it("after a failed run with none solved: Results is captioned Failed and Review says No solved run", async () => {
    await renderShell();
    invokeMock.mockImplementation((command: string) =>
      Promise.reject(new Error(command === "start_preview_mechanics_job_with_solver_mode" ? "Invented solve failure" : `Unexpected command ${command}`)));
    setTauriRuntime(true);
    fireEvent.click(screen.getByTestId("toolbar-run"));
    await waitFor(() => expect(railState("results").caption).toBe("Failed"));
    expect(railState("results")).toEqual({ disabled: false, tooltip: "Failed · Results", reason: null, caption: "Failed" });
    expect(railState("review")).toEqual({ disabled: true, tooltip: "No solved run", reason: "No solved run", caption: null });
    // §5.4 rule 7: the failed run's record carries no status, so the bar is empty.
    expect(chipFaces()).toEqual([]);
  });

  it("with a reopened Historical run: Results is captioned Historical and Review needs a current run", async () => {
    await renderShell();
    await openHistoricalRun();
    expect(railState("results")).toEqual({ disabled: false, tooltip: "Historical · Results", reason: null, caption: "Historical" });
    expect(railState("review")).toEqual({ disabled: true, tooltip: "Needs a current run", reason: "Needs a current run", caption: null });
    // The reopen lands on the Results stage, where the Historical record is read.
    expect(rail("results")).toHaveAttribute("aria-current", "page");
    expect(screen.queryByText("Stale")).not.toBeInTheDocument();
  });
});

describe("the status chip policy through the product (§5.4)", () => {
  it("rule 2: a complete model that no run has solved lights no chip", async () => {
    await renderShell();
    expect(chipFaces()).toEqual([]);
    fireEvent.click(rail("loads"));
    expect(chipFaces()).toEqual([]);
  });

  it("rules 3 and 4 with no rule pack: Solver alone off Review; Solver and Human on Review, and on no page over it", async () => {
    await renderShell();
    await solve();
    for (const stage of ["model", "loads", "results"] as const) {
      fireEvent.click(rail(stage));
      expect(chipFaces()).toEqual(["Solver · Mechanics solved"]);
    }
    fireEvent.click(rail("review"));
    expect(chipFaces()).toEqual(["Solver · Mechanics solved", "Human · Human review required"]);
    // A page over the Review stage is another page: Human belongs to Review only.
    fireEvent.click(screen.getByTestId("rail-page-libraries"));
    expect(chipFaces()).toEqual(["Solver · Mechanics solved"]);
    fireEvent.click(screen.getByTestId("workspace-dock-close"));
    expect(rail("review")).toHaveAttribute("aria-current", "page");
    expect(chipFaces()).toEqual(["Solver · Mechanics solved", "Human · Human review required"]);
  });

  it("rules 3 and 4 with a rule pack: two chips off Review, three on it", async () => {
    await renderShell();
    await solve();
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.reject(new Error("Invented catalog unavailable"));
      if (command === "run_rule_checks") {
        return Promise.resolve({
          document_kind: "openpipestress.rule_check.run",
          rule_pack_id: "invented_demo_rule_pack",
          grammar_version: "1.0.0",
          aggregate_status: "USER_RULE_FAILED",
          checks: [{
            check_id: "invented_shell_check",
            status: "USER_RULE_FAILED",
            acceptability_relation: "less_than_or_equal",
            bound_inputs: [],
            completeness_findings: [],
            evaluator_findings: [],
            diagnostic_codes: [],
          }],
          professional_boundary_notice:
            "Rule-check results remain engineering decision-support information requiring responsible-engineer review.",
        });
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    setTauriRuntime(true);
    act(() => nativeMenuCommand("analyze.rule-checks"));
    const solvePage = screen.getByTestId("workspace-section-solve");
    fireEvent.click(within(solvePage).getByTestId("rule-check-load-demo"));
    await within(solvePage).findByTestId("rule-check-binding-plan");
    fireEvent.click(within(solvePage).getByTestId("rule-check-run"));
    await waitFor(() => expect(chipFaces()).toEqual(["Solver · Mechanics solved", "Rule pack · User-rule failed"]));
    fireEvent.click(rail("review"));
    expect(chipFaces()).toEqual(["Solver · Mechanics solved", "Rule pack · User-rule failed", "Human · Human review required"]);
  });

  it("rule 6: a Historical run lights no chip on any stage it can reach", async () => {
    await renderShell();
    await openHistoricalRun();
    expect(chipFaces()).toEqual([]);
    for (const stage of ["model", "loads", "results"] as const) {
      fireEvent.click(rail(stage));
      expect(chipFaces()).toEqual([]);
    }
  });

  it("rule 5, as this tranche reads it: a model change drops the chips with the run", async () => {
    await renderShell();
    await solve();
    expect(chipFaces()).toEqual(["Solver · Mechanics solved"]);
    fireEvent.click(screen.getByTestId("menu-file"));
    invokeMock.mockReset();
    fireEvent.click(screen.getByTestId("menu-item-file.new-blank"));
    await waitFor(() => expect(chipFaces()).toEqual(["Solver · Model incomplete"]));
    expect(railState("results")).toMatchObject({ disabled: true, reason: "No run yet", caption: null });
  });

  it("a chip is read, never clicked to change anything: the click opens the raw token and the domain", async () => {
    await renderShell();
    await solve();
    const chip = screen.getByTestId("status-pill-mechanics");
    expect(chip).toHaveAttribute("title", "MECHANICS_SOLVED");
    expect(screen.queryByTestId("status-pill-mechanics-popover")).not.toBeInTheDocument();
    fireEvent.click(chip);
    const popover = screen.getByTestId("status-pill-mechanics-popover");
    expect(within(popover).getByText("MECHANICS_SOLVED").tagName).toBe("CODE");
    expect(popover).toHaveTextContent("Solver");
    expect(chipFaces()).toEqual(["Solver · Mechanics solved"]);
    fireEvent.click(chip);
    expect(screen.queryByTestId("status-pill-mechanics-popover")).not.toBeInTheDocument();
  });
});

describe("views, stages and mounted panels", () => {
  it("keeps the canvas, the inspector and all ten sections mounted, with their state, across every stage and view", async () => {
    const surfaces = await renderShell();
    await solve();
    // State to keep: a tree filter, a solver-mode choice on the Analyze page, an inspector tab.
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "N-1" } });
    act(() => nativeMenuCommand("view.section.solve"));
    fireEvent.click(screen.getByTestId("solver-mode-dense"));
    expect(screen.getByTestId("solver-mode-dense")).toHaveAttribute("aria-pressed", "true");
    fireEvent.click(screen.getByTestId("workspace-dock-close"));
    const inspector = screen.getByTestId("property-inspector");
    fireEvent.click(within(inspector).getByRole("tab", { name: "Task" }));

    const sections = ["operations", "loads", "libraries", "rule-packs", "solve", "results", "report", "project", "exports", "evidence"];
    const mounted = new Map<string, HTMLElement>([
      ["viewport-canvas", screen.getByTestId("viewport-canvas")],
      ["property-inspector", inspector],
      ["model-tree-filter-input", screen.getByTestId("model-tree-filter-input")],
      ["solver-mode-dense", screen.getByTestId("solver-mode-dense")],
      ...sections.map((id): [string, HTMLElement] => [`workspace-section-${id}`, screen.getByTestId(`workspace-section-${id}`)])
    ]);
    const expectAllMounted = (step: string) => {
      for (const [testId, element] of mounted) expect(screen.getByTestId(testId), `${step}: ${testId}`).toBe(element);
      expect(screen.getByTestId("model-tree-filter-input"), step).toHaveValue("N-1");
      expect(screen.getByTestId("solver-mode-dense"), step).toHaveAttribute("aria-pressed", "true");
      expect(within(inspector).getByRole("tab", { name: "Task" }), step).toHaveAttribute("aria-selected", "true");
    };

    for (const stage of ["model", "loads", "results", "review", "model"] as const) {
      fireEvent.click(rail(stage));
      expect(surfaces).toHaveAttribute("data-stage", stage);
      expectAllMounted(`stage ${stage}`);
      for (const view of ["table", "model", "both"] as const) {
        const segment = screen.getByTestId(`view-switch-${view}`);
        fireEvent.click(segment);
        if (stage === "review" && view !== "table") {
          expect(segment).toHaveAttribute("aria-disabled", "true");
          expect(document.getElementById(segment.getAttribute("aria-describedby")!)).toHaveTextContent("Review has no canvas");
          expect(surfaces).toHaveAttribute("data-view", "table");
        } else {
          expect(segment).toHaveAttribute("aria-pressed", "true");
          expect(surfaces).toHaveAttribute("data-view", view);
        }
        expectAllMounted(`stage ${stage}, view ${view}`);
      }
    }
  });

  it("opens each stage in its first-open view and restores the view each was left in", async () => {
    const surfaces = await renderShell();
    await solve();
    const firstOpen = { model: "both", loads: "table", results: "both", review: "table" } as const;
    for (const stage of ["model", "loads", "results", "review"] as const) {
      fireEvent.click(rail(stage));
      expect(surfaces).toHaveAttribute("data-view", firstOpen[stage]);
    }
    fireEvent.click(rail("model"));
    fireEvent.click(screen.getByTestId("view-switch-model"));
    fireEvent.click(rail("loads"));
    fireEvent.click(screen.getByTestId("view-switch-both"));
    fireEvent.click(rail("results"));
    expect(surfaces).toHaveAttribute("data-view", "both");
    fireEvent.click(rail("model"));
    expect(surfaces).toHaveAttribute("data-view", "model");
    fireEvent.click(rail("loads"));
    expect(surfaces).toHaveAttribute("data-view", "both");
    // A page over a stage changes neither the stage underneath nor its remembered view.
    fireEvent.click(screen.getByTestId("rail-page-rules"));
    expect(screen.getByTestId("workspace-dock")).toHaveAttribute("data-page", "rule-packs");
    expect(surfaces).toHaveAttribute("data-stage", "loads");
    fireEvent.click(screen.getByTestId("rail-page-rules"));
    expect(screen.getByTestId("workspace-dock")).toHaveClass("collapsed");
    expect(surfaces).toHaveAttribute("data-stage", "loads");
    expect(surfaces).toHaveAttribute("data-view", "both");
  });

  it("keeps the five project controls findable by role and name while the Project page is closed, and opens them on it", async () => {
    await renderShell();
    const names = [/^Create local$/, /^New blank$/, /^Open local$/, /^List local$/, /^Save local$/];
    expect(screen.getByTestId("workspace-dock")).toHaveClass("collapsed");
    for (const name of names) expect(screen.getByRole("button", { name })).toBeInTheDocument();
    expect(screen.getByTestId("open-local-project")).toHaveAccessibleName("Open local");
    act(() => nativeMenuCommand("view.section.project"));
    const controls = screen.getByLabelText("Local project controls");
    expect(controls).not.toHaveClass("inactive");
    for (const name of names) expect(within(controls).getByRole("button", { name })).toBeEnabled();
    expect(screen.getByTestId("workspace-dock-header")).toHaveTextContent("Project");
    fireEvent.click(screen.getByTestId("workspace-dock-close"));
    expect(controls).toHaveClass("inactive");
  });
});

describe("the toolbar band", () => {
  it("runs and stops through the session's handlers, and leaves no second route", async () => {
    await renderShell();
    const running = new Promise<never>(() => undefined);
    invokeMock.mockImplementation((command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return Promise.resolve({ job_id: "shell-run", backend_cancellation_token: "shell-run-token", state: "queued", cancellation_scope: "synthetic" });
      if (command === "poll_preview_mechanics_job") return running;
      if (command === "cancel_preview_mechanics_job") return Promise.resolve({ job_id: "shell-run", cancellation_requested: true, cancellation_status: "requested", cancellation_scope: "synthetic" });
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    setTauriRuntime(true);
    fireEvent.click(screen.getByTestId("toolbar-run"));
    const stop = await screen.findByTestId("toolbar-stop");
    expect(screen.queryByTestId("toolbar-run")).not.toBeInTheDocument();
    expect(invokeMock.mock.calls.filter(([command]) => command === "start_preview_mechanics_job_with_solver_mode")).toHaveLength(1);
    fireEvent.click(stop);
    await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "cancel_preview_mechanics_job")).toBe(true));
  });

  it("disables the Inspector toggle with its reason outside Both view, and the Agent toggle and strip always", async () => {
    await renderShell();
    const toggle = screen.getByTestId("toggle-inspector");
    const reason = () => document.getElementById(toggle.getAttribute("aria-describedby") ?? "")?.textContent ?? null;
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    expect(toggle).toHaveAttribute("aria-controls", "shell-inspector");
    expect(toggle).toHaveAttribute("title", "Inspector (⌘I)");
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    fireEvent.click(screen.getByTestId("inspector-close"));
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(screen.getByTestId("view-switch-table"));
    expect(toggle).toHaveAttribute("aria-disabled", "true");
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    expect(reason()).toBe("Table view opens rows in place");
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(screen.getByTestId("view-switch-model"));
    expect(toggle).toHaveAttribute("aria-disabled", "true");
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    expect(reason()).toBe("The inspector is always docked in Model view");
    expect(screen.queryByTestId("inspector-close")).not.toBeInTheDocument();

    for (const testId of ["toggle-agent", "agent-strip-open"]) {
      const control = screen.getByTestId(testId);
      expect(control).toHaveAttribute("aria-disabled", "true");
      expect(control).not.toBeDisabled();
      control.focus();
      expect(control).toHaveFocus();
      expect(document.getElementById(control.getAttribute("aria-describedby")!)).toHaveTextContent("Agent: not available yet");
      expect(control).toHaveAttribute("title", "Agent: not available yet");
    }
  });

  it("carries the pending-change count on the Model stage's Review changes tab and names the stage on the rail", async () => {
    // What the superseded WorkspaceToolbar's tests held: the Review control opens Review changes
    // and carries the pending count. Its "Current workspace task" readout is replaced by the rail.
    await renderShell();
    fireEvent.click(rail("loads"));
    expect(screen.queryByTestId("workspace-review")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("load-manager-primitive-load:L-100-P"));
    fireEvent.change(screen.getByTestId("load-manager-magnitude-value"), { target: { value: "1500000" } });
    fireEvent.click(screen.getByTestId("queue-load-magnitude-intent"));
    // Queueing a change summons Review changes, as before: that is now the Model stage's second tab.
    expect(rail("model")).toHaveAttribute("aria-current", "page");
    const review = screen.getByTestId("workspace-review");
    expect(review).toHaveTextContent("Review changes");
    expect(review).toHaveTextContent("1");
    expect(review).toHaveAttribute("aria-pressed", "true");
    fireEvent.click(screen.getByTestId("stage-tab-model-tree"));
    expect(review).toHaveAttribute("aria-pressed", "false");
    expect(review).toHaveTextContent("1");
    fireEvent.click(review);
    expect(review).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByTestId("workspace-section-operations")).not.toHaveClass("inactive");
    expect(screen.getByTestId("operation-tab-review")).toHaveAttribute("aria-pressed", "true");
    expect(await screen.findByTestId("apply-intent-editor-intent-1")).toBeInTheDocument();
    expect(screen.queryByRole("status", { name: "Current workspace task" })).not.toBeInTheDocument();
    expect(rail("model")).toHaveAttribute("aria-current", "page");
    fireEvent.click(screen.getByTestId("stage-tab-model-tree"));
    expect(screen.getByTestId("workspace-section-operations")).toHaveClass("inactive");
    expect(screen.getByTestId("shell-tree-host")).not.toHaveClass("inactive");
  });

  it("says Nothing to undo and Nothing to redo on the one undo stack's two buttons", async () => {
    await renderShell();
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(screen.getByTestId("workspace-undo")).toHaveAttribute("title", "Nothing to undo");
    expect(screen.getByTestId("workspace-redo")).toBeDisabled();
    expect(screen.getByTestId("workspace-redo")).toHaveAttribute("title", "Nothing to redo");
  });

  it("keeps theme and density on labelled controls, in the View menu and in the palette", async () => {
    await renderShell();
    const shell = screen.getByTestId("desktop-preview-shell");
    fireEvent.change(screen.getByLabelText("Appearance theme"), { target: { value: "dark" } });
    expect(shell).toHaveAttribute("data-theme", "dark");
    fireEvent.change(screen.getByLabelText("Workspace density"), { target: { value: "compact" } });
    expect(shell).toHaveAttribute("data-density", "compact");
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.theme.light"));
    expect(shell).toHaveAttribute("data-theme", "light");
    fireEvent.click(screen.getByTestId("toolkit-entry"));
    fireEvent.click(screen.getByTestId("toolkit-shell-view.density.comfortable"));
    expect(shell).toHaveAttribute("data-density", "comfortable");
    fireEvent.click(screen.getByTestId("toolkit-entry"));
    fireEvent.click(screen.getByTestId("toolkit-shell-view.view.model"));
    expect(screen.getByTestId("modeling-workspace")).toHaveAttribute("data-view", "model");
  });
});

describe("the pointer rule: a key only accelerates a visible control", () => {
  it("⌘1 ⌘2 ⌘3 do what the view switch's segments do, and nothing where a segment is disabled", async () => {
    const surfaces = await renderShell();
    for (const [key, view] of [["1", "table"], ["2", "model"], ["3", "both"]] as const) {
      const segment = screen.getByTestId(`view-switch-${view}`);
      expect(segment).toHaveAttribute("title", expect.stringContaining(`(⌘${key})`));
      fireEvent.keyDown(window, { key, metaKey: true });
      expect(surfaces).toHaveAttribute("data-view", view);
      expect(segment).toHaveAttribute("aria-pressed", "true");
    }
    await solve();
    fireEvent.click(rail("review"));
    fireEvent.keyDown(window, { key: "3", metaKey: true });
    expect(surfaces).toHaveAttribute("data-view", "table");
  });

  it("⌘I does what the Inspector toggle does, and nothing where the toggle is disabled", async () => {
    await renderShell();
    const toggle = screen.getByTestId("toggle-inspector");
    fireEvent.keyDown(window, { key: "i", metaKey: true });
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    fireEvent.keyDown(window, { key: "i", ctrlKey: true });
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(screen.getByTestId("view-switch-table"));
    fireEvent.keyDown(window, { key: "i", metaKey: true });
    expect(toggle).toHaveAttribute("aria-expanded", "false");
  });

  it("⌘K opens what the palette field opens, and ⎋ closes what a page's close control closes", async () => {
    await renderShell();
    expect(screen.getByTestId("toolkit-entry")).toHaveAttribute("title", "Search or command (⌘K)");
    fireEvent.keyDown(window, { key: "k", metaKey: true });
    expect(screen.getByRole("dialog", { name: "Find a modeling tool" })).toBeInTheDocument();
    fireEvent.keyDown(screen.getByRole("searchbox", { name: "Find a tool" }), { key: "Escape" });
    fireEvent.click(screen.getByTestId("toolkit-entry"));
    expect(screen.getByRole("dialog", { name: "Find a modeling tool" })).toBeInTheDocument();
    fireEvent.keyDown(screen.getByRole("searchbox", { name: "Find a tool" }), { key: "Escape" });

    fireEvent.click(rail("loads"));
    fireEvent.click(screen.getByTestId("rail-page-libraries"));
    expect(screen.getByTestId("workspace-dock-close")).toHaveAttribute("title", "Close (⎋)");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(screen.getByTestId("workspace-dock")).toHaveClass("collapsed");
    // ⎋ closes the page onto the stage underneath; it never leaves a stage.
    expect(rail("loads")).toHaveAttribute("aria-current", "page");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(rail("loads")).toHaveAttribute("aria-current", "page");
  });
});


describe("routing borrows the inspector", () => {
  it.each([true, false])("restores the prior collapsed state %s after routing ends", async (collapsed) => {
    const surfaces = await renderShell();
    if (!collapsed) fireEvent.click(screen.getByTestId("toggle-inspector"));
    expect(surfaces.classList.contains("inspector-collapsed")).toBe(collapsed);
    fireEvent.click(screen.getByTestId("command-pipe"));
    expect(surfaces).not.toHaveClass("inspector-collapsed");
    const routing = document.getElementById("shell-routing-panel")!;
    expect(routing.querySelector(".viewport-intents.active")).not.toBeNull();
    expect(document.querySelector(".viewport-shell > .viewport-intents.active")).toBeNull();
    fireEvent.click(screen.getByTestId("workspace-select"));
    expect(surfaces.classList.contains("inspector-collapsed")).toBe(collapsed);
  });

  it("reveals Both view for routing from Table and returns focus before hiding the inspector", async () => {
    const surfaces = await renderShell();
    fireEvent.click(screen.getByTestId("view-switch-table"));
    act(() => nativeMenuCommand("insert.node"));
    expect(surfaces).toHaveAttribute("data-view", "both");
    expect(surfaces).not.toHaveClass("inspector-collapsed");
    const routing = document.getElementById("shell-routing-panel")!;
    const control = routing.querySelector<HTMLElement>("input, button")!;
    control.focus();
    act(() => window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true })));
    expect(surfaces).toHaveClass("inspector-collapsed");
    expect(screen.getByTestId("workspace-select")).toHaveFocus();
  });
});

describe("native shell command equivalence", () => {
  it("uses the shared sink for native views, stages and appearance, and publishes matching menu state", async () => {
    const surfaces = await renderShell();
    setTauriRuntime(true);
    invokeMock.mockImplementation((command: string) => command === "sync_native_shell_state" ? Promise.resolve(undefined) : Promise.reject(new Error(`Unavailable test command ${command}`)));
    act(() => nativeMenuCommand("view.view.model"));
    expect(surfaces).toHaveAttribute("data-view", "model");
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("sync_native_shell_state", {
      state: expect.objectContaining({ stage: "model", view: "model", inspectorOpen: true, resultsStageEnabled: false, reviewStageEnabled: false, canUndo: false, canRedo: false, canRun: true, canCancel: false })
    }));
    act(() => nativeMenuCommand("view.view.both"));
    expect(surfaces).toHaveClass("inspector-collapsed");
    // The native menu owns Cmd-I. A webview key event cannot also toggle it.
    fireEvent.keyDown(window, { key: "i", metaKey: true });
    expect(surfaces).toHaveClass("inspector-collapsed");
    act(() => nativeMenuCommand("view.inspector"));
    expect(surfaces).not.toHaveClass("inspector-collapsed");
    act(() => nativeMenuCommand("view.stage.loads"));
    expect(rail("loads")).toHaveAttribute("aria-current", "page");
    act(() => nativeMenuCommand("view.stage.review"));
    expect(rail("loads")).toHaveAttribute("aria-current", "page");
    act(() => nativeMenuCommand("view.theme.dark"));
    act(() => nativeMenuCommand("view.density.compact"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("sync_native_shell_state", {
      state: expect.objectContaining({ stage: "loads", view: "table", theme: "dark", density: "compact", inspectorOpen: false })
    }));
  });
});


it("an explicit native Inspector choice during routing supersedes automatic restoration", async () => {
  const surfaces = await renderShell();
  fireEvent.click(screen.getByTestId("command-node"));
  expect(surfaces).not.toHaveClass("inspector-collapsed");
  act(() => nativeMenuCommand("view.inspector"));
  expect(surfaces).toHaveClass("inspector-collapsed");
  act(() => nativeMenuCommand("view.inspector"));
  expect(surfaces).not.toHaveClass("inspector-collapsed");
  fireEvent.click(screen.getByTestId("workspace-select"));
  expect(surfaces).not.toHaveClass("inspector-collapsed");
});


it("reissuing a native routing command reveals its closed inspector home", async () => {
  const surfaces = await renderShell(1024);
  fireEvent.click(screen.getByTestId("command-node"));
  fireEvent.click(screen.getByTestId("toggle-inspector"));
  expect(surfaces).toHaveClass("inspector-collapsed");
  act(() => nativeMenuCommand("insert.node"));
  expect(surfaces).not.toHaveClass("inspector-collapsed");
  expect(screen.getByTestId("command-node")).toHaveAttribute("aria-pressed", "true");
});
