// Way H's after-worker harness (DEL-04-05, `src-tauri/src/after_worker_perf_tests.rs`): the web's
// own code judges the native replies one harness run kept. The harness's Rust checks are only
// structural pre-checks; a run is web-accepted only by `judgeRun` passing over its replies.
//
// What it applies, in the web's order (`previewService.ts`: `readNativeFreshResult`,
// `validateNativeReaders`, `nativeStandingInputs`, `nativeEnvelopeRefusal`; `workspaceSession.ts`:
// the session's gates, its input manifest and its model-reference check; `NativeResultsPanel.tsx`:
// the first page and the semantics' recorded-table agreement):
//   header        decodeNativeRouteHeader(parsed, jobId)          nativeResult.ts
//   registration  decodeNativeValidation(parsed)                  nativeResult.ts
//   standing      decodeNativeVerdict(parsed)                     nativeResult.ts
//   summary       decodeResultSummary(bytes)                      resultSummary.ts
//   agreement     requireRouteHeaderAgreement(header, verdict)    nativeResult.ts
//   recovery      decodeSourceRecovery(bytes)                     resultDetail.ts
//   run_preview   decodeAnalysisRunPreview(bytes)                 analysisRunPreview.ts
//   rows_page     decodeRowsPage(bytes)                           resultDetail.ts
//   row_semantics decodeRowSemantics(text, ordinals)              resultDetail.ts
//   a rejection   nativeInvokeError(parsed)                       memoryPool.ts (nativeIngress.ts)
// and the connecting checks between replies: every request names the run's job; each request's
// arguments are the ones the web would send given the replies before it (the reader its route
// selects, the model texts the web's own `checkedJsonText` and `jsonTextKeepingNegativeZero`
// make, the standing S is read with, the manifest's model and solver basis, the rows filter and
// page, the semantics' ordinals); each conditional call is present exactly when the web would make
// it; the route header agrees with the final verdict; run_preview names the job and route. The
// run record's checksums are compared with a full build only on the C2 path
// (in `workspaceSession.ts`), not in this sequence, so not here.
//
// The session's gates are applied too (workspaceSession.ts: gates 1 and 2 before run_preview, the
// model reference before it commits): the registration (`hasNativeMechanicsInvocation`), a current source contract
// and a producer, read with the web's own `hasCurrentSourceContract` on a handle the web's own
// `createNativeResult` builds; then the model reference. A run whose sequence those gates would
// stop is judged to end there; a run that kept replies past a gate fails.
//
// Web predicates are imported wherever they are exported, among them `nativeStandingInputs` and
// `buildCurrentSessionInputManifest` (whose `validateArgs` runs as the web runs it; its canonical
// bytes and digest come from the wasm engine, built by `npm run build:wasm`). Restated, each with
// its reason and its source compared line by line (the family table in `tools/after_worker_family`,
// checked against the source by its generator): `nativeEnvelopeRefusal` (not exported; its retained predicates are
// imported), the reading cache's `keyOf` (not exported), the registration gate
// (`nativeRegistrationHolds` reads module state a reply does not carry), and the session's
// sequencing (inline in `readNativeFreshResult` and the session).
//
// Each reply is accepted in the kind it was delivered in (`acceptReply`), with the web's guard on
// that kind for that call: the verdict decoders' own (imported); the byte-response adapter
// `bytesInvoke` (restated: not exported) and, for S, `readNativeSummary`'s text refusal (restated:
// inline in a function that sends natively); and the exported adapters `readAnalysisRunPreview`,
// `queryResultRows`, `queryResultSourceRecovery` and `readResultRowSemantics`, imported and handed
// the kept reply in place of a native call, so their own guards, decoders and requests apply.
//
// The standing rule (Agent 0):
// A predicate of the session's acceptance is IMPORTED where it is exported. Where it is RESTATED,
// it has a row in the generated table with its source passage, found by its text, and a digest of
// that source text; a change to that text fails the check and names the row whose restatement must
// be revalidated. Any
// change to the judge, or to a web source it imports or restates, re-runs the check. The check
// gives traceability; it does not show that the judge equals the web, or that the table is
// complete.
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { createNativeResult, decodeNativeRouteHeader, decodeNativeValidation, decodeNativeVerdict, NATIVE_SUMMARY_COMMAND, nativeCustody, requireRouteHeaderAgreement, type NativeRouteHeader } from "../../services/nativeResult";
import { decodeResultSummary, type ResultSummary } from "../../services/resultSummary";
import { readAnalysisRunPreview, type AnalysisRunPreview } from "../../services/analysisRunPreview";
import { panelDetailFilter, queryResultRows, queryResultSourceRecovery, readResultRowSemantics, type DetailFilter, type RowsPage, type SourceRecovery } from "../../services/resultDetail";
import { nativeInvokeError, ResourceUnavailableError } from "../../services/memoryPool";
import { checkedJsonText } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { nativeStandingInputs } from "../../services/previewService";
import { NO_RULE_PACK_RESOLUTION } from "../../services/modelRulePackResolution";
import { sameModelData } from "../../features/results/resultStanding";
import { isRetainedIdentity, isRetainedRoute, PRESSURE_SOURCE_CONTRACT_ID } from "../../features/results/numericalResultQuality";
import { hasCurrentSourceContract, type ReaderDispatch } from "../../features/results/resultReader";
import { errorCode } from "../../services/solveTimeline";
import { DEFAULT_RESULT_PAGE_SIZE } from "../../features/results/ResultsPanel";

