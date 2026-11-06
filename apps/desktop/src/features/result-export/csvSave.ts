// RT-R6 CSV save (DEL-08-04-R14; Design `csv-export.md` §4). The one save function the
// table export controls call. The CSV text is screened on the governed local-first route
// `DOTH-CSV-011`, admitted only for `local_private` output with explicit user intent. In the
// desktop app it is written by the native save command (`save_local_result_json`, CSV
// profile); in the browser runtime it keeps the anchor download. The CSV is not a released
// report and is not shareable.
import { controlRouteExport, CSV_TABLE_ROUTE_ID } from "../redaction-controls/redactionExportControls";
import { CSV_BOM, isCsvExportFileName } from "./csvTable";
import {
  CSV_DATA_PREFIX,
  isNativeResultSaveRuntime,
  normalizeError,
  receipt as isSaveReceipt,
  saveNativeResultJson,
  type NativeResultSaveError,
  type NativeResultSaveReceipt,
  type NativeResultSaveRequest
} from "./nativeResultSave";
import { invokeWithIngress, type NativeIngressCommand } from "../../services/nativeIngress";

export type CsvSaveInput = {
  /** The serializer's output (`serializeCsvTable`), unchanged. */
  readonly text: string;
  /** `swbpipe-<csv kind>[-<qualifier>].csv` (`csvExportFileName`). */
  readonly fileName: string;
  /** The user's explicit local-private intent, owned by the calling control. */
  readonly explicitLocalPrivateIntent: boolean;
};

export type CsvSaveOutcome =
  | { readonly mode: "native"; readonly receipt: NativeResultSaveReceipt }
  | { readonly mode: "browser"; readonly file_name: string; readonly byte_count: number };

function refused(code: string): NativeResultSaveError {
  return { code, stage: "request", message: "Local CSV save did not complete.", partial_file_name: null, cleanup: "not_needed" };
}

/** The screened save request for one CSV file, or the refusal. Exposed for the controls'
 * tests; `saveCsvTable` is the entry point. */
export function csvSaveRequest(input: CsvSaveInput): NativeResultSaveRequest {
  if (typeof input.text !== "string" || !input.text.startsWith(CSV_BOM) || input.text.length === CSV_BOM.length) {
    throw refused("INVALID_CSV_TEXT");
  }
  if (typeof input.fileName !== "string" || !isCsvExportFileName(input.fileName)) throw refused("INVALID_FILE_NAME");
  const explicitIntent = input.explicitLocalPrivateIntent === true;
  const controlled = controlRouteExport(input.text, {
    routeId: CSV_TABLE_ROUTE_ID,
    exportContext: "local_private",
    explicitLocalPrivateIntent: explicitIntent,
    requireLosslessMaterialization: false
  });
  const local = controlled.summary.local_first;
  if (!explicitIntent || controlled.blocked || !local || local.blocked) {
    throw refused(explicitIntent ? "REQUEST_EVIDENCE_DENIED" : "LOCAL_PRIVATE_INTENT_REQUIRED");
  }
  const exact = controlled.payload === input.text;
  if (!exact) throw refused("CSV_PAYLOAD_MATERIALIZATION_CHANGED");
  return {
    href: `${CSV_DATA_PREFIX}${encodeURIComponent(input.text)}`,
    file_name: input.fileName,
    screening: {
      route_id: CSV_TABLE_ROUTE_ID,
      export_context: "local_private",
      explicit_local_private_intent: true,
      blocked: controlled.blocked,
      materialization_withheld: controlled.summary.materialization_withheld,
      lossless_required: false,
      exact_payload_match: exact,
      blocking_count: controlled.summary.blocking_count
    },
    local_first: local
  };
}

/** Save one CSV file the user asked to export. Native: the governed native save, which
 * names the saved file in its receipt. Browser: an anchor download of the same bytes. */
