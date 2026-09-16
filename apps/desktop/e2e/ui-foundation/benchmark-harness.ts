import { createHash } from "node:crypto";
import { execFile } from "node:child_process";
import { appendFile, mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { performance as nodePerformance } from "node:perf_hooks";
import { promisify } from "node:util";
import { inflateSync } from "node:zlib";
import type { Locator, Page } from "@playwright/test";
import { PAGE_CLOCK_SOURCE } from "./causal-presentation-extractor.mjs";

const execFileAsync = promisify(execFile);

export type BenchmarkPhase = "baseline" | "candidate";
export type SelectionVisualOracle = {
  source: string;
  theme: string;
  srgb: [number, number, number];
  tolerancePerChannel: number;
  minimumQualifyingInteriorPixels: number;
  sceneSrgb?: [number, number, number];
  requiredLocalContrastRatio?: number;
};

export type LoadedFixture = {
  bytes: string;
  model: any;
  samples: any;
  pointOracle: any;
  readMs: number;
  parseMs: number;
  supportFilesReadMs: number;
  supportFilesParseMs: number;
  modelPath: string;
  samplesPath: string;
  pointOraclePath: string;
  pointOracleSha256: string;
};

const root = path.dirname(new URL(import.meta.url).pathname);
const MAIN_VIEWPORT_HOST_SELECTOR = '[data-testid="viewport-canvas"]';
const MAIN_CANVAS_SELECTOR = 'canvas[data-testid="viewport-canvas"], [data-testid="viewport-canvas"] canvas';

export async function requireUniqueConnectedMainCanvas(page: Page): Promise<{
  locator: Locator;
  box: { x: number; y: number; width: number; height: number };
  identity: {
    hostCount: number;
    canvasCount: number;
    connected: boolean;
    hostContainsCanvas: boolean;
    hostIsCanvas: boolean;
    hostTestId: string | null;
    canvasTestId: string | null;
    canvasTagName: string;
  };
}> {
  const hostCount = await page.locator(MAIN_VIEWPORT_HOST_SELECTOR).count();
  const locator = page.locator(MAIN_CANVAS_SELECTOR);
  const canvasCount = await locator.count();
  if (hostCount !== 1 || canvasCount !== 1) {
    throw new Error(`main viewport requires exactly one host and one HTMLCanvasElement: ${JSON.stringify({ hostCount, canvasCount })}`);
  }
  const identity = await locator.evaluate((element, selectors) => {
    const host = document.querySelector(selectors.host);
    return {
      hostCount: document.querySelectorAll(selectors.host).length,
      canvasCount: document.querySelectorAll(selectors.canvas).length,
      connected: element instanceof HTMLCanvasElement && element.isConnected,
      hostContainsCanvas: host instanceof Element && (host === element || host.contains(element)),
      hostIsCanvas: host === element,
      hostTestId: host instanceof Element ? host.getAttribute("data-testid") : null,
      canvasTestId: element.getAttribute("data-testid"),
      canvasTagName: element.tagName
    };
  }, { host: MAIN_VIEWPORT_HOST_SELECTOR, canvas: MAIN_CANVAS_SELECTOR });
  if (identity.hostCount !== 1 || identity.canvasCount !== 1 || !identity.connected || !identity.hostContainsCanvas) {
    throw new Error(`main viewport canvas identity is unavailable or ambiguous: ${JSON.stringify(identity)}`);
  }
  const box = await locator.boundingBox();
  if (!box || ![box.x, box.y, box.width, box.height].every(Number.isFinite) || box.width <= 0 || box.height <= 0) {
    throw new Error(`connected main HTMLCanvasElement has no finite positive CSS bounding box: ${JSON.stringify(box)}`);
  }
  return { locator, box, identity };
}

export function treeRowTestId(ref: { type: string; id: string }, phase: BenchmarkPhase): string {
  return phase === "candidate"
    ? `tree-row-${encodeURIComponent(ref.type)}-${encodeURIComponent(ref.id)}`
    : `tree-row-${ref.id}`;
}

export function baselineOutsideFrustumRecord(sample: any, oracleProbe: any): any | null {
  if (oracleProbe?.baseline?.actionability !== "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM") return null;
  return { sample: sample.sample, probeAnchorRef: sample.probe_anchor_ref,
    status: "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM", durationMs: null,
    baselineNdc: oracleProbe.baseline.ndc, actionability: oracleProbe.baseline.actionability,
    oracleStatus: oracleProbe.baseline.oracle?.status ?? null,
    oracleReason: oracleProbe.baseline.oracle?.reason ?? null };
}

export function expectedVisibleEntityLabel(model: any, ref: { type: string; id: string }): string {
  const collections: Record<string, any[]> = {
    project: [model.project],
    material: model.materials ?? [],
    section: model.sections ?? [],
    node: model.nodes ?? [],
    pipe: model.pipe_segments ?? [],
    support: model.supports ?? [],
    component: model.components ?? [],
    load: model.load_cases ?? [],
    combination: model.combinations ?? []
  };
  const entity = collections[ref.type]?.find((entry) => entry.id === ref.id);
  if (!entity) throw new Error(`expected visible entity ${ref.type}:${ref.id} is absent from the fixture`);
  return entity.label ?? entity.name ?? entity.id;
}

export function pngPixelDigest(
  bytes: Buffer,
  selectionVisualOracle: SelectionVisualOracle | null = null,
  includePixels = false
): any {
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  if (bytes.length < 33 || !bytes.subarray(0, 8).equals(signature)) return { status: "FAIL_NOT_PNG" };
  let offset = 8;
  let ihdr: Buffer | null = null;
  const idat: Buffer[] = [];
  let ended = false;
  while (offset + 12 <= bytes.length) {
    const length = bytes.readUInt32BE(offset);
    const type = bytes.toString("ascii", offset + 4, offset + 8);
    const dataStart = offset + 8;
    const dataEnd = dataStart + length;
    if (dataEnd + 4 > bytes.length) return { status: "FAIL_TRUNCATED_PNG" };
    let crc = 0xffffffff;
    for (const byte of bytes.subarray(offset + 4, dataEnd)) { crc ^= byte; for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0); }
    if (((crc ^ 0xffffffff) >>> 0) !== bytes.readUInt32BE(dataEnd)) return { status: "FAIL_PNG_CRC" };
    if (type === "IHDR") { if (ihdr || offset !== 8) return { status: "FAIL_PNG_IHDR_ORDER" }; ihdr = bytes.subarray(dataStart, dataEnd); }
    if (type === "IDAT") idat.push(bytes.subarray(dataStart, dataEnd));
    offset = dataEnd + 4;
    if (type === "IEND") { ended = length === 0 && offset === bytes.length; break; }
  }
  if (!ended) return { status: "FAIL_PNG_END" };
  if (!ihdr || ihdr.length !== 13 || idat.length === 0) return { status: "FAIL_PNG_REQUIRED_CHUNKS_ABSENT" };
  const width = ihdr.readUInt32BE(0);
  const height = ihdr.readUInt32BE(4);
  const bitDepth = ihdr[8];
  const colorType = ihdr[9];
  const interlace = ihdr[12];
  const channels = colorType === 6 ? 4 : colorType === 2 ? 3 : colorType === 0 ? 1 : null;
  if (width < 1 || height < 1 || width > 32768 || height > 32768 || ihdr[10] !== 0 || ihdr[11] !== 0 || bitDepth !== 8 || interlace !== 0 || channels === null) {
    return { status: "FAIL_UNSUPPORTED_PNG_LAYOUT", width, height, bitDepth, colorType, interlace };
  }
  let filtered: Buffer;
  try { filtered = inflateSync(Buffer.concat(idat)); } catch { return { status: "FAIL_PNG_INFLATE" }; }
  const stride = width * channels;
  if (filtered.length !== height * (stride + 1)) {
    return { status: "FAIL_PNG_SCANLINE_LENGTH", width, height, channels, filteredBytes: filtered.length };
  }
  const pixels = Buffer.alloc(width * height * channels);
  const paeth = (a: number, b: number, c: number): number => {
    const p = a + b - c;
    const pa = Math.abs(p - a);
    const pb = Math.abs(p - b);
    const pc = Math.abs(p - c);
    return pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
  };
  for (let y = 0; y < height; y += 1) {
    const filter = filtered[y * (stride + 1)];
    for (let x = 0; x < stride; x += 1) {
      const raw = filtered[y * (stride + 1) + 1 + x];
      const outputIndex = y * stride + x;
      const left = x >= channels ? pixels[outputIndex - channels] : 0;
      const up = y > 0 ? pixels[outputIndex - stride] : 0;
      const upLeft = y > 0 && x >= channels ? pixels[outputIndex - stride - channels] : 0;
      const predictor = filter === 0 ? 0
        : filter === 1 ? left
          : filter === 2 ? up
            : filter === 3 ? Math.floor((left + up) / 2)
              : filter === 4 ? paeth(left, up, upLeft)
                : null;
      if (predictor === null) return { status: "FAIL_UNKNOWN_PNG_FILTER", filter, row: y };
      pixels[outputIndex] = (raw + predictor) & 0xff;
    }
  }
  let selectionColorLikeCount = 0;
  let selectionColorLikeCentralCount = 0;
  let localContrastQualifyingCount = 0;
  let maximumLocalRenderedContrastRatio = 0;
  const centerX = (width - 1) / 2;
  const centerY = (height - 1) / 2;
  const centralRadius = Math.min(width, height) * 0.25;
  const rgbAt = (x: number, y: number): [number, number, number] => {
    const index = (y * width + x) * channels;
    const red = pixels[index];
    return channels === 1 ? [red, red, red] : [red, pixels[index + 1], pixels[index + 2]];
  };
  const within = (actual: [number, number, number], expected: [number, number, number], tolerance: number): boolean =>
    actual.every((channel, index) => Math.abs(channel - expected[index]) <= tolerance);
  const luminance = (rgb: [number, number, number]): number => {
    const linear = rgb.map((channel) => {
      const value = channel / 255;
      return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
    });
    return linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722;
  };
  const contrast = (left: [number, number, number], right: [number, number, number]): number => {
    const l1 = luminance(left);
    const l2 = luminance(right);
    return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);
  };
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const rendered = rgbAt(x, y);
      const selectionLike = selectionVisualOracle !== null && within(rendered, selectionVisualOracle.srgb, selectionVisualOracle.tolerancePerChannel);
      if (!selectionLike) continue;
      selectionColorLikeCount += 1;
      if (Math.hypot(x - centerX, y - centerY) > centralRadius) continue;
      selectionColorLikeCentralCount += 1;
      if (!selectionVisualOracle?.sceneSrgb || !selectionVisualOracle.requiredLocalContrastRatio) continue;
      let bestNearbyContrast = 0;
      const directions = [[1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [1, -1], [-1, 1], [-1, -1]];
      for (const [dx, dy] of directions) {
        for (let distance = 2; distance <= 32; distance += 2) {
          const sx = x + dx * distance;
          const sy = y + dy * distance;
          if (sx < 0 || sy < 0 || sx >= width || sy >= height) break;
          const scenePixel = rgbAt(sx, sy);
          if (!within(scenePixel, selectionVisualOracle.sceneSrgb, selectionVisualOracle.tolerancePerChannel)) continue;
          bestNearbyContrast = Math.max(bestNearbyContrast, contrast(rendered, scenePixel));
          break;
        }
      }
      maximumLocalRenderedContrastRatio = Math.max(maximumLocalRenderedContrastRatio, bestNearbyContrast);
      if (bestNearbyContrast >= selectionVisualOracle.requiredLocalContrastRatio) localContrastQualifyingCount += 1;
    }
  }
  return {
    status: "PASS_DECODED_PIXEL_PAYLOAD",
    ...(includePixels ? { pixels } : {}),
    width,
    height,
    channels,
    pixelSha256: createHash("sha256").update(pixels).digest("hex"),
    selectionColorWitness: selectionVisualOracle === null ? null : {
      source: selectionVisualOracle.source,
      theme: selectionVisualOracle.theme,
      sourceBoundSelectionSrgb: selectionVisualOracle.srgb,
      tolerancePerChannel: selectionVisualOracle.tolerancePerChannel,
      count: selectionColorLikeCount,
      centralRadiusPixels: centralRadius,
      centralCount: selectionColorLikeCentralCount,
      minimumQualifyingInteriorPixels: selectionVisualOracle.minimumQualifyingInteriorPixels,
      sceneSrgb: selectionVisualOracle.sceneSrgb ?? null,
      requiredLocalContrastRatio: selectionVisualOracle.requiredLocalContrastRatio ?? null,
      localContrastQualifyingCount,
      maximumLocalRenderedContrastRatio
    }
  };
}

export const CUE_SOURCE_SHA256 = "00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931";
// V20S verified gesture-lifetime additions preserve primitive geometry; admit that exact source.
export const CUE_GEOMETRY_SOURCE_SHA256 = "c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e";
export function validateWinnerCuePlan(plan: any, probe: any): void {
  const { sha256, ...body } = plan ?? {};
  const same = (a: any, b: any) => JSON.stringify(a) === JSON.stringify(b);
  if (!plan || createHash("sha256").update(JSON.stringify(body)).digest("hex") !== sha256 || plan.schema !== "winner-cue-pair/v1" ||
      probe.candidate_runtime?.visual_plan?.sha256 !== sha256 || plan.sample !== probe.sample || !same(plan.expectedHitRef, probe.candidate_runtime?.oracle?.expectedHitRef) ||
      !same(plan.anchorRef, probe.probe_anchor_ref) || !same(plan.authoredAnchor, probe.authored_anchor) ||
      plan.source?.geometrySourceSha256 !== CUE_GEOMETRY_SOURCE_SHA256 || plan.source?.cueSourceSha256 !== CUE_SOURCE_SHA256 || plan.source?.visualTokensSha256 !== "009b27db887e3de224f72d0df221966fbaaa0305de218ee20848050ac6a1be1b" || plan.cue?.cssSize !== 11 || plan.cue.interior !== .34 || plan.cue.outer !== .5 || plan.cue.erosionDevice !== 1 ||
      !Number.isFinite(plan.dpr) || plan.dpr <= 0 || !Number.isSafeInteger(48 * plan.dpr) ||
      plan.clip?.width !== 48 || plan.clip?.height !== 48 || plan.clip.x !== Math.floor(plan.centerCss.x) - 24 || plan.clip.y !== Math.floor(plan.centerCss.y) - 24 ||
      plan.centerDevice?.x !== (plan.centerCss.x - plan.clip.x) * plan.dpr || plan.centerDevice?.y !== (plan.centerCss.y - plan.clip.y) * plan.dpr ||
      ![plan.centerCss.x, plan.centerCss.y, ...plan.authoredCenter].every(Number.isFinite)) throw new Error("invalid independent winner cue plan binding");
}
export function winnerCuePairs(plan: any): { interior: number; rim: number }[] {
  const size = 48 * plan.dpr, span = 11 * plan.dpr, c = plan.centerDevice;
  if (!Number.isSafeInteger(size) || size <= 0 || size > 768) throw new Error("unsafe cue PNG dimensions");
  const inside: any[] = [], rim: any[] = [];
  for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
    const l1 = Math.abs(x + .5 - c.x) + Math.abs(y + .5 - c.y), pixel = { x, y, l1, index: y * size + x };
    if (l1 <= .34 * span - 1) inside.push(pixel);
    if (l1 >= .34 * span + 1 && l1 <= .5 * span - 1) rim.push(pixel);
  }
  inside.sort((a, b) => b.l1 - a.l1 || a.y - b.y || a.x - b.x);
  const used = new Set<number>(), pairs: { interior: number; rim: number }[] = [];
  for (const a of inside) {
    const candidates = rim.filter((b) => !used.has(b.index) && (a.x - b.x) ** 2 + (a.y - b.y) ** 2 <= (.5 * span) ** 2)
      .sort((b, d) => ((a.x - b.x) ** 2 + (a.y - b.y) ** 2) - ((a.x - d.x) ** 2 + (a.y - d.y) ** 2) || b.y - d.y || b.x - d.x);
    if (candidates.length) { used.add(candidates[0].index); pairs.push({ interior: a.index, rim: candidates[0].index }); }
  }
  if (pairs.length < 4) throw new Error("conservative cue geometry has fewer than four pairs");
  return pairs;
}
export function pairedWinnerCueWitness(beforeBytes: Buffer, afterBytes: Buffer, plan: any, probe: any, oracle: any): any {
  try {
    validateWinnerCuePlan(plan, probe);
    if (oracle?.minimumQualifyingInteriorPixels !== 4 || oracle.requiredLocalContrastRatio !== 3 || oracle.sourceSha256 !== plan.source.visualTokensSha256 ||
        !Array.isArray(oracle.srgb) || JSON.stringify(oracle.srgb) !== JSON.stringify(oracle.theme === "light" ? [163, 68, 0] : oracle.theme === "dark" ? [240, 140, 34] : null) ||
        JSON.stringify(plan.cue.rimSrgb) !== JSON.stringify(oracle.theme === "light" ? [255, 255, 255] : [0, 0, 0]) || oracle.tolerancePerChannel !== 48) throw new Error("source visual token contract mismatch");
    const before = pngPixelDigest(beforeBytes, oracle, true), after = pngPixelDigest(afterBytes, oracle, true), size = 48 * plan.dpr;
    if (before.status !== "PASS_DECODED_PIXEL_PAYLOAD" || after.status !== "PASS_DECODED_PIXEL_PAYLOAD" || before.width !== size || after.width !== size || before.height !== size || after.height !== size || ![3, 4].includes(before.channels) || before.channels !== after.channels) throw new Error("invalid cue PNG layout/mapping");
    if (before.channels === 4 && [before, after].some((image) => image.pixels.some((v: number, i: number) => i % 4 === 3 && v !== 255))) throw new Error("nonopaque cue PNG");
    const rgb = (image: any, index: number) => [...image.pixels.subarray(index * image.channels, index * image.channels + 3)];
    const within = (a: number[], b: number[]) => a.every((v, i) => Math.abs(v - b[i]) <= oracle.tolerancePerChannel);
    const luminance = (a: number[]) => a.map((v) => v / 255).map((v) => v <= .04045 ? v / 12.92 : ((v + .055) / 1.055) ** 2.4).reduce((sum, v, i) => sum + v * [.2126, .7152, .0722][i], 0);
    const contrast = (a: number[], b: number[]) => { const x = luminance(a), y = luminance(b); return (Math.max(x, y) + .05) / (Math.min(x, y) + .05); };
    const facts = winnerCuePairs(plan).map((pair) => {
      const bi = rgb(before, pair.interior), br = rgb(before, pair.rim), ai = rgb(after, pair.interior), ar = rgb(after, pair.rim);
      const qualify = (i: number[], r: number[]) => within(i, oracle.srgb) && within(r, plan.cue.rimSrgb) && contrast(i, r) >= 3;
      return { ...pair, beforeInterior: bi, beforeRim: br, afterInterior: ai, afterRim: ar, beforeContrast: contrast(bi, br), afterContrast: contrast(ai, ar), newlyQualifying: qualify(ai, ar) && !qualify(bi, br) };
    });
    const count = facts.filter((p) => p.newlyQualifying).length, changed = before.pixelSha256 !== after.pixelSha256;
    delete before.pixels; delete after.pixels;
    return { status: changed && count >= 4 ? "PASS_PAIRED_WINNER_CUE_TRANSITION" : "FAIL_PAIRED_WINNER_CUE_TRANSITION", changed, count, minimum: 4, facts,
      before, after, planSha256: plan.sha256, netColorGrowthDiagnosticOnly: after.selectionColorWitness.count - before.selectionColorWitness.count };
  } catch (error) { return { status: "FAIL_CUE_PLAN_OR_PNG", error: String(error) }; }
}
function cueIdentity(snapshot: any) {
  return { model: snapshot.model, camera: snapshot.viewport.camera, canvas: snapshot.viewport.canvas, selection: snapshot.viewport.selection,
    inspector: snapshot.viewport.inspector, mainRender: snapshot.viewport.mainRender };
}
// Cross-action geometry may round differently after controls publish; capture-local identity stays exact.
export function cueCameraGeometryMatches(camera: any, frozen: any): boolean {
  const triple = (v: any) => Array.isArray(v) && v.length === 3 && v.every(Number.isFinite);
  const close = (a: any, b: any) => Number.isFinite(a) && Number.isFinite(b) && Math.abs(a - b) <= 1e-9;
  return !!camera && !!frozen && ["position", "target", "up"].every((key) => triple(camera[key]) && triple(frozen[key]) &&
    camera[key].every((v: number, i: number) => close(v, frozen[key][i]))) &&
    ["fovDegrees", "near", "far", "aspect"].every((key) => close(camera[key], frozen[key]));
}
export function cueCaptureHasNoDrift(before: any, after: any, identityBefore: any, identityAfter: any): boolean {
  return identityBefore?.valid === true && identityAfter?.valid === true &&
    JSON.stringify(identityBefore) === JSON.stringify(identityAfter) && JSON.stringify(cueIdentity(before)) === JSON.stringify(cueIdentity(after));
}
export async function captureWinnerCue(page: Page, probe: any, file: string): Promise<any> {
  const plan = probe.candidate_runtime?.visual_plan; validateWinnerCuePlan(plan, probe);
  const identity = () => page.evaluate(() => (globalThis as any).__uifHarness?.causal?.captureIdentity?.() ?? null);
  const identityBefore = await identity();
  const before = (await readCandidateDiagnostics(page, true)).snapshot, canvas = await requireUniqueConnectedMainCanvas(page);
  const dpr = await page.evaluate(() => devicePixelRatio);
  const close = (a: number, b: number) => Number.isFinite(a) && Number.isFinite(b) && Math.abs(a - b) <= 1e-9;
  const geometry = cueCameraGeometryMatches(before.viewport.camera, plan.camera) &&
    close(canvas.box.x, plan.canvas.cssLeft) && close(canvas.box.y, plan.canvas.cssTop) && close(canvas.box.width, plan.canvas.cssWidth) && close(canvas.box.height, plan.canvas.cssHeight) &&
    dpr === plan.dpr && before.viewport.canvas.bufferWidth === plan.canvas.bufferWidth && before.viewport.canvas.bufferHeight === plan.canvas.bufferHeight;
  await writeFile(`${file}.before.json`, `${JSON.stringify({ plan, before, canvas, dpr, geometry, identityBefore }, null, 2)}\n`, { flag: "wx" });
  if (!geometry) { await writeFile(`${file}.json`, `${JSON.stringify({ plan, before, canvas, dpr, geometry, identityBefore }, null, 2)}\n`, { flag: "wx" }); throw new Error("winner cue camera/canvas/DPR drift"); }
  await page.screenshot({ path: file, clip: plan.clip });
  const after = (await readCandidateDiagnostics(page, true)).snapshot, identityAfter = await identity();
  const noDrift = cueCaptureHasNoDrift(before, after, identityBefore, identityAfter);
  const record = { plan, before, after, canvas, dpr, geometry, noDrift, identityBefore, identityAfter, claim: "later bound visual capture, not marker-time framebuffer" };
  await writeFile(`${file}.json`, `${JSON.stringify(record, null, 2)}\n`, { flag: "wx" });
  if (!noDrift) throw new Error("winner cue capture state drift");
  return record;
}
export function cueActionBindingsStable(before: any, after: any): boolean {
  const a = before?.before, b = after?.before, ac = a?.viewport?.camera, bc = b?.viewport?.camera;
  const positive = (n: any) => Number.isSafeInteger(n) && n > 0;
  return before?.noDrift === true && after?.noDrift === true && before?.geometry === true && after?.geometry === true &&
    before?.plan?.sha256 === after?.plan?.sha256 && positive(a?.model?.generation) && a.model.generation === b?.model?.generation && JSON.stringify(a.model) === JSON.stringify(b.model) &&
    a?.viewport?.mainRender?.selectionPresentation?.resourceGeneration === b?.viewport?.mainRender?.selectionPresentation?.resourceGeneration &&
    positive(a?.viewport?.mainRender?.selectionPresentation?.resourceGeneration) &&
    positive(ac?.sequence) && positive(bc?.sequence) && bc.sequence >= ac.sequence && ac.kind === "perspective" && ac.kind === bc.kind &&
    Array.isArray(ac.localRenderOrigin) && ac.localRenderOrigin.length === 3 && ac.localRenderOrigin.every(Number.isFinite) &&
    JSON.stringify(ac.localRenderOrigin) === JSON.stringify(bc.localRenderOrigin) &&
    cueCameraGeometryMatches(ac, bc) && cueCameraGeometryMatches(ac, before.plan.camera) && cueCameraGeometryMatches(bc, after.plan.camera) &&
    before.identityBefore?.valid === true && after.identityBefore?.valid === true &&
    before.identityBefore.canvasEpoch === after.identityBefore.canvasEpoch && before.identityBefore.contextEpoch === after.identityBefore.contextEpoch &&
    JSON.stringify(a?.viewport?.canvas) === JSON.stringify(b?.viewport?.canvas) && before.dpr === after.dpr;
}
export function pointCaptureMatchesMarker(marker: any, capture: any, ref: any): boolean {
  const o = marker?.observed, snapshot = capture?.before, v = snapshot?.viewport, p = v?.mainRender?.selectionPresentation, first = o?.selectionPresentation;
  const positive = (n: any) => Number.isSafeInteger(n) && n > 0, nonnegative = (n: any) => Number.isSafeInteger(n) && n >= 0;
  const same = (a: any, b: any) => typeof a?.type === "string" && typeof a?.id === "string" && a.type === b?.type && a.id === b?.id;
  const singleton = (refs: any) => Array.isArray(refs) && refs.length === 1 && same(refs[0], ref);
  const m = o?.mainRenderSubmissionSequence, c = v?.mainRender?.submissionSequence;
  return capture?.noDrift === true && capture?.geometry === true && capture.identityBefore?.valid === true &&
    positive(o?.canvasEpoch) && positive(o?.contextEpoch) && o.canvasEpoch === capture.identityBefore.canvasEpoch && o.contextEpoch === capture.identityBefore.contextEpoch &&
    typeof marker?.token === "string" && marker.token === capture.identityBefore.token &&
    o?.pointerDown?.eventKind === "pointerdown" && Number.isFinite(o.pointerDown.pointerId) &&
    o.pointerDown.listenerObservedAt === o.capturedPointerDownListenerObservedAt &&
    JSON.stringify(o.pointerDown) === JSON.stringify(capture.identityBefore.pointerDown) &&
    positive(m) && positive(c) && c >= m && o.selectionRenderSubmissionSequence === m && first?.renderedSubmissionSequence === m &&
    positive(first?.resourceGeneration) && positive(first?.modelGeneration) && positive(first?.revision) &&
    nonnegative(first?.appliedAfterSubmissionSequence) && m > first.appliedAfterSubmissionSequence && singleton(first?.orderedRefs) &&
    first.modelGeneration === o.modelGeneration && first.resourceGeneration === p?.resourceGeneration && first.modelGeneration === p?.modelGeneration &&
    first.revision === p?.revision && first.appliedAfterSubmissionSequence === p?.appliedAfterSubmissionSequence && singleton(p?.orderedRefs) &&
    o.modelGeneration === snapshot?.model?.generation && v?.generation === o.modelGeneration && v?.mainRender?.generation === o.modelGeneration &&
    v?.selection?.generation === o.modelGeneration && v?.inspector?.generation === o.modelGeneration && o.inspectorGeneration === o.modelGeneration &&
    positive(o.actionSequence) && o.actionSequence === v?.selection?.actionSequence && positive(o.cameraSequence) && o.cameraSequence === v?.camera?.sequence &&
    v?.selection?.renderSubmissionSequence === c && p?.renderedSubmissionSequence === c &&
    o.selectionCount === 1 && singleton(o.orderedRefs) && singleton(v?.selection?.orderedRefs) &&
    same(o.selectionPrimaryRef, ref) && same(v?.selection?.primaryRef, ref) && same(o.inspectorRef, ref) && same(v?.inspector?.ref, ref) &&
    o.selectionInputKind === "pointer" && v?.selection?.inputKind === "pointer" &&
    Number.isFinite(o.selectionPointerDownAt) && Number.isFinite(o.capturedPointerDownListenerObservedAt) && o.selectionPointerDownAt >= o.capturedPointerDownListenerObservedAt &&
    o.selectionPointerDownAt === v?.selection?.pointerDownAt && Number.isFinite(o.selectionPublishedAt) && o.selectionPublishedAt >= o.selectionPointerDownAt &&
    o.selectionPublishedAt === v?.selection?.publishedAt;
}

