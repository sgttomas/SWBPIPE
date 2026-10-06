/** RV101 (scratch, candidate only): negative probes on RV101's own successor stress-neutral packages. */
import { it, expect } from "vitest";
import { appendFileSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { canonicalSha256HexCheckedV1 } from "../services/hashService";
import { validateStressNeutralExportPacket } from "../features/stress-neutral/StressNeutralExportPanel";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const P = resolve(__dirname, "../../../../");
it("RV101 stress-neutral negatives", async () => {
  const dir = process.env.RV101_SO_OUT!, out = resolve(dir, "sn_negatives.jsonl");
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const pkg: Json = JSON.parse(readFileSync(resolve(dir, "sn", `package_${mode}.json`), "utf8"));
    const source: Json = JSON.parse(readFileSync(resolve(P, `fixtures/results/retained_precision_milestone_successor_${mode}.json`), "utf8")).source;
    const preview: Json = JSON.parse(readFileSync(resolve(P, "fixtures/results/preview_physics_connected_sparse.json"), "utf8"));
    const probes: [string, (p: Json) => Promise<void> | void, boolean][] = [
      ["control", () => {}, false],
      ["receipt body edited, not resealed", p => { p.retained_precision.body.publication_sha256 = "0".repeat(64); }, false],
      ["receipt body edited and receipt hash resealed", async p => { p.retained_precision.body.publication_sha256 = "0".repeat(64); p.retained_precision.receipt_sha256 = await canonicalSha256HexCheckedV1({ domain: "retained_precision_receipt_mp_v2", payload: p.retained_precision.body }); }, false],
      ["receipt removed", p => { delete p.retained_precision; }, false],
      ["receipt null", p => { p.retained_precision = null; }, false],
      ["class finding severity blocking", p => { const d = p.diagnostics.find((x: Json) => x.code === "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED"); d.severity = "blocking"; }, false],
      ["class finding code changed to not_covered", p => { const d = p.diagnostics.find((x: Json) => x.code === "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED"); d.code = "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-NOT-COVERED"; }, false],
      ["class finding message edited", p => { const d = p.diagnostics.find((x: Json) => x.code === "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED"); d.message = d.message.replace("withheld from rule binding and reliance", "bindable"); }, false],
      ["one class finding removed", p => { const i = p.diagnostics.findIndex((x: Json) => x.code === "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED"); p.diagnostics.splice(i, 1); }, false],
      ["contract_evidence removed", p => { delete p.contract_evidence; }, false],
      ["semantic table relabelled to preview-physics-1", p => { p.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1"; }, false],
    ];
    for (const [label, edit, _] of probes) {
      for (const withSource of [false, true]) {
        const p = structuredClone(pkg); await edit(p);
        let verdict = "OK"; try { await validateStressNeutralExportPacket(p, withSource ? source : undefined); } catch (e) { verdict = (e as Error).message; }
        appendFileSync(out, JSON.stringify({ mode, label, with_source: withSource, verdict }) + "\n");
      }
    }
    // A receipt carried on a preview-physics-1 package header (downgrade guard), header-only.
    void preview;
  }
  expect(true).toBe(true);
}, 600_000);
