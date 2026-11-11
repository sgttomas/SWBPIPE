// RT-A4 on the desktop (DEL-07-03):
// the combination editor's unsigned bases, typed nested operands and three-axis static seismic
// template, through the real wasm operation engine. Each form queues an intent and the test
// applies it with `applyModelOperation`, the route Review/Apply uses. Models are built in test
// code from the committed invented demo document (`fixtures/product_preview/invented_demo_model.json`,
// model document 0.9.0, which carries nonlinear supports and a constant-effort support).
// No browser-reachable solve exists (the solve is a native job), so the solve's κ and
// nonlinear gates are stated by the editor and judged by the engine's Rust tests, not here.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import type { EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";
import { applyModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { modelDocumentSlotFindings } from "../model-file/modelDocumentShape";
import { chooseVirtualMultiTarget, chooseVirtualTarget } from "../../test-support/workspaceTestControls";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";
import { createCombinationIntent, operationIdToken } from "./combinationAuthoring";
import * as batchService from "../../services/operationBatchService";
import { emptySeismicTemplateDraft, seismicTemplateIntents } from "./seismicTemplate";
import { vi } from "vitest";

const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;

/** The demo document plus invented axis-state cases: each is a copy of the base case's
 * actions with one invented static excitation along its axis (test input only). */
function seismicModel(): PreviewModel {
  const model = clone(demoModel) as unknown as PreviewModel;
  const base = model.load_cases.find((loadCase) => loadCase.id === "load:L-100")!;
  for (const axis of ["X", "Y", "Z"] as const) {
    const state = clone(base);
    state.id = `load:EQ-${axis}`;
    state.label = `Invented base + ${axis} static excitation`;
    state.primitive_loads = [
      ...(state.primitive_loads ?? []).map((load) => ({ ...load, id: `${String(load.id)}-${axis}` })),
      { id: `load:EQ-${axis}-Q`, category: "occasional", target: { type: "element", pipe: "pipe:P-120" }, direction: `global_${axis.toLowerCase()}`,
        magnitude: { value: 40, unit: "N/m" }, dimension: "force_per_length", provenance: "invented_test_excitation" }
    ];
    model.load_cases.push(state);
  }
  return model;
}

async function applyOutcome(model: PreviewModel, intent: EditorOperationIntent): Promise<OperationOutcome> {
  return applyModelOperation(model, intent, await computeModelHash(model));
}
async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyOutcome(model, intent);
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}
const codes = (outcome: OperationOutcome) => outcome.diagnostics.map((item) => item.code);
const blockingSlotCodes = (model: PreviewModel) => modelDocumentSlotFindings(model).filter((item) => item.severity === "blocking").map((item) => `${item.code}@${item.path}`);
const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const combination = (model: PreviewModel, id: string) => model.combinations!.find((item) => item.id === id)!;

function mountPanel(initial: PreviewModel) {
  const queued: EditorOperationIntent[] = [];
  let model = initial;
  const element = () => <LoadCaseManagerPanel model={model} onQueueIntent={(intent) => queued.push(intent)} onSelect={() => undefined} selection={{ type: "project", id: model.project.id }} />;
  const view = render(element());
  return {
    queued,
    get model() { return model; },
    set(next: PreviewModel) { model = next; view.rerender(element()); }
  };
}

async function queueAndApply(panel: ReturnType<typeof mountPanel>, testId = "queue-create-combination-intent"): Promise<EditorOperationIntent> {
  const before = panel.queued.length;
  expect(byId(testId)).toBeEnabled();
  fireEvent.click(byId(testId));
  await waitFor(() => expect(panel.queued).toHaveLength(before + 1));
  const intent = panel.queued[before];
  panel.set(await apply(panel.model, intent));
  return intent;
}

