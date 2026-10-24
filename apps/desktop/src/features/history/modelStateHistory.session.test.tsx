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
function nativeStore(options: { projectStored?: boolean; find?: StoredProjectEntry | null; refuseProjectSave?: string } = {}) {
  const store = {
    projectStored: options.projectStored ?? false,
    states: [] as ModelStateSummary[],
    snapshots: new Map<string, string>(),
    sequence: 0
  };
  const answer = (command: string, args: Record<string, unknown>): unknown => {
    switch (command) {
      case "sync_native_shell_state":
        return null;
      case "find_local_project":
        return options.find ?? null;
      case "save_local_project": {
        if (options.refuseProjectSave) throw options.refuseProjectSave;
        const request = args.request as { model: PreviewModel; model_hash: unknown };
        store.projectStored = true;
        return {
          summary: { project_id: request.model.project.id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
          model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
          analysis_run: null, model_hash: request.model_hash, project_envelope_hash: { value: BASE }, model_document_version: null
        } as unknown as LocalProjectEnvelope;
      }
      case "save_model_state": {
        const request = args.request as SaveRequest;
        if (!store.projectStored) throw `MODEL-STATE-PROJECT-MISSING: project \`${request.project_id}\` has no stored local project; save the local project first.`;
        if (request.expected_base !== BASE) throw `PROJECT-BASE-MOVED: project \`${request.project_id}\` changed after this session opened or saved it.`;
        const existing = store.states.find((state) => state.model_hash === request.model_hash);
        if (existing) return { outcome: "already_saved", state: existing, message: `This model is already saved as state "${existing.state_name}" (${existing.state_id}); nothing was written.` };
        const created_at = `2026-10-24T10:00:0${++store.sequence}.000Z`;
        const state: ModelStateSummary = {
          ...request.labels, project_id: request.project_id, state_id: `state:${String(store.sequence).padStart(32, "0")}`,
          state_kind: request.state_kind as ModelStateSummary["state_kind"], created_at, parent_state_id: request.parent_state_id,
          model_hash: request.model_hash, record_hash: `record-${store.sequence}`, labels_updated_at: created_at, child_state_ids: []
        };
        store.states = [state, ...store.states.map((other) => other.state_id === request.parent_state_id ? { ...other, child_state_ids: [...other.child_state_ids, state.state_id] } : other)];
        store.snapshots.set(state.state_id, request.model_canonical);
        return { outcome: "saved", state, message: `Saved model state "${state.state_name}" (${state.state_id}). ${LABELS_DO_NOT_CHANGE_STATE}` };
      }
      case "list_model_states":
        return store.states.filter((state) => state.project_id === args.projectId);
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
const commandOrder = () => invokeMock.mock.calls.map(([name]) => name as string).filter((name) => name !== "sync_native_shell_state" && name !== "list_model_states");
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
    expect(await screen.findByTestId("open-state-prompt")).toHaveAttribute("role", "alertdialog");
    expect(screen.getByTestId("open-state-detail")).toHaveTextContent(`Opening the saved state "Basis" (${basis.state_id}) replaces the working model.`);
    expect(screen.getByTestId("open-state-cancel")).toHaveFocus();
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    fireEvent.click(screen.getByTestId("open-state-cancel"));
    await waitFor(() => expect(historyMessage()).toHaveTextContent("Not opened: the working model and its unsaved edits were kept."));
    expect(calls("read_model_state")).toHaveLength(0);
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();

    // Open replaces it with the snapshot; undo cannot walk back into the edited model.
    fireEvent.click(within(openProjectSection()).getByTestId(`model-state-${basis.state_id}-open`));
    fireEvent.click(await screen.findByTestId("open-state-confirm"));
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
