import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { WorkspaceToolbar } from "./WorkspaceToolbar";

describe("workspace task toolbar", () => {
  it("keeps every main task and review reachable without an application menu", () => {
    const onSection = vi.fn();
    render(<WorkspaceToolbar activeSection={null} selecting pendingCount={3} canUndo={false} canRedo={false} onSelect={vi.fn()} onSection={onSection} onUndo={vi.fn()} onRedo={vi.fn()}><button>Toolkit</button></WorkspaceToolbar>);
    for (const [task, value] of [["Model", null], ["Loads", "loads"], ["Analyze", "solve"], ["Results", "results"], ["Rules", "rule-packs"], ["Report", "report"]]) {
      fireEvent.click(screen.getByRole("button", { name: task! }));
      expect(onSection).toHaveBeenLastCalledWith(value);
    }
    fireEvent.click(screen.getByTestId("workspace-review"));
    expect(onSection).toHaveBeenLastCalledWith("operations");
    expect(screen.getByTestId("workspace-review")).toHaveTextContent("3");
    expect(screen.getByRole("button", { name: "Undo model edit" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Redo model edit" })).toBeDisabled();
  });
});
