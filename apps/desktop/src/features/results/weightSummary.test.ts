// A9 derivation (DEL-07-05 Design `weight-cog-summary.md` §3 to §5). Invented geometry and
// invented generator outputs only; the last block runs the actual candidate self-weight engine.
import { describe, expect, it } from "vitest";
import { computeModelHash } from "../../services/hashService";
import { loadPreviewModel } from "../../services/previewService";
import { generateSelfWeightPlan, type SelfWeightOperationPlan, type SelfWeightPlanOutcome } from "../../services/selfWeightPlanService";
import type { PreviewModel } from "../../types";
import {
  COMPONENT_WEIGHT_METHOD,
  COMPONENT_WEIGHT_NOT_APPLIED,
  PIPE_WEIGHT_METHOD,
  buildWeightSummary,
  collectSelfWeightEvidence,
  computeWeightSummary,
  parseWeightGravity,
  planWeightSummary,
  weightSummaryCaseId,
  weightSummaryScopes,
  type SelfWeightEvidenceCollection,
  type WeightGravity,
  type WeightSummary
} from "./weightSummary";

const GRAVITY: WeightGravity = { value: -9.5, unit: "m/s^2", axis: "global_z" };

function model(lengthUnit = "m", scale = 1): PreviewModel {
  const node = (id: string, x: number, y: number, z: number) => ({ id, label: id, position: { x: x * scale, y: y * scale, z: z * scale }, provenance: "invented" });
  const pipe = (id: string, from: string, to: string) => ({ id, label: id, from, to, section: {}, material: "m", provenance: "invented" });
  return {
    schema_version: "invented", document_kind: "invented", data_boundary: {},
    project: { id: "p", name: "p", description: "", units: { length: lengthUnit } },
    analysis_status: { mechanics: "draft", rule_check: "not_run", professional_acceptance: "not_assessed" },
    nodes: [node("A", 0, 0, 0), node("B", 2, 0, 0), node("C", 2, 4, 0), node("D", 2, 4, 3), node("E", 5, 4, 3)],
    pipe_segments: [pipe("P1", "A", "B"), pipe("P2", "B", "C"), pipe("P3", "C", "D"), pipe("P4", "D", "E")],
    supports: [], components: [], load_cases: [], diagnostics: []
  } as unknown as PreviewModel;
}

const pipeEvidence = (pipe: string, value: number, extra: Record<string, unknown> = {}) => ({
  method: PIPE_WEIGHT_METHOD, pipe_id: pipe, mass_kg_per_m: Math.abs(value) / 9.5, contents_absent: true, insulation_absent: true,
  generated_payload: { id: `load:${pipe}`, magnitude: { value, unit: "N/m" } }, ...extra
});
const componentEvidence = (component: string, pipe: string, node: string, weight: number, cog: number[], part = "force") => ({
  method: COMPONENT_WEIGHT_METHOD, component_id: component, component_node: node, omitted_pipe_id: pipe, weight_N: weight, center_of_gravity_m: cog, wrench_part: part
});
const plan = (evidence: unknown[], warnings: SelfWeightOperationPlan["warnings"] = []): SelfWeightOperationPlan =>
  ({ source_model_hash: "sha256:x", changes: [], source_evidence: evidence, scope_label: "selected_pipe_mass_only", warnings });
const collected = (plans: SelfWeightOperationPlan[], refused: { pipeRef: string; code: string }[] = []): SelfWeightEvidenceCollection => ({
  state: "collected", calls: plans.length + refused.length,
  plans: plans.map((item) => ({ pipeRefs: [], plan: item })),
  refused: refused.map((item) => ({ pipeRef: item.pipeRef, diagnostics: [{ code: item.code, severity: "blocking", message: `invented ${item.code}`, remediation: "", affected_objects: [item.pipeRef] }] }))
});

