import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { afterEach, describe, expect, it, vi } from "vitest";
import reviewCases from "../../../fixtures/caepipe_mbf/invented/desktop_export/review_cases.json";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import {
  RETIREMENT_ID,
  mbfExportUnavailable,
  resetMbfExportUnavailableForTests,
  retireMbfExportSession,
  type MbfCancelReceipt,
  type MbfRetirementId
} from "./features/caepipe-mbf/mbfExportCancel";
import {
  INGRESS_RAW_BODY_REQUIRED,
  MBF_EXPORT_EPOCH_EXHAUSTED,
  MBF_EXPORT_REFUSAL_TEXT,
  MBF_EXPORT_ROUTE_ID,
  MBF_EXPORT_UNAVAILABLE,
  MBF_REVIEW_NATIVE_ONLY,
  MBF_SAVE_OUTCOME_UNKNOWN_TEXT,
  closedSaveStatusLine,
  type MbfExportSaveOutcome,
  type MbfReviewPayload
} from "./features/caepipe-mbf/mbfExport";
import { isMenuCommandId } from "./features/workspace/menuCommands";

// DEL-17-04 Design `desktop-mbf-export-import.md` §3.1, §3.7 and §3.9.5 (slice 4b.2b): File > Export MBF… and the
// Exports section's Model batch file panel open the mounted export dialog on the session's model, with the barrier's
// native review and save (`mbfExportCancel.ts`). These run the real App: its menu, its Exports section, the dialog's
// close and the workspace's lifecycle, against a mocked native host.

type Args = Record<string, unknown>;
const CASES = reviewCases.cases as Record<string, { review_text: string }>;
const payload = (name: string) => JSON.parse(CASES[name].review_text) as MbfReviewPayload;
const SAVED: MbfExportSaveOutcome = {
  outcome: "saved",
  mbf: { file_name: "swbpipe-model.mbf", byte_count: 900, sha256: "a".repeat(64), replaced_existing: false, durability: "durable" },
  identity_file: { file_name: "swbpipe-model.swbpipe-ids.json", byte_count: 2000, sha256: "b".repeat(64), replaced_existing: false, durability: "durable" },
  durability: "durable",
  path_containment: "best_effort_non_adversarial",
  limitation: "invented limitation",
  warnings: []
};

// The mocked native retirement epoch: each applied retirement moves it on (the barrier's module, and so its last
// acknowledged epoch, lasts across this file's tests).
let nativeEpoch = 0;
const receipt = (args: Args, tooLate = 0): MbfCancelReceipt => ({
  cancelled: 0,
  too_late: tooLate,
  epoch: ++nativeEpoch,
  retirement_id: args[RETIREMENT_ID] as MbfRetirementId
});

