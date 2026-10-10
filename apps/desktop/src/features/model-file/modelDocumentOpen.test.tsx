import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import milestoneRequest from "../../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json";
import sensitiveTorsionModel from "../../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json";
import { App } from "../../App";
import { checkNativeOutcome, modelFileRefusalMessage, openModelDocumentFile } from "../../services/modelDocumentFile";
import type { PreviewModel } from "../../types";
import { MODEL_FILE_SHAPE, modelDocumentShapeDiagnostics } from "./modelDocumentShape";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function setTauriRuntime() {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function openedDocument(): PreviewModel {
  const document = clone(demoModel) as unknown as PreviewModel;
  document.project.id = "project:invented-opened-file";
  document.project.name = "Invented opened file";
  return document;
}

describe("desktop model document shape", () => {
  it("accepts the documents the desktop writes and bundles", () => {
    expect(modelDocumentShapeDiagnostics(demoModel)).toEqual([]);
  });

  it("refuses a solve request's bare model by name, listing every missing display member", () => {
    const diagnostics = modelDocumentShapeDiagnostics(milestoneRequest.model);
    expect(diagnostics.every((item) => item.code === MODEL_FILE_SHAPE)).toBe(true);
    const paths = diagnostics.map((item) => item.path);
    for (const path of ["data_boundary", "project.name", "project.description", "nodes[0].label", "pipe_segments[0].label",
      "supports[0].label", "materials[0].label", "components", "load_cases[0].status", "diagnostics"]) {
      expect(paths).toContain(path);
    }
    expect(modelDocumentShapeDiagnostics(sensitiveTorsionModel).length).toBeGreaterThan(0);
  });

  it("names wrong kinds and non-objects", () => {
    expect(modelDocumentShapeDiagnostics([])).toEqual([{ code: MODEL_FILE_SHAPE, path: "$", message: "$ is array; object required." }]);
    const document = clone(demoModel) as unknown as Record<string, unknown>;
    (document.nodes as Array<Record<string, unknown>>)[1].position = { x: 0, y: "1", z: 0 };
    expect(modelDocumentShapeDiagnostics(document)).toEqual([
      { code: MODEL_FILE_SHAPE, path: "nodes[1].position.y", message: "nodes[1].position.y is string; number required." }
    ]);
  });

  it("applies the shape check to a natively opened document and passes refusals through", () => {
    const opened = { outcome: "opened" as const, file_name: "m.json", byte_count: 2, schema_status: "current", document: milestoneRequest.model };
    const checked = checkNativeOutcome(opened);
    expect(checked.outcome).toBe("refused");
    const refusal = { outcome: "refused" as const, file_name: "m.json", diagnostics: [{ code: "MODEL-FILE-NOT-JSON", path: "", message: "bad" }] };
    expect(checkNativeOutcome(refusal)).toBe(refusal);
    expect(checkNativeOutcome({ ...opened, document: demoModel }).outcome).toBe("opened");
    expect(modelFileRefusalMessage("m.json", refusal.diagnostics)).toBe("Open model document refused (m.json): MODEL-FILE-NOT-JSON: bad");
  });

  it("is native-only: the browser preview refuses by name without a fixture fallback", async () => {
    const outcome = await openModelDocumentFile();
    expect(outcome).toMatchObject({ outcome: "refused", diagnostics: [{ code: "MODEL-FILE-NATIVE-ONLY" }] });
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

describe("File > Open Model Document…", () => {
  async function renderApp() {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await screen.findByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`);
    setTauriRuntime();
  }

  it("adopts the opened document unchanged as the unsaved session model and solves that document", async () => {
    const document = openedDocument();
    let solved: unknown = null;
    invokeMock.mockImplementation((command: string, args?: { model?: unknown }) => {
      if (command === "open_model_document_file") {
        return Promise.resolve({ outcome: "opened", file_name: "opened.json", byte_count: 1234, schema_status: "migrated", document: clone(document) });
      }
      if (command === "start_preview_mechanics_job_with_solver_mode") {
        solved = args?.model;
        return new Promise(() => {});
      }
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    await renderApp();
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      "Opened model document opened.json (1234 bytes; schema_version 0.1.0, migrated when saved as a project). Not saved as a local project yet."
    ));
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(document.project.id)}`)).toBeInTheDocument();
    expect(screen.queryByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`)).not.toBeInTheDocument();
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(solved).not.toBeNull());
    expect(solved).toEqual(document);
  });

  it("leaves the session as it was on a named refusal and on a cancelled chooser", async () => {
    const answers = [
      { outcome: "refused", file_name: "new.json", diagnostics: [{ code: "MODEL-FILE-SCHEMA-REFUSED", path: "schema_version", message: "schema_version `9.0.0` refused (newer_than_supported)." }] },
      { outcome: "opened", file_name: "request.json", byte_count: 10, schema_status: "migrated", document: clone(milestoneRequest.model) },
      { outcome: "cancelled" }
    ];
    invokeMock.mockImplementation((command: string) => {
      if (command === "open_model_document_file") return Promise.resolve(answers.shift());
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    await renderApp();
    const demoRow = `tree-row-project-${encodeURIComponent(demoModel.project.id)}`;
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      "Open model document refused (new.json): MODEL-FILE-SCHEMA-REFUSED at schema_version: schema_version `9.0.0` refused (newer_than_supported)."
    ));
    expect(screen.getByTestId(demoRow)).toBeInTheDocument();
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      "Open model document refused (request.json): MODEL-FILE-SHAPE at data_boundary: data_boundary is missing (object required)."
    ));
    expect(screen.getByTestId(demoRow)).toBeInTheDocument();
    const before = screen.getByTestId("local-project-message").textContent;
    fireEvent.click(screen.getByTestId("open-model-document"));
    await waitFor(() => expect(invokeMock.mock.calls.filter(([command]) => command === "open_model_document_file")).toHaveLength(3));
    expect(screen.getByTestId("local-project-message").textContent).toBe(before);
    expect(screen.getByTestId(demoRow)).toBeInTheDocument();
  });
});
