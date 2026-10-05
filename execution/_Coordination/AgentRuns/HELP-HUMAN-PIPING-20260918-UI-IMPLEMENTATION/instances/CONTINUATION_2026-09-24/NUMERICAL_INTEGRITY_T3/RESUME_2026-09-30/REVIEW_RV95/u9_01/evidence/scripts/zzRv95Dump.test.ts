// RV95 scratch (never committed): the TS reader and standing seam on the live milestone successors.
import { describe, it } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { validateRetainedPrecision } from "./retainedPrecision";
import { registerRetainedPrecision, retainedPrecisionStanding } from "./retainedPrecisionStanding";

describe("rv95 dump", () => {
  it("dumps", async () => {
    const dir = process.env.RV95_IN; const outPath = process.env.RV95_OUT;
    if (!dir || !outPath) return;
    const out: unknown[] = [];
    for (const name of fs.readdirSync(dir).filter(n => n.endsWith(".json")).sort()) {
      const doc = JSON.parse(fs.readFileSync(path.join(dir, name), "utf8"));
      const inv = doc.invocation; const other = { ...inv, solver_mode: inv.solver_mode === "sparse_interactive" ? "dense_scrutiny" : "sparse_interactive" };
      for (const [label, arg] of [["with_invocation", inv], ["without_invocation", undefined], ["mode_swapped", other]] as const) {
        try {
          const r = await validateRetainedPrecision(doc.source, arg);
          out.push({ file: name, case: label, ok: true, invocation_bound: r.invocation_bound, numerical_eligible: r.numerical_eligible, standing: r.standing, publication_sha256: r.publication_sha256, n_classifications: r.classifications.length });
        } catch (e) { out.push({ file: name, case: label, ok: false, error: String((e as { gate?: string }).gate ?? "") + ":" + String((e as { code?: string }).code ?? e) }); }
      }
      // Standing seam: registered with its invocation, without a live capture, then with one.
      const model = inv.request.model;
      const srcA = JSON.parse(JSON.stringify(doc.source));
      await registerRetainedPrecision(srcA, inv, null);
      out.push({ file: name, case: "standing_no_live_capture", ...retainedPrecisionStanding(srcA, model) });
      const srcB = JSON.parse(JSON.stringify(doc.source));
      await registerRetainedPrecision(srcB, inv, (m: unknown) => m === model);
      out.push({ file: name, case: "standing_live_capture", ...retainedPrecisionStanding(srcB, model) });
      out.push({ file: name, case: "standing_live_capture_other_model", ...retainedPrecisionStanding(srcB, JSON.parse(JSON.stringify(model))) });
    }
    fs.writeFileSync(outPath, JSON.stringify(out, null, 1));
  });
});
