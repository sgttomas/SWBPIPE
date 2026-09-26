import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import { SolvePanel } from "../solve/SolvePanel";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { createLocalProject, listLocalProjects, openLocalProject } from "../../services/projectService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import type { EditorOperationIntent, MechanicsResult, PreviewModel } from "../../types";
import connected from "../../../../../fixtures/product_preview/load_reference/connected.request.json";
import pressure from "../../../../../fixtures/product_preview/load_reference/pressure.request.json";
import eigenMotion from "../../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json";
import connectedRaw from "../../../../../fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json";

// T1 WP3 addendum. The committed 0.4.0 request models are used unmodified
// (headlessly authored: their load cases may have no label, kind or status and
// the model may have no desktop `diagnostics` array). Invented pass-through
// values below are marked as such.
afterEach(cleanup);
const FIXTURES = { connected: connected.model, pressure: pressure.model, eigen_motion: eigenMotion.model } as const;
type Name = keyof typeof FIXTURES;
const committed = (name: Name) => structuredClone(FIXTURES[name]) as unknown as PreviewModel;
const value = (label: string) => (screen.getByLabelText(label) as HTMLInputElement | HTMLSelectElement).value;

function renderPanels(model: PreviewModel, caseId: string, onQueueIntent: (intent: EditorOperationIntent) => void = vi.fn()) {
  return render(<>
    <LoadCaseManagerPanel model={model} selection={{ type: "load", id: caseId }} onQueueIntent={onQueueIntent} onSelect={vi.fn()} />
    <SolvePanel analysisRun={null} model={model} result={null} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive" onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />
  </>);
}

describe("committed 0.4.0 request models render unmodified in the Load Case Manager and SolvePanel", () => {
  for (const name of Object.keys(FIXTURES) as Name[]) {
    it(`${name}: renders every case, shows absent metadata as not set, changes no byte`, () => {
      const model = committed(name);
      const bytes = JSON.stringify(model);
      for (const loadCase of model.load_cases) {
        const view = renderPanels(model, loadCase.id);
        const row = screen.getByTestId(`load-manager-case-${loadCase.id}`);
        const raw = loadCase as unknown as Record<string, unknown>;
        // Display only: an absent label shows the id; absent kind/status show "not set".
        expect(within(row).getByText(typeof raw.label === "string" && raw.label ? raw.label : loadCase.id, { selector: "strong" })).toBeInTheDocument();
        expect(row).toHaveTextContent(`${loadCase.id}; ${typeof raw.kind === "string" ? raw.kind : "not set"}; ${typeof raw.status === "string" ? raw.status : "not set"};`);
        expect(screen.getByTestId("load-manager-selected-case")).toHaveTextContent(`current=${typeof raw.status === "string" ? raw.status : "not set"}`);
        expect(screen.getByTestId("load-state-analysis-state")).toHaveTextContent(`Analysis state — ${typeof raw.label === "string" && raw.label ? raw.label : loadCase.id}`);
        expect(screen.getByTestId("readiness-diagnostics")).toBeInTheDocument();
        view.unmount();
      }
      expect(JSON.stringify(model)).toBe(bytes);
      expect(Object.hasOwn(model, "diagnostics")).toBe(Object.hasOwn(FIXTURES[name], "diagnostics"));
      for (const loadCase of model.load_cases) for (const key of ["label", "kind", "status"])
        expect(Object.hasOwn(loadCase, key)).toBe(Object.hasOwn(FIXTURES[name].load_cases.find(c => c.id === loadCase.id)!, key));
    });
  }

  it("the fields show the authored values of the committed models", () => {
    renderPanels(committed("connected"), "case:cold");
    expect(value("Reference configuration 1 ID")).toBe("reference:installed");
    expect(value("Reference configuration 1 member 2 fit")).toBe("natural_length_change");
    expect(value("Reference configuration 1 member 2 natural length change value")).toBe("-1");
    expect(value("Reference configuration 1 member 2 natural length change unit")).toBe("mm");
    expect(value("Element state 1 material point")).toBe("point:cold");
    expect(value("Element state 1 expansion law")).toBe("law:secant");
    expect(value("Element state 1 operating temperature value")).toBe("20");
    expect(value("Support state 1 motion 2 DOF")).toBe("RZ");
    expect(value("Support state 1 motion 2 displacement value")).toBe("0.001");
    expect(value("Support state 1 motion 2 displacement unit")).toBe("rad");
    expect(value("Load source 1 primitive")).toBe("load:transverse:case:cold");
    expect(value("Load source 1 factor (dimensionless)")).toBe("1");
    cleanup();
    renderPanels(committed("eigen_motion"), "case:join");
    expect(value("Reference configuration 1 member 1 basis")).toBe("direct_strain_reference");
    expect(value("Reference configuration 1 member 1 fit strain value")).toBe("0.00004");
    expect(value("Element state 1 thermal state")).toBe("explicit_interval_strain");
    expect(value("Element state 1 thermal strain value")).toBe("0.00006");
    expect(value("Support state 3 motion 1 displacement value")).toBe("0.00005");
    expect(value("Support state 3 motion 1 displacement unit")).toBe("m");
    cleanup();
    const pressureModel = committed("pressure");
    render(<MaterialTemperatureForm model={pressureModel} selection={{ type: "material", id: pressureModel.materials![0].id }} onQueueIntent={vi.fn()} />);
    const law = pressureModel.materials![0].expansion_laws![0];
    expect(value("Expansion law 1 ID")).toBe(law.id);
    expect(value("Expansion law 1 definition")).toBe(law.definition);
  });

  it("SolvePanel shows the resolved-state block on the unmodified model without a diagnostics array", () => {
    const model = committed("connected");
    const result = structuredClone(connectedRaw) as unknown as MechanicsResult;
    const bytes = JSON.stringify(model);
    expect(Object.hasOwn(model, "diagnostics")).toBe(false);
    render(<SolvePanel analysisRun={null} model={model} result={result} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive" onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
    expect(screen.getByTestId("readiness-diagnostics")).toHaveTextContent(`${result.diagnostics.length} diagnostic`);
    expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("ordinary load-reference-1");
    expect(JSON.stringify(model)).toBe(bytes);
  });

  it("a delete intent for a case without label, kind or status matches the engine's before-value", async () => {
    const model = committed("connected");
    const queue = vi.fn();
    renderPanels(model, "case:hot", queue);
    fireEvent.click(screen.getByTestId("queue-delete-load-case-intent"));
    expect(queue).toHaveBeenCalledOnce();
    const intent = queue.mock.calls[0][0] as EditorOperationIntent;
    expect(intent.change.before).toBe("case:hot; TBD; TBD; TBD; primitives=2");
    const outcome = await validateModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.map(d => d.code)).not.toContain("OP-STALE-BEFORE-VALUE");
    expect(JSON.stringify(model)).toBe(JSON.stringify(FIXTURES.connected));
  });
});

