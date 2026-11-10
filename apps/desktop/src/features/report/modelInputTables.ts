import type { ReportComposition } from "./reportComposition";

/** The model-input sections (RT-R4 slice C5; DEL-08-01 Design `report-section-composer.md`
 * §2.7), in catalogue order. */
export const MODEL_INPUT_SECTION_IDS = [
  "load_combinations", "input_geometry", "input_materials_sections", "input_supports", "input_loads"
] as const;
export type ModelInputSectionId = typeof MODEL_INPUT_SECTION_IDS[number];

export function isModelInputSectionId(id: string): id is ModelInputSectionId {
  return (MODEL_INPUT_SECTION_IDS as readonly string[]).includes(id);
}

export const MODEL_INPUT_UNBOUND = "REPORT-COMPOSITION-MODEL-INPUT-UNBOUND";

/** A cell: a stored string, a number's `String(value)`, `"<number> <unit>"`, or null. */
export type ModelInputCell = string | null;
type Row<K extends string> = Record<K, ModelInputCell>;
export type ModelInputTables = {
  load_combinations?: Row<"id" | "label" | "basis" | "operands" | "mode">[];
  input_geometry?: {
    nodes: Row<"id" | "label" | "x" | "y" | "z">[];
    pipes: Row<"id" | "label" | "from" | "to" | "material" | "outside_diameter" | "wall_thickness" | "section_ref">[];
    components: Row<"id" | "label" | "kind">[];
  };
  input_materials_sections?: {
    materials: Row<"id" | "label" | "elastic_modulus" | "shear_modulus" | "poisson_ratio" | "thermal_expansion_coefficient">[];
    sections: Row<"id" | "name" | "section_type" | "outside_diameter" | "wall_thickness">[];
  };
  input_supports?: Row<"id" | "label" | "node" | "family" | "restraints" | "stiffness" | "nonlinear_behavior">[];
  input_loads?: Row<"case_id" | "load_id" | "category" | "target" | "direction" | "magnitude">[];
};

type Json = unknown;
type JsonObject = Record<string, Json>;

function isObject(value: Json): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function unprojectable(pointer: string, what: string): Error {
  return new Error(`${MODEL_INPUT_UNBOUND}: the model's ${pointer} is not ${what}, so its model-input table cannot be projected`);
}

function cell(value: Json, pointer: string): ModelInputCell {
  if (value === undefined || value === null) return null;
  if (typeof value === "string") return value;
  if (typeof value === "number") return String(value);
  if (isObject(value) && typeof value.value === "number" && typeof value.unit === "string") return `${String(value.value)} ${value.unit}`;
  throw unprojectable(pointer, "a stored string, number, quantity or null");
}

function member(object: JsonObject, pointer: string, key: string): ModelInputCell {
  return cell(object[key], `${pointer}/${key}`);
}

function optionalObject(value: Json, pointer: string): JsonObject | null {
  if (value === undefined || value === null) return null;
  if (isObject(value)) return value;
  throw unprojectable(pointer, "an object");
}

function objects(value: Json, pointer: string): [string, JsonObject][] {
  if (value === undefined || value === null) return [];
  if (!Array.isArray(value)) throw unprojectable(pointer, "a list");
  return value.map((item, index) => {
    const at = `${pointer}/${index}`;
    if (!isObject(item)) throw unprojectable(at, "an object");
    return [at, item];
  });
}

function joined(object: JsonObject, pointer: string, key: string, separator: string): ModelInputCell {
  const at = `${pointer}/${key}`;
  const value = object[key];
  if (value === undefined || value === null) return null;
  if (!Array.isArray(value)) throw unprojectable(at, "a list");
  return value.map((item, index) => {
    if (typeof item !== "string") throw unprojectable(`${at}/${index}`, "a string");
    return item;
  }).join(separator);
}

/** §2.7, by the combination's basis: `result_state_subtraction` as `"<minuend_id> − <subtrahend_id>"`
 * (null unless both are stored); `range_envelope` as `operand_ids` joined by `", "`; any other
 * basis as each term `"<factor> × <load_case or combination>"` joined by `"; "` (null when no
 * `terms` is stored). A stored member the basis does not use is not shown. */
function operands(combination: JsonObject, pointer: string): ModelInputCell {
  const basis = combination.basis;
  if (basis === "result_state_subtraction") {
    const minuend = member(combination, pointer, "minuend_id");
    const subtrahend = member(combination, pointer, "subtrahend_id");
    return minuend !== null && subtrahend !== null ? `${minuend} − ${subtrahend}` : null;
  }
  if (basis === "range_envelope") return joined(combination, pointer, "operand_ids", ", ");
  const terms = combination.terms;
  if (terms === undefined || terms === null) return null;
  if (!Array.isArray(terms)) throw unprojectable(`${pointer}/terms`, "a list");
  return terms.map((term: Json, index) => {
    const at = `${pointer}/terms/${index}`;
    const factor = isObject(term) ? term.factor : undefined;
    if (typeof factor !== "number") throw unprojectable(`${at}/factor`, "a number");
    const { load_case: loadCase, combination: other } = term as JsonObject;
    const operand = typeof loadCase === "string" ? loadCase
      : (loadCase === undefined || loadCase === null) && typeof other === "string" ? other : null;
    if (operand === null) throw unprojectable(at, "a term naming a load case or a combination");
    return `${String(factor)} × ${operand}`;
  }).join("; ");
}

/** A load's target: `"<type>:<node, pipe or support>"`, followed by `" <dof>"` when the target
 * names one; null when it names none of the three. */
