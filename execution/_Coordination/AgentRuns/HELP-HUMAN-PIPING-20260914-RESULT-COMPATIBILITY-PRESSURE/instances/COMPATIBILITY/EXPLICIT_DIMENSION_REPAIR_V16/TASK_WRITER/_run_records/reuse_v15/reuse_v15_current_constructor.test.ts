import { describe, expect, it } from "vitest";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { buildCurrentSessionInputManifest } from "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/services/inputManifestService";
import { buildAnalysisRunPreview } from "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/services/previewService";
import { buildCurrentResultExport } from "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/features/result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/features/stress-neutral/StressNeutralExportPanel";
import type { MechanicsResult, PreviewModel } from "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/types";

const project = "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping";
const v15 = path.join(project, "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records");
const output = path.join(project, "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/EXPLICIT_DIMENSION_REPAIR_V16/TASK_WRITER/_run_records/reuse_v15/output");
const load = (file: string) => JSON.parse(readFileSync(file, "utf8"));

async function compose(name: string, model: PreviewModel, result: MechanicsResult, inputManifest: any, analysisRun: any) {
  const before = JSON.stringify({ model, result });
  const resultExport = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
  const stress = await buildStressNeutralExportPacket({ model, result, analysisRun });
  await validateStressNeutralExportPacket(stress);
  expect(JSON.stringify({ model, result })).toBe(before);
  expect(resultExport.result_envelope.row_accounting).toHaveLength(result.results.length);
  expect(stress.result_rows).toHaveLength(result.results.length);
  const dir = path.join(output, name); mkdirSync(dir, { recursive: true });
  writeFileSync(path.join(dir, "stress_neutral_v0_2.json"), JSON.stringify(stress));
  return { name, rows: result.results.length, explicit_dimensions: result.results.filter((row) => Object.hasOwn(row, "dimension")).length };
}

describe("V15 carrier reuse through V16 Current constructor", () => {
  it("composes seven genuine V15 pairs and the original 830-row carrier", async () => {
    const caseMap = load(path.join(v15, "case_map_v15.json"));
    const receipts = [];
    for (const [name, entry] of Object.entries(caseMap.cases) as Array<[string, any]>) {
      const request = load(entry.request_path), producer = load(entry.output_path);
      const model = request.solve.preview_model.model;
      const inputManifest = await buildCurrentSessionInputManifest({ model, solver: entry.solver, active_rule_packs: [], external_assets: [] });
      const analysisRun = await buildAnalysisRunPreview(producer.payload.mechanics_envelope, { inputManifest });
      receipts.push(await compose(name, model, producer.payload.mechanics_envelope, inputManifest, analysisRun));
    }
    const model = load(path.join(project, "fixtures/product_preview/invented_preview_model.json"));
    const result = load(path.join(project, "fixtures/product_preview/invented_mechanics_result.json"));
    const solver = { solver_name: "open_pipe_stress_product_physics", solver_version: "0.1.0", solver_build_ref: "open_pipe_stress_product_physics@0.1.0", solver_mode: "sparse_interactive", settings: {} };
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
    receipts.push(await compose("original_830", model, result, inputManifest, analysisRun));
    expect(receipts).toHaveLength(8);
    expect(receipts.find((item) => item.name === "original_830")).toEqual({ name: "original_830", rows: 830, explicit_dimensions: 0 });
    writeFileSync(path.join(output, "receipts.json"), JSON.stringify(receipts, null, 2) + "\n");
  });
});
