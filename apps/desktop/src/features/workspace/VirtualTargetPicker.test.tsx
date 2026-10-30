import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { VirtualMultiTargetPicker, VirtualTargetPicker } from "./VirtualTargetPicker";

const options = Array.from({ length: 150 }, (_, index) => ({
  value: `node:${index}`,
  label: `Node ${index}`,
  keywords: [`branch ${Math.floor(index / 10)}`]
}));

describe("VirtualTargetPicker", () => {
  it("searches a virtual target set and supports End/Enter selection", () => {
    const onChange = vi.fn();
    render(<VirtualTargetPicker label="Pipe endpoint" onChange={onChange} options={options} testId="endpoint" value="" />);
    expect(screen.getByText("150 of 150 targets")).toBeInTheDocument();
    expect(screen.getByTestId("endpoint-list").querySelectorAll("[data-virtual-index]").length).toBeLessThan(30);
    const input = screen.getByRole("combobox", { name: "Pipe endpoint" });
    fireEvent.change(input, { target: { value: "branch 14" } });
    expect(screen.getByText("10 of 150 targets")).toBeInTheDocument();
    fireEvent.keyDown(input, { key: "End" });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onChange).toHaveBeenLastCalledWith("node:149");
  });

  it("reveals the 99th keyboard-active option through outer scrollers without moving focus or reacting to hover", () => {
    const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollIntoView");
    const scrollIntoView = vi.fn();
    Object.defineProperty(HTMLElement.prototype, "scrollIntoView", { configurable: true, value: scrollIntoView });
    try {
      render(<div style={{ height: 177, overflowY: "auto" }}><VirtualTargetPicker label="Bounded target" onChange={vi.fn()} options={options.slice(0, 99)} testId="bounded-target" value="" /></div>);
      const input = screen.getByRole("combobox", { name: "Bounded target" });
      input.focus();
      expect(scrollIntoView).not.toHaveBeenCalled();

      fireEvent.keyDown(input, { key: "End" });
      expect(input).toHaveFocus();
      const activeId = input.getAttribute("aria-activedescendant");
      const activeOption = activeId ? document.getElementById(activeId) : null;
      expect(activeOption).toHaveTextContent("Node 98");
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
      expect(scrollIntoView.mock.instances[0]).toBe(activeOption?.closest("[data-virtual-index]"));

      fireEvent.mouseMove(screen.getByRole("option", { name: /Node 97/ }));
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
      expect(input).toHaveFocus();
    } finally {
      if (original) Object.defineProperty(HTMLElement.prototype, "scrollIntoView", original);
      else delete (HTMLElement.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });

  it("keeps a valid current value while it is filtered out and clears explicitly", () => {
    const onChange = vi.fn();
    render(<VirtualTargetPicker label="Support node" onChange={onChange} options={options} testId="support-node" value="node:2" />);
    fireEvent.change(screen.getByRole("combobox", { name: "Support node" }), { target: { value: "Node 149" } });
    expect(screen.getByLabelText("Support node current value")).toHaveTextContent("Node 2 (node:2)");
    fireEvent.click(screen.getByRole("button", { name: "Clear" }));
    expect(onChange).toHaveBeenCalledWith("");
  });

  it("owns the controlled listbox and blocks every mutation while disabled", () => {
    const onChange = vi.fn();
    render(<VirtualTargetPicker disabled label="Disabled target" onChange={onChange} options={options} testId="disabled" value="node:2" />);
    const input = screen.getByRole("combobox", { name: "Disabled target" });
    const list = screen.getByRole("listbox", { name: "Disabled target options" });
    expect(input).toHaveAttribute("aria-controls", list.id);
    expect(screen.getByRole("button", { name: "Clear" })).toBeDisabled();
    expect(screen.getByRole("option", { name: /Node 0/ })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Clear" }));
    fireEvent.click(screen.getByRole("option", { name: /Node 0/ }));
    expect(onChange).not.toHaveBeenCalled();
  });

  it("keeps spaces in multiword search and uses Enter to toggle the active multi target", () => {
    const onChange = vi.fn();
    render(<VirtualMultiTargetPicker label="Copy targets" onChange={onChange} options={options} testId="copy-targets" values={[]} />);
    const input = screen.getByRole("combobox", { name: "Copy targets" });
    fireEvent.change(input, { target: { value: "branch" } });
    fireEvent.keyDown(input, { key: " " });
    fireEvent.change(input, { target: { value: "branch 14" } });
    expect(input).toHaveValue("branch 14");
    fireEvent.keyDown(input, { key: "End" });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onChange).toHaveBeenLastCalledWith(["node:149"]);
  });

  it("reveals multi-target keyboard navigation without query, hover, or Enter jumps", () => {
    const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollIntoView");
    const scrollIntoView = vi.fn();
    Object.defineProperty(HTMLElement.prototype, "scrollIntoView", { configurable: true, value: scrollIntoView });
    try {
      render(<div style={{ height: 177, overflowY: "auto" }}><VirtualMultiTargetPicker label="Bounded copy targets" onChange={vi.fn()} options={options.slice(0, 99)} testId="bounded-copy-targets" values={[]} /></div>);
      const input = screen.getByRole("combobox", { name: "Bounded copy targets" });
      input.focus();
      expect(scrollIntoView).not.toHaveBeenCalled();
      fireEvent.change(input, { target: { value: "branch 9" } });
      expect(scrollIntoView).not.toHaveBeenCalled();

      fireEvent.keyDown(input, { key: "End" });
      expect(input).toHaveFocus();
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
      const activeId = input.getAttribute("aria-activedescendant");
      const activeOption = activeId ? document.getElementById(activeId) : null;
      expect(scrollIntoView.mock.instances[0]).toBe(activeOption?.closest("[data-virtual-index]"));

      fireEvent.keyDown(input, { key: "Enter" });
      fireEvent.mouseMove(screen.getByRole("option", { name: /Node 98/ }));
      expect(scrollIntoView).toHaveBeenCalledTimes(1);
      expect(input).toHaveFocus();
    } finally {
      if (original) Object.defineProperty(HTMLElement.prototype, "scrollIntoView", original);
      else delete (HTMLElement.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });
});
