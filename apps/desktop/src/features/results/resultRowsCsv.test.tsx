// RT-R6 (DEL-08-04 AC-002): the results panel's row tables write the value the panel shows.
// HELP_HUMAN engineering-judgment resolution, 2026-10-28: the panel's own converted value, the
// session display converter's binary64 output, written as its shortest round-trip decimal. One
// regression per factor in use (0.0254, 4.4482216152605, 0.001, pi/180): the CSV value parses
// to exactly the panel's pre-rounding value, bit for bit, with the panel's own converter
// (`convertDisplayQuantities`, through `QuantityReadout`) as the oracle.
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider, QuantityReadout, useDisplayQuantityResolver, type DisplaySourceQuantity, type DisplayUnitPreference } from "../display-units";
import { convertDisplayQuantities } from "../../services/displayQuantityService";
import { loadBundledMechanicsReference } from "../../services/previewService";
import { CSV_NOT_RECORDED, serializeCsvTable } from "../result-export/csvTable";
import { numericField, preambleValue, readCsv, textField } from "../../test/csvRead";
import { displayConversionBasis, resultGroupCsv, type ResultRowsCsvRow } from "./resultRowsCsv";
import { ResultsPanel } from "./ResultsPanel";

const saved: Array<{ text: string; fileName: string }> = [];
vi.mock("../result-export/csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string }) => {
    saved.push(input);
    return { mode: "browser", file_name: input.fileName, byte_count: input.text.length };
  })
}));
afterEach(() => { saved.length = 0; });

const PROVENANCE = { modelHash: CSV_NOT_RECORDED, resultHash: CSV_NOT_RECORDED, analysisRoutes: [CSV_NOT_RECORDED] };

function Probe({ quantity, onText }: { quantity: DisplaySourceQuantity; onText: (text: string) => void }) {
  const resolver = useDisplayQuantityResolver();
  const [busy, setBusy] = useState(false);
  return (
    <div>
      <span data-testid="readout"><QuantityReadout quantity={quantity} /></span>
      <button type="button" disabled={busy} onClick={async () => {
        setBusy(true);
        const display = await resolver.resolve(quantity);
        const rows: ResultRowsCsvRow[] = [{ id: "r:1", category: "physical_quantity", label: "", health: "", withheld: "", componentBody: "", entity: "node:A", location: "node", frame: "Global", quantity, display }];
        const unitBasis = await displayConversionBasis({ preference: resolver.preference, rows, convertRaw: resolver.convertRaw });
        onText(serializeCsvTable(resultGroupCsv({ title: "Displacement", rows, provenance: PROVENANCE, softwareVersion: "0.1.0", unitBasis, caseOrFilter: "c" }).table));
        setBusy(false);
      }}>export</button>
    </div>
  );
}

async function exportUnder(preference: DisplayUnitPreference, quantity: DisplaySourceQuantity): Promise<{ csv: ReturnType<typeof readCsv>; readout: string }> {
  let text = "";
  render(<DisplayUnitsProvider initialPreference={preference}><Probe quantity={quantity} onText={(value) => { text = value; }} /></DisplayUnitsProvider>);
  const readout = within(screen.getByTestId("readout")).getByText((_, element) => element?.tagName === "SPAN" && element.hasAttribute("data-display-status"));
  await waitFor(() => expect(readout.getAttribute("data-display-status")).toBe(preference === "entered" ? "entered" : "converted"));
  fireEvent.click(screen.getByRole("button", { name: "export" }));
  await waitFor(() => expect(text).not.toBe(""));
  return { csv: readCsv(text), readout: readout.textContent ?? "" };
}

const FACTORS: Array<{ factor: string; quantity: DisplaySourceQuantity; to: string }> = [
  { factor: "0.0254 (in to m)", quantity: { value: 12.345678901234567, unit: "in", dimension_id: "length" }, to: "m" },
  { factor: "4.4482216152605 (lbf to N)", quantity: { value: -1234.5678, unit: "lbf", dimension_id: "force" }, to: "N" },
  { factor: "0.001 (mm to m)", quantity: { value: 0.30000000000000004, unit: "mm", dimension_id: "length" }, to: "m" },
  { factor: "pi/180 (deg to rad)", quantity: { value: 37.5, unit: "deg", dimension_id: "angle" }, to: "rad" }
];

