import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useRef, useState } from "react";
import { afterEach, expect, it, vi } from "vitest";
import { useEnumEditor } from "./useEnumEditor";

function Harness() {
  const input = useRef<HTMLInputElement>(null); const [text, setText] = useState("pi");
  const enumEditor = useEnumEditor({ input, options: ["pipe"], token: 1, source: "pipe", active: true, text, pending: false, initialTyped: true, onChange: setText });
  return <div className="engineering-table"><div role="grid"><div className="engineering-table-body-slot" /></div>
    <div className="engineering-table-editor-layer"><input ref={input} {...enumEditor.attributes} value={text} onChange={(event) => setText(event.target.value)} onClick={enumEditor.show} /></div>{enumEditor.popup}</div>;
}
afterEach(() => vi.restoreAllMocks());
it.each(["vertical", "horizontal"])("keeps partially visible %s editor options, passively closes on empty viewport intersection", async (axis) => {
  let rect = new DOMRect(20, 20, 50, 30);
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) { return this.tagName === "INPUT" ? rect : new DOMRect(0, 0, 100, 100); });
  vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(100);
  vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockReturnValue(100);
  render(<Harness />); const input = screen.getByRole("combobox"); act(() => input.focus()); fireEvent.click(input); expect(screen.getByRole("listbox")).toBeVisible();
  rect = axis === "vertical" ? new DOMRect(20, -15, 50, 30) : new DOMRect(-25, 20, 50, 30);
  fireEvent.scroll(document.querySelector('.engineering-table-body-slot')!); expect(screen.getByRole("listbox")).toBeVisible();
  rect = axis === "vertical" ? new DOMRect(20, -30, 50, 30) : new DOMRect(-50, 20, 50, 30);
  fireEvent.scroll(document.querySelector('.engineering-table-body-slot')!); await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
  expect(input).toHaveValue("pi"); expect(input).toHaveFocus();
});
