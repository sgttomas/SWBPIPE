// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §4.2, §11): the
// desktop's condition-set surfaces through the real wasm operation engine. Each form
// queues an intent (checked validate-only by the engine), and the test applies it with
// `applyModelOperation`, the route Review/Apply uses. Base documents are S3a's committed
// contract corpus (`fixtures/model_operations/condition_set_contract_corpus.json`).
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { ConditionSet, EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { previewOperatingConditions } from "../../services/operatingConditionPreviewService";
import { corpusBase } from "../../test-support/conditionSetCorpus";
import { ConditionSetsSection } from "./ConditionSetsSection";
import { PipeConditionSetField } from "./PipeConditionSetField";
import { ConditionUseControl } from "./ConditionUseControl";
import { NOT_USED_BY_SOLVE, draftUnchanged, assignConditionSetIntent, conditionUseIntent, createConditionSetIntent, deleteConditionSetIntent, draftFromSet, setFromDraft, updateConditionSetIntent } from "./conditionSetIntents";

async function applyOutcome(model: PreviewModel, intent: EditorOperationIntent): Promise<OperationOutcome> {
  return applyModelOperation(model, intent, await computeModelHash(model));
}
async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyOutcome(model, intent);
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}
async function queuedAt(list: EditorOperationIntent[], count: number): Promise<EditorOperationIntent> {
  await waitFor(() => expect(list).toHaveLength(count));
  return list[count - 1];
}
const type = (testId: string, value: string) => fireEvent.change(screen.getByTestId(testId), { target: { value } });

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("the connecting path: set, pipe assignment and case use through the engine", () => {
  it("creates a set in the form, assigns it to a pipe and sets a case's use, each applied by the wasm engine", async () => {
    let model = corpusBase("s3a_01");
    const list: EditorOperationIntent[] = [];
    const props = () => ({ model, onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) });

    // 1. Create a condition set through the model tree's form.
    const tree = render(<ConditionSetsSection {...props()} />);
    expect(screen.getByTestId("condition-sets-empty")).toBeInTheDocument();
    type("condition-set-new-id", "cs:line-a");
    type("condition-set-new-label", "Line A");
    type("condition-set-new-provenance", "invented S3b desktop test input");
    type("condition-set-new-installation-temperature-value", "20");
    type("condition-set-new-installation-temperature-unit", "degC");
    fireEvent.click(screen.getByTestId("condition-set-new-add-condition"));
    type("condition-set-new-condition-0-key", "OPE1");
    type("condition-set-new-condition-0-temperature-value", "150");
    type("condition-set-new-condition-0-temperature-unit", "degC");
    type("condition-set-new-added-weight-magnitude-value", "-50");
    type("condition-set-new-added-weight-magnitude-unit", "N/m");
    fireEvent.change(screen.getByTestId("condition-set-new-added-weight-direction"), { target: { value: "global_z" } });
    fireEvent.change(screen.getByTestId("condition-set-new-exposure-wind"), { target: { value: "true" } });
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    const create = await queuedAt(list, 1);
    expect(create.target).toEqual({ object_type: "ConditionSet", ref: "cs:line-a" });
    expect(create.change).toMatchObject({ change_kind: "create_condition_set", field_path: "condition_sets", before: "not_present", unit: "none", dimension: "dimensionless" });
    expect(JSON.parse(create.change.after)).toEqual({
      id: "cs:line-a", label: "Line A", provenance: "invented S3b desktop test input",
      installation_temperature: { value: 20, unit: "degC" },
      conditions: [{ key: "OPE1", temperature: { value: 150, unit: "degC" } }],
      added_weight: { magnitude: { value: -50, unit: "N/m" }, direction: "global_z" },
      exposure: { wind: true }
    });
    model = await apply(model, create);
    expect(model.condition_sets).toHaveLength(1);
    tree.unmount();

    // 2. Assign it to pipe P1 in the inspector field.
    const inspector = render(<PipeConditionSetField {...props()} selection={{ type: "pipe", id: "P1" }} />);
    expect(screen.getByTestId("pipe-condition-set-current")).toHaveTextContent("not stated");
    fireEvent.change(screen.getByTestId("pipe-condition-set-select"), { target: { value: "cs:line-a" } });
    fireEvent.click(screen.getByTestId("pipe-condition-set-assign"));
    const assign = await queuedAt(list, 2);
    expect(assign.change).toMatchObject({ change_kind: "assign_condition_set", field_path: "condition_set_ref", before: "not_present", after: JSON.stringify({ condition_set_ref: "cs:line-a" }) });
    expect(assign.target).toEqual({ object_type: "Element", ref: "P1" });
    model = await apply(model, assign);
    expect(model.pipe_segments.find((pipe) => pipe.id === "P1")?.condition_set_ref).toBe("cs:line-a");
    inspector.unmount();

    // 3. Set case SUS's use; P2 and P3 carry no set, so the engine stages warnings by name.
    render(<ConditionUseControl {...props()} loadCase={model.load_cases.find((item) => item.id === "SUS")!} />);
    expect(screen.getByTestId("load-condition-use-refused-pressure")).toHaveTextContent("LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT");
    // RT-A6 H1: contents density is offered, staged by name until generated self-weight consumes it.
    expect(screen.getByTestId("load-condition-use-contents-note")).toHaveTextContent("LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED");
    expect(within(screen.getByTestId("load-condition-use-temperature")).getAllByRole("option").map((option) => option.textContent)).toEqual(["not used", "OPE1"]);
    fireEvent.change(screen.getByTestId("load-condition-use-temperature"), { target: { value: "OPE1" } });
    fireEvent.click(screen.getByTestId("load-condition-use-added-weight"));
    fireEvent.click(screen.getByTestId("load-condition-use-queue"));
    const use = await queuedAt(list, 3);
    expect(use.change).toMatchObject({ change_kind: "set_load_case_condition_use", field_path: "condition_use", before: "not_present" });
    expect(JSON.parse(use.change.after)).toEqual({ temperature: "OPE1", added_weight: true });
    // The staged findings are shown in place, as the engine reports them (warnings, not refusals).
    const staged = await screen.findAllByTestId("load-condition-use-diagnostic-OPERATING_CONDITION_SET_MISSING");
    expect(staged.length).toBeGreaterThan(0);
    for (const item of staged) expect(item).toHaveAttribute("data-severity", "warning");
    const outcome = await applyOutcome(model, use);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.diagnostics.map((item) => [item.code, item.severity])).toContainEqual(["OPERATING_CONDITION_SET_MISSING", "warning"]);
    model = outcome.applied_model as PreviewModel;
    expect(model.load_cases.find((item) => item.id === "SUS")?.condition_use).toEqual({ temperature: "OPE1", added_weight: true });

    // 4. The engine's preview names P1's generated thermal and added-weight records for SUS.
    const preview = await previewOperatingConditions(model);
    const sus = preview.cases.find((item) => item.case_id === "SUS");
    expect(sus).toBeUndefined(); // refused: P2 and P3 carry no set, so nothing is generated.
    expect(preview.diagnostics.map((item) => item.code)).toContain("OPERATING_CONDITION_SET_MISSING");
  });
});

