// Run-sheet entry S1 (DEL-07-02 Design `run-sheet-entry.md` §7): the resolver,
// the builder and the record projection against the hand-built fixture
// `fixtures/model_operations/run_sheet_authoring.json`, applied by the wasm
// engine. The native lane applies the same batches in
// `core/model_operations/operation_applier/tests/run_sheet_authoring.rs`.
import { describe, expect, it } from "vitest";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { convertDisplayQuantities } from "../../../services/displayQuantityService";
import { canonicalJsonString, canonicalSha256Hex, computeModelHash } from "../../../services/hashService";
import { applyOperationBatch, validateOperationBatch, type OperationBatch, type OperationBatchOutcome } from "../../../services/operationBatchService";
import type { PreviewModel } from "../../../types";
import { cornerContext } from "../../bend-authoring/BendAuthoringForms";
import { BEND_LABELS, buildRunSheetBatch, mapOutcomeToRows, projectedCornerContext, type RunSheetBuild } from "./runSheetBatch";
import {
  DEFAULT_NUMBERING, MESSAGES, OPTIONAL_SECTION_COLUMNS, QUANTITY_COLUMNS, assignRowNumber, idsForNumber, initialIdState, modelIds,
  reservedOperationIds, resolveRunSheet, resolveRunSheetWithEngine,
  type ResolvedRunSheet, type ResolvedRunSheetRow, type RunSheetIdState, type RunSheetInput, type RunSheetRowInput, type RunSheetRule,
} from "./runSheetModel";

type FixtureCase = {
  id: string;
  sheet: RunSheetInput;
  expected_batch: OperationBatch;
  expected_applied_model: PreviewModel | null;
  expected_applied_model_sha256: string | null;
  expected_refusal?: { simulation_disposition: string; step_index: number; code: string; row: string };
  arithmetic?: { predecessor_m: number; increment_in: number; converted_then_added_m: number; absolute_entry_m: number };
};
const fixture = fixtureJson as unknown as { basis_model: PreviewModel; cases: FixtureCase[] };
const basis = () => JSON.parse(JSON.stringify(fixture.basis_model)) as PreviewModel;
const caseOf = (id: string) => fixture.cases.find((item) => item.id === id)!;
const APPLIED = fixture.cases.filter((item) => item.expected_applied_model !== null);

/** The fixture's id assignment: row numbers in row order from a fresh counter, skipping model ids. */
function idStateFor(input: RunSheetInput, model: PreviewModel, reserved: Iterable<string> = []): RunSheetIdState {
  const numbering = { ...DEFAULT_NUMBERING, ...(input.numbering ?? {}) };
  const taken = new Set([...modelIds(model), ...reserved]);
  return input.rows.reduce((state, row) => assignRowNumber(state, numbering, row.key, taken), initialIdState(numbering));
}
async function resolveCase(item: FixtureCase): Promise<ResolvedRunSheet> {
  const model = basis();
  const { resolved } = await resolveRunSheetWithEngine(item.sheet, { model, idState: idStateFor(item.sheet, model) });
  return resolved;
}
async function buildCase(item: FixtureCase): Promise<{ resolved: ResolvedRunSheet; build: RunSheetBuild }> {
  const resolved = await resolveCase(item);
  expect(resolved.rows.flatMap((row) => row.refusals), item.id).toEqual([]);
  expect(resolved.queueable, item.id).toBe(true);
  return { resolved, build: await buildRunSheetBatch(resolved) };
}
async function apply(model: PreviewModel, batch: OperationBatch): Promise<OperationBatchOutcome> {
  return applyOperationBatch(model, batch, await computeModelHash(model));
}

describe("run sheet S1: builder against the hand-built batches", () => {
  it.each(fixture.cases.map((item) => [item.id, item] as const))("%s: the builder's batch deep-equals the hand-built batch", async (_id, item) => {
    const { build } = await buildCase(item);
    expect(build.batch).toEqual(item.expected_batch);
  });
});

describe("run sheet S1: the wasm engine applies each batch to its expected model", () => {
  it.each(APPLIED.map((item) => [item.id, item] as const))("%s", async (_id, item) => {
    const model = basis();
    const outcome = await apply(model, item.expected_batch);
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    expect(outcome.simulation_disposition).toBe("committed_as_one_batch");
    expect(outcome.applied_model).toEqual(item.expected_applied_model);
    // The native lane (run_sheet_authoring.rs) asserts the same digest of its applied model.
    expect(`sha256:${await canonicalSha256Hex(outcome.applied_model)}`).toBe(item.expected_applied_model_sha256);
    expect(model).toEqual(fixture.basis_model);
  });
});

