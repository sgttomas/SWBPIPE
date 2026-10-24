// W-13 S2a (design model-document-0.7.0.md §6): the desktop authoring of the
// model-data slots. Each control queues a structured operation; the test
// applies the queued intent through the wasm applier (validated, diffed) and
// checks the emitted document. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { openLocalProject, refuseMalformedModelSlots, saveLocalProject } from "../../services/projectService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { entityKey, type OrderedSelectionState } from "../workspace/selectionState";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { SupportConfigurationForm } from "../support-configuration/SupportConfigurationForm";
import { ModelDataAnnotations } from "./ModelDataAnnotations";
import { LinesSection } from "./LinesSection";
import { AssumptionsPanel, filterAssumptions } from "./AssumptionsPanel";
import { ConstraintsPanel, NOT_CHECKED_NOTICE } from "./ConstraintsPanel";
import { RulePackModelReferences } from "./RulePackModelReferences";
import { modelDataIntent, parseTags, rulePackReferenceFromDocument } from "./modelDataIntents";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const read = (relative: string) => JSON.parse(readFileSync(path.join(projectRoot, relative), "utf8")) as Record<string, unknown>;
const everySlot = () => read("fixtures/model_document/invented_every_slot_model.json") as unknown as PreviewModel;
const demoPack = () => read("fixtures/product_preview/invented_demo_rule_pack.json");

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const hash = await computeModelHash(model);
  const preview = await validateModelOperation(model, intent, hash);
  expect(preview.diff_preview.length).toBeGreaterThan(0);
  const outcome = await applyModelOperation(model, intent, hash);
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}

function selection(refs: EntityRef[]): OrderedSelectionState {
  const keys = refs.map(entityKey);
  return { orderedKeys: keys, primaryKey: keys[0] ?? null, rangeAnchorKey: null, focusKey: keys[0] ?? null, preparationEpoch: 0 };
}

