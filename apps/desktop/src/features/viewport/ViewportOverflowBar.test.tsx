import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type ReactNode, useState } from "react";
import { ViewportOverflowBar, type ViewportBarGroup } from "./ViewportOverflowBar";
import type { ObserveSize } from "./viewportSizeWatch";

// jsdom has no layout. The bar's widths are given here as a browser would report them: the bar,
// each group and More. The fitting rule is what is tested; the drawn layout is the browser run's.
const WIDTHS: Record<string, number> = { display: 160, selection: 220, geometry: 140, axes: 100, measure: 70 };
const MORE = 60;
const GAP = 8;
let barWidth = 0;

function rect(width: number): DOMRect {
  return { x: 0, y: 0, top: 0, left: 0, right: width, bottom: 24, width, height: 24, toJSON: () => ({}) } as DOMRect;
}

function installLayout() {
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    if (this.dataset.testid === "viewport-bar") return rect(barWidth);
    if (this.classList.contains("viewport-more")) return rect(MORE);
    const group = this.dataset.barGroup;
    if (group) return rect(WIDTHS[group] ?? 0);
    return rect(0);
  });
  const real = window.getComputedStyle;
  vi.spyOn(window, "getComputedStyle").mockImplementation((element: Element) => {
    const style = real(element);
    if (element instanceof HTMLElement && (element.dataset.testid === "viewport-bar" || element.classList.contains("viewport-bar-groups"))) {
      return { ...style, getPropertyValue: (name: string) => name === "column-gap" ? `${GAP}px` : name.startsWith("padding") ? "0px" : style.getPropertyValue(name) } as CSSStyleDeclaration;
    }
    return style;
  });
}

function Harness({ watchSize }: { watchSize?: ObserveSize } = {}) {
  const [measure, setMeasure] = useState(false);
  const [boxSelect, setBoxSelect] = useState(false);
  const group = (key: string, label: string, content: ReactNode, onCount = 0): ViewportBarGroup =>
    ({ key, label, heading: label, onCount, signature: key, content });
  return (
    <ViewportOverflowBar
      watchSize={watchSize}
      label="Viewport controls"
      groups={[
        group("display", "Display", <button type="button" aria-pressed="true">Labels</button>, 1),
        group("selection", "Selection", <>
          <button type="button" data-testid="box" aria-pressed={boxSelect} onClick={() => setBoxSelect((value) => !value)}>Box Select</button>
          <button type="button" disabled title="Select geometry first.">Hide</button>
        </>, boxSelect ? 1 : 0),
        group("geometry", "Geometry", <button type="button" aria-pressed="false">Actual OD</button>),
        group("axes", "Element axes", <select aria-label="Element axes"><option>No axes</option></select>),
        group("measure", "Measure", <button type="button" aria-pressed={measure} data-testid="measure" onClick={() => setMeasure((value) => !value)}>Measure</button>, measure ? 1 : 0)
      ]}
      menuGroups={[group("fit", "Fit", <button type="button" data-testid="fit-visible">Fit Visible</button>)]}
    />
  );
}

function barGroups(): string[] {
  return [...screen.getByTestId("viewport-bar").querySelectorAll<HTMLElement>("[data-bar-group]")].map((element) => element.dataset.barGroup!);
}

afterEach(() => vi.restoreAllMocks());

