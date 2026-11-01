// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md` §5; DEL-08-04 `csv-export.md` §4): the streamed
// CSV save's webview side. Every chunk is screened on DOTH-CSV-011 (local-private, explicit intent,
// exact payload) and sent in order with its own evidence; long parts are split, never truncated;
// a refusal before or during the stream aborts it, and its error states the native cleanup; the
// receipt must be the single save's for the requested name and the bytes sent. The native side's
// own checks are `native_csv_stream_tests.rs`.
import { afterEach, describe, expect, it, vi } from "vitest";

// The route screen, observed: each payload it screens is recorded, and one payload can be blocked.
const screened = vi.hoisted(() => ({ payloads: [] as unknown[], block: null as string | null }));
vi.mock("../redaction-controls/redactionExportControls", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../redaction-controls/redactionExportControls")>();
  return {
    ...actual,
    controlRouteExport: (payload: unknown, options: Parameters<typeof actual.controlRouteExport>[1]) => {
      screened.payloads.push(payload);
      const controlled = actual.controlRouteExport(payload, options);
      return payload === screened.block ? { ...controlled, payload: null, blocked: true } : controlled;
    }
  };
});

import { csvChunkEvidence, csvSaveRequest, saveCsvTableStream, splitCsvChunk, CSV_STREAM_CHUNK_UNITS, type CsvStreamInvoke } from "./csvSave";

afterEach(() => { delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; screened.payloads.length = 0; screened.block = null; });

const HEAD = "﻿notice,x\r\ntable,results-displacement\r\n\r\nResult id,Value [mm]\r\n";
const NAME = "swbpipe-results-displacement.csv";
const ID = "0123456789abcdef0123456789abcdef";

async function* parts(...texts: string[]) {
  for (const text of texts) yield text;
}

/** The native stream as `native_csv_stream.rs` answers a well-formed sequence, recording calls. */
function stream(options: { refuse?: (command: string, args: Record<string, unknown>) => unknown } = {}) {
  const calls: { command: string; args: Record<string, unknown> }[] = [];
  let bytes = 0;
  const invoke: CsvStreamInvoke = async (command, args) => {
    calls.push({ command, args });
    const refusal = options.refuse?.(command, args);
    if (refusal) throw refusal;
    if (command === "begin_local_csv_stream") {
      const text = (args.request as { text: string }).text;
      bytes = new TextEncoder().encode(text).length;
      return { outcome: "streaming", stream_id: ID, file_name: NAME, sequence: 1, byte_count: bytes };
    }
    if (command === "append_local_csv_stream") {
      bytes += new TextEncoder().encode((args.chunk as { text: string }).text).length;
      return { outcome: "streaming", sequence: (args.sequence as number) + 1, byte_count: bytes };
    }
    if (command === "finish_local_csv_stream") return { outcome: "saved", file_name: NAME, byte_count: args.byteCount, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
    return { outcome: "aborted", partial_file_name: NAME, cleanup: "removed" };
  };
  return { invoke, calls };
}

const native = () => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; };

