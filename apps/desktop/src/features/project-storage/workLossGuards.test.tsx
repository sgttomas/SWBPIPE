import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

// Work-loss guards (HF-08, W-17): the File menu names the command that stores
// the working model as a save (only New Blank Project starts a new, empty
// model).

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

