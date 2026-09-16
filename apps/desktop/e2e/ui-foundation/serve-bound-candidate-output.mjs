#!/usr/bin/env node
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { readFile, realpath, stat } from "node:fs/promises";
import path from "node:path";

const sourceRootInput = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT;
const outputRootInput = process.env.UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT;
const manifestPathInput = process.env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST;
const expectedManifestSha256 = process.env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256;
const expectedSourceStage = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE;
if (![sourceRootInput, outputRootInput, manifestPathInput].every((value) => value && path.isAbsolute(value)) ||
    !/^[0-9a-f]{64}$/.test(expectedManifestSha256 ?? "") || !["preliminary", "final"].includes(expectedSourceStage)) {
  throw new Error("absolute candidate source/output/manifest paths, expected manifest SHA256, and preliminary/final source stage are required");
}
const [sourceRoot, outputRoot, manifestPath] = await Promise.all([
  realpath(sourceRootInput), realpath(outputRootInput), realpath(manifestPathInput)
]);
if (outputRoot !== path.join(sourceRoot, "apps/desktop/dist")) {
  throw new Error("candidate output root must be the bound source root apps/desktop/dist");
}
const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
const manifestBytes = await readFile(manifestPath);
if (digest(manifestBytes) !== expectedManifestSha256) throw new Error("candidate bundle manifest SHA256 mismatch");
const manifest = JSON.parse(manifestBytes.toString("utf8"));
if (manifest.schema !== "openpipestress.ui-foundation.external-candidate-production-bundle-manifest/v2" ||
    manifest.phase !== "candidate" || manifest.sourceStage !== expectedSourceStage ||
    manifest.candidateSourceRoot !== sourceRoot || !Array.isArray(manifest.mutableTestOnlySourceSnapshot)) {
  throw new Error("candidate bundle manifest identity mismatch");
}
const isTestOnlySourcePath = (relative) => /(^|\/)(__tests__\/|[^/]+\.(?:test|spec)\.[^/]+$)/.test(relative) ||
  /^apps\/desktop\/playwright(?:\.dist)?\.config\.ts$/.test(relative);
if (manifest.source.some((entry) => isTestOnlySourcePath(entry.path)) ||
    manifest.mutableTestOnlySourceSnapshot.some((entry) => !isTestOnlySourcePath(entry.path))) {
  throw new Error("candidate manifest production/test-only source classification mismatch");
}
for (const entry of [...manifest.source, ...manifest.files]) {
  const absolute = path.join(sourceRoot, entry.path);
  if (absolute !== sourceRoot && !absolute.startsWith(`${sourceRoot}${path.sep}`)) throw new Error(`manifest path escapes source root: ${entry.path}`);
  const bytes = await readFile(absolute);
  if (bytes.length !== entry.bytes || digest(bytes) !== entry.sha256) throw new Error(`candidate manifest entry drift: ${entry.path}`);
}

if (process.env.UI_FOUNDATION_CANDIDATE_VALIDATE_ONLY === "1") {
  process.stdout.write(`${JSON.stringify({
    status: "PASS_HASH_VERIFIED_EXTERNAL_CANDIDATE_BUNDLE_VALIDATE_ONLY",
    sourceRoot,
    outputRoot,
    manifestPath,
    manifestSha256: expectedManifestSha256,
    sourceStage: expectedSourceStage
  })}\n`);
  process.exit(0);
}

const mime = new Map([
  [".css", "text/css; charset=utf-8"], [".html", "text/html; charset=utf-8"],
  [".ico", "image/x-icon"], [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"], [".map", "application/json; charset=utf-8"],
  [".png", "image/png"], [".svg", "image/svg+xml"], [".wasm", "application/wasm"]
]);
const server = createServer(async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://127.0.0.1").pathname);
    const relative = pathname === "/" ? "index.html" : pathname.replace(/^\/+/, "");
    let absolute = path.resolve(outputRoot, relative);
    if (absolute !== outputRoot && !absolute.startsWith(`${outputRoot}${path.sep}`)) throw new Error("path escapes candidate output root");
    try {
      if ((await stat(absolute)).isDirectory()) absolute = path.join(absolute, "index.html");
    } catch (error) {
      if (path.extname(relative)) throw error;
      absolute = path.join(outputRoot, "index.html");
    }
    const bytes = await readFile(absolute);
    response.writeHead(200, {
      "content-type": mime.get(path.extname(absolute)) ?? "application/octet-stream",
      "cache-control": "no-store",
      "x-openpipestress-candidate-manifest-sha256": expectedManifestSha256,
      "x-openpipestress-candidate-source-stage": expectedSourceStage,
      "x-content-type-options": "nosniff"
    });
    response.end(bytes);
  } catch (error) {
    response.writeHead(404, {
      "content-type": "text/plain; charset=utf-8",
      "cache-control": "no-store",
      "x-openpipestress-candidate-manifest-sha256": expectedManifestSha256,
      "x-openpipestress-candidate-source-stage": expectedSourceStage
    });
    response.end(`not found: ${String(error)}`);
  }
});
server.listen(5176, "127.0.0.1", () => {
  process.stdout.write(`${JSON.stringify({
    status: "SERVING_HASH_VERIFIED_EXTERNAL_CANDIDATE_BUNDLE",
    sourceRoot,
    outputRoot,
    manifestPath,
    manifestSha256: expectedManifestSha256,
    sourceStage: expectedSourceStage
  })}\n`);
});
const close = () => server.close(() => process.exit(0));
process.once("SIGINT", close);
process.once("SIGTERM", close);
