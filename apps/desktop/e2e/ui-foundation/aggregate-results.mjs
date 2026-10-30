import { createHash } from "node:crypto";
import { readdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const evidenceRoot = process.argv[2];
if (!evidenceRoot || !path.isAbsolute(evidenceRoot) || !evidenceRoot.includes(`${path.sep}instances${path.sep}VERIFY${path.sep}`)) {
  throw new Error("usage: node aggregate-results.mjs <absolute instances/VERIFY/** evidence directory>");
}

function hash(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

const manifestBytes = await readFile(path.join(here, "fixture-manifest.json"));
const rawRoot = path.join(evidenceRoot, "raw");
const records = [];
for (const runName of await readdir(rawRoot).catch(() => [])) {
  const runDir = path.join(rawRoot, runName);
  for (const fixtureName of await readdir(runDir).catch(() => [])) {
    const summaryPath = path.join(runDir, fixtureName, "run-summary.json");
    try {
      records.push(JSON.parse(await readFile(summaryPath, "utf8")));
    } catch {
      records.push({ run: runName, pipeCount: fixtureName, status: "MISSING_RUN_SUMMARY", summaryPath });
    }
  }
}
records.sort((a, b) => Number(a.pipeCount) - Number(b.pipeCount) || Number(a.run) - Number(b.run));
const summary = {
  schema: "openpipestress.ui-foundation.benchmark-summary/v1",
  generatedAt: new Date().toISOString(),
  manifestSha256: hash(manifestBytes),
  records,
  counts: Object.fromEntries([...new Set(records.map((record) => String(record.status)))].sort().map((status) => [
    status,
    records.filter((record) => String(record.status) === status).length
  ])),
  acceptanceBoundary: "N/A and proxy records are never converted to PASS; full candidate qualification requires all five runs for both fixture sizes and every strict metric."
};
await writeFile(path.join(evidenceRoot, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(summary.counts)}\n`);
