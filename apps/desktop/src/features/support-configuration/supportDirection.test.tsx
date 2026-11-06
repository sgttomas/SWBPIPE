/** R-6/R-7 S-K1 desktop entry (DEL-07-02 Work `support-direction.md`): the support creation
 * form's optional Direction (x, y, z), its payload, the stored-direction display, and the
 * payload of every existing support edit path on a record with a stored `direction`. The
 * hash and operation services are stand-ins here; the engine-backed checks are in
 * `supportDirectionEngine.test.tsx`. Invented fixtures only. */
import { changeFormControl } from "../../test-support/workspaceTestControls";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SupportConfigurationForm } from "./SupportConfigurationForm";
import { DIRECTION_REFUSAL, parseDirectionEntry, STORED_DIRECTION_NOTE, storedDirectionText } from "./supportDirection";
import { validateModelOperation } from "../../services/operationService";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import { buildHangerSelectionBatch, type HangerDocument } from "../hanger-selection/hangerSelection";
import { buildGridOperationIntent, type GridColumn } from "../workspace/table/modelTableAdapter";
import { selectedProperties } from "../model-workspace/modelView";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import { stableJson } from "../model-data/modelDataIntents";
import fixture from "../../../../../fixtures/hanger/invented_hanger_library_valid.json";
import type { EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";

vi.mock("../../services/hashService", () => ({
  canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)),
  computeModelHash: vi.fn(async () => "sha256:test"),
  canonicalSha256Hex: vi.fn(async () => "0".repeat(64))
}));
vi.mock("../../services/operationService", () => ({ validateModelOperation: vi.fn() }));

const accepted = (): OperationOutcome => ({ validation: { application_status: "not_applied" }, diagnostics: [], applied_model: null } as unknown as OperationOutcome);
const refusedBy = (code: string): OperationOutcome => ({
  validation: { application_status: "blocked" },
  diagnostics: [{ id: `diagnostic:${code}`, code, severity: "blocking", message: `supports[1].direction of support \`support:new\` is refused (${code}).`, affected_refs: ["support:new"] }],
  applied_model: null
} as unknown as OperationOutcome);

beforeEach(() => { vi.mocked(validateModelOperation).mockReset(); vi.mocked(validateModelOperation).mockResolvedValue(accepted()); });
afterEach(cleanup);

const STORED = { x: 0.1, y: -0.2, z: 5e-300 };
function model(): PreviewModel {
  return {
    schema_version: "test", document_kind: "test", data_boundary: {},
    project: { id: "project:test", name: "Invented", description: "Original synthetic test data", units: {} },
    analysis_status: { mechanics: "not_run", rule_check: "not_run", professional_acceptance: "not_run" },
    nodes: [{ id: "node:a", label: "A", provenance: "invented", position: { x: 0, y: 0, z: 0 } }],
    supports: [{ id: "support:skew", label: "Skew stop", node: "node:a", family: "line_stop", restraints: ["UY"], direction: { ...STORED }, provenance: "invented" }],
    materials: [], pipe_segments: [], components: [], diagnostics: [], load_cases: []
  } as unknown as PreviewModel;
}
const enter = (label: string, value: string) => changeFormControl(screen.getByLabelText(label), { target: { value } });
function fillCreation(family = "line_stop", dof = "UY") {
  enter("Support ID", "support:new"); enter("Support label", "Invented stop"); enter("Support provenance", "original test");
  enter("Support family", family);
  fireEvent.click(screen.getByLabelText(`Restrain ${dof}`));
}
function renderCreate(queue = vi.fn(), m = model()) {
  render(<SupportConfigurationForm model={m} selection={{ type: "node", id: "node:a" }} onQueueIntent={queue} />);
  return queue;
}
const directionInputs = () => (["x", "y", "z"] as const).map(axis => screen.getByLabelText(`Direction ${axis}`) as HTMLInputElement);
const queuedAfter = (queue: ReturnType<typeof vi.fn>) => JSON.parse((queue.mock.calls[0][0] as EditorOperationIntent).change.after);

