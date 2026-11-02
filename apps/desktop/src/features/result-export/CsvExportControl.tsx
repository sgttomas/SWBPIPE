// RT-R6 (DEL-08-04-R14; Design `csv-export.md` §4): the "Export CSV" control of a results
// table. Pressing it is the user's explicit local-private intent: the table's adapter builds
// the file from the view model the panel renders, the serializer writes it, and `saveCsvTable`
// saves it on the governed local-first route `DOTH-CSV-011`. The status line names the file.
import { useEffect, useRef, useState } from "react";
import { saveCsvTable, saveCsvTableStream, type CsvSaveOutcome } from "./csvSave";
import { serializeCsvTable, type CsvTable } from "./csvTable";

/** What an adapter returns for one export: the table and its suggested file name. */
export type CsvExport = Readonly<{ table: CsvTable; fileName: string }>;
/** A table written in parts (a results group of a result held natively): its file name, the held
 * job its rows are read from, and its text in order, each part produced only when asked. */
export type CsvStreamExport = Readonly<{ fileName: string; jobId: string; chunks: AsyncIterable<string> }>;

/** Every table that exports to CSV carries this attribute, naming its CSV kind (the guard test
 * lists `<table` elements under `features/results` and `features/comparison`). */
export const CSV_EXPORT_ATTRIBUTE = "data-csv-export";

type Status = { outcome: "pending" } | { outcome: "saved"; text: string } | { outcome: "error"; text: string };

function savedText(outcome: CsvSaveOutcome): string {
  return outcome.mode === "native"
    ? `Saved ${outcome.receipt.file_name} (${outcome.receipt.byte_count} bytes).`
    : `Downloaded ${outcome.file_name} (${outcome.byte_count} bytes).`;
}

function errorText(error: unknown): string {
  if (error && typeof error === "object" && "code" in error && typeof (error as { code: unknown }).code === "string") {
    const { code, message } = error as { code: string; message?: unknown };
    return `CSV export did not complete (${code})${typeof message === "string" ? `: ${message}` : ""}.`;
  }
  return `CSV export did not complete: ${error instanceof Error ? error.message : String(error)}.`;
}

export function CsvExportControl({ tableLabel, build, stream, testId, save = saveCsvTable, saveStream }: {
  /** The table's name, in the control's accessible name ("Export CSV: <tableLabel>"). */
  tableLabel: string;
  /** The table's adapter over the view model the panel renders. */
  build?: () => Promise<CsvExport> | CsvExport;
  /** Instead of `build`: the table written in parts through the native stream. */
  stream?: () => Promise<CsvStreamExport> | CsvStreamExport;
  testId: string;
  save?: typeof saveCsvTable;
  saveStream?: (input: Parameters<typeof saveCsvTableStream>[0]) => Promise<CsvSaveOutcome>;
}) {
  const [status, setStatus] = useState<Status | null>(null);
  const mounted = useRef(true);
  const inFlight = useRef(false);
  // Set in setup as well as cleared in cleanup: React.StrictMode replays effects (cleanup, then
  // setup), and a ref left false would keep the button disabled and hide the outcome.
  useEffect(() => {
    mounted.current = true;
    return () => { mounted.current = false; };
  }, []);
  const onExport = async () => {
    if (inFlight.current) return;
    inFlight.current = true;
    setStatus({ outcome: "pending" });
    try {
      let outcome: CsvSaveOutcome;
      if (stream) {
        const parts = await stream();
        outcome = await (saveStream ?? saveCsvTableStream)({ fileName: parts.fileName, jobId: parts.jobId, chunks: parts.chunks, explicitLocalPrivateIntent: true });
      } else {
        const built = await build!();
        outcome = await save({ text: serializeCsvTable(built.table), fileName: built.fileName, explicitLocalPrivateIntent: true });
      }
      if (mounted.current) setStatus({ outcome: "saved", text: savedText(outcome) });
    } catch (error) {
      if (mounted.current) setStatus({ outcome: "error", text: errorText(error) });
    } finally {
      inFlight.current = false;
    }
  };
  return (
    <span className="csv-export-control">
      <button type="button" data-testid={testId} aria-label={`Export CSV: ${tableLabel}`} disabled={status?.outcome === "pending"} onClick={() => void onExport()}>
        Export CSV
      </button>
      <span role="status" data-testid={`${testId}-status`}>
        {status?.outcome === "pending" ? "Exporting CSV…" : status ? status.text : ""}
      </span>
    </span>
  );
}
