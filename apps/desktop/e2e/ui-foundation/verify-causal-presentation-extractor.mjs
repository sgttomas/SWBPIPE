import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";
import { readFile, writeFile } from "node:fs/promises";
import {
  CausalPresentationExtractionError,
  REQUIRED_CHROMIUM_BINDING,
  PAGE_CLOCK_SOURCE,
  extractCausalPresentations as currentExtract,
  inventoryCausalPresentationLineages as currentInventory,
  presentationGapsForMeasuredWindow
} from "./causal-presentation-extractor.mjs";
import { parseTraceEventsWithUnsafeIntegersAsDecimalStrings } from "./lossless-trace-json.mjs";

const referenceIndex = process.argv.indexOf("--reference");
const referencePath = referenceIndex < 0 ? null : process.argv[referenceIndex + 1];
const referenceSha256 = "6fbef3d327347a34e69d49ca226db9fb6f78c5042a36ea08f5a7a2427ebb7963";
if (referenceIndex >= 0 && !referencePath) throw new Error("--reference requires immutable original extractor path");
if (referencePath) assert.equal(createHash("sha256").update(await readFile(referencePath)).digest("hex"), referenceSha256);
const reference = referencePath ? await import(pathToFileURL(referencePath).href) : null;
let differentialCalls = 0;
const outcome = (fn, args) => {
  try { return { result: fn(...args) }; }
  catch (error) { return { error: { name: error.name, code: error.code, message: error.message, context: error.context } }; }
};
function extractCausalPresentations(...args) {
  if (reference) {
    assert.deepEqual(outcome(currentExtract, args), outcome(reference.extractCausalPresentations, args));
    assert.deepEqual(outcome(currentInventory, args), outcome(reference.inventoryCausalPresentationLineages, args));
    differentialCalls++;
  }
  return currentExtract(...args);
}

const schema = "openpipestress.ui-foundation.causal-feedback-marker/v1";
const rendererPid = 41;
const mainTid = 7;
const compositorTid = 9;
const actionToken = "synthetic.orbit";
const actionIdentity = { gesture: "frozen-orbit-pointer-path", warmupMs: 500, measuredMs: 2_000 };

const clone = (value) => structuredClone(value);
const event = (name, ts, ph, tid, args = {}, extra = {}) => ({ name, ts, ph, pid: rendererPid, tid, args, ...(name === "Layerize" ? { cat: "devtools.timeline" } : {}), ...extra });

