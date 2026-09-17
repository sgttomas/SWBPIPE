import { act, fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ToolkitPalette } from "./ToolkitPalette";
import { capabilityAvailability, capabilityRoute, toolkitCapabilities, type ToolkitContext } from "./capabilityCatalog";

const context: ToolkitContext = { selection: { type: "project", id: "project:test" }, selectionCardinality: 1, canUndo: false, canRedo: false, busy: false };
describe("human toolkit", () => {
  it("keeps explicit command focus after queued opener callbacks flush", () => {
    const callbacks: FrameRequestCallback[] = [];
    const raf = vi.spyOn(window, "requestAnimationFrame").mockImplementation((callback) => {
      callbacks.push(callback); return callbacks.length;
    });
    try {
      render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
      fireEvent.click(screen.getByTestId("toolkit-entry"));
      const command = screen.getByTestId("toolkit-build.node");
      expect(command).toBeEnabled();
      command.focus();
      expect(command).toHaveFocus();
      act(() => { for (const callback of callbacks.splice(0)) callback(0); });
      expect(command).toHaveFocus();
      expect(raf).not.toHaveBeenCalled();
    } finally { raf.mockRestore(); }
  });
  it("keeps reopened command and dispatched destination focus after old callbacks flush", () => {
    const callbacks: FrameRequestCallback[] = [];
    const raf = vi.spyOn(window, "requestAnimationFrame").mockImplementation((callback) => {
      callbacks.push(callback); return callbacks.length;
    });
    const destination = document.createElement("input");
    document.body.append(destination);
    const onChoose = vi.fn(() => destination.focus());
    try {
      render(<ToolkitPalette context={context} onChoose={onChoose} />);
      fireEvent.click(screen.getByTestId("toolkit-entry"));
      fireEvent.keyDown(screen.getByRole("searchbox"), { key: "Escape" });
      fireEvent.click(screen.getByTestId("toolkit-group-build"));
      const command = screen.getByTestId("toolkit-build.node");
      command.focus();
      act(() => { for (const callback of callbacks.splice(0)) callback(0); });
      expect(command).toHaveFocus();
      fireEvent.click(command);
      expect(onChoose).toHaveBeenCalledExactlyOnceWith(expect.objectContaining({ id: "build.node" }));
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
      act(() => { for (const callback of callbacks.splice(0)) callback(0); });
      expect(destination).toHaveFocus();
    } finally { raf.mockRestore(); destination.remove(); }
  });
  it("consumes owned backdrop pointer default and returns the group invoker", () => {
    render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
    const invoker = screen.getByTestId("toolkit-group-build");
    fireEvent.click(invoker);
    const event = new Event("pointerdown", { bubbles: true, cancelable: true });
    fireEvent(document.querySelector(".toolkit-backdrop")!, event);
    expect(event.defaultPrevented).toBe(true);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(invoker).toHaveFocus();
  });
  it("represents all accepted vocabulary rows and only grants executable routes", () => {
    expect([...new Set(toolkitCapabilities.flatMap((entry) => entry.vocabularyRows))].sort((a, b) => a - b)).toEqual(Array.from({ length: 24 }, (_, i) => i + 1));
    expect(new Set(toolkitCapabilities.map((entry) => entry.id)).size).toBe(toolkitCapabilities.length);
    for (const entry of toolkitCapabilities) {
      if (capabilityAvailability(entry, context).enabled) expect(entry.route || entry.history).toBeTruthy();
      if (["unavailable", "gated"].includes(entry.status)) expect(capabilityRoute(entry, context)).toBeUndefined();
    }
  });
  it("searches descriptions and explains a missing selection without dispatch", () => {
    const onChoose = vi.fn();
    render(<ToolkitPalette context={context} onChoose={onChoose} />);
    fireEvent.click(screen.getByRole("button", { name: "Find modeling commands" }));
    fireEvent.change(screen.getByRole("searchbox"), { target: { value: "temperature table" } });
    const command = screen.getByRole("button", { name: "Material temperature table" });
    expect(command).toBeDisabled();
    expect(screen.getByText("Select a material in the model tree first.")).toBeVisible();
    fireEvent.click(command);
    expect(onChoose).not.toHaveBeenCalled();
    fireEvent.keyDown(screen.getByRole("searchbox"), { key: "Escape" });
    expect(screen.getByRole("button", { name: "Find modeling commands" })).toHaveFocus();
    expect(screen.queryByRole("searchbox")).not.toBeInTheDocument();
  });
  it("contains keyboard focus in discovery and dismisses to its trigger", () => {
    render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
    fireEvent.click(screen.getByRole("button", { name: "Find modeling commands" }));
    const search = screen.getByRole("searchbox");
    expect(search).toHaveFocus();
    fireEvent.keyDown(search, { key: "Tab", shiftKey: true });
    expect(screen.getByText("Deferred roadmap")).toHaveFocus();
    fireEvent.keyDown(document.activeElement!, { key: "Tab" });
    expect(search).toHaveFocus();
    fireEvent.pointerDown(document.querySelector(".toolkit-backdrop")!);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Find modeling commands" })).toHaveFocus();
  });
  it("requires appropriate selection and available history before dispatch", () => {
    const remove = toolkitCapabilities.find((entry) => entry.id === "edit.remove")!;
    expect(capabilityAvailability(remove, context).enabled).toBe(false);
    expect(capabilityRoute(remove, { ...context, selection: { type: "pipe", id: "pipe:chosen" } })).toEqual({ surface: "inspector", inspectorView: "properties", focusTestId: "queue-delete-pipe-intent" });
    for (const action of ["undo", "redo"] as const) {
      const command = toolkitCapabilities.find((entry) => entry.history === action)!;
      expect(capabilityAvailability(command, context).enabled).toBe(false);
      expect(capabilityAvailability(command, { ...context, canUndo: true, canRedo: true, busy: true }).enabled).toBe(false);
    }
  });
  it("binds the Undo catalogue command to undo and dispatches that exact capability", () => {
    const onChoose = vi.fn();
    const undo = toolkitCapabilities.find((entry) => entry.id === "edit.undo")!;
    const redo = toolkitCapabilities.find((entry) => entry.id === "edit.redo")!;
    expect(undo.history).toBe("undo");
    expect(redo.history).toBe("redo");

    render(<ToolkitPalette context={{ ...context, canUndo: true }} onChoose={onChoose} />);
    fireEvent.click(screen.getByRole("button", { name: "Edit" }));
    fireEvent.click(screen.getByRole("button", { name: "Undo" }));
    expect(onChoose).toHaveBeenCalledOnce();
    expect(onChoose).toHaveBeenLastCalledWith(expect.objectContaining({ id: "edit.undo", history: "undo" }));
  });
  it("blocks single-target mutation routes for an ordered multi-selection", () => {
    const multiSelection = {
      ...context,
      selection: { type: "pipe", id: "pipe:chosen" } as const,
      selectionCardinality: 2,
      canUndo: true,
      canRedo: true
    };
    const singleTargetIds = [
      "build.node", "build.pipe", "build.component", "build.split",
      "supports.restraint", "supports.hanger", "supports.nonlinear", "supports.boundary",
      "properties.material", "properties.temperature", "properties.section", "properties.assign-section", "properties.hanger-library",
      "loads.cases", "loads.primitive", "loads.wind", "loads.wind-exposure", "loads.seismic", "loads.combinations", "edit.remove"
    ];
    for (const id of singleTargetIds) {
      const command = toolkitCapabilities.find((entry) => entry.id === id)!;
      expect(capabilityAvailability(command, multiSelection)).toEqual({
        enabled: false,
        reason: "This command changes one typed target. Reduce the selection to one item before starting it."
      });
      expect(capabilityRoute(command, multiSelection)).toBeUndefined();
    }
    expect(capabilityAvailability(toolkitCapabilities.find((entry) => entry.id === "loads.self-weight")!, multiSelection).enabled).toBe(true);
    expect(capabilityAvailability(toolkitCapabilities.find((entry) => entry.id === "edit.undo")!, multiSelection).enabled).toBe(true);
  });
  it("dispatches the stable command once and closes the palette", () => {
    const onChoose = vi.fn();
    render(<ToolkitPalette context={context} onChoose={onChoose} />);
    fireEvent.click(screen.getByRole("button", { name: "Find modeling commands" }));
    fireEvent.click(screen.getByTestId("toolkit-properties.material"));
    expect(onChoose).toHaveBeenCalledOnce();
    expect(onChoose.mock.calls[0][0].id).toBe("properties.material");
    expect(screen.queryByRole("searchbox")).not.toBeInTheDocument();
  });
  it("opens grouped command search from the visible platform shortcut", () => {
    render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
    expect(screen.getByText("⌘K")).toBeVisible();
    fireEvent.keyDown(window, { key: "k", metaKey: true });
    expect(screen.getByRole("dialog", { name: "Find a modeling tool" })).toBeVisible();
    for (const group of ["Build", "Supports", "Properties", "Loads", "Edit", "Select and View", "Review"]) {
      expect(screen.getByRole("heading", { name: group })).toBeVisible();
    }
  });
  it("exposes every command group in the visible compact band and filters the palette", () => {
    render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
    for (const label of ["Build", "Supports", "Properties", "Loads", "Edit", "Select/View", "Review"]) {
      expect(screen.getByRole("button", { name: label })).toBeVisible();
    }
    fireEvent.click(screen.getByRole("button", { name: "Supports" }));
    expect(screen.getByRole("status")).toHaveTextContent("Showing Supports commands");
    expect(screen.getByRole("heading", { name: "Supports" })).toBeVisible();
    expect(screen.queryByRole("heading", { name: "Build" })).not.toBeInTheDocument();
  });
});
