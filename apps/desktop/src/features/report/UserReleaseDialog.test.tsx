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
  USER_RELEASE_FRAME_STATEMENTS_LEAD,
  USER_RELEASE_FRAME_STATEMENTS_REFUSAL,
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
/** A witness's frame statements as the native host gives them, committed and held equal to the
 * native operation by `report_standing_package_tests.rs`. */
function committedFrameStatements(name: string): string[] {
  return JSON.parse(readFileSync(path.join(root, "fixtures/reports/invented/report_standing_witnesses", `${name}.frame_statements.json`), "utf8")).frame_statements;
}

function request(overrides: Partial<UserReleaseDialogRequest> = {}): UserReleaseDialogRequest {
  return {
    projectName: "Invented project",
    modelDocumentSha256: "0123456789abcdef".repeat(4),
    runId: "run:invented-001",
    statesStanding: true,
    frameStatements: [],
    refusal: null,
    opener: null,
    cancel: vi.fn(),
    save: vi.fn(),
    ...overrides
  };
}

describe("UserReleaseDialog", () => {
  /** DEL-08-01 REQ-024, "Package and user release": the report frame's statements on the
   * result's standing, as the native host gives them (the committed files of each witness),
   * each on its own line, word for word, before the statement. */
  it("states each of the frame's statements, word for word, before the statement, each on its own line", () => {
    const gap = committedFrameStatements("pressure_3_gap_unresolved");
    expect(gap).toHaveLength(2);
    expect(gap[0].startsWith("Support states: not confirmed.")).toBe(true);
    expect(gap[1].startsWith("Kept for inspection.")).toBe(true);
    render(<UserReleaseDialog request={request({ frameStatements: gap })} />);
    const block = screen.getByTestId("user-release-frame-statements");
    expect(block.firstElementChild).toHaveTextContent(USER_RELEASE_FRAME_STATEMENTS_LEAD);
    expect(screen.getAllByTestId("user-release-frame-statement").map((line) => line.textContent)).toEqual(gap);
    // Before the statement control in the dialog's order, and the save waits for the statement.
    const statement = screen.getByTestId("user-release-statement");
    expect(block.compareDocumentPosition(statement) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
    fireEvent.click(statement);
    expect(screen.getByTestId("user-release-confirm")).toBeEnabled();
  });

  it("states only the support-states line for a result with nonlinear supports and no recovery-basis mark", () => {
    const friction = committedFrameStatements("pressure_3_coupled_friction_sensitive");
    expect(friction).toHaveLength(1);
    expect(friction[0].startsWith("Support states: not confirmed.")).toBe(true);
    render(<UserReleaseDialog request={request({ frameStatements: friction })} />);
    expect(screen.getAllByTestId("user-release-frame-statement").map((line) => line.textContent)).toEqual(friction);
    expect(screen.getByTestId("user-release-dialog")).not.toHaveTextContent("Kept for inspection.");
  });

  it("states no frame statement where the report's frame states none", () => {
    render(<UserReleaseDialog request={request({ frameStatements: [] })} />);
    expect(screen.queryByTestId("user-release-frame-statements")).toBeNull();
    expect(screen.getByTestId("user-release-statement")).toBeInTheDocument();
  });

  it("refuses, offers no statement and saves nothing where the frame statements could not be obtained", () => {
    const save = vi.fn();
    render(<UserReleaseDialog request={request({ save, refusal: { kind: "frame_statements", reason: "REPORT-FRAME-STATEMENTS-UNAVAILABLE: invented" } })} />);
    expect(screen.getByTestId("user-release-refusal")).toHaveTextContent(USER_RELEASE_FRAME_STATEMENTS_REFUSAL);
    expect(screen.getByTestId("user-release-refusal-reason")).toHaveTextContent("REPORT-FRAME-STATEMENTS-UNAVAILABLE: invented");
    expect(screen.queryByTestId("user-release-statement")).toBeNull();
    expect(screen.queryByTestId("user-release-frame-statements")).toBeNull();
    fireEvent.click(screen.getByTestId("user-release-confirm"));
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
    expect(save).not.toHaveBeenCalled();
  });

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
      "It needs a result solved in this session: a pressure-free Euler–Bernoulli (preview-physics-2) result, or a pressure-free Timoshenko (preview-physics-timoshenko-1) or exact pressure route (pressure-3 or pressure-source-1) result whose mechanics are solved. The report states that result's numerical standing, whatever it is."
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
