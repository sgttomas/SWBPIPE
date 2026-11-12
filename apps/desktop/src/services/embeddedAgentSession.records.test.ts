// The record producer of swbpipe_read_records (swbpipe-catalog/5) against the operation
// engine: for each family the catalog admits edits or deletions on, an edit and a deletion
// are proposed with the before texts the producer returns, stamped as the agent's, and
// applied through the desktop batch service on the wasm engine. The engine compares every
// before text with the current value, so an application that passes shows the text is the
// one it compares; the same operation with the text changed by one character is refused as
// stale. Scalar field edits take their stored value from recordText, as the tool says.
// Route checks with scripted inputs on committed invented fixtures; nothing here is a
// person's entry or a use case completed.
import { describe, expect, it } from "vitest";
import referenceJson from "../../../../fixtures/reference_system/reference_system.json";
import demoJson from "../../../../fixtures/product_preview/invented_demo_model.json";
import m2 from "../../../../fixtures/model_operations/agent_m2_component_journey.json";
import catalog from "../../src-tauri/resources/agent-capability-catalog.json";
import type { PreviewModel } from "../types";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { familyRecords, RECORD_FAMILIES, recordBefore, stampAgentOperations, type RecordFamily } from "./embeddedAgentSession";
import { writesUserEnteredModifier } from "./operationAuthorship";
import { applyOperationBatch } from "./operationBatchService";
import { canonicalJsonString, computeModelHash } from "./hashService";

type AnyRecord = Record<string, unknown>;
const P = "invented: agent route check";
const STALE = ["OP-STALE-BEFORE-VALUE", "OP-BEFORE-VALUE-MISMATCH"];

async function read(model: PreviewModel, family: RecordFamily, id: string) {
  const entry = familyRecords(model, family).find((row) => row.id === id);
  if (!entry) throw new Error(`${family}: no ${id}`);
  const recordText = await canonicalJsonString(entry.text);
  return { recordText, record: JSON.parse(recordText) as AnyRecord, before: await recordBefore(family, entry.record, recordText) };
}
function op(kind: string, object: string, ref: string, changeKind: string, path: string, before: string, after: string, unit = "none", dimension = "dimensionless"): AnyRecord {
  return { operation_id: `op:${changeKind}:${ref}:${path}`, operation_kind: kind, target: { object_type: object, ref },
    change: { change_id: `change:${changeKind}:${ref}:${path}`, change_kind: changeKind, field_label: path, field_path: path, before, after, unit, dimension, source_note: P }, rationale: P };
}
async function run(model: PreviewModel, operation: AnyRecord) {
  const hash = await computeModelHash(model);
  return applyOperationBatch(model, { batch_id: "agent-records", operations: stampAgentOperations([operation], "proposal", "request"), source_model_hash: hash!.value }, hash);
}
const codes = (outcome: Awaited<ReturnType<typeof run>>) => outcome.diagnostics.filter((d) => d.severity === "blocking").map((d) => `${d.code}: ${d.message}`);
/** Applies the operation (it must pass), after showing that its before text with one more digit is refused as stale (a numeric before is compared by value). */
async function applied(model: PreviewModel, operation: AnyRecord): Promise<PreviewModel> {
  const change = operation.change as AnyRecord;
  const tampered = await run(model, { ...operation, change: { ...change, before: `${String(change.before)}1` } });
  expect(tampered.applied_model, `${String(change.change_kind)} ${String(change.field_path)}: tampered before`).toBeNull();
  expect(codes(tampered).some((code) => STALE.some((stale) => code.startsWith(stale))), `${String(change.field_path)}: ${codes(tampered).join(" | ")}`).toBe(true);
  const outcome = await run(model, operation);
  expect(outcome.validation.batch_validation_status, `${String(change.change_kind)} ${String(change.field_path)}: ${codes(outcome).join(" | ")}`).toBe("passed");
  return outcome.applied_model!;
}
const json = (value: unknown) => JSON.stringify(value);
const find = (model: PreviewModel, collection: string, id: string) => ((model as unknown as AnyRecord)[collection] as AnyRecord[]).find((row) => row.id === id)!;

