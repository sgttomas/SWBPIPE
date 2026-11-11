// W-12 S3a (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.1, §6):
// the state comparison's flattening (complete and fail-closed against the 0.9.0
// schema), the equal-hash invariant (including a reorder-only pair),
// classification by stable id, the duplicate-id refusal and the desktop's own
// field diff (nested paths, `unit_changed`). Parity with the reference engine is
// `stateComparisonParity.test.ts` and `tests/test_model_state_comparison_parity.py`.
// All data are invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { computeModelHash } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import {
  COLLECTION_MEMBERS,
  COMPARISON_DUPLICATE_ID,
  COMPARISON_ENTITY_ID_MISSING,
  COMPARISON_MEMBER_MALFORMED,
  COMPARISON_MEMBER_UNASSIGNED,
  compareFlattened,
  compareModelDocuments,
  flattenModelDocument,
  MEMBER_ASSIGNMENT,
  MODEL_ENTITY_MEMBERS,
  pathText,
  StateComparisonRefusal,
  type StateComparison
} from "./stateComparison";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

// Vitest runs with the desktop package as its working directory.
const SCHEMA_PATH = path.join(process.cwd(), "..", "..", "schemas", "model.schema.yaml");

/** The every-slot fixture, with the two 0.9.0 slots it lacks (pressure_contract,
 * reference_configurations) added so every family is exercised. */
function everySlot(): Json {
  const model = structuredClone(everySlotModel) as Json;
  model.pressure_contract = { version: "invented-1", mode: "invented_mode", shear_model: "invented_shear" };
  model.reference_configurations = [{ id: "refcfg:R-1", label: "Invented reference", geometry_ref: "geometry:invented", member_references: [], provenance: "invented_example" }];
  return model;
}

function refusal(run: () => unknown): StateComparisonRefusal {
  try {
    run();
  } catch (error) {
    if (error instanceof StateComparisonRefusal) return error;
    throw error;
  }
  throw new Error("expected a named refusal");
}

function differences(comparison: StateComparison) {
  return comparison.rows.filter((row) => row.classification !== "unchanged");
}

describe("flattening (complete and fail-closed)", () => {
  it("preserves 0.9.0 condition sets and spectra as authored model fields and reports their changes", () => {
    const left = everySlot();
    const before = JSON.stringify(left);
    const flat = flattenModelDocument(left).find((entity) => entity.family === "model")!;
    expect(flat.fields.condition_sets).toEqual(left.condition_sets);
    expect(flat.fields.spectra).toEqual(left.spectra);
    const right = structuredClone(left);
    right.condition_sets[0].conditions[0].temperature.value += 1;
    right.spectra[0].points[0][1] += 0.1;
    const changed = differences(compareModelDocuments(left, right));
    expect(changed).toHaveLength(1);
    expect(changed[0].family).toBe("model");
    expect(new Set(changed[0].changes.map((change) => change.path[0]))).toEqual(new Set(["condition_sets", "spectra"]));
    expect(JSON.stringify(left)).toBe(before);
  });

  it("assigns every top-level member of the 0.9.0 schema, and nothing else", () => {
    const schema = JSON.parse(readFileSync(SCHEMA_PATH, "utf8")) as { properties: Record<string, unknown>; additionalProperties: boolean };
    // The document is closed, so the schema's properties are every member it can hold.
    expect(schema.additionalProperties).toBe(false);
    // A member added to the schema without a family fails here.
    expect(Object.keys(MEMBER_ASSIGNMENT).sort()).toEqual(Object.keys(schema.properties).sort());
    expect(Object.keys(MEMBER_ASSIGNMENT)).toHaveLength(MODEL_ENTITY_MEMBERS.length + Object.keys(COLLECTION_MEMBERS).length);
  });

  it("refuses a document holding a member no family holds, naming it", () => {
    const model = everySlot();
    model.invented_future_member = [{ id: "x" }];
    const refused = refusal(() => flattenModelDocument(model, "left"));
    expect(refused.code).toBe(COMPARISON_MEMBER_UNASSIGNED);
    expect(refused.message).toMatch(/^COMPARISON-MEMBER-UNASSIGNED: the left model holds member `invented_future_member` that no comparison family holds/);
    expect(refusal(() => compareModelDocuments(everySlot(), model)).code).toBe(COMPARISON_MEMBER_UNASSIGNED);
  });

  it("refuses a collection that is not an array of objects, and an entity without a stable id", () => {
    const notArray = everySlot();
    notArray.nodes = { id: "node:N-1" };
    expect(refusal(() => flattenModelDocument(notArray)).code).toBe(COMPARISON_MEMBER_MALFORMED);
    const notObject = everySlot();
    notObject.lines.push("line:L-300");
    expect(refusal(() => flattenModelDocument(notObject)).code).toBe(COMPARISON_MEMBER_MALFORMED);
    for (const [member, mutate] of [
      ["nodes", (model: Json) => { delete model.nodes[0].id; }],
      ["unresolved_assumptions", (model: Json) => { model.unresolved_assumptions[0].assumption_id = "  "; }],
      ["load_cases", (model: Json) => { model.load_cases[0].primitive_loads[0].id = 7; }]
    ] as const) {
      const model = everySlot();
      mutate(model);
      const refused = refusal(() => flattenModelDocument(model));
      expect(refused.code, member).toBe(COMPARISON_ENTITY_ID_MISSING);
    }
  });

  it("gives one model entity, one entity per collection member, and primitive loads keyed by their load case", () => {
    const entities = flattenModelDocument(everySlot());
    expect(entities[0].family).toBe("model");
    expect(entities.filter((entity) => entity.family === "model")).toHaveLength(1);
    const model = entities[0].fields as Json;
    for (const member of ["schema_version", "document_kind", "project", "pressure_contract", "data_boundary", "analysis_status", "diagnostics", "model_settings"]) {
      expect(model, member).toHaveProperty([member]);
    }
    expect(model.collection_order.nodes).toEqual(["node:N-1", "node:N-2", "node:N-3"]);
    expect(model.primitive_load_order).toEqual([
      { load_case: "load:L-1", loads: ["load:L-1-Z"] },
      { load_case: "load:L-2", loads: [] },
      { load_case: "load:L-3", loads: [] }
    ]);
    const loadCase = entities.find((entity) => entity.family === "load_case")!;
    expect(loadCase.fields).not.toHaveProperty("primitive_loads");
    expect(loadCase.fields).not.toHaveProperty("id");
    const load = entities.find((entity) => entity.family === "primitive_load")!;
    expect(load).toMatchObject({ key: ["load:L-1", "load:L-1-Z"], id: "load:L-1/load:L-1-Z" });
    expect(load.fields).not.toHaveProperty("id");
    // Each family keys by its own stable id field.
    expect(entities.find((entity) => entity.family === "unresolved_assumption")!.id).toBe("assumption:A-001");
    expect(entities.find((entity) => entity.family === "constraint")!.id).toBe("constraint:K-001");
    expect(entities.find((entity) => entity.family === "rule_pack_ref")!.id).toBe("invented_demo_rule_pack");
    expect(entities.find((entity) => entity.family === "reference_configuration")!.id).toBe("refcfg:R-1");
  });
});