async function captureVisibleLocatorEvidence(locator: Locator, expectedText: string, screenshotPath: string): Promise<any> {
  try {
    await locator.waitFor({ state: "visible", timeout: 2_000 });
    const text = (await locator.textContent())?.trim() ?? null;
    const box = await locator.boundingBox();
    if (!box) return { status: "FAIL_VISIBLE_LOCATOR_NO_BOX", expectedText, observedText: text };
    await locator.screenshot({ path: screenshotPath });
    const pixelDigest = pngPixelDigest(await readFile(screenshotPath));
    return {
      status: text === expectedText && pixelDigest.status === "PASS_DECODED_PIXEL_PAYLOAD"
        ? "PASS_EXACT_VISIBLE_TEXT_AND_DECODED_PNG"
        : "FAIL_EXACT_VISIBLE_TEXT_OR_PNG",
      expectedText,
      observedText: text,
      box,
      path: path.basename(screenshotPath),
      pixelDigest
    };
  } catch (error) {
    return { status: "FAIL_VISIBLE_LOCATOR_CAPTURE", expectedText, error: String(error) };
  }
}

export async function loadFixture(pipeCount: number): Promise<LoadedFixture> {
  const modelPath = path.join(root, "fixtures", `ui-foundation-${pipeCount}.model.json`);
  const samplesPath = path.join(root, "samples", `ui-foundation-${pipeCount}.interactions.json`);
  const candidateOracleDir = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
  const oraclePath = candidateOracleDir
    ? path.join(candidateOracleDir, `ui-foundation-${pipeCount}.candidate-runtime-point-oracle-v3.json`)
    : path.join(root, "samples", `ui-foundation-${pipeCount}.point-oracle-v3.json`);
  const readStart = nodePerformance.now();
  const bytes = await readFile(modelPath, "utf8");
  const readMs = nodePerformance.now() - readStart;
  const supportReadStart = nodePerformance.now();
  const [sampleBytes, oracleBytes] = await Promise.all([readFile(samplesPath, "utf8"), readFile(oraclePath, "utf8")]);
  const supportFilesReadMs = nodePerformance.now() - supportReadStart;
  const parseStart = nodePerformance.now();
  const model = JSON.parse(bytes);
  const parseMs = nodePerformance.now() - parseStart;
  const supportParseStart = nodePerformance.now();
  const samples = JSON.parse(sampleBytes);
  const pointOracle = JSON.parse(oracleBytes);
  const supportFilesParseMs = nodePerformance.now() - supportParseStart;
  return {
    bytes, model, samples, pointOracle, readMs, parseMs, supportFilesReadMs, supportFilesParseMs,
    modelPath, samplesPath, pointOraclePath: oraclePath,
    pointOracleSha256: createHash("sha256").update(oracleBytes).digest("hex")
  };
}

export async function validateCandidateOracleBinding(fixture: LoadedFixture, pipeCount: number): Promise<any> {
  const oracleDir = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
  const expectedManifestHash = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256;
  if (!oracleDir || !path.isAbsolute(oracleDir) || !expectedManifestHash) {
    throw new Error("candidate requires absolute UI_FOUNDATION_CANDIDATE_ORACLE_DIR and UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256");
  }
  const manifestBytes = await readFile(path.join(oracleDir, "CANDIDATE_POINT_ORACLE_MANIFEST.json"));
  const actualManifestHash = createHash("sha256").update(manifestBytes).digest("hex");
  if (actualManifestHash !== expectedManifestHash) throw new Error(`candidate oracle manifest hash mismatch: ${actualManifestHash}`);
  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  if (manifest.status !== "PASS_ALL_200_ACTIONABLE_AND_PRODUCT_PROJECTION_CROSSCHECKED") {
    throw new Error(`candidate oracle manifest is not qualified: ${manifest.status}`);
  }
  const entry = manifest.files.find((candidate: any) => candidate.pipeCount === pipeCount);
  if (!entry || entry.sha256 !== fixture.pointOracleSha256 || entry.actionableCount !== 200 ||
    entry.boxSampleCount !== 20 ||
    manifest.boxSelectionPolicySha256 !== "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310" ||
    path.basename(fixture.pointOraclePath) !== entry.path) {
    throw new Error(`candidate runtime oracle file is not bound for N=${pipeCount}`);
  }
  const names = ["freeze-candidate-point-oracle.mjs", "point-hit-oracle.mjs", "box-selection-oracle.mjs"];
  if (manifest.cueSourceSha256 !== CUE_SOURCE_SHA256 || manifest.dependencies?.length !== names.length) throw new Error("winner generator closure absent");
  for (const [sourcePath, declared, required] of [[manifest.cueSourcePath, manifest.cueSourceSha256, CUE_SOURCE_SHA256],
    [manifest.geometrySourcePath, manifest.geometrySourceSha256, CUE_GEOMETRY_SOURCE_SHA256]]) {
    if (typeof sourcePath !== "string" || !path.isAbsolute(sourcePath) || declared !== required ||
        createHash("sha256").update(await readFile(sourcePath)).digest("hex") !== required) throw new Error("winner product geometry/cue source drift");
  }
  for (const name of names) {
    const expected = manifest.dependencies.find((dependency: any) => dependency.path === name);
    if (!expected || createHash("sha256").update(await readFile(path.join(root, name))).digest("hex") !== expected.sha256) throw new Error(`winner generator dependency drift: ${name}`);
  }
  const modelSha = createHash("sha256").update(fixture.bytes).digest("hex"), samplesSha = createHash("sha256").update(await readFile(fixture.samplesPath)).digest("hex");
  for (const probe of fixture.pointOracle.probes) {
    const plan = probe.candidate_runtime?.visual_plan; validateWinnerCuePlan(plan, probe);
    if (plan.source.modelSha256 !== modelSha || plan.source.sampleSha256 !== samplesSha || plan.source.cameraPreflightSha256 !== fixture.pointOracle.candidate_preflight.preflightSha256 ||
        JSON.stringify(plan.source.dependencies) !== JSON.stringify(manifest.dependencies) || JSON.stringify(plan.camera) !== JSON.stringify(fixture.pointOracle.candidate_preflight.camera) ||
        JSON.stringify(plan.canvas) !== JSON.stringify(fixture.pointOracle.candidate_preflight.rawCanvasReadback)) throw new Error("winner visual plan provenance mismatch");
    winnerCuePairs(plan);
  }
  return { manifestSha256: actualManifestHash, entry, dependencies: manifest.dependencies };
}

