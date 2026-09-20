import { act, render, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { makeRichIntent } from "../rich-authoring/formSupport";
import { buildGuardedRemoval } from "../toolkit/GuardedRemoval";
import { entityKey } from "./selectionState";
import {
  SHELL_STAGES,
  SHELL_VIEWS,
  rememberStageView,
  sectionForStage,
  shellLocation,
  stageViews,
  viewForStage
} from "./shellLayout";
import type { ShellStage, ShellView } from "./shellLayout";
import { useWorkspaceSession } from "./workspaceSession";
import * as hashService from "../../services/hashService";
import { createLocalProject } from "../../services/projectService";
import type { WorkspaceSession } from "./workspaceSession";
import { WorkspaceSessionProvider, useSessionChrome, useWorkspaceSessionContext } from "./WorkspaceSessionContext";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  window.localStorage.clear();
});

const SLICES = ["model", "selection", "results", "operations", "project", "chrome"] as const;

// Every setter the session hands to the view. The first eleven are the base's;
// the last is the one chrome setter slice B3 adds (the per-stage view memory).
const SESSION_SETTERS: Record<(typeof SLICES)[number], string[]> = {
  model: [],
  selection: ["setHiddenEntityKeys", "setIsolationSelectionKeys"],
  results: ["setSolverMode", "setReportPackagePrivateIntent"],
  operations: [],
  project: [],
  chrome: [
    "setUiPreferences",
    "setActiveSection",
    "setOpenMenu",
    "setOperationTab",
    "setReviewDetailsOpen",
    "setAuditDrawerOpen",
    "setIssuesDrawerOpen",
    "setStageViewMemory"
  ]
};

async function readySession() {
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  return hook;
}

function setterNames(slice: Record<string, unknown>): string[] {
  return Object.keys(slice).filter((key) => /^set[A-Z]/.test(key));
}

describe("the session's key set", () => {
  it("has exactly six slices and exposes exactly the named setters", async () => {
    const { result } = await readySession();
    const session = result.current;
    expect(Object.keys(session)).toEqual([...SLICES]);
    for (const slice of SLICES) {
      expect(setterNames(session[slice] as Record<string, unknown>), slice).toEqual(SESSION_SETTERS[slice]);
    }
  });

  it("lets no model, results, history, undo, redo, operation-outcome or project setter leave the session", async () => {
    const { result } = await readySession();
    const session = result.current;
    expect(setterNames(session.model)).toEqual([]);
    expect(setterNames(session.operations)).toEqual([]);
    expect(setterNames(session.project)).toEqual([]);
    const exposed = SLICES.flatMap((slice) => setterNames(session[slice] as Record<string, unknown>));
    const forbidden =
      /^set(Model|Knowledge|ModelHash|ModelAssignment|UiModelRevision|ProjectSessionGeneration|Result|HistoricalRun|AnalysisRun|InputManifest|RuleCheck|Comparison|Proposal|SelectedReviewTarget|SolveJob|SolveProof|Running|ReportPackage(Busy|Redaction|Route)|EditorIntents|RetainedReviewContext|OperationOutcomes|AppliedOperations|UndoStack|RedoStack|QueuedBatches|Batch|RequestEpoch|DirectDraft|OperationBusy|OperationMessage|OperationEngineStatus|Storage|Project|ModelDocumentMigration|ModelMigrationLedger|Selection|OrderedSelection)/;
    expect(exposed.filter((name) => forbidden.test(name))).toEqual([]);
    // The two results setters that are exposed are a solver-mode choice and a privacy intent, not results.
    expect(setterNames(session.results)).toEqual(["setSolverMode", "setReportPackagePrivateIntent"]);
  });
});

/**
 * The cells a stage or view switch must leave identical, by reference.
 * `ruleCheckRunBasis` and `dormantOutputBasis` are rebuilt on every render from
 * cells listed here (the generation, the revision, the result, the manifest),
 * so they are held through those.
 */