describe("run sheet S1: created coordinates", () => {
  it.each(APPLIED.map((item) => [item.id, item] as const))("%s: every created coordinate is bit-identical to the resolver's normalized value", async (_id, item) => {
    const { resolved } = await buildCase(item);
    const outcome = await apply(basis(), item.expected_batch);
    const stored = new Map(outcome.applied_model!.nodes.map((node) => [node.id, node.position]));
    const corners = new Set(resolved.rows.filter((row) => row.kind === "bend").map((row) => row.end!.id));
    let checked = 0;
    for (const row of resolved.rows) {
      if (!row.end || row.end.existing || corners.has(row.end.id)) continue;
      const position = stored.get(row.end.id)!;
      for (const axis of ["x", "y", "z"] as const) {
        expect(Object.is(position[axis], row.end.position[axis]), `${item.id} ${row.end.id}.${axis}`).toBe(true);
        expect(Object.is(row.end.position[axis], -0)).toBe(false);
        checked += 1;
      }
    }
    expect(checked).toBeGreaterThan(0);
  });

  it("non-identity units: x equals convert_for_dimension(0.3 in → m) plus one addition, and differs from absolute entry", async () => {
    const item = caseOf("inch-increments");
    const { resolved } = await buildCase(item);
    const [converted] = await convertDisplayQuantities([{ id: "dx", value: 0.3, from_unit: "in", to_unit: "m", dimension_id: "length" }]);
    expect(converted.status).toBe("converted");
    const viaFacade = 0.1 + (converted as { value: number }).value;
    const x = resolved.rows[1].end!.position.x;
    expect(Object.is(x, viaFacade)).toBe(true);
    expect(x).toBe(0.10762000000000001);
    expect(x).toBe(item.arithmetic!.converted_then_added_m);
    // Absolute entry: the predecessor converted to inches, 0.3 added, converted back.
    const [inInches] = await convertDisplayQuantities([{ id: "abs", value: 0.1, from_unit: "m", to_unit: "in", dimension_id: "length" }]);
    const [back] = await convertDisplayQuantities([{ id: "back", value: (inInches as { value: number }).value + 0.3, from_unit: "in", to_unit: "m", dimension_id: "length" }]);
    const absolute = (back as { value: number }).value;
    expect(absolute).toBe(0.10762000000000002);
    expect(absolute).toBe(item.arithmetic!.absolute_entry_m);
    expect(Object.is(x, absolute)).toBe(false);
    const outcome = await apply(basis(), item.expected_batch);
    const node = outcome.applied_model!.nodes.find((entry) => entry.id === "node:N20")!;
    expect(Object.is(node.position.x, viaFacade)).toBe(true);
  });

  it("a -0 entry: the normalized +0 is displayed, projected, emitted and stored", async () => {
    const item = caseOf("negative-zero");
    const { resolved, build } = await buildCase(item);
    const point = resolved.rows[0];
    expect(point.increments!.x).toBe(-0);
    expect(Object.is(point.increments!.x, -0)).toBe(true);
    for (const row of resolved.rows) for (const axis of ["x", "y", "z"] as const) expect(Object.is(row.end!.position[axis], -0)).toBe(false);
    expect(Object.is(point.end!.position.x, 0)).toBe(true);
    expect(Object.is(resolved.rows[1].end!.position.x, 0)).toBe(true);
    const projected = build.projection.nodes.find((node) => node.id === "node:N10")!;
    expect(Object.is(projected.position.x, 0) && Object.is(projected.position.z, 0)).toBe(true);
    expect(JSON.parse(build.batch.operations[0].change.after).position).toEqual({ x: 0, y: 12, z: 0 });
    const outcome = await apply(basis(), build.batch);
    const stored = outcome.applied_model!.nodes.find((node) => node.id === "node:N10")!;
    expect(Object.is(stored.position.x, 0) && Object.is(stored.position.z, 0)).toBe(true);
  });
});

/** The values the resolver shows for an operation of a row: its cells, readouts, ids and the row's detail. */
function displayed(row: ResolvedRunSheetRow, resolved: ResolvedRunSheet, changeKind: string): unknown {
  const source = resolved.source;
  const value = <T,>(cell: { value?: T; origin: { kind: string } }) => {
    expect(cell.origin.kind, `row ${row.number}`).not.toBe("blank");
    return cell.value;
  };
  switch (changeKind) {
    case "create_node":
      return { id: row.ids.node, label: row.label!.value, position: row.end!.position, provenance: source };
    case "connect_pipe_run": {
      const section: Record<string, unknown> = { outside_diameter: value(row.cells.od), wall_thickness: value(row.cells.wall) };
      for (const column of OPTIONAL_SECTION_COLUMNS) {
        const cell = row.cells[column];
        if (cell.value !== undefined && !cell.absent) section[QUANTITY_COLUMNS[column].member!] = value(cell);
      }
      return { id: row.ids.pipe, label: row.labels.pipe, from: row.start, to: row.end!.id, section, material: value(row.cells.material), y_reference: value(row.cells.orientation), provenance: source };
    }
    case "assign_section":
      return { section_ref: value(row.cells.section) };
    case "create_support":
      return { id: row.ids.support, label: row.labels.support, node: row.kind === "support" ? row.start : row.end!.id, restraints: row.support!.restraints, family: row.support!.family, provenance: source };
    case "insert_bend_at_corner":
      return {
        radius: row.bend!.radius, flexibility_factor: { value: row.bend!.k, unit: "none" }, flexibility_source_reference: row.bend!.kSource,
        new_nodes: [{ id: row.ids.bend!.start, label: BEND_LABELS.start, provenance: source }, { id: row.ids.bend!.end, label: BEND_LABELS.end, provenance: source }],
        new_pipe: { id: row.ids.bend!.arc, label: BEND_LABELS.arc, provenance: source },
        component: { id: row.ids.bend!.component, label: BEND_LABELS.component, provenance: source, geometry_source_reference: source },
      };
    default:
      throw new Error(changeKind);
  }
}

describe("run sheet S1: displayed equals emitted", () => {
  it.each(fixture.cases.map((item) => [item.id, item] as const))("%s: every emitted field has a displayed source of the same value", async (_id, item) => {
    const { resolved, build } = await buildCase(item);
    for (const built of build.rows) {
      const row = resolved.rows.find((entry) => entry.key === built.key)!;
      for (const id of built.operationIds) {
        const op = build.batch.operations.find((entry) => entry.operation_id === id)!;
        expect(JSON.parse(op.change.after), `${item.id} ${id}`).toEqual(displayed(row, resolved, op.change.change_kind));
        if (op.change.change_kind === "create_node") expect(op.change.unit).toBe(resolved.storedLengthUnit);
      }
    }
  });
});

