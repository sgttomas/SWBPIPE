// Issue 519: a write that settles after the session moved on states what it did to the stored
// proposal without hiding a status published while it ran, even one with identical text.
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useProjectSessionState } from "./projectSessionState";

describe("a late write's outcome on the status line", () => {
  it("takes the line, without the earlier details, when nothing was published since the write began", () => {
    const { result } = renderHook(() => useProjectSessionState());
    act(() => result.current.setProjectMessage("Opened.", "a detail of the open"));
    const seen = result.current.projectMessagePublications.current;
    act(() => result.current.settleLateProjectMessage(seen, "Saved. Replaced.", " Replaced."));
    expect(result.current.projectMessage).toBe("Saved. Replaced.");
    expect(result.current.projectMessageDetails).toBeNull();
  });

  it("is added to a status published while the write ran, even one whose text equals the earlier one", () => {
    const { result } = renderHook(() => useProjectSessionState());
    act(() => result.current.setProjectMessage("Window not closed: a dialog is open."));
    const seen = result.current.projectMessagePublications.current;
    act(() => result.current.setProjectMessage("Window not closed: a dialog is open.", "its detail"));
    act(() => result.current.settleLateProjectMessage(seen, "Saved. Replaced.", " Replaced."));
    expect(result.current.projectMessage).toBe("Window not closed: a dialog is open. Replaced.");
    expect(result.current.projectMessageDetails).toBe("its detail");
  });
});
