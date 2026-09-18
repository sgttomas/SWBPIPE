import { freshDemoPolicy } from "./fresh-demo-policy.mjs";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { collectionMode, runCollectionDisposition, bindReferenceProfile, CHARACTERIZATION_PRODUCT_REVISION } from "./characterization-mode";
import { benchmarkCommands, captureBoundary, persistBoundary, prepareFilterInput, insertFilterQuery, FILTER_STIMULUS, captureDisplayProfile } from "./characterization-commands";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile, realpath, readdir } from "node:fs/promises";
import path from "node:path";
import { performance as nodePerformance } from "node:perf_hooks";
import { setTimeout as delay } from "node:timers/promises";
import type { Page } from "@playwright/test";
import { armCausalFeedbackMarker, captureChromiumEnvironment, ensureViewportToggle, expectedVisibleEntityLabel,
  installInstrumentation, expectedBoxInspectorHeading, validatedPriorBoxBaseline, persistBoxPostcondition, readBoxVisibleIdentity, boxCallbackMatchesSnapshot, pairedWinnerCueWitness, captureWinnerCue, cueActionBindingsStable, pointCaptureMatchesMarker, validateWinnerCuePlan, projectCandidateAuthoredPoint, readCandidateDiagnostics,
  requireUniqueConnectedMainCanvas, restoreCausalFeedbackInstrumentation, routeModelFixture, settledRafState,
  stopCausalFeedbackMarker, treeRowTestId, validateCandidateMeasuredCameraBinding, validateCandidateOracleBinding,
  validateMainCanvasHitTarget, waitForCandidateExclusiveSelection, type LoadedFixture } from "./benchmark-harness";
import { assertMainCanvasHitTarget, assertStoppedCausalEvidence, canvasLocalToClient, normalizedCanvasPoint,
  observerValidity, ORBIT_START_NORMALIZED } from "./causal-method-contract";
import { CHROMIUM_TRACE_RAW_BYTE_LIMIT, beginChromiumCompositorTrace, endChromiumCompositorTrace, type ChromiumTraceCapture } from "./chromium-compositor-trace";
import { extractCausalPresentations, REQUIRED_CHROMIUM_BINDING, PAGE_CLOCK_SOURCE } from "./causal-presentation-extractor.mjs";
import { assertBoundCandidateDocumentResponse, bindCandidateDriverEntry, bindRequiredChromiumExecutable, REQUIRED_CHROMIUM_RUNTIME_BINDING, type CandidateDriverBinding } from "./candidate-server-response";
import { scorePerformanceRun, SAME_TRACE_EXPORT_PROFILE, validSameTraceBasis, type SameTraceDurationBasis, type ActionDuration, type OrbitEvidence, type PerformanceRun, type RunExpectation } from "./performance-targets";

export const FULL_COHORT_RECIPE = Object.freeze({ version: "causal-full-v20", pointCount: 200, boxCount: 20, filterCount: 20,
  discreteActionsPerTrace: 1, settleMs: 500, warmupMs: 2000, measuredMs: 10000, trailingMs: 250,
  maximumFeedbackMarkers: 4096, activeObserverSampleCap: 16000, globalObserverSampleCap: 24000,
  domMutationCap: 4096, rawTraceByteCap: CHROMIUM_TRACE_RAW_BYTE_LIMIT,
  traceStart: "before real reset/setup; assignment before navigation", drain: "only after immutable stopped evidence, exact SHA acknowledgement and successful derivation",
  memoryBasis: "V48 failed at the former64MiB post-window transfer cap after6/8 segments; partial trace had1501feedback callbacks and no reported Chromium loss. V49 estimated92–97MB total, not proved. Fixed256MiB allocation retains30s IO deadline; GiB-scale parsing transients, heap/deadline/headroom remain unproved pending live qualification. Overflow fails; no adaptive segmentation." });
const nextFloat = (value: number, up: boolean): number => {
  if (!Number.isFinite(value)) return value;
  if (value === 0) return up ? Number.MIN_VALUE : -Number.MIN_VALUE;
  const data = new DataView(new ArrayBuffer(8)); data.setFloat64(0, value);
  data.setBigUint64(0, data.getBigUint64(0) + ((value > 0) === up ? 1n : -1n));
  return data.getFloat64(0);
};
export function conservativePresentedGap(previous: { lower: number; upper: number }, next: { lower: number; upper: number }) {
  if (![previous.lower, previous.upper, next.lower, next.upper].every(Number.isFinite) ||
      previous.lower > previous.upper || next.lower > next.upper || next.upper < previous.lower) throw new Error("invalid presentation interval pair");
  return { lower: Math.max(0, nextFloat(next.lower - previous.upper, false)), upper: nextFloat(next.upper - previous.lower, true) };
}
const digest = (value: string | Buffer) => createHash("sha256").update(value).digest("hex");
async function immutable(file: string, value: unknown) {
  const bytes = `${JSON.stringify(value, null, 2)}\n`;
  await writeFile(file, bytes, { flag: "wx" });
  return digest(bytes);
}
export function expectedModelIdentity(model: any): string {
  const text = JSON.stringify(model); let a = 0x811c9dc5, b = 0x9e3779b9;
  for (let i = 0; i < text.length; i++) { a = Math.imul(a ^ text.charCodeAt(i), 0x01000193); b = Math.imul(b ^ text.charCodeAt(i), 0x85ebca6b); }
  return `ui-model-fnv32x2:${(a >>> 0).toString(16).padStart(8, "0")}${(b >>> 0).toString(16).padStart(8, "0")}:${text.length}`;
}
// Independent accepted legacy tree vocabulary; never import candidate search/index code.
function frozenLegacySearchTerms(type: string, record: any): unknown[] {
  const provenance = typeof record.provenance === "object" ? JSON.stringify(record.provenance) : record.provenance;
  switch (type) {
    case "project": return [record.description, "model"];
    case "material": case "node": case "combination": return [provenance];
    case "section": return [record.section_type, provenance, "pipe section"];
    case "pipe": return [record.material, provenance, "pipe segment"];
    case "support": return [record.hanger?.hanger_type, record.hanger?.source_reference,
      record.hanger?.mechanics_consumption, record.restraints?.join(" "), provenance];
    case "component": return [provenance, frozenComponentSearchTerms(record)];
    case "load": return [provenance, "load case"];
    default: return [];
  }
}
function frozenComponentSearchTerms(component: any): string {
  return [
    component.geometry?.bend_radius
      ? `${component.geometry.bend_radius.value} ${component.geometry.bend_radius.unit}`
      : "",
    component.geometry?.bend_angle
      ? `${component.geometry.bend_angle.value} ${component.geometry.bend_angle.unit}`
      : "",
    component.geometry?.bend_plane_orientation ?? "",
    component.geometry?.bend_pipe_ref ?? "",
    component.geometry?.bend_geometry_source_reference ?? "",
    component.geometry?.branch_header_pipe_ref ?? "",
    component.geometry?.branch_branch_pipe_ref ?? "",
    component.geometry?.branch_run_size
      ? `${component.geometry.branch_run_size.value} ${component.geometry.branch_run_size.unit}`
      : "",
    component.geometry?.branch_header_size
      ? `${component.geometry.branch_header_size.value} ${component.geometry.branch_header_size.unit}`
      : "",
    component.geometry?.branch_connection_angle
      ? `${component.geometry.branch_connection_angle.value} ${component.geometry.branch_connection_angle.unit}`
      : "",
    component.geometry?.branch_connection_type ?? "",
    component.geometry?.branch_reinforcement_reference ?? "",
    component.geometry?.branch_geometry_source_reference ?? "",
    component.geometry?.rigid_pipe_ref ?? "",
    component.geometry?.rigid_body_length
      ? `${component.geometry.rigid_body_length.value} ${component.geometry.rigid_body_length.unit}`
      : "",
    component.geometry?.end_a_size
      ? `${component.geometry.end_a_size.value} ${component.geometry.end_a_size.unit}`
      : "",
    component.geometry?.end_b_size
      ? `${component.geometry.end_b_size.value} ${component.geometry.end_b_size.unit}`
      : "",
    component.geometry?.weight ? `${component.geometry.weight.value} ${component.geometry.weight.unit}` : "",
    component.geometry?.center_of_gravity
      ? `${component.geometry.center_of_gravity.x} ${component.geometry.center_of_gravity.y} ${component.geometry.center_of_gravity.z} ${component.geometry.center_of_gravity.unit}`
      : "",
    component.geometry?.connection_end_a_reference ?? "",
    component.geometry?.connection_end_b_reference ?? "",
    component.geometry?.stiffness_behavior_reference ?? "",
    component.geometry?.rigid_component_source_reference ?? "",
    component.geometry?.expansion_joint_pipe_ref ?? "",
    component.geometry?.effective_area
      ? `${component.geometry.effective_area.value} ${component.geometry.effective_area.unit}`
      : "",
    component.geometry?.movement_limit
      ? `${component.geometry.movement_limit.value} ${component.geometry.movement_limit.unit}`
      : "",
    component.geometry?.hardware_reference ?? "",
    component.geometry?.manufacturer_reference ?? "",
    component.geometry?.pressure_thrust_reference ?? "",
    component.geometry?.expansion_joint_source_reference ?? "",
    component.modifiers?.sif_user_value
      ? `${component.modifiers.sif_user_value.value} ${component.modifiers.sif_user_value.unit}`
      : "",
    component.modifiers?.branch_header_sif_user_value
      ? `${component.modifiers.branch_header_sif_user_value.value} ${component.modifiers.branch_header_sif_user_value.unit}`
      : "",
    component.modifiers?.branch_branch_sif_user_value
      ? `${component.modifiers.branch_branch_sif_user_value.value} ${component.modifiers.branch_branch_sif_user_value.unit}`
      : "",
    component.modifiers?.flexibility_factor_user_value
      ? `${component.modifiers.flexibility_factor_user_value.value} ${component.modifiers.flexibility_factor_user_value.unit}`
      : "",
    component.modifiers?.stiffness_scaling_user_value
      ? `${component.modifiers.stiffness_scaling_user_value.value} ${component.modifiers.stiffness_scaling_user_value.unit}`
      : "",
    component.modifiers?.linear_stiffness_user_value
      ? `${component.modifiers.linear_stiffness_user_value.value} ${component.modifiers.linear_stiffness_user_value.unit}`
      : "",
    component.modifiers?.rotational_stiffness_user_value
      ? `${component.modifiers.rotational_stiffness_user_value.value} ${component.modifiers.rotational_stiffness_user_value.unit}`
      : "",
    component.modifiers?.axial_stiffness_user_value
      ? `${component.modifiers.axial_stiffness_user_value.value} ${component.modifiers.axial_stiffness_user_value.unit}`
      : "",
    component.modifiers?.lateral_stiffness_user_value
      ? `${component.modifiers.lateral_stiffness_user_value.value} ${component.modifiers.lateral_stiffness_user_value.unit}`
      : "",
    component.modifiers?.angular_stiffness_user_value
      ? `${component.modifiers.angular_stiffness_user_value.value} ${component.modifiers.angular_stiffness_user_value.unit}`
      : "",
    component.modifiers?.torsional_stiffness_user_value
      ? `${component.modifiers.torsional_stiffness_user_value.value} ${component.modifiers.torsional_stiffness_user_value.unit}`
      : "",
    component.modifiers?.source_reference ?? "",
    component.mechanics_interface?.solver_consumption ?? "",
    component.completeness?.map((finding: any) => `${finding.diagnostic_code} ${finding.status}`).join(" ") ?? ""
  ].join(" ");
}

