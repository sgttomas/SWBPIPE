// D5 slice 2 through the whole session (DEL-02-05 Design
// `autosave-and-backup.md` §1, §3): Open autosave runs the unsaved-changes
// guard and then adopts the record as Open Model Document adopts a file, with
// no stored base; and a landed Save Local Project discards that project's
// autosaves. Invented documents only.
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import { computeModelHash } from "../../services/hashService";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";
import { formatAutosaveTime } from "./autosaveRecovery";
import { autosaveSlot, type AutosaveEntry } from "./autosaveService";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO = demoModel as unknown as PreviewModel;
const RECOVERED_ID = "project:invented-autosave-recovered";
const RECOVERED_NAME = "Invented recovered model";
const WRITTEN = 1_790_000_000_000;

function recovered(): PreviewModel {
  const model = structuredClone(DEMO);
  model.project.id = RECOVERED_ID;
  model.project.name = RECOVERED_NAME;
  return model;
}

function envelope(model: PreviewModel, message: string): LocalProjectEnvelope {
  return {
    summary: { project_id: model.project.id, project_name: model.project.name, message },
    model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
    analysis_run: null, model_hash: null, project_envelope_hash: { value: "sha256:autosave-base" }, model_document_version: null
  } as unknown as LocalProjectEnvelope;
}

const called = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);
const toolbarProject = () => screen.getByTestId("toolbar-project-name");
const projectMessage = () => screen.getByTestId("local-project-message");

type Listing = AutosaveEntry[] | (() => Promise<AutosaveEntry[]>);
/** The model the last Save Local Project sent. */
let savedModel: PreviewModel | null = null;

/** The native commands the flows reach; `listings` are returned in turn by
 * `list_project_autosaves` (the last one thereafter). */
