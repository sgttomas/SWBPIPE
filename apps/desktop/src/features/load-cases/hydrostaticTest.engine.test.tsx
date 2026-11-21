// The Loads page's Hydrostatic test section and its intent builder, through the real wasm
// operation engine. Base documents and the expected intents are the committed contract corpus
// (`fixtures/model_operations/condition_set_contract_corpus.json`, cases hydro_21 to hydro_25).
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { corpusBase } from "../../test-support/conditionSetCorpus";
import { conditionUseIntent, contentsKeys, testConditionIntent } from "../condition-sets/conditionSetIntents";
import { ConditionUseControl } from "../condition-sets/ConditionUseControl";
import { HydrostaticTestSection, hydrostaticSummary } from "./HydrostaticTestSection";

type CorpusCase = { case_id: string; base_model: PreviewModel; intent: EditorOperationIntent; expected: { outcome: { applied: boolean; diagnostics: Array<{ code: string; severity: string }> } } };
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const corpus = (JSON.parse(readFileSync(path.join(root, "fixtures/model_operations/condition_set_contract_corpus.json"), "utf8")) as { cases: CorpusCase[] }).cases;
const corpusCase = (prefix: string) => {
  const found = corpus.find((item) => item.case_id.startsWith(prefix));
  if (!found) throw new Error(`no corpus case ${prefix}`);
  return structuredClone(found);
};
const ht = (model: PreviewModel) => model.load_cases.find((loadCase) => loadCase.id === "HT")!;

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  return outcome.applied_model as PreviewModel;
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("the test-condition intents against the corpus", () => {
  it.each([
    ["hydro_21", async (model: PreviewModel) => testConditionIntent(canonicalJsonString, ht(model), { kind: "hydrostatic", held_supports: "springs_and_constant_effort" })],
    ["hydro_22", async (model: PreviewModel) => conditionUseIntent(canonicalJsonString, ht(model), { contents_density: "HYDRO" })],
    ["hydro_23", async (model: PreviewModel) => testConditionIntent(canonicalJsonString, ht(model), null)],
    ["hydro_24", async (model: PreviewModel) => testConditionIntent(canonicalJsonString, ht(model), { kind: "hydrostatic", held_supports: "springs_and_constant_effort" })],
    ["hydro_25", async (model: PreviewModel) => testConditionIntent(canonicalJsonString, ht(model), null)]
  ])("%s: the built intent is the corpus's change, and the engine judges it as the corpus expects", async (prefix, build) => {
    const item = corpusCase(prefix);
    const intent = await build(item.base_model);
    const expected = item.intent.change;
    expect(intent.target).toEqual(item.intent.target);
    expect(intent.operation_kind).toBe(item.intent.operation_kind);
    expect(intent.change).toMatchObject({ change_kind: expected.change_kind, field_path: expected.field_path, before: expected.before, unit: expected.unit, dimension: expected.dimension });
    expect(intent.change.after === "not_present" ? "not_present" : JSON.parse(intent.change.after)).toEqual(expected.after === "not_present" ? "not_present" : JSON.parse(expected.after));
    const outcome = await validateModelOperation(item.base_model, intent, await computeModelHash(item.base_model));
    expect(outcome.diagnostics.map((d) => [d.code, d.severity])).toEqual(item.expected.outcome.diagnostics.map((d) => [d.code, d.severity]));
  });

  it("offers the condition keys that state a contents density", () => {
    expect(contentsKeys(corpusBase("hydro_21"))).toEqual([{ key: "HYDRO", densities: [{ value: 1015, unit: "kg/m^3" }] }]);
  });
});