function buildLineage(index, presentationTs) {
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

function buildFixture(presentationTimes = [1_100_000]) {
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

function expectError(code, callback) {
  try {
    callback();
  } catch (error) {
    if (error instanceof CausalPresentationExtractionError && error.code === code) return;
    throw new Error(`expected ${code}, received ${error?.code ?? String(error)}`);
  }
  throw new Error(`expected ${code}, but extraction passed`);
}

const cases = [];
const positive = buildFixture();
const positiveResult = extractCausalPresentations(positive.events, REQUIRED_CHROMIUM_BINDING, positive.markerEvidence);
cases.push({ name: "exact lineage with unrelated boundary crossings", status: positiveResult.status,
  unmatched: positiveResult.reporterBoundaryDiagnostics });

// Independent expectations fixed before execution: equal page readings do not
// collapse a marker's true clock interval, in either isolation mode.
for (const isolated of [false, true]) {
  const quantized = buildFixture();
  quantized.markerEvidence.active.actionMarker.traceClock = {
    source: PAGE_CLOCK_SOURCE, crossOriginIsolated: isolated, before: 100, after: 100 };
  const feedback = quantized.markerEvidence.active.feedbackMarkers[0];
  feedback.traceClock = { source: PAGE_CLOCK_SOURCE, crossOriginIsolated: isolated, before: 120.1, after: 120.1 };
  const result = extractCausalPresentations(quantized.events, REQUIRED_CHROMIUM_BINDING, quantized.markerEvidence).results[0];
  // Offsets intersect [899.9,900.0]; listener uncertainty extends duration
  // [100,100.1] to [99.9,100.2]. Outward machine rounding may widen further.
  if (!(result.pageToTraceOffsetIntervalMs.minimum <= 899.9 && result.pageToTraceOffsetIntervalMs.maximum >= 900 &&
        result.actionToPresentationIntervalMs.lower <= 99.9 && result.actionToPresentationIntervalMs.upper >= 100.2 &&
        result.actionToPresentationIntervalMs.upper < 100.200001)) throw new Error("quantized clock/listener bound is incorrect");
  cases.push({ name: `compatible source quantization isolated=${isolated}`, status: "PASS", interval: result.actionToPresentationIntervalMs });
}
const disjoint = clone(positive);
disjoint.markerEvidence.active.feedbackMarkers[0].traceClock.before += 1;
disjoint.markerEvidence.active.feedbackMarkers[0].traceClock.after += 1;
expectError("INCONSISTENT_PAGE_TRACE_CLOCK_MAPPING", () =>
  extractCausalPresentations(disjoint.events, REQUIRED_CHROMIUM_BINDING, disjoint.markerEvidence));
cases.push({ name: "truly disjoint source-bounded clocks rejected", status: "PASS_EXPECTED_REJECTION" });
const malformed = clone(positive);
malformed.markerEvidence.active.actionMarker.traceClock.before = 101;
expectError("INVALID_PAGE_TRACE_CLOCK_BOUNDS", () =>
  extractCausalPresentations(malformed.events, REQUIRED_CHROMIUM_BINDING, malformed.markerEvidence));
cases.push({ name: "reversed clock bracket rejected", status: "PASS_EXPECTED_REJECTION" });
const unboundClock = clone(positive);
unboundClock.markerEvidence.active.actionMarker.traceClock.source.revision = "@wrong";
expectError("PAGE_CLOCK_SOURCE_BINDING_MISMATCH", () =>
  extractCausalPresentations(unboundClock.events, REQUIRED_CHROMIUM_BINDING, unboundClock.markerEvidence));
cases.push({ name: "clock source revision mismatch rejected", status: "PASS_EXPECTED_REJECTION" });
const unknownIsolation = clone(positive);
delete unknownIsolation.markerEvidence.active.actionMarker.traceClock.crossOriginIsolated;
expectError("INVALID_PAGE_CLOCK_ISOLATION_EVIDENCE", () =>
  extractCausalPresentations(unknownIsolation.events, REQUIRED_CHROMIUM_BINDING, unknownIsolation.markerEvidence));
cases.push({ name: "missing prospective isolation evidence rejected", status: "PASS_EXPECTED_REJECTION" });

const orbit = buildFixture([1_100_000, 1_180_000, 1_240_000, 1_320_000]);
const orbitResult = extractCausalPresentations(orbit.events, REQUIRED_CHROMIUM_BINDING, orbit.markerEvidence);
const gaps = presentationGapsForMeasuredWindow(orbitResult,
  { startTraceTimestamp: 1_150_000, endTraceTimestamp: 1_250_000 });
cases.push({ name: "preceding, measured and trailing orbit coverage", status: gaps.status,
  presentationGapsMs: gaps.presentationGapsMs });

const raw = '{"traceEvents":[{"ts":1,"args":{"surface_frame_trace_id":2576121716840409001}}]}';
const lossless = parseTraceEventsWithUnsafeIntegersAsDecimalStrings(raw);
if (lossless.events[0].args.surface_frame_trace_id !== "2576121716840409001" || lossless.convertedCount !== 1) {
  throw new Error("lossless raw parser did not preserve the unsafe integer token exactly");
}
cases.push({ name: "raw unsafe integer becomes exact decimal string", status: "PASS", convertedCount: lossless.convertedCount });
let unsafeFractionRejected = false;
try {
  parseTraceEventsWithUnsafeIntegersAsDecimalStrings('{"traceEvents":[{"ts":1e309}]}');
} catch {
  unsafeFractionRejected = true;
}
if (!unsafeFractionRejected) throw new Error("lossless raw parser accepted an unsafe non-integer number");
cases.push({ name: "unsafe non-integer raw number rejected", status: "PASS_EXPECTED_REJECTION" });

expectError("CHROMIUM_SOURCE_BINDING_MISMATCH", () =>
  extractCausalPresentations(positive.events, { ...REQUIRED_CHROMIUM_BINDING, revision: "@wrong" }, positive.markerEvidence));
cases.push({ name: "unbound Chromium revision rejected", status: "PASS_EXPECTED_REJECTION" });

const rounded = clone(positive);
rounded.events.find((candidate) => candidate.args?.chrome_graphics_pipeline?.step === "STEP_GENERATE_COMPOSITOR_FRAME")
  .args.chrome_graphics_pipeline.surface_frame_trace_id = 2576121716840409001;
expectError("NONEXACT_64_BIT_IDENTIFIER", () =>
  extractCausalPresentations(rounded.events, REQUIRED_CHROMIUM_BINDING, rounded.markerEvidence));
cases.push({ name: "rounded Number ID rejected", status: "PASS_EXPECTED_REJECTION" });

const missingUpdate = clone(positive);
missingUpdate.events = missingUpdate.events.filter((candidate) => candidate.name !== "LayerTreeHost::DoUpdateLayers");
expectError("AMBIGUOUS_OR_MISSING_POST_MARKER_UPDATE_LAYERS", () =>
  extractCausalPresentations(missingUpdate.events, REQUIRED_CHROMIUM_BINDING, missingUpdate.markerEvidence));
cases.push({ name: "missing update rejected", status: "PASS_EXPECTED_REJECTION" });

const partialReporter = clone(positive);
partialReporter.events = partialReporter.events.filter((candidate) => !(candidate.name === "PipelineReporter" && candidate.ph === "e" && candidate.cat === "cc,benchmark"));
expectError("AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE", () =>
  extractCausalPresentations(partialReporter.events, REQUIRED_CHROMIUM_BINDING, partialReporter.markerEvidence));
cases.push({ name: "partial reporter rejected", status: "PASS_EXPECTED_REJECTION" });

const partialState = clone(positive);
partialState.events.find((candidate) => candidate.name === "PipelineReporter" && candidate.ph === "b" && candidate.cat === "cc,benchmark")
  .args.frame_reporter.state = "STATE_PRESENTED_PARTIAL";
expectError("AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE", () =>
  extractCausalPresentations(partialState.events, REQUIRED_CHROMIUM_BINDING, partialState.markerEvidence));
cases.push({ name: "partial presentation rejected", status: "PASS_EXPECTED_REJECTION" });

const noSubmitStage = clone(positive);
noSubmitStage.events = noSubmitStage.events.filter((candidate) => candidate.name !== "SubmitCompositorFrameToPresentationCompositorFrame");
expectError("AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE", () =>
  extractCausalPresentations(noSubmitStage.events, REQUIRED_CHROMIUM_BINDING, noSubmitStage.markerEvidence));
cases.push({ name: "reporter without contained submit-stage start rejected", status: "PASS_EXPECTED_REJECTION" });

const duplicateReporter = clone(positive);
const compatibleBegin = clone(duplicateReporter.events.find((candidate) =>
  candidate.name === "PipelineReporter" && candidate.ph === "b" && candidate.cat === "cc,benchmark"));
const compatibleEnd = clone(duplicateReporter.events.find((candidate) =>
  candidate.name === "PipelineReporter" && candidate.ph === "e" && candidate.cat === "cc,benchmark"));
const compatibleSubmitStage = clone(duplicateReporter.events.find((candidate) =>
  candidate.name === "SubmitCompositorFrameToPresentationCompositorFrame"));
const compatibleSubmitEnd = clone(duplicateReporter.events.find((candidate) =>
  candidate.name === "SubmitCompositorFrameToPresentationCompositorFrame" && candidate.ph === "e"));
for (const candidate of [compatibleBegin, compatibleEnd, compatibleSubmitStage, compatibleSubmitEnd]) {
  candidate.id2.local = "8576121716840409999";
}
duplicateReporter.events.push(compatibleBegin, compatibleSubmitStage, compatibleSubmitEnd, compatibleEnd);
expectError("AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE", () =>
  extractCausalPresentations(duplicateReporter.events, REQUIRED_CHROMIUM_BINDING, duplicateReporter.markerEvidence));
cases.push({ name: "reused S with two compatible reporter occurrences rejected", status: "PASS_EXPECTED_REJECTION" });

const overlappingReporter = clone(positive);
const overlappingBegin = clone(overlappingReporter.events.find((candidate) =>
  candidate.name === "PipelineReporter" && candidate.ph === "b" && candidate.cat === "cc,benchmark"));
overlappingBegin.ts += 1;
overlappingReporter.events.push(overlappingBegin);
expectError("AMBIGUOUS_OVERLAPPING_ASYNC_PAIR", () =>
  extractCausalPresentations(overlappingReporter.events, REQUIRED_CHROMIUM_BINDING, overlappingReporter.markerEvidence));
cases.push({ name: "overlapping same-track reporter occurrence rejected", status: "PASS_EXPECTED_REJECTION" });

const wrongProcess = clone(positive);
wrongProcess.events.find((candidate) => candidate.name === "ProxyImpl::Commit").pid = rendererPid + 1;
expectError("AMBIGUOUS_OR_MISSING_MAIN_FRAME_PIPELINE_STEP", () =>
  extractCausalPresentations(wrongProcess.events, REQUIRED_CHROMIUM_BINDING, wrongProcess.markerEvidence));
cases.push({ name: "cross-process main-frame ID join rejected", status: "PASS_EXPECTED_REJECTION" });

const pointerMismatch = clone(positive);
pointerMismatch.markerEvidence.active.pointerTransaction.up.pointerId = 2;
expectError("INVALID_OR_INCOMPLETE_POINTER_TRANSACTION", () =>
  extractCausalPresentations(pointerMismatch.events, REQUIRED_CHROMIUM_BINDING, pointerMismatch.markerEvidence));
cases.push({ name: "unpaired input transaction rejected", status: "PASS_EXPECTED_REJECTION" });

// Delayed main origin B690 is submitted in current B701. The simultaneous impl
// fork and later current-origin ALL occurrence must never replace that main update.
const reporterBegin = (fixture) => fixture.events.find((e) => e.name === "PipelineReporter" && e.ph === "b" && e.cat === "cc,benchmark");
const sendEvent = (fixture) => fixture.events.find((e) => e.name === "SendBeginMainFrame");
const childEvent = (fixture, ph) => fixture.events.find((e) => e.name === "SubmitCompositorFrameToPresentationCompositorFrame" && e.ph === ph);
const delayed = clone(positive);
const mainReporter = reporterBegin(delayed);
const originalCurrentS = mainReporter.args.frame_reporter.surface_frame_trace_id;
sendEvent(delayed).ts = 990_000; // May precede the action marker; it still binds exact M.
sendEvent(delayed).args.main_frame_pipeline.begin_frame_id.sequence_number = 690;
delayed.events.find((e) => e.name === "ProxyMain::BeginMainFrame").args.begin_frame_id = 690;
mainReporter.ts = 985_000;
mainReporter.args.frame_reporter.frame_sequence = 690;
mainReporter.args.frame_reporter.surface_frame_trace_id = "2576121716840408951";
const outerEnd = delayed.events.find((e) => e.name === "PipelineReporter" && e.ph === "e" && e.cat === "cc,benchmark");
for (const [local, partial, stageShift, endShift] of [["fork-current", true, 0, 0], ["later-current", false, 40_000, 40_000]]) {
  const begin = clone(mainReporter), childBegin = clone(childEvent(delayed, "b")), childEnd = clone(childEvent(delayed, "e")), end = clone(outerEnd);
  for (const e of [begin, childBegin, childEnd, end]) e.id2.local = local;
  begin.args.frame_reporter.frame_sequence = 701;
  begin.args.frame_reporter.surface_frame_trace_id = originalCurrentS;
  if (partial) { begin.args.frame_reporter.state = "STATE_PRESENTED_PARTIAL"; begin.args.frame_reporter.frame_type = "FORKED"; }
  childBegin.ts += stageShift; childEnd.ts += endShift; end.ts += endShift;
  delayed.events.push(begin, childBegin, childEnd, end);
}
const delayedResult = extractCausalPresentations(delayed.events, REQUIRED_CHROMIUM_BINDING, delayed.markerEvidence).results[0];
if (delayedResult.originBeginFrameId.sequenceNumber !== "690" || delayedResult.currentBeginFrameId.sequenceNumber !== "701" ||
    delayedResult.reporterOriginSurfaceFrameTraceId !== "2576121716840408951" ||
    delayedResult.surfaceFrameTraceId !== originalCurrentS || delayedResult.presentationTraceTimestamp !== 1_100_000 ||
    delayedResult.reporterBeginEventIndex !== delayed.events.indexOf(mainReporter)) throw new Error("delayed origin selected an impl/later reporter");
cases.push({ name: "delayed main origin with simultaneous fork and later current ALL", status: "PASS", origin: delayedResult.originBeginFrameId });

const reused = clone(delayed);
const oldOccurrence = [reporterBegin(reused), childEvent(reused, "b"), childEvent(reused, "e"),
  reused.events.find((e) => e.name === "PipelineReporter" && e.ph === "e" && e.cat === "cc,benchmark")].map(clone);
oldOccurrence.forEach((e, index) => { e.ts = 800_000 + index * 1_000; });
reused.events.push(...oldOccurrence);
const reusedResult = extractCausalPresentations(reused.events, REQUIRED_CHROMIUM_BINDING, reused.markerEvidence).results[0];
if (reusedResult.presentationTraceTimestamp !== 1_100_000 || reusedResult.reporterEndEventIndex >= delayed.events.length) {
  throw new Error("disjoint local-track reuse selected wrong occurrence");
}
cases.push({ name: "disjoint reused reporter track binds intended complete occurrence", status: "PASS" });

const rejectMutation = (name, code, mutate) => {
  const fixture = clone(positive);
  mutate(fixture);
  expectError(code, () => extractCausalPresentations(fixture.events, REQUIRED_CHROMIUM_BINDING, fixture.markerEvidence));
  cases.push({ name, status: "PASS_EXPECTED_REJECTION" });
};
const noReporter = "AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE";
const noSend = "AMBIGUOUS_OR_MISSING_MAIN_FRAME_PIPELINE_STEP";
rejectMutation("missing origin Send", noSend, (f) => { f.events = f.events.filter((e) => e.name !== "SendBeginMainFrame"); });
rejectMutation("duplicate origin Send", noSend, (f) => { f.events.push(clone(sendEvent(f))); });
rejectMutation("wrong-process Send", noSend, (f) => { sendEvent(f).pid += 1; });
rejectMutation("wrong-compositor-thread Send", "MISBOUND_OR_LATE_SEND_BEGIN_MAIN_FRAME", (f) => { sendEvent(f).tid = mainTid; });
rejectMutation("Send after main execution", "MISBOUND_OR_LATE_SEND_BEGIN_MAIN_FRAME", (f) => { sendEvent(f).ts = 1_019_000; });
rejectMutation("misbound Send main-frame ID", noSend, (f) => { sendEvent(f).args.main_frame_pipeline.main_frame_id = "999"; });
rejectMutation("proxy origin sequence mismatch", "PROXY_ORIGIN_SEQUENCE_MISMATCH", (f) => { f.events.find((e) => e.name === "ProxyMain::BeginMainFrame").args.begin_frame_id += 1; });
rejectMutation("wrong reporter source with equal sequence", noReporter, (f) => { reporterBegin(f).args.frame_reporter.frame_source = 4294967297; });
rejectMutation("unsafe rounded Send source", "NONEXACT_BEGIN_FRAME_IDENTIFIER", (f) => { sendEvent(f).args.main_frame_pipeline.begin_frame_id.source_id = Number.MAX_SAFE_INTEGER + 1; });
rejectMutation("unsafe rounded reporter sequence", "NONEXACT_BEGIN_FRAME_IDENTIFIER", (f) => { reporterBegin(f).args.frame_reporter.frame_sequence = Number.MAX_SAFE_INTEGER + 1; });
rejectMutation("unsafe rounded origin surface ID", "NONEXACT_64_BIT_IDENTIFIER", (f) => { reporterBegin(f).args.frame_reporter.surface_frame_trace_id = 2576121716840409001; });
rejectMutation("only forked ALL reporter", noReporter, (f) => { reporterBegin(f).args.frame_reporter.frame_type = "FORKED"; });
rejectMutation("only backfill ALL reporter", noReporter, (f) => { reporterBegin(f).args.frame_reporter.frame_type = "BACKFILL"; });
rejectMutation("only dropped reporter", noReporter, (f) => { reporterBegin(f).args.frame_reporter.state = "STATE_DROPPED"; });
rejectMutation("reporter missing content", noReporter, (f) => { reporterBegin(f).args.frame_reporter.has_missing_content = true; });
rejectMutation("reporter needs raster", noReporter, (f) => { reporterBegin(f).args.frame_reporter.checkerboarded_needs_raster = true; });
rejectMutation("reporter needs recording", noReporter, (f) => { reporterBegin(f).args.frame_reporter.checkerboarded_needs_record = true; });
rejectMutation("origin reporter child starts outside generation", noReporter, (f) => { childEvent(f, "b").ts = 1_080_000; });
rejectMutation("missing child endpoint", noReporter, (f) => { f.events = f.events.filter((e) => e !== childEvent(f, "e")); });
rejectMutation("misassociated child endpoint namespace", noReporter, (f) => { childEvent(f, "e").scope = "other"; });
rejectMutation("child endpoint outside reporter occurrence", noReporter, (f) => { childEvent(f, "e").ts = 1_100_001; });
rejectMutation("child ends before current submission", noReporter, (f) => { childEvent(f, "b").ts = 1_044_000; childEvent(f, "e").ts = 1_045_000; });
rejectMutation("two compatible child occurrences", noReporter, (f) => {
  const begin = clone(childEvent(f, "b")), end = clone(childEvent(f, "e"));
  childEvent(f, "e").ts = 1_048_000; begin.ts = 1_049_000; end.ts = 1_050_000; f.events.push(begin, end);
});
rejectMutation("wrong current submit S", "AMBIGUOUS_OR_MISSING_CONTAINED_SUBMIT_COMPOSITOR_FRAME", (f) => {
  f.events.find((e) => e.args?.chrome_graphics_pipeline?.step === "STEP_SUBMIT_COMPOSITOR_FRAME").args.chrome_graphics_pipeline.surface_frame_trace_id = "999";
});
rejectMutation("ambiguous current submission", "AMBIGUOUS_OR_MISSING_CONTAINED_SUBMIT_COMPOSITOR_FRAME", (f) => {
  f.events.push(clone(f.events.find((e) => e.args?.chrome_graphics_pipeline?.step === "STEP_SUBMIT_COMPOSITOR_FRAME")));
});
rejectMutation("current draw sequence mismatch", "CURRENT_DRAW_SEQUENCE_MISMATCH", (f) => {
  f.events.find((e) => e.name === "MainFrame.Draw").args.main_frame_pipeline.last_begin_frame_id_during_first_draw.sequence_number += 1;
});
rejectMutation("main frame aborted", "MAIN_FRAME_ABORTED", (f) => {
  f.events.push(event("MainFrameAborted", 1_030_000, "I", compositorTid,
    { main_frame_pipeline: { main_frame_id: sendEvent(f).args.main_frame_pipeline.main_frame_id } }));
});

// Prospective non-pointer entries reuse the actual exact-origin compositor tail.
const nonPointerFixture = (kind) => {
  const f = clone(positive), a = f.markerEvidence.active;
  a.feedbackKind = kind; a.stopped = true;
  Object.assign(a.actionMarker, { feedbackKind: kind, eventKind: kind === "assignment" ? "assignment" : "input",
    documentTimeOrigin: 10000, isTrusted: true, query: "frozen-query" });
  for (const m of a.feedbackMarkers) Object.assign(m, { feedbackKind: kind, domEpoch: 7,
    observed: { modelGeneration: 1, modelIdentityHash: "fixture-hash", indexGeneration: "index-1", projectSessionGeneration: 1 } });
  a.contentProof = { status: "PASS_EXACT_STOPPED_CONTENT", token: a.actionMarker.token,
    modelGeneration: 1, modelIdentityHash: "fixture-hash", indexGeneration: "index-1", projectSessionGeneration: 1,
    domEpoch: 7, documentTimeOrigin: 10000, pendingMutationCount: 0, interveningActionCount: 0,
    assignmentStartedAt: a.actionMarker.listenerObservedAt, assignmentCommitted: true, initialInputNull: true,
    responsiveTreeWitnessRequired: true, query: "frozen-query", inputEventTimeStamp: a.actionMarker.browserEventTimeStamp,
    inputAt: a.actionMarker.listenerObservedAt + 0.1, actionSequence: 2, priorActionSequence: 1 };
  return f;
};
const assignment = nonPointerFixture("assignment");
assignment.markerEvidence.active.actionMarker.listenerObservedAt += 0.5;
assignment.markerEvidence.active.contentProof.assignmentStartedAt = assignment.markerEvidence.active.actionMarker.listenerObservedAt;
assignment.markerEvidence.active.contentProof.laterDiagnosticReadAt = 180;
const assignmentResult = extractCausalPresentations(assignment.events, REQUIRED_CHROMIUM_BINDING, assignment.markerEvidence).results[0];
if (assignmentResult.actionToPresentationIntervalMs.upper >= 100 || assignmentResult.presentationTraceTimestamp !== 1_100_000) {
  throw new Error("assignment did not retain original product start and original presentation");
}
cases.push({ name: "assignment original product start survives later stopped publication", status: "PASS" });

const dom = nonPointerFixture("tree-filter");
for (const e of dom.events.filter((e) => e.name === "TimeStamp")) e.args.data.frame = "document-A";
const domProxy = dom.events.find((e) => e.name === "ProxyMain::BeginMainFrame");
domProxy.ts = 1_020_050; domProxy.dur = 2_950;
dom.events.find((e) => e.name === "BeginMainThreadFrame").ts = 1_020_075;
for (const [name, ts, dur, args] of [
  ["LocalFrameView::RunStyleAndLayoutLifecyclePhases", 1_020_100, 100, {}],
  ["PrePaint", 1_020_250, 100, { data: { frame: "document-A" } }],
  ["LocalFrameView::RunPaintLifecyclePhase", 1_020_400, 200, {}],
  ["Layerize", 1_020_450, 50, { data: { frame: "document-A" } }]
]) dom.events.push(event(name, ts, "X", mainTid, args, { dur }));
for (const m of dom.markerEvidence.active.feedbackMarkers) {
  delete m.mainContextColorClearObserved; delete m.canvasEpoch; delete m.contextEpoch;
}
const domResult = extractCausalPresentations(dom.events, REQUIRED_CHROMIUM_BINDING, dom.markerEvidence).results[0];
if (domResult.presentationTraceTimestamp !== 1_100_000 || domResult.domDocumentFrame !== "document-A") throw new Error("DOM lifecycle entry failed");
cases.push({ name: "DOM ready outside BMF uses native document lifecycle without WebGL callback", status: "PASS" });
const rejectNonPointer = (name, template, code, mutate) => {
  const f = clone(template); mutate(f);
  expectError(code, () => extractCausalPresentations(f.events, REQUIRED_CHROMIUM_BINDING, f.markerEvidence));
  cases.push({ name, status: "PASS_EXPECTED_REJECTION" });
};
rejectNonPointer("assignment replacement original start rejected", assignment, "INVALID_ORIGINAL_ASSIGNMENT_START", (f) => { f.markerEvidence.active.contentProof.assignmentStartedAt += 1; });
rejectNonPointer("assignment stale model identity rejected", assignment, "INVALID_STOPPED_CONTENT_PROOF", (f) => { f.markerEvidence.active.contentProof.modelIdentityHash = "other"; });
rejectNonPointer("assignment unresolved pending mutations rejected", assignment, "INVALID_STOPPED_CONTENT_PROOF", (f) => { f.markerEvidence.active.contentProof.pendingMutationCount = 1; });
rejectNonPointer("DOM wrong native document frame rejected", dom, "DOM_DOCUMENT_FRAME_MISMATCH", (f) => { f.events.find((e) => e.name === "TimeStamp").args.data.frame = "other"; });
rejectNonPointer("DOM missing frame Layerize rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.events = f.events.filter((e) => e.name !== "Layerize"); });
rejectNonPointer("DOM content after style pass rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.events.find((e) => e.name === "LocalFrameView::RunStyleAndLayoutLifecyclePhases").ts = 1_019_000; });
rejectNonPointer("DOM ABA epoch cannot requalify older content", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.markerEvidence.active.contentProof.domEpoch = 9; });
rejectNonPointer("DOM stopped publication wrong native input rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.markerEvidence.active.contentProof.inputEventTimeStamp += 1; });
rejectNonPointer("DOM untrusted event rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.markerEvidence.active.actionMarker.isTrusted = false; });
rejectNonPointer("DOM incomplete main frame tail rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.events = f.events.filter((e) => e.name !== "ProxyImpl::Commit"); });
rejectNonPointer("DOM duplicate lifecycle pass rejected", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => { f.events.push(clone(f.events.find((e) => e.name === "PrePaint"))); });

rejectNonPointer("DOM earlier eligible lifecycle with missing origin cannot use later valid frame", dom, "NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", (f) => {
  const proxy = clone(f.events.find((e) => e.name === "ProxyMain::BeginMainFrame"));
  proxy.ts = 1_020_001; proxy.dur = 40; f.events.push(proxy);
  for (const [name, ts, dur, args] of [
    ["LocalFrameView::RunStyleAndLayoutLifecyclePhases", 1_020_002, 5, {}],
    ["PrePaint", 1_020_008, 5, { data: { frame: "document-A" } }],
    ["LocalFrameView::RunPaintLifecyclePhase", 1_020_014, 10, {}],
    ["Layerize", 1_020_016, 2, { data: { frame: "document-A" } }],
    ["LayerTreeHost::DoUpdateLayers", 1_020_025, 1, { source_frame_number: 999 }]
  ]) f.events.push(event(name, ts, "X", mainTid, args, { dur }));
});

// Equal reported starts do not establish distinct ordered lifecycle occurrences.
for (const order of ["paired-duplicates", "reversed-ties", "interleaved-ties"]) {
  const f=clone(dom),style=clone(f.events.find(e=>e.name==="LocalFrameView::RunStyleAndLayoutLifecyclePhases")),prepaint=clone(f.events.find(e=>e.name==="PrePaint"));
  style.ts=prepaint.ts=1_020_100;style.dur=prepaint.dur=0;
  f.events=f.events.filter(e=>![style.name,prepaint.name].includes(e.name));
  const stages=order==="paired-duplicates"?[style,prepaint,clone(style),clone(prepaint)]
    :order==="reversed-ties"?[prepaint,style,clone(prepaint),clone(style)]:[style,clone(style),prepaint,clone(prepaint)];
  f.events.push(...stages);
  expectError("NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION",()=>currentExtract(f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence));
  cases.push({name:`ambiguous tied DOM ${order}`,status:"PASS_EXPECTED_REJECTION"});
}

// Self-contained source-backed DOM shapes; no archived run paths or learned endpoints.
const multipass = clone(dom);
const firstStyle = multipass.events.find(e => e.name === "LocalFrameView::RunStyleAndLayoutLifecyclePhases");
const firstPrepaint = multipass.events.find(e => e.name === "PrePaint");
firstStyle.ts = 1_020_100; firstStyle.dur = 40; firstPrepaint.ts = 1_020_150; firstPrepaint.dur = 40;
multipass.events.push(event(firstStyle.name, 1_020_220, "X", mainTid, {}, { dur: 40 }));
multipass.events.push(event("PrePaint", 1_020_270, "X", mainTid, { data: { frame: "document-A" } }, { dur: 40 }));
const instantDom = clone(dom);
const instantLayer = instantDom.events.find(e => e.name === "Layerize");
instantLayer.ph = "I"; instantLayer.s = "t"; delete instantLayer.dur;
for (const [name, fixture] of [["ordered two-pass DOM", multipass], ["enclosed thread instant Layerize", instantDom]]) {
  const result = currentExtract(fixture.events, REQUIRED_CHROMIUM_BINDING, fixture.markerEvidence).results[0];
  assert.equal(result.presentationTraceTimestamp, 1_100_000);
  assert.equal(result.domLifecycleEvidence.passes.length, name === "ordered two-pass DOM" ? 2 : 1);
  cases.push({ name, status: "PASS" });
}
for (const [name, template, mutate] of [
  ["duplicate terminal PrePaint", multipass, f => f.events.push(clone(f.events.filter(e=>e.name==="PrePaint").at(-1)))],
  ["overlapping passes", multipass, f => { f.events.filter(e=>e.name==="PrePaint")[0].dur=100; }],
  ["reversed pass", multipass, f => { f.events.filter(e=>e.name==="PrePaint")[1].ts=1_020_210; }],
  ["missing terminal PrePaint", multipass, f => { f.events.splice(f.events.findLastIndex(e=>e.name==="PrePaint"),1); }],
  ["terminal style before final marker", multipass, f => { for(const e of f.events.filter(e=>e.name==="LocalFrameView::RunStyleAndLayoutLifecyclePhases"))e.ts=1_019_000; }],
  ["mixed instant and complete Layerize", instantDom, f => { const e=clone(instantLayer);e.ph="X";e.dur=0;f.events.push(e); }],
  ["instant wrong frame", instantDom, f => { f.events.find(e=>e.name==="Layerize").args.data.frame="other"; }],
  ["instant wrong PID", instantDom, f => { f.events.find(e=>e.name==="Layerize").pid++; }],
  ["instant wrong TID", instantDom, f => { f.events.find(e=>e.name==="Layerize").tid++; }],
  ["instant wrong category", instantDom, f => { f.events.find(e=>e.name==="Layerize").cat="other"; }],
  ["instant wrong scope", instantDom, f => { f.events.find(e=>e.name==="Layerize").s="g"; }],
  ["instant duration invented", instantDom, f => { f.events.find(e=>e.name==="Layerize").dur=0; }],
  ["instant outside complete paint", instantDom, f => { f.events.find(e=>e.name==="Layerize").ts=1_020_700; }],
  ["absent paint duration", instantDom, f => { delete f.events.find(e=>e.name==="LocalFrameView::RunPaintLifecyclePhase").dur; }],
  ["malformed paint duration", instantDom, f => { f.events.find(e=>e.name==="LocalFrameView::RunPaintLifecyclePhase").dur="200"; }],
  ["missing complete paint parent", instantDom, f => { f.events=f.events.filter(e=>e.name!=="LocalFrameView::RunPaintLifecyclePhase"); }],
  ["negative paint duration", instantDom, f => { f.events.find(e=>e.name==="LocalFrameView::RunPaintLifecyclePhase").dur=-1; }],
  ["missing instant scope", instantDom, f => { delete f.events.find(e=>e.name==="Layerize").s; }],
  ["complete Layerize wrong category", dom, f => { f.events.find(e=>e.name==="Layerize").cat="other"; }],
  ["absent proxy duration", instantDom, f => { delete f.events.find(e=>e.name==="ProxyMain::BeginMainFrame").dur; }],
]) {
  const f=clone(template);mutate(f);
  expectError("NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION",()=>currentExtract(f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence));
  cases.push({name,status:"PASS_EXPECTED_REJECTION"});
}

for (const [name, mutate] of [
  ["missing", f => { f.events=f.events.filter(e=>e.name!=="SendBeginMainFrame"); }],
  ["partial", f => { f.events.find(e=>e.name==="PipelineReporter"&&e.ph==="b").args.frame_reporter.state="STATE_PRESENTED_PARTIAL"; }],
  ["aborted", f => { const proxy=f.events.find(e=>e.name==="ProxyMain::BeginMainFrame");f.events.push(event("MainFrameAborted",1_020_800,"I",mainTid,{main_frame_pipeline:{main_frame_id:proxy.args.main_frame_pipeline.main_frame_id}})); }],
  ["ambiguous", f => { f.events.push(clone(f.events.find(e=>e.name==="ProxyImpl::Commit"))); }],
]) {
  const f=clone(instantDom);mutate(f);
  const later=buildLineage(99,1_200_000).events.filter(e=>e.name!=="TimeStamp");
  for(const name of ["LocalFrameView::RunStyleAndLayoutLifecyclePhases","PrePaint","LocalFrameView::RunPaintLifecyclePhase","Layerize"]){
    const stage=clone(dom.events.find(e=>e.name===name));stage.ts+=100_000;later.push(stage);
  }
  // Independently prove the later candidate is valid, then ensure it cannot rescue first failure.
  const onlyLater=clone(dom);onlyLater.events=[...dom.events.filter(e=>e.name==="TimeStamp"),...later];
  assert.equal(currentExtract(onlyLater.events,REQUIRED_CHROMIUM_BINDING,onlyLater.markerEvidence).results[0].presentationTraceTimestamp,1_200_000);
  f.events.push(...later);
  expectError("NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION",()=>currentExtract(f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence));
  cases.push({name:`first ${name} DOM tail cannot use proven later valid frame`,status:"PASS_EXPECTED_REJECTION"});
}

// Each existing extraction/fault above also compares both public APIs when --reference is supplied.
for (const [name, mutate] of [
  ["irrelevant padding", f => { for (let i=0;i<10000;i++) f.events.push(event("Irrelevant",1_000_000+i,"I",mainTid,{ malformed: { main_frame_id: 1.5 } })); }],
  ["equal-time duplicate", f => f.events.push(clone(f.events.find(e => e.name === "BeginMainThreadFrame")))],
  ["typed process collision", f => { const e=clone(f.events.find(e=>e.name==="BeginMainThreadFrame"));e.pid=String(e.pid);f.events.push(e); }],
  ["malformed matching pipeline", f => { f.events.find(e=>e.name==="ProxyMain::BeginMainFrame").args.main_frame_pipeline.main_frame_id=1.5; }],
  ["nonmatching malformed pipeline", f => f.events.push(event("MainFrameAborted",1_020_000,"I",mainTid,{main_frame_pipeline:{main_frame_id:1.5}},{pid:"other"}))],
  ["global invalid irrelevant timestamp", f => f.events.push(event("Irrelevant",NaN,"I",mainTid))]
]) {
  const f=clone(positive);mutate(f);
  if (reference) {
    assert.deepEqual(outcome(currentExtract,[f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence]),outcome(reference.extractCausalPresentations,[f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence]));
    assert.deepEqual(outcome(currentInventory,[f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence]),outcome(reference.inventoryCausalPresentationLineages,[f.events,REQUIRED_CHROMIUM_BINDING,f.markerEvidence]));
    differentialCalls++;
  }
  cases.push({name:`event index differential: ${name}`,status:reference?"PASS_EXACT_BOTH_APIS":"REFERENCE_NOT_REQUESTED"});
}

const output = {
  schema: "openpipestress.ui-foundation.causal-presentation-extractor-synthetic-validation/v1",
  status: "PASS_ALL_SYNTHETIC_EXTRACTION_CASES",
  caseCount: cases.length,
  differential: { referencePath, referenceSha256: reference ? referenceSha256 : null, calls: differentialCalls, APIsPerCall: 2 },
  sourceBinding: REQUIRED_CHROMIUM_BINDING,
  cases
};
const outputIndex = process.argv.indexOf("--output");
if (outputIndex >= 0) {
  const outputPath = process.argv[outputIndex + 1];
  if (!outputPath) throw new Error("--output requires a path");
  await writeFile(outputPath, `${JSON.stringify(output, null, 2)}\n`, { flag: "wx" });
} else {
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}
