import { REQUIRED_CHROMIUM_BINDING, PAGE_CLOCK_SOURCE, extractCausalPresentations } from "./causal-presentation-extractor.mjs";
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
  // Historical oracle preimage; never imported or executed as the current product.
  const geometryPath = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT
    ? `${sourceRoot}/apps/desktop/src/features/viewport/viewportSelection.ts`
    : new URL("fixtures/frozen-oracle-geometry.ts.txt", methodRoot);
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
    const reporterBeginEventIndex=events.length;events.push({name:"PipelineReporter",ph:"b",pid:41,tid:9,ts:r.presentationTraceTimestamp-1,cat:"cc,benchmark",scope:"renderer",id2:{local:"0x8"}});
    const reporterEndEventIndex=events.length;events.push({name:"PipelineReporter",ph:"e",pid:41,tid:9,ts:r.presentationTraceTimestamp,cat:"cc,benchmark",scope:"renderer",id2:{local:"0x8"}});
    feedbackMarkers.push({markerIdentity,canvasEpoch:1,contextEpoch:2});
    return {...r,status:"PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION",feedbackKind:"orbit",token,markerIdentity,actionTraceTimestamp:1000000,
      canvasEpoch:1,contextEpoch:2,modelGeneration:3,rendererProcessId:41,rendererMainThreadId:7,rendererCompositorThreadId:9,layerTreeId:23,
      pageClockSource:{...PAGE_CLOCK_SOURCE,crossOriginIsolated:false},pipelineReporterOccurrence:'[41,9,"id2.local","0x8","cc,benchmark","renderer","PipelineReporter"]',reporterBeginEventIndex,reporterEndEventIndex};
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

// Source-shaped builder mirrors the maintained extractor controls, confined to this test.
function sourceOccurrenceFixture(): any {
const schema = "openpipestress.ui-foundation.causal-feedback-marker/v1";
const rendererPid = 41;
const mainTid = 7;
const compositorTid = 9;
const actionToken = "synthetic.orbit";
const actionIdentity = { gesture: "frozen-orbit-pointer-path", warmupMs: 500, measuredMs: 2_000 };


const event = (name: string, ts: number, ph: string, tid: number, args: any = {}, extra: any = {}) => ({ name, ts, ph, pid: rendererPid, tid, args, ...(name === "Layerize" ? { cat: "devtools.timeline" } : {}), ...extra });

function buildLineage(index: number, presentationTs: number) {
  const feedbackTs = presentationTs - 80_000;
  const mainFrameId = String(9_100_000_000_000_000n + BigInt(index));
  const surfaceFrameTraceId = String(2_576_121_716_840_409_000n + BigInt(index));
  const displayTraceId = String(7_576_121_716_840_409_000n + BigInt(index));
  const reporterLocal = String(8_576_121_716_840_409_000n + BigInt(index));
  const sourceFrameNumber = 100 + index;
  const layerTreeId = 23;
  const frameSequence = 700 + index;
  const markerIdentity = `${actionToken}.${index}`;
  const events = [
    event("SendBeginMainFrame", feedbackTs - 4_000, "I", compositorTid,
      { main_frame_pipeline: { step: "SEND_BEGIN_MAIN_FRAME", main_frame_id: mainFrameId,
        begin_frame_id: { source_id: 4294967296, sequence_number: frameSequence } } }),
    event("ProxyMain::BeginMainFrame", feedbackTs - 3_000, "X", mainTid,
      { begin_frame_id: frameSequence, main_frame_pipeline: { step: "BEGIN_MAIN_FRAME", main_frame_id: mainFrameId } }, { dur: 6_000 }),
    event("BeginMainThreadFrame", feedbackTs - 2_000, "X", mainTid,
      { data: { frameId: sourceFrameNumber }, layerTreeId }, { dur: 500 }),
    event("TimeStamp", feedbackTs, "I", mainTid, { data: { message: `UIF_CAUSAL_V1:FEEDBACK:${markerIdentity}` } }),
    event("LayerTreeHost::DoUpdateLayers", feedbackTs + 1_000, "X", mainTid,
      { source_frame_number: sourceFrameNumber }, { dur: 500 }),
    event("ProxyMain::BeginMainFrame::commit", feedbackTs + 4_000, "X", mainTid,
      { main_frame_pipeline: { step: "COMMIT_ON_MAIN", main_frame_id: mainFrameId } }, { dur: 500 }),
    event("ProxyImpl::Commit", feedbackTs + 8_000, "X", compositorTid,
      { main_frame_pipeline: { step: "COMMIT_ON_IMPL", main_frame_id: mainFrameId } }, { dur: 500 }),
    event("LayerTreeHostImpl::ActivateSyncTree", feedbackTs + 12_000, "X", compositorTid,
      { main_frame_pipeline: { step: "ACTIVATE", main_frame_id: mainFrameId } }, { dur: 2_000 }),
    event("ActivateLayerTree", feedbackTs + 12_500, "X", compositorTid,
      { frameId: sourceFrameNumber, layerTreeId }, { dur: 500 }),
    event("MainFrame.Draw", feedbackTs + 20_000, "X", compositorTid,
      { main_frame_pipeline: { step: "DRAW", main_frame_id: mainFrameId,
        last_begin_frame_id_during_first_draw: { source_id: 4294967296, sequence_number: frameSequence } } }, { dur: 40_000 }),
    event("PipelineReporter", feedbackTs + 20_000, "b", compositorTid, {
      frame_reporter: {
        state: "STATE_PRESENTED_ALL",
        has_missing_content: false,
        checkerboarded_needs_raster: false,
        checkerboarded_needs_record: false,
        layer_tree_host_id: layerTreeId,
        surface_frame_trace_id: surfaceFrameTraceId,
        display_trace_id: displayTraceId,
        frame_source: 4294967296,
        frame_sequence: frameSequence
      }
    }, { cat: "cc,benchmark", scope: "renderer", id2: { local: reporterLocal } }),
    event("Graphics.Pipeline", feedbackTs + 22_000, "X", compositorTid,
      { chrome_graphics_pipeline: { step: "STEP_GENERATE_COMPOSITOR_FRAME", surface_frame_trace_id: surfaceFrameTraceId } },
      { dur: 30_000 }),
    event("LayerTreeHostImpl::PrepareToDraw", feedbackTs + 23_000, "X", compositorTid,
      { SourceFrameNumber: sourceFrameNumber }, { dur: 500 }),
    event("DrawFrame", feedbackTs + 24_000, "X", compositorTid,
      { layerTreeId, frameSeqId: frameSequence }, { dur: 500 }),
    event("Graphics.Pipeline", feedbackTs + 26_000, "X", compositorTid,
      { chrome_graphics_pipeline: { step: "STEP_SUBMIT_COMPOSITOR_FRAME", surface_frame_trace_id: surfaceFrameTraceId } },
      { dur: 500 }),
    event("SubmitCompositorFrameToPresentationCompositorFrame", feedbackTs + 27_000, "b", compositorTid, {},
      { cat: "cc,benchmark", scope: "renderer", id2: { local: reporterLocal } }),
    event("SubmitCompositorFrameToPresentationCompositorFrame", presentationTs - 100, "e", compositorTid, {},
      { cat: "cc,benchmark", scope: "renderer", id2: { local: reporterLocal } }),
    event("PipelineReporter", presentationTs, "e", compositorTid, {},
      { cat: "cc,benchmark", scope: "renderer", id2: { local: reporterLocal } })
  ];
  const evidence = {
    schema,
    kind: "FEEDBACK",
    markerIdentity,
    token: actionToken,
    phase: "candidate",
    feedbackKind: "orbit",
    invocationId: index,
    registrationId: index,
    parentInvocationId: null,
    canvasEpoch: 1,
    contextEpoch: 1,
    callbackEntryAt: feedbackTs / 1000 - 900.2,
    callbackCompletedAt: feedbackTs / 1000 - 900,
    mainContextColorClearObserved: true,
    observed: { modelGeneration: 1, mainRenderSubmissionSequence: index, cameraSequence: index },
    traceClock: { source: PAGE_CLOCK_SOURCE, crossOriginIsolated: false, before: feedbackTs / 1000 - 900.1, after: feedbackTs / 1000 - 899.9 }
  };
  return { events, evidence };
}

function buildFixture(presentationTimes = [1_100_000]): any {
  const actionTraceTs = 1_000_000;
  const actionMarker = {
    schema,
    kind: "ACTION",
    token: actionToken,
    phase: "candidate",
    feedbackKind: "orbit",
    eventKind: "pointerdown",
    testId: "viewport-canvas",
    browserEventTimeStamp: 100,
    listenerObservedAt: 100,
    pointerId: 1,
    clientX: 400,
    clientY: 300,
    actionIdentity,
    traceClock: { source: PAGE_CLOCK_SOURCE, crossOriginIsolated: false, before: 99.9, after: 100.1 }
  };
  const pointerTransaction = {
    down: {
      eventKind: "pointerdown", browserEventTimeStamp: 100, listenerObservedAt: 100,
      pointerId: 1, clientX: 400, clientY: 300, testId: "viewport-canvas", targetTag: "CANVAS", actionIdentity
    },
    up: {
      eventKind: "pointerup", browserEventTimeStamp: 2_900, listenerObservedAt: 2_900,
      pointerId: 1, clientX: 410, clientY: 300, testId: "viewport-canvas", targetTag: "CANVAS", actionIdentity
    }
  };
  const lineages = presentationTimes.map((timestamp, index) => buildLineage(index + 1, timestamp));
  return {
    events: [
      event("PipelineReporter", 900_000, "e", compositorTid, {},
        { cat: "unrelated", scope: "other", id2: { local: "9900000000000000001" } }),
      event("TimeStamp", actionTraceTs, "I", mainTid, { data: { message: `UIF_CAUSAL_V1:ACTION:${actionToken}` } }),
      ...lineages.flatMap((lineage) => lineage.events),
      event("PipelineReporter", 1_400_000, "b", compositorTid, {},
        { cat: "unrelated", scope: "other", id2: { local: "9900000000000000002" } })
    ],
    markerEvidence: {
      active: {
        token: actionToken,
        phase: "candidate",
        feedbackKind: "orbit",
        actionStartEvent: "pointerdown",
        expectedActionTargetTestId: "viewport-canvas",
        actionIdentity,
        actionMarker,
        pointerTransaction,
        feedbackMarkers: lineages.map((lineage) => lineage.evidence),
        observerCostsMs: [0.1],
        stopped: true,
        stopReason: "EXPLICIT_STOP"
      }
    }
  };
}

  const f=buildFixture([1_100_000,1_300_000,1_500_000]);
  // First two nonoverlapping occurrences reuse one local track. A third distinct
  // track ends at the second timestamp but starts later, preserving exact lineage.
  for(const e of f.events) {
    if(e.name==="TimeStamp")e.args.data.frame="document-A";
    if(e.id2 && e.cat==="cc,benchmark") {
      const third=e.id2.local==="8576121716840409003";
      e.id2.local=third?"0x2":"0x8";
    }
  }
  for(const e of f.events.slice(38,56)) {e.ts-=150000;if(e.ph==="e")e.ts-=50000;}
  // Move the third lineage's non-async timestamps uniformly earlier; its end
  // remains1300000 and begin1290000, after the second main frame's draw.
  const third=f.markerEvidence.active.feedbackMarkers[2];
  for(const k of ["callbackEntryAt","callbackCompletedAt"])third[k]-=150;
  third.traceClock.before-=150;third.traceClock.after-=150;
  Object.assign(f.markerEvidence.active,{documentTimeOrigin:10000,evidenceEpoch:0,armedCanvasEpoch:1,armedContextEpoch:1});
  f.events.reverse(); // Raw indexes address storage, not chronology.
  return f;
}
function extractedOccurrenceInput(f:any) {
  const extraction=extractCausalPresentations(f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence);
  const rawBytes=Buffer.from(JSON.stringify({metadata:{"clock-domain":"MAC_MACH_ABSOLUTE_TIME"},traceEvents:f.events}));
  const rawSha256=createHash("sha256").update(rawBytes).digest("hex");
  return {rawBytes,capture:{events:f.events,rawTraceComplete:true,rawTraceSha256:rawSha256},
    trace:{traceDataLossOccurred:false,rawTraceTransport:{rawCompleteThroughEof:true,rawSha256}},extraction,stopped:f.markerEvidence};
}
test("actual extractor and binder retain reusable tracks distinct raw pairs and tied timestamps",()=>{
  const f=sourceOccurrenceFixture(),input=extractedOccurrenceInput(f),basis=bindSameTraceDurationBasis(input);
  expect(basis.references).toHaveLength(3);
  expect(basis.references.map(r=>r.markerIdentity)).toEqual(["synthetic.orbit.1","synthetic.orbit.2","synthetic.orbit.3"]);
  expect(basis.references.map(r=>r.reportedTimestamp)).toEqual([1100000,1300000,1300000]);
  expect(basis.references[0].reporterOccurrence).toBe(basis.references[1].reporterOccurrence);
  expect(basis.references.map(r=>[r.reporterBeginEventIndex,r.reporterEndEventIndex])).toEqual([[44,37],[26,19],[8,1]]);
  expect(basis.references.every(r=>r.reporterBeginEventIndex>r.reporterEndEventIndex)).toBe(true);
  expect(new Set(basis.references.flatMap(r=>[r.reporterBeginEventIndex,r.reporterEndEventIndex])).size).toBe(6);
  for(const [i,r] of basis.references.entries()) {
    expect(r.reporterBeginEventIndex).toBe(input.extraction.presentations[i].reporterBeginEventIndex);
    expect(f.events[r.reporterEndEventIndex].ts).toBe(r.reportedTimestamp);
  }
  expect(sameTracePresentedGap(1300000,1300000)).toEqual({lower:0,upper:.0010000000000000002});
  for(const mutate of [
    (b:any)=>{b.references[0].reporterBeginEventIndex=999;},(b:any)=>{b.references[0].reporterEndEventIndex=998;},
    (b:any)=>{b.references[0].reporterOccurrence="wrong-key";},(b:any)=>{b.references[0].reportedTimestamp++;},
    (b:any)=>{b.contextEpoch++;}
  ]){const bad=structuredClone(basis);mutate(bad);expect(()=>constructOrbitEvidence(input.extraction.presentations,100,"centerline",bad)).toThrow("orbit requires bound same-trace endpoint context");}
});
test("source-shaped extractor still rejects overlap missing pair and mismatched track endpoints",()=>{
  for(const mutate of [
    (f:any)=>{const b=f.events.find((e:any)=>e.name==="PipelineReporter"&&e.ph==="b"&&e.id2.local==="0x8");f.events.push({...structuredClone(b),ts:b.ts+1});},
    (f:any)=>{f.events.splice(f.events.findIndex((e:any)=>e.name==="PipelineReporter"&&e.ph==="e"&&e.id2.local==="0x8"),1);},
    ...["pid","tid","scope"].map(k=>(f:any)=>{const e=f.events.find((e:any)=>e.name==="PipelineReporter"&&e.ph==="e"&&e.id2.local==="0x8");e[k]=k==="scope"?"wrong":99;})
  ]){const f=sourceOccurrenceFixture();mutate(f);expect(()=>extractedOccurrenceInput(f)).toThrow();}
});


