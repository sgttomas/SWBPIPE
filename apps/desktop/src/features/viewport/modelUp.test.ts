// The desktop's rule for a model's up direction (`modelUp.ts`): up is opposite the axis on which
// every weight and self-weight load acts; with none or with disagreement it stays +Y and the
// readout says so. Over the reference system's fixture and the bundled demo (both Z up by their
// weight loads), a model with no weight load and one whose weight loads disagree.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { PIPE_WEIGHT_METHOD } from "../results/weightSummary";
import { deriveModelUp, modelUpReadout, SELF_WEIGHT_MEMBER_METHOD } from "./modelUp";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;

function model(loads: Json[]): Json {
  return { load_cases: [{ id: "load:A", kind: "primitive_user_load", primitive_loads: loads }] };
}
const weight = (direction: string, value: number, extra: Json = {}) => ({ id: `load:${direction}:${value}`, category: "weight", direction, magnitude: { value, unit: "N/m" }, dimension: "force_per_length", target: { type: "element", pipe: "pipe:P" }, ...extra });

describe("a model's up direction from its weight loads", () => {
  it("is +Z for the reference system: its generated member self-weights and component weights all act along −Z", () => {
    const reference = json("fixtures/reference_system/reference_system.json");
    const derived = deriveModelUp(reference);
    // Three load cases, each with 22 member self-weights and 3 component weight forces; the
    // component weight moments (rotation_y) are not read.
    expect(derived).toEqual({ up: "+z", basis: "weight_loads", weightLoads: 75 });
    expect(modelUpReadout(derived)).toBe("Up +Z, opposite the model's 75 weight loads.");
  });

  it("is +Z for the bundled demo: its weight loads act along −Z", () => {
    expect(deriveModelUp(json("fixtures/product_preview/invented_demo_model.json"))).toEqual({ up: "+z", basis: "weight_loads", weightLoads: 2 });
  });

  it("stays +Y for a model with no weight load, and says so", () => {
    const derived = deriveModelUp(model([{ id: "load:wind", category: "occasional", direction: "global_x", magnitude: { value: 10, unit: "N/m" } }]));
    expect(derived).toEqual({ up: "+y", basis: "no_weight_load", weightLoads: 0 });
    expect(modelUpReadout(derived)).toBe("Up +Y: the model has no weight load to take it from.");
    expect(deriveModelUp({})).toEqual({ up: "+y", basis: "no_weight_load", weightLoads: 0 });
  });

  it("stays +Y when the weight loads disagree, by axis or by sign, and says so", () => {
    const byAxis = deriveModelUp(model([weight("global_z", -1), weight("global_y", -1)]));
    expect(byAxis).toEqual({ up: "+y", basis: "weight_loads_disagree", weightLoads: 2 });
    expect(modelUpReadout(byAxis)).toBe("Up +Y: the model's weight loads do not act along one direction.");
    expect(deriveModelUp(model([weight("global_z", -1), weight("global_z", 1)])).basis).toBe("weight_loads_disagree");
  });

  it("takes up opposite the direction the weight acts in, from any spelling of a global axis", () => {
    expect(deriveModelUp(model([weight("GLOBAL_Y", -9.8)])).up).toBe("+y");
    expect(deriveModelUp(model([weight("uz", 5)])).up).toBe("-z");
    expect(deriveModelUp(model([weight("global_x", -2)])).up).toBe("+x");
  });

  it("reads a member self-weight only by the generator's method, and no weight moment or weight without magnitude", () => {
    expect(SELF_WEIGHT_MEMBER_METHOD).toBe(PIPE_WEIGHT_METHOD);
    const generated = { category: "distributed_force", direction: "global_z", magnitude: { value: -3, unit: "N/m" }, provenance: JSON.stringify({ method: SELF_WEIGHT_MEMBER_METHOD }) };
    const authored = { category: "distributed_force", direction: "global_y", magnitude: { value: -3, unit: "N/m" }, provenance: "user_entered" };
    expect(deriveModelUp(model([generated, authored]))).toEqual({ up: "+z", basis: "weight_loads", weightLoads: 1 });
    const moment = weight("rotation_y", -26, { dimension: "moment" });
    const zero = weight("global_x", 0);
    expect(deriveModelUp(model([weight("global_z", -1), moment, zero]))).toEqual({ up: "+z", basis: "weight_loads", weightLoads: 1 });
  });
});
