// Run sheet S2 measurement (DEL-07-02 Design `run-sheet-entry.md` §3.6 Cost): a
// 500-row sheet (50 bend rows) on an invented model of more than 5,000
// entities, over 100 model revisions. It reports the rows, expected ids and
// model entities; the median and 95th-percentile reconciliation time, with the
// host; the lookup count, asserted equal to the number of expected ids; and,
// separately, the median and 95th-percentile cold `buildModelIndex` time for
// the same model. A second test counts `buildModelIndex` calls per revision in
// the workspace session with the sheet mounted and without it, and asserts
// them equal. Timings are indicative: this is an ordinary test run on a shared
// host, not an exclusive measurement, and nothing is asserted about them.
import { act, render, waitFor } from "@testing-library/react";
import { cpus, platform } from "node:os";
import { afterEach, describe, expect, it, vi } from "vitest";

const builds = vi.hoisted(() => ({ count: 0 }));
vi.mock("../../workspace/modelIndex", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../workspace/modelIndex")>();
  // The same cache rule as `modelIndexFor` (one index per model object and
  // generation key), around a counted `buildModelIndex`.
  const cache = new WeakMap<object, { generation: string; index: ReturnType<typeof actual.buildModelIndex> }>();
  const buildModelIndex: typeof actual.buildModelIndex = (...args) => { builds.count += 1; return actual.buildModelIndex(...args); };
  const modelIndexFor: typeof actual.modelIndexFor = (model, sessionGeneration, modelRevision) => {
    const generation = `${sessionGeneration}:${modelRevision}`;
    const existing = cache.get(model);
    if (existing?.generation === generation) return existing.index;
    const index = buildModelIndex(model, sessionGeneration, modelRevision);
    cache.set(model, { generation, index });
    return index;
  };
  return { ...actual, buildModelIndex, modelIndexFor };
});

import { buildModelIndex } from "../../workspace/modelIndex";
import * as previewService from "../../../services/previewService";
import { buildGridOperationIntent, type GridColumn } from "../../workspace/table/modelTableAdapter";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import type { PreviewModel } from "../../../types";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { applySessionEvent, initialRunSheetState, reconcileRunSheet, blankRow, type OwnedRef, type RunSheetState } from "./runSheetLedger";
import { indexLookup, useRunSheetStore } from "./runSheetStore";
import { RunSheet } from "./RunSheet";

afterEach(() => { vi.restoreAllMocks(); });
const fixture = fixtureJson as unknown as { basis_model: PreviewModel };

const ROWS = 500;
const BEND_ROWS = 50;
const MATERIAL = { id: "material:steel" }; // a material of the fixture basis
const section = { outside_diameter: { value: 0.1143, unit: "m" }, wall_thickness: { value: 0.00602, unit: "m" } };

