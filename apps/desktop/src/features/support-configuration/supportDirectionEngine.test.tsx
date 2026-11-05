/** R-6/R-7 S-K1 desktop entry against the real operation engine (DEL-07-02 Work
 * `support-direction.md`). No hash or operation mocks: these need the rebuilt Wasm
 * (`npm run build:wasm`). Invented fixtures only.
 *
 * Two groups:
 * - Every existing support edit path on a record with a stored `direction`. Today's engine
 *   already keeps the member through `update_support` and refuses an inadmissible edit by
 *   the document rules' names, so these run now.
 * - The creation path. Main's engine still refuses every `direction` at `create_support`
 *   (`SUPPORT_DIRECTION_SKEW_UNSUPPORTED`; `OP-CREATE-SUPPORT-PAYLOAD-INVALID` for an axis
 *   vector), so these are skipped, pending the S-K1 applier on main (load-cases lane).
 *   Flip `SK1_CREATE_SUPPORT_DIRECTION_ON_MAIN` when that applier lands. */
import { changeFormControl } from "../../test-support/workspaceTestControls";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SupportConfigurationForm } from "./SupportConfigurationForm";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import { buildHangerSelectionBatch, type HangerDocument } from "../hanger-selection/hangerSelection";
import { buildGridOperationIntent, type GridColumn } from "../workspace/table/modelTableAdapter";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { loadPreviewModel } from "../../services/previewService";
import fixture from "../../../../../fixtures/hanger/invented_hanger_library_valid.json";
import type { EditorOperationIntent, PreviewModel } from "../../types";

/** Pending the S-K1 applier on main (load-cases lane): set to true when it lands. */
const SK1_CREATE_SUPPORT_DIRECTION_ON_MAIN = false;
const PENDING_SK1 = "pending the S-K1 applier on main (load-cases lane)";
const itPendingSk1 = SK1_CREATE_SUPPORT_DIRECTION_ON_MAIN ? it : it.skip;

afterEach(cleanup);
const enter = (label: string, value: string) => changeFormControl(screen.getByLabelText(label), { target: { value } });
const CONFIG = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];
const projection = (support: object) => Object.fromEntries(CONFIG.filter(k => k in support).map(k => [k, (support as Record<string, unknown>)[k]]));
/** What both a single and a batch outcome carry. */
type Outcome = { validation: { application_status: string }; diagnostics: Array<{ code: string }>; applied_model?: PreviewModel | null };
const codes = (outcome: Outcome) => outcome.diagnostics.map(d => d.code);
const bits = (v: number) => { const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, v); return view.getBigUint64(0); };
const bitsOf = (d: { x: number; y: number; z: number }) => [bits(d.x), bits(d.y), bits(d.z)];
/** Typed text the binary64 parse keeps: 0.1 and 0.2 are inexact, 5e-300 is far from 1. */
const STORED = { x: 0.1, y: -0.2, z: 5e-300 };

/** The sample model, with its first support a skewed line stop on UY at its node. */
async function skewModel(): Promise<PreviewModel> {
  const model = await loadPreviewModel();
  const support = model.supports[0] as unknown as Record<string, unknown>;
  for (const key of ["stiffness", "hanger", "nonlinear", "properties"]) delete support[key];
  Object.assign(support, { family: "line_stop", restraints: ["UY"], direction: { ...STORED } });
  return model;
}
function storedAfter(outcome: Outcome, id: string) {
  expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
  return outcome.applied_model!.supports.find(s => s.id === id)!;
}
function expectRefused(outcome: Outcome, code: string) {
  expect(outcome.validation.application_status).not.toBe("applied_to_session_model");
  expect(outcome.applied_model ?? null).toBeNull();
  expect(codes(outcome), JSON.stringify(outcome.diagnostics)).toContain(code);
}
async function queueFormEdit(model: PreviewModel, id: string, edit: () => void): Promise<EditorOperationIntent> {
  const queue = vi.fn();
  render(<SupportConfigurationForm model={model} selection={{ type: "support", id }} onQueueIntent={queue} />);
  expect(screen.getByLabelText("Stored direction")).toHaveTextContent("(0.1, -0.2, 5e-300)");
  edit();
  fireEvent.click(screen.getByText("Queue support configuration"));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  cleanup();
  return queue.mock.calls[0][0];
}