export async function saveCsvTable(input: CsvSaveInput, doc: Document = document): Promise<CsvSaveOutcome> {
  const request = csvSaveRequest(input);
  if (isNativeResultSaveRuntime()) return { mode: "native", receipt: await saveNativeResultJson(request) };
  const anchor = doc.createElement("a");
  anchor.href = request.href;
  anchor.download = request.file_name;
  anchor.rel = "noopener";
  anchor.style.display = "none";
  doc.body.appendChild(anchor);
  try { anchor.click(); } finally { anchor.remove(); }
  return { mode: "browser", file_name: request.file_name, byte_count: new TextEncoder().encode(input.text).length };
}

/* -------------------------------------------------------------------------- *
 * Streamed CSV (C1 native custody, DEL-04-05 `c1-native-custody-plan.md` §5)
 * -------------------------------------------------------------------------- */

/** The most UTF-16 units one streamed chunk carries (at most 768 KiB of UTF-8); a longer text is
 * split, never truncated, at a boundary that does not divide a surrogate pair. */
export const CSV_STREAM_CHUNK_UNITS = 256 * 1024;

/** A CSV table written in parts: its file name, the held job its rows are read from, and its
 * text in order (the head, then bounded pages of rows), each part produced only when asked. */
export type CsvStreamInput = {
  readonly fileName: string;
  readonly jobId: string;
  readonly chunks: AsyncIterable<string>;
  /** The user's explicit local-private intent, owned by the calling control. */
  readonly explicitLocalPrivateIntent: boolean;
};

/** The native stream commands (`native_csv_stream.rs`), raw-body ingress. */
export type CsvStreamInvoke = (command: "begin_local_csv_stream" | "append_local_csv_stream" | "finish_local_csv_stream" | "abort_local_csv_stream", args: Record<string, unknown>) => Promise<unknown>;
const ingressStreamInvoke: CsvStreamInvoke = (command, args) => invokeWithIngress<unknown>(command as NativeIngressCommand, args);

/** `text` in parts of at most `units` UTF-16 units, none ending inside a surrogate pair. */
export function splitCsvChunk(text: string, units = CSV_STREAM_CHUNK_UNITS): string[] {
  const parts: string[] = [];
  for (let at = 0; at < text.length;) {
    let end = Math.min(text.length, at + units);
    const last = text.charCodeAt(end - 1);
    if (end < text.length && last >= 0xd800 && last <= 0xdbff) end -= 1;
    parts.push(text.slice(at, end));
    at = end;
  }
  return parts;
}

/** One chunk's screening on the CSV route, as `csvSaveRequest` screens a whole file: local-private
 * output with explicit intent, the controlled payload exactly the chunk. A text payload's route
 * decision reads its path, not its characters, so each chunk's decision is the whole file's; it is
 * still taken for every chunk sent. The first chunk begins with the byte-order mark. */
export function csvChunkEvidence(text: string, first: boolean, explicitLocalPrivateIntent: boolean): Pick<NativeResultSaveRequest, "screening" | "local_first"> {
  if (typeof text !== "string" || text.length === 0 || (first && (!text.startsWith(CSV_BOM) || text.length === CSV_BOM.length))) throw refused("INVALID_CSV_TEXT");
  const explicitIntent = explicitLocalPrivateIntent === true;
  const controlled = controlRouteExport(text, {
    routeId: CSV_TABLE_ROUTE_ID,
    exportContext: "local_private",
    explicitLocalPrivateIntent: explicitIntent,
    requireLosslessMaterialization: false
  });
  const local = controlled.summary.local_first;
  if (!explicitIntent || controlled.blocked || !local || local.blocked) {
    throw refused(explicitIntent ? "REQUEST_EVIDENCE_DENIED" : "LOCAL_PRIVATE_INTENT_REQUIRED");
  }
  if (controlled.payload !== text) throw refused("CSV_PAYLOAD_MATERIALIZATION_CHANGED");
  return {
    screening: {
      route_id: CSV_TABLE_ROUTE_ID,
      export_context: "local_private",
      explicit_local_private_intent: true,
      blocked: controlled.blocked,
      materialization_withheld: controlled.summary.materialization_withheld,
      lossless_required: false,
      exact_payload_match: true,
      blocking_count: controlled.summary.blocking_count
    },
    local_first: local
  };
}

