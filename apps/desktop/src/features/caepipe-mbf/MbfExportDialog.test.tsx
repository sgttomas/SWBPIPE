// DEL-17-04 slice 4a: the export dialog, driven by the real wasm writer (`public/mbf-engine`) and the real hash
// engine, with the native save injected. The digests the dialog sends are checked against those the Rust wrapper
// recorded (`fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`), which the native save reproduces.
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import exportModel from "../../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json";
import exportDigests from "../../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json";
import { computeModelHash } from "../../services/hashService";
import { loadMbfEngine, type WasmMbfEngine } from "../../services/mbfEngine";
import type { PreviewModel } from "../../types";
import { MbfExportDialog, type MbfExportDialogRequest } from "./MbfExportDialog";
import {
  MBF_EXPORT_REVIEW_STALE,
  MBF_EXPORT_ROUTE_ID,
  fileNameSlug,
  suggestedMbfName,
  type MbfExportArgs,
  type MbfExportSaveOutcome
} from "./mbfExport";

type Doc = Record<string, any>;

function model(edit: (document: Doc) => void = () => {}): PreviewModel {
  const document = JSON.parse(JSON.stringify(exportModel)) as Doc;
  edit(document);
  return document as unknown as PreviewModel;
}

const SAVED: MbfExportSaveOutcome = {
  outcome: "saved",
  mbf: { file_name: "swbpipe-model.mbf", byte_count: 900, sha256: "a".repeat(64), replaced_existing: false, durability: "durable" },
  identity_file: { file_name: "swbpipe-model.swbpipe-ids.json", byte_count: 2000, sha256: "b".repeat(64), replaced_existing: false, durability: "durable" },
  durability: "durable",
  path_containment: "best_effort_non_adversarial",
  limitation: "invented limitation",
  warnings: []
};

async function spiedEngine(): Promise<WasmMbfEngine & { renderMbfJson: ReturnType<typeof vi.fn> }> {
  const real = await loadMbfEngine();
  return { renderMbfJson: vi.fn(real.renderMbfJson), importMbfJson: real.importMbfJson };
}

async function open(document: PreviewModel, saveMbfExport = vi.fn(async (_: MbfExportArgs) => SAVED)) {
  const engine = await spiedEngine();
  const close = vi.fn();
  const request: MbfExportDialogRequest = {
    model: document,
    modelHash: await computeModelHash(document),
    projectName: "Invented MBF export",
    opener: null,
    close,
    saveMbfExport,
    engine
  };
  render(<MbfExportDialog request={request} />);
  const dialog = screen.getByTestId("mbf-export-dialog");
  await waitFor(() => expect(within(dialog).queryByTestId("mbf-export-profile")).not.toBeNull(), { timeout: 20000 });
  return { dialog, engine, close, saveMbfExport };
}

function saveButton(dialog: HTMLElement): HTMLButtonElement {
  return within(dialog).getByTestId("mbf-export-save") as HTMLButtonElement;
}

