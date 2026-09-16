const MARKER_PREFIX = "UIF_CAUSAL_V1:";
const REQUIRED_CHROMIUM_BINDING = Object.freeze({
  browserVersion: "153.0.8010.36",
  revision: "@507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c",
  primarySourceManifestSha256: "f87fd65324e795fbaea89bb96f72ef00c387f06beb8bff8651eb2a500d37a109",
  diagnosisSha256: "15102056e9924b645aafa344cf808f3e36dcf2e4cfa05375c30be2565cff01fb",
  diagnosisOutputManifestSha256: "dce9bc5a1271367a8e05fb213592ce0fcac2d1fa077147d265e185335176ba68",
  methodBriefSha256: "bc21adf308ba8983265c4d0dd66521d53566c5be168098f07f76d3bb091a0fae"
});

// Pinned TimeClamper selects either grid edge (not nearest rounding). The fixed
// clamped origin is absorbed in the shared offset, not charged per marker.
export const PAGE_CLOCK_SOURCE = Object.freeze({
  revision: REQUIRED_CHROMIUM_BINDING.revision,
  timeClamperHeaderSha256: "f8065edb3ba8da66d58eb7042523fba44b6d515c04d482614670a5414a684944",
  timeClamperImplementationSha256: "2491cb64f50a54ccd10f778c058f0960ad4d9fa0861e39212bc27b8343bd103c",
  performanceImplementationSha256: "ff28d26044d705b1580a504d3b59814d0023d85ce700bc77c40c0d1724dc6234",
  quantumBoundMs: 0.1,
  isolatedResolutionMs: 0.005,
  nonIsolatedResolutionMs: 0.1
});
// Round each arithmetic operation outward, including trace microseconds / 1000.
// No residual fitting or decimal truncation is used.
const adjacentFloat = (value, up) => {
  if (!Number.isFinite(value)) return value;
  if (value === 0) return up ? Number.MIN_VALUE : -Number.MIN_VALUE;
  const bytes = new DataView(new ArrayBuffer(8));
  bytes.setFloat64(0, value);
  const bits = bytes.getBigUint64(0);
  bytes.setBigUint64(0, bits + ((value > 0) === up ? 1n : -1n));
  return bytes.getFloat64(0);
};
const lower = (value) => adjacentFloat(value, false);
const upper = (value) => adjacentFloat(value, true);

export class CausalPresentationExtractionError extends Error {
  constructor(code, message, context = {}) {
    super(`${code}: ${message}`);
    this.name = "CausalPresentationExtractionError";
    this.code = code;
    this.context = context;
  }
}

const fail = (code, message, context = {}) => {
  throw new CausalPresentationExtractionError(code, message, context);
};
const one = (values, code, context = {}) => {
  if (values.length !== 1) fail(code, `expected exactly one event, observed ${values.length}`, context);
  return values[0];
};
const safeTime = (value, label) => {
  if (typeof value !== "number" || !Number.isFinite(value) || Math.abs(value) > Number.MAX_SAFE_INTEGER) {
    fail("UNSAFE_OR_MISSING_TRACE_TIME", `${label} must be a finite safe Number`, { value });
  }
  return value;
};
const exactDecimalId = (value, label) => {
  if (typeof value !== "string" || !/^-?(?:0|[1-9]\d*)$/.test(value)) {
    fail("NONEXACT_64_BIT_IDENTIFIER", `${label} must be a losslessly parsed decimal string`, { value });
  }
  return value;
};
// BeginFrame source/sequence are uint64: safe numeric JSON tokens and lossless
// decimal strings normalize to one exact identity; rounded Numbers never qualify.
const exactUnsignedId = (value, label) => {
  const text = Number.isSafeInteger(value) && value >= 0 ? String(value) : value;
  if (typeof text !== "string" || !/^(?:0|[1-9]\d*)$/.test(text) || BigInt(text) > 18446744073709551615n) {
    fail("NONEXACT_BEGIN_FRAME_IDENTIFIER", `${label} must be an exact uint64`, { value });
  }
  return text;
};
const beginFrameIdentity = (value, label) => ({
  sourceId: exactUnsignedId(value?.source_id, `${label}.source_id`),
  sequenceNumber: exactUnsignedId(value?.sequence_number, `${label}.sequence_number`)
});
const sameOccurrenceNamespace = (left, right) => left.pid === right.pid && left.tid === right.tid &&
  left.id2?.local === right.id2?.local && (left.cat ?? "") === (right.cat ?? "") &&
  (left.scope ?? "") === (right.scope ?? "");
const safeInteger = (value, label) => {
  if (!Number.isSafeInteger(value)) fail("UNSAFE_OR_MISSING_SMALL_IDENTIFIER", `${label} must be a safe integer`, { value });
  return value;
};
const intervalContains = (outer, inner) => {
  const start = safeTime(outer.ts, `${outer.name}.ts`);
  const end = start + safeTime(outer.dur, `${outer.name}.dur`);
  const innerStart = safeTime(inner.ts, `${inner.name}.ts`);
  const innerEnd = innerStart + (typeof inner.dur === "number" ? safeTime(inner.dur, `${inner.name}.dur`) : 0);
  return innerStart >= start && innerEnd <= end;
};
const markerMessage = (event) => event?.args?.data?.message ?? event?.args?.message ?? event?.args?.name ?? null;
const decodeMarkerIdentity = (message) => {
  const match = typeof message === "string" && message.match(/^UIF_CAUSAL_V1:(ACTION|FEEDBACK):([A-Za-z0-9._:-]{1,160})$/);
  return match ? { kind: match[1], identity: match[2] } : null;
};
const mainFrame = (event) => event?.args?.main_frame_pipeline;
const graphics = (event) => event?.args?.chrome_graphics_pipeline;
const eventLocalTrackKey = (event) => {
  const local = event.id2?.local;
  if (typeof local !== "string" || local.length === 0) {
    fail("ASYNC_LOCAL_ID_UNAVAILABLE", `${event.name} requires exact id2.local`, { event });
  }
  return JSON.stringify([event.pid, event.tid, "id2.local", local, event.cat ?? "", event.scope ?? "", event.name]);
};

