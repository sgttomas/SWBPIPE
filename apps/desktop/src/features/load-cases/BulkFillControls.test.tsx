// Review findings N2 and N3 (PR #38) on the fill and copy controls. Invented ids.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { afterEach, expect, it, vi } from "vitest";
import type { DraftRecord } from "../rich-authoring/formSupport";
import { CopyFromCase, TemplateFill } from "./BulkFillControls";

afterEach(cleanup);
const TARGETS = ["pipe:A", "pipe:B", "pipe:C"].map(value => ({ value, label: value }));

function Harness({ initial }: { initial: DraftRecord[] }) {
  const [list, setList] = useState(initial);
  return <>
    <TemplateFill name="Element state" noun="pipe" list={list} keyName="pipe_ref" targets={TARGETS} onChange={setList} />
    <button type="button" onClick={() => setList(list.slice(1))}>Remove first row</button>
    <output data-testid="rows">{JSON.stringify(list)}</output>
  </>;
}

it("N3: a chosen template lapses when its row is removed, instead of becoming the next row", () => {
  render(<Harness initial={[{ pipe_ref: "pipe:A", t: "20" }, { pipe_ref: "pipe:B", t: "200" }]} />);
  fireEvent.change(screen.getByLabelText("Element state template"), { target: { value: "0" } });
  fireEvent.change(screen.getByLabelText("Element state fill targets"), { target: { value: "missing" } });
  expect(screen.getByRole("button", { name: "Fill element states" })).toBeEnabled();
  fireEvent.click(screen.getByRole("button", { name: "Remove first row" }));
  expect(screen.getByLabelText("Element state template")).toHaveValue("");
  expect(screen.getByRole("button", { name: "Fill element states" })).toBeDisabled();
  expect(screen.getByText("The rows changed; choose the template row again.")).toBeTruthy();
});

it("N2: a source case no longer offered cannot be copied from", () => {
  const onCopy = vi.fn(() => "copied");
  const { rerender } = render(<CopyFromCase what="pressure regions" cases={[{ value: "load:A", label: "A" }, { value: "load:B", label: "B" }]} onCopy={onCopy} />);
  fireEvent.change(screen.getByLabelText("Copy pressure regions from case"), { target: { value: "load:A" } });
  rerender(<CopyFromCase what="pressure regions" cases={[{ value: "load:B", label: "B" }]} onCopy={onCopy} />);
  expect(screen.getByLabelText("Copy pressure regions from case")).toHaveValue("");
  const button = screen.getByRole("button", { name: "Replace this draft's pressure regions with the copy" });
  expect(button).toBeDisabled();
  fireEvent.click(button);
  expect(onCopy).not.toHaveBeenCalled();
});

it("a refused copy reports why and leaves the draft to the caller unchanged", () => {
  const onCopy = vi.fn(() => { throw new Error("load:A has no pressure regions to copy."); });
  render(<CopyFromCase what="pressure regions" cases={[{ value: "load:A", label: "A" }]} onCopy={onCopy} />);
  fireEvent.change(screen.getByLabelText("Copy pressure regions from case"), { target: { value: "load:A" } });
  fireEvent.click(screen.getByRole("button", { name: "Replace this draft's pressure regions with the copy" }));
  expect(screen.getByRole("status")).toHaveTextContent("load:A has no pressure regions to copy.");
});