describe("run sheet S1: the record projection", () => {
  it.each(["two-bend-chain", "three-bend-chain"])("%s: each projected corner context equals the engine's", async (id) => {
    const item = caseOf(id);
    const { build } = await buildCase(item);
    const inserts = build.batch.operations.map((op, index) => [op, index] as const).filter(([op]) => op.change.change_kind === "insert_bend_at_corner");
    expect(inserts.length).toBe(id === "two-bend-chain" ? 2 : 3);
    for (const [op, index] of inserts) {
      // The engine's intermediate state: the batch prefix, applied to a scratch copy (test only).
      const prefix = await apply(basis(), { batch_id: `batch:prefix-${index}`, operations: build.batch.operations.slice(0, index) });
      expect(prefix.applied_model, JSON.stringify(prefix.diagnostics)).not.toBeNull();
      const engineContext = await canonicalJsonString(cornerContext(prefix.applied_model!, op.target.ref));
      expect(op.change.before).toBe(engineContext);
    }
  });

  it("a corrupted projection is refused by the engine with OP-BEFORE-VALUE-MISMATCH", async () => {
    const item = caseOf("two-bend-chain");
    const batch = JSON.parse(JSON.stringify(item.expected_batch)) as OperationBatch;
    const index = batch.operations.map((op) => op.change.change_kind).lastIndexOf("insert_bend_at_corner");
    // A projection that missed the first bend's retargeting: pipe:P30 still starts at the first corner.
    const corrupted = batch.operations[index].change.before.replace('"from":"node:N20-T2"', '"from":"node:N20"');
    expect(corrupted).not.toBe(batch.operations[index].change.before);
    batch.operations[index].change.before = corrupted;
    const model = basis();
    const outcome = await apply(model, batch);
    expect(outcome.applied_model).toBeNull();
    expect(outcome.simulation_disposition).toBe("rolled_back_no_model_published");
    const blocked = outcome.operation_outcomes.at(-1)!;
    expect(blocked.index).toBe(index);
    expect(blocked.simulation_status).toBe("blocked");
    expect(blocked.diagnostics.map((diagnostic) => diagnostic.code)).toContain("OP-BEFORE-VALUE-MISMATCH");
    expect(model).toEqual(fixture.basis_model);
  });

  it("projects appends, section_ref and corner replacement only; it places no tangent point", async () => {
    const { build } = await buildCase(caseOf("three-bend-chain"));
    const ids = new Set(build.projection.nodes.map((node) => node.id));
    for (const corner of ["node:N20", "node:N30", "node:N40"]) expect(ids.has(corner)).toBe(false);
    for (const tangent of ["node:N20-T1", "node:N20-T2"]) expect(ids.has(tangent)).toBe(false);
    expect(build.projection.pipe_segments.some((pipe) => pipe.id.endsWith("-bend"))).toBe(false);
    expect(build.projection.pipe_segments.find((pipe) => pipe.id === "pipe:P30")).toMatchObject({ from: "node:N20-T2", to: "node:N30-T1", section_ref: "section:sch-a" });
    expect(projectedCornerContext(build.projection, "node:N40").node).toBeNull();
  });
});

describe("run sheet S1: late-step rollback", () => {
  it("a refused last step leaves the model unchanged and maps to its row", async () => {
    const item = caseOf("refused-last-step");
    const { build } = await buildCase(item);
    const expected = item.expected_refusal!;
    const model = basis();
    for (const outcome of [await validateOperationBatch(model, build.batch, await computeModelHash(model)), await apply(model, build.batch)]) {
      expect(outcome.applied_model).toBeNull();
      expect(outcome.simulation_disposition).toBe(expected.simulation_disposition);
      expect(outcome.operation_outcomes.length).toBe(build.batch.operations.length);
      const blocked = outcome.operation_outcomes[expected.step_index];
      expect(expected.step_index).toBe(build.batch.operations.length - 1);
      expect(blocked.simulation_status).toBe("blocked");
      expect(blocked.diagnostics.map((diagnostic) => diagnostic.code)).toEqual([expected.code]);
      const rows = mapOutcomeToRows(outcome, build);
      const check = rows.get(expected.row)!;
      expect(check.status).toBe("refused");
      expect(check.status === "refused" ? check.diagnostics.map((diagnostic) => diagnostic.code) : []).toEqual([expected.code]);
      expect([...rows].filter(([, check]) => check.status === "refused").map(([key]) => key)).toEqual([expected.row]);
    }
    expect(model).toEqual(fixture.basis_model);
  });

  it("rows whose operations the engine did not reach read 'not checked'", async () => {
    const item = caseOf("straight-chain");
    const { build } = await buildCase(item);
    const batch = JSON.parse(JSON.stringify(build.batch)) as OperationBatch;
    // Blank row r3's node provenance so its create_node is refused; r4 is never reached.
    expect(batch.operations[2].operation_id).toBe("op:run-sheet:r3:0");
    batch.operations[2].change.after = JSON.stringify({ ...JSON.parse(batch.operations[2].change.after), provenance: " " });
    const outcome = await apply(basis(), batch);
    expect(outcome.applied_model).toBeNull();
    const rows = mapOutcomeToRows(outcome, build);
    expect(rows.get("r1")!.status).toBe("passed");
    expect(rows.get("r2")!.status).toBe("passed");
    expect(rows.get("r3")!.status).toBe("refused");
    expect(rows.get("r4")).toEqual({ status: "not_checked", message: MESSAGES.notChecked });
  });
});