function validatePointerTransaction(active, actionEvidence) {
  const transaction = active?.pointerTransaction;
  const down = transaction?.down;
  const up = transaction?.up;
  if (!down || !up || !Number.isSafeInteger(down.pointerId) || down.pointerId !== up.pointerId ||
      down.testId !== active.expectedActionTargetTestId || up.testId !== active.expectedActionTargetTestId ||
      down.targetTag !== "CANVAS" || up.targetTag !== "CANVAS" ||
      typeof down.listenerObservedAt !== "number" || typeof up.listenerObservedAt !== "number" ||
      up.listenerObservedAt < down.listenerObservedAt ||
      typeof down.browserEventTimeStamp !== "number" || typeof up.browserEventTimeStamp !== "number" ||
      up.browserEventTimeStamp < down.browserEventTimeStamp) {
    fail("INVALID_OR_INCOMPLETE_POINTER_TRANSACTION", "causal evidence requires one ordered real-canvas pointer down/up transaction", {
      transaction, expectedActionTargetTestId: active?.expectedActionTargetTestId
    });
  }
  const actionBoundary = actionEvidence.eventKind === "pointerdown" ? down
    : actionEvidence.eventKind === "pointerup" ? up : null;
  if (!actionBoundary || actionEvidence.pointerId !== actionBoundary.pointerId ||
      actionEvidence.clientX !== actionBoundary.clientX || actionEvidence.clientY !== actionBoundary.clientY ||
      actionEvidence.testId !== actionBoundary.testId ||
      actionEvidence.browserEventTimeStamp !== actionBoundary.browserEventTimeStamp) {
    fail("ACTION_MARKER_POINTER_BOUNDARY_MISMATCH", "the trace action marker is not the declared real pointer boundary", {
      actionEvidence, actionBoundary
    });
  }
  if (JSON.stringify(actionEvidence.actionIdentity) !== JSON.stringify(active.actionIdentity) ||
      JSON.stringify(down.actionIdentity) !== JSON.stringify(active.actionIdentity) ||
      JSON.stringify(up.actionIdentity) !== JSON.stringify(active.actionIdentity)) {
    fail("ACTION_IDENTITY_MISMATCH", "pointer boundaries and marker do not retain one exact frozen action identity", {
      activeIdentity: active.actionIdentity,
      actionIdentity: actionEvidence.actionIdentity,
      downIdentity: down.actionIdentity,
      upIdentity: up.actionIdentity
    });
  }
  if (active.feedbackKind === "point-selection") {
    const expected = active.actionIdentity?.expectedCssPoint;
    const tolerance = 0.01;
    if (!expected || ![expected.x, expected.y, down.clientX, down.clientY, up.clientX, up.clientY].every(Number.isFinite) ||
        Math.abs(down.clientX - expected.x) > tolerance || Math.abs(down.clientY - expected.y) > tolerance ||
        Math.abs(up.clientX - expected.x) > tolerance || Math.abs(up.clientY - expected.y) > tolerance) {
      fail("POINT_POINTER_COORDINATE_MISMATCH", "point transaction differs from its frozen CSS probe", {
        expected, down: { x: down.clientX, y: down.clientY }, up: { x: up.clientX, y: up.clientY }, tolerance
      });
    }
  }
  if (active.feedbackKind === "box-selection") {
    const start = active.actionIdentity?.start, end = active.actionIdentity?.end;
    if (actionEvidence.eventKind !== "pointerup" || !start || !end ||
        ![start.x, start.y, end.x, end.y, down.clientX, down.clientY, up.clientX, up.clientY].every(Number.isFinite) ||
        Math.abs(down.clientX - start.x) > 0.01 || Math.abs(down.clientY - start.y) > 0.01 ||
        Math.abs(up.clientX - end.x) > 0.01 || Math.abs(up.clientY - end.y) > 0.01) {
      fail("BOX_POINTER_COORDINATE_MISMATCH", "box commit is not the exact frozen real-pointer drag");
    }
  }
  return { down, up, actionBoundary: actionEvidence.eventKind };
}

function pairNestableAsync(events, name) {
  const open = new Map();
  const pairs = [];
  const unmatchedEnds = [];
  for (const event of events.filter((candidate) => candidate.name === name && (candidate.ph === "b" || candidate.ph === "e"))) {
    const key = eventLocalTrackKey(event);
    if (event.ph === "b") {
      if (open.has(key)) fail("AMBIGUOUS_OVERLAPPING_ASYNC_PAIR", `${name} reused a local track before its prior end`, { key });
      open.set(key, event);
    } else {
      const begin = open.get(key);
      if (!begin) {
        unmatchedEnds.push(event);
        continue;
      }
      open.delete(key);
      pairs.push({ key, begin, end: event });
    }
  }
  return { pairs, unmatchedBegins: [...open.values()], unmatchedEnds };
}

function validateSourceBinding(sourceBinding) {
  for (const [key, expected] of Object.entries(REQUIRED_CHROMIUM_BINDING)) {
    if (sourceBinding?.[key] !== expected) {
      fail("CHROMIUM_SOURCE_BINDING_MISMATCH", `${key} does not match the sealed Chrome 153 source`, {
        expected, actual: sourceBinding?.[key] ?? null
      });
    }
  }
}

// Per-call name buckets retain each indexed object, duplicate and globally sorted order.
function indexEventNames(events) {
  const byName = new Map();
  for (const event of events) {
    let bucket = byName.get(event.name);
    if (!bucket) { bucket = []; byName.set(event.name, bucket); }
    bucket.push(event);
  }
  return byName;
}

function stepEvent(byName, rendererPid, mainFrameId, name, step) {
  return one((byName.get(name) ?? []).filter((event) => event.pid === rendererPid && event.name === name &&
    mainFrame(event)?.step === step && mainFrame(event)?.main_frame_id === mainFrameId),
  "AMBIGUOUS_OR_MISSING_MAIN_FRAME_PIPELINE_STEP", { rendererPid, name, step, mainFrameId });
}

