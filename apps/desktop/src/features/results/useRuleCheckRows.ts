/** C1: the rows a native result's rule-check run binds, read from the result the native host holds
 * by bounded lookup of their ids (pages of at most `RESULT_DETAIL_MAX_KEYS`), for the stress and
 * ratio table, the key results and the viewport's rule ratios. Each row is keyed as a native
 * table's rows are (`nativeRowKey`); an id the lookup does not find is left out, so the table reads
 * the run as another result's, as on the full rows. One read per result and run, shared by the
 * displays. `null` until the rows are read, and when the read is refused. */
import { useEffect, useState } from "react";
import { RESULT_DETAIL_MAX_KEYS } from "../../services/resultDetail";
import { isNativeResult, lookupNativeRows, nativeRowKey, type NativeResult, type SessionResult } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { ruleCheckBoundResultIds } from "./stressRatioTable";

type ResultRow = MechanicsResult["results"][number];
export type RuleCheckRows = ReadonlyMap<string, ResultRow>;

const reads = new WeakMap<NativeResult, WeakMap<SessionRuleCheckRun, Promise<RuleCheckRows>>>();

/** The run's bound rows in `result`, by id; read once for the result and run. */
export function readRuleCheckRows(result: NativeResult, run: SessionRuleCheckRun): Promise<RuleCheckRows> {
  const forResult = reads.get(result) ?? new WeakMap<SessionRuleCheckRun, Promise<RuleCheckRows>>();
  reads.set(result, forResult);
  const known = forResult.get(run);
  if (known) return known;
  const read = (async () => {
    const ids = ruleCheckBoundResultIds(run);
    const rows = new Map<string, ResultRow>();
    for (let at = 0; at < ids.length; at += RESULT_DETAIL_MAX_KEYS) {
      const keys = ids.slice(at, at + RESULT_DETAIL_MAX_KEYS);
      // "last": the full rows' index by id keeps the last row of an id.
      const found = await lookupNativeRows(result, { ids: keys, match: "last" });
      found.ids.forEach((entry, i) => {
        if (entry) rows.set(keys[i], Object.freeze({ ...entry.row, id: nativeRowKey(entry.ordinal) }) as ResultRow);
      });
    }
    return rows as RuleCheckRows;
  })();
  forResult.set(run, read);
  // A refused read is not kept, so a later display reads again.
  read.catch(() => { if (forResult.get(run) === read) forResult.delete(run); });
  return read;
}

export function useRuleCheckRows(result: SessionResult | null | undefined, run: SessionRuleCheckRun | null | undefined): RuleCheckRows | null {
  const [read, setRead] = useState<{ result: NativeResult; run: SessionRuleCheckRun; rows: RuleCheckRows } | null>(null);
  const native = result && isNativeResult(result) ? result : null;
  useEffect(() => {
    if (!native || !run) return;
    let live = true;
    readRuleCheckRows(native, run).then(
      (rows) => { if (live) setRead({ result: native, run, rows }); },
      () => { if (live) setRead(null); }
    );
    return () => { live = false; };
  }, [native, run]);
  return native && run && read && read.result === native && read.run === run ? read.rows : null;
}
