// The reference-entry journey (RT-G2/RT-G3; exit criterion 7 as context): the reference system
// entered from a blank load/reference model, following the
// DEL-09-01 run sheet §2–§3.12 in order, through the desktop's own surfaces
// (the units dialog, the pressure, material, section, component and support forms, the run
// sheet's grid, the load manager with its fills and copies, the self-weight plan, batch review
// and Review/Apply) and one workspace session running the real wasm operation engine. At the end
// the document is compared with `fixtures/reference_system/reference_system.json` record by
// record, keyed by id, with only the run sheet's §3.14 differences allowed, each by member path;
// then it is saved in the browser store, reopened, and compared again.
//
// Functional authoring-path evidence only: not a human timing, native usability, physics or
// acceptance result. No solve runs here (§3.13 is a native job, out of this test's scope); every duration in
// the run sheet stays an estimate.
//
// Routes, per step: "typed UI" is a person's entry in a rendered control; "UI function" is the
// same function a control calls, named at the step. The run sheet's non-bend, non-valve/flange
// rows are entered through the sheet's commit functions, as `referenceBendSifs.test.tsx` does.
//
// The unit catalog is the desktop's (`get_unit_catalog`); the browser preview has none, and its
// section and component forms then offer only the project's units. As `rigidQuantityUnits.test.tsx`
// does, the committed catalog fixture (`test-support/unitCatalogFixture.ts`) answers in its place.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../fixtures/reference_system/reference_system.json";
import { buildBlankLoadStateModelDocument, buildBlankLocalModelDocument } from "../../services/projectService";
import * as previewService from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import type { EntityRef, PreviewModel } from "../../types";
import { useModelledVisibility } from "../../test-support/modelledVisibility";
import { chooseCompactOption, chooseVirtualTarget, chooseVirtualMultiTarget } from "../../test-support/workspaceTestControls";
import { enterReferenceHeader, enterReferenceRows, P, S5, type Cells } from "../../test-support/referenceRunSheetEntry";
import { ProjectUnitsDialog } from "../project-units/ProjectUnitsDialog";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { RunSheet, RUN_SHEET_TIMING } from "../model-tree/runSheet/RunSheet";
import { BEND_LABELS } from "../model-tree/runSheet/runSheetBatch";
import { useRunSheetStore } from "../model-tree/runSheet/runSheetStore";
import { LoadCaseManagerPanel } from "../load-cases/LoadCaseManagerPanel";
import { SelfWeightPlanPanel } from "../self-weight-authoring/SelfWeightPlanPanel";
import { BatchReviewPanel } from "../toolkit/BatchReviewPanel";
import { OperationApplyPanel } from "../operations/OperationApplyPanel";
import { useWorkspaceSession } from "./workspaceSession";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";

vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => ({ route: "tauri_unit_catalog", catalog: unitCatalogFixture() })
}));

afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });
useModelledVisibility();

const reference = referenceJson as unknown as PreviewModel;
/** The blank load/reference model's own members (projectService), for the document comparison. */
const BLANK = buildBlankLoadStateModelDocument(new Date("2026-11-04T00:00:00Z"));
type Session = ReturnType<typeof useWorkspaceSession>;
type AnyRecord = Record<string, unknown>;
const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;

/**
 * Steps the desktop cannot enter as the fixture records them. Empty: every step is entered, and
 * the comparison excludes no record. The list is asserted empty, so a step that stops entering
 * fails the comparison rather than being left out of it. (RS-VS1 was the last: #130 admits
 * its `restraints: []` for a variable spring hanger whose hanger stiffness DOF is UX/UY/UZ.)
 */
const BLOCKED: ReadonlyArray<{ step: string; missing: string; evidence: string; records: readonly string[] }> = [];

/** The run sheet's §3.14 differences, each allowed by member path (and checked to be the stated value). */
const ALLOWED_DIFFERENCES = [
  "project.id", "project.name", "project.description",
  "pipe_segments[].label (derived: the pipe id's token)",
  "nodes[RS-070, RS-075, RS-090, RS-095, RS-220, RS-225].label (derived bend start/end labels)",
  "pipe_segments[RS-P065, RS-P085, RS-P225].label (derived bend arc label)",
  "components[RS-B1, RS-B2, RS-B3].label (derived bend label)",
  "components[RS-EJ1].objective_connector.stiffness.provenance (the connector's own references)",
  "combinations[].terms ([] on a subtraction)",
  "load_cases[].primitive_loads[].provenance.source_model_hash (the generating model's hash)",
  "array order: every collection, compared by id",
  "array order: reference_configurations[].member_references, by pipe_ref (model pipe order)",
  "array order: load_cases[].analysis_state.element_states, by pipe_ref (model pipe order)",
  "array order: load_cases[].analysis_state.support_states, by support_ref (model support order)",
  "array order: load_cases[].pressure_regions[].member_pipe_ids, as a set (model pipe order)",
] as const;

/**
 * Document members the blank load/reference model carries and §3.14 does not list. They are not
 * records; the test asserts each is exactly the blank's, so the list cannot grow silently.
 */
const DOCUMENT_DIFFERENCES_NOT_IN_3_14 = ["data_boundary", "diagnostics"] as const;

/** Row labels the person types in the Label cell (§3.4: "for the fixture's text, type nodes[].label"). */
const ROW_NODES: Record<number, string> = {
  0: "RS-010", 1: "RS-020", 2: "RS-030", 3: "RS-040", 4: "RS-050", 5: "RS-060", 8: "RS-080", 10: "RS-100", 11: "RS-110",
  12: "RS-120", 13: "RS-130", 14: "RS-140", 15: "RS-150", 16: "RS-160", 17: "RS-170", 18: "RS-180", 19: "RS-190",
  20: "RS-200", 21: "RS-210", 23: "RS-215", 25: "RS-230",
};
const rowLabel = (index: number): Cells => {
  const id = ROW_NODES[index];
  return id ? { label: reference.nodes.find((node) => node.id === `node:${id}`)!.label! } : {};
};

// ---------------------------------------------------------------------------------------------
// The comparison
// ---------------------------------------------------------------------------------------------

const COLLECTIONS = ["materials", "sections", "nodes", "pipe_segments", "components", "supports", "load_cases", "combinations", "reference_configurations"] as const;
const BEND_NODES = ["node:RS-070", "node:RS-075", "node:RS-090", "node:RS-095", "node:RS-220", "node:RS-225"];
const BEND_ARCS = ["pipe:RS-P065", "pipe:RS-P085", "pipe:RS-P225"];
const BENDS = ["component:RS-B1", "component:RS-B2", "component:RS-B3"];
const token = (id: string) => id.slice(id.indexOf(":") + 1);
const byKey = (key: string) => (a: AnyRecord, b: AnyRecord) => String(a[key]).localeCompare(String(b[key]));