function spine(session: WorkspaceSession) {
  const { model, results, operations, project, selection } = session;
  return {
    model: model.model,
    knowledge: model.knowledge,
    modelHash: model.modelHash,
    modelAssignment: model.modelAssignment,
    activeModelIndex: model.activeModelIndex,
    projectSessionGeneration: model.projectSessionGeneration,
    uiModelRevision: model.uiModelRevision,
    modelRevision: model.modelRevision.current,
    result: results.result,
    currentSolvedResult: results.currentSolvedResult,
    historicalRun: results.historicalRun,
    analysisRun: results.analysisRun,
    inputManifest: results.inputManifest,
    ruleCheckAggregate: results.ruleCheckAggregate,
    solveJob: results.solveJob,
    solveProof: results.solveProof,
    running: results.running,
    comparison: results.comparison,
    editorIntents: operations.editorIntents,
    operationOutcomes: operations.operationOutcomes,
    appliedOperations: operations.appliedOperations,
    undoStack: operations.undoStack,
    redoStack: operations.redoStack,
    queuedBatches: operations.queuedBatches,
    batchOutcomes: operations.batchOutcomes,
    batchReceipts: operations.batchReceipts,
    requestEpoch: operations.requestEpoch,
    operationMessage: operations.operationMessage,
    projectSummary: project.projectSummary,
    projectIndex: project.projectIndex,
    projectMessage: project.projectMessage,
    projectEnvelopeHash: project.projectEnvelopeHash,
    modelMigrationLedger: project.modelMigrationLedger,
    selection: selection.selection,
    orderedSelection: selection.orderedSelection
  };
}

function expectSameSpine(before: ReturnType<typeof spine>, after: ReturnType<typeof spine>, step: string) {
  for (const key of Object.keys(before) as Array<keyof typeof before>) {
    expect(Object.is(after[key], before[key]), `${step}: ${key}`).toBe(true);
  }
}