/** Every authored 0.4.0 key the plain fields cannot edit yet, with an invented value. */
const PASS_THROUGH = {
  "element_states[0].analysis_basis_override": { reason: "invented_wp3_pass_through", provenance: "invented_wp3_pass_through" },
  "element_states[0].mass_state_ref": "invented:wp3-mass-state",
  "support_states[0].base_motion": [{ dof: "UZ", value: { value: 0.25, unit: "mm" }, meaning: "absolute_reference_displacement" }],
  "support_states[0].device_reference": { kind: "unloaded_reference", reference_position: { value: 1.5, unit: "mm" } },
  "support_states[1].participation": { kind: "locked_equivalent_support", components: [
    { dof: "UX", position_source: { kind: "entered", value: { value: 0.1, unit: "mm" } } },
    { dof: "UY", position_source: { kind: "predecessor_value", case_ref: "case:invented-predecessor", state_hash: "sha256:invented-wp3", support_ref: "spring", dof: "UY" } }
  ] }
} as const;

const PASS_CASE = "case:join";
function withPassThrough(): PreviewModel {
  const model = committed("eigen_motion");
  const state = model.load_cases.find(c => c.id === PASS_CASE)!.analysis_state as unknown as Record<string, Array<Record<string, unknown>>>;
  for (const [path, item] of Object.entries(PASS_THROUGH)) {
    const [, list, index, key] = /^(\w+)\[(\d+)\]\.(\w+)$/.exec(path)!;
    state[list][Number(index)][key] = structuredClone(item);
  }
  return model;
}
function passThroughValues(model: PreviewModel): string[] {
  const state = model.load_cases.find(c => c.id === PASS_CASE)!.analysis_state as unknown as Record<string, Array<Record<string, unknown>>>;
  return Object.keys(PASS_THROUGH).map(path => {
    const [, list, index, key] = /^(\w+)\[(\d+)\]\.(\w+)$/.exec(path)!;
    return JSON.stringify(state[list][Number(index)][key]);
  });
}