/** One side of the comparison: each collection keyed by id, nested lists in key order. */
function keyed(model: PreviewModel): Record<string, Record<string, AnyRecord>> {
  const out: Record<string, Record<string, AnyRecord>> = {};
  for (const collection of COLLECTIONS) {
    const list = clone(((model as unknown as AnyRecord)[collection] ?? []) as AnyRecord[]);
    expect(new Set(list.map((record) => record.id)).size, `${collection}: unique ids`).toBe(list.length);
    out[collection] = Object.fromEntries(list.map((record) => [String(record.id), record]));
  }
  for (const configuration of Object.values(out.reference_configurations)) (configuration.member_references as AnyRecord[] | undefined)?.sort(byKey("pipe_ref"));
  for (const loadCase of Object.values(out.load_cases)) {
    for (const region of (loadCase.pressure_regions as AnyRecord[] | undefined) ?? []) (region.member_pipe_ids as string[]).sort();
    const state = loadCase.analysis_state as AnyRecord | undefined;
    (state?.element_states as AnyRecord[] | undefined)?.sort(byKey("pipe_ref"));
    (state?.support_states as AnyRecord[] | undefined)?.sort(byKey("support_ref"));
  }
  return out;
}

/** Applies the §3.14 allowances to the entered side: each is checked to be the stated derived value, then set to the fixture's. */
function allowDifferences(entered: Record<string, Record<string, AnyRecord>>, fixture: Record<string, Record<string, AnyRecord>>): void {
  for (const [id, pipe] of Object.entries(entered.pipe_segments)) {
    const expected = BEND_ARCS.includes(id) ? BEND_LABELS.arc : token(id);
    expect(pipe.label, `${id}.label`).toBe(expected);
    if (fixture.pipe_segments[id]) pipe.label = fixture.pipe_segments[id].label;
  }
  for (const id of BEND_NODES) {
    const node = entered.nodes[id];
    if (!node) continue;
    expect(node.label, `${id}.label`).toBe(id.endsWith("0") ? BEND_LABELS.start : BEND_LABELS.end);
    node.label = fixture.nodes[id]?.label;
  }
  for (const id of BENDS) {
    const component = entered.components[id];
    if (!component) continue;
    expect(component.label, `${id}.label`).toBe(BEND_LABELS.component);
    component.label = fixture.components[id]?.label;
  }
  const connector = entered.components["component:RS-EJ1"]?.objective_connector as AnyRecord | undefined;
  if (connector) {
    const stiffness = connector.stiffness as AnyRecord;
    // The form writes the connector's own source, measurement and transform references (`componentIntent.ts` objectiveConnector).
    const own = connector.provenance as AnyRecord;
    expect(stiffness.provenance, "RS-EJ1 stiffness provenance").toEqual({ source_reference: own.source_reference, measurement_restraints: own.measurement_restraints, basis_transform_reference: own.basis_transform_reference });
    stiffness.provenance = ((fixture.components["component:RS-EJ1"].objective_connector as AnyRecord).stiffness as AnyRecord).provenance;
  }
  for (const [id, combination] of Object.entries(entered.combinations)) {
    expect(combination.terms, `${id}.terms`).toEqual([]);
    delete combination.terms;
  }
  // Entered side: each primitive names exactly the hash of the model it was generated from,
  // captured in the journey before Generate. Fixture side: the generator's own capture model,
  // which this test cannot reproduce, so only its form is checked.
  for (const [side, entered_] of [[entered, true], [fixture, false]] as const) for (const [caseId, loadCase] of Object.entries(side.load_cases)) for (const load of (loadCase.primitive_loads as AnyRecord[] | undefined) ?? []) {
    const provenance = JSON.parse(String(load.provenance)) as AnyRecord;
    if (entered_) expect(provenance.source_model_hash, `${String(load.id)}.provenance.source_model_hash`).toBe(generatingHashes.get(caseId));
    else expect(provenance.source_model_hash, `${String(load.id)}.provenance.source_model_hash`).toMatch(/^sha256:[0-9a-f]{64}$/);
    provenance.source_model_hash = "(the generating model's hash)";
    load.provenance = provenance;
  }
}

/** The hash of the model each self-weight case was generated from, captured in the journey. */
const generatingHashes = new Map<string, string>();

/** The journey's document against the fixture, by record. Returns the ids compared. */
function compareWithFixture(model: PreviewModel, where: string): number {
  // RS-VS1's nested entries (review r1 of #128, formerly excluded): each case with an analysis
  // state holds exactly one RS-VS1 support state, in the fixture and in the entered document,
  // and the two are equal. The record comparison below compares the whole lists as well.
  const vs1States = (document: PreviewModel, caseId: string) => ((document.load_cases.find((item) => item.id === caseId)?.analysis_state as AnyRecord | undefined)?.support_states as AnyRecord[] | undefined)?.filter((entry) => entry.support_ref === "support:RS-VS1");
  const statedCases = reference.load_cases.filter((item) => (item.analysis_state as AnyRecord | undefined)?.support_states).map((item) => item.id);
  expect(statedCases, "fixture: cases with support states").toEqual(["load:RS-SUS", "load:RS-OPE", "load:RS-SAM"]);
  for (const caseId of statedCases) {
    expect(vs1States(reference, caseId), `fixture: ${caseId} RS-VS1 support states`).toHaveLength(1);
    expect(vs1States(model, caseId), `${where}: ${caseId} RS-VS1 support states`).toEqual(vs1States(reference, caseId));
  }
  const entered = keyed(model);
  const fixture = keyed(reference);
  allowDifferences(entered, fixture);
  let compared = 0;
  for (const collection of COLLECTIONS) {
    const expectedIds = Object.keys(fixture[collection]).sort();
    expect(Object.keys(entered[collection]).sort(), `${where}: ${collection} ids`).toEqual(expectedIds);
    for (const id of expectedIds) expect(entered[collection][id], `${where}: ${collection} ${id}`).toEqual(fixture[collection][id]);
    compared += expectedIds.length;
  }
  // Document members outside the collections: each equals the fixture's, but for project.id, name and
  // description (§3.14) and the blank's own members §3.14 does not list, asserted to be the blank's.
  const document = model as unknown as AnyRecord, fixtureDocument = reference as unknown as AnyRecord;
  const members = [...new Set([...Object.keys(document), ...Object.keys(fixtureDocument)])].filter((key) => !(COLLECTIONS as readonly string[]).includes(key)).sort();
  expect(members, `${where}: document members`).toEqual(["analysis_status", "data_boundary", "diagnostics", "document_kind", "pressure_contract", "project", "schema_version"]);
  for (const key of members.filter((key) => key !== "project" && !(DOCUMENT_DIFFERENCES_NOT_IN_3_14 as readonly string[]).includes(key))) expect(document[key], `${where}: ${key}`).toEqual(fixtureDocument[key]);
  const { id, name, description, ...project } = model.project;
  expect([id, name, description], `${where}: project (§3.14)`).toEqual([expect.stringMatching(/^project:blank-local-/), expect.stringMatching(/^Blank Local Load\/reference Model/), BLANK.project.description]);
  expect(project, `${where}: project.units`).toEqual({ units: reference.project.units });
  const blank = BLANK as unknown as AnyRecord;
  expect(document.data_boundary, `${where}: data_boundary (the blank's; absent in the fixture)`).toEqual(blank.data_boundary);
  expect(fixtureDocument.data_boundary).toBeUndefined();
  expect(document.diagnostics, `${where}: diagnostics (the blank's, empty; absent in the fixture)`).toEqual([]);
  expect(fixtureDocument.diagnostics).toBeUndefined();
  return compared;
}

