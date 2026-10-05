/** RV92 (U6f): existing-behaviour sweep, TypeScript carriers. Base-compatible
 * imports only, so the same file runs in the base and candidate lanes. Review
 * harness; copied into each lane after its suite ran. */
import { describe, expect, it } from "vitest";
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { sourceContract, numericalResultStanding, hasCurrentSourceContract } from "./features/results/numericalResultQuality";
import { ruleBindingRefusal, isFreshSemanticResult, standingReason, knownSemanticNotices, resultRowLabel } from "./features/results/knownSemanticLimitations";
import { loadReferenceOutputRefusal } from "./features/results/loadReferenceOutputAvailability";
import { semanticContractForSource } from "./features/results/resultSemantics";
import { buildAnalysisRunV02, buildAnalysisRunV03, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./services/analysisRunCompatibility";
import type { MechanicsResult, PreviewModel } from "./types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_SWEEP, out = process.env.RV92_OUT;
const code = (e: unknown) => { const x = e as Json; return String(x?.code ?? x?.message ?? x).split(":")[0]; };
const sha = (v: unknown) => createHash("sha256").update(JSON.stringify(v)).digest("hex");
async function r(fn: () => unknown, fmt: (v: Json) => string = () => ""): Promise<string> {
  try { return "ok:" + fmt(await fn()); } catch (e) { return "err:" + code(e); }
}
const safe = <T,>(fn: () => T): T | string => { try { return fn(); } catch (e) { return "throw:" + code(e); } };

describe.skipIf(!dir || !out)("RV92 sweep (TypeScript)", () => {
  it("writes the TS sweep", async () => {
    writeFileSync(out!, "");
    const index: Json[] = JSON.parse(readFileSync(`${dir}/index.json`, "utf8"));
    for (const entry of index) {
      if (!["raw", "raw_injected", "analysis_run"].includes(entry.group)) continue;
      const p: Json = JSON.parse(readFileSync(`${dir}/${entry.file}`, "utf8"));
      const src = p.source as MechanicsResult;
      const o: Json = { id: p.id };
      if (entry.group === "analysis_run") {
        o.verify = await r(() => verifyAnalysisRunRecord(src as Json), String);
        appendFileSync(out!, JSON.stringify(o) + "\n");
        continue;
      }
      const model = { load_cases: (p.load_cases as string[]).map(id => ({ id })) } as unknown as PreviewModel;
      o.route = safe(() => sourceContract(src));
      const ns = safe(() => numericalResultStanding(src, model));
      o.standing = typeof ns === "string" ? ns : { status: ns.status, eligible: ns.eligible, findings: ns.findings };
      o.current = safe(() => hasCurrentSourceContract(src));
      o.fresh = safe(() => isFreshSemanticResult(src));
      o.standing_reason = safe(() => standingReason(src));
      const rows: Json[] = Array.isArray(src.results) ? src.results : [];
      o.binding = rows.map(row => safe(() => ruleBindingRefusal(src, row)));
      o.output_refusal = safe(() => loadReferenceOutputRefusal(src));
      o.notices = safe(() => knownSemanticNotices(src).map(n => `${n.id}:${sha(n.text).slice(0, 12)}`));
      o.row_labels = sha(rows.map(row => safe(() => resultRowLabel(row, src))));
      o.semantic_table = safe(() => (semanticContractForSource(src) as Json)?.semantic_contract_id ?? null);
      const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv92" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: src.model_ref }, solver_basis: { solver_name: src.producer?.component_name ?? "x", solver_version: src.producer?.component_version ?? "x", solver_build_ref: "rv92" } } };
      let record: Json = null;
      o.ar_build = await r(async () => { record = await buildAnalysisRunV03(structuredClone(src), manifest as Json); return record; }, (v: Json) => sha(v));
      if (record) o.ar_validate = await r(() => validateAnalysisRunV03(record, src));
      o.ar_v02 = await r(() => buildAnalysisRunV02(structuredClone(src), manifest as Json), (v: Json) => sha(v));
      appendFileSync(out!, JSON.stringify(o) + "\n");
    }
    expect(true).toBe(true);
  }, 3_600_000);
});
