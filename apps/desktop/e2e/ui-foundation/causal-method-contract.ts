import { PAGE_CLOCK_SOURCE } from "./causal-presentation-extractor.mjs";
export type Point = Readonly<{ x: number; y: number }>;
export type Rect = Readonly<{ x: number; y: number; width: number; height: number }>;

const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);

export function canvasLocalToClient(local: Point, rect: Rect): Point {
  if (![local.x, local.y, rect.x, rect.y, rect.width, rect.height].every(finite) || rect.width <= 0 || rect.height <= 0 ||
      local.x < 0 || local.y < 0 || local.x > rect.width || local.y > rect.height) {
    throw new Error("canvas-local point or current canvas rectangle is invalid");
  }
  return Object.freeze({ x: rect.x + local.x, y: rect.y + local.y });
}

export const ORBIT_START_NORMALIZED = Object.freeze({
  x: 0.5,
  y: 0.5 + 0.23 * Math.sin(Math.PI / 3)
});

export function normalizedCanvasPoint(rect: Rect, normalized: Point): Point {
  if (![normalized.x, normalized.y].every(finite) || normalized.x < 0 || normalized.x > 1 || normalized.y < 0 || normalized.y > 1) {
    throw new Error("normalized canvas point is outside the closed unit rectangle");
  }
  return canvasLocalToClient({ x: rect.width * normalized.x, y: rect.height * normalized.y }, rect);
}

export type HitTargetEvidence = Readonly<{
  status: "PASS_ACTUAL_CONNECTED_MAIN_CANVAS_TARGET" | "FAIL_MAIN_CANVAS_TARGET";
  clientPoint: Point;
  canvasEpoch: number | null;
  armedCanvasEpoch?: number | null;
  exactArmedCanvas?: boolean | null;
  targetTag: string | null;
  targetTestId: string | null;
  canvasConnected: boolean;
  exactCanvasTarget: boolean;
}>;

export function assertMainCanvasHitTarget(evidence: HitTargetEvidence): void {
  const armedCanvasMismatch = evidence.armedCanvasEpoch !== undefined && evidence.armedCanvasEpoch !== null &&
    (evidence.exactArmedCanvas !== true || evidence.canvasEpoch !== evidence.armedCanvasEpoch);
  if (evidence.status !== "PASS_ACTUAL_CONNECTED_MAIN_CANVAS_TARGET" || !evidence.exactCanvasTarget ||
      !evidence.canvasConnected || evidence.targetTag !== "CANVAS" || !Number.isInteger(evidence.canvasEpoch) ||
      (evidence.canvasEpoch ?? 0) < 1 || armedCanvasMismatch) {
    throw new Error(`prescribed pointer target is not the current main canvas: ${JSON.stringify(evidence)}`);
  }
}

export function assertStoppedCausalEvidence(evidence: any, feedbackKind: "point-selection" | "box-selection" | "orbit" | "assignment" | "tree-filter", requirePointerUp: boolean): void {
  const active = evidence?.active;
  const down = active?.pointerTransaction?.down;
  const up = active?.pointerTransaction?.up;
  const action = active?.actionMarker;
  const feedback = active?.feedbackMarkers;
  const completePointer = down?.targetTag === "CANVAS" && action?.testId === "viewport-canvas" &&
    action?.pointerId === down?.pointerId && (!requirePointerUp || (up?.targetTag === "CANVAS" && up?.pointerId === down?.pointerId));
  const nonPointer = feedbackKind === "assignment" || feedbackKind === "tree-filter";
  if (!active || active.feedbackKind !== feedbackKind || !action || (requirePointerUp && active.stopped !== true) ||
      (nonPointer ? active.contentProof?.status !== "PASS_EXACT_STOPPED_CONTENT" : !completePointer) || !Array.isArray(feedback) || feedback.length < 1 ||
      active.overflow?.total !== 0 || active.observerErrors?.length !== 0 ||
      active.rejections?.length !== 0) {
    throw new Error(`causal ${feedbackKind} evidence is incomplete: ${JSON.stringify({
      active: Boolean(active), action: Boolean(action), completePointer, feedbackCount: feedback?.length ?? null,
      overflow: active?.overflow ?? null, observerErrors: active?.observerErrors ?? null, rejections: active?.rejections ?? null
    })}`);
  }
}

