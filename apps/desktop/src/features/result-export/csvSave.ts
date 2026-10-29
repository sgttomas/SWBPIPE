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
  saveNativeResultJson,
  type NativeResultSaveError,
  type NativeResultSaveReceipt,
  type NativeResultSaveRequest
} from "./nativeResultSave";

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
