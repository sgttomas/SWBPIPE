// W-17 recovery export (DEL-02-02 Design `project-unit-selection.md` §3): a
// stored project refused at open for lacking `project.units` is offered "Save
// the stored document to a file…", behind the explicit intent. The webview
// sends only the stored row's identity; the native command writes the stored
// bytes (`src-tauri/src/stored_model_document_export.rs`, whose tests pin the
// bytes and the read-only store). Nothing is adopted. Invented data only.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
// Every call as it reaches the native host, before the ingress decoding.
const wireMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => {
  const decoding = (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock);
  return { invoke: (...call: unknown[]) => { wireMock(...call); return decoding(...call); } };
});

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { INGRESS_FORMAT, INGRESS_HEADER, ingressArgs } from "../../test/nativeIngress";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { ModelDocumentSaveDialog } from "./ModelDocumentSaveDialog";
import { MODEL_DOC_SAVE_DESKTOP_ONLY, MODEL_DOC_SAVE_INTENT_REQUIRED } from "./modelDocumentSave";
import {
  STORED_DOCUMENT_EXPORT_INTENT_LABEL,
  saveStoredModelDocumentFile,
  storedDocumentExportMessage,
  type StoredDocumentExportTarget
} from "./storedDocumentExport";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  wireMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

const PROJECT = "project:invented-no-units";
const REFUSAL = "Model document refused: MODEL-FILE-SHAPE at project.units: project.units is missing (object required).";
const TARGET: StoredDocumentExportTarget = { projectId: PROJECT, stateId: null, description: `the stored project \`${PROJECT}\``, refusal: REFUSAL };

function unitlessModel(): Json {
  const model = structuredClone(demoModel) as Json;
  model.project.id = PROJECT;
  delete model.project.units;
  return model;
}

describe("saveStoredModelDocumentFile", () => {
  it("needs the explicit intent and the desktop app; it sends only the row's identity", async () => {
    expect(await saveStoredModelDocumentFile(TARGET, false)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_INTENT_REQUIRED }] });
    expect(await saveStoredModelDocumentFile(TARGET, true)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_DESKTOP_ONLY }] });
    expect(invokeMock).not.toHaveBeenCalled();
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({ outcome: "saved", file_name: "invented-no-units.json", byte_count: 12, model_hash: `sha256:${"b".repeat(64)}`, replaced_existing: false, durability: "durable", warnings: [] });
    const outcome = await saveStoredModelDocumentFile({ ...TARGET, stateId: "state:invented-1" }, true);
    expect(invokeMock).toHaveBeenCalledWith("save_stored_model_document_file", { projectId: PROJECT, stateId: "state:invented-1", explicitLocalPrivateIntent: true });
    expect(storedDocumentExportMessage(TARGET, outcome)).toBe(
      `Saved the stored project \`${PROJECT}\` to invented-no-units.json exactly as stored (12 bytes; SHA-256 sha256:${"b".repeat(64)}). It was not opened. Add project.units to the file, then open it with Open Model Document….`
    );
  });

  // Codex r3 P2: the native command takes its arguments as the `args-json-v1`
  // raw body the history commands take, bounded before it is decoded; typed
  // (JSON) arguments are refused there.
  it("sends the arguments as the args-json-v1 raw body with the ingress header", async () => {
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({ outcome: "cancelled" });
    expect(await saveStoredModelDocumentFile(TARGET, true)).toEqual({ outcome: "cancelled" });
    expect(wireMock).toHaveBeenCalledOnce();
    const call = wireMock.mock.calls[0];
    expect(call[0]).toBe("save_stored_model_document_file");
    expect(Object.prototype.toString.call(call[1])).toBe("[object Uint8Array]");
    expect(call[2]).toEqual({ headers: { [INGRESS_HEADER]: INGRESS_FORMAT } });
    expect(new TextDecoder().decode(call[1] as Uint8Array)).toBe(JSON.stringify({ projectId: PROJECT, stateId: null, explicitLocalPrivateIntent: true }));
    expect(ingressArgs(call)).toEqual({ projectId: PROJECT, stateId: null, explicitLocalPrivateIntent: true });
  });
});