function createSubtraction(minuend: string, subtrahend: string) {
  type("load-manager-create-combination-basis", "result_state_subtraction");
  chooseVirtualTarget("load-manager-create-combination-minuend", minuend);
  chooseVirtualTarget("load-manager-create-combination-subtrahend", subtrahend);
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("the end-to-end path: one nested SRSS over two differences", () => {
  it("authors two differences and an SRSS naming them as typed combination operands; the engine applies each and the stored records are exactly as written", async () => {
    const start = seismicModel();
    const slotsBefore = blockingSlotCodes(start);
    const panel = mountPanel(start);

    // Difference 1: X state minus base; difference 2: Y state minus base (no difference_basis).
    createSubtraction("load:EQ-X", "load:L-100");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("Minuend: load:EQ-X resolves to the load case. Subtrahend: load:L-100 resolves to the load case.");
    const first = await queueAndApply(panel);
    expect(JSON.parse(first.change.after)).toEqual({
      id: "combination:C-300", label: "User mechanics combination", basis: "result_state_subtraction", provenance: "user_entered_local_preview",
      minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: []
    });
    expect(byId("load-manager-create-combination-id")).toHaveValue("combination:C-301");
    createSubtraction("load:EQ-Y", "load:L-100");
    await queueAndApply(panel);

    // The SRSS: two typed combination terms.
    type("load-manager-create-combination-basis", "srss");
    type("load-manager-create-combination-label", "Invented X/Y static seismic SRSS");
    type("load-manager-create-combination-term-0-kind", "combination");
    chooseVirtualTarget("load-manager-create-combination-term-0-combination", "combination:C-300");
    type("load-manager-create-combination-factor", "1");
    fireEvent.click(byId("load-manager-create-combination-add-term"));
    type("load-manager-create-combination-term-1-kind", "combination");
    chooseVirtualTarget("load-manager-create-combination-term-1-combination", "combination:C-301");
    type("load-manager-create-combination-term-1-factor", "1");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("basis=srss; terms=combination combination:C-300 x 1; combination combination:C-301 x 1");
    expect(byId("load-manager-create-combination-gates")).toHaveTextContent("not an equilibrium state");
    const srss = await queueAndApply(panel);
    expect(srss.change.after).toBe(JSON.stringify({
      id: "combination:C-302", label: "Invented X/Y static seismic SRSS", basis: "srss", provenance: "user_entered_local_preview",
      terms: [{ combination: "combination:C-300", factor: 1 }, { combination: "combination:C-301", factor: 1 }]
    }));

    const stored = panel.model;
    expect(combination(stored, "combination:C-302")).toEqual({
      id: "combination:C-302", label: "Invented X/Y static seismic SRSS", basis: "srss",
      terms: [{ combination: "combination:C-300", factor: 1 }, { combination: "combination:C-301", factor: 1 }], provenance: "user_entered_local_preview"
    });
    expect(Object.hasOwn(combination(stored, "combination:C-300"), "difference_basis")).toBe(false);
    // The engine's document validator adds no blocking finding; the stored rows show the nesting.
    expect(blockingSlotCodes(stored)).toEqual(slotsBefore);
    expect(byId("load-manager-combination-combination:C-302")).toHaveTextContent("terms=combination combination:C-300 x 1; combination combination:C-301 x 1");
    expect(byId("load-manager-combination-resolved-combination:C-300")).toHaveTextContent("load:EQ-X resolves to the load case; load:L-100 resolves to the load case");
  });
});

describe("typed terms", () => {
  it("writes each term exactly as typed, with its factor and kind, in the order shown; reordering moves the payload; repeats are refused in place", async () => {
    let model = seismicModel();
    model = await apply(model, createCombinationIntent({ model, payload: { id: "combination:D-X", label: "dx", basis: "result_state_subtraction", provenance: "test", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: [] }, rationale: "test", sourceNote: "test" }));
    const panel = mountPanel(model);
    type("load-manager-create-combination-basis", "absolute_sum");
    chooseVirtualTarget("load-manager-create-combination-load-case", "load:L-100");
    type("load-manager-create-combination-factor", "-2.5");
    fireEvent.click(byId("load-manager-create-combination-add-term"));
    type("load-manager-create-combination-term-1-kind", "combination");
    chooseVirtualTarget("load-manager-create-combination-term-1-combination", "combination:D-X");
    type("load-manager-create-combination-term-1-factor", "0.5");
    fireEvent.click(byId("load-manager-create-combination-add-term"));
    chooseVirtualTarget("load-manager-create-combination-term-2-load-case", "load:EQ-Z");
    type("load-manager-create-combination-term-2-factor", "3");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("terms=load:L-100 x -2.5; combination combination:D-X x 0.5; load:EQ-Z x 3");

    // Move the last term up twice: it becomes the first; the first term's controls keep their place.
    fireEvent.click(byId("load-manager-create-combination-term-2-up"));
    fireEvent.click(byId("load-manager-create-combination-term-1-up"));
    expect(byId("load-manager-create-combination-term-0-up")).toBeDisabled();
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("terms=load:EQ-Z x 3; load:L-100 x -2.5; combination combination:D-X x 0.5");

    // A repeated operand is refused in place and nothing can be queued.
    type("load-manager-create-combination-term-1-factor", "1");
    chooseVirtualTarget("load-manager-create-combination-term-1-load-case", "load:EQ-Z");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 2: repeats load case load:EQ-Z; one factor per operand");
    chooseVirtualTarget("load-manager-create-combination-term-1-load-case", "load:L-100");
    type("load-manager-create-combination-term-1-factor", "-2.5");

    const intent = await queueAndApply(panel);
    expect(intent.change.after).toBe(JSON.stringify({
      id: "combination:C-300", label: "User mechanics combination", basis: "absolute_sum", provenance: "user_entered_local_preview",
      terms: [{ load_case: "load:EQ-Z", factor: 3 }, { load_case: "load:L-100", factor: -2.5 }, { combination: "combination:D-X", factor: 0.5 }]
    }));
    expect(combination(panel.model, "combination:C-300").terms).toEqual([
      { load_case: "load:EQ-Z", factor: 3 }, { load_case: "load:L-100", factor: -2.5 }, { combination: "combination:D-X", factor: 0.5 }
    ]);
    // An unsigned draft naming load cases directly states the κ gate (a load case has κ = 1).
    type("load-manager-create-combination-basis", "srss");
    expect(byId("load-manager-create-combination-gates")).toHaveTextContent("CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND");
    expect(byId("load-manager-create-combination-gates")).toHaveTextContent("This draft names load cases load:EQ-Z, load:L-100 directly (κ = 1)");
    expect(byId("load-manager-create-combination-gates")).toHaveTextContent("constant-effort support support:CE-120; the solve decides whether it consumes it");
  });

  it("shows a bare reference's resolved target and offers a declared operating comparison load cases only", async () => {
    let model = seismicModel();
    model = await apply(model, createCombinationIntent({ model, payload: { id: "combination:D-X", label: "dx", basis: "result_state_subtraction", provenance: "test", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: [] }, rationale: "test", sourceNote: "test" }));
    // A combination shadowed by a load case id is not offered to a bare reference.
    model.combinations!.push({ id: "load:L-200", label: "shadowed", basis: "mechanics", terms: [{ load_case: "load:L-100", factor: 1 }], provenance: "test" });
    const panel = mountPanel(model);
    type("load-manager-create-combination-basis", "range_envelope");
    chooseVirtualMultiTarget("load-manager-create-combination-operands", "load:L-100");
    chooseVirtualMultiTarget("load-manager-create-combination-operands", "combination:D-X");
    type("load-manager-create-combination-mode", "max");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("combination:D-X resolves to the combination");
    // Search the (virtualized) list for the shared id: only the load case is offered.
    fireEvent.change(within(byId("load-manager-create-combination-operands")).getByRole("combobox"), { target: { value: "L-200" } });
    const operands = within(byId("load-manager-create-combination-operands")).getAllByRole("option").map((option) => option.textContent);
    expect(operands).toEqual(["Load case: Invented alternate gravity and occasional previewload:L-200"]);
    fireEvent.change(within(byId("load-manager-create-combination-operands")).getByRole("combobox"), { target: { value: "" } });
    const envelope = await queueAndApply(panel);
    expect(JSON.parse(envelope.change.after).operand_ids).toEqual(["load:L-100", "combination:D-X"]);

    type("load-manager-create-combination-basis", "result_state_subtraction");
    fireEvent.click(byId("load-manager-operating-difference-toggle"));
    const minuendOptions = within(byId("load-manager-create-combination-minuend")).getAllByRole("option").map((option) => option.textContent ?? "");
    expect(minuendOptions.some((text) => text.endsWith("combination:D-X"))).toBe(false);
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE");

    // Built outside the editor, the engine refuses the same thing by name.
    const refused = await applyOutcome(panel.model, createCombinationIntent({ model: panel.model, payload: { id: "combination:BAD", label: "bad", basis: "result_state_subtraction", provenance: "test",
      minuend_id: "combination:D-X", subtrahend_id: "load:L-100", terms: [], difference_basis: "operating_minus_sustained" }, rationale: "test", sourceNote: "test" }));
    expect(refused.validation.application_status).toBe("blocked");
    expect(codes(refused)).toContain("LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE");
  });
});

describe("cycles and refused operands", () => {
  it("prevents a self-reference and a cycle-closing term with the reason shown; the engine refuses each by name when built outside the editor", async () => {
    let model = seismicModel();
    const create = (payload: Record<string, unknown> & { id: string }) => createCombinationIntent({ model, payload, rationale: "test", sourceNote: "test" });
    model = await apply(model, create({ id: "combination:D-X", label: "dx", basis: "result_state_subtraction", provenance: "test", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: [] }));
    model = await apply(model, create({ id: "combination:B", label: "b", basis: "srss", provenance: "test", terms: [{ combination: "combination:D-X", factor: 1 }] }));
    model = await apply(model, create({ id: "combination:A", label: "a", basis: "mechanics", provenance: "test", terms: [{ load_case: "load:L-100", factor: 1 }, { combination: "combination:B", factor: 1 }] }));
    const panel = mountPanel(model);

    // Add a term to B: B itself is self-reference, A (which names B) would close a cycle.
    chooseVirtualTarget("load-manager-create-combination-term-combination", "combination:B");
    type("load-manager-create-combination-term-kind", "combination");
    const option = (suffix: string) => within(byId("load-manager-create-combination-term-operand-combination")).getAllByRole("option").find((item) => item.textContent?.endsWith(suffix))!;
    expect(option("combination:B")).toBeDisabled();
    expect(option("combination:B")).toHaveTextContent("would name itself (self-reference; the engine refuses it as LOAD_COMBINATION_OPERAND_CYCLE)");
    expect(option("combination:A")).toBeDisabled();
    expect(option("combination:A")).toHaveTextContent("would close a cycle (combination:A → combination:B → combination:A; the engine refuses it as LOAD_COMBINATION_OPERAND_CYCLE)");
    expect(option("combination:D-X")).toBeDisabled();
    expect(option("combination:D-X")).toHaveTextContent("already a term of combination:B");
    expect(option("combination:C-OPER-ALT")).toBeEnabled();
    chooseVirtualTarget("load-manager-create-combination-term-operand-combination", "combination:C-OPER-ALT");
    type("load-manager-create-combination-term-factor", "-1");
    const term = await queueAndApply(panel, "queue-create-combination-term-intent");
    expect(term.change).toMatchObject({ change_kind: "create_combination_term", field_path: "terms", after: JSON.stringify({ combination: "combination:C-OPER-ALT", factor: -1 }) });
    expect(combination(panel.model, "combination:B").terms).toEqual([{ combination: "combination:D-X", factor: 1 }, { combination: "combination:C-OPER-ALT", factor: -1 }]);

    // Built outside the editor: the engine refuses a cycle, a self-reference and an unknown operand by name.
    const term_ = (target: string, payload: Record<string, unknown>): EditorOperationIntent => ({
      ...term, target: { object_type: "Combination", ref: target }, change: { ...term.change, after: JSON.stringify(payload) }
    });
    const cycle = await applyOutcome(panel.model, term_("combination:B", { combination: "combination:A", factor: 1 }));
    expect(codes(cycle)).toContain("LOAD_COMBINATION_OPERAND_CYCLE");
    expect(cycle.applied_model ?? null).toBeNull();
    const self = await applyOutcome(panel.model, term_("combination:B", { combination: "combination:B", factor: 1 }));
    expect(codes(self)).toContain("LOAD_COMBINATION_OPERAND_CYCLE");
    const unknown = await applyOutcome(panel.model, createCombinationIntent({ model: panel.model, payload: { id: "combination:U", label: "u", basis: "range_envelope", provenance: "test", operand_ids: ["combination:NOPE"], mode: "max", terms: [] }, rationale: "test", sourceNote: "test" }));
    expect(unknown.validation.application_status).toBe("blocked");
    expect(codes(unknown)).toContain("OP-COMBINATION-OPERAND-LOAD-NOT-FOUND");

    // Since #115 a combination term deletes too; B is still named by A, so its delete names the referencing path first
    // (combinationDelete.engine.test.tsx covers the deletes and the refusal).
    fireEvent.click(byId("load-manager-combination-term-combination:B-1"));
    // Its before value is the stored term's canonical JSON, computed by the engine.
    await waitFor(() => expect(byId("queue-delete-combination-term-intent")).toBeEnabled());
    expect(byId("load-manager-combination-entity-delete-references")).toHaveTextContent("Named by combination:A.terms.1.");
  });
});

describe("the three-axis static seismic template", () => {
  function chooseTemplate(basis: "srss" | "absolute_sum") {
    chooseVirtualTarget("seismic-template-base", "load:L-100");
    chooseVirtualTarget("seismic-template-axis-x", "load:EQ-X");
    chooseVirtualTarget("seismic-template-axis-y", "load:EQ-Y");
    chooseVirtualTarget("seismic-template-axis-z", "load:EQ-Z");
    type("seismic-template-basis", basis);
    for (const axis of ["x", "y", "z"]) type(`seismic-template-factor-${axis}`, "1");
  }

  it("defaults nothing, previews the four records and the gates, has the engine check them as one sequence, and queues them; each applies through the engine", async () => {
    const start = seismicModel();
    // Invented variant: the constant-effort support declares one translational DOF, the form the solve consumes.
    start.supports.find((support) => support.id === "support:CE-120")!.restraints = ["UZ"];
    const slotsBefore = blockingSlotCodes(start);
    const panel = mountPanel(start);
    expect(byId("seismic-template-status")).toHaveTextContent("Not ready: choose the base case.");
    expect(byId("seismic-template-basis")).toHaveValue("");
    expect(byId("seismic-template-queue")).toBeDisabled();
    chooseVirtualTarget("seismic-template-base", "load:L-100");
    chooseVirtualTarget("seismic-template-axis-x", "load:L-100");
    expect(byId("seismic-template-status")).toHaveTextContent("the X axis-state case load:L-100 is already chosen");
    chooseTemplate("srss");
    expect(byId("seismic-template-status")).toHaveTextContent("Ready");

    // The preview: base vs axis states, κ = 0 differences, the unsigned combination and the gates.
    const preview = byId("seismic-template-preview");
    expect(preview).toHaveTextContent("each difference has κ = 0 (§5.1)");
    expect(preview).toHaveTextContent("not \"base case + EQ-only case\", which would carry the fixed load twice");
    expect(byId("seismic-template-record-0")).toHaveTextContent("combination:SEISMIC-X-300: result_state_subtraction, load:EQ-X − load:L-100 (κ = 0; no difference_basis)");
    expect(byId("seismic-template-record-3")).toHaveTextContent("combination:SEISMIC-300: srss over combination combination:SEISMIC-X-300 x 1, combination combination:SEISMIC-Y-300 x 1, combination combination:SEISMIC-Z-300 x 1 (unsigned; not an equilibrium state)");
    expect(byId("seismic-template-gate-fixed-load")).toHaveTextContent("CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND, INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND");
    expect(byId("seismic-template-gate-fixed-load")).toHaveTextContent("constant-effort support support:CE-120");
    expect(byId("seismic-template-gate-nonlinear")).toHaveTextContent("the operands are derived differences, not an equilibrium load state");
    expect(byId("seismic-template-gate-nonlinear")).toHaveTextContent("nonlinear supports support:NL-140, support:NL-130-FRIC");
    expect(byId("seismic-template-boundary")).toHaveTextContent("no code, response-spectrum or modal-response claim");
    expect(byId("seismic-template-apply")).toHaveTextContent("Apply them together with 'Review all queued changes as one batch' to apply all four or none");
    expect(byId("seismic-template-apply")).toHaveTextContent("applied singly, each applies on its own");
    expect(preview).not.toHaveTextContent("minuend_final");

    fireEvent.click(byId("seismic-template-queue"));
    await waitFor(() => expect(panel.queued).toHaveLength(4));
    expect(byId("seismic-template-engine-notice")).toHaveTextContent("Checked by the operation engine as one sequence and queued as four changes");
    expect(screen.queryByTestId("seismic-template-engine-error")).toBeNull();
    const afters = panel.queued.map((intent) => intent.change.after);
    const difference = (axis: string) => JSON.stringify({ id: `combination:SEISMIC-${axis}-300`, label: `${axis} axis state minus base (static seismic difference)`, basis: "result_state_subtraction",
      provenance: "user_entered_three_axis_static_seismic_template", minuend_id: `load:EQ-${axis}`, subtrahend_id: "load:L-100", terms: [] });
    expect(afters).toEqual([
      difference("X"), difference("Y"), difference("Z"),
      JSON.stringify({ id: "combination:SEISMIC-300", label: "Three-axis static seismic SRSS", basis: "srss", provenance: "user_entered_three_axis_static_seismic_template",
        terms: ["X", "Y", "Z"].map((axis) => ({ combination: `combination:SEISMIC-${axis}-300`, factor: 1 })) })
    ]);
    expect(afters.join("")).not.toContain("difference_basis");
    expect(afters.join("")).not.toContain("subtrahend_support_state");

    // Applied first, the unsigned combination is refused by name; in order, each applies.
    const early = await applyOutcome(panel.model, panel.queued[3]);
    expect(early.validation.application_status).toBe("blocked");
    expect(codes(early)).toContain("OP-COMBINATION-TERM-LOAD-NOT-FOUND");
    let model = panel.model;
    for (const intent of panel.queued) model = await apply(model, intent);
    expect(model.combinations!.slice(-4).map((item) => [item.id, item.basis])).toEqual([
      ["combination:SEISMIC-X-300", "result_state_subtraction"], ["combination:SEISMIC-Y-300", "result_state_subtraction"],
      ["combination:SEISMIC-Z-300", "result_state_subtraction"], ["combination:SEISMIC-300", "srss"]
    ]);
    // Load cases and supports are as authored: the template wrote combinations only.
    expect(model.load_cases).toEqual(start.load_cases);
    expect(model.supports).toEqual(start.supports);
    expect(blockingSlotCodes(model)).toEqual(slotsBefore);

    // Nonlinear supports where admitted (OQ-1): base case at factor 1 plus the κ = 0 SRSS, authored with typed terms.
    panel.set(model);
    type("load-manager-create-combination-label", "Invented base plus seismic bound");
    expect(byId("load-manager-create-combination-gates")).toHaveTextContent("NONLINEAR_COMBINATION_REQUIRES_SOLVE");
    chooseVirtualTarget("load-manager-create-combination-load-case", "load:L-100");
    type("load-manager-create-combination-factor", "1");
    fireEvent.click(byId("load-manager-create-combination-add-term"));
    type("load-manager-create-combination-term-1-kind", "combination");
    chooseVirtualTarget("load-manager-create-combination-term-1-combination", "combination:SEISMIC-300");
    type("load-manager-create-combination-term-1-factor", "1");
    const oq1 = await queueAndApply(panel);
    expect(JSON.parse(oq1.change.after).terms).toEqual([{ load_case: "load:L-100", factor: 1 }, { combination: "combination:SEISMIC-300", factor: 1 }]);
  });

  it("offers absolute_sum as the person's choice and refuses a taken id in place", async () => {
    const start = seismicModel();
    start.combinations!.push({ id: "combination:SEISMIC-Y-300", label: "taken", basis: "mechanics", terms: [{ load_case: "load:L-100", factor: 1 }], provenance: "test" });
    const panel = mountPanel(start);
    chooseTemplate("absolute_sum");
    // The proposal skips the taken id; an id the person types that exists is refused in place.
    expect(byId("seismic-template-difference-id-y")).toHaveValue("combination:SEISMIC-Y-301");
    type("seismic-template-difference-id-y", "combination:SEISMIC-Y-300");
    expect(byId("seismic-template-queue")).toBeDisabled();
    expect(byId("seismic-template-status")).toHaveTextContent("id=combination:SEISMIC-Y-300 already exists");
    type("seismic-template-difference-id-y", "combination:SEISMIC-Y-301");
    fireEvent.click(byId("seismic-template-queue"));
    await waitFor(() => expect(panel.queued).toHaveLength(4));
    const unsigned = JSON.parse(panel.queued[3].change.after);
    expect(unsigned).toMatchObject({ basis: "absolute_sum", label: "Three-axis static seismic absolute sum" });
    let model = panel.model;
    for (const intent of panel.queued) model = await apply(model, intent);
    expect(combination(model, "combination:SEISMIC-301").terms).toEqual(["X-301", "Y-301", "Z-301"].map((suffix) => ({ combination: `combination:SEISMIC-${suffix}`, factor: 1 })));
  });
});

describe("editing a stored nested record", () => {
  it("a factor or basis edit writes only its own member; the rest of the record and its references are kept", async () => {
    let model = seismicModel();
    const create = (payload: Record<string, unknown> & { id: string }) => createCombinationIntent({ model, payload, rationale: "test", sourceNote: "test" });
    model = await apply(model, create({ id: "combination:D-X", label: "dx", basis: "result_state_subtraction", provenance: "test", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: [] }));
    model = await apply(model, create({ id: "combination:S", label: "s", basis: "srss", provenance: "test", terms: [{ load_case: "load:EQ-Y", factor: 1 }, { combination: "combination:D-X", factor: 2 }] }));
    // A stored annotation the editor does not edit.
    (combination(model, "combination:S") as unknown as Record<string, unknown>).notes = "invented note kept as stored";
    const before = clone(combination(model, "combination:S"));
    const panel = mountPanel(model);
    fireEvent.click(byId("load-manager-combination-term-combination:S-1"));
    expect(byId("load-manager-selected-combination-term")).toHaveTextContent("term=combination combination:D-X; path=terms.1.factor");
    expect(byId("load-manager-combination-retained")).toHaveTextContent("Retained as stored (this editor does not edit them; a basis, factor or term edit writes only its own member): notes.");
    type("load-manager-combination-factor-value", "-0.75");
    await queueAndApply(panel, "queue-combination-factor-intent");
    // The engine returns its applied document with sorted keys; members and values are compared here.
    const edited = combination(panel.model, "combination:S");
    expect(edited).toEqual({ ...before, terms: [before.terms![0], { combination: "combination:D-X", factor: -0.75 }] });
    expect(Object.keys(edited.terms![1])).toEqual(["combination", "factor"]);

    type("load-manager-combination-basis-value", "absolute_sum");
    await queueAndApply(panel, "queue-combination-basis-intent");
    expect(combination(panel.model, "combination:S")).toEqual({ ...edited, basis: "absolute_sum" });
    expect(combination(panel.model, "combination:D-X")).toEqual(combination(model, "combination:D-X"));
  });
});

describe("nothing is chosen for the person (review round 1)", () => {
  it("a basis chosen and queued without operands queues nothing and names what is missing; a new row and add-term start blank", () => {
    const panel = mountPanel(seismicModel());
    for (const basis of ["mechanics", "srss", "absolute_sum"]) {
      type("load-manager-create-combination-basis", basis);
      expect(byId("load-manager-create-combination-heading")).toHaveTextContent(`basis=${basis}; term=TBD x TBD`);
      expect(byId("queue-create-combination-intent")).toBeDisabled();
      expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 1: not chosen: its load case");
      fireEvent.click(byId("queue-create-combination-intent"));
    }
    type("load-manager-create-combination-basis", "result_state_subtraction");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: the minuend");
    fireEvent.click(byId("queue-create-combination-intent"));
    type("load-manager-create-combination-basis", "range_envelope");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: any operand");
    expect(byId("load-manager-create-combination-mode")).toHaveValue("");
    fireEvent.click(byId("queue-create-combination-intent"));
    expect(panel.queued).toEqual([]);

    // A new row: blank operand, blank factor.
    type("load-manager-create-combination-basis", "srss");
    chooseVirtualTarget("load-manager-create-combination-load-case", "load:EQ-X");
    type("load-manager-create-combination-factor", "1");
    fireEvent.click(byId("load-manager-create-combination-add-term"));
    expect(byId("load-manager-create-combination-term-1-factor")).toHaveValue("");
    expect(within(byId("load-manager-create-combination-term-1-load-case")).getByLabelText("Term 2 load case current value")).toHaveTextContent("No target selected");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 2: not chosen: its load case");
    chooseVirtualTarget("load-manager-create-combination-term-1-load-case", "load:EQ-Y");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 2: not entered: its factor");
    expect(byId("queue-create-combination-intent")).toBeDisabled();

    // Add-term on an existing combination: no case or factor proposed.
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("new term; load=TBD; factor=TBD");
    expect(byId("load-manager-create-combination-term-factor")).toHaveValue("");
    expect(byId("queue-create-combination-term-intent")).toBeDisabled();
    expect(panel.queued).toEqual([]);
  });

  it("the template needs each difference's factor entered", () => {
    mountPanel(seismicModel());
    chooseVirtualTarget("seismic-template-base", "load:L-100");
    chooseVirtualTarget("seismic-template-axis-x", "load:EQ-X");
    chooseVirtualTarget("seismic-template-axis-y", "load:EQ-Y");
    chooseVirtualTarget("seismic-template-axis-z", "load:EQ-Z");
    type("seismic-template-basis", "srss");
    expect(byId("seismic-template-factor-x")).toHaveValue("");
    expect(byId("seismic-template-status")).toHaveTextContent("not entered: the factor of the X difference");
    expect(byId("seismic-template-queue")).toBeDisabled();
    type("seismic-template-factor-x", "1");
    type("seismic-template-factor-y", "0.5");
    expect(byId("seismic-template-status")).toHaveTextContent("not entered: the factor of the Z difference");
    type("seismic-template-factor-z", "1");
    expect(byId("seismic-template-record-3")).toHaveTextContent("combination combination:SEISMIC-Y-300 x 0.5");
  });
});

describe("a draft changed while the engine checks it (review round 1)", () => {
  it("queues nothing: the check is invalidated and the person is told to check again", async () => {
    const real = batchService.validateOperationBatch;
    let release: () => void = () => undefined;
    const gate = new Promise<void>((resolve) => { release = resolve; });
    const spy = vi.spyOn(batchService, "validateOperationBatch").mockImplementation(async (...args) => { await gate; return real(...args); });
    try {
      const panel = mountPanel(seismicModel());
      chooseVirtualTarget("seismic-template-base", "load:L-100");
      chooseVirtualTarget("seismic-template-axis-x", "load:EQ-X");
      chooseVirtualTarget("seismic-template-axis-y", "load:EQ-Y");
      chooseVirtualTarget("seismic-template-axis-z", "load:EQ-Z");
      type("seismic-template-basis", "srss");
      for (const axis of ["x", "y", "z"]) type(`seismic-template-factor-${axis}`, "1");
      fireEvent.click(byId("seismic-template-queue"));
      await waitFor(() => expect(spy).toHaveBeenCalledTimes(1));
      // The inputs are frozen while the check runs; a change that still reaches the draft invalidates the check.
      expect(byId("seismic-template-inputs")).toBeDisabled();
      type("seismic-template-basis", "absolute_sum");
      release();
      await waitFor(() => expect(byId("seismic-template-status")).not.toHaveTextContent("Checking"));
      // The change reached the draft (an event bypassing the frozen input): nothing is queued.
      expect(byId("seismic-template-basis")).toHaveValue("absolute_sum");
      expect(byId("seismic-template-engine-error")).toHaveTextContent("The template changed while the operation engine was checking it; nothing was queued.");
      expect(panel.queued).toEqual([]);
      expect(byId("seismic-template-inputs")).not.toBeDisabled();
    } finally {
      spy.mockRestore();
    }
  });
});

describe("operation ids (review round 1)", () => {
  it("distinct raw ids give distinct operation ids, and a template set using such ids passes batch preflight", async () => {
    expect(operationIdToken("combination:EQ/X")).toBe("combination:EQ~2FX");
    expect(operationIdToken("combination:EQ-X")).toBe("combination:EQ-X");
    expect(operationIdToken("a~2Fb")).toBe("a~7E2Fb");
    expect(operationIdToken("a/b")).not.toBe(operationIdToken("a~2Fb"));
    expect(operationIdToken("Δ")).toBe("~CE~94");
    const model = seismicModel();
    const draft = { ...emptySeismicTemplateDraft(model), baseId: "load:L-100", axisIds: { x: "load:EQ-X", y: "load:EQ-Y", z: "load:EQ-Z" }, basis: "srss" as const,
      factors: { x: "1", y: "1", z: "1" }, differenceIds: { x: "combination:EQ/X", y: "combination:EQ-X", z: "combination:EQ X" } };
    const intents = seismicTemplateIntents(model, draft);
    expect(intents.map((intent) => intent.operation_id)).toEqual([
      "op:load-manager-create-combination:EQ~2FX", "op:load-manager-create-combination:EQ-X", "op:load-manager-create-combination:EQ~20X", "op:load-manager-create-combination:SEISMIC-300"
    ]);
    expect(new Set(intents.map((intent) => intent.change.change_id)).size).toBe(4);
    // Target ids and payload values are the raw ids.
    expect(intents.map((intent) => intent.target.ref).slice(0, 3)).toEqual(["combination:EQ/X", "combination:EQ-X", "combination:EQ X"]);
    expect(JSON.parse(intents[3].change.after).terms.map((term: { combination: string }) => term.combination)).toEqual(["combination:EQ/X", "combination:EQ-X", "combination:EQ X"]);
    const outcome = await batchService.validateOperationBatch(model, { batch_id: "ids", operations: intents }, await computeModelHash(model));
    expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
    expect(outcome.validation.batch_validation_status).toBe("passed");
  });
});

describe("Go to Create load case (review round 1)", () => {
  it("focuses Create load case's ID field and says so in a status line", () => {
    mountPanel(seismicModel());
    expect(screen.queryByTestId("seismic-template-create-load-case-status")).toBeNull();
    fireEvent.click(byId("seismic-template-create-load-case"));
    expect(byId("load-manager-create-load-id")).toHaveFocus();
    expect(byId("seismic-template-create-load-case-status")).toHaveTextContent("Create load case (above, in this panel) is shown with its ID field focused.");
  });
});
