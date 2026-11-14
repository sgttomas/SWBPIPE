// Way H's after-worker harness (DEL-04-05): the web's own code judges the native replies the
// harness kept (`afterWorkerReplies.ts`). Two parts:
// - Maintained (always run): each malformed reply reaches its own decoder and fails it, and a run
//   whose manifest or reply file is missing fails, so a skipped or mis-wired decoder cannot pass.
// - The runs (only when SWBPIPE_AFTER_WORKER_REPLIES names the harness's replies directory): every
//   run subdirectory there is judged; with the variable set, nothing is skipped.
// Run the runs with the after-worker config (no suite setup; the judge's manifest builder loads the
// wasm engine from public/wasm-engine, which `npm run build:wasm --workspace apps/desktop` builds):
//   SWBPIPE_AFTER_WORKER_REPLIES=<dir> npx vitest run --config src/test-support/after-worker/vitest.config.ts
import { createHash } from "node:crypto";
import { mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { checkRevision, decoderFor, judgeRun, type ReplyManifest } from "./afterWorkerReplies";
import { DISPLAY_COLUMNS, SOURCE_OFFSETS, SOURCE_REFS, STRESS_COLUMNS } from "../../services/resultSummary";
import { PHYSICS_RETAINED_CONTRACT_ID, PRESSURE_CONTRACT_ID } from "../../features/results/numericalResultQuality";
import { checkedJsonText } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { inputManifestRef } from "../../services/inputManifestService";
import { panelDetailFilter } from "../../services/resultDetail";
import { DEFAULT_RESULT_PAGE_SIZE } from "../../features/results/ResultsPanel";

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
  it("a run without its manifest, or naming a missing reply, fails", async () => {
    const directory = mkdtempSync(path.join(tmpdir(), "after-worker-"));
    try {
      await expect(judgeRun(directory)).rejects.toThrow(/has no replies.json/);
      const manifest: ReplyManifest = { schema: "swbpipe.after_worker_harness.replies/1", head: null, job_id: job, entries: [{ seq: 1, call: "header", file: "01-header.json", kind: "json", bytes: 10, args_file: "01-header.args.json" }] };
      writeFileSync(path.join(directory, "replies.json"), JSON.stringify(manifest));
      writeFileSync(path.join(directory, "01-header.args.json"), JSON.stringify({ jobId: job, operation: "route_header" }));
      await expect(judgeRun(directory)).rejects.toThrow(/ENOENT/);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

// Connecting controls: synthetic sequences whose replies each pass their decoder, but whose
// connection the session refuses (workspaceSession.ts:1563-1597; previewService.ts:598-608). Each
// control asserts its disposition (the code it ends or fails at) and its native-call sequence
// (the replies the judge reads, in order, where it stops, and the readings the web's cache answers).
describe("the after-worker reply judge applies the session's connecting gates", () => {
  const JOB = "backend-solve-job-9";
  // Complete enough for the manifest builder's model and unit checks (inputManifestService.ts:162-180).
  const MODEL = { schema_version: "0.9.0", document_kind: "openpipestress.model", project: { id: "p", units: { length: "m" } }, load_cases: [] };
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
  async function judged(steps: Step[]) {
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
      return await judgeRun(directory);
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
  const OPENING = ["header", "registration", "standing", "summary"];
  const preview = step("run_preview", { invented: true }, { jobId: JOB });
  const stopsAfter = (code: string, sequence: string[], more: number) => new RegExp(`the session stops at ${code} after ${sequence.join(" > ")}; the run kept ${more} more replies`);

  it("a refused registration that the run carried on past fails at SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", async () => {
    await expect(judged([...opening({ ok: false, error: { code: "X", detail: null } }, envelope(PRODUCER)), preview])).rejects.toThrow(stopsAfter("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", OPENING, 1));
  });
  it("a refused registration whose run stops there is judged to end at the session's gate", async () => {
    const judgedRun = await judged(opening({ ok: false, error: { code: "X", detail: null } }, envelope(PRODUCER)));
    expect(judgedRun.ended).toBe("session gate: SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
    expect(judgedRun.native).toEqual(OPENING);
    expect(judgedRun.calls).toEqual([...OPENING, "dispatch_again: the web's cache", "agreement", "session gate SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED"]);
  });
  it("a retained identity claimed under another route refuses registration; the web's cache answers the standing and S read again", async () => {
    const retained = envelope({ ...PRODUCER, semantic_contract_id: PHYSICS_RETAINED_CONTRACT_ID });
    expect.assertions(6);
    // The reader's expectations (READING-320-third-codex finding 2), awaited: the judge is async.
    await expect(judged([...opening({ ok: true }, retained), preview])).rejects.toThrow(/the session stops at SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED/);
    expect((await judged(opening({ ok: true }, retained))).ended).toBe("session gate: SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
    // Its native-call sequence: the opening's four calls, nothing more.
    await expect(judged([...opening({ ok: true }, retained), preview])).rejects.toThrow(stopsAfter("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", OPENING, 1));
    const judgedRun = await judged(opening({ ok: true }, retained));
    expect(judgedRun.native).toEqual(OPENING);
    expect(judgedRun.calls).toEqual([...OPENING, "standing_again: the web's cache", "summary: the web's cache", "dispatch_again: the web's cache", "agreement", "session gate SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED"]);
    // A run that sent the standing and S again natively is not the web's sequence.
    const again = [step("standing_again", verdict(), standingArgs), step("summary", summary(retained), summaryArgs)];
    await expect(judged([...opening({ ok: true }, retained), ...again])).rejects.toThrow(stopsAfter("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", OPENING, 2));
  });
  it("a result without a current source contract fails at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", async () => {
    const steps = [
      step("header", header(false), { jobId: JOB, operation: "route_header" }),
      step("standing", verdict(false), standingArgs),
      step("summary", summary(envelope(PRODUCER)), summaryArgs),
    ];
    await expect(judged([...steps, preview])).rejects.toThrow(stopsAfter("SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", ["header", "standing", "summary"], 1));
    const judgedRun = await judged(steps);
    expect(judgedRun.ended).toBe("session gate: SOLVE-PRODUCER-CONTRACT-UNSUPPORTED");
    expect(judgedRun.native).toEqual(["header", "standing", "summary"]);
  });
  it("a result naming no producer fails at SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", async () => {
    await expect(judged([...opening({ ok: true }, envelope(undefined)), preview])).rejects.toThrow(stopsAfter("SOLVE-PRODUCER-CONTRACT-UNSUPPORTED", OPENING, 1));
    const judgedRun = await judged(opening({ ok: true }, envelope(undefined)));
    expect(judgedRun.ended).toBe("session gate: SOLVE-PRODUCER-CONTRACT-UNSUPPORTED");
    expect(judgedRun.native).toEqual(OPENING);
  });
  it("a producer with an empty component_name stops the session at the manifest builder's INPUT-MANIFEST-SOLVER-INCOMPLETE", async () => {
    // The manifest such a run would send, with its digest and ref made to agree (finding 1's case):
    // a comparison of the manifest's literal alone accepts it.
    const empty = envelope({ ...PRODUCER, component_name: "" });
    const manifest = {
      schema_version: "1.0.0", document_kind: "openpipestress.current_session_input_manifest",
      model_basis: { model_ref: "p", model_payload: MODEL }, unit_basis: { project_units: MODEL.project.units },
      solver_basis: { solver_name: "", solver_version: "1", solver_build_ref: "@1", solver_mode: "sparse_interactive", settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true } },
      load_basis: { load_cases: [], combinations: [] }, active_rule_packs: [], external_assets: [],
      identity_policy: { canonicalization: "rfc8785_jcs", hash_algorithm: "sha256", canonical_bytes_scope: "entire_input_manifest_object" },
      replay_boundary: { included_as_package_member: false, portable_replay_claimed: false, current_session_ref_hash_integrity_only: true },
    };
    const canonical = checkedJsonText(manifest);
    const sha = createHash("sha256").update(canonical, "utf8").digest("hex");
    const sent = step("run_preview", {
      document_kind: "swbpipe.desktop.analysis_run_preview", version: 1, job_id: JOB, route: "pressure", record_bytes: 1, rule_check_status: "RULE_INPUTS_INCOMPLETE", result_ref_count: 0, producer_diagnostic_count: 0, carried_members: [], rule_pack_ref_diagnostics: [],
      record: { analysis_run: { run_id: "r", analysis_status: [], hashes: [{ payload_scope: "analysis_run_record", value: "a".repeat(64) }, { payload_scope: "received_result", value: "b".repeat(64) }] } },
    }, { jobId: JOB, inputManifest: { manifest_ref: { object_type: "InputManifest", ref: inputManifestRef("p", sha) }, manifest_sha256: sha, canonical_bytes: canonical }, ruleCheckAggregate: null, rulePackExclusions: [] });
    const rows = step("rows_page", framed("SWBDETv1", { document_kind: "swbpipe.desktop.result_detail_rows", version: 1, lengths: [], total: 0, offset: 0, returned: 0, truncated: false, ordinals: [] }, false),
      { jobId: JOB, filter: panelDetailFilter({ filterText: "", familyFilter: "all", frame: "all", caseKey: "all" }), offset: 0, limit: DEFAULT_RESULT_PAGE_SIZE });
    await expect(judged([...opening({ ok: true }, empty), sent, rows])).rejects.toThrow(stopsAfter("INPUT-MANIFEST-SOLVER-INCOMPLETE", OPENING, 2));
    const judgedRun = await judged(opening({ ok: true }, empty));
    expect(judgedRun.ended).toBe("session gate: INPUT-MANIFEST-SOLVER-INCOMPLETE");
    expect(judgedRun.native).toEqual(OPENING);
  });
});

describe.skipIf(!ROOT)("the after-worker harness's kept replies, judged by the web's code", () => {
  const runs = ROOT ? readdirSync(ROOT).filter((name) => statSync(path.join(ROOT, name)).isDirectory()).sort() : [];
  it("names at least one run", () => {
    expect(runs.length).toBeGreaterThan(0);
  });
  for (const run of runs) {
    it(run, async () => {
      const directory = path.join(ROOT!, run);
      // Judged only by the decoders of the revision that produced the replies (or unchanged since).
      const head = checkRevision(JSON.parse(readFileSync(path.join(directory, "replies.json"), "utf8")) as ReplyManifest, REPOSITORY);
      const verdict = await judgeRun(directory);
      console.log(`after-worker replies ${run}: judged by the web's code at ${head}; ${verdict.calls.join(" > ")}; native ${verdict.native.join(" > ")}; ended ${verdict.ended}${verdict.refusal ? ` (the web's typed refusal ${JSON.stringify(verdict.refusal)})` : " (web-accepted)"}`);
    });
  }
});