describe("a stage or view switch is a presentation change", () => {
  async function switchEveryStageAndView(result: { current: WorkspaceSession }) {
    const before = spine(result.current);
    let visited = 0;
    for (const stage of [...SHELL_STAGES, "model" as ShellStage]) {
      act(() => result.current.chrome.setActiveSection(sectionForStage(stage, result.current.chrome.stageSurface)));
      const location = shellLocation(result.current.chrome.activeSection, result.current.chrome.stageSurface);
      expect(location.stage).toBe(stage);
      expect(location.page).toBeNull();
      expectSameSpine(before, spine(result.current), `stage ${stage}`);
      for (const view of stageViews(stage)) {
        act(() => result.current.chrome.setStageViewMemory((memory) => rememberStageView(memory, stage, view)));
        expect(viewForStage(result.current.chrome.stageViewMemory, stage)).toBe(view);
        expectSameSpine(before, spine(result.current), `stage ${stage}, view ${view}`);
        visited += 1;
      }
    }
    expect(visited).toBe(3 + 3 + 3 + 1 + 3);
    // The per-stage memory through the session: each stage kept the view it was last left in.
    const memory = result.current.chrome.stageViewMemory;
    expect(SHELL_STAGES.map((stage) => viewForStage(memory, stage))).toEqual(["both", "both", "both", "table"]);
  }

  it("leaves the model, the revision, the solve-input basis and a Current solved run identical", async () => {
    const { result } = await readySession();
    await act(async () => {
      await result.current.results.handleRun();
    });
    await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
    expect(result.current.results.solveProof).not.toBeNull();
    expect(result.current.results.inputManifest).not.toBeNull();
    expect(result.current.results.analysisRun).not.toBeNull();
    await switchEveryStageAndView(result);
  });

  it("leaves the undo stack, the applied operation and the run made after it identical", async () => {
    const { result } = await readySession();
    const basis = result.current.model.model!;
    const intent = makeRichIntent(
      { object_type: "Node", ref: basis.nodes[0].id },
      "set_field",
      "position.x",
      String(basis.nodes[0].position.x),
      basis.nodes[0].position.x + 0.25,
      "X coordinate"
    );
    intent.change.unit = basis.project.units.length;
    intent.change.dimension = "length";
    let applied = false;
    await act(async () => {
      applied = await result.current.operations.handleApplyIntent(intent);
    });
    expect(applied).toBe(true);
    await waitFor(() => expect(result.current.operations.undoStack.length).toBe(1));
    await waitFor(() => expect(result.current.model.modelAssignment?.status).toBe("committed"));
    expect(result.current.operations.appliedOperations.length).toBe(1);
    await act(async () => {
      await result.current.results.handleRun();
    });
    // The browser fixture's run of an edited model completes without solving; it is still a run to keep.
    await waitFor(() => expect(result.current.results.solveJob.state).toBe("completed"));
    expect(result.current.results.result).not.toBeNull();
    await switchEveryStageAndView(result);
    expect(result.current.operations.undoStack.length).toBe(1);
  });

  it("opens a page over the stage it found and returns to it, touching nothing else", async () => {
    const { result } = await readySession();
    const before = spine(result.current);
    act(() => result.current.chrome.setActiveSection("evidence"));
    await waitFor(() => expect(result.current.chrome.stageSurface).toBe("evidence"));
    act(() => result.current.chrome.setActiveSection("project"));
    let location = shellLocation(result.current.chrome.activeSection, result.current.chrome.stageSurface);
    expect(location).toMatchObject({ stage: "results", tab: "evidence", page: "project", pageKind: "over-stage" });
    act(() => result.current.chrome.setActiveSection("libraries"));
    location = shellLocation(result.current.chrome.activeSection, result.current.chrome.stageSurface);
    expect(location).toMatchObject({ stage: "results", tab: "evidence", page: "libraries", pageKind: "rail" });
    expect(result.current.chrome.stageSurface).toBe("evidence");
    expectSameSpine(before, spine(result.current), "pages");
  });

  it("starts every session from the first-open views", async () => {
    const { result } = await readySession();
    const views: ShellView[] = SHELL_STAGES.map((stage) => viewForStage(result.current.chrome.stageViewMemory, stage));
    expect(views).toEqual(["both", "table", "both", "table"]);
    expect(SHELL_VIEWS).toEqual(["table", "model", "both"]);
  });
});

describe("WorkspaceSessionContext", () => {
  it("hands a component the session's slices", async () => {
    const { result } = await readySession();
    const seen: { chrome?: WorkspaceSession["chrome"]; session?: WorkspaceSession } = {};
    function Probe() {
      seen.chrome = useSessionChrome();
      seen.session = useWorkspaceSessionContext();
      return null;
    }
    render(
      <WorkspaceSessionProvider session={result.current}>
        <Probe />
      </WorkspaceSessionProvider>
    );
    expect(seen.session).toBe(result.current);
    expect(seen.chrome).toBe(result.current.chrome);
  });

  it("refuses a component outside the provider", () => {
    function Probe() {
      useSessionChrome();
      return null;
    }
    const quiet = vi.spyOn(console, "error").mockImplementation(() => undefined);
    expect(() => render(<Probe />)).toThrow(/WORKSPACE-SESSION-CONTEXT-ABSENT/);
    quiet.mockRestore();
  });
});


