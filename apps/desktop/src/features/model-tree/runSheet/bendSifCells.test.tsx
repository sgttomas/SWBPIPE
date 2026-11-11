// Run-sheet bend SIF cells (RT-G2/RT-A3):
// B rows take an explicit directional SIF pair, both or neither, under the row's k
// source. The batch realizes the pair with one `bend_realization` after the bend's
// insertion, inside the same preview-gated batch, with `before` from the record
// projection. Invented S1 fixture data; the wasm engine applies every batch.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { canonicalJsonString, computeModelHash } from "../../../services/hashService";
import { applyOperationBatch, validateOperationBatch, type OperationBatch } from "../../../services/operationBatchService";
import { stampImportedBatch } from "../../../services/operationAuthorship";
import * as previewService from "../../../services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../../types";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import { batchRequiresPreview } from "../../toolkit/BatchReviewPanel";
import { BendRealizationForm, SIF_HINT, SIF_PAIR_REFUSAL } from "../../bend-authoring/BendAuthoringForms";
import { RunSheet, RUN_SHEET_TIMING } from "./RunSheet";
import { initialRunSheetState, reconcileRunSheet, type RunSheetState } from "./runSheetLedger";
import { modelIndexFor } from "../../workspace/modelIndex";
import {
  BEND_SIF_COLUMN, DEFAULT_NUMBERING, MESSAGES, assignRowNumber, initialIdState, modelIds, resolveRunSheetWithEngine,
  type ResolvedRunSheet, type RunSheetInput, type RunSheetRowInput,
} from "./runSheetModel";
import { buildRunSheetBatch } from "./runSheetBatch";
import { indexLookup, useRunSheetStore } from "./runSheetStore";
import { SHEET_COLUMNS, assignIds, cellView, setCell, sheetView, type SheetColumnKey } from "./runSheetView";

afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });
useModelledVisibility();