async function queued(list: EditorOperationIntent[], count: number): Promise<EditorOperationIntent> {
  await waitFor(() => expect(list).toHaveLength(count));
  return list[count - 1];
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("model-data intents", () => {
  it("parses tags, derives operation kinds and projects a stored pack to a reference", () => {
    expect(parseTags(" tie-in, rev-B,\ntie-in , ,x")).toEqual(["tie-in", "rev-B", "x"]);
    const base = { objectType: "Line" as const, ref: "line:L-1", fieldPath: "lines", before: "not_present", after: "{}", label: "x" };
    expect(modelDataIntent({ ...base, changeKind: "create_line" }).operation_kind).toBe("create");
    expect(modelDataIntent({ ...base, changeKind: "update_line" }).operation_kind).toBe("modify");
    expect(modelDataIntent({ ...base, changeKind: "delete_line" }).operation_kind).toBe("delete");
    expect(modelDataIntent({ ...base, objectType: "RulePackRef", changeKind: "remove_rule_pack_ref" }).operation_kind).toBe("delete");
    const reference = rulePackReferenceFromDocument(demoPack());
    expect(reference).toEqual({
      rule_pack_id: "invented_demo_rule_pack",
      name: "Invented Demonstration Rule Pack",
      rule_pack_version: "0.2.0",
      checksum: { algorithm: "sha256", canonicalization: "JCS", payload_scope: "rule_pack_payload", payload_excludes: ["checksums"],
        value: "c2b4ddce48cd07bed4bb2ff0be3a81d3755365e255896a0bbf583725a3fa47bd" },
      redistribution_status: "invented_non_engineering_example",
      source_notice: expect.stringContaining("Original invented demonstration data only")
    });
    const unstamped = demoPack();
    (unstamped.checksums as Record<string, unknown>).rule_pack_checksum = {};
    expect(() => rulePackReferenceFromDocument(unstamped)).toThrow(/stamped checksum/);
  });
});

describe("property inspector: tags, notes and provenance record", () => {
  it("queues tags and notes for the selected entity and the project, and clearing removes them", async () => {
    let model = everySlot();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelDataAnnotations model={model} selection={{ type: "pipe", id: "pipe:P-2" }} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(screen.getByTestId("annotations-pipe-tags"), { target: { value: "riser, rev-C" } });
    fireEvent.click(screen.getByTestId("annotations-pipe-tags-queue"));
    const tags = await queued(list, 1);
    expect(tags.target).toEqual({ object_type: "Element", ref: "pipe:P-2" });
    expect(tags.change).toMatchObject({ change_kind: "set_field", field_path: "tags", before: "not_present", after: JSON.stringify(["riser", "rev-C"]) });
    model = await apply(model, tags);
    expect(model.pipe_segments[1].tags).toEqual(["riser", "rev-C"]);

    view.rerender(<ModelDataAnnotations model={model} selection={{ type: "pipe", id: "pipe:P-2" }} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(screen.getByTestId("annotations-project-notes"), { target: { value: "Edited project note." } });
    fireEvent.click(screen.getByTestId("annotations-project-notes-queue"));
    const notes = await queued(list, 2);
    expect(notes.target).toEqual({ object_type: "Project", ref: "project:slots" });
    model = await apply(model, notes);
    expect(model.project.notes).toBe("Edited project note.");

    view.rerender(<ModelDataAnnotations model={model} selection={{ type: "pipe", id: "pipe:P-2" }} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(screen.getByTestId("annotations-pipe-tags-clear"));
    const cleared = await queued(list, 3);
    expect(cleared.change.after).toBe("null");
    model = await apply(model, cleared);
    expect(model.pipe_segments[1]).not.toHaveProperty("tags");
  });

  it("uses each family's change kind and labels review_status Source review", async () => {
    let model = everySlot();
    const list: EditorOperationIntent[] = [];
    render(<ModelDataAnnotations model={model} selection={{ type: "support", id: "support:S-1" }} onQueueIntent={(intent) => list.push(intent)} />);
    const editor = screen.getByTestId("provenance-record-editor");
    expect(within(editor).getByLabelText("Source review")).toBeInTheDocument();
    expect(within(editor).getByText("Source review records the review of the source data only.")).toBeInTheDocument();
    fireEvent.change(within(editor).getByTestId("provenance-record-source_name"), { target: { value: "Invented vendor sheet (rev 2)" } });
    fireEvent.click(within(editor).getByTestId("provenance-record-queue"));
    const record = await queued(list, 1);
    expect(record.change).toMatchObject({ change_kind: "update_support", field_path: "provenance_record" });
    model = await apply(model, record);
    expect(model.supports[0].provenance_record?.source_name).toBe("Invented vendor sheet (rev 2)");
    expect(model.supports[0].provenance_record?.review_status).toBe("pending");
  });

  it("offers no provenance record on nodes and pipes", () => {
    render(<ModelDataAnnotations model={everySlot()} selection={{ type: "node", id: "node:N-1" }} onQueueIntent={() => undefined} />);
    expect(screen.queryByTestId("provenance-record-editor")).toBeNull();
    expect(screen.getByTestId("annotations-node")).toBeInTheDocument();
  });
});

describe("model tree: lines", () => {
  it("creates a line from the selected pipes, renames it and removes it", async () => {
    let model = everySlot();
    model.lines = [];
    const list: EditorOperationIntent[] = [];
    const props = { onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) };
    const view = render(<LinesSection model={model} selectionState={selection([{ type: "pipe", id: "pipe:P-1" }, { type: "pipe", id: "pipe:P-2" }])} {...props} />);
    fireEvent.change(screen.getByTestId("line-create-label"), { target: { value: "Pump discharge" } });
    fireEvent.click(screen.getByTestId("line-create"));
    const create = await queued(list, 1);
    expect(create.change.change_kind).toBe("create_line");
    model = await apply(model, create);
    expect(model.lines).toEqual([{ id: "line:L-001", label: "Pump discharge", pipe_refs: ["pipe:P-1", "pipe:P-2"], provenance: "user_entered" }]);

    view.rerender(<LinesSection model={model} selectionState={selection([{ type: "pipe", id: "pipe:P-1" }])} {...props} />);
    expect(screen.getByTestId("line-create")).toBeDisabled();
    expect(screen.getByTestId("line-create-conflict")).toHaveTextContent("pipe:P-1 belongs to line:L-001");
    fireEvent.change(screen.getByTestId("line-rename-input-line:L-001"), { target: { value: "Discharge" } });
    fireEvent.click(screen.getByTestId("line-rename-line:L-001"));
    model = await apply(model, await queued(list, 2));
    expect(model.lines?.[0].label).toBe("Discharge");

    view.rerender(<LinesSection model={model} selectionState={selection([])} {...props} />);
    fireEvent.click(screen.getByTestId("line-remove-line:L-001"));
    model = await apply(model, await queued(list, 3));
    expect(model).not.toHaveProperty("lines");
  });
});

describe("assumptions and open issues", () => {
  it("adds a record for the selected subject, requires a note to resolve, and filters by status and subject", async () => {
    let model = everySlot();
    const list: EditorOperationIntent[] = [];
    const props = { onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) };
    const view = render(<AssumptionsPanel model={model} selection={{ type: "node", id: "node:N-3" }} {...props} />);
    expect(screen.getByTestId("assumption-create-subject")).toHaveTextContent("node:N-3");
    fireEvent.change(screen.getByTestId("assumption-create-kind"), { target: { value: "open_issue" } });
    fireEvent.change(screen.getByTestId("assumption-create-statement"), { target: { value: "Invented open issue at the end." } });
    fireEvent.click(screen.getByTestId("assumption-create"));
    model = await apply(model, await queued(list, 1));
    const added = model.unresolved_assumptions!.find((item) => item.assumption_id === "assumption:A-003")!;
    expect(added).toEqual({ assumption_id: "assumption:A-003", kind: "open_issue", statement: "Invented open issue at the end.", status: "unresolved", affected_refs: ["node:N-3"] });

    view.rerender(<AssumptionsPanel model={model} selection={null} {...props} />);
    const row = screen.getByTestId("assumption-row-assumption:A-003");
    expect(within(row).getByTestId("assumption-resolve-assumption:A-003")).toBeDisabled();
    fireEvent.change(within(row).getByTestId("assumption-note-assumption:A-003"), { target: { value: "Invented resolution." } });
    fireEvent.click(within(row).getByTestId("assumption-resolve-assumption:A-003"));
    model = await apply(model, await queued(list, 2));
    expect(model.unresolved_assumptions!.find((item) => item.assumption_id === "assumption:A-003")).toMatchObject({ status: "resolved", resolution_note: "Invented resolution." });

    view.rerender(<AssumptionsPanel model={model} selection={null} {...props} />);
    fireEvent.change(screen.getByTestId("assumptions-filter-status"), { target: { value: "unresolved" } });
    expect(screen.getByTestId("assumption-row-assumption:A-001")).toBeInTheDocument();
    expect(screen.queryByTestId("assumption-row-assumption:A-003")).toBeNull();
    expect(filterAssumptions(model.unresolved_assumptions!, "all", "node:N-3").map((item) => item.assumption_id)).toEqual(["assumption:A-003"]);
    expect(filterAssumptions(model.unresolved_assumptions!, "all", "__project__").map((item) => item.assumption_id)).toEqual(["assumption:A-002"]);
  });
});

describe("constraints and equipment interfaces", () => {
  it("says the records are not checked and adds and edits both kinds", async () => {
    let model = everySlot();
    const list: EditorOperationIntent[] = [];
    const props = { onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) };
    const view = render(<ConstraintsPanel model={model} selection={{ type: "node", id: "node:N-3" }} {...props} />);
    expect(screen.getByTestId("constraints-not-checked")).toHaveTextContent(NOT_CHECKED_NOTICE);
    fireEvent.change(screen.getByTestId("constraint-create-name"), { target: { value: "Invented access" } });
    fireEvent.change(screen.getByTestId("constraint-create-kind"), { target: { value: "access" } });
    fireEvent.click(screen.getByTestId("constraint-create"));
    model = await apply(model, await queued(list, 1));
    expect(model.constraints?.[1]).toEqual({ constraint_id: "constraint:K-002", name: "Invented access", constraint_kind: "access", state: "proposed", target_refs: ["node:N-3"], provenance: "user_entered" });

    view.rerender(<ConstraintsPanel model={model} selection={{ type: "node", id: "node:N-3" }} {...props} />);
    fireEvent.change(screen.getByTestId("equipment-interface-create-label"), { target: { value: "Invented nozzle" } });
    fireEvent.click(screen.getByTestId("equipment-interface-create"));
    model = await apply(model, await queued(list, 2));
    expect(model.equipment_interfaces?.[1]).toEqual({ id: "equipment-interface:E-002", label: "Invented nozzle", node_ref: "node:N-3", provenance: "user_entered" });

    view.rerender(<ConstraintsPanel model={model} selection={null} {...props} />);
    fireEvent.change(screen.getByTestId("constraint-state-constraint:K-002"), { target: { value: "active" } });
    fireEvent.click(screen.getByTestId("constraint-edit-constraint:K-002"));
    model = await apply(model, await queued(list, 3));
    expect(model.constraints?.[1].state).toBe("active");
  });
});

