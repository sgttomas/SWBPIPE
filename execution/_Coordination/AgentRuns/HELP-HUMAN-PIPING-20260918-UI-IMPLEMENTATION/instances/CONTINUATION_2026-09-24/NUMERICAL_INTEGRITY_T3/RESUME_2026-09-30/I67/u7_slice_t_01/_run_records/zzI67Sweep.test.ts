// I67 U6d existing-identity sweep (scratch lane only, never committed). Copied
// into src/features/results/ of a lane and run there. It walks every JSON file
// under P/{fixtures,core,tests}, finds every mechanics envelope (to depth 4),
// and records each carrier and standing outcome the desktop exposes, using only
// functions that exist at base. Output: I67_SWEEP_OUT (TSV + JSONL).
import { it } from "vitest";
import { createHash } from "node:crypto";
import { appendFileSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../../types";
import { sourceContract, currentSemanticContract, hasCurrentSourceContract, numericalResultStanding } from "./numericalResultQuality";
import { isFreshSemanticResult, standingReason, knownSemanticNotices, resultRowLabel, ruleBindingRefusal } from "./knownSemanticLimitations";
import { isLoadReferenceRoute, loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import { semanticContractForSource, semanticCategory } from "./resultSemantics";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { buildAnalysisRunV03, validateAnalysisRunV03, buildAnalysisRunV02 } from "../../services/analysisRunCompatibility";
import { bindSourceResultDimensions } from "../../services/previewService";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const P = resolve(__dirname, "../../../../../");
const OUT = process.env.I67_SWEEP_OUT!;
const isEnvelope = (v: Json) => v && typeof v === "object" && !Array.isArray(v) && Array.isArray(v.results) && Array.isArray(v.diagnostics) && v.status && typeof v.status === "object" && typeof v.run_id === "string";
function find(v: Json, path: string, depth: number, out: [string, Json][]) {
  if (isEnvelope(v)) { out.push([path, v]); return; }
  if (depth <= 0 || !v || typeof v !== "object") return;
  for (const [k, x] of Object.entries(v)) find(x, `${path}/${k}`, depth - 1, out);
}
function walk(dir: string, files: string[]) {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (name === "node_modules" || name === "target" || name.startsWith(".")) continue;
    const st = statSync(full);
    if (st.isDirectory()) walk(full, files);
    else if (name.endsWith(".json") && st.size < 64 * 1024 * 1024) files.push(full);
  }
}
const safe = <T,>(f: () => T): T | string => { try { return f(); } catch (e) { return `THROW:${e instanceof Error ? e.message : String(e)}`; } };
const safeAsync = async <T,>(f: () => Promise<T>): Promise<T | string> => { try { return await f(); } catch (e) { return `THROW:${e instanceof Error ? e.message : String(e)}`; } };
const manifestFor = (s: Json) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:i67-sweep" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: s.model_ref }, solver_basis: { solver_name: s.producer?.component_name ?? "x", solver_version: s.producer?.component_version ?? "x", solver_build_ref: "i67:sweep" } } });

it("i67 sweep", async () => {
  const files: string[] = [];
  for (const d of ["fixtures", "core", "tests"]) walk(join(P, d), files);
  files.sort();
  writeFileSync(`${OUT}.tsv`, ""); writeFileSync(`${OUT}.jsonl`, "");
  let n = 0;
  for (const file of files) {
    let doc: Json;
    try { doc = JSON.parse(readFileSync(file, "utf8")); } catch { continue; }
    const found: [string, Json][] = [];
    find(doc, "", 4, found);
    for (const [pointer, original] of found) {
      const source = structuredClone(original) as MechanicsResult;
      const ids = Array.isArray(source.numerical_quality?.cases) ? source.numerical_quality!.cases.map((c: Json) => c?.basis_ref?.ref_id) : [];
      const model = { load_cases: ids.map((id: Json) => ({ id })) } as unknown as PreviewModel;
      const o: Json = {};
      o.route = safe(() => sourceContract(source));
      o.binding = safe(() => currentSemanticContract(source));
      o.current = safe(() => hasCurrentSourceContract(source));
      o.fresh = safe(() => isFreshSemanticResult(source));
      o.standing_null = safe(() => numericalResultStanding(source, null));
      o.standing_model = safe(() => numericalResultStanding(source, model));
      o.reason = safe(() => standingReason(source));
      o.notices = safe(() => knownSemanticNotices(source));
      o.lr = safe(() => isLoadReferenceRoute(source));
      o.refusal = safe(() => loadReferenceOutputRefusal(source));
      o.report = safe(() => reportPackageUnavailableReason(source));
      o.table = safe(() => (semanticContractForSource(source) as Json).semantic_contract_id ?? "v0_2");
      const rows = Array.isArray(source.results) ? source.results : [];
      o.rows = rows.map((row: Json) => [safe(() => resultRowLabel(row, source)), safe(() => ruleBindingRefusal(source, row)), safe(() => semanticCategory(row, source))]);
      o.bound = safe(() => { const b = bindSourceResultDimensions(source); return b === source ? "same" : "new"; });
      o.v03 = await safeAsync(async () => { const r = await buildAnalysisRunV03(source, manifestFor(source)); const h = await canonicalSha256HexCheckedV1(r); const v = await safeAsync(() => validateAnalysisRunV03(r, source)); return [h, v === undefined ? "valid" : v]; });
      o.v02 = await safeAsync(async () => canonicalSha256HexCheckedV1(await buildAnalysisRunV02(source, manifestFor(source))));
      o.unchanged_input = JSON.stringify(source) === JSON.stringify(original);
      const text = JSON.stringify(o);
      const key = `${relative(P, file)}#${pointer}`;
      appendFileSync(`${OUT}.tsv`, `${key}\t${source.producer?.semantic_contract_id ?? "-"}\t${createHash("sha256").update(text).digest("hex")}\n`);
      appendFileSync(`${OUT}.jsonl`, `${JSON.stringify({ key, id: source.producer?.semantic_contract_id ?? null, o })}\n`);
      n += 1;
    }
  }
  appendFileSync(`${OUT}.tsv`, `#envelopes\t${n}\n`);
}, 3_600_000);