test("actual action lifecycle persists once before host tail and blocks next action through finalization",async()=>{
  const {runStoppedActionLifecycle}=await import("./full-cohort-controller");
  for(const kind of ["assignment","point-selection","box-selection","tree-filter","orbit"]) {
    const calls:string[]=[],snapshot={original:true},sha="a".repeat(64);
    let releaseWait!:()=>void,releaseEnd!:()=>void,enteredWait!:()=>void,enteredEnd!:()=>void;
    const waiting=new Promise<void>(r=>enteredWait=r),ending=new Promise<void>(r=>enteredEnd=r);
    const waitGate=new Promise<void>(r=>releaseWait=r),endGate=new Promise<void>(r=>releaseEnd=r);
    const promise=runStoppedActionLifecycle(kind,{
      begin:async()=>{calls.push("begin");},
      work:async stop=>{calls.push("work");const saved=await stop();calls.push("proof");expect(saved.evidence).toBe(snapshot);expect(saved.stoppedSha256).toBe(sha);return saved;},
      stop:async()=>{calls.push("stop");return snapshot;},
      persist:async evidence=>{calls.push("persist");expect(evidence).toBe(snapshot);return sha;},
      wait:async ms=>{calls.push(`host-wait:${ms}`);enteredWait();await waitGate;calls.push("wait-complete");},
      finalize:async()=>{calls.push("trace-end");enteredEnd();await endGate;calls.push("trace-complete");}
    }).then(result=>{calls.push("next-action-allowed");return result;});
    const tail=!["tree-filter","orbit"].includes(kind);
    if(tail){await waiting;expect(calls).toEqual(["begin","work","stop","persist","proof","host-wait:250"]);await Promise.resolve();expect(calls.at(-1)).toBe("host-wait:250");releaseWait();}
    await ending;expect(calls.at(-1)).toBe("trace-end");expect(calls).not.toContain("next-action-allowed");releaseEnd();
    const result=await promise;expect(result.errors).toEqual([]);expect(result.stopped).toBe(snapshot);expect(result.stoppedSha256).toBe(sha);
    expect(calls).toEqual(["begin","work","stop","persist","proof",...(tail?["host-wait:250","wait-complete"]:[]),"trace-end","trace-complete","next-action-allowed"]);
  }
});
test("actual lifecycle retains injected work stop persistence tail and finalization failures without replacement stop",async()=>{
  const {runStoppedActionLifecycle}=await import("./full-cohort-controller");
  for(const failure of ["work-before","stop","persist","work-after","wait","finalize"]) {
    const calls:string[]=[],snapshot={original:true},sha="b".repeat(64),error=new Error(`injected:${failure}`);
    const result=await runStoppedActionLifecycle("assignment",{
      begin:async()=>{calls.push("begin");},
      work:async stop=>{calls.push("work");if(failure==="work-before")throw error;await stop();if(failure==="work-after")throw error;return {};},
      stop:async()=>{calls.push("stop");if(failure==="stop")throw error;return snapshot;},
      persist:async evidence=>{calls.push("persist");expect(evidence).toBe(snapshot);if(failure==="persist")throw error;return sha;},
      wait:async ms=>{calls.push(`wait:${ms}`);if(failure==="wait")throw error;},
      finalize:async()=>{calls.push("finalize");if(failure==="finalize")throw error;}
    });
    expect(result.errors.some(e=>e.error===String(error))).toBe(true);
    expect(calls.filter(c=>c==="stop")).toHaveLength(1);expect(calls.filter(c=>c==="finalize")).toHaveLength(1);
    expect(calls.filter(c=>c==="persist")).toHaveLength(failure==="stop"?0:1);
    expect(result.stopped).toBe(failure==="stop"?null:snapshot);
    expect(result.stoppedSha256).toBe(["stop","persist"].includes(failure)?"":sha);
    expect(calls.filter(c=>c.startsWith("wait:"))).toHaveLength(["stop","persist"].includes(failure)?0:1);
    expect(calls.at(-1)).toBe("finalize");
  }
  const calls:string[]=[],result=await runStoppedActionLifecycle("assignment",{
    begin:async()=>{},work:async stop=>{for(let i=0;i<2;i++){try{await stop();}catch{}}throw new Error("work failed");},
    stop:async()=>{calls.push("stop");return {original:true};},persist:async()=>{calls.push("persist");throw new Error("persist failed");},
    wait:async()=>{calls.push("wait");},finalize:async()=>{calls.push("finalize");throw new Error("finalize failed");}
  });
  expect(calls).toEqual(["stop","persist","finalize"]);
  expect(result.errors.map(e=>e.error)).toEqual(["Error: persist failed","Error: work failed","Error: finalize failed"]);
});
test("assignment diagnostics freeze ten fresh identities and cannot qualify full or focused workload",async()=>{
  const {candidateDiagnosticMode,assignmentDiagnosticResult,assignmentDiagnosticSummary}=await import("./full-cohort-controller");
  const {scorePerformanceRun,scorePerformanceCohort}=await import("./performance-targets");
  expect(candidateDiagnosticMode(undefined)).toBe("full");expect(candidateDiagnosticMode(undefined,true)).toBe("focused");
  expect(candidateDiagnosticMode("assignment-collection")).toBe("assignment-collection");
  for(const value of ["","full","unknown"])expect(()=>candidateDiagnosticMode(value)).toThrow();
  expect(()=>candidateDiagnosticMode("assignment-collection",true)).toThrow();
  expect(()=>candidateDiagnosticMode(undefined,false,true)).toThrow();
  expect(()=>candidateDiagnosticMode("assignment-collection",false,false)).toThrow();
  expect(candidateDiagnosticMode("assignment-collection",false,true)).toBe("assignment-collection");
  const expected:any[]=[1000,10000].flatMap(fixtureSize=>[1,2,3,4,5].map(runNumber=>({fixtureSize,runNumber,runId:`d-${fixtureSize}-${runNumber}`,sessionId:`s-${fixtureSize}-${runNumber}`,bindings:{}})));
  const metric={qualification:"PASS_QUALIFIED_CAUSAL_EVIDENCE",durationIntervalMs:{lower:1,upper:2000}};
  const results=expected.map(e=>assignmentDiagnosticResult(e,metric,[],1));
  expect(assignmentDiagnosticSummary(results,expected).status).toBe("PASS_TEN_ASSIGNMENT_COLLECTION_DIAGNOSTICS");
  expect(results.every(r=>r.cohortContribution===0)).toBe(true);
  for(const missing of [results.slice(1),[...results.slice(0,9),results[0]],[]])expect(assignmentDiagnosticSummary(missing,expected).status).toBe("FAIL_ASSIGNMENT_COLLECTION_DIAGNOSTICS");
  expect(assignmentDiagnosticSummary(results,expected.slice(1)).status).toBe("FAIL_ASSIGNMENT_COLLECTION_DIAGNOSTICS");
  for(const [value,errors,count] of [[{...metric,durationIntervalMs:{lower:1,upper:2000.0001}},[],1],[metric,["observer/restoration/binding failed"],1],[metric,[],8],[undefined,[],1]] as any[])expect(assignmentDiagnosticResult(expected[0],value,errors,count).status).toBe("FAIL_ASSIGNMENT_COLLECTION_DIAGNOSTIC");
  expect(scorePerformanceRun(results[0] as any,expected[0]).status).toBe("FAIL_INVALID_EVIDENCE");
  expect(scorePerformanceCohort(results as any,expected).status).toBe("FAIL_COHORT");
});

