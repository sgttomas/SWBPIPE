// W-12 S3b (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.2, §6): the
// writer of the run-comparison parity golden. It flattens committed producer
// results (and invented edits of one) with the desktop's `flattenRunResult`,
// builds each run's context with `runContext`, compares pairs with
// `compareRunInputs` and records the rows, the contexts and the desktop's result.
// The standing gate is held open here (both sides eligible, one semantic
// identity): the golden covers only ungated pairs, and the gate and the extra
// identity fields are tested in `runComparison.test.ts`.
// `tests/test_analysis_run_comparison_parity.py` runs the reference engine
// (`core/comparison/analysis_run/engine.py`) on the same rows through an adapter
// to `quantity_results` and requires the same mappings, deltas, refusals and
// context deltas on the shared projection. Regenerate with
// OPENPIPESTRESS_WRITE_RUN_COMPARISON_GOLDEN=<path>; never edit the golden by
// hand. All data are invented.
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import type { MechanicsResult } from "../../types";
import { compareRunInputs, flattenRunResult, IDENTITY_FIELDS, runContext, SUPPORTED_RESULT_FAMILIES, type FlatRunRow, type RunComparisonInput } from "./runComparison";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const GOLDEN_PATH = resolve(__dirname, "../../../../../fixtures/comparison/analysis_run_comparison_golden.json");
const WRITE_GOLDEN_ENV = "OPENPIPESTRESS_WRITE_RUN_COMPARISON_GOLDEN";

const SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };

function manifestOf(model: Json, mode: string, settings: Record<string, unknown> = SETTINGS) {
  return {
    solver_basis: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: mode, settings },
    load_basis: { load_cases: model.load_cases ?? [], combinations: model.combinations ?? [] }
  };
}

/** The edited run: invented edits of the sparse solve's result and context. */
function edited(source: MechanicsResult, model: Json) {
  const result = structuredClone(source) as Json;
  const rows = result.results as Json[];
  const find = (prefix: string, nth = 0) => rows.filter((row) => row.id.startsWith(prefix))[nth];
  // Changed values (same unit): two stress rows and a force row.
  find("result:stress:")!.value += 1.5;
  find("result:stress:", 1)!.value *= 0.75;
  find("result:force:")!.value -= 10;
  // A displacement given in another unit (the reader reads it under no signature).
  const tip = rows.find((row) => row.id === "result:disp:node-fixture-tip")!;
  tip.value = tip.value / 1000;
  tip.unit = "m";
  // A row removed, a row added, a row of another entity under the same id, a repeated id.
  const removed = find("result:moment:")!;
  result.results = rows.filter((row) => row !== removed);
  result.results.push({ ...structuredClone(find("result:support-action:")), id: "result:support-action:invented-added" });
  find("result:disp:", 2)!.entity_ref = "node:invented-other";
  result.results.push(structuredClone(find("result:force:", 1)));
  const context = runContext({
    ...manifestOf(model, "sparse_interactive", { ...SETTINGS, sparse_evidence_lane: false, invented_added_setting: "on" }),
    load_basis: { load_cases: [...(model.load_cases ?? []), { id: "case:invented-added" }], combinations: model.combinations ?? [] }
  });
  return { result: result as MechanicsResult, context };
}

type GoldenRun = { description: string; rows: FlatRunRow[]; context: Record<string, unknown> };

function runs(): Record<string, GoldenRun> {
  const sparse = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const dense = nativeMechanicsReplayPair("dense_scrutiny", { profile: "preview" });
  const edit = edited(sparse.source, sparse.model);
  return {
    sparse: { description: "fixtures/results/preview_physics_connected_sparse.json (the sparse solve of fixtures/model_operations/precision_connected_ui_model.json)", rows: flattenRunResult(sparse.source), context: runContext(manifestOf(sparse.model, "sparse_interactive")) },
    dense: { description: "fixtures/results/preview_physics_connected_dense.json (the dense solve of the same model)", rows: flattenRunResult(dense.source), context: runContext(manifestOf(dense.model, "dense_scrutiny")) },
    edited: {
      description: "Invented edits of the sparse result: two stress values and a force value changed, the tip displacement given in m, a moment row removed, a support-action row added, a displacement row moved to another entity, a force row repeated; one setting changed, one added, one load case added.",
      rows: flattenRunResult(edit.result), context: edit.context
    }
  };
}

const PAIRS: Array<{ name: string; left: string; right: string; description: string }> = [
  { name: "identical", left: "sparse", right: "sparse", description: "One run on both sides: every supported pair compared with a zero delta." },
  { name: "sparse_against_dense", left: "sparse", right: "dense", description: "Two solves of one model: the solver mode differs in the context and in the solver-mode row's metadata basis (outside the shared projection: the desktop leaves it unmapped, the reference maps it)." },
  { name: "sparse_against_edited", left: "sparse", right: "edited", description: "Deltas, a unit change (the row reads under no signature, so it is unmapped), rows on one side only, an identity difference, a repeated id and context changes." }
];