describe("run sheet S1: existing-node ownership", () => {
  it("a run to an existing node emits no create_node and owns only its pipe", async () => {
    const { build } = await buildCase(caseOf("run-to-existing-node"));
    expect(build.batch.operations.filter((op) => op.change.change_kind === "create_node").map((op) => op.target.ref)).toEqual(["node:N10"]);
    expect(build.rows.find((row) => row.key === "r2")!.ownedIds).toEqual(["pipe:P20"]);
    expect(build.rows.find((row) => row.key === "r1")!.ownedIds).toEqual(["node:N10"]);
  });
  it.each(["f-branch-agreeing", "f-branch-disagreeing", "straight-chain"])("%s: the F row emits no create_node and owns no node", async (id) => {
    const { resolved, build } = await buildCase(caseOf(id));
    const from = resolved.rows[0];
    expect(from.kind).toBe("from");
    expect(build.rows[0].operationIds).toEqual([]);
    expect(build.rows[0].ownedIds).toEqual([]);
    expect(build.batch.operations.some((op) => op.change.change_kind === "create_node" && op.target.ref === from.end!.id)).toBe(false);
  });
  it("an F row with a support owns the support and never the node", async () => {
    const model = basis();
    const input = sheet([{ key: "r1", kind: "F", node: "node:E", support: "A", restraints: "*" }]);
    const resolved = resolveRunSheet(input, { model, idState: idStateFor(input, model) });
    const build = await buildRunSheetBatch(resolved);
    expect(build.batch.operations.map((op) => [op.change.change_kind, op.target.ref])).toEqual([["create_support", "support:E"]]);
    expect(build.rows[0].ownedIds).toEqual(["support:E"]);
  });
  it("a bend row owns its pipe, tangent points, arc and component, and not the corner", async () => {
    const { build } = await buildCase(caseOf("two-bend-chain"));
    expect(build.rows.find((row) => row.key === "r2")!.ownedIds).toEqual(["pipe:P20", "node:N20-T1", "node:N20-T2", "pipe:N20-bend", "component:N20-bend"]);
  });
});

// ---- Inheritance (§2.1–2.4) and the sheet rules (§4.2), on the fixture's basis model.
const SOURCE = "invented resolver test input";
function sheet(rows: RunSheetRowInput[], extra: Partial<RunSheetInput> = {}): RunSheetInput {
  return { sheetKey: "t", batchSeq: 1, source: SOURCE, rows, ...extra };
}
function resolveSync(input: RunSheetInput, options: { model?: PreviewModel; reserved?: string[] } = {}): ResolvedRunSheet {
  const model = options.model ?? basis();
  return resolveRunSheet(input, { model, idState: idStateFor(input, model), reservedIds: options.reserved });
}
const ELEMENT = { od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" };
const startE: RunSheetRowInput = { key: "r1", kind: "F", node: "node:E" };

describe("run sheet S1: inheritance §2.1", () => {
  it("every inheriting cell takes the predecessor's effective value", () => {
    const resolved = resolveSync(sheet([startE,
      { key: "r2", kind: "B", dx: "2", ...ELEMENT, insulationThickness: "0.05", insulationDensity: "130", contentsDensity: "1000", materialDensity: "7850", millTolerance: "0.0005", corrosionAllowance: "0.001", bendRadius: "0.3", k: "1", kSource: "k" },
      { key: "r3", dy: "2" },
    ], { columnUnits: { insulationDensity: "kg/m^3", contentsDensity: "kg/m^3", materialDensity: "kg/m^3" } }));
    const [, entered, next] = resolved.rows;
    for (const cell of ["od", "wall", "material", "orientation", ...OPTIONAL_SECTION_COLUMNS, "bendRadius"] as const) {
      expect(entered.cells[cell].origin, cell).toEqual({ kind: "entered" });
      expect(next.cells[cell].origin, cell).toEqual({ kind: "inherited", row: 2 });
      expect(next.cells[cell].value, cell).toEqual(entered.cells[cell].value);
    }
    expect(next.cells.od.value).toEqual({ value: 0.1143, unit: "m" });
    expect(next.cells.insulationDensity.value).toEqual({ value: 130, unit: "kg/m^3" });
  });
  it("Section inherits", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", section: "section:sch-a", material: "material:steel", orientation: "0 0 1" }, { key: "r3", dy: "2" }]));
    expect(resolved.rows[2].cells.section).toEqual({ value: "section:sch-a", origin: { kind: "inherited", row: 2 } });
  });
  it("a blank increment is zero, shown as such, and not inherited", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "3" }]));
    expect(resolved.rows[2].increments).toEqual({ x: 0, y: 3, z: 0 });
    expect(resolved.rows[2].end!.position).toEqual({ x: 2, y: 9, z: 0 });
  });
  it("Kind, Support, Restraints, Node, k, k source, Label and Ids never inherit", () => {
    const resolved = resolveSync(sheet([startE,
      { key: "r2", kind: "B", dx: "2", ...ELEMENT, bendRadius: "0.3", k: "1", kSource: "k", label: "Lab" },
      { key: "r3", kind: "B", dy: "2" },
      { key: "r4", dz: "2", orientation: "1 0 0" },
      { key: "r5", node: "node:D" }, { key: "r6", dx: "1" },
    ]));
    const [, r2, r3, r4, , r6] = resolved.rows;
    expect(r4.kind).toBe("run");
    expect(r3.bend).toBeNull();
    expect(r3.refusals).toContainEqual({ rule: "bend", column: "k", message: MESSAGES.bendK });
    expect(r3.label).toEqual({ value: "N30", origin: { kind: "derived", rule: "the node id's token" } });
    expect(r2.label).toEqual({ value: "Lab", origin: { kind: "entered" } });
    expect(r3.ids.node).not.toBe(r2.ids.node);
    expect(r3.ids.pipe).not.toBe(r2.ids.pipe);
    expect(r6.end!.existing).toBe(false);
    expect(r6.start).toBe("node:D");
    const supported = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT, support: "A", restraints: "*" }, { key: "r3", dy: "2" }]));
    expect(supported.rows[1].support).toEqual({ family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"] });
    expect(supported.rows[2].support).toBeNull();
  });
  it("Bend R inherits only from earlier bend rows of the same chain", () => {
    const resolved = resolveSync(sheet([startE,
      { key: "r2", kind: "B", dx: "2", ...ELEMENT, bendRadius: "0.3", k: "1", kSource: "k" },
      { key: "r3", dy: "2" },
      { key: "r4", kind: "P", dx: "20", dy: "20" },
      { key: "r5", kind: "B", dx: "2", ...ELEMENT, k: "1", kSource: "k" },
      { key: "r6", dy: "2" },
    ]));
    expect(resolved.rows[2].cells.bendRadius.value).toEqual({ value: 0.3, unit: "m" });
    expect(resolved.rows[4].cells.bendRadius).toEqual({ origin: { kind: "blank" } });
    expect(resolved.rows[4].refusals).toContainEqual({ rule: "bend", column: "Bend R", message: MESSAGES.bendRadius });
  });
});