import { collectionMode, validateFixedSelection, runCollectionDisposition, executeCollectionRun, attemptDisposition, validateReferenceProfile, CHARACTERIZATION_PRODUCT_REVISION } from "./characterization-mode";
import { validateBoundaryMetadata, validateDisplayProfile, prepareFilterInput, insertFilterQuery, FILTER_STIMULUS } from "./characterization-commands";

test("characterization rejects malformed/conflicting modes and every partial/reordered plan", () => {
  expect(collectionMode(undefined)).toBe("qualification"); expect(collectionMode("characterization")).toBe("characterization");
  for(const value of ["", "CHARACTERIZATION", "true", "unknown"])expect(()=>collectionMode(value)).toThrow();
  for(const [phase,diagnostic,focused] of [["baseline",undefined,false],["candidate","assignment-collection",false],["candidate",undefined,true]] as const)
    expect(()=>collectionMode("characterization",phase,diagnostic,focused)).toThrow();
  validateFixedSelection([1000,10000],[1,2,3,4,5]);
  for(const counts of [[1000],[10000,1000],[1000,1000],[1000,10000,1000],[NaN,10000]])expect(()=>validateFixedSelection(counts,[1,2,3,4,5])).toThrow();
  for(const runs of [[1],[5,4,3,2,1],[1,2,3,4,4],[1,2,3,4,5,6],[1,2,NaN,4,5]])expect(()=>validateFixedSelection([1000,10000],runs)).toThrow();
});
const targetMiss={status:"FAIL_TARGETS",validityFailures:[],targetFailures:["TARGET_EXCEEDED:centerlineP95"]};
test("real driver execution gate continues ten target-miss callbacks in order, strict stops first, invalid never retries", async () => {
  const plan=[1000,10000].flatMap(size=>[1,2,3,4,5].map(run=>`${size}.${run}`));
  for(const mode of ["characterization","qualification"] as const) {
    const calls:string[]=[];
    const sequence=async()=>{for(const id of plan)await executeCollectionRun(async()=>{
      calls.push(id);return {scored:targetMiss,collection:runCollectionDisposition(mode,targetMiss,[],243)};
    });};
    if(mode==="qualification")await expect(sequence()).rejects.toThrow();else await sequence();
    expect(calls).toEqual(mode==="qualification"?plan.slice(0,1):plan);
  }
  for(const failure of ["SETTLED_OWNED_RAF_NOT_EXACT_ZERO","INVALID_OR_MISMATCHED_BINDINGS","INCOMPLETE_OR_INVALID_ORBIT:actual-od"]){
    const calls:string[]=[];await expect((async()=>{for(const id of plan)await executeCollectionRun(async()=>{
      calls.push(id);const score={...targetMiss,validityFailures:calls.length===2?[failure]:[]};
      return {collection:runCollectionDisposition("characterization",score,[],243)};
    });})()).rejects.toThrow();expect(calls).toEqual(plan.slice(0,2));
  }
  for(const [errors,count] of [[["cleanup"],243],[[],242]] as const)expect(runCollectionDisposition("characterization",targetMiss,errors,count).attemptDisposition).toBe("ABORT_REMAINING");
  const original=new Error("original timeout");await expect(executeCollectionRun(async()=>{throw original;})).rejects.toBe(original);
});
test("attempt separates complete characterization from FAIL_COHORT and lists failed/unattempted runs",()=>{
  const plan=Array.from({length:10},(_,i)=>({runId:`r${i}`}));
  const records=plan.map(expected=>({expected,collection:runCollectionDisposition("characterization",targetMiss,[],243)}));
  const score={status:"FAIL_COHORT",validityFailures:[]};const frozen=JSON.stringify(score);
  expect(attemptDisposition("characterization",plan,records,score)).toMatchObject({attemptDisposition:"COMPLETED",targetOutcome:"FAIL_COHORT",collectionCompleteness:"COMPLETE_TEN_RUNS"});
  expect(attemptDisposition("qualification",plan,records,score).attemptDisposition).toBe("ABORTED");
  const partial=attemptDisposition("characterization",plan,[records[0],{expected:plan[1],started:true,error:"timeout"}],score);
  expect(partial.runs[1].disposition.attemptDisposition).toBe("ABORT_REMAINING");expect(partial.runs[2].disposition.attemptDisposition).toBe("UNATTEMPTED");expect(JSON.stringify(score)).toBe(frozen);
});
function metadataFixture(){return {id:"point-selection-1-ready",runId:"r",referenceProfileSha256:"a".repeat(64),bindings:{source:"s"},snapshot:{model:{generation:1,identityHash:"m"},viewport:{
  canvas:{cssLeft:0,cssTop:0,cssWidth:400,cssHeight:300,bufferWidth:800,bufferHeight:600,dpr:2},labels:{enabled:false,renderedCount:0,budget:200},
  geometry:{mode:"schematic",odGeneration:0,odStatus:"idle"},camera:{sequence:1,kind:"perspective",position:[1,2,3],target:[0,0,0],up:[0,1,0],localRenderOrigin:[0,0,0],fovDegrees:45,near:.1,far:1000,aspect:4/3}}},
  presentation:{browserDpr:2,windowWidth:1440,windowHeight:920,theme:"light",density:"comfortable",panes:[{selector:".workspace-pane-tree",visible:true,x:0,y:0,width:300,height:500},{selector:".workspace-pane-inspector",visible:true,x:1000,y:0,width:300,height:500}],panels:[{visible:true,width:300,height:400},{visible:true,width:300,height:400}]}};}