describe("ResultsPanel row tables: the panel's own converted value, bit for bit", () => {
  for (const { factor, quantity, to } of FACTORS) {
    it(`writes the converter's binary64 for factor ${factor}`, async () => {
      const [oracle] = await convertDisplayQuantities([{ id: "o", value: quantity.value as number, from_unit: quantity.unit, to_unit: to, dimension_id: quantity.dimension_id }]);
      expect(oracle.status).toBe("converted");
      const panelValue = (oracle as { value: number }).value;
      const { csv, readout } = await exportUnder("SI", quantity);
      // The panel shows this binary64 (its readout is `${value} ${unit}`).
      expect(readout.startsWith(`${String(panelValue)} ${to}`)).toBe(true);
      const cell = numericField(csv, csv.rows[0], "Value");
      expect(cell.unit).toBe(to);
      expect(Object.is(Number(cell.field), panelValue)).toBe(true);
      expect(cell.field).toBe(String(panelValue));
      // The entered value is kept beside it, exact, and the unit basis states the conversion.
      expect(Object.is(Number(numericField(csv, csv.rows[0], "Entered value").field), quantity.value)).toBe(true);
      const [one] = await convertDisplayQuantities([{ id: "1", value: 1, from_unit: quantity.unit, to_unit: to, dimension_id: quantity.dimension_id }]);
      const basis = preambleValue(csv, "unit_basis")!;
      expect(basis).toContain(`${quantity.dimension_id} ${quantity.unit}->${to} approximately v*${String((one as { value: number }).value)}`);
      expect(basis).toContain("binary64");
      expect(basis).toContain("approximate");
      expect(basis).not.toMatch(/exact/);
      expect(textField(csv, csv.rows[0], "Display")).toBe("converted");
    });
  }

  // Codex review of #1726, finding 2: the stated formula is approximate. The converter's unit
  // definitions (`core/units`) are not reachable from the webview, so the formula is the
  // converter's outputs at 0 and 1, labelled approximate; it is close to the CSV value, and it is
  // never offered as a bit-for-bit reconstruction.
  for (const { quantity, to, preference } of [
    ...FACTORS.map((item) => ({ ...item, preference: "SI" as const })),
    { factor: "affine", quantity: { value: 1000, unit: "degC", dimension_id: "temperature" }, to: "degF", preference: "US" as const }
  ]) {
    it(`states ${quantity.unit}->${to} as an approximate formula close to the CSV value`, async () => {
      const { csv } = await exportUnder(preference, quantity);
      const value = Number(numericField(csv, csv.rows[0], "Value").field);
      const basis = preambleValue(csv, "unit_basis")!;
      const entry = basis.split("; ").find((part) => part.startsWith(`${quantity.dimension_id} ${quantity.unit}->${to} `))!;
      const match = /approximately v\*(\S+?)(?:([+-]\S+))?$/.exec(entry);
      expect(match, entry).not.toBeNull();
      const formula = Number(match![1]) * (quantity.value as number) + (match![2] ? Number(match![2]) : 0);
      expect(Math.abs(formula - value)).toBeLessThanOrEqual(1e-12 * Math.max(1, Math.abs(value)));
      expect(basis).toContain("each formula below is approximate");
      expect(basis).not.toMatch(/exact/);
    });
  }

  it("under entered display units writes the stored value, exact, and says so", async () => {
    const quantity = { value: 0.1 + 0.2, unit: "mm", dimension_id: "length" };
    const { csv } = await exportUnder("entered", quantity);
    const cell = numericField(csv, csv.rows[0], "Value");
    expect(cell.unit).toBe("mm");
    expect(Object.is(Number(cell.field), quantity.value)).toBe(true);
    expect(preambleValue(csv, "unit_basis")).toBe("display units entered; values as stored, exact");
  });

  it("the panel's group and endpoint controls export every table, under SI, as the readouts show", async () => {
    const { source } = await loadBundledMechanicsReference();
    render(<DisplayUnitsProvider initialPreference="SI"><ResultsPanel result={source} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} /></DisplayUnitsProvider>);
    const buttons = screen.getAllByRole("button", { name: /^Export CSV: .* results$/ });
    expect(buttons.length).toBeGreaterThan(0);
    await waitFor(() => expect(document.querySelectorAll('[data-display-status="pending"]').length).toBe(0));
    for (const button of buttons) fireEvent.click(button);
    await waitFor(() => expect(saved.length).toBe(buttons.length));
    let compared = 0;
    for (const item of saved) {
      const csv = readCsv(item.text);
      expect(preambleValue(csv, "table")).toMatch(/^results-[a-z0-9-]+$/);
      expect(item.fileName).toMatch(/^swbpipe-results-[a-z0-9-]+\.csv$/);
      for (const line of csv.rows) {
        const id = textField(csv, line, "Result id");
        const shown = screen.queryByTestId(`result-row-${id}`);
        if (!shown) continue; // a later page
        const readout = shown.querySelector("[data-display-status]")!;
        const cell = numericField(csv, line, "Value");
        if (Number.isNaN(Number(cell.field))) continue;
        expect(readout.textContent!.startsWith(`${cell.field} ${cell.unit === "1" ? "" : cell.unit}`.trim())).toBe(true);
        compared += 1;
      }
    }
    expect(compared).toBeGreaterThan(5);
  });
});
