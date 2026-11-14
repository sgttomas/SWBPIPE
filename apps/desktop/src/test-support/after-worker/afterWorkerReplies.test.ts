// Way H's after-worker harness (DEL-04-05): the web's own code judges the native replies the
// harness kept (`afterWorkerReplies.ts`). Two parts:
// - Maintained (always run): each malformed reply reaches its own decoder and fails it, and a run
//   whose manifest or reply file is missing fails, so a skipped or mis-wired decoder cannot pass.
// - The runs (only when SWBPIPE_AFTER_WORKER_REPLIES names the harness's replies directory): every
//   run subdirectory there is judged; with the variable set, nothing is skipped.
// Run the runs with the after-worker config (no wasm setup; these decoders use none):
//   SWBPIPE_AFTER_WORKER_REPLIES=<dir> npx vitest run --config src/test-support/after-worker/vitest.config.ts
import { mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { checkRevision, decoderFor, judgeRun, type ReplyManifest } from "./afterWorkerReplies";
import { DISPLAY_COLUMNS, SOURCE_OFFSETS, SOURCE_REFS, STRESS_COLUMNS } from "../../services/resultSummary";
import { PHYSICS_RETAINED_CONTRACT_ID, PRESSURE_CONTRACT_ID } from "../../features/results/numericalResultQuality";
import { checkedJsonText } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";

const ROOT = process.env.SWBPIPE_AFTER_WORKER_REPLIES;
const REPOSITORY = path.resolve(__dirname, "../../../../..");

const utf8 = (value: unknown) => new TextEncoder().encode(typeof value === "string" ? value : JSON.stringify(value));
/** A framed response: magic, the header's length (u64 LE), the header, `pad` zeros, then `tail`. */
function framed(magic: string, header: unknown, pad: boolean, tail: Uint8Array = new Uint8Array()): Uint8Array {
  const head = utf8(header);
  let length = 16 + head.length;
  const padding = pad ? (8 - (length % 8)) % 8 : 0;
  length += padding + tail.length;
  const out = new Uint8Array(length);
  out.set(utf8(magic), 0);
  new DataView(out.buffer).setBigUint64(8, BigInt(head.length), true);
  out.set(head, 16);
  out.set(tail, 16 + head.length + padding);
  return out;
}

describe("the after-worker reply judge reaches the web's decoder for each call", () => {
  const job = "backend-solve-job-1";
  const cases: [string, Uint8Array, RegExp][] = [
    ["header", utf8({ document_kind: "swbpipe.desktop.result_route_header", version: 1, job_id: job, header_status: "recognized", route: "pressure", binding: { id: "another-table", sha256: "0".repeat(64) }, refusal: null }), /NATIVE-VERDICT-MALFORMED/],
    ["header", utf8({ document_kind: "swbpipe.desktop.result_route_header", version: 1, job_id: "another-job", header_status: "refused", route: "unsupported", binding: null, refusal: { code: "X", detail: null } }), /NATIVE-VERDICT-MALFORMED/],
    ["registration", utf8({ ok: false }), /NATIVE-VERDICT-MALFORMED/],
    ["standing", utf8({ ok: true, route: "pressure" }), /NATIVE-VERDICT-MALFORMED/],
    ["standing_again", utf8({ ok: true, route: "pressure", standing: { eligible: "yes", findings: [] } }), /NATIVE-VERDICT-MALFORMED/],
    // A JSON string, as Tauri would deliver a text body parsed: not the dispatch object.
    ["dispatch_again", utf8(JSON.stringify("not an object")), /NATIVE-VERDICT-MALFORMED/],
    // S without its schema planes.
    ["summary", framed("SWBSUMv1", { document_kind: "swbpipe.desktop.result_summary", version: 1, planes: [], planes_bytes: 0 }, true), /RESULT-SUMMARY-MALFORMED/],
    ["summary", framed("SWBSUMvX", {}, true), /RESULT-SUMMARY-MALFORMED/],
    // A preview whose received-result checksum is not a SHA-256.
    ["run_preview", utf8({ document_kind: "swbpipe.desktop.analysis_run_preview", version: 1, record: { analysis_run: { run_id: "r", analysis_status: [], hashes: [{ payload_scope: "analysis_run_record", value: "a".repeat(64) }, { payload_scope: "received_result", value: "not-a-digest" }] } } }), /ANALYSIS-RUN-PREVIEW-MALFORMED/],
    // A rows page whose text runs past the response, and one whose ordinals do not ascend.
    ["rows_page", framed("SWBDETv1", { document_kind: "swbpipe.desktop.result_detail_rows", version: 1, lengths: [100] }, false), /RESULT-DETAIL-MALFORMED/],
    ["rows_page", framed("SWBDETv1", { document_kind: "swbpipe.desktop.result_detail_rows", version: 1, lengths: [2, 2], total: 9, offset: 0, returned: 2, truncated: false, ordinals: [5, 3] }, false, utf8("{}{}")), /RESULT-DETAIL-MALFORMED/],
    ["row_semantics", utf8([{ ok: true }]), /RESULT-DETAIL-MALFORMED/],
    ["source_recovery", framed("SWBDETv1", { document_kind: "swbpipe.desktop.result_detail_rows", version: 1, lengths: [] }, false), /RESULT-DETAIL-MALFORMED/],
  ];
  for (const [call, bytes, code] of cases) {
    it(`${call}: a malformed reply fails ${code.source}`, () => {
      expect(() => decoderFor(call)(bytes, { jobId: job, ordinals: [1, 2] })).toThrow(code);
    });
  }
  it("an unknown call is refused", () => {
    expect(() => decoderFor("lookup")).toThrow(/AFTER-WORKER-REPLY-REFUSED/);
  });
  it("a run without its manifest, or naming a missing reply, fails", () => {
    const directory = mkdtempSync(path.join(tmpdir(), "after-worker-"));
    try {
      expect(() => judgeRun(directory)).toThrow(/has no replies.json/);
      const manifest: ReplyManifest = { schema: "swbpipe.after_worker_harness.replies/1", head: null, job_id: job, entries: [{ seq: 1, call: "header", file: "01-header.json", kind: "json", bytes: 10, args_file: "01-header.args.json" }] };
      writeFileSync(path.join(directory, "replies.json"), JSON.stringify(manifest));
      writeFileSync(path.join(directory, "01-header.args.json"), JSON.stringify({ jobId: job, operation: "route_header" }));
      expect(() => judgeRun(directory)).toThrow(/ENOENT/);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

// Connecting controls: synthetic sequences whose replies each pass their decoder, but whose
// connection the session refuses (workspaceSession.ts:1563-1567; previewService.ts:599-608).
describe("the after-worker reply judge applies the session's connecting gates", () => {
  const JOB = "backend-solve-job-9";
  const MODEL = { project: { id: "p" } };
  const S = jsonTextKeepingNegativeZero(MODEL);
  const R = jsonTextKeepingNegativeZero(JSON.parse(checkedJsonText({ request: { model: MODEL, materials: [] }, solver_mode: "sparse_interactive" })).request.model);
  const BINDING = { id: PRESSURE_CONTRACT_ID, sha256: "a".repeat(64) };
  type Step = { call: string; reply: unknown; args: Record<string, unknown>; kind?: "json" | "bin" };
  const header = (recognized = true) => recognized
    ? { document_kind: "swbpipe.desktop.result_route_header", version: 1, job_id: JOB, header_status: "recognized", route: "pressure", binding: BINDING, refusal: null }
    : { document_kind: "swbpipe.desktop.result_route_header", version: 1, job_id: JOB, header_status: "refused", route: "unsupported", binding: null, refusal: { code: "Y", detail: null } };
  const verdict = (recognized = true) => recognized
    ? { ok: true, route: "pressure", binding: BINDING, error: null, standing: { route: "pressure", eligible: false, findings: [] } }
    : { ok: false, route: "unsupported", binding: null, error: { code: "Y", detail: null }, standing: { route: "unsupported", eligible: false, findings: [] } };
  /** A minimal valid S: no rows, every schema plane, the one source offset. */
  const summary = (envelope: Record<string, unknown>) => {
    const names = [...DISPLAY_COLUMNS, SOURCE_OFFSETS, SOURCE_REFS, ...STRESS_COLUMNS];
    const planes = names.map((name) => ({ name, offset: names.indexOf(name) > names.indexOf(SOURCE_OFFSETS) ? 8 : 0, count: name === SOURCE_OFFSETS ? 1 : 0 }));
    const headerJson = { document_kind: "swbpipe.desktop.result_summary", version: 1, job_id: JOB, envelope, omitted_members: [], row_count: 0, values: [], display: { rows: 0, source_refs: 0 }, stress: { rows: 0 }, tables: { support_load_cases: [], stress_ratio: { groups: [] } }, planes, planes_bytes: 8 };
    return framed("SWBSUMv1", headerJson, true, new Uint8Array(8));
  };
  const envelope = (producer: Record<string, unknown> | undefined) => ({ model_ref: "p", ...(producer ? { producer } : {}) });
  const PRODUCER = { component_name: "invented", component_version: "1", semantic_contract_id: PRESSURE_CONTRACT_ID };
  const standingArgs = { jobId: JOB, modelJson: S };
  const summaryArgs = { jobId: JOB, modelJson: S, standing: { eligible: false, findings: [] } };
  const step = (call: string, reply: unknown, args: Record<string, unknown>): Step => ({ call, reply, args, kind: reply instanceof Uint8Array ? "bin" : "json" });
  function judged(steps: Step[]) {
    const directory = mkdtempSync(path.join(tmpdir(), "after-worker-gate-"));
    try {
      const entries = steps.map((s, i) => {
        const file = `${String(i + 1).padStart(2, "0")}-${s.call}.${s.kind}`;
        const bytes = s.reply instanceof Uint8Array ? s.reply : utf8(s.reply);
        writeFileSync(path.join(directory, file), bytes);
        writeFileSync(path.join(directory, `${file}.args.json`), JSON.stringify(s.args));
        return { seq: i + 1, call: s.call, file, kind: s.kind!, bytes: bytes.length, args_file: `${file}.args.json` };
      });
      writeFileSync(path.join(directory, "replies.json"), JSON.stringify({ schema: "swbpipe.after_worker_harness.replies/1", head: null, job_id: JOB, entries }));
      return judgeRun(directory);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  }
  const opening = (registration: unknown, env: Record<string, unknown>) => [
    step("header", header(), { jobId: JOB, operation: "route_header" }),
    step("registration", registration, { jobId: JOB, reader: "pressure", modelJson: R }),
    step("standing", verdict(), standingArgs),
    step("summary", summary(env), summaryArgs),
  ];
  const preview = step("run_preview", { invented: true }, { jobId: JOB });

  it("a refused registration that the run carried on past fails at SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", () => {
    expect(() => judged([...opening({ ok: false, error: { code: "X", detail: null } }, envelope(PRODUCER)), preview])).toThrow(/the session stops at SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED/);
  });
  it("a refused registration whose run stops there is judged to end at the session's gate", () => {
    expect(judged(opening({ ok: false, error: { code: "X", detail: null } }, envelope(PRODUCER))).ended).toBe("session gate: SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
  });
  it("a retained identity claimed under another route refuses registration, so the web reads the standing again", () => {
    const retained = envelope({ ...PRODUCER, semantic_contract_id: PHYSICS_RETAINED_CONTRACT_ID });
    expect(() => judged([...opening({ ok: true }, retained), preview])).toThrow(/the web would call standing_again next; the run called run_preview/);
    const again = [step("standing_again", verdict(), standingArgs), step("summary", summary(retained), summaryArgs)];
    expect(judged([...opening({ ok: true }, retained), ...again]).ended).toBe("session gate: SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
  });
  it("a result without a current source contract fails at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", () => {
    const steps = [
      step("header", header(false), { jobId: JOB, operation: "route_header" }),
      step("standing", verdict(false), standingArgs),
      step("summary", summary(envelope(PRODUCER)), summaryArgs),
    ];
    expect(() => judged([...steps, preview])).toThrow(/the session stops at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED/);
  });
  it("a result naming no producer fails at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", () => {
    expect(() => judged([...opening({ ok: true }, envelope(undefined)), preview])).toThrow(/the session stops at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED/);
  });
});

describe.skipIf(!ROOT)("the after-worker harness's kept replies, judged by the web's code", () => {
  const runs = ROOT ? readdirSync(ROOT).filter((name) => statSync(path.join(ROOT, name)).isDirectory()).sort() : [];
  it("names at least one run", () => {
    expect(runs.length).toBeGreaterThan(0);
  });
  for (const run of runs) {
    it(run, () => {
      const directory = path.join(ROOT!, run);
      // Judged only by the decoders of the revision that produced the replies (or unchanged since).
      const head = checkRevision(JSON.parse(readFileSync(path.join(directory, "replies.json"), "utf8")) as ReplyManifest, REPOSITORY);
      const verdict = judgeRun(directory);
      console.log(`after-worker replies ${run}: judged by the web's code at ${head}; ${verdict.calls.join(" > ")}; ended ${verdict.ended}${verdict.refusal ? ` (the web's typed refusal ${JSON.stringify(verdict.refusal)})` : " (web-accepted)"}`);
    });
  }
});