test("required metadata rejects missing fields and profile/model/binding drift while allowing legitimate camera/labels transitions",()=>{
  const value=metadataFixture(),expected={runId:"r",bindings:value.bindings};validateBoundaryMetadata(value,expected);
  for(const mutate of [(v:any)=>v.snapshot.viewport.canvas.cssWidth=0,(v:any)=>v.snapshot.viewport.canvas.bufferWidth=999,
    (v:any)=>v.presentation.browserDpr=NaN,(v:any)=>v.snapshot.viewport.labels.renderedCount=1,(v:any)=>v.snapshot.viewport.labels.renderedCount=-1,
    (v:any)=>v.snapshot.viewport.camera.position=[0,Infinity,0],(v:any)=>v.presentation.theme=null,(v:any)=>v.presentation.panels[0].visible=false,
    (v:any)=>v.snapshot.viewport.geometry={mode:"actual-od",odStatus:"idle",odGeneration:0},(v:any)=>v.bindings.source="drift"]){const bad=structuredClone(value);mutate(bad);expect(()=>validateBoundaryMetadata(bad,expected)).toThrow();}
  for(const mutate of [(v:any)=>v.presentation.theme="dark",(v:any)=>v.snapshot.model.generation++,(v:any)=>v.snapshot.viewport.canvas.cssLeft++]){const bad=structuredClone(value);mutate(bad);expect(()=>validateBoundaryMetadata(bad,expected,value)).toThrow("drift");}
  const changed=structuredClone(value);changed.id="orbit-1-ready";changed.snapshot.viewport.camera.position=[4,5,6];changed.snapshot.viewport.labels.enabled=true;changed.snapshot.viewport.labels.renderedCount=3;validateBoundaryMetadata(changed,expected,value);
});
test("keyboard adapter focuses before arm and inserts exactly one whole query without fill, clipboard or synthetic handlers",async()=>{
  const calls:string[]=[],page:any={getByTestId:()=>({inputValue:async()=>"",focus:async()=>calls.push("focus")}),keyboard:{insertText:async(q:string)=>calls.push(`insert:${q}`)}};
  await prepareFilterInput(page,100);calls.push("arm-first-input");await insertFilterQuery(page,"pipe 42");calls.push("stop-final-content");
  expect(calls).toEqual(["focus","arm-first-input","insert:pipe 42","stop-final-content"]);expect(FILTER_STIMULUS.expectedInputEvents).toBe(1);
  page.getByTestId=()=>({inputValue:async()=>"stale"});await expect(prepareFilterInput(page,100)).rejects.toThrow();
});
test("reference profile and external main online 60 Hz display binding fail closed including ambiguous profiles",()=>{
  const profile={schema:"ui-foundation.reference-profile/v1",cohortId:"c",productRevision:CHARACTERIZATION_PRODUCT_REVISION,hostModel:"Apple M5 Max",memoryBytes:128*1024**3,
    refreshHz:60,externallyVerified:true,verificationEvidence:"external.json",verifiedAt:"2026-09-17T23:00:00Z",viewport:[1440,920],browserDpr:2,effectiveDprCap:2};
  const host={model:"Apple M5 Max",memoryBytes:profile.memoryBytes};validateReferenceProfile(profile,"c",host);
  for(const bad of [{...profile,refreshHz:120},{...profile,externallyVerified:false},{...profile,cohortId:"other"}])expect(()=>validateReferenceProfile(bad,"c",host)).toThrow();
  expect(()=>validateReferenceProfile(profile,"c",{...host,memoryBytes:1})).toThrow();
  const identity={name:"LG",vendor:"v",product:"p",serial:"s",pixels:"3840 x 2160",resolution:"1920 x 1080 @ 60.00Hz",mirror:"spdisplays_off"};
  const display={_name:identity.name,"_spdisplays_display-vendor-id":"v","_spdisplays_display-product-id":"p","_spdisplays_display-serial-number":"s",_spdisplays_pixels:identity.pixels,_spdisplays_resolution:identity.resolution,spdisplays_mirror:identity.mirror,spdisplays_main:"spdisplays_yes",spdisplays_online:"spdisplays_yes"};
  const raw={SPDisplaysDataType:[{sppci_model:"Apple M5 Max",spdisplays_ndrvs:[display]}]};
  expect(validateDisplayProfile(raw,Object.fromEntries(Object.entries(identity).reverse()))).toEqual(identity);
  for(const bad of [{SPDisplaysDataType:[]},{SPDisplaysDataType:[{sppci_model:"Apple M5 Max",spdisplays_ndrvs:[display,display]}]}])expect(()=>validateDisplayProfile(bad,identity)).toThrow();
  expect(()=>validateDisplayProfile(raw,{...identity,resolution:"120 Hz"})).toThrow();
});

test("product inventory rejects omission duplicates split overlap and wrong final provenance",async()=>{
  const {validateInventoryCoverage,validateFinalProductProvenance}=await import("./full-cohort-controller");
  const root="apps/desktop/src",actual=[`${root}/App.tsx`,`${root}/App.test.tsx`],production=[{path:actual[0]}],testOnly=[{path:actual[1]}];
  validateInventoryCoverage([...production,...testOnly],actual,root);
  for(const bad of [production,[...production,...testOnly,...testOnly],[...production,{path:`${root}/omitted.ts`} ]])expect(()=>validateInventoryCoverage(bad,actual,root)).toThrow();
  validateFinalProductProvenance("final",CHARACTERIZATION_PRODUCT_REVISION,CHARACTERIZATION_PRODUCT_REVISION,false);
  for(const [stage,revision,head,same] of [["preliminary",CHARACTERIZATION_PRODUCT_REVISION,CHARACTERIZATION_PRODUCT_REVISION,false],["final","wrong",CHARACTERIZATION_PRODUCT_REVISION,false],["final",CHARACTERIZATION_PRODUCT_REVISION,"wrong",false],["final",CHARACTERIZATION_PRODUCT_REVISION,CHARACTERIZATION_PRODUCT_REVISION,true]] as const)expect(()=>validateFinalProductProvenance(stage,revision,head,same)).toThrow();
});
test("frozen actual product/profile inputs validate read-only when explicitly supplied",async()=>{
  test.skip(!process.env.D70_WRITER_PREPARED_INPUTS,"runner input file not supplied");
  const prepared=JSON.parse(await readFile(process.env.D70_WRITER_PREPARED_INPUTS!,"utf8"));
  const env=prepared.environment;const {bindReferenceProfile}=await import("./characterization-mode");
  expect((await bindReferenceProfile(env)).record.cohortId).toBe(env.UI_FOUNDATION_COHORT_ID);
  const {validateCharacterizationProduct}=await import("./full-cohort-controller");
  const bytes=await readFile(env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST);
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256);
  const bundle=JSON.parse(bytes.toString());const binding:any={sourceStage:env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE,candidateSourceRoot:env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT};
  await validateCharacterizationProduct(binding,bundle);
  const changed=structuredClone(bundle);changed.mutableTestOnlySourceSnapshot[0].sha256="0".repeat(64);
  await expect(validateCharacterizationProduct(binding,changed)).rejects.toThrow("bytes drift");
});


test("uninstrumented smoke canvas guard accepts observed exact canvas without weakening timed epochs",async()=>{
  const {assertSmokeMainCanvasHitTarget}=await import("./characterization-commands");
  const {assertMainCanvasHitTarget}=await import("./causal-method-contract");
  // Exact structural read from controls-smoke-01's canonical failure record.
  const observed={status:"PASS_ACTUAL_CONNECTED_MAIN_CANVAS_TARGET" as const,
    clientPoint:{x:576.3383298461134,y:510.83226543984887},canvasEpoch:null,armedCanvasEpoch:null,exactArmedCanvas:null,
    canvasConnected:true,exactCanvasTarget:true,targetTag:"CANVAS",targetTestId:"viewport-canvas",
    canvasRect:{x:293,y:285.1953125,width:794,height:557}};
  const original=JSON.stringify(observed);
  assertSmokeMainCanvasHitTarget(observed);
  expect(()=>assertMainCanvasHitTarget(observed)).toThrow("prescribed pointer target");
  expect(JSON.stringify(observed)).toBe(original);
  for(const change of [{canvasConnected:false},{exactCanvasTarget:false},{targetTag:"DIV"},{targetTestId:"overlay"},
    {status:"FAIL_MAIN_CANVAS_TARGET"},{clientPoint:{x:0,y:0}},{clientPoint:{x:NaN,y:500}},
    {canvasRect:{...observed.canvasRect,width:0}},{canvasEpoch:1},{armedCanvasEpoch:1},{exactArmedCanvas:false}])
    expect(()=>assertSmokeMainCanvasHitTarget({...observed,...change} as any)).toThrow("smoke pointer target");
  const timed={...observed,canvasEpoch:7,armedCanvasEpoch:7,exactArmedCanvas:true};
  expect(()=>assertMainCanvasHitTarget(timed)).not.toThrow();
  expect(()=>assertMainCanvasHitTarget({...timed,canvasEpoch:8})).toThrow();
  expect(()=>assertMainCanvasHitTarget({...timed,exactArmedCanvas:false})).toThrow();
});

