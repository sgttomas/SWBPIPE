import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expectedHitForProbe, projectPointToNdc, primitiveCenterIndex } from "./point-hit-oracle.mjs";
import { expectedBoxSelection } from "./box-selection-oracle.mjs";

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const POLICY_SHA256 = "bc0cb19948bec94fedddec4b8ea21005f32d011a16ca00e3d78c2ac9bb19e63c";
const BOX_POLICY_SHA256 = "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310";
const VISUAL_TOKENS_SHA256 = "009b27db887e3de224f72d0df221966fbaaa0305de218ee20848050ac6a1be1b";
const args = new Map();
for (let index = 2; index < process.argv.length; index += 2) {
  args.set(process.argv[index], process.argv[index + 1]);
}
const preflightDir = args.get("--preflight-dir");
const outputDir = args.get("--output-dir");
const visualTokensPath = args.get("--visual-tokens");
const cueSourcePath = args.get("--cue-source");
const geometrySourcePath = args.get("--geometry-source");
if (!geometrySourcePath || !path.isAbsolute(geometrySourcePath) || !cueSourcePath || !path.isAbsolute(cueSourcePath) || !preflightDir || !outputDir || !visualTokensPath || !path.isAbsolute(preflightDir) ||
    !path.isAbsolute(outputDir) || !path.isAbsolute(visualTokensPath)) {
  throw new Error("usage: node freeze-candidate-point-oracle.mjs --preflight-dir <absolute> --output-dir <absolute> --visual-tokens <absolute VISUAL_TOKENS_V4.json> --cue-source <absolute frozen viewportSelectionPresentation.ts> --geometry-source <absolute frozen viewportSelection.ts>");
}

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const stableBytes = (value) => Buffer.from(`${JSON.stringify(value)}\n`);
const triple = (value, label) => {
  const result = Array.isArray(value) ? value.map(Number) : [Number(value?.x), Number(value?.y), Number(value?.z)];
  assert.equal(result.length, 3, `${label} length`);
  assert(result.every(Number.isFinite), `${label} finite`);
  return result;
};

const cueSourceSha256 = "00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931";
assert.equal(sha256(await readFile(cueSourcePath)), cueSourceSha256, "unchanged source cue");
// Accepted gesture-lifetime additions leave primitive geometry unchanged.
const geometrySourceSha256 = "c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e";
assert.equal(sha256(await readFile(geometrySourcePath)), geometrySourceSha256, "unchanged primitive geometry source");
const dependencies = await Promise.all(["freeze-candidate-point-oracle.mjs", "point-hit-oracle.mjs", "box-selection-oracle.mjs"].map(async (name) => ({ path: name, sha256: sha256(await readFile(path.join(ROOT, name))) })));

