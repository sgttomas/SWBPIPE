// RT-R6 (DEL-08-04 AC-002): the "Export CSV" control, alone and in each results panel. It is a
// keyboard-reachable button named after its table; pressing it saves the adapter's file through
// `saveCsvTable` with explicit local-private intent, and the status line names the saved file.
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { healthStanding, overlayModel, overlayResult, reaction, row } from "../../test-support/resultOverlayFixtures";
import type { AnalysisRunEnvelope } from "../../types";
import { DisplacementExtremesPanel } from "../results/DisplacementExtremesPanel";
import { StressRatioTablePanel } from "../results/StressRatioTablePanel";
import { SupportLoadSummaryPanel } from "../results/SupportLoadSummaryPanel";
import { CsvExportControl } from "./CsvExportControl";
import { CSV_NOT_RECORDED, csvText, type CsvTable } from "./csvTable";
import { readCsv, preambleValue } from "../../test/csvRead";

const saved: Array<{ text: string; fileName: string; explicitLocalPrivateIntent: boolean }> = [];
vi.mock("./csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string; explicitLocalPrivateIntent: boolean }) => {
    saved.push(input);
    return { mode: "browser", file_name: input.fileName, byte_count: new TextEncoder().encode(input.text).length };
  })
}));
afterEach(() => { saved.length = 0; });

const table: CsvTable = {
  preamble: { table: "weight", softwareVersion: "0.1.0", modelHash: CSV_NOT_RECORDED, resultHash: "not_applicable", analysisRoutes: ["not_applicable"], caseOrFilter: "scope", unitBasis: "u" },
  columns: [{ kind: "text", label: "Category" }],
  rows: [[csvText("pipes")]]
};

describe("CsvExportControl", () => {
  it("is a named, keyboard-reachable button that saves with explicit intent and names the file", async () => {
    const save = vi.fn(async (input: { fileName: string; text: string }) => ({ mode: "native" as const, receipt: { outcome: "saved" as const, file_name: `${input.fileName.slice(0, -4)} (1).csv`, byte_count: 42, replaced_existing: false as const, durability: "not_guaranteed" as const, path_containment: "best_effort_non_adversarial" as const } }));
    render(<CsvExportControl tableLabel="Weight summary: Whole model" testId="x-export" build={() => ({ table, fileName: "swbpipe-weight-model.csv" })} save={save} />);
    const button = screen.getByRole("button", { name: "Export CSV: Weight summary: Whole model" });
    expect(button.tagName).toBe("BUTTON");
    expect(button).not.toHaveAttribute("tabindex", "-1");
    button.focus();
    expect(document.activeElement).toBe(button);
    fireEvent.click(button);
    await waitFor(() => expect(screen.getByTestId("x-export-status").textContent).toBe("Saved swbpipe-weight-model (1).csv (42 bytes)."));
    expect(save).toHaveBeenCalledOnce();
    expect(save.mock.calls[0][0]).toMatchObject({ fileName: "swbpipe-weight-model.csv", explicitLocalPrivateIntent: true });
    expect(screen.getByRole("status").textContent).toContain("swbpipe-weight-model (1).csv");
  });

  it("states a refusal without throwing", async () => {
    const save = vi.fn(async () => { throw { code: "LOCAL_PRIVATE_INTENT_REQUIRED", message: "Local CSV save did not complete." }; });
    render(<CsvExportControl tableLabel="T" testId="y" build={() => ({ table, fileName: "swbpipe-weight.csv" })} save={save as never} />);
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: T" }));
    await waitFor(() => expect(screen.getByTestId("y-status").textContent).toBe("CSV export did not complete (LOCAL_PRIVATE_INTENT_REQUIRED): Local CSV save did not complete.."));
  });
});

const PASSING = healthStanding({ eligible: true });
function rows() {
  return [
    row("ux:A", "global_nodal_displacement_x", "node:A", 1.5, "mm"),
    row("uy:A", "global_nodal_displacement_y", "node:A", -2, "mm"),
    reaction("support:S-A", "Fx", -12.25, "N"),
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", 123456789, "Pa", { metadata: { component: "sigma_plus", coordinate_system: "pipe_section", location: "end_i", basis: "invented", sign_convention: "invented" } })
  ];
}
const RUN = {
  schema_version: "0.3.0",
  analysis_run: { run_id: "run:invented", hashes: [{ algorithm: "sha256", canonicalization: "c", payload_ref: { object_type: "ResultEnvelope", ref: "result-envelope:run:invented" }, payload_scope: "received_result", value: "9f".repeat(32) }] }
} as unknown as AnalysisRunEnvelope;

describe("the results panels' Export CSV controls", () => {
  it("export the displacement extremes, support loads (shown and every support) and stress tables with real provenance", async () => {
    const model = overlayModel();
    const result = overlayResult(rows());
    const { unmount } = render(<>
      <DisplacementExtremesPanel model={model} result={result} analysisRun={RUN} standing={PASSING} />
      <SupportLoadSummaryPanel model={model} result={result} analysisRun={RUN} standing={PASSING} />
      <StressRatioTablePanel model={model} result={result} analysisRun={RUN} standing={PASSING} />
    </>);
    const names = [
      "Export CSV: Displacement and rotation extremes: All nodes",
      "Export CSV: Loads and movements at support support:S-A",
      "Export CSV: Loads and movements at every support",
      "Export CSV: Section normal-stress extreme σ±"
    ];
    const buttons = screen.getAllByRole("button", { name: /^Export CSV: / });
    expect(buttons.length).toBe(4);
    for (const button of buttons) fireEvent.click(button);
    await waitFor(() => expect(saved.length).toBe(4));
    const kinds = saved.map((item) => preambleValue(readCsv(item.text), "table")).sort();
    expect(kinds).toEqual(["displacement-extremes", "stress-ratio", "support-loads", "support-loads-all"]);
    for (const item of saved) {
      expect(item.explicitLocalPrivateIntent).toBe(true);
      const csv = readCsv(item.text);
      expect(preambleValue(csv, "result_hash")).toBe(`sha256:${"9f".repeat(32)}`);
      expect(preambleValue(csv, "model_hash")).toMatch(/^sha256:[0-9a-f]{64}$/);
      expect(preambleValue(csv, "analysis_route")).toBe(CSV_NOT_RECORDED);
    }
    for (const button of buttons) {
      const status = within(button.parentElement!).getByRole("status");
      await waitFor(() => expect(status.textContent).toMatch(/^Downloaded swbpipe-[a-z0-9-]+\.csv \(\d+ bytes\)\.$/));
    }
    expect(buttons.map((button) => button.getAttribute("aria-label"))).toEqual(names);
    unmount();
  });
});