function clockOffsetInterval(markerEvent, clock) {
  const traceMs = safeTime(markerEvent.ts, `${markerEvent.name}.ts`) / 1000;
  if (typeof clock?.before !== "number" || typeof clock?.after !== "number" ||
      !Number.isFinite(clock.before) || !Number.isFinite(clock.after) || clock.after < clock.before) {
    fail("INVALID_PAGE_TRACE_CLOCK_BOUNDS", "marker page-clock bounds are invalid", { clock });
  }
  for (const [key, expected] of Object.entries(PAGE_CLOCK_SOURCE)) {
    if (clock.source?.[key] !== expected) {
      fail("PAGE_CLOCK_SOURCE_BINDING_MISMATCH", "page clock does not bind the pinned source", { key, expected, actual: clock.source?.[key] });
    }
  }
  if (typeof clock.crossOriginIsolated !== "boolean") {
    fail("INVALID_PAGE_CLOCK_ISOLATION_EVIDENCE", "prospective isolation mode must be observed as a boolean");
  }
  const q = PAGE_CLOCK_SOURCE.quantumBoundMs;
  return { minimum: lower(lower(lower(traceMs) - clock.after) - q),
    maximum: upper(upper(upper(traceMs) - clock.before) + q) };
}

function exactTraceMarker(traceMarkers, kind, identity) {
  return one(traceMarkers.filter((marker) => marker.decoded.kind === kind && marker.decoded.identity === identity),
    "AMBIGUOUS_OR_MISSING_TRACE_MARKER", { kind, identity });
}

function domLifecycleProxies(events, feedbackMarker) {
  const marker = feedbackMarker.event;
  const frame = marker.args?.data?.frame;
  if (typeof frame !== "string" || !frame) fail("DOM_DOCUMENT_FRAME_UNAVAILABLE", "DOM marker requires native Blink document frame identity");
  const complete = (event) => event.ph === "X" && typeof event.dur === "number" && Number.isFinite(event.dur) && event.dur >= 0;
  const recognized = [];
  for (const proxy of events.filter((e) => e.name === "ProxyMain::BeginMainFrame" && complete(e) &&
    e.pid === marker.pid && e.tid === marker.tid && e.ts + e.dur >= marker.ts)) {
    // Keep malformed named stages in the population: they must not disappear merely
    // because another complete stage could supply a convenient terminal chain.
    const inside = events.filter((e) => e.pid === proxy.pid && e.tid === proxy.tid &&
      e.ts >= proxy.ts && e.ts <= proxy.ts + proxy.dur);
    const styles = inside.filter((e) => e.name === "LocalFrameView::RunStyleAndLayoutLifecyclePhases");
    const prepaints = inside.filter((e) => e.name === "PrePaint" && e.args?.data?.frame === frame);
    const paints = inside.filter((e) => e.name === "LocalFrameView::RunPaintLifecyclePhase");
    const layerizes = inside.filter((e) => e.name === "Layerize" && e.args?.data?.frame === frame);
    const updates = inside.filter((e) => e.name === "LayerTreeHost::DoUpdateLayers");
    if (!styles.length || styles.length !== prepaints.length || paints.length !== 1 || layerizes.length !== 1 || updates.length !== 1) continue;
    if (![...styles, ...prepaints, ...paints, ...updates].every((e) => complete(e) && intervalContains(proxy, e))) continue;
    // Each pass is accounted for in trace order. Duplicate/overlapping/reversed
    // passes cannot be discarded to select a last-looking timestamp.
    // Trace array order cannot resolve equal reported starts. Require strict
    // cross-kind start order as well as nonoverlapping completed intervals;
    // adjoining positive-duration spans remain allowed, tied zero spans do not.
    if (!styles.every((style, i) => style.ts < prepaints[i].ts && style.ts + style.dur <= prepaints[i].ts &&
      (i === 0 || (prepaints[i - 1].ts < style.ts && prepaints[i - 1].ts + prepaints[i - 1].dur <= style.ts)))) continue;
    const terminal = styles.length - 1, paint = paints[0], layerize = layerizes[0], update = updates[0];
    if (styles[terminal].ts < marker.ts || prepaints[terminal].ts + prepaints[terminal].dur > paint.ts ||
      paint.ts + paint.dur > update.ts) continue;
    const instant = layerize.ph === "I" && layerize.s === "t" && !Object.hasOwn(layerize, "dur") &&
      layerize.cat === "devtools.timeline";
    if (layerize.cat !== "devtools.timeline" || !(complete(layerize) || instant) || layerize.ts < paint.ts ||
      (instant ? layerize.ts : layerize.ts + layerize.dur) > paint.ts + paint.dur) continue;
    recognized.push({ proxy, lifecycle: styles.length > 1 || instant ? {
      passes: styles.map((style, i) => ({ styleEventIndex: style.__index, prePaintEventIndex: prepaints[i].__index,
        terminal: i === terminal })), paintEventIndex: paint.__index, updateEventIndex: update.__index,
      layerizeEventIndex: layerize.__index, layerizeSemantics: instant ? "thread-instant-checkpoint-enclosed-by-complete-paint" : "complete-X-span"
    } : null });
  }
  return recognized.sort((a, b) => a.proxy.ts - b.proxy.ts || a.proxy.__index - b.proxy.__index);
}

function extractByActionClass(events, markers, action, feedback, occurrences, evidence, byName) {
  if (action?.feedbackKind !== "tree-filter") return extractOne(events, markers, action, feedback, occurrences, evidence, null, byName);
  const a = exactTraceMarker(markers, "ACTION", action.token);
  const f = exactTraceMarker(markers, "FEEDBACK", feedback.markerIdentity);
  if (a.event.args?.data?.frame !== f.event.args?.data?.frame) fail("DOM_DOCUMENT_FRAME_MISMATCH", "input and content refer to different documents");
  const candidates = domLifecycleProxies(events, f);
  const examined = [];
  // The first eligible lifecycle owns the endpoint. A failed exact tail must
  // never be rescued by a later unrelated presentation.
  for (const { proxy, lifecycle } of candidates.slice(0, 1)) {
    try {
      const result = extractOne(events, markers, action, feedback, occurrences, evidence, proxy, byName);
      return { ...result, ...(lifecycle ? { domLifecycleEvidence: lifecycle } : {}), domDocumentFrame: f.event.args.data.frame,
        domLifecycleProxyEventIndex: proxy.__index, examinedDomMainFrames: [...examined, { eventIndex: proxy.__index, status: "PASS" }] };
    } catch (error) {
      if (!(error instanceof CausalPresentationExtractionError)) throw error;
      examined.push({ eventIndex: proxy.__index, status: "FAIL", code: error.code });
    }
  }
  fail("NO_COMPLETE_DOM_LIFECYCLE_PRESENTATION", "no natural unchanged-content lifecycle reached an exact presented reporter", { examined });
}