const visualTokenBytes = await readFile(visualTokensPath);
assert.equal(sha256(visualTokenBytes), VISUAL_TOKENS_SHA256, "candidate visual-token bytes");
const visualTokens = JSON.parse(visualTokenBytes);
assert.equal(visualTokens.schema, "piping-ui-visual-tokens/v4");
assert.equal(visualTokens.status, "frozen");
await mkdir(outputDir, { recursive: true });
const manifestEntries = [];
let overallStatus = "PASS_ALL_200_ACTIONABLE_AND_PRODUCT_PROJECTION_CROSSCHECKED";
for (const pipeCount of [1_000, 10_000]) {
  const preflightPath = path.join(preflightDir, `candidate-camera-preflight-${pipeCount}.json`);
  const [preflightBytes, modelBytes, baseOracleBytes, sampleBytes] = await Promise.all([
    readFile(preflightPath),
    readFile(path.join(ROOT, "fixtures", `ui-foundation-${pipeCount}.model.json`)),
    readFile(path.join(ROOT, "samples", `ui-foundation-${pipeCount}.point-oracle-v3.json`)),
    readFile(path.join(ROOT, "samples", `ui-foundation-${pipeCount}.interactions.json`))
  ]);
  const preflight = JSON.parse(preflightBytes);
  const model = JSON.parse(modelBytes);
  const baseOracle = JSON.parse(baseOracleBytes);
  const samples = JSON.parse(sampleBytes);
  assert.equal(preflight.schema, "openpipestress.ui-foundation.candidate-camera-preflight/v1");
  assert.equal(preflight.pipeCount, pipeCount);
  assert.equal(preflight.pointHitPolicySha256, POLICY_SHA256);
  assert.deepEqual(preflight.standardCommandRecipe, ["Isometric", "Fit Model"]);
  assert.equal(preflight.modelSha256, sha256(modelBytes));
  assert.equal(preflight.visualStyleReadback?.status, "AVAILABLE");
  const resolvedTheme = preflight.visualStyleReadback.resolvedTheme;
  assert(["light", "dark"].includes(resolvedTheme), "candidate resolved theme");
  const selectedThemeTokens = visualTokens[resolvedTheme];
  const visualCue = visualTokens.viewportSelectionCue;
  assert.equal(preflight.visualStyleReadback.viewportSelectionGeometry.toLowerCase(), selectedThemeTokens.viewportSelectionGeometry.toLowerCase());
  assert.deepEqual(visualCue.inputSrgb[resolvedTheme], resolvedTheme === "light" ? [163, 68, 0] : [240, 140, 34]);
  assert.equal(preflight.diagnostics.model.projectId, model.project.id);
  assert(Number.isInteger(preflight.diagnostics.model.generation));
  const viewport = preflight.diagnostics.viewport;
  const cameraReadback = viewport.camera;
  const canvas = viewport.canvas;
  const camera = {
    position: triple(cameraReadback.position, "camera.position"),
    target: triple(cameraReadback.target, "camera.target"),
    up: triple(cameraReadback.up, "camera.up"),
    fovDegrees: Number(cameraReadback.fovDegrees),
    near: Number(cameraReadback.near),
    far: Number(cameraReadback.far),
    aspect: Number(cameraReadback.aspect)
  };
  assert([camera.fovDegrees, camera.near, camera.far, camera.aspect].every(Number.isFinite));
  assert(Math.abs(Number(canvas.cssWidth) / Number(canvas.cssHeight) - camera.aspect) <= 1e-12,
    "camera aspect matches CSS canvas");
  const productProjectionBySample = new Map(preflight.projectionCrosschecks.map((entry) => [entry.sample, entry.result]));
  const failures = [];
  const probes = baseOracle.probes.map((probe) => {
    const independentProjection = projectPointToNdc(camera, probe.authored_anchor);
    const ndc = independentProjection.ndc;
    const product = productProjectionBySample.get(probe.sample);
    const actionable = ndc.every((value) => value >= -1 && value <= 1) &&
      product?.status === "available" && product.insideClosedNdc === true && product.insideCanvasCss === true;
    const deltas = product?.status === "available" ? {
      ndcX: Math.abs(Number(product.ndc.x) - ndc[0]),
      ndcY: Math.abs(Number(product.ndc.y) - ndc[1]),
      cssX: Math.abs(Number(product.canvasCssPoint.x) - ((ndc[0] + 1) / 2) * Number(canvas.cssWidth)),
      cssY: Math.abs(Number(product.canvasCssPoint.y) - ((1 - ndc[1]) / 2) * Number(canvas.cssHeight))
    } : null;
    const projectionMatches = Boolean(deltas && deltas.ndcX <= 1e-9 && deltas.ndcY <= 1e-9 &&
      deltas.cssX <= 1e-6 && deltas.cssY <= 1e-6 &&
      product.modelGeneration === preflight.diagnostics.model.generation &&
      product.cameraSequence === cameraReadback.sequence);
    if (!actionable || !projectionMatches) failures.push({ sample: probe.sample, actionable, projectionMatches, product, ndc, deltas });
    return {
      ...probe,
      candidate_runtime: {
        ndc,
        axial_depth: independentProjection.axialDepth,
        actionability: actionable && projectionMatches
          ? "ACTIONABLE_IN_CANDIDATE_FRUSTUM"
          : "NOT_ATTEMPTED_CANDIDATE_PREFLIGHT_NOT_ACTIONABLE",
        oracle: actionable ? expectedHitForProbe(model, camera, Number(canvas.cssHeight), ndc) : {
          status: "NO_RUNTIME_ACTION",
          reason: "CANDIDATE_PREFLIGHT_PROJECTION_NOT_ACTIONABLE",
          expectedHitRef: null
        },
        product_projection_crosscheck: { result: product ?? null, deltas, status: projectionMatches ? "MATCH" : "FAIL" }
      }
    };
  });
  const visualCenters = primitiveCenterIndex(model);
  for (const probe of probes) {
    const winner = probe.candidate_runtime.oracle.expectedHitRef;
    assert(winner, "visual winner required for every prescribed probe");
    const authoredCenter = visualCenters.get(`${winner.type}\0${winner.id}`);
    assert(authoredCenter, "winner primitive geometry missing");
    const projection = projectPointToNdc(camera, authoredCenter);
    assert(projection.ndc.every((v) => Number.isFinite(v) && v >= -1 && v <= 1), "winner closed frustum");
    const centerCss = { x: Number(canvas.cssLeft) + (projection.ndc[0] + 1) * Number(canvas.cssWidth) / 2,
      y: Number(canvas.cssTop) + (1 - projection.ndc[1]) * Number(canvas.cssHeight) / 2 };
    const clip = { x: Math.floor(centerCss.x) - 24, y: Math.floor(centerCss.y) - 24, width: 48, height: 48 };
    const dpr = Number(canvas.dpr);
    assert(Number.isFinite(dpr) && dpr > 0 && Number.isSafeInteger(48 * dpr), "exact PNG/DPR mapping");
    assert(clip.x >= canvas.cssLeft && clip.y >= canvas.cssTop && clip.x + 48 <= canvas.cssLeft + canvas.cssWidth && clip.y + 48 <= canvas.cssTop + canvas.cssHeight, "fixed winner ROI inside canvas");
    const plan = { schema: "winner-cue-pair/v1", sample: probe.sample, expectedHitRef: winner, anchorRef: probe.probe_anchor_ref,
      authoredAnchor: probe.authored_anchor, authoredCenter, camera, canvas, centerCss, clip, dpr,
      centerDevice: { x: (centerCss.x - clip.x) * dpr, y: (centerCss.y - clip.y) * dpr },
      cue: { cssSize: 11, interior: 0.34, outer: 0.5, erosionDevice: 1, rimSrgb: resolvedTheme === "light" ? [255, 255, 255] : [0, 0, 0] },
      source: { cueSourceSha256, geometrySourceSha256, visualTokensSha256: VISUAL_TOKENS_SHA256, modelSha256: sha256(modelBytes), sampleSha256: sha256(sampleBytes),
        cameraPreflightSha256: sha256(preflightBytes), dependencies } };
    probe.candidate_runtime.visual_plan = { ...plan, sha256: sha256(Buffer.from(JSON.stringify(plan))) };
  }
  if (failures.length > 0) overallStatus = "FAIL_NOT_ALL_200_ACTIONABLE_OR_PROJECTION_MISMATCH";
  const output = {
    ...baseOracle,
    schema: "openpipestress.ui-foundation.candidate-runtime-point-hit-oracle/v3",
    point_hit_policy_sha256: POLICY_SHA256,
    candidate_preflight: {
      preflightPath,
      preflightSha256: sha256(preflightBytes),
      recipe: preflight.standardCommandRecipe,
      modelGeneration: preflight.diagnostics.model.generation,
      cameraSequence: cameraReadback.sequence,
      camera,
      rawCameraReadback: cameraReadback,
      rawCanvasReadback: canvas,
      localRenderOrigin: viewport.camera.localRenderOrigin,
      selection_visual_oracle: {
        source: `VISUAL_TOKENS_V4.json#viewportSelectionCue:${resolvedTheme}`,
        sourceSha256: VISUAL_TOKENS_SHA256,
        theme: resolvedTheme,
        srgb: visualCue.inputSrgb[resolvedTheme],
        sceneSrgb: resolvedTheme === "light" ? [223, 229, 232] : [12, 17, 20],
        tolerancePerChannel: visualCue.renderedPixelOracle.interiorPixelPerChannelTolerance,
        requiredLocalContrastRatio: visualCue.renderedPixelOracle.requiredLocalContrastRatio,
        minimumQualifyingInteriorPixels: visualCue.renderedPixelOracle.minimumQualifyingInteriorPixels,
        actualCssSelectionToken: preflight.visualStyleReadback.viewportSelectionGeometry
      },
      expectedValueOrganization: "independent exhaustive V3 primitive evaluation with conservative per-primitive AABB rejection; product chunks not emulated",
      measuredSampleContribution: 0,
      actionableCount: 200 - failures.length,
      failureCount: failures.length,
      failures
    },
    candidate_box_selection: {
      box_selection_policy_sha256: BOX_POLICY_SHA256,
      expected_values_boundary: "Independent closed-frustum analytic oracle frozen before measured sessions; candidate box output is never an oracle input.",
      samples: samples.box_selection.map((sample) => expectedBoxSelection(model, camera, sample))
    },
    probes
  };
  const outputPath = path.join(outputDir, `ui-foundation-${pipeCount}.candidate-runtime-point-oracle-v3.json`);
  const outputBytes = stableBytes(output);
  await writeFile(outputPath, outputBytes, { flag: "wx" });
  manifestEntries.push({
    pipeCount,
    path: path.basename(outputPath),
    sha256: sha256(outputBytes),
    bytes: outputBytes.length,
    actionableCount: 200 - failures.length,
    boxSampleCount: output.candidate_box_selection.samples.length
  });
}

const outputManifest = {
  schema: "openpipestress.ui-foundation.candidate-point-oracle-manifest/v1",
  status: overallStatus,
  pointHitPolicySha256: POLICY_SHA256,
  boxSelectionPolicySha256: BOX_POLICY_SHA256,
  sourceFixtureManifestSha256: sha256(await readFile(path.join(ROOT, "fixture-manifest.json"))),
  cueSourcePath, cueSourceSha256, geometrySourcePath, geometrySourceSha256, dependencies,
  visualTokensPath,
  visualTokensSha256: VISUAL_TOKENS_SHA256,
  files: manifestEntries
};
const outputManifestBytes = stableBytes(outputManifest);
await writeFile(path.join(outputDir, "CANDIDATE_POINT_ORACLE_MANIFEST.json"), outputManifestBytes, { flag: "wx" });
process.stdout.write(`${JSON.stringify({ ...outputManifest, manifestSha256: sha256(outputManifestBytes) }, null, 2)}\n`);
if (overallStatus !== "PASS_ALL_200_ACTIONABLE_AND_PRODUCT_PROJECTION_CROSSCHECKED") process.exitCode = 1;