// ---------------------------------------------------------------------------------------------
// The journey
// ---------------------------------------------------------------------------------------------

describe("the reference system entered from blank through the desktop's authoring surfaces", () => {
  it("follows the run sheet §2–§3.12 through one session and the wasm engine, and equals the fixture by record; Undo/Redo per family; browser save and reopen", async () => {
    // The session opens on an unrelated blank; §2 then creates the load/reference blank through the units dialog.
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(buildBlankLocalModelDocument(new Date("2026-11-04T00:00:00Z")));
    RUN_SHEET_TIMING.engineCheckDelayMs = 60_000;
    const current = { sheet: false } as { session: Session; store: ReturnType<typeof useRunSheetStore>; sheet: boolean; rerender: () => void };
    function Harness() {
      const session = useWorkspaceSession();
      const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
      current.session = session; current.store = store;
      const model = session.model.model;
      const ops = session.operations;
      const selection = session.selection.selection;
      // As App does (App.tsx), the surfaces render once the session holds a model and a selection.
      return <>
        <ProjectUnitsDialog request={session.project.blankProjectUnitsRequest} />
        {model && selection ? <>
          <div data-testid="journey-inspector">
            <PropertyInspector getPreparationEpoch={ops.getPreparationEpoch} model={model} onQueueIntent={ops.handleQueueEditorIntent} onValidateIntent={ops.handleValidateIntent}
              onApplyIntent={ops.handleApplyIntent} operationBusy={ops.operationBusy} operationOutcomes={ops.operationOutcomes} projectSessionGeneration={session.model.projectSessionGeneration}
              queuedIntents={ops.editorIntents} selection={selection} selectionState={session.selection.orderedSelection} taskRequest={session.chrome.propertyTaskRequest} />
          </div>
          <div data-testid="journey-loads">
            <LoadCaseManagerPanel model={model} queuedIntents={ops.editorIntents} operationBusy={ops.operationBusy} getPreparationEpoch={ops.getPreparationEpoch}
              onQueueIntent={ops.handleQueueEditorIntent} onSelect={session.selection.handleSelectEntity} selection={selection} />
          </div>
          <div data-testid="journey-self-weight">
            <SelfWeightPlanPanel model={model} selection={selection} selectedPipeRefs={session.selection.selectedPipeRefs} onQueueBatch={ops.handleQueueOperationBatch} busy={ops.operationBusy} requestEpoch={ops.requestEpoch} />
          </div>
          <BatchReviewPanel onClear={ops.handleClearReviewQueue} batches={ops.queuedBatches} outcomes={ops.batchOutcomes} receipts={ops.batchReceipts}
            revision={session.model.modelRevision.current} busy={ops.operationBusy} message={ops.batchMessage}
            onValidate={(entry) => void ops.handleRunOperationBatch(entry, false)} onApply={(entry) => void ops.handleRunOperationBatch(entry, true)}
            onDismiss={(entry) => ops.handleDismissRetiredBatch(entry.key)} />
          <OperationApplyPanel queuedIntents={ops.editorIntents} outcomes={ops.operationOutcomes} appliedOperations={ops.appliedOperations} undoCount={ops.undoStack.length}
            redoCount={ops.redoStack.length} busy={ops.operationBusy} message={ops.operationMessage} engineStatus={ops.operationEngineStatus}
            onValidate={ops.handleValidateIntent} onApply={ops.handleApplyIntent} onReviewAllAsBatch={() => void ops.handleReviewQueuedIntentsAsBatch()}
            onUndo={ops.handleUndoSessionModelEdit} onRedo={ops.handleRedoSessionModelEdit} />
          {current.sheet ? <RunSheet model={model} host={{ store, queuedBatches: ops.queuedBatches, editorIntents: ops.editorIntents, registerBatch: ops.registerOperationBatch }} /> : null}
        </> : null}
      </>;
    }
    const view = render(<Harness />);
    current.rerender = () => view.rerender(<Harness />);
    await waitFor(() => expect(current.session?.model.model).toBeTruthy());

    const ops = () => current.session.operations;
    const model = () => current.session.model.model!;
    const inspector = () => within(screen.getByTestId("journey-inspector"));
    const loads = () => within(screen.getByTestId("journey-loads"));
    const alerts = () => screen.queryAllByRole("alert").map((item) => item.textContent).join(" | ");
    const diagnostics = () => Object.values(ops().operationOutcomes).flatMap((outcome) => outcome.diagnostics.filter((item) => item.severity === "blocking").map((item) => `${item.code}: ${item.message}`)).join(" | ");
    const change = (scope: ReturnType<typeof within>, label: string, value: string) => fireEvent.change(scope.getByLabelText(label), { target: { value } });
    const click = (scope: ReturnType<typeof within>, name: string | RegExp) => fireEvent.click(scope.getByRole("button", { name }));
    const select = (entity: EntityRef) => act(() => { current.session.selection.handleSelectEntity(entity); });
    const cycles: string[] = [];
    /** Picks `value` in a virtual picker found by its label (`VirtualTargetPicker`, `VirtualMultiTargetPicker`). */
    function pick(scope: ReturnType<typeof within>, label: string, values: readonly string[]) {
      const box = scope.getByRole("combobox", { name: label });
      const picker = box.closest(".virtual-target-picker") as HTMLElement;
      for (const value of values) {
        fireEvent.change(box, { target: { value } });
        const option = within(picker).getAllByRole("option").find((candidate) => candidate.querySelector("small")?.textContent === value);
        if (!option) throw new Error(`${label}: no option ${value}`);
        fireEvent.click(option);
      }
      fireEvent.change(box, { target: { value: "" } });
    }
    /** Chooses an option of a compact select by its visible label. */
    function chooseByLabel(control: HTMLElement, text: RegExp) {
      fireEvent.click(control);
      const list = document.getElementById(control.getAttribute("aria-controls") ?? "")!;
      const option = Array.from(list.querySelectorAll<HTMLElement>('[role="option"]')).find((candidate) => text.test(candidate.textContent ?? ""));
      if (!option) throw new Error(`no option ${text}: ${Array.from(list.querySelectorAll('[role="option"]')).map((o) => o.textContent).join(", ")}`);
      fireEvent.click(option);
    }
    /** Review/Apply: the one queued intent's Apply button (`OperationApplyPanel`). One Undo step. */
    async function applyQueuedIntent(what: string) {
      await waitFor(() => expect(ops().editorIntents, `${what}: queued (${alerts()})`).toHaveLength(1), { timeout: 20_000 });
      // The session keeps 25 Undo checkpoints (`workspaceSession.ts`), so an apply is a new head checkpoint.
      const head = ops().undoStack[0];
      const button = screen.getByTestId("operation-apply-panel").querySelector<HTMLButtonElement>('[data-testid^="apply-intent-"]')!;
      await waitFor(() => expect(button).toBeEnabled());
      await act(async () => { fireEvent.click(button); });
      await waitFor(() => expect(ops().undoStack[0] !== head && !ops().operationBusy, `${what}: applied (${ops().operationMessage} ${diagnostics()})`).toBe(true), { timeout: 20_000 });
      await waitFor(() => expect(ops().editorIntents, what).toHaveLength(0));
      cycles.push(what);
    }
    /** Batch review: Validate batch, then Apply entire batch (`BatchReviewPanel`). One Undo step. */
    async function applyQueuedBatch(what: string) {
      await waitFor(() => expect(ops().queuedBatches, `${what}: queued (${alerts()})`).toHaveLength(1), { timeout: 20_000 });
      const panel = document.getElementById("batch-review")!;
      const head = ops().undoStack[0];
      const validate = panel.querySelector<HTMLButtonElement>('[data-testid^="validate-batch-"]')!;
      await waitFor(() => expect(validate).toBeEnabled());
      await act(async () => { fireEvent.click(validate); });
      const apply = () => panel.querySelector<HTMLButtonElement>('[data-testid^="apply-batch-"]')!;
      await waitFor(() => expect(apply(), `${what}: validated (${ops().batchMessage})`).toBeEnabled(), { timeout: 20_000 });
      await act(async () => { fireEvent.click(apply()); });
      await waitFor(() => expect(ops().undoStack[0] !== head && !ops().operationBusy, `${what}: applied (${ops().batchMessage})`).toBe(true), { timeout: 30_000 });
      await waitFor(() => expect(ops().queuedBatches, what).toHaveLength(0));
      cycles.push(what);
    }
    /** Undo then Redo of the last step (`OperationApplyPanel`'s buttons): the prior document, then the applied one again. */
    async function undoRedo(what: string, prior: PreviewModel) {
      const applied = model();
      const panel = within(screen.getByTestId("operation-apply-panel"));
      act(() => { fireEvent.click(panel.getByTestId("undo-session-model-edit")); });
      await waitFor(() => expect(model(), `${what}: undone`).toEqual(prior));
      act(() => { fireEvent.click(panel.getByTestId("redo-session-model-edit")); });
      await waitFor(() => expect(model(), `${what}: redone`).toEqual(applied));
      undoRedone.push(what);
    }
    const undoRedone: string[] = [];
    /** The inspector's Task view: Start task from current selection, Property to edit, value, Add (`PropertyInspector`). */
    async function propertyEdit(entity: EntityRef, property: RegExp, value: string, what: string) {
      select(entity);
      fireEvent.click(inspector().getByRole("tab", { name: /^Task$/ }));
      const start = screen.queryByTestId("inspector-start-task");
      if (start) fireEvent.click(start);
      chooseByLabel(screen.getByTestId("editor-intent-field"), property);
      fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value } });
      fireEvent.click(screen.getByTestId("queue-editor-intent"));
      await applyQueuedIntent(what);
      fireEvent.click(screen.getByTestId("cancel-editor-intent"));
      fireEvent.click(inspector().getByRole("tab", { name: /^Properties$/ }));
    }

    // §2.1 Project page, Blank load/reference model: the units dialog, Preset SI, Create.
    // UI function: `requestBlankProject("load_state")`, the function the Project page's button calls; then the dialog typed.
    act(() => current.session.project.requestBlankProject("load_state"));
    const dialog = await screen.findByTestId("project-units-dialog");
    expect(within(dialog).getByTestId("project-units-preset")).toHaveValue("si");
    fireEvent.click(within(dialog).getByTestId("project-units-create"));
    await waitFor(() => expect(model().project.name).toMatch(/^Blank Local Load\/reference Model/));
    expect(model().pressure_contract).toEqual({ version: "3.0.0", mode: "exact_pressure_v3", shear_model: "timoshenko_annulus_energy_v1" });
    expect(model().project.units).toEqual(reference.project.units);

    // §3.1 Pressure profile (typed UI): exact_pressure_v3, Euler–Bernoulli.
    change(loads(), "Pressure profile", "exact_pressure_v3");
    change(loads(), "Shear model", "euler_bernoulli");
    click(loads(), "Queue pressure profile");
    await applyQueuedIntent("§3.1 pressure profile");
    expect(model().pressure_contract).toEqual(reference.pressure_contract);

    // §3.2.1 Material RS-M1 (typed UI): Create explicit E/nu material.
    change(loads(), "Exact material ID", "material:RS-M1");
    change(loads(), "Exact material label", "RS-M1 invented carbon-steel-like material");
    change(loads(), "Constitutive basis", "homogeneous_isotropic_E_nu_v1");
    change(loads(), "Elastic modulus E value", "200"); change(loads(), "Elastic modulus E unit", "GPa");
    change(loads(), "Poisson ratio nu value", "0.3"); change(loads(), "Poisson ratio nu unit", "1");
    change(loads(), "Thermal expansion coefficient value", "1.2e-5"); change(loads(), "Thermal expansion coefficient unit", "1/K");
    change(loads(), "Constitutive provenance", P);
    click(loads(), "Queue exact material");
    await applyQueuedIntent("§3.2 material");
    // §3.2.2 Temperature points (typed UI, after selecting the material: UI function `handleSelectEntity`, the Model tree's selection).
    select({ type: "material", id: "material:RS-M1" });
    const points = [["point:RS-M1-T020", "20", "200"], ["point:RS-M1-T100", "100", "195"], ["point:RS-M1-T200", "200", "189"], ["point:RS-M1-T300", "300", "182"]];
    for (const [index, [id, temperature, modulus]] of points.entries()) {
      click(inspector(), "Add temperature point");
      const n = `Point ${index + 1}`;
      change(inspector(), `${n} ID`, id);
      change(inspector(), `${n} Temperature value`, temperature); change(inspector(), `${n} Temperature unit`, "degC");
      change(inspector(), `${n} Elastic modulus value`, modulus); change(inspector(), `${n} Elastic modulus unit`, "GPa");
      change(inspector(), `${n} Poisson ratio (unit 1) value`, "0.3"); change(inspector(), `${n} Poisson ratio (unit 1) unit`, "1");
      change(inspector(), `${n} Thermal expansion coefficient value`, "1.2e-5"); change(inspector(), `${n} Thermal expansion coefficient unit`, "1/K");
      change(inspector(), `${n} provenance`, P);
    }
    click(inspector(), "Queue temperature points");
    await applyQueuedIntent("§3.2 temperature points");
    // §3.2.3 Expansion law (typed UI).
    click(inspector(), "Add expansion law");
    change(inspector(), "Expansion law 1 definition", "engineering_secant");
    change(inspector(), "Expansion law 1 ID", "law:RS-M1-SECANT");
    change(inspector(), "Expansion law 1 datum temperature value", "20"); change(inspector(), "Expansion law 1 datum temperature unit", "degC");
    change(inspector(), "Expansion law 1 data", "constant");
    change(inspector(), "Expansion law 1 constant coefficient value", "1.2e-5"); change(inspector(), "Expansion law 1 constant coefficient unit", "1/K");
    change(inspector(), "Expansion law 1 provenance", P);
    click(inspector(), "Queue expansion laws");
    await applyQueuedIntent("§3.2 expansion laws");
    expect(model().materials).toEqual(reference.materials);

    // §3.3 Sections (typed UI): Inspector, New section.
    select({ type: "project", id: model().project.id });
    for (const [id, od, wall] of [["section:RS-S270", "270", "9.0"], ["section:RS-S220", "220", "8.0"], ["section:RS-S170", "170", "7.0"]]) {
      change(inspector(), "New section ID", id);
      change(inspector(), "New section name", token(id));
      chooseCompactOption(screen.getByTestId("create-section-type"), "pipe");
      chooseCompactOption(screen.getByTestId("create-section-length-unit"), "mm");
      change(inspector(), "New section outside diameter", od);
      change(inspector(), "New section wall thickness", wall);
      change(inspector(), "New section provenance", P);
      fireEvent.click(screen.getByTestId("queue-create-section-intent"));
      await applyQueuedIntent(`§3.3 ${id}`);
    }
    expect(model().sections).toEqual(reference.sections);

    // §3.4/§3.5 The run sheet (Model tree Grid › Run sheet): header, 26 rows with their labels,
    // bends with SIFs, valve and flanges; Queue rows, Validate batch, Apply entire batch.
    // Typed UI: the bend rows and the valve/flange rows (with Component details) and their labels.
    // Draft setup: the header and column units are set in the sheet's draft state directly
    // (not through the header controls). UI function: the other rows, through the sheet's own
    // commit functions.
    const beforeSheet = model();
    current.sheet = true; current.rerender();
    await screen.findByTestId("run-sheet");
    enterReferenceHeader(current.store, model());
    const gridEntries = enterReferenceRows(() => current.store, model(), reference, rowLabel);
    expect(gridEntries).toBe(13 + 12 + 14 + 3 * (2 + 19 + 1));
    const queue = screen.getByTestId("run-sheet-queue");
    await waitFor(() => expect(queue, screen.getByTestId("run-sheet-queue-reason").textContent ?? "").toBeEnabled(), { timeout: 20_000 });
    await act(async () => { fireEvent.click(queue); });
    await applyQueuedBatch("§3.4 run sheet");
    current.sheet = false; current.rerender();
    expect(model().nodes).toHaveLength(27);
    expect(model().pipe_segments).toHaveLength(26);
    expect(model().supports).toHaveLength(10);
    // Undo/Redo, geometry and support families: the run sheet's batch, which creates the nodes,
    // pipes and bends and ten of the eleven supports (the eleventh, RS-VS1, is the form's).
    expect(beforeSheet.supports).toEqual([]);
    await undoRedo("geometry and supports: the run sheet's batch (§3.4)", beforeSheet);
    // §3.4 RS-050 correction (typed UI): Start task from current selection, X position 1.3, Add, Apply.
    expect(model().nodes.find((node) => node.id === "node:RS-050")!.position.x).toBe(1.2999999999999998);
    await propertyEdit({ type: "node", id: "node:RS-050" }, /^X position/, "1.3", "§3.4 RS-050 X");
    expect(model().nodes.find((node) => node.id === "node:RS-050")!.position.x).toBe(1.3);

    // §3.6.1 Expansion joint RS-EJ1 (typed UI): Inspector, New component, kind expansion_joint.
    select({ type: "project", id: model().project.id });
    const component = (label: string, value: string) => change(inspector(), `New component ${label.toLowerCase()}`, value);
    const componentField = (label: string, value: string) => change(inspector(), `New component ${label}`, value);
    // Kind, node and pipe first: choosing them re-proposes the draft's label (`componentDraftForKind`, `componentDraftForNode`).
    chooseCompactOption(screen.getByTestId("create-component-kind"), "expansion_joint");
    chooseVirtualTarget("create-component-node", "node:RS-190");
    chooseVirtualTarget("create-component-pipe", "pipe:RS-P180");
    componentField("ID", "component:RS-EJ1");
    componentField("label", "Expansion joint EJ1 (tied)");
    component("Connector axes (global)", "0 -1 0; 1 0 0; 0 0 1");
    component("End i node axes (global)", "1 0 0; 0 1 0; 0 0 1");
    component("End j node axes (global)", "1 0 0; 0 1 0; 0 0 1");
    component("End i offset (local)", "0 0 0"); component("End j offset (local)", "0 0 0"); component("Offset unit", "m");
    change(inspector(), "Connector reference state", "stress_free");
    component("Reference translation q_ref", "0 0 0"); component("Reference translation unit", "m");
    component("Reference rotation q_ref", "0 0 0"); component("Reference rotation unit", "rad");
    component("Installed reference temperature", "20"); component("Installed reference temperature unit", "degC");
    component("Translation scale", "1"); component("Translation scale unit", "m");
    component("Work matrix upper triangle (N*m)", "150000 0 0 0 0 0 400000 0 0 0 0 400000 0 0 0 500000 0 0 1500 0 1500");
    component("Installed geometry (calibration)", "authored model geometry");
    change(inspector(), "Connector hardware", "explicit_axial_rods_v1");
    for (const [rod, id, offset] of [[1, "rod:RS-EJ1-RYP", "0 0.2 0"], [2, "rod:RS-EJ1-RYN", "0 -0.2 0"]] as const) {
      click(inspector(), "Add tie rod");
      component(`Rod id (rod ${rod})`, id);
      component(`Offset at end i (connector axes) (rod ${rod})`, offset);
      component(`Offset at end j (connector axes) (rod ${rod})`, offset);
      component(`Offset unit (rod ${rod})`, "m");
      component(`Rod stiffness (rod ${rod})`, "1.2e8"); component(`Rod stiffness unit (rod ${rod})`, "N/m");
      component(`Reference extension (negative is pretension) (rod ${rod})`, "0"); component(`Reference extension unit (rod ${rod})`, "m");
      component(`Rod source reference (rod ${rod})`, S5);
    }
    change(inspector(), "Connector pressure model", "constant_effective_area_reference_v1");
    component("Joint effective area", "0.049"); component("Joint effective area unit", "m^2");
    change(inspector(), "Connector joint pressure basis", "internal_differential_zero_external_v1");
    component("Joint pressure area source reference", S5);
    component("Stiffness source reference", S5);
    component("Measurement restraints", "invented; no measurement");
    component("Basis transform reference", "none");
    component("Validity statement", "invented test object; no applicability claim");
    componentField("provenance", P);
    fireEvent.click(screen.getByTestId("queue-create-component-intent"));
    await applyQueuedIntent("§3.6 RS-EJ1");

    // §3.6.2 Tee RS-T1 (typed UI): New component, kind tee; then the two SIFs and the modifier source by Task edits.
    const beforeTee = model();
    chooseCompactOption(screen.getByTestId("create-component-kind"), "tee");
    chooseVirtualTarget("create-component-node", "node:RS-130");
    chooseVirtualTarget("create-component-pipe", "pipe:RS-P120");
    chooseVirtualTarget("create-component-secondary-pipe", "pipe:RS-P210");
    componentField("ID", "component:RS-T1");
    componentField("label", "Tee T1");
    component("Connection angle", "90");
    chooseCompactOption(screen.getByTestId("create-component-angle-unit"), "deg");
    chooseCompactOption(screen.getByTestId("create-component-length-unit"), "mm");
    component("Run size", "170"); component("Header size", "270");
    component("Connection type", "invented unreinforced tee");
    component("Reinforcement reference", "invented: no reinforcement");
    componentField("geometry source", S5);
    componentField("provenance", P);
    fireEvent.click(screen.getByTestId("queue-create-component-intent"));
    await applyQueuedIntent("§3.6 RS-T1");
    // Undo/Redo, component family: the tee's creation.
    await undoRedo("component: the tee RS-T1 (§3.6)", beforeTee);
    await propertyEdit({ type: "component", id: "component:RS-T1" }, /^Header SIF user value/, "2.4", "§3.6 RS-T1 header SIF");
    await propertyEdit({ type: "component", id: "component:RS-T1" }, /^Branch SIF user value/, "2.0", "§3.6 RS-T1 branch SIF");
    await propertyEdit({ type: "component", id: "component:RS-T1" }, /^Modifier source/, S5, "§3.6 RS-T1 modifier source");

    // §3.7 Variable spring RS-VS1 (typed UI): select RS-080, New support configuration, with hanger data.
    select({ type: "node", id: "node:RS-080" });
    const beforeSpring = model();
    const form = within(screen.getByRole("region", { name: "Support configuration" }));
    change(form, "Support ID", "support:RS-VS1");
    change(form, "Support label", "Variable spring VS1 (user stiffness)");
    expect(screen.getByTestId("support-node-picker").querySelector(".virtual-target-picker-current")).toHaveTextContent("node:RS-080");
    chooseCompactOption(form.getByRole("combobox", { name: "Support family" }), "variable_spring_hanger");
    change(form, "Support provenance", P);
    fireEvent.click(form.getByRole("checkbox", { name: "Include hanger data" }));
    chooseCompactOption(form.getByRole("combobox", { name: "Hanger type" }), "variable_spring_hanger");
    click(form, "Add hanger stiffness");
    chooseCompactOption(form.getByRole("combobox", { name: "Hanger stiffness DOF" }), "UZ");
    change(form, "Hanger stiffness quantity value", "180000"); change(form, "Hanger stiffness quantity unit", "N/m");
    for (const [key, value, unit] of [["installed load", "8000", "N"], ["cold load", "8000", "N"], ["hot load", "7400", "N"], ["travel range", "50", "mm"]]) {
      change(form, `Hanger ${key} value`, value); change(form, `Hanger ${key} unit`, unit);
    }
    change(form, "Hanger source reference", S5);
    change(form, "Hanger mechanics consumption", "linear_spring_primitive_user_stiffness");
    click(form, "Queue support creation");
    // The form sends the record as typed: exactly the fixture's RS-VS1, `restraints: []` (no
    // Restrain box) and hanger stiffness DOF "UZ", nothing added, normalized or defaulted.
    await waitFor(() => expect(ops().editorIntents, `§3.7 RS-VS1: queued (${alerts()})`).toHaveLength(1), { timeout: 20_000 });
    const springIntent = ops().editorIntents[0];
    expect([springIntent.operation_kind, springIntent.change.change_kind, springIntent.target]).toEqual(["create", "create_support", { object_type: "Support", ref: "support:RS-VS1" }]);
    const springPayload = JSON.parse(springIntent.change.after) as AnyRecord;
    expect(springPayload).toEqual(reference.supports.find((item) => item.id === "support:RS-VS1"));
    expect(springPayload.restraints).toEqual([]);
    expect((springPayload.hanger as AnyRecord).stiffness).toEqual({ dof: "UZ", value: { value: 180000, unit: "N/m" } });
    await applyQueuedIntent("§3.7 RS-VS1");
    expect(model().supports.find((item) => item.id === "support:RS-VS1")).toEqual(springPayload);
    // Undo/Redo, support family: the variable spring's creation.
    await undoRedo("support: the variable spring RS-VS1 (§3.7)", beforeSpring);

    // §3.8 Reference configuration (typed UI, with the member fill).
    select({ type: "project", id: model().project.id });
    const beforeReference = model();
    click(loads(), "Add reference configuration");
    change(loads(), "Reference configuration 1 ID", "reference:RS-INSTALLED");
    change(loads(), "Reference configuration 1 label", "Installed reference (20 degC)");
    change(loads(), "Reference configuration 1 geometry", "authored_model_geometry");
    click(loads(), "Add member to reference configuration 1");
    const m1 = "Reference configuration 1 member 1";
    change(loads(), `${m1} pipe`, "pipe:RS-P010");
    change(loads(), `${m1} basis`, "temperature_reference");
    change(loads(), `${m1} installation temperature value`, "20"); change(loads(), `${m1} installation temperature unit`, "degC");
    change(loads(), `${m1} fit`, "none");
    change(loads(), `${m1} provenance`, P);
    change(loads(), "Reference configuration 1 member template", "0");
    change(loads(), "Reference configuration 1 member fill targets", "missing");
    click(loads(), "Fill reference configuration 1 members");
    change(loads(), "Reference configuration 1 provenance", P);
    click(loads(), "Queue reference configurations");
    await applyQueuedIntent("§3.8 reference configuration");
    // Undo/Redo, load/reference state family: the reference configuration.
    await undoRedo("load/reference state: RS-INSTALLED (§3.8)", beforeReference);

    // §3.9 Self-weight, which creates the three cases (typed UI): every pipe except RS-P180.
    const selfWeight = within(screen.getByTestId("journey-self-weight"));
    const weighted = model().pipe_segments.map((pipe) => pipe.id).filter((id) => id !== "pipe:RS-P180");
    expect(weighted).toHaveLength(25);
    for (const [id, label] of [["load:RS-SUS", "RS-SUS sustained: W + P (operating pressure)"], ["load:RS-OPE", "RS-OPE operating: W + P + T (210 degC, modulus at 210 degC)"], ["load:RS-SAM", "RS-SAM seismic anchor movement: W + P + anchor RS-C moved UX +15 mm, UY -10 mm"]]) {
      change(selfWeight, "Self-weight action", "create");
      change(selfWeight, "Self-weight case ID", id);
      change(selfWeight, "Self-weight case label", label);
      change(selfWeight, "Gravity value", "-9.80665");
      change(selfWeight, "Gravity unit", "m/s^2");
      change(selfWeight, "Gravity direction", "global_z");
      change(selfWeight, "Self-weight provenance", `${P}; W of ${token(id)}: self-weight of every member except RS-P180, component weights at their centres of gravity (spec §8.1)`);
      // The panel keeps its pipe choice after a plan is queued: the 25 are picked for RS-SUS and kept for the next two cases.
      if (id === "load:RS-SUS") for (const pipe of weighted) chooseVirtualMultiTarget("self-weight-pipes", pipe);
      expect(screen.getByTestId("self-weight-pipes").querySelector(".virtual-target-picker-current")).toHaveTextContent("25 selected");
      // The generating model's hash, captured as the panel captures it: each primitive's
      // provenance must name exactly this hash (review r1 of #128).
      generatingHashes.set(id, (await computeModelHash(model()))!.value);
      click(selfWeight, "Generate self-weight plan");
      const queuePlan = await selfWeight.findByRole("button", { name: "Queue complete self-weight plan" }, { timeout: 20_000 });
      await act(async () => { fireEvent.click(queuePlan); });
      await applyQueuedBatch(`§3.9 ${id}`);
    }

    // §3.10 Pressure regions (typed UI for RS-SUS; "Copy pressure regions from another case" for RS-OPE and RS-SAM).
    const terminals = { "RG-1": ["node:RS-010", "node:RS-130"], "RG-2": ["node:RS-130", "node:RS-230"], "RG-3": ["node:RS-130", "node:RS-150"], "RG-R": ["node:RS-150", "node:RS-160"], "RG-4": ["node:RS-160", "node:RS-210"] } as const;
    const fixtureRegions = reference.load_cases.find((item) => item.id === "load:RS-SUS")!.pressure_regions!;
    select({ type: "load", id: "load:RS-SUS" });
    const regions = () => within(loads().getByRole("region", { name: "Case pressure regions" }));
    for (const [index, region] of fixtureRegions.entries()) {
      const n = `Region ${index + 1}`;
      click(regions(), "Add pressure region");
      change(regions(), `${n} ID`, String(region.id));
      pick(regions(), `${n} member pipes`, region.member_pipe_ids as string[]);
      if ((region.member_connector_ids as string[] | undefined)?.length) pick(regions(), `${n} member connectors`, region.member_connector_ids as string[]);
      change(regions(), `${n} pressure basis`, "internal_differential_zero_external_v1");
      change(regions(), `${n} pressure value`, "1600"); change(regions(), `${n} pressure unit`, "kPa");
      const ends = terminals[String(region.id).replace("region:RS-SUS-", "") as keyof typeof terminals];
      for (const t of [1, 2]) {
        change(regions(), `${n} terminal ${t} node`, ends[t - 1]);
        change(regions(), `${n} terminal ${t} closure transfer`, "transfers_to_wall");
        change(regions(), `${n} terminal ${t} provenance`, P);
      }
      change(regions(), `${n} provenance`, P);
    }
    click(regions(), "Queue pressure regions");
    await applyQueuedIntent("§3.10 RS-SUS regions");
    for (const caseId of ["load:RS-OPE", "load:RS-SAM"]) {
      select({ type: "load", id: caseId });
      change(regions(), "Copy pressure regions from case", "load:RS-SUS");
      click(regions(), "Replace this draft's pressure regions with the copy");
      click(regions(), "Queue pressure regions");
      await applyQueuedIntent(`§3.10 ${caseId} regions`);
    }

    // §3.11 Analysis states (typed UI, with the fills; RS-SAM from "Copy analysis state from another case").
    // Queries are scoped to the case's analysis-state editor (`LoadReferenceStateInputs`), which the load manager shows for the selected case.
    const state = () => within(screen.getByTestId("load-state-analysis-state"));
    const p180 = model().pipe_segments.findIndex((pipe) => pipe.id === "pipe:RS-P180") + 1;
    const rsC = model().supports.findIndex((item) => item.id === "support:RS-C") + 1;
    const replacedSpan = `${P}; replaced span RS-P180: zero eigenstrain (spec §5.1)`;
    const fill = (name: string) => {
      change(state(), `${name} template`, "0");
      change(state(), `${name} fill targets`, "missing");
      click(state(), `Fill ${name.toLowerCase()}s`);
    };
    const enterCommon = () => {
      change(state(), "Analysis state contract", "openpipestress.load_reference_state/1.0.0");
      change(state(), "Reference configuration", "reference:RS-INSTALLED");
      click(state(), "Add support state");
      change(state(), "Support state 1 support", "support:RS-A");
      change(state(), "Support state 1 participation", "active_model_device");
      fill("Support state");
      change(state(), "Factor for every primitive (dimensionless)", "1");
      click(state(), "Add every primitive of this case");
      change(state(), "History", "independent_equilibrium");
      change(state(), "Analysis state provenance", P);
    };
    // RS-SUS.
    select({ type: "load", id: "load:RS-SUS" });
    click(state(), "Add element state");
    change(state(), "Element state 1 pipe", "pipe:RS-P010");
    change(state(), "Element state 1 material selection", "exact_point");
    change(state(), "Element state 1 material", "material:RS-M1");
    change(state(), "Element state 1 material point", "point:RS-M1-T020");
    change(state(), "Element state 1 thermal state", "unchanged_reference");
    change(state(), "Element state 1 thermal provenance", `${P}; no temperature in this case`);
    fill("Element state");
    expect(state().getByLabelText(`Element state ${p180} pipe`)).toHaveValue("pipe:RS-P180");
    change(state(), `Element state ${p180} thermal provenance`, replacedSpan);
    enterCommon();
    click(state(), "Queue analysis state");
    await applyQueuedIntent("§3.11 RS-SUS state");
    // RS-OPE.
    select({ type: "load", id: "load:RS-OPE" });
    click(state(), "Add element state");
    change(state(), "Element state 1 pipe", "pipe:RS-P010");
    change(state(), "Element state 1 operating temperature value", "210"); change(state(), "Element state 1 operating temperature unit", "degC");
    change(state(), "Element state 1 material selection", "temperature_interpolation");
    change(state(), "Element state 1 material", "material:RS-M1");
    change(state(), "Element state 1 selection temperature value", "210"); change(state(), "Element state 1 selection temperature unit", "degC");
    change(state(), "Element state 1 interpolation", "piecewise_linear");
    change(state(), "Element state 1 extrapolation", "forbidden");
    change(state(), "Element state 1 thermal state", "free_length_state");
    change(state(), "Element state 1 expansion law", "law:RS-M1-SECANT");
    fill("Element state");
    change(state(), `Element state ${p180} operating temperature value`, ""); change(state(), `Element state ${p180} operating temperature unit`, "");
    change(state(), `Element state ${p180} thermal state`, "unchanged_reference");
    change(state(), `Element state ${p180} thermal provenance`, replacedSpan);
    enterCommon();
    click(state(), "Queue analysis state");
    await applyQueuedIntent("§3.11 RS-OPE state");
    // RS-SAM: copied from RS-SUS; its own primitives and RS-C's two motions.
    select({ type: "load", id: "load:RS-SAM" });
    change(state(), "Copy analysis state from case", "load:RS-SUS");
    click(state(), "Replace this draft's analysis state with the copy");
    expect(state().getByText(/28 load sources name primitives this case does not hold/)).toBeTruthy();
    change(state(), "Factor for every primitive (dimensionless)", "1");
    click(state(), "Add every primitive of this case");
    click(state(), `Add Support state ${rsC} boundary motion`);
    click(state(), `Add Support state ${rsC} boundary motion`);
    for (const [m, dof, value] of [[1, "UX", "15"], [2, "UY", "-10"]] as const) {
      change(state(), `Support state ${rsC} motion ${m} DOF`, dof);
      change(state(), `Support state ${rsC} motion ${m} displacement value`, value);
      change(state(), `Support state ${rsC} motion ${m} displacement unit`, "mm");
      change(state(), `Support state ${rsC} motion ${m} meaning`, "absolute_reference_displacement");
    }
    click(state(), "Queue analysis state");
    await applyQueuedIntent("§3.11 RS-SAM state");

    // §3.12 Combinations (typed UI): Create combination, result_state_subtraction.
    select({ type: "project", id: model().project.id });
    let beforeCombination = model();
    for (const [id, label, minuend] of [["combination:RS-EXP", "RS-EXP expansion: RS-OPE - RS-SUS", "load:RS-OPE"], ["combination:RS-dSAM", "RS-dSAM anchor movement term: RS-SAM - RS-SUS", "load:RS-SAM"]]) {
      beforeCombination = model();
      fireEvent.change(screen.getByTestId("load-manager-create-combination-id"), { target: { value: id } });
      fireEvent.change(screen.getByTestId("load-manager-create-combination-label"), { target: { value: label } });
      fireEvent.change(screen.getByTestId("load-manager-create-combination-basis"), { target: { value: "result_state_subtraction" } });
      chooseVirtualTarget("load-manager-create-combination-minuend", minuend);
      chooseVirtualTarget("load-manager-create-combination-subtrahend", "load:RS-SUS");
      fireEvent.change(screen.getByTestId("load-manager-create-combination-provenance"), { target: { value: P } });
      fireEvent.change(screen.getByLabelText("New combination rationale"), { target: { value: "reference system run sheet §3.12" } });
      fireEvent.click(screen.getByTestId("queue-create-combination-intent"));
      await applyQueuedIntent(`§3.12 ${id}`);
    }
    // Undo/Redo, combination family: RS-dSAM.
    await undoRedo("combination: RS-dSAM (§3.12)", beforeCombination);

    // The comparison, by record keyed by id, with the §3.14 differences allowed by member path.
    expect(BLOCKED).toEqual([]);
    expect(ALLOWED_DIFFERENCES).toHaveLength(15);
    expect(DOCUMENT_DIFFERENCES_NOT_IN_3_14).toEqual(["data_boundary", "diagnostics"]);
    // 27 queue/apply cycles: the run sheet's §5 count of 27 per-intent cycles.
    expect(cycles).toHaveLength(27);
    expect(undoRedone).toEqual([
      "geometry and supports: the run sheet's batch (§3.4)", "component: the tee RS-T1 (§3.6)",
      "support: the variable spring RS-VS1 (§3.7)", "load/reference state: RS-INSTALLED (§3.8)", "combination: RS-dSAM (§3.12)",
    ]);
    const compared = compareWithFixture(model(), "entered");
    // materials, sections, nodes, pipes, components, supports, cases, combinations, reference configurations: 82 of 82.
    expect(compared).toBe(1 + 3 + 27 + 26 + 8 + 11 + 3 + 2 + 1);
    expect(compared).toBe(82);

    // Browser save and reopen through the session (`handleSaveProject`, `handleOpenProject`), compared again.
    const projectId = model().project.id;
    const entered = model();
    await act(async () => { await current.session.project.handleSaveProject(); });
    await waitFor(() => expect(current.session.project.projectBusy).toBe(false));
    expect(current.session.project.modelEdited, current.session.project.projectMessage ?? "").toBe(false);
    await act(async () => { await current.session.project.handleOpenProject(projectId); });
    await waitFor(() => expect(current.session.project.projectBusy).toBe(false));
    expect(model()).not.toBe(entered);
    expect(model()).toEqual(entered);
    expect(compareWithFixture(model(), "reopened")).toBe(compared);
  }, 300_000);
});
