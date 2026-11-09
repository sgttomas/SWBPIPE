import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { afterEach, describe, expect, it, vi } from "vitest";
import sourceModel from "../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json";
import sourceText from "../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json?raw";
import reviewCases from "../../../fixtures/caepipe_mbf/invented/desktop_export/review_cases.json";
import importReplies from "../../../fixtures/caepipe_mbf/invented/desktop_import/import_replies.json";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
const validated = vi.hoisted(() => [] as unknown[]);
vi.mock("./features/model-file/modelDocumentShape", async (importOriginal) => {
  const original = await importOriginal<typeof import("./features/model-file/modelDocumentShape")>();
  return { ...original, modelDocumentSlotValidation: (document: unknown) => {
    validated.push(document);
    return original.modelDocumentSlotValidation(document);
  } };
});

import { App } from "./App";
import { retireMbfExportSession } from "./features/caepipe-mbf/mbfExportCancel";
import { retireMbfImportSession } from "./features/caepipe-mbf/mbfImportCancel";

// Slice 6: one mounted App session joins export and import. The host is mocked: review and import payloads
// are the maintained native-produced bytes, while the save receipt below is a transport stub using their
// exact pair digests/counts. The native generator separately replays task 3 -> disk -> task 5. Neither this
// suite nor a fixture replay is an actual launched App/native witness or reference-system coverage.
type Args = Record<string, unknown>;
const reviewCase = reviewCases.cases["degc-default"];
const exportReview = JSON.parse(reviewCase.review_text);
const replies = Object.fromEntries(Object.entries(importReplies.replies).map(([key, text]) => [key, JSON.parse(text)]));
const declaration = JSON.parse(importReplies.inputs.declaration_json);
const hash = (text: string) => createHash("sha256").update(text).digest("hex");
const menu = (command: string) => act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
let epoch = 0;
const retirement = (args: Args) => ({ cancelled: 0, too_late: 0, epoch: ++epoch, retirement_id: args.retirementId });

afterEach(async () => {
  invokeMock.mockImplementation((command: string, args: Args = {}) =>
    command.startsWith("cancel_mbf_") ? Promise.resolve(retirement(args)) : Promise.resolve(null));
  cleanup();
  await retireMbfExportSession();
  await retireMbfImportSession();
  validated.length = 0;
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function connectedHost(binding: "bound" | "unbound") {
  const calls: Array<{ command: string; args: Args }> = [];
  let saved = false;
  let sourceActive = false;
  let selected = false;
  let inspectSession = false;
  const adopted = structuredClone(replies[`${binding}-adopt`]);
  invokeMock.mockImplementation(async (command: string, args: Args = {}) => {
    calls.push({ command, args });
    switch (command) {
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "cancel_mbf_export": return retirement(args);
      case "cancel_mbf_import": sourceActive = false; return retirement(args);
      case "open_model_document_file":
        return { outcome: "opened", file_name: "invented-source.json", schema_status: "current", document: structuredClone(sourceModel), byte_count: new TextEncoder().encode(sourceText).length };
      case "review_mbf_export":
        expect(args.claimedModelHash).toBe(`sha256:${hash(args.canonicalText as string)}`);
        if (!inspectSession) {
          expect(JSON.parse(args.canonicalText as string)).toEqual(sourceModel);
          expect(hash(args.canonicalText as string)).toBe(reviewCase.model_sha256);
          expect(args.settingsJson).toBe(reviewCase.settings_json);
        }
        return { outcome: "reviewed", review: structuredClone(exportReview) };
      case "save_mbf_export": {
        const reviewed = calls.filter((call) => call.command === "review_mbf_export").at(-1)!.args;
        for (const key of ["canonicalText", "claimedModelHash", "settingsJson", "expectedEpoch"]) expect(args[key]).toBe(reviewed[key]);
        expect(args.reviewed).toEqual({ mbfSha256: importReplies.inputs.saved_mbf_sha256, sidecarSha256: importReplies.inputs.saved_identity_file_sha256 });
        expect(args.screening).toMatchObject({ route_id: "DOTH-MBF-012", export_context: "local_private", explicit_local_private_intent: true, exact_payload_match: true, payload_sha256: args.claimedModelHash });
        expect(args.localFirstEvidence).toMatchObject({ route_id: "DOTH-MBF-012", explicit_local_private_intent: true });
        saved = true;
        const file = (file_name: string, byte_count: number, sha256: string) => ({ file_name, byte_count, sha256, replaced_existing: false, durability: "durable" });
        return { outcome: "saved", mbf: file("swbpipe-model.mbf", exportReview.mbf_bytes, exportReview.mbf_sha256),
          identity_file: file("swbpipe-model.swbpipe-ids.json", exportReview.sidecar_bytes, exportReview.sidecar_sha256),
          durability: "durable", path_containment: "best_effort_non_adversarial", limitation: "mocked save receipt", warnings: [] };
      }
      case "open_mbf_import":
        expect(saved).toBe(true);
        sourceActive = true;
        selected = true;
        return structuredClone(replies[`${binding}-open`]);
      case "review_mbf_import":
        expect(sourceActive).toBe(true);
        expect(args.sourceId).toBe(replies[`${binding}-open`].sourceId);
        expect(args.declaration).toEqual(declaration);
        return structuredClone(replies["unbound-declared"]);
      case "adopt_mbf_import":
        expect(selected && sourceActive).toBe(true);
        expect(args).toMatchObject({ sourceId: replies[`${binding}-open`].sourceId,
          reviewSha256: replies[binding === "bound" ? "bound-open" : "unbound-declared"].reviewSha256,
          declaration: binding === "bound" ? null : declaration });
        return adopted;
      default: throw new Error(`Unexpected command ${command}`);
    }
  });
  const named = (command: string) => calls.filter((call) => call.command === command);
  return { calls, named, adopted, inspect: () => { inspectSession = true; } };
}

async function openExport() {
  // The model hash settles asynchronously after Open or an edit; the public action explicitly permits retry.
  for (let attempt = 0; attempt < 40; attempt++) {
    menu("file.export-mbf");
    try { return await screen.findByTestId("mbf-export-dialog", {}, { timeout: 500 }); } catch { /* hash pending */ }
  }
  throw new Error("Export did not open after its model hash settled");
}

async function closeExport(dialog: HTMLElement) {
  fireEvent.click(within(dialog).getByTestId("mbf-export-cancel"));
  await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).not.toBeInTheDocument());
}