describe("the entered direction", () => {
  it("is absent when every component is blank, and the typed numbers otherwise", () => {
    expect(parseDirectionEntry({ x: "", y: " ", z: "" })).toBeNull();
    expect(parseDirectionEntry({ x: "0.10", y: " -4 ", z: "5e-300" })).toEqual({ x: 0.1, y: -4, z: 5e-300 });
    // The typed binary64 values, bit for bit (2^-1023 stays subnormal: range is the engine's rule).
    const typed = parseDirectionEntry({ x: "0.1", y: "1.1125369292536007e-308", z: "3" })!;
    const bits = (v: number) => { const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, v); return view.getBigUint64(0); };
    expect([bits(typed.x), bits(typed.y), bits(typed.z)]).toEqual([bits(0.1), bits(2 ** -1023), bits(3)]);
  });

  it("refuses a partial vector, a non-number and -0 by name, and never fills a blank with zero", () => {
    expect(() => parseDirectionEntry({ x: "1", y: "", z: "" })).toThrow(DIRECTION_REFUSAL.partial(["y", "z"]));
    expect(() => parseDirectionEntry({ x: "1", y: "2", z: "" })).toThrow(DIRECTION_REFUSAL.partial(["z"]));
    for (const text of ["Infinity", "0x10", "1 m", "1e400", "NaN"]) expect(() => parseDirectionEntry({ x: text, y: "1", z: "0" })).toThrow(DIRECTION_REFUSAL.notNumber("x"));
    expect(() => parseDirectionEntry({ x: "1", y: "-0", z: "0" })).toThrow(DIRECTION_REFUSAL.negativeZero("y"));
  });

  it("is shown as stored, component by component", () => {
    expect(storedDirectionText({ direction: { x: 0.1, y: -0.2, z: 5e-300 } })).toBe("(0.1, -0.2, 5e-300)");
    expect(storedDirectionText({ direction: { z: 0, x: 3, y: -0 } })).toBe("(3, -0, 0)");
    expect(storedDirectionText({ direction: { x: 1, y: 2 } })).toBe('{"x":1,"y":2}');
    expect(storedDirectionText({ direction: null })).toBe("null");
    expect(storedDirectionText({ restraints: ["UY"] })).toBeNull();
  });
});