function summarize(m: PreviewModel, collection: SelfWeightEvidenceCollection, scopeId = "model", selected: string[] = []) {
  const scope = weightSummaryScopes(m, selected).find((item) => item.id === scopeId)!;
  return buildWeightSummary({ model: m, scope, gravity: GRAVITY, request: planWeightSummary(m, scope), collection });
}
function available(summary: WeightSummary) {
  if (summary.state !== "available") throw new Error(`unavailable: ${summary.reason}`);
  return summary;
}

describe("scopes and inputs", () => {
  it("offers the whole model, each line and the selection, resolving and counting references", () => {
    const m = model();
    m.lines = [{ id: "line:1", label: "Line one", pipe_refs: ["P2", "P1", "P2", "P9"] }, { id: "line:2", pipe_refs: [] }];
    const scopes = weightSummaryScopes(m, ["P4", "P8"]);
    expect(scopes.map((item) => [item.id, item.label, item.pipeRefs, item.unresolved])).toEqual([
      ["model", "Whole model (4 pipes)", ["P1", "P2", "P3", "P4"], []],
      ["line:line:1", "Line Line one (line:1) (2 pipes)", ["P2", "P1"], ["P9"]],
      ["line:line:2", "Line line:2 (0 pipes)", [], []],
      ["selection", "Selected pipes (1)", ["P4"], ["P8"]]
    ]);
    expect(weightSummaryScopes(m).map((item) => item.id)).not.toContain("selection");
    expect(Object.isFrozen(scopes[0])).toBe(true);
  });

  it("assumes no gravity value, unit or axis", () => {
    expect(parseWeightGravity({ value: "", unit: "m/s^2", axis: "global_z" })).toEqual({ error: "Enter the gravity value, its unit and its direction." });
    expect(parseWeightGravity({ value: "9.8", unit: " ", axis: "global_z" })).toHaveProperty("error");
    expect(parseWeightGravity({ value: "9.8", unit: "m/s^2", axis: "" })).toHaveProperty("error");
    expect(parseWeightGravity({ value: "0", unit: "m/s^2", axis: "global_z" })).toEqual({ error: "Gravity requires a finite, nonzero number." });
    expect(parseWeightGravity({ value: "1e999", unit: "m/s^2", axis: "global_z" })).toHaveProperty("error");
    expect(parseWeightGravity({ value: "x", unit: "m/s^2", axis: "global_z" })).toHaveProperty("error");
    expect(parseWeightGravity({ value: "9.8", unit: "m/s^2", axis: "up" })).toEqual({ error: "Choose a global gravity direction." });
    expect(parseWeightGravity({ value: " -9.8 ", unit: " m/s^2 ", axis: "global_y" })).toEqual({ gravity: { value: -9.8, unit: "m/s^2", axis: "global_y" } });
  });

  it("keeps bend spans out of the request and picks a case id that names nothing in the model", () => {
    const m = model();
    m.components = [{ id: "bend:1", label: "b", kind: "bend", node: "C", provenance: "i", geometry: { bend_pipe_ref: "P2" } },
      { id: "bend:2", label: "b", kind: "bend", node: "B", provenance: "i", geometry: { bend_pipe_ref: "P2" } }];
    const scope = weightSummaryScopes(m)[0];
    expect(planWeightSummary(m, scope)).toEqual({ send: ["P1", "P3", "P4"], bendSpans: [{ pipeRef: "P2", componentIds: ["bend:1", "bend:2"] }] });
    expect(weightSummaryCaseId(m)).toBe("weight-summary-view");
    m.load_cases = [{ id: "weight-summary-view", label: "", kind: "", status: "", provenance: "", primitive_loads: [{ id: "weight-summary-view:2" }] }];
    expect(weightSummaryCaseId(m)).toBe("weight-summary-view:3");
  });
});

