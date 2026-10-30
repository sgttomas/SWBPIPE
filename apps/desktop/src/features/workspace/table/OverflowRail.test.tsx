import { act, fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { OverflowRail } from "./OverflowRail";

function setup(name: "table controls" | "table status", direction: "Earlier" | "Later") {
  const view = render(<><OverflowRail enabled name={name} owner="fixture-owner"><input aria-label="Retained field" defaultValue="retained draft" /></OverflowRail><button>Outside</button></>);
  const viewport = view.container.querySelector<HTMLElement>(".table-overflow-viewport")!;
  // Explicit component allocation; real Chromium148 browser cases separately
  // exercise actual layout and the native synchronous blur-on-disable behavior.
  Object.defineProperties(viewport, { clientWidth: { configurable: true, value: 400 }, scrollWidth: { configurable: true, value: 500 } });
  fireEvent.scroll(viewport, { target: { scrollLeft: direction === "Earlier" ? 100 : 0 } });
  const pair = screen.getByRole("group", { name: `Pan ${name}` });
  vi.spyOn(pair, "getClientRects").mockReturnValue([{ width: 56, height: 28 }] as unknown as DOMRectList);
  const button = screen.getByRole("button", { name: `${direction} ${name}` });
  const sibling = screen.getByRole("button", { name: `${direction === "Earlier" ? "Later" : "Earlier"} ${name}` });
  return { viewport, pair, button, sibling };
}

function synchronousDisableBlur(button: HTMLElement, afterBlur?: () => void) {
  const original = button.setAttribute.bind(button); let sawBody = false;
  const setter = vi.spyOn(button, "setAttribute").mockImplementation((name, value) => {
    original(name, value);
    if (name === "disabled") {
      // jsdom will not blur an already-disabled button. Model the observed
      // Chromium148 post-mutation BODY focus explicitly, then restore the DOM.
      const prior = document.body.getAttribute("tabindex"); document.body.tabIndex = -1; document.body.focus();
      if (prior === null) document.body.removeAttribute("tabindex"); else document.body.setAttribute("tabindex", prior);
      sawBody = document.activeElement === document.body; afterBlur?.();
    }
  });
  return { setter, sawBody: () => sawBody };
}

describe("overflow rail endpoint focus", () => {
  for (const name of ["table controls", "table status"] as const) for (const direction of ["Earlier", "Later"] as const) {
    it(`recovers ${direction} ${name} after synchronous native disable blur`, () => {
      const { button, sibling } = setup(name, direction); const blur = synchronousDisableBlur(button);
      try {
        act(() => button.focus()); fireEvent.click(button);
        expect(blur.sawBody()).toBe(true); expect(button).toBeDisabled(); expect(sibling).toBeEnabled(); expect(sibling).toHaveFocus();
        expect(screen.getByRole("textbox", { name: "Retained field" })).toHaveValue("retained draft");
      } finally { blur.setter.mockRestore(); vi.restoreAllMocks(); }
    });
  }
  it("does not steal focus from an outside control or retain a stale request", () => {
    const { viewport, button, sibling } = setup("table controls", "Earlier"), outside = screen.getByRole("button", { name: "Outside" });
    const blur = synchronousDisableBlur(button, () => outside.focus());
    try {
      act(() => button.focus()); fireEvent.click(button); expect(outside).toHaveFocus(); expect(sibling).not.toHaveFocus();
      act(() => outside.blur()); fireEvent.scroll(viewport, { target: { scrollLeft: 100 } });
      expect(document.activeElement).toBe(document.body);
    } finally { blur.setter.mockRestore(); vi.restoreAllMocks(); }
  });
  for (const invalidation of ["hidden", "owner"] as const) {
    it(`declines a ${invalidation}-changed pair before focus recovery`, () => {
      const { pair, button, sibling } = setup("table status", "Earlier");
      const blur = synchronousDisableBlur(button, () => { if (invalidation === "hidden") pair.hidden = true; else button.dataset.tableChromeOwner = "replacement-owner"; });
      try {
        act(() => button.focus()); fireEvent.click(button); expect(blur.sawBody()).toBe(true); expect(sibling).not.toHaveFocus(); expect(document.activeElement).toBe(document.body);
      } finally { blur.setter.mockRestore(); vi.restoreAllMocks(); }
    });
  }
});