export function frozenTreeExpectation(model: any, query: string) {
  const groups: [string, string, any[]][] = [["Materials", "material", model.materials ?? []], ["Sections", "section", model.sections ?? []],
    ["Nodes", "node", model.nodes], ["Pipes", "pipe", model.pipe_segments], ["Supports", "support", model.supports],
    ["Components", "component", model.components], ["Load Cases", "load", model.load_cases], ["Combinations", "combination", model.combinations ?? []],
    ["Diagnostics", "diagnostic", model.diagnostics ?? []]];
  const item = (type: string, record: any, group: string) => {
    const ref = { type, id: record.id }, label = type === "diagnostic" ? record.code : expectedVisibleEntityLabel(model, ref);
    const simple = ["id", "label", "name", "from", "to", "node", "kind", "status", "family", "basis", "code", "message"]
      .flatMap((key) => typeof record[key] === "string" ? [record[key]] : []);
    return { ref, label, matches: [record.id, label, type, ...simple, ...frozenLegacySearchTerms(type, record), type, label, record.id, group].filter(Boolean).join(" ").toLowerCase().includes(query.trim().toLowerCase()) };
  };
  const project = item("project", model.project, "Project");
  const filtered = groups.map(([title, type, records]) => ({ title, items: records.map((r) => item(type, r, title)).filter((i) => i.matches) }))
    .filter((g) => g.items.length);
  const totalCount = 1 + groups.reduce((sum, [, , records]) => sum + records.length, 0);
  const visibleCount = Number(project.matches) + filtered.reduce((sum, g) => sum + g.items.length, 0);
  const size = filtered.length + Number(project.matches);
  const rows: { testId: string; label: string; position: number; setSize: number; level: number; expanded?: true }[] = [];
  if (project.matches) rows.push({ testId: treeRowTestId(project.ref, "candidate"), label: project.label, position: 1, setSize: size, level: 1 });
  filtered.forEach((g, index) => {
    rows.push({ testId: `tree-group-${encodeURIComponent(g.title)}`, label: g.title, expanded: true,
      position: index + (project.matches ? 2 : 1), setSize: size, level: 1 });
    g.items.forEach((entry, i) => rows.push({ testId: treeRowTestId(entry.ref, "candidate"), label: entry.label,
      position: i + 1, setSize: g.items.length, level: 2 }));
  });
  // Frozen fresh-session comfortable density: ModelTree height 420/row 34,
  // VirtualList threshold 100 and overscan 8; reset pins focus inside this prefix.
  const mountedCount = rows.length < 100 ? rows.length : Math.min(rows.length, Math.ceil(420 / 34) + 16);
  return { query, totalCount, visibleCount, rows, mountedCount };
}
export function sameTracePresentedGap(previousUs: number, nextUs: number) {
  if (![previousUs,nextUs].every(t=>Number.isSafeInteger(t)&&t>=0) || nextUs<previousUs) throw new Error("invalid same-trace integer timestamps");
  const delta=BigInt(nextUs)-BigInt(previousUs);
  return { lower: Math.max(0,nextFloat(Number(delta-1n)/1000,false)), upper: nextFloat(Number(delta+1n)/1000,true) };
}
export function bindSameTraceDurationBasis({rawBytes,capture,trace,extraction,stopped}: any): SameTraceDurationBasis {
  const active=stopped?.active, action=active?.actionMarker, results=extraction?.presentations;
  if (!Buffer.isBuffer(rawBytes) || rawBytes.length > CHROMIUM_TRACE_RAW_BYTE_LIMIT || !capture?.rawTraceComplete || trace?.traceDataLossOccurred!==false ||
      trace?.rawTraceTransport?.rawCompleteThroughEof!==true || digest(rawBytes)!==capture.rawTraceSha256 ||
      capture.rawTraceSha256!==trace.rawTraceTransport.rawSha256 ||
      extraction?.status!=="PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS" ||
      !Object.entries(REQUIRED_CHROMIUM_BINDING).every(([k,v])=>extraction.sourceBinding?.[k]===v) ||
      !Array.isArray(results) || !results.length || active?.feedbackKind!=="orbit" || action?.token!==active.token) throw new Error("same-trace capture/source qualification unavailable");
  // Transport retains lossless events but not top-level metadata. Parse only to read
  // clock-domain after hashing these bounded bytes; never use JSON.parse numeric events.
  const clockDomain=JSON.parse(rawBytes.toString("utf8"))?.metadata?.["clock-domain"];
  if (clockDomain!==SAME_TRACE_EXPORT_PROFILE.clockDomain) throw new Error("same-trace clock domain mismatch");
  const first=results[0], events=capture.events;
  const actionEvents=events.filter((e:any)=>e.name==="TimeStamp"&&e.args?.data?.message===`UIF_CAUSAL_V1:ACTION:${active.token}`);
  if(actionEvents.length!==1 || actionEvents[0].ts!==first.actionTraceTimestamp || actionEvents[0].pid!==first.rendererProcessId || actionEvents[0].tid!==first.rendererMainThreadId ||
     typeof actionEvents[0].args?.data?.frame!=="string") throw new Error("same-trace document/action unavailable");
  const documentFrame=actionEvents[0].args.data.frame;
  for(const r of results) {
    const feedback=active.feedbackMarkers?.find((m:any)=>m.markerIdentity===r.markerIdentity);
    const marker=events.filter((e:any)=>e.name==="TimeStamp"&&e.args?.data?.message===`UIF_CAUSAL_V1:FEEDBACK:${r.markerIdentity}`);
    const end=events[r.reporterEndEventIndex], begin=events[r.reporterBeginEventIndex];
    if(r.status!=="PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION" || r.feedbackKind!=="orbit" || r.token!==active.token || !feedback ||
      r.actionTraceTimestamp!==first.actionTraceTimestamp || r.canvasEpoch!==active.armedCanvasEpoch || r.contextEpoch!==active.armedContextEpoch ||
      ["rendererProcessId","rendererMainThreadId","rendererCompositorThreadId","layerTreeId","modelGeneration"].some(k=>r[k]!==first[k]) ||
      !Object.entries(PAGE_CLOCK_SOURCE).every(([k,v])=>r.pageClockSource?.[k]===v) ||
      typeof action.traceClock?.crossOriginIsolated!=="boolean" || r.pageClockSource.crossOriginIsolated!==action.traceClock.crossOriginIsolated ||
      feedback.canvasEpoch!==r.canvasEpoch || feedback.contextEpoch!==r.contextEpoch ||
      marker.length!==1 || marker[0].args?.data?.frame!==documentFrame || marker[0].pid!==r.rendererProcessId || marker[0].tid!==r.rendererMainThreadId ||
      begin?.name!=="PipelineReporter" || begin.ph!=="b" || end?.name!=="PipelineReporter" || end.ph!=="e" ||
      begin.pid!==r.rendererProcessId || end.pid!==r.rendererProcessId || begin.tid!==r.rendererCompositorThreadId || end.tid!==r.rendererCompositorThreadId ||
      end.ts!==r.presentationTraceTimestamp) throw new Error("same-trace endpoint identity/source mismatch");
    sameTracePresentedGap(first.presentationTraceTimestamp,r.presentationTraceTimestamp);
  }
  const basis: SameTraceDurationBasis={kind:"same-trace-integer-us/v1",qualification:"PASS_CALLER_BOUND_SAME_TRACE",
    rawCaptureSha256:capture.rawTraceSha256,profile:SAME_TRACE_EXPORT_PROFILE,documentFrame,documentTimeOrigin:active.documentTimeOrigin,documentEvidenceEpoch:active.evidenceEpoch,actionListenerObservedAt:action.listenerObservedAt,crossOriginIsolated:action.traceClock.crossOriginIsolated,
    actionToken:active.token,actionTraceTimestamp:first.actionTraceTimestamp,canvasEpoch:active.armedCanvasEpoch,contextEpoch:active.armedContextEpoch,
    modelGeneration:first.modelGeneration,rendererProcessId:first.rendererProcessId,rendererMainThreadId:first.rendererMainThreadId,
    rendererCompositorThreadId:first.rendererCompositorThreadId,layerTreeId:String(first.layerTreeId),
    references:results.map((r:any)=>({rawCaptureSha256:capture.rawTraceSha256,markerIdentity:r.markerIdentity,reportedTimestamp:r.presentationTraceTimestamp,
      reporterOccurrence:r.pipelineReporterOccurrence,reporterBeginEventIndex:r.reporterBeginEventIndex,reporterEndEventIndex:r.reporterEndEventIndex}))};
  if(!validSameTraceBasis(basis,results.length))throw new Error("same-trace declared basis invalid");
  return basis;
}
export function constructOrbitEvidence(results: readonly any[], actionAt: number, mode: OrbitEvidence["mode"], durationBasis: SameTraceDurationBasis): OrbitEvidence {
  if (actionAt!==durationBasis?.actionListenerObservedAt || !validSameTraceBasis(durationBasis,results.length) || results.some((r:any,i:number)=>
      r.presentationTraceTimestamp!==durationBasis.references[i].reportedTimestamp || r.markerIdentity!==durationBasis.references[i].markerIdentity ||
      r.token!==durationBasis.actionToken || r.actionTraceTimestamp!==durationBasis.actionTraceTimestamp ||
      r.canvasEpoch!==durationBasis.canvasEpoch || r.contextEpoch!==durationBasis.contextEpoch ||
      r.rendererProcessId!==durationBasis.rendererProcessId || r.rendererMainThreadId!==durationBasis.rendererMainThreadId ||
      r.rendererCompositorThreadId!==durationBasis.rendererCompositorThreadId || String(r.layerTreeId)!==durationBasis.layerTreeId ||
      r.modelGeneration!==durationBasis.modelGeneration || r.status!=="PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION" ||
      r.feedbackKind!=="orbit" || r.reporterBeginEventIndex!==durationBasis.references[i].reporterBeginEventIndex ||
      r.reporterEndEventIndex!==durationBasis.references[i].reporterEndEventIndex || r.pipelineReporterOccurrence!==durationBasis.references[i].reporterOccurrence)) throw new Error("orbit requires bound same-trace endpoint context");
  // Absolute page windows are prescribed from the captured action, not from
  // IPC receipt times or the actual loop completion (which may overshoot).
  const warmup = { startMs: actionAt, endMs: actionAt + 2000 }, measured = { startMs: actionAt + 2000, endMs: actionAt + 12000 };
  const presentationIntervals = results.map((r: any) => ({ lower: nextFloat(actionAt + r.actionToPresentationIntervalMs.lower, false),
    upper: nextFloat(actionAt + r.actionToPresentationIntervalMs.upper, true), traceMs: r.presentationTraceTimestamp / 1000 }));
  const offsetMin = Math.max(...results.map((r: any) => r.pageToTraceOffsetIntervalMs.minimum));
  const offsetMax = Math.min(...results.map((r: any) => r.pageToTraceOffsetIntervalMs.maximum));
  if (offsetMin > offsetMax) throw new Error("orbit has no common source-bound clock mapping");
  const first = presentationIntervals.reduce((found: number, p: any, i: number) => p.upper < measured.startMs ? i : found, -1);
  const last = presentationIntervals.findIndex((p: any) => p.lower > measured.endMs);
  if (first < 0 || last <= first + 1) throw new Error("orbit lacks conservative bracketing coverage");
  const endpoints = presentationIntervals.map((p: any, sourceIndex: number) => ({ sourceIndex,
    reportedTimestamp: results[sourceIndex].presentationTraceTimestamp, coordinateMs: p.traceMs - offsetMin,
    intervalMs: { lower: p.lower, upper: p.upper } }));
  const mapped = presentationIntervals.slice(first, last + 1);
  const shift = offsetMin; // A bound for coordinates only; no fitted/averaged clock or cost subtraction.
  const gaps = mapped.slice(1).map((p: any, i: number) => ({ sampleId: i + 1,
    fromMs: mapped[i].traceMs - shift, toMs: p.traceMs - shift, fromSourceIndex: first + i, toSourceIndex: first + i + 1,
    qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE" as const,
    durationIntervalMs: sameTracePresentedGap(results[first+i].presentationTraceTimestamp,results[first+i+1].presentationTraceTimestamp) }));
  return { mode, qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE", coverageStatus: "PASS_COMPLETE_BRACKETED_PRESENTATIONS",
    labelsOn: true, warmup, measured, durationBasis, traceToPageOffsetMs: offsetMin, endpoints, envelope: { first, last }, gaps,
    ...(mode === "actual-od" ? { odConversion: { status: "PASS_CURRENT_CONVERSION" as const, cache: "warm" as const } } : {}) };
}

