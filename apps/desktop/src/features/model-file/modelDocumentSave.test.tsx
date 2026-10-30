import { act, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import dispositionParity from "../../../../../fixtures/model_document/library_disposition_parity.json";
import { SHAPE_CASES, shapeCaseDocument } from "../../test/shapeCorpus";
import desktopEmitted from "../../../../../fixtures/model_document/desktop_emitted.json";
import applierEmitted from "../../../../../fixtures/model_document/applier_emitted.json";
import { App } from "../../App";
import { checkNativeOutcome } from "../../services/modelDocumentFile";
import { buildModelIndex } from "../workspace/modelIndex";
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
  type LibraryResolution,
  type ResolvedLibraryDisposition,
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
      blocking_count: 0,
      payload_sha256: `sha256:${createHash("sha256").update(preparation.args.canonicalText).digest("hex")}`
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

  const resolvedOnly = (dispositions: ResolvedLibraryDisposition[]): LibraryResolution => ({ dispositions, warnings: [], storeError: null });

  it("refuses a resolved library disposition that is quarantined, naming the library record", async () => {
    const preparation = await prepared(everySlotModel, {
      libraries: resolvedOnly([{ library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "public_permissive", review_status: "quarantined" }])
    });
    expect(codes(preparation)).toEqual([["PROTECTED_CONTENT_BLOCKED", "library_ref(material:library:invented-materials#steel)"]]);
  });

  it("saves private_only and unknown dispositions", async () => {
    const model = everySlot();
    model.materials[0].provenance_record.redistribution_status = "private_only";
    model.sections[0].provenance_record.redistribution_status = "unknown";
    model.rule_pack_refs[0].redistribution_status = "private_only";
    const preparation = await prepared(model, {
      libraries: resolvedOnly([
        { library_kind: "material", library_id: "library:invented-materials", record_id: null, redistribution_status: "private_only", review_status: "accepted" },
        { library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "unknown", review_status: null }
      ])
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

  function storeOf(document: Record<string, unknown> | null, opened: unknown[] = []) {
    return async (projectId: string, kind: string, id: string): Promise<LibraryOpenRoute> => {
      opened.push([projectId, kind, id]);
      return {
        route: "tauri_backend",
        envelope: document && {
          project_id: projectId, library_kind: "material", library_id: id, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0,
          document, validation: {} as never, message: ""
        }
      } as LibraryOpenRoute;
    };
  }
  const refPath = "materials[0].provenance_record.library_ref";

  it("resolves library references against the local store with the library import's own-field precedence", async () => {
    const opened: unknown[] = [];
    const resolution = await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, storeOf({
      material_library: { library_id: "library:invented-materials", provenance: { redistribution_status: "private_only", review_status: "accepted" } },
      material_records: [{ material_id: "steel", provenance: { review_status: "quarantined" } }]
    }, opened));
    expect(opened).toEqual([[everySlotModel.project.id, "material", "library:invented-materials"]]);
    expect(resolution).toEqual({
      dispositions: [
        { path: refPath, library_kind: "material", library_id: "library:invented-materials", record_id: null, redistribution_status: "private_only", review_status: "accepted" },
        { path: refPath, library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: null, review_status: "quarantined" }
      ],
      warnings: [],
      storeError: null
    });
    expect(codes(await prepared(everySlotModel, { libraries: resolution }))).toEqual([["PROTECTED_CONTENT_BLOCKED", refPath]]);
    expect(await resolveLibraryDispositions(demoModel as unknown as PreviewModel, storeOf(null))).toEqual({ dispositions: [], warnings: [], storeError: null });
  });

  // Review repair 1, findings 2-5 (the reviewer's regressions, adapted to the
  // resolution object; "library missing" follows the coordinator's ruling: it warns, it does not refuse).
  it("refuses when the library lookup throws (store unavailable)", async () => {
    const libraries = await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, async () => { throw new Error("store unavailable"); });
    expect(codes(await prepared(everySlotModel, { libraries }))).toEqual([["MODEL-DOC-SAVE-LIBRARY-STORE-UNAVAILABLE", refPath]]);
  });

  it("saves with a MODEL-LIBRARY-REF-UNRESOLVED warning when the store holds no such library or record", async () => {
    for (const document of [null, { material_library: { provenance: { review_status: "accepted" } }, material_records: [] }]) {
      const libraries = await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, storeOf(document));
      const preparation = await prepared(everySlotModel, { libraries });
      expect(preparation.status).toBe("ready");
      expect(preparation.status === "ready" && preparation.warnings.map(({ code, path }) => [code, path])).toEqual([["MODEL-LIBRARY-REF-UNRESOLVED", refPath]]);
    }
  });

  it("screens a quarantined provenance record nested below another provenance record", async () => {
    const d = everySlot();
    d.review_extension = { provenance_record: { source_name: "outer", review_status: "accepted", nested: { provenance_record: { source_name: "inner", review_status: "quarantined" } } } };
    expect(codes(await prepared(d))).toEqual([["PROTECTED_CONTENT_BLOCKED", "review_extension.provenance_record.nested.provenance_record"]]);
  });

  it("lets a library's own explicit protected field override its accepted provenance", async () => {
    const libraries = await resolveLibraryDispositions(everySlotModel as unknown as PreviewModel, storeOf({
      material_library: { redistribution_status: "protected_suspected", review_status: "quarantined", provenance: { redistribution_status: "private_only", review_status: "accepted" } },
      material_records: []
    }));
    expect(codes(await prepared(everySlotModel, { libraries }))).toEqual([["PROTECTED_CONTENT_BLOCKED", refPath]]);
  });

  it("blocks a rejected redistribution on a resolved library even when its review is accepted", async () => {
    const preparation = await prepared(everySlotModel, {
      libraries: resolvedOnly([{ library_kind: "material", library_id: "library:invented-materials", record_id: "steel", redistribution_status: "rejected", review_status: "accepted" }])
    });
    expect(codes(preparation)).toEqual([["PROTECTED_CONTENT_BLOCKED", "library_ref(material:library:invented-materials#steel)"]]);
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
      { outcome: "saved", file_name: "invented-demo-loop-01.json", byte_count: args.canonicalText.length, model_hash: args.claimedModelHash, replaced_existing: false, durability: "durable", warnings: [] },
      { outcome: "cancelled" },
      { outcome: "refused", diagnostics: [{ code: "MODEL-DOC-SAVE-HASH-MISMATCH", path: "", message: "m" }] },
      { outcome: "saved", file_name: "x.json", byte_count: 1, model_hash: "sha256:" + "0".repeat(64), replaced_existing: false, durability: "durable", warnings: [] },
      { outcome: "saved" },
      { outcome: "saved", file_name: "y.json", byte_count: 1, model_hash: args.claimedModelHash, replaced_existing: false, durability: "durable" }
    ];
    invokeMock.mockImplementation(() => Promise.resolve(answers.shift()));
    const saved = await saveModelDocumentFile(args);
    expect(invokeMock).toHaveBeenCalledWith("save_model_document_file", args);
    expect(modelDocumentSaveMessage(saved)).toBe(`Saved model document invented-demo-loop-01.json (${args.canonicalText.length} bytes; model hash ${args.claimedModelHash}).`);
    expect(modelDocumentSaveMessage(await saveModelDocumentFile(args))).toBe("Save model document cancelled; nothing was written.");
    expect(modelDocumentSaveMessage(await saveModelDocumentFile(args))).toBe("Save model document refused: MODEL-DOC-SAVE-HASH-MISMATCH: m");
    expect(await saveModelDocumentFile(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID }] });
    expect(await saveModelDocumentFile(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID }] });
    // A saved outcome without its warnings list is not read as saved.
    expect(await saveModelDocumentFile(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID }] });
  });

  it("names uncertain durability", () => {
    expect(modelDocumentSaveMessage({ outcome: "saved", file_name: "a.json", byte_count: 2, model_hash: "sha256:h", replaced_existing: true, durability: "saved_durability_uncertain", warnings: [] }))
      .toContain("durability is uncertain");
    expect(modelDocumentSaveMessage({ outcome: "saved", file_name: "a.json", byte_count: 2, model_hash: "sha256:h", replaced_existing: false, durability: "durable",
      warnings: [{ code: "MODEL-LIBRARY-REF-UNRESOLVED", path: "materials[0].provenance_record.library_ref", message: "m" }] }))
      .toBe("Saved model document a.json (2 bytes; model hash sha256:h). Warnings: MODEL-LIBRARY-REF-UNRESOLVED at materials[0].provenance_record.library_ref.");
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
        return Promise.resolve({ outcome: "saved", file_name: "invented-demo-loop-01.json", byte_count: (args!.canonicalText as string).length, model_hash: args!.claimedModelHash, replaced_existing: false, durability: "durable", warnings: [] });
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
    // Only the save went to the native host: no solve, no store write. D5 autosave's settings read and
    // record listing (RT-U3) are reads; any autosave write or discard still fails this.
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => command !== "sync_native_shell_state" && command !== "get_autosave_settings" && command !== "list_project_autosaves")).toEqual(["save_model_document_file"]);
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
        return Promise.resolve({ outcome: "saved", file_name: "m.json", byte_count: 1, model_hash: args!.claimedModelHash, replaced_existing: false, durability: "durable", warnings: [] });
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