type FixtureCase = { id: string; sheet: RunSheetInput; expected_batch: OperationBatch };
const fixture = fixtureJson as unknown as { basis_model: PreviewModel; cases: FixtureCase[] };
const basis = () => structuredClone(fixture.basis_model);
const SOURCE = "invented SIF-cell sheet source";
const K_SOURCE = "invented k and SIF source (user entry)";
const CURVED = "curved_bend_macro_element";
const pair = (ip: number, op: number) => ({ sif_in_plane_user_value: { value: ip, unit: "1" }, sif_out_of_plane_user_value: { value: op, unit: "1" } });
/** F node:E, one bend run and its next run (the S1 single-corner geometry). */
const rows = (bend: Partial<RunSheetRowInput> = {}, next: Partial<RunSheetRowInput> = {}): RunSheetRowInput[] => [
  { key: "r1", kind: "F", node: "node:E" },
  { key: "r2", kind: "B", dx: "3", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1", bendRadius: "0.5", k: "1.5", kSource: K_SOURCE, ...bend },
  { key: "r3", dy: "3", ...next },
];
/** The two-bend S1 chain, with each bend row's entries overridden. */
const twoBends = (first: Partial<RunSheetRowInput>, second: Partial<RunSheetRowInput>): RunSheetRowInput[] => {
  const sheet = structuredClone(fixture.cases.find((item) => item.id === "two-bend-chain")!.sheet.rows);
  Object.assign(sheet[1], first);
  Object.assign(sheet[2], second);
  return sheet;
};
async function resolve(model: PreviewModel, sheetRows: RunSheetRowInput[], input: Partial<RunSheetInput> = {}): Promise<ResolvedRunSheet> {
  let idState = initialIdState(DEFAULT_NUMBERING);
  for (const row of sheetRows) idState = assignRowNumber(idState, DEFAULT_NUMBERING, row.key, modelIds(model));
  return (await resolveRunSheetWithEngine({ sheetKey: "sifs", batchSeq: 1, source: SOURCE, lengthUnit: "m", rows: sheetRows, ...input }, { model, idState })).resolved;
}
/** The sheet state the cells would hold for these rows (the commit path's own functions). */
function stateOf(model: PreviewModel, sheetRows: RunSheetRowInput[]): RunSheetState {
  let state: RunSheetState = { ...initialRunSheetState(model.project.id, "sifs", sheetRows.length), header: { ...initialRunSheetState(null).header, source: SOURCE, lengthUnit: "m" } };
  for (const row of sheetRows) {
    for (const [column, text] of Object.entries(row)) if (column !== "key") state = setCell(state, row.key, column as SheetColumnKey, text as string);
    state = { ...state, idState: assignRowNumber(state.idState, state.header.numbering, row.key, modelIds(model)) };
  }
  return state;
}
const shownCell = (model: PreviewModel, sheetRows: RunSheetRowInput[], rowKey: string, column: SheetColumnKey) => {
  const view = sheetView({ state: stateOf(model, sheetRows), model, statuses: new Map(), conversions: new Map(), reservedIds: [] });
  const row = view.rows.find((item) => item.sheetRow.key === rowKey)!;
  return cellView(view, row, SHEET_COLUMNS.find((item) => item.key === column)!, () => "m");
};
const codesOf = (outcome: Awaited<ReturnType<typeof applyOperationBatch>>) =>
  [...outcome.diagnostics, ...outcome.operation_outcomes.flatMap((step) => step.diagnostics)].filter((item) => item.severity === "blocking").map((item) => item.code);

describe("bend SIF cells: the resolver, the builder and the engine", () => {
  it("independent i_ip and i_op reach one bend_realization after the bend's insertion, with the realization editor's payload and the k source", async () => {
    const model = basis();
    const resolved = await resolve(model, rows({ sifInPlane: "2.1", sifOutOfPlane: "1.8" }));
    expect(resolved.rows.flatMap((row) => row.refusals)).toEqual([]);
    const bend = resolved.rows[1];
    expect(bend.bend!.sifs).toEqual(pair(2.1, 1.8));
    const build = await buildRunSheetBatch(resolved);
    const operations = build.batch.operations;
    const insert = operations.findIndex((op) => op.change.change_kind === "insert_bend_at_corner");
    const realize = operations[insert + 1];
    expect(operations.filter((op) => op.change.field_path === "bend_realization")).toEqual([realize]);
    expect(realize).toMatchObject({
      operation_kind: "modify", author_type: "user", target: { object_type: "Component", ref: bend.ids.bend!.component },
      change: { change_kind: "set_field", field_path: "bend_realization", unit: "none", dimension: "dimensionless" },
    });
    expect(realize.operation_id).toMatch(/^op:run-sheet:r2:\d+$/);
    expect(JSON.parse(realize.change.after)).toEqual({ solver_consumption: CURVED, flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: K_SOURCE, ...pair(2.1, 1.8) });
    // `before` is the inserted component's realization projection, as the engine stores it.
    expect(realize.change.before).toBe(await canonicalJsonString({ solver_consumption: CURVED, flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: K_SOURCE }));
    expect(batchRequiresPreview(build.batch)).toBe(true);
    const before = structuredClone(model);
    const outcome = await applyOperationBatch(model, build.batch, await computeModelHash(model));
    expect(outcome.validation.batch_validation_status, JSON.stringify(outcome.diagnostics)).toBe("passed");
    expect(model).toEqual(before);
    const stored = outcome.applied_model!.components.find((item) => item.id === bend.ids.bend!.component)!;
    expect(stored.modifiers).toEqual({ flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: K_SOURCE, ...pair(2.1, 1.8) });
    // The record projection holds the bend component exactly as the engine stored it.
    expect(build.projection.components.find((item) => item.id === stored.id)).toEqual(stored);

    // The two values are independent: swapped entries are stored swapped.
    const swapped = await buildRunSheetBatch(await resolve(basis(), rows({ sifInPlane: "1.8", sifOutOfPlane: "2.1" })));
    const swappedModel = (await applyOperationBatch(basis(), swapped.batch, await computeModelHash(basis()))).applied_model!;
    expect(swappedModel.components.find((item) => item.id === stored.id)!.modifiers).toMatchObject(pair(1.8, 2.1));
  });

  it("rows with no SIFs emit exactly the fixture's bend batches, byte for byte, with no bend_realization", async () => {
    for (const id of ["two-bend-chain", "three-bend-chain"]) {
      const item = fixture.cases.find((entry) => entry.id === id)!;
      const model = basis();
      const numbering = { ...DEFAULT_NUMBERING, ...(item.sheet.numbering ?? {}) };
      const idState = item.sheet.rows.reduce((state, row) => assignRowNumber(state, numbering, row.key, modelIds(model)), initialIdState(numbering));
      const { resolved } = await resolveRunSheetWithEngine(item.sheet, { model, idState });
      const build = await buildRunSheetBatch(resolved);
      expect(JSON.stringify(build.batch), id).toBe(JSON.stringify(item.expected_batch));
      expect(build.batch.operations.some((op) => op.change.field_path === "bend_realization")).toBe(false);
    }
  });

  it.each([
    ["2.1", ""], ["", "1.8"], ["0", "1.8"], ["2.1", "-1"], ["abc", "1.8"], ["2.1", "Infinity"], ["0x10", "1.8"], ["1e400", "1.8"], ["2.1", "1.8 m"],
  ])("a partial or invalid pair (%j, %j) stays in its cells and refuses the row by the helper's and the engine's name", async (ip, op) => {
    const sheet = rows({ sifInPlane: ip, sifOutOfPlane: op });
    const resolved = await resolve(basis(), sheet);
    expect(resolved.queueable).toBe(false);
    expect(resolved.rows[1].refusals).toContainEqual({ rule: "bend", column: BEND_SIF_COLUMN, message: MESSAGES.bendSifPair });
    expect(MESSAGES.bendSifPair).toBe(`OP-BEND-DIRECTIONAL-SIF-INCOMPLETE: ${SIF_PAIR_REFUSAL}`);
    expect(resolved.rows[1].bend?.sifs).toBeUndefined();
    await expect(buildRunSheetBatch(resolved)).rejects.toThrow();
    for (const [column, typed] of [["sifInPlane", ip], ["sifOutOfPlane", op]] as const) {
      const cell = shownCell(basis(), sheet, "r2", column);
      expect(cell.text).toBe(typed);
      expect(cell.refusal).toBe(MESSAGES.bendSifPair);
    }
  });

  it("the engine names the same rule for a pair the sheet would refuse", async () => {
    const model = basis();
    const build = await buildRunSheetBatch(await resolve(model, rows({ sifInPlane: "2.1", sifOutOfPlane: "1.8" })));
    const partial = structuredClone(build.batch);
    const realize = partial.operations.find((op) => op.change.field_path === "bend_realization")!;
    const after = JSON.parse(realize.change.after);
    delete after.sif_out_of_plane_user_value;
    realize.change.after = JSON.stringify(after);
    const outcome = await validateOperationBatch(model, partial, await computeModelHash(model));
    expect(codesOf(outcome)).toContain("OP-BEND-DIRECTIONAL-SIF-INCOMPLETE");
  });

  it("the SIFs share the row's k source: no second source cell, and SIFs without a k source refuse by name", async () => {
    expect(SHEET_COLUMNS.filter((column) => /SIF|source/i.test(column.label)).map((column) => column.key)).toEqual(["kSource", "sifInPlane", "sifOutOfPlane", "bendSource", "component.rigidScaleSourceReference", "component.geometrySourceReference"]);
    expect(SHEET_COLUMNS.find((column) => column.key === "sifInPlane")).toMatchObject({ label: "In-plane SIF i_ip", note: "shares k source" });
    const sheet = rows({ sifInPlane: "2.1", sifOutOfPlane: "1.8", kSource: "" });
    const resolved = await resolve(basis(), sheet);
    expect(resolved.queueable).toBe(false);
    expect(resolved.rows[1].refusals).toEqual(expect.arrayContaining([
      { rule: "bend", column: "k", message: MESSAGES.bendK },
      { rule: "bend", column: BEND_SIF_COLUMN, message: MESSAGES.bendSifSource },
    ]));
    const cell = shownCell(basis(), sheet, "r2", "sifInPlane");
    expect(cell.refusal).toBe(MESSAGES.bendSifSource);
    expect(cell.origin).toBe("entered; source: this row's k source");
    expect(shownCell(basis(), rows(), "r2", "sifOutOfPlane").origin).toBe("blank: none; source: this row's k source");
  });

  it("SIFs entered on a row that is not a bend are refused, not discarded", async () => {
    const sheet = rows({}, { sifInPlane: "2.1" });
    const resolved = await resolve(basis(), sheet);
    expect(resolved.rows[2].refusals).toContainEqual({ rule: "bend", column: BEND_SIF_COLUMN, message: MESSAGES.bendSifNotApplicable });
    expect(shownCell(basis(), sheet, "r3", "sifInPlane").text).toBe("2.1");
  });

  it("a duplicate or stored bend component id refuses both rows and builds nothing", async () => {
    const duplicate = await resolve(basis(), twoBends(
      { sifInPlane: "2.1", sifOutOfPlane: "1.8", ids: { bendComponent: "component:dup" } },
      { sifInPlane: "2.3", sifOutOfPlane: "1.9", ids: { bendComponent: "component:dup" } }));
    for (const row of duplicate.rows.slice(1, 3)) expect(row.refusals).toContainEqual(expect.objectContaining({ message: MESSAGES.idUsed("component:dup", "Bend component id") }));
    await expect(buildRunSheetBatch(duplicate)).rejects.toThrow();
    const stored = await resolve(basis(), rows({ sifInPlane: "2.1", sifOutOfPlane: "1.8", ids: { bendComponent: "node:A" } }));
    expect(stored.rows[1].refusals).toContainEqual(expect.objectContaining({ message: MESSAGES.idUsed("node:A", "Bend component id") }));
  });

  it("consecutive bends each take their own pair; a stale projected before is refused by the engine", async () => {
    const model = basis();
    const resolved = await resolve(model, twoBends({ sifInPlane: "2.1", sifOutOfPlane: "1.8" }, { sifInPlane: "2.3", sifOutOfPlane: "1.9" }));
    expect(resolved.rows.flatMap((row) => row.refusals)).toEqual([]);
    const build = await buildRunSheetBatch(resolved);
    const kinds = build.batch.operations.map((op) => op.change.field_path === "bend_realization" ? "realize" : op.change.change_kind);
    expect(kinds.filter((kind) => kind === "insert_bend_at_corner" || kind === "realize")).toEqual(["insert_bend_at_corner", "realize", "insert_bend_at_corner", "realize"]);
    kinds.forEach((kind, index) => { if (kind === "realize") expect(kinds[index - 1]).toBe("insert_bend_at_corner"); });
    const outcome = await applyOperationBatch(model, build.batch, await computeModelHash(model));
    expect(outcome.validation.batch_validation_status, JSON.stringify(outcome.diagnostics)).toBe("passed");
    const components = resolved.rows.slice(1, 3).map((row) => outcome.applied_model!.components.find((item) => item.id === row.ids.bend!.component)!);
    expect(components.map((item) => item.modifiers)).toEqual([
      { flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: resolved.rows[1].bend!.kSource, ...pair(2.1, 1.8) },
      { flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: resolved.rows[2].bend!.kSource, ...pair(2.3, 1.9) },
    ]);
    for (const component of components) expect(build.projection.components.find((item) => item.id === component.id)).toEqual(component);
    const stale = structuredClone(build.batch);
    stale.operations.find((op) => op.change.field_path === "bend_realization")!.change.before = "not_present";
    const refused = await applyOperationBatch(model, stale, await computeModelHash(model));
    expect(refused.applied_model).toBeNull();
    expect(codesOf(refused)).toContain("OP-BEFORE-VALUE-MISMATCH");
  });

  it("an agent cannot write them: the embedded agent's and the import's author are refused by name, and the model is unchanged", async () => {
    const model = basis();
    const build = await buildRunSheetBatch(await resolve(model, rows({ sifInPlane: "2.1", sifOutOfPlane: "1.8" })));
    const at = build.batch.operations.findIndex((op) => op.change.field_path === "bend_realization");
    // The state the realization step meets: every earlier step, applied as the user.
    const prefix = await applyOperationBatch(model, { batch_id: "batch:sif-prefix", operations: build.batch.operations.slice(0, at) }, await computeModelHash(model));
    const inserted = prefix.applied_model!;
    const realize = build.batch.operations[at];
    const routes: Array<[string, OperationBatch]> = [
      ["embedded agent", { batch_id: "batch:sif-agent", operations: [{ ...structuredClone(realize), author_type: "agent", source: { source_ref: "agent:turn", source_channel: "embedded_agent", source_role: "agent" } } as EditorOperationIntent] }],
      ["import", stampImportedBatch({ batch_id: "batch:sif-import", operations: [structuredClone(realize)] }).batch],
    ];
    for (const [route, batch] of routes) {
      expect(batch.operations[0].author_type, route).toBe("agent");
      const snapshot = structuredClone(inserted);
      const outcome = await applyOperationBatch(inserted, batch, await computeModelHash(inserted));
      expect(outcome.applied_model, route).toBeNull();
      expect(codesOf(outcome), route).toContain("OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED");
      expect(inserted).toEqual(snapshot);
    }
    // The same step as the user applies on that state.
    const user = await applyOperationBatch(inserted, { batch_id: "batch:sif-user", operations: [realize] }, await computeModelHash(inserted));
    expect(user.validation.batch_validation_status, JSON.stringify(user.diagnostics)).toBe("passed");
  });
});

describe("bend SIF cells in the grid and the workspace session", () => {
  type Current = { session: ReturnType<typeof useWorkspaceSession>; store: ReturnType<typeof useRunSheetStore> };
  async function mount(sheetRows: RunSheetRowInput[]) {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(basis());
    RUN_SHEET_TIMING.engineCheckDelayMs = 60_000;
    const current = {} as Current;
    function Harness() {
      const session = useWorkspaceSession();
      const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
      current.session = session; current.store = store;
      return session.model.model ? <RunSheet model={session.model.model} host={{ store, queuedBatches: session.operations.queuedBatches, editorIntents: session.operations.editorIntents, registerBatch: session.operations.registerOperationBatch }} /> : null;
    }
    render(<Harness />);
    await screen.findByTestId("run-sheet");
    const model = current.session.model.model!;
    act(() => current.store.update(() => {
      let next: RunSheetState = { ...initialRunSheetState(model.project.id, "sifs", sheetRows.length), header: { ...initialRunSheetState(null).header, source: SOURCE, lengthUnit: "m" } };
      for (const row of sheetRows) {
        for (const [column, text] of Object.entries(row)) if (column !== "key") next = setCell(next, row.key, column as SheetColumnKey, text as string);
        next = assignIds(next, row.key, model, []);
      }
      return next;
    }));
    return { current, model };
  }
  const cell = (rowKey: string, column: string) => screen.getByTestId(`run-sheet-cell-${rowKey}-${column}`);
  /** Type into a cell as a person does: a printable key opens the editor, Enter commits. */
  function type(rowKey: string, column: string, value: string, finish: "Enter" | "Escape" = "Enter") {
    const target = cell(rowKey, column);
    act(() => target.focus());
    fireEvent.keyDown(target, { key: value[0] });
    const editor = within(target).getByRole("textbox");
    fireEvent.change(editor, { target: { value } });
    fireEvent.keyDown(editor, { key: finish });
  }

  it("enters, cancels, clears and refuses the pair in labelled cells; Queue, preview, Apply, Undo and Redo; an inspector edit detaches the row", async () => {
    const { current, model } = await mount(rows());
    expect(screen.getByTestId("run-sheet-sif-note")).toHaveTextContent(SIF_HINT);
    expect(screen.getByTestId("run-sheet-sif-note")).toHaveTextContent("their source is the row's k source");
    expect(screen.getByRole("columnheader", { name: /In-plane SIF i_ip \(shares k source\)/ })).toBeInTheDocument();
    type("r2", "sifInPlane", "2.1");
    expect(cell("r2", "sifInPlane")).toHaveAccessibleName("Row 2 In-plane SIF i_ip: 2.1");
    expect(cell("r2", "sifInPlane")).toHaveAccessibleDescription(/^entered; source: this row's k source/);
    // Partial: refused in both cells by name, the typed value kept, Queue disabled.
    expect(cell("r2", "sifInPlane")).toHaveAccessibleDescription(new RegExp(`refused: ${MESSAGES.bendSifPair.replace(/[().]/g, "\\$&")}`));
    expect(cell("r2", "sifOutOfPlane")).toHaveAccessibleDescription(/refused: OP-BEND-DIRECTIONAL-SIF-INCOMPLETE/);
    expect(screen.getByTestId("run-sheet-queue")).toBeDisabled();
    expect(screen.getByTestId("run-sheet-queue-reason")).toHaveTextContent("Refused rows: 2.");
    // Escape cancels an edit and changes nothing.
    type("r2", "sifOutOfPlane", "9", "Escape");
    expect(cell("r2", "sifOutOfPlane")).toHaveTextContent(/^$/);
    type("r2", "sifOutOfPlane", "1.8");
    expect(cell("r2", "sifOutOfPlane")).not.toHaveAccessibleDescription(/refused/);
    // Delete clears a cell; the other stays and the row is refused again.
    act(() => cell("r2", "sifInPlane").focus());
    fireEvent.keyDown(cell("r2", "sifInPlane"), { key: "Delete" });
    expect(cell("r2", "sifInPlane")).toHaveTextContent(/^$/);
    expect(cell("r2", "sifOutOfPlane")).toHaveTextContent("1.8");
    expect(cell("r2", "sifOutOfPlane")).toHaveAccessibleDescription(/refused: OP-BEND-DIRECTIONAL-SIF-INCOMPLETE/);
    type("r2", "sifInPlane", "2.1");
    const queue = screen.getByTestId("run-sheet-queue");
    await waitFor(() => expect(queue).toBeEnabled());
    await act(async () => { fireEvent.click(queue); });
    await waitFor(() => expect(current.session.operations.queuedBatches).toHaveLength(1));
    const entry = current.session.operations.queuedBatches[0];
    expect(entry.batch.operations.filter((op) => op.change.field_path === "bend_realization")).toHaveLength(1);
    // Preview-gated: Apply before a clean preview of this exact batch changes nothing.
    await act(async () => { await current.session.operations.handleRunOperationBatch(entry, true); });
    expect(current.session.model.model).toEqual(model);
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], false); });
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], true); });
    expect(current.session.operations.undoStack).toHaveLength(1);
    expect(current.store.statuses.get("r2")?.status).toBe("applied");
    const componentId = current.store.state.ledger.records[0].rowEntries.r2.ownedIds.find((ref) => ref.type === "component")!.id;
    const appliedComponent = current.session.model.model!.components.find((item) => item.id === componentId)!;
    expect(appliedComponent.modifiers).toEqual({ flexibility_factor_user_value: { value: 1.5, unit: "none" }, source_reference: K_SOURCE, ...pair(2.1, 1.8) });
    // The SIFs join the row's expected owned records; the applied cells read them back from the component.
    expect(current.store.state.ledger.records[0].rowEntries.r2.expected.find((item) => item.ref.id === componentId)!.record).toEqual(appliedComponent);
    expect(cell("r2", "sifInPlane")).toHaveTextContent("2.1");
    expect(cell("r2", "sifInPlane")).toHaveAccessibleDescription(/^stored component value; source: this row's k source/);
    expect(cell("r2", "sifInPlane")).toHaveAttribute("aria-readonly", "true");
    act(() => current.session.operations.handleUndoSessionModelEdit());
    expect(current.session.model.model).toEqual(model);
    expect(current.store.statuses.get("r2")?.status).toBe("draft");
    expect(cell("r2", "sifOutOfPlane")).toHaveTextContent("1.8");
    expect(cell("r2", "sifOutOfPlane")).toHaveAccessibleDescription(/^entered/);
    act(() => current.session.operations.handleRedoSessionModelEdit());
    expect(current.store.statuses.get("r2")?.status).toBe("applied");
    expect(current.store.state.rows.find((row) => row.key === "r2")!.edits).toEqual({});
    expect(current.session.model.model!.components.find((item) => item.id === componentId)).toEqual(appliedComponent);

    // Applied records are preserved: a later sheet batch leaves the bend's stored members alone.
    act(() => current.store.update((state) => {
      const added = { ...state, rows: [...state.rows, { key: "r4", edits: {}, restored: {}, pending: null, requeueNeeded: false, engineRefusal: null }], nextRowKey: state.nextRowKey + 1 };
      return assignIds(setCell(added, "r4", "dx", "2"), "r4", current.session.model.model!, []);
    }));
    await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
    await act(async () => { fireEvent.click(screen.getByTestId("run-sheet-queue")); });
    await waitFor(() => expect(current.session.operations.queuedBatches).toHaveLength(1));
    expect(current.session.operations.queuedBatches[0].batch.operations.some((op) => op.change.field_path === "bend_realization")).toBe(false);
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], true); });
    expect(current.store.statuses.get("r4")?.status).toBe("applied");
    expect(current.store.statuses.get("r2")?.status).toBe("applied");
    expect(current.session.model.model!.components.find((item) => item.id === componentId)).toEqual(appliedComponent);

    // The inspector's realization editor changes i_ip: the row reads "Changed outside the sheet".
    const queued = vi.fn();
    render(<BendRealizationForm model={current.session.model.model!} selection={{ type: "component", id: componentId }} component={appliedComponent} onQueueIntent={queued} />);
    fireEvent.change(screen.getByLabelText("Bend in-plane SIF i_ip"), { target: { value: "2.5" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue bend realization update" }));
    await waitFor(() => expect(queued).toHaveBeenCalledOnce());
    await act(async () => { await current.session.operations.handleApplyIntent(queued.mock.calls[0][0] as EditorOperationIntent); });
    expect(current.session.model.model!.components.find((item) => item.id === componentId)!.modifiers, current.session.operations.operationMessage ?? "").toMatchObject(pair(2.5, 1.8));
    expect(current.store.statuses.get("r2")?.status).toBe("detached");
    expect(current.store.statuses.get("r2")?.changed).toContain(componentId);
    expect(screen.getByTestId("run-sheet-detached-r2")).toHaveTextContent(`Changed outside the sheet: ${componentId}`);
    expect(cell("r2", "sifInPlane")).toHaveTextContent("2.5");
    // Undo of that outside edit restores the expected record exactly: the row is applied again.
    act(() => current.session.operations.handleUndoSessionModelEdit());
    expect(current.store.statuses.get("r2")?.status).toBe("applied");
    expect(cell("r2", "sifInPlane")).toHaveTextContent("2.1");
    // A bend component deleted elsewhere: the row is detached and its SIF cells say so, showing no value.
    const missing = structuredClone(current.session.model.model!);
    missing.components = missing.components.filter((item) => item.id !== componentId);
    const lookup = indexLookup(modelIndexFor(missing, 0, 0));
    const sheet = current.store.current();
    const missingView = sheetView({ state: sheet, model: missing, statuses: reconcileRunSheet(sheet, lookup), lookup, conversions: new Map(), reservedIds: [] });
    const missingRow = missingView.rows.find((row) => row.sheetRow.key === "r2")!;
    expect(missingRow.status).toBe("detached");
    const missingCell = cellView(missingView, missingRow, SHEET_COLUMNS.find((column) => column.key === "sifInPlane")!, () => "m");
    expect([missingCell.text, missingCell.origin]).toEqual(["", `missing: component ${componentId} no longer exists; source: this row's k source`]);
  }, 60_000);

  it("Fill down copies an entered SIF into the next bend row as an entered value; a blank cell never inherits", async () => {
    await mount(twoBends({ sifInPlane: "2.1", sifOutOfPlane: "1.8" }, {}));
    expect(cell("r3", "sifInPlane")).toHaveTextContent(/^$/);
    expect(cell("r3", "sifInPlane")).toHaveAccessibleDescription(/^blank: none/);
    act(() => cell("r2", "sifInPlane").focus());
    fireEvent.keyDown(cell("r2", "sifInPlane"), { key: "d", ctrlKey: true });
    expect(cell("r3", "sifInPlane")).toHaveTextContent("2.1");
    expect(cell("r3", "sifInPlane")).toHaveAccessibleDescription(/^entered/);
    // One entered value is a partial pair on that row: refused until the other is entered.
    expect(cell("r3", "sifOutOfPlane")).toHaveAccessibleDescription(/refused: OP-BEND-DIRECTIONAL-SIF-INCOMPLETE/);
  });
});