export type ObserverValidity = Readonly<{
  schema: "openpipestress.ui-foundation.instrumented-observer-accounting/v2";
  status: "PASS_OBSERVER_STRUCTURAL_EVIDENCE" | "FAIL_OBSERVER_STRUCTURAL_EVIDENCE";
  structuralFailures: readonly string[];
  rawOverheadDiagnostics: Readonly<{
    status: "WITHIN_RAW_REFERENCES_TRUE_COST_UNPROVED" | "EXCEEDS_RAW_REFERENCES" | "UNAVAILABLE_INVALID_EVIDENCE";
    references: Readonly<{ p95Ms: 0.5; maximumMs: 2; totalPercent: 3 }>;
    trueCostStatus: "UNPROVED_INCLUDING_UNBOUNDED_RECORDING_TAIL";
  }>;
  sourceSampleCount: number;
  sampleCount: number;
  invalidSampleCount: number;
  invalidSamples: readonly Readonly<{ index: number; operation: string | null; reason: string }>[];
  boundaryCrossingSampleCount: number;
  totalCostMs: number;
  p95Ms: number | null;
  maximumMs: number | null;
  observedWindowMs: number | null;
  totalPercent: number | null;
  percentilePopulation: "RAF_CALLBACK_ONLY";
  byOperation: Readonly<Record<string, Readonly<{
    sampleCount: number;
    totalCostMs: number;
    p95Ms: number;
    maximumMs: number;
    withinRawReferences: boolean;
  }>>>;
  overflowTotal: number;
  observerErrorCount: number;
  accountingDefinition: string;
  clockMeasurement: Readonly<{
    quantumBoundMs: number;
    oneDifferenceUncertaintyBoundMs: number;
    pageEdgeMembershipUncertaintyMs: number;
    selfRecordingTail: string;
    interpretation: string;
  }>;
}>;

export type ObserverSample = Readonly<{
  operation: string;
  startedAt: number;
  completedAt: number;
  durationMs: number;
  clockDifferenceCount?: number;
}>;

export type ObserverWindow = Readonly<{ start: number; end: number }>;

const percentile95 = (values: readonly number[]): number => values[Math.max(0, Math.ceil(values.length * 0.95) - 1)];

type ObservedPointerBoundary = Readonly<{
  eventKind: string; pointerId: number; listenerObservedAt: number;
}>;

export type ObserverRequirements = Readonly<{
  actionClass?: "pointer";
  feedbackObservations: readonly Readonly<{
    markerIdentity: string; invocationId: number; callbackEntryAt: number; callbackCompletedAt: number;
  }>[];
  pointerTransaction: Readonly<{ down: ObservedPointerBoundary; up: ObservedPointerBoundary }>;
  actionMarker: ObservedPointerBoundary;
  actionStartEvent: string;
  minimumRafCallbacks: number;
  requiredSourceOperations: readonly string[];
  scheduledAdditionalAnimationFrames: number;
  productObservationRaf: number | null;
  restorationStatus: string;
}>;

export type NonPointerObserverRequirements = Omit<ObserverRequirements,
  "actionClass" | "pointerTransaction" | "actionMarker" | "actionStartEvent"> & Readonly<{
  actionClass: "assignment" | "tree-filter";
  pointerTransaction?: never; actionMarker?: never; actionStartEvent?: never;
  expectedOperations: readonly Readonly<{ identity: string; operation: string; start: number; end: number }>[];
}>;

