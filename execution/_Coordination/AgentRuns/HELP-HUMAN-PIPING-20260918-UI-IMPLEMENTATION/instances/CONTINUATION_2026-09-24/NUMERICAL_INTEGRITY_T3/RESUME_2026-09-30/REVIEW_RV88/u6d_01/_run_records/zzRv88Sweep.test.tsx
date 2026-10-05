// RV88 (U6d review): independent existing-identity sweep of the desktop carriers.
// Scratch lanes only (base 844448112f, candidate 9555b6ffc2); identical source in
// both, base APIs only. Walks every JSON file under P (not execution, not
// node_modules), recursing to depth 9 into objects that carry a producer
// semantic_contract_id, plus legacy 0.1.0 raw envelopes. Per envelope, and per
// injected downgrade form, it records dispatch, binding, freshness, standing (no
// model, own-case model), standing reason, notices, output and report refusals,
// semantic table, every row's binding refusal, label and category, the rendered
// ResultsPanel text (sha256) and its standing line, and the AnalysisRun v0.3
// build/validate outcome. Output: RV88_SWEEP_OUT (TSV).
import { afterEach, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../../types";
import { sourceContract, currentSemanticContract, hasCurrentSourceContract, numericalResultStanding } from "./numericalResultQuality";
import { isFreshSemanticResult, standingReason, knownSemanticNotices, resultRowLabel, ruleBindingRefusal } from "./knownSemanticLimitations";
import { loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import { semanticContractForSource, semanticCategory } from "./resultSemantics";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { buildAnalysisRunV03, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { ResultsPanel } from "./ResultsPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const P = resolve(__dirname, "../../../../../");
const OUT = process.env.RV88_SWEEP_OUT;
const METHOD = "contribution_preserving_multiprecision_v1";
const SUCCESSOR = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
const sha = (s: string) => createHash("sha256").update(s).digest("hex").slice(0, 16);
afterEach(() => cleanup());

function walk(dir: string, files: string[]) {
  for (const name of readdirSync(dir).sort()) {
    if (["execution", "node_modules", "target", ".venv", "dist"].includes(name) || name.startsWith(".")) continue;
    const full = join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) walk(full, files);
    else if (name.endsWith(".json") && st.size < 40 * 1024 * 1024) files.push(full);
  }
}
function find(v: Json, path: string, depth: number, out: [string, Json][]) {
  if (depth > 9 || !v || typeof v !== "object") return;
  if (!Array.isArray(v)) {
    const identified = typeof v.producer?.semantic_contract_id === "string";
    const legacy = v.schema_version === "0.1.0" && Array.isArray(v.results) && v.status && typeof v.status === "object";
    if ((identified || legacy) && Array.isArray(v.results)) out.push([path, v]);
  }
  for (const [k, x] of Object.entries(v)) find(x, `${path}/${k}`, depth + 1, out);
}
const safe = (f: () => unknown): string => { try { return String(JSON.stringify(f())); } catch (e) { return `THROW:${e instanceof Error ? e.message : String(e)}`; } };
const safeAsync = async (f: () => Promise<unknown>): Promise<string> => { try { return String(JSON.stringify(await f())); } catch (e) { return `THROW:${e instanceof Error ? e.message : String(e)}`; } };
const manifestFor = (s: Json) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:rv88-sweep" }, manifest_sha256: "2".repeat(64), manifest: { model_basis: { model_ref: s.model_ref }, solver_basis: { solver_name: s.producer?.component_name ?? "x", solver_version: s.producer?.component_version ?? "x", solver_build_ref: "rv88:sweep" } } });

async function probe(key: string, source: MechanicsResult, lines: string[]) {
  const put = (field: string, value: string) => lines.push(`${key}\t${field}\t${value.replaceAll("\t", " ").replaceAll("\n", " ")}`);
  const ids = Array.isArray(source.numerical_quality?.cases) ? source.numerical_quality!.cases.map((c: Json) => c?.basis_ref?.ref_id) : [];
  const model = { load_cases: ids.map((id: Json) => ({ id })) } as unknown as PreviewModel;
  put("id", String(source.producer?.semantic_contract_id ?? ""));
  put("route", safe(() => sourceContract(source)));
  put("binding", safe(() => currentSemanticContract(source)));
  put("current", safe(() => hasCurrentSourceContract(source)));
  put("fresh", safe(() => isFreshSemanticResult(source)));
  put("standing_reason", safe(() => standingReason(source)));
  put("standing_nomodel", safe(() => numericalResultStanding(source, null)));
  put("standing_model", safe(() => numericalResultStanding(source, model)));
  put("notices", safe(() => knownSemanticNotices(source)));
  put("output_refusal", safe(() => loadReferenceOutputRefusal(source)));
  put("report_reason", safe(() => reportPackageUnavailableReason(source)));
  put("table", safe(() => (semanticContractForSource(source) as Json)?.semantic_contract_id));
  put("rows", safe(() => (source.results ?? []).map((r: Json) => [r.id, ruleBindingRefusal(source, r), resultRowLabel(r, source), safe(() => semanticCategory(r, source))])));
  put("panel", safe(() => {
    render(<ResultsPanel result={source} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    const standing = screen.queryByTestId("numerical-result-standing")?.textContent ?? null;
    const all = document.body.textContent ?? "";
    cleanup();
    return { standing, text: sha(all) };
  }));
  put("analysis_run", await safeAsync(async () => {
    const record = await buildAnalysisRunV03(structuredClone(source), manifestFor(source) as Json, undefined as Json, undefined as Json);
    await validateAnalysisRunV03(record, structuredClone(source), undefined as Json);
    return await canonicalSha256HexCheckedV1(record);
  }));
}

it("rv88 desktop sweep", async () => {
  if (!OUT) return;
  const files: string[] = [];
  walk(P, files);
  const lines: string[] = [];
  let n = 0;
  const receipt = JSON.parse(readFileSync(process.env.RV88_SPARSE!, "utf8")).source.retained_precision;
  for (const file of files) {
    let doc: Json;
    try { doc = JSON.parse(readFileSync(file, "utf8")); } catch { continue; }
    const found: [string, Json][] = [];
    find(doc, "", 0, found);
    for (const [pointer, original] of found) {
      n += 1;
      const key = `${relative(P, file)}#${pointer}`;
      await probe(key, structuredClone(original), lines);
      if (original.producer?.semantic_contract_id === SUCCESSOR) continue;
      const forms: [string, (s: Json) => void][] = [
        ["receipt", s => { s.retained_precision = structuredClone(receipt); }],
        ["null", s => { s.retained_precision = null; }],
        ["token0", s => { if (s.results.length) s.results[0].recovery_method = METHOD; }],
        ["tokenlast", s => { if (s.results.length) s.results[s.results.length - 1].recovery_method = METHOD; }],
        ["othertoken", s => { if (s.results.length) s.results[0].recovery_method = "rv88_other_method"; }],
      ];
      for (const [form, edit] of forms) {
        const s = structuredClone(original);
        edit(s);
        await probe(`${key}!${form}`, s, lines);
      }
    }
  }
  lines.push(`#envelopes\t${n}`);
  writeFileSync(OUT, lines.join("\n") + "\n");
}, 3_600_000);