describe("row editors follow their stored record", () => {
  it("re-seeds after apply and after undo, and Save is disabled for an unchanged draft", async () => {
    const original = everySlot();
    const list: EditorOperationIntent[] = [];
    const props = { onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) };
    const view = render(<ConstraintsPanel model={original} selection={null} {...props} />);
    const name = () => screen.getByTestId("constraint-name-constraint:K-001");
    const save = () => screen.getByTestId("constraint-edit-constraint:K-001");
    expect(save()).toBeDisabled();
    fireEvent.change(name(), { target: { value: "Invented clearance (edited)" } });
    expect(save()).toBeEnabled();
    fireEvent.click(save());
    const edited = await apply(original, await queued(list, 1));
    view.rerender(<ConstraintsPanel model={edited} selection={null} {...props} />);
    expect(name()).toHaveValue("Invented clearance (edited)");
    expect(save()).toBeDisabled();
    // Undo restores the session snapshot: the row shows the stored name again.
    view.rerender(<ConstraintsPanel model={original} selection={null} {...props} />);
    expect(name()).toHaveValue("Invented clearance");
    expect(save()).toBeDisabled();

    cleanup();
    const lines = render(<LinesSection model={original} selectionState={selection([])} {...props} />);
    fireEvent.change(screen.getByTestId("line-rename-input-line:L-100"), { target: { value: "Renamed" } });
    fireEvent.click(screen.getByTestId("line-rename-line:L-100"));
    const renamed = await apply(original, await queued(list, 2));
    lines.rerender(<LinesSection model={renamed} selectionState={selection([])} {...props} />);
    lines.rerender(<LinesSection model={original} selectionState={selection([])} {...props} />);
    expect(screen.getByTestId("line-rename-input-line:L-100")).toHaveValue("Pump discharge");
    expect(screen.getByTestId("line-rename-line:L-100")).toBeDisabled();

    cleanup();
    const issues = render(<AssumptionsPanel model={original} selection={null} {...props} />);
    fireEvent.change(screen.getByTestId("assumption-statement-assumption:A-001"), { target: { value: "Edited statement." } });
    fireEvent.click(screen.getByTestId("assumption-edit-assumption:A-001"));
    const restated = await apply(original, await queued(list, 3));
    issues.rerender(<AssumptionsPanel model={restated} selection={null} {...props} />);
    issues.rerender(<AssumptionsPanel model={original} selection={null} {...props} />);
    expect(screen.getByTestId("assumption-statement-assumption:A-001")).toHaveValue("Nozzle treated as anchor pending invented vendor loads.");
    expect(screen.getByTestId("assumption-edit-assumption:A-001")).toBeDisabled();
  });

  it("compares the provenance draft key-order independently", () => {
    const model = everySlot();
    const stored = model.materials![0].provenance_record!;
    // The stored record with library_ref first: the same record in another key order.
    model.materials![0].provenance_record = { library_ref: stored.library_ref, ...stored };
    render(<ModelDataAnnotations model={model} selection={{ type: "material", id: "material:steel" }} onQueueIntent={() => undefined} />);
    expect(screen.getByTestId("provenance-record-queue")).toBeDisabled();
  });
});

