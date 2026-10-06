/** RV101 (scratch): header-only validation of every committed stress-neutral package, same file in base and candidate. */
import { it, expect } from "vitest";
import { appendFileSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { validateStressNeutralExportPacket } from "../features/stress-neutral/StressNeutralExportPanel";
it("RV101 committed package validation differential", async () => {
  const dir = process.env.RV101_PKG_DIR!, out = process.env.RV101_PKG_OUT!;
  const index = JSON.parse(readFileSync(resolve(dir, "index.json.txt"), "utf8"));
  for (const e of index) {
    const pkg = JSON.parse(readFileSync(resolve(dir, e.file), "utf8"));
    let verdict = "OK"; try { await validateStressNeutralExportPacket(pkg); } catch (error) { verdict = (error as Error).message; }
    appendFileSync(out, JSON.stringify({ source: e.source, schema_version: e.schema_version, verdict }) + "\n");
  }
  expect(index.length).toBeGreaterThan(0);
}, 600_000);
