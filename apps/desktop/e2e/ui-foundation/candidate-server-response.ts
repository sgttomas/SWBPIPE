import { createHash } from "node:crypto";
import { closeSync, openSync, readFileSync, readSync, realpathSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { Response } from "@playwright/test";

export type CandidateDriverBinding = Readonly<{
  driver: string;
  fixtureManifestPath: string;
  fixtureManifestSha256: string;
  fixtureFileCount: number;
  candidateBundleManifestPath: string;
  candidateBundleManifestSha256: string;
  candidateSourceRoot: string;
  candidateOutputRoot: string;
  sourceStage: "preliminary" | "final";
}>;

const digest = (bytes: Buffer | string): string => createHash("sha256").update(bytes).digest("hex");

export type RequiredChromiumExecutableBinding = Readonly<{
  executablePath: string;
  bytes: number;
  sha256: string;
}>;

export const REQUIRED_CHROMIUM_RUNTIME_BINDING = Object.freeze({
  browserVersion: "153.0.8010.36",
  product: "Chrome/153.0.8010.36",
  revision: "@507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c"
});

/**
 * Resolve and hash the exact externally provisioned Chromium before Playwright
 * can launch. Both method gates use this binding; absence, drift, directories,
 * relative paths, and silent system/bundled fallbacks fail during config load.
 */
export function bindRequiredChromiumExecutable(): RequiredChromiumExecutableBinding {
  const requestedPath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
  const expectedSha256 = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256;
  if (!requestedPath || !path.isAbsolute(requestedPath)) {
    throw new Error("PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH must be an absolute content-pinned Chromium executable path");
  }
  if (!/^[0-9a-f]{64}$/.test(expectedSha256 ?? "")) {
    throw new Error("PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256 must be the lowercase expected executable SHA256");
  }
  const executablePath = realpathSync(requestedPath);
  const executableStat = statSync(executablePath);
  if (!executableStat.isFile()) throw new Error("the bound Chromium executable is not a regular file");
  const hash = createHash("sha256");
  const buffer = Buffer.allocUnsafe(1024 * 1024);
  const descriptor = openSync(executablePath, "r");
  try {
    let offset = 0;
    while (offset < executableStat.size) {
      const count = readSync(descriptor, buffer, 0, Math.min(buffer.length, executableStat.size - offset), offset);
      if (count <= 0) throw new Error(`short read while hashing the bound Chromium executable at byte ${offset}`);
      hash.update(buffer.subarray(0, count));
      offset += count;
    }
  } finally {
    closeSync(descriptor);
  }
  const sha256 = hash.digest("hex");
  if (sha256 !== expectedSha256) {
    throw new Error(`bound Chromium executable SHA256 mismatch: derived ${sha256}`);
  }
  return Object.freeze({ executablePath, bytes: executableStat.size, sha256 });
}

const requiredAbsolutePath = (name: string): string => {
  const value = process.env[name];
  if (!value || !path.isAbsolute(value)) throw new Error(`${name} must be an absolute path`);
  return realpathSync(value);
};

/**
 * Fail before a candidate driver defines or runs a test unless its local fixture
 * bytes and externally frozen candidate manifest are the exact declared inputs.
 * Returned hashes are derived from those bytes; callers must record these values
 * instead of copying an invocation string into evidence.
 */
export function bindCandidateDriverEntry(driver: string): CandidateDriverBinding {
  const fixtureManifestPath = fileURLToPath(new URL("./fixture-manifest.json", import.meta.url));
  const fixtureManifestBytes = readFileSync(fixtureManifestPath);
  const fixtureManifestSha256 = digest(fixtureManifestBytes);
  const declaredFixtureManifestSha256 = process.env.UI_FOUNDATION_MANIFEST_SHA256;
  if (!/^[0-9a-f]{64}$/.test(declaredFixtureManifestSha256 ?? "") ||
      declaredFixtureManifestSha256 !== fixtureManifestSha256) {
    throw new Error(`candidate ${driver} fixture manifest binding mismatch: derived ${fixtureManifestSha256}`);
  }
  const fixtureManifest = JSON.parse(fixtureManifestBytes.toString("utf8"));
  if (fixtureManifest.schema !== "openpipestress.ui-foundation.fixture-manifest/v2" ||
      fixtureManifest.status !== "frozen" || !Array.isArray(fixtureManifest.files)) {
    throw new Error(`candidate ${driver} fixture manifest is not the frozen v2 protocol`);
  }
  const fixtureRoot = path.dirname(fixtureManifestPath);
  for (const entry of fixtureManifest.files) {
    if (typeof entry?.path !== "string" || typeof entry?.bytes !== "number" ||
        !/^[0-9a-f]{64}$/.test(entry?.sha256 ?? "")) {
      throw new Error(`candidate ${driver} fixture manifest has an invalid file entry`);
    }
    const absolute = path.resolve(fixtureRoot, entry.path);
    if (absolute !== fixtureRoot && !absolute.startsWith(`${fixtureRoot}${path.sep}`)) {
      throw new Error(`candidate ${driver} fixture path escapes the frozen root: ${entry.path}`);
    }
    const bytes = readFileSync(absolute);
    if (bytes.length !== entry.bytes || digest(bytes) !== entry.sha256) {
      throw new Error(`candidate ${driver} frozen fixture drift: ${entry.path}`);
    }
  }

  const candidateBundleManifestPath = requiredAbsolutePath("UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST");
  const candidateBundleManifestBytes = readFileSync(candidateBundleManifestPath);
  const candidateBundleManifestSha256 = digest(candidateBundleManifestBytes);
  const declaredCandidateBundleManifestSha256 = process.env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256;
  if (!/^[0-9a-f]{64}$/.test(declaredCandidateBundleManifestSha256 ?? "") ||
      declaredCandidateBundleManifestSha256 !== candidateBundleManifestSha256) {
    throw new Error(`candidate ${driver} bundle manifest binding mismatch: derived ${candidateBundleManifestSha256}`);
  }
  const candidateManifest = JSON.parse(candidateBundleManifestBytes.toString("utf8"));
  const candidateSourceRoot = requiredAbsolutePath("UI_FOUNDATION_CANDIDATE_SOURCE_ROOT");
  const candidateOutputRoot = requiredAbsolutePath("UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT");
  const sourceStage = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE;
  if (candidateManifest.schema !== "openpipestress.ui-foundation.external-candidate-production-bundle-manifest/v2" ||
      candidateManifest.phase !== "candidate" || candidateManifest.sourceStage !== sourceStage ||
      candidateManifest.candidateSourceRoot !== candidateSourceRoot ||
      candidateOutputRoot !== path.join(candidateSourceRoot, "apps/desktop/dist") ||
      !["preliminary", "final"].includes(sourceStage ?? "") ||
      !Array.isArray(candidateManifest.source) || !Array.isArray(candidateManifest.files) ||
      digest(JSON.stringify(candidateManifest.source)) !== candidateManifest.sourceAggregateSha256 ||
      digest(JSON.stringify(candidateManifest.files)) !== candidateManifest.bundleAggregateSha256) {
    throw new Error(`candidate ${driver} external production bundle manifest identity mismatch`);
  }
  return Object.freeze({
    driver,
    fixtureManifestPath,
    fixtureManifestSha256,
    fixtureFileCount: fixtureManifest.files.length,
    candidateBundleManifestPath,
    candidateBundleManifestSha256,
    candidateSourceRoot,
    candidateOutputRoot,
    sourceStage: sourceStage as "preliminary" | "final"
  });
}

export function assertBoundCandidateDocumentResponse(response: Response | null): void {
  const expectedManifestSha256 = process.env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256;
  const expectedSourceStage = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE;
  if (!/^[0-9a-f]{64}$/.test(expectedManifestSha256 ?? "")) {
    throw new Error("candidate document response requires the expected external bundle manifest SHA256");
  }
  if (!response || response.status() !== 200) {
    throw new Error(`candidate document response unavailable or non-200: ${response?.status() ?? "null"}`);
  }
  const headers = response.headers();
  if (headers["cache-control"] !== "no-store") {
    throw new Error(`candidate document was not served no-store: ${headers["cache-control"] ?? "missing"}`);
  }
  if (headers["x-openpipestress-candidate-manifest-sha256"] !== expectedManifestSha256) {
    throw new Error("candidate document response manifest identity mismatch");
  }
  if (!["preliminary", "final"].includes(expectedSourceStage ?? "") ||
      headers["x-openpipestress-candidate-source-stage"] !== expectedSourceStage) {
    throw new Error("candidate document response source-stage mismatch");
  }
}
