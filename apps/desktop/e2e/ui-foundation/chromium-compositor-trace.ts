import { createHash } from "node:crypto";
import { open, readFile } from "node:fs/promises";
import type { CDPSession, Page } from "@playwright/test";
import { parseTraceEventsWithUnsafeIntegersAsDecimalStrings } from "./lossless-trace-json.mjs";

export const CHROMIUM_TRACE_RAW_BYTE_LIMIT = 256 * 1024 * 1024;

export type ChromiumTraceCapture = {
  session: CDPSession;
  markerRecipe: "preflight-orbit" | "discrete-segment";
  availableCategories: string[];
  selectedCategories: string[];
  events: any[];
  bufferUsageSamples: any[];
  markerPrefix: string;
  rawTracePath: string;
  rawTraceBytes: number;
  rawTraceComplete: boolean;
  rawTraceSha256: string | null;
  rawTraceReceivedCompleteChunkBytes: number;
  rawTraceReceivedCompleteChunkSha256: string | null;
  rawTraceFinalSyncError: string | null;
  rawTraceCloseError: string | null;
  rawTraceStreamCloseError: string | null;
  rawTraceInspectionError: string | null;
  unsafeIntegerTokensConvertedToDecimalStrings: number;
};

const CATEGORY_ALLOWLIST = [
  "benchmark",
  "cc",
  "devtools.timeline",
  "viz",
  "disabled-by-default-devtools.timeline.frame"
];

export async function beginChromiumCompositorTrace(
  page: Page,
  markerPrefix: string,
  rawTracePath: string,
  markerRecipe: "preflight-orbit" | "discrete-segment" = "preflight-orbit"
): Promise<ChromiumTraceCapture> {
  const session = await page.context().newCDPSession(page);
  const response = await session.send("Tracing.getCategories");
  const availableCategories = [...response.categories].sort();
  const available = new Set(availableCategories);
  const selectedCategories = CATEGORY_ALLOWLIST.filter((category) => available.has(category));
  for (const required of CATEGORY_ALLOWLIST) {
    if (!selectedCategories.includes(required)) {
      await session.detach();
      throw new Error(`Chromium trace category ${required} is unavailable; compositor timing remains unqualified`);
    }
  }
  const bufferUsageSamples: any[] = [];
  session.on("Tracing.bufferUsage", (payload) => {
    if (bufferUsageSamples.length < 1000) bufferUsageSamples.push(payload);
  });
  await session.send("Tracing.start", {
    categories: selectedCategories.join(","),
    options: "record-as-much-as-possible",
    transferMode: "ReturnAsStream",
    bufferUsageReportingInterval: 250
  });
  await page.evaluate((label) => console.timeStamp(label), `${markerPrefix}:TRACE_STARTED`);
  return {
    session, markerRecipe, availableCategories, selectedCategories, events: [], bufferUsageSamples, markerPrefix,
    rawTracePath, rawTraceBytes: 0, rawTraceComplete: false, rawTraceSha256: null,
    rawTraceReceivedCompleteChunkBytes: 0, rawTraceReceivedCompleteChunkSha256: null,
    rawTraceFinalSyncError: null, rawTraceCloseError: null, rawTraceInspectionError: null,
    rawTraceStreamCloseError: null,
    unsafeIntegerTokensConvertedToDecimalStrings: 0
  };
}

export async function traceMarker(page: Page, label: string): Promise<number> {
  const at = await page.evaluate((value) => {
    console.timeStamp(value);
    return performance.now();
  }, label);
  return at;
}

