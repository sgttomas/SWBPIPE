/** The Project page in the whole window, in a project saved in the browser preview: its
 * header's buttons, the project's line, the saved projects, Model History, and the storage
 * record closed, with both storage panels and their controls inside it. All data are invented. */
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";

afterEach(() => {
  invokeMock.mockReset();
  window.localStorage.clear();
});

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

describe("the Project page in a saved project", () => {
  it("leads with the project and its history, and keeps the storage record closed with both panels inside", async () => {
    Object.defineProperty(window, "innerWidth", { configurable: true, value: 1440 });
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    act(() => nativeMenuCommand("view.section.project"));
    const section = screen.getByTestId("workspace-section-project");
    await waitFor(() => expect(section).not.toHaveClass("inactive"));
    const page = await within(section).findByTestId("project-page");
    expect(within(page).getByTestId("project-page-saved-line")).toHaveTextContent(/^Not saved as a project\.$/);

    // The header holds New, Open… and Save; New's dialog offers the starting model.
    const controls = screen.getByLabelText("Local project controls");
    expect(within(controls).getAllByRole("button").map((button) => button.textContent?.trim())).toEqual(["New", "Open…", "Save"]);

    fireEvent.click(screen.getByTestId("project-save"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Saved"));
    await waitFor(() => expect(within(page).getByTestId("project-page-saved-line")).toHaveTextContent(/^Saved \d{4}-\d\d-\d\d \d\d:\d\d:\d\d\. No changes since\.$/));
    const name = within(page).getByTestId("project-page-name").textContent ?? "";
    expect(name).not.toBe("");

    // The saved projects, without a press: this one, by its name, marked as the one open.
    const saved = within(page).getByTestId("project-saved-list");
    await waitFor(() => expect(within(saved).getByText(name, { exact: false }).closest("button")).toHaveTextContent("open now"));
    // Recover is not offered: the browser preview keeps no autosave.
    expect(within(page).queryByTestId("project-recover-autosave")).toBeNull();
    expect(within(page).getByTestId("model-history-panel")).toBeInTheDocument();

    const record = within(page).getByTestId("project-storage-record") as HTMLDetailsElement;
    expect(record.open).toBe(false);
    for (const id of ["project-storage-panel", "project-validation-panel"]) expect(within(record).getByTestId(id)).toBeInTheDocument();
    // Nothing of the storage record is outside it, and no project:… identifier is on the page before it.
    expect(within(page).getAllByTestId(/^project-(storage|validation)-panel$/).every((panel) => record.contains(panel))).toBe(true);
    for (const part of [within(page).getByTestId("project-page-name").parentElement!, saved, within(page).getByTestId("model-history-panel")]) {
      expect(part).not.toHaveTextContent("project:");
    }
    fireEvent.click(within(record).getByText("Storage record", { selector: "summary" }));
    expect(record.open).toBe(true);
    for (const id of ["project-storage-export-link", "project-storage-export-link-local-private-intent", "project-storage-summary",
      "project-validation-export-link", "project-validation-export-link-local-private-intent", "project-validation-summary"]) {
      expect(within(record).getByTestId(id)).toBeInTheDocument();
    }
  });
});
