// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §4.1; DEL-07-02 Work
// `run-sheet-entry.md` §1.2, §2.1, §3.3): the run sheet's Condition set column. The basis is
// the committed run-sheet fixture with two sets created and assigned through the wasm
// engine's own operations (no document is authored here); each built batch is applied by
// the engine.
import { describe, expect, it } from "vitest";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { canonicalJsonString, computeModelHash } from "../../../services/hashService";
import { applyModelOperation } from "../../../services/operationService";
import { applyOperationBatch, type OperationBatch } from "../../../services/operationBatchService";
import type { ConditionSet, EditorOperationIntent, PreviewModel } from "../../../types";
import { cornerContext } from "../../bend-authoring/BendAuthoringForms";
import { assignConditionSetIntent, createConditionSetIntent } from "../../condition-sets/conditionSetIntents";
import { buildRunSheetBatch } from "./runSheetBatch";
import {
  DEFAULT_NUMBERING, MESSAGES, assignRowNumber, initialIdState, modelIds, resolveRunSheet, resolveRunSheetWithEngine,
  type ResolvedRunSheet, type RunSheetIdState, type RunSheetInput, type RunSheetRowInput,
} from "./runSheetModel";
import { SHEET_COLUMNS, cellView, effectiveText } from "./runSheetView";

const fixture = fixtureJson as unknown as { basis_model: PreviewModel };
const SOURCE = "invented S3b run-sheet input";
const set = (id: string): ConditionSet => ({ id, provenance: SOURCE, installation_temperature: { value: 20, unit: "degC" }, conditions: [{ key: "OPE1", temperature: { value: 150, unit: "degC" } }] });

