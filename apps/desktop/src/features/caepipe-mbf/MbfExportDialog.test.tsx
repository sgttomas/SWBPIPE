// DEL-17-04 slice 4a: the export dialog, driven by the real wasm writer (`public/mbf-engine`) and the real hash
// engine, with the native save injected. The digests the dialog sends are checked against those the Rust wrapper
// recorded (`fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`), which the native save reproduces.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import exportModel from "../../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json";
import exportDigests from "../../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { loadMbfEngine, mbfSettingsJson, type MbfWriterReport, type WasmMbfEngine } from "../../services/mbfEngine";
import type { PreviewModel } from "../../types";
import { MbfExportDialog, type MbfExportDialogRequest } from "./MbfExportDialog";
import {
  MBF_ASSUMPTIONS_SENTENCE,
  MBF_EXPORT_REVIEW_STALE,
  MBF_EXPORT_ROUTE_ID,
  MBF_LOSS_DESCRIPTIONS,
  MBF_PROFILE_LABEL,
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
    expect(within(dialog).getByTestId("mbf-export-profile").textContent).toContain(MBF_PROFILE_LABEL);
    expect(within(dialog).getByTestId("mbf-export-profile").textContent).toContain("TBD-17-01-003 (open)");
    expect(within(dialog).getByTestId("mbf-export-assumptions").textContent).toBe(MBF_ASSUMPTIONS_SENTENCE);
    expect(within(dialog).getByTestId("mbf-export-identity").textContent).toMatch(/^Identities travel in the identity file beside the MBF\./);
    expect(within(dialog).getByTestId("mbf-export-boundary").textContent).toMatch(/does not check how another program reads or analyses the file/);
    expect((within(dialog).getByTestId("mbf-export-title") as HTMLInputElement).value).toBe("Invented MBF export");
    expect((within(dialog).getByTestId("mbf-export-reference-temperature") as HTMLInputElement).value).toBe("20");
    // The dialog, its review included, names no other program.
    expect(dialog.textContent).not.toMatch(/CAEPIPE/i);
    // Save needs the explicit choice.
    expect(saveButton(dialog).disabled).toBe(true);
    fireEvent.click(within(dialog).getByTestId("mbf-export-intent"));
    expect(saveButton(dialog).disabled).toBe(false);
    fireEvent.click(saveButton(dialog));
    await waitFor(() => expect(saveMbfExport).toHaveBeenCalledTimes(1));
    const args = saveMbfExport.mock.calls[0][0];
    // The settings text reviewed is the text sent, the one both Rust sides read (the recorded case).
    expect(args.settingsJson).toBe(exportDigests.cases["degc-default"].settings_json);
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
    expect(within(blocked).getByTestId("mbf-export-not-written-component").textContent).toMatch(/MBF-SI-[A-Z-]+ \(component:component:bend-1\): /);
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
    expect(JSON.parse(args.settingsJson).referenceTemperature).toEqual({ value: 68, unit: "degF" });
    expect(args.settingsJson).toBe(exportDigests.cases["degf-project"].settings_json);
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
    expect(JSON.parse(saveMbfExport.mock.calls[1][0].settingsJson).title).toBe("Another title");
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
  it("writes the settings text both Rust sides read, large integers included", () => {
    const settings = (value: number, unit: "degC" | "degF") => ({ title: "Invented MBF export", referenceTemperature: { value, unit }, maxLayoutRows: null } as const);
    // JSON.stringify writes these in full; the engine's reader takes them as the same ints natively and in wasm.
    expect(mbfSettingsJson(settings(9223372036854776000, "degC"))).toBe(exportDigests.cases["degc-integer-above-2p63"].settings_json);
    expect(mbfSettingsJson(settings(1e20, "degF"))).toBe(exportDigests.cases["degf-integer-above-2p64"].settings_json);
    expect(mbfSettingsJson(settings(21.5, "degC"))).toBe(exportDigests.cases["degc-fractional"].settings_json);
  });

  it("states no fact about another program: no profile id, assumption, program name or engine message", async () => {
    const real = await loadMbfEngine();
    const cases: Array<{ name: string; doc: PreviewModel }> = [
      { name: "written", doc: model() },
      { name: "degF", doc: model((edit) => { edit.project.units.temperature = "degF"; }) },
      { name: "refused", doc: model((edit) => { edit.components[0].kind = "tee"; }) }
    ];
    for (const { name, doc } of cases) {
      cleanup();
      const { dialog } = await open(doc);
      const text = dialog.textContent ?? "";
      // The engine's own review of the same text and settings.
      const settingsJson = mbfSettingsJson({
        title: "Invented MBF export",
        referenceTemperature: doc.project?.units?.temperature === "degF" ? { value: 68, unit: "degF" } : { value: 20, unit: "degC" },
        maxLayoutRows: null
      });
      const raw = JSON.parse(real.renderMbfJson(await canonicalJsonString(doc), settingsJson)) as { profile_id: string; report_json: string };
      const report = JSON.parse(raw.report_json) as MbfWriterReport;
      expect(text, name).not.toContain(raw.profile_id);
      expect(text, name).not.toMatch(/CAEPIPE|target program/i);
      expect(report.assumptions.length + report.loss_report.length, name).toBeGreaterThan(0);
      for (const sentence of report.assumptions) expect(text, `${name}: ${sentence}`).not.toContain(sentence);
      for (const entry of report.loss_report) {
        expect(text, `${name}: ${entry.reason}`).not.toContain(entry.reason);
        expect(text, name).toContain(entry.code);
      }
      for (const item of report.diagnostics) expect(text, `${name}: ${item.message}`).not.toContain(item.message);
      if (name === "degF") {
        expect(within(dialog).getByTestId("mbf-export-conversions").textContent).toMatch(/degF absolute: T\(degC\) = \(T\(degF\) - 32\) x 5\/9/);
      }
      if (name === "refused") {
        expect(within(dialog).getByTestId("mbf-export-not-written-component").textContent).toContain(MBF_LOSS_DESCRIPTIONS["MBF-SI-COMPONENT-UNSUPPORTED"]);
      }
    }
  }, 60000);

  it("has a product-side description for every MBF-SI code the engine can emit", () => {
    const engine = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../core/handoff/caepipe_mbf_engine/src");
    const codes = new Set<string>();
    const walk = (directory: string) => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const full = path.join(directory, entry.name);
        if (entry.isDirectory()) walk(full);
        else if (entry.name.endsWith(".rs")) for (const match of readFileSync(full, "utf8").matchAll(/"(MBF-SI-[A-Z0-9-]+)"/g)) codes.add(match[1]);
      }
    };
    walk(engine);
    expect(codes.size).toBeGreaterThan(50);
    for (const code of codes) expect(MBF_LOSS_DESCRIPTIONS[code], code).toBeTruthy();
    for (const [code, description] of Object.entries(MBF_LOSS_DESCRIPTIONS)) {
      expect(codes.has(code), `${code} is not an engine code`).toBe(true);
      expect(description, code).not.toMatch(/CAEPIPE|target program|another program/i);
    }
  });
});
