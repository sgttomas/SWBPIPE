import { act, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { App } from "../../App";
import { canonicalJsonString, canonicalSha256Hex, computeModelHash } from "../../services/hashService";
import type { LibraryOpenRoute } from "../../services/libraryImportService";
import type { ModelHashEvidence, PreviewModel } from "../../types";
import { controlRouteExport } from "../redaction-controls/redactionExportControls";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import {
  MODEL_DOCUMENT_SAVE_INTENT_LABEL,
  MODEL_DOCUMENT_SAVE_ROUTE_ID,
  MODEL_DOC_SAVE_DESKTOP_ONLY,
  MODEL_DOC_SAVE_HASH_PENDING,
  MODEL_DOC_SAVE_INTENT_REQUIRED,
  MODEL_DOC_SAVE_RECEIPT_INVALID,
  MODEL_DOC_SAVE_VERSION_NOT_CURRENT,
  modelDocumentSaveMessage,
  modelDocumentSaveRefusalMessage,
  prepareModelDocumentSave,
  resolveLibraryDispositions,
  saveModelDocumentFile,
  screenModelDocumentDispositions,
  type ModelDocumentSaveArgs
} from "./modelDocumentSave";

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

function everySlot(): Record<string, any> {
  return clone(everySlotModel) as unknown as Record<string, any>;
}

async function hashOf(model: unknown): Promise<ModelHashEvidence> {
  return (await computeModelHash(model as PreviewModel))!;
}

async function prepared(model: unknown, extra: Partial<Parameters<typeof prepareModelDocumentSave>[0]> = {}) {
  return prepareModelDocumentSave({
    model: model as PreviewModel,
    modelHash: await hashOf(model),
    explicitLocalPrivateIntent: true,
    ...extra
  });
}

function codes(preparation: Awaited<ReturnType<typeof prepareModelDocumentSave>>) {
  return preparation.status === "refused" ? preparation.diagnostics.map(({ code, path }) => [code, path]) : "ready";
}

describe("Save Model Document…: the bytes", () => {
  it("sends the unchecked RFC 8785 text of the session document, whose SHA-256 is the session's model hash", async () => {
    for (const model of [demoModel, everySlotModel]) {
      const preparation = await prepared(model);
      expect(preparation.status).toBe("ready");
      if (preparation.status !== "ready") continue;
      const { canonicalText, claimedModelHash } = preparation.args;
      expect(canonicalText).toBe(await canonicalJsonString(model));
      expect(canonicalText.endsWith("\n")).toBe(false);
      expect(claimedModelHash).toBe((await hashOf(model)).value);
      // The text is its own canonical form, and its hash is the claimed hash.
      expect(await canonicalJsonString(JSON.parse(canonicalText))).toBe(canonicalText);
      expect(`sha256:${await canonicalSha256Hex(JSON.parse(canonicalText))}`).toBe(claimedModelHash);
    }
  });

  it("carries the governed route's lossless local-private evidence", async () => {
    const preparation = await prepared(demoModel);
    if (preparation.status !== "ready") throw new Error("demo refused");
    expect(preparation.args.screening).toEqual({
      route_id: "DOTH-MODEL-DOC-007",
      export_context: "local_private",
      explicit_local_private_intent: true,
      blocked: false,
      materialization_withheld: false,
      lossless_required: true,
      exact_payload_match: true,
      blocking_count: 0
    });
    expect(preparation.args.localFirstEvidence).toEqual({
      route_id: "DOTH-MODEL-DOC-007",
      export_context: "local_private",
      storage_context: "local_private",
      action: "include_metadata_only",
      reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED",
      blocked: false,
      metadata_only: true,
      explicit_local_private_intent: true
    });
  });
});

describe("Save Model Document…: admission in the webview", () => {
  it("needs the explicit intent and the session's computed model hash", async () => {
    expect(codes(await prepared(demoModel, { explicitLocalPrivateIntent: false }))).toEqual([[MODEL_DOC_SAVE_INTENT_REQUIRED, ""]]);
    expect(codes(await prepared(demoModel, { modelHash: null }))).toEqual([[MODEL_DOC_SAVE_HASH_PENDING, ""]]);
    const hash = await hashOf(demoModel);
    expect(codes(await prepared(demoModel, { modelHash: { ...hash, value: "sha256:abc" } }))).toEqual([[MODEL_DOC_SAVE_HASH_PENDING, ""]]);
  });

  it("refuses a version other than the current one, and the open-time shape and validator refusals", async () => {
    const retired = everySlot();
    retired.schema_version = "0.6.0";
    expect(codes(await prepared(retired))).toEqual([[MODEL_DOC_SAVE_VERSION_NOT_CURRENT, "schema_version"]]);
    const shapeless = everySlot();
    delete shapeless.nodes;
    expect(codes(await prepared(shapeless))).toEqual([["MODEL-FILE-SHAPE", "nodes"]]);
    const malformed = everySlot();
    malformed.lines[0].pipe_refs = ["pipe:P-absent"];
    expect(codes(await prepared(malformed))).toEqual([["MODEL-LINE-PIPE-MISSING", "lines[0].pipe_refs[0]"]]);
  });
});

describe("Save Model Document…: disposition screening", () => {
  it("is needed: the generic route projection passes a quarantined provenance record through", () => {
    const quarantined = everySlot();
    quarantined.materials[0].provenance_record.review_status = "quarantined";
    const generic = controlRouteExport(quarantined, {
      routeId: MODEL_DOCUMENT_SAVE_ROUTE_ID,
      exportContext: "local_private",
      explicitLocalPrivateIntent: true,
      requireLosslessMaterialization: true
    });
    expect(generic.blocked).toBe(false);
  });

  type Doc = Record<string, any>;
  it.each([
    ["a quarantined provenance record", (d: Doc) => { d.materials[0].provenance_record.review_status = "quarantined"; }, "materials[0].provenance_record", (d: Doc) => d.materials[0].id],
    ["a rejected provenance record", (d: Doc) => { d.sections[0].provenance_record.review_status = "rejected"; }, "sections[0].provenance_record", (d: Doc) => d.sections[0].id],
    ["a protected_suspected provenance record", (d: Doc) => { d.supports[0].provenance_record.redistribution_status = "protected_suspected"; }, "supports[0].provenance_record", (d: Doc) => d.supports[0].id],
    ["a protected_suspected rule-pack reference", (d: Doc) => { d.rule_pack_refs[0].redistribution_status = "protected_suspected"; }, "rule_pack_refs[0]", (d: Doc) => d.rule_pack_refs[0].rule_pack_id]
  ])("refuses %s with the screener's code, naming the record", async (_label, edit, path, owner) => {
    const model = everySlot();
    edit(model);
    const preparation = await prepared(model);
    expect(codes(preparation)).toEqual([["PROTECTED_CONTENT_BLOCKED", path]]);
    const diagnostics = preparation.status === "refused" ? preparation.diagnostics : [];
    expect(diagnostics[0].message).toContain(String(owner(model)));
    expect(modelDocumentSaveRefusalMessage(diagnostics)).toContain(`PROTECTED_CONTENT_BLOCKED at ${path}`);
  });

  it("refuses a resolved library disposition that is quarantined, naming the library record", async () => {
    const preparation = await prepared(everySlotModel, {
      resolvedLibraries: [{ library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "public_permissive", review_status: "quarantined" }]
    });
    expect(codes(preparation)).toEqual([["PROTECTED_CONTENT_BLOCKED", "library_ref(material:library:invented-materials#steel)"]]);
  });

  it("saves private_only and unknown dispositions", async () => {
    const model = everySlot();
    model.materials[0].provenance_record.redistribution_status = "private_only";
    model.sections[0].provenance_record.redistribution_status = "unknown";
    model.rule_pack_refs[0].redistribution_status = "private_only";
    const preparation = await prepared(model, {
      resolvedLibraries: [
        { library_kind: "material", library_id: "library:invented-materials", record_id: null, redistribution_status: "private_only", review_status: "accepted" },
        { library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "unknown", review_status: null }
      ]
    });
    expect(preparation.status).toBe("ready");
    const screening = screenModelDocumentDispositions(model, { explicitLocalPrivateIntent: true });
    expect(screening.blocking).toEqual([]);
    expect(screening.decisions.map((decision) => decision.path)).toEqual(expect.arrayContaining([
      "materials[0].provenance_record", "sections[0].provenance_record", "supports[0].provenance_record", "components[0].provenance_record", "rule_pack_refs[0]"
    ]));
    // Without the dialog's intent, private data is not admitted.
    expect(screenModelDocumentDispositions(model, { explicitLocalPrivateIntent: false }).blocking.map((item) => item.code)).toContain("LOCAL_PRIVATE_INTENT_REQUIRED");
  });

  it("resolves library references against the local store: the library's and the record's own dispositions", async () => {
    const opened: unknown[] = [];
    const open = async (projectId: string, kind: string, id: string): Promise<LibraryOpenRoute> => {
      opened.push([projectId, kind, id]);
      return {
        route: "tauri_backend",
        envelope: {
          project_id: projectId, library_kind: "material", library_id: id, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0,
          document: {
            material_library: { library_id: id, provenance: { redistribution_status: "private_only", review_status: "accepted" } },
            material_records: [{ material_id: "steel", provenance: { review_status: "quarantined" } }]
          },
          validation: {} as never,
          message: ""
        }
      } as LibraryOpenRoute;
    };
    const resolved = await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, open);
    expect(opened).toEqual([[everySlotModel.project.id, "material", "library:invented-materials"]]);
    expect(resolved).toEqual([
      { library_kind: "material", library_id: "library:invented-materials", record_id: null, redistribution_status: "private_only", review_status: "accepted" },
      { library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "private_only", review_status: "quarantined" }
    ]);
    expect(await resolveLibraryDispositions(demoModel as unknown as PreviewModel, open)).toEqual([]);
    const unavailable = async () => ({ route: "unavailable_browser" } as unknown as LibraryOpenRoute);
    expect(await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, unavailable)).toEqual([]);
    const failing = async () => { throw new Error("invented store failure"); };
    expect(await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, failing)).toEqual([]);
  });
});

