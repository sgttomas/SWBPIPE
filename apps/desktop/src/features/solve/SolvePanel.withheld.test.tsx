// The Solve command withheld (DEL-07-07 REQ-005, REQ-008, AC-004, AC-005;
// DEL-07-04 REQ-004, AC-001; DEL-07-06 REQ-002, REQ-003, REQ-011), read from the
// accessibility tree: the control's state and its reason, and each standing
// finding with the fields its producer supplied. Invented findings.
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { Diagnostic, PreviewModel } from "../../types";
import { resolveModelCheckEntity, runModelChecks, type ModelCheckFinding, type ModelCheckRun } from "../diagnostics/modelChecks";
import { ModelChecksPanel } from "../diagnostics/ModelChecksPanel";
import { readSolveGate, type SolveGateReading } from "../workspace/solveGates";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { SolvePanel } from "./SolvePanel";
import { SOLVE_WITHHELD_LISTED } from "./SolveWithheldNotice";

function finding(model: PreviewModel, diagnostic: Diagnostic, index = 0): ModelCheckFinding {
  const ref = diagnostic.affected_refs?.[0];
  return { key: `solve:${index}:${diagnostic.code}`, group: "solve", diagnostic, entity: ref ? resolveModelCheckEntity(model, ref) : null };
}

async function renderPanel(reading: SolveGateReading | null, panelModel?: PreviewModel) {
  const model = panelModel ?? await loadPreviewModel();
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
  const reading: SolveGateReading = { kind: "withheld", model, findings, diagnostics: findings.map((item) => item.diagnostic), notListedBlocking: 0, refusal: null, laterCheckNotCompleted: null };
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
  const { onOpenModelChecks } = await renderPanel({ kind: "withheld", model, findings, diagnostics: findings.map((item) => item.diagnostic), notListedBlocking: 3, refusal: null, laterCheckNotCompleted: null });
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  expect(within(notice).getAllByRole("button", { name: /^INVENTED_\d+ blocking/ })).toHaveLength(SOLVE_WITHHELD_LISTED);
  expect(within(notice).getByTestId("solve-withheld-summary")).toHaveTextContent(`${SOLVE_WITHHELD_LISTED + 5} solve-blocking findings stand`);
  const rest = within(notice).getByTestId("solve-withheld-rest");
  expect(rest).toHaveTextContent("2 more solve-blocking findings are listed in Model checks.");
  expect(rest).toHaveTextContent("3 more solve-blocking findings were counted by the bounded report but not listed.");
  fireEvent.click(within(notice).getByRole("button", { name: "Show Model checks" }));
  expect(onOpenModelChecks).toHaveBeenCalledOnce();
});

it("states the solve's own refusal when it cannot read the model", async () => {
  const model = await loadPreviewModel();
  await renderPanel({ kind: "withheld", model, findings: [], diagnostics: [], notListedBlocking: 0, refusal: "invented refusal text", laterCheckNotCompleted: null });
  expect(screen.getByRole("button", { name: "Run mechanics preview (withheld)" })).toHaveAccessibleDescription(/The solve cannot read this model: invented refusal text/);
});

it("leads a real material finding to its material, in the Model checks list and in the notice, by keyboard focus and activation", async () => {
  const model = JSON.parse(JSON.stringify(await loadPreviewModel())) as PreviewModel;
  const material = model.materials![0];
  delete (material as { shear_modulus?: unknown }).shear_modulus;
  const run = await runModelChecks(model);
  const standing = run.findings.find((item) => item.diagnostic.code === "MATERIAL_INPUT_MISSING")!;
  expect(standing.diagnostic.affected_refs).toEqual([material.id]);
  expect(standing.entity).toEqual({ type: "material", id: material.id });

  const listSelect = vi.fn();
  const list = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={listSelect} />);
  const inList = list.getByTestId("model-check-MATERIAL_INPUT_MISSING");
  expect(inList).toHaveAttribute("aria-disabled", "false");
  act(() => inList.focus());
  expect(document.activeElement).toBe(inList);
  fireEvent.click(inList); // A focused native button is activated by Enter and Space as by a click.
  expect(listSelect).toHaveBeenCalledWith({ type: "material", id: material.id });
  list.unmount();

  const reading = readSolveGate(model, run);
  expect(reading.kind).toBe("withheld");
  const onSelectEntity = vi.fn();
  render(<SolvePanel model={model} result={null} analysisRun={null} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive"
    onRun={() => {}} onCancel={() => {}} onSolverModeChange={() => {}} solveGate={reading} onSelectEntity={onSelectEntity} />);
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  const button = within(notice).getByRole("button", { name: /MATERIAL_INPUT_MISSING blocking/ });
  expect(button.tagName).toBe("BUTTON");
  expect(button.tabIndex).toBe(0);
  act(() => button.focus());
  expect(document.activeElement).toBe(button);
  fireEvent.click(button);
  expect(onSelectEntity).toHaveBeenCalledWith({ type: "material", id: material.id });
  expect(notice).not.toHaveTextContent("names no model entity");
});