/** An invented model: a base of 5,100 entities, plus every entity the 500 sheet rows own. */
function measuredModel(): { model: PreviewModel; owned: Map<string, OwnedRef[]> } {
  const nodes: PreviewModel["nodes"] = [];
  const pipes: PreviewModel["pipe_segments"] = [];
  const supports: NonNullable<PreviewModel["supports"]> = [];
  const components: PreviewModel["components"] = [];
  const node = (id: string, x: number) => nodes.push({ id, label: id, position: { x, y: 0, z: 0 }, provenance: "invented" } as never);
  const pipe = (id: string, from: string, to: string) => pipes.push({ id, label: id, from, to, section, material: MATERIAL.id, y_reference: { x: 0, y: 0, z: 1 }, provenance: "invented" } as never);
  for (let i = 0; i < 2500; i += 1) node(`node:base-${i}`, i);
  for (let i = 0; i < 2000; i += 1) pipe(`pipe:base-${i}`, `node:base-${i}`, `node:base-${i + 1}`);
  for (let i = 0; i < 600; i += 1) supports.push({ id: `support:base-${i}`, label: `S${i}`, node: `node:base-${i}`, family: "anchor", restraints: ["UX"], provenance: "invented" } as never);
  const owned = new Map<string, OwnedRef[]>();
  for (let row = 0; row < ROWS; row += 1) {
    const key = `r${row + 1}`;
    if (row < ROWS - BEND_ROWS) {
      node(`node:S${row}`, 10_000 + row);
      pipe(`pipe:S${row}`, row === 0 ? "node:base-0" : `node:S${row - 1}`, `node:S${row}`);
      supports.push({ id: `support:S${row}`, label: `S${row}`, node: `node:S${row}`, family: "guide", restraints: ["UY"], provenance: "invented" } as never);
      owned.set(key, [{ type: "node", id: `node:S${row}` }, { type: "pipe", id: `pipe:S${row}` }, { type: "support", id: `support:S${row}` }]);
    } else {
      node(`node:B${row}-T1`, 20_000 + row);
      node(`node:B${row}-T2`, 20_000.5 + row);
      pipe(`pipe:B${row}`, "node:base-1", `node:B${row}-T1`);
      pipe(`pipe:B${row}-bend`, `node:B${row}-T1`, `node:B${row}-T2`);
      components.push({ id: `component:B${row}-bend`, kind: "bend", node: `node:B${row}-T1`, provenance: "invented" } as never);
      owned.set(key, [{ type: "pipe", id: `pipe:B${row}` }, { type: "node", id: `node:B${row}-T1` }, { type: "node", id: `node:B${row}-T2` }, { type: "pipe", id: `pipe:B${row}-bend` }, { type: "component", id: `component:B${row}-bend` }]);
    }
  }
  // A valid model document: the S1 fixture's basis (its project, units, materials and sections), with these collections.
  const model = { ...structuredClone(fixture.basis_model), nodes, pipe_segments: pipes, supports, components } as PreviewModel;
  return { model, owned };
}

/** Every row applied by ten sheet checkpoints of 50 rows each, expectations from the model. */
function appliedSheet(model: PreviewModel, owned: Map<string, OwnedRef[]>, rows = ROWS): RunSheetState {
  let state: RunSheetState = { ...initialRunSheetState(model.project.id), rows: Array.from({ length: rows }, (_, index) => blankRow(`r${index + 1}`)) };
  for (let batch = 0; batch * 50 < rows; batch += 1) {
    const keys = state.rows.slice(batch * 50, batch * 50 + 50).map((row) => row.key);
    const batchId = `batch:run-sheet-measure-${batch + 1}`;
    state = {
      ...state,
      submitted: { ...state.submitted, [batchId]: { batchId, batch: { batch_id: batchId, operations: [] }, rows: keys.map((key) => ({ key, operationIds: [], ownedIds: owned.get(key)!.map((ref) => ref.id) })), ownedRefs: Object.fromEntries(keys.map((key) => [key, owned.get(key)!])), values: Object.fromEntries(keys.map((key) => [key, { dx: "1" }])) } },
      rows: state.rows.map((row) => keys.includes(row.key) ? { ...row, pending: { batchId, registrationKey: "k", action: "create" } } : row),
    };
    state = applySessionEvent(state, { type: "checkpoint", operationId: batchId, appliedModel: model, operationOutcomes: [] });
  }
  return state;
}
const entityCount = (model: PreviewModel) => model.nodes.length + model.pipe_segments.length + (model.supports ?? []).length + model.components.length + (model.materials ?? []).length + (model.sections ?? []).length;
const quantile = (values: number[], q: number) => {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.ceil(q * sorted.length) - 1)];
};