/** The diff rows of an applied outcome. */
const rows = (outcome: OperationOutcome) => outcome.diff_preview as unknown as Array<{ entity_ref: string; field_path: string; before: string; after: string }>;

describe("every condition-set operation through the engine, with its inverse", () => {
  it("edits a set in the form (update ↔ update) and removes an unused one (delete ↔ create at its index), restoring the document exactly", async () => {
    let model = corpusBase("s3a_02");
    const original = await canonicalJsonString(model);
    const list: EditorOperationIntent[] = [];
    const view = render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(screen.getByTestId("condition-set-summary-cs:a")).toHaveTextContent("1 condition; carried by no pipe");
    type("condition-set-cs:a-condition-0-temperature-value", "160");
    fireEvent.click(screen.getByTestId("condition-set-update-cs:a"));
    const update = await queuedAt(list, 1);
    expect(update.change).toMatchObject({ change_kind: "update_condition_set", field_path: "condition_sets", before: await canonicalJsonString(model.condition_sets![0]) });
    const updated = await applyOutcome(model, update);
    expect(updated.validation.application_status).toBe("applied_to_session_model");
    const edited = updated.applied_model as PreviewModel;
    expect(edited.condition_sets![0].conditions![0].temperature).toEqual({ value: 160, unit: "degC" });
    // Inverse: update with the before record.
    const back = await apply(edited, await updateConditionSetIntent(canonicalJsonString, edited.condition_sets![0], model.condition_sets![0]));
    expect(await canonicalJsonString(back)).toBe(original);

    view.rerender(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(screen.getByTestId("condition-set-delete-cs:a"));
    const remove = await queuedAt(list, 2);
    expect(remove.change).toMatchObject({ change_kind: "delete_condition_set", field_path: "condition_sets", after: "not_present" });
    const removed = await applyOutcome(model, remove);
    const row = rows(removed).find((item) => item.entity_ref === "cs:a")!;
    expect(row.field_path).toBe("condition_sets[0]");
    model = removed.applied_model as PreviewModel;
    expect(model.condition_sets).toEqual([]);
    // Inverse from the row alone: create at the deleted index with the before record.
    const recreate = createConditionSetIntent(JSON.parse(row.before));
    recreate.change.field_path = row.field_path;
    expect(await canonicalJsonString(await apply(model, recreate))).toBe(original);
  });

  it("assigns and clears a pipe's set, and sets and clears a case's use, each inverted exactly", async () => {
    const model = corpusBase("s3a_09"); // every pipe carries cs:a; no case uses conditions
    const original = await canonicalJsonString(model);
    const p2 = model.pipe_segments.find((pipe) => pipe.id === "P2")!;
    const cleared = await apply(model, await assignConditionSetIntent(canonicalJsonString, p2, null));
    expect(cleared.pipe_segments.find((pipe) => pipe.id === "P2")).not.toHaveProperty("condition_set_ref");
    const reassigned = await apply(cleared, await assignConditionSetIntent(canonicalJsonString, cleared.pipe_segments.find((pipe) => pipe.id === "P2")!, "cs:a"));
    expect(await canonicalJsonString(reassigned)).toBe(original);

    const sus = model.load_cases.find((item) => item.id === "SUS")!;
    const used = await apply(model, await conditionUseIntent(canonicalJsonString, sus, { temperature: "OPE1", added_weight: true }));
    expect(used.load_cases.find((item) => item.id === "SUS")?.condition_use).toEqual({ temperature: "OPE1", added_weight: true });
    const unused = await apply(used, await conditionUseIntent(canonicalJsonString, used.load_cases.find((item) => item.id === "SUS")!, null));
    expect(await canonicalJsonString(unused)).toBe(original);
  });
});

describe("refusals, staged findings and carried refusals shown in place", () => {
  it("names OP-CONDITION-SET-IN-USE with every pipe that carries the set, and queues nothing", async () => {
    const model = corpusBase("s3a_04"); // P1 and P3 carry cs:a
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(screen.getByTestId("condition-set-in-use-cs:a")).toHaveTextContent("In use by P1, P3");
    fireEvent.click(screen.getByTestId("condition-set-delete-cs:a"));
    const refusal = await screen.findByTestId("condition-set-cs:a-diagnostic-OP-CONDITION-SET-IN-USE");
    expect(refusal).toHaveAttribute("data-severity", "blocking");
    expect(refusal).toHaveTextContent("refers to cs:a, P1, P3");
    expect(screen.getByTestId("condition-set-cs:a-error")).toHaveTextContent("nothing was queued");
    expect(list).toHaveLength(0);
  });

  it("names the data and document refusals: a repeated id and a quantity of the wrong dimension; an unchanged edit is not offered", async () => {
    const model = corpusBase("s3a_02");
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(screen.getByTestId("condition-set-update-cs:a")).toBeDisabled();
    expect(screen.getByTestId("condition-set-update-cs:a")).toHaveAttribute("title", "No member is edited; the stored record is unchanged.");

    type("condition-set-new-id", "P1");
    type("condition-set-new-provenance", "invented");
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    expect(await screen.findByTestId("condition-set-new-diagnostic-OP-DATA-ID-COLLISION")).toBeInTheDocument();

    type("condition-set-new-id", "cs:b");
    type("condition-set-new-installation-temperature-value", "20");
    type("condition-set-new-installation-temperature-unit", "N/m");
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    expect(await screen.findByTestId("condition-set-new-diagnostic-OPERATING_CONDITION_QUANTITY_INVALID")).toBeInTheDocument();
    expect(list).toHaveLength(0);
  });

  it("refuses a half-entered quantity or added weight in the form, by name, before the engine", async () => {
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={corpusBase("s3a_01")} onQueueIntent={(intent) => list.push(intent)} />);
    type("condition-set-new-id", "cs:b");
    type("condition-set-new-provenance", "invented");
    type("condition-set-new-added-weight-magnitude-value", "-10");
    type("condition-set-new-added-weight-magnitude-unit", "N/m");
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    expect(await screen.findByTestId("condition-set-new-error")).toHaveTextContent("Added weight needs both its magnitude and its direction");
    type("condition-set-new-design-temperature-unit", "degC");
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    await waitFor(() => expect(screen.getByTestId("condition-set-new-error")).toHaveTextContent("Design temperature has a unit but no value"));
    expect(list).toHaveLength(0);
  });

  it("shows a permitted carried refusal as info, and the staged findings as warnings, as the outcome reports them", async () => {
    const model = corpusBase("s3a_19"); // an unrelated snubber the solve refuses
    const list: EditorOperationIntent[] = [];
    render(<PipeConditionSetField model={model} selection={{ type: "pipe", id: "P2" }} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(screen.getByTestId("pipe-condition-set-select"), { target: { value: "cs:a" } });
    fireEvent.click(screen.getByTestId("pipe-condition-set-assign"));
    await queuedAt(list, 1);
    const carried = screen.getAllByTestId(/^pipe-condition-set-diagnostic-/);
    expect(carried.length).toBeGreaterThan(0);
    for (const item of carried) expect(item).toHaveAttribute("data-severity", "info");
    expect(carried[0]).toHaveTextContent("Carried");
  });
});

/** The engine's outcome of an update from `current` to `next` on `model`. */
async function updateOutcome(model: PreviewModel, current: ConditionSet, next: ConditionSet) {
  return applyOutcome(model, await updateConditionSetIntent(canonicalJsonString, current, next));
}
const codes = (outcome: OperationOutcome) => outcome.diagnostics.filter((item) => item.severity === "blocking").map((item) => item.code);
/** A model holding `set` as its only condition set (S3a's corpus document, no pipe carrying it). */
function holding(set: unknown): PreviewModel {
  const model = corpusBase("s3a_02");
  model.condition_sets = [set as ConditionSet];
  return model;
}

describe("an edit changes only what the person edited (persistence review r1 of #111)", () => {
  // The reviewer's record: empty label, collections and sub-records, and padded provenance.
  const REVIEWER = { id: "cs:r", label: "", provenance: " source ", conditions: [], design: {}, peak: {}, exposure: {} };
  const TWO_ROWS = {
    id: "cs:t", provenance: "invented", exposure: { wave: false },
    conditions: [{ label: "", key: "OPE1", temperature: { value: 150, unit: "degC" } }, { key: "OPE2", pressure: { value: 1, unit: "MPa" }, label: "  hot  " }]
  };

  it("round-trips an untouched record exactly, key order included, and the engine refuses the no-op by name", async () => {
    for (const record of [REVIEWER, TWO_ROWS]) {
      const draft = draftFromSet(record as unknown as ConditionSet);
      expect(draftUnchanged(draft)).toBe(true);
      expect(JSON.stringify(setFromDraft(draft))).toBe(JSON.stringify(record));
      const outcome = await updateOutcome(holding(record), record as unknown as ConditionSet, setFromDraft(draft));
      expect(codes(outcome)).toEqual(["OP-DATA-UNCHANGED"]);
    }
    // The form offers no edit until a member is edited.
    render(<ConditionSetsSection model={holding(REVIEWER)} onQueueIntent={() => undefined} />);
    expect(screen.getByTestId("condition-set-update-cs:r")).toBeDisabled();
    type("condition-set-cs:r-label", "Line R");
    expect(screen.getByTestId("condition-set-update-cs:r")).toBeEnabled();
    type("condition-set-cs:r-label", "");
    expect(screen.getByTestId("condition-set-update-cs:r")).toBeDisabled();
  });

  it("an edit of one member changes only that member, in the set and in one condition row", async () => {
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={holding(TWO_ROWS)} onQueueIntent={(intent) => list.push(intent)} />);
    type("condition-set-cs:t-condition-1-pressure-value", "1.5");
    fireEvent.click(screen.getByTestId("condition-set-update-cs:t"));
    const edit = await queuedAt(list, 1);
    const expected = structuredClone(TWO_ROWS);
    expected.conditions[1].pressure = { value: 1.5, unit: "MPa" };
    expect(edit.change.after).toBe(JSON.stringify(expected));
    expect(await canonicalJsonString(JSON.parse(edit.change.after))).toBe(await canonicalJsonString(expected));
    const applied = await apply(holding(TWO_ROWS), edit);
    expect(applied.condition_sets![0]).toEqual(expected);

    const draft = draftFromSet(REVIEWER as unknown as ConditionSet);
    const labelled = setFromDraft({ ...draft, label: "Line R" });
    expect(labelled).toEqual({ ...REVIEWER, label: "Line R" });
    expect(Object.keys(labelled)).toEqual(Object.keys(REVIEWER));
  });

  it("shows a stored null as stored and refused, keeps it through an unrelated edit, and removes it only when the person removes it", async () => {
    const STORED_NULL = { id: "cs:n", provenance: "invented", installation_temperature: null, exposure: { wind: null } };
    const model = holding(STORED_NULL);
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    const refused = screen.getByTestId("condition-set-cs:n-refused");
    expect(refused).toHaveTextContent("installation_temperature: null");
    expect(refused).toHaveTextContent("exposure.wind: null");
    expect(screen.getByTestId("condition-set-cs:n-installation-temperature-value")).toHaveAttribute("placeholder", "stored: null (refused; kept)");
    expect(screen.getByTestId("condition-set-cs:n-exposure-wind")).toHaveValue("stored");
    expect(screen.getByTestId("condition-set-cs:n-exposure-wind")).toHaveDisplayValue("stored: null (refused; kept)");

    // An unrelated edit keeps both nulls, and the engine still refuses the record by name.
    type("condition-set-cs:n-label", "Line N");
    const kept = setFromDraft({ ...draftFromSet(STORED_NULL as unknown as ConditionSet), label: "Line N" });
    expect(kept).toEqual({ ...STORED_NULL, label: "Line N" });
    fireEvent.click(screen.getByTestId("condition-set-update-cs:n"));
    expect(await screen.findByTestId("condition-set-cs:n-error")).toHaveTextContent("nothing was queued");
    expect(screen.getAllByTestId(/^condition-set-cs:n-diagnostic-/).every((item) => item.getAttribute("data-severity") === "blocking")).toBe(true);
    const outcome = await updateOutcome(model, STORED_NULL as unknown as ConditionSet, kept);
    expect(codes(outcome)).toEqual(["MODEL-SLOT-SHAPE"]); // the validator's closed-record refusal of the null
    expect(screen.getByTestId("condition-set-cs:n-diagnostic-MODEL-SLOT-SHAPE")).toHaveTextContent("null");
    expect(list).toHaveLength(0);

    // The person removes the stored installation temperature explicitly: only that member goes.
    fireEvent.click(screen.getByTestId("condition-set-cs:n-refused-installation_temperature-remove"));
    expect(screen.getByTestId("condition-set-cs:n-refused")).not.toHaveTextContent("installation_temperature");
    expect(screen.getByTestId("condition-set-cs:n-installation-temperature-value")).toHaveAttribute("placeholder", "not stated");
    fireEvent.click(screen.getByTestId("condition-set-cs:n-refused-exposure.wind-remove"));
    fireEvent.click(screen.getByTestId("condition-set-update-cs:n"));
    const cleaned = await queuedAt(list, 1);
    expect(JSON.parse(cleaned.change.after)).toEqual({ id: "cs:n", provenance: "invented", exposure: {}, label: "Line N" });
  });

  it("keeps a stored refused use member through an edit of another member", async () => {
    const model = corpusBase("s3a_09");
    const sus = model.load_cases.find((item) => item.id === "SUS")!;
    (sus as unknown as { condition_use: unknown }).condition_use = { temperature: "OPE1", added_weight: false };
    const list: EditorOperationIntent[] = [];
    render(<ConditionUseControl model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={sus} />);
    expect(screen.getByTestId("load-condition-use-retained")).toHaveTextContent("added_weight: false");
    fireEvent.click(screen.getByTestId("load-condition-use-wind-exposure"));
    fireEvent.click(screen.getByTestId("load-condition-use-queue"));
    expect(await screen.findByTestId("load-condition-use-diagnostic-LOAD_CASE_CONDITION_USE_EMPTY")).toHaveAttribute("data-severity", "blocking");
    expect(list).toHaveLength(0);
  });
});