export type ReplyEntry = { seq: number; call: string; file: string; kind: "json" | "bin" | "txt" | "error"; bytes: number; args_file: string | null };
export type ReplyManifest = { schema: string; head: string | null; job_id: string; extracted_from_record?: string; entries: ReplyEntry[] };
/** `calls`: the judge's steps (native calls, cache answers, checks, gates); `native`: the native
 * calls alone, in order, which are exactly the replies the run kept. */
export type RunVerdict = { calls: string[]; native: string[]; ended: string; refusal: { kind: string; required_bytes?: number; budget_bytes?: number } | null };

/** The session's solver mode (`resultsSessionState.ts`, its `useState` default). */
const SOLVER_MODE = "sparse_interactive";
/** The web's decoder trees (the services and the result readers they import). A run from another
 * head is judged only if these are unchanged since it; this judge's own connecting checks are the
 * current code, applied to every run. */
const DECODER_PATHS = ["apps/desktop/src/services", "apps/desktop/src/features/results"];

function fail(reason: string): never {
  throw new Error(`AFTER-WORKER-REPLY-REFUSED: ${reason}`);
}
const text = (bytes: Uint8Array) => new TextDecoder("utf-8", { fatal: true }).decode(bytes);
// Request arguments are compared as values (`sameModelData`: the same members, each `Object.is`):
// the harness writes them in serde_json's member order where the web writes `JSON.stringify`'s,
// and the native ingress decodes them by member name (`command_ingress::admit_raw`), so the order
// never reaches a command.
const same = (a: unknown, b: unknown) => sameModelData(a, b);

/** The panel's first-page filter (`NativeResultsPanel.tsx`, its default filters). */
const FIRST_PAGE_FILTER = panelDetailFilter({ filterText: "", familyFilter: "all", frame: "all", caseKey: "all" });
type ManifestEvidence = Pick<CurrentSessionInputManifestEvidence, "manifest_ref" | "manifest_sha256" | "canonical_bytes">;
export type ReplyContext = { jobId: string; ordinals?: readonly number[]; inputManifest?: ManifestEvidence; filter?: DetailFilter };
export type Accepted = { value: unknown; sent: { command: string; args: Record<string, unknown> } | null };

/** A kept reply as the web's caller receives it, by the harness's delivery kind: `json`, the parsed
 * JSON value Tauri delivers; `bin`, bytes (an ArrayBuffer view); `txt`, a string. */