describe("Save Model Document…: the native adapter", () => {
  async function readyArgs(): Promise<ModelDocumentSaveArgs> {
    const preparation = await prepared(demoModel);
    if (preparation.status !== "ready") throw new Error("demo refused");
    return preparation.args;
  }

  it("is desktop-only: the browser preview refuses by name and sends nothing", async () => {
    expect(await saveModelDocumentFile(await readyArgs())).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_DESKTOP_ONLY }] });
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("sends the arguments as a raw ingress body and reads each outcome", async () => {
    const args = await readyArgs();
    setTauriRuntime();
    const answers: unknown[] = [
      { outcome: "saved", file_name: "invented-demo-loop-01.json", byte_count: args.canonicalText.length, model_hash: args.claimedModelHash, replaced_existing: false, durability: "durable" },
      { outcome: "cancelled" },
      { outcome: "refused", diagnostics: [{ code: "MODEL-DOC-SAVE-HASH-MISMATCH", path: "", message: "m" }] },
      { outcome: "saved", file_name: "x.json", byte_count: 1, model_hash: "sha256:" + "0".repeat(64), replaced_existing: false, durability: "durable" },
      { outcome: "saved" }
    ];
    invokeMock.mockImplementation(() => Promise.resolve(answers.shift()));
    const saved = await saveModelDocumentFile(args);
    expect(invokeMock).toHaveBeenCalledWith("save_model_document_file", args);
    expect(modelDocumentSaveMessage(saved)).toBe(`Saved model document invented-demo-loop-01.json (${args.canonicalText.length} bytes; model hash ${args.claimedModelHash}).`);
    expect(modelDocumentSaveMessage(await saveModelDocumentFile(args))).toBe("Save model document cancelled; nothing was written.");
    expect(modelDocumentSaveMessage(await saveModelDocumentFile(args))).toBe("Save model document refused: MODEL-DOC-SAVE-HASH-MISMATCH: m");
    expect(await saveModelDocumentFile(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID }] });
    expect(await saveModelDocumentFile(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID }] });
  });

  it("names uncertain durability", () => {
    expect(modelDocumentSaveMessage({ outcome: "saved", file_name: "a.json", byte_count: 2, model_hash: "sha256:h", replaced_existing: true, durability: "saved_durability_uncertain" }))
      .toContain("durability is uncertain");
  });
});