describe("the recovery dialog", () => {
  it("names the refusal and asks for the explicit intent before the chooser", () => {
    const save = vi.fn();
    render(<ModelDocumentSaveDialog request={{ projectName: PROJECT, opener: null, cancel: vi.fn(), save, recovery: { description: TARGET.description, refusal: REFUSAL } }} />);
    const dialog = screen.getByTestId("model-document-save-dialog");
    expect(dialog).toHaveTextContent("Save the stored document to a file");
    expect(within(dialog).getByTestId("model-document-save-detail")).toHaveTextContent(`Not opened: ${REFUSAL} You can save the stored project \`${PROJECT}\` to a file you choose, exactly as the local store holds it`);
    expect(within(dialog).getByText(STORED_DOCUMENT_EXPORT_INTENT_LABEL)).toBeInTheDocument();
    const confirm = within(dialog).getByTestId("model-document-save-confirm");
    expect(confirm).toHaveTextContent("Save the stored document to a file…");
    expect(confirm).toBeDisabled();
    fireEvent.click(within(dialog).getByTestId("model-document-save-intent"));
    fireEvent.click(confirm);
    expect(save).toHaveBeenCalledWith(true);
  });
});

describe("Open local project refused for lacking project.units", () => {
  it("offers the recovery export; the export writes through the native command and adopts nothing", async () => {
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "open_local_project") {
        return { summary: { project_id: PROJECT }, model: unitlessModel(), editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: null, project_envelope_hash: null };
      }
      if (command === "save_stored_model_document_file") {
        return { outcome: "saved", file_name: "invented-no-units.json", byte_count: 4096, model_hash: `sha256:${"c".repeat(64)}`, replaced_existing: false, durability: "durable", warnings: [] };
      }
      if (command === "sync_native_shell_state") return null;
      throw new Error(`Unexpected command ${command}`);
    });
    const before = hook.result.current;
    const spine = (session: typeof before) => [session.model.model, session.model.modelHash, session.model.projectSessionGeneration, session.results.result, session.project.modelEdited];
    const was = spine(before);
    await act(async () => { await before.project.handleOpenProject(PROJECT); });
    expect(hook.result.current.project.projectMessage).toBe(`Open failed: Error: ${REFUSAL}`);
    const request = hook.result.current.project.modelDocumentSaveRequest;
    expect(request?.recovery).toEqual({ description: TARGET.description, refusal: REFUSAL });
    act(() => request!.save(true));
    await waitFor(() => expect(hook.result.current.project.projectOperation).toBe("export_stored_document"));
    expect(hook.result.current.project.projectMessage).toContain(`Saved the stored project \`${PROJECT}\` to invented-no-units.json exactly as stored (4096 bytes;`);
    expect(invokeMock).toHaveBeenCalledWith("save_stored_model_document_file", { projectId: PROJECT, stateId: null, explicitLocalPrivateIntent: true });
    // Only the open's read and the export reached the native host: nothing was stored.
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => command !== "sync_native_shell_state"))
      .toEqual(["open_local_project", "save_stored_model_document_file"]);
    spine(hook.result.current).forEach((value, index) => expect(value).toBe(was[index]));
    expect(hook.result.current.project.projectBusy).toBe(false);
  });

  it("offers nothing for any other open failure", async () => {
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "open_local_project") throw new Error("invented store failure");
      if (command === "sync_native_shell_state") return null;
      throw new Error(`Unexpected command ${command}`);
    });
    await act(async () => { await hook.result.current.project.handleOpenProject(PROJECT); });
    expect(hook.result.current.project.projectOperation).toBe("open_failed");
    expect(hook.result.current.project.modelDocumentSaveRequest).toBeNull();
  });
});
