import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import type { ModelCheckRun } from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";

// DEL-04-06 Design `on-demand-model-checks.md`: the panel lists a run's
// findings by check with their own code and text; choosing one selects the
// entity it names. Invented findings.
const model = { diagnostics: [] } as unknown as PreviewModel;
const run: ModelCheckRun = {
  model,
  solve: { kind: "checked" },
  notShown: { solve: 0, slots: 0, library: 0 },
  notShownBlocking: 0,
  findings: [
    { key: "solve:0:a", group: "solve", entity: { type: "node", id: "node:a" },
      diagnostic: { id: "diagnostic:topology:disconnected", code: "MODEL_TOPOLOGY_DISCONNECTED", severity: "warning", message: "invented two-part text", affected_refs: ["node:a", "node:c"] } },
    { key: "solve:1:b", group: "solve", entity: null,
      diagnostic: { id: "diagnostic:physics:no-restraints", code: "SUPPORT_INPUT_MISSING", severity: "blocking", message: "invented restraint text", affected_refs: ["supports"] } },
    { key: "slots:0:c", group: "slots", entity: { type: "pipe", id: "pipe:b" },
      diagnostic: { id: "model-document:SLOT:pipe", code: "INVENTED_SLOT_CODE", severity: "warning", message: "invented slot text", affected_refs: ["pipe:b"] } }
  ]
};