describe("workspace session on a committed 0.4.0 model opened through the browser open route", () => {
  async function openInSession(model: PreviewModel) {
    // The real browser persistence route: create the local browser snapshot,
    // then the session's own Open local handler restores it.
    await createLocalProject(model);
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    await act(async () => { await hook.result.current.project.handleOpenProject(); });
    await waitFor(() => expect(hook.result.current.model.model?.project.id).toBe(model.project.id));
    return hook;
  }

  // The two ordinary request models lack the desktop-only keys (data_boundary,
  // diagnostics, sections, labels) that the workspace session reads outside
  // this write set; see RETURN addendum. The joined request models are
  // desktop-shaped and open in the whole session.
  for (const name of ["eigen_motion"] as const) {
    it(`${name}: opens byte-exact, renders the fields, and an unrelated edit plus Undo restores every byte`, async () => {
      const model = committed(name);
      const bytes = JSON.stringify(model);
      const hook = await openInSession(model);
      expect(JSON.stringify(hook.result.current.model.model)).toBe(bytes);
      const session = hook.result.current;
      const caseId = model.load_cases[0].id;
      const queue = vi.fn();
      renderPanels(session.model.model!, caseId, queue);
      expect(value("Reference configuration 1 ID")).toBe(model.reference_configurations![0].id);
      expect(JSON.stringify(hook.result.current.model.model)).toBe(bytes);
      // Unrelated edit through the fields: the analysis-state provenance.
      fireEvent.change(screen.getByLabelText("Analysis state provenance"), { target: { value: "invented_wp3_unrelated_edit" } });
      fireEvent.click(screen.getByRole("button", { name: "Queue analysis state" }));
      await waitFor(() => expect(queue).toHaveBeenCalledOnce(), { timeout: 20_000 });
      await act(async () => { expect(await hook.result.current.operations.handleApplyIntent(queue.mock.calls[0][0])).toBe(true); });
      await waitFor(() => expect(JSON.stringify(hook.result.current.model.model)).not.toBe(bytes));
      expect(hook.result.current.model.model!.load_cases[0].analysis_state!.provenance).toBe("invented_wp3_unrelated_edit");
      act(() => hook.result.current.operations.handleUndoSessionModelEdit());
      await waitFor(() => expect(JSON.stringify(hook.result.current.model.model)).toBe(bytes));
      hook.unmount();
    });
  }

  it("every authored key the fields cannot edit passes through an edit byte-exactly, and Undo restores the document", async () => {
    const model = withPassThrough();
    const bytes = JSON.stringify(model);
    const before = passThroughValues(model);
    const hook = await openInSession(model);
    expect(JSON.stringify(hook.result.current.model.model)).toBe(bytes);
    const queue = vi.fn();
    renderPanels(hook.result.current.model.model!, PASS_CASE, queue);
    expect(screen.getByTestId("load-state-analysis-state")).toHaveTextContent("Retained as authored (not edited here): analysis_basis_override, mass_state_ref.");
    expect(screen.getByTestId("load-state-analysis-state")).toHaveTextContent("Retained as authored (not edited here): base_motion, device_reference.");
    expect(screen.getByTestId("load-state-analysis-state")).toHaveTextContent("Retained as authored (not edited here): participation.components.");
    // An edit elsewhere in the same record: a support motion value and the load-source factor.
    fireEvent.change(screen.getByLabelText("Support state 1 motion 1 displacement value"), { target: { value: "0.75" } });
    fireEvent.change(screen.getByLabelText("Load source 1 factor (dimensionless)"), { target: { value: "2.5" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue analysis state" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce(), { timeout: 20_000 });
    await act(async () => { expect(await hook.result.current.operations.handleApplyIntent(queue.mock.calls[0][0])).toBe(true); });
    await waitFor(() => expect(hook.result.current.model.model!.load_cases.find(c => c.id === PASS_CASE)!.analysis_state!.load_sources[0].factor).toBe(2.5));
    const edited = hook.result.current.model.model!;
    // Value-exact: the wasm operation route returns the applied model with
    // sorted keys (WP2 RETURN §6.3), so each subtree is compared by its RFC 8785
    // canonical bytes. Undo below restores the document byte-for-byte.
    const canonical = async (texts: string[]) => Promise.all(texts.map(text => canonicalJsonString(JSON.parse(text))));
    expect(await canonical(passThroughValues(edited))).toEqual(await canonical(before));
    expect(edited.load_cases.find(c => c.id === PASS_CASE)!.analysis_state!.support_states[0].boundary_motion![0].value).toEqual({ value: 0.75, unit: "mm" });
    act(() => hook.result.current.operations.handleUndoSessionModelEdit());
    await waitFor(() => expect(JSON.stringify(hook.result.current.model.model)).toBe(bytes));
    hook.unmount();
  });
});

describe("product-shaped documents: the workspace crash writes no stored state (routed to UI-SUCCESSOR)", () => {
  for (const name of ["connected", "pressure"] as const) {
    it(`${name}: the open route fails in the model index and the stored project is unchanged`, async () => {
      const model = committed(name);
      const bytes = JSON.stringify(model);
      await createLocalProject(model);
      const stored = JSON.stringify(await openLocalProject());
      const listed = JSON.stringify(await listLocalProjects());
      const hook = renderHook(() => useWorkspaceSession());
      await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
      let failure: unknown = null;
      try { await act(async () => { await hook.result.current.project.handleOpenProject(); }); }
      catch (error) { failure = error; }
      // Pre-existing, version-independent: the model index reads model.diagnostics.
      expect(failure).toBeInstanceOf(TypeError);
      expect(String((failure as Error).stack)).toMatch(/buildModelIndex \(.*features\/workspace\/modelIndex\.ts:259:/);
      // Stored state after the crash: same envelope bytes (model, result, run,
      // intents, ledger) and the same persistence timestamps; nothing was saved.
      const after = await openLocalProject();
      expect(JSON.stringify(after)).toBe(stored);
      expect(JSON.stringify(after!.model)).toBe(bytes);
      expect(after!.mechanics_result).toBeNull();
      expect(after!.analysis_run).toBeNull();
      expect(after!.editor_intents).toEqual([]);
      expect(after!.model_migration_ledger).toEqual(JSON.parse(stored).model_migration_ledger);
      expect(JSON.stringify(await listLocalProjects())).toBe(listed);
      expect(JSON.stringify(model)).toBe(bytes);
    });
  }
});
