// W-12 S3b (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.2): the run
// comparison's row identity and automatic mapping, each condition of the
// standing gate, the context deltas, the solve diagnostics added and removed,
// and the unmatched rows. The reader-backed parts (family, dimension, route,
// standing) run on committed producer results. All data are invented.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { numericalResultStanding } from "../results/resultStanding";
import type { MechanicsResult, PreviewModel } from "../../types";
import {
  compareRunInputs,
  contextDeltas,
  diagnosticChanges,
  flattenRunResult,
  resultSemanticIdentity,
  runContext,
  runDiagnostics,
  SUPPORTED_RESULT_FAMILIES,
  type FlatRunRow,
  type RunComparisonInput,
  type RunRowIdentity
} from "./runComparison";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;

function identity(id: string, overrides: Partial<RunRowIdentity> = {}): RunRowIdentity {
  return {
    result_id: id, family: "stress", object_ref: "pipe:P-1", basis_ref: { ref_type: "load_case", ref_id: "case:L-1" }, dimension: "stress",
    component: "invented_component", location: "pipe:P-1/end-a", basis: "invented basis", coordinate_system: "element_local", sign_convention: "invented",
    ...overrides
  };
}

function row(id: string, value: unknown, unit = "MPa", overrides: Partial<RunRowIdentity> = {}): FlatRunRow {
  return { identity: identity(id, overrides), value, unit, row_index: 0 };
}

const ELIGIBLE = { eligible: true, status: "integrity_checked" as const, contract: "preview_physics" as const, findings: [] };
const IDENTITY = { route: "preview_physics", contract_id: "openpipestress.result_semantics/0.3.0/preview-physics-2", contract_sha256: "ee80" };

function input(rows: FlatRunRow[], overrides: Partial<RunComparisonInput> = {}): RunComparisonInput {
  return { rows, standing: ELIGIBLE, semantic_identity: IDENTITY, context: {}, diagnostics: [], ...overrides };
}

describe("row identity and mapping", () => {
  it("maps rows whose whole identity is equal and gives right − left in their unit", () => {
    const comparison = compareRunInputs(input([row("r:1", 10), row("r:2", 4.5)]), input([row("r:2", 4), row("r:1", 12.25)]));
    expect(comparison.pairs).toEqual([
      { status: "compared", result_id: "r:1", identity: identity("r:1"), unit: "MPa", left: 10, right: 12.25, delta: 2.25 },
      { status: "compared", result_id: "r:2", identity: identity("r:2"), unit: "MPa", left: 4.5, right: 4, delta: -0.5 }
    ]);
    expect(comparison.summary).toMatchObject({ mapped_pairs: 2, compared: 2, compared_nonzero: 2, not_compared: 0, unmatched_left: 0, unmatched_right: 0 });
    expect(comparison.run_gate).toEqual([]);
  });

  it("leaves unmapped every pair whose result id is equal but another identity member differs, naming the members", () => {
    for (const field of ["family", "object_ref", "basis_ref", "dimension", "component", "location", "basis", "coordinate_system", "sign_convention"] as const) {
      const other: Partial<RunRowIdentity> = field === "basis_ref" ? { basis_ref: { ref_type: "combination", ref_id: "combo:C-1" } } : { [field]: `other-${field}` };
      const comparison = compareRunInputs(input([row("r:1", 1)]), input([row("r:1", 1, "MPa", other)]));
      expect(comparison.pairs, field).toEqual([]);
      expect(comparison.unmatched.map((entry) => [entry.side, entry.reason, entry.differing_fields]), field).toEqual([
        ["left", "identity_differs", [field]],
        ["right", "identity_differs", [field]]
      ]);
    }
  });

  it("lists rows of one run only, repeated result ids and rows without an id per side, as the reference leaves them unmapped", () => {
    const comparison = compareRunInputs(
      input([row("r:only-left", 1), row("r:dup", 1), row("r:dup", 2), row("r:shared", 3), row("", 9)]),
      input([row("r:dup", 1), row("r:shared", 3), row("r:only-right", 5)])
    );
    expect(comparison.pairs.map((pair) => [pair.result_id, pair.status])).toEqual([["r:shared", "compared"]]);
    expect(comparison.unmatched.map((entry) => [entry.side, entry.result_id, entry.reason])).toEqual([
      ["left", "", "result_id_missing"],
      ["left", "r:dup", "duplicate_result_id"],
      ["left", "r:dup", "duplicate_result_id"],
      ["left", "r:only-left", "no_row_with_this_result_id"],
      ["right", "r:dup", "duplicate_result_id"],
      ["right", "r:only-right", "no_row_with_this_result_id"]
    ]);
    expect(comparison.summary).toMatchObject({ unmatched_left: 4, unmatched_right: 2, compared: 1, compared_nonzero: 0 });
  });

  it("reads each row's family and dimension through the reader, with its metadata", () => {
    const { source } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const rows = flattenRunResult(source);
    expect(rows).toHaveLength(source.results.length);
    const mode = rows.find((entry) => entry.identity.result_id === "result:solver-mode:linear-solve-basis")!;
    expect(mode.identity).toMatchObject({ family: null, dimension: "dimensionless", component: "linear_solver_mode", object_ref: "solver:linear_static_preview" });
    const tip = rows.find((entry) => entry.identity.result_id === "result:disp:node-fixture-tip")!;
    expect(tip.identity).toMatchObject({ family: "displacement", dimension: "length", basis_ref: { ref_type: "load_case", ref_id: "case:six-component-load" } });
    expect(tip.unit).toBe("mm");
    // Every family the producer emits is one the reference compares.
    expect(new Set(rows.flatMap((entry) => (entry.identity.family ? [entry.identity.family] : [])))).toEqual(new Set(["displacement", "rotation", "force", "moment", "reaction", "stress"]));
    expect(SUPPORTED_RESULT_FAMILIES).toContain("ratio");
  });
});

