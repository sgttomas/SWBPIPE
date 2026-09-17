import { REQUIRED_CHROMIUM_BINDING, PAGE_CLOCK_SOURCE } from "./causal-presentation-extractor.mjs";
import { beginChromiumCompositorTrace, endChromiumCompositorTrace, CHROMIUM_TRACE_RAW_BYTE_LIMIT } from "./chromium-compositor-trace";
import { createHash } from "node:crypto";
import { deflateSync } from "node:zlib";
import { createContext, runInContext } from "node:vm";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { loadFixture, validateCandidateOracleBinding, validatedPriorBoxBaseline, persistBoxPostcondition, boxCallbackMatchesSnapshot, expectedBoxInspectorHeading, pairedWinnerCueWitness, winnerCuePairs, validateWinnerCuePlan, cueActionBindingsStable, cueCameraGeometryMatches, cueCaptureHasNoDrift, pointCaptureMatchesMarker, captureWinnerCue, installInstrumentation, CUE_GEOMETRY_SOURCE_SHA256, CUE_SOURCE_SHA256 } from "./benchmark-harness";
import { expect, test } from "@playwright/test";
import { bindSameTraceDurationBasis, sameTracePresentedGap, constructOrbitEvidence, conservativePresentedGap, expectedModelIdentity, frozenTreeExpectation, selectedEpochEvidence,
  segmentObserverRequirements, FULL_COHORT_RECIPE } from "./full-cohort-controller";

test("frozen controller recipe retains prescribed workload and bounded independent segments", () => {
  expect(FULL_COHORT_RECIPE).toMatchObject({ pointCount: 200, boxCount: 20, filterCount: 20,
    discreteActionsPerTrace: 1, warmupMs: 2000, measuredMs: 10000, rawTraceByteCap: 268435456 });
});
test("tree oracle is independently derived with complete typed order and ARIA groups", () => {
  const model = { project: { id: "p", name: "Project" }, materials: [], sections: [],
    nodes: [{ id: "same", label: "A node" }], pipe_segments: [{ id: "same", label: "A pipe" }],
    supports: [], components: [], load_cases: [], diagnostics: [{ id: "d", code: "INFO", message: "fixture" }] };
  const all = frozenTreeExpectation(model, "");
  expect(all.totalCount).toBe(4);
  expect(all.rows.map((r) => r.testId)).toEqual(["tree-row-project-p", "tree-group-Nodes", "tree-row-node-same",
    "tree-group-Pipes", "tree-row-pipe-same", "tree-group-Diagnostics", "tree-row-diagnostic-d"]);
  const pipe = frozenTreeExpectation(model, "A pipe");
  expect(pipe.visibleCount).toBe(1);
  expect(pipe.rows).toEqual([{ testId: "tree-group-Pipes", label: "Pipes", expanded: true, position: 1, setSize: 1, level: 1 },
    { testId: "tree-row-pipe-same", label: "A pipe", position: 1, setSize: 1, level: 2 }]);
  expect(frozenTreeExpectation(model, "not present").rows).toEqual([]);
  expect(expectedModelIdentity(model)).not.toBe(expectedModelIdentity({ ...model, project: { ...model.project, name: "Other" } }));
});
for (const feedbackKind of ["assignment", "tree-filter"] as const) {
  test(`only one unchanged final DOM epoch can enter the extractor and earlier epochs stay explicit: ${feedbackKind}`, () => {
    const evidence = { active: { feedbackKind, contentProof: { status: "PASS_EXACT_STOPPED_CONTENT", domEpoch: 3 },
      feedbackMarkers: [{ domEpoch: 1, markerIdentity: "old" }, { domEpoch: 3, markerIdentity: "current" }] } };
    const selected = selectedEpochEvidence(evidence);
    expect(selected.active.feedbackMarkers).toEqual([{ domEpoch: 3, markerIdentity: "current" }]);
    expect(selected.excludedInvalidatedContentMarkers).toEqual([{ domEpoch: 1, markerIdentity: "old" }]);
    expect(evidence.active.feedbackMarkers).toHaveLength(2);
    for (const markers of [[], [{ domEpoch: 1 }], [{ domEpoch: 3 }, { domEpoch: 3 }]]) {
      expect(() => selectedEpochEvidence({ active: { ...evidence.active, feedbackMarkers: markers } })).toThrow();
    }
    expect(() => selectedEpochEvidence({ active: { ...evidence.active, contentProof: { status: "FAIL_STOPPED_CONTENT", domEpoch: 3 } } })).toThrow();
  });
}
test("presented gap bounds widen outward and never subtract observer work", () => {
  const gap = conservativePresentedGap({ lower: 100, upper: 100.1 }, { lower: 116.5, upper: 116.8 });
  // Independently derived binary64 dyadic outputs (V20C), not decimal input assumptions.
  expect(gap).toEqual({ lower: 4616189618054759 / 2 ** 48, upper: 4728779608739021 / 2 ** 48 });
  // Halfway subtraction rounds upward: the lower bound must widen downward.
  expect(conservativePresentedGap({ lower: 2 ** -54, upper: 2 ** -54 }, { lower: 1, upper: 1 }))
    .toEqual({ lower: 1 - 2 ** -53, upper: 1 + 2 ** -52 });
  // Halfway subtraction rounds downward: the upper bound must widen upward.
  expect(conservativePresentedGap({ lower: 3 * 2 ** -54, upper: 3 * 2 ** -54 }, { lower: 1, upper: 1 }))
    .toEqual({ lower: 1 - 3 * 2 ** -53, upper: 1 - 2 ** -53 });
  expect(() => conservativePresentedGap({ lower: 101, upper: 100 }, { lower: 110, upper: 111 })).toThrow();
  expect(() => conservativePresentedGap({ lower: 100, upper: 101 }, { lower: 110, upper: Infinity })).toThrow();
});
const cueOracle = { source: "frozen token", sourceSha256: "009b27db887e3de224f72d0df221966fbaaa0305de218ee20848050ac6a1be1b", theme: "light", srgb: [163, 68, 0], sceneSrgb: [223, 229, 232], tolerancePerChannel: 48, requiredLocalContrastRatio: 3, minimumQualifyingInteriorPixels: 4 };
function syntheticCue() {
  const ref = { type: "pipe", id: "winner" }, anchor = { type: "node", id: "anchor" };
  const body = { schema: "winner-cue-pair/v1", sample: 13, expectedHitRef: ref, anchorRef: anchor, authoredAnchor: [0, 0, 0], authoredCenter: [1, 0, 0],
    centerCss: { x: 100.25, y: 100.25 }, clip: { x: 76, y: 76, width: 48, height: 48 }, dpr: 2, centerDevice: { x: 48.5, y: 48.5 },
    cue: { cssSize: 11, interior: .34, outer: .5, erosionDevice: 1, rimSrgb: [255, 255, 255] },
    source: { geometrySourceSha256: CUE_GEOMETRY_SOURCE_SHA256, cueSourceSha256: CUE_SOURCE_SHA256, visualTokensSha256: cueOracle.sourceSha256 } };
  const plan = { ...body, sha256: createHash("sha256").update(JSON.stringify(body)).digest("hex") };
  return { plan, probe: { sample: 13, probe_anchor_ref: anchor, authored_anchor: [0, 0, 0], candidate_runtime: { oracle: { expectedHitRef: ref }, visual_plan: plan } } };
}
function png(rgb: Buffer, width = 96, height = 96): Buffer {
  const chunk = (type: string, data: Buffer) => {
    const t = Buffer.from(type), bytes = Buffer.concat([t, data]); let crc = 0xffffffff;
    for (const byte of bytes) { crc ^= byte; for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0); }
    const head = Buffer.alloc(4), tail = Buffer.alloc(4); head.writeUInt32BE(data.length); tail.writeUInt32BE((crc ^ 0xffffffff) >>> 0);
    return Buffer.concat([head, bytes, tail]);
  };
  const header = Buffer.alloc(13); header.writeUInt32BE(width); header.writeUInt32BE(height, 4); header[8] = 8; header[9] = 2;
  const rows = Buffer.concat(Array.from({ length: height }, (_, y) => Buffer.concat([Buffer.from([0]), rgb.subarray(y * width * 3, (y + 1) * width * 3)])));
  return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]), chunk("IHDR", header), chunk("IDAT", deflateSync(rows)), chunk("IEND", Buffer.alloc(0))]);
}
const pixels = (rgb: number[]) => Buffer.from(Array.from({ length: 96 * 96 }, () => rgb).flat());
const paint = (data: Buffer, index: number, rgb: number[]) => rgb.forEach((v, i) => { data[index * 3 + i] = v; });
test("paired cue permits contrasting rim replacing orange and rejects unchanged/outside changes", () => {
  const { plan, probe } = syntheticCue(), before = pixels(cueOracle.srgb), after = Buffer.from(before);
  for (const pair of winnerCuePairs(plan)) paint(after, pair.rim, [255, 255, 255]);
  const verdict = pairedWinnerCueWitness(png(before), png(after), plan, probe, cueOracle);
  expect(verdict.status).toBe("PASS_PAIRED_WINNER_CUE_TRANSITION"); expect(verdict.netColorGrowthDiagnosticOnly).toBeLessThan(0);
  expect(pairedWinnerCueWitness(png(after), png(after), plan, probe, cueOracle).count).toBe(0);
  const outside = Buffer.from(before); paint(outside, 0, [255,255,255]);
  expect(pairedWinnerCueWitness(png(before), png(outside), plan, probe, cueOracle).status).toBe("FAIL_PAIRED_WINNER_CUE_TRANSITION");
});
test("exact four distinct paired witnesses and actual contrast boundary remain mandatory", () => {
  const { plan, probe } = syntheticCue(), before = pixels([80,80,80]), pairs = winnerCuePairs(plan);
  const image = (count: number, interior = cueOracle.srgb, rim = [255,255,255]) => { const after = Buffer.from(before); for (const pair of pairs.slice(0, count)) { paint(after, pair.interior, interior); paint(after, pair.rim, rim); } return png(after); };
  expect(pairedWinnerCueWitness(png(before), image(4), plan, probe, cueOracle).count).toBe(4);
  expect(pairedWinnerCueWitness(png(before), image(3), plan, probe, cueOracle).status).toBe("FAIL_PAIRED_WINNER_CUE_TRANSITION");
  const bright = [211,116,48], lum = (rgb: number[]) => rgb.map((n) => n / 255).map((n) => n <= .04045 ? n / 12.92 : ((n + .055) / 1.055) ** 2.4).reduce((sum, n, i) => sum + n * [.2126,.7152,.0722][i], 0);
  const edge = Array.from({ length: 49 }, (_, i) => 207 + i).find((g) => (lum([g,g,g]) + .05) / (lum(bright) + .05) >= 3)!;
  expect(edge).toBeGreaterThan(207);
  expect(pairedWinnerCueWitness(png(before), image(4, bright, [edge,edge,edge]), plan, probe, cueOracle).status).toBe("PASS_PAIRED_WINNER_CUE_TRANSITION");
  expect(pairedWinnerCueWitness(png(before), image(4, bright, [edge-1,edge-1,edge-1]), plan, probe, cueOracle).status).toBe("FAIL_PAIRED_WINNER_CUE_TRANSITION");
  for (const imageBytes of [image(4, [0,255,0]), image(4, cueOracle.srgb, [0,0,0])]) expect(pairedWinnerCueWitness(png(before), imageBytes, plan, probe, cueOracle).status).toBe("FAIL_PAIRED_WINNER_CUE_TRANSITION");
});
test("cue plan rejects wrong typed winner anchor camera DPR clip and corrupt PNG mappings", () => {
  const { plan, probe } = syntheticCue(), image = png(pixels(cueOracle.srgb));
  for (const mutate of [(p: any) => { p.expectedHitRef.type = "node"; }, (p: any) => { p.anchorRef.id = "wrong"; },
    (p: any) => { p.camera = { position: [99,0,0] }; }, (p: any) => { p.dpr = 1; }, (p: any) => { p.clip.x++; }, (p: any) => { p.centerDevice.x++; }]) {
    const changed = structuredClone(plan); mutate(changed);
    expect(() => validateWinnerCuePlan(changed, probe)).toThrow();
    const { sha256, ...body } = changed; changed.sha256 = createHash("sha256").update(JSON.stringify(body)).digest("hex");
    expect(() => validateWinnerCuePlan(changed, probe)).toThrow(); // still disagrees with separately frozen oracle plan
  }
  const corrupted = Buffer.from(image); corrupted[corrupted.length - 1] ^= 1;
  for (const bad of [corrupted, image.subarray(0,20), png(pixels([0,0,0]), 48,48)]) expect(pairedWinnerCueWitness(image, bad, plan, probe, cueOracle).status).toBe("FAIL_CUE_PLAN_OR_PNG");
});
test("geometry pairing is deterministic conservative and one-to-one", () => {
  const { plan } = syntheticCue(), pairs = winnerCuePairs(plan), center = plan.centerDevice;
  expect(new Set(pairs.map((p) => p.interior)).size).toBe(pairs.length); expect(new Set(pairs.map((p) => p.rim)).size).toBe(pairs.length);
  const l1 = (i: number) => Math.abs(i % 96 + .5 - center.x) + Math.abs(Math.floor(i / 96) + .5 - center.y);
  for (const pair of pairs) { expect(l1(pair.interior)).toBeLessThanOrEqual(.34 * 22 - 1); expect(l1(pair.rim)).toBeGreaterThanOrEqual(.34 * 22 + 1); expect(l1(pair.rim)).toBeLessThanOrEqual(.5 * 22 - 1); }
  expect(winnerCuePairs(plan)).toEqual(pairs);
  expect(() => winnerCuePairs({ ...plan, centerDevice: { x: -100, y: -100 } })).toThrow();
});
test("real caller observer requirements retain complete observations and DOM zero RAF class", () => {
  const markers = [{ markerIdentity: "old" }, { markerIdentity: "new" }];
  const requirements = segmentObserverRequirements({ active: { token: "filter", feedbackKind: "tree-filter",
    feedbackMarkers: markers, actionMarker: { listenerObservedAt: 50 } } }, "PASS_FULL_RESTORE", null);
  expect(requirements.feedbackObservations).toBe(markers);
  expect(requirements.minimumRafCallbacks).toBe(0);
  expect(requirements).toMatchObject({ actionClass: "tree-filter", expectedOperations:
    [{ operation: "input-capture-listener-total", start: 50, end: 50 }] });
});