it("webview history accelerators use one checkpoint route in both runtimes and preserve text undo", async () => {
  const { result } = await readySession();
  const key = (shiftKey = false, altKey = false) => {
    const event = new KeyboardEvent("keydown", { key: "z", metaKey: true, shiftKey, altKey, cancelable: true });
    act(() => window.dispatchEvent(event));
    return event;
  };
  expect(key().defaultPrevented).toBe(false);
  expect(key(true).defaultPrevented).toBe(false);
  const basis = result.current.model.model!;
  const intent = makeRichIntent({ object_type: "Node", ref: basis.nodes[0].id }, "set_field", "position.x", String(basis.nodes[0].position.x), basis.nodes[0].position.x + 0.25, "X coordinate");
  intent.change.unit = basis.project.units.length;
  intent.change.dimension = "length";
  await act(async () => { expect(await result.current.operations.handleApplyIntent(intent)).toBe(true); });
  await waitFor(() => expect(result.current.operations.undoStack).toHaveLength(1));
  const edited = result.current.model.model!;
  expect(key(false, true).defaultPrevented).toBe(false);
  const consumed = new KeyboardEvent("keydown", { key: "z", metaKey: true, cancelable: true });
  consumed.preventDefault();
  act(() => window.dispatchEvent(consumed));
  expect(result.current.model.model).toBe(edited);
  for (const tag of ["input", "textarea", "div"]) {
    const editing = document.createElement(tag);
    if (tag === "div") editing.setAttribute("contenteditable", "true");
    document.body.append(editing);
    editing.focus();
    for (const shiftKey of [false, true]) {
      const event = new KeyboardEvent("keydown", { key: "z", metaKey: true, shiftKey, cancelable: true, bubbles: true });
      act(() => editing.dispatchEvent(event));
      expect(event.defaultPrevented).toBe(false);
      expect(result.current.model.model).toBe(edited);
    }
    editing.remove();
  }
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const nativeText = document.createElement("input");
  document.body.append(nativeText);
  nativeText.focus();
  const nativeTextUndo = new KeyboardEvent("keydown", { key: "z", metaKey: true, cancelable: true, bubbles: true });
  act(() => nativeText.dispatchEvent(nativeTextUndo));
  expect(nativeTextUndo.defaultPrevented).toBe(false);
  expect(result.current.model.model).toBe(edited);
  nativeText.remove();
  expect(key().defaultPrevented).toBe(true);
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  expect(result.current.operations.undoStack).toHaveLength(0);
  expect(result.current.operations.redoStack).toHaveLength(1);
  expect(result.current.model.model!.nodes[0].position.x).toBe(basis.nodes[0].position.x);
  expect(key(true).defaultPrevented).toBe(true);
  expect(result.current.operations.undoStack).toHaveLength(1);
  expect(result.current.operations.redoStack).toHaveLength(0);
  expect(result.current.model.model!.nodes[0].position.x).toBe(edited.nodes[0].position.x);
  expect(key(true).defaultPrevented).toBe(false);
});


it("publishes committed Open observations after the actual draft invalidation callback", async () => {
  const { result } = await readySession();
  const model = structuredClone(result.current.model.model!);
  model.project.id = "project:b3b-open-observation";
  const modelHash = await hashService.computeModelHash(model);
  const envelopeHash = await hashService.computeProjectEnvelopeHash({ model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: modelHash });
  const opened = await createLocalProject(model, [], null, null, null, null, modelHash, envelopeHash);
  let release!: () => void;
  const gate = new Promise<void>((resolve) => { release = resolve; });
  const original = hashService.computeModelHash;
  vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
    if (input === opened.model) await gate;
    return original(input);
  });
  invokeMock.mockImplementation((command: string) => command === "open_local_project" ? Promise.resolve(opened) : Promise.resolve({}));
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  try {
    let opening!: Promise<void>;
    act(() => { opening = result.current.project.handleOpenProject(); });
    await waitFor(() => expect(result.current.model.model?.project.id).toBe(opened.model.project.id));
    const epoch = result.current.operations.requestEpoch;
    act(() => result.current.operations.invalidateDirectDraftContext());
    expect(result.current.operations.requestEpoch).toBeGreaterThan(epoch);
    await act(async () => { release(); await opening; });
    expect(result.current.project.modelHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "open", persisted_value: modelHash!.value });
    expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "open", persisted_value: envelopeHash!.value });
    expect(result.current.project.projectBusy).toBe(false);
  } finally { release(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; }
});