describe("run sheet S1: inheritance §2.2", () => {
  it("values inherit recursively back to the row they were entered on", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2" }, { key: "r4", dx: "-1" }]));
    expect(resolved.rows[3].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
  });
  it("an S row passes its predecessor's values through unchanged", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT },
      { key: "r3", kind: "S", support: "V", restraints: "UY", material: "material:alloy" },
      { key: "r4", dy: "2" }]));
    expect(resolved.rows[2].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
    expect(resolved.rows[3].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
    expect(resolved.rows[3].start).toBe(resolved.rows[1].end!.id);
    expect(resolved.rows[2].start).toBe(resolved.rows[1].end!.id);
  });
  it("F and P rows have no predecessor", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", kind: "P", dx: "30", dy: "30" }, { key: "r4", dx: "1" }]));
    expect(resolved.rows[2].cells.material).toEqual({ origin: { kind: "blank" } });
    expect(resolved.rows[3].cells.material).toEqual({ origin: { kind: "blank" } });
    expect(resolved.rows[3].refusals.map((finding) => finding.rule)).toContain("missing-material");
  });
});

describe("run sheet S1: inheritance §2.3", () => {
  it("F's own entered cells start the chain", () => {
    const resolved = resolveSync(sheet([{ ...startE, material: "material:alloy" }, { key: "r2", dx: "2", od: "0.1", wall: "0.005", orientation: "0 0 1" }]));
    expect(resolved.rows[1].cells.material).toEqual({ value: "material:alloy", origin: { kind: "inherited", row: 1 } });
  });
  it("F naming a node drafted earlier takes the drafting row's effective values", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2", material: "material:alloy" },
      { key: "r4", kind: "F", node: "node:N20" }, { key: "r5", dz: "-2", orientation: "0 1 0" }]));
    expect(resolved.rows[3].start).toBe("node:N20");
    expect(resolved.rows[3].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
    expect(resolved.rows[4].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
    expect(resolved.rows[4].start).toBe("node:N20");
    expect(resolved.rows[4].refusals).toEqual([]);
  });
  it("F naming a stored node with agreeing pipes inherits from them, shown as from the first pipe", () => {
    const resolved = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:B" }]));
    const cells = resolved.rows[0].cells;
    for (const cell of ["od", "wall", "material", "orientation"] as const) expect(cells[cell].origin).toEqual({ kind: "stored", pipe: "pipe:AB" });
    expect(cells.material.value).toBe("material:steel");
    expect(cells.od.value).toEqual({ value: 0.1143, unit: "m" });
    expect(cells.insulationThickness).toEqual({ absent: true, origin: { kind: "stored", pipe: "pipe:AB" } });
    expect(cells.section).toEqual({ origin: { kind: "blank" } });
  });
  it("F naming a stored node with disagreeing pipes leaves the field blank for the next run to enter", () => {
    const resolved = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:C" }, { key: "r2", dx: "2" }]));
    expect(resolved.rows[0].cells.material).toEqual({ origin: { kind: "blank" } });
    expect(resolved.rows[0].cells.od.origin).toEqual({ kind: "stored", pipe: "pipe:BC" });
    expect(resolved.rows[1].refusals).toContainEqual({ rule: "missing-material", column: "Material", message: MESSAGES.missingMaterial });
  });
  it("F naming a stored node no pipe meets inherits nothing", () => {
    const resolved = resolveSync(sheet([startE]));
    for (const cell of ["section", "od", "wall", "material", "orientation", "insulationThickness"] as const) expect(resolved.rows[0].cells[cell]).toEqual({ origin: { kind: "blank" } });
  });
  it("a shared section inherits as the Section id when every meeting pipe has the same section_ref", () => {
    const model = basis();
    for (const pipe of model.pipe_segments) {
      if (pipe.id === "pipe:AB" || pipe.id === "pipe:BC") {
        pipe.section_ref = "section:sch-a";
        pipe.section = { outside_diameter: { value: 0.1683, unit: "m" }, wall_thickness: { value: 0.00711, unit: "m" } };
      }
    }
    const resolved = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:B" }]), { model });
    expect(resolved.rows[0].cells.section).toEqual({ value: "section:sch-a", origin: { kind: "stored", pipe: "pipe:AB" } });
    expect(resolved.rows[0].cells.od).toEqual({ value: { value: 0.1683, unit: "m" }, origin: { kind: "derived", rule: "from section section:sch-a" } });
  });
  it("P inherits nothing", () => {
    const resolved = resolveSync(sheet([{ key: "r1", kind: "P", dx: "30", dy: "30", material: "material:steel" }, { key: "r2", dx: "1" }]));
    expect(resolved.rows[0].cells.od).toEqual({ origin: { kind: "blank" } });
    expect(resolved.rows[1].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 1 } });
  });
});

