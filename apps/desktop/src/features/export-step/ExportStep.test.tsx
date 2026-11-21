import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ExportRow, ExportStepButton } from "./ExportStep";

// DEL-07-12: the shared step between an export's button and its save, as the five export rows use it. Invented
// content only: the row and the step add nothing to what a panel passes them.
function Row({ record = true, label = null }: { record?: boolean; label?: string | null }) {
  return (
    <ExportRow
      testId="invented-panel"
      className="panel"
      ariaLabel="Invented export"
      stepTestId="invented"
      icon={null}
      name="Invented Export"
      what="Writes an invented file."
      units={<span data-testid="invented-units">source=length=m; target=none; results=none; conversion=false</span>}
      label={label}
      notices={<p data-testid="invented-notice">A notice of the result.</p>}
      action={(open) => record ? (
        <ExportStepButton stepTestId="invented" open={open} />
      ) : (
        <>
          <p className="export-row-status" id="invented-empty" data-testid="invented-empty">Solve the model first.</p>
          <ExportStepButton stepTestId="invented" open={open} disabled describedBy="invented-empty" />
        </>
      )}
      record={record ? <div data-testid="invented-record">count=3; conversion=false</div> : null}
      saves={<a href="data:text/plain,x" download="x.txt" data-testid="invented-save">Save</a>}
    />
  );
}

describe("the export row at rest", () => {
  it("holds the name, what it writes, the units line and the button; the record and the save are not in the page", () => {
    render(<Row />);
    const row = screen.getByTestId("invented-panel");
    expect(row).toHaveTextContent("Invented Export");
    expect(within(row).getByTestId("invented-what")).toHaveTextContent("Writes an invented file.");
    expect(within(row).getByTestId("invented-units")).toHaveTextContent("conversion=false");
    expect(screen.getByTestId("invented-open")).toBeEnabled();
    expect(screen.queryByTestId("invented-record")).toBeNull();
    expect(screen.queryByTestId("invented-notice")).toBeNull();
    expect(screen.queryByTestId("invented-save")).toBeNull();
    expect(screen.queryByTestId("invented-step")).toBeNull();
  });

  it("with no record, says why beside the disabled button and keeps the result's notices in the row", () => {
    render(<Row record={false} />);
    const button = screen.getByTestId("invented-open");
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("aria-describedby", "invented-empty");
    expect(screen.getByTestId("invented-empty")).toHaveTextContent("Solve the model first.");
    expect(within(screen.getByTestId("invented-panel")).getByTestId("invented-notice")).toBeInTheDocument();
    fireEvent.click(button);
    expect(screen.queryByTestId("invented-step")).toBeNull();
  });

  it("carries a record's label in the row and in the step", () => {
    render(<Row label="Preview: not an actual current generated record." />);
    expect(screen.getByTestId("invented-record-label")).toHaveTextContent("Preview");
    fireEvent.click(screen.getByTestId("invented-open"));
    expect(within(screen.getByTestId("invented-step")).getByTestId("invented-step-record-label")).toHaveTextContent("Preview");
  });
});

describe("the step", () => {
  it("opens with focus inside it and the record, then the save, in it", () => {
    render(<Row />);
    fireEvent.click(screen.getByTestId("invented-open"));
    const step = screen.getByTestId("invented-step");
    expect(step).toHaveAttribute("role", "dialog");
    expect(step).toHaveAttribute("aria-modal", "true");
    expect(step).toHaveAccessibleName("Invented Export");
    expect(step.contains(document.activeElement)).toBe(true);
    const record = within(step).getByTestId("invented-record");
    const save = within(step).getByTestId("invented-save");
    expect(within(step).getByTestId("invented-notice")).toBeInTheDocument();
    expect(record.compareDocumentPosition(save) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  });

  it("keeps Tab on its controls and holds back window shortcuts", () => {
    render(<Row />);
    fireEvent.click(screen.getByTestId("invented-open"));
    const step = screen.getByTestId("invented-step");
    for (let index = 0; index < 5; index += 1) {
      fireEvent.keyDown(window, { key: "Tab" });
      expect(step.contains(document.activeElement)).toBe(true);
    }
    const shortcut = new KeyboardEvent("keydown", { key: "s", metaKey: true, bubbles: true, cancelable: true });
    window.dispatchEvent(shortcut);
    expect(shortcut.defaultPrevented).toBe(true);
  });

  it.each(["Escape", "Close"])("is left by %s, writes nothing and returns focus to the row's button", async (how) => {
    render(<Row />);
    const button = screen.getByTestId("invented-open");
    fireEvent.click(button);
    if (how === "Escape") fireEvent.keyDown(window, { key: "Escape" });
    else fireEvent.click(screen.getByTestId("invented-step-close"));
    expect(screen.queryByTestId("invented-step")).toBeNull();
    expect(screen.queryByTestId("invented-save")).toBeNull();
    await waitFor(() => expect(button).toHaveFocus());
  });

  it("is retired when the record it shows is gone", () => {
    const view = render(<Row />);
    fireEvent.click(screen.getByTestId("invented-open"));
    expect(screen.getByTestId("invented-step")).toBeInTheDocument();
    view.rerender(<Row record={false} />);
    expect(screen.queryByTestId("invented-step")).toBeNull();
    expect(screen.queryByTestId("invented-save")).toBeNull();
  });
});