describe("C3 shell visibility projection", () => {
  it("keeps isolation separate from Hide through selection and stage/view changes without model or history edits", async () => {
    const { result } = await readySession();
    const model = result.current.model.model!;
    const beforeHash = result.current.model.modelHash;
    const beforeUndo = result.current.operations.undoStack;
    const beforeResult = result.current.results.result;
    const [first, second] = model.pipe_segments;
    const firstKey = entityKey({ type: "pipe", id: first.id });
    const secondKey = entityKey({ type: "pipe", id: second.id });
    act(() => result.current.selection.setIsolationSelectionKeys(new Set([firstKey])));
    expect(result.current.selection.isolationActive).toBe(true);
    expect(result.current.selection.effectiveHiddenKeys.size).toBe(0);
    expect(result.current.selection.hiddenCount).toBe(0);
    expect(result.current.selection.dimmedKeys.has(firstKey)).toBe(false);
    expect(result.current.selection.dimmedKeys.has(secondKey)).toBe(true);
    act(() => result.current.selection.handleSelectEntity({ type: "pipe", id: second.id }));
    act(() => result.current.chrome.setActiveSection("loads"));
    act(() => result.current.chrome.setStageViewMemory((memory) => rememberStageView(memory, "loads", "model")));
    expect(result.current.selection.isolationSelectionKeys).toEqual(new Set([firstKey]));
    act(() => result.current.selection.setHiddenEntityKeys(new Set([secondKey])));
    expect(result.current.selection.hiddenCount).toBe(1);
    expect(result.current.selection.effectiveHiddenKeys.has(secondKey)).toBe(true);
    expect(result.current.selection.dimmedKeys.has(secondKey)).toBe(false);
    act(() => result.current.selection.handleSelectEntity({ type: "pipe", id: first.id }));
    expect(result.current.selection.hiddenCount).toBe(1);
    expect(result.current.model.model).toBe(model);
    expect(result.current.model.modelHash).toBe(beforeHash);
    expect(result.current.operations.undoStack).toBe(beforeUndo);
    expect(result.current.results.result).toBe(beforeResult);
    act(() => result.current.selection.handleClearVisibility());
    expect(result.current.selection.isolationSelectionKeys).toBeNull();
    expect(result.current.selection.isolationActive).toBe(false);
    expect(result.current.selection.hiddenCount).toBe(0);
    expect(result.current.selection.dimmedKeys.size).toBe(0);
  });

  it("retains active empty isolation after deletion, dims restored geometry and resets on project replacement", async () => {
    const { result } = await readySession();
    const component = result.current.model.model!.components[0];
    const key = entityKey({ type: "component", id: component.id });
    act(() => result.current.selection.setIsolationSelectionKeys(new Set([key])));
    const removal = buildGuardedRemoval({ type: "component", id: component.id }, await hashService.canonicalJsonString(component));
    await act(async () => { expect(await result.current.operations.handleApplyIntent(removal)).toBe(true); });
    await waitFor(() => expect(result.current.selection.isolationSelectionKeys).toEqual(new Set()));
    expect(result.current.selection.isolationActive).toBe(true);
    expect(result.current.selection.hiddenCount).toBe(0);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    await waitFor(() => expect(result.current.model.activeModelIndex!.entities.has(key)).toBe(true));
    expect(result.current.selection.isolationSelectionKeys).toEqual(new Set());
    expect(result.current.selection.dimmedKeys.has(key)).toBe(true);
    act(() => result.current.selection.setHiddenEntityKeys(new Set([key])));
    await act(async () => { await result.current.project.handleCreateBlankProject(); });
    expect(result.current.selection.isolationSelectionKeys).toBeNull();
    expect(result.current.selection.hiddenEntityKeys.size).toBe(0);
    expect(result.current.selection.hiddenCount).toBe(0);
    expect(result.current.selection.isolationActive).toBe(false);
  });
});