describe("support creation with a direction", () => {
  it("is never prefilled; empty writes no direction member and needs no engine check", async () => {
    const queue = renderCreate();
    for (const input of directionInputs()) expect(input.value).toBe("");
    fillCreation();
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const after = queuedAfter(queue);
    expect(after).toEqual({ id: "support:new", label: "Invented stop", node: "node:a", family: "line_stop", restraints: ["UY"], provenance: "original test" });
    expect(after).not.toHaveProperty("direction");
    expect(validateModelOperation).not.toHaveBeenCalled();
  });

  it("writes exactly the entered members, keeps the typed text, and is engine-checked before queueing", async () => {
    const queue = renderCreate();
    fillCreation();
    enter("Direction x", "0.10"); enter("Direction y", "0.2"); enter("Direction z", "0");
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0] as EditorOperationIntent;
    expect(intent.change).toMatchObject({ change_kind: "create_support", field_path: "supports", before: "not_present", unit: "none", dimension: "dimensionless" });
    expect(intent.operation_kind).toBe("create");
    const after = JSON.parse(intent.change.after);
    expect(Object.keys(after).sort()).toEqual(["direction", "family", "id", "label", "node", "provenance", "restraints"]);
    expect(after.direction).toEqual({ x: 0.1, y: 0.2, z: 0 });
    for (const axis of ["x", "y", "z"] as const) expect(Object.is(after.direction[axis], Number({ x: "0.10", y: "0.2", z: "0" }[axis]))).toBe(true);
    // Never normalized: (0.1, 0.2, 0) is not a unit vector and is written as typed.
    expect(Math.hypot(after.direction.x, after.direction.y, after.direction.z)).not.toBe(1);
    expect(directionInputs().map(input => input.value)).toEqual(["0.10", "0.2", "0"]);
    expect(validateModelOperation).toHaveBeenCalledOnce();
    expect(vi.mocked(validateModelOperation).mock.calls[0][1]).toBe(intent);
    expect(screen.getByTestId("support-direction-engine-notice")).toHaveTextContent("checked by the operation engine");
  });

  it("refuses a partial vector before queueing, with no engine call", async () => {
    const queue = renderCreate();
    fillCreation();
    enter("Direction x", "3");
    fireEvent.click(screen.getByText("Queue support creation"));
    expect(await screen.findByRole("alert")).toHaveTextContent(DIRECTION_REFUSAL.partial(["y", "z"]));
    expect(queue).not.toHaveBeenCalled();
    expect(validateModelOperation).not.toHaveBeenCalled();
  });

  // The form does not judge the family, DOF or nonlinear rules: an inadmissible record is
  // sent to the engine and its refusal is shown in place by name; nothing is queued.
  it.each([
    ["guide", "UY", "SUPPORT_DIRECTION_RECORD_UNSUPPORTED"],
    ["line_stop", "RX", "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED"],
    ["line_stop", "UY", "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED"],
    ["spring", "UY", "SUPPORT_DIRECTION_RANGE_UNSUPPORTED"]
  ])("shows the engine's refusal for %s on %s by name (%s)", async (family, dof, code) => {
    vi.mocked(validateModelOperation).mockResolvedValue(refusedBy(code));
    const queue = renderCreate();
    fillCreation(family, dof);
    enter("Direction x", "3"); enter("Direction y", "4"); enter("Direction z", "0");
    fireEvent.click(screen.getByText("Queue support creation"));
    const finding = await screen.findByTestId(`support-direction-engine-diagnostic-${code}`);
    expect(finding).toHaveTextContent(`Refused: ${code}`);
    expect(finding).toHaveAttribute("data-severity", "blocking");
    expect(screen.getByTestId("support-direction-engine-error")).toHaveTextContent("refused by the operation engine; nothing was queued");
    expect(queue).not.toHaveBeenCalled();
  });

  it("is not offered when editing an existing support, and starts empty for a new one from it", () => {
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:skew" }} onQueueIntent={vi.fn()} />);
    expect(screen.queryByLabelText("Direction x")).toBeNull();
    fireEvent.click(screen.getByLabelText("Create a new support"));
    for (const input of directionInputs()) expect(input.value).toBe("");
    expect(screen.queryByTestId("support-direction-stored")).toBeNull();
  });

  it("labels its inputs and keeps them in the keyboard order", () => {
    renderCreate();
    const group = screen.getByRole("group", { name: "Direction (x, y, z)" });
    expect(group).toHaveAccessibleDescription(/never normalized/);
    const inputs = directionInputs();
    for (const input of inputs) {
      expect(within(group).getByLabelText(input.getAttribute("aria-label")!)).toBe(input);
      expect(input.tabIndex).toBe(0);
      expect(input).toHaveAttribute("inputmode", "decimal");
      input.focus();
      expect(document.activeElement).toBe(input);
    }
    const queueButton = screen.getByText("Queue support creation");
    const follows = (a: Element, b: Element) => Boolean(a.compareDocumentPosition(b) & Node.DOCUMENT_POSITION_FOLLOWING);
    expect(follows(inputs[0], inputs[1]) && follows(inputs[1], inputs[2]) && follows(inputs[2], queueButton)).toBe(true);
  });
});