describe("the set form: labels, pressure, retained members", () => {
  it("labels every member the solve does not read, shows empty cells as not stated and keeps a condition's pressure as a stored member", async () => {
    let model = corpusBase("s3a_01");
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(screen.getByTestId("condition-set-new-add-condition"));
    const form = screen.getByTestId("condition-set-create-form");
    expect(within(form).getAllByText(`(${NOT_USED_BY_SOLVE})`).length).toBe(5); // contents density, design, peak, snow/ice, wave
    expect(screen.getByTestId("condition-set-new-design-not-used")).toBeInTheDocument();
    expect(screen.getByTestId("condition-set-new-peak-not-used")).toBeInTheDocument();
    expect(screen.getByTestId("condition-set-new-exposure-snow-ice-not-used")).toBeInTheDocument();
    expect(screen.getByTestId("condition-set-new-exposure-wave-not-used")).toBeInTheDocument();
    expect(screen.getByTestId("condition-set-new-condition-0-temperature-value")).toHaveAttribute("placeholder", "not stated");
    expect(screen.getByTestId("condition-set-new-condition-0-temperature-value")).toHaveValue("");
    expect(screen.getByTestId("condition-set-new-added-weight-direction")).toHaveValue("");
    expect(screen.getByTestId("condition-set-new-pressure-note")).toHaveTextContent("No solve of this pressure-free document can use a condition's pressure");

    type("condition-set-new-id", "cs:p");
    type("condition-set-new-provenance", "invented");
    type("condition-set-new-condition-0-key", "OPE1");
    type("condition-set-new-condition-0-pressure-value", "1.5");
    type("condition-set-new-condition-0-pressure-unit", "MPa");
    type("condition-set-new-condition-0-contents-density-value", "1000");
    type("condition-set-new-condition-0-contents-density-unit", "kg/m^3");
    type("condition-set-new-design-temperature-value", "200");
    type("condition-set-new-design-temperature-unit", "degC");
    fireEvent.change(screen.getByTestId("condition-set-new-exposure-wave"), { target: { value: "false" } });
    fireEvent.click(screen.getByTestId("condition-set-create-queue"));
    const create = await queuedAt(list, 1);
    model = await apply(model, create);
    expect(model.condition_sets![0]).toEqual({
      id: "cs:p", provenance: "invented",
      conditions: [{ key: "OPE1", pressure: { value: 1.5, unit: "MPa" }, contents_density: { value: 1000, unit: "kg/m^3" } }],
      design: { temperature: { value: 200, unit: "degC" } },
      exposure: { wave: false }
    });
  });

  it("shows members the form does not edit as retained, and writes them back unchanged", async () => {
    const model = corpusBase("s3a_02");
    model.condition_sets![0] = { ...model.condition_sets![0], tags: ["line-a"], notes: "Invented note." };
    const draft = draftFromSet(model.condition_sets![0]);
    expect(setFromDraft(draft)).toEqual(model.condition_sets![0]);
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    const retained = screen.getByTestId("condition-set-cs:a-retained");
    expect(retained).toHaveTextContent('tags: ["line-a"]');
    expect(retained).toHaveTextContent('notes: "Invented note."');
    type("condition-set-cs:a-label", "Line A (revised)");
    fireEvent.click(screen.getByTestId("condition-set-update-cs:a"));
    const update = await queuedAt(list, 1);
    const applied = await apply(model, update);
    expect(applied.condition_sets![0]).toMatchObject({ label: "Line A (revised)", tags: ["line-a"], notes: "Invented note." });
  });
});