const ELIGIBLE = { eligible: true, status: "integrity_checked" as const, contract: "preview_physics" as const, findings: [] };
const IDENTITY = { route: "preview_physics", contract_id: "openpipestress.result_semantics/0.3.0/preview-physics-2", contract_sha256: null };

function input(run: GoldenRun): RunComparisonInput {
  return { rows: run.rows, standing: ELIGIBLE, semantic_identity: IDENTITY, context: run.context, diagnostics: [] };
}

function golden() {
  const all = runs();
  return {
    description: "Runs flattened by the desktop (apps/desktop/src/features/comparison/runComparison.ts flattenRunResult, runContext) from committed producer results and invented edits of one, each pair with the desktop's comparison (compareRunInputs) under an open standing gate (both runs eligible, one semantic identity; the gate and the desktop's extra identity fields are tested in runComparison.test.ts). tests/test_analysis_run_comparison_parity.py runs core/comparison/analysis_run/engine.py (derive_exact_result_id_mappings, compare_analysis_runs) on the same rows through an adapter to quantity_results and the same contexts as settings, and requires the same mappings (apart from rows the desktop leaves unmapped for its extra metadata fields only), raw deltas, family and unit refusals, and settings deltas. Written by apps/desktop/src/features/comparison/runComparisonParity.test.ts with OPENPIPESTRESS_WRITE_RUN_COMPARISON_GOLDEN=<path>; never edited by hand.",
    supported_result_families: [...SUPPORTED_RESULT_FAMILIES],
    identity_fields: [...IDENTITY_FIELDS],
    runs: all,
    pairs: PAIRS.map((pair) => {
      const comparison = compareRunInputs(input(all[pair.left]), input(all[pair.right]));
      return {
        ...pair,
        desktop: {
          summary: comparison.summary,
          pairs: comparison.pairs.map((entry) => (entry.status === "compared"
            ? { result_id: entry.result_id, status: entry.status, left: entry.left, right: entry.right, delta: entry.delta, unit: entry.unit }
            : { result_id: entry.result_id, status: entry.status, reasons: entry.reasons })),
          unmatched: comparison.unmatched.map((entry) => ({ side: entry.side, result_id: entry.result_id, reason: entry.reason, differing_fields: entry.differing_fields })),
          context_deltas: comparison.context_deltas
        }
      };
    })
  };
}

describe("the run-comparison parity golden (W-12 S3b)", () => {
  it("is the desktop's flattening and comparison of the runs (the writer)", () => {
    const text = `${JSON.stringify(golden(), null, 1)}\n`;
    const target = process.env[WRITE_GOLDEN_ENV];
    if (target) { writeFileSync(target, text); return; }
    expect(readFileSync(GOLDEN_PATH, "utf8"), `the committed golden is stale; regenerate it with ${WRITE_GOLDEN_ENV}`).toBe(text);
  });

  it("covers deltas, an unsupported family, a unit change, every unmatched reason it can, and context deltas", () => {
    const committed = JSON.parse(readFileSync(GOLDEN_PATH, "utf8")) as Json;
    const edited = committed.pairs.find((pair: Json) => pair.name === "sparse_against_edited").desktop;
    expect(edited.pairs.some((pair: Json) => pair.status === "compared" && pair.delta !== 0)).toBe(true);
    // The reader reads a row's family and dimension from its kind and unit, so the tip
    // displacement in m reads under no signature: it is unmapped, not a unit difference.
    expect(edited.unmatched.filter((entry: Json) => entry.result_id === "result:disp:node-fixture-tip").map((entry: Json) => [entry.side, entry.reason, entry.differing_fields])).toEqual([
      ["left", "identity_differs", ["family", "dimension"]],
      ["right", "identity_differs", ["family", "dimension"]]
    ]);
    expect(edited.pairs.some((pair: Json) => pair.reasons?.includes("family_not_compared"))).toBe(true);
    expect(new Set(edited.unmatched.map((entry: Json) => entry.reason))).toEqual(new Set(["no_row_with_this_result_id", "identity_differs", "duplicate_result_id"]));
    expect(edited.context_deltas.map((delta: Json) => delta.kind).sort()).toEqual(["added", "added", "changed"]);
    const dense = committed.pairs.find((pair: Json) => pair.name === "sparse_against_dense").desktop;
    expect(dense.unmatched.map((entry: Json) => entry.differing_fields)).toEqual([["basis"], ["basis"]]);
    // No gate reason reaches the golden.
    const reasons = committed.pairs.flatMap((pair: Json) => pair.desktop.pairs.flatMap((entry: Json) => entry.reasons ?? []));
    expect(new Set(reasons)).toEqual(new Set(["family_not_compared"]));
  });
});