export async function endChromiumCompositorTrace(page: Page, capture: ChromiumTraceCapture): Promise<any> {
  const endPageClock = await traceMarker(page, `${capture.markerPrefix}:TRACE_END`);
  let tracingCompletePayload: any = null;
  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  const complete = new Promise<void>((resolve) => capture.session.once("Tracing.tracingComplete", (payload) => {
    tracingCompletePayload = payload;
    resolve();
  }));
  const boundedTimeout = new Promise<never>((_, reject) => {
    timeoutId = setTimeout(() => reject(new Error("Tracing.tracingComplete not observed within 30000ms")), 30_000);
  });
  let completionError: string | null = null;
  try {
    await capture.session.send("Tracing.end");
    await Promise.race([complete, boundedTimeout]);
    const stream = tracingCompletePayload?.stream;
    if (typeof stream !== "string" || stream.length === 0) {
      throw new Error("Tracing.tracingComplete did not provide a ReturnAsStream handle");
    }
    const maximumRawBytes = CHROMIUM_TRACE_RAW_BYTE_LIMIT;
    const ioDeadlineAt = Date.now() + 30_000;
    const rawHandle = await open(capture.rawTracePath, "wx");
    const rawDigest = createHash("sha256");
    let transferError: unknown = null;
    let reachedEof = false;
    try {
      for (;;) {
        const remainingMs = ioDeadlineAt - Date.now();
        if (remainingMs <= 0) throw new Error("ReturnAsStream IO deadline exceeded 30000ms");
        let timeoutId: ReturnType<typeof setTimeout> | null = null;
        const chunk = await Promise.race([
          capture.session.send("IO.read", { handle: stream, size: 1_048_576 }),
          new Promise<never>((_, reject) => {
            timeoutId = setTimeout(() => reject(new Error("ReturnAsStream IO.read exceeded bounded deadline")), remainingMs);
          })
        ]).finally(() => { if (timeoutId) clearTimeout(timeoutId); });
        const bytes = Buffer.from(chunk.data, chunk.base64Encoded ? "base64" : "utf8");
        if (capture.rawTraceReceivedCompleteChunkBytes + bytes.length > maximumRawBytes) {
          throw new Error(`ReturnAsStream raw trace exceeds ${maximumRawBytes} byte limit`);
        }
        let written = 0;
        while (written < bytes.length) {
          const result = await rawHandle.write(bytes, written, bytes.length - written);
          if (result.bytesWritten <= 0) throw new Error("ReturnAsStream raw trace write made no progress");
          written += result.bytesWritten;
        }
        await rawHandle.sync();
        rawDigest.update(bytes);
        capture.rawTraceReceivedCompleteChunkBytes += bytes.length;
        if (chunk.eof) {
          reachedEof = true;
          break;
        }
      }
    } catch (error) {
      transferError = error;
    }
    capture.rawTraceReceivedCompleteChunkSha256 = rawDigest.digest("hex");
    try {
      await rawHandle.sync();
    } catch (error) {
      capture.rawTraceFinalSyncError = String(error);
    }
    try {
      await rawHandle.close();
    } catch (error) {
      capture.rawTraceCloseError = String(error);
    }
    let rawBytes: Buffer | null = null;
    try {
      rawBytes = await readFile(capture.rawTracePath);
      capture.rawTraceBytes = rawBytes.length;
      capture.rawTraceSha256 = createHash("sha256").update(rawBytes).digest("hex");
    } catch (error) {
      capture.rawTraceInspectionError = String(error);
    }
    try {
      await capture.session.send("IO.close", { handle: stream });
    } catch (error) {
      capture.rawTraceStreamCloseError = String(error);
    }
    capture.rawTraceComplete = reachedEof && transferError === null &&
      capture.rawTraceFinalSyncError === null && capture.rawTraceCloseError === null &&
      capture.rawTraceStreamCloseError === null && capture.rawTraceInspectionError === null && rawBytes !== null &&
      capture.rawTraceBytes === capture.rawTraceReceivedCompleteChunkBytes &&
      capture.rawTraceSha256 === capture.rawTraceReceivedCompleteChunkSha256;
    if (transferError !== null) throw transferError;
    if (capture.rawTraceFinalSyncError !== null || capture.rawTraceCloseError !== null ||
        capture.rawTraceStreamCloseError !== null ||
        capture.rawTraceInspectionError !== null) {
      throw new Error(`ReturnAsStream preserved-file finalization failed: ${JSON.stringify({
        sync: capture.rawTraceFinalSyncError,
        close: capture.rawTraceCloseError,
        streamClose: capture.rawTraceStreamCloseError,
        inspection: capture.rawTraceInspectionError
      })}`);
    }
    if (!capture.rawTraceComplete || rawBytes === null) {
      throw new Error("durable raw trace does not exactly match all received complete chunks through EOF");
    }
    const lossless = parseTraceEventsWithUnsafeIntegersAsDecimalStrings(rawBytes.toString("utf8"));
    capture.events = lossless.events;
    capture.unsafeIntegerTokensConvertedToDecimalStrings = lossless.convertedCount;
  } catch (error) {
    completionError = String(error);
  } finally {
    if (timeoutId) clearTimeout(timeoutId);
    await capture.session.detach().catch(() => undefined);
  }
  const eventNameCounts = new Map<string, number>();
  const candidateEventCounts = new Map<string, number>();
  const candidatePattern = /EventLatency|CompositorFrameReporter|FramePresented|PipelineReporter|FrameSequenceTracker|DrawAndSwap|SwapBuffers|SubmitCompositorFrame|BeginFrame|^Frame(?:$|\s|\/|-)/i;
  for (const event of capture.events) {
    const name = String(event.name ?? "<unnamed>");
    eventNameCounts.set(name, (eventNameCounts.get(name) ?? 0) + 1);
    if (candidatePattern.test(name)) candidateEventCounts.set(name, (candidateEventCounts.get(name) ?? 0) + 1);
  }
  const candidateEvents = capture.events.filter((event) => candidatePattern.test(String(event.name ?? "")));
  const perNameSampleCount = new Map<string, number>();
  const candidateSamples = candidateEvents.filter((event) => {
    const name = String(event.name ?? "<unnamed>");
    const retained = perNameSampleCount.get(name) ?? 0;
    if (retained >= 20) return false;
    perNameSampleCount.set(name, retained + 1);
    return true;
  }).slice(0, 1000).map((event) => ({
    name: event.name,
    category: event.cat,
    phase: event.ph,
    timestamp: event.ts,
    processId: event.pid,
    threadId: event.tid,
    duration: event.dur ?? null,
    threadTimestamp: event.tts ?? null,
    threadDuration: event.tdur ?? null,
    id: event.id ?? null,
    id2: event.id2 ?? null,
    scope: event.scope ?? null,
    bindId: event.bind_id ?? null,
    flowIn: event.flow_in ?? null,
    flowOut: event.flow_out ?? null,
    args: event.args ?? null
  }));
  const traceMarkers = capture.events.filter((event) => {
    const message = event.args?.data?.message ?? event.args?.message ?? event.args?.name;
    return typeof message === "string" && message.startsWith(`${capture.markerPrefix}:`);
  }).map((event) => ({
    message: event.args?.data?.message ?? event.args?.message ?? event.args?.name,
    name: event.name,
    category: event.cat,
    phase: event.ph,
    timestamp: event.ts,
    processId: event.pid,
    threadId: event.tid
  }));
  const requiredMarkerMessages = capture.markerRecipe === "discrete-segment"
    ? [`${capture.markerPrefix}:TRACE_STARTED`, `${capture.markerPrefix}:TRACE_END`] : [
    `${capture.markerPrefix}:TRACE_STARTED`,
    `${capture.markerPrefix}:ORBIT_WARMUP_START`,
    `${capture.markerPrefix}:MEASURED_START`,
    `${capture.markerPrefix}:MEASURED_END`,
    `${capture.markerPrefix}:TRACE_END`
  ];
  const countByMessage = Object.fromEntries(requiredMarkerMessages.map((message) => [
    message,
    traceMarkers.filter((marker) => marker.message === message).length
  ]));
  const firstTimestampByMessage = Object.fromEntries(requiredMarkerMessages.map((message) => [
    message,
    traceMarkers.find((marker) => marker.message === message)?.timestamp ?? null
  ]));
  const missingMarkers = requiredMarkerMessages.filter((message) => firstTimestampByMessage[message] === null);
  const markerTimestamps = requiredMarkerMessages.map((message) => firstTimestampByMessage[message]).filter((value): value is number => typeof value === "number");
  const exactlyOneEach = requiredMarkerMessages.every((message) => countByMessage[message] === 1);
  const finiteTimestamps = markerTimestamps.length === requiredMarkerMessages.length && markerTimestamps.every(Number.isFinite);
  const ordered = markerTimestamps.every((value, index) => index === 0 || value >= markerTimestamps[index - 1]);
  const markerValidation = {
    status: missingMarkers.length === 0 && exactlyOneEach && finiteTimestamps && ordered
      ? "PASS_REQUIRED_MARKERS_PRESENT_ONCE_WITH_FINITE_ORDERED_TIMESTAMPS"
      : "FAIL_REQUIRED_TRACE_MARKERS",
    requiredMarkerMessages,
    countByMessage,
    firstTimestampByMessage,
    missingMarkers,
    exactlyOneEach,
    finiteTimestamps,
    ordered
  };
  const processThreadMetadata = capture.events.filter((event) =>
    event.ph === "M" && ["process_name", "process_labels", "thread_name", "thread_sort_index"].includes(String(event.name))
  ).map((event) => ({ name: event.name, processId: event.pid, threadId: event.tid, args: event.args }));
  const markerProcessIds = [...new Set(traceMarkers.filter((marker) => requiredMarkerMessages.includes(marker.message)).map((marker) => marker.processId))];
  const appRendererPid = markerProcessIds.length === 1 ? markerProcessIds[0] : null;
  const processName = appRendererPid === null ? null : processThreadMetadata.find((event) =>
    event.processId === appRendererPid && event.name === "process_name"
  )?.args?.name ?? null;
  const measuredStart = firstTimestampByMessage[`${capture.markerPrefix}:MEASURED_START`];
  const measuredEnd = firstTimestampByMessage[`${capture.markerPrefix}:MEASURED_END`];
  const appSurfaceMapping = {
    status: "GENERIC_FLOW_TOKEN_JOIN_DISABLED_FOR_QUALIFICATION",
    markerProcessIds,
    appRendererPid,
    processName,
    measuredWindow: { startTraceTimestamp: measuredStart ?? null, endTraceTimestamp: measuredEnd ?? null },
    qualification: "N/A_EXACT_CAUSAL_EXTRACTOR_MUST_USE_PROCESS_SCOPED_MAIN_FRAME_AND_EXACT_SURFACE_REPORTER_OCCURRENCE"
  };
  return {
    schema: "openpipestress.ui-foundation.chromium-compositor-trace-preflight/v2",
    status: completionError !== null
      ? "FAIL_PREFLIGHT_TRACE_CAPTURE_OR_LOSSLESS_PARSE"
      : markerValidation.status !== "PASS_REQUIRED_MARKERS_PRESENT_ONCE_WITH_FINITE_ORDERED_TIMESTAMPS"
      ? "FAIL_PREFLIGHT_REQUIRED_TRACE_MARKERS"
      : tracingCompletePayload?.dataLossOccurred === true
        ? "FAIL_PREFLIGHT_TRACE_DATA_LOSS"
        : tracingCompletePayload?.dataLossOccurred === false
          ? "PREFLIGHT_EVENT_INVENTORY_ONLY_NO_ORBIT_PASS"
          : "PREFLIGHT_EVENT_INVENTORY_COMPLETENESS_UNKNOWN_NO_ORBIT_PASS",
    markerPrefix: capture.markerPrefix,
    endPageClock,
    availableCategories: capture.availableCategories,
    selectedCategories: capture.selectedCategories,
    tracingComplete: tracingCompletePayload,
    traceCompletionError: completionError,
    traceDataLossOccurred: tracingCompletePayload?.dataLossOccurred ?? null,
    traceCompletenessValidation: completionError === null && tracingCompletePayload?.dataLossOccurred === false
      ? "PASS_TRACING_COMPLETE_AND_NO_REPORTED_DATA_LOSS"
      : "FAIL_OR_UNKNOWN_TRACE_COMPLETENESS_NO_QUALIFICATION",
    rawTraceTransport: {
      transferMode: "ReturnAsStream",
      rawPath: capture.rawTracePath,
      rawUtf8Bytes: capture.rawTraceBytes,
      rawCompleteThroughEof: capture.rawTraceComplete,
      rawByteLimit: CHROMIUM_TRACE_RAW_BYTE_LIMIT,
      ioDeadlineMs: 30_000,
      rawSha256: capture.rawTraceSha256,
      receivedCompleteChunkBytes: capture.rawTraceReceivedCompleteChunkBytes,
      receivedCompleteChunkSha256: capture.rawTraceReceivedCompleteChunkSha256,
      finalSyncError: capture.rawTraceFinalSyncError,
      closeError: capture.rawTraceCloseError,
      streamCloseError: capture.rawTraceStreamCloseError,
      preservedFileInspectionError: capture.rawTraceInspectionError,
      unsafeIntegerTokensConvertedToDecimalStrings: capture.unsafeIntegerTokensConvertedToDecimalStrings,
      joinIdentifierPolicy: "Unsafe integer JSON tokens are retained from raw browser UTF-8 and parsed as exact decimal strings; no Number fallback is allowed. Trace timestamps remain numeric only when represented as safe JSON numbers."
    },
    bufferUsageSamples: capture.bufferUsageSamples,
    eventCount: capture.events.length,
    eventNameCounts: Object.fromEntries([...eventNameCounts].sort(([a], [b]) => a.localeCompare(b))),
    candidateEventCounts: Object.fromEntries([...candidateEventCounts].sort(([a], [b]) => a.localeCompare(b))),
    candidateSamples,
    traceMarkers,
    markerValidation,
    processThreadMetadata,
    appSurfaceMapping,
    limitation: "Event-name presence alone does not identify the app's main renderer/surface or prove presentation. Unknown trace completeness or data loss cannot qualify. Qualification requires marker-window mapping to the relevant surface and actual presented-frame gaps."
  };
}
