import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { IssuesList } from "./IssuesList";
import { buildIssueList, countIssues } from "./issueRows";
import { blankSession, demoSession } from "./issueRows.testSessions";

// The heading's words belong to another surface (features/results/integrityStatus.ts); the list
// shows whatever words it is given.
const HEADING = "invented heading above the solver's own messages";

function rows() {
  return screen.getAllByTestId("issue-row");
}

describe("IssuesList", () => {
  it("draws the blank model's three findings first, each with its act, under one count line", () => {
    const onAct = vi.fn();
    const onSelectEntity = vi.fn();
    render(<IssuesList list={buildIssueList(blankSession())} onAct={onAct} onSelectEntity={onSelectEntity} solverMessagesHeading={HEADING} />);
    expect(screen.getByTestId("issues-count-line")).toHaveTextContent(/^3 to fix before Run$/);
    expect(rows().map((row) => row.getAttribute("data-code"))).toEqual([
      "NODE_INPUT_MISSING", "PIPE_INPUT_MISSING", "LOAD_INPUT_MISSING",
      "BLANK_PROJECT_AUTHORING_TARGET", "SUPPORT_STIFFNESS_UNRESOLVED", "RULE_CHECK_NOT_PERFORMED"
    ]);
    const first = rows()[0]!;
    // The sentence first, then where it stands, the act, and the code last.
    const parts = Array.from(first.querySelectorAll(".issue-sentence, [data-testid]")).map((item) => item.textContent);
    expect(parts).toEqual(["linear-static preview requires at least two explicit nodes", "Fix before Run", "Add a node", "NODE_INPUT_MISSING"]);
    // These name no entity: nothing to select.
    expect(within(first).queryByTestId("issue-entity")).toBeNull();
    expect(rows().slice(0, 3).map((row) => within(row).getByTestId("issue-act").textContent)).toEqual(["Add a node", "Add a pipe", "Add a load"]);
    fireEvent.click(within(rows()[1]!).getByRole("button", { name: "Add a pipe" }));
    expect(onAct).toHaveBeenCalledWith("insert.pipe");
    // No row is one of the solver's own messages, so no heading.
    expect(screen.queryByTestId("issues-solver-messages-heading")).toBeNull();
    // No tiles, filter, second list or detail pane.
    expect(screen.queryByRole("searchbox")).toBeNull();
    expect(screen.getAllByRole("list")).toHaveLength(1);
  });

  it("draws a row with no entity and no act with neither control, and says so when opened", () => {
    render(<IssuesList list={buildIssueList(blankSession())} onAct={() => {}} onSelectEntity={() => {}} />);
    const note = rows().find((row) => row.getAttribute("data-code") === "RULE_CHECK_NOT_PERFORMED")!;
    expect(within(note).queryByTestId("issue-entity")).toBeNull();
    expect(within(note).queryByTestId("issue-act")).toBeNull();
    expect(within(note).getByTestId("issue-tier")).toHaveTextContent("Note");
    fireEvent.click(within(note).getByRole("button", { name: /No code or owner rule pack/ }));
    const detail = within(note).getByTestId("issue-detail");
    expect(detail).toHaveTextContent("Model objectsnone named");
    expect(detail).toHaveTextContent("Remediationnot supplied by producer");
    expect(detail).toHaveTextContent("Provenancenot supplied by producer");
  });

  it("offers each entity a row names", () => {
    const onSelectEntity = vi.fn();
    render(<IssuesList list={buildIssueList(demoSession())} onAct={() => {}} onSelectEntity={onSelectEntity} />);
    const row = rows().find((item) => item.getAttribute("data-code") === "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED")!;
    expect(within(row).getAllByTestId("issue-entity").map((item) => item.textContent)).toEqual(["support:CE-120", "node:N-120"]);
    fireEvent.click(within(row).getByRole("button", { name: "Select node:N-120" }));
    expect(onSelectEntity).toHaveBeenCalledWith({ type: "node", id: "node:N-120" });
  });

  it("draws SH-140's hanger warnings as one row that selects the support and opens in place", () => {
    const onSelectEntity = vi.fn();
    const onOpenRow = vi.fn();
    const list = buildIssueList(demoSession());
    render(
      <IssuesList
        list={list}
        onAct={() => {}}
        onOpenRow={onOpenRow}
        onSelectEntity={onSelectEntity}
        renderDetail={(row) => <div data-testid="caller-detail">{row.code}</div>}
        solverMessagesHeading={HEADING}
      />
    );
    const count = countIssues(list);
    expect(screen.getByTestId("issues-count-line")).toHaveTextContent(`Nothing to fix. ${count.warnings} warnings, ${count.notes} notes.`);
    expect(rows()).toHaveLength(count.total);
    // Rows include the solver's own messages: the heading stands once, at the head of the list.
    expect(screen.getAllByTestId("issues-solver-messages-heading").map((item) => item.textContent)).toEqual([HEADING]);
    const hanger = rows().filter((row) => row.getAttribute("data-code") === "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD");
    expect(hanger).toHaveLength(1);
    const row = hanger[0]!;
    fireEvent.click(within(row).getByRole("button", { name: "Select support:SH-140" }));
    expect(onSelectEntity).toHaveBeenCalledWith({ type: "support", id: "support:SH-140" });
    expect(within(row).queryByTestId("issue-act")).toBeNull();
    const sentence = within(row).getByRole("button", { name: /its installed load \(hanger\.installed_load\), cold load/ });
    expect(sentence).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(sentence);
    expect(sentence).toHaveAttribute("aria-expanded", "true");
    expect(onOpenRow).toHaveBeenCalledWith(expect.objectContaining({ code: "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD" }));
    const detail = within(row).getByTestId("issue-detail");
    expect(sentence.getAttribute("aria-controls")).toBe(detail.id);
    expect(detail).toHaveTextContent("Members");
    expect(detail).toHaveTextContent("hanger.installed_load, hanger.cold_load, hanger.hot_load");
    expect(detail).toHaveTextContent("Severity");
    expect(detail).toHaveTextContent("diagnostic:spring-hanger:support-SH-140:hanger.cold_load");
    // Each load's own sentence, and the result's presented copy, stay readable.
    expect(detail).toHaveTextContent("its hot load (hanger.hot_load) is not applied as preload");
    expect(detail).toHaveTextContent("Supplied diagnostic text, unverified");
    expect(within(detail).getByTestId("caller-detail")).toHaveTextContent("SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD");
    fireEvent.click(sentence);
    expect(within(row).queryByTestId("issue-detail")).toBeNull();
  });

  it("moves between rows with the arrow keys", () => {
    render(<IssuesList list={buildIssueList(blankSession())} onAct={() => {}} onSelectEntity={() => {}} />);
    const sentences = rows().map((row) => row.querySelector<HTMLButtonElement>(".issue-sentence")!);
    sentences[0]!.focus();
    fireEvent.keyDown(sentences[0]!, { key: "ArrowDown" });
    expect(document.activeElement).toBe(sentences[1]);
    fireEvent.keyDown(sentences[1]!, { key: "End" });
    expect(document.activeElement).toBe(sentences[sentences.length - 1]);
    fireEvent.keyDown(sentences[sentences.length - 1]!, { key: "Home" });
    expect(document.activeElement).toBe(sentences[0]);
  });
});