function extractOne(events, traceMarkers, actionEvidence, feedbackEvidence, reporterOccurrences, markerEvidence, proxyOverride = null, byName) {
  if (actionEvidence?.schema !== "openpipestress.ui-foundation.causal-feedback-marker/v1" ||
      feedbackEvidence?.schema !== actionEvidence.schema || actionEvidence.kind !== "ACTION" || feedbackEvidence.kind !== "FEEDBACK") {
    fail("INVALID_IN_PAGE_MARKER_EVIDENCE", "action/feedback evidence has the wrong schema or kind");
  }
  if (actionEvidence.token !== feedbackEvidence.token || actionEvidence.phase !== feedbackEvidence.phase ||
      actionEvidence.feedbackKind !== feedbackEvidence.feedbackKind) {
    fail("ACTION_FEEDBACK_MARKER_BINDING_MISMATCH", "action and feedback evidence differ", { actionEvidence, feedbackEvidence });
  }
  const active = markerEvidence?.active;
  const nonPointer = actionEvidence.feedbackKind === "assignment" || actionEvidence.feedbackKind === "tree-filter";
  const pointerTransaction = nonPointer ? null : validatePointerTransaction(active ?? null, actionEvidence);
  if (nonPointer) {
    const proof = active?.contentProof;
    if (proof?.status !== "PASS_EXACT_STOPPED_CONTENT" || proof.token !== actionEvidence.token ||
        proof.modelGeneration !== feedbackEvidence.observed?.modelGeneration ||
        proof.domEpoch !== feedbackEvidence.domEpoch || proof.documentTimeOrigin !== actionEvidence.documentTimeOrigin || !Number.isFinite(proof.documentTimeOrigin) ||
        typeof proof.indexGeneration !== "string" || !Number.isSafeInteger(proof.projectSessionGeneration) ||
        proof.pendingMutationCount !== 0 || proof.interveningActionCount !== 0 ||
        typeof proof.modelIdentityHash !== "string" || proof.modelIdentityHash !== feedbackEvidence.observed?.modelIdentityHash ||
        proof.indexGeneration !== feedbackEvidence.observed?.indexGeneration ||
        proof.projectSessionGeneration !== feedbackEvidence.observed?.projectSessionGeneration ||
        active?.stopped !== true) fail("INVALID_STOPPED_CONTENT_PROOF", "non-pointer endpoint lacks unchanged exact stopped content");
    if (actionEvidence.feedbackKind === "assignment") {
      if (actionEvidence.eventKind !== "assignment" || actionEvidence.listenerObservedAt !== proof.assignmentStartedAt ||
          proof.assignmentCommitted !== true || proof.initialInputNull !== true || proof.responsiveTreeWitnessRequired !== true) {
        fail("INVALID_ORIGINAL_ASSIGNMENT_START", "assignment must retain the original product start and initial ready tree");
      }
    } else if (actionEvidence.eventKind !== "input" || actionEvidence.isTrusted !== true ||
        actionEvidence.query !== proof.query || actionEvidence.browserEventTimeStamp !== proof.inputEventTimeStamp ||
        proof.inputAt < actionEvidence.listenerObservedAt || proof.actionSequence <= proof.priorActionSequence) {
      fail("INVALID_FILTER_INPUT_PUBLICATION", "filter input and stopped publication do not bind one action");
    }
  }
  if (actionEvidence.feedbackKind !== "tree-filter" && (feedbackEvidence.mainContextColorClearObserved !== true ||
      !Number.isSafeInteger(feedbackEvidence.canvasEpoch) || !Number.isSafeInteger(feedbackEvidence.contextEpoch))) {
    fail("MAIN_CANVAS_ACTIVITY_SENTINEL_UNAVAILABLE", "feedback lacks a healthy exact main-canvas/context clear sentinel", { feedbackEvidence });
  }
  const actionMarker = exactTraceMarker(traceMarkers, "ACTION", actionEvidence.token);
  const feedbackMarker = exactTraceMarker(traceMarkers, "FEEDBACK", feedbackEvidence.markerIdentity);
  if (actionMarker.event.pid !== feedbackMarker.event.pid || actionMarker.event.tid !== feedbackMarker.event.tid) {
    fail("ACTION_FEEDBACK_RENDERER_THREAD_MISMATCH", "action and feedback markers are not on one renderer main thread");
  }
  const actionTime = safeTime(actionMarker.event.ts, "action marker ts");
  const markerTime = safeTime(feedbackMarker.event.ts, "feedback marker ts");
  if (markerTime < actionTime) fail("FEEDBACK_PRECEDES_ACTION", "feedback marker precedes action marker");

  const proxy = proxyOverride ?? one((byName.get("ProxyMain::BeginMainFrame") ?? []).filter((event) => event.name === "ProxyMain::BeginMainFrame" && event.ph === "X" &&
    event.pid === feedbackMarker.event.pid && event.tid === feedbackMarker.event.tid && intervalContains(event, feedbackMarker.event)),
  "AMBIGUOUS_OR_MISSING_CONTAINING_BEGIN_MAIN_FRAME", { markerIdentity: feedbackEvidence.markerIdentity });
  const mainFrameId = exactDecimalId(mainFrame(proxy)?.main_frame_id, "ProxyMain::BeginMainFrame.main_frame_id");
  if (mainFrame(proxy)?.step !== "BEGIN_MAIN_FRAME") fail("WRONG_BEGIN_MAIN_FRAME_STEP", "containing ProxyMain event has unexpected step");

  const beginMainThreadFrame = one((byName.get("BeginMainThreadFrame") ?? []).filter((event) => event.name === "BeginMainThreadFrame" &&
    event.pid === proxy.pid && event.tid === proxy.tid && intervalContains(proxy, event)),
  "AMBIGUOUS_OR_MISSING_BEGIN_MAIN_THREAD_FRAME", { mainFrameId });
  const sourceFrameNumber = safeInteger(beginMainThreadFrame.args?.data?.frameId, "BeginMainThreadFrame.frameId");
  const layerTreeId = safeInteger(beginMainThreadFrame.args?.layerTreeId, "BeginMainThreadFrame.layerTreeId");
  const updateLayers = one((byName.get("LayerTreeHost::DoUpdateLayers") ?? []).filter((event) => event.name === "LayerTreeHost::DoUpdateLayers" &&
    event.pid === proxy.pid && event.tid === proxy.tid && intervalContains(proxy, event) &&
    event.args?.source_frame_number === sourceFrameNumber && safeTime(event.ts, "DoUpdateLayers.ts") > markerTime),
  "AMBIGUOUS_OR_MISSING_POST_MARKER_UPDATE_LAYERS", { sourceFrameNumber, mainFrameId });

  const commitMain = stepEvent(byName, proxy.pid, mainFrameId, "ProxyMain::BeginMainFrame::commit", "COMMIT_ON_MAIN");
  const commitImpl = stepEvent(byName, proxy.pid, mainFrameId, "ProxyImpl::Commit", "COMMIT_ON_IMPL");
  const activate = stepEvent(byName, proxy.pid, mainFrameId, "LayerTreeHostImpl::ActivateSyncTree", "ACTIVATE");
  if (commitMain.tid !== proxy.tid || commitImpl.tid !== activate.tid) {
    fail("MAIN_FRAME_PIPELINE_THREAD_MISMATCH", "main commit must remain on the marker thread and impl commit/activation on one compositor thread", {
      markerThread: proxy.tid, commitMainThread: commitMain.tid, commitImplThread: commitImpl.tid, activateThread: activate.tid
    });
  }
  const ordered = [updateLayers, commitMain, commitImpl, activate];
  for (let index = 1; index < ordered.length; index += 1) {
    if (safeTime(ordered[index].ts, `${ordered[index].name}.ts`) < safeTime(ordered[index - 1].ts, `${ordered[index - 1].name}.ts`)) {
      fail("MAIN_FRAME_PIPELINE_ORDER_VIOLATION", "main-frame pipeline steps are not ordered", { mainFrameId });
    }
  }
  const aborts = (byName.get("MainFrameAborted") ?? []).filter((event) => event.pid === proxy.pid && event.name === "MainFrameAborted" &&
    mainFrame(event)?.main_frame_id === mainFrameId);
  if (aborts.length) fail("MAIN_FRAME_ABORTED", "the feedback-containing main frame was aborted", { mainFrameId, aborts });
  one((byName.get("ActivateLayerTree") ?? []).filter((event) => event.name === "ActivateLayerTree" && event.pid === proxy.pid &&
    event.args?.frameId === sourceFrameNumber && event.args?.layerTreeId === layerTreeId && intervalContains(activate, event)),
  "AMBIGUOUS_OR_MISSING_ACTIVATE_LAYER_TREE", { sourceFrameNumber, layerTreeId, mainFrameId });

  const draw = stepEvent(byName, proxy.pid, mainFrameId, "MainFrame.Draw", "DRAW");
  if (safeTime(draw.ts, "MainFrame.Draw.ts") < safeTime(activate.ts, "ActivateSyncTree.ts")) {
    fail("DRAW_PRECEDES_ACTIVATION", "source main-frame draw precedes activation", { mainFrameId });
  }
  const generate = one((byName.get("Graphics.Pipeline") ?? []).filter((event) => event.name === "Graphics.Pipeline" && event.pid === proxy.pid &&
    graphics(event)?.step === "STEP_GENERATE_COMPOSITOR_FRAME" && intervalContains(draw, event)),
  "AMBIGUOUS_OR_MISSING_GENERATE_COMPOSITOR_FRAME", { sourceFrameNumber, mainFrameId });
  if (draw.tid !== commitImpl.tid || generate.tid !== commitImpl.tid) {
    fail("DRAW_PIPELINE_THREAD_MISMATCH", "draw and generated frame must use the exact impl commit/activation compositor thread", {
      commitImplThread: commitImpl.tid, drawThread: draw.tid, generateThread: generate.tid
    });
  }
  one((byName.get("LayerTreeHostImpl::PrepareToDraw") ?? []).filter((event) => event.name === "LayerTreeHostImpl::PrepareToDraw" && event.pid === proxy.pid &&
    event.tid === generate.tid && event.args?.SourceFrameNumber === sourceFrameNumber && intervalContains(generate, event)),
  "AMBIGUOUS_OR_MISSING_PREPARE_TO_DRAW", { sourceFrameNumber, mainFrameId });
  const surfaceFrameTraceId = exactDecimalId(graphics(generate)?.surface_frame_trace_id,
    "STEP_GENERATE_COMPOSITOR_FRAME.surface_frame_trace_id");
  const submit = one((byName.get("Graphics.Pipeline") ?? []).filter((event) => event.name === "Graphics.Pipeline" && event.pid === proxy.pid &&
    event.tid === generate.tid && graphics(event)?.step === "STEP_SUBMIT_COMPOSITOR_FRAME" &&
    graphics(event)?.surface_frame_trace_id === surfaceFrameTraceId && intervalContains(generate, event)),
  "AMBIGUOUS_OR_MISSING_CONTAINED_SUBMIT_COMPOSITOR_FRAME", { surfaceFrameTraceId, mainFrameId });
  const drawFrame = one((byName.get("DrawFrame") ?? []).filter((event) => event.name === "DrawFrame" && event.pid === proxy.pid &&
    event.tid === generate.tid && event.args?.layerTreeId === layerTreeId && intervalContains(generate, event)),
  "AMBIGUOUS_OR_MISSING_DRAW_FRAME_HOST_CROSSCHECK", { surfaceFrameTraceId, layerTreeId });
  const frameSequence = safeInteger(drawFrame.args?.frameSeqId, "DrawFrame.frameSeqId");

  // The Send's original BeginFrame identifies the main update. A delayed main
  // update may be submitted under a different current compositor BeginFrame/S.
  const send = stepEvent(byName, proxy.pid, mainFrameId, "SendBeginMainFrame", "SEND_BEGIN_MAIN_FRAME");
  if (send.tid !== generate.tid || safeTime(send.ts, "SendBeginMainFrame.ts") > safeTime(proxy.ts, "ProxyMain.ts")) {
    fail("MISBOUND_OR_LATE_SEND_BEGIN_MAIN_FRAME", "origin Send must use the proven compositor thread and precede main execution", { mainFrameId });
  }
  const originBeginFrameId = beginFrameIdentity(mainFrame(send)?.begin_frame_id, "SendBeginMainFrame.begin_frame_id");
  if (proxy.args?.begin_frame_id !== undefined &&
      exactUnsignedId(proxy.args.begin_frame_id, "ProxyMain.begin_frame_id") !== originBeginFrameId.sequenceNumber) {
    fail("PROXY_ORIGIN_SEQUENCE_MISMATCH", "proxy sequence differs from the exact originating Send", { mainFrameId });
  }
  const currentBeginFrameId = beginFrameIdentity(mainFrame(draw)?.last_begin_frame_id_during_first_draw,
    "MainFrame.Draw.last_begin_frame_id_during_first_draw");
  if (currentBeginFrameId.sequenceNumber !== exactUnsignedId(frameSequence, "DrawFrame.frameSeqId")) {
    fail("CURRENT_DRAW_SEQUENCE_MISMATCH", "current draw BeginFrame and DrawFrame sequence differ", { mainFrameId });
  }
  const matches = reporterOccurrences.pairs.flatMap((reporter) => {
    const { begin, end } = reporter;
    const frame = begin.args?.frame_reporter;
    if (begin.pid !== proxy.pid || begin.tid !== generate.tid || frame?.state !== "STATE_PRESENTED_ALL" ||
        frame?.frame_type !== undefined || frame?.has_missing_content !== false ||
        frame?.checkerboarded_needs_raster !== false || frame?.checkerboarded_needs_record !== false ||
        frame?.layer_tree_host_id !== layerTreeId ||
        safeTime(end.ts, "PipelineReporter.end.ts") < safeTime(submit.ts, "submit.ts")) return [];
    const reporterOrigin = {
      sourceId: exactUnsignedId(frame.frame_source, "PipelineReporter.frame_source"),
      sequenceNumber: exactUnsignedId(frame.frame_sequence, "PipelineReporter.frame_sequence")
    };
    if (reporterOrigin.sourceId !== originBeginFrameId.sourceId ||
        reporterOrigin.sequenceNumber !== originBeginFrameId.sequenceNumber) return [];
    const originSurfaceFrameTraceId = exactDecimalId(frame.surface_frame_trace_id, "PipelineReporter.origin_surface_frame_trace_id");
    // Pair the child's full namespace and endpoints, then bind that occurrence
    // within this particular outer occurrence (not another reuse of its track).
    const children = reporterOccurrences.submitStages.pairs.filter((child) =>
      sameOccurrenceNamespace(child.begin, begin) &&
      safeTime(child.begin.ts, "submit stage begin") >= safeTime(begin.ts, "reporter begin") &&
      safeTime(child.end.ts, "submit stage end") <= safeTime(end.ts, "reporter end") &&
      safeTime(child.end.ts, "submit stage end") >= safeTime(child.begin.ts, "submit stage begin") &&
      safeTime(child.end.ts, "submit stage end") >= safeTime(submit.ts, "submit.ts") &&
      intervalContains(generate, child.begin));
    // A relevant unpaired start/end cannot be hidden by a different complete child.
    const incomplete = [...reporterOccurrences.submitStages.unmatchedBegins, ...reporterOccurrences.submitStages.unmatchedEnds]
      .some((event) => sameOccurrenceNamespace(event, begin) && event.ts >= begin.ts && event.ts <= end.ts);
    if (children.length !== 1 || incomplete) return [];
    return [{ reporter, submitStage: children[0], originSurfaceFrameTraceId }];
  });
  const selected = one(matches, "AMBIGUOUS_OR_MISSING_PRESENTED_PIPELINE_REPORTER_OCCURRENCE", {
    surfaceFrameTraceId, layerTreeId, frameSequence, mainFrameId, originBeginFrameId
  });
  const { reporter, submitStage, originSurfaceFrameTraceId } = selected;
  const displayTraceId = exactDecimalId(reporter.begin.args.frame_reporter.display_trace_id,
    "PipelineReporter.display_trace_id");
  const presentationTraceTimestamp = safeTime(reporter.end.ts, "PipelineReporter presentation end ts");

  const actionOffset = clockOffsetInterval(actionMarker.event, actionEvidence.traceClock);
  const feedbackOffset = clockOffsetInterval(feedbackMarker.event, feedbackEvidence.traceClock);
  if (actionEvidence.traceClock.crossOriginIsolated !== feedbackEvidence.traceClock.crossOriginIsolated) {
    fail("PAGE_CLOCK_ISOLATION_MISMATCH", "action and feedback must use the same observed isolation mode");
  }
  const offsetMinimum = Math.max(actionOffset.minimum, feedbackOffset.minimum);
  const offsetMaximum = Math.min(actionOffset.maximum, feedbackOffset.maximum);
  if (offsetMinimum > offsetMaximum) {
    fail("INCONSISTENT_PAGE_TRACE_CLOCK_MAPPING", "action and feedback clock intervals do not intersect", {
      actionOffset, feedbackOffset
    });
  }
  const actionPageTime = actionEvidence.listenerObservedAt;
  if (typeof actionPageTime !== "number" || !Number.isFinite(actionPageTime)) {
    fail("INVALID_ACTION_PAGE_CLOCK", "listenerObservedAt must be finite", { actionPageTime });
  }
  const presentationPageLower = lower(lower(presentationTraceTimestamp / 1000) - offsetMaximum);
  const presentationPageUpper = upper(upper(presentationTraceTimestamp / 1000) - offsetMinimum);
  const durationLower = lower(lower(presentationPageLower - actionPageTime) - PAGE_CLOCK_SOURCE.quantumBoundMs);
  const durationUpper = upper(upper(presentationPageUpper - actionPageTime) + PAGE_CLOCK_SOURCE.quantumBoundMs);

  return {
    schema: "openpipestress.ui-foundation.causal-presentation-result/v1",
    status: "PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION",
    token: actionEvidence.token,
    markerIdentity: feedbackEvidence.markerIdentity,
    phase: actionEvidence.phase,
    feedbackKind: actionEvidence.feedbackKind,
    canvasEpoch: feedbackEvidence.canvasEpoch,
    contextEpoch: feedbackEvidence.contextEpoch,
    actionTraceTimestamp: actionTime,
    feedbackMarkerTraceTimestamp: markerTime,
    presentationTraceTimestamp,
    pageToTraceOffsetIntervalMs: { minimum: offsetMinimum, maximum: offsetMaximum },
    pageClockSource: { ...PAGE_CLOCK_SOURCE, crossOriginIsolated: actionEvidence.traceClock.crossOriginIsolated },
    actionToPresentationIntervalMs: {
      lower: durationLower,
      upper: durationUpper,
      scoredConservativeBound: durationUpper
    },
    actionMarkerObservationToPresentationMs: (presentationTraceTimestamp - actionTime) / 1000,
    feedbackMarkerToPresentationMs: (presentationTraceTimestamp - markerTime) / 1000,
    modelGeneration: feedbackEvidence.observed?.modelGeneration,
    modelIdentityHash: feedbackEvidence.observed?.modelIdentityHash,
    rendererProcessId: proxy.pid,
    rendererMainThreadId: proxy.tid,
    rendererCompositorThreadId: generate.tid,
    mainFrameId,
    sourceFrameNumber,
    layerTreeId,
    frameSequence,
    surfaceFrameTraceId,
    displayTraceId,
    pipelineReporterOccurrence: reporter.key,
    originBeginFrameId,
    currentBeginFrameId,
    reporterOriginSurfaceFrameTraceId: originSurfaceFrameTraceId,
    originSendEventIndex: send.__index,
    reporterBeginEventIndex: reporter.begin.__index,
    reporterEndEventIndex: reporter.end.__index,
    submitStageBeginEventIndex: submitStage.begin.__index,
    submitStageEndEventIndex: submitStage.end.__index,
    pointerTransaction,
    chromiumMetricMeaning: "Chromium PipelineReporter STATE_PRESENTED_ALL outer async end for the exact process/main/origin BeginFrame/layer/current generate-submit/complete reporter occurrence; hardware scan-out is not claimed."
  };
}