export function selectedEpochEvidence(evidence: any) {
  const active = evidence?.active;
  if (!active || !Array.isArray(active.feedbackMarkers)) throw new Error("missing complete stopped observations");
  if (!["assignment", "tree-filter"].includes(active.feedbackKind)) return evidence;
  if (active.contentProof?.status !== "PASS_EXACT_STOPPED_CONTENT") throw new Error("stopped content reconciliation failed");
  const current = active.feedbackMarkers.filter((m: any) => m.domEpoch === active.contentProof.domEpoch);
  if (current.length !== 1) throw new Error("unchanged final content epoch is absent or ambiguous");
  return { ...evidence, active: { ...active, feedbackMarkers: current },
    excludedInvalidatedContentMarkers: active.feedbackMarkers.filter((m: any) => m.domEpoch !== active.contentProof.domEpoch) };
}
export function segmentObserverRequirements(evidence: any, restorationStatus: string, productObservationRaf: number | null) {
  const a = evidence.active;
  const common = { feedbackObservations: a.feedbackMarkers, requiredSourceOperations: ["raf-registration"],
    scheduledAdditionalAnimationFrames: 0, productObservationRaf, restorationStatus };
  if (a.feedbackKind === "assignment") return { ...common, actionClass: "assignment" as const, minimumRafCallbacks: 1,
    requiredSourceOperations: ["raf-registration", "assignment-calibration", "dom-stopped-reconciliation"],
    expectedOperations: [{ identity: `${a.token}.calibration`, operation: "assignment-calibration", start: a.actionMarker.traceClock.before, end: a.actionMarker.traceClock.after }] };
  if (a.feedbackKind === "tree-filter") return { ...common, actionClass: "tree-filter" as const, minimumRafCallbacks: 0,
    requiredSourceOperations: ["input-capture-listener-total", "dom-mutation-callback", "dom-stopped-reconciliation"],
    expectedOperations: [{ identity: `${a.token}.input`, operation: "input-capture-listener-total", start: a.actionMarker.listenerObservedAt, end: a.actionMarker.listenerObservedAt }] };
  return { ...common, actionClass: "pointer" as const, minimumRafCallbacks: a.feedbackMarkers.length,
    requiredSourceOperations: ["raf-registration", "pointerdown-capture-listener-total", "pointerup-capture-listener-total"],
    pointerTransaction: a.pointerTransaction, actionMarker: a.actionMarker, actionStartEvent: a.actionStartEvent };
}

export function focusedBoxSamples(fixture: LoadedFixture) {
  const samples = [1, 7].map((id) => fixture.samples.box_selection.find((s: any) => s.sample === id));
  const oracles = [1, 7].map((id) => fixture.pointOracle.candidate_box_selection?.samples?.find((s: any) => s.sample === id));
  if (samples.some((s) => !s) || oracles[0]?.orderedRefs?.length !== 0 || oracles[0]?.primaryRef !== null ||
      oracles[0]?.direction !== "left-to-right" || oracles[0]?.filter !== "all" ||
      oracles[1]?.direction !== "left-to-right" || oracles[1]?.filter !== "pipes" || oracles[1]?.orderedRefs?.length !== 181 ||
      digest(JSON.stringify(oracles[1].orderedRefs)) !== "1b1eced5fc5deaf7050c72f096259b4115b190ff97fb40c4865193526e75e732" ||
      oracles[1]?.primaryRef?.type !== "pipe" || oracles[1]?.primaryRef?.id !== "entity:UIF-TYPED-COLLISION-01000") throw new Error("focused independent box1/box7 binding mismatch");
  return samples;
}

async function resetProject(page: Page, fixture: LoadedFixture, timeout: number) {
  const alternate = { type: "material", id: fixture.model.materials[0].id }, project = { type: "project", id: fixture.model.project.id };
  const snapshots = [];
  // Real keyboard navigation remounts the first virtual rows after viewport
  // selection has legitimately moved tree focus to a distant entity.
  const commands = benchmarkCommands(page, timeout);
  await commands.home();
  for (const ref of [alternate, project]) {
    const before = (await readCandidateDiagnostics(page)).snapshot;
    await commands.select(ref);
    snapshots.push(await waitForCandidateExclusiveSelection(page, ref, before.model.generation,
      before.viewport.selection.actionSequence, before.viewport.mainRender.submissionSequence, timeout));
  }
  return snapshots;
}
async function cameraRecipe(page: Page, fixture: LoadedFixture, timeout: number) {
  await benchmarkCommands(page, timeout).camera();
  await delay(FULL_COHORT_RECIPE.settleMs);
  return validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
}