describe("run sheet reconciliation cost (§3.6), indicative", () => {
  it("500 rows on a model of more than 5,000 entities over 100 revisions: one lookup per expected id; times reported", () => {
    const { model: basis, owned } = measuredModel();
    const state = appliedSheet(basis, owned);
    const expectedIds = state.rows.reduce((sum, row) => sum + (owned.get(row.key)?.length ?? 0), 0);
    const reconcile: number[] = [];
    const cold: number[] = [];
    let model = basis;
    for (let revision = 1; revision <= 100; revision += 1) {
      // Each revision is a new model object, as an apply, Undo or Redo publishes; an unrelated base node moves.
      model = { ...model, nodes: model.nodes.map((node, index) => index === revision ? { ...node, position: { ...node.position, y: revision } } : node) };
      const started = performance.now();
      const index = buildModelIndex(model, 1, revision);
      cold.push(performance.now() - started);
      let lookups = 0;
      const base = indexLookup(index);
      const begin = performance.now();
      const statuses = reconcileRunSheet(state, (ref) => { lookups += 1; return base(ref); });
      reconcile.push(performance.now() - begin);
      expect(lookups).toBe(expectedIds);
      expect([...statuses.values()].every((item) => item.status === "applied")).toBe(true);
    }
    const report = {
      rows: state.rows.length, bendRows: BEND_ROWS, expectedIds, modelEntities: entityCount(model), revisions: 100,
      reconcileMs: { median: quantile(reconcile, 0.5), p95: quantile(reconcile, 0.95) },
      coldBuildModelIndexMs: { median: quantile(cold, 0.5), p95: quantile(cold, 0.95) },
      host: { cpu: cpus()[0]?.model ?? "unknown", cores: cpus().length, platform: platform(), node: process.version },
      note: "indicative; ordinary vitest run on a shared host, not an exclusive measurement",
    };
    console.log(`RUN_SHEET_MEASUREMENT ${JSON.stringify(report)}`);
    expect(report.modelEntities).toBeGreaterThanOrEqual(5000);
    expect(report.expectedIds).toBeGreaterThanOrEqual(1500);
  });

  it("the sheet adds no buildModelIndex call: per revision, with the sheet mounted and without it", async () => {
    const { model, owned } = measuredModel();
    const counts: number[][] = [];
    for (const mounted of [false, true]) {
      vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(model));
      handle.session = null;
      handle.store = null;
      const view = render(mounted ? <SessionWithSheet /> : <SessionOnly />);
      await waitFor(() => expect(handle.session?.model.model).toBeTruthy());
      if (mounted) {
        act(() => handle.store!.update(() => appliedSheet(handle.session!.model.model!, owned, 50)));
        await waitFor(() => expect(view.getByTestId("run-sheet-row-r50")).toHaveAttribute("data-status", "applied"));
      }
      const current = handle.session!.model.model!;
      const target = current.nodes[3];
      const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => "m", value: () => String(target.position.y) };
      const intent = buildGridOperationIntent({ column, model: current, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: 1, value: "7", interaction: "cell" });
      const perRevision: number[] = [];
      let before = builds.count;
      let applied = false;
      await act(async () => { applied = await handle.session!.operations.handleApplyIntent(intent); });
      expect(applied, JSON.stringify(Object.values(handle.session!.operations.operationOutcomes).flatMap((outcome) => outcome.diagnostics.map((item) => [item.code, item.message])))).toBe(true);
      perRevision.push(builds.count - before);
      for (let step = 0; step < 20; step += 1) {
        before = builds.count;
        const revision = handle.session!.model.uiModelRevision;
        act(() => step % 2 === 0 ? handle.session!.operations.handleUndoSessionModelEdit() : handle.session!.operations.handleRedoSessionModelEdit());
        expect(handle.session!.model.uiModelRevision).toBe(revision + 1);
        perRevision.push(builds.count - before);
      }
      if (mounted) expect(view.getByTestId("run-sheet-row-r50")).toHaveAttribute("data-status", "applied");
      counts.push(perRevision);
      view.unmount();
      vi.restoreAllMocks();
    }
    console.log(`RUN_SHEET_INDEX_BUILDS ${JSON.stringify({ without: counts[0], with: counts[1] })}`);
    expect(counts[1]).toEqual(counts[0]);
  });
});

const handle: { session: ReturnType<typeof useWorkspaceSession> | null; store: ReturnType<typeof useRunSheetStore> | null } = { session: null, store: null };
function SessionOnly() {
  handle.session = useWorkspaceSession();
  return null;
}
function SessionWithSheet() {
  const session = useWorkspaceSession();
  const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
  handle.session = session;
  handle.store = store;
  return session.model.model ? <RunSheet host={{ store, queuedBatches: session.operations.queuedBatches, editorIntents: session.operations.editorIntents, registerBatch: session.operations.registerOperationBatch }} model={session.model.model} /> : null;
}
