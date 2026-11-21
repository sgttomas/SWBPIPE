/** DEL-08-01 REQ-024, "Package and user release": the report frame's statements on a
 * result's standing, for the user-release dialog to show before the user's statement.
 *
 * The words are the renderer's (`report_renderer::standing::frame_statements`), given by the
 * native host for a result in its custody (`readNativeFrameStatements`): each a line, in
 * the frame's order (support states not confirmed; kept for inspection). Nothing here
 * composes, reorders or rewords a statement. A pressure-free result's report carries no
 * standing record, so its frame states none. An exact-route result whose statements cannot
 * be obtained is refused by name: the dialog then shows the refusal in place of the
 * statement, and nothing is saved. */
import { nativeCustodyOf, readNativeFrameStatements } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";
import { isExactReportSource } from "./exactReportSections";

export const REPORT_FRAME_STATEMENTS_UNAVAILABLE = "REPORT-FRAME-STATEMENTS-UNAVAILABLE";

export type ReportFrameStatements =
  | { readonly ok: true; readonly statements: readonly string[] }
  | { readonly ok: false; readonly reason: string };

/** The frame statements of `result`'s report: none for a pressure-free result; for an
 * exact-route result in native custody, the native host's, read with the texts its native
 * standing was bound to; otherwise a refusal by name. */
export async function reportFrameStatements(result: unknown): Promise<ReportFrameStatements> {
  if (!isExactReportSource(result as MechanicsResult)) return { ok: true, statements: Object.freeze([]) };
  const native = nativeCustodyOf(result);
  if (!native || !native.binding.standing) {
    return { ok: false, reason: `${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: the native host holds no reading of this result's standing to give its report's statements from.` };
  }
  try {
    const read = await readNativeFrameStatements(native.jobId, { modelText: native.binding.standingModelText, invocationText: native.binding.invocationText });
    return { ok: true, statements: read.frame_statements };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return { ok: false, reason: message.startsWith(REPORT_FRAME_STATEMENTS_UNAVAILABLE) ? message : `${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: ${message}` };
  }
}
