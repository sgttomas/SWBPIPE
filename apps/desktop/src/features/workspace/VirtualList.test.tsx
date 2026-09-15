import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
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
});