describe("MbfExportDialog", () => {
  it("reviews a written model, warns on omissions and saves the reviewed digests with the dialog's settings", async () => {
    const { dialog, saveMbfExport } = await open(model());
    expect(within(dialog).queryByTestId("mbf-export-not-written")).toBeNull();
    const omitted = within(dialog).getByTestId("mbf-export-omitted");
    expect(omitted.textContent).toMatch(/You can still save/);
    expect(omitted.querySelectorAll("li").length).toBeGreaterThan(0);
    expect(within(dialog).getByTestId("mbf-export-profile").textContent).toContain("caepipe-mbf-si-pilot-1");
    expect(within(dialog).getByTestId("mbf-export-profile").textContent).toContain("TBD-17-01-003 (open)");
    expect(within(dialog).getByTestId("mbf-export-identity").textContent).toMatch(/^Identities travel in the identity file beside the MBF\./);
    expect(within(dialog).getByTestId("mbf-export-boundary").textContent).toMatch(/does not check how another program reads or analyses the file/);
    expect((within(dialog).getByTestId("mbf-export-title") as HTMLInputElement).value).toBe("Invented MBF export");
    expect((within(dialog).getByTestId("mbf-export-reference-temperature") as HTMLInputElement).value).toBe("20");
    // The dialog's own text names no other program (the review region shows the writer's own profile id and texts).
    const own = dialog.cloneNode(true) as HTMLElement;
    own.querySelector('[data-testid="mbf-export-review"]')?.remove();
    expect(own.textContent).not.toMatch(/CAEPIPE/i);
    // Save needs the explicit choice.
    expect(saveButton(dialog).disabled).toBe(true);
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    expect(saveButton(dialog).disabled).toBe(false);
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(1));
    const args = saveMbfExport.mock.calls[0][0];
    expect(args.settings).toEqual({ title: "Invented MBF export", referenceTemperature: { value: 20, unit: "degC" }, maxLayoutRows: null });
    expect(args.suggestedName).toBe("swbpipe-model.mbf");
    expect(args.reviewed).toEqual({
      mbfSha256: exportDigests.cases["degc-default"].mbf_sha256,
      sidecarSha256: exportDigests.cases["degc-default"].sidecar_sha256
    });
    expect(args.screening).toMatchObject({ route_id: MBF_EXPORT_ROUTE_ID, export_context: "local_private", lossless_required: true, exact_payload_match: true });
    expect(args.localFirstEvidence).toMatchObject({ route_id: MBF_EXPORT_ROUTE_ID, blocked: false });
    await waitFor(() => expect(within(dialog).getByTestId("mbf-export-status").textContent).toMatch(
      /^Saved swbpipe-model\.mbf and its identity file swbpipe-model\.swbpipe-ids\.json; \d+ omitted entries\.$/
    ));
  }, 30000);

  it("blocks Save while anything is not written, listing the refusals by family", async () => {
    const { dialog, saveMbfExport } = await open(model((document) => { document.components[0].kind = "tee"; }));
    const blocked = within(dialog).getByTestId("mbf-export-not-written");
    expect(within(blocked).getByTestId("mbf-export-not-written-component").textContent).toMatch(/component:component:bend-1: MBF-SI-[A-Z-]+/);
    expect(within(blocked).getByRole("heading", { name: "Components" })).toBeTruthy();
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    expect(saveButton(dialog).disabled).toBe(true);
    fireEvent.click(saveButton(dialog));
    expect(saveMbfExport).not.toHaveBeenCalled();
  }, 30000);

  it("passes a degF project's reference temperature as 68 degF, exactly", async () => {
    const { dialog, saveMbfExport } = await open(model((document) => { document.project.units.temperature = "degF"; }));
    expect((within(dialog).getByTestId("mbf-export-reference-temperature") as HTMLInputElement).value).toBe("68");
    expect(dialog.textContent).toContain("Reference temperature (°F)");
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(1));
    const args = saveMbfExport.mock.calls[0][0];
    expect(args.settings.referenceTemperature).toEqual({ value: 68, unit: "degF" });
    expect(args.reviewed).toEqual({
      mbfSha256: exportDigests.cases["degf-project"].mbf_sha256,
      sidecarSha256: exportDigests.cases["degf-project"].sidecar_sha256
    });
  }, 30000);

  it("runs the review again when a setting changes or the native save finds it stale", async () => {
    let outcome: MbfExportSaveOutcome = {
      outcome: "refused",
      diagnostics: [{ code: MBF_EXPORT_REVIEW_STALE, path: "reviewed", message: "The model or the export settings changed since the review." }]
    };
    const saveMbfExport = vi.fn(async (_: MbfExportArgs) => outcome);
    const { dialog, engine } = await open(model(), saveMbfExport);
    const before = engine.renderMbfJson.mock.calls.length;
    // A setting changes: the review is stale, Save waits for the new review.
    fireEvent.change(within(dialog).getByTestId("mbf-export-reference-temperature"), { target: { value: "21.5" } });
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    await waitFor(() => expect(engine.renderMbfJson.mock.calls.length).toBeGreaterThan(before), { timeout: 20000 });
    await waitFor(() => expect(saveButton(dialog).disabled).toBe(false), { timeout: 20000 });
    // The native save refuses it as stale: the refusal is shown and the review runs again.
    const runs = engine.renderMbfJson.mock.calls.length;
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(within(dialog).getByTestId("mbf-export-refusal").textContent).toContain(MBF_EXPORT_REVIEW_STALE));
    await waitFor(() => expect(engine.renderMbfJson.mock.calls.length).toBeGreaterThan(runs), { timeout: 20000 });
    expect(saveMbfExport.mock.calls[0][0].reviewed).toEqual({
      mbfSha256: exportDigests.cases["degc-fractional"].mbf_sha256,
      sidecarSha256: exportDigests.cases["degc-fractional"].sidecar_sha256
    });
    // A new title changes the file, so the next save sends the new review's digests.
    outcome = SAVED;
    fireEvent.change(within(dialog).getByTestId("mbf-export-title"), { target: { value: "Another title" } });
    await waitFor(() => expect(saveButton(dialog).disabled).toBe(false), { timeout: 20000 });
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(2));
    expect(saveMbfExport.mock.calls[1][0].reviewed.mbfSha256).not.toBe(exportDigests.cases["degc-fractional"].mbf_sha256);
    expect(saveMbfExport.mock.calls[1][0].settings.title).toBe("Another title");
  }, 60000);

  it("refuses a reference temperature that is not a number before any review", async () => {
    const { dialog } = await open(model());
    fireEvent.change(within(dialog).getByTestId("mbf-export-reference-temperature"), { target: { value: "" } });
    await waitFor(() => expect(within(dialog).getByTestId("mbf-export-settings-invalid")).toBeTruthy());
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    expect(saveButton(dialog).disabled).toBe(true);
  }, 30000);

  it("suggests swbpipe-model[-<label slug>].mbf and names the identity file beside it", async () => {
    expect(suggestedMbfName("")).toBe("swbpipe-model.mbf");
    expect(suggestedMbfName("  ")).toBe("swbpipe-model.mbf");
    expect(suggestedMbfName("Rev B!")).toBe("swbpipe-model-rev-b.mbf");
    expect(suggestedMbfName("Línea 2 -- final")).toBe("swbpipe-model-linea-2-final.mbf");
    expect(fileNameSlug("***")).toBe("");
    for (const label of ["Rev B!", "Línea 2 -- final", "x"]) {
      expect(suggestedMbfName(label)).toMatch(/^swbpipe-model(-[a-z0-9]+)*\.mbf$/);
    }
    const saveMbfExport = vi.fn(async (_: MbfExportArgs) => SAVED);
    const { dialog } = await open(model(), saveMbfExport);
    fireEvent.change(within(dialog).getByTestId("mbf-export-label"), { target: { value: "Rev B" } });
    expect(within(dialog).getByTestId("mbf-export-suggested-name").textContent).toBe(
      "Suggested name: swbpipe-model-rev-b.mbf. The identity file is saved beside it with the same name, as swbpipe-model-rev-b.swbpipe-ids.json."
    );
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(1));
    expect(saveMbfExport.mock.calls[0][0].suggestedName).toBe("swbpipe-model-rev-b.mbf");
  }, 30000);

  it("is reachable by keyboard: focus starts in the dialog, Tab cycles its controls, Escape closes", async () => {
    const { dialog, close, saveMbfExport } = await open(model());
    expect(document.activeElement).toBe(within(dialog).getByTestId("mbf-export-title"));
    const order = ["mbf-export-reference-temperature", "mbf-export-label", "mbf-export-intent", "mbf-export-cancel", "mbf-export-title"];
    for (const id of order) {
      fireEvent.keyDown(window, { key: "Tab" });
      expect(document.activeElement).toBe(within(dialog).getByTestId(id));
    }
    // With the choice made, Save joins the cycle and is pressed from the keyboard.
    fireEvent.keyDown(window, { key: "Tab", shiftKey: true });
    fireEvent.keyDown(window, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(within(dialog).getByTestId("mbf-export-intent"));
    fireEvent.click(document.activeElement as HTMLElement);
    fireEvent.keyDown(window, { key: "Tab" });
    fireEvent.keyDown(window, { key: "Tab" });
    expect(document.activeElement).toBe(saveButton(dialog));
    fireEvent.click(document.activeElement as HTMLElement);
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(1));
    // Window shortcuts are held back while the dialog is open.
    const shortcut = new KeyboardEvent("keydown", { key: "s", metaKey: true, bubbles: true, cancelable: true });
    act(() => { window.dispatchEvent(shortcut); });
    expect(shortcut.defaultPrevented).toBe(true);
    fireEvent.keyDown(window, { key: "Escape" });
    expect(close).toHaveBeenCalledTimes(1);
  }, 30000);
});
