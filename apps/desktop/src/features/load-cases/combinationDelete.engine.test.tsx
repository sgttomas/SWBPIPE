// Combination and combination-term deletes on the applier contract of #115 (DEL-05-02 Work
// `absolute-sum-srss-combinations.md` §10.5; DEL-07-03 `Work/combination-editor.md`), through
// the load manager and the real wasm operation engine. Each delete is built by the panel from
// the stored record or term (historical display where the engine keeps it, canonical JSON
// otherwise), checked by the engine before it is queued, and applied with
// `applyModelOperation`, the route Review/Apply uses. Models are built in test code from the
// committed invented demo document.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import * as previewService from "../../services/previewService";
import { applyModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { EditorOperationIntent, EntityRef, OperationOutcome, PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";
import { createCombinationIntent } from "./combinationAuthoring";
import { engineDisplayNumber, historicalCombinationDisplay } from "./combinationDelete";
import { emptySeismicTemplateDraft, seismicTemplateIntents } from "./seismicTemplate";
import type { Combination } from "./combinationOperands";

const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;
type Record_ = Record<string, unknown> & { id: string };

/** The demo document plus invented axis-state cases and the stored records under test (test input only). */
function baseModel(): PreviewModel {
  const model = clone(demoModel) as unknown as PreviewModel;
  model.components ??= [];
  model.diagnostics ??= [];
  const base = model.load_cases.find((loadCase) => loadCase.id === "load:L-100")!;
  for (const axis of ["X", "Y", "Z"] as const) {
    const state = clone(base);
    state.id = `load:EQ-${axis}`;
    state.label = `Invented base + ${axis} static excitation`;
    state.primitive_loads = (state.primitive_loads ?? []).map((load) => ({ ...load, id: `${String(load.id)}-${axis}` }));
    model.load_cases.push(state);
  }
  return model;
}

const SUB_T = { id: "combination:SUB-T", label: "sub with empty terms", basis: "result_state_subtraction", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100", terms: [], provenance: "test" };
const SUB_N = { id: "combination:SUB-N", label: "sub without terms", basis: "result_state_subtraction", minuend_id: "load:EQ-Y", subtrahend_id: "load:L-100", provenance: "test" };
const ENV = { id: "combination:ENV", label: "envelope", basis: "range_envelope", operand_ids: ["load:L-100", "load:L-200"], mode: "max_abs", provenance: "test" };
const SRSS = { id: "combination:S", label: "srss", basis: "srss", terms: [{ combination: "combination:SUB-T", factor: 1 }, { combination: "combination:SUB-N", factor: 0.5 }], provenance: "test" };
const MECH = { id: "combination:M", label: " flat mechanics ", basis: "mechanics", terms: [{ load_case: "load:L-100", factor: 1.5 }, { load_case: "load:L-200", factor: 1e-7 }], provenance: "test" };

function modelWith(...records: Record_[]): PreviewModel {
  const model = baseModel();
  model.combinations!.push(...(clone(records) as unknown as Combination[]));
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
const byId = (testId: string) => screen.getByTestId(testId);
const ids = (model: PreviewModel) => (model.combinations ?? []).map((combination) => combination.id);
const stored = (model: PreviewModel, id: string) => model.combinations!.find((combination) => combination.id === id)!;

/** The load manager with a working selection, as the app wires it. */
function mountPanel(initial: PreviewModel) {
  const queued: EditorOperationIntent[] = [];
  const handle: { setModel?: (model: PreviewModel) => void; model: PreviewModel } = { model: initial };
  function Harness() {
    const [model, setModel] = useState(initial);
    const [selection, setSelection] = useState<EntityRef>({ type: "project", id: initial.project.id });
    handle.setModel = (next) => { handle.model = next; setModel(next); };
    return <LoadCaseManagerPanel model={model} onQueueIntent={(intent) => queued.push(intent)} onSelect={setSelection} selection={selection} queuedIntents={[]} />;
  }
  render(<Harness />);
  return { queued, handle, set: (next: PreviewModel) => act(() => handle.setModel!(next)) };
}

async function queueEntityDelete(panel: ReturnType<typeof mountPanel>, id: string): Promise<EditorOperationIntent> {
  fireEvent.click(byId(`load-manager-combination-select-${id}`));
  const before = panel.queued.length;
  await waitFor(() => expect(byId("queue-delete-combination-intent")).toBeEnabled());
  fireEvent.click(byId("queue-delete-combination-intent"));
  await waitFor(() => expect(panel.queued).toHaveLength(before + 1));
  return panel.queued[before];
}

async function queueTermDelete(panel: ReturnType<typeof mountPanel>, id: string, index: number): Promise<EditorOperationIntent> {
  fireEvent.click(byId(`load-manager-combination-term-${id}-${index}`));
  const before = panel.queued.length;
  await waitFor(() => expect(byId("queue-delete-combination-term-intent")).toBeEnabled());
  fireEvent.click(byId("queue-delete-combination-term-intent"));
  await waitFor(() => expect(panel.queued).toHaveLength(before + 1));
  return panel.queued[before];
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(() => { cleanup(); vi.restoreAllMocks(); });

describe("the before value is built from the stored record only", () => {
  it("mirrors the engine's display_number", () => {
    expect([1, -3, 0, -0, 1.5, 1e-7, 1e15, 1e21, 123456789.125, -2.5e-10].map(engineDisplayNumber)).toEqual(
      ["1", "-3", "0", "0", "1.5", "0.0000001", "1000000000000000", "1000000000000000000000", "123456789.125", "-0.00000000025"]);
    expect(historicalCombinationDisplay(SUB_T)).toBe("combination:SUB-T; sub with empty terms; basis=result_state_subtraction; terms=none");
    expect(historicalCombinationDisplay(MECH)).toBe("combination:M; flat mechanics; basis=mechanics; terms=load:L-100 x 1.5; load:L-200 x 0.0000001");
    expect(historicalCombinationDisplay(SUB_N)).toBeNull();
    expect(historicalCombinationDisplay(SRSS)).toBeNull();
  });
});

describe("deleting each record form through the panel and the engine", () => {
  it("deletes an SRSS of combinations (canonical), subtractions with and without terms, an envelope and a flat mechanics record", async () => {
    const start = modelWith(SUB_T, SUB_N, ENV, SRSS, MECH);
    const panel = mountPanel(start);
    let model = start;

    // SRSS names both subtractions, so it goes first; its before is its canonical JSON.
    const srss = await queueEntityDelete(panel, "combination:S");
    expect(srss.change).toMatchObject({ change_kind: "delete_combination", field_path: "combinations", before: await canonicalJsonString(stored(start, "combination:S")), after: "not_present" });
    expect(byId("load-manager-combination-entity-delete-preview")).toHaveTextContent(`before=${await canonicalJsonString(stored(start, "combination:S"))}`);
    model = await apply(model, srss);
    panel.set(model);

    const expected: Array<[string, string]> = [
      ["combination:SUB-T", "combination:SUB-T; sub with empty terms; basis=result_state_subtraction; terms=none"],
      ["combination:SUB-N", await canonicalJsonString(stored(start, "combination:SUB-N"))],
      ["combination:ENV", await canonicalJsonString(stored(start, "combination:ENV"))],
      ["combination:M", "combination:M; flat mechanics; basis=mechanics; terms=load:L-100 x 1.5; load:L-200 x 0.0000001"]
    ];
    for (const [id, before] of expected) {
      const intent = await queueEntityDelete(panel, id);
      expect(intent.change.before).toBe(before);
      const next = await apply(model, intent);
      expect(ids(next)).toEqual(ids(model).filter((item) => item !== id));
      model = next;
      panel.set(model);
    }
    expect(ids(model)).toEqual(["combination:C-OPER-ALT"]);
    expect(await canonicalJsonString(model)).toBe(await canonicalJsonString(baseModel()));
  });

  it("deletes a term naming a combination (canonical JSON of the term) and a flat term (historical display)", async () => {
    let model = modelWith(SUB_T, SUB_N, SRSS, MECH);
    const panel = mountPanel(model);
    const nested = await queueTermDelete(panel, "combination:S", 1);
    expect(nested.change).toMatchObject({ change_kind: "delete_combination_term", field_path: "terms.1", before: await canonicalJsonString({ combination: "combination:SUB-N", factor: 0.5 }), after: "not_present" });
    model = await apply(model, nested);
    expect(stored(model, "combination:S").terms).toEqual([{ combination: "combination:SUB-T", factor: 1 }]);
    panel.set(model);
    const flat = await queueTermDelete(panel, "combination:M", 1);
    expect(flat.change).toMatchObject({ field_path: "terms.1", before: "load:L-200 x 0.0000001" });
    model = await apply(model, flat);
    expect(stored(model, "combination:M").terms).toEqual([{ load_case: "load:L-100", factor: 1.5 }]);
  });
});

describe("a referenced combination", () => {
  it("names every referencing path before the attempt, and the engine refuses the delete by name; nothing is deleted", async () => {
    const zero = { id: "combination:Z", label: "zero factor", basis: "mechanics", terms: [{ load_case: "load:L-100", factor: 1 }, { combination: "combination:SUB-T", factor: 0 }], provenance: "test" };
    const sub = { id: "combination:SUBX", label: "sub of a sub", basis: "result_state_subtraction", minuend_id: "combination:SUB-T", subtrahend_id: "load:L-100", terms: [], provenance: "test" };
    const env = { id: "combination:ENV2", label: "envelope of a sub", basis: "range_envelope", operand_ids: ["load:L-200", "combination:SUB-T"], mode: "max", terms: [], provenance: "test" };
    const model = modelWith(SUB_T, SUB_N, SRSS, zero, sub, env);
    const panel = mountPanel(model);
    fireEvent.click(byId("load-manager-combination-select-combination:SUB-T"));
    expect(byId("load-manager-combination-entity-delete-references")).toHaveTextContent(
      "Named by combination:ENV2.operand_ids.1, combination:S.terms.0, combination:SUBX.minuend_id, combination:Z.terms.1. The operation engine refuses deleting it while another combination names it (OP-COMBINATION-DELETE-REFERENCED); nothing is deleted with it. Delete top-down: delete or change combination:ENV2, combination:S, combination:SUBX, combination:Z first.");
    expect(byId("queue-delete-combination-intent")).toBeEnabled();
    fireEvent.click(byId("queue-delete-combination-intent"));
    const finding = await screen.findByTestId("load-manager-combination-entity-delete-engine-diagnostic-OP-COMBINATION-DELETE-REFERENCED");
    expect(finding).toHaveTextContent("Refused: OP-COMBINATION-DELETE-REFERENCED");
    expect(finding).toHaveTextContent("refers to combination:SUB-T, combination:ENV2.operand_ids.1, combination:S.terms.0, combination:SUBX.minuend_id, combination:Z.terms.1");
    expect(byId("load-manager-combination-entity-delete-engine-error")).toHaveTextContent("nothing was queued");
    expect(panel.queued).toEqual([]);
    expect(ids(panel.handle.model)).toEqual(ids(model));
  });

  it("deletes a three-axis static seismic template set top-down: the unsigned combination first", async () => {
    let model = baseModel();
    const start = clone(model);
    const draft = { ...emptySeismicTemplateDraft(model), baseId: "load:L-100", axisIds: { x: "load:EQ-X", y: "load:EQ-Y", z: "load:EQ-Z" }, basis: "srss" as const, factors: { x: "1", y: "1", z: "1" } };
    for (const intent of seismicTemplateIntents(model, draft)) model = await apply(model, intent);
    const panel = mountPanel(model);
    fireEvent.click(byId("load-manager-combination-select-combination:SEISMIC-X-300"));
    expect(byId("load-manager-combination-entity-delete-references")).toHaveTextContent(
      "Named by combination:SEISMIC-300.terms.0. The operation engine refuses deleting it while another combination names it (OP-COMBINATION-DELETE-REFERENCED); nothing is deleted with it. This is part of a three-axis static seismic template set: delete top-down: the unsigned combination first.");
    for (const id of ["combination:SEISMIC-300", "combination:SEISMIC-X-300", "combination:SEISMIC-Y-300", "combination:SEISMIC-Z-300"]) {
      const intent = await queueEntityDelete(panel, id);
      model = await apply(model, intent);
      panel.set(model);
    }
    expect(await canonicalJsonString(model)).toBe(await canonicalJsonString(start));
  });
});

/** The inverse create the engine admits (#115): the stored record's own JSON at its index, never display text. */
function restoreRecordIntent(model: PreviewModel, record: unknown, index: number): EditorOperationIntent {
  const intent = createCombinationIntent({ model, payload: record as Record_, rationale: "restore", sourceNote: "restore of a deleted record" });
  return { ...intent, change: { ...intent.change, field_path: `combinations[${index}]`, after: JSON.stringify(record) } };
}
function restoreTermIntent(model: PreviewModel, combinationId: string, term: unknown, index: number): EditorOperationIntent {
  const intent = createCombinationIntent({ model, payload: { id: combinationId }, rationale: "restore", sourceNote: "restore of a deleted term" });
  return { ...intent, operation_id: `${intent.operation_id}-term-${index}`, target: { object_type: "Combination", ref: combinationId },
    change: { ...intent.change, change_id: `${intent.change.change_id}-term-${index}`, change_kind: "create_combination_term", field_label: "Combination term", field_path: `terms.${index}`, after: JSON.stringify(term) } };
}

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}

describe("undo and the restore forms", () => {
  it("the inverse create_combination at combinations[i] and create_combination_term at terms.i restore the stored record and term at their index", async () => {
    const start = modelWith(SUB_T, SUB_N, SRSS, MECH);
    const panel = mountPanel(start);
    const index = ids(start).indexOf("combination:S");
    const record = clone(stored(start, "combination:S"));
    const deleted = await apply(start, await queueEntityDelete(panel, "combination:S"));
    const restored = await apply(deleted, restoreRecordIntent(deleted, record, index));
    expect(ids(restored)).toEqual(ids(start));
    expect(await canonicalJsonString(restored)).toBe(await canonicalJsonString(start));

    panel.set(start);
    const term = clone(stored(start, "combination:S").terms![0]);
    const withoutTerm = await apply(start, await queueTermDelete(panel, "combination:S", 0));
    const termRestored = await apply(withoutTerm, restoreTermIntent(withoutTerm, "combination:S", term, 0));
    expect(stored(termRestored, "combination:S").terms).toEqual(stored(start, "combination:S").terms);
    expect(await canonicalJsonString(termRestored)).toBe(await canonicalJsonString(start));
  });

  it("where create refuses the restore (a termless mechanics record, minuend_final), the refusal is named and session Undo still restores exactly", async () => {
    const termless = { id: "combination:T0", label: "termless mechanics", basis: "mechanics", provenance: "test" };
    const final = { id: "combination:MF", label: "minuend final", basis: "result_state_subtraction", minuend_id: "load:EQ-X", subtrahend_id: "load:L-100",
      difference_basis: "operating_minus_sustained", subtrahend_support_state: "minuend_final", provenance: "test" };
    for (const [record, code] of [[termless, "OP-CREATE-COMBINATION-PAYLOAD-INVALID"], [final, "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED"]] as const) {
      const start = modelWith(record);
      const index = ids(start).indexOf(record.id);
      const panel = mountPanel(start);
      const intent = await queueEntityDelete(panel, record.id);
      expect(intent.change.before).toBe(await canonicalJsonString(stored(start, record.id)));
      cleanup();
      const deleted = await apply(start, intent);
      const refused = await applyOutcome(deleted, restoreRecordIntent(deleted, clone(record), index));
      expect(refused.validation.application_status).toBe("blocked");
      expect(codes(refused)).toContain(code);

      // The session checkpoint restores the document exactly.
      const result = await session(start);
      const ops = () => result.current.operations;
      const hash = (await computeModelHash(result.current.model.model!))!.value;
      act(() => ops().handleQueueEditorIntent(intent));
      let applied = false;
      await act(async () => { applied = await ops().handleApplyIntent(ops().editorIntents[0]); });
      expect(applied).toBe(true);
      expect(ids(result.current.model.model!)).not.toContain(record.id);
      act(() => ops().handleUndoSessionModelEdit());
      expect((await computeModelHash(result.current.model.model!))!.value).toBe(hash);
      expect(stored(result.current.model.model!, record.id)).toEqual(record);
      cleanup();
      vi.restoreAllMocks();
    }
  });
});