describe("the Hydrostatic test section", () => {
  it("switches a case to a test with both choices as one action, then off with the two clears in order", async () => {
    let model = corpusBase("hydro_21");
    const list: EditorOperationIntent[] = [];
    const view = render(<HydrostaticTestSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={ht(model)} />);
    expect(hydrostaticSummary(ht(model))).toBe("off");
    expect(screen.queryByTestId("load-case-hydrotest-held")).toBeNull();
    fireEvent.click(screen.getByTestId("load-case-hydrotest-switch"));
    expect(screen.getByTestId("load-case-hydrotest-queue")).toBeDisabled();
    fireEvent.change(screen.getByTestId("load-case-hydrotest-held"), { target: { value: "springs_and_constant_effort" } });
    expect(screen.getByRole("option", { name: "HYDRO (1015 kg/m^3)" })).toBeInTheDocument();
    fireEvent.change(screen.getByTestId("load-case-hydrotest-contents"), { target: { value: "HYDRO" } });
    expect(screen.getByTestId("load-case-hydrotest-self-weight")).toHaveTextContent("Self-weight is taken with the selected test contents at solve.");
    fireEvent.click(screen.getByTestId("load-case-hydrotest-queue"));
    await waitFor(() => expect(list).toHaveLength(2));
    expect(list.map((intent) => intent.change.change_kind)).toEqual(["set_load_case_test_condition", "set_load_case_condition_use"]);
    // The state the two leave has contents but no generated self-weight: the engine's staged warning.
    expect(await screen.findByTestId("load-case-hydrotest-finding-LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID")).toHaveAttribute("data-severity", "warning");
    expect(screen.queryByTestId("load-case-hydrotest-finding-LOAD_CASE_TEST_CONTENTS_UNSELECTED")).toBeNull();
    for (const intent of list) model = await apply(model, intent);
    expect(ht(model)).toMatchObject({ test_condition: { kind: "hydrostatic", held_supports: "springs_and_constant_effort" }, condition_use: { contents_density: "HYDRO" } });
    expect(hydrostaticSummary(ht(model))).toBe("springs held, contents HYDRO");
    view.unmount();

    list.length = 0;
    render(<HydrostaticTestSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={ht(model)} />);
    expect(screen.getByTestId("load-case-hydrotest-switch")).toBeChecked();
    fireEvent.click(screen.getByTestId("load-case-hydrotest-switch"));
    fireEvent.click(screen.getByTestId("load-case-hydrotest-queue"));
    await waitFor(() => expect(list).toHaveLength(2));
    expect(list.map((intent) => intent.change.change_kind)).toEqual(["clear_load_case_condition_use", "clear_load_case_test_condition"]);
    for (const intent of list) model = await apply(model, intent);
    expect(ht(model).condition_use).toBeUndefined();
    expect((ht(model) as { test_condition?: unknown }).test_condition).toBeUndefined();
  });

  it("queues the test alone with the staged warning when no contents are chosen", async () => {
    const model = corpusBase("hydro_21");
    const list: EditorOperationIntent[] = [];
    render(<HydrostaticTestSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={ht(model)} />);
    fireEvent.click(screen.getByTestId("load-case-hydrotest-switch"));
    fireEvent.change(screen.getByTestId("load-case-hydrotest-held"), { target: { value: "none" } });
    fireEvent.click(screen.getByTestId("load-case-hydrotest-queue"));
    await waitFor(() => expect(list).toHaveLength(1));
    expect(JSON.parse(list[0].change.after)).toEqual({ kind: "hydrostatic", held_supports: "none" });
    const warning = await screen.findByTestId("load-case-hydrotest-finding-LOAD_CASE_TEST_CONTENTS_UNSELECTED");
    expect(warning).toHaveTextContent("Warning: test contents not chosen");
  });

  it("shows the engine's refusal on a document without a pressure contract and queues nothing", async () => {
    const model = corpusBase("hydro_24");
    const list: EditorOperationIntent[] = [];
    render(<HydrostaticTestSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={ht(model)} />);
    fireEvent.click(screen.getByTestId("load-case-hydrotest-switch"));
    fireEvent.change(screen.getByTestId("load-case-hydrotest-held"), { target: { value: "springs_and_constant_effort" } });
    fireEvent.click(screen.getByTestId("load-case-hydrotest-queue"));
    const refusal = await screen.findByTestId("load-case-hydrotest-finding-LOAD_CASE_TEST_CONDITION_PRESSURE_FREE");
    expect(refusal).toHaveAttribute("data-severity", "blocking");
    expect(refusal).toHaveTextContent("Refused: the model has no pressure contract");
    expect(screen.getByTestId("load-case-hydrotest-error")).toHaveTextContent("nothing was queued");
    expect(list).toEqual([]);
  });
});

describe("the Conditions control on a document with a pressure contract", () => {
  it("shows a test case's contents as chosen in the Hydrostatic test section and keeps every other member refused", () => {
    const model = corpusBase("hydro_25");
    render(<ConditionUseControl model={model} onQueueIntent={() => undefined} loadCase={ht(model)} />);
    expect(screen.getByTestId("load-condition-use-test-contents")).toHaveTextContent("Test contents: HYDRO, set in the Hydrostatic test section.");
    expect(screen.getByTestId("load-condition-use-refused-exact")).toHaveTextContent("LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    expect(screen.queryByTestId("load-condition-use-temperature")).toBeNull();
  });

  it("is unchanged for an ordinary case", () => {
    const model = corpusBase("hydro_21");
    render(<ConditionUseControl model={model} onQueueIntent={() => undefined} loadCase={ht(model)} />);
    expect(screen.queryByTestId("load-condition-use-test-contents")).toBeNull();
    expect(screen.getByTestId("load-condition-use-refused-exact")).toHaveTextContent("refused on a document with a pressure contract until the exact route resolves it");
  });
});