afterEach(async () => {
  // Leave the barrier settled and acknowledged for the next test.
  invokeMock.mockImplementation((command: string, args?: Args) =>
    command === "cancel_mbf_export" ? Promise.resolve(receipt(args ?? {})) : Promise.reject(new Error(command)));
  cleanup();
  await retireMbfExportSession();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

type Held = { command: string; args: Args; resolve: (value: unknown) => void; reject: (error: unknown) => void };

/** The native host: the shell's own commands, the export's three, and a hold on the commands the test answers itself. */
function nativeHost(options: {
  hold?: ReadonlySet<string>;
  review?: (args: Args) => unknown;
  save?: (args: Args) => unknown;
  tooLate?: () => number;
  other?: (command: string, args: Args) => unknown;
} = {}) {
  const commands: Array<{ command: string; args: Args }> = [];
  const held: Held[] = [];
  invokeMock.mockImplementation((command: string, raw?: Args) => {
    const args = raw ?? {};
    commands.push({ command, args });
    if (options.hold?.has(command)) return new Promise((resolve, reject) => held.push({ command, args, resolve, reject }));
    switch (command) {
      case "sync_native_shell_state":
        return Promise.resolve(null);
      case "check_model_library_refs":
        return Promise.resolve([]);
      case "cancel_mbf_export":
        return Promise.resolve(receipt(args, options.tooLate?.() ?? 0));
      case "review_mbf_export":
        return Promise.resolve(options.review ? options.review(args) : { outcome: "reviewed", review: payload("degc-default") });
      case "save_mbf_export":
        return Promise.resolve(options.save ? options.save(args) : SAVED);
      default:
        return options.other ? Promise.resolve(options.other(command, args)) : Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
  const named = (command: string) => commands.filter((item) => item.command === command);
  return { commands, held, named, count: (command: string) => named(command).length };
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

const setTauriRuntime = () => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; };
const projectMessage = () => screen.getByTestId("local-project-message");
const dialog = () => screen.getByTestId("mbf-export-dialog");

async function renderApp() {
  const view = render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(/^tree-row-project-/, {}, { timeout: 20000 });
  return view;
}

/** Chooses Export MBF… (by `choose`) until its dialog opens: until the session's model hash is shown it is refused
 * by name. Each choice is given time to open the dialog before the next, which would supersede it. */
async function untilOpen(choose: () => void) {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    choose();
    try {
      await waitFor(() => expect(screen.getByTestId("mbf-export-dialog")).toBeInTheDocument(), { timeout: 500 });
      return dialog();
    } catch {
      // Not open yet: choose again.
    }
  }
  throw new Error("Export MBF… did not open");
}

/** File > Export MBF… from the native menu's event. */
const openFromMenu = () => untilOpen(() => act(() => nativeMenuCommand("file.export-mbf")));

const sha256 = (text: string) => `sha256:${createHash("sha256").update(text, "utf8").digest("hex")}`;

describe("File > Export MBF… (the mounted dialog)", () => {
  it("opens from the native menu's item; it reviews and saves through the native commands, and closing retires the export", async () => {
    expect(isMenuCommandId("file.export-mbf")).toBe(true);
    const host = nativeHost();
    await renderApp();
    setTauriRuntime();
    // The native menu's item dispatches its id to the workspace (lib.rs `NATIVE_FILE_MENU`).
    await openFromMenu();
    const opened = dialog();
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    // The review ran natively on the session's canonical text and its hash, with an acknowledged retirement's epoch.
    const [reviewCall] = host.named("review_mbf_export");
    expect(host.count("review_mbf_export")).toBe(1);
    expect(reviewCall.args.claimedModelHash).toBe(sha256(reviewCall.args.canonicalText as string));
    expect(reviewCall.args.expectedEpoch).toBe(nativeEpoch);
    const ticket = reviewCall.args.expectedEpoch;
    // Save needs the explicit choice.
    const save = within(opened).getByTestId("mbf-export-save") as HTMLButtonElement;
    expect(save.disabled).toBe(true);
    fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
    expect(save.disabled).toBe(false);
    fireEvent.click(save);
    await waitFor(() => expect(within(opened).getByTestId("mbf-export-status")).toHaveTextContent("Saved swbpipe-model.mbf and its identity file swbpipe-model.swbpipe-ids.json"));
    const [saveCall] = host.named("save_mbf_export");
    const reviewed = payload("degc-default");
    expect(saveCall.args.reviewed).toEqual({ mbfSha256: reviewed.mbf_sha256, sidecarSha256: reviewed.sidecar_sha256 });
    expect(saveCall.args.settingsJson).toBe(reviewCall.args.settingsJson);
    expect(saveCall.args.canonicalText).toBe(reviewCall.args.canonicalText);
    expect(saveCall.args.claimedModelHash).toBe(reviewCall.args.claimedModelHash);
    expect(saveCall.args.suggestedName).toBe("swbpipe-model.mbf");
    expect((saveCall.args.screening as Args).route_id).toBe(MBF_EXPORT_ROUTE_ID);
    expect((saveCall.args.localFirstEvidence as Args).route_id).toBe(MBF_EXPORT_ROUTE_ID);
    expect(saveCall.args.expectedEpoch).toBe(ticket);
    // Close: one keyed retirement, and the workspace is live again.
    const cancels = host.count("cancel_mbf_export");
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 1));
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(host.count("cancel_mbf_export")).toBe(cancels + 1);
    expect(screen.getByTestId("desktop-preview-shell")).not.toHaveAttribute("inert");
    // It opens again, and Escape closes it.
    const reopened = await openFromMenu();
    await within(reopened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 2));
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(host.count("cancel_mbf_export")).toBe(cancels + 2);
  }, 60000);

  it("opens the same dialog from the Exports section's Model batch file panel", async () => {
    const host = nativeHost();
    await renderApp();
    setTauriRuntime();
    act(() => nativeMenuCommand("view.section.exports"));
    const panel = await screen.findByLabelText("Model batch file (.mbf) export", {}, { timeout: 20000 });
    expect(panel.querySelectorAll("a[download]")).toHaveLength(0);
    const action = within(panel).getByTestId("caepipe-mbf-export");
    const opened = await untilOpen(() => {
      action.focus();
      fireEvent.click(action);
    });
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    expect(host.count("review_mbf_export")).toBe(1);
    const cancels = host.count("cancel_mbf_export");
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 1));
    // Focus returns to the panel's action.
    await waitFor(() => expect(document.activeElement).toBe(action));
  }, 60000);

  it("is an in-window File menu item; in the browser preview it declines the review with MBF-REVIEW-NATIVE-ONLY before anything reaches the native export", async () => {
    const host = nativeHost();
    await renderApp();
    const before = host.commands.length;
    // The in-window File menu's item, after Save Handoff….
    fireEvent.click(screen.getByTestId("menu-file"));
    const item = await screen.findByTestId("menu-item-file.export-mbf");
    expect(item).toHaveTextContent("Export MBF…");
    expect(item).toBeEnabled();
    const items = [...document.querySelectorAll("[data-testid^='menu-item-file.']")].map((node) => node.getAttribute("data-testid"));
    expect(items[items.indexOf("menu-item-file.save-handoff") + 1]).toBe("menu-item-file.export-mbf");
    fireEvent.click(item);
    const opened = screen.queryByTestId("mbf-export-dialog") ? dialog() : await untilOpen(() => {
      if (screen.queryByTestId("mbf-export-dialog")) return;
      if (!screen.queryByTestId("menu-item-file.export-mbf")) fireEvent.click(screen.getByTestId("menu-file"));
      fireEvent.click(screen.getByTestId("menu-item-file.export-mbf"));
    });
    expect(await within(opened).findByTestId("mbf-export-review-declined")).toHaveTextContent(MBF_REVIEW_NATIVE_ONLY);
    expect((within(opened).getByTestId("mbf-export-save") as HTMLButtonElement).disabled).toBe(true);
    const sent = host.commands.slice(before).map((item) => item.command);
    expect(sent).not.toContain("review_mbf_export");
    expect(sent).not.toContain("save_mbf_export");
  }, 60000);

  it("discards a review that answers after its dialog closed; the reopened dialog shows only its own", async () => {
    const host = nativeHost({ hold: new Set(["review_mbf_export"]) });
    await renderApp();
    setTauriRuntime();
    await openFromMenu();
    await waitFor(() => expect(host.held).toHaveLength(1));
    fireEvent.click(within(dialog()).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    const reopened = await openFromMenu();
    await waitFor(() => expect(host.held).toHaveLength(2));
    // The first dialog's review answers late, with a review that would block: it is not shown.
    await act(async () => { host.held[0].resolve({ outcome: "reviewed", review: payload("refused-component") }); });
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 20)); });
    expect(within(reopened).getByTestId("mbf-export-review-running")).toBeInTheDocument();
    expect(within(reopened).queryByTestId("mbf-export-not-written")).toBeNull();
    await act(async () => { host.held[1].resolve({ outcome: "reviewed", review: payload("degc-default") }); });
    await within(reopened).findByTestId("mbf-export-profile");
    expect(within(reopened).queryByTestId("mbf-export-not-written")).toBeNull();
    // Each start carried the epoch acknowledged after the retirement before it.
    expect(host.held[1].args.expectedEpoch as number).toBeGreaterThan(host.held[0].args.expectedEpoch as number);
  }, 60000);

  /** Opens the dialog, starts a save the host holds, and closes the dialog once the save has committed natively (the
   * close's retirement reaches it too late). The project's name and the held save. */
  async function closeDuringCommittedSave(options: Parameters<typeof nativeHost>[0] = {}) {
    let tooLate = 0;
    const host = nativeHost({ ...options, hold: new Set(["save_mbf_export"]), tooLate: () => tooLate });
    await renderApp();
    setTauriRuntime();
    const opened = await openFromMenu();
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    const projectName = (JSON.parse(host.named("review_mbf_export")[0].args.canonicalText as string) as { project: { name: string } }).project.name;
    fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
    fireEvent.click(within(opened).getByTestId("mbf-export-save"));
    await waitFor(() => expect(host.held).toHaveLength(1));
    tooLate = 1;
    const cancels = host.count("cancel_mbf_export");
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    // A committed save may still finish or fail: the status line says only that it is finishing.
    const attribution = { projectName, earlierSession: false };
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "finishing" }, attribution)));
    expect(projectMessage().textContent).not.toMatch(/written|saved/i);
    expect(host.count("cancel_mbf_export")).toBe(cancels + 1);
    return { host, save: host.held[0], projectName, attribution };
  }

  it("a save that had committed when the dialog closed: finishing, then both files written once it has saved", async () => {
    const { save, attribution } = await closeDuringCommittedSave();
    await act(async () => { save.resolve(SAVED); });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "saved", outcome: SAVED as Extract<MbfExportSaveOutcome, { outcome: "saved" }> }, attribution)));
    expect(projectMessage()).toHaveTextContent("both files were written, swbpipe-model.mbf and its identity file swbpipe-model.swbpipe-ids.json");
    expect(screen.queryByTestId("mbf-export-dialog")).toBeNull();
  }, 60000);

  it("a save that had committed when the dialog closed and then failed to write: the failure, not success", async () => {
    const { save, attribution } = await closeDuringCommittedSave();
    const diagnostics = [{ code: "MBF-EXPORT-SAVE-RENAME-FAILED", path: "", message: "Invented: the model batch file could not be moved into place. Nothing was kept." }];
    await act(async () => { save.resolve({ outcome: "refused", diagnostics }); });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "failed", diagnostics }, attribution)));
    expect(projectMessage()).toHaveTextContent("did not finish. MBF-EXPORT-SAVE-RENAME-FAILED: Invented: the model batch file could not be moved into place.");
    expect(projectMessage().textContent).not.toMatch(/were written|finished:/);
  }, 60000);

  it("a save that had committed when the dialog closed and then undid itself: the rollback's own codes", async () => {
    const { save, attribution } = await closeDuringCommittedSave();
    const diagnostics = [
      { code: "MBF-EXPORT-READBACK-MISMATCH", path: "", message: "Invented: the file read back differs from what was written. The save was undone." },
      { code: "MBF-EXPORT-ROLLBACK-INCOMPLETE", path: "", message: "Invented: kept as .swbpipe-mbf-0.bak; rename it back by hand." }
    ];
    await act(async () => { save.resolve({ outcome: "refused", diagnostics }); });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "failed", diagnostics }, attribution)));
    expect(projectMessage()).toHaveTextContent("MBF-EXPORT-READBACK-MISMATCH");
    expect(projectMessage()).toHaveTextContent("MBF-EXPORT-ROLLBACK-INCOMPLETE: Invented: kept as .swbpipe-mbf-0.bak");
    expect(projectMessage().textContent).not.toMatch(/were written|finished:/);
  }, 60000);

  // Tauri 2.11.1's transport resends an invoke through its fallback when its `fetch` fails or when reading its reply
  // fails: the original raw save may commit and write (or undo itself) before its reply is lost, and the resend is then
  // refused at the ingress. That refusal proves only that the resend wrote nothing.
  const FALLBACK_REFUSAL = { code: "INVALID_REQUEST", message: "INGRESS-RAW-BODY-REQUIRED: send `save_mbf_export`'s arguments as raw UTF-8 JSON bytes." };

  it("a save that had committed when the dialog closed, whose reply was lost and whose resend was refused after the fallback: reported as unknown, not as nothing written", async () => {
    const { save, attribution } = await closeDuringCommittedSave();
    // The original committed (the receipt said too late); its reply is lost and the transport's resend is refused.
    await act(async () => { save.reject(FALLBACK_REFUSAL); });
    const diagnostics = [{ code: INGRESS_RAW_BODY_REQUIRED, path: "", message: `${FALLBACK_REFUSAL.code}: ${FALLBACK_REFUSAL.message}` }];
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "unknown", diagnostics }, attribution)));
    expect(projectMessage()).toHaveTextContent(`has no confirmed result. ${INGRESS_RAW_BODY_REQUIRED}: ${MBF_SAVE_OUTCOME_UNKNOWN_TEXT}`);
    expect(projectMessage().textContent).not.toMatch(/Nothing was written|isn't available|did not finish|raw UTF-8/);
  }, 60000);

  it("a save in the open dialog whose original committed, whose reply was lost and whose resend was refused after the fallback: the dialog says the outcome is unknown", async () => {
    let written = 0;
    // The host commits and writes the original save; the page sees only the resend's refusal.
    nativeHost({ save: () => { written += 1; return Promise.reject(FALLBACK_REFUSAL); } });
    await renderApp();
    setTauriRuntime();
    const opened = await openFromMenu();
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
    fireEvent.click(within(opened).getByTestId("mbf-export-save"));
    const refusal = await within(opened).findByTestId("mbf-export-refusal");
    expect(written).toBe(1);
    expect(refusal).toHaveTextContent(`${INGRESS_RAW_BODY_REQUIRED}: ${MBF_SAVE_OUTCOME_UNKNOWN_TEXT}`);
    expect(refusal.textContent).not.toMatch(/Nothing was written|isn't available|raw UTF-8/);
    expect(within(opened).getByTestId("mbf-export-status").textContent).toBe("");
  }, 60000);

  // After a save whose outcome is unknown this page sends no review and no save until it is reloaded or SWBPIPE
  // restarts; the keyed retirement, sent in the JSON form that still reaches the host, is not held.
  const unknownSaves: Array<[string, () => unknown]> = [
    ["rejected", () => Promise.reject(FALLBACK_REFUSAL)],
    ["refused with the code", () => ({ outcome: "refused", diagnostics: [{ code: INGRESS_RAW_BODY_REQUIRED, path: "", message: FALLBACK_REFUSAL.message }] })]
  ];

  it.each(unknownSaves)("a save %s after the fallback: no further native start in this page, through settings, close and reopen; the retirement still sends", async (_form, answer) => {
    const host = nativeHost({ save: answer });
    await renderApp();
    setTauriRuntime();
    const opened = await openFromMenu();
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
    const save = within(opened).getByTestId("mbf-export-save") as HTMLButtonElement;
    fireEvent.click(save);
    const refusal = await within(opened).findByTestId("mbf-export-refusal");
    expect(refusal).toHaveTextContent(`${INGRESS_RAW_BODY_REQUIRED}: ${MBF_SAVE_OUTCOME_UNKNOWN_TEXT}`);
    await waitFor(() => expect(save.disabled).toBe(true));
    expect(mbfExportUnavailable()).toBe(true);
    const starts = () => host.count("review_mbf_export") + host.count("save_mbf_export");
    expect(starts()).toBe(2);
    fireEvent.click(save);
    // A setting changes: the review is not sent, and the dialog says why.
    fireEvent.change(within(opened).getByTestId("mbf-export-title"), { target: { value: "Another title" } });
    expect(await within(opened).findByTestId("mbf-export-review-declined")).toHaveTextContent(`${MBF_EXPORT_UNAVAILABLE}: ${MBF_EXPORT_REFUSAL_TEXT[MBF_EXPORT_UNAVAILABLE]}`);
    expect(save.disabled).toBe(true);
    // Close: the keyed retirement is still sent.
    const cancels = host.count("cancel_mbf_export");
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 1));
    // Reopened: still no review and no Save.
    const reopened = await openFromMenu();
    expect(await within(reopened).findByTestId("mbf-export-review-declined", {}, { timeout: 20000 })).toHaveTextContent(MBF_EXPORT_REFUSAL_TEXT[MBF_EXPORT_UNAVAILABLE]);
    fireEvent.click(within(reopened).getByTestId("mbf-export-intent"));
    expect((within(reopened).getByTestId("mbf-export-save") as HTMLButtonElement).disabled).toBe(true);
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(starts()).toBe(2);
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 2));
    // A fresh page (the tests' reset) reviews and saves again.
    resetMbfExportUnavailableForTests();
    host.commands.length = 0;
    const fresh = await openFromMenu();
    await within(fresh).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    expect(host.count("review_mbf_export")).toBe(1);
  }, 60000);

  it("a closed dialog's save whose outcome is unknown leaves the page unable to export too; the reopened dialog says why and starts nothing", async () => {
    const { host, save, attribution } = await closeDuringCommittedSave();
    await act(async () => { save.reject(FALLBACK_REFUSAL); });
    const diagnostics = [{ code: INGRESS_RAW_BODY_REQUIRED, path: "", message: `${FALLBACK_REFUSAL.code}: ${FALLBACK_REFUSAL.message}` }];
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "unknown", diagnostics }, attribution)));
    expect(mbfExportUnavailable()).toBe(true);
    const reviews = host.count("review_mbf_export");
    const reopened = await openFromMenu();
    expect(await within(reopened).findByTestId("mbf-export-review-declined", {}, { timeout: 20000 })).toHaveTextContent(MBF_EXPORT_REFUSAL_TEXT[MBF_EXPORT_UNAVAILABLE]);
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(host.count("review_mbf_export")).toBe(reviews);
    expect(host.count("save_mbf_export")).toBe(1);
  }, 60000);

  it("a save that had already completed natively when the dialog closed (the receipt counts it in neither total) still reports its failure", async () => {
    const host = nativeHost({ hold: new Set(["save_mbf_export"]), tooLate: () => 0 });
    await renderApp();
    setTauriRuntime();
    const opened = await openFromMenu();
    await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    const projectName = (JSON.parse(host.named("review_mbf_export")[0].args.canonicalText as string) as { project: { name: string } }).project.name;
    fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
    fireEvent.click(within(opened).getByTestId("mbf-export-save"));
    await waitFor(() => expect(host.held).toHaveLength(1));
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    const diagnostics = [{ code: "MBF-EXPORT-ROLLBACK-INCOMPLETE", path: "", message: "Invented: kept as .swbpipe-mbf-0.bak; rename it back by hand." }];
    await act(async () => { host.held[0].resolve({ outcome: "refused", diagnostics }); });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(closedSaveStatusLine({ kind: "failed", diagnostics }, { projectName, earlierSession: false })));
    expect(projectMessage()).toHaveTextContent("MBF-EXPORT-ROLLBACK-INCOMPLETE: Invented: kept as .swbpipe-mbf-0.bak");
  }, 60000);

  it("a committed save that completes after a new project session reports as the earlier session's export, never the new project's", async () => {
    let second: Record<string, unknown> | null = null;
    const { host, save, projectName } = await closeDuringCommittedSave({
      other: (command) => command === "open_model_document_file"
        ? { outcome: "opened", file_name: "invented-second.json", byte_count: 1, schema_status: "current", document: second }
        : Promise.reject(new Error(`Unexpected command ${command}`))
    });
    const document = JSON.parse(host.named("review_mbf_export")[0].args.canonicalText as string) as { project: { name: string } };
    document.project.name = "Second invented project";
    second = document as unknown as Record<string, unknown>;
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(host.count("open_model_document_file")).toBe(1));
    await waitFor(() => expect(screen.getAllByText(/Second invented project/).length).toBeGreaterThan(0), { timeout: 20000 });
    await act(async () => { save.resolve(SAVED); });
    const attributed = closedSaveStatusLine({ kind: "saved", outcome: SAVED as Extract<MbfExportSaveOutcome, { outcome: "saved" }> }, { projectName, earlierSession: true });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(attributed));
    expect(projectMessage()).toHaveTextContent(`The export of "${projectName}" from an earlier project session`);
    expect(projectMessage().textContent).not.toContain("Second invented project");
  }, 60000);

  it("closing during a save leaves the next dialog usable, before and after the old save settles", async () => {
    const host = nativeHost({ hold: new Set(["save_mbf_export"]) });
    await renderApp();
    setTauriRuntime();
    const first = await openFromMenu();
    await within(first).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    fireEvent.click(within(first).getByTestId("mbf-export-intent"));
    fireEvent.click(within(first).getByTestId("mbf-export-save"));
    await waitFor(() => expect(host.held).toHaveLength(1));
    expect(within(first).getByTestId("mbf-export-save")).toHaveTextContent("Saving…");
    fireEvent.click(within(first).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    // Reopened while the old save is still unsettled: Save is offered again.
    const reopened = await openFromMenu();
    await within(reopened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
    const save = within(reopened).getByTestId("mbf-export-save") as HTMLButtonElement;
    expect(save).toHaveTextContent("Choose file and save…");
    fireEvent.click(within(reopened).getByTestId("mbf-export-intent"));
    expect(save.disabled).toBe(false);
    // The old save settles (cancelled by the close): the new dialog is unchanged.
    await act(async () => { host.held[0].resolve({ outcome: "refused", diagnostics: [{ code: "MBF-EXPORT-CANCELLED", path: "", message: "cancelled" }] }); });
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 20)); });
    expect(save).toHaveTextContent("Choose file and save…");
    expect(save.disabled).toBe(false);
    expect(within(reopened).queryByTestId("mbf-export-refusal")).toBeNull();
    // And it saves.
    fireEvent.click(save);
    await waitFor(() => expect(host.held).toHaveLength(2));
    expect(save).toHaveTextContent("Saving…");
    await act(async () => { host.held[1].resolve(SAVED); });
    await waitFor(() => expect(within(reopened).getByTestId("mbf-export-status")).toHaveTextContent("Saved swbpipe-model.mbf"));
    expect(save).toHaveTextContent("Choose file and save…");
  }, 60000);

  it("says what to do when the export is exhausted (restart) or the window can no longer send it (reload or restart)", async () => {
    const answers: Array<() => unknown> = [
      () => ({ outcome: "refused", diagnostics: [{ code: MBF_EXPORT_EPOCH_EXHAUSTED, path: "", message: "The export did not start: the export cannot be retired any further in this session. Nothing was written." }] }),
      () => Promise.reject({ code: "INVALID_REQUEST", message: "INGRESS-RAW-BODY-REQUIRED: send `review_mbf_export`'s arguments as raw UTF-8 JSON bytes." })
    ];
    nativeHost({ review: () => answers.shift()!() });
    await renderApp();
    setTauriRuntime();
    let opened = await openFromMenu();
    const exhausted = await within(opened).findByTestId("mbf-export-review-declined", {}, { timeout: 20000 });
    expect(exhausted).toHaveTextContent(`${MBF_EXPORT_EPOCH_EXHAUSTED}: ${MBF_EXPORT_REFUSAL_TEXT[MBF_EXPORT_EPOCH_EXHAUSTED]}`);
    expect(exhausted.textContent).toContain("Restart SWBPIPE");
    expect(exhausted.textContent).not.toMatch(/retired any further|epoch/);
    fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
    await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
    opened = await openFromMenu();
    const fellBack = await within(opened).findByTestId("mbf-export-review-declined", {}, { timeout: 20000 });
    // A review writes nothing, so its refusal after the fallback says so.
    expect(fellBack).toHaveTextContent(`${INGRESS_RAW_BODY_REQUIRED}: Export isn't available in this window. Nothing was written.`);
    expect(fellBack.textContent).toContain("Reload the window or restart SWBPIPE");
    expect(fellBack.textContent).not.toContain("raw UTF-8");
  }, 60000);

  it("is retired with the workspace: unmounting the App with the dialog open cancels the export natively", async () => {
    const host = nativeHost({ hold: new Set(["review_mbf_export"]) });
    const view = await renderApp();
    setTauriRuntime();
    await openFromMenu();
    await waitFor(() => expect(host.held).toHaveLength(1));
    const cancels = host.count("cancel_mbf_export");
    view.unmount();
    // One logical retirement: the dialog's and the workspace's cleanup share it.
    await waitFor(() => expect(host.count("cancel_mbf_export")).toBe(cancels + 1));
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(host.count("cancel_mbf_export")).toBe(cancels + 1);
    // The late review answers into nothing.
    await act(async () => { host.held[0].resolve({ outcome: "reviewed", review: payload("degc-default") }); });
    expect(screen.queryByTestId("mbf-export-dialog")).toBeNull();
  }, 60000);
});