function nativeCommands(listings: Listing[]) {
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    switch (command) {
      case "sync_native_shell_state": return Promise.resolve(null);
      case "list_model_states": return Promise.resolve([]);
      case "find_local_project": return Promise.resolve(null);
      case "list_local_projects": return Promise.resolve([]);
      case "get_autosave_settings": return Promise.resolve({ settings: { autosave_minutes: 5, backup_folder: null, backup_keep: null }, source: "default", status: null });
      case "list_project_autosaves": {
        const listing = listings.length > 1 ? listings.shift()! : listings[0];
        return (typeof listing === "function" ? listing() : Promise.resolve(listing)).then((entries) => ({ outcome: "listed", entries, not_listed: 0 }));
      }
      case "read_project_autosave": {
        const document = recovered();
        return Promise.resolve({ outcome: "read", file_name: args!.fileName, slot: "", written_unix_millis: WRITTEN, model_hash: "sha256:" + "d".repeat(64), byte_count: 2048, document });
      }
      case "discard_project_autosave": return Promise.resolve({ outcome: "discarded", file_name: args!.fileName });
      case "acknowledge_close_request": return Promise.resolve(true);
      case "resolve_close_request": return Promise.resolve(null);
      case "save_local_project":
        savedModel = (args!.request as { model: PreviewModel }).model;
        return Promise.resolve(envelope((args!.request as { model: PreviewModel }).model, "Saved current project model snapshot without external file copies."));
      default: return Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
}

async function renderApp() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(DEMO.project.id)}`);
}

/** One applied edit of node N-100's y through the browser operation engine. */
async function applyNodeEdit() {
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  const summary = screen.getByTestId("node-grid-review-disclosure");
  if (summary.getAttribute("aria-expanded") !== "true") fireEvent.click(summary);
  fireEvent.doubleClick(screen.getByTestId("review-cell-node:N-100-y"));
  fireEvent.change(screen.getByTestId("engineering-table-review").querySelector("input")!, { target: { value: "0.5" } });
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
  fireEvent.click(screen.getAllByTestId(/^apply-intent-editor-intent-\d+$/).at(-1)!);
  await waitFor(() => expect(screen.getByTestId("operation-apply-summary")).toHaveTextContent("1 applied"), { timeout: 10000 });
  await waitFor(() => expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited"));
}

/** The bundled demo with one unsaved edit, then the desktop runtime, whose
 * autosave starts at the session's next render. */
async function editedNativeSession(listings: Listing[]) {
  await renderApp();
  await applyNodeEdit();
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  nativeCommands(listings);
  fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
}

async function recoveredRecord(): Promise<AutosaveEntry> {
  const slot = await autosaveSlot(RECOVERED_ID);
  return { file_name: `${slot}-${WRITTEN}-${"d".repeat(64)}.json`, slot, written_unix_millis: WRITTEN, model_hash: "sha256:" + "d".repeat(64), byte_count: 2048 };
}

describe("Open autosave", () => {
  it("goes through the unsaved-changes guard: Cancel keeps the working model and its edits", async () => {
    await editedNativeSession([[await recoveredRecord()]]);
    expect(await screen.findByTestId("autosave-recovery-prompt")).toBeInTheDocument();
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    fireEvent.click(screen.getByTestId("autosave-recovery-open"));
    expect(await screen.findByTestId("unsaved-changes-prompt")).toBeInTheDocument();
    expect(screen.getByTestId("unsaved-changes-detail")).toHaveTextContent("Opening an autosave replaces the working model.");
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Not opened: the working model and its unsaved edits were kept."));
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    expect(toolbarProject()).toHaveTextContent(DEMO.project.name);
    // Only the offer read the record; nothing was adopted.
    expect(called("read_project_autosave")).toHaveLength(1);
  });

  it("after Discard edits, reads the record verified again and adopts it as Open Model Document does, with no stored base", async () => {
    const record = await recoveredRecord();
    await editedNativeSession([[record]]);
    await screen.findByTestId("autosave-recovery-prompt");
    fireEvent.click(screen.getByTestId("autosave-recovery-open"));
    fireEvent.click(await screen.findByTestId("unsaved-changes-discard"));
    await waitFor(() => expect(toolbarProject()).toHaveTextContent(RECOVERED_NAME));
    expect(called("read_project_autosave").map(([, args]) => args.fileName)).toEqual([record.file_name, record.file_name]);
    expect(projectMessage()).toHaveTextContent(`Opened the autosave of ${RECOVERED_NAME} written ${formatAutosaveTime(WRITTEN)} (2048 bytes). No stored project was changed. Not saved as a local project yet.`);
    expect(screen.queryByTestId("project-edited")).toBeNull();
    // No stored project was opened or written.
    expect(called("open_local_project")).toHaveLength(0);
    expect(called("save_local_project")).toHaveLength(0);
  });
});

describe("a quit while the recovery offer is open", () => {
  it("closes nothing: the offer holds the window (the shared modal hold), and stays", async () => {
    await editedNativeSession([[await recoveredRecord()]]);
    await screen.findByTestId("autosave-recovery-prompt");
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-close-request", { detail: { requestId: 31, attempt: 1, kind: "quit" } })); });
    await waitFor(() => expect(called("resolve_close_request").map(([, args]) => args)).toEqual([{ requestId: 31, proceed: false }]));
    expect(projectMessage()).toHaveTextContent("Not quit: a dialog is open. Finish or cancel it first; nothing was closed.");
    expect(screen.getByTestId("autosave-recovery-prompt")).toBeInTheDocument();
    expect(screen.queryByTestId("unsaved-changes-prompt")).toBeNull();
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
  });
});

describe("Save Local Project", () => {
  it("discards the project's autosave holding the saved model once the save has landed, and keeps one with another hash", async () => {
    const slot = await autosaveSlot(DEMO.project.id);
    const other: AutosaveEntry = { file_name: `${slot}-1000000000000-${"a".repeat(64)}.json`, slot, written_unix_millis: 1_000_000_000_000, model_hash: "sha256:" + "a".repeat(64), byte_count: 10 };
    let same: AutosaveEntry | null = null;
    // The start's check finds nothing; the check after the save finds a record
    // of the saved model and an older one of another model.
    await editedNativeSession([[], async () => {
      const hash = (await computeModelHash(savedModel!))!.value;
      same = { file_name: `${slot}-1000000000001-${hash.slice(7)}.json`, slot, written_unix_millis: 1_000_000_000_001, model_hash: hash, byte_count: 10 };
      return [same, other];
    }]);
    await waitFor(() => expect(called("list_project_autosaves")).toHaveLength(1));
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: "file.save-local" })); });
    await waitFor(() => expect(called("save_local_project")).toHaveLength(1));
    await waitFor(() => expect(called("discard_project_autosave").map(([, args]) => args)).toEqual([{ fileName: same!.file_name }]));
  });
});
