import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import importReplies from "../../../fixtures/caepipe_mbf/invented/desktop_import/import_replies.json";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// The Open consumer's one webview validator, counted with the document it is given (the shared path is unchanged).
const validatorCalls = vi.hoisted(() => [] as unknown[]);
vi.mock("./features/model-file/modelDocumentShape", async (importOriginal) => {
  const original = await importOriginal<typeof import("./features/model-file/modelDocumentShape")>();
  return {
    ...original,
    modelDocumentSlotValidation: (document: unknown) => {
      validatorCalls.push(document);
      return original.modelDocumentSlotValidation(document);
    }
  };
});

import demoModel from "../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "./App";
import { retireMbfImportSession } from "./features/caepipe-mbf/mbfImportCancel";
import { MBF_IMPORT_DESCRIPTIONS } from "./features/caepipe-mbf/mbfImport";

// DEL-17-04 Design `desktop-mbf-export-import.md` §4.1, §4.8.5–§4.8.7 and §4.8.9 (slice 5): File > Import MBF… in the
// real App, against a native host whose replies are the actual native replies for the export's saved pair
// (`import_replies.json` captures verbatim response strings, held to task 5's bytes by `mbf_import_tests.rs`;
// its companion explicitly covers the invented transcript and inputs, not the inner runtime provenance).
// Parse those strings unchanged, with no reconstructed/enriched model. Adopt goes through the maintained Open
// transaction: one `checkNativeOutcome` (one webview validator call, given the reply's own document object), the
// unsaved-changes guard, the index guard and the commit; refusals leave the session as it was.

type Args = Record<string, unknown>;
type Reply = Record<string, unknown>;
const REPLIES = Object.fromEntries(Object.entries(importReplies.replies).map(([name, text]) => [name, JSON.parse(text)])) as Record<string, Reply>;
const DECLARATION = JSON.parse(importReplies.inputs.declaration_json) as Record<string, unknown>;
const reply = (name: string): Reply => structuredClone(REPLIES[name]);
const DEMO_ROW = `tree-row-project-${encodeURIComponent(demoModel.project.id)}`;
const importedDocument = (REPLIES["bound-adopt"].document as { project: { id: string } });
const IMPORTED_ROW = `tree-row-project-${encodeURIComponent(importedDocument.project.id)}`;

