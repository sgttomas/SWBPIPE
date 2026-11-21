/** C1: the rows a native result's rule-check run binds, read from the result the native host holds
 * by bounded lookup of their ids (pages of at most `RESULT_DETAIL_MAX_KEYS`), for the stress and
 * ratio table, the key results and the viewport's rule ratios.
 *
 * Only what those displays read of a row is kept: its native row key (`nativeRowKey`, as a native
 * table keys its rows), its entity and its location, component and section-basis metadata. What is
 * kept is bounded (`RULE_CHECK_ROWS_KEPT_LIMIT_BYTES`). An id the lookup does not find is left out,
 * so the table reads the run as another result's, as on the full rows. One read per result and run
 * is shared by the displays while the result is held; a read whose custody ends meanwhile, or
 * ended before a kept read is returned, is refused as disposed and not kept. */
import { useEffect, useState } from "react";
import { RESULT_DETAIL_MAX_KEYS, type Lookup, type LookupRequest } from "../../services/resultDetail";
import { isNativeJobLive, lookupNativeRows, nativeCustody, nativeRowKey, NATIVE_RESULT_DISPOSED, isNativeResult, type NativeResult, type SessionResult } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { ruleCheckBoundResultIds, type RuleRowsRead } from "./stressRatioTable";

type ResultRow = MechanicsResult["results"][number];
export type RuleCheckRows = ReadonlyMap<string, ResultRow>;

/** The read's state: none asked (no native result or no run), being read, read, or refused (with
 * the refusal kept, its message in the native refusal's words). */
export type RuleCheckRowsStatus = Readonly<
  | { state: "idle" }
  | { state: "loading" }
  | { state: "ready"; rows: RuleCheckRows }
  | { state: "error"; error: unknown; message: string }
>;

/** The most the kept rows may hold, counted as the UTF-16 length of their kept strings: the run's
 * reply limit (`MAX_RULE_CHECK_REPLY_BYTES`, 8 MiB). The run's outcomes, and so its bound ids, are
 * bounded by that reply; a held result whose bound rows carry longer locations is refused here. */
export const RULE_CHECK_ROWS_KEPT_LIMIT_BYTES = 8 * 1024 * 1024;

/** What a display reads of a bound row, keyed by its native row key. */
export function keptRuleCheckRow(ordinal: number, row: ResultRow): ResultRow {
  const metadata = (row.metadata ?? {}) as Record<string, unknown>;
  const kept: Record<string, string> = {};
  for (const key of ["location", "component", "basis"] as const) {
    if (typeof metadata[key] === "string") kept[key] = metadata[key] as string;
  }
  return Object.freeze({ id: nativeRowKey(ordinal), entity_ref: row.entity_ref, metadata: Object.freeze(kept) }) as unknown as ResultRow;
}

function keptLength(row: ResultRow): number {
  const metadata = row.metadata as Record<string, string>;
  return row.id.length + (row.entity_ref?.length ?? 0) + Object.values(metadata).reduce((sum, text) => sum + text.length, 0);
}

/** The bound rows `ids` names, page by page through `lookup` ("last": the full rows' index by id
 * keeps the last row of an id), each kept as `keptRuleCheckRow` keeps it; `stillWanted` is checked
 * after every page, and the read refused by its error once it is not. */
export async function collectRuleCheckRows(
  ids: readonly string[],
  lookup: (request: LookupRequest) => Promise<Lookup>,
  stillWanted: () => void = () => undefined
): Promise<RuleCheckRows> {
  const rows = new Map<string, ResultRow>();
  let kept = 0;
  for (let at = 0; at < ids.length; at += RESULT_DETAIL_MAX_KEYS) {
    const keys = ids.slice(at, at + RESULT_DETAIL_MAX_KEYS);
    const found = await lookup({ ids: keys, match: "last" });
    stillWanted();
    found.ids.forEach((entry, i) => {
      if (!entry) return;
      const row = keptRuleCheckRow(entry.ordinal, entry.row as ResultRow);
      kept += keptLength(row);
      rows.set(keys[i], row);
    });
    if (kept > RULE_CHECK_ROWS_KEPT_LIMIT_BYTES) {
      throw new Error(`RESULT-DETAIL-RESPONSE-LIMIT: the rule-check rows the displays keep pass their ${RULE_CHECK_ROWS_KEPT_LIMIT_BYTES}-byte limit.`);
    }
  }
  return rows;
}

const reads = new WeakMap<NativeResult, WeakMap<SessionRuleCheckRun, Promise<RuleCheckRows>>>();

function held(jobId: string): void {
  if (!isNativeJobLive(jobId)) throw new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody has ended.`);
}

/** The run's bound rows in `result`, by id; read once for the result and run while it is held. */
export function readRuleCheckRows(result: NativeResult, run: SessionRuleCheckRun): Promise<RuleCheckRows> {
  const { jobId } = nativeCustody(result);
  const forResult = reads.get(result) ?? new WeakMap<SessionRuleCheckRun, Promise<RuleCheckRows>>();
  reads.set(result, forResult);
  const known = forResult.get(run);
  if (known) {
    // A kept read is returned only while the result is held; once custody ends it is dropped.
    if (!isNativeJobLive(jobId)) {
      reads.delete(result);
      return Promise.reject(new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody has ended.`));
    }
    return known.then((rows) => {
      held(jobId);
      return rows;
    });
  }
  const read = (async () => {
    held(jobId);
    const rows = await collectRuleCheckRows(ruleCheckBoundResultIds(run), (request) => lookupNativeRows(result, request), () => held(jobId));
    held(jobId);
    return rows;
  })();
  forResult.set(run, read);
  // A refused read is not kept, so a later display reads again.
  read.catch(() => { if (forResult.get(run) === read) forResult.delete(run); });
  return read;
}

const IDLE: RuleCheckRowsStatus = Object.freeze({ state: "idle" });
const LOADING: RuleCheckRowsStatus = Object.freeze({ state: "loading" });
const NO_ROWS: RuleCheckRowsStatus = Object.freeze({ state: "ready", rows: new Map() });

export function useRuleCheckRows(result: SessionResult | null | undefined, run: SessionRuleCheckRun | null | undefined): RuleCheckRowsStatus {
  const native = result && isNativeResult(result) ? result : null;
  const [read, setRead] = useState<{ result: NativeResult; run: SessionRuleCheckRun; status: RuleCheckRowsStatus } | null>(null);
  // A run that binds no row is ready at once, with nothing to read.
  const bindsNothing = !!run && ruleCheckBoundResultIds(run).length === 0;
  useEffect(() => {
    // An obsolete read (another result or run, or none) is cleared, not kept.
    setRead(null);
    if (!native || !run || bindsNothing) return;
    let live = true;
    readRuleCheckRows(native, run).then(
      (rows) => { if (live) setRead({ result: native, run, status: Object.freeze({ state: "ready", rows }) }); },
      (error: unknown) => {
        if (live) setRead({ result: native, run, status: Object.freeze({ state: "error", error, message: error instanceof Error ? error.message : String(error) }) });
      }
    );
    return () => { live = false; };
  }, [native, run, bindsNothing]);
  if (!native || !run) return IDLE;
  if (bindsNothing) return NO_ROWS;
  return read && read.result === native && read.run === run ? read.status : LOADING;
}

/** The read as the table builders take it (`stressRatioTableWithRuleRows`); with no run nothing is read. */
export function ruleRowsRead(status: RuleCheckRowsStatus): RuleRowsRead {
  return status.state === "idle" ? NO_ROWS as RuleRowsRead : status as RuleRowsRead;
}
