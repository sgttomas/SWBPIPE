/** I75 scratch probe (never committed): writes successor stress-neutral packages for schema validation. */
import { expect, it, vi } from "vitest";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { checkedJsonText } from "../../services/hashService";
import { runPreviewMechanics } from "../../services/previewService";
import { registerRetainedPrecision } from "../results/retainedPrecisionStanding";
import { buildStressNeutralExportPacket } from "./StressNeutralExportPanel";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (p: string) => JSON.parse(readFileSync(resolve(root, p), "utf8"));
const manifestFor = (s: MechanicsResult, m: PreviewModel, mode: string) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-i75-probe" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: s.model_ref, model_payload: m }, solver_basis: { solver_name: s.producer!.component_name, solver_version: s.producer!.component_version, solver_build_ref: "probe", solver_mode: mode } } }) as Json;
it("writes successor packages", async () => {
  const out = process.env.I75_OUT!; mkdirSync(out, { recursive: true });
  for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
    const doc = json(`fixtures/results/retained_precision_milestone_successor_${mode}.json`);
    const model = doc.invocation.request.model as PreviewModel;
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async () => structuredClone(doc.source));
    const received = await runPreviewMechanics(model, mode);
    const analysisRun = await buildAnalysisRunV03(received, manifestFor(received, model, mode), undefined, modelLoadBasisRefs(model));
    const packet = await buildStressNeutralExportPacket({ model, result: received, analysisRun });
    writeFileSync(resolve(out, `sn_successor_${mode}.json`), JSON.stringify(packet));
    writeFileSync(resolve(out, `analysis_run_successor_${mode}.json`), JSON.stringify(analysisRun));
  }
  const corpus = json("fixtures/results/retained_precision_cases.json");
  const entry = corpus.cases.find((c: Json) => c.id === "two_case_synthetic");
  const source = structuredClone(entry.source), model = structuredClone(entry.invocation.request.model);
  const captured = checkedJsonText(model);
  await registerRetainedPrecision(source, structuredClone(entry.invocation), (m) => checkedJsonText(m) === captured);
  const analysisRun = await buildAnalysisRunV03(source, manifestFor(source, model, entry.invocation.solver_mode), undefined, modelLoadBasisRefs(model));
  writeFileSync(resolve(out, "sn_two_case_synthetic.json"), JSON.stringify(await buildStressNeutralExportPacket({ model, result: source, analysisRun })));
  expect(true).toBe(true);
});