// Review N1 kept for the analysis-basis override now that it is edited on its row:
// a fill carries it only when the person ticks it for that fill.
const BASIS = { reason: "invented reason, pipe:A only", provenance: "invented provenance" };
function OverrideHarness({ initial }: { initial: DraftRecord[] }) {
  const [list, setList] = useState(initial);
  return <>
    <TemplateFill name="Element state" noun="pipe" list={list} keyName="pipe_ref" targets={TARGETS} onChange={setList}
      omit={["mass_state_ref", "analysis_basis_override"]}
      includable={{ key: "analysis_basis_override", label: "analysis-basis override", describe: v => JSON.stringify(v) }} />
    <output data-testid="rows">{JSON.stringify(list)}</output>
  </>;
}
const INCLUDE = "Element state fill includes the template's analysis-basis override";
const fillStatus = () => screen.getByText(/^Added /);
const rowsOf = () => JSON.parse(screen.getByTestId("rows").textContent!) as DraftRecord[];
function chooseAndFill(template: string, scope = "missing") {
  fireEvent.change(screen.getByLabelText("Element state template"), { target: { value: template } });
  fireEvent.change(screen.getByLabelText("Element state fill targets"), { target: { value: scope } });
}

it("N1: the override is offered unticked, shown as text, and not copied by default; mass_state_ref never", () => {
  render(<OverrideHarness initial={[{ pipe_ref: "pipe:A", analysis_basis_override: BASIS, mass_state_ref: "mass:A" }]} />);
  expect(screen.queryByLabelText(INCLUDE)).toBeNull();
  chooseAndFill("0");
  const box = screen.getByLabelText(INCLUDE) as HTMLInputElement;
  expect(box.checked).toBe(false);
  expect(screen.getByText(/Element state 1 analysis-basis override: .*invented reason, pipe:A only.*Not copied unless ticked/)).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Fill element states" }));
  expect(rowsOf().slice(1)).toEqual([{ pipe_ref: "pipe:B" }, { pipe_ref: "pipe:C" }]);
  expect(fillStatus()).toHaveTextContent("Not copied (kept only on their own row): mass_state_ref, analysis_basis_override.");
});

it("N1: ticked, the fill copies the override to every row it adds, says so, and the tick lapses after the fill", () => {
  render(<OverrideHarness initial={[{ pipe_ref: "pipe:A", analysis_basis_override: BASIS, mass_state_ref: "mass:A" }]} />);
  chooseAndFill("0", "range");
  fireEvent.change(screen.getByLabelText("Element state fill from pipe"), { target: { value: "pipe:B" } });
  fireEvent.change(screen.getByLabelText("Element state fill to pipe"), { target: { value: "pipe:B" } });
  fireEvent.click(screen.getByLabelText(INCLUDE));
  expect(screen.getByText(/It will be copied to every row this fill adds/)).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Fill element states" }));
  expect(rowsOf()[1]).toEqual({ pipe_ref: "pipe:B", analysis_basis_override: BASIS });
  expect(fillStatus()).toHaveTextContent("Included on every added row, as chosen: analysis_basis_override. Not copied (kept only on their own row): mass_state_ref.");
  // The tick held for that fill only: the next fill does not include it unless ticked again.
  expect((screen.getByLabelText(INCLUDE) as HTMLInputElement).checked).toBe(false);
  fireEvent.change(screen.getByLabelText("Element state fill targets"), { target: { value: "missing" } });
  fireEvent.click(screen.getByRole("button", { name: "Fill element states" }));
  expect(rowsOf()[2]).toEqual({ pipe_ref: "pipe:C" });
});

it("N1: choosing another template unticks the inclusion; a template without the override offers none", () => {
  render(<OverrideHarness initial={[{ pipe_ref: "pipe:A", analysis_basis_override: BASIS }, { pipe_ref: "pipe:B", analysis_basis_override: { ...BASIS, reason: "pipe:B" } }, { pipe_ref: "pipe:C" }]} />);
  chooseAndFill("0");
  fireEvent.click(screen.getByLabelText(INCLUDE));
  fireEvent.change(screen.getByLabelText("Element state template"), { target: { value: "1" } });
  expect((screen.getByLabelText(INCLUDE) as HTMLInputElement).checked).toBe(false);
  fireEvent.change(screen.getByLabelText("Element state template"), { target: { value: "2" } });
  expect(screen.queryByLabelText(INCLUDE)).toBeNull();
});