export function extractCausalPresentations(traceEvents, sourceBinding, markerEvidence) {
  validateSourceBinding(sourceBinding);
  if (!Array.isArray(traceEvents) || traceEvents.length === 0) fail("TRACE_EVENTS_UNAVAILABLE", "traceEvents must be a non-empty array");
  const indexed = traceEvents.map((event, index) => ({ ...event, __index: index })).sort((left, right) =>
    safeTime(left.ts, `${left.name}.ts`) - safeTime(right.ts, `${right.name}.ts`) || left.__index - right.__index);
  const traceMarkers = indexed.map((event) => ({ event, decoded: decodeMarkerIdentity(markerMessage(event)) }))
    .filter(({ decoded }) => decoded);
  const actionEvidence = markerEvidence?.active?.actionMarker;
  const feedbackEvidence = markerEvidence?.active?.feedbackMarkers;
  if (!actionEvidence || !Array.isArray(feedbackEvidence) || feedbackEvidence.length === 0) {
    fail("IN_PAGE_MARKER_EVIDENCE_UNAVAILABLE", "bounded in-page action/feedback evidence is absent");
  }
  const reporterOccurrences = { ...pairNestableAsync(indexed, "PipelineReporter"),
    submitStages: pairNestableAsync(indexed, "SubmitCompositorFrameToPresentationCompositorFrame") };
  const byName = indexEventNames(indexed);
  const results = feedbackEvidence.map((feedback) =>
    extractByActionClass(indexed, traceMarkers, actionEvidence, feedback, reporterOccurrences, markerEvidence, byName));
  const presentationKeys = new Set();
  const uniquePresentations = [];
  for (const result of results) {
    const key = JSON.stringify([
      result.rendererProcessId, result.mainFrameId, result.sourceFrameNumber, result.layerTreeId,
      result.surfaceFrameTraceId, result.pipelineReporterOccurrence, result.presentationTraceTimestamp
    ]);
    if (presentationKeys.has(key)) continue;
    presentationKeys.add(key);
    uniquePresentations.push(result);
  }
  const sortedPresentations = [...uniquePresentations].sort((left, right) =>
    left.presentationTraceTimestamp - right.presentationTraceTimestamp);
  return {
    schema: "openpipestress.ui-foundation.causal-presentation-extraction/v2",
    status: "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS",
    sourceBinding: { ...sourceBinding },
    traceMarkerCount: traceMarkers.length,
    reporterBoundaryDiagnostics: {
      unmatchedBeginCount: reporterOccurrences.unmatchedBegins.length,
      unmatchedEndCount: reporterOccurrences.unmatchedEnds.length,
      note: "Boundary-crossing unrelated reporter occurrences are retained but cannot qualify; each accepted result has its own complete pair."
    },
    lineageCount: results.length,
    uniquePresentationCount: sortedPresentations.length,
    coalescedLineageCount: results.length - sortedPresentations.length,
    results,
    presentations: sortedPresentations,
    presentationGapsMs: sortedPresentations.slice(1).map((result, index) =>
      (result.presentationTraceTimestamp - sortedPresentations[index].presentationTraceTimestamp) / 1000)
  };
}

