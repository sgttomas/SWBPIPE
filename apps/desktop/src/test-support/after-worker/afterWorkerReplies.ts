// Way H's after-worker harness (DEL-04-05, `src-tauri/src/after_worker_perf_tests.rs`): the web's
// own code judges the native replies one harness run kept. The harness's Rust checks are only
// structural pre-checks; a run is web-accepted only by `judgeRun` passing over its replies.
//
// What it applies, in the web's order (`previewService.ts` `readNativeFreshResult` :617-679,
// `validateNativeReaders` :568-594, `nativeStandingInputs` :554-560, `nativeEnvelopeRefusal`
// :598-608; `workspaceSession.ts` :1563-1606; `NativeResultsPanel.tsx` :202-217):
//   header        decodeNativeRouteHeader(parsed, jobId)          nativeResult.ts:308
//   registration  decodeNativeValidation(parsed)                  nativeResult.ts:289
//   standing      decodeNativeVerdict(parsed)                     nativeResult.ts:280
//   summary       decodeResultSummary(bytes)                      resultSummary.ts:72
//   agreement     requireRouteHeaderAgreement(header, verdict)    nativeResult.ts:322
//   recovery      decodeSourceRecovery(bytes)                     resultDetail.ts:244
//   run_preview   decodeAnalysisRunPreview(bytes)                 analysisRunPreview.ts:78
//   rows_page     decodeRowsPage(bytes)                           resultDetail.ts:161
//   row_semantics decodeRowSemantics(text, ordinals)              resultDetail.ts:212
//   a rejection   nativeInvokeError(parsed)                       memoryPool.ts:237 (nativeIngress.ts:140)
// and the connecting checks between replies: every request names the run's job; each request's
// arguments are the ones the web would send given the replies before it (the reader its route
// selects, the model texts the web's own `checkedJsonText` and `jsonTextKeepingNegativeZero`
// make, the standing S is read with, the manifest's model and solver basis, the rows filter and
// page, the semantics' ordinals); each conditional call is present exactly when the web would make
// it; the route header agrees with the final verdict; run_preview names the job and route. The
// run record's checksums are compared with a full build only on the C2 path
// (`workspaceSession.ts:590`), not in this sequence, so not here.
//
// One copy, stated: `nativeEnvelopeRefusal` is not exported, so `envelopeRefusal` below is its
// logic restated (previewService.ts:598-608).
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { decodeNativeRouteHeader, decodeNativeValidation, decodeNativeVerdict, requireRouteHeaderAgreement, type NativeRouteHeader } from "../../services/nativeResult";
import { decodeResultSummary, type ResultSummary } from "../../services/resultSummary";
import { decodeAnalysisRunPreview, type AnalysisRunPreview } from "../../services/analysisRunPreview";
import { decodeRowSemantics, decodeRowsPage, decodeSourceRecovery, panelDetailFilter } from "../../services/resultDetail";
import { nativeInvokeError, ResourceUnavailableError } from "../../services/memoryPool";
import { checkedJsonText } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { inputManifestRef } from "../../services/inputManifestService";
import { sameModelData } from "../../features/results/resultStanding";
import { PRESSURE_SOURCE_CONTRACT_ID } from "../../features/results/numericalResultQuality";
import type { ReaderDispatch } from "../../features/results/resultReader";

export type ReplyEntry = { seq: number; call: string; file: string; kind: "json" | "bin" | "txt" | "error"; bytes: number; args_file: string | null };
export type ReplyManifest = { schema: string; head: string | null; job_id: string; extracted_from_record?: string; entries: ReplyEntry[] };
export type RunVerdict = { calls: string[]; ended: string; refusal: { kind: string; required_bytes?: number; budget_bytes?: number } | null };

/** `DEFAULT_RESULT_PAGE_SIZE` (`ResultsPanel.tsx:45`, `RESULT_PAGE_SIZE_OPTIONS[0]`). */
const FIRST_PAGE_ROWS = 50;
/** The session's solver mode (`resultsSessionState.ts:170`). */
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

