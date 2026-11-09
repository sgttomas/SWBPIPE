// RT-R4 slice C5 (DEL-08-01 Design `report-section-composer.md` §2.7): the model-input
// projection P(M, C), held equal across the layers. This test projects committed model
// fixtures with `modelInputTables` and writes
// `fixtures/reports/invented/report_model_input_vectors.json`, and fails when it is stale;
// `OPENPIPESTRESS_WRITE_MODEL_INPUT_VECTORS=<path>` writes it there instead. The Rust mirror
// (`core/reporting/report_package/src/model_input.rs`) projects the same models again in
// `core/reporting/report_package/tests/report_rows_parity.rs`.
//
// Invented models only; the tables carry the models' stored values and no result.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { MODEL_INPUT_SECTION_IDS, listedModelInputIds, modelInputTables } from "./modelInputTables";
import type { ReportComposition } from "./reportComposition";

const here = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(here, "../../../../..");
const VECTORS = path.join(projectRoot, "fixtures/reports/invented/report_model_input_vectors.json");
const WRITE_ENV = "OPENPIPESTRESS_WRITE_MODEL_INPUT_VECTORS";
const GENERATOR = "apps/desktop/src/features/report/modelInputTables.parity.test.ts";

/** Every model-input id, in an order other than the catalogue's (list order does not change P). */
const COMPOSITION: ReportComposition = {
  composition_version: "1.0.0",
  sections: [
    { section_id: "input_loads", cases: "all", view: { kind: "all" } },
    { section_id: "result_rows_all", cases: "all", view: { kind: "all" } },
    { section_id: "input_supports", cases: "all", view: { kind: "all" } },
    { section_id: "input_geometry", cases: "all", view: { kind: "all" } },
    { section_id: "load_combinations", cases: "all", view: { kind: "all" } },
    { section_id: "input_materials_sections", cases: "all", view: { kind: "all" } }
  ]
};

/** The committed models: their fixture and the JSON pointer of the model in it. */
const MODELS = [
  {
    name: "every_slot",
    reason: "components, shared sections, materials with every listed member, supports with stiffness and nonlinear behaviour; mechanics, result-state subtraction, absolute-sum and SRSS combinations",
    fixture: "fixtures/model_document/invented_every_slot_model.json",
    pointer: ""
  },
  {
    name: "preview_loads",
    reason: "primitive loads of weight, thermal, pressure and occasional categories on node and element targets",
    fixture: "fixtures/product_preview/invented_preview_model.json",
    pointer: ""
  },
  {
    name: "every_combination_basis",
    reason: "combinations of every basis: mechanics terms, result-state subtraction, range envelope, absolute sum and SRSS",
    fixture: "fixtures/results/skew_along_direction_combinations_preview.json",
    pointer: "/request/model"
  },
  {
    name: "support_target",
    reason: "a primitive load on a support target, shown as its type, support and DOF",
    fixture: "fixtures/results/combination_certificate_corpus.json",
    pointer: "/documents/3/model"
  }
] as const;

function at(value: unknown, pointer: string): unknown {
  return pointer.split("/").slice(1).reduce<unknown>((node, token) =>
    (node as Record<string, unknown>)[token.replace(/~1/g, "/").replace(/~0/g, "~")], value);
}

function vectorsDocument() {
  return {
    schema_version: "1.0.0",
    generator: GENERATOR,
    description:
      "Invented data. Model-input projection vectors (DEL-08-01 Design report-section-composer.md section 2.7, slice C5): for each committed model (model_fixture at model_pointer) and the composition, the expected model_input_tables P(M, C) the desktop builds (modelInputTables.ts). The Rust projection (report_package::model_input::model_input_tables) must give the same. The tables carry the models' stored values as entered and no result.",
    composition: COMPOSITION,
    vectors: MODELS.map(({ name, reason, fixture, pointer }) => {
      const model = at(JSON.parse(readFileSync(path.join(projectRoot, fixture), "utf8")), pointer);
      return { name, reason, model_fixture: fixture, model_pointer: pointer, expected: modelInputTables(model, COMPOSITION) };
    })
  };
}

describe("the model-input projection P(M, C) (§2.7)", () => {
  it("writes its vectors, report_model_input_vectors.json, and fails when they are stale", () => {
    const text = `${JSON.stringify(vectorsDocument(), null, 1)}\n`;
    const target = process.env[WRITE_ENV];
    if (target) {
      writeFileSync(target, text);
      return;
    }
    const committed = existsSync(VECTORS) ? readFileSync(VECTORS, "utf8") : "";
    expect(text === committed, `fixtures/reports/invented/report_model_input_vectors.json is stale; regenerate it with ${WRITE_ENV}=<path>`).toBe(true);
  });

  it("covers what the vectors are for", () => {
    const { vectors } = vectorsDocument();
    const tables = vectors.map((vector) => vector.expected!);
    const bases = new Set(tables.flatMap((table) => table.load_combinations!.map((row) => row.basis)));
    expect([...bases].sort()).toEqual(["absolute_sum", "mechanics", "range_envelope", "result_state_subtraction", "srss"]);
    expect(new Set(tables.flatMap((table) => table.input_loads!.map((row) => row.category))).size).toBeGreaterThanOrEqual(4);
    expect(tables.some((table) => table.input_loads!.some((row) => /^support:\S+ \S+$/.test(String(row.target))))).toBe(true);
    for (const key of ["pipes", "components", "nodes"] as const) expect(tables.some((table) => table.input_geometry![key].length > 0)).toBe(true);
    expect(tables.some((table) => table.input_materials_sections!.sections.length > 0)).toBe(true);
    expect(tables.some((table) => table.input_supports!.some((row) => row.stiffness !== null))).toBe(true);
    for (const table of tables) expect(Object.keys(table).sort()).toEqual([...MODEL_INPUT_SECTION_IDS].sort());
  });

  it("projects only listed ids, in model order, and refuses a member of another form by its pointer", () => {
    expect(modelInputTables({}, null)).toBeNull();
    expect(listedModelInputIds(COMPOSITION)).toEqual([...MODEL_INPUT_SECTION_IDS]);
    const model = { nodes: [{ id: "node:b", position: { x: -0, y: 1e21, z: 0.1 } }, { id: "node:a" }] };
    const only = { composition_version: "1.0.0", sections: [{ section_id: "input_geometry", cases: "all", view: { kind: "all" } }] } as ReportComposition;
    expect(modelInputTables(model, only)).toEqual({ input_geometry: {
      nodes: [{ id: "node:b", label: null, x: "0", y: "1e+21", z: "0.1" }, { id: "node:a", label: null, x: null, y: null, z: null }],
      pipes: [], components: []
    } });
    expect(() => modelInputTables({ nodes: [{ id: "node:a", label: { secret: "x" } }] }, only))
      .toThrow("REPORT-COMPOSITION-MODEL-INPUT-UNBOUND: the model's /nodes/0/label is not a stored string, number, quantity or null, so its model-input table cannot be projected");
  });
});
