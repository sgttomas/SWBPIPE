// W-12 S3a (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.1, §6): the
// writer of the state-comparison parity golden. It flattens invented state
// pairs with the desktop's `flattenModelDocument`, compares them with
// `compareFlattened` and records both the flattened lists and the desktop's
// result. `tests/test_model_state_comparison_parity.py` runs the reference engine
// (`core/comparison/model_state/engine.py`) on the same lists through an adapter
// and requires equal summaries and per-row classifications on the shared
// projection. Regenerate with OPENPIPESTRESS_WRITE_STATE_COMPARISON_GOLDEN=<path>;
// never edit the golden by hand. All data are invented.
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { computeModelHash } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { compareFlattened, flattenModelDocument, MEMBER_ASSIGNMENT, StateComparisonRefusal, type FlatEntity } from "./stateComparison";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const GOLDEN_PATH = resolve(__dirname, "../../../../../fixtures/comparison/model_state_comparison_golden.json");
const WRITE_GOLDEN_ENV = "OPENPIPESTRESS_WRITE_STATE_COMPARISON_GOLDEN";

function everySlot(): Json {
  const model = structuredClone(everySlotModel) as Json;
  model.pressure_contract = { version: "invented-1", mode: "invented_mode", shear_model: "invented_shear" };
  model.reference_configurations = [{ id: "refcfg:R-1", label: "Invented reference", geometry_ref: "geometry:invented", member_references: [], provenance: "invented_example" }];
  return model;
}

function reversedMembers(value: Json): Json {
  if (Array.isArray(value)) return value.map(reversedMembers);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).reverse().map(([key, item]) => [key, reversedMembers(item)]));
  return value;
}

type PairSpec = { name: string; description: string; right: (model: Json) => Json; left?: (model: Json) => Json };

const PAIRS: PairSpec[] = [
  { name: "identical", description: "The same document on both sides.", right: (model) => model },
  { name: "member_order_only", description: "Every object's members in reverse order: equal JCS text and model hash.", right: reversedMembers },
  { name: "reorder_only", description: "The nodes in reverse order: only the model entity's collection order changes.", right: (model) => { model.nodes.reverse(); return model; } },
  {
    name: "field_edits",
    description: "Edits within entities: a support's label, a primitive load's magnitude (same unit), a section property, a rule-pack reference's version, the project name and a model setting's value.",
    right: (model) => {
      model.supports[0].label = "Invented relabelled support";
      model.load_cases[0].primitive_loads[0].magnitude = { value: -250, unit: "N" };
      const [property] = Object.keys(model.sections[0].properties);
      model.sections[0].properties[property] = { value: 0.25, unit: model.sections[0].properties[property].unit };
      model.rule_pack_refs[0].rule_pack_version = "9.9.9";
      model.project.name = "Invented renamed project";
      model.model_settings.node_proximity_tolerance = { value: 3, unit: "mm" };
      return model;
    }
  },
  {
    name: "unit_changes",
    description: "Quantities whose unit differs: a material's elastic modulus (Pa to MPa) and the model's node proximity tolerance (mm to m). Reported as unit_changed; the reference's unresolved with INCOMPATIBLE_UNIT.",
    right: (model) => {
      model.materials[0].elastic_modulus = { value: 200000, unit: "MPa" };
      model.model_settings.node_proximity_tolerance = { value: 0.002, unit: "m" };
      return model;
    }
  },
  {
    name: "added_and_removed",
    description: "A node, a load case with a primitive load and a constraint added; a line and an assumption removed.",
    right: (model) => {
      model.nodes.push({ id: "node:N-4", label: "Invented added node", position: { x: 9, y: 0, z: 0 }, provenance: "invented_example" });
      model.load_cases.push({ id: "load:L-2", label: "Invented case", primitive_loads: [{ ...structuredClone(model.load_cases[0].primitive_loads[0]), id: "load:L-2-X" }] });
      model.constraints.push({ ...structuredClone(model.constraints[0]), constraint_id: "constraint:K-002", name: "Invented added constraint" });
      model.lines = model.lines.filter((line: Json) => line.id !== "line:L-200");
      model.unresolved_assumptions.pop();
      return model;
    }
  },
  {
    name: "quantity_removed",
    description: "A material's shear modulus removed: a quantity on one side only is an ordinary change, not a unit change.",
    right: (model) => { delete model.materials[0].shear_modulus; return model; }
  },
  {
    name: "duplicate_id_refused",
    description: "A node id repeated on the left: the desktop refuses (COMPARISON-DUPLICATE-ID), as the reference engine raises.",
    left: (model) => { model.nodes.push(structuredClone(model.nodes[0])); return model; },
    right: (model) => model
  }
];