export const requiredMethodFiles = [
    "fresh-demo-policy.mjs", "fresh-demo-policy.d.mts", "fresh-demo-policy.spec.ts", "fixtures/frozen-oracle-geometry.ts.txt",
    "characterization-mode.ts", "characterization-commands.ts", "characterization-observations.mjs",
    "characterization-observations.d.mts", "verify-characterization-observations.mjs", "README.md",
    "benchmark-harness.ts",
    "candidate-server-response.ts",
    "chromium-compositor-trace.ts",
    "lossless-trace-json.mjs",
    "lossless-trace-json.d.mts",
    "causal-presentation-extractor.mjs",
    "causal-presentation-extractor.d.mts",
    "causal-presentation-preflight.benchmark.ts",
    "verify-causal-presentation-extractor.mjs",
    "playwright.causal-presentation-preflight.config.ts",
    "playwright.candidate-causal-presentation-preflight.config.ts",
    "fixture-manifest.json",
    "causal-method-contract.ts",
    "causal-phase-journal.ts",
    "causal-method-contract.spec.ts",
    "playwright.causal-method-contract.config.ts",
    "tsconfig.causal-method.json",
    "performance-targets.ts",
    "performance-targets.spec.ts",
    "ui-foundation-performance.benchmark.ts",
    "playwright.candidate-performance.config.ts",
    "full-cohort-controller.ts",
    "full-cohort-controller.spec.ts",
    "playwright.performance.config.ts",
    "serve-bound-candidate-output.mjs", "freeze-candidate-point-oracle.mjs", "point-hit-oracle.mjs", "box-selection-oracle.mjs"
];
export function validateInventoryCoverage(declared: readonly {path:string}[], actual: readonly string[], root: string) {
  const names=declared.map(e=>e.path).filter(name=>name.startsWith(`${root}/`)).sort();
  if(new Set(declared.map(e=>e.path)).size!==declared.length || JSON.stringify(names)!==JSON.stringify([...actual].sort())) throw new Error(`incomplete or duplicate product inventory: ${root}`);
}
export function validateFinalProductProvenance(stage: string, revision: string, gitHead: string, sameRoot: boolean, expectedRevision = CHARACTERIZATION_PRODUCT_REVISION) {
  if(stage!=="final" || revision!==expectedRevision || gitHead!==expectedRevision || sameRoot)throw new Error("separate frozen final product provenance required");
}
export async function validateCharacterizationProduct(binding: CandidateDriverBinding, bundle: any) {
  const {stdout}=await promisify(execFile)("git",["rev-parse","HEAD"],{cwd:binding.candidateSourceRoot});
  validateFinalProductProvenance(binding.sourceStage,bundle.productRevision,stdout.trim(),await realpath(binding.candidateSourceRoot)===await realpath(process.cwd()),freshDemoPolicy()?.productRevision ?? CHARACTERIZATION_PRODUCT_REVISION);
  const inventory = async (relative: string): Promise<string[]> => {
    const entries = await readdir(path.join(binding.candidateSourceRoot, relative), { withFileTypes: true });
    const result: string[] = [];
    for (const entry of entries) {
      const name = `${relative}/${entry.name}`;
      if (entry.isDirectory()) result.push(...await inventory(name));
      else if (entry.isFile()) result.push(name);
      else throw new Error("unsupported product inventory entry");
    }
    return result.sort();
  };
  const sources = [...(bundle.source ?? []), ...(bundle.mutableTestOnlySourceSnapshot ?? [])];
  for (const [root, declared] of [["apps/desktop/src", sources], ["apps/desktop/src-tauri/src", sources], ["apps/desktop/dist", bundle.files], ["apps/desktop/public/wasm-engine", bundle.files]] as const)
    validateInventoryCoverage(declared,await inventory(root),root);
  for(const entry of [...sources,...bundle.files]) {
    if(typeof entry.path!=="string" || path.isAbsolute(entry.path) || entry.path.split(/[\\/]/).includes(".."))throw new Error("product inventory path invalid");
    const bytes=await readFile(path.join(binding.candidateSourceRoot,entry.path));
    if(bytes.length!==entry.bytes || digest(bytes)!==entry.sha256)throw new Error(`product source/dist bytes drift: ${entry.path}`);
  }
}
export async function performanceExpectation(fixture: LoadedFixture, binding: CandidateDriverBinding,
  identity: Omit<RunExpectation, "bindings">): Promise<RunExpectation> {
  const currentEntry = bindCandidateDriverEntry(binding.driver);
  if (JSON.stringify(currentEntry) !== JSON.stringify(binding)) throw new Error("candidate entry binding drift");
  if (JSON.stringify(JSON.parse(await readFile(fixture.samplesPath, "utf8"))) !== JSON.stringify(fixture.samples)) throw new Error("loaded sample inventory drift");
  if (fixture.model.pipe_segments?.length !== identity.fixtureSize || fixture.samples.fixture_pipe_count !== identity.fixtureSize ||
      fixture.samples.point_selection?.length !== 200 || fixture.samples.box_selection?.length !== 20 || fixture.samples.tree_filters?.length !== 20) {
    throw new Error("fixture/run/sample inventory mismatch");
  }
  if (digest(await readFile(fixture.modelPath)) !== digest(fixture.bytes) ||
      digest(await readFile(fixture.pointOraclePath)) !== fixture.pointOracleSha256) throw new Error("loaded fixture/oracle bytes drift");
  await validateCandidateOracleBinding(fixture, identity.fixtureSize);
  const methodPath = process.env.UI_FOUNDATION_METHOD_MANIFEST_PATH;
  const methodSha = process.env.UI_FOUNDATION_METHOD_MANIFEST_SHA256;
  if (!methodPath || !path.isAbsolute(methodPath) || !/^[a-f0-9]{64}$/.test(methodSha ?? "")) throw new Error("final method manifest is required");
  const bytes = await readFile(methodPath);
  if (digest(bytes) !== methodSha) throw new Error("method manifest hash drift");
  const method = JSON.parse(bytes.toString());

  if (!Array.isArray(method.files) || method.files.length !== requiredMethodFiles.length || new Set(method.files.map((f: any) => f.path)).size !== method.files.length ||
      requiredMethodFiles.some((name) => !method.files.some((f: any) => f.path === `apps/desktop/e2e/ui-foundation/${name}`))) throw new Error("method inventory incomplete");
  for (const file of method.files) {
    if (!file.path.startsWith("apps/desktop/e2e/ui-foundation/") || file.path.includes("..") ||
        digest(await readFile(path.resolve(process.cwd(), file.path))) !== file.sha256) throw new Error(`method source drift: ${file.path}`);
  }
  const bundleBytes = await readFile(binding.candidateBundleManifestPath);
  if (digest(bundleBytes) !== binding.candidateBundleManifestSha256) throw new Error("candidate bundle manifest changed");
  const bundle = JSON.parse(bundleBytes.toString());
  if (collectionMode(process.env.UI_FOUNDATION_COLLECTION_MODE) === "characterization") await validateCharacterizationProduct(binding,bundle);
  for (const [root, entries] of [[binding.candidateSourceRoot, bundle.source], [binding.candidateSourceRoot, bundle.files]] as const) {
    if (!Array.isArray(entries) || !entries.length) throw new Error("candidate source/dist inventory absent");
    for (const entry of entries) {
      if (typeof entry.path !== "string" || path.isAbsolute(entry.path) || entry.path.split(/[\\/]/).includes("..")) throw new Error("candidate inventory path escapes root");
      const bytes = await readFile(path.join(root, entry.path));
      if (bytes.length !== entry.bytes || digest(bytes) !== entry.sha256) throw new Error(`candidate bytes drift: ${entry.path}`);
    }
  }
  const browser = bindRequiredChromiumExecutable();
  return { ...identity, bindings: { validationStatus: "PASS_CALLER_VALIDATED_BINDINGS",
    sourceSha256: bundle.sourceAggregateSha256, buildSha256: binding.candidateBundleManifestSha256,
    fixtureSha256: digest(fixture.bytes), browserSha256: browser.sha256,
    oracleSha256: fixture.pointOracleSha256, methodSha256: methodSha!, samplesSha256: digest(await readFile(fixture.samplesPath)) } };
}

