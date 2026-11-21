import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ExportRow, ExportStepButton } from "./ExportStep";
import { AS_GIVEN, type ControlDisposition, type PackageRecord } from "./exportForm";

// DEL-07-12: the shared row and record of the five export rows. The criterion's clause: "the panel states at the
// export's row the limits every file of that format carries and shows the whole record on request, one press from
// the row; and it shows the record unasked, before the file is written, whenever the export is withheld or refused,
// the written file's values are replaced, or the package holds a loss, an unsupported-behaviour entry or a failed
// validation that this model produced." Invented content only: the row and the record add nothing to what a panel
// passes them. The invented rows use the review geometry format's table, whose five fixed loss entries are its
// standing limits.
const STANDING: PackageRecord["entries"] = [
  { kind: "loss", term: "loss:pipe-segments-centerline-only", category: "approximated" },
  { kind: "loss", term: "loss:supports-point-markers", category: "approximated" },
  { kind: "loss", term: "loss:components-point-markers", category: "approximated" },
  { kind: "loss", term: "loss:loads-and-combinations-omitted", category: "omitted" },
  { kind: "loss", term: "loss:glb-and-viewer-behavior-tbd", category: "TBD" },
  { kind: "finding", term: "REVIEW-GEOMETRY-VISUAL-ONLY", severity: "warning" }
];
const OWN: PackageRecord["entries"][number] = { kind: "finding", term: "REVIEW-GEOMETRY-UNITS-TBD", severity: "blocking", text: "Invented: this model's units." };