function delivery(kind: ReplyEntry["kind"], bytes: Uint8Array): unknown {
  if (kind === "json") {
    try {
      return JSON.parse(text(bytes));
    } catch {
      return fail("a json delivery that is not JSON");
    }
  }
  if (kind === "txt") return text(bytes);
  if (kind === "bin") return bytes;
  return fail(`a ${kind} delivery is not a reply`);
}

/** `bytesInvoke` (nativeResult.ts), restated: not exported. The byte-response commands'
 * adapter passes bytes and a string through and refuses any other delivery. */
function bytesDelivery(command: string, delivered: unknown): ArrayBuffer | Uint8Array | string {
  if (delivered instanceof ArrayBuffer || ArrayBuffer.isView(delivered)) return delivered as ArrayBuffer | Uint8Array;
  if (Object.prototype.toString.call(delivered) === "[object ArrayBuffer]") return delivered as ArrayBuffer;
  if (typeof delivered === "string") return delivered;
  throw new Error(`NATIVE-RESULT-RESPONSE-KIND: ${command} delivered a ${delivered === null ? "null" : typeof delivered}, not bytes`);
}

/** The web's acceptance of `call`'s reply delivered as `kind`: the call's own guard on the kind,
 * then its decoder. The byte-response calls go through `bytesInvoke` into the web's own adapter,
 * which is handed the kept reply in place of the native call; the request it sends is returned. */
export async function acceptReply(call: string, kind: ReplyEntry["kind"], bytes: Uint8Array, context: ReplyContext): Promise<Accepted> {
  const delivered = delivery(kind, bytes);
  const capture: { sent: Accepted["sent"] } = { sent: null };
  const invoke = async (command: string, args: Record<string, unknown>) => {
    capture.sent = { command, args };
    return bytesDelivery(command, delivered);
  };
  switch (call) {
    // The verdict command's deliveries: the decoders' own guards refuse anything but a parsed object.
    case "header": return { value: decodeNativeRouteHeader(delivered, context.jobId), sent: null };
    case "registration": return { value: decodeNativeValidation(delivered), sent: null };
    case "standing": case "standing_again": case "dispatch_again": return { value: decodeNativeVerdict(delivered), sent: null };
    case "summary": {
      const response = bytesDelivery(NATIVE_SUMMARY_COMMAND, delivered);
      // `readNativeSummary` (nativeResult.ts), restated: inline in a function that sends natively.
      if (typeof response === "string") throw new Error("RESULT-SUMMARY-MALFORMED: a text response where bytes were expected");
      return { value: decodeResultSummary(response), sent: null };
    }
    case "source_recovery": return { value: await queryResultSourceRecovery(invoke as never, context.jobId, 0, 1), sent: capture.sent };
    case "run_preview": return { value: await readAnalysisRunPreview(invoke as never, context.jobId, { inputManifest: context.inputManifest ?? fail("run_preview needs its manifest"), rulePackExclusions: NO_RULE_PACK_RESOLUTION.excluded }), sent: capture.sent };
    case "rows_page": return { value: await queryResultRows(invoke as never, context.jobId, context.filter ?? fail("the rows need their filter"), 0, DEFAULT_RESULT_PAGE_SIZE), sent: capture.sent };
    case "row_semantics": return { value: await readResultRowSemantics(invoke as never, context.jobId, context.ordinals ?? fail("the semantics need their ordinals")), sent: capture.sent };
    default: return fail(`a reply of an unknown call: ${call}`);
  }
}

/** The kind the harness keeps each call's reply in (after_worker_perf_tests.rs, its `Reply` kinds). */
const KEPT_KIND: Readonly<Record<string, ReplyEntry["kind"]>> = { header: "json", registration: "json", standing: "json", standing_again: "json", dispatch_again: "json", summary: "bin", source_recovery: "bin", run_preview: "bin", rows_page: "bin", row_semantics: "txt" };
/** A reply's acceptance in the kind the harness keeps it in (the maintained wiring test checks each). */
export function decoderFor(call: string): (bytes: Uint8Array, context: ReplyContext) => Promise<unknown> {
  const kind = Object.hasOwn(KEPT_KIND, call) ? KEPT_KIND[call] : fail(`a reply of an unknown call: ${call}`);
  const manifest: ManifestEvidence = { manifest_ref: { object_type: "InputManifest", ref: "wiring" }, manifest_sha256: "0".repeat(64), canonical_bytes: "{}" };
  return async (bytes, context) => (await acceptReply(call, kind, bytes, { filter: FIRST_PAGE_FILTER, inputManifest: manifest, ...context })).value;
}

