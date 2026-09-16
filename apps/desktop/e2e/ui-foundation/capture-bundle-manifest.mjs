import { createHash } from "node:crypto";
import { lstat, mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(here, "../../../..");
const output = process.argv[2];
const phase = process.argv[3] ?? "unspecified";
if (!output || !path.isAbsolute(output) || !output.includes(`${path.sep}instances${path.sep}VERIFY${path.sep}`)) {
  throw new Error("usage: node capture-bundle-manifest.mjs <absolute instances/VERIFY/** output.json> <phase>");
}

async function walk(relativeRoot) {
  const absoluteRoot = path.join(projectRoot, relativeRoot);
  const entries = [];
  async function visit(absolute, relative) {
    const stat = await lstat(absolute);
    if (stat.isDirectory()) {
      for (const name of (await readdir(absolute)).sort()) await visit(path.join(absolute, name), path.join(relative, name));
      return;
    }
    if (!stat.isFile()) return;
    const bytes = await readFile(absolute);
    entries.push({ path: relative.split(path.sep).join("/"), bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") });
  }
  await visit(absoluteRoot, relativeRoot);
  return entries;
}

const paths = ["apps/desktop/dist", "apps/desktop/public/wasm-engine"];
const files = (await Promise.all(paths.map(walk))).flat().sort((a, b) => a.path.localeCompare(b.path));
const sourcePaths = [
  "apps/desktop/package.json", "apps/desktop/vite.config.ts", "apps/desktop/playwright.config.ts",
  "apps/desktop/playwright.dist.config.ts", "package.json", "package-lock.json"
];
const source = [];
for (const relative of sourcePaths) {
  const bytes = await readFile(path.join(projectRoot, relative));
  source.push({ path: relative, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") });
}
for (const tree of ["apps/desktop/src", "apps/desktop/src-tauri/src"]) source.push(...await walk(tree));
source.sort((a, b) => a.path.localeCompare(b.path));
const payload = {
  schema: "openpipestress.ui-foundation.production-bundle-manifest/v1",
  phase,
  generatedAt: new Date().toISOString(),
  source,
  sourceAggregateSha256: createHash("sha256").update(JSON.stringify(source)).digest("hex"),
  files,
  bundleAggregateSha256: createHash("sha256").update(JSON.stringify(files)).digest("hex")
};
await mkdir(path.dirname(output), { recursive: true });
await writeFile(output, `${JSON.stringify(payload, null, 2)}\n`);
const outputBytes = await readFile(output);
process.stdout.write(`${JSON.stringify({
  output,
  outputSha256: createHash("sha256").update(outputBytes).digest("hex"),
  sourceAggregateSha256: payload.sourceAggregateSha256,
  bundleAggregateSha256: payload.bundleAggregateSha256,
  sourceFileCount: source.length,
  bundleFileCount: files.length
})}\n`);
