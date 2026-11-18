/** What a report holder keeps of a controlled route export once its per-value decisions and
 * findings have been read for the last time (DEL-08-01): the route summary (counts and route
 * evidence) and whether the export is blocked, and, for a holder that reads it, the controlled
 * payload. The summary holds no reference to the decisions or the findings, so a holder of
 * this evidence does not keep them reachable. The control (DEL-12-02) still returns every
 * decision and finding to its caller, and the package preparations return what they did; this
 * only names what the report's panel and the session's package state keep afterwards. */
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";

export type ControlledRouteEvidence = Pick<ControlledRouteExport, "summary" | "blocked">;
export type ControlledRouteEvidenceWithPayload = ControlledRouteEvidence & Pick<ControlledRouteExport, "payload">;

/** The evidence a holder keeps of `controlled`: its summary and blocked flag, and its payload
 * only when `keep` asks for it. */
export function controlledRouteEvidence(controlled: ControlledRouteExport): ControlledRouteEvidence;
export function controlledRouteEvidence(controlled: ControlledRouteExport, keep: "payload"): ControlledRouteEvidenceWithPayload;
export function controlledRouteEvidence(controlled: ControlledRouteExport, keep?: "payload"): ControlledRouteEvidence | ControlledRouteEvidenceWithPayload {
  const evidence: ControlledRouteEvidence = { summary: controlled.summary, blocked: controlled.blocked };
  return keep === "payload" ? { ...evidence, payload: controlled.payload } : evidence;
}
