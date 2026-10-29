import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { LABELS_DO_NOT_CHANGE_STATE, MODEL_STATE_STORE_UNAVAILABLE, type ModelStateSummary, type StateLabels } from "../../services/modelStateService";
import type { StoredProjectEntry } from "../../services/projectService";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

// W-12 S1 (DEL-14-01 Design `desktop-states-runs-comparison.md` §2): the
// session's Save Model State…, Open State, label edit and delete flows, against
// a fake of the native store's commands. The store's own checks (hashes,
// immutability, blockers, transactions) are the Rust tests in model_states.rs.

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO = demoModel as unknown as PreviewModel;
const BASE = "sha256:stored-base-1";

function setTauriRuntime(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

type SaveRequest = {
  project_id: string; expected_base: string; model_canonical: string; model_hash: string;
  state_kind: string; parent_state_id: string | null; labels: StateLabels;
};

/** A fake of the native store: one project row and its states. */
type RowsOptions = {
  /** Stored project rows (id -> envelope hash), checked as the native store checks bases. */
  rows: Map<string, string>;
  /** Ids whose next in-place save is refused once, leaving the row unchanged. */
  refuseOnce?: Set<string>;
  /** `check_model_library_refs`' answer. */
  libraryFindings?: unknown[];
  /** When set, the next Save Local Project answers only once this settles,
   * judged against the rows as they are then. */
  gate?: Promise<void> | null;
};

function rowEntry(projectId: string, base: string): StoredProjectEntry {
  return { project_id: projectId, project_name: "Stored invented project", storage_mode: "local_sqlite",
    created_at_unix: 1760000000, updated_at_unix: 1760086400, revision: `rev:${base}`, saved_state_count: 0 };
}

function nativeStore(options: { projectStored?: boolean; find?: StoredProjectEntry | null; refuseProjectSave?: string; storedDemo?: StoredProjectEntry; rows?: RowsOptions } = {}) {
  const rows = options.rows;
  let rowSequence = 0;
  const store = {
    projectStored: options.projectStored ?? false,
    storedProjects: new Set<string>(),
    states: [] as ModelStateSummary[],
    snapshots: new Map<string, string>(),
    sequence: 0
  };
  const answer = (command: string, args: Record<string, unknown>): unknown => {
    switch (command) {
      case "sync_native_shell_state":
        return null;
      case "check_model_library_refs":
        return rows?.libraryFindings ?? [];
      case "open_local_project": {
        const base = rows!.rows.get(DEMO.project.id)!;
        return {
          summary: { project_id: DEMO.project.id, project_name: DEMO.project.name, message: "Opened local SQLite project snapshot." },
          model: structuredClone(DEMO), editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
          analysis_run: null, model_hash: null, project_envelope_hash: { value: base }, model_document_version: null
        } as unknown as LocalProjectEnvelope;
      }
      case "find_local_project":
        if (rows) return rows.rows.has(args.projectId as string) ? rowEntry(args.projectId as string, rows.rows.get(args.projectId as string)!) : null;
        if (options.storedDemo) return args.projectId === DEMO.project.id ? options.storedDemo : null;
        return options.find ?? null;
      case "save_local_project": {
        if (options.refuseProjectSave) throw options.refuseProjectSave;
        const request = args.request as { model: PreviewModel; model_hash: unknown; confirmed_revision?: string | null; expected_base?: string | null };
        if (rows?.gate) {
          const gate = rows.gate;
          rows.gate = null;
          return gate.then(() => answer(command, args));
        }
        if (rows) {
          const id = request.model.project.id;
          const stored = rows.rows.get(id);
          if (stored !== undefined) {
            if (rows.refuseOnce?.delete(id)) throw `PROJECT-BASE-MOVED: project \`${id}\` ("Stored invented project", last saved at unix 1760086400) changed after this session opened or saved it; replacing it needs the user's confirmation.`;
            const confirmed = request.confirmed_revision === `rev:${stored}`;
            if (!confirmed && request.expected_base !== stored) {
              throw request.expected_base == null && request.confirmed_revision == null
                ? `PROJECT-ID-EXISTS: project \`${id}\` ("Stored invented project", last saved at unix 1760086400) is already stored; replacing it needs the user's confirmation.`
                : `PROJECT-BASE-MOVED: project \`${id}\` ("Stored invented project", last saved at unix 1760086400) changed after this session opened or saved it; replacing it needs the user's confirmation.`;
            }
          }
          const base = `sha256:${id}#${++rowSequence}`;
          rows.rows.set(id, base);
          return {
            summary: { project_id: id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
            model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
            analysis_run: null, model_hash: request.model_hash, project_envelope_hash: { value: base }, model_document_version: null
          } as unknown as LocalProjectEnvelope;
        }
        // A stored demo row: writing the demo's id needs the Replace the prompt showed.
        if (options.storedDemo && request.model.project.id === DEMO.project.id && request.confirmed_revision !== options.storedDemo.revision) {
          throw `PROJECT-ID-EXISTS: project \`${DEMO.project.id}\` ("${options.storedDemo.project_name}", last saved at unix ${options.storedDemo.updated_at_unix}) is already stored; replacing it needs the user's confirmation.`;
        }
        store.projectStored = true;
        store.storedProjects.add(request.model.project.id);
        return {
          summary: { project_id: request.model.project.id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
          model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
          analysis_run: null, model_hash: request.model_hash, project_envelope_hash: { value: BASE }, model_document_version: null
        } as unknown as LocalProjectEnvelope;
      }
      case "save_model_state": {
        const request = args.request as SaveRequest;
        if (rows) {
          if (!rows.rows.has(request.project_id)) throw `MODEL-STATE-PROJECT-MISSING: project \`${request.project_id}\` has no stored local project; save the local project first.`;
          if (request.expected_base !== rows.rows.get(request.project_id)) throw `PROJECT-BASE-MOVED: project \`${request.project_id}\` changed after this session opened or saved it; replacing it needs the user's confirmation.`;
        } else if (!store.projectStored || (options.storedDemo && !store.storedProjects.has(request.project_id))) throw `MODEL-STATE-PROJECT-MISSING: project \`${request.project_id}\` has no stored local project; save the local project first.`;
        if (!rows && request.expected_base !== BASE) throw `PROJECT-BASE-MOVED: project \`${request.project_id}\` changed after this session opened or saved it.`;
        const existing = store.states.find((state) => state.model_hash === request.model_hash);
        if (existing) return { outcome: "already_saved", state: existing, message: `This model is already saved as state "${existing.state_name}" (${existing.state_id}); nothing was written.` };
        const created_at = `2026-10-24T10:00:0${++store.sequence}.000Z`;
        const state: ModelStateSummary = {
          ...request.labels, project_id: request.project_id, state_id: `state:${String(store.sequence).padStart(32, "0")}`,
          state_kind: request.state_kind as ModelStateSummary["state_kind"], created_at, parent_state_id: request.parent_state_id,
          model_hash: request.model_hash, record_hash: `record-${store.sequence}`, labels_updated_at: created_at, child_state_ids: [],
          analysis_status: ["HUMAN_REVIEW_REQUIRED"]
        };
        store.states = [state, ...store.states.map((other) => other.state_id === request.parent_state_id ? { ...other, child_state_ids: [...other.child_state_ids, state.state_id] } : other)];
        store.snapshots.set(state.state_id, request.model_canonical);
        return { outcome: "saved", state, message: `Saved model state "${state.state_name}" (${state.state_id}). ${LABELS_DO_NOT_CHANGE_STATE}` };
      }
      case "list_model_states":
        return store.states.filter((state) => state.project_id === args.projectId);
      case "list_analysis_runs":
        // W-12 S2: the History panel lists recorded runs with the states; none here.
        return [];
      case "read_model_state": {
        const state = store.states.find((candidate) => candidate.state_id === args.stateId)!;
        return { state, record: {}, model: JSON.parse(store.snapshots.get(state.state_id)!), model_document_version: { status: "current" } };
      }
      case "edit_model_state_labels": {
        const at = store.states.findIndex((candidate) => candidate.state_id === args.stateId);
        if (store.states[at].labels_updated_at !== args.expectedUpdatedAt) throw "MODEL-STATE-LABELS-MOVED";
        store.states[at] = { ...store.states[at], ...(args.labels as StateLabels), labels_updated_at: "2026-10-24T11:00:00.000Z" };
        return store.states[at];
      }
      case "delete_model_state": {
        const state = store.states.find((candidate) => candidate.state_id === args.stateId)!;
        if (state.child_state_ids.length) throw `MODEL-STATE-DELETE-BLOCKED: state "${state.state_name}" is a parent.`;
        store.states = store.states.filter((candidate) => candidate !== state)
          .map((other) => ({ ...other, child_state_ids: other.child_state_ids.filter((id) => id !== state.state_id) }));
        return { project_id: state.project_id, state_id: state.state_id, deleted: true, message: `Deleted model state "${state.state_name}" (${state.state_id}).` };
      }
      default:
        throw new Error(`Unexpected command ${command}`);
    }
  };
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    try { return Promise.resolve(answer(command, args ?? {})); } catch (error) { return Promise.reject(error); }
  });
  return store;
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as Record<string, unknown>);
// Read-only listings, including D5 autosave's settings read and record listing (RT-U3), are not part of the save order.
const commandOrder = () => invokeMock.mock.calls.map(([name]) => name as string).filter((name) => name !== "sync_native_shell_state" && name !== "list_model_states" && name !== "list_analysis_runs" && name !== "get_autosave_settings" && name !== "list_project_autosaves");
const historyMessage = () => screen.getByTestId("model-history-message");

async function renderApp() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(DEMO.project.id)}`);
}

function openProjectSection() {
  const section = screen.queryByTestId("workspace-section-project");
  if (!section || section.classList.contains("inactive")) act(() => nativeMenuCommand("view.section.project"));
  return screen.getByTestId("model-history-panel");
}

async function refreshHistory() {
  fireEvent.click(within(openProjectSection()).getByTestId("model-history-refresh"));
  await waitFor(() => expect(calls("list_model_states").length).toBeGreaterThan(0));
}

async function saveStateNamed(name: string) {
  const panel = openProjectSection();
  fireEvent.click(within(panel).getByTestId("model-state-save-open"));
  fireEvent.change(within(panel).getByTestId("model-state-save-name"), { target: { value: name } });
  fireEvent.change(within(panel).getByTestId("model-state-save-tags"), { target: { value: "tie-in, rev-B" } });
  fireEvent.change(within(panel).getByTestId("model-state-save-notes"), { target: { value: "Invented basis note." } });
  const before = calls("save_model_state").length;
  fireEvent.click(within(panel).getByTestId("model-state-save-submit"));
  await waitFor(() => expect(calls("save_model_state").length).toBe(before + 1));
  await waitFor(() => expect(historyMessage()).not.toHaveTextContent("saving the local project first"));
}

function openNodeGridReview() {
  const summary = screen.getByTestId("node-grid-review-disclosure");
  if (summary.getAttribute("aria-expanded") !== "true") fireEvent.click(summary);
}

function reviewEditor(cell: string): HTMLInputElement {
  const table = screen.getByTestId("engineering-table-review");
  const existing = table.querySelector<HTMLInputElement>("input");
  const split = cell.lastIndexOf("-"); const row = cell.slice(0, split); const column = cell.slice(split + 1);
  if (existing?.getAttribute("aria-label")?.startsWith(`${row} ${column.toUpperCase()}`)) return existing;
  const keep = table.querySelector<HTMLButtonElement>('[data-table-action="apply"]');
  if (keep) fireEvent.click(keep);
  fireEvent.doubleClick(screen.getByTestId(`review-cell-${cell}`));
  return table.querySelector<HTMLInputElement>("input")!;
}

function reviewValue(cell: string): string {
  const table = screen.getByTestId("engineering-table-review");
  const button = table.querySelector<HTMLButtonElement>(`[data-testid="review-cell-${cell}"]`);
  if (button) return button.textContent ?? "";
  return table.querySelector<HTMLInputElement>("input")?.value ?? "";
}

/** One applied edit of node N-100's y, through the browser operation engine;
 * `applied` is the session's applied-operation count after it. */
async function applyNodeEdit(value: string, applied: number) {
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  openNodeGridReview();
  fireEvent.change(reviewEditor("node:N-100-y"), { target: { value } });
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
  setTauriRuntime(false);
  const apply = screen.getAllByTestId(/^apply-intent-editor-intent-\d+$/).at(-1)!;
  fireEvent.click(apply);
  await waitFor(() => expect(screen.getByTestId("operation-apply-summary")).toHaveTextContent(`${applied} applied`), { timeout: 10000 });
  setTauriRuntime(true);
}

const originalY = String(DEMO.nodes.find((node) => node.id === "node:N-100")!.position.y);

describe("model states in the session (W-12 S1)", () => {
  it("the browser preview keeps no states and says so", async () => {
    await renderApp();
    expect(within(openProjectSection()).getByTestId("model-history-message")).toHaveTextContent(MODEL_STATE_STORE_UNAVAILABLE);
  });

  it("Save Model State… needs the project row: Save Local Project's flow runs first, then the canonical snapshot and its model hash are sent", async () => {
    await renderApp();
    setTauriRuntime(true);
    const store = nativeStore();
    await refreshHistory();
    expect(await screen.findByTestId("model-history-empty")).toBeInTheDocument();
    await saveStateNamed("Basis – rev A");
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`Saved model state "Basis – rev A"`));
    expect(commandOrder().filter((name) => name !== "find_local_project")).toEqual(["save_local_project", "save_model_state"]);
    const [request] = calls("save_model_state").map((args) => args.request as SaveRequest);
    expect(request.expected_base).toBe(BASE);
    expect(request.project_id).toBe(DEMO.project.id);
    expect(request.model_canonical).toBe(await canonicalJsonString(DEMO));
    expect(JSON.parse(request.model_canonical)).toEqual(DEMO);
    expect(request.model_hash).toBe((await computeModelHash(DEMO))!.value);
    expect(request.state_kind).toBe("design_snapshot");
    expect(request.parent_state_id).toBeNull();
    expect(request.labels).toEqual({ state_name: "Basis – rev A", tags: ["tie-in", "rev-B"], notes: ["Invented basis note."], assumptions: [] });
    const row = await screen.findByTestId(`model-state-${store.states[0].state_id}`);
    expect(within(row).getByTestId(`model-state-${store.states[0].state_id}-name`)).toHaveTextContent("Basis – rev A");
    expect(within(row).getByTestId(`model-state-${store.states[0].state_id}-tags`)).toHaveTextContent("Tags: tie-in, rev-B");
    // Saving the same model again names the existing state; nothing new is written.
    await saveStateNamed("Again");
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`This model is already saved as state "Basis – rev A"`));
    expect(store.states).toHaveLength(1);
    expect(calls("save_local_project")).toHaveLength(1);
  });

  it("is not saved when Save Local Project's flow is cancelled", async () => {
    await renderApp();
    setTauriRuntime(true);
    const stored: StoredProjectEntry = {
      project_id: DEMO.project.id, project_name: "Stored invented demo", storage_mode: "local_sqlite",
      created_at_unix: 1760000000, updated_at_unix: 1760086400, revision: "sha256:stored@1760086400", saved_state_count: 2
    };
    nativeStore({ find: stored, refuseProjectSave: `PROJECT-ID-EXISTS: project \`${DEMO.project.id}\` ("Stored invented demo", last saved at unix 1760086400) is already stored; replacing it needs the user's confirmation.` });
    await refreshHistory();
    const panel = openProjectSection();
    fireEvent.click(within(panel).getByTestId("model-state-save-open"));
    fireEvent.change(within(panel).getByTestId("model-state-save-name"), { target: { value: "Basis" } });
    fireEvent.click(within(panel).getByTestId("model-state-save-submit"));
    // The replace prompt says how many saved states the project id keeps.
    expect(await screen.findByTestId("replace-project-detail")).toHaveTextContent(
      `Replacing it keeps the 2 saved model states attached to project id ${DEMO.project.id}.`
    );
    fireEvent.click(screen.getByTestId("replace-project-cancel"));
    await waitFor(() => expect(historyMessage()).toHaveTextContent("Not saved: the local project was not saved, so the model state was not saved."));
    expect(calls("save_model_state")).toHaveLength(0);
  });

  describe("Save Model State… whose prerequisite project save meets a stored project id", () => {
    const STORED: StoredProjectEntry = {
      project_id: DEMO.project.id, project_name: "Stored invented demo", storage_mode: "local_sqlite",
      created_at_unix: 1760000000, updated_at_unix: 1760086400, revision: "sha256:stored@1760086400", saved_state_count: 0
    };

    async function saveThroughReplacePrompt(choice: "save-as-new" | "replace" | "cancel") {
      await renderApp();
      setTauriRuntime(true);
      const store = nativeStore({ storedDemo: STORED });
      await refreshHistory();
      const panel = openProjectSection();
      fireEvent.click(within(panel).getByTestId("model-state-save-open"));
      fireEvent.change(within(panel).getByTestId("model-state-save-name"), { target: { value: "Basis" } });
      fireEvent.click(within(panel).getByTestId("model-state-save-submit"));
      await screen.findByTestId("replace-project-prompt");
      fireEvent.click(screen.getByTestId(`replace-project-${choice === "save-as-new" ? "save-as-new" : choice}`));
      return store;
    }

    it("Save as a new project: the state is saved once, under the new project id and its base", async () => {
      const store = await saveThroughReplacePrompt("save-as-new");
      await waitFor(() => expect(historyMessage()).toHaveTextContent('Saved model state "Basis"'));
      const projectSaves = calls("save_local_project").map((args) => (args.request as { model: PreviewModel }).model.project.id);
      expect(projectSaves).toHaveLength(2);
      expect(projectSaves[0]).toBe(DEMO.project.id);
      expect(projectSaves[1]).toMatch(new RegExp(`^${DEMO.project.id}:copy-`));
      const stateSaves = calls("save_model_state").map((args) => args.request as SaveRequest);
      expect(stateSaves).toHaveLength(1);
      expect(stateSaves[0].project_id).toBe(projectSaves[1]);
      expect(stateSaves[0].expected_base).toBe(BASE);
      expect(JSON.parse(stateSaves[0].model_canonical).project.id).toBe(projectSaves[1]);
      expect(stateSaves[0].parent_state_id).toBeNull();
      expect(store.states.map((state) => state.project_id)).toEqual([projectSaves[1]]);
      // The History panel now lists the new project's state.
      expect(await screen.findByTestId(`model-state-${store.states[0].state_id}`)).toBeInTheDocument();
    });

    it("Replace: the state is saved once, under the original project id", async () => {
      await saveThroughReplacePrompt("replace");
      await waitFor(() => expect(historyMessage()).toHaveTextContent('Saved model state "Basis"'));
      expect(calls("save_local_project").map((args) => (args.request as { confirmed_revision: string | null }).confirmed_revision)).toEqual([null, STORED.revision]);
      const stateSaves = calls("save_model_state").map((args) => args.request as SaveRequest);
      expect(stateSaves.map((request) => request.project_id)).toEqual([DEMO.project.id]);
    });

    it("Cancel: neither the project nor the state is saved", async () => {
      await saveThroughReplacePrompt("cancel");
      await waitFor(() => expect(historyMessage()).toHaveTextContent("Not saved: the local project was not saved, so the model state was not saved."));
      expect(calls("save_local_project")).toHaveLength(1);
      expect(calls("save_model_state")).toHaveLength(0);
    });
  });

  describe("while another project operation owns the session", () => {
    const A = DEMO.project.id;

    it("Codex r4: a pending save keeps Open State and every model-replacing command closed; the Replace prompt is the only dialog and holds input", async () => {
      await renderApp();
      setTauriRuntime(true);
      let release!: () => void;
      const rows: RowsOptions = { rows: new Map([[A, "sha256:A-1"]]) };
      const store = nativeStore({ rows });
      act(() => nativeMenuCommand("file.open-local"));
      await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened local SQLite project snapshot."));
      await refreshHistory();
      await saveStateNamed("Basis");
      const basis = store.states[0];
      await applyNodeEdit("0.5", 1);
      // The Save Model State form is open before the operation starts.
      fireEvent.click(within(openProjectSection()).getByTestId("model-state-save-open"));
      fireEvent.change(screen.getByTestId("model-state-save-name"), { target: { value: "During the save" } });
      rows.gate = new Promise<void>((resolve) => { release = resolve; });
      const callsBefore = invokeMock.mock.calls.length;
      act(() => nativeMenuCommand("file.save-local"));
      await waitFor(() => expect(calls("save_local_project")).toHaveLength(1));

      // Open State and Save Model State are disabled with a stated reason.
      const open = screen.getByTestId(`model-state-${basis.state_id}-open`);
      expect(open).toBeDisabled();
      expect(open).toHaveAttribute("title", "Another project operation or prompt is in progress.");
      expect(screen.getByTestId("model-state-save-submit")).toBeDisabled();
      expect(screen.getByTestId("model-state-save-submit")).toHaveAttribute("title", "Another project operation or prompt is in progress.");
      fireEvent.click(open);
      // Every model-replacing command starts nothing: no prompt, no native call.
      for (const command of ["file.open-local", "file.open-model", "file.new-blank"]) act(() => nativeMenuCommand(command));
      for (const id of ["open-local-project", "open-model-document", "new-blank-040"]) expect(screen.getByTestId(id)).toBeDisabled();
      expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
      expect(screen.queryByTestId("project-units-dialog")).not.toBeInTheDocument();
      expect(invokeMock.mock.calls.slice(callsBefore).map(([name]) => name)
        .filter((name) => ["open_local_project", "open_model_document_file", "read_model_state", "save_model_state"].includes(name as string))).toEqual([]);

      // Another window saves A; the pending save meets it and asks.
      rows.rows.set(A, "sha256:A-other-window");
      await act(async () => { release(); });
      await screen.findByTestId("replace-project-prompt");
      expect(screen.getAllByRole("alertdialog")).toHaveLength(1);
      expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
      // Native Undo and shortcuts are held while the prompt is open.
      act(() => nativeMenuCommand("edit.undo"));
      fireEvent.keyDown(screen.getByTestId("replace-project-cancel"), { key: "z", metaKey: true });
      expect(reviewValue("node:N-100-y")).toBe("0.5");
      expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
      fireEvent.click(screen.getByTestId("replace-project-cancel"));
      await waitFor(() => expect(screen.getByTestId("desktop-preview-shell")).not.toHaveAttribute("inert"));
      // Released: Undo works again, and Open State is enabled.
      await waitFor(() => expect(screen.getByTestId(`model-state-${basis.state_id}-open`)).toBeEnabled());
      act(() => nativeMenuCommand("edit.undo"));
      expect(reviewValue("node:N-100-y")).toBe(originalY);
      expect(rows.rows.get(A)).toBe("sha256:A-other-window");
    });

    it("Open State and Save Model State refuse by name if reached while the session is busy", async () => {
      await renderApp();
      setTauriRuntime(true);
      let release!: () => void;
      const rows: RowsOptions = { rows: new Map([[A, "sha256:A-1"]]) };
      const store = nativeStore({ rows });
      act(() => nativeMenuCommand("file.open-local"));
      await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened local SQLite project snapshot."));
      await refreshHistory();
      await saveStateNamed("Basis");
      await applyNodeEdit("0.5", 1);
      // The commands' own refusal, below the disabled controls: the open runs
      // through the form submit path of the History hook.
      const panel = openProjectSection();
      fireEvent.click(within(panel).getByTestId("model-state-save-open"));
      fireEvent.change(within(panel).getByTestId("model-state-save-name"), { target: { value: "During the save" } });
      const form = within(panel).getByTestId("model-state-save-form");
      rows.gate = new Promise<void>((resolve) => { release = resolve; });
      act(() => nativeMenuCommand("file.save-local"));
      await waitFor(() => expect(calls("save_local_project")).toHaveLength(1));
      fireEvent.submit(form);
      await waitFor(() => expect(historyMessage()).toHaveTextContent("Not saved: another project operation or prompt is in progress; nothing was replaced."));
      expect(calls("save_model_state")).toHaveLength(1);
      expect(store.states).toHaveLength(1);
      await act(async () => { release(); });
      await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Saved current project model snapshot"));
    });
  });

  describe("Open State binds the session to the state's project", () => {
    const A = DEMO.project.id;

    /** The session opens stored project A (base A-1), saves state "Basis" in
     * place, edits, then opens "Basis" through Save first → Save as a new project. */
    async function openBasisThroughSaveAsNew(prepare: (rows: RowsOptions) => void) {
      await renderApp();
      setTauriRuntime(true);
      const rows: RowsOptions = { rows: new Map([[A, "sha256:A-1"]]), refuseOnce: new Set() };
      const store = nativeStore({ rows });
      act(() => nativeMenuCommand("file.open-local"));
      await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened local SQLite project snapshot."));
      await refreshHistory();
      await saveStateNamed("Basis");
      const basis = store.states[0];
      expect(calls("save_local_project")).toHaveLength(0);
      expect((calls("save_model_state")[0].request as SaveRequest).expected_base).toBe("sha256:A-1");
      await applyNodeEdit("0.5", 1);
      prepare(rows);
      fireEvent.click(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-open`));
      fireEvent.click(await screen.findByTestId("unsaved-changes-save"));
      fireEvent.click(await screen.findByTestId("replace-project-save-as-new"));
      await waitFor(() => expect(historyMessage()).toHaveTextContent(`Opened model state "Basis" (${basis.state_id}) as the working model.`));
      const projectSaves = calls("save_local_project").map((args) => (args.request as { model: PreviewModel }).model.project.id);
      expect(projectSaves).toHaveLength(2);
      expect(projectSaves[1]).toMatch(new RegExp(`^${A}:copy-`));
      expect(screen.getByTestId("toolbar-project-name")).toHaveTextContent(DEMO.project.name);
      // An edit of the opened state, then Save Model State.
      await applyNodeEdit("0.75", 1);
      const stateSavesBefore = calls("save_model_state").length;
      await saveStateNamed("Rev B");
      return { rows, stateSavesBefore };
    }

    it("keeps the base it held for that project before the guard: one state save, no extra project save", async () => {
      const { stateSavesBefore } = await openBasisThroughSaveAsNew((rows) => rows.refuseOnce!.add(A));
      await waitFor(() => expect(historyMessage()).toHaveTextContent('Saved model state "Rev B"'));
      expect(calls("save_local_project")).toHaveLength(2);
      const stateSaves = calls("save_model_state").slice(stateSavesBefore).map((args) => args.request as SaveRequest);
      expect(stateSaves).toHaveLength(1);
      expect(stateSaves[0].project_id).toBe(A);
      expect(stateSaves[0].expected_base).toBe("sha256:A-1");
    });

    it("never hides another window's save: the kept base is refused as PROJECT-BASE-MOVED", async () => {
      const { rows, stateSavesBefore } = await openBasisThroughSaveAsNew((rows) => rows.rows.set(A, "sha256:A-other-window"));
      await waitFor(() => expect(historyMessage()).toHaveTextContent("Not saved: PROJECT-BASE-MOVED"));
      expect(calls("save_local_project")).toHaveLength(2);
      const stateSaves = calls("save_model_state").slice(stateSavesBefore).map((args) => args.request as SaveRequest);
      expect(stateSaves.map((request) => [request.project_id, request.expected_base])).toEqual([[A, "sha256:A-1"]]);
      expect(rows.rows.get(A)).toBe("sha256:A-other-window");
    });

    it("checks the opened snapshot's library references, as a project open does", async () => {
      await renderApp();
      setTauriRuntime(true);
      const finding = { code: "MODEL-LIBRARY-REF-VERSION-CHANGED", severity: "warning", entity_ref: DEMO.materials![0].id, library_kind: "material",
        library_id: "matlib.invented.state", record_id: "mat.state", recorded_version: "sha256:old", stored_version: "sha256:new",
        message: "The stored record has changed since it was applied." };
      const store = nativeStore({ rows: { rows: new Map([[A, "sha256:A-1"]]), libraryFindings: [finding] } });
      const withRef = structuredClone(DEMO);
      withRef.materials![0].provenance_record = { source_name: "Invented", library_ref: { library_kind: "material", library_id: "matlib.invented.state", library_version: "sha256:old", record_id: "mat.state" } };
      const hash = (await computeModelHash(withRef))!.value;
      store.states = [{ project_id: A, state_id: "state:library-ref", state_kind: "design_snapshot", created_at: "2026-10-24T10:00:00.000Z",
        parent_state_id: null, model_hash: hash, record_hash: "record-lib", labels_updated_at: "2026-10-24T10:00:00.000Z", child_state_ids: [],
        analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "With a library reference", tags: [], notes: [], assumptions: [] }];
      store.snapshots.set("state:library-ref", JSON.stringify(withRef));
      await refreshHistory();
      fireEvent.click(await screen.findByTestId("model-state-state:library-ref-open"));
      await waitFor(() => expect(historyMessage()).toHaveTextContent("MODEL-LIBRARY-REF-VERSION-CHANGED"));
      expect(historyMessage()).toHaveTextContent('Opened model state "With a library reference" (state:library-ref) as the working model.');
      const checks = calls("check_model_library_refs");
      expect(checks).toHaveLength(1);
      expect(checks[0]).toEqual({ projectId: A, references: [{ entity_ref: DEMO.materials![0].id, library_kind: "material", library_id: "matlib.invented.state", library_version: "sha256:old", record_id: "mat.state" }] });
    });
  });

  it("Open State waits behind the unsaved-changes guard and starts a fresh undo history; edits make a new working model", async () => {
    await renderApp();
    setTauriRuntime(true);
    const store = nativeStore();
    await refreshHistory();
    await saveStateNamed("Basis");
    const basis = store.states[0];
    await applyNodeEdit("0.5", 1);
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    expect(screen.getByTestId("workspace-undo")).toBeEnabled();

    // Cancel (the default) keeps the working model and its edits.
    fireEvent.click(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-open`));
    expect(await screen.findByTestId("unsaved-changes-prompt")).toHaveAttribute("role", "alertdialog");
    expect(screen.getByTestId("unsaved-changes-detail")).toHaveTextContent(`Opening the saved state "Basis" (${basis.state_id}) replaces the working model.`);
    expect(screen.getByTestId("unsaved-changes-cancel")).toHaveFocus();
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(historyMessage()).toHaveTextContent("Not opened: the working model and its unsaved edits were kept."));
    expect(calls("read_model_state")).toHaveLength(0);
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();

    // Open replaces it with the snapshot; undo cannot walk back into the edited model.
    fireEvent.click(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-open`));
    fireEvent.click(await screen.findByTestId("unsaved-changes-discard"));
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`Opened model state "Basis" (${basis.state_id}) as the working model.`));
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(screen.getByTestId("workspace-redo")).toBeDisabled();
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    openNodeGridReview();
    expect(reviewValue("node:N-100-y")).toBe(originalY);
    expect(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-basis`)).toBeInTheDocument();

    // An edit after opening is a new working model: undo returns to the snapshot and no further.
    await applyNodeEdit("0.75", 1);
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    fireEvent.click(screen.getByTestId("workspace-undo"));
    expect(reviewValue("node:N-100-y")).toBe(originalY);
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    fireEvent.click(screen.getByTestId("workspace-redo"));
    expect(reviewValue("node:N-100-y")).toBe("0.75");
    // Saving it makes a new state whose parent is the opened one.
    await saveStateNamed("Rev B");
    const second = calls("save_model_state").at(-1)!.request as SaveRequest;
    expect(second.parent_state_id).toBe(basis.state_id);
    expect(second.model_hash).not.toBe(basis.model_hash);
    expect(store.states).toHaveLength(2);
    expect(calls("save_local_project")).toHaveLength(1);
  });

  it("Open State's Save Local Project first saves the edits, then opens the state", async () => {
    await renderApp();
    setTauriRuntime(true);
    const store = nativeStore();
    await refreshHistory();
    await saveStateNamed("Basis");
    const basis = store.states[0];
    await applyNodeEdit("0.5", 1);
    const savesBefore = calls("save_local_project").length;
    act(() => { fireEvent.click(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-open`)); });
    expect(screen.getByTestId("unsaved-changes-cancel")).toHaveFocus();
    fireEvent.click(screen.getByTestId("unsaved-changes-save"));
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`Opened model state "Basis" (${basis.state_id}) as the working model.`));
    expect(calls("save_local_project")).toHaveLength(savesBefore + 1);
    const saved = calls("save_local_project").at(-1)!.request as { model: PreviewModel };
    expect(saved.model.nodes.find((node) => node.id === "node:N-100")!.position.y).toBe(0.5);
    const sequence = commandOrder();
    expect(sequence.lastIndexOf("save_local_project")).toBeLessThan(sequence.lastIndexOf("read_model_state"));
    expect(sequence.filter((name) => name === "read_model_state")).toHaveLength(1);
  });

  it("names why a state with blocking document findings cannot be opened, and does not open it", async () => {
    await renderApp();
    setTauriRuntime(true);
    const store = nativeStore();
    await refreshHistory();
    await saveStateNamed("Incomplete basis");
    store.states[0] = { ...store.states[0], analysis_status: ["HUMAN_REVIEW_REQUIRED", "MODEL_INCOMPLETE"] };
    await refreshHistory();
    const id = store.states[0].state_id;
    await waitFor(() => expect(screen.getByTestId(`model-state-${id}-not-openable`)).toHaveTextContent(
      `Cannot be opened: the saved state "Incomplete basis" (${id}) is MODEL_INCOMPLETE: its model has blocking document findings, so it is kept as a comparison basis but cannot be opened as the working model`
    ));
    expect(screen.getByTestId(`model-state-${id}-open`)).toBeDisabled();
    expect(calls("read_model_state")).toHaveLength(0);
  });

  it("edits labels with the sentence that they do not change the saved state, and names a delete's blockers", async () => {
    await renderApp();
    setTauriRuntime(true);
    const store = nativeStore();
    await refreshHistory();
    await saveStateNamed("Parent");
    const parent = store.states[0];
    await applyNodeEdit("0.5", 1);
    await saveStateNamed("Child");
    const child = store.states[0];
    expect(child.parent_state_id).toBe(parent.state_id);

    // Label edit.
    const parentRow = await screen.findByTestId(`model-state-${parent.state_id}`);
    fireEvent.click(within(parentRow).getByTestId(`model-state-${parent.state_id}-edit`));
    expect(within(parentRow).getByTestId("model-state-labels-note")).toHaveTextContent("Renaming, tagging or adding notes does not change the saved state.");
    fireEvent.change(within(parentRow).getByTestId(`model-state-${parent.state_id}-labels-name`), { target: { value: "Parent – renamed" } });
    fireEvent.click(within(parentRow).getByTestId(`model-state-${parent.state_id}-labels-add-assumption`));
    fireEvent.change(within(parentRow).getByLabelText("Statement"), { target: { value: "Vendor loads pending." } });
    fireEvent.click(within(parentRow).getByTestId(`model-state-${parent.state_id}-labels-save`));
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`Labels of model state "Parent – renamed" (${parent.state_id}) saved.`));
    const [edit] = calls("edit_model_state_labels");
    expect(edit.expectedUpdatedAt).toBe(parent.labels_updated_at);
    expect((edit.labels as StateLabels).assumptions).toEqual([
      expect.objectContaining({ kind: "assumption", status: "unresolved", statement: "Vendor loads pending.", affected_refs: [] })
    ]);
    expect(store.states.find((state) => state.state_id === parent.state_id)!.record_hash).toBe(parent.record_hash);

    // Delete: the parent is blocked while its child names it.
    const renamedRow = await screen.findByTestId(`model-state-${parent.state_id}`);
    await waitFor(() => expect(within(renamedRow).getByTestId(`model-state-${parent.state_id}-name`)).toHaveTextContent("Parent – renamed"));
    expect(within(renamedRow).getByTestId(`model-state-${parent.state_id}-delete`)).toBeDisabled();
    expect(within(renamedRow).getByTestId(`model-state-${parent.state_id}-delete-blocked`)).toHaveTextContent(
      `Delete is blocked while saved state "Child" (${child.state_id}) names it as parent.`
    );
    const childRow = screen.getByTestId(`model-state-${child.state_id}`);
    fireEvent.click(within(childRow).getByTestId(`model-state-${child.state_id}-delete`));
    fireEvent.click(within(childRow).getByTestId(`model-state-${child.state_id}-delete-yes`));
    await waitFor(() => expect(historyMessage()).toHaveTextContent(`Deleted model state "Child" (${child.state_id}).`));
    await waitFor(() => expect(within(screen.getByTestId(`model-state-${parent.state_id}`)).getByTestId(`model-state-${parent.state_id}-delete`)).toBeEnabled());
    expect(calls("delete_model_state").map((args) => args.stateId)).toEqual([child.state_id]);
  });
});