describe("the standing gate (conservative, whole run)", () => {
  const left = [row("r:1", 1), row("r:2", 2, "N")];
  const right = [row("r:1", 3), row("r:2", 4, "kN")];

  it("compares nothing with a different unit, and never converts", () => {
    const comparison = compareRunInputs(input(left), input(right));
    expect(comparison.pairs[1]).toEqual({ status: "not_compared", result_id: "r:2", identity: identity("r:2"), left_unit: "N", right_unit: "kN", left: 2, right: 4, reasons: ["units_differ"] });
    expect(comparison.pairs[0].status).toBe("compared");
  });

  it("compares no pair when either run is not integrity_checked, naming the side", () => {
    const needs = { eligible: false, status: "needs_recompute" as const, contract: "pressure_source" as const, findings: ["SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED"] };
    for (const [side, reason] of [["left", "left_run_needs_recompute"], ["right", "right_run_needs_recompute"]] as const) {
      const comparison = compareRunInputs(input(left, side === "left" ? { standing: needs } : {}), input(right, side === "right" ? { standing: needs } : {}));
      expect(comparison.run_gate).toEqual([reason]);
      expect(comparison.pairs.map((pair) => pair.status === "not_compared" && pair.reasons)).toEqual([[reason], [reason, "units_differ"]]);
      expect(comparison.summary.compared).toBe(0);
    }
    const both = compareRunInputs(input(left, { standing: needs }), input(right, { standing: needs }));
    expect(both.run_gate).toEqual(["left_run_needs_recompute", "right_run_needs_recompute"]);
  });

  it("compares no pair when the runs' result semantic identities differ (route, contract or its table hash)", () => {
    for (const other of [
      { ...IDENTITY, route: "pressure" },
      { ...IDENTITY, contract_id: "openpipestress.result_semantics/0.3.0/pressure-2" },
      { ...IDENTITY, contract_sha256: "6991" }
    ]) {
      const comparison = compareRunInputs(input(left), input(right, { semantic_identity: other }));
      expect(comparison.run_gate).toEqual(["semantic_identity_differs"]);
      expect(comparison.summary.compared).toBe(0);
    }
  });

  it("computes no delta for a family the reference does not compare, or a value that is not a number", () => {
    const comparison = compareRunInputs(
      input([row("r:mode", 1, "mode_code", { family: null }), row("r:temp", 1, "degC", { family: "temperature" }), row("r:text", "x")]),
      input([row("r:mode", 2, "mode_code", { family: null }), row("r:temp", 2, "degC", { family: "temperature" }), row("r:text", 1)])
    );
    expect(comparison.pairs.map((pair) => pair.status === "not_compared" && pair.reasons)).toEqual([["family_not_compared"], ["family_not_compared"], ["value_not_numeric"]]);
  });

  it("reads the standing and route of committed producer results: eligible preview runs, an unregistered pressure-source run", () => {
    const sparse = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const pressure = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    expect(numericalResultStanding(sparse.source, sparse.model)).toMatchObject({ eligible: true, status: "integrity_checked" });
    expect(resultSemanticIdentity(sparse.source)).toEqual({ route: "preview_physics", contract_id: "openpipestress.result_semantics/0.3.0/preview-physics-2", contract_sha256: expect.stringMatching(/^[0-9a-f]{64}$/) });
    expect(resultSemanticIdentity(pressure.source).route).toBe("pressure");
    // A pressure-source result nothing registered (a recorded run's) reads needs_recompute.
    const source = json("fixtures/product_preview/pressure_source/physics_source/fields-sparse_interactive.raw.json") as MechanicsResult;
    const model = json("fixtures/product_preview/pressure_source/physics_source/fields.request.json").model as PreviewModel;
    expect(resultSemanticIdentity(source).route).toBe("pressure_source");
    expect(numericalResultStanding(source, model)).toMatchObject({ eligible: false, status: "needs_recompute" });
    // Two runs on different routes compare no pair.
    const comparison = compareRunInputs(
      input(flattenRunResult(sparse.source), { semantic_identity: resultSemanticIdentity(sparse.source) }),
      input(flattenRunResult(pressure.source), { semantic_identity: resultSemanticIdentity(pressure.source) })
    );
    expect(comparison.run_gate).toEqual(["semantic_identity_differs"]);
  });

  it("compares two eligible solves of one model: right − left per mapped row", () => {
    const model = json("fixtures/product_preview/invented_preview_model.json") as PreviewModel;
    const sparse = json("fixtures/results/preview_physics_invented_sparse.json") as MechanicsResult;
    const dense = json("fixtures/results/preview_physics_invented_dense.json") as MechanicsResult;
    const side = (result: MechanicsResult): RunComparisonInput => ({
      rows: flattenRunResult(result), standing: numericalResultStanding(result, model), semantic_identity: resultSemanticIdentity(result), context: {}, diagnostics: []
    });
    const left = side(sparse);
    const right = side(dense);
    expect([left.standing.eligible, right.standing.eligible]).toEqual([true, true]);
    const comparison = compareRunInputs(left, right);
    expect(comparison.run_gate).toEqual([]);
    const byId = new Map(dense.results.map((entry) => [entry.id, entry]));
    for (const pair of comparison.pairs) {
      if (pair.status !== "compared") continue;
      expect(pair.delta).toBe(byId.get(pair.result_id)!.value - sparse.results.find((entry) => entry.id === pair.result_id)!.value);
    }
    expect(comparison.summary.compared_nonzero).toBeGreaterThan(0);
    // Rows whose metadata basis text names the solve (the solver-mode row, the nonlinear
    // support rows) differ in it between a sparse and a dense solve, so they do not map;
    // nothing else is unmatched.
    expect(comparison.unmatched.map((entry) => [entry.side, entry.result_id, entry.reason, entry.differing_fields])).toEqual(
      expect.arrayContaining([["left", "result:solver-mode:linear-solve-basis", "identity_differs", ["basis"]]])
    );
    expect(comparison.unmatched.filter((entry) => !(entry.reason === "identity_differs" && JSON.stringify(entry.differing_fields) === '["basis"]'))).toEqual([]);
    expect(comparison.unmatched.find((entry) => entry.identity.family === "reaction")?.identity.basis).toMatch(/dense|sparse/);
  });
});