export async function installInstrumentation(
  page: Page,
  options: { captureGlobalRaf?: boolean; causalFeedbackMarkers?: boolean; syntheticObserverSetupFault?: boolean; assignmentSpecification?: any } = {}
): Promise<void> {
  await page.addInitScript(({ captureGlobalRaf, causalFeedbackMarkers, syntheticObserverSetupFault,
    mainCanvasSelector, mainViewportHostSelector, pageClockSource, assignmentSpecification }) => {
    const originalRequestAnimationFrame = window.requestAnimationFrame;
    const originalCancelAnimationFrame = window.cancelAnimationFrame;
    const state: any = {
      rafRequested: 0,
      rafCompleted: 0,
      rafCancelled: 0,
      pendingRaf: new Set<number>(),
      captureFrames: false,
      frameTimes: [],
      eventStarts: {},
      activePointSample: null,
      fixtureModuleEvaluated: null,
      captureGlobalRaf,
      causalFeedbackMarkers,
      syntheticObserverSetupFault,
      causal: {
        schema: "openpipestress.ui-foundation.causal-feedback-marker/v1",
        canvasEpoch: 0,
        contextEpoch: 0,
        canvas: null,
        context: null,
        contextClearOwnDescriptor: null,
        contextClearOriginal: null,
        activeInvocation: null,
        active: null,
        nextRegistrationId: 1,
        nextInvocationId: 1,
        records: [],
        observerCostsMs: [],
        actionMarkerCostsMs: [],
        inputBoundaryCostsMs: [],
        observerSamples: [],
        rejections: [],
        overflow: { total: 0, byCollection: {} },
        errors: [],
        restored: false
      }
    };
    (globalThis as any).__uifHarness = state;
    const causal = state.causal;
    const boundedPush = (collection: any[], value: any, limit: number, collectionName: string, active = causal.active) => {
      if (collection.length >= limit) {
        causal.overflow.total += 1;
        causal.overflow.byCollection[collectionName] = (causal.overflow.byCollection[collectionName] ?? 0) + 1;
        if (active) {
          active.overflow.total += 1;
          active.overflow.byCollection[collectionName] = (active.overflow.byCollection[collectionName] ?? 0) + 1;
        }
        return false;
      }
      collection.push(value);
      return true;
    };
    const recordObserver = (operation: string, startedAt: number, active = causal.active, includeInTotal = true) => {
      const value = { operation, phase: active?.feedbackKind ?? "unarmed", startedAt,
        completedAt: 0, durationMs: 0, clockDifferenceCount: 1 };
      if (includeInTotal && active && !active.stopped) {
        boundedPush(causal.observerSamples, value, 24000, "observerSamples", active);
        if (active) boundedPush(active.observerSamples, value, 16000, "active.observerSamples", active);
      }
      // Charge record allocation, bounded pushes and overflow accounting. The
      // final timestamp assignments/return are the disclosed self-recording tail.
      value.completedAt = performance.now();
      value.durationMs = value.completedAt - startedAt;
      return value.durationMs;
    };
    const recordObserverError = (code: string, error: unknown, active = causal.active) => {
      const entry = { at: performance.now(), code, error: String(error) };
      boundedPush(causal.errors, entry, 100, "errors", active);
      if (active) boundedPush(active.observerErrors, entry, 100, "active.observerErrors", active);
    };
    const shortMarker = (kind: "ACTION" | "FEEDBACK", identity: string): string =>
      `UIF_CAUSAL_V1:${kind}:${identity}`;
    const currentCanvas = (): HTMLCanvasElement | null => {
      const hosts = document.querySelectorAll(mainViewportHostSelector);
      const canvases = document.querySelectorAll(mainCanvasSelector);
      if (hosts.length !== 1 || canvases.length !== 1) return null;
      const host = hosts[0];
      const canvas = canvases[0];
      return canvas instanceof HTMLCanvasElement && canvas.isConnected &&
        (host === canvas || host.contains(canvas)) ? canvas : null;
    };
    const restoreClear = () => {
      const context = causal.context;
      if (!context || !causal.contextClearOriginal) return;
      try {
        if (causal.contextClearOwnDescriptor) Object.defineProperty(context, "clear", causal.contextClearOwnDescriptor);
        else delete context.clear;
      } catch (error) {
        recordObserverError("CLEAR_RESTORE_FAILED", error);
      }
      causal.context = null;
      causal.contextClearOwnDescriptor = null;
      causal.contextClearOriginal = null;
    };
    const refreshCanvasEpoch = () => {
      const canvas = currentCanvas();
      if (canvas !== causal.canvas) {
        restoreClear();
        causal.canvas = canvas;
        causal.canvasEpoch += 1;
      }
      return { canvas, canvasEpoch: causal.canvasEpoch };
    };
    const installClearSentinel = (canvas: HTMLCanvasElement) => {
      // A canvas cannot replace its established context object. Reuse only for
      // the current exact canvas; loss is still checked at entry and return.
      const context = canvas === causal.canvas && causal.context
        ? causal.context : canvas.getContext("webgl2") ?? canvas.getContext("webgl");
      if (!context) return null;
      if (context !== causal.context) {
        restoreClear();
        const original = context.clear;
        const ownDescriptor = Object.getOwnPropertyDescriptor(context, "clear") ?? null;
        const wrappedClear = function (this: WebGLRenderingContext | WebGL2RenderingContext, mask: number) {
          const clearObserverStartedAt = performance.now();
          const invocation = causal.activeInvocation;
          if (invocation && this === context && (mask & context.COLOR_BUFFER_BIT) !== 0) {
            invocation.mainContextColorClearObserved = true;
          }
          const originalClearStartedAt = performance.now();
          try {
            return Reflect.apply(original, this, [mask]);
          } finally {
            const originalClearCompletedAt = performance.now();
            if (invocation) {
              invocation.clearCallCount += 1;
              invocation.clearProductWorkDurationMs += originalClearCompletedAt - originalClearStartedAt;
              const clearObserverCompletedAt = performance.now();
              invocation.clearObserverCostMs += (originalClearStartedAt - clearObserverStartedAt) +
                (clearObserverCompletedAt - originalClearCompletedAt);
            }
          }
        };
        try {
          Object.defineProperty(context, "clear", { configurable: true, writable: true, value: wrappedClear });
        } catch (error) {
          recordObserverError("CLEAR_SENTINEL_INSTALL_FAILED", error);
          return null;
        }
        causal.context = context;
        causal.contextClearOwnDescriptor = ownDescriptor;
        causal.contextClearOriginal = original;
        causal.contextEpoch += 1;
      }
      return { context, contextEpoch: causal.contextEpoch, lost: context.isContextLost() };
    };
    // Uses the same object epochs as the callback observer; no RAF, input, or readiness wait.
    // Establish before-action identity before arming, then reject any replacement through capture.
    causal.captureIdentity = () => {
      const { canvas, canvasEpoch } = refreshCanvasEpoch();
      const state = canvas instanceof HTMLCanvasElement ? installClearSentinel(canvas) : null;
      const actual = canvas?.getContext("webgl2") ?? canvas?.getContext("webgl");
      return { canvasEpoch, contextEpoch: causal.contextEpoch,
        token: causal.active?.token ?? null, pointerDown: causal.active?.pointerDown ? { ...causal.active.pointerDown, actionIdentity: { ...causal.active.pointerDown.actionIdentity } } : null,
        valid: canvas instanceof HTMLCanvasElement && canvas.isConnected && !!state && !state.lost && actual === causal.context };
    };
    const sameRef = (left: any, right: any) => left?.type === right?.type && left?.id === right?.id;
    const feedbackReady = (active: any, invocation: any): { ready: boolean; observed: any } => {
      if (!invocation.mainContextColorClearObserved) {
        return { ready: false, observed: { reason: "MAIN_CONTEXT_COLOR_CLEAR_NOT_OBSERVED" } };
      }
      const current = refreshCanvasEpoch();
      const contextLostAtReturn = invocation.context?.isContextLost?.() ?? true;
      if (!(invocation.canvas instanceof HTMLCanvasElement) || !invocation.canvas.isConnected || invocation.contextLost ||
          contextLostAtReturn || current.canvas !== invocation.canvas || invocation.canvas !== active.armedCanvas ||
          current.canvasEpoch !== invocation.canvasEpoch || invocation.canvasEpoch !== active.armedCanvasEpoch ||
          invocation.context !== causal.context || invocation.contextEpoch !== active.armedContextEpoch) {
        return { ready: false, observed: { reason: "STALE_REPLACED_DETACHED_OR_LOST_MAIN_CONTEXT",
          canvasConnectedAtReturn: invocation.canvas?.isConnected ?? false, contextLostAtReturn,
          currentCanvasEpoch: current.canvasEpoch, invocationCanvasEpoch: invocation.canvasEpoch,
          armedCanvasEpoch: active.armedCanvasEpoch, invocationContextEpoch: invocation.contextEpoch,
          armedContextEpoch: active.armedContextEpoch } };
      }
      if (active.phase === "candidate") {
        const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
        const expectation = active.candidateExpectation;
        const viewport = snapshot?.viewport;
        const mainRender = viewport?.mainRender;
        const boxNumericBinding = active.feedbackKind !== "box-selection" ||
          (Number.isSafeInteger(expectation.priorRenderSubmissionSequence) && expectation.priorRenderSubmissionSequence >= 0 &&
            Number.isSafeInteger(active.lastSubmissionSequence) && active.lastSubmissionSequence >= 0 &&
            Number.isSafeInteger(mainRender?.submissionSequence));
        const common = boxNumericBinding && snapshot?.model?.generation === expectation.modelGeneration &&
          viewport?.generation === expectation.modelGeneration &&
          mainRender?.generation === expectation.modelGeneration &&
          Number.isInteger(mainRender?.submissionSequence) &&
          mainRender.submissionSequence > Math.max(expectation.priorRenderSubmissionSequence, active.lastSubmissionSequence ?? -1);
        if (active.feedbackKind === "assignment") {
          const model = snapshot?.model, assignment = model?.assignment;
          const content = treeContent(active);
          const ready = model?.projectId === expectation.projectId && model?.identityHash === expectation.modelIdentityHash &&
            assignment?.status === "committed" && Number.isFinite(assignment?.startedAt) &&
            assignment.startedAt >= active.assignmentCalibration?.before && Number.isSafeInteger(model?.generation) &&
            model.generation > 0 && typeof model.indexGeneration === "string" && Number.isSafeInteger(model.projectSessionGeneration) &&
            mainRender?.generation === model.generation && Number.isSafeInteger(mainRender.submissionSequence) &&
            mainRender.submissionSequence > Math.max(0, active.lastSubmissionSequence ?? 0) &&
            (!active.actionMarker || assignment.startedAt === active.actionMarker.listenerObservedAt) &&
            mainRender.submittedAt >= assignment.startedAt && content.ready;
          if (ready && !active.actionMarker) active.actionMarker = { schema: causal.schema, kind: "ACTION", token: active.token,
            phase: active.phase, feedbackKind: active.feedbackKind, eventKind: "assignment", listenerObservedAt: assignment.startedAt,
            documentTimeOrigin: performance.timeOrigin, actionIdentity: active.actionIdentity, traceClock: active.assignmentCalibration };
          return { ready, observed: { modelGeneration: model?.generation, modelIdentityHash: model?.identityHash,
            indexGeneration: model?.indexGeneration, projectSessionGeneration: model?.projectSessionGeneration,
            assignmentStartedAt: assignment?.startedAt, mainRenderSubmissionSequence: mainRender?.submissionSequence, content } };
        }
        if (active.feedbackKind === "box-selection") {
          const box = viewport?.box, selection = viewport?.selection, inspector = viewport?.inspector;
          const presented = mainRender?.selectionPresentation;
          const scalar = (value: any) => typeof value === "string" ? value.slice(0, 160) :
            typeof value === "number" && !Number.isFinite(value) ? String(value) :
            value === null || typeof value === "number" || typeof value === "boolean" ? value : value === undefined ? "<undefined>" : "<non-scalar>";
          const refFact = (ref: any) => ref === null ? null : { type: scalar(ref?.type), id: scalar(ref?.id) };
          const sameNullable = (a: any, b: any) => a === null || b === null ? a === b :
            typeof a?.type === "string" && typeof a?.id === "string" && sameRef(a, b);
          const mismatch = (refs: any) => {
            if (!Array.isArray(refs) || !Array.isArray(expectation.orderedRefs)) return { unavailable: true };
            const index = expectation.orderedRefs.findIndex((ref: any, i: number) => !sameNullable(refs[i], ref));
            const first = index >= 0 ? index : refs.length !== expectation.orderedRefs.length ? expectation.orderedRefs.length : -1;
            return first < 0 ? null : { index: first, actual: refFact(refs[first]), expected: refFact(expectation.orderedRefs[first]) };
          };
          const boxMismatch = mismatch(box?.orderedRefs), selectionMismatch = mismatch(selection?.orderedRefs), renderedMismatch = mismatch(presented?.orderedRefs);
          const readouts = document.querySelectorAll('[data-testid="command-selection-readout"]');
          const headings = document.querySelectorAll('.workspace-pane-inspector .panel.inspector > h2');
          const visibleText = (nodes: NodeListOf<Element>) => nodes.length === 1 && nodes[0].isConnected &&
            (nodes[0] as HTMLElement).getClientRects().length > 0 && getComputedStyle(nodes[0]).visibility !== "hidden"
            ? nodes[0].textContent?.trim() ?? null : null;
          const readout = visibleText(readouts), inspectorHeading = visibleText(headings);
          const aggregates = document.querySelectorAll('[data-testid="aggregate-property-inspector"]');
          const primaryRows = aggregates.length === 1 ? Array.from(aggregates[0].querySelectorAll(':scope > dl > div'))
            .filter((row) => row.querySelector(':scope > dt')?.textContent?.trim() === "Primary") : [];
          const aggregatePrimary = aggregates.length === 1 && primaryRows.length === 1 && (aggregates[0] as HTMLElement).getClientRects().length > 0
            ? visibleText(primaryRows[0].querySelectorAll(':scope > dd')) : null;
          const identity = expectation.primaryRef ?? { type: "project", id: expectation.projectId };
          const readoutMatch = readout?.match(/^Selected ([^:;]+): (.+?); [0-9]+ queued$/);
          const checks = {
            common, generation: box?.generation === expectation.modelGeneration,
            priorSequence: Number.isSafeInteger(expectation.priorBoxActionSequence) && expectation.priorBoxActionSequence >= 0,
            advancingAction: Number.isSafeInteger(expectation.priorBoxActionSequence) && expectation.priorBoxActionSequence >= 0 &&
              Number.isSafeInteger(box?.actionSequence) && box.actionSequence > 0 && box.actionSequence > expectation.priorBoxActionSequence,
            direction: box?.direction === expectation.direction, filter: box?.filter === expectation.filter,
            render: Number.isSafeInteger(expectation.priorRenderSubmissionSequence) && expectation.priorRenderSubmissionSequence >= 0 &&
              Number.isSafeInteger(box?.renderSubmissionSequence) && box.renderSubmissionSequence > expectation.priorRenderSubmissionSequence &&
              Number.isSafeInteger(mainRender?.submissionSequence) && mainRender.submissionSequence >= box.renderSubmissionSequence,
            pointerUp: Number.isFinite(active.pointerUp?.listenerObservedAt) && Number.isFinite(box?.publishedAt) && box.publishedAt >= active.pointerUp.listenerObservedAt,
            selectionGeneration: selection?.generation === expectation.modelGeneration,
            selectionPublication: Number.isSafeInteger(selection?.actionSequence) && selection.actionSequence > 0 && Number.isFinite(selection?.publishedAt),
            selectionRender: Number.isSafeInteger(selection?.renderSubmissionSequence) && selection.renderSubmissionSequence > 0 && selection.renderSubmissionSequence === mainRender?.submissionSequence,
            inspectorGeneration: inspector?.generation === expectation.modelGeneration,
            inspectorPublication: Number.isSafeInteger(inspector?.publicationSequence) && inspector.publicationSequence > 0,
            boxOrderedRefs: boxMismatch === null, selectionOrderedRefs: selectionMismatch === null,
            boxPrimary: sameNullable(box?.primaryRef, expectation.primaryRef), selectionPrimary: sameNullable(selection?.primaryRef, expectation.primaryRef),
            inspector: sameNullable(inspector?.ref, identity),
            renderedResource: Number.isSafeInteger(expectation.resourceGeneration) && expectation.resourceGeneration > 0 && presented?.resourceGeneration === expectation.resourceGeneration,
            renderedModel: presented?.modelGeneration === expectation.modelGeneration,
            renderedRevision: Number.isSafeInteger(presented?.revision) && presented.revision > 0,
            renderedBarrier: Number.isSafeInteger(presented?.appliedAfterSubmissionSequence) && presented.appliedAfterSubmissionSequence >= 0 &&
              Number.isSafeInteger(presented?.renderedSubmissionSequence) && presented.renderedSubmissionSequence > presented.appliedAfterSubmissionSequence &&
              presented.renderedSubmissionSequence === mainRender?.submissionSequence,
            renderedOrderedRefs: renderedMismatch === null,
            readout: readoutMatch?.[1] === identity.type && readoutMatch?.[2] === identity.id,
            inspectorHeading: typeof expectation.expectedInspectorHeading === "string" && inspectorHeading === expectation.expectedInspectorHeading,
            aggregatePrimary: expectation.orderedRefs?.length > 1 ? aggregates.length === 1 && aggregatePrimary === `${identity.type}: ${identity.id}` : aggregates.length === 0
          };
          // This immutable verdict belongs only to this genuine callback. Never retain a
          // first-false (or first-true) exact comparison across invocations.
          const failedPredicates = Object.keys(checks).filter((key) => !checks[key as keyof typeof checks]);
          return { ready: failedPredicates.length === 0, observed: {
            reason: failedPredicates.length ? "BOX_SUBMISSION_NOT_READY" : "BOX_RENDERED_SELECTION_READY", checks, failedPredicates,
            snapshotSequence: scalar(snapshot?.snapshotSequence), capturedAt: scalar(snapshot?.capturedAt),
            priorKind: scalar(expectation.priorBoxBaseline?.kind), priorActionSequence: scalar(expectation.priorBoxActionSequence),
            modelGeneration: scalar(snapshot?.model?.generation), viewportGeneration: scalar(viewport?.generation),
            boxStatus: scalar(box?.status), boxGeneration: scalar(box?.generation), actionSequence: scalar(box?.actionSequence), cameraSequence: scalar(viewport?.camera?.sequence),
            boxRenderSubmissionSequence: scalar(box?.renderSubmissionSequence), mainRenderSubmissionSequence: scalar(mainRender?.submissionSequence),
            mainRenderGeneration: scalar(mainRender?.generation), mainRenderSubmittedAt: scalar(mainRender?.submittedAt),
            publishedAt: scalar(box?.publishedAt), pointerUpAt: scalar(active.pointerUp?.listenerObservedAt),
            selectionGeneration: scalar(selection?.generation), selectionActionSequence: scalar(selection?.actionSequence),
            selectionInputKind: scalar(selection?.inputKind), selectionPointerDownAt: scalar(selection?.pointerDownAt),
            selectionPublishedAt: scalar(selection?.publishedAt), selectionRenderSubmissionSequence: scalar(selection?.renderSubmissionSequence),
            inspectorGeneration: scalar(inspector?.generation), inspectorPublicationSequence: scalar(inspector?.publicationSequence), inspectorPublishedAt: scalar(inspector?.publishedAt),
            expectedBindings: { modelGeneration: scalar(expectation.modelGeneration), resourceGeneration: scalar(expectation.resourceGeneration),
              priorRenderSubmissionSequence: scalar(expectation.priorRenderSubmissionSequence), lastSubmissionSequence: scalar(active.lastSubmissionSequence) },
            inspectorRef: refFact(inspector?.ref), expectedInspector: refFact(identity), expectedPrimary: refFact(expectation.primaryRef),
            boxPrimary: refFact(box?.primaryRef), selectionPrimary: refFact(selection?.primaryRef),
            orderedLengths: { expected: scalar(expectation.orderedRefs?.length), box: scalar(box?.orderedRefs?.length), selection: scalar(selection?.orderedRefs?.length), rendered: scalar(presented?.orderedRefs?.length) },
            mismatches: { box: boxMismatch, selection: selectionMismatch, rendered: renderedMismatch },
            presentation: { resourceGeneration: scalar(presented?.resourceGeneration), modelGeneration: scalar(presented?.modelGeneration), revision: scalar(presented?.revision),
              appliedAfterSubmissionSequence: scalar(presented?.appliedAfterSubmissionSequence), renderedSubmissionSequence: scalar(presented?.renderedSubmissionSequence) },
            direction: scalar(box?.direction), filter: scalar(box?.filter), expectedDirection: scalar(expectation.direction), expectedFilter: scalar(expectation.filter),
            readout: scalar(readout), inspectorHeading: scalar(inspectorHeading), aggregateCount: aggregates.length, aggregatePrimary: scalar(aggregatePrimary), expectedInspectorHeading: scalar(expectation.expectedInspectorHeading),
            oracleSha256: scalar(expectation.oracleSha256)
          } };
        }
        // Orbit never consumes inspector text, readout strings or selected refs.
        if (active.feedbackKind === "orbit") {
          return { ready: common && viewport?.camera?.sequence > expectation.priorCameraSequence,
            observed: { modelGeneration: snapshot?.model?.generation ?? null,
              mainRenderSubmissionSequence: mainRender?.submissionSequence ?? null,
              cameraSequence: viewport?.camera?.sequence ?? null,
              canvasEpoch: invocation.canvasEpoch, contextEpoch: invocation.contextEpoch } };
        }
        if (active.feedbackKind !== "point-selection") return { ready: false, observed: { reason: "UNKNOWN_FEEDBACK_KIND" } };
        const selection = viewport?.selection;
        const inspector = viewport?.inspector;
        // Resolve the small identity surfaces once in this synchronous observation.
        // No retained DOM cache can survive a replacement; duplicates fail closed.
        const readouts = document.querySelectorAll('[data-testid="command-selection-readout"]');
        const headings = document.querySelectorAll('.workspace-pane-inspector .panel.inspector > h2');
        const readout = readouts.length === 1 && readouts[0].isConnected ? readouts[0].textContent?.trim() ?? null : null;
        const inspectorHeading = headings.length === 1 && headings[0].isConnected ? headings[0].textContent?.trim() ?? null : null;
        const readoutMatch = readout?.match(/^Selected ([^:;]+): (.+?);/);
        const orderedRefs = Array.isArray(selection?.orderedRefs) ? selection.orderedRefs.map((ref: any) => ({ type: ref.type, id: ref.id })) : null;
        const ready = common && selection?.actionSequence > expectation.priorActionSequence &&
          selection?.generation === expectation.modelGeneration &&
          selection?.orderedRefs?.length === 1 && sameRef(selection.orderedRefs[0], expectation.expectedRef) &&
          sameRef(selection.primaryRef, expectation.expectedRef) &&
          inspector?.generation === expectation.modelGeneration && sameRef(inspector?.ref, expectation.expectedRef) &&
          selection.inputKind === "pointer" && typeof selection.pointerDownAt === "number" &&
          typeof active.pointerDown?.listenerObservedAt === "number" &&
          selection.pointerDownAt >= active.pointerDown.listenerObservedAt &&
          typeof selection.publishedAt === "number" && selection.publishedAt >= selection.pointerDownAt &&
          Number.isInteger(selection.renderSubmissionSequence) && selection.renderSubmissionSequence > 0 &&
          mainRender.submissionSequence >= selection.renderSubmissionSequence &&
          readoutMatch?.[1] === expectation.expectedRef.type && readoutMatch?.[2] === expectation.expectedRef.id &&
          typeof expectation.expectedInspectorHeading === "string" && inspectorHeading === expectation.expectedInspectorHeading;
        return {
          ready,
          observed: {
            modelGeneration: snapshot?.model?.generation ?? null,
            mainRenderSubmissionSequence: mainRender?.submissionSequence ?? null,
            actionSequence: selection?.actionSequence ?? null,
            cameraSequence: viewport?.camera?.sequence ?? null,
            // Copy the already-read first application; later captures cannot backfill missing data.
            pointerDown: active.pointerDown ? { ...active.pointerDown, actionIdentity: { ...active.pointerDown.actionIdentity } } : null,
            selectionPresentation: mainRender?.selectionPresentation ? {
              resourceGeneration: mainRender.selectionPresentation.resourceGeneration,
              modelGeneration: mainRender.selectionPresentation.modelGeneration,
              revision: mainRender.selectionPresentation.revision,
              appliedAfterSubmissionSequence: mainRender.selectionPresentation.appliedAfterSubmissionSequence,
              renderedSubmissionSequence: mainRender.selectionPresentation.renderedSubmissionSequence,
              orderedRefs: Array.isArray(mainRender.selectionPresentation.orderedRefs)
                ? mainRender.selectionPresentation.orderedRefs.map((ref: any) => ({ type: ref?.type, id: ref?.id })) : null
            } : null,
            selectionCount: selection?.orderedRefs?.length ?? null,
            orderedRefs,
            selectionPrimaryRef: selection?.primaryRef ?? null,
            selectionInputKind: selection?.inputKind ?? null,
            selectionPointerDownAt: selection?.pointerDownAt ?? null,
            capturedPointerDownListenerObservedAt: active.pointerDown?.listenerObservedAt ?? null,
            selectionPublishedAt: selection?.publishedAt ?? null,
            selectionRenderSubmissionSequence: selection?.renderSubmissionSequence ?? null,
            inspectorGeneration: inspector?.generation ?? null,
            inspectorRef: inspector?.ref ?? null,
            readout,
            inspectorHeading,
            canvasEpoch: invocation.canvasEpoch,
            contextEpoch: invocation.contextEpoch
          }
        };
      }
      const expectation = active.baselineExpectation;
      if (active.feedbackKind === "orbit") return { ready: true,
        observed: { canvasEpoch: invocation.canvasEpoch, contextEpoch: invocation.contextEpoch } };
      const readout = document.querySelector('[data-testid="command-selection-readout"]')?.textContent?.trim() ?? null;
      const inspectorHeading = document.querySelector(".workspace-pane-inspector .panel.inspector > h2")?.textContent?.trim() ?? null;
      const ready = active.feedbackKind === "point-selection"
        ? readout?.includes(expectation.expectedRef.id) === true && readout?.includes(expectation.expectedRef.type) === true &&
          (inspectorHeading?.includes(expectation.expectedRef.id) === true || inspectorHeading === expectation.expectedInspectorHeading)
        : active.feedbackKind === "orbit";
      return {
        ready,
        observed: {
          readout,
          inspectorHeading,
          expectedRef: expectation?.expectedRef ?? null,
          canvasEpoch: invocation.canvasEpoch,
          contextEpoch: invocation.contextEpoch
        }
      };
    };
    const timestampWithClockBounds = (label: string) => {
      const before = performance.now();
      console.timeStamp(label);
      const after = performance.now();
      return { label, before, after, source: pageClockSource, crossOriginIsolated: globalThis.crossOriginIsolated };
    };
    const emitActionMarker = (eventKind: string, event: Event, target: HTMLElement | null) => {
      const active = causal.active;
      if (!causalFeedbackMarkers || !active || active.stopped || active.actionMarker || active.actionStartEvent !== eventKind) return;
      const observerStartedAt = performance.now();
      const testId = target?.closest?.("[data-testid]")?.getAttribute("data-testid") ?? null;
      const exactCanvas = target === active.armedCanvas && target instanceof HTMLCanvasElement && target.isConnected;
      if (testId !== active.expectedActionTargetTestId || !exactCanvas) {
        boundedPush(active.rejections, { at: performance.now(), code: "ACTION_TARGET_REJECTED", eventKind, testId,
          targetTag: target?.tagName ?? null, exactArmedCanvas: exactCanvas }, 100, "active.rejections", active);
        recordObserver("action-listener-rejected", observerStartedAt, active, false);
        return;
      }
      const listenerObservedAt = performance.now();
      const traceClock = timestampWithClockBounds(shortMarker("ACTION", active.token));
      active.actionMarker = {
        schema: causal.schema,
        kind: "ACTION",
        token: active.token,
        phase: active.phase,
        feedbackKind: active.feedbackKind,
        armedCanvasEpoch: active.armedCanvasEpoch,
        armedContextEpoch: active.armedContextEpoch,
        eventKind,
        testId,
        browserEventTimeStamp: event.timeStamp,
        listenerObservedAt,
        pointerId: event instanceof PointerEvent ? event.pointerId : null,
        clientX: event instanceof PointerEvent ? event.clientX : null,
        clientY: event instanceof PointerEvent ? event.clientY : null,
        targetTag: target?.tagName ?? null,
        actionIdentity: active.actionIdentity,
        traceClock
      };
      const cost = recordObserver("action-listener", observerStartedAt, active, false);
      boundedPush(causal.actionMarkerCostsMs, cost, 1200, "actionMarkerCostsMs", active);
      boundedPush(active.actionMarkerCostsMs, cost, 1200, "active.actionMarkerCostsMs", active);
    };
    const recordPointerTransactionBoundary = (eventKind: "pointerdown" | "pointerup", event: PointerEvent, target: HTMLElement | null) => {
      const active = causal.active;
      if (!causalFeedbackMarkers || !active || active.stopped) return;
      const observerStartedAt = performance.now();
      const testId = target?.closest?.("[data-testid]")?.getAttribute("data-testid") ?? null;
      const exactCanvas = target === active.armedCanvas && target instanceof HTMLCanvasElement && target.isConnected;
      if (testId !== active.expectedActionTargetTestId || !exactCanvas) {
        boundedPush(active.rejections, { at: performance.now(), code: "POINTER_BOUNDARY_TARGET_REJECTED", eventKind,
          testId, targetTag: target?.tagName ?? null, exactArmedCanvas: exactCanvas, pointerId: event.pointerId,
          clientX: event.clientX, clientY: event.clientY }, 100, "active.rejections", active);
        recordObserver("pointer-boundary-rejected", observerStartedAt, active, false);
        return;
      }
      const evidence = {
        eventKind,
        browserEventTimeStamp: event.timeStamp,
        listenerObservedAt: performance.now(),
        pointerId: event.pointerId,
        clientX: event.clientX,
        clientY: event.clientY,
        testId,
        targetTag: target?.tagName ?? null,
        actionIdentity: active.actionIdentity
      };
      if (eventKind === "pointerdown" && active.pointerDown === null) active.pointerDown = evidence;
      if (eventKind === "pointerup" && active.pointerUp === null) active.pointerUp = evidence;
      const cost = recordObserver(`pointer-${eventKind}`, observerStartedAt, active, false);
      boundedPush(causal.inputBoundaryCostsMs, cost, 2400, "inputBoundaryCostsMs", active);
      boundedPush(active.inputBoundaryCostsMs, cost, 2400, "active.inputBoundaryCostsMs", active);
    };
    const maybeEmitFeedbackMarker = (invocation: any) => {
      const active = causal.active;
      if (!causalFeedbackMarkers || !active || active.stopped ||
          (!active.actionMarker && active.feedbackKind !== "assignment")) return;
      const assignment = active.feedbackKind === "assignment";
      if (assignment && active.lastAssignmentFeedbackEpoch === active.domEpoch) return;
      if (!assignment && active.feedbackKind !== "orbit" && active.feedbackMarkers.length > 0) return;
      if (!assignment && active.feedbackMarkers.length >= active.maximumFeedbackMarkers) {
        active.stopped = true;
        active.stopReason = "MAXIMUM_FEEDBACK_MARKERS_REACHED";
        active.overflow.total += 1;
        active.overflow.byCollection.feedbackMarkers = (active.overflow.byCollection.feedbackMarkers ?? 0) + 1;
        return;
      }
      const verdict = feedbackReady(active, invocation);
      if (!verdict.ready) {
        const reason = verdict.observed?.reason ?? "CONTENT_NOT_READY";
        active.feedbackRejectionCounts[reason] = (active.feedbackRejectionCounts[reason] ?? 0) + 1;
        boundedPush(active.feedbackRejectionDetails, { at: performance.now(), invocationId: invocation.invocationId,
          reason, observed: verdict.observed }, 100, "active.feedbackRejectionDetails", active);
        return;
      }
      // An ineligible frame does not consume an epoch or overflow assignment history.
      if (assignment && active.feedbackMarkers.length >= active.maximumFeedbackMarkers) {
        active.stopped = true;
        active.stopReason = "MAXIMUM_FEEDBACK_MARKERS_REACHED";
        active.overflow.total += 1;
        active.overflow.byCollection.feedbackMarkers = (active.overflow.byCollection.feedbackMarkers ?? 0) + 1;
        return;
      }
      const markerIdentity = `${active.token}.${invocation.invocationId}`;
      const callbackCompletedAt = performance.now();
      const traceClock = timestampWithClockBounds(shortMarker("FEEDBACK", markerIdentity));
      const record = {
        schema: causal.schema,
        kind: "FEEDBACK",
        markerIdentity,
        token: active.token,
        phase: active.phase,
        feedbackKind: active.feedbackKind,
        invocationId: invocation.invocationId,
        registrationId: invocation.registrationId,
        parentInvocationId: invocation.parentInvocationId,
        canvasEpoch: invocation.canvasEpoch,
        contextEpoch: invocation.contextEpoch,
        callbackEntryAt: invocation.callbackEntryAt,
        callbackCompletedAt,
        domEpoch: active.domEpoch, documentTimeOrigin: performance.timeOrigin,
        mainContextColorClearObserved: invocation.mainContextColorClearObserved,
        clearSentinelObserverCostMs: invocation.clearObserverCostMs,
        clearProductWorkDurationMs: invocation.clearProductWorkDurationMs,
        clearCallCount: invocation.clearCallCount,
        observed: verdict.observed,
        traceClock
      };
      boundedPush(active.feedbackMarkers, record, active.maximumFeedbackMarkers, "active.feedbackMarkers", active);
      if (assignment) active.lastAssignmentFeedbackEpoch = active.domEpoch;
      active.lastSubmissionSequence = verdict.observed.mainRenderSubmissionSequence ?? active.lastSubmissionSequence;
      boundedPush(causal.records, record, 8192, "records", active);
    };
    const treeContent = (active: any) => {
      const expected = active.treeExpectation;
      const roots = document.querySelectorAll(".panel.model-tree");
      const root = roots.length === 1 ? roots[0] : null;
      const input = root?.querySelector('[data-testid="model-tree-filter-input"]') as HTMLInputElement | null;
      const tree = root?.querySelector('[data-testid="model-tree-virtual"]');
      const rows = Array.from(root?.querySelectorAll('[role="treeitem"]') ?? []);
      const summary = root?.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent?.trim();
      const ancestors = []; for (let e = root; e; e = e.parentElement) ancestors.push(e);
      const layoutSignature = ancestors.map((e) => `${e.tagName}|${e.className}|${e.getAttribute("style") ?? ""}`);
      const structurallyVisible = root && !root.closest('[hidden], [aria-hidden="true"], [inert]') && document.visibilityState === "visible" &&
        !ancestors.some((e) => e instanceof HTMLElement && (e.style.display === "none" || e.style.visibility === "hidden")) &&
        (!expected?.layoutSignature || JSON.stringify(layoutSignature) === JSON.stringify(expected.layoutSignature));
      let mismatch: number | null = null;
      rows.forEach((row, i) => {
        const e = expected?.rows?.[i];
        if (!e || row.getAttribute("data-testid") !== e.testId || Number(row.getAttribute("aria-posinset")) !== e.position ||
            Number(row.getAttribute("aria-setsize")) !== e.setSize || Number(row.getAttribute("aria-level")) !== e.level ||
            row.hasAttribute("disabled") || !row.isConnected ||
            (e.expanded === true && row.getAttribute("aria-expanded") !== "true") ||
            (e.label && (row.querySelector("strong")?.textContent?.trim() ?? row.textContent?.trim()) !== e.label)) mismatch ??= i;
      });
      const empty = expected?.visibleCount === 0;
      const ready = Boolean(expected && structurallyVisible && input?.isConnected && !input.disabled &&
        input.value === expected.query && (active.feedbackKind !== "tree-filter" || input === active.armedInput) && summary === `${expected.visibleCount} of ${expected.totalCount} model entities visible` &&
        (empty ? rows.length === 0 && root?.querySelector('[data-testid="model-tree-filter-empty"]')
          : tree && tree.scrollTop === 0 && rows.length === (expected.mountedCount ?? expected.rows.length) && rows.length > 0 && mismatch === null));
      return { ready, epoch: active.domEpoch, query: input?.value ?? null, mountedCount: rows.length,
        firstMismatch: mismatch, summary, layoutSignature, documentTimeOrigin: performance.timeOrigin };
    };
    const checkDomFeedback = () => {
      const active = causal.active;
      if (!active || active.stopped || active.feedbackKind !== "tree-filter" || !active.actionMarker) return;
      const callbackEntryAt = performance.now();
      const content = treeContent(active);
      if (content.ready && active.lastDomReadyEpoch !== active.domEpoch) {
        const invocationId = causal.nextInvocationId++;
        const markerIdentity = `${active.token}.${invocationId}`;
        const callbackCompletedAt = performance.now();
        const traceClock = timestampWithClockBounds(shortMarker("FEEDBACK", markerIdentity));
        const record = { schema: causal.schema, kind: "FEEDBACK", token: active.token, phase: active.phase,
          feedbackKind: active.feedbackKind, markerIdentity, invocationId, callbackEntryAt, callbackCompletedAt,
          domEpoch: active.domEpoch, documentTimeOrigin: performance.timeOrigin, traceClock,
          observationOperation: "dom-mutation-callback", observed: { modelGeneration: active.candidateExpectation.modelGeneration,
            modelIdentityHash: active.candidateExpectation.modelIdentityHash,
            indexGeneration: active.candidateExpectation.indexGeneration,
            projectSessionGeneration: active.candidateExpectation.projectSessionGeneration, content } };
        boundedPush(active.feedbackMarkers, record, active.maximumFeedbackMarkers, "active.feedbackMarkers", active);
        boundedPush(causal.records, record, 8192, "records", active);
        active.lastDomReadyEpoch = active.domEpoch;
      }
    };
    const processTreeMutations = (records: MutationRecord[], pendingAtStop = false) => {
      const active = causal.active;
      if (!active || active.stopped || !["assignment", "tree-filter"].includes(active.feedbackKind)) return;
      const relevant = records.filter((record) => {
        const target = record.target instanceof Element ? record.target : record.target.parentElement;
        return target?.closest(".model-tree") || target?.querySelector(".model-tree") ||
          [...record.addedNodes, ...record.removedNodes].some((node) => node instanceof Element &&
            (node.matches(".model-tree") || node.querySelector(".model-tree")));
      });
      if (!relevant.length) return;
      active.domEpoch += 1;
      boundedPush(active.domJournal, { epoch: active.domEpoch, at: performance.now(), recordCount: relevant.length,
        pendingAtStop }, 4096, "active.domJournal", active);
      if (!pendingAtStop) checkDomFeedback();
    };
    const treeObserver = new MutationObserver((records) => {
      const active = causal.active;
      if (!active || active.stopped || !["assignment", "tree-filter"].includes(active.feedbackKind)) return;
      const startedAt = performance.now();
      try { processTreeMutations(records); } catch (error) { recordObserverError("DOM_OBSERVER_FAILED", error); }
      finally { recordObserver("dom-mutation-callback", startedAt, active); }
    });
    if (causalFeedbackMarkers) treeObserver.observe(document, { subtree: true, childList: true, characterData: true, attributes: true });
    const stoppedContent = (active: any, pendingMutationCount: number) => {
      const content = treeContent(active);
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      const model = snapshot?.model, publication = snapshot?.viewport?.filter;
      const expected = active.candidateExpectation;
      const assignment = active.feedbackKind === "assignment";
      const modelMatches = model?.projectId === expected.projectId && model?.identityHash === expected.modelIdentityHash &&
        Number.isSafeInteger(model?.generation) && model.generation > 0 && snapshot?.tree?.generation === model.generation &&
        (assignment || (model.generation === expected.modelGeneration && model.indexGeneration === expected.indexGeneration &&
          model.projectSessionGeneration === expected.projectSessionGeneration));
      const publicationMatches = publication?.generation === model?.generation && publication?.query === active.treeExpectation.query &&
        publication?.visibleCount === active.treeExpectation.visibleCount && snapshot?.tree?.query === publication.query &&
        snapshot?.tree?.visibleCount === publication.visibleCount && Number.isSafeInteger(snapshot?.tree?.publicationSequence) &&
        Number.isFinite(publication?.publishedAt) && publication.publishedAt >= active.actionMarker?.listenerObservedAt;
      const actionMatches = assignment ? model?.assignment?.status === "committed" &&
        model.assignment.startedAt === active.actionMarker?.listenerObservedAt && publication?.inputAt === null && publication?.inputEventTimeStamp === null
        : active.inputEvents.length === 1 && publication?.actionSequence > expected.priorActionSequence &&
          publication?.inputAt >= active.actionMarker?.listenerObservedAt && publication?.inputEventTimeStamp === active.actionMarker?.browserEventTimeStamp;
      return { status: content.ready && modelMatches && publicationMatches && actionMatches && pendingMutationCount === 0
          ? "PASS_EXACT_STOPPED_CONTENT" : "FAIL_STOPPED_CONTENT", token: active.token, domEpoch: active.domEpoch,
        documentTimeOrigin: performance.timeOrigin, pendingMutationCount, interveningActionCount: Math.max(0, active.inputEvents.length - (assignment ? 0 : 1)),
        modelGeneration: model?.generation, modelIdentityHash: model?.identityHash, indexGeneration: model?.indexGeneration,
        projectSessionGeneration: model?.projectSessionGeneration, assignmentStartedAt: model?.assignment?.startedAt,
        assignmentCommitted: model?.assignment?.status === "committed", initialInputNull: publication?.inputAt === null && publication?.inputEventTimeStamp === null,
        responsiveTreeWitnessRequired: assignment, query: publication?.query, inputAt: publication?.inputAt,
        inputEventTimeStamp: publication?.inputEventTimeStamp, actionSequence: publication?.actionSequence,
        priorActionSequence: expected.priorActionSequence, content, snapshot };
    };
    causal.arm = (spec: any) => {
      if (!causalFeedbackMarkers) throw new Error("causal feedback marker instrumentation is disabled");
      if (causal.active && !causal.active.stopped) throw new Error("a causal feedback marker is already armed");
      if (!spec || typeof spec.token !== "string" || !/^[A-Za-z0-9._:-]{1,96}$/.test(spec.token) ||
          !["baseline", "candidate"].includes(spec.phase) ||
          !["point-selection", "box-selection", "orbit", "assignment", "tree-filter"].includes(spec.feedbackKind) ||
          !["pointerdown", "pointerup", "assignment", "input"].includes(spec.actionStartEvent) ||
          typeof spec.expectedActionTargetTestId !== "string" ||
          !spec.actionIdentity || typeof spec.actionIdentity !== "object" ||
          !Number.isInteger(spec.maximumFeedbackMarkers) || spec.maximumFeedbackMarkers < 1 || spec.maximumFeedbackMarkers > 4096) {
        throw new Error("invalid causal feedback marker specification");
      }
      const { canvas, canvasEpoch } = refreshCanvasEpoch();
      const nonPointer = spec.feedbackKind === "assignment" || spec.feedbackKind === "tree-filter";
      if (!nonPointer && (!(canvas instanceof HTMLCanvasElement) || !canvas.isConnected)) throw new Error("current main viewport canvas is unavailable");
      const contextState = canvas instanceof HTMLCanvasElement ? installClearSentinel(canvas) : null;
      if (!nonPointer && (!contextState || contextState.lost)) throw new Error("current main viewport WebGL context is unavailable or lost");
      const active = {
        ...spec,
        armedAt: performance.now(),
        documentTimeOrigin: performance.timeOrigin,
        evidenceEpoch: causal.evidenceEpoch ?? 0,
        domEpoch: 0, domJournal: [], contentProof: null, inputEvents: [],
        armedCanvas: canvas,
        armedCanvasEpoch: canvasEpoch,
        armedContextEpoch: contextState?.contextEpoch ?? null,
        actionMarker: null,
        actionIdentity: spec.actionIdentity,
        pointerDown: null,
        pointerUp: null,
        feedbackMarkers: [],
        observerSamples: [],
        actionMarkerCostsMs: [],
        inputBoundaryCostsMs: [],
        observerErrors: [],
        rejections: [],
        feedbackRejectionCounts: {},
        feedbackRejectionDetails: [],
        overflow: { total: 0, byCollection: {} },
        lastSubmissionSequence: spec.candidateExpectation?.priorRenderSubmissionSequence ?? null,
        stopped: false,
        stopReason: null
      };
      causal.active = active;
      if (spec.feedbackKind === "tree-filter") {
        active.armedInput = document.querySelector('[data-testid="model-tree-filter-input"]');
        if (!(active.armedInput instanceof HTMLInputElement) || !active.armedInput.isConnected) throw new Error("filter input unavailable");
      }
      return {
        token: active.token,
        phase: active.phase,
        feedbackKind: active.feedbackKind,
        armedCanvasEpoch: canvasEpoch,
        armedContextEpoch: contextState?.contextEpoch ?? null,
        armedAt: active.armedAt
      };
    };
    causal.stop = (token: string) => {
      if (causal.active?.token !== token) throw new Error("causal feedback marker token is not active");
      const active = causal.active;
      if (["assignment", "tree-filter"].includes(active.feedbackKind)) {
        const startedAt = performance.now();
        const pending = treeObserver.takeRecords();
        const priorEpoch = active.domEpoch;
        processTreeMutations(pending, true);
        active.contentProof = stoppedContent(active, active.domEpoch === priorEpoch ? 0 : pending.length);
        recordObserver("dom-stopped-reconciliation", startedAt, active);
      }
      causal.active.stopped = true;
      causal.active.stopReason = causal.active.stopReason ?? "EXPLICIT_STOP";
      return causal.read(token);
    };
    causal.read = (token: string) => {
      const active = causal.active?.token === token ? causal.active : null;
      return {
        active: active ? {
          token: active.token,
          phase: active.phase,
          feedbackKind: active.feedbackKind,
          actionStartEvent: active.actionStartEvent,
          expectedActionTargetTestId: active.expectedActionTargetTestId,
          armedAt: active.armedAt,
          armedCanvasEpoch: active.armedCanvasEpoch,
          armedContextEpoch: active.armedContextEpoch,
          actionMarker: active.actionMarker,
          actionIdentity: active.actionIdentity,
          documentTimeOrigin: active.documentTimeOrigin, evidenceEpoch: active.evidenceEpoch,
          domEpoch: active.domEpoch, domJournal: [...active.domJournal], inputEvents: [...active.inputEvents],
          contentProof: active.contentProof,
          pointerTransaction: {
            down: active.pointerDown,
            up: active.pointerUp
          },
          feedbackMarkers: [...active.feedbackMarkers],
          observerSamples: [...active.observerSamples],
          observerCostsMs: active.observerSamples.filter((sample: any) => sample.operation === "raf-callback").map((sample: any) => sample.durationMs),
          actionMarkerCostsMs: [...active.actionMarkerCostsMs],
          inputBoundaryCostsMs: [...active.inputBoundaryCostsMs],
          observerErrors: [...active.observerErrors],
          rejections: [...active.rejections],
          feedbackRejectionCounts: { ...active.feedbackRejectionCounts },
          feedbackRejectionDetails: [...active.feedbackRejectionDetails],
          overflow: { total: active.overflow.total, byCollection: { ...active.overflow.byCollection } },
          stopped: active.stopped,
          stopReason: active.stopReason
        } : null,
        observerCostsMs: [...causal.observerCostsMs],
        observerSamples: [...causal.observerSamples],
        overflow: { total: causal.overflow.total, byCollection: { ...causal.overflow.byCollection } },
        errors: [...causal.errors]
      };
    };
    causal.acknowledgeStopped = (token: string, persistedEvidenceSha256: string) => {
      if (!causal.active?.stopped || causal.active.token !== token || !/^[a-f0-9]{64}$/.test(persistedEvidenceSha256) ||
          causal.overflow.total || causal.errors.length) throw new Error("unacknowledged, active or invalid evidence cannot be drained");
      const drained = { token, persistedEvidenceSha256, epoch: causal.evidenceEpoch ?? 0,
        records: causal.records.length, observerSamples: causal.observerSamples.length };
      for (const key of ["records", "observerSamples", "observerCostsMs", "actionMarkerCostsMs", "inputBoundaryCostsMs", "rejections"]) causal[key] = [];
      causal.active = null;
      causal.evidenceEpoch = drained.epoch + 1;
      return drained;
    };
    causal.prepareAssignment = () => {
      if (!assignmentSpecification) return;
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      if (!snapshot || snapshot.model?.assignment?.status !== "idle") throw new Error("assignment pre-arm did not precede product assignment");
      causal.arm(assignmentSpecification);
      const active = causal.active;
      const startedAt = performance.now();
      active.assignmentCalibration = timestampWithClockBounds(shortMarker("ACTION", active.token));
      recordObserver("assignment-calibration", startedAt, active);
    };
    causal.restore = () => {
      if (causal.active && !causal.active.stopped) {
        causal.active.stopped = true;
        causal.active.stopReason = "RESTORED";
      }
      treeObserver.disconnect();
      restoreClear();
      window.requestAnimationFrame = originalRequestAnimationFrame;
      window.cancelAnimationFrame = originalCancelAnimationFrame;
      removeEventListener("pointerdown", pointerDownListener, true);
      removeEventListener("pointerup", pointerUpListener, true);
      removeEventListener("input", inputListener, true);
      causal.restored = true;
      return { status: causal.errors.length === 0 && causal.overflow.total === 0 ? "PASS_FULL_RESTORE" : "FAIL_RESTORE_OR_OBSERVER_STATE",
        restoredAt: performance.now(), pendingNativeHandlesUntouched: state.pendingRaf.size,
        listenersRemoved: ["pointerdown", "pointerup", "input"], overflow: causal.overflow, errors: [...causal.errors] };
    };
    if (captureGlobalRaf || causalFeedbackMarkers) {
      window.requestAnimationFrame = function (callback: FrameRequestCallback): number {
        const registrationStartedAt = performance.now();
        state.rafRequested += 1;
        const registrationId = causal.nextRegistrationId++;
        const parentInvocationId = causal.activeInvocation?.invocationId ?? null;
        let requestId = 0;
        const wrapped = function (this: unknown, timestamp: number) {
          const wrapperEntry = performance.now();
          state.pendingRaf.delete(requestId);
          state.rafCompleted += 1;
          if (state.captureFrames) state.frameTimes.push(timestamp);
          const priorInvocation = causal.activeInvocation;
          const activeForObserver = causal.active && !causal.active.stopped ? causal.active : null;
          let invocation: any = null;
          try {
            if (syntheticObserverSetupFault && activeForObserver?.actionMarker && !activeForObserver.syntheticSetupFaultConsumed) {
              activeForObserver.syntheticSetupFaultConsumed = true;
              throw new Error("SYNTHETIC_OBSERVER_SETUP_FAULT");
            }
            if (causalFeedbackMarkers && causal.active && !causal.active.stopped && !causal.active.actionMarker) {
              causal.active.feedbackRejectionCounts.RAF_WITHOUT_ACTION_MARKER =
                (causal.active.feedbackRejectionCounts.RAF_WITHOUT_ACTION_MARKER ?? 0) + 1;
            }
            if (causalFeedbackMarkers && causal.active && !causal.active.stopped &&
                (causal.active.actionMarker || causal.active.feedbackKind === "assignment")) {
              const { canvas, canvasEpoch } = refreshCanvasEpoch();
              if (canvas instanceof HTMLCanvasElement && canvas.isConnected) {
                const contextState = installClearSentinel(canvas);
                if (causal.active.feedbackKind === "assignment" && !causal.active.armedCanvas) {
                  causal.active.armedCanvas = canvas; causal.active.armedCanvasEpoch = canvasEpoch;
                  causal.active.armedContextEpoch = contextState?.contextEpoch ?? null;
                }
                invocation = {
                  invocationId: causal.nextInvocationId++, registrationId, parentInvocationId,
                  callbackEntryAt: performance.now(), canvas, canvasEpoch,
                  context: contextState?.context ?? null,
                  contextEpoch: contextState?.contextEpoch ?? null,
                  contextLost: contextState?.lost ?? true,
                  mainContextColorClearObserved: false,
                  clearObserverCostMs: 0,
                  clearProductWorkDurationMs: 0,
                  clearCallCount: 0
                };
              }
            }
          } catch (error) {
            recordObserverError("RAF_OBSERVER_SETUP_FAILED", error);
          }
          causal.activeInvocation = invocation;
          const observerBeforeOriginalCallback = performance.now();
          let completed = false;
          try {
            const result = Reflect.apply(callback, this, [timestamp]);
            completed = true;
            return result;
          } finally {
            const postStart = performance.now();
            causal.activeInvocation = priorInvocation;
            if (completed && invocation) {
              try {
                maybeEmitFeedbackMarker(invocation);
              } catch (error) {
                recordObserverError("FEEDBACK_MARKER_FAILED", error);
              }
            }
            if (activeForObserver) {
              const sample = { operation: "raf-callback", phase: activeForObserver.feedbackKind,
                startedAt: wrapperEntry, completedAt: 0, durationMs: 0,
                clockDifferenceCount: 2 + 2 * (invocation?.clearCallCount ?? 0),
                rawSlices: { wrapperEntry, observerBeforeOriginalCallback, postStart,
                  clearObserverCostMs: invocation?.clearObserverCostMs ?? 0 } };
              boundedPush(causal.observerSamples, sample, 24000, "observerSamples", activeForObserver);
              boundedPush(activeForObserver.observerSamples, sample, 16000, "active.observerSamples", activeForObserver);
              const costIndex = causal.observerCostsMs.length;
              const costRecorded = boundedPush(causal.observerCostsMs, 0, 8192, "observerCostsMs", activeForObserver);
              sample.completedAt = performance.now();
              sample.durationMs = (observerBeforeOriginalCallback - wrapperEntry) + (sample.completedAt - postStart) +
                (invocation?.clearObserverCostMs ?? 0);
              if (costRecorded) causal.observerCostsMs[costIndex] = sample.durationMs;

            }
          }
        };
        requestId = Reflect.apply(originalRequestAnimationFrame, this, [wrapped]);
        state.pendingRaf.add(requestId);
        recordObserver("raf-registration", registrationStartedAt);
        return requestId;
      };
      window.cancelAnimationFrame = function (requestId: number): void {
        const cancelStartedAt = performance.now();
        if (state.pendingRaf.delete(requestId)) state.rafCancelled += 1;
        try {
          return Reflect.apply(originalCancelAnimationFrame, this, [requestId]);
        } finally {
          recordObserver("raf-cancellation", cancelStartedAt);
        }
      };
    }
    const pointerDownListener = (event: PointerEvent) => {
      const observerStartedAt = performance.now();
      const target = event.target as HTMLElement | null;
      const listenerObservedAt = performance.now();
      state.eventStarts.pointerdown = {
        at: listenerObservedAt,
        listenerObservedAt,
        browserEventTimeStamp: event.timeStamp,
        clockMeaning: "capture-listener performance.now observation; browserEventTimeStamp retained separately",
        testId: target?.closest?.("[data-testid]")?.getAttribute("data-testid") ?? null,
        targetTag: target?.tagName ?? null,
        clientX: event.clientX,
        clientY: event.clientY,
        pointerId: event.pointerId,
        activePointSample: state.activePointSample
      };
      recordPointerTransactionBoundary("pointerdown", event, target);
      emitActionMarker("pointerdown", event, target);
      recordObserver("pointerdown-capture-listener-total", observerStartedAt);
    };
    const pointerUpListener = (event: PointerEvent) => {
      const observerStartedAt = performance.now();
      const target = event.target as HTMLElement | null;
      const listenerObservedAt = performance.now();
      state.eventStarts.pointerup = {
        at: listenerObservedAt,
        listenerObservedAt,
        browserEventTimeStamp: event.timeStamp,
        clockMeaning: "capture-listener performance.now observation; browserEventTimeStamp retained separately",
        testId: target?.closest?.("[data-testid]")?.getAttribute("data-testid") ?? null,
        targetTag: target?.tagName ?? null,
        clientX: event.clientX,
        clientY: event.clientY,
        pointerId: event.pointerId,
        activePointSample: state.activePointSample
      };
      recordPointerTransactionBoundary("pointerup", event, target);
      emitActionMarker("pointerup", event, target);
      recordObserver("pointerup-capture-listener-total", observerStartedAt);
    };
    const inputListener = (event: Event) => {
      const observerStartedAt = performance.now();
      const target = event.target as HTMLInputElement | null;
      state.eventStarts.input = {
        at: performance.now(),
        testId: target?.getAttribute?.("data-testid") ?? null,
        value: target?.value ?? null
      };
      const active = causal.active;
      if (active && !active.stopped && ["assignment", "tree-filter"].includes(active.feedbackKind)) {
        const observed = { eventKind: "input", isTrusted: event.isTrusted, listenerObservedAt: performance.now(),
          browserEventTimeStamp: event.timeStamp, query: target?.value ?? null, targetTestId: target?.getAttribute?.("data-testid"),
          inputType: (event as InputEvent).inputType, data: (event as InputEvent).data, isComposing: (event as InputEvent).isComposing };
        boundedPush(active.inputEvents, observed, 32, "active.inputEvents", active);
        if (active.feedbackKind === "tree-filter" && active.inputEvents.length === 1 && observed.isTrusted &&
            target === active.armedInput && observed.query === active.treeExpectation.query && !observed.isComposing) {
          active.actionMarker = { ...observed, schema: causal.schema, kind: "ACTION", token: active.token, phase: active.phase,
            feedbackKind: active.feedbackKind, documentTimeOrigin: performance.timeOrigin,
            actionIdentity: active.actionIdentity, traceClock: timestampWithClockBounds(shortMarker("ACTION", active.token)) };
        }
      }
      recordObserver("input-capture-listener-total", observerStartedAt);
    };
    addEventListener("pointerdown", pointerDownListener, true);
    addEventListener("pointerup", pointerUpListener, true);
    addEventListener("input", inputListener, true);
  }, {
    captureGlobalRaf: options.captureGlobalRaf === true,
    causalFeedbackMarkers: options.causalFeedbackMarkers === true,
    syntheticObserverSetupFault: options.syntheticObserverSetupFault === true,
    mainCanvasSelector: MAIN_CANVAS_SELECTOR,
    mainViewportHostSelector: MAIN_VIEWPORT_HOST_SELECTOR,
    pageClockSource: PAGE_CLOCK_SOURCE,
    assignmentSpecification: options.assignmentSpecification ?? null
  });
}