async function applyOne(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  return outcome.applied_model as PreviewModel;
}
/** The fixture basis with sets cs:a and cs:b; pipe:AB and pipe:BC carry cs:a and pipe:CD carries cs:b. */
async function basisWithSets(): Promise<PreviewModel> {
  let model = JSON.parse(JSON.stringify(fixture.basis_model)) as PreviewModel;
  model = await applyOne(model, createConditionSetIntent(set("cs:a")));
  model = await applyOne(model, createConditionSetIntent(set("cs:b")));
  for (const [pipe, id] of [["pipe:AB", "cs:a"], ["pipe:BC", "cs:a"], ["pipe:CD", "cs:b"]]) {
    model = await applyOne(model, await assignConditionSetIntent(canonicalJsonString, model.pipe_segments.find((item) => item.id === pipe)!, id));
  }
  return model;
}
function idStateFor(input: RunSheetInput, model: PreviewModel): RunSheetIdState {
  const taken = new Set(modelIds(model));
  return input.rows.reduce((state, row) => assignRowNumber(state, DEFAULT_NUMBERING, row.key, taken), initialIdState(DEFAULT_NUMBERING));
}
const sheet = (rows: RunSheetRowInput[]): RunSheetInput => ({ sheetKey: "cs", batchSeq: 1, source: SOURCE, rows });
const resolveSync = (input: RunSheetInput, model: PreviewModel) => resolveRunSheet(input, { model, idState: idStateFor(input, model) });
async function resolveWithEngine(input: RunSheetInput, model: PreviewModel): Promise<ResolvedRunSheet> {
  return (await resolveRunSheetWithEngine(input, { model, idState: idStateFor(input, model) })).resolved;
}
const ELEMENT = { od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" };
const startE: RunSheetRowInput = { key: "r1", kind: "F", node: "node:E" };
const BEND = { bendRadius: "0.5", k: "1.5", kSource: "invented k (user entry)" };

describe("Condition set column: inheritance (§4.1, run-sheet §2.1–§2.4)", () => {
  it("inherits like Material; `-` means absent, ends inheritance, and the absence inherits", async () => {
    const model = await basisWithSets();
    const resolved = resolveSync(sheet([startE,
      { key: "r2", dx: "2", ...ELEMENT, conditionSet: "cs:a" },
      { key: "r3", dy: "2" },
      { key: "r4", dx: "2", conditionSet: "-" },
      { key: "r5", dy: "2" },
      { key: "r6", dx: "2", conditionSet: "cs:b" },
    ]), model);
    const cells = resolved.rows.map((row) => row.cells.conditionSet);
    expect(cells[1]).toEqual({ value: "cs:a", origin: { kind: "entered" } });
    expect(cells[2]).toEqual({ value: "cs:a", origin: { kind: "inherited", row: 2 } });
    expect(cells[3]).toEqual({ absent: true, origin: { kind: "entered" } });
    expect(cells[4]).toEqual({ absent: true, origin: { kind: "inherited", row: 4 } });
    expect(cells[5]).toEqual({ value: "cs:b", origin: { kind: "entered" } });
    expect(resolved.rows.flatMap((row) => row.refusals)).toEqual([]);
  });

  it("an F row from a stored node inherits only when every meeting pipe carries the same set", async () => {
    const model = await basisWithSets();
    const agreeing = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:B" }, { key: "r2", dz: "2" }]), model);
    expect(agreeing.rows[0].cells.conditionSet).toEqual({ value: "cs:a", origin: { kind: "stored", pipe: "pipe:AB" } });
    expect(agreeing.rows[1].cells.conditionSet).toEqual({ value: "cs:a", origin: { kind: "stored", pipe: "pipe:AB" } });
    const disagreeing = resolveSync(sheet([{ key: "r1", kind: "F", node: "node:C" }]), model);
    expect(disagreeing.rows[0].cells.conditionSet).toEqual({ origin: { kind: "blank" } });
    const none = resolveSync(sheet([startE]), model);
    expect(none.rows[0].cells.conditionSet).toEqual({ origin: { kind: "blank" } });
  });

  it("an id that names no set of the model is refused by name in the Condition set column", async () => {
    const model = await basisWithSets();
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT, conditionSet: "cs:absent" }]), model);
    expect(resolved.rows[1].refusals).toContainEqual({ rule: "condition-set", column: "Condition set", message: MESSAGES.unknownConditionSet("cs:absent") });
    expect(resolved.queueable).toBe(false);
  });

  it("shows origin and absence in the cell, and Fill down copies the effective value", async () => {
    const model = await basisWithSets();
    const resolved = resolveSync(sheet([startE, { key: "r2", dx: "2", ...ELEMENT, conditionSet: "cs:a" }, { key: "r3", dy: "2" }, { key: "r4", dx: "2", conditionSet: "-" }]), model);
    const column = SHEET_COLUMNS.find((item) => item.key === "conditionSet")!;
    expect(column.label).toBe("Condition set");
    const view = { resolved } as never;
    const viewRow = (index: number) => ({ resolved: resolved.rows[index], values: {}, refusals: resolved.rows[index].refusals, status: "draft" }) as never;
    expect(cellView(view, viewRow(2), column, () => "")).toMatchObject({ text: "cs:a", muted: true, origin: "inherited from row 2" });
    expect(cellView(view, viewRow(3), column, () => "")).toMatchObject({ text: "—", origin: "absent" });
    expect(effectiveText(resolved.rows[2], "conditionSet", "")).toBe("cs:a");
    expect(effectiveText(resolved.rows[3], "conditionSet", "")).toBe("-");
  });
});