/** `nativeEnvelopeRefusal` (previewService.ts), restated: it is not exported. Its
 * `capture` is never null here (a fresh solve's capture exists), so its first line does not apply;
 * the retained predicates are the web's own. */
function envelopeRefusal(envelope: Record<string, unknown>, route: string, capturedModel: { project: { id: string } }, callerModel: unknown): string | null {
  if (envelope.model_ref !== capturedModel.project.id) return "CAPTURE_MISMATCH";
  const producer = envelope.producer as { semantic_contract_id?: string } | undefined;
  if (route !== "pressure_source" && producer?.semantic_contract_id === PRESSURE_SOURCE_CONTRACT_ID) {
    if (!sameModelData(JSON.parse(checkedJsonText(callerModel)), capturedModel)) return "CALLER_DISPATCH_NORMALIZATION_MISMATCH";
    return route === "unsupported" ? "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" : "SOURCE_BLOCKS_CONTRACT";
  }
  if (isRetainedRoute(route as never) || isRetainedIdentity(producer?.semantic_contract_id)) return "RETAINED_PRECISION_NATIVE_REGISTRATION_UNAVAILABLE";
  return null;
}

/** That `directory`'s replies were produced at a head whose decoders are this checkout's. */
export function checkRevision(manifest: ReplyManifest, repository: string): string {
  const head = execFileSync("git", ["-C", repository, "rev-parse", "HEAD"], { encoding: "utf8" }).trim();
  if (!manifest.head) fail("the replies name no head");
  if (manifest.head !== head) {
    try {
      execFileSync("git", ["-C", repository, "diff", "--quiet", manifest.head, "HEAD", "--", ...DECODER_PATHS]);
    } catch {
      fail(`the replies come from ${manifest.head}, whose decoders differ from this checkout's ${head}`);
    }
  }
  return head;
}