test("actual product pre-box unavailable and status-less active union is strict", () => {
  const snapshot = (box: any) => ({ model: { generation: 3 }, viewport: { generation: 3, box } });
  const active = { generation: 3, actionSequence: 2, renderSubmissionSequence: 7, publishedAt: 10,
    direction: "left-to-right", filter: "all", orderedRefs: [], primaryRef: null };
  expect(validatedPriorBoxBaseline(snapshot({ status: "unavailable" }))).toEqual({ kind: "unavailable", actionSequence: 0, generation: 3 });
  expect(validatedPriorBoxBaseline(snapshot(active))).toEqual({ kind: "active", actionSequence: 2, generation: 3 });
  for (const bad of [undefined, null, {}, { status: "active", ...active }, { status: "unknown" },
    { status: "unavailable", actionSequence: 0 }, { status: "unavailable", generation: 3 }, { ...active, generation: 2 },
    ...[undefined, NaN, Infinity, -1, 0, 1.5, Number.MAX_SAFE_INTEGER + 1].map((actionSequence) => ({ ...active, actionSequence }))]) {
    expect(() => validatedPriorBoxBaseline(snapshot(bad))).toThrow();
  }
  expect(() => validatedPriorBoxBaseline({ model: { generation: 3 }, viewport: { generation: 2, box: active } })).toThrow();
});

