import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { VirtualList } from "./VirtualList";

const items = Array.from({ length: 1_000 }, (_, index) => `row:${index}`);

describe("VirtualList", () => {
  it("mounts a bounded window and scrolls an off-window active item into view", () => {
    const view = render(
      <VirtualList
        activeIndex={0}
        ariaLabel="Virtual rows"
        height={120}
        itemKey={(item) => item}
        items={items}
        renderItem={(item, index) => <button data-testid={item}>{index}</button>}
        rowHeight={30}
        role="list"
        testId="virtual-list"
      />
    );
    expect(screen.getByRole("list").querySelectorAll("[data-virtual-index]").length).toBeLessThan(30);
    expect(screen.queryByTestId("row:900")).toBeNull();
    view.rerender(
      <VirtualList
        activeIndex={900}
        ariaLabel="Virtual rows"
        height={120}
        itemKey={(item) => item}
        items={items}
        renderItem={(item, index) => <button data-testid={item}>{index}</button>}
        rowHeight={30}
        role="list"
        testId="virtual-list"
      />
    );
    expect(screen.getByTestId("virtual-list").scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("row:900")).toBeInTheDocument();
    fireEvent.scroll(screen.getByTestId("virtual-list"), { target: { scrollTop: 0 } });
    expect(screen.getByTestId("virtual-list").scrollTop).toBe(0);
  });

  it("reveals a mounted keyboard-active row through outer scroll ancestors only on request", () => {
    const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollIntoView");
    const scrollIntoView = vi.fn();
    Object.defineProperty(HTMLElement.prototype, "scrollIntoView", { configurable: true, value: scrollIntoView });
    try {
      const view = render(
        <VirtualList activeIndex={0} revealActiveRequest={0} height={120} itemKey={(item) => item} items={items} renderItem={(item) => <button data-testid={item}>{item}</button>} rowHeight={30} testId="revealed-list" />
      );
      expect(scrollIntoView).not.toHaveBeenCalled();
      view.rerender(
        <VirtualList activeIndex={900} revealActiveRequest={1} height={120} itemKey={(item) => item} items={items} renderItem={(item) => <button data-testid={item}>{item}</button>} rowHeight={30} testId="revealed-list" />
      );
      const activeRow = screen.getByTestId("row:900").closest("[data-virtual-index]");
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
      expect(scrollIntoView.mock.instances[0]).toBe(activeRow);
      expect(scrollIntoView).toHaveBeenCalledWith({ block: "nearest", inline: "nearest" });

      view.rerender(
        <VirtualList activeIndex={899} revealActiveRequest={1} height={120} itemKey={(item) => item} items={items} renderItem={(item) => <button data-testid={item}>{item}</button>} rowHeight={30} testId="revealed-list" />
      );
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
    } finally {
      if (original) Object.defineProperty(HTMLElement.prototype, "scrollIntoView", original);
      else delete (HTMLElement.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });

  it("keeps one requested focused row pinned without mounting the full list", () => {
    render(
      <VirtualList
        height={120}
        itemKey={(item) => item}
        items={items}
        pinIndex={999}
        renderItem={(item) => <span data-testid={item}>{item}</span>}
        rowHeight={30}
        testId="pinned-list"
      />
    );
    expect(screen.getByTestId("row:999")).toBeInTheDocument();
    expect(screen.getByTestId("pinned-list").querySelectorAll("[data-virtual-index]").length).toBeLessThan(30);
    fireEvent.scroll(screen.getByTestId("pinned-list"), { target: { scrollTop: 600 } });
    expect(screen.getByTestId("row:999")).toBeInTheDocument();
  });

  it("clamps a retained off-window scroll position when filtering shrinks the virtual list", () => {
    const view = render(
      <VirtualList height={120} itemKey={(item) => item} items={items} renderItem={(item) => <span data-testid={item}>{item}</span>} rowHeight={30} testId="shrinking-list" />
    );
    fireEvent.scroll(screen.getByTestId("shrinking-list"), { target: { scrollTop: 27_000 } });
    view.rerender(
      <VirtualList height={120} itemKey={(item) => item} items={items.slice(0, 150)} renderItem={(item) => <span data-testid={item}>{item}</span>} rowHeight={30} testId="shrinking-list" />
    );
    expect(screen.getByTestId("shrinking-list").scrollTop).toBeLessThanOrEqual(4_380);
    expect(screen.getByTestId("row:149")).toBeInTheDocument();
  });

  it("bounds and reveals active rows below the virtualization threshold", () => {
    const short = items.slice(0, 50);
    const view = render(<VirtualList activeIndex={0} height={96} itemKey={(item) => item} items={short} renderItem={(item) => <button data-testid={item}>{item}</button>} rowHeight={32} testId="short-list" />);
    expect(screen.getByTestId("short-list")).toHaveStyle({ height: "96px", overflowY: "auto" });
    view.rerender(<VirtualList activeIndex={49} height={96} itemKey={(item) => item} items={short} renderItem={(item) => <button data-testid={item}>{item}</button>} rowHeight={32} testId="short-list" />);
    expect(screen.getByTestId("short-list").scrollTop).toBe(1504);
    expect(screen.getByTestId("row:49")).toBeInTheDocument();
  });
});
