import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ToolkitPalette } from "../toolkit/ToolkitPalette";
import { WorkspaceToolbar } from "./WorkspaceToolbar";

describe("workspace task toolbar", () => {
  it("keeps one compact edit/review band without the superseded task-navigation row", () => {
    const onSection = vi.fn();
    render(<WorkspaceToolbar activeSection={null} selecting pendingCount={3} canUndo={false} canRedo={false} onSelect={vi.fn()} onSection={onSection} onUndo={vi.fn()} onRedo={vi.fn()}><button>Toolkit</button></WorkspaceToolbar>);
    expect(screen.queryByRole("navigation", { name: "Workspace tasks" })).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-review"));
    expect(onSection).toHaveBeenLastCalledWith("operations");
    expect(screen.getByTestId("workspace-review")).toHaveTextContent("3");
    expect(screen.getByRole("status", { name: "Current workspace task" })).toHaveTextContent("Model");
    expect(screen.getByRole("button", { name: "Undo model edit" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Redo model edit" })).toBeDisabled();
  });

  it("keeps the compact seven-group command band in the toolbar", () => {
    render(<WorkspaceToolbar activeSection={null} selecting pendingCount={0} canUndo={false} canRedo={false} onSelect={vi.fn()} onSection={vi.fn()} onUndo={vi.fn()} onRedo={vi.fn()}>
      <ToolkitPalette
        context={{ selection: { type: "project", id: "project:test" }, selectionCardinality: 1, canUndo: false, canRedo: false, busy: false }}
        onChoose={vi.fn()}
      />
    </WorkspaceToolbar>);
    const toolbar = screen.getByTestId("workspace-toolbar");
    for (const id of ["build", "supports", "properties", "loads", "edit", "select-view", "review"]) {
      expect(toolbar).toContainElement(screen.getByTestId(`toolkit-group-${id}`));
    }
    expect(toolbar).toContainElement(screen.getByRole("button", { name: "Find modeling commands" }));
    expect(toolbar.querySelectorAll(".workspace-task-nav")).toHaveLength(0);
  });
});
