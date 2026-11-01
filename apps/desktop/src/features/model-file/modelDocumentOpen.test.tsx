import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// The session's last guard builds the model index for the candidate. One invented
// project id makes it throw, standing in for any unindexable document.
vi.mock("../workspace/modelIndex", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../workspace/modelIndex")>();
  return {
    ...actual,
    buildModelIndex: (...args: Parameters<typeof actual.buildModelIndex>) => {
      if (args[0].project.id === "project:invented-unindexable") throw new TypeError("invented index failure");
      return actual.buildModelIndex(...args);
    }
  };
});

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import milestoneRequest from "../../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json";
import sensitiveTorsionModel from "../../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { App } from "../../App";
import { checkNativeOutcome, modelFileRefusalMessage, modelFileWarningSuffix, openModelDocumentFile } from "../../services/modelDocumentFile";
import type { PreviewModel } from "../../types";
import { MODEL_FILE_SHAPE, modelDocumentShapeDiagnostics, modelDocumentSlotValidation } from "./modelDocumentShape";

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

  it("checks the nested members the desktop dereferences", () => {
    const document = clone(demoModel) as unknown as Record<string, unknown>;
    document.sections = [{ id: "sec:a", name: "a", section_type: "pipe", provenance: { source: "invented" } }];
    const materials = document.materials as Array<Record<string, unknown>>;
    delete materials[0].elastic_modulus;
    (document.combinations as Array<Record<string, unknown>>)[0].terms = [null, { load_case: "load:L-100", factor: "1" }];
    (document.supports as Array<Record<string, unknown>>)[0].restraints = ["UX", 3];
    const section = (document.pipe_segments as Array<Record<string, unknown>>)[0].section as Record<string, unknown>;
    section.wall_thickness = { value: 0.007 };
    expect(modelDocumentShapeDiagnostics(document).map((item) => item.path)).toEqual([
      "materials[0].elastic_modulus",
      "sections[0].properties",
      "pipe_segments[0].section.wall_thickness.unit",
      "supports[0].restraints[1]",
      "combinations[0].terms[0]",
      "combinations[0].terms[1].factor"
    ]);
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

  // W-17: model document 0.7.0's schema leaves `project.units` optional; the desktop
  // requires the map at open and refuses its absence by name. A missing key opens.
  it("refuses a document without project.units by name, and opens one missing a key", () => {
    const opened = { outcome: "opened" as const, file_name: "m.json", byte_count: 2, schema_status: "current" };
    const withoutMap = openedDocument() as unknown as { project: Record<string, unknown> };
    delete withoutMap.project.units;
    expect(checkNativeOutcome({ ...opened, document: withoutMap })).toEqual({ outcome: "refused", file_name: "m.json", diagnostics: [
      { code: MODEL_FILE_SHAPE, path: "project.units", message: "project.units is missing (object required)." }
    ] });
    const withoutAngle = openedDocument();
    delete (withoutAngle.project.units as Record<string, string>).angle;
    expect(checkNativeOutcome({ ...opened, document: withoutAngle }).outcome).toBe("opened");
  });

  // W-13 §5.3: the webview runs the applier's Rust document validator through the
  // wasm engine (no TypeScript re-implementation) before adopting a document.
  it("opens a document carrying every model-data slot unchanged, with no warnings", () => {
    expect(modelDocumentShapeDiagnostics(everySlotModel)).toEqual([]);
    expect(modelDocumentSlotValidation(everySlotModel)).toEqual({ blocking: [], warnings: [] });
    const native = { outcome: "opened" as const, file_name: "slots.json", byte_count: 3, schema_status: "current", document: clone(everySlotModel) };
    const checked = checkNativeOutcome(native);
    expect(checked).toEqual({ ...native, warnings: [] });
    expect(checked.outcome === "opened" && checked.document).toEqual(everySlotModel);
  });

  it("refuses malformed model-data slots by name with their JSON paths", () => {
    const document = clone(everySlotModel) as unknown as Record<string, any>;
    document.lines[1].pipe_refs = ["pipe:P-1"];
    document.rule_pack_refs[0].checksum.value = "not-a-digest";
    document.load_cases[0].analysis_state = { notes: "closed solve input" };
    const checked = checkNativeOutcome({ outcome: "opened", file_name: "bad.json", byte_count: 3, schema_status: "current", document });
    expect(checked.outcome).toBe("refused");
    expect(checked.outcome === "refused" && checked.diagnostics.map(({ code, path }) => [code, path])).toEqual([
      ["MODEL-ANNOTATION-NOT-ALLOWED", "load_cases[0].analysis_state.notes"],
      ["MODEL-LINE-PIPE-SHARED", "lines[1].pipe_refs[0]"],
      ["MODEL-RULE-PACK-REF-INVALID", "rule_pack_refs[0].checksum.value"]
    ]);
  });

  it("opens with the validator's warnings when a subject is missing or a disposition quarantined", () => {
    const document = clone(everySlotModel) as unknown as Record<string, any>;
    document.unresolved_assumptions[0].affected_refs = ["support:S-absent"];
    document.supports[0].provenance_record.redistribution_status = "protected_suspected";
    const checked = checkNativeOutcome({ outcome: "opened", file_name: "warn.json", byte_count: 3, schema_status: "current", document });
    expect(checked.outcome).toBe("opened");
    const warnings = checked.outcome === "opened" ? checked.warnings : [];
    expect(warnings.map(({ code, path }) => [code, path])).toEqual([
      ["MODEL-PROVENANCE-QUARANTINE", "supports[0].provenance_record.redistribution_status"],
      ["MODEL-ASSUMPTION-SUBJECT-MISSING", "unresolved_assumptions[0].affected_refs[0]"]
    ]);
    expect(modelFileWarningSuffix(warnings)).toBe(
      " Warnings: MODEL-PROVENANCE-QUARANTINE at supports[0].provenance_record.redistribution_status | MODEL-ASSUMPTION-SUBJECT-MISSING at unresolved_assumptions[0].affected_refs[0]."
    );
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
    invokeMock.mockImplementation((command: string, args?: unknown) => {
      if (command === "open_model_document_file") {
        return Promise.resolve({ outcome: "opened", file_name: "opened.json", byte_count: 1234, schema_status: "current", document: clone(document) });
      }
      if (command === "start_preview_mechanics_job") {
        solved = JSON.parse(new TextDecoder().decode(args as Uint8Array));
        return new Promise(() => {});
      }
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    await renderApp();
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      "Opened model document opened.json (1234 bytes; schema_version 0.9.0, current). Not saved as a local project yet."
    ));
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(document.project.id)}`)).toBeInTheDocument();
    expect(screen.queryByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`)).not.toBeInTheDocument();
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(solved).not.toBeNull());
    expect(solved).toEqual(document);
  });

  it("refuses a document the desktop cannot index instead of adopting it", async () => {
    const document = openedDocument();
    document.project.id = "project:invented-unindexable";
    invokeMock.mockImplementation((command: string) => {
      if (command === "open_model_document_file") {
        return Promise.resolve({ outcome: "opened", file_name: "unindexable.json", byte_count: 1, schema_status: "current", document });
      }
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    await renderApp();
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      "Open model document refused (unindexable.json): MODEL-FILE-SHAPE: The desktop could not index the document: TypeError: invented index failure."
    ));
    expect(screen.getByTestId("desktop-preview-shell")).toBeInTheDocument();
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`)).toBeInTheDocument();
  });

  it("leaves the session as it was on a named refusal and on a cancelled chooser", async () => {
    const answers = [
      { outcome: "refused", file_name: "new.json", diagnostics: [{ code: "MODEL-FILE-SCHEMA-REFUSED", path: "schema_version", message: "schema_version `9.0.0` refused (newer_than_supported)." }] },
      { outcome: "opened", file_name: "request.json", byte_count: 10, schema_status: "current", document: clone(milestoneRequest.model) },
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