describe("swbpipe_read_records before texts are the ones the engine compares", () => {
  it("produces exactly the catalog's record families", () => {
    expect([...RECORD_FAMILIES]).toEqual(catalog.definitions.recordsInput.properties.family.enum);
    expect([...RECORD_FAMILIES]).toEqual(catalog.definitions.recordsData.properties.family.enum);
  });

  it("projects load cases, primitive loads and the project without cutting a member", async () => {
    const model = structuredClone(referenceJson) as unknown as PreviewModel;
    const sus = await read(model, "load_cases", "load:RS-SUS");
    expect(Object.keys(sus.record).sort()).toEqual(["id", "kind", "label", "provenance", "status"]);
    const stored = find(model, "load_cases", "load:RS-SUS");
    expect(sus.before.analysis_state).toBe(await canonicalJsonString(stored.analysis_state));
    expect(sus.before.pressure_regions).toBe(await canonicalJsonString(stored.pressure_regions));
    expect(sus.before.condition_use).toBe("not_present");
    const loads = familyRecords(model, "primitive_loads");
    expect(loads).toHaveLength(84);
    const first = await read(model, "primitive_loads", String(loads[0].id));
    expect(first.record).toEqual({ load_case: "load:RS-SUS", primitive_load: (stored.primitive_loads as AnyRecord[])[0] });
    expect(first.before).toEqual({});
    const project = await read(model, "project", model.project.id);
    expect(project.record).toEqual({ project: model.project, reference_configurations: (model as unknown as AnyRecord).reference_configurations });
    expect(project.before).toEqual({ reference_configurations: await canonicalJsonString((model as unknown as AnyRecord).reference_configurations),
      "model_settings.node_proximity_tolerance": "not_present", "model_settings.shear_model": "not_present" });
  });

  it("exact route: materials, sections, pipes, components, reference configuration, load cases and combinations", async () => {
    let model = structuredClone(referenceJson) as unknown as PreviewModel;
    // Materials: the three structured members and a scalar field.
    let material = await read(model, "materials", "material:RS-M1");
    const constitutive = JSON.parse(material.before.constitutive_properties) as AnyRecord;
    model = await applied(model, op("modify", "Material", "material:RS-M1", "set_field", "constitutive_properties", material.before.constitutive_properties, json({ ...constitutive, provenance: P })));
    material = await read(model, "materials", "material:RS-M1");
    const points = JSON.parse(material.before.temperature_points) as AnyRecord[];
    model = await applied(model, op("modify", "Material", "material:RS-M1", "set_field", "temperature_points", material.before.temperature_points, json([{ ...points[0], provenance: P }, ...points.slice(1)])));
    material = await read(model, "materials", "material:RS-M1");
    const laws = JSON.parse(material.before.expansion_laws) as AnyRecord[];
    model = await applied(model, op("modify", "Material", "material:RS-M1", "set_field", "expansion_laws", material.before.expansion_laws, json([{ ...laws[0], provenance: P }])));
    material = await read(model, "materials", "material:RS-M1");
    const modulus = material.record.elastic_modulus as { value: number; unit: string };
    model = await applied(model, op("modify", "Material", "material:RS-M1", "set_field", "elastic_modulus.value", String(modulus.value), json({ value: modulus.value, unit: modulus.unit }), modulus.unit, "stress"));
    expect(find(model, "materials", "material:RS-M1").provenance).toBe(P);
    // A material and a section of the agent's own, then their deletions.
    const created = { id: "material:agent-check", label: "agent check", constitutive_basis: "homogeneous_isotropic_E_nu_v1", elastic_modulus: { value: 200000000000, unit: "Pa" }, poisson_ratio: { value: 0.3, unit: "1" }, provenance: P };
    model = await applied(model, op("create", "Material", "material:agent-check", "create_material", "materials", "not_present", json(created), "Pa", "stress"));
    model = await applied(model, op("delete", "Material", "material:agent-check", "delete_material", "materials", (await read(model, "materials", "material:agent-check")).before.delete, "deleted"));
    const s270 = find(model, "sections", "section:RS-S270");
    model = await applied(model, op("create", "Section", "section:agent-S270", "create_section", "sections", "not_present", json({ ...s270, id: "section:agent-S270", name: "agent-S270", provenance: P }), "mm", "length"));
    // assign_section takes the pipe's recordText; a section field takes its stored number.
    let pipe = await read(model, "pipe_segments", "pipe:RS-P010");
    model = await applied(model, op("modify", "Element", "pipe:RS-P010", "assign_section", "section_ref", pipe.recordText, json({ section_ref: "section:agent-S270" })));
    pipe = await read(model, "pipe_segments", "pipe:RS-P010");
    const allowance = (pipe.record.section as AnyRecord).corrosion_allowance as { value: number; unit: string };
    model = await applied(model, op("modify", "Element", "pipe:RS-P010", "set_field", "section.corrosion_allowance.value", String(allowance.value), json({ value: allowance.value + 0.5, unit: allowance.unit }), allowance.unit, "length"));
    model = await applied(model, op("modify", "Element", "pipe:RS-P010", "set_field", "material", String(pipe.record.material), "material:RS-M1"));
    model = await applied(model, op("modify", "Element", "pipe:RS-P010", "assign_section", "section_ref", (await read(model, "pipe_segments", "pipe:RS-P010")).recordText, json({ section_ref: "section:RS-S270" })));
    model = await applied(model, op("delete", "Section", "section:agent-S270", "delete_section", "sections", (await read(model, "sections", "section:agent-S270")).before.delete, "deleted"));
    // Component geometry: a quantity and a text member; the modifiers stay byte for byte.
    const modifiersBefore = await canonicalJsonString(model.components.map((c) => (c as unknown as AnyRecord).modifiers ?? null));
    const bend = await read(model, "components", "component:RS-B1");
    const radius = (bend.record.geometry as AnyRecord).bend_radius as { value: number; unit: string };
    model = await applied(model, op("modify", "Component", "component:RS-B1", "set_field", "geometry.bend_radius.value", String(radius.value), json(radius), radius.unit, "length"));
    const tee = await read(model, "components", "component:RS-T1");
    model = await applied(model, op("modify", "Component", "component:RS-T1", "set_field", "geometry.branch_connection_type", String((tee.record.geometry as AnyRecord).branch_connection_type), "invented agent-stated connection type"));
    expect(await canonicalJsonString(model.components.map((c) => (c as unknown as AnyRecord).modifiers ?? null))).toBe(modifiersBefore);
    // Reference configuration, from the project record.
    const project = await read(model, "project", model.project.id);
    const configurations = JSON.parse(project.before.reference_configurations) as AnyRecord[];
    model = await applied(model, op("modify", "Model", model.project.id, "set_field", "reference_configurations", project.before.reference_configurations, json([{ ...configurations[0], provenance: P }])));
    // A load case's analysis state and pressure regions.
    let sus = await read(model, "load_cases", "load:RS-SUS");
    const state = JSON.parse(sus.before.analysis_state) as AnyRecord;
    model = await applied(model, op("modify", "Load", "load:RS-SUS", "update_load", "analysis_state", sus.before.analysis_state, json({ ...state, provenance: P })));
    sus = await read(model, "load_cases", "load:RS-SUS");
    const regions = JSON.parse(sus.before.pressure_regions) as AnyRecord[];
    model = await applied(model, op("modify", "Load", "load:RS-SUS", "update_load", "pressure_regions", sus.before.pressure_regions, json([{ ...regions[0], provenance: P }, ...regions.slice(1)])));
    // A case of the agent's own, then its deletion by the engine's deletion text.
    model = await applied(model, op("create", "Load", "load:agent-check", "create_load_case", "load_cases", "not_present", json({ id: "load:agent-check", label: "  agent check ", kind: "primitive_user_load", status: "draft", provenance: P })));
    const check = await read(model, "load_cases", "load:agent-check");
    expect(check.before.delete).toBe("load:agent-check; agent check; primitive_user_load; draft; primitives=0");
    model = await applied(model, op("delete", "Load", "load:agent-check", "delete_load_case", "load_cases", check.before.delete, "not_present"));
    // Combinations: a subtraction by its canonical JSON, a factored one by the engine's summary.
    model = await applied(model, op("delete", "Combination", "combination:RS-dSAM", "delete_combination", "combinations", (await read(model, "combinations", "combination:RS-dSAM")).before.delete, "not_present"));
    model = await applied(model, op("create", "Combination", "combination:agent-M", "create_combination", "combinations", "not_present", json({ id: "combination:agent-M", label: "agent M", basis: "mechanics", terms: [{ load_case: "load:RS-OPE", factor: 1 }], provenance: P })));
    model = await applied(model, op("create", "Combination", "combination:agent-M", "create_combination_term", "terms", "not_present", json({ load_case: "load:RS-SUS", factor: 1.25e-7 })));
    const factored = await read(model, "combinations", "combination:agent-M");
    expect(factored.before.delete).toBe("combination:agent-M; agent M; basis=mechanics; terms=load:RS-OPE x 1; load:RS-SUS x 0.000000125");
    model = await applied(model, op("delete", "Combination", "combination:agent-M", "delete_combination", "combinations", factored.before.delete, "not_present"));
    expect((model.combinations ?? []).map((c) => c.id)).toEqual(["combination:RS-EXP"]);
  }, 120_000);

  it("pressure-free route: settings, condition sets, rule-pack references, wind exposure and a support's provenance record", async () => {
    let model = structuredClone(demoJson) as unknown as PreviewModel;
    const id = model.project.id;
    // Model settings, from the project record.
    let project = await read(model, "project", id);
    model = await applied(model, op("modify", "Project", id, "set_model_setting", "model_settings.node_proximity_tolerance", project.before["model_settings.node_proximity_tolerance"], "5", "mm", "length"));
    project = await read(model, "project", id);
    expect(project.before["model_settings.node_proximity_tolerance"]).toBe('{"unit":"mm","value":5}');
    model = await applied(model, op("modify", "Project", id, "set_model_setting", "model_settings.node_proximity_tolerance", project.before["model_settings.node_proximity_tolerance"], "6", "mm", "length"));
    model = await applied(model, op("modify", "Project", id, "set_model_setting", "model_settings.shear_model", project.before["model_settings.shear_model"], "euler_bernoulli"));
    // Project units take the summary's stored symbol.
    model = await applied(model, op("modify", "Project", id, "set_project_unit", "project.units.force", model.project.units.force!, "kN", "kN", "force"));
    // A condition set: created, replaced by its recordText, assigned and cleared, deleted.
    const set = { id: "cs:agent", label: "agent set", provenance: P, installation_temperature: { value: 20, unit: "degC" }, conditions: [{ key: "OPE1", temperature: { value: 150, unit: "degC" } }], exposure: { wind: false } };
    model = await applied(model, op("create", "ConditionSet", "cs:agent", "create_condition_set", "condition_sets", "not_present", json(set)));
    model = await applied(model, op("modify", "ConditionSet", "cs:agent", "update_condition_set", "condition_sets", (await read(model, "condition_sets", "cs:agent")).recordText, json({ ...set, label: "agent set, replaced" })));
    const pipeId = model.pipe_segments[0].id;
    model = await applied(model, op("modify", "Element", pipeId, "assign_condition_set", "condition_set_ref", (await read(model, "pipe_segments", pipeId)).before.condition_set_ref, json({ condition_set_ref: "cs:agent" })));
    const assigned = await read(model, "pipe_segments", pipeId);
    expect(assigned.before.condition_set_ref).toBe('{"condition_set_ref":"cs:agent"}');
    model = await applied(model, op("modify", "Element", pipeId, "assign_condition_set", "condition_set_ref", assigned.before.condition_set_ref, "not_present"));
    model = await applied(model, op("delete", "ConditionSet", "cs:agent", "delete_condition_set", "condition_sets", (await read(model, "condition_sets", "cs:agent")).before.delete, "not_present"));
    // A case's condition use, by its before text.
    model = await applied(model, op("create", "ConditionSet", "cs:agent", "create_condition_set", "condition_sets", "not_present", json(set)));
    const caseId = model.load_cases[0].id;
    model = await applied(model, op("modify", "Load", caseId, "set_load_case_condition_use", "condition_use", (await read(model, "load_cases", caseId)).before.condition_use, json({ temperature: "OPE1" })));
    expect((await read(model, "load_cases", caseId)).before.condition_use).toBe('{"temperature":"OPE1"}');
    // A rule-pack reference, keyed by its pack id.
    const pack = { rule_pack_id: "invented_private_pack", name: "Invented pack", rule_pack_version: "0.1.0",
      checksum: { algorithm: "sha256", canonicalization: "JCS", payload_scope: "rule_pack_payload", payload_excludes: ["checksums"], value: "a".repeat(64) }, redistribution_status: "private_only" };
    model = await applied(model, op("create", "RulePackRef", "invented_private_pack", "add_rule_pack_ref", "rule_pack_refs", "not_present", json(pack)));
    model = await applied(model, op("delete", "RulePackRef", "invented_private_pack", "remove_rule_pack_ref", "rule_pack_refs", (await read(model, "rule_pack_refs", "invented_private_pack")).before.delete, "not_present"));
    // Wind exposure: present once the case has wind inputs.
    const windCase = model.load_cases[1].id;
    expect((await read(model, "load_cases", windCase)).before["equivalent_static.wind.exposure"]).toBeUndefined();
    model = await applied(model, op("modify", "Load", windCase, "set_field", "equivalent_static.wind.direction", "TBD", "global_y"));
    const wind = await read(model, "load_cases", windCase);
    expect(wind.before["equivalent_static.wind.exposure"]).toBe('{"exposed_pipe_refs":[],"exposed_spans":[]}');
    model = await applied(model, op("modify", "Load", windCase, "update_load", "equivalent_static.wind.exposure", wind.before["equivalent_static.wind.exposure"], json({ exposed_pipe_refs: [pipeId], exposed_spans: [] })));
    // A support's provenance record, as the hanger selection writes it.
    const supportId = model.supports[0].id;
    const record = { source_name: "Invented catalogue", redistribution_status: "private_only", review_status: "pending" };
    model = await applied(model, op("modify", "Support", supportId, "update_support", "provenance_record", (await read(model, "supports", supportId)).before.provenance_record, json(record)));
    model = await applied(model, op("modify", "Support", supportId, "update_support", "provenance_record", (await read(model, "supports", supportId)).before.provenance_record, json({ ...record, review_status: "quarantined" })));
    expect((find(model, "supports", supportId).provenance_record as AnyRecord).review_status).toBe("quarantined");
  }, 120_000);
});