export async function armCausalFeedbackMarker(page: Page, specification: any): Promise<any> {
  return page.evaluate((specificationValue) => {
    const causal = (globalThis as any).__uifHarness?.causal;
    if (typeof causal?.arm !== "function") throw new Error("causal feedback marker instrumentation is unavailable");
    return causal.arm(specificationValue);
  }, specification);
}

export async function validateMainCanvasHitTarget(page: Page, clientPoint: { x: number; y: number }): Promise<any> {
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  return mainCanvas.locator.evaluate((canvas, point) => {
    const target = document.elementFromPoint(point.x, point.y);
    const causal = (globalThis as any).__uifHarness?.causal;
    const canvasEpoch = typeof causal?.canvasEpoch === "number" ? causal.canvasEpoch : null;
    const armedCanvasEpoch = typeof causal?.active?.armedCanvasEpoch === "number" ? causal.active.armedCanvasEpoch : null;
    const exactArmedCanvas = causal?.active ? causal.active.armedCanvas === canvas : null;
    const exactCanvasTarget = target === canvas;
    return {
      status: exactCanvasTarget && canvas.isConnected && (exactArmedCanvas === null || exactArmedCanvas)
        ? "PASS_ACTUAL_CONNECTED_MAIN_CANVAS_TARGET"
        : "FAIL_MAIN_CANVAS_TARGET",
      clientPoint: point,
      canvasEpoch,
      armedCanvasEpoch,
      exactArmedCanvas,
      canvasConnected: canvas.isConnected,
      exactCanvasTarget,
      targetTag: target?.tagName ?? null,
      targetTestId: target instanceof Element ? target.closest("[data-testid]")?.getAttribute("data-testid") ?? null : null,
      canvasRect: (() => {
        const rect = canvas.getBoundingClientRect();
        return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
      })()
    };
  }, clientPoint);
}