describe("File > Save Model Document…", () => {
  async function renderApp() {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await screen.findByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`);
  }

  function saveCalls() {
    return invokeMock.mock.calls.filter(([command]) => command === "save_model_document_file");
  }

  it("is in the File menu whenever a model is open, and asks for the explicit intent before anything is chosen", async () => {
    await renderApp();
    fireEvent.click(screen.getByTestId("menu-file"));
    const item = await screen.findByTestId("menu-item-file.save-model-document");
    expect(item).toHaveTextContent("Save Model Document…");
    expect(item).toBeEnabled();
    fireEvent.click(item);
    const dialog = await screen.findByTestId("model-document-save-dialog");
    expect(within(dialog).getByText(MODEL_DOCUMENT_SAVE_INTENT_LABEL)).toBeInTheDocument();
    expect(within(dialog).getByTestId("model-document-save-intent")).not.toBeChecked();
    expect(within(dialog).getByTestId("model-document-save-confirm")).toBeDisabled();
    expect(document.activeElement).toBe(within(dialog).getByTestId("model-document-save-cancel"));
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    // Cancel: nothing is sent, the dialog closes.
    fireEvent.click(within(dialog).getByTestId("model-document-save-cancel"));
    expect(screen.queryByTestId("model-document-save-dialog")).not.toBeInTheDocument();
    // Escape cancels too.
    act(() => nativeMenuCommand("file.save-model-document"));
    await screen.findByTestId("model-document-save-dialog");
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("model-document-save-dialog")).not.toBeInTheDocument());
    expect(saveCalls()).toHaveLength(0);
  });

  it("saves the session's canonical document through the native command after the explicit intent", async () => {
    await renderApp();
    setTauriRuntime();
    const expectedText = await canonicalJsonString(demoModel);
    const expectedHash = (await hashOf(demoModel)).value;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "save_model_document_file") {
        return Promise.resolve({ outcome: "saved", file_name: "invented-demo-loop-01.json", byte_count: (args!.canonicalText as string).length, model_hash: args!.claimedModelHash, replaced_existing: false, durability: "durable" });
      }
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    act(() => nativeMenuCommand("file.save-model-document"));
    const dialog = await screen.findByTestId("model-document-save-dialog");
    fireEvent.click(within(dialog).getByTestId("model-document-save-intent"));
    fireEvent.click(within(dialog).getByTestId("model-document-save-confirm"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(
      `Saved model document invented-demo-loop-01.json (${expectedText.length} bytes; model hash ${expectedHash}).`
    ));
    expect(saveCalls()).toHaveLength(1);
    const sent = saveCalls()[0][1] as ModelDocumentSaveArgs;
    expect(sent.canonicalText).toBe(expectedText);
    expect(sent.claimedModelHash).toBe(expectedHash);
    expect(sent.screening.route_id).toBe("DOTH-MODEL-DOC-007");
    // Only the save went to the native host: no solve, no store write.
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => command !== "sync_native_shell_state")).toEqual(["save_model_document_file"]);
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`)).toBeInTheDocument();
  });

  it("treats a cancelled chooser as an outcome and names a refusal", async () => {
    await renderApp();
    setTauriRuntime();
    const answers = [{ outcome: "cancelled" }, { outcome: "refused", diagnostics: [{ code: "MODEL-FILE-SCHEMA-REFUSED", path: "schema_version", message: "invented" }] }];
    invokeMock.mockImplementation((command: string) => {
      if (command === "save_model_document_file") return Promise.resolve(answers.shift());
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    for (const expected of ["Save model document cancelled; nothing was written.", "Save model document refused: MODEL-FILE-SCHEMA-REFUSED at schema_version: invented"]) {
      act(() => nativeMenuCommand("file.save-model-document"));
      const dialog = await screen.findByTestId("model-document-save-dialog");
      fireEvent.click(within(dialog).getByTestId("model-document-save-intent"));
      fireEvent.click(within(dialog).getByTestId("model-document-save-confirm"));
      await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(expected));
    }
    expect(saveCalls()).toHaveLength(2);
  });
});

describe("Save Model Document… leaves the session as it was", () => {
  it("keeps the model, its hash, the result and the solve standing by reference", async () => {
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
    setTauriRuntime();
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "save_model_document_file") {
        return Promise.resolve({ outcome: "saved", file_name: "m.json", byte_count: 1, model_hash: args!.claimedModelHash, replaced_existing: false, durability: "durable" });
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    const before = hook.result.current;
    const spine = (session: typeof before) => [
      session.model.model, session.model.modelHash, session.model.uiModelRevision, session.model.projectSessionGeneration,
      session.results.result, session.results.currentSolvedResult, session.results.solveProof, session.results.solveJob,
      session.results.analysisRun, session.project.modelEdited
    ];
    const was = spine(before);
    act(() => before.project.requestSaveModelDocument());
    await waitFor(() => expect(hook.result.current.project.modelDocumentSaveRequest).not.toBeNull());
    act(() => hook.result.current.project.modelDocumentSaveRequest!.save(true));
    await waitFor(() => expect(hook.result.current.project.projectMessage).toContain("Saved model document m.json"));
    const now = spine(hook.result.current);
    now.forEach((value, index) => expect(value).toBe(was[index]));
    expect(hook.result.current.project.projectBusy).toBe(false);
  });
});