function Row({
  record = true,
  label = null,
  entries = STANDING,
  disposition = AS_GIVEN
}: {
  record?: boolean;
  label?: string | null;
  entries?: PackageRecord["entries"];
  disposition?: ControlDisposition;
}) {
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
      notWritten={label ? "Invented: what this file does not write." : null}
      packageRecord={record ? { format: "review-geometry", entries } : null}
      disposition={disposition}
      notices={<p data-testid="invented-notice">A notice of the result.</p>}
      action={({ open, label: words, write }) => record ? (
        write ?? <ExportStepButton stepTestId="invented" open={open}>{words}</ExportStepButton>
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

describe("a row whose package holds only the format's standing limits", () => {
  it("holds the save, states the limits by their count, and one press writes", () => {
    render(<Row />);
    const row = screen.getByTestId("invented-panel");
    expect(within(row).getByTestId("invented-what")).toHaveTextContent("Writes an invented file.");
    expect(within(row).getByTestId("invented-units")).toHaveTextContent("conversion=false");
    expect(within(row).getByTestId("invented-save")).toBeInTheDocument();
    expect(screen.queryByTestId("invented-open")).toBeNull();
    expect(within(row).getByTestId("invented-standing")).toHaveTextContent("5 things this format does not carry");
    expect(screen.queryByTestId("invented-step")).toBeNull();
  });

  it("shows the whole record on request, one press from the row, with no second save", () => {
    render(<Row />);
    fireEvent.click(screen.getByTestId("invented-record-open"));
    const step = screen.getByTestId("invented-step");
    expect(within(step).getByTestId("invented-record")).toBeInTheDocument();
    expect(within(step).getByTestId("invented-notice")).toBeInTheDocument();
    expect(within(step).getAllByTestId("invented-step-entry").map((item) => item.querySelector("strong")?.textContent)).toEqual(STANDING.map((entry) => entry.term));
    expect(within(step).queryByTestId("invented-save")).toBeNull();
    expect(within(step).queryByTestId("invented-step-causes")).toBeNull();
    expect(screen.getAllByTestId("invented-save")).toHaveLength(1);
  });

  it("the count of the standing limits opens the record too", () => {
    render(<Row />);
    fireEvent.click(screen.getByTestId("invented-standing"));
    expect(screen.getByTestId("invented-step")).toBeInTheDocument();
  });

  it("states the limits by their terms where they fit on one line", () => {
    render(<Row entries={STANDING.slice(0, 2)} />);
    const standing = screen.getByTestId("invented-standing");
    expect(standing.tagName).toBe("P");
    expect(standing).toHaveTextContent("Does not carry: pipe-segments-centerline-only; supports-point-markers");
  });
});

describe("a row whose record opens first", () => {
  it("for an entry this model produced: the button says what stands in the way, and the record opens with it first and the save after", () => {
    render(<Row entries={[...STANDING, OWN]} />);
    const row = screen.getByTestId("invented-panel");
    expect(within(row).queryByTestId("invented-save")).toBeNull();
    const button = screen.getByTestId("invented-open");
    expect(button).toHaveTextContent("Review 1 entry from this model, then export…");
    expect(within(row).getByTestId("invented-standing")).toHaveTextContent("5 things this format does not carry");
    fireEvent.click(button);
    const step = screen.getByTestId("invented-step");
    const causes = within(step).getByTestId("invented-step-causes");
    expect(within(causes).getAllByTestId("invented-step-cause").map((item) => item.querySelector("strong")?.textContent)).toEqual(["REVIEW-GEOMETRY-UNITS-TBD"]);
    const record = within(step).getByTestId("invented-record");
    const save = within(step).getByTestId("invented-save");
    expect(causes.compareDocumentPosition(record) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(record.compareDocumentPosition(save) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  });

  it("for a withheld export: the reason heads the record", () => {
    render(<Row disposition={{ kind: "withheld", reason: "Invented: not written." }} />);
    expect(screen.getByTestId("invented-open")).toHaveTextContent("Review…");
    fireEvent.click(screen.getByTestId("invented-open"));
    expect(within(screen.getByTestId("invented-step")).getByTestId("invented-step-reason")).toHaveTextContent("Invented: not written.");
  });

  it("for replaced values: the button gives their count", () => {
    render(<Row disposition={{ kind: "replaced", count: 3 }} />);
    expect(screen.getByTestId("invented-open")).toHaveTextContent("Review 3 replaced values, then export…");
  });
});

describe("the row without a record, and a labelled record", () => {
  it("with no record, says why beside the disabled button and keeps the result's notices in the row", () => {
    render(<Row record={false} />);
    const button = screen.getByTestId("invented-open");
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("aria-describedby", "invented-empty");
    expect(screen.getByTestId("invented-empty")).toHaveTextContent("Solve the model first.");
    expect(within(screen.getByTestId("invented-panel")).getByTestId("invented-notice")).toBeInTheDocument();
    expect(screen.queryByTestId("invented-standing")).toBeNull();
    fireEvent.click(button);
    expect(screen.queryByTestId("invented-step")).toBeNull();
  });

  it("carries a record's label, what is not written and the export used instead, in the row and in the record", () => {
    render(<Row label="Preview" />);
    expect(screen.getByTestId("invented-record-label")).toHaveTextContent("Preview");
    expect(screen.getByTestId("invented-not-written")).toHaveTextContent("Invented: what this file does not write.");
    expect(screen.getByTestId("invented-exchange")).toHaveTextContent("For exchange with another pipe stress program use Model batch file (.mbf).");
    fireEvent.click(screen.getByTestId("invented-record-open"));
    const step = screen.getByTestId("invented-step");
    expect(within(step).getByTestId("invented-step-record-label")).toHaveTextContent("Preview");
    expect(within(step).getByTestId("invented-step-not-written")).toBeInTheDocument();
    expect(within(step).getByTestId("invented-step-exchange")).toBeInTheDocument();
  });
});

describe("the record", () => {
  it("opens with focus inside it", () => {
    render(<Row />);
    fireEvent.click(screen.getByTestId("invented-record-open"));
    const step = screen.getByTestId("invented-step");
    expect(step).toHaveAttribute("role", "dialog");
    expect(step).toHaveAttribute("aria-modal", "true");
    expect(step).toHaveAccessibleName("Invented Export");
    expect(step.contains(document.activeElement)).toBe(true);
  });

  it("keeps Tab on its controls and holds back window shortcuts", () => {
    render(<Row entries={[...STANDING, OWN]} />);
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

  it.each(["Escape", "Close"])("is left by %s, writes nothing and returns focus to the control that opened it", async (how) => {
    render(<Row entries={[...STANDING, OWN]} />);
    const button = screen.getByTestId("invented-open");
    fireEvent.click(button);
    if (how === "Escape") fireEvent.keyDown(window, { key: "Escape" });
    else fireEvent.click(screen.getByTestId("invented-step-close"));
    expect(screen.queryByTestId("invented-step")).toBeNull();
    expect(screen.queryByTestId("invented-save")).toBeNull();
    await waitFor(() => expect(button).toHaveFocus());
  });

  it("is retired when the record it shows is gone", () => {
    const view = render(<Row entries={[...STANDING, OWN]} />);
    fireEvent.click(screen.getByTestId("invented-open"));
    expect(screen.getByTestId("invented-step")).toBeInTheDocument();
    view.rerender(<Row record={false} />);
    expect(screen.queryByTestId("invented-step")).toBeNull();
    expect(screen.queryByTestId("invented-save")).toBeNull();
  });
});