it("offers each entity a finding names: the first by the finding, the second by its own control", async () => {
  const model = await loadPreviewModel();
  const [first, second] = [model.nodes[0].id, model.supports[0].id];
  const diagnostic: Diagnostic = { code: "INVENTED_TWO_ENTITIES", severity: "blocking", message: "invented finding naming two entities", affected_refs: [first, second] };
  const findings = [finding(model, diagnostic)];
  const { onSelectEntity } = await renderPanel({ kind: "withheld", model, findings, diagnostics: [diagnostic], notListedBlocking: 0, refusal: null, laterCheckNotCompleted: null });
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  fireEvent.click(within(notice).getByRole("button", { name: /INVENTED_TWO_ENTITIES blocking/ }));
  expect(onSelectEntity).toHaveBeenLastCalledWith({ type: "node", id: first });
  const objects = within(notice).getByRole("list", { name: "Objects INVENTED_TWO_ENTITIES names" });
  expect(objects).toHaveTextContent(`${first} (the finding above selects it)`);
  const secondControl = within(objects).getByRole("button", { name: `Select ${second}` });
  act(() => secondControl.focus());
  expect(document.activeElement).toBe(secondControl);
  fireEvent.click(secondControl);
  expect(onSelectEntity).toHaveBeenLastCalledWith({ type: "support", id: second });
});

it("states plainly a refusal with no findings: the solve's own text, and no empty list", async () => {
  const model = await loadPreviewModel();
  const run: ModelCheckRun = { model, solve: { kind: "refused", refusal: "request: missing field `outside_diameter`" }, findings: [], notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 } };
  const reading = readSolveGate(model, run);
  expect(reading).toMatchObject({ kind: "withheld", findings: [], diagnostics: [], notListedBlocking: 0, refusal: "request: missing field `outside_diameter`" });
  await renderPanel(reading);
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  expect(within(notice).getByTestId("solve-withheld-summary")).toHaveTextContent("The solve cannot read this model: request: missing field `outside_diameter`.");
  expect(within(notice).queryByRole("list")).toBeNull();
  expect(within(notice).queryByTestId("solve-withheld-rest")).toBeNull();
  expect(screen.getByRole("button", { name: "Run mechanics preview (withheld)" })).toHaveAccessibleDescription(/The solve cannot read this model: request: missing field `outside_diameter`/);
});

it("leads a finding that names a section to that section record, in the Model checks list and in the notice", async () => {
  const model = JSON.parse(JSON.stringify(await loadPreviewModel())) as PreviewModel;
  const section = { id: "section:invented-gate", name: "Invented section", section_type: "pipe", properties: {}, provenance: "invented" };
  model.sections = [...(model.sections ?? []), section as never];
  const diagnostic: Diagnostic = { code: "INVENTED_SECTION_INPUT_MISSING", severity: "blocking", message: "invented section finding", affected_refs: [section.id] };
  const standing = finding(model, diagnostic);
  expect(standing.entity).toEqual({ type: "section", id: section.id });

  const listSelect = vi.fn();
  const run: ModelCheckRun = { model, solve: { kind: "checked" }, findings: [standing], notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 } };
  const list = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={listSelect} />);
  const inList = list.getByTestId("model-check-INVENTED_SECTION_INPUT_MISSING");
  expect(inList).toHaveAttribute("aria-disabled", "false");
  act(() => inList.focus());
  expect(document.activeElement).toBe(inList);
  fireEvent.click(inList);
  expect(listSelect).toHaveBeenCalledWith({ type: "section", id: section.id });
  list.unmount();

  const { onSelectEntity } = await renderPanel(readSolveGate(model, run), model);
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  fireEvent.click(within(notice).getByRole("button", { name: /INVENTED_SECTION_INPUT_MISSING blocking/ }));
  expect(onSelectEntity).toHaveBeenCalledWith({ type: "section", id: section.id });
  expect(notice).not.toHaveTextContent("names no model entity");
});

it("names a kept refusal with no findings as the refusal, above it, when a later check could not finish", async () => {
  const model = await loadPreviewModel();
  await renderPanel({ kind: "withheld", model, findings: [], diagnostics: [], notListedBlocking: 0, refusal: "request: missing field `outside_diameter`", laterCheckNotCompleted: "invented admission refusal" }, model);
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  const sentence = within(notice).getByTestId("solve-withheld-later-not-completed");
  expect(sentence).toHaveTextContent("A later run of the checks on this model could not finish (invented admission refusal). The refusal below is from the last run that finished, and it still applies.");
  expect(sentence).not.toHaveTextContent("findings");
  const summary = within(notice).getByTestId("solve-withheld-summary");
  expect(summary).toHaveTextContent("The solve cannot read this model: request: missing field `outside_diameter`.");
  expect(sentence.compareDocumentPosition(summary) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
});