describe("collecting the generator's weights", () => {
  const ready = (refs: readonly string[]): SelfWeightPlanOutcome => ({
    document_kind: "openpipestress.desktop.self_weight_plan_outcome", status: "ready", input_model_unchanged: true, source_model_hash: "sha256:x",
    plan: plan(refs.map((ref) => pipeEvidence(ref, -1))), diagnostics: [], mutation_route: "structured_operations_only", requires_user_acceptance: true
  });
  const blocked = (diagnostics: SelfWeightPlanOutcome["diagnostics"]): SelfWeightPlanOutcome => ({ ...ready([]), status: "blocked", plan: null, diagnostics });
  const pipeFailure = (field: string) => ({ code: "SELF_WEIGHT_INPUT_INVALID", severity: "blocking" as const, message: `${field} invalid`, remediation: "", affected_objects: [field] });

  it("makes one call when every pipe is ready", async () => {
    const calls: string[][] = [];
    const out = await collectSelfWeightEvidence(["P1", "P2", "P3"], async (refs) => { calls.push([...refs]); return ready(refs); });
    expect(calls).toEqual([["P1", "P2", "P3"]]);
    expect(out).toMatchObject({ state: "collected", calls: 1, refused: [] });
  });

  it("halves the pipes until each refusal is held by one pipe", async () => {
    const failing = new Set(["P2", "P4"]);
    const calls: string[][] = [];
    const out = await collectSelfWeightEvidence(["P1", "P2", "P3", "P4", "P5"], async (refs) => {
      calls.push([...refs]);
      return refs.some((ref) => failing.has(ref)) ? blocked([pipeFailure("material_density")]) : ready(refs);
    });
    if (out.state !== "collected") throw new Error("refused");
    expect(out.refused.map((item) => item.pipeRef)).toEqual(["P2", "P4"]);
    expect(out.plans.map((item) => item.pipeRefs)).toEqual([["P1"], ["P3"], ["P5"]]);
    expect(out.calls).toBe(calls.length);
    expect(calls[0]).toEqual(["P1", "P2", "P3", "P4", "P5"]);
  });

  it("stops at a refusal of the request or document as a whole", async () => {
    for (const diagnostics of [
      [{ ...pipeFailure("gravity"), code: "SELF_WEIGHT_INPUT_INVALID" }],
      [{ ...pipeFailure("x"), code: "SELF-WEIGHT-SOURCE-HASH-MISMATCH" }],
      []
    ]) {
      let calls = 0;
      const out = await collectSelfWeightEvidence(["P1", "P2", "P3", "P4"], async () => { calls += 1; return blocked(diagnostics); });
      expect(out.state).toBe("refused");
      expect(calls).toBe(1);
    }
    const thrown = await collectSelfWeightEvidence(["P1"], async () => { throw new Error("SELF-WEIGHT-ENGINE-ASSET-ABSENT: invented"); });
    expect(thrown).toMatchObject({ state: "refused", diagnostics: [{ code: "SELF-WEIGHT-ENGINE-UNAVAILABLE" }] });
  });
});