// The independent reviewer's serializer witnesses (W-6a review round 1).
describe("Save Model Document…: serializer witnesses", () => {
  const probes: Record<string, unknown> = {
    unicode: "é中😀", escapes: "\n\t\"\\", large: 1e30, small: 1e-30, negativeZero: -0,
    keyOrder: { "😀": 1, "é": 2, a: 3, "中": 4 }, unsafeInt: 1000000000000000100
  };
  it.each(Object.entries(probes))("%s: the canonical text, no trailing newline, its SHA-256 is the claim", async (_label, probe) => {
    const d = everySlot();
    d.review_probe = probe;
    const preparation = await prepared(d);
    expect(preparation.status).toBe("ready");
    if (preparation.status !== "ready") return;
    expect(preparation.args.canonicalText).toBe(await canonicalJsonString(d));
    expect(preparation.args.canonicalText.endsWith("\n")).toBe(false);
    expect(`sha256:${createHash("sha256").update(preparation.args.canonicalText).digest("hex")}`).toBe(preparation.args.claimedModelHash);
    expect(preparation.args.screening.payload_sha256).toBe(preparation.args.claimedModelHash);
  });

  it("a lone surrogate refuses before any native invoke", async () => {
    const d = everySlot();
    d.review_probe = "\ud800";
    await expect(prepared(d)).rejects.toThrow();
  });
});