describe("the person-entered members the agent's proposals are refused for", () => {
  // The engine's refusal of agent-written person-entered values is the two expressions
  // `writes_flexibility` and `writes_modifier` in the operation applier. Their literals are
  // pinned here: a change to either fails this test, so operationAuthorship.ts
  // writesUserEnteredModifier, the one list the agent's executor uses, is reviewed with it.
  const source = readFileSync(resolve(__dirname, "../../../../core/model_operations/operation_applier/src/lib.rs"), "utf8");
  const expression = (name: string) => {
    const start = source.indexOf(`let ${name} =`);
    expect(start, name).toBeGreaterThan(0);
    return source.slice(start, source.indexOf(";", start));
  };
  const literals = (text: string) => [...text.matchAll(/"([^"]*)"/g)].map((m) => m[1]);
  const op = (kind: string, object: string, path: string, after: unknown) =>
    ({ operation_id: "op", target: { object_type: object, ref: "r" }, change: { change_kind: kind, field_path: path, after: JSON.stringify(after) } });

  it("covers every route and member the engine's refusal names", () => {
    expect(literals(expression("writes_flexibility"))).toEqual(["insert_bend_at_corner", "Component", "bend_realization", "modifiers.flexibility_factor_user_value.value"]);
    expect(literals(expression("writes_modifier"))).toEqual(["Component", "modifiers.", "insert_component_symbol", "modifiers"]);
    expect(writesUserEnteredModifier(op("insert_bend_at_corner", "Node", "bend_at_corner", {}))?.route).toBe("refused");
    expect(writesUserEnteredModifier(op("set_field", "Component", "bend_realization", {}))?.route).toBe("refused");
    expect(writesUserEnteredModifier(op("set_field", "Component", "modifiers.flexibility_factor_user_value.value", {}))?.route).toBe("refused");
    expect(writesUserEnteredModifier(op("insert_component_symbol", "Component", "components", { modifiers: {} }))?.route).toBe("refused");
    expect(writesUserEnteredModifier(op("apply_library_record", "Component", "components", { "modifiers.sif_user_value": {} }))?.route).toBe("refused");
  });

  it("names a member the engine drops at any depth, with its path, and refuses past the walk's bound", () => {
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", { modifiers: {} }))).toMatchObject({ member: "modifiers", route: "dropped", path: "modifiers" });
    expect(writesUserEnteredModifier(op("create_node", "Node", "nodes", { bend_realization: null }))).toMatchObject({ member: "bend_realization", route: "dropped" });
    expect(writesUserEnteredModifier(op("insert_component_symbol", "Component", "components", { mechanics_interface: { solver_consumption: "curved_bend_macro_element" } }))).toMatchObject({ member: "mechanics_interface", route: "dropped" });
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", { geometry: { modifiers: {} } }))).toMatchObject({ member: "modifiers", route: "dropped", path: "geometry.modifiers" });
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", [{ modifiers: {} }]))).toMatchObject({ member: "modifiers", path: "[0].modifiers" });
    expect(writesUserEnteredModifier(op("insert_component_symbol", "Component", "components", { geometry: { modifiers: {} } }))).toMatchObject({ route: "dropped", path: "geometry.modifiers" });
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", { "odd key": { modifiers: 1 } }))?.path).toBe('["odd key"].modifiers');
    for (const after of [{ mechanics_interface: { solver_consumption: "mechanics_geometry_only" } }, [1, { id: "x" }], "TBD"])
      expect(writesUserEnteredModifier(op("create_material", "Material", "materials", after)), JSON.stringify(after)).toBeNull();
    expect(writesUserEnteredModifier({ change: { change_kind: "create_material", after: "{" } })).toBeNull();
    let deep: unknown = { leaf: 1 };
    for (let i = 0; i < 33; i++) deep = [deep];
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", deep))).toMatchObject({ route: "unchecked" });
    let fits: unknown = { leaf: 1 };
    for (let i = 0; i < 31; i++) fits = [fits];
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", fits))).toBeNull();
    expect(writesUserEnteredModifier(op("create_material", "Material", "materials", Array.from({ length: 50_001 }, () => 0)))).toMatchObject({ route: "unchecked" });
  });
});

