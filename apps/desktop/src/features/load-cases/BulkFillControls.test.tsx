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