const observerOperations = new Set(["raf-callback", "raf-registration", "raf-cancellation",
  "pointerdown-capture-listener-total", "pointerup-capture-listener-total", "input-capture-listener-total",
  "dom-mutation-callback", "dom-stopped-reconciliation", "assignment-calibration"]);

export function observerValidity(samples: readonly ObserverSample[], window: ObserverWindow,
  overflowTotal: number, observerErrorCount: number, requirements: ObserverRequirements | NonPointerObserverRequirements): ObserverValidity {
  const invalidSamples: { index: number; operation: string | null; reason: string }[] = [];
  const validSamples: ObserverSample[] = [];
  const sourceSamples = Array.isArray(samples) ? Array.from(samples) : [];
  sourceSamples.forEach((sample, index) => {
    const operation = typeof sample?.operation === "string" && sample.operation.length > 0 ? sample.operation : null;
    let reason: string | null = null;
    if (operation === null || !observerOperations.has(operation)) reason = "INVALID_OPERATION";
    else if (![sample.startedAt, sample.completedAt, sample.durationMs].every(finite)) reason = "NONFINITE_CLOCK_OR_DURATION";
    else if (sample.startedAt < 0 || sample.completedAt < sample.startedAt || sample.durationMs < 0) {
      reason = "NEGATIVE_OR_REVERSED_CLOCK_OR_DURATION";
    }
    if (reason !== null) invalidSamples.push({ index, operation, reason });
    else validSamples.push(sample);
  });
  const validWindow = finite(window?.start) && finite(window?.end) && window.start >= 0 && window.end > window.start;
  const observedWindowMs = validWindow ? window.end - window.start : null;
  const relevant = validWindow
    ? validSamples.filter((sample) => sample.completedAt + 2 * PAGE_CLOCK_SOURCE.quantumBoundMs >= window.start &&
      sample.startedAt - 2 * PAGE_CLOCK_SOURCE.quantumBoundMs <= window.end)
    : [];
  const boundaryCrossingSampleCount = validWindow
    ? relevant.filter((sample) => sample.startedAt < window.start || sample.completedAt > window.end).length
    : 0;
  const grouped = new Map<string, number[]>();
  for (const sample of relevant) {
    const values = grouped.get(sample.operation) ?? [];
    values.push(sample.durationMs);
    grouped.set(sample.operation, values);
  }
  const byOperation: Record<string, { sampleCount: number; totalCostMs: number; p95Ms: number; maximumMs: number;
    withinRawReferences: boolean }> = {};
  for (const operation of [...grouped.keys()].sort()) {
    const values = grouped.get(operation)!.sort((a, b) => a - b);
    const p95Ms = percentile95(values);
    const maximumMs = values.at(-1)!;
    byOperation[operation] = {
      sampleCount: values.length,
      totalCostMs: values.reduce((sum, value) => sum + value, 0),
      p95Ms,
      maximumMs,
      withinRawReferences: p95Ms <= 0.5 && maximumMs <= 2
    };
  }
  const totalCostMs = Object.values(byOperation).reduce((sum, group) => sum + group.totalCostMs, 0);
  const raf = byOperation["raf-callback"] ?? null;
  const p95Ms = raf?.p95Ms ?? null;
  const maximumMs = raf?.maximumMs ?? null;
  const totalPercent = validWindow && observedWindowMs !== null ? totalCostMs / observedWindowMs * 100 : null;
  const structuralFailures: string[] = [];
  if (!Array.isArray(samples) || sourceSamples.length === 0) structuralFailures.push("MISSING_OBSERVER_SAMPLES");
  if (!validWindow || !finite(observedWindowMs)) structuralFailures.push("INVALID_OBSERVER_WINDOW");
  if (invalidSamples.length) structuralFailures.push("INVALID_OBSERVER_SAMPLES");
  if (overflowTotal !== 0) structuralFailures.push("OBSERVER_OVERFLOW_OR_MISSING_COUNT");
  if (observerErrorCount !== 0) structuralFailures.push("OBSERVER_ERRORS_OR_MISSING_COUNT");
  if (!requirements || !Number.isSafeInteger(requirements.minimumRafCallbacks) || requirements.minimumRafCallbacks < (requirements.actionClass === "tree-filter" ? 0 : 1) ||
      !Array.isArray(requirements.requiredSourceOperations) ||
      requirements.requiredSourceOperations.some((operation) => !observerOperations.has(operation))) {
    structuralFailures.push("INVALID_EXPECTED_OBSERVER_POPULATIONS");
  } else {
    if (requirements.minimumRafCallbacks > 0 && (!raf || raf.sampleCount < requirements.minimumRafCallbacks)) structuralFailures.push("MISSING_EXPECTED_RAF_POPULATION");
    for (const operation of requirements.requiredSourceOperations) {
      if (!validSamples.some((sample) => sample.operation === operation)) structuralFailures.push(`MISSING_OPERATION:${operation}`);
    }
  }
  if (requirements?.actionClass === "assignment" || requirements?.actionClass === "tree-filter") {
    const observations = requirements.feedbackObservations;
    const expected = requirements.expectedOperations;
    const ids = new Set<string>(), invocations = new Set<number>(), used = new Set<number>();
    const validObservation = Array.isArray(observations) && observations.length > 0 && observations.every((o) => {
      if (!o || typeof o.markerIdentity !== "string" || !o.markerIdentity || !Number.isSafeInteger(o.invocationId) || o.invocationId < 1 ||
          !finite(o.callbackEntryAt) || !finite(o.callbackCompletedAt) || o.callbackEntryAt < 0 || o.callbackCompletedAt < o.callbackEntryAt ||
          ids.has(o.markerIdentity) || invocations.has(o.invocationId)) return false;
      ids.add(o.markerIdentity); invocations.add(o.invocationId); return true;
    });
    const operationIds = new Set<string>();
    if (!validObservation || !Array.isArray(expected) || !expected.length || expected.some((o) => {
      if (!o || typeof o.identity !== "string" || !o.identity || operationIds.has(o.identity) || !observerOperations.has(o.operation) ||
          !finite(o.start) || !finite(o.end) || o.start < 0 || o.end < o.start) return true;
      operationIds.add(o.identity); return false;
    })) structuralFailures.push("INVALID_EXPECTED_OBSERVATIONS");
    else {
      const rows = [...observations.map((o) => ({ identity: o.markerIdentity,
        operation: requirements.actionClass === "assignment" ? "raf-callback" : "dom-mutation-callback",
        start: o.callbackEntryAt, end: o.callbackCompletedAt })), ...expected];
      for (const o of rows) {
        const matches = validSamples.map((sample, index) => ({ sample, index })).filter(({ sample }) =>
          sample.operation === o.operation && sample.startedAt <= o.start && sample.completedAt >= o.end);
        if (matches.length !== 1) structuralFailures.push(`MISSING_OR_AMBIGUOUS_OBSERVATION_COST:${o.operation}`);
        else if (used.has(matches[0].index)) structuralFailures.push("REUSED_CALLBACK_COST_RECORD");
        else used.add(matches[0].index);
      }
      const required = requirements.actionClass === "assignment" ? "assignment-calibration" : "input-capture-listener-total";
      if (expected.filter((o) => o.operation === required).length !== 1) structuralFailures.push("MISSING_ACTION_CLASS_OBSERVATION");
    }
  } else {
  // Cost samples have no invocation identity. Bind only exact same-page-clock
  // enclosure; ambiguity fails, and distinct callbacks cannot share a record.
  const observations = requirements?.feedbackObservations;
  const down = requirements?.pointerTransaction?.down;
  const up = requirements?.pointerTransaction?.up;
  const action = requirements?.actionMarker;
  const validBoundary = (value: ObservedPointerBoundary | undefined, kind: string) =>
    value?.eventKind === kind && Number.isSafeInteger(value.pointerId) &&
    finite(value.listenerObservedAt) && value.listenerObservedAt >= 0;
  const actionKind = requirements?.actionStartEvent;
  const pointerValid = validBoundary(down, "pointerdown") && validBoundary(up, "pointerup") &&
    (actionKind === "pointerdown" || actionKind === "pointerup") && validBoundary(action, actionKind) &&
    down!.pointerId === up!.pointerId && action!.pointerId === down!.pointerId &&
    down!.listenerObservedAt <= up!.listenerObservedAt &&
    action!.listenerObservedAt >= (actionKind === "pointerdown" ? down! : up!).listenerObservedAt &&
    (actionKind !== "pointerdown" || action!.listenerObservedAt <= up!.listenerObservedAt);
  const identities = new Set<string>();
  const invocations = new Set<number>();
  const feedbackValid = Array.isArray(observations) && observations.length > 0 && observations.every((observation) => {
    if (typeof observation?.markerIdentity !== "string" || observation.markerIdentity.length === 0 ||
        !Number.isSafeInteger(observation.invocationId) || observation.invocationId < 1 ||
        !finite(observation.callbackEntryAt) || !finite(observation.callbackCompletedAt) ||
        observation.callbackEntryAt < 0 || observation.callbackCompletedAt < observation.callbackEntryAt ||
        !action || observation.callbackEntryAt < action.listenerObservedAt ||
        identities.has(observation.markerIdentity) || invocations.has(observation.invocationId)) return false;
    identities.add(observation.markerIdentity); invocations.add(observation.invocationId);
    return true;
  });
  if (!pointerValid || !feedbackValid) structuralFailures.push("INVALID_EXPECTED_OBSERVATIONS");
  else {
    const enclosing = (operation: string, start: number, end: number): number | null => {
      const matches = validSamples.map((sample, index) => ({ sample, index })).filter(({ sample }) =>
        sample.operation === operation && sample.startedAt <= start && sample.completedAt >= end);
      if (matches.length !== 1) {
        structuralFailures.push(`MISSING_OR_AMBIGUOUS_OBSERVATION_COST:${operation}`);
        return null;
      }
      return matches[0].index;
    };
    const used = new Set<number>();
    for (const observation of observations!) {
      const index = enclosing("raf-callback", observation.callbackEntryAt, observation.callbackCompletedAt);
      if (index !== null) {
        if (used.has(index)) structuralFailures.push("REUSED_CALLBACK_COST_RECORD");
        used.add(index);
      }
    }
    const downIndex = enclosing("pointerdown-capture-listener-total", down!.listenerObservedAt, down!.listenerObservedAt);
    const upIndex = enclosing("pointerup-capture-listener-total", up!.listenerObservedAt, up!.listenerObservedAt);
    const actionIndex = enclosing(`${actionKind}-capture-listener-total`, action!.listenerObservedAt, action!.listenerObservedAt);
    // Transaction and action observations are nested in the same outer listener.
    if (actionIndex !== null && actionIndex !== (actionKind === "pointerdown" ? downIndex : upIndex)) {
      structuralFailures.push("ACTION_POINTER_COST_RECORD_MISMATCH");
    }
  }
  }
  if (requirements?.scheduledAdditionalAnimationFrames !== 0) structuralFailures.push("EXTRA_OR_UNKNOWN_OBSERVER_RAF");
  if (requirements?.productObservationRaf !== null) structuralFailures.push("EXTRA_OR_UNKNOWN_PRODUCT_OBSERVATION_RAF");
  if (requirements?.restorationStatus !== "PASS_FULL_RESTORE") structuralFailures.push("INCOMPLETE_INSTRUMENTATION_RESTORE");
  if (!finite(totalCostMs) || !finite(totalPercent) ||
      Object.values(byOperation).some((group) => ![group.totalCostMs, group.p95Ms, group.maximumMs].every(finite))) {
    structuralFailures.push("NONFINITE_DERIVED_ACCOUNTING");
  }
  const structurallyValid = structuralFailures.length === 0;
  const withinRawReferences = Object.values(byOperation).every((group) => group.withinRawReferences) &&
    totalPercent !== null && totalPercent <= 3;
  return Object.freeze({ schema: "openpipestress.ui-foundation.instrumented-observer-accounting/v2",
    status: structurallyValid ? "PASS_OBSERVER_STRUCTURAL_EVIDENCE" : "FAIL_OBSERVER_STRUCTURAL_EVIDENCE",
    structuralFailures: Object.freeze(structuralFailures),
    rawOverheadDiagnostics: Object.freeze({
      status: !structurallyValid ? "UNAVAILABLE_INVALID_EVIDENCE" : withinRawReferences
        ? "WITHIN_RAW_REFERENCES_TRUE_COST_UNPROVED" : "EXCEEDS_RAW_REFERENCES",
      references: Object.freeze({ p95Ms: 0.5, maximumMs: 2, totalPercent: 3 }),
      trueCostStatus: "UNPROVED_INCLUDING_UNBOUNDED_RECORDING_TAIL" }),
    sourceSampleCount: sourceSamples.length, sampleCount: relevant.length, invalidSampleCount: invalidSamples.length,
    invalidSamples: Object.freeze(invalidSamples), boundaryCrossingSampleCount, totalCostMs, p95Ms, maximumMs,
    observedWindowMs, totalPercent, percentilePopulation: "RAF_CALLBACK_ONLY", byOperation: Object.freeze(byOperation),
    overflowTotal, observerErrorCount,
    clockMeasurement: Object.freeze({ quantumBoundMs: PAGE_CLOCK_SOURCE.quantumBoundMs,
      oneDifferenceUncertaintyBoundMs: 2 * PAGE_CLOCK_SOURCE.quantumBoundMs,
      pageEdgeMembershipUncertaintyMs: 2 * PAGE_CLOCK_SOURCE.quantumBoundMs,
      selfRecordingTail: "Final endpoint/duration assignments, clear-cost accumulation and returns occur after their final clock reading; this unavoidable tail is unmeasured, never asserted zero.",
      interpretation: "Raw readings and fixed diagnostic references are unchanged. A slice difference has up to 0.2 ms uncertainty; RAF has two wrapper differences plus two per clear. Zero is not proof of no work. Structural PASS concerns evidence integrity only. Within-reference raw readings do not prove true-cost compliance; omitted clock-call/bookkeeping tails have no finite bound established by this evidence. Only exact instrumented-workload timings may be considered against unchanged owner targets. No tolerance or subtraction is applied." }),
    accountingDefinition: "Every possibly intersecting operation (including source-bound page-edge uncertainty) is charged at its full recorded observer duration. RAF callback p95/max use only complete raf-callback observer costs; each non-RAF operation class is reported independently. The total is the disjoint sum of wrapper callback, RAF registration/cancellation, and outer capture-listener totals; nested action/pointer diagnostic slices and original WebGL clear work are excluded from this observer sum and retained separately." });
}


// This is the actual preflight conjunction, not a product-target scorer.
export function instrumentedPreflightPass(evidence: Readonly<{
  traceComplete: boolean;
  extractionPass: boolean;
  pointObserver: ObserverValidity;
  orbitObserver: ObserverValidity;
  pointVisualIdentityStatus: string;
}>): boolean {
  return evidence.traceComplete === true && evidence.extractionPass === true &&
    evidence.pointObserver.status === "PASS_OBSERVER_STRUCTURAL_EVIDENCE" &&
    evidence.orbitObserver.status === "PASS_OBSERVER_STRUCTURAL_EVIDENCE" &&
    evidence.pointVisualIdentityStatus === "PASS_EXACT_IDENTITY_AND_SOURCE_BOUND_SELECTION_COLOR_IN_PROJECTED_ROI";
}