export const POST_STOP_COLLECTION_MS = 250;
export function candidateDiagnosticMode(value: string | undefined, focused = false, assignmentOnly?: boolean) {
  if (value !== undefined && (value !== "assignment-collection" || focused)) throw new Error("unknown or conflicting candidate diagnostic mode");
  const mode=value === undefined ? focused ? "focused" : "full" : "assignment-collection";
  if(assignmentOnly!==undefined && assignmentOnly!==(mode==="assignment-collection"))throw new Error("diagnostic flag/options conflict");
  return mode;
}
// Host-only tail: stopped evidence has already been persisted. Finalization runs
// exactly once even if waiting fails; its error cannot erase the original error.
export async function finalizeStoppedCollection(kind: string, stoppedSha256: string,
  finalize: () => Promise<void>, wait: (ms: number) => Promise<unknown> = delay) {
  const budgetMs = ["assignment", "point-selection", "box-selection"].includes(kind) && /^[a-f0-9]{64}$/.test(stoppedSha256) ? POST_STOP_COLLECTION_MS : 0;
  const errors: { phase: string; error: string }[] = [];
  const startedAtHostMs = nodePerformance.now();
  try { if (budgetMs) await wait(budgetMs); }
  catch (error) { errors.push({ phase: "collection-tail", error: String(error) }); }
  finally { try { await finalize(); } catch (error) { errors.push({ phase: "trace-finalize", error: String(error) }); } }
  return { policy: "fixed-post-stop-host-collection/v1", kind, stoppedSha256, budgetMs,
    startedAtHostMs, finalizedAtHostMs: nodePerformance.now(), timingMeaning: "collection bookkeeping only; never a metric endpoint", errors };
}
// Shared by the real action wrapper and pure controls. The stop promise records
// the first attempt before either snapshot acquisition or persistence can fail.
export async function runStoppedActionLifecycle(kind: string, operations: {
  begin: () => Promise<void>;
  work: (stop: () => Promise<{ evidence: any; stoppedSha256: string }>) => Promise<any>;
  stop: () => Promise<any>;
  persist: (evidence: any) => Promise<string>;
  finalize: () => Promise<void>;
  wait?: (ms: number) => Promise<unknown>;
}) {
  const errors: { phase: string; error: string }[] = [];
  let began=false, stopped:any=null, stoppedSha256="", auxiliary:any=null;
  let stopAttempt: Promise<{ evidence:any; stoppedSha256:string }> | undefined;
  const stopOnce = () => {
    if (!stopAttempt) stopAttempt=(async()=>{
      try { stopped=await operations.stop(); }
      catch(error){errors.push({phase:"stop",error:String(error)});throw error;}
      try { stoppedSha256=await operations.persist(stopped); }
      catch(error){errors.push({phase:"stop-persist",error:String(error)});throw error;}
      return {evidence:stopped,stoppedSha256};
    })();
    return stopAttempt;
  };
  let collection: Awaited<ReturnType<typeof finalizeStoppedCollection>> | null=null;
  try { await operations.begin();began=true;auxiliary=await operations.work(stopOnce); }
  catch(error){errors.push({phase:"work",error:String(error)});}
  finally {
    if(began) {try {await (stopAttempt ?? stopOnce());}catch{/* Original stop/persist error is retained above. */}}
    if(began) {
      collection=await finalizeStoppedCollection(kind,stoppedSha256,operations.finalize,operations.wait);
      errors.push(...collection.errors);
    }
  }
  return {stopped,stoppedSha256,auxiliary,collection,errors};
}

export function assignmentDiagnosticResult(expected: RunExpectation, assignment: any, errors: readonly unknown[], segmentCount: number) {
  const i=assignment?.durationIntervalMs;
  const pass=Array.isArray(errors) && errors.length===0 && segmentCount===1 && assignment?.qualification==="PASS_QUALIFIED_CAUSAL_EVIDENCE" &&
    Number.isFinite(i?.lower) && Number.isFinite(i?.upper) && i.lower>=0 && i.upper>=i.lower && i.upper<=2000;
  return { status: pass ? "PASS_ASSIGNMENT_COLLECTION_DIAGNOSTIC" : "FAIL_ASSIGNMENT_COLLECTION_DIAGNOSTIC",
    scope: "ASSIGNMENT_ONLY_INCOMPLETE_WORKLOAD", cohortContribution: 0, expected, assignment, errors, segmentCount, targetUpperMs: 2000 };
}
export function assignmentDiagnosticSummary(results: readonly any[], expected: readonly RunExpectation[]) {
  const planned=expected.length===10 && new Set(expected.map(e=>e.sessionId)).size===10 &&
    [1000,10000].every(n=>[1,2,3,4,5].every(r=>expected.filter(e=>e.fixtureSize===n&&e.runNumber===r).length===1));
  const pass=planned && results.length===10 && expected.every(e=>{
    const matches=results.filter(r=>JSON.stringify(r.expected)===JSON.stringify(e));
    return matches.length===1 && matches[0].scope==="ASSIGNMENT_ONLY_INCOMPLETE_WORKLOAD" && matches[0].cohortContribution===0 &&
      matches[0].status==="PASS_ASSIGNMENT_COLLECTION_DIAGNOSTIC" &&
      assignmentDiagnosticResult(e,matches[0].assignment,matches[0].errors,matches[0].segmentCount).status==="PASS_ASSIGNMENT_COLLECTION_DIAGNOSTIC";
  });
  return { status: pass ? "PASS_TEN_ASSIGNMENT_COLLECTION_DIAGNOSTICS" : "FAIL_ASSIGNMENT_COLLECTION_DIAGNOSTICS",
    scope: "ASSIGNMENT_ONLY_INCOMPLETE_WORKLOAD", cohortContribution: 0, plannedSessions: 10, completedSessions: results.length };
}

type Segment = { token: string; kind: string; directory: string; stoppedPath: string; stoppedSha256: string;
  extraction: any; auxiliary: any; window: { start: number; end: number } };
