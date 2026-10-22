import type { ReactNode } from "react";
import { OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL, routeSurfaceDecision } from "./outputPolicy";
import { sourceContract } from "./resultReader";
import type { MechanicsResult } from "../../types";

function gateRefusal(result: MechanicsResult | null | undefined, surface: string): string | null {
  if (!result) return null;
  let route: string;
  try { route = sourceContract(result); } catch { return null; }
  const decided = routeSurfaceDecision(route, surface);
  return decided === null ? null : decided.reason || OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL;
}

/** Output gate for one surface, named by its test-id prefix (`OUTPUT_SURFACES`): for a
 * gated route's result it shows the policy's refusal for this surface in place of the
 * surface's download or handoff controls; otherwise it renders them unchanged. Epoch 8
 * (DEL-04-07 Design `desktop-exact-outputs.md` §5.2): the gate reads the surface's own
 * decision, so an `admitted` cell (the exact routes on `rule-check`) adds no refusal. A
 * prefix the policy does not name refuses, and so does any other decision, including
 * `admitted_when_eligible`: this gate reads no standing, so it opens only where the policy
 * adds no refusal at all. Display stays. */
export function LoadReferenceOutputGate({ result, testIdPrefix, children }: { result: MechanicsResult | null | undefined; testIdPrefix: string; children: ReactNode }) {
  const refusal = gateRefusal(result, testIdPrefix);
  if (refusal) return <p className="muted" role="status" data-testid={`${testIdPrefix}-load-reference-output-unavailable`}>{refusal}</p>;
  return <>{children}</>;
}