describe("equal model hashes hold exactly when no row is other than unchanged", () => {
  const mutations: Array<[string, (model: Json) => void]> = [
    ["nodes reordered (only the order)", (model) => { model.nodes.reverse(); }],
    ["lines reordered", (model) => { model.lines.reverse(); }],
    ["primitive loads reordered within their case", (model) => {
      model.load_cases[0].primitive_loads.push({ ...structuredClone(model.load_cases[0].primitive_loads[0]), id: "load:L-1-Y" });
      model.load_cases[0].primitive_loads.reverse();
    }],
    ["a node moved", (model) => { model.nodes[1].position = { ...model.nodes[1].position, x: 99 }; }],
    ["a node's label set to null", (model) => { model.nodes[0].label = null; }],
    ["a member left out", (model) => { delete model.model_settings; }],
    ["a collection emptied", (model) => { model.components = []; }],
    ["primitive_loads emptied", (model) => { model.load_cases[0].primitive_loads = []; }],
    ["primitive_loads left out", (model) => { delete model.load_cases[0].primitive_loads; }],
    ["a quantity's unit changed", (model) => { model.materials[0].elastic_modulus = { value: 200000, unit: "MPa" }; }],
    ["a nested provenance field changed", (model) => { model.materials[0].provenance_record.source_name = "Invented other handbook"; }],
    ["the project renamed", (model) => { model.project.name = "Invented renamed project"; }],
    ["a diagnostic added", (model) => { model.diagnostics.push({ code: "INVENTED", severity: "info", message: "Invented." }); }],
    ["a rule-pack reference's version changed", (model) => { model.rule_pack_refs[0].rule_pack_version = "9.9.9"; }],
    ["an assumption removed", (model) => { model.unresolved_assumptions.pop(); }],
    ["an empty object member", (model) => { model.model_settings = {}; }]
  ];

  it("an identical pair and a member-order-only pair hash equal and compare all unchanged", async () => {
    const left = everySlot();
    // Every object's members in reverse order, at every depth (arrays keep their order).
    const reversed = (value: Json): Json => Array.isArray(value)
      ? value.map(reversed)
      : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).reverse().map(([key, item]) => [key, reversed(item)])) : value;
    const keyOrder = reversed(left);
    expect(JSON.stringify(keyOrder)).not.toBe(JSON.stringify(left));
    for (const right of [structuredClone(left), keyOrder]) {
      const [a, b] = await Promise.all([computeModelHash(left as PreviewModel), computeModelHash(right as PreviewModel)]);
      expect(a!.value).toBe(b!.value);
      const comparison = compareModelDocuments(left, right);
      expect(differences(comparison)).toEqual([]);
      expect(comparison.summary.unchanged).toBe(comparison.summary.total);
    }
  });

  it.each(mutations)("%s: the hash differs and a row is not unchanged", async (_name, mutate) => {
    const left = everySlot();
    const right = everySlot();
    mutate(right);
    const [a, b] = await Promise.all([computeModelHash(left as PreviewModel), computeModelHash(right as PreviewModel)]);
    expect(a!.value).not.toBe(b!.value);
    expect(differences(compareModelDocuments(left, right)).length).toBeGreaterThan(0);
  });

  it("an empty collection added to a document without it: the hash differs and the model entity changes", async () => {
    const left = everySlot();
    delete left.reference_configurations;
    const right = everySlot();
    right.reference_configurations = [];
    const [a, b] = await Promise.all([computeModelHash(left as PreviewModel), computeModelHash(right as PreviewModel)]);
    expect(a!.value).not.toBe(b!.value);
    const changed = differences(compareModelDocuments(left, right));
    expect(changed.map((row) => row.family)).toEqual(["model"]);
    expect(changed[0].changes).toEqual([{ path: ["collection_order", "reference_configurations"], kind: "added", right: [] }]);
  });

  it("a reorder-only pair changes only the model entity's collection order", () => {
    const left = everySlot();
    const right = everySlot();
    right.nodes.reverse();
    const changed = differences(compareModelDocuments(left, right));
    expect(changed.map((row) => [row.family, row.id, row.classification])).toEqual([["model", "model", "changed"]]);
    expect(changed[0].changes.map((change) => pathText(change.path))).toEqual(["collection_order.nodes"]);
    expect(changed[0].changes[0]).toMatchObject({ kind: "changed", left: ["node:N-1", "node:N-2", "node:N-3"], right: ["node:N-3", "node:N-2", "node:N-1"] });
  });
});

