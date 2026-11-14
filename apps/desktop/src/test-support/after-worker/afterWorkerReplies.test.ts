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