test("stopped box rejection persists exact fields before throwing and never overwrites evidence", async ({}, testInfo) => {
  const project = { type: "project", id: "p" }, a = { type: "pipe", id: "collision" }, b = { type: "pipe", id: "b" };
  const context = (refs: any[], primary: any) => ({ prior: { kind: "unavailable", actionSequence: 0, generation: 3 }, priorRender: 1,
    projectId: "p", resourceGeneration: 1, expectedInspectorHeading: refs.length > 1 ? `${refs.length} selected items` : `${primary ? "Entity" : "Project"} — ${(primary ?? project).type}: ${(primary ?? project).id}`,
    dom: { readout: `Selected ${(primary ?? project).type}: ${(primary ?? project).id}; 0 queued`,
      inspectorHeading: refs.length > 1 ? `${refs.length} selected items` : `${primary ? "Entity" : "Project"} — ${(primary ?? project).type}: ${(primary ?? project).id}`,
      aggregateCount: refs.length > 1 ? 1 : 0, aggregatePrimary: refs.length > 1 ? `${primary.type}: ${primary.id}` : null }, actionIdentity: { sample: 1 }, canvas: { box: { width: 794, height: 484 } },
    pointerEvents: { pointerup: { listenerObservedAt: 10 } }, expected: { direction: "left-to-right", filter: "all", orderedRefs: refs, primaryRef: primary },
    after: { model: { generation: 3 }, viewport: { generation: 3, camera: { sequence: 5 },
      box: { generation: 3, actionSequence: 1, renderSubmissionSequence: 2, publishedAt: 11, direction: "left-to-right", filter: "all", orderedRefs: refs, primaryRef: primary },
      selection: { generation: 3, actionSequence: 1, publishedAt: 11, renderSubmissionSequence: 2, orderedRefs: refs, primaryRef: primary }, inspector: { generation: 3, publicationSequence: 1, ref: primary ?? project },
      mainRender: { generation: 3, submissionSequence: 2, submittedAt: 12, selectionPresentation: {
        resourceGeneration: 1, modelGeneration: 3, revision: 1, appliedAfterSubmissionSequence: 1, renderedSubmissionSequence: 2, orderedRefs: refs } } } } });
  const model = { project: { id: "p", name: "Project" }, pipe_segments: [{ id: "b", label: "Pipe B" }] };
  expect(expectedBoxInspectorHeading(model, [], null, "p")).toBe("Project — project: p");
  expect(expectedBoxInspectorHeading(model, [b], b, "p")).toBe("Pipe B — pipe: b");
  expect(expectedBoxInspectorHeading(model, [a, b], b, "p")).toBe("2 selected items");
  for (const [i, [refs, primary]] of [[[], null], [[project], project], [[a, b], b]].entries()) {
    const file = testInfo.outputPath(`valid-${i}.json`);
    expect((await persistBoxPostcondition(file, context(refs as any[], primary))).status).toBe("PASS_STOPPED_BOX_POSTCONDITION");
  }
  const bound = context([], null);
  const marker = { observed: { reason: "BOX_RENDERED_SELECTION_READY", failedPredicates: [], mainRenderSubmissionSequence: 2,
    modelGeneration: 3, cameraSequence: 5, actionSequence: 1, selectionActionSequence: 1, selectionRenderSubmissionSequence: 2,
    inspectorPublicationSequence: 1, presentation: { resourceGeneration: 1, modelGeneration: 3, revision: 1,
      appliedAfterSubmissionSequence: 1, renderedSubmissionSequence: 2 } } };
  expect(boxCallbackMatchesSnapshot(marker, bound.after)).toBe(true);
  for (const change of [
    (c: any) => { c.viewport.camera.sequence++; },
    (c: any) => { c.viewport.selection.actionSequence++; },
    (c: any) => { c.viewport.box.actionSequence++; },
    (c: any) => { c.viewport.inspector.publicationSequence++; },
    (c: any) => { c.viewport.mainRender.selectionPresentation.revision++; },
    (c: any) => { c.viewport.mainRender.submissionSequence++; }
  ]) {
    const drift = structuredClone(bound.after); change(drift);
    expect(boxCallbackMatchesSnapshot(marker, drift)).toBe(false);
  }
  const mutations: ((c: any) => void)[] = [
    (c) => { c.after.viewport.selection.primaryRef = project; },
    (c) => { c.after.viewport.inspector.ref = null; },
    (c) => { c.after.viewport.inspector.ref = { type: "project", id: "other" }; },
    (c) => { c.after.viewport.selection.orderedRefs = [project]; c.after.viewport.selection.primaryRef = project; },
    (c) => { c.after.viewport.box.actionSequence = 0; },
    (c) => { c.after.viewport.box.actionSequence = NaN; },
    (c) => { c.prior.actionSequence = 1; },
    (c) => { c.prior.actionSequence = 2; },
    (c) => { c.after.viewport.box.generation = 2; },
    (c) => { c.after.viewport.mainRender.submissionSequence = 1; },
    (c) => { c.after.viewport.box.publishedAt = 9; },
    (c) => { c.after.viewport.mainRender.selectionPresentation = null; },
    (c) => { c.after.viewport.mainRender.selectionPresentation.appliedAfterSubmissionSequence = 2; },
    (c) => { c.after.viewport.mainRender.selectionPresentation.resourceGeneration = 2; },
    (c) => { c.after.viewport.mainRender.selectionPresentation.modelGeneration = 2; },
    (c) => { c.after.viewport.mainRender.selectionPresentation.orderedRefs = [project]; },
    (c) => { c.after.viewport.selection.renderSubmissionSequence = 1; },
    (c) => { c.dom.inspectorHeading = "Wrong — project: p"; },
    (c) => { c.dom.readout = "Selected pipe: p; 0 queued"; }
  ];
  for (const [i, mutate] of mutations.entries()) {
    const c = context([], null); mutate(c);
    const file = testInfo.outputPath(`rejected-${i}.json`);
    await expect(persistBoxPostcondition(file, c)).rejects.toThrow("stopped box postcondition failed");
    const bytes = await readFile(file, "utf8"), saved = JSON.parse(bytes);
    expect(saved.failedFields.length).toBeGreaterThan(0);
    expect(saved.expected).toEqual(c.expected);
    expect(saved.after.model).toEqual(c.after.model);
    expect(saved.after.viewport.camera).toEqual(c.after.viewport.camera);
    expect(saved.after.viewport.box).toEqual(JSON.parse(JSON.stringify(c.after.viewport.box,
      (_key, value) => typeof value === "number" && !Number.isFinite(value) ? { nonFiniteNumber: String(value) } : value)));
    expect(saved.after.viewport.selection).toEqual(c.after.viewport.selection);
    expect(saved.after.viewport.inspector).toEqual(c.after.viewport.inspector);
    await expect(persistBoxPostcondition(file, context([], null))).rejects.toThrow();
    expect(await readFile(file, "utf8")).toBe(bytes);
  }
  for (const [i, mutate] of [
    (c: any) => { c.dom.inspectorHeading = "Pipe B — pipe: b"; },
    (c: any) => { c.dom.inspectorHeading = "3 selected items"; },
    (c: any) => { c.dom.aggregatePrimary = "node: b"; },
    (c: any) => { c.dom.aggregatePrimary = "pipe: wrong"; },
    (c: any) => { c.dom.aggregateCount = 2; }
  ].entries()) {
    const c = context([a, b], b); mutate(c);
    const file = testInfo.outputPath(`aggregate-rejected-${i}.json`);
    await expect(persistBoxPostcondition(file, c)).rejects.toThrow();
    expect(JSON.parse(await readFile(file, "utf8")).failedFields.length).toBeGreaterThan(0);
  }
  const interrupted = { ...context([], null), primaryError: "original wait/capture failure" };
  const interruptedFile = testInfo.outputPath("interrupted.json");
  await expect(persistBoxPostcondition(interruptedFile, interrupted)).rejects.toThrow("original wait/capture failure");
  expect(JSON.parse(await readFile(interruptedFile, "utf8"))).toMatchObject({
    primaryError: interrupted.primaryError, failedFields: [], status: "FAIL_STOPPED_BOX_POSTCONDITION" });
  for (const [i, refs] of [[b], [b, a], [{ type: "node", id: "collision" }, b]].entries()) {
    const c = context([a, b], b); c.after.viewport.selection.orderedRefs = refs;
    const file = testInfo.outputPath(`members-${i}.json`);
    await expect(persistBoxPostcondition(file, c)).rejects.toThrow();
    expect(JSON.parse(await readFile(file, "utf8")).failedFields).toContain("selectionOrderedRefs");
  }
});

const cameraBinding = () => ({ sequence: 7, kind: "perspective", position: [16.232120013664673, 11.814889013664672, 11.182067013664671],
  target: [5.05, .6327689999999999, -5.299999999998448e-5], up: [0,1,0], fovDegrees: 42, near: .001, far: 1000, aspect: 1.640495867768595,
  localRenderOrigin: [5, .629966, 1.5000000000000256e-5] });