describe("sums and centre of gravity", () => {
  it("sums pipe and component weights and forms the centre of gravity over the same entities", () => {
    // P1: 10 N/m over 2 m = 20 N at (1, 0, 0); P2: 5 N/m over 4 m = 20 N at (2, 2, 0);
    // P3 carries its component: 30 N at C + (0.1, 0, 0.2) m = (2.1, 4, 0.2); P4: 2 N/m over 3 m = 6 N at (3.5, 4, 3).
    const m = model();
    const summary = available(summarize(m, collected([plan([pipeEvidence("P1", -10), pipeEvidence("P2", 5, { contents_absent: false }),
      componentEvidence("V", "P3", "C", 30, [0.1, 0, 0.2]), componentEvidence("V", "P3", "C", 30, [0.1, 0, 0.2], "moment"),
      pipeEvidence("P4", -2, { insulation_absent: false, contents_absent: false })])])));
    expect(summary.entities.map((item) => [item.kind, item.pipeRef, item.weight])).toEqual([["pipe", "P1", 20], ["pipe", "P2", 20], ["component", "P3", 30], ["pipe", "P4", 6]]);
    expect(summary.pipes).toEqual({ count: 3, totals: [{ unit: "N", weight: 46 }] });
    expect(summary.components).toEqual({ count: 1, totals: [{ unit: "N", weight: 30 }] });
    expect(summary.total).toEqual({ weight: 76, unit: "N" });
    const cog = summary.centreOfGravity!;
    expect(cog.x).toBeCloseTo((20 * 1 + 20 * 2 + 30 * 2.1 + 6 * 3.5) / 76, 12);
    expect(cog.y).toBeCloseTo((20 * 2 + 30 * 4 + 6 * 4) / 76, 12);
    expect(cog.z).toBeCloseTo((30 * 0.2 + 6 * 3) / 76, 12);
    expect([summary.contentsCount, summary.insulationCount, summary.note, summary.notCounted]).toEqual([2, 1, null, []]);
    expect(Object.isFrozen(summary.entities[0])).toBe(true);
  });

  it("converts lengths to metres and offsets to the model's length unit by exact factors", () => {
    const metres = available(summarize(model(), collected([plan([pipeEvidence("P1", -10), componentEvidence("V", "P3", "C", 30, [0.1, 0, 0.2])])])));
    const millimetres = available(summarize(model("mm", 1000), collected([plan([pipeEvidence("P1", -10), componentEvidence("V", "P3", "C", 30, [0.1, 0, 0.2])])])));
    expect(millimetres.total).toEqual(metres.total);
    expect(millimetres.centreOfGravity!.x).toBeCloseTo(metres.centreOfGravity!.x * 1000, 9);
    expect(millimetres.centreOfGravity!.z).toBeCloseTo(metres.centreOfGravity!.z * 1000, 9);
    const feet = available(summarize(model("ft"), collected([plan([pipeEvidence("P1", -10)])])));
    expect(feet.total!.weight).toBeCloseTo(10 * 2 * 0.3048, 12);
    const unknown = summarize(model("furlong"), collected([plan([pipeEvidence("P1", -10)])]));
    expect(unknown).toMatchObject({ state: "unavailable", reason: expect.stringContaining('"furlong" has no routine conversion') });
  });

  it("forms no centre of gravity when no weight is counted", () => {
    const summary = available(summarize(model(), collected([])));
    expect([summary.total, summary.centreOfGravity, summary.note]).toEqual([null, null, "No weight is counted; no total or centre of gravity is formed."]);
    expect(summary.notCounted.map((item) => item.reason)).toEqual(Array(4).fill("The self-weight generator returned no weight for this pipe."));
  });

  it("counts no zero, negative or non-finite weight and takes the magnitude of a signed intensity", () => {
    const summary = available(summarize(model(), collected([plan([pipeEvidence("P1", 0), pipeEvidence("P2", Number.NaN),
      componentEvidence("V", "P3", "C", -30, [0, 0, 0]), pipeEvidence("P4", 2)])])));
    expect(summary.entities.map((item) => [item.pipeRef, item.weight])).toEqual([["P4", 6]]);
    expect(summary.notCounted.map((item) => item.pipeRef)).toEqual(["P1", "P2", "P3"]);
    expect(summary.notCounted[2].reason).toBe("The generator's weight of V is not a finite positive value.");
  });

  it("forms no total across differing units", () => {
    const summary = available(summarize(model(), collected([plan([pipeEvidence("P1", -10), { ...pipeEvidence("P2", -1), generated_payload: { magnitude: { value: -1, unit: "lbf/m" } } },
      { ...pipeEvidence("P4", -1), generated_payload: { magnitude: { value: -1, unit: "N" } } }])])));
    expect(summary.pipes.totals).toEqual([{ unit: "N", weight: 20 }, { unit: "lbf", weight: 4 }]);
    expect([summary.total, summary.centreOfGravity]).toEqual([null, null]);
    expect(summary.note).toBe("Units differ (N, lbf); no total or centre of gravity is formed.");
    expect(summary.notCounted).toEqual([{ pipeRef: "P3", reason: "The self-weight generator returned no weight for this pipe." },
      { pipeRef: "P4", reason: "The generator's intensity unit N is not per metre." }].sort((a, b) => a.pipeRef.localeCompare(b.pipeRef)));
  });

  it("counts and states pipes without a weight, bend spans and components whose weight is not included", () => {
    const m = model();
    m.components = [{ id: "bend:1", label: "b", kind: "bend", node: "C", provenance: "i", geometry: { bend_pipe_ref: "P2" } },
      { id: "valve:warned", label: "v", kind: "valve", node: "D", provenance: "i", geometry: { weight: { value: 9, unit: "N" } } },
      { id: "valve:loose", label: "v", kind: "valve", node: "E", provenance: "i", geometry: { weight: { value: 7, unit: "N" } } }];
    const warning = { id: "w", code: COMPONENT_WEIGHT_NOT_APPLIED, severity: "warning" as const, message: "invented: not applied", affected_refs: ["valve:warned", "P4"] };
    const collection = collected([plan([pipeEvidence("P1", -10), pipeEvidence("P4", -1)], [warning])], [{ pipeRef: "P3", code: "SELF_WEIGHT_INPUT_INVALID" }]);
    const whole = available(summarize(m, collection));
    expect(whole.notCounted).toEqual([
      { pipeRef: "P2", reason: "Named as a bend span by bend:1; the length along a realized bend's arc is the solver's and is not available here." },
      { pipeRef: "P3", reason: "SELF_WEIGHT_INPUT_INVALID: invented SELF_WEIGHT_INPUT_INVALID" }
    ]);
    expect(whole.componentsNotIncluded).toEqual([
      { componentId: "valve:warned", reason: "invented: not applied" },
      { componentId: "valve:loose", reason: "Its stated weight is applied by the self-weight generator to no counted member, so it is not included." }
    ]);
    expect(whole.total).toEqual({ weight: 23, unit: "N" });
    // A selection subset sums only its own pipes; components outside the generator's evidence are listed for the whole model only.
    const subset = available(summarize(m, collection, "selection", ["P4", "P1"]));
    expect(subset.entities.map((item) => item.pipeRef)).toEqual(["P4", "P1"]);
    expect(subset.notCounted).toEqual([]);
    expect(subset.componentsNotIncluded.map((item) => item.componentId)).toEqual(["valve:warned"]);
    expect(subset.centreOfGravity!.x).toBeCloseTo((20 * 1 + 3 * 3.5) / 23, 12);
  });

  it("is unavailable when the generator refuses the request", () => {
    const m = model();
    const summary = summarize(m, { state: "refused", calls: 1, diagnostics: [{ code: "SELF-WEIGHT-SOURCE-HASH-MISMATCH", severity: "blocking", message: "m", remediation: "r", affected_objects: [] }] });
    expect(summary).toMatchObject({ state: "unavailable", reason: "The self-weight generator refused the request; no weight is summed.", diagnostics: [{ code: "SELF-WEIGHT-SOURCE-HASH-MISMATCH" }] });
  });
});