test("continuation metadata allows truthful inner content heights but preserves outer geometry and rejected evidence",async({},info)=>{
  const {validateBoundaryWithRejectionRecord,boundaryFieldDifferences}=await import("./characterization-commands");
  const before=metadataFixture(),expected={runId:before.runId,fixtureSize:1000,runNumber:2,bindings:before.bindings};
  // Exact inner dimensions from retained continuation-smoke-01, portable without its raw cache.
  before.presentation.panels[1].width=338;before.presentation.panels[1].height=299.96875;
  const selected=structuredClone(before);selected.presentation.panels[1].width=323;selected.presentation.panels[1].height=1530.625;
  (selected.snapshot.viewport as any).selection={primaryRef:{type:"pipe",id:"pipe:UIF-00205"}} as any;
  validateBoundaryMetadata(selected,expected,before);
  expect((selected as any).contentGeometryTransitions).toContainEqual({field:"presentation.panels.1.width",before:338,after:323});
  expect((selected as any).contentGeometryTransitions).toContainEqual({field:"presentation.panels.1.height",before:299.96875,after:1530.625});
  const empty=structuredClone(before);empty.presentation.panels[0].height=100;validateBoundaryMetadata(empty,expected,before);
  for(const size of [1000,10000]) {
    const fixture=await loadFixture(size);const sample=fixture.samples.tree_filters.find((s:any)=>s.sample===18);
    expect(sample.query).toBe("no-match-ui-foundation");expect(frozenTreeExpectation(fixture.model,sample.query).visibleCount).toBe(0);
  }
  const directory=info.outputPath("rejected");await mkdir(directory,{recursive:true});
  for(const [i,mutate] of [(v:any)=>v.presentation.panes[0].x++,(v:any)=>v.presentation.panes[1].height++,
    (v:any)=>v.presentation.panels[0].visible=false,(v:any)=>v.snapshot.viewport.canvas.cssLeft++,
    (v:any)=>v.snapshot.model.generation++,(v:any)=>v.bindings.source="changed",(v:any)=>v.referenceProfileSha256="b".repeat(64),
    (v:any)=>delete v.presentation.panes[0].width].entries()) {
    const bad=structuredClone(before);bad.id=`rejected-${i}`;mutate(bad);
    await expect(validateBoundaryWithRejectionRecord(bad,expected,before,directory)).rejects.toThrow();
    const record=JSON.parse(await readFile(`${directory}/${bad.id}-rejected.json`,"utf8"));
    expect(record.actual).toEqual(bad);expect(record.referenceId).toBe(before.id);expect(record.expected.runNumber).toBe(2);
    expect(record.fieldDifferences).toEqual(boundaryFieldDifferences(before,bad));expect(record.error).toMatch(/metadata|drift/);
  }
  const bad=structuredClone(before);bad.presentation.panes[0].height++;
  await expect(validateBoundaryWithRejectionRecord(bad,expected,before,directory,async()=>{throw new Error("disk failure");})).rejects.toThrow("rejected metadata persistence failed: Error: disk failure");
});

test("continuation launcher consumes failed/interrupted slots and executes later sizes only after independent gates",async({},info)=>{
  const {launchContinuationSlot,continuationClaims,continuationBudgetReport,continuationExternalBindings,CONTINUATION_SLOTS,ORIGINAL_RETURN_SHA256}=await import("./characterization-observations.mjs");
  const {rm}=await import("node:fs/promises");
  const original=process.env.D70_WRITER_ORIGINAL_RETURN;
  test.skip(!original,"hash-bound original return must be explicitly supplied");
  const seedBytes=await readFile(original!);expect(createHash("sha256").update(seedBytes).digest("hex")).toBe(ORIGINAL_RETURN_SHA256);
  const seed=JSON.parse(seedBytes.toString()),directory=info.outputPath("continuation");await mkdir(directory,{recursive:true});
  const save=async(name:string,value:any)=>{const file=`${directory}/${name}`,bytes=typeof value==="string"?value:JSON.stringify(value);await writeFile(file,bytes,{flag:"wx"});return {path:file,sha256:createHash("sha256").update(bytes).digest("hex")};};
  const seedRef={path:original!,sha256:ORIGINAL_RETURN_SHA256};
  const policy={schema:"ui-foundation.continuation-policy/v1",seed:seedRef,cohortId:seed.frozenEnvironment.UI_FOUNDATION_COHORT_ID,
    ledgerRoot:`${directory}/ledger`,instrumentProjectRoot:directory,instrumentRevision:"a".repeat(40),method:{path:`${directory}/method.json`,sha256:"b".repeat(64)},
    attemptRoots:Object.fromEntries(CONTINUATION_SLOTS.map(slot=>[slot,`${directory}/attempt-${slot}`]))};
  const policyRef=await save("policy.json",policy),calls:string[]=[],checks:string[]=[];
  const receipt=async(slot:string,previous:string,recovery=false)=>{
    const cleanup=await save(`cleanup-${slot}-${recovery}.json`,{previousClaimSha256:previous,browserProcessesRemaining:0,serverListening:false,verificationStatus:"VERIFIED",processDisposition:recovery?"EXTERNAL_RECOVERY_VERIFIED":"NORMAL_EXIT"});
    const bindings=await save(`bindings-${slot}-${recovery}.json`,{status:"PASS_INDEPENDENT_EXTERNAL_REVALIDATION",externalBindings:continuationExternalBindings(seed)});
    return save(`receipt-${slot}-${recovery}.json`,{schema:"ui-foundation.continuation-preconditions/v1",slot,previousClaimSha256:previous,verifiedAt:new Date().toISOString(),
      cleanup:{status:"VERIFIED_NO_REMAINING_BROWSER_OR_SERVER",evidence:cleanup},bindingsStatus:"VERIFIED_UNCHANGED",externalBindings:continuationExternalBindings(seed),bindingEvidence:bindings,evidence:[cleanup,bindings]});
  };
  let registry:any=null;
  const operations={bindRegistry:async(_file:string,value:any)=>{if(registry&&JSON.stringify(registry)!==JSON.stringify(value))throw new Error("approved ledger relocation/reset rejected");registry=value;},verifyFiles:async()=>{checks.push("external");return seed.frozenEnvironment;},spawn:async(_p:any,env:any)=>{
    const claim=JSON.parse(await readFile(env.UI_FOUNDATION_CONTINUATION_CLAIM,"utf8"));calls.push(claim.slot);
    if(claim.slot==="1000.2")throw new Error("interrupted spawn/process");return {exitCode:1,signal:null};
  }};
  const firstReceipt=await receipt("1000.2",ORIGINAL_RETURN_SHA256);
  await expect(launchContinuationSlot(policyRef,"1000.2",firstReceipt,{...operations,verifyFiles:async()=>{throw new Error("actual binding hash drift");}})).rejects.toThrow("binding hash drift");
  expect(await continuationClaims(policy)).toHaveLength(0);expect(calls).toEqual([]);
  const receiptBody=JSON.parse(await readFile(firstReceipt.path,"utf8"));
  const dirty=await save("dirty-cleanup.json",{previousClaimSha256:ORIGINAL_RETURN_SHA256,browserProcessesRemaining:1,serverListening:false,verificationStatus:"VERIFIED",processDisposition:"NORMAL_EXIT"});
  const dirtyReceipt=await save("dirty-receipt.json",{...receiptBody,cleanup:{...receiptBody.cleanup,evidence:dirty}});
  await expect(launchContinuationSlot(policyRef,"1000.2",dirtyReceipt,operations)).rejects.toThrow("cleanup evidence");
  const wrongBindings=await save("wrong-bindings.json",{status:"PASS_INDEPENDENT_EXTERNAL_REVALIDATION",externalBindings:{...continuationExternalBindings(seed),UI_FOUNDATION_MANIFEST_SHA256:"wrong"}});
  const wrongReceipt=await save("wrong-receipt.json",{...receiptBody,bindingEvidence:wrongBindings});
  await expect(launchContinuationSlot(policyRef,"1000.2",wrongReceipt,operations)).rejects.toThrow("binding proof mismatch");
  expect(await continuationClaims(policy)).toHaveLength(0);expect(calls).toEqual([]);
  const copiedSeed=await save("copied-original.json",seedBytes.toString());
  const copiedPolicy=await save("copied-policy.json",{...policy,seed:copiedSeed});
  await expect(launchContinuationSlot(copiedPolicy,"1000.2",firstReceipt,operations)).rejects.toThrow("canonical original seed");
  const first=await launchContinuationSlot(policyRef,"1000.2",firstReceipt,operations);
  expect(first.exitCode).toBeNull();expect(first.launchError).toContain("interrupted");
  await rm(`${policy.ledgerRoot}/terminal-1000.2.json`); // models launcher termination before terminal persistence
  const claims=await continuationClaims(policy);expect(claims.map(c=>c.claim.slot)).toEqual(["1000.2"]);
  const nextReceipt=await receipt("1000.3",claims[0].sha256);
  await expect(launchContinuationSlot(policyRef,"1000.3",nextReceipt,operations)).rejects.toThrow("external recovery");expect(calls).toEqual(["1000.2"]);
  for(const bad of ["1000.1","1000.2","1000.6","10000.1"])await expect(launchContinuationSlot(policyRef,bad,nextReceipt,operations)).rejects.toThrow();
  for(const slot of CONTINUATION_SLOTS.slice(1)) {
    const prior=(await continuationClaims(policy)).at(-1)!;const output=await launchContinuationSlot(policyRef,slot,await receipt(slot,prior.sha256,slot==="1000.3"),operations);
    expect(output.exitCode).toBe(1);expect(output.cleanup).toBe("UNKNOWN_REQUIRES_INDEPENDENT_REVALIDATION");
  }
  expect(calls).toEqual(CONTINUATION_SLOTS);expect(checks).toHaveLength(9);expect(await continuationClaims(policy)).toHaveLength(9);
  const budget=JSON.parse(await readFile(`${policy.ledgerRoot}/budget-after-10000.5.json`,"utf8"));expect(budget.consumed).toBe(10);
  expect(budget.validCompleteBySize).toEqual({1000:0,10000:0});expect(budget.qualificationCohort).toBe(false);expect(budget.slots[0].originalScored).toEqual(seed.originalScores);
  // Constructed results preserve target misses while reconciling process failure independently.
  for(const slot of ["1000.3","1000.4"]) {
    const dir=`${policy.attemptRoots[slot]}/raw/run-${slot.split(".")[1].padStart(2,"0")}/1000`;
    await mkdir(dir,{recursive:true});await writeFile(`${dir}/result.json`,JSON.stringify({collection:{evidenceValidity:"VALID",collectionCompleteness:"COMPLETE"},scored:{targetOutcome:"FAIL"}}));
  }
  await writeFile(`${policy.ledgerRoot}/terminal-1000.4.json`,JSON.stringify({exitCode:0,signal:null}));
  const reconciled=await continuationBudgetReport(policy,seed);
  expect(reconciled.validCompleteBySize).toEqual({1000:1,10000:0});
  expect(reconciled.slots.find((s:any)=>s.slot==="1000.3")?.runtimeDisposition).toBe("FAILED_OR_UNAVAILABLE");
  expect(reconciled.slots.find((s:any)=>s.slot==="1000.4")?.originalScored).toEqual({targetOutcome:"FAIL"});
  const altered={...policy,method:{...policy.method,sha256:"c".repeat(64)}};await expect(continuationClaims(altered)).rejects.toThrow("untracked method");
  const moved=await save("moved-policy.json",{...policy,ledgerRoot:`${directory}/another-ledger`});await expect(launchContinuationSlot(moved,"1000.2",nextReceipt,operations)).rejects.toThrow("relocation/reset");
});