function pointBindingFixture() {
  const ref = { type: "pipe", id: "winner" }, pointerDown = { eventKind: "pointerdown", pointerId: 9, listenerObservedAt: 5,
    clientX: 100, clientY: 100, actionIdentity: { sample: 13, expectedRef: ref } };
  const application = { resourceGeneration: 1, modelGeneration: 1, revision: 12, appliedAfterSubmissionSequence: 9, renderedSubmissionSequence: 10, orderedRefs: [ref] };
  const snapshot = { model: { generation: 1 }, viewport: { generation: 1, camera: cameraBinding(), canvas: { dpr: 2 },
    mainRender: { generation: 1, submissionSequence: 10, selectionPresentation: application },
    selection: { generation: 1, actionSequence: 6, orderedRefs: [ref], primaryRef: ref, inputKind: "pointer", pointerDownAt: 6, publishedAt: 7, renderSubmissionSequence: 10 },
    inspector: { generation: 1, ref } } };
  const marker = { token: "point", observed: { modelGeneration: 1, mainRenderSubmissionSequence: 10, selectionRenderSubmissionSequence: 10,
    selectionPresentation: structuredClone(application), actionSequence: 6, cameraSequence: 7, canvasEpoch: 1, contextEpoch: 1,
    selectionCount: 1, orderedRefs: [ref], selectionPrimaryRef: ref, inspectorGeneration: 1, inspectorRef: ref,
    selectionInputKind: "pointer", selectionPointerDownAt: 6, selectionPublishedAt: 7, capturedPointerDownListenerObservedAt: 5, pointerDown } };
  const identity = { valid: true, canvasEpoch: 1, contextEpoch: 1, token: "point", pointerDown };
  const capture = { plan: { sha256: "frozen", camera: cameraBinding() }, before: snapshot, after: structuredClone(snapshot),
    geometry: true, noDrift: true, dpr: 2, identityBefore: identity, identityAfter: structuredClone(identity) };
  return { ref, marker, capture };
}
test("cross-action camera accepts finite sub-nanounit 7 to 8 publication and rejects incomplete or changed geometry", () => {
  const { capture } = pointBindingFixture(), later = structuredClone(capture);
  later.before.viewport.camera.sequence = 8;
  later.before.viewport.camera.position[2] = 11.182067013664673;
  later.before.viewport.camera.target[2] = -5.29999999999845e-5;
  expect(cueActionBindingsStable(capture, later)).toBe(true);
  for (const mutate of [(c: any) => { c.before.viewport.camera.sequence = 6; }, (c: any) => { c.before.viewport.camera.sequence = 1.5; },
    (c: any) => { c.before.viewport.camera.position[2] += 2e-9; }, (c: any) => { c.before.viewport.camera.up = [0,1]; },
    (c: any) => { c.before.viewport.camera.position = []; }, (c: any) => { c.before.viewport.camera.far = Infinity; },
    (c: any) => { delete c.before.viewport.camera.near; }, (c: any) => { c.before.viewport.camera.kind = "orthographic"; },
    (c: any) => { c.before.viewport.camera.localRenderOrigin[0] += 1e-12; }, (c: any) => { c.before.model.generation++; },
    (c: any) => { c.before.viewport.mainRender.selectionPresentation.resourceGeneration++; }, (c: any) => { c.dpr++; },
    (c: any) => { c.before.viewport.canvas.dpr++; }, (c: any) => { c.identityBefore.canvasEpoch++; },
    (c: any) => { c.identityBefore.contextEpoch++; }, (c: any) => { c.noDrift = false; }]) {
    const bad = structuredClone(later); mutate(bad); expect(cueActionBindingsStable(capture, bad)).toBe(false);
  }
  const wrongPlan = structuredClone(capture); wrongPlan.plan.camera.position[0] += 1;
  expect(cueActionBindingsStable(wrongPlan, structuredClone(wrongPlan))).toBe(false);
  expect(cueCameraGeometryMatches({ position: [] }, { position: [] })).toBe(false);
  expect(cueCaptureHasNoDrift(capture.before, capture.after, capture.identityBefore, capture.identityAfter)).toBe(true);
  for (const key of ["camera", "canvas", "selection", "inspector", "mainRender"]) {
    const after: any = structuredClone(capture.before); after.viewport[key].drift = true;
    expect(cueCaptureHasNoDrift(capture.before, after, capture.identityBefore, capture.identityAfter)).toBe(false);
  }
  const tiny = structuredClone(capture.before); tiny.viewport.camera.position[2] += 1e-12;
  expect(cueCaptureHasNoDrift(capture.before, tiny, capture.identityBefore, capture.identityAfter)).toBe(false);
});
test("first applied tuple binds same or later submission only for the exact unchanged application and pointer", () => {
  const { ref, marker, capture } = pointBindingFixture();
  expect(pointCaptureMatchesMarker(marker, capture, ref)).toBe(true);
  const later = structuredClone(capture);
  later.before.viewport.mainRender.submissionSequence = 11;
  later.before.viewport.mainRender.selectionPresentation.renderedSubmissionSequence = 11;
  later.before.viewport.selection.renderSubmissionSequence = 11;
  expect(pointCaptureMatchesMarker(marker, later, ref)).toBe(true);
  for (const mutate of [(c: any) => { c.before.viewport.mainRender.selectionPresentation.revision++; },
    (c: any) => { c.before.viewport.mainRender.selectionPresentation.appliedAfterSubmissionSequence++; },
    (c: any) => { c.before.viewport.mainRender.selectionPresentation.resourceGeneration++; },
    (c: any) => { c.before.viewport.mainRender.selectionPresentation.modelGeneration++; },
    (c: any) => { c.before.viewport.mainRender.selectionPresentation.orderedRefs[0].type = "node"; },
    (c: any) => { c.before.viewport.selection.primaryRef = { type: "node", id: "winner" }; },
    (c: any) => { c.before.viewport.inspector.ref = { type: "pipe", id: "wrong" }; },
    (c: any) => { c.before.viewport.selection.actionSequence++; }, (c: any) => { c.before.viewport.camera.sequence++; },
    (c: any) => { c.before.viewport.selection.pointerDownAt++; }, (c: any) => { c.before.viewport.selection.inputKind = "programmatic"; },
    (c: any) => { c.identityBefore.pointerDown.pointerId++; }, (c: any) => { c.identityBefore.pointerDown.actionIdentity.sample++; },
    (c: any) => { c.identityBefore.token = "other"; }, (c: any) => { c.identityBefore.canvasEpoch++; }, (c: any) => { c.identityBefore.contextEpoch++; },
    (c: any) => { c.before.viewport.mainRender.submissionSequence = c.before.viewport.selection.renderSubmissionSequence = c.before.viewport.mainRender.selectionPresentation.renderedSubmissionSequence = 9; }]) {
    const bad = structuredClone(later); mutate(bad); expect(pointCaptureMatchesMarker(marker, bad, ref)).toBe(false);
  }
  for (const mutate of [(m: any) => { delete m.observed.selectionPresentation; }, (m: any) => { m.observed.selectionPresentation = null; },
    (m: any) => { m.observed.selectionPresentation.appliedAfterSubmissionSequence = 10; }, (m: any) => { m.observed.selectionPresentation.appliedAfterSubmissionSequence = -1; },
    (m: any) => { m.observed.selectionPresentation.revision = NaN; }, (m: any) => { m.observed.selectionPresentation.renderedSubmissionSequence = 11; },
    (m: any) => { m.observed.selectionPresentation.orderedRefs = []; }, (m: any) => { m.observed.selectionRenderSubmissionSequence = 9; }]) {
    const bad = structuredClone(marker); mutate(bad); expect(pointCaptureMatchesMarker(bad, later, ref)).toBe(false);
  }
});