describe("a person-entered member nested in a record the engine rebuilds is dropped by the engine, so the shared refusal is needed", () => {
  // Engine-backed: each operation is stamped as the agent's and applied on the wasm engine with
  // no gate in front. Each creator rebuilds the nested record from its named members, so the
  // member is written nowhere, while the review row repeats the proposed text. The shared
  // function names each with its path; the three entries refuse on it (their own tests).
  const sentinel = { sif_user_value: { value: 2.4, unit: "1" } };
  const count = (model: unknown) => (JSON.stringify(model).match(/"sif_user_value"/g) ?? []).length;
  async function engineDrops(model: PreviewModel, operation: AnyRecord, path: string) {
    expect(writesUserEnteredModifier(operation)).toMatchObject({ member: "modifiers", route: "dropped", path });
    const outcome = await run(model, operation);
    expect(outcome.validation.batch_validation_status, `${path}: ${codes(outcome).join(" | ")}`).toBe("passed");
    expect(count(outcome.applied_model), path).toBe(count(model));
    return outcome.applied_model!;
  }
  it("the reviewer's combination term, a node's position and a quantity edit", async () => {
    const model = structuredClone(referenceJson) as unknown as PreviewModel;
    const review = '{"id":"combination:review-nested","label":"Review example","basis":"mechanics","terms":[{"load_case":"load:RS-SUS","factor":1,"modifiers":{"sif_user_value":{"value":2.4,"unit":"1"}}}],"provenance":"invented review example"}';
    const combined = await engineDrops(model, op("create", "Combination", "combination:review-nested", "create_combination", "combinations", "not_present", review), "terms[0].modifiers");
    expect(find(combined, "combinations", "combination:review-nested").terms).toEqual([{ load_case: "load:RS-SUS", factor: 1 }]);
    const node = await engineDrops(model, op("create", "Node", "node:review-n", "create_node", "nodes", "not_present",
      json({ id: "node:review-n", label: "review", position: { x: 0, y: 0, z: 9, modifiers: sentinel }, provenance: P }), "m", "length"), "position.modifiers");
    expect(find(node, "nodes", "node:review-n").position).toEqual({ x: 0, y: 0, z: 9 });
    const modulus = find(model, "materials", "material:RS-M1").elastic_modulus as { value: number; unit: string };
    const edited = await engineDrops(model, op("modify", "Material", "material:RS-M1", "set_field", "elastic_modulus.value", String(modulus.value),
      json({ value: 201, unit: modulus.unit, modifiers: sentinel }), modulus.unit, "stress"), "modifiers");
    expect(find(edited, "materials", "material:RS-M1").elastic_modulus).toEqual({ value: 201, unit: modulus.unit });
  }, 60_000);
  it("a component's geometry", async () => {
    let model = structuredClone(m2.blank.model) as unknown as PreviewModel;
    const [geometry, components] = m2.proposals as { operations: AnyRecord[] }[];
    for (const operation of geometry.operations) model = (await run(model, operation)).applied_model!;
    const bend = structuredClone(components.operations[0]);
    const change = bend.change as AnyRecord;
    const after = JSON.parse(String(change.after)) as AnyRecord;
    change.after = json({ ...after, geometry: { ...(after.geometry as AnyRecord), modifiers: sentinel } });
    const applied = await engineDrops(model, bend, "geometry.modifiers");
    expect((find(applied, "components", String(after.id)).geometry as AnyRecord).modifiers).toBeUndefined();
  }, 60_000);
});