describe("context deltas and solve diagnostics", () => {
  it("compares the solver, its settings and the load-case and combination sets as the reference's _settings_deltas", () => {
    const left = runContext({
      solver_basis: { solver_name: "s", solver_version: "0.2.0", solver_build_ref: "s@0.2.0", solver_mode: "sparse_interactive", settings: { lane: true, policy: "a", gone: 1 } },
      load_basis: { load_cases: [{ id: "case:L-1" }, { id: "case:L-2" }], combinations: [{ id: "combo:C-1" }] }
    });
    const right = runContext({
      solver_basis: { solver_name: "s", solver_version: "0.2.1", solver_build_ref: "s@0.2.1", solver_mode: "dense_scrutiny", settings: { lane: true, policy: "b", added: { nested: [1] } } },
      load_basis: { load_cases: [{ id: "case:L-1" }, { id: "case:L-3" }], combinations: [] }
    });
    expect(left).toMatchObject({ solver_mode: "sparse_interactive", "settings.lane": true, "load_case:case:L-2": true, "combination:combo:C-1": true });
    expect(contextDeltas(left, right)).toEqual([
      { key: "combination:combo:C-1", left: true, right: null, kind: "removed" },
      { key: "load_case:case:L-2", left: true, right: null, kind: "removed" },
      { key: "load_case:case:L-3", left: null, right: true, kind: "added" },
      { key: "settings.added", left: null, right: { nested: [1] }, kind: "added" },
      { key: "settings.gone", left: 1, right: null, kind: "removed" },
      { key: "settings.policy", left: "a", right: "b", kind: "changed" },
      { key: "solver_build_ref", left: "s@0.2.0", right: "s@0.2.1", kind: "changed" },
      { key: "solver_mode", left: "sparse_interactive", right: "dense_scrutiny", kind: "changed" },
      { key: "solver_version", left: "0.2.0", right: "0.2.1", kind: "changed" }
    ]);
    // Member order inside a setting does not make a difference.
    expect(contextDeltas({ x: { a: 1, b: 2 } }, { x: { b: 2, a: 1 } })).toEqual([]);
  });

  it("reports solve diagnostics added and removed by code and affected refs", () => {
    const record = (diagnostics: Array<[string, string[] | null]>) => ({
      analysis_run: { diagnostics: diagnostics.map(([code, refs]) => ({ source_annotation: { code, severity: "warning", message: `${code} message`, affected_refs: refs } })) }
    });
    const left = runDiagnostics(record([["A", ["x"]], ["A", ["y"]], ["B", null], ["C", ["z"]], ["C", ["z"]]]));
    const right = runDiagnostics(record([["A", ["x"]], ["A", ["w"]], ["B", null], ["C", ["z"]], ["D", ["q", "r"]]]));
    expect(left[2]).toEqual({ code: "B", severity: "warning", affected_refs: [], message: "B message" });
    const { changes, common } = diagnosticChanges(left, right);
    expect(changes.map((change) => [change.change, change.code, change.affected_refs])).toEqual([
      ["removed", "A", ["y"]],
      ["removed", "C", ["z"]],
      ["added", "A", ["w"]],
      ["added", "D", ["q", "r"]]
    ]);
    expect(common).toBe(3);
    const comparison = compareRunInputs(input([], { diagnostics: left }), input([], { diagnostics: right }));
    expect(comparison.summary).toMatchObject({ diagnostics_added: 2, diagnostics_removed: 2, diagnostics_common: 3 });
  });

  it("reads a committed record's diagnostics (the producer's, then the rule-pack reference diagnostics)", () => {
    const record = json("fixtures/persistence/recorded_run_witness_rule_packs.json").recorded_record;
    const diagnostics = runDiagnostics(record);
    expect(diagnostics.length).toBe(record.analysis_run.diagnostics.length);
    expect(diagnostics.map((diagnostic) => diagnostic.code)).toEqual(expect.arrayContaining(["MODEL-RULE-PACK-REF-UNRESOLVED", "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH"]));
  });
});