export async function runCandidateCausalPerformance(page: Page, fixture: LoadedFixture, expected: RunExpectation,
  directory: string, options: { focused?: boolean; assignmentOnly?: boolean; timeoutMs: number; binding: CandidateDriverBinding }) {
  candidateDiagnosticMode(process.env.UI_FOUNDATION_DIAGNOSTIC_MODE, options.focused, options.assignmentOnly === true);
  const mode = collectionMode(process.env.UI_FOUNDATION_COLLECTION_MODE, "candidate", process.env.UI_FOUNDATION_DIAGNOSTIC_MODE, options.focused);
  const characterization = mode === "characterization";
  await mkdir(directory, { recursive: true });
  const actualViewport = await page.evaluate(() => ({ width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio }));
  await immutable(path.join(directory, "initialized.json"), { expected, actualViewport,
    fixturePreparationMilliseconds: { fileRead: fixture.readMs, modelParse: fixture.parseMs,
      supportFilesRead: fixture.supportFilesReadMs, supportFilesParse: fixture.supportFilesParseMs,
      meaning: "Host preparation outside original product assignment interval; no subtraction" },
    mode, recipe: FULL_COHORT_RECIPE, focused: options.focused === true,
    qualificationScope: options.assignmentOnly ? "ASSIGNMENT_ONLY_INCOMPLETE_WORKLOAD_ZERO_COHORT" : options.focused ? "METHOD_ONLY_ZERO_COHORT_SAMPLES" : "PRESCRIBED_INSTRUMENTED_COHORT_RUN" });
  if (actualViewport.width !== 1440 || actualViewport.height !== 920 || actualViewport.deviceScaleFactor !== 2) throw new Error("prescribed viewport/DPR binding mismatch");
  const initialStorage = await page.context().storageState();
  if (initialStorage.cookies.length || initialStorage.origins.length || page.context().pages().length !== 1 || page.url() !== "about:blank") throw new Error("fresh Playwright application context required");
  const segments: Segment[] = [];
  const errors: { phase: string; error: string }[] = [];
  let restore: any = null;
  let profile: Awaited<ReturnType<typeof bindReferenceProfile>> | undefined, display: any;
  let initialBoundary: any;
  const boundaries: any[] = [];
  const boundary = async (id: string) => {
    if (!characterization) return undefined;
    const metadata = await captureBoundary(page, expected, id, initialBoundary, directory);
    initialBoundary ??= metadata;
    const ref = await persistBoundary(directory, metadata); boundaries.push(ref); return ref;
  };
  const timeout = options.timeoutMs;
  const action = async (kind: string, sample: number, work: (token: string, dir: string, stopAction: () => Promise<{evidence:any;stoppedSha256:string}>) => Promise<any>) => {
    const token = `${expected.runId}.${kind}.${sample}`;
    const dir = path.join(directory, `${kind}-${String(sample).padStart(3, "0")}`);
    await mkdir(dir);
    await immutable(path.join(dir, "prepared.json"), { token, kind, sample, expected, recipe: FULL_COHORT_RECIPE });
    const traceState: {capture: ChromiumTraceCapture | null} = {capture:null};
    let trace: any = null, stopped: any = null, auxiliary: any = null;
    const lifecycle = await runStoppedActionLifecycle(kind, {
      begin: async () => {
        // Before reset/setup, so original Send may precede the measured action.
        traceState.capture = await beginChromiumCompositorTrace(page, token, path.join(dir, "trace-events.raw.json"), "discrete-segment");
      },
      work: stopAction => work(token, dir, stopAction),
      stop: () => stopCausalFeedbackMarker(page, token),
      persist: async evidence => {
        const sha = await immutable(path.join(dir, "action-stopped.json"), evidence);
        await boundary(`${kind}-${sample}-stopped`);
        return sha;
      },
      finalize: async () => {
        trace = await endChromiumCompositorTrace(page, traceState.capture!);
        await immutable(path.join(dir, "trace-finalized.json"), trace);
      }
    });
    stopped=lifecycle.stopped;auxiliary=lifecycle.auxiliary;
    const stoppedSha256=lifecycle.stoppedSha256;
    errors.push(...lifecycle.errors.map(e=>({phase:`${token}:${e.phase}`,error:e.error})));
    if(lifecycle.collection) {
      try { await immutable(path.join(dir,"collection-tail.json"),lifecycle.collection); }
      catch(error){errors.push({phase:`${token}:collection-persist`,error:String(error)});}
    }
    if (errors.length || !traceState.capture || !stopped || !traceState.capture.rawTraceComplete || !traceState.capture.rawTraceSha256 ||
        trace?.traceDataLossOccurred !== false || trace?.markerValidation?.status !== "PASS_REQUIRED_MARKERS_PRESENT_ONCE_WITH_FINITE_ORDERED_TIMESTAMPS") {
      await immutable(path.join(dir, "failed.json"), { errors, trace, stoppedAvailable: Boolean(stopped), stopped, stoppedSha256 });
      throw new Error(`segment ${token} failed; canonical earlier evidence retained`);
    }
    try {
    assertStoppedCausalEvidence(stopped, kind as any, true);
    const selected = selectedEpochEvidence(stopped);
    const extraction = extractCausalPresentations(traceState.capture.events, REQUIRED_CHROMIUM_BINDING, selected);
    if (extraction.status !== "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS") throw new Error("causal extraction incomplete");
    const window = { start: stopped.active.actionMarker.listenerObservedAt,
      end: Math.max(...stopped.active.feedbackMarkers.map((m: any) => m.callbackCompletedAt)) };
    // Post-window only. A bounded reread supplies metadata omitted by the existing
    // transport result; bytes/temporary parsed metadata are not retained in segment evidence.
    const orbitDurationBasis = kind === "orbit" ? bindSameTraceDurationBasis({ rawBytes: await readFile(traceState.capture.rawTracePath),
      capture: traceState.capture, trace, extraction, stopped }) : undefined;
    const segment = { token, kind, orbitDurationBasis, directory: dir, stoppedPath: path.join(dir, "action-stopped.json"), stoppedSha256,
      extraction, auxiliary: { ...auxiliary, evidence: undefined }, window };
    await immutable(path.join(dir, "derived.json"), { ...segment, metadataReferences: boundaries.filter(b => b.id.startsWith(`${kind}-${sample}-`)), excludedInvalidatedContentMarkers: selected.excludedInvalidatedContentMarkers });
    segments.push(segment);
    const drain = await page.evaluate(({ token, sha }) => (globalThis as any).__uifHarness.causal.acknowledgeStopped(token, sha), { token, sha: stoppedSha256 });
    await immutable(path.join(dir, "drain-acknowledged.json"), drain);
    traceState.capture.events = [];
    return segment;
    } catch (error) {
      errors.push({ phase: `${token}:derivation`, error: String(error) });
      await immutable(path.join(dir, "failed.json"), { errors, stoppedSha256, rawSha256: traceState.capture.rawTraceSha256 });
      throw error;
    }
  };

  const modelExpectation = (snapshot: any) => ({ projectId: fixture.model.project.id, modelIdentityHash: expectedModelIdentity(fixture.model),
    modelGeneration: snapshot.model.generation, indexGeneration: snapshot.model.indexGeneration,
    projectSessionGeneration: snapshot.model.projectSessionGeneration, priorRenderSubmissionSequence: snapshot.viewport.mainRender.submissionSequence });
  let assignment: Segment | null = null;
  const points: ActionDuration[] = [], boxes: ActionDuration[] = [], filters: ActionDuration[] = [];
  const orbitResults: Partial<Record<"centerline" | "actual-od", OrbitEvidence>> = {};
  let settled: any = null, environment: any = null;
  const metric = (segment: Segment, sampleId: number): ActionDuration => ({ sampleId,
    qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE", durationIntervalMs: {
      lower: Math.max(0, segment.extraction.results[0].actionToPresentationIntervalMs.lower),
      upper: segment.extraction.results[0].actionToPresentationIntervalMs.upper } });
  try {
    if (characterization) {
      profile = await bindReferenceProfile();
      display = await captureDisplayProfile(directory, "before", profile.record.display);
      await immutable(path.join(directory, "reference-profile-before.json"), { profile, display });
    }
    const assignmentToken = `${expected.runId}.assignment.1`;
    await installInstrumentation(page, { captureGlobalRaf: false, causalFeedbackMarkers: true,
      assignmentSpecification: { token: assignmentToken, phase: "candidate", feedbackKind: "assignment", actionStartEvent: "assignment",
        expectedActionTargetTestId: "model-tree-virtual", maximumFeedbackMarkers: FULL_COHORT_RECIPE.maximumFeedbackMarkers,
        actionIdentity: { fixtureSha256: expected.bindings.fixtureSha256, runId: expected.runId, sessionId: expected.sessionId },
        treeExpectation: frozenTreeExpectation(fixture.model, ""), candidateExpectation: {
          projectId: fixture.model.project.id, modelIdentityHash: expectedModelIdentity(fixture.model), priorRenderSubmissionSequence: 0 } } });
    await routeModelFixture(page, fixture);
    assignment = await action("assignment", 1, async (token, dir, stopAction) => {
      const response = await page.goto("/", { waitUntil: "domcontentloaded", timeout });
      assertBoundCandidateDocumentResponse(response);
      await delay(FULL_COHORT_RECIPE.settleMs);
      return stopAction();
    });
    await boundary("initial-presentation");
    // Same-model responsiveness witness is outside the assignment metric.
    const responsive = await resetProject(page, fixture, timeout);
    if (responsive.some((s) => s.model.generation !== assignment!.extraction.results[0].modelGeneration ||
        s.model.identityHash !== expectedModelIdentity(fixture.model))) throw new Error("responsive tree witness model mismatch");
    await immutable(path.join(directory, "assignment-responsive-tree-witness.json"), responsive);
    environment = await captureChromiumEnvironment(page);
    const configured = bindRequiredChromiumExecutable();
    const args = environment.browserCommandLine?.arguments;
    if (!Array.isArray(args) || typeof args[0] !== "string" || !path.isAbsolute(args[0])) throw new Error("actual browser command line unavailable");
    const executablePath = await realpath(args[0]);
    const executableBytes = await readFile(executablePath);
    if (executablePath !== configured.executablePath || digest(executableBytes) !== configured.sha256 ||
        executableBytes.length !== configured.bytes || environment.browserVersion !== REQUIRED_CHROMIUM_BINDING.browserVersion ||
        environment.browserProtocolVersion?.revision !== REQUIRED_CHROMIUM_BINDING.revision ||
        environment.browserProtocolVersion?.product !== REQUIRED_CHROMIUM_RUNTIME_BINDING.product) throw new Error("actual browser pin/source mismatch");
    environment.actualExecutable = { path: executablePath, bytes: executableBytes.length, sha256: digest(executableBytes) };
    await immutable(path.join(directory, "chromium-environment.json"), environment);
    if (!options.assignmentOnly) {
    const pointSamples = options.focused ? fixture.samples.point_selection.filter((s: any) => s.sample === 2) : fixture.samples.point_selection;
    for (const sample of pointSamples) {
      const segment = await action("point-selection", sample.sample, async (token, dir, stopAction) => {
        await resetProject(page, fixture, timeout);
        await ensureViewportToggle(page, "toggle-viewport-labels", false, timeout);
        await cameraRecipe(page, fixture, timeout);
        const before = (await readCandidateDiagnostics(page, true)).snapshot;
        const probe = fixture.pointOracle.probes.find((p: any) => p.sample === sample.sample);
        const expectedRef = probe?.candidate_runtime?.oracle?.expectedHitRef;
        await immutable(path.join(dir, "point-before-state.json"), { before, probe, sample, expectedRef });
        if (!expectedRef) throw new Error("prescribed candidate point has no independent exact hit");
        const projection = await projectCandidateAuthoredPoint(page, { modelGeneration: before.model.generation,
          cameraSequence: before.viewport.camera.sequence,
          authoredPoint: { x: probe.authored_anchor[0], y: probe.authored_anchor[1], z: probe.authored_anchor[2] } });
        const canvas = await requireUniqueConnectedMainCanvas(page);
        const local = projection.canvasCssPoint;
        if (!local || projection.status !== "available" || projection.insideClosedNdc !== true || projection.insideCanvasCss !== true ||
            Math.abs(projection.canvasCss.width - canvas.box.width) > 1e-6 || Math.abs(projection.canvasCss.height - canvas.box.height) > 1e-6) throw new Error("prescribed point not actionable");
        const point = canvasLocalToClient(local, canvas.box);
        const visual = fixture.pointOracle.candidate_preflight?.selection_visual_oracle;
        const plan = probe.candidate_runtime.visual_plan; validateWinnerCuePlan(plan, probe);
        const beforeImage = path.join(dir, "highlight-before.png"), afterImage = path.join(dir, "highlight-after.png");
        const beforeCapture = await captureWinnerCue(page, probe, beforeImage);
        await immutable(path.join(dir, "point-before.json"), { before, beforeCapture, plan, expectedRef, probe });
        await boundary(`point-selection-${sample.sample}-ready`);
        await armCausalFeedbackMarker(page, { token, phase: "candidate", feedbackKind: "point-selection", actionStartEvent: "pointerdown",
          expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
          actionIdentity: { sample: sample.sample, expectedRef, expectedCssPoint: point, expectedCanvasLocalPoint: local, expectedClientPoint: point },
          candidateExpectation: { ...modelExpectation(before), priorActionSequence: before.viewport.selection.actionSequence, expectedRef,
            expectedInspectorHeading: `${expectedVisibleEntityLabel(fixture.model, expectedRef)} — ${expectedRef.type}: ${expectedRef.id}` } });
        assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, point));
        await page.mouse.click(point.x, point.y, { button: "left" });
        await delay(FULL_COHORT_RECIPE.settleMs);
        const stopped = await stopAction();
        const after = (await readCandidateDiagnostics(page, true)).snapshot;
        await immutable(path.join(dir, "point-stopped.json"), { before, after, expectedRef, plan, stopped });
        const afterCapture = await captureWinnerCue(page, probe, afterImage);
        const verdict = pairedWinnerCueWitness(await readFile(beforeImage), await readFile(afterImage), plan, probe, visual);
        const refs = after.viewport.selection.orderedRefs;
        const exact = refs.length === 1 && refs[0].type === expectedRef.type && refs[0].id === expectedRef.id &&
          after.viewport.selection.primaryRef?.id === expectedRef.id && after.viewport.selection.primaryRef?.type === expectedRef.type &&
          after.viewport.inspector.ref?.id === expectedRef.id && after.viewport.inspector.ref?.type === expectedRef.type;
        const markerBound = cueActionBindingsStable(beforeCapture, afterCapture) && stopped.evidence.active.feedbackMarkers.length === 1 && pointCaptureMatchesMarker(stopped.evidence.active.feedbackMarkers[0], afterCapture, expectedRef);
        await immutable(path.join(dir, "pixel-witness.json"), { beforeCapture, afterCapture, verdict, visual, expectedRef, plan, exact, markerBound });
        if (!exact || !markerBound || verdict.status !== "PASS_PAIRED_WINNER_CUE_TRANSITION") throw new Error("winner cue transition/typed capture binding failed");
        return { ...stopped, expectedRef, modelGeneration: after.model.generation, productObservationRaf: after.viewport.mainRender.nextPaintOpportunity };
      });
      points.push(metric(segment, sample.sample));
    }
    const boxSamples = options.focused ? focusedBoxSamples(fixture) : fixture.samples.box_selection;
    for (const sample of boxSamples) {
      const segment = await action("box-selection", sample.sample, async (token, dir, stopAction) => {
        await resetProject(page, fixture, timeout);
        await cameraRecipe(page, fixture, timeout);
        const oracle = fixture.pointOracle.candidate_box_selection;
        const box = oracle?.samples?.find((s: any) => s.sample === sample.sample);
        if (oracle?.box_selection_policy_sha256 !== "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310" ||
            !box || box.filter !== sample.filter || !Array.isArray(box.orderedRefs)) throw new Error("box oracle mismatch");
        await page.getByTestId("viewport-selection-filter").selectOption({ label: sample.filter === "all" ? "All" :
          ({ pipes: "Pipes", nodes: "Nodes", supports: "Supports", components: "Components" } as any)[sample.filter] }, { timeout });
        await ensureViewportToggle(page, "viewport-box-select", true, timeout);
        const before = (await readCandidateDiagnostics(page, true)).snapshot;
        const canvas = await requireUniqueConnectedMainCanvas(page);
        const start = normalizedCanvasPoint(canvas.box, { x: sample.start_normalized[0], y: sample.start_normalized[1] });
        const end = normalizedCanvasPoint(canvas.box, { x: sample.end_normalized[0], y: sample.end_normalized[1] });
        await immutable(path.join(dir, "box-before.json"), { before, expected: box, sample, start, end, canvas,
          oracleSha256: fixture.pointOracleSha256 });
        const priorBox = validatedPriorBoxBaseline(before);
        const resourceGeneration = before.viewport.mainRender.selectionPresentation?.resourceGeneration;
        if (!Number.isSafeInteger(resourceGeneration) || resourceGeneration <= 0) throw new Error("Box rendered resource binding unavailable");
        const expectedInspectorHeading = expectedBoxInspectorHeading(fixture.model, box.orderedRefs, box.primaryRef, fixture.model.project.id);
        await boundary(`box-selection-${sample.sample}-ready`);
        await armCausalFeedbackMarker(page, { token, phase: "candidate", feedbackKind: "box-selection", actionStartEvent: "pointerup",
          expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
          actionIdentity: { sample: sample.sample, start, end, direction: box.direction, filter: box.filter, oracleSha256: fixture.pointOracleSha256 },
          candidateExpectation: { ...modelExpectation(before), priorBoxActionSequence: priorBox.actionSequence, priorBoxBaseline: priorBox, resourceGeneration, expectedInspectorHeading,
            direction: box.direction, filter: box.filter, orderedRefs: box.orderedRefs, primaryRef: box.primaryRef,
            oracleSha256: fixture.pointOracleSha256 } });
        assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, start));
        assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, end));
        await page.mouse.move(start.x, start.y); await page.mouse.down({ button: "left" });
        await page.mouse.move(end.x, end.y, { steps: 8 }); await page.mouse.up({ button: "left" });
        await delay(FULL_COHORT_RECIPE.settleMs);
        const stopped = await stopAction();
        const after = (await readCandidateDiagnostics(page, true)).snapshot;
        await persistBoxPostcondition(path.join(dir, "box-stopped.json"), { before, after, expected: box, prior: priorBox,
          priorRender: before.viewport.mainRender.submissionSequence, projectId: fixture.model.project.id,
          pointerEvents: { pointerdown: stopped.evidence.active.pointerTransaction.down, pointerup: stopped.evidence.active.pointerTransaction.up },
          actionIdentity: stopped.evidence.active.actionMarker?.actionIdentity ?? { sample, start, end }, canvas,
          oracleSha256: fixture.pointOracleSha256, resourceGeneration, expectedInspectorHeading, dom: await readBoxVisibleIdentity(page) });
        const domBeforeCapture = await readBoxVisibleIdentity(page);
        await page.screenshot({ path: path.join(dir, "appearance.png") });
        const domAfterCapture = await readBoxVisibleIdentity(page);
        const captured = (await readCandidateDiagnostics(page, true)).snapshot;
        const identity = (snapshot: any) => ({ model: snapshot.model, camera: snapshot.viewport.camera,
          selection: snapshot.viewport.selection, box: snapshot.viewport.box, mainRender: snapshot.viewport.mainRender });
        const marker = stopped.evidence.active.feedbackMarkers[0];
        const noDrift = stopped.evidence.active.feedbackMarkers.length === 1 &&
          boxCallbackMatchesSnapshot(marker, after) &&
          JSON.stringify(identity(after)) === JSON.stringify(identity(captured)) && JSON.stringify(domBeforeCapture) === JSON.stringify(domAfterCapture);
        await immutable(path.join(dir, "box-capture-binding.json"), { marker, beforeCapture: after, afterCapture: captured, domBeforeCapture, domAfterCapture, noDrift });
        if (!noDrift) throw new Error("Box capture drifted from qualified callback");
        return { ...stopped, productObservationRaf: after.viewport.mainRender.nextPaintOpportunity };
      });
      boxes.push(metric(segment, sample.sample));
    }
    const filterSamples = options.focused ? [fixture.samples.tree_filters[0], { sample: 0, query: "__UIF_FOCUSED_EMPTY_NO_ENTITY__" }] : fixture.samples.tree_filters;
    for (const sample of filterSamples) {
      const segment = await action("tree-filter", sample.sample, async (token, dir, stopAction) => {
        const input = page.getByTestId("model-tree-filter-input");
        await input.fill("", { timeout });
        await delay(FULL_COHORT_RECIPE.settleMs);
        const tree = page.getByTestId("model-tree-virtual");
        await tree.focus({ timeout }); await tree.press("Home", { timeout });
        const before = (await readCandidateDiagnostics(page, true)).snapshot;
        const initial = frozenTreeExpectation(fixture.model, "");
        if (before.viewport.filter.query !== "" || before.viewport.filter.visibleCount !== initial.totalCount) throw new Error("filter reset did not restore full inventory");
        if (!(await page.locator(".panel.model-tree").isVisible())) throw new Error("tree is not visible under frozen layout");
        const layoutSignature = await page.locator(".panel.model-tree").evaluate((root) => {
          const ancestors = []; for (let e: Element | null = root; e; e = e.parentElement) ancestors.push(e);
          return ancestors.map((e) => `${e.tagName}|${e.className}|${e.getAttribute("style") ?? ""}`);
        });
        if (characterization) await prepareFilterInput(page, timeout);
        await boundary(`tree-filter-${sample.sample}-ready`);
        await armCausalFeedbackMarker(page, { token, phase: "candidate", feedbackKind: "tree-filter", actionStartEvent: "input",
          expectedActionTargetTestId: "model-tree-filter-input", maximumFeedbackMarkers: 4096,
          actionIdentity: { sample: sample.sample, query: sample.query, fixtureSha256: expected.bindings.fixtureSha256, stimulus: characterization ? FILTER_STIMULUS : "historical-playwright-fill" },
          treeExpectation: { ...frozenTreeExpectation(fixture.model, sample.query), layoutSignature },
          candidateExpectation: { ...modelExpectation(before), priorActionSequence: before.viewport.filter.actionSequence } });
        if (characterization) await insertFilterQuery(page, sample.query);
        else await input.fill(sample.query, { timeout });
        await delay(FULL_COHORT_RECIPE.settleMs);
        const stopped = await stopAction();
        await page.screenshot({ path: path.join(dir, "appearance.png") });
        return { ...stopped, productObservationRaf: stopped.evidence.active.contentProof.snapshot.viewport.mainRender.nextPaintOpportunity };
      });
      filters.push(metric(segment, sample.sample));
    }
    await page.getByTestId("model-tree-filter-input").fill("", { timeout });
    await delay(FULL_COHORT_RECIPE.settleMs);
    for (const mode of ["centerline", "actual-od"] as const) {
      const segment = await action("orbit", mode === "centerline" ? 1 : 2, async (token, dir, stopAction) => {
        await ensureViewportToggle(page, "toggle-viewport-labels", true, timeout);
        const cold = (await readCandidateDiagnostics(page, true)).snapshot;
        if (mode === "actual-od" && cold.viewport.geometry.odGeneration !== 0) throw new Error("OD cold first-conversion precondition lost");
        await page.getByTestId(mode === "centerline" ? "viewport-geometry-schematic" : "viewport-geometry-actual-od").click({ timeout });
        await page.waitForFunction(({ mode }) => {
          const s = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
          return s?.viewport?.geometry?.mode === (mode === "centerline" ? "schematic" : "actual-od") &&
            (mode === "centerline" || (s.viewport.geometry.odStatus === "available" && s.viewport.geometry.odGeneration > 0));
        }, { mode }, { timeout, polling: 50 });
        await cameraRecipe(page, fixture, timeout);
        const before = (await readCandidateDiagnostics(page, true)).snapshot;
        const canvas = await requireUniqueConnectedMainCanvas(page), start = normalizedCanvasPoint(canvas.box, ORBIT_START_NORMALIZED);
        await boundary(`orbit-${mode === "centerline" ? 1 : 2}-ready`);
        await armCausalFeedbackMarker(page, { token, phase: "candidate", feedbackKind: "orbit", actionStartEvent: "pointerdown",
          expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: FULL_COHORT_RECIPE.maximumFeedbackMarkers,
          actionIdentity: { gesture: "frozen-orbit-pointer-path", mode, startNormalized: ORBIT_START_NORMALIZED,
            startClient: start, warmupMs: 2000, measuredMs: 10000 },
          candidateExpectation: { ...modelExpectation(before), priorCameraSequence: before.viewport.camera.sequence } });
        assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, start));
        await page.mouse.move(start.x, start.y); await page.mouse.down({ button: "left" });
        const started = nodePerformance.now(); let moves = 0;
        while (nodePerformance.now() - started < 12250) {
          const t = nodePerformance.now() - started;
          await page.mouse.move(canvas.box.x + canvas.box.width * (0.5 + 0.31 * Math.sin(2 * Math.PI * t / 2400)),
            canvas.box.y + canvas.box.height * (0.5 + 0.23 * Math.sin(2 * Math.PI * t / 1700 + Math.PI / 3)));
          moves++; await delay(Math.max(1, 16 - (moves % 5 === 0 ? 1 : 0)));
        }
        await page.mouse.up({ button: "left" }); await delay(FULL_COHORT_RECIPE.trailingMs);
        const stopped = await stopAction();
        const after = (await readCandidateDiagnostics(page, true)).snapshot;
        if (after.model.generation !== before.model.generation || after.viewport.geometry.mode !== before.viewport.geometry.mode ||
            (mode === "actual-od" && (after.viewport.geometry.odStatus !== "available" || after.viewport.geometry.odGeneration !== before.viewport.geometry.odGeneration))) throw new Error("orbit model/geometry changed");
        return { ...stopped, mode, moves, conversion: { before: cold.viewport.geometry, ready: before.viewport.geometry, after: after.viewport.geometry },
          productObservationRaf: after.viewport.mainRender.nextPaintOpportunity };
      });
      orbitResults[mode] = constructOrbitEvidence(segment.extraction.presentations, segment.window.start, mode, segment.orbitDurationBasis!);
    }
    }
    const settleStart = nodePerformance.now();
    do {
      const current = (await readCandidateDiagnostics(page, true)).snapshot;
      settled = { status: "PASS_OBSERVED_SETTLED", ownedRafCount: current.viewport.resources.ownedPendingRafCount,
        modelGeneration: current.model.generation, readAt: current.capturedAt };
      if (settled.ownedRafCount === 0) break;
      await delay(100);
    } while (nodePerformance.now() - settleStart < 5000);
    await immutable(path.join(directory, "settled-owned-raf.json"), settled);
  } catch (error) { errors.push({ phase: "run", error: String(error) }); }
  finally {
    try { restore = await restoreCausalFeedbackInstrumentation(page); }
    catch (error) { errors.push({ phase: "restore", error: String(error) }); }
    try { await immutable(path.join(directory, "restoration.json"), { restore, errors }); }
    catch (error) { errors.push({ phase: "restore-persist", error: String(error) }); }
    if (characterization) {
      try {
        const postProfile = await bindReferenceProfile();
        if (!profile || JSON.stringify(postProfile) !== JSON.stringify(profile)) throw new Error("reference profile drift");
        const postDisplay = await captureDisplayProfile(directory, "after", postProfile.record.display, display?.identity);
        await immutable(path.join(directory, "reference-profile-after.json"), { profile: postProfile, display: postDisplay });
      } catch (error) { errors.push({ phase: "profile-exit", error: String(error) }); }
    }
  }
  const observers = [];
  for (const segment of segments) {
    try {
    const bytes = await readFile(segment.stoppedPath);
    if (digest(bytes) !== segment.stoppedSha256) throw new Error("persisted stopped evidence drift");
    const stopped = JSON.parse(bytes.toString());
    const productRaf = segment.kind === "assignment" ? stopped.active.contentProof.snapshot.viewport.mainRender.nextPaintOpportunity
      : segment.auxiliary.productObservationRaf;
    const observer = observerValidity(stopped.active.observerSamples, segment.window, stopped.active.overflow.total,
      stopped.active.observerErrors.length, segmentObserverRequirements(stopped, restore?.status, productRaf));
    observers.push({ token: segment.token, observer });
    if (observer.status !== "PASS_OBSERVER_STRUCTURAL_EVIDENCE") errors.push({ phase: `${segment.token}:observer`, error: JSON.stringify(observer.structuralFailures) });
    } catch (error) { errors.push({ phase: `${segment.token}:observer-read`, error: String(error) }); }
  }
  try { await immutable(path.join(directory, "observer-accounting.json"), observers); }
  catch (error) { errors.push({ phase: "observer-accounting-persist", error: String(error) }); }
  // Binding verification precedes publication and scoring so a failed post-run
  // hash cannot leave an apparently qualified result for cohort aggregation.
  try {
    const post = await performanceExpectation(fixture, options.binding, expected);
    if (JSON.stringify(post.bindings) !== JSON.stringify(expected.bindings)) throw new Error("pre/post binding mismatch");
    await immutable(path.join(directory, "post-binding.json"), { status: "PASS", bindings: post.bindings });
  } catch (error) {
    errors.push({ phase: "post-binding", error: String(error) });
    await immutable(path.join(directory, "post-binding.json"), { status: "FAIL", error: String(error) });
  }
  if (options.assignmentOnly) {
    const result = { ...assignmentDiagnosticResult(expected, assignment ? metric(assignment,1) : undefined, errors, segments.length), environment,
      trueCost: "UNPROVED_NO_UNINSTRUMENTED_HEADROOM_CLAIM" };
    await immutable(path.join(directory,"assignment-diagnostic-result.json"),result);
    if(result.status!=="PASS_ASSIGNMENT_COLLECTION_DIAGNOSTIC")throw new Error("assignment collection diagnostic failed; no cohort contribution");
    return result;
  }
  const evidence: PerformanceRun = { ...expected, freshSession: true, qualification: errors.length ? "FAIL" as any : "PASS_QUALIFIED_RUN",
    assignment: assignment ? metric(assignment, 1) : undefined as any, points, boxes, filters,
    centerline: orbitResults.centerline!, actualOd: orbitResults["actual-od"]!,
    settled: settled ?? { status: "FAIL" as any, ownedRafCount: Number.NaN }, observerDiagnostics: { evidenceRef: "observer-accounting.json", status: "RAW_REFERENCES_DIAGNOSTIC_TRUE_COST_UNPROVED" } };
  const scored = scorePerformanceRun(evidence, expected);
  if (options.focused) {
    const allowedIncomplete = ["INCOMPLETE_OR_INVALID_POINTS", "INCOMPLETE_OR_INVALID_BOXES", "INCOMPLETE_OR_INVALID_FILTERS"];
    const unexpected = scored.validityFailures.filter((failure) => !allowedIncomplete.includes(failure));
    if (unexpected.length) errors.push({ phase: "focused-structural-scoring", error: JSON.stringify(unexpected) });
    if (segments.length !== 8) errors.push({ phase: "focused-coverage", error: `expected 8 complete segments, got ${segments.length}` });
  }
  const collection = runCollectionDisposition(mode, scored, errors, segments.length);
  const result = { collection, metadataReferences: boundaries, status: options.focused ? errors.length ? "FAIL_FOCUSED_METHOD" : "PASS_FOCUSED_METHOD_ZERO_COHORT" : scored.status,
    expected, evidence, scored, errors, segmentCount: segments.length, environment,
    separateObservations: { startup: "UNAVAILABLE_NATIVE_STARTUP_NOT_MEASURED", loading: "ORIGINAL_ASSIGNMENT_METRIC_PLUS_HOST_PREPARATION_IN_INITIALIZED_JSON",
      processMemory: "UNAVAILABLE_NOT_MEASURED", heap: "UNAVAILABLE_NOT_MEASURED", resources: "BOUNDARY_DIAGNOSTIC_COUNTS_ONLY_NOT_FULL_RESOURCE_PROOF" },
    resourceNativeAcceptance: "SEPARATELY_OWNED_NOT_CLAIMED", trueCost: "UNPROVED_NO_UNINSTRUMENTED_HEADROOM_CLAIM" };
  await immutable(path.join(directory, "result.json"), result);
  if (errors.length || (!options.focused && collection.attemptDisposition === "ABORT_REMAINING")) throw new Error(`candidate causal run failed: ${JSON.stringify({ errors, scored })}`);
  return result;
}