describe("the view's bar and its More menu", () => {
  it("creates no ResizeObserver: it watches the bar and its shown groups through the view's watch, once each, for its lifetime", () => {
    installLayout();
    barWidth = 2000;
    const constructed = vi.fn();
    vi.stubGlobal("ResizeObserver", class { constructor() { constructed(); } observe() {} unobserve() {} disconnect() {} });
    const callbacks = new Map<Element, () => void>();
    const stops: Element[] = [];
    const watchSize = vi.fn((element: Element, callback: () => void) => {
      callbacks.set(element, callback);
      return () => { stops.push(element); callbacks.delete(element); };
    });
    try {
      const view = render(<Harness watchSize={watchSize} />);
      expect(watchSize).toHaveBeenCalledTimes(6);
      // Renders (a control switched on and off) watch nothing again.
      fireEvent.click(screen.getByTestId("measure"));
      fireEvent.click(screen.getByTestId("measure"));
      expect(watchSize).toHaveBeenCalledTimes(6);
      // The bar narrows: its watch refits it, and the groups that left the bar are no longer watched.
      barWidth = 620;
      act(() => callbacks.get(screen.getByTestId("viewport-bar"))!());
      expect(barGroups()).toEqual(["display", "selection", "geometry"]);
      expect(stops).toHaveLength(2);
      view.unmount();
      expect(callbacks.size).toBe(0);
      expect(constructed).not.toHaveBeenCalled();
    } finally {
      vi.unstubAllGlobals();
    }
  });

  it("keeps every group in the bar while the bar has room", () => {
    installLayout();
    barWidth = 2000;
    render(<Harness />);
    expect(barGroups()).toEqual(["display", "selection", "geometry", "axes", "measure"]);
  });

  it("moves the lowest-priority groups whole into More while the bar is too narrow", () => {
    installLayout();
    // 160 + 8 + 220 + 8 + 140 = 536 fits beside More (60 + gap 8) in 620; adding axes (644) does not.
    barWidth = 620;
    render(<Harness />);
    expect(barGroups()).toEqual(["display", "selection", "geometry"]);
    expect(screen.queryByRole("button", { name: "Measure" })).toBeNull();

    fireEvent.click(screen.getByTestId("viewport-more"));
    const menu = screen.getByTestId("viewport-more-menu");
    expect(menu).toBeVisible();
    // The same controls, whole groups, in priority order, then the menu's own Fit group.
    expect([...menu.querySelectorAll<HTMLElement>("[data-menu-group]")].map((element) => element.dataset.menuGroup)).toEqual(["axes", "measure", "fit"]);
    expect(within(menu).getByRole("combobox", { name: "Element axes" })).toBeInTheDocument();
    expect(within(menu).getByRole("button", { name: "Measure" })).toHaveAttribute("aria-pressed", "false");
  });

  it("shows on More how many controls in it are on", () => {
    installLayout();
    barWidth = 620;
    render(<Harness />);
    const more = screen.getByTestId("viewport-more");
    expect(more).toHaveAccessibleName("More view controls");
    fireEvent.click(more);
    expect(screen.getByTestId("viewport-more-menu")).toBeVisible();
    fireEvent.click(within(screen.getByTestId("viewport-more-menu")).getByTestId("measure"));
    // A toggle leaves the menu open, so its state can be read where it was changed.
    expect(within(screen.getByTestId("viewport-more-menu")).getByTestId("measure")).toHaveAttribute("aria-pressed", "true");
    expect(more).toHaveAccessibleName("More view controls, 1 on");
    expect(screen.getByTestId("viewport-more-count")).toHaveTextContent("1");
  });

  it("closes on Escape, returning focus to More, and on a press outside", () => {
    installLayout();
    barWidth = 2000;
    render(<><Harness /><button type="button">outside</button></>);
    const more = screen.getByTestId("viewport-more");
    fireEvent.click(more);
    expect(more).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByTestId("viewport-more-menu")).toBeVisible();
    fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });
    expect(screen.queryByTestId("viewport-more-menu")).toBeNull();
    expect(more).toHaveFocus();

    fireEvent.click(more);
    expect(screen.getByTestId("viewport-more-menu")).toBeVisible();
    fireEvent.pointerDown(screen.getByRole("button", { name: "outside" }));
    expect(screen.queryByTestId("viewport-more-menu")).toBeNull();
  });

  it("closes after a command and keeps a disabled control's reason", () => {
    installLayout();
    barWidth = 200;
    render(<Harness />);
    fireEvent.click(screen.getByTestId("viewport-more"));
    const menu = screen.getByTestId("viewport-more-menu");
    expect(menu).toBeVisible();
    expect(within(menu).getByRole("button", { name: "Hide" })).toBeDisabled();
    expect(within(menu).getByRole("button", { name: "Hide" })).toHaveAttribute("title", "Select geometry first.");
    act(() => { fireEvent.click(within(menu).getByTestId("fit-visible")); });
    expect(screen.queryByTestId("viewport-more-menu")).toBeNull();
  });

  it("keeps every group in the bar when no width can be measured", () => {
    barWidth = 0;
    render(<Harness />);
    expect(barGroups()).toEqual(["display", "selection", "geometry", "axes", "measure"]);
  });
});