export async function readCausalFeedbackMarker(page: Page, token: string): Promise<any> {
  return page.evaluate((tokenValue) => {
    const causal = (globalThis as any).__uifHarness?.causal;
    if (typeof causal?.read !== "function") throw new Error("causal feedback marker instrumentation is unavailable");
    return causal.read(tokenValue);
  }, token);
}

export async function stopCausalFeedbackMarker(page: Page, token: string): Promise<any> {
  return page.evaluate((tokenValue) => {
    const causal = (globalThis as any).__uifHarness?.causal;
    if (typeof causal?.stop !== "function") throw new Error("causal feedback marker instrumentation is unavailable");
    return causal.stop(tokenValue);
  }, token);
}

export async function restoreCausalFeedbackInstrumentation(page: Page): Promise<any> {
  return page.evaluate(() => {
    const causal = (globalThis as any).__uifHarness?.causal;
    if (typeof causal?.restore !== "function") throw new Error("causal feedback marker instrumentation is unavailable");
    return causal.restore();
  });
}

export async function routeModelFixture(page: Page, fixture: Pick<LoadedFixture, "bytes">): Promise<void> {
  const moduleBody = [
    `const model=${fixture.bytes.trim()};`,
    "if(globalThis.__uifHarness){globalThis.__uifHarness.fixtureModuleEvaluated=performance.now();globalThis.__uifHarness.causal?.prepareAssignment?.();}",
    "export { model as default };"
  ].join("\n");
  await page.route(/(?:invented_preview_model-[^/]+\.js|fixtures\/product_preview\/invented_preview_model\.json)(?:\?.*)?$/, async (route) => {
    await route.fulfill({ status: 200, contentType: "application/javascript", body: moduleBody });
  });
}

export async function captureChromiumEnvironment(page: Page): Promise<any> {
  const browser = page.context().browser();
  const result: any = {
    browserVersion: browser?.version() ?? null,
    browserProtocolVersion: { status: "UNAVAILABLE" },
    browserCommandLine: { status: "UNAVAILABLE" },
    systemInfo: { status: "UNAVAILABLE" },
    processInfo: { status: "UNAVAILABLE" }
  };
  if (browser) {
    const browserSession = await browser.newBrowserCDPSession();
    const safeSend = async (method: string): Promise<any> => {
      try {
        return await browserSession.send(method as any);
      } catch (error) {
        return { status: "UNAVAILABLE", method, error: String(error) };
      }
    };
    result.browserProtocolVersion = await safeSend("Browser.getVersion");
    result.browserCommandLine = await safeSend("Browser.getBrowserCommandLine");
    result.systemInfo = await safeSend("SystemInfo.getInfo");
    result.processInfo = await safeSend("SystemInfo.getProcessInfo");
    await browserSession.detach();
  }
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  result.webgl = await mainCanvas.locator.evaluate((canvas) => {
    if (!(canvas instanceof HTMLCanvasElement)) throw new Error("bound main canvas is not an HTMLCanvasElement");
    const gl = canvas.getContext("webgl2") ?? canvas.getContext("webgl");
    if (!gl) return { status: "UNAVAILABLE" };
    const debug = gl.getExtension("WEBGL_debug_renderer_info");
    return {
      status: "AVAILABLE",
      vendor: gl.getParameter(gl.VENDOR),
      renderer: gl.getParameter(gl.RENDERER),
      unmaskedVendor: debug ? gl.getParameter(debug.UNMASKED_VENDOR_WEBGL) : null,
      unmaskedRenderer: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : null
    };
  });
  return result;
}

export async function captureBrowserProcessAndHeapState(page: Page): Promise<any> {
  const capturedAt = new Date().toISOString();
  const browser = page.context().browser();
  let chromiumProcessInfo: any = { status: "UNAVAILABLE" };
  if (browser) {
    try {
      const session = await browser.newBrowserCDPSession();
      chromiumProcessInfo = await session.send("SystemInfo.getProcessInfo");
      await session.detach();
    } catch (error) {
      chromiumProcessInfo = { status: "UNAVAILABLE", error: String(error) };
    }
  }
  const pids = Array.isArray(chromiumProcessInfo?.processInfo)
    ? [...new Set(chromiumProcessInfo.processInfo.map((entry: any) => Number(entry.id)).filter((id: number) => Number.isInteger(id) && id > 0))]
    : [];
  let operatingSystemProcessSnapshot: any = { status: "UNAVAILABLE", pids };
  if (pids.length) {
    try {
      const { stdout } = await execFileAsync("/bin/ps", ["-o", "pid=,rss=,etime=,comm=", "-p", pids.join(",")]);
      const rows = stdout.trim().split("\n").filter(Boolean).map((line) => {
        const match = line.trim().match(/^(\d+)\s+(\d+)\s+(\S+)\s+(.+)$/);
        return match ? { pid: Number(match[1]), rssKiB: Number(match[2]), elapsed: match[3], command: match[4] } : { raw: line };
      });
      operatingSystemProcessSnapshot = {
        status: "MEASURED_FOR_CHROMIUM_REPORTED_PROCESS_IDS",
        rows,
        rssKiBSum: rows.reduce((sum, row: any) => sum + (typeof row.rssKiB === "number" ? row.rssKiB : 0), 0),
        attributionLimitation: "SystemInfo-reported process IDs are scoped to the isolated launched browser, but RSS is an OS snapshot and is not a proof of exclusive GPU allocation ownership."
      };
    } catch (error) {
      operatingSystemProcessSnapshot = { status: "UNAVAILABLE", pids, error: String(error) };
    }
  }
  const javascriptHeap = await page.evaluate(() => {
    const memory = (performance as any).memory;
    return memory ? {
      status: "MEASURED_CHROMIUM_NONSTANDARD",
      usedJSHeapSize: memory.usedJSHeapSize,
      totalJSHeapSize: memory.totalJSHeapSize,
      jsHeapSizeLimit: memory.jsHeapSizeLimit
    } : { status: "N/A_PERFORMANCE_MEMORY_UNAVAILABLE" };
  });
  return {
    capturedAt,
    javascriptHeap,
    testWorkerProcessMemory: process.memoryUsage(),
    chromiumProcessInfo,
    operatingSystemProcessSnapshot
  };
}

async function afterNextRafOpportunity(page: Page): Promise<number> {
  return page.evaluate(async () => {
    await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
    return performance.now();
  });
}

export async function waitForFirstUsable(
  page: Page,
  fixture: LoadedFixture,
  timeoutMs: number,
  phase: BenchmarkPhase = "baseline"
): Promise<any> {
  const projectRowTestId = treeRowTestId({ type: "project", id: fixture.model.project.id }, phase);
  const projectRow = page.getByTestId(projectRowTestId);
  const canvas = page.getByTestId("viewport-canvas");
  const summary = page.getByTestId("model-tree-filter-summary");
  const treeCount = 1 + (fixture.model.materials?.length ?? 0) + (fixture.model.sections?.length ?? 0) +
    fixture.model.nodes.length + fixture.model.pipe_segments.length + fixture.model.supports.length +
    fixture.model.components.length + fixture.model.load_cases.length + (fixture.model.combinations?.length ?? 0) +
    (phase === "candidate" ? (fixture.model.diagnostics?.length ?? 0) : 0);
  await projectRow.waitFor({ state: "visible", timeout: timeoutMs });
  await canvas.waitFor({ state: "visible", timeout: timeoutMs });
  await page.waitForFunction(({ count, hostSelector, canvasSelector }) => {
    const value = document.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent ?? "";
    const hosts = document.querySelectorAll(hostSelector);
    const canvases = document.querySelectorAll(canvasSelector);
    return value.includes(`${count} of ${count}`) && hosts.length === 1 && canvases.length === 1 &&
      canvases[0] instanceof HTMLCanvasElement && canvases[0].isConnected &&
      (hosts[0] === canvases[0] || hosts[0].contains(canvases[0]));
  }, { count: treeCount, hostSelector: MAIN_VIEWPORT_HOST_SELECTOR, canvasSelector: MAIN_CANVAS_SELECTOR }, { timeout: timeoutMs });
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  let candidateAfterSelection: any = null;
  let readinessSelectionActions: any = null;
  if (phase === "candidate") {
    const candidateInitialSelection = (await readCandidateDiagnostics(page, true)).snapshot;
    const alternateRef = fixture.model.materials?.[0]
      ? { type: "material", id: fixture.model.materials[0].id }
      : { type: "node", id: fixture.model.nodes[0].id };
    const tree = page.getByTestId("model-tree-virtual");
    const projectRowDomId = treeRowTestId({ type: "project", id: fixture.model.project.id }, "candidate");
    const alternateRowDomId = treeRowTestId(alternateRef, "candidate");
    await tree.focus({ timeout: timeoutMs });
    await tree.press("Home", { timeout: timeoutMs });
    await page.waitForFunction((id) => document.querySelector('[data-testid="model-tree-virtual"]')?.getAttribute("aria-activedescendant") === id,
      projectRowDomId, { timeout: timeoutMs });
    await tree.press("ArrowDown", { timeout: timeoutMs });
    await tree.press("ArrowDown", { timeout: timeoutMs });
    await page.waitForFunction((id) => document.querySelector('[data-testid="model-tree-virtual"]')?.getAttribute("aria-activedescendant") === id,
      alternateRowDomId, { timeout: timeoutMs });
    await tree.press("Enter", { timeout: timeoutMs });
    const candidateAfterAlternate = await waitForCandidateExclusiveSelection(
      page,
      alternateRef,
      candidateInitialSelection.model.generation,
      candidateInitialSelection.viewport.selection.actionSequence,
      candidateInitialSelection.viewport.mainRender.submissionSequence,
      timeoutMs
    );
    await tree.press("Home", { timeout: timeoutMs });
    await page.waitForFunction((id) => document.querySelector('[data-testid="model-tree-virtual"]')?.getAttribute("aria-activedescendant") === id,
      projectRowDomId, { timeout: timeoutMs });
    await tree.press("Enter", { timeout: timeoutMs });
    candidateAfterSelection = await waitForCandidateExclusiveSelection(
      page,
      { type: "project", id: fixture.model.project.id },
      candidateAfterAlternate.model.generation,
      candidateAfterAlternate.viewport.selection.actionSequence,
      candidateAfterAlternate.viewport.mainRender.submissionSequence,
      timeoutMs
    );
    readinessSelectionActions = {
      measured: false,
      input: "real ARIA tree keyboard navigation and Enter activation",
      navigation: ["Home", "ArrowDown", "ArrowDown", "Enter", "Home", "Enter"],
      initial: candidateInitialSelection.viewport.selection,
      alternate: { ref: alternateRef, selection: candidateAfterAlternate.viewport.selection },
      project: { ref: { type: "project", id: fixture.model.project.id }, selection: candidateAfterSelection.viewport.selection }
    };
  } else {
    await projectRow.click({ timeout: timeoutMs });
    await page.waitForFunction(({ projectId, rowTestId }) =>
      (document.querySelector(`[data-testid="${CSS.escape(rowTestId)}"]`)?.getAttribute("aria-pressed") === "true") &&
      (document.querySelector('[data-testid="command-selection-readout"]')?.textContent ?? "").includes(projectId),
      { projectId: fixture.model.project.id, rowTestId: projectRowTestId },
      { timeout: timeoutMs }
    );
  }
  const nextRafOpportunityAt = await afterNextRafOpportunity(page);
  const baselineRecord = await page.evaluate(({ nextRafOpportunityAtValue, expectedTreeCount }) => {
    const state = (globalThis as any).__uifHarness;
    return {
      status: "PROXY_ONLY_EXACT_ASSIGNMENT_START_UNAVAILABLE",
      treeCount: expectedTreeCount,
      fixtureModuleEvaluated: state?.fixtureModuleEvaluated ?? null,
      nextRafOpportunityAt: nextRafOpportunityAtValue,
      proxyDurationMs: typeof state?.fixtureModuleEvaluated === "number" ? nextRafOpportunityAtValue - state.fixtureModuleEvaluated : null,
      limitation: "Fixture module evaluation occurs after object construction but before Promise.all resolves and app-level commitModel begins. The end is a next-RAF paint opportunity, not compositor proof. This baseline proxy cannot earn strict assignment PASS."
    };
  }, { nextRafOpportunityAtValue: nextRafOpportunityAt, expectedTreeCount: treeCount });
  if (phase === "baseline") return { ...baselineRecord, mainCanvasIdentity: mainCanvas.identity, mainCanvasCssBox: mainCanvas.box };
  return {
    status: "CANDIDATE_ASSIGNMENT_AND_RESPONSIVE_TREE_GENERATION_BOUND_AWAITING_BROWSER_CAPTURE",
    treeCount,
    fixtureModuleEvaluated: baselineRecord.fixtureModuleEvaluated,
    strictAssignmentStartedAt: candidateAfterSelection.model.assignment.startedAt,
    strictAssignmentCommittedAt: candidateAfterSelection.model.assignment.committedAt,
    modelGeneration: candidateAfterSelection.model.generation,
    tree: candidateAfterSelection.tree,
    mainRender: candidateAfterSelection.viewport.mainRender,
    responsiveTreeSelection: candidateAfterSelection.viewport.selection,
    readinessSelectionActions,
    mainCanvasIdentity: mainCanvas.identity,
    mainCanvasCssBox: mainCanvas.box,
    nextRafOpportunityAt,
    limitation: "The next RAF is only an app paint opportunity. The caller must retain a browser capture and use its post-capture page timestamp as the conservative presentation upper bound."
  };
}

export async function measureAccessibleLabelSelection(page: Page, entityId: string, timeoutMs: number): Promise<any> {
  const target = page.getByTestId(`viewport-select-${entityId}`);
  await target.click({ force: true, timeout: timeoutMs });
  await page.waitForFunction((id) => {
    const readout = document.querySelector('[data-testid="command-selection-readout"]')?.textContent ?? "";
    const active = document.querySelector(`[data-testid="viewport-select-${CSS.escape(id)}"]`)?.getAttribute("aria-pressed");
    return readout.includes(id) && active === "true";
  }, entityId, { timeout: timeoutMs });
  const endedAt = await afterNextRafOpportunity(page);
  return page.evaluate(({ id, end }) => {
    const start = (globalThis as any).__uifHarness?.eventStarts?.pointerdown;
    return {
      entityId: id,
      action: "viewport_accessible_label_activation",
      status: typeof start?.at === "number" ? "MEASURED_COMMON_ACTION_PROXY" : "MEASUREMENT_FAILED",
      startAt: start?.at ?? null,
      endAt: end,
      durationMs: typeof start?.at === "number" ? end - start.at : null,
      strictPointPickStatus: "N/A_BASELINE_LABEL_ACTIVATION_IS_NOT_CANVAS_RAYCAST",
      publication: "selection readout and active label observed, followed by a next-RAF paint opportunity; compositor presentation and renderer-highlight completion remain unproved baseline limits"
    };
  }, { id: entityId, end: endedAt });
}

export async function ensureViewportToggle(page: Page, testId: string, enabled: boolean, timeoutMs: number): Promise<void> {
  const toggle = page.getByTestId(testId);
  const expected = String(enabled);
  if (await toggle.getAttribute("aria-pressed") !== expected) await toggle.click({ timeout: timeoutMs });
  await page.waitForFunction(({ id, pressed }) =>
    document.querySelector(`[data-testid="${CSS.escape(id)}"]`)?.getAttribute("aria-pressed") === pressed,
    { id: testId, pressed: expected }, { timeout: timeoutMs });
}

export async function currentSelectionState(page: Page, expectedInspectorRef: { type: string; id: string } | null = null): Promise<any> {
  return page.evaluate((expected) => {
    const readout = document.querySelector('[data-testid="command-selection-readout"]')?.textContent?.trim() ?? null;
    const match = readout?.match(/^Selected ([^:;]+): (.+?);/);
    const inspector = document.querySelector('.workspace-pane-inspector .panel.inspector');
    const inspectorHeading = inspector?.querySelector(':scope > h2')?.textContent?.trim() ?? null;
    const inspectorContainsExpectedId = expected
      ? (inspector?.textContent ?? "").includes(expected.id)
      : null;
    return {
      readout,
      ref: match ? { type: match[1], id: match[2] } : null,
      inspectorHeading,
      inspectorContainsExpectedId
    };
  }, expectedInspectorRef);
}

export async function readCandidateDiagnostics(page: Page, requireFrozen = false): Promise<any> {
  return page.evaluate(({ checkFrozen }) => {
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, "__openPipeStressUiDiagnosticsV1");
    const surface = (globalThis as any).__openPipeStressUiDiagnosticsV1;
    if (!descriptor || descriptor.writable !== false || descriptor.configurable !== false ||
      !surface || surface.schema !== "openpipestress.ui-diagnostics/v1" ||
      typeof surface.readCurrent !== "function" || typeof surface.projectAuthoredPoint !== "function") {
      throw new Error("candidate diagnostics global is absent or violates the frozen attachment");
    }
    const snapshot = surface.readCurrent();
    const recursivelyFrozen = (value: unknown, seen = new Set<object>()): boolean => {
      if (value === null || typeof value !== "object") return true;
      const object = value as object;
      if (seen.has(object)) return true;
      seen.add(object);
      if (!Object.isFrozen(object)) return false;
      return Reflect.ownKeys(object).every((key) => recursivelyFrozen((object as any)[key], seen));
    };
    if (checkFrozen && (!Object.isFrozen(surface) || !recursivelyFrozen(snapshot))) {
      throw new Error("candidate diagnostics surface or snapshot is not recursively frozen");
    }
    return {
      descriptor: { writable: descriptor.writable, configurable: descriptor.configurable },
      surfaceFrozen: Object.isFrozen(surface),
      snapshotRecursivelyFrozen: checkFrozen ? recursivelyFrozen(snapshot) : null,
      snapshot
    };
  }, { checkFrozen: requireFrozen });
}

export async function projectCandidateAuthoredPoint(
  page: Page,
  request: { modelGeneration: number; cameraSequence: number; authoredPoint: { x: number; y: number; z: number } }
): Promise<any> {
  return page.evaluate((projectionRequest) => {
    const surface = (globalThis as any).__openPipeStressUiDiagnosticsV1;
    if (!surface || typeof surface.projectAuthoredPoint !== "function") {
      throw new Error("candidate diagnostics projection surface is unavailable");
    }
    const result = surface.projectAuthoredPoint(projectionRequest);
    const recursivelyFrozen = (value: unknown, seen = new Set<object>()): boolean => {
      if (value === null || typeof value !== "object") return true;
      const object = value as object;
      if (seen.has(object)) return true;
      seen.add(object);
      return Object.isFrozen(object) && Reflect.ownKeys(object).every((key) => recursivelyFrozen((object as any)[key], seen));
    };
    if (!recursivelyFrozen(result)) throw new Error("candidate projection result is not recursively frozen");
    const keys = Reflect.ownKeys(result).map(String).sort();
    const expectedKeys = result.status === "available"
      ? ["cameraSequence", "canvasCss", "canvasCssPoint", "canvasDevice", "clip", "insideCanvasCss", "insideClosedNdc", "localPoint", "modelGeneration", "ndc", "status"].sort()
      : result.status === "stale"
        ? ["current", "requested", "status"].sort()
        : result.status === "invalid"
          ? ["reason", "status"].sort()
          : [];
    if (expectedKeys.length === 0 || JSON.stringify(keys) !== JSON.stringify(expectedKeys)) {
      throw new Error(`candidate projection result union drift: ${JSON.stringify(keys)}`);
    }
    return result;
  }, request);
}

