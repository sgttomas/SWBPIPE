import type { ReactNode } from "react";
import { surfaceRouteRefusal, type OutputSurface } from "./outputPolicy";
import type { MechanicsResult } from "../../types";

/** Output gate for one surface, named by its test-id prefix (`OUTPUT_SURFACES`): for a
 * gated route's result it shows the policy's refusal for this surface in place of the
 * surface's download or handoff controls; otherwise it renders them unchanged. Epoch 8
 * (DEL-04-07 Design `desktop-exact-outputs.md` §5.2): the gate reads the surface's own
 * decision, so an `admitted` cell (the exact routes on `rule-check`) adds no refusal. A
 * prefix the policy does not name refuses. Display stays. */
export function LoadReferenceOutputGate({ result, testIdPrefix, children }: { result: MechanicsResult | null | undefined; testIdPrefix: string; children: ReactNode }) {
  const refusal = surfaceRouteRefusal(result, testIdPrefix as OutputSurface);
  if (refusal) return <p className="muted" role="status" data-testid={`${testIdPrefix}-load-reference-output-unavailable`}>{refusal}</p>;
  return <>{children}</>;
}