export function inventoryCausalPresentationLineages(traceEvents, sourceBinding, markerEvidence) {
  try {
    validateSourceBinding(sourceBinding);
    if (!Array.isArray(traceEvents) || traceEvents.length === 0) {
      return { schema: "openpipestress.ui-foundation.causal-lineage-inventory/v1", status: "FAIL_INCOMPLETE_LINEAGE_INVENTORY",
        globalFailure: { code: "TRACE_EVENTS_UNAVAILABLE", message: "traceEvents must be a non-empty array" }, entries: [] };
    }
    const indexed = traceEvents.map((event, index) => ({ ...event, __index: index })).sort((left, right) =>
      safeTime(left.ts, `${left.name}.ts`) - safeTime(right.ts, `${right.name}.ts`) || left.__index - right.__index);
    const traceMarkers = indexed.map((event) => ({ event, decoded: decodeMarkerIdentity(markerMessage(event)) }))
      .filter(({ decoded }) => decoded);
    const actionEvidence = markerEvidence?.active?.actionMarker;
    const feedbackEvidence = markerEvidence?.active?.feedbackMarkers;
    if (!actionEvidence || !Array.isArray(feedbackEvidence) || feedbackEvidence.length === 0) {
      return { schema: "openpipestress.ui-foundation.causal-lineage-inventory/v1", status: "FAIL_INCOMPLETE_LINEAGE_INVENTORY",
        globalFailure: { code: "IN_PAGE_MARKER_EVIDENCE_UNAVAILABLE",
          message: "bounded in-page action/feedback evidence is absent", actionPresent: Boolean(actionEvidence),
          feedbackCount: Array.isArray(feedbackEvidence) ? feedbackEvidence.length : null }, entries: [] };
    }
    const reporterOccurrences = { ...pairNestableAsync(indexed, "PipelineReporter"),
    submitStages: pairNestableAsync(indexed, "SubmitCompositorFrameToPresentationCompositorFrame") };
    const byName = indexEventNames(indexed);
    const entries = feedbackEvidence.map((feedback) => {
      try {
        return { markerIdentity: feedback?.markerIdentity ?? null, status: "PASS_EXACT_LINEAGE",
          result: extractByActionClass(indexed, traceMarkers, actionEvidence, feedback, reporterOccurrences, markerEvidence, byName) };
      } catch (error) {
        return error instanceof CausalPresentationExtractionError
          ? { markerIdentity: feedback?.markerIdentity ?? null, status: "FAIL_LINEAGE", code: error.code,
              message: error.message, context: error.context }
          : { markerIdentity: feedback?.markerIdentity ?? null, status: "FAIL_LINEAGE",
              code: "UNEXPECTED_EXTRACTION_ERROR", message: String(error), context: {} };
      }
    });
    const failureCount = entries.filter((entry) => entry.status !== "PASS_EXACT_LINEAGE").length;
    return { schema: "openpipestress.ui-foundation.causal-lineage-inventory/v1",
      status: failureCount === 0 ? "PASS_ALL_LINEAGES" : "FAIL_INCOMPLETE_LINEAGE_INVENTORY",
      traceMarkerCount: traceMarkers.length, feedbackCount: entries.length, failureCount, entries };
  } catch (error) {
    return error instanceof CausalPresentationExtractionError
      ? { schema: "openpipestress.ui-foundation.causal-lineage-inventory/v1", status: "FAIL_INCOMPLETE_LINEAGE_INVENTORY",
          globalFailure: { code: error.code, message: error.message, context: error.context }, entries: [] }
      : { schema: "openpipestress.ui-foundation.causal-lineage-inventory/v1", status: "FAIL_INCOMPLETE_LINEAGE_INVENTORY",
          globalFailure: { code: "UNEXPECTED_EXTRACTION_ERROR", message: String(error) }, entries: [] };
  }
}