describe("run sheet S1: inheritance §2.4", () => {
  it("an entered value continues until another entered value", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2" }, { key: "r4", dx: "1", material: "material:alloy" }, { key: "r5", dy: "1" }]));
    expect(resolved.rows.slice(1).map((row) => row.cells.material.value)).toEqual(["material:steel", "material:steel", "material:alloy", "material:alloy"]);
    expect(resolved.rows[4].cells.material.origin).toEqual({ kind: "inherited", row: 4 });
  });
  it("an entered value equal to the inherited one is still entered", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2", material: "material:steel" }]));
    expect(resolved.rows[2].cells.material.origin).toEqual({ kind: "entered" });
  });
  it("clearing an entered cell returns it to inheritance", () => {
    const rows: RunSheetRowInput[] = [startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2", material: "material:alloy" }];
    expect(resolveSync(sheet(rows)).rows[2].cells.material.origin).toEqual({ kind: "entered" });
    rows[2] = { ...rows[2], material: "" };
    expect(resolveSync(sheet(rows)).rows[2].cells.material).toEqual({ value: "material:steel", origin: { kind: "inherited", row: 2 } });
  });
  it("`-` ends an optional member and the absence inherits", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT, insulationThickness: "0.05" }, { key: "r3", dy: "2", insulationThickness: "-" }, { key: "r4", dx: "1" }]));
    expect(resolved.rows[2].cells.insulationThickness).toEqual({ absent: true, origin: { kind: "entered" } });
    expect(resolved.rows[3].cells.insulationThickness).toEqual({ absent: true, origin: { kind: "inherited", row: 3 } });
  });
  it("entering a Section shows the section's OD and wall read-only", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2", section: "section:sch-a" }]));
    expect(resolved.rows[2].cells.od).toEqual({ value: { value: 0.1683, unit: "m" }, origin: { kind: "derived", rule: "from section section:sch-a" } });
    expect(resolved.rows[2].cells.wall).toEqual({ value: { value: 0.00711, unit: "m" }, origin: { kind: "derived", rule: "from section section:sch-a" } });
  });
  it("entering OD or Wall under an inherited Section clears the Section for that row and onward", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", section: "section:sch-a", material: "material:steel", orientation: "0 0 1" }, { key: "r3", dy: "2", od: "0.2" }, { key: "r4", dx: "1" }]));
    expect(resolved.rows[2].cells.section).toEqual({ origin: { kind: "derived", rule: "cleared by the OD or Wall entered on this row" } });
    expect(resolved.rows[2].cells.od).toEqual({ value: { value: 0.2, unit: "m" }, origin: { kind: "entered" } });
    expect(resolved.rows[2].cells.wall).toEqual({ value: { value: 0.00711, unit: "m" }, origin: { kind: "inherited", row: 2 } });
    expect(resolved.rows[3].cells.section.value).toBeUndefined();
    expect(resolved.rows[3].cells.od.value).toEqual({ value: 0.2, unit: "m" });
  });
});

describe("run sheet S1: ids §3.2", () => {
  it("assigns each row a number once, skipping any number that yields a reserved id, and never reuses one", () => {
    const numbering = DEFAULT_NUMBERING;
    let state = initialIdState(numbering);
    const reserved = new Set(["pipe:P10", "component:N20-bend"]);
    state = assignRowNumber(state, numbering, "a", reserved);
    expect(state.assigned.a).toBe(30);
    expect(assignRowNumber(state, numbering, "a", reserved)).toBe(state);
    state = assignRowNumber(state, numbering, "b", reserved);
    expect(state.assigned.b).toBe(40);
    // Deleting row "a" releases nothing: the counter has moved on.
    const { a: _deleted, ...rest } = state.assigned;
    state = assignRowNumber({ ...state, assigned: rest }, numbering, "c", reserved);
    expect(state.assigned.c).toBe(50);
    expect(idsForNumber(numbering, 50)).toEqual({ node: "node:N50", pipe: "pipe:P50", support: "support:N50",
      bend: { start: "node:N50-T1", end: "node:N50-T2", arc: "pipe:N50-bend", component: "component:N50-bend" } });
  });
  it("reserves create, connect and insert targets of pending and queued operations, and a queued bend's new ids", () => {
    const ids = reservedOperationIds([...caseOf("two-bend-chain").expected_batch.operations,
      { operation_kind: "modify", target: { ref: "pipe:X" } }]);
    expect([...ids].sort()).toEqual(["component:N20-bend", "component:N30-bend", "node:N20", "node:N20-T1", "node:N20-T2", "node:N30", "node:N30-T1", "node:N30-T2",
      "node:N40", "pipe:N20-bend", "pipe:N30-bend", "pipe:P20", "pipe:P30", "pipe:P40"].sort());
  });
  it("an overwritten id is used as entered", () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT, ids: { node: "node:X1", pipe: "pipe:X1" } }]));
    expect(resolved.rows[1].ids).toMatchObject({ node: "node:X1", pipe: "pipe:X1" });
    expect(resolved.rows[1].label!.value).toBe("X1");
  });
});

function refusalsOf(resolved: ResolvedRunSheet, row: number): RunSheetRule[] {
  return resolved.rows[row - 1].refusals.map((finding) => finding.rule);
}

