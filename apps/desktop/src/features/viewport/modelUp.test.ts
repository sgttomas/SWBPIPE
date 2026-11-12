// The desktop's rule for a model's up direction (`modelUp.ts`): "A load is read as weight if its
// category is weight, whoever authored it, or if it is a self-weight load the generator wrote,
// known by its provenance method; a distributed force not so marked is not read; weight moments are
// not read; the readout says which loads the direction was taken from, so many weight loads and so
// many generated self-weight loads, or why it stayed on Y. A model whose only gravity is generated
// self-weight is the ordinary first model and must open upright." Over the reference system's
// fixture and the bundled demo (both Z up), the ordinary first model, a hand-entered weight, a model
// with no weight load and one whose loads disagree.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { PIPE_WEIGHT_METHOD } from "../results/weightSummary";
import { deriveModelUp, modelUpReadout, SELF_WEIGHT_MEMBER_METHOD } from "./modelUp";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;

function model(loads: Json[], status = "active"): Json {
  return { load_cases: [{ id: "load:A", kind: "primitive_user_load", status, primitive_loads: loads }] };
}
const weight = (direction: string, value: number, extra: Json = {}) => ({
  id: `load:${direction}:${value}`, category: "weight", direction, magnitude: { value, unit: "N/m" }, dimension: "force_per_length",
  target: { type: "element", pipe: "pipe:P" }, provenance: "user_entered", ...extra
});
const generated = (pipe: string, direction = "global_z", value = -1070) => ({
  id: `load:self-weight:${pipe}`, category: "distributed_force", direction, magnitude: { value, unit: "N/m" }, dimension: "force_per_length",
  target: { type: "element", pipe }, provenance: JSON.stringify({ method: SELF_WEIGHT_MEMBER_METHOD, pipe_id: pipe })
});

describe("a model's up direction from its weight and generated self-weight loads", () => {
  it("is +Z for the reference system, taken from 9 weight loads and 66 generated self-weight loads, all acting along −Z", () => {
    const derived = deriveModelUp(json("fixtures/reference_system/reference_system.json"));
    // Three load cases, each with 3 component weight forces (category weight) and 22 generated
    // member self-weights; the component weight moments (rotation_y) are not read.
    expect(derived).toEqual({ up: "+z", basis: "weight_loads", weightLoads: 9, selfWeightLoads: 66, acting: ["-z"], offAxis: false });
    expect(modelUpReadout(derived)).toBe("Up +Z, opposite 9 weight loads and 66 generated self-weight loads.");
  });

  it("is +Z for the bundled demo, taken from its 2 weight loads", () => {
    const derived = deriveModelUp(json("fixtures/product_preview/invented_demo_model.json"));
    expect(derived).toMatchObject({ up: "+z", basis: "weight_loads", weightLoads: 2, selfWeightLoads: 0 });
    expect(modelUpReadout(derived)).toBe("Up +Z, opposite 2 weight loads and 0 generated self-weight loads.");
  });

  it("opens the ordinary first model upright: its only gravity is generated self-weight, no weight-category load", () => {
    const derived = deriveModelUp(model([generated("pipe:P-1"), generated("pipe:P-2")]));
    expect(derived).toMatchObject({ up: "+z", basis: "weight_loads", weightLoads: 0, selfWeightLoads: 2 });
    expect(modelUpReadout(derived)).toBe("Up +Z, opposite 0 weight loads and 2 generated self-weight loads.");
  });

  it("reads a weight-category load whoever authored it, and no distributed force without the generator's method", () => {
    expect(SELF_WEIGHT_MEMBER_METHOD).toBe(PIPE_WEIGHT_METHOD);
    const handEntered = weight("global_y", -9.8, { provenance: "user_entered" });
    const unmarked = { category: "distributed_force", direction: "global_z", magnitude: { value: -3, unit: "N/m" }, provenance: "user_entered" };
    expect(deriveModelUp(model([handEntered, unmarked]))).toMatchObject({ up: "+y", basis: "weight_loads", weightLoads: 1, selfWeightLoads: 0 });
    expect(deriveModelUp(model([unmarked])).basis).toBe("no_weight_load");
  });

  it("reads no weight moment, no weight without a magnitude, and every load case whatever its status", () => {
    const moment = weight("rotation_y", -26, { dimension: "moment" });
    const zero = weight("global_x", 0);
    expect(deriveModelUp(model([weight("global_z", -1), moment, zero]))).toMatchObject({ up: "+z", weightLoads: 1, selfWeightLoads: 0 });
    expect(deriveModelUp(model([weight("global_z", -1)], "draft")).up).toBe("+z");
  });

  it("takes up opposite the direction the loads act in, from any spelling of a global axis", () => {
    expect(deriveModelUp(model([weight("GLOBAL_Y", -9.8)])).up).toBe("+y");
    expect(deriveModelUp(model([weight("uz", 5)])).up).toBe("-z");
    expect(deriveModelUp(model([weight("global_x", -2)])).up).toBe("+x");
  });

  it("stays on Y for a model with no weight or generated self-weight load, and says why", () => {
    const derived = deriveModelUp(model([{ id: "load:wind", category: "occasional", direction: "global_x", magnitude: { value: 10, unit: "N/m" } }]));
    expect(derived).toEqual({ up: "+y", basis: "no_weight_load", weightLoads: 0, selfWeightLoads: 0, acting: [], offAxis: false });
    expect(modelUpReadout(derived)).toBe("Up +Y: the model has no weight load and no generated self-weight load.");
    expect(deriveModelUp({}).basis).toBe("no_weight_load");
  });

  it("stays on Y when the loads disagree, by axis or by sign, and names the axes", () => {
    const byAxis = deriveModelUp(model([weight("global_z", -1), generated("pipe:P-1", "global_y", -5)]));
    expect(byAxis).toMatchObject({ up: "+y", basis: "weight_loads_disagree", weightLoads: 1, selfWeightLoads: 1, acting: ["-y", "-z"] });
    expect(modelUpReadout(byAxis)).toBe("Up +Y: the model's 1 weight load and 1 generated self-weight load disagree (they act along −Y and −Z).");
    const bySign = deriveModelUp(model([weight("global_z", -1), weight("global_z", 1)]));
    expect(bySign).toMatchObject({ basis: "weight_loads_disagree", acting: ["+z", "-z"] });
    expect(modelUpReadout(bySign)).toBe("Up +Y: the model's 2 weight loads and 0 generated self-weight loads disagree (they act along +Z and −Z).");
    const offAxis = deriveModelUp(model([weight("global_z", -1), weight("local_y", -1)]));
    expect(modelUpReadout(offAxis)).toBe("Up +Y: the model's 2 weight loads and 0 generated self-weight loads disagree (they act along −Z and one acts along no global axis).");
  });
});