describe("classification by stable id within each family", () => {
  it("reports added, removed, changed and unchanged entities, primitive loads by `<load case>/<load>`", () => {
    const left = everySlot();
    const right = everySlot();
    right.nodes.push({ id: "node:N-4", label: "Invented added node", position: { x: 9, y: 0, z: 0, unit: "m" }, provenance: "invented_example" });
    right.lines = right.lines.filter((line: Json) => line.id !== "line:L-200");
    right.supports[0].label = "Invented relabelled support";
    right.load_cases[0].primitive_loads[0].magnitude = { value: -250, unit: "N" };
    const comparison = compareModelDocuments(left, right);
    const byId = new Map(comparison.rows.map((row) => [`${row.family}:${row.id}`, row]));
    expect(byId.get("node:node:N-4")!.classification).toBe("added");
    expect(byId.get("line:line:L-200")!.classification).toBe("removed");
    expect(byId.get("support:support:S-1")!.classification).toBe("changed");
    expect(byId.get("node:node:N-1")!.classification).toBe("unchanged");
    // The primitive load is its own entity; the load case excludes it, so stays unchanged.
    expect(byId.get("primitive_load:load:L-1/load:L-1-Z")!.classification).toBe("changed");
    expect(byId.get("load_case:load:L-1")!.classification).toBe("unchanged");
    // The model entity records the changed collection orders.
    expect(byId.get("model:model")!.changes.map((change) => pathText(change.path))).toEqual(["collection_order.lines", "collection_order.nodes"]);
    expect(comparison.summary).toMatchObject({ added: 1, removed: 1, changed: 3, total: comparison.rows.length });
    expect(comparison.summary.unchanged).toBe(comparison.rows.length - 5);
    // Rows are in family order, then id order.
    expect(comparison.rows[0].family).toBe("model");
    expect(comparison.rows.map((row) => row.family).indexOf("node")).toBeLessThan(comparison.rows.map((row) => row.family).indexOf("line"));
  });

  it("refuses a stable id repeated within one family on either side, naming it; equal ids in different families are distinct", () => {
    for (const side of ["left", "right"] as const) {
      const duplicated = everySlot();
      duplicated.nodes.push(structuredClone(duplicated.nodes[0]));
      const refused = refusal(() => side === "left" ? compareModelDocuments(duplicated, everySlot()) : compareModelDocuments(everySlot(), duplicated));
      expect(refused.code).toBe(COMPARISON_DUPLICATE_ID);
      expect(refused.message).toBe(`COMPARISON-DUPLICATE-ID: the ${side} model holds more than one node with stable id \`node:N-1\`, so its entities cannot be mapped.`);
    }
    const shared = everySlot();
    shared.lines.push({ id: "node:N-1", pipe_refs: [] });
    expect(() => compareModelDocuments(shared, shared)).not.toThrow();
    // A primitive load id may repeat across load cases: the key is the pair.
    const loads = everySlot();
    loads.load_cases.push({ id: "load:comparison-added", primitive_loads: [structuredClone(loads.load_cases[0].primitive_loads[0])] });
    const rows = compareModelDocuments(everySlot(), loads).rows.filter((row) => row.family === "primitive_load");
    expect(rows.map((row) => [row.id, row.classification])).toEqual([["load:L-1/load:L-1-Z", "unchanged"], ["load:comparison-added/load:L-1-Z", "added"]]);
  });
});