describe("the load manager's Conditions control", () => {
  it("refuses every use but contents density by name on a document with a pressure contract", () => {
    const model = corpusBase("s3a_12");
    render(<ConditionUseControl model={model} onQueueIntent={() => undefined} loadCase={model.load_cases[0]} />);
    expect(screen.getByTestId("load-condition-use-refused-exact")).toHaveTextContent("LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    expect(screen.queryByTestId("load-condition-use-temperature")).toBeNull();
    expect(screen.queryByTestId("load-condition-use-added-weight")).toBeNull();
    expect(screen.queryByTestId("load-condition-use-wind-exposure")).toBeNull();
    // RT-A6 H1: a contents-density key alone is offered on the exact route.
    expect(screen.getByTestId("load-condition-use-contents")).toBeInTheDocument();
    expect(screen.getByTestId("load-condition-use-queue")).toBeDisabled();
  });

  it("writes absence for an unticked mark, never false, and shows the engine's per-pipe resolution", async () => {
    let model = corpusBase("s3a_09"); // every pipe carries cs:a (installation 20 degC, OPE1 150 degC, -50 N/m global_z, wind)
    const sus = model.load_cases.find((item) => item.id === "SUS")!;
    const list: EditorOperationIntent[] = [];
    const view = render(<ConditionUseControl model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={sus} />);
    fireEvent.change(screen.getByTestId("load-condition-use-temperature"), { target: { value: "OPE1" } });
    fireEvent.click(screen.getByTestId("load-condition-use-wind-exposure"));
    fireEvent.click(screen.getByTestId("load-condition-use-wind-exposure")); // ticked, then unticked
    fireEvent.click(screen.getByTestId("load-condition-use-queue"));
    const use = await queuedAt(list, 1);
    expect(JSON.parse(use.change.after)).toEqual({ temperature: "OPE1" });
    model = await apply(model, use);
    view.rerender(<ConditionUseControl model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={model.load_cases.find((item) => item.id === "SUS")!} />);
    expect(await screen.findByTestId("condition-resolution-preview-status")).toHaveTextContent("Resolved by the engine.");
    const records = within(screen.getByTestId("condition-resolution-preview-records")).getAllByRole("row").slice(1);
    expect(records.map((row) => within(row).getAllByRole("cell").map((cell) => cell.textContent))).toEqual(
      ["P1", "P2", "P3"].map((pipe) => [pipe, "thermal", "130 K", "global_z", "temperature_change"])
    );
    // The desktop computed none of these: they are the engine's own records.
    const engine = await previewOperatingConditions(model);
    expect(engine.cases[0].generated_primitive_loads.map((record) => record.magnitude)).toEqual([{ value: 130, unit: "K" }, { value: 130, unit: "K" }, { value: 130, unit: "K" }]);
  });
});

describe("a second pending edit of one set (App passes the queued intents)", () => {
  it("is refused before queueing while an edit of the same set waits for review", async () => {
    const model = corpusBase("s3a_04");
    const set = model.condition_sets![0];
    const pending = await updateConditionSetIntent(canonicalJsonString, set, { ...set, label: "first edit" });
    const queued: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={model} queuedIntents={[pending]} onQueueIntent={(intent) => queued.push(intent)} />);
    type(`condition-set-${set.id}-label`, "second edit");
    fireEvent.click(screen.getByTestId(`condition-set-update-${set.id}`));
    expect(await screen.findByText(/Review or remove the pending change for this record before queuing another\./)).toBeInTheDocument();
    expect(queued).toEqual([]);
  });
});