describe("the streamed CSV save", () => {
  it("screens and sends every chunk in order, each with its own evidence, and returns the single save's receipt", async () => {
    native();
    const { invoke, calls } = stream();
    const rows = ["a,1\r\n", "\"q,\"\"x\"\"\",-0\r\n", "'=SUM(A1),é ☃\r\n"];
    const outcome = await saveCsvTableStream({ fileName: NAME, jobId: "job-1", chunks: parts(HEAD, ...rows), explicitLocalPrivateIntent: true }, invoke);
    const text = HEAD + rows.join("");
    expect(outcome).toEqual({ mode: "native", receipt: { outcome: "saved", file_name: NAME, byte_count: new TextEncoder().encode(text).length, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" } });
    expect(calls.map((call) => call.command)).toEqual(["begin_local_csv_stream", "append_local_csv_stream", "append_local_csv_stream", "append_local_csv_stream", "finish_local_csv_stream"]);
    expect(calls[0].args).toEqual({ jobId: "job-1", request: { file_name: NAME, text: HEAD, ...csvChunkEvidence(HEAD, true, true) } });
    calls.slice(1, 4).forEach((call, i) => expect(call.args).toEqual({ streamId: ID, sequence: i + 1, chunk: { text: rows[i], ...csvChunkEvidence(rows[i], false, true) } }));
    expect(calls[4].args).toEqual({ streamId: ID, sequence: 4, byteCount: new TextEncoder().encode(text).length });
    // The route screen ran on each chunk sent (and its evidence is a whole save's evidence).
    for (const chunk of [HEAD, ...rows]) expect(screened.payloads).toContain(chunk);
    expect(csvChunkEvidence(HEAD, true, true)).toEqual({ screening: csvSaveRequest({ text: HEAD, fileName: NAME, explicitLocalPrivateIntent: true }).screening, local_first: csvSaveRequest({ text: HEAD, fileName: NAME, explicitLocalPrivateIntent: true }).local_first });
  });

  it("splits a long part at the chunk bound without dividing a surrogate pair, and loses nothing", async () => {
    native();
    const long = `${"x".repeat(CSV_STREAM_CHUNK_UNITS - 1)}😀${"y".repeat(10)}\r\n`;
    expect(splitCsvChunk(long).join("")).toBe(long);
    expect(splitCsvChunk(long)[0].length).toBe(CSV_STREAM_CHUNK_UNITS - 1);
    expect(splitCsvChunk(long).every((part) => part.length <= CSV_STREAM_CHUNK_UNITS)).toBe(true);
    const { invoke, calls } = stream();
    await saveCsvTableStream({ fileName: NAME, jobId: "job-1", chunks: parts(HEAD, long), explicitLocalPrivateIntent: true }, invoke);
    expect(calls.filter((call) => call.command === "append_local_csv_stream").map((call) => (call.args.chunk as { text: string }).text).join("")).toBe(long);
  });

  it("refuses without intent, a non-CSV name, a head without the BOM, or outside the native runtime, before sending anything", async () => {
    const { invoke, calls } = stream();
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD), explicitLocalPrivateIntent: true }, invoke)).rejects.toMatchObject({ code: "NATIVE_RUNTIME_REQUIRED" });
    native();
    await expect(saveCsvTableStream({ fileName: "swbpipe-results.csv", jobId: "j", chunks: parts(HEAD), explicitLocalPrivateIntent: true }, invoke)).rejects.toMatchObject({ code: "INVALID_FILE_NAME" });
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD), explicitLocalPrivateIntent: false }, invoke)).rejects.toMatchObject({ code: "LOCAL_PRIVATE_INTENT_REQUIRED" });
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts("Result id\r\n"), explicitLocalPrivateIntent: true }, invoke)).rejects.toMatchObject({ code: "INVALID_CSV_TEXT" });
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(), explicitLocalPrivateIntent: true }, invoke)).rejects.toMatchObject({ code: "INVALID_CSV_TEXT" });
    expect(calls).toEqual([]);
  });

  it("aborts the stream when a part cannot be produced, and states the native cleanup", async () => {
    native();
    const { invoke, calls } = stream();
    async function* failing() {
      yield HEAD;
      throw new Error("RESULT-DETAIL-RESPONSE-LIMIT: a page could not be read");
    }
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: failing(), explicitLocalPrivateIntent: true }, invoke)).rejects.toThrow("RESULT-DETAIL-RESPONSE-LIMIT");
    expect(calls.map((call) => call.command)).toEqual(["begin_local_csv_stream", "abort_local_csv_stream"]);
    expect(calls[1].args).toEqual({ streamId: ID });
    // A chunk refused by the webview's own screen after begin: the error states the abort's cleanup.
    const blocking = stream();
    screened.block = "blocked\r\n";
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD, "blocked\r\n"), explicitLocalPrivateIntent: true }, blocking.invoke)).rejects.toMatchObject({ code: "REQUEST_EVIDENCE_DENIED", cleanup: "removed", partial_file_name: NAME });
    expect(blocking.calls.map((call) => call.command)).toEqual(["begin_local_csv_stream", "abort_local_csv_stream"]);
  });

  it("states a native refusal as the typed save error, and a receipt that is not the save's is refused", async () => {
    native();
    const refused = stream({ refuse: (command) => command === "append_local_csv_stream" ? { code: "STREAM_SEQUENCE_INVALID", stage: "stream", message: "Local result save did not complete.", partial_file_name: NAME, cleanup: "removed" } : null });
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD, "a\r\n"), explicitLocalPrivateIntent: true }, refused.invoke)).rejects.toEqual({ code: "STREAM_SEQUENCE_INVALID", stage: "stream", message: "Local result save did not complete.", partial_file_name: NAME, cleanup: "removed" });
    // A partial file name outside the requested family is not repeated.
    const foreign = stream({ refuse: (command) => command === "append_local_csv_stream" ? { code: "WRITE_FAILED", stage: "write", message: "x", partial_file_name: "../other.csv", cleanup: "removed" } : null });
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD, "a\r\n"), explicitLocalPrivateIntent: true }, foreign.invoke)).rejects.toMatchObject({ code: "IPC_REJECTED" });
    // A finish receipt for another name or another byte count.
    for (const wrong of [{ file_name: "swbpipe-results-other.csv" }, { byte_count: 1 }]) {
      const base = stream();
      const invoke: CsvStreamInvoke = async (command, args) => {
        const reply = await base.invoke(command, args);
        return command === "finish_local_csv_stream" ? { ...(reply as object), ...wrong } : reply;
      };
      await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD), explicitLocalPrivateIntent: true }, invoke)).rejects.toMatchObject({ code: "INVALID_SAVE_RECEIPT" });
    }
    // A begin receipt without a stream id of the native form.
    const odd: CsvStreamInvoke = async (command) => command === "begin_local_csv_stream" ? { outcome: "streaming", stream_id: "../x", sequence: 1 } : { outcome: "aborted", cleanup: "not_needed", partial_file_name: null };
    await expect(saveCsvTableStream({ fileName: NAME, jobId: "j", chunks: parts(HEAD), explicitLocalPrivateIntent: true }, odd)).rejects.toMatchObject({ code: "INVALID_STREAM_RECEIPT" });
  });
});