// Independent review round 2.
describe("Save Model Document…: review round 2", () => {
  const refPath = "materials[0].provenance_record.library_ref";
  function modelReferencing(reference: Record<string, unknown>) {
    const d = everySlot();
    d.materials[0].provenance_record.library_ref = reference;
    return d as unknown as PreviewModel;
  }
  function storeAnswering(match: (kind: string, id: string) => Record<string, unknown> | null) {
    return async (projectId: string, kind: string, id: string): Promise<LibraryOpenRoute> => {
      const document = match(kind, id);
      return { route: "tauri_backend", envelope: document && { project_id: projectId, library_kind: kind, library_id: id, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0, document, validation: {} as never, message: "" } } as LibraryOpenRoute;
    };
  }
  const blockedAt = (model: PreviewModel, libraries: LibraryResolution) =>
    screenModelDocumentDispositions(model, { explicitLocalPrivateIntent: true, resolvedLibraries: libraries.dispositions })
      .blocking.filter((item) => item.code === "PROTECTED_CONTENT_BLOCKED").map((item) => item.path);

  it("normalizes library identities as the store's open path does", async () => {
    const model = modelReferencing({ library_kind: "material", library_id: " library:invented-materials ", record_id: " steel " });
    const opened: string[] = [];
    const libraries = await resolveLibraryDispositions(model, storeAnswering((_kind, id) => {
      opened.push(id);
      return id === "library:invented-materials" ? { material_library: { review_status: "accepted" }, material_records: [{ material_id: "steel", review_status: "quarantined" }] } : null;
    }));
    expect(opened).toEqual(["library:invented-materials"]);
    expect(blockedAt(model, libraries)).toEqual([refPath]);
  });

  it("screens every record with the referenced id: a clear first record does not hide a quarantined duplicate", async () => {
    const model = modelReferencing({ library_kind: "material", library_id: "library:invented-materials", record_id: "steel" });
    const libraries = await resolveLibraryDispositions(model, storeAnswering(() => ({
      material_library: { review_status: "accepted" },
      material_records: [{ material_id: "steel", review_status: "accepted" }, { material_id: "steel", review_status: "quarantined" }]
    })));
    expect(blockedAt(model, libraries)).toEqual([refPath]);
  });

  it("applies the hanger import's conflict rule (own accepted over a quarantined provenance blocks)", async () => {
    const model = modelReferencing({ library_kind: "hanger", library_id: "invented.hangers", record_id: "invented.variable" });
    const libraries = await resolveLibraryDispositions(model, storeAnswering(() => ({
      hanger_library: { library_id: "invented.hangers", provenance: { redistribution_status: "private_only", review_status: "accepted" } },
      hanger_records: [{ hanger_id: "invented.variable", review_status: "accepted", provenance: { redistribution_status: "private_only", review_status: "quarantined" } }]
    })));
    expect(blockedAt(model, libraries)).toEqual([refPath]);
  });

  it("matches the library import's verdict on every row of the shared parity table", async () => {
    const idKeys: Record<string, string> = { material: "material_id", section: "section_id", component: "component_id", hanger: "hanger_id" };
    expect(dispositionParity.rows).toHaveLength(1440);
    const mismatches: unknown[] = [];
    for (const row of dispositionParity.rows) {
      const [kind, level, ownRedistribution, ownReview, provenanceRedistribution, provenanceReview, blocked] = row as [string, string, string | null, string | null, string, string, boolean];
      const clear = { redistribution_status: "private_only", review_status: "accepted" };
      const varied: Record<string, unknown> = { provenance: { redistribution_status: provenanceRedistribution, review_status: provenanceReview } };
      if (ownRedistribution !== null) varied.redistribution_status = ownRedistribution;
      if (ownReview !== null) varied.review_status = ownReview;
      const library = { library_id: "invented.parity", provenance: clear, ...(level === "library" ? varied : {}) };
      const record = { [idKeys[kind]]: "invented.record", provenance: clear, ...(level === "record" ? varied : {}) };
      const model = modelReferencing({ library_kind: kind, library_id: "invented.parity", record_id: "invented.record" });
      const libraries = await resolveLibraryDispositions(model, storeAnswering(() => ({ [`${kind}_library`]: library, [`${kind}_records`]: [record] })));
      if ((blockedAt(model, libraries).length > 0) !== blocked) mismatches.push(row);
    }
    expect(mismatches).toEqual([]);
  });

  it("opens a refused section_ref by name instead of failing to index", () => {
    const d = everySlot();
    d.pipe_segments[0].section_ref = 17;
    const checked = checkNativeOutcome({ outcome: "opened", file_name: "n.json", byte_count: 1, schema_status: "current", document: d });
    expect(checked.outcome === "refused" && checked.diagnostics.map(({ code, path }) => [code, path])).toEqual([["MODEL-FILE-SHAPE", "pipe_segments[0].section_ref"]]);
  });

  it("opens every shape-clean case of the shared corpus through the webview's open checks and model index", () => {
    const clean = SHAPE_CASES.filter((item) => item.expected.length === 0);
    expect(clean.length).toBeGreaterThanOrEqual(4);
    for (const item of clean) {
      const checked = checkNativeOutcome({ outcome: "opened", file_name: `${item.case_id}.json`, byte_count: 1, schema_status: "current", document: shapeCaseDocument(item) });
      expect(checked.outcome, item.case_id).toBe("opened");
      if (checked.outcome === "opened") expect(() => buildModelIndex(checked.document, 0, 0), item.case_id).not.toThrow();
    }
    for (const item of SHAPE_CASES.filter((entry) => entry.expected.length > 0)) {
      const checked = checkNativeOutcome({ outcome: "opened", file_name: `${item.case_id}.json`, byte_count: 1, schema_status: "current", document: shapeCaseDocument(item) });
      expect(checked.outcome, item.case_id).toBe("refused");
    }
  });
});