/** A reply's decoder, by the call that produced it (the maintained wiring test checks each). */
export function decoderFor(call: string): (bytes: Uint8Array, context: { jobId: string; ordinals?: readonly number[] }) => unknown {
  switch (call) {
    case "header": return (bytes, { jobId }) => decodeNativeRouteHeader(JSON.parse(text(bytes)), jobId);
    case "registration": return (bytes) => decodeNativeValidation(JSON.parse(text(bytes)));
    case "standing": case "standing_again": case "dispatch_again": return (bytes) => decodeNativeVerdict(JSON.parse(text(bytes)));
    case "summary": return (bytes) => decodeResultSummary(bytes);
    case "source_recovery": return (bytes) => decodeSourceRecovery(bytes);
    case "run_preview": return (bytes) => decodeAnalysisRunPreview(bytes);
    case "rows_page": return (bytes) => decodeRowsPage(bytes);
    case "row_semantics": return (bytes, { ordinals }) => decodeRowSemantics(text(bytes), ordinals ?? fail("the semantics need their ordinals"));
    default: return fail(`a reply of an unknown call: ${call}`);
  }
}

/** `nativeEnvelopeRefusal` (previewService.ts:598-608), restated: it is not exported. */
function envelopeRefusal(envelope: Record<string, unknown>, route: string, capturedModel: { project: { id: string } }, callerModel: unknown): string | null {
  if (envelope.model_ref !== capturedModel.project.id) return "CAPTURE_MISMATCH";
  const producer = envelope.producer as { semantic_contract_id?: string } | undefined;
  if (route !== "pressure_source" && producer?.semantic_contract_id === PRESSURE_SOURCE_CONTRACT_ID) {
    if (!sameModelData(JSON.parse(checkedJsonText(callerModel)), capturedModel)) return "CALLER_DISPATCH_NORMALIZATION_MISMATCH";
    return route === "unsupported" ? "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" : "SOURCE_BLOCKS_CONTRACT";
  }
  if (route === "retained_preview_physics" || route === "retained_physics") return "RETAINED_PRECISION_NATIVE_REGISTRATION_UNAVAILABLE";
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
export function judgeRun(directory: string): RunVerdict {
  const manifestPath = path.join(directory, "replies.json");
  if (!existsSync(manifestPath)) fail(`${directory} has no replies.json`);
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as ReplyManifest;
  if (manifest.schema !== "swbpipe.after_worker_harness.replies/1") fail("not a replies manifest");
  const jobId = manifest.job_id;
  const entries = [...manifest.entries];
  if (entries.some((entry, i) => entry.seq !== i + 1)) fail("the replies are not numbered in call order");
  const calls: string[] = [];
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
    if (entry.kind === "error") {
      const error = nativeInvokeError(JSON.parse(text(bytes)));
      if (error instanceof ResourceUnavailableError) {
        if (entries.length) fail(`the web ends at ${call}'s resource refusal; the run kept ${entries.length} more replies`);
        return { refused: { calls, ended: `${call}: resource refusal`, refusal: { ...error.refusal } as RunVerdict["refusal"] } };
      }
      return { rejected: error, entry };
    }
    return { entry, args, bytes };
  };
  const ended = (verdict: RunVerdict) => verdict;

  // header
  const h = next("header");
  if ("refused" in h) return ended(h.refused);
  if ("rejected" in h) fail(`the header was rejected: ${String(h.rejected)}`);
  if (h.args && h.args.operation !== "route_header") fail("the header request is not route_header");
  const header = decoderFor("header")(h.bytes, { jobId }) as NativeRouteHeader;
  const route = header.route as string;

  // registration (`validateNativeReaders`): its reader by route, with the web's own model texts.
  const reader = route === "pressure_source" ? "pressure-source" : route === "preview_physics" ? "preview-physics" : route === "pressure" ? "pressure" : null;
  // The session model: the standing's text is `jsonTextKeepingNegativeZero(solveModel)`, whose parse is that model.
  const standingEntry = manifest.entries.find((entry) => entry.call === "standing");
  const standingArgs = standingEntry && standingEntry.args_file ? JSON.parse(readFileSync(path.join(directory, standingEntry.args_file), "utf8")) as Record<string, unknown> : null;
  const S = standingArgs ? String(standingArgs.modelJson) : null;
  const solveModel = S === null ? null : JSON.parse(S);
  const invocation = solveModel === null ? null : JSON.parse(checkedJsonText({ request: { model: solveModel, materials: [] }, solver_mode: SOLVER_MODE }));
  let readers: string | null = null;
  if (reader) {
    const r = next("registration");
    if ("refused" in r) return ended(r.refused);
    if ("rejected" in r) {
      readers = (/^([A-Z][A-Z0-9_-]*)(?=[:\s]|$)/.exec(String((r.rejected as { message?: unknown })?.message ?? r.rejected))?.[1]) ?? "UNCODED_REFUSAL";
    } else {
      if (r.args) {
        if (invocation === null) fail("the run kept no standing request to take the session model from");
        if (r.args.reader !== reader) fail(`the ${route} route's registration reads ${reader}, not ${String(r.args.reader)}`);
        if (reader === "pressure-source") {
          if (r.args.invocationJson !== jsonTextKeepingNegativeZero(invocation)) fail("the registration's invocation text is not the web's");
        } else if (r.args.modelJson !== jsonTextKeepingNegativeZero(invocation.request.model)) fail("the registration's model text is not the web's (checkedJsonText, then jsonTextKeepingNegativeZero)");
      }
      const outcome = decoderFor("registration")(r.bytes, { jobId }) as { ok: boolean; error?: { code: string } };
      readers = outcome.ok ? null : outcome.error!.code;
    }
  }

  // standing and summary (`nativeStandingInputs`, `readNativeSummary`)
  const standingInputs = (registered: boolean) => ({ modelJson: S, invocationJson: route === "pressure_source" && registered ? jsonTextKeepingNegativeZero(invocation) : undefined });
  const readStanding = (call: string, registered: boolean): ReaderDispatch => {
    const s = next(call);
    if ("refused" in s) throw s;
    if ("rejected" in s) fail(`${call} was rejected: ${String(s.rejected)}`);
    const expected = standingInputs(registered);
    if (s.args && (s.args.modelJson !== expected.modelJson || s.args.invocationJson !== expected.invocationJson)) fail(`${call}'s request is not nativeStandingInputs'`);
    return decoderFor(call)(s.bytes, { jobId }) as ReaderDispatch;
  };
  const readSummary = (verdict: ReaderDispatch): ResultSummary => {
    const s = next("summary");
    if ("refused" in s) throw s;
    if ("rejected" in s) fail(`the summary was rejected: ${String(s.rejected)}`);
    if (s.args && (s.args.modelJson !== S || !same(s.args.standing, { eligible: verdict.standing.eligible, findings: [...verdict.standing.findings] }))) fail("the summary's request is not the standing's model text and standing");
    return decoderFor("summary")(s.bytes, { jobId }) as ResultSummary;
  };
  let verdict: ReaderDispatch;
  let summary: ResultSummary;
  try {
    let lastRegistered = readers === null;
    verdict = readStanding("standing", lastRegistered);
    summary = readSummary(verdict);
    const unregistered = readers ?? envelopeRefusal(summary.header.envelope, route, invocation.request.model, solveModel);
    if (unregistered !== null && readers === null) {
      lastRegistered = false;
      verdict = readStanding("standing_again", false);
      summary = readSummary(verdict);
    }
    // The dispatch again with the final inputs; the web's cache answers an unchanged key.
    if (standingInputs(lastRegistered).invocationJson !== standingInputs(unregistered === null).invocationJson) verdict = readStanding("dispatch_again", unregistered === null);
  } catch (thrown) {
    if (thrown && typeof thrown === "object" && "refused" in thrown) return ended((thrown as { refused: RunVerdict }).refused);
    throw thrown;
  }
  requireRouteHeaderAgreement(header, verdict);
  calls.push("agreement");

  // source recovery, only where the web reads it
  if (route === "pressure_source" || summary.header.omitted_members.includes("source_block_recovery")) {
    const c = next("source_recovery");
    if ("refused" in c) return ended(c.refused);
    if ("rejected" in c) fail(`the source recovery was rejected: ${String(c.rejected)}`);
    if (c.args && (c.args.offset !== 0 || c.args.limit !== 1 || c.args.raw !== false)) fail("the source recovery's request is not the web's");
    decoderFor("source_recovery")(c.bytes, { jobId });
  }

  // run_preview, with the session's input manifest (`buildCurrentSessionInputManifest`)
  const p = next("run_preview");
  if ("refused" in p) return ended(p.refused);
  if ("rejected" in p) fail(`run_preview was rejected: ${String(p.rejected)}`);
  if (p.args) {
    const evidence = p.args.inputManifest as { manifest_ref: { object_type: string; ref: string }; manifest_sha256: string; canonical_bytes: string };
    const manifest = JSON.parse(evidence.canonical_bytes);
    const sha = createHash("sha256").update(evidence.canonical_bytes, "utf8").digest("hex");
    if (sha !== evidence.manifest_sha256) fail("the manifest's SHA-256 is not its canonical bytes'");
    if (evidence.manifest_ref.object_type !== "InputManifest" || evidence.manifest_ref.ref !== inputManifestRef(manifest.model_basis.model_ref, sha)) fail("the manifest's ref is not inputManifestRef's");
    if (manifest.model_basis.model_ref !== solveModel.project.id || !sameModelData(manifest.model_basis.model_payload, JSON.parse(checkedJsonText(solveModel)))) fail("the manifest's model is not the session model");
    const producer = summary.header.envelope.producer as { component_name: string; component_version: string };
    const solver = manifest.solver_basis;
    if (solver.solver_name !== producer.component_name || solver.solver_version !== producer.component_version || solver.solver_build_ref !== `${producer.component_name}@${producer.component_version}` || solver.solver_mode !== SOLVER_MODE) fail("the manifest's solver basis is not the result's producer and the session's mode");
    if ((solveModel.rule_pack_refs ?? []).length !== 0 || p.args.ruleCheckAggregate !== null || !same(p.args.rulePackExclusions, [])) fail("the rule-pack arguments are not NO_RULE_PACK_RESOLUTION's");
  }
  const preview = decoderFor("run_preview")(p.bytes, { jobId }) as AnalysisRunPreview;
  if (preview.job_id !== jobId || preview.route !== route) fail("run_preview names another job or route");

  // the panel's first page and its row semantics
  const rowsReply = next("rows_page");
  if ("refused" in rowsReply) return ended(rowsReply.refused);
  if ("rejected" in rowsReply) fail(`the rows page was rejected: ${String(rowsReply.rejected)}`);
  const filter = panelDetailFilter({ filterText: "", familyFilter: "all", frame: "all", caseKey: "all" });
  if (rowsReply.args && (!same(rowsReply.args.filter, filter) || rowsReply.args.offset !== 0 || rowsReply.args.limit !== FIRST_PAGE_ROWS)) fail("the rows request is not the panel's first page");
  const page = decoderFor("rows_page")(rowsReply.bytes, { jobId }) as ReturnType<typeof decodeRowsPage>;
  if (page.rows.length === 0 && page.total > 0) fail("the panel would read the last page; the run did not");
  if (page.rows.length > 0) {
    const ordinals = page.rows.map((row) => row.ordinal);
    const sem = next("row_semantics");
    if ("refused" in sem) return ended(sem.refused);
    if ("rejected" in sem) fail(`the row semantics were rejected: ${String(sem.rejected)}`);
    if (sem.args && !same(sem.args.ordinals, ordinals)) fail("the semantics' ordinals are not the page's");
    decoderFor("row_semantics")(sem.bytes, { jobId, ordinals });
  }
  if (entries.length) fail(`the web stops at the first page; the run kept ${entries.length} more replies`);
  return { calls, ended: "first_page_ready", refusal: null };
}