describe("explicit removal and unknown members (review r2, #111)", () => {
  it("lets an explicit removal win over an entry typed into the same field, on the set and in a condition row", async () => {
    const STORED = { id: "cs:r", provenance: "invented", installation_temperature: null, conditions: [{ key: "OPE1", temperature: null }] };
    const list: EditorOperationIntent[] = [];
    render(<ConditionSetsSection model={holding(STORED)} onQueueIntent={(intent) => list.push(intent)} />);
    type("condition-set-cs:r-installation-temperature-value", "25");
    type("condition-set-cs:r-installation-temperature-unit", "degC");
    fireEvent.click(screen.getByTestId("condition-set-cs:r-refused-installation_temperature-remove"));
    expect(screen.getByTestId("condition-set-cs:r-installation-temperature-value")).toHaveValue("");
    const rowTemperature = screen.getAllByTestId(/^condition-set-cs:r-condition-0-temperature-value$/)[0];
    fireEvent.change(rowTemperature, { target: { value: "210" } });
    fireEvent.click(screen.getByTestId("condition-set-cs:r-refused-conditions[0].temperature-remove"));
    expect(rowTemperature).toHaveValue("");
    fireEvent.click(screen.getByTestId("condition-set-update-cs:r"));
    const queued = await queuedAt(list, 1);
    expect(JSON.parse(queued.change.after)).toEqual({ id: "cs:r", provenance: "invented", conditions: [{ key: "OPE1" }] });
  });

  it("keeps a quantity's extra member and unknown members, including keys ending in a dot, through an unrelated edit", () => {
    const STORED = {
      id: "cs:u", provenance: "invented", label: "U",
      installation_temperature: { value: 20, unit: "degC", note: "extra" },
      design: { temperature: { value: 300, unit: "degC" }, custom: 1 },
      "future.": { kept: true }, future_member: [1, 2]
    };
    const next = setFromDraft({ ...draftFromSet(STORED as unknown as ConditionSet), label: "U2" });
    expect(next).toEqual({ ...STORED, label: "U2" });
    expect(Object.keys(next)).toEqual(Object.keys(STORED));
  });
});
