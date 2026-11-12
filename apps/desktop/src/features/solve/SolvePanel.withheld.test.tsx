// The Solve command withheld (DEL-07-07 REQ-005, REQ-008, AC-004, AC-005;
// DEL-07-04 REQ-004, AC-001; DEL-07-06 REQ-002, REQ-003, REQ-011), read from the
// accessibility tree: the control's state and its reason, and each standing
// finding with the fields its producer supplied. Invented findings.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { Diagnostic, PreviewModel } from "../../types";
import type { ModelCheckFinding } from "../diagnostics/modelChecks";
import type { SolveGateReading } from "../workspace/solveGates";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { SolvePanel } from "./SolvePanel";
import { SOLVE_WITHHELD_LISTED } from "./SolveWithheldNotice";

function finding(model: PreviewModel, diagnostic: Diagnostic, index = 0): ModelCheckFinding {
  const ref = diagnostic.affected_refs?.[0];
  const node = model.nodes.find((item) => item.id === ref);
  return { key: `solve:${index}:${diagnostic.code}`, group: "solve", diagnostic, entity: node ? { type: "node", id: node.id } : null };
}

async function renderPanel(reading: SolveGateReading | null) {
  const model = await loadPreviewModel();
  const onRun = vi.fn();
  const onSelectEntity = vi.fn();
  const onOpenModelChecks = vi.fn();
  render(
    <SolvePanel model={model} result={null} analysisRun={null} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive"
      onRun={onRun} onCancel={() => {}} onSolverModeChange={() => {}} solveGate={reading} onSelectEntity={onSelectEntity} onOpenModelChecks={onOpenModelChecks} />
  );
  return { model, onRun, onSelectEntity, onOpenModelChecks };
}

it("offers Solve with no withheld reason while the gate is open or unknown", async () => {
  await renderPanel({ kind: "open" });
  const solve = screen.getByRole("button", { name: "Run mechanics preview" });
  expect(solve).not.toHaveAttribute("aria-disabled");
  expect(solve).not.toHaveAccessibleDescription();
  expect(screen.queryByRole("region", { name: "Solve is withheld" })).toBeNull();
});

it("withholds Solve, states why in the accessibility tree, and names each finding with every supplied field", async () => {
  const model = await loadPreviewModel();
  const nodeId = model.nodes[0].id;
  const full: Diagnostic = {
    id: "diagnostic:invented:full", code: "INVENTED_INPUT_MISSING", severity: "blocking", diagnostic_class: "solve_blocking",
    source: "invented source", message: "invented missing value", remediation: "invented remediation",
    provenance: "invented provenance", affected_refs: [nodeId, "nodes"]
  };
  const bare: Diagnostic = { code: "INVENTED_BARE", severity: "error", message: "invented bare finding" };
  const findings = [finding(model, full), finding(model, bare, 1)];
  const reading: SolveGateReading = { kind: "withheld", model, findings, diagnostics: findings.map((item) => item.diagnostic), notListedBlocking: 0, refusal: null };
  const { onRun, onSelectEntity } = await renderPanel(reading);

  const solve = screen.getByRole("button", { name: "Run mechanics preview (withheld)" });
  expect(solve).toHaveAttribute("aria-disabled", "true");
  expect(solve).toHaveAccessibleDescription(/2 solve-blocking findings stand for this model/);
  fireEvent.click(solve);
  expect(onRun).toHaveBeenCalledOnce(); // The session's gate refuses it and opens Model checks.

  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  const list = within(notice).getByRole("list", { name: "Findings that withhold Solve" });
  const fullButton = within(list).getByRole("button", { name: /INVENTED_INPUT_MISSING blocking/ });
  expect(fullButton).toHaveTextContent("invented missing value");
  fireEvent.click(fullButton);
  expect(onSelectEntity).toHaveBeenCalledWith({ type: "node", id: nodeId });
  const fullFields = within(notice).getByTestId("solve-withheld-fields-INVENTED_INPUT_MISSING");
  for (const text of ["solve_blocking", "invented source", "invented remediation", "invented provenance", `${nodeId} (the finding above selects it)`, "nodes (names no model entity to select)"]) {
    expect(fullFields).toHaveTextContent(text);
  }
  // A field the producer did not supply reads as not supplied; nothing is composed.
  const bareFields = within(notice).getByTestId("solve-withheld-fields-INVENTED_BARE");
  expect(within(bareFields).getAllByText("not supplied by producer")).toHaveLength(5);
  const bareButton = within(list).getByRole("button", { name: /INVENTED_BARE error/ });
  expect(bareButton).toHaveAttribute("aria-disabled", "true");
  expect(bareButton).toHaveAccessibleDescription("This finding names no model entity to select.");
  expect(notice.textContent).not.toMatch(/certif|approv|complian|seal/i);
});

it("bounds the list and points to Model checks for the rest, counting unlisted findings", async () => {
  const model = await loadPreviewModel();
  const findings = Array.from({ length: SOLVE_WITHHELD_LISTED + 2 }, (_, index) =>
    finding(model, { code: `INVENTED_${index}`, severity: "blocking", message: `invented ${index}` }, index));
  const { onOpenModelChecks } = await renderPanel({ kind: "withheld", model, findings, diagnostics: findings.map((item) => item.diagnostic), notListedBlocking: 3, refusal: null });
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  expect(within(notice).getAllByRole("button", { name: /^INVENTED_\d+ blocking/ })).toHaveLength(SOLVE_WITHHELD_LISTED);
  expect(within(notice).getByTestId("solve-withheld-summary")).toHaveTextContent(`${SOLVE_WITHHELD_LISTED + 5} solve-blocking findings stand`);
  expect(within(notice).getByTestId("solve-withheld-rest")).toHaveTextContent("5 more solve-blocking findings are not listed here.");
  fireEvent.click(within(notice).getByRole("button", { name: "Show Model checks" }));
  expect(onOpenModelChecks).toHaveBeenCalledOnce();
});

it("states the solve's own refusal when it cannot read the model", async () => {
  const model = await loadPreviewModel();
  await renderPanel({ kind: "withheld", model, findings: [], diagnostics: [], notListedBlocking: 0, refusal: "invented refusal text" });
  expect(screen.getByRole("button", { name: "Run mechanics preview (withheld)" })).toHaveAccessibleDescription(/The solve cannot read this model: invented refusal text/);
});