describe("the desktop field diff (extensions to the reference engine)", () => {
  it("skips only the entity's own id field and reports nested paths", () => {
    const left = everySlot();
    const right = everySlot();
    right.materials[0].provenance_record.library_ref.library_id = "library:invented-other";
    right.supports[0].restraints = [...right.supports[0].restraints].reverse();
    const changes = compareModelDocuments(left, right).rows.filter((row) => row.classification === "changed").flatMap((row) => row.changes.map((change) => `${row.family}:${pathText(change.path)}`));
    expect(changes).toEqual(["material:provenance_record.library_ref.library_id", "support:restraints"]);
    // Arrays holding objects are compared by index; a key that is not a plain name is quoted.
    const flatLeft = [{ family: "section" as const, key: ["s"], id: "s", fields: { properties: { "outer diameter": { value: 1, unit: "m" } }, points: [{ t: 1 }, { t: 2 }] } }];
    const flatRight = [{ family: "section" as const, key: ["s"], id: "s", fields: { properties: { "outer diameter": { value: 2, unit: "m" } }, points: [{ t: 1 }, { t: 3 }, { t: 4 }] } }];
    const row = compareFlattened(flatLeft, flatRight).rows[0];
    expect(row.changes.map((change) => [pathText(change.path), change.kind])).toEqual([
      ["points[1].t", "changed"],
      ["points[2].t", "added"],
      ['properties["outer diameter"]', "changed"]
    ]);
    expect(row.changes[2]).toMatchObject({ left: { value: 1, unit: "m" }, right: { value: 2, unit: "m" }, left_unit: "m", right_unit: "m" });
  });

  it("reports a quantity whose unit differs as unit_changed with both values and units, never converted", () => {
    const left = everySlot();
    const right = everySlot();
    right.materials[0].elastic_modulus = { value: 200000, unit: "MPa" };
    // A vector quantity: the fixture's positions carry no unit, so both sides get one.
    left.nodes[2].position = { ...left.nodes[2].position, unit: "m" };
    right.nodes[2].position = { ...right.nodes[2].position, unit: "mm" };
    const comparison = compareModelDocuments(left, right);
    const material = comparison.rows.find((row) => row.family === "material")!;
    expect(material.classification).toBe("changed");
    expect(material.changes).toEqual([{
      path: ["elastic_modulus"], kind: "unit_changed",
      left: left.materials[0].elastic_modulus, right: { value: 200000, unit: "MPa" }, left_unit: "Pa", right_unit: "MPa"
    }]);
    const node = comparison.rows.find((row) => row.id === "node:N-3")!;
    expect(node.changes).toMatchObject([{ path: ["position"], kind: "unit_changed", left_unit: "m", right_unit: "mm" }]);
    expect(comparison.summary.unit_changed_fields).toBe(2);
    // The same unit with another value is an ordinary change, still with its units.
    const value = everySlot();
    value.materials[0].elastic_modulus = { value: 1, unit: "Pa" };
    expect(compareModelDocuments(left, value).rows.find((row) => row.family === "material")!.changes).toEqual([{
      path: ["elastic_modulus"], kind: "changed", left: left.materials[0].elastic_modulus, right: { value: 1, unit: "Pa" }, left_unit: "Pa", right_unit: "Pa"
    }]);
  });

  it("tells a present null from an absent member, and an empty array from an empty object", () => {
    const left = [{ family: "node" as const, key: ["n"], id: "n", fields: { label: null, tags: [] } }];
    const right = [{ family: "node" as const, key: ["n"], id: "n", fields: { tags: {} } }];
    expect(compareFlattened(left, right).rows[0].changes.map((change) => [pathText(change.path), change.kind])).toEqual([
      ["label", "removed"],
      ["tags", "changed"]
    ]);
  });
});