describe("rule pack manager: use with this model", () => {
  it("references a stored pack by its stamped checksum and removes the reference", async () => {
    let model = everySlot();
    delete model.rule_pack_refs;
    const list: EditorOperationIntent[] = [];
    const entry = { project_id: "project:slots", rule_pack_id: "invented_demo_rule_pack", rule_pack_name: "Invented Demonstration Rule Pack",
      rule_pack_version: "0.2.0", lifecycle_status: "accepted_public_example", privacy_class: "public_invented_example",
      storage_mode: "sqlite", created_at_unix: 0, updated_at_unix: 0 };
    const openPack = async () => ({ route: "tauri_backend" as const, envelope: { project_id: "project:slots", rule_pack_id: entry.rule_pack_id,
      storage_mode: "sqlite", created_at_unix: 0, updated_at_unix: 0, document: demoPack(), validation: {} as never, message: "Opened." } });
    const props = { entries: [entry], openPack, onQueueIntent: (intent: EditorOperationIntent) => list.push(intent) };
    const view = render(<RulePackModelReferences model={model} listRefreshed {...props} />);
    await act(async () => fireEvent.click(screen.getByTestId("rule-pack-use-with-model-project:slots-invented_demo_rule_pack")));
    const add = await queued(list, 1);
    expect(add.change.change_kind).toBe("add_rule_pack_ref");
    model = await apply(model, add);
    expect(model.rule_pack_refs?.[0].checksum.value).toBe("c2b4ddce48cd07bed4bb2ff0be3a81d3755365e255896a0bbf583725a3fa47bd");

    // The same pack id stored under two projects renders two distinct entries.
    view.rerender(<RulePackModelReferences model={model} listRefreshed {...props} entries={[entry, { ...entry, project_id: "project:other" }]} />);
    expect(screen.getByTestId("rule-pack-use-with-model-project:slots-invented_demo_rule_pack")).toBeDisabled();
    expect(screen.getByTestId("rule-pack-use-with-model-project:other-invented_demo_rule_pack")).toBeInTheDocument();
    view.rerender(<RulePackModelReferences model={model} listRefreshed {...props} entries={[]} />);
    expect(screen.getByTestId("rule-pack-ref-unresolved-invented_demo_rule_pack")).toHaveTextContent("MODEL-RULE-PACK-REF-UNRESOLVED");
    fireEvent.click(screen.getByTestId("rule-pack-remove-from-model-invented_demo_rule_pack"));
    model = await apply(model, await queued(list, 2));
    expect(model).not.toHaveProperty("rule_pack_refs");
  });
});

