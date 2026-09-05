import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ToolkitPalette } from "./ToolkitPalette";
import { capabilityAvailability, capabilityRoute, toolkitCapabilities, type ToolkitContext } from "./capabilityCatalog";

const context: ToolkitContext = { selection: { type: "project", id: "project:test" }, canUndo: false, canRedo: false, busy: false };
describe("human toolkit", () => {
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
    fireEvent.click(screen.getByRole("button", { name: "Toolkit" }));
    fireEvent.change(screen.getByRole("searchbox"), { target: { value: "temperature table" } });
    const command = screen.getByRole("button", { name: "Material temperature table" });
    expect(command).toBeDisabled();
    expect(screen.getByText("Select a material in the model tree first.")).toBeVisible();
    fireEvent.click(command);
    expect(onChoose).not.toHaveBeenCalled();
    fireEvent.keyDown(screen.getByRole("searchbox"), { key: "Escape" });
    expect(screen.getByRole("button", { name: "Toolkit" })).toHaveFocus();
    expect(screen.queryByRole("searchbox")).not.toBeInTheDocument();
  });
  it("contains keyboard focus in discovery and dismisses to its trigger", () => {
    render(<ToolkitPalette context={context} onChoose={vi.fn()} />);
    fireEvent.click(screen.getByRole("button", { name: "Toolkit" }));
    const search = screen.getByRole("searchbox");
    expect(search).toHaveFocus();
    fireEvent.keyDown(search, { key: "Tab", shiftKey: true });
    expect(screen.getByText("Deferred roadmap")).toHaveFocus();
    fireEvent.keyDown(document.activeElement!, { key: "Tab" });
    expect(search).toHaveFocus();
    fireEvent.pointerDown(document.body);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Toolkit" })).toHaveFocus();
  });
  it("requires appropriate selection and available history before dispatch", () => {
    const remove = toolkitCapabilities.find((entry) => entry.id === "edit.remove")!;
    expect(capabilityAvailability(remove, context).enabled).toBe(false);
    expect(capabilityRoute(remove, { ...context, selection: { type: "pipe", id: "pipe:chosen" } })).toEqual({ surface: "inspector", focusTestId: "queue-delete-pipe-intent" });
    for (const action of ["undo", "redo"] as const) {
      const command = toolkitCapabilities.find((entry) => entry.history === action)!;
      expect(capabilityAvailability(command, context).enabled).toBe(false);
      expect(capabilityAvailability(command, { ...context, canUndo: true, canRedo: true, busy: true }).enabled).toBe(false);
    }
  });
  it("dispatches the stable command once and closes the palette", () => {
    const onChoose = vi.fn();
    render(<ToolkitPalette context={context} onChoose={onChoose} />);
    fireEvent.click(screen.getByRole("button", { name: "Toolkit" }));
    fireEvent.click(screen.getByTestId("toolkit-properties.material"));
    expect(onChoose).toHaveBeenCalledOnce();
    expect(onChoose.mock.calls[0][0].id).toBe("properties.material");
    expect(screen.queryByRole("searchbox")).not.toBeInTheDocument();
  });
});