describe("Condition set column: emission, bends and the record projection (§4.1, run-sheet §3.3)", () => {
  it("emits one assign_condition_set after connect_pipe_run and assign_section, only where the effective value is present", async () => {
    const model = await basisWithSets();
    const input = sheet([startE,
      { key: "r2", dx: "2", section: "section:sch-a", material: "material:steel", orientation: "0 0 1", conditionSet: "cs:a" },
      { key: "r3", dy: "2", conditionSet: "-" },
      { key: "r4", dx: "2" },
    ]);
    const resolved = await resolveWithEngine(input, model);
    expect(resolved.queueable).toBe(true);
    const build = await buildRunSheetBatch(resolved);
    const kinds = (rowKey: string) => build.batch.operations.filter((op) => op.operation_id.startsWith(`op:run-sheet:${rowKey}:`)).map((op) => op.change.change_kind);
    expect(kinds("r2")).toEqual(["create_node", "connect_pipe_run", "assign_section", "assign_condition_set"]);
    expect(kinds("r3")).toEqual(["create_node", "connect_pipe_run", "assign_section"]);
    expect(kinds("r4")).toEqual(["create_node", "connect_pipe_run", "assign_section"]);
    const assign = build.batch.operations.find((op) => op.change.change_kind === "assign_condition_set")!;
    expect(assign.target).toEqual({ object_type: "Element", ref: resolved.rows[1].ids.pipe });
    expect(assign.change).toMatchObject({ field_path: "condition_set_ref", before: "not_present", after: JSON.stringify({ condition_set_ref: "cs:a" }), unit: "none", dimension: "dimensionless" });
    // connect_pipe_run's payload is unchanged: it never carries condition_set_ref.
    for (const op of build.batch.operations.filter((item) => item.change.change_kind === "connect_pipe_run")) expect(JSON.parse(op.change.after)).not.toHaveProperty("condition_set_ref");

    const outcome = await applyOperationBatch(model, build.batch, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    const stored = (id: string) => outcome.applied_model!.pipe_segments.find((pipe) => pipe.id === id)!;
    expect(stored(resolved.rows[1].ids.pipe!).condition_set_ref).toBe("cs:a");
    expect(stored(resolved.rows[2].ids.pipe!)).not.toHaveProperty("condition_set_ref");
    expect(stored(resolved.rows[3].ids.pipe!)).not.toHaveProperty("condition_set_ref");
    // The projection holds exactly the pipe records the engine stored.
    for (const row of resolved.rows.slice(1)) expect(build.projection.pipe_segments.find((pipe) => pipe.id === row.ids.pipe)).toEqual(stored(row.ids.pipe!));
  });

  it("a bend row and its next run row need the same effective set, or none on both", async () => {
    const model = await basisWithSets();
    const refused = resolveSync(sheet([startE, { key: "r2", kind: "B", dx: "3", ...ELEMENT, ...BEND, conditionSet: "cs:a" }, { key: "r3", dy: "3", conditionSet: "-" }]), model);
    expect(refused.rows[1].refusals).toContainEqual({ rule: "bend", message: MESSAGES.bendConditionSetMismatch(3) });
    const other = resolveSync(sheet([startE, { key: "r2", kind: "B", dx: "3", ...ELEMENT, ...BEND, conditionSet: "cs:a" }, { key: "r3", dy: "3", conditionSet: "cs:b" }]), model);
    expect(other.rows[1].refusals).toContainEqual({ rule: "bend", message: MESSAGES.bendConditionSetMismatch(3) });
    const absentAndBlank = resolveSync(sheet([startE, { key: "r2", kind: "B", dx: "3", ...ELEMENT, ...BEND, conditionSet: "-" }, { key: "r3", dy: "3", conditionSet: "" }]), model);
    expect(absentAndBlank.rows[1].refusals.filter((item) => item.rule === "bend")).toEqual([]);
  });

  it("projects condition_set_ref into the pipe records a later bend's before context holds, and the engine applies the chain", async () => {
    const model = await basisWithSets();
    const input = sheet([startE,
      { key: "r2", kind: "B", dx: "3", ...ELEMENT, ...BEND, conditionSet: "cs:a" },
      { key: "r3", kind: "B", dy: "3", k: "1.5", kSource: "invented k (user entry)" },
      { key: "r4", dz: "3", orientation: "1 0 0" },
    ]);
    const resolved = await resolveWithEngine(input, model);
    expect(resolved.rows.flatMap((row) => row.refusals)).toEqual([]);
    const build = await buildRunSheetBatch(resolved);
    const operations = build.batch.operations;
    const inserts = operations.map((op, index) => [op, index] as const).filter(([op]) => op.change.change_kind === "insert_bend_at_corner");
    expect(inserts).toHaveLength(2);
    for (const [op, index] of inserts) {
      // Each bend's before context names its straights' condition_set_ref, as the engine holds them.
      expect(op.change.before).toContain('"condition_set_ref":"cs:a"');
      const prefix = await applyOperationBatch(model, { batch_id: `batch:prefix-${index}`, operations: operations.slice(0, index) } as OperationBatch, await computeModelHash(model));
      expect(prefix.applied_model, JSON.stringify(prefix.diagnostics)).not.toBeNull();
      expect(op.change.before).toBe(await canonicalJsonString(cornerContext(prefix.applied_model!, op.target.ref)));
    }
    const outcome = await applyOperationBatch(model, build.batch, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    const created = outcome.applied_model!.pipe_segments.filter((pipe) => !model.pipe_segments.some((item) => item.id === pipe.id));
    // Every created pipe, the arcs included (the bend carries its straights' set), carries cs:a.
    expect(created.length).toBe(5);
    expect(created.map((pipe) => pipe.condition_set_ref)).toEqual(created.map(() => "cs:a"));
  });
});