// Independent review round 3 (the reviewer's regression): these documents pass
// buildModelIndex but crashed the workspace on adoption; preparation refuses them.
describe("adoption render preconditions", () => {
  it.each(["unit-value", "component-completeness", "diagnostic-id"])("refuses %s", async (scenario) => {
    const model = clone(demoModel) as any;
    if (scenario === "unit-value") model.project.units.force = 17;
    if (scenario === "component-completeness") model.components[0].completeness = {};
    if (scenario === "diagnostic-id") model.diagnostics[0].id = {};
    expect(() => buildModelIndex(model, 0, 0)).not.toThrow();
    expect(codes(await prepared(model))).toEqual([["MODEL-FILE-SHAPE", { "unit-value": "project.units.force", "component-completeness": "components[0].completeness", "diagnostic-id": "diagnostics[0].id" }[scenario]]]);
  });
});

// Main's authoring and library-apply outputs (W-13 S2a/S2b emission goldens,
// including `library_material_insert`): each passes open's admission, indexes and
// is ready to save.
describe("emitted documents", () => {
  const emitted = [
    ...desktopEmitted.entries.map((entry) => [`desktop ${entry.path}`, entry.document] as const),
    ...applierEmitted.entries.map((entry, index) => [`applier ${index} ${entry.change_kind}`, entry.applied_model] as const)
  ];
  it("has all 36", () => expect(emitted).toHaveLength(36));
  it.each(emitted)("%s opens and is ready to save", async (_label, document) => {
    const checked = checkNativeOutcome({ outcome: "opened", file_name: "emitted.json", byte_count: 1, schema_status: "current", document: clone(document) });
    expect(checked.outcome === "refused" ? checked.diagnostics : []).toEqual([]);
    expect(() => buildModelIndex(document as unknown as PreviewModel, 0, 0)).not.toThrow();
    expect(codes(await prepared(clone(document)))).toBe("ready");
  });
});
