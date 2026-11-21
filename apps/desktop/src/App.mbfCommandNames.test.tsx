import { fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { App } from "./App";
import { CaepipeMbfExportPanel } from "./features/caepipe-mbf/CaepipeMbfExportPanel";
import demoModelFixture from "../../../fixtures/product_preview/invented_demo_model.json";
import type { PreviewModel } from "./types";

// DEL-07-12 REQ-007: no product text names the format's program; a person finds the model batch file's export
// and import by what they recognise, the file's extension. Invented data only.
afterEach(() => {
  invokeMock.mockReset();
  window.localStorage.clear();
});

describe("the model batch file's commands, found by the file's extension", () => {
  it.each([".mbf", "mbf", "batch"])("typing %s in the palette finds both the export and the import", async (query) => {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(screen.getByTestId("toolkit-entry"));
    fireEvent.change(screen.getByLabelText("Find a tool"), { target: { value: query } });
    const file = screen.getByRole("region", { name: "File" });
    expect(within(file).getByTestId("toolkit-shell-file.export-mbf")).toHaveTextContent("Export model batch file (.mbf)…");
    expect(within(file).getByTestId("toolkit-shell-file.import-mbf")).toHaveTextContent("Import model batch file (.mbf)…");
  });

  it("names both in the File menu with the extension", async () => {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(screen.getByTestId("menu-file"));
    expect(await screen.findByTestId("menu-item-file.export-mbf")).toHaveTextContent("Export model batch file (.mbf)…");
    expect(screen.getByTestId("menu-item-file.import-mbf")).toHaveTextContent("Import model batch file (.mbf)…");
  });

  it("the Exports row shows its name and its one line", () => {
    render(<CaepipeMbfExportPanel model={structuredClone(demoModelFixture) as PreviewModel} onExportMbf={() => {}} />);
    const row = screen.getByTestId("caepipe-mbf-panel");
    expect(row).toHaveTextContent("Model batch file (.mbf)");
    expect(within(row).getByTestId("caepipe-mbf-description")).toHaveTextContent("For exchange with pipe stress programs that read and write .mbf files");
  });
});