describe("a stored direction", () => {
  it("is shown read-only exactly as stored in the support configuration and the inspector", () => {
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:skew" }} onQueueIntent={vi.fn()} />);
    const shown = screen.getByTestId("support-direction-stored");
    expect(within(shown).getByLabelText("Stored direction")).toHaveTextContent("(0.1, -0.2, 5e-300)");
    expect(shown).toHaveTextContent(STORED_DIRECTION_NOTE);
    expect(within(shown).queryByRole("textbox")).toBeNull();
    expect(selectedProperties(model(), { type: "support", id: "support:skew" })).toContainEqual([`Direction (x, y, z) — ${STORED_DIRECTION_NOTE}`, "(0.1, -0.2, 5e-300)"]);
    const plain = model(); delete (plain.supports[0] as { direction?: unknown }).direction;
    expect(selectedProperties(plain, { type: "support", id: "support:skew" }).some(([label]) => label.startsWith("Direction"))).toBe(false);
  });

  // Each edit path's payload leaves `direction` out, so the applier's member-wise update keeps
  // it (`update_support configuration` replaces only CONFIG; a field path only its member).
  it("is left out of the support configuration form's update_support payload", async () => {
    const m = model(), original = JSON.stringify(m), queue = vi.fn();
    render(<SupportConfigurationForm model={m} selection={{ type: "support", id: "support:skew" }} onQueueIntent={queue} />);
    enter("Support provenance", "revised");
    fireEvent.click(screen.getByLabelText("Restrain UY"));
    fireEvent.click(screen.getByLabelText("Restrain UX"));
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0] as EditorOperationIntent;
    expect(intent.change).toMatchObject({ change_kind: "update_support", field_path: "configuration" });
    expect(JSON.parse(intent.change.before)).toEqual({ family: "line_stop", restraints: ["UY"], provenance: "invented" });
    expect(JSON.parse(intent.change.after)).toEqual({ family: "line_stop", restraints: ["UX"], provenance: "revised" });
    expect(intent.change.after).not.toContain("direction");
    expect(validateModelOperation).not.toHaveBeenCalled();
    expect(JSON.stringify(m)).toBe(original);
  });

  it("is left out of bulk change's support-kind payload", async () => {
    const m = bulkChangeModel();
    Object.assign(m.supports[1], { family: "line_stop", restraints: ["UY"], direction: { ...STORED } });
    const built = await buildBulkChangeBatch(m, [{ type: "support", id: "support:B" }], { kind: "support-kind", fromFamily: "line_stop", toFamily: "guide", restraints: "keep" }, async v => `canon:${stableJson(v)}`, "bulk-direction");
    expect(built.operations).toHaveLength(1);
    const change = built.operations[0].change;
    expect(change).toMatchObject({ change_kind: "update_support", field_path: "configuration" });
    expect(change.before).toBe(`canon:${stableJson({ family: "line_stop", restraints: ["UY"], provenance: "invented B2 bulk-change test input" })}`);
    expect(JSON.parse(change.after)).toEqual({ family: "guide", restraints: ["UY"], provenance: "invented B2 bulk-change test input" });
    expect(m.supports[1].direction).toEqual(STORED);
  });

  it("is left out of hanger selection's update_support payload", async () => {
    const m = model();
    const doc = structuredClone(fixture) as HangerDocument;
    const batch = await buildHangerSelectionBatch(m, doc, doc.hanger_records[0].hanger_id, { supportId: "support:skew", restraints: ["UY"] });
    const config = batch.operations[0].change;
    expect(config).toMatchObject({ change_kind: "update_support", field_path: "configuration" });
    expect(JSON.parse(config.before)).toEqual({ family: "line_stop", restraints: ["UY"], provenance: "invented" });
    expect(Object.keys(JSON.parse(config.after)).sort()).toEqual(["family", "hanger", "provenance", "restraints"]);
    for (const operation of batch.operations) expect(operation.change.after).not.toContain("direction");
  });

  it("is left out of the support grid's cell payloads", () => {
    const m = model();
    const row = { id: "support:skew", label: "Skew stop", type: "support" as const, searchText: "", raw: m.supports[0] };
    for (const [fieldPath, value] of [["label", "Renamed"], ["node", "node:a"], ["restraints", "UX"], ["provenance", "revised"]]) {
      const column: GridColumn = { key: fieldPath, label: fieldPath, fieldPath, objectType: "Support", changeKind: "update_support", dimension: "dimensionless", sourceNote: "test", unit: () => "none", value: () => String((m.supports[0] as unknown as Record<string, unknown>)[fieldPath]) };
      const intent = buildGridOperationIntent({ row, column, model: m, sequence: 1, value, interaction: "cell" });
      expect(intent.change).toMatchObject({ change_kind: "update_support", field_path: fieldPath, after: value });
    }
  });
});