let nativeEpoch = 0;
afterEach(async () => {
  invokeMock.mockImplementation((command: string, args?: Args) =>
    command === "cancel_mbf_import" || command === "cancel_mbf_export"
      ? Promise.resolve({ cancelled: 0, too_late: 0, epoch: ++nativeEpoch, retirement_id: args?.retirementId })
      : Promise.resolve(null));
  cleanup();
  await retireMbfImportSession();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  validatorCalls.length = 0;
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

type Held = { command: string; args: Args; resolve: (value: unknown) => void; reject: (error: unknown) => void };

function nativeHost(answers: Partial<Record<"open_mbf_import" | "review_mbf_import" | "adopt_mbf_import" | "open_model_document_file", (args: Args) => unknown>>, hold: ReadonlySet<string> = new Set()) {
  const commands: Array<{ command: string; args: Args; raw: unknown }> = [];
  const held: Held[] = [];
  invokeMock.mockImplementation((command: string, raw?: Args) => {
    const args = raw ?? {};
    commands.push({ command, args, raw });
    if (hold.has(command)) return new Promise((resolve, reject) => held.push({ command, args, resolve, reject }));
    if (command === "cancel_mbf_import" || command === "cancel_mbf_export") {
      return Promise.resolve({ cancelled: 0, too_late: 0, epoch: ++nativeEpoch, retirement_id: args.retirementId });
    }
    const answer = answers[command as keyof typeof answers];
    if (answer) {
      try {
        return Promise.resolve(answer(args));
      } catch (error) {
        return Promise.reject(error);
      }
    }
    if (command === "check_model_library_refs") return Promise.resolve([]);
    return Promise.resolve(null);
  });
  const named = (command: string) => commands.filter((item) => item.command === command);
  return { commands, held, named, count: (command: string) => named(command).length };
}

const nativeMenuCommand = (command: string) => window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
const setTauriRuntime = () => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; };
const projectMessage = () => screen.getByTestId("local-project-message");
const toolbarProject = () => screen.getByTestId("toolbar-project-name");

async function renderApp() {
  const view = render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(DEMO_ROW, {}, { timeout: 20000 });
  return view;
}

async function openImport() {
  act(() => nativeMenuCommand("file.import-mbf"));
  return screen.findByTestId("mbf-import-dialog");
}

async function reviewShown() {
  await waitFor(() => expect(screen.getByTestId("mbf-import-review")).toBeInTheDocument());
}

function expectDemoSession() {
  expect(screen.getByTestId(DEMO_ROW)).toBeInTheDocument();
  expect(screen.queryByTestId(IMPORTED_ROW)).not.toBeInTheDocument();
  expect(toolbarProject()).toHaveTextContent(demoModel.project.name);
}

/** No start carries a path, file bytes or base64, and no member beyond its own. */
function expectNoFileContent(host: ReturnType<typeof nativeHost>) {
  const allowed: Record<string, string[]> = {
    open_mbf_import: ["expectedEpoch", "requestSequence"],
    review_mbf_import: ["expectedEpoch", "requestSequence", "sourceId", "declaration"],
    adopt_mbf_import: ["expectedEpoch", "requestSequence", "sourceId", "declaration", "reviewSha256"]
  };
  for (const { command, args } of host.commands) {
    if (!(command in allowed)) continue;
    for (const key of Object.keys(args)) expect(allowed[command]).toContain(key);
  }
}

describe("File > Import MBF… in the App", () => {
  it("keeps the raw capture's saved-source and review bindings when parsed for the host", () => {
    const bound = REPLIES["bound-open"];
    const declared = REPLIES["unbound-declared"];
    for (const name of ["bound-open", "unbound-open", "unbound-declared"]) {
      const review = REPLIES[name].review as { source: { mbf_sha256: string } };
      expect(review.source.mbf_sha256).toBe(importReplies.inputs.saved_mbf_sha256);
    }
    expect((bound.review as { source: { identity_file: { sha256: string } } }).source.identity_file.sha256).toBe(importReplies.inputs.saved_identity_file_sha256);
    expect(REPLIES["bound-adopt"].reviewSha256).toBe(bound.reviewSha256);
    expect(REPLIES["unbound-adopt"].reviewSha256).toBe(declared.reviewSha256);
  });

  it("refuses an old review before showing format-2 identities or adopting, and still retires on Close", async () => {
    await renderApp();
    setTauriRuntime();
    const old = reply("bound-open");
    // A foreign format-1 transport reply, not a reconstructed format-2 native capture.
    old.review = { document_kind: "swbpipe.mbf_import_review", format: 1, adoptable: true };
    const host = nativeHost({ open_mbf_import: () => old });
    const dialog = await openImport();
    await waitFor(() => expect(dialog).toHaveTextContent("MBF-IMPORT-WORKER-PROTOCOL"));
    expect(within(dialog).queryByTestId("mbf-import-identity")).not.toBeInTheDocument();
    expect(within(dialog).getByTestId("mbf-import-adopt")).toBeDisabled();
    expect(host.count("adopt_mbf_import")).toBe(0);
    expectDemoSession();
    const before = host.count("cancel_mbf_import");
    fireEvent.click(within(dialog).getByRole("button", { name: "Close" }));
    await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(before + 1));
    expectDemoSession();
  });

  it("opens from the native menu and the in-window File menu, shows the native review and adopts the bound import through the Open transaction", async () => {
    await renderApp();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("bound-open"), adopt_mbf_import: () => reply("bound-adopt") });
    const dialog = await openImport();
    await reviewShown();
    expect(within(dialog).getByTestId("mbf-import-binding")).toHaveTextContent("Bound");
    expect(within(dialog).getByTestId("mbf-import-completeness-state")).toHaveTextContent("Complete");
    const assumptions = within(dialog).getByTestId("mbf-import-assumptions");
    expect(assumptions).toHaveTextContent("Node positions are summed from the printed offsets as exact rationals");
    expect(assumptions).not.toHaveTextContent("CAEPIPE");
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    // While the dialog holds the session, a second Import MBF… starts nothing.
    act(() => nativeMenuCommand("file.import-mbf"));
    expect(host.count("open_mbf_import")).toBe(1);
    const retirementsBefore = host.count("cancel_mbf_import");
    validatorCalls.length = 0;
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(screen.getByTestId(IMPORTED_ROW)).toBeInTheDocument(), { timeout: 20000 });
    expect(screen.queryByTestId("mbf-import-dialog")).not.toBeInTheDocument();
    expect(projectMessage()).toHaveTextContent("Imported model batch file swbpipe-model.mbf (bound by its identity file;");
    expect(projectMessage()).toHaveTextContent("Not saved as a local project yet.");
    // The adopt request carried the reviewed digest and the source; the reply's document object reached the one
    // webview validator unchanged (no clone, no second call).
    const adoptCall = host.named("adopt_mbf_import")[0].args;
    expect(adoptCall).toMatchObject({ sourceId: REPLIES["bound-open"].sourceId, declaration: null, reviewSha256: REPLIES["bound-open"].reviewSha256 });
    expect(validatorCalls).toHaveLength(1);
    // The source is retired after Open returns (one logical retirement).
    await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(retirementsBefore + 1));
    await act(async () => {}); // Flush the retiring generation's passive cleanup too.
    expect(host.count("cancel_mbf_import")).toBe(retirementsBefore + 1);
    expectNoFileContent(host);
  });

  it("gives the Open consumer the same document object, one validator call and the same adoption as ordinary Open", async () => {
    const adopted = reply("bound-adopt");
    const opened = { outcome: "opened", file_name: "same.json", byte_count: 1, schema_status: "current", document: reply("bound-adopt").document };
    await renderApp();
    setTauriRuntime();
    nativeHost({ open_model_document_file: () => opened });
    validatorCalls.length = 0;
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId(IMPORTED_ROW)).toBeInTheDocument(), { timeout: 20000 });
    expect(validatorCalls).toEqual([opened.document]);
    expect(validatorCalls[0]).toBe(opened.document);
    cleanup();
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    await renderApp();
    setTauriRuntime();
    nativeHost({ open_mbf_import: () => reply("bound-open"), adopt_mbf_import: () => adopted });
    const dialog = await openImport();
    await reviewShown();
    validatorCalls.length = 0;
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(screen.getByTestId(IMPORTED_ROW)).toBeInTheDocument(), { timeout: 20000 });
    expect(validatorCalls).toHaveLength(1);
    expect(validatorCalls[0]).toBe(adopted.document);
  });

  it("declares per listed material, re-reviews, holds Adopt until the declared review returns, and adopts unbound", async () => {
    await renderApp();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("unbound-open"), adopt_mbf_import: () => reply("unbound-adopt") }, new Set(["review_mbf_import"]));
    const dialog = await openImport();
    await reviewShown();
    expect(within(dialog).getByTestId("mbf-import-binding")).toHaveTextContent("no identity file");
    expect(within(dialog).getByTestId("mbf-import-adopt")).toBeDisabled();
    const declaration = within(dialog).getByTestId("mbf-import-declaration");
    const select = within(declaration).getByRole("combobox", { name: /Source of / });
    const offered = within(select).getAllByRole("option").map((option) => option.textContent);
    expect(offered).toEqual(["Not declared", "Project-invented example data", "My own source"]);
    fireEvent.change(select, { target: { value: "invented" } });
    await waitFor(() => expect(host.held).toHaveLength(1));
    const sent = host.held[0].args.declaration as Record<string, unknown>;
    expect(sent).toEqual(DECLARATION);
    expect(Object.keys(sent)).toEqual(Object.keys(DECLARATION));
    expect(Object.keys(sent)).not.toContain("*");
    expect(within(dialog).getByTestId("mbf-import-adopt")).toBeDisabled();
    await act(async () => { host.held[0].resolve(reply("unbound-declared")); });
    await waitFor(() => expect(within(dialog).getByTestId("mbf-import-adopt")).toBeEnabled());
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("unbound; ids were formed from node numbers"), { timeout: 20000 });
    expect(host.named("adopt_mbf_import")[0].args.declaration).toEqual(sent);
    expectNoFileContent(host);
  });

  it("refuses a prior-format declaration review without adopting or replacing the session", async () => {
    await renderApp();
    setTauriRuntime();
    const old = reply("unbound-declared");
    old.review = { document_kind: "swbpipe.mbf_import_review", format: 1, adoptable: true };
    const host = nativeHost({ open_mbf_import: () => reply("unbound-open"), review_mbf_import: () => old });
    const dialog = await openImport();
    await reviewShown();
    fireEvent.change(within(dialog).getByRole("combobox", { name: /Source of / }), { target: { value: "invented" } });
    await waitFor(() => expect(dialog).toHaveTextContent("MBF-IMPORT-WORKER-PROTOCOL"));
    expect(within(dialog).queryByTestId("mbf-import-identity")).not.toBeInTheDocument();
    expect(within(dialog).getByTestId("mbf-import-adopt")).toBeDisabled();
    expect(host.count("adopt_mbf_import")).toBe(0);
    expectDemoSession();
    const before = host.count("cancel_mbf_import");
    fireEvent.click(within(dialog).getByRole("button", { name: "Close" }));
    await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(before + 1));
  });

  it("invalidates a pending declaration review on reversion, then can review and adopt again", async () => {
    await renderApp();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("unbound-open"), adopt_mbf_import: () => reply("unbound-adopt") }, new Set(["review_mbf_import"]));
    const dialog = await openImport();
    await reviewShown();
    const select = within(dialog).getByRole("combobox", { name: /Source of / });
    fireEvent.change(select, { target: { value: "invented" } });
    await waitFor(() => expect(host.held).toHaveLength(1));
    fireEvent.change(select, { target: { value: "none" } });
    await act(async () => { host.held[0].resolve(reply("unbound-declared")); });
    expect(within(dialog).getByTestId("mbf-import-adopt")).toBeDisabled();
    fireEvent.change(select, { target: { value: "invented" } });
    await waitFor(() => expect(host.held).toHaveLength(2));
    await act(async () => { host.held[1].resolve(reply("unbound-declared")); });
    await waitFor(() => expect(within(dialog).getByTestId("mbf-import-adopt")).toBeEnabled());
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("unbound; ids were formed from node numbers"), { timeout: 20000 });
  });

  it("keeps a prototype-shaped material's declaration through the dialog and transport", async () => {
    await renderApp();
    setTauriRuntime();
    const initial = reply("unbound-open");
    (initial.review as { declaration_needed: string[] }).declaration_needed = ["__proto__"];
    const host = nativeHost({ open_mbf_import: () => initial }, new Set(["review_mbf_import"]));
    const dialog = await openImport();
    await reviewShown();
    fireEvent.change(within(dialog).getByRole("combobox", { name: "Source of __proto__" }), { target: { value: "invented" } });
    await waitFor(() => expect(host.held).toHaveLength(1));
    const declaration = host.held[0].args.declaration as Record<string, unknown>;
    expect(Object.hasOwn(declaration, "__proto__")).toBe(true);
    expect(declaration["__proto__"]).toEqual({ origin: "project_invented", source_name: "project-invented example data" });
  });

  it("Close, Escape and an App unmount each send one retirement; a reply after close is discarded", async () => {
    const view = await renderApp();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("bound-open") });
    for (const close of ["button", "escape"] as const) {
      const dialog = await openImport();
      await reviewShown();
      const before = host.count("cancel_mbf_import");
      if (close === "button") fireEvent.click(within(dialog).getByTestId("mbf-import-close"));
      else fireEvent.keyDown(window, { key: "Escape" });
      await waitFor(() => expect(screen.queryByTestId("mbf-import-dialog")).not.toBeInTheDocument());
      await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(before + 1));
      expect(host.count("adopt_mbf_import")).toBe(0);
    }
    // A reply that arrives after its dialog closed is discarded.
    const late = nativeHost({}, new Set(["open_mbf_import"]));
    await openImport();
    await waitFor(() => expect(late.held).toHaveLength(1));
    fireEvent.click(screen.getByTestId("mbf-import-close"));
    await act(async () => { late.held[0].resolve(reply("bound-open")); });
    expect(screen.queryByTestId("mbf-import-dialog")).not.toBeInTheDocument();
    expect(screen.queryByTestId("mbf-import-review")).not.toBeInTheDocument();
    expectDemoSession();
    // A fresh request is still choosing when the App unmounts; that request retires exactly once.
    await openImport();
    await waitFor(() => expect(late.held).toHaveLength(2));
    const before = late.count("cancel_mbf_import");
    view.unmount();
    await waitFor(() => expect(late.count("cancel_mbf_import")).toBe(before + 1));
    await act(async () => { late.held[1].resolve(reply("bound-open")); });
  });

  it.each([
    ["a native refusal", () => ({ outcome: "refused", file_name: "swbpipe-model.mbf", diagnostics: [{ code: "MBF-IMPORT-SOURCE-CHANGED", path: "", message: "native text" }] }), "MBF-IMPORT-SOURCE-CHANGED"],
    ["a stale model digest", () => ({ ...reply("bound-adopt"), modelSha256: "0".repeat(64) }), "MBF-IMPORT-REVIEW-STALE"],
    ["the raw-body transport refusal", () => { throw new Error("INVALID_REQUEST: INGRESS-RAW-BODY-REQUIRED: send `adopt_mbf_import`'s arguments as raw UTF-8 JSON bytes."); }, "INGRESS-RAW-BODY-REQUIRED"],
    ["the desktop shape check", () => ({ ...reply("bound-adopt"), document: { ...(reply("bound-adopt").document as object), nodes: "not a list" } }), "MODEL"]
  ])("leaves the session exactly as it was after %s", async (_label, adopt, code) => {
    await renderApp();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("bound-open"), adopt_mbf_import: adopt });
    const dialog = await openImport();
    await reviewShown();
    const before = host.count("cancel_mbf_import");
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("refused"), { timeout: 20000 });
    expect(projectMessage()).toHaveTextContent(code);
    expect(projectMessage()).not.toHaveTextContent("native text");
    if (code === "INGRESS-RAW-BODY-REQUIRED") expect(projectMessage()).toHaveTextContent(MBF_IMPORT_DESCRIPTIONS[code]);
    expectDemoSession();
    expect(screen.queryByTestId("project-edited")?.textContent ?? "").not.toContain("Edited");
    await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(before + 1));
  });

  it("is an in-window File menu item after Open Model Document…, refused by name in the browser-only route before anything is sent", async () => {
    await renderApp();
    const host = nativeHost({});
    fireEvent.click(screen.getByTestId("menu-file"));
    await screen.findByTestId("menu-item-file.import-mbf");
    const items = [...document.querySelectorAll("[data-testid^='menu-item-file.']")].map((node) => node.getAttribute("data-testid"));
    expect(items[items.indexOf("menu-item-file.open-model") + 1]).toBe("menu-item-file.import-mbf");
    fireEvent.click(screen.getByTestId("menu-item-file.import-mbf"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("MBF-IMPORT-NATIVE-ONLY"));
    expect(screen.queryByTestId("mbf-import-dialog")).not.toBeInTheDocument();
    expect(host.count("open_mbf_import")).toBe(0);
  });
});

/** One applied edit of node N-100's y (as `unsavedChangesPrompt.test.tsx` makes one). */
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

describe("Import MBF… with unsaved edits", () => {
  it("asks at Adopt; Cancel keeps the edits, sends no adopt and retires the source once", async () => {
    await renderApp();
    await applyNodeEdit();
    setTauriRuntime();
    const host = nativeHost({ open_mbf_import: () => reply("bound-open"), adopt_mbf_import: () => reply("bound-adopt") });
    const dialog = await openImport();
    await reviewShown();
    const before = host.count("cancel_mbf_import");
    fireEvent.click(within(dialog).getByTestId("mbf-import-adopt"));
    const prompt = await screen.findByTestId("unsaved-changes-prompt");
    expect(prompt).toHaveTextContent("Importing a model batch file");
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Not opened"));
    expect(host.count("adopt_mbf_import")).toBe(0);
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    expect(toolbarProject()).toHaveTextContent(demoModel.project.name);
    expect(screen.queryByTestId(IMPORTED_ROW)).not.toBeInTheDocument();
    await waitFor(() => expect(host.count("cancel_mbf_import")).toBe(before + 1));
  });
});