describe("validator findings on the desktop", () => {
  it("lists the validator's warnings in the diagnostics list", () => {
    const model = everySlot();
    model.unresolved_assumptions![0].affected_refs = ["support:S-absent"];
    render(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => undefined} />);
    expect(screen.getByTestId("diagnostic-MODEL-ASSUMPTION-SUBJECT-MISSING")).toHaveTextContent("unresolved_assumptions[0].affected_refs[0]");
  });

  it("refuses to store a document with a blocking finding, by name", () => {
    const model = everySlot();
    expect(() => refuseMalformedModelSlots(model)).not.toThrow();
    model.lines![0].pipe_refs = ["pipe:P-absent"];
    expect(() => refuseMalformedModelSlots(model)).toThrow(/MODEL-LINE-PIPE-MISSING at lines\[0\]\.pipe_refs\[0\]/);
  });

  it("browser-preview save refuses a malformed document by name and save then open keeps a valid one", async () => {
    const valid = everySlot();
    valid.project.id = "project:browser-slots";
    valid.unresolved_assumptions![0].affected_refs = ["support:S-absent"]; // a warning never refuses
    const saved = await saveLocalProject(valid);
    expect(saved.model).toEqual(valid);
    const opened = await openLocalProject("project:browser-slots");
    expect(opened?.model).toEqual(valid);

    const malformed = everySlot();
    malformed.project.id = "project:browser-malformed";
    malformed.constraints![0].constraint_kind = "equipment_interface";
    await expect(saveLocalProject(malformed)).rejects.toThrow(/MODEL-CONSTRAINT-KIND-INVALID at constraints\[0\]\.constraint_kind/);
    expect(await openLocalProject("project:browser-malformed")).toBeNull();
  });
});

describe("support configuration: nonlinear null", () => {
  it("reads support.nonlinear: null as a linear support", () => {
    const model = everySlot();
    (model.supports[0] as Record<string, unknown>).nonlinear = null;
    render(<SupportConfigurationForm model={model} selection={{ type: "support", id: "support:S-1" }} onQueueIntent={() => undefined} />);
    expect(screen.getByLabelText("Include nonlinear behavior")).not.toBeChecked();
  });
});