describe("a stored direction through every existing support edit path (real engine)", () => {
  it("the support configuration form's update_support keeps it bit for bit (a), and UY → UX keeps it (b)", async () => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    const provenance = await queueFormEdit(model, id, () => enter("Support provenance", "revised invented provenance"));
    const kept = storedAfter(await applyModelOperation(model, provenance, null), id);
    expect(kept.provenance).toBe("revised invented provenance");
    expect(bitsOf(kept.direction!)).toEqual(bitsOf(STORED));
    const moved = await queueFormEdit(model, id, () => { fireEvent.click(screen.getByLabelText("Restrain UY")); fireEvent.click(screen.getByLabelText("Restrain UX")); });
    const token = storedAfter(await applyModelOperation(model, moved, null), id);
    expect(token.restraints).toEqual(["UX"]);
    expect(bitsOf(token.direction!)).toEqual(bitsOf(STORED));
    expect(JSON.stringify(model)).toBe(original);
  });

  it("the support configuration form's update_support is refused by name when the edit makes it inadmissible (a)", async () => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    expectRefused(await applyModelOperation(model, await queueFormEdit(model, id, () => enter("Support family", "guide")), null), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expectRefused(await applyModelOperation(model, await queueFormEdit(model, id, () => fireEvent.click(screen.getByLabelText("Restrain UX"))), null), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expectRefused(await applyModelOperation(model, await queueFormEdit(model, id, () => { fireEvent.click(screen.getByLabelText("Restrain UY")); fireEvent.click(screen.getByLabelText("Restrain RX")); }), null), "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED");
    // A nonlinear member, as the configuration payload carries it.
    const before = projection(model.supports[0]);
    const nonlinear = makeRichIntent({ object_type: "Support", ref: id }, "update_support", "configuration", await canonicalJsonString(before),
      { ...before, nonlinear: { behavior: "one_way", dof: "UY", initial_state: "active", active_when: "positive_reaction" } }, "Support configuration");
    expectRefused(await applyModelOperation(model, nonlinear, null), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expect(JSON.stringify(model)).toBe(original);
  });

  it("bulk change's support kind keeps it, or is refused by name", async () => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    const scope = [{ type: "support" as const, id }];
    const retoken = await buildBulkChangeBatch(model, scope, { kind: "support-kind", fromFamily: "line_stop", toFamily: "line_stop", restraints: ["UZ"] }, canonicalJsonString, "bulk-direction-1");
    const applied = await applyOperationBatch(model, retoken, await computeModelHash(model));
    const kept = storedAfter(applied, id);
    expect(kept.restraints).toEqual(["UZ"]);
    expect(bitsOf(kept.direction!)).toEqual(bitsOf(STORED));
    const guide = await buildBulkChangeBatch(model, scope, { kind: "support-kind", fromFamily: "line_stop", toFamily: "guide", restraints: "keep" }, canonicalJsonString, "bulk-direction-2");
    expectRefused(await applyOperationBatch(model, guide, await computeModelHash(model)), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expect(JSON.stringify(model)).toBe(original);
  });

  it("hanger selection's update_support is refused by name and never drops it", async () => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    const doc = structuredClone(fixture) as HangerDocument;
    doc.hanger_records[0].hanger.cold_load = { magnitude: 15, unit: "N", dimension: "force", provenance: { ...doc.hanger_records[0].provenance } };
    doc.hanger_records[0].hanger.hot_load = { magnitude: 18, unit: "N", dimension: "force", provenance: { ...doc.hanger_records[0].provenance } };
    const batch = await buildHangerSelectionBatch(model, doc, doc.hanger_records[0].hanger_id, { supportId: id, restraints: ["UY"] });
    expectRefused(await applyOperationBatch(model, batch, await computeModelHash(model)), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expect(JSON.stringify(model)).toBe(original);
  });

  it("the support grid's cells keep it, or are refused by name", async () => {
    const model = await skewModel(), original = JSON.stringify(model), support = model.supports[0], id = support.id;
    const row = { id, label: support.label, type: "support" as const, searchText: "", raw: support };
    const cell = (fieldPath: string, before: string, value: string) => {
      const column: GridColumn = { key: fieldPath, label: fieldPath, fieldPath, objectType: "Support", changeKind: "update_support", dimension: "dimensionless", sourceNote: "test", unit: () => "none", value: () => before };
      return buildGridOperationIntent({ row, column, model, sequence: 1, value, interaction: "cell" });
    };
    for (const [fieldPath, before, value] of [["label", support.label, "Renamed skew stop"], ["provenance", support.provenance, "revised"], ["restraints", "UY", "UZ"]]) {
      const kept = storedAfter(await applyModelOperation(model, cell(fieldPath, before, value), null), id);
      expect(bitsOf(kept.direction!), fieldPath).toEqual(bitsOf(STORED));
    }
    expectRefused(await applyModelOperation(model, cell("restraints", "UY", "UX, UY"), null), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    expect(JSON.stringify(model)).toBe(original);
  });
});

describe(`support creation with a direction (real engine), ${PENDING_SK1}`, () => {
  async function createThroughForm(model: PreviewModel, family: string, dof: string, direction: [string, string, string], extra?: () => void) {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueIntent={queue} />);
    enter("Support ID", "support:skew-new"); enter("Support label", "Invented skew"); enter("Support provenance", "original synthetic test");
    enter("Support family", family);
    fireEvent.click(screen.getByLabelText(`Restrain ${dof}`));
    extra?.();
    enter("Direction x", direction[0]); enter("Direction y", direction[1]); enter("Direction z", direction[2]);
    fireEvent.click(screen.getByText("Queue support creation"));
    return queue;
  }

  itPendingSk1("creates a skewed line stop through the form; the stored bits equal the input", async () => {
    const model = await loadPreviewModel(), original = JSON.stringify(model);
    const queue = await createThroughForm(model, "line_stop", "UY", ["0.1", "-0.2", "5e-300"]);
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const stored = storedAfter(await applyModelOperation(model, queue.mock.calls[0][0], null), "support:skew-new");
    expect(bitsOf(stored.direction!)).toEqual(bitsOf(STORED));
    expect(JSON.stringify(model)).toBe(original);
  });

  itPendingSk1("creates a skewed spring through the form; the stored bits equal the input", async () => {
    const model = await loadPreviewModel();
    const queue = await createThroughForm(model, "spring", "UY", ["1", "1", "0"], () => {
      fireEvent.click(screen.getByText("Add support stiffness"));
      enter("Support stiffness DOF", "UY"); enter("Support stiffness quantity value", "1000"); enter("Support stiffness quantity unit", "N/m");
    });
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const stored = storedAfter(await applyModelOperation(model, queue.mock.calls[0][0], null), "support:skew-new");
    expect(bitsOf(stored.direction!)).toEqual(bitsOf({ x: 1, y: 1, z: 0 }));
  });

  itPendingSk1.each([
    ["the exact route", "line_stop", "UY", ["3", "4", "0"], "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED", true],
    ["the range rule", "line_stop", "UY", ["1", "1.1125369292536007e-308", "0"], "SUPPORT_DIRECTION_RANGE_UNSUPPORTED", false],
    ["the record rule (family)", "guide", "UY", ["3", "4", "0"], "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", true],
    ["the rotational rule", "line_stop", "RX", ["3", "4", "0"], "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED", false],
    ["the zero rule", "line_stop", "UY", ["0", "0", "0"], "SUPPORT_DIRECTION_ZERO", false]
  ] as const)("shows the engine's refusal by %s in place, and queues nothing", async (_rule, family, dof, direction, code, exact) => {
    const model = await loadPreviewModel();
    // Document rules come first: a guide on the exact route is the record rule's refusal.
    if (exact) Object.assign(model, { pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3" } });
    const queue = await createThroughForm(model, family, dof, [...direction]);
    const finding = await screen.findByTestId(`support-direction-engine-diagnostic-${code}`);
    expect(finding).toHaveTextContent(code);
    if (family === "guide") expect(screen.queryByTestId("support-direction-engine-diagnostic-SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
  });

  itPendingSk1("(a) and (b) through update_support on an engine-created skewed stop", async () => {
    const model = await loadPreviewModel();
    const queue = await createThroughForm(model, "line_stop", "UY", ["0.1", "-0.2", "5e-300"]);
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const created = (await applyModelOperation(model, queue.mock.calls[0][0], null)).applied_model!;
    cleanup();
    const moved = await queueFormEdit(created, "support:skew-new", () => { fireEvent.click(screen.getByLabelText("Restrain UY")); fireEvent.click(screen.getByLabelText("Restrain UZ")); });
    expect(bitsOf(storedAfter(await applyModelOperation(created, moved, null), "support:skew-new").direction!)).toEqual(bitsOf(STORED));
    expectRefused(await applyModelOperation(created, await queueFormEdit(created, "support:skew-new", () => enter("Support family", "anchor")), null), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
  });
});
