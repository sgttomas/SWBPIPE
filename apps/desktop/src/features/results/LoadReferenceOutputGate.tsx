import type { ReactNode } from "react";
import { loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import type { MechanicsResult } from "../../types";

/** Output gate for one surface: for a load/reference-state result it shows the
 * shared not-yet-available reason in place of the surface's download or
 * handoff controls; otherwise it renders them unchanged. Display stays. */
export function LoadReferenceOutputGate({ result, testIdPrefix, children }: { result: MechanicsResult | null | undefined; testIdPrefix: string; children: ReactNode }) {
  const refusal = loadReferenceOutputRefusal(result);
  if (refusal) return <p className="muted" role="status" data-testid={`${testIdPrefix}-load-reference-output-unavailable`}>{refusal}</p>;
  return <>{children}</>;
}