export async function validateCandidateMeasuredCameraBinding(page: Page, pointOracle: any): Promise<any> {
  const snapshot = (await readCandidateDiagnostics(page, true)).snapshot;
  const expectedCamera = pointOracle?.candidate_preflight?.rawCameraReadback;
  const expectedCanvas = pointOracle?.candidate_preflight?.rawCanvasReadback;
  if (!expectedCamera || !expectedCanvas) throw new Error("candidate runtime oracle lacks frozen camera/canvas readback");
  const actualCamera = snapshot.viewport.camera;
  const actualCanvas = snapshot.viewport.canvas;
  const numericDiffs: Array<{ field: string; actual: number; expected: number; tolerance: number }> = [];
  const compare = (field: string, actual: unknown, expected: unknown, tolerance: number) => {
    const left = Number(actual);
    const right = Number(expected);
    if (!Number.isFinite(left) || !Number.isFinite(right) || Math.abs(left - right) > tolerance) {
      numericDiffs.push({ field, actual: left, expected: right, tolerance });
    }
  };
  for (const [index, field] of ["x", "y", "z"].entries()) {
    compare(`camera.position.${field}`, actualCamera.position?.[index], expectedCamera.position?.[index], 1e-9);
    compare(`camera.target.${field}`, actualCamera.target?.[index], expectedCamera.target?.[index], 1e-9);
    compare(`camera.up.${field}`, actualCamera.up?.[index], expectedCamera.up?.[index], 1e-9);
    compare(`camera.localRenderOrigin.${field}`, actualCamera.localRenderOrigin?.[index], expectedCamera.localRenderOrigin?.[index], 1e-9);
  }
  for (const field of ["fovDegrees", "near", "far", "aspect"]) {
    compare(`camera.${field}`, actualCamera[field], expectedCamera[field], 1e-12);
  }
  for (const field of ["cssWidth", "cssHeight", "dpr", "bufferWidth", "bufferHeight"]) {
    compare(`canvas.${field}`, actualCanvas[field], expectedCanvas[field], field.startsWith("css") ? 1e-6 : 0);
  }
  if (numericDiffs.length) throw new Error(`measured candidate camera/canvas drifted from frozen preflight: ${JSON.stringify(numericDiffs)}`);

  const crosschecks = [];
  for (const probe of pointOracle.probes) {
    const expected = probe.candidate_runtime;
    const result = await projectCandidateAuthoredPoint(page, {
      modelGeneration: snapshot.model.generation,
      cameraSequence: actualCamera.sequence,
      authoredPoint: { x: probe.authored_anchor[0], y: probe.authored_anchor[1], z: probe.authored_anchor[2] }
    });
    const mismatch = result.status !== "available" || !result.insideClosedNdc || !result.insideCanvasCss ||
      Math.abs(Number(result.ndc?.x) - Number(expected?.ndc?.[0])) > 1e-9 ||
      Math.abs(Number(result.ndc?.y) - Number(expected?.ndc?.[1])) > 1e-9;
    crosschecks.push({ sample: probe.sample, status: result.status, actionable: !mismatch, ndc: result.status === "available" ? result.ndc : null });
  }
  const failures = crosschecks.filter((entry) => !entry.actionable);
  if (failures.length) throw new Error(`measured candidate camera has ${failures.length}/200 non-actionable or mismatched probes`);
  return {
    status: "PASS_FROZEN_CAMERA_CANVAS_AND_ALL_200_PRODUCT_PROJECTIONS_MATCH",
    modelGeneration: snapshot.model.generation,
    cameraSequence: actualCamera.sequence,
    camera: actualCamera,
    canvas: actualCanvas,
    actionableCount: crosschecks.length,
    crosscheckSha256: createHash("sha256").update(JSON.stringify(crosschecks)).digest("hex"),
    numericDiffs
  };
}

export async function waitForCandidateExclusiveSelection(
  page: Page,
  expectedRef: { type: string; id: string },
  modelGeneration: number,
  previousActionSequence: number,
  previousRenderSubmissionSequence: number,
  timeoutMs: number,
  allowPreexistingExact = false
): Promise<any> {
  await page.waitForFunction(({ expected, generation, priorAction, priorRender, allowPreexisting }) => {
    const surface = (globalThis as any).__openPipeStressUiDiagnosticsV1;
    const snapshot = surface?.readCurrent?.();
    const viewport = snapshot?.viewport;
    const selection = viewport?.selection;
    const inspector = viewport?.inspector;
    const render = viewport?.mainRender;
    const refs = selection?.orderedRefs;
    const exactRef = (actual: any) => actual?.type === expected.type && actual?.id === expected.id;
    const exactSelection = Array.isArray(refs) && refs.length === 1 && exactRef(refs[0]) && exactRef(selection?.primaryRef) &&
      inspector?.generation === generation && exactRef(inspector.ref);
    const validPublicationClock = typeof selection?.publishedAt === "number" &&
      (typeof selection.pointerDownAt === "number"
        ? selection.publishedAt >= selection.pointerDownAt
        : selection.inputKind === "external" && selection.pointerDownAt === null);
    const advancedPublication = selection?.actionSequence > priorAction && validPublicationClock &&
      Number.isInteger(selection.renderSubmissionSequence) &&
      render?.generation === generation &&
      render?.submissionSequence > priorRender &&
      render.submissionSequence >= selection.renderSubmissionSequence &&
      typeof render.submittedAt === "number";
    return snapshot?.model?.generation === generation &&
      viewport?.generation === generation &&
      selection?.generation === generation &&
      exactSelection && (advancedPublication || allowPreexisting);
  }, {
    expected: expectedRef,
    generation: modelGeneration,
    priorAction: previousActionSequence,
    priorRender: previousRenderSubmissionSequence,
    allowPreexisting: allowPreexistingExact
  }, { timeout: timeoutMs });
  return (await readCandidateDiagnostics(page)).snapshot;
}

export async function capturePointInspectorEvidence(page: Page, phase: BenchmarkPhase, expectedLabel: string,
  expectedRef: { type: string; id: string }, screenshotPath: string): Promise<any> {
  const locator = phase === "baseline"
    ? page.locator(".workspace-pane-inspector .panel.inspector > h2").first()
    : page.getByTestId("property-inspector").locator(":scope > h2").first();
  const heading = phase === "baseline" ? expectedLabel : `${expectedLabel} — ${expectedRef.type}: ${expectedRef.id}`;
  return captureVisibleLocatorEvidence(locator, heading, screenshotPath);
}

export async function measureCanvasPointSelection(
  page: Page,
  sample: any,
  oracleProbe: any,
  timeoutMs: number,
  screenshotPath: string,
  phase: BenchmarkPhase,
  fixtureProjectId: string,
  expectedVisibleLabel: string,
  selectionVisualOracle: SelectionVisualOracle,
  causalToken: string | null = null
): Promise<any> {
  const probe = phase === "baseline" ? oracleProbe.baseline : oracleProbe.candidate_runtime;
  if (!probe || probe.actionability !== (phase === "baseline" ? "ACTIONABLE_IN_BASELINE_FRUSTUM" : "ACTIONABLE_IN_CANDIDATE_FRUSTUM")) {
    return {
      sample: sample.sample,
      probeAnchorRef: sample.probe_anchor_ref,
      status: phase === "baseline" ? "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM" : "NOT_ATTEMPTED_CANDIDATE_PREFLIGHT_NOT_ACTIONABLE",
      durationMs: null,
      ndc: probe?.ndc ?? null,
      expectedHitRef: probe?.oracle?.expectedHitRef ?? null,
      detail: probe?.oracle?.reason ?? "Candidate runtime oracle preflight is absent or not actionable."
    };
  }
  const expectedHitRef = probe.oracle?.expectedHitRef;
  if (!expectedHitRef) throw new Error(`actionable point sample ${sample.sample} has no expected_hit_ref`);
  const candidateBeforeReset = phase === "candidate" ? (await readCandidateDiagnostics(page)).snapshot : null;
  const resetStartedAt = await page.evaluate(() => performance.now());
  const projectRowTestId = treeRowTestId({ type: "project", id: fixtureProjectId }, phase);
  await page.getByTestId(projectRowTestId).click({ timeout: timeoutMs });
  let resetCandidateSnapshot: any = null;
  if (phase === "candidate") {
    resetCandidateSnapshot = await waitForCandidateExclusiveSelection(
      page,
      { type: "project", id: fixtureProjectId },
      candidateBeforeReset.model.generation,
      candidateBeforeReset.viewport.selection.actionSequence,
      candidateBeforeReset.viewport.mainRender.submissionSequence,
      timeoutMs,
      Array.isArray(candidateBeforeReset.viewport.selection.orderedRefs) &&
        candidateBeforeReset.viewport.selection.orderedRefs.length === 1 &&
        candidateBeforeReset.viewport.selection.orderedRefs[0].type === "project" &&
        candidateBeforeReset.viewport.selection.orderedRefs[0].id === fixtureProjectId &&
        candidateBeforeReset.viewport.selection.primaryRef?.type === "project" &&
        candidateBeforeReset.viewport.selection.primaryRef?.id === fixtureProjectId
    );
  } else {
    await page.waitForFunction(({ projectId, rowTestId }) => {
      const row = document.querySelector(`[data-testid="${CSS.escape(rowTestId)}"]`);
      const readout = document.querySelector('[data-testid="command-selection-readout"]')?.textContent?.trim() ?? "";
      const match = readout.match(/^Selected ([^:;]+): (.+?);/);
      return row?.getAttribute("aria-pressed") === "true" && match?.[1] === "project" && match?.[2] === projectId;
    }, { projectId: fixtureProjectId, rowTestId: projectRowTestId }, { timeout: Math.min(timeoutMs, 5_000) });
  }
  const resetNextRafOpportunityAt = await afterNextRafOpportunity(page);
  const before = await currentSelectionState(page);
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  const box = mainCanvas.box;
  const candidateBeforePoint = phase === "candidate" ? resetCandidateSnapshot : null;
  if (phase === "candidate") {
    await mkdir(path.dirname(screenshotPath), { recursive: true });
    await writeFile(`${screenshotPath}.point-before.json`, `${JSON.stringify({ candidateBeforePoint, oracleProbe, sample }, null, 2)}\n`, { flag: "wx" });
  }
  const candidateProjection = phase === "candidate" ? await projectCandidateAuthoredPoint(page, {
    modelGeneration: candidateBeforePoint.model.generation,
    cameraSequence: candidateBeforePoint.viewport.camera.sequence,
    authoredPoint: { x: oracleProbe.authored_anchor[0], y: oracleProbe.authored_anchor[1], z: oracleProbe.authored_anchor[2] }
  }) : null;
  if (phase === "candidate" && (candidateProjection.status !== "available" ||
    !candidateProjection.insideClosedNdc || !candidateProjection.insideCanvasCss)) {
    throw new Error(`candidate projection is not actionable for sample ${sample.sample}: ${JSON.stringify(candidateProjection)}`);
  }
  if (phase === "candidate" &&
      (Math.abs(Number(candidateProjection.canvasCss?.width) - box.width) > 1e-6 ||
       Math.abs(Number(candidateProjection.canvasCss?.height) - box.height) > 1e-6)) {
    throw new Error(`candidate projection dimensions differ from the unique connected main canvas for sample ${sample.sample}: ${JSON.stringify({ projection: candidateProjection.canvasCss, canvasBox: box })}`);
  }
  const [ndcX, ndcY] = probe.ndc;
  const point = phase === "candidate"
    ? { x: box.x + candidateProjection.canvasCssPoint.x, y: box.y + candidateProjection.canvasCssPoint.y }
    : { x: box.x + ((ndcX + 1) / 2) * box.width, y: box.y + ((1 - ndcY) / 2) * box.height };
  const highlightClip = phase === "candidate" ? probe.visual_plan?.clip : {
    x: Math.max(0, point.x - 48),
    y: Math.max(0, point.y - 48),
    width: Math.min(96, 1440 - Math.max(0, point.x - 48)),
    height: Math.min(96, 920 - Math.max(0, point.y - 48))
  };
  const captureStem = path.basename(screenshotPath, path.extname(screenshotPath));
  const preHighlightPath = path.join(path.dirname(screenshotPath), `${captureStem}-highlight-before.png`);
  const postHighlightPath = path.join(path.dirname(screenshotPath), `${captureStem}-highlight-after.png`);
  const beforeCueCapture = phase === "candidate" ? await captureWinnerCue(page, oracleProbe, preHighlightPath) : null;
  if (phase === "baseline") await page.screenshot({ path: preHighlightPath, clip: highlightClip });
  const preHighlightDigest = pngPixelDigest(await readFile(preHighlightPath), selectionVisualOracle);
  await page.evaluate((binding) => {
    const state = (globalThis as any).__uifHarness;
    if (!state) throw new Error("point sample instrumentation state is absent");
    state.activePointSample = binding;
  }, { sample: sample.sample, expectedHitRef, expectedCssPoint: point });
  let causalArming: any = null;
  if (causalToken !== null) {
    causalArming = await armCausalFeedbackMarker(page, {
      token: causalToken,
      phase,
      feedbackKind: "point-selection",
      actionStartEvent: "pointerdown",
      expectedActionTargetTestId: "viewport-canvas",
      maximumFeedbackMarkers: 1,
      actionIdentity: {
        sample: sample.sample,
        expectedRef: expectedHitRef,
        expectedCssPoint: point
      },
      candidateExpectation: phase === "candidate" ? {
        modelGeneration: candidateBeforePoint.model.generation,
        priorRenderSubmissionSequence: candidateBeforePoint.viewport.mainRender.submissionSequence,
        priorActionSequence: candidateBeforePoint.viewport.selection.actionSequence,
        expectedInspectorHeading: `${expectedVisibleLabel} — ${expectedHitRef.type}: ${expectedHitRef.id}`,
        expectedRef: expectedHitRef
      } : null,
      baselineExpectation: phase === "baseline" ? {
        expectedRef: expectedHitRef,
        expectedInspectorHeading: expectedVisibleLabel
      } : null
    });
  }
  await page.mouse.click(point.x, point.y, { button: "left" });
  let publicationStatus = "SELECTION_PUBLICATION_CHANGED";
  let candidateAfterPoint: any = null;
  if (phase === "candidate") {
    candidateAfterPoint = await waitForCandidateExclusiveSelection(
      page,
      expectedHitRef,
      candidateBeforePoint.model.generation,
      candidateBeforePoint.viewport.selection.actionSequence,
      candidateBeforePoint.viewport.mainRender.submissionSequence,
      timeoutMs
    );
  } else {
    try {
      await page.waitForFunction((previous) => {
        const current = document.querySelector('[data-testid="command-selection-readout"]')?.textContent?.trim() ?? null;
        return current !== previous;
      }, before.readout, { timeout: Math.min(timeoutMs, 2_000) });
    } catch {
      publicationStatus = "NO_CHANGED_SELECTION_PUBLICATION_WITHIN_2S";
    }
  }
  const beforeCaptureAt = await page.evaluate(() => performance.now());
  await page.screenshot({ path: screenshotPath });
  const afterCaptureAt = await page.evaluate(() => performance.now());
  const after = await currentSelectionState(page, expectedHitRef);
  const pointerEvents = await page.evaluate(() => ({
    pointerdown: (globalThis as any).__uifHarness?.eventStarts?.pointerdown ?? null,
    pointerup: (globalThis as any).__uifHarness?.eventStarts?.pointerup ?? null
  }));
  const causalMarkerEvidence = causalToken === null ? null : await stopCausalFeedbackMarker(page, causalToken);
  await page.evaluate(() => {
    if ((globalThis as any).__uifHarness) (globalThis as any).__uifHarness.activePointSample = null;
  });
  const eventStart = pointerEvents.pointerdown;
  const coordinateToleranceCssPx = 0.01;
  const activeSampleMatches = pointerEvents.pointerdown?.activePointSample?.sample === sample.sample &&
    pointerEvents.pointerup?.activePointSample?.sample === sample.sample &&
    pointerEvents.pointerdown?.activePointSample?.expectedHitRef?.type === expectedHitRef.type &&
    pointerEvents.pointerdown?.activePointSample?.expectedHitRef?.id === expectedHitRef.id &&
    pointerEvents.pointerup?.activePointSample?.expectedHitRef?.type === expectedHitRef.type &&
    pointerEvents.pointerup?.activePointSample?.expectedHitRef?.id === expectedHitRef.id;
  const pointerCoordinatesMatch = typeof pointerEvents.pointerdown?.clientX === "number" &&
    typeof pointerEvents.pointerdown?.clientY === "number" &&
    typeof pointerEvents.pointerup?.clientX === "number" &&
    typeof pointerEvents.pointerup?.clientY === "number" &&
    Math.abs(pointerEvents.pointerdown.clientX - point.x) <= coordinateToleranceCssPx &&
    Math.abs(pointerEvents.pointerdown.clientY - point.y) <= coordinateToleranceCssPx &&
    Math.abs(pointerEvents.pointerup.clientX - point.x) <= coordinateToleranceCssPx &&
    Math.abs(pointerEvents.pointerup.clientY - point.y) <= coordinateToleranceCssPx;
  const samePointerTransaction = typeof pointerEvents.pointerdown?.pointerId === "number" &&
    pointerEvents.pointerup?.pointerId === pointerEvents.pointerdown.pointerId &&
    typeof pointerEvents.pointerup?.listenerObservedAt === "number" &&
    pointerEvents.pointerup.listenerObservedAt >= pointerEvents.pointerdown.listenerObservedAt &&
    pointerCoordinatesMatch && activeSampleMatches;
  if (phase === "candidate") await writeFile(`${postHighlightPath}.stopped.json`, `${JSON.stringify({ candidateBeforePoint, candidateAfterPoint, causalMarkerEvidence, plan: probe.visual_plan }, null, 2)}\n`, { flag: "wx" });
  const afterCueCapture = phase === "candidate" ? await captureWinnerCue(page, oracleProbe, postHighlightPath) : null;
  if (phase === "baseline") await page.screenshot({ path: postHighlightPath, clip: highlightClip });
  const pairedCue = phase === "candidate" ? pairedWinnerCueWitness(await readFile(preHighlightPath), await readFile(postHighlightPath), probe.visual_plan, oracleProbe, selectionVisualOracle) : null;
  const markerCaptureBound = phase !== "candidate" || (cueActionBindingsStable(beforeCueCapture, afterCueCapture) && (causalToken === null ||
    (causalMarkerEvidence?.active?.feedbackMarkers?.length === 1 && pointCaptureMatchesMarker(causalMarkerEvidence.active.feedbackMarkers[0], afterCueCapture, expectedHitRef))));
  if (phase === "candidate") await writeFile(`${postHighlightPath}.verdict.json`, `${JSON.stringify({ beforeCueCapture, afterCueCapture, pairedCue, markerCaptureBound }, null, 2)}\n`, { flag: "wx" });
  const postHighlightDigest = pngPixelDigest(await readFile(postHighlightPath), selectionVisualOracle);
  const visibleSelectionText = phase === "baseline"
    ? `Selected: ${expectedHitRef.id}`
    : `Selected ${expectedHitRef.type}: ${expectedHitRef.id}; 0 queued`;
  const visibleSelectionLocator = phase === "baseline"
    ? page.locator(".viewport-toolbar > span").filter({ hasText: visibleSelectionText }).last()
    : page.getByTestId("command-selection-readout");
  const visibleSelectionEvidence = await captureVisibleLocatorEvidence(
    visibleSelectionLocator,
    visibleSelectionText,
    path.join(path.dirname(screenshotPath), `${captureStem}-visible-selection.png`)
  );
  const visibleInspectorEvidence = await capturePointInspectorEvidence(
    page, phase, expectedVisibleLabel, expectedHitRef,
    path.join(path.dirname(screenshotPath), `${captureStem}-visible-inspector-heading.png`)
  );
  const highlightPixelsChanged = preHighlightDigest.status === "PASS_DECODED_PIXEL_PAYLOAD" &&
    postHighlightDigest.status === "PASS_DECODED_PIXEL_PAYLOAD" &&
    preHighlightDigest.width === postHighlightDigest.width && preHighlightDigest.height === postHighlightDigest.height &&
    preHighlightDigest.pixelSha256 !== postHighlightDigest.pixelSha256;
  const expectedSelectionColorIncreasedAtProjectedPoint = highlightPixelsChanged &&
    Number(postHighlightDigest.selectionColorWitness?.centralCount) - Number(preHighlightDigest.selectionColorWitness?.centralCount) >= selectionVisualOracle.minimumQualifyingInteriorPixels &&
    Number(postHighlightDigest.selectionColorWitness?.centralCount) >= selectionVisualOracle.minimumQualifyingInteriorPixels;
  const renderedLocalContrastPass = selectionVisualOracle.requiredLocalContrastRatio === undefined ||
    Number(postHighlightDigest.selectionColorWitness?.localContrastQualifyingCount) >= selectionVisualOracle.minimumQualifyingInteriorPixels;
  const visiblePixelEvidencePass = (phase === "candidate" ? pairedCue?.status === "PASS_PAIRED_WINNER_CUE_TRANSITION" && markerCaptureBound : expectedSelectionColorIncreasedAtProjectedPoint && renderedLocalContrastPass) &&
    visibleSelectionEvidence.status === "PASS_EXACT_VISIBLE_TEXT_AND_DECODED_PNG" &&
    visibleInspectorEvidence.status === "PASS_EXACT_VISIBLE_TEXT_AND_DECODED_PNG";
  const exactExpectedSelection = phase === "candidate"
    ? candidateAfterPoint.viewport.selection.orderedRefs.length === 1 &&
      candidateAfterPoint.viewport.selection.orderedRefs[0].type === expectedHitRef.type &&
      candidateAfterPoint.viewport.selection.orderedRefs[0].id === expectedHitRef.id &&
      candidateAfterPoint.viewport.selection.primaryRef.type === expectedHitRef.type &&
      candidateAfterPoint.viewport.selection.primaryRef.id === expectedHitRef.id
    : after.ref?.type === expectedHitRef.type && after.ref?.id === expectedHitRef.id;
  const propertyIdentityMatches = phase === "candidate"
    ? candidateAfterPoint.viewport.inspector.ref.type === expectedHitRef.type &&
      candidateAfterPoint.viewport.inspector.ref.id === expectedHitRef.id
    : after.inspectorContainsExpectedId === true;
  const realCanvasEvent = eventStart?.testId === "viewport-canvas" && eventStart?.targetTag === "CANVAS";
  const realCanvasUpEvent = pointerEvents.pointerup?.testId === "viewport-canvas" && pointerEvents.pointerup?.targetTag === "CANVAS";
  const samePointerEvent = phase === "candidate"
    ? typeof eventStart?.at === "number" &&
      Math.abs(candidateAfterPoint.viewport.selection.pointerDownAt - eventStart.at) <= 5
    : null;
  return {
    sample: sample.sample,
    probeAnchorRef: sample.probe_anchor_ref,
    action: "real_canvas_pointer_pick_labels_off",
    status: exactExpectedSelection && propertyIdentityMatches && realCanvasEvent && realCanvasUpEvent && samePointerTransaction && visiblePixelEvidencePass && samePointerEvent !== false
      ? (phase === "baseline" ? "BASELINE_EXPECTED_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED" : "CANDIDATE_EXPECTED_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED")
      : "FAIL_EXPECTED_POINT_FEEDBACK",
    publicationStatus,
    ndc: probe.ndc,
    canvasCssBox: box,
    mainCanvasIdentity: mainCanvas.identity,
    pointerCssPoint: point,
    resetPrecondition: {
      action: "real_project_tree_row_activation",
      rowTestId: projectRowTestId,
      expectedExclusiveRef: { type: "project", id: fixtureProjectId },
      observed: before,
      candidatePublication: phase === "candidate" ? {
        beforeActionSequence: candidateBeforeReset.viewport.selection.actionSequence,
        afterActionSequence: resetCandidateSnapshot.viewport.selection.actionSequence,
        stateWasAlreadyExclusiveProject: candidateBeforeReset.viewport.selection.orderedRefs?.length === 1 &&
          candidateBeforeReset.viewport.selection.orderedRefs[0]?.type === "project" &&
          candidateBeforeReset.viewport.selection.orderedRefs[0]?.id === fixtureProjectId,
        resultingSelection: resetCandidateSnapshot.viewport.selection,
        resultingMainRender: resetCandidateSnapshot.viewport.mainRender
      } : null,
      startedAt: resetStartedAt,
      nextRafOpportunityAt: resetNextRafOpportunityAt,
      durationMs: resetNextRafOpportunityAt - resetStartedAt,
      excludedFromTimedPointDuration: true
    },
    candidateProjection,
    candidateGenerationEvidence: phase === "candidate" ? {
      beforePoint: {
        modelGeneration: candidateBeforePoint.model.generation,
        cameraSequence: candidateBeforePoint.viewport.camera.sequence,
        selectionActionSequence: candidateBeforePoint.viewport.selection.actionSequence,
        renderSubmissionSequence: candidateBeforePoint.viewport.mainRender.submissionSequence
      },
      afterPoint: {
        modelGeneration: candidateAfterPoint.model.generation,
        selection: candidateAfterPoint.viewport.selection,
        inspector: candidateAfterPoint.viewport.inspector,
        mainRender: candidateAfterPoint.viewport.mainRender
      }
    } : null,
    causalMarker: causalToken === null ? null : {
      arming: causalArming,
      evidence: causalMarkerEvidence,
      claim: "Harness callback/content marker evidence only; Chromium lineage extraction is a separate required step."
    },
    expectedHitRef,
    actualSelection: after,
    eventStart,
    pointerEvents,
    samePointerTransaction,
    sampleEventBinding: {
      sample: sample.sample,
      expectedCssPoint: point,
      pointerId: pointerEvents.pointerdown?.pointerId ?? null,
      coordinateToleranceCssPx,
      pointerCoordinatesMatch,
      activeSampleMatches,
      orderedDownThenUp: typeof pointerEvents.pointerdown?.listenerObservedAt === "number" &&
        typeof pointerEvents.pointerup?.listenerObservedAt === "number" &&
        pointerEvents.pointerup.listenerObservedAt >= pointerEvents.pointerdown.listenerObservedAt
    },
    visiblePixelEvidence: {
      status: visiblePixelEvidencePass ? "PASS_EXACT_VISIBLE_IDENTITY_AND_HIGHLIGHT_PIXEL_CHANGE" : "FAIL_VISIBLE_IDENTITY_OR_HIGHLIGHT_PIXEL_EVIDENCE",
      highlightClip, pairedCue, beforeCueCapture, afterCueCapture, markerCaptureBound,
      before: { path: path.basename(preHighlightPath), pixelDigest: preHighlightDigest },
      after: { path: path.basename(postHighlightPath), pixelDigest: postHighlightDigest },
      highlightPixelsChanged,
      expectedSelectionColorIncreasedAtProjectedPoint,
      renderedLocalContrastPass,
      independentlyProjectedExpectedPrimitive: {
        expectedHitRef,
        probeAnchorRef: sample.probe_anchor_ref,
        authoredAnchor: oracleProbe.authored_anchor,
        ndc: probe.ndc,
        pointerCssPoint: point
      },
      selection: visibleSelectionEvidence,
      inspectorHeading: visibleInspectorEvidence,
      limitation: "The phase-specific source-bound selection-color increase is checked in an ROI centred on the independently projected expected primitive. At overlapping primitives the pixels alone cannot identify which typed primitive changed; exact typed selection/property evidence and independent oracle identity remain jointly required."
    },
    browserObservedCapture: {
      path: path.basename(screenshotPath),
      beforeCaptureAt,
      postCaptureAt: afterCaptureAt,
      captureElapsedUpperBoundMs: afterCaptureAt - beforeCaptureAt,
      format: "png",
      scale: "device",
      fullPage: false
    },
    durationMs: typeof eventStart?.at === "number" ? afterCaptureAt - eventStart.at : null,
    durationStartMeaning: "capture-listener performance.now observation of pointerdown; browser event.timeStamp retained separately",
    exactExpectedSelection,
    propertyIdentityMatches,
    realCanvasEvent,
    realCanvasUpEvent,
    samePointerEvent,
    strictPresentedStatus: phase === "baseline"
      ? "N/A_BASELINE_CAPTURE_HAS_NO_GENERATION_BOUND_MAIN_RENDER_SUBMISSION"
      : "CANDIDATE_GENERATION_BOUND_SUBMISSION_AND_BROWSER_CAPTURE_CONSERVATIVE_UPPER_BOUND"
  };
}

