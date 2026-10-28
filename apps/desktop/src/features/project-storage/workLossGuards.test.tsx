import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

// Work-loss guards (HF-08, W-17): the File menu names the command that stores
// the working model as a save (only New Blank Project starts a new, empty
// model); ⌘S / Ctrl+S is Save Local Project and waits while a modal holds the
// window; a native window close or quit asks first while the working model has
// unsaved edits (the host side is src-tauri/src/close_guard.rs).

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO = demoModel as unknown as PreviewModel;

function envelope(model: PreviewModel, message: string): LocalProjectEnvelope {
  return {
    summary: { project_id: model.project.id, project_name: model.project.name, message },
    model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
    analysis_run: null, model_hash: null, project_envelope_hash: { value: "sha256:guard-base" }, model_document_version: null
  } as unknown as LocalProjectEnvelope;
}

function nativeCommands() {
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    switch (command) {
      case "sync_native_shell_state": return Promise.resolve(null);
      case "list_model_states": return Promise.resolve([]);
      case "find_local_project": return Promise.resolve(null);
      case "create_local_project":
        return Promise.resolve(envelope((args as { model: PreviewModel }).model, "Created local SQLite project snapshot without external file copies."));
      case "save_local_project":
        return Promise.resolve(envelope((args!.request as { model: PreviewModel }).model, "Saved current project model snapshot without external file copies."));
      case "acknowledge_close_request": return Promise.resolve(true);
      case "resolve_close_request": return Promise.resolve(null);
      default: return Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
}

const called = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);