// Execute the actual installed observer and capture entry points in a deterministic DOM/GL host.
// This is an offline wiring control, not a real-browser/framebuffer qualification.
test("actual first callback copies application and capture rejects replacement drift or later backfill", async ({}, testInfo) => {
  for (const variant of ["same", "redraw", "missing-first", "canvas-replaced", "context-replaced", "during-capture", "noncausal"]) {
    const { ref } = pointBindingFixture(), { plan: basePlan, probe } = syntheticCue();
    const camera = cameraBinding(), canvasFacts = { cssLeft: 0, cssTop: 0, cssWidth: 400, cssHeight: 300, dpr: 2, bufferWidth: 800, bufferHeight: 600 };
    const { sha256: ignored, ...body } = basePlan;
    const plan = { ...body, camera, canvas: canvasFacts, sha256: "" };
    const { sha256: ignored2, ...planBody } = plan; plan.sha256 = createHash("sha256").update(JSON.stringify(planBody)).digest("hex");
    probe.candidate_runtime.visual_plan = plan;
    let clock = 0, nextRaf = 0; const raf = new Map<number, Function>(), listeners = new Map<string, Function>();
    class Element {
      isConnected = true; tagName = "DIV"; textContent = "";
      contains(value: any) { return value === canvas; }
      closest() { return host; }
      getAttribute() { return "viewport-canvas"; }
    }
    class GL { COLOR_BUFFER_BIT = 16384; clear(_mask: number) {} isContextLost() { return false; } }
    let gl = new GL();
    class Canvas extends Element { tagName = "CANVAS"; getContext() { return gl; } }
    let canvas = new Canvas(); const host = new Element(), readout = new Element(), heading = new Element();
    readout.textContent = "Selected pipe: winner; 0 queued"; heading.textContent = "Winner — pipe: winner";
    const select = (selector: string) => selector.includes("command-selection") ? [readout] : selector.includes("h2") ? [heading] :
      selector.includes("canvas") ? [selector.endsWith("canvas") ? canvas : host] : [];
    let snapshot: any, captureMayDrift = false;
    const frozen = (value: any): any => { if (value && typeof value === "object") { Object.values(value).forEach(frozen); Object.freeze(value); } return value; };
    const realm: any = { Element, HTMLElement: Element, HTMLCanvasElement: Canvas, HTMLInputElement: Element,
      PointerEvent: class {}, MutationObserver: class { observe() {} disconnect() {} takeRecords() { return []; } },
      performance: { now: () => ++clock / 10, timeOrigin: 1000 }, console: { timeStamp() {} }, devicePixelRatio: 2,
      document: { querySelectorAll: select, querySelector: (s: string) => select(s)[0] ?? null },
      addEventListener: (type: string, fn: Function) => listeners.set(type, fn), removeEventListener() {},
      requestAnimationFrame: (fn: Function) => { raf.set(++nextRaf, fn); return nextRaf; }, cancelAnimationFrame: (id: number) => raf.delete(id) };
    realm.window = realm; const context = createContext(realm);
    Object.defineProperty(realm, "__openPipeStressUiDiagnosticsV1", { value: Object.freeze({ schema: "openpipestress.ui-diagnostics/v1", readCurrent: () => snapshot, projectAuthoredPoint() {} }) });
    const evaluate = (fn: Function, arg?: any) => { realm.__arg = arg; return runInContext(`(${fn.toString()})(__arg)`, context); };
    const page: any = { addInitScript: async (fn: Function, arg: any) => evaluate(fn, arg), evaluate: async (fn: Function, arg: any) => structuredClone(evaluate(fn, arg)),
      locator: () => ({ count: async () => 1, boundingBox: async () => ({ x: 0, y: 0, width: 400, height: 300 }),
        evaluate: async (fn: Function, arg: any) => { realm.__element = canvas; realm.__arg = arg; return runInContext(`(${fn.toString()})(__element,__arg)`, context); } }),
      screenshot: async ({ path }: any) => { await writeFile(path, png(pixels(cueOracle.srgb))); if (variant === "during-capture" && captureMayDrift) {
        const changed = structuredClone(snapshot); changed.viewport.camera.position[2] += 1e-12; snapshot = frozen(changed);
      } } };
    const initial = pointBindingFixture().capture.before; initial.viewport.camera = camera; initial.viewport.canvas = canvasFacts;
    snapshot = frozen(initial);
    await installInstrumentation(page, { causalFeedbackMarkers: variant !== "noncausal" });
    const causal = realm.__uifHarness.causal;
    // Both the first preflight and ordinary helper capture before arm; the ordinary route never arms.
    expect(causal.canvasEpoch).toBe(0); expect(causal.contextEpoch).toBe(0);
    const beforeArm = await captureWinnerCue(page, probe, testInfo.outputPath(`${variant}-before-arm.png`));
    expect(beforeArm.identityBefore).toMatchObject({ valid: true, canvasEpoch: 1, contextEpoch: 1, token: null, pointerDown: null });
    expect(raf.size).toBe(0);
    if (variant === "noncausal") {
      const afterOrdinary = await captureWinnerCue(page, probe, testInfo.outputPath("noncausal-after.png"));
      expect(cueActionBindingsStable(beforeArm, afterOrdinary)).toBe(true);
      expect(causal.active).toBe(null); expect(raf.size).toBe(0);
      continue;
    }
    causal.arm({ token: "point", phase: "candidate", feedbackKind: "point-selection", actionStartEvent: "pointerdown", expectedActionTargetTestId: "viewport-canvas",
      maximumFeedbackMarkers: 1, actionIdentity: { sample: 13, expectedRef: ref }, candidateExpectation: { modelGeneration: 1, priorRenderSubmissionSequence: 9,
        priorActionSequence: 5, expectedRef: ref, expectedInspectorHeading: heading.textContent } });
    listeners.get("pointerdown")!({ target: canvas, pointerId: 9, clientX: 100, clientY: 100, timeStamp: 1 });
    const state = pointBindingFixture().capture.before; state.viewport.camera = camera; state.viewport.canvas = canvasFacts;
    state.viewport.selection.pointerDownAt = causal.active.pointerDown.listenerObservedAt + 1;
    state.viewport.selection.publishedAt = state.viewport.selection.pointerDownAt + 1;
    if (variant === "missing-first") delete (state.viewport.mainRender as any).selectionPresentation;
    snapshot = frozen(state);
    realm.requestAnimationFrame(() => gl.clear(gl.COLOR_BUFFER_BIT));
    const callback = raf.get(nextRaf)!; callback(clock);
    expect(causal.active.feedbackMarkers).toHaveLength(1);
    const marker = structuredClone(causal.active.feedbackMarkers[0]);
    expect(marker.observed.selectionPresentation?.renderedSubmissionSequence ?? null).toBe(variant === "missing-first" ? null : 10);
    const next = structuredClone(snapshot);
    next.viewport.mainRender.selectionPresentation = structuredClone(pointBindingFixture().capture.before.viewport.mainRender.selectionPresentation);
    if (variant !== "same") next.viewport.mainRender.submissionSequence = next.viewport.selection.renderSubmissionSequence = next.viewport.mainRender.selectionPresentation.renderedSubmissionSequence = 11;
    snapshot = frozen(next);
    // A subsequent real callback must not replace the saved first marker or its copied tuple.
    realm.requestAnimationFrame(() => gl.clear(gl.COLOR_BUFFER_BIT)); raf.get(nextRaf)!(clock);
    expect(causal.active.feedbackMarkers).toHaveLength(1); expect(causal.active.feedbackMarkers[0]).toEqual(marker);
    causal.stop("point");
    if (variant === "canvas-replaced") canvas = new Canvas();
    if (variant === "context-replaced") gl = new GL();
    const file = testInfo.outputPath(`${variant}.png`); captureMayDrift = true;
    if (["during-capture", "context-replaced"].includes(variant)) {
      await expect(captureWinnerCue(page, probe, file)).rejects.toThrow("capture state drift");
      expect(JSON.parse(await readFile(`${file}.json`, "utf8")).noDrift).toBe(false);
    } else {
      const capture = await captureWinnerCue(page, probe, file);
      expect(pointCaptureMatchesMarker(marker, capture, ref)).toBe(["same", "redraw"].includes(variant));
      expect(cueActionBindingsStable(beforeArm, capture)).toBe(variant !== "canvas-replaced");
    }
  }
});

test("winner plan requires geometry provenance even when the altered plan is rehashed and self-consistent", () => {
  for (const hash of [undefined, "stale-geometry-source"]) {
    const { plan, probe } = syntheticCue(); const altered: any = structuredClone(plan);
    altered.source.geometrySourceSha256 = hash;
    const { sha256, ...body } = altered; altered.sha256 = createHash("sha256").update(JSON.stringify(body)).digest("hex");
    probe.candidate_runtime.visual_plan = altered;
    expect(() => validateWinnerCuePlan(altered, probe)).toThrow("independent winner cue plan binding");
  }
});
test("cue at the wrong attachment location cannot qualify the frozen winner center", () => {
  const { plan, probe } = syntheticCue(), before = pixels([80,80,80]), after = Buffer.from(before);
  const misplaced = { ...plan, centerDevice: { x: plan.centerDevice.x + 30, y: plan.centerDevice.y } };
  for (const pair of winnerCuePairs(misplaced)) { paint(after, pair.interior, cueOracle.srgb); paint(after, pair.rim, [255,255,255]); }
  expect(pairedWinnerCueWitness(png(before), png(after), plan, probe, cueOracle).status).toBe("FAIL_PAIRED_WINNER_CUE_TRANSITION");
});