describe("run sheet S1: sheet rules §4.2", () => {
  it("no current node: a first row that is not F or P, or F naming no node", () => {
    expect(resolveSync(sheet([{ key: "r1", dx: "2", ...ELEMENT }])).rows[0].refusals).toContainEqual({ rule: "no-current-node", message: MESSAGES.noCurrentNode });
    expect(refusalsOf(resolveSync(sheet([{ key: "r1", kind: "F" }])), 1)).toEqual(["no-current-node"]);
    const unknown = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:nowhere" }]));
    expect(unknown.rows[0].refusals).toEqual([{ rule: "no-current-node", column: "Node", message: MESSAGES.noCurrentNode, detail: MESSAGES.unknownNode("node:nowhere") }]);
    expect(refusalsOf(resolveSync(sheet([{ key: "r1", kind: "F", node: "node:nowhere" }, { key: "r2", dx: "1", ...ELEMENT }])), 2)).toContain("no-current-node");
  });
  it("zero length", () => {
    expect(resolveSync(sheet([startE, { key: "r2", ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "zero-length", message: MESSAGES.zeroLength });
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dx: "0", dy: "-0", ...ELEMENT }])), 2)).toContain("zero-length");
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", node: "node:E", ...ELEMENT }])), 2)).toContain("zero-length");
  });
  it("below resolution", () => {
    const model = basis();
    model.nodes.push({ id: "node:far", label: "far", position: { x: 1e17, y: 0, z: 30 }, provenance: "invented" });
    const resolved = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:far" }, { key: "r2", dx: "1", ...ELEMENT }]), { model });
    expect(resolved.rows[1].refusals).toContainEqual({ rule: "below-resolution", column: "dX, dY, dZ", message: MESSAGES.belowResolution });
  });
  it("both forms", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", node: "node:A", ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "both-forms", column: "Node", message: MESSAGES.bothForms });
  });
  it("coincident with a stored or a drafted node", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dy: "-6", ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "coincident", message: MESSAGES.coincident("node:A") });
    const drafted = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT }, { key: "r3", dy: "2" }, { key: "r4", dy: "-2", orientation: "1 0 0" }]));
    expect(drafted.rows[3].refusals).toContainEqual({ rule: "coincident", message: MESSAGES.coincident("node:N20") });
    expect(refusalsOf(resolveSync(sheet([{ key: "r1", kind: "P", dx: "4" }])), 1)).toEqual(["coincident"]);
  });
  it("near-coincident is a warning within the model's tolerance, not a refusal", () => {
    const near = resolveSync(sheet([startE, { key: "r2", dy: "-5.997", ...ELEMENT, orientation: "1 0 0" }]));
    expect(near.rows[1].refusals).toEqual([]);
    expect(near.rows[1].warnings).toEqual([{ rule: "near-coincident", message: MESSAGES.nearCoincident("node:A") }]);
    expect(near.queueable).toBe(true);
    const model = basis();
    model.model_settings = { node_proximity_tolerance: { value: 1, unit: "mm" } };
    return resolveRunSheetWithEngine(sheet([startE, { key: "r2", dy: "-5.997", ...ELEMENT, orientation: "1 0 0" }]), { model, idState: idStateFor(sheet([startE, { key: "r2" }]), model) })
      .then(({ resolved }) => expect(resolved.rows[1].warnings).toEqual([]));
  });
  it("missing material", () => {
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, material: "" }])), 2)).toContain("missing-material");
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, material: "material:none" }])).rows[1].refusals).toContainEqual({ rule: "missing-material", column: "Material", message: MESSAGES.missingMaterial });
  });
  it("missing section, with the specific failure", () => {
    const detail = (row: RunSheetRowInput) => resolveSync(sheet([startE, row])).rows[1].refusals.find((finding) => finding.rule === "missing-section");
    expect(detail({ key: "r2", dx: "1", material: "material:steel", orientation: "0 0 1" })).toEqual({ rule: "missing-section", column: "Section", message: MESSAGES.missingSection, detail: "OD is missing." });
    expect(detail({ key: "r2", dx: "1", ...ELEMENT, wall: "" })!.detail).toBe("Wall is missing.");
    expect(detail({ key: "r2", dx: "1", ...ELEMENT, od: "0" })!.detail).toBe("OD and wall must be positive.");
    expect(detail({ key: "r2", dx: "1", ...ELEMENT, wall: "0.06" })).toEqual({ rule: "missing-section", column: "Wall", message: MESSAGES.missingSection, detail: "The wall must be less than half the OD." });
    expect(detail({ key: "r2", dx: "1", ...ELEMENT, od: "big" })!.detail).toBe(MESSAGES.quantityNotNumber("OD", "m"));
    expect(detail({ key: "r2", dx: "1", ...ELEMENT, section: "section:none" })!.detail).toBe("Section section:none is not a section of this model.");
    const model = basis();
    model.sections!.push({ id: "section:rigid", name: "Invented rigid", section_type: "rigid", properties: { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.01, unit: "m" } }, provenance: "invented" });
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", section: "section:rigid", material: "material:steel", orientation: "0 0 1" }]), { model }).rows[1].refusals)
      .toContainEqual({ rule: "missing-section", column: "Section", message: MESSAGES.missingSection, detail: "Section section:rigid is not a shared pipe section (type pipe, outside diameter and wall only)." });
  });
  it("missing orientation: blank, zero or not three numbers", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, orientation: "" }])).rows[1].refusals).toContainEqual({ rule: "missing-orientation", column: "Orientation", message: MESSAGES.missingOrientation });
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, orientation: "0 0 0" }])), 2)).toContain("missing-orientation");
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, orientation: "0 1" }])).rows[1].refusals)
      .toContainEqual({ rule: "missing-orientation", column: "Orientation", message: MESSAGES.missingOrientation, detail: "Enter the orientation as three numbers." });
  });
  it("parallel orientation (exactly) is refused; near-parallel is left to the engine", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dz: "1", ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "parallel-orientation", column: "Orientation", message: MESSAGES.parallelOrientation });
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dz: "-1", ...ELEMENT, orientation: "0 0 -3" }])), 2)).toContain("parallel-orientation");
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dz: "1", ...ELEMENT, orientation: "0 1e-9 1" }])), 2)).toEqual([]);
  });
  it("invalid kind or support code", () => {
    expect(resolveSync(sheet([startE, { key: "r2", kind: "x", dx: "1", ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "invalid-code", column: "Kind", message: MESSAGES.invalidCode("x", "kind") });
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, support: "Q", restraints: "*" }])).rows[1].refusals).toContainEqual({ rule: "invalid-code", column: "Support", message: MESSAGES.invalidCode("Q", "support") });
    expect(resolveSync(sheet([startE, { key: "r2", kind: "b", dx: "1", ...ELEMENT }])).rows[1].kind).toBe("bend");
  });
  it("unit text in an increment", () => {
    for (const text of ["2 m", "0x10", "Infinity", "1,5"]) {
      expect(resolveSync(sheet([startE, { key: "r2", dx: text, ...ELEMENT }])).rows[1].refusals).toContainEqual({ rule: "increment-not-number", column: "dX, dY, dZ", message: MESSAGES.incrementNotNumber("m") });
    }
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dx: ".5e1", ...ELEMENT }])), 2)).toEqual([]);
  });
  it("a quantity without a column unit", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, insulationDensity: "130" }])).rows[1].refusals).toContainEqual({ rule: "quantity-without-unit", column: "Ins. dens.", message: MESSAGES.quantityWithoutUnit("Ins. dens.") });
    const model = basis();
    delete (model.project.units as Record<string, string>).length;
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT }], { lengthUnit: "m" }), { model });
    expect(resolved.rows[1].refusals).toContainEqual({ rule: "quantity-without-unit", column: "OD", message: MESSAGES.quantityWithoutUnit("OD") });
    expect(resolved.blockers).toContainEqual({ rule: "length-unit", message: MESSAGES.noStoredUnit });
    expect(refusalsOf(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, insulationDensity: "dense" }], { columnUnits: { insulationDensity: "kg/m^3" } })), 2)).toEqual(["quantity-not-number"]);
  });
  it("support: no restraints, invalid or duplicate tokens; restraints without a support code", () => {
    for (const restraints of ["", "UX QQ", "UX ux"]) {
      expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, support: "G", restraints }])).rows[1].refusals).toContainEqual({ rule: "support", column: "Restraints", message: MESSAGES.support });
    }
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, restraints: "UX" }])).rows[1].refusals).toContainEqual({ rule: "support", column: "Restraints", message: MESSAGES.restraintsWithoutSupport });
  });
  it("bend: the next run, one section and material, no support, nothing else at the corner, R, k and its source", () => {
    const bend = { kind: "B", dx: "2", ...ELEMENT, bendRadius: "0.3", k: "1", kSource: "k" };
    const findings = (rows: RunSheetRowInput[], row = 2) => resolveSync(sheet([startE, ...rows])).rows[row - 1].refusals;
    expect(findings([{ key: "r2", ...bend }])).toContainEqual({ rule: "bend", message: MESSAGES.bendNeedsNextRun });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", kind: "S", support: "A", restraints: "*" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendNeedsNextRun });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2", material: "material:alloy" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendMismatch("Material", 3) });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2", od: "0.2" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendMismatch("OD", 3) });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2", insulationThickness: "0.05" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendMismatch("Ins. thk", 3) });
    expect(findings([{ key: "r2", ...bend, support: "A", restraints: "*" }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", column: "Support", message: MESSAGES.bendSupport });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2" }, { key: "r4", kind: "F", node: "node:N20" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendCornerReferenced(4) });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2" }, { key: "r4", kind: "P", dx: "9", dy: "9" }, { key: "r5", node: "node:N20", ...ELEMENT }])).toContainEqual({ rule: "bend", message: MESSAGES.bendCornerReferenced(5) });
    expect(findings([{ key: "r2", kind: "B", node: "node:D", ...ELEMENT, bendRadius: "0.3", k: "1", kSource: "k" }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", message: MESSAGES.bendCornerPipes(3) });
    expect(findings([{ key: "r2", ...bend, bendRadius: "0" }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", column: "Bend R", message: MESSAGES.bendRadius });
    expect(findings([{ key: "r2", ...bend, k: "" }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", column: "k", message: MESSAGES.bendK });
    expect(findings([{ key: "r2", ...bend, k: "-1" }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", column: "k", message: MESSAGES.bendK });
    expect(findings([{ key: "r2", ...bend, kSource: " " }, { key: "r3", dy: "2" }])).toContainEqual({ rule: "bend", column: "k", message: MESSAGES.bendK });
    expect(findings([{ key: "r2", ...bend }, { key: "r3", dy: "2" }])).toEqual([]);
  });
  it("ids: reserved, duplicate or blank", () => {
    expect(resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT }]), { reserved: [] }).rows[1].refusals).toEqual([]);
    const input = sheet([startE, { key: "r2", dx: "1", ...ELEMENT }]);
    const model = basis();
    const state = idStateFor(input, model);
    const reserved = resolveRunSheet(input, { model, idState: state, reservedIds: ["pipe:P20"] });
    expect(reserved.rows[1].refusals).toEqual([{ rule: "ids", column: "Ids", message: MESSAGES.idUsed("pipe:P20") }]);
    const stored = resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT, ids: { node: "node:D" } }]));
    expect(stored.rows[1].refusals).toContainEqual({ rule: "ids", column: "Ids", message: MESSAGES.idUsed("node:D") });
    const duplicate = resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT }, { key: "r3", dy: "1", ids: { pipe: "pipe:P20" } }]));
    expect(duplicate.rows[1].refusals).toContainEqual({ rule: "ids", column: "Ids", message: MESSAGES.idUsed("pipe:P20") });
    expect(duplicate.rows[2].refusals).toContainEqual({ rule: "ids", column: "Ids", message: MESSAGES.idUsed("pipe:P20") });
    const support = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:A", support: "A", restraints: "*" }]));
    expect(support.rows[0].refusals).toEqual([{ rule: "ids", column: "Ids", message: MESSAGES.idUsed("support:A") }]);
    const blank = resolveRunSheet(input, { model, idState: initialIdState() });
    expect(blank.rows[1].refusals).toEqual([
      { rule: "ids", column: "Ids", message: MESSAGES.idBlank("node") },
      { rule: "ids", column: "Ids", message: MESSAGES.idBlank("pipe") },
    ]);
  });
  it("source: a blank Source disables Queue", async () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT }], { source: "  " }));
    expect(resolved.blockers).toEqual([{ rule: "source", message: MESSAGES.source }]);
    expect(resolved.queueable).toBe(false);
    await expect(buildRunSheetBatch(resolved)).rejects.toThrow();
  });
  it("a refused row blocks the whole batch (all or nothing)", async () => {
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "1", ...ELEMENT }, { key: "r3", dy: "1", material: "material:none" }]));
    expect(resolved.queueable).toBe(false);
    await expect(buildRunSheetBatch(resolved)).rejects.toThrow();
  });
  it("a sheet unit that is not a length unit is refused, never guessed", async () => {
    const model = basis();
    const input = sheet([startE, { key: "r2", dx: "1", ...ELEMENT }], { lengthUnit: "Pa" });
    const { resolved } = await resolveRunSheetWithEngine(input, { model, idState: idStateFor(input, model) });
    expect(resolved.rows[1].refusals.map((finding) => finding.rule)).toContain("length-unit");
  });
});