export type PriorBoxBaseline = Readonly<{ kind: "unavailable" | "active"; actionSequence: number; generation: number }>;
export function validatedPriorBoxBaseline(snapshot: any): PriorBoxBaseline {
  const generation = snapshot?.model?.generation, viewport = snapshot?.viewport, box = viewport?.box;
  if (!Number.isSafeInteger(generation) || generation <= 0 || viewport?.generation !== generation ||
      !box || typeof box !== "object" || Array.isArray(box)) throw new Error("invalid pre-box model/viewport/record binding");
  if (Object.prototype.hasOwnProperty.call(box, "status")) {
    if (box.status !== "unavailable" || Object.keys(box).length !== 1) throw new Error("contradictory or unknown pre-box unavailable record");
    return { kind: "unavailable", actionSequence: 0, generation };
  }
  if (box.generation !== generation || !Number.isSafeInteger(box.actionSequence) || box.actionSequence <= 0 ||
      !Number.isSafeInteger(box.renderSubmissionSequence) || box.renderSubmissionSequence < 0 ||
      typeof box.publishedAt !== "number" || !Number.isFinite(box.publishedAt) || box.publishedAt < 0 ||
      !["left-to-right", "right-to-left"].includes(box.direction) || typeof box.filter !== "string" ||
      !Array.isArray(box.orderedRefs) || box.orderedRefs.some((ref: any) => typeof ref?.type !== "string" || !ref.type || typeof ref?.id !== "string" || !ref.id) || !(box.primaryRef === null || (typeof box.primaryRef?.type === "string" && typeof box.primaryRef?.id === "string"))) {
    throw new Error("invalid or stale active pre-box publication");
  }
  return { kind: "active", actionSequence: box.actionSequence, generation };
}

export function expectedBoxInspectorHeading(model: any, orderedRefs: any[], primaryRef: any, projectId: string): string {
  if (!Array.isArray(orderedRefs)) throw new Error("independent Box ordered expectation unavailable");
  if (orderedRefs.length > 1) return `${orderedRefs.length} selected items`;
  const ref = primaryRef ?? { type: "project", id: projectId };
  return `${expectedVisibleEntityLabel(model, ref)} — ${ref.type}: ${ref.id}`;
}

export function boxCallbackMatchesSnapshot(marker: any, snapshot: any): boolean {
  const o = marker?.observed, v = snapshot?.viewport, p = v?.mainRender?.selectionPresentation;
  return o?.reason === "BOX_RENDERED_SELECTION_READY" && o.failedPredicates?.length === 0 &&
    Number.isSafeInteger(o.mainRenderSubmissionSequence) && o.mainRenderSubmissionSequence === v?.mainRender?.submissionSequence &&
    o.modelGeneration === snapshot?.model?.generation && o.cameraSequence === v?.camera?.sequence &&
    o.actionSequence === v?.box?.actionSequence && o.selectionActionSequence === v?.selection?.actionSequence &&
    o.selectionRenderSubmissionSequence === v?.selection?.renderSubmissionSequence &&
    o.inspectorPublicationSequence === v?.inspector?.publicationSequence &&
    o.presentation?.resourceGeneration === p?.resourceGeneration && o.presentation?.modelGeneration === p?.modelGeneration &&
    o.presentation?.revision === p?.revision && o.presentation?.renderedSubmissionSequence === p?.renderedSubmissionSequence &&
    o.presentation?.appliedAfterSubmissionSequence === p?.appliedAfterSubmissionSequence;
}

export async function readBoxVisibleIdentity(page: Page): Promise<any> {
  return page.evaluate(() => {
    const text = (selector: string) => {
      const nodes = document.querySelectorAll(selector);
      return nodes.length === 1 && nodes[0].isConnected && (nodes[0] as HTMLElement).getClientRects().length > 0 &&
        getComputedStyle(nodes[0]).visibility !== "hidden" ? nodes[0].textContent?.trim() ?? null : null;
    };
    const aggregates = document.querySelectorAll('[data-testid="aggregate-property-inspector"]');
    const rows = aggregates.length === 1 ? Array.from(aggregates[0].querySelectorAll(':scope > dl > div'))
      .filter((row) => row.querySelector(':scope > dt')?.textContent?.trim() === "Primary") : [];
    const values = rows.length === 1 ? rows[0].querySelectorAll(':scope > dd') : [];
    const aggregatePrimary = aggregates.length === 1 && values.length === 1 && (values[0] as HTMLElement).getClientRects().length > 0 &&
      getComputedStyle(values[0]).visibility !== "hidden" ? values[0].textContent?.trim() ?? null : null;
    return { readout: text('[data-testid="command-selection-readout"]'), inspectorHeading: text('.workspace-pane-inspector .panel.inspector > h2'),
      aggregateCount: aggregates.length, aggregatePrimary };
  });
}

// Both box callers write this complete context before asserting the stopped result.
// The callback work and any waiting have already stopped; these writes are not timing endpoints.
export async function persistBoxPostcondition(file: string, context: any): Promise<any> {
  const { after, expected, prior, priorRender, pointerEvents } = context;
  const v = after?.viewport, box = v?.box, selection = v?.selection, inspector = v?.inspector;
  const sameRef = (a: any, b: any) => a === null || b === null ? a === b :
    typeof a?.type === "string" && a.type === b?.type && typeof a?.id === "string" && a.id === b?.id;
  const sameRefs = (a: any, b: any) => Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((ref, i) => sameRef(ref, b[i]));
  const presented = v?.mainRender?.selectionPresentation;
  const expectedIdentity = expected.primaryRef ?? { type: "project", id: context.projectId };
  const readoutMatch = context.dom?.readout?.match(/^Selected ([^:;]+): (.+?); [0-9]+ queued$/);
  const checks = {
    selectionPublication: Number.isSafeInteger(selection?.actionSequence) && selection.actionSequence > 0 && Number.isFinite(selection?.publishedAt),
    inspectorPublication: Number.isSafeInteger(inspector?.publicationSequence) && inspector.publicationSequence > 0,
    selectionRender: Number.isSafeInteger(selection?.renderSubmissionSequence) && selection.renderSubmissionSequence > 0 && selection.renderSubmissionSequence === v?.mainRender?.submissionSequence,
    renderedResource: Number.isSafeInteger(context.resourceGeneration) && context.resourceGeneration > 0 && presented?.resourceGeneration === context.resourceGeneration,
    renderedModel: presented?.modelGeneration === prior.generation,
    renderedRevision: Number.isSafeInteger(presented?.revision) && presented.revision > 0,
    renderedBarrier: Number.isSafeInteger(presented?.appliedAfterSubmissionSequence) && presented.appliedAfterSubmissionSequence >= 0 &&
      Number.isSafeInteger(presented?.renderedSubmissionSequence) && presented.renderedSubmissionSequence > presented.appliedAfterSubmissionSequence && presented.renderedSubmissionSequence === v?.mainRender?.submissionSequence,
    renderedOrderedRefs: sameRefs(presented?.orderedRefs, expected.orderedRefs),
    readout: readoutMatch?.[1] === expectedIdentity.type && readoutMatch?.[2] === expectedIdentity.id,
    inspectorHeading: typeof context.expectedInspectorHeading === "string" && context.dom?.inspectorHeading === context.expectedInspectorHeading,
    aggregatePrimary: expected.orderedRefs?.length > 1 ? context.dom?.aggregateCount === 1 && context.dom?.aggregatePrimary === `${expectedIdentity.type}: ${expectedIdentity.id}` : context.dom?.aggregateCount === 0,
    generation: after?.model?.generation === prior.generation && v?.generation === prior.generation && box?.generation === prior.generation && selection?.generation === prior.generation && inspector?.generation === prior.generation,
    actionSequence: Number.isSafeInteger(prior.actionSequence) && prior.actionSequence >= 0 && Number.isSafeInteger(box?.actionSequence) && box.actionSequence > 0 && box.actionSequence > prior.actionSequence,
    render: Number.isSafeInteger(priorRender) && priorRender >= 0 && Number.isSafeInteger(box?.renderSubmissionSequence) && box.renderSubmissionSequence > priorRender &&
      v?.mainRender?.generation === prior.generation && Number.isSafeInteger(v.mainRender.submissionSequence) &&
      v.mainRender.submissionSequence > priorRender && v.mainRender.submissionSequence >= box.renderSubmissionSequence && Number.isFinite(v.mainRender.submittedAt),
    publication: Number.isFinite(box?.publishedAt) && Number.isFinite(pointerEvents?.pointerup?.listenerObservedAt) && box.publishedAt >= pointerEvents.pointerup.listenerObservedAt,
    direction: box?.direction === expected.direction, filter: box?.filter === expected.filter,
    boxOrderedRefs: sameRefs(box?.orderedRefs, expected.orderedRefs), selectionOrderedRefs: sameRefs(selection?.orderedRefs, expected.orderedRefs),
    boxPrimary: sameRef(box?.primaryRef, expected.primaryRef), selectionPrimary: sameRef(selection?.primaryRef, expected.primaryRef),
    inspector: sameRef(inspector?.ref, expected.primaryRef ?? { type: "project", id: context.projectId })
  };
  const failedFields = Object.entries(checks).filter(([, pass]) => !pass).map(([field]) => field);
  const record = { ...context, checks, failedFields, status: failedFields.length || context.primaryError ? "FAIL_STOPPED_BOX_POSTCONDITION" : "PASS_STOPPED_BOX_POSTCONDITION" };
  await writeFile(file, `${JSON.stringify(record, (_key, value) => typeof value === "number" && !Number.isFinite(value)
    ? { nonFiniteNumber: String(value) } : value, 2)}\n`, { flag: "wx" });
  if (record.status !== "PASS_STOPPED_BOX_POSTCONDITION") throw new Error(`stopped box postcondition failed: ${JSON.stringify({ failedFields, primaryError: context.primaryError })}`);
  return record;
}

function boxFilterLabel(value: string): string {
  const labels: Record<string, string> = {
    all: "All",
    pipes: "Pipes",
    nodes: "Nodes",
    supports: "Supports",
    components: "Components"
  };
  const label = labels[value];
  if (!label) throw new Error(`unknown frozen box-selection filter: ${value}`);
  return label;
}