for (const [id, expectedEnabled] of [["point-selection-1-stopped", false], ["orbit-1-ready", true]] as const) {
  test(`phase label rejection preserves actual boundary metadata: ${id}`, async ({}, info) => {
    const {validateBoundaryWithRejectionRecord} = await import("./characterization-commands");
    const previous=metadataFixture();previous.id="initial-presentation";
    const expected={runId:previous.runId,fixtureSize:1000,runNumber:2,bindings:previous.bindings};
    const value=structuredClone(previous);value.id=id;value.snapshot.viewport.labels.enabled=!expectedEnabled;
    const directory=info.outputPath("phase-label-rejection");await mkdir(directory,{recursive:true});
    await expect(validateBoundaryWithRejectionRecord(value,expected,previous,directory)).rejects.toThrow("phase label policy mismatch");
    const rejected=JSON.parse(await readFile(`${directory}/${id}-rejected.json`,"utf8"));
    expect(rejected.status).toBe("REJECTED_BOUNDARY_METADATA");
    expect(rejected.actual).toEqual(value);
    expect(rejected.actual.snapshot.viewport.labels.enabled).toBe(!expectedEnabled);
    expect(rejected.expected).toEqual(expected);expect(rejected.reference).toEqual(previous);
    expect(rejected.error).toBe("Error: phase label policy mismatch");
    const allowed=structuredClone(value);allowed.snapshot.viewport.labels.enabled=expectedEnabled;
    expect(()=>validateBoundaryMetadata(allowed,expected,previous)).not.toThrow();
  });
}

test("captured scrollable inspector dimensions remain observations while true layout drift rejects", async ({}, info) => {
  const file=process.env.D70_CONTENT_WIDTH_REJECTED;
  test.skip(!file,"requires the hash-bound continuation-smoke-01 rejected snapshot");
  const bytes=await readFile(file!);
  expect(createHash("sha256").update(bytes).digest("hex")).toBe("9d7b0d55113f13dec928ad44a7a383825021fc83d4cf9cbcae519d194f128169");
  const {actual,reference,expected}=JSON.parse(bytes.toString());
  expect(reference.presentation.panels[1]).toMatchObject({width:338,height:299.96875});
  expect(actual.presentation.panels[1]).toMatchObject({width:323,height:1530.625});
  expect(actual.presentation.panes).toEqual(reference.presentation.panes);
  expect(actual.snapshot.viewport.canvas).toEqual(reference.snapshot.viewport.canvas);
  expect(actual.snapshot.viewport.selection.primaryRef).toEqual({type:"pipe",id:"pipe:UIF-00205"});
  validateBoundaryMetadata(actual,expected,reference);
  expect(actual.contentGeometryTransitions).toEqual([
    {field:"presentation.panels.1.width",before:338,after:323},
    {field:"presentation.panels.1.height",before:299.96875,after:1530.625}
  ]);
  const {validateBoundaryWithRejectionRecord}=await import("./characterization-commands");
  const directory=info.outputPath("actual-layout-rejections");await mkdir(directory,{recursive:true});
  const mutations=[(v:any)=>v.presentation.panes[1].width--,(v:any)=>v.presentation.panes[0].x++,
    (v:any)=>v.snapshot.viewport.canvas.cssLeft++,(v:any)=>v.presentation.panels[1].selector=".other",
    (v:any)=>v.presentation.panels[1].visible=false,(v:any)=>v.presentation.panels[1].width=0,
    (v:any)=>v.presentation.panels[1].height=0];
  for(const [index,mutate] of mutations.entries()) {
    const bad=structuredClone(actual);bad.id=`content-width-negative-${index}`;mutate(bad);
    await expect(validateBoundaryWithRejectionRecord(bad,expected,reference,directory)).rejects.toThrow();
    const rejected=JSON.parse(await readFile(`${directory}/${bad.id}-rejected.json`,"utf8"));
    expect(rejected.actual).toEqual(bad);expect(rejected.error).toMatch(/metadata|drift/);
  }
});

