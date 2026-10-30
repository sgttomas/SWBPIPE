#!/usr/bin/env node
import { createHash } from "node:crypto";
import { lstat, mkdir, readdir, readFile, realpath, writeFile } from "node:fs/promises";
import path from "node:path";

const [sourceRootInput, output] = process.argv.slice(2);
if (!sourceRootInput || !path.isAbsolute(sourceRootInput) || !output || !path.isAbsolute(output) ||
    !output.includes(`${path.sep}instances${path.sep}VERIFY${path.sep}`)) {
  throw new Error("usage: capture-external-candidate-bundle-manifest.mjs <absolute candidate source root> <absolute instances/VERIFY/** output.json>");
}
const sourceRoot = await realpath(sourceRootInput);
const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
const sourceStage = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE ?? "preliminary";
if (!["preliminary", "final"].includes(sourceStage)) {
  throw new Error("UI_FOUNDATION_CANDIDATE_SOURCE_STAGE must be preliminary or final");
}
const isTestOnlySourcePath = (relative) => /(^|\/)(__tests__\/|[^/]+\.(?:test|spec)\.[^/]+$)/.test(relative) ||
  /^apps\/desktop\/playwright(?:\.dist)?\.config\.ts$/.test(relative);

async function walk(relativeRoot) {
  const absoluteRoot = path.join(sourceRoot, relativeRoot);
  const entries = [];
  async function visit(absolute, relative) {
    const stat = await lstat(absolute);
    if (stat.isDirectory()) {
      for (const name of (await readdir(absolute)).sort()) await visit(path.join(absolute, name), path.join(relative, name));
      return;
    }
    if (!stat.isFile()) return;
    const bytes = await readFile(absolute);
    entries.push({ path: relative.split(path.sep).join("/"), bytes: bytes.length, sha256: digest(bytes) });
  }
  await visit(absoluteRoot, relativeRoot);
  return entries;
}

const bundleRoots = ["apps/desktop/dist", "apps/desktop/public/wasm-engine"];
const files = (await Promise.all(bundleRoots.map(walk))).flat().sort((a, b) => a.path.localeCompare(b.path));
const sourceRoots = ["apps/desktop/src", "apps/desktop/src-tauri/src"];
const allSource = (await Promise.all(sourceRoots.map(walk))).flat();
const mutableTestOnlySourceSnapshot = allSource.filter((entry) => isTestOnlySourcePath(entry.path));
const source = allSource.filter((entry) => !isTestOnlySourcePath(entry.path));
for (const relative of [
  "apps/desktop/package.json", "apps/desktop/vite.config.ts", "package.json", "package-lock.json"
]) {
  const bytes = await readFile(path.join(sourceRoot, relative));
  source.push({ path: relative, bytes: bytes.length, sha256: digest(bytes) });
}
for (const relative of ["apps/desktop/playwright.config.ts", "apps/desktop/playwright.dist.config.ts"]) {
  const bytes = await readFile(path.join(sourceRoot, relative));
  mutableTestOnlySourceSnapshot.push({ path: relative, bytes: bytes.length, sha256: digest(bytes) });
}
source.sort((a, b) => a.path.localeCompare(b.path));
mutableTestOnlySourceSnapshot.sort((a, b) => a.path.localeCompare(b.path));
const payload = {
  schema: "openpipestress.ui-foundation.external-candidate-production-bundle-manifest/v2",
  phase: "candidate",
  sourceStage,
  candidateSourceRoot: sourceRoot,
  generatedAt: new Date().toISOString(),
  immutableProductionSourcePolicy: "Production source/config and built output are rehashed before serve and after the run. Test-only source is snapshot evidence only and is excluded from the immutable production-byte gate.",
  source,
  sourceAggregateSha256: digest(JSON.stringify(source)),
  mutableTestOnlySourceSnapshot,
  mutableTestOnlySourceAggregateSha256: digest(JSON.stringify(mutableTestOnlySourceSnapshot)),
  files,
  bundleAggregateSha256: digest(JSON.stringify(files))
};
await mkdir(path.dirname(output), { recursive: true });
await writeFile(output, `${JSON.stringify(payload, null, 2)}\n`);
const outputBytes = await readFile(output);
process.stdout.write(`${JSON.stringify({
  output,
  outputSha256: digest(outputBytes),
  sourceAggregateSha256: payload.sourceAggregateSha256,
  bundleAggregateSha256: payload.bundleAggregateSha256,
    sourceFileCount: source.length,
    mutableTestOnlySourceFileCount: mutableTestOnlySourceSnapshot.length,
  bundleFileCount: files.length
})}\n`);
