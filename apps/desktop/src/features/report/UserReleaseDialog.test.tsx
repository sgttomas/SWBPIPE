// W-18b desktop slice (DEL-08-01 Design `report-user-release.md` §2.2, §2.3): the user-release
// dialog. The statement is the record's `statement`, verbatim; the checkbox starts unticked at
// every opening; the save is disabled until it is ticked and while a refusal stands; Escape and
// Cancel discard the selection. All data here is invented; no user release is recorded.
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import path from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  USER_RELEASE_DIALOG_TITLE,
  USER_RELEASE_LIBRARY_REFUSAL,
  USER_RELEASE_RESULT_REFUSAL,
  UserReleaseDialog,
  type UserReleaseDialogRequest
} from "./UserReleaseDialog";
import { USER_RELEASE_STATEMENT } from "./reportPackageRequest";

afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

const root = path.resolve(__dirname, "../../../../..");

function request(overrides: Partial<UserReleaseDialogRequest> = {}): UserReleaseDialogRequest {
  return {
    projectName: "Invented project",
    modelDocumentSha256: "0123456789abcdef".repeat(4),
    runId: "run:invented-001",
    refusal: null,
    opener: null,
    cancel: vi.fn(),
    save: vi.fn(),
    ...overrides
  };
}

describe("UserReleaseDialog", () => {
  it("states the statement verbatim as the record's and the renderer's constant", () => {
    const renderer = readFileSync(path.join(root, "core/reporting/report_renderer/src/lib.rs"), "utf8");
    const constant = /pub const USER_RELEASE_STATEMENT: &str =\s*"([^"]+)";/.exec(renderer)?.[1];
    expect(constant).toBe(USER_RELEASE_STATEMENT);
    render(<UserReleaseDialog request={request()} />);
    const statement = screen.getByTestId("user-release-statement");
    expect(statement.closest("label")).toHaveTextContent(USER_RELEASE_STATEMENT);
    expect(screen.getByRole("dialog")).toHaveAttribute("aria-modal", "true");
    expect(screen.getByRole("heading")).toHaveTextContent(USER_RELEASE_DIALOG_TITLE);
  });

  it("names the one report it covers and what it carries", () => {
    render(<UserReleaseDialog request={request()} />);
    expect(screen.getByTestId("user-release-scope")).toHaveTextContent(
      'It covers only this report: model "Invented project" (SHA-256 0123456789ab…), run run:invented-001.'
    );
    const terms = screen.getByTestId("user-release-terms");
    expect(terms).toHaveTextContent("in the report sections you chose; every result row is still listed with its checksum.");
    expect(terms).toHaveTextContent("any model-input tables");
    expect(terms).toHaveTextContent(
      "It needs a result solved in this session: a pressure-free (preview-physics-2) result, or an exact pressure route (pressure-2 or pressure-source-1) result that is eligible for output."
    );
    expect(terms).toHaveTextContent("Nothing is remembered: your next report is local private unless you choose this again.");
  });

  it("takes focus on the unticked statement and saves only once it is ticked, with the clock's reading", () => {
    vi.useFakeTimers({ now: new Date("2026-11-09T10:11:12.345Z"), toFake: ["Date"] });
    const save = vi.fn();
    render(<UserReleaseDialog request={request({ save })} />);
    const statement = screen.getByTestId("user-release-statement") as HTMLInputElement;
    const confirm = screen.getByTestId("user-release-confirm");
    expect(document.activeElement).toBe(statement);
    expect(statement.checked).toBe(false);
    expect(confirm).toBeDisabled();
    fireEvent.click(confirm);
    expect(save).not.toHaveBeenCalled();
    fireEvent.click(statement);
    expect(confirm).toBeEnabled();
    fireEvent.click(confirm);
    expect(save).toHaveBeenCalledWith({ userReleasedAt: "2026-11-09T10:11:12Z" });
  });

  it("starts unticked at every opening", () => {
    const { rerender } = render(<UserReleaseDialog request={request()} />);
    fireEvent.click(screen.getByTestId("user-release-statement"));
    rerender(<UserReleaseDialog request={null} />);
    rerender(<UserReleaseDialog request={request()} />);
    expect((screen.getByTestId("user-release-statement") as HTMLInputElement).checked).toBe(false);
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
  });

  it("discards the selection on Escape and on Cancel", () => {
    const cancel = vi.fn();
    const save = vi.fn();
    render(<UserReleaseDialog request={request({ cancel, save })} />);
    fireEvent.click(screen.getByTestId("user-release-statement"));
    act(() => { window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" })); });
    expect(cancel).toHaveBeenCalledTimes(1);
    fireEvent.click(screen.getByTestId("user-release-cancel"));
    expect(cancel).toHaveBeenCalledTimes(2);
    expect(save).not.toHaveBeenCalled();
  });

  it("shows the library or rule-pack refusal in place of the statement", () => {
    render(<UserReleaseDialog request={request({ refusal: { kind: "library_or_rule_pack", reason: "REPORT-USER-RELEASE-LIBRARY-REFERENCED: invented" } })} />);
    expect(screen.queryByTestId("user-release-statement")).not.toBeInTheDocument();
    expect(screen.getByTestId("user-release-refusal")).toHaveTextContent(USER_RELEASE_LIBRARY_REFUSAL);
    expect(screen.getByTestId("user-release-refusal-reason")).toHaveTextContent("REPORT-USER-RELEASE-LIBRARY-REFERENCED: invented");
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
    expect(document.activeElement).toBe(screen.getByTestId("user-release-cancel"));
  });

  it("shows the result refusal in place of the statement", () => {
    render(<UserReleaseDialog request={request({ refusal: { kind: "result", reason: "REPORT-PACKAGE-ROUTE-NOT-ADMITTED: invented" } })} />);
    expect(screen.queryByTestId("user-release-statement")).not.toBeInTheDocument();
    expect(screen.getByTestId("user-release-refusal")).toHaveTextContent(USER_RELEASE_RESULT_REFUSAL);
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
  });
});
