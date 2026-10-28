/** A failed Undo or Redo is visible where it was asked for. The toolbar
 * buttons and the ⌘Z / ⇧⌘Z accelerators run outside Operations -> Review,
 * where the operation message is shown; the toolbar's alert beside Undo and
 * Redo carries the failure, and the next model publication clears it. */
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as sessionModel from "./features/workspace/sessionModel";

afterEach(() => { vi.restoreAllMocks(); });

const cell = () => screen.getByTestId("table-cell-node:N-100-y");

/** Applies y = 0.5 through the table, as a user does, so Undo is available. */
async function appliedTableEdit() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  fireEvent.click(cell()); fireEvent.click(cell());
  fireEvent.change(screen.getByRole("textbox", { name: "node:N-100 Y [m]" }), { target: { value: "0.5" } });
  fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
  await waitFor(() => expect(cell()).toHaveTextContent("0.5"));
}

/** The identity hash of the next model commit fails. */
function failNextModelCommit() {
  vi.spyOn(sessionModel, "uiModelIdentityHash").mockImplementationOnce(() => { throw new Error("injected identity-hash failure"); });
}

function expectReviewMessageNotShown() {
  const message = screen.queryByTestId("operation-apply-message");
  if (message) expect(message).not.toBeVisible();
}

describe("a failed undo or redo is shown in the toolbar", () => {
  it("through the toolbar buttons: the failure is visible, the model is unchanged, and the next step clears it", async () => {
    await appliedTableEdit();

    failNextModelCommit();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    const alert = screen.getByRole("alert");
    expect(alert).toBe(screen.getByTestId("workspace-history-failure"));
    expect(alert).toBeVisible();
    expect(alert).toHaveTextContent("Undo failed to run: Error: injected identity-hash failure");
    expectReviewMessageNotShown();
    expect(cell()).toHaveTextContent("0.5");

    fireEvent.click(screen.getByTestId("workspace-undo"));
    expect(cell()).toHaveTextContent("0");
    expect(screen.queryByTestId("workspace-history-failure")).toBeNull();

    failNextModelCommit();
    fireEvent.click(screen.getByTestId("workspace-redo"));
    expect(screen.getByTestId("workspace-history-failure")).toBeVisible();
    expect(screen.getByTestId("workspace-history-failure")).toHaveTextContent("Redo failed to run: Error: injected identity-hash failure");
    expectReviewMessageNotShown();
    expect(cell()).toHaveTextContent("0");
    fireEvent.click(screen.getByTestId("workspace-redo"));
    expect(cell()).toHaveTextContent("0.5");
    expect(screen.queryByTestId("workspace-history-failure")).toBeNull();
  });

  it("through the ⌘Z and ⇧⌘Z accelerators: the failure is visible and the model is unchanged", async () => {
    await appliedTableEdit();
    (document.activeElement as HTMLElement | null)?.blur();

    failNextModelCommit();
    fireEvent.keyDown(window, { key: "z", metaKey: true });
    expect(screen.getByTestId("workspace-history-failure")).toBeVisible();
    expect(screen.getByTestId("workspace-history-failure")).toHaveTextContent("Undo failed to run: Error: injected identity-hash failure");
    expectReviewMessageNotShown();
    expect(cell()).toHaveTextContent("0.5");

    fireEvent.keyDown(window, { key: "z", metaKey: true });
    expect(cell()).toHaveTextContent("0");
    failNextModelCommit();
    fireEvent.keyDown(window, { key: "z", metaKey: true, shiftKey: true });
    expect(screen.getByTestId("workspace-history-failure")).toHaveTextContent("Redo failed to run: Error: injected identity-hash failure");
    expectReviewMessageNotShown();
    expect(cell()).toHaveTextContent("0");
  });
});
