// DEL-07-05 numerical health §3: under the verdict-only hold the Solver chip's title is
// the health headline and its popover shows the headline and the output sentence, not
// the internal `needs_recompute` token; without the hold the chip is unchanged.
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { WorkspaceSessionProvider } from "../WorkspaceSessionContext";
import { ShellStatusBar } from "./ShellStatusBar";
import type { WorkspaceSession } from "../workspaceSession";

const hold = {
  headline: "Numerical health: 1 sensitive",
  shortHeadline: "Numerical health: sensitive",
  outputSentence: "No results of this run can be exported or used by rule checks while 1 load case(s) are not checks passed."
};
function session(numericalHealthHold: typeof hold | null): WorkspaceSession {
  return {
    model: { model: { analysis_status: { mechanics: "MECHANICS_SOLVED" } } },
    results: { result: { status: { mechanics: "MECHANICS_SOLVED", professional_acceptance: "NOT_PROVIDED" } }, currentSolvedResult: null, numericalHealthHold, historicalRun: null, solveJob: { state: "completed" }, ruleCheckAggregate: null },
    selection: { selection: null, orderedSelection: { orderedKeys: [] } },
    chrome: { activeSection: "results", stageSurface: "results", auditDrawerOpen: false, setAuditDrawerOpen() {}, issuesDrawerOpen: false, setIssuesDrawerOpen() {} }
  } as unknown as WorkspaceSession;
}

describe("held Solver chip", () => {
  it("shows the headline as its title and the headline and output sentence in its popover", () => {
    render(<WorkspaceSessionProvider session={session(hold)}><ShellStatusBar issueCount={0} /></WorkspaceSessionProvider>);
    const chip = screen.getByTestId("status-pill-mechanics");
    expect(chip).toHaveTextContent("Solver · Numerical health: 1 sensitive");
    expect(chip).toHaveAttribute("title", "Numerical health: 1 sensitive");
    fireEvent.click(chip);
    const popover = screen.getByTestId("status-pill-mechanics-popover");
    expect(screen.getByTestId("status-chip-numerical-health-headline")).toHaveTextContent("Numerical health: 1 sensitive");
    expect(screen.getByTestId("status-chip-numerical-health-output-sentence")).toHaveTextContent(hold.outputSentence);
    expect(popover.textContent).not.toContain("needs_recompute");
    expect(popover.querySelector("code")).toBeNull();
  });

  it("keeps the Needs recompute chip and its token without the hold", () => {
    render(<WorkspaceSessionProvider session={session(null)}><ShellStatusBar issueCount={0} /></WorkspaceSessionProvider>);
    const chip = screen.getByTestId("status-pill-mechanics");
    expect(chip).toHaveTextContent("Solver · Needs recompute");
    expect(chip).toHaveAttribute("title", "Derived numerical qualification: needs_recompute");
    fireEvent.click(chip);
    expect(screen.getByTestId("status-pill-mechanics-popover").querySelector("code")?.textContent).toBe("needs_recompute");
  });
});