for (const outcomes of [["target-miss"],["invalid","valid"],["invalid","invalid","invalid","invalid","invalid"],["interrupted","valid"],["empty-exit0","valid"],["stale-success"]]) {
  test(`owner one-success transition preserves history and bounded dispatch: ${outcomes.join(",")}`,async({},info)=>{
    const original=process.env.D70_WRITER_ORIGINAL_RETURN;test.skip(!original,"requires hash-bound original seed");
    const {launchContinuationSlot,closeOneSuccess,continuationClaims,continuationBudgetReport,continuationExternalBindings,CONTINUATION_SLOTS,ORIGINAL_RETURN_SHA256}=await import("./characterization-observations.mjs");
    const {dirname,resolve}=await import("node:path");
    const root=info.outputPath("one-success");await mkdir(`${root}/ledger/claims`,{recursive:true});
    let sequence=0;
    const save=async(file:string,value:any)=>{const bytes=JSON.stringify(value);await writeFile(file,bytes,{flag:"wx"});return {path:file,sha256:createHash("sha256").update(bytes).digest("hex")};};
    const seed=JSON.parse(await readFile(original!,"utf8")),seedRef={path:original!,sha256:ORIGINAL_RETURN_SHA256};
    const files=Array.from({length:34},(_,i)=>({path:`apps/desktop/e2e/ui-foundation/${i===0?"characterization-observations.mjs":`frozen-${i}.ts`}`,sha256:"a".repeat(64)}));
    const oldMethod=await save(`${root}/old-method.json`,{files}),method=await save(`${root}/method.json`,{files:files.map((f,i)=>i?f:{...f,sha256:"b".repeat(64)})});
    const prior:any={schema:"ui-foundation.continuation-policy/v1",seed:seedRef,cohortId:seed.frozenEnvironment.UI_FOUNDATION_COHORT_ID,ledgerRoot:`${root}/ledger`,instrumentProjectRoot:root,instrumentRevision:"a".repeat(40),method:oldMethod,
      attemptRoots:Object.fromEntries(CONTINUATION_SLOTS.map(slot=>[slot,`${root}/attempt-${slot}`]))};
    const priorRef=await save(`${root}/prior.json`,prior),history:any[]=[],originalHashes:any[]=[];
    const resultFor=(claim:any,status:string)=>{
      const [size,ordinal]=claim.slot.split("."),expected={runId:`${claim.cohortId}.${claim.slot}`,fixtureSize:Number(size),runNumber:Number(ordinal),sessionId:`session-${claim.slot}`,bindings:{methodSha256:claim.methodSha256}};
      return {expected,evidence:{...expected,freshSession:true,qualification:"PASS_QUALIFIED_RUN",points:Array(200).fill({}),boxes:Array(20).fill({}),filters:Array(20).fill({})},segmentCount:243,errors:[],scored:{status:status==="target-miss"?"FAIL_TARGETS":"PASS_METRIC_ACCEPTANCE",validityFailures:[],targetFailures:status==="target-miss"?["POINT_TARGET"]:[]},collection:{evidenceValidity:"VALID",collectionCompleteness:"COMPLETE"}};
    };
    const persistResult=async(claim:any,status:string)=>{
      const [size,ordinal]=claim.slot.split("."),dir=`${claim.evidenceRoot}/raw/run-${ordinal.padStart(2,"0")}/${size}`;await mkdir(dir,{recursive:true});return save(`${dir}/result.json`,resultFor(claim,status));
    };
    let previous=ORIGINAL_RETURN_SHA256;
    for(const [i,slot] of ["1000.2","1000.3","1000.4"].entries()) {
      const claim={slot,cohortId:prior.cohortId,seedSha256:ORIGINAL_RETURN_SHA256,policySha256:priorRef.sha256,methodSha256:prior.method.sha256,instrumentRevision:prior.instrumentRevision,evidenceRoot:prior.attemptRoots[slot],previousClaimSha256:previous,claimedAt:"2026-09-18T00:00:00Z"};
      const c=await save(`${prior.ledgerRoot}/claims/${slot}.json`,claim),t=await save(`${prior.ledgerRoot}/terminal-${slot}.json`,{slot,claim:c,exitCode:i===2?null:i,signal:i===2?"SIGTERM":null,cleanup:"UNKNOWN_REQUIRES_INDEPENDENT_REVALIDATION"});
      const evidence=[c,t];if(i===0)evidence.push(await persistResult(claim,"valid"));
      const returned=await save(`${root}/return-${slot}.json`,{slot,processExit:i,collection:{evidenceValidity:i?"INVALID":"VALID",collectionCompleteness:i?"INCOMPLETE":"COMPLETE"},cleanup:{verificationStatus:"VERIFIED",browserProcessesRemaining:0,serverListening:false},externalBindings:"PASS_INDEPENDENT_EXTERNAL_REVALIDATION",evidence,
        ...(i===2?{status:"OWNER_CANCELLED_STARTED_SLOT_INTERRUPTED",startedBeforeSteering:true,externalRecovery:{verificationStatus:"VERIFIED",browserProcessesRemaining:0,serverListening:false}}:{})});
      history.push({claim:c,terminal:t,return:returned});originalHashes.push(c,t,returned);previous=c.sha256;
    }
    const registry=await save(`${root}/original-registry.json`,{seedSha256:ORIGINAL_RETURN_SHA256,cohortId:prior.cohortId,ledgerRoot:prior.ledgerRoot,policySha256:priorRef.sha256,methodSha256:prior.method.sha256,instrumentRevision:prior.instrumentRevision});
    const authority={path:resolve(dirname(original!),"../../OWNER_DIRECTION_ONE_SUCCESS_20260917.md"),sha256:"ba5e8bceea55838cc0d23e815cc9a890ed543534085a9e9bdd932d133fb37fa2"};
    const policy:any={...prior,instrumentRevision:"b".repeat(40),method,ownerTransition:{schema:"ui-foundation.one-success-transition/v1",authority,previousPolicy:priorRef,registry,history}};
    const policyRef=await save(`${root}/policy.json`,policy),calls:string[]=[];
    const receipt=async(slot:string,previousHash:string,recovery:boolean,dirty=false)=>{
      const n=sequence++,cleanup=await save(`${root}/cleanup-${n}.json`,{previousClaimSha256:previousHash,browserProcessesRemaining:dirty?1:0,serverListening:false,verificationStatus:"VERIFIED",processDisposition:recovery?"EXTERNAL_RECOVERY_VERIFIED":"NORMAL_EXIT"});
      const bindings=await save(`${root}/bindings-${n}.json`,{status:"PASS_INDEPENDENT_EXTERNAL_REVALIDATION",externalBindings:continuationExternalBindings(seed)});
      return save(`${root}/receipt-${n}.json`,{schema:"ui-foundation.continuation-preconditions/v1",slot,previousClaimSha256:previousHash,verifiedAt:new Date().toISOString(),cleanup:{status:"VERIFIED_NO_REMAINING_BROWSER_OR_SERVER",evidence:cleanup},bindingsStatus:"VERIFIED_UNCHANGED",externalBindings:continuationExternalBindings(seed),bindingEvidence:bindings,evidence:[cleanup,bindings]});
    };
    const operations={bindRegistry:async(_file:string,value:any)=>expect(value.policySha256).toBe(priorRef.sha256),verifyFiles:async()=>seed.frozenEnvironment,spawn:async(_p:any,env:any)=>{
      const claim=JSON.parse(await readFile(env.UI_FOUNDATION_CONTINUATION_CLAIM,"utf8"));calls.push(claim.slot);
      const outcome=outcomes[calls.length-1];if(outcome==="interrupted")throw new Error("interrupted actual process");
      if(["valid","target-miss"].includes(outcome))await persistResult(claim,outcome);
      return {exitCode:outcome==="invalid"?1:0,signal:null};
    }};
    const firstReceipt=await receipt("10000.1",history[2].claim.sha256,true);
    const wrongAuthority=await save(`${root}/wrong-authority-policy.json`,{...policy,ownerTransition:{...policy.ownerTransition,authority:{...authority,sha256:"0".repeat(64)}}});
    await expect(launchContinuationSlot(wrongAuthority,"10000.1",firstReceipt,operations)).rejects.toThrow("authority");
    const wrongHistory=await save(`${root}/wrong-history-policy.json`,{...policy,ownerTransition:{...policy.ownerTransition,history:history.map((h,i)=>i===1?{...h,claim:{...h.claim,sha256:"0".repeat(64)}}:h)}});
    await expect(launchContinuationSlot(wrongHistory,"10000.1",firstReceipt,operations)).rejects.toThrow("bound JSON changed");
    await expect(launchContinuationSlot(policyRef,"10000.1",await receipt("10000.1",history[2].claim.sha256,false),operations)).rejects.toThrow("external recovery");
    for(const bad of ["1000.5","1000.4","10000.2","10000.6"])await expect(launchContinuationSlot(policyRef,bad,firstReceipt,operations)).rejects.toThrow();
    await expect(launchContinuationSlot(policyRef,"10000.1",firstReceipt,{...operations,verifyFiles:async()=>{throw new Error("binding drift");}})).rejects.toThrow("binding drift");
    expect(await continuationClaims(policy)).toHaveLength(3);expect(calls).toEqual([]);
    for(const [i,outcome] of outcomes.entries()) {
      const slot=`10000.${i+1}`,last=(await continuationClaims(policy)).at(-1)!;
      const output=await launchContinuationSlot(policyRef,slot,await receipt(slot,last.sha256,i===0||outcomes[i-1]==="interrupted"),operations);
      expect(output.slot).toBe(slot);expect((await continuationClaims(policy)).at(-1)!.claim.methodSha256).toBe(method.sha256);
      await expect(launchContinuationSlot(policyRef,slot,firstReceipt,operations)).rejects.toThrow();
      if(outcome==="stale-success") {
        const current=(await continuationClaims(policy)).at(-1)!;
        await expect(launchContinuationSlot(policyRef,"10000.2",await receipt("10000.2",current.sha256,false),{
          ...operations,verifyFiles:async()=>{await persistResult(current.claim,"valid");return seed.frozenEnvironment;}
        })).rejects.toThrow("stale concurrent launch");
        expect(await continuationClaims(policy)).toHaveLength(4);expect(calls).toEqual(["10000.1"]);
      }
      if(["valid","target-miss","stale-success"].includes(outcome)) {
        const current=(await continuationClaims(policy)).at(-1)!;
        await expect(launchContinuationSlot(policyRef,`10000.${i+2}`,firstReceipt,operations)).rejects.toThrow();
        await expect(closeOneSuccess(policyRef,await receipt(slot,current.sha256,false,true),operations)).rejects.toThrow("cleanup");
        const closed=await closeOneSuccess(policyRef,await receipt(slot,current.sha256,false),operations);
        expect(closed.waivedUnattempted).toHaveLength(4-i);expect(closed.targetAcceptanceClaim).toBe(false);
        await expect(launchContinuationSlot(policyRef,`10000.${i+2}`,firstReceipt,operations)).rejects.toThrow();
      }
    }
    const report=await continuationBudgetReport(policy,seed);
    expect(report.slots.find((s:any)=>s.slot==="1000.5")).toMatchObject({consumed:false,disposition:"WAIVED_UNATTEMPTED_BY_OWNER"});
    expect(report.slots.find((s:any)=>s.slot==="1000.4")).toMatchObject({consumed:true,ownerDisposition:"OWNER_INTERRUPTED_CONSUMED"});
    expect(report.consumed).toBe(4+outcomes.length);expect(calls).toEqual(outcomes.map((_,i)=>`10000.${i+1}`));
    expect(report.interrupted).toBe(1+outcomes.filter(o=>o==="interrupted").length);
    expect(report.invalidFailed).toBe(2+outcomes.filter(o=>["invalid","empty-exit0"].includes(o)).length);
    expect(report.confirmedSuccessBySize).toEqual({1000:1,10000:outcomes.some(o=>["valid","target-miss","stale-success"].includes(o))?1:0});
    if(outcomes.length===5){expect(report.ownerStoppingStatus).toBe("FIVE_ATTEMPTS_EXHAUSTED_NO_SUCCESS");expect(report.waived).toBe(1);await expect(closeOneSuccess(policyRef,firstReceipt,operations)).rejects.toThrow();}
    else expect(report.ownerStoppingStatus).toBe("SUCCESS_CONFIRMED_AND_REMAINDER_WAIVED");
    for(const ref of [...originalHashes,registry,priorRef])expect(createHash("sha256").update(await readFile(ref.path)).digest("hex")).toBe(ref.sha256);
  });
}