// Actual ReturnAsStream transport and real file writes; only CDP/page are mocked.
async function mockBoundedTrace(file: string, length: number) {
  const prefix = Buffer.from('{"traceEvents":[{"name":"TimeStamp","ts":1,"pid":1,"tid":1,"args":{"data":{"message":"BOUND:TRACE_STARTED"}}},{"name":"TimeStamp","ts":2,"pid":1,"tid":1,"args":{"data":{"message":"BOUND:TRACE_END"}}},{"name":"SubmitCompositorFrame","ts":2,"pid":1,"tid":1,"args":{"frame":9007199254740993}}]');
  const callbacks = new Map<string, Function>(), expectedDigest = createHash("sha256");
  let offset = 0, closed = false, detached = false;
  const session: any = { on() {}, once: (name: string, callback: Function) => callbacks.set(name, callback), detach: async () => { detached = true; },
    send: async (method: string, args: any) => {
      if (method === "Tracing.getCategories") return { categories: ["benchmark","cc","devtools.timeline","viz","disabled-by-default-devtools.timeline.frame"] };
      if (method === "Tracing.start") { expect(args.transferMode).toBe("ReturnAsStream"); return {}; }
      if (method === "Tracing.end") { callbacks.get("Tracing.tracingComplete")!({ stream: "mock", dataLossOccurred: false }); return {}; }
      if (method === "IO.close") { closed = true; return {}; }
      if (method !== "IO.read") throw new Error(`unexpected CDP ${method}`);
      expect(args.size).toBe(1_048_576);
      const end = Math.min(offset + args.size, length), bytes = Buffer.alloc(end - offset, 32);
      if (offset === 0) prefix.copy(bytes);
      if (end === length) bytes[bytes.length - 1] = 125; // final JSON brace only at true EOF
      if (end <= CHROMIUM_TRACE_RAW_BYTE_LIMIT) expectedDigest.update(bytes);
      offset = end;
      return { data: bytes.toString("base64"), base64Encoded: true, eof: end === length };
    } };
  const page: any = { context: () => ({ newCDPSession: async () => session }), evaluate: async () => 10 };
  const capture = await beginChromiumCompositorTrace(page, "BOUND", file, "discrete-segment");
  const result = await endChromiumCompositorTrace(page, capture);
  return { capture, result, expectedSha: expectedDigest.digest("hex"), closed, detached, receivedBytes: offset };
}
test("actual ReturnAsStream beyond former64MiB preserves EOF exact hash and unsafe identifiers", async ({}, info) => {
  const length = 64 * 1024 * 1024 + 4096, file = info.outputPath("over-old-cap.json");
  const outcome = await mockBoundedTrace(file, length);
  expect(CHROMIUM_TRACE_RAW_BYTE_LIMIT).toBe(268435456);
  expect(outcome.result.status).toBe("PREFLIGHT_EVENT_INVENTORY_ONLY_NO_ORBIT_PASS");
  expect(outcome.result.rawTraceTransport).toMatchObject({ rawCompleteThroughEof: true, rawUtf8Bytes: length,
    receivedCompleteChunkBytes: length, rawByteLimit: CHROMIUM_TRACE_RAW_BYTE_LIMIT, ioDeadlineMs: 30000,
    rawSha256: outcome.expectedSha, receivedCompleteChunkSha256: outcome.expectedSha, unsafeIntegerTokensConvertedToDecimalStrings: 1 });
  expect(outcome.capture.events.at(-1).args.frame).toBe("9007199254740993");
  expect(createHash("sha256").update(await readFile(file)).digest("hex")).toBe(outcome.expectedSha);
  expect(outcome.closed && outcome.detached).toBe(true);
});
test("actual ReturnAsStream beyond fixed256MiB fails closed preserving exact bounded prefix", async ({}, info) => {
  const file = info.outputPath("over-new-cap-prefix.json"), outcome = await mockBoundedTrace(file, CHROMIUM_TRACE_RAW_BYTE_LIMIT + 1);
  expect(outcome.result.status).toBe("FAIL_PREFLIGHT_TRACE_CAPTURE_OR_LOSSLESS_PARSE");
  expect(outcome.result.traceCompletionError).toContain(`exceeds ${CHROMIUM_TRACE_RAW_BYTE_LIMIT} byte limit`);
  expect(outcome.result.rawTraceTransport).toMatchObject({ rawCompleteThroughEof: false, rawUtf8Bytes: CHROMIUM_TRACE_RAW_BYTE_LIMIT,
    receivedCompleteChunkBytes: CHROMIUM_TRACE_RAW_BYTE_LIMIT, rawSha256: outcome.expectedSha, receivedCompleteChunkSha256: outcome.expectedSha, ioDeadlineMs: 30000 });
  expect(outcome.capture.events).toEqual([]);
  expect(createHash("sha256").update(await readFile(file)).digest("hex")).toBe(outcome.expectedSha);
  expect(outcome.receivedBytes).toBe(CHROMIUM_TRACE_RAW_BYTE_LIMIT + 1);
  expect(outcome.closed && outcome.detached).toBe(true);
});