describe("through the actual candidate self-weight engine", () => {
  it("sums the bundled demo's weights: the valve's 420 N stands for pipe:P-130, the other pipes state no density", async () => {
    const demo = await loadPreviewModel();
    const scope = weightSummaryScopes(demo)[0];
    const summary = available(await computeWeightSummary({ model: demo, scope, gravity: { value: -9.8, unit: "m/s^2", axis: "global_y" }, hash: computeModelHash, generate: generateSelfWeightPlan }));
    expect(summary.entities.map((item) => [item.kind, item.pipeRef, item.componentId, item.weight])).toEqual([["component", "pipe:P-130", "component:C-130", 420]]);
    expect(summary.total).toEqual({ weight: 420, unit: "N" });
    // node:N-130 is (7.6, 2.4, 0) m and the valve's centre of gravity is (0.12, 0.02, 0) m from it.
    expect(summary.centreOfGravity!.x).toBeCloseTo(7.72, 12);
    expect(summary.centreOfGravity!.y).toBeCloseTo(2.42, 12);
    expect(summary.centreOfGravity!.z).toBe(0);
    expect(summary.notCounted.map((item) => item.pipeRef)).toEqual(["pipe:P-100", "pipe:P-110", "pipe:P-120"]);
    for (const item of summary.notCounted) expect(item.reason).toContain("SELF_WEIGHT_INPUT_INVALID");
    expect(summary.engineCalls).toBe(7);
  });

  it("gives each pipe the engine's mass per length over its span, with contents and insulation as stated", async () => {
    // Invented sections: OD 0.2 m, wall 0.01 m, steel-like 8000 kg/m^3; P1 also carries 1000 kg/m^3
    // contents and 0.05 m of 100 kg/m^3 insulation. Spans 2 m (P1) and 4 m (P2), positions in mm.
    const demo = await loadPreviewModel();
    const m = structuredClone(demo);
    m.project.units = { ...m.project.units, length: "mm" };
    const section = (extra: Record<string, { value: number; unit: string }> = {}) => ({
      outside_diameter: { value: 0.2, unit: "m" }, wall_thickness: { value: 0.01, unit: "m" }, material_density: { value: 8000, unit: "kg/m^3" }, ...extra
    });
    m.nodes = [{ id: "n:a", label: "a", position: { x: 0, y: 0, z: 0 }, provenance: "invented" },
      { id: "n:b", label: "b", position: { x: 2000, y: 0, z: 0 }, provenance: "invented" },
      { id: "n:c", label: "c", position: { x: 2000, y: 4000, z: 0 }, provenance: "invented" }];
    m.pipe_segments = [
      { ...demo.pipe_segments[0], id: "pipe:1", from: "n:a", to: "n:b", section: section({ contents_density: { value: 1000, unit: "kg/m^3" }, insulation_thickness: { value: 0.05, unit: "m" }, insulation_density: { value: 100, unit: "kg/m^3" } }) },
      { ...demo.pipe_segments[0], id: "pipe:2", from: "n:b", to: "n:c", section: section() }
    ];
    m.components = []; m.supports = []; m.load_cases = []; m.combinations = [];
    const summary = available(await computeWeightSummary({ model: m, scope: weightSummaryScopes(m)[0], gravity: { value: 9.81, unit: "m/s^2", axis: "global_z" }, hash: computeModelHash, generate: generateSelfWeightPlan }));
    const ring = (outer: number, inner: number) => Math.PI / 4 * (outer * outer - inner * inner);
    const metal = ring(0.2, 0.18) * 8000;
    const w1 = (metal + ring(0.18, 0) * 1000 + ring(0.3, 0.2) * 100) * 9.81 * 2;
    const w2 = metal * 9.81 * 4;
    expect(summary.entities.map((item) => item.pipeRef)).toEqual(["pipe:1", "pipe:2"]);
    expect(summary.entities[0].weight).toBeCloseTo(w1, 8);
    expect(summary.entities[1].weight).toBeCloseTo(w2, 8);
    expect(summary.total!.weight).toBeCloseTo(w1 + w2, 8);
    expect(summary.centreOfGravity!.x).toBeCloseTo((w1 * 1000 + w2 * 2000) / (w1 + w2), 6);
    expect(summary.centreOfGravity!.y).toBeCloseTo(w2 * 2000 / (w1 + w2), 6);
    expect([summary.contentsCount, summary.insulationCount, summary.engineCalls]).toEqual([1, 1, 1]);
  });
});