/** Judges one run directory's replies by the web's code, in order. Throws on any refusal. */
export async function judgeRun(directory: string): Promise<RunVerdict> {
  const manifestPath = path.join(directory, "replies.json");
  if (!existsSync(manifestPath)) fail(`${directory} has no replies.json`);
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as ReplyManifest;
  if (manifest.schema !== "swbpipe.after_worker_harness.replies/1") fail("not a replies manifest");
  const jobId = manifest.job_id;
  const entries = [...manifest.entries];
  if (entries.some((entry, i) => entry.seq !== i + 1)) fail("the replies are not numbered in call order");
  const calls: string[] = [];
  const native: string[] = [];
  const read = (entry: ReplyEntry) => {
    // A missing or truncated file fails the run; nothing is skipped.
    const buffer = readFileSync(path.join(directory, entry.file));
    if (buffer.byteLength !== entry.bytes) fail(`${entry.file}: ${buffer.byteLength} bytes, the harness recorded ${entry.bytes}`);
    return new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
  };
  const argsOf = (entry: ReplyEntry): Record<string, unknown> | null => {
    if (entry.args_file === null) {
      if (!manifest.extracted_from_record) fail(`${entry.file} has no request`);
      return null;
    }
    const args = JSON.parse(readFileSync(path.join(directory, entry.args_file), "utf8")) as Record<string, unknown>;
    if (args.jobId !== jobId) fail(`${entry.call}'s request names job ${String(args.jobId)}, not ${jobId}`);
    return args;
  };
  /** The next reply, which must be `call`'s; a rejection is read as the web reads it. */
  const next = (call: string): { entry: ReplyEntry; args: Record<string, unknown> | null; bytes: Uint8Array } | { refused: RunVerdict } | { rejected: unknown; entry: ReplyEntry } => {
    const entry = entries.shift() ?? fail(`the web would call ${call} next; the run kept no reply for it`);
    if (entry.call !== call) fail(`the web would call ${call} next; the run called ${entry.call}`);
    const args = argsOf(entry);
    const bytes = read(entry);
    calls.push(call);
    native.push(call);
    if (entry.kind === "error") {
      const error = nativeInvokeError(JSON.parse(text(bytes)));
      if (error instanceof ResourceUnavailableError) {
        if (entries.length) fail(`the web ends at ${call}'s resource refusal; the run kept ${entries.length} more replies`);
        return { refused: { calls, native, ended: `${call}: resource refusal`, refusal: { ...error.refusal } as RunVerdict["refusal"] } };
      }
      return { rejected: error, entry };
    }
    return { entry, args, bytes };
  };
  const ended = (verdict: RunVerdict) => verdict;
  type Kept = { entry: ReplyEntry; args: Record<string, unknown> | null; bytes: Uint8Array };
  /** `call`'s kept reply accepted as the web accepts it in its delivered kind, refused at that call;
   * a request the web's own adapter sends must be the kept one. */
  const accept = async <T>(call: string, reply: Kept, context: Omit<ReplyContext, "jobId"> = {}): Promise<T> => {
    let accepted: Accepted;
    try {
      accepted = await acceptReply(call, reply.entry.kind, reply.bytes, { jobId, ...context });
    } catch (error) {
      throw new Error(`AFTER-WORKER-REPLY-REFUSED: the web refuses ${call}'s ${reply.entry.kind} reply after ${native.join(" > ")}: ${(error as Error)?.message ?? String(error)}`);
    }
    if (accepted.sent && reply.args && !same(reply.args, accepted.sent.args)) fail(`${call}'s request is not the one the web's adapter sends`);
    return accepted.value as T;
  };

  // header
  const h = next("header");
  if ("refused" in h) return ended(h.refused);
  if ("rejected" in h) fail(`the header was rejected: ${String(h.rejected)}`);
  if (h.args && h.args.operation !== "route_header") fail("the header request is not route_header");
  const header = await accept<NativeRouteHeader>("header", h);
  const route = header.route as string;

  // registration (`validateNativeReaders`): its reader by route, with the web's own model texts.
  const reader = route === "pressure_source" ? "pressure-source" : route === "preview_physics" ? "preview-physics" : route === "pressure" ? "pressure" : null;
  // The session model: the standing's text is `jsonTextKeepingNegativeZero(solveModel)`, whose parse is that model.
  const standingEntry = manifest.entries.find((entry) => entry.call === "standing");
  const standingArgs = standingEntry && standingEntry.args_file ? JSON.parse(readFileSync(path.join(directory, standingEntry.args_file), "utf8")) as Record<string, unknown> : null;
  const S = standingArgs ? String(standingArgs.modelJson) : null;
  const solveModel = S === null ? null : JSON.parse(S);
  const invocation = solveModel === null ? null : JSON.parse(checkedJsonText({ request: { model: solveModel, materials: [] }, solver_mode: SOLVER_MODE }));
  // The fresh capture `captureNativeInvocation` made at the start (previewService.ts): the
  // dispatched invocation and the caller's model. Its fingerprints are read by no predicate this
  // judge applies (`nativeStandingInputs` reads `invalidated`, the invocation and the caller's model).
  const capture = solveModel === null ? null : ({ invocation, fingerprint: "", callerModel: solveModel, callerFingerprint: "", invalidated: false, terminalClaimed: true } as NonNullable<Parameters<typeof nativeStandingInputs>[1]>);
  let readers: string | null = null;
  // validateNativeReaders, its normalization pre-check: on the pressure-source route a normalization between the caller's
  // model and the dispatched one registers nothing, and no reader is called.
  const normalized = reader === "pressure-source" && solveModel !== null && !sameModelData(JSON.parse(checkedJsonText(solveModel)), invocation.request.model);
  if (normalized) readers = "CALLER_DISPATCH_NORMALIZATION_MISMATCH";
  if (reader && !normalized) {
    const r = next("registration");
    if ("refused" in r) return ended(r.refused);
    if ("rejected" in r) {
      // validateNativeReaders, its catch: `errorCode(error) ?? "UNCODED_REFUSAL"` (the web's errorCode).
      readers = errorCode(r.rejected) ?? "UNCODED_REFUSAL";
    } else {
      if (r.args) {
        if (invocation === null) fail("the run kept no standing request to take the session model from");
        if (r.args.reader !== reader) fail(`the ${route} route's registration reads ${reader}, not ${String(r.args.reader)}`);
        if (reader === "pressure-source") {
          if (r.args.invocationJson !== jsonTextKeepingNegativeZero(invocation)) fail("the registration's invocation text is not the web's");
        } else if (r.args.modelJson !== jsonTextKeepingNegativeZero(invocation.request.model)) fail("the registration's model text is not the web's (checkedJsonText, then jsonTextKeepingNegativeZero)");
      }
      const outcome = await accept<{ ok: boolean; error?: { code: string } }>("registration", r);
      // validateNativeReaders: a refusal counts only when its code is not empty (`if (code)`).
      readers = outcome.ok || !outcome.error!.code ? null : outcome.error!.code;
    }
  }

  // standing and summary, as `readNativeFreshResult` reads them (previewService.ts), with the
  // web's own `nativeStandingInputs` on the fresh capture, the session model being both
  // the standing's and the summary's (workspaceSession.ts). The web's per-job reading cache
  // (`cached`, nativeResult.ts) answers a reading whose key it has read before, and then
  // nothing is sent natively: `readNativeVerdict`'s key is keyOf("dispatch", modelText,
  // invocationText), `readNativeSummary`'s keyOf("summary", modelText,
  // JSON.stringify(standing)). `keyOf` is not exported: restated.
  const keyOf = (...parts: readonly (string | null | undefined)[]) => parts.map((part) => part ?? "\u0001").join("\u0000");
  const readings = new Map<string, unknown>();
  const sessionModel = () => solveModel ?? fail("the run kept no standing request to take the session model from");
  const readStanding = async (call: string, registered: boolean): Promise<ReaderDispatch> => {
    const inputs = nativeStandingInputs(route, capture, registered, sessionModel());
    const key = keyOf("dispatch", inputs.modelText, inputs.invocationText);
    if (readings.has(key)) {
      calls.push(`${call}: the web's cache`);
      return readings.get(key) as ReaderDispatch;
    }
    const s = next(call);
    if ("refused" in s) throw s;
    if ("rejected" in s) fail(`${call} was rejected: ${String(s.rejected)}`);
    if (s.args && (s.args.modelJson !== inputs.modelText || s.args.invocationJson !== (inputs.invocationText ?? undefined))) fail(`${call}'s request is not nativeStandingInputs'`);
    const reading = await accept<ReaderDispatch>(call, s);
    readings.set(key, reading);
    return reading;
  };
  const readSummary = async (verdict: ReaderDispatch): Promise<ResultSummary> => {
    const modelText = jsonTextKeepingNegativeZero(sessionModel());
    const standing = { eligible: verdict.standing.eligible, findings: [...verdict.standing.findings] };
    const key = keyOf("summary", modelText, JSON.stringify(standing));
    if (readings.has(key)) {
      calls.push("summary: the web's cache");
      return readings.get(key) as ResultSummary;
    }
    const s = next("summary");
    if ("refused" in s) throw s;
    if ("rejected" in s) fail(`the summary was rejected: ${String(s.rejected)}`);
    if (s.args && (s.args.modelJson !== modelText || !same(s.args.standing, standing))) fail("the summary's request is not the standing's model text and standing");
    const reading = await accept<ResultSummary>("summary", s);
    readings.set(key, reading);
    return reading;
  };
  let verdict: ReaderDispatch;
  let summary: ResultSummary;
  let unregistered: string | null = null;
  let finalInvocationText: string | undefined;
  try {
    verdict = await readStanding("standing", readers === null);
    summary = await readSummary(verdict);
    unregistered = readers ?? envelopeRefusal(summary.header.envelope, route, invocation.request.model, solveModel);
    if (unregistered !== null && readers === null) {
      verdict = await readStanding("standing_again", false);
      summary = await readSummary(verdict);
    }
    // The dispatch with the final inputs (`readNativeFreshResult`, previewService.ts).
    finalInvocationText = nativeStandingInputs(route, capture, unregistered === null, sessionModel()).invocationText ?? undefined;
    verdict = await readStanding("dispatch_again", unregistered === null);
  } catch (thrown) {
    if (thrown && typeof thrown === "object" && "refused" in thrown) return ended((thrown as { refused: RunVerdict }).refused);
    throw thrown;
  }
  requireRouteHeaderAgreement(header, verdict);
  calls.push("agreement");

  // source recovery, only where the web reads it
  let recovery: SourceRecovery | null = null;
  if (route === "pressure_source" || summary.header.omitted_members.includes("source_block_recovery")) {
    const c = next("source_recovery");
    if ("refused" in c) return ended(c.refused);
    if ("rejected" in c) fail(`the source recovery was rejected: ${String(c.rejected)}`);
    if (c.args && (c.args.offset !== 0 || c.args.limit !== 1 || c.args.raw !== false)) fail("the source recovery's request is not the web's");
    recovery = await accept<SourceRecovery>("source_recovery", c);
  }

  // The session's gates (workspaceSession.ts), on the handle `readNativeFreshResult`
  // builds (previewService.ts), with the web's own `createNativeResult` and
  // `hasCurrentSourceContract`. `hasNativeMechanicsInvocation` for a native handle is
  // `nativeRegistrationHolds`, which needs the handle in `nativeRegistrations`, set
  // only when nothing refused the registration (in `readNativeFreshResult`); its other conditions (the job live, the
  // capture not invalidated, the solver mode, the captured fingerprints) are the session's own
  // in-memory state for a fresh solve and are carried by no reply. `resultBytes` is not read by
  // either gate.
  const handle = createNativeResult({
    jobId, origin: "fresh_job", resultBytes: 0, summary, verdict, recovery,
    binding: { standingModelText: String(S), invocationText: finalInvocationText ?? null, summaryModelText: String(S), standing: { eligible: verdict.standing.eligible, findings: [...verdict.standing.findings] } },
  });
  const gate = (code: string): RunVerdict => {
    if (entries.length) fail(`the session stops at ${code} after ${native.join(" > ")}; the run kept ${entries.length} more replies`);
    calls.push(`session gate ${code}`);
    return { calls, native, ended: `session gate: ${code}`, refusal: null };
  };
  if (unregistered !== null) return gate("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
  if (!hasCurrentSourceContract(handle) || !handle.producer) return gate("SOLVE-PRODUCER-CONTRACT-UNSUPPORTED");

  // The session's input manifest (workspaceSession.ts), built by the web's own
  // `buildCurrentSessionInputManifest` (inputManifestService.ts), whose `validateArgs` runs
  // first, with the arguments the session passes: the solve model, the handle's producer, the
  // session's solver mode and settings, and NO_RULE_PACK_RESOLUTION for a model without rule-pack
  // references (this judge resolves no rule-pack store). A builder refusal stops the session.
  const model = sessionModel();
  if ((model.rule_pack_refs ?? []).length !== 0) fail("this judge judges a model without rule-pack references (NO_RULE_PACK_RESOLUTION) only");
  const producer = handle.producer!;
  let built: CurrentSessionInputManifestEvidence;
  try {
    built = await buildCurrentSessionInputManifest({
      model,
      solver: {
        solver_name: producer.component_name, solver_version: producer.component_version,
        solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: SOLVER_MODE,
        settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: SOLVER_MODE === "sparse_interactive" },
      },
      active_rule_packs: NO_RULE_PACK_RESOLUTION.active_rule_packs,
      external_assets: [],
    });
  } catch (error) {
    if (String((error as Error)?.message ?? "").startsWith("WASM-ENGINE-ASSET-ABSENT")) throw error;
    return gate(String((error as Error)?.message ?? error).match(/^([A-Z][A-Z0-9_-]*)(?=[:\s]|$)/)?.[1] ?? "UNCODED_MANIFEST_REFUSAL");
  }
  // workspaceSession.ts, the standing-model text check: the manifest's model text is the standing's bound text.
  if (jsonTextKeepingNegativeZero(built.manifest.model_basis.model_payload) !== nativeCustody(handle).binding.standingModelText) return gate("SOLVE-NATIVE-STANDING-MODEL-MISMATCH");

  // run_preview, with that manifest's evidence (`readAnalysisRunPreview`, analysisRunPreview.ts)
  const p = next("run_preview");
  if ("refused" in p) return ended(p.refused);
  if ("rejected" in p) fail(`run_preview was rejected: ${String(p.rejected)}`);
  if (p.args) {
    const sent = p.args.inputManifest as { manifest_ref: unknown; manifest_sha256: string; canonical_bytes: string };
    if (sent.canonical_bytes !== built.canonical_bytes) fail("the manifest's canonical bytes are not buildCurrentSessionInputManifest's");
    if (sent.manifest_sha256 !== built.manifest_sha256) fail("the manifest's SHA-256 is not buildCurrentSessionInputManifest's");
    if (!same(sent.manifest_ref, built.manifest_ref)) fail("the manifest's ref is not buildCurrentSessionInputManifest's");
    if (!sameModelData(JSON.parse(sent.canonical_bytes), JSON.parse(checkedJsonText(built.manifest)))) fail("the manifest is not buildCurrentSessionInputManifest's");
    if (p.args.ruleCheckAggregate !== null || !same(p.args.rulePackExclusions, NO_RULE_PACK_RESOLUTION.excluded)) fail("the rule-pack arguments are not NO_RULE_PACK_RESOLUTION's");
  }
  const preview = await accept<AnalysisRunPreview>("run_preview", p, { inputManifest: built });
  if (preview.job_id !== jobId || preview.route !== route) fail("run_preview names another job or route");
  // The session's model-reference check before it commits (workspaceSession.ts).
  if (handle.model_ref !== solveModel.project.id) return gate("SOLVE-RESULT-MODEL-MISMATCH");

  // the panel's first page and its row semantics
  const rowsReply = next("rows_page");
  if ("refused" in rowsReply) return ended(rowsReply.refused);
  if ("rejected" in rowsReply) fail(`the rows page was rejected: ${String(rowsReply.rejected)}`);
  const filter = FIRST_PAGE_FILTER;
  if (rowsReply.args && (!same(rowsReply.args.filter, filter) || rowsReply.args.offset !== 0 || rowsReply.args.limit !== DEFAULT_RESULT_PAGE_SIZE)) fail("the rows request is not the panel's first page");
  const page = await accept<RowsPage>("rows_page", rowsReply, { filter });
  if (page.rows.length === 0 && page.total > 0) fail("the panel would read the last page; the run did not");
  if (page.rows.length > 0) {
    const ordinals = page.rows.map((row) => row.ordinal);
    const sem = next("row_semantics");
    if ("refused" in sem) return ended(sem.refused);
    if ("rejected" in sem) fail(`the row semantics were rejected: ${String(sem.rejected)}`);
    if (sem.args && !same(sem.args.ordinals, ordinals)) fail("the semantics' ordinals are not the page's");
    await accept("row_semantics", sem, { ordinals });
  }
  if (entries.length) fail(`the web stops at the first page; the run kept ${entries.length} more replies`);
  return { calls, native, ended: "first_page_ready", refusal: null };
}
