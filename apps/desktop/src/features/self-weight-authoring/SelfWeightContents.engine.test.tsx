// RT-A6 H1 (DEL-05-01 Work `hydrostatic-test.md` §4.1, §6 item 3): the desktop's explicit
// contents selection through the actual candidate Wasm engines. The person states a source
// case's contents-density use in the Conditions control, chooses that case in the self-weight
// panel, and generates into a different new case; the operation engine applies the plan
// atomically; the project is saved and reopened through the maintained project service; the
// panel's refresh inspection finds the generated weight fresh. A selected-source edit (an
// equal density under another key) stales it, and the refresh resolves the target's own
// persisted selection. Only the queue callbacks are observed; every plan, apply and inspection
// is the engines' own. Base document: S3a's committed corpus (s3a_09), with local invented
// mass and contents inputs.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyOperationBatch, type OperationBatch } from "../../services/operationBatchService";
import { applyModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { findLocalProject, openLocalProject, saveLocalProject } from "../../services/projectService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { generateSelfWeightPlan } from "../../services/selfWeightPlanService";
import { previewOperatingConditions } from "../../services/operatingConditionPreviewService";
import { chooseVirtualMultiTarget } from "../../test-support/workspaceTestControls";
import { corpusBase } from "../../test-support/conditionSetCorpus";
import { ConditionUseControl } from "../condition-sets/ConditionUseControl";
import { SelfWeightPlanPanel } from "./SelfWeightPlanPanel";

beforeAll(async () => { await loadWasmEngine(); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const SOURCE = "SUS";
const TARGET = "load:h1-hydro-weight";
const PROJECT = "project:h1-contents-self-weight";
// Independent: r_o = 0.1 m, t = 0.01 m, steel 7,850 kg/m^3, contents 1,000 kg/m^3, gravity
// entered as -9.80665 m/s^2: |w| = pi (0.0019 * 7850 + 0.0081 * 1000) * 9.80665 N/m.
const REFERENCE = (902800199 / 4000000) * Math.PI;
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const caseOf = (model: PreviewModel, id: string) => model.load_cases.find((item) => item.id === id)!;

function base(exact = false): PreviewModel {
  const model = corpusBase("s3a_09");
  model.project.id = PROJECT;
  model.components ??= [];
  model.diagnostics ??= [];
  for (const pipe of model.pipe_segments) pipe.section = { ...pipe.section, outside_diameter: { value: 0.2, unit: "m" }, wall_thickness: { value: 0.01, unit: "m" }, material_density: { value: 7850, unit: "kg/m^3" } };
  if (exact) {
    model.pressure_contract = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" };
    for (const pipe of model.pipe_segments) pipe.section.corrosion_allowance = { value: 0.001, unit: "m" };
    for (const material of model.materials ?? []) {
      material.constitutive_basis = "homogeneous_isotropic_E_nu_v1";
      material.elastic_modulus = { value: 2e11, unit: "Pa" };
      material.shear_modulus = { value: 8e10, unit: "Pa" };
      material.poisson_ratio = { value: 0.25, unit: "1" };
    }
    for (const loadCase of model.load_cases) loadCase.pressure_regions = [];
  }
  const set = model.condition_sets![0];
  // The positive whole-case selection is explicitly complete; other tests
  // exercise missing assignments as named invalid-input cases.
  for (const pipe of model.pipe_segments) pipe.condition_set_ref = set.id;
  set.conditions = [...(set.conditions ?? []),
    { key: "HYDRO", contents_density: { value: 1000, unit: "kg/m^3" } },
    { key: "ALT", contents_density: { value: 1000, unit: "kg/m^3" } }];
  return model;
}
async function applyIntent(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}
async function applyBatch(model: PreviewModel, batch: OperationBatch): Promise<PreviewModel> {
  const before = await canonicalJsonString(model);
  const outcome = await applyOperationBatch(model, batch, await computeModelHash(model));
  expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
  expect(await canonicalJsonString(model)).toBe(before);
  return outcome.applied_model!;
}
/** The Conditions control: choose a contents key for `caseId` and queue it. */
async function chooseContents(model: PreviewModel, caseId: string, key: string): Promise<EditorOperationIntent> {
  const list: EditorOperationIntent[] = [];
  render(<ConditionUseControl model={model} loadCase={caseOf(model, caseId)} onQueueIntent={(intent) => list.push(intent)} />);
  fireEvent.change(screen.getByTestId("load-condition-use-contents"), { target: { value: key } });
  fireEvent.click(screen.getByTestId("load-condition-use-queue"));
  await waitFor(() => expect(list).toHaveLength(1));
  // The source's valid selection remains authored; the engine names the
  // absence of contents weight without blocking it or inventing a force.
  const staged = await screen.findAllByTestId("load-condition-use-diagnostic-CASE_CONTENTS_WEIGHT_NOT_APPLIED");
  for (const item of staged) expect(item).toHaveAttribute("data-severity", "warning");
  cleanup();
  return list[0];
}
async function saveAndReopen(model: PreviewModel): Promise<PreviewModel> {
  const stored = await findLocalProject(PROJECT);
  await saveLocalProject(model, [], null, null, null, null, null, null, stored?.revision);
  const opened = (await openLocalProject(PROJECT))!.model as PreviewModel;
  expect(await canonicalJsonString(opened)).toBe(await canonicalJsonString(model));
  return opened;
}
async function refreshOutcome(model: PreviewModel) {
  const queue = vi.fn();
  render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
  enter("Self-weight action", "refresh");
  enter("Existing self-weight case", TARGET);
  fireEvent.click(screen.getByRole("button", { name: "Generate self-weight refresh plan" }));
  return queue;
}

describe("explicit contents from a source case, through the actual engines (RT-A6 H1)", () => {
  it.each([false, true])("generates/applies/reopens the complete model without removing its source (EX=%s)", async (exact) => {
    const reference = exact ? (5686876335 / 25000000) * Math.PI : REFERENCE;
    // 1. The source case's selection, authored in the Conditions control and applied by the engine.
    const original = base(exact);
    let model = await applyIntent(original, await chooseContents(original, SOURCE, "HYDRO"));
    expect(caseOf(model, SOURCE).condition_use).toEqual({ contents_density: "HYDRO" });

    // 2. The self-weight panel: the source picker lists the case, and the plan names it.
    const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    expect(screen.getByTestId("self-weight-contents-source")).toHaveTextContent(`${SOURCE} (condition HYDRO)`);
    enter("Self-weight case ID", TARGET); enter("Self-weight case label", "Invented hydrotest weight");
    enter("Gravity value", "-9.80665"); enter("Gravity unit", "m/s^2"); enter("Gravity direction", "global_y");
    enter("Self-weight provenance", "Invented H1 inputs only"); chooseVirtualMultiTarget("self-weight-pipes", "P1");
    enter("Contents density from case", SOURCE);
    fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
    expect(screen.getByTestId("self-weight-plan-contents")).toHaveTextContent(`records condition_use.contents_density = HYDRO`);
    fireEvent.click(screen.getByRole("button", { name: "Queue complete self-weight plan" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const batch = queue.mock.calls[0][0] as OperationBatch;
    cleanup();

    // 3. The engine applies the batch atomically: a new case stating the selection as its own.
    model = await applyBatch(model, batch);
    const target = caseOf(model, TARGET);
    expect(target.condition_use).toEqual({ contents_density: "HYDRO" });
    if (exact) expect(target.pressure_regions).toEqual([]);
    else expect(target.pressure_regions).toBeUndefined();
    const load = target.primitive_loads![0];
    const intensity = (load.magnitude as { value: number }).value;
    expect(Math.abs(intensity + reference)).toBeLessThanOrEqual(1e-14 * reference);
    const evidence = JSON.parse(load.provenance as string);
    expect(evidence.contents_source).toMatchObject({ origin_case: SOURCE, condition_set: "cs:a", condition_key: "HYDRO" });
    expect(caseOf(model, SOURCE).primitive_loads).toEqual([]);

    // 4. Save and reopen through the project service; the refresh inspection finds it fresh.
    model = await saveAndReopen(model);
    expect(caseOf(model, SOURCE).condition_use).toEqual({ contents_density: "HYDRO" });
    const preview = await previewOperatingConditions(model);
    expect(preview.resolved).toBe(true);
    expect(preview.diagnostics.some((d) => d.code === "CASE_CONTENTS_WEIGHT_NOT_APPLIED" && d.severity === "warning")).toBe(true);
    expect(preview.diagnostics.some((d) => d.severity === "blocking")).toBe(false);
    let refresh = await refreshOutcome(model);
    expect(await screen.findByRole("alert")).toHaveTextContent("SELF-WEIGHT-REFRESH-NOT-NEEDED");
    expect(refresh).not.toHaveBeenCalled();
    cleanup();

    // 5. The origin is provenance only: changing its key leaves the target fresh.
    const originEdited = await applyIntent(model, await chooseContents(model, SOURCE, "ALT"));
    await refreshOutcome(originEdited);
    expect(await screen.findByRole("alert")).toHaveTextContent("SELF-WEIGHT-REFRESH-NOT-NEEDED");
    cleanup();

    // 6. The target's own selection, an equal density under another key: stale; the refresh
    //    resolves the persisted selection, keeps identity and origin, and is fresh after reopening.
    model = await applyIntent(model, await chooseContents(model, TARGET, "ALT"));
    refresh = await refreshOutcome(model);
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
    fireEvent.click(screen.getByRole("button", { name: "Queue complete self-weight plan" }));
    await waitFor(() => expect(refresh).toHaveBeenCalledOnce());
    model = await saveAndReopen(await applyBatch(model, refresh.mock.calls[0][0] as OperationBatch));
    cleanup();
    const refreshed = caseOf(model, TARGET).primitive_loads![0];
    expect(refreshed.id).toBe(load.id);
    expect((refreshed.magnitude as { value: number }).value).toBe(intensity);
    expect(JSON.parse(refreshed.provenance as string).contents_source).toMatchObject({ origin_case: SOURCE, condition_key: "ALT", selected_by_case: TARGET });
    await refreshOutcome(model);
    expect(await screen.findByRole("alert")).toHaveTextContent("SELF-WEIGHT-REFRESH-NOT-NEEDED");
  });

  it("refuses an unresolvable or malformed source by name and never falls back to section density", async () => {
    const model = base();
    const hash = (await computeModelHash(model))!;
    const request = (source: unknown) => ({ case_id: TARGET, label: "x", pipe_refs: ["P1"], gravity: { value: -9.80665, unit: "m/s^2", axis: "global_y" },
      provenance: "Invented", source_model_hash: hash.value, contents_source_case_ref: source }) as unknown as Parameters<typeof generateSelfWeightPlan>[1];
    for (const [source, code] of [[null, "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"], ["  ", "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"], [7, "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"],
      [SOURCE, "CASE_CONTENTS_CONTEXT_MISSING"], ["load:none", "CASE_CONTENTS_CONTEXT_MISSING"]] as const) {
      const outcome = await generateSelfWeightPlan(model, request(source));
      expect(outcome.status, String(source)).toBe("blocked");
      expect(outcome.plan).toBeNull();
      expect(outcome.diagnostics[0].code, String(source)).toBe(code);
    }
    // The panel offers no source when no case states a contents use.
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={vi.fn()} />);
    expect(screen.getByTestId("self-weight-contents-source-note")).toHaveTextContent("No load case states a contents-density condition use");
  });
});
