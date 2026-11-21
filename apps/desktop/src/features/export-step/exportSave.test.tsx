import { act, render, renderHook, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ExportSave } from "./ExportStep";
import { EXPORT_FILE_KINDS, exportFileName, localFeaHandoffFileName, projectSlug, rowSaveStatusText, useRowSave, type SaveExport } from "./exportSave";

// DEL-07-12: an export row's save interface, its status line and the suggested file name. Invented names only.
describe("the suggested file name", () => {
  it("is the project's display name as a slug, then the kind and extension from the row's table", () => {
    expect(exportFileName({ id: "project:invented-demo-loop-01", name: "Invented Utility Loop Demo" }, EXPORT_FILE_KINDS.pcfPackage)).toBe("invented-utility-loop-demo-pcf-package.json");
    expect(exportFileName({ id: "project:x", name: "Loop" }, EXPORT_FILE_KINDS.pcfText)).toBe("loop-pcf.pcf");
    expect(exportFileName({ id: "project:x", name: "Loop" }, EXPORT_FILE_KINDS.stressNeutralCsv)).toBe("loop-stress-neutral.csv");
  });

  it("falls back to the id without its prefix where the display name is empty or only punctuation, and to \"project\" where that is empty too", () => {
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: "" })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: null })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: " ¿?!… — " })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:", name: "***" })).toBe("project");
  });

  it("drops non-Latin characters by the same rule, and keeps a name that already ends in the kind as it is", () => {
    expect(projectSlug({ id: "project:x", name: "Трубопровод 7" })).toBe("7");
    expect(projectSlug({ id: "project:loop-02", name: "配管" })).toBe("loop-02");
    expect(exportFileName({ id: "project:x", name: "Loop PCF package" }, EXPORT_FILE_KINDS.pcfPackage)).toBe("loop-pcf-package-pcf-package.json");
  });

  it("is at most 64 characters before the kind, cut with no trailing hyphen", () => {
    const slug = projectSlug({ id: "project:x", name: `${"a".repeat(63)} b c d` });
    expect(slug).toBe("a".repeat(63));
    expect(projectSlug({ id: "project:x", name: "b".repeat(80) })).toHaveLength(64);
  });

  it("names a local FEA handoff's region where it has one", () => {
    expect(localFeaHandoffFileName({ id: "project:x", name: "Loop" }, "local-region:pipe:P-100:node:N-140")).toBe("loop-local-fea-handoff-pipe-p-100-node-n-140.json");
    expect(localFeaHandoffFileName({ id: "project:x", name: "Loop" }, null)).toBe("loop-local-fea-handoff.json");
  });
});

describe("the status line", () => {
  it("has one line for each outcome, and none before one", () => {
    expect(rowSaveStatusText(null)).toBe("");
    expect(rowSaveStatusText({ kind: "saving" })).toBe("Saving…");
    expect(rowSaveStatusText({ kind: "saved", name: "loop-pcf.pcf" })).toBe("Saved as loop-pcf.pcf.");
    expect(rowSaveStatusText({ kind: "saved-unconfirmed", name: "loop-pcf.pcf" })).toBe("Saved as loop-pcf.pcf. The system did not confirm it is safely on disk.");
    expect(rowSaveStatusText({ kind: "cancelled" })).toBe("Not saved: the save was cancelled.");
    expect(rowSaveStatusText({ kind: "refused", reason: "Invented reason.", code: "INVENTED" })).toBe("Not saved: Invented reason. (INVENTED)");
    expect(rowSaveStatusText({ kind: "failed", stage: "writing", temporaryMayRemain: true })).toBe("Not saved: the write failed while writing. A temporary file may remain beside it.");
    expect(rowSaveStatusText({ kind: "unknown" })).toBe("It is not known whether the file was saved.");
    // Not known is never worded as nothing written.
    expect(rowSaveStatusText({ kind: "unknown" })).not.toMatch(/not saved|nothing/i);
  });

  it("ignores an outcome that arrives after its file changed", async () => {
    let finish!: (value: unknown) => void;
    const save: SaveExport = () => new Promise((resolve) => { finish = resolve; });
    const { result, rerender } = renderHook(({ file }) => useRowSave(save, file), { initialProps: { file: "first" as unknown } });
    act(() => result.current.run({ control: "invented-link", suggestedName: "loop-pcf.pcf", href: "data:text/plain,x" }));
    expect(result.current.state).toEqual({ kind: "saving" });
    rerender({ file: "second" });
    expect(result.current.state).toBeNull();
    await act(async () => finish({ kind: "saved", name: "loop-pcf.pcf" }));
    expect(result.current.state).toBeNull();
  });

  it("keeps the outcome of the file it saved", async () => {
    const save: SaveExport = async () => ({ kind: "saved", name: "loop-pcf.pcf" });
    const { result } = renderHook(() => useRowSave(save, "same"));
    await act(async () => result.current.run({ control: "invented-link", suggestedName: "loop-pcf.pcf", href: "data:text/plain,x" }));
    expect(result.current.state).toEqual({ kind: "saved", name: "loop-pcf.pcf" });
  });
});

describe("a save control that writes nothing", () => {
  it("is not shown as a control: one line stands in its place", () => {
    render(
      <ExportSave control="invented-link" file="CSV">
        <div className="controlled-export-control"><span data-testid="invented-link" aria-disabled="true">CSV</span></div>
      </ExportSave>
    );
    expect(screen.getByTestId("invented-link-unavailable")).toHaveTextContent("CSV: not available. This file is withheld.");
    expect(screen.getByTestId("invented-link-save-status")).toHaveTextContent("");
  });

  it("a link with its address is a control, and no line is added", () => {
    render(
      <ExportSave control="invented-link" file="CSV">
        <div className="controlled-export-control"><a data-testid="invented-link" href="data:text/plain,x" download="x.csv">CSV</a></div>
      </ExportSave>
    );
    expect(screen.queryByTestId("invented-link-unavailable")).toBeNull();
  });
});