async function renderApp() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(DEMO.project.id)}`);
}

function setTauriRuntime(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

const projectMessage = () => screen.getByTestId("local-project-message");

function openNodeGridReview() {
  const summary = screen.getByTestId("node-grid-review-disclosure");
  if (summary.getAttribute("aria-expanded") !== "true") fireEvent.click(summary);
}

/** One applied edit of node N-100's y through the browser operation engine. */
async function applyNodeEdit() {
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  openNodeGridReview();
  fireEvent.doubleClick(screen.getByTestId("review-cell-node:N-100-y"));
  fireEvent.change(screen.getByTestId("engineering-table-review").querySelector("input")!, { target: { value: "0.5" } });
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
  fireEvent.click(screen.getAllByTestId(/^apply-intent-editor-intent-\d+$/).at(-1)!);
  await waitFor(() => expect(screen.getByTestId("operation-apply-summary")).toHaveTextContent("1 applied"), { timeout: 10000 });
  await waitFor(() => expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited"));
}

function pressSave(init: KeyboardEventInit = { metaKey: true }) {
  const event = new KeyboardEvent("keydown", { key: "s", bubbles: true, cancelable: true, ...init });
  act(() => { window.dispatchEvent(event); });
  return event;
}

function closeRequest(requestId: number, kind: "window" | "quit") {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-close-request", { detail: { requestId, kind } })); });
}

const answers = () => called("resolve_close_request").map(([, args]) => args as { requestId: number; proceed: boolean });
const acknowledgements = () => called("acknowledge_close_request").map(([, args]) => (args as { requestId: number }).requestId);

function fileMenuLabels(): string[] {
  fireEvent.click(screen.getByTestId("menu-file"));
  const labels = within(screen.getByTestId("menu-dropdown-file")).getAllByRole("menuitem").map((item) => item.firstChild?.textContent ?? "");
  fireEvent.click(screen.getByTestId("menu-file"));
  return labels;
}

describe("File menu: New starts empty; storing the working model is a save", () => {
  it("offers New Blank Project as the only New and names file.new-local Save as New Local Project, after Save Local Project", async () => {
    nativeCommands();
    await renderApp();
    const labels = fileMenuLabels();
    expect(labels[0]).toBe("New Blank Project");
    expect(labels.filter((label) => label.startsWith("New "))).toEqual(["New Blank Project"]);
    expect(labels.slice(labels.indexOf("Save Local Project"), labels.indexOf("Save Local Project") + 2))
      .toEqual(["Save Local Project", "Save as New Local Project"]);
  });

  it("Save as New Local Project stores the working model itself (the behaviour is kept)", async () => {
    nativeCommands();
    await renderApp();
    fireEvent.click(screen.getByTestId("menu-file"));
    expect(screen.getByTestId("menu-item-file.new-local")).toHaveTextContent("Save as New Local Project");
    fireEvent.click(screen.getByTestId("menu-file"));
    // The native store, through the native menu's same command id.
    setTauriRuntime(true);
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: "file.new-local" })); });
    await waitFor(() => expect(called("create_local_project")).toHaveLength(1));
    const request = called("create_local_project")[0][1] as { model: PreviewModel };
    expect(request.model.project.id).toBe(DEMO.project.id);
    expect(request.model.nodes?.length).toBe(DEMO.nodes?.length);
  });
});

describe("⌘S: Save Local Project", () => {
  it("⌘S and Ctrl+S save the local project, and the browser's own Save is held back", async () => {
    nativeCommands();
    await renderApp();
    // The browser preview saves to its in-memory project store.
    expect(pressSave().defaultPrevented).toBe(true);
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Saved local browser-preview project snapshot"));
    // Ctrl+S takes the same route (the handler claims the key).
    expect(pressSave({ ctrlKey: true }).defaultPrevented).toBe(true);
    // ⌥⌘S and ⇧⌘S are not Save.
    expect(pressSave({ metaKey: true, shiftKey: true }).defaultPrevented).toBe(false);
  });

  it("the File menu shows ⌘S on Save Local Project", async () => {
    nativeCommands();
    await renderApp();
    fireEvent.click(screen.getByTestId("menu-file"));
    const item = screen.getByTestId("menu-item-file.save-local");
    expect(item).toHaveTextContent("⌘S");
    expect(item).toHaveAttribute("aria-keyshortcuts", "Meta+S Control+S");
  });

  it("waits while a modal holds the window: nothing is saved", async () => {
    nativeCommands();
    await renderApp();
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: "file.new-blank" })); });
    await screen.findByTestId("project-units-dialog");
    pressSave();
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(called("save_local_project")).toHaveLength(0);
  });

  it("in the native runtime the menu accelerator owns ⌘S: the webview does not save a second time", async () => {
    nativeCommands();
    await renderApp();
    setTauriRuntime(true);
    expect(pressSave().defaultPrevented).toBe(false);
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(called("save_local_project")).toHaveLength(0);
  });
});

describe("Window close and quit: asked first while the working model has unsaved edits", () => {
  it("without edits the request is acknowledged and goes ahead without a prompt", async () => {
    nativeCommands();
    await renderApp();
    setTauriRuntime(true);
    closeRequest(1, "quit");
    await waitFor(() => expect(answers()).toEqual([{ requestId: 1, proceed: true }]));
    expect(acknowledgements()).toEqual([1]);
    expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
  });

  it("with edits Quit asks; Cancel keeps the app open and the edits", async () => {
    nativeCommands();
    await renderApp();
    await applyNodeEdit();
    setTauriRuntime(true);
    closeRequest(2, "quit");
    expect(await screen.findByTestId("unsaved-changes-detail")).toHaveTextContent("Quitting SWBPIPE closes the working model.");
    expect(screen.getByTestId("unsaved-changes-discard")).toHaveTextContent("Discard edits and quit");
    expect(acknowledgements()).toEqual([2]);
    expect(answers()).toEqual([]);
    // A repeat of the open request is acknowledged; its prompt is already showing.
    closeRequest(2, "quit");
    expect(acknowledgements()).toEqual([2, 2]);
    expect(screen.getAllByTestId("unsaved-changes-prompt")).toHaveLength(1);
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(answers()).toEqual([{ requestId: 2, proceed: false }]));
    expect(projectMessage()).toHaveTextContent("Not quit: the working model and its unsaved edits were kept.");
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
  });

  it("with edits a window close asks; Discard closes without saving", async () => {
    nativeCommands();
    await renderApp();
    await applyNodeEdit();
    setTauriRuntime(true);
    closeRequest(3, "window");
    expect(await screen.findByTestId("unsaved-changes-detail")).toHaveTextContent("Closing the window quits SWBPIPE");
    fireEvent.click(screen.getByTestId("unsaved-changes-discard"));
    await waitFor(() => expect(answers()).toEqual([{ requestId: 3, proceed: true }]));
    expect(called("save_local_project")).toHaveLength(0);
  });

  it("with edits Save Local Project first saves the edits, then quits", async () => {
    nativeCommands();
    await renderApp();
    await applyNodeEdit();
    setTauriRuntime(true);
    closeRequest(4, "quit");
    fireEvent.click(await screen.findByTestId("unsaved-changes-save"));
    await waitFor(() => expect(answers()).toEqual([{ requestId: 4, proceed: true }]));
    const saved = called("save_local_project")[0][1] as { request: { model: PreviewModel } };
    expect(saved.request.model.nodes.find((node) => node.id === "node:N-100")!.position.y).toBe(0.5);
  });

  it("after ⌘S the model is no longer edited, so a quit goes ahead without a prompt", async () => {
    nativeCommands();
    await renderApp();
    await applyNodeEdit();
    pressSave();
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
    setTauriRuntime(true);
    closeRequest(5, "quit");
    await waitFor(() => expect(answers()).toEqual([{ requestId: 5, proceed: true }]));
    expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
  });

  it("in the browser preview, unloading with edits asks through the page's unload prompt", async () => {
    nativeCommands();
    await renderApp();
    const clean = new Event("beforeunload", { cancelable: true });
    window.dispatchEvent(clean);
    expect(clean.defaultPrevented).toBe(false);
    await applyNodeEdit();
    const edited = new Event("beforeunload", { cancelable: true });
    window.dispatchEvent(edited);
    expect(edited.defaultPrevented).toBe(true);
  });
});