test("explicit internal120 transition changes only authorized profile bindings and retains legacy60 gates",async({},info)=>{
  const original=process.env.D70_WRITER_ORIGINAL_RETURN;test.skip(!original,"requires frozen seed and owner direction");
  const {validateDisplayTransition,continuationProfileAuthorization,continuationExternalBindings,INTERNAL120_AUTHORITY_SHA256,ORIGINAL_RETURN_SHA256}=await import("./characterization-observations.mjs");
  const {bindReferenceProfile}=await import("./characterization-mode");
  const {dirname,resolve}=await import("node:path");
  const directory=info.outputPath("internal120");await mkdir(directory,{recursive:true});
  const save=async(name:string,value:any)=>{const file=`${directory}/${name}`,bytes=JSON.stringify(value);await writeFile(file,bytes,{flag:"wx"});return {path:file,sha256:createHash("sha256").update(bytes).digest("hex")};};
  const seed=JSON.parse(await readFile(original!,"utf8"));
  const previousProfile={path:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE,sha256:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE_SHA256};
  const legacy=JSON.parse(await readFile(previousProfile.path,"utf8"));
  const display={name:"Color LCD",vendor:"610",product:"a05f",serial:"fd626d62",pixels:"3456 x 2234",resolution:"1728 x 1117 @ 120.00Hz",mirror:"spdisplays_off",connection:"spdisplays_internal"};
  const internal={...legacy,refreshHz:120,display}; // deterministic test profile; not a live verification claim
  const profile=await save("profile.json",internal);
  const authority={path:resolve(dirname(original!),"../../OWNER_DIRECTION_INTERNAL120_20260917.md"),sha256:INTERNAL120_AUTHORITY_SHA256};
  const policy:any={seed:{path:original!,sha256:ORIGINAL_RETURN_SHA256},cohortId:legacy.cohortId,ownerTransition:{},method:{sha256:"c".repeat(64)},displayTransition:{schema:"ui-foundation.internal120-transition/v1",authority,previousProfile,profile}};
  const transition=await validateDisplayTransition(policy,seed);expect(transition.record).toEqual(internal);
  const base=continuationExternalBindings(seed),next=continuationExternalBindings(seed,policy);
  expect(Object.keys(next).filter(k=>next[k]!==base[k])).toEqual(["UI_FOUNDATION_REFERENCE_PROFILE","UI_FOUNDATION_REFERENCE_PROFILE_SHA256"]);
  expect(next.UI_FOUNDATION_REFERENCE_PROFILE).toBe(profile.path);expect(next.UI_FOUNDATION_REFERENCE_PROFILE_SHA256).toBe(profile.sha256);
  const host={model:"Apple M5 Max",memoryBytes:128*1024**3};
  expect(()=>validateReferenceProfile(internal,legacy.cohortId,host)).toThrow();
  expect(()=>validateReferenceProfile(legacy,legacy.cohortId,host)).not.toThrow();
  const policyRef=await save("policy.json",policy),claim=await save("claim.json",{slot:"10000.1",policySha256:policyRef.sha256,methodSha256:policy.method.sha256,executionToken:"token"});
  const env={...seed.frozenEnvironment,...next,UI_FOUNDATION_CONTINUATION_POLICY:policyRef.path,UI_FOUNDATION_CONTINUATION_POLICY_SHA256:policyRef.sha256,
    UI_FOUNDATION_CONTINUATION_CLAIM:claim.path,UI_FOUNDATION_CONTINUATION_CLAIM_SHA256:claim.sha256,UI_FOUNDATION_CONTINUATION_TOKEN:"token"};
  const authorization=await continuationProfileAuthorization(env);
  expect(()=>validateReferenceProfile(internal,legacy.cohortId,host,authorization)).not.toThrow();
  expect(()=>validateReferenceProfile(legacy,legacy.cohortId,host,authorization)).toThrow();
  expect((await bindReferenceProfile(env)).record.refreshHz).toBe(120);
  await expect(bindReferenceProfile({...env,UI_FOUNDATION_CONTINUATION_POLICY:undefined,UI_FOUNDATION_CONTINUATION_POLICY_SHA256:undefined})).rejects.toThrow();
  const oldClaim=await save("1000-claim.json",{slot:"1000.5",policySha256:policyRef.sha256,methodSha256:policy.method.sha256,executionToken:"token"});
  await expect(continuationProfileAuthorization({...env,UI_FOUNDATION_CONTINUATION_CLAIM:oldClaim.path,UI_FOUNDATION_CONTINUATION_CLAIM_SHA256:oldClaim.sha256})).rejects.toThrow("authorized10000 claim");
  await expect(validateDisplayTransition({...policy,displayTransition:{...policy.displayTransition,authority:{...authority,sha256:"0".repeat(64)}}},seed)).rejects.toThrow("owner/profile lineage");
  await expect(validateDisplayTransition({...policy,displayTransition:{...policy.displayTransition,previousProfile:profile}},seed)).rejects.toThrow("lineage");
  for(const [index,changed] of [{...internal,refreshHz:60},{...internal,viewport:[100,100]},{...internal,display:{...display,connection:"external"}}].entries()) {
    const bad=await save(`bad-profile-${index}.json`,changed);
    await expect(validateDisplayTransition({...policy,displayTransition:{...policy.displayTransition,profile:bad}},seed)).rejects.toThrow("reference profile mismatch");
  }
  const raw={SPDisplaysDataType:[{sppci_model:"Apple M5 Max",spdisplays_ndrvs:[{_name:display.name,"_spdisplays_display-vendor-id":display.vendor,"_spdisplays_display-product-id":display.product,
    "_spdisplays_display-serial-number":display.serial,_spdisplays_pixels:display.pixels,_spdisplays_resolution:display.resolution,spdisplays_mirror:display.mirror,spdisplays_connection_type:display.connection,spdisplays_main:"spdisplays_yes",spdisplays_online:"spdisplays_yes"}]}]};
  expect(validateDisplayProfile(raw,display)).toEqual(display);
  expect(()=>validateDisplayProfile(raw,legacy.display)).toThrow();
  for(const [key,value] of [["_spdisplays_resolution","1728 x 1117 @ 60.00Hz"],["_spdisplays_display-serial-number","wrong"],["spdisplays_connection_type","external"],["_spdisplays_pixels","999 x 999"]]) {
    const drift=structuredClone(raw);(drift.SPDisplaysDataType[0].spdisplays_ndrvs[0] as any)[key]=value;
    expect(()=>validateDisplayProfile(drift,display)).toThrow();
  }
  expect(()=>validateDisplayProfile(raw,display,{...display,resolution:"other"})).toThrow();
});

test('fresh demonstration provenance accepts only its bound revision and internal120 profile', async()=>{
  const {validateFinalProductProvenance}=await import('./full-cohort-controller');
  const revision='b'.repeat(40);
  expect(()=>validateFinalProductProvenance('final',revision,revision,false)).toThrow();
  expect(()=>validateFinalProductProvenance('final',revision,revision,false,revision)).not.toThrow();
  expect(()=>validateFinalProductProvenance('final',revision,CHARACTERIZATION_PRODUCT_REVISION,false,revision)).toThrow();
  expect(()=>validateFinalProductProvenance('final',revision,revision,true,revision)).toThrow();
  const profile={schema:'ui-foundation.reference-profile/v1',cohortId:'fresh',productRevision:revision,hostModel:'Apple M5 Max',memoryBytes:128*1024**3,
    refreshHz:120,externallyVerified:true,verificationEvidence:'frozen evidence hash',verifiedAt:'2026-09-18T00:00:00Z',viewport:[1440,920],browserDpr:2,effectiveDprCap:2,
    display:{name:'Color LCD',connection:'spdisplays_internal',resolution:'1728 x 1117 @ 120.00Hz'}};
  const host={model:profile.hostModel,memoryBytes:profile.memoryBytes},authorization={refreshHz:120,freshProductRevision:revision};
  expect(()=>validateReferenceProfile(profile,'fresh',host,authorization)).not.toThrow();
  expect(()=>validateReferenceProfile(profile,'fresh',host)).toThrow();
  expect(()=>validateReferenceProfile({...profile,productRevision:CHARACTERIZATION_PRODUCT_REVISION},'fresh',host,authorization)).toThrow();
  expect(()=>validateReferenceProfile({...profile,refreshHz:60},'fresh',host,authorization)).toThrow();
});