async function saveSource(host: ReturnType<typeof connectedHost>) {
  expect(hash(sourceText)).toBe(importReplies.inputs.source_model_file_sha256);
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(/^tree-row-project-/, {}, { timeout: 20000 });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  menu("file.open-model");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(sourceModel.project.id)}`, {}, { timeout: 20000 });
  const dialog = await openExport();
  await within(dialog).findByTestId("mbf-export-profile");
  expect(exportReview.completeness).toBe("complete");
  expect(exportReview.output_ceiling).toBe("within");
  expect(exportReview.mbf_sha256).toBe(importReplies.inputs.saved_mbf_sha256);
  expect(exportReview.sidecar_sha256).toBe(importReplies.inputs.saved_identity_file_sha256);
  expect(reviewCase.settings_json).toBe(importReplies.inputs.export_settings_json);
  const save = within(dialog).getByTestId("mbf-export-save");
  expect(save).toBeDisabled();
  fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
  await waitFor(() => expect(save).toBeEnabled());
  fireEvent.click(save);
  await waitFor(() => expect(within(dialog).getByTestId("mbf-export-status")).toHaveTextContent("Saved swbpipe-model.mbf and its identity file"));
  const before = host.named("cancel_mbf_export").length;
  await closeExport(dialog);
  await waitFor(() => expect(host.named("cancel_mbf_export")).toHaveLength(before + 1));
}

async function openImport() {
  menu("file.import-mbf");
  const dialog = await screen.findByTestId("mbf-import-dialog");
  await within(dialog).findByTestId("mbf-import-review");
  return dialog;
}

/** Read the committed model through the real export entry, rather than observing only the validator argument. */
async function sessionModel(host: ReturnType<typeof connectedHost>) {
  host.inspect();
  const before = host.named("review_mbf_export").length;
  const dialog = await openExport();
  await waitFor(() => expect(host.named("review_mbf_export")).toHaveLength(before + 1));
  const model = JSON.parse(host.named("review_mbf_export").at(-1)!.args.canonicalText as string);
  await closeExport(dialog);
  return model;
}

describe("connected supported-subset MBF App round trip (mocked native host)", () => {
  it.each(["bound", "unbound"] as const)("exports the selected source, imports %s and commits the native candidate through Open", async (binding) => {
    const host = connectedHost(binding);
    await saveSource(host);
    const dialog = await openImport();
    const identity = within(dialog).getByTestId("mbf-import-identity");
    expect(identity).toHaveTextContent("These counts describe this import. They do not establish that every original source identity was preserved.");
    expect(identity).toHaveTextContent("The loss report also includes fields and other information the file cannot recover.");
    const initialReview = replies[`${binding}-open`].review;
    expect(initialReview.format).toBe(2);
    expect(initialReview.identity.unrecoverable).toBe(0);
    expect(initialReview.identity.unrecovered_fields).toBe(binding === "bound" ? 1 : 0);
    expect(initialReview.identity.source_completeness).toBe("not_established");
    // Keep both the warning payload and its displayed entries: the caveat supplies no completeness claim
    // even when every class is restored, or when an unbound import has zero known missing ids.
    if (binding === "bound") {
      expect(identity).toHaveTextContent("Identity file applied; each class lists restored and formed ids.");
      expect(identity).toHaveTextContent("node: 5 (5 restored, 0 formed)");
      const loss = within(dialog).getByTestId("mbf-import-loss");
      expect(within(loss).getAllByRole("listitem")).toHaveLength(initialReview.loss_entries.length);
      expect(loss).toHaveTextContent("MBF-IMPORT-REFERENCE-UNRECOVERABLE (approximated, component:bend-1.node)");
      expect(loss).toHaveTextContent("MBF-IMPORT-SOURCE-COMPLETENESS-UNESTABLISHED (approximated, unexported members)");
    } else {
      expect(identity).toHaveTextContent("Ids formed from node numbers.");
    }
    const before = host.named("cancel_mbf_import").length;
    const adopt = within(dialog).getByTestId("mbf-import-adopt");
    if (binding === "unbound") {
      expect(adopt).toBeDisabled();
      fireEvent.change(within(dialog).getByRole("combobox", { name: /Source of / }), { target: { value: "invented" } });
      await waitFor(() => expect(host.named("review_mbf_import")).toHaveLength(1));
    }
    await waitFor(() => expect(adopt).toBeEnabled());
    if (binding === "unbound") {
      expect(replies["unbound-declared"].review.identity.unrecoverable).toBe(0);
      expect(identity).toHaveTextContent("node: 5 (0 restored, 5 formed)");
      expect(identity).toHaveTextContent("They do not establish that every original source identity was preserved.");
    }
    validated.length = 0;
    fireEvent.click(adopt);
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Not saved as a local project yet."));
    expect(validated).toHaveLength(1);
    expect(validated[0]).toBe(host.adopted.document);
    await waitFor(() => expect(host.named("cancel_mbf_import")).toHaveLength(before + 1));
    await act(async () => {}); // Include the retiring dialog's passive cleanup.
    expect(host.named("cancel_mbf_import")).toHaveLength(before + 1);
    expect(await sessionModel(host)).toEqual(host.adopted.document);
    expect(host.named("save_mbf_export")).toHaveLength(1);
    // Import requests carry identities/declarations only: the chooser's path and file bytes stay native.
    const allowed: Record<string, string[]> = {
      open_mbf_import: ["expectedEpoch", "requestSequence"],
      review_mbf_import: ["expectedEpoch", "requestSequence", "sourceId", "declaration"],
      adopt_mbf_import: ["expectedEpoch", "requestSequence", "sourceId", "declaration", "reviewSha256"]
    };
    for (const { command, args } of host.calls) if (command in allowed) expect(Object.keys(args).every((key) => allowed[command].includes(key))).toBe(true);
  }, 60000);

  it("keeps later unsaved edits when Open's guard cancels the import of the earlier export", async () => {
    const host = connectedHost("bound");
    await saveSource(host);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const disclosure = screen.getByTestId("node-grid-review-disclosure");
    if (disclosure.getAttribute("aria-expanded") !== "true") fireEvent.click(disclosure);
    fireEvent.doubleClick(screen.getByTestId("review-cell-node:A-y"));
    fireEvent.change(screen.getByTestId("engineering-table-review").querySelector("input")!, { target: { value: "0.5" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    fireEvent.click(screen.getAllByTestId(/^apply-intent-editor-intent-\d+$/).at(-1)!);
    await waitFor(() => expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited"));
    const edited = await sessionModel(host);
    expect(edited.nodes.find((node: { id: string }) => node.id === "node:A").position.y).toBe(0.5);
    const dialog = await openImport();
    const before = host.named("cancel_mbf_import").length;
    validated.length = 0;
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await screen.findByTestId("unsaved-changes-prompt");
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Not opened"));
    expect(host.named("adopt_mbf_import")).toHaveLength(0);
    expect(validated).toHaveLength(0);
    await waitFor(() => expect(host.named("cancel_mbf_import")).toHaveLength(before + 1));
    expect(await sessionModel(host)).toEqual(edited);
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
  }, 60000);

  it("refuses a stale Adopt after export and preserves the complete source model in the session", async () => {
    const host = connectedHost("bound");
    await saveSource(host);
    host.adopted.modelSha256 = "0".repeat(64);
    const dialog = await openImport();
    const before = host.named("cancel_mbf_import").length;
    validated.length = 0;
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("MBF-IMPORT-REVIEW-STALE"));
    expect(validated).toHaveLength(0);
    await waitFor(() => expect(host.named("cancel_mbf_import")).toHaveLength(before + 1));
    expect(await sessionModel(host)).toEqual(sourceModel);
  }, 60000);
});