const utf8Length = (text: string) => new TextEncoder().encode(text).length;

/** Saves a CSV table streamed in parts through the native stream: each part screened and sent in
 * order, at most one part held here at a time, the file finished with the single save's receipt.
 * Any failure (a part that cannot be produced or screened, a refused chunk) aborts the stream, so
 * its partial file is removed natively; the error is the save's typed error. Native runtime only:
 * a result held natively exists only there. */
export async function saveCsvTableStream(input: CsvStreamInput, invoke: CsvStreamInvoke = ingressStreamInvoke): Promise<CsvSaveOutcome> {
  if (!isNativeResultSaveRuntime()) throw { ...refused("NATIVE_RUNTIME_REQUIRED"), stage: "invoke" };
  if (typeof input.fileName !== "string" || !isCsvExportFileName(input.fileName)) throw refused("INVALID_FILE_NAME");
  const named = { file_name: input.fileName, screening: { route_id: CSV_TABLE_ROUTE_ID } } as Pick<NativeResultSaveRequest, "file_name" | "screening">;
  let streamId: string | null = null;
  let sequence = 0;
  let bytes = 0;
  let done = false;
  const send = async (command: Parameters<CsvStreamInvoke>[0], args: Record<string, unknown>) => {
    try { return await invoke(command, args); } catch (error) { throw normalizeError(error, named); }
  };
  try {
    for await (const chunk of input.chunks) {
      for (const text of splitCsvChunk(chunk)) {
        const evidence = csvChunkEvidence(text, sequence === 0, input.explicitLocalPrivateIntent);
        if (streamId === null) {
          const begun = await send("begin_local_csv_stream", { jobId: input.jobId, request: { file_name: input.fileName, text, ...evidence } }) as { stream_id?: unknown; sequence?: unknown };
          if (typeof begun?.stream_id !== "string" || !/^[0-9a-f]{32}$/.test(begun.stream_id) || begun.sequence !== 1) throw refused("INVALID_STREAM_RECEIPT");
          streamId = begun.stream_id;
        } else {
          const appended = await send("append_local_csv_stream", { streamId, sequence, chunk: { text, ...evidence } }) as { sequence?: unknown };
          if (appended?.sequence !== sequence + 1) throw refused("INVALID_STREAM_RECEIPT");
        }
        sequence += 1;
        bytes += utf8Length(text);
      }
    }
    if (streamId === null) throw refused("INVALID_CSV_TEXT");
    const finished = await send("finish_local_csv_stream", { streamId, sequence, byteCount: bytes });
    done = true;
    if (!isSaveReceipt(finished, named) || finished.byte_count !== bytes) throw { ...refused("INVALID_SAVE_RECEIPT"), stage: "worker", cleanup: "unknown" };
    return { mode: "native", receipt: finished };
  } catch (error) {
    if (streamId !== null && !done) {
      // The native side removes the partial file (a stream it already ended is unknown there, and
      // its refusal already stated the cleanup). A refusal raised here states the abort's cleanup.
      const aborted = await invoke("abort_local_csv_stream", { streamId }).catch(() => null) as { cleanup?: unknown; partial_file_name?: unknown } | null;
      const own = error as Partial<NativeResultSaveError> | null;
      if (aborted && own && typeof own === "object" && typeof own.code === "string" && typeof own.stage === "string") {
        throw normalizeError({ code: own.code, stage: own.stage, message: own.message, cleanup: aborted.cleanup, partial_file_name: aborted.partial_file_name ?? null }, named);
      }
    }
    throw error;
  }
}