export async function measureCanvasBoxSelection(
  page: Page,
  sample: any,
  expected: any,
  timeoutMs: number,
  screenshotPath: string,
  fixtureProjectId: string,
  expectedInspectorHeading: string,
  proofToken?: string
): Promise<any> {
  if (expected?.sample !== sample.sample || expected?.filter !== sample.filter) {
    throw new Error(`box oracle/sample binding mismatch for sample ${sample.sample}`);
  }
  const expectedDirection = sample.end_normalized[0] >= sample.start_normalized[0]
    ? "left-to-right"
    : "right-to-left";
  if (expected.direction !== expectedDirection) {
    throw new Error(`box oracle direction mismatch for sample ${sample.sample}`);
  }

  // Establish an ordinary, exclusive Project selection before each measured
  // plain drag. This reset and the mode/filter setup are outside the timer.
  const beforeReset = (await readCandidateDiagnostics(page)).snapshot;
  const projectRowTestId = treeRowTestId({ type: "project", id: fixtureProjectId }, "candidate");
  const resetStartedAt = await page.evaluate(() => performance.now());
  await page.getByTestId(projectRowTestId).click({ timeout: timeoutMs });
  const afterReset = await waitForCandidateExclusiveSelection(
    page,
    { type: "project", id: fixtureProjectId },
    beforeReset.model.generation,
    beforeReset.viewport.selection.actionSequence,
    beforeReset.viewport.mainRender.submissionSequence,
    timeoutMs,
    Array.isArray(beforeReset.viewport.selection.orderedRefs) &&
      beforeReset.viewport.selection.orderedRefs.length === 1 &&
      beforeReset.viewport.selection.orderedRefs[0]?.type === "project" &&
      beforeReset.viewport.selection.orderedRefs[0]?.id === fixtureProjectId &&
      beforeReset.viewport.selection.primaryRef?.type === "project" &&
      beforeReset.viewport.selection.primaryRef?.id === fixtureProjectId
  );
  const resetNextRafOpportunityAt = await afterNextRafOpportunity(page);

  const filter = page.getByTestId("viewport-selection-filter");
  const expectedFilterLabel = boxFilterLabel(sample.filter);
  await filter.selectOption({ label: expectedFilterLabel }, { timeout: timeoutMs });
  await page.waitForFunction(({ testId, label }) => {
    const element = document.querySelector(`[data-testid="${CSS.escape(testId)}"]`) as HTMLSelectElement | null;
    return element?.selectedOptions?.[0]?.textContent?.trim() === label;
  }, { testId: "viewport-selection-filter", label: expectedFilterLabel }, { timeout: timeoutMs });
  await ensureViewportToggle(page, "viewport-box-select", true, timeoutMs);

  const before = (await readCandidateDiagnostics(page)).snapshot;
  const evidenceStem = screenshotPath.replace(/\.png$/i, "");
  await mkdir(path.dirname(screenshotPath), { recursive: true });
  await writeFile(`${evidenceStem}-before.json`, `${JSON.stringify({ before, expected, sample, projectId: fixtureProjectId }, null, 2)}\n`, { flag: "wx" });
  const priorBox = validatedPriorBoxBaseline(before);
  const resourceGeneration = before.viewport.mainRender.selectionPresentation?.resourceGeneration;
  if (!Number.isSafeInteger(resourceGeneration) || resourceGeneration <= 0 || typeof expectedInspectorHeading !== "string") throw new Error("box render/DOM expectation unavailable");
  if (before.model.generation !== afterReset.model.generation) {
    throw new Error(`box sample ${sample.sample} setup crossed a model generation`);
  }
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  const canvasBox = mainCanvas.box;
  const start = {
    x: canvasBox.x + sample.start_normalized[0] * canvasBox.width,
    y: canvasBox.y + sample.start_normalized[1] * canvasBox.height
  };
  const end = {
    x: canvasBox.x + sample.end_normalized[0] * canvasBox.width,
    y: canvasBox.y + sample.end_normalized[1] * canvasBox.height
  };

  await writeFile(`${evidenceStem}-action.json`, `${JSON.stringify({ sample, expected, priorBox, start, end,
    canvas: mainCanvas, camera: before.viewport.camera, model: before.model }, null, 2)}\n`, { flag: "wx" });
  if (proofToken) await armCausalFeedbackMarker(page, { token: proofToken, phase: "candidate", feedbackKind: "box-selection",
    actionStartEvent: "pointerup", expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
    actionIdentity: { sample: sample.sample, start, end, qualification: "CALLBACK_ONLY_NO_COMPOSITOR_TIMING" },
    candidateExpectation: { modelGeneration: before.model.generation, priorRenderSubmissionSequence: before.viewport.mainRender.submissionSequence,
      priorBoxActionSequence: priorBox.actionSequence, priorBoxBaseline: priorBox, resourceGeneration, expectedInspectorHeading,
      direction: expected.direction, filter: expected.filter, orderedRefs: expected.orderedRefs, primaryRef: expected.primaryRef, projectId: fixtureProjectId } });
  let primaryError: string | null = null;
  try {
    await page.mouse.move(start.x, start.y);
    await page.mouse.down({ button: "left" });
    await page.mouse.move(end.x, end.y, { steps: 8 });
    await page.mouse.up({ button: "left" });

    await page.waitForFunction(({ generation, priorBoxAction, priorRender, direction, boxFilter, orderedRefs, primaryRef, projectId, resourceGeneration, expectedInspectorHeading }) => {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      const viewport = snapshot?.viewport;
      const box = viewport?.box;
      const selection = viewport?.selection;
      const inspector = viewport?.inspector;
      const render = viewport?.mainRender;
      const sameRef = (left: any, right: any) => left === null || right === null
        ? left === right
        : left?.type === right?.type && left?.id === right?.id;
      const sameRefs = (left: any, right: any) => Array.isArray(left) && Array.isArray(right) &&
        left.length === right.length && left.every((ref: any, index: number) => sameRef(ref, right[index]));
      const expectedInspector = primaryRef ?? { type: "project", id: projectId };
      const eventCommit = (globalThis as any).__uifHarness?.eventStarts?.pointerup;
      const presented = render?.selectionPresentation;
      const text = (selector: string) => {
        const nodes = document.querySelectorAll(selector);
        return nodes.length === 1 && nodes[0].isConnected && (nodes[0] as HTMLElement).getClientRects().length > 0 &&
          getComputedStyle(nodes[0]).visibility !== "hidden" ? nodes[0].textContent?.trim() ?? null : null;
      };
      const readout = text('[data-testid="command-selection-readout"]')?.match(/^Selected ([^:;]+): (.+?); [0-9]+ queued$/);
      const aggregates = document.querySelectorAll('[data-testid="aggregate-property-inspector"]');
      const rows = aggregates.length === 1 ? Array.from(aggregates[0].querySelectorAll(':scope > dl > div'))
        .filter((row) => row.querySelector(':scope > dt')?.textContent?.trim() === "Primary") : [];
      const values = rows.length === 1 ? rows[0].querySelectorAll(':scope > dd') : [];
      const aggregatePrimary = values.length === 1 && (values[0] as HTMLElement).getClientRects().length > 0 && getComputedStyle(values[0]).visibility !== "hidden" ? values[0].textContent?.trim() ?? null : null;
      const aggregateMatches = orderedRefs.length > 1 ? aggregates.length === 1 && aggregatePrimary === `${expectedInspector.type}: ${expectedInspector.id}` : aggregates.length === 0;
      return aggregateMatches && Number.isSafeInteger(selection?.actionSequence) && selection.actionSequence > 0 && Number.isFinite(selection?.publishedAt) &&
        Number.isSafeInteger(inspector?.publicationSequence) && inspector.publicationSequence > 0 &&
        selection?.renderSubmissionSequence === render?.submissionSequence && Number.isSafeInteger(selection?.renderSubmissionSequence) && selection.renderSubmissionSequence > 0 &&
        presented?.resourceGeneration === resourceGeneration && presented?.modelGeneration === generation &&
        Number.isSafeInteger(presented?.revision) && presented.revision > 0 &&
        Number.isSafeInteger(presented?.appliedAfterSubmissionSequence) && presented.appliedAfterSubmissionSequence >= 0 &&
        Number.isSafeInteger(presented?.renderedSubmissionSequence) && presented.renderedSubmissionSequence > presented.appliedAfterSubmissionSequence &&
        presented.renderedSubmissionSequence === render?.submissionSequence && sameRefs(presented.orderedRefs, orderedRefs) &&
        readout?.[1] === expectedInspector.type && readout?.[2] === expectedInspector.id &&
        text('.workspace-pane-inspector .panel.inspector > h2') === expectedInspectorHeading &&
        snapshot?.model?.generation === generation && viewport?.generation === generation &&
        box?.generation === generation && Number.isSafeInteger(priorBoxAction) && priorBoxAction >= 0 &&
        Number.isSafeInteger(box.actionSequence) && box.actionSequence > 0 && box.actionSequence > priorBoxAction &&
        box.direction === direction && box.filter === boxFilter &&
        sameRefs(box.orderedRefs, orderedRefs) && sameRef(box.primaryRef, primaryRef) &&
        Number.isFinite(box.publishedAt) && Number.isFinite(eventCommit?.at) && box.publishedAt >= eventCommit.at &&
        selection?.generation === generation && sameRefs(selection?.orderedRefs, orderedRefs) && sameRef(selection?.primaryRef, primaryRef) &&
        inspector?.generation === generation && sameRef(inspector?.ref, expectedInspector) &&
        Number.isSafeInteger(box.renderSubmissionSequence) && box.renderSubmissionSequence > priorRender && render?.generation === generation &&
        Number.isSafeInteger(priorRender) && priorRender >= 0 && Number.isSafeInteger(render.submissionSequence) &&
        render.submissionSequence > priorRender && render.submissionSequence >= box.renderSubmissionSequence &&
        Number.isFinite(render.submittedAt);
    }, {
      generation: before.model.generation,
      priorBoxAction: priorBox.actionSequence,
      priorRender: before.viewport.mainRender.submissionSequence,
      direction: expected.direction,
      boxFilter: expected.filter,
      orderedRefs: expected.orderedRefs,
      primaryRef: expected.primaryRef,
      projectId: fixtureProjectId, resourceGeneration, expectedInspectorHeading
    }, { timeout: timeoutMs });
  } catch (error) { primaryError = String(error); }
  const callbackProof = proofToken ? await stopCausalFeedbackMarker(page, proofToken) : null;
  if (callbackProof) {
    await writeFile(`${evidenceStem}-callback.json`, `${JSON.stringify(callbackProof, null, 2)}\n`, { flag: "wx" });
    if (callbackProof.active.feedbackMarkers.length !== 1 && !primaryError) primaryError = "ordinary Box control has no unique qualified callback";
  }
  if (primaryError) {
    const after = (await readCandidateDiagnostics(page)).snapshot;
    const pointerEvents = await page.evaluate(() => ({ pointerdown: (globalThis as any).__uifHarness?.eventStarts?.pointerdown ?? null,
      pointerup: (globalThis as any).__uifHarness?.eventStarts?.pointerup ?? null }));
    await persistBoxPostcondition(`${evidenceStem}-stopped.json`, { before, after, expected, prior: priorBox,
      priorRender: before.viewport.mainRender.submissionSequence, projectId: fixtureProjectId, pointerEvents,
      actionIdentity: { sample, start, end }, canvas: mainCanvas, primaryError, resourceGeneration, expectedInspectorHeading, dom: await readBoxVisibleIdentity(page) });
    throw new Error(primaryError);
  }

  let postCaptureAt: number | null = null;
  try {
    await page.screenshot({ path: screenshotPath });
    postCaptureAt = await page.evaluate(() => performance.now());
  } catch (error) { primaryError = String(error); }
  const after = (await readCandidateDiagnostics(page)).snapshot;
  const pointerEvents = await page.evaluate(() => ({
    pointerdown: (globalThis as any).__uifHarness?.eventStarts?.pointerdown ?? null,
    pointerup: (globalThis as any).__uifHarness?.eventStarts?.pointerup ?? null
  }));
  if (callbackProof && !boxCallbackMatchesSnapshot(callbackProof.active.feedbackMarkers[0], after))
    primaryError = primaryError ?? "Box capture drifted from qualified callback submission";
  await persistBoxPostcondition(`${evidenceStem}-stopped.json`, { before, after, expected, prior: priorBox,
    priorRender: before.viewport.mainRender.submissionSequence, projectId: fixtureProjectId, pointerEvents,
    actionIdentity: { sample, start, end }, canvas: mainCanvas, primaryError, resourceGeneration, expectedInspectorHeading, dom: await readBoxVisibleIdentity(page) });
  if (postCaptureAt === null) throw new Error("box capture endpoint unavailable");
  const eventStart = pointerEvents.pointerdown;
  const eventCommit = pointerEvents.pointerup;
  const coordinateToleranceCssPx = 1;
  const realCanvasDownEvent = eventStart?.testId === "viewport-canvas" && eventStart?.targetTag === "CANVAS";
  const realCanvasUpEvent = eventCommit?.testId === "viewport-canvas" && eventCommit?.targetTag === "CANVAS";
  const samePointerTransaction = typeof eventStart?.pointerId === "number" &&
    eventCommit?.pointerId === eventStart.pointerId &&
    typeof eventStart?.listenerObservedAt === "number" &&
    typeof eventCommit?.listenerObservedAt === "number" &&
    eventCommit.listenerObservedAt >= eventStart.listenerObservedAt;
  const pointerCoordinatesMatch = typeof eventStart?.clientX === "number" &&
    typeof eventStart?.clientY === "number" &&
    typeof eventCommit?.clientX === "number" &&
    typeof eventCommit?.clientY === "number" &&
    Math.abs(eventStart.clientX - start.x) <= coordinateToleranceCssPx &&
    Math.abs(eventStart.clientY - start.y) <= coordinateToleranceCssPx &&
    Math.abs(eventCommit.clientX - end.x) <= coordinateToleranceCssPx &&
    Math.abs(eventCommit.clientY - end.y) <= coordinateToleranceCssPx;
  const exactBox = after.viewport.box.direction === expected.direction &&
    after.viewport.box.filter === expected.filter &&
    JSON.stringify(after.viewport.box.orderedRefs) === JSON.stringify(expected.orderedRefs) &&
    JSON.stringify(after.viewport.box.primaryRef) === JSON.stringify(expected.primaryRef);
  return {
    sample: sample.sample,
    action: "real_primary_pointer_canvas_box_drag",
    status: realCanvasDownEvent && realCanvasUpEvent && samePointerTransaction && pointerCoordinatesMatch && exactBox
      ? "CANDIDATE_EXPECTED_BOX_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED"
      : "FAIL_EXPECTED_BOX_FEEDBACK",
    filter: sample.filter,
    direction: expected.direction,
    startNormalized: sample.start_normalized,
    endNormalized: sample.end_normalized,
    canvasCssBox: canvasBox,
    mainCanvasIdentity: mainCanvas.identity,
    pointerCssStart: start,
    pointerCssEnd: end,
    expectedOrderedRefs: expected.orderedRefs,
    expectedPrimaryRef: expected.primaryRef,
    eventStart,
    eventCommit,
    realCanvasDownEvent,
    realCanvasUpEvent,
    samePointerTransaction,
    pointerCoordinatesMatch,
    coordinateToleranceCssPx,
    exactBox,
    callbackProof,
    candidateGenerationEvidence: {
      before: { modelGeneration: before.model.generation, box: before.viewport.box, mainRender: before.viewport.mainRender },
      after: { modelGeneration: after.model.generation, box: after.viewport.box, selection: after.viewport.selection, inspector: after.viewport.inspector, mainRender: after.viewport.mainRender }
    },
    resetPrecondition: {
      action: "real_project_tree_row_activation",
      rowTestId: projectRowTestId,
      expectedExclusiveRef: { type: "project", id: fixtureProjectId },
      startedAt: resetStartedAt,
      nextRafOpportunityAt: resetNextRafOpportunityAt,
      durationMs: resetNextRafOpportunityAt - resetStartedAt,
      excludedFromTimedBoxDuration: true
    },
    modeAndFilterSetupExcludedFromTimedDuration: true,
    browserObservedCapture: { path: path.basename(screenshotPath), postCaptureAt },
    durationStartMeaning: "capture-listener performance.now observation of the real pointerup that commits the completed box gesture; browser event.timeStamp retained separately",
    durationMs: typeof eventCommit?.at === "number" ? postCaptureAt - eventCommit.at : null,
    wholeGestureCaptureDurationMs: typeof eventStart?.at === "number" ? postCaptureAt - eventStart.at : null,
    userControlledDragDurationMs: typeof eventStart?.at === "number" && typeof eventCommit?.at === "number"
      ? eventCommit.at - eventStart.at
      : null,
    strictPresentedStatus: "CANDIDATE_GENERATION_BOUND_BOX_SUBMISSION_AND_BROWSER_CAPTURE_CONSERVATIVE_UPPER_BOUND"
  };
}

export async function measureTreeFilter(
  page: Page,
  query: string,
  expectedCount: number,
  totalCount: number,
  timeoutMs: number,
  phase: BenchmarkPhase = "baseline",
  screenshotPath?: string
): Promise<any> {
  const candidateBefore = phase === "candidate" ? (await readCandidateDiagnostics(page)).snapshot : null;
  const input = page.getByTestId("model-tree-filter-input");
  await input.fill(query, { timeout: timeoutMs });
  await page.waitForFunction(({ value, expectedSummary }) => {
    const inputElement = document.querySelector('[data-testid="model-tree-filter-input"]') as HTMLInputElement | null;
    const summary = document.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent ?? "";
    return inputElement?.value === value && summary.includes(expectedSummary);
  }, { value: query, expectedSummary: `${expectedCount} of ${totalCount} model entities visible` }, { timeout: timeoutMs });
  let candidateAfter: any = null;
  if (phase === "candidate") {
    await page.waitForFunction(({ generation, priorAction, priorRender, expectedQuery, count }) => {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      const filter = snapshot?.viewport?.filter;
      const render = snapshot?.viewport?.mainRender;
      return snapshot?.model?.generation === generation &&
        filter?.generation === generation && filter?.actionSequence > priorAction &&
        filter.query === expectedQuery && filter.visibleCount === count &&
        typeof filter.publishedAt === "number" && Number.isInteger(filter.renderSubmissionSequence) &&
        render?.generation === generation && render?.submissionSequence > priorRender &&
        render.submissionSequence >= filter.renderSubmissionSequence && typeof render.submittedAt === "number";
    }, {
      generation: candidateBefore.model.generation,
      priorAction: candidateBefore.viewport.filter.actionSequence,
      priorRender: candidateBefore.viewport.mainRender.submissionSequence,
      expectedQuery: query,
      count: expectedCount
    }, { timeout: timeoutMs });
    candidateAfter = (await readCandidateDiagnostics(page)).snapshot;
  }
  const endedAt = await afterNextRafOpportunity(page);
  const record: any = await page.evaluate(({ value, end, expectedCount }) => {
    const start = (globalThis as any).__uifHarness?.eventStarts?.input;
    return {
      query: value,
      status: typeof start?.at === "number" ? "MEASURED" : "MEASUREMENT_FAILED",
      startAt: start?.at ?? null,
      endAt: end,
      durationMs: typeof start?.at === "number" ? end - start.at : null,
      expectedCount,
      summary: document.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent ?? null,
      publication: "exact query and expected count summary committed, followed by a next-RAF paint opportunity; no compositor claim"
    };
  }, { value: query, end: endedAt, expectedCount });
  record.nextRafObserverOpportunityAt = endedAt;
  record.nextRafObserverOpportunityLimitation = "Observer wait overhead retained in both cohorts; this RAF opportunity is not compositor presentation evidence.";
  if (!screenshotPath) throw new Error(`${phase} tree-filter measurement requires the matched screenshot endpoint`);
  {
    const beforeCaptureAt = await page.evaluate(() => performance.now());
    await page.screenshot({ path: screenshotPath });
    const postCaptureAt = await page.evaluate(() => performance.now());
    record.status = phase === "candidate"
      ? "CANDIDATE_GENERATION_BOUND_FILTER_AND_BROWSER_CAPTURE_CONSERVATIVE_UPPER_BOUND"
      : "BASELINE_FILTER_SUMMARY_AND_BROWSER_CAPTURE_CONSERVATIVE_UPPER_BOUND";
    record.endAt = postCaptureAt;
    record.durationMs = typeof record.startAt === "number" ? postCaptureAt - record.startAt : null;
    if (phase === "candidate") {
      record.candidateGenerationEvidence = {
        before: { modelGeneration: candidateBefore.model.generation, filter: candidateBefore.viewport.filter, mainRender: candidateBefore.viewport.mainRender },
        after: { modelGeneration: candidateAfter.model.generation, filter: candidateAfter.viewport.filter, mainRender: candidateAfter.viewport.mainRender }
      };
    }
    record.browserObservedCapture = {
      path: path.basename(screenshotPath),
      beforeCaptureAt,
      postCaptureAt,
      captureElapsedUpperBoundMs: postCaptureAt - beforeCaptureAt,
      format: "png",
      scale: "device",
      fullPage: false
    };
    record.publication = "exact query and expected count summary committed, matched observer wait retained, then browser screenshot and post-capture conservative upper bound; no compositor claim";
  }
  const beforeClear = phase === "candidate" ? candidateAfter : null;
  await page.getByTestId("clear-model-tree-filter").click({ timeout: timeoutMs });
  if (phase === "candidate") {
    await page.waitForFunction(({ generation, priorAction, total }) => {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      const filter = snapshot?.viewport?.filter;
      return snapshot?.model?.generation === generation && filter?.generation === generation &&
        filter.actionSequence > priorAction && filter.query === "" && filter.visibleCount === total;
    }, {
      generation: beforeClear.model.generation,
      priorAction: beforeClear.viewport.filter.actionSequence,
      total: totalCount
    }, { timeout: timeoutMs });
  }
  return record;
}

export async function measureOrbit(page: Page, warmupMs: number, measuredMs: number): Promise<any> {
  const mainCanvas = await requireUniqueConnectedMainCanvas(page);
  const box = mainCanvas.box;
  await page.evaluate(() => {
    const state = (globalThis as any).__uifHarness;
    if (!state?.captureGlobalRaf) return;
    state.frameTimes = [];
    state.captureFrames = true;
    state.orbitCaptureStarted = performance.now();
  });
  const start = nodePerformance.now();
  await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.5);
  await page.mouse.down({ button: "left" });
  let index = 0;
  const duration = warmupMs + measuredMs;
  while (nodePerformance.now() - start < duration) {
    const elapsed = nodePerformance.now() - start;
    const x = 0.5 + 0.31 * Math.sin((Math.PI * 2 * elapsed) / 2400);
    const y = 0.5 + 0.23 * Math.sin((Math.PI * 2 * elapsed) / 1700 + Math.PI / 3);
    await page.mouse.move(box.x + box.width * x, box.y + box.height * y);
    index += 1;
    await page.waitForTimeout(Math.max(1, 16 - ((index % 5) === 0 ? 1 : 0)));
  }
  await page.mouse.up({ button: "left" });
  return page.evaluate(({ warmup, expectedMeasured }) => {
    const state = (globalThis as any).__uifHarness;
    if (!state?.captureGlobalRaf) {
      return {
        status: "N/A_GLOBAL_RAF_INSTRUMENTATION_DISABLED",
        warmupMs: warmup,
        requestedMeasuredMs: expectedMeasured,
        capturedCallbackOpportunityCount: 0,
        callbackOpportunityIntervalsMs: [],
        strictMainRenderFrameStatus: "N/A_REQUIRES_CDP_COMPOSITOR_TRACE_BOUND_TO_APP_SURFACE"
      };
    }
    state.captureFrames = false;
    const captureStart = state.orbitCaptureStarted;
    const callbackOpportunities = [...new Set<number>(state.frameTimes.filter((timestamp: number) => timestamp >= captureStart + warmup))];
    const intervals = callbackOpportunities.slice(1).map((timestamp: number, index: number) => timestamp - callbackOpportunities[index]);
    return {
      status: intervals.length > 0 ? "MEASURED_GLOBAL_RAF_CALLBACK_OPPORTUNITY_PROXY" : "MEASUREMENT_FAILED",
      warmupMs: warmup,
      requestedMeasuredMs: expectedMeasured,
      capturedCallbackOpportunityCount: callbackOpportunities.length,
      callbackOpportunityIntervalsMs: intervals,
      strictMainRenderFrameStatus: "N/A_BASELINE_GLOBAL_RAF_IS_NOT_MAIN_RENDER_OR_COMPOSITOR_EVIDENCE",
      pointerMoveCount: state.eventStarts?.pointerdown ? undefined : null
    };
  }, { warmup: warmupMs, expectedMeasured: measuredMs });
}

export function expectedTreeFilterCount(model: any, rawQuery: string): number {
  const query = rawQuery.trim().toLowerCase();
  const items: Array<{ group: string; type: string; values: unknown[] }> = [{
    group: "Project", type: "project", values: [model.project.id, model.project.name, model.project.description, "project", "model"]
  }];
  for (const material of model.materials ?? []) items.push({ group: "Materials", type: "material", values: [material.id, material.label, material.provenance, "material", "materials"] });
  for (const section of model.sections ?? []) items.push({ group: "Sections", type: "section", values: [section.id, section.name, section.section_type, section.provenance, "section", "sections", "pipe section"] });
  for (const node of model.nodes) items.push({ group: "Nodes", type: "node", values: [node.id, node.label, node.provenance, "node", "nodes"] });
  for (const pipe of model.pipe_segments) items.push({ group: "Pipes", type: "pipe", values: [pipe.id, pipe.label, pipe.from, pipe.to, pipe.material, pipe.provenance, "pipe", "pipe segment"] });
  for (const support of model.supports) items.push({ group: "Supports", type: "support", values: [support.id, support.label, support.node, support.family, support.restraints?.join(" "), support.provenance, "support", "supports"] });
  for (const component of model.components) items.push({ group: "Components", type: "component", values: [component.id, component.label, component.kind, component.node, component.provenance, JSON.stringify(component.geometry ?? {}), JSON.stringify(component.modifiers ?? {}), "component", "components"] });
  for (const loadCase of model.load_cases) items.push({ group: "Load Cases", type: "load", values: [loadCase.id, loadCase.label, loadCase.kind, loadCase.status, loadCase.provenance, "load", "load case"] });
  for (const combination of model.combinations ?? []) items.push({ group: "Combinations", type: "combination", values: [combination.id, combination.label, combination.basis, combination.provenance, "combination", "combinations"] });
  if (!query) return items.length;
  return items.filter((item) => [...item.values, item.type, item.group].filter(Boolean).join(" ").toLowerCase().includes(query)).length;
}

export async function settledRafState(page: Page, settleTimeoutMs = 5_000): Promise<any> {
  const started = nodePerformance.now();
  let snapshot: any = null;
  do {
    snapshot = await page.evaluate(() => {
      const state = (globalThis as any).__uifHarness;
      if (!state?.captureGlobalRaf) return { captureGlobalRaf: false };
      return {
        captureGlobalRaf: true,
        pending: state?.pendingRaf?.size ?? null,
        requested: state?.rafRequested ?? null,
        completed: state?.rafCompleted ?? null,
        cancelled: state?.rafCancelled ?? null
      };
    });
    if (snapshot.captureGlobalRaf === false) break;
    if (snapshot.pending === 0) break;
    await page.waitForTimeout(100);
  } while (nodePerformance.now() - started < settleTimeoutMs);
  return {
    ...snapshot,
    status: snapshot.captureGlobalRaf === false
      ? "N/A_HARNESS_DID_NOT_WRAP_APP_RAF"
      : snapshot.pending === 0 ? "PASS" : "FAIL_SCHEDULED_ANIMATION_FRAMES_REMAIN",
    settleWaitMs: nodePerformance.now() - started
  };
}

export function nearestRankP95(values: number[]): number | null {
  if (values.length === 0) return null;
  const sorted = values.slice().sort((a, b) => a - b);
  return sorted[Math.ceil(0.95 * sorted.length) - 1];
}

export async function writeJson(filePath: string, value: unknown): Promise<void> {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

export async function appendJsonLine(filePath: string, value: unknown): Promise<void> {
  await mkdir(path.dirname(filePath), { recursive: true });
  await appendFile(filePath, `${JSON.stringify(value)}\n`);
}

export function requireEvidenceRoot(): string {
  const configured = process.env.UI_FOUNDATION_EVIDENCE_DIR;
  if (!configured || !path.isAbsolute(configured) || !configured.includes(`${path.sep}instances${path.sep}VERIFY${path.sep}`)) {
    throw new Error("UI_FOUNDATION_EVIDENCE_DIR must be an absolute instances/VERIFY/** path");
  }
  return configured;
}

export function benchmarkPhase(): BenchmarkPhase {
  const value = process.env.UI_FOUNDATION_PHASE;
  if (value !== "baseline" && value !== "candidate") {
    throw new Error("UI_FOUNDATION_PHASE must be baseline or candidate");
  }
  return value;
}

export function unsupportedRecords(samples: any[], code: string, detail: string): any[] {
  return samples.map((sample) => ({ ...sample, status: code, durationMs: null, detail }));
}
