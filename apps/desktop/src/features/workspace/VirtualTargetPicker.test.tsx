import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { VirtualTargetPicker } from "./VirtualTargetPicker";

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

  it("keeps a valid current value while it is filtered out and clears explicitly", () => {
    const onChange = vi.fn();
    render(<VirtualTargetPicker label="Support node" onChange={onChange} options={options} testId="support-node" value="node:2" />);
    fireEvent.change(screen.getByRole("combobox", { name: "Support node" }), { target: { value: "Node 149" } });
    expect(screen.getByLabelText("Support node current value")).toHaveTextContent("Node 2 (node:2)");
    fireEvent.click(screen.getByRole("button", { name: "Clear" }));
    expect(onChange).toHaveBeenCalledWith("");
  });
});