type GoldenPair = {
  name: string;
  description: string;
  hashes_equal: boolean;
  left_entities: FlatEntity[];
  right_entities: FlatEntity[];
  desktop:
    | { refusal: string }
    | { summary: Record<string, number>; rows: Array<{ family: string; key: string[]; id: string; classification: string; changes: Array<{ path: Array<string | number>; kind: string }> }> };
};

async function goldenPair(spec: PairSpec): Promise<GoldenPair> {
  const leftModel = (spec.left ?? ((model: Json) => model))(everySlot());
  const rightModel = spec.right(everySlot());
  const [leftHash, rightHash] = await Promise.all([computeModelHash(leftModel as PreviewModel), computeModelHash(rightModel as PreviewModel)]);
  const left = flattenModelDocument(leftModel, "left");
  const right = flattenModelDocument(rightModel, "right");
  let desktop: GoldenPair["desktop"];
  try {
    const comparison = compareFlattened(left, right);
    desktop = {
      summary: comparison.summary,
      rows: comparison.rows.map((row) => ({
        family: row.family, key: row.key, id: row.id, classification: row.classification,
        changes: row.changes.map((change) => ({ path: change.path, kind: change.kind }))
      }))
    };
    // The invariant, recorded with the golden: equal hashes exactly when every row is unchanged.
    expect(leftHash!.value === rightHash!.value, spec.name).toBe(comparison.rows.every((row) => row.classification === "unchanged"));
  } catch (error) {
    if (!(error instanceof StateComparisonRefusal)) throw error;
    desktop = { refusal: error.code };
  }
  // Plain JSON: the null-prototype records written as ordinary objects.
  const plain = (entities: FlatEntity[]) => JSON.parse(JSON.stringify(entities)) as FlatEntity[];
  return { name: spec.name, description: spec.description, hashes_equal: leftHash!.value === rightHash!.value, left_entities: plain(left), right_entities: plain(right), desktop };
}

describe("the state-comparison parity golden (W-12 S3a)", () => {
  it("is the desktop's flattening and comparison of the invented pairs (the writer)", async () => {
    const golden = {
      description: "Invented model-state pairs flattened by the desktop (apps/desktop/src/features/comparison/stateComparison.ts flattenModelDocument) from fixtures/model_document/invented_every_slot_model.json, each with the desktop's comparison (compareFlattened): its summary and per-row classifications and field paths, or its named refusal. tests/test_model_state_comparison_parity.py runs core/comparison/model_state/engine.py on the same lists through an adapter (family to category, entity key to stable_id, field paths projected to top-level keys outside the reference's skipped ids, unit-bearing paths declared in settings) and requires equal summaries and per-row classifications; unit_changed maps to the reference's unresolved with INCOMPATIBLE_UNIT. Written by apps/desktop/src/features/comparison/stateComparisonParity.test.ts with OPENPIPESTRESS_WRITE_STATE_COMPARISON_GOLDEN=<path>; never edited by hand.",
      member_assignment: MEMBER_ASSIGNMENT,
      pairs: await Promise.all(PAIRS.map(goldenPair))
    };
    const text = `${JSON.stringify(golden, null, 2)}\n`;
    const target = process.env[WRITE_GOLDEN_ENV];
    if (target) { writeFileSync(target, text); return; }
    expect(readFileSync(GOLDEN_PATH, "utf8"), `the committed golden is stale; regenerate it with ${WRITE_GOLDEN_ENV}`).toBe(text);
  });

  it("covers every classification, a unit change, a reorder and the duplicate-id refusal", async () => {
    const golden = JSON.parse(readFileSync(GOLDEN_PATH, "utf8")) as { pairs: GoldenPair[] };
    const rows = golden.pairs.flatMap((pair) => ("rows" in pair.desktop ? pair.desktop.rows : []));
    expect(new Set(rows.map((row) => row.classification))).toEqual(new Set(["added", "removed", "changed", "unchanged"]));
    expect(rows.some((row) => row.changes.some((change) => change.kind === "unit_changed"))).toBe(true);
    expect(golden.pairs.find((pair) => pair.name === "reorder_only")!.hashes_equal).toBe(false);
    expect(golden.pairs.find((pair) => pair.name === "member_order_only")!.hashes_equal).toBe(true);
    expect(golden.pairs.find((pair) => pair.name === "duplicate_id_refused")!.desktop).toEqual({ refusal: "COMPARISON-DUPLICATE-ID" });
  });
});