describe("ModelChecksPanel", () => {
  it("offers the run before any run, and says the checks never change the model", () => {
    const onRun = vi.fn();
    render(<ModelChecksPanel model={model} run={null} onRun={onRun} onSelectEntity={() => {}} />);
    expect(screen.getByTestId("model-checks-summary")).toHaveTextContent("The checks have not been run on this model. They read the model and never change it.");
    const button = screen.getByRole("button", { name: "Run model checks" });
    expect(button).toHaveAttribute("aria-keyshortcuts", "Shift+Meta+M Shift+Control+M");
    fireEvent.click(button);
    expect(onRun).toHaveBeenCalledTimes(1);
    expect(screen.queryByTestId("model-checks-findings")).toBeNull();
  });

  it("lists each finding by its check with its code and text, and selects the entity it names", () => {
    const onSelectEntity = vi.fn();
    render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={onSelectEntity} />);
    expect(screen.getByTestId("model-checks-summary")).toHaveTextContent("3 findings: 1 blocking or error, 2 warnings, 0 info.");
    const list = screen.getByRole("list", { name: "Model check findings" });
    const groups = within(list).getAllByRole("listitem").filter((item) => item.hasAttribute("data-group"));
    expect(groups.map((item) => item.getAttribute("data-group"))).toEqual(["solve", "slots", "library"]);
    expect(groups[0]).toHaveTextContent("Solve input, restraint and topology checks 2 findings");
    expect(groups[2]).toHaveTextContent("Library references 0 findings");
    const topology = screen.getByTestId("model-check-MODEL_TOPOLOGY_DISCONNECTED");
    expect(topology).toHaveTextContent("MODEL_TOPOLOGY_DISCONNECTED warning");
    expect(topology).toHaveTextContent("invented two-part text");
    fireEvent.click(topology);
    expect(onSelectEntity).toHaveBeenCalledWith({ type: "node", id: "node:a" });
    // A finding that names no entity is marked and selects nothing.
    const restraint = screen.getByTestId("model-check-SUPPORT_INPUT_MISSING");
    expect(restraint).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(restraint);
    expect(onSelectEntity).toHaveBeenCalledTimes(1);
  });

  it("moves keyboard focus through the findings with the arrow, Home and End keys", () => {
    render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    const list = screen.getByRole("list", { name: "Model check findings" });
    const [first, second, third] = ["MODEL_TOPOLOGY_DISCONNECTED", "SUPPORT_INPUT_MISSING", "INVENTED_SLOT_CODE"].map((code) => screen.getByTestId(`model-check-${code}`));
    first.focus();
    fireEvent.keyDown(list, { key: "ArrowDown" });
    expect(document.activeElement).toBe(second);
    fireEvent.keyDown(list, { key: "ArrowDown" });
    expect(document.activeElement).toBe(third);
    fireEvent.keyDown(list, { key: "ArrowDown" });
    expect(document.activeElement).toBe(third);
    fireEvent.keyDown(list, { key: "Home" });
    expect(document.activeElement).toBe(first);
    fireEvent.keyDown(list, { key: "End" });
    expect(document.activeElement).toBe(third);
    fireEvent.keyDown(list, { key: "ArrowUp" });
    expect(document.activeElement).toBe(second);
  });

  it("moves focus to a new run's outcome once", () => {
    const view = render(<ModelChecksPanel model={model} run={null} onRun={() => {}} onSelectEntity={() => {}} />);
    const fresh = { ...run };
    view.rerender(<ModelChecksPanel model={model} run={fresh} onRun={() => {}} onSelectEntity={() => {}} />);
    const summary = screen.getByTestId("model-checks-summary");
    expect(summary).toHaveAttribute("role", "status");
    expect(document.activeElement).toBe(summary);
    // Showing the same run again (the drawer reopened) leaves focus where it is.
    screen.getByTestId("model-checks-run").focus();
    view.unmount();
    render(<ModelChecksPanel model={model} run={fresh} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(document.activeElement).not.toBe(screen.getByTestId("model-checks-summary"));
    expect(screen.queryByTestId("model-checks-stale")).toBeNull();
  });

  it("marks a run of an earlier model as out of date and states when the solve's checks did not run", () => {
    const edited = { diagnostics: [] } as unknown as PreviewModel;
    render(<ModelChecksPanel model={edited} run={{ ...run, solve: { kind: "refused", refusal: "INVENTED_REFUSAL: text" }, findings: run.findings.filter((item) => item.group !== "solve") }} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(screen.getByTestId("model-checks-stale")).toHaveTextContent("The model has changed since these checks ran; run them again to check the current model.");
    expect(screen.getByTestId("model-checks-solve-unchecked")).toHaveTextContent("The solve's checks did not run: the solve cannot read this model (INVENTED_REFUSAL: text).");
    expect(screen.getByRole("list", { name: "Model check findings" })).toHaveTextContent("Solve input, restraint and topology checks not run");
  });

  it("states how many findings the bounded report left out, marks shortened text, and shows a run in progress", () => {
    const bounded: ModelCheckRun = { ...run, notShown: { solve: 9, slots: 3, library: 0 }, notShownBlocking: 4, findings: [{ ...run.findings[0], diagnostic: { ...run.findings[0].diagnostic, truncated: true } as typeof run.findings[0]["diagnostic"] }] };
    const onRun = vi.fn();
    const view = render(<ModelChecksPanel model={model} run={bounded} onRun={onRun} onSelectEntity={() => {}} />);
    expect(screen.getByTestId("model-checks-not-shown")).toHaveTextContent("12 more findings are not listed: the report is bounded.");
    expect(screen.getByTestId("model-checks-summary")).toHaveTextContent("1 finding: 0 blocking or error, 1 warning, 0 info. 12 more not listed; 4 of them blocking or error.");
    expect(screen.getByTestId("model-check-truncated")).toHaveTextContent("Text shortened in the bounded report.");
    view.rerender(<ModelChecksPanel model={model} run={bounded} busy onRun={onRun} onSelectEntity={() => {}} />);
    const button = screen.getByTestId("model-checks-run");
    expect(button).toHaveTextContent("Running model checks");
    expect(button).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(button);
    expect(onRun).not.toHaveBeenCalled();
  });

  // Codex review r3 of #1665 (its probe: a support id of 1.1 million
  // characters gives one blocking finding the report counts but cannot list):
  // a run whose findings were all left out is not an empty result.
  it("does not call a run empty when every finding was counted rather than listed", () => {
    const omitted: ModelCheckRun = { ...run, findings: [], notShown: { solve: 1, slots: 0, library: 0 }, notShownBlocking: 1 };
    render(<ModelChecksPanel model={model} run={omitted} onRun={() => {}} onSelectEntity={() => {}} />);
    const summary = screen.getByTestId("model-checks-summary");
    expect(summary).toHaveTextContent("1 finding, none listed; 1 of them blocking or error.");
    expect(summary).not.toHaveTextContent("nothing to report");
  });
});
