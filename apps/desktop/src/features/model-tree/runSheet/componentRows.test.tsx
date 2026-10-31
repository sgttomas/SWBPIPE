import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../../fixtures/reference_system/reference_system.json";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { computeModelHash } from "../../../services/hashService";
import { applyOperationBatch } from "../../../services/operationBatchService";
import { createLocalProject, openLocalProject } from "../../../services/projectService";
import * as previewService from "../../../services/previewService";
import type { PreviewModel } from "../../../types";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import { batchRequiresPreview } from "../../toolkit/BatchReviewPanel";
import { modelIndexFor } from "../../workspace/modelIndex";
import { inspectorAuthoringPaths } from "../PropertyInspector";
import { RunSheet, RUN_SHEET_TIMING } from "./RunSheet";
import { initialRunSheetState, reconcileRunSheet } from "./runSheetLedger";
import { DEFAULT_NUMBERING, assignRowNumber, initialIdState, modelIds, resolveRunSheetWithEngine, type RunSheetInput, type RunSheetRowInput } from "./runSheetModel";
import { buildRunSheetBatch } from "./runSheetBatch";
import { indexLookup, useRunSheetStore } from "./runSheetStore";
import { SHEET_COLUMNS, assignIds, cellView, setCell, sheetView } from "./runSheetView";
import { COMPONENT_FIELDS, componentColumn, type ComponentRowInput } from "./runSheetComponents";

afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });
useModelledVisibility();
const basis = () => structuredClone((fixtureJson as unknown as { basis_model: PreviewModel }).basis_model);
const SOURCE = "invented row source";
const component = (overrides: ComponentRowInput = {}): ComponentRowInput => ({
  id: "component:sheet-valve", label: "Sheet valve", rigidBodyLength: "1", endASize: "100", endASizeUnit: "mm", endBSize: "80", endBSizeUnit: "mm", weight: "100",
  centerOfGravityX: "-0.5", centerOfGravityY: "0", centerOfGravityZ: "0", connectionEndAReference: "entered end A", connectionEndBReference: "entered end B", stiffnessBehaviorReference: "entered behavior",
  geometrySourceReference: "entered geometry source", provenance: "invented component", ...overrides
});
const input = (rows: RunSheetRowInput[]): RunSheetInput => ({ sheetKey: "component-rows", batchSeq: 1, source: SOURCE, lengthUnit: "m", rows });
const chain = (values = component()): RunSheetRowInput[] => [
  { key: "r1", kind: "F", node: "node:E", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
  { key: "r2", kind: "V", dx: "1", component: values }
];
async function resolved(model: PreviewModel, rows: RunSheetRowInput[], reservedIds: string[] = []) {
  let idState = initialIdState(DEFAULT_NUMBERING);
  for (const row of rows) idState = assignRowNumber(idState, DEFAULT_NUMBERING, row.key, modelIds(model));
  return (await resolveRunSheetWithEngine(input(rows), { model, idState, reservedIds })).resolved;
}

describe("component runs through the existing operation engine", () => {
  it("recreates the reference valve and two flanges in the sheet, preserving every component value/unit through save and reopen", async () => {
    const model = structuredClone(referenceJson) as unknown as PreviewModel;
    model.diagnostics = [];
    const names = ["RS-F1", "RS-V1", "RS-F2"];
    const expected = names.map((name) => model.components.find((item) => item.id === `component:${name}`)!);
    model.components = model.components.filter((item) => !expected.includes(item));
    model.pipe_segments = model.pipe_segments.filter((item) => !["pipe:RS-P020", "pipe:RS-P030", "pipe:RS-P040"].includes(item.id));
    const rows: RunSheetRowInput[] = [{ key: "r1", kind: "F", node: "node:RS-020", section: "section:RS-S270", material: "material:RS-M1", orientation: "0 0 1" }];
    const values = [
      { kind: "L", node: "030", pipe: "020", body: "0.15", weight: "350", cog: ["-0.075", "0", "0"], ends: ["020", "030"] },
      { kind: "V", node: "040", pipe: "030", body: "0.6", weight: "2600", cog: ["-0.3", "0", "0.3"], ends: ["030", "040"] },
      { kind: "L", node: "050", pipe: "040", body: "0.15", weight: "350", cog: ["-0.075", "0", "0"], ends: ["040", "050"] },
    ];
    values.forEach((value, index) => rows.push({
      key: `r${index + 2}`, kind: value.kind, node: `node:RS-${value.node}`, ids: { pipe: `pipe:RS-P${value.pipe}` },
      component: component({ id: expected[index].id, label: expected[index].label, rigidBodyLength: value.body, endASize: "270", endBSize: "270", weight: value.weight,
        centerOfGravityX: value.cog[0], centerOfGravityY: value.cog[1], centerOfGravityZ: value.cog[2],
        connectionEndAReference: `node:RS-${value.ends[0]} (body end on RS-P${value.pipe})`, connectionEndBReference: `node:RS-${value.ends[1]} (body end on RS-P${value.pipe})`,
        stiffnessBehaviorReference: expected[index].geometry!.stiffness_behavior_reference!, geometrySourceReference: expected[index].geometry!.rigid_component_source_reference!, provenance: expected[index].provenance,
        rigidStiffnessScale: "10", rigidScaleSourceReference: expected[index].modifiers!.source_reference!, rigidSolverConsumption: "mechanics_geometry_only" })
    }));
    const result = await resolved(model, rows);
    expect(result.rows.flatMap((row) => row.refusals)).toEqual([]);
    const built = await buildRunSheetBatch(result);
    const created = built.batch.operations.filter((op) => op.change.change_kind === "insert_component_symbol");
    expect(created.map((op) => JSON.parse(op.change.after))).toEqual(expected);
    expect(created.every((op) => op.author_type === "user")).toBe(true);
    const before = structuredClone(model);
    const applied = await applyOperationBatch(model, built.batch, await computeModelHash(model));
    expect(applied.validation.batch_validation_status, JSON.stringify(applied)).toBe("passed");
    expect(model).toEqual(before);
    await createLocalProject(applied.applied_model!);
    const reopened = await openLocalProject(model.project.id);
    expect(names.map((name) => reopened!.model.components.find((item) => item.id === `component:${name}`))).toEqual(expected);
  });

  it.each([
    ["centerOfGravityX", "", "COG X"], ["rigidBodyLength", "0", "Body length"], ["endASizeUnit", "s", "End A size unit"],
    ["rigidScaleSourceReference", "source without scale", "Stiffness scale"], ["rigidSolverConsumption", "unknown", "Solver consumption"]
  ] as const)("refuses %s by field name without supplying a value", async (field, value, label) => {
    const result = await resolved(basis(), chain(component({ [field]: value })));
    expect(result.queueable).toBe(false);
    expect(result.rows[1].refusals.map((item) => item.message).join(" ")).toContain(label);
  });

  it("reserves component identities against both model records and other queued operations", async () => {
    const model = basis();
    for (const id of [model.nodes[0].id, "component:queued"]) {
      const result = await resolved(model, chain(component({ id })), ["component:queued"]);
      expect(result.rows[1].refusals.some((item) => item.rule === "ids" && item.message.includes(id))).toBe(true);
    }
  });

  it("does not inherit engineering inputs or silently discard them after a kind change", async () => {
    const model = basis();
    const rows = [...chain(), { key: "r3", kind: "L", dx: "1" }];
    const result = await resolved(model, rows);
    expect(result.rows[2].component!.rigidBodyLength).toBe("");
    expect(result.rows[2].component!.centerOfGravityX).toBe("");
    expect(result.queueable).toBe(false);
    const changed = chain(); changed[1].kind = "R";
    expect((await resolved(model, changed)).rows[1].refusals.some((item) => item.rule === "component")).toBe(true);
  });

  it("names duplicate component ids on both rows and requires a force unit when the project supplies none", async () => {
    const model = basis();
    const rows = [...chain(), { key: "r3", kind: "L", dx: "1", component: component() }];
    const duplicate = await resolved(model, rows);
    for (const row of duplicate.rows.slice(1)) expect(row.refusals.some((item) => item.rule === "ids" && item.message.includes("component:sheet-valve"))).toBe(true);
    delete (model.project.units as Record<string, string>).force;
    expect((await resolved(model, chain())).rows[1].refusals.some((item) => item.column === "Weight unit")).toBe(true);
    expect((await resolved(model, chain(component({ forceUnit: "N" })))).queueable).toBe(true);
  });

  it("reports absent project units without substituting engineering values or throwing", async () => {
    const model = basis();
    delete (model.project as unknown as Record<string, unknown>).units;
    const result = await resolved(model, chain());
    expect(result.queueable).toBe(false);
    expect(result.blockers.some((item) => item.rule === "length-unit")).toBe(true);
    expect(result.rows[1].component!.lengthUnit).toBe("");
    expect(result.rows[1].refusals.some((item) => item.column === "Body length unit")).toBe(true);
  });

  it.each(["V", "L"])("realizes preceding bends before attaching a %s component to their outgoing straight", async (kind) => {
    const fixture = fixtureJson as unknown as { cases: Array<{ id: string; sheet: { rows: RunSheetRowInput[] } }> };
    const rows = structuredClone(fixture.cases.find((item) => item.id === "two-bend-chain")!.sheet.rows);
    Object.assign(rows.at(-1)!, { kind, component: component({ rigidBodyLength: "2.5", centerOfGravityX: "0", centerOfGravityZ: "-1.25" }) });
    const model = basis();
    const sheet = await resolved(model, rows);
    expect(sheet.rows.flatMap((row) => row.refusals)).toEqual([]);
    const built = await buildRunSheetBatch(sheet);
    const kinds = built.batch.operations.map((op) => op.change.change_kind);
    expect(kinds.lastIndexOf("insert_bend_at_corner")).toBeLessThan(kinds.indexOf("insert_component_symbol"));
    expect(batchRequiresPreview(built.batch)).toBe(true);
    const applied = await applyOperationBatch(model, built.batch, await computeModelHash(model));
    expect(applied.validation.batch_validation_status, JSON.stringify(applied.diagnostics)).toBe("passed");
    expect(applied.applied_model!.components.find((item) => item.id === "component:sheet-valve")!.kind).toBe(kind === "V" ? "valve" : "flange");
  });
});

describe("component rows in the workspace and accessible grid", () => {
  it("queues an explicit row from the keyboard, applies once, tracks ownership, and restores it with Undo/Redo", async () => {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(basis());
    RUN_SHEET_TIMING.engineCheckDelayMs = 60_000;
    let current: { session: ReturnType<typeof useWorkspaceSession>; store: ReturnType<typeof useRunSheetStore> };
    function Harness() {
      const session = useWorkspaceSession();
      const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
      current = { session, store };
      return session.model.model ? <RunSheet model={session.model.model} host={{ store, queuedBatches: session.operations.queuedBatches, editorIntents: session.operations.editorIntents, registerBatch: session.operations.registerOperationBatch }} /> : null;
    }
    render(<Harness />);
    await screen.findByTestId("run-sheet");
    const model = current!.session.model.model!;
    act(() => current!.store.update((state) => {
      let next = { ...initialRunSheetState(model.project.id, "components", 2), header: { ...state.header, source: SOURCE, lengthUnit: "m" } };
      for (const row of chain(component())) {
        for (const [key, value] of Object.entries(row)) if (key !== "key" && key !== "component") next = setCell(next, row.key, key as never, value as string);
        // Leave the component inputs empty: they are entered through real cells below.
        next = assignIds(next, row.key, model, []);
      }
      return next;
    }));
    fireEvent.click(screen.getByRole("button", { name: /Component details/ }));
    const cell = (column: string) => screen.getByTestId(`run-sheet-cell-r2-${column}`);
    const code = cell("kind");
    fireEvent.keyDown(code, { key: "l" }); expect(code).toHaveTextContent("L");
    fireEvent.keyDown(code, { key: "v" }); expect(code).toHaveTextContent("V");
    const entered = component();
    for (const field of COMPONENT_FIELDS) {
      const value = entered[field.key]; if (value === undefined || value === "") continue;
      const target = cell(componentColumn(field.key));
      act(() => target.focus()); fireEvent.keyDown(target, { key: value[0] });
      const editor = within(target).getByRole("textbox");
      fireEvent.change(editor, { target: { value } }); fireEvent.keyDown(editor, { key: "Tab" });
    }
    expect(cell("component.lengthUnit")).toHaveTextContent("m");
    expect(cell("component.lengthUnit")).toHaveAccessibleDescription(/derived: the project unit/);
    const queue = screen.getByRole("button", { name: /^Queue rows$/ });
    await waitFor(() => expect(queue).toBeEnabled());
    await act(async () => { fireEvent.click(queue); });
    await waitFor(() => expect(current!.session.operations.queuedBatches).toHaveLength(1));
    const entry = current!.session.operations.queuedBatches[0];
    expect(current!.session.model.model).toEqual(model);
    await act(async () => { await current!.session.operations.handleRunOperationBatch(entry, true); });
    expect(current!.session.operations.undoStack).toHaveLength(1);
    expect(current!.store.statuses.get("r2")?.status).toBe("applied");
    expect(current!.store.state.ledger.records[0].rowEntries.r2.ownedIds).toContainEqual({ type: "component", id: "component:sheet-valve" });
    expect(current!.session.model.model!.components.some((item) => item.id === "component:sheet-valve")).toBe(true);
    act(() => current!.session.operations.handleUndoSessionModelEdit());
    expect(current!.session.model.model).toEqual(model);
    expect(current!.store.statuses.get("r2")?.status).toBe("draft");
    act(() => current!.session.operations.handleRedoSessionModelEdit());
    expect(current!.store.statuses.get("r2")?.status).toBe("applied");
    expect(current!.store.state.rows.find((row) => row.key === "r2")!.edits).toEqual({});
    expect(cell("component.endASizeUnit")).toHaveTextContent("mm");
    expect(cell("component.rigidBodyLength")).toHaveAttribute("aria-readonly", "true");
    const after = current!.session.model.model!;
    const foreign = inspectorAuthoringPaths.buildOperationIntent({
      field: { label: "Weight", fieldPath: "geometry.weight.value", before: "100", objectType: "Component", changeKind: "set_field", dimension: "force", unit: "N", sourceNote: "entered weight", unitEditable: true, valueKind: "quantity" },
      model: after, proposedValue: "200", proposedUnit: "N", rationale: "entered weight", selection: { type: "component", id: "component:sheet-valve" }, unitCatalogRoute: null
    });
    await act(async () => { await current!.session.operations.handleApplyIntent(foreign); });
    expect(current!.session.model.model!.components.find((item) => item.id === "component:sheet-valve")!.geometry!.weight!.value, current!.session.operations.operationMessage ?? "").toBe(200);
    expect(current!.store.statuses.get("r2")?.status).toBe("detached");
    expect(cell("component.weight")).toHaveTextContent("200");
    expect(cell("component.forceUnit")).toHaveTextContent("N");
    const sheet = current!.store.current();
    const missing = structuredClone(current!.session.model.model!);
    missing.components = missing.components.filter((item) => item.id !== "component:sheet-valve");
    const lookup = indexLookup(modelIndexFor(missing, 0, 0));
    const missingView = sheetView({ state: sheet, model: missing, statuses: reconcileRunSheet(sheet, lookup), lookup, conversions: new Map(), reservedIds: [] });
    const missingRow = missingView.rows.find((row) => row.sheetRow.key === "r2")!;
    expect(missingRow.status).toBe("detached");
    const missingCell = cellView(missingView, missingRow, SHEET_COLUMNS.find((column) => column.key === "component.weight")!, () => "m");
    expect(missingCell.text).toBe("");
    expect(missingCell.origin).toContain("missing: component component:sheet-valve");
    act(() => cell("component.weight").focus());
    const toggle = screen.getByRole("button", { name: "Component details" });
    act(() => toggle.focus());
    fireEvent.click(toggle);
    expect(toggle).toHaveFocus();
    expect(cell("pipeId")).toHaveAttribute("tabindex", "0");
    expect(screen.queryByTestId("run-sheet-cell-r2-component.weight")).toBeNull();
  });
});
