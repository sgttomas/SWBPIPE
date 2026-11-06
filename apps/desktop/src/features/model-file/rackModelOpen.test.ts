// RT-U4 (DEL-04-05): the reference racks of `core/solver/performance_harness/rack/rack_model.py`
// open in the desktop. Each profile's document, as the generator writes it, passes the very
// check File > Open Model Document applies (`checkNativeOutcome`: the desktop shape check,
// then the wasm document validator) with no refusal.
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { checkNativeOutcome } from "../../services/modelDocumentFile";

const generator = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../core/solver/performance_harness/rack/rack_model.py");
const generate = (profile: string, nodes: number): string =>
  execFileSync("python3", [generator, "--profile", profile, "--nodes", String(nodes)], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"], maxBuffer: 8 * 1024 * 1024 });

describe("the reference racks open in the desktop", () => {
  for (const [profile, nodes] of [["linear-ex", 46], ["linear-ex", 82], ["linear-ex", 500], ["linear-ex", 1000], ["nonlinear-ex", 51], ["nonlinear-pf", 51]] as const) {
    it(`${profile} at ${nodes} nodes passes the open path's checks`, () => {
      const text = generate(profile, nodes);
      const outcome = checkNativeOutcome({ outcome: "opened", file_name: `${profile}.json`, byte_count: text.length, schema_status: "current", document: JSON.parse(text) });
      expect(outcome.outcome === "refused" ? outcome.diagnostics.slice(0, 5) : []).toEqual([]);
      expect(outcome.outcome).toBe("opened");
    });
  }
});
