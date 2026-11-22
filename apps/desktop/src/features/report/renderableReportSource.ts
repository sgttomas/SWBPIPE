/** DEL-08-01 REQ-017 and REQ-024: the rendered report's source on the desktop.
 *
 * The rendered report and its print view take an exact-route result whatever its
 * numerical standing, where Current's other requirements hold, its mechanics are
 * solved and the product reader reads it. Every other output, the report package
 * on the pressure-free routes included, keeps Current (`currentSolvedResult`), which
 * also requires numerical eligibility. On the exact route the report package and the
 * user release take this same source (DEL-08-01 REQ-024, "Package and user release").
 *
 * This selection is a candidate. The webview holds a native result's header and its
 * native standing, not its rows, so it judges readability from the standing's
 * findings: each must be a finding that states a readable result's standing (the
 * closed standing-finding vocabulary, `STANDING_FINDINGS`). The reader's checks that
 * need the full result (a case record against its integrity code, duplicated
 * evidence) are made before any render: by the build step on the full copy
 * (`buildCurrentResultExport` with purpose "report") and by the native host
 * (`exact_source_binding.rs` `read_delivered_source`). If either refuses, the panel
 * shows that refusal by name and no rendered report. */

/** The closed `standing_finding` vocabulary (`core/reporting/numerical_vocabulary`),
 * held equal to the generated catalogue `fixtures/reports/numerical_vocabulary.json`
 * by `renderableReportSource.test.ts`. */
export const STANDING_FINDINGS: readonly string[] = Object.freeze([
  "NUMERICAL_INTEGRITY_NOT_QUALIFIED",
  "NUMERICAL_CASE_EVIDENCE_INCOMPLETE",
  "SOURCE_BLOCKS_CASE_UNQUALIFIED",
  "SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY",
  "SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED",
  "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1",
  "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"
]);

/** The rendered report's source: `current` (an eligible result) when there is one;
 * else `result` when it is an exact-route result, every other Current requirement
 * holds, its mechanics are solved and its standing names only standing findings;
 * else null. `standing` is the reader's standing of `result` as the session holds it
 * (null where it has none). */
export function renderableReportSource<R>({
  current,
  result,
  otherCurrentRequirementsHold,
  exactRoute,
  mechanicsSolved,
  standing
}: {
  current: R | null;
  result: R | null;
  otherCurrentRequirementsHold: boolean;
  exactRoute: boolean;
  mechanicsSolved: boolean;
  standing: Readonly<{ findings: readonly string[] }> | null;
}): R | null {
  if (current !== null) return current;
  if (result === null || !otherCurrentRequirementsHold || !exactRoute || !mechanicsSolved || standing === null) return null;
  return standing.findings.every((finding) => STANDING_FINDINGS.includes(finding)) ? result : null;
}