test("tree oracle restores accepted project restraint provenance and component vocabulary", () => {
  const model: any = { project: { id: "p", name: "P", description: "unique-description" },
    materials: [{ id: "m", label: "M", provenance: "unique-material-origin" }],
    sections: [{ id: "s", name: "S", section_type: "unique-section", provenance: { source: "unique-structured-origin" } }],
    nodes: [{ id: "n", label: "N", provenance: "unique-node-origin" }],
    pipe_segments: [{ id: "e", label: "E", material: "unique-material-ref", provenance: "unique-pipe-origin" }],
    supports: [{ id: "h", label: "H", restraints: ["UX"], hanger: { hanger_type: "unique-hanger", source_reference: "unique-hanger-ref", mechanics_consumption: "unique-hanger-consumption" }, provenance: "unique-support-origin" }],
    components: [{ id: "c", label: "C", geometry: { bend_radius: { value: 1.25, unit: "radiusunit" }, bend_geometry_source_reference: "unique-bend-ref", center_of_gravity: { x: 1, y: 2, z: 3, unit: "cgunit" } }, modifiers: { axial_stiffness_user_value: { value: 12, unit: "stiffnessunit" }, source_reference: "unique-modifier-ref" }, mechanics_interface: { solver_consumption: "unique-solver" }, completeness: [{ diagnostic_code: "unique-finding", status: "pending" }], provenance: "unique-component-origin" }],
    load_cases: [{ id: "l", label: "L", status: "unique-status", provenance: "unique-load-origin" }],
    combinations: [{ id: "b", label: "B", basis: "unique-basis", provenance: "unique-combination-origin" }],
    diagnostics: [{ id: "d", code: "D", message: "unique-message" }] };
  const cases: [string, string][] = [
    ["unique-description", "project-p"], ["model", "project-p"], ["unique-material-origin", "material-m"],
    ["unique-section", "section-s"], ["unique-structured-origin", "section-s"], ["pipe section", "section-s"],
    ["unique-node-origin", "node-n"], ["unique-material-ref", "pipe-e"], ["pipe segment", "pipe-e"], ["unique-pipe-origin", "pipe-e"],
    ["UX", "support-h"], ["unique-hanger", "support-h"], ["unique-hanger-ref", "support-h"], ["unique-hanger-consumption", "support-h"], ["unique-support-origin", "support-h"],
    ["1.25 radiusunit", "component-c"], ["unique-bend-ref", "component-c"], ["1 2 3 cgunit", "component-c"], ["12 stiffnessunit", "component-c"], ["unique-modifier-ref", "component-c"], ["unique-solver", "component-c"], ["unique-finding pending", "component-c"], ["unique-component-origin", "component-c"],
    ["load case", "load-l"], ["unique-status", "load-l"], ["unique-load-origin", "load-l"], ["unique-basis", "combination-b"], ["unique-combination-origin", "combination-b"], ["unique-message", "diagnostic-d"]];
  for (const [query, expected] of cases) {
    const result = frozenTreeExpectation(model, `  ${query.toUpperCase()}  `);
    expect(result.rows.filter(r => r.testId.startsWith("tree-row-")).map(r => r.testId), query).toEqual([`tree-row-${expected}`]);
    expect(result.visibleCount).toBe(1);
    expect(frozenTreeExpectation(model, `${query} absent-suffix`).visibleCount).toBe(0);
  }
});
// Frozen before repair by ROOT: TREE_QUERY_EXPECTATIONS_V30 manifest
// d61b6dd86718aeafb19e1960b749f646fe9782f3081c96c7050c1d3696053349.
const frozenQueryMemberships: [number, number, string, number, string][] = [
  [1000, 1, "UI benchmark pipe 00001", 1, "9a30796bec4ef2d1e8f9c56849c0e316e820b456da3451036ef52d02e13cdfb0"],
  [1000, 2, "UI benchmark pipe 00100", 1, "bb1c15a91dc23736d0d1615fa7cb284b4a1ea5d0f282465065ddb009529401b2"],
  [1000, 3, "UI benchmark pipe 00500", 1, "1a503c99ebf41ce0433147ef7738029953ea1e26b01587297dd8f5597dd4eb07"],
  [1000, 4, "UI benchmark node 00001", 1, "234d27b9429cbb815c3b93fa8f6891c9617e1f5d62d580ee183c5591a8337f1c"],
  [1000, 5, "UI benchmark node 00100", 1, "de59820ef55b27c11b6b356c41f3a6b32e137d87dba6e8f15c1ba954724d1010"],
  [1000, 6, "UI benchmark node 00500", 1, "91076fa335b246685ba2c7e01c639b9b1a3a88cbe0b2f268df0b1cb241cc3651"],
  [1000, 7, "UI benchmark support", 50, "e9e674eed2ff3b21c6be4ff137b1c8792f470e2e22bb25c13526918f385c819a"],
  [1000, 8, "UI benchmark valve", 10, "91d680fe307aa57f46a5b280b0b5528e3b688bdcdb9cdbfb02cd9d7447cde8bc"],
  [1000, 9, "section:UIF-OD-060", 1, "7b8212f8ae702e419c883990a6fda051d15740ef36bf06a2407d979ecbc8f085"],
  [1000, 10, "section:UIF-OD-180", 1, "5e3880d1296952d6e1ff0e41fb0740c896ad67ab66984bc101e7d80c436bbd74"],
  [1000, 11, "pipe:UIF-00077", 1, "d9929469c43630ca9f2cf9f70b7906e64071e0734e64ec30c5bf76e93ddf4710"],
  [1000, 12, "node:UIF-00088", 3, "cc39f81f9ba6af5d6db1b3c68b2fe1f9cc63c33d9ef8aeafb418807f8ec1c3e1"],
  [1000, 13, "support:UIF-00120", 1, "339ea5b93ae77b4d1c7633639201eb34020a939fdaacdb126324dad7b0640c29"],
  [1000, 14, "component:UIF-00200", 1, "575bd1827dc899128a3745239f3b3bd4c3e0fa4e09fed9b8530317dc52eda85f"],
  [1000, 15, "generated UI nodal arrow", 1, "c7c9338f4d83c05e9750fc05baba9680c6d70848b61e03f3fd5b1e617968eee7"],
  [1000, 16, "invented OD 90", 1, "581aa790ca089cdc09961e974f87895743f802e2a06f7434ac364e0287b0470a"],
  [1000, 17, "invented elastic material", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  [1000, 18, "no-match-ui-foundation", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  [1000, 19, "UIF-00", 2058, "b9999a66cfd35261f5d38273c12142c1273abf0538574d54494cc8a7196c054d"],
  [1000, 20, "generated ui", 2, "e6806950b139f6852346c75aa91fe24f91a0e043730cbe3b2ca762ad87309542"],
  [10000, 1, "UI benchmark pipe 00001", 1, "9a30796bec4ef2d1e8f9c56849c0e316e820b456da3451036ef52d02e13cdfb0"],
  [10000, 2, "UI benchmark pipe 00100", 1, "bb1c15a91dc23736d0d1615fa7cb284b4a1ea5d0f282465065ddb009529401b2"],
  [10000, 3, "UI benchmark pipe 00500", 1, "1a503c99ebf41ce0433147ef7738029953ea1e26b01587297dd8f5597dd4eb07"],
  [10000, 4, "UI benchmark node 00001", 1, "234d27b9429cbb815c3b93fa8f6891c9617e1f5d62d580ee183c5591a8337f1c"],
  [10000, 5, "UI benchmark node 00100", 1, "de59820ef55b27c11b6b356c41f3a6b32e137d87dba6e8f15c1ba954724d1010"],
  [10000, 6, "UI benchmark node 00500", 1, "91076fa335b246685ba2c7e01c639b9b1a3a88cbe0b2f268df0b1cb241cc3651"],
  [10000, 7, "UI benchmark support", 500, "92a5dc10b1d293c55a571615706023df43848c069e5c778afe9c6871c1836c20"],
  [10000, 8, "UI benchmark valve", 100, "c2c4ec1db718366604a06b2cc5320d9ddfa6ca1e485ce70cc710c10cf7d2dd7a"],
  [10000, 9, "section:UIF-OD-060", 1, "7b8212f8ae702e419c883990a6fda051d15740ef36bf06a2407d979ecbc8f085"],
  [10000, 10, "section:UIF-OD-180", 1, "5e3880d1296952d6e1ff0e41fb0740c896ad67ab66984bc101e7d80c436bbd74"],
  [10000, 11, "pipe:UIF-00077", 1, "d9929469c43630ca9f2cf9f70b7906e64071e0734e64ec30c5bf76e93ddf4710"],
  [10000, 12, "node:UIF-00088", 3, "cc39f81f9ba6af5d6db1b3c68b2fe1f9cc63c33d9ef8aeafb418807f8ec1c3e1"],
  [10000, 13, "support:UIF-00120", 1, "339ea5b93ae77b4d1c7633639201eb34020a939fdaacdb126324dad7b0640c29"],
  [10000, 14, "component:UIF-00200", 1, "575bd1827dc899128a3745239f3b3bd4c3e0fa4e09fed9b8530317dc52eda85f"],
  [10000, 15, "generated UI nodal arrow", 1, "c7c9338f4d83c05e9750fc05baba9680c6d70848b61e03f3fd5b1e617968eee7"],
  [10000, 16, "invented OD 90", 1, "581aa790ca089cdc09961e974f87895743f802e2a06f7434ac364e0287b0470a"],
  [10000, 17, "invented elastic material", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  [10000, 18, "no-match-ui-foundation", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  [10000, 19, "UIF-00", 2058, "b9999a66cfd35261f5d38273c12142c1273abf0538574d54494cc8a7196c054d"],
  [10000, 20, "generated ui", 2, "5899202cc9fa8b0b45d327b294b6981c2fd1a5e188f3d84053378fbcdb7ca9bf"]
];

test("tree oracle matches all 40 independently frozen ordered query memberships", async () => {
  for (const size of [1000, 10000]) {
    const model = JSON.parse(await readFile(new URL(`./fixtures/ui-foundation-${size}.model.json`, import.meta.url), "utf8"));
    const samples = JSON.parse(await readFile(new URL(`./samples/ui-foundation-${size}.interactions.json`, import.meta.url), "utf8"));
    expect(samples.tree_filters).toHaveLength(20);
    for (const [n, sample, query, count, hash] of frozenQueryMemberships.filter(row => row[0] === size)) {
      expect(samples.tree_filters.find((q: any) => q.sample === sample).query).toBe(query);
      const result = frozenTreeExpectation(model, query);
      const ids = result.rows.filter(r => r.testId.startsWith("tree-row-")).map(r => r.testId);
      expect(result.visibleCount, `${n}/${sample}/${query}`).toBe(count);
      expect(createHash("sha256").update(JSON.stringify(ids)).digest("hex"), `${n}/${sample}/${query}`).toBe(hash);
    }
  }
});

test("runtime geometry pin rejects old and mutated plan sources after valid selfhash rebinding", () => {
  expect(CUE_GEOMETRY_SOURCE_SHA256).toBe("c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e");
  for (const hash of ["97b18c9671fc7f98e1cbb94bf6833f737c91e3ef4580d02c711d306c82486aee", "0".repeat(64)]) {
    const { probe, plan } = syntheticCue(); plan.source.geometrySourceSha256 = hash;
    const { sha256: ignored, ...body } = plan; plan.sha256 = createHash("sha256").update(JSON.stringify(body)).digest("hex");
    probe.candidate_runtime.visual_plan = plan;
    expect(() => validateWinnerCuePlan(plan, probe)).toThrow("independent winner cue plan binding");
  }
});

// Self-contained admission controls: maintained fixtures/source plus test-owned manifests.
// Split verification checkouts may explicitly set the existing candidate source root.
test("runtime geometry pin checks self-contained manifests and actual disk source", async ({}, testInfo) => {
  const methodRoot = new URL("./", import.meta.url);
  const sourceRoot = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT ?? new URL("../../../../", import.meta.url).pathname;
  const hash = (bytes: string | Buffer) => createHash("sha256").update(bytes).digest("hex");
  const geometryPath = `${sourceRoot}/apps/desktop/src/features/viewport/viewportSelection.ts`;
  const cuePath = `${sourceRoot}/apps/desktop/src/features/viewport/viewportSelectionPresentation.ts`;
  const geometry = await readFile(geometryPath);
  expect(hash(geometry)).toBe(CUE_GEOMETRY_SOURCE_SHA256);
  expect(hash(await readFile(cuePath))).toBe(CUE_SOURCE_SHA256);
  const dependencies = await Promise.all(["freeze-candidate-point-oracle.mjs", "point-hit-oracle.mjs", "box-selection-oracle.mjs"]
    .map(async name => ({ path: name, sha256: hash(await readFile(new URL(name, methodRoot))) })));
  const priorDir = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR, priorHash = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256;
  try {
    // Read only maintained baseline fixtures; external oracle configuration cannot select inputs.
    delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
    for (const size of [1000, 10000]) {
      const fixture = await loadFixture(size);
      const camera = { fixture: "synthetic-camera" }, canvas = { fixture: "synthetic-canvas" }, preflightSha256 = hash("synthetic-preflight");
      const { probe, plan } = syntheticCue();
      const body: any = { ...plan, camera, canvas, source: { ...plan.source, dependencies,
        modelSha256: hash(fixture.bytes), sampleSha256: hash(await readFile(fixture.samplesPath)), cameraPreflightSha256: preflightSha256 } };
      delete body.sha256;
      const checkedPlan = { ...body, sha256: hash(JSON.stringify(body)) };
      const checkedProbe = { ...probe, candidate_runtime: { ...probe.candidate_runtime, visual_plan: checkedPlan } };
      const oracle = { probes: [checkedProbe], candidate_preflight: { preflightSha256, camera, rawCanvasReadback: canvas } };
      const oracleBytes = JSON.stringify(oracle), oracleName = `ui-foundation-${size}.candidate-runtime-point-oracle-v3.json`;
      for (const mode of ["accepted-copy", "old-declaration", "mutated-disk", "mutated-declaration-and-disk"]) {
        const dir = testInfo.outputPath(`${size}-${mode}`); await mkdir(dir, { recursive: true });
        const sourcePath = `${dir}/viewportSelection.ts`;
        const bytes = mode.includes("mutated") ? Buffer.concat([geometry, Buffer.from("\n// mutation\n")]) : geometry;
        await writeFile(sourcePath, bytes, { flag: "wx" });
        const manifest = { status: "PASS_ALL_200_ACTIONABLE_AND_PRODUCT_PROJECTION_CROSSCHECKED",
          cueSourcePath: cuePath, cueSourceSha256: CUE_SOURCE_SHA256, geometrySourcePath: sourcePath,
          geometrySourceSha256: mode === "old-declaration" ? "97b18c9671fc7f98e1cbb94bf6833f737c91e3ef4580d02c711d306c82486aee"
            : mode === "mutated-declaration-and-disk" ? hash(bytes) : CUE_GEOMETRY_SOURCE_SHA256,
          dependencies, boxSelectionPolicySha256: "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310",
          files: [{ pipeCount: size, path: oracleName, sha256: hash(oracleBytes), actionableCount: 200, boxSampleCount: 20 }] };
        const manifestBytes = JSON.stringify(manifest);
        await writeFile(`${dir}/${oracleName}`, oracleBytes, { flag: "wx" });
        await writeFile(`${dir}/CANDIDATE_POINT_ORACLE_MANIFEST.json`, manifestBytes, { flag: "wx" });
        process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR = dir;
        process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 = hash(manifestBytes);
        const input = { ...fixture, pointOracle: oracle, pointOraclePath: `${dir}/${oracleName}`, pointOracleSha256: hash(oracleBytes) };
        if (mode === "accepted-copy") { await validateCandidateOracleBinding(input, size); validateWinnerCuePlan(checkedPlan, checkedProbe); }
        else await expect(validateCandidateOracleBinding(input, size)).rejects.toThrow("winner product geometry/cue source drift");
      }
      delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
    }
  } finally {
    if (priorDir === undefined) delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR; else process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR = priorDir;
    if (priorHash === undefined) delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256; else process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 = priorHash;
  }
});

test("orbit constructor preserves full qualified sequence and unchanged interval gaps", () => {
  const actionAt = 1000, times = [2990,2999,3000.062,5000,9000,13000.062,13001,13010];
  const results = times.map(t => ({ presentationTraceTimestamp: t * 1000,
    pageToTraceOffsetIntervalMs: { minimum: 0, maximum: 0 },
    actionToPresentationIntervalMs: { lower: t - .125 - actionAt, upper: t + .125 - actionAt } }));
  const input = syntheticTraceInput(results, actionAt), basis = bindSameTraceDurationBasis(input);
  const boundResults = input.extraction.presentations, before = structuredClone(boundResults), evidence = constructOrbitEvidence(boundResults, actionAt, "centerline", basis);
  expect(evidence.envelope).toEqual({ first: 1, last: 6 });
  expect(evidence.endpoints.map(p=>p.sourceIndex)).toEqual([0,1,2,3,4,5,6,7]);
  expect(evidence.endpoints.map(p=>p.reportedTimestamp)).toEqual(times.map(t=>t*1000));
  expect(evidence.gaps).toHaveLength(5);
  evidence.gaps.forEach((g,i)=> {
    expect(g.fromSourceIndex).toBe(i+1);expect(g.toSourceIndex).toBe(i+2);
    expect(g.durationIntervalMs).toEqual(sameTracePresentedGap(evidence.endpoints[i+1].reportedTimestamp,evidence.endpoints[i+2].reportedTimestamp));
  });
  expect(evidence.warmup).toEqual({startMs:1000,endMs:3000});
  expect(evidence.measured).toEqual({startMs:3000,endMs:13000});expect(boundResults).toEqual(before);
  expect(()=>constructOrbitEvidence(boundResults.map((r:any)=>({...r,pageToTraceOffsetIntervalMs:{minimum:1,maximum:0}})),actionAt,"centerline",basis)).toThrow("no common source-bound clock mapping");
});

function syntheticTraceInput(rows: any[], actionAt=1000) {
  const token="synthetic-bound-orbit", frame="document-A", events:any[]=[{name:"TimeStamp",ts:1000000,pid:41,tid:7,args:{data:{message:`UIF_CAUSAL_V1:ACTION:${token}`,frame}}}];
  const feedbackMarkers:any[]=[];
  const presentations=rows.map((r,i)=>{
    const markerIdentity=`${token}.${i}`;events.push({name:"TimeStamp",pid:41,tid:7,ts:1000100+i,args:{data:{message:`UIF_CAUSAL_V1:FEEDBACK:${markerIdentity}`,frame}}});
    const reporterBeginEventIndex=events.length;events.push({name:"PipelineReporter",ph:"b",pid:41,tid:9,ts:r.presentationTraceTimestamp-1});
    const reporterEndEventIndex=events.length;events.push({name:"PipelineReporter",ph:"e",pid:41,tid:9,ts:r.presentationTraceTimestamp});
    feedbackMarkers.push({markerIdentity,canvasEpoch:1,contextEpoch:2});
    return {...r,status:"PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION",feedbackKind:"orbit",token,markerIdentity,actionTraceTimestamp:1000000,
      canvasEpoch:1,contextEpoch:2,modelGeneration:3,rendererProcessId:41,rendererMainThreadId:7,rendererCompositorThreadId:9,layerTreeId:23,
      pageClockSource:{...PAGE_CLOCK_SOURCE,crossOriginIsolated:false},pipelineReporterOccurrence:`reporter-${i}`,reporterBeginEventIndex,reporterEndEventIndex};
  });
  const rawBytes=Buffer.from(JSON.stringify({metadata:{"clock-domain":"MAC_MACH_ABSOLUTE_TIME"},traceEvents:events}));
  const rawSha256=createHash("sha256").update(rawBytes).digest("hex");
  return {rawBytes,capture:{events,rawTraceComplete:true,rawTraceSha256:rawSha256},trace:{traceDataLossOccurred:false,rawTraceTransport:{rawCompleteThroughEof:true,rawSha256}},
    extraction:{status:"PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS",sourceBinding:REQUIRED_CHROMIUM_BINDING,presentations},
    stopped:{active:{token,feedbackKind:"orbit",documentTimeOrigin:10000,evidenceEpoch:0,armedCanvasEpoch:1,armedContextEpoch:2,
      actionMarker:{token,listenerObservedAt:actionAt,traceClock:{crossOriginIsolated:false}},feedbackMarkers}}};
}
// Independently frozen V65 rational/binary64 expectations, copied as literals (no run dependency).
test("same-trace integer arithmetic matches independent rational bounds and translation",()=>{
  const cases=[[0, 0, 0.0010000000000000002], [1, 0, 0.0020000000000000005], [16666, 16.664999999999996, 16.667000000000005], [16667, 16.665999999999997, 16.668000000000003], [33332, 33.330999999999996, 33.333000000000006], [33333, 33.331999999999994, 33.33400000000001], [9007199254740989, 9007199254740.986, 9007199254740.992]];
  for(const[delta,lower,upper]of cases){
    expect(sameTracePresentedGap(0,delta)).toEqual({lower,upper});
    expect(sameTracePresentedGap(Number.MAX_SAFE_INTEGER-delta,Number.MAX_SAFE_INTEGER)).toEqual({lower,upper});
  }
  for(const pair of [[2,1],[-1,0],[0,.5],[0,Number.MAX_SAFE_INTEGER+1],[NaN,1],[0,Infinity]])expect(()=>sameTracePresentedGap(...pair as [number,number])).toThrow();
  expect(conservativePresentedGap({lower:100,upper:100.1},{lower:116.5,upper:116.8})).toEqual({lower:4616189618054759/2**48,upper:4728779608739021/2**48});
});
test("same-trace boundary rejects mixed raw clock source document action epoch process and lineage",()=>{
  const make=()=>syntheticTraceInput([1000001,1016668].map(t=>({presentationTraceTimestamp:t}))), good=make();
  expect(bindSameTraceDurationBasis(good).references).toHaveLength(2);
  for(const mutate of [
    (v:any)=>{v.capture.rawTraceSha256="0".repeat(64);},(v:any)=>{v.trace.rawTraceTransport.rawSha256="0".repeat(64);},
    (v:any)=>{v.rawBytes=Buffer.from(v.rawBytes.toString().replace("MAC_MACH_ABSOLUTE_TIME","OTHER_CLOCK"));const h=createHash("sha256").update(v.rawBytes).digest("hex");v.capture.rawTraceSha256=h;v.trace.rawTraceTransport.rawSha256=h;},
    (v:any)=>{v.extraction.sourceBinding={...v.extraction.sourceBinding,revision:"other"};},
    (v:any)=>{v.capture.events[1].args.data.frame="other";},(v:any)=>{v.stopped.active.documentTimeOrigin=undefined;},
    (v:any)=>{v.stopped.active.evidenceEpoch=undefined;},(v:any)=>{v.extraction.presentations[1].token="other";},
    (v:any)=>{v.extraction.presentations[1].canvasEpoch++;},(v:any)=>{v.extraction.presentations[1].contextEpoch++;},
    (v:any)=>{v.extraction.presentations[1].rendererProcessId++;},(v:any)=>{v.extraction.presentations[1].layerTreeId++;},
    (v:any)=>{v.extraction.presentations[1].pageClockSource.quantumBoundMs=0;},(v:any)=>{v.stopped.active.actionMarker.traceClock.crossOriginIsolated=undefined;},
    (v:any)=>{v.extraction.presentations[1].status="FAIL";},(v:any)=>{v.extraction.status="FAIL";},
    (v:any)=>{v.extraction.presentations[1].reporterEndEventIndex=999;},(v:any)=>{v.stopped.active.feedbackMarkers.pop();},
    (v:any)=>{v.capture.rawTraceComplete=false;},(v:any)=>{v.trace.traceDataLossOccurred=true;}
  ]){const bad=make();mutate(bad);expect(()=>bindSameTraceDurationBasis(bad)).toThrow();}
});