export function presentationGapsForMeasuredWindow(extraction, measuredWindow) {
  if (extraction?.status !== "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS") {
    fail("UNQUALIFIED_CAUSAL_EXTRACTION", "presentation gaps require a complete causal extraction");
  }
  const start = safeTime(measuredWindow?.startTraceTimestamp, "measured window start");
  const end = safeTime(measuredWindow?.endTraceTimestamp, "measured window end");
  if (end <= start) fail("INVALID_MEASURED_WINDOW", "measured window end must follow its start", { start, end });
  const presentations = [...extraction.presentations].sort((left, right) =>
    left.presentationTraceTimestamp - right.presentationTraceTimestamp);
  const preceding = presentations.filter((entry) => entry.presentationTraceTimestamp < start).at(-1) ?? null;
  const within = presentations.filter((entry) => entry.presentationTraceTimestamp >= start && entry.presentationTraceTimestamp <= end);
  const trailing = presentations.find((entry) => entry.presentationTraceTimestamp > end) ?? null;
  if (!preceding || within.length < 2 || !trailing) {
    fail("INCOMPLETE_MEASURED_WINDOW_PRESENTATION_COVERAGE",
      "orbit gaps require one validated preceding presentation, at least two within-window presentations, and trailing coverage", {
        preceding: Boolean(preceding), withinCount: within.length, trailing: Boolean(trailing), start, end
      });
  }
  const series = [preceding, ...within];
  const gaps = series.slice(1).map((entry, index) =>
    (entry.presentationTraceTimestamp - series[index].presentationTraceTimestamp) / 1000);
  return {
    schema: "openpipestress.ui-foundation.causal-presentation-gaps/v1",
    status: "PASS_COMPLETE_MEASURED_WINDOW_CAUSAL_PRESENTATION_COVERAGE",
    measuredWindow: { startTraceTimestamp: start, endTraceTimestamp: end },
    precedingPresentationTraceTimestamp: preceding.presentationTraceTimestamp,
    withinPresentationCount: within.length,
    trailingPresentationTraceTimestamp: trailing.presentationTraceTimestamp,
    presentationGapsMs: gaps,
    trailingCoverageGapMs: (trailing.presentationTraceTimestamp - within.at(-1).presentationTraceTimestamp) / 1000,
    metricMeaning: "Gaps between consecutive exact app-updated Chromium-reported presentations whose right endpoint is inside the frozen measured window; the preceding and trailing validated presentations are retained as coverage boundaries."
  };
}

export { MARKER_PREFIX, REQUIRED_CHROMIUM_BINDING };
