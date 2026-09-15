import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { expect, it } from "vitest";

import modelJson from "../../../../../../../../../../fixtures/product_preview/invented_preview_model.json";
import resultJson from "../../../../../../../../../../fixtures/product_preview/invented_mechanics_result.json";
import { buildCurrentSessionInputManifest } from "../../../../../../../../../../apps/desktop/src/services/inputManifestService";
import { buildAnalysisRunPreview } from "../../../../../../../../../../apps/desktop/src/services/previewService";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "../../../../../../../../../../apps/desktop/src/features/stress-neutral/StressNeutralExportPanel";
import type { MechanicsResult, PreviewModel } from "../../../../../../../../../../apps/desktop/src/types";

it("captures actual withheld packets from the unchanged desktop producer", async () => {
  const model = modelJson as PreviewModel;
  const baseResult = structuredClone(resultJson) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: "fixture", solver_version: "1", solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [],
    external_assets: [],
  });
  const analysisRun = await buildAnalysisRunPreview(baseResult, { inputManifest });
  const cases: Array<[string, string, (row: MechanicsResult["results"][number]) => void]> = [
    ["unknown-kind", "SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC", (row) => { row.kind = "unknown_native_quantity"; }],
    ["missing-component", "SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC", (row) => { delete (row as any).metadata.component; }],
    ["unit-component-contradiction", "SN-UNIT-WITNESS-WITHHELD-CONTRADICTION", (row) => { row.unit = "Pa"; }],
  ];
  const output = process.env.RESULTS_CONTRACT_OUTPUT_DIR;
  expect(output).toBeTruthy();
  mkdirSync(output!, { recursive: true });
  const summary = [];
  for (const [name, code, mutate] of cases) {
    const result = structuredClone(baseResult);
    const source = result.results.find((row) => row.kind === "element_local_axial_force")!;
    mutate(source);
    const packet = await buildStressNeutralExportPacket({ model, result, analysisRun });
    await expect(validateStressNeutralExportPacket(packet)).resolves.toBeUndefined();
    const findings = packet.diagnostics.filter((item: any) => item.source?.ref === source.id && item.code === code);
    expect(findings).toHaveLength(1);
    expect(packet.unit_preservation_witnesses.some((item: any) => item.result_id === source.id)).toBe(false);
    expect(packet.validation_ready).toBe(false);
    writeFileSync(path.join(output!, `${name}.packet.json`), JSON.stringify(packet));
    summary.push({ name, code, source_result_id: source.id, row: packet.result_rows.find((item: any) => item.result_id === source.id), finding: findings[0] });
  }
  writeFileSync(path.join(output!, "producer-probes.summary.json"), JSON.stringify(summary, null, 2) + "\n");
});