function target(load: JsonObject, pointer: string): ModelInputCell {
  const at = `${pointer}/target`;
  const value = optionalObject(load.target, at);
  if (!value) return null;
  const kind = member(value, at, "type");
  let named: ModelInputCell = null;
  for (const key of ["node", "pipe", "support"]) {
    named = member(value, at, key);
    if (named !== null) break;
  }
  const dof = member(value, at, "dof");
  return kind !== null && named !== null ? `${kind}:${named}${dof !== null ? ` ${dof}` : ""}` : null;
}

function nested(object: JsonObject, at: string, key: string) {
  const inner = optionalObject(object[key], `${at}/${key}`);
  return (name: string) => inner ? member(inner, `${at}/${key}`, name) : null;
}

const PROJECTIONS: { [K in ModelInputSectionId]: (model: JsonObject) => NonNullable<ModelInputTables[K]> } = {
  load_combinations: (model) => objects(model.combinations, "/combinations").map(([at, item]) => ({
    id: member(item, at, "id"), label: member(item, at, "label"), basis: member(item, at, "basis"),
    operands: operands(item, at), mode: member(item, at, "mode")
  })),
  input_geometry: (model) => ({
    nodes: objects(model.nodes, "/nodes").map(([at, item]) => {
      const position = nested(item, at, "position");
      return { id: member(item, at, "id"), label: member(item, at, "label"), x: position("x"), y: position("y"), z: position("z") };
    }),
    pipes: objects(model.pipe_segments, "/pipe_segments").map(([at, item]) => {
      const section = nested(item, at, "section");
      return {
        id: member(item, at, "id"), label: member(item, at, "label"), from: member(item, at, "from"), to: member(item, at, "to"),
        material: member(item, at, "material"), outside_diameter: section("outside_diameter"), wall_thickness: section("wall_thickness"),
        section_ref: member(item, at, "section_ref")
      };
    }),
    components: objects(model.components, "/components").map(([at, item]) => ({
      id: member(item, at, "id"), label: member(item, at, "label"), kind: member(item, at, "kind")
    }))
  }),
  input_materials_sections: (model) => ({
    materials: objects(model.materials, "/materials").map(([at, item]) => ({
      id: member(item, at, "id"), label: member(item, at, "label"), elastic_modulus: member(item, at, "elastic_modulus"),
      shear_modulus: member(item, at, "shear_modulus"), poisson_ratio: member(item, at, "poisson_ratio"),
      thermal_expansion_coefficient: member(item, at, "thermal_expansion_coefficient")
    })),
    sections: objects(model.sections, "/sections").map(([at, item]) => {
      const properties = nested(item, at, "properties");
      return {
        id: member(item, at, "id"), name: member(item, at, "name"), section_type: member(item, at, "section_type"),
        outside_diameter: properties("outside_diameter"), wall_thickness: properties("wall_thickness")
      };
    })
  }),
  input_supports: (model) => objects(model.supports, "/supports").map(([at, item]) => {
    const stiffnessAt = `${at}/stiffness`;
    const stiffnessValue = optionalObject(item.stiffness, stiffnessAt);
    let stiffness: ModelInputCell = null;
    if (stiffnessValue) {
      const quantity = member(stiffnessValue, stiffnessAt, "value");
      if (typeof stiffnessValue.dof !== "string" || quantity === null) throw unprojectable(stiffnessAt, "a stiffness with a degree of freedom and a value");
      stiffness = `${stiffnessValue.dof}: ${quantity}`;
    }
    const nonlinear = optionalObject(item.nonlinear, `${at}/nonlinear`);
    return {
      id: member(item, at, "id"), label: member(item, at, "label"), node: member(item, at, "node"), family: member(item, at, "family"),
      restraints: joined(item, at, "restraints", ", "), stiffness,
      nonlinear_behavior: nonlinear ? member(nonlinear, `${at}/nonlinear`, "behavior") : null
    };
  }),
  input_loads: (model) => objects(model.load_cases, "/load_cases").flatMap(([caseAt, loadCase]) => {
    const caseId = member(loadCase, caseAt, "id");
    return objects(loadCase.primitive_loads, `${caseAt}/primitive_loads`).map(([at, load]) => ({
      case_id: caseId, load_id: member(load, at, "id"), category: member(load, at, "category"), target: target(load, at),
      direction: member(load, at, "direction"), magnitude: member(load, at, "magnitude")
    }));
  })
};

/** The model-input ids `composition` lists, in catalogue order. */
export function listedModelInputIds(composition: ReportComposition | null | undefined): ModelInputSectionId[] {
  const listed = new Set((composition?.sections ?? []).map((section) => section.section_id as string));
  return MODEL_INPUT_SECTION_IDS.filter((id) => listed.has(id));
}

/** `P(M, C)` (§2.7): null when the composition lists no model-input id; otherwise one member
 * per listed id, each a list of rows in the model's array order. The Rust mirror is
 * `report_package::model_input::model_input_tables`, held equal by
 * `fixtures/reports/invented/report_model_input_vectors.json`. */
export function modelInputTables(model: unknown, composition: ReportComposition | null | undefined): ModelInputTables | null {
  const ids = listedModelInputIds(composition);
  if (ids.length === 0) return null;
  if (!isObject(model)) throw unprojectable("root", "an object");
  const tables: Record<string, unknown> = {};
  for (const id of ids) tables[id] = PROJECTIONS[id](model);
  return tables as ModelInputTables;
}
